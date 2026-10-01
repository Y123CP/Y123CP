extern "C" {
    fn gzclose_r(file: gzFile) -> ::core::ffi::c_int;
}
pub type __off_t = ::core::ffi::c_long;
pub type off_t = __off_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gzFile_s {
    pub have: ::core::ffi::c_uint,
    pub next: *mut ::core::ffi::c_uchar,
    pub pos: off_t,
}
pub type gzFile = *mut gzFile_s;
#[no_mangle]
pub unsafe extern "C" fn gzclose(mut file: gzFile) -> ::core::ffi::c_int {
    return gzclose_r(file);
}
