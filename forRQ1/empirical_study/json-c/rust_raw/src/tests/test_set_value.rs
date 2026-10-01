extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type json_object;
    fn __assert_fail(
        __assertion: *const ::core::ffi::c_char,
        __file: *const ::core::ffi::c_char,
        __line: ::core::ffi::c_uint,
        __function: *const ::core::ffi::c_char,
    ) -> !;
    static mut stderr: *mut FILE;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn printf(__format: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strncmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn json_tokener_parse(str: *const ::core::ffi::c_char) -> *mut json_object;
    fn json_object_put(obj: *mut json_object) -> ::core::ffi::c_int;
    fn json_object_to_json_string(obj: *mut json_object) -> *const ::core::ffi::c_char;
    fn json_object_new_boolean(b: json_bool) -> *mut json_object;
    fn json_object_get_boolean(obj: *const json_object) -> json_bool;
    fn json_object_set_boolean(obj: *mut json_object, new_value: json_bool) -> ::core::ffi::c_int;
    fn json_object_new_int(i: int32_t) -> *mut json_object;
    fn json_object_new_uint64(i: uint64_t) -> *mut json_object;
    fn json_object_get_int(obj: *const json_object) -> int32_t;
    fn json_object_set_int(
        obj: *mut json_object,
        new_value: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn json_object_get_int64(obj: *const json_object) -> int64_t;
    fn json_object_get_uint64(obj: *const json_object) -> uint64_t;
    fn json_object_set_int64(obj: *mut json_object, new_value: int64_t) -> ::core::ffi::c_int;
    fn json_object_set_uint64(obj: *mut json_object, new_value: uint64_t) -> ::core::ffi::c_int;
    fn json_object_new_double(d: ::core::ffi::c_double) -> *mut json_object;
    fn json_object_get_double(obj: *const json_object) -> ::core::ffi::c_double;
    fn json_object_set_double(
        obj: *mut json_object,
        new_value: ::core::ffi::c_double,
    ) -> ::core::ffi::c_int;
    fn json_object_new_string(s: *const ::core::ffi::c_char) -> *mut json_object;
    fn json_object_get_string(obj: *mut json_object) -> *const ::core::ffi::c_char;
    fn json_object_set_string(
        obj: *mut json_object,
        new_value: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
}
pub type size_t = usize;
pub type __int32_t = i32;
pub type __int64_t = i64;
pub type __uint64_t = u64;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: ::core::ffi::c_int,
    pub _IO_read_ptr: *mut ::core::ffi::c_char,
    pub _IO_read_end: *mut ::core::ffi::c_char,
    pub _IO_read_base: *mut ::core::ffi::c_char,
    pub _IO_write_base: *mut ::core::ffi::c_char,
    pub _IO_write_ptr: *mut ::core::ffi::c_char,
    pub _IO_write_end: *mut ::core::ffi::c_char,
    pub _IO_buf_base: *mut ::core::ffi::c_char,
    pub _IO_buf_end: *mut ::core::ffi::c_char,
    pub _IO_save_base: *mut ::core::ffi::c_char,
    pub _IO_backup_base: *mut ::core::ffi::c_char,
    pub _IO_save_end: *mut ::core::ffi::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: ::core::ffi::c_int,
    pub _flags2: ::core::ffi::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: ::core::ffi::c_ushort,
    pub _vtable_offset: ::core::ffi::c_schar,
    pub _shortbuf: [::core::ffi::c_char; 1],
    pub _lock: *mut ::core::ffi::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut ::core::ffi::c_void,
    pub __pad5: size_t,
    pub _mode: ::core::ffi::c_int,
    pub _unused2: [::core::ffi::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
pub type uint64_t = __uint64_t;
pub type json_bool = ::core::ffi::c_int;
pub const __ASSERT_FUNCTION: [::core::ffi::c_char; 23] = unsafe {
    ::core::mem::transmute::<[u8; 23], [::core::ffi::c_char; 23]>(*b"int main(int, char **)\0")
};
unsafe fn main_0(
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut tmp: *mut json_object = json_object_new_int(123 as int32_t) as *mut json_object;
    '_c2rust_label: {
        if json_object_get_int(tmp) == 123 as int32_t {
        } else {
            __assert_fail(
                b"json_object_get_int(tmp) == 123\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_set_value.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                13 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    json_object_set_int(tmp as *mut json_object, 321 as ::core::ffi::c_int);
    '_c2rust_label_0: {
        if json_object_get_int(tmp) == 321 as int32_t {
        } else {
            __assert_fail(
                b"json_object_get_int(tmp) == 321\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_set_value.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                15 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    printf(b"INT PASSED\n\0" as *const u8 as *const ::core::ffi::c_char);
    json_object_set_int64(
        tmp as *mut json_object,
        321321321 as ::core::ffi::c_int as int64_t,
    );
    '_c2rust_label_1: {
        if json_object_get_int64(tmp) == 321321321 as int64_t {
        } else {
            __assert_fail(
                b"json_object_get_int64(tmp) == 321321321\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_set_value.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                18 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    json_object_put(tmp as *mut json_object);
    printf(b"INT64 PASSED\n\0" as *const u8 as *const ::core::ffi::c_char);
    tmp = json_object_new_uint64(123 as uint64_t) as *mut json_object;
    '_c2rust_label_2: {
        if json_object_get_boolean(tmp) == 1 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"json_object_get_boolean(tmp) == 1\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_set_value.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                22 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    '_c2rust_label_3: {
        if json_object_get_int(tmp) == 123 as int32_t {
        } else {
            __assert_fail(
                b"json_object_get_int(tmp) == 123\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_set_value.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                23 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    '_c2rust_label_4: {
        if json_object_get_int64(tmp) == 123 as int64_t {
        } else {
            __assert_fail(
                b"json_object_get_int64(tmp) == 123\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_set_value.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                24 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    '_c2rust_label_5: {
        if json_object_get_uint64(tmp) == 123 as uint64_t {
        } else {
            __assert_fail(
                b"json_object_get_uint64(tmp) == 123\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_set_value.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                25 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    '_c2rust_label_6: {
        if json_object_get_double(tmp) == 123.000000f64 {
        } else {
            __assert_fail(
                b"json_object_get_double(tmp) == 123.000000\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_set_value.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                26 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    json_object_set_uint64(
        tmp as *mut json_object,
        321321321 as ::core::ffi::c_int as uint64_t,
    );
    '_c2rust_label_7: {
        if json_object_get_uint64(tmp) == 321321321 as uint64_t {
        } else {
            __assert_fail(
                b"json_object_get_uint64(tmp) == 321321321\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_set_value.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                28 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    json_object_set_uint64(tmp as *mut json_object, 9223372036854775808 as uint64_t);
    '_c2rust_label_8: {
        if json_object_get_int(tmp) == 2147483647 as int32_t {
        } else {
            __assert_fail(
                b"json_object_get_int(tmp) == INT32_MAX\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_set_value.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                30 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    '_c2rust_label_9: {
        if json_object_get_uint64(tmp) == 9223372036854775808 as uint64_t {
        } else {
            __assert_fail(
                b"json_object_get_uint64(tmp) == 9223372036854775808U\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_set_value.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                31 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    json_object_put(tmp as *mut json_object);
    printf(b"UINT64 PASSED\n\0" as *const u8 as *const ::core::ffi::c_char);
    tmp = json_object_new_boolean(1 as json_bool) as *mut json_object;
    '_c2rust_label_10: {
        if json_object_get_boolean(tmp) == 1 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"json_object_get_boolean(tmp) == 1\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_set_value.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                35 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    json_object_set_boolean(tmp as *mut json_object, 0 as json_bool);
    '_c2rust_label_11: {
        if json_object_get_boolean(tmp) == 0 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"json_object_get_boolean(tmp) == 0\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_set_value.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                37 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    json_object_set_boolean(tmp as *mut json_object, 1 as json_bool);
    '_c2rust_label_12: {
        if json_object_get_boolean(tmp) == 1 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"json_object_get_boolean(tmp) == 1\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_set_value.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                39 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    json_object_put(tmp as *mut json_object);
    printf(b"BOOL PASSED\n\0" as *const u8 as *const ::core::ffi::c_char);
    tmp = json_object_new_double(12.34f64) as *mut json_object;
    '_c2rust_label_13: {
        if json_object_get_double(tmp) == 12.34f64 {
        } else {
            __assert_fail(
                b"json_object_get_double(tmp) == 12.34\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_set_value.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                43 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    json_object_set_double(tmp as *mut json_object, 34.56f64);
    '_c2rust_label_14: {
        if json_object_get_double(tmp) == 34.56f64 {
        } else {
            __assert_fail(
                b"json_object_get_double(tmp) == 34.56\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_set_value.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                45 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    json_object_set_double(tmp as *mut json_object, 6435.34f64);
    '_c2rust_label_15: {
        if json_object_get_double(tmp) == 6435.34f64 {
        } else {
            __assert_fail(
                b"json_object_get_double(tmp) == 6435.34\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_set_value.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                47 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    json_object_set_double(tmp as *mut json_object, 2e21f64);
    '_c2rust_label_16: {
        if json_object_get_int(tmp) == 2147483647 as int32_t {
        } else {
            __assert_fail(
                b"json_object_get_int(tmp) == INT32_MAX\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_set_value.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                49 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    '_c2rust_label_17: {
        if json_object_get_int64(tmp) == 9223372036854775807 as int64_t {
        } else {
            __assert_fail(
                b"json_object_get_int64(tmp) == INT64_MAX\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_set_value.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                50 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    '_c2rust_label_18: {
        if json_object_get_uint64(tmp) == 18446744073709551615 as uint64_t {
        } else {
            __assert_fail(
                b"json_object_get_uint64(tmp) == UINT64_MAX\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_set_value.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                51 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    json_object_set_double(tmp as *mut json_object, -2e21f64);
    '_c2rust_label_19: {
        if json_object_get_int(tmp) == -(2147483647 as int32_t) - 1 as int32_t {
        } else {
            __assert_fail(
                b"json_object_get_int(tmp) == INT32_MIN\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_set_value.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                53 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    '_c2rust_label_20: {
        if json_object_get_int64(tmp) == -(9223372036854775807 as int64_t) - 1 as int64_t {
        } else {
            __assert_fail(
                b"json_object_get_int64(tmp) == INT64_MIN\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_set_value.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                54 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    '_c2rust_label_21: {
        if json_object_get_uint64(tmp) == 0 as uint64_t {
        } else {
            __assert_fail(
                b"json_object_get_uint64(tmp) == 0\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_set_value.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                55 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    json_object_put(tmp as *mut json_object);
    printf(b"DOUBLE PASSED\n\0" as *const u8 as *const ::core::ffi::c_char);
    tmp = json_object_new_string(MID.as_ptr()) as *mut json_object;
    '_c2rust_label_22: {
        if strcmp(
            json_object_get_string(tmp as *mut json_object),
            b"A MID STRING\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
        } else {
            __assert_fail(
                b"strcmp(json_object_get_string(tmp), MID) == 0\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_set_value.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                63 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    '_c2rust_label_23: {
        if strcmp(
            json_object_to_json_string(tmp as *mut json_object),
            b"\"A MID STRING\"\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
        } else {
            __assert_fail(
                b"strcmp(json_object_to_json_string(tmp), \"\\\"\" MID \"\\\"\") == 0\0"
                    as *const u8 as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_set_value.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                64 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    json_object_set_string(tmp, SHORT.as_ptr());
    '_c2rust_label_24: {
        if strcmp(
            json_object_get_string(tmp as *mut json_object),
            b"SHORT\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
        } else {
            __assert_fail(
                b"strcmp(json_object_get_string(tmp), SHORT) == 0\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_set_value.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                66 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    '_c2rust_label_25: {
        if strcmp(
            json_object_to_json_string(tmp as *mut json_object),
            b"\"SHORT\"\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
        } else {
            __assert_fail(
                b"strcmp(json_object_to_json_string(tmp), \"\\\"\" SHORT \"\\\"\") == 0\0"
                    as *const u8 as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_set_value.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                67 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    json_object_set_string(tmp, HUGE.as_ptr());
    '_c2rust_label_26: {
        if strcmp(
            json_object_get_string(tmp as *mut json_object),
            b"A string longer than 32 chars as to check non local buf codepath\0" as *const u8
                as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
        } else {
            __assert_fail(
                b"strcmp(json_object_get_string(tmp), HUGE) == 0\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_set_value.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                69 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    '_c2rust_label_27: {
        if strcmp(
            json_object_to_json_string(tmp as *mut json_object),
            b"\"A string longer than 32 chars as to check non local buf codepath\"\0" as *const u8
                as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
        } else {
            __assert_fail(
                b"strcmp(json_object_to_json_string(tmp), \"\\\"\" HUGE \"\\\"\") == 0\0"
                    as *const u8 as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_set_value.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                70 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    json_object_set_string(tmp, SHORT.as_ptr());
    '_c2rust_label_28: {
        if strcmp(
            json_object_get_string(tmp as *mut json_object),
            b"SHORT\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
        } else {
            __assert_fail(
                b"strcmp(json_object_get_string(tmp), SHORT) == 0\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_set_value.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                72 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    '_c2rust_label_29: {
        if strcmp(
            json_object_to_json_string(tmp as *mut json_object),
            b"\"SHORT\"\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
        } else {
            __assert_fail(
                b"strcmp(json_object_to_json_string(tmp), \"\\\"\" SHORT \"\\\"\") == 0\0"
                    as *const u8 as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_set_value.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                73 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    json_object_set_string(tmp, b"\0" as *const u8 as *const ::core::ffi::c_char);
    json_object_set_string(tmp, HUGE.as_ptr());
    json_object_set_string(tmp, b"\0" as *const u8 as *const ::core::ffi::c_char);
    json_object_set_string(tmp, HUGE.as_ptr());
    json_object_put(tmp as *mut json_object);
    printf(b"STRING PASSED\n\0" as *const u8 as *const ::core::ffi::c_char);
    tmp = json_object_new_string(STR.as_ptr()) as *mut json_object;
    '_c2rust_label_30: {
        if json_object_get_double(tmp) == 0.0f64 {
        } else {
            __assert_fail(
                b"json_object_get_double(tmp) == 0.0\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_set_value.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                92 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    json_object_set_string(tmp, DOUBLE.as_ptr());
    '_c2rust_label_31: {
        if json_object_get_double(tmp) == 123.123000f64 {
        } else {
            __assert_fail(
                b"json_object_get_double(tmp) == 123.123000\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_set_value.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                94 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    json_object_set_string(tmp, DOUBLE_E.as_ptr());
    '_c2rust_label_32: {
        if json_object_get_double(tmp) == 12000.000000f64 {
        } else {
            __assert_fail(
                b"json_object_get_double(tmp) == 12000.000000\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_set_value.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                96 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    json_object_set_string(tmp, DOUBLE_STR.as_ptr());
    '_c2rust_label_33: {
        if json_object_get_double(tmp) == 0.0f64 {
        } else {
            __assert_fail(
                b"json_object_get_double(tmp) == 0.0\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_set_value.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                98 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    json_object_set_string(tmp, DOUBLE_OVER.as_ptr());
    '_c2rust_label_34: {
        if json_object_get_double(tmp) == 0.0f64 {
        } else {
            __assert_fail(
                b"json_object_get_double(tmp) == 0.0\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_set_value.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                100 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    json_object_set_string(tmp, DOUBLE_OVER_NEGATIVE.as_ptr());
    '_c2rust_label_35: {
        if json_object_get_double(tmp) == 0.0f64 {
        } else {
            __assert_fail(
                b"json_object_get_double(tmp) == 0.0\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_set_value.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                102 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    json_object_put(tmp as *mut json_object);
    printf(b"STRINGTODOUBLE PASSED\n\0" as *const u8 as *const ::core::ffi::c_char);
    tmp = json_tokener_parse(b"1.234\0" as *const u8 as *const ::core::ffi::c_char)
        as *mut json_object;
    json_object_set_double(tmp as *mut json_object, 12.3f64);
    let mut serialized: *const ::core::ffi::c_char =
        json_object_to_json_string(tmp as *mut json_object);
    fprintf(
        stderr,
        b"%s\n\0" as *const u8 as *const ::core::ffi::c_char,
        serialized,
    );
    '_c2rust_label_36: {
        if strncmp(
            serialized,
            b"12.3\0" as *const u8 as *const ::core::ffi::c_char,
            4 as size_t,
        ) == 0 as ::core::ffi::c_int
        {
        } else {
            __assert_fail(
                b"strncmp(serialized, \"12.3\", 4) == 0\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_set_value.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                110 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    json_object_put(tmp as *mut json_object);
    printf(b"PARSE AND SET PASSED\n\0" as *const u8 as *const ::core::ffi::c_char);
    printf(b"PASSED\n\0" as *const u8 as *const ::core::ffi::c_char);
    return 0 as ::core::ffi::c_int;
}
pub const SHORT: [::core::ffi::c_char; 6] =
    unsafe { ::core::mem::transmute::<[u8; 6], [::core::ffi::c_char; 6]>(*b"SHORT\0") };
pub const MID: [::core::ffi::c_char; 13] =
    unsafe { ::core::mem::transmute::<[u8; 13], [::core::ffi::c_char; 13]>(*b"A MID STRING\0") };
pub const HUGE: [::core::ffi::c_char; 65] = unsafe {
    ::core::mem::transmute::<[u8; 65], [::core::ffi::c_char; 65]>(
        *b"A string longer than 32 chars as to check non local buf codepath\0",
    )
};
pub const STR: [::core::ffi::c_char; 4] =
    unsafe { ::core::mem::transmute::<[u8; 4], [::core::ffi::c_char; 4]>(*b"STR\0") };
pub const DOUBLE: [::core::ffi::c_char; 8] =
    unsafe { ::core::mem::transmute::<[u8; 8], [::core::ffi::c_char; 8]>(*b"123.123\0") };
pub const DOUBLE_E: [::core::ffi::c_char; 6] =
    unsafe { ::core::mem::transmute::<[u8; 6], [::core::ffi::c_char; 6]>(*b"12E+3\0") };
pub const DOUBLE_STR: [::core::ffi::c_char; 11] =
    unsafe { ::core::mem::transmute::<[u8; 11], [::core::ffi::c_char; 11]>(*b"123.123STR\0") };
pub const DOUBLE_OVER: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"1.8E+308\0") };
pub const DOUBLE_OVER_NEGATIVE: [::core::ffi::c_char; 10] =
    unsafe { ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(*b"-1.8E+308\0") };
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
