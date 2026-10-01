use core::ffi::*;
use crate::src::libpng::pngerror::png_safecat;
use crate::src::libpng::png::png_sig_cmp;
pub use crate::src::ffi::*;
pub use crate::src::c_consts::*;
pub use crate::src::c_structs::*;
pub use crate::src::c_types::*;
pub use crate::src::zlib::deflate::internal_state;
extern "C" {
    fn png_malloc(png_ptr: png_const_structrp, size: png_alloc_size_t) -> png_voidp;
    fn png_calloc(png_ptr: png_const_structrp, size: png_alloc_size_t) -> png_voidp;
    fn png_malloc_warn(png_ptr: png_const_structrp, size: png_alloc_size_t) -> png_voidp;
    fn png_free(png_ptr: png_const_structrp, ptr: png_voidp);
    fn png_error(png_ptr: png_const_structrp, error_message: png_const_charp) -> !;
    fn png_chunk_error(png_ptr: png_const_structrp, error_message: png_const_charp) -> !;
    fn png_chunk_warning(png_ptr: png_const_structrp, warning_message: png_const_charp);
    fn png_chunk_benign_error(png_ptr: png_const_structrp, warning_message: png_const_charp);
    fn png_set_bKGD(
        png_ptr: png_const_structrp,
        info_ptr: png_inforp,
        background: png_const_color_16p,
    );
    fn png_set_hIST(png_ptr: png_const_structrp, info_ptr: png_inforp, hist: png_const_uint_16p);
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
    fn png_set_PLTE(
        png_ptr: png_structrp,
        info_ptr: png_inforp,
        palette: png_const_colorp,
        num_palette: c_int,
    );
    fn png_set_sBIT(png_ptr: png_const_structrp, info_ptr: png_inforp, sig_bit: png_const_color_8p);
    fn png_set_tRNS(
        png_ptr: png_structrp,
        info_ptr: png_inforp,
        trans_alpha: png_const_bytep,
        num_trans: c_int,
        trans_color: png_const_color_16p,
    );
    fn png_set_unknown_chunks(
        png_ptr: png_const_structrp,
        info_ptr: png_inforp,
        unknowns: png_const_unknown_chunkp,
        num_unknowns: c_int,
    );
    fn inflate(strm: z_streamp, flush: c_int) -> c_int;
    fn inflateReset2(strm: z_streamp, windowBits: c_int) -> c_int;
    fn inflateInit2_(
        strm: z_streamp,
        windowBits: c_int,
        version: *const c_char,
        stream_size: c_int,
    ) -> c_int;
    fn png_zstream_error(png_ptr: png_structrp, ret: c_int);
    fn png_malloc_base(png_ptr: png_const_structrp, size: png_alloc_size_t) -> png_voidp;
    fn png_reset_crc(png_ptr: png_structrp);
    fn png_read_data(png_ptr: png_structrp, data: png_bytep, length: png_size_t);
    fn png_calculate_crc(png_ptr: png_structrp, ptr: png_const_bytep, length: png_size_t);
    fn png_init_read_transformations(png_ptr: png_structrp);
}

pub type png_uint_32p = *mut png_uint_32;
pub type png_const_uint_32p = *const png_uint_32;

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
pub type png_structrp = *mut png_struct;
pub type png_const_structrp = *const png_struct;
pub type png_inforp = *mut png_info;

pub type z_streamp = *mut z_stream;
static mut row_mask: [[[png_uint_32; 6]; 3]; 2] = [[[0; 6]; 3]; 2];
static mut display_mask: [[[png_uint_32; 3]; 3]; 2] = [[[0; 3]; 3]; 2];

pub const PNG_COLOR_TYPE_GRAY: c_int = 0;
pub const PNG_COLOR_TYPE_PALETTE: c_int = 3;
pub const PNG_COLOR_TYPE_RGB: c_int = 2;
pub const PNG_COLOR_TYPE_RGB_ALPHA: c_int = 6;
pub const PNG_COLOR_TYPE_GRAY_ALPHA: c_int = 4;

pub const PNG_FILTER_VALUE_NONE: c_int = 0 as c_int;
pub const PNG_FILTER_VALUE_SUB: c_int = 1 as c_int;
pub const PNG_FILTER_VALUE_UP: c_int = 2 as c_int;
pub const PNG_FILTER_VALUE_AVG: c_int = 3 as c_int;
pub const PNG_FILTER_VALUE_PAETH: c_int = 4 as c_int;

pub const PNG_HANDLE_CHUNK_IF_SAFE: c_int = 2 as c_int;

pub const PNG_IO_READING: c_int = 0x1 as c_int;

pub const PNG_IO_CHUNK_HDR: c_int = 0x20 as c_int;
pub const PNG_IO_CHUNK_DATA: c_int = 0x40 as c_int;
pub const PNG_IO_CHUNK_CRC: c_int = 0x80 as c_int;

pub const ZLIB_VERSION: [c_char; 15] =
    unsafe { ::core::mem::transmute::<[u8; 15], [c_char; 15]>(*b"1.2.11-optipng\0") };
