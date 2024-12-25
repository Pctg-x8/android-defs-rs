#[repr(C)]
pub struct AAssetManager(
    [u8; 0],
    core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
);

#[repr(C)]
pub struct AAssetDir(
    [u8; 0],
    core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
);

#[repr(C)]
pub struct AAsset(
    [u8; 0],
    core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
);

pub const AASSET_MODE_UNKNOWN: core::ffi::c_int = 0;
pub const AASSET_MODE_RANDOM: core::ffi::c_int = 1;
pub const AASSET_MODE_STREAMING: core::ffi::c_int = 2;
pub const AASSET_MODE_BUFFER: core::ffi::c_int = 3;

#[allow(non_camel_case_types)]
pub type off_t = core::ffi::c_longlong;
#[allow(non_camel_case_types)]
pub type off64_t = core::ffi::c_longlong;

#[allow(non_snake_case)]
unsafe extern "C" {
    pub unsafe fn AAssetManager_fromJava(
        env: jni::JNIEnv,
        assetManager: jni::objects::JObject,
    ) -> *mut AAssetManager;
    pub unsafe fn AAssetManager_openDir(
        mgr: *mut AAssetManager,
        dirName: *const core::ffi::c_char,
    ) -> *mut AAssetDir;
    pub unsafe fn AAssetManager_open(
        mgr: *mut AAssetManager,
        filename: *const core::ffi::c_char,
        mode: core::ffi::c_int,
    ) -> *mut AAsset;
    pub unsafe fn AAssetDir_getNextFileName(assetDir: *mut AAssetDir) -> *const core::ffi::c_char;
    pub unsafe fn AAssetDir_rewind(assetDir: *mut AAssetDir);
    pub unsafe fn AAssetDir_close(assetDir: *mut AAssetDir);
    pub unsafe fn AAsset_read(
        asset: *mut AAsset,
        buf: *mut core::ffi::c_void,
        count: usize,
    ) -> core::ffi::c_int;
    pub unsafe fn AAsset_seek(asset: *mut AAsset, offset: off_t, whence: core::ffi::c_int)
        -> off_t;
    pub unsafe fn AAsset_seek64(
        asset: *mut AAsset,
        offset: off64_t,
        whence: core::ffi::c_int,
    ) -> off64_t;
    pub unsafe fn AAsset_close(asset: *mut AAsset);
    pub unsafe fn AAsset_getBuffer(asset: *mut AAsset) -> *const core::ffi::c_void;
    pub unsafe fn AAsset_getLength(asset: *mut AAsset) -> off_t;
    pub unsafe fn AAsset_getLength64(asset: *mut AAsset) -> off64_t;
    pub unsafe fn AAsset_getRemainingLength(asset: *mut AAsset) -> off_t;
    pub unsafe fn AAsset_getRemainingLength64(asset: *mut AAsset) -> off64_t;
    pub unsafe fn AAsset_openFileDescriptor(
        asset: *mut AAsset,
        outStart: *mut off_t,
        outLength: *mut off_t,
    ) -> core::ffi::c_int;
    pub unsafe fn AAsset_openFileDescriptor64(
        asset: *mut AAsset,
        outStart: *mut off64_t,
        outLength: *mut off64_t,
    ) -> core::ffi::c_int;
    pub unsafe fn AAsset_isAllocated(asset: *mut AAsset) -> core::ffi::c_int;
}
