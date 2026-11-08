use crate::op::Op;
use crate::pen::Pen;

fn track(pen: &mut Pen, c: char) {
    match c {
        c if c.is_ascii_graphic() || c == ' ' => pen.advance(),
        '\r' => {
            if pen.row.is_some() {
                pen.col = Some(0);
            }
        }
        '\n' => {
            pen.col = None;
            let max = pen.max_row();
            if let Some(row) = pen.row.as_mut() {
                *row = row.saturating_add(1).min(max);
            }
        }
        _ => pen.invalidate_pos(),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Print<'a>(pub &'a str);

impl<'a> Op for Print<'a> {
    fn encode(&self, pen: &mut Pen, out: &mut Vec<u8>) {
        out.extend_from_slice(self.0.as_bytes());

        for c in self.0.chars() {
            track(pen, c);
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PrintChar(pub char);

impl Op for PrintChar {
    fn encode(&self, pen: &mut Pen, out: &mut Vec<u8>) {
        let mut buf = [0u8; 4];
        let encoded = self.0.encode_utf8(&mut buf);
        out.extend_from_slice(encoded.as_bytes());

        track(pen, self.0);
    }
}

// escape hatch for probing the terminal
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Raw<'a>(pub &'a [u8]);

impl<'a> Op for Raw<'a> {
    fn encode(&self, pen: &mut Pen, out: &mut Vec<u8>) {
        out.extend_from_slice(self.0);
        pen.invalidate();
    }
}
