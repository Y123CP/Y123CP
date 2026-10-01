extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type json_object;
    static mut stderr: *mut FILE;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn json_object_get_type(obj: *const json_object) -> json_type;
    fn json_object_get_object(obj: *const json_object) -> *mut lh_table;
    fn json_object_array_length(obj: *const json_object) -> size_t;
    fn json_object_array_get_idx(obj: *const json_object, idx: size_t) -> *mut json_object;
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
pub type json_type = ::core::ffi::c_uint;
pub const json_type_string: json_type = 6;
pub const json_type_array: json_type = 5;
pub const json_type_object: json_type = 4;
pub const json_type_int: json_type = 3;
pub const json_type_double: json_type = 2;
pub const json_type_boolean: json_type = 1;
pub const json_type_null: json_type = 0;
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
pub type json_c_visit_userfunc = unsafe extern "C" fn(
    *mut json_object,
    ::core::ffi::c_int,
    *mut json_object,
    *const ::core::ffi::c_char,
    *mut size_t,
    *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const JSON_C_VISIT_SECOND: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const JSON_C_VISIT_RETURN_CONTINUE: ::core::ffi::c_int = 0;
pub const JSON_C_VISIT_RETURN_SKIP: ::core::ffi::c_int = 7547;
pub const JSON_C_VISIT_RETURN_POP: ::core::ffi::c_int = 767;
pub const JSON_C_VISIT_RETURN_STOP: ::core::ffi::c_int = 7867;
pub const JSON_C_VISIT_RETURN_ERROR: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
#[no_mangle]
pub unsafe extern "C" fn json_c_visit(
    mut jso: *mut json_object,
    mut future_flags: ::core::ffi::c_int,
    mut userfunc: Option<json_c_visit_userfunc>,
    mut userarg: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = _json_c_visit(
        jso,
        ::core::ptr::null_mut::<json_object>(),
        ::core::ptr::null::<::core::ffi::c_char>(),
        ::core::ptr::null_mut::<size_t>(),
        userfunc,
        userarg,
    );
    match ret {
        JSON_C_VISIT_RETURN_CONTINUE
        | JSON_C_VISIT_RETURN_SKIP
        | JSON_C_VISIT_RETURN_POP
        | JSON_C_VISIT_RETURN_STOP => return 0 as ::core::ffi::c_int,
        _ => return JSON_C_VISIT_RETURN_ERROR,
    };
}
unsafe extern "C" fn _json_c_visit(
    mut jso: *mut json_object,
    mut parent_jso: *mut json_object,
    mut jso_key: *const ::core::ffi::c_char,
    mut jso_index: *mut size_t,
    mut userfunc: Option<json_c_visit_userfunc>,
    mut userarg: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut userret: ::core::ffi::c_int = userfunc.expect("non-null function pointer")(
        jso,
        0 as ::core::ffi::c_int,
        parent_jso,
        jso_key,
        jso_index,
        userarg,
    );
    match userret {
        JSON_C_VISIT_RETURN_CONTINUE => {}
        JSON_C_VISIT_RETURN_SKIP
        | JSON_C_VISIT_RETURN_POP
        | JSON_C_VISIT_RETURN_STOP
        | JSON_C_VISIT_RETURN_ERROR => return userret,
        _ => {
            fprintf(
                stderr,
                b"ERROR: invalid return value from json_c_visit userfunc: %d\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                userret,
            );
            return JSON_C_VISIT_RETURN_ERROR;
        }
    }
    match json_object_get_type(jso) as ::core::ffi::c_uint {
        0 | 1 | 2 | 3 | 6 => return JSON_C_VISIT_RETURN_CONTINUE,
        4 => {
            let mut key: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
            let mut child: *mut json_object = ::core::ptr::null_mut::<json_object>();
            let mut entrykey: *mut lh_entry = lh_table_head(json_object_get_object(jso));
            let mut entry_nextkey: *mut lh_entry = ::core::ptr::null_mut::<lh_entry>();
            while !({
                if !entrykey.is_null() {
                    key = lh_entry_k(entrykey) as *mut ::core::ffi::c_char;
                    child = lh_entry_v(entrykey) as *mut json_object;
                    entry_nextkey = lh_entry_next(entrykey);
                }
                entrykey
            })
            .is_null()
            {
                userret = _json_c_visit(
                    child as *mut json_object,
                    jso,
                    key,
                    ::core::ptr::null_mut::<size_t>(),
                    userfunc,
                    userarg,
                );
                if userret == JSON_C_VISIT_RETURN_POP {
                    break;
                }
                if userret == JSON_C_VISIT_RETURN_STOP || userret == JSON_C_VISIT_RETURN_ERROR {
                    return userret;
                }
                if userret != JSON_C_VISIT_RETURN_CONTINUE && userret != JSON_C_VISIT_RETURN_SKIP {
                    fprintf(
                        stderr,
                        b"INTERNAL ERROR: _json_c_visit returned %d\n\0" as *const u8
                            as *const ::core::ffi::c_char,
                        userret,
                    );
                    return JSON_C_VISIT_RETURN_ERROR;
                }
                entrykey = entry_nextkey;
            }
        }
        5 => {
            let mut array_len: size_t = json_object_array_length(jso);
            let mut ii: size_t = 0;
            ii = 0 as size_t;
            while ii < array_len {
                let mut child_0: *mut json_object =
                    json_object_array_get_idx(jso, ii) as *mut json_object;
                userret = _json_c_visit(
                    child_0,
                    jso,
                    ::core::ptr::null::<::core::ffi::c_char>(),
                    &raw mut ii,
                    userfunc,
                    userarg,
                );
                if userret == JSON_C_VISIT_RETURN_POP {
                    break;
                }
                if userret == JSON_C_VISIT_RETURN_STOP || userret == JSON_C_VISIT_RETURN_ERROR {
                    return userret;
                }
                if userret != JSON_C_VISIT_RETURN_CONTINUE && userret != JSON_C_VISIT_RETURN_SKIP {
                    fprintf(
                        stderr,
                        b"INTERNAL ERROR: _json_c_visit returned %d\n\0" as *const u8
                            as *const ::core::ffi::c_char,
                        userret,
                    );
                    return JSON_C_VISIT_RETURN_ERROR;
                }
                ii = ii.wrapping_add(1);
            }
        }
        _ => {
            fprintf(
                stderr,
                b"INTERNAL ERROR: _json_c_visit found object of unknown type: %d\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                json_object_get_type(jso) as ::core::ffi::c_uint,
            );
            return JSON_C_VISIT_RETURN_ERROR;
        }
    }
    userret = userfunc.expect("non-null function pointer")(
        jso,
        JSON_C_VISIT_SECOND,
        parent_jso,
        jso_key,
        jso_index,
        userarg,
    );
    match userret {
        JSON_C_VISIT_RETURN_SKIP | JSON_C_VISIT_RETURN_POP | JSON_C_VISIT_RETURN_CONTINUE => {
            return JSON_C_VISIT_RETURN_CONTINUE
        }
        JSON_C_VISIT_RETURN_STOP | JSON_C_VISIT_RETURN_ERROR => return userret,
        _ => {
            fprintf(
                stderr,
                b"ERROR: invalid return value from json_c_visit userfunc: %d\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                userret,
            );
            return JSON_C_VISIT_RETURN_ERROR;
        }
    };
}
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
