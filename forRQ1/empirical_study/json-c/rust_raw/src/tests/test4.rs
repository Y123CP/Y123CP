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
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
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
    fn putc(__c: ::core::ffi::c_int, __stream: *mut FILE) -> ::core::ffi::c_int;
    fn puts(__s: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn abort() -> !;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn json_tokener_parse(str: *const ::core::ffi::c_char) -> *mut json_object;
    fn json_object_put(obj: *mut json_object) -> ::core::ffi::c_int;
    fn json_object_to_json_string(obj: *mut json_object) -> *const ::core::ffi::c_char;
    fn json_object_new_object() -> *mut json_object;
    fn json_object_object_length(obj: *const json_object) -> ::core::ffi::c_int;
    fn json_object_object_add(
        obj: *mut json_object,
        key: *const ::core::ffi::c_char,
        val: *mut json_object,
    ) -> ::core::ffi::c_int;
    fn json_object_new_int(i: int32_t) -> *mut json_object;
    fn json_object_get_string(obj: *mut json_object) -> *const ::core::ffi::c_char;
}
pub type size_t = usize;
pub type __int32_t = i32;
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
pub const __ASSERT_FUNCTION: [::core::ffi::c_char; 28] = unsafe {
    ::core::mem::transmute::<[u8; 28], [::core::ffi::c_char; 28]>(*b"void test_lot_of_adds(void)\0")
};
#[inline]
unsafe extern "C" fn putchar(mut __c: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return putc(__c, stdout);
}
#[no_mangle]
pub unsafe extern "C" fn print_hex(mut s: *const ::core::ffi::c_char) {
    let mut iter: *const ::core::ffi::c_char = s;
    let mut ch: ::core::ffi::c_uchar = 0;
    loop {
        let fresh0 = iter;
        iter = iter.offset(1);
        ch = *fresh0 as ::core::ffi::c_uchar;
        if !(ch as ::core::ffi::c_int != 0 as ::core::ffi::c_int) {
            break;
        }
        if ',' as i32 != ch as ::core::ffi::c_int {
            printf(
                b"%x \0" as *const u8 as *const ::core::ffi::c_char,
                ch as ::core::ffi::c_int,
            );
        } else {
            printf(b",\0" as *const u8 as *const ::core::ffi::c_char);
        }
    }
    putchar('\n' as i32);
}
unsafe extern "C" fn test_lot_of_adds() {
    let mut ii: ::core::ffi::c_int = 0;
    let mut key: [::core::ffi::c_char; 50] = [0; 50];
    let mut jobj: *mut json_object = json_object_new_object() as *mut json_object;
    '_c2rust_label: {
        if !jobj.is_null() {
        } else {
            __assert_fail(
                b"jobj != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test4.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                39 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    ii = 0 as ::core::ffi::c_int;
    while ii < 500 as ::core::ffi::c_int {
        snprintf(
            &raw mut key as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 50]>() as size_t,
            b"k%d\0" as *const u8 as *const ::core::ffi::c_char,
            ii,
        );
        let mut iobj: *mut json_object = json_object_new_int(ii as int32_t) as *mut json_object;
        '_c2rust_label_0: {
            if !iobj.is_null() {
            } else {
                __assert_fail(
                    b"iobj != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                    b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test4.c\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    44 as ::core::ffi::c_uint,
                    __ASSERT_FUNCTION.as_ptr(),
                );
            }
        };
        if json_object_object_add(
            jobj as *mut json_object,
            &raw mut key as *mut ::core::ffi::c_char,
            iobj as *mut json_object,
        ) != 0
        {
            fprintf(
                stderr,
                b"FAILED to add object #%d\n\0" as *const u8 as *const ::core::ffi::c_char,
                ii,
            );
            abort();
        }
        ii += 1;
    }
    printf(
        b"%s\n\0" as *const u8 as *const ::core::ffi::c_char,
        json_object_to_json_string(jobj as *mut json_object),
    );
    '_c2rust_label_1: {
        if json_object_object_length(jobj) == 500 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"json_object_object_length(jobj) == 500\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test4.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                52 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    json_object_put(jobj as *mut json_object);
}
unsafe fn main_0() -> ::core::ffi::c_int {
    let mut input: *const ::core::ffi::c_char =
        b"\"\\ud840\\udd26,\\ud840\\udd27,\\ud800\\udd26,\\ud800\\udd27\"\0" as *const u8
            as *const ::core::ffi::c_char;
    let mut expected: *const ::core::ffi::c_char =
        b"\xF0\xA0\x84\xA6,\xF0\xA0\x84\xA7,\xF0\x90\x84\xA6,\xF0\x90\x84\xA7\0" as *const u8
            as *const ::core::ffi::c_char;
    let mut parse_result: *mut json_object = json_tokener_parse(input);
    let mut unjson: *const ::core::ffi::c_char = json_object_get_string(parse_result);
    printf(
        b"input: %s\n\0" as *const u8 as *const ::core::ffi::c_char,
        input,
    );
    let mut strings_match: ::core::ffi::c_int =
        (strcmp(expected, unjson) == 0) as ::core::ffi::c_int;
    let mut retval: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if strings_match != 0 {
        printf(
            b"JSON parse result is correct: %s\n\0" as *const u8 as *const ::core::ffi::c_char,
            unjson,
        );
        puts(b"PASS\0" as *const u8 as *const ::core::ffi::c_char);
    } else {
        printf(
            b"JSON parse result doesn't match expected string\n\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        printf(b"expected string bytes: \0" as *const u8 as *const ::core::ffi::c_char);
        print_hex(expected);
        printf(b"parsed string bytes:   \0" as *const u8 as *const ::core::ffi::c_char);
        print_hex(unjson);
        puts(b"FAIL\0" as *const u8 as *const ::core::ffi::c_char);
        retval = 1 as ::core::ffi::c_int;
    }
    json_object_put(parse_result);
    test_lot_of_adds();
    return retval;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
