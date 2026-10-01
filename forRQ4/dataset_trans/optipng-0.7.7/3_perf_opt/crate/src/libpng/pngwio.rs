use core::ffi::*;
pub use crate::src::ffi::*;
pub use crate::src::c_consts::*;
pub use crate::src::c_structs::*;
pub use crate::src::c_types::*;
pub use crate::src::c_extern_types::*;
pub use crate::src::zlib::deflate::internal_state;
extern "C" {
    fn png_error(png_ptr: png_const_structrp, error_message: png_const_charp) -> !;
    fn png_warning(png_ptr: png_const_structrp, warning_message: png_const_charp);
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

#[no_mangle]
pub extern "C" fn png_write_data(
    mut png_ptr: png_structrp,
    mut data: png_const_bytep,
    mut length: png_size_t,
) { unsafe {
    if (*png_ptr).write_data_fn.is_some() {
        Some((*png_ptr).write_data_fn.expect("non-null function pointer"))
            .expect("non-null function pointer")(
            png_ptr as png_structp,
            data as *const c_void as png_ptruint as png_bytep,
            length,
        );
    } else {
        png_error(
            png_ptr,
            b"Call to NULL write function\0" as *const u8 as png_const_charp,
        );
    };
} }
#[no_mangle]
pub extern "C" fn png_default_write_data(
    mut png_ptr: png_structp,
    mut data: png_bytep,
    mut length: png_size_t,
) { unsafe {
    let mut check: png_size_t = 0;
    if png_ptr.is_null() {
        return;
    }
    check = fwrite(
        data as *const c_void,
        1 as size_t,
        length as size_t,
        (*png_ptr).io_ptr as *mut FILE,
    ) as png_size_t;
    if check != length {
        png_error(
            png_ptr as png_const_structrp,
            b"Write Error\0" as *const u8 as png_const_charp,
        );
    }
} }
#[no_mangle]
pub extern "C" fn png_flush(mut png_ptr: png_structrp) { unsafe {
    if (*png_ptr).output_flush_fn.is_some() {
        Some(
            (*png_ptr)
                .output_flush_fn
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(png_ptr as png_structp);
    }
} }
#[no_mangle]
pub extern "C" fn png_default_flush(mut png_ptr: png_structp) { unsafe {
    let mut io_ptr: png_FILE_p = ::core::ptr::null_mut::<FILE>();
    if png_ptr.is_null() {
        return;
    }
    io_ptr = (*png_ptr).io_ptr as png_FILE_p;
    fflush(io_ptr as *mut FILE);
} }
#[no_mangle]
pub extern "C" fn png_set_write_fn(
    mut png_ptr: png_structrp,
    mut io_ptr: png_voidp,
    mut write_data_fn: png_rw_ptr,
    mut output_flush_fn: png_flush_ptr,
) { unsafe {
    if png_ptr.is_null() {
        return;
    }
    (*png_ptr).io_ptr = io_ptr;
    if write_data_fn.is_some() {
        (*png_ptr).write_data_fn = write_data_fn;
    } else {
        (*png_ptr).write_data_fn = Some(
            png_default_write_data
                as unsafe extern "C" fn(png_structp, png_bytep, png_size_t) -> (),
        ) as png_rw_ptr;
    }
    if output_flush_fn.is_some() {
        (*png_ptr).output_flush_fn = output_flush_fn;
    } else {
        (*png_ptr).output_flush_fn =
            Some(png_default_flush as unsafe extern "C" fn(png_structp) -> ()) as png_flush_ptr;
    }
    if (*png_ptr).read_data_fn.is_some() {
        (*png_ptr).read_data_fn = None;
        png_warning(
            png_ptr,
            b"Can't set both read_data_fn and write_data_fn in the same structure\0" as *const u8
                as png_const_charp,
        );
    }
} }
