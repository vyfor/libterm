pub mod stack;

pub use stack::Modes;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Mode {
    /// https://invisible-island.net/xterm/ctlseqs/ctlseqs.html#h2-The-Alternate-Screen-Buffer
    AltScreen,
    // todo: selective reporting modes
    /// https://invisible-island.net/xterm/ctlseqs/ctlseqs.html#h2-Mouse-Tracking
    MouseTracking,
    /// https://invisible-island.net/xterm/ctlseqs/ctlseqs.html#h2-Bracketed-Paste-Mode
    BracketedPaste,
    /// https://invisible-island.net/xterm/ctlseqs/ctlseqs.html#h3-FocusIn_FocusOut
    FocusEvents,
    /// https://github.com/contour-terminal/vt-extensions/blob/master/synchronized-output.md
    SynchronizedOutput,
    /// https://github.com/kovidgoyal/kitty/issues/7642
    InBandResize,
    /// https://vt100.net/docs/vt510-rm/DECTCEM.html
    CursorVisible(bool),
    /// https://sw.kovidgoyal.net/kitty/keyboard-protocol
    KittyKeyboard,
    Custom {
        enable: Vec<u8>,
        disable: Vec<u8>,
    },
}

impl Mode {
    pub fn enable(&self) -> &[u8] {
        match self {
            Mode::AltScreen => b"\x1b[?1049h",
            Mode::MouseTracking => b"\x1b[?1003h\x1b[?1006h",
            Mode::BracketedPaste => b"\x1b[?2004h",
            Mode::FocusEvents => b"\x1b[?1004h",
            Mode::SynchronizedOutput => b"\x1b[?2026h",
            Mode::InBandResize => b"\x1b[?2048h",
            Mode::CursorVisible(true) => b"\x1b[?25h",
            Mode::CursorVisible(false) => b"\x1b[?25l",
            Mode::KittyKeyboard => b"\x1b[>1u",
            Mode::Custom { enable, .. } => enable.as_slice(),
        }
    }

    pub fn disable(&self) -> &[u8] {
        match self {
            Mode::AltScreen => b"\x1b[?1049l",
            Mode::MouseTracking => b"\x1b[?1006l\x1b[?1003l",
            Mode::BracketedPaste => b"\x1b[?2004l",
            Mode::FocusEvents => b"\x1b[?1004l",
            Mode::SynchronizedOutput => b"\x1b[?2026l",
            Mode::InBandResize => b"\x1b[?2048l",
            Mode::CursorVisible(true) => b"\x1b[?25l",
            Mode::CursorVisible(false) => b"\x1b[?25h",
            Mode::KittyKeyboard => b"\x1b[<u",
            Mode::Custom { disable, .. } => disable.as_slice(),
        }
    }
}
