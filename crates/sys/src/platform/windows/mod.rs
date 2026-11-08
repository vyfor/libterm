//! https://learn.microsoft.com/en-us/windows/console/consoles

/// https://github.com/mingw-w64/mingw-w64/blob/master/mingw-w64-headers/include/wincontypes.h
#[allow(non_snake_case)]
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Coord {
    pub X: i16,
    pub Y: i16,
}

/// https://github.com/mingw-w64/mingw-w64/blob/master/mingw-w64-headers/include/wincontypes.h
#[allow(non_snake_case)]
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct SmallRect {
    pub Left: i16,
    pub Top: i16,
    pub Right: i16,
    pub Bottom: i16,
}

/// https://github.com/mingw-w64/mingw-w64/blob/master/mingw-w64-headers/include/consoleapi2.h
#[allow(non_snake_case)]
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct ConsoleScreenBufferInfo {
    pub dwSize: Coord,
    pub dwCursorPosition: Coord,
    pub wAttributes: u16,
    pub srWindow: SmallRect,
    pub dwMaximumWindowSize: Coord,
}

/// https://github.com/mingw-w64/mingw-w64/blob/master/mingw-w64-headers/include/consoleapi.h
pub const ENABLE_VIRTUAL_TERMINAL_INPUT: u32 = 0x0200;

/// https://github.com/mingw-w64/mingw-w64/blob/master/mingw-w64-headers/include/consoleapi.h
pub const ENABLE_VIRTUAL_TERMINAL_PROCESSING: u32 = 0x0004;

/// https://github.com/mingw-w64/mingw-w64/blob/master/mingw-w64-headers/include/consoleapi.h
pub const ENABLE_PROCESSED_INPUT: u32 = 0x0001;

/// https://github.com/mingw-w64/mingw-w64/blob/master/mingw-w64-headers/include/consoleapi.h
pub const ENABLE_PROCESSED_OUTPUT: u32 = 0x0001;

/// https://github.com/mingw-w64/mingw-w64/blob/master/mingw-w64-headers/include/consoleapi.h
pub const ENABLE_LINE_INPUT: u32 = 0x0002;

/// https://github.com/mingw-w64/mingw-w64/blob/master/mingw-w64-headers/include/consoleapi.h
pub const ENABLE_ECHO_INPUT: u32 = 0x0004;

/// https://github.com/mingw-w64/mingw-w64/blob/master/mingw-w64-headers/include/consoleapi.h
pub const ENABLE_WINDOW_INPUT: u32 = 0x0008;

/// https://github.com/mingw-w64/mingw-w64/blob/master/mingw-w64-headers/include/consoleapi.h
pub const ENABLE_QUICK_EDIT_MODE: u32 = 0x0040;

/// https://github.com/mingw-w64/mingw-w64/blob/master/mingw-w64-headers/include/consoleapi.h
pub const ENABLE_EXTENDED_FLAGS: u32 = 0x0080;

/// https://github.com/mingw-w64/mingw-w64/blob/master/mingw-w64-headers/include/winbase.h
pub const STD_INPUT_HANDLE: u32 = 0xFFFFFFF6;

/// https://github.com/mingw-w64/mingw-w64/blob/master/mingw-w64-headers/include/winbase.h
pub const STD_OUTPUT_HANDLE: u32 = 0xFFFFFFF5;

/// https://github.com/mingw-w64/mingw-w64/blob/master/mingw-w64-headers/include/winnt.h
pub const GENERIC_READ: u32 = 0x80000000;

/// https://github.com/mingw-w64/mingw-w64/blob/master/mingw-w64-headers/include/winnt.h
pub const GENERIC_WRITE: u32 = 0x40000000;

/// https://github.com/mingw-w64/mingw-w64/blob/master/mingw-w64-headers/include/winnt.h
pub const FILE_SHARE_READ: u32 = 1;

/// https://github.com/mingw-w64/mingw-w64/blob/master/mingw-w64-headers/include/winnt.h
pub const FILE_SHARE_WRITE: u32 = 2;

/// https://github.com/mingw-w64/mingw-w64/blob/master/mingw-w64-headers/include/fileapi.h
pub const OPEN_EXISTING: u32 = 3;

