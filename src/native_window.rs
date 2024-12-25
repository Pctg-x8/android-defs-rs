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
    pub unsafe fn ANativeWindow_fromSurface(
        env: jni::JNIEnv,
        surface: jni::objects::JObject,
    ) -> *mut ANativeWindow;
}
