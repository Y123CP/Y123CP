use core::ffi::*;
pub use crate::src::c_consts::*;
pub use crate::src::c_structs::*;
pub use crate::src::c_types::*;
extern "C" {
    pub type internal_state;
    fn png_error(png_ptr: png_const_structrp, error_message: png_const_charp) -> !;
    fn png_warning(png_ptr: png_const_structrp, warning_message: png_const_charp);
    fn png_do_check_palette_indexes(png_ptr: png_structrp, row_info: png_row_infop);
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
pub type png_structrp = *mut png_struct;
pub type png_const_structrp = *const png_struct;
pub type png_inforp = *mut png_info;

pub const PNG_COLOR_TYPE_PALETTE: c_int =
    PNG_COLOR_MASK_COLOR | PNG_COLOR_MASK_PALETTE;
pub const PNG_CRC_DEFAULT: c_int = 0;
pub const PNG_CRC_ERROR_QUIT: c_int = 1;
pub const PNG_CRC_WARN_DISCARD: c_int = 2;
pub const PNG_CRC_WARN_USE: c_int = 3;
pub const PNG_CRC_QUIET_USE: c_int = 4;
pub const PNG_CRC_NO_CHANGE: c_int = 5;
pub const PNG_COMPOSE: c_uint = 0x80 as c_uint;
pub const PNG_BACKGROUND_EXPAND: c_uint = 0x100 as c_uint;
pub const PNG_ENCODE_ALPHA: c_uint = 0x800000 as c_uint;

pub const PNG_FLAG_OPTIMIZE_ALPHA: c_uint = 0x2000 as c_uint;
pub const PNG_FLAG_DETECT_UNINITIALIZED: c_uint = 0x4000 as c_uint;

pub const PNG_FLAG_CRC_CRITICAL_MASK: c_uint =
    PNG_FLAG_CRC_CRITICAL_USE | PNG_FLAG_CRC_CRITICAL_IGNORE;
#[no_mangle]
pub unsafe extern "C" fn png_set_crc_action(
    mut png_ptr: png_structrp,
    mut crit_action: c_int,
    mut ancil_action: c_int,
) {
    if png_ptr.is_null() {
        return;
    }
    let mut current_block_8: u64;
    match crit_action {
        PNG_CRC_NO_CHANGE => {
            current_block_8 = 13109137661213826276;
        }
        PNG_CRC_WARN_USE => {
            (*png_ptr).flags &= !PNG_FLAG_CRC_CRITICAL_MASK;
            (*png_ptr).flags |= PNG_FLAG_CRC_CRITICAL_USE;
            current_block_8 = 13109137661213826276;
        }
        PNG_CRC_QUIET_USE => {
            (*png_ptr).flags &= !PNG_FLAG_CRC_CRITICAL_MASK;
            (*png_ptr).flags |= PNG_FLAG_CRC_CRITICAL_USE | PNG_FLAG_CRC_CRITICAL_IGNORE;
            current_block_8 = 13109137661213826276;
        }
        PNG_CRC_WARN_DISCARD => {
            png_warning(
                png_ptr,
                b"Can't discard critical data on CRC error\0" as *const u8 as png_const_charp,
            );
            current_block_8 = 14060278150046481628;
        }
        PNG_CRC_ERROR_QUIT => {
            current_block_8 = 14060278150046481628;
        }
        PNG_CRC_DEFAULT | _ => {
            current_block_8 = 14060278150046481628;
        }
    }
    match current_block_8 {
        14060278150046481628 => {
            (*png_ptr).flags &= !PNG_FLAG_CRC_CRITICAL_MASK;
        }
        _ => {}
    }
    match ancil_action {
        PNG_CRC_NO_CHANGE => {}
        PNG_CRC_WARN_USE => {
            (*png_ptr).flags &= !PNG_FLAG_CRC_ANCILLARY_MASK;
            (*png_ptr).flags |= PNG_FLAG_CRC_ANCILLARY_USE;
        }
        PNG_CRC_QUIET_USE => {
            (*png_ptr).flags &= !PNG_FLAG_CRC_ANCILLARY_MASK;
            (*png_ptr).flags |= PNG_FLAG_CRC_ANCILLARY_USE | PNG_FLAG_CRC_ANCILLARY_NOWARN;
        }
        PNG_CRC_ERROR_QUIT => {
            (*png_ptr).flags &= !PNG_FLAG_CRC_ANCILLARY_MASK;
            (*png_ptr).flags |= PNG_FLAG_CRC_ANCILLARY_NOWARN;
        }
        PNG_CRC_WARN_DISCARD | PNG_CRC_DEFAULT | _ => {
            (*png_ptr).flags &= !PNG_FLAG_CRC_ANCILLARY_MASK;
        }
    };
}
unsafe extern "C" fn png_init_palette_transformations(mut png_ptr: png_structrp) {
    let mut input_has_alpha: c_int = 0 as c_int;
    let mut input_has_transparency: c_int = 0 as c_int;
    if (*png_ptr).num_trans as c_int > 0 as c_int {
        let mut i: c_int = 0;
        i = 0 as c_int;
        while i < (*png_ptr).num_trans as c_int {
            if !(*(*png_ptr).trans_alpha.offset(i as isize) as c_int
                == 255 as c_int)
            {
                if *(*png_ptr).trans_alpha.offset(i as isize) as c_int
                    == 0 as c_int
                {
                    input_has_transparency = 1 as c_int;
                } else {
                    input_has_transparency = 1 as c_int;
                    input_has_alpha = 1 as c_int;
                    break;
                }
            }
            i += 1;
        }
    }
    if input_has_alpha == 0 as c_int {
        (*png_ptr).transformations &= !PNG_ENCODE_ALPHA;
        (*png_ptr).flags &= !PNG_FLAG_OPTIMIZE_ALPHA;
        if input_has_transparency == 0 as c_int {
            (*png_ptr).transformations &= !(PNG_COMPOSE | PNG_BACKGROUND_EXPAND);
        }
    }
}
unsafe extern "C" fn png_init_rgb_transformations(mut png_ptr: png_structrp) {
    let mut input_has_alpha: c_int =
        ((*png_ptr).color_type as c_int & PNG_COLOR_MASK_ALPHA
            != 0 as c_int) as c_int;
    let mut input_has_transparency: c_int =
        ((*png_ptr).num_trans as c_int > 0 as c_int)
            as c_int;
    if input_has_alpha == 0 as c_int {
        if input_has_transparency == 0 as c_int {
            (*png_ptr).transformations &= !(PNG_COMPOSE | PNG_BACKGROUND_EXPAND);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn png_init_read_transformations(mut png_ptr: png_structrp) {
    if (*png_ptr).color_type as c_int == PNG_COLOR_TYPE_PALETTE {
        png_init_palette_transformations(png_ptr);
    } else {
        png_init_rgb_transformations(png_ptr);
    };
}
#[no_mangle]
pub unsafe extern "C" fn png_read_transform_info(
    mut png_ptr: png_structrp,
    mut info_ptr: png_inforp,
) {
    (*info_ptr).bit_depth as c_int == 16 as c_int;
    if (*info_ptr).color_type as c_int == PNG_COLOR_TYPE_PALETTE {
        (*info_ptr).channels = 1 as png_byte;
    } else if (*info_ptr).color_type as c_int & PNG_COLOR_MASK_COLOR
        != 0 as c_int
    {
        (*info_ptr).channels = 3 as png_byte;
    } else {
        (*info_ptr).channels = 1 as png_byte;
    }
    if (*info_ptr).color_type as c_int & PNG_COLOR_MASK_ALPHA
        != 0 as c_int
    {
        (*info_ptr).channels = (*info_ptr).channels.wrapping_add(1);
    }
    (*info_ptr).pixel_depth = ((*info_ptr).channels as c_int
        * (*info_ptr).bit_depth as c_int) as png_byte;
    (*info_ptr).rowbytes =
        if (*info_ptr).pixel_depth as c_int >= 8 as c_int {
            ((*info_ptr).width as png_size_t)
                .wrapping_mul((*info_ptr).pixel_depth as png_size_t >> 3 as c_int)
        } else {
            ((*info_ptr).width as png_size_t)
                .wrapping_mul((*info_ptr).pixel_depth as png_size_t)
                .wrapping_add(7 as png_size_t)
                >> 3 as c_int
        };
    (*png_ptr).info_rowbytes = (*info_ptr).rowbytes;
    if !png_ptr.is_null() {
        return;
    }
}
#[no_mangle]
pub unsafe extern "C" fn png_do_read_transformations(
    mut png_ptr: png_structrp,
    mut row_info: png_row_infop,
) {
    if (*png_ptr).row_buf.is_null() {
        png_error(
            png_ptr,
            b"NULL row buffer\0" as *const u8 as png_const_charp,
        );
    }
    if (*png_ptr).flags as c_uint & PNG_FLAG_DETECT_UNINITIALIZED
        != 0 as c_uint
        && (*png_ptr).flags as c_uint & PNG_FLAG_ROW_INIT == 0 as c_uint
    {
        png_error(
            png_ptr,
            b"Uninitialized row\0" as *const u8 as png_const_charp,
        );
    }
    if (*row_info).color_type as c_int == PNG_COLOR_TYPE_PALETTE
        && (*png_ptr).num_palette_max >= 0 as c_int
    {
        png_do_check_palette_indexes(png_ptr, row_info);
    }
}
