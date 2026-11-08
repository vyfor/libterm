use crate::op::Op;
use crate::pen::Pen;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClearType {
    All,
    Purge,
    FromCursorDown,
    FromCursorUp,
    CurrentLine,
    UntilNewLine,
}

/// ### Erase in Display
///
/// This control function erases characters from part or all of the display.
/// When you erase complete lines, they become single-height, single-width
/// lines, with all visual character attributes cleared. ED works inside or
/// outside the scrolling margins.
///
/// https://vt100.net/docs/vt510-rm/ED.html
///
/// ### Erase in Line
///
/// This control function erases characters on the line that has the cursor. EL
/// clears all character attributes from erased character positions. EL works
/// inside or outside the scrolling margins.
///
/// https://vt100.net/docs/vt510-rm/EL.html
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Clear(pub ClearType);

impl Op for Clear {
    fn encode(&self, _pen: &mut Pen, out: &mut Vec<u8>) {
        match self.0 {
            ClearType::All => out.extend_from_slice(b"\x1b[2J"),
            ClearType::Purge => out.extend_from_slice(b"\x1b[3J"),
            ClearType::FromCursorDown => out.extend_from_slice(b"\x1b[0J"),
            ClearType::FromCursorUp => out.extend_from_slice(b"\x1b[1J"),
            ClearType::CurrentLine => out.extend_from_slice(b"\x1b[2K"),
            ClearType::UntilNewLine => out.extend_from_slice(b"\x1b[0K"),
        }
    }
}
