use core::ffi::*;
use crate::src::libpng::pngerror::png_safecat;
use crate::src::libpng::pngwutil::png_save_uint_32;
pub use crate::src::ffi::*;
pub use crate::src::c_consts::*;
pub use crate::src::c_structs::*;
pub use crate::src::c_types::*;
pub use crate::src::c_extern_types::*;
pub use crate::src::zlib::deflate::internal_state;
extern "C" {
    fn png_set_error_fn(
        png_ptr: png_structrp,
        error_ptr: png_voidp,
        error_fn: png_error_ptr,
        warning_fn: png_error_ptr,
    );
    fn png_malloc_warn(png_ptr: png_const_structrp, size: png_alloc_size_t) -> png_voidp;
    fn png_free(png_ptr: png_const_structrp, ptr: png_voidp);
    fn png_error(png_ptr: png_const_structrp, error_message: png_const_charp) -> !;
    fn png_warning(png_ptr: png_const_structrp, warning_message: png_const_charp);
    fn inflateReset(strm: z_streamp) -> c_int;
    fn png_malloc_base(png_ptr: png_const_structrp, size: png_alloc_size_t) -> png_voidp;
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct png_struct_def {
    pub error_fn: png_error_ptr,
    pub warning_fn: png_error_ptr,
    pub error_ptr: png_voidp,
    pub write_data_fn: png_rw_ptr,
    pub read_data_fn: png_rw_ptr,
    pub io_ptr: png_voidp,
    pub mode: png_uint_32,
    pub flags: png_uint_32,
    pub transformations: png_uint_32,
    pub zowner: png_uint_32,
    pub zstream: z_stream,
    pub zbuffer_list: png_compression_bufferp,
    pub zbuffer_size: uInt,
    pub zlib_level: c_int,
    pub zlib_method: c_int,
    pub zlib_window_bits: c_int,
    pub zlib_mem_level: c_int,
    pub zlib_strategy: c_int,
    pub zlib_set_level: c_int,
    pub zlib_set_method: c_int,
    pub zlib_set_window_bits: c_int,
    pub zlib_set_mem_level: c_int,
    pub zlib_set_strategy: c_int,
    pub width: png_uint_32,
    pub height: png_uint_32,
    pub num_rows: png_uint_32,
    pub usr_width: png_uint_32,
    pub rowbytes: png_size_t,
    pub iwidth: png_uint_32,
    pub row_number: png_uint_32,
    pub chunk_name: png_uint_32,
    pub prev_row: png_bytep,
    pub row_buf: png_bytep,
    pub try_row: png_bytep,
    pub tst_row: png_bytep,
    pub info_rowbytes: png_size_t,
    pub idat_size: png_uint_32,
    pub crc: png_uint_32,
    pub palette: png_colorp,
    pub num_palette: png_uint_16,
    pub num_palette_max: c_int,
    pub num_trans: png_uint_16,
    pub compression: png_byte,
    pub filter: png_byte,
    pub interlaced: png_byte,
    pub pass: png_byte,
    pub do_filter: png_byte,
    pub color_type: png_byte,
    pub bit_depth: png_byte,
    pub usr_bit_depth: png_byte,
    pub pixel_depth: png_byte,
    pub channels: png_byte,
    pub usr_channels: png_byte,
    pub sig_bytes: png_byte,
    pub maximum_pixel_depth: png_byte,
    pub transformed_pixel_depth: png_byte,
    pub zstream_start: png_byte,
    pub background_gamma_type: png_byte,
    pub background_gamma: png_fixed_point,
    pub background: png_color_16,
    pub output_flush_fn: png_flush_ptr,
    pub flush_dist: png_uint_32,
    pub flush_rows: png_uint_32,
    pub sig_bit: png_color_8,
    pub trans_alpha: png_bytep,
    pub trans_color: png_color_16,
    pub read_row_fn: png_read_status_ptr,
    pub write_row_fn: png_write_status_ptr,
    pub free_me: png_uint_32,
    pub unknown_default: c_int,
    pub num_chunk_list: c_uint,
    pub chunk_list: png_bytep,
    pub big_row_buf: png_bytep,
    pub compression_type: png_byte,
    pub user_width_max: png_uint_32,
    pub user_height_max: png_uint_32,
    pub user_chunk_cache_max: png_uint_32,
    pub user_chunk_malloc_max: png_alloc_size_t,
    pub unknown_chunk: png_unknown_chunk,
    pub old_big_row_buf_size: png_size_t,
    pub read_buffer: png_bytep,
    pub read_buffer_size: png_alloc_size_t,
    pub IDAT_read_size: uInt,
    pub io_state: png_uint_32,
    pub big_prev_row: png_bytep,
    pub read_filter:
        [Option<unsafe extern "C" fn(png_row_infop, png_bytep, png_const_bytep) -> ()>; 4],
}

pub type png_write_status_ptr =
    Option<unsafe extern "C" fn(png_structp, png_uint_32, c_int) -> ()>;
pub type png_structp = *mut png_struct;
pub type png_struct = png_struct_def;
pub type png_read_status_ptr =
    Option<unsafe extern "C" fn(png_structp, png_uint_32, c_int) -> ()>;

pub type png_flush_ptr = Option<unsafe extern "C" fn(png_structp) -> ()>;

pub type z_stream = z_stream_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct z_stream_s {
    pub next_in: *const Bytef,
    pub avail_in: uInt,
    pub total_in: uLong,
    pub next_out: *mut Bytef,
    pub avail_out: uInt,
    pub total_out: uLong,
    pub msg: *const c_char,
    pub state: *mut internal_state,
    pub zalloc: alloc_func,
    pub zfree: free_func,
    pub opaque: voidpf,
    pub data_type: c_int,
    pub adler: uLong,
    pub reserved: uLong,
}

pub type png_rw_ptr = Option<unsafe extern "C" fn(png_structp, png_bytep, png_size_t) -> ()>;
pub type png_error_ptr = Option<unsafe extern "C" fn(png_structp, png_const_charp) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct png_info_def {
    pub width: png_uint_32,
    pub height: png_uint_32,
    pub valid: png_uint_32,
    pub rowbytes: png_size_t,
    pub palette: png_colorp,
    pub num_palette: png_uint_16,
    pub num_trans: png_uint_16,
    pub bit_depth: png_byte,
    pub color_type: png_byte,
    pub compression_type: png_byte,
    pub filter_type: png_byte,
    pub interlace_type: png_byte,
    pub channels: png_byte,
    pub pixel_depth: png_byte,
    pub spare_byte: png_byte,
    pub signature: [png_byte; 8],
    pub sig_bit: png_color_8,
    pub trans_alpha: png_bytep,
    pub trans_color: png_color_16,
    pub background: png_color_16,
    pub hist: png_uint_16p,
    pub free_me: png_uint_32,
    pub unknown_chunks: png_unknown_chunkp,
    pub unknown_chunks_num: c_int,
    pub row_pointers: png_bytepp,
}

pub type png_info = png_info_def;
pub type png_infop = *mut png_info;
pub type png_infopp = *mut *mut png_info;
pub type png_structrp = *mut png_struct;
pub type png_const_structrp = *const png_struct;
pub type png_inforp = *mut png_info;
pub type png_malloc_ptr = Option<unsafe extern "C" fn(png_structp, png_alloc_size_t) -> png_voidp>;
pub type png_free_ptr = Option<unsafe extern "C" fn(png_structp, png_voidp) -> ()>;
pub type z_streamp = *mut z_stream;
pub const PNG_USER_CHUNK_CACHE_MAX: c_int = 1000 as c_int;
pub const PNG_USER_CHUNK_MALLOC_MAX: c_int = 8000000 as c_int;
pub const PNG_USER_HEIGHT_MAX: c_int = 1000000 as c_int;
pub const PNG_USER_WIDTH_MAX: c_int = 1000000 as c_int;

pub const PNG_LIBPNG_VER_STRING: [c_char; 7] =
    unsafe { ::core::mem::transmute::<[u8; 7], [c_char; 7]>(*b"1.6.34\0") };
pub const PNG_LIBPNG_VER: c_int = 10634 as c_int;

pub const PNG_COLOR_TYPE_PALETTE: c_int =
    PNG_COLOR_MASK_COLOR | PNG_COLOR_MASK_PALETTE;
pub const PNG_COLOR_TYPE_RGB: c_int = 2 as c_int;
pub const PNG_COLOR_TYPE_RGB_ALPHA: c_int =
    PNG_COLOR_MASK_COLOR | PNG_COLOR_MASK_ALPHA;
pub const PNG_COLOR_TYPE_GRAY_ALPHA: c_int = 4 as c_int;

pub const PNG_INTERLACE_LAST: c_int = 2 as c_int;

pub const PNG_FREE_MUL: c_uint = 0x4220 as c_uint;

pub const PNG_FLAG_LIBRARY_MISMATCH: c_uint = 0x20000 as c_uint;

pub const Z_OK: c_int = 0;
pub const Z_STREAM_END: c_int = 1;
pub const Z_NEED_DICT: c_int = 2;
pub const Z_ERRNO: c_int = -1;

pub const Z_DATA_ERROR: c_int = -3;
pub const Z_MEM_ERROR: c_int = -4;
pub const Z_BUF_ERROR: c_int = -5;
pub const Z_VERSION_ERROR: c_int = -6;

#[no_mangle]
pub extern "C" fn png_set_sig_bytes(
    mut png_ptr: png_structrp,
    mut num_bytes: c_int,
) { unsafe {
    let mut nb: c_uint = num_bytes as c_uint;
    if png_ptr.is_null() {
        return;
    }
    if num_bytes < 0 as c_int {
        nb = 0 as c_uint;
    }
    if nb > 8 as c_uint {
        png_error(
            png_ptr,
            b"Too many bytes for PNG signature\0" as *const u8 as png_const_charp,
        );
    }
    (*png_ptr).sig_bytes = nb as png_byte;
} }
#[inline]
pub fn png_sig_cmp(
    mut sig: png_const_bytep,
    mut start: png_size_t,
    mut num_to_check: png_size_t,
) -> c_int { unsafe {
    let mut png_signature: [png_byte; 8] = [
        137 as c_int as png_byte,
        80 as c_int as png_byte,
        78 as c_int as png_byte,
        71 as c_int as png_byte,
        13 as c_int as png_byte,
        10 as c_int as png_byte,
        26 as c_int as png_byte,
        10 as c_int as png_byte,
    ];
    if num_to_check > 8 as png_size_t {
        num_to_check = 8 as png_size_t;
    } else if num_to_check < 1 as png_size_t {
        return -(1 as c_int);
    }
    if start > 7 as png_size_t {
        return -(1 as c_int);
    }
    if start.wrapping_add(num_to_check) > 8 as png_size_t {
        num_to_check = (8 as png_size_t).wrapping_sub(start);
    }
    return memcmp(
        sig.offset(start as isize) as *const png_byte as *const c_void,
        (&raw mut png_signature as *mut png_byte).offset(start as isize) as *mut png_byte
            as *const c_void,
        num_to_check as size_t,
    );
} }
#[no_mangle]
pub extern "C" fn png_zalloc(
    mut png_ptr: voidpf,
    mut items: uInt,
    mut size: uInt,
) -> voidpf { unsafe {
    let mut num_bytes: png_alloc_size_t = size as png_alloc_size_t;
    if png_ptr.is_null() {
        return NULL;
    }
    if items as png_alloc_size_t
        >= (!(0 as c_int as png_alloc_size_t)).wrapping_div(size as png_alloc_size_t)
    {
        png_warning(
            png_ptr as png_const_structrp,
            b"Potential overflow in png_zalloc()\0" as *const u8 as png_const_charp,
        );
        return NULL;
    }
    num_bytes = (num_bytes as c_ulong).wrapping_mul(items as c_ulong)
        as png_alloc_size_t as png_alloc_size_t;
    return png_malloc_warn(png_ptr as png_const_structrp, num_bytes) as voidpf;
} }
#[no_mangle]
pub extern "C" fn png_zfree(mut png_ptr: voidpf, mut ptr: voidpf) { unsafe {
    png_free(png_ptr as png_const_structrp, ptr as png_voidp);
} }
#[no_mangle]
pub extern "C" fn png_reset_crc(mut png_ptr: png_structrp) { unsafe {
    (*png_ptr).crc = crc32(0 as uLong, ::core::ptr::null::<Bytef>(), 0 as uInt) as png_uint_32;
} }
#[no_mangle]
pub extern "C" fn png_calculate_crc(
    mut png_ptr: png_structrp,
    mut ptr: png_const_bytep,
    mut length: png_size_t,
) { unsafe {
    let mut need_crc: c_int = 1 as c_int;
    if 1 as png_uint_32 & (*png_ptr).chunk_name >> 29 as c_int
        != 0 as c_uint
    {
        if (*png_ptr).flags as c_uint & PNG_FLAG_CRC_ANCILLARY_MASK
            == PNG_FLAG_CRC_ANCILLARY_USE | PNG_FLAG_CRC_ANCILLARY_NOWARN
        {
            need_crc = 0 as c_int;
        }
    } else if (*png_ptr).flags as c_uint & PNG_FLAG_CRC_CRITICAL_IGNORE
        != 0 as c_uint
    {
        need_crc = 0 as c_int;
    }
    if need_crc != 0 as c_int && length > 0 as png_size_t {
        let mut crc: uLong = (*png_ptr).crc as uLong;
        loop {
            let mut safe_length: uInt = length as uInt;
            if safe_length == 0 as c_uint {
                safe_length = -(1 as c_int) as uInt;
            }
            crc = crc32(crc, ptr as *const Bytef, safe_length);
            ptr = ptr.offset(safe_length as isize);
            length = (length as c_ulong)
                .wrapping_sub(safe_length as c_ulong)
                as png_size_t as png_size_t;
            if !(length > 0 as png_size_t) {
                break;
            }
        }
        (*png_ptr).crc = crc as png_uint_32;
    }
} }
#[inline]
pub fn png_user_version_check(
    mut png_ptr: png_structrp,
    mut user_png_ver: png_const_charp,
) -> c_int { unsafe {
    if !user_png_ver.is_null() {
        let mut i: c_int = -(1 as c_int);
        let mut found_dots: c_int = 0 as c_int;
        loop {
            i += 1;
            if *user_png_ver.offset(i as isize) as c_int
                != PNG_LIBPNG_VER_STRING[i as usize] as c_int
            {
                (*png_ptr).flags |= PNG_FLAG_LIBRARY_MISMATCH;
            }
            if *user_png_ver.offset(i as isize) as c_int == '.' as i32 {
                found_dots += 1;
            }
            if !(found_dots < 2 as c_int
                && *user_png_ver.offset(i as isize) as c_int
                    != 0 as c_int
                && PNG_LIBPNG_VER_STRING[i as usize] as c_int
                    != 0 as c_int)
            {
                break;
            }
        }
    } else {
        (*png_ptr).flags |= PNG_FLAG_LIBRARY_MISMATCH;
    }
    if (*png_ptr).flags as c_uint & PNG_FLAG_LIBRARY_MISMATCH
        != 0 as c_uint
    {
        let mut pos: size_t = 0 as size_t;
        let mut m: [c_char; 128] = [0; 128];
        pos = png_safecat(
            &raw mut m as png_charp,
            ::core::mem::size_of::<[c_char; 128]>() as size_t,
            pos,
            b"Application built with libpng-\0" as *const u8 as png_const_charp,
        );
        pos = png_safecat(
            &raw mut m as png_charp,
            ::core::mem::size_of::<[c_char; 128]>() as size_t,
            pos,
            user_png_ver,
        );
        pos = png_safecat(
            &raw mut m as png_charp,
            ::core::mem::size_of::<[c_char; 128]>() as size_t,
            pos,
            b" but running with \0" as *const u8 as png_const_charp,
        );
        pos = png_safecat(
            &raw mut m as png_charp,
            ::core::mem::size_of::<[c_char; 128]>() as size_t,
            pos,
            PNG_LIBPNG_VER_STRING.as_ptr(),
        );
        png_warning(
            png_ptr,
            &raw mut m as *mut c_char as png_const_charp,
        );
        return 0 as c_int;
    }
    return 1 as c_int;
} }
#[no_mangle]
pub extern "C" fn png_create_png_struct(
    mut user_png_ver: png_const_charp,
    mut error_ptr: png_voidp,
    mut error_fn: png_error_ptr,
    mut warn_fn: png_error_ptr,
    mut mem_ptr: png_voidp,
    mut malloc_fn: png_malloc_ptr,
    mut free_fn: png_free_ptr,
) -> png_structp { unsafe {
    let mut create_struct: png_struct = png_struct {
        error_fn: None,
        warning_fn: None,
        error_ptr: ::core::ptr::null_mut::<c_void>(),
        write_data_fn: None,
        read_data_fn: None,
        io_ptr: ::core::ptr::null_mut::<c_void>(),
        mode: 0,
        flags: 0,
        transformations: 0,
        zowner: 0,
        zstream: z_stream {
            next_in: ::core::ptr::null::<Bytef>(),
            avail_in: 0,
            total_in: 0,
            next_out: ::core::ptr::null_mut::<Bytef>(),
            avail_out: 0,
            total_out: 0,
            msg: ::core::ptr::null::<c_char>(),
            state: ::core::ptr::null_mut::<internal_state>(),
            zalloc: None,
            zfree: None,
            opaque: ::core::ptr::null_mut::<c_void>(),
            data_type: 0,
            adler: 0,
            reserved: 0,
        },
        zbuffer_list: ::core::ptr::null_mut::<png_compression_buffer>(),
        zbuffer_size: 0,
        zlib_level: 0,
        zlib_method: 0,
        zlib_window_bits: 0,
        zlib_mem_level: 0,
        zlib_strategy: 0,
        zlib_set_level: 0,
        zlib_set_method: 0,
        zlib_set_window_bits: 0,
        zlib_set_mem_level: 0,
        zlib_set_strategy: 0,
        width: 0,
        height: 0,
        num_rows: 0,
        usr_width: 0,
        rowbytes: 0,
        iwidth: 0,
        row_number: 0,
        chunk_name: 0,
        prev_row: ::core::ptr::null_mut::<png_byte>(),
        row_buf: ::core::ptr::null_mut::<png_byte>(),
        try_row: ::core::ptr::null_mut::<png_byte>(),
        tst_row: ::core::ptr::null_mut::<png_byte>(),
        info_rowbytes: 0,
        idat_size: 0,
        crc: 0,
        palette: ::core::ptr::null_mut::<png_color>(),
        num_palette: 0,
        num_palette_max: 0,
        num_trans: 0,
        compression: 0,
        filter: 0,
        interlaced: 0,
        pass: 0,
        do_filter: 0,
        color_type: 0,
        bit_depth: 0,
        usr_bit_depth: 0,
        pixel_depth: 0,
        channels: 0,
        usr_channels: 0,
        sig_bytes: 0,
        maximum_pixel_depth: 0,
        transformed_pixel_depth: 0,
        zstream_start: 0,
        background_gamma_type: 0,
        background_gamma: 0,
        background: png_color_16 {
            index: 0,
            red: 0,
            green: 0,
            blue: 0,
            gray: 0,
        },
        output_flush_fn: None,
        flush_dist: 0,
        flush_rows: 0,
        sig_bit: png_color_8 {
            red: 0,
            green: 0,
            blue: 0,
            gray: 0,
            alpha: 0,
        },
        trans_alpha: ::core::ptr::null_mut::<png_byte>(),
        trans_color: png_color_16 {
            index: 0,
            red: 0,
            green: 0,
            blue: 0,
            gray: 0,
        },
        read_row_fn: None,
        write_row_fn: None,
        free_me: 0,
        unknown_default: 0,
        num_chunk_list: 0,
        chunk_list: ::core::ptr::null_mut::<png_byte>(),
        big_row_buf: ::core::ptr::null_mut::<png_byte>(),
        compression_type: 0,
        user_width_max: 0,
        user_height_max: 0,
        user_chunk_cache_max: 0,
        user_chunk_malloc_max: 0,
        unknown_chunk: png_unknown_chunk {
            name: [0; 5],
            data: ::core::ptr::null_mut::<png_byte>(),
            size: 0,
            location: 0,
        },
        old_big_row_buf_size: 0,
        read_buffer: ::core::ptr::null_mut::<png_byte>(),
        read_buffer_size: 0,
        IDAT_read_size: 0,
        io_state: 0,
        big_prev_row: ::core::ptr::null_mut::<png_byte>(),
        read_filter: [None; 4],
    };
    memset(
        &raw mut create_struct as *mut c_void,
        0 as c_int,
        ::core::mem::size_of::<png_struct>() as size_t,
    );
    create_struct.user_width_max = PNG_USER_WIDTH_MAX as png_uint_32;
    create_struct.user_height_max = PNG_USER_HEIGHT_MAX as png_uint_32;
    create_struct.user_chunk_cache_max = PNG_USER_CHUNK_CACHE_MAX as png_uint_32;
    create_struct.user_chunk_malloc_max = PNG_USER_CHUNK_MALLOC_MAX as png_alloc_size_t;
    png_set_error_fn(&raw mut create_struct, error_ptr, error_fn, warn_fn);
    if png_user_version_check(&raw mut create_struct, user_png_ver) != 0 as c_int {
        let mut png_ptr: png_structrp = png_malloc_warn(
            &raw mut create_struct,
            ::core::mem::size_of::<png_struct>() as png_alloc_size_t,
        ) as png_structrp;
        if !png_ptr.is_null() {
            create_struct.zstream.zalloc =
                Some(png_zalloc as unsafe extern "C" fn(voidpf, uInt, uInt) -> voidpf)
                    as alloc_func;
            create_struct.zstream.zfree =
                Some(png_zfree as unsafe extern "C" fn(voidpf, voidpf) -> ()) as free_func;
            create_struct.zstream.opaque = png_ptr as voidpf;
            *png_ptr = create_struct;
            return png_ptr as png_structp;
        }
    }
    return ::core::ptr::null_mut::<png_struct>();
} }
#[no_mangle]
pub extern "C" fn png_create_info_struct(mut png_ptr: png_const_structrp) -> png_infop { unsafe {
    let mut info_ptr: png_inforp = ::core::ptr::null_mut::<png_info>();
    if png_ptr.is_null() {
        return ::core::ptr::null_mut::<png_info>();
    }
    info_ptr = png_malloc_base(
        png_ptr,
        ::core::mem::size_of::<png_info>() as png_alloc_size_t,
    ) as *mut png_info as png_inforp;
    if !info_ptr.is_null() {
        memset(
            info_ptr as *mut c_void,
            0 as c_int,
            ::core::mem::size_of::<png_info>() as size_t,
        );
    }
    return info_ptr as png_infop;
} }
#[no_mangle]
pub extern "C" fn png_destroy_info_struct(
    mut png_ptr: png_const_structrp,
    mut info_ptr_ptr: png_infopp,
) { unsafe {
    let mut info_ptr: png_inforp = ::core::ptr::null_mut::<png_info>();
    if png_ptr.is_null() {
        return;
    }
    if !info_ptr_ptr.is_null() {
        info_ptr = *info_ptr_ptr as png_inforp;
    }
    if !info_ptr.is_null() {
        *info_ptr_ptr = ::core::ptr::null_mut::<png_info>();
        png_free_data(png_ptr, info_ptr, PNG_FREE_ALL, -(1 as c_int));
        memset(
            info_ptr as *mut c_void,
            0 as c_int,
            ::core::mem::size_of::<png_info>() as size_t,
        );
        png_free(png_ptr, info_ptr as png_voidp);
    }
} }
#[inline]
pub fn png_info_init_3(
    mut ptr_ptr: png_infopp,
    mut png_info_struct_size: png_size_t,
) { unsafe {
    let mut info_ptr: png_inforp = *ptr_ptr;
    if info_ptr.is_null() {
        return;
    }
    if ::core::mem::size_of::<png_info>() as usize > png_info_struct_size {
        *ptr_ptr = ::core::ptr::null_mut::<png_info>();
        free(info_ptr as *mut c_void);
        info_ptr = png_malloc_base(
            ::core::ptr::null::<png_struct>(),
            ::core::mem::size_of::<png_info>() as png_alloc_size_t,
        ) as *mut png_info as png_inforp;
        if info_ptr.is_null() {
            return;
        }
        *ptr_ptr = info_ptr as *mut png_info;
    }
    memset(
        info_ptr as *mut c_void,
        0 as c_int,
        ::core::mem::size_of::<png_info>() as size_t,
    );
} }
#[no_mangle]
pub extern "C" fn png_data_freer(
    mut png_ptr: png_const_structrp,
    mut info_ptr: png_inforp,
    mut freer: c_int,
    mut mask: png_uint_32,
) { unsafe {
    if png_ptr.is_null() || info_ptr.is_null() {
        return;
    }
    if freer == PNG_DESTROY_WILL_FREE_DATA {
        (*info_ptr).free_me |= mask as c_uint;
    } else if freer == PNG_USER_WILL_FREE_DATA {
        (*info_ptr).free_me &= !mask as c_uint;
    } else {
        png_error(
            png_ptr,
            b"Unknown freer parameter in png_data_freer\0" as *const u8 as png_const_charp,
        );
    };
} }
#[no_mangle]
pub extern "C" fn png_free_data(
    mut png_ptr: png_const_structrp,
    mut info_ptr: png_inforp,
    mut mask: png_uint_32,
    mut num: c_int,
) { unsafe {
    if png_ptr.is_null() || info_ptr.is_null() {
        return;
    }
    if mask & PNG_FREE_TRNS & (*info_ptr).free_me != 0 as c_uint {
        (*info_ptr).valid &= !PNG_INFO_tRNS;
        png_free(png_ptr, (*info_ptr).trans_alpha as png_voidp);
        (*info_ptr).trans_alpha = ::core::ptr::null_mut::<png_byte>();
        (*info_ptr).num_trans = 0 as png_uint_16;
    }
    if !(*info_ptr).unknown_chunks.is_null()
        && mask & PNG_FREE_UNKN & (*info_ptr).free_me != 0 as c_uint
    {
        if num != -(1 as c_int) {
            png_free(
                png_ptr,
                (*(*info_ptr).unknown_chunks.offset(num as isize)).data as png_voidp,
            );
            let ref mut fresh0 = (*(*info_ptr).unknown_chunks.offset(num as isize)).data;
            *fresh0 = ::core::ptr::null_mut::<png_byte>();
        } else {
            let mut i: c_int = 0;
            i = 0 as c_int;
            while i < (*info_ptr).unknown_chunks_num {
                png_free(
                    png_ptr,
                    (*(*info_ptr).unknown_chunks.offset(i as isize)).data as png_voidp,
                );
                i += 1;
            }
            png_free(png_ptr, (*info_ptr).unknown_chunks as png_voidp);
            (*info_ptr).unknown_chunks = ::core::ptr::null_mut::<png_unknown_chunk>();
            (*info_ptr).unknown_chunks_num = 0 as c_int;
        }
    }
    if mask & PNG_FREE_HIST & (*info_ptr).free_me != 0 as c_uint {
        png_free(png_ptr, (*info_ptr).hist as png_voidp);
        (*info_ptr).hist = ::core::ptr::null_mut::<png_uint_16>();
        (*info_ptr).valid &= !PNG_INFO_hIST;
    }
    if mask & PNG_FREE_PLTE & (*info_ptr).free_me != 0 as c_uint {
        png_free(png_ptr, (*info_ptr).palette as png_voidp);
        (*info_ptr).palette = ::core::ptr::null_mut::<png_color>();
        (*info_ptr).valid &= !PNG_INFO_PLTE;
        (*info_ptr).num_palette = 0 as png_uint_16;
    }
    if mask & PNG_FREE_ROWS & (*info_ptr).free_me != 0 as c_uint {
        if !(*info_ptr).row_pointers.is_null() {
            let mut row: png_uint_32 = 0;
            row = 0 as png_uint_32;
            while row < (*info_ptr).height {
                png_free(
                    png_ptr,
                    *(*info_ptr).row_pointers.offset(row as isize) as png_voidp,
                );
                row = row.wrapping_add(1);
            }
            png_free(png_ptr, (*info_ptr).row_pointers as png_voidp);
            (*info_ptr).row_pointers = ::core::ptr::null_mut::<*mut png_byte>();
        }
        (*info_ptr).valid &= !PNG_INFO_IDAT;
    }
    if num != -(1 as c_int) {
        mask &= !PNG_FREE_MUL;
    }
    (*info_ptr).free_me &= !mask as c_uint;
} }
#[no_mangle]
pub extern "C" fn png_get_io_ptr(mut png_ptr: png_const_structrp) -> png_voidp { unsafe {
    if png_ptr.is_null() {
        return ::core::ptr::null_mut::<c_void>();
    }
    return (*png_ptr).io_ptr;
} }
#[inline]
pub fn png_init_io(mut png_ptr: png_structrp, mut fp: png_FILE_p) { unsafe {
    if png_ptr.is_null() {
        return;
    }
    (*png_ptr).io_ptr = fp as png_voidp;
} }
#[inline]
pub fn png_save_int_32(mut buf: png_bytep, mut i: png_int_32) { {
    png_save_uint_32(buf, i as png_uint_32);
} }
#[inline]
pub fn png_get_copyright(mut png_ptr: png_const_structrp) -> png_const_charp { {
    return b"\nlibpng version 1.6.34 - September 29, 2017\nCopyright (c) 1998-2002,2004,2006-2017 Glenn Randers-Pehrson\nCopyright (c) 1996-1997 Andreas Dilger\nCopyright (c) 1995-1996 Guy Eric Schalnat, Group 42, Inc.\n\0"
        as *const u8 as png_const_charp;
} }
#[no_mangle]
pub extern "C" fn png_get_libpng_ver(mut png_ptr: png_const_structrp) -> png_const_charp { {
    return png_get_header_ver(png_ptr);
} }
#[inline]
pub fn png_get_header_ver(mut png_ptr: png_const_structrp) -> png_const_charp { {
    return PNG_LIBPNG_VER_STRING.as_ptr();
} }
#[inline]
pub fn png_get_header_version(
    mut png_ptr: png_const_structrp,
) -> png_const_charp { {
    return b" libpng version 1.6.34 - September 29, 2017\n\n\0" as *const u8 as png_const_charp;
} }
#[no_mangle]
pub extern "C" fn png_handle_as_unknown(
    mut png_ptr: png_const_structrp,
    mut chunk_name: png_const_bytep,
) -> c_int { unsafe {
    let mut p: png_const_bytep = ::core::ptr::null::<png_byte>();
    let mut p_end: png_const_bytep = ::core::ptr::null::<png_byte>();
    if png_ptr.is_null()
        || chunk_name.is_null()
        || (*png_ptr).num_chunk_list == 0 as c_uint
    {
        return PNG_HANDLE_CHUNK_AS_DEFAULT;
    }
    p_end = (*png_ptr).chunk_list as png_const_bytep;
    p = p_end.offset(
        (*png_ptr)
            .num_chunk_list
            .wrapping_mul(5 as c_uint) as isize,
    );
    loop {
        p = p.offset(-(5 as c_int as isize));
        if memcmp(
            chunk_name as *const c_void,
            p as *const c_void,
            4 as size_t,
        ) == 0 as c_int
        {
            return *p.offset(4 as c_int as isize) as c_int;
        }
        if !(p > p_end) {
            break;
        }
    }
    return PNG_HANDLE_CHUNK_AS_DEFAULT;
} }
#[no_mangle]
pub extern "C" fn png_chunk_unknown_handling(
    mut png_ptr: png_const_structrp,
    mut chunk_name: png_uint_32,
) -> c_int { unsafe {
    let mut chunk_string: [png_byte; 5] = [0; 5];
    *(&raw mut chunk_string as *mut png_byte as *mut c_char)
        .offset(0 as c_int as isize) =
        (chunk_name as c_uint >> 24 as c_int
            & 0xff as c_uint) as c_char;
    *(&raw mut chunk_string as *mut png_byte as *mut c_char)
        .offset(1 as c_int as isize) =
        (chunk_name as c_uint >> 16 as c_int
            & 0xff as c_uint) as c_char;
    *(&raw mut chunk_string as *mut png_byte as *mut c_char)
        .offset(2 as c_int as isize) = (chunk_name as c_uint
        >> 8 as c_int
        & 0xff as c_uint)
        as c_char;
    *(&raw mut chunk_string as *mut png_byte as *mut c_char)
        .offset(3 as c_int as isize) =
        (chunk_name as c_uint & 0xff as c_uint) as c_char;
    *(&raw mut chunk_string as *mut png_byte as *mut c_char)
        .offset(4 as c_int as isize) = 0 as c_char;
    return png_handle_as_unknown(
        png_ptr,
        &raw mut chunk_string as *mut png_byte as png_const_bytep,
    );
} }
#[inline]
pub fn png_reset_zstream(mut png_ptr: png_structrp) -> c_int { unsafe {
    if png_ptr.is_null() {
        return Z_STREAM_ERROR;
    }
    return inflateReset(&raw mut (*png_ptr).zstream);
} }
#[inline]
pub fn png_access_version_number() -> png_uint_32 { {
    return PNG_LIBPNG_VER as png_uint_32;
} }
#[no_mangle]
pub extern "C" fn png_zstream_error(mut png_ptr: png_structrp, mut ret: c_int) { unsafe {
    if (*png_ptr).zstream.msg.is_null() {
        match ret {
            Z_STREAM_END => {
                (*png_ptr).zstream.msg =
                    b"unexpected end of LZ stream\0" as *const u8 as *const c_char;
            }
            Z_NEED_DICT => {
                (*png_ptr).zstream.msg =
                    b"missing LZ dictionary\0" as *const u8 as *const c_char;
            }
            Z_ERRNO => {
                (*png_ptr).zstream.msg =
                    b"zlib IO error\0" as *const u8 as *const c_char;
            }
            Z_STREAM_ERROR => {
                (*png_ptr).zstream.msg =
                    b"bad parameters to zlib\0" as *const u8 as *const c_char;
            }
            Z_DATA_ERROR => {
                (*png_ptr).zstream.msg =
                    b"damaged LZ stream\0" as *const u8 as *const c_char;
            }
            Z_MEM_ERROR => {
                (*png_ptr).zstream.msg =
                    b"insufficient memory\0" as *const u8 as *const c_char;
            }
            Z_BUF_ERROR => {
                (*png_ptr).zstream.msg = b"truncated\0" as *const u8 as *const c_char;
            }
            Z_VERSION_ERROR => {
                (*png_ptr).zstream.msg =
                    b"unsupported zlib version\0" as *const u8 as *const c_char;
            }
            PNG_UNEXPECTED_ZLIB_RETURN => {
                (*png_ptr).zstream.msg =
                    b"unexpected zlib return\0" as *const u8 as *const c_char;
            }
            Z_OK | _ => {
                (*png_ptr).zstream.msg =
                    b"unexpected zlib return code\0" as *const u8 as *const c_char;
            }
        }
    }
} }
fn png_gt(mut a: size_t, mut b: size_t) -> c_int { {
    return (a > b) as c_int;
} }
#[no_mangle]
pub extern "C" fn png_check_IHDR(
    mut png_ptr: png_const_structrp,
    mut width: png_uint_32,
    mut height: png_uint_32,
    mut bit_depth: c_int,
    mut color_type: c_int,
    mut interlace_type: c_int,
    mut compression_type: c_int,
    mut filter_type: c_int,
) { unsafe {
    let mut error: c_int = 0 as c_int;
    if width == 0 as c_uint {
        png_warning(
            png_ptr,
            b"Image width is zero in IHDR\0" as *const u8 as png_const_charp,
        );
        error = 1 as c_int;
    }
    if width > PNG_UINT_31_MAX {
        png_warning(
            png_ptr,
            b"Invalid image width in IHDR\0" as *const u8 as png_const_charp,
        );
        error = 1 as c_int;
    }
    if png_gt(
        ((width as c_uint).wrapping_add(7 as c_uint)
            & !(7 as c_uint)) as size_t,
        PNG_SIZE_MAX
            .wrapping_sub(48 as size_t)
            .wrapping_sub(1 as size_t)
            .wrapping_div(8 as size_t)
            .wrapping_sub(1 as size_t),
    ) != 0
    {
        png_warning(
            png_ptr,
            b"Image width is too large for this architecture\0" as *const u8 as png_const_charp,
        );
        error = 1 as c_int;
    }
    if width > (*png_ptr).user_width_max {
        png_warning(
            png_ptr,
            b"Image width exceeds user limit in IHDR\0" as *const u8 as png_const_charp,
        );
        error = 1 as c_int;
    }
    if height == 0 as c_uint {
        png_warning(
            png_ptr,
            b"Image height is zero in IHDR\0" as *const u8 as png_const_charp,
        );
        error = 1 as c_int;
    }
    if height > PNG_UINT_31_MAX {
        png_warning(
            png_ptr,
            b"Invalid image height in IHDR\0" as *const u8 as png_const_charp,
        );
        error = 1 as c_int;
    }
    if height > (*png_ptr).user_height_max {
        png_warning(
            png_ptr,
            b"Image height exceeds user limit in IHDR\0" as *const u8 as png_const_charp,
        );
        error = 1 as c_int;
    }
    if bit_depth != 1 as c_int
        && bit_depth != 2 as c_int
        && bit_depth != 4 as c_int
        && bit_depth != 8 as c_int
        && bit_depth != 16 as c_int
    {
        png_warning(
            png_ptr,
            b"Invalid bit depth in IHDR\0" as *const u8 as png_const_charp,
        );
        error = 1 as c_int;
    }
    if color_type < 0 as c_int
        || color_type == 1 as c_int
        || color_type == 5 as c_int
        || color_type > 6 as c_int
    {
        png_warning(
            png_ptr,
            b"Invalid color type in IHDR\0" as *const u8 as png_const_charp,
        );
        error = 1 as c_int;
    }
    if color_type == PNG_COLOR_TYPE_PALETTE && bit_depth > 8 as c_int
        || (color_type == PNG_COLOR_TYPE_RGB
            || color_type == PNG_COLOR_TYPE_GRAY_ALPHA
            || color_type == PNG_COLOR_TYPE_RGB_ALPHA)
            && bit_depth < 8 as c_int
    {
        png_warning(
            png_ptr,
            b"Invalid color type/bit depth combination in IHDR\0" as *const u8 as png_const_charp,
        );
        error = 1 as c_int;
    }
    if interlace_type >= PNG_INTERLACE_LAST {
        png_warning(
            png_ptr,
            b"Unknown interlace method in IHDR\0" as *const u8 as png_const_charp,
        );
        error = 1 as c_int;
    }
    if compression_type != PNG_COMPRESSION_TYPE_BASE {
        png_warning(
            png_ptr,
            b"Unknown compression method in IHDR\0" as *const u8 as png_const_charp,
        );
        error = 1 as c_int;
    }
    if filter_type != PNG_FILTER_TYPE_BASE {
        png_warning(
            png_ptr,
            b"Unknown filter method in IHDR\0" as *const u8 as png_const_charp,
        );
        error = 1 as c_int;
    }
    if error == 1 as c_int {
        png_error(
            png_ptr,
            b"Invalid IHDR data\0" as *const u8 as png_const_charp,
        );
    }
} }
pub const PNG_UNEXPECTED_ZLIB_RETURN: c_int = -7;
