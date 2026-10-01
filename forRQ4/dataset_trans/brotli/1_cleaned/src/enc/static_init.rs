use core::ffi::*;
pub use crate::src::c_consts::*;

#[no_mangle]
pub unsafe extern "C" fn BrotliEncoderEnsureStaticInit() -> c_int {
    return BROTLI_TRUE;
}
