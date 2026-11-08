pub(crate) mod csi;
pub(crate) mod esc;

use crate::event::{EventRef, KeyCode, KeyEvent, Modifiers};
use crate::token::TokenRef;

// https://en.wikipedia.org/wiki/C0_and_C1_control_codes
// https://www.columbia.edu/kermit/ascii.html
fn c0(b: u8) -> Option<(KeyCode, Modifiers)> {
    match b {
        0x00 => Some((KeyCode::Null, Modifiers::NONE)),
        0x08 | 0x7F => Some((KeyCode::Backspace, Modifiers::NONE)),
        0x09 => Some((KeyCode::Tab, Modifiers::NONE)),
        0x0A | 0x0D => Some((KeyCode::Enter, Modifiers::NONE)),
        0x1B => Some((KeyCode::Esc, Modifiers::NONE)),
        1..=26 => {
            Some((KeyCode::Char((b'a' + b - 1) as char), Modifiers::CTRL))
        }
        0x1C..=0x1F => {
            Some((KeyCode::Char((b + 0x40) as char), Modifiers::CTRL))
        }
        _ => None,
    }
}

pub struct Interpreter {
    in_paste: bool,
    // pastes can be split across multiple tokens
    paste_buf: Vec<u8>,
}

impl Default for Interpreter {
    fn default() -> Self {
        Self::new()
    }
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            in_paste: false,
            paste_buf: Vec::new(),
        }
    }

    pub fn interpret<'a>(
        &mut self,
        token: TokenRef<'a>,
        mut sink: impl FnMut(EventRef<'a>),
    ) {
        // https://invisible-island.net/xterm/ctlseqs/ctlseqs.html#h2-Bracketed-Paste-Mode
        if let TokenRef::Csi {
            prefix,
            intermediate,
            final_byte,
            params,
        } = &token
            && prefix.is_none()
            && intermediate.is_none()
            && *final_byte == b'~'
        {
            if *params == b"200" {
                self.in_paste = true;
                self.paste_buf.clear();
                return;
            } else if *params == b"201" {
                self.in_paste = false;
                let content = std::mem::take(&mut self.paste_buf);
                sink(EventRef::Paste(
                    String::from_utf8_lossy(&content).into_owned(),
                ));
                return;
            }
        }

        if self.in_paste {
            self.append_paste(token);
            return;
        }

        match token {
            TokenRef::Text(s) => {
                for ch in s.chars() {
                    sink(EventRef::Key(KeyEvent::new(
                        KeyCode::Char(ch),
                        Modifiers::NONE,
                    )));
                }
            }
            // https://en.wikipedia.org/wiki/C0_and_C1_control_codes
            TokenRef::C0(b) => match c0(b) {
                Some((code, modifiers)) => {
                    sink(EventRef::Key(KeyEvent::new(code, modifiers)))
                }
                None => sink(EventRef::Raw(TokenRef::C0(b))),
            },
            TokenRef::Esc {
                intermediate,
                final_byte,
            } => {
                if let Some(event) =
                    esc::interpret_esc(intermediate, final_byte)
                {
                    sink(event);
                } else {
                    sink(EventRef::Raw(TokenRef::Esc {
                        intermediate,
                        final_byte,
                    }));
                }
            }
            TokenRef::Ss3(b) => {
                if let Some(event) = esc::interpret_ss3(b) {
                    sink(event);
                } else {
                    sink(EventRef::Raw(TokenRef::Ss3(b)));
                }
            }
            TokenRef::Csi {
                prefix,
                params,
                intermediate,
                final_byte,
            } => {
                if let Some(event) =
                    csi::interpret_csi(prefix, params, intermediate, final_byte)
                {
                    sink(event);
                } else {
                    sink(EventRef::Raw(TokenRef::Csi {
                        prefix,
                        params,
                        intermediate,
                        final_byte,
                    }));
                }
            }
            // https://invisible-island.net/xterm/ctlseqs/ctlseqs.html#h3-Operating-System-Commands
            TokenRef::Osc(b) => {
                sink(EventRef::Raw(TokenRef::Osc(b)));
            }
            // https://invisible-island.net/xterm/ctlseqs/ctlseqs.html#h3-Application-Program-Command-functions
            TokenRef::Apc(b) => {
                sink(EventRef::Raw(TokenRef::Apc(b)));
            }
            // https://invisible-island.net/xterm/ctlseqs/ctlseqs.html#h3-Device-Control-functions
            TokenRef::Dcs(b) => {
                sink(EventRef::Raw(TokenRef::Dcs(b)));
            }
            // https://invisible-island.net/xterm/ctlseqs/ctlseqs.html#h3-Privacy-Message
            TokenRef::Pm(b) => {
                sink(EventRef::Raw(TokenRef::Pm(b)));
            }
            // ECMA-48 @ 8.3.128
            TokenRef::Sos(b) => {
                sink(EventRef::Raw(TokenRef::Sos(b)));
            }
        }
    }

    fn append_paste(&mut self, token: TokenRef<'_>) {
        match token {
            TokenRef::Text(s) => self.paste_buf.extend_from_slice(s.as_bytes()),
            TokenRef::C0(b) => self.paste_buf.push(b),
            TokenRef::Esc {
                intermediate,
                final_byte,
            } => {
                self.paste_buf.push(0x1B);
                if let Some(intermediate) = intermediate {
                    self.paste_buf.push(intermediate);
                }
                self.paste_buf.push(final_byte);
            }
            TokenRef::Ss3(b) => {
                self.paste_buf.extend_from_slice(b"\x1bO");
                self.paste_buf.push(b);
            }
            TokenRef::Csi {
                prefix,
                params,
                intermediate,
                final_byte,
            } => {
                self.paste_buf.extend_from_slice(b"\x1b[");
                if let Some(prefix) = prefix {
                    self.paste_buf.push(prefix);
                }
                self.paste_buf.extend_from_slice(params);
                if let Some(intermediate) = intermediate {
                    self.paste_buf.push(intermediate);
                }
                self.paste_buf.push(final_byte);
            }
            TokenRef::Osc(b) => {
                self.paste_buf.extend_from_slice(b"\x1b]");
                self.paste_buf.extend_from_slice(b);
                self.paste_buf.extend_from_slice(b"\x1b\\");
            }
            TokenRef::Apc(b) => {
                self.paste_buf.extend_from_slice(b"\x1b_");
                self.paste_buf.extend_from_slice(b);
                self.paste_buf.extend_from_slice(b"\x1b\\");
            }
            TokenRef::Dcs(b) => {
                self.paste_buf.extend_from_slice(b"\x1bP");
                self.paste_buf.extend_from_slice(b);
                self.paste_buf.extend_from_slice(b"\x1b\\");
            }
            TokenRef::Pm(b) => {
                self.paste_buf.extend_from_slice(b"\x1b^");
                self.paste_buf.extend_from_slice(b);
                self.paste_buf.extend_from_slice(b"\x1b\\");
            }
            TokenRef::Sos(b) => {
                self.paste_buf.extend_from_slice(b"\x1bX");
                self.paste_buf.extend_from_slice(b);
                self.paste_buf.extend_from_slice(b"\x1b\\");
            }
        }
    }
}
