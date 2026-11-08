pub mod state;

use self::state::State;
use crate::token::TokenRef;

/// https://vt100.net/emu/dec_ansi_parser
/// https://wezfurlong.org/ecma48/07-control.html
pub struct Tokenizer {
    state: State,
    utf8_buf: [u8; 4],
    utf8_len: usize,
    csi_private: Option<u8>,
    csi_intermediate: Option<u8>,
    csi_params: Vec<u8>,
    string_buf: Vec<u8>,
    string_pending_esc: bool,
}

#[derive(Clone, Copy)]
enum StringKind {
    Osc,
    Apc,
    Dcs,
    Pm,
    Sos,
}

impl StringKind {
    fn token<'a>(self, bytes: &'a [u8]) -> TokenRef<'a> {
        match self {
            StringKind::Osc => TokenRef::Osc(bytes),
            StringKind::Apc => TokenRef::Apc(bytes),
            StringKind::Dcs => TokenRef::Dcs(bytes),
            StringKind::Pm => TokenRef::Pm(bytes),
            StringKind::Sos => TokenRef::Sos(bytes),
        }
    }
}

fn is_control(b: u8) -> bool {
    b < 0x20 || b == 0x7F
}

impl Default for Tokenizer {
    fn default() -> Self {
        Self::new()
    }
}

impl Tokenizer {
    pub fn new() -> Self {
        Self {
            state: State::Ground,
            utf8_buf: [0; 4],
            utf8_len: 0,
            csi_private: None,
            csi_intermediate: None,
            csi_params: Vec::new(),
            string_buf: Vec::new(),
            string_pending_esc: false,
        }
    }

    pub fn reset(&mut self) {
        self.state = State::Ground;
        self.utf8_len = 0;
        self.csi_private = None;
        self.csi_intermediate = None;
        self.csi_params.clear();
        self.string_buf.clear();
        self.string_pending_esc = false;
    }

    pub fn has_pending(&self) -> bool {
        self.state == State::Escape || self.introducer().is_some()
    }

