extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type json_object;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn open(
        __file: *const ::core::ffi::c_char,
        __oflag: ::core::ffi::c_int,
        ...
    ) -> ::core::ffi::c_int;
    static mut optarg: *mut ::core::ffi::c_char;
    static mut optind: ::core::ffi::c_int;
    fn getopt(
        ___argc: ::core::ffi::c_int,
        ___argv: *const *mut ::core::ffi::c_char,
        __shortopts: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fflush(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn printf(__format: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn strtol(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_long;
    fn exit(__status: ::core::ffi::c_int) -> !;
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strerror(__errnum: ::core::ffi::c_int) -> *mut ::core::ffi::c_char;
    fn read(__fd: ::core::ffi::c_int, __buf: *mut ::core::ffi::c_void, __nbytes: size_t)
        -> ssize_t;
    fn json_object_put(obj: *mut json_object) -> ::core::ffi::c_int;
    fn json_object_to_json_string_ext(
        obj: *mut json_object,
        flags: ::core::ffi::c_int,
    ) -> *const ::core::ffi::c_char;
    fn json_tokener_error_desc(jerr: json_tokener_error) -> *const ::core::ffi::c_char;
    fn json_tokener_get_error(tok: *mut json_tokener) -> json_tokener_error;
    fn json_tokener_new_ex(depth: ::core::ffi::c_int) -> *mut json_tokener;
    fn json_tokener_free(tok: *mut json_tokener);
    fn json_tokener_set_flags(tok: *mut json_tokener, flags: ::core::ffi::c_int);
    fn json_tokener_parse_ex(
        tok: *mut json_tokener,
        str: *const ::core::ffi::c_char,
        len: ::core::ffi::c_int,
    ) -> *mut json_object;
    fn getrusage(__who: __rusage_who_t, __usage: *mut rusage) -> ::core::ffi::c_int;
}
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type __time_t = ::core::ffi::c_long;
pub type __suseconds_t = ::core::ffi::c_long;
pub type __ssize_t = ::core::ffi::c_long;
pub type __syscall_slong_t = ::core::ffi::c_long;
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
pub type ssize_t = __ssize_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct printbuf {
    pub buf: *mut ::core::ffi::c_char,
    pub bpos: ::core::ffi::c_int,
    pub size: ::core::ffi::c_int,
}
pub type json_tokener_error = ::core::ffi::c_uint;
pub const json_tokener_error_size: json_tokener_error = 16;
pub const json_tokener_error_memory: json_tokener_error = 15;
pub const json_tokener_error_parse_utf8_string: json_tokener_error = 14;
pub const json_tokener_error_parse_comment: json_tokener_error = 13;
pub const json_tokener_error_parse_string: json_tokener_error = 12;
pub const json_tokener_error_parse_object_value_sep: json_tokener_error = 11;
pub const json_tokener_error_parse_object_key_sep: json_tokener_error = 10;
pub const json_tokener_error_parse_object_key_name: json_tokener_error = 9;
pub const json_tokener_error_parse_array: json_tokener_error = 8;
pub const json_tokener_error_parse_number: json_tokener_error = 7;
pub const json_tokener_error_parse_boolean: json_tokener_error = 6;
pub const json_tokener_error_parse_null: json_tokener_error = 5;
pub const json_tokener_error_parse_unexpected: json_tokener_error = 4;
pub const json_tokener_error_parse_eof: json_tokener_error = 3;
pub const json_tokener_error_depth: json_tokener_error = 2;
pub const json_tokener_continue: json_tokener_error = 1;
pub const json_tokener_success: json_tokener_error = 0;
pub type json_tokener_state = ::core::ffi::c_uint;
pub const json_tokener_state_inf: json_tokener_state = 26;
pub const json_tokener_state_object_field_start_after_sep: json_tokener_state = 25;
pub const json_tokener_state_array_after_sep: json_tokener_state = 24;
pub const json_tokener_state_object_sep: json_tokener_state = 23;
pub const json_tokener_state_object_value_add: json_tokener_state = 22;
pub const json_tokener_state_object_value: json_tokener_state = 21;
pub const json_tokener_state_object_field_end: json_tokener_state = 20;
pub const json_tokener_state_object_field: json_tokener_state = 19;
pub const json_tokener_state_object_field_start: json_tokener_state = 18;
pub const json_tokener_state_array_sep: json_tokener_state = 17;
pub const json_tokener_state_array_add: json_tokener_state = 16;
pub const json_tokener_state_array: json_tokener_state = 15;
pub const json_tokener_state_number: json_tokener_state = 14;
pub const json_tokener_state_boolean: json_tokener_state = 13;
pub const json_tokener_state_escape_unicode_need_u: json_tokener_state = 12;
pub const json_tokener_state_escape_unicode_need_escape: json_tokener_state = 11;
pub const json_tokener_state_escape_unicode: json_tokener_state = 10;
pub const json_tokener_state_string_escape: json_tokener_state = 9;
pub const json_tokener_state_string: json_tokener_state = 8;
pub const json_tokener_state_comment_end: json_tokener_state = 7;
pub const json_tokener_state_comment_eol: json_tokener_state = 6;
pub const json_tokener_state_comment: json_tokener_state = 5;
pub const json_tokener_state_comment_start: json_tokener_state = 4;
pub const json_tokener_state_null: json_tokener_state = 3;
pub const json_tokener_state_finish: json_tokener_state = 2;
pub const json_tokener_state_start: json_tokener_state = 1;
pub const json_tokener_state_eatws: json_tokener_state = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct json_tokener_srec {
    pub state: json_tokener_state,
    pub saved_state: json_tokener_state,
    pub obj: *mut json_object,
    pub current: *mut json_object,
    pub obj_field_name: *mut ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct json_tokener {
    pub str_0: *mut ::core::ffi::c_char,
    pub pb: *mut printbuf,
    pub max_depth: ::core::ffi::c_int,
    pub depth: ::core::ffi::c_int,
    pub is_double: ::core::ffi::c_int,
    pub st_pos: ::core::ffi::c_int,
    pub char_offset: ::core::ffi::c_int,
    pub err: json_tokener_error,
    pub ucs_char: ::core::ffi::c_uint,
    pub high_surrogate: ::core::ffi::c_uint,
    pub quote_char: ::core::ffi::c_char,
    pub stack: *mut json_tokener_srec,
    pub flags: ::core::ffi::c_int,
}
pub type __rusage_who = ::core::ffi::c_int;
pub const RUSAGE_THREAD: __rusage_who = 1;
pub const RUSAGE_CHILDREN: __rusage_who = -1;
pub const RUSAGE_SELF: __rusage_who = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rusage {
    pub ru_utime: timeval,
    pub ru_stime: timeval,
    pub c2rust_unnamed: C2RustUnnamed_12,
    pub c2rust_unnamed_0: C2RustUnnamed_11,
    pub c2rust_unnamed_1: C2RustUnnamed_10,
    pub c2rust_unnamed_2: C2RustUnnamed_9,
    pub c2rust_unnamed_3: C2RustUnnamed_8,
    pub c2rust_unnamed_4: C2RustUnnamed_7,
    pub c2rust_unnamed_5: C2RustUnnamed_6,
    pub c2rust_unnamed_6: C2RustUnnamed_5,
    pub c2rust_unnamed_7: C2RustUnnamed_4,
    pub c2rust_unnamed_8: C2RustUnnamed_3,
    pub c2rust_unnamed_9: C2RustUnnamed_2,
    pub c2rust_unnamed_10: C2RustUnnamed_1,
    pub c2rust_unnamed_11: C2RustUnnamed_0,
    pub c2rust_unnamed_12: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub ru_nivcsw: ::core::ffi::c_long,
    pub __ru_nivcsw_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub ru_nvcsw: ::core::ffi::c_long,
    pub __ru_nvcsw_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub ru_nsignals: ::core::ffi::c_long,
    pub __ru_nsignals_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_2 {
    pub ru_msgrcv: ::core::ffi::c_long,
    pub __ru_msgrcv_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
    pub ru_msgsnd: ::core::ffi::c_long,
    pub __ru_msgsnd_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_4 {
    pub ru_oublock: ::core::ffi::c_long,
    pub __ru_oublock_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_5 {
    pub ru_inblock: ::core::ffi::c_long,
    pub __ru_inblock_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_6 {
    pub ru_nswap: ::core::ffi::c_long,
    pub __ru_nswap_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_7 {
    pub ru_majflt: ::core::ffi::c_long,
    pub __ru_majflt_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_8 {
    pub ru_minflt: ::core::ffi::c_long,
    pub __ru_minflt_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_9 {
    pub ru_isrss: ::core::ffi::c_long,
    pub __ru_isrss_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_10 {
    pub ru_idrss: ::core::ffi::c_long,
    pub __ru_idrss_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_11 {
    pub ru_ixrss: ::core::ffi::c_long,
    pub __ru_ixrss_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_12 {
    pub ru_maxrss: ::core::ffi::c_long,
    pub __ru_maxrss_word: __syscall_slong_t,
}
pub type __rusage_who_t = __rusage_who;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL_0: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const O_RDONLY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const EXIT_FAILURE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const EXIT_SUCCESS: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const ::core::ffi::c_char) -> ::core::ffi::c_int {
    return strtol(
        __nptr,
        NULL as *mut *mut ::core::ffi::c_char,
        10 as ::core::ffi::c_int,
    ) as ::core::ffi::c_int;
}
pub const JSON_C_TO_STRING_SPACED: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int;
pub const JSON_C_TO_STRING_PRETTY: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int;
pub const JSON_C_TO_STRING_COLOR: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int;
pub const JSON_TOKENER_DEFAULT_DEPTH: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
pub const JSON_TOKENER_STRICT: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const JSON_TOKENER_ALLOW_TRAILING_CHARS: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
static mut formatted_output: ::core::ffi::c_int = JSON_C_TO_STRING_SPACED;
static mut show_output: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
static mut strict_mode: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut color: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut fname: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
unsafe extern "C" fn showmem() {
    let mut rusage: rusage = rusage {
        ru_utime: timeval {
            tv_sec: 0,
            tv_usec: 0,
        },
        ru_stime: timeval {
            tv_sec: 0,
            tv_usec: 0,
        },
        c2rust_unnamed: C2RustUnnamed_12 { ru_maxrss: 0 },
        c2rust_unnamed_0: C2RustUnnamed_11 { ru_ixrss: 0 },
        c2rust_unnamed_1: C2RustUnnamed_10 { ru_idrss: 0 },
        c2rust_unnamed_2: C2RustUnnamed_9 { ru_isrss: 0 },
        c2rust_unnamed_3: C2RustUnnamed_8 { ru_minflt: 0 },
        c2rust_unnamed_4: C2RustUnnamed_7 { ru_majflt: 0 },
        c2rust_unnamed_5: C2RustUnnamed_6 { ru_nswap: 0 },
        c2rust_unnamed_6: C2RustUnnamed_5 { ru_inblock: 0 },
        c2rust_unnamed_7: C2RustUnnamed_4 { ru_oublock: 0 },
        c2rust_unnamed_8: C2RustUnnamed_3 { ru_msgsnd: 0 },
        c2rust_unnamed_9: C2RustUnnamed_2 { ru_msgrcv: 0 },
        c2rust_unnamed_10: C2RustUnnamed_1 { ru_nsignals: 0 },
        c2rust_unnamed_11: C2RustUnnamed_0 { ru_nvcsw: 0 },
        c2rust_unnamed_12: C2RustUnnamed { ru_nivcsw: 0 },
    };
    memset(
        &raw mut rusage as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<rusage>() as size_t,
    );
    getrusage(RUSAGE_SELF, &raw mut rusage);
    fprintf(
        stderr,
        b"maxrss: %ld KB\n\0" as *const u8 as *const ::core::ffi::c_char,
        rusage.c2rust_unnamed.ru_maxrss,
    );
}
unsafe extern "C" fn parseit(
    mut fd: ::core::ffi::c_int,
    mut callback: Option<unsafe extern "C" fn(*mut json_object) -> ::core::ffi::c_int>,
) -> ::core::ffi::c_int {
    let mut obj: *mut json_object = ::core::ptr::null_mut::<json_object>();
    let mut buf: [::core::ffi::c_char; 32768] = [0; 32768];
    let mut ret: ssize_t = 0;
    let mut depth: ::core::ffi::c_int = JSON_TOKENER_DEFAULT_DEPTH;
    let mut tok: *mut json_tokener = ::core::ptr::null_mut::<json_tokener>();
    tok = json_tokener_new_ex(depth) as *mut json_tokener;
    if tok.is_null() {
        fprintf(
            stderr,
            b"unable to allocate json_tokener: %s\n\0" as *const u8 as *const ::core::ffi::c_char,
            strerror(*__errno_location()),
        );
        return 1 as ::core::ffi::c_int;
    }
    json_tokener_set_flags(
        tok as *mut json_tokener,
        JSON_TOKENER_STRICT | JSON_TOKENER_ALLOW_TRAILING_CHARS,
    );
    let mut total_read: size_t = 0 as size_t;
    loop {
        ret = read(
            fd,
            &raw mut buf as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
            ::core::mem::size_of::<[::core::ffi::c_char; 32768]>() as size_t,
        );
        if !(ret > 0 as ::core::ffi::c_long) {
            break;
        }
        let mut retu: size_t = ret as size_t;
        total_read = (total_read as ::core::ffi::c_ulong).wrapping_add(retu as ::core::ffi::c_ulong)
            as size_t as size_t;
        let mut start_pos: size_t = 0 as size_t;
        while start_pos != retu {
            obj = json_tokener_parse_ex(
                tok as *mut json_tokener,
                (&raw mut buf as *mut ::core::ffi::c_char).offset(start_pos as isize)
                    as *mut ::core::ffi::c_char,
                retu.wrapping_sub(start_pos) as ::core::ffi::c_int,
            );
            let mut jerr: json_tokener_error = json_tokener_get_error(tok as *mut json_tokener);
            let mut parse_end: size_t = (*tok).char_offset as size_t;
            if obj.is_null()
                && jerr as ::core::ffi::c_uint
                    != json_tokener_continue as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                let mut aterr: *const ::core::ffi::c_char = if start_pos.wrapping_add(parse_end)
                    < ::core::mem::size_of::<[::core::ffi::c_char; 32768]>() as ::core::ffi::c_int
                        as size_t
                {
                    (&raw mut buf as *mut ::core::ffi::c_char)
                        .offset(start_pos.wrapping_add(parse_end) as isize)
                        as *mut ::core::ffi::c_char
                        as *const ::core::ffi::c_char
                } else {
                    b"\0" as *const u8 as *const ::core::ffi::c_char
                };
                fflush(stdout);
                let mut fail_offset: size_t = total_read
                    .wrapping_sub(retu)
                    .wrapping_add(start_pos)
                    .wrapping_add(parse_end);
                fprintf(
                    stderr,
                    b"Failed at offset %lu: %s %c\n\0" as *const u8 as *const ::core::ffi::c_char,
                    fail_offset as ::core::ffi::c_ulong,
                    json_tokener_error_desc(jerr),
                    *aterr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
                );
                json_tokener_free(tok as *mut json_tokener);
                return 1 as ::core::ffi::c_int;
            }
            if !obj.is_null() {
                let mut cb_ret: ::core::ffi::c_int =
                    callback.expect("non-null function pointer")(obj);
                json_object_put(obj);
                if cb_ret != 0 as ::core::ffi::c_int {
                    json_tokener_free(tok as *mut json_tokener);
                    return 1 as ::core::ffi::c_int;
                }
            }
            start_pos = (start_pos as ::core::ffi::c_ulong)
                .wrapping_add((*tok).char_offset as ::core::ffi::c_ulong)
                as size_t as size_t;
        }
    }
    if ret < 0 as ::core::ffi::c_long {
        fprintf(
            stderr,
            b"error reading fd %d: %s\n\0" as *const u8 as *const ::core::ffi::c_char,
            fd,
            strerror(*__errno_location()),
        );
    }
    json_tokener_free(tok as *mut json_tokener);
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn showobj(mut new_obj: *mut json_object) -> ::core::ffi::c_int {
    if new_obj.is_null() {
        fprintf(
            stderr,
            b"%s: Failed to parse\n\0" as *const u8 as *const ::core::ffi::c_char,
            fname,
        );
        return 1 as ::core::ffi::c_int;
    }
    fprintf(
        stderr,
        b"Successfully parsed object from %s\n\0" as *const u8 as *const ::core::ffi::c_char,
        fname,
    );
    if show_output != 0 {
        let mut output: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
        output = json_object_to_json_string_ext(new_obj, formatted_output | color);
        printf(b"%s\n\0" as *const u8 as *const ::core::ffi::c_char, output);
    }
    showmem();
    return 0 as ::core::ffi::c_int;
}
#[cold]
unsafe extern "C" fn usage(
    mut argv0: *const ::core::ffi::c_char,
    mut exitval: ::core::ffi::c_int,
    mut errmsg: *const ::core::ffi::c_char,
) -> ! {
    let mut fp: *mut FILE = stdout;
    if exitval != 0 as ::core::ffi::c_int {
        fp = stderr;
    }
    if !errmsg.is_null() {
        fprintf(
            fp,
            b"ERROR: %s\n\n\0" as *const u8 as *const ::core::ffi::c_char,
            errmsg,
        );
    }
    fprintf(
        fp,
        b"Usage: %s [-f|-F <arg>] [-n] [-s]\n\0" as *const u8 as *const ::core::ffi::c_char,
        argv0,
    );
    fprintf(
        fp,
        b"  -f - Format the output to stdout with JSON_C_TO_STRING_PRETTY (default is JSON_C_TO_STRING_SPACED)\n\0"
            as *const u8 as *const ::core::ffi::c_char,
    );
    fprintf(
        fp,
        b"  -F - Format the output to stdout with <arg>, e.g. 0 for JSON_C_TO_STRING_PLAIN\n\0"
            as *const u8 as *const ::core::ffi::c_char,
    );
    fprintf(
        fp,
        b"  -n - No output\n\0" as *const u8 as *const ::core::ffi::c_char,
    );
    fprintf(
        fp,
        b"  -c - color\n\0" as *const u8 as *const ::core::ffi::c_char,
    );
    fprintf(
        fp,
        b"  -s - Parse in strict mode, flags:\n\0" as *const u8 as *const ::core::ffi::c_char,
    );
    fprintf(
        fp,
        b"       JSON_TOKENER_STRICT|JSON_TOKENER_ALLOW_TRAILING_CHARS\n\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    fprintf(
        fp,
        b" Diagnostic information will be emitted to stderr\n\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    fprintf(
        fp,
        b"\nWARNING WARNING WARNING\n\0" as *const u8 as *const ::core::ffi::c_char,
    );
    fprintf(
        fp,
        b"This is a prototype, it may change or be removed at any time!\n\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    exit(exitval);
}
unsafe fn main_0(
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut opt: ::core::ffi::c_int = 0;
    loop {
        opt = getopt(
            argc,
            argv,
            b"fF:hnsc\0" as *const u8 as *const ::core::ffi::c_char,
        );
        if !(opt != -(1 as ::core::ffi::c_int)) {
            break;
        }
        match opt {
            102 => {
                formatted_output = JSON_C_TO_STRING_PRETTY;
            }
            70 => {
                formatted_output = atoi(optarg);
            }
            110 => {
                show_output = 0 as ::core::ffi::c_int;
            }
            115 => {
                strict_mode = 1 as ::core::ffi::c_int;
            }
            99 => {
                color = JSON_C_TO_STRING_COLOR;
            }
            104 => {
                usage(
                    *argv.offset(0 as ::core::ffi::c_int as isize),
                    0 as ::core::ffi::c_int,
                    ::core::ptr::null::<::core::ffi::c_char>(),
                );
            }
            _ => {
                usage(
                    *argv.offset(0 as ::core::ffi::c_int as isize),
                    EXIT_FAILURE,
                    b"Unknown arguments\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
        }
    }
    if optind >= argc {
        usage(
            *argv.offset(0 as ::core::ffi::c_int as isize),
            EXIT_FAILURE,
            b"Expected argument after options\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    fname = *argv.offset(optind as isize);
    let mut fd: ::core::ffi::c_int = open(
        *argv.offset(optind as isize),
        O_RDONLY,
        0 as ::core::ffi::c_int,
    );
    showmem();
    if parseit(
        fd,
        Some(showobj as unsafe extern "C" fn(*mut json_object) -> ::core::ffi::c_int),
    ) != 0 as ::core::ffi::c_int
    {
        exit(EXIT_FAILURE);
    }
    showmem();
    exit(EXIT_SUCCESS);
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
