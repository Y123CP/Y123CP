use core::ffi::*;
pub use crate::src::c_consts::*;

#[inline]
pub fn BrotliDecoderEnsureStaticInit() -> c_int { {
    return BROTLI_TRUE;
} }
