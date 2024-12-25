//! ALooper

#[repr(C)]
pub struct ALooper(
    [u8; 0],
    core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
);

pub const ALOOPER_PREPARE_ALLOW_NON_CALLBACKS: core::ffi::c_int = 1 << 0;

pub const ALOOPER_EVENT_INPUT: core::ffi::c_int = 1 << 0;
pub const ALOOPER_EVENT_OUTPUT: core::ffi::c_int = 1 << 1;
pub const ALOOPER_EVENT_ERROR: core::ffi::c_int = 1 << 2;
pub const ALOOPER_EVENT_HANGUP: core::ffi::c_int = 1 << 3;
pub const ALOOPER_EVENT_INVALID: core::ffi::c_int = 1 << 4;

#[allow(non_camel_case_types)]
pub type ALooper_callbackFunc = Option<
    extern "C" fn(core::ffi::c_int, core::ffi::c_int, *mut core::ffi::c_void) -> core::ffi::c_int,
>;

#[allow(non_snake_case)]
unsafe extern "C" {
    pub unsafe fn ALooper_forThread() -> *mut ALooper;
    pub unsafe fn ALooper_prepare(opts: core::ffi::c_int) -> *mut ALooper;
    pub unsafe fn ALooper_acquire(looper: *mut ALooper);
    pub unsafe fn ALooper_release(looper: *mut ALooper);
    pub unsafe fn ALooper_pollOnce(
        timeout_millis: core::ffi::c_int,
        outfd: *mut core::ffi::c_int,
        out_events: *mut core::ffi::c_int,
        out_data: *mut *mut core::ffi::c_void,
    ) -> core::ffi::c_int;
    pub unsafe fn ALooper_pollAll(
        timeout_millis: core::ffi::c_int,
        outfd: *mut core::ffi::c_int,
        outevents: *mut core::ffi::c_int,
        outdata: *mut *mut core::ffi::c_void,
    ) -> core::ffi::c_int;
    pub unsafe fn ALooper_addFd(
        looper: *mut ALooper,
        fd: core::ffi::c_int,
        ident: core::ffi::c_int,
        events: core::ffi::c_int,
        callback: ALooper_callbackFunc,
        data: *mut core::ffi::c_void,
    ) -> core::ffi::c_int;
    pub unsafe fn ALooper_removeFd(looper: *mut ALooper, fd: core::ffi::c_int) -> core::ffi::c_int;
    pub unsafe fn ALooper_wake(looper: *mut ALooper);
}
