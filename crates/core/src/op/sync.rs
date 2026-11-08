use crate::op::Op;
use crate::pen::Pen;

/// ### Synchronized Output
///
/// When rendering the screen of the terminal, the Emulator usually iterates
/// through each visible grid cell and renders its current state. With
/// applications updating the screen a at higher frequency this can cause
/// tearing.
///
/// This mode attempts to mitigate that.
///
/// When the synchronization mode is enabled following render calls will keep
/// rendering the last rendered state. The terminal Emulator keeps processing
/// incoming text and sequences. When the synchronized update mode is disabled
/// again the renderer may fetch the latest screen buffer state again,
/// effectively avoiding the tearing effect by unintentionally rendering in the
/// middle a of an application screen update.
///
/// https://github.com/contour-terminal/vt-extensions/blob/master/synchronized-output.md
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BeginSync;

impl Op for BeginSync {
    fn encode(&self, _pen: &mut Pen, out: &mut Vec<u8>) {
        out.extend_from_slice(b"\x1b[?2026h");
    }
}

/// ### Synchronized Output
///
/// When rendering the screen of the terminal, the Emulator usually iterates
/// through each visible grid cell and renders its current state. With
/// applications updating the screen a at higher frequency this can cause
/// tearing.
///
/// This mode attempts to mitigate that.
///
/// When the synchronization mode is enabled following render calls will keep
/// rendering the last rendered state. The terminal Emulator keeps processing
/// incoming text and sequences. When the synchronized update mode is disabled
/// again the renderer may fetch the latest screen buffer state again,
/// effectively avoiding the tearing effect by unintentionally rendering in the
/// middle a of an application screen update.
///
/// https://github.com/contour-terminal/vt-extensions/blob/master/synchronized-output.md
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EndSync;

impl Op for EndSync {
    fn encode(&self, _pen: &mut Pen, out: &mut Vec<u8>) {
        out.extend_from_slice(b"\x1b[?2026l");
    }
}
