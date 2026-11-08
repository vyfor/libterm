// todo: other unix platforms
pub mod linux;
pub use linux::*;

/// https://sourceware.org/cgit/glibc/tree/termios/cfmakeraw.c
pub fn raw_mode(termios: &mut Termios) {
    termios.c_iflag &=
        !(IGNBRK | BRKINT | PARMRK | ISTRIP | INLCR | IGNCR | ICRNL | IXON);
    termios.c_oflag &= !OPOST;
    termios.c_lflag &= !(ECHO | ECHONL | ICANON | ISIG | IEXTEN);
    termios.c_cflag &= !(CSIZE | PARENB);
    termios.c_cflag |= CS8;
    termios.c_cc[VMIN] = 1;
    termios.c_cc[VTIME] = 0;
}
