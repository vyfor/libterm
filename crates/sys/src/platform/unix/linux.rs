//! https://www.linusakesson.net/programming/tty/
//! https://docs.kernel.org/driver-api/tty/index.html
//! https://man7.org/linux/man-pages/man3/termios.3.html
//! https://man7.org/linux/man-pages/man7/termios.7.html
//! https://man7.org/linux/man-pages/man3/cfmakeraw.3.html
//! https://man7.org/linux/man-pages/man4/console_codes.4.html
//! https://man7.org/linux/man-pages/man4/tty.4.html
//! https://man7.org/linux/man-pages/man7/pty.7.html
//! https://man7.org/linux/man-pages/man2/eventfd.2.html
//! https://man7.org/linux/man-pages/man4/tty_ioctl.4.html

/// https://sourceware.org/cgit/glibc/tree/sysdeps/unix/sysv/linux/bits/termios.h
pub type Tcflag = u32;

/// https://sourceware.org/cgit/glibc/tree/sysdeps/unix/sysv/linux/bits/termios.h
pub type Speed = u32;

/// https://sourceware.org/cgit/glibc/tree/sysdeps/unix/sysv/linux/bits/termios.h
pub type Cc = u8;

/// https://sourceware.org/cgit/glibc/tree/sysdeps/unix/sysv/linux/bits/termios-struct.h
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Termios {
    pub c_iflag: Tcflag,
    pub c_oflag: Tcflag,
    pub c_cflag: Tcflag,
    pub c_lflag: Tcflag,
    pub c_line: Cc,
    pub c_cc: [Cc; 32],
    pub c_ispeed: Speed,
    pub c_ospeed: Speed,
}

/// https://sourceware.org/cgit/glibc/tree/sysdeps/unix/sysv/linux/bits/ioctl-types.h
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Winsize {
    pub ws_row: u16,
    pub ws_col: u16,
    pub ws_xpixel: u16,
    pub ws_ypixel: u16,
}

/// https://sourceware.org/cgit/glibc/tree/io/sys/poll.h
#[repr(C)]
pub struct Pollfd {
    pub fd: i32,
    pub events: i16,
    pub revents: i16,
}

/// https://git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git/tree/include/uapi/asm-generic/ioctls.h
pub const TIOCGWINSZ: core::ffi::c_ulong = 0x5413;

/// https://sourceware.org/cgit/glibc/tree/sysdeps/unix/sysv/linux/bits/termios-tcflow.h
pub const TCSANOW: i32 = 0;

/// https://sourceware.org/cgit/glibc/tree/sysdeps/unix/sysv/linux/bits/poll.h
pub const POLLIN: i16 = 0x0001;

/// https://sourceware.org/cgit/glibc/tree/sysdeps/unix/sysv/linux/bits/poll.h
pub const POLLOUT: i16 = 0x0004;

/// https://sourceware.org/cgit/glibc/tree/sysdeps/unix/sysv/linux/bits/poll.h
pub const POLLERR: i16 = 0x0008;

/// https://sourceware.org/cgit/glibc/tree/sysdeps/unix/sysv/linux/bits/poll.h
pub const POLLHUP: i16 = 0x0010;

/// https://sourceware.org/cgit/glibc/tree/sysdeps/unix/sysv/linux/bits/poll.h
pub const POLLNVAL: i16 = 0x0020;

/// https://sourceware.org/cgit/glibc/tree/sysdeps/unix/sysv/linux/bits/eventfd.h
pub const EFD_CLOEXEC: i32 = 0x80000;

/// https://sourceware.org/cgit/glibc/tree/sysdeps/unix/sysv/linux/bits/eventfd.h
pub const EFD_NONBLOCK: i32 = 0x800;

/// https://sourceware.org/cgit/glibc/tree/sysdeps/unix/sysv/linux/bits/fcntl-linux.h
pub const O_RDWR: i32 = 2;

/// https://sourceware.org/cgit/glibc/tree/sysdeps/unix/sysv/linux/bits/fcntl-linux.h
pub const O_NONBLOCK: i32 = 0x800;

/// https://sourceware.org/cgit/glibc/tree/sysdeps/unix/sysv/linux/bits/fcntl-linux.h
/// https://sourceware.org/cgit/glibc/tree/sysdeps/unix/sysv/linux/bits/cloexec.h
pub const O_CLOEXEC: i32 = 0x80000;

/// https://sourceware.org/cgit/glibc/tree/sysdeps/unix/sysv/linux/bits/fcntl-linux.h
pub const F_GETFL: i32 = 3;

/// https://sourceware.org/cgit/glibc/tree/sysdeps/unix/sysv/linux/bits/fcntl-linux.h
pub const F_SETFL: i32 = 4;

/// https://sourceware.org/cgit/glibc/tree/sysdeps/unix/sysv/linux/bits/termios-c_iflag.h
pub const IGNBRK: Tcflag = 0x1;

/// https://sourceware.org/cgit/glibc/tree/sysdeps/unix/sysv/linux/bits/termios-c_iflag.h
pub const BRKINT: Tcflag = 0x2;

/// https://sourceware.org/cgit/glibc/tree/sysdeps/unix/sysv/linux/bits/termios-c_iflag.h
pub const PARMRK: Tcflag = 0x8;

