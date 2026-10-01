extern "C" {
    pub type png_struct_def;
    pub type png_info_def;
    fn png_malloc(png_ptr: png_const_structrp, size: png_alloc_size_t) -> png_voidp;
    fn png_free(png_ptr: png_const_structrp, ptr: png_voidp);
    fn png_free_data(
        png_ptr: png_const_structrp,
        info_ptr: png_inforp,
        free_me: png_uint_32,
        num: ::core::ffi::c_int,
    );
    fn png_warning(png_ptr: png_const_structrp, warning_message: png_const_charp);
    fn png_get_valid(
        png_ptr: png_const_structrp,
        info_ptr: png_const_inforp,
        flag: png_uint_32,
    ) -> png_uint_32;
    fn png_get_rows(png_ptr: png_const_structrp, info_ptr: png_const_inforp) -> png_bytepp;
    fn png_get_channels(png_ptr: png_const_structrp, info_ptr: png_const_inforp) -> png_byte;
    fn png_get_image_width(png_ptr: png_const_structrp, info_ptr: png_const_inforp) -> png_uint_32;
    fn png_get_image_height(png_ptr: png_const_structrp, info_ptr: png_const_inforp)
        -> png_uint_32;
    fn png_get_bit_depth(png_ptr: png_const_structrp, info_ptr: png_const_inforp) -> png_byte;
    fn png_get_color_type(png_ptr: png_const_structrp, info_ptr: png_const_inforp) -> png_byte;
    fn png_get_bKGD(
        png_ptr: png_const_structrp,
        info_ptr: png_inforp,
        background: *mut png_color_16p,
    ) -> png_uint_32;
    fn png_get_hIST(
        png_ptr: png_const_structrp,
        info_ptr: png_inforp,
        hist: *mut png_uint_16p,
    ) -> png_uint_32;
    fn png_get_IHDR(
        png_ptr: png_const_structrp,
        info_ptr: png_const_inforp,
        width: *mut png_uint_32,
        height: *mut png_uint_32,
        bit_depth: *mut ::core::ffi::c_int,
        color_type: *mut ::core::ffi::c_int,
        interlace_method: *mut ::core::ffi::c_int,
        compression_method: *mut ::core::ffi::c_int,
        filter_method: *mut ::core::ffi::c_int,
    ) -> png_uint_32;
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
    fn png_get_PLTE(
        png_ptr: png_const_structrp,
        info_ptr: png_inforp,
        palette: *mut png_colorp,
        num_palette: *mut ::core::ffi::c_int,
    ) -> png_uint_32;
    fn png_set_PLTE(
        png_ptr: png_structrp,
        info_ptr: png_inforp,
        palette: png_const_colorp,
        num_palette: ::core::ffi::c_int,
    );
    fn png_get_sBIT(
        png_ptr: png_const_structrp,
        info_ptr: png_inforp,
        sig_bit: *mut png_color_8p,
    ) -> png_uint_32;
    fn png_get_tRNS(
        png_ptr: png_const_structrp,
        info_ptr: png_inforp,
        trans_alpha: *mut png_bytep,
        num_trans: *mut ::core::ffi::c_int,
        trans_color: *mut png_color_16p,
    ) -> png_uint_32;
    fn png_set_tRNS(
        png_ptr: png_structrp,
        info_ptr: png_inforp,
        trans_alpha: png_const_bytep,
        num_trans: ::core::ffi::c_int,
        trans_color: png_const_color_16p,
    );
    fn png_set_invalid(png_ptr: png_const_structrp, info_ptr: png_inforp, mask: ::core::ffi::c_int);
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn __assert_fail(
        __assertion: *const ::core::ffi::c_char,
        __file: *const ::core::ffi::c_char,
        __line: ::core::ffi::c_uint,
        __function: *const ::core::ffi::c_char,
    ) -> !;
}
pub type size_t = usize;
pub type png_byte = ::core::ffi::c_uchar;
pub type png_uint_16 = ::core::ffi::c_ushort;
pub type png_uint_32 = ::core::ffi::c_uint;
pub type png_size_t = size_t;
pub type png_alloc_size_t = png_size_t;
pub type png_voidp = *mut ::core::ffi::c_void;
pub type png_bytep = *mut png_byte;
pub type png_const_bytep = *const png_byte;
pub type png_uint_16p = *mut png_uint_16;
pub type png_const_charp = *const ::core::ffi::c_char;
pub type png_bytepp = *mut *mut png_byte;
pub type png_struct = png_struct_def;
pub type png_structp = *mut png_struct;
pub type png_info = png_info_def;
pub type png_infop = *mut png_info;
pub type png_structrp = *mut png_struct;
pub type png_const_structrp = *const png_struct;
pub type png_inforp = *mut png_info;
pub type png_const_inforp = *const png_info;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct png_color_struct {
    pub red: png_byte,
    pub green: png_byte,
    pub blue: png_byte,
}
pub type png_color = png_color_struct;
pub type png_colorp = *mut png_color;
pub type png_const_colorp = *const png_color;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct png_color_16_struct {
    pub index: png_byte,
    pub red: png_uint_16,
    pub green: png_uint_16,
    pub blue: png_uint_16,
    pub gray: png_uint_16,
}
pub type png_color_16 = png_color_16_struct;
pub type png_color_16p = *mut png_color_16;
pub type png_const_color_16p = *const png_color_16;
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
pub type png_color_8p = *mut png_color_8;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct png_row_info_struct {
    pub width: png_uint_32,
    pub rowbytes: png_size_t,
    pub color_type: png_byte,
    pub bit_depth: png_byte,
    pub channels: png_byte,
    pub pixel_depth: png_byte,
}
pub type png_row_info = png_row_info_struct;
pub type png_row_infop = *mut png_row_info;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const PNG_COLOR_MASK_PALETTE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const PNG_COLOR_MASK_COLOR: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const PNG_COLOR_MASK_ALPHA: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const PNG_COLOR_TYPE_GRAY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const PNG_COLOR_TYPE_PALETTE: ::core::ffi::c_int =
    PNG_COLOR_MASK_COLOR | PNG_COLOR_MASK_PALETTE;
