use libterm_core::encoder::Encoder;
use libterm_core::event::EventRef;
use libterm_core::interpreter::Interpreter;
use libterm_core::mode::Modes;
use libterm_core::tokenizer::Tokenizer;

pub struct TerminalState {
    tokenizer: Tokenizer,
    interpreter: Interpreter,
    modes: Modes,
    encoder: Encoder,
}

impl Default for TerminalState {
    fn default() -> Self {
        Self::new()
    }
}

impl TerminalState {
    pub fn new() -> Self {
        Self {
            tokenizer: Tokenizer::new(),
            interpreter: Interpreter::new(),
            modes: Modes::new(),
            encoder: Encoder::new(),
        }
    }

    pub fn input(&mut self, bytes: &[u8], mut sink: impl FnMut(EventRef<'_>)) {
        let interpreter = &mut self.interpreter;
        let encoder = &mut self.encoder;

        self.tokenizer.feed(bytes, |token| {
            interpreter.interpret(token, |event| {
                if let EventRef::Resize { cols, rows } = event {
                    encoder.set_size(cols, rows);
                }
                sink(event);
            });
        });
    }

    pub fn has_pending(&self) -> bool {
        self.tokenizer.has_pending()
    }

    pub fn timeout(&mut self, mut sink: impl FnMut(EventRef<'_>)) {
        let interpreter = &mut self.interpreter;
        let encoder = &mut self.encoder;

        self.tokenizer.timed_out(|token| {
            interpreter.interpret(token, |event| {
                if let EventRef::Resize { cols, rows } = event {
                    encoder.set_size(cols, rows);
                }
                sink(event);
            });
        });
    }

    pub fn encoder_mut(&mut self) -> &mut Encoder {
        &mut self.encoder
    }

    pub fn modes_mut(&mut self) -> &mut Modes {
        &mut self.modes
    }
}
