pub const ANDROID_LOG_UNKOWN: core::ffi::c_int = 0;
pub const ANDROID_LOG_DEFAULT: core::ffi::c_int = 1;
pub const ANDROID_LOG_VERBOSE: core::ffi::c_int = 2;
pub const ANDROID_LOG_DEBUG: core::ffi::c_int = 3;
pub const ANDROID_LOG_INFO: core::ffi::c_int = 4;
pub const ANDROID_LOG_WARN: core::ffi::c_int = 5;
pub const ANDROID_LOG_ERROR: core::ffi::c_int = 6;
pub const ANDROID_LOG_FATAL: core::ffi::c_int = 7;
pub const ANDROID_LOG_SILENT: core::ffi::c_int = 8;

pub const LOG_ID_MAIN: core::ffi::c_int = 0;
pub const LOG_ID_RADIO: core::ffi::c_int = 1;
pub const LOG_ID_EVENTS: core::ffi::c_int = 2;
pub const LOG_ID_SYSTEM: core::ffi::c_int = 3;
pub const LOG_ID_CRASH: core::ffi::c_int = 4;
pub const LOG_ID_STATS: core::ffi::c_int = 5;
pub const LOG_ID_SECURITY: core::ffi::c_int = 6;
pub const LOG_ID_KERNEL: core::ffi::c_int = 7;

#[allow(non_camel_case_types)]
pub type __android_aborter_function = Option<extern "C" fn(abort_message: *const core::ffi::c_char)>;

#[allow(non_camel_case_types)]
pub type __android_logger_function = Option<extern "C" fn(log_message: *const __android_log_message)>;

#[repr(C)]
#[allow(non_camel_case_types)]
pub struct __android_log_message {
    pub struct_size: usize,
    pub buffer_id: i32,
    pub priority: i32,
    pub tag: *const core::ffi::c_char,
    pub file: *const core::ffi::c_char,
    pub line: u32,
    pub message: *const core::ffi::c_char,
}

unsafe extern "C" {
    pub unsafe fn __android_log_assert(cond: *const core::ffi::c_char, tag: *const core::ffi::c_char, fmt: *const core::ffi::c_char, ...);
    pub unsafe fn __android_log_buf_print(bufId: core::ffi::c_int, prio: core::ffi::c_int, tag: *const core::ffi::c_char, fmt: *const core::ffi::c_char, ...) -> core::ffi::c_int;
    pub unsafe fn __android_log_buf_write(bufId: core::ffi::c_int, prio: core::ffi::c_int, tag: *const core::ffi::c_char, text: *const core::ffi::c_char) -> core::ffi::c_int;
    pub unsafe fn __androidlog_call_aborter(abort_message: *const core::ffi::c_char);
    pub unsafe fn __android_log_default_aborter(abort_message: *const core::ffi::c_char);
    pub unsafe fn __android_log_get_minimum_priority() -> i32;
    pub unsafe fn __android_log_is_loggable(prio: core::ffi::c_int, tag: *const core::ffi::c_char, default_prio: core::ffi::c_int) -> core::ffi::c_int;
    pub unsafe fn __android_log_is_loggable_len(prio: core::ffi::c_int, tag: *const core::ffi::c_char, len: usize, default_prio: core::ffi::c_int) -> core::ffi::c_int;
    pub unsafe fn __android_log_logd_logger(log_message: *const __android_log_message);
    pub unsafe fn __android_log_print(prio: core::ffi::c_int, tag: *const core::ffi::c_char, fmt: *const core::ffi::c_char, ...) -> core::ffi::c_int;
    pub unsafe fn __android_log_set_aborter(aborter: __android_aborter_function);
    pub unsafe fn __android_log_set_default_tag(tag: *const core::ffi::c_char);
    pub unsafe fn __android_log_set_logger(logger: __android_logger_function);
    pub unsafe fn __android_log_set_minimum_priority(priority: i32) -> i32;
    pub unsafe fn __android_log_stderr_logger(log_message: *const __android_log_message);
    // requires nightly api
    // pub unsafe fn __android_log_vprint(prio: core::ffi::c_int, tag: *const core::ffi::c_char, fmt: *const core::ffi::c_char, ap: core::ffi::VaList) -> core::ffi::c_int;
    pub unsafe fn __android_log_write(prio: core::ffi::c_int, tag: *const core::ffi::c_char, text: *const core::ffi::c_char) -> core::ffi::c_int;
    pub unsafe fn __android_log_write_log_message(log_message: *mut __android_log_message);
}

#[macro_export]
macro_rules! android_log_print {
    (tag: $tag: expr, $($arg: tt)*) => {
        let str = core::ffi::CString::new(&format!($($arg)*)).expect("invalid sequence");
        unsafe { __android_log_print($crate::ANDROID_LOG_DEFAULT, tag.as_ptr(), str.as_ptr()); }
    };
    ($($arg: tt)*) => {
        let str = core::ffi::CString::new(&format!($($arg)*)).expect("invalid sequence");
        unsafe { __android_log_print($crate::ANDROID_LOG_DEFAULT, core::ffi::null(), str.as_ptr()); }
    }
}
