extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type png_struct_def;
    pub type png_info_def;
    fn getc(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn ungetc(__c: ::core::ffi::c_int, __stream: *mut FILE) -> ::core::ffi::c_int;
    fn fread(
        __ptr: *mut ::core::ffi::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> ::core::ffi::c_ulong;
    fn fseek(
        __stream: *mut FILE,
        __off: ::core::ffi::c_long,
        __whence: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn png_set_sig_bytes(png_ptr: png_structrp, num_bytes: ::core::ffi::c_int);
    fn png_set_read_fn(png_ptr: png_structrp, io_ptr: png_voidp, read_data_fn: png_rw_ptr);
    fn png_error(png_ptr: png_const_structrp, error_message: png_const_charp) -> !;
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
    fn png_set_PLTE(
        png_ptr: png_structrp,
        info_ptr: png_inforp,
        palette: png_const_colorp,
        num_palette: ::core::ffi::c_int,
    );
    fn png_set_sBIT(png_ptr: png_const_structrp, info_ptr: png_inforp, sig_bit: png_const_color_8p);
    fn png_read_png(
        png_ptr: png_structrp,
        info_ptr: png_inforp,
        transforms: ::core::ffi::c_int,
        params: png_voidp,
    );
    fn pngx_malloc_rows_extended(
        png_ptr: png_structp,
        info_ptr: png_infop,
        min_row_size: pngx_alloc_size_t,
        filler: ::core::ffi::c_int,
    ) -> png_bytepp;
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
pub type png_structrp = *mut png_struct;
pub type png_const_structrp = *const png_struct;
pub type png_inforp = *mut png_info;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct png_color_struct {
    pub red: png_byte,
    pub green: png_byte,
    pub blue: png_byte,
}
pub type png_color = png_color_struct;
pub type png_const_colorp = *const png_color;
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
pub type png_rw_ptr = Option<unsafe extern "C" fn(png_structp, png_bytep, png_size_t) -> ()>;
pub type pngx_alloc_size_t = png_alloc_size_t;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const EOF: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
pub const SEEK_CUR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const PNG_UINT_31_MAX: png_uint_32 = 0x7fffffff as ::core::ffi::c_long as png_uint_32;
pub const PNG_UINT_32_MAX: png_uint_32 = -(1 as ::core::ffi::c_int) as png_uint_32;
pub const PNG_COLOR_MASK_PALETTE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const PNG_COLOR_MASK_COLOR: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const PNG_COLOR_MASK_ALPHA: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const PNG_COLOR_TYPE_GRAY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const PNG_COLOR_TYPE_PALETTE: ::core::ffi::c_int =
    PNG_COLOR_MASK_COLOR | PNG_COLOR_MASK_PALETTE;
pub const PNG_COLOR_TYPE_RGB: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const PNG_COLOR_TYPE_RGB_ALPHA: ::core::ffi::c_int =
    PNG_COLOR_MASK_COLOR | PNG_COLOR_MASK_ALPHA;
pub const PNG_COLOR_TYPE_RGBA: ::core::ffi::c_int = PNG_COLOR_TYPE_RGB_ALPHA;
pub const PNG_COMPRESSION_TYPE_BASE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const PNG_FILTER_TYPE_BASE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const PNG_INTERLACE_NONE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const BMP_SIGNATURE: ::core::ffi::c_int = 0x4d42 as ::core::ffi::c_int;
pub const BFH_WTYPE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const BFH_DOFFBITS: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const BFH_DBIHSIZE: ::core::ffi::c_int = 14 as ::core::ffi::c_int;
pub const FILEHED_SIZE: ::core::ffi::c_int = 14 as ::core::ffi::c_int;
pub const BIHSIZE_SIZE: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const BIH_LWIDTH: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const BIH_LHEIGHT: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const BIH_WBITCOUNT: ::core::ffi::c_int = 14 as ::core::ffi::c_int;
pub const BIH_DCOMPRESSION: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const B4H_DREDMASK: ::core::ffi::c_int = 40 as ::core::ffi::c_int;
pub const B4H_DGREENMASK: ::core::ffi::c_int = 44 as ::core::ffi::c_int;
pub const B4H_DBLUEMASK: ::core::ffi::c_int = 48 as ::core::ffi::c_int;
pub const B4H_DALPHAMASK: ::core::ffi::c_int = 52 as ::core::ffi::c_int;
pub const INFOHED_SIZE: ::core::ffi::c_int = 40 as ::core::ffi::c_int;
pub const BMPV5HED_SIZE: ::core::ffi::c_int = 124 as ::core::ffi::c_int;
pub const BCH_WWIDTH: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const BCH_WHEIGHT: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const BCH_WBITCOUNT: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const COREHED_SIZE: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
pub const RGB_BLUE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const RGB_GREEN: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const RGB_RED: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const RGBTRIPLE_SIZE: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const RGBQUAD_SIZE: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const BI_RGB: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const BI_RLE8: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const BI_RLE4: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const BI_BITFIELDS: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const BI_JPEG: png_uint_32 = 4 as png_uint_32;
pub const BI_PNG: png_uint_32 = 5 as png_uint_32;
unsafe extern "C" fn bmp_get_word(mut ptr: png_bytep) -> ::core::ffi::c_uint {
    return (*ptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint).wrapping_add(
        (*ptr.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
            << 8 as ::core::ffi::c_int,
    );
}
unsafe extern "C" fn bmp_get_dword(mut ptr: png_bytep) -> png_uint_32 {
    return (*ptr.offset(0 as ::core::ffi::c_int as isize) as png_uint_32)
        .wrapping_add(
            (*ptr.offset(1 as ::core::ffi::c_int as isize) as png_uint_32)
                << 8 as ::core::ffi::c_int,
        )
        .wrapping_add(
            (*ptr.offset(2 as ::core::ffi::c_int as isize) as png_uint_32)
                << 16 as ::core::ffi::c_int,
        )
        .wrapping_add(
            (*ptr.offset(3 as ::core::ffi::c_int as isize) as png_uint_32)
                << 24 as ::core::ffi::c_int,
        );
}
unsafe extern "C" fn bmp_memset_bytes(
    mut ptr: png_bytep,
    mut offset: size_t,
    mut ch: ::core::ffi::c_int,
    mut len: size_t,
) {
    memset(
        ptr.offset(offset as isize) as *mut ::core::ffi::c_void,
        ch,
        len,
    );
}
unsafe extern "C" fn bmp_memset_halfbytes(
    mut ptr: png_bytep,
    mut offset: size_t,
    mut ch: ::core::ffi::c_int,
    mut len: size_t,
) {
    if len == 0 as size_t {
        return;
    }
    ptr = ptr.offset(offset.wrapping_div(2 as size_t) as isize);
    if offset & 1 as size_t != 0 {
        *ptr = (*ptr as ::core::ffi::c_int & 0xf0 as ::core::ffi::c_int
            | ch & 0xf as ::core::ffi::c_int) as png_byte;
        ch = (ch & 0xf0 as ::core::ffi::c_int) >> 4 as ::core::ffi::c_int
            | (ch & 0xf as ::core::ffi::c_int) << 4 as ::core::ffi::c_int;
        ptr = ptr.offset(1);
        len = len.wrapping_sub(1);
    }
    memset(
        ptr as *mut ::core::ffi::c_void,
        ch,
        len.wrapping_div(2 as size_t),
    );
    if len & 1 as size_t != 0 {
        *ptr.offset(len.wrapping_div(2 as size_t) as isize) =
            (ch & 0xf0 as ::core::ffi::c_int) as png_byte;
    }
}
unsafe extern "C" fn bmp_fread_bytes(
    mut ptr: png_bytep,
    mut offset: size_t,
    mut len: size_t,
    mut stream: *mut FILE,
) -> size_t {
    let mut result: size_t = 0;
    result = fread(
        ptr.offset(offset as isize) as *mut ::core::ffi::c_void,
        1 as size_t,
        len,
        stream,
    ) as size_t;
    if len & 1 as size_t != 0 {
        getc(stream);
    }
    return result;
}
unsafe extern "C" fn bmp_fread_halfbytes(
    mut ptr: png_bytep,
    mut offset: size_t,
    mut len: size_t,
    mut stream: *mut FILE,
) -> size_t {
    let mut result: size_t = 0;
    let mut ch: ::core::ffi::c_int = 0;
    if len == 0 as size_t {
        return 0 as size_t;
    }
    ptr = ptr.offset(offset.wrapping_div(2 as size_t) as isize);
    if offset & 1 as size_t != 0 {
        result = 0 as size_t;
        while result < len.wrapping_sub(1 as size_t) {
            ch = getc(stream);
            if ch == EOF {
                break;
            }
            *ptr = (*ptr as ::core::ffi::c_int & 0xf0 as ::core::ffi::c_int
                | (ch & 0xf0 as ::core::ffi::c_int) >> 4 as ::core::ffi::c_int)
                as png_byte;
            ptr = ptr.offset(1);
            *ptr = ((ch & 0xf as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as png_byte;
            result = (result as ::core::ffi::c_ulong).wrapping_add(2 as ::core::ffi::c_ulong)
                as size_t as size_t;
        }
    } else {
        result = fread(
            ptr as *mut ::core::ffi::c_void,
            1 as size_t,
            len.wrapping_add(1 as size_t).wrapping_div(2 as size_t),
            stream,
        )
        .wrapping_mul(2 as ::core::ffi::c_ulong) as size_t;
    }
    if len & 2 as size_t != 0 {
        getc(stream);
    }
    return if result <= len { result } else { len };
}
unsafe extern "C" fn bmp_process_mask(
    mut bmp_mask: png_uint_32,
    mut sig_bit: png_bytep,
    mut shift_bit: png_bytep,
) {
    *shift_bit = 0 as ::core::ffi::c_int as png_byte;
    *sig_bit = *shift_bit;
    if bmp_mask == 0 as ::core::ffi::c_uint {
        return;
    }
    while bmp_mask as ::core::ffi::c_uint & 1 as ::core::ffi::c_uint == 0 as ::core::ffi::c_uint {
        bmp_mask >>= 1 as ::core::ffi::c_int;
        *shift_bit = (*shift_bit).wrapping_add(1);
    }
    while bmp_mask != 0 as ::core::ffi::c_uint {
        if bmp_mask as ::core::ffi::c_uint & 1 as ::core::ffi::c_uint == 0 as ::core::ffi::c_uint
            || *sig_bit as ::core::ffi::c_int >= 8 as ::core::ffi::c_int
        {
            *sig_bit = 0 as ::core::ffi::c_int as png_byte;
            return;
        }
        bmp_mask >>= 1 as ::core::ffi::c_int;
        *sig_bit = (*sig_bit).wrapping_add(1);
    }
}
unsafe extern "C" fn bmp_read_rows(
    mut begin_row: png_bytepp,
    mut end_row: png_bytepp,
    mut row_size: size_t,
    mut compression: ::core::ffi::c_uint,
    mut stream: *mut FILE,
) -> size_t {
    let mut result: size_t = 0;
    let mut crt_row: png_bytepp = ::core::ptr::null_mut::<*mut png_byte>();
    let mut inc: ::core::ffi::c_int = 0;
    let mut crtn: size_t = 0;
    let mut dcrtn: size_t = 0;
    let mut endn: size_t = 0;
    let mut len: ::core::ffi::c_uint = 0;
    let mut b1: ::core::ffi::c_uint = 0;
    let mut b2: ::core::ffi::c_uint = 0;
    let mut ch: ::core::ffi::c_int = 0;
    let mut bmp_memset_fn: Option<
        unsafe extern "C" fn(png_bytep, size_t, ::core::ffi::c_int, size_t) -> (),
    > = None;
    let mut bmp_fread_fn: Option<
        unsafe extern "C" fn(png_bytep, size_t, size_t, *mut FILE) -> size_t,
    > = None;
    if row_size == 0 as size_t {
        return 0 as size_t;
    }
    inc = if begin_row <= end_row {
        1 as ::core::ffi::c_int
    } else {
        -(1 as ::core::ffi::c_int)
    };
    crtn = 0 as size_t;
    result = 0 as size_t;
    if compression == BI_RLE4 as ::core::ffi::c_uint {
        endn = row_size.wrapping_mul(2 as size_t);
        if endn <= row_size {
            return 0 as size_t;
        }
        bmp_memset_fn = Some(
            bmp_memset_halfbytes
                as unsafe extern "C" fn(png_bytep, size_t, ::core::ffi::c_int, size_t) -> (),
        )
            as Option<unsafe extern "C" fn(png_bytep, size_t, ::core::ffi::c_int, size_t) -> ()>;
        bmp_fread_fn = Some(
            bmp_fread_halfbytes
                as unsafe extern "C" fn(png_bytep, size_t, size_t, *mut FILE) -> size_t,
        )
            as Option<unsafe extern "C" fn(png_bytep, size_t, size_t, *mut FILE) -> size_t>;
    } else {
        endn = row_size;
        bmp_memset_fn = Some(
            bmp_memset_bytes
                as unsafe extern "C" fn(png_bytep, size_t, ::core::ffi::c_int, size_t) -> (),
        )
            as Option<unsafe extern "C" fn(png_bytep, size_t, ::core::ffi::c_int, size_t) -> ()>;
        bmp_fread_fn = Some(
            bmp_fread_bytes as unsafe extern "C" fn(png_bytep, size_t, size_t, *mut FILE) -> size_t,
        )
            as Option<unsafe extern "C" fn(png_bytep, size_t, size_t, *mut FILE) -> size_t>;
    }
    if compression == BI_RGB as ::core::ffi::c_uint
        || compression == BI_BITFIELDS as ::core::ffi::c_uint
    {
        crt_row = begin_row;
        while crt_row != end_row {
            crtn = bmp_fread_fn.expect("non-null function pointer")(
                *crt_row,
                0 as size_t,
                endn,
                stream,
            );
            if crtn != endn {
                break;
            }
            result = result.wrapping_add(1);
            crt_row = crt_row.offset(inc as isize);
        }
    } else if compression == BI_RLE8 as ::core::ffi::c_uint
        || compression == BI_RLE4 as ::core::ffi::c_uint
    {
        if compression == BI_RLE8 as ::core::ffi::c_uint {
            endn = row_size;
        } else {
            endn = row_size.wrapping_mul(2 as size_t);
            if endn <= row_size {
                return 0 as size_t;
            }
        }
        crt_row = begin_row;
        while crt_row != end_row {
            ch = getc(stream);
            b1 = ch as ::core::ffi::c_uint;
            ch = getc(stream);
            b2 = ch as ::core::ffi::c_uint;
            if ch == EOF {
                break;
            }
            if b1 == 0 as ::core::ffi::c_uint {
                if b2 == 0 as ::core::ffi::c_uint {
                    bmp_memset_fn.expect("non-null function pointer")(
                        *crt_row,
                        crtn,
                        0 as ::core::ffi::c_int,
                        endn.wrapping_sub(crtn),
                    );
                    crt_row = crt_row.offset(inc as isize);
                    crtn = 0 as size_t;
                    result = result.wrapping_add(1);
                    if !(crt_row == end_row) {
                        continue;
                    }
                    ch = getc(stream);
                    if ch != EOF && ch != 0 as ::core::ffi::c_int {
                        ungetc(ch, stream);
                        break;
                    } else {
                        getc(stream);
                        break;
                    }
                } else if b2 == 1 as ::core::ffi::c_uint {
                    bmp_memset_fn.expect("non-null function pointer")(
                        *crt_row,
                        crtn,
                        0 as ::core::ffi::c_int,
                        endn.wrapping_sub(crtn),
                    );
                    crt_row = crt_row.offset(inc as isize);
                    crtn = 0 as size_t;
                    result = (if begin_row <= end_row {
                        end_row.offset_from(begin_row) as ::core::ffi::c_long
                    } else {
                        begin_row.offset_from(end_row) as ::core::ffi::c_long
                    }) as size_t;
                    break;
                } else if b2 == 2 as ::core::ffi::c_uint {
                    ch = getc(stream);
                    b1 = ch as ::core::ffi::c_uint;
                    ch = getc(stream);
                    b2 = ch as ::core::ffi::c_uint;
                    if ch == EOF {
                        break;
                    }
                    dcrtn = if (b1 as size_t) < endn.wrapping_sub(crtn) {
                        crtn.wrapping_add(b1 as size_t)
                    } else {
                        endn
                    };
                    while b2 > 0 as ::core::ffi::c_uint {
                        bmp_memset_fn.expect("non-null function pointer")(
                            *crt_row,
                            crtn,
                            0 as ::core::ffi::c_int,
                            endn.wrapping_sub(crtn),
                        );
                        crt_row = crt_row.offset(inc as isize);
                        crtn = 0 as size_t;
                        result = result.wrapping_add(1);
                        if crt_row == end_row {
                            break;
                        }
                        b2 = b2.wrapping_sub(1);
                    }
                    if crt_row != end_row {
                        bmp_memset_fn.expect("non-null function pointer")(
                            *crt_row,
                            crtn,
                            0 as ::core::ffi::c_int,
                            dcrtn.wrapping_sub(crtn),
                        );
                    }
                } else {
                    len = if b2 as size_t <= endn.wrapping_sub(crtn) {
                        b2
                    } else {
                        endn.wrapping_sub(crtn) as ::core::ffi::c_uint
                    };
                    if bmp_fread_fn.expect("non-null function pointer")(
                        *crt_row,
                        crtn,
                        len as size_t,
                        stream,
                    ) != len as size_t
                    {
                        break;
                    }
                    crtn = (crtn as ::core::ffi::c_ulong).wrapping_add(len as ::core::ffi::c_ulong)
                        as size_t as size_t;
                }
            } else {
                len = if b1 as size_t <= endn.wrapping_sub(crtn) {
                    b1
                } else {
                    endn.wrapping_sub(crtn) as ::core::ffi::c_uint
                };
                bmp_memset_fn.expect("non-null function pointer")(
                    *crt_row,
                    crtn,
                    b2 as ::core::ffi::c_int,
                    len as size_t,
                );
                crtn = (crtn as ::core::ffi::c_ulong).wrapping_add(len as ::core::ffi::c_ulong)
                    as size_t as size_t;
            }
        }
    } else {
        return 0 as size_t;
    }
    while crt_row != end_row {
        bmp_memset_fn.expect("non-null function pointer")(
            *crt_row,
            crtn,
            0 as ::core::ffi::c_int,
            endn.wrapping_sub(crtn),
        );
        crtn = 0 as size_t;
        crt_row = crt_row.offset(inc as isize);
    }
    return result;
}
unsafe extern "C" fn bmp_to_png_rows(
    mut row_pointers: png_bytepp,
    mut width: png_uint_32,
    mut height: png_uint_32,
    mut pixdepth: ::core::ffi::c_uint,
    mut rgba_sig: png_bytep,
    mut rgba_shift: png_bytep,
) {
    let mut src_ptr: png_bytep = ::core::ptr::null_mut::<png_byte>();
    let mut dest_ptr: png_bytep = ::core::ptr::null_mut::<png_byte>();
    let mut rgba_mask: [::core::ffi::c_uint; 4] = [0; 4];
    let mut num_samples: ::core::ffi::c_uint = 0;
    let mut sample: ::core::ffi::c_uint = 0;
    let mut mask: ::core::ffi::c_uint = 0;
    let mut wpix: ::core::ffi::c_uint = 0;
    let mut dwpix: png_uint_32 = 0;
    let mut x: png_uint_32 = 0;
    let mut y: png_uint_32 = 0;
    let mut i: ::core::ffi::c_uint = 0;
    if pixdepth == 24 as ::core::ffi::c_uint {
        y = 0 as png_uint_32;
        while y < height {
            src_ptr = *row_pointers.offset(y as isize) as png_bytep;
            x = 0 as png_uint_32;
            while x < width {
                let mut tmp: png_byte = *src_ptr.offset(0 as ::core::ffi::c_int as isize);
                *src_ptr.offset(0 as ::core::ffi::c_int as isize) =
                    *src_ptr.offset(2 as ::core::ffi::c_int as isize);
                *src_ptr.offset(2 as ::core::ffi::c_int as isize) = tmp;
                x = x.wrapping_add(1);
                src_ptr = src_ptr.offset(3 as ::core::ffi::c_int as isize);
            }
            y = y.wrapping_add(1);
        }
        return;
    }
    num_samples = (if *rgba_sig.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        != 0 as ::core::ffi::c_int
    {
        4 as ::core::ffi::c_int
    } else {
        3 as ::core::ffi::c_int
    }) as ::core::ffi::c_uint;
    i = 0 as ::core::ffi::c_uint;
    while i < num_samples {
        rgba_mask[i as usize] = ((1 as ::core::ffi::c_uint)
            << *rgba_sig.offset(i as isize) as ::core::ffi::c_int)
            .wrapping_sub(1 as ::core::ffi::c_uint);
        i = i.wrapping_add(1);
    }
    if pixdepth == 16 as ::core::ffi::c_uint {
        y = 0 as png_uint_32;
        while y < height {
            src_ptr = (*row_pointers.offset(y as isize)).offset(
                (width as ::core::ffi::c_uint)
                    .wrapping_sub(1 as ::core::ffi::c_uint)
                    .wrapping_mul(2 as ::core::ffi::c_uint) as isize,
            ) as png_bytep;
            dest_ptr = (*row_pointers.offset(y as isize)).offset(
                (width as ::core::ffi::c_uint)
                    .wrapping_sub(1 as ::core::ffi::c_uint)
                    .wrapping_mul(num_samples) as isize,
            ) as png_bytep;
            x = 0 as png_uint_32;
            while x < width {
                wpix = (*src_ptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    .wrapping_add(
                        (*src_ptr.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                            << 8 as ::core::ffi::c_int,
                    );
                i = 0 as ::core::ffi::c_uint;
                while i < num_samples {
                    mask = rgba_mask[i as usize];
                    sample = wpix >> *rgba_shift.offset(i as isize) as ::core::ffi::c_int & mask;
                    *dest_ptr.offset(i as isize) = sample
                        .wrapping_mul(255 as ::core::ffi::c_uint)
                        .wrapping_add(mask.wrapping_div(2 as ::core::ffi::c_uint))
                        .wrapping_div(mask)
                        as png_byte;
                    i = i.wrapping_add(1);
                }
                x = x.wrapping_add(1);
                src_ptr = src_ptr.offset(-(2 as ::core::ffi::c_int as isize));
                dest_ptr = dest_ptr.offset(-(num_samples as isize));
            }
            y = y.wrapping_add(1);
        }
    } else if pixdepth == 32 as ::core::ffi::c_uint {
        y = 0 as png_uint_32;
        while y < height {
            dest_ptr = *row_pointers.offset(y as isize) as png_bytep;
            src_ptr = dest_ptr;
            x = 0 as png_uint_32;
            while x < width {
                dwpix = (*src_ptr.offset(0 as ::core::ffi::c_int as isize) as png_uint_32)
                    .wrapping_add(
                        (*src_ptr.offset(1 as ::core::ffi::c_int as isize) as png_uint_32)
                            << 8 as ::core::ffi::c_int,
                    )
                    .wrapping_add(
                        (*src_ptr.offset(2 as ::core::ffi::c_int as isize) as png_uint_32)
                            << 16 as ::core::ffi::c_int,
                    )
                    .wrapping_add(
                        (*src_ptr.offset(3 as ::core::ffi::c_int as isize) as png_uint_32)
                            << 24 as ::core::ffi::c_int,
                    );
                i = 0 as ::core::ffi::c_uint;
                while i < num_samples {
                    mask = rgba_mask[i as usize];
                    sample = dwpix as ::core::ffi::c_uint
                        >> *rgba_shift.offset(i as isize) as ::core::ffi::c_int
                        & mask;
                    *dest_ptr.offset(i as isize) = sample
                        .wrapping_mul(255 as ::core::ffi::c_uint)
                        .wrapping_add(mask.wrapping_div(2 as ::core::ffi::c_uint))
                        .wrapping_div(mask)
                        as png_byte;
                    i = i.wrapping_add(1);
                }
                x = x.wrapping_add(1);
                src_ptr = src_ptr.offset(4 as ::core::ffi::c_int as isize);
                dest_ptr = dest_ptr.offset(num_samples as isize);
            }
            y = y.wrapping_add(1);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn pngx_sig_is_bmp(
    mut sig: png_bytep,
    mut sig_size: size_t,
    mut fmt_name_ptr: png_const_charpp,
    mut fmt_long_name_ptr: png_const_charpp,
) -> ::core::ffi::c_int {
    static mut bmp_fmt_name: [::core::ffi::c_char; 4] =
        unsafe { ::core::mem::transmute::<[u8; 4], [::core::ffi::c_char; 4]>(*b"BMP\0") };
    static mut os2bmp_fmt_long_name: [::core::ffi::c_char; 12] =
        unsafe { ::core::mem::transmute::<[u8; 12], [::core::ffi::c_char; 12]>(*b"OS/2 Bitmap\0") };
    static mut winbmp_fmt_long_name: [::core::ffi::c_char; 15] = unsafe {
        ::core::mem::transmute::<[u8; 15], [::core::ffi::c_char; 15]>(*b"Windows Bitmap\0")
    };
    let mut bihsize: png_uint_32 = 0;
    if sig_size < (FILEHED_SIZE + 4 as ::core::ffi::c_int) as size_t {
        return -(1 as ::core::ffi::c_int);
    }
    if bmp_get_word(sig) != BMP_SIGNATURE as ::core::ffi::c_uint {
        return 0 as ::core::ffi::c_int;
    }
    bihsize = bmp_get_dword(sig.offset(FILEHED_SIZE as isize));
    if bihsize > PNG_UINT_31_MAX
        || bihsize != COREHED_SIZE as ::core::ffi::c_uint
            && bihsize < INFOHED_SIZE as ::core::ffi::c_uint
    {
        return 0 as ::core::ffi::c_int;
    }
    if !fmt_name_ptr.is_null() {
        *fmt_name_ptr = &raw const bmp_fmt_name as *const ::core::ffi::c_char;
    }
    if !fmt_long_name_ptr.is_null() {
        if bihsize == COREHED_SIZE as ::core::ffi::c_uint {
            *fmt_long_name_ptr = &raw const os2bmp_fmt_long_name as *const ::core::ffi::c_char;
        } else {
            *fmt_long_name_ptr = &raw const winbmp_fmt_long_name as *const ::core::ffi::c_char;
        }
    }
    return 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn pngx_read_bmp(
    mut png_ptr: png_structp,
    mut info_ptr: png_infop,
    mut stream: *mut FILE,
) -> ::core::ffi::c_int {
    let mut bfh: [png_byte; 138] = [0; 138];
    let bih: png_bytep = (&raw mut bfh as *mut png_byte).offset(FILEHED_SIZE as isize);
    let mut rgbq: [png_byte; 4] = [0; 4];
    let mut offbits: png_uint_32 = 0;
    let mut bihsize: png_uint_32 = 0;
    let mut skip: png_uint_32 = 0;
    let mut width: png_uint_32 = 0;
    let mut height: png_uint_32 = 0;
    let mut rowsize: png_uint_32 = 0;
    let mut topdown: ::core::ffi::c_int = 0;
    let mut pixdepth: ::core::ffi::c_uint = 0;
    let mut compression: png_uint_32 = 0;
    let mut palsize: ::core::ffi::c_uint = 0;
    let mut palnum: ::core::ffi::c_uint = 0;
    let mut rgba_mask: [png_uint_32; 4] = [0; 4];
    let mut rgba_sig: [png_byte; 4] = [0; 4];
    let mut rgba_shift: [png_byte; 4] = [0; 4];
    let mut bit_depth: ::core::ffi::c_int = 0;
    let mut color_type: ::core::ffi::c_int = 0;
    let mut palette: [png_color; 256] = [png_color {
        red: 0,
        green: 0,
        blue: 0,
    }; 256];
    let mut sig_bit: png_color_8 = png_color_8 {
        red: 0,
        green: 0,
        blue: 0,
        gray: 0,
        alpha: 0,
    };
    let mut row_pointers: png_bytepp = ::core::ptr::null_mut::<*mut png_byte>();
    let mut begin_row: png_bytepp = ::core::ptr::null_mut::<*mut png_byte>();
    let mut end_row: png_bytepp = ::core::ptr::null_mut::<*mut png_byte>();
    let mut i: ::core::ffi::c_uint = 0;
    let mut y: size_t = 0;
    i = 0 as ::core::ffi::c_uint;
    loop {
        if fread(
            &raw mut bfh as *mut png_byte as *mut ::core::ffi::c_void,
            (FILEHED_SIZE + BIHSIZE_SIZE) as size_t,
            1 as size_t,
            stream,
        ) != 1 as ::core::ffi::c_ulong
        {
            i = i.wrapping_add(1);
        } else if bmp_get_word((&raw mut bfh as *mut png_byte).offset(BFH_WTYPE as isize))
            == BMP_SIGNATURE as ::core::ffi::c_uint
        {
            break;
        }
        if fread(
            &raw mut bfh as *mut png_byte as *mut ::core::ffi::c_void,
            (128 as ::core::ffi::c_int - FILEHED_SIZE - BIHSIZE_SIZE) as size_t,
            1 as size_t,
            stream,
        ) != 1 as ::core::ffi::c_ulong
        {
            i = i.wrapping_add(1);
        }
        if i > 0 as ::core::ffi::c_uint {
            return 0 as ::core::ffi::c_int;
        }
        i = i.wrapping_add(1);
    }
    offbits = bmp_get_dword((&raw mut bfh as *mut png_byte).offset(BFH_DOFFBITS as isize));
    bihsize = bmp_get_dword((&raw mut bfh as *mut png_byte).offset(BFH_DBIHSIZE as isize));
    if offbits > PNG_UINT_31_MAX
        || bihsize > PNG_UINT_31_MAX
        || offbits
            < (bihsize as ::core::ffi::c_uint).wrapping_add(FILEHED_SIZE as ::core::ffi::c_uint)
        || bihsize != COREHED_SIZE as ::core::ffi::c_uint
            && bihsize < INFOHED_SIZE as ::core::ffi::c_uint
    {
        return 0 as ::core::ffi::c_int;
    }
    if bihsize > BMPV5HED_SIZE as ::core::ffi::c_uint {
        skip = (bihsize as ::core::ffi::c_uint).wrapping_sub(BMPV5HED_SIZE as ::core::ffi::c_uint)
            as png_uint_32;
        bihsize = BMPV5HED_SIZE as png_uint_32;
    } else {
        skip = 0 as png_uint_32;
    }
    if fread(
        bih.offset(BIHSIZE_SIZE as isize) as *mut ::core::ffi::c_void,
        (bihsize as ::core::ffi::c_uint).wrapping_sub(BIHSIZE_SIZE as ::core::ffi::c_uint)
            as size_t,
        1 as size_t,
        stream,
    ) != 1 as ::core::ffi::c_ulong
    {
        return 0 as ::core::ffi::c_int;
    }
    if skip > 0 as ::core::ffi::c_uint {
        if fseek(stream, skip as ::core::ffi::c_long, SEEK_CUR) != 0 as ::core::ffi::c_int {
            return 0 as ::core::ffi::c_int;
        }
    }
    skip = (offbits as ::core::ffi::c_uint)
        .wrapping_sub(bihsize as ::core::ffi::c_uint)
        .wrapping_sub(FILEHED_SIZE as ::core::ffi::c_uint) as png_uint_32;
    topdown = 0 as ::core::ffi::c_int;
    if bihsize < INFOHED_SIZE as ::core::ffi::c_uint {
        width = bmp_get_word(bih.offset(BCH_WWIDTH as isize)) as png_uint_32;
        height = bmp_get_word(bih.offset(BCH_WHEIGHT as isize)) as png_uint_32;
        pixdepth = bmp_get_word(bih.offset(BCH_WBITCOUNT as isize));
        compression = BI_RGB as png_uint_32;
        palsize = RGBTRIPLE_SIZE as ::core::ffi::c_uint;
    } else {
        width = bmp_get_dword(bih.offset(BIH_LWIDTH as isize));
        height = bmp_get_dword(bih.offset(BIH_LHEIGHT as isize));
        pixdepth = bmp_get_word(bih.offset(BIH_WBITCOUNT as isize));
        compression = bmp_get_dword(bih.offset(BIH_DCOMPRESSION as isize));
        palsize = RGBQUAD_SIZE as ::core::ffi::c_uint;
        if height > PNG_UINT_31_MAX {
            height = PNG_UINT_32_MAX
                .wrapping_sub(height as ::core::ffi::c_uint)
                .wrapping_add(1 as ::core::ffi::c_uint) as png_uint_32;
            topdown = 1 as ::core::ffi::c_int;
        }
        if bihsize == INFOHED_SIZE as ::core::ffi::c_uint
            && compression == BI_BITFIELDS as ::core::ffi::c_uint
        {
            i = if skip <= 16 as ::core::ffi::c_uint {
                skip
            } else {
                16 as ::core::ffi::c_uint
            };
            if fread(
                bih.offset(B4H_DREDMASK as isize) as *mut ::core::ffi::c_void,
                i as size_t,
                1 as size_t,
                stream,
            ) != 1 as ::core::ffi::c_ulong
            {
                return 0 as ::core::ffi::c_int;
            }
            bihsize =
                (bihsize as ::core::ffi::c_uint).wrapping_add(i) as png_uint_32 as png_uint_32;
            skip = (skip as ::core::ffi::c_uint).wrapping_sub(i) as png_uint_32 as png_uint_32;
        }
    }
    memset(
        &raw mut rgba_mask as *mut png_uint_32 as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<[png_uint_32; 4]>() as size_t,
    );
    if pixdepth > 8 as ::core::ffi::c_uint {
        if compression == BI_RGB as ::core::ffi::c_uint {
            if pixdepth == 16 as ::core::ffi::c_uint {
                compression = BI_BITFIELDS as png_uint_32;
                rgba_mask[0 as ::core::ffi::c_int as usize] = 0x7c00 as png_uint_32;
                rgba_mask[1 as ::core::ffi::c_int as usize] = 0x3e0 as png_uint_32;
                rgba_mask[2 as ::core::ffi::c_int as usize] = 0x1f as png_uint_32;
            } else {
                rgba_mask[0 as ::core::ffi::c_int as usize] =
                    0xff0000 as ::core::ffi::c_long as png_uint_32;
                rgba_mask[1 as ::core::ffi::c_int as usize] =
                    0xff00 as ::core::ffi::c_long as png_uint_32;
                rgba_mask[2 as ::core::ffi::c_int as usize] =
                    0xff as ::core::ffi::c_long as png_uint_32;
            }
        } else if compression == BI_BITFIELDS as ::core::ffi::c_uint {
            if bihsize >= (INFOHED_SIZE + 12 as ::core::ffi::c_int) as ::core::ffi::c_uint {
                rgba_mask[0 as ::core::ffi::c_int as usize] =
                    bmp_get_dword(bih.offset(B4H_DREDMASK as isize));
                rgba_mask[1 as ::core::ffi::c_int as usize] =
                    bmp_get_dword(bih.offset(B4H_DGREENMASK as isize));
                rgba_mask[2 as ::core::ffi::c_int as usize] =
                    bmp_get_dword(bih.offset(B4H_DBLUEMASK as isize));
            } else {
                png_error(
                    png_ptr as png_const_structrp,
                    b"Missing color mask in BMP file\0" as *const u8 as png_const_charp,
                );
            }
        }
        if bihsize >= (INFOHED_SIZE + 16 as ::core::ffi::c_int) as ::core::ffi::c_uint {
            rgba_mask[3 as ::core::ffi::c_int as usize] =
                bmp_get_dword(bih.offset(B4H_DALPHAMASK as isize));
        }
    }
    match compression {
        0 => {
            if pixdepth > 0 as ::core::ffi::c_uint
                && (32 as ::core::ffi::c_uint).wrapping_rem(pixdepth) != 0 as ::core::ffi::c_uint
                && pixdepth != 24 as ::core::ffi::c_uint
            {
                pixdepth = 0 as ::core::ffi::c_uint;
            }
        }
        1 => {
            if pixdepth != 8 as ::core::ffi::c_uint {
                pixdepth = 0 as ::core::ffi::c_uint;
            }
        }
        2 => {
            if pixdepth != 4 as ::core::ffi::c_uint {
                pixdepth = 0 as ::core::ffi::c_uint;
            }
        }
        3 => {
            if pixdepth != 16 as ::core::ffi::c_uint && pixdepth != 32 as ::core::ffi::c_uint {
                pixdepth = 0 as ::core::ffi::c_uint;
            }
        }
        4 => {
            png_error(
                png_ptr as png_const_structrp,
                b"JPEG-compressed BMP files are not supported\0" as *const u8 as png_const_charp,
            );
        }
        5 => {
            if ungetc(getc(stream), stream) == 0 as ::core::ffi::c_int {
                png_set_sig_bytes(png_ptr as png_structrp, 8 as ::core::ffi::c_int);
            }
            png_set_read_fn(png_ptr as png_structrp, stream as png_voidp, None);
            png_read_png(
                png_ptr as png_structrp,
                info_ptr as png_inforp,
                0 as ::core::ffi::c_int,
                NULL,
            );
            return 1 as ::core::ffi::c_int;
        }
        _ => {
            png_error(
                png_ptr as png_const_structrp,
                b"Unsupported compression method in BMP file\0" as *const u8 as png_const_charp,
            );
        }
    }
    if width == 0 as ::core::ffi::c_uint
        || width > PNG_UINT_31_MAX
        || height == 0 as ::core::ffi::c_uint
    {
        png_error(
            png_ptr as png_const_structrp,
            b"Invalid image dimensions in BMP file\0" as *const u8 as png_const_charp,
        );
    }
    if pixdepth == 0 as ::core::ffi::c_uint {
        png_error(
            png_ptr as png_const_structrp,
            b"Invalid pixel depth in BMP file\0" as *const u8 as png_const_charp,
        );
    }
    if pixdepth <= 8 as ::core::ffi::c_uint {
        palnum = (skip as ::core::ffi::c_uint).wrapping_div(palsize);
        if palnum > 256 as ::core::ffi::c_uint {
            palnum = 256 as ::core::ffi::c_uint;
        }
        skip = (skip as ::core::ffi::c_uint).wrapping_sub(palsize.wrapping_mul(palnum))
            as png_uint_32 as png_uint_32;
        rowsize = (width as ::core::ffi::c_uint)
            .wrapping_add((32 as ::core::ffi::c_uint).wrapping_div(pixdepth))
            .wrapping_sub(1 as ::core::ffi::c_uint)
            .wrapping_div((32 as ::core::ffi::c_uint).wrapping_div(pixdepth))
            .wrapping_mul(4 as ::core::ffi::c_uint) as png_uint_32;
        bit_depth = pixdepth as ::core::ffi::c_int;
        color_type = if palnum > 0 as ::core::ffi::c_uint {
            PNG_COLOR_TYPE_PALETTE
        } else {
            PNG_COLOR_TYPE_GRAY
        };
    } else {
        palnum = 0 as ::core::ffi::c_uint;
        bit_depth = 8 as ::core::ffi::c_int;
        match pixdepth {
            16 => {
                rowsize = ((width as ::core::ffi::c_uint)
                    .wrapping_mul(2 as ::core::ffi::c_uint)
                    .wrapping_add(3 as ::core::ffi::c_uint)
                    & !(3 as ::core::ffi::c_int) as ::core::ffi::c_uint)
                    as png_uint_32;
            }
            24 => {
                rowsize = ((width as ::core::ffi::c_uint)
                    .wrapping_mul(3 as ::core::ffi::c_uint)
                    .wrapping_add(3 as ::core::ffi::c_uint)
                    & !(3 as ::core::ffi::c_int) as ::core::ffi::c_uint)
                    as png_uint_32;
            }
            32 => {
                rowsize = (width as ::core::ffi::c_uint).wrapping_mul(4 as ::core::ffi::c_uint)
                    as png_uint_32;
            }
            _ => {
                bit_depth = 0 as ::core::ffi::c_int;
                rowsize = 0 as png_uint_32;
            }
        }
        if rowsize.wrapping_div(width) < pixdepth.wrapping_div(8 as ::core::ffi::c_uint) {
            rowsize = 0 as png_uint_32;
        }
        color_type = if rgba_mask[3 as ::core::ffi::c_int as usize] != 0 as ::core::ffi::c_uint {
            PNG_COLOR_TYPE_RGBA
        } else {
            PNG_COLOR_TYPE_RGB
        };
    }
    if rowsize == 0 as ::core::ffi::c_uint {
        png_error(
            png_ptr as png_const_structrp,
            b"Can't handle exceedingly large BMP dimensions\0" as *const u8 as png_const_charp,
        );
    }
    png_set_IHDR(
        png_ptr as png_const_structrp,
        info_ptr as png_inforp,
        width,
        height,
        bit_depth,
        color_type,
        PNG_INTERLACE_NONE,
        PNG_COMPRESSION_TYPE_BASE,
        PNG_FILTER_TYPE_BASE,
    );
    if pixdepth > 8 as ::core::ffi::c_uint {
        i = 0 as ::core::ffi::c_uint;
        while i < 4 as ::core::ffi::c_uint {
            bmp_process_mask(
                rgba_mask[i as usize],
                (&raw mut rgba_sig as *mut png_byte).offset(i as isize) as png_bytep,
                (&raw mut rgba_shift as *mut png_byte).offset(i as isize) as png_bytep,
            );
            i = i.wrapping_add(1);
        }
        if rgba_sig[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
            || rgba_sig[1 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
            || rgba_sig[2 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
        {
            png_error(
                png_ptr as png_const_structrp,
                b"Invalid color mask in BMP file\0" as *const u8 as png_const_charp,
            );
        }
        if rgba_sig[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
            != 8 as ::core::ffi::c_int
            || rgba_sig[1 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                != 8 as ::core::ffi::c_int
            || rgba_sig[2 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                != 8 as ::core::ffi::c_int
            || rgba_sig[3 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                != 0 as ::core::ffi::c_int
                && rgba_sig[3 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                    != 8 as ::core::ffi::c_int
        {
            sig_bit.red = rgba_sig[0 as ::core::ffi::c_int as usize];
            sig_bit.green = rgba_sig[1 as ::core::ffi::c_int as usize];
            sig_bit.blue = rgba_sig[2 as ::core::ffi::c_int as usize];
            sig_bit.alpha = rgba_sig[3 as ::core::ffi::c_int as usize];
            png_set_sBIT(
                png_ptr as png_const_structrp,
                info_ptr as png_inforp,
                &raw mut sig_bit as png_const_color_8p,
            );
        }
    }
    if palnum > 0 as ::core::ffi::c_uint {
        i = 0 as ::core::ffi::c_uint;
        while i < palnum {
            if fread(
                &raw mut rgbq as *mut png_byte as *mut ::core::ffi::c_void,
                palsize as size_t,
                1 as size_t,
                stream,
            ) != 1 as ::core::ffi::c_ulong
            {
                break;
            }
            palette[i as usize].red = rgbq[RGB_RED as usize];
            palette[i as usize].green = rgbq[RGB_GREEN as usize];
            palette[i as usize].blue = rgbq[RGB_BLUE as usize];
            i = i.wrapping_add(1);
        }
        png_set_PLTE(
            png_ptr as png_structrp,
            info_ptr as png_inforp,
            &raw mut palette as *mut png_color as png_const_colorp,
            i as ::core::ffi::c_int,
        );
        if i != palnum {
            png_error(
                png_ptr as png_const_structrp,
                b"Error reading color palette in BMP file\0" as *const u8 as png_const_charp,
            );
        }
    }
    row_pointers = pngx_malloc_rows_extended(
        png_ptr,
        info_ptr,
        rowsize as pngx_alloc_size_t,
        -(1 as ::core::ffi::c_int),
    );
    if topdown != 0 {
        begin_row = row_pointers;
        end_row = row_pointers.offset(height as isize);
    } else {
        begin_row = row_pointers
            .offset(height as isize)
            .offset(-(1 as ::core::ffi::c_int as isize));
        end_row = row_pointers.offset(-(1 as ::core::ffi::c_int as isize));
    }
    if skip > 0 as ::core::ffi::c_uint {
        fseek(stream, skip as ::core::ffi::c_long, SEEK_CUR);
    }
    y = bmp_read_rows(
        begin_row,
        end_row,
        rowsize as size_t,
        compression as ::core::ffi::c_uint,
        stream,
    );
    if pixdepth > 8 as ::core::ffi::c_uint {
        bmp_to_png_rows(
            row_pointers,
            width,
            height,
            pixdepth,
            &raw mut rgba_sig as png_bytep,
            &raw mut rgba_shift as png_bytep,
        );
    }
    if y != height as size_t {
        png_error(
            png_ptr as png_const_structrp,
            b"Error reading BMP file\0" as *const u8 as png_const_charp,
        );
    }
    return 1 as ::core::ffi::c_int;
}
