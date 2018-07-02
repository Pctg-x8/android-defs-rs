
use libc::*;
use super::*;
use jni::sys::{JavaVM, JNIEnv, jobject};

#[repr(C)]
pub struct ANativeActivity {
    pub callbacks: *mut ANativeActivityCallbacks,
    pub vm: *mut JavaVM, pub env: *mut JNIEnv,
    pub class: jobject, pub internal_data_path: *const c_char,
    pub external_data_path: *const c_char, pub sdk_version: i32,
    pub instance: *mut c_void, pub asset_manager: *mut AAssetManager,
    pub obb_path: *const c_char
}
#[repr(C)]
pub struct ANativeActivityCallbacks {
    pub on_start: Option<extern "C" fn(*mut ANativeActivity)>,
    pub on_resume: Option<extern "C" fn(*mut ANativeActivity)>,
    pub on_save_instance_state: Option<extern "C" fn(*mut ANativeActivity, *mut size_t) -> *mut c_void>,
    pub on_pause: Option<extern "C" fn(*mut ANativeActivity)>,
    pub on_stop: Option<extern "C" fn(*mut ANativeActivity)>,
    pub on_destroy: Option<extern "C" fn(*mut ANativeActivity)>,
    pub on_window_focus_changed: Option<extern "C" fn(*mut ANativeActivity, c_int)>,
    pub on_native_window_created: Option<extern "C" fn(*mut ANativeActivity, *mut ANativeWindow)>,
    pub on_native_window_resized: Option<extern "C" fn(*mut ANativeActivity, *mut ANativeWindow)>,
    pub on_native_window_redraw_needed: Option<extern "C" fn(*mut ANativeActivity, *mut ANativeWindow)>,
    pub on_native_window_destroyed: Option<extern "C" fn(*mut ANativeActivity, *mut ANativeWindow)>,
    pub on_input_queue_created: Option<extern "C" fn(*mut ANativeActivity, *mut AInputQueue)>,
    pub on_input_queue_destroyed: Option<extern "C" fn(*mut ANativeActivity, *mut AInputQueue)>,
    pub on_content_rect_changed: Option<extern "C" fn(*mut ANativeActivity, *const ARect)>,
    pub on_configuration_changed: Option<extern "C" fn(*mut ANativeActivity)>,
    pub on_low_memory: Option<extern "C" fn(*mut ANativeActivity)>
}
