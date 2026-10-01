extern "C" {
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strncmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn json_object_get(obj: *mut json_object) -> *mut json_object;
    fn json_object_put(obj: *mut json_object) -> ::core::ffi::c_int;
    fn json_object_is_type(obj: *const json_object, type_0: json_type) -> ::core::ffi::c_int;
    fn json_object_object_get_ex(
        obj: *const json_object,
        key: *const ::core::ffi::c_char,
        value: *mut *mut json_object,
    ) -> json_bool;
    fn json_object_object_del(obj: *mut json_object, key: *const ::core::ffi::c_char);
    fn json_object_array_length(obj: *const json_object) -> size_t;
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
    fn json_object_get_string(obj: *mut json_object) -> *const ::core::ffi::c_char;
    fn json_object_equal(obj1: *mut json_object, obj2: *mut json_object) -> ::core::ffi::c_int;
    fn json_object_deep_copy(
        src: *mut json_object,
        dst: *mut *mut json_object,
        shallow_copy: Option<json_c_shallow_copy_fn>,
    ) -> ::core::ffi::c_int;
    fn json_pointer_get(
        obj: *mut json_object,
        path: *const ::core::ffi::c_char,
        res: *mut *mut json_object,
    ) -> ::core::ffi::c_int;
    fn json_pointer_get_internal(
        obj: *mut json_object,
        path: *const ::core::ffi::c_char,
        res: *mut json_pointer_get_result,
    ) -> ::core::ffi::c_int;
    fn json_pointer_set_with_array_cb(
        obj: *mut *mut json_object,
        path: *const ::core::ffi::c_char,
        value: *mut json_object,
        array_set_cb: json_pointer_array_set_cb,
        priv_0: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
}
pub type size_t = usize;
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
pub type json_c_shallow_copy_fn = unsafe extern "C" fn(
    *mut json_object,
    *mut json_object,
    *const ::core::ffi::c_char,
    size_t,
    *mut *mut json_object,
) -> ::core::ffi::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct json_patch_error {
    pub errno_code: ::core::ffi::c_int,
    pub patch_failure_idx: size_t,
    pub errmsg: *const ::core::ffi::c_char,
}
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
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const ENOENT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const EINVAL: ::core::ffi::c_int = 22 as ::core::ffi::c_int;
pub const ULONG_MAX: ::core::ffi::c_ulong = (__LONG_MAX__ as ::core::ffi::c_ulong)
    .wrapping_mul(2 as ::core::ffi::c_ulong)
    .wrapping_add(1 as ::core::ffi::c_ulong);
pub const SIZE_T_MAX: ::core::ffi::c_ulong = ULONG_MAX;
unsafe extern "C" fn json_patch_apply_test(
    mut res: *mut *mut json_object,
    mut patch_elem: *mut json_object,
    mut path: *const ::core::ffi::c_char,
    mut patch_error: *mut json_patch_error,
) -> ::core::ffi::c_int {
    let mut value1: *mut json_object = ::core::ptr::null_mut::<json_object>();
    let mut value2: *mut json_object = ::core::ptr::null_mut::<json_object>();
    if json_object_object_get_ex(
        patch_elem,
        b"value\0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut value1,
    ) == 0
    {
        (*patch_error).errno_code = 22 as ::core::ffi::c_int;
        (*patch_error).errmsg = b"Patch object does not contain a 'value' field\0" as *const u8
            as *const ::core::ffi::c_char;
        *__errno_location() = 0 as ::core::ffi::c_int;
        return -(1 as ::core::ffi::c_int);
    }
    if json_pointer_get(*res, path, &raw mut value2) != 0 {
        (*patch_error).errno_code = *__errno_location();
        (*patch_error).errmsg = if *__errno_location() == ENOENT {
            b"Did not find element referenced by path field\0" as *const u8
                as *const ::core::ffi::c_char
        } else {
            b"Invalid path field\0" as *const u8 as *const ::core::ffi::c_char
        };
        *__errno_location() = 0 as ::core::ffi::c_int;
        return -(1 as ::core::ffi::c_int);
    }
    if json_object_equal(value1, value2) == 0 {
        (*patch_error).errno_code = 2 as ::core::ffi::c_int;
        (*patch_error).errmsg =
            b"Value of element referenced by 'path' field did not match 'value' field\0"
                as *const u8 as *const ::core::ffi::c_char;
        *__errno_location() = 0 as ::core::ffi::c_int;
        return -(1 as ::core::ffi::c_int);
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn __json_patch_apply_remove(
    mut jpres: *mut json_pointer_get_result,
) -> ::core::ffi::c_int {
    if json_object_is_type((*jpres).parent, json_type_array) != 0 {
        return json_object_array_del_idx(
            (*jpres).parent,
            (*jpres).index_in_parent as size_t,
            1 as size_t,
        );
    } else if !(*jpres).parent.is_null() && !(*jpres).key_in_parent.is_null() {
        json_object_object_del((*jpres).parent, (*jpres).key_in_parent);
        return 0 as ::core::ffi::c_int;
    } else {
        json_object_put((*jpres).obj);
        (*jpres).obj = ::core::ptr::null_mut::<json_object>();
        return 0 as ::core::ffi::c_int;
    };
}
unsafe extern "C" fn json_patch_apply_remove(
    mut res: *mut *mut json_object,
    mut path: *const ::core::ffi::c_char,
    mut patch_error: *mut json_patch_error,
) -> ::core::ffi::c_int {
    let mut jpres: json_pointer_get_result = json_pointer_get_result {
        parent: ::core::ptr::null_mut::<json_object>(),
        obj: ::core::ptr::null_mut::<json_object>(),
        key_in_parent: ::core::ptr::null::<::core::ffi::c_char>(),
        index_in_parent: 0,
    };
    let mut rc: ::core::ffi::c_int = 0;
    if json_pointer_get_internal(*res, path, &raw mut jpres) != 0 {
        (*patch_error).errno_code = *__errno_location();
        (*patch_error).errmsg = if *__errno_location() == ENOENT {
            b"Did not find element referenced by path field\0" as *const u8
                as *const ::core::ffi::c_char
        } else {
            b"Invalid path field\0" as *const u8 as *const ::core::ffi::c_char
        };
        *__errno_location() = 0 as ::core::ffi::c_int;
        return -(1 as ::core::ffi::c_int);
    }
    rc = __json_patch_apply_remove(&raw mut jpres);
    if rc < 0 as ::core::ffi::c_int {
        (*patch_error).errno_code = 22 as ::core::ffi::c_int;
        (*patch_error).errmsg = b"Unable to remove path referenced by 'path' field\0" as *const u8
            as *const ::core::ffi::c_char;
        *__errno_location() = 0 as ::core::ffi::c_int;
    }
    if jpres.parent.is_null() {
        *res = ::core::ptr::null_mut::<json_object>();
    }
    return rc;
}
unsafe extern "C" fn json_object_array_insert_idx_cb(
    mut parent: *mut json_object,
    mut idx: size_t,
    mut value: *mut json_object,
    mut priv_0: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = 0;
    let mut add: *mut ::core::ffi::c_int = priv_0 as *mut ::core::ffi::c_int;
    if idx > json_object_array_length(parent) {
        *__errno_location() = EINVAL;
        return -(1 as ::core::ffi::c_int);
    }
    if *add != 0 {
        rc = json_object_array_insert_idx(parent, idx, value);
    } else {
        rc = json_object_array_put_idx(parent, idx, value);
    }
    if rc < 0 as ::core::ffi::c_int {
        *__errno_location() = EINVAL;
    }
    return rc;
}
unsafe extern "C" fn json_patch_apply_add_replace(
    mut res: *mut *mut json_object,
    mut patch_elem: *mut json_object,
    mut path: *const ::core::ffi::c_char,
    mut add: ::core::ffi::c_int,
    mut patch_error: *mut json_patch_error,
) -> ::core::ffi::c_int {
    let mut value: *mut json_object = ::core::ptr::null_mut::<json_object>();
    let mut rc: ::core::ffi::c_int = 0;
    if json_object_object_get_ex(
        patch_elem,
        b"value\0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut value,
    ) == 0
    {
        (*patch_error).errno_code = 22 as ::core::ffi::c_int;
        (*patch_error).errmsg = b"Patch object does not contain a 'value' field\0" as *const u8
            as *const ::core::ffi::c_char;
        *__errno_location() = 0 as ::core::ffi::c_int;
        return -(1 as ::core::ffi::c_int);
    }
    if add == 0 && json_pointer_get(*res, path, ::core::ptr::null_mut::<*mut json_object>()) != 0 {
        (*patch_error).errno_code = *__errno_location();
        (*patch_error).errmsg = if *__errno_location() == ENOENT {
            b"Did not find element referenced by path field\0" as *const u8
                as *const ::core::ffi::c_char
        } else {
            b"Invalid path field\0" as *const u8 as *const ::core::ffi::c_char
        };
        *__errno_location() = 0 as ::core::ffi::c_int;
        return -(1 as ::core::ffi::c_int);
    }
    rc = json_pointer_set_with_array_cb(
        res,
        path,
        json_object_get(value),
        Some(
            json_object_array_insert_idx_cb
                as unsafe extern "C" fn(
                    *mut json_object,
                    size_t,
                    *mut json_object,
                    *mut ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
        &raw mut add as *mut ::core::ffi::c_void,
    );
    if rc != 0 {
        (*patch_error).errno_code = *__errno_location();
        (*patch_error).errmsg = b"Failed to set value at path referenced by 'path' field\0"
            as *const u8 as *const ::core::ffi::c_char;
        *__errno_location() = 0 as ::core::ffi::c_int;
        json_object_put(value);
    }
    return rc;
}
unsafe extern "C" fn json_object_array_move_cb(
    mut parent: *mut json_object,
    mut idx: size_t,
    mut value: *mut json_object,
    mut priv_0: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = 0;
    let mut from: *mut json_pointer_get_result = priv_0 as *mut json_pointer_get_result;
    let mut len: size_t = json_object_array_length(parent);
    if parent == (*from).parent {
        len = len.wrapping_add(1);
    }
    if idx > len {
        *__errno_location() = EINVAL;
        return -(1 as ::core::ffi::c_int);
    }
    rc = json_object_array_insert_idx(parent, idx, value);
    if rc < 0 as ::core::ffi::c_int {
        *__errno_location() = EINVAL;
    }
    return rc;
}
unsafe extern "C" fn json_patch_apply_move_copy(
    mut res: *mut *mut json_object,
    mut patch_elem: *mut json_object,
    mut path: *const ::core::ffi::c_char,
    mut move_0: ::core::ffi::c_int,
    mut patch_error: *mut json_patch_error,
) -> ::core::ffi::c_int {
    let mut array_set_cb: json_pointer_array_set_cb = None;
    let mut from: json_pointer_get_result = json_pointer_get_result {
        parent: ::core::ptr::null_mut::<json_object>(),
        obj: ::core::ptr::null_mut::<json_object>(),
        key_in_parent: ::core::ptr::null::<::core::ffi::c_char>(),
        index_in_parent: 0,
    };
    let mut jfrom: *mut json_object = ::core::ptr::null_mut::<json_object>();
    let mut from_s: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut from_s_len: size_t = 0;
    let mut rc: ::core::ffi::c_int = 0;
    if json_object_object_get_ex(
        patch_elem,
        b"from\0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut jfrom,
    ) == 0
    {
        (*patch_error).errno_code = 22 as ::core::ffi::c_int;
        (*patch_error).errmsg =
            b"Patch does not contain a 'from' field\0" as *const u8 as *const ::core::ffi::c_char;
        *__errno_location() = 0 as ::core::ffi::c_int;
        return -(1 as ::core::ffi::c_int);
    }
    from_s = json_object_get_string(jfrom);
    from_s_len = strlen(from_s);
    if strncmp(from_s, path, from_s_len) == 0 as ::core::ffi::c_int {
        if from_s_len == strlen(path) {
            return 0 as ::core::ffi::c_int;
        }
        (*patch_error).errno_code = 22 as ::core::ffi::c_int;
        (*patch_error).errmsg = b"Invalid attempt to move parent under a child\0" as *const u8
            as *const ::core::ffi::c_char;
        *__errno_location() = 0 as ::core::ffi::c_int;
        return -(1 as ::core::ffi::c_int);
    }
    rc = json_pointer_get_internal(*res, from_s, &raw mut from);
    if rc != 0 {
        (*patch_error).errno_code = *__errno_location();
        (*patch_error).errmsg = if *__errno_location() == ENOENT {
            b"Did not find element referenced by from field\0" as *const u8
                as *const ::core::ffi::c_char
        } else {
            b"Invalid from field\0" as *const u8 as *const ::core::ffi::c_char
        };
        *__errno_location() = 0 as ::core::ffi::c_int;
        return rc;
    }
    json_object_get(from.obj);
    if move_0 == 0 {
        array_set_cb = Some(
            json_object_array_insert_idx_cb
                as unsafe extern "C" fn(
                    *mut json_object,
                    size_t,
                    *mut json_object,
                    *mut ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ) as json_pointer_array_set_cb;
    } else {
        rc = __json_patch_apply_remove(&raw mut from);
        if rc < 0 as ::core::ffi::c_int {
            json_object_put(from.obj);
            return rc;
        }
        array_set_cb = Some(
            json_object_array_move_cb
                as unsafe extern "C" fn(
                    *mut json_object,
                    size_t,
                    *mut json_object,
                    *mut ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ) as json_pointer_array_set_cb;
    }
    rc = json_pointer_set_with_array_cb(
        res,
        path,
        from.obj,
        array_set_cb,
        &raw mut from as *mut ::core::ffi::c_void,
    );
    if rc != 0 {
        (*patch_error).errno_code = *__errno_location();
        (*patch_error).errmsg = b"Failed to set value at path referenced by 'path' field\0"
            as *const u8 as *const ::core::ffi::c_char;
        *__errno_location() = 0 as ::core::ffi::c_int;
        json_object_put(from.obj);
    }
    return rc;
}
#[no_mangle]
pub unsafe extern "C" fn json_patch_apply(
    mut copy_from: *mut json_object,
    mut patch: *mut json_object,
    mut base: *mut *mut json_object,
    mut patch_error: *mut json_patch_error,
) -> ::core::ffi::c_int {
    let mut ii: size_t = 0;
    let mut rc: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut placeholder: json_patch_error = json_patch_error {
        errno_code: 0,
        patch_failure_idx: 0,
        errmsg: ::core::ptr::null::<::core::ffi::c_char>(),
    };
    if patch_error.is_null() {
        patch_error = &raw mut placeholder;
    }
    (*patch_error).patch_failure_idx = SIZE_T_MAX as size_t;
    (*patch_error).errno_code = 0 as ::core::ffi::c_int;
    if base.is_null()
        || (*base).is_null() && copy_from.is_null()
        || !(*base).is_null() && !copy_from.is_null()
    {
        (*patch_error).errno_code = 14 as ::core::ffi::c_int;
        (*patch_error).errmsg = b"Exactly one of *base or copy_from must be non-NULL\0" as *const u8
            as *const ::core::ffi::c_char;
        *__errno_location() = 0 as ::core::ffi::c_int;
        return -(1 as ::core::ffi::c_int);
    }
    if json_object_is_type(patch, json_type_array) == 0 {
        (*patch_error).errno_code = 14 as ::core::ffi::c_int;
        (*patch_error).errmsg = b"Patch object is not of type json_type_array\0" as *const u8
            as *const ::core::ffi::c_char;
        *__errno_location() = 0 as ::core::ffi::c_int;
        return -(1 as ::core::ffi::c_int);
    }
    if !copy_from.is_null() {
        if json_object_deep_copy(copy_from, base, None) < 0 as ::core::ffi::c_int {
            (*patch_error).errno_code = 12 as ::core::ffi::c_int;
            (*patch_error).errmsg = b"Unable to copy copy_from using json_object_deep_copy()\0"
                as *const u8 as *const ::core::ffi::c_char;
            *__errno_location() = 0 as ::core::ffi::c_int;
            return -(1 as ::core::ffi::c_int);
        }
    }
    ii = 0 as size_t;
    while ii < json_object_array_length(patch) {
        let mut jop: *mut json_object = ::core::ptr::null_mut::<json_object>();
        let mut jpath: *mut json_object = ::core::ptr::null_mut::<json_object>();
        let mut patch_elem: *mut json_object = json_object_array_get_idx(patch, ii);
        let mut op: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
        let mut path: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
        (*patch_error).patch_failure_idx = ii;
        if json_object_object_get_ex(
            patch_elem,
            b"op\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut jop,
        ) == 0
        {
            (*patch_error).errno_code = 22 as ::core::ffi::c_int;
            (*patch_error).errmsg = b"Patch object does not contain 'op' field\0" as *const u8
                as *const ::core::ffi::c_char;
            *__errno_location() = 0 as ::core::ffi::c_int;
            return -(1 as ::core::ffi::c_int);
        }
        op = json_object_get_string(jop);
        if json_object_object_get_ex(
            patch_elem,
            b"path\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut jpath,
        ) == 0
        {
            (*patch_error).errno_code = 22 as ::core::ffi::c_int;
            (*patch_error).errmsg = b"Patch object does not contain 'path' field\0" as *const u8
                as *const ::core::ffi::c_char;
            *__errno_location() = 0 as ::core::ffi::c_int;
            return -(1 as ::core::ffi::c_int);
        }
        path = json_object_get_string(jpath);
        if strcmp(op, b"test\0" as *const u8 as *const ::core::ffi::c_char) == 0 {
            rc = json_patch_apply_test(base, patch_elem, path, patch_error);
        } else if strcmp(op, b"remove\0" as *const u8 as *const ::core::ffi::c_char) == 0 {
            rc = json_patch_apply_remove(base, path, patch_error);
        } else if strcmp(op, b"add\0" as *const u8 as *const ::core::ffi::c_char) == 0 {
            rc = json_patch_apply_add_replace(
                base,
                patch_elem,
                path,
                1 as ::core::ffi::c_int,
                patch_error,
            );
        } else if strcmp(op, b"replace\0" as *const u8 as *const ::core::ffi::c_char) == 0 {
            rc = json_patch_apply_add_replace(
                base,
                patch_elem,
                path,
                0 as ::core::ffi::c_int,
                patch_error,
            );
        } else if strcmp(op, b"move\0" as *const u8 as *const ::core::ffi::c_char) == 0 {
            rc = json_patch_apply_move_copy(
                base,
                patch_elem,
                path,
                1 as ::core::ffi::c_int,
                patch_error,
            );
        } else if strcmp(op, b"copy\0" as *const u8 as *const ::core::ffi::c_char) == 0 {
            rc = json_patch_apply_move_copy(
                base,
                patch_elem,
                path,
                0 as ::core::ffi::c_int,
                patch_error,
            );
        } else {
            (*patch_error).errno_code = 22 as ::core::ffi::c_int;
            (*patch_error).errmsg =
                b"Patch object has invalid 'op' field\0" as *const u8 as *const ::core::ffi::c_char;
            *__errno_location() = 0 as ::core::ffi::c_int;
            return -(1 as ::core::ffi::c_int);
        }
        if rc < 0 as ::core::ffi::c_int {
            break;
        }
        ii = ii.wrapping_add(1);
    }
    return rc;
}
pub const __LONG_MAX__: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
