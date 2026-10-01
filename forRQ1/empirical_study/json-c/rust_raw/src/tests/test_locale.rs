extern "C" {
    pub type json_object;
    fn printf(__format: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn mc_set_debug(debug: ::core::ffi::c_int);
    fn json_object_put(obj: *mut json_object) -> ::core::ffi::c_int;
    fn json_object_to_json_string_ext(
        obj: *mut json_object,
        flags: ::core::ffi::c_int,
    ) -> *const ::core::ffi::c_char;
    fn json_object_array_length(obj: *const json_object) -> size_t;
    fn json_object_array_get_idx(obj: *const json_object, idx: size_t) -> *mut json_object;
    fn json_object_get_double(obj: *const json_object) -> ::core::ffi::c_double;
    fn json_tokener_parse(str: *const ::core::ffi::c_char) -> *mut json_object;
    fn setlocale(
        __category: ::core::ffi::c_int,
        __locale: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
}
pub type size_t = usize;
pub const __LC_NUMERIC: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const JSON_C_TO_STRING_NOZERO: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int;
pub const LC_NUMERIC: ::core::ffi::c_int = __LC_NUMERIC;
unsafe fn main_0(
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut new_obj: *mut json_object = ::core::ptr::null_mut::<json_object>();
    setlocale(
        LC_NUMERIC,
        b"de_DE\0" as *const u8 as *const ::core::ffi::c_char,
    );
    let mut buf1: [::core::ffi::c_char; 10] = [0; 10];
    let mut buf2: [::core::ffi::c_char; 10] = [0; 10];
    snprintf(
        &raw mut buf1 as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 10]>() as size_t,
        b"%f\0" as *const u8 as *const ::core::ffi::c_char,
        0.1f64,
    );
    new_obj = json_tokener_parse(
        b"[1.2,3.4,123456.78,5.0,2.3e10]\0" as *const u8 as *const ::core::ffi::c_char,
    ) as *mut json_object;
    snprintf(
        &raw mut buf2 as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 10]>() as size_t,
        b"%f\0" as *const u8 as *const ::core::ffi::c_char,
        0.1f64,
    );
    if strcmp(
        &raw mut buf1 as *mut ::core::ffi::c_char,
        &raw mut buf2 as *mut ::core::ffi::c_char,
    ) != 0 as ::core::ffi::c_int
    {
        printf(
            b"ERROR: Original locale not restored \"%s\" != \"%s\"\0" as *const u8
                as *const ::core::ffi::c_char,
            &raw mut buf1 as *mut ::core::ffi::c_char,
            &raw mut buf2 as *mut ::core::ffi::c_char,
        );
    }
    setlocale(
        LC_NUMERIC,
        b"C\0" as *const u8 as *const ::core::ffi::c_char,
    );
    printf(b"new_obj.to_string()=[\0" as *const u8 as *const ::core::ffi::c_char);
    let mut ii: ::core::ffi::c_uint = 0;
    ii = 0 as ::core::ffi::c_uint;
    while (ii as size_t) < json_object_array_length(new_obj) {
        let mut val: *mut json_object =
            json_object_array_get_idx(new_obj, ii as size_t) as *mut json_object;
        printf(
            b"%s%.2lf\0" as *const u8 as *const ::core::ffi::c_char,
            if ii > 0 as ::core::ffi::c_uint {
                b",\0" as *const u8 as *const ::core::ffi::c_char
            } else {
                b"\0" as *const u8 as *const ::core::ffi::c_char
            },
            json_object_get_double(val),
        );
        ii = ii.wrapping_add(1);
    }
    printf(b"]\n\0" as *const u8 as *const ::core::ffi::c_char);
    printf(
        b"new_obj.to_string()=%s\n\0" as *const u8 as *const ::core::ffi::c_char,
        json_object_to_json_string_ext(new_obj as *mut json_object, JSON_C_TO_STRING_NOZERO),
    );
    json_object_put(new_obj as *mut json_object);
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
