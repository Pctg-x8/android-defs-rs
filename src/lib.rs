extern crate libc;
extern crate jni;

/// rect
#[repr(C)] pub struct ARect {
    pub left: i32, pub top: i32, pub right: i32, pub bottom: i32
}

mod asset_manager; pub use self::asset_manager::*;
mod native_window; pub use self::native_window::*;
mod input; pub use self::input::*;
mod configuration; pub use self::configuration::*;
mod native_activity; pub use self::native_activity::*;
pub mod looper; pub use self::looper::*;
use libc::*;

#[repr(C)]
pub struct PollSource {
    pub id: i32, pub app: *mut App, pub process_fn: Option<extern "C" fn(*mut App, *mut PollSource)>
}
impl PollSource {
    pub fn process(&mut self, app: &mut App) {
        if let Some(p) = self.process_fn { p(app, self); }
    }
}

#[repr(C)]
pub struct App {
    pub user_data: *mut c_void,
    pub on_app_cmd: Option<extern "C" fn(*mut App, i32)>,
    pub on_input_event: Option<extern "C" fn(*mut App, *mut AInputEvent) -> i32>,
    pub activity: *mut ANativeActivity,
    pub config: *mut AConfiguration,
    pub saved_state: *mut c_void, pub saved_state_size: size_t,
    pub looper: *mut ALooper,
    pub input_queue: *mut AInputQueue,
    pub window: *mut ANativeWindow,
    pub content_rect: ARect,
    pub activity_state: c_int,
    pub destroy_requested: c_int,
    mutex: pthread_mutex_t, cond: pthread_cond_t,
    msgread: c_int, msgwrite: c_int,
    thread: pthread_t,
    cmd_poll_source: PollSource, input_poll_source: PollSource,
    running: c_int, state_saved: c_int, destroyed: c_int, redraw_needed: c_int,
    pending_input_queue: *mut AInputQueue, pending_window: *mut ANativeWindow, pending_content_rect: ARect
}

pub const APP_CMD_INPUT_CHANGED: i32 = 0;
pub const APP_CMD_INIT_WINDOW: i32 = 1;
pub const APP_CMD_TERM_WINDOW: i32 = 2;
pub const APP_CMD_WINDOW_RESIZED: i32 = 3;
pub const APP_CMD_WINDOW_REDRAW_NEEDED: i32 = 4;
pub const APP_CMD_CONTENT_RECT_CHANGED: i32 = 5;
pub const APP_CMD_GAINED_FOCUS: i32 = 6;
pub const APP_CMD_LOST_FOCUS: i32 = 7;
pub const APP_CMD_CONFIG_CHANGED: i32 = 8;
pub const APP_CMD_LOW_MEMORY: i32 = 9;
pub const APP_CMD_START: i32 = 10;
pub const APP_CMD_RESUME: i32 = 11;
pub const APP_CMD_SAVE_STATE: i32 = 12;
pub const APP_CMD_PAUSE: i32 = 13;
pub const APP_CMD_STOP: i32 = 14;
pub const APP_CMD_DESTROY: i32 = 15;
