pub const BROTLI_TRUE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn BrotliEncoderEnsureStaticInit() -> ::core::ffi::c_int {
    return BROTLI_TRUE;
}
