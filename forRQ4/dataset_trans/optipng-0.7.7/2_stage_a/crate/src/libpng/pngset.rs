use core::ffi::*;
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
    fn png_free_data(
        png_ptr: png_const_structrp,
        info_ptr: png_inforp,
        free_me: png_uint_32,
        num: c_int,
    );
    fn png_error(png_ptr: png_const_structrp, error_message: png_const_charp) -> !;
    fn png_warning(png_ptr: png_const_structrp, warning_message: png_const_charp);
    fn png_free_buffer_list(png_ptr: png_structrp, list: *mut png_compression_bufferp);
    fn png_malloc_base(png_ptr: png_const_structrp, size: png_alloc_size_t) -> png_voidp;
    fn png_realloc_array(
        png_ptr: png_const_structrp,
        array: png_const_voidp,
        old_elements: c_int,
        add_elements: c_int,
        element_size: size_t,
    ) -> png_voidp;
    fn png_check_IHDR(
        png_ptr: png_const_structrp,
        width: png_uint_32,
        height: png_uint_32,
        bit_depth: c_int,
        color_type: c_int,
        interlace_type: c_int,
        compression_type: c_int,
        filter_type: c_int,
    );
    fn png_app_warning(png_ptr: png_const_structrp, message: png_const_charp);
    fn png_app_error(png_ptr: png_const_structrp, message: png_const_charp);
    fn png_chunk_report(
        png_ptr: png_const_structrp,
        message: png_const_charp,
        error: c_int,
    );
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

pub const PNG_COLOR_TYPE_GRAY: c_int = 0 as c_int;
pub const PNG_COLOR_TYPE_PALETTE: c_int =
    PNG_COLOR_MASK_COLOR | PNG_COLOR_MASK_PALETTE;
pub const PNG_COLOR_TYPE_RGB: c_int = 2 as c_int;

pub const PNG_HANDLE_CHUNK_LAST: c_int = 4 as c_int;

