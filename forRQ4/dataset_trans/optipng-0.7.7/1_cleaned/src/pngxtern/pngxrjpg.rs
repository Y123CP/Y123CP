use core::ffi::*;
pub use crate::src::ffi::*;
pub use crate::src::c_consts::*;
pub use crate::src::c_structs::*;
pub use crate::src::c_types::*;
pub use crate::src::c_extern_types::*;
extern "C" {
    pub type png_struct_def;
    pub type png_info_def;
    fn png_error(png_ptr: png_const_structrp, error_message: png_const_charp) -> !;
}

pub type png_struct = png_struct_def;
pub type png_structp = *mut png_struct;
pub type png_info = png_info_def;
pub type png_infop = *mut png_info;
pub type png_const_structrp = *const png_struct;

pub const JPEG_SIG_JP2_SIZE: c_int = 12 as c_int;
pub const JPEG_SIG_JPC_SIZE: c_int = 4 as c_int;
pub const JPEG_SIG_JNG_SIZE: c_int = 8 as c_int;
pub const JPEG_SIG_SIZE_MAX: c_int = 12 as c_int;
static mut jpeg_sig_jp2: [png_byte; 12] = [
    0 as c_int as png_byte,
    0 as c_int as png_byte,
    0 as c_int as png_byte,
    0xc as c_int as png_byte,
    0x6a as c_int as png_byte,
    0x50 as c_int as png_byte,
    0x20 as c_int as png_byte,
    0x20 as c_int as png_byte,
    0xd as c_int as png_byte,
    0xa as c_int as png_byte,
    0x87 as c_int as png_byte,
    0xa as c_int as png_byte,
];
static mut jpeg_sig_jpc: [png_byte; 4] = [
    0xff as c_int as png_byte,
    0x4f as c_int as png_byte,
    0xff as c_int as png_byte,
    0x51 as c_int as png_byte,
];
static mut jpeg_sig_jng: [png_byte; 8] = [
    0x8b as c_int as png_byte,
    0x4a as c_int as png_byte,
    0x4e as c_int as png_byte,
    0x47 as c_int as png_byte,
    0xd as c_int as png_byte,
    0xa as c_int as png_byte,
    0x1a as c_int as png_byte,
    0xa as c_int as png_byte,
];
static mut jpeg_sig_jng_jhdr: [png_byte; 8] = [
    0 as c_int as png_byte,
    0 as c_int as png_byte,
    0 as c_int as png_byte,
    0x1a as c_int as png_byte,
    0x4a as c_int as png_byte,
    0x48 as c_int as png_byte,
    0x44 as c_int as png_byte,
    0x52 as c_int as png_byte,
];
#[no_mangle]
pub unsafe extern "C" fn pngx_sig_is_jpeg(
    mut sig: png_bytep,
    mut sig_size: size_t,
    mut fmt_name_ptr: png_const_charpp,
    mut fmt_long_name_ptr: png_const_charpp,
) -> c_int {
    let mut fmt: *const c_char = ::core::ptr::null::<c_char>();
    let mut marker: c_uint = 0;
    let mut result: c_int = 0;
    if sig_size < JPEG_SIG_SIZE_MAX as size_t {
        return -(1 as c_int);
    }
    if *sig.offset(0 as c_int as isize) as c_int
        == 0xff as c_int
        && *sig.offset(1 as c_int as isize) as c_int
            == 0xd8 as c_int
        && *sig.offset(2 as c_int as isize) as c_int
            == 0xff as c_int
    {
        marker = 0xff00 as c_uint
            | *sig.offset(3 as c_int as isize) as c_uint;
        if marker >= 0xffc0 as c_uint && marker <= 0xffcf as c_uint
            || marker >= 0xffda as c_uint && marker <= 0xfffe as c_uint
        {
            fmt = b"JPEG\0" as *const u8 as *const c_char;
            result = 1 as c_int;
        } else {
            return 0 as c_int;
        }
    } else if memcmp(
        sig as *const c_void,
        &raw const jpeg_sig_jp2 as *const png_byte as *const c_void,
        JPEG_SIG_JP2_SIZE as size_t,
    ) == 0 as c_int
        || memcmp(
            sig as *const c_void,
            &raw const jpeg_sig_jpc as *const png_byte as *const c_void,
            JPEG_SIG_JPC_SIZE as size_t,
        ) == 0 as c_int
    {
        fmt = b"JPEG-2000\0" as *const u8 as *const c_char;
        result = 2 as c_int;
    } else if memcmp(
        sig as *const c_void,
        &raw const jpeg_sig_jng as *const png_byte as *const c_void,
        JPEG_SIG_JNG_SIZE as size_t,
    ) == 0 as c_int
        || memcmp(
            sig as *const c_void,
            &raw const jpeg_sig_jng_jhdr as *const png_byte as *const c_void,
            JPEG_SIG_JNG_SIZE as size_t,
        ) == 0 as c_int
    {
        fmt = b"JNG\0" as *const u8 as *const c_char;
        result = 3 as c_int;
    } else {
        return 0 as c_int;
    }
    if !fmt_name_ptr.is_null() {
        *fmt_name_ptr = fmt;
    }
    if !fmt_long_name_ptr.is_null() {
        *fmt_long_name_ptr = fmt;
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn pngx_read_jpeg(
    mut png_ptr: png_structp,
    mut info_ptr: png_infop,
    mut stream: *mut FILE,
) -> c_int {
    let mut buf: [png_byte; 12] = [0; 12];
    let mut sig_code: c_int = 0;
    if fread(
        &raw mut buf as *mut png_byte as *mut c_void,
        JPEG_SIG_SIZE_MAX as size_t,
        1 as size_t,
        stream,
    ) != 1 as c_ulong
    {
        return 0 as c_int;
    }
    sig_code = pngx_sig_is_jpeg(
        &raw mut buf as png_bytep,
        JPEG_SIG_SIZE_MAX as size_t,
        ::core::ptr::null_mut::<*const c_char>(),
        ::core::ptr::null_mut::<*const c_char>(),
    );
    match sig_code {
        1 => {
            png_error(
                png_ptr as png_const_structrp,
                b"JPEG decoding is not supported\0" as *const u8 as png_const_charp,
            );
        }
        2 => {
            png_error(
                png_ptr as png_const_structrp,
                b"JPEG-2000 decoding is not supported\0" as *const u8 as png_const_charp,
            );
        }
        3 => {
            png_error(
                png_ptr as png_const_structrp,
                b"JNG (JPEG) decoding is not supported\0" as *const u8 as png_const_charp,
            );
        }
        _ => {}
    }
    if info_ptr.is_null() {
        return 0 as c_int;
    }
    return 0 as c_int;
}
