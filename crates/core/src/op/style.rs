use crate::decimal;
use crate::op::Op;
use crate::pen::{Color, Pen, Style};

/// ### Select Graphic Rendition
///
/// This control function selects one or more character attributes at the same
/// time.
///
/// https://vt100.net/docs/vt510-rm/SGR.html
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SetStyle(pub Style);

impl SetStyle {
    pub const fn new(style: Style) -> Self {
        Self(style)
    }
}

struct Sgr<'a> {
    out: &'a mut Vec<u8>,
    first: bool,
}

impl<'a> Sgr<'a> {
    fn start(out: &'a mut Vec<u8>) -> Self {
        out.extend_from_slice(b"\x1b[");
        Self { out, first: true }
    }

    fn param(&mut self, value: u32) {
        if !self.first {
            self.out.push(b';');
        }
        self.first = false;
        decimal::write(self.out, value);
    }

    fn color(&mut self, color: Color, is_fg: bool) {
        let base = if is_fg { 30 } else { 40 };
        let bright = if is_fg { 90 } else { 100 };
        match color {
            Color::Reset => self.param(if is_fg { 39 } else { 49 }),
            Color::Black => self.param(base),
            Color::Red => self.param(base + 1),
            Color::Green => self.param(base + 2),
            Color::Yellow => self.param(base + 3),
            Color::Blue => self.param(base + 4),
            Color::Magenta => self.param(base + 5),
            Color::Cyan => self.param(base + 6),
            Color::White => self.param(base + 7),
            Color::BrightBlack => self.param(bright),
            Color::BrightRed => self.param(bright + 1),
            Color::BrightGreen => self.param(bright + 2),
            Color::BrightYellow => self.param(bright + 3),
            Color::BrightBlue => self.param(bright + 4),
            Color::BrightMagenta => self.param(bright + 5),
            Color::BrightCyan => self.param(bright + 6),
            Color::BrightWhite => self.param(bright + 7),
            Color::Ansi(n) => {
                self.param(if is_fg { 38 } else { 48 });
                self.param(5);
                self.param(n as u32);
            }
            Color::Rgb(r, g, b) => {
                self.param(if is_fg { 38 } else { 48 });
                self.param(2);
                self.param(r as u32);
                self.param(g as u32);
                self.param(b as u32);
            }
        }
    }

    fn finish(self) -> bool {
        if self.first {
            self.out.truncate(self.out.len() - 2);
            false
        } else {
            self.out.push(b'm');
            true
        }
    }
}

impl Op for SetStyle {
    fn encode(&self, pen: &mut Pen, out: &mut Vec<u8>) {
        let target = self.0;
        if pen.style == Some(target) {
            return;
        }

        let curr = pen.style.unwrap_or_default();
        let reset = pen.style.is_none()
            || (curr.attributes.bold && !target.attributes.bold)
            || (curr.attributes.dim && !target.attributes.dim)
            || (curr.attributes.italic && !target.attributes.italic)
            || (curr.attributes.underline && !target.attributes.underline)
            || (curr.attributes.blink && !target.attributes.blink)
            || (curr.attributes.reverse && !target.attributes.reverse)
            || (curr.attributes.hidden && !target.attributes.hidden)
            || (curr.attributes.strike && !target.attributes.strike);

        let mut sgr = Sgr::start(out);

        if reset {
            sgr.param(0);
        }

        let attr = |sgr: &mut Sgr<'_>, was: bool, now: bool, code: u32| {
            if now && (reset || !was) {
                sgr.param(code);
            }
        };

        attr(&mut sgr, curr.attributes.bold, target.attributes.bold, 1);
        attr(&mut sgr, curr.attributes.dim, target.attributes.dim, 2);
        attr(
            &mut sgr,
            curr.attributes.italic,
            target.attributes.italic,
            3,
        );
        attr(
            &mut sgr,
            curr.attributes.underline,
            target.attributes.underline,
            4,
        );
        attr(&mut sgr, curr.attributes.blink, target.attributes.blink, 5);
        attr(
            &mut sgr,
            curr.attributes.reverse,
            target.attributes.reverse,
            7,
        );
        attr(
            &mut sgr,
            curr.attributes.hidden,
            target.attributes.hidden,
            8,
        );
        attr(
            &mut sgr,
            curr.attributes.strike,
            target.attributes.strike,
            9,
        );

        if reset {
            if let Some(color) = target.fg {
                sgr.color(color, true);
            }
            if let Some(color) = target.bg {
                sgr.color(color, false);
            }
        } else {
            if curr.fg != target.fg {
                match target.fg {
                    Some(color) => sgr.color(color, true),
                    None => sgr.param(39),
                }
            }

            if curr.bg != target.bg {
                match target.bg {
                    Some(color) => sgr.color(color, false),
                    None => sgr.param(49),
                }
            }
        }

        sgr.finish();
        pen.style = Some(target);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ResetColor;

impl Op for ResetColor {
    fn encode(&self, pen: &mut Pen, out: &mut Vec<u8>) {
        let needs_reset = match pen.style {
            Some(style) => style.fg.is_some() || style.bg.is_some(),
            None => true,
        };

        if needs_reset {
            out.extend_from_slice(b"\x1b[39;49m");
            if let Some(style) = pen.style.as_mut() {
                style.fg = None;
                style.bg = None;
            }
        }
    }
}
