use core::ffi::*;
pub use crate::src::ffi::*;
pub use crate::src::c_types::*;

#[no_mangle]
pub unsafe extern "C" fn BrotliDefaultAllocFunc(
    mut opaque: *mut c_void,
    mut size: size_t,
) -> *mut c_void {
    return malloc(size);
}
#[no_mangle]
pub unsafe extern "C" fn BrotliDefaultFreeFunc(
    mut opaque: *mut c_void,
    mut address: *mut c_void,
) {
    free(address);
}
