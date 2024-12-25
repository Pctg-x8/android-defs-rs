use jni::{objects::JObject, JNIEnv};

#[repr(C)]
pub struct ANativeWindow(
    [u8; 0],
    core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
);

#[allow(non_snake_case)]
unsafe extern "C" {
    pub unsafe fn ANativeWindow_getWidth(window: *mut ANativeWindow) -> i32;
    pub unsafe fn ANativeWindow_getHeight(window: *mut ANativeWindow) -> i32;
    pub unsafe fn ANativeWindow_getFormat(window: *mut ANativeWindow) -> i32;
    pub unsafe fn ANativeWindow_fromSurface(env: JNIEnv, surface: JObject) -> *mut ANativeWindow;
    pub unsafe fn ANativeWindow_acquire(window: *mut ANativeWindow);
    pub unsafe fn ANativeWindow_release(window: *mut ANativeWindow);
}
