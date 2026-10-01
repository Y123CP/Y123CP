extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type png_struct_def;
    pub type png_info_def;
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
    fn pngx_malloc_rows(
        png_ptr: png_structp,
        info_ptr: png_infop,
        filler: ::core::ffi::c_int,
    ) -> png_bytepp;
    fn minitiff_init_info(info_ptr: *mut minitiff_info);
    fn minitiff_validate_info(info_ptr: *const minitiff_info);
    fn minitiff_destroy_info(info_ptr: *mut minitiff_info);
    fn minitiff_read_info(info_ptr: *mut minitiff_info, stream: *mut FILE);
    fn minitiff_read_row(
        info_ptr: *mut minitiff_info,
        row_ptr: *mut ::core::ffi::c_uchar,
        row_index: size_t,
        stream: *mut FILE,
    );
    static minitiff_sig_m: [::core::ffi::c_char; 4];
    static minitiff_sig_i: [::core::ffi::c_char; 4];
    fn memcmp(
        __s1: *const ::core::ffi::c_void,
        __s2: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn strstr(
        __haystack: *const ::core::ffi::c_char,
        __needle: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
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
pub struct minitiff_info {
    pub error_handler: Option<unsafe extern "C" fn(*const ::core::ffi::c_char) -> ()>,
    pub warning_handler: Option<unsafe extern "C" fn(*const ::core::ffi::c_char) -> ()>,
    pub byte_order: ::core::ffi::c_int,
    pub width: size_t,
    pub height: size_t,
    pub bits_per_sample: ::core::ffi::c_uint,
    pub compression: ::core::ffi::c_uint,
    pub photometric: ::core::ffi::c_uint,
    pub strip_offsets_count: size_t,
    pub strip_offsets: *mut ::core::ffi::c_ulong,
    pub orientation: ::core::ffi::c_uint,
    pub samples_per_pixel: ::core::ffi::c_uint,
    pub rows_per_strip: size_t,
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const PNG_COLOR_MASK_COLOR: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const PNG_COLOR_MASK_ALPHA: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const PNG_COLOR_TYPE_GRAY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const PNG_COLOR_TYPE_RGB: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const PNG_COLOR_TYPE_RGB_ALPHA: ::core::ffi::c_int =
    PNG_COLOR_MASK_COLOR | PNG_COLOR_MASK_ALPHA;
pub const PNG_COLOR_TYPE_GRAY_ALPHA: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const PNG_COMPRESSION_TYPE_BASE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const PNG_FILTER_TYPE_BASE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const PNG_INTERLACE_NONE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut tiff_fmt_name: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"TIFF\0") };
static mut tiff_fmt_long_name: [::core::ffi::c_char; 25] = unsafe {
    ::core::mem::transmute::<[u8; 25], [::core::ffi::c_char; 25]>(*b"Tagged Image File Format\0")
};
#[no_mangle]
pub unsafe extern "C" fn pngx_sig_is_tiff(
    mut sig: png_bytep,
    mut sig_size: size_t,
    mut fmt_name_ptr: png_const_charpp,
    mut fmt_long_name_ptr: png_const_charpp,
) -> ::core::ffi::c_int {
    if sig_size < 8 as size_t {
        return -(1 as ::core::ffi::c_int);
    }
    if memcmp(
        sig as *const ::core::ffi::c_void,
        &raw const minitiff_sig_m as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
        4 as size_t,
    ) != 0 as ::core::ffi::c_int
        && memcmp(
            sig as *const ::core::ffi::c_void,
            &raw const minitiff_sig_i as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
            4 as size_t,
        ) != 0 as ::core::ffi::c_int
    {
        return 0 as ::core::ffi::c_int;
    }
    if !fmt_name_ptr.is_null() {
        *fmt_name_ptr = &raw const tiff_fmt_name as *const ::core::ffi::c_char;
    }
    if !fmt_long_name_ptr.is_null() {
        *fmt_long_name_ptr = &raw const tiff_fmt_long_name as *const ::core::ffi::c_char;
    }
    return 1 as ::core::ffi::c_int;
}
static mut err_png_ptr: png_structp = ::core::ptr::null::<png_struct>() as *mut png_struct;
static mut num_extra_images: ::core::ffi::c_uint = 0;
unsafe extern "C" fn pngx_tiff_error(mut msg: *const ::core::ffi::c_char) {
    png_error(err_png_ptr as png_const_structrp, msg as png_const_charp);
}
unsafe extern "C" fn pngx_tiff_warning(mut msg: *const ::core::ffi::c_char) {
    if !strstr(
        msg,
        b"multi-image\0" as *const u8 as *const ::core::ffi::c_char,
    )
    .is_null()
    {
        num_extra_images = num_extra_images.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn pngx_read_tiff(
    mut png_ptr: png_structp,
    mut info_ptr: png_infop,
    mut stream: *mut FILE,
) -> ::core::ffi::c_int {
    let mut tiff_info: minitiff_info = minitiff_info {
        error_handler: None,
        warning_handler: None,
        byte_order: 0,
        width: 0,
        height: 0,
        bits_per_sample: 0,
        compression: 0,
        photometric: 0,
        strip_offsets_count: 0,
        strip_offsets: ::core::ptr::null_mut::<::core::ffi::c_ulong>(),
        orientation: 0,
        samples_per_pixel: 0,
        rows_per_strip: 0,
    };
    let mut width: ::core::ffi::c_uint = 0;
    let mut height: ::core::ffi::c_uint = 0;
    let mut pixel_size: ::core::ffi::c_uint = 0;
    let mut sample_depth: ::core::ffi::c_uint = 0;
    let mut sample_max: ::core::ffi::c_uint = 0;
    let mut color_type: ::core::ffi::c_int = 0;
    let mut sample_overflow: ::core::ffi::c_int = 0;
    let mut row_pointers: png_bytepp = ::core::ptr::null_mut::<*mut png_byte>();
    let mut row: png_bytep = ::core::ptr::null_mut::<png_byte>();
    let mut i: ::core::ffi::c_uint = 0;
    let mut j: ::core::ffi::c_uint = 0;
    let mut k: ::core::ffi::c_uint = 0;
    err_png_ptr = png_ptr;
    num_extra_images = 0 as ::core::ffi::c_uint;
    minitiff_init_info(&raw mut tiff_info);
    tiff_info.error_handler =
        Some(pngx_tiff_error as unsafe extern "C" fn(*const ::core::ffi::c_char) -> ())
            as Option<unsafe extern "C" fn(*const ::core::ffi::c_char) -> ()>;
    tiff_info.warning_handler =
        Some(pngx_tiff_warning as unsafe extern "C" fn(*const ::core::ffi::c_char) -> ())
            as Option<unsafe extern "C" fn(*const ::core::ffi::c_char) -> ()>;
    minitiff_read_info(&raw mut tiff_info, stream);
    minitiff_validate_info(&raw mut tiff_info);
    width = tiff_info.width as ::core::ffi::c_uint;
    height = tiff_info.height as ::core::ffi::c_uint;
    pixel_size = tiff_info.samples_per_pixel;
    sample_depth = tiff_info.bits_per_sample;
    match pixel_size {
        1 => {
            color_type = PNG_COLOR_TYPE_GRAY;
        }
        2 => {
            color_type = PNG_COLOR_TYPE_GRAY_ALPHA;
        }
        3 => {
            color_type = PNG_COLOR_TYPE_RGB;
        }
        4 => {
            color_type = PNG_COLOR_TYPE_RGB_ALPHA;
        }
        _ => {
            png_error(
                png_ptr as png_const_structrp,
                b"Unsupported TIFF color space\0" as *const u8 as png_const_charp,
            );
            return 0 as ::core::ffi::c_int;
        }
    }
    if sample_depth > 16 as ::core::ffi::c_uint {
        png_error(
            png_ptr as png_const_structrp,
            b"Unsupported TIFF sample depth\0" as *const u8 as png_const_charp,
        );
    }
    sample_max = (((1 as ::core::ffi::c_int) << sample_depth) - 1 as ::core::ffi::c_int)
        as ::core::ffi::c_uint;
    sample_overflow = 0 as ::core::ffi::c_int;
    png_set_IHDR(
        png_ptr as png_const_structrp,
        info_ptr as png_inforp,
        width as png_uint_32,
        height as png_uint_32,
        if sample_depth <= 8 as ::core::ffi::c_uint {
            8 as ::core::ffi::c_int
        } else {
            16 as ::core::ffi::c_int
        },
        color_type,
        PNG_INTERLACE_NONE,
        PNG_COMPRESSION_TYPE_BASE,
        PNG_FILTER_TYPE_BASE,
    );
    row_pointers = pngx_malloc_rows(png_ptr, info_ptr, 0 as ::core::ffi::c_int);
    if sample_depth <= 8 as ::core::ffi::c_uint {
        i = 0 as ::core::ffi::c_uint;
        while i < height {
            row = *row_pointers.offset(i as isize) as png_bytep;
            minitiff_read_row(
                &raw mut tiff_info,
                row as *mut ::core::ffi::c_uchar,
                i as size_t,
                stream,
            );
            if sample_depth < 8 as ::core::ffi::c_uint {
                j = 0 as ::core::ffi::c_uint;
                while j < pixel_size.wrapping_mul(width) {
                    let mut b: ::core::ffi::c_uint = *row.offset(j as isize) as ::core::ffi::c_uint;
                    if b > sample_max {
                        b = sample_max;
                        sample_overflow = 1 as ::core::ffi::c_int;
                    }
                    *row.offset(j as isize) =
                        b.wrapping_mul(255 as ::core::ffi::c_uint)
                            .wrapping_add(sample_max.wrapping_div(2 as ::core::ffi::c_uint))
                            .wrapping_div(sample_max) as png_byte;
                    j = j.wrapping_add(1);
                }
            }
            if tiff_info.photometric == 0 as ::core::ffi::c_uint {
                j = 0 as ::core::ffi::c_uint;
                while j < pixel_size.wrapping_mul(width) {
                    *row.offset(j as isize) = (255 as ::core::ffi::c_int
                        - *row.offset(j as isize) as ::core::ffi::c_int)
                        as png_byte;
                    j = j.wrapping_add(1);
                }
            }
            i = i.wrapping_add(1);
        }
    } else {
        i = 0 as ::core::ffi::c_uint;
        while i < height {
            row = *row_pointers.offset(i as isize) as png_bytep;
            minitiff_read_row(
                &raw mut tiff_info,
                row as *mut ::core::ffi::c_uchar,
                i as size_t,
                stream,
            );
            if tiff_info.byte_order == 'I' as i32 {
                k = 0 as ::core::ffi::c_uint;
                j = k;
                while j < pixel_size.wrapping_mul(width) {
                    let mut b_0: png_byte = *row.offset(k as isize);
                    *row.offset(k as isize) =
                        *row.offset(k.wrapping_add(1 as ::core::ffi::c_uint) as isize);
                    *row.offset(k.wrapping_add(1 as ::core::ffi::c_uint) as isize) = b_0;
                    j = j.wrapping_add(1);
                    k = k.wrapping_add(2 as ::core::ffi::c_uint);
                }
            }
            if sample_depth < 16 as ::core::ffi::c_uint {
                k = 0 as ::core::ffi::c_uint;
                j = k;
                while k < pixel_size.wrapping_mul(width) {
                    let mut b_1: ::core::ffi::c_uint = (((*row.offset(k as isize)
                        as ::core::ffi::c_int)
                        << 8 as ::core::ffi::c_int)
                        + *row.offset(k.wrapping_add(1 as ::core::ffi::c_uint) as isize)
                            as ::core::ffi::c_int)
                        as ::core::ffi::c_uint;
                    if b_1 > sample_max {
                        b_1 = sample_max;
                        sample_overflow = 1 as ::core::ffi::c_int;
                    }
                    b_1 = b_1
                        .wrapping_mul(65535 as ::core::ffi::c_uint)
                        .wrapping_add(sample_max.wrapping_div(2 as ::core::ffi::c_uint))
                        .wrapping_div(sample_max);
                    *row.offset(k as isize) = (b_1 >> 8 as ::core::ffi::c_int) as png_byte;
                    *row.offset(k.wrapping_add(1 as ::core::ffi::c_uint) as isize) =
                        (b_1 & 255 as ::core::ffi::c_uint) as png_byte;
                    j = j.wrapping_add(1);
                    k = k.wrapping_add(2 as ::core::ffi::c_uint);
                }
            }
            i = i.wrapping_add(1);
        }
    }
    if sample_overflow != 0 {
        png_warning(
            png_ptr as png_const_structrp,
            b"Overflow in TIFF samples\0" as *const u8 as png_const_charp,
        );
    }
    minitiff_destroy_info(&raw mut tiff_info);
    return (1 as ::core::ffi::c_uint).wrapping_add(num_extra_images) as ::core::ffi::c_int;
}
