use core::ffi::*;
use crate::src::libpng::pngerror::png_safecat;
pub use crate::src::ffi::*;
pub use crate::src::c_consts::*;
pub use crate::src::c_structs::*;
pub use crate::src::c_types::*;
pub use crate::src::zlib::deflate::internal_state;
extern "C" {
    fn png_write_flush(png_ptr: png_structrp);
    fn png_malloc(png_ptr: png_const_structrp, size: png_alloc_size_t) -> png_voidp;
    fn png_calloc(png_ptr: png_const_structrp, size: png_alloc_size_t) -> png_voidp;
    fn png_free(png_ptr: png_const_structrp, ptr: png_voidp);
    fn png_error(png_ptr: png_const_structrp, error_message: png_const_charp) -> !;
    fn png_warning(png_ptr: png_const_structrp, warning_message: png_const_charp);
    fn deflate(strm: z_streamp, flush: c_int) -> c_int;
    fn deflateEnd(strm: z_streamp) -> c_int;
    fn deflateReset(strm: z_streamp) -> c_int;
    fn deflateInit2_(
        strm: z_streamp,
        level: c_int,
        method: c_int,
        windowBits: c_int,
        memLevel: c_int,
        strategy: c_int,
        version: *const c_char,
        stream_size: c_int,
    ) -> c_int;
    fn png_zstream_error(png_ptr: png_structrp, ret: c_int);
    fn png_reset_crc(png_ptr: png_structrp);
    fn png_write_data(png_ptr: png_structrp, data: png_const_bytep, length: png_size_t);
    fn png_calculate_crc(png_ptr: png_structrp, ptr: png_const_bytep, length: png_size_t);
    fn png_app_warning(png_ptr: png_const_structrp, message: png_const_charp);
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
pub type png_structrp = *mut png_struct;
pub type png_const_structrp = *const png_struct;

pub type z_streamp = *mut z_stream;
pub const PNG_Z_DEFAULT_NOFILTER_STRATEGY: c_int = 0 as c_int;

pub const PNG_COLOR_TYPE_GRAY: c_int = 0;
pub const PNG_COLOR_TYPE_PALETTE: c_int =
    PNG_COLOR_MASK_COLOR | PNG_COLOR_MASK_PALETTE;
pub const PNG_COLOR_TYPE_RGB: c_int = 2;
pub const PNG_COLOR_TYPE_RGB_ALPHA: c_int = 6;
pub const PNG_COLOR_TYPE_GRAY_ALPHA: c_int = 4;

pub const PNG_NO_FILTERS: c_int = 0 as c_int;

pub const PNG_FILTER_VALUE_NONE: c_int = 0 as c_int;
pub const PNG_FILTER_VALUE_SUB: c_int = 1 as c_int;
pub const PNG_FILTER_VALUE_UP: c_int = 2 as c_int;
pub const PNG_FILTER_VALUE_AVG: c_int = 3 as c_int;
pub const PNG_FILTER_VALUE_PAETH: c_int = 4 as c_int;
pub const PNG_IO_WRITING: c_int = 0x2 as c_int;

pub const PNG_IO_CHUNK_HDR: c_int = 0x20 as c_int;
pub const PNG_IO_CHUNK_DATA: c_int = 0x40 as c_int;
pub const PNG_IO_CHUNK_CRC: c_int = 0x80 as c_int;

pub const ZLIB_VERSION: [c_char; 15] =
    unsafe { ::core::mem::transmute::<[u8; 15], [c_char; 15]>(*b"1.2.11-optipng\0") };

pub const Z_OK: c_int = 0 as c_int;
pub const Z_STREAM_END: c_int = 1 as c_int;

#[inline]
pub fn png_save_uint_32(mut buf: png_bytep, mut i: png_uint_32) { unsafe {
    *buf.offset(0 as c_int as isize) =
        (i as c_uint >> 24 as c_int & 0xff as c_uint)
            as png_byte;
    *buf.offset(1 as c_int as isize) =
        (i as c_uint >> 16 as c_int & 0xff as c_uint)
            as png_byte;
    *buf.offset(2 as c_int as isize) =
        (i as c_uint >> 8 as c_int & 0xff as c_uint)
            as png_byte;
    *buf.offset(3 as c_int as isize) =
        (i as c_uint & 0xff as c_uint) as png_byte;
} }
#[inline]
pub fn png_save_uint_16(mut buf: png_bytep, mut i: c_uint) { unsafe {
    *buf.offset(0 as c_int as isize) =
        (i >> 8 as c_int & 0xff as c_uint) as png_byte;
    *buf.offset(1 as c_int as isize) = (i & 0xff as c_uint) as png_byte;
} }
#[no_mangle]
pub extern "C" fn png_write_sig(mut png_ptr: png_structrp) { unsafe {
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
    (*png_ptr).io_state = (PNG_IO_WRITING | PNG_IO_SIGNATURE) as png_uint_32;
    png_write_data(
        png_ptr,
        (&raw mut png_signature as *mut png_byte).offset((*png_ptr).sig_bytes as isize)
            as *mut png_byte as png_const_bytep,
        (8 as c_int - (*png_ptr).sig_bytes as c_int) as png_size_t,
    );
    if ((*png_ptr).sig_bytes as c_int) < 3 as c_int {
        (*png_ptr).mode |= PNG_HAVE_PNG_SIGNATURE;
    }
} }
fn png_write_chunk_header(
    mut png_ptr: png_structrp,
    mut chunk_name: png_uint_32,
    mut length: png_uint_32,
) { unsafe {
    let mut buf: [png_byte; 8] = [0; 8];
    if png_ptr.is_null() {
        return;
    }
    (*png_ptr).io_state = (PNG_IO_WRITING | PNG_IO_CHUNK_HDR) as png_uint_32;
    png_save_uint_32(&raw mut buf as png_bytep, length);
    png_save_uint_32(
        (&raw mut buf as *mut png_byte).offset(4 as c_int as isize),
        chunk_name,
    );
    png_write_data(
        png_ptr,
        &raw mut buf as *mut png_byte as png_const_bytep,
        8 as png_size_t,
    );
    (*png_ptr).chunk_name = chunk_name;
    png_reset_crc(png_ptr);
    png_calculate_crc(
        png_ptr,
        (&raw mut buf as *mut png_byte).offset(4 as c_int as isize) as png_const_bytep,
        4 as png_size_t,
    );
    (*png_ptr).io_state = (PNG_IO_WRITING | PNG_IO_CHUNK_DATA) as png_uint_32;
} }
#[inline]
pub fn png_write_chunk_start(
    mut png_ptr: png_structrp,
    mut chunk_string: png_const_bytep,
    mut length: png_uint_32,
) { unsafe {
    png_write_chunk_header(
        png_ptr,
        ((0xff as c_int
            & *chunk_string.offset(0 as c_int as isize) as c_int)
            as png_uint_32)
            << 24 as c_int
            | ((0xff as c_int
                & *chunk_string.offset(1 as c_int as isize) as c_int)
                as png_uint_32)
                << 16 as c_int
            | ((0xff as c_int
                & *chunk_string.offset(2 as c_int as isize) as c_int)
                as png_uint_32)
                << 8 as c_int
            | ((0xff as c_int
                & *chunk_string.offset(3 as c_int as isize) as c_int)
                as png_uint_32)
                << 0 as c_int,
        length,
    );
} }
#[inline]
pub fn png_write_chunk_data(
    mut png_ptr: png_structrp,
    mut data: png_const_bytep,
    mut length: png_size_t,
) { unsafe {
    if png_ptr.is_null() {
        return;
    }
    if !data.is_null() && length > 0 as png_size_t {
        png_write_data(png_ptr, data, length);
        png_calculate_crc(png_ptr, data, length);
    }
} }
#[inline]
pub fn png_write_chunk_end(mut png_ptr: png_structrp) { unsafe {
    let mut buf: [png_byte; 4] = [0; 4];
    if png_ptr.is_null() {
        return;
    }
    (*png_ptr).io_state = (PNG_IO_WRITING | PNG_IO_CHUNK_CRC) as png_uint_32;
    png_save_uint_32(&raw mut buf as png_bytep, (*png_ptr).crc);
    png_write_data(
        png_ptr,
        &raw mut buf as *mut png_byte as png_const_bytep,
        4 as c_int as png_size_t,
    );
} }
fn png_write_complete_chunk(
    mut png_ptr: png_structrp,
    mut chunk_name: png_uint_32,
    mut data: png_const_bytep,
    mut length: png_size_t,
) { unsafe {
    if png_ptr.is_null() {
        return;
    }
    if length > PNG_UINT_31_MAX as png_size_t {
        png_error(
            png_ptr,
            b"length exceeds PNG maximum\0" as *const u8 as png_const_charp,
        );
    }
    png_write_chunk_header(png_ptr, chunk_name, length as png_uint_32);
    png_write_chunk_data(png_ptr, data, length);
    png_write_chunk_end(png_ptr);
} }
#[no_mangle]
pub extern "C" fn png_write_chunk(
    mut png_ptr: png_structrp,
    mut chunk_string: png_const_bytep,
    mut data: png_const_bytep,
    mut length: png_size_t,
) { unsafe {
    png_write_complete_chunk(
        png_ptr,
        ((0xff as c_int
            & *chunk_string.offset(0 as c_int as isize) as c_int)
            as png_uint_32)
            << 24 as c_int
            | ((0xff as c_int
                & *chunk_string.offset(1 as c_int as isize) as c_int)
                as png_uint_32)
                << 16 as c_int
            | ((0xff as c_int
                & *chunk_string.offset(2 as c_int as isize) as c_int)
                as png_uint_32)
                << 8 as c_int
            | ((0xff as c_int
                & *chunk_string.offset(3 as c_int as isize) as c_int)
                as png_uint_32)
                << 0 as c_int,
        data,
        length,
    );
} }
fn png_image_size(mut png_ptr: png_structrp) -> png_alloc_size_t { unsafe {
    let mut h: png_uint_32 = (*png_ptr).height;
    if (*png_ptr).rowbytes < 32768 as png_size_t && h < 32768 as c_uint {
        if (*png_ptr).interlaced as c_int != 0 as c_int {
            let mut w: png_uint_32 = (*png_ptr).width;
            let mut pd: c_uint = (*png_ptr).pixel_depth as c_uint;
            let mut cb_base: png_alloc_size_t = 0;
            let mut pass: c_int = 0;
            cb_base = 0 as png_alloc_size_t;
            pass = 0 as c_int;
            while pass <= 6 as c_int {
                let mut pw: png_uint_32 = w.wrapping_add(
                    (((1 as c_int)
                        << (if pass > 1 as c_int {
                            7 as c_int - pass >> 1 as c_int
                        } else {
                            3 as c_int
                        }))
                        - 1 as c_int
                        - ((1 as c_int & pass)
                            << 3 as c_int
                                - (pass + 1 as c_int >> 1 as c_int)
                            & 7 as c_int)) as png_uint_32,
                ) >> (if pass > 1 as c_int {
                    7 as c_int - pass >> 1 as c_int
                } else {
                    3 as c_int
                });
                if pw > 0 as c_uint {
                    cb_base = (cb_base as c_ulong).wrapping_add(
                        (if pd >= 8 as c_uint {
                            (pw as png_size_t)
                                .wrapping_mul(pd as png_size_t >> 3 as c_int)
                        } else {
                            (pw as png_size_t)
                                .wrapping_mul(pd as png_size_t)
                                .wrapping_add(7 as png_size_t)
                                >> 3 as c_int
                        })
                        .wrapping_add(1 as png_size_t)
                        .wrapping_mul(
                            ((h as c_uint).wrapping_add(
                                (((1 as c_int)
                                    << (if pass > 2 as c_int {
                                        8 as c_int - pass >> 1 as c_int
                                    } else {
                                        3 as c_int
                                    }))
                                    - 1 as c_int
                                    - ((1 as c_int & !pass)
                                        << 3 as c_int
                                            - (pass >> 1 as c_int)
                                        & 7 as c_int))
                                    as c_uint,
                            ) >> (if pass > 2 as c_int {
                                8 as c_int - pass >> 1 as c_int
                            } else {
                                3 as c_int
                            })) as png_size_t,
                        ) as c_ulong,
                    ) as png_alloc_size_t as png_alloc_size_t;
                }
                pass += 1;
            }
            return cb_base;
        } else {
            return ((*png_ptr).rowbytes as png_alloc_size_t)
                .wrapping_add(1 as png_alloc_size_t)
                .wrapping_mul(h as png_alloc_size_t);
        }
    } else {
        return 0xffffffff as c_uint as png_alloc_size_t;
    };
} }
fn optimize_cmf(mut data: png_bytep, mut data_size: png_alloc_size_t) { unsafe {
    if data_size <= 16384 as png_alloc_size_t {
        let mut z_cmf: c_uint =
            *data.offset(0 as c_int as isize) as c_uint;
        if z_cmf & 0xf as c_uint == 8 as c_uint
            && z_cmf & 0xf0 as c_uint <= 0x70 as c_uint
        {
            let mut z_cinfo: c_uint = 0;
            let mut half_z_window_size: c_uint = 0;
            z_cinfo = z_cmf >> 4 as c_int;
            half_z_window_size =
                (1 as c_uint) << z_cinfo.wrapping_add(7 as c_uint);
            if data_size <= half_z_window_size as png_alloc_size_t {
                let mut tmp: c_uint = 0;
                loop {
                    half_z_window_size >>= 1 as c_int;
                    z_cinfo = z_cinfo.wrapping_sub(1);
                    if !(z_cinfo > 0 as c_uint
                        && data_size <= half_z_window_size as png_alloc_size_t)
                    {
                        break;
                    }
                }
                z_cmf = z_cmf & 0xf as c_uint | z_cinfo << 4 as c_int;
                *data.offset(0 as c_int as isize) = z_cmf as png_byte;
                tmp = (*data.offset(1 as c_int as isize) as c_int
                    & 0xe0 as c_int) as c_uint;
                tmp = tmp.wrapping_add(
                    (0x1f as c_uint).wrapping_sub(
                        (z_cmf << 8 as c_int)
                            .wrapping_add(tmp)
                            .wrapping_rem(0x1f as c_uint),
                    ),
                );
                *data.offset(1 as c_int as isize) = tmp as png_byte;
            }
        }
    }
} }
fn png_deflate_claim(
    mut png_ptr: png_structrp,
    mut owner: png_uint_32,
    mut data_size: png_alloc_size_t,
) -> c_int { unsafe {
    if (*png_ptr).zowner != 0 as c_uint {
        let mut msg: [c_char; 64] = [0; 64];
        *(&raw mut msg as *mut c_char).offset(0 as c_int as isize) =
            (owner as c_uint >> 24 as c_int & 0xff as c_uint)
                as c_char;
        *(&raw mut msg as *mut c_char).offset(1 as c_int as isize) =
            (owner as c_uint >> 16 as c_int & 0xff as c_uint)
                as c_char;
        *(&raw mut msg as *mut c_char).offset(2 as c_int as isize) =
            (owner as c_uint >> 8 as c_int & 0xff as c_uint)
                as c_char;
        *(&raw mut msg as *mut c_char).offset(3 as c_int as isize) =
            (owner as c_uint & 0xff as c_uint) as c_char;
        msg[4 as c_int as usize] = ':' as i32 as c_char;
        msg[5 as c_int as usize] = ' ' as i32 as c_char;
        *(&raw mut msg as *mut c_char)
            .offset(6 as c_int as isize)
            .offset(0 as c_int as isize) =
            ((*png_ptr).zowner as c_uint >> 24 as c_int
                & 0xff as c_uint) as c_char;
        *(&raw mut msg as *mut c_char)
            .offset(6 as c_int as isize)
            .offset(1 as c_int as isize) =
            ((*png_ptr).zowner as c_uint >> 16 as c_int
                & 0xff as c_uint) as c_char;
        *(&raw mut msg as *mut c_char)
            .offset(6 as c_int as isize)
            .offset(2 as c_int as isize) =
            ((*png_ptr).zowner as c_uint >> 8 as c_int
                & 0xff as c_uint) as c_char;
        *(&raw mut msg as *mut c_char)
            .offset(6 as c_int as isize)
            .offset(3 as c_int as isize) = ((*png_ptr).zowner as c_uint
            & 0xff as c_uint)
            as c_char;
        png_safecat(
            &raw mut msg as png_charp,
            ::core::mem::size_of::<[c_char; 64]>() as size_t,
            10 as size_t,
            b" using zstream\0" as *const u8 as png_const_charp,
        );
        png_warning(
            png_ptr,
            &raw mut msg as *mut c_char as png_const_charp,
        );
        if (*png_ptr).zowner == png_IDAT {
            (*png_ptr).zstream.msg = b"in use by IDAT\0" as *const u8 as *const c_char;
            return Z_STREAM_ERROR;
        }
        (*png_ptr).zowner = 0 as png_uint_32;
    }
    let mut level: c_int = (*png_ptr).zlib_level;
    let mut method: c_int = (*png_ptr).zlib_method;
    let mut windowBits: c_int = (*png_ptr).zlib_window_bits;
    let mut memLevel: c_int = (*png_ptr).zlib_mem_level;
    let mut strategy: c_int = 0;
    let mut ret: c_int = 0;
    if owner == png_IDAT {
        if (*png_ptr).flags as c_uint & PNG_FLAG_ZLIB_CUSTOM_STRATEGY
            != 0 as c_uint
        {
            strategy = (*png_ptr).zlib_strategy;
        } else if (*png_ptr).do_filter as c_int != PNG_FILTER_NONE {
            strategy = PNG_Z_DEFAULT_STRATEGY;
        } else {
            strategy = PNG_Z_DEFAULT_NOFILTER_STRATEGY;
        }
    } else {
        strategy = Z_DEFAULT_STRATEGY;
    }
    if data_size <= 16384 as png_alloc_size_t {
        let mut half_window_size: c_uint =
            (1 as c_uint) << windowBits - 1 as c_int;
        while data_size.wrapping_add(262 as png_alloc_size_t)
            <= half_window_size as png_alloc_size_t
        {
            half_window_size >>= 1 as c_int;
            windowBits -= 1;
        }
    }
    if (*png_ptr).flags as c_uint & PNG_FLAG_ZSTREAM_INITIALIZED
        != 0 as c_uint
        && ((*png_ptr).zlib_set_level != level
            || (*png_ptr).zlib_set_method != method
            || (*png_ptr).zlib_set_window_bits != windowBits
            || (*png_ptr).zlib_set_mem_level != memLevel
            || (*png_ptr).zlib_set_strategy != strategy)
    {
        if deflateEnd(&raw mut (*png_ptr).zstream) != Z_OK {
            png_warning(
                png_ptr,
                b"deflateEnd failed (ignored)\0" as *const u8 as png_const_charp,
            );
        }
        (*png_ptr).flags &= !PNG_FLAG_ZSTREAM_INITIALIZED;
    }
    (*png_ptr).zstream.next_in = ::core::ptr::null::<Bytef>();
    (*png_ptr).zstream.avail_in = 0 as uInt;
    (*png_ptr).zstream.next_out = ::core::ptr::null_mut::<Bytef>();
    (*png_ptr).zstream.avail_out = 0 as uInt;
    if (*png_ptr).flags as c_uint & PNG_FLAG_ZSTREAM_INITIALIZED
        != 0 as c_uint
    {
        ret = deflateReset(&raw mut (*png_ptr).zstream);
    } else {
        ret = deflateInit2_(
            &raw mut (*png_ptr).zstream,
            level,
            method,
            windowBits,
            memLevel,
            strategy,
            ZLIB_VERSION.as_ptr(),
            ::core::mem::size_of::<z_stream>() as c_int,
        );
        if ret == Z_OK {
            (*png_ptr).flags |= PNG_FLAG_ZSTREAM_INITIALIZED;
        }
    }
    if ret == Z_OK {
        (*png_ptr).zowner = owner;
    } else {
        png_zstream_error(png_ptr, ret);
    }
    return ret;
} }
#[no_mangle]
pub unsafe extern "C" fn png_free_buffer_list(
    mut png_ptr: png_structrp,
    mut listp: *mut png_compression_bufferp,
) {
    let mut list: png_compression_bufferp = *listp;
    if !list.is_null() {
        *listp = ::core::ptr::null_mut::<png_compression_buffer>();
        loop {
            let mut next: png_compression_bufferp = (*list).next as png_compression_bufferp;
            png_free(png_ptr, list as png_voidp);
            list = next;
            if list.is_null() {
                break;
            }
        }
    }
}
#[no_mangle]
pub extern "C" fn png_write_IHDR(
    mut png_ptr: png_structrp,
    mut width: png_uint_32,
    mut height: png_uint_32,
    mut bit_depth: c_int,
    mut color_type: c_int,
    mut compression_type: c_int,
    mut filter_type: c_int,
    mut interlace_type: c_int,
) { unsafe {
    let mut buf: [png_byte; 13] = [0; 13];
    let mut is_invalid_depth: c_int = 0;
    match color_type {
        PNG_COLOR_TYPE_GRAY => match bit_depth {
            1 | 2 | 4 | 8 | 16 => {
                (*png_ptr).channels = 1 as png_byte;
            }
            _ => {
                png_error(
                    png_ptr,
                    b"Invalid bit depth for grayscale image\0" as *const u8 as png_const_charp,
                );
            }
        },
        PNG_COLOR_TYPE_RGB => {
            is_invalid_depth = (bit_depth != 8 as c_int) as c_int;
            is_invalid_depth = (is_invalid_depth != 0 && bit_depth != 16 as c_int)
                as c_int;
            if is_invalid_depth != 0 {
                png_error(
                    png_ptr,
                    b"Invalid bit depth for RGB image\0" as *const u8 as png_const_charp,
                );
            }
            (*png_ptr).channels = 3 as png_byte;
        }
        PNG_COLOR_TYPE_PALETTE => match bit_depth {
            1 | 2 | 4 | 8 => {
                (*png_ptr).channels = 1 as png_byte;
            }
            _ => {
                png_error(
                    png_ptr,
                    b"Invalid bit depth for paletted image\0" as *const u8 as png_const_charp,
                );
            }
        },
        PNG_COLOR_TYPE_GRAY_ALPHA => {
            is_invalid_depth = (bit_depth != 8 as c_int) as c_int;
            is_invalid_depth = (is_invalid_depth != 0 && bit_depth != 16 as c_int)
                as c_int;
            if is_invalid_depth != 0 {
                png_error(
                    png_ptr,
                    b"Invalid bit depth for grayscale+alpha image\0" as *const u8
                        as png_const_charp,
                );
            }
            (*png_ptr).channels = 2 as png_byte;
        }
        PNG_COLOR_TYPE_RGB_ALPHA => {
            is_invalid_depth = (bit_depth != 8 as c_int) as c_int;
            is_invalid_depth = (is_invalid_depth != 0 && bit_depth != 16 as c_int)
                as c_int;
            if is_invalid_depth != 0 {
                png_error(
                    png_ptr,
                    b"Invalid bit depth for RGBA image\0" as *const u8 as png_const_charp,
                );
            }
            (*png_ptr).channels = 4 as png_byte;
        }
        _ => {
            png_error(
                png_ptr,
                b"Invalid image color type specified\0" as *const u8 as png_const_charp,
            );
        }
    }
    if compression_type != PNG_COMPRESSION_TYPE_BASE {
        png_warning(
            png_ptr,
            b"Invalid compression type specified\0" as *const u8 as png_const_charp,
        );
        compression_type = PNG_COMPRESSION_TYPE_BASE;
    }
    if filter_type != PNG_FILTER_TYPE_BASE {
        png_warning(
            png_ptr,
            b"Invalid filter type specified\0" as *const u8 as png_const_charp,
        );
        filter_type = PNG_FILTER_TYPE_BASE;
    }
    if interlace_type != PNG_INTERLACE_NONE && interlace_type != PNG_INTERLACE_ADAM7 {
        png_warning(
            png_ptr,
            b"Invalid interlace type specified\0" as *const u8 as png_const_charp,
        );
        interlace_type = PNG_INTERLACE_ADAM7;
    }
    (*png_ptr).bit_depth = bit_depth as png_byte;
    (*png_ptr).color_type = color_type as png_byte;
    (*png_ptr).interlaced = interlace_type as png_byte;
    (*png_ptr).compression_type = compression_type as png_byte;
    (*png_ptr).width = width;
    (*png_ptr).height = height;
    (*png_ptr).pixel_depth = (bit_depth * (*png_ptr).channels as c_int) as png_byte;
    (*png_ptr).rowbytes = if (*png_ptr).pixel_depth as c_int >= 8 as c_int
    {
        (width as png_size_t)
            .wrapping_mul((*png_ptr).pixel_depth as png_size_t >> 3 as c_int)
    } else {
        (width as png_size_t)
            .wrapping_mul((*png_ptr).pixel_depth as png_size_t)
            .wrapping_add(7 as png_size_t)
            >> 3 as c_int
    };
    (*png_ptr).usr_width = (*png_ptr).width;
    (*png_ptr).usr_bit_depth = (*png_ptr).bit_depth;
    (*png_ptr).usr_channels = (*png_ptr).channels;
    png_save_uint_32(&raw mut buf as png_bytep, width);
    png_save_uint_32(
        (&raw mut buf as *mut png_byte).offset(4 as c_int as isize),
        height,
    );
    buf[8 as c_int as usize] = bit_depth as png_byte;
    buf[9 as c_int as usize] = color_type as png_byte;
    buf[10 as c_int as usize] = compression_type as png_byte;
    buf[11 as c_int as usize] = filter_type as png_byte;
    buf[12 as c_int as usize] = interlace_type as png_byte;
    png_write_complete_chunk(
        png_ptr,
        png_IHDR,
        &raw mut buf as *mut png_byte as png_const_bytep,
        13 as c_int as png_size_t,
    );
    if (*png_ptr).do_filter as c_int == PNG_NO_FILTERS {
        if (*png_ptr).color_type as c_int == PNG_COLOR_TYPE_PALETTE
            || ((*png_ptr).bit_depth as c_int) < 8 as c_int
        {
            (*png_ptr).do_filter = PNG_FILTER_NONE as png_byte;
        } else {
            (*png_ptr).do_filter = PNG_ALL_FILTERS as png_byte;
        }
    }
    (*png_ptr).mode = PNG_HAVE_IHDR as png_uint_32;
} }
#[no_mangle]
pub extern "C" fn png_write_PLTE(
    mut png_ptr: png_structrp,
    mut palette: png_const_colorp,
    mut num_pal: png_uint_32,
) { unsafe {
    let mut max_palette_length: png_uint_32 = 0;
    let mut i: png_uint_32 = 0;
    let mut pal_ptr: png_const_colorp = ::core::ptr::null::<png_color>();
    let mut buf: [png_byte; 3] = [0; 3];
    max_palette_length = (if (*png_ptr).color_type as c_int == PNG_COLOR_TYPE_PALETTE {
        (1 as c_int) << (*png_ptr).bit_depth as c_int
    } else {
        PNG_MAX_PALETTE_LENGTH
    }) as png_uint_32;
    if num_pal == 0 as c_uint || num_pal > max_palette_length {
        if (*png_ptr).color_type as c_int == PNG_COLOR_TYPE_PALETTE {
            png_error(
                png_ptr,
                b"Invalid number of colors in palette\0" as *const u8 as png_const_charp,
            );
        } else {
            png_warning(
                png_ptr,
                b"Invalid number of colors in palette\0" as *const u8 as png_const_charp,
            );
            return;
        }
    }
    if (*png_ptr).color_type as c_int & PNG_COLOR_MASK_COLOR == 0 as c_int
    {
        png_warning(
            png_ptr,
            b"Ignoring request to write a PLTE chunk in grayscale PNG\0" as *const u8
                as png_const_charp,
        );
        return;
    }
    (*png_ptr).num_palette = num_pal as png_uint_16;
    png_write_chunk_header(
        png_ptr,
        png_PLTE,
        (num_pal as c_uint).wrapping_mul(3 as c_uint),
    );
    i = 0 as png_uint_32;
    pal_ptr = palette;
    while i < num_pal {
        buf[0 as c_int as usize] = (*pal_ptr).red;
        buf[1 as c_int as usize] = (*pal_ptr).green;
        buf[2 as c_int as usize] = (*pal_ptr).blue;
        png_write_chunk_data(
            png_ptr,
            &raw mut buf as *mut png_byte as png_const_bytep,
            3 as c_int as png_size_t,
        );
        i = i.wrapping_add(1);
        pal_ptr = pal_ptr.offset(1);
    }
    png_write_chunk_end(png_ptr);
    (*png_ptr).mode |= PNG_HAVE_PLTE as c_uint;
} }
#[no_mangle]
pub extern "C" fn png_compress_IDAT(
    mut png_ptr: png_structrp,
    mut input: png_const_bytep,
    mut input_len: png_alloc_size_t,
    mut flush: c_int,
) { unsafe {
    if (*png_ptr).zowner != png_IDAT {
        if (*png_ptr).zbuffer_list.is_null() {
            (*png_ptr).zbuffer_list = png_malloc(
                png_ptr,
                (8 as png_alloc_size_t).wrapping_add((*png_ptr).zbuffer_size as png_alloc_size_t),
            ) as png_compression_bufferp;
            (*(*png_ptr).zbuffer_list).next = ::core::ptr::null_mut::<png_compression_buffer>();
        } else {
            png_free_buffer_list(png_ptr, &raw mut (*(*png_ptr).zbuffer_list).next);
        }
        if png_deflate_claim(png_ptr, png_IDAT, png_image_size(png_ptr)) != Z_OK {
            png_error(png_ptr, (*png_ptr).zstream.msg as png_const_charp);
        }
        (*png_ptr).zstream.next_out =
            &raw mut (*(*png_ptr).zbuffer_list).output as *mut png_byte as *mut Bytef;
        (*png_ptr).zstream.avail_out = (*png_ptr).zbuffer_size;
    }
    (*png_ptr).zstream.next_in = input as *const Bytef;
    (*png_ptr).zstream.avail_in = 0 as uInt;
    loop {
        let mut ret: c_int = 0;
        let mut avail: uInt = ZLIB_IO_MAX;
        if avail as png_alloc_size_t > input_len {
            avail = input_len as uInt;
        }
        (*png_ptr).zstream.avail_in = avail;
        input_len = (input_len as c_ulong).wrapping_sub(avail as c_ulong)
            as png_alloc_size_t as png_alloc_size_t;
        ret = deflate(
            &raw mut (*png_ptr).zstream,
            if input_len > 0 as png_alloc_size_t {
                Z_NO_FLUSH
            } else {
                flush
            },
        );
        input_len = (input_len as c_ulong)
            .wrapping_add((*png_ptr).zstream.avail_in as c_ulong)
            as png_alloc_size_t as png_alloc_size_t;
        (*png_ptr).zstream.avail_in = 0 as uInt;
        if (*png_ptr).zstream.avail_out == 0 as c_uint {
            let mut data: png_bytep = &raw mut (*(*png_ptr).zbuffer_list).output as png_bytep;
            let mut size: uInt = (*png_ptr).zbuffer_size;
            if (*png_ptr).mode as c_uint & PNG_HAVE_IDAT == 0 as c_uint
                && (*png_ptr).compression_type as c_int == PNG_COMPRESSION_TYPE_BASE
            {
                optimize_cmf(data, png_image_size(png_ptr));
            }
            if size > 0 as c_uint {
                png_write_complete_chunk(
                    png_ptr,
                    png_IDAT,
                    data as png_const_bytep,
                    size as png_size_t,
                );
            }
            (*png_ptr).mode |= PNG_HAVE_IDAT;
            (*png_ptr).zstream.next_out = data as *mut Bytef;
            (*png_ptr).zstream.avail_out = size;
            if ret == Z_OK && flush != Z_NO_FLUSH {
                continue;
            }
        }
        if ret == Z_OK {
            if input_len == 0 as png_alloc_size_t {
                if flush == Z_FINISH {
                    png_error(
                        png_ptr,
                        b"Z_OK on Z_FINISH with output space\0" as *const u8 as png_const_charp,
                    );
                }
                return;
            }
        } else if ret == Z_STREAM_END && flush == Z_FINISH {
            let mut data_0: png_bytep = &raw mut (*(*png_ptr).zbuffer_list).output as png_bytep;
            let mut size_0: uInt = (*png_ptr)
                .zbuffer_size
                .wrapping_sub((*png_ptr).zstream.avail_out);
            if (*png_ptr).mode as c_uint & PNG_HAVE_IDAT == 0 as c_uint
                && (*png_ptr).compression_type as c_int == PNG_COMPRESSION_TYPE_BASE
            {
                optimize_cmf(data_0, png_image_size(png_ptr));
            }
            if size_0 > 0 as c_uint {
                png_write_complete_chunk(
                    png_ptr,
                    png_IDAT,
                    data_0 as png_const_bytep,
                    size_0 as png_size_t,
                );
            }
            (*png_ptr).zstream.avail_out = 0 as uInt;
            (*png_ptr).zstream.next_out = ::core::ptr::null_mut::<Bytef>();
            (*png_ptr).mode |= PNG_HAVE_IDAT | PNG_AFTER_IDAT as c_uint;
            (*png_ptr).zowner = 0 as png_uint_32;
            return;
        } else {
            png_zstream_error(png_ptr, ret);
            png_error(png_ptr, (*png_ptr).zstream.msg as png_const_charp);
        }
    }
} }
#[no_mangle]
pub extern "C" fn png_write_IEND(mut png_ptr: png_structrp) { unsafe {
    png_write_complete_chunk(
        png_ptr,
        png_IEND,
        ::core::ptr::null::<png_byte>(),
        0 as c_int as png_size_t,
    );
    (*png_ptr).mode |= PNG_HAVE_IEND;
} }
#[no_mangle]
pub extern "C" fn png_write_sBIT(
    mut png_ptr: png_structrp,
    mut sbit: png_const_color_8p,
    mut color_type: c_int,
) { unsafe {
    let mut buf: [png_byte; 4] = [0; 4];
    let mut size: png_size_t = 0;
    if color_type & PNG_COLOR_MASK_COLOR != 0 as c_int {
        let mut maxbits: png_byte = 0;
        maxbits = (if color_type == PNG_COLOR_TYPE_PALETTE {
            8 as c_int
        } else {
            (*png_ptr).usr_bit_depth as c_int
        }) as png_byte;
        if (*sbit).red as c_int == 0 as c_int
            || (*sbit).red as c_int > maxbits as c_int
            || (*sbit).green as c_int == 0 as c_int
            || (*sbit).green as c_int > maxbits as c_int
            || (*sbit).blue as c_int == 0 as c_int
            || (*sbit).blue as c_int > maxbits as c_int
        {
            png_warning(
                png_ptr,
                b"Invalid sBIT depth specified\0" as *const u8 as png_const_charp,
            );
            return;
        }
        buf[0 as c_int as usize] = (*sbit).red;
        buf[1 as c_int as usize] = (*sbit).green;
        buf[2 as c_int as usize] = (*sbit).blue;
        size = 3 as png_size_t;
    } else {
        if (*sbit).gray as c_int == 0 as c_int
            || (*sbit).gray as c_int > (*png_ptr).usr_bit_depth as c_int
        {
            png_warning(
                png_ptr,
                b"Invalid sBIT depth specified\0" as *const u8 as png_const_charp,
            );
            return;
        }
        buf[0 as c_int as usize] = (*sbit).gray;
        size = 1 as png_size_t;
    }
    if color_type & PNG_COLOR_MASK_ALPHA != 0 as c_int {
        if (*sbit).alpha as c_int == 0 as c_int
            || (*sbit).alpha as c_int > (*png_ptr).usr_bit_depth as c_int
        {
            png_warning(
                png_ptr,
                b"Invalid sBIT depth specified\0" as *const u8 as png_const_charp,
            );
            return;
        }
        let fresh0 = size;
        size = size.wrapping_add(1);
        buf[fresh0 as usize] = (*sbit).alpha;
    }
    png_write_complete_chunk(
        png_ptr,
        png_sBIT,
        &raw mut buf as *mut png_byte as png_const_bytep,
        size,
    );
} }
#[no_mangle]
pub extern "C" fn png_write_tRNS(
    mut png_ptr: png_structrp,
    mut trans_alpha: png_const_bytep,
    mut tran: png_const_color_16p,
    mut num_trans: c_int,
    mut color_type: c_int,
) { unsafe {
    let mut buf: [png_byte; 6] = [0; 6];
    if color_type == PNG_COLOR_TYPE_PALETTE {
        if num_trans <= 0 as c_int
            || num_trans > (*png_ptr).num_palette as c_int
        {
            png_app_warning(
                png_ptr,
                b"Invalid number of transparent colors specified\0" as *const u8 as png_const_charp,
            );
            return;
        }
        png_write_complete_chunk(png_ptr, png_tRNS, trans_alpha, num_trans as png_size_t);
    } else if color_type == PNG_COLOR_TYPE_GRAY {
        if (*tran).gray as c_int
            >= (1 as c_int) << (*png_ptr).bit_depth as c_int
        {
            png_app_warning(
                png_ptr,
                b"Ignoring attempt to write tRNS chunk out-of-range for bit_depth\0" as *const u8
                    as png_const_charp,
            );
            return;
        }
        png_save_uint_16(
            &raw mut buf as png_bytep,
            (*tran).gray as c_uint,
        );
        png_write_complete_chunk(
            png_ptr,
            png_tRNS,
            &raw mut buf as *mut png_byte as png_const_bytep,
            2 as c_int as png_size_t,
        );
    } else if color_type == PNG_COLOR_TYPE_RGB {
        png_save_uint_16(
            &raw mut buf as png_bytep,
            (*tran).red as c_uint,
        );
        png_save_uint_16(
            (&raw mut buf as *mut png_byte).offset(2 as c_int as isize),
            (*tran).green as c_uint,
        );
        png_save_uint_16(
            (&raw mut buf as *mut png_byte).offset(4 as c_int as isize),
            (*tran).blue as c_uint,
        );
        if (*png_ptr).bit_depth as c_int == 8 as c_int
            && buf[0 as c_int as usize] as c_int
                | buf[2 as c_int as usize] as c_int
                | buf[4 as c_int as usize] as c_int
                != 0 as c_int
        {
            png_app_warning(
                png_ptr,
                b"Ignoring attempt to write 16-bit tRNS chunk when bit_depth is 8\0" as *const u8
                    as png_const_charp,
            );
            return;
        }
        png_write_complete_chunk(
            png_ptr,
            png_tRNS,
            &raw mut buf as *mut png_byte as png_const_bytep,
            6 as c_int as png_size_t,
        );
    } else {
        png_app_warning(
            png_ptr,
            b"Can't write tRNS with an alpha channel\0" as *const u8 as png_const_charp,
        );
    };
} }
#[no_mangle]
pub extern "C" fn png_write_bKGD(
    mut png_ptr: png_structrp,
    mut back: png_const_color_16p,
    mut color_type: c_int,
) { unsafe {
    let mut buf: [png_byte; 6] = [0; 6];
    if color_type == PNG_COLOR_TYPE_PALETTE {
        if (*back).index as c_int >= (*png_ptr).num_palette as c_int {
            png_warning(
                png_ptr,
                b"Invalid background palette index\0" as *const u8 as png_const_charp,
            );
            return;
        }
        buf[0 as c_int as usize] = (*back).index;
        png_write_complete_chunk(
            png_ptr,
            png_bKGD,
            &raw mut buf as *mut png_byte as png_const_bytep,
            1 as c_int as png_size_t,
        );
    } else if color_type & PNG_COLOR_MASK_COLOR != 0 as c_int {
        png_save_uint_16(
            &raw mut buf as png_bytep,
            (*back).red as c_uint,
        );
        png_save_uint_16(
            (&raw mut buf as *mut png_byte).offset(2 as c_int as isize),
            (*back).green as c_uint,
        );
        png_save_uint_16(
            (&raw mut buf as *mut png_byte).offset(4 as c_int as isize),
            (*back).blue as c_uint,
        );
        if (*png_ptr).bit_depth as c_int == 8 as c_int
            && buf[0 as c_int as usize] as c_int
                | buf[2 as c_int as usize] as c_int
                | buf[4 as c_int as usize] as c_int
                != 0 as c_int
        {
            png_warning(
                png_ptr,
                b"Ignoring attempt to write 16-bit bKGD chunk when bit_depth is 8\0" as *const u8
                    as png_const_charp,
            );
            return;
        }
        png_write_complete_chunk(
            png_ptr,
            png_bKGD,
            &raw mut buf as *mut png_byte as png_const_bytep,
            6 as c_int as png_size_t,
        );
    } else {
        if (*back).gray as c_int
            >= (1 as c_int) << (*png_ptr).bit_depth as c_int
        {
            png_warning(
                png_ptr,
                b"Ignoring attempt to write bKGD chunk out-of-range for bit_depth\0" as *const u8
                    as png_const_charp,
            );
            return;
        }
        png_save_uint_16(
            &raw mut buf as png_bytep,
            (*back).gray as c_uint,
        );
        png_write_complete_chunk(
            png_ptr,
            png_bKGD,
            &raw mut buf as *mut png_byte as png_const_bytep,
            2 as c_int as png_size_t,
        );
    };
} }
#[no_mangle]
pub extern "C" fn png_write_hIST(
    mut png_ptr: png_structrp,
    mut hist: png_const_uint_16p,
    mut num_hist: c_int,
) { unsafe {
    let mut i: c_int = 0;
    let mut buf: [png_byte; 3] = [0; 3];
    if num_hist > (*png_ptr).num_palette as c_int {
        png_warning(
            png_ptr,
            b"Invalid number of histogram entries specified\0" as *const u8 as png_const_charp,
        );
        return;
    }
    png_write_chunk_header(
        png_ptr,
        png_hIST,
        (num_hist * 2 as c_int) as png_uint_32,
    );
    i = 0 as c_int;
    while i < num_hist {
        png_save_uint_16(
            &raw mut buf as png_bytep,
            *hist.offset(i as isize) as c_uint,
        );
        png_write_chunk_data(
            png_ptr,
            &raw mut buf as *mut png_byte as png_const_bytep,
            2 as c_int as png_size_t,
        );
        i += 1;
    }
    png_write_chunk_end(png_ptr);
} }
#[no_mangle]
pub extern "C" fn png_write_start_row(mut png_ptr: png_structrp) { unsafe {
    static mut png_pass_start: [png_byte; 7] = [
        0 as c_int as png_byte,
        4 as c_int as png_byte,
        0 as c_int as png_byte,
        2 as c_int as png_byte,
        0 as c_int as png_byte,
        1 as c_int as png_byte,
        0 as c_int as png_byte,
    ];
    static mut png_pass_inc: [png_byte; 7] = [
        8 as c_int as png_byte,
        8 as c_int as png_byte,
        4 as c_int as png_byte,
        4 as c_int as png_byte,
        2 as c_int as png_byte,
        2 as c_int as png_byte,
        1 as c_int as png_byte,
    ];
    static mut png_pass_ystart: [png_byte; 7] = [
        0 as c_int as png_byte,
        0 as c_int as png_byte,
        4 as c_int as png_byte,
        0 as c_int as png_byte,
        2 as c_int as png_byte,
        0 as c_int as png_byte,
        1 as c_int as png_byte,
    ];
    static mut png_pass_yinc: [png_byte; 7] = [
        8 as c_int as png_byte,
        8 as c_int as png_byte,
        8 as c_int as png_byte,
        4 as c_int as png_byte,
        4 as c_int as png_byte,
        2 as c_int as png_byte,
        2 as c_int as png_byte,
    ];
    let mut buf_size: png_alloc_size_t = 0;
    let mut usr_pixel_depth: c_int = 0;
    let mut filters: png_byte = 0;
    usr_pixel_depth = (*png_ptr).usr_channels as c_int
        * (*png_ptr).usr_bit_depth as c_int;
    buf_size = (if usr_pixel_depth >= 8 as c_int {
        ((*png_ptr).width as png_size_t)
            .wrapping_mul(usr_pixel_depth as png_size_t >> 3 as c_int)
    } else {
        ((*png_ptr).width as png_size_t)
            .wrapping_mul(usr_pixel_depth as png_size_t)
            .wrapping_add(7 as png_size_t)
            >> 3 as c_int
    })
    .wrapping_add(1 as png_size_t) as png_alloc_size_t;
    (*png_ptr).transformed_pixel_depth = (*png_ptr).pixel_depth;
    (*png_ptr).maximum_pixel_depth = usr_pixel_depth as png_byte;
    (*png_ptr).row_buf = png_malloc(png_ptr, buf_size) as png_bytep;
    *(*png_ptr).row_buf.offset(0 as c_int as isize) =
        PNG_FILTER_VALUE_NONE as png_byte;
    filters = (*png_ptr).do_filter;
    if (*png_ptr).height == 1 as c_uint {
        filters = (filters as c_int
            & (0xff as c_int & !(PNG_FILTER_UP | PNG_FILTER_AVG | PNG_FILTER_PAETH)))
            as png_byte;
    }
    if (*png_ptr).width == 1 as c_uint {
        filters = (filters as c_int
            & (0xff as c_int & !(PNG_FILTER_SUB | PNG_FILTER_AVG | PNG_FILTER_PAETH)))
            as png_byte;
    }
    if filters as c_int == 0 as c_int {
        filters = PNG_FILTER_NONE as png_byte;
    }
    (*png_ptr).do_filter = filters;
    if filters as c_int
        & (PNG_FILTER_SUB | PNG_FILTER_UP | PNG_FILTER_AVG | PNG_FILTER_PAETH)
        != 0 as c_int
        && (*png_ptr).try_row.is_null()
    {
        let mut num_filters: c_int = 0 as c_int;
        (*png_ptr).try_row = png_malloc(png_ptr, buf_size) as png_bytep;
        if filters as c_int & PNG_FILTER_SUB != 0 {
            num_filters += 1;
        }
        if filters as c_int & PNG_FILTER_UP != 0 {
            num_filters += 1;
        }
        if filters as c_int & PNG_FILTER_AVG != 0 {
            num_filters += 1;
        }
        if filters as c_int & PNG_FILTER_PAETH != 0 {
            num_filters += 1;
        }
        if num_filters > 1 as c_int {
            (*png_ptr).tst_row = png_malloc(png_ptr, buf_size) as png_bytep;
        }
    }
    if filters as c_int & (PNG_FILTER_AVG | PNG_FILTER_UP | PNG_FILTER_PAETH)
        != 0 as c_int
    {
        (*png_ptr).prev_row = png_calloc(png_ptr, buf_size) as png_bytep;
    }
    if (*png_ptr).interlaced as c_int != 0 as c_int {
        if (*png_ptr).transformations as c_uint & PNG_INTERLACE
            == 0 as c_uint
        {
            (*png_ptr).num_rows = ((*png_ptr).height as c_uint)
                .wrapping_add(
                    png_pass_yinc[0 as c_int as usize] as c_uint,
                )
                .wrapping_sub(1 as c_uint)
                .wrapping_sub(
                    png_pass_ystart[0 as c_int as usize] as c_uint,
                )
                .wrapping_div(
                    png_pass_yinc[0 as c_int as usize] as c_uint,
                ) as png_uint_32;
            (*png_ptr).usr_width = ((*png_ptr).width as c_uint)
                .wrapping_add(png_pass_inc[0 as c_int as usize] as c_uint)
                .wrapping_sub(1 as c_uint)
                .wrapping_sub(
                    png_pass_start[0 as c_int as usize] as c_uint,
                )
                .wrapping_div(png_pass_inc[0 as c_int as usize] as c_uint)
                as png_uint_32;
        } else {
            (*png_ptr).num_rows = (*png_ptr).height;
            (*png_ptr).usr_width = (*png_ptr).width;
        }
    } else {
        (*png_ptr).num_rows = (*png_ptr).height;
        (*png_ptr).usr_width = (*png_ptr).width;
    };
} }
#[no_mangle]
pub extern "C" fn png_write_finish_row(mut png_ptr: png_structrp) { unsafe {
    static mut png_pass_start: [png_byte; 7] = [
        0 as c_int as png_byte,
        4 as c_int as png_byte,
        0 as c_int as png_byte,
        2 as c_int as png_byte,
        0 as c_int as png_byte,
        1 as c_int as png_byte,
        0 as c_int as png_byte,
    ];
    static mut png_pass_inc: [png_byte; 7] = [
        8 as c_int as png_byte,
        8 as c_int as png_byte,
        4 as c_int as png_byte,
        4 as c_int as png_byte,
        2 as c_int as png_byte,
        2 as c_int as png_byte,
        1 as c_int as png_byte,
    ];
    static mut png_pass_ystart: [png_byte; 7] = [
        0 as c_int as png_byte,
        0 as c_int as png_byte,
        4 as c_int as png_byte,
        0 as c_int as png_byte,
        2 as c_int as png_byte,
        0 as c_int as png_byte,
        1 as c_int as png_byte,
    ];
    static mut png_pass_yinc: [png_byte; 7] = [
        8 as c_int as png_byte,
        8 as c_int as png_byte,
        8 as c_int as png_byte,
        4 as c_int as png_byte,
        4 as c_int as png_byte,
        2 as c_int as png_byte,
        2 as c_int as png_byte,
    ];
    (*png_ptr).row_number = (*png_ptr).row_number.wrapping_add(1);
    if (*png_ptr).row_number < (*png_ptr).num_rows {
        return;
    }
    if (*png_ptr).interlaced as c_int != 0 as c_int {
        (*png_ptr).row_number = 0 as png_uint_32;
        if (*png_ptr).transformations as c_uint & PNG_INTERLACE
            != 0 as c_uint
        {
            (*png_ptr).pass = (*png_ptr).pass.wrapping_add(1);
        } else {
            loop {
                (*png_ptr).pass = (*png_ptr).pass.wrapping_add(1);
                if (*png_ptr).pass as c_int >= 7 as c_int {
                    break;
                }
                (*png_ptr).usr_width = ((*png_ptr).width as c_uint)
                    .wrapping_add(png_pass_inc[(*png_ptr).pass as usize] as c_uint)
                    .wrapping_sub(1 as c_uint)
                    .wrapping_sub(png_pass_start[(*png_ptr).pass as usize] as c_uint)
                    .wrapping_div(png_pass_inc[(*png_ptr).pass as usize] as c_uint)
                    as png_uint_32;
                (*png_ptr).num_rows = ((*png_ptr).height as c_uint)
                    .wrapping_add(png_pass_yinc[(*png_ptr).pass as usize] as c_uint)
                    .wrapping_sub(1 as c_uint)
                    .wrapping_sub(png_pass_ystart[(*png_ptr).pass as usize] as c_uint)
                    .wrapping_div(png_pass_yinc[(*png_ptr).pass as usize] as c_uint)
                    as png_uint_32;
                if (*png_ptr).transformations as c_uint & PNG_INTERLACE
                    != 0 as c_uint
                {
                    break;
                }
                if !((*png_ptr).usr_width == 0 as c_uint
                    || (*png_ptr).num_rows == 0 as c_uint)
                {
                    break;
                }
            }
        }
        if ((*png_ptr).pass as c_int) < 7 as c_int {
            if !(*png_ptr).prev_row.is_null() {
                memset(
                    (*png_ptr).prev_row as *mut c_void,
                    0 as c_int,
                    (if (*png_ptr).usr_channels as c_int
                        * (*png_ptr).usr_bit_depth as c_int
                        >= 8 as c_int
                    {
                        ((*png_ptr).width as png_size_t).wrapping_mul(
                            ((*png_ptr).usr_channels as c_int
                                * (*png_ptr).usr_bit_depth as c_int)
                                as png_size_t
                                >> 3 as c_int,
                        )
                    } else {
                        ((*png_ptr).width as png_size_t)
                            .wrapping_mul(
                                ((*png_ptr).usr_channels as c_int
                                    * (*png_ptr).usr_bit_depth as c_int)
                                    as png_size_t,
                            )
                            .wrapping_add(7 as png_size_t)
                            >> 3 as c_int
                    })
                    .wrapping_add(1 as size_t),
                );
            }
            return;
        }
    }
    png_compress_IDAT(
        png_ptr,
        ::core::ptr::null::<png_byte>(),
        0 as png_alloc_size_t,
        Z_FINISH,
    );
} }
#[inline]
pub fn png_do_write_interlace(
    mut row_info: png_row_infop,
    mut row: png_bytep,
    mut pass: c_int,
) { unsafe {
    static mut png_pass_start: [png_byte; 7] = [
        0 as c_int as png_byte,
        4 as c_int as png_byte,
        0 as c_int as png_byte,
        2 as c_int as png_byte,
        0 as c_int as png_byte,
        1 as c_int as png_byte,
        0 as c_int as png_byte,
    ];
    static mut png_pass_inc: [png_byte; 7] = [
        8 as c_int as png_byte,
        8 as c_int as png_byte,
        4 as c_int as png_byte,
        4 as c_int as png_byte,
        2 as c_int as png_byte,
        2 as c_int as png_byte,
        1 as c_int as png_byte,
    ];
    if pass < 6 as c_int {
        match (*row_info).pixel_depth as c_int {
            1 => {
                let mut sp: png_bytep = ::core::ptr::null_mut::<png_byte>();
                let mut dp: png_bytep = ::core::ptr::null_mut::<png_byte>();
                let mut shift: c_uint = 0;
                let mut d: c_int = 0;
                let mut value: c_int = 0;
                let mut i: png_uint_32 = 0;
                let mut row_width: png_uint_32 = (*row_info).width;
                dp = row;
                d = 0 as c_int;
                shift = 7 as c_uint;
                i = png_pass_start[pass as usize] as png_uint_32;
                while i < row_width {
                    sp = row.offset((i >> 3 as c_int) as png_size_t as isize);
                    value = *sp as c_int
                        >> 7 as c_int
                            - (i as c_uint & 0x7 as c_uint)
                                as c_int
                        & 0x1 as c_int;
                    d |= value << shift;
                    if shift == 0 as c_uint {
                        shift = 7 as c_uint;
                        let fresh1 = dp;
                        dp = dp.offset(1);
                        *fresh1 = d as png_byte;
                        d = 0 as c_int;
                    } else {
                        shift = shift.wrapping_sub(1);
                    }
                    i = (i as c_uint)
                        .wrapping_add(png_pass_inc[pass as usize] as c_uint)
                        as png_uint_32 as png_uint_32;
                }
                if shift != 7 as c_uint {
                    *dp = d as png_byte;
                }
            }
            2 => {
                let mut sp_0: png_bytep = ::core::ptr::null_mut::<png_byte>();
                let mut dp_0: png_bytep = ::core::ptr::null_mut::<png_byte>();
                let mut shift_0: c_uint = 0;
                let mut d_0: c_int = 0;
                let mut value_0: c_int = 0;
                let mut i_0: png_uint_32 = 0;
                let mut row_width_0: png_uint_32 = (*row_info).width;
                dp_0 = row;
                shift_0 = 6 as c_uint;
                d_0 = 0 as c_int;
                i_0 = png_pass_start[pass as usize] as png_uint_32;
                while i_0 < row_width_0 {
                    sp_0 = row.offset((i_0 >> 2 as c_int) as png_size_t as isize);
                    value_0 = *sp_0 as c_int
                        >> ((3 as c_int
                            - (i_0 as c_uint & 0x3 as c_uint)
                                as c_int)
                            << 1 as c_int)
                        & 0x3 as c_int;
                    d_0 |= value_0 << shift_0;
                    if shift_0 == 0 as c_uint {
                        shift_0 = 6 as c_uint;
                        let fresh2 = dp_0;
                        dp_0 = dp_0.offset(1);
                        *fresh2 = d_0 as png_byte;
                        d_0 = 0 as c_int;
                    } else {
                        shift_0 = shift_0.wrapping_sub(2 as c_uint);
                    }
                    i_0 = (i_0 as c_uint)
                        .wrapping_add(png_pass_inc[pass as usize] as c_uint)
                        as png_uint_32 as png_uint_32;
                }
                if shift_0 != 6 as c_uint {
                    *dp_0 = d_0 as png_byte;
                }
            }
            4 => {
                let mut sp_1: png_bytep = ::core::ptr::null_mut::<png_byte>();
                let mut dp_1: png_bytep = ::core::ptr::null_mut::<png_byte>();
                let mut shift_1: c_uint = 0;
                let mut d_1: c_int = 0;
                let mut value_1: c_int = 0;
                let mut i_1: png_uint_32 = 0;
                let mut row_width_1: png_uint_32 = (*row_info).width;
                dp_1 = row;
                shift_1 = 4 as c_uint;
                d_1 = 0 as c_int;
                i_1 = png_pass_start[pass as usize] as png_uint_32;
                while i_1 < row_width_1 {
                    sp_1 = row.offset((i_1 >> 1 as c_int) as png_size_t as isize);
                    value_1 = *sp_1 as c_int
                        >> ((1 as c_int
                            - (i_1 as c_uint & 0x1 as c_uint)
                                as c_int)
                            << 2 as c_int)
                        & 0xf as c_int;
                    d_1 |= value_1 << shift_1;
                    if shift_1 == 0 as c_uint {
                        shift_1 = 4 as c_uint;
                        let fresh3 = dp_1;
                        dp_1 = dp_1.offset(1);
                        *fresh3 = d_1 as png_byte;
                        d_1 = 0 as c_int;
                    } else {
                        shift_1 = shift_1.wrapping_sub(4 as c_uint);
                    }
                    i_1 = (i_1 as c_uint)
                        .wrapping_add(png_pass_inc[pass as usize] as c_uint)
                        as png_uint_32 as png_uint_32;
                }
                if shift_1 != 4 as c_uint {
                    *dp_1 = d_1 as png_byte;
                }
            }
            _ => {
                let mut sp_2: png_bytep = ::core::ptr::null_mut::<png_byte>();
                let mut dp_2: png_bytep = ::core::ptr::null_mut::<png_byte>();
                let mut i_2: png_uint_32 = 0;
                let mut row_width_2: png_uint_32 = (*row_info).width;
                let mut pixel_bytes: png_size_t = 0;
                dp_2 = row;
                pixel_bytes = ((*row_info).pixel_depth as c_int
                    >> 3 as c_int) as png_size_t;
                i_2 = png_pass_start[pass as usize] as png_uint_32;
                while i_2 < row_width_2 {
                    sp_2 = row.offset((i_2 as png_size_t).wrapping_mul(pixel_bytes) as isize);
                    if dp_2 != sp_2 {
                        memcpy(
                            dp_2 as *mut c_void,
                            sp_2 as *const c_void,
                            pixel_bytes as size_t,
                        );
                    }
                    dp_2 = dp_2.offset(pixel_bytes as isize);
                    i_2 = (i_2 as c_uint)
                        .wrapping_add(png_pass_inc[pass as usize] as c_uint)
                        as png_uint_32 as png_uint_32;
                }
            }
        }
        (*row_info).width = ((*row_info).width as c_uint)
            .wrapping_add(png_pass_inc[pass as usize] as c_uint)
            .wrapping_sub(1 as c_uint)
            .wrapping_sub(png_pass_start[pass as usize] as c_uint)
            .wrapping_div(png_pass_inc[pass as usize] as c_uint)
            as png_uint_32;
        (*row_info).rowbytes =
            if (*row_info).pixel_depth as c_int >= 8 as c_int {
                ((*row_info).width as png_size_t)
                    .wrapping_mul((*row_info).pixel_depth as png_size_t >> 3 as c_int)
            } else {
                ((*row_info).width as png_size_t)
                    .wrapping_mul((*row_info).pixel_depth as png_size_t)
                    .wrapping_add(7 as png_size_t)
                    >> 3 as c_int
            };
    }
} }
fn png_setup_sub_row(
    mut png_ptr: png_structrp,
    bpp: png_uint_32,
    row_bytes: png_size_t,
    lmins: png_size_t,
) -> png_size_t { unsafe {
    let mut rp: png_bytep = ::core::ptr::null_mut::<png_byte>();
    let mut dp: png_bytep = ::core::ptr::null_mut::<png_byte>();
    let mut lp: png_bytep = ::core::ptr::null_mut::<png_byte>();
    let mut i: png_size_t = 0;
    let mut sum: png_size_t = 0 as png_size_t;
    let mut v: c_uint = 0;
    *(*png_ptr).try_row.offset(0 as c_int as isize) = PNG_FILTER_VALUE_SUB as png_byte;
    i = 0 as png_size_t;
    rp = (*png_ptr).row_buf.offset(1 as c_int as isize);
    dp = (*png_ptr).try_row.offset(1 as c_int as isize);
    while i < bpp as png_size_t {
        *dp = *rp;
        v = *dp as c_uint;
        sum = (sum as c_ulong).wrapping_add(
            (if v < 128 as c_uint {
                v
            } else {
                (256 as c_uint).wrapping_sub(v)
            }) as c_ulong,
        ) as png_size_t as png_size_t;
        i = i.wrapping_add(1);
        rp = rp.offset(1);
        dp = dp.offset(1);
    }
    lp = (*png_ptr).row_buf.offset(1 as c_int as isize);
    while i < row_bytes {
        *dp = (*rp as c_int - *lp as c_int & 0xff as c_int)
            as png_byte;
        v = *dp as c_uint;
        sum = (sum as c_ulong).wrapping_add(
            (if v < 128 as c_uint {
                v
            } else {
                (256 as c_uint).wrapping_sub(v)
            }) as c_ulong,
        ) as png_size_t as png_size_t;
        if sum > lmins {
            break;
        }
        i = i.wrapping_add(1);
        rp = rp.offset(1);
        lp = lp.offset(1);
        dp = dp.offset(1);
    }
    return sum;
} }
fn png_setup_sub_row_only(
    mut png_ptr: png_structrp,
    bpp: png_uint_32,
    row_bytes: png_size_t,
) { unsafe {
    let mut rp: png_bytep = ::core::ptr::null_mut::<png_byte>();
    let mut dp: png_bytep = ::core::ptr::null_mut::<png_byte>();
    let mut lp: png_bytep = ::core::ptr::null_mut::<png_byte>();
    let mut i: png_size_t = 0;
    *(*png_ptr).try_row.offset(0 as c_int as isize) = PNG_FILTER_VALUE_SUB as png_byte;
    i = 0 as png_size_t;
    rp = (*png_ptr).row_buf.offset(1 as c_int as isize);
    dp = (*png_ptr).try_row.offset(1 as c_int as isize);
    while i < bpp as png_size_t {
        *dp = *rp;
        i = i.wrapping_add(1);
        rp = rp.offset(1);
        dp = dp.offset(1);
    }
    lp = (*png_ptr).row_buf.offset(1 as c_int as isize);
    while i < row_bytes {
        *dp = (*rp as c_int - *lp as c_int & 0xff as c_int)
            as png_byte;
        i = i.wrapping_add(1);
        rp = rp.offset(1);
        lp = lp.offset(1);
        dp = dp.offset(1);
    }
} }
fn png_setup_up_row(
    mut png_ptr: png_structrp,
    row_bytes: png_size_t,
    lmins: png_size_t,
) -> png_size_t { unsafe {
    let mut rp: png_bytep = ::core::ptr::null_mut::<png_byte>();
    let mut dp: png_bytep = ::core::ptr::null_mut::<png_byte>();
    let mut pp: png_bytep = ::core::ptr::null_mut::<png_byte>();
    let mut i: png_size_t = 0;
    let mut sum: png_size_t = 0 as png_size_t;
    let mut v: c_uint = 0;
    *(*png_ptr).try_row.offset(0 as c_int as isize) = PNG_FILTER_VALUE_UP as png_byte;
    i = 0 as png_size_t;
    rp = (*png_ptr).row_buf.offset(1 as c_int as isize);
    dp = (*png_ptr).try_row.offset(1 as c_int as isize);
    pp = (*png_ptr).prev_row.offset(1 as c_int as isize);
    while i < row_bytes {
        *dp = (*rp as c_int - *pp as c_int & 0xff as c_int)
            as png_byte;
        v = *dp as c_uint;
        sum = (sum as c_ulong).wrapping_add(
            (if v < 128 as c_uint {
                v
            } else {
                (256 as c_uint).wrapping_sub(v)
            }) as c_ulong,
        ) as png_size_t as png_size_t;
        if sum > lmins {
            break;
        }
        i = i.wrapping_add(1);
        rp = rp.offset(1);
        pp = pp.offset(1);
        dp = dp.offset(1);
    }
    return sum;
} }
fn png_setup_up_row_only(mut png_ptr: png_structrp, row_bytes: png_size_t) { unsafe {
    let mut rp: png_bytep = ::core::ptr::null_mut::<png_byte>();
    let mut dp: png_bytep = ::core::ptr::null_mut::<png_byte>();
    let mut pp: png_bytep = ::core::ptr::null_mut::<png_byte>();
    let mut i: png_size_t = 0;
    *(*png_ptr).try_row.offset(0 as c_int as isize) = PNG_FILTER_VALUE_UP as png_byte;
    i = 0 as png_size_t;
    rp = (*png_ptr).row_buf.offset(1 as c_int as isize);
    dp = (*png_ptr).try_row.offset(1 as c_int as isize);
    pp = (*png_ptr).prev_row.offset(1 as c_int as isize);
    while i < row_bytes {
        *dp = (*rp as c_int - *pp as c_int & 0xff as c_int)
            as png_byte;
        i = i.wrapping_add(1);
        rp = rp.offset(1);
        pp = pp.offset(1);
        dp = dp.offset(1);
    }
} }
fn png_setup_avg_row(
    mut png_ptr: png_structrp,
    bpp: png_uint_32,
    row_bytes: png_size_t,
    lmins: png_size_t,
) -> png_size_t { unsafe {
    let mut rp: png_bytep = ::core::ptr::null_mut::<png_byte>();
    let mut dp: png_bytep = ::core::ptr::null_mut::<png_byte>();
    let mut pp: png_bytep = ::core::ptr::null_mut::<png_byte>();
    let mut lp: png_bytep = ::core::ptr::null_mut::<png_byte>();
    let mut i: png_uint_32 = 0;
    let mut sum: png_size_t = 0 as png_size_t;
    let mut v: c_uint = 0;
    *(*png_ptr).try_row.offset(0 as c_int as isize) = PNG_FILTER_VALUE_AVG as png_byte;
    i = 0 as png_uint_32;
    rp = (*png_ptr).row_buf.offset(1 as c_int as isize);
    dp = (*png_ptr).try_row.offset(1 as c_int as isize);
    pp = (*png_ptr).prev_row.offset(1 as c_int as isize);
    while i < bpp {
        let fresh20 = rp;
        rp = rp.offset(1);
        let fresh21 = pp;
        pp = pp.offset(1);
        let fresh22 = dp;
        dp = dp.offset(1);
        *fresh22 = (*fresh20 as c_int
            - *fresh21 as c_int / 2 as c_int
            & 0xff as c_int) as png_byte;
        v = *fresh22 as c_uint;
        sum = (sum as c_ulong).wrapping_add(
            (if v < 128 as c_uint {
                v
            } else {
                (256 as c_uint).wrapping_sub(v)
            }) as c_ulong,
        ) as png_size_t as png_size_t;
        i = i.wrapping_add(1);
    }
    lp = (*png_ptr).row_buf.offset(1 as c_int as isize);
    while (i as png_size_t) < row_bytes {
        let fresh23 = rp;
        rp = rp.offset(1);
        let fresh24 = pp;
        pp = pp.offset(1);
        let fresh25 = lp;
        lp = lp.offset(1);
        let fresh26 = dp;
        dp = dp.offset(1);
        *fresh26 = (*fresh23 as c_int
            - (*fresh24 as c_int + *fresh25 as c_int)
                / 2 as c_int
            & 0xff as c_int) as png_byte;
        v = *fresh26 as c_uint;
        sum = (sum as c_ulong).wrapping_add(
            (if v < 128 as c_uint {
                v
            } else {
                (256 as c_uint).wrapping_sub(v)
            }) as c_ulong,
        ) as png_size_t as png_size_t;
        if sum > lmins {
            break;
        }
        i = i.wrapping_add(1);
    }
    return sum;
} }
fn png_setup_avg_row_only(
    mut png_ptr: png_structrp,
    bpp: png_uint_32,
    row_bytes: png_size_t,
) { unsafe {
    let mut rp: png_bytep = ::core::ptr::null_mut::<png_byte>();
    let mut dp: png_bytep = ::core::ptr::null_mut::<png_byte>();
    let mut pp: png_bytep = ::core::ptr::null_mut::<png_byte>();
    let mut lp: png_bytep = ::core::ptr::null_mut::<png_byte>();
    let mut i: png_uint_32 = 0;
    *(*png_ptr).try_row.offset(0 as c_int as isize) = PNG_FILTER_VALUE_AVG as png_byte;
    i = 0 as png_uint_32;
    rp = (*png_ptr).row_buf.offset(1 as c_int as isize);
    dp = (*png_ptr).try_row.offset(1 as c_int as isize);
    pp = (*png_ptr).prev_row.offset(1 as c_int as isize);
    while i < bpp {
        let fresh27 = rp;
        rp = rp.offset(1);
        let fresh28 = pp;
        pp = pp.offset(1);
        let fresh29 = dp;
        dp = dp.offset(1);
        *fresh29 = (*fresh27 as c_int
            - *fresh28 as c_int / 2 as c_int
            & 0xff as c_int) as png_byte;
        i = i.wrapping_add(1);
    }
    lp = (*png_ptr).row_buf.offset(1 as c_int as isize);
    while (i as png_size_t) < row_bytes {
        let fresh30 = rp;
        rp = rp.offset(1);
        let fresh31 = pp;
        pp = pp.offset(1);
        let fresh32 = lp;
        lp = lp.offset(1);
        let fresh33 = dp;
        dp = dp.offset(1);
        *fresh33 = (*fresh30 as c_int
            - (*fresh31 as c_int + *fresh32 as c_int)
                / 2 as c_int
            & 0xff as c_int) as png_byte;
        i = i.wrapping_add(1);
    }
} }
fn png_setup_paeth_row(
    mut png_ptr: png_structrp,
    bpp: png_uint_32,
    row_bytes: png_size_t,
    lmins: png_size_t,
) -> png_size_t { unsafe {
    let mut rp: png_bytep = ::core::ptr::null_mut::<png_byte>();
    let mut dp: png_bytep = ::core::ptr::null_mut::<png_byte>();
    let mut pp: png_bytep = ::core::ptr::null_mut::<png_byte>();
    let mut cp: png_bytep = ::core::ptr::null_mut::<png_byte>();
    let mut lp: png_bytep = ::core::ptr::null_mut::<png_byte>();
    let mut i: png_size_t = 0;
    let mut sum: png_size_t = 0 as png_size_t;
    let mut v: c_uint = 0;
    *(*png_ptr).try_row.offset(0 as c_int as isize) =
        PNG_FILTER_VALUE_PAETH as png_byte;
    i = 0 as png_size_t;
    rp = (*png_ptr).row_buf.offset(1 as c_int as isize);
    dp = (*png_ptr).try_row.offset(1 as c_int as isize);
    pp = (*png_ptr).prev_row.offset(1 as c_int as isize);
    while i < bpp as png_size_t {
        let fresh4 = rp;
        rp = rp.offset(1);
        let fresh5 = pp;
        pp = pp.offset(1);
        let fresh6 = dp;
        dp = dp.offset(1);
        *fresh6 = (*fresh4 as c_int - *fresh5 as c_int
            & 0xff as c_int) as png_byte;
        v = *fresh6 as c_uint;
        sum = (sum as c_ulong).wrapping_add(
            (if v < 128 as c_uint {
                v
            } else {
                (256 as c_uint).wrapping_sub(v)
            }) as c_ulong,
        ) as png_size_t as png_size_t;
        i = i.wrapping_add(1);
    }
    lp = (*png_ptr).row_buf.offset(1 as c_int as isize);
    cp = (*png_ptr).prev_row.offset(1 as c_int as isize);
    while i < row_bytes {
        let mut a: c_int = 0;
        let mut b: c_int = 0;
        let mut c: c_int = 0;
        let mut pa: c_int = 0;
        let mut pb: c_int = 0;
        let mut pc: c_int = 0;
        let mut p: c_int = 0;
        let fresh7 = pp;
        pp = pp.offset(1);
        b = *fresh7 as c_int;
        let fresh8 = cp;
        cp = cp.offset(1);
        c = *fresh8 as c_int;
        let fresh9 = lp;
        lp = lp.offset(1);
        a = *fresh9 as c_int;
        p = b - c;
        pc = a - c;
        pa = if p < 0 as c_int { -p } else { p };
        pb = if pc < 0 as c_int {
            -pc
        } else {
            pc
        };
        pc = if p + pc < 0 as c_int {
            -(p + pc)
        } else {
            p + pc
        };
        p = if pa <= pb && pa <= pc {
            a
        } else if pb <= pc {
            b
        } else {
            c
        };
        let fresh10 = rp;
        rp = rp.offset(1);
        let fresh11 = dp;
        dp = dp.offset(1);
        *fresh11 = (*fresh10 as c_int - p & 0xff as c_int) as png_byte;
        v = *fresh11 as c_uint;
        sum = (sum as c_ulong).wrapping_add(
            (if v < 128 as c_uint {
                v
            } else {
                (256 as c_uint).wrapping_sub(v)
            }) as c_ulong,
        ) as png_size_t as png_size_t;
        if sum > lmins {
            break;
        }
        i = i.wrapping_add(1);
    }
    return sum;
} }
fn png_setup_paeth_row_only(
    mut png_ptr: png_structrp,
    bpp: png_uint_32,
    row_bytes: png_size_t,
) { unsafe {
    let mut rp: png_bytep = ::core::ptr::null_mut::<png_byte>();
    let mut dp: png_bytep = ::core::ptr::null_mut::<png_byte>();
    let mut pp: png_bytep = ::core::ptr::null_mut::<png_byte>();
    let mut cp: png_bytep = ::core::ptr::null_mut::<png_byte>();
    let mut lp: png_bytep = ::core::ptr::null_mut::<png_byte>();
    let mut i: png_size_t = 0;
    *(*png_ptr).try_row.offset(0 as c_int as isize) =
        PNG_FILTER_VALUE_PAETH as png_byte;
    i = 0 as png_size_t;
    rp = (*png_ptr).row_buf.offset(1 as c_int as isize);
    dp = (*png_ptr).try_row.offset(1 as c_int as isize);
    pp = (*png_ptr).prev_row.offset(1 as c_int as isize);
    while i < bpp as png_size_t {
        let fresh12 = rp;
        rp = rp.offset(1);
        let fresh13 = pp;
        pp = pp.offset(1);
        let fresh14 = dp;
        dp = dp.offset(1);
        *fresh14 = (*fresh12 as c_int - *fresh13 as c_int
            & 0xff as c_int) as png_byte;
        i = i.wrapping_add(1);
    }
    lp = (*png_ptr).row_buf.offset(1 as c_int as isize);
    cp = (*png_ptr).prev_row.offset(1 as c_int as isize);
    while i < row_bytes {
        let mut a: c_int = 0;
        let mut b: c_int = 0;
        let mut c: c_int = 0;
        let mut pa: c_int = 0;
        let mut pb: c_int = 0;
        let mut pc: c_int = 0;
        let mut p: c_int = 0;
        let fresh15 = pp;
        pp = pp.offset(1);
        b = *fresh15 as c_int;
        let fresh16 = cp;
        cp = cp.offset(1);
        c = *fresh16 as c_int;
        let fresh17 = lp;
        lp = lp.offset(1);
        a = *fresh17 as c_int;
        p = b - c;
        pc = a - c;
        pa = if p < 0 as c_int { -p } else { p };
        pb = if pc < 0 as c_int {
            -pc
        } else {
            pc
        };
        pc = if p + pc < 0 as c_int {
            -(p + pc)
        } else {
            p + pc
        };
        p = if pa <= pb && pa <= pc {
            a
        } else if pb <= pc {
            b
        } else {
            c
        };
        let fresh18 = rp;
        rp = rp.offset(1);
        let fresh19 = dp;
        dp = dp.offset(1);
        *fresh19 = (*fresh18 as c_int - p & 0xff as c_int) as png_byte;
        i = i.wrapping_add(1);
    }
} }
#[no_mangle]
pub extern "C" fn png_write_find_filter(
    mut png_ptr: png_structrp,
    mut row_info: png_row_infop,
) { unsafe {
    let mut filter_to_do: c_uint = (*png_ptr).do_filter as c_uint;
    let mut row_buf: png_bytep = ::core::ptr::null_mut::<png_byte>();
    let mut best_row: png_bytep = ::core::ptr::null_mut::<png_byte>();
    let mut bpp: png_uint_32 = 0;
    let mut mins: png_size_t = 0;
    let mut row_bytes: png_size_t = (*row_info).rowbytes;
    bpp = ((*row_info).pixel_depth as c_int + 7 as c_int
        >> 3 as c_int) as png_uint_32;
    row_buf = (*png_ptr).row_buf;
    mins = PNG_SIZE_MAX.wrapping_sub(256 as png_size_t);
    best_row = (*png_ptr).row_buf;
    if PNG_SIZE_MAX.wrapping_div(128 as png_size_t) <= row_bytes {
        filter_to_do &= (0 as c_uint).wrapping_sub(filter_to_do);
    } else if filter_to_do & PNG_FILTER_NONE as c_uint != 0 as c_uint
        && filter_to_do != PNG_FILTER_NONE as c_uint
    {
        let mut rp: png_bytep = ::core::ptr::null_mut::<png_byte>();
        let mut sum: png_size_t = 0 as png_size_t;
        let mut i: png_size_t = 0;
        let mut v: c_uint = 0;
        i = 0 as png_size_t;
        rp = row_buf.offset(1 as c_int as isize);
        while i < row_bytes {
            v = *rp as c_uint;
            sum = (sum as c_ulong).wrapping_add(
                (if v < 128 as c_uint {
                    v
                } else {
                    (256 as c_uint).wrapping_sub(v)
                }) as c_ulong,
            ) as png_size_t as png_size_t;
            i = i.wrapping_add(1);
            rp = rp.offset(1);
        }
        mins = sum;
    }
    if filter_to_do == PNG_FILTER_SUB as c_uint {
        png_setup_sub_row_only(png_ptr, bpp, row_bytes);
        best_row = (*png_ptr).try_row;
    } else if filter_to_do & PNG_FILTER_SUB as c_uint != 0 as c_uint {
        let mut sum_0: png_size_t = 0;
        let mut lmins: png_size_t = mins;
        sum_0 = png_setup_sub_row(png_ptr, bpp, row_bytes, lmins);
        if sum_0 < mins {
            mins = sum_0;
            best_row = (*png_ptr).try_row;
            if !(*png_ptr).tst_row.is_null() {
                (*png_ptr).try_row = (*png_ptr).tst_row;
                (*png_ptr).tst_row = best_row;
            }
        }
    }
    if filter_to_do == PNG_FILTER_UP as c_uint {
        png_setup_up_row_only(png_ptr, row_bytes);
        best_row = (*png_ptr).try_row;
    } else if filter_to_do & PNG_FILTER_UP as c_uint != 0 as c_uint {
        let mut sum_1: png_size_t = 0;
        let mut lmins_0: png_size_t = mins;
        sum_1 = png_setup_up_row(png_ptr, row_bytes, lmins_0);
        if sum_1 < mins {
            mins = sum_1;
            best_row = (*png_ptr).try_row;
            if !(*png_ptr).tst_row.is_null() {
                (*png_ptr).try_row = (*png_ptr).tst_row;
                (*png_ptr).tst_row = best_row;
            }
        }
    }
    if filter_to_do == PNG_FILTER_AVG as c_uint {
        png_setup_avg_row_only(png_ptr, bpp, row_bytes);
        best_row = (*png_ptr).try_row;
    } else if filter_to_do & PNG_FILTER_AVG as c_uint != 0 as c_uint {
        let mut sum_2: png_size_t = 0;
        let mut lmins_1: png_size_t = mins;
        sum_2 = png_setup_avg_row(png_ptr, bpp, row_bytes, lmins_1);
        if sum_2 < mins {
            mins = sum_2;
            best_row = (*png_ptr).try_row;
            if !(*png_ptr).tst_row.is_null() {
                (*png_ptr).try_row = (*png_ptr).tst_row;
                (*png_ptr).tst_row = best_row;
            }
        }
    }
    if filter_to_do == PNG_FILTER_PAETH as c_uint {
        png_setup_paeth_row_only(png_ptr, bpp, row_bytes);
        best_row = (*png_ptr).try_row;
    } else if filter_to_do & PNG_FILTER_PAETH as c_uint != 0 as c_uint {
        let mut sum_3: png_size_t = 0;
        let mut lmins_2: png_size_t = mins;
        sum_3 = png_setup_paeth_row(png_ptr, bpp, row_bytes, lmins_2);
        if sum_3 < mins {
            best_row = (*png_ptr).try_row;
            if !(*png_ptr).tst_row.is_null() {
                (*png_ptr).try_row = (*png_ptr).tst_row;
                (*png_ptr).tst_row = best_row;
            }
        }
    }
    png_write_filtered_row(
        png_ptr,
        best_row,
        (*row_info).rowbytes.wrapping_add(1 as png_size_t),
    );
} }
fn png_write_filtered_row(
    mut png_ptr: png_structrp,
    mut filtered_row: png_bytep,
    mut full_row_length: png_size_t,
) { unsafe {
    png_compress_IDAT(
        png_ptr,
        filtered_row as png_const_bytep,
        full_row_length as png_alloc_size_t,
        Z_NO_FLUSH,
    );
    if !(*png_ptr).prev_row.is_null() {
        let mut tptr: png_bytep = ::core::ptr::null_mut::<png_byte>();
        tptr = (*png_ptr).prev_row;
        (*png_ptr).prev_row = (*png_ptr).row_buf;
        (*png_ptr).row_buf = tptr;
    }
    png_write_finish_row(png_ptr);
    (*png_ptr).flush_rows = (*png_ptr).flush_rows.wrapping_add(1);
    if (*png_ptr).flush_dist > 0 as c_uint
        && (*png_ptr).flush_rows >= (*png_ptr).flush_dist
    {
        png_write_flush(png_ptr);
    }
} }