    pub fn timed_out(&mut self, mut sink: impl FnMut(TokenRef<'_>)) {
        if self.state == State::Escape {
            sink(TokenRef::C0(0x1B));
        } else if let Some(introducer) = self.introducer() {
            sink(TokenRef::Esc {
                intermediate: None,
                final_byte: introducer,
            });
        } else {
            return;
        }

        self.reset();
    }

    fn introducer(&self) -> Option<u8> {
        let untouched = self.string_buf.is_empty() && !self.string_pending_esc;
        match self.state {
            State::Ss3 => Some(b'O'),
            State::CsiEntry => Some(b'['),
            State::OscString if untouched => Some(b']'),
            State::ApcString if untouched => Some(b'_'),
            State::DcsString if untouched => Some(b'P'),
            State::PmString if untouched => Some(b'^'),
            State::SosString if untouched => Some(b'X'),
            _ => None,
        }
    }

    pub fn feed(&mut self, bytes: &[u8], mut sink: impl FnMut(TokenRef<'_>)) {
        let mut idx = 0;

        if self.utf8_len > 0 {
            while idx < bytes.len() && self.utf8_len < 4 {
                self.utf8_buf[self.utf8_len] = bytes[idx];
                self.utf8_len += 1;
                idx += 1;

                if let Ok(s) =
                    core::str::from_utf8(&self.utf8_buf[..self.utf8_len])
                {
                    sink(TokenRef::Text(s));
                    self.utf8_len = 0;
                    break;
                }
            }

            if self.utf8_len == 4 {
                let buf = self.utf8_buf;
                self.utf8_len = 0;
                for byte in buf {
                    sink(TokenRef::C0(byte));
                }
            }
        }

        while idx < bytes.len() {
            let b = bytes[idx];

            if b == 0x18 || b == 0x1A {
                self.reset();
                sink(TokenRef::C0(b));
                idx += 1;
                continue;
            }

            match self.state {
                State::Ground => {
                    if b == 0x1B {
                        self.state = State::Escape;
                        idx += 1;
                    } else if is_control(b) {
                        sink(TokenRef::C0(b));
                        idx += 1;
                    } else {
                        let start = idx;
                        let mut end = idx;
                        while end < bytes.len() {
                            let curr = bytes[end];
                            if curr == 0x1B || is_control(curr) {
                                break;
                            }
                            end += 1;
                        }

                        let slice = &bytes[start..end];
                        match core::str::from_utf8(slice) {
                            Ok(s) => {
                                sink(TokenRef::Text(s));
                                idx = end;
                            }
                            Err(e) => {
                                let up_to = e.valid_up_to();
                                if up_to > 0
                                    && let Ok(s) =
                                        core::str::from_utf8(&slice[..up_to])
                                {
                                    sink(TokenRef::Text(s));
                                }

                                if let Some(error_len) = e.error_len() {
                                    for &byte in
                                        &slice[up_to..up_to + error_len]
                                    {
                                        sink(TokenRef::C0(byte));
                                    }
                                    idx = start + up_to + error_len;
                                } else {
                                    let remaining = &slice[up_to..];
                                    self.utf8_len = remaining.len().min(4);
                                    self.utf8_buf[..self.utf8_len]
                                        .copy_from_slice(
                                            &remaining[..self.utf8_len],
                                        );
                                    idx = end;
                                }
                            }
                        }
                    }
                }
                State::Escape => match b {
                    b'[' => {
                        self.csi_private = None;
                        self.csi_intermediate = None;
                        self.csi_params.clear();
                        self.state = State::CsiEntry;
                        idx += 1;
                    }
                    b']' => {
                        self.string_buf.clear();
                        self.string_pending_esc = false;
                        self.state = State::OscString;
                        idx += 1;
                    }
                    b'_' => {
                        self.string_buf.clear();
                        self.string_pending_esc = false;
                        self.state = State::ApcString;
                        idx += 1;
                    }
                    b'P' => {
                        self.string_buf.clear();
                        self.string_pending_esc = false;
                        self.state = State::DcsString;
                        idx += 1;
                    }
                    b'^' => {
                        self.string_buf.clear();
                        self.string_pending_esc = false;
                        self.state = State::PmString;
                        idx += 1;
                    }
                    b'X' => {
                        self.string_buf.clear();
                        self.string_pending_esc = false;
                        self.state = State::SosString;
                        idx += 1;
                    }
                    b'O' => {
                        self.state = State::Ss3;
                        idx += 1;
                    }
                    0x20..=0x2F => {
                        self.state = State::EscapeIntermediate(b);
                        idx += 1;
                    }
                    0x30..=0x7E => {
                        sink(TokenRef::Esc {
                            intermediate: None,
                            final_byte: b,
                        });
                        self.state = State::Ground;
                        idx += 1;
                    }
                    0x1B => {
                        sink(TokenRef::C0(0x1B));
                        idx += 1;
                    }
                    0x00..=0x1F | 0x7F => {
                        sink(TokenRef::Esc {
                            intermediate: None,
                            final_byte: b,
                        });
                        self.state = State::Ground;
                        idx += 1;
                    }
                    _ => {
                        self.state = State::Ground;
                        idx += 1;
                    }
                },
                State::EscapeIntermediate(inter) => match b {
                    0x30..=0x7E => {
                        sink(TokenRef::Esc {
                            intermediate: Some(inter),
                            final_byte: b,
                        });
                        self.state = State::Ground;
                        idx += 1;
                    }
                    0x1B => {
                        sink(TokenRef::C0(0x1B));
                        self.state = State::Escape;
                        idx += 1;
                    }
                    _ => {
                        self.state = State::Ground;
                        idx += 1;
                    }
                },
                State::Ss3 => match b {
                    0x40..=0x7E => {
                        sink(TokenRef::Ss3(b));
                        self.state = State::Ground;
                        idx += 1;
                    }
                    0x1B => {
                        sink(TokenRef::C0(0x1B));
                        self.state = State::Escape;
                        idx += 1;
                    }
                    _ => {
                        self.state = State::Ground;
                        idx += 1;
                    }
                },
                State::CsiEntry => match b {
                    0x3C..=0x3F => {
                        self.csi_private = Some(b);
                        self.state = State::CsiParam;
                        idx += 1;
                    }
                    0x30..=0x39 | b';' | b':' => {
                        self.csi_params.push(b);
                        self.state = State::CsiParam;
                        idx += 1;
                    }
                    0x20..=0x2F => {
                        self.csi_intermediate = Some(b);
                        self.state = State::CsiIntermediate;
                        idx += 1;
                    }
                    0x40..=0x7E => {
                        sink(TokenRef::Csi {
                            prefix: self.csi_private,
                            params: &self.csi_params,
                            intermediate: self.csi_intermediate,
                            final_byte: b,
                        });
                        self.csi_params.clear();
                        self.csi_private = None;
                        self.csi_intermediate = None;
                        self.state = State::Ground;
                        idx += 1;
                    }
                    0x1B => {
                        self.reset();
                        self.state = State::Escape;
                        idx += 1;
                    }
                    _ => {
                        self.state = State::CsiIgnore;
                        idx += 1;
                    }
                },
                State::CsiParam => match b {
                    0x30..=0x39 | b';' | b':' => {
                        if self.csi_params.len() < 128 {
                            self.csi_params.push(b);
                        }
                        idx += 1;
                    }
                    0x20..=0x2F => {
                        self.csi_intermediate = Some(b);
                        self.state = State::CsiIntermediate;
                        idx += 1;
                    }
                    0x40..=0x7E => {
                        sink(TokenRef::Csi {
                            prefix: self.csi_private,
                            params: &self.csi_params,
                            intermediate: self.csi_intermediate,
                            final_byte: b,
                        });
                        self.csi_params.clear();
                        self.csi_private = None;
                        self.csi_intermediate = None;
                        self.state = State::Ground;
                        idx += 1;
                    }
                    0x1B => {
                        self.reset();
                        self.state = State::Escape;
                        idx += 1;
                    }
                    _ => {
                        self.state = State::CsiIgnore;
                        idx += 1;
                    }
                },
                State::CsiIntermediate => match b {
                    0x20..=0x2F => {
                        self.csi_intermediate = Some(b);
                        idx += 1;
                    }
                    0x40..=0x7E => {
                        sink(TokenRef::Csi {
                            prefix: self.csi_private,
                            params: &self.csi_params,
                            intermediate: self.csi_intermediate,
                            final_byte: b,
                        });
                        self.csi_params.clear();
                        self.csi_private = None;
                        self.csi_intermediate = None;
                        self.state = State::Ground;
                        idx += 1;
                    }
                    0x1B => {
                        self.reset();
                        self.state = State::Escape;
                        idx += 1;
                    }
                    _ => {
                        self.state = State::CsiIgnore;
                        idx += 1;
                    }
                },
                State::CsiIgnore => match b {
                    0x40..=0x7E => {
                        self.reset();
                        idx += 1;
                    }
                    0x1B => {
                        self.reset();
                        self.state = State::Escape;
                        idx += 1;
                    }
                    _ => {
                        idx += 1;
                    }
                },
                State::OscString => {
                    self.feed_string(b, &mut idx, &mut sink, StringKind::Osc)
                }
                State::ApcString => {
                    self.feed_string(b, &mut idx, &mut sink, StringKind::Apc)
                }
                State::DcsString => {
                    self.feed_string(b, &mut idx, &mut sink, StringKind::Dcs)
                }
                State::PmString => {
                    self.feed_string(b, &mut idx, &mut sink, StringKind::Pm)
                }
                State::SosString => {
                    self.feed_string(b, &mut idx, &mut sink, StringKind::Sos)
                }
            }
        }
    }

    fn feed_string(
        &mut self,
        b: u8,
        idx: &mut usize,
        sink: &mut impl FnMut(TokenRef<'_>),
        kind: StringKind,
    ) {
        if b == 0x9C {
            sink(kind.token(&self.string_buf));
            self.string_buf.clear();
            self.string_pending_esc = false;
            self.state = State::Ground;
            *idx += 1;
        } else if b == 0x1B {
            self.string_pending_esc = true;
            *idx += 1;
        } else if self.string_pending_esc {
            if b == b'\\' {
                sink(kind.token(&self.string_buf));
                self.string_buf.clear();
                self.string_pending_esc = false;
                self.state = State::Ground;
            } else {
                if self.string_buf.len() < 65536 {
                    self.string_buf.push(0x1B);
                    self.string_buf.push(b);
                }
                self.string_pending_esc = false;
            }
            *idx += 1;
        } else {
            if self.string_buf.len() < 65536 {
                self.string_buf.push(b);
            }
            *idx += 1;
        }
    }
}
