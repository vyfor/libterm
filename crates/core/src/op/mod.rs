pub mod clear;
pub mod cursor;
pub mod screen;
pub mod style;
pub mod sync;
pub mod text;

pub use clear::{Clear, ClearType};
pub use cursor::{
    HideCursor, MoveDown, MoveLeft, MoveRight, MoveTo, MoveUp, ShowCursor,
};
pub use screen::{ScrollDown, ScrollUp, SetTitle};
pub use style::{ResetColor, SetStyle};
pub use sync::{BeginSync, EndSync};
pub use text::{Print, PrintChar, Raw};

use crate::pen::Pen;

pub trait Op {
    fn encode(&self, pen: &mut Pen, out: &mut Vec<u8>);
}
