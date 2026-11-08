pub use libterm_core as core;
pub use libterm_sys as sys;

pub mod state;
pub mod terminal;

pub use state::TerminalState;
pub use terminal::Terminal;

pub use libterm_core::event::{
    Event, EventRef, KeyCode, KeyEvent, KeyEventKind, Modifiers, MouseButton,
    MouseEvent, MouseEventKind,
};
pub use libterm_core::mode::Mode;
pub use libterm_core::op::{
    BeginSync, Clear, ClearType, EndSync, HideCursor, MoveDown, MoveLeft,
    MoveRight, MoveTo, MoveUp, Op, Print, PrintChar, Raw, ResetColor,
    ScrollDown, ScrollUp, SetStyle, SetTitle, ShowCursor,
};
pub use libterm_core::pen::{Attributes, Color, Pen, Style};
pub use libterm_sys::{PollResult, RawGuard, Tty, Waker};
