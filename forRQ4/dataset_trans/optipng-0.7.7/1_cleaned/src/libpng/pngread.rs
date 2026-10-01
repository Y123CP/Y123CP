use core::ffi::*;
pub use crate::src::ffi::*;
pub use crate::src::c_consts::*;
pub use crate::src::c_structs::*;
pub use crate::src::c_types::*;
extern "C" {
    pub type internal_state;
    fn png_set_interlace_handling(png_ptr: png_structrp) -> c_int;
    fn png_destroy_info_struct(png_ptr: png_const_structrp, info_ptr_ptr: png_infopp);
    fn png_set_read_fn(png_ptr: png_structrp, io_ptr: png_voidp, read_data_fn: png_rw_ptr);
    fn png_malloc(png_ptr: png_const_structrp, size: png_alloc_size_t) -> png_voidp;
    fn png_free(png_ptr: png_const_structrp, ptr: png_voidp);
    fn png_free_data(
        png_ptr: png_const_structrp,
        info_ptr: png_inforp,
        free_me: png_uint_32,
        num: c_int,
    );
    fn png_error(png_ptr: png_const_structrp, error_message: png_const_charp) -> !;
    fn png_chunk_error(png_ptr: png_const_structrp, error_message: png_const_charp) -> !;
    fn png_warning(png_ptr: png_const_structrp, warning_message: png_const_charp);
    fn png_benign_error(png_ptr: png_const_structrp, warning_message: png_const_charp);
    fn png_chunk_benign_error(png_ptr: png_const_structrp, warning_message: png_const_charp);
    fn inflateEnd(strm: z_streamp) -> c_int;
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
    fn png_zfree(png_ptr: voidpf, ptr: voidpf);
    fn png_read_sig(png_ptr: png_structrp, info_ptr: png_inforp);
    fn png_read_chunk_header(png_ptr: png_structrp) -> png_uint_32;
    fn png_crc_finish(png_ptr: png_structrp, skip: png_uint_32) -> c_int;
    fn png_combine_row(png_ptr: png_const_structrp, row: png_bytep, display: c_int);
    fn png_do_read_interlace(
        row_info: png_row_infop,
        row: png_bytep,
        pass: c_int,
        transformations: png_uint_32,
    );
    fn png_read_filter_row(
        pp: png_structrp,
        row_info: png_row_infop,
        row: png_bytep,
        prev_row: png_const_bytep,
        filter: c_int,
    );
    fn png_read_IDAT_data(png_ptr: png_structrp, output: png_bytep, avail_out: png_alloc_size_t);
    fn png_read_finish_IDAT(png_ptr: png_structrp);
    fn png_read_finish_row(png_ptr: png_structrp);
    fn png_read_start_row(png_ptr: png_structrp);
    fn png_read_transform_info(png_ptr: png_structrp, info_ptr: png_inforp);
    fn png_handle_IHDR(png_ptr: png_structrp, info_ptr: png_inforp, length: png_uint_32);
    fn png_handle_PLTE(png_ptr: png_structrp, info_ptr: png_inforp, length: png_uint_32);
    fn png_handle_IEND(png_ptr: png_structrp, info_ptr: png_inforp, length: png_uint_32);
    fn png_handle_bKGD(png_ptr: png_structrp, info_ptr: png_inforp, length: png_uint_32);
    fn png_handle_hIST(png_ptr: png_structrp, info_ptr: png_inforp, length: png_uint_32);
    fn png_handle_sBIT(png_ptr: png_structrp, info_ptr: png_inforp, length: png_uint_32);
    fn png_handle_tRNS(png_ptr: png_structrp, info_ptr: png_inforp, length: png_uint_32);
    fn png_handle_unknown(
        png_ptr: png_structrp,
        info_ptr: png_inforp,
        length: png_uint_32,
        keep: c_int,
    );
    fn png_chunk_unknown_handling(
        png_ptr: png_const_structrp,
        chunk_name: png_uint_32,
    ) -> c_int;
    fn png_do_read_transformations(png_ptr: png_structrp, row_info: png_row_infop);
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
pub type png_malloc_ptr = Option<unsafe extern "C" fn(png_structp, png_alloc_size_t) -> png_voidp>;
pub type png_free_ptr = Option<unsafe extern "C" fn(png_structp, png_voidp) -> ()>;
pub type z_streamp = *mut z_stream;

pub const PNG_IDAT_READ_SIZE: c_int = PNG_ZBUF_SIZE;

pub const PNG_COLOR_TYPE_PALETTE: c_int =
    PNG_COLOR_MASK_COLOR | PNG_COLOR_MASK_PALETTE;

pub const PNG_TRANSFORM_STRIP_16: c_int = 0x1 as c_int;
pub const PNG_TRANSFORM_STRIP_ALPHA: c_int = 0x2 as c_int;

pub const PNG_TRANSFORM_EXPAND: c_int = 0x10 as c_int;

pub const PNG_TRANSFORM_GRAY_TO_RGB: c_int = 0x2000 as c_int;
pub const PNG_TRANSFORM_EXPAND_16: c_int = 0x4000 as c_int;
pub const PNG_TRANSFORM_SCALE_16: c_int = 0x8000 as c_int;
pub const PNG_FILTER_VALUE_NONE: c_int = 0 as c_int;

pub const PNG_HAVE_CHUNK_AFTER_IDAT: c_uint = 0x2000 as c_uint;

#[no_mangle]
pub unsafe extern "C" fn png_create_read_struct(
    mut user_png_ver: png_const_charp,
    mut error_ptr: png_voidp,
    mut error_fn: png_error_ptr,
    mut warn_fn: png_error_ptr,
) -> png_structp {
    let mut png_ptr: png_structp =
        png_create_png_struct(user_png_ver, error_ptr, error_fn, warn_fn, NULL, None, None);
    if !png_ptr.is_null() {
        (*png_ptr).mode = PNG_IS_READ_STRUCT as png_uint_32;
        (*png_ptr).IDAT_read_size = PNG_IDAT_READ_SIZE as uInt;
        (*png_ptr).flags |= PNG_FLAG_BENIGN_ERRORS_WARN;
        (*png_ptr).flags |= PNG_FLAG_APP_WARNINGS_WARN;
        png_set_read_fn(png_ptr as png_structrp, NULL, None);
    }
    return png_ptr;
}
#[no_mangle]
pub unsafe extern "C" fn png_read_info(mut png_ptr: png_structrp, mut info_ptr: png_inforp) {
    let mut keep: c_int = 0;
    if png_ptr.is_null() || info_ptr.is_null() {
        return;
    }
    png_read_sig(png_ptr, info_ptr);
    loop {
        let mut length: png_uint_32 = png_read_chunk_header(png_ptr);
        let mut chunk_name: png_uint_32 = (*png_ptr).chunk_name;
        if chunk_name == png_IDAT {
            if (*png_ptr).mode as c_uint & PNG_HAVE_IHDR as c_uint
                == 0 as c_uint
            {
                png_chunk_error(
                    png_ptr,
                    b"Missing IHDR before IDAT\0" as *const u8 as png_const_charp,
                );
            } else if (*png_ptr).color_type as c_int == PNG_COLOR_TYPE_PALETTE
                && (*png_ptr).mode as c_uint & PNG_HAVE_PLTE as c_uint
                    == 0 as c_uint
            {
                png_chunk_error(
                    png_ptr,
                    b"Missing PLTE before IDAT\0" as *const u8 as png_const_charp,
                );
            } else if (*png_ptr).mode as c_uint & PNG_AFTER_IDAT as c_uint
                != 0 as c_uint
            {
                png_chunk_benign_error(
                    png_ptr,
                    b"Too many IDATs found\0" as *const u8 as png_const_charp,
                );
            }
            (*png_ptr).mode |= PNG_HAVE_IDAT;
        } else if (*png_ptr).mode as c_uint & PNG_HAVE_IDAT != 0 as c_uint
        {
            (*png_ptr).mode |= PNG_HAVE_CHUNK_AFTER_IDAT;
            (*png_ptr).mode |= PNG_AFTER_IDAT as c_uint;
        }
        if chunk_name == png_IHDR {
            png_handle_IHDR(png_ptr, info_ptr, length);
        } else if chunk_name == png_IEND {
            png_handle_IEND(png_ptr, info_ptr, length);
        } else {
            keep = png_chunk_unknown_handling(png_ptr, chunk_name);
            if keep != 0 as c_int {
                png_handle_unknown(png_ptr, info_ptr, length, keep);
                if chunk_name == png_PLTE {
                    (*png_ptr).mode |= PNG_HAVE_PLTE as c_uint;
                } else {
                    if !(chunk_name == png_IDAT) {
                        continue;
                    }
                    (*png_ptr).idat_size = 0 as png_uint_32;
                    break;
                }
            } else if chunk_name == png_PLTE {
                png_handle_PLTE(png_ptr, info_ptr, length);
            } else if chunk_name == png_IDAT {
                (*png_ptr).idat_size = length;
                break;
            } else if chunk_name == png_bKGD {
                png_handle_bKGD(png_ptr, info_ptr, length);
            } else if chunk_name == png_hIST {
                png_handle_hIST(png_ptr, info_ptr, length);
            } else if chunk_name == png_sBIT {
                png_handle_sBIT(png_ptr, info_ptr, length);
            } else if chunk_name == png_tRNS {
                png_handle_tRNS(png_ptr, info_ptr, length);
            } else {
                png_handle_unknown(png_ptr, info_ptr, length, PNG_HANDLE_CHUNK_AS_DEFAULT);
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn png_read_update_info(mut png_ptr: png_structrp, mut info_ptr: png_inforp) {
    if !png_ptr.is_null() {
        if (*png_ptr).flags as c_uint & PNG_FLAG_ROW_INIT == 0 as c_uint {
            png_read_start_row(png_ptr);
            png_read_transform_info(png_ptr, info_ptr);
        } else {
            png_app_error(
                png_ptr,
                b"png_read_update_info/png_start_read_image: duplicate call\0" as *const u8
                    as png_const_charp,
            );
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn png_start_read_image(mut png_ptr: png_structrp) {
    if !png_ptr.is_null() {
        if (*png_ptr).flags as c_uint & PNG_FLAG_ROW_INIT == 0 as c_uint {
            png_read_start_row(png_ptr);
        } else {
            png_app_error(
                png_ptr,
                b"png_start_read_image/png_read_update_info: duplicate call\0" as *const u8
                    as png_const_charp,
            );
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn png_read_row(
    mut png_ptr: png_structrp,
    mut row: png_bytep,
    mut dsp_row: png_bytep,
) {
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
    if (*png_ptr).flags as c_uint & PNG_FLAG_ROW_INIT == 0 as c_uint {
        png_read_start_row(png_ptr);
    }
    row_info.width = (*png_ptr).iwidth;
    row_info.color_type = (*png_ptr).color_type;
    row_info.bit_depth = (*png_ptr).bit_depth;
    row_info.channels = (*png_ptr).channels;
    row_info.pixel_depth = (*png_ptr).pixel_depth;
    row_info.rowbytes = if row_info.pixel_depth as c_int >= 8 as c_int {
        (row_info.width as png_size_t)
            .wrapping_mul(row_info.pixel_depth as png_size_t >> 3 as c_int)
    } else {
        (row_info.width as png_size_t)
            .wrapping_mul(row_info.pixel_depth as png_size_t)
            .wrapping_add(7 as png_size_t)
            >> 3 as c_int
    };
    (*png_ptr).row_number == 0 as c_uint
        && (*png_ptr).pass as c_int == 0 as c_int;
    if (*png_ptr).interlaced as c_int != 0 as c_int
        && (*png_ptr).transformations as c_uint & PNG_INTERLACE
            != 0 as c_uint
    {
        match (*png_ptr).pass as c_int {
            0 => {
                if (*png_ptr).row_number as c_uint & 0x7 as c_uint != 0 {
                    if !dsp_row.is_null() {
                        png_combine_row(png_ptr, dsp_row, 1 as c_int);
                    }
                    png_read_finish_row(png_ptr);
                    return;
                }
            }
            1 => {
                if (*png_ptr).row_number as c_uint & 0x7 as c_uint != 0
                    || (*png_ptr).width < 5 as c_uint
                {
                    if !dsp_row.is_null() {
                        png_combine_row(png_ptr, dsp_row, 1 as c_int);
                    }
                    png_read_finish_row(png_ptr);
                    return;
                }
            }
            2 => {
                if (*png_ptr).row_number as c_uint & 0x7 as c_uint
                    != 4 as c_uint
                {
                    if !dsp_row.is_null()
                        && (*png_ptr).row_number as c_uint & 4 as c_uint
                            != 0
                    {
                        png_combine_row(png_ptr, dsp_row, 1 as c_int);
                    }
                    png_read_finish_row(png_ptr);
                    return;
                }
            }
            3 => {
                if (*png_ptr).row_number as c_uint & 3 as c_uint != 0
                    || (*png_ptr).width < 3 as c_uint
                {
                    if !dsp_row.is_null() {
                        png_combine_row(png_ptr, dsp_row, 1 as c_int);
                    }
                    png_read_finish_row(png_ptr);
                    return;
                }
            }
            4 => {
                if (*png_ptr).row_number as c_uint & 3 as c_uint
                    != 2 as c_uint
                {
                    if !dsp_row.is_null()
                        && (*png_ptr).row_number as c_uint & 2 as c_uint
                            != 0
                    {
                        png_combine_row(png_ptr, dsp_row, 1 as c_int);
                    }
                    png_read_finish_row(png_ptr);
                    return;
                }
            }
            5 => {
                if (*png_ptr).row_number as c_uint & 1 as c_uint != 0
                    || (*png_ptr).width < 2 as c_uint
                {
                    if !dsp_row.is_null() {
                        png_combine_row(png_ptr, dsp_row, 1 as c_int);
                    }
                    png_read_finish_row(png_ptr);
                    return;
                }
            }
            6 | _ => {
                if (*png_ptr).row_number as c_uint & 1 as c_uint
                    == 0 as c_uint
                {
                    png_read_finish_row(png_ptr);
                    return;
                }
            }
        }
    }
    if (*png_ptr).mode as c_uint & PNG_HAVE_IDAT == 0 as c_uint {
        png_error(
            png_ptr,
            b"Invalid attempt to read row data\0" as *const u8 as png_const_charp,
        );
    }
    *(*png_ptr).row_buf.offset(0 as c_int as isize) = 255 as png_byte;
    png_read_IDAT_data(
        png_ptr,
        (*png_ptr).row_buf,
        (row_info.rowbytes as png_alloc_size_t).wrapping_add(1 as png_alloc_size_t),
    );
    if *(*png_ptr).row_buf.offset(0 as c_int as isize) as c_int
        > PNG_FILTER_VALUE_NONE
    {
        if (*(*png_ptr).row_buf.offset(0 as c_int as isize) as c_int)
            < PNG_FILTER_VALUE_LAST
        {
            png_read_filter_row(
                png_ptr,
                &raw mut row_info,
                (*png_ptr).row_buf.offset(1 as c_int as isize),
                (*png_ptr).prev_row.offset(1 as c_int as isize) as png_const_bytep,
                *(*png_ptr).row_buf.offset(0 as c_int as isize) as c_int,
            );
        } else {
            png_error(
                png_ptr,
                b"bad adaptive filter value\0" as *const u8 as png_const_charp,
            );
        }
    }
    memcpy(
        (*png_ptr).prev_row as *mut c_void,
        (*png_ptr).row_buf as *const c_void,
        (row_info.rowbytes as size_t).wrapping_add(1 as size_t),
    );
    if (*png_ptr).transformations != 0 {
        png_do_read_transformations(png_ptr, &raw mut row_info);
    }
    if (*png_ptr).transformed_pixel_depth as c_int == 0 as c_int {
        (*png_ptr).transformed_pixel_depth = row_info.pixel_depth;
        if row_info.pixel_depth as c_int
            > (*png_ptr).maximum_pixel_depth as c_int
        {
            png_error(
                png_ptr,
                b"sequential row overflow\0" as *const u8 as png_const_charp,
            );
        }
    } else if (*png_ptr).transformed_pixel_depth as c_int
        != row_info.pixel_depth as c_int
    {
        png_error(
            png_ptr,
            b"internal sequential row size calculation error\0" as *const u8 as png_const_charp,
        );
    }
    if (*png_ptr).interlaced as c_int != 0 as c_int
        && (*png_ptr).transformations as c_uint & PNG_INTERLACE
            != 0 as c_uint
    {
        if ((*png_ptr).pass as c_int) < 6 as c_int {
            png_do_read_interlace(
                &raw mut row_info,
                (*png_ptr).row_buf.offset(1 as c_int as isize),
                (*png_ptr).pass as c_int,
                (*png_ptr).transformations,
            );
        }
        if !dsp_row.is_null() {
            png_combine_row(png_ptr, dsp_row, 1 as c_int);
        }
        if !row.is_null() {
            png_combine_row(png_ptr, row, 0 as c_int);
        }
    } else {
        if !row.is_null() {
            png_combine_row(png_ptr, row, -(1 as c_int));
        }
        if !dsp_row.is_null() {
            png_combine_row(png_ptr, dsp_row, -(1 as c_int));
        }
    }
    png_read_finish_row(png_ptr);
    if (*png_ptr).read_row_fn.is_some() {
        Some((*png_ptr).read_row_fn.expect("non-null function pointer"))
            .expect("non-null function pointer")(
            png_ptr as png_structp,
            (*png_ptr).row_number,
            (*png_ptr).pass as c_int,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn png_read_rows(
    mut png_ptr: png_structrp,
    mut row: png_bytepp,
    mut display_row: png_bytepp,
    mut num_rows: png_uint_32,
) {
    let mut i: png_uint_32 = 0;
    let mut rp: png_bytepp = ::core::ptr::null_mut::<*mut png_byte>();
    let mut dp: png_bytepp = ::core::ptr::null_mut::<*mut png_byte>();
    if png_ptr.is_null() {
        return;
    }
    rp = row;
    dp = display_row;
    if !rp.is_null() && !dp.is_null() {
        i = 0 as png_uint_32;
        while i < num_rows {
            let fresh0 = rp;
            rp = rp.offset(1);
            let mut rptr: png_bytep = *fresh0;
            let fresh1 = dp;
            dp = dp.offset(1);
            let mut dptr: png_bytep = *fresh1;
            png_read_row(png_ptr, rptr, dptr);
            i = i.wrapping_add(1);
        }
    } else if !rp.is_null() {
        i = 0 as png_uint_32;
        while i < num_rows {
            let mut rptr_0: png_bytep = *rp;
            png_read_row(png_ptr, rptr_0, ::core::ptr::null_mut::<png_byte>());
            rp = rp.offset(1);
            i = i.wrapping_add(1);
        }
    } else if !dp.is_null() {
        i = 0 as png_uint_32;
        while i < num_rows {
            let mut dptr_0: png_bytep = *dp;
            png_read_row(png_ptr, ::core::ptr::null_mut::<png_byte>(), dptr_0);
            dp = dp.offset(1);
            i = i.wrapping_add(1);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn png_read_image(mut png_ptr: png_structrp, mut image: png_bytepp) {
    let mut i: png_uint_32 = 0;
    let mut image_height: png_uint_32 = 0;
    let mut pass: c_int = 0;
    let mut j: c_int = 0;
    let mut rp: png_bytepp = ::core::ptr::null_mut::<*mut png_byte>();
    if png_ptr.is_null() {
        return;
    }
    if (*png_ptr).flags as c_uint & PNG_FLAG_ROW_INIT == 0 as c_uint {
        pass = png_set_interlace_handling(png_ptr);
        png_start_read_image(png_ptr);
    } else {
        if (*png_ptr).interlaced as c_int != 0 as c_int
            && (*png_ptr).transformations as c_uint & PNG_INTERLACE
                == 0 as c_uint
        {
            png_warning(
                png_ptr,
                b"Interlace handling should be turned on when using png_read_image\0" as *const u8
                    as png_const_charp,
            );
            (*png_ptr).num_rows = (*png_ptr).height;
        }
        pass = png_set_interlace_handling(png_ptr);
    }
    image_height = (*png_ptr).height;
    j = 0 as c_int;
    while j < pass {
        rp = image;
        i = 0 as png_uint_32;
        while i < image_height {
            png_read_row(png_ptr, *rp, ::core::ptr::null_mut::<png_byte>());
            rp = rp.offset(1);
            i = i.wrapping_add(1);
        }
        j += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn png_read_end(mut png_ptr: png_structrp, mut info_ptr: png_inforp) {
    let mut keep: c_int = 0;
    if png_ptr.is_null() {
        return;
    }
    if png_chunk_unknown_handling(png_ptr, png_IDAT) == 0 as c_int {
        png_read_finish_IDAT(png_ptr);
    }
    if (*png_ptr).color_type as c_int == PNG_COLOR_TYPE_PALETTE
        && (*png_ptr).num_palette_max > (*png_ptr).num_palette as c_int
    {
        png_benign_error(
            png_ptr,
            b"Read palette index exceeding num_palette\0" as *const u8 as png_const_charp,
        );
    }
    loop {
        let mut length: png_uint_32 = png_read_chunk_header(png_ptr);
        let mut chunk_name: png_uint_32 = (*png_ptr).chunk_name;
        if chunk_name != png_IDAT {
            (*png_ptr).mode |= PNG_HAVE_CHUNK_AFTER_IDAT;
        }
        if chunk_name == png_IEND {
            png_handle_IEND(png_ptr, info_ptr, length);
        } else if chunk_name == png_IHDR {
            png_handle_IHDR(png_ptr, info_ptr, length);
        } else if info_ptr.is_null() {
            png_crc_finish(png_ptr, length);
        } else {
            keep = png_chunk_unknown_handling(png_ptr, chunk_name);
            if keep != 0 as c_int {
                if chunk_name == png_IDAT {
                    if length > 0 as c_uint
                        && (*png_ptr).flags as c_uint & PNG_FLAG_ZSTREAM_ENDED == 0
                        || (*png_ptr).mode as c_uint & PNG_HAVE_CHUNK_AFTER_IDAT
                            != 0 as c_uint
                    {
                        png_benign_error(
                            png_ptr,
                            b".Too many IDATs found\0" as *const u8 as png_const_charp,
                        );
                    }
                }
                png_handle_unknown(png_ptr, info_ptr, length, keep);
                if chunk_name == png_PLTE {
                    (*png_ptr).mode |= PNG_HAVE_PLTE as c_uint;
                }
            } else if chunk_name == png_IDAT {
                if length > 0 as c_uint
                    && (*png_ptr).flags as c_uint & PNG_FLAG_ZSTREAM_ENDED == 0
                    || (*png_ptr).mode as c_uint & PNG_HAVE_CHUNK_AFTER_IDAT
                        != 0 as c_uint
                {
                    png_benign_error(
                        png_ptr,
                        b"..Too many IDATs found\0" as *const u8 as png_const_charp,
                    );
                }
                png_crc_finish(png_ptr, length);
            } else if chunk_name == png_PLTE {
                png_handle_PLTE(png_ptr, info_ptr, length);
            } else if chunk_name == png_bKGD {
                png_handle_bKGD(png_ptr, info_ptr, length);
            } else if chunk_name == png_hIST {
                png_handle_hIST(png_ptr, info_ptr, length);
            } else if chunk_name == png_sBIT {
                png_handle_sBIT(png_ptr, info_ptr, length);
            } else if chunk_name == png_tRNS {
                png_handle_tRNS(png_ptr, info_ptr, length);
            } else {
                png_handle_unknown(png_ptr, info_ptr, length, PNG_HANDLE_CHUNK_AS_DEFAULT);
            }
        }
        if !((*png_ptr).mode as c_uint & PNG_HAVE_IEND == 0 as c_uint) {
            break;
        }
    }
}
unsafe extern "C" fn png_read_destroy(mut png_ptr: png_structrp) {
    png_free(png_ptr, (*png_ptr).big_row_buf as png_voidp);
    (*png_ptr).big_row_buf = ::core::ptr::null_mut::<png_byte>();
    png_free(png_ptr, (*png_ptr).big_prev_row as png_voidp);
    (*png_ptr).big_prev_row = ::core::ptr::null_mut::<png_byte>();
    png_free(png_ptr, (*png_ptr).read_buffer as png_voidp);
    (*png_ptr).read_buffer = ::core::ptr::null_mut::<png_byte>();
    if (*png_ptr).free_me as c_uint & PNG_FREE_PLTE != 0 as c_uint {
        png_zfree(png_ptr as voidpf, (*png_ptr).palette as voidpf);
        (*png_ptr).palette = ::core::ptr::null_mut::<png_color>();
    }
    (*png_ptr).free_me &= !PNG_FREE_PLTE;
    if (*png_ptr).free_me as c_uint & PNG_FREE_TRNS != 0 as c_uint {
        png_free(png_ptr, (*png_ptr).trans_alpha as png_voidp);
        (*png_ptr).trans_alpha = ::core::ptr::null_mut::<png_byte>();
    }
    (*png_ptr).free_me &= !PNG_FREE_TRNS;
    inflateEnd(&raw mut (*png_ptr).zstream);
    png_free(png_ptr, (*png_ptr).unknown_chunk.data as png_voidp);
    (*png_ptr).unknown_chunk.data = ::core::ptr::null_mut::<png_byte>();
    png_free(png_ptr, (*png_ptr).chunk_list as png_voidp);
    (*png_ptr).chunk_list = ::core::ptr::null_mut::<png_byte>();
}
#[no_mangle]
pub unsafe extern "C" fn png_destroy_read_struct(
    mut png_ptr_ptr: png_structpp,
    mut info_ptr_ptr: png_infopp,
    mut end_info_ptr_ptr: png_infopp,
) {
    let mut png_ptr: png_structrp = ::core::ptr::null_mut::<png_struct>();
    if !png_ptr_ptr.is_null() {
        png_ptr = *png_ptr_ptr as png_structrp;
    }
    if png_ptr.is_null() {
        return;
    }
    png_destroy_info_struct(png_ptr, end_info_ptr_ptr);
    png_destroy_info_struct(png_ptr, info_ptr_ptr);
    *png_ptr_ptr = ::core::ptr::null_mut::<png_struct>();
    png_read_destroy(png_ptr);
    png_destroy_png_struct(png_ptr);
}
#[no_mangle]
pub unsafe extern "C" fn png_set_read_status_fn(
    mut png_ptr: png_structrp,
    mut read_row_fn: png_read_status_ptr,
) {
    if png_ptr.is_null() {
        return;
    }
    (*png_ptr).read_row_fn = read_row_fn;
}
#[no_mangle]
pub unsafe extern "C" fn png_read_png(
    mut png_ptr: png_structrp,
    mut info_ptr: png_inforp,
    mut transforms: c_int,
    mut params: voidp,
) {
    if png_ptr.is_null() || info_ptr.is_null() {
        return;
    }
    png_read_info(png_ptr, info_ptr);
    if (*info_ptr).height as usize
        > (PNG_UINT_32_MAX as usize).wrapping_div(::core::mem::size_of::<png_bytep>() as usize)
    {
        png_error(
            png_ptr,
            b"Image is too high to process with png_read_png()\0" as *const u8 as png_const_charp,
        );
    }
    if transforms & PNG_TRANSFORM_SCALE_16 != 0 as c_int {
        png_app_error(
            png_ptr,
            b"PNG_TRANSFORM_SCALE_16 not supported\0" as *const u8 as png_const_charp,
        );
    }
    if transforms & PNG_TRANSFORM_STRIP_16 != 0 as c_int {
        png_app_error(
            png_ptr,
            b"PNG_TRANSFORM_STRIP_16 not supported\0" as *const u8 as png_const_charp,
        );
    }
    if transforms & PNG_TRANSFORM_STRIP_ALPHA != 0 as c_int {
        png_app_error(
            png_ptr,
            b"PNG_TRANSFORM_STRIP_ALPHA not supported\0" as *const u8 as png_const_charp,
        );
    }
    if transforms & PNG_TRANSFORM_PACKING != 0 as c_int {
        png_app_error(
            png_ptr,
            b"PNG_TRANSFORM_PACKING not supported\0" as *const u8 as png_const_charp,
        );
    }
    if transforms & PNG_TRANSFORM_PACKSWAP != 0 as c_int {
        png_app_error(
            png_ptr,
            b"PNG_TRANSFORM_PACKSWAP not supported\0" as *const u8 as png_const_charp,
        );
    }
    if transforms & PNG_TRANSFORM_EXPAND != 0 as c_int {
        png_app_error(
            png_ptr,
            b"PNG_TRANSFORM_EXPAND not supported\0" as *const u8 as png_const_charp,
        );
    }
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
    if transforms & PNG_TRANSFORM_BGR != 0 as c_int {
        png_app_error(
            png_ptr,
            b"PNG_TRANSFORM_BGR not supported\0" as *const u8 as png_const_charp,
        );
    }
    if transforms & PNG_TRANSFORM_SWAP_ALPHA != 0 as c_int {
        png_app_error(
            png_ptr,
            b"PNG_TRANSFORM_SWAP_ALPHA not supported\0" as *const u8 as png_const_charp,
        );
    }
    if transforms & PNG_TRANSFORM_SWAP_ENDIAN != 0 as c_int {
        png_app_error(
            png_ptr,
            b"PNG_TRANSFORM_SWAP_ENDIAN not supported\0" as *const u8 as png_const_charp,
        );
    }
    if transforms & PNG_TRANSFORM_INVERT_ALPHA != 0 as c_int {
        png_app_error(
            png_ptr,
            b"PNG_TRANSFORM_INVERT_ALPHA not supported\0" as *const u8 as png_const_charp,
        );
    }
    if transforms & PNG_TRANSFORM_GRAY_TO_RGB != 0 as c_int {
        png_app_error(
            png_ptr,
            b"PNG_TRANSFORM_GRAY_TO_RGB not supported\0" as *const u8 as png_const_charp,
        );
    }
    if transforms & PNG_TRANSFORM_EXPAND_16 != 0 as c_int {
        png_app_error(
            png_ptr,
            b"PNG_TRANSFORM_EXPAND_16 not supported\0" as *const u8 as png_const_charp,
        );
    }
    png_set_interlace_handling(png_ptr);
    png_read_update_info(png_ptr, info_ptr);
    png_free_data(png_ptr, info_ptr, PNG_FREE_ROWS, 0 as c_int);
    if (*info_ptr).row_pointers.is_null() {
        let mut iptr: png_uint_32 = 0;
        (*info_ptr).row_pointers = png_malloc(
            png_ptr,
            ((*info_ptr).height as png_alloc_size_t)
                .wrapping_mul(::core::mem::size_of::<png_bytep>() as png_alloc_size_t),
        ) as png_bytepp;
        iptr = 0 as png_uint_32;
        while iptr < (*info_ptr).height {
            let ref mut fresh2 = *(*info_ptr).row_pointers.offset(iptr as isize);
            *fresh2 = ::core::ptr::null_mut::<png_byte>();
            iptr = iptr.wrapping_add(1);
        }
        (*info_ptr).free_me |= PNG_FREE_ROWS;
        iptr = 0 as png_uint_32;
        while iptr < (*info_ptr).height {
            let ref mut fresh3 = *(*info_ptr).row_pointers.offset(iptr as isize);
            *fresh3 =
                png_malloc(png_ptr, (*info_ptr).rowbytes as png_alloc_size_t) as *mut png_byte;
            iptr = iptr.wrapping_add(1);
        }
    }
    png_read_image(png_ptr, (*info_ptr).row_pointers);
    (*info_ptr).valid |= PNG_INFO_IDAT;
    png_read_end(png_ptr, info_ptr);
}
