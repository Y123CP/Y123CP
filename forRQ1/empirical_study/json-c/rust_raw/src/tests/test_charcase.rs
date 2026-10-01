extern "C" {
    pub type json_object;
    fn __assert_fail(
        __assertion: *const ::core::ffi::c_char,
        __file: *const ::core::ffi::c_char,
        __line: ::core::ffi::c_uint,
        __function: *const ::core::ffi::c_char,
    ) -> !;
    fn printf(__format: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn mc_set_debug(debug: ::core::ffi::c_int);
    fn json_tokener_new() -> *mut json_tokener;
    fn json_tokener_free(tok: *mut json_tokener);
    fn json_tokener_set_flags(tok: *mut json_tokener, flags: ::core::ffi::c_int);
    fn json_tokener_parse_ex(
        tok: *mut json_tokener,
        str: *const ::core::ffi::c_char,
        len: ::core::ffi::c_int,
    ) -> *mut json_object;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct printbuf {
    pub buf: *mut ::core::ffi::c_char,
    pub bpos: ::core::ffi::c_int,
    pub size: ::core::ffi::c_int,
}
pub type json_tokener_error = ::core::ffi::c_uint;
pub const json_tokener_error_size: json_tokener_error = 16;
pub const json_tokener_error_memory: json_tokener_error = 15;
pub const json_tokener_error_parse_utf8_string: json_tokener_error = 14;
pub const json_tokener_error_parse_comment: json_tokener_error = 13;
pub const json_tokener_error_parse_string: json_tokener_error = 12;
pub const json_tokener_error_parse_object_value_sep: json_tokener_error = 11;
pub const json_tokener_error_parse_object_key_sep: json_tokener_error = 10;
pub const json_tokener_error_parse_object_key_name: json_tokener_error = 9;
pub const json_tokener_error_parse_array: json_tokener_error = 8;
pub const json_tokener_error_parse_number: json_tokener_error = 7;
pub const json_tokener_error_parse_boolean: json_tokener_error = 6;
pub const json_tokener_error_parse_null: json_tokener_error = 5;
pub const json_tokener_error_parse_unexpected: json_tokener_error = 4;
pub const json_tokener_error_parse_eof: json_tokener_error = 3;
pub const json_tokener_error_depth: json_tokener_error = 2;
pub const json_tokener_continue: json_tokener_error = 1;
pub const json_tokener_success: json_tokener_error = 0;
pub type json_tokener_state = ::core::ffi::c_uint;
pub const json_tokener_state_inf: json_tokener_state = 26;
pub const json_tokener_state_object_field_start_after_sep: json_tokener_state = 25;
pub const json_tokener_state_array_after_sep: json_tokener_state = 24;
pub const json_tokener_state_object_sep: json_tokener_state = 23;
pub const json_tokener_state_object_value_add: json_tokener_state = 22;
pub const json_tokener_state_object_value: json_tokener_state = 21;
pub const json_tokener_state_object_field_end: json_tokener_state = 20;
pub const json_tokener_state_object_field: json_tokener_state = 19;
pub const json_tokener_state_object_field_start: json_tokener_state = 18;
pub const json_tokener_state_array_sep: json_tokener_state = 17;
pub const json_tokener_state_array_add: json_tokener_state = 16;
pub const json_tokener_state_array: json_tokener_state = 15;
pub const json_tokener_state_number: json_tokener_state = 14;
pub const json_tokener_state_boolean: json_tokener_state = 13;
pub const json_tokener_state_escape_unicode_need_u: json_tokener_state = 12;
pub const json_tokener_state_escape_unicode_need_escape: json_tokener_state = 11;
pub const json_tokener_state_escape_unicode: json_tokener_state = 10;
pub const json_tokener_state_string_escape: json_tokener_state = 9;
pub const json_tokener_state_string: json_tokener_state = 8;
pub const json_tokener_state_comment_end: json_tokener_state = 7;
pub const json_tokener_state_comment_eol: json_tokener_state = 6;
pub const json_tokener_state_comment: json_tokener_state = 5;
pub const json_tokener_state_comment_start: json_tokener_state = 4;
pub const json_tokener_state_null: json_tokener_state = 3;
pub const json_tokener_state_finish: json_tokener_state = 2;
pub const json_tokener_state_start: json_tokener_state = 1;
pub const json_tokener_state_eatws: json_tokener_state = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct json_tokener_srec {
    pub state: json_tokener_state,
    pub saved_state: json_tokener_state,
    pub obj: *mut json_object,
    pub current: *mut json_object,
    pub obj_field_name: *mut ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct json_tokener {
    pub str_0: *mut ::core::ffi::c_char,
    pub pb: *mut printbuf,
    pub max_depth: ::core::ffi::c_int,
    pub depth: ::core::ffi::c_int,
    pub is_double: ::core::ffi::c_int,
    pub st_pos: ::core::ffi::c_int,
    pub char_offset: ::core::ffi::c_int,
    pub err: json_tokener_error,
    pub ucs_char: ::core::ffi::c_uint,
    pub high_surrogate: ::core::ffi::c_uint,
    pub quote_char: ::core::ffi::c_char,
    pub stack: *mut json_tokener_srec,
    pub flags: ::core::ffi::c_int,
}
pub const __ASSERT_FUNCTION: [::core::ffi::c_char; 27] = unsafe {
    ::core::mem::transmute::<[u8; 27], [::core::ffi::c_char; 27]>(*b"void test_case_parse(void)\0")
};
pub const JSON_TOKENER_STRICT: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
unsafe fn main_0(
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    test_case_parse();
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn test_case_parse() {
    let mut tok: *mut json_tokener = ::core::ptr::null_mut::<json_tokener>();
    let mut new_obj: *mut json_object = ::core::ptr::null_mut::<json_object>();
    tok = json_tokener_new();
    json_tokener_set_flags(tok, JSON_TOKENER_STRICT);
    new_obj = json_tokener_parse_ex(
        tok,
        b"True\0" as *const u8 as *const ::core::ffi::c_char,
        4 as ::core::ffi::c_int,
    ) as *mut json_object;
    '_c2rust_label: {
        if new_obj.is_null() {
        } else {
            __assert_fail(
                b"new_obj == NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_charcase.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                34 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    new_obj = json_tokener_parse_ex(
        tok,
        b"False\0" as *const u8 as *const ::core::ffi::c_char,
        5 as ::core::ffi::c_int,
    ) as *mut json_object;
    '_c2rust_label_0: {
        if new_obj.is_null() {
        } else {
            __assert_fail(
                b"new_obj == NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_charcase.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                37 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    new_obj = json_tokener_parse_ex(
        tok,
        b"Null\0" as *const u8 as *const ::core::ffi::c_char,
        4 as ::core::ffi::c_int,
    ) as *mut json_object;
    '_c2rust_label_1: {
        if new_obj.is_null() {
        } else {
            __assert_fail(
                b"new_obj == NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_charcase.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                40 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    printf(b"OK\n\0" as *const u8 as *const ::core::ffi::c_char);
    json_tokener_free(tok);
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
