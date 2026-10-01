extern "C" {
    pub type json_object;
    fn printf(__format: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn __assert_fail(
        __assertion: *const ::core::ffi::c_char,
        __file: *const ::core::ffi::c_char,
        __line: ::core::ffi::c_uint,
        __function: *const ::core::ffi::c_char,
    ) -> !;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn time(__timer: *mut time_t) -> time_t;
    fn sprintbuf(p: *mut printbuf, msg: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn json_object_get(obj: *mut json_object) -> *mut json_object;
    fn json_object_put(obj: *mut json_object) -> ::core::ffi::c_int;
    fn json_object_to_json_string(obj: *mut json_object) -> *const ::core::ffi::c_char;
    fn json_object_to_json_string_ext(
        obj: *mut json_object,
        flags: ::core::ffi::c_int,
    ) -> *const ::core::ffi::c_char;
    fn json_object_get_userdata(jso: *mut json_object) -> *mut ::core::ffi::c_void;
    fn json_object_set_serializer(
        jso: *mut json_object,
        to_string_func: Option<json_object_to_json_string_fn>,
        userdata: *mut ::core::ffi::c_void,
        user_delete: Option<json_object_delete_fn>,
    );
    fn json_object_object_add(
        obj: *mut json_object,
        key: *const ::core::ffi::c_char,
        val: *mut json_object,
    ) -> ::core::ffi::c_int;
    fn json_object_object_get(
        obj: *const json_object,
        key: *const ::core::ffi::c_char,
    ) -> *mut json_object;
    fn json_object_new_string(s: *const ::core::ffi::c_char) -> *mut json_object;
    fn json_object_get_string(obj: *mut json_object) -> *const ::core::ffi::c_char;
    fn json_object_equal(obj1: *mut json_object, obj2: *mut json_object) -> ::core::ffi::c_int;
    fn json_c_shallow_copy_default(
        _: *mut json_object,
        _: *mut json_object,
        _: *const ::core::ffi::c_char,
        _: size_t,
        _: *mut *mut json_object,
    ) -> ::core::ffi::c_int;
    fn json_object_deep_copy(
        src: *mut json_object,
        dst: *mut *mut json_object,
        shallow_copy: Option<json_c_shallow_copy_fn>,
    ) -> ::core::ffi::c_int;
    fn json_tokener_parse(str: *const ::core::ffi::c_char) -> *mut json_object;
}
pub type size_t = usize;
pub type __time_t = ::core::ffi::c_long;
pub type time_t = __time_t;
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
pub type json_c_shallow_copy_fn = unsafe extern "C" fn(
    *mut json_object,
    *mut json_object,
    *const ::core::ffi::c_char,
    size_t,
    *mut *mut json_object,
) -> ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const __ASSERT_FUNCTION: [::core::ffi::c_char; 23] = unsafe {
    ::core::mem::transmute::<[u8; 23], [::core::ffi::c_char; 23]>(*b"int main(int, char **)\0")
};
pub const JSON_C_TO_STRING_PRETTY: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int;
static mut json_str1: *const ::core::ffi::c_char = b"{    \"glossary\": {        \"title\": \"example glossary\",        \"GlossDiv\": {            \"number\": 16446744073709551615,            \"title\": \"S\",            \"null_obj\": null,             \"exist\": false,            \"quantity\":20,            \"univalent\":19.8,            \"GlossList\": {                \"GlossEntry\": {                    \"ID\": \"SGML\",                    \"SortAs\": \"SGML\",                    \"GlossTerm\": \"Standard Generalized Markup Language\",                    \"Acronym\": \"SGML\",                    \"Abbrev\": \"ISO 8879:1986\",                    \"GlossDef\": {                        \"para\": \"A meta-markup language, used to create markup languages such as DocBook.\",                        \"GlossSeeAlso\": [\"GML\", \"XML\"]                    },                    \"GlossSee\": \"markup\"                }            }        }    }}\0"
    as *const u8 as *const ::core::ffi::c_char;
static mut json_str2: *const ::core::ffi::c_char = b"{\"menu\": {    \"header\": \"SVG Viewer\",    \"items\": [        {\"id\": \"Open\"},        {\"id\": \"OpenNew\", \"label\": \"Open New\"},        null,        {\"id\": \"ZoomIn\", \"label\": \"Zoom In\"},        {\"id\": \"ZoomOut\", \"label\": \"Zoom Out\"},        {\"id\": \"OriginalView\", \"label\": \"Original View\"},        null,        {\"id\": \"Quality\", \"another_null\": null},        {\"id\": \"Pause\"},        {\"id\": \"Mute\"},        null,        {\"id\": \"Find\", \"label\": \"Find...\"},        {\"id\": \"FindAgain\", \"label\": \"Find Again\"},        {\"id\": \"Copy\"},        {\"id\": \"CopyAgain\", \"label\": \"Copy Again\"},        {\"id\": \"CopySVG\", \"label\": \"Copy SVG\"},        {\"id\": \"ViewSVG\", \"label\": \"View SVG\"},        {\"id\": \"ViewSource\", \"label\": \"View Source\"},        {\"id\": \"SaveAs\", \"label\": \"Save As\"},        null,        {\"id\": \"Help\"},        {\"id\": \"About\", \"label\": \"About Adobe CVG Viewer...\"}    ]}}\0"
    as *const u8 as *const ::core::ffi::c_char;
static mut json_str3: *const ::core::ffi::c_char = b"{\"menu\": {  \"id\": \"file\",  \"value\": \"File\",  \"popup\": {    \"menuitem\": [      {\"value\": \"New\", \"onclick\": \"CreateNewDoc()\"},      {\"value\": \"Open\", \"onclick\": \"OpenDoc()\"},      {\"value\": \"Close\", \"onclick\": \"CloseDoc()\"}    ]  }}}\0"
    as *const u8 as *const ::core::ffi::c_char;
#[no_mangle]
pub unsafe extern "C" fn my_custom_serializer(
    mut jso: *mut json_object,
    mut pb: *mut printbuf,
    mut level: ::core::ffi::c_int,
    mut flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    sprintbuf(pb, b"OTHER\0" as *const u8 as *const ::core::ffi::c_char);
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn my_shallow_copy(
    mut src: *mut json_object,
    mut parent: *mut json_object,
    mut key: *const ::core::ffi::c_char,
    mut index: size_t,
    mut dst: *mut *mut json_object,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = 0;
    rc = json_c_shallow_copy_default(src, parent, key, index, dst);
    if rc < 0 as ::core::ffi::c_int {
        return rc;
    }
    if !key.is_null()
        && strcmp(
            key,
            b"with_serializer\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
    {
        printf(
            b"CALLED: my_shallow_copy on with_serializer object\n\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        let mut userdata: *mut ::core::ffi::c_void = json_object_get_userdata(src);
        json_object_set_serializer(
            *dst,
            Some(
                my_custom_serializer
                    as unsafe extern "C" fn(
                        *mut json_object,
                        *mut printbuf,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
            userdata,
            None,
        );
        return 2 as ::core::ffi::c_int;
    }
    return rc;
}
unsafe fn main_0(
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut src1: *mut json_object = ::core::ptr::null_mut::<json_object>();
    let mut src2: *mut json_object = ::core::ptr::null_mut::<json_object>();
    let mut src3: *mut json_object = ::core::ptr::null_mut::<json_object>();
    let mut dst1: *mut json_object = ::core::ptr::null_mut::<json_object>();
    let mut dst2: *mut json_object = ::core::ptr::null_mut::<json_object>();
    let mut dst3: *mut json_object = ::core::ptr::null_mut::<json_object>();
    let mut benchmark: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if argc > 1 as ::core::ffi::c_int
        && strcmp(
            *argv.offset(1 as ::core::ffi::c_int as isize),
            b"--benchmark\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
    {
        benchmark = 1 as ::core::ffi::c_int;
    }
    src1 = json_tokener_parse(json_str1);
    src2 = json_tokener_parse(json_str2);
    src3 = json_tokener_parse(json_str3);
    '_c2rust_label: {
        if !src1.is_null() {
        } else {
            __assert_fail(
                b"src1 != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_deep_copy.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                128 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    '_c2rust_label_0: {
        if !src2.is_null() {
        } else {
            __assert_fail(
                b"src2 != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_deep_copy.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                129 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    '_c2rust_label_1: {
        if !src3.is_null() {
        } else {
            __assert_fail(
                b"src3 != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_deep_copy.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                130 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    printf(b"PASSED - loaded input data\n\0" as *const u8 as *const ::core::ffi::c_char);
    '_c2rust_label_2: {
        if 0 as ::core::ffi::c_int == json_object_deep_copy(src1, &raw mut dst1, None) {
        } else {
            __assert_fail(
                b"0 == json_object_deep_copy(src1, &dst1, NULL)\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_deep_copy.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                135 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    '_c2rust_label_3: {
        if 0 as ::core::ffi::c_int == json_object_deep_copy(src2, &raw mut dst2, None) {
        } else {
            __assert_fail(
                b"0 == json_object_deep_copy(src2, &dst2, NULL)\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_deep_copy.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                136 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    '_c2rust_label_4: {
        if 0 as ::core::ffi::c_int == json_object_deep_copy(src3, &raw mut dst3, None) {
        } else {
            __assert_fail(
                b"0 == json_object_deep_copy(src3, &dst3, NULL)\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_deep_copy.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                137 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    printf(
        b"PASSED - all json_object_deep_copy() returned successful\n\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    '_c2rust_label_5: {
        if -(1 as ::core::ffi::c_int) == json_object_deep_copy(src1, &raw mut dst1, None) {
        } else {
            __assert_fail(
                b"-1 == json_object_deep_copy(src1, &dst1, NULL)\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_deep_copy.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                141 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    '_c2rust_label_6: {
        if *__errno_location() == 22 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"errno == EINVAL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_deep_copy.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                142 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    '_c2rust_label_7: {
        if -(1 as ::core::ffi::c_int) == json_object_deep_copy(src2, &raw mut dst2, None) {
        } else {
            __assert_fail(
                b"-1 == json_object_deep_copy(src2, &dst2, NULL)\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_deep_copy.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                143 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    '_c2rust_label_8: {
        if *__errno_location() == 22 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"errno == EINVAL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_deep_copy.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                144 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    '_c2rust_label_9: {
        if -(1 as ::core::ffi::c_int) == json_object_deep_copy(src3, &raw mut dst3, None) {
        } else {
            __assert_fail(
                b"-1 == json_object_deep_copy(src3, &dst3, NULL)\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_deep_copy.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                145 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    '_c2rust_label_10: {
        if *__errno_location() == 22 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"errno == EINVAL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_deep_copy.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                146 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    printf(
        b"PASSED - all json_object_deep_copy() returned EINVAL for non-null pointer\n\0"
            as *const u8 as *const ::core::ffi::c_char,
    );
    '_c2rust_label_11: {
        if 1 as ::core::ffi::c_int == json_object_equal(src1, dst1) {
        } else {
            __assert_fail(
                b"1 == json_object_equal(src1, dst1)\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_deep_copy.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                150 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    '_c2rust_label_12: {
        if 1 as ::core::ffi::c_int == json_object_equal(src2, dst2) {
        } else {
            __assert_fail(
                b"1 == json_object_equal(src2, dst2)\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_deep_copy.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                151 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    '_c2rust_label_13: {
        if 1 as ::core::ffi::c_int == json_object_equal(src3, dst3) {
        } else {
            __assert_fail(
                b"1 == json_object_equal(src3, dst3)\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_deep_copy.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                152 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    printf(
        b"PASSED - all json_object_equal() tests returned successful\n\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    '_c2rust_label_14: {
        if 0 as ::core::ffi::c_int
            == strcmp(
                json_object_to_json_string_ext(
                    src1,
                    (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
                ),
                json_object_to_json_string_ext(
                    dst1,
                    (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
                ),
            )
        {
        } else {
            __assert_fail(
                b"0 == strcmp(json_object_to_json_string_ext(src1, JSON_C_TO_STRING_PRETTY), json_object_to_json_string_ext(dst1, JSON_C_TO_STRING_PRETTY))\0"
                    as *const u8 as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_deep_copy.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                157 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    '_c2rust_label_15: {
        if 0 as ::core::ffi::c_int
            == strcmp(
                json_object_to_json_string_ext(
                    src2,
                    (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
                ),
                json_object_to_json_string_ext(
                    dst2,
                    (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
                ),
            )
        {
        } else {
            __assert_fail(
                b"0 == strcmp(json_object_to_json_string_ext(src2, JSON_C_TO_STRING_PRETTY), json_object_to_json_string_ext(dst2, JSON_C_TO_STRING_PRETTY))\0"
                    as *const u8 as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_deep_copy.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                159 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    '_c2rust_label_16: {
        if 0 as ::core::ffi::c_int
            == strcmp(
                json_object_to_json_string_ext(
                    src3,
                    (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
                ),
                json_object_to_json_string_ext(
                    dst3,
                    (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
                ),
            )
        {
        } else {
            __assert_fail(
                b"0 == strcmp(json_object_to_json_string_ext(src3, JSON_C_TO_STRING_PRETTY), json_object_to_json_string_ext(dst3, JSON_C_TO_STRING_PRETTY))\0"
                    as *const u8 as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_deep_copy.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                161 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    printf(b"PASSED - comparison of string output\n\0" as *const u8 as *const ::core::ffi::c_char);
    json_object_get(dst1);
    '_c2rust_label_17: {
        if -(1 as ::core::ffi::c_int) == json_object_deep_copy(src1, &raw mut dst1, None) {
        } else {
            __assert_fail(
                b"-1 == json_object_deep_copy(src1, &dst1, NULL)\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_deep_copy.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                166 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    '_c2rust_label_18: {
        if *__errno_location() == 22 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"errno == EINVAL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_deep_copy.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                167 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    json_object_put(dst1);
    printf(
        b"PASSED - trying to overrwrite an object that has refcount > 1\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    printf(
        b"\nPrinting JSON objects for visual inspection\n\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    printf(
        b"------------------------------------------------\n\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    printf(b" JSON1\n\0" as *const u8 as *const ::core::ffi::c_char);
    printf(
        b"%s\n\0" as *const u8 as *const ::core::ffi::c_char,
        json_object_to_json_string_ext(dst1, JSON_C_TO_STRING_PRETTY),
    );
    printf(
        b"------------------------------------------------\n\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    printf(
        b"------------------------------------------------\n\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    printf(b" JSON2\n\0" as *const u8 as *const ::core::ffi::c_char);
    printf(
        b"%s\n\0" as *const u8 as *const ::core::ffi::c_char,
        json_object_to_json_string_ext(dst2, JSON_C_TO_STRING_PRETTY),
    );
    printf(
        b"------------------------------------------------\n\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    printf(
        b"------------------------------------------------\n\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    printf(b" JSON3\n\0" as *const u8 as *const ::core::ffi::c_char);
    printf(
        b"------------------------------------------------\n\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    printf(
        b"%s\n\0" as *const u8 as *const ::core::ffi::c_char,
        json_object_to_json_string_ext(dst3, JSON_C_TO_STRING_PRETTY),
    );
    printf(
        b"------------------------------------------------\n\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    json_object_put(dst1);
    json_object_put(dst2);
    json_object_put(dst3);
    printf(
        b"\nTesting deep_copy with a custom serializer set\n\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    let mut with_serializer: *mut json_object =
        json_object_new_string(b"notemitted\0" as *const u8 as *const ::core::ffi::c_char)
            as *mut json_object;
    let mut udata: [::core::ffi::c_char; 15] =
        ::core::mem::transmute::<[u8; 15], [::core::ffi::c_char; 15]>(*b"dummy userdata\0");
    json_object_set_serializer(
        with_serializer,
        Some(
            my_custom_serializer
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
    json_object_object_add(
        src1,
        b"with_serializer\0" as *const u8 as *const ::core::ffi::c_char,
        with_serializer as *mut json_object,
    );
    dst1 = ::core::ptr::null_mut::<json_object>();
    '_c2rust_label_19: {
        if -(1 as ::core::ffi::c_int) == json_object_deep_copy(src1, &raw mut dst1, None) {
        } else {
            __assert_fail(
                b"-1 == json_object_deep_copy(src1, &dst1, NULL)\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_deep_copy.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                201 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    '_c2rust_label_20: {
        if 0 as ::core::ffi::c_int
            == json_object_deep_copy(
                src1,
                &raw mut dst1,
                Some(
                    my_shallow_copy
                        as unsafe extern "C" fn(
                            *mut json_object,
                            *mut json_object,
                            *const ::core::ffi::c_char,
                            size_t,
                            *mut *mut json_object,
                        ) -> ::core::ffi::c_int,
                ),
            )
        {
        } else {
            __assert_fail(
                b"0 == json_object_deep_copy(src1, &dst1, my_shallow_copy)\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_deep_copy.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                202 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    let mut dest_with_serializer: *mut json_object = json_object_object_get(
        dst1,
        b"with_serializer\0" as *const u8 as *const ::core::ffi::c_char,
    ) as *mut json_object;
    '_c2rust_label_21: {
        if !dest_with_serializer.is_null() {
        } else {
            __assert_fail(
                b"dest_with_serializer != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_deep_copy.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                205 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    let mut dst_userdata: *mut ::core::ffi::c_char =
        json_object_get_userdata(dest_with_serializer) as *mut ::core::ffi::c_char;
    '_c2rust_label_22: {
        if strcmp(
            dst_userdata,
            b"dummy userdata\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
        } else {
            __assert_fail(
                b"strcmp(dst_userdata, \"dummy userdata\") == 0\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_deep_copy.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                207 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    let mut special_output: *const ::core::ffi::c_char =
        json_object_to_json_string(dest_with_serializer as *mut json_object);
    '_c2rust_label_23: {
        if strcmp(
            special_output,
            b"OTHER\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
        } else {
            __assert_fail(
                b"strcmp(special_output, \"OTHER\") == 0\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_deep_copy.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                210 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    printf(
        b"\ndeep_copy with custom serializer worked OK.\n\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    json_object_put(dst1);
    if benchmark != 0 {
        do_benchmark(src2 as *mut json_object);
    }
    json_object_put(src1);
    json_object_put(src2);
    json_object_put(src3);
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn do_benchmark(mut src2: *mut json_object) {
    let mut dst2: *mut json_object = ::core::ptr::null_mut::<json_object>();
    let mut ii: ::core::ffi::c_int = 0;
    let mut iterations: ::core::ffi::c_int = 1000000 as ::core::ffi::c_int;
    let mut start: time_t = time(::core::ptr::null_mut::<time_t>());
    start = time(::core::ptr::null_mut::<time_t>());
    ii = 0 as ::core::ffi::c_int;
    while ii < iterations {
        dst2 = json_tokener_parse(json_object_get_string(src2 as *mut json_object))
            as *mut json_object;
        json_object_put(dst2 as *mut json_object);
        ii += 1;
    }
    printf(
        b"BENCHMARK - %d iterations of 'dst2 = json_tokener_parse(json_object_get_string(src2))' took %d seconds\n\0"
            as *const u8 as *const ::core::ffi::c_char,
        iterations,
        (time(::core::ptr::null_mut::<time_t>()) - start) as ::core::ffi::c_int,
    );
    start = time(::core::ptr::null_mut::<time_t>());
    dst2 = ::core::ptr::null_mut::<json_object>();
    ii = 0 as ::core::ffi::c_int;
    while ii < iterations {
        json_object_deep_copy(src2 as *mut json_object, &raw mut dst2, None);
        json_object_put(dst2 as *mut json_object);
        dst2 = ::core::ptr::null_mut::<json_object>();
        ii += 1;
    }
    printf(
        b"BENCHMARK - %d iterations of 'json_object_deep_copy(src2, &dst2, NULL)' took %d seconds\n\0"
            as *const u8 as *const ::core::ffi::c_char,
        iterations,
        (time(::core::ptr::null_mut::<time_t>()) - start) as ::core::ffi::c_int,
    );
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
