pub mod color;
pub mod style;

pub use color::Color;
pub use style::{Attributes, Style};

#[derive(Debug, Clone, Default)]
pub struct Pen {
    pub col: Option<u16>,
    pub row: Option<u16>,
    pub style: Option<Style>,
    pub size: Option<(u16, u16)>,
}

impl Pen {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_size(&mut self, cols: u16, rows: u16) {
        let size = (cols > 0 && rows > 0).then_some((cols, rows));
        if self.size != size {
            self.size = size;
            self.invalidate_pos();
        }
    }

    pub fn invalidate_pos(&mut self) {
        self.col = None;
        self.row = None;
    }

    pub fn invalidate(&mut self) {
        self.invalidate_pos();
        self.style = None;
    }

    pub fn max_col(&self) -> u16 {
        match self.size {
            Some((cols, _)) => cols.saturating_sub(1),
            None => u16::MAX,
        }
    }

    pub fn max_row(&self) -> u16 {
        match self.size {
            Some((_, rows)) => rows.saturating_sub(1),
            None => u16::MAX,
        }
    }

    pub fn advance(&mut self) {
        let Some(col) = self.col else { return };
        if col >= self.max_col() {
            self.invalidate_pos();
        } else {
            self.col = Some(col + 1);
        }
    }
}
