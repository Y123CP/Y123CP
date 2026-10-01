use core::ffi::*;
pub use crate::src::c_consts::*;

#[inline]
pub fn BrotliEncoderEnsureStaticInit() -> c_int { {
    return BROTLI_TRUE;
} }
