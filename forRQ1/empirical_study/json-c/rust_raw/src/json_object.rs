extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn printbuf_new() -> *mut printbuf;
    fn printbuf_memappend(
        p: *mut printbuf,
        buf: *const ::core::ffi::c_char,
        size: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn printbuf_memset(
        pb: *mut printbuf,
        offset: ::core::ffi::c_int,
        charvalue: ::core::ffi::c_int,
        len: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn printbuf_reset(p: *mut printbuf);
    fn printbuf_free(p: *mut printbuf);
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memcmp(
        __s1: *const ::core::ffi::c_void,
        __s2: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn strcat(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strdup(__s: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn strchr(__s: *const ::core::ffi::c_char, __c: ::core::ffi::c_int)
        -> *mut ::core::ffi::c_char;
    fn strstr(
        __haystack: *const ::core::ffi::c_char,
        __needle: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    static mut stderr: *mut FILE;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn strtod(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_double;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn abort() -> !;
    fn json_parse_int64(
        buf: *const ::core::ffi::c_char,
        retval: *mut int64_t,
    ) -> ::core::ffi::c_int;
    fn json_parse_uint64(
        buf: *const ::core::ffi::c_char,
        retval: *mut uint64_t,
    ) -> ::core::ffi::c_int;
    fn array_list_new2(
        free_fn: Option<array_list_free_fn>,
        initial_size: ::core::ffi::c_int,
    ) -> *mut array_list;
    fn array_list_free(al: *mut array_list);
    fn array_list_get_idx(al: *mut array_list, i: size_t) -> *mut ::core::ffi::c_void;
    fn array_list_insert_idx(
        al: *mut array_list,
        i: size_t,
        data: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn array_list_put_idx(
        al: *mut array_list,
        i: size_t,
        data: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn array_list_add(al: *mut array_list, data: *mut ::core::ffi::c_void) -> ::core::ffi::c_int;
    fn array_list_length(al: *mut array_list) -> size_t;
    fn array_list_sort(
        arr: *mut array_list,
        compar: Option<
            unsafe extern "C" fn(
                *const ::core::ffi::c_void,
                *const ::core::ffi::c_void,
            ) -> ::core::ffi::c_int,
        >,
    );
    fn array_list_bsearch(
        key: *mut *const ::core::ffi::c_void,
        arr: *mut array_list,
        compar: Option<
            unsafe extern "C" fn(
                *const ::core::ffi::c_void,
                *const ::core::ffi::c_void,
            ) -> ::core::ffi::c_int,
        >,
    ) -> *mut ::core::ffi::c_void;
    fn array_list_del_idx(arr: *mut array_list, idx: size_t, count: size_t) -> ::core::ffi::c_int;
    fn array_list_shrink(arr: *mut array_list, empty_slots: size_t) -> ::core::ffi::c_int;
    fn _json_c_set_last_err(err_fmt: *const ::core::ffi::c_char, ...);
    fn lh_kchar_table_new(
        size: ::core::ffi::c_int,
        free_fn: Option<lh_entry_free_fn>,
    ) -> *mut lh_table;
    fn lh_table_free(t: *mut lh_table);
    fn lh_table_insert_w_hash(
        t: *mut lh_table,
        k: *const ::core::ffi::c_void,
        v: *const ::core::ffi::c_void,
        h: ::core::ffi::c_ulong,
        opts: ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;
    fn lh_table_lookup_entry_w_hash(
        t: *mut lh_table,
        k: *const ::core::ffi::c_void,
        h: ::core::ffi::c_ulong,
    ) -> *mut lh_entry;
    fn lh_table_lookup_ex(
        t: *mut lh_table,
        k: *const ::core::ffi::c_void,
        v: *mut *mut ::core::ffi::c_void,
    ) -> json_bool;
    fn lh_table_delete(t: *mut lh_table, k: *const ::core::ffi::c_void) -> ::core::ffi::c_int;
    fn lh_table_length(t: *mut lh_table) -> ::core::ffi::c_int;
}
pub type __int32_t = i32;
pub type __uint32_t = u32;
pub type __int64_t = i64;
pub type __uint64_t = u64;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type __ssize_t = ::core::ffi::c_long;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type uintptr_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct printbuf {
    pub buf: *mut ::core::ffi::c_char,
    pub bpos: ::core::ffi::c_int,
    pub size: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct json_object_iter {
    pub key: *mut ::core::ffi::c_char,
    pub val: *mut json_object,
    pub entry: *mut lh_entry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lh_entry {
    pub k: *const ::core::ffi::c_void,
    pub k_is_constant: ::core::ffi::c_int,
    pub v: *const ::core::ffi::c_void,
    pub next: *mut lh_entry,
    pub prev: *mut lh_entry,
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
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub idata: [::core::ffi::c_char; 1],
    pub pdata: *mut ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct json_object_string {
    pub base: json_object,
    pub len: ssize_t,
    pub c_string: C2RustUnnamed,
}
pub type ssize_t = __ssize_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct array_list {
    pub array: *mut *mut ::core::ffi::c_void,
    pub length: size_t,
    pub size: size_t,
    pub free_fn: Option<array_list_free_fn>,
}
pub type array_list_free_fn = unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct json_object_array {
    pub base: json_object,
    pub c_array: *mut array_list,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lh_table {
    pub size: ::core::ffi::c_int,
    pub count: ::core::ffi::c_int,
    pub head: *mut lh_entry,
    pub tail: *mut lh_entry,
    pub table: *mut lh_entry,
    pub free_fn: Option<lh_entry_free_fn>,
    pub hash_fn: Option<lh_hash_fn>,
    pub equal_fn: Option<lh_equal_fn>,
}
pub type lh_equal_fn = unsafe extern "C" fn(
    *const ::core::ffi::c_void,
    *const ::core::ffi::c_void,
) -> ::core::ffi::c_int;
pub type lh_hash_fn = unsafe extern "C" fn(*const ::core::ffi::c_void) -> ::core::ffi::c_ulong;
pub type lh_entry_free_fn = unsafe extern "C" fn(*mut lh_entry) -> ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct json_object_object {
    pub base: json_object,
    pub c_object: *mut lh_table,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub c_int64: int64_t,
    pub c_uint64: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct json_object_int {
    pub base: json_object,
    pub cint_type: json_object_int_type,
    pub cint: C2RustUnnamed_0,
}
pub type json_object_int_type = ::core::ffi::c_uint;
pub const json_object_int_type_uint64: json_object_int_type = 1;
pub const json_object_int_type_int64: json_object_int_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct json_object_double {
    pub base: json_object,
    pub c_double: ::core::ffi::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct json_object_boolean {
    pub base: json_object,
    pub c_boolean: json_bool,
}
pub type FILE = _IO_FILE;
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
pub type json_c_shallow_copy_fn = unsafe extern "C" fn(
    *mut json_object,
    *mut json_object,
    *const ::core::ffi::c_char,
    size_t,
    *mut *mut json_object,
) -> ::core::ffi::c_int;
pub const INT32_MIN: ::core::ffi::c_int =
    -(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int;
pub const INT64_MIN: ::core::ffi::c_long =
    -(9223372036854775807 as ::core::ffi::c_long) - 1 as ::core::ffi::c_long;
pub const INT32_MAX: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
pub const INT64_MAX: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
pub const UINT64_MAX: ::core::ffi::c_ulong = 18446744073709551615 as ::core::ffi::c_ulong;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const JSON_OBJECT_DEF_HASH_ENTRIES: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const JSON_C_TO_STRING_SPACED: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int;
pub const JSON_C_TO_STRING_PRETTY: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int;
pub const JSON_C_TO_STRING_PRETTY_TAB: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int;
pub const JSON_C_TO_STRING_NOZERO: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int;
pub const JSON_C_TO_STRING_NOSLASHESCAPE: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int;
pub const JSON_C_TO_STRING_COLOR: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int;
pub const JSON_C_OBJECT_ADD_KEY_IS_NEW: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int;
pub const JSON_C_OBJECT_ADD_CONSTANT_KEY: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int;
pub const JSON_C_OPTION_GLOBAL: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const JSON_C_OPTION_THREAD: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const ENOMEM: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
pub const EINVAL: ::core::ffi::c_int = 22 as ::core::ffi::c_int;
pub const ERANGE: ::core::ffi::c_int = 34 as ::core::ffi::c_int;
pub const INT_MAX: ::core::ffi::c_int = __INT_MAX__;
pub const LONG_MAX: ::core::ffi::c_long = __LONG_MAX__;
pub const UINT_MAX: ::core::ffi::c_uint = (__INT_MAX__ as ::core::ffi::c_uint)
    .wrapping_mul(2 as ::core::ffi::c_uint)
    .wrapping_add(1 as ::core::ffi::c_uint);
pub const ARRAY_LIST_DEFAULT_SIZE: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn lh_table_head(mut t: *const lh_table) -> *mut lh_entry {
    return (*t).head;
}
#[inline]
unsafe extern "C" fn lh_get_hash(
    mut t: *const lh_table,
    mut k: *const ::core::ffi::c_void,
) -> ::core::ffi::c_ulong {
    return (*t).hash_fn.expect("non-null function pointer")(k);
}
#[inline]
unsafe extern "C" fn lh_entry_k(mut e: *const lh_entry) -> *mut ::core::ffi::c_void {
    return (*e).k as uintptr_t as *mut ::core::ffi::c_void;
}
#[inline]
unsafe extern "C" fn lh_entry_k_is_constant(mut e: *const lh_entry) -> ::core::ffi::c_int {
    return (*e).k_is_constant;
}
#[inline]
unsafe extern "C" fn lh_entry_v(mut e: *const lh_entry) -> *mut ::core::ffi::c_void {
    return (*e).v as uintptr_t as *mut ::core::ffi::c_void;
}
#[inline]
unsafe extern "C" fn lh_entry_set_val(mut e: *mut lh_entry, mut newval: *mut ::core::ffi::c_void) {
    (*e).v = newval;
}
#[inline]
unsafe extern "C" fn lh_entry_next(mut e: *const lh_entry) -> *mut lh_entry {
    return (*e).next;
}
pub const SSIZE_T_MAX: ::core::ffi::c_long = LONG_MAX;
#[no_mangle]
pub static mut json_number_chars: *const ::core::ffi::c_char =
    b"0123456789.+-eE\0" as *const u8 as *const ::core::ffi::c_char;
#[no_mangle]
pub static mut json_hex_chars: *const ::core::ffi::c_char =
    b"0123456789abcdefABCDEF\0" as *const u8 as *const ::core::ffi::c_char;
#[inline]
unsafe extern "C" fn JC_OBJECT(mut jso: *mut json_object) -> *mut json_object_object {
    return jso as *mut ::core::ffi::c_void as *mut json_object_object;
}
#[inline]
unsafe extern "C" fn JC_OBJECT_C(mut jso: *const json_object) -> *const json_object_object {
    return jso as *const ::core::ffi::c_void as *const json_object_object;
}
#[inline]
unsafe extern "C" fn JC_ARRAY(mut jso: *mut json_object) -> *mut json_object_array {
    return jso as *mut ::core::ffi::c_void as *mut json_object_array;
}
#[inline]
unsafe extern "C" fn JC_ARRAY_C(mut jso: *const json_object) -> *const json_object_array {
    return jso as *const ::core::ffi::c_void as *const json_object_array;
}
#[inline]
unsafe extern "C" fn JC_BOOL(mut jso: *mut json_object) -> *mut json_object_boolean {
    return jso as *mut ::core::ffi::c_void as *mut json_object_boolean;
}
#[inline]
unsafe extern "C" fn JC_BOOL_C(mut jso: *const json_object) -> *const json_object_boolean {
    return jso as *const ::core::ffi::c_void as *const json_object_boolean;
}
#[inline]
unsafe extern "C" fn JC_DOUBLE(mut jso: *mut json_object) -> *mut json_object_double {
    return jso as *mut ::core::ffi::c_void as *mut json_object_double;
}
#[inline]
unsafe extern "C" fn JC_DOUBLE_C(mut jso: *const json_object) -> *const json_object_double {
    return jso as *const ::core::ffi::c_void as *const json_object_double;
}
#[inline]
unsafe extern "C" fn JC_INT(mut jso: *mut json_object) -> *mut json_object_int {
    return jso as *mut ::core::ffi::c_void as *mut json_object_int;
}
#[inline]
unsafe extern "C" fn JC_INT_C(mut jso: *const json_object) -> *const json_object_int {
    return jso as *const ::core::ffi::c_void as *const json_object_int;
}
#[inline]
unsafe extern "C" fn JC_STRING(mut jso: *mut json_object) -> *mut json_object_string {
    return jso as *mut ::core::ffi::c_void as *mut json_object_string;
}
#[inline]
unsafe extern "C" fn JC_STRING_C(mut jso: *const json_object) -> *const json_object_string {
    return jso as *const ::core::ffi::c_void as *const json_object_string;
}
#[inline]
unsafe extern "C" fn get_string_component_mutable(
    mut jso: *mut json_object,
) -> *mut ::core::ffi::c_char {
    if (*JC_STRING_C(jso)).len < 0 as ::core::ffi::c_long {
        return (*JC_STRING(jso)).c_string.pdata;
    }
    return &raw mut (*(JC_STRING
        as unsafe extern "C" fn(*mut json_object) -> *mut json_object_string)(
        jso
    ))
    .c_string
    .idata as *mut ::core::ffi::c_char;
}
#[inline]
unsafe extern "C" fn get_string_component(
    mut jso: *const json_object,
) -> *const ::core::ffi::c_char {
    return get_string_component_mutable(
        jso as *const ::core::ffi::c_void as uintptr_t as *mut ::core::ffi::c_void
            as *mut json_object,
    );
}
unsafe extern "C" fn json_escape_str(
    mut pb: *mut printbuf,
    mut str: *const ::core::ffi::c_char,
    mut len: size_t,
    mut flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut pos: size_t = 0 as size_t;
    let mut start_offset: size_t = 0 as size_t;
    let mut c: ::core::ffi::c_uchar = 0;
    while len != 0 {
        len = len.wrapping_sub(1);
        c = *str.offset(pos as isize) as ::core::ffi::c_uchar;
        match c as ::core::ffi::c_int {
            8 | 10 | 13 | 9 | 12 | 34 | 92 | 47 => {
                if flags & JSON_C_TO_STRING_NOSLASHESCAPE != 0
                    && c as ::core::ffi::c_int == '/' as i32
                {
                    pos = pos.wrapping_add(1);
                } else {
                    if pos > start_offset {
                        printbuf_memappend(
                            pb,
                            str.offset(start_offset as isize),
                            pos.wrapping_sub(start_offset) as ::core::ffi::c_int,
                        );
                    }
                    if c as ::core::ffi::c_int == '\u{8}' as i32 {
                        printbuf_memappend(
                            pb,
                            b"\\b\0" as *const u8 as *const ::core::ffi::c_char,
                            2 as ::core::ffi::c_int,
                        );
                    } else if c as ::core::ffi::c_int == '\n' as i32 {
                        printbuf_memappend(
                            pb,
                            b"\\n\0" as *const u8 as *const ::core::ffi::c_char,
                            2 as ::core::ffi::c_int,
                        );
                    } else if c as ::core::ffi::c_int == '\r' as i32 {
                        printbuf_memappend(
                            pb,
                            b"\\r\0" as *const u8 as *const ::core::ffi::c_char,
                            2 as ::core::ffi::c_int,
                        );
                    } else if c as ::core::ffi::c_int == '\t' as i32 {
                        printbuf_memappend(
                            pb,
                            b"\\t\0" as *const u8 as *const ::core::ffi::c_char,
                            2 as ::core::ffi::c_int,
                        );
                    } else if c as ::core::ffi::c_int == '\u{c}' as i32 {
                        printbuf_memappend(
                            pb,
                            b"\\f\0" as *const u8 as *const ::core::ffi::c_char,
                            2 as ::core::ffi::c_int,
                        );
                    } else if c as ::core::ffi::c_int == '"' as i32 {
                        printbuf_memappend(
                            pb,
                            b"\\\"\0" as *const u8 as *const ::core::ffi::c_char,
                            2 as ::core::ffi::c_int,
                        );
                    } else if c as ::core::ffi::c_int == '\\' as i32 {
                        printbuf_memappend(
                            pb,
                            b"\\\\\0" as *const u8 as *const ::core::ffi::c_char,
                            2 as ::core::ffi::c_int,
                        );
                    } else if c as ::core::ffi::c_int == '/' as i32 {
                        printbuf_memappend(
                            pb,
                            b"\\/\0" as *const u8 as *const ::core::ffi::c_char,
                            2 as ::core::ffi::c_int,
                        );
                    }
                    pos = pos.wrapping_add(1);
                    start_offset = pos;
                }
            }
            _ => {
                if (c as ::core::ffi::c_int) < ' ' as i32 {
                    let mut sbuf: [::core::ffi::c_char; 7] = [0; 7];
                    if pos > start_offset {
                        printbuf_memappend(
                            pb,
                            str.offset(start_offset as isize),
                            pos.wrapping_sub(start_offset) as ::core::ffi::c_int,
                        );
                    }
                    snprintf(
                        &raw mut sbuf as *mut ::core::ffi::c_char,
                        ::core::mem::size_of::<[::core::ffi::c_char; 7]>() as size_t,
                        b"\\u00%c%c\0" as *const u8 as *const ::core::ffi::c_char,
                        *json_hex_chars
                            .offset((c as ::core::ffi::c_int >> 4 as ::core::ffi::c_int) as isize)
                            as ::core::ffi::c_int,
                        *json_hex_chars
                            .offset((c as ::core::ffi::c_int & 0xf as ::core::ffi::c_int) as isize)
                            as ::core::ffi::c_int,
                    );
                    if (*pb).size - (*pb).bpos
                        > ::core::mem::size_of::<[::core::ffi::c_char; 7]>() as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int
                    {
                        memcpy(
                            (*pb).buf.offset((*pb).bpos as isize) as *mut ::core::ffi::c_void,
                            &raw mut sbuf as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
                            (::core::mem::size_of::<[::core::ffi::c_char; 7]>()
                                as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int) as size_t,
                        );
                        (*pb).bpos += ::core::mem::size_of::<[::core::ffi::c_char; 7]>()
                            as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int;
                        *(*pb).buf.offset((*pb).bpos as isize) = '\0' as i32 as ::core::ffi::c_char;
                    } else {
                        printbuf_memappend(
                            pb,
                            &raw mut sbuf as *mut ::core::ffi::c_char,
                            ::core::mem::size_of::<[::core::ffi::c_char; 7]>()
                                as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int,
                        );
                    }
                    pos = pos.wrapping_add(1);
                    start_offset = pos;
                } else {
                    pos = pos.wrapping_add(1);
                }
            }
        }
    }
    if pos > start_offset {
        printbuf_memappend(
            pb,
            str.offset(start_offset as isize),
            pos.wrapping_sub(start_offset) as ::core::ffi::c_int,
        );
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn json_object_get(mut jso: *mut json_object) -> *mut json_object {
    if jso.is_null() {
        return jso;
    }
    (*jso)._ref_count = (*jso)._ref_count.wrapping_add(1);
    return jso;
}
#[no_mangle]
pub unsafe extern "C" fn json_object_put(mut jso: *mut json_object) -> ::core::ffi::c_int {
    if jso.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    (*jso)._ref_count = (*jso)._ref_count.wrapping_sub(1);
    if (*jso)._ref_count > 0 as uint32_t {
        return 0 as ::core::ffi::c_int;
    }
    if (*jso)._user_delete.is_some() {
        (*jso)._user_delete.expect("non-null function pointer")(jso, (*jso)._userdata);
    }
    match (*jso).o_type as ::core::ffi::c_uint {
        4 => {
            json_object_object_delete(jso);
        }
        5 => {
            json_object_array_delete(jso);
        }
        6 => {
            json_object_string_delete(jso);
        }
        _ => {
            json_object_generic_delete(jso);
        }
    }
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn json_object_generic_delete(mut jso: *mut json_object) {
    printbuf_free((*jso)._pb);
    free(jso as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn json_object_new(
    mut o_type: json_type,
    mut alloc_size: size_t,
    mut to_json_string: Option<json_object_to_json_string_fn>,
) -> *mut json_object {
    let mut jso: *mut json_object = ::core::ptr::null_mut::<json_object>();
    jso = malloc(alloc_size) as *mut json_object;
    if jso.is_null() {
        return ::core::ptr::null_mut::<json_object>();
    }
    (*jso).o_type = o_type;
    (*jso)._ref_count = 1 as uint32_t;
    (*jso)._to_json_string = to_json_string;
    (*jso)._pb = ::core::ptr::null_mut::<printbuf>();
    (*jso)._user_delete = None;
    (*jso)._userdata = NULL;
    return jso;
}
#[no_mangle]
pub unsafe extern "C" fn json_object_is_type(
    mut jso: *const json_object,
    mut type_0: json_type,
) -> ::core::ffi::c_int {
    if jso.is_null() {
        return (type_0 as ::core::ffi::c_uint
            == json_type_null as ::core::ffi::c_int as ::core::ffi::c_uint)
            as ::core::ffi::c_int;
    }
    return ((*jso).o_type as ::core::ffi::c_uint == type_0 as ::core::ffi::c_uint)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn json_object_get_type(mut jso: *const json_object) -> json_type {
    if jso.is_null() {
        return json_type_null;
    }
    return (*jso).o_type;
}
#[no_mangle]
pub unsafe extern "C" fn json_object_get_userdata(
    mut jso: *mut json_object,
) -> *mut ::core::ffi::c_void {
    return if !jso.is_null() {
        (*jso)._userdata
    } else {
        NULL
    };
}
#[no_mangle]
pub unsafe extern "C" fn json_object_set_userdata(
    mut jso: *mut json_object,
    mut userdata: *mut ::core::ffi::c_void,
    mut user_delete: Option<json_object_delete_fn>,
) {
    if (*jso)._user_delete.is_some() {
        (*jso)._user_delete.expect("non-null function pointer")(
            jso as *mut json_object,
            (*jso)._userdata,
        );
    }
    (*jso)._userdata = userdata;
    (*jso)._user_delete = user_delete;
}
#[no_mangle]
pub unsafe extern "C" fn json_object_set_serializer(
    mut jso: *mut json_object,
    mut to_string_func: Option<json_object_to_json_string_fn>,
    mut userdata: *mut ::core::ffi::c_void,
    mut user_delete: Option<json_object_delete_fn>,
) {
    json_object_set_userdata(jso, userdata, user_delete);
    if to_string_func.is_none() {
        match (*jso).o_type as ::core::ffi::c_uint {
            0 => {
                (*jso)._to_json_string = None;
            }
            1 => {
                (*jso)._to_json_string = Some(
                    json_object_boolean_to_json_string
                        as unsafe extern "C" fn(
                            *mut json_object,
                            *mut printbuf,
                            ::core::ffi::c_int,
                            ::core::ffi::c_int,
                        ) -> ::core::ffi::c_int,
                );
            }
            2 => {
                (*jso)._to_json_string = Some(
                    json_object_double_to_json_string_default
                        as unsafe extern "C" fn(
                            *mut json_object,
                            *mut printbuf,
                            ::core::ffi::c_int,
                            ::core::ffi::c_int,
                        ) -> ::core::ffi::c_int,
                );
            }
            3 => {
                (*jso)._to_json_string = Some(
                    json_object_int_to_json_string
                        as unsafe extern "C" fn(
                            *mut json_object,
                            *mut printbuf,
                            ::core::ffi::c_int,
                            ::core::ffi::c_int,
                        ) -> ::core::ffi::c_int,
                );
            }
            4 => {
                (*jso)._to_json_string = Some(
                    json_object_object_to_json_string
                        as unsafe extern "C" fn(
                            *mut json_object,
                            *mut printbuf,
                            ::core::ffi::c_int,
                            ::core::ffi::c_int,
                        ) -> ::core::ffi::c_int,
                );
            }
            5 => {
                (*jso)._to_json_string = Some(
                    json_object_array_to_json_string
                        as unsafe extern "C" fn(
                            *mut json_object,
                            *mut printbuf,
                            ::core::ffi::c_int,
                            ::core::ffi::c_int,
                        ) -> ::core::ffi::c_int,
                );
            }
            6 => {
                (*jso)._to_json_string = Some(
                    json_object_string_to_json_string
                        as unsafe extern "C" fn(
                            *mut json_object,
                            *mut printbuf,
                            ::core::ffi::c_int,
                            ::core::ffi::c_int,
                        ) -> ::core::ffi::c_int,
                );
            }
            _ => {}
        }
        return;
    }
    (*jso)._to_json_string = to_string_func;
}
#[no_mangle]
pub unsafe extern "C" fn json_object_to_json_string_length(
    mut jso: *mut json_object,
    mut flags: ::core::ffi::c_int,
    mut length: *mut size_t,
) -> *const ::core::ffi::c_char {
    let mut r: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut s: size_t = 0 as size_t;
    if jso.is_null() {
        s = 4 as size_t;
        r = b"null\0" as *const u8 as *const ::core::ffi::c_char;
    } else if !(*jso)._pb.is_null() || {
        (*jso)._pb = printbuf_new();
        !(*jso)._pb.is_null()
    } {
        printbuf_reset((*jso)._pb);
        if (*jso)._to_json_string.expect("non-null function pointer")(
            jso,
            (*jso)._pb,
            0 as ::core::ffi::c_int,
            flags,
        ) >= 0 as ::core::ffi::c_int
        {
            s = (*(*jso)._pb).bpos as size_t;
            r = (*(*jso)._pb).buf;
        }
    }
    if !length.is_null() {
        *length = s;
    }
    return r;
}
#[no_mangle]
pub unsafe extern "C" fn json_object_to_json_string_ext(
    mut jso: *mut json_object,
    mut flags: ::core::ffi::c_int,
) -> *const ::core::ffi::c_char {
    return json_object_to_json_string_length(jso, flags, ::core::ptr::null_mut::<size_t>());
}
#[no_mangle]
pub unsafe extern "C" fn json_object_to_json_string(
    mut jso: *mut json_object,
) -> *const ::core::ffi::c_char {
    return json_object_to_json_string_ext(jso, JSON_C_TO_STRING_SPACED);
}
unsafe extern "C" fn indent(
    mut pb: *mut printbuf,
    mut level: ::core::ffi::c_int,
    mut flags: ::core::ffi::c_int,
) {
    if flags & JSON_C_TO_STRING_PRETTY != 0 {
        if flags & JSON_C_TO_STRING_PRETTY_TAB != 0 {
            printbuf_memset(pb, -(1 as ::core::ffi::c_int), '\t' as i32, level);
        } else {
            printbuf_memset(
                pb,
                -(1 as ::core::ffi::c_int),
                ' ' as i32,
                level * 2 as ::core::ffi::c_int,
            );
        }
    }
}
unsafe extern "C" fn json_object_object_to_json_string(
    mut jso: *mut json_object,
    mut pb: *mut printbuf,
    mut level: ::core::ffi::c_int,
    mut flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut had_children: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut iter: json_object_iter = json_object_iter {
        key: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        val: ::core::ptr::null_mut::<json_object>(),
        entry: ::core::ptr::null_mut::<lh_entry>(),
    };
    printbuf_memappend(
        pb,
        b"{\0" as *const u8 as *const ::core::ffi::c_char,
        (::core::mem::size_of::<[::core::ffi::c_char; 2]>() as usize).wrapping_sub(1 as usize)
            as ::core::ffi::c_int,
    );
    iter.entry = lh_table_head(json_object_get_object(jso)) as *mut lh_entry;
    while !if !iter.entry.is_null() {
        iter.key = lh_entry_k(iter.entry) as *mut ::core::ffi::c_char;
        iter.val = lh_entry_v(iter.entry) as *mut json_object as *mut json_object;
        iter.entry
    } else {
        ::core::ptr::null_mut::<lh_entry>()
    }
    .is_null()
    {
        if had_children != 0 {
            printbuf_memappend(
                pb,
                b",\0" as *const u8 as *const ::core::ffi::c_char,
                (::core::mem::size_of::<[::core::ffi::c_char; 2]>() as usize)
                    .wrapping_sub(1 as usize) as ::core::ffi::c_int,
            );
        }
        if flags & JSON_C_TO_STRING_PRETTY != 0 {
            printbuf_memappend(
                pb,
                b"\n\0" as *const u8 as *const ::core::ffi::c_char,
                (::core::mem::size_of::<[::core::ffi::c_char; 2]>() as usize)
                    .wrapping_sub(1 as usize) as ::core::ffi::c_int,
            );
        }
        had_children = 1 as ::core::ffi::c_int;
        if flags & JSON_C_TO_STRING_SPACED != 0 && flags & JSON_C_TO_STRING_PRETTY == 0 {
            printbuf_memappend(
                pb,
                b" \0" as *const u8 as *const ::core::ffi::c_char,
                (::core::mem::size_of::<[::core::ffi::c_char; 2]>() as usize)
                    .wrapping_sub(1 as usize) as ::core::ffi::c_int,
            );
        }
        indent(pb, level + 1 as ::core::ffi::c_int, flags);
        if flags & JSON_C_TO_STRING_COLOR != 0 {
            printbuf_memappend(
                pb,
                b"\x1B[0;34m\0" as *const u8 as *const ::core::ffi::c_char,
                (::core::mem::size_of::<[::core::ffi::c_char; 8]>() as usize)
                    .wrapping_sub(1 as usize) as ::core::ffi::c_int,
            );
        }
        printbuf_memappend(
            pb,
            b"\"\0" as *const u8 as *const ::core::ffi::c_char,
            (::core::mem::size_of::<[::core::ffi::c_char; 2]>() as usize).wrapping_sub(1 as usize)
                as ::core::ffi::c_int,
        );
        json_escape_str(pb, iter.key, strlen(iter.key), flags);
        printbuf_memappend(
            pb,
            b"\"\0" as *const u8 as *const ::core::ffi::c_char,
            (::core::mem::size_of::<[::core::ffi::c_char; 2]>() as usize).wrapping_sub(1 as usize)
                as ::core::ffi::c_int,
        );
        if flags & JSON_C_TO_STRING_COLOR != 0 {
            printbuf_memappend(
                pb,
                b"\x1B[0m\0" as *const u8 as *const ::core::ffi::c_char,
                (::core::mem::size_of::<[::core::ffi::c_char; 5]>() as usize)
                    .wrapping_sub(1 as usize) as ::core::ffi::c_int,
            );
        }
        if flags & JSON_C_TO_STRING_SPACED != 0 {
            printbuf_memappend(
                pb,
                b": \0" as *const u8 as *const ::core::ffi::c_char,
                (::core::mem::size_of::<[::core::ffi::c_char; 3]>() as usize)
                    .wrapping_sub(1 as usize) as ::core::ffi::c_int,
            );
        } else {
            printbuf_memappend(
                pb,
                b":\0" as *const u8 as *const ::core::ffi::c_char,
                (::core::mem::size_of::<[::core::ffi::c_char; 2]>() as usize)
                    .wrapping_sub(1 as usize) as ::core::ffi::c_int,
            );
        }
        if iter.val.is_null() {
            if flags & JSON_C_TO_STRING_COLOR != 0 {
                printbuf_memappend(
                    pb,
                    b"\x1B[0;35m\0" as *const u8 as *const ::core::ffi::c_char,
                    (::core::mem::size_of::<[::core::ffi::c_char; 8]>() as usize)
                        .wrapping_sub(1 as usize) as ::core::ffi::c_int,
                );
            }
            printbuf_memappend(
                pb,
                b"null\0" as *const u8 as *const ::core::ffi::c_char,
                (::core::mem::size_of::<[::core::ffi::c_char; 5]>() as usize)
                    .wrapping_sub(1 as usize) as ::core::ffi::c_int,
            );
            if flags & JSON_C_TO_STRING_COLOR != 0 {
                printbuf_memappend(
                    pb,
                    b"\x1B[0m\0" as *const u8 as *const ::core::ffi::c_char,
                    (::core::mem::size_of::<[::core::ffi::c_char; 5]>() as usize)
                        .wrapping_sub(1 as usize) as ::core::ffi::c_int,
                );
            }
        } else if (*iter.val)
            ._to_json_string
            .expect("non-null function pointer")(
            iter.val as *mut json_object,
            pb,
            level + 1 as ::core::ffi::c_int,
            flags,
        ) < 0 as ::core::ffi::c_int
        {
            return -(1 as ::core::ffi::c_int);
        }
        iter.entry = lh_entry_next(iter.entry) as *mut lh_entry;
    }
    if flags & JSON_C_TO_STRING_PRETTY != 0 && had_children != 0 {
        printbuf_memappend(
            pb,
            b"\n\0" as *const u8 as *const ::core::ffi::c_char,
            (::core::mem::size_of::<[::core::ffi::c_char; 2]>() as usize).wrapping_sub(1 as usize)
                as ::core::ffi::c_int,
        );
        indent(pb, level, flags);
    }
    if flags & JSON_C_TO_STRING_SPACED != 0 && flags & JSON_C_TO_STRING_PRETTY == 0 {
        return printbuf_memappend(
            pb,
            b" }\0" as *const u8 as *const ::core::ffi::c_char,
            (::core::mem::size_of::<[::core::ffi::c_char; 3]>() as usize).wrapping_sub(1 as usize)
                as ::core::ffi::c_int,
        );
    } else {
        return printbuf_memappend(
            pb,
            b"}\0" as *const u8 as *const ::core::ffi::c_char,
            (::core::mem::size_of::<[::core::ffi::c_char; 2]>() as usize).wrapping_sub(1 as usize)
                as ::core::ffi::c_int,
        );
    };
}
unsafe extern "C" fn json_object_lh_entry_free(mut ent: *mut lh_entry) {
    if lh_entry_k_is_constant(ent) == 0 {
        free(lh_entry_k(ent));
    }
    json_object_put(lh_entry_v(ent) as *mut json_object);
}
unsafe extern "C" fn json_object_object_delete(mut jso_base: *mut json_object) {
    lh_table_free((*JC_OBJECT(jso_base)).c_object);
    json_object_generic_delete(jso_base);
}
#[no_mangle]
pub unsafe extern "C" fn json_object_new_object() -> *mut json_object {
    let mut jso: *mut json_object_object = json_object_new(
        json_type_object,
        ::core::mem::size_of::<json_object_object>() as size_t,
        Some(
            json_object_object_to_json_string
                as unsafe extern "C" fn(
                    *mut json_object,
                    *mut printbuf,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
    ) as *mut json_object_object;
    if jso.is_null() {
        return ::core::ptr::null_mut::<json_object>();
    }
    (*jso).c_object = lh_kchar_table_new(
        JSON_OBJECT_DEF_HASH_ENTRIES,
        Some(json_object_lh_entry_free as unsafe extern "C" fn(*mut lh_entry) -> ()),
    );
    if (*jso).c_object.is_null() {
        json_object_generic_delete(&raw mut (*jso).base);
        *__errno_location() = ENOMEM;
        return ::core::ptr::null_mut::<json_object>();
    }
    return &raw mut (*jso).base;
}
#[no_mangle]
pub unsafe extern "C" fn json_object_get_object(mut jso: *const json_object) -> *mut lh_table {
    if jso.is_null() {
        return ::core::ptr::null_mut::<lh_table>();
    }
    match (*jso).o_type as ::core::ffi::c_uint {
        4 => return (*JC_OBJECT_C(jso)).c_object as *mut lh_table,
        _ => return ::core::ptr::null_mut::<lh_table>(),
    };
}
#[no_mangle]
pub unsafe extern "C" fn json_object_object_add_ex(
    mut jso: *mut json_object,
    key: *const ::core::ffi::c_char,
    val: *mut json_object,
    opts: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    let mut existing_value: *mut json_object = ::core::ptr::null_mut::<json_object>();
    let mut existing_entry: *mut lh_entry = ::core::ptr::null_mut::<lh_entry>();
    let mut hash: ::core::ffi::c_ulong = 0;
    hash = lh_get_hash(
        (*JC_OBJECT(jso)).c_object,
        key as *const ::core::ffi::c_void,
    );
    existing_entry = if opts & JSON_C_OBJECT_ADD_KEY_IS_NEW as ::core::ffi::c_uint != 0 {
        ::core::ptr::null_mut::<lh_entry>()
    } else {
        lh_table_lookup_entry_w_hash(
            (*JC_OBJECT(jso)).c_object,
            key as *const ::core::ffi::c_void,
            hash,
        )
    };
    if jso == val {
        return -(1 as ::core::ffi::c_int);
    }
    if existing_entry.is_null() {
        let k: *const ::core::ffi::c_void =
            if opts & JSON_C_OBJECT_ADD_CONSTANT_KEY as ::core::ffi::c_uint != 0 {
                key as *const ::core::ffi::c_void
            } else {
                strdup(key) as *const ::core::ffi::c_void
            };
        if k.is_null() {
            return -(1 as ::core::ffi::c_int);
        }
        return lh_table_insert_w_hash(
            (*JC_OBJECT(jso)).c_object,
            k,
            val as *const ::core::ffi::c_void,
            hash,
            opts,
        );
    }
    existing_value = lh_entry_v(existing_entry) as *mut json_object as *mut json_object;
    if !existing_value.is_null() {
        json_object_put(existing_value);
    }
    lh_entry_set_val(existing_entry, val as *mut ::core::ffi::c_void);
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn json_object_object_add(
    mut jso: *mut json_object,
    mut key: *const ::core::ffi::c_char,
    mut val: *mut json_object,
) -> ::core::ffi::c_int {
    return json_object_object_add_ex(jso, key, val, 0 as ::core::ffi::c_uint);
}
#[no_mangle]
pub unsafe extern "C" fn json_object_object_length(
    mut jso: *const json_object,
) -> ::core::ffi::c_int {
    return lh_table_length((*JC_OBJECT_C(jso)).c_object);
}
#[no_mangle]
pub unsafe extern "C" fn json_c_object_sizeof() -> size_t {
    return ::core::mem::size_of::<json_object>() as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn json_object_object_get(
    mut jso: *const json_object,
    mut key: *const ::core::ffi::c_char,
) -> *mut json_object {
    let mut result: *mut json_object = ::core::ptr::null_mut::<json_object>();
    json_object_object_get_ex(jso, key, &raw mut result);
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn json_object_object_get_ex(
    mut jso: *const json_object,
    mut key: *const ::core::ffi::c_char,
    mut value: *mut *mut json_object,
) -> json_bool {
    if !value.is_null() {
        *value = ::core::ptr::null_mut::<json_object>();
    }
    if jso.is_null() {
        return 0 as json_bool;
    }
    match (*jso).o_type as ::core::ffi::c_uint {
        4 => {
            return lh_table_lookup_ex(
                (*JC_OBJECT_C(jso)).c_object,
                key as *const ::core::ffi::c_void,
                value as *mut *mut ::core::ffi::c_void,
            );
        }
        _ => {
            if !value.is_null() {
                *value = ::core::ptr::null_mut::<json_object>();
            }
            return 0 as json_bool;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn json_object_object_del(
    mut jso: *mut json_object,
    mut key: *const ::core::ffi::c_char,
) {
    lh_table_delete(
        (*JC_OBJECT(jso)).c_object,
        key as *const ::core::ffi::c_void,
    );
}
unsafe extern "C" fn json_object_boolean_to_json_string(
    mut jso: *mut json_object,
    mut pb: *mut printbuf,
    mut level: ::core::ffi::c_int,
    mut flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;
    if flags & JSON_C_TO_STRING_COLOR != 0 {
        printbuf_memappend(
            pb,
            b"\x1B[0;35m\0" as *const u8 as *const ::core::ffi::c_char,
            (::core::mem::size_of::<[::core::ffi::c_char; 8]>() as usize).wrapping_sub(1 as usize)
                as ::core::ffi::c_int,
        );
    }
    if (*JC_BOOL(jso)).c_boolean != 0 {
        ret = printbuf_memappend(
            pb,
            b"true\0" as *const u8 as *const ::core::ffi::c_char,
            (::core::mem::size_of::<[::core::ffi::c_char; 5]>() as usize).wrapping_sub(1 as usize)
                as ::core::ffi::c_int,
        );
    } else {
        ret = printbuf_memappend(
            pb,
            b"false\0" as *const u8 as *const ::core::ffi::c_char,
            (::core::mem::size_of::<[::core::ffi::c_char; 6]>() as usize).wrapping_sub(1 as usize)
                as ::core::ffi::c_int,
        );
    }
    if ret > -(1 as ::core::ffi::c_int) && flags & JSON_C_TO_STRING_COLOR != 0 {
        return printbuf_memappend(
            pb,
            b"\x1B[0m\0" as *const u8 as *const ::core::ffi::c_char,
            (::core::mem::size_of::<[::core::ffi::c_char; 5]>() as usize).wrapping_sub(1 as usize)
                as ::core::ffi::c_int,
        );
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn json_object_new_boolean(mut b: json_bool) -> *mut json_object {
    let mut jso: *mut json_object_boolean = json_object_new(
        json_type_boolean,
        ::core::mem::size_of::<json_object_boolean>() as size_t,
        Some(
            json_object_boolean_to_json_string
                as unsafe extern "C" fn(
                    *mut json_object,
                    *mut printbuf,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
    ) as *mut json_object_boolean;
    if jso.is_null() {
        return ::core::ptr::null_mut::<json_object>();
    }
    (*jso).c_boolean = b;
    return &raw mut (*jso).base;
}
#[no_mangle]
pub unsafe extern "C" fn json_object_get_boolean(mut jso: *const json_object) -> json_bool {
    if jso.is_null() {
        return 0 as json_bool;
    }
    match (*jso).o_type as ::core::ffi::c_uint {
        1 => return (*JC_BOOL_C(jso)).c_boolean,
        3 => match (*JC_INT_C(jso)).cint_type as ::core::ffi::c_uint {
            0 => {
                return ((*JC_INT_C(jso)).cint.c_int64 != 0 as int64_t) as ::core::ffi::c_int;
            }
            1 => {
                return ((*JC_INT_C(jso)).cint.c_uint64 != 0 as uint64_t) as ::core::ffi::c_int;
            }
            _ => {
                json_abort(b"invalid cint_type\0" as *const u8 as *const ::core::ffi::c_char);
            }
        },
        2 => {
            return ((*JC_DOUBLE_C(jso)).c_double
                != 0 as ::core::ffi::c_int as ::core::ffi::c_double)
                as ::core::ffi::c_int;
        }
        6 => {
            return ((*JC_STRING_C(jso)).len != 0 as ::core::ffi::c_long) as ::core::ffi::c_int;
        }
        _ => return 0 as json_bool,
    };
}
#[no_mangle]
pub unsafe extern "C" fn json_object_set_boolean(
    mut jso: *mut json_object,
    mut new_value: json_bool,
) -> ::core::ffi::c_int {
    if jso.is_null()
        || (*jso).o_type as ::core::ffi::c_uint
            != json_type_boolean as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return 0 as ::core::ffi::c_int;
    }
    (*JC_BOOL(jso)).c_boolean = new_value;
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn json_object_int_to_json_string(
    mut jso: *mut json_object,
    mut pb: *mut printbuf,
    mut level: ::core::ffi::c_int,
    mut flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut sbuf: [::core::ffi::c_char; 21] = [0; 21];
    if (*JC_INT(jso)).cint_type as ::core::ffi::c_uint
        == json_object_int_type_int64 as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        snprintf(
            &raw mut sbuf as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 21]>() as size_t,
            b"%ld\0" as *const u8 as *const ::core::ffi::c_char,
            (*JC_INT(jso)).cint.c_int64,
        );
    } else {
        snprintf(
            &raw mut sbuf as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 21]>() as size_t,
            b"%lu\0" as *const u8 as *const ::core::ffi::c_char,
            (*JC_INT(jso)).cint.c_uint64,
        );
    }
    return printbuf_memappend(
        pb,
        &raw mut sbuf as *mut ::core::ffi::c_char,
        strlen(&raw mut sbuf as *mut ::core::ffi::c_char) as ::core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn json_object_new_int(mut i: int32_t) -> *mut json_object {
    return json_object_new_int64(i as int64_t);
}
#[no_mangle]
pub unsafe extern "C" fn json_object_get_int(mut jso: *const json_object) -> int32_t {
    let mut cint64: int64_t = 0 as int64_t;
    let mut cdouble: ::core::ffi::c_double = 0.;
    let mut o_type: json_type = json_type_null;
    if jso.is_null() {
        return 0 as int32_t;
    }
    o_type = (*jso).o_type;
    if o_type as ::core::ffi::c_uint == json_type_int as ::core::ffi::c_int as ::core::ffi::c_uint {
        let mut jsoint: *const json_object_int = JC_INT_C(jso);
        if (*jsoint).cint_type as ::core::ffi::c_uint
            == json_object_int_type_int64 as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            cint64 = (*jsoint).cint.c_int64;
        } else if (*jsoint).cint.c_uint64 >= INT64_MAX as uint64_t {
            cint64 = INT64_MAX as int64_t;
        } else {
            cint64 = (*jsoint).cint.c_uint64 as int64_t;
        }
    } else if o_type as ::core::ffi::c_uint
        == json_type_string as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        if json_parse_int64(get_string_component(jso), &raw mut cint64) != 0 as ::core::ffi::c_int {
            return 0 as int32_t;
        }
        o_type = json_type_int;
    }
    match o_type as ::core::ffi::c_uint {
        3 => {
            if cint64 <= INT32_MIN as int64_t {
                return INT32_MIN as int32_t;
            }
            if cint64 >= INT32_MAX as int64_t {
                return INT32_MAX as int32_t;
            }
            return cint64 as int32_t;
        }
        2 => {
            cdouble = (*JC_DOUBLE_C(jso)).c_double;
            if cdouble <= INT32_MIN as ::core::ffi::c_double {
                return INT32_MIN as int32_t;
            }
            if cdouble >= INT32_MAX as ::core::ffi::c_double {
                return INT32_MAX as int32_t;
            }
            return cdouble as int32_t;
        }
        1 => return (*JC_BOOL_C(jso)).c_boolean as int32_t,
        _ => return 0 as int32_t,
    };
}
#[no_mangle]
pub unsafe extern "C" fn json_object_set_int(
    mut jso: *mut json_object,
    mut new_value: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return json_object_set_int64(jso, new_value as int64_t);
}
#[no_mangle]
pub unsafe extern "C" fn json_object_new_int64(mut i: int64_t) -> *mut json_object {
    let mut jso: *mut json_object_int = json_object_new(
        json_type_int,
        ::core::mem::size_of::<json_object_int>() as size_t,
        Some(
            json_object_int_to_json_string
                as unsafe extern "C" fn(
                    *mut json_object,
                    *mut printbuf,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
    ) as *mut json_object_int;
    if jso.is_null() {
        return ::core::ptr::null_mut::<json_object>();
    }
    (*jso).cint.c_int64 = i;
    (*jso).cint_type = json_object_int_type_int64;
    return &raw mut (*jso).base;
}
#[no_mangle]
pub unsafe extern "C" fn json_object_new_uint64(mut i: uint64_t) -> *mut json_object {
    let mut jso: *mut json_object_int = json_object_new(
        json_type_int,
        ::core::mem::size_of::<json_object_int>() as size_t,
        Some(
            json_object_int_to_json_string
                as unsafe extern "C" fn(
                    *mut json_object,
                    *mut printbuf,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
    ) as *mut json_object_int;
    if jso.is_null() {
        return ::core::ptr::null_mut::<json_object>();
    }
    (*jso).cint.c_uint64 = i;
    (*jso).cint_type = json_object_int_type_uint64;
    return &raw mut (*jso).base;
}
#[no_mangle]
pub unsafe extern "C" fn json_object_get_int64(mut jso: *const json_object) -> int64_t {
    let mut cint: int64_t = 0;
    if jso.is_null() {
        return 0 as int64_t;
    }
    match (*jso).o_type as ::core::ffi::c_uint {
        3 => {
            let mut jsoint: *const json_object_int = JC_INT_C(jso);
            match (*jsoint).cint_type as ::core::ffi::c_uint {
                0 => return (*jsoint).cint.c_int64,
                1 => {
                    if (*jsoint).cint.c_uint64 >= INT64_MAX as uint64_t {
                        return INT64_MAX as int64_t;
                    }
                    return (*jsoint).cint.c_uint64 as int64_t;
                }
                _ => {
                    json_abort(b"invalid cint_type\0" as *const u8 as *const ::core::ffi::c_char);
                }
            }
        }
        2 => {
            if (*JC_DOUBLE_C(jso)).c_double >= INT64_MAX as ::core::ffi::c_double {
                return INT64_MAX as int64_t;
            }
            if (*JC_DOUBLE_C(jso)).c_double <= INT64_MIN as ::core::ffi::c_double {
                return INT64_MIN as int64_t;
            }
            return (*JC_DOUBLE_C(jso)).c_double as int64_t;
        }
        1 => return (*JC_BOOL_C(jso)).c_boolean as int64_t,
        6 => {
            if json_parse_int64(get_string_component(jso), &raw mut cint) == 0 as ::core::ffi::c_int
            {
                return cint;
            }
        }
        _ => {}
    }
    return 0 as int64_t;
}
#[no_mangle]
pub unsafe extern "C" fn json_object_get_uint64(mut jso: *const json_object) -> uint64_t {
    let mut cuint: uint64_t = 0;
    if jso.is_null() {
        return 0 as uint64_t;
    }
    match (*jso).o_type as ::core::ffi::c_uint {
        3 => {
            let mut jsoint: *const json_object_int = JC_INT_C(jso);
            match (*jsoint).cint_type as ::core::ffi::c_uint {
                0 => {
                    if (*jsoint).cint.c_int64 < 0 as int64_t {
                        return 0 as uint64_t;
                    }
                    return (*jsoint).cint.c_int64 as uint64_t;
                }
                1 => return (*jsoint).cint.c_uint64,
                _ => {
                    json_abort(b"invalid cint_type\0" as *const u8 as *const ::core::ffi::c_char);
                }
            }
        }
        2 => {
            if (*JC_DOUBLE_C(jso)).c_double >= UINT64_MAX as ::core::ffi::c_double {
                return UINT64_MAX as uint64_t;
            }
            if (*JC_DOUBLE_C(jso)).c_double < 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                return 0 as uint64_t;
            }
            return (*JC_DOUBLE_C(jso)).c_double as uint64_t;
        }
        1 => return (*JC_BOOL_C(jso)).c_boolean as uint64_t,
        6 => {
            if json_parse_uint64(get_string_component(jso), &raw mut cuint)
                == 0 as ::core::ffi::c_int
            {
                return cuint;
            }
        }
        _ => {}
    }
    return 0 as uint64_t;
}
#[no_mangle]
pub unsafe extern "C" fn json_object_set_int64(
    mut jso: *mut json_object,
    mut new_value: int64_t,
) -> ::core::ffi::c_int {
    if jso.is_null()
        || (*jso).o_type as ::core::ffi::c_uint
            != json_type_int as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return 0 as ::core::ffi::c_int;
    }
    (*JC_INT(jso)).cint.c_int64 = new_value;
    (*JC_INT(jso)).cint_type = json_object_int_type_int64;
    return 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn json_object_set_uint64(
    mut jso: *mut json_object,
    mut new_value: uint64_t,
) -> ::core::ffi::c_int {
    if jso.is_null()
        || (*jso).o_type as ::core::ffi::c_uint
            != json_type_int as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return 0 as ::core::ffi::c_int;
    }
    (*JC_INT(jso)).cint.c_uint64 = new_value;
    (*JC_INT(jso)).cint_type = json_object_int_type_uint64;
    return 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn json_object_int_inc(
    mut jso: *mut json_object,
    mut val: int64_t,
) -> ::core::ffi::c_int {
    let mut jsoint: *mut json_object_int = ::core::ptr::null_mut::<json_object_int>();
    if jso.is_null()
        || (*jso).o_type as ::core::ffi::c_uint
            != json_type_int as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return 0 as ::core::ffi::c_int;
    }
    jsoint = JC_INT(jso);
    match (*jsoint).cint_type as ::core::ffi::c_uint {
        0 => {
            if val > 0 as int64_t && (*jsoint).cint.c_int64 > INT64_MAX as int64_t - val {
                (*jsoint).cint.c_uint64 =
                    ((*jsoint).cint.c_int64 as uint64_t).wrapping_add(val as uint64_t);
                (*jsoint).cint_type = json_object_int_type_uint64;
            } else if val < 0 as int64_t && (*jsoint).cint.c_int64 < INT64_MIN as int64_t - val {
                (*jsoint).cint.c_int64 = INT64_MIN as int64_t;
            } else {
                (*jsoint).cint.c_int64 = ((*jsoint).cint.c_int64 as ::core::ffi::c_long
                    + val as ::core::ffi::c_long)
                    as int64_t;
            }
            return 1 as ::core::ffi::c_int;
        }
        1 => {
            if val > 0 as int64_t
                && (*jsoint).cint.c_uint64 > (UINT64_MAX as uint64_t).wrapping_sub(val as uint64_t)
            {
                (*jsoint).cint.c_uint64 = UINT64_MAX as uint64_t;
            } else if val < 0 as int64_t && (*jsoint).cint.c_uint64 < -val as uint64_t {
                (*jsoint).cint.c_int64 = (*jsoint).cint.c_uint64 as int64_t + val;
                (*jsoint).cint_type = json_object_int_type_int64;
            } else if val < 0 as int64_t && (*jsoint).cint.c_uint64 >= -val as uint64_t {
                (*jsoint).cint.c_uint64 = ((*jsoint).cint.c_uint64 as ::core::ffi::c_ulong)
                    .wrapping_sub(-val as uint64_t as ::core::ffi::c_ulong)
                    as uint64_t as uint64_t;
            } else {
                (*jsoint).cint.c_uint64 = ((*jsoint).cint.c_uint64 as ::core::ffi::c_ulong)
                    .wrapping_add(val as ::core::ffi::c_ulong)
                    as uint64_t as uint64_t;
            }
            return 1 as ::core::ffi::c_int;
        }
        _ => {
            json_abort(b"invalid cint_type\0" as *const u8 as *const ::core::ffi::c_char);
        }
    };
}
#[thread_local]
static mut tls_serialization_float_format: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
static mut global_serialization_float_format: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
#[no_mangle]
pub unsafe extern "C" fn json_c_set_serialization_double_format(
    mut double_format: *const ::core::ffi::c_char,
    mut global_or_thread: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if global_or_thread == JSON_C_OPTION_GLOBAL {
        if !tls_serialization_float_format.is_null() {
            free(tls_serialization_float_format as *mut ::core::ffi::c_void);
            tls_serialization_float_format = ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        if !global_serialization_float_format.is_null() {
            free(global_serialization_float_format as *mut ::core::ffi::c_void);
        }
        if !double_format.is_null() {
            let mut p: *mut ::core::ffi::c_char = strdup(double_format);
            if p.is_null() {
                _json_c_set_last_err(
                    b"json_c_set_serialization_double_format: out of memory\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                return -(1 as ::core::ffi::c_int);
            }
            global_serialization_float_format = p;
        } else {
            global_serialization_float_format = ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
    } else if global_or_thread == JSON_C_OPTION_THREAD {
        if !tls_serialization_float_format.is_null() {
            free(tls_serialization_float_format as *mut ::core::ffi::c_void);
            tls_serialization_float_format = ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        if !double_format.is_null() {
            let mut p_0: *mut ::core::ffi::c_char = strdup(double_format);
            if p_0.is_null() {
                _json_c_set_last_err(
                    b"json_c_set_serialization_double_format: out of memory\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                return -(1 as ::core::ffi::c_int);
            }
            tls_serialization_float_format = p_0;
        } else {
            tls_serialization_float_format = ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
    } else {
        _json_c_set_last_err(
            b"json_c_set_serialization_double_format: invalid global_or_thread value: %d\n\0"
                as *const u8 as *const ::core::ffi::c_char,
            global_or_thread,
        );
        return -(1 as ::core::ffi::c_int);
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn json_object_double_to_json_string_format(
    mut jso: *mut json_object,
    mut pb: *mut printbuf,
    mut level: ::core::ffi::c_int,
    mut flags: ::core::ffi::c_int,
    mut format: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut jsodbl: *mut json_object_double = JC_DOUBLE(jso);
    let mut buf: [::core::ffi::c_char; 128] = [0; 128];
    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut q: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut size: ::core::ffi::c_int = 0;
    if (*jsodbl).c_double.is_nan() as i32 != 0 {
        size = snprintf(
            &raw mut buf as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 128]>() as size_t,
            b"NaN\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else if if (*jsodbl).c_double.is_infinite() {
        if (*jsodbl).c_double.is_sign_positive() {
            1
        } else {
            -1
        }
    } else {
        0
    } != 0
    {
        if (*jsodbl).c_double > 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            size = snprintf(
                &raw mut buf as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 128]>() as size_t,
                b"Infinity\0" as *const u8 as *const ::core::ffi::c_char,
            );
        } else {
            size = snprintf(
                &raw mut buf as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 128]>() as size_t,
                b"-Infinity\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    } else {
        let mut std_format: *const ::core::ffi::c_char =
            b"%.17g\0" as *const u8 as *const ::core::ffi::c_char;
        let mut format_drops_decimals: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut looks_numeric: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        if format.is_null() {
            if !tls_serialization_float_format.is_null() {
                format = tls_serialization_float_format;
            } else if !global_serialization_float_format.is_null() {
                format = global_serialization_float_format;
            } else {
                format = std_format;
            }
        }
        size = snprintf(
            &raw mut buf as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 128]>() as size_t,
            format,
            (*jsodbl).c_double,
        );
        if size < 0 as ::core::ffi::c_int {
            return -(1 as ::core::ffi::c_int);
        }
        p = strchr(&raw mut buf as *mut ::core::ffi::c_char, ',' as i32);
        if !p.is_null() {
            *p = '.' as i32 as ::core::ffi::c_char;
        } else {
            p = strchr(&raw mut buf as *mut ::core::ffi::c_char, '.' as i32);
        }
        if format == std_format
            || strstr(format, b".0f\0" as *const u8 as *const ::core::ffi::c_char).is_null()
        {
            format_drops_decimals = 1 as ::core::ffi::c_int;
        }
        looks_numeric = (buf[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int >= '0' as i32
            && buf[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int <= '9' as i32
            || size > 1 as ::core::ffi::c_int
                && buf[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int == '-' as i32
                && (buf[1 as ::core::ffi::c_int as usize] as ::core::ffi::c_int >= '0' as i32
                    && buf[1 as ::core::ffi::c_int as usize] as ::core::ffi::c_int <= '9' as i32))
            as ::core::ffi::c_int;
        if size
            < ::core::mem::size_of::<[::core::ffi::c_char; 128]>() as ::core::ffi::c_int
                - 2 as ::core::ffi::c_int
            && looks_numeric != 0
            && p.is_null()
            && strchr(&raw mut buf as *mut ::core::ffi::c_char, 'e' as i32).is_null()
            && format_drops_decimals != 0
        {
            strcat(
                &raw mut buf as *mut ::core::ffi::c_char,
                b".0\0" as *const u8 as *const ::core::ffi::c_char,
            );
            size += 2 as ::core::ffi::c_int;
        }
        if !p.is_null() && flags & JSON_C_TO_STRING_NOZERO != 0 {
            p = p.offset(1);
            q = p;
            while *q != 0 {
                if *q as ::core::ffi::c_int != '0' as i32 {
                    p = q;
                }
                q = q.offset(1);
            }
            if *p as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                p = p.offset(1);
                *p = 0 as ::core::ffi::c_char;
            }
            size = p.offset_from(&raw mut buf as *mut ::core::ffi::c_char) as ::core::ffi::c_long
                as ::core::ffi::c_int;
        }
    }
    if size < 0 as ::core::ffi::c_int {
        return -(1 as ::core::ffi::c_int);
    }
    if size >= ::core::mem::size_of::<[::core::ffi::c_char; 128]>() as ::core::ffi::c_int {
        size = (::core::mem::size_of::<[::core::ffi::c_char; 128]>() as usize)
            .wrapping_sub(1 as usize) as ::core::ffi::c_int;
    }
    printbuf_memappend(pb, &raw mut buf as *mut ::core::ffi::c_char, size);
    return size;
}
unsafe extern "C" fn json_object_double_to_json_string_default(
    mut jso: *mut json_object,
    mut pb: *mut printbuf,
    mut level: ::core::ffi::c_int,
    mut flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return json_object_double_to_json_string_format(
        jso,
        pb,
        level,
        flags,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn json_object_double_to_json_string(
    mut jso: *mut json_object,
    mut pb: *mut printbuf,
    mut level: ::core::ffi::c_int,
    mut flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return json_object_double_to_json_string_format(
        jso,
        pb,
        level,
        flags,
        (*jso)._userdata as *const ::core::ffi::c_char,
    );
}
#[no_mangle]
pub unsafe extern "C" fn json_object_new_double(mut d: ::core::ffi::c_double) -> *mut json_object {
    let mut jso: *mut json_object_double = json_object_new(
        json_type_double,
        ::core::mem::size_of::<json_object_double>() as size_t,
        Some(
            json_object_double_to_json_string
                as unsafe extern "C" fn(
                    *mut json_object,
                    *mut printbuf,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
    ) as *mut json_object_double;
    if jso.is_null() {
        return ::core::ptr::null_mut::<json_object>();
    }
    (*jso).base._to_json_string = Some(
        json_object_double_to_json_string_default
            as unsafe extern "C" fn(
                *mut json_object,
                *mut printbuf,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
    );
    (*jso).c_double = d;
    return &raw mut (*jso).base;
}
#[no_mangle]
pub unsafe extern "C" fn json_object_new_double_s(
    mut d: ::core::ffi::c_double,
    mut ds: *const ::core::ffi::c_char,
) -> *mut json_object {
    let mut new_ds: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut jso: *mut json_object = json_object_new_double(d);
    if jso.is_null() {
        return ::core::ptr::null_mut::<json_object>();
    }
    new_ds = strdup(ds);
    if new_ds.is_null() {
        json_object_generic_delete(jso);
        *__errno_location() = ENOMEM;
        return ::core::ptr::null_mut::<json_object>();
    }
    json_object_set_serializer(
        jso as *mut json_object,
        Some(
            _json_object_userdata_to_json_string
                as unsafe extern "C" fn(
                    *mut json_object,
                    *mut printbuf,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        new_ds as *mut ::core::ffi::c_void,
        Some(
            json_object_free_userdata
                as unsafe extern "C" fn(*mut json_object, *mut ::core::ffi::c_void) -> (),
        ),
    );
    return jso;
}
unsafe extern "C" fn _json_object_userdata_to_json_string(
    mut jso: *mut json_object,
    mut pb: *mut printbuf,
    mut level: ::core::ffi::c_int,
    mut flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return json_object_userdata_to_json_string(jso, pb, level, flags);
}
#[no_mangle]
pub unsafe extern "C" fn json_object_userdata_to_json_string(
    mut jso: *mut json_object,
    mut pb: *mut printbuf,
    mut level: ::core::ffi::c_int,
    mut flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut userdata_len: ::core::ffi::c_int =
        strlen((*jso)._userdata as *const ::core::ffi::c_char) as ::core::ffi::c_int;
    printbuf_memappend(
        pb,
        (*jso)._userdata as *const ::core::ffi::c_char,
        userdata_len,
    );
    return userdata_len;
}
#[no_mangle]
pub unsafe extern "C" fn json_object_free_userdata(
    mut jso: *mut json_object,
    mut userdata: *mut ::core::ffi::c_void,
) {
    free(userdata);
}
#[no_mangle]
pub unsafe extern "C" fn json_object_get_double(
    mut jso: *const json_object,
) -> ::core::ffi::c_double {
    let mut cdouble: ::core::ffi::c_double = 0.;
    let mut errPtr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if jso.is_null() {
        return 0.0f64;
    }
    match (*jso).o_type as ::core::ffi::c_uint {
        2 => return (*JC_DOUBLE_C(jso)).c_double,
        3 => match (*JC_INT_C(jso)).cint_type as ::core::ffi::c_uint {
            0 => return (*JC_INT_C(jso)).cint.c_int64 as ::core::ffi::c_double,
            1 => return (*JC_INT_C(jso)).cint.c_uint64 as ::core::ffi::c_double,
            _ => {
                json_abort(b"invalid cint_type\0" as *const u8 as *const ::core::ffi::c_char);
            }
        },
        1 => return (*JC_BOOL_C(jso)).c_boolean as ::core::ffi::c_double,
        6 => {
            *__errno_location() = 0 as ::core::ffi::c_int;
            cdouble = strtod(get_string_component(jso), &raw mut errPtr);
            if errPtr == get_string_component(jso) as *mut ::core::ffi::c_char {
                *__errno_location() = EINVAL;
                return 0.0f64;
            }
            if *errPtr as ::core::ffi::c_int != '\0' as i32 {
                *__errno_location() = EINVAL;
                return 0.0f64;
            }
            if (::core::f64::INFINITY == cdouble || -::core::f64::INFINITY == cdouble)
                && ERANGE == *__errno_location()
            {
                cdouble = 0.0f64;
            }
            return cdouble;
        }
        _ => {
            *__errno_location() = EINVAL;
            return 0.0f64;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn json_object_set_double(
    mut jso: *mut json_object,
    mut new_value: ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    if jso.is_null()
        || (*jso).o_type as ::core::ffi::c_uint
            != json_type_double as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return 0 as ::core::ffi::c_int;
    }
    (*JC_DOUBLE(jso)).c_double = new_value;
    if (*jso)._to_json_string
        == Some(
            _json_object_userdata_to_json_string
                as unsafe extern "C" fn(
                    *mut json_object,
                    *mut printbuf,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        )
    {
        json_object_set_serializer(jso as *mut json_object, None, NULL, None);
    }
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn json_object_string_to_json_string(
    mut jso: *mut json_object,
    mut pb: *mut printbuf,
    mut level: ::core::ffi::c_int,
    mut flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut len: ssize_t = (*JC_STRING(jso)).len;
    if flags & JSON_C_TO_STRING_COLOR != 0 {
        printbuf_memappend(
            pb,
            b"\x1B[0;32m\0" as *const u8 as *const ::core::ffi::c_char,
            (::core::mem::size_of::<[::core::ffi::c_char; 8]>() as usize).wrapping_sub(1 as usize)
                as ::core::ffi::c_int,
        );
    }
    printbuf_memappend(
        pb,
        b"\"\0" as *const u8 as *const ::core::ffi::c_char,
        (::core::mem::size_of::<[::core::ffi::c_char; 2]>() as usize).wrapping_sub(1 as usize)
            as ::core::ffi::c_int,
    );
    json_escape_str(
        pb,
        get_string_component(jso),
        (if len < 0 as ::core::ffi::c_long {
            -len
        } else {
            len as ::core::ffi::c_long
        }) as size_t,
        flags,
    );
    printbuf_memappend(
        pb,
        b"\"\0" as *const u8 as *const ::core::ffi::c_char,
        (::core::mem::size_of::<[::core::ffi::c_char; 2]>() as usize).wrapping_sub(1 as usize)
            as ::core::ffi::c_int,
    );
    if flags & JSON_C_TO_STRING_COLOR != 0 {
        printbuf_memappend(
            pb,
            b"\x1B[0m\0" as *const u8 as *const ::core::ffi::c_char,
            (::core::mem::size_of::<[::core::ffi::c_char; 5]>() as usize).wrapping_sub(1 as usize)
                as ::core::ffi::c_int,
        );
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn json_object_string_delete(mut jso: *mut json_object) {
    if (*JC_STRING(jso)).len < 0 as ::core::ffi::c_long {
        free((*JC_STRING(jso)).c_string.pdata as *mut ::core::ffi::c_void);
    }
    json_object_generic_delete(jso);
}
unsafe extern "C" fn _json_object_new_string(
    mut s: *const ::core::ffi::c_char,
    len: size_t,
) -> *mut json_object {
    let mut objsize: size_t = 0;
    let mut jso: *mut json_object_string = ::core::ptr::null_mut::<json_object_string>();
    if len
        > (SSIZE_T_MAX as usize)
            .wrapping_sub(
                (::core::mem::size_of::<json_object_string>() as usize)
                    .wrapping_sub(::core::mem::size_of::<C2RustUnnamed>() as usize),
            )
            .wrapping_sub(1 as usize)
    {
        return ::core::ptr::null_mut::<json_object>();
    }
    objsize = (::core::mem::size_of::<json_object_string>() as usize)
        .wrapping_sub(::core::mem::size_of::<C2RustUnnamed>() as usize)
        .wrapping_add(len as usize)
        .wrapping_add(1 as usize) as size_t;
    if len < ::core::mem::size_of::<*mut ::core::ffi::c_void>() as usize {
        objsize = (objsize as ::core::ffi::c_ulong).wrapping_add(
            (::core::mem::size_of::<*mut ::core::ffi::c_void>() as usize).wrapping_sub(len as usize)
                as ::core::ffi::c_ulong,
        ) as size_t as size_t;
    }
    jso = json_object_new(
        json_type_string,
        objsize,
        Some(
            json_object_string_to_json_string
                as unsafe extern "C" fn(
                    *mut json_object,
                    *mut printbuf,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
    ) as *mut json_object_string;
    if jso.is_null() {
        return ::core::ptr::null_mut::<json_object>();
    }
    (*jso).len = len as ssize_t;
    memcpy(
        &raw mut (*jso).c_string.idata as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        s as *const ::core::ffi::c_void,
        len,
    );
    *(&raw mut (*jso).c_string.idata as *mut ::core::ffi::c_char).offset(len as isize) =
        '\0' as i32 as ::core::ffi::c_char;
    return &raw mut (*jso).base;
}
#[no_mangle]
pub unsafe extern "C" fn json_object_new_string(
    mut s: *const ::core::ffi::c_char,
) -> *mut json_object {
    return _json_object_new_string(s, strlen(s));
}
#[no_mangle]
pub unsafe extern "C" fn json_object_new_string_len(
    mut s: *const ::core::ffi::c_char,
    len: ::core::ffi::c_int,
) -> *mut json_object {
    return _json_object_new_string(s, len as size_t);
}
#[no_mangle]
pub unsafe extern "C" fn json_object_get_string(
    mut jso: *mut json_object,
) -> *const ::core::ffi::c_char {
    if jso.is_null() {
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    match (*jso).o_type as ::core::ffi::c_uint {
        6 => return get_string_component(jso),
        _ => return json_object_to_json_string(jso),
    };
}
#[inline]
unsafe extern "C" fn _json_object_get_string_len(mut jso: *const json_object_string) -> ssize_t {
    let mut len: ssize_t = 0;
    len = (*jso).len;
    return if len < 0 as ::core::ffi::c_long {
        -len
    } else {
        len
    };
}
#[no_mangle]
pub unsafe extern "C" fn json_object_get_string_len(
    mut jso: *const json_object,
) -> ::core::ffi::c_int {
    if jso.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    match (*jso).o_type as ::core::ffi::c_uint {
        6 => return _json_object_get_string_len(JC_STRING_C(jso)) as ::core::ffi::c_int,
        _ => return 0 as ::core::ffi::c_int,
    };
}
unsafe extern "C" fn _json_object_set_string_len(
    mut jso: *mut json_object,
    mut s: *const ::core::ffi::c_char,
    mut len: size_t,
) -> ::core::ffi::c_int {
    let mut dstbuf: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut curlen: ssize_t = 0;
    let mut newlen: ssize_t = 0;
    if jso.is_null()
        || (*jso).o_type as ::core::ffi::c_uint
            != json_type_string as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return 0 as ::core::ffi::c_int;
    }
    if len >= (INT_MAX - 1 as ::core::ffi::c_int) as size_t {
        return 0 as ::core::ffi::c_int;
    }
    curlen = (*JC_STRING(jso as *mut json_object)).len;
    if curlen < 0 as ::core::ffi::c_long {
        if len == 0 as size_t {
            free((*JC_STRING(jso as *mut json_object)).c_string.pdata as *mut ::core::ffi::c_void);
            curlen = 0 as ssize_t;
            (*JC_STRING(jso as *mut json_object)).len = curlen;
        } else {
            curlen = -curlen;
        }
    }
    newlen = len as ssize_t;
    dstbuf = get_string_component_mutable(jso as *mut json_object);
    if len as ssize_t > curlen {
        dstbuf = malloc(len.wrapping_add(1 as size_t)) as *mut ::core::ffi::c_char;
        if dstbuf.is_null() {
            return 0 as ::core::ffi::c_int;
        }
        if (*JC_STRING(jso as *mut json_object)).len < 0 as ::core::ffi::c_long {
            free((*JC_STRING(jso as *mut json_object)).c_string.pdata as *mut ::core::ffi::c_void);
        }
        let ref mut fresh0 = (*JC_STRING(jso as *mut json_object)).c_string.pdata;
        *fresh0 = dstbuf;
        newlen = -(len as ssize_t);
    } else if (*JC_STRING(jso as *mut json_object)).len < 0 as ::core::ffi::c_long {
        newlen = -(len as ssize_t);
    }
    memcpy(
        dstbuf as *mut ::core::ffi::c_void,
        s as *const ::core::ffi::c_void,
        len,
    );
    *dstbuf.offset(len as isize) = '\0' as i32 as ::core::ffi::c_char;
    (*JC_STRING(jso as *mut json_object)).len = newlen;
    return 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn json_object_set_string(
    mut jso: *mut json_object,
    mut s: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    return _json_object_set_string_len(jso, s, strlen(s));
}
#[no_mangle]
pub unsafe extern "C" fn json_object_set_string_len(
    mut jso: *mut json_object,
    mut s: *const ::core::ffi::c_char,
    mut len: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return _json_object_set_string_len(jso, s, len as size_t);
}
unsafe extern "C" fn json_object_array_to_json_string(
    mut jso: *mut json_object,
    mut pb: *mut printbuf,
    mut level: ::core::ffi::c_int,
    mut flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut had_children: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut ii: size_t = 0;
    printbuf_memappend(
        pb,
        b"[\0" as *const u8 as *const ::core::ffi::c_char,
        (::core::mem::size_of::<[::core::ffi::c_char; 2]>() as usize).wrapping_sub(1 as usize)
            as ::core::ffi::c_int,
    );
    ii = 0 as size_t;
    while ii < json_object_array_length(jso) {
        let mut val: *mut json_object = ::core::ptr::null_mut::<json_object>();
        if had_children != 0 {
            printbuf_memappend(
                pb,
                b",\0" as *const u8 as *const ::core::ffi::c_char,
                (::core::mem::size_of::<[::core::ffi::c_char; 2]>() as usize)
                    .wrapping_sub(1 as usize) as ::core::ffi::c_int,
            );
        }
        if flags & JSON_C_TO_STRING_PRETTY != 0 {
            printbuf_memappend(
                pb,
                b"\n\0" as *const u8 as *const ::core::ffi::c_char,
                (::core::mem::size_of::<[::core::ffi::c_char; 2]>() as usize)
                    .wrapping_sub(1 as usize) as ::core::ffi::c_int,
            );
        }
        had_children = 1 as ::core::ffi::c_int;
        if flags & JSON_C_TO_STRING_SPACED != 0 && flags & JSON_C_TO_STRING_PRETTY == 0 {
            printbuf_memappend(
                pb,
                b" \0" as *const u8 as *const ::core::ffi::c_char,
                (::core::mem::size_of::<[::core::ffi::c_char; 2]>() as usize)
                    .wrapping_sub(1 as usize) as ::core::ffi::c_int,
            );
        }
        indent(pb, level + 1 as ::core::ffi::c_int, flags);
        val = json_object_array_get_idx(jso, ii);
        if val.is_null() {
            if flags & JSON_C_TO_STRING_COLOR != 0 {
                printbuf_memappend(
                    pb,
                    b"\x1B[0;35m\0" as *const u8 as *const ::core::ffi::c_char,
                    (::core::mem::size_of::<[::core::ffi::c_char; 8]>() as usize)
                        .wrapping_sub(1 as usize) as ::core::ffi::c_int,
                );
            }
            printbuf_memappend(
                pb,
                b"null\0" as *const u8 as *const ::core::ffi::c_char,
                (::core::mem::size_of::<[::core::ffi::c_char; 5]>() as usize)
                    .wrapping_sub(1 as usize) as ::core::ffi::c_int,
            );
            if flags & JSON_C_TO_STRING_COLOR != 0 {
                printbuf_memappend(
                    pb,
                    b"\x1B[0m\0" as *const u8 as *const ::core::ffi::c_char,
                    (::core::mem::size_of::<[::core::ffi::c_char; 5]>() as usize)
                        .wrapping_sub(1 as usize) as ::core::ffi::c_int,
                );
            }
        } else if (*val)._to_json_string.expect("non-null function pointer")(
            val,
            pb,
            level + 1 as ::core::ffi::c_int,
            flags,
        ) < 0 as ::core::ffi::c_int
        {
            return -(1 as ::core::ffi::c_int);
        }
        ii = ii.wrapping_add(1);
    }
    if flags & JSON_C_TO_STRING_PRETTY != 0 && had_children != 0 {
        printbuf_memappend(
            pb,
            b"\n\0" as *const u8 as *const ::core::ffi::c_char,
            (::core::mem::size_of::<[::core::ffi::c_char; 2]>() as usize).wrapping_sub(1 as usize)
                as ::core::ffi::c_int,
        );
        indent(pb, level, flags);
    }
    if flags & JSON_C_TO_STRING_SPACED != 0 && flags & JSON_C_TO_STRING_PRETTY == 0 {
        return printbuf_memappend(
            pb,
            b" ]\0" as *const u8 as *const ::core::ffi::c_char,
            (::core::mem::size_of::<[::core::ffi::c_char; 3]>() as usize).wrapping_sub(1 as usize)
                as ::core::ffi::c_int,
        );
    }
    return printbuf_memappend(
        pb,
        b"]\0" as *const u8 as *const ::core::ffi::c_char,
        (::core::mem::size_of::<[::core::ffi::c_char; 2]>() as usize).wrapping_sub(1 as usize)
            as ::core::ffi::c_int,
    );
}
unsafe extern "C" fn json_object_array_entry_free(mut data: *mut ::core::ffi::c_void) {
    json_object_put(data as *mut json_object);
}
unsafe extern "C" fn json_object_array_delete(mut jso: *mut json_object) {
    array_list_free((*JC_ARRAY(jso)).c_array);
    json_object_generic_delete(jso);
}
#[no_mangle]
pub unsafe extern "C" fn json_object_new_array() -> *mut json_object {
    return json_object_new_array_ext(ARRAY_LIST_DEFAULT_SIZE);
}
#[no_mangle]
pub unsafe extern "C" fn json_object_new_array_ext(
    mut initial_size: ::core::ffi::c_int,
) -> *mut json_object {
    let mut jso: *mut json_object_array = json_object_new(
        json_type_array,
        ::core::mem::size_of::<json_object_array>() as size_t,
        Some(
            json_object_array_to_json_string
                as unsafe extern "C" fn(
                    *mut json_object,
                    *mut printbuf,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
    ) as *mut json_object_array;
    if jso.is_null() {
        return ::core::ptr::null_mut::<json_object>();
    }
    (*jso).c_array = array_list_new2(
        Some(json_object_array_entry_free as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()),
        initial_size,
    );
    if (*jso).c_array.is_null() {
        free(jso as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<json_object>();
    }
    return &raw mut (*jso).base;
}
#[no_mangle]
pub unsafe extern "C" fn json_object_get_array(mut jso: *const json_object) -> *mut array_list {
    if jso.is_null() {
        return ::core::ptr::null_mut::<array_list>();
    }
    match (*jso).o_type as ::core::ffi::c_uint {
        5 => return (*JC_ARRAY_C(jso)).c_array as *mut array_list,
        _ => return ::core::ptr::null_mut::<array_list>(),
    };
}
#[no_mangle]
pub unsafe extern "C" fn json_object_array_sort(
    mut jso: *mut json_object,
    mut sort_fn: Option<
        unsafe extern "C" fn(
            *const ::core::ffi::c_void,
            *const ::core::ffi::c_void,
        ) -> ::core::ffi::c_int,
    >,
) {
    array_list_sort((*JC_ARRAY(jso)).c_array, sort_fn);
}
#[no_mangle]
pub unsafe extern "C" fn json_object_array_bsearch(
    mut key: *const json_object,
    mut jso: *const json_object,
    mut sort_fn: Option<
        unsafe extern "C" fn(
            *const ::core::ffi::c_void,
            *const ::core::ffi::c_void,
        ) -> ::core::ffi::c_int,
    >,
) -> *mut json_object {
    let mut result: *mut *mut json_object = ::core::ptr::null_mut::<*mut json_object>();
    result = array_list_bsearch(
        &raw mut key as *mut ::core::ffi::c_void as *mut *const ::core::ffi::c_void,
        (*JC_ARRAY_C(jso)).c_array,
        sort_fn,
    ) as *mut *mut json_object;
    if result.is_null() {
        return ::core::ptr::null_mut::<json_object>();
    }
    return *result;
}
#[no_mangle]
pub unsafe extern "C" fn json_object_array_length(mut jso: *const json_object) -> size_t {
    return array_list_length((*JC_ARRAY_C(jso)).c_array);
}
#[no_mangle]
pub unsafe extern "C" fn json_object_array_add(
    mut jso: *mut json_object,
    mut val: *mut json_object,
) -> ::core::ffi::c_int {
    return array_list_add((*JC_ARRAY(jso)).c_array, val as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn json_object_array_insert_idx(
    mut jso: *mut json_object,
    mut idx: size_t,
    mut val: *mut json_object,
) -> ::core::ffi::c_int {
    return array_list_insert_idx(
        (*JC_ARRAY(jso)).c_array,
        idx,
        val as *mut ::core::ffi::c_void,
    );
}
#[no_mangle]
pub unsafe extern "C" fn json_object_array_put_idx(
    mut jso: *mut json_object,
    mut idx: size_t,
    mut val: *mut json_object,
) -> ::core::ffi::c_int {
    return array_list_put_idx(
        (*JC_ARRAY(jso)).c_array,
        idx,
        val as *mut ::core::ffi::c_void,
    );
}
#[no_mangle]
pub unsafe extern "C" fn json_object_array_del_idx(
    mut jso: *mut json_object,
    mut idx: size_t,
    mut count: size_t,
) -> ::core::ffi::c_int {
    return array_list_del_idx((*JC_ARRAY(jso)).c_array, idx, count);
}
#[no_mangle]
pub unsafe extern "C" fn json_object_array_get_idx(
    mut jso: *const json_object,
    mut idx: size_t,
) -> *mut json_object {
    return array_list_get_idx((*JC_ARRAY_C(jso)).c_array, idx) as *mut json_object;
}
unsafe extern "C" fn json_array_equal(
    mut jso1: *mut json_object,
    mut jso2: *mut json_object,
) -> ::core::ffi::c_int {
    let mut len: size_t = 0;
    let mut i: size_t = 0;
    len = json_object_array_length(jso1);
    if len != json_object_array_length(jso2) {
        return 0 as ::core::ffi::c_int;
    }
    i = 0 as size_t;
    while i < len {
        if json_object_equal(
            json_object_array_get_idx(jso1, i),
            json_object_array_get_idx(jso2, i),
        ) == 0
        {
            return 0 as ::core::ffi::c_int;
        }
        i = i.wrapping_add(1);
    }
    return 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn json_object_array_shrink(
    mut jso: *mut json_object,
    mut empty_slots: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if empty_slots < 0 as ::core::ffi::c_int {
        json_abort(
            b"json_object_array_shrink called with negative empty_slots\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    return array_list_shrink((*JC_ARRAY(jso)).c_array, empty_slots as size_t);
}
#[no_mangle]
pub unsafe extern "C" fn json_object_new_null() -> *mut json_object {
    return ::core::ptr::null_mut::<json_object>();
}
unsafe extern "C" fn json_object_all_values_equal(
    mut jso1: *mut json_object,
    mut jso2: *mut json_object,
) -> ::core::ffi::c_int {
    let mut iter: json_object_iter = json_object_iter {
        key: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        val: ::core::ptr::null_mut::<json_object>(),
        entry: ::core::ptr::null_mut::<lh_entry>(),
    };
    let mut sub: *mut json_object = ::core::ptr::null_mut::<json_object>();
    iter.entry = lh_table_head(json_object_get_object(jso1)) as *mut lh_entry;
    while !if !iter.entry.is_null() {
        iter.key = lh_entry_k(iter.entry) as *mut ::core::ffi::c_char;
        iter.val = lh_entry_v(iter.entry) as *mut json_object as *mut json_object;
        iter.entry
    } else {
        ::core::ptr::null_mut::<lh_entry>()
    }
    .is_null()
    {
        if lh_table_lookup_ex(
            (*JC_OBJECT(jso2)).c_object,
            iter.key as *mut ::core::ffi::c_void,
            &raw mut sub as *mut ::core::ffi::c_void as *mut *mut ::core::ffi::c_void,
        ) == 0
        {
            return 0 as ::core::ffi::c_int;
        }
        if json_object_equal(iter.val as *mut json_object, sub) == 0 {
            return 0 as ::core::ffi::c_int;
        }
        iter.entry = lh_entry_next(iter.entry) as *mut lh_entry;
    }
    iter.entry = lh_table_head(json_object_get_object(jso2)) as *mut lh_entry;
    while !if !iter.entry.is_null() {
        iter.key = lh_entry_k(iter.entry) as *mut ::core::ffi::c_char;
        iter.val = lh_entry_v(iter.entry) as *mut json_object as *mut json_object;
        iter.entry
    } else {
        ::core::ptr::null_mut::<lh_entry>()
    }
    .is_null()
    {
        if lh_table_lookup_ex(
            (*JC_OBJECT(jso1)).c_object,
            iter.key as *mut ::core::ffi::c_void,
            &raw mut sub as *mut ::core::ffi::c_void as *mut *mut ::core::ffi::c_void,
        ) == 0
        {
            return 0 as ::core::ffi::c_int;
        }
        iter.entry = lh_entry_next(iter.entry) as *mut lh_entry;
    }
    return 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn json_object_equal(
    mut jso1: *mut json_object,
    mut jso2: *mut json_object,
) -> ::core::ffi::c_int {
    if jso1 == jso2 {
        return 1 as ::core::ffi::c_int;
    }
    if jso1.is_null() || jso2.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    if (*jso1).o_type as ::core::ffi::c_uint != (*jso2).o_type as ::core::ffi::c_uint {
        return 0 as ::core::ffi::c_int;
    }
    match (*jso1).o_type as ::core::ffi::c_uint {
        1 => {
            return ((*JC_BOOL(jso1)).c_boolean == (*JC_BOOL(jso2)).c_boolean)
                as ::core::ffi::c_int;
        }
        2 => {
            return ((*JC_DOUBLE(jso1)).c_double == (*JC_DOUBLE(jso2)).c_double)
                as ::core::ffi::c_int;
        }
        3 => {
            let mut int1: *mut json_object_int = JC_INT(jso1);
            let mut int2: *mut json_object_int = JC_INT(jso2);
            if (*int1).cint_type as ::core::ffi::c_uint
                == json_object_int_type_int64 as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                if (*int2).cint_type as ::core::ffi::c_uint
                    == json_object_int_type_int64 as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    return ((*int1).cint.c_int64 == (*int2).cint.c_int64) as ::core::ffi::c_int;
                }
                if (*int1).cint.c_int64 < 0 as int64_t {
                    return 0 as ::core::ffi::c_int;
                }
                return ((*int1).cint.c_int64 as uint64_t == (*int2).cint.c_uint64)
                    as ::core::ffi::c_int;
            }
            if (*int2).cint_type as ::core::ffi::c_uint
                == json_object_int_type_uint64 as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                return ((*int1).cint.c_uint64 == (*int2).cint.c_uint64) as ::core::ffi::c_int;
            }
            if (*int2).cint.c_int64 < 0 as int64_t {
                return 0 as ::core::ffi::c_int;
            }
            return ((*int1).cint.c_uint64 == (*int2).cint.c_int64 as uint64_t)
                as ::core::ffi::c_int;
        }
        6 => {
            return (_json_object_get_string_len(JC_STRING(jso1))
                == _json_object_get_string_len(JC_STRING(jso2))
                && memcmp(
                    get_string_component(jso1) as *const ::core::ffi::c_void,
                    get_string_component(jso2) as *const ::core::ffi::c_void,
                    _json_object_get_string_len(JC_STRING(jso1)) as size_t,
                ) == 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
        }
        4 => return json_object_all_values_equal(jso1, jso2),
        5 => return json_array_equal(jso1, jso2),
        0 => return 1 as ::core::ffi::c_int,
        _ => {}
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn json_object_copy_serializer_data(
    mut src: *mut json_object,
    mut dst: *mut json_object,
) -> ::core::ffi::c_int {
    if (*src)._userdata.is_null() && (*src)._user_delete.is_none() {
        return 0 as ::core::ffi::c_int;
    }
    if (*dst)._to_json_string
        == Some(
            json_object_userdata_to_json_string
                as unsafe extern "C" fn(
                    *mut json_object,
                    *mut printbuf,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        )
        || (*dst)._to_json_string
            == Some(
                _json_object_userdata_to_json_string
                    as unsafe extern "C" fn(
                        *mut json_object,
                        *mut printbuf,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            )
    {
        let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        p = strdup((*src)._userdata as *const ::core::ffi::c_char);
        if p.is_null() {
            _json_c_set_last_err(
                b"json_object_copy_serializer_data: out of memory\n\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            return -(1 as ::core::ffi::c_int);
        }
        (*dst)._userdata = p as *mut ::core::ffi::c_void;
    } else {
        _json_c_set_last_err(
            b"json_object_copy_serializer_data: unable to copy unknown serializer data: %p\n\0"
                as *const u8 as *const ::core::ffi::c_char,
            ::core::mem::transmute::<Option<json_object_to_json_string_fn>, *mut ::core::ffi::c_void>(
                (*dst)._to_json_string,
            ),
        );
        return -(1 as ::core::ffi::c_int);
    }
    (*dst)._user_delete = (*src)._user_delete;
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn json_c_shallow_copy_default(
    mut src: *mut json_object,
    mut parent: *mut json_object,
    mut key: *const ::core::ffi::c_char,
    mut index: size_t,
    mut dst: *mut *mut json_object,
) -> ::core::ffi::c_int {
    match (*src).o_type as ::core::ffi::c_uint {
        1 => {
            *dst = json_object_new_boolean((*JC_BOOL(src as *mut json_object)).c_boolean)
                as *mut json_object;
        }
        2 => {
            *dst = json_object_new_double((*JC_DOUBLE(src as *mut json_object)).c_double)
                as *mut json_object;
        }
        3 => match (*JC_INT(src as *mut json_object)).cint_type as ::core::ffi::c_uint {
            0 => {
                *dst = json_object_new_int64((*JC_INT(src as *mut json_object)).cint.c_int64)
                    as *mut json_object;
            }
            1 => {
                *dst = json_object_new_uint64((*JC_INT(src as *mut json_object)).cint.c_uint64)
                    as *mut json_object;
            }
            _ => {
                json_abort(b"invalid cint_type\0" as *const u8 as *const ::core::ffi::c_char);
            }
        },
        6 => {
            *dst = json_object_new_string_len(
                get_string_component(src),
                _json_object_get_string_len(JC_STRING(src as *mut json_object))
                    as ::core::ffi::c_int,
            ) as *mut json_object;
        }
        4 => {
            *dst = json_object_new_object() as *mut json_object;
        }
        5 => {
            *dst = json_object_new_array() as *mut json_object;
        }
        _ => {
            *__errno_location() = EINVAL;
            return -(1 as ::core::ffi::c_int);
        }
    }
    if (*dst).is_null() {
        *__errno_location() = ENOMEM;
        return -(1 as ::core::ffi::c_int);
    }
    (**dst)._to_json_string = (*src)._to_json_string;
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn json_object_deep_copy_recursive(
    mut src: *mut json_object,
    mut parent: *mut json_object,
    mut key_in_parent: *const ::core::ffi::c_char,
    mut index_in_parent: size_t,
    mut dst: *mut *mut json_object,
    mut shallow_copy: Option<json_c_shallow_copy_fn>,
) -> ::core::ffi::c_int {
    let mut iter: json_object_iter = json_object_iter {
        key: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        val: ::core::ptr::null_mut::<json_object>(),
        entry: ::core::ptr::null_mut::<lh_entry>(),
    };
    let mut src_array_len: size_t = 0;
    let mut ii: size_t = 0;
    let mut shallow_copy_rc: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    shallow_copy_rc = shallow_copy.expect("non-null function pointer")(
        src as *mut json_object,
        parent as *mut json_object,
        key_in_parent,
        index_in_parent,
        dst as *mut *mut json_object,
    );
    if shallow_copy_rc < 1 as ::core::ffi::c_int {
        *__errno_location() = EINVAL;
        return -(1 as ::core::ffi::c_int);
    }
    match (*src).o_type as ::core::ffi::c_uint {
        4 => {
            iter.entry = lh_table_head(json_object_get_object(src)) as *mut lh_entry;
            while !if !iter.entry.is_null() {
                iter.key = lh_entry_k(iter.entry) as *mut ::core::ffi::c_char;
                iter.val = lh_entry_v(iter.entry) as *mut json_object as *mut json_object;
                iter.entry
            } else {
                ::core::ptr::null_mut::<lh_entry>()
            }
            .is_null()
            {
                let mut jso: *mut json_object = ::core::ptr::null_mut::<json_object>();
                if iter.val.is_null() {
                    jso = ::core::ptr::null_mut::<json_object>();
                } else if json_object_deep_copy_recursive(
                    iter.val as *mut json_object,
                    src,
                    iter.key,
                    UINT_MAX as size_t,
                    &raw mut jso,
                    shallow_copy,
                ) < 0 as ::core::ffi::c_int
                {
                    json_object_put(jso);
                    return -(1 as ::core::ffi::c_int);
                }
                if json_object_object_add(*dst, iter.key, jso) < 0 as ::core::ffi::c_int {
                    json_object_put(jso);
                    return -(1 as ::core::ffi::c_int);
                }
                iter.entry = lh_entry_next(iter.entry) as *mut lh_entry;
            }
        }
        5 => {
            src_array_len = json_object_array_length(src);
            ii = 0 as size_t;
            while ii < src_array_len {
                let mut jso_0: *mut json_object = ::core::ptr::null_mut::<json_object>();
                let mut jso1: *mut json_object = json_object_array_get_idx(src, ii);
                if jso1.is_null() {
                    jso_0 = ::core::ptr::null_mut::<json_object>();
                } else if json_object_deep_copy_recursive(
                    jso1,
                    src,
                    ::core::ptr::null::<::core::ffi::c_char>(),
                    ii,
                    &raw mut jso_0,
                    shallow_copy,
                ) < 0 as ::core::ffi::c_int
                {
                    json_object_put(jso_0);
                    return -(1 as ::core::ffi::c_int);
                }
                if json_object_array_add(*dst, jso_0) < 0 as ::core::ffi::c_int {
                    json_object_put(jso_0);
                    return -(1 as ::core::ffi::c_int);
                }
                ii = ii.wrapping_add(1);
            }
        }
        _ => {}
    }
    if shallow_copy_rc != 2 as ::core::ffi::c_int {
        return json_object_copy_serializer_data(src, *dst);
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn json_object_deep_copy(
    mut src: *mut json_object,
    mut dst: *mut *mut json_object,
    mut shallow_copy: Option<json_c_shallow_copy_fn>,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = 0;
    if src.is_null() || dst.is_null() || !(*dst).is_null() {
        *__errno_location() = EINVAL;
        return -(1 as ::core::ffi::c_int);
    }
    if shallow_copy.is_none() {
        shallow_copy = Some(
            json_c_shallow_copy_default
                as unsafe extern "C" fn(
                    *mut json_object,
                    *mut json_object,
                    *const ::core::ffi::c_char,
                    size_t,
                    *mut *mut json_object,
                ) -> ::core::ffi::c_int,
        );
    }
    rc = json_object_deep_copy_recursive(
        src,
        ::core::ptr::null_mut::<json_object>(),
        ::core::ptr::null::<::core::ffi::c_char>(),
        UINT_MAX as size_t,
        dst,
        shallow_copy,
    );
    if rc < 0 as ::core::ffi::c_int {
        json_object_put(*dst);
        *dst = ::core::ptr::null_mut::<json_object>();
    }
    return rc;
}
#[cold]
unsafe extern "C" fn json_abort(mut message: *const ::core::ffi::c_char) -> ! {
    if !message.is_null() {
        fprintf(
            stderr,
            b"json-c aborts with error: %s\n\0" as *const u8 as *const ::core::ffi::c_char,
            message,
        );
    }
    abort();
}
pub const __INT_MAX__: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
pub const __LONG_MAX__: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
