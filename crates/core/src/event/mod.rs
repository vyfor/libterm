pub mod key;
pub mod modifiers;
pub mod mouse;

pub use key::{KeyCode, KeyEvent, KeyEventKind};
pub use modifiers::Modifiers;
pub use mouse::{MouseButton, MouseEvent, MouseEventKind};

use crate::token::{Token, TokenRef};

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EventRef<'a> {
    Key(KeyEvent),
    Mouse(MouseEvent),
    Paste(String),
    FocusGained,
    FocusLost,
    Resize { cols: u16, rows: u16 },
    // escape hatch for probing the terminal
    Raw(TokenRef<'a>),
}

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
    Key(KeyEvent),
    Mouse(MouseEvent),
    Paste(String),
    FocusGained,
    FocusLost,
    Resize { cols: u16, rows: u16 },
    // escape hatch for probing the terminal
    Raw(Token),
}

impl<'a> EventRef<'a> {
    pub fn into_owned(self) -> Event {
        match self {
            EventRef::Key(k) => Event::Key(k),
            EventRef::Mouse(m) => Event::Mouse(m),
            EventRef::Paste(p) => Event::Paste(p),
            EventRef::FocusGained => Event::FocusGained,
            EventRef::FocusLost => Event::FocusLost,
            EventRef::Resize { cols, rows } => Event::Resize { cols, rows },
            EventRef::Raw(t) => Event::Raw(t.to_owned()),
        }
    }
}
