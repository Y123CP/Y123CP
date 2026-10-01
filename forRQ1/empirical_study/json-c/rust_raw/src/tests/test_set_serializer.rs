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
    fn sprintbuf(p: *mut printbuf, msg: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn json_object_get(obj: *mut json_object) -> *mut json_object;
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
    fn json_object_new_object() -> *mut json_object;
    fn json_object_object_add(
        obj: *mut json_object,
        key: *const ::core::ffi::c_char,
        val: *mut json_object,
    ) -> ::core::ffi::c_int;
    fn json_object_new_int(i: int32_t) -> *mut json_object;
    fn json_object_new_double(d: ::core::ffi::c_double) -> *mut json_object;
    fn json_object_double_to_json_string(
        jso: *mut json_object,
        pb: *mut printbuf,
        level: ::core::ffi::c_int,
        flags: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn json_object_new_string(s: *const ::core::ffi::c_char) -> *mut json_object;
}
pub type __int32_t = i32;
pub type int32_t = __int32_t;
pub type uintptr_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct printbuf {
    pub buf: *mut ::core::ffi::c_char,
    pub bpos: ::core::ffi::c_int,
    pub size: ::core::ffi::c_int,
}
pub type json_object_delete_fn =
    unsafe extern "C" fn(*mut json_object, *mut ::core::ffi::c_void) -> ();
pub type json_object_to_json_string_fn = unsafe extern "C" fn(
    *mut json_object,
    *mut printbuf,
    ::core::ffi::c_int,
    ::core::ffi::c_int,
) -> ::core::ffi::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct myinfo {
    pub value: ::core::ffi::c_int,
}
pub const __ASSERT_FUNCTION: [::core::ffi::c_char; 23] = unsafe {
    ::core::mem::transmute::<[u8; 23], [::core::ffi::c_char; 23]>(*b"int main(int, char **)\0")
};
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
static mut freeit_was_called: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
unsafe extern "C" fn freeit(mut jso: *mut json_object, mut userdata: *mut ::core::ffi::c_void) {
    let mut info: *mut myinfo = userdata as *mut myinfo;
    printf(
        b"freeit, value=%d\n\0" as *const u8 as *const ::core::ffi::c_char,
        (*info).value,
    );
    freeit_was_called = 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn custom_serializer(
    mut o: *mut json_object,
    mut pb: *mut printbuf,
    mut level: ::core::ffi::c_int,
    mut flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    sprintbuf(
        pb,
        b"Custom Output\0" as *const u8 as *const ::core::ffi::c_char,
    );
    return 0 as ::core::ffi::c_int;
}
unsafe fn main_0(
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut my_object: *mut json_object = ::core::ptr::null_mut::<json_object>();
    let mut my_sub_object: *mut json_object = ::core::ptr::null_mut::<json_object>();
    printf(
        b"Test setting, then resetting a custom serializer:\n\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    my_object = json_object_new_object() as *mut json_object;
    json_object_object_add(
        my_object as *mut json_object,
        b"abc\0" as *const u8 as *const ::core::ffi::c_char,
        json_object_new_int(12 as int32_t),
    );
    json_object_object_add(
        my_object as *mut json_object,
        b"foo\0" as *const u8 as *const ::core::ffi::c_char,
        json_object_new_string(b"bar\0" as *const u8 as *const ::core::ffi::c_char),
    );
    printf(
        b"my_object.to_string(standard)=%s\n\0" as *const u8 as *const ::core::ffi::c_char,
        json_object_to_json_string(my_object as *mut json_object),
    );
    let mut userdata: myinfo = myinfo {
        value: 123 as ::core::ffi::c_int,
    };
    json_object_set_serializer(
        my_object,
        Some(
            custom_serializer
                as unsafe extern "C" fn(
                    *mut json_object,
                    *mut printbuf,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        &raw mut userdata as *mut ::core::ffi::c_void,
        Some(freeit as unsafe extern "C" fn(*mut json_object, *mut ::core::ffi::c_void) -> ()),
    );
    printf(
        b"my_object.to_string(custom serializer)=%s\n\0" as *const u8 as *const ::core::ffi::c_char,
        json_object_to_json_string(my_object as *mut json_object),
    );
    printf(
        b"Next line of output should be from the custom freeit function:\n\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    freeit_was_called = 0 as ::core::ffi::c_int;
    json_object_set_serializer(my_object, None, NULL, None);
    '_c2rust_label: {
        if freeit_was_called != 0 {
        } else {
            __assert_fail(
                b"freeit_was_called\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_set_serializer.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                52 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    printf(
        b"my_object.to_string(standard)=%s\n\0" as *const u8 as *const ::core::ffi::c_char,
        json_object_to_json_string(my_object as *mut json_object),
    );
    json_object_put(my_object as *mut json_object);
    my_object = json_object_new_object() as *mut json_object;
    printf(
        b"Check that the custom serializer isn't free'd until the last json_object_put:\n\0"
            as *const u8 as *const ::core::ffi::c_char,
    );
    json_object_set_serializer(
        my_object,
        Some(
            custom_serializer
                as unsafe extern "C" fn(
                    *mut json_object,
                    *mut printbuf,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        &raw mut userdata as *mut ::core::ffi::c_void,
        Some(freeit as unsafe extern "C" fn(*mut json_object, *mut ::core::ffi::c_void) -> ()),
    );
    json_object_get(my_object as *mut json_object);
    json_object_put(my_object as *mut json_object);
    printf(
        b"my_object.to_string(custom serializer)=%s\n\0" as *const u8 as *const ::core::ffi::c_char,
        json_object_to_json_string(my_object as *mut json_object),
    );
    printf(
        b"Next line of output should be from the custom freeit function:\n\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    freeit_was_called = 0 as ::core::ffi::c_int;
    json_object_put(my_object as *mut json_object);
    '_c2rust_label_0: {
        if freeit_was_called != 0 {
        } else {
            __assert_fail(
                b"freeit_was_called\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_set_serializer.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                71 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    my_object = json_object_new_object() as *mut json_object;
    my_sub_object = json_object_new_double(1.0f64) as *mut json_object;
    json_object_object_add(
        my_object as *mut json_object,
        b"double\0" as *const u8 as *const ::core::ffi::c_char,
        my_sub_object as *mut json_object,
    );
    printf(
        b"Check that the custom serializer does not include nul byte:\n\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    json_object_set_serializer(
        my_sub_object,
        Some(
            json_object_double_to_json_string
                as unsafe extern "C" fn(
                    *mut json_object,
                    *mut printbuf,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        b"%125.0f\0" as *const u8 as *const ::core::ffi::c_char as *const ::core::ffi::c_void
            as uintptr_t as *mut ::core::ffi::c_void,
        None,
    );
    printf(
        b"my_object.to_string(custom serializer)=%s\n\0" as *const u8 as *const ::core::ffi::c_char,
        json_object_to_json_string_ext(my_object as *mut json_object, JSON_C_TO_STRING_NOZERO),
    );
    json_object_put(my_object as *mut json_object);
    return 0 as ::core::ffi::c_int;
}
pub const JSON_C_TO_STRING_NOZERO: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int;
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
