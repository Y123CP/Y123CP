extern "C" {
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
}
pub type size_t = usize;
#[no_mangle]
pub unsafe extern "C" fn BrotliDefaultAllocFunc(
    mut opaque: *mut ::core::ffi::c_void,
    mut size: size_t,
) -> *mut ::core::ffi::c_void {
    return malloc(size);
}
#[no_mangle]
pub unsafe extern "C" fn BrotliDefaultFreeFunc(
    mut opaque: *mut ::core::ffi::c_void,
    mut address: *mut ::core::ffi::c_void,
) {
    free(address);
}
