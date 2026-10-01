extern "C" {
    pub type json_object;
    fn printf(__format: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn mc_set_debug(debug: ::core::ffi::c_int);
    fn json_object_put(obj: *mut json_object) -> ::core::ffi::c_int;
    fn json_object_new_object() -> *mut json_object;
    fn json_object_get_object(obj: *const json_object) -> *mut lh_table;
    fn json_object_object_add(
        obj: *mut json_object,
        key: *const ::core::ffi::c_char,
        val: *mut json_object,
    ) -> ::core::ffi::c_int;
    fn json_object_object_del(obj: *mut json_object, key: *const ::core::ffi::c_char);
    fn json_object_new_string(s: *const ::core::ffi::c_char) -> *mut json_object;
}
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
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
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
unsafe fn main_0(
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut my_object: *mut json_object = json_object_new_object() as *mut json_object;
    json_object_object_add(
        my_object as *mut json_object,
        b"foo1\0" as *const u8 as *const ::core::ffi::c_char,
        json_object_new_string(b"bar1\0" as *const u8 as *const ::core::ffi::c_char),
    );
    json_object_object_add(
        my_object as *mut json_object,
        b"foo2\0" as *const u8 as *const ::core::ffi::c_char,
        json_object_new_string(b"bar2\0" as *const u8 as *const ::core::ffi::c_char),
    );
    json_object_object_add(
        my_object as *mut json_object,
        b"deleteme\0" as *const u8 as *const ::core::ffi::c_char,
        json_object_new_string(b"bar2\0" as *const u8 as *const ::core::ffi::c_char),
    );
    json_object_object_add(
        my_object as *mut json_object,
        b"foo3\0" as *const u8 as *const ::core::ffi::c_char,
        json_object_new_string(b"bar3\0" as *const u8 as *const ::core::ffi::c_char),
    );
    printf(
        b"==== delete-in-loop test starting ====\n\0" as *const u8 as *const ::core::ffi::c_char,
    );
    let mut orig_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut key0: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut val0: *mut json_object = ::core::ptr::null_mut::<json_object>();
    let mut entrykey0: *mut lh_entry = lh_table_head(json_object_get_object(my_object));
    let mut entry_nextkey0: *mut lh_entry = ::core::ptr::null_mut::<lh_entry>();
    while !({
        if !entrykey0.is_null() {
            key0 = lh_entry_k(entrykey0) as *mut ::core::ffi::c_char;
            val0 = lh_entry_v(entrykey0) as *mut json_object;
            entry_nextkey0 = lh_entry_next(entrykey0);
        }
        entrykey0
    })
    .is_null()
    {
        printf(
            b"Key at index %d is [%s] %d\0" as *const u8 as *const ::core::ffi::c_char,
            orig_count,
            key0,
            (val0 == NULL as *mut json_object) as ::core::ffi::c_int,
        );
        if strcmp(
            key0,
            b"deleteme\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            json_object_object_del(my_object as *mut json_object, key0);
            printf(b" (deleted)\n\0" as *const u8 as *const ::core::ffi::c_char);
        } else {
            printf(b" (kept)\n\0" as *const u8 as *const ::core::ffi::c_char);
        }
        orig_count += 1;
        entrykey0 = entry_nextkey0;
    }
    printf(
        b"==== replace-value first loop starting ====\n\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    let mut original_key: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    orig_count = 0 as ::core::ffi::c_int;
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
            b"Key at index %d is [%s] %d\n\0" as *const u8 as *const ::core::ffi::c_char,
            orig_count,
            key,
            (val == NULL as *mut json_object) as ::core::ffi::c_int,
        );
        orig_count += 1;
        if !(strcmp(key, b"foo2\0" as *const u8 as *const ::core::ffi::c_char)
            != 0 as ::core::ffi::c_int)
        {
            printf(
                b"replacing value for key [%s]\n\0" as *const u8 as *const ::core::ffi::c_char,
                key,
            );
            original_key = key;
            json_object_object_add(
                my_object as *mut json_object,
                key,
                json_object_new_string(b"zzz\0" as *const u8 as *const ::core::ffi::c_char),
            );
        }
        entrykey = entry_nextkey;
    }
    printf(b"==== second loop starting ====\n\0" as *const u8 as *const ::core::ffi::c_char);
    let mut new_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut retval: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut key2: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut val2: *mut json_object = ::core::ptr::null_mut::<json_object>();
    let mut entrykey2: *mut lh_entry = lh_table_head(json_object_get_object(my_object));
    let mut entry_nextkey2: *mut lh_entry = ::core::ptr::null_mut::<lh_entry>();
    while !({
        if !entrykey2.is_null() {
            key2 = lh_entry_k(entrykey2) as *mut ::core::ffi::c_char;
            val2 = lh_entry_v(entrykey2) as *mut json_object;
            entry_nextkey2 = lh_entry_next(entrykey2);
        }
        entrykey2
    })
    .is_null()
    {
        printf(
            b"Key at index %d is [%s] %d\n\0" as *const u8 as *const ::core::ffi::c_char,
            new_count,
            key2,
            (val2 == NULL as *mut json_object) as ::core::ffi::c_int,
        );
        new_count += 1;
        if !(strcmp(key2, b"foo2\0" as *const u8 as *const ::core::ffi::c_char)
            != 0 as ::core::ffi::c_int)
        {
            printf(
                b"pointer for key [%s] does %smatch\n\0" as *const u8 as *const ::core::ffi::c_char,
                key2,
                if key2 == original_key as *mut ::core::ffi::c_char {
                    b"\0" as *const u8 as *const ::core::ffi::c_char
                } else {
                    b"NOT \0" as *const u8 as *const ::core::ffi::c_char
                },
            );
            if key2 != original_key as *mut ::core::ffi::c_char {
                retval = 1 as ::core::ffi::c_int;
            }
        }
        entrykey2 = entry_nextkey2;
    }
    if new_count != orig_count {
        printf(
            b"mismatch between original count (%d) and new count (%d)\n\0" as *const u8
                as *const ::core::ffi::c_char,
            orig_count,
            new_count,
        );
        retval = 1 as ::core::ffi::c_int;
    }
    json_object_put(my_object as *mut json_object);
    return retval;
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
