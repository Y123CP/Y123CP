use core::ffi::*;
use crate::src::pngxtern::pngxrbmp::pngx_sig_is_bmp;
use crate::src::pngxtern::pngxrgif::pngx_sig_is_gif;
use crate::src::pngxtern::pngxrjpg::pngx_sig_is_jpeg;
use crate::src::pngxtern::pngxrpnm::pngx_sig_is_pnm;
use crate::src::pngxtern::pngxrtif::pngx_sig_is_tiff;
pub use crate::src::ffi::*;
pub use crate::src::c_consts::*;
pub use crate::src::c_structs::*;
pub use crate::src::c_types::*;
pub use crate::src::c_extern_types::*;
pub use crate::src::libpng::pngwrite::png_info_def;
pub use crate::src::libpng::pngwrite::png_struct_def;
extern "C" {
    fn fgetpos(__stream: *mut FILE, __pos: *mut fpos_t) -> c_int;
    fn fsetpos(__stream: *mut FILE, __pos: *const fpos_t) -> c_int;
    fn png_get_io_ptr(png_ptr: png_const_structrp) -> png_voidp;
    fn png_error(png_ptr: png_const_structrp, error_message: png_const_charp) -> !;
    fn png_warning(png_ptr: png_const_structrp, warning_message: png_const_charp);
    fn png_read_png(
        png_ptr: png_structrp,
        info_ptr: png_inforp,
        transforms: c_int,
        params: png_voidp,
    );
    fn pngx_read_bmp(
        png_ptr: png_structp,
        info_ptr: png_infop,
        stream: *mut FILE,
    ) -> c_int;
    fn pngx_read_gif(
        png_ptr: png_structp,
        info_ptr: png_infop,
        stream: *mut FILE,
    ) -> c_int;
    fn pngx_read_jpeg(
        png_ptr: png_structp,
        info_ptr: png_infop,
        stream: *mut FILE,
    ) -> c_int;
    fn pngx_read_pnm(
        png_ptr: png_structp,
        info_ptr: png_infop,
        stream: *mut FILE,
    ) -> c_int;
    fn pngx_read_tiff(
        png_ptr: png_structp,
        info_ptr: png_infop,
        stream: *mut FILE,
    ) -> c_int;
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct __mbstate_t {
    pub __count: c_int,
    pub __value: C2RustUnnamed_hua5a5cedf,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_hua5a5cedf {
    pub __wch: c_uint,
    pub __wchb: [c_char; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _G_fpos_t {
    pub __pos: __off_t,
    pub __state: __mbstate_t,
}
pub type __fpos_t = _G_fpos_t;

pub type fpos_t = __fpos_t;

pub type png_struct = png_struct_def;
pub type png_structp = *mut png_struct;
pub type png_info = png_info_def;
pub type png_infop = *mut png_info;
pub type png_structrp = *mut png_struct;
pub type png_const_structrp = *const png_struct;
pub type png_inforp = *mut png_info;

fn pngx_sig_is_png(
    mut png_ptr: png_structp,
    mut sig: png_bytep,
    mut sig_size: size_t,
    mut fmt_name_ptr: png_const_charpp,
    mut fmt_long_name_ptr: png_const_charpp,
) -> c_int { unsafe {
    static mut pngx_png_standalone_fmt_name: [c_char; 4] =
        unsafe { ::core::mem::transmute::<[u8; 4], [c_char; 4]>(*b"PNG\0") };
    static mut pngx_png_datastream_fmt_name: [c_char; 15] = unsafe {
        ::core::mem::transmute::<[u8; 15], [c_char; 15]>(*b"PNG datastream\0")
    };
    static mut pngx_png_standalone_fmt_long_name: [c_char; 26] = unsafe {
        ::core::mem::transmute::<[u8; 26], [c_char; 26]>(
            *b"Portable Network Graphics\0",
        )
    };
    static mut pngx_png_datastream_fmt_long_name: [c_char; 46] = unsafe {
        ::core::mem::transmute::<[u8; 46], [c_char; 46]>(
            *b"Portable Network Graphics embedded datastream\0",
        )
    };
    static mut png_file_sig: [png_byte; 8] = [
        137 as c_int as png_byte,
        80 as c_int as png_byte,
        78 as c_int as png_byte,
        71 as c_int as png_byte,
        13 as c_int as png_byte,
        10 as c_int as png_byte,
        26 as c_int as png_byte,
        10 as c_int as png_byte,
    ];
    static mut mng_file_sig: [png_byte; 8] = [
        138 as c_int as png_byte,
        77 as c_int as png_byte,
        78 as c_int as png_byte,
        71 as c_int as png_byte,
        13 as c_int as png_byte,
        10 as c_int as png_byte,
        26 as c_int as png_byte,
        10 as c_int as png_byte,
    ];
    static mut png_ihdr_sig: [png_byte; 8] = [
        0 as c_int as png_byte,
        0 as c_int as png_byte,
        0 as c_int as png_byte,
        13 as c_int as png_byte,
        73 as c_int as png_byte,
        72 as c_int as png_byte,
        68 as c_int as png_byte,
        82 as c_int as png_byte,
    ];
    let mut has_png_sig: c_int = 0;
    if sig_size <= (25 as c_int + 18 as c_int) as size_t {
        return -(1 as c_int);
    }
    has_png_sig = (memcmp(
        sig as *const c_void,
        &raw const png_file_sig as *const png_byte as *const c_void,
        8 as size_t,
    ) == 0 as c_int) as c_int;
    if memcmp(
        sig.offset(
            (if has_png_sig != 0 {
                8 as c_int
            } else {
                0 as c_int
            }) as isize,
        ) as *const c_void,
        &raw const png_ihdr_sig as *const png_byte as *const c_void,
        8 as size_t,
    ) != 0 as c_int
    {
        if memcmp(
            sig as *const c_void,
            &raw const png_file_sig as *const png_byte as *const c_void,
            4 as size_t,
        ) == 0 as c_int
            && (*sig.offset(4 as c_int as isize) as c_int
                == 10 as c_int
                || *sig.offset(4 as c_int as isize) as c_int
                    == 13 as c_int)
        {
            png_error(
                png_ptr as png_const_structrp,
                b"PNG file appears to be corrupted by text file conversions\0" as *const u8
                    as png_const_charp,
            );
        } else if memcmp(
            sig as *const c_void,
            &raw const mng_file_sig as *const png_byte as *const c_void,
            8 as size_t,
        ) == 0 as c_int
        {
            png_error(
                png_ptr as png_const_structrp,
                b"MNG decoding is not supported\0" as *const u8 as png_const_charp,
            );
        }
        return 0 as c_int;
    }
    if !fmt_name_ptr.is_null() {
        *fmt_name_ptr = if has_png_sig != 0 {
            &raw const pngx_png_standalone_fmt_name as *const c_char
        } else {
            &raw const pngx_png_datastream_fmt_name as *const c_char
        };
    }
    if !fmt_long_name_ptr.is_null() {
        *fmt_long_name_ptr = if has_png_sig != 0 {
            &raw const pngx_png_standalone_fmt_long_name as *const c_char
        } else {
            &raw const pngx_png_datastream_fmt_long_name as *const c_char
        };
    }
    return 1 as c_int;
} }
#[inline]
pub fn pngx_read_image(
    mut png_ptr: png_structp,
    mut info_ptr: png_infop,
    mut fmt_name_ptr: png_const_charpp,
    mut fmt_long_name_ptr: png_const_charpp,
) -> c_int { unsafe {
    let mut sig: [png_byte; 128] = [0; 128];
    let mut num: size_t = 0;
    let mut read_fn: Option<
        unsafe extern "C" fn(png_structp, png_infop, *mut FILE) -> c_int,
    > = None;
    let mut stream: *mut FILE = ::core::ptr::null_mut::<FILE>();
    let mut fpos: fpos_t = fpos_t {
        __pos: 0,
        __state: __mbstate_t {
            __count: 0,
            __value: C2RustUnnamed_hua5a5cedf { __wch: 0 },
        },
    };
    let mut result: c_int = 0;
    stream = png_get_io_ptr(png_ptr as png_const_structrp) as *mut FILE;
    if fgetpos(stream, &raw mut fpos) != 0 as c_int {
        png_error(
            png_ptr as png_const_structrp,
            b"Can't ftell in input file stream\0" as *const u8 as png_const_charp,
        );
    }
    num = fread(
        &raw mut sig as *mut png_byte as *mut c_void,
        1 as size_t,
        ::core::mem::size_of::<[png_byte; 128]>() as size_t,
        stream,
    ) as size_t;
    if fsetpos(stream, &raw mut fpos) != 0 as c_int {
        png_error(
            png_ptr as png_const_structrp,
            b"Can't fseek in input file stream\0" as *const u8 as png_const_charp,
        );
    }
    if pngx_sig_is_png(
        png_ptr,
        &raw mut sig as png_bytep,
        num,
        fmt_name_ptr,
        fmt_long_name_ptr,
    ) > 0 as c_int
    {
        png_read_png(
            png_ptr as png_structrp,
            info_ptr as png_inforp,
            0 as c_int,
            NULL,
        );
        if getc(stream) != EOF {
            png_warning(
                png_ptr as png_const_structrp,
                b"Extraneous data found after IEND\0" as *const u8 as png_const_charp,
            );
            fseek(stream, 0 as c_long, SEEK_END);
        }
        return 1 as c_int;
    }
    if pngx_sig_is_bmp(
        &raw mut sig as png_bytep,
        num,
        fmt_name_ptr,
        fmt_long_name_ptr,
    ) > 0 as c_int
    {
        read_fn = Some(
            pngx_read_bmp
                as unsafe extern "C" fn(png_structp, png_infop, *mut FILE) -> c_int,
        )
            as Option<
                unsafe extern "C" fn(png_structp, png_infop, *mut FILE) -> c_int,
            >;
    } else if pngx_sig_is_gif(
        &raw mut sig as png_bytep,
        num,
        fmt_name_ptr,
        fmt_long_name_ptr,
    ) > 0 as c_int
    {
        read_fn = Some(
            pngx_read_gif
                as unsafe extern "C" fn(png_structp, png_infop, *mut FILE) -> c_int,
        )
            as Option<
                unsafe extern "C" fn(png_structp, png_infop, *mut FILE) -> c_int,
            >;
    } else if pngx_sig_is_jpeg(
        &raw mut sig as png_bytep,
        num,
        fmt_name_ptr,
        fmt_long_name_ptr,
    ) > 0 as c_int
    {
        read_fn = Some(
            pngx_read_jpeg
                as unsafe extern "C" fn(png_structp, png_infop, *mut FILE) -> c_int,
        )
            as Option<
                unsafe extern "C" fn(png_structp, png_infop, *mut FILE) -> c_int,
            >;
    } else if pngx_sig_is_pnm(
        &raw mut sig as png_bytep,
        num,
        fmt_name_ptr,
        fmt_long_name_ptr,
    ) > 0 as c_int
    {
        read_fn = Some(
            pngx_read_pnm
                as unsafe extern "C" fn(png_structp, png_infop, *mut FILE) -> c_int,
        )
            as Option<
                unsafe extern "C" fn(png_structp, png_infop, *mut FILE) -> c_int,
            >;
    } else if pngx_sig_is_tiff(
        &raw mut sig as png_bytep,
        num,
        fmt_name_ptr,
        fmt_long_name_ptr,
    ) > 0 as c_int
    {
        read_fn = Some(
            pngx_read_tiff
                as unsafe extern "C" fn(png_structp, png_infop, *mut FILE) -> c_int,
        )
            as Option<
                unsafe extern "C" fn(png_structp, png_infop, *mut FILE) -> c_int,
            >;
    } else {
        return 0 as c_int;
    }
    result = read_fn.expect("non-null function pointer")(png_ptr, info_ptr, stream);
    if result <= 0 as c_int {
        if fsetpos(stream, &raw mut fpos) != 0 as c_int {
            png_error(
                png_ptr as png_const_structrp,
                b"Can't fseek in input file stream\0" as *const u8 as png_const_charp,
            );
        }
    }
    return result;
} }
