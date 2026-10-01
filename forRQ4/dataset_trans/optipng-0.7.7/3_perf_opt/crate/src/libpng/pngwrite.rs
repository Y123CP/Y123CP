use core::ffi::*;
use crate::src::libpng::pngwutil::png_do_write_interlace;
pub use crate::src::ffi::*;
pub use crate::src::c_consts::*;
pub use crate::src::c_structs::*;
pub use crate::src::c_types::*;
pub use crate::src::zlib::deflate::internal_state;
extern "C" {
    fn png_write_sig(png_ptr: png_structrp);
    fn png_write_chunk(
        png_ptr: png_structrp,
        chunk_name: png_const_bytep,
        data: png_const_bytep,
        length: png_size_t,
    );
    fn png_set_interlace_handling(png_ptr: png_structrp) -> c_int;
    fn png_destroy_info_struct(png_ptr: png_const_structrp, info_ptr_ptr: png_infopp);
    fn png_set_write_fn(
        png_ptr: png_structrp,
        io_ptr: png_voidp,
        write_data_fn: png_rw_ptr,
        output_flush_fn: png_flush_ptr,
    );
    fn png_malloc(png_ptr: png_const_structrp, size: png_alloc_size_t) -> png_voidp;
    fn png_free(png_ptr: png_const_structrp, ptr: png_voidp);
    fn png_error(png_ptr: png_const_structrp, error_message: png_const_charp) -> !;
    fn png_warning(png_ptr: png_const_structrp, warning_message: png_const_charp);
    fn png_benign_error(png_ptr: png_const_structrp, warning_message: png_const_charp);
    fn png_handle_as_unknown(
        png_ptr: png_const_structrp,
        chunk_name: png_const_bytep,
    ) -> c_int;
    fn deflateEnd(strm: z_streamp) -> c_int;
    fn png_free_buffer_list(png_ptr: png_structrp, list: *mut png_compression_bufferp);
    fn png_create_png_struct(
        user_png_ver: png_const_charp,
        error_ptr: png_voidp,
        error_fn: png_error_ptr,
        warn_fn: png_error_ptr,
        mem_ptr: png_voidp,
        malloc_fn: png_malloc_ptr,
        free_fn: png_free_ptr,
    ) -> png_structp;
    fn png_destroy_png_struct(png_ptr: png_structrp);
    fn png_flush(png_ptr: png_structrp);
    fn png_write_IHDR(
        png_ptr: png_structrp,
        width: png_uint_32,
        height: png_uint_32,
        bit_depth: c_int,
        color_type: c_int,
        compression_method: c_int,
        filter_method: c_int,
        interlace_method: c_int,
    );
    fn png_write_PLTE(png_ptr: png_structrp, palette: png_const_colorp, num_pal: png_uint_32);
    fn png_compress_IDAT(
        png_ptr: png_structrp,
        row_data: png_const_bytep,
        row_data_length: png_alloc_size_t,
        flush: c_int,
    );
    fn png_write_IEND(png_ptr: png_structrp);
    fn png_write_sBIT(
        png_ptr: png_structrp,
        sbit: png_const_color_8p,
        color_type: c_int,
    );
    fn png_write_tRNS(
        png_ptr: png_structrp,
        trans: png_const_bytep,
        values: png_const_color_16p,
        number: c_int,
        color_type: c_int,
    );
    fn png_write_bKGD(
        png_ptr: png_structrp,
        values: png_const_color_16p,
        color_type: c_int,
    );
    fn png_write_hIST(
        png_ptr: png_structrp,
        hist: png_const_uint_16p,
        num_hist: c_int,
    );
    fn png_write_finish_row(png_ptr: png_structrp);
    fn png_write_start_row(png_ptr: png_structrp);
    fn png_write_find_filter(png_ptr: png_structrp, row_info: png_row_infop);
    fn png_do_check_palette_indexes(png_ptr: png_structrp, row_info: png_row_infop);
    fn png_app_warning(png_ptr: png_const_structrp, message: png_const_charp);
    fn png_app_error(png_ptr: png_const_structrp, message: png_const_charp);
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
pub type png_structpp = *mut *mut png_struct;
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
pub type png_infopp = *mut *mut png_info;
pub type png_structrp = *mut png_struct;
pub type png_const_structrp = *const png_struct;
pub type png_inforp = *mut png_info;
pub type png_const_inforp = *const png_info;

pub type png_malloc_ptr = Option<unsafe extern "C" fn(png_structp, png_alloc_size_t) -> png_voidp>;
pub type png_free_ptr = Option<unsafe extern "C" fn(png_structp, png_voidp) -> ()>;
pub type z_streamp = *mut z_stream;

pub const PNG_Z_DEFAULT_COMPRESSION: c_int = -(1 as c_int);

pub const PNG_COLOR_TYPE_PALETTE: c_int =
    PNG_COLOR_MASK_COLOR | PNG_COLOR_MASK_PALETTE;

pub const PNG_TRANSFORM_STRIP_FILLER: c_int = 0x800 as c_int;
pub const PNG_TRANSFORM_STRIP_FILLER_BEFORE: c_int = PNG_TRANSFORM_STRIP_FILLER;
pub const PNG_TRANSFORM_STRIP_FILLER_AFTER: c_int = 0x1000 as c_int;

pub const PNG_FILTER_VALUE_NONE: c_int = 0;
pub const PNG_FILTER_VALUE_SUB: c_int = 1;
pub const PNG_FILTER_VALUE_UP: c_int = 2;
pub const PNG_FILTER_VALUE_AVG: c_int = 3;
pub const PNG_FILTER_VALUE_PAETH: c_int = 4;

pub const PNG_WROTE_INFO_BEFORE_PLTE: c_uint = 0x400 as c_uint;

pub const Z_SYNC_FLUSH: c_int = 2 as c_int;
fn write_unknown_chunks(
    mut png_ptr: png_structrp,
    mut info_ptr: png_const_inforp,
    mut where_0: c_uint,
) { unsafe {
    if (*info_ptr).unknown_chunks_num != 0 as c_int {
        let mut up: png_const_unknown_chunkp = ::core::ptr::null::<png_unknown_chunk>();
        up = (*info_ptr).unknown_chunks as png_const_unknown_chunkp;
        while up
            < (*info_ptr)
                .unknown_chunks
                .offset((*info_ptr).unknown_chunks_num as isize)
                as png_const_unknown_chunkp
        {
            if (*up).location as c_uint & where_0 != 0 as c_uint {
                let mut keep: c_int =
                    png_handle_as_unknown(png_ptr, &raw const (*up).name as png_const_bytep);
                if keep != PNG_HANDLE_CHUNK_NEVER
                    && ((*up).name[3 as c_int as usize] as c_int
                        & 0x20 as c_int
                        != 0
                        || keep == PNG_HANDLE_CHUNK_ALWAYS
                        || keep == PNG_HANDLE_CHUNK_AS_DEFAULT
                            && (*png_ptr).unknown_default == PNG_HANDLE_CHUNK_ALWAYS)
                {
                    if (*up).size == 0 as png_size_t {
                        png_warning(
                            png_ptr,
                            b"Writing zero-length unknown chunk\0" as *const u8 as png_const_charp,
                        );
                    }
                    png_write_chunk(
                        png_ptr,
                        &raw const (*up).name as png_const_bytep,
                        (*up).data as png_const_bytep,
                        (*up).size,
                    );
                }
            }
            up = up.offset(1);
        }
    }
} }
#[inline]
pub fn png_write_info_before_PLTE(
    mut png_ptr: png_structrp,
    mut info_ptr: png_const_inforp,
) { unsafe {
    if png_ptr.is_null() || info_ptr.is_null() {
        return;
    }
    if (*png_ptr).mode as c_uint & PNG_WROTE_INFO_BEFORE_PLTE
        == 0 as c_uint
    {
        png_write_sig(png_ptr);
        png_write_IHDR(
            png_ptr,
            (*info_ptr).width,
            (*info_ptr).height,
            (*info_ptr).bit_depth as c_int,
            (*info_ptr).color_type as c_int,
            (*info_ptr).compression_type as c_int,
            (*info_ptr).filter_type as c_int,
            (*info_ptr).interlace_type as c_int,
        );
        if (*info_ptr).valid as c_uint & PNG_INFO_sBIT != 0 as c_uint {
            png_write_sBIT(
                png_ptr,
                &raw const (*info_ptr).sig_bit,
                (*info_ptr).color_type as c_int,
            );
        }
        write_unknown_chunks(png_ptr, info_ptr, PNG_HAVE_IHDR as c_uint);
        (*png_ptr).mode |= PNG_WROTE_INFO_BEFORE_PLTE;
    }
} }
#[inline]
pub fn png_write_info(mut png_ptr: png_structrp, mut info_ptr: png_const_inforp) { unsafe {
    if png_ptr.is_null() || info_ptr.is_null() {
        return;
    }
    png_write_info_before_PLTE(png_ptr, info_ptr);
    if (*info_ptr).valid as c_uint & PNG_INFO_PLTE != 0 as c_uint {
        png_write_PLTE(
            png_ptr,
            (*info_ptr).palette as png_const_colorp,
            (*info_ptr).num_palette as png_uint_32,
        );
    } else if (*info_ptr).color_type as c_int == PNG_COLOR_TYPE_PALETTE {
        png_error(
            png_ptr,
            b"Valid palette required for paletted images\0" as *const u8 as png_const_charp,
        );
    }
    if (*info_ptr).valid as c_uint & PNG_INFO_tRNS != 0 as c_uint {
        png_write_tRNS(
            png_ptr,
            (*info_ptr).trans_alpha as png_const_bytep,
            &raw const (*info_ptr).trans_color,
            (*info_ptr).num_trans as c_int,
            (*info_ptr).color_type as c_int,
        );
    }
    if (*info_ptr).valid as c_uint & PNG_INFO_bKGD != 0 as c_uint {
        png_write_bKGD(
            png_ptr,
            &raw const (*info_ptr).background,
            (*info_ptr).color_type as c_int,
        );
    }
    if (*info_ptr).valid as c_uint & PNG_INFO_hIST != 0 as c_uint {
        png_write_hIST(
            png_ptr,
            (*info_ptr).hist as png_const_uint_16p,
            (*info_ptr).num_palette as c_int,
        );
    }
    write_unknown_chunks(png_ptr, info_ptr, PNG_HAVE_PLTE as c_uint);
} }
#[inline]
pub fn png_write_end(mut png_ptr: png_structrp, mut info_ptr: png_inforp) { unsafe {
    if png_ptr.is_null() {
        return;
    }
    if (*png_ptr).mode as c_uint & PNG_HAVE_IDAT == 0 as c_uint {
        png_error(
            png_ptr,
            b"No IDATs written into file\0" as *const u8 as png_const_charp,
        );
    }
    if (*png_ptr).num_palette_max > (*png_ptr).num_palette as c_int {
        png_benign_error(
            png_ptr,
            b"Wrote palette index exceeding num_palette\0" as *const u8 as png_const_charp,
        );
    }
    if !info_ptr.is_null() {
        write_unknown_chunks(png_ptr, info_ptr, PNG_AFTER_IDAT as c_uint);
    }
    (*png_ptr).mode |= PNG_AFTER_IDAT as c_uint;
    png_write_IEND(png_ptr);
} }
#[inline]
pub fn png_create_write_struct(
    mut user_png_ver: png_const_charp,
    mut error_ptr: png_voidp,
    mut error_fn: png_error_ptr,
    mut warn_fn: png_error_ptr,
) -> png_structp { unsafe {
    let mut png_ptr: png_structrp =
        png_create_png_struct(user_png_ver, error_ptr, error_fn, warn_fn, NULL, None, None)
            as png_structrp;
    if !png_ptr.is_null() {
        (*png_ptr).zbuffer_size = PNG_ZBUF_SIZE as uInt;
        (*png_ptr).zlib_strategy = PNG_Z_DEFAULT_STRATEGY;
        (*png_ptr).zlib_level = PNG_Z_DEFAULT_COMPRESSION;
        (*png_ptr).zlib_mem_level = 8 as c_int;
        (*png_ptr).zlib_window_bits = 15 as c_int;
        (*png_ptr).zlib_method = 8 as c_int;
        (*png_ptr).flags |= PNG_FLAG_APP_WARNINGS_WARN;
        png_set_write_fn(png_ptr, NULL, None, None);
    }
    return png_ptr as png_structp;
} }
#[inline]
pub fn png_write_rows(
    mut png_ptr: png_structrp,
    mut row: png_bytepp,
    mut num_rows: png_uint_32,
) { unsafe {
    let mut i: png_uint_32 = 0;
    let mut rp: png_bytepp = ::core::ptr::null_mut::<*mut png_byte>();
    if png_ptr.is_null() {
        return;
    }
    i = 0 as png_uint_32;
    rp = row;
    while i < num_rows {
        png_write_row(png_ptr, *rp as png_const_bytep);
        i = i.wrapping_add(1);
        rp = rp.offset(1);
    }
} }
#[inline]
pub fn png_write_image(mut png_ptr: png_structrp, mut image: png_bytepp) { unsafe {
    let mut i: png_uint_32 = 0;
    let mut pass: c_int = 0;
    let mut num_pass: c_int = 0;
    let mut rp: png_bytepp = ::core::ptr::null_mut::<*mut png_byte>();
    if png_ptr.is_null() {
        return;
    }
    num_pass = png_set_interlace_handling(png_ptr);
    pass = 0 as c_int;
    while pass < num_pass {
        i = 0 as png_uint_32;
        rp = image;
        while i < (*png_ptr).height {
            png_write_row(png_ptr, *rp as png_const_bytep);
            i = i.wrapping_add(1);
            rp = rp.offset(1);
        }
        pass += 1;
    }
} }
#[inline]
pub fn png_write_row(mut png_ptr: png_structrp, mut row: png_const_bytep) { unsafe {
    let mut row_info: png_row_info = png_row_info {
        width: 0,
        rowbytes: 0,
        color_type: 0,
        bit_depth: 0,
        channels: 0,
        pixel_depth: 0,
    };
    if png_ptr.is_null() {
        return;
    }
    if (*png_ptr).row_number == 0 as c_uint
        && (*png_ptr).pass as c_int == 0 as c_int
    {
        if (*png_ptr).mode as c_uint & PNG_WROTE_INFO_BEFORE_PLTE
            == 0 as c_uint
        {
            png_error(
                png_ptr,
                b"png_write_info was never called before png_write_row\0" as *const u8
                    as png_const_charp,
            );
        }
        png_write_start_row(png_ptr);
    }
    if (*png_ptr).interlaced as c_int != 0 as c_int
        && (*png_ptr).transformations as c_uint & PNG_INTERLACE
            != 0 as c_uint
    {
        match (*png_ptr).pass as c_int {
            0 => {
                if (*png_ptr).row_number as c_uint & 0x7 as c_uint
                    != 0 as c_uint
                {
                    png_write_finish_row(png_ptr);
                    return;
                }
            }
            1 => {
                if (*png_ptr).row_number as c_uint & 0x7 as c_uint
                    != 0 as c_uint
                    || (*png_ptr).width < 5 as c_uint
                {
                    png_write_finish_row(png_ptr);
                    return;
                }
            }
            2 => {
                if (*png_ptr).row_number as c_uint & 0x7 as c_uint
                    != 4 as c_uint
                {
                    png_write_finish_row(png_ptr);
                    return;
                }
            }
            3 => {
                if (*png_ptr).row_number as c_uint & 0x3 as c_uint
                    != 0 as c_uint
                    || (*png_ptr).width < 3 as c_uint
                {
                    png_write_finish_row(png_ptr);
                    return;
                }
            }
            4 => {
                if (*png_ptr).row_number as c_uint & 0x3 as c_uint
                    != 2 as c_uint
                {
                    png_write_finish_row(png_ptr);
                    return;
                }
            }
            5 => {
                if (*png_ptr).row_number as c_uint & 0x1 as c_uint
                    != 0 as c_uint
                    || (*png_ptr).width < 2 as c_uint
                {
                    png_write_finish_row(png_ptr);
                    return;
                }
            }
            6 => {
                if (*png_ptr).row_number as c_uint & 0x1 as c_uint
                    == 0 as c_uint
                {
                    png_write_finish_row(png_ptr);
                    return;
                }
            }
            _ => {}
        }
    }
    row_info.color_type = (*png_ptr).color_type;
    row_info.width = (*png_ptr).usr_width;
    row_info.channels = (*png_ptr).usr_channels;
    row_info.bit_depth = (*png_ptr).usr_bit_depth;
    row_info.pixel_depth = (row_info.bit_depth as c_int
        * row_info.channels as c_int) as png_byte;
    row_info.rowbytes = if row_info.pixel_depth as c_int >= 8 as c_int {
        (row_info.width as png_size_t)
            .wrapping_mul(row_info.pixel_depth as png_size_t >> 3 as c_int)
    } else {
        (row_info.width as png_size_t)
            .wrapping_mul(row_info.pixel_depth as png_size_t)
            .wrapping_add(7 as png_size_t)
            >> 3 as c_int
    };
    memcpy(
        (*png_ptr).row_buf.offset(1 as c_int as isize) as *mut c_void,
        row as *const c_void,
        row_info.rowbytes as size_t,
    );
    if (*png_ptr).interlaced as c_int != 0
        && ((*png_ptr).pass as c_int) < 6 as c_int
        && (*png_ptr).transformations as c_uint & PNG_INTERLACE
            != 0 as c_uint
    {
        png_do_write_interlace(
            &raw mut row_info,
            (*png_ptr).row_buf.offset(1 as c_int as isize),
            (*png_ptr).pass as c_int,
        );
        if row_info.width == 0 as c_uint {
            png_write_finish_row(png_ptr);
            return;
        }
    }
    if row_info.pixel_depth as c_int != (*png_ptr).pixel_depth as c_int
        || row_info.pixel_depth as c_int
            != (*png_ptr).transformed_pixel_depth as c_int
    {
        png_error(
            png_ptr,
            b"internal write transform logic error\0" as *const u8 as png_const_charp,
        );
    }
    if row_info.color_type as c_int == PNG_COLOR_TYPE_PALETTE
        && (*png_ptr).num_palette_max >= 0 as c_int
    {
        png_do_check_palette_indexes(png_ptr, &raw mut row_info);
    }
    png_write_find_filter(png_ptr, &raw mut row_info);
    if (*png_ptr).write_row_fn.is_some() {
        Some((*png_ptr).write_row_fn.expect("non-null function pointer"))
            .expect("non-null function pointer")(
            png_ptr as png_structp,
            (*png_ptr).row_number,
            (*png_ptr).pass as c_int,
        );
    }
} }
#[inline]
pub fn png_set_flush(mut png_ptr: png_structrp, mut nrows: c_int) { unsafe {
    if png_ptr.is_null() {
        return;
    }
    (*png_ptr).flush_dist = (if nrows < 0 as c_int {
        0 as c_uint
    } else {
        nrows as c_uint
    }) as png_uint_32;
} }
#[no_mangle]
pub extern "C" fn png_write_flush(mut png_ptr: png_structrp) { unsafe {
    if png_ptr.is_null() {
        return;
    }
    if (*png_ptr).row_number >= (*png_ptr).num_rows {
        return;
    }
    png_compress_IDAT(
        png_ptr,
        ::core::ptr::null::<png_byte>(),
        0 as png_alloc_size_t,
        Z_SYNC_FLUSH,
    );
    (*png_ptr).flush_rows = 0 as png_uint_32;
    png_flush(png_ptr);
} }
fn png_write_destroy(mut png_ptr: png_structrp) { unsafe {
    if (*png_ptr).flags as c_uint & PNG_FLAG_ZSTREAM_INITIALIZED
        != 0 as c_uint
    {
        deflateEnd(&raw mut (*png_ptr).zstream);
    }
    png_free_buffer_list(png_ptr, &raw mut (*png_ptr).zbuffer_list);
    png_free(png_ptr, (*png_ptr).row_buf as png_voidp);
    (*png_ptr).row_buf = ::core::ptr::null_mut::<png_byte>();
    png_free(png_ptr, (*png_ptr).prev_row as png_voidp);
    png_free(png_ptr, (*png_ptr).try_row as png_voidp);
    png_free(png_ptr, (*png_ptr).tst_row as png_voidp);
    (*png_ptr).prev_row = ::core::ptr::null_mut::<png_byte>();
    (*png_ptr).try_row = ::core::ptr::null_mut::<png_byte>();
    (*png_ptr).tst_row = ::core::ptr::null_mut::<png_byte>();
    png_free(png_ptr, (*png_ptr).chunk_list as png_voidp);
    (*png_ptr).chunk_list = ::core::ptr::null_mut::<png_byte>();
} }
#[inline]
pub fn png_destroy_write_struct(
    mut png_ptr_ptr: png_structpp,
    mut info_ptr_ptr: png_infopp,
) { unsafe {
    if !png_ptr_ptr.is_null() {
        let mut png_ptr: png_structrp = *png_ptr_ptr;
        if !png_ptr.is_null() {
            png_destroy_info_struct(png_ptr, info_ptr_ptr);
            *png_ptr_ptr = ::core::ptr::null_mut::<png_struct>();
            png_write_destroy(png_ptr);
            png_destroy_png_struct(png_ptr);
        }
    }
} }
#[inline]
pub fn png_set_filter(
    mut png_ptr: png_structrp,
    mut method: c_int,
    mut filters: c_int,
) { unsafe {
    if png_ptr.is_null() {
        return;
    }
    if method == PNG_FILTER_TYPE_BASE {
        let mut current_block_9: u64;
        match filters & (PNG_ALL_FILTERS | 0x7 as c_int) {
            5 | 6 | 7 => {
                png_app_error(
                    png_ptr,
                    b"Unknown row filter for method 0\0" as *const u8 as png_const_charp,
                );
                current_block_9 = 6563404168892233726;
            }
            PNG_FILTER_VALUE_NONE => {
                current_block_9 = 6563404168892233726;
            }
            PNG_FILTER_VALUE_SUB => {
                (*png_ptr).do_filter = PNG_FILTER_SUB as png_byte;
                current_block_9 = 10599921512955367680;
            }
            PNG_FILTER_VALUE_UP => {
                (*png_ptr).do_filter = PNG_FILTER_UP as png_byte;
                current_block_9 = 10599921512955367680;
            }
            PNG_FILTER_VALUE_AVG => {
                (*png_ptr).do_filter = PNG_FILTER_AVG as png_byte;
                current_block_9 = 10599921512955367680;
            }
            PNG_FILTER_VALUE_PAETH => {
                (*png_ptr).do_filter = PNG_FILTER_PAETH as png_byte;
                current_block_9 = 10599921512955367680;
            }
            _ => {
                (*png_ptr).do_filter = filters as png_byte;
                current_block_9 = 10599921512955367680;
            }
        }
        match current_block_9 {
            6563404168892233726 => {
                (*png_ptr).do_filter = PNG_FILTER_NONE as png_byte;
            }
            _ => {}
        }
        if !(*png_ptr).row_buf.is_null() {
            let mut num_filters: c_int = 0;
            let mut buf_size: png_alloc_size_t = 0;
            if (*png_ptr).height == 1 as c_uint {
                filters &= !(PNG_FILTER_UP | PNG_FILTER_AVG | PNG_FILTER_PAETH);
            }
            if (*png_ptr).width == 1 as c_uint {
                filters &= !(PNG_FILTER_SUB | PNG_FILTER_AVG | PNG_FILTER_PAETH);
            }
            if filters & (PNG_FILTER_UP | PNG_FILTER_AVG | PNG_FILTER_PAETH)
                != 0 as c_int
                && (*png_ptr).prev_row.is_null()
            {
                png_app_warning(
                    png_ptr,
                    b"png_set_filter: UP/AVG/PAETH cannot be added after start\0" as *const u8
                        as png_const_charp,
                );
                filters &= !(PNG_FILTER_UP | PNG_FILTER_AVG | PNG_FILTER_PAETH);
            }
            num_filters = 0 as c_int;
            if filters & PNG_FILTER_SUB != 0 {
                num_filters += 1;
            }
            if filters & PNG_FILTER_UP != 0 {
                num_filters += 1;
            }
            if filters & PNG_FILTER_AVG != 0 {
                num_filters += 1;
            }
            if filters & PNG_FILTER_PAETH != 0 {
                num_filters += 1;
            }
            buf_size = (if (*png_ptr).usr_channels as c_int
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
            .wrapping_add(1 as png_size_t) as png_alloc_size_t;
            if (*png_ptr).try_row.is_null() {
                (*png_ptr).try_row = png_malloc(png_ptr, buf_size) as png_bytep;
            }
            if num_filters > 1 as c_int {
                if (*png_ptr).tst_row.is_null() {
                    (*png_ptr).tst_row = png_malloc(png_ptr, buf_size) as png_bytep;
                }
            }
        }
        (*png_ptr).do_filter = filters as png_byte;
    } else {
        png_error(
            png_ptr,
            b"Unknown custom filter method\0" as *const u8 as png_const_charp,
        );
    };
} }
#[inline]
pub fn png_set_compression_level(
    mut png_ptr: png_structrp,
    mut level: c_int,
) { unsafe {
    if png_ptr.is_null() {
        return;
    }
    (*png_ptr).zlib_level = level;
} }
#[inline]
pub fn png_set_compression_mem_level(
    mut png_ptr: png_structrp,
    mut mem_level: c_int,
) { unsafe {
    if png_ptr.is_null() {
        return;
    }
    (*png_ptr).zlib_mem_level = mem_level;
} }
#[inline]
pub fn png_set_compression_strategy(
    mut png_ptr: png_structrp,
    mut strategy: c_int,
) { unsafe {
    if png_ptr.is_null() {
        return;
    }
    (*png_ptr).flags |= PNG_FLAG_ZLIB_CUSTOM_STRATEGY;
    (*png_ptr).zlib_strategy = strategy;
} }
#[inline]
pub fn png_set_compression_window_bits(
    mut png_ptr: png_structrp,
    mut window_bits: c_int,
) { unsafe {
    if png_ptr.is_null() {
        return;
    }
    if window_bits > 15 as c_int {
        png_warning(
            png_ptr,
            b"Only compression windows <= 32k supported by PNG\0" as *const u8 as png_const_charp,
        );
        window_bits = 15 as c_int;
    } else if window_bits < 8 as c_int {
        png_warning(
            png_ptr,
            b"Only compression windows >= 256 supported by PNG\0" as *const u8 as png_const_charp,
        );
        window_bits = 8 as c_int;
    }
    (*png_ptr).zlib_window_bits = window_bits;
} }
#[inline]
pub fn png_set_compression_method(
    mut png_ptr: png_structrp,
    mut method: c_int,
) { unsafe {
    if png_ptr.is_null() {
        return;
    }
    if method != 8 as c_int {
        png_warning(
            png_ptr,
            b"Only compression method 8 is supported by PNG\0" as *const u8 as png_const_charp,
        );
    }
    (*png_ptr).zlib_method = method;
} }
#[inline]
pub fn png_set_write_status_fn(
    mut png_ptr: png_structrp,
    mut write_row_fn: png_write_status_ptr,
) { unsafe {
    if png_ptr.is_null() {
        return;
    }
    (*png_ptr).write_row_fn = write_row_fn;
} }
#[inline]
pub fn png_write_png(
    mut png_ptr: png_structrp,
    mut info_ptr: png_inforp,
    mut transforms: c_int,
    mut params: voidp,
) { unsafe {
    if png_ptr.is_null() || info_ptr.is_null() {
        return;
    }
    if (*info_ptr).valid as c_uint & PNG_INFO_IDAT == 0 as c_uint {
        png_app_error(
            png_ptr,
            b"no rows for png_write_image to write\0" as *const u8 as png_const_charp,
        );
        return;
    }
    png_write_info(png_ptr, info_ptr);
    if transforms & PNG_TRANSFORM_INVERT_MONO != 0 as c_int {
        png_app_error(
            png_ptr,
            b"PNG_TRANSFORM_INVERT_MONO not supported\0" as *const u8 as png_const_charp,
        );
    }
    if transforms & PNG_TRANSFORM_SHIFT != 0 as c_int {
        png_app_error(
            png_ptr,
            b"PNG_TRANSFORM_SHIFT not supported\0" as *const u8 as png_const_charp,
        );
    }
    if transforms & PNG_TRANSFORM_PACKING != 0 as c_int {
        png_app_error(
            png_ptr,
            b"PNG_TRANSFORM_PACKING not supported\0" as *const u8 as png_const_charp,
        );
    }
    if transforms & PNG_TRANSFORM_SWAP_ALPHA != 0 as c_int {
        png_app_error(
            png_ptr,
            b"PNG_TRANSFORM_SWAP_ALPHA not supported\0" as *const u8 as png_const_charp,
        );
    }
    if transforms & (PNG_TRANSFORM_STRIP_FILLER_AFTER | PNG_TRANSFORM_STRIP_FILLER_BEFORE)
        != 0 as c_int
    {
        png_app_error(
            png_ptr,
            b"PNG_TRANSFORM_STRIP_FILLER not supported\0" as *const u8 as png_const_charp,
        );
    }
    if transforms & PNG_TRANSFORM_BGR != 0 as c_int {
        png_app_error(
            png_ptr,
            b"PNG_TRANSFORM_BGR not supported\0" as *const u8 as png_const_charp,
        );
    }
    if transforms & PNG_TRANSFORM_SWAP_ENDIAN != 0 as c_int {
        png_app_error(
            png_ptr,
            b"PNG_TRANSFORM_SWAP_ENDIAN not supported\0" as *const u8 as png_const_charp,
        );
    }
    if transforms & PNG_TRANSFORM_PACKSWAP != 0 as c_int {
        png_app_error(
            png_ptr,
            b"PNG_TRANSFORM_PACKSWAP not supported\0" as *const u8 as png_const_charp,
        );
    }
    if transforms & PNG_TRANSFORM_INVERT_ALPHA != 0 as c_int {
        png_app_error(
            png_ptr,
            b"PNG_TRANSFORM_INVERT_ALPHA not supported\0" as *const u8 as png_const_charp,
        );
    }
    png_write_image(png_ptr, (*info_ptr).row_pointers);
    png_write_end(png_ptr, info_ptr);
} }
