extern "C" {
    pub type __locale_data;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn strtod(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_double;
    fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn strncmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn strdup(__s: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn strchr(__s: *const ::core::ffi::c_char, __c: ::core::ffi::c_int)
        -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn strncasecmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn mc_debug(msg: *const ::core::ffi::c_char, ...);
    fn printbuf_new() -> *mut printbuf;
    fn printbuf_memappend(
        p: *mut printbuf,
        buf: *const ::core::ffi::c_char,
        size: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn printbuf_reset(p: *mut printbuf);
    fn printbuf_free(p: *mut printbuf);
    fn json_object_get(obj: *mut json_object) -> *mut json_object;
    fn json_object_put(obj: *mut json_object) -> ::core::ffi::c_int;
    fn json_object_new_object() -> *mut json_object;
    fn json_object_object_add(
        obj: *mut json_object,
        key: *const ::core::ffi::c_char,
        val: *mut json_object,
    ) -> ::core::ffi::c_int;
    fn json_object_new_array() -> *mut json_object;
    fn json_object_array_add(obj: *mut json_object, val: *mut json_object) -> ::core::ffi::c_int;
    fn json_object_array_shrink(
        jso: *mut json_object,
        empty_slots: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn json_object_new_boolean(b: json_bool) -> *mut json_object;
    fn json_object_new_int64(i: int64_t) -> *mut json_object;
    fn json_object_new_uint64(i: uint64_t) -> *mut json_object;
    fn json_object_new_double(d: ::core::ffi::c_double) -> *mut json_object;
    fn json_object_new_double_s(
        d: ::core::ffi::c_double,
        ds: *const ::core::ffi::c_char,
    ) -> *mut json_object;
    fn json_object_new_string_len(
        s: *const ::core::ffi::c_char,
        len: ::core::ffi::c_int,
    ) -> *mut json_object;
    fn newlocale(
        __category_mask: ::core::ffi::c_int,
        __locale: *const ::core::ffi::c_char,
        __base: locale_t,
    ) -> locale_t;
    fn duplocale(__dataset: locale_t) -> locale_t;
    fn freelocale(__dataset: locale_t);
    fn uselocale(__dataset: locale_t) -> locale_t;
    fn json_parse_int64(
        buf: *const ::core::ffi::c_char,
        retval: *mut int64_t,
    ) -> ::core::ffi::c_int;
    fn json_parse_uint64(
        buf: *const ::core::ffi::c_char,
        retval: *mut uint64_t,
    ) -> ::core::ffi::c_int;
}
pub type __uint32_t = u32;
pub type __int64_t = i64;
pub type __uint64_t = u64;
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __locale_struct {
    pub __locales: [*mut __locale_data; 13],
    pub __ctype_b: *const ::core::ffi::c_ushort,
    pub __ctype_tolower: *const ::core::ffi::c_int,
    pub __ctype_toupper: *const ::core::ffi::c_int,
    pub __names: [*const ::core::ffi::c_char; 13],
}
pub type __locale_t = *mut __locale_struct;
pub type locale_t = __locale_t;
pub type int64_t = __int64_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct printbuf {
    pub buf: *mut ::core::ffi::c_char,
    pub bpos: ::core::ffi::c_int,
    pub size: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct json_object {
    pub o_type: json_type,
    pub _ref_count: uint32_t,
    pub _to_json_string: Option<json_object_to_json_string_fn>,
    pub _pb: *mut printbuf,
    pub _user_delete: Option<json_object_delete_fn>,
    pub _userdata: *mut ::core::ffi::c_void,
}
pub type json_object_delete_fn =
    unsafe extern "C" fn(*mut json_object, *mut ::core::ffi::c_void) -> ();
pub type json_object_to_json_string_fn = unsafe extern "C" fn(
    *mut json_object,
    *mut printbuf,
    ::core::ffi::c_int,
    ::core::ffi::c_int,
) -> ::core::ffi::c_int;
pub type json_type = ::core::ffi::c_uint;
pub const json_type_string: json_type = 6;
pub const json_type_array: json_type = 5;
pub const json_type_object: json_type = 4;
pub const json_type_int: json_type = 3;
pub const json_type_double: json_type = 2;
pub const json_type_boolean: json_type = 1;
pub const json_type_null: json_type = 0;
pub type json_bool = ::core::ffi::c_int;
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
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const INT32_MAX: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
pub const INT64_MAX: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
pub const JSON_TOKENER_DEFAULT_DEPTH: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
pub const JSON_TOKENER_STRICT: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const JSON_TOKENER_ALLOW_TRAILING_CHARS: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const JSON_TOKENER_VALIDATE_UTF8: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn is_ws_char(mut c: ::core::ffi::c_char) -> ::core::ffi::c_int {
    return (c as ::core::ffi::c_int == ' ' as i32
        || c as ::core::ffi::c_int == '\t' as i32
        || c as ::core::ffi::c_int == '\n' as i32
        || c as ::core::ffi::c_int == '\r' as i32) as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn is_hex_char(mut c: ::core::ffi::c_char) -> ::core::ffi::c_int {
    return (c as ::core::ffi::c_int >= '0' as i32 && c as ::core::ffi::c_int <= '9' as i32
        || c as ::core::ffi::c_int >= 'A' as i32 && c as ::core::ffi::c_int <= 'F' as i32
        || c as ::core::ffi::c_int >= 'a' as i32 && c as ::core::ffi::c_int <= 'f' as i32)
        as ::core::ffi::c_int;
}
static mut json_null_str: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"null\0") };
static mut json_null_str_len: ::core::ffi::c_int = 0;
static mut json_inf_str: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"Infinity\0") };
static mut json_inf_str_invert: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"iNFINITY\0") };
static mut json_inf_str_len: ::core::ffi::c_uint = 0;
static mut json_nan_str: [::core::ffi::c_char; 4] =
    unsafe { ::core::mem::transmute::<[u8; 4], [::core::ffi::c_char; 4]>(*b"NaN\0") };
static mut json_nan_str_len: ::core::ffi::c_int = 0;
static mut json_true_str: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"true\0") };
static mut json_true_str_len: ::core::ffi::c_int = 0;
static mut json_false_str: [::core::ffi::c_char; 6] =
    unsafe { ::core::mem::transmute::<[u8; 6], [::core::ffi::c_char; 6]>(*b"false\0") };
