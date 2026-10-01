#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(extern_types)]
#![feature(label_break_value)]
#![feature(raw_ref_op)]
#[allow(unused_imports)]
use ::libcsv_raw;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn puts(__s: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn memcmp(
        __s1: *const ::core::ffi::c_void,
        __s2: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn exit(__status: ::core::ffi::c_int) -> !;
    fn csv_init(p: *mut csv_parser, options: ::core::ffi::c_uchar) -> ::core::ffi::c_int;
    fn csv_fini(
        p: *mut csv_parser,
        cb1_0: Option<
            unsafe extern "C" fn(*mut ::core::ffi::c_void, size_t, *mut ::core::ffi::c_void) -> (),
        >,
        cb2_0: Option<unsafe extern "C" fn(::core::ffi::c_int, *mut ::core::ffi::c_void) -> ()>,
        data: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn csv_free(p: *mut csv_parser);
    fn csv_parse(
        p: *mut csv_parser,
        s: *const ::core::ffi::c_void,
        len: size_t,
        cb1_0: Option<
            unsafe extern "C" fn(*mut ::core::ffi::c_void, size_t, *mut ::core::ffi::c_void) -> (),
        >,
        cb2_0: Option<unsafe extern "C" fn(::core::ffi::c_int, *mut ::core::ffi::c_void) -> ()>,
        data: *mut ::core::ffi::c_void,
    ) -> size_t;
    fn csv_write(
        dest: *mut ::core::ffi::c_void,
        dest_size: size_t,
        src: *const ::core::ffi::c_void,
        src_size: size_t,
    ) -> size_t;
    fn csv_write2(
        dest: *mut ::core::ffi::c_void,
        dest_size: size_t,
        src: *const ::core::ffi::c_void,
        src_size: size_t,
        quote: ::core::ffi::c_uchar,
    ) -> size_t;
    fn csv_set_delim(p: *mut csv_parser, c: ::core::ffi::c_uchar);
    fn csv_set_quote(p: *mut csv_parser, c: ::core::ffi::c_uchar);
    fn csv_set_space_func(
        p: *mut csv_parser,
        f: Option<unsafe extern "C" fn(::core::ffi::c_uchar) -> ::core::ffi::c_int>,
    );
    fn csv_set_term_func(
        p: *mut csv_parser,
        f: Option<unsafe extern "C" fn(::core::ffi::c_uchar) -> ::core::ffi::c_int>,
    );
}
pub type size_t = usize;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct csv_parser {
    pub pstate: ::core::ffi::c_int,
    pub quoted: ::core::ffi::c_int,
    pub spaces: size_t,
    pub entry_buf: *mut ::core::ffi::c_uchar,
    pub entry_pos: size_t,
    pub entry_size: size_t,
    pub status: ::core::ffi::c_int,
    pub options: ::core::ffi::c_uchar,
    pub quote_char: ::core::ffi::c_uchar,
    pub delim_char: ::core::ffi::c_uchar,
    pub is_space: Option<unsafe extern "C" fn(::core::ffi::c_uchar) -> ::core::ffi::c_int>,
    pub is_term: Option<unsafe extern "C" fn(::core::ffi::c_uchar) -> ::core::ffi::c_int>,
    pub blk_size: size_t,
    pub malloc_func: Option<unsafe extern "C" fn(size_t) -> *mut ::core::ffi::c_void>,
    pub realloc_func:
        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, size_t) -> *mut ::core::ffi::c_void>,
    pub free_func: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct event {
    pub event_type: ::core::ffi::c_int,
    pub retval: ::core::ffi::c_int,
    pub size: size_t,
    pub data: *mut ::core::ffi::c_char,
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const EXIT_FAILURE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const CSV_COMMA: ::core::ffi::c_int = 0x2c as ::core::ffi::c_int;
pub const CSV_QUOTE: ::core::ffi::c_int = 0x22 as ::core::ffi::c_int;
pub const CSV_END: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const CSV_COL: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const CSV_ROW: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const CSV_ERR: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
#[no_mangle]
pub static mut event_ptr: *mut event = ::core::ptr::null::<event>() as *mut event;
#[no_mangle]
pub static mut event_idx: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut row: size_t = 0;
#[no_mangle]
pub static mut col: size_t = 0;
#[no_mangle]
pub unsafe extern "C" fn fail_parser(
    mut test_name: *mut ::core::ffi::c_char,
    mut message: *mut ::core::ffi::c_char,
) {
    fprintf(
        stderr,
        b"Parser test %s failed on event %d: %s\n\0" as *const u8 as *const ::core::ffi::c_char,
        test_name,
        event_idx,
        message,
    );
    exit(EXIT_FAILURE);
}
#[no_mangle]
pub unsafe extern "C" fn fail_writer(
    mut test_name: *mut ::core::ffi::c_char,
    mut message: *mut ::core::ffi::c_char,
) {
    fprintf(
        stderr,
        b"Writer test %s failed: %s\n\0" as *const u8 as *const ::core::ffi::c_char,
        test_name,
        message,
    );
    exit(EXIT_FAILURE);
}
#[no_mangle]
pub unsafe extern "C" fn cb1(
    mut data: *mut ::core::ffi::c_void,
    mut len: size_t,
    mut t: *mut ::core::ffi::c_void,
) {
    let mut test_name: *mut ::core::ffi::c_char = t as *mut ::core::ffi::c_char;
    if (*event_ptr).event_type != CSV_COL {
        fail_parser(
            test_name,
            b"didn't expect a column\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        );
    }
    if (*event_ptr).size != len {
        fail_parser(
            test_name,
            b"actual data length doesn't match expected data length\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
    }
    if (*event_ptr).data.is_null() || data.is_null() {
        if (*event_ptr).data != data as *mut ::core::ffi::c_char {
            fail_parser(
                test_name,
                b"actual data doesn't match expected data\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
        }
    } else if memcmp((*event_ptr).data as *const ::core::ffi::c_void, data, len)
        != 0 as ::core::ffi::c_int
    {
        fail_parser(
            test_name,
            b"actual data doesn't match expected data\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        );
    }
    event_idx += 1;
    event_ptr = event_ptr.offset(1);
    col = col.wrapping_add(1);
}
#[no_mangle]
pub unsafe extern "C" fn cb2(mut c: ::core::ffi::c_int, mut t: *mut ::core::ffi::c_void) {
    let mut test_name: *mut ::core::ffi::c_char = t as *mut ::core::ffi::c_char;
    if (*event_ptr).event_type != CSV_ROW {
        fail_parser(
            test_name,
            b"didn't expect end of row\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        );
    }
    if (*event_ptr).retval != c {
        fail_parser(
            test_name,
            b"row ended with unexpected character\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        );
    }
    event_idx += 1;
    event_ptr = event_ptr.offset(1);
    col = 1 as size_t;
    row = row.wrapping_add(1);
}
#[no_mangle]
pub unsafe extern "C" fn test_parser(
    mut test_name: *mut ::core::ffi::c_char,
    mut options: ::core::ffi::c_uchar,
    mut input: *mut ::core::ffi::c_void,
    mut len: size_t,
    mut expected: *mut event,
    mut delimiter: ::core::ffi::c_char,
    mut quote: ::core::ffi::c_char,
    mut space_func: Option<unsafe extern "C" fn(::core::ffi::c_uchar) -> ::core::ffi::c_int>,
    mut term_func: Option<unsafe extern "C" fn(::core::ffi::c_uchar) -> ::core::ffi::c_int>,
) {
    let mut result: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut retval: size_t = 0;
    let mut p: csv_parser = csv_parser {
        pstate: 0,
        quoted: 0,
        spaces: 0,
        entry_buf: ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
        entry_pos: 0,
        entry_size: 0,
        status: 0,
        options: 0,
        quote_char: 0,
        delim_char: 0,
        is_space: None,
        is_term: None,
        blk_size: 0,
        malloc_func: None,
        realloc_func: None,
        free_func: None,
    };
    let mut size: size_t = 0;
    size = 1 as size_t;
    while size <= len {
        let mut bytes_processed: size_t = 0 as size_t;
        csv_init(&raw mut p, options);
        csv_set_delim(&raw mut p, delimiter as ::core::ffi::c_uchar);
        csv_set_quote(&raw mut p, quote as ::core::ffi::c_uchar);
        csv_set_space_func(&raw mut p, space_func);
        csv_set_term_func(&raw mut p, term_func);
        col = 1 as size_t;
        row = col;
        event_ptr = expected.offset(0 as ::core::ffi::c_int as isize) as *mut event;
        event_idx = 1 as ::core::ffi::c_int;
        loop {
            let mut bytes: size_t = if size < len.wrapping_sub(bytes_processed) {
                size
            } else {
                len.wrapping_sub(bytes_processed)
            };
            retval = csv_parse(
                &raw mut p,
                input.offset(bytes_processed as isize),
                bytes,
                Some(
                    cb1 as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        size_t,
                        *mut ::core::ffi::c_void,
                    ) -> (),
                ),
                Some(
                    cb2 as unsafe extern "C" fn(::core::ffi::c_int, *mut ::core::ffi::c_void) -> (),
                ),
                test_name as *mut ::core::ffi::c_void,
            );
            if retval != bytes {
                if (*event_ptr).event_type != CSV_ERR {
                    fail_parser(
                        test_name,
                        b"unexpected parse error occurred\0" as *const u8
                            as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                    );
                } else {
                    csv_free(&raw mut p);
                    return;
                }
            }
            bytes_processed = (bytes_processed as ::core::ffi::c_ulong)
                .wrapping_add(bytes as ::core::ffi::c_ulong) as size_t
                as size_t;
            if !(bytes_processed < len) {
                break;
            }
        }
        result = csv_fini(
            &raw mut p,
            Some(
                cb1 as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    size_t,
                    *mut ::core::ffi::c_void,
                ) -> (),
            ),
            Some(cb2 as unsafe extern "C" fn(::core::ffi::c_int, *mut ::core::ffi::c_void) -> ()),
            test_name as *mut ::core::ffi::c_void,
        );
        if result != 0 as ::core::ffi::c_int {
            if (*event_ptr).event_type != CSV_ERR {
                fail_parser(
                    test_name,
                    b"unexpected parse error occurred\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                );
            } else {
                csv_free(&raw mut p);
                return;
            }
        }
        csv_free(&raw mut p);
        if (*event_ptr).event_type != CSV_END {
            fail_parser(
                test_name,
                b"unexpected end of input\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
            );
        }
        size = size.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn test_writer(
    mut test_name: *mut ::core::ffi::c_char,
    mut input: *mut ::core::ffi::c_char,
    mut input_len: size_t,
    mut expected: *mut ::core::ffi::c_char,
    mut expected_len: size_t,
) {
    let mut actual_len: size_t = 0;
    let mut temp: *mut ::core::ffi::c_char = malloc(
        input_len
            .wrapping_mul(2 as size_t)
            .wrapping_add(2 as size_t),
    ) as *mut ::core::ffi::c_char;
    if temp.is_null() {
        fprintf(
            stderr,
            b"Failed to allocate memory in test_writer!\n\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        exit(EXIT_FAILURE);
    }
    actual_len = csv_write(
        temp as *mut ::core::ffi::c_void,
        input_len
            .wrapping_mul(2 as size_t)
            .wrapping_add(2 as size_t),
        input as *const ::core::ffi::c_void,
        input_len,
    );
    if actual_len != expected_len {
        fail_writer(
            test_name,
            b"actual length doesn't match expected length\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
    }
    if memcmp(
        temp as *const ::core::ffi::c_void,
        expected as *const ::core::ffi::c_void,
        actual_len,
    ) != 0 as ::core::ffi::c_int
    {
        fail_writer(
            test_name,
            b"actual data doesn't match expected data\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn test_writer2(
    mut test_name: *mut ::core::ffi::c_char,
    mut input: *mut ::core::ffi::c_char,
    mut input_len: size_t,
    mut expected: *mut ::core::ffi::c_char,
    mut expected_len: size_t,
    mut quote: ::core::ffi::c_char,
) {
    let mut actual_len: size_t = 0;
    let mut temp: *mut ::core::ffi::c_char = malloc(
        input_len
            .wrapping_mul(2 as size_t)
            .wrapping_add(2 as size_t),
    ) as *mut ::core::ffi::c_char;
    if temp.is_null() {
        fprintf(
            stderr,
            b"Failed to allocate memory in test_writer!\n\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        exit(EXIT_FAILURE);
    }
    actual_len = csv_write2(
        temp as *mut ::core::ffi::c_void,
        input_len
            .wrapping_mul(2 as size_t)
            .wrapping_add(2 as size_t),
        input as *const ::core::ffi::c_void,
        input_len,
        quote as ::core::ffi::c_uchar,
    );
    if actual_len != expected_len {
        fail_writer(
            test_name,
            b"actual length doesn't match expected length\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
    }
    if memcmp(
        temp as *const ::core::ffi::c_void,
        expected as *const ::core::ffi::c_void,
        actual_len,
    ) != 0 as ::core::ffi::c_int
    {
        fail_writer(
            test_name,
            b"actual data doesn't match expected data\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        );
    }
}
unsafe fn main_0() -> ::core::ffi::c_int {
    let mut test01_data: [::core::ffi::c_char; 25] = ::core::mem::transmute::<
        [u8; 25],
        [::core::ffi::c_char; 25],
    >(*b" 1,2 ,  3         ,4,5\r\n\0");
    let mut test02_data: [::core::ffi::c_char; 7] =
        ::core::mem::transmute::<[u8; 7], [::core::ffi::c_char; 7]>(*b",,,,,\n\0");
    let mut test03_data: [::core::ffi::c_char; 11] =
        ::core::mem::transmute::<[u8; 11], [::core::ffi::c_char; 11]>(*b"\",\",\",\",\"\"\0");
    let mut test04_data: [::core::ffi::c_char; 150] = ::core::mem::transmute::<
        [u8; 150],
        [::core::ffi::c_char; 150],
    >(
        *b"\"I call our world Flatland,\nnot because we call it so,\nbut to make its nature clearer\nto you, my happy readers,\nwho are privileged to live in Space.\"\0",
    );
    let mut test05_data: [::core::ffi::c_char; 43] =
        ::core::mem::transmute::<[u8; 43], [::core::ffi::c_char; 43]>(
            *b"\"\"\"a,b\"\"\",,\" \"\"\"\" \",\"\"\"\"\" \",\" \"\"\"\"\",\"\"\"\"\"\"\0",
        );
    let mut test06_data: [::core::ffi::c_char; 21] =
        ::core::mem::transmute::<[u8; 21], [::core::ffi::c_char; 21]>(*b"\" a, b ,c \", a b  c,\0");
    let mut test07_data: [::core::ffi::c_char; 14] =
        ::core::mem::transmute::<[u8; 14], [::core::ffi::c_char; 14]>(*b"\" \"\" \" \" \"\" \"\0");
    let mut test07b_data: [::core::ffi::c_char; 14] =
        ::core::mem::transmute::<[u8; 14], [::core::ffi::c_char; 14]>(*b"\" \"\" \" \" \"\" \"\0");
    let mut test08_data: [::core::ffi::c_char; 473] = ::core::mem::transmute::<
        [u8; 473],
        [::core::ffi::c_char; 473],
    >(
        *b"\" abc\"                                                                                                                                                                                                                                                                                                                                                                                                                                                                          \", \"123\"\0",
    );
    let mut test09_data: [::core::ffi::c_char; 1] =
        ::core::mem::transmute::<[u8; 1], [::core::ffi::c_char; 1]>(*b"\0");
    let mut test10_data: [::core::ffi::c_char; 3] =
        ::core::mem::transmute::<[u8; 3], [::core::ffi::c_char; 3]>(*b"a\n\0");
    let mut test11_data: [::core::ffi::c_char; 10] =
        ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(*b"1,2 ,3,4\n\0");
    let mut test12_data: [::core::ffi::c_char; 5] =
        ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"\n\n\n\n\0");
    let mut test12b_data: [::core::ffi::c_char; 5] =
        ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"\n\n\n\n\0");
    let mut test13_data: [::core::ffi::c_char; 6] =
        ::core::mem::transmute::<[u8; 6], [::core::ffi::c_char; 6]>(*b"\"abc\"\0");
    let mut test14_data: [::core::ffi::c_char; 21] = ::core::mem::transmute::<
        [u8; 21],
        [::core::ffi::c_char; 21],
    >(*b"1, 2, 3,\n\r\n  \"4\", \r,\0");
    let mut test15_data: [::core::ffi::c_char; 22] = ::core::mem::transmute::<
        [u8; 22],
        [::core::ffi::c_char; 22],
    >(*b"1, 2, 3,\n\r\n  \"4\", \r\"\"\0");
    let mut test16_data: [::core::ffi::c_char; 13] =
        ::core::mem::transmute::<[u8; 13], [::core::ffi::c_char; 13]>(*b"\"1\",\"2\",\" 3 \0");
    let mut test16b_data: [::core::ffi::c_char; 13] =
        ::core::mem::transmute::<[u8; 13], [::core::ffi::c_char; 13]>(*b"\"1\",\"2\",\" 3 \0");
    let mut test17_data: [::core::ffi::c_char; 8] =
        ::core::mem::transmute::<[u8; 8], [::core::ffi::c_char; 8]>(*b" a\0b\0c \0");
    let mut test18_data: [::core::ffi::c_char; 33] = ::core::mem::transmute::<
        [u8; 33],
        [::core::ffi::c_char; 33],
    >(*b"12345678901234567890123456789012\0");
    let mut test19_data: [::core::ffi::c_char; 9] =
        ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"  , \"\" ,\0");
    let mut custom01_data: [::core::ffi::c_char; 43] =
        ::core::mem::transmute::<[u8; 43], [::core::ffi::c_char; 43]>(
            *b"'''a;b''';;' '''' ';''''' ';' ''''';''''''\0",
        );
    let mut test01_results: [event; 7] = [
        event {
            event_type: CSV_COL,
            retval: 0 as ::core::ffi::c_int,
            size: 1 as size_t,
            data: b"1\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        },
        event {
            event_type: CSV_COL,
            retval: 0 as ::core::ffi::c_int,
            size: 1 as size_t,
            data: b"2\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        },
        event {
            event_type: CSV_COL,
            retval: 0 as ::core::ffi::c_int,
            size: 1 as size_t,
            data: b"3\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        },
        event {
            event_type: CSV_COL,
            retval: 0 as ::core::ffi::c_int,
            size: 1 as size_t,
            data: b"4\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        },
        event {
            event_type: CSV_COL,
            retval: 0 as ::core::ffi::c_int,
            size: 1 as size_t,
            data: b"5\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        },
        event {
            event_type: CSV_ROW,
            retval: '\r' as i32,
            size: 1 as size_t,
            data: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        },
        event {
            event_type: CSV_END,
            retval: 0 as ::core::ffi::c_int,
            size: 0 as size_t,
            data: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        },
    ];
    let mut test02_results: [event; 8] = [
        event {
            event_type: CSV_COL,
            retval: 0 as ::core::ffi::c_int,
            size: 0 as size_t,
            data: b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        },
        event {
            event_type: CSV_COL,
            retval: 0 as ::core::ffi::c_int,
            size: 0 as size_t,
            data: b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        },
        event {
            event_type: CSV_COL,
            retval: 0 as ::core::ffi::c_int,
            size: 0 as size_t,
            data: b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        },
        event {
            event_type: CSV_COL,
            retval: 0 as ::core::ffi::c_int,
            size: 0 as size_t,
            data: b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        },
        event {
            event_type: CSV_COL,
            retval: 0 as ::core::ffi::c_int,
            size: 0 as size_t,
            data: b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        },
        event {
            event_type: CSV_COL,
            retval: 0 as ::core::ffi::c_int,
            size: 0 as size_t,
            data: b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        },
        event {
            event_type: CSV_ROW,
            retval: '\n' as i32,
            size: 1 as size_t,
            data: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        },
        event {
            event_type: CSV_END,
            retval: 0 as ::core::ffi::c_int,
            size: 0 as size_t,
            data: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        },
    ];
    let mut test03_results: [event; 5] = [
        event {
            event_type: CSV_COL,
            retval: 0 as ::core::ffi::c_int,
            size: 1 as size_t,
            data: b",\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        },
        event {
            event_type: CSV_COL,
            retval: 0 as ::core::ffi::c_int,
            size: 1 as size_t,
            data: b",\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        },
        event {
            event_type: CSV_COL,
            retval: 0 as ::core::ffi::c_int,
            size: 0 as size_t,
            data: b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        },
        event {
            event_type: CSV_ROW,
            retval: -(1 as ::core::ffi::c_int),
            size: 1 as size_t,
            data: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        },
        event {
            event_type: CSV_END,
            retval: 0 as ::core::ffi::c_int,
            size: 0 as size_t,
            data: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        },
    ];
    let mut test04_results: [event; 3] = [
        event {
            event_type: CSV_COL,
            retval: 0 as ::core::ffi::c_int,
            size: 147 as size_t,
            data: b"I call our world Flatland,\nnot because we call it so,\nbut to make its nature clearer\nto you, my happy readers,\nwho are privileged to live in Space.\0"
                as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        },
        event {
            event_type: CSV_ROW,
            retval: -(1 as ::core::ffi::c_int),
            size: 1 as size_t,
            data: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        },
        event {
            event_type: CSV_END,
            retval: 0 as ::core::ffi::c_int,
            size: 0 as size_t,
            data: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        },
    ];
    let mut test05_results: [event; 8] = [
        event {
            event_type: CSV_COL,
            retval: 0 as ::core::ffi::c_int,
            size: 5 as size_t,
            data: b"\"a,b\"\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        },
        event {
            event_type: CSV_COL,
            retval: 0 as ::core::ffi::c_int,
            size: 0 as size_t,
            data: b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        },
        event {
            event_type: CSV_COL,
            retval: 0 as ::core::ffi::c_int,
            size: 4 as size_t,
            data: b" \"\" \0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        },
        event {
            event_type: CSV_COL,
            retval: 0 as ::core::ffi::c_int,
            size: 3 as size_t,
            data: b"\"\" \0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        },
        event {
            event_type: CSV_COL,
            retval: 0 as ::core::ffi::c_int,
            size: 3 as size_t,
            data: b" \"\"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        },
        event {
            event_type: CSV_COL,
            retval: 0 as ::core::ffi::c_int,
            size: 2 as size_t,
            data: b"\"\"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        },
        event {
            event_type: CSV_ROW,
            retval: -(1 as ::core::ffi::c_int),
            size: 1 as size_t,
            data: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        },
        event {
            event_type: CSV_END,
            retval: 0 as ::core::ffi::c_int,
            size: 0 as size_t,
            data: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        },
    ];
    let mut test06_results: [event; 5] = [
        event {
            event_type: CSV_COL,
            retval: 0 as ::core::ffi::c_int,
            size: 9 as size_t,
            data: b" a, b ,c \0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        },
        event {
            event_type: CSV_COL,
            retval: 0 as ::core::ffi::c_int,
            size: 6 as size_t,
            data: b"a b  c\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        },
        event {
            event_type: CSV_COL,
            retval: 0 as ::core::ffi::c_int,
            size: 0 as size_t,
            data: b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        },
        event {
            event_type: CSV_ROW,
            retval: -(1 as ::core::ffi::c_int),
            size: 1 as size_t,
            data: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        },
        event {
            event_type: CSV_END,
            retval: 0 as ::core::ffi::c_int,
            size: 0 as size_t,
            data: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        },
    ];
    let mut test07_results: [event; 3] = [
        event {
            event_type: CSV_COL,
            retval: 0 as ::core::ffi::c_int,
            size: 9 as size_t,
            data: b" \" \" \" \" \0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        },
        event {
            event_type: CSV_ROW,
            retval: -(1 as ::core::ffi::c_int),
            size: 1 as size_t,
            data: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        },
        event {
            event_type: CSV_END,
            retval: 0 as ::core::ffi::c_int,
            size: 0 as size_t,
            data: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        },
    ];
    let mut test07b_results: [event; 1] = [event {
        event_type: CSV_ERR,
        retval: 0 as ::core::ffi::c_int,
        size: 0 as size_t,
        data: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    }];
    let mut test08_results: [event; 4] = [
        event {
            event_type: CSV_COL,
            retval: 0 as ::core::ffi::c_int,
            size: 463 as size_t,
            data: b" abc\"                                                                                                                                                                                                                                                                                                                                                                                                                                                                          \0"
                as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        },
        event {
            event_type: CSV_COL,
            retval: 0 as ::core::ffi::c_int,
            size: 3 as size_t,
            data: b"123\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        },
        event {
            event_type: CSV_ROW,
            retval: -(1 as ::core::ffi::c_int),
            size: 1 as size_t,
            data: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        },
        event {
            event_type: CSV_END,
            retval: 0 as ::core::ffi::c_int,
            size: 0 as size_t,
            data: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        },
    ];
    let mut test09_results: [event; 1] = [event {
        event_type: CSV_END,
        retval: 0 as ::core::ffi::c_int,
        size: 0 as size_t,
        data: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    }];
    let mut test10_results: [event; 3] = [
        event {
            event_type: CSV_COL,
            retval: 0 as ::core::ffi::c_int,
            size: 1 as size_t,
            data: b"a\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        },
        event {
            event_type: CSV_ROW,
            retval: '\n' as i32,
            size: 1 as size_t,
            data: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        },
        event {
            event_type: CSV_END,
            retval: 0 as ::core::ffi::c_int,
            size: 0 as size_t,
            data: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        },
    ];
    let mut test11_results: [event; 6] = [
        event {
            event_type: CSV_COL,
            retval: 0 as ::core::ffi::c_int,
            size: 1 as size_t,
            data: b"1\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        },
        event {
            event_type: CSV_COL,
            retval: 0 as ::core::ffi::c_int,
            size: 1 as size_t,
            data: b"2\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        },
        event {
            event_type: CSV_COL,
            retval: 0 as ::core::ffi::c_int,
            size: 1 as size_t,
            data: b"3\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        },
        event {
            event_type: CSV_COL,
            retval: 0 as ::core::ffi::c_int,
            size: 1 as size_t,
            data: b"4\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        },
        event {
            event_type: CSV_ROW,
            retval: '\n' as i32,
            size: 1 as size_t,
            data: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        },
        event {
            event_type: CSV_END,
            retval: 0 as ::core::ffi::c_int,
            size: 0 as size_t,
            data: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        },
    ];
    let mut test12_results: [event; 1] = [event {
        event_type: CSV_END,
        retval: 0 as ::core::ffi::c_int,
        size: 0 as size_t,
        data: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    }];
    let mut test12b_results: [event; 5] = [
        event {
            event_type: CSV_ROW,
            retval: '\n' as i32,
            size: 1 as size_t,
            data: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        },
        event {
            event_type: CSV_ROW,
            retval: '\n' as i32,
            size: 1 as size_t,
            data: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        },
        event {
            event_type: CSV_ROW,
            retval: '\n' as i32,
            size: 1 as size_t,
            data: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        },
        event {
            event_type: CSV_ROW,
            retval: '\n' as i32,
            size: 1 as size_t,
            data: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        },
        event {
            event_type: CSV_END,
            retval: 0 as ::core::ffi::c_int,
            size: 0 as size_t,
            data: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        },
    ];
    let mut test13_results: [event; 3] = [
        event {
            event_type: CSV_COL,
            retval: 0 as ::core::ffi::c_int,
            size: 3 as size_t,
            data: b"abc\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        },
        event {
            event_type: CSV_ROW,
            retval: -(1 as ::core::ffi::c_int),
            size: 1 as size_t,
            data: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        },
        event {
            event_type: CSV_END,
            retval: 0 as ::core::ffi::c_int,
            size: 0 as size_t,
            data: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        },
    ];
    let mut test14_results: [event; 12] = [
        event {
            event_type: CSV_COL,
            retval: 0 as ::core::ffi::c_int,
            size: 1 as size_t,
            data: b"1\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        },
        event {
            event_type: CSV_COL,
            retval: 0 as ::core::ffi::c_int,
            size: 1 as size_t,
            data: b"2\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        },
        event {
            event_type: CSV_COL,
            retval: 0 as ::core::ffi::c_int,
            size: 1 as size_t,
            data: b"3\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        },
        event {
            event_type: CSV_COL,
            retval: 0 as ::core::ffi::c_int,
            size: 0 as size_t,
            data: b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        },
        event {
            event_type: CSV_ROW,
            retval: '\n' as i32,
            size: 1 as size_t,
            data: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        },
        event {
            event_type: CSV_COL,
            retval: 0 as ::core::ffi::c_int,
            size: 1 as size_t,
            data: b"4\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        },
        event {
            event_type: CSV_COL,
            retval: 0 as ::core::ffi::c_int,
            size: 0 as size_t,
            data: b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        },
        event {
            event_type: CSV_ROW,
            retval: '\r' as i32,
            size: 1 as size_t,
            data: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        },
        event {
            event_type: CSV_COL,
            retval: 0 as ::core::ffi::c_int,
            size: 0 as size_t,
            data: b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        },
        event {
            event_type: CSV_COL,
            retval: 0 as ::core::ffi::c_int,
            size: 0 as size_t,
            data: b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        },
        event {
            event_type: CSV_ROW,
            retval: -(1 as ::core::ffi::c_int),
            size: 0 as size_t,
            data: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        },
        event {
            event_type: CSV_END,
            retval: 0 as ::core::ffi::c_int,
            size: 0 as size_t,
            data: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        },
    ];
    let mut test15_results: [event; 11] = [
        event {
            event_type: CSV_COL,
            retval: 0 as ::core::ffi::c_int,
            size: 1 as size_t,
            data: b"1\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        },
        event {
            event_type: CSV_COL,
            retval: 0 as ::core::ffi::c_int,
            size: 1 as size_t,
            data: b"2\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        },
        event {
            event_type: CSV_COL,
            retval: 0 as ::core::ffi::c_int,
            size: 1 as size_t,
            data: b"3\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        },
        event {
            event_type: CSV_COL,
            retval: 0 as ::core::ffi::c_int,
            size: 0 as size_t,
            data: b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        },
        event {
            event_type: CSV_ROW,
            retval: '\n' as i32,
            size: 1 as size_t,
            data: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        },
        event {
            event_type: CSV_COL,
            retval: 0 as ::core::ffi::c_int,
            size: 1 as size_t,
            data: b"4\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        },
        event {
            event_type: CSV_COL,
            retval: 0 as ::core::ffi::c_int,
            size: 0 as size_t,
            data: b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        },
        event {
            event_type: CSV_ROW,
            retval: '\r' as i32,
            size: 1 as size_t,
            data: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        },
        event {
            event_type: CSV_COL,
            retval: 0 as ::core::ffi::c_int,
            size: 0 as size_t,
            data: b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        },
        event {
            event_type: CSV_ROW,
            retval: -(1 as ::core::ffi::c_int),
            size: 0 as size_t,
            data: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        },
        event {
            event_type: CSV_END,
            retval: 0 as ::core::ffi::c_int,
            size: 0 as size_t,
            data: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        },
    ];
    let mut test16_results: [event; 5] = [
        event {
            event_type: CSV_COL,
            retval: 0 as ::core::ffi::c_int,
            size: 1 as size_t,
            data: b"1\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        },
        event {
            event_type: CSV_COL,
            retval: 0 as ::core::ffi::c_int,
            size: 1 as size_t,
            data: b"2\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        },
        event {
            event_type: CSV_COL,
            retval: 0 as ::core::ffi::c_int,
            size: 3 as size_t,
            data: b" 3 \0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        },
        event {
            event_type: CSV_ROW,
            retval: -(1 as ::core::ffi::c_int),
            size: 1 as size_t,
            data: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        },
        event {
            event_type: CSV_END,
            retval: 0 as ::core::ffi::c_int,
            size: 0 as size_t,
            data: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        },
    ];
    let mut test16b_results: [event; 3] = [
        event {
            event_type: CSV_COL,
            retval: 0 as ::core::ffi::c_int,
            size: 1 as size_t,
            data: b"1\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        },
        event {
            event_type: CSV_COL,
            retval: 0 as ::core::ffi::c_int,
            size: 1 as size_t,
            data: b"2\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        },
        event {
            event_type: CSV_ERR,
            retval: 0 as ::core::ffi::c_int,
            size: 0 as size_t,
            data: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        },
    ];
    let mut test17_results: [event; 3] = [
        event {
            event_type: CSV_COL,
            retval: 0 as ::core::ffi::c_int,
            size: 5 as size_t,
            data: b"a\0b\0c\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        },
        event {
            event_type: CSV_ROW,
            retval: -(1 as ::core::ffi::c_int),
            size: 1 as size_t,
            data: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        },
        event {
            event_type: CSV_END,
            retval: 0 as ::core::ffi::c_int,
            size: 0 as size_t,
            data: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        },
    ];
    let mut test19_results: [event; 5] = [
        event {
            event_type: CSV_COL,
            retval: 0 as ::core::ffi::c_int,
            size: 0 as size_t,
            data: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        },
        event {
            event_type: CSV_COL,
            retval: 0 as ::core::ffi::c_int,
            size: 0 as size_t,
            data: b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        },
        event {
            event_type: CSV_COL,
            retval: 0 as ::core::ffi::c_int,
            size: 0 as size_t,
            data: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        },
        event {
            event_type: CSV_ROW,
            retval: -(1 as ::core::ffi::c_int),
            size: 1 as size_t,
            data: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        },
        event {
            event_type: CSV_END,
            retval: 0 as ::core::ffi::c_int,
            size: 0 as size_t,
            data: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        },
    ];
    let mut custom01_results: [event; 8] = [
        event {
            event_type: CSV_COL,
            retval: 0 as ::core::ffi::c_int,
            size: 5 as size_t,
            data: b"'a;b'\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        },
        event {
            event_type: CSV_COL,
            retval: 0 as ::core::ffi::c_int,
            size: 0 as size_t,
            data: b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        },
        event {
            event_type: CSV_COL,
            retval: 0 as ::core::ffi::c_int,
            size: 4 as size_t,
            data: b" '' \0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        },
        event {
            event_type: CSV_COL,
            retval: 0 as ::core::ffi::c_int,
            size: 3 as size_t,
            data: b"'' \0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        },
        event {
            event_type: CSV_COL,
            retval: 0 as ::core::ffi::c_int,
            size: 3 as size_t,
            data: b" ''\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        },
        event {
            event_type: CSV_COL,
            retval: 0 as ::core::ffi::c_int,
            size: 2 as size_t,
            data: b"''\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        },
        event {
            event_type: CSV_ROW,
            retval: -(1 as ::core::ffi::c_int),
            size: 1 as size_t,
            data: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        },
        event {
            event_type: CSV_END,
            retval: 0 as ::core::ffi::c_int,
            size: 0 as size_t,
            data: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        },
    ];
    test_parser(
        b"test01\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        0 as ::core::ffi::c_uchar,
        &raw mut test01_data as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        (::core::mem::size_of::<[::core::ffi::c_char; 25]>() as size_t).wrapping_sub(1 as size_t),
        &raw mut test01_results as *mut event,
        CSV_COMMA as ::core::ffi::c_char,
        CSV_QUOTE as ::core::ffi::c_char,
        None,
        None,
    );
    test_parser(
        b"test01\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        1 as ::core::ffi::c_uchar,
        &raw mut test01_data as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        (::core::mem::size_of::<[::core::ffi::c_char; 25]>() as size_t).wrapping_sub(1 as size_t),
        &raw mut test01_results as *mut event,
        CSV_COMMA as ::core::ffi::c_char,
        CSV_QUOTE as ::core::ffi::c_char,
        None,
        None,
    );
    test_parser(
        b"test01\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        (1 as ::core::ffi::c_int | 16 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
        &raw mut test01_data as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        (::core::mem::size_of::<[::core::ffi::c_char; 25]>() as size_t).wrapping_sub(1 as size_t),
        &raw mut test01_results as *mut event,
        CSV_COMMA as ::core::ffi::c_char,
        CSV_QUOTE as ::core::ffi::c_char,
        None,
        None,
    );
    test_parser(
        b"test02\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        0 as ::core::ffi::c_uchar,
        &raw mut test02_data as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        (::core::mem::size_of::<[::core::ffi::c_char; 7]>() as size_t).wrapping_sub(1 as size_t),
        &raw mut test02_results as *mut event,
        CSV_COMMA as ::core::ffi::c_char,
        CSV_QUOTE as ::core::ffi::c_char,
        None,
        None,
    );
    test_parser(
        b"test02\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        1 as ::core::ffi::c_uchar,
        &raw mut test02_data as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        (::core::mem::size_of::<[::core::ffi::c_char; 7]>() as size_t).wrapping_sub(1 as size_t),
        &raw mut test02_results as *mut event,
        CSV_COMMA as ::core::ffi::c_char,
        CSV_QUOTE as ::core::ffi::c_char,
        None,
        None,
    );
    test_parser(
        b"test03\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        0 as ::core::ffi::c_uchar,
        &raw mut test03_data as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        (::core::mem::size_of::<[::core::ffi::c_char; 11]>() as size_t).wrapping_sub(1 as size_t),
        &raw mut test03_results as *mut event,
        CSV_COMMA as ::core::ffi::c_char,
        CSV_QUOTE as ::core::ffi::c_char,
        None,
        None,
    );
    test_parser(
        b"test03\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        1 as ::core::ffi::c_uchar,
        &raw mut test03_data as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        (::core::mem::size_of::<[::core::ffi::c_char; 11]>() as size_t).wrapping_sub(1 as size_t),
        &raw mut test03_results as *mut event,
        CSV_COMMA as ::core::ffi::c_char,
        CSV_QUOTE as ::core::ffi::c_char,
        None,
        None,
    );
    test_parser(
        b"test04\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        0 as ::core::ffi::c_uchar,
        &raw mut test04_data as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        (::core::mem::size_of::<[::core::ffi::c_char; 150]>() as size_t).wrapping_sub(1 as size_t),
        &raw mut test04_results as *mut event,
        CSV_COMMA as ::core::ffi::c_char,
        CSV_QUOTE as ::core::ffi::c_char,
        None,
        None,
    );
    test_parser(
        b"test04\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        1 as ::core::ffi::c_uchar,
        &raw mut test04_data as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        (::core::mem::size_of::<[::core::ffi::c_char; 150]>() as size_t).wrapping_sub(1 as size_t),
        &raw mut test04_results as *mut event,
        CSV_COMMA as ::core::ffi::c_char,
        CSV_QUOTE as ::core::ffi::c_char,
        None,
        None,
    );
    test_parser(
        b"test05\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        0 as ::core::ffi::c_uchar,
        &raw mut test05_data as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        (::core::mem::size_of::<[::core::ffi::c_char; 43]>() as size_t).wrapping_sub(1 as size_t),
        &raw mut test05_results as *mut event,
        CSV_COMMA as ::core::ffi::c_char,
        CSV_QUOTE as ::core::ffi::c_char,
        None,
        None,
    );
    test_parser(
        b"test05\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        1 as ::core::ffi::c_uchar,
        &raw mut test05_data as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        (::core::mem::size_of::<[::core::ffi::c_char; 43]>() as size_t).wrapping_sub(1 as size_t),
        &raw mut test05_results as *mut event,
        CSV_COMMA as ::core::ffi::c_char,
        CSV_QUOTE as ::core::ffi::c_char,
        None,
        None,
    );
    test_parser(
        b"test05\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        (1 as ::core::ffi::c_int | 4 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
        &raw mut test05_data as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        (::core::mem::size_of::<[::core::ffi::c_char; 43]>() as size_t).wrapping_sub(1 as size_t),
        &raw mut test05_results as *mut event,
        CSV_COMMA as ::core::ffi::c_char,
        CSV_QUOTE as ::core::ffi::c_char,
        None,
        None,
    );
    test_parser(
        b"test06\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        0 as ::core::ffi::c_uchar,
        &raw mut test06_data as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        (::core::mem::size_of::<[::core::ffi::c_char; 21]>() as size_t).wrapping_sub(1 as size_t),
        &raw mut test06_results as *mut event,
        CSV_COMMA as ::core::ffi::c_char,
        CSV_QUOTE as ::core::ffi::c_char,
        None,
        None,
    );
    test_parser(
        b"test06\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        1 as ::core::ffi::c_uchar,
        &raw mut test06_data as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        (::core::mem::size_of::<[::core::ffi::c_char; 21]>() as size_t).wrapping_sub(1 as size_t),
        &raw mut test06_results as *mut event,
        CSV_COMMA as ::core::ffi::c_char,
        CSV_QUOTE as ::core::ffi::c_char,
        None,
        None,
    );
    test_parser(
        b"test07\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        0 as ::core::ffi::c_uchar,
        &raw mut test07_data as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        (::core::mem::size_of::<[::core::ffi::c_char; 14]>() as size_t).wrapping_sub(1 as size_t),
        &raw mut test07_results as *mut event,
        CSV_COMMA as ::core::ffi::c_char,
        CSV_QUOTE as ::core::ffi::c_char,
        None,
        None,
    );
    test_parser(
        b"test07b\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        1 as ::core::ffi::c_uchar,
        &raw mut test07b_data as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        (::core::mem::size_of::<[::core::ffi::c_char; 14]>() as size_t).wrapping_sub(1 as size_t),
        &raw mut test07b_results as *mut event,
        CSV_COMMA as ::core::ffi::c_char,
        CSV_QUOTE as ::core::ffi::c_char,
        None,
        None,
    );
    test_parser(
        b"test08\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        0 as ::core::ffi::c_uchar,
        &raw mut test08_data as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        (::core::mem::size_of::<[::core::ffi::c_char; 473]>() as size_t).wrapping_sub(1 as size_t),
        &raw mut test08_results as *mut event,
        CSV_COMMA as ::core::ffi::c_char,
        CSV_QUOTE as ::core::ffi::c_char,
        None,
        None,
    );
    test_parser(
        b"test09\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        0 as ::core::ffi::c_uchar,
        &raw mut test09_data as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        (::core::mem::size_of::<[::core::ffi::c_char; 1]>() as size_t).wrapping_sub(1 as size_t),
        &raw mut test09_results as *mut event,
        CSV_COMMA as ::core::ffi::c_char,
        CSV_QUOTE as ::core::ffi::c_char,
        None,
        None,
    );
    test_parser(
        b"test09\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        16 as ::core::ffi::c_uchar,
        &raw mut test09_data as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        (::core::mem::size_of::<[::core::ffi::c_char; 1]>() as size_t).wrapping_sub(1 as size_t),
        &raw mut test09_results as *mut event,
        CSV_COMMA as ::core::ffi::c_char,
        CSV_QUOTE as ::core::ffi::c_char,
        None,
        None,
    );
    test_parser(
        b"test10\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        0 as ::core::ffi::c_uchar,
        &raw mut test10_data as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        (::core::mem::size_of::<[::core::ffi::c_char; 3]>() as size_t).wrapping_sub(1 as size_t),
        &raw mut test10_results as *mut event,
        CSV_COMMA as ::core::ffi::c_char,
        CSV_QUOTE as ::core::ffi::c_char,
        None,
        None,
    );
    test_parser(
        b"test11\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        0 as ::core::ffi::c_uchar,
        &raw mut test11_data as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        (::core::mem::size_of::<[::core::ffi::c_char; 10]>() as size_t).wrapping_sub(1 as size_t),
        &raw mut test11_results as *mut event,
        CSV_COMMA as ::core::ffi::c_char,
        CSV_QUOTE as ::core::ffi::c_char,
        None,
        None,
    );
    test_parser(
        b"test11\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        16 as ::core::ffi::c_uchar,
        &raw mut test11_data as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        (::core::mem::size_of::<[::core::ffi::c_char; 10]>() as size_t).wrapping_sub(1 as size_t),
        &raw mut test11_results as *mut event,
        CSV_COMMA as ::core::ffi::c_char,
        CSV_QUOTE as ::core::ffi::c_char,
        None,
        None,
    );
    test_parser(
        b"test12\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        0 as ::core::ffi::c_uchar,
        &raw mut test12_data as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        (::core::mem::size_of::<[::core::ffi::c_char; 5]>() as size_t).wrapping_sub(1 as size_t),
        &raw mut test12_results as *mut event,
        CSV_COMMA as ::core::ffi::c_char,
        CSV_QUOTE as ::core::ffi::c_char,
        None,
        None,
    );
    test_parser(
        b"test12\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        16 as ::core::ffi::c_uchar,
        &raw mut test12_data as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        (::core::mem::size_of::<[::core::ffi::c_char; 5]>() as size_t).wrapping_sub(1 as size_t),
        &raw mut test12_results as *mut event,
        CSV_COMMA as ::core::ffi::c_char,
        CSV_QUOTE as ::core::ffi::c_char,
        None,
        None,
    );
    test_parser(
        b"test12b\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        2 as ::core::ffi::c_uchar,
        &raw mut test12b_data as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        (::core::mem::size_of::<[::core::ffi::c_char; 5]>() as size_t).wrapping_sub(1 as size_t),
        &raw mut test12b_results as *mut event,
        CSV_COMMA as ::core::ffi::c_char,
        CSV_QUOTE as ::core::ffi::c_char,
        None,
        None,
    );
    test_parser(
        b"test12b\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        (2 as ::core::ffi::c_int | 16 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
        &raw mut test12b_data as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        (::core::mem::size_of::<[::core::ffi::c_char; 5]>() as size_t).wrapping_sub(1 as size_t),
        &raw mut test12b_results as *mut event,
        CSV_COMMA as ::core::ffi::c_char,
        CSV_QUOTE as ::core::ffi::c_char,
        None,
        None,
    );
    test_parser(
        b"test13\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        0 as ::core::ffi::c_uchar,
        &raw mut test13_data as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        (::core::mem::size_of::<[::core::ffi::c_char; 6]>() as size_t).wrapping_sub(1 as size_t),
        &raw mut test13_results as *mut event,
        CSV_COMMA as ::core::ffi::c_char,
        CSV_QUOTE as ::core::ffi::c_char,
        None,
        None,
    );
    test_parser(
        b"test14\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        0 as ::core::ffi::c_uchar,
        &raw mut test14_data as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        (::core::mem::size_of::<[::core::ffi::c_char; 21]>() as size_t).wrapping_sub(1 as size_t),
        &raw mut test14_results as *mut event,
        CSV_COMMA as ::core::ffi::c_char,
        CSV_QUOTE as ::core::ffi::c_char,
        None,
        None,
    );
    test_parser(
        b"test14\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        1 as ::core::ffi::c_uchar,
        &raw mut test14_data as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        (::core::mem::size_of::<[::core::ffi::c_char; 21]>() as size_t).wrapping_sub(1 as size_t),
        &raw mut test14_results as *mut event,
        CSV_COMMA as ::core::ffi::c_char,
        CSV_QUOTE as ::core::ffi::c_char,
        None,
        None,
    );
    test_parser(
        b"test15\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        0 as ::core::ffi::c_uchar,
        &raw mut test15_data as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        (::core::mem::size_of::<[::core::ffi::c_char; 22]>() as size_t).wrapping_sub(1 as size_t),
        &raw mut test15_results as *mut event,
        CSV_COMMA as ::core::ffi::c_char,
        CSV_QUOTE as ::core::ffi::c_char,
        None,
        None,
    );
    test_parser(
        b"test15\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        1 as ::core::ffi::c_uchar,
        &raw mut test15_data as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        (::core::mem::size_of::<[::core::ffi::c_char; 22]>() as size_t).wrapping_sub(1 as size_t),
        &raw mut test15_results as *mut event,
        CSV_COMMA as ::core::ffi::c_char,
        CSV_QUOTE as ::core::ffi::c_char,
        None,
        None,
    );
    test_parser(
        b"test16\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        0 as ::core::ffi::c_uchar,
        &raw mut test16_data as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        (::core::mem::size_of::<[::core::ffi::c_char; 13]>() as size_t).wrapping_sub(1 as size_t),
        &raw mut test16_results as *mut event,
        CSV_COMMA as ::core::ffi::c_char,
        CSV_QUOTE as ::core::ffi::c_char,
        None,
        None,
    );
    test_parser(
        b"test16\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        1 as ::core::ffi::c_uchar,
        &raw mut test16_data as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        (::core::mem::size_of::<[::core::ffi::c_char; 13]>() as size_t).wrapping_sub(1 as size_t),
        &raw mut test16_results as *mut event,
        CSV_COMMA as ::core::ffi::c_char,
        CSV_QUOTE as ::core::ffi::c_char,
        None,
        None,
    );
    test_parser(
        b"test16b\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        (1 as ::core::ffi::c_int | 4 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
        &raw mut test16b_data as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        (::core::mem::size_of::<[::core::ffi::c_char; 13]>() as size_t).wrapping_sub(1 as size_t),
        &raw mut test16b_results as *mut event,
        CSV_COMMA as ::core::ffi::c_char,
        CSV_QUOTE as ::core::ffi::c_char,
        None,
        None,
    );
    test_parser(
        b"test16\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        0 as ::core::ffi::c_uchar,
        &raw mut test16_data as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        (::core::mem::size_of::<[::core::ffi::c_char; 13]>() as size_t).wrapping_sub(1 as size_t),
        &raw mut test16_results as *mut event,
        CSV_COMMA as ::core::ffi::c_char,
        CSV_QUOTE as ::core::ffi::c_char,
        None,
        None,
    );
    test_parser(
        b"test16\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        1 as ::core::ffi::c_uchar,
        &raw mut test16_data as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        (::core::mem::size_of::<[::core::ffi::c_char; 13]>() as size_t).wrapping_sub(1 as size_t),
        &raw mut test16_results as *mut event,
        CSV_COMMA as ::core::ffi::c_char,
        CSV_QUOTE as ::core::ffi::c_char,
        None,
        None,
    );
    test_parser(
        b"test17\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        0 as ::core::ffi::c_uchar,
        &raw mut test17_data as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        (::core::mem::size_of::<[::core::ffi::c_char; 8]>() as size_t).wrapping_sub(1 as size_t),
        &raw mut test17_results as *mut event,
        CSV_COMMA as ::core::ffi::c_char,
        CSV_QUOTE as ::core::ffi::c_char,
        None,
        None,
    );
    test_parser(
        b"test17\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        1 as ::core::ffi::c_uchar,
        &raw mut test17_data as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        (::core::mem::size_of::<[::core::ffi::c_char; 8]>() as size_t).wrapping_sub(1 as size_t),
        &raw mut test17_results as *mut event,
        CSV_COMMA as ::core::ffi::c_char,
        CSV_QUOTE as ::core::ffi::c_char,
        None,
        None,
    );
    test_parser(
        b"test17\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        (1 as ::core::ffi::c_int | 16 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
        &raw mut test17_data as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        (::core::mem::size_of::<[::core::ffi::c_char; 8]>() as size_t).wrapping_sub(1 as size_t),
        &raw mut test17_results as *mut event,
        CSV_COMMA as ::core::ffi::c_char,
        CSV_QUOTE as ::core::ffi::c_char,
        None,
        None,
    );
    test_parser(
        b"test19\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        16 as ::core::ffi::c_uchar,
        &raw mut test19_data as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        (::core::mem::size_of::<[::core::ffi::c_char; 9]>() as size_t).wrapping_sub(1 as size_t),
        &raw mut test19_results as *mut event,
        CSV_COMMA as ::core::ffi::c_char,
        CSV_QUOTE as ::core::ffi::c_char,
        None,
        None,
    );
    test_parser(
        b"custom01\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        0 as ::core::ffi::c_uchar,
        &raw mut custom01_data as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        (::core::mem::size_of::<[::core::ffi::c_char; 43]>() as size_t).wrapping_sub(1 as size_t),
        &raw mut custom01_results as *mut event,
        ';' as i32 as ::core::ffi::c_char,
        '\'' as i32 as ::core::ffi::c_char,
        None,
        None,
    );
    test_writer(
        b"1\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"abc\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        3 as size_t,
        b"\"abc\"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        5 as size_t,
    );
    test_writer(
        b"2\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"\"\"\"\"\"\"\"\"\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        8 as size_t,
        b"\"\"\"\"\"\"\"\"\"\"\"\"\"\"\"\"\"\"\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        18 as size_t,
    );
    test_writer2(
        b"1\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"abc\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        3 as size_t,
        b"'abc'\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        5 as size_t,
        '\'' as i32 as ::core::ffi::c_char,
    );
    test_writer2(
        b"2\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"''''''''\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        8 as size_t,
        b"''''''''''''''''''\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        18 as size_t,
        '\'' as i32 as ::core::ffi::c_char,
    );
    puts(b"All tests passed\0" as *const u8 as *const ::core::ffi::c_char);
    return 0 as ::core::ffi::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
