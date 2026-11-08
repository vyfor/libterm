use crate::op::Op;
use crate::pen::Pen;

pub struct Encoder {
    pen: Pen,
    buffer: Vec<u8>,
}

impl Default for Encoder {
    fn default() -> Self {
        Self::new()
    }
}

impl Encoder {
    pub fn new() -> Self {
        Self {
            pen: Pen::new(),
            buffer: Vec::with_capacity(4096),
        }
    }

    pub fn emit(&mut self, op: impl Op) {
        op.encode(&mut self.pen, &mut self.buffer);
    }

    pub fn set_size(&mut self, cols: u16, rows: u16) {
        self.pen.set_size(cols, rows);
    }

    pub fn pending(&self) -> &[u8] {
        &self.buffer
    }

    pub fn commit(&mut self) {
        self.buffer.clear();
    }

    pub fn invalidate(&mut self) {
        self.buffer.clear();
        self.pen.invalidate();
    }
}
