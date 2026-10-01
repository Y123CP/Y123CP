use core::ffi::*;
pub use crate::src::ffi::*;
pub use crate::src::c_consts::*;
pub use crate::src::c_structs::*;
pub use crate::src::c_types::*;
pub use crate::src::c_extern_types::*;
extern "C" {
    pub type png_struct_def;
    pub type png_info_def;
    fn png_malloc(png_ptr: png_const_structrp, size: png_alloc_size_t) -> png_voidp;
    fn png_free(png_ptr: png_const_structrp, ptr: png_voidp);
    fn png_error(png_ptr: png_const_structrp, error_message: png_const_charp) -> !;
    fn png_warning(png_ptr: png_const_structrp, warning_message: png_const_charp);
    fn png_set_IHDR(
        png_ptr: png_const_structrp,
        info_ptr: png_inforp,
        width: png_uint_32,
        height: png_uint_32,
        bit_depth: c_int,
        color_type: c_int,
        interlace_method: c_int,
        compression_method: c_int,
        filter_method: c_int,
    );
    fn png_set_sBIT(png_ptr: png_const_structrp, info_ptr: png_inforp, sig_bit: png_const_color_8p);
    fn pngx_malloc_rows(
        png_ptr: png_structp,
        info_ptr: png_infop,
        filler: c_int,
    ) -> png_bytepp;
    fn pnm_fget_header(pnm_ptr: *mut pnm_struct, stream: *mut FILE) -> c_int;
    fn pnm_fget_values(
        pnm_ptr: *const pnm_struct,
        sample_values: *mut c_uint,
        num_rows: c_uint,
        stream: *mut FILE,
    ) -> c_int;
    fn pnm_fget_bytes(
        pnm_ptr: *const pnm_struct,
        sample_bytes: *mut c_uchar,
        sample_size: size_t,
        num_rows: c_uint,
        stream: *mut FILE,
    ) -> c_int;
}

pub type png_struct = png_struct_def;
pub type png_structp = *mut png_struct;
pub type png_info = png_info_def;
pub type png_infop = *mut png_info;
pub type png_const_structrp = *const png_struct;
pub type png_inforp = *mut png_info;

pub type C2RustUnnamed = c_uint;
pub const PNM_P7: C2RustUnnamed = 7;
pub const PNM_P6: C2RustUnnamed = 6;
pub const PNM_P5: C2RustUnnamed = 5;
pub const PNM_P4: C2RustUnnamed = 4;
pub const PNM_P3: C2RustUnnamed = 3;
pub const PNM_P2: C2RustUnnamed = 2;
pub const PNM_P1: C2RustUnnamed = 1;

pub const PNG_COLOR_TYPE_GRAY: c_int = 0 as c_int;
pub const PNG_COLOR_TYPE_RGB: c_int = 2 as c_int;

pub const PNGX_PNM_LENGTH_MAX: c_uint = 0x7fffffff as c_uint;
static mut pbm_fmt_name: [c_char; 4] =
    unsafe { ::core::mem::transmute::<[u8; 4], [c_char; 4]>(*b"PBM\0") };
static mut pgm_fmt_name: [c_char; 4] =
    unsafe { ::core::mem::transmute::<[u8; 4], [c_char; 4]>(*b"PGM\0") };
static mut ppm_fmt_name: [c_char; 4] =
    unsafe { ::core::mem::transmute::<[u8; 4], [c_char; 4]>(*b"PPM\0") };
static mut pam_fmt_name: [c_char; 4] =
    unsafe { ::core::mem::transmute::<[u8; 4], [c_char; 4]>(*b"PAM\0") };
static mut pbm_fmt_long_name: [c_char; 16] =
    unsafe { ::core::mem::transmute::<[u8; 16], [c_char; 16]>(*b"Portable Bitmap\0") };
static mut pgm_fmt_long_name: [c_char; 17] = unsafe {
    ::core::mem::transmute::<[u8; 17], [c_char; 17]>(*b"Portable Graymap\0")
};
static mut ppm_fmt_long_name: [c_char; 16] =
    unsafe { ::core::mem::transmute::<[u8; 16], [c_char; 16]>(*b"Portable Pixmap\0") };
