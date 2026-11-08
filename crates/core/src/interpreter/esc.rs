use super::c0;
use crate::event::{EventRef, KeyCode, KeyEvent, Modifiers};

/// https://vt100.net/docs/vt510-rm/chapter4.html
/// https://ecma-international.org/publications-and-standards/standards/ecma-48
pub fn interpret_esc(
    intermediate: Option<u8>,
    b: u8,
) -> Option<EventRef<'static>> {
    if intermediate.is_some() {
        return None;
    }

    let (code, modifiers) = match b {
        0x20..=0x7E => (KeyCode::Char(b as char), Modifiers::NONE),
        _ => c0(b)?,
    };

    Some(EventRef::Key(KeyEvent::new(
        code,
        Modifiers {
            alt: true,
            ..modifiers
        },
    )))
}

pub fn interpret_ss3(b: u8) -> Option<EventRef<'static>> {
    let code = match b {
        b'A' => KeyCode::Up,
        b'B' => KeyCode::Down,
        b'C' => KeyCode::Right,
        b'D' => KeyCode::Left,
        b'H' => KeyCode::Home,
        b'F' => KeyCode::End,
        b'P' => KeyCode::F(1),
        b'Q' => KeyCode::F(2),
        b'R' => KeyCode::F(3),
        b'S' => KeyCode::F(4),
        _ => return None,
    };

    Some(EventRef::Key(KeyEvent::new(code, Modifiers::NONE)))
}
