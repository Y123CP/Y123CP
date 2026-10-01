use core::ffi::*;
pub use crate::src::ffi::*;
pub use crate::src::c_consts::*;
pub use crate::src::c_types::*;
extern "C" {
    pub type png_struct_def;
    pub type png_info_def;
    fn png_malloc(png_ptr: png_const_structrp, size: png_alloc_size_t) -> png_voidp;
    fn png_free(png_ptr: png_const_structrp, ptr: png_voidp);
    fn png_free_data(
        png_ptr: png_const_structrp,
        info_ptr: png_inforp,
        free_me: png_uint_32,
        num: c_int,
    );
    fn png_error(png_ptr: png_const_structrp, error_message: png_const_charp) -> !;
    fn png_get_rowbytes(png_ptr: png_const_structrp, info_ptr: png_const_inforp) -> png_size_t;
    fn png_set_rows(png_ptr: png_const_structrp, info_ptr: png_inforp, row_pointers: png_bytepp);
    fn png_get_image_height(png_ptr: png_const_structrp, info_ptr: png_const_inforp)
        -> png_uint_32;
}

pub type png_struct = png_struct_def;
pub type png_structp = *mut png_struct;
pub type png_info = png_info_def;
pub type png_infop = *mut png_info;
pub type png_const_structrp = *const png_struct;
pub type png_inforp = *mut png_info;
pub type png_const_inforp = *const png_info;

#[no_mangle]
pub unsafe extern "C" fn pngx_malloc_rows(
    mut png_ptr: png_structp,
    mut info_ptr: png_infop,
    mut filler: c_int,
) -> png_bytepp {
    return pngx_malloc_rows_extended(png_ptr, info_ptr, 0 as pngx_alloc_size_t, filler);
}
#[no_mangle]
pub unsafe extern "C" fn pngx_malloc_rows_extended(
    mut png_ptr: png_structp,
    mut info_ptr: png_infop,
    mut min_row_size: pngx_alloc_size_t,
    mut filler: c_int,
) -> png_bytepp {
    let mut row_size: pngx_alloc_size_t = 0;
    let mut row: png_bytep = ::core::ptr::null_mut::<png_byte>();
    let mut rows: png_bytepp = ::core::ptr::null_mut::<*mut png_byte>();
    let mut height: png_uint_32 = 0;
    let mut i: png_uint_32 = 0;
    height = png_get_image_height(png_ptr as png_const_structrp, info_ptr as png_const_inforp);
    if height == 0 as c_uint {
        png_error(
            png_ptr as png_const_structrp,
            b"Missing IHDR\0" as *const u8 as png_const_charp,
        );
    }
    row_size = png_get_rowbytes(png_ptr as png_const_structrp, info_ptr as png_const_inforp)
        as pngx_alloc_size_t;
    if row_size == 0 as pngx_alloc_size_t
        || height as pngx_alloc_size_t
            > (-(1 as c_int) as pngx_alloc_size_t)
                .wrapping_div(::core::mem::size_of::<png_bytep>() as pngx_alloc_size_t)
    {
        png_error(
            png_ptr as png_const_structrp,
            b"Can't handle exceedingly large image dimensions\0" as *const u8 as png_const_charp,
        );
    }
    if row_size < min_row_size {
        row_size = min_row_size;
    }
    png_free_data(
        png_ptr as png_const_structrp,
        info_ptr as png_inforp,
        PNG_FREE_ROWS,
        0 as c_int,
    );
    rows = png_malloc(
        png_ptr as png_const_structrp,
        (height as usize).wrapping_mul(::core::mem::size_of::<png_bytep>() as usize),
    ) as png_bytepp;
    if rows.is_null() {
        return ::core::ptr::null_mut::<*mut png_byte>();
    }
    i = 0 as png_uint_32;
    while i < height {
        row = png_malloc(png_ptr as png_const_structrp, row_size as png_alloc_size_t) as png_bytep;
        if row.is_null() {
            while i > 0 as c_uint {
                i = i.wrapping_sub(1);
                png_free(
                    png_ptr as png_const_structrp,
                    *rows.offset(i as isize) as png_voidp,
                );
            }
            png_free(png_ptr as png_const_structrp, rows as png_voidp);
            return ::core::ptr::null_mut::<*mut png_byte>();
        }
        if filler >= 0 as c_int {
            memset(row as *mut c_void, filler, row_size as size_t);
        }
        let ref mut fresh0 = *rows.offset(i as isize);
        *fresh0 = row as *mut png_byte;
        i = i.wrapping_add(1);
    }
    png_set_rows(png_ptr as png_const_structrp, info_ptr as png_inforp, rows);
    return rows;
}
