use std::io;
use std::sync::Arc;

#[cfg(unix)]
use crate::platform::unix as sys_unix;

#[cfg(windows)]
use crate::platform::windows as sys_win;

struct WakerInner {
    #[cfg(unix)]
    fd: i32,

    #[cfg(windows)]
    handle: *mut core::ffi::c_void,
}

unsafe impl Send for WakerInner {}
unsafe impl Sync for WakerInner {}

/// https://man7.org/linux/man-pages/man2/eventfd.2.html
/// https://learn.microsoft.com/en-us/windows/win32/api/synchapi/nf-synchapi-createeventw
#[derive(Clone)]
pub struct Waker {
    inner: Arc<WakerInner>,
}

impl Waker {
    #[cfg(unix)]
    pub fn new() -> io::Result<Self> {
        let fd = unsafe {
            sys_unix::eventfd(0, sys_unix::EFD_CLOEXEC | sys_unix::EFD_NONBLOCK)
        };

        if fd < 0 {
            Err(io::Error::last_os_error())
        } else {
            Ok(Self {
                inner: Arc::new(WakerInner { fd }),
            })
        }
    }

    #[cfg(windows)]
    pub fn new() -> io::Result<Self> {
        let handle = unsafe {
            sys_win::CreateEventW(
                core::ptr::null_mut(),
                0,
                0,
                core::ptr::null(),
            )
        };

        if handle.is_null() {
            Err(io::Error::last_os_error())
        } else {
            Ok(Self {
                inner: Arc::new(WakerInner { handle }),
            })
        }
    }

    #[cfg(unix)]
    pub fn wake(&self) -> io::Result<()> {
        let val: u64 = 1;
        let res = unsafe {
            sys_unix::write(
                self.inner.fd,
                &val as *const u64 as *const u8,
                core::mem::size_of::<u64>(),
            )
        };

        if res < 0 {
            let err = io::Error::last_os_error();
            if err.kind() == io::ErrorKind::WouldBlock {
                Ok(())
            } else {
                Err(err)
            }
        } else {
            Ok(())
        }
    }

    #[cfg(windows)]
    pub fn wake(&self) -> io::Result<()> {
        let res = unsafe { sys_win::SetEvent(self.inner.handle) };

        if res == 0 {
            Err(io::Error::last_os_error())
        } else {
            Ok(())
        }
    }

    #[cfg(unix)]
    pub fn drain(&self) -> io::Result<()> {
        let mut val = 0u64;
        let res = unsafe {
            sys_unix::read(
                self.inner.fd,
                &mut val as *mut u64 as *mut u8,
                core::mem::size_of::<u64>(),
            )
        };

        if res < 0 {
            let err = io::Error::last_os_error();
            if err.kind() == io::ErrorKind::WouldBlock {
                Ok(())
            } else {
                Err(err)
            }
        } else {
            Ok(())
        }
    }

    #[cfg(windows)]
    pub fn drain(&self) -> io::Result<()> {
        let res = unsafe { sys_win::ResetEvent(self.inner.handle) };

        if res == 0 {
            Err(io::Error::last_os_error())
        } else {
            Ok(())
        }
    }

    #[cfg(unix)]
    pub fn fd(&self) -> i32 {
        self.inner.fd
    }

    #[cfg(windows)]
    pub fn handle(&self) -> *mut core::ffi::c_void {
        self.inner.handle
    }
}

impl Drop for WakerInner {
    fn drop(&mut self) {
        #[cfg(unix)]
        unsafe {
            sys_unix::close(self.fd);
        }

        #[cfg(windows)]
        if !self.handle.is_null() && self.handle as isize != -1 {
            unsafe {
                sys_win::CloseHandle(self.handle);
            }
        }
    }
}
