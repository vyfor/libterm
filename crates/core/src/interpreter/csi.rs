use crate::event::{
    EventRef, KeyCode, KeyEvent, KeyEventKind, Modifiers, MouseButton,
    MouseEvent, MouseEventKind,
};

// todo: fully cover kitty's keyboard protocol
/// https://invisible-island.net/xterm/ecma-48-parameter-format.html
/// https://vt100.net/docs/vt510-rm/chapter4.html
/// https://invisible-island.net/xterm/ctlseqs/ctlseqs.html#h3-Focus-Reporting
/// https://invisible-island.net/xterm/ctlseqs/ctlseqs.html#h3-Mouse-Tracking
/// https://sw.kovidgoyal.net/kitty/keyboard-protocol/
pub fn interpret_csi(
    prefix: Option<u8>,
    params_bytes: &[u8],
    intermediate: Option<u8>,
    final_byte: u8,
) -> Option<EventRef<'static>> {
    let params = params(params_bytes);

    if prefix == Some(b'<')
        && (final_byte == b'M' || final_byte == b'm')
        && params.len() >= 3
    {
        let cb = params[0].unwrap_or(0);
        let cx = params[1].unwrap_or(1).saturating_sub(1) as u16;
        let cy = params[2].unwrap_or(1).saturating_sub(1) as u16;

        let shift = (cb & 4) != 0;
        let alt = (cb & 8) != 0;
        let ctrl = (cb & 16) != 0;
        let modifiers = Modifiers {
            shift,
            alt,
            ctrl,
            ..Modifiers::NONE
        };

        let is_release = final_byte == b'm';
        let motion = (cb & 32) != 0;
        let wheel = (cb & 64) != 0;
        let extended = (cb & 128) != 0;

        let kind = if wheel {
            match cb & 3 {
                0 => MouseEventKind::ScrollUp,
                1 => MouseEventKind::ScrollDown,
                2 => MouseEventKind::ScrollLeft,
                _ => MouseEventKind::ScrollRight,
            }
        } else if motion && !extended && cb & 3 == 3 {
            MouseEventKind::Moved
        } else if motion {
            MouseEventKind::Drag(mouse_button(cb))
        } else if is_release {
            MouseEventKind::Up(mouse_button(cb))
        } else {
            MouseEventKind::Down(mouse_button(cb))
        };

        return Some(EventRef::Mouse(MouseEvent {
            kind,
            col: cx,
            row: cy,
            modifiers,
        }));
    }

    if final_byte == b'u'
        && prefix.is_none()
        && let Some(first) = params.first().copied().flatten()
    {
        let code = match first {
            13 => KeyCode::Enter,
            9 => KeyCode::Tab,
            127 | 8 => KeyCode::Backspace,
            27 => KeyCode::Esc,
            57361 => KeyCode::PrintScreen,
            57362 => KeyCode::Pause,
            57363 => KeyCode::Menu,
            57376..=57398 => KeyCode::F((first - 57376 + 13) as u8),
            57344..=63743 => return None,
            cp => {
                if let Some(ch) = char::from_u32(cp) {
                    KeyCode::Char(ch)
                } else {
                    return None;
                }
            }
        };

        let mut modifiers = Modifiers::NONE;
        let mut kind = KeyEventKind::Press;

        if params.len() > 1
            && let Some(m) = params[1]
        {
            modifiers = Modifiers::from_kitty(m);
        }

        if params.len() > 2
            && let Some(k) = params[2]
        {
            kind = match k {
                1 => KeyEventKind::Press,
                2 => KeyEventKind::Repeat,
                3 => KeyEventKind::Release,
                _ => KeyEventKind::Press,
            };
        }

        return Some(EventRef::Key(KeyEvent {
            code,
            modifiers,
            kind,
        }));
    }

    if prefix.is_none() && intermediate.is_none() {
        match final_byte {
            b'A' | b'B' | b'C' | b'D' | b'H' | b'F' | b'P' | b'Q' | b'S' => {
                let code = match final_byte {
                    b'A' => KeyCode::Up,
                    b'B' => KeyCode::Down,
                    b'C' => KeyCode::Right,
                    b'D' => KeyCode::Left,
                    b'H' => KeyCode::Home,
                    b'F' => KeyCode::End,
                    b'P' => KeyCode::F(1),
                    b'Q' => KeyCode::F(2),
                    b'S' => KeyCode::F(4),
                    _ => unreachable!(),
                };

                let modifiers = if params.len() >= 2 {
                    Modifiers::from_xterm(params[1].unwrap_or(1))
                } else {
                    Modifiers::NONE
                };

                return Some(EventRef::Key(KeyEvent::new(code, modifiers)));
            }
            b'Z' if params.is_empty() => {
                return Some(EventRef::Key(KeyEvent::new(
                    KeyCode::Tab,
                    Modifiers::SHIFT,
                )));
            }
            b'I' if params.is_empty() => return Some(EventRef::FocusGained),
            b'O' if params.is_empty() => return Some(EventRef::FocusLost),
            b'~' => {
                if let Some(num) = params.first().copied().flatten() {
                    let modifiers = if params.len() >= 2 {
                        Modifiers::from_xterm(params[1].unwrap_or(1))
                    } else {
                        Modifiers::NONE
                    };

                    let code = match num {
                        1 | 7 => KeyCode::Home,
                        2 => KeyCode::Insert,
                        3 => KeyCode::Delete,
                        4 | 8 => KeyCode::End,
                        5 => KeyCode::PageUp,
                        6 => KeyCode::PageDown,
                        11 => KeyCode::F(1),
                        12 => KeyCode::F(2),
                        13 => KeyCode::F(3),
                        14 => KeyCode::F(4),
                        15 => KeyCode::F(5),
                        17 => KeyCode::F(6),
                        18 => KeyCode::F(7),
                        19 => KeyCode::F(8),
                        20 => KeyCode::F(9),
                        21 => KeyCode::F(10),
                        23 => KeyCode::F(11),
                        24 => KeyCode::F(12),
                        25 => KeyCode::F(13),
                        26 => KeyCode::F(14),
                        28 => KeyCode::F(15),
                        29 => KeyCode::Menu,
                        31..=34 => KeyCode::F((num - 31 + 17) as u8),
                        _ => return None,
                    };

                    return Some(EventRef::Key(KeyEvent::new(code, modifiers)));
                }
            }
            b't' if params.len() >= 3 && params[0] == Some(48) => {
                let rows = params[1].unwrap_or(0) as u16;
                let cols = params[2].unwrap_or(0) as u16;
                return Some(EventRef::Resize { cols, rows });
            }
            _ => {}
        }
    }

    None
}

fn mouse_button(cb: u32) -> MouseButton {
    match ((cb & 128) != 0, cb & 3) {
        (false, 0) => MouseButton::Left,
        (false, 1) => MouseButton::Middle,
        (false, 2) => MouseButton::Right,
        (false, n) => MouseButton::Other(n as u8),
        (true, n) => MouseButton::Other(8 + n as u8),
    }
}

fn params(prms: &[u8]) -> Vec<Option<u32>> {
    if prms.is_empty() {
        return Vec::new();
    }

    let mut res = Vec::new();
    let mut curr: Option<u32> = None;

    for &b in prms {
        match b {
            b'0'..=b'9' => {
                let val = (b - b'0') as u32;
                curr = Some(
                    curr.unwrap_or(0).saturating_mul(10).saturating_add(val),
                );
            }
            b';' | b':' => {
                res.push(curr);
                curr = None;
            }
            _ => {}
        }
    }

    res.push(curr);
    res
}
