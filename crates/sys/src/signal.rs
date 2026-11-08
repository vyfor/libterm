#[cfg(unix)]
use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};

#[cfg(unix)]
use crate::platform::unix as sys_unix;

#[cfg(unix)]
static RESIZED: AtomicBool = AtomicBool::new(false);
#[cfg(unix)]
static WAKER_FD: AtomicI32 = AtomicI32::new(-1);

#[cfg(unix)]
extern "C" fn handle_sigwinch(_sig: i32) {
    RESIZED.store(true, Ordering::Relaxed);
    let fd = WAKER_FD.load(Ordering::Relaxed);
    if fd >= 0 {
        let val: u64 = 1;
        unsafe {
            sys_unix::write(
                fd,
                &val as *const u64 as *const u8,
                core::mem::size_of::<u64>(),
            );
        }
    }
}

#[cfg(unix)]
pub fn register_sigwinch(waker_fd: i32) {
    WAKER_FD.store(waker_fd, Ordering::Relaxed);
    unsafe {
        sys_unix::signal(sys_unix::SIGWINCH, Some(handle_sigwinch));
    }
}

#[cfg(unix)]
pub fn unregister_sigwinch() {
    WAKER_FD.store(-1, Ordering::Relaxed);
    unsafe {
        sys_unix::signal(sys_unix::SIGWINCH, sys_unix::SIG_DFL);
    }
}

pub fn resized() -> bool {
    #[cfg(unix)]
    {
        RESIZED.swap(false, Ordering::Relaxed)
    }
    #[cfg(not(unix))]
    {
        false
    }
}