static mut pam_fmt_long_name: [c_char; 16] =
    unsafe { ::core::mem::transmute::<[u8; 16], [c_char; 16]>(*b"Portable Anymap\0") };
#[no_mangle]
pub unsafe extern "C" fn pngx_sig_is_pnm(
    mut sig: png_bytep,
    mut sig_size: size_t,
    mut fmt_name_ptr: png_const_charpp,
    mut fmt_long_name_ptr: png_const_charpp,
) -> c_int {
    static mut fmt_names: [*const c_char; 7] = unsafe {
        [
            &raw const pbm_fmt_name as *const c_char,
            &raw const pgm_fmt_name as *const c_char,
            &raw const ppm_fmt_name as *const c_char,
            &raw const pbm_fmt_name as *const c_char,
            &raw const pgm_fmt_name as *const c_char,
            &raw const ppm_fmt_name as *const c_char,
            &raw const pam_fmt_name as *const c_char,
        ]
    };
    static mut fmt_long_names: [*const c_char; 7] = unsafe {
        [
            &raw const pbm_fmt_long_name as *const c_char,
            &raw const pgm_fmt_long_name as *const c_char,
            &raw const ppm_fmt_long_name as *const c_char,
            &raw const pbm_fmt_long_name as *const c_char,
            &raw const pgm_fmt_long_name as *const c_char,
            &raw const ppm_fmt_long_name as *const c_char,
            &raw const pam_fmt_long_name as *const c_char,
        ]
    };
    if sig_size < 4 as size_t {
        return -(1 as c_int);
    }
    if *sig.offset(0 as c_int as isize) as c_int != 'P' as i32
        || (*sig.offset(1 as c_int as isize) as c_int) < '1' as i32
        || *sig.offset(1 as c_int as isize) as c_int > '7' as i32
    {
        return 0 as c_int;
    }
    if *sig.offset(2 as c_int as isize) as c_int != ' ' as i32
        && *sig.offset(2 as c_int as isize) as c_int != '\t' as i32
        && *sig.offset(2 as c_int as isize) as c_int != '\n' as i32
        && *sig.offset(2 as c_int as isize) as c_int != '\r' as i32
        && *sig.offset(2 as c_int as isize) as c_int != '#' as i32
    {
        return 0 as c_int;
    }
    if !fmt_name_ptr.is_null() {
        *fmt_name_ptr = fmt_names[(*sig.offset(1 as c_int as isize)
            as c_int
            - '1' as i32) as usize];
    }
    if !fmt_long_name_ptr.is_null() {
        *fmt_long_name_ptr = fmt_long_names[(*sig.offset(1 as c_int as isize)
            as c_int
            - '1' as i32) as usize];
    }
    return 1 as c_int;
}
unsafe extern "C" fn pnm_fpeek_eof(
    mut pnm_ptr: *mut pnm_struct,
    mut stream: *mut FILE,
) -> c_int {
    let mut ch: c_int = 0;
    if (*pnm_ptr).format >= PNM_P1 as c_int as c_uint
        && (*pnm_ptr).format <= PNM_P3 as c_int as c_uint
    {
        loop {
            ch = getc(stream);
            if ch == '#' as i32 {
                loop {
                    ch = getc(stream);
                    if !(ch != EOF && ch != '\n' as i32 && ch != '\r' as i32) {
                        break;
                    }
                }
            }
            if ch == EOF {
                return 1 as c_int;
            }
            if !(ch == ' ' as i32 || ch == '\t' as i32 || ch == '\n' as i32 || ch == '\r' as i32) {
                break;
            }
        }
    } else {
        ch = getc(stream);
        if ch == EOF {
            return 1 as c_int;
        }
    }
    ungetc(ch, stream);
    return 0 as c_int;
}
#[no_mangle]
pub unsafe extern "C" fn pngx_read_pnm(
    mut png_ptr: png_structp,
    mut info_ptr: png_infop,
    mut stream: *mut FILE,
) -> c_int {
    let mut pnminfo: pnm_struct = pnm_struct {
        format: 0,
        depth: 0,
        width: 0,
        height: 0,
        maxval: 0,
    };
    let mut format: c_uint = 0;
    let mut depth: c_uint = 0;
    let mut width: c_uint = 0;
    let mut height: c_uint = 0;
    let mut maxval: c_uint = 0;
    let mut max_width: c_uint = 0;
    let mut num_samples: c_uint = 0;
    let mut sample_size: c_uint = 0;
    let mut pnmrow: *mut c_uint = ::core::ptr::null_mut::<c_uint>();
    let mut row_size: size_t = 0;
    let mut row_pointers: png_bytepp = ::core::ptr::null_mut::<*mut png_byte>();
    let mut sig_bit: png_color_8 = png_color_8 {
        red: 0,
        green: 0,
        blue: 0,
        gray: 0,
        alpha: 0,
    };
    let mut i: c_uint = 0;
    let mut j: c_uint = 0;
    let mut failed: c_int = 0;
    let mut overflow: c_int = 0;
    if pnm_fget_header(&raw mut pnminfo, stream) != 1 as c_int {
        return 0 as c_int;
    }
    format = pnminfo.format;
    depth = pnminfo.depth;
    width = pnminfo.width;
    height = pnminfo.height;
    maxval = pnminfo.maxval;
    if format > PNM_P6 as c_int as c_uint {
        png_error(
            png_ptr as png_const_structrp,
            b"Can't handle PNM formats newer than PPM (\"P6\")\0" as *const u8 as png_const_charp,
        );
    }
    max_width = (if ::core::mem::size_of::<size_t>() as usize
        <= ::core::mem::size_of::<c_uint>() as usize
    {
        (UINT_MAX as usize)
            .wrapping_div(::core::mem::size_of::<c_uint>() as usize)
            .wrapping_div(depth as usize)
    } else {
        UINT_MAX as usize
    }) as c_uint;
    if max_width > PNGX_PNM_LENGTH_MAX {
        max_width = PNGX_PNM_LENGTH_MAX;
    }
    if width > max_width {
        png_error(
            png_ptr as png_const_structrp,
            b"Can't handle exceedingly large PNM dimensions\0" as *const u8 as png_const_charp,
        );
    }
    sample_size = 1 as c_uint;
    num_samples = depth.wrapping_mul(width);
    row_size = num_samples as size_t;
    if maxval > 65535 as c_uint {
        png_error(
            png_ptr as png_const_structrp,
            b"Can't handle PNM samples larger than 16 bits\0" as *const u8 as png_const_charp,
        );
    } else if maxval > 255 as c_uint {
        sample_size = 2 as c_uint;
        row_size = (row_size as c_ulong).wrapping_mul(2 as c_ulong)
            as size_t as size_t;
    }
    png_set_IHDR(
        png_ptr as png_const_structrp,
        info_ptr as png_inforp,
        width as png_uint_32,
        height as png_uint_32,
        if maxval <= 255 as c_uint {
            8 as c_int
        } else {
            16 as c_int
        },
        if depth == 1 as c_uint {
            PNG_COLOR_TYPE_GRAY
        } else {
            PNG_COLOR_TYPE_RGB
        },
        PNG_INTERLACE_NONE,
        PNG_COMPRESSION_TYPE_BASE,
        PNG_FILTER_TYPE_BASE,
    );
    i = 1 as c_uint;
    j = 2 as c_uint;
    while j.wrapping_sub(1 as c_uint) < maxval {
        i = i.wrapping_add(1);
        j <<= 1 as c_int;
    }
    if j.wrapping_sub(1 as c_uint) != maxval {
        png_warning(
            png_ptr as png_const_structrp,
            b"Possibly inexact sample conversion from PNM to PNG\0" as *const u8 as png_const_charp,
        );
    } else if i.wrapping_rem(8 as c_uint) != 0 as c_uint
        && (depth > 1 as c_uint
            || (8 as c_uint).wrapping_rem(i) != 0 as c_uint)
    {
        sig_bit.gray = i as png_byte;
        sig_bit.blue = sig_bit.gray;
        sig_bit.green = sig_bit.blue;
        sig_bit.red = sig_bit.green;
        sig_bit.alpha = 0 as png_byte;
        png_set_sBIT(
            png_ptr as png_const_structrp,
            info_ptr as png_inforp,
            &raw mut sig_bit as png_const_color_8p,
        );
    }
    row_pointers = pngx_malloc_rows(png_ptr, info_ptr, -(1 as c_int));
    if format >= PNM_P4 as c_int as c_uint
        && (maxval == 255 as c_uint || maxval == 65535 as c_uint)
    {
        pnmrow = ::core::ptr::null_mut::<c_uint>();
    } else {
        pnmrow = png_malloc(
            png_ptr as png_const_structrp,
            (num_samples as png_alloc_size_t)
                .wrapping_mul(::core::mem::size_of::<c_uint>() as png_alloc_size_t),
        ) as *mut c_uint;
    }
    failed = 0 as c_int;
    overflow = 0 as c_int;
    if !pnmrow.is_null() {
        i = 0 as c_uint;
        while i < height {
            if pnm_fget_values(&raw mut pnminfo, pnmrow, 1 as c_uint, stream)
                <= 0 as c_int
            {
                failed = 1 as c_int;
            }
            if maxval <= 255 as c_uint {
                j = 0 as c_uint;
                while j < num_samples {
                    let mut val: c_uint = *pnmrow.offset(j as isize);
                    if val > maxval {
                        val = 255 as c_uint;
                        overflow = 1 as c_int;
                    } else if maxval != 255 as c_uint {
                        val = val
                            .wrapping_mul(255 as c_uint)
                            .wrapping_add(maxval.wrapping_div(2 as c_uint))
                            .wrapping_div(maxval);
                    }
                    *(*row_pointers.offset(i as isize)).offset(j as isize) = val as png_byte;
                    j = j.wrapping_add(1);
                }
            } else {
                j = 0 as c_uint;
                while j < num_samples {
                    let mut val_0: png_uint_32 = *pnmrow.offset(j as isize) as png_uint_32;
                    if val_0 > maxval {
                        val_0 = 65535 as png_uint_32;
                        overflow = 1 as c_int;
                    } else if maxval != 65535 as c_uint {
                        val_0 = (val_0 as c_uint)
                            .wrapping_mul(65535 as c_uint)
                            .wrapping_add(maxval.wrapping_div(2 as c_uint))
                            .wrapping_div(maxval) as png_uint_32;
                    }
                    *(*row_pointers.offset(i as isize))
                        .offset((2 as c_uint).wrapping_mul(j) as isize) =
                        (val_0 >> 8 as c_int) as png_byte;
                    *(*row_pointers.offset(i as isize)).offset(
                        (2 as c_uint)
                            .wrapping_mul(j)
                            .wrapping_add(1 as c_uint)
                            as isize,
                    ) = (val_0 as c_uint & 0xff as c_uint) as png_byte;
                    j = j.wrapping_add(1);
                }
            }
            if failed != 0 {
                break;
            }
            i = i.wrapping_add(1);
        }
    } else {
        i = 0 as c_uint;
        while i < height {
            if pnm_fget_bytes(
                &raw mut pnminfo,
                *row_pointers.offset(i as isize) as *mut c_uchar,
                sample_size as size_t,
                1 as c_uint,
                stream,
            ) <= 0 as c_int
            {
                failed = 1 as c_int;
                break;
            } else {
                i = i.wrapping_add(1);
            }
        }
    }
    while i < height {
        memset(
            *row_pointers.offset(i as isize) as *mut c_void,
            0 as c_int,
            row_size,
        );
        i = i.wrapping_add(1);
    }
    if !pnmrow.is_null() {
        png_free(png_ptr as png_const_structrp, pnmrow as png_voidp);
    }
    if overflow != 0 {
        png_warning(
            png_ptr as png_const_structrp,
            b"Overflow in PNM samples\0" as *const u8 as png_const_charp,
        );
    }
    if failed != 0 {
        png_error(
            png_ptr as png_const_structrp,
            b"Error in PNM image file\0" as *const u8 as png_const_charp,
        );
    } else if pnm_fpeek_eof(&raw mut pnminfo, stream) == 0 {
        png_warning(
            png_ptr as png_const_structrp,
            b"Extraneous data found after PNM image\0" as *const u8 as png_const_charp,
        );
    }
    return 1 as c_int;
}

pub const UINT_MAX: c_uint = (__INT_MAX__ as c_uint)
    .wrapping_mul(2 as c_uint)
    .wrapping_add(1 as c_uint);
