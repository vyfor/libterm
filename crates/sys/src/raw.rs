use crate::tty::Tty;
use std::io;

#[cfg(unix)]
use crate::platform::unix as sys_unix;

#[cfg(windows)]
use crate::platform::windows as sys_win;

/// https://man7.org/linux/man-pages/man3/cfmakeraw.3.html
/// https://learn.microsoft.com/en-us/windows/console/high-level-console-modes
pub struct RawGuard {
    #[cfg(unix)]
    fd: i32,
    #[cfg(unix)]
    orig: sys_unix::Termios,

    #[cfg(windows)]
    in_handle: *mut core::ffi::c_void,
    #[cfg(windows)]
    out_handle: *mut core::ffi::c_void,
    #[cfg(windows)]
    orig_in_mode: u32,
    #[cfg(windows)]
    orig_out_mode: u32,

    active: bool,
}

impl RawGuard {
    #[cfg(unix)]
    pub fn enter(tty: &Tty) -> io::Result<Self> {
        let fd = tty.read_fd;
        let mut orig = sys_unix::Termios::default();
        let res = unsafe { sys_unix::tcgetattr(fd, &mut orig) };
        if res < 0 {
            return Err(io::Error::last_os_error());
        }

        let mut raw = orig;
        sys_unix::raw_mode(&mut raw);

        let res = unsafe { sys_unix::tcsetattr(fd, sys_unix::TCSANOW, &raw) };
        if res < 0 {
            return Err(io::Error::last_os_error());
        }

        Ok(Self {
            fd,
            orig,
            active: true,
        })
    }

    #[cfg(windows)]
    pub fn enter(tty: &Tty) -> io::Result<Self> {
        let mut orig_in_mode = 0u32;
        let mut orig_out_mode = 0u32;

        let res = unsafe {
            sys_win::GetConsoleMode(tty.in_handle, &mut orig_in_mode)
        };
        if res == 0 {
            return Err(io::Error::last_os_error());
        }

        let res = unsafe {
            sys_win::GetConsoleMode(tty.out_handle, &mut orig_out_mode)
        };
        if res == 0 {
            return Err(io::Error::last_os_error());
        }

        let raw_in = (orig_in_mode
            | sys_win::ENABLE_EXTENDED_FLAGS
            | sys_win::ENABLE_VIRTUAL_TERMINAL_INPUT)
            & !(sys_win::ENABLE_LINE_INPUT
                | sys_win::ENABLE_ECHO_INPUT
                | sys_win::ENABLE_PROCESSED_INPUT
                | sys_win::ENABLE_WINDOW_INPUT
                | sys_win::ENABLE_QUICK_EDIT_MODE);

        let raw_out = orig_out_mode
            | sys_win::ENABLE_PROCESSED_OUTPUT
            | sys_win::ENABLE_VIRTUAL_TERMINAL_PROCESSING;

        let res = unsafe { sys_win::SetConsoleMode(tty.in_handle, raw_in) };
        if res == 0 {
            return Err(io::Error::last_os_error());
        }

        let res = unsafe { sys_win::SetConsoleMode(tty.out_handle, raw_out) };
        if res == 0 {
            unsafe {
                sys_win::SetConsoleMode(tty.in_handle, orig_in_mode);
            }
            return Err(io::Error::last_os_error());
        }

        Ok(Self {
            in_handle: tty.in_handle,
            out_handle: tty.out_handle,
            orig_in_mode,
            orig_out_mode,
            active: true,
        })
    }

    #[cfg(unix)]
    pub fn restore(&mut self) -> io::Result<()> {
        if self.active {
            let res = unsafe {
                sys_unix::tcsetattr(self.fd, sys_unix::TCSANOW, &self.orig)
            };
            if res < 0 {
                return Err(io::Error::last_os_error());
            }
            self.active = false;
        }
        Ok(())
    }

    #[cfg(windows)]
    pub fn restore(&mut self) -> io::Result<()> {
        if self.active {
            let in_res = unsafe {
                sys_win::SetConsoleMode(self.in_handle, self.orig_in_mode)
            };
            let out_res = unsafe {
                sys_win::SetConsoleMode(self.out_handle, self.orig_out_mode)
            };
            self.active = false;

            if in_res == 0 {
                return Err(io::Error::last_os_error());
            }
            if out_res == 0 {
                return Err(io::Error::last_os_error());
            }
        }
        Ok(())
    }
}

impl Drop for RawGuard {
    fn drop(&mut self) {
        let _ = self.restore();
    }
}
