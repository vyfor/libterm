use crate::waker::Waker;
use std::io;

#[cfg(unix)]
use crate::platform::unix as sys_unix;

#[cfg(windows)]
use crate::platform::windows as sys_win;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PollResult {
    Input,
    Waker,
    Timeout,
}

pub struct Tty {
    #[cfg(unix)]
    pub(crate) read_fd: i32,
    #[cfg(unix)]
    pub(crate) write_fd: i32,
    #[cfg(unix)]
    pub(crate) owned: bool,
    #[cfg(unix)]
    pub(crate) flags: Option<i32>,

    #[cfg(windows)]
    pub(crate) in_handle: *mut core::ffi::c_void,
    #[cfg(windows)]
    pub(crate) out_handle: *mut core::ffi::c_void,
    #[cfg(windows)]
    pub(crate) in_owned: bool,
    #[cfg(windows)]
    pub(crate) out_owned: bool,
}

unsafe impl Send for Tty {}
unsafe impl Sync for Tty {}

// https://man7.org/linux/man-pages/man4/tty.4.html
// https://learn.microsoft.com/en-us/windows/console/console-handles
impl Tty {
    #[cfg(unix)]
    pub fn open() -> io::Result<Self> {
        let path = b"/dev/tty\0";
        let fd = unsafe {
            sys_unix::open(
                path.as_ptr() as *const core::ffi::c_char,
                sys_unix::O_RDWR | sys_unix::O_NONBLOCK | sys_unix::O_CLOEXEC,
            )
        };

        if fd >= 0 {
            Ok(Self {
                read_fd: fd,
                write_fd: fd,
                owned: true,
                flags: None,
            })
        } else {
            let stdin_fd = 0;
            let stdout_fd = 1;

            let flags = unsafe { sys_unix::fcntl(stdin_fd, sys_unix::F_GETFL) };
            let mut orig = None;
            if flags >= 0 {
                let res = unsafe {
                    sys_unix::fcntl(
                        stdin_fd,
                        sys_unix::F_SETFL,
                        flags | sys_unix::O_NONBLOCK,
                    )
                };
                if res >= 0 {
                    orig = Some(flags);
                }
            }

            Ok(Self {
                read_fd: stdin_fd,
                write_fd: stdout_fd,
                owned: false,
                flags: orig,
            })
        }
    }

    #[cfg(windows)]
    pub fn open() -> io::Result<Self> {
        let conin_name: Vec<u16> = "CONIN$\0".encode_utf16().collect();
        let conout_name: Vec<u16> = "CONOUT$\0".encode_utf16().collect();

        let in_handle = unsafe {
            sys_win::CreateFileW(
                conin_name.as_ptr(),
                sys_win::GENERIC_READ | sys_win::GENERIC_WRITE,
                sys_win::FILE_SHARE_READ | sys_win::FILE_SHARE_WRITE,
                core::ptr::null_mut(),
                sys_win::OPEN_EXISTING,
                0,
                core::ptr::null_mut(),
            )
        };

        let out_handle = unsafe {
            sys_win::CreateFileW(
                conout_name.as_ptr(),
                sys_win::GENERIC_READ | sys_win::GENERIC_WRITE,
                sys_win::FILE_SHARE_READ | sys_win::FILE_SHARE_WRITE,
                core::ptr::null_mut(),
                sys_win::OPEN_EXISTING,
                0,
                core::ptr::null_mut(),
            )
        };

        let in_owned = !(in_handle as isize == -1 || in_handle.is_null());
        let in_handle = if in_owned {
            in_handle
        } else {
            unsafe { sys_win::GetStdHandle(sys_win::STD_INPUT_HANDLE) }
        };

        let out_owned = !(out_handle as isize == -1 || out_handle.is_null());
        let out_handle = if out_owned {
            out_handle
        } else {
            unsafe { sys_win::GetStdHandle(sys_win::STD_OUTPUT_HANDLE) }
        };

        Ok(Self {
            in_handle,
            out_handle,
            in_owned,
            out_owned,
        })
    }