/// https://sourceware.org/cgit/glibc/tree/sysdeps/unix/sysv/linux/bits/termios-c_iflag.h
pub const ISTRIP: Tcflag = 0x20;

/// https://sourceware.org/cgit/glibc/tree/sysdeps/unix/sysv/linux/bits/termios-c_iflag.h
pub const INLCR: Tcflag = 0x40;

/// https://sourceware.org/cgit/glibc/tree/sysdeps/unix/sysv/linux/bits/termios-c_iflag.h
pub const IGNCR: Tcflag = 0x80;

/// https://sourceware.org/cgit/glibc/tree/sysdeps/unix/sysv/linux/bits/termios-c_iflag.h
pub const ICRNL: Tcflag = 0x100;

/// https://sourceware.org/cgit/glibc/tree/sysdeps/unix/sysv/linux/bits/termios-c_iflag.h
pub const IXON: Tcflag = 0x400;

/// https://sourceware.org/cgit/glibc/tree/sysdeps/unix/sysv/linux/bits/termios-c_oflag.h
pub const OPOST: Tcflag = 0x1;

/// https://sourceware.org/cgit/glibc/tree/sysdeps/unix/sysv/linux/bits/termios-c_lflag.h
pub const ECHO: Tcflag = 0x8;

/// https://sourceware.org/cgit/glibc/tree/sysdeps/unix/sysv/linux/bits/termios-c_lflag.h
pub const ECHONL: Tcflag = 0x40;

/// https://sourceware.org/cgit/glibc/tree/sysdeps/unix/sysv/linux/bits/termios-c_lflag.h
pub const ICANON: Tcflag = 0x2;

/// https://sourceware.org/cgit/glibc/tree/sysdeps/unix/sysv/linux/bits/termios-c_lflag.h
pub const ISIG: Tcflag = 0x1;

/// https://sourceware.org/cgit/glibc/tree/sysdeps/unix/sysv/linux/bits/termios-c_lflag.h
pub const IEXTEN: Tcflag = 0x8000;

/// https://sourceware.org/cgit/glibc/tree/sysdeps/unix/sysv/linux/bits/termios-c_cflag.h
pub const CSIZE: Tcflag = 0x30;

/// https://sourceware.org/cgit/glibc/tree/sysdeps/unix/sysv/linux/bits/termios-c_cflag.h
pub const PARENB: Tcflag = 0x100;

/// https://sourceware.org/cgit/glibc/tree/sysdeps/unix/sysv/linux/bits/termios-c_cflag.h
pub const CS8: Tcflag = 0x30;

/// https://sourceware.org/cgit/glibc/tree/sysdeps/unix/sysv/linux/bits/termios-c_cc.h
pub const VMIN: usize = 6;

/// https://sourceware.org/cgit/glibc/tree/sysdeps/unix/sysv/linux/bits/termios-c_cc.h
pub const VTIME: usize = 5;

/// https://sourceware.org/cgit/glibc/tree/sysdeps/unix/sysv/linux/bits/signum-arch.h
pub const SIGWINCH: i32 = 28;

/// https://sourceware.org/cgit/glibc/tree/signal/signal.h
pub type Sighandler = Option<unsafe extern "C" fn(i32)>;

/// https://sourceware.org/cgit/glibc/tree/bits/signum-generic.h
pub const SIG_DFL: Sighandler = None;

#[link(name = "c")]
unsafe extern "C" {
    /// https://sourceware.org/cgit/glibc/tree/termios/termios.h
    pub fn tcgetattr(fd: i32, termios_p: *mut Termios) -> i32;
    /// https://sourceware.org/cgit/glibc/tree/termios/termios.h
    pub fn tcsetattr(
        fd: i32,
        optional_actions: i32,
        termios_p: *const Termios,
    ) -> i32;
    /// https://sourceware.org/cgit/glibc/tree/misc/sys/ioctl.h
    pub fn ioctl(fd: i32, request: core::ffi::c_ulong, ...) -> i32;
    /// https://sourceware.org/cgit/glibc/tree/io/sys/poll.h
    pub fn poll(fds: *mut Pollfd, nfds: usize, timeout: i32) -> i32;
    /// https://sourceware.org/cgit/glibc/tree/io/fcntl.h
    pub fn open(pathname: *const core::ffi::c_char, flags: i32, ...) -> i32;
    /// https://sourceware.org/cgit/glibc/tree/posix/unistd.h
    pub fn close(fd: i32) -> i32;
    /// https://sourceware.org/cgit/glibc/tree/posix/unistd.h
    pub fn read(fd: i32, buf: *mut u8, count: usize) -> isize;
    /// https://sourceware.org/cgit/glibc/tree/posix/unistd.h
    pub fn write(fd: i32, buf: *const u8, count: usize) -> isize;
    /// https://sourceware.org/cgit/glibc/tree/sysdeps/unix/sysv/linux/sys/eventfd.h
    pub fn eventfd(initval: u32, flags: i32) -> i32;
    /// https://sourceware.org/cgit/glibc/tree/io/fcntl.h
    pub fn fcntl(fd: i32, op: i32, ...) -> i32;
    /// https://sourceware.org/cgit/glibc/tree/signal/signal.h
    pub fn signal(signum: i32, handler: Sighandler) -> Sighandler;
}