#[no_mangle]
pub extern "C" fn png_set_bKGD(
    mut png_ptr: png_const_structrp,
    mut info_ptr: png_inforp,
    mut background: png_const_color_16p,
) { unsafe {
    if png_ptr.is_null() || info_ptr.is_null() || background.is_null() {
        return;
    }
    (*info_ptr).background = *background;
    (*info_ptr).valid |= PNG_INFO_bKGD;
} }
#[no_mangle]
pub extern "C" fn png_set_hIST(
    mut png_ptr: png_const_structrp,
    mut info_ptr: png_inforp,
    mut hist: png_const_uint_16p,
) { unsafe {
    let mut i: c_int = 0;
    if png_ptr.is_null() || info_ptr.is_null() {
        return;
    }
    if (*info_ptr).num_palette as c_int == 0 as c_int
        || (*info_ptr).num_palette as c_int > PNG_MAX_PALETTE_LENGTH
    {
        png_warning(
            png_ptr,
            b"Invalid palette size, hIST allocation skipped\0" as *const u8 as png_const_charp,
        );
        return;
    }
    png_free_data(png_ptr, info_ptr, PNG_FREE_HIST, 0 as c_int);
    (*info_ptr).hist = png_malloc_warn(
        png_ptr,
        (256 as png_alloc_size_t)
            .wrapping_mul(::core::mem::size_of::<png_uint_16>() as png_alloc_size_t),
    ) as png_uint_16p;
    if (*info_ptr).hist.is_null() {
        png_warning(
            png_ptr,
            b"Insufficient memory for hIST chunk data\0" as *const u8 as png_const_charp,
        );
        return;
    }
    (*info_ptr).free_me |= PNG_FREE_HIST;
    i = 0 as c_int;
    while i < (*info_ptr).num_palette as c_int {
        *(*info_ptr).hist.offset(i as isize) = *hist.offset(i as isize);
        i += 1;
    }
    (*info_ptr).valid |= PNG_INFO_hIST;
} }
#[no_mangle]
pub extern "C" fn png_set_IHDR(
    mut png_ptr: png_const_structrp,
    mut info_ptr: png_inforp,
    mut width: png_uint_32,
    mut height: png_uint_32,
    mut bit_depth: c_int,
    mut color_type: c_int,
    mut interlace_type: c_int,
    mut compression_type: c_int,
    mut filter_type: c_int,
) { unsafe {
    if png_ptr.is_null() || info_ptr.is_null() {
        return;
    }
    (*info_ptr).width = width;
    (*info_ptr).height = height;
    (*info_ptr).bit_depth = bit_depth as png_byte;
    (*info_ptr).color_type = color_type as png_byte;
    (*info_ptr).compression_type = compression_type as png_byte;
    (*info_ptr).filter_type = filter_type as png_byte;
    (*info_ptr).interlace_type = interlace_type as png_byte;
    png_check_IHDR(
        png_ptr,
        (*info_ptr).width,
        (*info_ptr).height,
        (*info_ptr).bit_depth as c_int,
        (*info_ptr).color_type as c_int,
        (*info_ptr).interlace_type as c_int,
        (*info_ptr).compression_type as c_int,
        (*info_ptr).filter_type as c_int,
    );
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
            (width as png_size_t)
                .wrapping_mul((*info_ptr).pixel_depth as png_size_t >> 3 as c_int)
        } else {
            (width as png_size_t)
                .wrapping_mul((*info_ptr).pixel_depth as png_size_t)
                .wrapping_add(7 as png_size_t)
                >> 3 as c_int
        };
} }
#[no_mangle]
pub extern "C" fn png_set_PLTE(
    mut png_ptr: png_structrp,
    mut info_ptr: png_inforp,
    mut palette: png_const_colorp,
    mut num_palette: c_int,
) { unsafe {
    let mut max_palette_length: png_uint_32 = 0;
    if png_ptr.is_null() || info_ptr.is_null() {
        return;
    }
    max_palette_length = (if (*info_ptr).color_type as c_int == PNG_COLOR_TYPE_PALETTE
    {
        (1 as c_int) << (*info_ptr).bit_depth as c_int
    } else {
        PNG_MAX_PALETTE_LENGTH
    }) as png_uint_32;
    if num_palette < 0 as c_int
        || num_palette > max_palette_length as c_int
    {
        if (*info_ptr).color_type as c_int == PNG_COLOR_TYPE_PALETTE {
            png_error(
                png_ptr,
                b"Invalid palette length\0" as *const u8 as png_const_charp,
            );
        } else {
            png_warning(
                png_ptr,
                b"Invalid palette length\0" as *const u8 as png_const_charp,
            );
            return;
        }
    }
    if num_palette > 0 as c_int && palette.is_null()
        || num_palette == 0 as c_int
    {
        png_error(
            png_ptr,
            b"Invalid palette\0" as *const u8 as png_const_charp,
        );
    }
    png_free_data(png_ptr, info_ptr, PNG_FREE_PLTE, 0 as c_int);
    (*png_ptr).palette = png_calloc(
        png_ptr,
        (256 as png_alloc_size_t)
            .wrapping_mul(::core::mem::size_of::<png_color>() as png_alloc_size_t),
    ) as png_colorp;
    if num_palette > 0 as c_int {
        memcpy(
            (*png_ptr).palette as *mut c_void,
            palette as *const c_void,
            (num_palette as c_uint as size_t)
                .wrapping_mul(::core::mem::size_of::<png_color>() as size_t),
        );
    }
    (*info_ptr).palette = (*png_ptr).palette;
    (*png_ptr).num_palette = num_palette as png_uint_16;
    (*info_ptr).num_palette = (*png_ptr).num_palette;
    (*info_ptr).free_me |= PNG_FREE_PLTE;
    (*info_ptr).valid |= PNG_INFO_PLTE;
} }
#[no_mangle]
pub extern "C" fn png_set_sBIT(
    mut png_ptr: png_const_structrp,
    mut info_ptr: png_inforp,
    mut sig_bit: png_const_color_8p,
) { unsafe {
    if png_ptr.is_null() || info_ptr.is_null() || sig_bit.is_null() {
        return;
    }
    (*info_ptr).sig_bit = *sig_bit;
    (*info_ptr).valid |= PNG_INFO_sBIT;
} }
#[no_mangle]
pub extern "C" fn png_set_tRNS(
    mut png_ptr: png_structrp,
    mut info_ptr: png_inforp,
    mut trans_alpha: png_const_bytep,
    mut num_trans: c_int,
    mut trans_color: png_const_color_16p,
) { unsafe {
    if png_ptr.is_null() || info_ptr.is_null() {
        return;
    }
    if !trans_alpha.is_null() {
        png_free_data(png_ptr, info_ptr, PNG_FREE_TRNS, 0 as c_int);
        if num_trans > 0 as c_int && num_trans <= PNG_MAX_PALETTE_LENGTH {
            (*info_ptr).trans_alpha = png_malloc(png_ptr, 256 as png_alloc_size_t) as png_bytep;
            memcpy(
                (*info_ptr).trans_alpha as *mut c_void,
                trans_alpha as *const c_void,
                num_trans as size_t,
            );
        }
        (*png_ptr).trans_alpha = (*info_ptr).trans_alpha;
    }
    if !trans_color.is_null() {
        if ((*info_ptr).bit_depth as c_int) < 16 as c_int {
            let mut sample_max: c_int = ((1 as c_int)
                << (*info_ptr).bit_depth as c_int)
                - 1 as c_int;
            if (*info_ptr).color_type as c_int == PNG_COLOR_TYPE_GRAY
                && (*trans_color).gray as c_int > sample_max
                || (*info_ptr).color_type as c_int == PNG_COLOR_TYPE_RGB
                    && ((*trans_color).red as c_int > sample_max
                        || (*trans_color).green as c_int > sample_max
                        || (*trans_color).blue as c_int > sample_max)
            {
                png_warning(
                    png_ptr,
                    b"tRNS chunk has out-of-range samples for bit_depth\0" as *const u8
                        as png_const_charp,
                );
            }
        }
        (*info_ptr).trans_color = *trans_color;
        if num_trans == 0 as c_int {
            num_trans = 1 as c_int;
        }
    }
    (*info_ptr).num_trans = num_trans as png_uint_16;
    if num_trans != 0 as c_int {
        (*info_ptr).valid |= PNG_INFO_tRNS;
        (*info_ptr).free_me |= PNG_FREE_TRNS;
    }
} }
fn check_location(
    mut png_ptr: png_const_structrp,
    mut location: c_int,
) -> png_byte { unsafe {
    location &= PNG_HAVE_IHDR | PNG_HAVE_PLTE | PNG_AFTER_IDAT;
    if location == 0 as c_int
        && (*png_ptr).mode as c_uint & PNG_IS_READ_STRUCT == 0 as c_uint
    {
        png_app_warning(
            png_ptr,
            b"png_set_unknown_chunks now expects a valid location\0" as *const u8
                as png_const_charp,
        );
        location = ((*png_ptr).mode as c_uint
            & (PNG_HAVE_IHDR | PNG_HAVE_PLTE | PNG_AFTER_IDAT) as c_uint)
            as png_byte as c_int;
    }
    if location == 0 as c_int {
        png_error(
            png_ptr,
            b"invalid location in png_set_unknown_chunks\0" as *const u8 as png_const_charp,
        );
    }
    while location != location & -location {
        location &= !(location & -location);
    }
    return location as png_byte;
} }
#[no_mangle]
pub extern "C" fn png_set_unknown_chunks(
    mut png_ptr: png_const_structrp,
    mut info_ptr: png_inforp,
    mut unknowns: png_const_unknown_chunkp,
    mut num_unknowns: c_int,
) { unsafe {
    let mut np: png_unknown_chunkp = ::core::ptr::null_mut::<png_unknown_chunk>();
    if png_ptr.is_null()
        || info_ptr.is_null()
        || num_unknowns <= 0 as c_int
        || unknowns.is_null()
    {
        return;
    }
    np = png_realloc_array(
        png_ptr,
        (*info_ptr).unknown_chunks as png_const_voidp,
        (*info_ptr).unknown_chunks_num,
        num_unknowns,
        ::core::mem::size_of::<png_unknown_chunk>() as size_t,
    ) as png_unknown_chunkp;
    if np.is_null() {
        png_chunk_report(
            png_ptr,
            b"too many unknown chunks\0" as *const u8 as png_const_charp,
            PNG_CHUNK_WRITE_ERROR,
        );
        return;
    }
    png_free(png_ptr, (*info_ptr).unknown_chunks as png_voidp);
    (*info_ptr).unknown_chunks = np;
    (*info_ptr).free_me |= PNG_FREE_UNKN;
    np = np.offset((*info_ptr).unknown_chunks_num as isize);
    let mut current_block_22: u64;
    while num_unknowns > 0 as c_int {
        memcpy(
            &raw mut (*np).name as *mut png_byte as *mut c_void,
            &raw const (*unknowns).name as *const png_byte as *const c_void,
            ::core::mem::size_of::<[png_byte; 5]>() as size_t,
        );
        (*np).name[(::core::mem::size_of::<[png_byte; 5]>() as usize).wrapping_sub(1 as usize)
            as usize] = '\0' as i32 as png_byte;
        (*np).location = check_location(png_ptr, (*unknowns).location as c_int);
        if (*unknowns).size == 0 as png_size_t {
            (*np).data = ::core::ptr::null_mut::<png_byte>();
            (*np).size = 0 as png_size_t;
            current_block_22 = 17833034027772472439;
        } else {
            (*np).data =
                png_malloc_base(png_ptr, (*unknowns).size as png_alloc_size_t) as *mut png_byte;
            if (*np).data.is_null() {
                png_chunk_report(
                    png_ptr,
                    b"unknown chunk: out of memory\0" as *const u8 as png_const_charp,
                    PNG_CHUNK_WRITE_ERROR,
                );
                current_block_22 = 10886091980245723256;
            } else {
                memcpy(
                    (*np).data as *mut c_void,
                    (*unknowns).data as *const c_void,
                    (*unknowns).size as size_t,
                );
                (*np).size = (*unknowns).size;
                current_block_22 = 17833034027772472439;
            }
        }
        match current_block_22 {
            17833034027772472439 => {
                np = np.offset(1);
                (*info_ptr).unknown_chunks_num += 1;
            }
            _ => {}
        }
        num_unknowns -= 1;
        unknowns = unknowns.offset(1);
    }
} }
#[no_mangle]
pub extern "C" fn png_set_unknown_chunk_location(
    mut png_ptr: png_const_structrp,
    mut info_ptr: png_inforp,
    mut chunk: c_int,
    mut location: c_int,
) { unsafe {
    if !png_ptr.is_null()
        && !info_ptr.is_null()
        && chunk >= 0 as c_int
        && chunk < (*info_ptr).unknown_chunks_num
    {
        if location & (PNG_HAVE_IHDR | PNG_HAVE_PLTE | PNG_AFTER_IDAT) == 0 as c_int {
            png_app_error(
                png_ptr,
                b"invalid unknown chunk location\0" as *const u8 as png_const_charp,
            );
            if location as c_uint & PNG_HAVE_IDAT != 0 as c_uint {
                location = PNG_AFTER_IDAT;
            } else {
                location = PNG_HAVE_IHDR;
            }
        }
        (*(*info_ptr).unknown_chunks.offset(chunk as isize)).location =
            check_location(png_ptr, location);
    }
} }
fn add_one_chunk(
    mut list: png_bytep,
    mut count: c_uint,
    mut add: png_const_bytep,
    mut keep: c_int,
) -> c_uint { unsafe {
    let mut i: c_uint = 0;
    i = 0 as c_uint;
    while i < count {
        if memcmp(
            list as *const c_void,
            add as *const c_void,
            4 as size_t,
        ) == 0 as c_int
        {
            *list.offset(4 as c_int as isize) = keep as png_byte;
            return count;
        }
        i = i.wrapping_add(1);
        list = list.offset(5 as c_int as isize);
    }
    if keep != PNG_HANDLE_CHUNK_AS_DEFAULT {
        count = count.wrapping_add(1);
        memcpy(
            list as *mut c_void,
            add as *const c_void,
            4 as size_t,
        );
        *list.offset(4 as c_int as isize) = keep as png_byte;
    }
    return count;
} }
#[no_mangle]
pub extern "C" fn png_set_keep_unknown_chunks(
    mut png_ptr: png_structrp,
    mut keep: c_int,
    mut chunk_list: png_const_bytep,
    mut num_chunks_in: c_int,
) { unsafe {
    let mut new_list: png_bytep = ::core::ptr::null_mut::<png_byte>();
    let mut num_chunks: c_uint = 0;
    let mut old_num_chunks: c_uint = 0;
    if png_ptr.is_null() {
        return;
    }
    if keep < 0 as c_int || keep >= PNG_HANDLE_CHUNK_LAST {
        png_app_error(
            png_ptr,
            b"png_set_keep_unknown_chunks: invalid keep\0" as *const u8 as png_const_charp,
        );
        return;
    }
    if num_chunks_in <= 0 as c_int {
        (*png_ptr).unknown_default = keep;
        if num_chunks_in == 0 as c_int {
            return;
        }
    }
    if num_chunks_in < 0 as c_int {
        static mut chunks_to_ignore: [png_byte; 90] = [
            98 as c_int as png_byte,
            75 as c_int as png_byte,
            71 as c_int as png_byte,
            68 as c_int as png_byte,
            '\0' as i32 as png_byte,
            99 as c_int as png_byte,
            72 as c_int as png_byte,
            82 as c_int as png_byte,
            77 as c_int as png_byte,
            '\0' as i32 as png_byte,
            101 as c_int as png_byte,
            88 as c_int as png_byte,
            73 as c_int as png_byte,
            102 as c_int as png_byte,
            '\0' as i32 as png_byte,
            103 as c_int as png_byte,
            65 as c_int as png_byte,
            77 as c_int as png_byte,
            65 as c_int as png_byte,
            '\0' as i32 as png_byte,
            104 as c_int as png_byte,
            73 as c_int as png_byte,
            83 as c_int as png_byte,
            84 as c_int as png_byte,
            '\0' as i32 as png_byte,
            105 as c_int as png_byte,
            67 as c_int as png_byte,
            67 as c_int as png_byte,
            80 as c_int as png_byte,
            '\0' as i32 as png_byte,
            105 as c_int as png_byte,
            84 as c_int as png_byte,
            88 as c_int as png_byte,
            116 as c_int as png_byte,
            '\0' as i32 as png_byte,
            111 as c_int as png_byte,
            70 as c_int as png_byte,
            70 as c_int as png_byte,
            115 as c_int as png_byte,
            '\0' as i32 as png_byte,
            112 as c_int as png_byte,
            67 as c_int as png_byte,
            65 as c_int as png_byte,
            76 as c_int as png_byte,
            '\0' as i32 as png_byte,
            112 as c_int as png_byte,
            72 as c_int as png_byte,
            89 as c_int as png_byte,
            115 as c_int as png_byte,
            '\0' as i32 as png_byte,
            115 as c_int as png_byte,
            66 as c_int as png_byte,
            73 as c_int as png_byte,
            84 as c_int as png_byte,
            '\0' as i32 as png_byte,
            115 as c_int as png_byte,
            67 as c_int as png_byte,
            65 as c_int as png_byte,
            76 as c_int as png_byte,
            '\0' as i32 as png_byte,
            115 as c_int as png_byte,
            80 as c_int as png_byte,
            76 as c_int as png_byte,
            84 as c_int as png_byte,
            '\0' as i32 as png_byte,
            115 as c_int as png_byte,
            84 as c_int as png_byte,
            69 as c_int as png_byte,
            82 as c_int as png_byte,
            '\0' as i32 as png_byte,
            115 as c_int as png_byte,
            82 as c_int as png_byte,
            71 as c_int as png_byte,
            66 as c_int as png_byte,
            '\0' as i32 as png_byte,
            116 as c_int as png_byte,
            69 as c_int as png_byte,
            88 as c_int as png_byte,
            116 as c_int as png_byte,
            '\0' as i32 as png_byte,
            116 as c_int as png_byte,
            73 as c_int as png_byte,
            77 as c_int as png_byte,
            69 as c_int as png_byte,
            '\0' as i32 as png_byte,
            122 as c_int as png_byte,
            84 as c_int as png_byte,
            88 as c_int as png_byte,
            116 as c_int as png_byte,
            '\0' as i32 as png_byte,
        ];
        chunk_list = &raw const chunks_to_ignore as *const png_byte as png_const_bytep;
        num_chunks = (::core::mem::size_of::<[png_byte; 90]>() as c_uint)
            .wrapping_div(5 as c_uint);
    } else {
        if chunk_list.is_null() {
            png_app_error(
                png_ptr,
                b"png_set_keep_unknown_chunks: no chunk list\0" as *const u8 as png_const_charp,
            );
            return;
        }
        num_chunks = num_chunks_in as c_uint;
    }
    old_num_chunks = (*png_ptr).num_chunk_list;
    if (*png_ptr).chunk_list.is_null() {
        old_num_chunks = 0 as c_uint;
    }
    if num_chunks.wrapping_add(old_num_chunks) > UINT_MAX.wrapping_div(5 as c_uint) {
        png_app_error(
            png_ptr,
            b"png_set_keep_unknown_chunks: too many chunks\0" as *const u8 as png_const_charp,
        );
        return;
    }
    if keep != 0 as c_int {
        new_list = png_malloc(
            png_ptr,
            (5 as c_uint).wrapping_mul(num_chunks.wrapping_add(old_num_chunks))
                as png_alloc_size_t,
        ) as png_bytep;
        if old_num_chunks > 0 as c_uint {
            memcpy(
                new_list as *mut c_void,
                (*png_ptr).chunk_list as *const c_void,
                (5 as c_uint).wrapping_mul(old_num_chunks) as size_t,
            );
        }
    } else if old_num_chunks > 0 as c_uint {
        new_list = (*png_ptr).chunk_list;
    } else {
        new_list = ::core::ptr::null_mut::<png_byte>();
    }
    if !new_list.is_null() {
        let mut inlist: png_const_bytep = ::core::ptr::null::<png_byte>();
        let mut outlist: png_bytep = ::core::ptr::null_mut::<png_byte>();
        let mut i: c_uint = 0;
        i = 0 as c_uint;
        while i < num_chunks {
            old_num_chunks = add_one_chunk(
                new_list,
                old_num_chunks,
                chunk_list.offset((5 as c_uint).wrapping_mul(i) as isize),
                keep,
            );
            i = i.wrapping_add(1);
        }
        num_chunks = 0 as c_uint;
        i = 0 as c_uint;
        outlist = new_list;
        inlist = outlist as png_const_bytep;
        while i < old_num_chunks {
            if *inlist.offset(4 as c_int as isize) != 0 {
                if outlist != inlist as png_bytep {
                    memcpy(
                        outlist as *mut c_void,
                        inlist as *const c_void,
                        5 as size_t,
                    );
                }
                outlist = outlist.offset(5 as c_int as isize);
                num_chunks = num_chunks.wrapping_add(1);
            }
            i = i.wrapping_add(1);
            inlist = inlist.offset(5 as c_int as isize);
        }
        if num_chunks == 0 as c_uint {
            if (*png_ptr).chunk_list != new_list {
                png_free(png_ptr, new_list as png_voidp);
            }
            new_list = ::core::ptr::null_mut::<png_byte>();
        }
    } else {
        num_chunks = 0 as c_uint;
    }
    (*png_ptr).num_chunk_list = num_chunks;
    if (*png_ptr).chunk_list != new_list {
        if !(*png_ptr).chunk_list.is_null() {
            png_free(png_ptr, (*png_ptr).chunk_list as png_voidp);
        }
        (*png_ptr).chunk_list = new_list;
    }
} }
#[no_mangle]
pub extern "C" fn png_set_rows(
    mut png_ptr: png_const_structrp,
    mut info_ptr: png_inforp,
    mut row_pointers: png_bytepp,
) { unsafe {
    if png_ptr.is_null() || info_ptr.is_null() {
        return;
    }
    if !(*info_ptr).row_pointers.is_null() && (*info_ptr).row_pointers != row_pointers {
        png_free_data(png_ptr, info_ptr, PNG_FREE_ROWS, 0 as c_int);
    }
    (*info_ptr).row_pointers = row_pointers;
    if !row_pointers.is_null() {
        (*info_ptr).valid |= PNG_INFO_IDAT;
    }
} }
#[inline]
pub fn png_set_compression_buffer_size(
    mut png_ptr: png_structrp,
    mut size: png_size_t,
) { unsafe {
    if png_ptr.is_null() {
        return;
    }
    if size == 0 as png_size_t || size > PNG_UINT_31_MAX as png_size_t {
        png_error(
            png_ptr,
            b"invalid compression buffer size\0" as *const u8 as png_const_charp,
        );
    }
    if (*png_ptr).mode as c_uint & PNG_IS_READ_STRUCT != 0 as c_uint {
        (*png_ptr).IDAT_read_size = size as png_uint_32 as uInt;
        return;
    }
    if (*png_ptr).mode as c_uint & PNG_IS_READ_STRUCT == 0 as c_uint {
        if (*png_ptr).zowner != 0 as c_uint {
            png_warning(
                png_ptr,
                b"Compression buffer size cannot be changed because it is in use\0" as *const u8
                    as png_const_charp,
            );
            return;
        }
        if size > ZLIB_IO_MAX as png_size_t {
            png_warning(
                png_ptr,
                b"Compression buffer size limited to system maximum\0" as *const u8
                    as png_const_charp,
            );
            size = ZLIB_IO_MAX as png_size_t;
        }
        if size < 6 as png_size_t {
            png_warning(
                png_ptr,
                b"Compression buffer size cannot be reduced below 6\0" as *const u8
                    as png_const_charp,
            );
            return;
        }
        if (*png_ptr).zbuffer_size as png_size_t != size {
            png_free_buffer_list(png_ptr, &raw mut (*png_ptr).zbuffer_list);
            (*png_ptr).zbuffer_size = size as uInt;
        }
    }
} }
#[no_mangle]
pub extern "C" fn png_set_invalid(
    mut png_ptr: png_const_structrp,
    mut info_ptr: png_inforp,
    mut mask: c_int,
) { unsafe {
    if !png_ptr.is_null() && !info_ptr.is_null() {
        (*info_ptr).valid &= !mask as c_uint;
    }
} }
#[no_mangle]
pub extern "C" fn png_set_user_limits(
    mut png_ptr: png_structrp,
    mut user_width_max: png_uint_32,
    mut user_height_max: png_uint_32,
) { unsafe {
    if png_ptr.is_null() {
        return;
    }
    (*png_ptr).user_width_max = user_width_max;
    (*png_ptr).user_height_max = user_height_max;
} }
#[inline]
pub fn png_set_chunk_cache_max(
    mut png_ptr: png_structrp,
    mut user_chunk_cache_max: png_uint_32,
) { unsafe {
    if !png_ptr.is_null() {
        (*png_ptr).user_chunk_cache_max = user_chunk_cache_max;
    }
} }
#[inline]
pub fn png_set_chunk_malloc_max(
    mut png_ptr: png_structrp,
    mut user_chunk_malloc_max: png_alloc_size_t,
) { unsafe {
    if !png_ptr.is_null() {
        (*png_ptr).user_chunk_malloc_max = user_chunk_malloc_max;
    }
} }
#[inline]
pub fn png_set_benign_errors(
    mut png_ptr: png_structrp,
    mut allowed: c_int,
) { unsafe {
    if allowed != 0 as c_int {
        (*png_ptr).flags |=
            PNG_FLAG_BENIGN_ERRORS_WARN | PNG_FLAG_APP_WARNINGS_WARN | PNG_FLAG_APP_ERRORS_WARN;
    } else {
        (*png_ptr).flags &=
            !(PNG_FLAG_BENIGN_ERRORS_WARN | PNG_FLAG_APP_WARNINGS_WARN | PNG_FLAG_APP_ERRORS_WARN);
    };
} }
#[inline]
pub fn png_set_check_for_invalid_index(
    mut png_ptr: png_structrp,
    mut allowed: c_int,
) { unsafe {
    if allowed > 0 as c_int {
        (*png_ptr).num_palette_max = 0 as c_int;
    } else {
        (*png_ptr).num_palette_max = -(1 as c_int);
    };
} }

pub const UINT_MAX: c_uint = (__INT_MAX__ as c_uint)
    .wrapping_mul(2 as c_uint)
    .wrapping_add(1 as c_uint);