/// https://github.com/mingw-w64/mingw-w64/blob/master/mingw-w64-headers/include/winbase.h
pub const WAIT_OBJECT_0: u32 = 0;

/// https://github.com/mingw-w64/mingw-w64/blob/master/mingw-w64-headers/include/winerror.h
pub const WAIT_TIMEOUT: u32 = 258;

/// https://github.com/mingw-w64/mingw-w64/blob/master/mingw-w64-headers/include/winbase.h
pub const INFINITE: u32 = 0xFFFFFFFF;

#[link(name = "kernel32")]
unsafe extern "system" {
    /// https://github.com/mingw-w64/mingw-w64/blob/master/mingw-w64-headers/include/processenv.h
    pub fn GetStdHandle(n_std_handle: u32) -> *mut core::ffi::c_void;
    /// https://github.com/mingw-w64/mingw-w64/blob/master/mingw-w64-headers/include/consoleapi.h
    pub fn GetConsoleMode(
        h_console_handle: *mut core::ffi::c_void,
        lp_mode: *mut u32,
    ) -> i32;
    /// https://github.com/mingw-w64/mingw-w64/blob/master/mingw-w64-headers/include/consoleapi.h
    pub fn SetConsoleMode(
        h_console_handle: *mut core::ffi::c_void,
        dw_mode: u32,
    ) -> i32;
    /// https://github.com/mingw-w64/mingw-w64/blob/master/mingw-w64-headers/include/consoleapi2.h
    pub fn GetConsoleScreenBufferInfo(
        h_console_handle: *mut core::ffi::c_void,
        lp_console_screen_buffer_info: *mut ConsoleScreenBufferInfo,
    ) -> i32;
    /// https://github.com/mingw-w64/mingw-w64/blob/master/mingw-w64-headers/include/fileapi.h
    pub fn CreateFileW(
        lp_file_name: *const u16,
        dw_desired_access: u32,
        dw_share_mode: u32,
        lp_security_attributes: *mut core::ffi::c_void,
        dw_creation_disposition: u32,
        dw_flags_and_attributes: u32,
        h_template_file: *mut core::ffi::c_void,
    ) -> *mut core::ffi::c_void;
    /// https://github.com/mingw-w64/mingw-w64/blob/master/mingw-w64-headers/include/fileapi.h
    pub fn ReadFile(
        h_file: *mut core::ffi::c_void,
        lp_buffer: *mut u8,
        n_number_of_bytes_to_read: u32,
        lp_number_of_bytes_read: *mut u32,
        lp_overlapped: *mut core::ffi::c_void,
    ) -> i32;
    /// https://github.com/mingw-w64/mingw-w64/blob/master/mingw-w64-headers/include/fileapi.h
    pub fn WriteFile(
        h_file: *mut core::ffi::c_void,
        lp_buffer: *const u8,
        n_number_of_bytes_to_write: u32,
        lp_number_of_bytes_written: *mut u32,
        lp_overlapped: *mut core::ffi::c_void,
    ) -> i32;
    /// https://github.com/mingw-w64/mingw-w64/blob/master/mingw-w64-headers/include/handleapi.h
    pub fn CloseHandle(h_object: *mut core::ffi::c_void) -> i32;
    /// https://github.com/mingw-w64/mingw-w64/blob/master/mingw-w64-headers/include/synchapi.h
    pub fn CreateEventW(
        lp_event_attributes: *mut core::ffi::c_void,
        b_manual_reset: i32,
        b_initial_state: i32,
        lp_name: *const u16,
    ) -> *mut core::ffi::c_void;
    /// https://github.com/mingw-w64/mingw-w64/blob/master/mingw-w64-headers/include/synchapi.h
    pub fn SetEvent(h_event: *mut core::ffi::c_void) -> i32;
    /// https://github.com/mingw-w64/mingw-w64/blob/master/mingw-w64-headers/include/synchapi.h
    pub fn ResetEvent(h_event: *mut core::ffi::c_void) -> i32;
    /// https://github.com/mingw-w64/mingw-w64/blob/master/mingw-w64-headers/include/winbase.h
    pub fn WaitForMultipleObjects(
        n_count: u32,
        lp_handles: *const *mut core::ffi::c_void,
        b_wait_all: i32,
        dw_milliseconds: u32,
    ) -> u32;
}
