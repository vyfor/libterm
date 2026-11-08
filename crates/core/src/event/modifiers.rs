// todo: bitwise ops
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Modifiers {
    pub shift: bool,
    pub alt: bool,
    pub ctrl: bool,
    // aka super key
    pub system: bool,
    pub hyper: bool,
    pub meta: bool,
    pub caps_lock: bool,
    pub num_lock: bool,
}

impl Modifiers {
    pub const NONE: Self = Self {
        shift: false,
        alt: false,
        ctrl: false,
        system: false,
        hyper: false,
        meta: false,
        caps_lock: false,
        num_lock: false,
    };

    pub const SHIFT: Self = Self {
        shift: true,
        ..Self::NONE
    };

    pub const ALT: Self = Self {
        alt: true,
        ..Self::NONE
    };

    pub const CTRL: Self = Self {
        ctrl: true,
        ..Self::NONE
    };

    pub const SYSTEM: Self = Self {
        system: true,
        ..Self::NONE
    };

    pub const HYPER: Self = Self {
        hyper: true,
        ..Self::NONE
    };

    pub const META: Self = Self {
        meta: true,
        ..Self::NONE
    };

    pub const CAPS_LOCK: Self = Self {
        caps_lock: true,
        ..Self::NONE
    };

    pub const NUM_LOCK: Self = Self {
        num_lock: true,
        ..Self::NONE
    };

    // https://invisible-island.net/xterm/ctlseqs/ctlseqs.html#h3-PC-Style-Function-Keys
    pub fn from_xterm(val: u32) -> Self {
        if val <= 1 {
            return Self::NONE;
        }

        let bits = val - 1;

        Self {
            shift: (bits & 1) != 0,
            alt: (bits & 2) != 0,
            ctrl: (bits & 4) != 0,
            system: false,
            meta: false,
            hyper: false,
            caps_lock: false,
            num_lock: false,
        }
    }

    /// https://sw.kovidgoyal.net/kitty/keyboard-protocol/#modifiers
    pub fn from_kitty(val: u32) -> Self {
        if val <= 1 {
            return Self::NONE;
        }

        let bits = val - 1;

        Self {
            shift: (bits & 1) != 0,
            alt: (bits & 2) != 0,
            ctrl: (bits & 4) != 0,
            system: (bits & 8) != 0,
            hyper: (bits & 16) != 0,
            meta: (bits & 32) != 0,
            caps_lock: (bits & 64) != 0,
            num_lock: (bits & 128) != 0,
        }
    }
}
