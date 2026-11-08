use crate::decimal;
use crate::op::Op;
use crate::pen::Pen;

fn len(count: u32) -> usize {
    if count == 1 {
        0
    } else {
        decimal::digits(count)
    }
}

fn count(out: &mut Vec<u8>, count: u32, fin: u8) {
    out.extend_from_slice(b"\x1b[");

    if count != 1 {
        decimal::write(out, count);
    }

    out.push(fin);
}

fn cha(out: &mut Vec<u8>, col: u16) {
    count(out, col as u32 + 1, b'G');
}

fn cup(out: &mut Vec<u8>, row: u16, col: u16) {
    out.extend_from_slice(b"\x1b[");

    if row != 0 {
        decimal::write(out, row as u32 + 1);
    }

    if col != 0 {
        out.push(b';');
        decimal::write(out, col as u32 + 1);
    }

    out.push(b'H');
}

/// ### Cursor Position
///
/// This control function moves the cursor to the specified line and column. The
/// starting point for lines and columns depends on the setting of origin mode
/// (DECOM).
///
/// https://vt100.net/docs/vt510-rm/CUP.html
/// https://vt100.net/docs/vt510-rm/CHA.html
/// https://vt100.net/docs/vt510-rm/CUF.html
/// https://vt100.net/docs/vt510-rm/CUB.html
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MoveTo {
    pub col: u16,
    pub row: u16,
}

impl MoveTo {
    pub const fn new(col: u16, row: u16) -> Self {
        Self { col, row }
    }
}

impl Op for MoveTo {
    fn encode(&self, pen: &mut Pen, out: &mut Vec<u8>) {
        let col = self.col.min(pen.max_col());
        let row = self.row.min(pen.max_row());

        if pen.col == Some(col) && pen.row == Some(row) {
            return;
        }

        if pen.row == Some(row)
            && let Some(c) = pen.col
        {
            let diff = c.abs_diff(col) as u32;
            let relative_len = 3 + len(diff);
            let cha_len = 3 + len(col as u32 + 1);

            if relative_len < cha_len {
                count(out, diff, if col > c { b'C' } else { b'D' });
            } else {
                cha(out, col);
            }

            pen.col = Some(col);
            return;
        }

        cup(out, row, col);

        pen.col = Some(col);
        pen.row = Some(row);
    }
}

/// ### Cursor Up
///
/// Moves the cursor up a specified number of lines in the same column. The
/// cursor stops at the top margin. If the cursor is already above the top
/// margin, then the cursor stops at the top line.
///
/// https://vt100.net/docs/vt510-rm/CUU.html
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MoveUp(pub u16);
impl Op for MoveUp {
    fn encode(&self, pen: &mut Pen, out: &mut Vec<u8>) {
        if self.0 == 0 {
            return;
        }

        count(out, self.0 as u32, b'A');

        if let Some(r) = pen.row.as_mut() {
            *r = r.saturating_sub(self.0);
        }
    }
}

/// ### Cursor Down
///
/// This control function moves the cursor down a specified number of lines in
/// the same column. The cursor stops at the bottom margin. If the cursor is
/// already below the bottom margin, then the cursor stops at the bottom line.
///
/// https://vt100.net/docs/vt510-rm/CUD.html
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MoveDown(pub u16);
impl Op for MoveDown {
    fn encode(&self, pen: &mut Pen, out: &mut Vec<u8>) {
        if self.0 == 0 {
            return;
        }

        count(out, self.0 as u32, b'B');

        let max = pen.max_row();
        if let Some(r) = pen.row.as_mut() {
            *r = r.saturating_add(self.0).min(max);
        }
    }
}

/// ### Cursor Forward
///
/// This control function moves the cursor to the right by a specified number of
/// columns. The cursor stops at the right border of the page.
///
/// https://vt100.net/docs/vt510-rm/CUF.html
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MoveRight(pub u16);
impl Op for MoveRight {
    fn encode(&self, pen: &mut Pen, out: &mut Vec<u8>) {
        if self.0 == 0 {
            return;
        }

        count(out, self.0 as u32, b'C');

        let max = pen.max_col();
        if let Some(c) = pen.col.as_mut() {
            *c = c.saturating_add(self.0).min(max);
        }
    }
}

/// ### Cursor Backward
///
/// This control function moves the cursor to the left by a specified number of
/// columns. The cursor stops at the left border of the page.
///
/// https://vt100.net/docs/vt510-rm/CUB.html
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MoveLeft(pub u16);
impl Op for MoveLeft {
    fn encode(&self, pen: &mut Pen, out: &mut Vec<u8>) {
        if self.0 == 0 {
            return;
        }

        count(out, self.0 as u32, b'D');

        if let Some(c) = pen.col.as_mut() {
            *c = c.saturating_sub(self.0);
        }
    }
}

/// ### Text Cursor Disable Mode
///
/// This control function makes the cursor invisible.
///
/// https://vt100.net/docs/vt510-rm/DECTCEM.html
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HideCursor;
impl Op for HideCursor {
    fn encode(&self, _pen: &mut Pen, out: &mut Vec<u8>) {
        out.extend_from_slice(b"\x1b[?25l");
    }
}

/// ### Text Cursor Enable Mode
///
/// This control function makes the cursor visible.
///
/// https://vt100.net/docs/vt510-rm/DECTCEM.html
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ShowCursor;
impl Op for ShowCursor {
    fn encode(&self, _pen: &mut Pen, out: &mut Vec<u8>) {
        out.extend_from_slice(b"\x1b[?25h");
    }
}
