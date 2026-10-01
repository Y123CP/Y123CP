extern "C" {
    pub type json_object;
    fn __assert_fail(
        __assertion: *const ::core::ffi::c_char,
        __file: *const ::core::ffi::c_char,
        __line: ::core::ffi::c_uint,
        __function: *const ::core::ffi::c_char,
    ) -> !;
    fn printf(__format: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn json_object_put(obj: *mut json_object) -> ::core::ffi::c_int;
    fn json_object_new_int(i: int32_t) -> *mut json_object;
    fn json_object_new_int64(i: int64_t) -> *mut json_object;
    fn json_object_new_uint64(i: uint64_t) -> *mut json_object;
    fn json_object_get_int(obj: *const json_object) -> int32_t;
    fn json_object_int_inc(obj: *mut json_object, val: int64_t) -> ::core::ffi::c_int;
    fn json_object_get_int64(obj: *const json_object) -> int64_t;
    fn json_object_get_uint64(obj: *const json_object) -> uint64_t;
}
pub type __int32_t = i32;
pub type __int64_t = i64;
pub type __uint64_t = u64;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
pub type uint64_t = __uint64_t;
pub const __ASSERT_FUNCTION: [::core::ffi::c_char; 23] = unsafe {
    ::core::mem::transmute::<[u8; 23], [::core::ffi::c_char; 23]>(*b"int main(int, char **)\0")
};
pub const INT32_MIN: ::core::ffi::c_int =
    -(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int;
pub const INT64_MIN: ::core::ffi::c_long =
    -(9223372036854775807 as ::core::ffi::c_long) - 1 as ::core::ffi::c_long;
pub const INT32_MAX: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
pub const INT64_MAX: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
pub const UINT64_MAX: ::core::ffi::c_ulong = 18446744073709551615 as ::core::ffi::c_ulong;
unsafe fn main_0(
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut tmp: *mut json_object = json_object_new_int(123 as int32_t) as *mut json_object;
    json_object_int_inc(tmp as *mut json_object, 123 as int64_t);
    '_c2rust_label: {
        if json_object_get_int(tmp) == 246 as int32_t {
        } else {
            __assert_fail(
                b"json_object_get_int(tmp) == 246\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_int_add.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                13 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    json_object_put(tmp as *mut json_object);
    printf(b"INT ADD PASSED\n\0" as *const u8 as *const ::core::ffi::c_char);
    tmp = json_object_new_int(INT32_MAX as int32_t) as *mut json_object;
    json_object_int_inc(tmp as *mut json_object, 100 as int64_t);
    '_c2rust_label_0: {
        if json_object_get_int(tmp) == 2147483647 as int32_t {
        } else {
            __assert_fail(
                b"json_object_get_int(tmp) == INT32_MAX\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_int_add.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                18 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    '_c2rust_label_1: {
        if json_object_get_int64(tmp)
            == 2147483647 as ::core::ffi::c_int as int64_t + 100 as int64_t
        {
        } else {
            __assert_fail(
                b"json_object_get_int64(tmp) == (int64_t)INT32_MAX + 100L\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_int_add.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                19 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    json_object_put(tmp as *mut json_object);
    printf(b"INT ADD OVERFLOW PASSED\n\0" as *const u8 as *const ::core::ffi::c_char);
    tmp = json_object_new_int(INT32_MIN as int32_t) as *mut json_object;
    json_object_int_inc(
        tmp as *mut json_object,
        -(100 as ::core::ffi::c_int) as int64_t,
    );
    '_c2rust_label_2: {
        if json_object_get_int(tmp) == -(2147483647 as int32_t) - 1 as int32_t {
        } else {
            __assert_fail(
                b"json_object_get_int(tmp) == INT32_MIN\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_int_add.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                24 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    '_c2rust_label_3: {
        if json_object_get_int64(tmp)
            == (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int) as int64_t
                - 100 as int64_t
        {
        } else {
            __assert_fail(
                b"json_object_get_int64(tmp) == (int64_t)INT32_MIN - 100L\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_int_add.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                25 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    json_object_put(tmp as *mut json_object);
    printf(b"INT ADD UNDERFLOW PASSED\n\0" as *const u8 as *const ::core::ffi::c_char);
    tmp = json_object_new_int64(321321321 as int64_t) as *mut json_object;
    json_object_int_inc(tmp as *mut json_object, 321321321 as int64_t);
    '_c2rust_label_4: {
        if json_object_get_int(tmp) == 642642642 as int32_t {
        } else {
            __assert_fail(
                b"json_object_get_int(tmp) == 642642642\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_int_add.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                30 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    json_object_put(tmp as *mut json_object);
    printf(b"INT64 ADD PASSED\n\0" as *const u8 as *const ::core::ffi::c_char);
    tmp = json_object_new_int64(INT64_MAX as int64_t) as *mut json_object;
    json_object_int_inc(tmp as *mut json_object, 100 as int64_t);
    '_c2rust_label_5: {
        if json_object_get_int64(tmp) == 9223372036854775807 as int64_t {
        } else {
            __assert_fail(
                b"json_object_get_int64(tmp) == INT64_MAX\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_int_add.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                35 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    '_c2rust_label_6: {
        if json_object_get_uint64(tmp)
            == (9223372036854775807 as ::core::ffi::c_long as uint64_t)
                .wrapping_add(100 as uint64_t)
        {
        } else {
            __assert_fail(
                b"json_object_get_uint64(tmp) == (uint64_t)INT64_MAX + 100U\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_int_add.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                36 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    json_object_int_inc(
        tmp as *mut json_object,
        -(100 as ::core::ffi::c_int) as int64_t,
    );
    '_c2rust_label_7: {
        if json_object_get_int64(tmp) == 9223372036854775807 as int64_t {
        } else {
            __assert_fail(
                b"json_object_get_int64(tmp) == INT64_MAX\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_int_add.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                38 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    '_c2rust_label_8: {
        if json_object_get_uint64(tmp) == 9223372036854775807 as ::core::ffi::c_long as uint64_t {
        } else {
            __assert_fail(
                b"json_object_get_uint64(tmp) == (uint64_t)INT64_MAX\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_int_add.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                39 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    json_object_put(tmp as *mut json_object);
    printf(b"INT64 ADD OVERFLOW PASSED\n\0" as *const u8 as *const ::core::ffi::c_char);
    tmp = json_object_new_int64(INT64_MIN as int64_t) as *mut json_object;
    json_object_int_inc(
        tmp as *mut json_object,
        -(100 as ::core::ffi::c_int) as int64_t,
    );
    '_c2rust_label_9: {
        if json_object_get_int64(tmp) == -(9223372036854775807 as int64_t) - 1 as int64_t {
        } else {
            __assert_fail(
                b"json_object_get_int64(tmp) == INT64_MIN\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_int_add.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                44 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    json_object_int_inc(tmp as *mut json_object, 100 as int64_t);
    '_c2rust_label_10: {
        if json_object_get_int64(tmp) != -(9223372036854775807 as int64_t) - 1 as int64_t {
        } else {
            __assert_fail(
                b"json_object_get_int64(tmp) != INT64_MIN\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_int_add.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                46 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    json_object_put(tmp as *mut json_object);
    printf(b"INT64 ADD UNDERFLOW PASSED\n\0" as *const u8 as *const ::core::ffi::c_char);
    tmp = json_object_new_uint64(400 as uint64_t) as *mut json_object;
    json_object_int_inc(
        tmp as *mut json_object,
        -(200 as ::core::ffi::c_int) as int64_t,
    );
    '_c2rust_label_11: {
        if json_object_get_int64(tmp) == 200 as int64_t {
        } else {
            __assert_fail(
                b"json_object_get_int64(tmp) == 200\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_int_add.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                52 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    '_c2rust_label_12: {
        if json_object_get_uint64(tmp) == 200 as uint64_t {
        } else {
            __assert_fail(
                b"json_object_get_uint64(tmp) == 200\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_int_add.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                53 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    json_object_int_inc(tmp as *mut json_object, 200 as int64_t);
    '_c2rust_label_13: {
        if json_object_get_int64(tmp) == 400 as int64_t {
        } else {
            __assert_fail(
                b"json_object_get_int64(tmp) == 400\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_int_add.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                55 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    '_c2rust_label_14: {
        if json_object_get_uint64(tmp) == 400 as uint64_t {
        } else {
            __assert_fail(
                b"json_object_get_uint64(tmp) == 400\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_int_add.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                56 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    json_object_put(tmp as *mut json_object);
    printf(b"UINT64 ADD PASSED\n\0" as *const u8 as *const ::core::ffi::c_char);
    tmp = json_object_new_uint64((UINT64_MAX as uint64_t).wrapping_sub(50 as uint64_t))
        as *mut json_object;
    json_object_int_inc(tmp as *mut json_object, 100 as int64_t);
    '_c2rust_label_15: {
        if json_object_get_int64(tmp) == 9223372036854775807 as int64_t {
        } else {
            __assert_fail(
                b"json_object_get_int64(tmp) == INT64_MAX\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_int_add.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                61 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    '_c2rust_label_16: {
        if json_object_get_uint64(tmp) == 18446744073709551615 as uint64_t {
        } else {
            __assert_fail(
                b"json_object_get_uint64(tmp) == UINT64_MAX\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_int_add.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                62 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    json_object_put(tmp as *mut json_object);
    printf(b"UINT64 ADD OVERFLOW PASSED\n\0" as *const u8 as *const ::core::ffi::c_char);
    tmp = json_object_new_uint64(100 as uint64_t) as *mut json_object;
    json_object_int_inc(
        tmp as *mut json_object,
        -(200 as ::core::ffi::c_int) as int64_t,
    );
    '_c2rust_label_17: {
        if json_object_get_int64(tmp) == -(100 as ::core::ffi::c_int) as int64_t {
        } else {
            __assert_fail(
                b"json_object_get_int64(tmp) == -100\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_int_add.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                67 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    '_c2rust_label_18: {
        if json_object_get_uint64(tmp) == 0 as uint64_t {
        } else {
            __assert_fail(
                b"json_object_get_uint64(tmp) == 0\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_int_add.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                68 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    json_object_put(tmp as *mut json_object);
    printf(b"UINT64 ADD UNDERFLOW PASSED\n\0" as *const u8 as *const ::core::ffi::c_char);
    printf(b"PASSED\n\0" as *const u8 as *const ::core::ffi::c_char);
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
