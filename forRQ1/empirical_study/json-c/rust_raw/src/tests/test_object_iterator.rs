extern "C" {
    pub type json_object;
    fn printf(__format: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn json_object_iter_init_default() -> json_object_iterator;
    fn json_object_iter_begin(obj: *mut json_object) -> json_object_iterator;
    fn json_object_iter_end(obj: *const json_object) -> json_object_iterator;
    fn json_object_iter_next(iter: *mut json_object_iterator);
    fn json_object_iter_peek_name(iter: *const json_object_iterator) -> *const ::core::ffi::c_char;
    fn json_object_iter_peek_value(iter: *const json_object_iterator) -> *mut json_object;
    fn json_object_iter_equal(
        iter1: *const json_object_iterator,
        iter2: *const json_object_iterator,
    ) -> json_bool;
    fn json_object_put(obj: *mut json_object) -> ::core::ffi::c_int;
    fn json_object_to_json_string(obj: *mut json_object) -> *const ::core::ffi::c_char;
    fn json_tokener_parse(str: *const ::core::ffi::c_char) -> *mut json_object;
}
pub type json_bool = ::core::ffi::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct json_object_iterator {
    pub opaque_: *const ::core::ffi::c_void,
}
unsafe fn main_0(
    mut atgc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut input: *const ::core::ffi::c_char = b"{\n\t\t\"string_of_digits\": \"123\",\n\t\t\"regular_number\": 222,\n\t\t\"decimal_number\": 99.55,\n\t\t\"boolean_true\": true,\n\t\t\"boolean_false\": false,\n\t\t\"big_number\": 2147483649,\n\t\t\"a_null\": null,\n\t\t}\0"
        as *const u8 as *const ::core::ffi::c_char;
    let mut new_obj: *mut json_object = ::core::ptr::null_mut::<json_object>();
    let mut it: json_object_iterator = json_object_iterator {
        opaque_: ::core::ptr::null::<::core::ffi::c_void>(),
    };
    let mut itEnd: json_object_iterator = json_object_iterator {
        opaque_: ::core::ptr::null::<::core::ffi::c_void>(),
    };
    it = json_object_iter_init_default();
    new_obj = json_tokener_parse(input);
    it = json_object_iter_begin(new_obj);
    itEnd = json_object_iter_end(new_obj);
    while json_object_iter_equal(&raw mut it, &raw mut itEnd) == 0 {
        printf(
            b"%s\n\0" as *const u8 as *const ::core::ffi::c_char,
            json_object_iter_peek_name(&raw mut it),
        );
        printf(
            b"%s\n\0" as *const u8 as *const ::core::ffi::c_char,
            json_object_to_json_string(json_object_iter_peek_value(&raw mut it)),
        );
        json_object_iter_next(&raw mut it);
    }
    json_object_put(new_obj);
    return 0 as ::core::ffi::c_int;
}
pub fn main() {
    let mut args_strings: Vec<Vec<u8>> = ::std::env::args()
        .map(|arg| {
            ::std::ffi::CString::new(arg)
                .expect("Failed to convert argument into CString.")
                .into_bytes_with_nul()
        })
        .collect();
    let mut args_ptrs: Vec<*mut ::core::ffi::c_char> = args_strings
        .iter_mut()
        .map(|arg| arg.as_mut_ptr() as *mut ::core::ffi::c_char)
        .chain(::core::iter::once(::core::ptr::null_mut()))
        .collect();
    unsafe {
        ::std::process::exit(main_0(
            (args_ptrs.len() - 1) as ::core::ffi::c_int,
            args_ptrs.as_mut_ptr() as *mut *mut ::core::ffi::c_char,
        ) as i32)
    }
}