static mut json_false_str_len: ::core::ffi::c_int = 0;
static mut json_tokener_errors: [*const ::core::ffi::c_char; 17] = [
    b"success\0" as *const u8 as *const ::core::ffi::c_char,
    b"continue\0" as *const u8 as *const ::core::ffi::c_char,
    b"nesting too deep\0" as *const u8 as *const ::core::ffi::c_char,
    b"unexpected end of data\0" as *const u8 as *const ::core::ffi::c_char,
    b"unexpected character\0" as *const u8 as *const ::core::ffi::c_char,
    b"null expected\0" as *const u8 as *const ::core::ffi::c_char,
    b"boolean expected\0" as *const u8 as *const ::core::ffi::c_char,
    b"number expected\0" as *const u8 as *const ::core::ffi::c_char,
    b"array value separator ',' expected\0" as *const u8 as *const ::core::ffi::c_char,
    b"quoted object property name expected\0" as *const u8 as *const ::core::ffi::c_char,
    b"object property name separator ':' expected\0" as *const u8 as *const ::core::ffi::c_char,
    b"object value separator ',' expected\0" as *const u8 as *const ::core::ffi::c_char,
    b"invalid string sequence\0" as *const u8 as *const ::core::ffi::c_char,
    b"expected comment\0" as *const u8 as *const ::core::ffi::c_char,
    b"invalid utf-8 string\0" as *const u8 as *const ::core::ffi::c_char,
    b"buffer size overflow\0" as *const u8 as *const ::core::ffi::c_char,
    b"out of memory\0" as *const u8 as *const ::core::ffi::c_char,
];
#[no_mangle]
pub unsafe extern "C" fn json_tokener_error_desc(
    mut jerr: json_tokener_error,
) -> *const ::core::ffi::c_char {
    let mut jerr_int: ::core::ffi::c_int = jerr as ::core::ffi::c_int;
    if jerr_int < 0 as ::core::ffi::c_int
        || jerr_int
            >= (::core::mem::size_of::<[*const ::core::ffi::c_char; 17]>() as usize)
                .wrapping_div(::core::mem::size_of::<*const ::core::ffi::c_char>() as usize)
                as ::core::ffi::c_int
    {
        return b"Unknown error, invalid json_tokener_error value passed to json_tokener_error_desc()\0"
            as *const u8 as *const ::core::ffi::c_char;
    }
    return json_tokener_errors[jerr as usize];
}
#[no_mangle]
pub unsafe extern "C" fn json_tokener_get_error(mut tok: *mut json_tokener) -> json_tokener_error {
    return (*tok).err;
}
static mut utf8_replacement_char: [::core::ffi::c_uchar; 3] = [
    0xef as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xbf as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xbd as ::core::ffi::c_int as ::core::ffi::c_uchar,
];
#[no_mangle]
pub unsafe extern "C" fn json_tokener_new_ex(mut depth: ::core::ffi::c_int) -> *mut json_tokener {
    let mut tok: *mut json_tokener = ::core::ptr::null_mut::<json_tokener>();
    tok = calloc(
        1 as size_t,
        ::core::mem::size_of::<json_tokener>() as size_t,
    ) as *mut json_tokener;
    if tok.is_null() {
        return ::core::ptr::null_mut::<json_tokener>();
    }
    (*tok).stack = calloc(
        depth as size_t,
        ::core::mem::size_of::<json_tokener_srec>() as size_t,
    ) as *mut json_tokener_srec;
    if (*tok).stack.is_null() {
        free(tok as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<json_tokener>();
    }
    (*tok).pb = printbuf_new();
    if (*tok).pb.is_null() {
        free((*tok).stack as *mut ::core::ffi::c_void);
        free(tok as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<json_tokener>();
    }
    (*tok).max_depth = depth;
    json_tokener_reset(tok);
    return tok;
}
#[no_mangle]
pub unsafe extern "C" fn json_tokener_new() -> *mut json_tokener {
    return json_tokener_new_ex(JSON_TOKENER_DEFAULT_DEPTH);
}
#[no_mangle]
pub unsafe extern "C" fn json_tokener_free(mut tok: *mut json_tokener) {
    json_tokener_reset(tok);
    if !(*tok).pb.is_null() {
        printbuf_free((*tok).pb);
    }
    free((*tok).stack as *mut ::core::ffi::c_void);
    free(tok as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn json_tokener_reset_level(
    mut tok: *mut json_tokener,
    mut depth: ::core::ffi::c_int,
) {
    (*(*tok).stack.offset(depth as isize)).state = json_tokener_state_eatws;
    (*(*tok).stack.offset(depth as isize)).saved_state = json_tokener_state_start;
    json_object_put((*(*tok).stack.offset(depth as isize)).current);
    let ref mut fresh0 = (*(*tok).stack.offset(depth as isize)).current;
    *fresh0 = ::core::ptr::null_mut::<json_object>();
    free((*(*tok).stack.offset(depth as isize)).obj_field_name as *mut ::core::ffi::c_void);
    let ref mut fresh1 = (*(*tok).stack.offset(depth as isize)).obj_field_name;
    *fresh1 = ::core::ptr::null_mut::<::core::ffi::c_char>();
}
#[no_mangle]
pub unsafe extern "C" fn json_tokener_reset(mut tok: *mut json_tokener) {
    let mut i: ::core::ffi::c_int = 0;
    if tok.is_null() {
        return;
    }
    i = (*tok).depth;
    while i >= 0 as ::core::ffi::c_int {
        json_tokener_reset_level(tok, i);
        i -= 1;
    }
    (*tok).depth = 0 as ::core::ffi::c_int;
    (*tok).err = json_tokener_success;
}
#[no_mangle]
pub unsafe extern "C" fn json_tokener_parse(
    mut str: *const ::core::ffi::c_char,
) -> *mut json_object {
    let mut jerr_ignored: json_tokener_error = json_tokener_success;
    let mut obj: *mut json_object = ::core::ptr::null_mut::<json_object>();
    obj = json_tokener_parse_verbose(str, &raw mut jerr_ignored);
    return obj;
}
#[no_mangle]
pub unsafe extern "C" fn json_tokener_parse_verbose(
    mut str: *const ::core::ffi::c_char,
    mut error: *mut json_tokener_error,
) -> *mut json_object {
    let mut tok: *mut json_tokener = ::core::ptr::null_mut::<json_tokener>();
    let mut obj: *mut json_object = ::core::ptr::null_mut::<json_object>();
    tok = json_tokener_new();
    if tok.is_null() {
        return ::core::ptr::null_mut::<json_object>();
    }
    obj = json_tokener_parse_ex(tok, str, -(1 as ::core::ffi::c_int));
    *error = (*tok).err;
    if (*tok).err as ::core::ffi::c_uint
        != json_tokener_success as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        if !obj.is_null() {
            json_object_put(obj);
        }
        obj = ::core::ptr::null_mut::<json_object>();
    }
    json_tokener_free(tok);
    return obj;
}
#[no_mangle]
pub unsafe extern "C" fn json_tokener_parse_ex(
    mut tok: *mut json_tokener,
    mut str: *const ::core::ffi::c_char,
    mut len: ::core::ffi::c_int,
) -> *mut json_object {
    let mut case_start_0: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut case_start: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut case_start_3: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut case_start_1: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut size: ::core::ffi::c_int = 0;
    let mut size_nan: ::core::ffi::c_int = 0;
    let mut size1: ::core::ffi::c_int = 0;
    let mut size2: ::core::ffi::c_int = 0;
    let mut current_block: u64;
    let mut obj: *mut json_object = ::core::ptr::null_mut::<json_object>();
    let mut c: ::core::ffi::c_char = '\u{1}' as i32 as ::core::ffi::c_char;
    let mut nBytes: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    let mut nBytesp: *mut ::core::ffi::c_uint = &raw mut nBytes;
    let mut oldlocale: locale_t = uselocale(::core::ptr::null_mut::<__locale_struct>());
    let mut newloc: locale_t = ::core::ptr::null_mut::<__locale_struct>();
    (*tok).char_offset = 0 as ::core::ffi::c_int;
    (*tok).err = json_tokener_success;
    if len < -(1 as ::core::ffi::c_int)
        || len == -(1 as ::core::ffi::c_int) && strlen(str) > INT32_MAX as size_t
    {
        (*tok).err = json_tokener_error_size;
        return ::core::ptr::null_mut::<json_object>();
    }
    let mut duploc: locale_t = duplocale(oldlocale);
    newloc = newlocale(
        LC_NUMERIC_MASK,
        b"C\0" as *const u8 as *const ::core::ffi::c_char,
        duploc,
    );
    if newloc.is_null() {
        freelocale(duploc);
        return ::core::ptr::null_mut::<json_object>();
    }
    uselocale(newloc);
    's_57: while if (*tok).char_offset == len {
        if (*tok).depth == 0 as ::core::ffi::c_int
            && (*(*tok).stack.offset((*tok).depth as isize)).state as ::core::ffi::c_uint
                == json_tokener_state_eatws as ::core::ffi::c_int as ::core::ffi::c_uint
            && (*(*tok).stack.offset((*tok).depth as isize)).saved_state as ::core::ffi::c_uint
                == json_tokener_state_finish as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            (*tok).err = json_tokener_success;
            0 as ::core::ffi::c_int
        } else {
            (*tok).err = json_tokener_continue;
            0 as ::core::ffi::c_int
        }
    } else if (*tok).flags & JSON_TOKENER_VALIDATE_UTF8 != 0
        && json_tokener_validate_utf8(*str, nBytesp) == 0
    {
        (*tok).err = json_tokener_error_parse_utf8_string;
        0 as ::core::ffi::c_int
    } else {
        c = *str;
        1 as ::core::ffi::c_int
    } != 0
    {
        loop {
            match (*(*tok).stack.offset((*tok).depth as isize)).state as ::core::ffi::c_uint {
                0 => {
                    while is_ws_char(c) != 0 {
                        str = str.offset(1);
                        (*tok).char_offset += 1;
                        if c == 0
                            || (if (*tok).char_offset == len {
                                (if (*tok).depth == 0 as ::core::ffi::c_int
                                    && (*(*tok).stack.offset((*tok).depth as isize)).state
                                        as ::core::ffi::c_uint
                                        == json_tokener_state_eatws as ::core::ffi::c_int
                                            as ::core::ffi::c_uint
                                    && (*(*tok).stack.offset((*tok).depth as isize)).saved_state
                                        as ::core::ffi::c_uint
                                        == json_tokener_state_finish as ::core::ffi::c_int
                                            as ::core::ffi::c_uint
                                {
                                    (*tok).err = json_tokener_success;
                                    0 as ::core::ffi::c_int
                                } else {
                                    (*tok).err = json_tokener_continue;
                                    0 as ::core::ffi::c_int
                                })
                            } else {
                                (if (*tok).flags & JSON_TOKENER_VALIDATE_UTF8 != 0
                                    && json_tokener_validate_utf8(*str, nBytesp) == 0
                                {
                                    (*tok).err = json_tokener_error_parse_utf8_string;
                                    0 as ::core::ffi::c_int
                                } else {
                                    c = *str;
                                    1 as ::core::ffi::c_int
                                })
                            }) == 0
                        {
                            break 's_57;
                        }
                    }
                    if c as ::core::ffi::c_int == '/' as i32
                        && (*tok).flags & JSON_TOKENER_STRICT == 0
                    {
                        printbuf_reset((*tok).pb);
                        if printbuf_memappend((*tok).pb, &raw mut c, 1 as ::core::ffi::c_int)
                            < 0 as ::core::ffi::c_int
                        {
                            current_block = 5689001924483802034;
                            break;
                        } else {
                            current_block = 26972500619410423;
                            break;
                        }
                    } else {
                        (*(*tok).stack.offset((*tok).depth as isize)).state =
                            (*(*tok).stack.offset((*tok).depth as isize)).saved_state;
                    }
                }
                1 => match c as ::core::ffi::c_int {
                    123 => {
                        (*(*tok).stack.offset((*tok).depth as isize)).state =
                            json_tokener_state_eatws;
                        (*(*tok).stack.offset((*tok).depth as isize)).saved_state =
                            json_tokener_state_object_field_start;
                        let ref mut fresh2 = (*(*tok).stack.offset((*tok).depth as isize)).current;
                        *fresh2 = json_object_new_object();
                        if (*(*tok).stack.offset((*tok).depth as isize))
                            .current
                            .is_null()
                        {
                            current_block = 11057878835866523405;
                            break;
                        } else {
                            current_block = 18421592892663940172;
                            break;
                        }
                    }
                    91 => {
                        (*(*tok).stack.offset((*tok).depth as isize)).state =
                            json_tokener_state_eatws;
                        (*(*tok).stack.offset((*tok).depth as isize)).saved_state =
                            json_tokener_state_array;
                        let ref mut fresh3 = (*(*tok).stack.offset((*tok).depth as isize)).current;
                        *fresh3 = json_object_new_array();
                        if (*(*tok).stack.offset((*tok).depth as isize))
                            .current
                            .is_null()
                        {
                            current_block = 17281240262373992796;
                            break;
                        } else {
                            current_block = 18421592892663940172;
                            break;
                        }
                    }
                    73 | 105 => {
                        (*(*tok).stack.offset((*tok).depth as isize)).state =
                            json_tokener_state_inf;
                        printbuf_reset((*tok).pb);
                        (*tok).st_pos = 0 as ::core::ffi::c_int;
                    }
                    78 | 110 => {
                        (*(*tok).stack.offset((*tok).depth as isize)).state =
                            json_tokener_state_null;
                        printbuf_reset((*tok).pb);
                        (*tok).st_pos = 0 as ::core::ffi::c_int;
                    }
                    39 => {
                        if (*tok).flags & JSON_TOKENER_STRICT != 0 {
                            current_block = 11743904203796629665;
                            break;
                        } else {
                            current_block = 964414645882542267;
                            break;
                        }
                    }
                    34 => {
                        current_block = 964414645882542267;
                        break;
                    }
                    84 | 116 | 70 | 102 => {
                        (*(*tok).stack.offset((*tok).depth as isize)).state =
                            json_tokener_state_boolean;
                        printbuf_reset((*tok).pb);
                        (*tok).st_pos = 0 as ::core::ffi::c_int;
                    }
                    48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 | 45 => {
                        (*(*tok).stack.offset((*tok).depth as isize)).state =
                            json_tokener_state_number;
                        printbuf_reset((*tok).pb);
                        (*tok).is_double = 0 as ::core::ffi::c_int;
                    }
                    _ => {
                        (*tok).err = json_tokener_error_parse_unexpected;
                        break 's_57;
                    }
                },
                2 => {
                    if (*tok).depth == 0 as ::core::ffi::c_int {
                        break 's_57;
                    }
                    obj = json_object_get((*(*tok).stack.offset((*tok).depth as isize)).current);
                    json_tokener_reset_level(tok, (*tok).depth);
                    (*tok).depth -= 1;
                }
                26 => {
                    let mut is_negative: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    while (*tok).st_pos < json_inf_str_len as ::core::ffi::c_int {
                        let mut inf_char: ::core::ffi::c_char = *str;
                        if inf_char as ::core::ffi::c_int
                            != json_inf_str[(*tok).st_pos as usize] as ::core::ffi::c_int
                            && ((*tok).flags & JSON_TOKENER_STRICT != 0
                                || inf_char as ::core::ffi::c_int
                                    != json_inf_str_invert[(*tok).st_pos as usize]
                                        as ::core::ffi::c_int)
                        {
                            (*tok).err = json_tokener_error_parse_unexpected;
                            break 's_57;
                        } else {
                            (*tok).st_pos += 1;
                            str = str.offset(1);
                            (*tok).char_offset += 1;
                            if if (*tok).char_offset == len {
                                if (*tok).depth == 0 as ::core::ffi::c_int
                                    && (*(*tok).stack.offset((*tok).depth as isize)).state
                                        as ::core::ffi::c_uint
                                        == json_tokener_state_eatws as ::core::ffi::c_int
                                            as ::core::ffi::c_uint
                                    && (*(*tok).stack.offset((*tok).depth as isize)).saved_state
                                        as ::core::ffi::c_uint
                                        == json_tokener_state_finish as ::core::ffi::c_int
                                            as ::core::ffi::c_uint
                                {
                                    (*tok).err = json_tokener_success;
                                    0 as ::core::ffi::c_int
                                } else {
                                    (*tok).err = json_tokener_continue;
                                    0 as ::core::ffi::c_int
                                }
                            } else if (*tok).flags & JSON_TOKENER_VALIDATE_UTF8 != 0
                                && json_tokener_validate_utf8(*str, nBytesp) == 0
                            {
                                (*tok).err = json_tokener_error_parse_utf8_string;
                                0 as ::core::ffi::c_int
                            } else {
                                c = *str;
                                1 as ::core::ffi::c_int
                            } == 0
                            {
                                break 's_57;
                            }
                        }
                    }
                    if (*(*tok).pb).bpos > 0 as ::core::ffi::c_int
                        && *(*(*tok).pb).buf as ::core::ffi::c_int == '-' as i32
                    {
                        is_negative = 1 as ::core::ffi::c_int;
                    }
                    let ref mut fresh4 = (*(*tok).stack.offset((*tok).depth as isize)).current;
                    *fresh4 = json_object_new_double(
                        (if is_negative != 0 {
                            -::core::f32::INFINITY
                        } else {
                            ::core::f32::INFINITY
                        }) as ::core::ffi::c_double,
                    );
                    if (*(*tok).stack.offset((*tok).depth as isize))
                        .current
                        .is_null()
                    {
                        (*tok).err = json_tokener_error_memory;
                        break 's_57;
                    } else {
                        (*(*tok).stack.offset((*tok).depth as isize)).saved_state =
                            json_tokener_state_finish;
                        (*(*tok).stack.offset((*tok).depth as isize)).state =
                            json_tokener_state_eatws;
                    }
                }
                3 => {
                    size = 0;
                    size_nan = 0;
                    if printbuf_memappend((*tok).pb, &raw mut c, 1 as ::core::ffi::c_int)
                        < 0 as ::core::ffi::c_int
                    {
                        (*tok).err = json_tokener_error_memory;
                        break 's_57;
                    } else {
                        size = if ((*tok).st_pos + 1 as ::core::ffi::c_int) < json_null_str_len {
                            (*tok).st_pos + 1 as ::core::ffi::c_int
                        } else {
                            json_null_str_len
                        };
                        size_nan = if ((*tok).st_pos + 1 as ::core::ffi::c_int) < json_nan_str_len {
                            (*tok).st_pos + 1 as ::core::ffi::c_int
                        } else {
                            json_nan_str_len
                        };
                        if (*tok).flags & JSON_TOKENER_STRICT == 0
                            && strncasecmp(
                                &raw const json_null_str as *const ::core::ffi::c_char,
                                (*(*tok).pb).buf,
                                size as size_t,
                            ) == 0 as ::core::ffi::c_int
                            || strncmp(
                                &raw const json_null_str as *const ::core::ffi::c_char,
                                (*(*tok).pb).buf,
                                size as size_t,
                            ) == 0 as ::core::ffi::c_int
                        {
                            if !((*tok).st_pos == json_null_str_len) {
                                current_block = 16313536926714486912;
                                break;
                            }
                            let ref mut fresh5 =
                                (*(*tok).stack.offset((*tok).depth as isize)).current;
                            *fresh5 = ::core::ptr::null_mut::<json_object>();
                            (*(*tok).stack.offset((*tok).depth as isize)).saved_state =
                                json_tokener_state_finish;
                            (*(*tok).stack.offset((*tok).depth as isize)).state =
                                json_tokener_state_eatws;
                        } else if (*tok).flags & JSON_TOKENER_STRICT == 0
                            && strncasecmp(
                                &raw const json_nan_str as *const ::core::ffi::c_char,
                                (*(*tok).pb).buf,
                                size_nan as size_t,
                            ) == 0 as ::core::ffi::c_int
                            || strncmp(
                                &raw const json_nan_str as *const ::core::ffi::c_char,
                                (*(*tok).pb).buf,
                                size_nan as size_t,
                            ) == 0 as ::core::ffi::c_int
                        {
                            if !((*tok).st_pos == json_nan_str_len) {
                                current_block = 16313536926714486912;
                                break;
                            }
                            let ref mut fresh6 =
                                (*(*tok).stack.offset((*tok).depth as isize)).current;
                            *fresh6 =
                                json_object_new_double(::core::f32::NAN as ::core::ffi::c_double);
                            if (*(*tok).stack.offset((*tok).depth as isize))
                                .current
                                .is_null()
                            {
                                (*tok).err = json_tokener_error_memory;
                                break 's_57;
                            } else {
                                (*(*tok).stack.offset((*tok).depth as isize)).saved_state =
                                    json_tokener_state_finish;
                                (*(*tok).stack.offset((*tok).depth as isize)).state =
                                    json_tokener_state_eatws;
                            }
                        } else {
                            (*tok).err = json_tokener_error_parse_null;
                            break 's_57;
                        }
                    }
                }
                4 => {
                    if c as ::core::ffi::c_int == '*' as i32 {
                        current_block = 7416055328783156979;
                        break;
                    } else {
                        current_block = 6406431739208918833;
                        break;
                    }
                }
                5 => {
                    case_start = str;
                    current_block = 11865390570819897086;
                    break;
                }
                6 => {
                    case_start_0 = str;
                    current_block = 14648249180243006330;
                    break;
                }
                7 => {
                    if printbuf_memappend((*tok).pb, &raw mut c, 1 as ::core::ffi::c_int)
                        < 0 as ::core::ffi::c_int
                    {
                        current_block = 7370318721998929769;
                        break;
                    } else {
                        current_block = 14579489411542934868;
                        break;
                    }
                }
                8 => {
                    case_start_1 = str;
                    current_block = 9255187738567101705;
                    break;
                }
                9 => match c as ::core::ffi::c_int {
                    34 | 92 | 47 => {
                        current_block = 9240481512215375588;
                        break;
                    }
                    98 | 110 | 114 | 116 | 102 => {
                        current_block = 10799024050558338665;
                        break;
                    }
                    117 => {
                        current_block = 16011976826082558567;
                        break;
                    }
                    _ => {
                        current_block = 13221763560517913443;
                        break;
                    }
                },
                10 => {
                    current_block = 6950536787749910113;
                    break;
                }
                11 => {
                    if c == 0 || c as ::core::ffi::c_int != '\\' as i32 {
                        if printbuf_memappend(
                            (*tok).pb,
                            &raw mut utf8_replacement_char as *mut ::core::ffi::c_uchar
                                as *mut ::core::ffi::c_char,
                            3 as ::core::ffi::c_int,
                        ) < 0 as ::core::ffi::c_int
                        {
                            (*tok).err = json_tokener_error_memory;
                            break 's_57;
                        } else {
                            (*tok).high_surrogate = 0 as ::core::ffi::c_uint;
                            (*tok).ucs_char = 0 as ::core::ffi::c_uint;
                            (*tok).st_pos = 0 as ::core::ffi::c_int;
                            (*(*tok).stack.offset((*tok).depth as isize)).state =
                                (*(*tok).stack.offset((*tok).depth as isize)).saved_state;
                        }
                    } else {
                        (*(*tok).stack.offset((*tok).depth as isize)).state =
                            json_tokener_state_escape_unicode_need_u;
                        current_block = 18421592892663940172;
                        break;
                    }
                }
                12 => {
                    if c == 0 || c as ::core::ffi::c_int != 'u' as i32 {
                        if printbuf_memappend(
                            (*tok).pb,
                            &raw mut utf8_replacement_char as *mut ::core::ffi::c_uchar
                                as *mut ::core::ffi::c_char,
                            3 as ::core::ffi::c_int,
                        ) < 0 as ::core::ffi::c_int
                        {
                            (*tok).err = json_tokener_error_memory;
                            break 's_57;
                        } else {
                            (*tok).high_surrogate = 0 as ::core::ffi::c_uint;
                            (*tok).ucs_char = 0 as ::core::ffi::c_uint;
                            (*tok).st_pos = 0 as ::core::ffi::c_int;
                            (*(*tok).stack.offset((*tok).depth as isize)).state =
                                json_tokener_state_string_escape;
                        }
                    } else {
                        (*(*tok).stack.offset((*tok).depth as isize)).state =
                            json_tokener_state_escape_unicode;
                        current_block = 18421592892663940172;
                        break;
                    }
                }
                13 => {
                    size1 = 0;
                    size2 = 0;
                    if printbuf_memappend((*tok).pb, &raw mut c, 1 as ::core::ffi::c_int)
                        < 0 as ::core::ffi::c_int
                    {
                        (*tok).err = json_tokener_error_memory;
                        break 's_57;
                    } else {
                        size1 = if ((*tok).st_pos + 1 as ::core::ffi::c_int) < json_true_str_len {
                            (*tok).st_pos + 1 as ::core::ffi::c_int
                        } else {
                            json_true_str_len
                        };
                        size2 = if ((*tok).st_pos + 1 as ::core::ffi::c_int) < json_false_str_len {
                            (*tok).st_pos + 1 as ::core::ffi::c_int
                        } else {
                            json_false_str_len
                        };
                        if (*tok).flags & JSON_TOKENER_STRICT == 0
                            && strncasecmp(
                                &raw const json_true_str as *const ::core::ffi::c_char,
                                (*(*tok).pb).buf,
                                size1 as size_t,
                            ) == 0 as ::core::ffi::c_int
                            || strncmp(
                                &raw const json_true_str as *const ::core::ffi::c_char,
                                (*(*tok).pb).buf,
                                size1 as size_t,
                            ) == 0 as ::core::ffi::c_int
                        {
                            if !((*tok).st_pos == json_true_str_len) {
                                current_block = 7403371700182256114;
                                break;
                            }
                            let ref mut fresh8 =
                                (*(*tok).stack.offset((*tok).depth as isize)).current;
                            *fresh8 = json_object_new_boolean(1 as json_bool);
                            if (*(*tok).stack.offset((*tok).depth as isize))
                                .current
                                .is_null()
                            {
                                (*tok).err = json_tokener_error_memory;
                                break 's_57;
                            } else {
                                (*(*tok).stack.offset((*tok).depth as isize)).saved_state =
                                    json_tokener_state_finish;
                                (*(*tok).stack.offset((*tok).depth as isize)).state =
                                    json_tokener_state_eatws;
                            }
                        } else if (*tok).flags & JSON_TOKENER_STRICT == 0
                            && strncasecmp(
                                &raw const json_false_str as *const ::core::ffi::c_char,
                                (*(*tok).pb).buf,
                                size2 as size_t,
                            ) == 0 as ::core::ffi::c_int
                            || strncmp(
                                &raw const json_false_str as *const ::core::ffi::c_char,
                                (*(*tok).pb).buf,
                                size2 as size_t,
                            ) == 0 as ::core::ffi::c_int
                        {
                            if !((*tok).st_pos == json_false_str_len) {
                                current_block = 7403371700182256114;
                                break;
                            }
                            let ref mut fresh9 =
                                (*(*tok).stack.offset((*tok).depth as isize)).current;
                            *fresh9 = json_object_new_boolean(0 as json_bool);
                            if (*(*tok).stack.offset((*tok).depth as isize))
                                .current
                                .is_null()
                            {
                                (*tok).err = json_tokener_error_memory;
                                break 's_57;
                            } else {
                                (*(*tok).stack.offset((*tok).depth as isize)).saved_state =
                                    json_tokener_state_finish;
                                (*(*tok).stack.offset((*tok).depth as isize)).state =
                                    json_tokener_state_eatws;
                            }
                        } else {
                            (*tok).err = json_tokener_error_parse_boolean;
                            break 's_57;
                        }
                    }
                }
                14 => {
                    let mut case_start_2: *const ::core::ffi::c_char = str;
                    let mut case_len: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    let mut is_exponent: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    let mut neg_sign_ok: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                    let mut pos_sign_ok: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    if (*(*tok).pb).bpos > 0 as ::core::ffi::c_int {
                        let mut e_loc: *mut ::core::ffi::c_char =
                            strchr((*(*tok).pb).buf, 'e' as i32);
                        if e_loc.is_null() {
                            e_loc = strchr((*(*tok).pb).buf, 'E' as i32);
                        }
                        if !e_loc.is_null() {
                            let mut last_saved_char: *mut ::core::ffi::c_char = (*(*tok).pb)
                                .buf
                                .offset(((*(*tok).pb).bpos - 1 as ::core::ffi::c_int) as isize)
                                as *mut ::core::ffi::c_char;
                            is_exponent = 1 as ::core::ffi::c_int;
                            neg_sign_ok = 1 as ::core::ffi::c_int;
                            pos_sign_ok = neg_sign_ok;
                            if e_loc != last_saved_char {
                                neg_sign_ok = 0 as ::core::ffi::c_int;
                                pos_sign_ok = 0 as ::core::ffi::c_int;
                            }
                        }
                    }
                    while c as ::core::ffi::c_int != 0
                        && (c as ::core::ffi::c_int >= '0' as i32
                            && c as ::core::ffi::c_int <= '9' as i32
                            || is_exponent == 0
                                && (c as ::core::ffi::c_int == 'e' as i32
                                    || c as ::core::ffi::c_int == 'E' as i32)
                            || neg_sign_ok != 0 && c as ::core::ffi::c_int == '-' as i32
                            || pos_sign_ok != 0 && c as ::core::ffi::c_int == '+' as i32
                            || (*tok).is_double == 0 && c as ::core::ffi::c_int == '.' as i32)
                    {
                        neg_sign_ok = 0 as ::core::ffi::c_int;
                        pos_sign_ok = neg_sign_ok;
                        case_len += 1;
                        match c as ::core::ffi::c_int {
                            46 => {
                                (*tok).is_double = 1 as ::core::ffi::c_int;
                                pos_sign_ok = 1 as ::core::ffi::c_int;
                                neg_sign_ok = 1 as ::core::ffi::c_int;
                            }
                            101 | 69 => {
                                is_exponent = 1 as ::core::ffi::c_int;
                                (*tok).is_double = 1 as ::core::ffi::c_int;
                                neg_sign_ok = 1 as ::core::ffi::c_int;
                                pos_sign_ok = neg_sign_ok;
                            }
                            _ => {}
                        }
                        str = str.offset(1);
                        (*tok).char_offset += 1;
                        if !(c == 0
                            || (if (*tok).char_offset == len {
                                (if (*tok).depth == 0 as ::core::ffi::c_int
                                    && (*(*tok).stack.offset((*tok).depth as isize)).state
                                        as ::core::ffi::c_uint
                                        == json_tokener_state_eatws as ::core::ffi::c_int
                                            as ::core::ffi::c_uint
                                    && (*(*tok).stack.offset((*tok).depth as isize)).saved_state
                                        as ::core::ffi::c_uint
                                        == json_tokener_state_finish as ::core::ffi::c_int
                                            as ::core::ffi::c_uint
                                {
                                    (*tok).err = json_tokener_success;
                                    0 as ::core::ffi::c_int
                                } else {
                                    (*tok).err = json_tokener_continue;
                                    0 as ::core::ffi::c_int
                                })
                            } else {
                                (if (*tok).flags & JSON_TOKENER_VALIDATE_UTF8 != 0
                                    && json_tokener_validate_utf8(*str, nBytesp) == 0
                                {
                                    (*tok).err = json_tokener_error_parse_utf8_string;
                                    0 as ::core::ffi::c_int
                                } else {
                                    c = *str;
                                    1 as ::core::ffi::c_int
                                })
                            }) == 0)
                        {
                            continue;
                        }
                        if !(printbuf_memappend((*tok).pb, case_start_2, case_len)
                            < 0 as ::core::ffi::c_int)
                        {
                            break 's_57;
                        }
                        (*tok).err = json_tokener_error_memory;
                        break 's_57;
                    }
                    if (*tok).depth > 0 as ::core::ffi::c_int
                        && c as ::core::ffi::c_int != ',' as i32
                        && c as ::core::ffi::c_int != ']' as i32
                        && c as ::core::ffi::c_int != '}' as i32
                        && c as ::core::ffi::c_int != '/' as i32
                        && c as ::core::ffi::c_int != 'I' as i32
                        && c as ::core::ffi::c_int != 'i' as i32
                        && is_ws_char(c) == 0
                    {
                        (*tok).err = json_tokener_error_parse_number;
                        break 's_57;
                    } else {
                        if case_len > 0 as ::core::ffi::c_int {
                            if printbuf_memappend((*tok).pb, case_start_2, case_len)
                                < 0 as ::core::ffi::c_int
                            {
                                (*tok).err = json_tokener_error_memory;
                                break 's_57;
                            }
                        }
                        if *(*(*tok).pb).buf.offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                            == '-' as i32
                            && case_len <= 1 as ::core::ffi::c_int
                            && (c as ::core::ffi::c_int == 'i' as i32
                                || c as ::core::ffi::c_int == 'I' as i32)
                        {
                            (*(*tok).stack.offset((*tok).depth as isize)).state =
                                json_tokener_state_inf;
                            (*tok).st_pos = 0 as ::core::ffi::c_int;
                        } else {
                            if (*tok).is_double != 0 && (*tok).flags & JSON_TOKENER_STRICT == 0 {
                                while (*(*tok).pb).bpos > 1 as ::core::ffi::c_int {
                                    let mut last_char: ::core::ffi::c_char =
                                        *(*(*tok).pb).buf.offset(
                                            ((*(*tok).pb).bpos - 1 as ::core::ffi::c_int) as isize,
                                        );
                                    if last_char as ::core::ffi::c_int != 'e' as i32
                                        && last_char as ::core::ffi::c_int != 'E' as i32
                                        && last_char as ::core::ffi::c_int != '-' as i32
                                        && last_char as ::core::ffi::c_int != '+' as i32
                                    {
                                        break;
                                    }
                                    *(*(*tok).pb).buf.offset(
                                        ((*(*tok).pb).bpos - 1 as ::core::ffi::c_int) as isize,
                                    ) = '\0' as i32 as ::core::ffi::c_char;
                                    (*(*tok).pb).bpos -= 1;
                                }
                            }
                            let mut num64: int64_t = 0;
                            let mut numuint64: uint64_t = 0;
                            let mut numd: ::core::ffi::c_double = 0.;
                            if (*tok).is_double == 0
                                && *(*(*tok).pb).buf.offset(0 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int
                                    == '-' as i32
                                && json_parse_int64((*(*tok).pb).buf, &raw mut num64)
                                    == 0 as ::core::ffi::c_int
                            {
                                if *__errno_location() == ERANGE
                                    && (*tok).flags & JSON_TOKENER_STRICT != 0
                                {
                                    (*tok).err = json_tokener_error_parse_number;
                                    break 's_57;
                                } else {
                                    let ref mut fresh10 =
                                        (*(*tok).stack.offset((*tok).depth as isize)).current;
                                    *fresh10 = json_object_new_int64(num64);
                                    if (*(*tok).stack.offset((*tok).depth as isize))
                                        .current
                                        .is_null()
                                    {
                                        (*tok).err = json_tokener_error_memory;
                                        break 's_57;
                                    }
                                }
                            } else if (*tok).is_double == 0
                                && *(*(*tok).pb).buf.offset(0 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int
                                    != '-' as i32
                                && json_parse_uint64((*(*tok).pb).buf, &raw mut numuint64)
                                    == 0 as ::core::ffi::c_int
                            {
                                if *__errno_location() == ERANGE
                                    && (*tok).flags & JSON_TOKENER_STRICT != 0
                                {
                                    (*tok).err = json_tokener_error_parse_number;
                                    break 's_57;
                                } else if numuint64 != 0
                                    && *(*(*tok).pb).buf.offset(0 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_int
                                        == '0' as i32
                                    && (*tok).flags & JSON_TOKENER_STRICT != 0
                                {
                                    (*tok).err = json_tokener_error_parse_number;
                                    break 's_57;
                                } else if numuint64 <= INT64_MAX as uint64_t {
                                    num64 = numuint64 as int64_t;
                                    let ref mut fresh11 =
                                        (*(*tok).stack.offset((*tok).depth as isize)).current;
                                    *fresh11 = json_object_new_int64(num64);
                                    if (*(*tok).stack.offset((*tok).depth as isize))
                                        .current
                                        .is_null()
                                    {
                                        (*tok).err = json_tokener_error_memory;
                                        break 's_57;
                                    }
                                } else {
                                    let ref mut fresh12 =
                                        (*(*tok).stack.offset((*tok).depth as isize)).current;
                                    *fresh12 = json_object_new_uint64(numuint64);
                                    if (*(*tok).stack.offset((*tok).depth as isize))
                                        .current
                                        .is_null()
                                    {
                                        (*tok).err = json_tokener_error_memory;
                                        break 's_57;
                                    }
                                }
                            } else if (*tok).is_double != 0
                                && json_tokener_parse_double(
                                    (*(*tok).pb).buf,
                                    (*(*tok).pb).bpos,
                                    &raw mut numd,
                                ) == 0 as ::core::ffi::c_int
                            {
                                let ref mut fresh13 =
                                    (*(*tok).stack.offset((*tok).depth as isize)).current;
                                *fresh13 = json_object_new_double_s(numd, (*(*tok).pb).buf);
                                if (*(*tok).stack.offset((*tok).depth as isize))
                                    .current
                                    .is_null()
                                {
                                    (*tok).err = json_tokener_error_memory;
                                    break 's_57;
                                }
                            } else {
                                (*tok).err = json_tokener_error_parse_number;
                                break 's_57;
                            }
                            (*(*tok).stack.offset((*tok).depth as isize)).saved_state =
                                json_tokener_state_finish;
                            (*(*tok).stack.offset((*tok).depth as isize)).state =
                                json_tokener_state_eatws;
                        }
                    }
                }
                24 | 15 => {
                    if c as ::core::ffi::c_int == ']' as i32 {
                        json_object_array_shrink(
                            (*(*tok).stack.offset((*tok).depth as isize)).current,
                            0 as ::core::ffi::c_int,
                        );
                        if (*(*tok).stack.offset((*tok).depth as isize)).state
                            as ::core::ffi::c_uint
                            == json_tokener_state_array_after_sep as ::core::ffi::c_int
                                as ::core::ffi::c_uint
                            && (*tok).flags & JSON_TOKENER_STRICT != 0
                        {
                            current_block = 2055788883580451251;
                            break;
                        } else {
                            current_block = 9012643552738962216;
                            break;
                        }
                    } else if (*tok).depth >= (*tok).max_depth - 1 as ::core::ffi::c_int {
                        (*tok).err = json_tokener_error_depth;
                        break 's_57;
                    } else {
                        (*(*tok).stack.offset((*tok).depth as isize)).state =
                            json_tokener_state_array_add;
                        (*tok).depth += 1;
                        json_tokener_reset_level(tok, (*tok).depth);
                    }
                }
                16 => {
                    if json_object_array_add(
                        (*(*tok).stack.offset((*tok).depth as isize)).current,
                        obj,
                    ) != 0 as ::core::ffi::c_int
                    {
                        (*tok).err = json_tokener_error_memory;
                        break 's_57;
                    } else {
                        (*(*tok).stack.offset((*tok).depth as isize)).saved_state =
                            json_tokener_state_array_sep;
                        (*(*tok).stack.offset((*tok).depth as isize)).state =
                            json_tokener_state_eatws;
                    }
                }
                17 => {
                    if c as ::core::ffi::c_int == ']' as i32 {
                        current_block = 18086180200029192508;
                        break;
                    } else {
                        current_block = 10637788488912693274;
                        break;
                    }
                }
                18 | 25 => {
                    if c as ::core::ffi::c_int == '}' as i32 {
                        current_block = 4032406349246985652;
                        break;
                    } else {
                        current_block = 92411086296410708;
                        break;
                    }
                }
                19 => {
                    case_start_3 = str;
                    current_block = 17167530779362783665;
                    break;
                }
                20 => {
                    if c as ::core::ffi::c_int == ':' as i32 {
                        current_block = 5981395450828152283;
                        break;
                    } else {
                        current_block = 16816250088755962910;
                        break;
                    }
                }
                21 => {
                    if (*tok).depth >= (*tok).max_depth - 1 as ::core::ffi::c_int {
                        (*tok).err = json_tokener_error_depth;
                        break 's_57;
                    } else {
                        (*(*tok).stack.offset((*tok).depth as isize)).state =
                            json_tokener_state_object_value_add;
                        (*tok).depth += 1;
                        json_tokener_reset_level(tok, (*tok).depth);
                    }
                }
                22 => {
                    json_object_object_add(
                        (*(*tok).stack.offset((*tok).depth as isize)).current,
                        (*(*tok).stack.offset((*tok).depth as isize)).obj_field_name,
                        obj,
                    );
                    free(
                        (*(*tok).stack.offset((*tok).depth as isize)).obj_field_name
                            as *mut ::core::ffi::c_void,
                    );
                    let ref mut fresh15 =
                        (*(*tok).stack.offset((*tok).depth as isize)).obj_field_name;
                    *fresh15 = ::core::ptr::null_mut::<::core::ffi::c_char>();
                    (*(*tok).stack.offset((*tok).depth as isize)).saved_state =
                        json_tokener_state_object_sep;
                    (*(*tok).stack.offset((*tok).depth as isize)).state = json_tokener_state_eatws;
                }
                23 => {
                    if c as ::core::ffi::c_int == '}' as i32 {
                        current_block = 10725100205392175504;
                        break;
                    } else {
                        current_block = 2885348163762349817;
                        break;
                    }
                }
                _ => {
                    current_block = 18421592892663940172;
                    break;
                }
            }
        }
        match current_block {
            14648249180243006330 => {
                while c as ::core::ffi::c_int != '\n' as i32 {
                    str = str.offset(1);
                    (*tok).char_offset += 1;
                    if !(c == 0
                        || (if (*tok).char_offset == len {
                            (if (*tok).depth == 0 as ::core::ffi::c_int
                                && (*(*tok).stack.offset((*tok).depth as isize)).state
                                    as ::core::ffi::c_uint
                                    == json_tokener_state_eatws as ::core::ffi::c_int
                                        as ::core::ffi::c_uint
                                && (*(*tok).stack.offset((*tok).depth as isize)).saved_state
                                    as ::core::ffi::c_uint
                                    == json_tokener_state_finish as ::core::ffi::c_int
                                        as ::core::ffi::c_uint
                            {
                                (*tok).err = json_tokener_success;
                                0 as ::core::ffi::c_int
                            } else {
                                (*tok).err = json_tokener_continue;
                                0 as ::core::ffi::c_int
                            })
                        } else {
                            (if (*tok).flags & JSON_TOKENER_VALIDATE_UTF8 != 0
                                && json_tokener_validate_utf8(*str, nBytesp) == 0
                            {
                                (*tok).err = json_tokener_error_parse_utf8_string;
                                0 as ::core::ffi::c_int
                            } else {
                                c = *str;
                                1 as ::core::ffi::c_int
                            })
                        }) == 0)
                    {
                        continue;
                    }
                    if !(printbuf_memappend(
                        (*tok).pb,
                        case_start_0,
                        str.offset_from(case_start_0) as ::core::ffi::c_long as ::core::ffi::c_int,
                    ) < 0 as ::core::ffi::c_int)
                    {
                        break 's_57;
                    }
                    (*tok).err = json_tokener_error_memory;
                    break 's_57;
                }
                if printbuf_memappend(
                    (*tok).pb,
                    case_start_0,
                    str.offset_from(case_start_0) as ::core::ffi::c_long as ::core::ffi::c_int,
                ) < 0 as ::core::ffi::c_int
                {
                    (*tok).err = json_tokener_error_memory;
                    break;
                } else {
                    (*(*tok).stack.offset((*tok).depth as isize)).state = json_tokener_state_eatws;
                }
                current_block = 18421592892663940172;
            }
            11865390570819897086 => {
                while c as ::core::ffi::c_int != '*' as i32 {
                    str = str.offset(1);
                    (*tok).char_offset += 1;
                    if !(c == 0
                        || (if (*tok).char_offset == len {
                            (if (*tok).depth == 0 as ::core::ffi::c_int
                                && (*(*tok).stack.offset((*tok).depth as isize)).state
                                    as ::core::ffi::c_uint
                                    == json_tokener_state_eatws as ::core::ffi::c_int
                                        as ::core::ffi::c_uint
                                && (*(*tok).stack.offset((*tok).depth as isize)).saved_state
                                    as ::core::ffi::c_uint
                                    == json_tokener_state_finish as ::core::ffi::c_int
                                        as ::core::ffi::c_uint
                            {
                                (*tok).err = json_tokener_success;
                                0 as ::core::ffi::c_int
                            } else {
                                (*tok).err = json_tokener_continue;
                                0 as ::core::ffi::c_int
                            })
                        } else {
                            (if (*tok).flags & JSON_TOKENER_VALIDATE_UTF8 != 0
                                && json_tokener_validate_utf8(*str, nBytesp) == 0
                            {
                                (*tok).err = json_tokener_error_parse_utf8_string;
                                0 as ::core::ffi::c_int
                            } else {
                                c = *str;
                                1 as ::core::ffi::c_int
                            })
                        }) == 0)
                    {
                        continue;
                    }
                    if !(printbuf_memappend(
                        (*tok).pb,
                        case_start,
                        str.offset_from(case_start) as ::core::ffi::c_long as ::core::ffi::c_int,
                    ) < 0 as ::core::ffi::c_int)
                    {
                        break 's_57;
                    }
                    (*tok).err = json_tokener_error_memory;
                    break 's_57;
                }
                if printbuf_memappend(
                    (*tok).pb,
                    case_start,
                    str.offset(1 as ::core::ffi::c_int as isize)
                        .offset_from(case_start) as ::core::ffi::c_long
                        as ::core::ffi::c_int,
                ) < 0 as ::core::ffi::c_int
                {
                    (*tok).err = json_tokener_error_memory;
                    break;
                } else {
                    (*(*tok).stack.offset((*tok).depth as isize)).state =
                        json_tokener_state_comment_end;
                }
                current_block = 18421592892663940172;
            }
            2885348163762349817 => {
                if c as ::core::ffi::c_int == ',' as i32 {
                    (*(*tok).stack.offset((*tok).depth as isize)).saved_state =
                        json_tokener_state_object_field_start_after_sep;
                    (*(*tok).stack.offset((*tok).depth as isize)).state = json_tokener_state_eatws;
                } else {
                    (*tok).err = json_tokener_error_parse_object_value_sep;
                    break;
                }
                current_block = 18421592892663940172;
            }
            17167530779362783665 => {
                loop {
                    if c as ::core::ffi::c_int == (*tok).quote_char as ::core::ffi::c_int {
                        if printbuf_memappend(
                            (*tok).pb,
                            case_start_3,
                            str.offset_from(case_start_3) as ::core::ffi::c_long
                                as ::core::ffi::c_int,
                        ) < 0 as ::core::ffi::c_int
                        {
                            (*tok).err = json_tokener_error_memory;
                            break 's_57;
                        } else {
                            let ref mut fresh14 =
                                (*(*tok).stack.offset((*tok).depth as isize)).obj_field_name;
                            *fresh14 = strdup((*(*tok).pb).buf);
                            if (*(*tok).stack.offset((*tok).depth as isize))
                                .obj_field_name
                                .is_null()
                            {
                                (*tok).err = json_tokener_error_memory;
                                break 's_57;
                            } else {
                                (*(*tok).stack.offset((*tok).depth as isize)).saved_state =
                                    json_tokener_state_object_field_end;
                                (*(*tok).stack.offset((*tok).depth as isize)).state =
                                    json_tokener_state_eatws;
                                break;
                            }
                        }
                    } else if c as ::core::ffi::c_int == '\\' as i32 {
                        if printbuf_memappend(
                            (*tok).pb,
                            case_start_3,
                            str.offset_from(case_start_3) as ::core::ffi::c_long
                                as ::core::ffi::c_int,
                        ) < 0 as ::core::ffi::c_int
                        {
                            (*tok).err = json_tokener_error_memory;
                            break 's_57;
                        } else {
                            (*(*tok).stack.offset((*tok).depth as isize)).saved_state =
                                json_tokener_state_object_field;
                            (*(*tok).stack.offset((*tok).depth as isize)).state =
                                json_tokener_state_string_escape;
                            break;
                        }
                    } else {
                        str = str.offset(1);
                        (*tok).char_offset += 1;
                        if !(c == 0
                            || (if (*tok).char_offset == len {
                                (if (*tok).depth == 0 as ::core::ffi::c_int
                                    && (*(*tok).stack.offset((*tok).depth as isize)).state
                                        as ::core::ffi::c_uint
                                        == json_tokener_state_eatws as ::core::ffi::c_int
                                            as ::core::ffi::c_uint
                                    && (*(*tok).stack.offset((*tok).depth as isize)).saved_state
                                        as ::core::ffi::c_uint
                                        == json_tokener_state_finish as ::core::ffi::c_int
                                            as ::core::ffi::c_uint
                                {
                                    (*tok).err = json_tokener_success;
                                    0 as ::core::ffi::c_int
                                } else {
                                    (*tok).err = json_tokener_continue;
                                    0 as ::core::ffi::c_int
                                })
                            } else {
                                (if (*tok).flags & JSON_TOKENER_VALIDATE_UTF8 != 0
                                    && json_tokener_validate_utf8(*str, nBytesp) == 0
                                {
                                    (*tok).err = json_tokener_error_parse_utf8_string;
                                    0 as ::core::ffi::c_int
                                } else {
                                    c = *str;
                                    1 as ::core::ffi::c_int
                                })
                            }) == 0)
                        {
                            continue;
                        }
                        if !(printbuf_memappend(
                            (*tok).pb,
                            case_start_3,
                            str.offset_from(case_start_3) as ::core::ffi::c_long
                                as ::core::ffi::c_int,
                        ) < 0 as ::core::ffi::c_int)
                        {
                            break 's_57;
                        }
                        (*tok).err = json_tokener_error_memory;
                        break 's_57;
                    }
                }
                current_block = 18421592892663940172;
            }
            92411086296410708 => {
                if c as ::core::ffi::c_int == '"' as i32 || c as ::core::ffi::c_int == '\'' as i32 {
                    (*tok).quote_char = c;
                    printbuf_reset((*tok).pb);
                    (*(*tok).stack.offset((*tok).depth as isize)).state =
                        json_tokener_state_object_field;
                } else {
                    (*tok).err = json_tokener_error_parse_object_key_name;
                    break;
                }
                current_block = 18421592892663940172;
            }
            4032406349246985652 => {
                if (*(*tok).stack.offset((*tok).depth as isize)).state as ::core::ffi::c_uint
                    == json_tokener_state_object_field_start_after_sep as ::core::ffi::c_int
                        as ::core::ffi::c_uint
                    && (*tok).flags & JSON_TOKENER_STRICT != 0
                {
                    (*tok).err = json_tokener_error_parse_unexpected;
                    break;
                } else {
                    (*(*tok).stack.offset((*tok).depth as isize)).saved_state =
                        json_tokener_state_finish;
                    (*(*tok).stack.offset((*tok).depth as isize)).state = json_tokener_state_eatws;
                }
                current_block = 18421592892663940172;
            }
            10637788488912693274 => {
                if c as ::core::ffi::c_int == ',' as i32 {
                    (*(*tok).stack.offset((*tok).depth as isize)).saved_state =
                        json_tokener_state_array_after_sep;
                    (*(*tok).stack.offset((*tok).depth as isize)).state = json_tokener_state_eatws;
                } else {
                    (*tok).err = json_tokener_error_parse_array;
                    break;
                }
                current_block = 18421592892663940172;
            }
            9012643552738962216 => {
                (*(*tok).stack.offset((*tok).depth as isize)).saved_state =
                    json_tokener_state_finish;
                (*(*tok).stack.offset((*tok).depth as isize)).state = json_tokener_state_eatws;
                current_block = 18421592892663940172;
            }
            9255187738567101705 => {
                loop {
                    if c as ::core::ffi::c_int == (*tok).quote_char as ::core::ffi::c_int {
                        if printbuf_memappend(
                            (*tok).pb,
                            case_start_1,
                            str.offset_from(case_start_1) as ::core::ffi::c_long
                                as ::core::ffi::c_int,
                        ) < 0 as ::core::ffi::c_int
                        {
                            (*tok).err = json_tokener_error_memory;
                            break 's_57;
                        } else {
                            let ref mut fresh7 =
                                (*(*tok).stack.offset((*tok).depth as isize)).current;
                            *fresh7 =
                                json_object_new_string_len((*(*tok).pb).buf, (*(*tok).pb).bpos);
                            if (*(*tok).stack.offset((*tok).depth as isize))
                                .current
                                .is_null()
                            {
                                (*tok).err = json_tokener_error_memory;
                                break 's_57;
                            } else {
                                (*(*tok).stack.offset((*tok).depth as isize)).saved_state =
                                    json_tokener_state_finish;
                                (*(*tok).stack.offset((*tok).depth as isize)).state =
                                    json_tokener_state_eatws;
                                break;
                            }
                        }
                    } else if c as ::core::ffi::c_int == '\\' as i32 {
                        if printbuf_memappend(
                            (*tok).pb,
                            case_start_1,
                            str.offset_from(case_start_1) as ::core::ffi::c_long
                                as ::core::ffi::c_int,
                        ) < 0 as ::core::ffi::c_int
                        {
                            (*tok).err = json_tokener_error_memory;
                            break 's_57;
                        } else {
                            (*(*tok).stack.offset((*tok).depth as isize)).saved_state =
                                json_tokener_state_string;
                            (*(*tok).stack.offset((*tok).depth as isize)).state =
                                json_tokener_state_string_escape;
                            break;
                        }
                    } else {
                        str = str.offset(1);
                        (*tok).char_offset += 1;
                        if !(c == 0
                            || (if (*tok).char_offset == len {
                                (if (*tok).depth == 0 as ::core::ffi::c_int
                                    && (*(*tok).stack.offset((*tok).depth as isize)).state
                                        as ::core::ffi::c_uint
                                        == json_tokener_state_eatws as ::core::ffi::c_int
                                            as ::core::ffi::c_uint
                                    && (*(*tok).stack.offset((*tok).depth as isize)).saved_state
                                        as ::core::ffi::c_uint
                                        == json_tokener_state_finish as ::core::ffi::c_int
                                            as ::core::ffi::c_uint
                                {
                                    (*tok).err = json_tokener_success;
                                    0 as ::core::ffi::c_int
                                } else {
                                    (*tok).err = json_tokener_continue;
                                    0 as ::core::ffi::c_int
                                })
                            } else {
                                (if (*tok).flags & JSON_TOKENER_VALIDATE_UTF8 != 0
                                    && json_tokener_validate_utf8(*str, nBytesp) == 0
                                {
                                    (*tok).err = json_tokener_error_parse_utf8_string;
                                    0 as ::core::ffi::c_int
                                } else {
                                    c = *str;
                                    1 as ::core::ffi::c_int
                                })
                            }) == 0)
                        {
                            continue;
                        }
                        if !(printbuf_memappend(
                            (*tok).pb,
                            case_start_1,
                            str.offset_from(case_start_1) as ::core::ffi::c_long
                                as ::core::ffi::c_int,
                        ) < 0 as ::core::ffi::c_int)
                        {
                            break 's_57;
                        }
                        (*tok).err = json_tokener_error_memory;
                        break 's_57;
                    }
                }
                current_block = 18421592892663940172;
            }
            6950536787749910113 => {
                loop {
                    if c == 0 || is_hex_char(c) == 0 {
                        (*tok).err = json_tokener_error_parse_string;
                        break 's_57;
                    } else {
                        (*tok).ucs_char |= ((if c as ::core::ffi::c_int <= '9' as i32 {
                            c as ::core::ffi::c_int - '0' as i32
                        } else {
                            (c as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                                + 9 as ::core::ffi::c_int
                        }) as ::core::ffi::c_uint)
                            << (3 as ::core::ffi::c_int - (*tok).st_pos) * 4 as ::core::ffi::c_int;
                        (*tok).st_pos += 1;
                        if (*tok).st_pos >= 4 as ::core::ffi::c_int {
                            break;
                        }
                        str = str.offset(1);
                        (*tok).char_offset += 1;
                        if if (*tok).char_offset == len {
                            if (*tok).depth == 0 as ::core::ffi::c_int
                                && (*(*tok).stack.offset((*tok).depth as isize)).state
                                    as ::core::ffi::c_uint
                                    == json_tokener_state_eatws as ::core::ffi::c_int
                                        as ::core::ffi::c_uint
                                && (*(*tok).stack.offset((*tok).depth as isize)).saved_state
                                    as ::core::ffi::c_uint
                                    == json_tokener_state_finish as ::core::ffi::c_int
                                        as ::core::ffi::c_uint
                            {
                                (*tok).err = json_tokener_success;
                                0 as ::core::ffi::c_int
                            } else {
                                (*tok).err = json_tokener_continue;
                                0 as ::core::ffi::c_int
                            }
                        } else if (*tok).flags & JSON_TOKENER_VALIDATE_UTF8 != 0
                            && json_tokener_validate_utf8(*str, nBytesp) == 0
                        {
                            (*tok).err = json_tokener_error_parse_utf8_string;
                            0 as ::core::ffi::c_int
                        } else {
                            c = *str;
                            1 as ::core::ffi::c_int
                        } == 0
                        {
                            break 's_57;
                        }
                    }
                }
                (*tok).st_pos = 0 as ::core::ffi::c_int;
                if (*tok).high_surrogate != 0 {
                    if (*tok).ucs_char & 0xfc00 as ::core::ffi::c_uint
                        == 0xdc00 as ::core::ffi::c_uint
                    {
                        (*tok).ucs_char = (((*tok).high_surrogate & 0x3ff as ::core::ffi::c_uint)
                            << 10 as ::core::ffi::c_int)
                            .wrapping_add((*tok).ucs_char & 0x3ff as ::core::ffi::c_uint)
                            .wrapping_add(0x10000 as ::core::ffi::c_int as ::core::ffi::c_uint);
                    } else if printbuf_memappend(
                        (*tok).pb,
                        &raw mut utf8_replacement_char as *mut ::core::ffi::c_uchar
                            as *mut ::core::ffi::c_char,
                        3 as ::core::ffi::c_int,
                    ) < 0 as ::core::ffi::c_int
                    {
                        (*tok).err = json_tokener_error_memory;
                        break;
                    }
                    (*tok).high_surrogate = 0 as ::core::ffi::c_uint;
                }
                if (*tok).ucs_char < 0x80 as ::core::ffi::c_uint {
                    let mut unescaped_utf: [::core::ffi::c_uchar; 1] = [0; 1];
                    unescaped_utf[0 as ::core::ffi::c_int as usize] =
                        (*tok).ucs_char as ::core::ffi::c_uchar;
                    if printbuf_memappend(
                        (*tok).pb,
                        &raw mut unescaped_utf as *mut ::core::ffi::c_uchar
                            as *mut ::core::ffi::c_char,
                        1 as ::core::ffi::c_int,
                    ) < 0 as ::core::ffi::c_int
                    {
                        (*tok).err = json_tokener_error_memory;
                        break;
                    } else {
                        current_block = 18330534242458572360;
                    }
                } else if (*tok).ucs_char < 0x800 as ::core::ffi::c_uint {
                    let mut unescaped_utf_0: [::core::ffi::c_uchar; 2] = [0; 2];
                    unescaped_utf_0[0 as ::core::ffi::c_int as usize] =
                        (0xc0 as ::core::ffi::c_uint | (*tok).ucs_char >> 6 as ::core::ffi::c_int)
                            as ::core::ffi::c_uchar;
                    unescaped_utf_0[1 as ::core::ffi::c_int as usize] = (0x80
                        as ::core::ffi::c_uint
                        | (*tok).ucs_char & 0x3f as ::core::ffi::c_uint)
                        as ::core::ffi::c_uchar;
                    if printbuf_memappend(
                        (*tok).pb,
                        &raw mut unescaped_utf_0 as *mut ::core::ffi::c_uchar
                            as *mut ::core::ffi::c_char,
                        2 as ::core::ffi::c_int,
                    ) < 0 as ::core::ffi::c_int
                    {
                        (*tok).err = json_tokener_error_memory;
                        break;
                    } else {
                        current_block = 18330534242458572360;
                    }
                } else if (*tok).ucs_char & 0xfc00 as ::core::ffi::c_uint
                    == 0xd800 as ::core::ffi::c_uint
                {
                    (*tok).high_surrogate = (*tok).ucs_char;
                    (*tok).ucs_char = 0 as ::core::ffi::c_uint;
                    (*(*tok).stack.offset((*tok).depth as isize)).state =
                        json_tokener_state_escape_unicode_need_escape;
                    current_block = 18421592892663940172;
                } else {
                    if (*tok).ucs_char & 0xfc00 as ::core::ffi::c_uint
                        == 0xdc00 as ::core::ffi::c_uint
                    {
                        if printbuf_memappend(
                            (*tok).pb,
                            &raw mut utf8_replacement_char as *mut ::core::ffi::c_uchar
                                as *mut ::core::ffi::c_char,
                            3 as ::core::ffi::c_int,
                        ) < 0 as ::core::ffi::c_int
                        {
                            (*tok).err = json_tokener_error_memory;
                            break;
                        }
                    } else if (*tok).ucs_char < 0x10000 as ::core::ffi::c_int as ::core::ffi::c_uint
                    {
                        let mut unescaped_utf_1: [::core::ffi::c_uchar; 3] = [0; 3];
                        unescaped_utf_1[0 as ::core::ffi::c_int as usize] = (0xe0
                            as ::core::ffi::c_uint
                            | (*tok).ucs_char >> 12 as ::core::ffi::c_int)
                            as ::core::ffi::c_uchar;
                        unescaped_utf_1[1 as ::core::ffi::c_int as usize] = (0x80
                            as ::core::ffi::c_uint
                            | (*tok).ucs_char >> 6 as ::core::ffi::c_int
                                & 0x3f as ::core::ffi::c_uint)
                            as ::core::ffi::c_uchar;
                        unescaped_utf_1[2 as ::core::ffi::c_int as usize] = (0x80
                            as ::core::ffi::c_uint
                            | (*tok).ucs_char & 0x3f as ::core::ffi::c_uint)
                            as ::core::ffi::c_uchar;
                        if printbuf_memappend(
                            (*tok).pb,
                            &raw mut unescaped_utf_1 as *mut ::core::ffi::c_uchar
                                as *mut ::core::ffi::c_char,
                            3 as ::core::ffi::c_int,
                        ) < 0 as ::core::ffi::c_int
                        {
                            (*tok).err = json_tokener_error_memory;
                            break;
                        }
                    } else if (*tok).ucs_char
                        < 0x110000 as ::core::ffi::c_int as ::core::ffi::c_uint
                    {
                        let mut unescaped_utf_2: [::core::ffi::c_uchar; 4] = [0; 4];
                        unescaped_utf_2[0 as ::core::ffi::c_int as usize] = (0xf0
                            as ::core::ffi::c_uint
                            | (*tok).ucs_char >> 18 as ::core::ffi::c_int
                                & 0x7 as ::core::ffi::c_uint)
                            as ::core::ffi::c_uchar;
                        unescaped_utf_2[1 as ::core::ffi::c_int as usize] = (0x80
                            as ::core::ffi::c_uint
                            | (*tok).ucs_char >> 12 as ::core::ffi::c_int
                                & 0x3f as ::core::ffi::c_uint)
                            as ::core::ffi::c_uchar;
                        unescaped_utf_2[2 as ::core::ffi::c_int as usize] = (0x80
                            as ::core::ffi::c_uint
                            | (*tok).ucs_char >> 6 as ::core::ffi::c_int
                                & 0x3f as ::core::ffi::c_uint)
                            as ::core::ffi::c_uchar;
                        unescaped_utf_2[3 as ::core::ffi::c_int as usize] = (0x80
                            as ::core::ffi::c_uint
                            | (*tok).ucs_char & 0x3f as ::core::ffi::c_uint)
                            as ::core::ffi::c_uchar;
                        if printbuf_memappend(
                            (*tok).pb,
                            &raw mut unescaped_utf_2 as *mut ::core::ffi::c_uchar
                                as *mut ::core::ffi::c_char,
                            4 as ::core::ffi::c_int,
                        ) < 0 as ::core::ffi::c_int
                        {
                            (*tok).err = json_tokener_error_memory;
                            break;
                        }
                    } else if printbuf_memappend(
                        (*tok).pb,
                        &raw mut utf8_replacement_char as *mut ::core::ffi::c_uchar
                            as *mut ::core::ffi::c_char,
                        3 as ::core::ffi::c_int,
                    ) < 0 as ::core::ffi::c_int
                    {
                        (*tok).err = json_tokener_error_memory;
                        break;
                    }
                    current_block = 18330534242458572360;
                }
                match current_block {
                    18421592892663940172 => {}
                    _ => {
                        (*(*tok).stack.offset((*tok).depth as isize)).state =
                            (*(*tok).stack.offset((*tok).depth as isize)).saved_state;
                        current_block = 18421592892663940172;
                    }
                }
            }
            16011976826082558567 => {
                (*tok).ucs_char = 0 as ::core::ffi::c_uint;
                (*tok).st_pos = 0 as ::core::ffi::c_int;
                (*(*tok).stack.offset((*tok).depth as isize)).state =
                    json_tokener_state_escape_unicode;
                current_block = 18421592892663940172;
            }
            10799024050558338665 => {
                if c as ::core::ffi::c_int == 'b' as i32 {
                    if printbuf_memappend(
                        (*tok).pb,
                        b"\x08\0" as *const u8 as *const ::core::ffi::c_char,
                        1 as ::core::ffi::c_int,
                    ) < 0 as ::core::ffi::c_int
                    {
                        (*tok).err = json_tokener_error_memory;
                        break;
                    }
                } else if c as ::core::ffi::c_int == 'n' as i32 {
                    if printbuf_memappend(
                        (*tok).pb,
                        b"\n\0" as *const u8 as *const ::core::ffi::c_char,
                        1 as ::core::ffi::c_int,
                    ) < 0 as ::core::ffi::c_int
                    {
                        (*tok).err = json_tokener_error_memory;
                        break;
                    }
                } else if c as ::core::ffi::c_int == 'r' as i32 {
                    if printbuf_memappend(
                        (*tok).pb,
                        b"\r\0" as *const u8 as *const ::core::ffi::c_char,
                        1 as ::core::ffi::c_int,
                    ) < 0 as ::core::ffi::c_int
                    {
                        (*tok).err = json_tokener_error_memory;
                        break;
                    }
                } else if c as ::core::ffi::c_int == 't' as i32 {
                    if printbuf_memappend(
                        (*tok).pb,
                        b"\t\0" as *const u8 as *const ::core::ffi::c_char,
                        1 as ::core::ffi::c_int,
                    ) < 0 as ::core::ffi::c_int
                    {
                        (*tok).err = json_tokener_error_memory;
                        break;
                    }
                } else if c as ::core::ffi::c_int == 'f' as i32 {
                    if printbuf_memappend(
                        (*tok).pb,
                        b"\x0C\0" as *const u8 as *const ::core::ffi::c_char,
                        1 as ::core::ffi::c_int,
                    ) < 0 as ::core::ffi::c_int
                    {
                        (*tok).err = json_tokener_error_memory;
                        break;
                    }
                }
                (*(*tok).stack.offset((*tok).depth as isize)).state =
                    (*(*tok).stack.offset((*tok).depth as isize)).saved_state;
                current_block = 18421592892663940172;
            }
            9240481512215375588 => {
                if printbuf_memappend((*tok).pb, &raw mut c, 1 as ::core::ffi::c_int)
                    < 0 as ::core::ffi::c_int
                {
                    (*tok).err = json_tokener_error_memory;
                    break;
                } else {
                    (*(*tok).stack.offset((*tok).depth as isize)).state =
                        (*(*tok).stack.offset((*tok).depth as isize)).saved_state;
                }
                current_block = 18421592892663940172;
            }
            6406431739208918833 => {
                if c as ::core::ffi::c_int == '/' as i32 {
                    (*(*tok).stack.offset((*tok).depth as isize)).state =
                        json_tokener_state_comment_eol;
                } else {
                    (*tok).err = json_tokener_error_parse_comment;
                    break;
                }
                current_block = 2705889988320590074;
            }
            964414645882542267 => {
                (*(*tok).stack.offset((*tok).depth as isize)).state = json_tokener_state_string;
                printbuf_reset((*tok).pb);
                (*tok).quote_char = c;
                current_block = 18421592892663940172;
            }
            14579489411542934868 => {
                if c as ::core::ffi::c_int == '/' as i32 {
                    (*(*tok).stack.offset((*tok).depth as isize)).state = json_tokener_state_eatws;
                } else {
                    (*(*tok).stack.offset((*tok).depth as isize)).state =
                        json_tokener_state_comment;
                }
                current_block = 18421592892663940172;
            }
            13221763560517913443 => {
                (*tok).err = json_tokener_error_parse_string;
                break;
            }
            16816250088755962910 => {
                (*tok).err = json_tokener_error_parse_object_key_sep;
                break;
            }
            26972500619410423 => {
                (*(*tok).stack.offset((*tok).depth as isize)).state =
                    json_tokener_state_comment_start;
                current_block = 18421592892663940172;
            }
            11057878835866523405 => {
                (*tok).err = json_tokener_error_memory;
                break;
            }
            17281240262373992796 => {
                (*tok).err = json_tokener_error_memory;
                break;
            }
            11743904203796629665 => {
                (*tok).err = json_tokener_error_parse_unexpected;
                break;
            }
            5689001924483802034 => {
                (*tok).err = json_tokener_error_memory;
                break;
            }
            10725100205392175504 => {
                (*(*tok).stack.offset((*tok).depth as isize)).saved_state =
                    json_tokener_state_finish;
                (*(*tok).stack.offset((*tok).depth as isize)).state = json_tokener_state_eatws;
                current_block = 18421592892663940172;
            }
            5981395450828152283 => {
                (*(*tok).stack.offset((*tok).depth as isize)).saved_state =
                    json_tokener_state_object_value;
                (*(*tok).stack.offset((*tok).depth as isize)).state = json_tokener_state_eatws;
                current_block = 18421592892663940172;
            }
            16313536926714486912 => {
                (*tok).st_pos += 1;
                current_block = 18421592892663940172;
            }
            18086180200029192508 => {
                json_object_array_shrink(
                    (*(*tok).stack.offset((*tok).depth as isize)).current,
                    0 as ::core::ffi::c_int,
                );
                (*(*tok).stack.offset((*tok).depth as isize)).saved_state =
                    json_tokener_state_finish;
                (*(*tok).stack.offset((*tok).depth as isize)).state = json_tokener_state_eatws;
                current_block = 18421592892663940172;
            }
            7370318721998929769 => {
                (*tok).err = json_tokener_error_memory;
                break;
            }
            7416055328783156979 => {
                (*(*tok).stack.offset((*tok).depth as isize)).state = json_tokener_state_comment;
                current_block = 2705889988320590074;
            }
            7403371700182256114 => {
                (*tok).st_pos += 1;
                current_block = 18421592892663940172;
            }
            2055788883580451251 => {
                (*tok).err = json_tokener_error_parse_unexpected;
                break;
            }
            _ => {}
        }
        match current_block {
            2705889988320590074 => {
                if printbuf_memappend((*tok).pb, &raw mut c, 1 as ::core::ffi::c_int)
                    < 0 as ::core::ffi::c_int
                {
                    (*tok).err = json_tokener_error_memory;
                    break;
                }
            }
            _ => {}
        }
        str = str.offset(1);
        (*tok).char_offset += 1;
        if c == 0 {
            break;
        }
    }
    if (*tok).flags & JSON_TOKENER_VALIDATE_UTF8 != 0 && nBytes != 0 as ::core::ffi::c_uint {
        (*tok).err = json_tokener_error_parse_utf8_string;
    }
    if c as ::core::ffi::c_int != 0
        && (*(*tok).stack.offset((*tok).depth as isize)).state as ::core::ffi::c_uint
            == json_tokener_state_finish as ::core::ffi::c_int as ::core::ffi::c_uint
        && (*tok).depth == 0 as ::core::ffi::c_int
        && (*tok).flags & (JSON_TOKENER_STRICT | JSON_TOKENER_ALLOW_TRAILING_CHARS)
            == JSON_TOKENER_STRICT
    {
        (*tok).err = json_tokener_error_parse_unexpected;
    }
    if c == 0 {
        if (*(*tok).stack.offset((*tok).depth as isize)).state as ::core::ffi::c_uint
            != json_tokener_state_finish as ::core::ffi::c_int as ::core::ffi::c_uint
            && (*(*tok).stack.offset((*tok).depth as isize)).saved_state as ::core::ffi::c_uint
                != json_tokener_state_finish as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            (*tok).err = json_tokener_error_parse_eof;
        }
    }
    uselocale(oldlocale);
    freelocale(newloc);
    if (*tok).err as ::core::ffi::c_uint
        == json_tokener_success as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        let mut ret: *mut json_object =
            json_object_get((*(*tok).stack.offset((*tok).depth as isize)).current)
                as *mut json_object;
        let mut ii: ::core::ffi::c_int = 0;
        ii = (*tok).depth;
        while ii >= 0 as ::core::ffi::c_int {
            json_tokener_reset_level(tok, ii);
            ii -= 1;
        }
        return ret as *mut json_object;
    }
    return ::core::ptr::null_mut::<json_object>();
}
unsafe extern "C" fn json_tokener_validate_utf8(
    c: ::core::ffi::c_char,
    mut nBytes: *mut ::core::ffi::c_uint,
) -> json_bool {
    let mut chr: ::core::ffi::c_uchar = c as ::core::ffi::c_uchar;
    if *nBytes == 0 as ::core::ffi::c_uint {
        if chr as ::core::ffi::c_int >= 0x80 as ::core::ffi::c_int {
            if chr as ::core::ffi::c_int & 0xe0 as ::core::ffi::c_int == 0xc0 as ::core::ffi::c_int
            {
                *nBytes = 1 as ::core::ffi::c_uint;
            } else if chr as ::core::ffi::c_int & 0xf0 as ::core::ffi::c_int
                == 0xe0 as ::core::ffi::c_int
            {
                *nBytes = 2 as ::core::ffi::c_uint;
            } else if chr as ::core::ffi::c_int & 0xf8 as ::core::ffi::c_int
                == 0xf0 as ::core::ffi::c_int
            {
                *nBytes = 3 as ::core::ffi::c_uint;
            } else {
                return 0 as json_bool;
            }
        }
    } else {
        if chr as ::core::ffi::c_int & 0xc0 as ::core::ffi::c_int != 0x80 as ::core::ffi::c_int {
            return 0 as json_bool;
        }
        *nBytes = (*nBytes).wrapping_sub(1);
    }
    return 1 as json_bool;
}
#[no_mangle]
pub unsafe extern "C" fn json_tokener_set_flags(
    mut tok: *mut json_tokener,
    mut flags: ::core::ffi::c_int,
) {
    (*tok).flags = flags;
}
#[no_mangle]
pub unsafe extern "C" fn json_tokener_get_parse_end(mut tok: *mut json_tokener) -> size_t {
    return (*tok).char_offset as size_t;
}
unsafe extern "C" fn json_tokener_parse_double(
    mut buf: *const ::core::ffi::c_char,
    mut len: ::core::ffi::c_int,
    mut retval: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let mut end: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    *retval = strtod(buf, &raw mut end);
    if buf.offset(len as isize) == end as *const ::core::ffi::c_char {
        return 0 as ::core::ffi::c_int;
    }
    return 1 as ::core::ffi::c_int;
}
pub const LC_NUMERIC_MASK: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << __LC_NUMERIC;
pub const ERANGE: ::core::ffi::c_int = 34 as ::core::ffi::c_int;
pub const __LC_NUMERIC: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
unsafe extern "C" fn run_static_initializers() {
    json_false_str_len = (::core::mem::size_of::<[::core::ffi::c_char; 6]>() as usize)
        .wrapping_sub(1 as usize) as ::core::ffi::c_int;
    json_true_str_len = (::core::mem::size_of::<[::core::ffi::c_char; 5]>() as usize)
        .wrapping_sub(1 as usize) as ::core::ffi::c_int;
    json_nan_str_len = (::core::mem::size_of::<[::core::ffi::c_char; 4]>() as usize)
        .wrapping_sub(1 as usize) as ::core::ffi::c_int;
    json_null_str_len = (::core::mem::size_of::<[::core::ffi::c_char; 5]>() as usize)
        .wrapping_sub(1 as usize) as ::core::ffi::c_int;
    json_inf_str_len = (::core::mem::size_of::<[::core::ffi::c_char; 9]>() as usize)
        .wrapping_sub(1 as usize) as ::core::ffi::c_uint;
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
