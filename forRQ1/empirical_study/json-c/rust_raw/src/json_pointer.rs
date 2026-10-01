extern "C" {
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn json_object_put(obj: *mut json_object) -> ::core::ffi::c_int;
    fn json_object_is_type(obj: *const json_object, type_0: json_type) -> ::core::ffi::c_int;
    fn json_object_object_add(
        obj: *mut json_object,
        key: *const ::core::ffi::c_char,
        val: *mut json_object,
    ) -> ::core::ffi::c_int;
    fn json_object_object_get_ex(
        obj: *const json_object,
        key: *const ::core::ffi::c_char,
        value: *mut *mut json_object,
    ) -> json_bool;
    fn json_object_array_length(obj: *const json_object) -> size_t;
    fn json_object_array_add(obj: *mut json_object, val: *mut json_object) -> ::core::ffi::c_int;
    fn json_object_array_put_idx(
        obj: *mut json_object,
        idx: size_t,
        val: *mut json_object,
    ) -> ::core::ffi::c_int;
    fn json_object_array_get_idx(obj: *const json_object, idx: size_t) -> *mut json_object;
    fn memmove(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strdup(__s: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn strchr(__s: *const ::core::ffi::c_char, __c: ::core::ffi::c_int)
        -> *mut ::core::ffi::c_char;
    fn strrchr(
        __s: *const ::core::ffi::c_char,
        __c: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    fn strstr(
        __haystack: *const ::core::ffi::c_char,
        __needle: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn vasprintf(
        __ptr: *mut *mut ::core::ffi::c_char,
        __f: *const ::core::ffi::c_char,
        __arg: ::core::ffi::VaList,
    ) -> ::core::ffi::c_int;
    fn strtoull(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_ulonglong;
    fn free(__ptr: *mut ::core::ffi::c_void);
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
pub type __uint32_t = u32;
pub type uint32_t = __uint32_t;
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
pub type size_t = usize;
pub type va_list = __builtin_va_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct json_pointer_get_result {
    pub parent: *mut json_object,
    pub obj: *mut json_object,
    pub key_in_parent: *const ::core::ffi::c_char,
    pub index_in_parent: uint32_t,
}
pub type json_pointer_array_set_cb = Option<
    unsafe extern "C" fn(
        *mut json_object,
        size_t,
        *mut json_object,
        *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int,
>;
pub const ENOENT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const ENOMEM: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
pub const EINVAL: ::core::ffi::c_int = 22 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
unsafe extern "C" fn string_replace_all_occurrences_with_char(
    mut s: *mut ::core::ffi::c_char,
    mut occur: *const ::core::ffi::c_char,
    mut repl_char: ::core::ffi::c_char,
) {
    let mut slen: size_t = strlen(s);
    let mut skip: size_t = strlen(occur).wrapping_sub(1 as size_t);
    let mut p: *mut ::core::ffi::c_char = s;
    loop {
        p = strstr(p, occur);
        if p.is_null() {
            break;
        }
        *p = repl_char;
        p = p.offset(1);
        slen = (slen as ::core::ffi::c_ulong).wrapping_sub(skip as ::core::ffi::c_ulong) as size_t
            as size_t;
        memmove(
            p as *mut ::core::ffi::c_void,
            p.offset(skip as isize) as *const ::core::ffi::c_void,
            slen.wrapping_sub(p.offset_from(s) as ::core::ffi::c_long as size_t)
                .wrapping_add(1 as size_t),
        );
    }
}
unsafe extern "C" fn is_valid_index(
    mut path: *const ::core::ffi::c_char,
    mut idx: *mut size_t,
) -> ::core::ffi::c_int {
    let mut i: size_t = 0;
    let mut len: size_t = strlen(path);
    if len == 1 as size_t {
        if *path.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int >= '0' as i32
            && *path.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int <= '9' as i32
        {
            *idx = (*path.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                - '0' as i32) as size_t;
            return 1 as ::core::ffi::c_int;
        }
        *__errno_location() = EINVAL;
        return 0 as ::core::ffi::c_int;
    }
    if *path.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '0' as i32 {
        *__errno_location() = EINVAL;
        return 0 as ::core::ffi::c_int;
    }
    i = 0 as size_t;
    while i < len {
        if !(*path.offset(i as isize) as ::core::ffi::c_int >= '0' as i32
            && *path.offset(i as isize) as ::core::ffi::c_int <= '9' as i32)
        {
            *__errno_location() = EINVAL;
            return 0 as ::core::ffi::c_int;
        }
        i = i.wrapping_add(1);
    }
    *idx = strtoull(
        path,
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        10 as ::core::ffi::c_int,
    ) as size_t;
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn json_pointer_get_single_path(
    mut obj: *mut json_object,
    mut path: *mut ::core::ffi::c_char,
    mut value: *mut *mut json_object,
    mut idx: *mut size_t,
) -> ::core::ffi::c_int {
    if json_object_is_type(obj, json_type_array) != 0 {
        if is_valid_index(path, idx) == 0 {
            return -(1 as ::core::ffi::c_int);
        }
        if *idx >= json_object_array_length(obj) {
            *__errno_location() = ENOENT;
            return -(1 as ::core::ffi::c_int);
        }
        obj = json_object_array_get_idx(obj, *idx);
        if !obj.is_null() {
            if !value.is_null() {
                *value = obj;
            }
            return 0 as ::core::ffi::c_int;
        }
        *__errno_location() = ENOENT;
        return -(1 as ::core::ffi::c_int);
    }
    string_replace_all_occurrences_with_char(
        path,
        b"~1\0" as *const u8 as *const ::core::ffi::c_char,
        '/' as i32 as ::core::ffi::c_char,
    );
    string_replace_all_occurrences_with_char(
        path,
        b"~0\0" as *const u8 as *const ::core::ffi::c_char,
        '~' as i32 as ::core::ffi::c_char,
    );
    if json_object_object_get_ex(obj, path, value) == 0 {
        *__errno_location() = ENOENT;
        return -(1 as ::core::ffi::c_int);
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn json_object_array_put_idx_cb(
    mut parent: *mut json_object,
    mut idx: size_t,
    mut value: *mut json_object,
    mut priv_0: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    return json_object_array_put_idx(parent, idx, value);
}
unsafe extern "C" fn json_pointer_set_single_path(
    mut parent: *mut json_object,
    mut path: *const ::core::ffi::c_char,
    mut value: *mut json_object,
    mut array_set_cb: json_pointer_array_set_cb,
    mut priv_0: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    if json_object_is_type(parent, json_type_array) != 0 {
        let mut idx: size_t = 0;
        if *path.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '-' as i32
            && *path.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '\0' as i32
        {
            return json_object_array_add(parent, value);
        }
        if is_valid_index(path, &raw mut idx) == 0 {
            return -(1 as ::core::ffi::c_int);
        }
        return array_set_cb.expect("non-null function pointer")(
            parent as *mut json_object,
            idx,
            value as *mut json_object,
            priv_0,
        );
    }
    if json_object_is_type(parent, json_type_object) != 0 {
        return json_object_object_add(parent, path, value);
    }
    *__errno_location() = ENOENT;
    return -(1 as ::core::ffi::c_int);
}
unsafe extern "C" fn json_pointer_result_get_recursive(
    mut obj: *mut json_object,
    mut path: *mut ::core::ffi::c_char,
    mut res: *mut json_pointer_get_result,
) -> ::core::ffi::c_int {
    let mut parent_obj: *mut json_object = obj;
    let mut idx: size_t = 0;
    let mut endp: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut rc: ::core::ffi::c_int = 0;
    if *path.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != '/' as i32 {
        *__errno_location() = EINVAL;
        return -(1 as ::core::ffi::c_int);
    }
    path = path.offset(1);
    endp = strchr(path, '/' as i32);
    if !endp.is_null() {
        *endp = '\0' as i32 as ::core::ffi::c_char;
    }
    rc = json_pointer_get_single_path(obj, path, &raw mut obj, &raw mut idx);
    if rc != 0 {
        return rc;
    }
    if !endp.is_null() {
        *endp = '/' as i32 as ::core::ffi::c_char;
        return json_pointer_result_get_recursive(obj, endp, res);
    }
    if !res.is_null() {
        (*res).parent = parent_obj;
        (*res).obj = obj;
        if json_object_is_type((*res).parent, json_type_array) != 0 {
            (*res).index_in_parent = idx as uint32_t;
        } else {
            (*res).key_in_parent = path;
        }
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn json_pointer_object_get_recursive(
    mut obj: *mut json_object,
    mut path: *mut ::core::ffi::c_char,
    mut value: *mut *mut json_object,
) -> ::core::ffi::c_int {
    let mut res: json_pointer_get_result = json_pointer_get_result {
        parent: ::core::ptr::null_mut::<json_object>(),
        obj: ::core::ptr::null_mut::<json_object>(),
        key_in_parent: ::core::ptr::null::<::core::ffi::c_char>(),
        index_in_parent: 0,
    };
    let mut rc: ::core::ffi::c_int = 0;
    rc = json_pointer_result_get_recursive(obj, path, &raw mut res);
    if rc != 0 {
        return rc;
    }
    if !value.is_null() {
        *value = res.obj;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn json_pointer_get_internal(
    mut obj: *mut json_object,
    mut path: *const ::core::ffi::c_char,
    mut res: *mut json_pointer_get_result,
) -> ::core::ffi::c_int {
    let mut path_copy: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut rc: ::core::ffi::c_int = 0;
    if obj.is_null() || path.is_null() {
        *__errno_location() = EINVAL;
        return -(1 as ::core::ffi::c_int);
    }
    if *path.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '\0' as i32 {
        (*res).parent = ::core::ptr::null_mut::<json_object>();
        (*res).obj = obj;
        (*res).key_in_parent = ::core::ptr::null::<::core::ffi::c_char>();
        (*res).index_in_parent = -(1 as ::core::ffi::c_int) as uint32_t;
        return 0 as ::core::ffi::c_int;
    }
    path_copy = strdup(path);
    if path_copy.is_null() {
        *__errno_location() = ENOMEM;
        return -(1 as ::core::ffi::c_int);
    }
    rc = json_pointer_result_get_recursive(obj, path_copy, res);
    if rc == 0 as ::core::ffi::c_int
        && json_object_is_type((*res).parent, json_type_object) != 0
        && !(*res).key_in_parent.is_null()
    {
        (*res).key_in_parent = path
            .offset((*res).key_in_parent.offset_from(path_copy) as ::core::ffi::c_long as isize);
    }
    free(path_copy as *mut ::core::ffi::c_void);
    return rc;
}
#[no_mangle]
pub unsafe extern "C" fn json_pointer_get(
    mut obj: *mut json_object,
    mut path: *const ::core::ffi::c_char,
    mut res: *mut *mut json_object,
) -> ::core::ffi::c_int {
    let mut jpres: json_pointer_get_result = json_pointer_get_result {
        parent: ::core::ptr::null_mut::<json_object>(),
        obj: ::core::ptr::null_mut::<json_object>(),
        key_in_parent: ::core::ptr::null::<::core::ffi::c_char>(),
        index_in_parent: 0,
    };
    let mut rc: ::core::ffi::c_int = 0;
    rc = json_pointer_get_internal(obj, path, &raw mut jpres);
    if rc != 0 {
        return rc;
    }
    if !res.is_null() {
        *res = jpres.obj;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn json_pointer_getf(
    mut obj: *mut json_object,
    mut res: *mut *mut json_object,
    mut path_fmt: *const ::core::ffi::c_char,
    mut args: ...
) -> ::core::ffi::c_int {
    let mut path_copy: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut rc: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut args_0: ::core::ffi::VaListImpl;
    if obj.is_null() || path_fmt.is_null() {
        *__errno_location() = EINVAL;
        return -(1 as ::core::ffi::c_int);
    }
    args_0 = args.clone();
    rc = vasprintf(&raw mut path_copy, path_fmt, args_0.as_va_list());
    if rc < 0 as ::core::ffi::c_int {
        return rc;
    }
    if *path_copy.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '\0' as i32 {
        if !res.is_null() {
            *res = obj;
        }
    } else {
        rc = json_pointer_object_get_recursive(obj, path_copy, res);
    }
    free(path_copy as *mut ::core::ffi::c_void);
    return rc;
}
#[no_mangle]
pub unsafe extern "C" fn json_pointer_set_with_array_cb(
    mut obj: *mut *mut json_object,
    mut path: *const ::core::ffi::c_char,
    mut value: *mut json_object,
    mut array_set_cb: json_pointer_array_set_cb,
    mut priv_0: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut endp: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut path_copy: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut set: *mut json_object = ::core::ptr::null_mut::<json_object>();
    let mut rc: ::core::ffi::c_int = 0;
    if obj.is_null() || path.is_null() {
        *__errno_location() = EINVAL;
        return -(1 as ::core::ffi::c_int);
    }
    if *path.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '\0' as i32 {
        json_object_put(*obj);
        *obj = value;
        return 0 as ::core::ffi::c_int;
    }
    if *path.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != '/' as i32 {
        *__errno_location() = EINVAL;
        return -(1 as ::core::ffi::c_int);
    }
    endp = strrchr(path, '/' as i32);
    if endp == path {
        path = path.offset(1);
        return json_pointer_set_single_path(*obj, path, value, array_set_cb, priv_0);
    }
    path_copy = strdup(path);
    if path_copy.is_null() {
        *__errno_location() = ENOMEM;
        return -(1 as ::core::ffi::c_int);
    }
    *path_copy.offset(endp.offset_from(path) as ::core::ffi::c_long as isize) =
        '\0' as i32 as ::core::ffi::c_char;
    rc = json_pointer_object_get_recursive(*obj, path_copy, &raw mut set);
    free(path_copy as *mut ::core::ffi::c_void);
    if rc != 0 {
        return rc;
    }
    endp = endp.offset(1);
    return json_pointer_set_single_path(set, endp, value, array_set_cb, priv_0);
}
#[no_mangle]
pub unsafe extern "C" fn json_pointer_set(
    mut obj: *mut *mut json_object,
    mut path: *const ::core::ffi::c_char,
    mut value: *mut json_object,
) -> ::core::ffi::c_int {
    return json_pointer_set_with_array_cb(
        obj,
        path,
        value,
        Some(
            json_object_array_put_idx_cb
                as unsafe extern "C" fn(
                    *mut json_object,
                    size_t,
                    *mut json_object,
                    *mut ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
        NULL,
    );
}
#[no_mangle]
pub unsafe extern "C" fn json_pointer_setf(
    mut obj: *mut *mut json_object,
    mut value: *mut json_object,
    mut path_fmt: *const ::core::ffi::c_char,
    mut args: ...
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut endp: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut path_copy: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut set: *mut json_object = ::core::ptr::null_mut::<json_object>();
    let mut args_0: ::core::ffi::VaListImpl;
    let mut rc: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if obj.is_null() || path_fmt.is_null() {
        *__errno_location() = EINVAL;
        return -(1 as ::core::ffi::c_int);
    }
    args_0 = args.clone();
    rc = vasprintf(&raw mut path_copy, path_fmt, args_0.as_va_list());
    if rc < 0 as ::core::ffi::c_int {
        return rc;
    }
    if *path_copy.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '\0' as i32 {
        json_object_put(*obj);
        *obj = value;
    } else if *path_copy.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        != '/' as i32
    {
        *__errno_location() = EINVAL;
        rc = -(1 as ::core::ffi::c_int);
    } else {
        endp = strrchr(path_copy, '/' as i32);
        if endp == path_copy {
            set = *obj;
            current_block = 11664691324294961930;
        } else {
            *endp = '\0' as i32 as ::core::ffi::c_char;
            rc = json_pointer_object_get_recursive(*obj, path_copy, &raw mut set);
            if rc != 0 {
                current_block = 16926291421723995522;
            } else {
                current_block = 11664691324294961930;
            }
        }
        match current_block {
            16926291421723995522 => {}
            _ => {
                endp = endp.offset(1);
                rc = json_pointer_set_single_path(
                    set,
                    endp,
                    value,
                    Some(
                        json_object_array_put_idx_cb
                            as unsafe extern "C" fn(
                                *mut json_object,
                                size_t,
                                *mut json_object,
                                *mut ::core::ffi::c_void,
                            )
                                -> ::core::ffi::c_int,
                    ),
                    NULL,
                );
            }
        }
    }
    free(path_copy as *mut ::core::ffi::c_void);
    return rc;
}
