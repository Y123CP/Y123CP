extern "C" {
    pub type json_object;
    fn __assert_fail(
        __assertion: *const ::core::ffi::c_char,
        __file: *const ::core::ffi::c_char,
        __line: ::core::ffi::c_uint,
        __function: *const ::core::ffi::c_char,
    ) -> !;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn printf(__format: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn json_pointer_get(
        obj: *mut json_object,
        path: *const ::core::ffi::c_char,
        res: *mut *mut json_object,
    ) -> ::core::ffi::c_int;
    fn json_pointer_getf(
        obj: *mut json_object,
        res: *mut *mut json_object,
        path_fmt: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn json_pointer_set(
        obj: *mut *mut json_object,
        path: *const ::core::ffi::c_char,
        value: *mut json_object,
    ) -> ::core::ffi::c_int;
    fn json_pointer_setf(
        obj: *mut *mut json_object,
        value: *mut json_object,
        path_fmt: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn json_tokener_parse(str: *const ::core::ffi::c_char) -> *mut json_object;
    fn json_object_put(obj: *mut json_object) -> ::core::ffi::c_int;
    fn json_object_is_type(obj: *const json_object, type_0: json_type) -> ::core::ffi::c_int;
    fn json_object_new_object() -> *mut json_object;
    fn json_object_object_get(
        obj: *const json_object,
        key: *const ::core::ffi::c_char,
    ) -> *mut json_object;
    fn json_object_new_array() -> *mut json_object;
    fn json_object_array_add(obj: *mut json_object, val: *mut json_object) -> ::core::ffi::c_int;
    fn json_object_array_get_idx(obj: *const json_object, idx: size_t) -> *mut json_object;
    fn json_object_new_int(i: int32_t) -> *mut json_object;
    fn json_object_get_int(obj: *const json_object) -> int32_t;
    fn json_object_new_string(s: *const ::core::ffi::c_char) -> *mut json_object;
    fn json_object_get_string(obj: *mut json_object) -> *const ::core::ffi::c_char;
    fn json_object_equal(obj1: *mut json_object, obj2: *mut json_object) -> ::core::ffi::c_int;
}
pub type size_t = usize;
pub type __int32_t = i32;
pub type int32_t = __int32_t;
pub type json_type = ::core::ffi::c_uint;
pub const json_type_string: json_type = 6;
pub const json_type_array: json_type = 5;
pub const json_type_object: json_type = 4;
pub const json_type_int: json_type = 3;
pub const json_type_double: json_type = 2;
pub const json_type_boolean: json_type = 1;
pub const json_type_null: json_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct json_pointer_map_s_i {
    pub s: *const ::core::ffi::c_char,
    pub i: ::core::ffi::c_int,
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
unsafe extern "C" fn test_example_int(
    mut jo1: *mut json_object,
    mut json_pointer: *const ::core::ffi::c_char,
    mut expected_int: ::core::ffi::c_int,
) {
    let mut jo2: *mut json_object = ::core::ptr::null_mut::<json_object>();
    '_c2rust_label: {
        if 0 as ::core::ffi::c_int
            == json_pointer_get(
                jo1,
                json_pointer,
                ::core::ptr::null_mut::<*mut json_object>(),
            )
        {
        } else {
            __assert_fail(
                b"0 == json_pointer_get(jo1, json_pointer, NULL)\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_json_pointer.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                14 as ::core::ffi::c_uint,
                b"void test_example_int(struct json_object *, const char *, int)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if 0 as ::core::ffi::c_int == json_pointer_get(jo1, json_pointer, &raw mut jo2) {
        } else {
            __assert_fail(
                b"0 == json_pointer_get(jo1, json_pointer, &jo2)\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_json_pointer.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                15 as ::core::ffi::c_uint,
                b"void test_example_int(struct json_object *, const char *, int)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_1: {
        if json_object_is_type(jo2, json_type_int) != 0 {
        } else {
            __assert_fail(
                b"json_object_is_type(jo2, json_type_int)\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_json_pointer.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                16 as ::core::ffi::c_uint,
                b"void test_example_int(struct json_object *, const char *, int)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_2: {
        if expected_int as int32_t == json_object_get_int(jo2) {
        } else {
            __assert_fail(
                b"expected_int == json_object_get_int(jo2)\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_json_pointer.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                17 as ::core::ffi::c_uint,
                b"void test_example_int(struct json_object *, const char *, int)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    printf(
        b"PASSED - GET -  %s == %d\n\0" as *const u8 as *const ::core::ffi::c_char,
        json_pointer,
        expected_int,
    );
}
static mut input_json_str: *const ::core::ffi::c_char = b"{ 'foo': ['bar', 'baz'], '': 0, 'a/b': 1, 'c%d': 2, 'e^f': 3, 'g|h': 4, 'i\\\\j': 5, 'k\\\"l': 6, ' ': 7, 'm~n': 8 }\0"
    as *const u8 as *const ::core::ffi::c_char;
static mut rec_input_json_str: *const ::core::ffi::c_char = b"{'arr' : [{'obj': [{},{},{'obj1': 0,'obj2': \"1\"}]}],'obj' : {'obj': {'obj': [{'obj1': 0,'obj2': \"1\"}]}}}\0"
    as *const u8 as *const ::core::ffi::c_char;
unsafe extern "C" fn test_example_get() {
    let mut i: ::core::ffi::c_int = 0;
    let mut jo1: *mut json_object = ::core::ptr::null_mut::<json_object>();
    let mut jo2: *mut json_object = ::core::ptr::null_mut::<json_object>();
    let mut jo3: *mut json_object = ::core::ptr::null_mut::<json_object>();
    let mut json_pointers: [json_pointer_map_s_i; 10] = [
        json_pointer_map_s_i {
            s: b"/\0" as *const u8 as *const ::core::ffi::c_char,
            i: 0 as ::core::ffi::c_int,
        },
        json_pointer_map_s_i {
            s: b"/a~1b\0" as *const u8 as *const ::core::ffi::c_char,
            i: 1 as ::core::ffi::c_int,
        },
        json_pointer_map_s_i {
            s: b"/c%d\0" as *const u8 as *const ::core::ffi::c_char,
            i: 2 as ::core::ffi::c_int,
        },
        json_pointer_map_s_i {
            s: b"/e^f\0" as *const u8 as *const ::core::ffi::c_char,
            i: 3 as ::core::ffi::c_int,
        },
        json_pointer_map_s_i {
            s: b"/g|h\0" as *const u8 as *const ::core::ffi::c_char,
            i: 4 as ::core::ffi::c_int,
        },
        json_pointer_map_s_i {
            s: b"/i\\j\0" as *const u8 as *const ::core::ffi::c_char,
            i: 5 as ::core::ffi::c_int,
        },
        json_pointer_map_s_i {
            s: b"/k\"l\0" as *const u8 as *const ::core::ffi::c_char,
            i: 6 as ::core::ffi::c_int,
        },
        json_pointer_map_s_i {
            s: b"/ \0" as *const u8 as *const ::core::ffi::c_char,
            i: 7 as ::core::ffi::c_int,
        },
        json_pointer_map_s_i {
            s: b"/m~0n\0" as *const u8 as *const ::core::ffi::c_char,
            i: 8 as ::core::ffi::c_int,
        },
        json_pointer_map_s_i {
            s: ::core::ptr::null::<::core::ffi::c_char>(),
            i: 0 as ::core::ffi::c_int,
        },
    ];
    jo1 = json_tokener_parse(input_json_str);
    '_c2rust_label: {
        if !jo1.is_null() {
        } else {
            __assert_fail(
                b"NULL != jo1\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_json_pointer.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                88 as ::core::ffi::c_uint,
                b"void test_example_get(void)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    printf(b"PASSED - GET - LOADED TEST JSON\n\0" as *const u8 as *const ::core::ffi::c_char);
    printf(
        b"%s\n\0" as *const u8 as *const ::core::ffi::c_char,
        json_object_get_string(jo1),
    );
    jo2 = ::core::ptr::null_mut::<json_object>();
    '_c2rust_label_0: {
        if 0 as ::core::ffi::c_int
            == json_pointer_get(
                jo1,
                b"\0" as *const u8 as *const ::core::ffi::c_char,
                ::core::ptr::null_mut::<*mut json_object>(),
            )
        {
        } else {
            __assert_fail(
                b"0 == json_pointer_get(jo1, \"\", NULL)\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_json_pointer.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                95 as ::core::ffi::c_uint,
                b"void test_example_get(void)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_1: {
        if 0 as ::core::ffi::c_int
            == json_pointer_get(
                jo1,
                b"\0" as *const u8 as *const ::core::ffi::c_char,
                &raw mut jo2,
            )
        {
        } else {
            __assert_fail(
                b"0 == json_pointer_get(jo1, \"\", &jo2)\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_json_pointer.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                96 as ::core::ffi::c_uint,
                b"void test_example_get(void)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_2: {
        if json_object_equal(jo2, jo1) != 0 {
        } else {
            __assert_fail(
                b"json_object_equal(jo2, jo1)\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_json_pointer.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                97 as ::core::ffi::c_uint,
                b"void test_example_get(void)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    printf(b"PASSED - GET - ENTIRE OBJECT WORKED\n\0" as *const u8 as *const ::core::ffi::c_char);
    jo3 = json_object_new_array();
    json_object_array_add(
        jo3,
        json_object_new_string(b"bar\0" as *const u8 as *const ::core::ffi::c_char),
    );
    json_object_array_add(
        jo3,
        json_object_new_string(b"baz\0" as *const u8 as *const ::core::ffi::c_char),
    );
    jo2 = ::core::ptr::null_mut::<json_object>();
    '_c2rust_label_3: {
        if 0 as ::core::ffi::c_int
            == json_pointer_get(
                jo1,
                b"/foo\0" as *const u8 as *const ::core::ffi::c_char,
                ::core::ptr::null_mut::<*mut json_object>(),
            )
        {
        } else {
            __assert_fail(
                b"0 == json_pointer_get(jo1, \"/foo\", NULL)\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_json_pointer.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                106 as ::core::ffi::c_uint,
                b"void test_example_get(void)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_4: {
        if 0 as ::core::ffi::c_int
            == json_pointer_get(
                jo1,
                b"/foo\0" as *const u8 as *const ::core::ffi::c_char,
                &raw mut jo2,
            )
        {
        } else {
            __assert_fail(
                b"0 == json_pointer_get(jo1, \"/foo\", &jo2)\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_json_pointer.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                107 as ::core::ffi::c_uint,
                b"void test_example_get(void)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_5: {
        if !jo2.is_null() {
        } else {
            __assert_fail(
                b"NULL != jo2\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_json_pointer.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                108 as ::core::ffi::c_uint,
                b"void test_example_get(void)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_6: {
        if json_object_equal(jo2, jo3) != 0 {
        } else {
            __assert_fail(
                b"json_object_equal(jo2, jo3)\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_json_pointer.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                109 as ::core::ffi::c_uint,
                b"void test_example_get(void)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    json_object_put(jo3);
    printf(b"PASSED - GET - /foo == ['bar', 'baz']\n\0" as *const u8 as *const ::core::ffi::c_char);
    jo2 = ::core::ptr::null_mut::<json_object>();
    '_c2rust_label_7: {
        if 0 as ::core::ffi::c_int
            == json_pointer_get(
                jo1,
                b"/foo/0\0" as *const u8 as *const ::core::ffi::c_char,
                ::core::ptr::null_mut::<*mut json_object>(),
            )
        {
        } else {
            __assert_fail(
                b"0 == json_pointer_get(jo1, \"/foo/0\", NULL)\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_json_pointer.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                115 as ::core::ffi::c_uint,
                b"void test_example_get(void)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_8: {
        if 0 as ::core::ffi::c_int
            == json_pointer_get(
                jo1,
                b"/foo/0\0" as *const u8 as *const ::core::ffi::c_char,
                &raw mut jo2,
            )
        {
        } else {
            __assert_fail(
                b"0 == json_pointer_get(jo1, \"/foo/0\", &jo2)\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_json_pointer.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                116 as ::core::ffi::c_uint,
                b"void test_example_get(void)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_9: {
        if !jo2.is_null() {
        } else {
            __assert_fail(
                b"NULL != jo2\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_json_pointer.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                117 as ::core::ffi::c_uint,
                b"void test_example_get(void)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_10: {
        if 0 as ::core::ffi::c_int
            == strcmp(
                b"bar\0" as *const u8 as *const ::core::ffi::c_char,
                json_object_get_string(jo2),
            )
        {
        } else {
            __assert_fail(
                b"0 == strcmp(\"bar\", json_object_get_string(jo2))\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_json_pointer.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                118 as ::core::ffi::c_uint,
                b"void test_example_get(void)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    printf(b"PASSED - GET - /foo/0 == 'bar'\n\0" as *const u8 as *const ::core::ffi::c_char);
    i = 0 as ::core::ffi::c_int;
    while !json_pointers[i as usize].s.is_null() {
        test_example_int(
            jo1,
            json_pointers[i as usize].s,
            json_pointers[i as usize].i,
        );
        i += 1;
    }
    json_object_put(jo1);
}
unsafe extern "C" fn test_recursion_get() {
    let mut jo2: *mut json_object = ::core::ptr::null_mut::<json_object>();
    let mut jo1: *mut json_object = json_tokener_parse(rec_input_json_str);
    jo2 = ::core::ptr::null_mut::<json_object>();
    '_c2rust_label: {
        if !jo1.is_null() {
        } else {
            __assert_fail(
                b"jo1 != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_json_pointer.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                133 as ::core::ffi::c_uint,
                b"void test_recursion_get(void)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    printf(
        b"%s\n\0" as *const u8 as *const ::core::ffi::c_char,
        json_object_get_string(jo1),
    );
    '_c2rust_label_0: {
        if 0 as ::core::ffi::c_int
            == json_pointer_get(
                jo1,
                b"/arr/0/obj/2/obj1\0" as *const u8 as *const ::core::ffi::c_char,
                &raw mut jo2,
            )
        {
        } else {
            __assert_fail(
                b"0 == json_pointer_get(jo1, \"/arr/0/obj/2/obj1\", &jo2)\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_json_pointer.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                135 as ::core::ffi::c_uint,
                b"void test_recursion_get(void)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_1: {
        if json_object_is_type(jo2, json_type_int) != 0 {
        } else {
            __assert_fail(
                b"json_object_is_type(jo2, json_type_int)\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_json_pointer.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                136 as ::core::ffi::c_uint,
                b"void test_recursion_get(void)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_2: {
        if 0 as int32_t == json_object_get_int(jo2) {
        } else {
            __assert_fail(
                b"0 == json_object_get_int(jo2)\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_json_pointer.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                137 as ::core::ffi::c_uint,
                b"void test_recursion_get(void)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_3: {
        if 0 as ::core::ffi::c_int
            == json_pointer_get(
                jo1,
                b"/arr/0/obj/2/obj2\0" as *const u8 as *const ::core::ffi::c_char,
                &raw mut jo2,
            )
        {
        } else {
            __assert_fail(
                b"0 == json_pointer_get(jo1, \"/arr/0/obj/2/obj2\", &jo2)\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_json_pointer.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                139 as ::core::ffi::c_uint,
                b"void test_recursion_get(void)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_4: {
        if json_object_is_type(jo2, json_type_string) != 0 {
        } else {
            __assert_fail(
                b"json_object_is_type(jo2, json_type_string)\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_json_pointer.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                140 as ::core::ffi::c_uint,
                b"void test_recursion_get(void)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_5: {
        if 0 as ::core::ffi::c_int
            == strcmp(
                b"1\0" as *const u8 as *const ::core::ffi::c_char,
                json_object_get_string(jo2),
            )
        {
        } else {
            __assert_fail(
                b"0 == strcmp(\"1\", json_object_get_string(jo2))\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_json_pointer.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                141 as ::core::ffi::c_uint,
                b"void test_recursion_get(void)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_6: {
        if 0 as ::core::ffi::c_int
            == json_pointer_getf(
                jo1,
                &raw mut jo2,
                b"/%s/%d/%s/%d/%s\0" as *const u8 as *const ::core::ffi::c_char,
                b"arr\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int,
                b"obj\0" as *const u8 as *const ::core::ffi::c_char,
                2 as ::core::ffi::c_int,
                b"obj2\0" as *const u8 as *const ::core::ffi::c_char,
            )
        {
        } else {
            __assert_fail(
                b"0 == json_pointer_getf(jo1, &jo2, \"/%s/%d/%s/%d/%s\", \"arr\", 0, \"obj\", 2, \"obj2\")\0"
                    as *const u8 as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_json_pointer.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                143 as ::core::ffi::c_uint,
                b"void test_recursion_get(void)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_7: {
        if json_object_is_type(jo2, json_type_string) != 0 {
        } else {
            __assert_fail(
                b"json_object_is_type(jo2, json_type_string)\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_json_pointer.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                144 as ::core::ffi::c_uint,
                b"void test_recursion_get(void)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_8: {
        if 0 as ::core::ffi::c_int
            == strcmp(
                b"1\0" as *const u8 as *const ::core::ffi::c_char,
                json_object_get_string(jo2),
            )
        {
        } else {
            __assert_fail(
                b"0 == strcmp(\"1\", json_object_get_string(jo2))\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_json_pointer.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                145 as ::core::ffi::c_uint,
                b"void test_recursion_get(void)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_9: {
        if !jo1.is_null() {
        } else {
            __assert_fail(
                b"jo1 != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_json_pointer.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                147 as ::core::ffi::c_uint,
                b"void test_recursion_get(void)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_10: {
        if 0 as ::core::ffi::c_int
            == json_pointer_get(
                jo1,
                b"/obj/obj/obj/0/obj1\0" as *const u8 as *const ::core::ffi::c_char,
                &raw mut jo2,
            )
        {
        } else {
            __assert_fail(
                b"0 == json_pointer_get(jo1, \"/obj/obj/obj/0/obj1\", &jo2)\0"
                    as *const u8 as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_json_pointer.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                148 as ::core::ffi::c_uint,
                b"void test_recursion_get(void)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_11: {
        if json_object_is_type(jo2, json_type_int) != 0 {
        } else {
            __assert_fail(
                b"json_object_is_type(jo2, json_type_int)\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_json_pointer.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                149 as ::core::ffi::c_uint,
                b"void test_recursion_get(void)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_12: {
        if 0 as int32_t == json_object_get_int(jo2) {
        } else {
            __assert_fail(
                b"0 == json_object_get_int(jo2)\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_json_pointer.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                150 as ::core::ffi::c_uint,
                b"void test_recursion_get(void)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_13: {
        if 0 as ::core::ffi::c_int
            == json_pointer_get(
                jo1,
                b"/obj/obj/obj/0/obj2\0" as *const u8 as *const ::core::ffi::c_char,
                &raw mut jo2,
            )
        {
        } else {
            __assert_fail(
                b"0 == json_pointer_get(jo1, \"/obj/obj/obj/0/obj2\", &jo2)\0"
                    as *const u8 as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_json_pointer.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                152 as ::core::ffi::c_uint,
                b"void test_recursion_get(void)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_14: {
        if json_object_is_type(jo2, json_type_string) != 0 {
        } else {
            __assert_fail(
                b"json_object_is_type(jo2, json_type_string)\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_json_pointer.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                153 as ::core::ffi::c_uint,
                b"void test_recursion_get(void)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_15: {
        if 0 as ::core::ffi::c_int
            == strcmp(
                b"1\0" as *const u8 as *const ::core::ffi::c_char,
                json_object_get_string(jo2),
            )
        {
        } else {
            __assert_fail(
                b"0 == strcmp(\"1\", json_object_get_string(jo2))\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_json_pointer.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                154 as ::core::ffi::c_uint,
                b"void test_recursion_get(void)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_16: {
        if 0 as ::core::ffi::c_int
            == json_pointer_getf(
                jo1,
                &raw mut jo2,
                b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                b"\0\0" as *const u8 as *const ::core::ffi::c_char,
            )
        {
        } else {
            __assert_fail(
                b"0 == json_pointer_getf(jo1, &jo2, \"%s\", \"\\0\")\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_json_pointer.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                156 as ::core::ffi::c_uint,
                b"void test_recursion_get(void)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    printf(b"PASSED - GET - RECURSION TEST\n\0" as *const u8 as *const ::core::ffi::c_char);
    json_object_put(jo1);
}
unsafe extern "C" fn test_wrong_inputs_get() {
    let mut jo2: *mut json_object = ::core::ptr::null_mut::<json_object>();
    let mut jo1: *mut json_object = json_tokener_parse(input_json_str);
    '_c2rust_label: {
        if !jo1.is_null() {
        } else {
            __assert_fail(
                b"NULL != jo1\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_json_pointer.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                167 as ::core::ffi::c_uint,
                b"void test_wrong_inputs_get(void)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    printf(b"PASSED - GET - LOADED TEST JSON\n\0" as *const u8 as *const ::core::ffi::c_char);
    printf(
        b"%s\n\0" as *const u8 as *const ::core::ffi::c_char,
        json_object_get_string(jo1),
    );
    jo2 = ::core::ptr::null_mut::<json_object>();
    *__errno_location() = 0 as ::core::ffi::c_int;
    '_c2rust_label_0: {
        if 0 as ::core::ffi::c_int
            != json_pointer_get(
                jo1,
                b"foo/bar\0" as *const u8 as *const ::core::ffi::c_char,
                ::core::ptr::null_mut::<*mut json_object>(),
            )
        {
        } else {
            __assert_fail(
                b"0 != json_pointer_get(jo1, \"foo/bar\", NULL)\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_json_pointer.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                174 as ::core::ffi::c_uint,
                b"void test_wrong_inputs_get(void)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_1: {
        if 0 as ::core::ffi::c_int
            != json_pointer_get(
                jo1,
                b"foo/bar\0" as *const u8 as *const ::core::ffi::c_char,
                &raw mut jo2,
            )
        {
        } else {
            __assert_fail(
                b"0 != json_pointer_get(jo1, \"foo/bar\", &jo2)\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_json_pointer.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                175 as ::core::ffi::c_uint,
                b"void test_wrong_inputs_get(void)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_2: {
        if *__errno_location() == 22 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"errno == EINVAL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_json_pointer.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                176 as ::core::ffi::c_uint,
                b"void test_wrong_inputs_get(void)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_3: {
        if jo2.is_null() {
        } else {
            __assert_fail(
                b"jo2 == NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_json_pointer.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                177 as ::core::ffi::c_uint,
                b"void test_wrong_inputs_get(void)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    printf(b"PASSED - GET - MISSING /\n\0" as *const u8 as *const ::core::ffi::c_char);
    *__errno_location() = 0 as ::core::ffi::c_int;
    '_c2rust_label_4: {
        if 0 as ::core::ffi::c_int
            != json_pointer_get(
                ::core::ptr::null_mut::<json_object>(),
                b"foo/bar\0" as *const u8 as *const ::core::ffi::c_char,
                ::core::ptr::null_mut::<*mut json_object>(),
            )
        {
        } else {
            __assert_fail(
                b"0 != json_pointer_get(NULL, \"foo/bar\", NULL)\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_json_pointer.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                182 as ::core::ffi::c_uint,
                b"void test_wrong_inputs_get(void)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_5: {
        if *__errno_location() == 22 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"errno == EINVAL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_json_pointer.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                183 as ::core::ffi::c_uint,
                b"void test_wrong_inputs_get(void)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    *__errno_location() = 0 as ::core::ffi::c_int;
    '_c2rust_label_6: {
        if 0 as ::core::ffi::c_int
            != json_pointer_get(
                ::core::ptr::null_mut::<json_object>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null_mut::<*mut json_object>(),
            )
        {
        } else {
            __assert_fail(
                b"0 != json_pointer_get(NULL, NULL, NULL)\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_json_pointer.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                185 as ::core::ffi::c_uint,
                b"void test_wrong_inputs_get(void)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_7: {
        if *__errno_location() == 22 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"errno == EINVAL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_json_pointer.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                186 as ::core::ffi::c_uint,
                b"void test_wrong_inputs_get(void)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    *__errno_location() = 0 as ::core::ffi::c_int;
    '_c2rust_label_8: {
        if 0 as ::core::ffi::c_int
            != json_pointer_getf(
                ::core::ptr::null_mut::<json_object>(),
                ::core::ptr::null_mut::<*mut json_object>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
            )
        {
        } else {
            __assert_fail(
                b"0 != json_pointer_getf(NULL, NULL, NULL)\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_json_pointer.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                188 as ::core::ffi::c_uint,
                b"void test_wrong_inputs_get(void)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_9: {
        if *__errno_location() == 22 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"errno == EINVAL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_json_pointer.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                189 as ::core::ffi::c_uint,
                b"void test_wrong_inputs_get(void)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    *__errno_location() = 0 as ::core::ffi::c_int;
    '_c2rust_label_10: {
        if 0 as ::core::ffi::c_int
            != json_pointer_get(
                jo1,
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null_mut::<*mut json_object>(),
            )
        {
        } else {
            __assert_fail(
                b"0 != json_pointer_get(jo1, NULL, NULL)\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_json_pointer.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                191 as ::core::ffi::c_uint,
                b"void test_wrong_inputs_get(void)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_11: {
        if *__errno_location() == 22 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"errno == EINVAL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_json_pointer.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                192 as ::core::ffi::c_uint,
                b"void test_wrong_inputs_get(void)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    *__errno_location() = 0 as ::core::ffi::c_int;
    '_c2rust_label_12: {
        if 0 as ::core::ffi::c_int
            != json_pointer_getf(
                jo1,
                ::core::ptr::null_mut::<*mut json_object>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
            )
        {
        } else {
            __assert_fail(
                b"0 != json_pointer_getf(jo1, NULL, NULL)\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_json_pointer.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                194 as ::core::ffi::c_uint,
                b"void test_wrong_inputs_get(void)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_13: {
        if *__errno_location() == 22 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"errno == EINVAL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_json_pointer.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                195 as ::core::ffi::c_uint,
                b"void test_wrong_inputs_get(void)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    printf(b"PASSED - GET - NULL INPUTS\n\0" as *const u8 as *const ::core::ffi::c_char);
    *__errno_location() = 0 as ::core::ffi::c_int;
    '_c2rust_label_14: {
        if 0 as ::core::ffi::c_int
            != json_pointer_get(
                jo1,
                b"/foo/a\0" as *const u8 as *const ::core::ffi::c_char,
                ::core::ptr::null_mut::<*mut json_object>(),
            )
        {
        } else {
            __assert_fail(
                b"0 != json_pointer_get(jo1, \"/foo/a\", NULL)\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_json_pointer.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                200 as ::core::ffi::c_uint,
                b"void test_wrong_inputs_get(void)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_15: {
        if *__errno_location() == 22 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"errno == EINVAL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_json_pointer.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                201 as ::core::ffi::c_uint,
                b"void test_wrong_inputs_get(void)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    *__errno_location() = 0 as ::core::ffi::c_int;
    '_c2rust_label_16: {
        if 0 as ::core::ffi::c_int
            != json_pointer_get(
                jo1,
                b"/foo/01\0" as *const u8 as *const ::core::ffi::c_char,
                ::core::ptr::null_mut::<*mut json_object>(),
            )
        {
        } else {
            __assert_fail(
                b"0 != json_pointer_get(jo1, \"/foo/01\", NULL)\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_json_pointer.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                203 as ::core::ffi::c_uint,
                b"void test_wrong_inputs_get(void)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_17: {
        if *__errno_location() == 22 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"errno == EINVAL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_json_pointer.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                204 as ::core::ffi::c_uint,
                b"void test_wrong_inputs_get(void)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    *__errno_location() = 0 as ::core::ffi::c_int;
    '_c2rust_label_18: {
        if 0 as ::core::ffi::c_int
            != json_pointer_getf(
                jo1,
                ::core::ptr::null_mut::<*mut json_object>(),
                b"/%s/a\0" as *const u8 as *const ::core::ffi::c_char,
                b"foo\0" as *const u8 as *const ::core::ffi::c_char,
            )
        {
        } else {
            __assert_fail(
                b"0 != json_pointer_getf(jo1, NULL, \"/%s/a\", \"foo\")\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_json_pointer.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                206 as ::core::ffi::c_uint,
                b"void test_wrong_inputs_get(void)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_19: {
        if *__errno_location() == 22 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"errno == EINVAL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_json_pointer.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                207 as ::core::ffi::c_uint,
                b"void test_wrong_inputs_get(void)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    *__errno_location() = 0 as ::core::ffi::c_int;
    '_c2rust_label_20: {
        if 0 as ::core::ffi::c_int
            != json_pointer_get(
                jo1,
                b"/foo/-\0" as *const u8 as *const ::core::ffi::c_char,
                ::core::ptr::null_mut::<*mut json_object>(),
            )
        {
        } else {
            __assert_fail(
                b"0 != json_pointer_get(jo1, \"/foo/-\", NULL)\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_json_pointer.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                209 as ::core::ffi::c_uint,
                b"void test_wrong_inputs_get(void)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_21: {
        if *__errno_location() == 22 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"errno == EINVAL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_json_pointer.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                210 as ::core::ffi::c_uint,
                b"void test_wrong_inputs_get(void)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    *__errno_location() = 0 as ::core::ffi::c_int;
    '_c2rust_label_22: {
        if 0 as ::core::ffi::c_int
            != json_pointer_get(
                jo1,
                b"/foo/4\0" as *const u8 as *const ::core::ffi::c_char,
                ::core::ptr::null_mut::<*mut json_object>(),
            )
        {
        } else {
            __assert_fail(
                b"0 != json_pointer_get(jo1, \"/foo/4\", NULL)\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_json_pointer.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                213 as ::core::ffi::c_uint,
                b"void test_wrong_inputs_get(void)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_23: {
        if *__errno_location() == 2 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"errno == ENOENT\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_json_pointer.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                214 as ::core::ffi::c_uint,
                b"void test_wrong_inputs_get(void)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    *__errno_location() = 0 as ::core::ffi::c_int;
    '_c2rust_label_24: {
        if 0 as ::core::ffi::c_int
            != json_pointer_getf(
                jo1,
                ::core::ptr::null_mut::<*mut json_object>(),
                b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                b"/foo/22\0" as *const u8 as *const ::core::ffi::c_char,
            )
        {
        } else {
            __assert_fail(
                b"0 != json_pointer_getf(jo1, NULL, \"%s\", \"/foo/22\")\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_json_pointer.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                217 as ::core::ffi::c_uint,
                b"void test_wrong_inputs_get(void)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_25: {
        if *__errno_location() == 2 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"errno == ENOENT\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_json_pointer.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                218 as ::core::ffi::c_uint,
                b"void test_wrong_inputs_get(void)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    *__errno_location() = 0 as ::core::ffi::c_int;
    '_c2rust_label_26: {
        if 0 as ::core::ffi::c_int
            != json_pointer_getf(
                jo1,
                ::core::ptr::null_mut::<*mut json_object>(),
                b"/%s/%d\0" as *const u8 as *const ::core::ffi::c_char,
                b"foo\0" as *const u8 as *const ::core::ffi::c_char,
                22 as ::core::ffi::c_int,
            )
        {
        } else {
            __assert_fail(
                b"0 != json_pointer_getf(jo1, NULL, \"/%s/%d\", \"foo\", 22)\0"
                    as *const u8 as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_json_pointer.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                220 as ::core::ffi::c_uint,
                b"void test_wrong_inputs_get(void)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_27: {
        if *__errno_location() == 2 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"errno == ENOENT\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_json_pointer.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                221 as ::core::ffi::c_uint,
                b"void test_wrong_inputs_get(void)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    *__errno_location() = 0 as ::core::ffi::c_int;
    '_c2rust_label_28: {
        if 0 as ::core::ffi::c_int
            != json_pointer_get(
                jo1,
                b"/foo/-1\0" as *const u8 as *const ::core::ffi::c_char,
                ::core::ptr::null_mut::<*mut json_object>(),
            )
        {
        } else {
            __assert_fail(
                b"0 != json_pointer_get(jo1, \"/foo/-1\", NULL)\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_json_pointer.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                223 as ::core::ffi::c_uint,
                b"void test_wrong_inputs_get(void)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_29: {
        if *__errno_location() == 22 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"errno == EINVAL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_json_pointer.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                224 as ::core::ffi::c_uint,
                b"void test_wrong_inputs_get(void)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    *__errno_location() = 0 as ::core::ffi::c_int;
    '_c2rust_label_30: {
        if 0 as ::core::ffi::c_int
            != json_pointer_get(
                jo1,
                b"/foo/10\0" as *const u8 as *const ::core::ffi::c_char,
                ::core::ptr::null_mut::<*mut json_object>(),
            )
        {
        } else {
            __assert_fail(
                b"0 != json_pointer_get(jo1, \"/foo/10\", NULL)\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_json_pointer.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                226 as ::core::ffi::c_uint,
                b"void test_wrong_inputs_get(void)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_31: {
        if *__errno_location() == 2 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"errno == ENOENT\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_json_pointer.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                227 as ::core::ffi::c_uint,
                b"void test_wrong_inputs_get(void)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    printf(b"PASSED - GET - INVALID INDEXES\n\0" as *const u8 as *const ::core::ffi::c_char);
    json_object_put(jo1);
}
unsafe extern "C" fn test_example_set() {
    let mut jo2: *mut json_object = ::core::ptr::null_mut::<json_object>();
    let mut jo1: *mut json_object = json_tokener_parse(input_json_str);
    '_c2rust_label: {
        if !jo1.is_null() {
        } else {
            __assert_fail(
                b"jo1 != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_json_pointer.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                237 as ::core::ffi::c_uint,
                b"void test_example_set(void)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    printf(b"PASSED - SET - LOADED TEST JSON\n\0" as *const u8 as *const ::core::ffi::c_char);
    printf(
        b"%s\n\0" as *const u8 as *const ::core::ffi::c_char,
        json_object_get_string(jo1),
    );
    '_c2rust_label_0: {
        if 0 as ::core::ffi::c_int
            == json_pointer_set(
                &raw mut jo1,
                b"/foo/1\0" as *const u8 as *const ::core::ffi::c_char,
                json_object_new_string(b"cod\0" as *const u8 as *const ::core::ffi::c_char),
            )
        {
        } else {
            __assert_fail(
                b"0 == json_pointer_set(&jo1, \"/foo/1\", json_object_new_string(\"cod\"))\0"
                    as *const u8 as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_json_pointer.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                241 as ::core::ffi::c_uint,
                b"void test_example_set(void)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_1: {
        if 0 as ::core::ffi::c_int
            == strcmp(
                b"cod\0" as *const u8 as *const ::core::ffi::c_char,
                json_object_get_string(json_object_array_get_idx(
                    json_object_object_get(
                        jo1,
                        b"foo\0" as *const u8 as *const ::core::ffi::c_char,
                    ),
                    1 as size_t,
                )),
            )
        {
        } else {
            __assert_fail(
                b"0 == strcmp(\"cod\", json_object_get_string(json_object_array_get_idx( json_object_object_get(jo1, \"foo\"), 1)))\0"
                    as *const u8 as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_json_pointer.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                243 as ::core::ffi::c_uint,
                b"void test_example_set(void)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    printf(b"PASSED - SET - 'cod' in /foo/1\n\0" as *const u8 as *const ::core::ffi::c_char);
    '_c2rust_label_2: {
        jo2 = json_tokener_parse(b"[1,2,3]\0" as *const u8 as *const ::core::ffi::c_char);
        if 0 as ::core::ffi::c_int
            != json_pointer_set(
                &raw mut jo1,
                b"/fud/gaw\0" as *const u8 as *const ::core::ffi::c_char,
                jo2,
            )
        {
        } else {
            __assert_fail(
                b"0 != json_pointer_set(&jo1, \"/fud/gaw\", (jo2 = json_tokener_parse(\"[1,2,3]\")))\0"
                    as *const u8 as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_json_pointer.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                245 as ::core::ffi::c_uint,
                b"void test_example_set(void)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_3: {
        if *__errno_location() == 2 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"errno == ENOENT\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_json_pointer.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                246 as ::core::ffi::c_uint,
                b"void test_example_set(void)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    printf(b"PASSED - SET - non-existing /fud/gaw\n\0" as *const u8 as *const ::core::ffi::c_char);
    '_c2rust_label_4: {
        if 0 as ::core::ffi::c_int
            == json_pointer_set(
                &raw mut jo1,
                b"/fud\0" as *const u8 as *const ::core::ffi::c_char,
                json_object_new_object(),
            )
        {
        } else {
            __assert_fail(
                b"0 == json_pointer_set(&jo1, \"/fud\", json_object_new_object())\0"
                    as *const u8 as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_json_pointer.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                248 as ::core::ffi::c_uint,
                b"void test_example_set(void)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    printf(b"PASSED - SET - /fud == {}\n\0" as *const u8 as *const ::core::ffi::c_char);
    '_c2rust_label_5: {
        if 0 as ::core::ffi::c_int
            == json_pointer_set(
                &raw mut jo1,
                b"/fud/gaw\0" as *const u8 as *const ::core::ffi::c_char,
                jo2,
            )
        {
        } else {
            __assert_fail(
                b"0 == json_pointer_set(&jo1, \"/fud/gaw\", jo2)\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_json_pointer.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                250 as ::core::ffi::c_uint,
                b"void test_example_set(void)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    printf(b"PASSED - SET - /fug/gaw == [1,2,3]\n\0" as *const u8 as *const ::core::ffi::c_char);
    '_c2rust_label_6: {
        if 0 as ::core::ffi::c_int
            == json_pointer_set(
                &raw mut jo1,
                b"/fud/gaw/0\0" as *const u8 as *const ::core::ffi::c_char,
                json_object_new_int(0 as int32_t),
            )
        {
        } else {
            __assert_fail(
                b"0 == json_pointer_set(&jo1, \"/fud/gaw/0\", json_object_new_int(0))\0"
                    as *const u8 as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_json_pointer.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                252 as ::core::ffi::c_uint,
                b"void test_example_set(void)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_7: {
        if 0 as ::core::ffi::c_int
            == json_pointer_setf(
                &raw mut jo1,
                json_object_new_int(0 as int32_t),
                b"%s%s/%d\0" as *const u8 as *const ::core::ffi::c_char,
                b"/fud\0" as *const u8 as *const ::core::ffi::c_char,
                b"/gaw\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int,
            )
        {
        } else {
            __assert_fail(
                b"0 == json_pointer_setf(&jo1, json_object_new_int(0), \"%s%s/%d\", \"/fud\", \"/gaw\", 0)\0"
                    as *const u8 as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_json_pointer.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                253 as ::core::ffi::c_uint,
                b"void test_example_set(void)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    printf(b"PASSED - SET - /fug/gaw == [0,2,3]\n\0" as *const u8 as *const ::core::ffi::c_char);
    '_c2rust_label_8: {
        if 0 as ::core::ffi::c_int
            == json_pointer_set(
                &raw mut jo1,
                b"/fud/gaw/-\0" as *const u8 as *const ::core::ffi::c_char,
                json_object_new_int(4 as int32_t),
            )
        {
        } else {
            __assert_fail(
                b"0 == json_pointer_set(&jo1, \"/fud/gaw/-\", json_object_new_int(4))\0"
                    as *const u8 as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_json_pointer.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                255 as ::core::ffi::c_uint,
                b"void test_example_set(void)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    printf(b"PASSED - SET - /fug/gaw == [0,2,3,4]\n\0" as *const u8 as *const ::core::ffi::c_char);
    '_c2rust_label_9: {
        if 0 as ::core::ffi::c_int
            == json_pointer_set(
                &raw mut jo1,
                b"/\0" as *const u8 as *const ::core::ffi::c_char,
                json_object_new_int(9 as int32_t),
            )
        {
        } else {
            __assert_fail(
                b"0 == json_pointer_set(&jo1, \"/\", json_object_new_int(9))\0"
                    as *const u8 as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_json_pointer.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                257 as ::core::ffi::c_uint,
                b"void test_example_set(void)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    printf(b"PASSED - SET - / == 9\n\0" as *const u8 as *const ::core::ffi::c_char);
    jo2 = json_tokener_parse(
        b"{ 'foo': [ 'bar', 'cod' ], '': 9, 'a/b': 1, 'c%d': 2, 'e^f': 3, 'g|h': 4, 'i\\\\j': 5, 'k\\\"l': 6, ' ': 7, 'm~n': 8, 'fud': { 'gaw': [ 0, 2, 3, 4 ] } }\0"
            as *const u8 as *const ::core::ffi::c_char,
    );
    '_c2rust_label_10: {
        if json_object_equal(jo2, jo1) != 0 {
        } else {
            __assert_fail(
                b"json_object_equal(jo2, jo1)\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_json_pointer.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                263 as ::core::ffi::c_uint,
                b"void test_example_set(void)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    printf(
        b"PASSED - SET - Final JSON is: %s\n\0" as *const u8 as *const ::core::ffi::c_char,
        json_object_get_string(jo1),
    );
    json_object_put(jo2);
    '_c2rust_label_11: {
        if 0 as ::core::ffi::c_int
            == json_pointer_set(
                &raw mut jo1,
                b"\0" as *const u8 as *const ::core::ffi::c_char,
                json_object_new_int(10 as int32_t),
            )
        {
        } else {
            __assert_fail(
                b"0 == json_pointer_set(&jo1, \"\", json_object_new_int(10))\0"
                    as *const u8 as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_json_pointer.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                267 as ::core::ffi::c_uint,
                b"void test_example_set(void)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_12: {
        if 10 as int32_t == json_object_get_int(jo1) {
        } else {
            __assert_fail(
                b"10 == json_object_get_int(jo1)\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_json_pointer.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                268 as ::core::ffi::c_uint,
                b"void test_example_set(void)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    printf(
        b"%s\n\0" as *const u8 as *const ::core::ffi::c_char,
        json_object_get_string(jo1),
    );
    json_object_put(jo1);
    jo1 = json_tokener_parse(b"[0, 1, 2, 3]\0" as *const u8 as *const ::core::ffi::c_char);
    jo2 = json_tokener_parse(
        b"[0, 1, 2, 3, null, null, null, 7]\0" as *const u8 as *const ::core::ffi::c_char,
    );
    '_c2rust_label_13: {
        if 0 as ::core::ffi::c_int
            == json_pointer_set(
                &raw mut jo1,
                b"/7\0" as *const u8 as *const ::core::ffi::c_char,
                json_object_new_int(7 as int32_t),
            )
        {
        } else {
            __assert_fail(
                b"0 == json_pointer_set(&jo1, \"/7\", json_object_new_int(7))\0"
                    as *const u8 as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_json_pointer.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                276 as ::core::ffi::c_uint,
                b"void test_example_set(void)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_14: {
        if 1 as ::core::ffi::c_int == json_object_equal(jo1, jo2) {
        } else {
            __assert_fail(
                b"1 == json_object_equal(jo1, jo2)\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_json_pointer.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                277 as ::core::ffi::c_uint,
                b"void test_example_set(void)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    json_object_put(jo1);
    jo1 = json_tokener_parse(b"[0, 1, 2, 3]\0" as *const u8 as *const ::core::ffi::c_char);
    '_c2rust_label_15: {
        if 0 as ::core::ffi::c_int
            == json_pointer_setf(
                &raw mut jo1,
                json_object_new_int(7 as int32_t),
                b"/%u\0" as *const u8 as *const ::core::ffi::c_char,
                7 as ::core::ffi::c_int,
            )
        {
        } else {
            __assert_fail(
                b"0 == json_pointer_setf(&jo1, json_object_new_int(7), \"/%u\", 7)\0"
                    as *const u8 as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_json_pointer.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                283 as ::core::ffi::c_uint,
                b"void test_example_set(void)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_16: {
        if 1 as ::core::ffi::c_int == json_object_equal(jo1, jo2) {
        } else {
            __assert_fail(
                b"1 == json_object_equal(jo1, jo2)\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_json_pointer.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                284 as ::core::ffi::c_uint,
                b"void test_example_set(void)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    json_object_put(jo1);
    json_object_put(jo2);
}
unsafe extern "C" fn test_wrong_inputs_set() {
    let mut jo2: *mut json_object = ::core::ptr::null_mut::<json_object>();
    let mut jo1: *mut json_object = json_tokener_parse(input_json_str);
    '_c2rust_label: {
        if !jo1.is_null() {
        } else {
            __assert_fail(
                b"jo1 != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_json_pointer.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                294 as ::core::ffi::c_uint,
                b"void test_wrong_inputs_set(void)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    printf(b"PASSED - SET - LOADED TEST JSON\n\0" as *const u8 as *const ::core::ffi::c_char);
    printf(
        b"%s\n\0" as *const u8 as *const ::core::ffi::c_char,
        json_object_get_string(jo1),
    );
    '_c2rust_label_0: {
        if 0 as ::core::ffi::c_int
            != json_pointer_set(
                ::core::ptr::null_mut::<*mut json_object>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null_mut::<json_object>(),
            )
        {
        } else {
            __assert_fail(
                b"0 != json_pointer_set(NULL, NULL, NULL)\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_json_pointer.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                298 as ::core::ffi::c_uint,
                b"void test_wrong_inputs_set(void)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_1: {
        if 0 as ::core::ffi::c_int
            != json_pointer_setf(
                ::core::ptr::null_mut::<*mut json_object>(),
                ::core::ptr::null_mut::<json_object>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
            )
        {
        } else {
            __assert_fail(
                b"0 != json_pointer_setf(NULL, NULL, NULL)\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_json_pointer.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                299 as ::core::ffi::c_uint,
                b"void test_wrong_inputs_set(void)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_2: {
        if 0 as ::core::ffi::c_int
            != json_pointer_set(
                &raw mut jo1,
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null_mut::<json_object>(),
            )
        {
        } else {
            __assert_fail(
                b"0 != json_pointer_set(&jo1, NULL, NULL)\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_json_pointer.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                300 as ::core::ffi::c_uint,
                b"void test_wrong_inputs_set(void)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_3: {
        if 0 as ::core::ffi::c_int
            != json_pointer_setf(
                &raw mut jo1,
                ::core::ptr::null_mut::<json_object>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
            )
        {
        } else {
            __assert_fail(
                b"0 != json_pointer_setf(&jo1, NULL, NULL)\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_json_pointer.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                301 as ::core::ffi::c_uint,
                b"void test_wrong_inputs_set(void)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    printf(
        b"PASSED - SET - failed with NULL params for input json & path\n\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    '_c2rust_label_4: {
        jo2 = json_object_new_string(b"cod\0" as *const u8 as *const ::core::ffi::c_char);
        if 0 as ::core::ffi::c_int
            != json_pointer_set(
                &raw mut jo1,
                b"foo/bar\0" as *const u8 as *const ::core::ffi::c_char,
                jo2,
            )
        {
        } else {
            __assert_fail(
                b"0 != json_pointer_set(&jo1, \"foo/bar\", (jo2 = json_object_new_string(\"cod\")))\0"
                    as *const u8 as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_json_pointer.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                304 as ::core::ffi::c_uint,
                b"void test_wrong_inputs_set(void)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    printf(
        b"PASSED - SET - failed 'cod' with path 'foo/bar'\n\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    json_object_put(jo2);
    '_c2rust_label_5: {
        jo2 = json_object_new_string(b"cod\0" as *const u8 as *const ::core::ffi::c_char);
        if 0 as ::core::ffi::c_int
            != json_pointer_setf(
                &raw mut jo1,
                jo2,
                b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                b"foo/bar\0" as *const u8 as *const ::core::ffi::c_char,
            )
        {
        } else {
            __assert_fail(
                b"0 != json_pointer_setf(&jo1, (jo2 = json_object_new_string(\"cod\")), \"%s\", \"foo/bar\")\0"
                    as *const u8 as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_json_pointer.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                309 as ::core::ffi::c_uint,
                b"void test_wrong_inputs_set(void)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    printf(
        b"PASSED - SET - failed 'cod' with path 'foo/bar'\n\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    json_object_put(jo2);
    '_c2rust_label_6: {
        jo2 = json_object_new_string(b"cod\0" as *const u8 as *const ::core::ffi::c_char);
        if 0 as ::core::ffi::c_int
            != json_pointer_set(
                &raw mut jo1,
                b"0\0" as *const u8 as *const ::core::ffi::c_char,
                jo2,
            )
        {
        } else {
            __assert_fail(
                b"0 != json_pointer_set(&jo1, \"0\", (jo2 = json_object_new_string(\"cod\")))\0"
                    as *const u8 as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_json_pointer.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                313 as ::core::ffi::c_uint,
                b"void test_wrong_inputs_set(void)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    printf(
        b"PASSED - SET - failed with invalid array index'\n\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    json_object_put(jo2);
    jo2 = json_object_new_string(b"whatever\0" as *const u8 as *const ::core::ffi::c_char);
    '_c2rust_label_7: {
        if 0 as ::core::ffi::c_int
            != json_pointer_set(
                &raw mut jo1,
                b"/fud/gaw\0" as *const u8 as *const ::core::ffi::c_char,
                jo2,
            )
        {
        } else {
            __assert_fail(
                b"0 != json_pointer_set(&jo1, \"/fud/gaw\", jo2)\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_json_pointer.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                318 as ::core::ffi::c_uint,
                b"void test_wrong_inputs_set(void)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_8: {
        if 0 as ::core::ffi::c_int
            == json_pointer_set(
                &raw mut jo1,
                b"/fud\0" as *const u8 as *const ::core::ffi::c_char,
                json_object_new_object(),
            )
        {
        } else {
            __assert_fail(
                b"0 == json_pointer_set(&jo1, \"/fud\", json_object_new_object())\0"
                    as *const u8 as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_json_pointer.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                319 as ::core::ffi::c_uint,
                b"void test_wrong_inputs_set(void)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_9: {
        if 0 as ::core::ffi::c_int
            == json_pointer_set(
                &raw mut jo1,
                b"/fud/gaw\0" as *const u8 as *const ::core::ffi::c_char,
                jo2,
            )
        {
        } else {
            __assert_fail(
                b"0 == json_pointer_set(&jo1, \"/fud/gaw\", jo2)\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_json_pointer.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                320 as ::core::ffi::c_uint,
                b"void test_wrong_inputs_set(void)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    jo2 = json_object_new_int(0 as int32_t);
    '_c2rust_label_10: {
        if 0 as ::core::ffi::c_int
            != json_pointer_set(
                &raw mut jo1,
                b"/fud/gaw/0\0" as *const u8 as *const ::core::ffi::c_char,
                jo2,
            )
        {
        } else {
            __assert_fail(
                b"0 != json_pointer_set(&jo1, \"/fud/gaw/0\", jo2)\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_json_pointer.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                324 as ::core::ffi::c_uint,
                b"void test_wrong_inputs_set(void)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    json_object_put(jo2);
    jo2 = json_object_new_int(0 as int32_t);
    '_c2rust_label_11: {
        if 0 as ::core::ffi::c_int
            != json_pointer_set(
                &raw mut jo1,
                b"/fud/gaw/\0" as *const u8 as *const ::core::ffi::c_char,
                jo2,
            )
        {
        } else {
            __assert_fail(
                b"0 != json_pointer_set(&jo1, \"/fud/gaw/\", jo2)\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_json_pointer.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                327 as ::core::ffi::c_uint,
                b"void test_wrong_inputs_set(void)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    json_object_put(jo2);
    printf(
        b"PASSED - SET - failed to set index to non-array\n\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    '_c2rust_label_12: {
        if 0 as ::core::ffi::c_int
            == json_pointer_setf(
                &raw mut jo1,
                json_object_new_string(b"cod\0" as *const u8 as *const ::core::ffi::c_char),
                b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                b"\0\0" as *const u8 as *const ::core::ffi::c_char,
            )
        {
        } else {
            __assert_fail(
                b"0 == json_pointer_setf(&jo1, json_object_new_string(\"cod\"), \"%s\", \"\\0\")\0"
                    as *const u8 as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_json_pointer.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                331 as ::core::ffi::c_uint,
                b"void test_wrong_inputs_set(void)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    json_object_put(jo1);
}
unsafe fn main_0(
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    test_example_get();
    test_recursion_get();
    test_wrong_inputs_get();
    test_example_set();
    test_wrong_inputs_set();
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