    #[cfg(unix)]
    pub fn read(&self, buf: &mut [u8]) -> io::Result<usize> {
        let res = unsafe {
            sys_unix::read(self.read_fd, buf.as_mut_ptr(), buf.len())
        };

        if res < 0 {
            Err(io::Error::last_os_error())
        } else {
            Ok(res as usize)
        }
    }

    #[cfg(windows)]
    pub fn read(&self, buf: &mut [u8]) -> io::Result<usize> {
        let mut bytes_read = 0u32;
        let res = unsafe {
            sys_win::ReadFile(
                self.in_handle,
                buf.as_mut_ptr(),
                buf.len() as u32,
                &mut bytes_read,
                core::ptr::null_mut(),
            )
        };

        if res == 0 {
            Err(io::Error::last_os_error())
        } else {
            Ok(bytes_read as usize)
        }
    }

    #[cfg(unix)]
    pub fn write(&self, buf: &[u8]) -> io::Result<usize> {
        let res =
            unsafe { sys_unix::write(self.write_fd, buf.as_ptr(), buf.len()) };

        if res < 0 {
            Err(io::Error::last_os_error())
        } else {
            Ok(res as usize)
        }
    }

    #[cfg(windows)]
    pub fn write(&self, buf: &[u8]) -> io::Result<usize> {
        let mut bytes_written = 0u32;
        let res = unsafe {
            sys_win::WriteFile(
                self.out_handle,
                buf.as_ptr(),
                buf.len() as u32,
                &mut bytes_written,
                core::ptr::null_mut(),
            )
        };

        if res == 0 {
            Err(io::Error::last_os_error())
        } else {
            Ok(bytes_written as usize)
        }
    }

