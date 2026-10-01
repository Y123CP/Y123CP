extern "C" {
    fn heman_image_create(
        width: ::core::ffi::c_int,
        height: ::core::ffi::c_int,
        nbands: ::core::ffi::c_int,
    ) -> *mut heman_image;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct heman_image_s {
    pub width: ::core::ffi::c_int,
    pub height: ::core::ffi::c_int,
    pub nbands: ::core::ffi::c_int,
    pub data: *mut ::core::ffi::c_float,
}
pub type heman_image = heman_image_s;
pub type heman_byte = ::core::ffi::c_uchar;
#[no_mangle]
pub unsafe extern "C" fn heman_import_u8(
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
    mut nbands: ::core::ffi::c_int,
    mut source: *const heman_byte,
    mut minval: ::core::ffi::c_float,
    mut maxval: ::core::ffi::c_float,
) -> *mut heman_image {
    let mut result: *mut heman_image = heman_image_create(width, height, nbands);
    let mut inp: *const heman_byte = source;
    let mut outp: *mut ::core::ffi::c_float = (*result).data;
    let mut scale: ::core::ffi::c_float = (maxval - minval) / 255.0f32;
    let mut size: ::core::ffi::c_int = height * width * nbands;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < size {
        let fresh0 = inp;
        inp = inp.offset(1);
        let mut v: ::core::ffi::c_float =
            *fresh0 as ::core::ffi::c_int as ::core::ffi::c_float * scale + minval;
        let fresh1 = outp;
        outp = outp.offset(1);
        *fresh1 = if minval > (if maxval > v { v } else { maxval }) {
            minval
        } else if maxval > v {
            v
        } else {
            maxval
        };
        i += 1;
    }
    return result;
}
