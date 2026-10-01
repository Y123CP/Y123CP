extern "C" {
    pub type json_object;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn printbuf_new() -> *mut printbuf;
    fn printbuf_memappend(
        p: *mut printbuf,
        buf: *const ::core::ffi::c_char,
        size: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn printbuf_free(p: *mut printbuf);
    fn json_object_to_json_string_ext(
        obj: *mut json_object,
        flags: ::core::ffi::c_int,
    ) -> *const ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn _json_c_strerror(errno_in: ::core::ffi::c_int) -> *mut ::core::ffi::c_char;
    fn vsnprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        __arg: ::core::ffi::VaList,
    ) -> ::core::ffi::c_int;
    fn strtod(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_double;
    fn strtoll(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_longlong;
    fn strtoull(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_ulonglong;
    fn open(
        __file: *const ::core::ffi::c_char,
        __oflag: ::core::ffi::c_int,
        ...
    ) -> ::core::ffi::c_int;
    fn close(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn read(__fd: ::core::ffi::c_int, __buf: *mut ::core::ffi::c_void, __nbytes: size_t)
        -> ssize_t;
    fn write(__fd: ::core::ffi::c_int, __buf: *const ::core::ffi::c_void, __n: size_t) -> ssize_t;
    fn json_tokener_error_desc(jerr: json_tokener_error) -> *const ::core::ffi::c_char;
    fn json_tokener_get_error(tok: *mut json_tokener) -> json_tokener_error;
    fn json_tokener_new_ex(depth: ::core::ffi::c_int) -> *mut json_tokener;
    fn json_tokener_free(tok: *mut json_tokener);
    fn json_tokener_parse_ex(
        tok: *mut json_tokener,
        str: *const ::core::ffi::c_char,
        len: ::core::ffi::c_int,
    ) -> *mut json_object;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: ::core::ffi::c_uint,
    pub fp_offset: ::core::ffi::c_uint,
    pub overflow_arg_area: *mut ::core::ffi::c_void,
    pub reg_save_area: *mut ::core::ffi::c_void,
}
pub type __int64_t = i64;
pub type __uint64_t = u64;
pub type __ssize_t = ::core::ffi::c_long;
pub type int64_t = __int64_t;
pub type uint64_t = __uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct printbuf {
    pub buf: *mut ::core::ffi::c_char,
    pub bpos: ::core::ffi::c_int,
    pub size: ::core::ffi::c_int,
}
pub type json_type = ::core::ffi::c_uint;
pub const json_type_string: json_type = 6;
pub const json_type_array: json_type = 5;
pub const json_type_object: json_type = 4;
pub const json_type_int: json_type = 3;
pub const json_type_double: json_type = 2;
pub const json_type_boolean: json_type = 1;
pub const json_type_null: json_type = 0;
pub type size_t = usize;
pub type va_list = __builtin_va_list;
pub type ssize_t = __ssize_t;
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
pub const EINVAL: ::core::ffi::c_int = 22 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const JSON_C_TO_STRING_PLAIN: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const O_RDONLY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const O_WRONLY: ::core::ffi::c_int = 0o1 as ::core::ffi::c_int;
pub const O_CREAT: ::core::ffi::c_int = 0o100 as ::core::ffi::c_int;
pub const O_TRUNC: ::core::ffi::c_int = 0o1000 as ::core::ffi::c_int;
pub const JSON_TOKENER_DEFAULT_DEPTH: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
static mut _last_err: [::core::ffi::c_char; 256] = unsafe {
    ::core::mem::transmute::<
        [u8; 256],
        [::core::ffi::c_char; 256],
    >(
        *b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
    )
};
#[no_mangle]
pub unsafe extern "C" fn json_util_get_last_err() -> *const ::core::ffi::c_char {
    if _last_err[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int == '\0' as i32 {
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    return &raw mut _last_err as *mut ::core::ffi::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn _json_c_set_last_err(
    mut err_fmt: *const ::core::ffi::c_char,
    mut args: ...
) {
    let mut ap: ::core::ffi::VaListImpl;
    ap = args.clone();
    vsnprintf(
        &raw mut _last_err as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>() as size_t,
        err_fmt,
        ap.as_va_list(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn json_object_from_fd(mut fd: ::core::ffi::c_int) -> *mut json_object {
    return json_object_from_fd_ex(fd, -(1 as ::core::ffi::c_int));
}
#[no_mangle]
pub unsafe extern "C" fn json_object_from_fd_ex(
    mut fd: ::core::ffi::c_int,
    mut in_depth: ::core::ffi::c_int,
) -> *mut json_object {
    let mut pb: *mut printbuf = ::core::ptr::null_mut::<printbuf>();
    let mut obj: *mut json_object = ::core::ptr::null_mut::<json_object>();
    let mut buf: [::core::ffi::c_char; 4096] = [0; 4096];
    let mut ret: ssize_t = 0;
    let mut depth: ::core::ffi::c_int = JSON_TOKENER_DEFAULT_DEPTH;
    let mut tok: *mut json_tokener = ::core::ptr::null_mut::<json_tokener>();
    pb = printbuf_new();
    if pb.is_null() {
        _json_c_set_last_err(
            b"json_object_from_fd_ex: printbuf_new failed\n\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<json_object>();
    }
    if in_depth != -(1 as ::core::ffi::c_int) {
        depth = in_depth;
    }
    tok = json_tokener_new_ex(depth) as *mut json_tokener;
    if tok.is_null() {
        _json_c_set_last_err(
            b"json_object_from_fd_ex: unable to allocate json_tokener(depth=%d): %s\n\0"
                as *const u8 as *const ::core::ffi::c_char,
            depth,
            _json_c_strerror(*__errno_location()),
        );
        printbuf_free(pb);
        return ::core::ptr::null_mut::<json_object>();
    }
    loop {
        ret = read(
            fd,
            &raw mut buf as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
            ::core::mem::size_of::<[::core::ffi::c_char; 4096]>() as size_t,
        );
        if !(ret > 0 as ::core::ffi::c_long) {
            break;
        }
        if printbuf_memappend(
            pb,
            &raw mut buf as *mut ::core::ffi::c_char,
            ret as ::core::ffi::c_int,
        ) < 0 as ::core::ffi::c_int
        {
            _json_c_set_last_err(
                b"json_object_from_fd_ex: failed to printbuf_memappend after reading %d+%d bytes: %s\0"
                    as *const u8 as *const ::core::ffi::c_char,
                (*pb).bpos,
                ret as ::core::ffi::c_int,
                _json_c_strerror(*__errno_location()),
            );
            json_tokener_free(tok as *mut json_tokener);
            printbuf_free(pb);
            return ::core::ptr::null_mut::<json_object>();
        }
    }
    if ret < 0 as ::core::ffi::c_long {
        _json_c_set_last_err(
            b"json_object_from_fd_ex: error reading fd %d: %s\n\0" as *const u8
                as *const ::core::ffi::c_char,
            fd,
            _json_c_strerror(*__errno_location()),
        );
        json_tokener_free(tok as *mut json_tokener);
        printbuf_free(pb);
        return ::core::ptr::null_mut::<json_object>();
    }
    obj = json_tokener_parse_ex(tok as *mut json_tokener, (*pb).buf, (*pb).bpos);
    if obj.is_null() {
        _json_c_set_last_err(
            b"json_tokener_parse_ex failed: %s\n\0" as *const u8 as *const ::core::ffi::c_char,
            json_tokener_error_desc(json_tokener_get_error(tok as *mut json_tokener)),
        );
    }
    json_tokener_free(tok as *mut json_tokener);
    printbuf_free(pb);
    return obj;
}
#[no_mangle]
pub unsafe extern "C" fn json_object_from_file(
    mut filename: *const ::core::ffi::c_char,
) -> *mut json_object {
    let mut obj: *mut json_object = ::core::ptr::null_mut::<json_object>();
    let mut fd: ::core::ffi::c_int = 0;
    fd = open(filename, O_RDONLY);
    if fd < 0 as ::core::ffi::c_int {
        _json_c_set_last_err(
            b"json_object_from_file: error opening file %s: %s\n\0" as *const u8
                as *const ::core::ffi::c_char,
            filename,
            _json_c_strerror(*__errno_location()),
        );
        return ::core::ptr::null_mut::<json_object>();
    }
    obj = json_object_from_fd(fd);
    close(fd);
    return obj;
}
#[no_mangle]
pub unsafe extern "C" fn json_object_to_file_ext(
    mut filename: *const ::core::ffi::c_char,
    mut obj: *mut json_object,
    mut flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut fd: ::core::ffi::c_int = 0;
    let mut ret: ::core::ffi::c_int = 0;
    let mut saved_errno: ::core::ffi::c_int = 0;
    if obj.is_null() {
        _json_c_set_last_err(
            b"json_object_to_file_ext: object is null\n\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int);
    }
    fd = open(
        filename,
        O_WRONLY | O_TRUNC | O_CREAT,
        0o644 as ::core::ffi::c_int,
    );
    if fd < 0 as ::core::ffi::c_int {
        _json_c_set_last_err(
            b"json_object_to_file_ext: error opening file %s: %s\n\0" as *const u8
                as *const ::core::ffi::c_char,
            filename,
            _json_c_strerror(*__errno_location()),
        );
        return -(1 as ::core::ffi::c_int);
    }
    ret = _json_object_to_fd(fd, obj, flags, filename);
    saved_errno = *__errno_location();
    close(fd);
    *__errno_location() = saved_errno;
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn json_object_to_fd(
    mut fd: ::core::ffi::c_int,
    mut obj: *mut json_object,
    mut flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if obj.is_null() {
        _json_c_set_last_err(
            b"json_object_to_fd: object is null\n\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int);
    }
    return _json_object_to_fd(fd, obj, flags, ::core::ptr::null::<::core::ffi::c_char>());
}
unsafe extern "C" fn _json_object_to_fd(
    mut fd: ::core::ffi::c_int,
    mut obj: *mut json_object,
    mut flags: ::core::ffi::c_int,
    mut filename: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut ret: ssize_t = 0;
    let mut json_str: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut wpos: size_t = 0;
    let mut wsize: size_t = 0;
    filename = if !filename.is_null() {
        filename
    } else {
        b"(fd)\0" as *const u8 as *const ::core::ffi::c_char
    };
    json_str = json_object_to_json_string_ext(obj, flags);
    if json_str.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    wsize = strlen(json_str);
    wpos = 0 as size_t;
    while wpos < wsize {
        ret = write(
            fd,
            json_str.offset(wpos as isize) as *const ::core::ffi::c_void,
            wsize.wrapping_sub(wpos),
        );
        if ret < 0 as ::core::ffi::c_long {
            _json_c_set_last_err(
                b"json_object_to_fd: error writing file %s: %s\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                filename,
                _json_c_strerror(*__errno_location()),
            );
            return -(1 as ::core::ffi::c_int);
        }
        wpos = (wpos as ::core::ffi::c_ulong).wrapping_add(ret as size_t as ::core::ffi::c_ulong)
            as size_t as size_t;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn json_object_to_file(
    mut filename: *const ::core::ffi::c_char,
    mut obj: *mut json_object,
) -> ::core::ffi::c_int {
    return json_object_to_file_ext(filename, obj, JSON_C_TO_STRING_PLAIN);
}
#[no_mangle]
pub unsafe extern "C" fn json_parse_double(
    mut buf: *const ::core::ffi::c_char,
    mut retval: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let mut end: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    *retval = strtod(buf, &raw mut end);
    return if end == buf as *mut ::core::ffi::c_char {
        1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn json_parse_int64(
    mut buf: *const ::core::ffi::c_char,
    mut retval: *mut int64_t,
) -> ::core::ffi::c_int {
    let mut end: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut val: int64_t = 0;
    *__errno_location() = 0 as ::core::ffi::c_int;
    val = strtoll(buf, &raw mut end, 10 as ::core::ffi::c_int) as int64_t;
    if end != buf as *mut ::core::ffi::c_char {
        *retval = val;
    }
    if val == 0 as int64_t && *__errno_location() != 0 as ::core::ffi::c_int
        || end == buf as *mut ::core::ffi::c_char
    {
        *__errno_location() = EINVAL;
        return 1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn json_parse_uint64(
    mut buf: *const ::core::ffi::c_char,
    mut retval: *mut uint64_t,
) -> ::core::ffi::c_int {
    let mut end: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut val: uint64_t = 0;
    *__errno_location() = 0 as ::core::ffi::c_int;
    while *buf as ::core::ffi::c_int == ' ' as i32 {
        buf = buf.offset(1);
    }
    if *buf as ::core::ffi::c_int == '-' as i32 {
        return 1 as ::core::ffi::c_int;
    }
    val = strtoull(buf, &raw mut end, 10 as ::core::ffi::c_int) as uint64_t;
    if end != buf as *mut ::core::ffi::c_char {
        *retval = val;
    }
    if val == 0 as uint64_t && *__errno_location() != 0 as ::core::ffi::c_int
        || end == buf as *mut ::core::ffi::c_char
    {
        *__errno_location() = EINVAL;
        return 1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
static mut json_type_name: [*const ::core::ffi::c_char; 7] = [
    b"null\0" as *const u8 as *const ::core::ffi::c_char,
    b"boolean\0" as *const u8 as *const ::core::ffi::c_char,
    b"double\0" as *const u8 as *const ::core::ffi::c_char,
    b"int\0" as *const u8 as *const ::core::ffi::c_char,
    b"object\0" as *const u8 as *const ::core::ffi::c_char,
    b"array\0" as *const u8 as *const ::core::ffi::c_char,
    b"string\0" as *const u8 as *const ::core::ffi::c_char,
];
#[no_mangle]
pub unsafe extern "C" fn json_type_to_name(mut o_type: json_type) -> *const ::core::ffi::c_char {
    let mut o_type_int: ::core::ffi::c_int = o_type as ::core::ffi::c_int;
    if o_type_int < 0 as ::core::ffi::c_int
        || o_type_int
            >= (::core::mem::size_of::<[*const ::core::ffi::c_char; 7]>() as usize)
                .wrapping_div(::core::mem::size_of::<*const ::core::ffi::c_char>() as usize)
                as ::core::ffi::c_int
    {
        _json_c_set_last_err(
            b"json_type_to_name: type %d is out of range [0,%u]\n\0" as *const u8
                as *const ::core::ffi::c_char,
            o_type as ::core::ffi::c_uint,
            (::core::mem::size_of::<[*const ::core::ffi::c_char; 7]>() as usize)
                .wrapping_div(::core::mem::size_of::<*const ::core::ffi::c_char>() as usize)
                as ::core::ffi::c_uint,
        );
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    return json_type_name[o_type as usize];
}
