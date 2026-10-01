extern "C" {
    pub type json_object;
    fn json_object_put(obj: *mut json_object) -> ::core::ffi::c_int;
    fn json_object_to_json_string_ext(
        obj: *mut json_object,
        flags: ::core::ffi::c_int,
    ) -> *const ::core::ffi::c_char;
    fn json_object_new_double(d: ::core::ffi::c_double) -> *mut json_object;
    fn printf(__format: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
}
pub const JSON_C_TO_STRING_PRETTY: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int;
unsafe fn main_0() -> ::core::ffi::c_int {
    let mut json: *mut json_object = ::core::ptr::null_mut::<json_object>();
    json = json_object_new_double(1.0f64) as *mut json_object;
    printf(
        b"json = %s\n\0" as *const u8 as *const ::core::ffi::c_char,
        json_object_to_json_string_ext(json as *mut json_object, JSON_C_TO_STRING_PRETTY),
    );
    json_object_put(json as *mut json_object);
    json = json_object_new_double(-1.0f64) as *mut json_object;
    printf(
        b"json = %s\n\0" as *const u8 as *const ::core::ffi::c_char,
        json_object_to_json_string_ext(json as *mut json_object, JSON_C_TO_STRING_PRETTY),
    );
    json_object_put(json as *mut json_object);
    json = json_object_new_double(1.23f64) as *mut json_object;
    printf(
        b"json = %s\n\0" as *const u8 as *const ::core::ffi::c_char,
        json_object_to_json_string_ext(json as *mut json_object, JSON_C_TO_STRING_PRETTY),
    );
    json_object_put(json as *mut json_object);
    json = json_object_new_double(123456789.0f64) as *mut json_object;
    printf(
        b"json = %s\n\0" as *const u8 as *const ::core::ffi::c_char,
        json_object_to_json_string_ext(json as *mut json_object, JSON_C_TO_STRING_PRETTY),
    );
    json_object_put(json as *mut json_object);
    json = json_object_new_double(123456789.123f64) as *mut json_object;
    printf(
        b"json = %s\n\0" as *const u8 as *const ::core::ffi::c_char,
        json_object_to_json_string_ext(json as *mut json_object, JSON_C_TO_STRING_PRETTY),
    );
    json_object_put(json as *mut json_object);
    return 0 as ::core::ffi::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