pub const PNG_COLOR_TYPE_RGB: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const PNG_INFO_PLTE: ::core::ffi::c_uint = 0x8 as ::core::ffi::c_uint;
pub const PNG_INFO_tRNS: ::core::ffi::c_uint = 0x10 as ::core::ffi::c_uint;
pub const PNG_INFO_hIST: ::core::ffi::c_uint = 0x40 as ::core::ffi::c_uint;
pub const PNG_INFO_IDAT: ::core::ffi::c_uint = 0x8000 as ::core::ffi::c_uint;
pub const PNG_FREE_HIST: ::core::ffi::c_uint = 0x8 as ::core::ffi::c_uint;
pub const PNG_FREE_PLTE: ::core::ffi::c_uint = 0x1000 as ::core::ffi::c_uint;
pub const PNG_FREE_TRNS: ::core::ffi::c_uint = 0x2000 as ::core::ffi::c_uint;
pub const OPNG_REDUCE_NONE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const OPNG_REDUCE_16_TO_8: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const OPNG_REDUCE_8_TO_4_2_1: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const OPNG_REDUCE_RGB_TO_GRAY: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const OPNG_REDUCE_STRIP_ALPHA: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const OPNG_REDUCE_RGB_TO_PALETTE: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const OPNG_REDUCE_GRAY_TO_PALETTE: ::core::ffi::c_int = 0x40 as ::core::ffi::c_int;
pub const OPNG_REDUCE_PALETTE_TO_GRAY: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;
pub const OPNG_REDUCE_PALETTE_FAST: ::core::ffi::c_int = 0x200 as ::core::ffi::c_int;
pub const OPNG_REDUCE_REPAIR: ::core::ffi::c_int = 0x2000 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn opng_validate_image(
    mut png_ptr: png_structp,
    mut info_ptr: png_infop,
) -> ::core::ffi::c_int {
    if png_get_bit_depth(png_ptr as png_const_structrp, info_ptr as png_const_inforp)
        as ::core::ffi::c_int
        == 0 as ::core::ffi::c_int
    {
        return 0 as ::core::ffi::c_int;
    }
    if png_get_color_type(png_ptr as png_const_structrp, info_ptr as png_const_inforp)
        as ::core::ffi::c_int
        & PNG_COLOR_MASK_PALETTE
        != 0
    {
        if png_get_valid(
            png_ptr as png_const_structrp,
            info_ptr as png_const_inforp,
            PNG_INFO_PLTE,
        ) == 0
        {
            return 0 as ::core::ffi::c_int;
        }
    }
    if png_get_valid(
        png_ptr as png_const_structrp,
        info_ptr as png_const_inforp,
        PNG_INFO_IDAT,
    ) == 0
    {
        return 0 as ::core::ffi::c_int;
    }
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn opng_insert_palette_entry(
    mut palette: png_colorp,
    mut num_palette: *mut ::core::ffi::c_int,
    mut trans_alpha: png_bytep,
    mut num_trans: *mut ::core::ffi::c_int,
    mut max_tuples: ::core::ffi::c_int,
    mut red: ::core::ffi::c_uint,
    mut green: ::core::ffi::c_uint,
    mut blue: ::core::ffi::c_uint,
    mut alpha: ::core::ffi::c_uint,
    mut index: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut low: ::core::ffi::c_int = 0;
    let mut high: ::core::ffi::c_int = 0;
    let mut mid: ::core::ffi::c_int = 0;
    let mut cmp: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    '_c2rust_label: {
        if *num_palette >= 0 as ::core::ffi::c_int && *num_palette <= max_tuples {
        } else {
            __assert_fail(
                b"*num_palette >= 0 && *num_palette <= max_tuples\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"opngreduc.c\0" as *const u8 as *const ::core::ffi::c_char,
                109 as ::core::ffi::c_uint,
                b"int opng_insert_palette_entry(png_colorp, int *, png_bytep, int *, int, unsigned int, unsigned int, unsigned int, unsigned int, int *)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if *num_trans >= 0 as ::core::ffi::c_int && *num_trans <= *num_palette {
        } else {
            __assert_fail(
                b"*num_trans >= 0 && *num_trans <= *num_palette\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"opngreduc.c\0" as *const u8 as *const ::core::ffi::c_char,
                110 as ::core::ffi::c_uint,
                b"int opng_insert_palette_entry(png_colorp, int *, png_bytep, int *, int, unsigned int, unsigned int, unsigned int, unsigned int, int *)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    if alpha < 255 as ::core::ffi::c_uint {
        low = 0 as ::core::ffi::c_int;
        high = *num_trans - 1 as ::core::ffi::c_int;
        while low <= high {
            mid = (low + high) / 2 as ::core::ffi::c_int;
            cmp = if alpha as ::core::ffi::c_int
                != *trans_alpha.offset(mid as isize) as ::core::ffi::c_int
            {
                alpha as ::core::ffi::c_int
                    - *trans_alpha.offset(mid as isize) as ::core::ffi::c_int
            } else if red as ::core::ffi::c_int
                != (*palette.offset(mid as isize)).red as ::core::ffi::c_int
            {
                red as ::core::ffi::c_int
                    - (*palette.offset(mid as isize)).red as ::core::ffi::c_int
            } else if green as ::core::ffi::c_int
                != (*palette.offset(mid as isize)).green as ::core::ffi::c_int
            {
                green as ::core::ffi::c_int
                    - (*palette.offset(mid as isize)).green as ::core::ffi::c_int
            } else {
                blue as ::core::ffi::c_int
                    - (*palette.offset(mid as isize)).blue as ::core::ffi::c_int
            };
            if cmp < 0 as ::core::ffi::c_int {
                high = mid - 1 as ::core::ffi::c_int;
            } else if cmp > 0 as ::core::ffi::c_int {
                low = mid + 1 as ::core::ffi::c_int;
            } else {
                *index = mid;
                return 0 as ::core::ffi::c_int;
            }
        }
    } else {
        low = *num_trans;
        high = *num_palette - 1 as ::core::ffi::c_int;
        while low <= high {
            mid = (low + high) / 2 as ::core::ffi::c_int;
            cmp = if red as ::core::ffi::c_int
                != (*palette.offset(mid as isize)).red as ::core::ffi::c_int
            {
                red as ::core::ffi::c_int
                    - (*palette.offset(mid as isize)).red as ::core::ffi::c_int
            } else if green as ::core::ffi::c_int
                != (*palette.offset(mid as isize)).green as ::core::ffi::c_int
            {
                green as ::core::ffi::c_int
                    - (*palette.offset(mid as isize)).green as ::core::ffi::c_int
            } else {
                blue as ::core::ffi::c_int
                    - (*palette.offset(mid as isize)).blue as ::core::ffi::c_int
            };
            if cmp < 0 as ::core::ffi::c_int {
                high = mid - 1 as ::core::ffi::c_int;
            } else if cmp > 0 as ::core::ffi::c_int {
                low = mid + 1 as ::core::ffi::c_int;
            } else {
                *index = mid;
                return 0 as ::core::ffi::c_int;
            }
        }
    }
    if alpha > 255 as ::core::ffi::c_uint {
        i = 0 as ::core::ffi::c_int;
        while i < *num_trans {
            cmp = if red as ::core::ffi::c_int
                != (*palette.offset(i as isize)).red as ::core::ffi::c_int
            {
                red as ::core::ffi::c_int - (*palette.offset(i as isize)).red as ::core::ffi::c_int
            } else if green as ::core::ffi::c_int
                != (*palette.offset(i as isize)).green as ::core::ffi::c_int
            {
                green as ::core::ffi::c_int
                    - (*palette.offset(i as isize)).green as ::core::ffi::c_int
            } else {
                blue as ::core::ffi::c_int
                    - (*palette.offset(i as isize)).blue as ::core::ffi::c_int
            };
            if cmp == 0 as ::core::ffi::c_int {
                *index = i;
                return 0 as ::core::ffi::c_int;
            }
            i += 1;
        }
    }
    if *num_palette >= max_tuples {
        *index = -(1 as ::core::ffi::c_int);
        *num_trans = *index;
        *num_palette = *num_trans;
        return -(1 as ::core::ffi::c_int);
    }
    '_c2rust_label_1: {
        if low >= 0 as ::core::ffi::c_int && low <= *num_palette {
        } else {
            __assert_fail(
                b"low >= 0 && low <= *num_palette\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"opngreduc.c\0" as *const u8 as *const ::core::ffi::c_char,
                179 as ::core::ffi::c_uint,
                b"int opng_insert_palette_entry(png_colorp, int *, png_bytep, int *, int, unsigned int, unsigned int, unsigned int, unsigned int, int *)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    i = *num_palette;
    while i > low {
        *palette.offset(i as isize) = *palette.offset((i - 1 as ::core::ffi::c_int) as isize);
        i -= 1;
    }
    (*palette.offset(low as isize)).red = red as png_byte;
    (*palette.offset(low as isize)).green = green as png_byte;
    (*palette.offset(low as isize)).blue = blue as png_byte;
    *num_palette += 1;
    if alpha < 255 as ::core::ffi::c_uint {
        '_c2rust_label_2: {
            if low <= *num_trans {
            } else {
                __assert_fail(
                    b"low <= *num_trans\0" as *const u8 as *const ::core::ffi::c_char,
                    b"opngreduc.c\0" as *const u8 as *const ::core::ffi::c_char,
                    188 as ::core::ffi::c_uint,
                    b"int opng_insert_palette_entry(png_colorp, int *, png_bytep, int *, int, unsigned int, unsigned int, unsigned int, unsigned int, int *)\0"
                        as *const u8 as *const ::core::ffi::c_char,
                );
            }
        };
        i = *num_trans;
        while i > low {
            *trans_alpha.offset(i as isize) =
                *trans_alpha.offset((i - 1 as ::core::ffi::c_int) as isize);
            i -= 1;
        }
        *trans_alpha.offset(low as isize) = alpha as png_byte;
        *num_trans += 1;
    }
    *index = low;
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn opng_realloc_PLTE(
    mut png_ptr: png_structp,
    mut info_ptr: png_infop,
    mut num_palette: ::core::ffi::c_int,
) {
    let mut buffer: [png_color; 256] = [png_color {
        red: 0,
        green: 0,
        blue: 0,
    }; 256];
    let mut palette: png_colorp = ::core::ptr::null_mut::<png_color>();
    let mut src_num_palette: ::core::ffi::c_int = 0;
    '_c2rust_label: {
        if num_palette > 0 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"num_palette > 0\0" as *const u8 as *const ::core::ffi::c_char,
                b"opngreduc.c\0" as *const u8 as *const ::core::ffi::c_char,
                212 as ::core::ffi::c_uint,
                b"void opng_realloc_PLTE(png_structp, png_infop, int)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    src_num_palette = 0 as ::core::ffi::c_int;
    png_get_PLTE(
        png_ptr as png_const_structrp,
        info_ptr as png_inforp,
        &raw mut palette,
        &raw mut src_num_palette,
    );
    if num_palette == src_num_palette {
        return;
    }
    memcpy(
        &raw mut buffer as *mut png_color as *mut ::core::ffi::c_void,
        palette as *const ::core::ffi::c_void,
        (num_palette as size_t).wrapping_mul(::core::mem::size_of::<png_color>() as size_t),
    );
    if num_palette > src_num_palette {
        memset(
            (&raw mut buffer as *mut png_color).offset(src_num_palette as isize)
                as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ((num_palette - src_num_palette) as size_t)
                .wrapping_mul(::core::mem::size_of::<png_color>() as size_t),
        );
    }
    png_set_PLTE(
        png_ptr as png_structrp,
        info_ptr as png_inforp,
        &raw mut buffer as *mut png_color as png_const_colorp,
        num_palette,
    );
}
unsafe extern "C" fn opng_realloc_tRNS(
    mut png_ptr: png_structp,
    mut info_ptr: png_infop,
    mut num_trans: ::core::ffi::c_int,
) {
    let mut buffer: [png_byte; 256] = [0; 256];
    let mut trans_alpha: png_bytep = ::core::ptr::null_mut::<png_byte>();
    let mut src_num_trans: ::core::ffi::c_int = 0;
    '_c2rust_label: {
        if num_trans > 0 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"num_trans > 0\0" as *const u8 as *const ::core::ffi::c_char,
                b"opngreduc.c\0" as *const u8 as *const ::core::ffi::c_char,
                238 as ::core::ffi::c_uint,
                b"void opng_realloc_tRNS(png_structp, png_infop, int)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    src_num_trans = 0 as ::core::ffi::c_int;
    png_get_tRNS(
        png_ptr as png_const_structrp,
        info_ptr as png_inforp,
        &raw mut trans_alpha,
        &raw mut src_num_trans,
        ::core::ptr::null_mut::<png_color_16p>(),
    );
    if num_trans == src_num_trans {
        return;
    }
    memcpy(
        &raw mut buffer as *mut png_byte as *mut ::core::ffi::c_void,
        trans_alpha as *const ::core::ffi::c_void,
        num_trans as size_t,
    );
    if num_trans > src_num_trans {
        memset(
            (&raw mut buffer as *mut png_byte).offset(src_num_trans as isize)
                as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            (num_trans - src_num_trans) as size_t,
        );
    }
    png_set_tRNS(
        png_ptr as png_structrp,
        info_ptr as png_inforp,
        &raw mut buffer as *mut png_byte as png_const_bytep,
        num_trans,
        ::core::ptr::null::<png_color_16>(),
    );
}
unsafe extern "C" fn opng_get_alpha_row(
    mut row_info_ptr: png_row_infop,
    mut trans_color: png_color_16p,
    mut row: png_bytep,
    mut alpha_row: png_bytep,
) {
    let mut sample_ptr: png_bytep = ::core::ptr::null_mut::<png_byte>();
    let mut width: png_uint_32 = 0;
    let mut color_type: ::core::ffi::c_int = 0;
    let mut bit_depth: ::core::ffi::c_int = 0;
    let mut channels: ::core::ffi::c_int = 0;
    let mut trans_red: png_byte = 0;
    let mut trans_green: png_byte = 0;
    let mut trans_blue: png_byte = 0;
    let mut trans_gray: png_byte = 0;
    let mut i: png_uint_32 = 0;
    width = (*row_info_ptr).width;
    color_type = (*row_info_ptr).color_type as ::core::ffi::c_int;
    bit_depth = (*row_info_ptr).bit_depth as ::core::ffi::c_int;
    channels = (*row_info_ptr).channels as ::core::ffi::c_int;
    '_c2rust_label: {
        if color_type & 1 as ::core::ffi::c_int == 0 {
        } else {
            __assert_fail(
                b"!(color_type & 1)\0" as *const u8 as *const ::core::ffi::c_char,
                b"opngreduc.c\0" as *const u8 as *const ::core::ffi::c_char,
                267 as ::core::ffi::c_uint,
                b"void opng_get_alpha_row(png_row_infop, png_color_16p, png_bytep, png_bytep)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if bit_depth == 8 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"bit_depth == 8\0" as *const u8 as *const ::core::ffi::c_char,
                b"opngreduc.c\0" as *const u8 as *const ::core::ffi::c_char,
                268 as ::core::ffi::c_uint,
                b"void opng_get_alpha_row(png_row_infop, png_color_16p, png_bytep, png_bytep)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    if color_type & PNG_COLOR_MASK_ALPHA == 0 {
        if trans_color.is_null() {
            memset(
                alpha_row as *mut ::core::ffi::c_void,
                255 as ::core::ffi::c_int,
                width as size_t,
            );
            return;
        }
        if color_type == PNG_COLOR_TYPE_RGB {
            '_c2rust_label_1: {
                if channels == 3 as ::core::ffi::c_int {
                } else {
                    __assert_fail(
                        b"channels == 3\0" as *const u8 as *const ::core::ffi::c_char,
                        b"opngreduc.c\0" as *const u8 as *const ::core::ffi::c_char,
                        280 as ::core::ffi::c_uint,
                        b"void opng_get_alpha_row(png_row_infop, png_color_16p, png_bytep, png_bytep)\0"
                            as *const u8 as *const ::core::ffi::c_char,
                    );
                }
            };
            trans_red = (*trans_color).red as png_byte;
            trans_green = (*trans_color).green as png_byte;
            trans_blue = (*trans_color).blue as png_byte;
            sample_ptr = row;
            i = 0 as png_uint_32;
            while i < width {
                *alpha_row.offset(i as isize) = (if *sample_ptr
                    .offset(0 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    == trans_red as ::core::ffi::c_int
                    && *sample_ptr.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        == trans_green as ::core::ffi::c_int
                    && *sample_ptr.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        == trans_blue as ::core::ffi::c_int
                {
                    0 as ::core::ffi::c_int
                } else {
                    255 as ::core::ffi::c_int
                }) as png_byte;
                i = i.wrapping_add(1);
                sample_ptr = sample_ptr.offset(3 as ::core::ffi::c_int as isize);
            }
        } else {
            '_c2rust_label_2: {
                if color_type == 0 as ::core::ffi::c_int {
                } else {
                    __assert_fail(
                        b"color_type == 0\0" as *const u8 as *const ::core::ffi::c_char,
                        b"opngreduc.c\0" as *const u8 as *const ::core::ffi::c_char,
                        293 as ::core::ffi::c_uint,
                        b"void opng_get_alpha_row(png_row_infop, png_color_16p, png_bytep, png_bytep)\0"
                            as *const u8 as *const ::core::ffi::c_char,
                    );
                }
            };
            '_c2rust_label_3: {
                if channels == 1 as ::core::ffi::c_int {
                } else {
                    __assert_fail(
                        b"channels == 1\0" as *const u8 as *const ::core::ffi::c_char,
                        b"opngreduc.c\0" as *const u8 as *const ::core::ffi::c_char,
                        294 as ::core::ffi::c_uint,
                        b"void opng_get_alpha_row(png_row_infop, png_color_16p, png_bytep, png_bytep)\0"
                            as *const u8 as *const ::core::ffi::c_char,
                    );
                }
            };
            trans_gray = (*trans_color).gray as png_byte;
            i = 0 as png_uint_32;
            while i < width {
                *alpha_row.offset(i as isize) = (if *row.offset(i as isize) as ::core::ffi::c_int
                    == trans_gray as ::core::ffi::c_int
                {
                    0 as ::core::ffi::c_int
                } else {
                    255 as ::core::ffi::c_int
                }) as png_byte;
                i = i.wrapping_add(1);
            }
        }
        return;
    }
    '_c2rust_label_4: {
        if channels > 1 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"channels > 1\0" as *const u8 as *const ::core::ffi::c_char,
                b"opngreduc.c\0" as *const u8 as *const ::core::ffi::c_char,
                303 as ::core::ffi::c_uint,
                b"void opng_get_alpha_row(png_row_infop, png_color_16p, png_bytep, png_bytep)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    sample_ptr = row.offset((channels - 1 as ::core::ffi::c_int) as isize);
    i = 0 as png_uint_32;
    while i < width {
        *alpha_row = *sample_ptr;
        i = i.wrapping_add(1);
        sample_ptr = sample_ptr.offset(channels as isize);
        alpha_row = alpha_row.offset(1);
    }
}
unsafe extern "C" fn opng_analyze_bits(
    mut png_ptr: png_structp,
    mut info_ptr: png_infop,
    mut reductions: png_uint_32,
) -> png_uint_32 {
    let mut row_ptr: png_bytepp = ::core::ptr::null_mut::<*mut png_byte>();
    let mut component_ptr: png_bytep = ::core::ptr::null_mut::<png_byte>();
    let mut height: png_uint_32 = 0;
    let mut width: png_uint_32 = 0;
    let mut bit_depth: ::core::ffi::c_int = 0;
    let mut color_type: ::core::ffi::c_int = 0;
    let mut byte_depth: ::core::ffi::c_int = 0;
    let mut channels: ::core::ffi::c_int = 0;
    let mut sample_size: ::core::ffi::c_int = 0;
    let mut offset_alpha: ::core::ffi::c_int = 0;
    let mut background: png_color_16p = ::core::ptr::null_mut::<png_color_16>();
    let mut i: png_uint_32 = 0;
    let mut j: png_uint_32 = 0;
    png_get_IHDR(
        png_ptr as png_const_structrp,
        info_ptr as png_const_inforp,
        &raw mut width,
        &raw mut height,
        &raw mut bit_depth,
        &raw mut color_type,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
    );
    if bit_depth < 8 as ::core::ffi::c_int {
        return OPNG_REDUCE_NONE as png_uint_32;
    }
    if color_type & PNG_COLOR_MASK_PALETTE != 0 {
        return OPNG_REDUCE_NONE as png_uint_32;
    }
    byte_depth = bit_depth / 8 as ::core::ffi::c_int;
    channels = png_get_channels(png_ptr as png_const_structrp, info_ptr as png_const_inforp)
        as ::core::ffi::c_int;
    sample_size = channels * byte_depth;
    offset_alpha = (channels - 1 as ::core::ffi::c_int) * byte_depth;
    reductions &= (OPNG_REDUCE_16_TO_8 | OPNG_REDUCE_RGB_TO_GRAY | OPNG_REDUCE_STRIP_ALPHA)
        as ::core::ffi::c_uint;
    if bit_depth <= 8 as ::core::ffi::c_int {
        reductions &= !OPNG_REDUCE_16_TO_8 as ::core::ffi::c_uint;
    }
    if color_type & PNG_COLOR_MASK_COLOR == 0 {
        reductions &= !OPNG_REDUCE_RGB_TO_GRAY as ::core::ffi::c_uint;
    }
    if color_type & PNG_COLOR_MASK_ALPHA == 0 {
        reductions &= !OPNG_REDUCE_STRIP_ALPHA as ::core::ffi::c_uint;
    }
    if png_get_bKGD(
        png_ptr as png_const_structrp,
        info_ptr as png_inforp,
        &raw mut background,
    ) != 0
    {
        if reductions as ::core::ffi::c_uint & OPNG_REDUCE_16_TO_8 as ::core::ffi::c_uint != 0 {
            if (*background).red as ::core::ffi::c_int % 257 as ::core::ffi::c_int
                != 0 as ::core::ffi::c_int
                || (*background).green as ::core::ffi::c_int % 257 as ::core::ffi::c_int
                    != 0 as ::core::ffi::c_int
                || (*background).blue as ::core::ffi::c_int % 257 as ::core::ffi::c_int
                    != 0 as ::core::ffi::c_int
                || (*background).gray as ::core::ffi::c_int % 257 as ::core::ffi::c_int
                    != 0 as ::core::ffi::c_int
            {
                reductions &= !OPNG_REDUCE_16_TO_8 as ::core::ffi::c_uint;
            }
        }
        if reductions as ::core::ffi::c_uint & OPNG_REDUCE_RGB_TO_GRAY as ::core::ffi::c_uint != 0 {
            if (*background).red as ::core::ffi::c_int != (*background).green as ::core::ffi::c_int
                || (*background).red as ::core::ffi::c_int
                    != (*background).blue as ::core::ffi::c_int
            {
                reductions &= !OPNG_REDUCE_RGB_TO_GRAY as ::core::ffi::c_uint;
            }
        }
    }
    row_ptr = png_get_rows(png_ptr as png_const_structrp, info_ptr as png_const_inforp);
    i = 0 as png_uint_32;
    while i < height {
        if reductions == OPNG_REDUCE_NONE as ::core::ffi::c_uint {
            return OPNG_REDUCE_NONE as png_uint_32;
        }
        if reductions as ::core::ffi::c_uint & OPNG_REDUCE_16_TO_8 as ::core::ffi::c_uint != 0 {
            component_ptr = *row_ptr as png_bytep;
            j = 0 as png_uint_32;
            while j < (channels as png_uint_32).wrapping_mul(width) {
                if *component_ptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    != *component_ptr.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                {
                    reductions &= !OPNG_REDUCE_16_TO_8 as ::core::ffi::c_uint;
                    break;
                } else {
                    j = j.wrapping_add(1);
                    component_ptr = component_ptr.offset(2 as ::core::ffi::c_int as isize);
                }
            }
        }
        if bit_depth == 8 as ::core::ffi::c_int {
            if reductions as ::core::ffi::c_uint & OPNG_REDUCE_RGB_TO_GRAY as ::core::ffi::c_uint
                != 0
            {
                component_ptr = *row_ptr as png_bytep;
                j = 0 as png_uint_32;
                while j < width {
                    if *component_ptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        != *component_ptr.offset(1 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                        || *component_ptr.offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                            != *component_ptr.offset(2 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int
                    {
                        reductions &= !OPNG_REDUCE_RGB_TO_GRAY as ::core::ffi::c_uint;
                        break;
                    } else {
                        j = j.wrapping_add(1);
                        component_ptr = component_ptr.offset(sample_size as isize);
                    }
                }
            }
            if reductions as ::core::ffi::c_uint & OPNG_REDUCE_STRIP_ALPHA as ::core::ffi::c_uint
                != 0
            {
                component_ptr = (*row_ptr).offset(offset_alpha as isize) as png_bytep;
                j = 0 as png_uint_32;
                while j < width {
                    if *component_ptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        != 255 as ::core::ffi::c_int
                    {
                        reductions &= !OPNG_REDUCE_STRIP_ALPHA as ::core::ffi::c_uint;
                        break;
                    } else {
                        j = j.wrapping_add(1);
                        component_ptr = component_ptr.offset(sample_size as isize);
                    }
                }
            }
        } else {
            if reductions as ::core::ffi::c_uint & OPNG_REDUCE_RGB_TO_GRAY as ::core::ffi::c_uint
                != 0
            {
                component_ptr = *row_ptr as png_bytep;
                j = 0 as png_uint_32;
                while j < width {
                    if *component_ptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        != *component_ptr.offset(2 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                        || *component_ptr.offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                            != *component_ptr.offset(4 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int
                        || *component_ptr.offset(1 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                            != *component_ptr.offset(3 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int
                        || *component_ptr.offset(1 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                            != *component_ptr.offset(5 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int
                    {
                        reductions &= !OPNG_REDUCE_RGB_TO_GRAY as ::core::ffi::c_uint;
                        break;
                    } else {
                        j = j.wrapping_add(1);
                        component_ptr = component_ptr.offset(sample_size as isize);
                    }
                }
            }
            if reductions as ::core::ffi::c_uint & OPNG_REDUCE_STRIP_ALPHA as ::core::ffi::c_uint
                != 0
            {
                component_ptr = (*row_ptr).offset(offset_alpha as isize) as png_bytep;
                j = 0 as png_uint_32;
                while j < width {
                    if *component_ptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        != 255 as ::core::ffi::c_int
                        || *component_ptr.offset(1 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                            != 255 as ::core::ffi::c_int
                    {
                        reductions &= !OPNG_REDUCE_STRIP_ALPHA as ::core::ffi::c_uint;
                        break;
                    } else {
                        j = j.wrapping_add(1);
                        component_ptr = component_ptr.offset(sample_size as isize);
                    }
                }
            }
        }
        i = i.wrapping_add(1);
        row_ptr = row_ptr.offset(1);
    }
    return reductions;
}
unsafe extern "C" fn opng_reduce_bits(
    mut png_ptr: png_structp,
    mut info_ptr: png_infop,
    mut reductions: png_uint_32,
) -> png_uint_32 {
    let mut row_ptr: png_bytepp = ::core::ptr::null_mut::<*mut png_byte>();
    let mut src_ptr: png_bytep = ::core::ptr::null_mut::<png_byte>();
    let mut dest_ptr: png_bytep = ::core::ptr::null_mut::<png_byte>();
    let mut width: png_uint_32 = 0;
    let mut height: png_uint_32 = 0;
    let mut interlace_type: ::core::ffi::c_int = 0;
    let mut compression_type: ::core::ffi::c_int = 0;
    let mut filter_type: ::core::ffi::c_int = 0;
    let mut src_bit_depth: ::core::ffi::c_int = 0;
    let mut dest_bit_depth: ::core::ffi::c_int = 0;
    let mut src_byte_depth: ::core::ffi::c_int = 0;
    let mut dest_byte_depth: ::core::ffi::c_int = 0;
    let mut src_color_type: ::core::ffi::c_int = 0;
    let mut dest_color_type: ::core::ffi::c_int = 0;
    let mut src_channels: ::core::ffi::c_int = 0;
    let mut dest_channels: ::core::ffi::c_int = 0;
    let mut src_sample_size: ::core::ffi::c_int = 0;
    let mut dest_sample_size: ::core::ffi::c_int = 0;
    let mut tran_tbl: [::core::ffi::c_int; 8] = [0; 8];
    let mut trans_color: png_color_16p = ::core::ptr::null_mut::<png_color_16>();
    let mut background: png_color_16p = ::core::ptr::null_mut::<png_color_16>();
    let mut sig_bits: png_color_8p = ::core::ptr::null_mut::<png_color_8>();
    let mut i: png_uint_32 = 0;
    let mut j: png_uint_32 = 0;
    let mut k: ::core::ffi::c_int = 0;
    reductions = opng_analyze_bits(png_ptr, info_ptr, reductions);
    if reductions == OPNG_REDUCE_NONE as ::core::ffi::c_uint {
        return OPNG_REDUCE_NONE as png_uint_32;
    }
    png_get_IHDR(
        png_ptr as png_const_structrp,
        info_ptr as png_const_inforp,
        &raw mut width,
        &raw mut height,
        &raw mut src_bit_depth,
        &raw mut src_color_type,
        &raw mut interlace_type,
        &raw mut compression_type,
        &raw mut filter_type,
    );
    '_c2rust_label: {
        if src_bit_depth >= 8 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"src_bit_depth >= 8\0" as *const u8 as *const ::core::ffi::c_char,
                b"opngreduc.c\0" as *const u8 as *const ::core::ffi::c_char,
                506 as ::core::ffi::c_uint,
                b"png_uint_32 opng_reduce_bits(png_structp, png_infop, png_uint_32)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    if reductions as ::core::ffi::c_uint & OPNG_REDUCE_16_TO_8 as ::core::ffi::c_uint != 0 {
        '_c2rust_label_0: {
            if src_bit_depth == 16 as ::core::ffi::c_int {
            } else {
                __assert_fail(
                    b"src_bit_depth == 16\0" as *const u8 as *const ::core::ffi::c_char,
                    b"opngreduc.c\0" as *const u8 as *const ::core::ffi::c_char,
                    509 as ::core::ffi::c_uint,
                    b"png_uint_32 opng_reduce_bits(png_structp, png_infop, png_uint_32)\0"
                        as *const u8 as *const ::core::ffi::c_char,
                );
            }
        };
        dest_bit_depth = 8 as ::core::ffi::c_int;
    } else {
        dest_bit_depth = src_bit_depth;
    }
    src_byte_depth = src_bit_depth / 8 as ::core::ffi::c_int;
    dest_byte_depth = dest_bit_depth / 8 as ::core::ffi::c_int;
    dest_color_type = src_color_type;
    if reductions as ::core::ffi::c_uint & OPNG_REDUCE_RGB_TO_GRAY as ::core::ffi::c_uint != 0 {
        '_c2rust_label_1: {
            if src_color_type & 2 as ::core::ffi::c_int != 0 {
            } else {
                __assert_fail(
                    b"src_color_type & 2\0" as *const u8 as *const ::core::ffi::c_char,
                    b"opngreduc.c\0" as *const u8 as *const ::core::ffi::c_char,
                    521 as ::core::ffi::c_uint,
                    b"png_uint_32 opng_reduce_bits(png_structp, png_infop, png_uint_32)\0"
                        as *const u8 as *const ::core::ffi::c_char,
                );
            }
        };
        dest_color_type &= !PNG_COLOR_MASK_COLOR;
    }
    if reductions as ::core::ffi::c_uint & OPNG_REDUCE_STRIP_ALPHA as ::core::ffi::c_uint != 0 {
        '_c2rust_label_2: {
            if src_color_type & 4 as ::core::ffi::c_int != 0 {
            } else {
                __assert_fail(
                    b"src_color_type & 4\0" as *const u8 as *const ::core::ffi::c_char,
                    b"opngreduc.c\0" as *const u8 as *const ::core::ffi::c_char,
                    526 as ::core::ffi::c_uint,
                    b"png_uint_32 opng_reduce_bits(png_structp, png_infop, png_uint_32)\0"
                        as *const u8 as *const ::core::ffi::c_char,
                );
            }
        };
        dest_color_type &= !PNG_COLOR_MASK_ALPHA;
    }
    src_channels = png_get_channels(png_ptr as png_const_structrp, info_ptr as png_const_inforp)
        as ::core::ffi::c_int;
    dest_channels = (if dest_color_type & PNG_COLOR_MASK_COLOR != 0 {
        3 as ::core::ffi::c_int
    } else {
        1 as ::core::ffi::c_int
    }) + (if dest_color_type & PNG_COLOR_MASK_ALPHA != 0 {
        1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    });
    src_sample_size = src_channels * src_byte_depth;
    dest_sample_size = dest_channels * dest_byte_depth;
    k = 0 as ::core::ffi::c_int;
    while k < 4 as ::core::ffi::c_int * dest_byte_depth {
        tran_tbl[k as usize] = k * src_bit_depth / dest_bit_depth;
        k += 1;
    }
    if reductions as ::core::ffi::c_uint & OPNG_REDUCE_RGB_TO_GRAY as ::core::ffi::c_uint != 0
        && dest_color_type & PNG_COLOR_MASK_ALPHA != 0
    {
        tran_tbl[dest_byte_depth as usize] =
            tran_tbl[(3 as ::core::ffi::c_int * dest_byte_depth) as usize];
        if dest_byte_depth == 2 as ::core::ffi::c_int {
            tran_tbl[(dest_byte_depth + 1 as ::core::ffi::c_int) as usize] = tran_tbl
                [(3 as ::core::ffi::c_int * dest_byte_depth + 1 as ::core::ffi::c_int) as usize];
        }
    }
    '_c2rust_label_3: {
        if src_sample_size > dest_sample_size {
        } else {
            __assert_fail(
                b"src_sample_size > dest_sample_size\0" as *const u8 as *const ::core::ffi::c_char,
                b"opngreduc.c\0" as *const u8 as *const ::core::ffi::c_char,
                551 as ::core::ffi::c_uint,
                b"png_uint_32 opng_reduce_bits(png_structp, png_infop, png_uint_32)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    row_ptr = png_get_rows(png_ptr as png_const_structrp, info_ptr as png_const_inforp);
    i = 0 as png_uint_32;
    while i < height {
        dest_ptr = *row_ptr as png_bytep;
        src_ptr = dest_ptr;
        j = 0 as png_uint_32;
        while j < width {
            k = 0 as ::core::ffi::c_int;
            while k < dest_sample_size {
                *dest_ptr.offset(k as isize) = *src_ptr.offset(tran_tbl[k as usize] as isize);
                k += 1;
            }
            src_ptr = src_ptr.offset(src_sample_size as isize);
            dest_ptr = dest_ptr.offset(dest_sample_size as isize);
            j = j.wrapping_add(1);
        }
        i = i.wrapping_add(1);
        row_ptr = row_ptr.offset(1);
    }
    if png_get_tRNS(
        png_ptr as png_const_structrp,
        info_ptr as png_inforp,
        ::core::ptr::null_mut::<png_bytep>(),
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
        &raw mut trans_color,
    ) != 0
    {
        if reductions as ::core::ffi::c_uint & OPNG_REDUCE_16_TO_8 as ::core::ffi::c_uint != 0 {
            if (*trans_color).red as ::core::ffi::c_int % 257 as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
                && (*trans_color).green as ::core::ffi::c_int % 257 as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
                && (*trans_color).blue as ::core::ffi::c_int % 257 as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
                && (*trans_color).gray as ::core::ffi::c_int % 257 as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
            {
                (*trans_color).red = ((*trans_color).red as ::core::ffi::c_int
                    & 255 as ::core::ffi::c_int)
                    as png_uint_16;
                (*trans_color).green = ((*trans_color).green as ::core::ffi::c_int
                    & 255 as ::core::ffi::c_int)
                    as png_uint_16;
                (*trans_color).blue = ((*trans_color).blue as ::core::ffi::c_int
                    & 255 as ::core::ffi::c_int)
                    as png_uint_16;
                (*trans_color).gray = ((*trans_color).gray as ::core::ffi::c_int
                    & 255 as ::core::ffi::c_int)
                    as png_uint_16;
            } else {
                png_free_data(
                    png_ptr as png_const_structrp,
                    info_ptr as png_inforp,
                    PNG_FREE_TRNS,
                    -(1 as ::core::ffi::c_int),
                );
                png_set_invalid(
                    png_ptr as png_const_structrp,
                    info_ptr as png_inforp,
                    PNG_INFO_tRNS as ::core::ffi::c_int,
                );
            }
        }
        if reductions as ::core::ffi::c_uint & OPNG_REDUCE_RGB_TO_GRAY as ::core::ffi::c_uint != 0 {
            if (*trans_color).red as ::core::ffi::c_int
                == (*trans_color).green as ::core::ffi::c_int
                || (*trans_color).red as ::core::ffi::c_int
                    == (*trans_color).blue as ::core::ffi::c_int
            {
                (*trans_color).gray = (*trans_color).red;
            } else {
                png_free_data(
                    png_ptr as png_const_structrp,
                    info_ptr as png_inforp,
                    PNG_FREE_TRNS,
                    -(1 as ::core::ffi::c_int),
                );
                png_set_invalid(
                    png_ptr as png_const_structrp,
                    info_ptr as png_inforp,
                    PNG_INFO_tRNS as ::core::ffi::c_int,
                );
            }
        }
    }
    if png_get_bKGD(
        png_ptr as png_const_structrp,
        info_ptr as png_inforp,
        &raw mut background,
    ) != 0
    {
        if reductions as ::core::ffi::c_uint & OPNG_REDUCE_16_TO_8 as ::core::ffi::c_uint != 0 {
            (*background).red = ((*background).red as ::core::ffi::c_int
                & 255 as ::core::ffi::c_int) as png_uint_16;
            (*background).green = ((*background).green as ::core::ffi::c_int
                & 255 as ::core::ffi::c_int) as png_uint_16;
            (*background).blue = ((*background).blue as ::core::ffi::c_int
                & 255 as ::core::ffi::c_int) as png_uint_16;
            (*background).gray = ((*background).gray as ::core::ffi::c_int
                & 255 as ::core::ffi::c_int) as png_uint_16;
        }
        if reductions as ::core::ffi::c_uint & OPNG_REDUCE_RGB_TO_GRAY as ::core::ffi::c_uint != 0 {
            (*background).gray = (*background).red;
        }
    }
    if png_get_sBIT(
        png_ptr as png_const_structrp,
        info_ptr as png_inforp,
        &raw mut sig_bits,
    ) != 0
    {
        if reductions as ::core::ffi::c_uint & OPNG_REDUCE_16_TO_8 as ::core::ffi::c_uint != 0 {
            if (*sig_bits).red as ::core::ffi::c_int > 8 as ::core::ffi::c_int {
                (*sig_bits).red = 8 as png_byte;
            }
            if (*sig_bits).green as ::core::ffi::c_int > 8 as ::core::ffi::c_int {
                (*sig_bits).green = 8 as png_byte;
            }
            if (*sig_bits).blue as ::core::ffi::c_int > 8 as ::core::ffi::c_int {
                (*sig_bits).blue = 8 as png_byte;
            }
            if (*sig_bits).gray as ::core::ffi::c_int > 8 as ::core::ffi::c_int {
                (*sig_bits).gray = 8 as png_byte;
            }
            if (*sig_bits).alpha as ::core::ffi::c_int > 8 as ::core::ffi::c_int {
                (*sig_bits).alpha = 8 as png_byte;
            }
        }
        if reductions as ::core::ffi::c_uint & OPNG_REDUCE_RGB_TO_GRAY as ::core::ffi::c_uint != 0 {
            let mut max_sig_bits: png_byte = (*sig_bits).red;
            if (max_sig_bits as ::core::ffi::c_int) < (*sig_bits).green as ::core::ffi::c_int {
                max_sig_bits = (*sig_bits).green;
            }
            if (max_sig_bits as ::core::ffi::c_int) < (*sig_bits).blue as ::core::ffi::c_int {
                max_sig_bits = (*sig_bits).blue;
            }
            (*sig_bits).gray = max_sig_bits;
        }
    }
    png_set_IHDR(
        png_ptr as png_const_structrp,
        info_ptr as png_inforp,
        width,
        height,
        dest_bit_depth,
        dest_color_type,
        interlace_type,
        compression_type,
        filter_type,
    );
    return reductions;
}
unsafe extern "C" fn opng_reduce_palette_bits(
    mut png_ptr: png_structp,
    mut info_ptr: png_infop,
    mut reductions: png_uint_32,
) -> png_uint_32 {
    let mut row_ptr: png_bytepp = ::core::ptr::null_mut::<*mut png_byte>();
    let mut src_sample_ptr: png_bytep = ::core::ptr::null_mut::<png_byte>();
    let mut dest_sample_ptr: png_bytep = ::core::ptr::null_mut::<png_byte>();
    let mut width: png_uint_32 = 0;
    let mut height: png_uint_32 = 0;
    let mut color_type: ::core::ffi::c_int = 0;
    let mut interlace_type: ::core::ffi::c_int = 0;
    let mut compression_type: ::core::ffi::c_int = 0;
    let mut filter_type: ::core::ffi::c_int = 0;
    let mut src_bit_depth: ::core::ffi::c_int = 0;
    let mut dest_bit_depth: ::core::ffi::c_int = 0;
    let mut src_mask_init: ::core::ffi::c_uint = 0;
    let mut src_mask: ::core::ffi::c_uint = 0;
    let mut src_shift: ::core::ffi::c_uint = 0;
    let mut dest_shift: ::core::ffi::c_uint = 0;
    let mut sample: ::core::ffi::c_uint = 0;
    let mut dest_buf: ::core::ffi::c_uint = 0;
    let mut palette: png_colorp = ::core::ptr::null_mut::<png_color>();
    let mut num_palette: ::core::ffi::c_int = 0;
    let mut i: png_uint_32 = 0;
    let mut j: png_uint_32 = 0;
    if reductions as ::core::ffi::c_uint & OPNG_REDUCE_8_TO_4_2_1 as ::core::ffi::c_uint == 0 {
        return OPNG_REDUCE_NONE as png_uint_32;
    }
    png_get_IHDR(
        png_ptr as png_const_structrp,
        info_ptr as png_const_inforp,
        &raw mut width,
        &raw mut height,
        &raw mut src_bit_depth,
        &raw mut color_type,
        &raw mut interlace_type,
        &raw mut compression_type,
        &raw mut filter_type,
    );
    if color_type != PNG_COLOR_TYPE_PALETTE {
        return OPNG_REDUCE_NONE as png_uint_32;
    }
    if png_get_PLTE(
        png_ptr as png_const_structrp,
        info_ptr as png_inforp,
        &raw mut palette,
        &raw mut num_palette,
    ) == 0
    {
        num_palette = 0 as ::core::ffi::c_int;
    }
    if num_palette > 16 as ::core::ffi::c_int {
        return OPNG_REDUCE_NONE as png_uint_32;
    } else if num_palette > 4 as ::core::ffi::c_int {
        dest_bit_depth = 4 as ::core::ffi::c_int;
    } else if num_palette > 2 as ::core::ffi::c_int {
        dest_bit_depth = 2 as ::core::ffi::c_int;
    } else {
        '_c2rust_label: {
            if num_palette > 0 as ::core::ffi::c_int {
            } else {
                __assert_fail(
                    b"num_palette > 0\0" as *const u8 as *const ::core::ffi::c_char,
                    b"opngreduc.c\0" as *const u8 as *const ::core::ffi::c_char,
                    691 as ::core::ffi::c_uint,
                    b"png_uint_32 opng_reduce_palette_bits(png_structp, png_infop, png_uint_32)\0"
                        as *const u8 as *const ::core::ffi::c_char,
                );
            }
        };
        dest_bit_depth = 1 as ::core::ffi::c_int;
    }
    if src_bit_depth <= dest_bit_depth {
        '_c2rust_label_0: {
            if src_bit_depth == dest_bit_depth {
            } else {
                __assert_fail(
                    b"src_bit_depth == dest_bit_depth\0" as *const u8 as *const ::core::ffi::c_char,
                    b"opngreduc.c\0" as *const u8 as *const ::core::ffi::c_char,
                    697 as ::core::ffi::c_uint,
                    b"png_uint_32 opng_reduce_palette_bits(png_structp, png_infop, png_uint_32)\0"
                        as *const u8 as *const ::core::ffi::c_char,
                );
            }
        };
        return OPNG_REDUCE_NONE as png_uint_32;
    }
    row_ptr = png_get_rows(png_ptr as png_const_structrp, info_ptr as png_const_inforp);
    if src_bit_depth == 8 as ::core::ffi::c_int {
        i = 0 as png_uint_32;
        while i < height {
            dest_sample_ptr = *row_ptr as png_bytep;
            src_sample_ptr = dest_sample_ptr;
            dest_shift = 8 as ::core::ffi::c_uint;
            dest_buf = 0 as ::core::ffi::c_uint;
            j = 0 as png_uint_32;
            while j < width {
                dest_shift = dest_shift.wrapping_sub(dest_bit_depth as ::core::ffi::c_uint);
                if dest_shift > 0 as ::core::ffi::c_uint {
                    dest_buf |= ((*src_sample_ptr as ::core::ffi::c_int) << dest_shift)
                        as ::core::ffi::c_uint;
                } else {
                    let fresh0 = dest_sample_ptr;
                    dest_sample_ptr = dest_sample_ptr.offset(1);
                    *fresh0 = (dest_buf | *src_sample_ptr as ::core::ffi::c_uint) as png_byte;
                    dest_shift = 8 as ::core::ffi::c_uint;
                    dest_buf = 0 as ::core::ffi::c_uint;
                }
                src_sample_ptr = src_sample_ptr.offset(1);
                j = j.wrapping_add(1);
            }
            if dest_shift != 0 as ::core::ffi::c_uint {
                *dest_sample_ptr = dest_buf as png_byte;
            }
            i = i.wrapping_add(1);
            row_ptr = row_ptr.offset(1);
        }
    } else {
        src_mask_init = (((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int + src_bit_depth)
            - ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int))
            as ::core::ffi::c_uint;
        i = 0 as png_uint_32;
        while i < height {
            dest_sample_ptr = *row_ptr as png_bytep;
            src_sample_ptr = dest_sample_ptr;
            dest_shift = 8 as ::core::ffi::c_uint;
            src_shift = dest_shift;
            src_mask = src_mask_init;
            dest_buf = 0 as ::core::ffi::c_uint;
            j = 0 as png_uint_32;
            while j < width {
                src_shift = src_shift.wrapping_sub(src_bit_depth as ::core::ffi::c_uint);
                src_mask >>= src_bit_depth;
                sample = (*src_sample_ptr as ::core::ffi::c_uint & src_mask) >> src_shift;
                dest_shift = dest_shift.wrapping_sub(dest_bit_depth as ::core::ffi::c_uint);
                if dest_shift > 0 as ::core::ffi::c_uint {
                    dest_buf |= sample << dest_shift;
                } else {
                    let fresh1 = dest_sample_ptr;
                    dest_sample_ptr = dest_sample_ptr.offset(1);
                    *fresh1 = (dest_buf | sample) as png_byte;
                    dest_shift = 8 as ::core::ffi::c_uint;
                    dest_buf = 0 as ::core::ffi::c_uint;
                }
                if src_shift == 0 as ::core::ffi::c_uint {
                    src_shift = 8 as ::core::ffi::c_uint;
                    src_mask = src_mask_init;
                    src_sample_ptr = src_sample_ptr.offset(1);
                }
                j = j.wrapping_add(1);
            }
            if dest_shift != 0 as ::core::ffi::c_uint {
                *dest_sample_ptr = dest_buf as png_byte;
            }
            i = i.wrapping_add(1);
            row_ptr = row_ptr.offset(1);
        }
    }
    png_set_IHDR(
        png_ptr as png_const_structrp,
        info_ptr as png_inforp,
        width,
        height,
        dest_bit_depth,
        color_type,
        interlace_type,
        compression_type,
        filter_type,
    );
    return OPNG_REDUCE_8_TO_4_2_1 as png_uint_32;
}
unsafe extern "C" fn opng_reduce_to_palette(
    mut png_ptr: png_structp,
    mut info_ptr: png_infop,
    mut reductions: png_uint_32,
) -> png_uint_32 {
    let mut result: png_uint_32 = 0;
    let mut row_info: png_row_info = png_row_info {
        width: 0,
        rowbytes: 0,
        color_type: 0,
        bit_depth: 0,
        channels: 0,
        pixel_depth: 0,
    };
    let mut row_ptr: png_bytepp = ::core::ptr::null_mut::<*mut png_byte>();
    let mut sample_ptr: png_bytep = ::core::ptr::null_mut::<png_byte>();
    let mut alpha_row: png_bytep = ::core::ptr::null_mut::<png_byte>();
    let mut height: png_uint_32 = 0;
    let mut width: png_uint_32 = 0;
    let mut color_type: ::core::ffi::c_int = 0;
    let mut interlace_type: ::core::ffi::c_int = 0;
    let mut compression_type: ::core::ffi::c_int = 0;
    let mut filter_type: ::core::ffi::c_int = 0;
    let mut src_bit_depth: ::core::ffi::c_int = 0;
    let mut dest_bit_depth: ::core::ffi::c_int = 0;
    let mut channels: ::core::ffi::c_int = 0;
    let mut palette: [png_color; 256] = [png_color {
        red: 0,
        green: 0,
        blue: 0,
    }; 256];
    let mut trans_alpha: [png_byte; 256] = [0; 256];
    let mut trans_color: png_color_16p = ::core::ptr::null_mut::<png_color_16>();
    let mut num_palette: ::core::ffi::c_int = 0;
    let mut num_trans: ::core::ffi::c_int = 0;
    let mut index: ::core::ffi::c_int = 0;
    let mut gray: ::core::ffi::c_uint = 0;
    let mut red: ::core::ffi::c_uint = 0;
    let mut green: ::core::ffi::c_uint = 0;
    let mut blue: ::core::ffi::c_uint = 0;
    let mut alpha: ::core::ffi::c_uint = 0;
    let mut prev_gray: ::core::ffi::c_uint = 0;
    let mut prev_red: ::core::ffi::c_uint = 0;
    let mut prev_green: ::core::ffi::c_uint = 0;
    let mut prev_blue: ::core::ffi::c_uint = 0;
    let mut prev_alpha: ::core::ffi::c_uint = 0;
    let mut background: png_color_16p = ::core::ptr::null_mut::<png_color_16>();
    let mut i: png_uint_32 = 0;
    let mut j: png_uint_32 = 0;
    png_get_IHDR(
        png_ptr as png_const_structrp,
        info_ptr as png_const_inforp,
        &raw mut width,
        &raw mut height,
        &raw mut src_bit_depth,
        &raw mut color_type,
        &raw mut interlace_type,
        &raw mut compression_type,
        &raw mut filter_type,
    );
    if src_bit_depth != 8 as ::core::ffi::c_int {
        return OPNG_REDUCE_NONE as png_uint_32;
    }
    '_c2rust_label: {
        if color_type & 1 as ::core::ffi::c_int == 0 {
        } else {
            __assert_fail(
                b"!(color_type & 1)\0" as *const u8 as *const ::core::ffi::c_char,
                b"opngreduc.c\0" as *const u8 as *const ::core::ffi::c_char,
                802 as ::core::ffi::c_uint,
                b"png_uint_32 opng_reduce_to_palette(png_structp, png_infop, png_uint_32)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    row_ptr = png_get_rows(png_ptr as png_const_structrp, info_ptr as png_const_inforp);
    channels = png_get_channels(png_ptr as png_const_structrp, info_ptr as png_const_inforp)
        as ::core::ffi::c_int;
    alpha_row = png_malloc(png_ptr as png_const_structrp, width as png_alloc_size_t) as png_bytep;
    row_info.width = width;
    row_info.rowbytes = 0 as png_size_t;
    row_info.color_type = color_type as png_byte;
    row_info.bit_depth = src_bit_depth as png_byte;
    row_info.channels = channels as png_byte;
    row_info.pixel_depth = 0 as png_byte;
    num_trans = 0 as ::core::ffi::c_int;
    num_palette = num_trans;
    trans_color = ::core::ptr::null_mut::<png_color_16>();
    png_get_tRNS(
        png_ptr as png_const_structrp,
        info_ptr as png_inforp,
        ::core::ptr::null_mut::<png_bytep>(),
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
        &raw mut trans_color,
    );
    prev_alpha = 256 as ::core::ffi::c_uint;
    prev_blue = prev_alpha;
    prev_green = prev_blue;
    prev_red = prev_green;
    prev_gray = prev_red;
    i = 0 as png_uint_32;
    while i < height {
        sample_ptr = *row_ptr as png_bytep;
        opng_get_alpha_row(&raw mut row_info, trans_color, *row_ptr, alpha_row);
        if color_type & PNG_COLOR_MASK_COLOR != 0 {
            j = 0 as png_uint_32;
            while j < width {
                red = *sample_ptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint;
                green = *sample_ptr.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint;
                blue = *sample_ptr.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint;
                alpha = *alpha_row.offset(j as isize) as ::core::ffi::c_uint;
                if red != prev_red
                    || green != prev_green
                    || blue != prev_blue
                    || alpha != prev_alpha
                {
                    prev_red = red;
                    prev_green = green;
                    prev_blue = blue;
                    prev_alpha = alpha;
                    if opng_insert_palette_entry(
                        &raw mut palette as png_colorp,
                        &raw mut num_palette,
                        &raw mut trans_alpha as png_bytep,
                        &raw mut num_trans,
                        256 as ::core::ffi::c_int,
                        red,
                        green,
                        blue,
                        alpha,
                        &raw mut index,
                    ) < 0 as ::core::ffi::c_int
                    {
                        '_c2rust_label_0: {
                            if num_palette < 0 as ::core::ffi::c_int {
                            } else {
                                __assert_fail(
                                    b"num_palette < 0\0" as *const u8
                                        as *const ::core::ffi::c_char,
                                    b"opngreduc.c\0" as *const u8 as *const ::core::ffi::c_char,
                                    844 as ::core::ffi::c_uint,
                                    b"png_uint_32 opng_reduce_to_palette(png_structp, png_infop, png_uint_32)\0"
                                        as *const u8 as *const ::core::ffi::c_char,
                                );
                            }
                        };
                        i = height;
                        break;
                    }
                }
                j = j.wrapping_add(1);
                sample_ptr = sample_ptr.offset(channels as isize);
            }
        } else {
            j = 0 as png_uint_32;
            while j < width {
                gray = *sample_ptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint;
                alpha = *alpha_row.offset(j as isize) as ::core::ffi::c_uint;
                if gray != prev_gray || alpha != prev_alpha {
                    prev_gray = gray;
                    prev_alpha = alpha;
                    if opng_insert_palette_entry(
                        &raw mut palette as png_colorp,
                        &raw mut num_palette,
                        &raw mut trans_alpha as png_bytep,
                        &raw mut num_trans,
                        256 as ::core::ffi::c_int,
                        gray,
                        gray,
                        gray,
                        alpha,
                        &raw mut index,
                    ) < 0 as ::core::ffi::c_int
                    {
                        '_c2rust_label_1: {
                            if num_palette < 0 as ::core::ffi::c_int {
                            } else {
                                __assert_fail(
                                    b"num_palette < 0\0" as *const u8
                                        as *const ::core::ffi::c_char,
                                    b"opngreduc.c\0" as *const u8 as *const ::core::ffi::c_char,
                                    866 as ::core::ffi::c_uint,
                                    b"png_uint_32 opng_reduce_to_palette(png_structp, png_infop, png_uint_32)\0"
                                        as *const u8 as *const ::core::ffi::c_char,
                                );
                            }
                        };
                        i = height;
                        break;
                    }
                }
                j = j.wrapping_add(1);
                sample_ptr = sample_ptr.offset(channels as isize);
            }
        }
        i = i.wrapping_add(1);
        row_ptr = row_ptr.offset(1);
    }
    if num_palette >= 0 as ::core::ffi::c_int
        && png_get_bKGD(
            png_ptr as png_const_structrp,
            info_ptr as png_inforp,
            &raw mut background,
        ) != 0
    {
        if color_type & PNG_COLOR_MASK_COLOR != 0 {
            red = (*background).red as ::core::ffi::c_uint;
            green = (*background).green as ::core::ffi::c_uint;
            blue = (*background).blue as ::core::ffi::c_uint;
        } else {
            blue = (*background).gray as ::core::ffi::c_uint;
            green = blue;
            red = green;
        }
        opng_insert_palette_entry(
            &raw mut palette as png_colorp,
            &raw mut num_palette,
            &raw mut trans_alpha as png_bytep,
            &raw mut num_trans,
            256 as ::core::ffi::c_int,
            red,
            green,
            blue,
            256 as ::core::ffi::c_uint,
            &raw mut index,
        );
        if index >= 0 as ::core::ffi::c_int {
            (*background).index = index as png_byte;
        }
    }
    if num_palette >= 0 as ::core::ffi::c_int {
        '_c2rust_label_2: {
            if num_palette > 0 as ::core::ffi::c_int && num_palette <= 256 as ::core::ffi::c_int {
            } else {
                __assert_fail(
                    b"num_palette > 0 && num_palette <= 256\0" as *const u8
                        as *const ::core::ffi::c_char,
                    b"opngreduc.c\0" as *const u8 as *const ::core::ffi::c_char,
                    905 as ::core::ffi::c_uint,
                    b"png_uint_32 opng_reduce_to_palette(png_structp, png_infop, png_uint_32)\0"
                        as *const u8 as *const ::core::ffi::c_char,
                );
            }
        };
        '_c2rust_label_3: {
            if num_trans >= 0 as ::core::ffi::c_int && num_trans <= num_palette {
            } else {
                __assert_fail(
                    b"num_trans >= 0 && num_trans <= num_palette\0" as *const u8
                        as *const ::core::ffi::c_char,
                    b"opngreduc.c\0" as *const u8 as *const ::core::ffi::c_char,
                    906 as ::core::ffi::c_uint,
                    b"png_uint_32 opng_reduce_to_palette(png_structp, png_infop, png_uint_32)\0"
                        as *const u8 as *const ::core::ffi::c_char,
                );
            }
        };
        if num_palette <= 2 as ::core::ffi::c_int {
            dest_bit_depth = 1 as ::core::ffi::c_int;
        } else if num_palette <= 4 as ::core::ffi::c_int {
            dest_bit_depth = 2 as ::core::ffi::c_int;
        } else if num_palette <= 16 as ::core::ffi::c_int {
            dest_bit_depth = 4 as ::core::ffi::c_int;
        } else {
            dest_bit_depth = 8 as ::core::ffi::c_int;
        }
        if channels * 8 as ::core::ffi::c_int == dest_bit_depth
            || (((3 as ::core::ffi::c_int * num_palette + num_trans) * 8 as ::core::ffi::c_int
                / (channels * 8 as ::core::ffi::c_int - dest_bit_depth))
                as png_uint_32)
                .wrapping_div(width)
                .wrapping_div(height)
                >= 1 as ::core::ffi::c_uint
        {
            num_palette = -(1 as ::core::ffi::c_int);
        }
    }
    if num_palette < 0 as ::core::ffi::c_int {
        png_free(png_ptr as png_const_structrp, alpha_row as png_voidp);
        return OPNG_REDUCE_NONE as png_uint_32;
    }
    row_ptr = png_get_rows(png_ptr as png_const_structrp, info_ptr as png_const_inforp);
    index = -(1 as ::core::ffi::c_int);
    prev_alpha = -(1 as ::core::ffi::c_int) as ::core::ffi::c_uint;
    prev_blue = prev_alpha;
    prev_green = prev_blue;
    prev_red = prev_green;
    i = 0 as png_uint_32;
    while i < height {
        sample_ptr = *row_ptr as png_bytep;
        opng_get_alpha_row(&raw mut row_info, trans_color, *row_ptr, alpha_row);
        if color_type & PNG_COLOR_MASK_COLOR != 0 {
            j = 0 as png_uint_32;
            while j < width {
                red = *sample_ptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint;
                green = *sample_ptr.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint;
                blue = *sample_ptr.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint;
                alpha = *alpha_row.offset(j as isize) as ::core::ffi::c_uint;
                if red != prev_red
                    || green != prev_green
                    || blue != prev_blue
                    || alpha != prev_alpha
                {
                    prev_red = red;
                    prev_green = green;
                    prev_blue = blue;
                    prev_alpha = alpha;
                    if opng_insert_palette_entry(
                        &raw mut palette as png_colorp,
                        &raw mut num_palette,
                        &raw mut trans_alpha as png_bytep,
                        &raw mut num_trans,
                        256 as ::core::ffi::c_int,
                        red,
                        green,
                        blue,
                        alpha,
                        &raw mut index,
                    ) != 0 as ::core::ffi::c_int
                    {
                        index = -(1 as ::core::ffi::c_int);
                    }
                }
                '_c2rust_label_4: {
                    if index >= 0 as ::core::ffi::c_int {
                    } else {
                        __assert_fail(
                            b"index >= 0\0" as *const u8 as *const ::core::ffi::c_char,
                            b"opngreduc.c\0" as *const u8 as *const ::core::ffi::c_char,
                            957 as ::core::ffi::c_uint,
                            b"png_uint_32 opng_reduce_to_palette(png_structp, png_infop, png_uint_32)\0"
                                as *const u8 as *const ::core::ffi::c_char,
                        );
                    }
                };
                *(*row_ptr).offset(j as isize) = index as png_byte;
                j = j.wrapping_add(1);
                sample_ptr = sample_ptr.offset(channels as isize);
            }
        } else {
            j = 0 as png_uint_32;
            while j < width {
                gray = *sample_ptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint;
                alpha = *alpha_row.offset(j as isize) as ::core::ffi::c_uint;
                if gray != prev_gray || alpha != prev_alpha {
                    prev_gray = gray;
                    prev_alpha = alpha;
                    if opng_insert_palette_entry(
                        &raw mut palette as png_colorp,
                        &raw mut num_palette,
                        &raw mut trans_alpha as png_bytep,
                        &raw mut num_trans,
                        256 as ::core::ffi::c_int,
                        gray,
                        gray,
                        gray,
                        alpha,
                        &raw mut index,
                    ) != 0 as ::core::ffi::c_int
                    {
                        index = -(1 as ::core::ffi::c_int);
                    }
                }
                '_c2rust_label_5: {
                    if index >= 0 as ::core::ffi::c_int {
                    } else {
                        __assert_fail(
                            b"index >= 0\0" as *const u8 as *const ::core::ffi::c_char,
                            b"opngreduc.c\0" as *const u8 as *const ::core::ffi::c_char,
                            977 as ::core::ffi::c_uint,
                            b"png_uint_32 opng_reduce_to_palette(png_structp, png_infop, png_uint_32)\0"
                                as *const u8 as *const ::core::ffi::c_char,
                        );
                    }
                };
                *(*row_ptr).offset(j as isize) = index as png_byte;
                j = j.wrapping_add(1);
                sample_ptr = sample_ptr.offset(channels as isize);
            }
        }
        i = i.wrapping_add(1);
        row_ptr = row_ptr.offset(1);
    }
    png_set_IHDR(
        png_ptr as png_const_structrp,
        info_ptr as png_inforp,
        width,
        height,
        8 as ::core::ffi::c_int,
        PNG_COLOR_TYPE_PALETTE,
        interlace_type,
        compression_type,
        filter_type,
    );
    png_set_PLTE(
        png_ptr as png_structrp,
        info_ptr as png_inforp,
        &raw mut palette as *mut png_color as png_const_colorp,
        num_palette,
    );
    if num_trans > 0 as ::core::ffi::c_int {
        png_set_tRNS(
            png_ptr as png_structrp,
            info_ptr as png_inforp,
            &raw mut trans_alpha as *mut png_byte as png_const_bytep,
            num_trans,
            ::core::ptr::null::<png_color_16>(),
        );
    }
    png_free(png_ptr as png_const_structrp, alpha_row as png_voidp);
    result = OPNG_REDUCE_RGB_TO_PALETTE as png_uint_32;
    if reductions as ::core::ffi::c_uint & OPNG_REDUCE_8_TO_4_2_1 as ::core::ffi::c_uint != 0 {
        result |= opng_reduce_palette_bits(png_ptr, info_ptr, reductions) as ::core::ffi::c_uint;
    }
    return result;
}
unsafe extern "C" fn opng_analyze_sample_usage(
    mut png_ptr: png_structp,
    mut info_ptr: png_infop,
    mut usage_map: png_bytep,
) {
    let mut row_ptr: png_bytepp = ::core::ptr::null_mut::<*mut png_byte>();
    let mut sample_ptr: png_bytep = ::core::ptr::null_mut::<png_byte>();
    let mut width: png_uint_32 = 0;
    let mut height: png_uint_32 = 0;
    let mut bit_depth: ::core::ffi::c_int = 0;
    let mut init_shift: ::core::ffi::c_int = 0;
    let mut init_mask: ::core::ffi::c_int = 0;
    let mut shift: ::core::ffi::c_int = 0;
    let mut mask: ::core::ffi::c_int = 0;
    let mut background: png_color_16p = ::core::ptr::null_mut::<png_color_16>();
    let mut i: png_uint_32 = 0;
    let mut j: png_uint_32 = 0;
    height = png_get_image_height(png_ptr as png_const_structrp, info_ptr as png_const_inforp);
    width = png_get_image_width(png_ptr as png_const_structrp, info_ptr as png_const_inforp);
    bit_depth = png_get_bit_depth(png_ptr as png_const_structrp, info_ptr as png_const_inforp)
        as ::core::ffi::c_int;
    row_ptr = png_get_rows(png_ptr as png_const_structrp, info_ptr as png_const_inforp);
    memset(
        usage_map as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        256 as size_t,
    );
    if bit_depth == 8 as ::core::ffi::c_int {
        i = 0 as png_uint_32;
        while i < height {
            j = 0 as png_uint_32;
            sample_ptr = *row_ptr as png_bytep;
            while j < width {
                *usage_map.offset(*sample_ptr as isize) = 1 as png_byte;
                j = j.wrapping_add(1);
                sample_ptr = sample_ptr.offset(1);
            }
            i = i.wrapping_add(1);
            row_ptr = row_ptr.offset(1);
        }
    } else {
        '_c2rust_label: {
            if bit_depth < 8 as ::core::ffi::c_int {
            } else {
                __assert_fail(
                    b"bit_depth < 8\0" as *const u8 as *const ::core::ffi::c_char,
                    b"opngreduc.c\0" as *const u8 as *const ::core::ffi::c_char,
                    1039 as ::core::ffi::c_uint,
                    b"void opng_analyze_sample_usage(png_structp, png_infop, png_bytep)\0"
                        as *const u8 as *const ::core::ffi::c_char,
                );
            }
        };
        init_shift = 8 as ::core::ffi::c_int - bit_depth;
        init_mask = ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)
            - ((1 as ::core::ffi::c_int) << init_shift);
        i = 0 as png_uint_32;
        while i < height {
            j = 0 as png_uint_32;
            sample_ptr = *row_ptr as png_bytep;
            while j < width {
                mask = init_mask;
                shift = init_shift;
                loop {
                    *usage_map
                        .offset(((*sample_ptr as ::core::ffi::c_int & mask) >> shift) as isize) =
                        1 as png_byte;
                    mask >>= bit_depth;
                    shift -= bit_depth;
                    j = j.wrapping_add(1);
                    if !(mask > 0 as ::core::ffi::c_int && j < width) {
                        break;
                    }
                }
                sample_ptr = sample_ptr.offset(1);
            }
            i = i.wrapping_add(1);
            row_ptr = row_ptr.offset(1);
        }
    }
    if png_get_bKGD(
        png_ptr as png_const_structrp,
        info_ptr as png_inforp,
        &raw mut background,
    ) != 0
    {
        *usage_map.offset((*background).index as isize) = 1 as png_byte;
    }
}
unsafe extern "C" fn opng_reduce_palette(
    mut png_ptr: png_structp,
    mut info_ptr: png_infop,
    mut reductions: png_uint_32,
) -> png_uint_32 {
    let mut result: png_uint_32 = 0;
    let mut palette: png_colorp = ::core::ptr::null_mut::<png_color>();
    let mut trans_alpha: png_bytep = ::core::ptr::null_mut::<png_byte>();
    let mut row_ptr: png_bytepp = ::core::ptr::null_mut::<*mut png_byte>();
    let mut width: png_uint_32 = 0;
    let mut height: png_uint_32 = 0;
    let mut bit_depth: ::core::ffi::c_int = 0;
    let mut color_type: ::core::ffi::c_int = 0;
    let mut interlace_type: ::core::ffi::c_int = 0;
    let mut compression_type: ::core::ffi::c_int = 0;
    let mut filter_type: ::core::ffi::c_int = 0;
    let mut num_palette: ::core::ffi::c_int = 0;
    let mut num_trans: ::core::ffi::c_int = 0;
    let mut last_color_index: ::core::ffi::c_int = 0;
    let mut last_trans_index: ::core::ffi::c_int = 0;
    let mut crt_trans_value: png_byte = 0;
    let mut last_trans_value: png_byte = 0;
    let mut is_used: [png_byte; 256] = [0; 256];
    let mut gray_trans: png_color_16 = png_color_16 {
        index: 0,
        red: 0,
        green: 0,
        blue: 0,
        gray: 0,
    };
    let mut is_gray: ::core::ffi::c_int = 0;
    let mut background: png_color_16p = ::core::ptr::null_mut::<png_color_16>();
    let mut hist: png_uint_16p = ::core::ptr::null_mut::<png_uint_16>();
    let mut sig_bits: png_color_8p = ::core::ptr::null_mut::<png_color_8>();
    let mut i: png_uint_32 = 0;
    let mut j: png_uint_32 = 0;
    let mut k: ::core::ffi::c_int = 0;
    result = OPNG_REDUCE_NONE as png_uint_32;
    png_get_IHDR(
        png_ptr as png_const_structrp,
        info_ptr as png_const_inforp,
        &raw mut width,
        &raw mut height,
        &raw mut bit_depth,
        &raw mut color_type,
        &raw mut interlace_type,
        &raw mut compression_type,
        &raw mut filter_type,
    );
    row_ptr = png_get_rows(png_ptr as png_const_structrp, info_ptr as png_const_inforp);
    if png_get_PLTE(
        png_ptr as png_const_structrp,
        info_ptr as png_inforp,
        &raw mut palette,
        &raw mut num_palette,
    ) == 0
    {
        palette = ::core::ptr::null_mut::<png_color>();
        num_palette = 0 as ::core::ffi::c_int;
    }
    if png_get_tRNS(
        png_ptr as png_const_structrp,
        info_ptr as png_inforp,
        &raw mut trans_alpha,
        &raw mut num_trans,
        ::core::ptr::null_mut::<png_color_16p>(),
    ) == 0
    {
        trans_alpha = ::core::ptr::null_mut::<png_byte>();
        num_trans = 0 as ::core::ffi::c_int;
    } else {
        '_c2rust_label: {
            if !trans_alpha.is_null() && num_trans > 0 as ::core::ffi::c_int {
            } else {
                __assert_fail(
                    b"trans_alpha != ((void*)0) && num_trans > 0\0" as *const u8
                        as *const ::core::ffi::c_char,
                    b"opngreduc.c\0" as *const u8 as *const ::core::ffi::c_char,
                    1117 as ::core::ffi::c_uint,
                    b"png_uint_32 opng_reduce_palette(png_structp, png_infop, png_uint_32)\0"
                        as *const u8 as *const ::core::ffi::c_char,
                );
            }
        };
    }
    opng_analyze_sample_usage(png_ptr, info_ptr, &raw mut is_used as png_bytep);
    is_gray = (reductions as ::core::ffi::c_uint
        & OPNG_REDUCE_PALETTE_TO_GRAY as ::core::ffi::c_uint
        != 0
        && bit_depth == 8 as ::core::ffi::c_int) as ::core::ffi::c_int;
    last_trans_index = -(1 as ::core::ffi::c_int);
    last_color_index = last_trans_index;
    k = 0 as ::core::ffi::c_int;
    while k < 256 as ::core::ffi::c_int {
        if !(is_used[k as usize] == 0) {
            last_color_index = k;
            if k < num_trans
                && (*trans_alpha.offset(k as isize) as ::core::ffi::c_int)
                    < 255 as ::core::ffi::c_int
            {
                last_trans_index = k;
            }
            if is_gray != 0 {
                if (*palette.offset(k as isize)).red as ::core::ffi::c_int
                    != (*palette.offset(k as isize)).green as ::core::ffi::c_int
                    || (*palette.offset(k as isize)).red as ::core::ffi::c_int
                        != (*palette.offset(k as isize)).blue as ::core::ffi::c_int
                {
                    is_gray = 0 as ::core::ffi::c_int;
                }
            }
        }
        k += 1;
    }
    '_c2rust_label_0: {
        if last_color_index >= 0 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"last_color_index >= 0\0" as *const u8 as *const ::core::ffi::c_char,
                b"opngreduc.c\0" as *const u8 as *const ::core::ffi::c_char,
                1135 as ::core::ffi::c_uint,
                b"png_uint_32 opng_reduce_palette(png_structp, png_infop, png_uint_32)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_1: {
        if last_color_index >= last_trans_index {
        } else {
            __assert_fail(
                b"last_color_index >= last_trans_index\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"opngreduc.c\0" as *const u8 as *const ::core::ffi::c_char,
                1136 as ::core::ffi::c_uint,
                b"png_uint_32 opng_reduce_palette(png_structp, png_infop, png_uint_32)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    if last_color_index >= num_palette {
        png_warning(
            png_ptr as png_const_structrp,
            b"Too few colors in PLTE\0" as *const u8 as png_const_charp,
        );
        opng_realloc_PLTE(
            png_ptr,
            info_ptr,
            last_color_index + 1 as ::core::ffi::c_int,
        );
        png_get_PLTE(
            png_ptr as png_const_structrp,
            info_ptr as png_inforp,
            &raw mut palette,
            &raw mut num_palette,
        );
        '_c2rust_label_2: {
            if num_palette == last_color_index + 1 as ::core::ffi::c_int {
            } else {
                __assert_fail(
                    b"num_palette == last_color_index + 1\0" as *const u8
                        as *const ::core::ffi::c_char,
                    b"opngreduc.c\0" as *const u8 as *const ::core::ffi::c_char,
                    1145 as ::core::ffi::c_uint,
                    b"png_uint_32 opng_reduce_palette(png_structp, png_infop, png_uint_32)\0"
                        as *const u8 as *const ::core::ffi::c_char,
                );
            }
        };
        result |= OPNG_REDUCE_REPAIR as ::core::ffi::c_uint;
    }
    if num_trans > num_palette {
        png_warning(
            png_ptr as png_const_structrp,
            b"Too many alpha values in tRNS\0" as *const u8 as png_const_charp,
        );
        result |= OPNG_REDUCE_REPAIR as ::core::ffi::c_uint;
    }
    if is_gray != 0 && last_trans_index >= 0 as ::core::ffi::c_int {
        gray_trans.gray = (*palette.offset(last_trans_index as isize)).red as png_uint_16;
        last_trans_value = *trans_alpha.offset(last_trans_index as isize);
        k = 0 as ::core::ffi::c_int;
        while k <= last_color_index {
            if !(is_used[k as usize] == 0) {
                if k <= last_trans_index {
                    crt_trans_value = *trans_alpha.offset(k as isize);
                    if (crt_trans_value as ::core::ffi::c_int) < 255 as ::core::ffi::c_int
                        && (*palette.offset(k as isize)).red as ::core::ffi::c_int
                            != gray_trans.gray as ::core::ffi::c_int
                    {
                        is_gray = 0 as ::core::ffi::c_int;
                        break;
                    }
                } else {
                    crt_trans_value = 255 as png_byte;
                }
                if (*palette.offset(k as isize)).red as ::core::ffi::c_int
                    == gray_trans.gray as ::core::ffi::c_int
                    && crt_trans_value as ::core::ffi::c_int
                        != last_trans_value as ::core::ffi::c_int
                {
                    is_gray = 0 as ::core::ffi::c_int;
                    break;
                }
            }
            k += 1;
        }
    }
    if num_trans > 0 as ::core::ffi::c_int && last_trans_index < 0 as ::core::ffi::c_int {
        num_trans = 0 as ::core::ffi::c_int;
        png_free_data(
            png_ptr as png_const_structrp,
            info_ptr as png_inforp,
            PNG_FREE_TRNS,
            -(1 as ::core::ffi::c_int),
        );
        png_set_invalid(
            png_ptr as png_const_structrp,
            info_ptr as png_inforp,
            PNG_INFO_tRNS as ::core::ffi::c_int,
        );
        result |= OPNG_REDUCE_PALETTE_FAST as ::core::ffi::c_uint;
    }
    if reductions as ::core::ffi::c_uint & OPNG_REDUCE_PALETTE_FAST as ::core::ffi::c_uint != 0 {
        if num_palette != last_color_index + 1 as ::core::ffi::c_int {
            opng_realloc_PLTE(
                png_ptr,
                info_ptr,
                last_color_index + 1 as ::core::ffi::c_int,
            );
            png_get_PLTE(
                png_ptr as png_const_structrp,
                info_ptr as png_inforp,
                &raw mut palette,
                &raw mut num_palette,
            );
            '_c2rust_label_3: {
                if num_palette == last_color_index + 1 as ::core::ffi::c_int {
                } else {
                    __assert_fail(
                        b"num_palette == last_color_index + 1\0" as *const u8
                            as *const ::core::ffi::c_char,
                        b"opngreduc.c\0" as *const u8 as *const ::core::ffi::c_char,
                        1203 as ::core::ffi::c_uint,
                        b"png_uint_32 opng_reduce_palette(png_structp, png_infop, png_uint_32)\0"
                            as *const u8 as *const ::core::ffi::c_char,
                    );
                }
            };
            result |= OPNG_REDUCE_PALETTE_FAST as ::core::ffi::c_uint;
        }
        if num_trans > 0 as ::core::ffi::c_int
            && num_trans != last_trans_index + 1 as ::core::ffi::c_int
        {
            opng_realloc_tRNS(
                png_ptr,
                info_ptr,
                last_trans_index + 1 as ::core::ffi::c_int,
            );
            png_get_tRNS(
                png_ptr as png_const_structrp,
                info_ptr as png_inforp,
                &raw mut trans_alpha,
                &raw mut num_trans,
                ::core::ptr::null_mut::<png_color_16p>(),
            );
            '_c2rust_label_4: {
                if num_trans == last_trans_index + 1 as ::core::ffi::c_int {
                } else {
                    __assert_fail(
                        b"num_trans == last_trans_index + 1\0" as *const u8
                            as *const ::core::ffi::c_char,
                        b"opngreduc.c\0" as *const u8 as *const ::core::ffi::c_char,
                        1212 as ::core::ffi::c_uint,
                        b"png_uint_32 opng_reduce_palette(png_structp, png_infop, png_uint_32)\0"
                            as *const u8 as *const ::core::ffi::c_char,
                    );
                }
            };
            result |= OPNG_REDUCE_PALETTE_FAST as ::core::ffi::c_uint;
        }
    }
    if reductions as ::core::ffi::c_uint & OPNG_REDUCE_8_TO_4_2_1 as ::core::ffi::c_uint != 0 {
        result |= opng_reduce_palette_bits(png_ptr, info_ptr, reductions) as ::core::ffi::c_uint;
        bit_depth = png_get_bit_depth(png_ptr as png_const_structrp, info_ptr as png_const_inforp)
            as ::core::ffi::c_int;
    }
    if bit_depth < 8 as ::core::ffi::c_int || is_gray == 0 {
        return result;
    }
    i = 0 as png_uint_32;
    while i < height {
        j = 0 as png_uint_32;
        while j < width {
            *(*row_ptr.offset(i as isize)).offset(j as isize) =
                (*palette.offset(*(*row_ptr.offset(i as isize)).offset(j as isize) as isize)).red;
            j = j.wrapping_add(1);
        }
        i = i.wrapping_add(1);
    }
    if num_trans > 0 as ::core::ffi::c_int {
        png_set_tRNS(
            png_ptr as png_structrp,
            info_ptr as png_inforp,
            ::core::ptr::null::<png_byte>(),
            0 as ::core::ffi::c_int,
            &raw mut gray_trans as png_const_color_16p,
        );
    }
    if png_get_bKGD(
        png_ptr as png_const_structrp,
        info_ptr as png_inforp,
        &raw mut background,
    ) != 0
    {
        (*background).gray = (*palette.offset((*background).index as isize)).red as png_uint_16;
    }
    if png_get_hIST(
        png_ptr as png_const_structrp,
        info_ptr as png_inforp,
        &raw mut hist,
    ) != 0
    {
        png_free_data(
            png_ptr as png_const_structrp,
            info_ptr as png_inforp,
            PNG_FREE_HIST,
            -(1 as ::core::ffi::c_int),
        );
        png_set_invalid(
            png_ptr as png_const_structrp,
            info_ptr as png_inforp,
            PNG_INFO_hIST as ::core::ffi::c_int,
        );
    }
    if png_get_sBIT(
        png_ptr as png_const_structrp,
        info_ptr as png_inforp,
        &raw mut sig_bits,
    ) != 0
    {
        let mut max_sig_bits: png_byte = (*sig_bits).red;
        if (max_sig_bits as ::core::ffi::c_int) < (*sig_bits).green as ::core::ffi::c_int {
            max_sig_bits = (*sig_bits).green;
        }
        if (max_sig_bits as ::core::ffi::c_int) < (*sig_bits).blue as ::core::ffi::c_int {
            max_sig_bits = (*sig_bits).blue;
        }
        (*sig_bits).gray = max_sig_bits;
    }
    png_set_IHDR(
        png_ptr as png_const_structrp,
        info_ptr as png_inforp,
        width,
        height,
        bit_depth,
        PNG_COLOR_TYPE_GRAY,
        interlace_type,
        compression_type,
        filter_type,
    );
    png_free_data(
        png_ptr as png_const_structrp,
        info_ptr as png_inforp,
        PNG_FREE_PLTE,
        -(1 as ::core::ffi::c_int),
    );
    png_set_invalid(
        png_ptr as png_const_structrp,
        info_ptr as png_inforp,
        PNG_INFO_PLTE as ::core::ffi::c_int,
    );
    return OPNG_REDUCE_PALETTE_TO_GRAY as png_uint_32;
}
#[no_mangle]
pub unsafe extern "C" fn opng_reduce_image(
    mut png_ptr: png_structp,
    mut info_ptr: png_infop,
    mut reductions: png_uint_32,
) -> png_uint_32 {
    let mut result: png_uint_32 = 0;
    let mut color_type: ::core::ffi::c_int = 0;
    if opng_validate_image(png_ptr, info_ptr) == 0 {
        png_warning(
            png_ptr as png_const_structrp,
            b"Image reduction requires the presence of all critical information\0" as *const u8
                as png_const_charp,
        );
        return OPNG_REDUCE_NONE as png_uint_32;
    }
    color_type = png_get_color_type(png_ptr as png_const_structrp, info_ptr as png_const_inforp)
        as ::core::ffi::c_int;
    result = opng_reduce_bits(png_ptr, info_ptr, reductions);
    if color_type == PNG_COLOR_TYPE_PALETTE
        && reductions as ::core::ffi::c_uint
            & (OPNG_REDUCE_PALETTE_TO_GRAY | OPNG_REDUCE_PALETTE_FAST | OPNG_REDUCE_8_TO_4_2_1)
                as ::core::ffi::c_uint
            != 0
    {
        result |= opng_reduce_palette(png_ptr, info_ptr, reductions) as ::core::ffi::c_uint;
    }
    if color_type & !PNG_COLOR_MASK_ALPHA == PNG_COLOR_TYPE_GRAY
        && reductions as ::core::ffi::c_uint & OPNG_REDUCE_GRAY_TO_PALETTE as ::core::ffi::c_uint
            != 0
        || color_type & !PNG_COLOR_MASK_ALPHA == PNG_COLOR_TYPE_RGB
            && reductions as ::core::ffi::c_uint & OPNG_REDUCE_RGB_TO_PALETTE as ::core::ffi::c_uint
                != 0
    {
        if result as ::core::ffi::c_uint & OPNG_REDUCE_PALETTE_TO_GRAY as ::core::ffi::c_uint == 0 {
            result |= opng_reduce_to_palette(png_ptr, info_ptr, reductions) as ::core::ffi::c_uint;
        }
    }
    return result;
}
