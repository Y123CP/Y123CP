extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type png_struct_def;
    pub type png_info_def;
    fn getc(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn ungetc(__c: ::core::ffi::c_int, __stream: *mut FILE) -> ::core::ffi::c_int;
    fn png_malloc(png_ptr: png_const_structrp, size: png_alloc_size_t) -> png_voidp;
    fn png_free(png_ptr: png_const_structrp, ptr: png_voidp);
    fn png_error(png_ptr: png_const_structrp, error_message: png_const_charp) -> !;
    fn png_warning(png_ptr: png_const_structrp, warning_message: png_const_charp);
    fn png_set_IHDR(
        png_ptr: png_const_structrp,
        info_ptr: png_inforp,
        width: png_uint_32,
        height: png_uint_32,
        bit_depth: ::core::ffi::c_int,
        color_type: ::core::ffi::c_int,
        interlace_method: ::core::ffi::c_int,
        compression_method: ::core::ffi::c_int,
        filter_method: ::core::ffi::c_int,
    );
    fn png_set_sBIT(png_ptr: png_const_structrp, info_ptr: png_inforp, sig_bit: png_const_color_8p);
    fn pngx_malloc_rows(
        png_ptr: png_structp,
        info_ptr: png_infop,
        filler: ::core::ffi::c_int,
    ) -> png_bytepp;
    fn pnm_fget_header(pnm_ptr: *mut pnm_struct, stream: *mut FILE) -> ::core::ffi::c_int;
    fn pnm_fget_values(
        pnm_ptr: *const pnm_struct,
        sample_values: *mut ::core::ffi::c_uint,
        num_rows: ::core::ffi::c_uint,
        stream: *mut FILE,
    ) -> ::core::ffi::c_int;
    fn pnm_fget_bytes(
        pnm_ptr: *const pnm_struct,
        sample_bytes: *mut ::core::ffi::c_uchar,
        sample_size: size_t,
        num_rows: ::core::ffi::c_uint,
        stream: *mut FILE,
    ) -> ::core::ffi::c_int;
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
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
pub type png_uint_32 = ::core::ffi::c_uint;
pub type png_size_t = size_t;
pub type png_alloc_size_t = png_size_t;
pub type png_voidp = *mut ::core::ffi::c_void;
pub type png_bytep = *mut png_byte;
pub type png_const_charp = *const ::core::ffi::c_char;
pub type png_bytepp = *mut *mut png_byte;
pub type png_const_charpp = *mut *const ::core::ffi::c_char;
pub type png_struct = png_struct_def;
pub type png_structp = *mut png_struct;
pub type png_info = png_info_def;
pub type png_infop = *mut png_info;
pub type png_const_structrp = *const png_struct;
pub type png_inforp = *mut png_info;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct png_color_8_struct {
    pub red: png_byte,
    pub green: png_byte,
    pub blue: png_byte,
    pub gray: png_byte,
    pub alpha: png_byte,
}
pub type png_color_8 = png_color_8_struct;
pub type png_const_color_8p = *const png_color_8;
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const PNM_P7: C2RustUnnamed = 7;
pub const PNM_P6: C2RustUnnamed = 6;
pub const PNM_P5: C2RustUnnamed = 5;
pub const PNM_P4: C2RustUnnamed = 4;
pub const PNM_P3: C2RustUnnamed = 3;
pub const PNM_P2: C2RustUnnamed = 2;
pub const PNM_P1: C2RustUnnamed = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pnm_struct {
    pub format: ::core::ffi::c_uint,
    pub depth: ::core::ffi::c_uint,
    pub width: ::core::ffi::c_uint,
    pub height: ::core::ffi::c_uint,
    pub maxval: ::core::ffi::c_uint,
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const EOF: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
pub const PNG_COLOR_MASK_COLOR: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const PNG_COLOR_TYPE_GRAY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const PNG_COLOR_TYPE_RGB: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const PNG_COMPRESSION_TYPE_BASE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const PNG_FILTER_TYPE_BASE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const PNG_INTERLACE_NONE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const PNGX_PNM_LENGTH_MAX: ::core::ffi::c_uint = 0x7fffffff as ::core::ffi::c_uint;
static mut pbm_fmt_name: [::core::ffi::c_char; 4] =
    unsafe { ::core::mem::transmute::<[u8; 4], [::core::ffi::c_char; 4]>(*b"PBM\0") };
static mut pgm_fmt_name: [::core::ffi::c_char; 4] =
    unsafe { ::core::mem::transmute::<[u8; 4], [::core::ffi::c_char; 4]>(*b"PGM\0") };
static mut ppm_fmt_name: [::core::ffi::c_char; 4] =
    unsafe { ::core::mem::transmute::<[u8; 4], [::core::ffi::c_char; 4]>(*b"PPM\0") };
static mut pam_fmt_name: [::core::ffi::c_char; 4] =
    unsafe { ::core::mem::transmute::<[u8; 4], [::core::ffi::c_char; 4]>(*b"PAM\0") };
static mut pbm_fmt_long_name: [::core::ffi::c_char; 16] =
    unsafe { ::core::mem::transmute::<[u8; 16], [::core::ffi::c_char; 16]>(*b"Portable Bitmap\0") };
static mut pgm_fmt_long_name: [::core::ffi::c_char; 17] = unsafe {
    ::core::mem::transmute::<[u8; 17], [::core::ffi::c_char; 17]>(*b"Portable Graymap\0")
};
static mut ppm_fmt_long_name: [::core::ffi::c_char; 16] =
    unsafe { ::core::mem::transmute::<[u8; 16], [::core::ffi::c_char; 16]>(*b"Portable Pixmap\0") };
static mut pam_fmt_long_name: [::core::ffi::c_char; 16] =
    unsafe { ::core::mem::transmute::<[u8; 16], [::core::ffi::c_char; 16]>(*b"Portable Anymap\0") };
#[no_mangle]
pub unsafe extern "C" fn pngx_sig_is_pnm(
    mut sig: png_bytep,
    mut sig_size: size_t,
    mut fmt_name_ptr: png_const_charpp,
    mut fmt_long_name_ptr: png_const_charpp,
) -> ::core::ffi::c_int {
    static mut fmt_names: [*const ::core::ffi::c_char; 7] = unsafe {
        [
            &raw const pbm_fmt_name as *const ::core::ffi::c_char,
            &raw const pgm_fmt_name as *const ::core::ffi::c_char,
            &raw const ppm_fmt_name as *const ::core::ffi::c_char,
            &raw const pbm_fmt_name as *const ::core::ffi::c_char,
            &raw const pgm_fmt_name as *const ::core::ffi::c_char,
            &raw const ppm_fmt_name as *const ::core::ffi::c_char,
            &raw const pam_fmt_name as *const ::core::ffi::c_char,
        ]
    };
    static mut fmt_long_names: [*const ::core::ffi::c_char; 7] = unsafe {
        [
            &raw const pbm_fmt_long_name as *const ::core::ffi::c_char,
            &raw const pgm_fmt_long_name as *const ::core::ffi::c_char,
            &raw const ppm_fmt_long_name as *const ::core::ffi::c_char,
            &raw const pbm_fmt_long_name as *const ::core::ffi::c_char,
            &raw const pgm_fmt_long_name as *const ::core::ffi::c_char,
            &raw const ppm_fmt_long_name as *const ::core::ffi::c_char,
            &raw const pam_fmt_long_name as *const ::core::ffi::c_char,
        ]
    };
    if sig_size < 4 as size_t {
        return -(1 as ::core::ffi::c_int);
    }
    if *sig.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != 'P' as i32
        || (*sig.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int) < '1' as i32
        || *sig.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int > '7' as i32
    {
        return 0 as ::core::ffi::c_int;
    }
    if *sig.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != ' ' as i32
        && *sig.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != '\t' as i32
        && *sig.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != '\n' as i32
        && *sig.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != '\r' as i32
        && *sig.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != '#' as i32
    {
        return 0 as ::core::ffi::c_int;
    }
    if !fmt_name_ptr.is_null() {
        *fmt_name_ptr = fmt_names[(*sig.offset(1 as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            - '1' as i32) as usize];
    }
    if !fmt_long_name_ptr.is_null() {
        *fmt_long_name_ptr = fmt_long_names[(*sig.offset(1 as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            - '1' as i32) as usize];
    }
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn pnm_fpeek_eof(
    mut pnm_ptr: *mut pnm_struct,
    mut stream: *mut FILE,
) -> ::core::ffi::c_int {
    let mut ch: ::core::ffi::c_int = 0;
    if (*pnm_ptr).format >= PNM_P1 as ::core::ffi::c_int as ::core::ffi::c_uint
        && (*pnm_ptr).format <= PNM_P3 as ::core::ffi::c_int as ::core::ffi::c_uint
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
                return 1 as ::core::ffi::c_int;
            }
            if !(ch == ' ' as i32 || ch == '\t' as i32 || ch == '\n' as i32 || ch == '\r' as i32) {
                break;
            }
        }
    } else {
        ch = getc(stream);
        if ch == EOF {
            return 1 as ::core::ffi::c_int;
        }
    }
    ungetc(ch, stream);
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn pngx_read_pnm(
    mut png_ptr: png_structp,
    mut info_ptr: png_infop,
    mut stream: *mut FILE,
) -> ::core::ffi::c_int {
    let mut pnminfo: pnm_struct = pnm_struct {
        format: 0,
        depth: 0,
        width: 0,
        height: 0,
        maxval: 0,
    };
    let mut format: ::core::ffi::c_uint = 0;
    let mut depth: ::core::ffi::c_uint = 0;
    let mut width: ::core::ffi::c_uint = 0;
    let mut height: ::core::ffi::c_uint = 0;
    let mut maxval: ::core::ffi::c_uint = 0;
    let mut max_width: ::core::ffi::c_uint = 0;
    let mut num_samples: ::core::ffi::c_uint = 0;
    let mut sample_size: ::core::ffi::c_uint = 0;
    let mut pnmrow: *mut ::core::ffi::c_uint = ::core::ptr::null_mut::<::core::ffi::c_uint>();
    let mut row_size: size_t = 0;
    let mut row_pointers: png_bytepp = ::core::ptr::null_mut::<*mut png_byte>();
    let mut sig_bit: png_color_8 = png_color_8 {
        red: 0,
        green: 0,
        blue: 0,
        gray: 0,
        alpha: 0,
    };
    let mut i: ::core::ffi::c_uint = 0;
    let mut j: ::core::ffi::c_uint = 0;
    let mut failed: ::core::ffi::c_int = 0;
    let mut overflow: ::core::ffi::c_int = 0;
    if pnm_fget_header(&raw mut pnminfo, stream) != 1 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    format = pnminfo.format;
    depth = pnminfo.depth;
    width = pnminfo.width;
    height = pnminfo.height;
    maxval = pnminfo.maxval;
    if format > PNM_P6 as ::core::ffi::c_int as ::core::ffi::c_uint {
        png_error(
            png_ptr as png_const_structrp,
            b"Can't handle PNM formats newer than PPM (\"P6\")\0" as *const u8 as png_const_charp,
        );
    }
    max_width = (if ::core::mem::size_of::<size_t>() as usize
        <= ::core::mem::size_of::<::core::ffi::c_uint>() as usize
    {
        (UINT_MAX as usize)
            .wrapping_div(::core::mem::size_of::<::core::ffi::c_uint>() as usize)
            .wrapping_div(depth as usize)
    } else {
        UINT_MAX as usize
    }) as ::core::ffi::c_uint;
    if max_width > PNGX_PNM_LENGTH_MAX {
        max_width = PNGX_PNM_LENGTH_MAX;
    }
    if width > max_width {
        png_error(
            png_ptr as png_const_structrp,
            b"Can't handle exceedingly large PNM dimensions\0" as *const u8 as png_const_charp,
        );
    }
    sample_size = 1 as ::core::ffi::c_uint;
    num_samples = depth.wrapping_mul(width);
    row_size = num_samples as size_t;
    if maxval > 65535 as ::core::ffi::c_uint {
        png_error(
            png_ptr as png_const_structrp,
            b"Can't handle PNM samples larger than 16 bits\0" as *const u8 as png_const_charp,
        );
    } else if maxval > 255 as ::core::ffi::c_uint {
        sample_size = 2 as ::core::ffi::c_uint;
        row_size = (row_size as ::core::ffi::c_ulong).wrapping_mul(2 as ::core::ffi::c_ulong)
            as size_t as size_t;
    }
    png_set_IHDR(
        png_ptr as png_const_structrp,
        info_ptr as png_inforp,
        width as png_uint_32,
        height as png_uint_32,
        if maxval <= 255 as ::core::ffi::c_uint {
            8 as ::core::ffi::c_int
        } else {
            16 as ::core::ffi::c_int
        },
        if depth == 1 as ::core::ffi::c_uint {
            PNG_COLOR_TYPE_GRAY
        } else {
            PNG_COLOR_TYPE_RGB
        },
        PNG_INTERLACE_NONE,
        PNG_COMPRESSION_TYPE_BASE,
        PNG_FILTER_TYPE_BASE,
    );
    i = 1 as ::core::ffi::c_uint;
    j = 2 as ::core::ffi::c_uint;
    while j.wrapping_sub(1 as ::core::ffi::c_uint) < maxval {
        i = i.wrapping_add(1);
        j <<= 1 as ::core::ffi::c_int;
    }
    if j.wrapping_sub(1 as ::core::ffi::c_uint) != maxval {
        png_warning(
            png_ptr as png_const_structrp,
            b"Possibly inexact sample conversion from PNM to PNG\0" as *const u8 as png_const_charp,
        );
    } else if i.wrapping_rem(8 as ::core::ffi::c_uint) != 0 as ::core::ffi::c_uint
        && (depth > 1 as ::core::ffi::c_uint
            || (8 as ::core::ffi::c_uint).wrapping_rem(i) != 0 as ::core::ffi::c_uint)
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
    row_pointers = pngx_malloc_rows(png_ptr, info_ptr, -(1 as ::core::ffi::c_int));
    if format >= PNM_P4 as ::core::ffi::c_int as ::core::ffi::c_uint
        && (maxval == 255 as ::core::ffi::c_uint || maxval == 65535 as ::core::ffi::c_uint)
    {
        pnmrow = ::core::ptr::null_mut::<::core::ffi::c_uint>();
    } else {
        pnmrow = png_malloc(
            png_ptr as png_const_structrp,
            (num_samples as png_alloc_size_t)
                .wrapping_mul(::core::mem::size_of::<::core::ffi::c_uint>() as png_alloc_size_t),
        ) as *mut ::core::ffi::c_uint;
    }
    failed = 0 as ::core::ffi::c_int;
    overflow = 0 as ::core::ffi::c_int;
    if !pnmrow.is_null() {
        i = 0 as ::core::ffi::c_uint;
        while i < height {
            if pnm_fget_values(&raw mut pnminfo, pnmrow, 1 as ::core::ffi::c_uint, stream)
                <= 0 as ::core::ffi::c_int
            {
                failed = 1 as ::core::ffi::c_int;
            }
            if maxval <= 255 as ::core::ffi::c_uint {
                j = 0 as ::core::ffi::c_uint;
                while j < num_samples {
                    let mut val: ::core::ffi::c_uint = *pnmrow.offset(j as isize);
                    if val > maxval {
                        val = 255 as ::core::ffi::c_uint;
                        overflow = 1 as ::core::ffi::c_int;
                    } else if maxval != 255 as ::core::ffi::c_uint {
                        val = val
                            .wrapping_mul(255 as ::core::ffi::c_uint)
                            .wrapping_add(maxval.wrapping_div(2 as ::core::ffi::c_uint))
                            .wrapping_div(maxval);
                    }
                    *(*row_pointers.offset(i as isize)).offset(j as isize) = val as png_byte;
                    j = j.wrapping_add(1);
                }
            } else {
                j = 0 as ::core::ffi::c_uint;
                while j < num_samples {
                    let mut val_0: png_uint_32 = *pnmrow.offset(j as isize) as png_uint_32;
                    if val_0 > maxval {
                        val_0 = 65535 as png_uint_32;
                        overflow = 1 as ::core::ffi::c_int;
                    } else if maxval != 65535 as ::core::ffi::c_uint {
                        val_0 = (val_0 as ::core::ffi::c_uint)
                            .wrapping_mul(65535 as ::core::ffi::c_uint)
                            .wrapping_add(maxval.wrapping_div(2 as ::core::ffi::c_uint))
                            .wrapping_div(maxval) as png_uint_32;
                    }
                    *(*row_pointers.offset(i as isize))
                        .offset((2 as ::core::ffi::c_uint).wrapping_mul(j) as isize) =
                        (val_0 >> 8 as ::core::ffi::c_int) as png_byte;
                    *(*row_pointers.offset(i as isize)).offset(
                        (2 as ::core::ffi::c_uint)
                            .wrapping_mul(j)
                            .wrapping_add(1 as ::core::ffi::c_uint)
                            as isize,
                    ) = (val_0 as ::core::ffi::c_uint & 0xff as ::core::ffi::c_uint) as png_byte;
                    j = j.wrapping_add(1);
                }
            }
            if failed != 0 {
                break;
            }
            i = i.wrapping_add(1);
        }
    } else {
        i = 0 as ::core::ffi::c_uint;
        while i < height {
            if pnm_fget_bytes(
                &raw mut pnminfo,
                *row_pointers.offset(i as isize) as *mut ::core::ffi::c_uchar,
                sample_size as size_t,
                1 as ::core::ffi::c_uint,
                stream,
            ) <= 0 as ::core::ffi::c_int
            {
                failed = 1 as ::core::ffi::c_int;
                break;
            } else {
                i = i.wrapping_add(1);
            }
        }
    }
    while i < height {
        memset(
            *row_pointers.offset(i as isize) as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
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
    return 1 as ::core::ffi::c_int;
}
pub const __INT_MAX__: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
pub const UINT_MAX: ::core::ffi::c_uint = (__INT_MAX__ as ::core::ffi::c_uint)
    .wrapping_mul(2 as ::core::ffi::c_uint)
    .wrapping_add(1 as ::core::ffi::c_uint);
