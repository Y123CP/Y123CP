extern "C" {
    pub type png_struct_def;
    pub type png_info_def;
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
}
pub type png_uint_32 = ::core::ffi::c_uint;
pub type png_struct = png_struct_def;
pub type png_structp = *mut png_struct;
pub type png_info = png_info_def;
pub type png_infop = *mut png_info;
pub type png_const_structrp = *const png_struct;
pub type png_inforp = *mut png_info;
pub type png_const_inforp = *const png_info;
#[no_mangle]
pub unsafe extern "C" fn pngx_set_compression_type(
    mut png_ptr: png_structp,
    mut info_ptr: png_infop,
    mut compression_type: ::core::ffi::c_int,
) {
    let mut width: png_uint_32 = 0;
    let mut height: png_uint_32 = 0;
    let mut bit_depth: ::core::ffi::c_int = 0;
    let mut color_type: ::core::ffi::c_int = 0;
    let mut interlace_type: ::core::ffi::c_int = 0;
    let mut filter_type: ::core::ffi::c_int = 0;
    let mut old_compression_type: ::core::ffi::c_int = 0;
    if png_get_IHDR(
        png_ptr as png_const_structrp,
        info_ptr as png_const_inforp,
        &raw mut width,
        &raw mut height,
        &raw mut bit_depth,
        &raw mut color_type,
        &raw mut interlace_type,
        &raw mut old_compression_type,
        &raw mut filter_type,
    ) == 0
    {
        return;
    }
    if compression_type == old_compression_type {
        return;
    }
    png_set_IHDR(
        png_ptr as png_const_structrp,
        info_ptr as png_inforp,
        width,
        height,
        bit_depth,
        color_type,
        interlace_type,
        compression_type,
        filter_type,
    );
}
#[no_mangle]
pub unsafe extern "C" fn pngx_set_filter_type(
    mut png_ptr: png_structp,
    mut info_ptr: png_infop,
    mut filter_type: ::core::ffi::c_int,
) {
    let mut width: png_uint_32 = 0;
    let mut height: png_uint_32 = 0;
    let mut bit_depth: ::core::ffi::c_int = 0;
    let mut color_type: ::core::ffi::c_int = 0;
    let mut interlace_type: ::core::ffi::c_int = 0;
    let mut compression_type: ::core::ffi::c_int = 0;
    let mut old_filter_type: ::core::ffi::c_int = 0;
    if png_get_IHDR(
        png_ptr as png_const_structrp,
        info_ptr as png_const_inforp,
        &raw mut width,
        &raw mut height,
        &raw mut bit_depth,
        &raw mut color_type,
        &raw mut interlace_type,
        &raw mut compression_type,
        &raw mut old_filter_type,
    ) == 0
    {
        return;
    }
    if filter_type == old_filter_type {
        return;
    }
    png_set_IHDR(
        png_ptr as png_const_structrp,
        info_ptr as png_inforp,
        width,
        height,
        bit_depth,
        color_type,
        interlace_type,
        compression_type,
        filter_type,
    );
}
#[no_mangle]
pub unsafe extern "C" fn pngx_set_interlace_type(
    mut png_ptr: png_structp,
    mut info_ptr: png_infop,
    mut interlace_type: ::core::ffi::c_int,
) {
    let mut width: png_uint_32 = 0;
    let mut height: png_uint_32 = 0;
    let mut bit_depth: ::core::ffi::c_int = 0;
    let mut color_type: ::core::ffi::c_int = 0;
    let mut compression_type: ::core::ffi::c_int = 0;
    let mut filter_type: ::core::ffi::c_int = 0;
    let mut old_interlace_type: ::core::ffi::c_int = 0;
    if png_get_IHDR(
        png_ptr as png_const_structrp,
        info_ptr as png_const_inforp,
        &raw mut width,
        &raw mut height,
        &raw mut bit_depth,
        &raw mut color_type,
        &raw mut old_interlace_type,
        &raw mut compression_type,
        &raw mut filter_type,
    ) == 0
    {
        return;
    }
    if interlace_type == old_interlace_type {
        return;
    }
    png_set_IHDR(
        png_ptr as png_const_structrp,
        info_ptr as png_inforp,
        width,
        height,
        bit_depth,
        color_type,
        interlace_type,
        compression_type,
        filter_type,
    );
}
