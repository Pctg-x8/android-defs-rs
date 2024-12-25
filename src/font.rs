pub const AFONT_WEIGHT_THIN: u16 = 100;
pub const AFONT_WEIGHT_EXTRA_LIGHT: u16 = 200;
pub const AFONT_WEIGHT_LIGHT: u16 = 300;
pub const AFONT_WEIGHT_NORMAL: u16 = 400;
pub const AFONT_WEIGHT_MEDIUM: u16 = 500;
pub const AFONT_WEIGHT_SEMI_BOLD: u16 = 600;
pub const AFONT_WEIGHT_BOLD: u16 = 700;
pub const AFONT_WEIGHT_EXTRA_BOLD: u16 = 800;
pub const AFONT_WEIGHT_BLACK: u16 = 900;

pub const AFAMILY_VARIANT_DEFAULT: u32 = 0;
pub const AFAMILY_VARIANT_COMPACT: u32 = 1;
pub const AFAMILY_VARIANT_ELEGANT: u32 = 2;

#[repr(C)]
pub struct AFontMatcher(
    [u8; 0],
    core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
);

#[repr(C)]
pub struct AFont(
    [u8; 0],
    core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
);

#[repr(C)]
pub struct ASystemFontIterator(
    [u8; 0],
    core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
);

#[allow(non_snake_case)]
unsafe extern "C" {
    pub unsafe fn AFontMatcher_create() -> *mut AFontMatcher;
    pub unsafe fn AFontMatcher_destroy(matcher: *mut AFontMatcher);
    pub unsafe fn AFontMatcher_match(
        matcher: *const AFontMatcher,
        family_name: *const core::ffi::c_char,
        text: *const u16,
        text_length: u32,
        runLengthOut: *mut u32,
    ) -> *mut AFont;
    pub unsafe fn AFontMatcher_setFamilyVariant(matcher: *mut AFontMatcher, family_variant: u32);
    pub unsafe fn AFontMatcher_setLocales(
        matcher: *mut AFontMatcher,
        language_tags: *const core::ffi::c_char,
    );
    pub unsafe fn AFontMatcher_setStyle(matcher: *mut AFontMatcher, weight: u16, italic: bool);

    pub unsafe fn AFont_close(font: *mut AFont);
    pub unsafe fn AFont_getAxisCount(font: *const AFont) -> usize;
    pub unsafe fn AFont_getAxisTag(font: *const AFont, axis_index: u32) -> u32;
    pub unsafe fn AFont_getAxisValue(font: *const AFont, axis_index: u32) -> core::ffi::c_float;
    pub unsafe fn AFont_getCollectionIndex(font: *const AFont) -> usize;
    pub unsafe fn AFont_getFontFilePath(font: *const AFont) -> *const core::ffi::c_char;
    pub unsafe fn AFont_getLocale(font: *const AFont) -> *const core::ffi::c_char;
    pub unsafe fn AFont_getWeight(font: *const AFont) -> u16;
    pub unsafe fn AFont_isItalic(font: *const AFont) -> bool;
    pub unsafe fn ASystemFontIterator_close(iterator: *mut ASystemFontIterator);
    pub unsafe fn ASystemFontIterator_next(iterator: *mut ASystemFontIterator) -> *mut AFont;
    pub unsafe fn ASystemFontIterator_open() -> *mut ASystemFontIterator;
}
