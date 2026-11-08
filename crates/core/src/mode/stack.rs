use crate::mode::Mode;

#[derive(Default, Debug, Clone)]
pub struct Modes {
    stack: Vec<Mode>,
    restoration: Vec<u8>,
}

impl Modes {
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
            restoration: Vec::new(),
        }
    }

    pub fn push(&mut self, mode: Mode) -> &[u8] {
        if !self.stack.contains(&mode) {
            self.stack.push(mode.clone());
        }
        self.eval();

        let index = self.stack.iter().rposition(|m| *m == mode).unwrap();
        self.stack[index].enable()
    }

    pub fn pop(&mut self) -> Option<Vec<u8>> {
        let mode = self.stack.pop()?;
        self.eval();
        Some(mode.disable().to_vec())
    }

    pub fn restore(&mut self) -> Vec<u8> {
        self.stack.clear();
        std::mem::take(&mut self.restoration)
    }

    fn eval(&mut self) {
        self.restoration.clear();
        for mode in self.stack.iter().rev() {
            self.restoration.extend_from_slice(mode.disable());
        }
    }
}
