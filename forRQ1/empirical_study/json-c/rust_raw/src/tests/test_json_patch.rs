extern "C" {
    pub type json_object;
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn json_object_put(obj: *mut json_object) -> ::core::ffi::c_int;
    fn json_object_to_json_string(obj: *mut json_object) -> *const ::core::ffi::c_char;
    fn json_object_object_get(
        obj: *const json_object,
        key: *const ::core::ffi::c_char,
    ) -> *mut json_object;
    fn json_object_object_get_ex(
        obj: *const json_object,
        key: *const ::core::ffi::c_char,
        value: *mut *mut json_object,
    ) -> json_bool;
    fn json_object_array_length(obj: *const json_object) -> size_t;
    fn json_object_array_get_idx(obj: *const json_object, idx: size_t) -> *mut json_object;
    fn json_object_get_string(obj: *mut json_object) -> *const ::core::ffi::c_char;
    fn json_object_equal(obj1: *mut json_object, obj2: *mut json_object) -> ::core::ffi::c_int;
    fn strncmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn _json_c_strerror(errno_in: ::core::ffi::c_int) -> *mut ::core::ffi::c_char;
    fn __assert_fail(
        __assertion: *const ::core::ffi::c_char,
        __file: *const ::core::ffi::c_char,
        __line: ::core::ffi::c_uint,
        __function: *const ::core::ffi::c_char,
    ) -> !;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fflush(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn printf(__format: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn exit(__status: ::core::ffi::c_int) -> !;
    fn json_c_version() -> *const ::core::ffi::c_char;
    fn json_c_version_num() -> ::core::ffi::c_int;
    fn json_patch_apply(
        copy_from: *mut json_object,
        patch: *mut json_object,
        base: *mut *mut json_object,
        patch_error: *mut json_patch_error,
    ) -> ::core::ffi::c_int;
    fn json_object_from_file(filename: *const ::core::ffi::c_char) -> *mut json_object;
}
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type json_bool = ::core::ffi::c_int;
pub type size_t = usize;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct json_patch_error {
    pub errno_code: ::core::ffi::c_int,
    pub patch_failure_idx: size_t,
    pub errmsg: *const ::core::ffi::c_char,
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const __ASSERT_FUNCTION: [::core::ffi::c_char; 46] = unsafe {
    ::core::mem::transmute::<[u8; 46], [::core::ffi::c_char; 46]>(
        *b"void test_json_patch_op(struct json_object *)\0",
    )
};
pub const EXIT_FAILURE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const JSON_C_MAJOR_VERSION: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const JSON_C_MINOR_VERSION: ::core::ffi::c_int = 17 as ::core::ffi::c_int;
pub const JSON_C_MICRO_VERSION: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const JSON_C_VERSION_NUM: ::core::ffi::c_int = JSON_C_MAJOR_VERSION << 16 as ::core::ffi::c_int
    | JSON_C_MINOR_VERSION << 8 as ::core::ffi::c_int
    | JSON_C_MICRO_VERSION;
pub const JSON_C_VERSION: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"0.17\0") };
#[no_mangle]
pub unsafe extern "C" fn test_json_patch_op(mut jo: *mut json_object) {
    let mut comment: *const ::core::ffi::c_char = json_object_get_string(json_object_object_get(
        jo,
        b"comment\0" as *const u8 as *const ::core::ffi::c_char,
    ));
    let mut doc: *mut json_object =
        json_object_object_get(jo, b"doc\0" as *const u8 as *const ::core::ffi::c_char);
    let mut patch: *mut json_object =
        json_object_object_get(jo, b"patch\0" as *const u8 as *const ::core::ffi::c_char);
    let mut expected: *mut json_object = ::core::ptr::null_mut::<json_object>();
    let mut have_expected: json_bool = json_object_object_get_ex(
        jo,
        b"expected\0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut expected,
    );
    let mut error: *mut json_object =
        json_object_object_get(jo, b"error\0" as *const u8 as *const ::core::ffi::c_char);
    let mut error_s: *const ::core::ffi::c_char = json_object_get_string(error);
    let mut res: *mut json_object = ::core::ptr::null_mut::<json_object>();
    let mut ret: ::core::ffi::c_int = 0;
    printf(
        b"Testing '%s', doc '%s' patch '%s' : \0" as *const u8 as *const ::core::ffi::c_char,
        if !comment.is_null() { comment } else { error_s },
        json_object_get_string(doc),
        json_object_get_string(patch),
    );
    if error.is_null() && have_expected == 0 {
        printf(
            b"BAD TEST - no expected or error conditions in test: %s\n\0" as *const u8
                as *const ::core::ffi::c_char,
            json_object_to_json_string(jo),
        );
        '_c2rust_label: {
            __assert_fail(
                b"0\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_json_patch.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                37 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        };
    }
    fflush(stdout);
    let mut jperr: json_patch_error = json_patch_error {
        errno_code: 0,
        patch_failure_idx: 0,
        errmsg: ::core::ptr::null::<::core::ffi::c_char>(),
    };
    if !error.is_null() {
        '_c2rust_label_0: {
            if -(1 as ::core::ffi::c_int)
                == json_patch_apply(doc, patch, &raw mut res, &raw mut jperr)
            {
            } else {
                __assert_fail(
                    b"-1 == json_patch_apply(doc, patch, &res, &jperr)\0" as *const u8
                        as *const ::core::ffi::c_char,
                    b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_json_patch.c\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    42 as ::core::ffi::c_uint,
                    __ASSERT_FUNCTION.as_ptr(),
                );
            }
        };
        '_c2rust_label_1: {
            if jperr.errno_code != 0 as ::core::ffi::c_int {
            } else {
                __assert_fail(
                    b"jperr.errno_code != 0\0" as *const u8
                        as *const ::core::ffi::c_char,
                    b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_json_patch.c\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    43 as ::core::ffi::c_uint,
                    __ASSERT_FUNCTION.as_ptr(),
                );
            }
        };
        printf(b"OK\n\0" as *const u8 as *const ::core::ffi::c_char);
        printf(
            b" => json_patch_apply failed as expected: %s at patch idx %zu: %s\n\0" as *const u8
                as *const ::core::ffi::c_char,
            _json_c_strerror(jperr.errno_code),
            jperr.patch_failure_idx,
            jperr.errmsg,
        );
        json_object_put(res);
    } else {
        ret = json_patch_apply(doc, patch, &raw mut res, &raw mut jperr);
        if ret != 0 {
            fprintf(
                stderr,
                b"json_patch_apply() returned '%d'\n\0" as *const u8 as *const ::core::ffi::c_char,
                ret,
            );
            fprintf(
                stderr,
                b"Expected: %s\n\0" as *const u8 as *const ::core::ffi::c_char,
                json_object_get_string(expected),
            );
            fprintf(
                stderr,
                b"Got: %s\n\0" as *const u8 as *const ::core::ffi::c_char,
                if !res.is_null() {
                    json_object_get_string(res)
                } else {
                    b"(null)\0" as *const u8 as *const ::core::ffi::c_char
                },
            );
            fprintf(
                stderr,
                b"json_patch_apply failed: %s at patch idx %zu: %s\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                _json_c_strerror(jperr.errno_code),
                jperr.patch_failure_idx,
                jperr.errmsg,
            );
            fflush(stderr);
            '_c2rust_label_2: {
                __assert_fail(
                    b"0\0" as *const u8 as *const ::core::ffi::c_char,
                    b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_json_patch.c\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    57 as ::core::ffi::c_uint,
                    __ASSERT_FUNCTION.as_ptr(),
                );
            };
        }
        '_c2rust_label_3: {
            if jperr.errno_code == 0 as ::core::ffi::c_int {
            } else {
                __assert_fail(
                    b"jperr.errno_code == 0\0" as *const u8
                        as *const ::core::ffi::c_char,
                    b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_json_patch.c\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    60 as ::core::ffi::c_uint,
                    __ASSERT_FUNCTION.as_ptr(),
                );
            }
        };
        ret = json_object_equal(expected, res);
        if ret == 0 as ::core::ffi::c_int {
            fprintf(
                stderr,
                b"json_object_equal() returned '%d'\n\0" as *const u8 as *const ::core::ffi::c_char,
                ret,
            );
            fprintf(
                stderr,
                b"Expected: %s\n\0" as *const u8 as *const ::core::ffi::c_char,
                json_object_get_string(expected),
            );
            fprintf(
                stderr,
                b"Got: %s\n\0" as *const u8 as *const ::core::ffi::c_char,
                json_object_get_string(res),
            );
            fflush(stderr);
            '_c2rust_label_4: {
                __assert_fail(
                    b"0\0" as *const u8 as *const ::core::ffi::c_char,
                    b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_json_patch.c\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    67 as ::core::ffi::c_uint,
                    __ASSERT_FUNCTION.as_ptr(),
                );
            };
        }
        json_object_put(res);
        res = ::core::ptr::null_mut::<json_object>();
        printf(b"OK\n\0" as *const u8 as *const ::core::ffi::c_char);
    };
}
#[no_mangle]
pub unsafe extern "C" fn test_json_patch_using_file(
    mut testdir: *const ::core::ffi::c_char,
    mut filename: *const ::core::ffi::c_char,
) {
    let mut full_filename: [::core::ffi::c_char; 4096] = [0; 4096];
    snprintf(
        &raw mut full_filename as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 4096]>() as size_t,
        b"%s/%s\0" as *const u8 as *const ::core::ffi::c_char,
        testdir,
        filename,
    );
    let mut ii: size_t = 0;
    printf(
        b"Testing using file %s\n\0" as *const u8 as *const ::core::ffi::c_char,
        filename,
    );
    let mut jo: *mut json_object =
        json_object_from_file(&raw mut full_filename as *mut ::core::ffi::c_char)
            as *mut json_object;
    if jo.is_null() {
        fprintf(
            stderr,
            b"FAIL: unable to open %s: %s\n\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut full_filename as *mut ::core::ffi::c_char,
            _json_c_strerror(*__errno_location()),
        );
        exit(EXIT_FAILURE);
    }
    ii = 0 as size_t;
    while ii < json_object_array_length(jo) {
        let mut jo1: *mut json_object = json_object_array_get_idx(jo, ii);
        test_json_patch_op(jo1);
        ii = ii.wrapping_add(1);
    }
    json_object_put(jo as *mut json_object);
}
unsafe fn main_0(
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut testdir: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    if argc < 2 as ::core::ffi::c_int {
        fprintf(
            stderr,
            b"Usage: %s <testdir>\n  <testdir> is the location of input files\n\0" as *const u8
                as *const ::core::ffi::c_char,
            *argv.offset(0 as ::core::ffi::c_int as isize),
        );
        return EXIT_FAILURE;
    }
    testdir = *argv.offset(1 as ::core::ffi::c_int as isize);
    if strncmp(
        json_c_version(),
        JSON_C_VERSION.as_ptr(),
        ::core::mem::size_of::<[::core::ffi::c_char; 5]>() as size_t,
    ) != 0
    {
        printf(
            b"FAIL: Output from json_c_version(): %s does not match %s\0" as *const u8
                as *const ::core::ffi::c_char,
            json_c_version(),
            JSON_C_VERSION.as_ptr(),
        );
        return EXIT_FAILURE;
    }
    if json_c_version_num() != JSON_C_VERSION_NUM {
        printf(
            b"FAIL: Output from json_c_version_num(): %d does not match %d\0" as *const u8
                as *const ::core::ffi::c_char,
            json_c_version_num(),
            JSON_C_VERSION_NUM,
        );
        return EXIT_FAILURE;
    }
    test_json_patch_using_file(
        testdir,
        b"json_patch_spec_tests.json\0" as *const u8 as *const ::core::ffi::c_char,
    );
    test_json_patch_using_file(
        testdir,
        b"json_patch_tests.json\0" as *const u8 as *const ::core::ffi::c_char,
    );
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