    #[cfg(unix)]
    pub fn write_all(&self, mut buf: &[u8]) -> io::Result<()> {
        while !buf.is_empty() {
            match self.write(buf) {
                Ok(0) => {
                    return Err(io::Error::from(io::ErrorKind::WriteZero));
                }
                Ok(n) => buf = &buf[n..],
                Err(e) if e.kind() == io::ErrorKind::Interrupted => continue,
                Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                    let mut pfd = sys_unix::Pollfd {
                        fd: self.write_fd,
                        events: sys_unix::POLLOUT,
                        revents: 0,
                    };

                    let res = unsafe { sys_unix::poll(&mut pfd, 1, -1) };

                    if res < 0 {
                        let err = io::Error::last_os_error();
                        if err.kind() == io::ErrorKind::Interrupted {
                            continue;
                        }
                        return Err(err);
                    }

                    if (pfd.revents
                        & (sys_unix::POLLERR
                            | sys_unix::POLLHUP
                            | sys_unix::POLLNVAL))
                        != 0
                    {
                        return Err(io::Error::from(io::ErrorKind::BrokenPipe));
                    }
                }
                Err(e) => return Err(e),
            }
        }
        Ok(())
    }

    #[cfg(windows)]
    pub fn write_all(&self, mut buf: &[u8]) -> io::Result<()> {
        while !buf.is_empty() {
            match self.write(buf) {
                Ok(0) => {
                    return Err(io::Error::from(io::ErrorKind::WriteZero));
                }
                Ok(n) => buf = &buf[n..],
                Err(e) if e.kind() == io::ErrorKind::Interrupted => continue,
                Err(e) => return Err(e),
            }
        }
        Ok(())
    }

    pub fn size(&self) -> io::Result<(u16, u16)> {
        #[cfg(unix)]
        {
            let mut ws = sys_unix::Winsize::default();
            let res = unsafe {
                sys_unix::ioctl(self.read_fd, sys_unix::TIOCGWINSZ, &mut ws)
            };
            if res == 0 && ws.ws_col > 0 && ws.ws_row > 0 {
                return Ok((ws.ws_col, ws.ws_row));
            }

            let res = unsafe {
                sys_unix::ioctl(self.write_fd, sys_unix::TIOCGWINSZ, &mut ws)
            };
            if res == 0 && ws.ws_col > 0 && ws.ws_row > 0 {
                return Ok((ws.ws_col, ws.ws_row));
            }

            Err(io::Error::last_os_error())
        }

        #[cfg(windows)]
        {
            let mut csbi = sys_win::ConsoleScreenBufferInfo::default();
            let res = unsafe {
                sys_win::GetConsoleScreenBufferInfo(self.out_handle, &mut csbi)
            };
            if res == 0 {
                return Err(io::Error::last_os_error());
            }

            let cols =
                (csbi.srWindow.Right - csbi.srWindow.Left + 1).max(0) as u16;
            let rows =
                (csbi.srWindow.Bottom - csbi.srWindow.Top + 1).max(0) as u16;

            Ok((cols, rows))
        }
    }

    pub fn poll(
        &self,
        waker: Option<&Waker>,
        timeout_ms: i32,
    ) -> io::Result<PollResult> {
        #[cfg(unix)]
        {
            let mut fds = [
                sys_unix::Pollfd {
                    fd: self.read_fd,
                    events: sys_unix::POLLIN,
                    revents: 0,
                },
                sys_unix::Pollfd {
                    fd: -1,
                    events: 0,
                    revents: 0,
                },
            ];

            let count = if let Some(w) = waker {
                fds[1] = sys_unix::Pollfd {
                    fd: w.fd(),
                    events: sys_unix::POLLIN,
                    revents: 0,
                };
                2
            } else {
                1
            };

            let res =
                unsafe { sys_unix::poll(fds.as_mut_ptr(), count, timeout_ms) };
            if res < 0 {
                let err = io::Error::last_os_error();
                if err.kind() == io::ErrorKind::Interrupted {
                    return Ok(PollResult::Timeout);
                }
                return Err(err);
            }

            if res == 0 {
                return Ok(PollResult::Timeout);
            }

            if (fds[0].revents & sys_unix::POLLIN) != 0 {
                return Ok(PollResult::Input);
            }

            if count > 1 && (fds[1].revents & sys_unix::POLLIN) != 0 {
                if let Some(w) = waker {
                    let _ = w.drain();
                }
                return Ok(PollResult::Waker);
            }

            Ok(PollResult::Timeout)
        }

        #[cfg(windows)]
        {
            let mut handles = [core::ptr::null_mut(); 2];
            handles[0] = self.in_handle;
            let count = if let Some(w) = waker {
                handles[1] = w.handle();
                2
            } else {
                1
            };

            let timeout = if timeout_ms < 0 {
                sys_win::INFINITE
            } else {
                timeout_ms as u32
            };

            let res = unsafe {
                sys_win::WaitForMultipleObjects(
                    count as u32,
                    handles.as_ptr(),
                    0,
                    timeout,
                )
            };

            if res == sys_win::WAIT_OBJECT_0 {
                Ok(PollResult::Input)
            } else if count > 1 && res == sys_win::WAIT_OBJECT_0 + 1 {
                if let Some(w) = waker {
                    let _ = w.drain();
                }
                Ok(PollResult::Waker)
            } else if res == sys_win::WAIT_TIMEOUT {
                Ok(PollResult::Timeout)
            } else {
                Err(io::Error::last_os_error())
            }
        }
    }
}

impl Drop for Tty {
    fn drop(&mut self) {
        #[cfg(unix)]
        if let Some(flags) = self.flags {
            unsafe {
                sys_unix::fcntl(self.read_fd, sys_unix::F_SETFL, flags);
            }
        }

        #[cfg(unix)]
        if self.owned && self.read_fd >= 0 {
            unsafe {
                sys_unix::close(self.read_fd);
            }
        }

        #[cfg(windows)]
        {
            if self.in_owned
                && !self.in_handle.is_null()
                && self.in_handle as isize != -1
            {
                unsafe {
                    sys_win::CloseHandle(self.in_handle);
                }
            }
            if self.out_owned
                && !self.out_handle.is_null()
                && self.out_handle as isize != -1
            {
                unsafe {
                    sys_win::CloseHandle(self.out_handle);
                }
            }
        }
    }
}
