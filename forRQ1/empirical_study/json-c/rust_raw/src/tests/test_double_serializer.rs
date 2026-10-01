extern "C" {
    fn printf(__format: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn json_object_put(obj: *mut json_object) -> ::core::ffi::c_int;
    fn json_object_to_json_string(obj: *mut json_object) -> *const ::core::ffi::c_char;
    fn json_object_to_json_string_ext(
        obj: *mut json_object,
        flags: ::core::ffi::c_int,
    ) -> *const ::core::ffi::c_char;
    fn json_object_set_serializer(
        jso: *mut json_object,
        to_string_func: Option<json_object_to_json_string_fn>,
        userdata: *mut ::core::ffi::c_void,
        user_delete: Option<json_object_delete_fn>,
    );
    fn json_object_new_double(d: ::core::ffi::c_double) -> *mut json_object;
    fn json_c_set_serialization_double_format(
        double_format: *const ::core::ffi::c_char,
        global_or_thread: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn json_object_double_to_json_string(
        jso: *mut json_object,
        pb: *mut printbuf,
        level: ::core::ffi::c_int,
        flags: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
}
pub type __uint32_t = u32;
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct printbuf {
    pub buf: *mut ::core::ffi::c_char,
    pub bpos: ::core::ffi::c_int,
    pub size: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct json_object {
    pub o_type: json_type,
    pub _ref_count: uint32_t,
    pub _to_json_string: Option<json_object_to_json_string_fn>,
    pub _pb: *mut printbuf,
    pub _user_delete: Option<json_object_delete_fn>,
    pub _userdata: *mut ::core::ffi::c_void,
}
pub type json_object_delete_fn =
    unsafe extern "C" fn(*mut json_object, *mut ::core::ffi::c_void) -> ();
pub type json_object_to_json_string_fn = unsafe extern "C" fn(
    *mut json_object,
    *mut printbuf,
    ::core::ffi::c_int,
    ::core::ffi::c_int,
) -> ::core::ffi::c_int;
pub type json_type = ::core::ffi::c_uint;
pub const json_type_string: json_type = 6;
pub const json_type_array: json_type = 5;
pub const json_type_object: json_type = 4;
pub const json_type_int: json_type = 3;
pub const json_type_double: json_type = 2;
pub const json_type_boolean: json_type = 1;
pub const json_type_null: json_type = 0;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const JSON_C_OPTION_GLOBAL: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const JSON_C_OPTION_THREAD: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
#[no_mangle]
pub static mut zero_dot_zero: ::core::ffi::c_double = 0.0f64;
unsafe fn main_0(
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut obj: *mut json_object = json_object_new_double(0.5f64);
    let mut udata: [::core::ffi::c_char; 5] =
        ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"test\0");
    printf(b"Test default serializer:\n\0" as *const u8 as *const ::core::ffi::c_char);
    printf(
        b"obj.to_string(standard)=%s\n\0" as *const u8 as *const ::core::ffi::c_char,
        json_object_to_json_string(obj),
    );
    printf(
        b"Test default serializer with custom userdata:\n\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    (*obj)._userdata = &raw mut udata as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void;
    printf(
        b"obj.to_string(userdata)=%s\n\0" as *const u8 as *const ::core::ffi::c_char,
        json_object_to_json_string(obj),
    );
    printf(
        b"Test explicit serializer with custom userdata:\n\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    json_object_set_serializer(
        obj as *mut json_object,
        Some(
            json_object_double_to_json_string
                as unsafe extern "C" fn(
                    *mut json_object,
                    *mut printbuf,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        &raw mut udata as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        None,
    );
    printf(
        b"obj.to_string(custom)=%s\n\0" as *const u8 as *const ::core::ffi::c_char,
        json_object_to_json_string(obj),
    );
    printf(b"Test reset serializer:\n\0" as *const u8 as *const ::core::ffi::c_char);
    json_object_set_serializer(obj as *mut json_object, None, NULL, None);
    printf(
        b"obj.to_string(reset)=%s\n\0" as *const u8 as *const ::core::ffi::c_char,
        json_object_to_json_string(obj),
    );
    json_object_put(obj);
    printf(b"Test no zero reset serializer:\n\0" as *const u8 as *const ::core::ffi::c_char);
    obj = json_object_new_double(3.1415000f64);
    let mut data: [::core::ffi::c_char; 6] =
        ::core::mem::transmute::<[u8; 6], [::core::ffi::c_char; 6]>(*b"%.17g\0");
    json_object_set_serializer(
        obj as *mut json_object,
        Some(
            json_object_double_to_json_string
                as unsafe extern "C" fn(
                    *mut json_object,
                    *mut printbuf,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        &raw mut data as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        None,
    );
    printf(
        b"obj.to_string(reset)=%s\n\0" as *const u8 as *const ::core::ffi::c_char,
        json_object_to_json_string_ext(obj, 4 as ::core::ffi::c_int),
    );
    json_object_put(obj);
    obj = json_object_new_double(0.52381f64);
    printf(
        b"obj.to_string(default format)=%s\n\0" as *const u8 as *const ::core::ffi::c_char,
        json_object_to_json_string(obj),
    );
    if json_c_set_serialization_double_format(
        b"x%0.3fy\0" as *const u8 as *const ::core::ffi::c_char,
        JSON_C_OPTION_GLOBAL,
    ) < 0 as ::core::ffi::c_int
    {
        printf(
            b"ERROR: json_c_set_serialization_double_format() failed\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    printf(
        b"obj.to_string(with global format)=%s\n\0" as *const u8 as *const ::core::ffi::c_char,
        json_object_to_json_string(obj),
    );
    if json_c_set_serialization_double_format(
        b"T%0.2fX\0" as *const u8 as *const ::core::ffi::c_char,
        JSON_C_OPTION_THREAD,
    ) < 0 as ::core::ffi::c_int
    {
        printf(
            b"ERROR: json_c_set_serialization_double_format() failed\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    printf(
        b"obj.to_string(with thread format)=%s\n\0" as *const u8 as *const ::core::ffi::c_char,
        json_object_to_json_string(obj),
    );
    if json_c_set_serialization_double_format(
        b"Ttttttttttttt%0.2fxxxxxxxxxxxxxxxxxxX\0" as *const u8 as *const ::core::ffi::c_char,
        JSON_C_OPTION_THREAD,
    ) < 0 as ::core::ffi::c_int
    {
        printf(
            b"ERROR: json_c_set_serialization_double_format() failed\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    printf(
        b"obj.to_string(long thread format)=%s\n\0" as *const u8 as *const ::core::ffi::c_char,
        json_object_to_json_string(obj),
    );
    if json_c_set_serialization_double_format(
        ::core::ptr::null::<::core::ffi::c_char>(),
        JSON_C_OPTION_THREAD,
    ) < 0 as ::core::ffi::c_int
    {
        printf(
            b"ERROR: json_c_set_serialization_double_format() failed\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    printf(
        b"obj.to_string(back to global format)=%s\n\0" as *const u8 as *const ::core::ffi::c_char,
        json_object_to_json_string(obj),
    );
    if json_c_set_serialization_double_format(
        ::core::ptr::null::<::core::ffi::c_char>(),
        JSON_C_OPTION_GLOBAL,
    ) < 0 as ::core::ffi::c_int
    {
        printf(
            b"ERROR: json_c_set_serialization_double_format() failed\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    printf(
        b"obj.to_string(back to default format)=%s\n\0" as *const u8 as *const ::core::ffi::c_char,
        json_object_to_json_string(obj),
    );
    json_object_put(obj);
    obj = json_object_new_double(12.0f64);
    printf(
        b"obj(12.0).to_string(default format)=%s\n\0" as *const u8 as *const ::core::ffi::c_char,
        json_object_to_json_string(obj),
    );
    if json_c_set_serialization_double_format(
        b"%.0f\0" as *const u8 as *const ::core::ffi::c_char,
        JSON_C_OPTION_GLOBAL,
    ) < 0 as ::core::ffi::c_int
    {
        printf(
            b"ERROR: json_c_set_serialization_double_format() failed\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    printf(
        b"obj(12.0).to_string(%%.0f)=%s\n\0" as *const u8 as *const ::core::ffi::c_char,
        json_object_to_json_string(obj),
    );
    if json_c_set_serialization_double_format(
        b"%.0g\0" as *const u8 as *const ::core::ffi::c_char,
        JSON_C_OPTION_GLOBAL,
    ) < 0 as ::core::ffi::c_int
    {
        printf(
            b"ERROR: json_c_set_serialization_double_format() failed\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    printf(
        b"obj(12.0).to_string(%%.0g)=%s\n\0" as *const u8 as *const ::core::ffi::c_char,
        json_object_to_json_string(obj),
    );
    if json_c_set_serialization_double_format(
        b"%.2g\0" as *const u8 as *const ::core::ffi::c_char,
        JSON_C_OPTION_GLOBAL,
    ) < 0 as ::core::ffi::c_int
    {
        printf(
            b"ERROR: json_c_set_serialization_double_format() failed\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    printf(
        b"obj(12.0).to_string(%%.1g)=%s\n\0" as *const u8 as *const ::core::ffi::c_char,
        json_object_to_json_string(obj),
    );
    if json_c_set_serialization_double_format(
        ::core::ptr::null::<::core::ffi::c_char>(),
        JSON_C_OPTION_GLOBAL,
    ) < 0 as ::core::ffi::c_int
    {
        printf(
            b"ERROR: json_c_set_serialization_double_format() failed\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    json_object_put(obj);
    obj = json_object_new_double(-12.0f64);
    printf(
        b"obj(-12.0).to_string(default format)=%s\n\0" as *const u8 as *const ::core::ffi::c_char,
        json_object_to_json_string(obj),
    );
    json_object_put(obj);
    obj = json_object_new_double(zero_dot_zero / zero_dot_zero);
    printf(
        b"obj(0.0/0.0)=%s\n\0" as *const u8 as *const ::core::ffi::c_char,
        json_object_to_json_string(obj),
    );
    json_object_put(obj);
    obj = json_object_new_double(1.0f64 / zero_dot_zero);
    printf(
        b"obj(1.0/0.0)=%s\n\0" as *const u8 as *const ::core::ffi::c_char,
        json_object_to_json_string(obj),
    );
    json_object_put(obj);
    obj = json_object_new_double(-1.0f64 / zero_dot_zero);
    printf(
        b"obj(-1.0/0.0)=%s\n\0" as *const u8 as *const ::core::ffi::c_char,
        json_object_to_json_string(obj),
    );
    json_object_put(obj);
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
