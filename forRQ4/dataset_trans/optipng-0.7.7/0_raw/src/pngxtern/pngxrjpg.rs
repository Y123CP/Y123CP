extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type png_struct_def;
    pub type png_info_def;
    fn fread(
        __ptr: *mut ::core::ffi::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> ::core::ffi::c_ulong;
    fn png_error(png_ptr: png_const_structrp, error_message: png_const_charp) -> !;
    fn memcmp(
        __s1: *const ::core::ffi::c_void,
        __s2: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> ::core::ffi::c_int;
}
pub type size_t = usize;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: ::core::ffi::c_int,
    pub _IO_read_ptr: *mut ::core::ffi::c_char,
    pub _IO_read_end: *mut ::core::ffi::c_char,
    pub _IO_read_base: *mut ::core::ffi::c_char,
    pub _IO_write_base: *mut ::core::ffi::c_char,
    pub _IO_write_ptr: *mut ::core::ffi::c_char,
    pub _IO_write_end: *mut ::core::ffi::c_char,
    pub _IO_buf_base: *mut ::core::ffi::c_char,
    pub _IO_buf_end: *mut ::core::ffi::c_char,
    pub _IO_save_base: *mut ::core::ffi::c_char,
    pub _IO_backup_base: *mut ::core::ffi::c_char,
    pub _IO_save_end: *mut ::core::ffi::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: ::core::ffi::c_int,
    pub _flags2: ::core::ffi::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: ::core::ffi::c_ushort,
    pub _vtable_offset: ::core::ffi::c_schar,
    pub _shortbuf: [::core::ffi::c_char; 1],
    pub _lock: *mut ::core::ffi::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut ::core::ffi::c_void,
    pub __pad5: size_t,
    pub _mode: ::core::ffi::c_int,
    pub _unused2: [::core::ffi::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type png_byte = ::core::ffi::c_uchar;
pub type png_bytep = *mut png_byte;
pub type png_const_charp = *const ::core::ffi::c_char;
pub type png_const_charpp = *mut *const ::core::ffi::c_char;
pub type png_struct = png_struct_def;
pub type png_structp = *mut png_struct;
pub type png_info = png_info_def;
pub type png_infop = *mut png_info;
pub type png_const_structrp = *const png_struct;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const JPEG_SIG_JP2_SIZE: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
pub const JPEG_SIG_JPC_SIZE: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const JPEG_SIG_JNG_SIZE: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const JPEG_SIG_SIZE_MAX: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
static mut jpeg_sig_jp2: [png_byte; 12] = [
    0 as ::core::ffi::c_int as png_byte,
    0 as ::core::ffi::c_int as png_byte,
    0 as ::core::ffi::c_int as png_byte,
    0xc as ::core::ffi::c_int as png_byte,
    0x6a as ::core::ffi::c_int as png_byte,
    0x50 as ::core::ffi::c_int as png_byte,
    0x20 as ::core::ffi::c_int as png_byte,
    0x20 as ::core::ffi::c_int as png_byte,
    0xd as ::core::ffi::c_int as png_byte,
    0xa as ::core::ffi::c_int as png_byte,
    0x87 as ::core::ffi::c_int as png_byte,
    0xa as ::core::ffi::c_int as png_byte,
];
static mut jpeg_sig_jpc: [png_byte; 4] = [
    0xff as ::core::ffi::c_int as png_byte,
    0x4f as ::core::ffi::c_int as png_byte,
    0xff as ::core::ffi::c_int as png_byte,
    0x51 as ::core::ffi::c_int as png_byte,
];
static mut jpeg_sig_jng: [png_byte; 8] = [
    0x8b as ::core::ffi::c_int as png_byte,
    0x4a as ::core::ffi::c_int as png_byte,
    0x4e as ::core::ffi::c_int as png_byte,
    0x47 as ::core::ffi::c_int as png_byte,
    0xd as ::core::ffi::c_int as png_byte,
    0xa as ::core::ffi::c_int as png_byte,
    0x1a as ::core::ffi::c_int as png_byte,
    0xa as ::core::ffi::c_int as png_byte,
];
static mut jpeg_sig_jng_jhdr: [png_byte; 8] = [
    0 as ::core::ffi::c_int as png_byte,
    0 as ::core::ffi::c_int as png_byte,
    0 as ::core::ffi::c_int as png_byte,
    0x1a as ::core::ffi::c_int as png_byte,
    0x4a as ::core::ffi::c_int as png_byte,
    0x48 as ::core::ffi::c_int as png_byte,
    0x44 as ::core::ffi::c_int as png_byte,
    0x52 as ::core::ffi::c_int as png_byte,
];
#[no_mangle]
pub unsafe extern "C" fn pngx_sig_is_jpeg(
    mut sig: png_bytep,
    mut sig_size: size_t,
    mut fmt_name_ptr: png_const_charpp,
    mut fmt_long_name_ptr: png_const_charpp,
) -> ::core::ffi::c_int {
    let mut fmt: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut marker: ::core::ffi::c_uint = 0;
    let mut result: ::core::ffi::c_int = 0;
    if sig_size < JPEG_SIG_SIZE_MAX as size_t {
        return -(1 as ::core::ffi::c_int);
    }
    if *sig.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        == 0xff as ::core::ffi::c_int
        && *sig.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 0xd8 as ::core::ffi::c_int
        && *sig.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 0xff as ::core::ffi::c_int
    {
        marker = 0xff00 as ::core::ffi::c_uint
            | *sig.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint;
        if marker >= 0xffc0 as ::core::ffi::c_uint && marker <= 0xffcf as ::core::ffi::c_uint
            || marker >= 0xffda as ::core::ffi::c_uint && marker <= 0xfffe as ::core::ffi::c_uint
        {
            fmt = b"JPEG\0" as *const u8 as *const ::core::ffi::c_char;
            result = 1 as ::core::ffi::c_int;
        } else {
            return 0 as ::core::ffi::c_int;
        }
    } else if memcmp(
        sig as *const ::core::ffi::c_void,
        &raw const jpeg_sig_jp2 as *const png_byte as *const ::core::ffi::c_void,
        JPEG_SIG_JP2_SIZE as size_t,
    ) == 0 as ::core::ffi::c_int
        || memcmp(
            sig as *const ::core::ffi::c_void,
            &raw const jpeg_sig_jpc as *const png_byte as *const ::core::ffi::c_void,
            JPEG_SIG_JPC_SIZE as size_t,
        ) == 0 as ::core::ffi::c_int
    {
        fmt = b"JPEG-2000\0" as *const u8 as *const ::core::ffi::c_char;
        result = 2 as ::core::ffi::c_int;
    } else if memcmp(
        sig as *const ::core::ffi::c_void,
        &raw const jpeg_sig_jng as *const png_byte as *const ::core::ffi::c_void,
        JPEG_SIG_JNG_SIZE as size_t,
    ) == 0 as ::core::ffi::c_int
        || memcmp(
            sig as *const ::core::ffi::c_void,
            &raw const jpeg_sig_jng_jhdr as *const png_byte as *const ::core::ffi::c_void,
            JPEG_SIG_JNG_SIZE as size_t,
        ) == 0 as ::core::ffi::c_int
    {
        fmt = b"JNG\0" as *const u8 as *const ::core::ffi::c_char;
        result = 3 as ::core::ffi::c_int;
    } else {
        return 0 as ::core::ffi::c_int;
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
) -> ::core::ffi::c_int {
    let mut buf: [png_byte; 12] = [0; 12];
    let mut sig_code: ::core::ffi::c_int = 0;
    if fread(
        &raw mut buf as *mut png_byte as *mut ::core::ffi::c_void,
        JPEG_SIG_SIZE_MAX as size_t,
        1 as size_t,
        stream,
    ) != 1 as ::core::ffi::c_ulong
    {
        return 0 as ::core::ffi::c_int;
    }
    sig_code = pngx_sig_is_jpeg(
        &raw mut buf as png_bytep,
        JPEG_SIG_SIZE_MAX as size_t,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
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
        return 0 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
