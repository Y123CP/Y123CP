pub const JSON_C_MAJOR_VERSION: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const JSON_C_MINOR_VERSION: ::core::ffi::c_int = 17 as ::core::ffi::c_int;
pub const JSON_C_MICRO_VERSION: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const JSON_C_VERSION_NUM: ::core::ffi::c_int = JSON_C_MAJOR_VERSION << 16 as ::core::ffi::c_int
    | JSON_C_MINOR_VERSION << 8 as ::core::ffi::c_int
    | JSON_C_MICRO_VERSION;
pub const JSON_C_VERSION: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"0.17\0") };
#[no_mangle]
pub unsafe extern "C" fn json_c_version() -> *const ::core::ffi::c_char {
    return JSON_C_VERSION.as_ptr();
}
#[no_mangle]
pub unsafe extern "C" fn json_c_version_num() -> ::core::ffi::c_int {
    return JSON_C_VERSION_NUM;
}
