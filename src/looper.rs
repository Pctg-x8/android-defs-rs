//! ALooper

use libc::*;
use std::ptr::NonNull;
use std::mem::replace;

pub enum ALooper {}

pub const ALOOPER_PREPARE_ALLOW_NON_CALLBACKS: c_int = 1 << 0;

pub const ALOOPER_EVENT_INPUT: c_int = 1 << 0;
pub const ALOOPER_EVENT_OUTPUT: c_int = 1 << 1;
pub const ALOOPER_EVENT_ERROR: c_int = 1 << 2;
pub const ALOOPER_EVENT_HANGUP: c_int = 1 << 3;
pub const ALOOPER_EVENT_INVALID: c_int = 1 << 4;

pub type CallbackFunc = Option<fn(c_int, c_int, *mut c_void) -> c_int>;

#[allow(non_snake_case)]
extern "C" {
    pub fn ALooper_forThread() -> *mut ALooper;
    pub fn ALooper_prepare(opts: c_int) -> *mut ALooper;
    pub fn ALooper_acquire(looper: *mut ALooper);
    pub fn ALooper_release(looper: *mut ALooper);
    pub fn ALooper_pollOnce(timeout_millis: c_int, outfd: *mut c_int, out_events: *mut c_int, out_data: *mut *mut c_void) -> c_int;
    pub fn ALooper_pollAll(timeout_millis: c_int, outfd: *mut c_int, outevents: *mut c_int, outdata: *mut *mut c_void) -> c_int;
    pub fn ALooper_wake(looper: *mut ALooper);
}

pub struct Looper(NonNull<ALooper>);
impl Looper {
    pub fn prepare(opts: c_int) -> Option<Self> {
        NonNull::new(unsafe { ALooper_prepare(opts) }).map(Looper)
    }
    pub fn poll_all(timeout: c_int, outfd: &mut c_int, outevents: &mut c_int, outdata: *mut *mut c_void) -> c_int {
        unsafe { ALooper_pollAll(timeout, outfd, outevents, outdata) }
    }
}
impl Drop for Looper {
    fn drop(&mut self) {
        unsafe { ALooper_release(replace(&mut self.0, NonNull::dangling()).as_ptr()) }
    }
}
