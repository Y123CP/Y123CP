extern "C" {
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn realloc(__ptr: *mut ::core::ffi::c_void, __size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn qsort(
        __base: *mut ::core::ffi::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    fn memmove(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
}
pub type size_t = usize;
pub type __compar_fn_t = Option<
    unsafe extern "C" fn(
        *const ::core::ffi::c_void,
        *const ::core::ffi::c_void,
    ) -> ::core::ffi::c_int,
>;
pub type array_list_free_fn = unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct array_list {
    pub array: *mut *mut ::core::ffi::c_void,
    pub length: size_t,
    pub size: size_t,
    pub free_fn: Option<array_list_free_fn>,
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL_0: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
#[inline]
unsafe extern "C" fn bsearch(
    mut __key: *const ::core::ffi::c_void,
    mut __base: *const ::core::ffi::c_void,
    mut __nmemb: size_t,
    mut __size: size_t,
    mut __compar: __compar_fn_t,
) -> *mut ::core::ffi::c_void {
    let mut __l: size_t = 0;
    let mut __u: size_t = 0;
    let mut __idx: size_t = 0;
    let mut __p: *const ::core::ffi::c_void = ::core::ptr::null::<::core::ffi::c_void>();
    let mut __comparison: ::core::ffi::c_int = 0;
    __l = 0 as size_t;
    __u = __nmemb;
    while __l < __u {
        __idx = __l.wrapping_add(__u).wrapping_div(2 as size_t);
        __p = (__base as *const ::core::ffi::c_char).offset(__idx.wrapping_mul(__size) as isize)
            as *mut ::core::ffi::c_void;
        __comparison = Some(__compar.expect("non-null function pointer"))
            .expect("non-null function pointer")(__key, __p);
        if __comparison < 0 as ::core::ffi::c_int {
            __u = __idx;
        } else if __comparison > 0 as ::core::ffi::c_int {
            __l = __idx.wrapping_add(1 as size_t);
        } else {
            return __p as *mut ::core::ffi::c_void;
        }
    }
    return NULL;
}
pub const ARRAY_LIST_DEFAULT_SIZE: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
pub const SIZE_T_MAX: ::core::ffi::c_ulong = ULONG_MAX;
#[no_mangle]
pub unsafe extern "C" fn array_list_new(
    mut free_fn: Option<array_list_free_fn>,
) -> *mut array_list {
    return array_list_new2(free_fn, ARRAY_LIST_DEFAULT_SIZE);
}
#[no_mangle]
pub unsafe extern "C" fn array_list_new2(
    mut free_fn: Option<array_list_free_fn>,
    mut initial_size: ::core::ffi::c_int,
) -> *mut array_list {
    let mut arr: *mut array_list = ::core::ptr::null_mut::<array_list>();
    if initial_size < 0 as ::core::ffi::c_int
        || initial_size as size_t
            >= (SIZE_T_MAX as usize)
                .wrapping_div(::core::mem::size_of::<*mut ::core::ffi::c_void>() as usize)
    {
        return ::core::ptr::null_mut::<array_list>();
    }
    arr = malloc(::core::mem::size_of::<array_list>() as size_t) as *mut array_list;
    if arr.is_null() {
        return ::core::ptr::null_mut::<array_list>();
    }
    (*arr).size = initial_size as size_t;
    (*arr).length = 0 as size_t;
    (*arr).free_fn = free_fn;
    (*arr).array = malloc(
        (*arr)
            .size
            .wrapping_mul(::core::mem::size_of::<*mut ::core::ffi::c_void>() as size_t),
    ) as *mut *mut ::core::ffi::c_void;
    if (*arr).array.is_null() {
        free(arr as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<array_list>();
    }
    return arr;
}
#[no_mangle]
pub unsafe extern "C" fn array_list_free(mut arr: *mut array_list) {
    let mut i: size_t = 0;
    i = 0 as size_t;
    while i < (*arr).length {
        if !(*(*arr).array.offset(i as isize)).is_null() {
            (*arr).free_fn.expect("non-null function pointer")(*(*arr).array.offset(i as isize));
        }
        i = i.wrapping_add(1);
    }
    free((*arr).array as *mut ::core::ffi::c_void);
    free(arr as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn array_list_get_idx(
    mut arr: *mut array_list,
    mut i: size_t,
) -> *mut ::core::ffi::c_void {
    if i >= (*arr).length {
        return NULL_0;
    }
    return *(*arr).array.offset(i as isize);
}
unsafe extern "C" fn array_list_expand_internal(
    mut arr: *mut array_list,
    mut max: size_t,
) -> ::core::ffi::c_int {
    let mut t: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut new_size: size_t = 0;
    if max < (*arr).size {
        return 0 as ::core::ffi::c_int;
    }
    if (*arr).size >= (SIZE_T_MAX as size_t).wrapping_div(2 as size_t) {
        new_size = max;
    } else {
        new_size = (*arr).size << 1 as ::core::ffi::c_int;
        if new_size < max {
            new_size = max;
        }
    }
    if new_size
        > (!(0 as ::core::ffi::c_int as size_t))
            .wrapping_div(::core::mem::size_of::<*mut ::core::ffi::c_void>() as size_t)
    {
        return -(1 as ::core::ffi::c_int);
    }
    t = realloc(
        (*arr).array as *mut ::core::ffi::c_void,
        new_size.wrapping_mul(::core::mem::size_of::<*mut ::core::ffi::c_void>() as size_t),
    );
    if t.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    (*arr).array = t as *mut *mut ::core::ffi::c_void;
    (*arr).size = new_size;
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn array_list_shrink(
    mut arr: *mut array_list,
    mut empty_slots: size_t,
) -> ::core::ffi::c_int {
    let mut t: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut new_size: size_t = 0;
    if empty_slots
        >= (SIZE_T_MAX as usize)
            .wrapping_div(::core::mem::size_of::<*mut ::core::ffi::c_void>() as usize)
            .wrapping_sub((*arr).length as usize)
    {
        return -(1 as ::core::ffi::c_int);
    }
    new_size = (*arr).length.wrapping_add(empty_slots);
    if new_size == (*arr).size {
        return 0 as ::core::ffi::c_int;
    }
    if new_size > (*arr).size {
        return array_list_expand_internal(arr, new_size);
    }
    if new_size == 0 as size_t {
        new_size = 1 as size_t;
    }
    t = realloc(
        (*arr).array as *mut ::core::ffi::c_void,
        new_size.wrapping_mul(::core::mem::size_of::<*mut ::core::ffi::c_void>() as size_t),
    );
    if t.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    (*arr).array = t as *mut *mut ::core::ffi::c_void;
    (*arr).size = new_size;
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn array_list_insert_idx(
    mut arr: *mut array_list,
    mut idx: size_t,
    mut data: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut move_amount: size_t = 0;
    if idx >= (*arr).length {
        return array_list_put_idx(arr, idx, data);
    }
    if (*arr).length == SIZE_T_MAX as size_t {
        return -(1 as ::core::ffi::c_int);
    }
    if array_list_expand_internal(arr, (*arr).length.wrapping_add(1 as size_t)) != 0 {
        return -(1 as ::core::ffi::c_int);
    }
    move_amount = (*arr)
        .length
        .wrapping_sub(idx)
        .wrapping_mul(::core::mem::size_of::<*mut ::core::ffi::c_void>() as size_t);
    memmove(
        (*arr)
            .array
            .offset(idx as isize)
            .offset(1 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_void,
        (*arr).array.offset(idx as isize) as *const ::core::ffi::c_void,
        move_amount,
    );
    let ref mut fresh0 = *(*arr).array.offset(idx as isize);
    *fresh0 = data;
    (*arr).length = (*arr).length.wrapping_add(1);
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn array_list_put_idx(
    mut arr: *mut array_list,
    mut idx: size_t,
    mut data: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    if idx > (SIZE_T_MAX as size_t).wrapping_sub(1 as size_t) {
        return -(1 as ::core::ffi::c_int);
    }
    if array_list_expand_internal(arr, idx.wrapping_add(1 as size_t)) != 0 {
        return -(1 as ::core::ffi::c_int);
    }
    if idx < (*arr).length && !(*(*arr).array.offset(idx as isize)).is_null() {
        (*arr).free_fn.expect("non-null function pointer")(*(*arr).array.offset(idx as isize));
    }
    let ref mut fresh1 = *(*arr).array.offset(idx as isize);
    *fresh1 = data;
    if idx > (*arr).length {
        memset(
            (*arr).array.offset((*arr).length as isize) as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            idx.wrapping_sub((*arr).length)
                .wrapping_mul(::core::mem::size_of::<*mut ::core::ffi::c_void>() as size_t),
        );
    }
    if (*arr).length <= idx {
        (*arr).length = idx.wrapping_add(1 as size_t);
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn array_list_add(
    mut arr: *mut array_list,
    mut data: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut idx: size_t = (*arr).length;
    if idx > (SIZE_T_MAX as size_t).wrapping_sub(1 as size_t) {
        return -(1 as ::core::ffi::c_int);
    }
    if array_list_expand_internal(arr, idx.wrapping_add(1 as size_t)) != 0 {
        return -(1 as ::core::ffi::c_int);
    }
    let ref mut fresh2 = *(*arr).array.offset(idx as isize);
    *fresh2 = data;
    (*arr).length = (*arr).length.wrapping_add(1);
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn array_list_sort(
    mut arr: *mut array_list,
    mut compar: Option<
        unsafe extern "C" fn(
            *const ::core::ffi::c_void,
            *const ::core::ffi::c_void,
        ) -> ::core::ffi::c_int,
    >,
) {
    qsort(
        (*arr).array as *mut ::core::ffi::c_void,
        (*arr).length,
        ::core::mem::size_of::<*mut ::core::ffi::c_void>() as size_t,
        compar as __compar_fn_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn array_list_bsearch(
    mut key: *mut *const ::core::ffi::c_void,
    mut arr: *mut array_list,
    mut compar: Option<
        unsafe extern "C" fn(
            *const ::core::ffi::c_void,
            *const ::core::ffi::c_void,
        ) -> ::core::ffi::c_int,
    >,
) -> *mut ::core::ffi::c_void {
    return bsearch(
        key as *const ::core::ffi::c_void,
        (*arr).array as *const ::core::ffi::c_void,
        (*arr).length,
        ::core::mem::size_of::<*mut ::core::ffi::c_void>() as size_t,
        compar as __compar_fn_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn array_list_length(mut arr: *mut array_list) -> size_t {
    return (*arr).length;
}
#[no_mangle]
pub unsafe extern "C" fn array_list_del_idx(
    mut arr: *mut array_list,
    mut idx: size_t,
    mut count: size_t,
) -> ::core::ffi::c_int {
    let mut i: size_t = 0;
    let mut stop: size_t = 0;
    if idx > (SIZE_T_MAX as size_t).wrapping_sub(count) {
        return -(1 as ::core::ffi::c_int);
    }
    stop = idx.wrapping_add(count);
    if idx >= (*arr).length || stop > (*arr).length {
        return -(1 as ::core::ffi::c_int);
    }
    i = idx;
    while i < stop {
        if !(*(*arr).array.offset(i as isize)).is_null() {
            (*arr).free_fn.expect("non-null function pointer")(*(*arr).array.offset(i as isize));
        }
        i = i.wrapping_add(1);
    }
    memmove(
        (*arr).array.offset(idx as isize) as *mut ::core::ffi::c_void,
        (*arr).array.offset(stop as isize) as *const ::core::ffi::c_void,
        (*arr)
            .length
            .wrapping_sub(stop)
            .wrapping_mul(::core::mem::size_of::<*mut ::core::ffi::c_void>() as size_t),
    );
    (*arr).length = ((*arr).length as ::core::ffi::c_ulong)
        .wrapping_sub(count as ::core::ffi::c_ulong) as size_t as size_t;
    return 0 as ::core::ffi::c_int;
}
pub const __LONG_MAX__: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
pub const ULONG_MAX: ::core::ffi::c_ulong = (__LONG_MAX__ as ::core::ffi::c_ulong)
    .wrapping_mul(2 as ::core::ffi::c_ulong)
    .wrapping_add(1 as ::core::ffi::c_ulong);
