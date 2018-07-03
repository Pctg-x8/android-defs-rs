
pub enum ANativeWindow {}
impl ANativeWindow {
    pub fn width(&self) -> i32 {
        unsafe { ANativeWindow_getWidth(self as *const _ as _) }
    }
    pub fn height(&self) -> i32 {
        unsafe { ANativeWindow_getHeight(self as *const _ as _) }
    }
}

#[allow(non_snake_case)]
extern "C" {
    pub fn ANativeWindow_getWidth(window: *mut ANativeWindow) -> i32;
    pub fn ANativeWindow_getHeight(window: *mut ANativeWindow) -> i32;
    pub fn ANativeWindow_getFormat(window: *mut ANativeWindow) -> i32;
}
