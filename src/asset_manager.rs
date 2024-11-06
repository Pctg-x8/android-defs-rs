use libc::*;
use std::io::{Error as IOError, ErrorKind, Read, Result as IOResult, Seek, SeekFrom};
use std::mem::transmute;
use std::ptr::NonNull;

pub enum AAssetManager {}
pub enum AAssetDir {}
pub enum AAsset {}

pub const AASSET_MODE_UNKNOWN: c_int = 0;
pub const AASSET_MODE_RANDOM: c_int = 1;
pub const AASSET_MODE_STREAMING: c_int = 2;
pub const AASSET_MODE_BUFFER: c_int = 3;

pub type off_t = c_longlong;
pub type off64_t = c_longlong;

#[allow(non_snake_case)]
extern "C" {
    pub fn AAssetManager_fromJava(
        env: jni::JNIEnv,
        assetManager: jni::objects::JObject,
    ) -> *mut AAssetManager;
    pub fn AAssetManager_openDir(mgr: *mut AAssetManager, dirName: *const c_char)
        -> *mut AAssetDir;
    pub fn AAssetManager_open(
        mgr: *mut AAssetManager,
        filename: *const c_char,
        mode: c_int,
    ) -> *mut AAsset;
    pub fn AAssetDir_getNextFileName(assetDir: *mut AAssetDir) -> *const c_char;
    pub fn AAssetDir_rewind(assetDir: *mut AAssetDir);
    pub fn AAssetDir_close(assetDir: *mut AAssetDir);
    pub fn AAsset_read(asset: *mut AAsset, buf: *mut c_void, count: size_t) -> c_int;
    pub fn AAsset_seek(asset: *mut AAsset, offset: off_t, whence: c_int) -> off_t;
    pub fn AAsset_seek64(asset: *mut AAsset, offset: off64_t, whence: c_int) -> off64_t;
    pub fn AAsset_close(asset: *mut AAsset);
    pub fn AAsset_getBuffer(asset: *mut AAsset) -> *const c_void;
    pub fn AAsset_getLength(asset: *mut AAsset) -> off_t;
    pub fn AAsset_getLength64(asset: *mut AAsset) -> off64_t;
    pub fn AAsset_getRemainingLength(asset: *mut AAsset) -> off_t;
    pub fn AAsset_getRemainingLength64(asset: *mut AAsset) -> off64_t;
    pub fn AAsset_openFileDescriptor(
        asset: *mut AAsset,
        outStart: *mut off_t,
        outLength: *mut off_t,
    ) -> c_int;
    pub fn AAsset_openFileDescriptor64(
        asset: *mut AAsset,
        outStart: *mut off64_t,
        outLength: *mut off64_t,
    ) -> c_int;
    pub fn AAsset_isAllocated(asset: *mut AAsset) -> c_int;
}

pub struct AssetManager(NonNull<AAssetManager>);
unsafe impl Sync for AssetManager {}
unsafe impl Send for AssetManager {}
impl AssetManager {
    pub unsafe fn from_ptr(p: *mut AAssetManager) -> Option<Self> {
        NonNull::new(p).map(Self)
    }
    pub unsafe fn from_java(env: jni::JNIEnv, object: jni::objects::JObject) -> Option<Self> {
        Self::from_ptr(AAssetManager_fromJava(env, object))
    }

    pub fn open(&self, name: *const c_char, mode: c_int) -> Option<Asset> {
        unsafe { Asset::from_ptr(AAssetManager_open(self.0.as_ptr(), name, mode)) }
    }
}

pub struct Asset(NonNull<AAsset>);
unsafe impl Sync for Asset {}
unsafe impl Send for Asset {}
impl Asset {
    pub unsafe fn from_ptr(p: *mut AAsset) -> Option<Self> {
        NonNull::new(p).map(Self)
    }
}
impl Read for Asset {
    fn read(&mut self, buf: &mut [u8]) -> IOResult<usize> {
        let read =
            unsafe { AAsset_read(self.0.as_ptr(), buf.as_mut_ptr() as *mut _, buf.len() as _) };
        if read < 0 {
            return Err(IOError::from_raw_os_error(-read));
        } else {
            return Ok(read as _);
        }
    }
}
impl Seek for Asset {
    fn seek(&mut self, pos: SeekFrom) -> IOResult<u64> {
        let p = match pos {
            SeekFrom::Current(p) => unsafe { AAsset_seek64(self.0.as_ptr(), p, SEEK_CUR) },
            SeekFrom::Start(p) => unsafe { AAsset_seek64(self.0.as_ptr(), transmute(p), SEEK_SET) },
            SeekFrom::End(p) => unsafe { AAsset_seek64(self.0.as_ptr(), p, SEEK_END) },
        };
        if unsafe { transmute::<_, i64>(p) < 0 } {
            return Err(IOError::new(ErrorKind::Other, ""));
        } else {
            return Ok(p as _);
        }
    }
}
impl Drop for Asset {
    fn drop(&mut self) {
        unsafe { AAsset_close(self.0.as_ptr()) }
    }
}
