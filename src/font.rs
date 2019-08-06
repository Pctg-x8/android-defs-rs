
use libc::c_char;

pub enum AFontMatcher {}
pub enum AFont {}

#[allow(non_snake_case)]
extern "C"
{
    fn AFontMatcher_create() -> *mut AFontMatcher;
    fn AFontMatcher_destroy(matcher: *mut AFontMatcher);
    fn AFontMatcher_match(matcher: *const AFontMatcher, family_name: *const c_char,
        text: *const u16, text_length: u32, runLengthOut: *mut u32) -> *mut AFont;
    fn AFontMatcher_setFamilyVariant(matcher: *mut AFontMatcher, variant: u32);
    fn AFontMatcher_setStyle(matcher: *mut AFontMatcher, weight: u16, italic: bool);

    fn AFont_close(font: *mut AFont);
    fn AFont_getFontFilePath(font: *const AFont) -> *const c_char;
}

use std::ptr::NonNull;
use std::ffi::{CStr, CString};
use widestring::U16CString;

#[repr(C)] #[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FamilyVariant
{
    Default = 0, Compact = 1, Elegant = 2
}

pub struct FontMatcher(NonNull<AFontMatcher>);
impl FontMatcher
{
    pub fn new() -> Option<Self> { unsafe { NonNull::new(AFontMatcher_create()).map(FontMatcher) } }

    pub fn set_family_variant(&mut self, variant: FamilyVariant)
    {
        unsafe { AFontMatcher_setFamilyVariant(self.0.as_ptr(), variant as _) }
    }
    pub fn set_style(&mut self, weight: u16, italic: bool)
    {
        unsafe { AFontMatcher_setStyle(self.0.as_ptr(), weight, italic) }
    }

    pub fn match_font(&self, family_name: &CString, text: &U16CString, run_length_out: Option<&mut u32>) -> Option<Font>
    {
        unsafe
        {
            Font::from_ptr(AFontMatcher_match(self.0.as_ptr(), family_name.as_ptr(), text.as_ptr(), text.len() as _,
                run_length_out.map_or_else(std::ptr::null_mut, |x| x as *mut _)))
        }
    }
}
impl Drop for FontMatcher
{
    fn drop(&mut self) { unsafe { AFontMatcher_destroy(self.0.as_ptr()); } }
}

pub struct Font(NonNull<AFont>);
impl Font
{
    pub unsafe fn from_ptr(p: *mut AFont) -> Option<Self> { NonNull::new(p).map(Font) }

    pub fn file_path(&self) -> &CStr
    {
        unsafe { CStr::from_ptr(AFont_getFontFilePath(self.0.as_ptr())) }
    }
}
impl Drop for Font
{
    fn drop(&mut self) { unsafe { AFont_close(self.0.as_ptr()); } }
}
