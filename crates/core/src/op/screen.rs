use crate::decimal;
use crate::op::Op;
use crate::pen::Pen;

/// ### Pan Down
///
/// This control function moves the user window down a specified number of lines
/// in page memory.
///
/// https://vt100.net/docs/vt510-rm/SU.html
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ScrollUp(pub u16);

impl Op for ScrollUp {
    fn encode(&self, _pen: &mut Pen, out: &mut Vec<u8>) {
        if self.0 == 0 {
            return;
        }
        out.extend_from_slice(b"\x1b[");
        decimal::write(out, self.0 as u32);
        out.push(b'S');
    }
}

/// ### Pan Up
///
/// This control function moves the user window up a specified number of lines
/// in page memory.
///
/// https://vt100.net/docs/vt510-rm/SD.html
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ScrollDown(pub u16);

impl Op for ScrollDown {
    fn encode(&self, _pen: &mut Pen, out: &mut Vec<u8>) {
        if self.0 == 0 {
            return;
        }
        out.extend_from_slice(b"\x1b[");
        decimal::write(out, self.0 as u32);
        out.push(b'T');
    }
}

/// ### Window Title
///
/// The following commands allows the application to set the title of the
/// console window to the given string parameter. The string must be less than
/// 255 characters to be accepted.
///
/// https://learn.microsoft.com/en-us/windows/console/console-virtual-terminal-sequences#window-title
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SetTitle<'a>(pub &'a str);

impl<'a> Op for SetTitle<'a> {
    fn encode(&self, _pen: &mut Pen, out: &mut Vec<u8>) {
        out.extend_from_slice(b"\x1b]0;");
        let mut buf = [0u8; 4];
        for ch in self.0.chars().take(254).filter(|ch| !ch.is_control()) {
            out.extend_from_slice(ch.encode_utf8(&mut buf).as_bytes());
        }
        out.push(0x07);
    }
}