pub const Z_OK: c_int = 0 as c_int;
pub const Z_STREAM_END: c_int = 1 as c_int;
pub const Z_DATA_ERROR: c_int = -(3 as c_int);
#[inline]
pub fn png_get_uint_31(
    mut png_ptr: png_const_structrp,
    mut buf: png_const_bytep,
) -> png_uint_32 { unsafe {
    let mut uval: png_uint_32 = ((*buf as png_uint_32) << 24 as c_int)
        .wrapping_add(
            (*buf.offset(1 as c_int as isize) as png_uint_32)
                << 16 as c_int,
        )
        .wrapping_add(
            (*buf.offset(2 as c_int as isize) as png_uint_32)
                << 8 as c_int,
        )
        .wrapping_add(*buf.offset(3 as c_int as isize) as png_uint_32);
    if uval > PNG_UINT_31_MAX {
        png_error(
            png_ptr,
            b"PNG unsigned integer out of range\0" as *const u8 as png_const_charp,
        );
    }
    return uval;
} }
#[inline]
pub fn png_get_uint_32(mut buf: png_const_bytep) -> png_uint_32 { unsafe {
    let mut uval: png_uint_32 = ((*buf as png_uint_32) << 24 as c_int)
        .wrapping_add(
            (*buf.offset(1 as c_int as isize) as png_uint_32)
                << 16 as c_int,
        )
        .wrapping_add(
            (*buf.offset(2 as c_int as isize) as png_uint_32)
                << 8 as c_int,
        )
        .wrapping_add(*buf.offset(3 as c_int as isize) as png_uint_32);
    return uval;
} }
#[inline]
pub fn png_get_int_32(mut buf: png_const_bytep) -> png_int_32 { unsafe {
    let mut uval: png_uint_32 = ((*buf as png_uint_32) << 24 as c_int)
        .wrapping_add(
            (*buf.offset(1 as c_int as isize) as png_uint_32)
                << 16 as c_int,
        )
        .wrapping_add(
            (*buf.offset(2 as c_int as isize) as png_uint_32)
                << 8 as c_int,
        )
        .wrapping_add(*buf.offset(3 as c_int as isize) as png_uint_32);
    if uval as c_uint & 0x80000000 as c_uint == 0 as c_uint {
        return uval as png_int_32;
    }
    uval = (uval as c_uint ^ 0xffffffff as c_uint)
        .wrapping_add(1 as c_uint) as png_uint_32;
    if uval as c_uint & 0x80000000 as c_uint == 0 as c_uint {
        return -(uval as png_int_32);
    }
    return 0 as png_int_32;
} }
#[inline]
pub fn png_get_uint_16(mut buf: png_const_bytep) -> png_uint_16 { unsafe {
    let mut val: c_uint = ((*buf as c_uint) << 8 as c_int)
        .wrapping_add(*buf.offset(1 as c_int as isize) as c_uint);
    return val as png_uint_16;
} }
#[no_mangle]
pub extern "C" fn png_read_sig(mut png_ptr: png_structrp, mut info_ptr: png_inforp) { unsafe {
    let mut num_checked: png_size_t = 0;
    let mut num_to_check: png_size_t = 0;
    if (*png_ptr).sig_bytes as c_int >= 8 as c_int {
        return;
    }
    num_checked = (*png_ptr).sig_bytes as png_size_t;
    num_to_check = (8 as png_size_t).wrapping_sub(num_checked);
    (*png_ptr).io_state = (PNG_IO_READING | PNG_IO_SIGNATURE) as png_uint_32;
    png_read_data(
        png_ptr,
        (&raw mut (*info_ptr).signature as *mut png_byte).offset(num_checked as isize) as png_bytep,
        num_to_check,
    );
    (*png_ptr).sig_bytes = 8 as png_byte;
    if png_sig_cmp(
        &raw mut (*info_ptr).signature as *mut png_byte as png_const_bytep,
        num_checked,
        num_to_check,
    ) != 0 as c_int
    {
        if num_checked < 4 as png_size_t
            && png_sig_cmp(
                &raw mut (*info_ptr).signature as *mut png_byte as png_const_bytep,
                num_checked,
                num_to_check.wrapping_sub(4 as png_size_t),
            ) != 0
        {
            png_error(png_ptr, b"Not a PNG file\0" as *const u8 as png_const_charp);
        } else {
            png_error(
                png_ptr,
                b"PNG file corrupted by ASCII conversion\0" as *const u8 as png_const_charp,
            );
        }
    }
    if num_checked < 3 as png_size_t {
        (*png_ptr).mode |= PNG_HAVE_PNG_SIGNATURE;
    }
} }
#[no_mangle]
pub extern "C" fn png_read_chunk_header(mut png_ptr: png_structrp) -> png_uint_32 { unsafe {
    let mut buf: [png_byte; 8] = [0; 8];
    let mut length: png_uint_32 = 0;
    (*png_ptr).io_state = (PNG_IO_READING | PNG_IO_CHUNK_HDR) as png_uint_32;
    png_read_data(png_ptr, &raw mut buf as png_bytep, 8 as png_size_t);
    length = png_get_uint_31(png_ptr, &raw mut buf as *mut png_byte as png_const_bytep);
    (*png_ptr).chunk_name = ((0xff as c_int
        & *(&raw mut buf as *mut png_byte)
            .offset(4 as c_int as isize)
            .offset(0 as c_int as isize) as c_int)
        as png_uint_32)
        << 24 as c_int
        | ((0xff as c_int
            & *(&raw mut buf as *mut png_byte)
                .offset(4 as c_int as isize)
                .offset(1 as c_int as isize) as c_int)
            as png_uint_32)
            << 16 as c_int
        | ((0xff as c_int
            & *(&raw mut buf as *mut png_byte)
                .offset(4 as c_int as isize)
                .offset(2 as c_int as isize) as c_int)
            as png_uint_32)
            << 8 as c_int
        | ((0xff as c_int
            & *(&raw mut buf as *mut png_byte)
                .offset(4 as c_int as isize)
                .offset(3 as c_int as isize) as c_int)
            as png_uint_32)
            << 0 as c_int;
    png_reset_crc(png_ptr);
    png_calculate_crc(
        png_ptr,
        (&raw mut buf as *mut png_byte).offset(4 as c_int as isize) as png_const_bytep,
        4 as png_size_t,
    );
    png_check_chunk_name(png_ptr, (*png_ptr).chunk_name);
    png_check_chunk_length(png_ptr, length);
    (*png_ptr).io_state = (PNG_IO_READING | PNG_IO_CHUNK_DATA) as png_uint_32;
    return length;
} }
#[inline]
pub fn png_crc_read(
    mut png_ptr: png_structrp,
    mut buf: png_bytep,
    mut length: png_uint_32,
) { unsafe {
    if png_ptr.is_null() {
        return;
    }
    png_read_data(png_ptr, buf, length as png_size_t);
    png_calculate_crc(png_ptr, buf as png_const_bytep, length as png_size_t);
} }
#[no_mangle]
pub extern "C" fn png_crc_finish(
    mut png_ptr: png_structrp,
    mut skip: png_uint_32,
) -> c_int { unsafe {
    while skip > 0 as c_uint {
        let mut len: png_uint_32 = 0;
        let mut tmpbuf: [png_byte; 1024] = [0; 1024];
        len = ::core::mem::size_of::<[png_byte; 1024]>() as png_uint_32;
        if len > skip {
            len = skip;
        }
        skip = (skip as c_uint).wrapping_sub(len as c_uint) as png_uint_32
            as png_uint_32;
        png_crc_read(png_ptr, &raw mut tmpbuf as png_bytep, len);
    }
    if png_crc_error(png_ptr) != 0 as c_int {
        if if 1 as png_uint_32 & (*png_ptr).chunk_name >> 29 as c_int
            != 0 as c_uint
        {
            ((*png_ptr).flags as c_uint & PNG_FLAG_CRC_ANCILLARY_NOWARN
                == 0 as c_uint) as c_int
        } else {
            ((*png_ptr).flags as c_uint & PNG_FLAG_CRC_CRITICAL_USE
                != 0 as c_uint) as c_int
        } != 0
        {
            png_chunk_warning(png_ptr, b"CRC error\0" as *const u8 as png_const_charp);
        } else {
            png_chunk_error(png_ptr, b"CRC error\0" as *const u8 as png_const_charp);
        }
        return 1 as c_int;
    }
    return 0 as c_int;
} }
#[inline]
pub fn png_crc_error(mut png_ptr: png_structrp) -> c_int { unsafe {
    let mut crc_bytes: [png_byte; 4] = [0; 4];
    let mut crc: png_uint_32 = 0;
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
    (*png_ptr).io_state = (PNG_IO_READING | PNG_IO_CHUNK_CRC) as png_uint_32;
    png_read_data(png_ptr, &raw mut crc_bytes as png_bytep, 4 as png_size_t);
    if need_crc != 0 as c_int {
        crc = ((*(&raw mut crc_bytes as *mut png_byte) as png_uint_32) << 24 as c_int)
            .wrapping_add(
                (*(&raw mut crc_bytes as *mut png_byte).offset(1 as c_int as isize)
                    as png_uint_32)
                    << 16 as c_int,
            )
            .wrapping_add(
                (*(&raw mut crc_bytes as *mut png_byte).offset(2 as c_int as isize)
                    as png_uint_32)
                    << 8 as c_int,
            )
            .wrapping_add(
                *(&raw mut crc_bytes as *mut png_byte).offset(3 as c_int as isize)
                    as png_uint_32,
            );
        return (crc != (*png_ptr).crc) as c_int;
    } else {
        return 0 as c_int;
    };
} }
fn png_read_buffer(
    mut png_ptr: png_structrp,
    mut new_size: png_alloc_size_t,
    mut warn: c_int,
) -> png_bytep { unsafe {
    let mut buffer: png_bytep = (*png_ptr).read_buffer;
    if !buffer.is_null() && new_size > (*png_ptr).read_buffer_size {
        (*png_ptr).read_buffer = ::core::ptr::null_mut::<png_byte>();
        (*png_ptr).read_buffer = ::core::ptr::null_mut::<png_byte>();
        (*png_ptr).read_buffer_size = 0 as png_alloc_size_t;
        png_free(png_ptr, buffer as png_voidp);
        buffer = ::core::ptr::null_mut::<png_byte>();
    }
    if buffer.is_null() {
        buffer = png_malloc_base(png_ptr, new_size) as png_bytep;
        if !buffer.is_null() {
            memset(
                buffer as *mut c_void,
                0 as c_int,
                new_size as size_t,
            );
            (*png_ptr).read_buffer = buffer;
            (*png_ptr).read_buffer_size = new_size;
        } else if warn < 2 as c_int {
            if warn != 0 as c_int {
                png_chunk_warning(
                    png_ptr,
                    b"insufficient memory to read chunk\0" as *const u8 as png_const_charp,
                );
            } else {
                png_chunk_error(
                    png_ptr,
                    b"insufficient memory to read chunk\0" as *const u8 as png_const_charp,
                );
            }
        }
    }
    return buffer;
} }
fn png_inflate_claim(
    mut png_ptr: png_structrp,
    mut owner: png_uint_32,
) -> c_int { unsafe {
    if (*png_ptr).zowner != 0 as c_uint {
        let mut msg: [c_char; 64] = [0; 64];
        *(&raw mut msg as *mut c_char).offset(0 as c_int as isize) =
            ((*png_ptr).zowner as c_uint >> 24 as c_int
                & 0xff as c_uint) as c_char;
        *(&raw mut msg as *mut c_char).offset(1 as c_int as isize) =
            ((*png_ptr).zowner as c_uint >> 16 as c_int
                & 0xff as c_uint) as c_char;
        *(&raw mut msg as *mut c_char).offset(2 as c_int as isize) =
            ((*png_ptr).zowner as c_uint >> 8 as c_int
                & 0xff as c_uint) as c_char;
        *(&raw mut msg as *mut c_char).offset(3 as c_int as isize) =
            ((*png_ptr).zowner as c_uint & 0xff as c_uint)
                as c_char;
        png_safecat(
            &raw mut msg as png_charp,
            ::core::mem::size_of::<[c_char; 64]>() as size_t,
            4 as size_t,
            b" using zstream\0" as *const u8 as png_const_charp,
        );
        png_chunk_warning(
            png_ptr,
            &raw mut msg as *mut c_char as png_const_charp,
        );
        (*png_ptr).zowner = 0 as png_uint_32;
    }
    let mut ret: c_int = 0;
    let mut window_bits: c_int = 0 as c_int;
    (*png_ptr).zstream.next_in = ::core::ptr::null::<Bytef>();
    (*png_ptr).zstream.avail_in = 0 as uInt;
    (*png_ptr).zstream.next_out = ::core::ptr::null_mut::<Bytef>();
    (*png_ptr).zstream.avail_out = 0 as uInt;
    if (*png_ptr).flags as c_uint & PNG_FLAG_ZSTREAM_INITIALIZED
        != 0 as c_uint
    {
        ret = inflateReset2(&raw mut (*png_ptr).zstream, window_bits);
    } else {
        ret = inflateInit2_(
            &raw mut (*png_ptr).zstream,
            window_bits,
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
#[inline]
pub fn png_zlib_inflate(
    mut png_ptr: png_structrp,
    mut flush: c_int,
) -> c_int { unsafe {
    if (*png_ptr).zstream_start as c_int != 0
        && (*png_ptr).zstream.avail_in > 0 as c_uint
    {
        if *(*png_ptr).zstream.next_in as c_int >> 4 as c_int
            > 7 as c_int
        {
            (*png_ptr).zstream.msg =
                b"invalid window size (libpng)\0" as *const u8 as *const c_char;
            return Z_DATA_ERROR;
        }
        (*png_ptr).zstream_start = 0 as png_byte;
    }
    return inflate(&raw mut (*png_ptr).zstream, flush);
} }
#[no_mangle]
pub extern "C" fn png_handle_IHDR(
    mut png_ptr: png_structrp,
    mut info_ptr: png_inforp,
    mut length: png_uint_32,
) { unsafe {
    let mut buf: [png_byte; 13] = [0; 13];
    let mut width: png_uint_32 = 0;
    let mut height: png_uint_32 = 0;
    let mut bit_depth: c_int = 0;
    let mut color_type: c_int = 0;
    let mut compression_type: c_int = 0;
    let mut filter_type: c_int = 0;
    let mut interlace_type: c_int = 0;
    if (*png_ptr).mode as c_uint & PNG_HAVE_IHDR as c_uint
        != 0 as c_uint
    {
        png_chunk_error(png_ptr, b"out of place\0" as *const u8 as png_const_charp);
    }
    if length != 13 as c_uint {
        png_chunk_error(png_ptr, b"invalid\0" as *const u8 as png_const_charp);
    }
    (*png_ptr).mode |= PNG_HAVE_IHDR as c_uint;
    png_crc_read(png_ptr, &raw mut buf as png_bytep, 13 as png_uint_32);
    png_crc_finish(png_ptr, 0 as png_uint_32);
    width = png_get_uint_31(png_ptr, &raw mut buf as *mut png_byte as png_const_bytep);
    height = png_get_uint_31(
        png_ptr,
        (&raw mut buf as *mut png_byte).offset(4 as c_int as isize) as png_const_bytep,
    );
    bit_depth = buf[8 as c_int as usize] as c_int;
    color_type = buf[9 as c_int as usize] as c_int;
    compression_type = buf[10 as c_int as usize] as c_int;
    filter_type = buf[11 as c_int as usize] as c_int;
    interlace_type = buf[12 as c_int as usize] as c_int;
    (*png_ptr).width = width;
    (*png_ptr).height = height;
    (*png_ptr).bit_depth = bit_depth as png_byte;
    (*png_ptr).interlaced = interlace_type as png_byte;
    (*png_ptr).color_type = color_type as png_byte;
    (*png_ptr).compression_type = compression_type as png_byte;
    match (*png_ptr).color_type as c_int {
        PNG_COLOR_TYPE_RGB => {
            (*png_ptr).channels = 3 as png_byte;
        }
        PNG_COLOR_TYPE_GRAY_ALPHA => {
            (*png_ptr).channels = 2 as png_byte;
        }
        PNG_COLOR_TYPE_RGB_ALPHA => {
            (*png_ptr).channels = 4 as png_byte;
        }
        PNG_COLOR_TYPE_GRAY | PNG_COLOR_TYPE_PALETTE | _ => {
            (*png_ptr).channels = 1 as png_byte;
        }
    }
    (*png_ptr).pixel_depth = ((*png_ptr).bit_depth as c_int
        * (*png_ptr).channels as c_int) as png_byte;
    (*png_ptr).rowbytes = if (*png_ptr).pixel_depth as c_int >= 8 as c_int
    {
        ((*png_ptr).width as png_size_t)
            .wrapping_mul((*png_ptr).pixel_depth as png_size_t >> 3 as c_int)
    } else {
        ((*png_ptr).width as png_size_t)
            .wrapping_mul((*png_ptr).pixel_depth as png_size_t)
            .wrapping_add(7 as png_size_t)
            >> 3 as c_int
    };
    png_set_IHDR(
        png_ptr,
        info_ptr,
        width,
        height,
        bit_depth,
        color_type,
        interlace_type,
        compression_type,
        filter_type,
    );
} }
#[no_mangle]
pub extern "C" fn png_handle_PLTE(
    mut png_ptr: png_structrp,
    mut info_ptr: png_inforp,
    mut length: png_uint_32,
) { unsafe {
    let mut palette: [png_color; 256] = [png_color {
        red: 0,
        green: 0,
        blue: 0,
    }; 256];
    let mut max_palette_length: c_int = 0;
    let mut num: c_int = 0;
    let mut i: c_int = 0;
    let mut pal_ptr: png_colorp = ::core::ptr::null_mut::<png_color>();
    if (*png_ptr).mode as c_uint & PNG_HAVE_IHDR as c_uint
        == 0 as c_uint
    {
        png_chunk_error(png_ptr, b"missing IHDR\0" as *const u8 as png_const_charp);
    } else if (*png_ptr).mode as c_uint & PNG_HAVE_PLTE as c_uint
        != 0 as c_uint
    {
        png_chunk_error(png_ptr, b"duplicate\0" as *const u8 as png_const_charp);
    } else if (*png_ptr).mode as c_uint & PNG_HAVE_IDAT != 0 as c_uint {
        png_crc_finish(png_ptr, length);
        png_chunk_benign_error(png_ptr, b"out of place\0" as *const u8 as png_const_charp);
        return;
    }
    (*png_ptr).mode |= PNG_HAVE_PLTE as c_uint;
    if (*png_ptr).color_type as c_int & PNG_COLOR_MASK_COLOR == 0 as c_int
    {
        png_crc_finish(png_ptr, length);
        png_chunk_benign_error(
            png_ptr,
            b"ignored in grayscale PNG\0" as *const u8 as png_const_charp,
        );
        return;
    }
    if length > (3 as c_int * PNG_MAX_PALETTE_LENGTH) as c_uint
        || (length as c_uint).wrapping_rem(3 as c_uint) != 0
    {
        png_crc_finish(png_ptr, length);
        if (*png_ptr).color_type as c_int != PNG_COLOR_TYPE_PALETTE {
            png_chunk_benign_error(png_ptr, b"invalid\0" as *const u8 as png_const_charp);
        } else {
            png_chunk_error(png_ptr, b"invalid\0" as *const u8 as png_const_charp);
        }
        return;
    }
    num = length as c_int / 3 as c_int;
    if (*png_ptr).color_type as c_int == PNG_COLOR_TYPE_PALETTE {
        max_palette_length =
            (1 as c_int) << (*png_ptr).bit_depth as c_int;
    } else {
        max_palette_length = PNG_MAX_PALETTE_LENGTH;
    }
    if num > max_palette_length {
        num = max_palette_length;
    }
    i = 0 as c_int;
    pal_ptr = &raw mut palette as *mut png_color as png_colorp;
    while i < num {
        let mut buf: [png_byte; 3] = [0; 3];
        png_crc_read(png_ptr, &raw mut buf as png_bytep, 3 as png_uint_32);
        (*pal_ptr).red = buf[0 as c_int as usize];
        (*pal_ptr).green = buf[1 as c_int as usize];
        (*pal_ptr).blue = buf[2 as c_int as usize];
        i += 1;
        pal_ptr = pal_ptr.offset(1);
    }
    png_crc_finish(
        png_ptr,
        (length as c_uint)
            .wrapping_sub((num as c_uint).wrapping_mul(3 as c_uint)),
    );
    png_set_PLTE(
        png_ptr,
        info_ptr,
        &raw mut palette as *mut png_color as png_const_colorp,
        num,
    );
    if (*png_ptr).num_trans as c_int > 0 as c_int
        || !info_ptr.is_null()
            && (*info_ptr).valid as c_uint & PNG_INFO_tRNS != 0 as c_uint
    {
        (*png_ptr).num_trans = 0 as png_uint_16;
        if !info_ptr.is_null() {
            (*info_ptr).num_trans = 0 as png_uint_16;
        }
        png_chunk_benign_error(
            png_ptr,
            b"tRNS must be after\0" as *const u8 as png_const_charp,
        );
    }
    if !info_ptr.is_null()
        && (*info_ptr).valid as c_uint & PNG_INFO_hIST != 0 as c_uint
    {
        png_chunk_benign_error(
            png_ptr,
            b"hIST must be after\0" as *const u8 as png_const_charp,
        );
    }
    if !info_ptr.is_null()
        && (*info_ptr).valid as c_uint & PNG_INFO_bKGD != 0 as c_uint
    {
        png_chunk_benign_error(
            png_ptr,
            b"bKGD must be after\0" as *const u8 as png_const_charp,
        );
    }
} }
#[no_mangle]
pub extern "C" fn png_handle_IEND(
    mut png_ptr: png_structrp,
    mut info_ptr: png_inforp,
    mut length: png_uint_32,
) { unsafe {
    if (*png_ptr).mode as c_uint & PNG_HAVE_IHDR as c_uint
        == 0 as c_uint
        || (*png_ptr).mode as c_uint & PNG_HAVE_IDAT == 0 as c_uint
    {
        png_chunk_error(png_ptr, b"out of place\0" as *const u8 as png_const_charp);
    }
    (*png_ptr).mode |= PNG_AFTER_IDAT as c_uint | PNG_HAVE_IEND;
    png_crc_finish(png_ptr, length);
    if length != 0 as c_uint {
        png_chunk_benign_error(png_ptr, b"invalid\0" as *const u8 as png_const_charp);
    }
} }
#[no_mangle]
pub extern "C" fn png_handle_sBIT(
    mut png_ptr: png_structrp,
    mut info_ptr: png_inforp,
    mut length: png_uint_32,
) { unsafe {
    let mut truelen: c_uint = 0;
    let mut i: c_uint = 0;
    let mut sample_depth: png_byte = 0;
    let mut buf: [png_byte; 4] = [0; 4];
    if (*png_ptr).mode as c_uint & PNG_HAVE_IHDR as c_uint
        == 0 as c_uint
    {
        png_chunk_error(png_ptr, b"missing IHDR\0" as *const u8 as png_const_charp);
    } else if (*png_ptr).mode as c_uint
        & (PNG_HAVE_IDAT | PNG_HAVE_PLTE as c_uint)
        != 0 as c_uint
    {
        png_crc_finish(png_ptr, length);
        png_chunk_benign_error(png_ptr, b"out of place\0" as *const u8 as png_const_charp);
        return;
    }
    if !info_ptr.is_null()
        && (*info_ptr).valid as c_uint & PNG_INFO_sBIT != 0 as c_uint
    {
        png_crc_finish(png_ptr, length);
        png_chunk_benign_error(png_ptr, b"duplicate\0" as *const u8 as png_const_charp);
        return;
    }
    if (*png_ptr).color_type as c_int == PNG_COLOR_TYPE_PALETTE {
        truelen = 3 as c_uint;
        sample_depth = 8 as png_byte;
    } else {
        truelen = (*png_ptr).channels as c_uint;
        sample_depth = (*png_ptr).bit_depth;
    }
    if length != truelen || length > 4 as c_uint {
        png_chunk_benign_error(png_ptr, b"invalid\0" as *const u8 as png_const_charp);
        png_crc_finish(png_ptr, length);
        return;
    }
    buf[3 as c_int as usize] = sample_depth;
    buf[2 as c_int as usize] = buf[3 as c_int as usize];
    buf[1 as c_int as usize] = buf[2 as c_int as usize];
    buf[0 as c_int as usize] = buf[1 as c_int as usize];
    png_crc_read(png_ptr, &raw mut buf as png_bytep, truelen as png_uint_32);
    if png_crc_finish(png_ptr, 0 as png_uint_32) != 0 as c_int {
        return;
    }
    i = 0 as c_uint;
    while i < truelen {
        if buf[i as usize] as c_int == 0 as c_int
            || buf[i as usize] as c_int > sample_depth as c_int
        {
            png_chunk_benign_error(png_ptr, b"invalid\0" as *const u8 as png_const_charp);
            return;
        }
        i = i.wrapping_add(1);
    }
    if (*png_ptr).color_type as c_int & PNG_COLOR_MASK_COLOR != 0 as c_int
    {
        (*png_ptr).sig_bit.red = buf[0 as c_int as usize];
        (*png_ptr).sig_bit.green = buf[1 as c_int as usize];
        (*png_ptr).sig_bit.blue = buf[2 as c_int as usize];
        (*png_ptr).sig_bit.alpha = buf[3 as c_int as usize];
    } else {
        (*png_ptr).sig_bit.gray = buf[0 as c_int as usize];
        (*png_ptr).sig_bit.red = buf[0 as c_int as usize];
        (*png_ptr).sig_bit.green = buf[0 as c_int as usize];
        (*png_ptr).sig_bit.blue = buf[0 as c_int as usize];
        (*png_ptr).sig_bit.alpha = buf[1 as c_int as usize];
    }
    png_set_sBIT(
        png_ptr,
        info_ptr,
        &raw mut (*png_ptr).sig_bit as png_const_color_8p,
    );
} }
#[no_mangle]
pub extern "C" fn png_handle_tRNS(
    mut png_ptr: png_structrp,
    mut info_ptr: png_inforp,
    mut length: png_uint_32,
) { unsafe {
    let mut readbuf: [png_byte; 256] = [0; 256];
    if (*png_ptr).mode as c_uint & PNG_HAVE_IHDR as c_uint
        == 0 as c_uint
    {
        png_chunk_error(png_ptr, b"missing IHDR\0" as *const u8 as png_const_charp);
    } else if (*png_ptr).mode as c_uint & PNG_HAVE_IDAT != 0 as c_uint {
        png_crc_finish(png_ptr, length);
        png_chunk_benign_error(png_ptr, b"out of place\0" as *const u8 as png_const_charp);
        return;
    } else if !info_ptr.is_null()
        && (*info_ptr).valid as c_uint & PNG_INFO_tRNS != 0 as c_uint
    {
        png_crc_finish(png_ptr, length);
        png_chunk_benign_error(png_ptr, b"duplicate\0" as *const u8 as png_const_charp);
        return;
    }
    if (*png_ptr).color_type as c_int == PNG_COLOR_TYPE_GRAY {
        let mut buf: [png_byte; 2] = [0; 2];
        if length != 2 as c_uint {
            png_crc_finish(png_ptr, length);
            png_chunk_benign_error(png_ptr, b"invalid\0" as *const u8 as png_const_charp);
            return;
        }
        png_crc_read(png_ptr, &raw mut buf as png_bytep, 2 as png_uint_32);
        (*png_ptr).num_trans = 1 as png_uint_16;
        (*png_ptr).trans_color.gray = ((*(&raw mut buf as *mut png_byte) as c_uint)
            << 8 as c_int)
            .wrapping_add(
                *(&raw mut buf as *mut png_byte).offset(1 as c_int as isize)
                    as c_uint,
            ) as png_uint_16;
    } else if (*png_ptr).color_type as c_int == PNG_COLOR_TYPE_RGB {
        let mut buf_0: [png_byte; 6] = [0; 6];
        if length != 6 as c_uint {
            png_crc_finish(png_ptr, length);
            png_chunk_benign_error(png_ptr, b"invalid\0" as *const u8 as png_const_charp);
            return;
        }
        png_crc_read(png_ptr, &raw mut buf_0 as png_bytep, length);
        (*png_ptr).num_trans = 1 as png_uint_16;
        (*png_ptr).trans_color.red = ((*(&raw mut buf_0 as *mut png_byte) as c_uint)
            << 8 as c_int)
            .wrapping_add(
                *(&raw mut buf_0 as *mut png_byte).offset(1 as c_int as isize)
                    as c_uint,
            ) as png_uint_16;
        (*png_ptr).trans_color.green = ((*(&raw mut buf_0 as *mut png_byte)
            .offset(2 as c_int as isize)
            as c_uint)
            << 8 as c_int)
            .wrapping_add(
                *(&raw mut buf_0 as *mut png_byte)
                    .offset(2 as c_int as isize)
                    .offset(1 as c_int as isize)
                    as c_uint,
            ) as png_uint_16;
        (*png_ptr).trans_color.blue = ((*(&raw mut buf_0 as *mut png_byte)
            .offset(4 as c_int as isize)
            as c_uint)
            << 8 as c_int)
            .wrapping_add(
                *(&raw mut buf_0 as *mut png_byte)
                    .offset(4 as c_int as isize)
                    .offset(1 as c_int as isize)
                    as c_uint,
            ) as png_uint_16;
    } else if (*png_ptr).color_type as c_int == PNG_COLOR_TYPE_PALETTE {
        if (*png_ptr).mode as c_uint & PNG_HAVE_PLTE as c_uint
            == 0 as c_uint
        {
            png_crc_finish(png_ptr, length);
            png_chunk_benign_error(png_ptr, b"out of place\0" as *const u8 as png_const_charp);
            return;
        }
        if length > (*png_ptr).num_palette as c_uint
            || length > PNG_MAX_PALETTE_LENGTH as c_uint
            || length == 0 as c_uint
        {
            png_crc_finish(png_ptr, length);
            png_chunk_benign_error(png_ptr, b"invalid\0" as *const u8 as png_const_charp);
            return;
        }
        png_crc_read(png_ptr, &raw mut readbuf as png_bytep, length);
        (*png_ptr).num_trans = length as png_uint_16;
    } else {
        png_crc_finish(png_ptr, length);
        png_chunk_benign_error(
            png_ptr,
            b"invalid with alpha channel\0" as *const u8 as png_const_charp,
        );
        return;
    }
    if png_crc_finish(png_ptr, 0 as png_uint_32) != 0 as c_int {
        (*png_ptr).num_trans = 0 as png_uint_16;
        return;
    }
    png_set_tRNS(
        png_ptr,
        info_ptr,
        &raw mut readbuf as *mut png_byte as png_const_bytep,
        (*png_ptr).num_trans as c_int,
        &raw mut (*png_ptr).trans_color as png_const_color_16p,
    );
} }
#[no_mangle]
pub extern "C" fn png_handle_bKGD(
    mut png_ptr: png_structrp,
    mut info_ptr: png_inforp,
    mut length: png_uint_32,
) { unsafe {
    let mut truelen: c_uint = 0;
    let mut buf: [png_byte; 6] = [0; 6];
    let mut background: png_color_16 = png_color_16 {
        index: 0,
        red: 0,
        green: 0,
        blue: 0,
        gray: 0,
    };
    if (*png_ptr).mode as c_uint & PNG_HAVE_IHDR as c_uint
        == 0 as c_uint
    {
        png_chunk_error(png_ptr, b"missing IHDR\0" as *const u8 as png_const_charp);
    } else if (*png_ptr).mode as c_uint & PNG_HAVE_IDAT != 0 as c_uint
        || (*png_ptr).color_type as c_int == PNG_COLOR_TYPE_PALETTE
            && (*png_ptr).mode as c_uint & PNG_HAVE_PLTE as c_uint
                == 0 as c_uint
    {
        png_crc_finish(png_ptr, length);
        png_chunk_benign_error(png_ptr, b"out of place\0" as *const u8 as png_const_charp);
        return;
    } else if !info_ptr.is_null()
        && (*info_ptr).valid as c_uint & PNG_INFO_bKGD != 0 as c_uint
    {
        png_crc_finish(png_ptr, length);
        png_chunk_benign_error(png_ptr, b"duplicate\0" as *const u8 as png_const_charp);
        return;
    }
    if (*png_ptr).color_type as c_int == PNG_COLOR_TYPE_PALETTE {
        truelen = 1 as c_uint;
    } else if (*png_ptr).color_type as c_int & PNG_COLOR_MASK_COLOR
        != 0 as c_int
    {
        truelen = 6 as c_uint;
    } else {
        truelen = 2 as c_uint;
    }
    if length != truelen {
        png_crc_finish(png_ptr, length);
        png_chunk_benign_error(png_ptr, b"invalid\0" as *const u8 as png_const_charp);
        return;
    }
    png_crc_read(png_ptr, &raw mut buf as png_bytep, truelen as png_uint_32);
    if png_crc_finish(png_ptr, 0 as png_uint_32) != 0 as c_int {
        return;
    }
    if (*png_ptr).color_type as c_int == PNG_COLOR_TYPE_PALETTE {
        background.index = buf[0 as c_int as usize];
        if !info_ptr.is_null()
            && (*info_ptr).num_palette as c_int != 0 as c_int
        {
            if buf[0 as c_int as usize] as c_int
                >= (*info_ptr).num_palette as c_int
            {
                png_chunk_benign_error(png_ptr, b"invalid index\0" as *const u8 as png_const_charp);
                return;
            }
            background.red = (*(*png_ptr)
                .palette
                .offset(buf[0 as c_int as usize] as isize))
            .red as png_uint_16;
            background.green = (*(*png_ptr)
                .palette
                .offset(buf[0 as c_int as usize] as isize))
            .green as png_uint_16;
            background.blue = (*(*png_ptr)
                .palette
                .offset(buf[0 as c_int as usize] as isize))
            .blue as png_uint_16;
        } else {
            background.blue = 0 as png_uint_16;
            background.green = background.blue;
            background.red = background.green;
        }
        background.gray = 0 as png_uint_16;
    } else if (*png_ptr).color_type as c_int & PNG_COLOR_MASK_COLOR
        == 0 as c_int
    {
        background.index = 0 as png_byte;
        background.gray = ((*(&raw mut buf as *mut png_byte) as c_uint)
            << 8 as c_int)
            .wrapping_add(
                *(&raw mut buf as *mut png_byte).offset(1 as c_int as isize)
                    as c_uint,
            ) as png_uint_16;
        background.blue = background.gray;
        background.green = background.blue;
        background.red = background.green;
    } else {
        background.index = 0 as png_byte;
        background.red = ((*(&raw mut buf as *mut png_byte) as c_uint)
            << 8 as c_int)
            .wrapping_add(
                *(&raw mut buf as *mut png_byte).offset(1 as c_int as isize)
                    as c_uint,
            ) as png_uint_16;
        background.green = ((*(&raw mut buf as *mut png_byte)
            .offset(2 as c_int as isize)
            as c_uint)
            << 8 as c_int)
            .wrapping_add(
                *(&raw mut buf as *mut png_byte)
                    .offset(2 as c_int as isize)
                    .offset(1 as c_int as isize)
                    as c_uint,
            ) as png_uint_16;
        background.blue = ((*(&raw mut buf as *mut png_byte)
            .offset(4 as c_int as isize)
            as c_uint)
            << 8 as c_int)
            .wrapping_add(
                *(&raw mut buf as *mut png_byte)
                    .offset(4 as c_int as isize)
                    .offset(1 as c_int as isize)
                    as c_uint,
            ) as png_uint_16;
        background.gray = 0 as png_uint_16;
    }
    png_set_bKGD(
        png_ptr,
        info_ptr,
        &raw mut background as png_const_color_16p,
    );
} }
#[no_mangle]
pub extern "C" fn png_handle_hIST(
    mut png_ptr: png_structrp,
    mut info_ptr: png_inforp,
    mut length: png_uint_32,
) { unsafe {
    let mut num: c_uint = 0;
    let mut i: c_uint = 0;
    let mut readbuf: [png_uint_16; 256] = [0; 256];
    if (*png_ptr).mode as c_uint & PNG_HAVE_IHDR as c_uint
        == 0 as c_uint
    {
        png_chunk_error(png_ptr, b"missing IHDR\0" as *const u8 as png_const_charp);
    } else if (*png_ptr).mode as c_uint & PNG_HAVE_IDAT != 0 as c_uint
        || (*png_ptr).mode as c_uint & PNG_HAVE_PLTE as c_uint
            == 0 as c_uint
    {
        png_crc_finish(png_ptr, length);
        png_chunk_benign_error(png_ptr, b"out of place\0" as *const u8 as png_const_charp);
        return;
    } else if !info_ptr.is_null()
        && (*info_ptr).valid as c_uint & PNG_INFO_hIST != 0 as c_uint
    {
        png_crc_finish(png_ptr, length);
        png_chunk_benign_error(png_ptr, b"duplicate\0" as *const u8 as png_const_charp);
        return;
    }
    num = (length as c_uint).wrapping_div(2 as c_uint);
    if num != (*png_ptr).num_palette as c_uint
        || num > PNG_MAX_PALETTE_LENGTH as c_uint
    {
        png_crc_finish(png_ptr, length);
        png_chunk_benign_error(png_ptr, b"invalid\0" as *const u8 as png_const_charp);
        return;
    }
    i = 0 as c_uint;
    while i < num {
        let mut buf: [png_byte; 2] = [0; 2];
        png_crc_read(png_ptr, &raw mut buf as png_bytep, 2 as png_uint_32);
        readbuf[i as usize] = ((*(&raw mut buf as *mut png_byte) as c_uint)
            << 8 as c_int)
            .wrapping_add(
                *(&raw mut buf as *mut png_byte).offset(1 as c_int as isize)
                    as c_uint,
            ) as png_uint_16;
        i = i.wrapping_add(1);
    }
    if png_crc_finish(png_ptr, 0 as png_uint_32) != 0 as c_int {
        return;
    }
    png_set_hIST(
        png_ptr,
        info_ptr,
        &raw mut readbuf as *mut png_uint_16 as png_const_uint_16p,
    );
} }
fn png_cache_unknown_chunk(
    mut png_ptr: png_structrp,
    mut length: png_uint_32,
) -> c_int { unsafe {
    let mut limit: png_alloc_size_t = PNG_SIZE_MAX;
    if !(*png_ptr).unknown_chunk.data.is_null() {
        png_free(png_ptr, (*png_ptr).unknown_chunk.data as png_voidp);
        (*png_ptr).unknown_chunk.data = ::core::ptr::null_mut::<png_byte>();
    }
    if (*png_ptr).user_chunk_malloc_max > 0 as png_alloc_size_t
        && (*png_ptr).user_chunk_malloc_max < limit
    {
        limit = (*png_ptr).user_chunk_malloc_max;
    }
    if length as png_alloc_size_t <= limit {
        *(&raw mut (*png_ptr).unknown_chunk.name as *mut png_byte as *mut c_char)
            .offset(0 as c_int as isize) =
            ((*png_ptr).chunk_name as c_uint >> 24 as c_int
                & 0xff as c_uint) as c_char;
        *(&raw mut (*png_ptr).unknown_chunk.name as *mut png_byte as *mut c_char)
            .offset(1 as c_int as isize) =
            ((*png_ptr).chunk_name as c_uint >> 16 as c_int
                & 0xff as c_uint) as c_char;
        *(&raw mut (*png_ptr).unknown_chunk.name as *mut png_byte as *mut c_char)
            .offset(2 as c_int as isize) =
            ((*png_ptr).chunk_name as c_uint >> 8 as c_int
                & 0xff as c_uint) as c_char;
        *(&raw mut (*png_ptr).unknown_chunk.name as *mut png_byte as *mut c_char)
            .offset(3 as c_int as isize) =
            ((*png_ptr).chunk_name as c_uint & 0xff as c_uint)
                as c_char;
        *(&raw mut (*png_ptr).unknown_chunk.name as *mut png_byte as *mut c_char)
            .offset(4 as c_int as isize) = 0 as c_char;
        (*png_ptr).unknown_chunk.size = length as png_size_t;
        (*png_ptr).unknown_chunk.location = (*png_ptr).mode as png_byte;
        if length == 0 as c_uint {
            (*png_ptr).unknown_chunk.data = ::core::ptr::null_mut::<png_byte>();
        } else {
            (*png_ptr).unknown_chunk.data =
                png_malloc_warn(png_ptr, length as png_alloc_size_t) as *mut png_byte;
        }
    }
    if (*png_ptr).unknown_chunk.data.is_null() && length > 0 as c_uint {
        png_crc_finish(png_ptr, length);
        png_chunk_benign_error(
            png_ptr,
            b"unknown chunk exceeds memory limits\0" as *const u8 as png_const_charp,
        );
        return 0 as c_int;
    } else {
        if length > 0 as c_uint {
            png_crc_read(png_ptr, (*png_ptr).unknown_chunk.data as png_bytep, length);
        }
        png_crc_finish(png_ptr, 0 as png_uint_32);
        return 1 as c_int;
    };
} }
#[no_mangle]
pub extern "C" fn png_handle_unknown(
    mut png_ptr: png_structrp,
    mut info_ptr: png_inforp,
    mut length: png_uint_32,
    mut keep: c_int,
) { unsafe {
    let mut handled: c_int = 0 as c_int;
    if keep == PNG_HANDLE_CHUNK_AS_DEFAULT {
        keep = (*png_ptr).unknown_default;
    }
    if keep == PNG_HANDLE_CHUNK_ALWAYS
        || keep == PNG_HANDLE_CHUNK_IF_SAFE
            && 1 as png_uint_32 & (*png_ptr).chunk_name >> 29 as c_int != 0
    {
        if png_cache_unknown_chunk(png_ptr, length) == 0 as c_int {
            keep = PNG_HANDLE_CHUNK_NEVER;
        }
    } else {
        png_crc_finish(png_ptr, length);
    }
    if keep == PNG_HANDLE_CHUNK_ALWAYS
        || keep == PNG_HANDLE_CHUNK_IF_SAFE
            && 1 as png_uint_32 & (*png_ptr).chunk_name >> 29 as c_int != 0
    {
        let mut current_block_13: u64;
        match (*png_ptr).user_chunk_cache_max {
            2 => {
                (*png_ptr).user_chunk_cache_max = 1 as png_uint_32;
                png_chunk_benign_error(
                    png_ptr,
                    b"no space in chunk cache\0" as *const u8 as png_const_charp,
                );
                current_block_13 = 3512920355445576850;
            }
            1 => {
                current_block_13 = 3512920355445576850;
            }
            0 => {
                current_block_13 = 7763499777922743165;
            }
            _ => {
                (*png_ptr).user_chunk_cache_max = (*png_ptr).user_chunk_cache_max.wrapping_sub(1);
                current_block_13 = 7763499777922743165;
            }
        }
        match current_block_13 {
            7763499777922743165 => {
                png_set_unknown_chunks(
                    png_ptr,
                    info_ptr,
                    &raw mut (*png_ptr).unknown_chunk as png_const_unknown_chunkp,
                    1 as c_int,
                );
                handled = 1 as c_int;
            }
            _ => {}
        }
    }
    if !(*png_ptr).unknown_chunk.data.is_null() {
        png_free(png_ptr, (*png_ptr).unknown_chunk.data as png_voidp);
    }
    (*png_ptr).unknown_chunk.data = ::core::ptr::null_mut::<png_byte>();
    if handled == 0 as c_int
        && 1 as png_uint_32 & (*png_ptr).chunk_name >> 29 as c_int == 0
    {
        png_chunk_error(
            png_ptr,
            b"unhandled critical chunk\0" as *const u8 as png_const_charp,
        );
    }
} }
#[inline]
pub fn png_check_chunk_name(
    mut png_ptr: png_const_structrp,
    chunk_name: png_uint_32,
) { unsafe {
    let mut i: c_int = 0;
    let mut cn: png_uint_32 = chunk_name;
    i = 1 as c_int;
    while i <= 4 as c_int {
        let mut c: c_int =
            (cn as c_uint & 0xff as c_uint) as c_int;
        if c < 65 as c_int
            || c > 122 as c_int
            || c > 90 as c_int && c < 97 as c_int
        {
            png_chunk_error(
                png_ptr,
                b"invalid chunk type\0" as *const u8 as png_const_charp,
            );
        }
        cn >>= 8 as c_int;
        i += 1;
    }
} }
#[inline]
pub fn png_check_chunk_length(
    mut png_ptr: png_const_structrp,
    length: png_uint_32,
) { unsafe {
    let mut limit: png_alloc_size_t = PNG_UINT_31_MAX as png_alloc_size_t;
    if (*png_ptr).user_chunk_malloc_max > 0 as png_alloc_size_t
        && (*png_ptr).user_chunk_malloc_max < limit
    {
        limit = (*png_ptr).user_chunk_malloc_max;
    }
    if (*png_ptr).chunk_name == png_IDAT {
        let mut idat_limit: png_alloc_size_t = PNG_UINT_31_MAX as png_alloc_size_t;
        let mut row_factor: size_t = ((*png_ptr).width as c_uint)
            .wrapping_mul((*png_ptr).channels as c_uint)
            .wrapping_mul(
                (if (*png_ptr).bit_depth as c_int > 8 as c_int {
                    2 as c_int
                } else {
                    1 as c_int
                }) as c_uint,
            )
            .wrapping_add(1 as c_uint)
            .wrapping_add(
                (if (*png_ptr).interlaced as c_int != 0 {
                    6 as c_int
                } else {
                    0 as c_int
                }) as c_uint,
            ) as size_t;
        if (*png_ptr).height as size_t > (PNG_UINT_32_MAX as size_t).wrapping_div(row_factor) {
            idat_limit = PNG_UINT_31_MAX as png_alloc_size_t;
        } else {
            idat_limit = ((*png_ptr).height as size_t).wrapping_mul(row_factor) as png_alloc_size_t;
        }
        row_factor = if row_factor > 32566 as size_t {
            32566 as size_t
        } else {
            row_factor
        };
        idat_limit = (idat_limit as c_ulong).wrapping_add(
            (6 as png_alloc_size_t).wrapping_add(
                (5 as png_alloc_size_t).wrapping_mul(
                    idat_limit
                        .wrapping_div(row_factor as png_alloc_size_t)
                        .wrapping_add(1 as png_alloc_size_t),
                ),
            ) as c_ulong,
        ) as png_alloc_size_t as png_alloc_size_t;
        idat_limit = if idat_limit < PNG_UINT_31_MAX as png_alloc_size_t {
            idat_limit
        } else {
            PNG_UINT_31_MAX as png_alloc_size_t
        };
        limit = if limit < idat_limit {
            idat_limit
        } else {
            limit
        };
    }
    if length as png_alloc_size_t > limit {
        png_chunk_error(
            png_ptr,
            b"chunk data is too large\0" as *const u8 as png_const_charp,
        );
    }
} }
#[no_mangle]
pub extern "C" fn png_combine_row(
    mut png_ptr: png_const_structrp,
    mut dp: png_bytep,
    mut display: c_int,
) {
    unsafe {
        let mut pixel_depth: c_uint = (*png_ptr).transformed_pixel_depth as c_uint;
        let mut sp: png_const_bytep = (*png_ptr).row_buf.offset(1) as png_const_bytep;
        let mut row_width: png_alloc_size_t = (*png_ptr).width as png_alloc_size_t;
        let pass: c_uint = (*png_ptr).pass as c_uint;
        let mut end_ptr: png_bytep = ::core::ptr::null_mut::<png_byte>();
        let mut end_byte: png_byte = 0;
        let mut end_mask: c_uint;

        if pixel_depth == 0 {
            png_error(
                png_ptr,
                b"internal row logic error\0" as *const u8 as png_const_charp,
            );
        }

        let row_bytes: png_size_t = if pixel_depth >= 8 {
            row_width.wrapping_mul((pixel_depth >> 3) as png_size_t)
        } else {
            row_width
                .wrapping_mul(pixel_depth as png_size_t)
                .wrapping_add(7 as png_size_t)
                >> 3
        };

        if (*png_ptr).info_rowbytes != 0 && (*png_ptr).info_rowbytes != row_bytes {
            png_error(
                png_ptr,
                b"internal row size calculation error\0" as *const u8 as png_const_charp,
            );
        }
        if row_width == 0 {
            png_error(
                png_ptr,
                b"internal row width error\0" as *const u8 as png_const_charp,
            );
        }

        end_mask = ((pixel_depth as png_alloc_size_t).wrapping_mul(row_width) & 7) as c_uint;
        if end_mask != 0 {
            // SAFETY: row_bytes > 0 when end_mask != 0 because row_width > 0 and pixel_depth > 0.
            end_ptr = dp.offset(row_bytes as isize - 1);
            end_byte = *end_ptr;
            end_mask = (0xff >> end_mask) as c_uint;
        }

        if (*png_ptr).interlaced as c_int != 0
            && ((*png_ptr).transformations as c_uint & PNG_INTERLACE) != 0
            && pass < 6
            && (display == 0 || (display == 1 && (pass & 1) != 0))
        {
            let initial_offset_pixels: png_alloc_size_t =
                (((1 as c_uint & pass) << (3 as c_uint).wrapping_sub((pass + 1) >> 1)) & 7)
                    as png_alloc_size_t;

            if row_width <= initial_offset_pixels {
                return;
            }

            if pixel_depth < 8 {
                let pixels_per_byte: png_uint_32 = 8u32.wrapping_div(pixel_depth as png_uint_32);
                let pd_index = if pixel_depth == 1 {
                    0usize
                } else if pixel_depth == 2 {
                    1usize
                } else {
                    2usize
                };
                let mut mask: png_uint_32 = if display != 0 {
                    display_mask[1][pd_index][(pass >> 1) as usize] as png_uint_32
                } else {
                    row_mask[1][pd_index][pass as usize] as png_uint_32
                };

                loop {
                    let mut m = mask;
                    mask = (m >> 8) | (m << 24);
                    m &= 0xff;
                    if m != 0 {
                        if m == 0xff {
                            *dp = *sp;
                        } else {
                            *dp = ((*dp as png_uint_32 & !m) | (*sp as png_uint_32 & m)) as png_byte;
                        }
                    }
                    if row_width <= pixels_per_byte as png_alloc_size_t {
                        break;
                    }
                    row_width = row_width.wrapping_sub(pixels_per_byte as png_alloc_size_t);
                    // SAFETY: advancing within the current row byte buffers one byte at a time.
                    dp = dp.offset(1);
                    // SAFETY: advancing within the current row byte buffers one byte at a time.
                    sp = sp.offset(1);
                }
            } else {
                if (pixel_depth & 7) != 0 {
                    png_error(
                        png_ptr,
                        b"invalid user transform pixel depth\0" as *const u8 as png_const_charp,
                    );
                }

                pixel_depth >>= 3;
                row_width = row_width.wrapping_mul(pixel_depth as png_alloc_size_t);

                let offset: c_uint = ((((1 as c_uint & pass)
                    << (3 as c_uint).wrapping_sub((pass + 1) >> 1))
                    & 7)
                    .wrapping_mul(pixel_depth)) as c_uint;

                row_width = row_width.wrapping_sub(offset as png_alloc_size_t);
                // SAFETY: offset is derived from pass geometry and bounded by the earlier row_width check.
                dp = dp.offset(offset as isize);
                // SAFETY: offset is derived from pass geometry and bounded by the earlier row_width check.
                sp = sp.offset(offset as isize);

                let mut bytes_to_copy: c_uint = if display != 0 {
                    let mut btc = (((1 as c_int) << ((6 as c_uint).wrapping_sub(pass) >> 1))
                        as c_uint)
                        .wrapping_mul(pixel_depth);
                    if btc as png_alloc_size_t > row_width {
                        btc = row_width as c_uint;
                    }
                    btc
                } else {
                    pixel_depth
                };

                let bytes_to_jump: c_uint =
                    (((1 as c_int) << ((7 as c_uint).wrapping_sub(pass) >> 1)) as c_uint)
                        .wrapping_mul(pixel_depth);

                match bytes_to_copy {
                    1 => loop {
                        *dp = *sp;
                        if row_width <= bytes_to_jump as png_alloc_size_t {
                            return;
                        }
                        // SAFETY: jump stays within the row stride pattern.
                        dp = dp.offset(bytes_to_jump as isize);
                        // SAFETY: jump stays within the row stride pattern.
                        sp = sp.offset(bytes_to_jump as isize);
                        row_width = row_width.wrapping_sub(bytes_to_jump as png_alloc_size_t);
                    },
                    2 => {
                        loop {
                            *dp = *sp;
                            *dp.offset(1) = *sp.offset(1);
                            if row_width <= bytes_to_jump as png_alloc_size_t {
                                return;
                            }
                            // SAFETY: jump stays within the row stride pattern.
                            sp = sp.offset(bytes_to_jump as isize);
                            // SAFETY: jump stays within the row stride pattern.
                            dp = dp.offset(bytes_to_jump as isize);
                            row_width = row_width.wrapping_sub(bytes_to_jump as png_alloc_size_t);
                            if !(row_width > 1) {
                                break;
                            }
                        }
                        *dp = *sp;
                        return;
                    }
                    3 => loop {
                        *dp = *sp;
                        *dp.offset(1) = *sp.offset(1);
                        *dp.offset(2) = *sp.offset(2);
                        if row_width <= bytes_to_jump as png_alloc_size_t {
                            return;
                        }
                        // SAFETY: jump stays within the row stride pattern.
                        sp = sp.offset(bytes_to_jump as isize);
                        // SAFETY: jump stays within the row stride pattern.
                        dp = dp.offset(bytes_to_jump as isize);
                        row_width = row_width.wrapping_sub(bytes_to_jump as png_alloc_size_t);
                    },
                    _ => {
                        let dp_addr = dp as usize;
                        let sp_addr = sp as usize;
                        let align16 = ::core::mem::align_of::<png_uint_16>();
                        let align32 = ::core::mem::align_of::<png_uint_32>();

                        if bytes_to_copy < 16
                            && (dp_addr & (align16 - 1)) == 0
                            && (sp_addr & (align16 - 1)) == 0
                            && (bytes_to_copy as usize) % ::core::mem::size_of::<png_uint_16>() == 0
                            && (bytes_to_jump as usize) % ::core::mem::size_of::<png_uint_16>() == 0
                        {
                            if (dp_addr & (align32 - 1)) == 0
                                && (sp_addr & (align32 - 1)) == 0
                                && (bytes_to_copy as usize)
                                    % ::core::mem::size_of::<png_uint_32>()
                                    == 0
                                && (bytes_to_jump as usize)
                                    % ::core::mem::size_of::<png_uint_32>()
                                    == 0
                            {
                                let mut dp32: png_uint_32p = dp as *mut c_void as png_uint_32p;
                                let mut sp32: png_const_uint_32p =
                                    sp as *const c_void as png_const_uint_32p;
                                let skip: size_t = (bytes_to_jump.wrapping_sub(bytes_to_copy)
                                    as size_t)
                                    / ::core::mem::size_of::<png_uint_32>();

                                loop {
                                    let mut c = bytes_to_copy as size_t;
                                    loop {
                                        let v = *sp32;
                                        // SAFETY: pointers are aligned and advanced within the selected copy span.
                                        sp32 = sp32.offset(1);
                                        // SAFETY: pointers are aligned and advanced within the selected copy span.
                                        *dp32 = v;
                                        // SAFETY: pointers are aligned and advanced within the selected copy span.
                                        dp32 = dp32.offset(1);
                                        c -= ::core::mem::size_of::<png_uint_32>();
                                        if c == 0 {
                                            break;
                                        }
                                    }
                                    if row_width <= bytes_to_jump as png_alloc_size_t {
                                        return;
                                    }
                                    // SAFETY: skip advances by whole aligned elements within the row pattern.
                                    dp32 = dp32.offset(skip as isize);
                                    // SAFETY: skip advances by whole aligned elements within the row pattern.
                                    sp32 = sp32.offset(skip as isize);
                                    row_width = row_width.wrapping_sub(bytes_to_jump as png_alloc_size_t);
                                    if !(bytes_to_copy as png_alloc_size_t <= row_width) {
                                        break;
                                    }
                                }

                                dp = dp32 as png_bytep;
                                sp = sp32 as png_const_bytep;
                                while row_width > 0 {
                                    *dp = *sp;
                                    // SAFETY: tail copy advances one byte at a time within remaining row_width.
                                    dp = dp.offset(1);
                                    // SAFETY: tail copy advances one byte at a time within remaining row_width.
                                    sp = sp.offset(1);
                                    row_width = row_width.wrapping_sub(1);
                                }
                                return;
                            } else {
                                let mut dp16: png_uint_16p = dp as *mut c_void as png_uint_16p;
                                let mut sp16: png_const_uint_16p =
                                    sp as *const c_void as png_const_uint_16p;
                                let skip_0: size_t = (bytes_to_jump.wrapping_sub(bytes_to_copy)
                                    as size_t)
                                    / ::core::mem::size_of::<png_uint_16>();

                                loop {
                                    let mut c_0 = bytes_to_copy as size_t;
                                    loop {
                                        let v = *sp16;
                                        // SAFETY: pointers are aligned and advanced within the selected copy span.
                                        sp16 = sp16.offset(1);
                                        // SAFETY: pointers are aligned and advanced within the selected copy span.
                                        *dp16 = v;
                                        // SAFETY: pointers are aligned and advanced within the selected copy span.
                                        dp16 = dp16.offset(1);
                                        c_0 -= ::core::mem::size_of::<png_uint_16>();
                                        if c_0 == 0 {
                                            break;
                                        }
                                    }
                                    if row_width <= bytes_to_jump as png_alloc_size_t {
                                        return;
                                    }
                                    // SAFETY: skip advances by whole aligned elements within the row pattern.
                                    dp16 = dp16.offset(skip_0 as isize);
                                    // SAFETY: skip advances by whole aligned elements within the row pattern.
                                    sp16 = sp16.offset(skip_0 as isize);
                                    row_width = row_width.wrapping_sub(bytes_to_jump as png_alloc_size_t);
                                    if !(bytes_to_copy as png_alloc_size_t <= row_width) {
                                        break;
                                    }
                                }

                                dp = dp16 as png_bytep;
                                sp = sp16 as png_const_bytep;
                                while row_width > 0 {
                                    *dp = *sp;
                                    // SAFETY: tail copy advances one byte at a time within remaining row_width.
                                    dp = dp.offset(1);
                                    // SAFETY: tail copy advances one byte at a time within remaining row_width.
                                    sp = sp.offset(1);
                                    row_width = row_width.wrapping_sub(1);
                                }
                                return;
                            }
                        }

                        loop {
                            memcpy(
                                dp as *mut c_void,
                                sp as *const c_void,
                                bytes_to_copy as size_t,
                            );
                            if row_width <= bytes_to_jump as png_alloc_size_t {
                                return;
                            }
                            // SAFETY: jump stays within the row stride pattern.
                            sp = sp.offset(bytes_to_jump as isize);
                            // SAFETY: jump stays within the row stride pattern.
                            dp = dp.offset(bytes_to_jump as isize);
                            row_width = row_width.wrapping_sub(bytes_to_jump as png_alloc_size_t);
                            if bytes_to_copy as png_alloc_size_t > row_width {
                                bytes_to_copy = row_width as c_uint;
                            }
                        }
                    }
                }
            }
        } else {
            memcpy(dp as *mut c_void, sp as *const c_void, row_bytes);
        }

        if !end_ptr.is_null() {
            *end_ptr = ((end_byte as c_uint & end_mask) | (*end_ptr as c_uint & !end_mask))
                as png_byte;
        }
    }
}
#[inline]
pub fn png_do_read_interlace(
    mut row_info: png_row_infop,
    mut row: png_bytep,
    mut pass: c_int,
    mut transformations: png_uint_32,
) { unsafe {
    static PNG_PASS_INC: [c_uint; 7] = [8, 8, 4, 4, 2, 2, 1];

    if row.is_null() || row_info.is_null() {
        return;
    }

    let width = (*row_info).width;
    let pixel_depth = (*row_info).pixel_depth as c_int;
    let jstop = PNG_PASS_INC[pass as usize] as c_int;
    let final_width = (width as c_uint).wrapping_mul(PNG_PASS_INC[pass as usize]) as png_uint_32;

    match pixel_depth {
        1 => {
            let mut sp = row.offset((((width as c_uint).wrapping_sub(1) >> 3) as png_size_t) as isize);
            let mut dp = row.offset((((final_width as c_uint).wrapping_sub(1) >> 3) as png_size_t) as isize);
            let mut sshift = (7 as c_uint).wrapping_sub((width as c_uint).wrapping_add(7) & 7);
            let mut dshift = (7 as c_uint).wrapping_sub((final_width as c_uint).wrapping_add(7) & 7);
            let mut i = 0 as png_uint_32;

            while i < width {
                let v = ((*sp as c_int >> sshift) & 0x1) as png_byte;
                let mut j = 0;
                while j < jstop {
                    *dp = ((*dp as c_uint & !(1u32 << dshift)) | ((v as c_uint) << dshift)) as png_byte;
                    if dshift == 7 {
                        dshift = 0;
                        dp = dp.offset(-1);
                    } else {
                        dshift += 1;
                    }
                    j += 1;
                }

                if sshift == 7 {
                    sshift = 0;
                    sp = sp.offset(-1);
                } else {
                    sshift += 1;
                }
                i = i.wrapping_add(1);
            }
        }
        2 => {
            let mut sp = row.offset((((width as c_uint).wrapping_sub(1) >> 2) as isize));
            let mut dp = row.offset((((final_width as c_uint).wrapping_sub(1) >> 2) as isize));
            let mut sshift = (3 as c_uint).wrapping_sub((width as c_uint).wrapping_add(3) & 3) << 1;
            let mut dshift = (3 as c_uint).wrapping_sub((final_width as c_uint).wrapping_add(3) & 3) << 1;
            let mut i = 0 as png_uint_32;

            while i < width {
                let v = ((*sp as c_int >> sshift) & 0x3) as png_byte;
                let mut j = 0;
                while j < jstop {
                    *dp = ((*dp as c_uint & !(3u32 << dshift)) | ((v as c_uint) << dshift)) as png_byte;
                    if dshift == 6 {
                        dshift = 0;
                        dp = dp.offset(-1);
                    } else {
                        dshift += 2;
                    }
                    j += 1;
                }

                if sshift == 6 {
                    sshift = 0;
                    sp = sp.offset(-1);
                } else {
                    sshift += 2;
                }
                i = i.wrapping_add(1);
            }
        }
        4 => {
            let mut sp = row.offset((((width as c_uint).wrapping_sub(1) >> 1) as png_size_t) as isize);
            let mut dp = row.offset((((final_width as c_uint).wrapping_sub(1) >> 1) as png_size_t) as isize);
            let mut sshift = (1 as c_uint).wrapping_sub((width as c_uint).wrapping_add(1) & 1) << 2;
            let mut dshift = (1 as c_uint).wrapping_sub((final_width as c_uint).wrapping_add(1) & 1) << 2;
            let mut i = 0 as png_uint_32;

            while i < width {
                let v = ((*sp as c_int >> sshift) & 0xf) as png_byte;
                let mut j = 0;
                while j < jstop {
                    *dp = ((*dp as c_uint & !(0xFu32 << dshift)) | ((v as c_uint) << dshift)) as png_byte;
                    if dshift == 4 {
                        dshift = 0;
                        dp = dp.offset(-1);
                    } else {
                        dshift += 4;
                    }
                    j += 1;
                }

                if sshift == 4 {
                    sshift = 0;
                    sp = sp.offset(-1);
                } else {
                    sshift += 4;
                }
                i = i.wrapping_add(1);
            }
        }
        _ => {
            let pixel_bytes = (pixel_depth >> 3) as png_size_t;
            let mut sp = row.offset((((width as c_uint).wrapping_sub(1) as png_size_t).wrapping_mul(pixel_bytes)) as isize);
            let mut dp = row.offset((((final_width as c_uint).wrapping_sub(1) as png_size_t).wrapping_mul(pixel_bytes)) as isize);
            let mut i = 0 as png_uint_32;

            while i < width {
                let mut j = 0;
                while j < jstop {
                    // SAFETY: `sp` and `dp` point within the same valid row buffer, and `pixel_bytes`
                    // is the exact size of one pixel for the current format; source and destination
                    // regions may overlap, so `copy` (memmove semantics) preserves original behavior.
                    std::ptr::copy(sp, dp, pixel_bytes);
                    dp = dp.offset(-(pixel_bytes as isize));
                    j += 1;
                }
                sp = sp.offset(-(pixel_bytes as isize));
                i = i.wrapping_add(1);
            }
        }
    }

    (*row_info).width = final_width;
    (*row_info).rowbytes =
        if (*row_info).pixel_depth as c_int >= 8 {
            (final_width as png_size_t)
                .wrapping_mul((*row_info).pixel_depth as png_size_t >> 3)
        } else {
            (final_width as png_size_t)
                .wrapping_mul((*row_info).pixel_depth as png_size_t)
                .wrapping_add(7)
                >> 3
        };

    let _ = transformations;
} }
extern "C" fn png_read_filter_row_sub(
    mut row_info: png_row_infop,
    mut row: png_bytep,
    mut prev_row: png_const_bytep,
) { unsafe {
    let mut i: png_size_t = 0;
    let mut istop: png_size_t = (*row_info).rowbytes;
    let mut bpp: c_uint = ((*row_info).pixel_depth as c_int
        + 7 as c_int
        >> 3 as c_int) as c_uint;
    let mut rp: png_bytep = row.offset(bpp as isize);
    i = bpp as png_size_t;
    while i < istop {
        *rp = (*rp as c_int + *rp.offset(-(bpp as isize)) as c_int
            & 0xff as c_int) as png_byte;
        rp = rp.offset(1);
        i = i.wrapping_add(1);
    }
} }
extern "C" fn png_read_filter_row_up(
    mut row_info: png_row_infop,
    mut row: png_bytep,
    mut prev_row: png_const_bytep,
) { unsafe {
    let mut i: png_size_t = 0;
    let mut istop: png_size_t = (*row_info).rowbytes;
    let mut rp: png_bytep = row;
    let mut pp: png_const_bytep = prev_row;
    i = 0 as png_size_t;
    while i < istop {
        let fresh18 = pp;
        pp = pp.offset(1);
        *rp = (*rp as c_int + *fresh18 as c_int
            & 0xff as c_int) as png_byte;
        rp = rp.offset(1);
        i = i.wrapping_add(1);
    }
} }
extern "C" fn png_read_filter_row_avg(
    mut row_info: png_row_infop,
    mut row: png_bytep,
    mut prev_row: png_const_bytep,
) { unsafe {
    let mut i: png_size_t = 0;
    let mut rp: png_bytep = row;
    let mut pp: png_const_bytep = prev_row;
    let mut bpp: c_uint = ((*row_info).pixel_depth as c_int
        + 7 as c_int
        >> 3 as c_int) as c_uint;
    let mut istop: png_size_t = (*row_info).rowbytes.wrapping_sub(bpp as png_size_t);
    i = 0 as png_size_t;
    while i < bpp as png_size_t {
        let fresh16 = pp;
        pp = pp.offset(1);
        *rp = (*rp as c_int + *fresh16 as c_int / 2 as c_int
            & 0xff as c_int) as png_byte;
        rp = rp.offset(1);
        i = i.wrapping_add(1);
    }
    i = 0 as png_size_t;
    while i < istop {
        let fresh17 = pp;
        pp = pp.offset(1);
        *rp = (*rp as c_int
            + (*fresh17 as c_int + *rp.offset(-(bpp as isize)) as c_int)
                / 2 as c_int
            & 0xff as c_int) as png_byte;
        rp = rp.offset(1);
        i = i.wrapping_add(1);
    }
} }
extern "C" fn png_read_filter_row_paeth_1byte_pixel(
    mut row_info: png_row_infop,
    mut row: png_bytep,
    mut prev_row: png_const_bytep,
) { unsafe {
    let mut rp_end: png_bytep = row.offset((*row_info).rowbytes as isize);
    let mut a: c_int = 0;
    let mut c: c_int = 0;
    let fresh12 = prev_row;
    prev_row = prev_row.offset(1);
    c = *fresh12 as c_int;
    a = *row as c_int + c;
    let fresh13 = row;
    row = row.offset(1);
    *fresh13 = a as png_byte;
    while row < rp_end {
        let mut b: c_int = 0;
        let mut pa: c_int = 0;
        let mut pb: c_int = 0;
        let mut pc: c_int = 0;
        let mut p: c_int = 0;
        a &= 0xff as c_int;
        let fresh14 = prev_row;
        prev_row = prev_row.offset(1);
        b = *fresh14 as c_int;
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
        if pb < pa {
            pa = pb;
            a = b;
        }
        if pc < pa {
            a = c;
        }
        c = b;
        a += *row as c_int;
        let fresh15 = row;
        row = row.offset(1);
        *fresh15 = a as png_byte;
    }
} }
extern "C" fn png_read_filter_row_paeth_multibyte_pixel(
    mut row_info: png_row_infop,
    mut row: png_bytep,
    mut prev_row: png_const_bytep,
) { unsafe {
    let mut bpp: c_uint = ((*row_info).pixel_depth as c_int
        + 7 as c_int
        >> 3 as c_int) as c_uint;
    let mut rp_end: png_bytep = row.offset(bpp as isize);
    while row < rp_end {
        let fresh8 = prev_row;
        prev_row = prev_row.offset(1);
        let mut a: c_int = *row as c_int + *fresh8 as c_int;
        let fresh9 = row;
        row = row.offset(1);
        *fresh9 = a as png_byte;
    }
    rp_end = rp_end.offset((*row_info).rowbytes.wrapping_sub(bpp as png_size_t) as isize);
    while row < rp_end {
        let mut a_0: c_int = 0;
        let mut b: c_int = 0;
        let mut c: c_int = 0;
        let mut pa: c_int = 0;
        let mut pb: c_int = 0;
        let mut pc: c_int = 0;
        let mut p: c_int = 0;
        c = *prev_row.offset(-(bpp as isize)) as c_int;
        a_0 = *row.offset(-(bpp as isize)) as c_int;
        let fresh10 = prev_row;
        prev_row = prev_row.offset(1);
        b = *fresh10 as c_int;
        p = b - c;
        pc = a_0 - c;
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
        if pb < pa {
            pa = pb;
            a_0 = b;
        }
        if pc < pa {
            a_0 = c;
        }
        a_0 += *row as c_int;
        let fresh11 = row;
        row = row.offset(1);
        *fresh11 = a_0 as png_byte;
    }
} }
fn png_init_filter_functions(mut pp: png_structrp) { unsafe {
    let mut bpp: c_uint = ((*pp).pixel_depth as c_int
        + 7 as c_int
        >> 3 as c_int) as c_uint;
    (*pp).read_filter[(PNG_FILTER_VALUE_SUB - 1 as c_int) as usize] = Some(
        png_read_filter_row_sub
            as unsafe extern "C" fn(png_row_infop, png_bytep, png_const_bytep) -> (),
    )
        as Option<unsafe extern "C" fn(png_row_infop, png_bytep, png_const_bytep) -> ()>;
    (*pp).read_filter[(PNG_FILTER_VALUE_UP - 1 as c_int) as usize] = Some(
        png_read_filter_row_up
            as unsafe extern "C" fn(png_row_infop, png_bytep, png_const_bytep) -> (),
    )
        as Option<unsafe extern "C" fn(png_row_infop, png_bytep, png_const_bytep) -> ()>;
    (*pp).read_filter[(PNG_FILTER_VALUE_AVG - 1 as c_int) as usize] = Some(
        png_read_filter_row_avg
            as unsafe extern "C" fn(png_row_infop, png_bytep, png_const_bytep) -> (),
    )
        as Option<unsafe extern "C" fn(png_row_infop, png_bytep, png_const_bytep) -> ()>;
    if bpp == 1 as c_uint {
        (*pp).read_filter[(PNG_FILTER_VALUE_PAETH - 1 as c_int) as usize] = Some(
            png_read_filter_row_paeth_1byte_pixel
                as unsafe extern "C" fn(png_row_infop, png_bytep, png_const_bytep) -> (),
        )
            as Option<unsafe extern "C" fn(png_row_infop, png_bytep, png_const_bytep) -> ()>;
    } else {
        (*pp).read_filter[(PNG_FILTER_VALUE_PAETH - 1 as c_int) as usize] = Some(
            png_read_filter_row_paeth_multibyte_pixel
                as unsafe extern "C" fn(png_row_infop, png_bytep, png_const_bytep) -> (),
        )
            as Option<unsafe extern "C" fn(png_row_infop, png_bytep, png_const_bytep) -> ()>;
    };
} }
#[no_mangle]
pub extern "C" fn png_read_filter_row(
    mut pp: png_structrp,
    mut row_info: png_row_infop,
    mut row: png_bytep,
    mut prev_row: png_const_bytep,
    mut filter: c_int,
) { unsafe {
    if filter > PNG_FILTER_VALUE_NONE && filter < PNG_FILTER_VALUE_LAST {
        if (*pp).read_filter[0 as c_int as usize].is_none() {
            png_init_filter_functions(pp);
        }
        (*pp).read_filter[(filter - 1 as c_int) as usize]
            .expect("non-null function pointer")(row_info, row, prev_row);
    }
} }
#[no_mangle]
pub extern "C" fn png_read_IDAT_data(
    mut png_ptr: png_structrp,
    mut output: png_bytep,
    mut avail_out: png_alloc_size_t,
) { unsafe {
    (*png_ptr).zstream.next_out = output as *mut Bytef;
    (*png_ptr).zstream.avail_out = 0 as uInt;
    if output.is_null() {
        avail_out = 0 as png_alloc_size_t;
    }
    loop {
        let mut ret: c_int = 0;
        let mut tmpbuf: [png_byte; 1024] = [0; 1024];
        if (*png_ptr).zstream.avail_in == 0 as c_uint {
            let mut avail_in: uInt = 0;
            let mut buffer: png_bytep = ::core::ptr::null_mut::<png_byte>();
            while (*png_ptr).idat_size == 0 as c_uint {
                png_crc_finish(png_ptr, 0 as png_uint_32);
                (*png_ptr).idat_size = png_read_chunk_header(png_ptr);
                if (*png_ptr).chunk_name != png_IDAT {
                    png_error(
                        png_ptr,
                        b"Not enough image data\0" as *const u8 as png_const_charp,
                    );
                }
            }
            avail_in = (*png_ptr).IDAT_read_size;
            if avail_in > (*png_ptr).idat_size {
                avail_in = (*png_ptr).idat_size;
            }
            buffer = png_read_buffer(
                png_ptr,
                avail_in as png_alloc_size_t,
                0 as c_int,
            );
            png_crc_read(png_ptr, buffer, avail_in as png_uint_32);
            (*png_ptr).idat_size = ((*png_ptr).idat_size as c_uint)
                .wrapping_sub(avail_in as c_uint)
                as png_uint_32 as png_uint_32;
            (*png_ptr).zstream.next_in = buffer as *const Bytef;
            (*png_ptr).zstream.avail_in = avail_in;
        }
        if !output.is_null() {
            let mut out: uInt = ZLIB_IO_MAX;
            if out as png_alloc_size_t > avail_out {
                out = avail_out as uInt;
            }
            avail_out = (avail_out as c_ulong)
                .wrapping_sub(out as c_ulong)
                as png_alloc_size_t as png_alloc_size_t;
            (*png_ptr).zstream.avail_out = out;
        } else {
            (*png_ptr).zstream.next_out = &raw mut tmpbuf as *mut png_byte as *mut Bytef;
            (*png_ptr).zstream.avail_out = ::core::mem::size_of::<[png_byte; 1024]>() as uInt;
        }
        ret = png_zlib_inflate(png_ptr, 0 as c_int);
        if !output.is_null() {
            avail_out = (avail_out as c_ulong)
                .wrapping_add((*png_ptr).zstream.avail_out as c_ulong)
                as png_alloc_size_t as png_alloc_size_t;
        } else {
            avail_out = (avail_out as c_ulong).wrapping_add(
                (::core::mem::size_of::<[png_byte; 1024]>() as usize)
                    .wrapping_sub((*png_ptr).zstream.avail_out as usize)
                    as c_ulong,
            ) as png_alloc_size_t as png_alloc_size_t;
        }
        (*png_ptr).zstream.avail_out = 0 as uInt;
        if ret == Z_STREAM_END {
            (*png_ptr).zstream.next_out = ::core::ptr::null_mut::<Bytef>();
            (*png_ptr).mode |= PNG_AFTER_IDAT as c_uint;
            (*png_ptr).flags |= PNG_FLAG_ZSTREAM_ENDED;
            if (*png_ptr).zstream.avail_in > 0 as c_uint
                || (*png_ptr).idat_size > 0 as c_uint
            {
                png_chunk_benign_error(
                    png_ptr,
                    b"Extra compressed data\0" as *const u8 as png_const_charp,
                );
            }
            break;
        } else {
            if ret != Z_OK {
                png_zstream_error(png_ptr, ret);
                if !output.is_null() {
                    png_chunk_error(png_ptr, (*png_ptr).zstream.msg as png_const_charp);
                } else {
                    png_chunk_benign_error(png_ptr, (*png_ptr).zstream.msg as png_const_charp);
                    return;
                }
            }
            if !(avail_out > 0 as png_alloc_size_t) {
                break;
            }
        }
    }
    if avail_out > 0 as png_alloc_size_t {
        if !output.is_null() {
            png_error(
                png_ptr,
                b"Not enough image data\0" as *const u8 as png_const_charp,
            );
        } else {
            png_chunk_benign_error(
                png_ptr,
                b"Too much image data\0" as *const u8 as png_const_charp,
            );
        }
    }
} }
#[no_mangle]
pub extern "C" fn png_read_finish_IDAT(mut png_ptr: png_structrp) { unsafe {
    if (*png_ptr).flags as c_uint & PNG_FLAG_ZSTREAM_ENDED == 0 as c_uint
    {
        png_read_IDAT_data(
            png_ptr,
            ::core::ptr::null_mut::<png_byte>(),
            0 as png_alloc_size_t,
        );
        (*png_ptr).zstream.next_out = ::core::ptr::null_mut::<Bytef>();
        if (*png_ptr).flags as c_uint & PNG_FLAG_ZSTREAM_ENDED
            == 0 as c_uint
        {
            (*png_ptr).mode |= PNG_AFTER_IDAT as c_uint;
            (*png_ptr).flags |= PNG_FLAG_ZSTREAM_ENDED;
        }
    }
    if (*png_ptr).zowner == png_IDAT {
        (*png_ptr).zstream.next_in = ::core::ptr::null::<Bytef>();
        (*png_ptr).zstream.avail_in = 0 as uInt;
        (*png_ptr).zowner = 0 as png_uint_32;
        png_crc_finish(png_ptr, (*png_ptr).idat_size);
    }
} }
#[no_mangle]
pub extern "C" fn png_read_finish_row(mut png_ptr: png_structrp) { unsafe {
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
        memset(
            (*png_ptr).prev_row as *mut c_void,
            0 as c_int,
            ((*png_ptr).rowbytes as size_t).wrapping_add(1 as size_t),
        );
        loop {
            (*png_ptr).pass = (*png_ptr).pass.wrapping_add(1);
            if (*png_ptr).pass as c_int >= 7 as c_int {
                break;
            }
            (*png_ptr).iwidth = ((*png_ptr).width as c_uint)
                .wrapping_add(png_pass_inc[(*png_ptr).pass as usize] as c_uint)
                .wrapping_sub(1 as c_uint)
                .wrapping_sub(png_pass_start[(*png_ptr).pass as usize] as c_uint)
                .wrapping_div(png_pass_inc[(*png_ptr).pass as usize] as c_uint)
                as png_uint_32;
            if !((*png_ptr).transformations as c_uint & PNG_INTERLACE
                == 0 as c_uint)
            {
                break;
            }
            (*png_ptr).num_rows = ((*png_ptr).height as c_uint)
                .wrapping_add(png_pass_yinc[(*png_ptr).pass as usize] as c_uint)
                .wrapping_sub(1 as c_uint)
                .wrapping_sub(png_pass_ystart[(*png_ptr).pass as usize] as c_uint)
                .wrapping_div(png_pass_yinc[(*png_ptr).pass as usize] as c_uint)
                as png_uint_32;
            if !((*png_ptr).num_rows == 0 as c_uint
                || (*png_ptr).iwidth == 0 as c_uint)
            {
                break;
            }
        }
        if ((*png_ptr).pass as c_int) < 7 as c_int {
            return;
        }
    }
    png_read_finish_IDAT(png_ptr);
} }
#[no_mangle]
pub extern "C" fn png_read_start_row(mut png_ptr: png_structrp) { unsafe {
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
    let mut max_pixel_depth: c_uint = 0;
    let mut row_bytes: png_size_t = 0;
    png_init_read_transformations(png_ptr);
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
        } else {
            (*png_ptr).num_rows = (*png_ptr).height;
        }
        (*png_ptr).iwidth = ((*png_ptr).width as c_uint)
            .wrapping_add(png_pass_inc[(*png_ptr).pass as usize] as c_uint)
            .wrapping_sub(1 as c_uint)
            .wrapping_sub(png_pass_start[(*png_ptr).pass as usize] as c_uint)
            .wrapping_div(png_pass_inc[(*png_ptr).pass as usize] as c_uint)
            as png_uint_32;
    } else {
        (*png_ptr).num_rows = (*png_ptr).height;
        (*png_ptr).iwidth = (*png_ptr).width;
    }
    max_pixel_depth = (*png_ptr).pixel_depth as c_uint;
    (*png_ptr).maximum_pixel_depth = max_pixel_depth as png_byte;
    (*png_ptr).transformed_pixel_depth = 0 as png_byte;
    row_bytes = ((*png_ptr).width.wrapping_add(7 as png_uint_32)
        & !(7 as c_int as png_uint_32)) as png_size_t;
    row_bytes = (if max_pixel_depth >= 8 as c_uint {
        row_bytes.wrapping_mul(max_pixel_depth as png_size_t >> 3 as c_int)
    } else {
        row_bytes
            .wrapping_mul(max_pixel_depth as png_size_t)
            .wrapping_add(7 as png_size_t)
            >> 3 as c_int
    })
    .wrapping_add(1 as png_size_t)
    .wrapping_add(
        (max_pixel_depth.wrapping_add(7 as c_uint) >> 3 as c_uint)
            as png_size_t,
    );
    if row_bytes.wrapping_add(48 as png_size_t) > (*png_ptr).old_big_row_buf_size {
        png_free(png_ptr, (*png_ptr).big_row_buf as png_voidp);
        png_free(png_ptr, (*png_ptr).big_prev_row as png_voidp);
        if (*png_ptr).interlaced as c_int != 0 as c_int {
            (*png_ptr).big_row_buf = png_calloc(
                png_ptr,
                (row_bytes as png_alloc_size_t).wrapping_add(48 as png_alloc_size_t),
            ) as png_bytep;
        } else {
            (*png_ptr).big_row_buf = png_malloc(
                png_ptr,
                (row_bytes as png_alloc_size_t).wrapping_add(48 as png_alloc_size_t),
            ) as png_bytep;
        }
        (*png_ptr).big_prev_row = png_malloc(
            png_ptr,
            (row_bytes as png_alloc_size_t).wrapping_add(48 as png_alloc_size_t),
        ) as png_bytep;
        let mut temp: png_bytep = (*png_ptr)
            .big_row_buf
            .offset(32 as c_int as isize);
        let mut extra: c_int =
            (temp.offset_from(::core::ptr::null_mut::<png_byte>()) as c_long
                & 0xf as c_long) as c_int;
        (*png_ptr).row_buf = temp
            .offset(-(extra as isize))
            .offset(-(1 as c_int as isize));
        temp = (*png_ptr)
            .big_prev_row
            .offset(32 as c_int as isize);
        extra = (temp.offset_from(::core::ptr::null_mut::<png_byte>()) as c_long
            & 0xf as c_long) as c_int;
        (*png_ptr).prev_row = temp
            .offset(-(extra as isize))
            .offset(-(1 as c_int as isize));
        (*png_ptr).old_big_row_buf_size = row_bytes.wrapping_add(48 as png_size_t);
    }
    if (*png_ptr).rowbytes > PNG_SIZE_MAX.wrapping_sub(1 as png_size_t) {
        png_error(
            png_ptr,
            b"Row has too many bytes to allocate in memory\0" as *const u8 as png_const_charp,
        );
    }
    memset(
        (*png_ptr).prev_row as *mut c_void,
        0 as c_int,
        ((*png_ptr).rowbytes as size_t).wrapping_add(1 as size_t),
    );
    if !(*png_ptr).read_buffer.is_null() {
        let mut buffer: png_bytep = (*png_ptr).read_buffer;
        (*png_ptr).read_buffer_size = 0 as png_alloc_size_t;
        (*png_ptr).read_buffer = ::core::ptr::null_mut::<png_byte>();
        png_free(png_ptr, buffer as png_voidp);
    }
    if png_inflate_claim(png_ptr, png_IDAT) != Z_OK {
        png_error(png_ptr, (*png_ptr).zstream.msg as png_const_charp);
    }
    (*png_ptr).flags |= PNG_FLAG_ROW_INIT;
} }
extern "C" fn run_static_initializers() { unsafe {
    row_mask = [
        [
            [
                (if (if (0 as c_int) < 4 as c_int {
                    0x80088822 as c_uint
                        >> ((3 as c_int - 0 as c_int)
                            * 8 as c_int
                            + (7 as c_int - 0 as c_int)
                            & 0x1f as c_int)
                } else {
                    0xaa55ff00 as c_uint
                        >> ((7 as c_int - 0 as c_int)
                            * 8 as c_int
                            + (7 as c_int - 0 as c_int)
                            & 0x1f as c_int)
                }) & 1 as c_uint
                    != 0
                {
                    ((1 as c_uint)
                        << (1 as c_int & 0x1f as c_int))
                        .wrapping_sub(1 as c_uint)
                        << ((0 as c_int * 1 as c_int
                            ^ (if 0 as c_int != 0 {
                                8 as c_int - 1 as c_int
                            } else {
                                0 as c_int
                            }))
                            & 0x1f as c_int)
                } else {
                    0 as c_uint
                })
                .wrapping_add(
                    (if (if (0 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 0 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 1 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 0 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 1 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (1 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((1 as c_int * 1 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 1 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (0 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 0 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 2 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 0 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 2 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (1 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((2 as c_int * 1 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 1 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (0 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 0 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 3 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 0 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 3 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (1 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((3 as c_int * 1 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 1 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (0 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 0 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 4 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 0 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 4 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (1 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((4 as c_int * 1 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 1 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (0 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 0 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 5 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 0 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 5 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (1 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((5 as c_int * 1 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 1 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (0 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 0 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 6 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 0 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 6 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (1 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((6 as c_int * 1 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 1 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (0 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 0 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 7 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 0 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 7 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (1 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((7 as c_int * 1 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 1 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_mul(
                    (if 1 as c_int == 1 as c_int {
                        0x1010101 as c_int
                    } else {
                        (if 1 as c_int == 2 as c_int {
                            0x10001 as c_int
                        } else {
                            1 as c_int
                        })
                    }) as c_uint,
                ),
                (if (if (1 as c_int) < 4 as c_int {
                    0x80088822 as c_uint
                        >> ((3 as c_int - 1 as c_int)
                            * 8 as c_int
                            + (7 as c_int - 0 as c_int)
                            & 0x1f as c_int)
                } else {
                    0xaa55ff00 as c_uint
                        >> ((7 as c_int - 1 as c_int)
                            * 8 as c_int
                            + (7 as c_int - 0 as c_int)
                            & 0x1f as c_int)
                }) & 1 as c_uint
                    != 0
                {
                    ((1 as c_uint)
                        << (1 as c_int & 0x1f as c_int))
                        .wrapping_sub(1 as c_uint)
                        << ((0 as c_int * 1 as c_int
                            ^ (if 0 as c_int != 0 {
                                8 as c_int - 1 as c_int
                            } else {
                                0 as c_int
                            }))
                            & 0x1f as c_int)
                } else {
                    0 as c_uint
                })
                .wrapping_add(
                    (if (if (1 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 1 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 1 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (1 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((1 as c_int * 1 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 1 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (1 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 2 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 2 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (1 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((2 as c_int * 1 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 1 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (1 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 3 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 3 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (1 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((3 as c_int * 1 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 1 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (1 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 4 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 4 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (1 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((4 as c_int * 1 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 1 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (1 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 5 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 5 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (1 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((5 as c_int * 1 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 1 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (1 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 6 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 6 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (1 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((6 as c_int * 1 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 1 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (1 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 7 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 7 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (1 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((7 as c_int * 1 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 1 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_mul(
                    (if 1 as c_int == 1 as c_int {
                        0x1010101 as c_int
                    } else {
                        (if 1 as c_int == 2 as c_int {
                            0x10001 as c_int
                        } else {
                            1 as c_int
                        })
                    }) as c_uint,
                ),
                (if (if (2 as c_int) < 4 as c_int {
                    0x80088822 as c_uint
                        >> ((3 as c_int - 2 as c_int)
                            * 8 as c_int
                            + (7 as c_int - 0 as c_int)
                            & 0x1f as c_int)
                } else {
                    0xaa55ff00 as c_uint
                        >> ((7 as c_int - 2 as c_int)
                            * 8 as c_int
                            + (7 as c_int - 0 as c_int)
                            & 0x1f as c_int)
                }) & 1 as c_uint
                    != 0
                {
                    ((1 as c_uint)
                        << (1 as c_int & 0x1f as c_int))
                        .wrapping_sub(1 as c_uint)
                        << ((0 as c_int * 1 as c_int
                            ^ (if 0 as c_int != 0 {
                                8 as c_int - 1 as c_int
                            } else {
                                0 as c_int
                            }))
                            & 0x1f as c_int)
                } else {
                    0 as c_uint
                })
                .wrapping_add(
                    (if (if (2 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 2 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 1 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 2 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 1 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (1 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((1 as c_int * 1 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 1 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (2 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 2 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 2 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 2 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 2 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (1 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((2 as c_int * 1 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 1 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (2 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 2 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 3 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 2 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 3 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (1 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((3 as c_int * 1 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 1 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (2 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 2 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 4 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 2 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 4 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (1 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((4 as c_int * 1 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 1 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (2 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 2 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 5 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 2 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 5 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (1 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((5 as c_int * 1 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 1 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (2 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 2 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 6 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 2 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 6 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (1 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((6 as c_int * 1 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 1 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (2 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 2 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 7 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 2 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 7 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (1 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((7 as c_int * 1 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 1 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_mul(
                    (if 1 as c_int == 1 as c_int {
                        0x1010101 as c_int
                    } else {
                        (if 1 as c_int == 2 as c_int {
                            0x10001 as c_int
                        } else {
                            1 as c_int
                        })
                    }) as c_uint,
                ),
                (if (if (3 as c_int) < 4 as c_int {
                    0x80088822 as c_uint
                        >> ((3 as c_int - 3 as c_int)
                            * 8 as c_int
                            + (7 as c_int - 0 as c_int)
                            & 0x1f as c_int)
                } else {
                    0xaa55ff00 as c_uint
                        >> ((7 as c_int - 3 as c_int)
                            * 8 as c_int
                            + (7 as c_int - 0 as c_int)
                            & 0x1f as c_int)
                }) & 1 as c_uint
                    != 0
                {
                    ((1 as c_uint)
                        << (1 as c_int & 0x1f as c_int))
                        .wrapping_sub(1 as c_uint)
                        << ((0 as c_int * 1 as c_int
                            ^ (if 0 as c_int != 0 {
                                8 as c_int - 1 as c_int
                            } else {
                                0 as c_int
                            }))
                            & 0x1f as c_int)
                } else {
                    0 as c_uint
                })
                .wrapping_add(
                    (if (if (3 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 1 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 1 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (1 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((1 as c_int * 1 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 1 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (3 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 2 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 2 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (1 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((2 as c_int * 1 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 1 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (3 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 3 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 3 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (1 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((3 as c_int * 1 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 1 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (3 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 4 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 4 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (1 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((4 as c_int * 1 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 1 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (3 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 5 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 5 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (1 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((5 as c_int * 1 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 1 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (3 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 6 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 6 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (1 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((6 as c_int * 1 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 1 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (3 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 7 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 7 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (1 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((7 as c_int * 1 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 1 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_mul(
                    (if 1 as c_int == 1 as c_int {
                        0x1010101 as c_int
                    } else {
                        (if 1 as c_int == 2 as c_int {
                            0x10001 as c_int
                        } else {
                            1 as c_int
                        })
                    }) as c_uint,
                ),
                (if (if (4 as c_int) < 4 as c_int {
                    0x80088822 as c_uint
                        >> ((3 as c_int - 4 as c_int)
                            * 8 as c_int
                            + (7 as c_int - 0 as c_int)
                            & 0x1f as c_int)
                } else {
                    0xaa55ff00 as c_uint
                        >> ((7 as c_int - 4 as c_int)
                            * 8 as c_int
                            + (7 as c_int - 0 as c_int)
                            & 0x1f as c_int)
                }) & 1 as c_uint
                    != 0
                {
                    ((1 as c_uint)
                        << (1 as c_int & 0x1f as c_int))
                        .wrapping_sub(1 as c_uint)
                        << ((0 as c_int * 1 as c_int
                            ^ (if 0 as c_int != 0 {
                                8 as c_int - 1 as c_int
                            } else {
                                0 as c_int
                            }))
                            & 0x1f as c_int)
                } else {
                    0 as c_uint
                })
                .wrapping_add(
                    (if (if (4 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 4 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 1 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 4 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 1 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (1 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((1 as c_int * 1 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 1 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (4 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 4 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 2 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 4 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 2 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (1 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((2 as c_int * 1 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 1 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (4 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 4 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 3 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 4 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 3 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (1 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((3 as c_int * 1 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 1 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (4 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 4 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 4 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 4 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 4 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (1 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((4 as c_int * 1 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 1 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (4 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 4 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 5 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 4 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 5 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (1 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((5 as c_int * 1 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 1 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (4 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 4 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 6 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 4 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 6 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (1 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((6 as c_int * 1 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 1 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (4 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 4 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 7 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 4 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 7 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (1 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((7 as c_int * 1 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 1 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_mul(
                    (if 1 as c_int == 1 as c_int {
                        0x1010101 as c_int
                    } else {
                        (if 1 as c_int == 2 as c_int {
                            0x10001 as c_int
                        } else {
                            1 as c_int
                        })
                    }) as c_uint,
                ),
                (if (if (5 as c_int) < 4 as c_int {
                    0x80088822 as c_uint
                        >> ((3 as c_int - 5 as c_int)
                            * 8 as c_int
                            + (7 as c_int - 0 as c_int)
                            & 0x1f as c_int)
                } else {
                    0xaa55ff00 as c_uint
                        >> ((7 as c_int - 5 as c_int)
                            * 8 as c_int
                            + (7 as c_int - 0 as c_int)
                            & 0x1f as c_int)
                }) & 1 as c_uint
                    != 0
                {
                    ((1 as c_uint)
                        << (1 as c_int & 0x1f as c_int))
                        .wrapping_sub(1 as c_uint)
                        << ((0 as c_int * 1 as c_int
                            ^ (if 0 as c_int != 0 {
                                8 as c_int - 1 as c_int
                            } else {
                                0 as c_int
                            }))
                            & 0x1f as c_int)
                } else {
                    0 as c_uint
                })
                .wrapping_add(
                    (if (if (5 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 1 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 1 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (1 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((1 as c_int * 1 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 1 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (5 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 2 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 2 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (1 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((2 as c_int * 1 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 1 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (5 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 3 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 3 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (1 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((3 as c_int * 1 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 1 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (5 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 4 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 4 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (1 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((4 as c_int * 1 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 1 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (5 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 5 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 5 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (1 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((5 as c_int * 1 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 1 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (5 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 6 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 6 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (1 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((6 as c_int * 1 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 1 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (5 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 7 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 7 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (1 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((7 as c_int * 1 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 1 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_mul(
                    (if 1 as c_int == 1 as c_int {
                        0x1010101 as c_int
                    } else {
                        (if 1 as c_int == 2 as c_int {
                            0x10001 as c_int
                        } else {
                            1 as c_int
                        })
                    }) as c_uint,
                ),
            ],
            [
                (if (if (0 as c_int) < 4 as c_int {
                    0x80088822 as c_uint
                        >> ((3 as c_int - 0 as c_int)
                            * 8 as c_int
                            + (7 as c_int - 0 as c_int)
                            & 0x1f as c_int)
                } else {
                    0xaa55ff00 as c_uint
                        >> ((7 as c_int - 0 as c_int)
                            * 8 as c_int
                            + (7 as c_int - 0 as c_int)
                            & 0x1f as c_int)
                }) & 1 as c_uint
                    != 0
                {
                    ((1 as c_uint)
                        << (2 as c_int & 0x1f as c_int))
                        .wrapping_sub(1 as c_uint)
                        << ((0 as c_int * 2 as c_int
                            ^ (if 0 as c_int != 0 {
                                8 as c_int - 2 as c_int
                            } else {
                                0 as c_int
                            }))
                            & 0x1f as c_int)
                } else {
                    0 as c_uint
                })
                .wrapping_add(
                    (if (if (0 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 0 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 1 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 0 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 1 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (2 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((1 as c_int * 2 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 2 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (0 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 0 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 2 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 0 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 2 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (2 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((2 as c_int * 2 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 2 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (0 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 0 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 3 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 0 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 3 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (2 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((3 as c_int * 2 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 2 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (0 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 0 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 4 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 0 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 4 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (2 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((4 as c_int * 2 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 2 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (0 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 0 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 5 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 0 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 5 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (2 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((5 as c_int * 2 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 2 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (0 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 0 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 6 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 0 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 6 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (2 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((6 as c_int * 2 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 2 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (0 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 0 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 7 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 0 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 7 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (2 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((7 as c_int * 2 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 2 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_mul(
                    (if 2 as c_int == 1 as c_int {
                        0x1010101 as c_int
                    } else {
                        (if 2 as c_int == 2 as c_int {
                            0x10001 as c_int
                        } else {
                            1 as c_int
                        })
                    }) as c_uint,
                ),
                (if (if (1 as c_int) < 4 as c_int {
                    0x80088822 as c_uint
                        >> ((3 as c_int - 1 as c_int)
                            * 8 as c_int
                            + (7 as c_int - 0 as c_int)
                            & 0x1f as c_int)
                } else {
                    0xaa55ff00 as c_uint
                        >> ((7 as c_int - 1 as c_int)
                            * 8 as c_int
                            + (7 as c_int - 0 as c_int)
                            & 0x1f as c_int)
                }) & 1 as c_uint
                    != 0
                {
                    ((1 as c_uint)
                        << (2 as c_int & 0x1f as c_int))
                        .wrapping_sub(1 as c_uint)
                        << ((0 as c_int * 2 as c_int
                            ^ (if 0 as c_int != 0 {
                                8 as c_int - 2 as c_int
                            } else {
                                0 as c_int
                            }))
                            & 0x1f as c_int)
                } else {
                    0 as c_uint
                })
                .wrapping_add(
                    (if (if (1 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 1 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 1 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (2 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((1 as c_int * 2 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 2 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (1 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 2 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 2 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (2 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((2 as c_int * 2 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 2 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (1 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 3 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 3 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (2 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((3 as c_int * 2 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 2 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (1 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 4 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 4 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (2 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((4 as c_int * 2 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 2 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (1 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 5 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 5 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (2 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((5 as c_int * 2 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 2 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (1 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 6 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 6 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (2 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((6 as c_int * 2 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 2 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (1 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 7 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 7 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (2 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((7 as c_int * 2 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 2 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_mul(
                    (if 2 as c_int == 1 as c_int {
                        0x1010101 as c_int
                    } else {
                        (if 2 as c_int == 2 as c_int {
                            0x10001 as c_int
                        } else {
                            1 as c_int
                        })
                    }) as c_uint,
                ),
                (if (if (2 as c_int) < 4 as c_int {
                    0x80088822 as c_uint
                        >> ((3 as c_int - 2 as c_int)
                            * 8 as c_int
                            + (7 as c_int - 0 as c_int)
                            & 0x1f as c_int)
                } else {
                    0xaa55ff00 as c_uint
                        >> ((7 as c_int - 2 as c_int)
                            * 8 as c_int
                            + (7 as c_int - 0 as c_int)
                            & 0x1f as c_int)
                }) & 1 as c_uint
                    != 0
                {
                    ((1 as c_uint)
                        << (2 as c_int & 0x1f as c_int))
                        .wrapping_sub(1 as c_uint)
                        << ((0 as c_int * 2 as c_int
                            ^ (if 0 as c_int != 0 {
                                8 as c_int - 2 as c_int
                            } else {
                                0 as c_int
                            }))
                            & 0x1f as c_int)
                } else {
                    0 as c_uint
                })
                .wrapping_add(
                    (if (if (2 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 2 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 1 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 2 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 1 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (2 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((1 as c_int * 2 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 2 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (2 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 2 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 2 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 2 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 2 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (2 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((2 as c_int * 2 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 2 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (2 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 2 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 3 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 2 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 3 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (2 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((3 as c_int * 2 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 2 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (2 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 2 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 4 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 2 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 4 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (2 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((4 as c_int * 2 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 2 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (2 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 2 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 5 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 2 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 5 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (2 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((5 as c_int * 2 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 2 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (2 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 2 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 6 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 2 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 6 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (2 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((6 as c_int * 2 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 2 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (2 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 2 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 7 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 2 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 7 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (2 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((7 as c_int * 2 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 2 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_mul(
                    (if 2 as c_int == 1 as c_int {
                        0x1010101 as c_int
                    } else {
                        (if 2 as c_int == 2 as c_int {
                            0x10001 as c_int
                        } else {
                            1 as c_int
                        })
                    }) as c_uint,
                ),
                (if (if (3 as c_int) < 4 as c_int {
                    0x80088822 as c_uint
                        >> ((3 as c_int - 3 as c_int)
                            * 8 as c_int
                            + (7 as c_int - 0 as c_int)
                            & 0x1f as c_int)
                } else {
                    0xaa55ff00 as c_uint
                        >> ((7 as c_int - 3 as c_int)
                            * 8 as c_int
                            + (7 as c_int - 0 as c_int)
                            & 0x1f as c_int)
                }) & 1 as c_uint
                    != 0
                {
                    ((1 as c_uint)
                        << (2 as c_int & 0x1f as c_int))
                        .wrapping_sub(1 as c_uint)
                        << ((0 as c_int * 2 as c_int
                            ^ (if 0 as c_int != 0 {
                                8 as c_int - 2 as c_int
                            } else {
                                0 as c_int
                            }))
                            & 0x1f as c_int)
                } else {
                    0 as c_uint
                })
                .wrapping_add(
                    (if (if (3 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 1 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 1 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (2 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((1 as c_int * 2 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 2 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (3 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 2 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 2 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (2 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((2 as c_int * 2 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 2 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (3 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 3 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 3 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (2 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((3 as c_int * 2 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 2 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (3 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 4 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 4 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (2 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((4 as c_int * 2 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 2 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (3 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 5 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 5 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (2 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((5 as c_int * 2 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 2 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (3 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 6 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 6 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (2 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((6 as c_int * 2 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 2 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (3 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 7 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 7 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (2 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((7 as c_int * 2 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 2 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_mul(
                    (if 2 as c_int == 1 as c_int {
                        0x1010101 as c_int
                    } else {
                        (if 2 as c_int == 2 as c_int {
                            0x10001 as c_int
                        } else {
                            1 as c_int
                        })
                    }) as c_uint,
                ),
                (if (if (4 as c_int) < 4 as c_int {
                    0x80088822 as c_uint
                        >> ((3 as c_int - 4 as c_int)
                            * 8 as c_int
                            + (7 as c_int - 0 as c_int)
                            & 0x1f as c_int)
                } else {
                    0xaa55ff00 as c_uint
                        >> ((7 as c_int - 4 as c_int)
                            * 8 as c_int
                            + (7 as c_int - 0 as c_int)
                            & 0x1f as c_int)
                }) & 1 as c_uint
                    != 0
                {
                    ((1 as c_uint)
                        << (2 as c_int & 0x1f as c_int))
                        .wrapping_sub(1 as c_uint)
                        << ((0 as c_int * 2 as c_int
                            ^ (if 0 as c_int != 0 {
                                8 as c_int - 2 as c_int
                            } else {
                                0 as c_int
                            }))
                            & 0x1f as c_int)
                } else {
                    0 as c_uint
                })
                .wrapping_add(
                    (if (if (4 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 4 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 1 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 4 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 1 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (2 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((1 as c_int * 2 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 2 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (4 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 4 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 2 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 4 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 2 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (2 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((2 as c_int * 2 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 2 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (4 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 4 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 3 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 4 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 3 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (2 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((3 as c_int * 2 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 2 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (4 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 4 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 4 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 4 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 4 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (2 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((4 as c_int * 2 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 2 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (4 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 4 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 5 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 4 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 5 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (2 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((5 as c_int * 2 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 2 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (4 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 4 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 6 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 4 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 6 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (2 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((6 as c_int * 2 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 2 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (4 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 4 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 7 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 4 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 7 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (2 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((7 as c_int * 2 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 2 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_mul(
                    (if 2 as c_int == 1 as c_int {
                        0x1010101 as c_int
                    } else {
                        (if 2 as c_int == 2 as c_int {
                            0x10001 as c_int
                        } else {
                            1 as c_int
                        })
                    }) as c_uint,
                ),
                (if (if (5 as c_int) < 4 as c_int {
                    0x80088822 as c_uint
                        >> ((3 as c_int - 5 as c_int)
                            * 8 as c_int
                            + (7 as c_int - 0 as c_int)
                            & 0x1f as c_int)
                } else {
                    0xaa55ff00 as c_uint
                        >> ((7 as c_int - 5 as c_int)
                            * 8 as c_int
                            + (7 as c_int - 0 as c_int)
                            & 0x1f as c_int)
                }) & 1 as c_uint
                    != 0
                {
                    ((1 as c_uint)
                        << (2 as c_int & 0x1f as c_int))
                        .wrapping_sub(1 as c_uint)
                        << ((0 as c_int * 2 as c_int
                            ^ (if 0 as c_int != 0 {
                                8 as c_int - 2 as c_int
                            } else {
                                0 as c_int
                            }))
                            & 0x1f as c_int)
                } else {
                    0 as c_uint
                })
                .wrapping_add(
                    (if (if (5 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 1 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 1 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (2 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((1 as c_int * 2 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 2 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (5 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 2 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 2 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (2 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((2 as c_int * 2 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 2 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (5 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 3 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 3 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (2 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((3 as c_int * 2 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 2 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (5 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 4 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 4 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (2 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((4 as c_int * 2 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 2 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (5 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 5 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 5 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (2 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((5 as c_int * 2 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 2 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (5 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 6 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 6 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (2 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((6 as c_int * 2 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 2 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (5 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 7 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 7 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (2 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((7 as c_int * 2 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 2 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_mul(
                    (if 2 as c_int == 1 as c_int {
                        0x1010101 as c_int
                    } else {
                        (if 2 as c_int == 2 as c_int {
                            0x10001 as c_int
                        } else {
                            1 as c_int
                        })
                    }) as c_uint,
                ),
            ],
            [
                (if (if (0 as c_int) < 4 as c_int {
                    0x80088822 as c_uint
                        >> ((3 as c_int - 0 as c_int)
                            * 8 as c_int
                            + (7 as c_int - 0 as c_int)
                            & 0x1f as c_int)
                } else {
                    0xaa55ff00 as c_uint
                        >> ((7 as c_int - 0 as c_int)
                            * 8 as c_int
                            + (7 as c_int - 0 as c_int)
                            & 0x1f as c_int)
                }) & 1 as c_uint
                    != 0
                {
                    ((1 as c_uint)
                        << (4 as c_int & 0x1f as c_int))
                        .wrapping_sub(1 as c_uint)
                        << ((0 as c_int * 4 as c_int
                            ^ (if 0 as c_int != 0 {
                                8 as c_int - 4 as c_int
                            } else {
                                0 as c_int
                            }))
                            & 0x1f as c_int)
                } else {
                    0 as c_uint
                })
                .wrapping_add(
                    (if (if (0 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 0 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 1 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 0 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 1 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (4 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((1 as c_int * 4 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 4 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (0 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 0 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 2 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 0 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 2 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (4 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((2 as c_int * 4 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 4 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (0 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 0 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 3 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 0 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 3 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (4 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((3 as c_int * 4 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 4 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (0 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 0 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 4 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 0 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 4 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (4 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((4 as c_int * 4 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 4 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (0 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 0 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 5 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 0 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 5 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (4 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((5 as c_int * 4 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 4 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (0 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 0 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 6 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 0 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 6 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (4 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((6 as c_int * 4 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 4 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (0 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 0 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 7 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 0 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 7 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (4 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((7 as c_int * 4 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 4 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_mul(
                    (if 4 as c_int == 1 as c_int {
                        0x1010101 as c_int
                    } else {
                        (if 4 as c_int == 2 as c_int {
                            0x10001 as c_int
                        } else {
                            1 as c_int
                        })
                    }) as c_uint,
                ),
                (if (if (1 as c_int) < 4 as c_int {
                    0x80088822 as c_uint
                        >> ((3 as c_int - 1 as c_int)
                            * 8 as c_int
                            + (7 as c_int - 0 as c_int)
                            & 0x1f as c_int)
                } else {
                    0xaa55ff00 as c_uint
                        >> ((7 as c_int - 1 as c_int)
                            * 8 as c_int
                            + (7 as c_int - 0 as c_int)
                            & 0x1f as c_int)
                }) & 1 as c_uint
                    != 0
                {
                    ((1 as c_uint)
                        << (4 as c_int & 0x1f as c_int))
                        .wrapping_sub(1 as c_uint)
                        << ((0 as c_int * 4 as c_int
                            ^ (if 0 as c_int != 0 {
                                8 as c_int - 4 as c_int
                            } else {
                                0 as c_int
                            }))
                            & 0x1f as c_int)
                } else {
                    0 as c_uint
                })
                .wrapping_add(
                    (if (if (1 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 1 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 1 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (4 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((1 as c_int * 4 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 4 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (1 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 2 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 2 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (4 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((2 as c_int * 4 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 4 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (1 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 3 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 3 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (4 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((3 as c_int * 4 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 4 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (1 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 4 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 4 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (4 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((4 as c_int * 4 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 4 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (1 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 5 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 5 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (4 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((5 as c_int * 4 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 4 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (1 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 6 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 6 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (4 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((6 as c_int * 4 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 4 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (1 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 7 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 7 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (4 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((7 as c_int * 4 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 4 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_mul(
                    (if 4 as c_int == 1 as c_int {
                        0x1010101 as c_int
                    } else {
                        (if 4 as c_int == 2 as c_int {
                            0x10001 as c_int
                        } else {
                            1 as c_int
                        })
                    }) as c_uint,
                ),
                (if (if (2 as c_int) < 4 as c_int {
                    0x80088822 as c_uint
                        >> ((3 as c_int - 2 as c_int)
                            * 8 as c_int
                            + (7 as c_int - 0 as c_int)
                            & 0x1f as c_int)
                } else {
                    0xaa55ff00 as c_uint
                        >> ((7 as c_int - 2 as c_int)
                            * 8 as c_int
                            + (7 as c_int - 0 as c_int)
                            & 0x1f as c_int)
                }) & 1 as c_uint
                    != 0
                {
                    ((1 as c_uint)
                        << (4 as c_int & 0x1f as c_int))
                        .wrapping_sub(1 as c_uint)
                        << ((0 as c_int * 4 as c_int
                            ^ (if 0 as c_int != 0 {
                                8 as c_int - 4 as c_int
                            } else {
                                0 as c_int
                            }))
                            & 0x1f as c_int)
                } else {
                    0 as c_uint
                })
                .wrapping_add(
                    (if (if (2 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 2 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 1 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 2 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 1 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (4 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((1 as c_int * 4 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 4 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (2 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 2 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 2 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 2 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 2 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (4 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((2 as c_int * 4 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 4 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (2 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 2 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 3 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 2 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 3 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (4 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((3 as c_int * 4 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 4 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (2 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 2 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 4 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 2 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 4 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (4 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((4 as c_int * 4 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 4 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (2 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 2 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 5 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 2 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 5 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (4 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((5 as c_int * 4 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 4 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (2 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 2 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 6 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 2 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 6 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (4 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((6 as c_int * 4 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 4 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (2 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 2 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 7 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 2 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 7 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (4 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((7 as c_int * 4 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 4 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_mul(
                    (if 4 as c_int == 1 as c_int {
                        0x1010101 as c_int
                    } else {
                        (if 4 as c_int == 2 as c_int {
                            0x10001 as c_int
                        } else {
                            1 as c_int
                        })
                    }) as c_uint,
                ),
                (if (if (3 as c_int) < 4 as c_int {
                    0x80088822 as c_uint
                        >> ((3 as c_int - 3 as c_int)
                            * 8 as c_int
                            + (7 as c_int - 0 as c_int)
                            & 0x1f as c_int)
                } else {
                    0xaa55ff00 as c_uint
                        >> ((7 as c_int - 3 as c_int)
                            * 8 as c_int
                            + (7 as c_int - 0 as c_int)
                            & 0x1f as c_int)
                }) & 1 as c_uint
                    != 0
                {
                    ((1 as c_uint)
                        << (4 as c_int & 0x1f as c_int))
                        .wrapping_sub(1 as c_uint)
                        << ((0 as c_int * 4 as c_int
                            ^ (if 0 as c_int != 0 {
                                8 as c_int - 4 as c_int
                            } else {
                                0 as c_int
                            }))
                            & 0x1f as c_int)
                } else {
                    0 as c_uint
                })
                .wrapping_add(
                    (if (if (3 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 1 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 1 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (4 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((1 as c_int * 4 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 4 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (3 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 2 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 2 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (4 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((2 as c_int * 4 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 4 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (3 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 3 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 3 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (4 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((3 as c_int * 4 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 4 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (3 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 4 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 4 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (4 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((4 as c_int * 4 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 4 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (3 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 5 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 5 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (4 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((5 as c_int * 4 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 4 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (3 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 6 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 6 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (4 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((6 as c_int * 4 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 4 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (3 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 7 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 7 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (4 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((7 as c_int * 4 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 4 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_mul(
                    (if 4 as c_int == 1 as c_int {
                        0x1010101 as c_int
                    } else {
                        (if 4 as c_int == 2 as c_int {
                            0x10001 as c_int
                        } else {
                            1 as c_int
                        })
                    }) as c_uint,
                ),
                (if (if (4 as c_int) < 4 as c_int {
                    0x80088822 as c_uint
                        >> ((3 as c_int - 4 as c_int)
                            * 8 as c_int
                            + (7 as c_int - 0 as c_int)
                            & 0x1f as c_int)
                } else {
                    0xaa55ff00 as c_uint
                        >> ((7 as c_int - 4 as c_int)
                            * 8 as c_int
                            + (7 as c_int - 0 as c_int)
                            & 0x1f as c_int)
                }) & 1 as c_uint
                    != 0
                {
                    ((1 as c_uint)
                        << (4 as c_int & 0x1f as c_int))
                        .wrapping_sub(1 as c_uint)
                        << ((0 as c_int * 4 as c_int
                            ^ (if 0 as c_int != 0 {
                                8 as c_int - 4 as c_int
                            } else {
                                0 as c_int
                            }))
                            & 0x1f as c_int)
                } else {
                    0 as c_uint
                })
                .wrapping_add(
                    (if (if (4 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 4 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 1 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 4 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 1 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (4 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((1 as c_int * 4 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 4 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (4 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 4 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 2 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 4 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 2 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (4 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((2 as c_int * 4 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 4 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (4 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 4 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 3 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 4 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 3 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (4 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((3 as c_int * 4 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 4 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (4 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 4 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 4 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 4 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 4 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (4 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((4 as c_int * 4 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 4 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (4 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 4 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 5 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 4 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 5 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (4 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((5 as c_int * 4 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 4 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (4 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 4 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 6 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 4 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 6 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (4 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((6 as c_int * 4 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 4 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (4 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 4 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 7 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 4 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 7 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (4 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((7 as c_int * 4 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 4 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_mul(
                    (if 4 as c_int == 1 as c_int {
                        0x1010101 as c_int
                    } else {
                        (if 4 as c_int == 2 as c_int {
                            0x10001 as c_int
                        } else {
                            1 as c_int
                        })
                    }) as c_uint,
                ),
                (if (if (5 as c_int) < 4 as c_int {
                    0x80088822 as c_uint
                        >> ((3 as c_int - 5 as c_int)
                            * 8 as c_int
                            + (7 as c_int - 0 as c_int)
                            & 0x1f as c_int)
                } else {
                    0xaa55ff00 as c_uint
                        >> ((7 as c_int - 5 as c_int)
                            * 8 as c_int
                            + (7 as c_int - 0 as c_int)
                            & 0x1f as c_int)
                }) & 1 as c_uint
                    != 0
                {
                    ((1 as c_uint)
                        << (4 as c_int & 0x1f as c_int))
                        .wrapping_sub(1 as c_uint)
                        << ((0 as c_int * 4 as c_int
                            ^ (if 0 as c_int != 0 {
                                8 as c_int - 4 as c_int
                            } else {
                                0 as c_int
                            }))
                            & 0x1f as c_int)
                } else {
                    0 as c_uint
                })
                .wrapping_add(
                    (if (if (5 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 1 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 1 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (4 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((1 as c_int * 4 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 4 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (5 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 2 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 2 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (4 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((2 as c_int * 4 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 4 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (5 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 3 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 3 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (4 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((3 as c_int * 4 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 4 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (5 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 4 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 4 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (4 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((4 as c_int * 4 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 4 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (5 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 5 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 5 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (4 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((5 as c_int * 4 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 4 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (5 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 6 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 6 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (4 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((6 as c_int * 4 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 4 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (5 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 7 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 7 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (4 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((7 as c_int * 4 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 4 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_mul(
                    (if 4 as c_int == 1 as c_int {
                        0x1010101 as c_int
                    } else {
                        (if 4 as c_int == 2 as c_int {
                            0x10001 as c_int
                        } else {
                            1 as c_int
                        })
                    }) as c_uint,
                ),
            ],
        ],
        [
            [
                (if (if (0 as c_int) < 4 as c_int {
                    0x80088822 as c_uint
                        >> ((3 as c_int - 0 as c_int)
                            * 8 as c_int
                            + (7 as c_int - 0 as c_int)
                            & 0x1f as c_int)
                } else {
                    0xaa55ff00 as c_uint
                        >> ((7 as c_int - 0 as c_int)
                            * 8 as c_int
                            + (7 as c_int - 0 as c_int)
                            & 0x1f as c_int)
                }) & 1 as c_uint
                    != 0
                {
                    ((1 as c_uint)
                        << (1 as c_int & 0x1f as c_int))
                        .wrapping_sub(1 as c_uint)
                        << ((0 as c_int * 1 as c_int
                            ^ (if 1 as c_int != 0 {
                                8 as c_int - 1 as c_int
                            } else {
                                0 as c_int
                            }))
                            & 0x1f as c_int)
                } else {
                    0 as c_uint
                })
                .wrapping_add(
                    (if (if (0 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 0 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 1 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 0 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 1 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (1 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((1 as c_int * 1 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 1 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (0 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 0 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 2 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 0 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 2 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (1 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((2 as c_int * 1 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 1 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (0 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 0 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 3 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 0 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 3 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (1 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((3 as c_int * 1 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 1 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (0 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 0 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 4 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 0 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 4 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (1 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((4 as c_int * 1 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 1 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (0 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 0 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 5 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 0 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 5 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (1 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((5 as c_int * 1 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 1 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (0 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 0 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 6 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 0 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 6 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (1 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((6 as c_int * 1 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 1 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (0 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 0 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 7 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 0 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 7 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (1 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((7 as c_int * 1 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 1 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_mul(
                    (if 1 as c_int == 1 as c_int {
                        0x1010101 as c_int
                    } else {
                        (if 1 as c_int == 2 as c_int {
                            0x10001 as c_int
                        } else {
                            1 as c_int
                        })
                    }) as c_uint,
                ),
                (if (if (1 as c_int) < 4 as c_int {
                    0x80088822 as c_uint
                        >> ((3 as c_int - 1 as c_int)
                            * 8 as c_int
                            + (7 as c_int - 0 as c_int)
                            & 0x1f as c_int)
                } else {
                    0xaa55ff00 as c_uint
                        >> ((7 as c_int - 1 as c_int)
                            * 8 as c_int
                            + (7 as c_int - 0 as c_int)
                            & 0x1f as c_int)
                }) & 1 as c_uint
                    != 0
                {
                    ((1 as c_uint)
                        << (1 as c_int & 0x1f as c_int))
                        .wrapping_sub(1 as c_uint)
                        << ((0 as c_int * 1 as c_int
                            ^ (if 1 as c_int != 0 {
                                8 as c_int - 1 as c_int
                            } else {
                                0 as c_int
                            }))
                            & 0x1f as c_int)
                } else {
                    0 as c_uint
                })
                .wrapping_add(
                    (if (if (1 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 1 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 1 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (1 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((1 as c_int * 1 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 1 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (1 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 2 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 2 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (1 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((2 as c_int * 1 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 1 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (1 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 3 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 3 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (1 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((3 as c_int * 1 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 1 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (1 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 4 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 4 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (1 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((4 as c_int * 1 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 1 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (1 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 5 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 5 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (1 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((5 as c_int * 1 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 1 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (1 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 6 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 6 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (1 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((6 as c_int * 1 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 1 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (1 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 7 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 7 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (1 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((7 as c_int * 1 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 1 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_mul(
                    (if 1 as c_int == 1 as c_int {
                        0x1010101 as c_int
                    } else {
                        (if 1 as c_int == 2 as c_int {
                            0x10001 as c_int
                        } else {
                            1 as c_int
                        })
                    }) as c_uint,
                ),
                (if (if (2 as c_int) < 4 as c_int {
                    0x80088822 as c_uint
                        >> ((3 as c_int - 2 as c_int)
                            * 8 as c_int
                            + (7 as c_int - 0 as c_int)
                            & 0x1f as c_int)
                } else {
                    0xaa55ff00 as c_uint
                        >> ((7 as c_int - 2 as c_int)
                            * 8 as c_int
                            + (7 as c_int - 0 as c_int)
                            & 0x1f as c_int)
                }) & 1 as c_uint
                    != 0
                {
                    ((1 as c_uint)
                        << (1 as c_int & 0x1f as c_int))
                        .wrapping_sub(1 as c_uint)
                        << ((0 as c_int * 1 as c_int
                            ^ (if 1 as c_int != 0 {
                                8 as c_int - 1 as c_int
                            } else {
                                0 as c_int
                            }))
                            & 0x1f as c_int)
                } else {
                    0 as c_uint
                })
                .wrapping_add(
                    (if (if (2 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 2 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 1 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 2 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 1 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (1 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((1 as c_int * 1 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 1 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (2 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 2 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 2 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 2 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 2 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (1 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((2 as c_int * 1 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 1 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (2 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 2 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 3 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 2 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 3 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (1 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((3 as c_int * 1 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 1 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (2 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 2 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 4 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 2 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 4 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (1 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((4 as c_int * 1 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 1 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (2 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 2 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 5 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 2 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 5 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (1 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((5 as c_int * 1 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 1 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (2 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 2 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 6 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 2 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 6 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (1 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((6 as c_int * 1 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 1 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (2 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 2 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 7 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 2 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 7 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (1 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((7 as c_int * 1 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 1 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_mul(
                    (if 1 as c_int == 1 as c_int {
                        0x1010101 as c_int
                    } else {
                        (if 1 as c_int == 2 as c_int {
                            0x10001 as c_int
                        } else {
                            1 as c_int
                        })
                    }) as c_uint,
                ),
                (if (if (3 as c_int) < 4 as c_int {
                    0x80088822 as c_uint
                        >> ((3 as c_int - 3 as c_int)
                            * 8 as c_int
                            + (7 as c_int - 0 as c_int)
                            & 0x1f as c_int)
                } else {
                    0xaa55ff00 as c_uint
                        >> ((7 as c_int - 3 as c_int)
                            * 8 as c_int
                            + (7 as c_int - 0 as c_int)
                            & 0x1f as c_int)
                }) & 1 as c_uint
                    != 0
                {
                    ((1 as c_uint)
                        << (1 as c_int & 0x1f as c_int))
                        .wrapping_sub(1 as c_uint)
                        << ((0 as c_int * 1 as c_int
                            ^ (if 1 as c_int != 0 {
                                8 as c_int - 1 as c_int
                            } else {
                                0 as c_int
                            }))
                            & 0x1f as c_int)
                } else {
                    0 as c_uint
                })
                .wrapping_add(
                    (if (if (3 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 1 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 1 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (1 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((1 as c_int * 1 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 1 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (3 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 2 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 2 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (1 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((2 as c_int * 1 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 1 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (3 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 3 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 3 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (1 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((3 as c_int * 1 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 1 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (3 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 4 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 4 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (1 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((4 as c_int * 1 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 1 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (3 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 5 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 5 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (1 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((5 as c_int * 1 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 1 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (3 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 6 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 6 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (1 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((6 as c_int * 1 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 1 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (3 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 7 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 7 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (1 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((7 as c_int * 1 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 1 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_mul(
                    (if 1 as c_int == 1 as c_int {
                        0x1010101 as c_int
                    } else {
                        (if 1 as c_int == 2 as c_int {
                            0x10001 as c_int
                        } else {
                            1 as c_int
                        })
                    }) as c_uint,
                ),
                (if (if (4 as c_int) < 4 as c_int {
                    0x80088822 as c_uint
                        >> ((3 as c_int - 4 as c_int)
                            * 8 as c_int
                            + (7 as c_int - 0 as c_int)
                            & 0x1f as c_int)
                } else {
                    0xaa55ff00 as c_uint
                        >> ((7 as c_int - 4 as c_int)
                            * 8 as c_int
                            + (7 as c_int - 0 as c_int)
                            & 0x1f as c_int)
                }) & 1 as c_uint
                    != 0
                {
                    ((1 as c_uint)
                        << (1 as c_int & 0x1f as c_int))
                        .wrapping_sub(1 as c_uint)
                        << ((0 as c_int * 1 as c_int
                            ^ (if 1 as c_int != 0 {
                                8 as c_int - 1 as c_int
                            } else {
                                0 as c_int
                            }))
                            & 0x1f as c_int)
                } else {
                    0 as c_uint
                })
                .wrapping_add(
                    (if (if (4 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 4 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 1 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 4 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 1 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (1 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((1 as c_int * 1 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 1 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (4 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 4 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 2 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 4 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 2 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (1 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((2 as c_int * 1 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 1 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (4 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 4 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 3 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 4 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 3 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (1 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((3 as c_int * 1 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 1 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (4 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 4 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 4 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 4 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 4 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (1 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((4 as c_int * 1 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 1 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (4 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 4 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 5 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 4 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 5 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (1 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((5 as c_int * 1 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 1 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (4 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 4 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 6 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 4 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 6 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (1 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((6 as c_int * 1 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 1 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (4 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 4 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 7 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 4 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 7 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (1 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((7 as c_int * 1 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 1 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_mul(
                    (if 1 as c_int == 1 as c_int {
                        0x1010101 as c_int
                    } else {
                        (if 1 as c_int == 2 as c_int {
                            0x10001 as c_int
                        } else {
                            1 as c_int
                        })
                    }) as c_uint,
                ),
                (if (if (5 as c_int) < 4 as c_int {
                    0x80088822 as c_uint
                        >> ((3 as c_int - 5 as c_int)
                            * 8 as c_int
                            + (7 as c_int - 0 as c_int)
                            & 0x1f as c_int)
                } else {
                    0xaa55ff00 as c_uint
                        >> ((7 as c_int - 5 as c_int)
                            * 8 as c_int
                            + (7 as c_int - 0 as c_int)
                            & 0x1f as c_int)
                }) & 1 as c_uint
                    != 0
                {
                    ((1 as c_uint)
                        << (1 as c_int & 0x1f as c_int))
                        .wrapping_sub(1 as c_uint)
                        << ((0 as c_int * 1 as c_int
                            ^ (if 1 as c_int != 0 {
                                8 as c_int - 1 as c_int
                            } else {
                                0 as c_int
                            }))
                            & 0x1f as c_int)
                } else {
                    0 as c_uint
                })
                .wrapping_add(
                    (if (if (5 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 1 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 1 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (1 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((1 as c_int * 1 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 1 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (5 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 2 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 2 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (1 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((2 as c_int * 1 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 1 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (5 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 3 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 3 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (1 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((3 as c_int * 1 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 1 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (5 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 4 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 4 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (1 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((4 as c_int * 1 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 1 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (5 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 5 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 5 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (1 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((5 as c_int * 1 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 1 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (5 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 6 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 6 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (1 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((6 as c_int * 1 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 1 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (5 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 7 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 7 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (1 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((7 as c_int * 1 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 1 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_mul(
                    (if 1 as c_int == 1 as c_int {
                        0x1010101 as c_int
                    } else {
                        (if 1 as c_int == 2 as c_int {
                            0x10001 as c_int
                        } else {
                            1 as c_int
                        })
                    }) as c_uint,
                ),
            ],
            [
                (if (if (0 as c_int) < 4 as c_int {
                    0x80088822 as c_uint
                        >> ((3 as c_int - 0 as c_int)
                            * 8 as c_int
                            + (7 as c_int - 0 as c_int)
                            & 0x1f as c_int)
                } else {
                    0xaa55ff00 as c_uint
                        >> ((7 as c_int - 0 as c_int)
                            * 8 as c_int
                            + (7 as c_int - 0 as c_int)
                            & 0x1f as c_int)
                }) & 1 as c_uint
                    != 0
                {
                    ((1 as c_uint)
                        << (2 as c_int & 0x1f as c_int))
                        .wrapping_sub(1 as c_uint)
                        << ((0 as c_int * 2 as c_int
                            ^ (if 1 as c_int != 0 {
                                8 as c_int - 2 as c_int
                            } else {
                                0 as c_int
                            }))
                            & 0x1f as c_int)
                } else {
                    0 as c_uint
                })
                .wrapping_add(
                    (if (if (0 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 0 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 1 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 0 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 1 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (2 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((1 as c_int * 2 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 2 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (0 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 0 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 2 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 0 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 2 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (2 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((2 as c_int * 2 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 2 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (0 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 0 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 3 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 0 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 3 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (2 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((3 as c_int * 2 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 2 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (0 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 0 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 4 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 0 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 4 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (2 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((4 as c_int * 2 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 2 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (0 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 0 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 5 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 0 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 5 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (2 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((5 as c_int * 2 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 2 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (0 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 0 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 6 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 0 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 6 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (2 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((6 as c_int * 2 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 2 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (0 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 0 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 7 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 0 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 7 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (2 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((7 as c_int * 2 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 2 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_mul(
                    (if 2 as c_int == 1 as c_int {
                        0x1010101 as c_int
                    } else {
                        (if 2 as c_int == 2 as c_int {
                            0x10001 as c_int
                        } else {
                            1 as c_int
                        })
                    }) as c_uint,
                ),
                (if (if (1 as c_int) < 4 as c_int {
                    0x80088822 as c_uint
                        >> ((3 as c_int - 1 as c_int)
                            * 8 as c_int
                            + (7 as c_int - 0 as c_int)
                            & 0x1f as c_int)
                } else {
                    0xaa55ff00 as c_uint
                        >> ((7 as c_int - 1 as c_int)
                            * 8 as c_int
                            + (7 as c_int - 0 as c_int)
                            & 0x1f as c_int)
                }) & 1 as c_uint
                    != 0
                {
                    ((1 as c_uint)
                        << (2 as c_int & 0x1f as c_int))
                        .wrapping_sub(1 as c_uint)
                        << ((0 as c_int * 2 as c_int
                            ^ (if 1 as c_int != 0 {
                                8 as c_int - 2 as c_int
                            } else {
                                0 as c_int
                            }))
                            & 0x1f as c_int)
                } else {
                    0 as c_uint
                })
                .wrapping_add(
                    (if (if (1 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 1 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 1 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (2 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((1 as c_int * 2 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 2 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (1 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 2 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 2 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (2 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((2 as c_int * 2 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 2 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (1 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 3 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 3 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (2 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((3 as c_int * 2 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 2 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (1 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 4 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 4 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (2 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((4 as c_int * 2 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 2 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (1 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 5 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 5 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (2 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((5 as c_int * 2 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 2 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (1 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 6 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 6 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (2 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((6 as c_int * 2 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 2 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (1 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 7 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 7 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (2 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((7 as c_int * 2 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 2 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_mul(
                    (if 2 as c_int == 1 as c_int {
                        0x1010101 as c_int
                    } else {
                        (if 2 as c_int == 2 as c_int {
                            0x10001 as c_int
                        } else {
                            1 as c_int
                        })
                    }) as c_uint,
                ),
                (if (if (2 as c_int) < 4 as c_int {
                    0x80088822 as c_uint
                        >> ((3 as c_int - 2 as c_int)
                            * 8 as c_int
                            + (7 as c_int - 0 as c_int)
                            & 0x1f as c_int)
                } else {
                    0xaa55ff00 as c_uint
                        >> ((7 as c_int - 2 as c_int)
                            * 8 as c_int
                            + (7 as c_int - 0 as c_int)
                            & 0x1f as c_int)
                }) & 1 as c_uint
                    != 0
                {
                    ((1 as c_uint)
                        << (2 as c_int & 0x1f as c_int))
                        .wrapping_sub(1 as c_uint)
                        << ((0 as c_int * 2 as c_int
                            ^ (if 1 as c_int != 0 {
                                8 as c_int - 2 as c_int
                            } else {
                                0 as c_int
                            }))
                            & 0x1f as c_int)
                } else {
                    0 as c_uint
                })
                .wrapping_add(
                    (if (if (2 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 2 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 1 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 2 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 1 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (2 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((1 as c_int * 2 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 2 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (2 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 2 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 2 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 2 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 2 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (2 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((2 as c_int * 2 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 2 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (2 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 2 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 3 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 2 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 3 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (2 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((3 as c_int * 2 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 2 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (2 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 2 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 4 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 2 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 4 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (2 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((4 as c_int * 2 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 2 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (2 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 2 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 5 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 2 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 5 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (2 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((5 as c_int * 2 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 2 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (2 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 2 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 6 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 2 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 6 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (2 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((6 as c_int * 2 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 2 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (2 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 2 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 7 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 2 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 7 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (2 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((7 as c_int * 2 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 2 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_mul(
                    (if 2 as c_int == 1 as c_int {
                        0x1010101 as c_int
                    } else {
                        (if 2 as c_int == 2 as c_int {
                            0x10001 as c_int
                        } else {
                            1 as c_int
                        })
                    }) as c_uint,
                ),
                (if (if (3 as c_int) < 4 as c_int {
                    0x80088822 as c_uint
                        >> ((3 as c_int - 3 as c_int)
                            * 8 as c_int
                            + (7 as c_int - 0 as c_int)
                            & 0x1f as c_int)
                } else {
                    0xaa55ff00 as c_uint
                        >> ((7 as c_int - 3 as c_int)
                            * 8 as c_int
                            + (7 as c_int - 0 as c_int)
                            & 0x1f as c_int)
                }) & 1 as c_uint
                    != 0
                {
                    ((1 as c_uint)
                        << (2 as c_int & 0x1f as c_int))
                        .wrapping_sub(1 as c_uint)
                        << ((0 as c_int * 2 as c_int
                            ^ (if 1 as c_int != 0 {
                                8 as c_int - 2 as c_int
                            } else {
                                0 as c_int
                            }))
                            & 0x1f as c_int)
                } else {
                    0 as c_uint
                })
                .wrapping_add(
                    (if (if (3 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 1 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 1 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (2 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((1 as c_int * 2 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 2 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (3 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 2 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 2 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (2 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((2 as c_int * 2 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 2 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (3 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 3 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 3 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (2 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((3 as c_int * 2 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 2 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (3 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 4 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 4 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (2 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((4 as c_int * 2 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 2 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (3 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 5 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 5 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (2 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((5 as c_int * 2 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 2 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (3 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 6 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 6 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (2 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((6 as c_int * 2 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 2 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (3 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 7 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 7 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (2 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((7 as c_int * 2 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 2 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_mul(
                    (if 2 as c_int == 1 as c_int {
                        0x1010101 as c_int
                    } else {
                        (if 2 as c_int == 2 as c_int {
                            0x10001 as c_int
                        } else {
                            1 as c_int
                        })
                    }) as c_uint,
                ),
                (if (if (4 as c_int) < 4 as c_int {
                    0x80088822 as c_uint
                        >> ((3 as c_int - 4 as c_int)
                            * 8 as c_int
                            + (7 as c_int - 0 as c_int)
                            & 0x1f as c_int)
                } else {
                    0xaa55ff00 as c_uint
                        >> ((7 as c_int - 4 as c_int)
                            * 8 as c_int
                            + (7 as c_int - 0 as c_int)
                            & 0x1f as c_int)
                }) & 1 as c_uint
                    != 0
                {
                    ((1 as c_uint)
                        << (2 as c_int & 0x1f as c_int))
                        .wrapping_sub(1 as c_uint)
                        << ((0 as c_int * 2 as c_int
                            ^ (if 1 as c_int != 0 {
                                8 as c_int - 2 as c_int
                            } else {
                                0 as c_int
                            }))
                            & 0x1f as c_int)
                } else {
                    0 as c_uint
                })
                .wrapping_add(
                    (if (if (4 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 4 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 1 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 4 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 1 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (2 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((1 as c_int * 2 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 2 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (4 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 4 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 2 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 4 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 2 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (2 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((2 as c_int * 2 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 2 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (4 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 4 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 3 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 4 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 3 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (2 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((3 as c_int * 2 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 2 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (4 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 4 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 4 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 4 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 4 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (2 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((4 as c_int * 2 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 2 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (4 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 4 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 5 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 4 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 5 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (2 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((5 as c_int * 2 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 2 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (4 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 4 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 6 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 4 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 6 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (2 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((6 as c_int * 2 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 2 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (4 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 4 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 7 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 4 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 7 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (2 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((7 as c_int * 2 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 2 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_mul(
                    (if 2 as c_int == 1 as c_int {
                        0x1010101 as c_int
                    } else {
                        (if 2 as c_int == 2 as c_int {
                            0x10001 as c_int
                        } else {
                            1 as c_int
                        })
                    }) as c_uint,
                ),
                (if (if (5 as c_int) < 4 as c_int {
                    0x80088822 as c_uint
                        >> ((3 as c_int - 5 as c_int)
                            * 8 as c_int
                            + (7 as c_int - 0 as c_int)
                            & 0x1f as c_int)
                } else {
                    0xaa55ff00 as c_uint
                        >> ((7 as c_int - 5 as c_int)
                            * 8 as c_int
                            + (7 as c_int - 0 as c_int)
                            & 0x1f as c_int)
                }) & 1 as c_uint
                    != 0
                {
                    ((1 as c_uint)
                        << (2 as c_int & 0x1f as c_int))
                        .wrapping_sub(1 as c_uint)
                        << ((0 as c_int * 2 as c_int
                            ^ (if 1 as c_int != 0 {
                                8 as c_int - 2 as c_int
                            } else {
                                0 as c_int
                            }))
                            & 0x1f as c_int)
                } else {
                    0 as c_uint
                })
                .wrapping_add(
                    (if (if (5 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 1 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 1 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (2 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((1 as c_int * 2 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 2 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (5 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 2 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 2 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (2 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((2 as c_int * 2 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 2 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (5 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 3 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 3 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (2 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((3 as c_int * 2 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 2 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (5 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 4 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 4 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (2 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((4 as c_int * 2 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 2 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (5 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 5 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 5 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (2 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((5 as c_int * 2 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 2 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (5 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 6 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 6 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (2 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((6 as c_int * 2 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 2 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (5 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 7 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 7 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (2 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((7 as c_int * 2 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 2 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_mul(
                    (if 2 as c_int == 1 as c_int {
                        0x1010101 as c_int
                    } else {
                        (if 2 as c_int == 2 as c_int {
                            0x10001 as c_int
                        } else {
                            1 as c_int
                        })
                    }) as c_uint,
                ),
            ],
            [
                (if (if (0 as c_int) < 4 as c_int {
                    0x80088822 as c_uint
                        >> ((3 as c_int - 0 as c_int)
                            * 8 as c_int
                            + (7 as c_int - 0 as c_int)
                            & 0x1f as c_int)
                } else {
                    0xaa55ff00 as c_uint
                        >> ((7 as c_int - 0 as c_int)
                            * 8 as c_int
                            + (7 as c_int - 0 as c_int)
                            & 0x1f as c_int)
                }) & 1 as c_uint
                    != 0
                {
                    ((1 as c_uint)
                        << (4 as c_int & 0x1f as c_int))
                        .wrapping_sub(1 as c_uint)
                        << ((0 as c_int * 4 as c_int
                            ^ (if 1 as c_int != 0 {
                                8 as c_int - 4 as c_int
                            } else {
                                0 as c_int
                            }))
                            & 0x1f as c_int)
                } else {
                    0 as c_uint
                })
                .wrapping_add(
                    (if (if (0 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 0 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 1 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 0 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 1 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (4 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((1 as c_int * 4 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 4 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (0 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 0 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 2 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 0 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 2 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (4 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((2 as c_int * 4 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 4 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (0 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 0 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 3 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 0 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 3 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (4 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((3 as c_int * 4 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 4 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (0 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 0 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 4 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 0 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 4 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (4 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((4 as c_int * 4 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 4 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (0 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 0 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 5 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 0 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 5 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (4 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((5 as c_int * 4 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 4 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (0 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 0 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 6 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 0 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 6 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (4 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((6 as c_int * 4 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 4 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (0 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 0 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 7 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 0 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 7 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (4 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((7 as c_int * 4 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 4 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_mul(
                    (if 4 as c_int == 1 as c_int {
                        0x1010101 as c_int
                    } else {
                        (if 4 as c_int == 2 as c_int {
                            0x10001 as c_int
                        } else {
                            1 as c_int
                        })
                    }) as c_uint,
                ),
                (if (if (1 as c_int) < 4 as c_int {
                    0x80088822 as c_uint
                        >> ((3 as c_int - 1 as c_int)
                            * 8 as c_int
                            + (7 as c_int - 0 as c_int)
                            & 0x1f as c_int)
                } else {
                    0xaa55ff00 as c_uint
                        >> ((7 as c_int - 1 as c_int)
                            * 8 as c_int
                            + (7 as c_int - 0 as c_int)
                            & 0x1f as c_int)
                }) & 1 as c_uint
                    != 0
                {
                    ((1 as c_uint)
                        << (4 as c_int & 0x1f as c_int))
                        .wrapping_sub(1 as c_uint)
                        << ((0 as c_int * 4 as c_int
                            ^ (if 1 as c_int != 0 {
                                8 as c_int - 4 as c_int
                            } else {
                                0 as c_int
                            }))
                            & 0x1f as c_int)
                } else {
                    0 as c_uint
                })
                .wrapping_add(
                    (if (if (1 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 1 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 1 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (4 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((1 as c_int * 4 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 4 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (1 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 2 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 2 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (4 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((2 as c_int * 4 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 4 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (1 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 3 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 3 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (4 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((3 as c_int * 4 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 4 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (1 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 4 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 4 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (4 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((4 as c_int * 4 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 4 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (1 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 5 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 5 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (4 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((5 as c_int * 4 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 4 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (1 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 6 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 6 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (4 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((6 as c_int * 4 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 4 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (1 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 7 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 7 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (4 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((7 as c_int * 4 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 4 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_mul(
                    (if 4 as c_int == 1 as c_int {
                        0x1010101 as c_int
                    } else {
                        (if 4 as c_int == 2 as c_int {
                            0x10001 as c_int
                        } else {
                            1 as c_int
                        })
                    }) as c_uint,
                ),
                (if (if (2 as c_int) < 4 as c_int {
                    0x80088822 as c_uint
                        >> ((3 as c_int - 2 as c_int)
                            * 8 as c_int
                            + (7 as c_int - 0 as c_int)
                            & 0x1f as c_int)
                } else {
                    0xaa55ff00 as c_uint
                        >> ((7 as c_int - 2 as c_int)
                            * 8 as c_int
                            + (7 as c_int - 0 as c_int)
                            & 0x1f as c_int)
                }) & 1 as c_uint
                    != 0
                {
                    ((1 as c_uint)
                        << (4 as c_int & 0x1f as c_int))
                        .wrapping_sub(1 as c_uint)
                        << ((0 as c_int * 4 as c_int
                            ^ (if 1 as c_int != 0 {
                                8 as c_int - 4 as c_int
                            } else {
                                0 as c_int
                            }))
                            & 0x1f as c_int)
                } else {
                    0 as c_uint
                })
                .wrapping_add(
                    (if (if (2 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 2 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 1 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 2 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 1 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (4 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((1 as c_int * 4 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 4 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (2 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 2 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 2 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 2 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 2 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (4 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((2 as c_int * 4 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 4 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (2 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 2 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 3 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 2 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 3 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (4 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((3 as c_int * 4 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 4 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (2 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 2 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 4 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 2 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 4 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (4 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((4 as c_int * 4 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 4 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (2 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 2 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 5 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 2 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 5 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (4 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((5 as c_int * 4 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 4 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (2 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 2 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 6 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 2 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 6 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (4 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((6 as c_int * 4 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 4 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (2 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 2 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 7 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 2 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 7 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (4 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((7 as c_int * 4 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 4 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_mul(
                    (if 4 as c_int == 1 as c_int {
                        0x1010101 as c_int
                    } else {
                        (if 4 as c_int == 2 as c_int {
                            0x10001 as c_int
                        } else {
                            1 as c_int
                        })
                    }) as c_uint,
                ),
                (if (if (3 as c_int) < 4 as c_int {
                    0x80088822 as c_uint
                        >> ((3 as c_int - 3 as c_int)
                            * 8 as c_int
                            + (7 as c_int - 0 as c_int)
                            & 0x1f as c_int)
                } else {
                    0xaa55ff00 as c_uint
                        >> ((7 as c_int - 3 as c_int)
                            * 8 as c_int
                            + (7 as c_int - 0 as c_int)
                            & 0x1f as c_int)
                }) & 1 as c_uint
                    != 0
                {
                    ((1 as c_uint)
                        << (4 as c_int & 0x1f as c_int))
                        .wrapping_sub(1 as c_uint)
                        << ((0 as c_int * 4 as c_int
                            ^ (if 1 as c_int != 0 {
                                8 as c_int - 4 as c_int
                            } else {
                                0 as c_int
                            }))
                            & 0x1f as c_int)
                } else {
                    0 as c_uint
                })
                .wrapping_add(
                    (if (if (3 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 1 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 1 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (4 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((1 as c_int * 4 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 4 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (3 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 2 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 2 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (4 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((2 as c_int * 4 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 4 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (3 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 3 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 3 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (4 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((3 as c_int * 4 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 4 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (3 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 4 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 4 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (4 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((4 as c_int * 4 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 4 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (3 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 5 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 5 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (4 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((5 as c_int * 4 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 4 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (3 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 6 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 6 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (4 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((6 as c_int * 4 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 4 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (3 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 7 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 7 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (4 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((7 as c_int * 4 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 4 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_mul(
                    (if 4 as c_int == 1 as c_int {
                        0x1010101 as c_int
                    } else {
                        (if 4 as c_int == 2 as c_int {
                            0x10001 as c_int
                        } else {
                            1 as c_int
                        })
                    }) as c_uint,
                ),
                (if (if (4 as c_int) < 4 as c_int {
                    0x80088822 as c_uint
                        >> ((3 as c_int - 4 as c_int)
                            * 8 as c_int
                            + (7 as c_int - 0 as c_int)
                            & 0x1f as c_int)
                } else {
                    0xaa55ff00 as c_uint
                        >> ((7 as c_int - 4 as c_int)
                            * 8 as c_int
                            + (7 as c_int - 0 as c_int)
                            & 0x1f as c_int)
                }) & 1 as c_uint
                    != 0
                {
                    ((1 as c_uint)
                        << (4 as c_int & 0x1f as c_int))
                        .wrapping_sub(1 as c_uint)
                        << ((0 as c_int * 4 as c_int
                            ^ (if 1 as c_int != 0 {
                                8 as c_int - 4 as c_int
                            } else {
                                0 as c_int
                            }))
                            & 0x1f as c_int)
                } else {
                    0 as c_uint
                })
                .wrapping_add(
                    (if (if (4 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 4 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 1 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 4 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 1 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (4 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((1 as c_int * 4 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 4 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (4 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 4 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 2 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 4 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 2 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (4 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((2 as c_int * 4 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 4 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (4 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 4 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 3 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 4 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 3 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (4 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((3 as c_int * 4 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 4 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (4 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 4 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 4 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 4 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 4 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (4 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((4 as c_int * 4 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 4 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (4 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 4 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 5 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 4 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 5 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (4 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((5 as c_int * 4 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 4 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (4 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 4 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 6 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 4 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 6 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (4 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((6 as c_int * 4 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 4 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (4 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 4 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 7 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 4 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 7 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (4 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((7 as c_int * 4 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 4 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_mul(
                    (if 4 as c_int == 1 as c_int {
                        0x1010101 as c_int
                    } else {
                        (if 4 as c_int == 2 as c_int {
                            0x10001 as c_int
                        } else {
                            1 as c_int
                        })
                    }) as c_uint,
                ),
                (if (if (5 as c_int) < 4 as c_int {
                    0x80088822 as c_uint
                        >> ((3 as c_int - 5 as c_int)
                            * 8 as c_int
                            + (7 as c_int - 0 as c_int)
                            & 0x1f as c_int)
                } else {
                    0xaa55ff00 as c_uint
                        >> ((7 as c_int - 5 as c_int)
                            * 8 as c_int
                            + (7 as c_int - 0 as c_int)
                            & 0x1f as c_int)
                }) & 1 as c_uint
                    != 0
                {
                    ((1 as c_uint)
                        << (4 as c_int & 0x1f as c_int))
                        .wrapping_sub(1 as c_uint)
                        << ((0 as c_int * 4 as c_int
                            ^ (if 1 as c_int != 0 {
                                8 as c_int - 4 as c_int
                            } else {
                                0 as c_int
                            }))
                            & 0x1f as c_int)
                } else {
                    0 as c_uint
                })
                .wrapping_add(
                    (if (if (5 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 1 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 1 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (4 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((1 as c_int * 4 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 4 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (5 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 2 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 2 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (4 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((2 as c_int * 4 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 4 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (5 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 3 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 3 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (4 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((3 as c_int * 4 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 4 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (5 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 4 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 4 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (4 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((4 as c_int * 4 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 4 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (5 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 5 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 5 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (4 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((5 as c_int * 4 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 4 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (5 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 6 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 6 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (4 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((6 as c_int * 4 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 4 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (5 as c_int) < 4 as c_int {
                        0x80088822 as c_uint
                            >> ((3 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 7 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xaa55ff00 as c_uint
                            >> ((7 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 7 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (4 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((7 as c_int * 4 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 4 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_mul(
                    (if 4 as c_int == 1 as c_int {
                        0x1010101 as c_int
                    } else {
                        (if 4 as c_int == 2 as c_int {
                            0x10001 as c_int
                        } else {
                            1 as c_int
                        })
                    }) as c_uint,
                ),
            ],
        ],
    ];
    display_mask = [
        [
            [
                (if (if (1 as c_int) < 4 as c_int {
                    0xff0fff33 as c_uint
                        >> ((3 as c_int - 1 as c_int)
                            * 8 as c_int
                            + (7 as c_int - 0 as c_int)
                            & 0x1f as c_int)
                } else {
                    0xff55ff00 as c_uint
                        >> ((7 as c_int - 1 as c_int)
                            * 8 as c_int
                            + (7 as c_int - 0 as c_int)
                            & 0x1f as c_int)
                }) & 1 as c_uint
                    != 0
                {
                    ((1 as c_uint)
                        << (1 as c_int & 0x1f as c_int))
                        .wrapping_sub(1 as c_uint)
                        << ((0 as c_int * 1 as c_int
                            ^ (if 0 as c_int != 0 {
                                8 as c_int - 1 as c_int
                            } else {
                                0 as c_int
                            }))
                            & 0x1f as c_int)
                } else {
                    0 as c_uint
                })
                .wrapping_add(
                    (if (if (1 as c_int) < 4 as c_int {
                        0xff0fff33 as c_uint
                            >> ((3 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 1 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xff55ff00 as c_uint
                            >> ((7 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 1 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (1 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((1 as c_int * 1 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 1 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (1 as c_int) < 4 as c_int {
                        0xff0fff33 as c_uint
                            >> ((3 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 2 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xff55ff00 as c_uint
                            >> ((7 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 2 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (1 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((2 as c_int * 1 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 1 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (1 as c_int) < 4 as c_int {
                        0xff0fff33 as c_uint
                            >> ((3 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 3 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xff55ff00 as c_uint
                            >> ((7 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 3 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (1 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((3 as c_int * 1 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 1 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (1 as c_int) < 4 as c_int {
                        0xff0fff33 as c_uint
                            >> ((3 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 4 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xff55ff00 as c_uint
                            >> ((7 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 4 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (1 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((4 as c_int * 1 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 1 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (1 as c_int) < 4 as c_int {
                        0xff0fff33 as c_uint
                            >> ((3 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 5 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xff55ff00 as c_uint
                            >> ((7 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 5 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (1 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((5 as c_int * 1 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 1 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (1 as c_int) < 4 as c_int {
                        0xff0fff33 as c_uint
                            >> ((3 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 6 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xff55ff00 as c_uint
                            >> ((7 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 6 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (1 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((6 as c_int * 1 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 1 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (1 as c_int) < 4 as c_int {
                        0xff0fff33 as c_uint
                            >> ((3 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 7 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xff55ff00 as c_uint
                            >> ((7 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 7 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (1 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((7 as c_int * 1 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 1 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_mul(
                    (if 1 as c_int == 1 as c_int {
                        0x1010101 as c_int
                    } else {
                        (if 1 as c_int == 2 as c_int {
                            0x10001 as c_int
                        } else {
                            1 as c_int
                        })
                    }) as c_uint,
                ),
                (if (if (3 as c_int) < 4 as c_int {
                    0xff0fff33 as c_uint
                        >> ((3 as c_int - 3 as c_int)
                            * 8 as c_int
                            + (7 as c_int - 0 as c_int)
                            & 0x1f as c_int)
                } else {
                    0xff55ff00 as c_uint
                        >> ((7 as c_int - 3 as c_int)
                            * 8 as c_int
                            + (7 as c_int - 0 as c_int)
                            & 0x1f as c_int)
                }) & 1 as c_uint
                    != 0
                {
                    ((1 as c_uint)
                        << (1 as c_int & 0x1f as c_int))
                        .wrapping_sub(1 as c_uint)
                        << ((0 as c_int * 1 as c_int
                            ^ (if 0 as c_int != 0 {
                                8 as c_int - 1 as c_int
                            } else {
                                0 as c_int
                            }))
                            & 0x1f as c_int)
                } else {
                    0 as c_uint
                })
                .wrapping_add(
                    (if (if (3 as c_int) < 4 as c_int {
                        0xff0fff33 as c_uint
                            >> ((3 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 1 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xff55ff00 as c_uint
                            >> ((7 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 1 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (1 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((1 as c_int * 1 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 1 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (3 as c_int) < 4 as c_int {
                        0xff0fff33 as c_uint
                            >> ((3 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 2 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xff55ff00 as c_uint
                            >> ((7 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 2 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (1 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((2 as c_int * 1 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 1 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (3 as c_int) < 4 as c_int {
                        0xff0fff33 as c_uint
                            >> ((3 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 3 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xff55ff00 as c_uint
                            >> ((7 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 3 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (1 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((3 as c_int * 1 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 1 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (3 as c_int) < 4 as c_int {
                        0xff0fff33 as c_uint
                            >> ((3 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 4 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xff55ff00 as c_uint
                            >> ((7 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 4 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (1 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((4 as c_int * 1 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 1 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (3 as c_int) < 4 as c_int {
                        0xff0fff33 as c_uint
                            >> ((3 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 5 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xff55ff00 as c_uint
                            >> ((7 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 5 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (1 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((5 as c_int * 1 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 1 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (3 as c_int) < 4 as c_int {
                        0xff0fff33 as c_uint
                            >> ((3 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 6 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xff55ff00 as c_uint
                            >> ((7 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 6 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (1 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((6 as c_int * 1 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 1 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (3 as c_int) < 4 as c_int {
                        0xff0fff33 as c_uint
                            >> ((3 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 7 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xff55ff00 as c_uint
                            >> ((7 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 7 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (1 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((7 as c_int * 1 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 1 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_mul(
                    (if 1 as c_int == 1 as c_int {
                        0x1010101 as c_int
                    } else {
                        (if 1 as c_int == 2 as c_int {
                            0x10001 as c_int
                        } else {
                            1 as c_int
                        })
                    }) as c_uint,
                ),
                (if (if (5 as c_int) < 4 as c_int {
                    0xff0fff33 as c_uint
                        >> ((3 as c_int - 5 as c_int)
                            * 8 as c_int
                            + (7 as c_int - 0 as c_int)
                            & 0x1f as c_int)
                } else {
                    0xff55ff00 as c_uint
                        >> ((7 as c_int - 5 as c_int)
                            * 8 as c_int
                            + (7 as c_int - 0 as c_int)
                            & 0x1f as c_int)
                }) & 1 as c_uint
                    != 0
                {
                    ((1 as c_uint)
                        << (1 as c_int & 0x1f as c_int))
                        .wrapping_sub(1 as c_uint)
                        << ((0 as c_int * 1 as c_int
                            ^ (if 0 as c_int != 0 {
                                8 as c_int - 1 as c_int
                            } else {
                                0 as c_int
                            }))
                            & 0x1f as c_int)
                } else {
                    0 as c_uint
                })
                .wrapping_add(
                    (if (if (5 as c_int) < 4 as c_int {
                        0xff0fff33 as c_uint
                            >> ((3 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 1 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xff55ff00 as c_uint
                            >> ((7 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 1 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (1 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((1 as c_int * 1 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 1 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (5 as c_int) < 4 as c_int {
                        0xff0fff33 as c_uint
                            >> ((3 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 2 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xff55ff00 as c_uint
                            >> ((7 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 2 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (1 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((2 as c_int * 1 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 1 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (5 as c_int) < 4 as c_int {
                        0xff0fff33 as c_uint
                            >> ((3 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 3 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xff55ff00 as c_uint
                            >> ((7 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 3 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (1 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((3 as c_int * 1 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 1 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (5 as c_int) < 4 as c_int {
                        0xff0fff33 as c_uint
                            >> ((3 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 4 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xff55ff00 as c_uint
                            >> ((7 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 4 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (1 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((4 as c_int * 1 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 1 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (5 as c_int) < 4 as c_int {
                        0xff0fff33 as c_uint
                            >> ((3 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 5 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xff55ff00 as c_uint
                            >> ((7 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 5 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (1 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((5 as c_int * 1 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 1 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (5 as c_int) < 4 as c_int {
                        0xff0fff33 as c_uint
                            >> ((3 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 6 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xff55ff00 as c_uint
                            >> ((7 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 6 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (1 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((6 as c_int * 1 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 1 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (5 as c_int) < 4 as c_int {
                        0xff0fff33 as c_uint
                            >> ((3 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 7 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xff55ff00 as c_uint
                            >> ((7 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 7 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (1 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((7 as c_int * 1 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 1 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_mul(
                    (if 1 as c_int == 1 as c_int {
                        0x1010101 as c_int
                    } else {
                        (if 1 as c_int == 2 as c_int {
                            0x10001 as c_int
                        } else {
                            1 as c_int
                        })
                    }) as c_uint,
                ),
            ],
            [
                (if (if (1 as c_int) < 4 as c_int {
                    0xff0fff33 as c_uint
                        >> ((3 as c_int - 1 as c_int)
                            * 8 as c_int
                            + (7 as c_int - 0 as c_int)
                            & 0x1f as c_int)
                } else {
                    0xff55ff00 as c_uint
                        >> ((7 as c_int - 1 as c_int)
                            * 8 as c_int
                            + (7 as c_int - 0 as c_int)
                            & 0x1f as c_int)
                }) & 1 as c_uint
                    != 0
                {
                    ((1 as c_uint)
                        << (2 as c_int & 0x1f as c_int))
                        .wrapping_sub(1 as c_uint)
                        << ((0 as c_int * 2 as c_int
                            ^ (if 0 as c_int != 0 {
                                8 as c_int - 2 as c_int
                            } else {
                                0 as c_int
                            }))
                            & 0x1f as c_int)
                } else {
                    0 as c_uint
                })
                .wrapping_add(
                    (if (if (1 as c_int) < 4 as c_int {
                        0xff0fff33 as c_uint
                            >> ((3 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 1 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xff55ff00 as c_uint
                            >> ((7 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 1 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (2 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((1 as c_int * 2 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 2 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (1 as c_int) < 4 as c_int {
                        0xff0fff33 as c_uint
                            >> ((3 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 2 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xff55ff00 as c_uint
                            >> ((7 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 2 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (2 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((2 as c_int * 2 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 2 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (1 as c_int) < 4 as c_int {
                        0xff0fff33 as c_uint
                            >> ((3 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 3 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xff55ff00 as c_uint
                            >> ((7 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 3 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (2 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((3 as c_int * 2 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 2 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (1 as c_int) < 4 as c_int {
                        0xff0fff33 as c_uint
                            >> ((3 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 4 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xff55ff00 as c_uint
                            >> ((7 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 4 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (2 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((4 as c_int * 2 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 2 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (1 as c_int) < 4 as c_int {
                        0xff0fff33 as c_uint
                            >> ((3 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 5 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xff55ff00 as c_uint
                            >> ((7 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 5 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (2 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((5 as c_int * 2 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 2 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (1 as c_int) < 4 as c_int {
                        0xff0fff33 as c_uint
                            >> ((3 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 6 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xff55ff00 as c_uint
                            >> ((7 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 6 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (2 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((6 as c_int * 2 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 2 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (1 as c_int) < 4 as c_int {
                        0xff0fff33 as c_uint
                            >> ((3 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 7 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xff55ff00 as c_uint
                            >> ((7 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 7 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (2 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((7 as c_int * 2 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 2 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_mul(
                    (if 2 as c_int == 1 as c_int {
                        0x1010101 as c_int
                    } else {
                        (if 2 as c_int == 2 as c_int {
                            0x10001 as c_int
                        } else {
                            1 as c_int
                        })
                    }) as c_uint,
                ),
                (if (if (3 as c_int) < 4 as c_int {
                    0xff0fff33 as c_uint
                        >> ((3 as c_int - 3 as c_int)
                            * 8 as c_int
                            + (7 as c_int - 0 as c_int)
                            & 0x1f as c_int)
                } else {
                    0xff55ff00 as c_uint
                        >> ((7 as c_int - 3 as c_int)
                            * 8 as c_int
                            + (7 as c_int - 0 as c_int)
                            & 0x1f as c_int)
                }) & 1 as c_uint
                    != 0
                {
                    ((1 as c_uint)
                        << (2 as c_int & 0x1f as c_int))
                        .wrapping_sub(1 as c_uint)
                        << ((0 as c_int * 2 as c_int
                            ^ (if 0 as c_int != 0 {
                                8 as c_int - 2 as c_int
                            } else {
                                0 as c_int
                            }))
                            & 0x1f as c_int)
                } else {
                    0 as c_uint
                })
                .wrapping_add(
                    (if (if (3 as c_int) < 4 as c_int {
                        0xff0fff33 as c_uint
                            >> ((3 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 1 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xff55ff00 as c_uint
                            >> ((7 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 1 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (2 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((1 as c_int * 2 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 2 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (3 as c_int) < 4 as c_int {
                        0xff0fff33 as c_uint
                            >> ((3 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 2 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xff55ff00 as c_uint
                            >> ((7 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 2 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (2 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((2 as c_int * 2 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 2 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (3 as c_int) < 4 as c_int {
                        0xff0fff33 as c_uint
                            >> ((3 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 3 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xff55ff00 as c_uint
                            >> ((7 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 3 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (2 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((3 as c_int * 2 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 2 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (3 as c_int) < 4 as c_int {
                        0xff0fff33 as c_uint
                            >> ((3 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 4 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xff55ff00 as c_uint
                            >> ((7 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 4 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (2 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((4 as c_int * 2 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 2 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (3 as c_int) < 4 as c_int {
                        0xff0fff33 as c_uint
                            >> ((3 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 5 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xff55ff00 as c_uint
                            >> ((7 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 5 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (2 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((5 as c_int * 2 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 2 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (3 as c_int) < 4 as c_int {
                        0xff0fff33 as c_uint
                            >> ((3 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 6 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xff55ff00 as c_uint
                            >> ((7 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 6 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (2 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((6 as c_int * 2 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 2 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (3 as c_int) < 4 as c_int {
                        0xff0fff33 as c_uint
                            >> ((3 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 7 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xff55ff00 as c_uint
                            >> ((7 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 7 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (2 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((7 as c_int * 2 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 2 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_mul(
                    (if 2 as c_int == 1 as c_int {
                        0x1010101 as c_int
                    } else {
                        (if 2 as c_int == 2 as c_int {
                            0x10001 as c_int
                        } else {
                            1 as c_int
                        })
                    }) as c_uint,
                ),
                (if (if (5 as c_int) < 4 as c_int {
                    0xff0fff33 as c_uint
                        >> ((3 as c_int - 5 as c_int)
                            * 8 as c_int
                            + (7 as c_int - 0 as c_int)
                            & 0x1f as c_int)
                } else {
                    0xff55ff00 as c_uint
                        >> ((7 as c_int - 5 as c_int)
                            * 8 as c_int
                            + (7 as c_int - 0 as c_int)
                            & 0x1f as c_int)
                }) & 1 as c_uint
                    != 0
                {
                    ((1 as c_uint)
                        << (2 as c_int & 0x1f as c_int))
                        .wrapping_sub(1 as c_uint)
                        << ((0 as c_int * 2 as c_int
                            ^ (if 0 as c_int != 0 {
                                8 as c_int - 2 as c_int
                            } else {
                                0 as c_int
                            }))
                            & 0x1f as c_int)
                } else {
                    0 as c_uint
                })
                .wrapping_add(
                    (if (if (5 as c_int) < 4 as c_int {
                        0xff0fff33 as c_uint
                            >> ((3 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 1 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xff55ff00 as c_uint
                            >> ((7 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 1 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (2 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((1 as c_int * 2 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 2 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (5 as c_int) < 4 as c_int {
                        0xff0fff33 as c_uint
                            >> ((3 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 2 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xff55ff00 as c_uint
                            >> ((7 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 2 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (2 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((2 as c_int * 2 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 2 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (5 as c_int) < 4 as c_int {
                        0xff0fff33 as c_uint
                            >> ((3 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 3 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xff55ff00 as c_uint
                            >> ((7 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 3 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (2 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((3 as c_int * 2 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 2 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (5 as c_int) < 4 as c_int {
                        0xff0fff33 as c_uint
                            >> ((3 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 4 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xff55ff00 as c_uint
                            >> ((7 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 4 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (2 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((4 as c_int * 2 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 2 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (5 as c_int) < 4 as c_int {
                        0xff0fff33 as c_uint
                            >> ((3 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 5 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xff55ff00 as c_uint
                            >> ((7 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 5 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (2 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((5 as c_int * 2 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 2 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (5 as c_int) < 4 as c_int {
                        0xff0fff33 as c_uint
                            >> ((3 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 6 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xff55ff00 as c_uint
                            >> ((7 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 6 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (2 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((6 as c_int * 2 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 2 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (5 as c_int) < 4 as c_int {
                        0xff0fff33 as c_uint
                            >> ((3 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 7 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xff55ff00 as c_uint
                            >> ((7 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 7 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (2 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((7 as c_int * 2 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 2 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_mul(
                    (if 2 as c_int == 1 as c_int {
                        0x1010101 as c_int
                    } else {
                        (if 2 as c_int == 2 as c_int {
                            0x10001 as c_int
                        } else {
                            1 as c_int
                        })
                    }) as c_uint,
                ),
            ],
            [
                (if (if (1 as c_int) < 4 as c_int {
                    0xff0fff33 as c_uint
                        >> ((3 as c_int - 1 as c_int)
                            * 8 as c_int
                            + (7 as c_int - 0 as c_int)
                            & 0x1f as c_int)
                } else {
                    0xff55ff00 as c_uint
                        >> ((7 as c_int - 1 as c_int)
                            * 8 as c_int
                            + (7 as c_int - 0 as c_int)
                            & 0x1f as c_int)
                }) & 1 as c_uint
                    != 0
                {
                    ((1 as c_uint)
                        << (4 as c_int & 0x1f as c_int))
                        .wrapping_sub(1 as c_uint)
                        << ((0 as c_int * 4 as c_int
                            ^ (if 0 as c_int != 0 {
                                8 as c_int - 4 as c_int
                            } else {
                                0 as c_int
                            }))
                            & 0x1f as c_int)
                } else {
                    0 as c_uint
                })
                .wrapping_add(
                    (if (if (1 as c_int) < 4 as c_int {
                        0xff0fff33 as c_uint
                            >> ((3 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 1 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xff55ff00 as c_uint
                            >> ((7 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 1 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (4 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((1 as c_int * 4 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 4 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (1 as c_int) < 4 as c_int {
                        0xff0fff33 as c_uint
                            >> ((3 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 2 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xff55ff00 as c_uint
                            >> ((7 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 2 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (4 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((2 as c_int * 4 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 4 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (1 as c_int) < 4 as c_int {
                        0xff0fff33 as c_uint
                            >> ((3 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 3 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xff55ff00 as c_uint
                            >> ((7 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 3 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (4 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((3 as c_int * 4 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 4 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (1 as c_int) < 4 as c_int {
                        0xff0fff33 as c_uint
                            >> ((3 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 4 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xff55ff00 as c_uint
                            >> ((7 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 4 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (4 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((4 as c_int * 4 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 4 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (1 as c_int) < 4 as c_int {
                        0xff0fff33 as c_uint
                            >> ((3 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 5 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xff55ff00 as c_uint
                            >> ((7 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 5 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (4 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((5 as c_int * 4 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 4 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (1 as c_int) < 4 as c_int {
                        0xff0fff33 as c_uint
                            >> ((3 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 6 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xff55ff00 as c_uint
                            >> ((7 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 6 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (4 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((6 as c_int * 4 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 4 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (1 as c_int) < 4 as c_int {
                        0xff0fff33 as c_uint
                            >> ((3 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 7 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xff55ff00 as c_uint
                            >> ((7 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 7 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (4 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((7 as c_int * 4 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 4 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_mul(
                    (if 4 as c_int == 1 as c_int {
                        0x1010101 as c_int
                    } else {
                        (if 4 as c_int == 2 as c_int {
                            0x10001 as c_int
                        } else {
                            1 as c_int
                        })
                    }) as c_uint,
                ),
                (if (if (3 as c_int) < 4 as c_int {
                    0xff0fff33 as c_uint
                        >> ((3 as c_int - 3 as c_int)
                            * 8 as c_int
                            + (7 as c_int - 0 as c_int)
                            & 0x1f as c_int)
                } else {
                    0xff55ff00 as c_uint
                        >> ((7 as c_int - 3 as c_int)
                            * 8 as c_int
                            + (7 as c_int - 0 as c_int)
                            & 0x1f as c_int)
                }) & 1 as c_uint
                    != 0
                {
                    ((1 as c_uint)
                        << (4 as c_int & 0x1f as c_int))
                        .wrapping_sub(1 as c_uint)
                        << ((0 as c_int * 4 as c_int
                            ^ (if 0 as c_int != 0 {
                                8 as c_int - 4 as c_int
                            } else {
                                0 as c_int
                            }))
                            & 0x1f as c_int)
                } else {
                    0 as c_uint
                })
                .wrapping_add(
                    (if (if (3 as c_int) < 4 as c_int {
                        0xff0fff33 as c_uint
                            >> ((3 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 1 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xff55ff00 as c_uint
                            >> ((7 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 1 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (4 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((1 as c_int * 4 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 4 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (3 as c_int) < 4 as c_int {
                        0xff0fff33 as c_uint
                            >> ((3 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 2 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xff55ff00 as c_uint
                            >> ((7 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 2 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (4 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((2 as c_int * 4 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 4 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (3 as c_int) < 4 as c_int {
                        0xff0fff33 as c_uint
                            >> ((3 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 3 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xff55ff00 as c_uint
                            >> ((7 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 3 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (4 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((3 as c_int * 4 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 4 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (3 as c_int) < 4 as c_int {
                        0xff0fff33 as c_uint
                            >> ((3 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 4 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xff55ff00 as c_uint
                            >> ((7 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 4 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (4 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((4 as c_int * 4 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 4 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (3 as c_int) < 4 as c_int {
                        0xff0fff33 as c_uint
                            >> ((3 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 5 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xff55ff00 as c_uint
                            >> ((7 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 5 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (4 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((5 as c_int * 4 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 4 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (3 as c_int) < 4 as c_int {
                        0xff0fff33 as c_uint
                            >> ((3 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 6 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xff55ff00 as c_uint
                            >> ((7 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 6 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (4 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((6 as c_int * 4 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 4 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (3 as c_int) < 4 as c_int {
                        0xff0fff33 as c_uint
                            >> ((3 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 7 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xff55ff00 as c_uint
                            >> ((7 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 7 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (4 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((7 as c_int * 4 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 4 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_mul(
                    (if 4 as c_int == 1 as c_int {
                        0x1010101 as c_int
                    } else {
                        (if 4 as c_int == 2 as c_int {
                            0x10001 as c_int
                        } else {
                            1 as c_int
                        })
                    }) as c_uint,
                ),
                (if (if (5 as c_int) < 4 as c_int {
                    0xff0fff33 as c_uint
                        >> ((3 as c_int - 5 as c_int)
                            * 8 as c_int
                            + (7 as c_int - 0 as c_int)
                            & 0x1f as c_int)
                } else {
                    0xff55ff00 as c_uint
                        >> ((7 as c_int - 5 as c_int)
                            * 8 as c_int
                            + (7 as c_int - 0 as c_int)
                            & 0x1f as c_int)
                }) & 1 as c_uint
                    != 0
                {
                    ((1 as c_uint)
                        << (4 as c_int & 0x1f as c_int))
                        .wrapping_sub(1 as c_uint)
                        << ((0 as c_int * 4 as c_int
                            ^ (if 0 as c_int != 0 {
                                8 as c_int - 4 as c_int
                            } else {
                                0 as c_int
                            }))
                            & 0x1f as c_int)
                } else {
                    0 as c_uint
                })
                .wrapping_add(
                    (if (if (5 as c_int) < 4 as c_int {
                        0xff0fff33 as c_uint
                            >> ((3 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 1 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xff55ff00 as c_uint
                            >> ((7 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 1 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (4 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((1 as c_int * 4 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 4 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (5 as c_int) < 4 as c_int {
                        0xff0fff33 as c_uint
                            >> ((3 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 2 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xff55ff00 as c_uint
                            >> ((7 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 2 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (4 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((2 as c_int * 4 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 4 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (5 as c_int) < 4 as c_int {
                        0xff0fff33 as c_uint
                            >> ((3 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 3 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xff55ff00 as c_uint
                            >> ((7 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 3 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (4 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((3 as c_int * 4 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 4 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (5 as c_int) < 4 as c_int {
                        0xff0fff33 as c_uint
                            >> ((3 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 4 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xff55ff00 as c_uint
                            >> ((7 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 4 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (4 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((4 as c_int * 4 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 4 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (5 as c_int) < 4 as c_int {
                        0xff0fff33 as c_uint
                            >> ((3 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 5 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xff55ff00 as c_uint
                            >> ((7 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 5 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (4 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((5 as c_int * 4 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 4 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (5 as c_int) < 4 as c_int {
                        0xff0fff33 as c_uint
                            >> ((3 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 6 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xff55ff00 as c_uint
                            >> ((7 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 6 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (4 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((6 as c_int * 4 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 4 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (5 as c_int) < 4 as c_int {
                        0xff0fff33 as c_uint
                            >> ((3 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 7 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xff55ff00 as c_uint
                            >> ((7 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 7 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (4 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((7 as c_int * 4 as c_int
                                ^ (if 0 as c_int != 0 {
                                    8 as c_int - 4 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_mul(
                    (if 4 as c_int == 1 as c_int {
                        0x1010101 as c_int
                    } else {
                        (if 4 as c_int == 2 as c_int {
                            0x10001 as c_int
                        } else {
                            1 as c_int
                        })
                    }) as c_uint,
                ),
            ],
        ],
        [
            [
                (if (if (1 as c_int) < 4 as c_int {
                    0xff0fff33 as c_uint
                        >> ((3 as c_int - 1 as c_int)
                            * 8 as c_int
                            + (7 as c_int - 0 as c_int)
                            & 0x1f as c_int)
                } else {
                    0xff55ff00 as c_uint
                        >> ((7 as c_int - 1 as c_int)
                            * 8 as c_int
                            + (7 as c_int - 0 as c_int)
                            & 0x1f as c_int)
                }) & 1 as c_uint
                    != 0
                {
                    ((1 as c_uint)
                        << (1 as c_int & 0x1f as c_int))
                        .wrapping_sub(1 as c_uint)
                        << ((0 as c_int * 1 as c_int
                            ^ (if 1 as c_int != 0 {
                                8 as c_int - 1 as c_int
                            } else {
                                0 as c_int
                            }))
                            & 0x1f as c_int)
                } else {
                    0 as c_uint
                })
                .wrapping_add(
                    (if (if (1 as c_int) < 4 as c_int {
                        0xff0fff33 as c_uint
                            >> ((3 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 1 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xff55ff00 as c_uint
                            >> ((7 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 1 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (1 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((1 as c_int * 1 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 1 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (1 as c_int) < 4 as c_int {
                        0xff0fff33 as c_uint
                            >> ((3 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 2 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xff55ff00 as c_uint
                            >> ((7 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 2 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (1 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((2 as c_int * 1 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 1 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (1 as c_int) < 4 as c_int {
                        0xff0fff33 as c_uint
                            >> ((3 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 3 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xff55ff00 as c_uint
                            >> ((7 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 3 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (1 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((3 as c_int * 1 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 1 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (1 as c_int) < 4 as c_int {
                        0xff0fff33 as c_uint
                            >> ((3 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 4 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xff55ff00 as c_uint
                            >> ((7 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 4 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (1 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((4 as c_int * 1 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 1 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (1 as c_int) < 4 as c_int {
                        0xff0fff33 as c_uint
                            >> ((3 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 5 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xff55ff00 as c_uint
                            >> ((7 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 5 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (1 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((5 as c_int * 1 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 1 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (1 as c_int) < 4 as c_int {
                        0xff0fff33 as c_uint
                            >> ((3 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 6 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xff55ff00 as c_uint
                            >> ((7 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 6 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (1 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((6 as c_int * 1 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 1 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (1 as c_int) < 4 as c_int {
                        0xff0fff33 as c_uint
                            >> ((3 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 7 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xff55ff00 as c_uint
                            >> ((7 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 7 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (1 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((7 as c_int * 1 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 1 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_mul(
                    (if 1 as c_int == 1 as c_int {
                        0x1010101 as c_int
                    } else {
                        (if 1 as c_int == 2 as c_int {
                            0x10001 as c_int
                        } else {
                            1 as c_int
                        })
                    }) as c_uint,
                ),
                (if (if (3 as c_int) < 4 as c_int {
                    0xff0fff33 as c_uint
                        >> ((3 as c_int - 3 as c_int)
                            * 8 as c_int
                            + (7 as c_int - 0 as c_int)
                            & 0x1f as c_int)
                } else {
                    0xff55ff00 as c_uint
                        >> ((7 as c_int - 3 as c_int)
                            * 8 as c_int
                            + (7 as c_int - 0 as c_int)
                            & 0x1f as c_int)
                }) & 1 as c_uint
                    != 0
                {
                    ((1 as c_uint)
                        << (1 as c_int & 0x1f as c_int))
                        .wrapping_sub(1 as c_uint)
                        << ((0 as c_int * 1 as c_int
                            ^ (if 1 as c_int != 0 {
                                8 as c_int - 1 as c_int
                            } else {
                                0 as c_int
                            }))
                            & 0x1f as c_int)
                } else {
                    0 as c_uint
                })
                .wrapping_add(
                    (if (if (3 as c_int) < 4 as c_int {
                        0xff0fff33 as c_uint
                            >> ((3 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 1 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xff55ff00 as c_uint
                            >> ((7 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 1 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (1 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((1 as c_int * 1 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 1 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (3 as c_int) < 4 as c_int {
                        0xff0fff33 as c_uint
                            >> ((3 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 2 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xff55ff00 as c_uint
                            >> ((7 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 2 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (1 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((2 as c_int * 1 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 1 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (3 as c_int) < 4 as c_int {
                        0xff0fff33 as c_uint
                            >> ((3 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 3 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xff55ff00 as c_uint
                            >> ((7 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 3 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (1 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((3 as c_int * 1 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 1 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (3 as c_int) < 4 as c_int {
                        0xff0fff33 as c_uint
                            >> ((3 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 4 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xff55ff00 as c_uint
                            >> ((7 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 4 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (1 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((4 as c_int * 1 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 1 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (3 as c_int) < 4 as c_int {
                        0xff0fff33 as c_uint
                            >> ((3 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 5 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xff55ff00 as c_uint
                            >> ((7 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 5 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (1 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((5 as c_int * 1 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 1 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (3 as c_int) < 4 as c_int {
                        0xff0fff33 as c_uint
                            >> ((3 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 6 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xff55ff00 as c_uint
                            >> ((7 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 6 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (1 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((6 as c_int * 1 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 1 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (3 as c_int) < 4 as c_int {
                        0xff0fff33 as c_uint
                            >> ((3 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 7 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xff55ff00 as c_uint
                            >> ((7 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 7 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (1 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((7 as c_int * 1 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 1 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_mul(
                    (if 1 as c_int == 1 as c_int {
                        0x1010101 as c_int
                    } else {
                        (if 1 as c_int == 2 as c_int {
                            0x10001 as c_int
                        } else {
                            1 as c_int
                        })
                    }) as c_uint,
                ),
                (if (if (5 as c_int) < 4 as c_int {
                    0xff0fff33 as c_uint
                        >> ((3 as c_int - 5 as c_int)
                            * 8 as c_int
                            + (7 as c_int - 0 as c_int)
                            & 0x1f as c_int)
                } else {
                    0xff55ff00 as c_uint
                        >> ((7 as c_int - 5 as c_int)
                            * 8 as c_int
                            + (7 as c_int - 0 as c_int)
                            & 0x1f as c_int)
                }) & 1 as c_uint
                    != 0
                {
                    ((1 as c_uint)
                        << (1 as c_int & 0x1f as c_int))
                        .wrapping_sub(1 as c_uint)
                        << ((0 as c_int * 1 as c_int
                            ^ (if 1 as c_int != 0 {
                                8 as c_int - 1 as c_int
                            } else {
                                0 as c_int
                            }))
                            & 0x1f as c_int)
                } else {
                    0 as c_uint
                })
                .wrapping_add(
                    (if (if (5 as c_int) < 4 as c_int {
                        0xff0fff33 as c_uint
                            >> ((3 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 1 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xff55ff00 as c_uint
                            >> ((7 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 1 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (1 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((1 as c_int * 1 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 1 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (5 as c_int) < 4 as c_int {
                        0xff0fff33 as c_uint
                            >> ((3 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 2 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xff55ff00 as c_uint
                            >> ((7 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 2 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (1 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((2 as c_int * 1 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 1 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (5 as c_int) < 4 as c_int {
                        0xff0fff33 as c_uint
                            >> ((3 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 3 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xff55ff00 as c_uint
                            >> ((7 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 3 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (1 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((3 as c_int * 1 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 1 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (5 as c_int) < 4 as c_int {
                        0xff0fff33 as c_uint
                            >> ((3 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 4 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xff55ff00 as c_uint
                            >> ((7 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 4 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (1 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((4 as c_int * 1 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 1 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (5 as c_int) < 4 as c_int {
                        0xff0fff33 as c_uint
                            >> ((3 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 5 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xff55ff00 as c_uint
                            >> ((7 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 5 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (1 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((5 as c_int * 1 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 1 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (5 as c_int) < 4 as c_int {
                        0xff0fff33 as c_uint
                            >> ((3 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 6 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xff55ff00 as c_uint
                            >> ((7 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 6 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (1 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((6 as c_int * 1 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 1 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (5 as c_int) < 4 as c_int {
                        0xff0fff33 as c_uint
                            >> ((3 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 7 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xff55ff00 as c_uint
                            >> ((7 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 7 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (1 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((7 as c_int * 1 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 1 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_mul(
                    (if 1 as c_int == 1 as c_int {
                        0x1010101 as c_int
                    } else {
                        (if 1 as c_int == 2 as c_int {
                            0x10001 as c_int
                        } else {
                            1 as c_int
                        })
                    }) as c_uint,
                ),
            ],
            [
                (if (if (1 as c_int) < 4 as c_int {
                    0xff0fff33 as c_uint
                        >> ((3 as c_int - 1 as c_int)
                            * 8 as c_int
                            + (7 as c_int - 0 as c_int)
                            & 0x1f as c_int)
                } else {
                    0xff55ff00 as c_uint
                        >> ((7 as c_int - 1 as c_int)
                            * 8 as c_int
                            + (7 as c_int - 0 as c_int)
                            & 0x1f as c_int)
                }) & 1 as c_uint
                    != 0
                {
                    ((1 as c_uint)
                        << (2 as c_int & 0x1f as c_int))
                        .wrapping_sub(1 as c_uint)
                        << ((0 as c_int * 2 as c_int
                            ^ (if 1 as c_int != 0 {
                                8 as c_int - 2 as c_int
                            } else {
                                0 as c_int
                            }))
                            & 0x1f as c_int)
                } else {
                    0 as c_uint
                })
                .wrapping_add(
                    (if (if (1 as c_int) < 4 as c_int {
                        0xff0fff33 as c_uint
                            >> ((3 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 1 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xff55ff00 as c_uint
                            >> ((7 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 1 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (2 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((1 as c_int * 2 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 2 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (1 as c_int) < 4 as c_int {
                        0xff0fff33 as c_uint
                            >> ((3 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 2 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xff55ff00 as c_uint
                            >> ((7 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 2 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (2 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((2 as c_int * 2 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 2 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (1 as c_int) < 4 as c_int {
                        0xff0fff33 as c_uint
                            >> ((3 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 3 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xff55ff00 as c_uint
                            >> ((7 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 3 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (2 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((3 as c_int * 2 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 2 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (1 as c_int) < 4 as c_int {
                        0xff0fff33 as c_uint
                            >> ((3 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 4 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xff55ff00 as c_uint
                            >> ((7 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 4 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (2 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((4 as c_int * 2 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 2 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (1 as c_int) < 4 as c_int {
                        0xff0fff33 as c_uint
                            >> ((3 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 5 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xff55ff00 as c_uint
                            >> ((7 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 5 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (2 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((5 as c_int * 2 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 2 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (1 as c_int) < 4 as c_int {
                        0xff0fff33 as c_uint
                            >> ((3 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 6 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xff55ff00 as c_uint
                            >> ((7 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 6 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (2 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((6 as c_int * 2 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 2 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (1 as c_int) < 4 as c_int {
                        0xff0fff33 as c_uint
                            >> ((3 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 7 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xff55ff00 as c_uint
                            >> ((7 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 7 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (2 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((7 as c_int * 2 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 2 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_mul(
                    (if 2 as c_int == 1 as c_int {
                        0x1010101 as c_int
                    } else {
                        (if 2 as c_int == 2 as c_int {
                            0x10001 as c_int
                        } else {
                            1 as c_int
                        })
                    }) as c_uint,
                ),
                (if (if (3 as c_int) < 4 as c_int {
                    0xff0fff33 as c_uint
                        >> ((3 as c_int - 3 as c_int)
                            * 8 as c_int
                            + (7 as c_int - 0 as c_int)
                            & 0x1f as c_int)
                } else {
                    0xff55ff00 as c_uint
                        >> ((7 as c_int - 3 as c_int)
                            * 8 as c_int
                            + (7 as c_int - 0 as c_int)
                            & 0x1f as c_int)
                }) & 1 as c_uint
                    != 0
                {
                    ((1 as c_uint)
                        << (2 as c_int & 0x1f as c_int))
                        .wrapping_sub(1 as c_uint)
                        << ((0 as c_int * 2 as c_int
                            ^ (if 1 as c_int != 0 {
                                8 as c_int - 2 as c_int
                            } else {
                                0 as c_int
                            }))
                            & 0x1f as c_int)
                } else {
                    0 as c_uint
                })
                .wrapping_add(
                    (if (if (3 as c_int) < 4 as c_int {
                        0xff0fff33 as c_uint
                            >> ((3 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 1 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xff55ff00 as c_uint
                            >> ((7 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 1 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (2 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((1 as c_int * 2 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 2 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (3 as c_int) < 4 as c_int {
                        0xff0fff33 as c_uint
                            >> ((3 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 2 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xff55ff00 as c_uint
                            >> ((7 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 2 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (2 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((2 as c_int * 2 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 2 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (3 as c_int) < 4 as c_int {
                        0xff0fff33 as c_uint
                            >> ((3 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 3 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xff55ff00 as c_uint
                            >> ((7 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 3 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (2 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((3 as c_int * 2 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 2 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (3 as c_int) < 4 as c_int {
                        0xff0fff33 as c_uint
                            >> ((3 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 4 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xff55ff00 as c_uint
                            >> ((7 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 4 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (2 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((4 as c_int * 2 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 2 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (3 as c_int) < 4 as c_int {
                        0xff0fff33 as c_uint
                            >> ((3 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 5 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xff55ff00 as c_uint
                            >> ((7 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 5 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (2 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((5 as c_int * 2 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 2 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (3 as c_int) < 4 as c_int {
                        0xff0fff33 as c_uint
                            >> ((3 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 6 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xff55ff00 as c_uint
                            >> ((7 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 6 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (2 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((6 as c_int * 2 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 2 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (3 as c_int) < 4 as c_int {
                        0xff0fff33 as c_uint
                            >> ((3 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 7 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xff55ff00 as c_uint
                            >> ((7 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 7 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (2 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((7 as c_int * 2 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 2 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_mul(
                    (if 2 as c_int == 1 as c_int {
                        0x1010101 as c_int
                    } else {
                        (if 2 as c_int == 2 as c_int {
                            0x10001 as c_int
                        } else {
                            1 as c_int
                        })
                    }) as c_uint,
                ),
                (if (if (5 as c_int) < 4 as c_int {
                    0xff0fff33 as c_uint
                        >> ((3 as c_int - 5 as c_int)
                            * 8 as c_int
                            + (7 as c_int - 0 as c_int)
                            & 0x1f as c_int)
                } else {
                    0xff55ff00 as c_uint
                        >> ((7 as c_int - 5 as c_int)
                            * 8 as c_int
                            + (7 as c_int - 0 as c_int)
                            & 0x1f as c_int)
                }) & 1 as c_uint
                    != 0
                {
                    ((1 as c_uint)
                        << (2 as c_int & 0x1f as c_int))
                        .wrapping_sub(1 as c_uint)
                        << ((0 as c_int * 2 as c_int
                            ^ (if 1 as c_int != 0 {
                                8 as c_int - 2 as c_int
                            } else {
                                0 as c_int
                            }))
                            & 0x1f as c_int)
                } else {
                    0 as c_uint
                })
                .wrapping_add(
                    (if (if (5 as c_int) < 4 as c_int {
                        0xff0fff33 as c_uint
                            >> ((3 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 1 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xff55ff00 as c_uint
                            >> ((7 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 1 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (2 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((1 as c_int * 2 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 2 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (5 as c_int) < 4 as c_int {
                        0xff0fff33 as c_uint
                            >> ((3 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 2 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xff55ff00 as c_uint
                            >> ((7 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 2 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (2 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((2 as c_int * 2 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 2 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (5 as c_int) < 4 as c_int {
                        0xff0fff33 as c_uint
                            >> ((3 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 3 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xff55ff00 as c_uint
                            >> ((7 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 3 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (2 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((3 as c_int * 2 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 2 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (5 as c_int) < 4 as c_int {
                        0xff0fff33 as c_uint
                            >> ((3 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 4 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xff55ff00 as c_uint
                            >> ((7 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 4 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (2 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((4 as c_int * 2 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 2 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (5 as c_int) < 4 as c_int {
                        0xff0fff33 as c_uint
                            >> ((3 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 5 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xff55ff00 as c_uint
                            >> ((7 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 5 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (2 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((5 as c_int * 2 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 2 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (5 as c_int) < 4 as c_int {
                        0xff0fff33 as c_uint
                            >> ((3 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 6 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xff55ff00 as c_uint
                            >> ((7 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 6 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (2 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((6 as c_int * 2 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 2 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (5 as c_int) < 4 as c_int {
                        0xff0fff33 as c_uint
                            >> ((3 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 7 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xff55ff00 as c_uint
                            >> ((7 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 7 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (2 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((7 as c_int * 2 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 2 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_mul(
                    (if 2 as c_int == 1 as c_int {
                        0x1010101 as c_int
                    } else {
                        (if 2 as c_int == 2 as c_int {
                            0x10001 as c_int
                        } else {
                            1 as c_int
                        })
                    }) as c_uint,
                ),
            ],
            [
                (if (if (1 as c_int) < 4 as c_int {
                    0xff0fff33 as c_uint
                        >> ((3 as c_int - 1 as c_int)
                            * 8 as c_int
                            + (7 as c_int - 0 as c_int)
                            & 0x1f as c_int)
                } else {
                    0xff55ff00 as c_uint
                        >> ((7 as c_int - 1 as c_int)
                            * 8 as c_int
                            + (7 as c_int - 0 as c_int)
                            & 0x1f as c_int)
                }) & 1 as c_uint
                    != 0
                {
                    ((1 as c_uint)
                        << (4 as c_int & 0x1f as c_int))
                        .wrapping_sub(1 as c_uint)
                        << ((0 as c_int * 4 as c_int
                            ^ (if 1 as c_int != 0 {
                                8 as c_int - 4 as c_int
                            } else {
                                0 as c_int
                            }))
                            & 0x1f as c_int)
                } else {
                    0 as c_uint
                })
                .wrapping_add(
                    (if (if (1 as c_int) < 4 as c_int {
                        0xff0fff33 as c_uint
                            >> ((3 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 1 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xff55ff00 as c_uint
                            >> ((7 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 1 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (4 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((1 as c_int * 4 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 4 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (1 as c_int) < 4 as c_int {
                        0xff0fff33 as c_uint
                            >> ((3 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 2 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xff55ff00 as c_uint
                            >> ((7 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 2 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (4 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((2 as c_int * 4 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 4 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (1 as c_int) < 4 as c_int {
                        0xff0fff33 as c_uint
                            >> ((3 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 3 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xff55ff00 as c_uint
                            >> ((7 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 3 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (4 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((3 as c_int * 4 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 4 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (1 as c_int) < 4 as c_int {
                        0xff0fff33 as c_uint
                            >> ((3 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 4 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xff55ff00 as c_uint
                            >> ((7 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 4 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (4 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((4 as c_int * 4 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 4 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (1 as c_int) < 4 as c_int {
                        0xff0fff33 as c_uint
                            >> ((3 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 5 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xff55ff00 as c_uint
                            >> ((7 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 5 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (4 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((5 as c_int * 4 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 4 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (1 as c_int) < 4 as c_int {
                        0xff0fff33 as c_uint
                            >> ((3 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 6 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xff55ff00 as c_uint
                            >> ((7 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 6 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (4 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((6 as c_int * 4 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 4 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (1 as c_int) < 4 as c_int {
                        0xff0fff33 as c_uint
                            >> ((3 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 7 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xff55ff00 as c_uint
                            >> ((7 as c_int - 1 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 7 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (4 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((7 as c_int * 4 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 4 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_mul(
                    (if 4 as c_int == 1 as c_int {
                        0x1010101 as c_int
                    } else {
                        (if 4 as c_int == 2 as c_int {
                            0x10001 as c_int
                        } else {
                            1 as c_int
                        })
                    }) as c_uint,
                ),
                (if (if (3 as c_int) < 4 as c_int {
                    0xff0fff33 as c_uint
                        >> ((3 as c_int - 3 as c_int)
                            * 8 as c_int
                            + (7 as c_int - 0 as c_int)
                            & 0x1f as c_int)
                } else {
                    0xff55ff00 as c_uint
                        >> ((7 as c_int - 3 as c_int)
                            * 8 as c_int
                            + (7 as c_int - 0 as c_int)
                            & 0x1f as c_int)
                }) & 1 as c_uint
                    != 0
                {
                    ((1 as c_uint)
                        << (4 as c_int & 0x1f as c_int))
                        .wrapping_sub(1 as c_uint)
                        << ((0 as c_int * 4 as c_int
                            ^ (if 1 as c_int != 0 {
                                8 as c_int - 4 as c_int
                            } else {
                                0 as c_int
                            }))
                            & 0x1f as c_int)
                } else {
                    0 as c_uint
                })
                .wrapping_add(
                    (if (if (3 as c_int) < 4 as c_int {
                        0xff0fff33 as c_uint
                            >> ((3 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 1 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xff55ff00 as c_uint
                            >> ((7 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 1 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (4 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((1 as c_int * 4 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 4 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (3 as c_int) < 4 as c_int {
                        0xff0fff33 as c_uint
                            >> ((3 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 2 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xff55ff00 as c_uint
                            >> ((7 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 2 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (4 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((2 as c_int * 4 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 4 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (3 as c_int) < 4 as c_int {
                        0xff0fff33 as c_uint
                            >> ((3 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 3 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xff55ff00 as c_uint
                            >> ((7 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 3 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (4 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((3 as c_int * 4 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 4 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (3 as c_int) < 4 as c_int {
                        0xff0fff33 as c_uint
                            >> ((3 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 4 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xff55ff00 as c_uint
                            >> ((7 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 4 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (4 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((4 as c_int * 4 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 4 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (3 as c_int) < 4 as c_int {
                        0xff0fff33 as c_uint
                            >> ((3 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 5 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xff55ff00 as c_uint
                            >> ((7 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 5 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (4 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((5 as c_int * 4 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 4 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (3 as c_int) < 4 as c_int {
                        0xff0fff33 as c_uint
                            >> ((3 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 6 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xff55ff00 as c_uint
                            >> ((7 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 6 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (4 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((6 as c_int * 4 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 4 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (3 as c_int) < 4 as c_int {
                        0xff0fff33 as c_uint
                            >> ((3 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 7 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xff55ff00 as c_uint
                            >> ((7 as c_int - 3 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 7 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (4 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((7 as c_int * 4 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 4 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_mul(
                    (if 4 as c_int == 1 as c_int {
                        0x1010101 as c_int
                    } else {
                        (if 4 as c_int == 2 as c_int {
                            0x10001 as c_int
                        } else {
                            1 as c_int
                        })
                    }) as c_uint,
                ),
                (if (if (5 as c_int) < 4 as c_int {
                    0xff0fff33 as c_uint
                        >> ((3 as c_int - 5 as c_int)
                            * 8 as c_int
                            + (7 as c_int - 0 as c_int)
                            & 0x1f as c_int)
                } else {
                    0xff55ff00 as c_uint
                        >> ((7 as c_int - 5 as c_int)
                            * 8 as c_int
                            + (7 as c_int - 0 as c_int)
                            & 0x1f as c_int)
                }) & 1 as c_uint
                    != 0
                {
                    ((1 as c_uint)
                        << (4 as c_int & 0x1f as c_int))
                        .wrapping_sub(1 as c_uint)
                        << ((0 as c_int * 4 as c_int
                            ^ (if 1 as c_int != 0 {
                                8 as c_int - 4 as c_int
                            } else {
                                0 as c_int
                            }))
                            & 0x1f as c_int)
                } else {
                    0 as c_uint
                })
                .wrapping_add(
                    (if (if (5 as c_int) < 4 as c_int {
                        0xff0fff33 as c_uint
                            >> ((3 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 1 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xff55ff00 as c_uint
                            >> ((7 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 1 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (4 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((1 as c_int * 4 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 4 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (5 as c_int) < 4 as c_int {
                        0xff0fff33 as c_uint
                            >> ((3 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 2 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xff55ff00 as c_uint
                            >> ((7 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 2 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (4 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((2 as c_int * 4 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 4 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (5 as c_int) < 4 as c_int {
                        0xff0fff33 as c_uint
                            >> ((3 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 3 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xff55ff00 as c_uint
                            >> ((7 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 3 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (4 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((3 as c_int * 4 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 4 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (5 as c_int) < 4 as c_int {
                        0xff0fff33 as c_uint
                            >> ((3 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 4 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xff55ff00 as c_uint
                            >> ((7 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 4 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (4 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((4 as c_int * 4 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 4 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (5 as c_int) < 4 as c_int {
                        0xff0fff33 as c_uint
                            >> ((3 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 5 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xff55ff00 as c_uint
                            >> ((7 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 5 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (4 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((5 as c_int * 4 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 4 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (5 as c_int) < 4 as c_int {
                        0xff0fff33 as c_uint
                            >> ((3 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 6 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xff55ff00 as c_uint
                            >> ((7 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 6 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (4 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((6 as c_int * 4 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 4 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_add(
                    (if (if (5 as c_int) < 4 as c_int {
                        0xff0fff33 as c_uint
                            >> ((3 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 7 as c_int)
                                & 0x1f as c_int)
                    } else {
                        0xff55ff00 as c_uint
                            >> ((7 as c_int - 5 as c_int)
                                * 8 as c_int
                                + (7 as c_int - 7 as c_int)
                                & 0x1f as c_int)
                    }) & 1 as c_uint
                        != 0
                    {
                        ((1 as c_uint)
                            << (4 as c_int & 0x1f as c_int))
                            .wrapping_sub(1 as c_uint)
                            << ((7 as c_int * 4 as c_int
                                ^ (if 1 as c_int != 0 {
                                    8 as c_int - 4 as c_int
                                } else {
                                    0 as c_int
                                }))
                                & 0x1f as c_int)
                    } else {
                        0 as c_uint
                    }),
                )
                .wrapping_mul(
                    (if 4 as c_int == 1 as c_int {
                        0x1010101 as c_int
                    } else {
                        (if 4 as c_int == 2 as c_int {
                            0x10001 as c_int
                        } else {
                            1 as c_int
                        })
                    }) as c_uint,
                ),
            ],
        ],
    ];
} }
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
