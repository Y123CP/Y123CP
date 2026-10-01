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
    fn fflush(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn printf(__format: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn mc_set_debug(debug: ::core::ffi::c_int);
    fn json_object_get(obj: *mut json_object) -> *mut json_object;
    fn json_object_put(obj: *mut json_object) -> ::core::ffi::c_int;
    fn json_object_to_json_string(obj: *mut json_object) -> *const ::core::ffi::c_char;
    fn json_object_to_json_string_ext(
        obj: *mut json_object,
        flags: ::core::ffi::c_int,
    ) -> *const ::core::ffi::c_char;
    fn json_object_new_object() -> *mut json_object;
    fn json_object_get_object(obj: *const json_object) -> *mut lh_table;
    fn json_object_object_add(
        obj: *mut json_object,
        key: *const ::core::ffi::c_char,
        val: *mut json_object,
    ) -> ::core::ffi::c_int;
    fn json_object_object_del(obj: *mut json_object, key: *const ::core::ffi::c_char);
    fn json_object_new_array() -> *mut json_object;
    fn json_object_new_array_ext(initial_size: ::core::ffi::c_int) -> *mut json_object;
    fn json_object_array_length(obj: *const json_object) -> size_t;
    fn json_object_array_sort(
        jso: *mut json_object,
        sort_fn_0: Option<
            unsafe extern "C" fn(
                *const ::core::ffi::c_void,
                *const ::core::ffi::c_void,
            ) -> ::core::ffi::c_int,
        >,
    );
    fn json_object_array_bsearch(
        key: *const json_object,
        jso: *const json_object,
        sort_fn_0: Option<
            unsafe extern "C" fn(
                *const ::core::ffi::c_void,
                *const ::core::ffi::c_void,
            ) -> ::core::ffi::c_int,
        >,
    ) -> *mut json_object;
    fn json_object_array_add(obj: *mut json_object, val: *mut json_object) -> ::core::ffi::c_int;
    fn json_object_array_put_idx(
        obj: *mut json_object,
        idx: size_t,
        val: *mut json_object,
    ) -> ::core::ffi::c_int;
    fn json_object_array_insert_idx(
        obj: *mut json_object,
        idx: size_t,
        val: *mut json_object,
    ) -> ::core::ffi::c_int;
    fn json_object_array_get_idx(obj: *const json_object, idx: size_t) -> *mut json_object;
    fn json_object_array_del_idx(
        obj: *mut json_object,
        idx: size_t,
        count: size_t,
    ) -> ::core::ffi::c_int;
    fn json_object_new_boolean(b: json_bool) -> *mut json_object;
    fn json_object_new_int(i: int32_t) -> *mut json_object;
    fn json_object_get_int(obj: *const json_object) -> int32_t;
    fn json_object_new_string(s: *const ::core::ffi::c_char) -> *mut json_object;
    fn json_object_get_string(obj: *mut json_object) -> *const ::core::ffi::c_char;
    fn json_object_new_null() -> *mut json_object;
    fn json_object_equal(obj1: *mut json_object, obj2: *mut json_object) -> ::core::ffi::c_int;
    fn json_tokener_parse(str: *const ::core::ffi::c_char) -> *mut json_object;
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
pub type uintptr_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lh_entry {
    pub k: *const ::core::ffi::c_void,
    pub k_is_constant: ::core::ffi::c_int,
    pub v: *const ::core::ffi::c_void,
    pub next: *mut lh_entry,
    pub prev: *mut lh_entry,
}
pub type json_bool = ::core::ffi::c_int;
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
pub const INT_MIN: ::core::ffi::c_int = -__INT_MAX__ - 1 as ::core::ffi::c_int;
pub const __ASSERT_FUNCTION: [::core::ffi::c_char; 33] = unsafe {
    ::core::mem::transmute::<[u8; 33], [::core::ffi::c_char; 33]>(
        *b"void test_array_insert_idx(void)\0",
    )
};
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const EXIT_SUCCESS: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const ARRAY_LIST_DEFAULT_SIZE: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
pub const SIZE_MAX: ::core::ffi::c_ulong = 18446744073709551615 as ::core::ffi::c_ulong;
pub const JSON_C_TO_STRING_NOSLASHESCAPE: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn lh_table_head(mut t: *const lh_table) -> *mut lh_entry {
    return (*t).head;
}
#[inline]
unsafe extern "C" fn lh_entry_k(mut e: *const lh_entry) -> *mut ::core::ffi::c_void {
    return (*e).k as uintptr_t as *mut ::core::ffi::c_void;
}
#[inline]
unsafe extern "C" fn lh_entry_v(mut e: *const lh_entry) -> *mut ::core::ffi::c_void {
    return (*e).v as uintptr_t as *mut ::core::ffi::c_void;
}
#[inline]
unsafe extern "C" fn lh_entry_next(mut e: *const lh_entry) -> *mut lh_entry {
    return (*e).next;
}
unsafe extern "C" fn sort_fn(
    mut j1: *const ::core::ffi::c_void,
    mut j2: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut jso1: *const *mut json_object = ::core::ptr::null::<*mut json_object>();
    let mut jso2: *const *mut json_object = ::core::ptr::null::<*mut json_object>();
    let mut i1: ::core::ffi::c_int = 0;
    let mut i2: ::core::ffi::c_int = 0;
    jso1 = j1 as *const *mut json_object;
    jso2 = j2 as *const *mut json_object;
    if (*jso1).is_null() && (*jso2).is_null() {
        return 0 as ::core::ffi::c_int;
    }
    if (*jso1).is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    if (*jso2).is_null() {
        return 1 as ::core::ffi::c_int;
    }
    i1 = json_object_get_int(*jso1) as ::core::ffi::c_int;
    i2 = json_object_get_int(*jso2) as ::core::ffi::c_int;
    return i1 - i2;
}
#[no_mangle]
pub unsafe extern "C" fn make_array() -> *mut json_object {
    let mut my_array: *mut json_object = ::core::ptr::null_mut::<json_object>();
    my_array = json_object_new_array() as *mut json_object;
    json_object_array_add(
        my_array as *mut json_object,
        json_object_new_int(1 as int32_t),
    );
    json_object_array_add(
        my_array as *mut json_object,
        json_object_new_int(2 as int32_t),
    );
    json_object_array_add(
        my_array as *mut json_object,
        json_object_new_int(3 as int32_t),
    );
    json_object_array_put_idx(
        my_array as *mut json_object,
        4 as size_t,
        json_object_new_int(5 as int32_t),
    );
    json_object_array_put_idx(
        my_array as *mut json_object,
        3 as size_t,
        json_object_new_int(4 as int32_t),
    );
    json_object_array_put_idx(
        my_array as *mut json_object,
        6 as size_t,
        json_object_new_int(7 as int32_t),
    );
    return my_array;
}
#[no_mangle]
pub unsafe extern "C" fn test_array_del_idx() {
    let mut rc: ::core::ffi::c_int = 0;
    let mut ii: size_t = 0;
    let mut orig_array_len: size_t = 0;
    let mut my_array: *mut json_object = ::core::ptr::null_mut::<json_object>();
    my_array = make_array();
    orig_array_len = json_object_array_length(my_array);
    printf(b"my_array=\n\0" as *const u8 as *const ::core::ffi::c_char);
    ii = 0 as size_t;
    while ii < json_object_array_length(my_array) {
        let mut obj: *mut json_object = json_object_array_get_idx(my_array, ii) as *mut json_object;
        printf(
            b"\t[%d]=%s\n\0" as *const u8 as *const ::core::ffi::c_char,
            ii as ::core::ffi::c_int,
            json_object_to_json_string(obj as *mut json_object),
        );
        ii = ii.wrapping_add(1);
    }
    printf(
        b"my_array.to_string()=%s\n\0" as *const u8 as *const ::core::ffi::c_char,
        json_object_to_json_string(my_array as *mut json_object),
    );
    ii = 0 as size_t;
    while ii < orig_array_len {
        rc = json_object_array_del_idx(my_array as *mut json_object, 0 as size_t, 1 as size_t);
        printf(
            b"after del_idx(0,1)=%d, my_array.to_string()=%s\n\0" as *const u8
                as *const ::core::ffi::c_char,
            rc,
            json_object_to_json_string(my_array as *mut json_object),
        );
        ii = ii.wrapping_add(1);
    }
    rc = json_object_array_del_idx(my_array as *mut json_object, 0 as size_t, 1 as size_t);
    printf(
        b"after del_idx(0,1)=%d, my_array.to_string()=%s\n\0" as *const u8
            as *const ::core::ffi::c_char,
        rc,
        json_object_to_json_string(my_array as *mut json_object),
    );
    json_object_put(my_array as *mut json_object);
    my_array = make_array();
    rc = json_object_array_del_idx(my_array as *mut json_object, 0 as size_t, orig_array_len);
    printf(
        b"after del_idx(0,%d)=%d, my_array.to_string()=%s\n\0" as *const u8
            as *const ::core::ffi::c_char,
        orig_array_len as ::core::ffi::c_int,
        rc,
        json_object_to_json_string(my_array as *mut json_object),
    );
    json_object_put(my_array as *mut json_object);
    my_array = make_array();
    rc = json_object_array_del_idx(
        my_array as *mut json_object,
        0 as size_t,
        orig_array_len.wrapping_add(1 as size_t),
    );
    printf(
        b"after del_idx(0,%d)=%d, my_array.to_string()=%s\n\0" as *const u8
            as *const ::core::ffi::c_char,
        orig_array_len.wrapping_add(1 as size_t) as ::core::ffi::c_int,
        rc,
        json_object_to_json_string(my_array as *mut json_object),
    );
    json_object_put(my_array as *mut json_object);
    my_array = make_array();
    rc = json_object_array_del_idx(
        my_array as *mut json_object,
        0 as size_t,
        orig_array_len.wrapping_sub(1 as size_t),
    );
    printf(
        b"after del_idx(0,%d)=%d, my_array.to_string()=%s\n\0" as *const u8
            as *const ::core::ffi::c_char,
        orig_array_len.wrapping_sub(1 as size_t) as ::core::ffi::c_int,
        rc,
        json_object_to_json_string(my_array as *mut json_object),
    );
    json_object_array_add(
        my_array as *mut json_object,
        json_object_new_string(b"s1\0" as *const u8 as *const ::core::ffi::c_char),
    );
    json_object_array_add(
        my_array as *mut json_object,
        json_object_new_string(b"s2\0" as *const u8 as *const ::core::ffi::c_char),
    );
    json_object_array_add(
        my_array as *mut json_object,
        json_object_new_string(b"s3\0" as *const u8 as *const ::core::ffi::c_char),
    );
    printf(
        b"after adding more entries, my_array.to_string()=%s\n\0" as *const u8
            as *const ::core::ffi::c_char,
        json_object_to_json_string(my_array as *mut json_object),
    );
    json_object_put(my_array as *mut json_object);
}
#[no_mangle]
pub unsafe extern "C" fn test_array_list_expand_internal() {
    let mut rc: ::core::ffi::c_int = 0;
    let mut ii: size_t = 0;
    let mut idx: size_t = 0;
    let mut my_array: *mut json_object = ::core::ptr::null_mut::<json_object>();
    my_array = make_array();
    printf(b"my_array=\n\0" as *const u8 as *const ::core::ffi::c_char);
    ii = 0 as size_t;
    while ii < json_object_array_length(my_array) {
        let mut obj: *mut json_object = json_object_array_get_idx(my_array, ii) as *mut json_object;
        printf(
            b"\t[%d]=%s\n\0" as *const u8 as *const ::core::ffi::c_char,
            ii as ::core::ffi::c_int,
            json_object_to_json_string(obj as *mut json_object),
        );
        ii = ii.wrapping_add(1);
    }
    printf(
        b"my_array.to_string()=%s\n\0" as *const u8 as *const ::core::ffi::c_char,
        json_object_to_json_string(my_array as *mut json_object),
    );
    rc = json_object_array_put_idx(
        my_array as *mut json_object,
        5 as size_t,
        json_object_new_int(6 as int32_t),
    );
    printf(
        b"put_idx(5,6)=%d\n\0" as *const u8 as *const ::core::ffi::c_char,
        rc,
    );
    idx = (ARRAY_LIST_DEFAULT_SIZE * 2 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as size_t;
    rc = json_object_array_put_idx(
        my_array as *mut json_object,
        idx,
        json_object_new_int(0 as int32_t),
    );
    printf(
        b"put_idx(%d,0)=%d\n\0" as *const u8 as *const ::core::ffi::c_char,
        idx as ::core::ffi::c_int,
        rc,
    );
    idx = (ARRAY_LIST_DEFAULT_SIZE * 2 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as size_t;
    rc = json_object_array_put_idx(
        my_array as *mut json_object,
        idx,
        json_object_new_int(0 as int32_t),
    );
    printf(
        b"put_idx(%d,0)=%d\n\0" as *const u8 as *const ::core::ffi::c_char,
        idx as ::core::ffi::c_int,
        rc,
    );
    idx = SIZE_MAX as size_t;
    let mut tmp: *mut json_object = json_object_new_int(10 as int32_t) as *mut json_object;
    rc = json_object_array_put_idx(my_array as *mut json_object, idx, tmp as *mut json_object);
    printf(
        b"put_idx(SIZE_T_MAX,0)=%d\n\0" as *const u8 as *const ::core::ffi::c_char,
        rc,
    );
    if rc == -(1 as ::core::ffi::c_int) {
        json_object_put(tmp as *mut json_object);
    }
    json_object_put(my_array as *mut json_object);
}
#[no_mangle]
pub unsafe extern "C" fn test_array_insert_idx() {
    let mut my_array: *mut json_object = ::core::ptr::null_mut::<json_object>();
    let mut jo1: *mut json_object = ::core::ptr::null_mut::<json_object>();
    my_array = json_object_new_array() as *mut json_object;
    json_object_array_add(
        my_array as *mut json_object,
        json_object_new_int(1 as int32_t),
    );
    json_object_array_add(
        my_array as *mut json_object,
        json_object_new_int(2 as int32_t),
    );
    json_object_array_add(
        my_array as *mut json_object,
        json_object_new_int(5 as int32_t),
    );
    json_object_array_insert_idx(
        my_array as *mut json_object,
        2 as size_t,
        json_object_new_int(4 as int32_t),
    );
    jo1 = json_tokener_parse(b"[1, 2, 4, 5]\0" as *const u8 as *const ::core::ffi::c_char);
    '_c2rust_label: {
        if 1 as ::core::ffi::c_int == json_object_equal(my_array as *mut json_object, jo1) {
        } else {
            __assert_fail(
                b"1 == json_object_equal(my_array, jo1)\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test1.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                205 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    json_object_put(jo1);
    json_object_array_insert_idx(
        my_array as *mut json_object,
        2 as size_t,
        json_object_new_int(3 as int32_t),
    );
    jo1 = json_tokener_parse(b"[1, 2, 3, 4, 5]\0" as *const u8 as *const ::core::ffi::c_char);
    '_c2rust_label_0: {
        if 1 as ::core::ffi::c_int == json_object_equal(my_array as *mut json_object, jo1) {
        } else {
            __assert_fail(
                b"1 == json_object_equal(my_array, jo1)\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test1.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                211 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    json_object_put(jo1);
    json_object_array_insert_idx(
        my_array as *mut json_object,
        5 as size_t,
        json_object_new_int(6 as int32_t),
    );
    jo1 = json_tokener_parse(b"[1, 2, 3, 4, 5, 6]\0" as *const u8 as *const ::core::ffi::c_char);
    '_c2rust_label_1: {
        if 1 as ::core::ffi::c_int == json_object_equal(my_array as *mut json_object, jo1) {
        } else {
            __assert_fail(
                b"1 == json_object_equal(my_array, jo1)\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test1.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                217 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    json_object_put(jo1);
    json_object_array_insert_idx(
        my_array as *mut json_object,
        7 as size_t,
        json_object_new_int(8 as int32_t),
    );
    jo1 = json_tokener_parse(
        b"[1, 2, 3, 4, 5, 6, null, 8]\0" as *const u8 as *const ::core::ffi::c_char,
    );
    '_c2rust_label_2: {
        if 1 as ::core::ffi::c_int == json_object_equal(my_array as *mut json_object, jo1) {
        } else {
            __assert_fail(
                b"1 == json_object_equal(my_array, jo1)\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test1.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                222 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    json_object_put(jo1);
    json_object_put(my_array as *mut json_object);
}
unsafe fn main_0(
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut my_string: *mut json_object = ::core::ptr::null_mut::<json_object>();
    let mut my_int: *mut json_object = ::core::ptr::null_mut::<json_object>();
    let mut my_null: *mut json_object = ::core::ptr::null_mut::<json_object>();
    let mut my_object: *mut json_object = ::core::ptr::null_mut::<json_object>();
    let mut my_array: *mut json_object = ::core::ptr::null_mut::<json_object>();
    let mut i: size_t = 0;
    my_string = json_object_new_string(b"\t\0" as *const u8 as *const ::core::ffi::c_char)
        as *mut json_object;
    printf(
        b"my_string=%s\n\0" as *const u8 as *const ::core::ffi::c_char,
        json_object_get_string(my_string as *mut json_object),
    );
    printf(
        b"my_string.to_string()=%s\n\0" as *const u8 as *const ::core::ffi::c_char,
        json_object_to_json_string(my_string as *mut json_object),
    );
    json_object_put(my_string as *mut json_object);
    my_string = json_object_new_string(b"\\\0" as *const u8 as *const ::core::ffi::c_char)
        as *mut json_object;
    printf(
        b"my_string=%s\n\0" as *const u8 as *const ::core::ffi::c_char,
        json_object_get_string(my_string as *mut json_object),
    );
    printf(
        b"my_string.to_string()=%s\n\0" as *const u8 as *const ::core::ffi::c_char,
        json_object_to_json_string(my_string as *mut json_object),
    );
    json_object_put(my_string as *mut json_object);
    my_string = json_object_new_string(b"/\0" as *const u8 as *const ::core::ffi::c_char)
        as *mut json_object;
    printf(
        b"my_string=%s\n\0" as *const u8 as *const ::core::ffi::c_char,
        json_object_get_string(my_string as *mut json_object),
    );
    printf(
        b"my_string.to_string()=%s\n\0" as *const u8 as *const ::core::ffi::c_char,
        json_object_to_json_string(my_string as *mut json_object),
    );
    printf(
        b"my_string.to_string(NOSLASHESCAPE)=%s\n\0" as *const u8 as *const ::core::ffi::c_char,
        json_object_to_json_string_ext(
            my_string as *mut json_object,
            JSON_C_TO_STRING_NOSLASHESCAPE,
        ),
    );
    json_object_put(my_string as *mut json_object);
    my_string = json_object_new_string(b"/foo/bar/baz\0" as *const u8 as *const ::core::ffi::c_char)
        as *mut json_object;
    printf(
        b"my_string=%s\n\0" as *const u8 as *const ::core::ffi::c_char,
        json_object_get_string(my_string as *mut json_object),
    );
    printf(
        b"my_string.to_string()=%s\n\0" as *const u8 as *const ::core::ffi::c_char,
        json_object_to_json_string(my_string as *mut json_object),
    );
    printf(
        b"my_string.to_string(NOSLASHESCAPE)=%s\n\0" as *const u8 as *const ::core::ffi::c_char,
        json_object_to_json_string_ext(
            my_string as *mut json_object,
            JSON_C_TO_STRING_NOSLASHESCAPE,
        ),
    );
    json_object_put(my_string as *mut json_object);
    my_string = json_object_new_string(b"foo\0" as *const u8 as *const ::core::ffi::c_char)
        as *mut json_object;
    printf(
        b"my_string=%s\n\0" as *const u8 as *const ::core::ffi::c_char,
        json_object_get_string(my_string as *mut json_object),
    );
    printf(
        b"my_string.to_string()=%s\n\0" as *const u8 as *const ::core::ffi::c_char,
        json_object_to_json_string(my_string as *mut json_object),
    );
    my_int = json_object_new_int(9 as int32_t) as *mut json_object;
    printf(
        b"my_int=%d\n\0" as *const u8 as *const ::core::ffi::c_char,
        json_object_get_int(my_int),
    );
    printf(
        b"my_int.to_string()=%s\n\0" as *const u8 as *const ::core::ffi::c_char,
        json_object_to_json_string(my_int as *mut json_object),
    );
    my_null = json_object_new_null() as *mut json_object;
    printf(
        b"my_null.to_string()=%s\n\0" as *const u8 as *const ::core::ffi::c_char,
        json_object_to_json_string(my_null as *mut json_object),
    );
    my_array = json_object_new_array() as *mut json_object;
    json_object_array_add(
        my_array as *mut json_object,
        json_object_new_int(1 as int32_t),
    );
    json_object_array_add(
        my_array as *mut json_object,
        json_object_new_int(2 as int32_t),
    );
    json_object_array_add(
        my_array as *mut json_object,
        json_object_new_int(3 as int32_t),
    );
    json_object_array_put_idx(
        my_array as *mut json_object,
        4 as size_t,
        json_object_new_int(5 as int32_t),
    );
    printf(b"my_array=\n\0" as *const u8 as *const ::core::ffi::c_char);
    i = 0 as size_t;
    while i < json_object_array_length(my_array) {
        let mut obj: *mut json_object = json_object_array_get_idx(my_array, i) as *mut json_object;
        printf(
            b"\t[%d]=%s\n\0" as *const u8 as *const ::core::ffi::c_char,
            i as ::core::ffi::c_int,
            json_object_to_json_string(obj as *mut json_object),
        );
        i = i.wrapping_add(1);
    }
    printf(
        b"my_array.to_string()=%s\n\0" as *const u8 as *const ::core::ffi::c_char,
        json_object_to_json_string(my_array as *mut json_object),
    );
    json_object_put(my_array as *mut json_object);
    test_array_insert_idx();
    test_array_del_idx();
    test_array_list_expand_internal();
    my_array = json_object_new_array_ext(5 as ::core::ffi::c_int) as *mut json_object;
    json_object_array_add(
        my_array as *mut json_object,
        json_object_new_int(3 as int32_t),
    );
    json_object_array_add(
        my_array as *mut json_object,
        json_object_new_int(1 as int32_t),
    );
    json_object_array_add(
        my_array as *mut json_object,
        json_object_new_int(2 as int32_t),
    );
    json_object_array_put_idx(
        my_array as *mut json_object,
        4 as size_t,
        json_object_new_int(0 as int32_t),
    );
    printf(b"my_array=\n\0" as *const u8 as *const ::core::ffi::c_char);
    i = 0 as size_t;
    while i < json_object_array_length(my_array) {
        let mut obj_0: *mut json_object =
            json_object_array_get_idx(my_array, i) as *mut json_object;
        printf(
            b"\t[%d]=%s\n\0" as *const u8 as *const ::core::ffi::c_char,
            i as ::core::ffi::c_int,
            json_object_to_json_string(obj_0 as *mut json_object),
        );
        i = i.wrapping_add(1);
    }
    printf(
        b"my_array.to_string()=%s\n\0" as *const u8 as *const ::core::ffi::c_char,
        json_object_to_json_string(my_array as *mut json_object),
    );
    json_object_array_sort(
        my_array as *mut json_object,
        Some(
            sort_fn
                as unsafe extern "C" fn(
                    *const ::core::ffi::c_void,
                    *const ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
    );
    printf(b"my_array=\n\0" as *const u8 as *const ::core::ffi::c_char);
    i = 0 as size_t;
    while i < json_object_array_length(my_array) {
        let mut obj_1: *mut json_object =
            json_object_array_get_idx(my_array, i) as *mut json_object;
        printf(
            b"\t[%d]=%s\n\0" as *const u8 as *const ::core::ffi::c_char,
            i as ::core::ffi::c_int,
            json_object_to_json_string(obj_1 as *mut json_object),
        );
        i = i.wrapping_add(1);
    }
    printf(
        b"my_array.to_string()=%s\n\0" as *const u8 as *const ::core::ffi::c_char,
        json_object_to_json_string(my_array as *mut json_object),
    );
    let mut one: *mut json_object = json_object_new_int(1 as int32_t) as *mut json_object;
    let mut result: *mut json_object = json_object_array_bsearch(
        one,
        my_array,
        Some(
            sort_fn
                as unsafe extern "C" fn(
                    *const ::core::ffi::c_void,
                    *const ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
    ) as *mut json_object;
    printf(
        b"find json_object(1) in my_array successfully: %s\n\0" as *const u8
            as *const ::core::ffi::c_char,
        json_object_to_json_string(result as *mut json_object),
    );
    json_object_put(one as *mut json_object);
    my_object = json_object_new_object() as *mut json_object;
    let mut rc: ::core::ffi::c_int = json_object_object_add(
        my_object as *mut json_object,
        b"abc\0" as *const u8 as *const ::core::ffi::c_char,
        my_object as *mut json_object,
    );
    if rc != -(1 as ::core::ffi::c_int) {
        printf(
            b"ERROR: able to successfully add object to itself!\n\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        fflush(stdout);
    }
    json_object_object_add(
        my_object as *mut json_object,
        b"abc\0" as *const u8 as *const ::core::ffi::c_char,
        json_object_new_int(12 as int32_t),
    );
    json_object_object_add(
        my_object as *mut json_object,
        b"foo\0" as *const u8 as *const ::core::ffi::c_char,
        json_object_new_string(b"bar\0" as *const u8 as *const ::core::ffi::c_char),
    );
    json_object_object_add(
        my_object as *mut json_object,
        b"bool0\0" as *const u8 as *const ::core::ffi::c_char,
        json_object_new_boolean(0 as json_bool),
    );
    json_object_object_add(
        my_object as *mut json_object,
        b"bool1\0" as *const u8 as *const ::core::ffi::c_char,
        json_object_new_boolean(1 as json_bool),
    );
    json_object_object_add(
        my_object as *mut json_object,
        b"baz\0" as *const u8 as *const ::core::ffi::c_char,
        json_object_new_string(b"bang\0" as *const u8 as *const ::core::ffi::c_char),
    );
    let mut baz_obj: *mut json_object =
        json_object_new_string(b"fark\0" as *const u8 as *const ::core::ffi::c_char)
            as *mut json_object;
    json_object_get(baz_obj as *mut json_object);
    json_object_object_add(
        my_object as *mut json_object,
        b"baz\0" as *const u8 as *const ::core::ffi::c_char,
        baz_obj as *mut json_object,
    );
    json_object_object_del(
        my_object as *mut json_object,
        b"baz\0" as *const u8 as *const ::core::ffi::c_char,
    );
    printf(
        b"baz_obj.to_string()=%s\n\0" as *const u8 as *const ::core::ffi::c_char,
        json_object_to_json_string(baz_obj as *mut json_object),
    );
    json_object_put(baz_obj as *mut json_object);
    printf(b"my_object=\n\0" as *const u8 as *const ::core::ffi::c_char);
    let mut key: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut val: *mut json_object = ::core::ptr::null_mut::<json_object>();
    let mut entrykey: *mut lh_entry = lh_table_head(json_object_get_object(my_object));
    let mut entry_nextkey: *mut lh_entry = ::core::ptr::null_mut::<lh_entry>();
    while !({
        if !entrykey.is_null() {
            key = lh_entry_k(entrykey) as *mut ::core::ffi::c_char;
            val = lh_entry_v(entrykey) as *mut json_object;
            entry_nextkey = lh_entry_next(entrykey);
        }
        entrykey
    })
    .is_null()
    {
        printf(
            b"\t%s: %s\n\0" as *const u8 as *const ::core::ffi::c_char,
            key,
            json_object_to_json_string(val),
        );
        entrykey = entry_nextkey;
    }
    let mut empty_array: *mut json_object = json_object_new_array() as *mut json_object;
    let mut empty_obj: *mut json_object = json_object_new_object() as *mut json_object;
    json_object_object_add(
        my_object as *mut json_object,
        b"empty_array\0" as *const u8 as *const ::core::ffi::c_char,
        empty_array as *mut json_object,
    );
    json_object_object_add(
        my_object as *mut json_object,
        b"empty_obj\0" as *const u8 as *const ::core::ffi::c_char,
        empty_obj as *mut json_object,
    );
    printf(
        b"my_object.to_string()=%s\n\0" as *const u8 as *const ::core::ffi::c_char,
        json_object_to_json_string(my_object as *mut json_object),
    );
    json_object_put(my_array as *mut json_object);
    my_array = json_object_new_array_ext(INT_MIN + 1 as ::core::ffi::c_int) as *mut json_object;
    if !my_array.is_null() {
        printf(
            b"ERROR: able to allocate an array of negative size!\n\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        fflush(stdout);
        json_object_put(my_array as *mut json_object);
        my_array = ::core::ptr::null_mut::<json_object>();
    }
    json_object_put(my_string as *mut json_object);
    json_object_put(my_int as *mut json_object);
    json_object_put(my_null as *mut json_object);
    json_object_put(my_object as *mut json_object);
    json_object_put(my_array as *mut json_object);
    return EXIT_SUCCESS;
}
pub const __INT_MAX__: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
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
