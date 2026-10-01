extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn __assert_fail(
        __assertion: *const ::core::ffi::c_char,
        __file: *const ::core::ffi::c_char,
        __line: ::core::ffi::c_uint,
        __function: *const ::core::ffi::c_char,
    ) -> !;
    fn realloc(__ptr: *mut ::core::ffi::c_void, __size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn fputc(__c: ::core::ffi::c_int, __stream: *mut FILE) -> ::core::ffi::c_int;
}
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct csv_parser {
    pub pstate: ::core::ffi::c_int,
    pub quoted: ::core::ffi::c_int,
    pub spaces: size_t,
    pub entry_buf: *mut ::core::ffi::c_uchar,
    pub entry_pos: size_t,
    pub entry_size: size_t,
    pub status: ::core::ffi::c_int,
    pub options: ::core::ffi::c_uchar,
    pub quote_char: ::core::ffi::c_uchar,
    pub delim_char: ::core::ffi::c_uchar,
    pub is_space: Option<unsafe extern "C" fn(::core::ffi::c_uchar) -> ::core::ffi::c_int>,
    pub is_term: Option<unsafe extern "C" fn(::core::ffi::c_uchar) -> ::core::ffi::c_int>,
    pub blk_size: size_t,
    pub malloc_func: Option<unsafe extern "C" fn(size_t) -> *mut ::core::ffi::c_void>,
    pub realloc_func:
        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, size_t) -> *mut ::core::ffi::c_void>,
    pub free_func: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
}
pub const SIZE_MAX: ::core::ffi::c_ulong = 18446744073709551615 as ::core::ffi::c_ulong;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const EOF: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
pub const CSV_EPARSE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const CSV_ENOMEM: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const CSV_ETOOBIG: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const CSV_EINVALID: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const CSV_STRICT: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const CSV_REPALL_NL: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const CSV_STRICT_FINI: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const CSV_APPEND_NULL: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const CSV_EMPTY_IS_NULL: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const CSV_TAB: ::core::ffi::c_int = 0x9 as ::core::ffi::c_int;
pub const CSV_SPACE: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
pub const CSV_CR: ::core::ffi::c_int = 0xd as ::core::ffi::c_int;
pub const CSV_LF: ::core::ffi::c_int = 0xa as ::core::ffi::c_int;
pub const CSV_COMMA: ::core::ffi::c_int = 0x2c as ::core::ffi::c_int;
pub const CSV_QUOTE: ::core::ffi::c_int = 0x22 as ::core::ffi::c_int;
pub const ROW_NOT_BEGUN: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const FIELD_NOT_BEGUN: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const FIELD_BEGUN: ::core::ffi::c_int = 2;
pub const FIELD_MIGHT_HAVE_ENDED: ::core::ffi::c_int = 3;
pub const MEM_BLK_SIZE: ::core::ffi::c_int = 128 as ::core::ffi::c_int;
static mut csv_errors: [*const ::core::ffi::c_char; 5] = [
    b"success\0" as *const u8 as *const ::core::ffi::c_char,
    b"error parsing data while strict checking enabled\0" as *const u8
        as *const ::core::ffi::c_char,
    b"memory exhausted while increasing buffer size\0" as *const u8 as *const ::core::ffi::c_char,
    b"data size too large\0" as *const u8 as *const ::core::ffi::c_char,
    b"invalid status code\0" as *const u8 as *const ::core::ffi::c_char,
];
#[no_mangle]
pub unsafe extern "C" fn csv_error(mut p: *const csv_parser) -> ::core::ffi::c_int {
    '_c2rust_label: {
        if !p.is_null()
            && !(b"received null csv_parser\0" as *const u8 as *const ::core::ffi::c_char).is_null()
        {
        } else {
            __assert_fail(
                b"p && \"received null csv_parser\"\0" as *const u8 as *const ::core::ffi::c_char,
                b"libcsv.c\0" as *const u8 as *const ::core::ffi::c_char,
                82 as ::core::ffi::c_uint,
                b"int csv_error(const struct csv_parser *)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    return (*p).status;
}
#[no_mangle]
pub unsafe extern "C" fn csv_strerror(
    mut status: ::core::ffi::c_int,
) -> *const ::core::ffi::c_char {
    if status >= CSV_EINVALID || status < 0 as ::core::ffi::c_int {
        return csv_errors[CSV_EINVALID as usize];
    } else {
        return csv_errors[status as usize];
    };
}
#[no_mangle]
pub unsafe extern "C" fn csv_get_opts(mut p: *const csv_parser) -> ::core::ffi::c_int {
    if p.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    return (*p).options as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn csv_set_opts(
    mut p: *mut csv_parser,
    mut options: ::core::ffi::c_uchar,
) -> ::core::ffi::c_int {
    if p.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    (*p).options = options;
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn csv_init(
    mut p: *mut csv_parser,
    mut options: ::core::ffi::c_uchar,
) -> ::core::ffi::c_int {
    if p.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    (*p).entry_buf = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    (*p).pstate = ROW_NOT_BEGUN;
    (*p).quoted = 0 as ::core::ffi::c_int;
    (*p).spaces = 0 as size_t;
    (*p).entry_pos = 0 as size_t;
    (*p).entry_size = 0 as size_t;
    (*p).status = 0 as ::core::ffi::c_int;
    (*p).options = options;
    (*p).quote_char = CSV_QUOTE as ::core::ffi::c_uchar;
    (*p).delim_char = CSV_COMMA as ::core::ffi::c_uchar;
    (*p).is_space = None;
    (*p).is_term = None;
    (*p).blk_size = MEM_BLK_SIZE as size_t;
    (*p).malloc_func = None;
    (*p).realloc_func = Some(
        realloc
            as unsafe extern "C" fn(*mut ::core::ffi::c_void, size_t) -> *mut ::core::ffi::c_void,
    )
        as Option<
            unsafe extern "C" fn(*mut ::core::ffi::c_void, size_t) -> *mut ::core::ffi::c_void,
        >;
    (*p).free_func = Some(free as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ())
        as Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn csv_free(mut p: *mut csv_parser) {
    if p.is_null() {
        return;
    }
    if !(*p).entry_buf.is_null() && (*p).free_func.is_some() {
        (*p).free_func.expect("non-null function pointer")(
            (*p).entry_buf as *mut ::core::ffi::c_void,
        );
    }
    (*p).entry_buf = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    (*p).entry_size = 0 as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn csv_fini(
    mut p: *mut csv_parser,
    mut cb1: Option<
        unsafe extern "C" fn(*mut ::core::ffi::c_void, size_t, *mut ::core::ffi::c_void) -> (),
    >,
    mut cb2: Option<unsafe extern "C" fn(::core::ffi::c_int, *mut ::core::ffi::c_void) -> ()>,
    mut data: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    if p.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    let mut quoted: ::core::ffi::c_int = (*p).quoted;
    let mut pstate: ::core::ffi::c_int = (*p).pstate;
    let mut spaces: size_t = (*p).spaces;
    let mut entry_pos: size_t = (*p).entry_pos;
    if pstate == FIELD_BEGUN
        && (*p).quoted != 0
        && (*p).options as ::core::ffi::c_int & CSV_STRICT != 0
        && (*p).options as ::core::ffi::c_int & CSV_STRICT_FINI != 0
    {
        (*p).status = CSV_EPARSE;
        return -(1 as ::core::ffi::c_int);
    }
    let mut current_block_26: u64;
    match pstate {
        FIELD_MIGHT_HAVE_ENDED => {
            (*p).entry_pos = ((*p).entry_pos as ::core::ffi::c_ulong)
                .wrapping_sub((*p).spaces.wrapping_add(1 as size_t) as ::core::ffi::c_ulong)
                as size_t as size_t;
            entry_pos = (*p).entry_pos;
            current_block_26 = 17192747335566778533;
        }
        FIELD_NOT_BEGUN | FIELD_BEGUN => {
            current_block_26 = 17192747335566778533;
        }
        ROW_NOT_BEGUN | _ => {
            current_block_26 = 15768484401365413375;
        }
    }
    match current_block_26 {
        17192747335566778533 => {
            if quoted == 0 {
                entry_pos = (entry_pos as ::core::ffi::c_ulong)
                    .wrapping_sub(spaces as ::core::ffi::c_ulong)
                    as size_t as size_t;
            }
            if (*p).options as ::core::ffi::c_int & CSV_APPEND_NULL != 0 {
                *(*p).entry_buf.offset(entry_pos as isize) = '\0' as i32 as ::core::ffi::c_uchar;
            }
            if cb1.is_some()
                && (*p).options as ::core::ffi::c_int & CSV_EMPTY_IS_NULL != 0
                && quoted == 0
                && entry_pos == 0 as size_t
            {
                cb1.expect("non-null function pointer")(NULL, entry_pos, data);
            } else if cb1.is_some() {
                cb1.expect("non-null function pointer")(
                    (*p).entry_buf as *mut ::core::ffi::c_void,
                    entry_pos,
                    data,
                );
            }
            pstate = FIELD_NOT_BEGUN;
            spaces = 0 as size_t;
            quoted = spaces as ::core::ffi::c_int;
            entry_pos = quoted as size_t;
            if cb2.is_some() {
                cb2.expect("non-null function pointer")(-(1 as ::core::ffi::c_int), data);
            }
            pstate = ROW_NOT_BEGUN;
            spaces = 0 as size_t;
            quoted = spaces as ::core::ffi::c_int;
            entry_pos = quoted as size_t;
        }
        _ => {}
    }
    (*p).status = 0 as ::core::ffi::c_int;
    (*p).entry_pos = (*p).status as size_t;
    (*p).quoted = (*p).entry_pos as ::core::ffi::c_int;
    (*p).spaces = (*p).quoted as size_t;
    (*p).pstate = ROW_NOT_BEGUN;
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn csv_set_delim(mut p: *mut csv_parser, mut c: ::core::ffi::c_uchar) {
    if !p.is_null() {
        (*p).delim_char = c;
    }
}
#[no_mangle]
pub unsafe extern "C" fn csv_set_quote(mut p: *mut csv_parser, mut c: ::core::ffi::c_uchar) {
    if !p.is_null() {
        (*p).quote_char = c;
    }
}
#[no_mangle]
pub unsafe extern "C" fn csv_get_delim(mut p: *const csv_parser) -> ::core::ffi::c_uchar {
    '_c2rust_label: {
        if !p.is_null()
            && !(b"received null csv_parser\0" as *const u8 as *const ::core::ffi::c_char).is_null()
        {
        } else {
            __assert_fail(
                b"p && \"received null csv_parser\"\0" as *const u8 as *const ::core::ffi::c_char,
                b"libcsv.c\0" as *const u8 as *const ::core::ffi::c_char,
                222 as ::core::ffi::c_uint,
                b"unsigned char csv_get_delim(const struct csv_parser *)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    return (*p).delim_char;
}
#[no_mangle]
pub unsafe extern "C" fn csv_get_quote(mut p: *const csv_parser) -> ::core::ffi::c_uchar {
    '_c2rust_label: {
        if !p.is_null()
            && !(b"received null csv_parser\0" as *const u8 as *const ::core::ffi::c_char).is_null()
        {
        } else {
            __assert_fail(
                b"p && \"received null csv_parser\"\0" as *const u8 as *const ::core::ffi::c_char,
                b"libcsv.c\0" as *const u8 as *const ::core::ffi::c_char,
                231 as ::core::ffi::c_uint,
                b"unsigned char csv_get_quote(const struct csv_parser *)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    return (*p).quote_char;
}
#[no_mangle]
pub unsafe extern "C" fn csv_set_space_func(
    mut p: *mut csv_parser,
    mut f: Option<unsafe extern "C" fn(::core::ffi::c_uchar) -> ::core::ffi::c_int>,
) {
    if !p.is_null() {
        (*p).is_space = f;
    }
}
#[no_mangle]
pub unsafe extern "C" fn csv_set_term_func(
    mut p: *mut csv_parser,
    mut f: Option<unsafe extern "C" fn(::core::ffi::c_uchar) -> ::core::ffi::c_int>,
) {
    if !p.is_null() {
        (*p).is_term = f;
    }
}
#[no_mangle]
pub unsafe extern "C" fn csv_set_realloc_func(
    mut p: *mut csv_parser,
    mut f: Option<
        unsafe extern "C" fn(*mut ::core::ffi::c_void, size_t) -> *mut ::core::ffi::c_void,
    >,
) {
    if !p.is_null() && f.is_some() {
        (*p).realloc_func = f;
    }
}
#[no_mangle]
pub unsafe extern "C" fn csv_set_free_func(
    mut p: *mut csv_parser,
    mut f: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
) {
    if !p.is_null() && f.is_some() {
        (*p).free_func = f;
    }
}
#[no_mangle]
pub unsafe extern "C" fn csv_set_blk_size(mut p: *mut csv_parser, mut size: size_t) {
    if !p.is_null() {
        (*p).blk_size = size;
    }
}
#[no_mangle]
pub unsafe extern "C" fn csv_get_buffer_size(mut p: *const csv_parser) -> size_t {
    if !p.is_null() {
        return (*p).entry_size;
    }
    return 0 as size_t;
}
unsafe extern "C" fn csv_increase_buffer(mut p: *mut csv_parser) -> ::core::ffi::c_int {
    if p.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    if (*p).realloc_func.is_none() {
        return 0 as ::core::ffi::c_int;
    }
    let mut to_add: size_t = (*p).blk_size;
    let mut vp: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    if (*p).entry_size >= (SIZE_MAX as size_t).wrapping_sub(to_add) {
        to_add = (SIZE_MAX as size_t).wrapping_sub((*p).entry_size);
    }
    if to_add == 0 {
        (*p).status = CSV_ETOOBIG;
        return -(1 as ::core::ffi::c_int);
    }
    loop {
        vp = (*p).realloc_func.expect("non-null function pointer")(
            (*p).entry_buf as *mut ::core::ffi::c_void,
            (*p).entry_size.wrapping_add(to_add),
        );
        if !vp.is_null() {
            break;
        }
        to_add = (to_add as ::core::ffi::c_ulong).wrapping_div(2 as ::core::ffi::c_ulong) as size_t
            as size_t;
        if to_add == 0 {
            (*p).status = CSV_ENOMEM;
            return -(1 as ::core::ffi::c_int);
        }
    }
    (*p).entry_buf = vp as *mut ::core::ffi::c_uchar;
    (*p).entry_size = ((*p).entry_size as ::core::ffi::c_ulong)
        .wrapping_add(to_add as ::core::ffi::c_ulong) as size_t as size_t;
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn csv_parse(
    mut p: *mut csv_parser,
    mut s: *const ::core::ffi::c_void,
    mut len: size_t,
    mut cb1: Option<
        unsafe extern "C" fn(*mut ::core::ffi::c_void, size_t, *mut ::core::ffi::c_void) -> (),
    >,
    mut cb2: Option<unsafe extern "C" fn(::core::ffi::c_int, *mut ::core::ffi::c_void) -> ()>,
    mut data: *mut ::core::ffi::c_void,
) -> size_t {
    '_c2rust_label: {
        if !p.is_null()
            && !(b"received null csv_parser\0" as *const u8 as *const ::core::ffi::c_char).is_null()
        {
        } else {
            __assert_fail(
                b"p && \"received null csv_parser\"\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"libcsv.c\0" as *const u8 as *const ::core::ffi::c_char,
                321 as ::core::ffi::c_uint,
                b"size_t csv_parse(struct csv_parser *, const void *, size_t, void (*)(void *, size_t, void *), void (*)(int, void *), void *)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    if s.is_null() {
        return 0 as size_t;
    }
    let mut us: *const ::core::ffi::c_uchar = s as *const ::core::ffi::c_uchar;
    let mut c: ::core::ffi::c_uchar = 0;
    let mut pos: size_t = 0 as size_t;
    let mut delim: ::core::ffi::c_uchar = (*p).delim_char;
    let mut quote: ::core::ffi::c_uchar = (*p).quote_char;
    let mut is_space: Option<unsafe extern "C" fn(::core::ffi::c_uchar) -> ::core::ffi::c_int> =
        (*p).is_space;
    let mut is_term: Option<unsafe extern "C" fn(::core::ffi::c_uchar) -> ::core::ffi::c_int> =
        (*p).is_term;
    let mut quoted: ::core::ffi::c_int = (*p).quoted;
    let mut pstate: ::core::ffi::c_int = (*p).pstate;
    let mut spaces: size_t = (*p).spaces;
    let mut entry_pos: size_t = (*p).entry_pos;
    if (*p).entry_buf.is_null() && pos < len {
        if csv_increase_buffer(p) != 0 as ::core::ffi::c_int {
            (*p).quoted = quoted;
            (*p).pstate = pstate;
            (*p).spaces = spaces;
            (*p).entry_pos = entry_pos;
            return pos;
        }
    }
    while pos < len {
        if entry_pos
            == (if (*p).options as ::core::ffi::c_int & CSV_APPEND_NULL != 0 {
                (*p).entry_size.wrapping_sub(1 as size_t)
            } else {
                (*p).entry_size
            })
        {
            if csv_increase_buffer(p) != 0 as ::core::ffi::c_int {
                (*p).quoted = quoted;
                (*p).pstate = pstate;
                (*p).spaces = spaces;
                (*p).entry_pos = entry_pos;
                return pos;
            }
        }
        let fresh0 = pos;
        pos = pos.wrapping_add(1);
        c = *us.offset(fresh0 as isize);
        match pstate {
            ROW_NOT_BEGUN | FIELD_NOT_BEGUN => {
                if (if is_space.is_some() {
                    is_space.expect("non-null function pointer")(c)
                } else {
                    (c as ::core::ffi::c_int == CSV_SPACE || c as ::core::ffi::c_int == CSV_TAB)
                        as ::core::ffi::c_int
                }) != 0
                    && c as ::core::ffi::c_int != delim as ::core::ffi::c_int
                {
                    continue;
                }
                if if is_term.is_some() {
                    is_term.expect("non-null function pointer")(c)
                } else {
                    (c as ::core::ffi::c_int == CSV_CR || c as ::core::ffi::c_int == CSV_LF)
                        as ::core::ffi::c_int
                } != 0
                {
                    if pstate == FIELD_NOT_BEGUN {
                        if quoted == 0 {
                            entry_pos = (entry_pos as ::core::ffi::c_ulong)
                                .wrapping_sub(spaces as ::core::ffi::c_ulong)
                                as size_t as size_t;
                        }
                        if (*p).options as ::core::ffi::c_int & CSV_APPEND_NULL != 0 {
                            *(*p).entry_buf.offset(entry_pos as isize) =
                                '\0' as i32 as ::core::ffi::c_uchar;
                        }
                        if cb1.is_some()
                            && (*p).options as ::core::ffi::c_int & CSV_EMPTY_IS_NULL != 0
                            && quoted == 0
                            && entry_pos == 0 as size_t
                        {
                            cb1.expect("non-null function pointer")(NULL, entry_pos, data);
                        } else if cb1.is_some() {
                            cb1.expect("non-null function pointer")(
                                (*p).entry_buf as *mut ::core::ffi::c_void,
                                entry_pos,
                                data,
                            );
                        }
                        pstate = FIELD_NOT_BEGUN;
                        spaces = 0 as size_t;
                        quoted = spaces as ::core::ffi::c_int;
                        entry_pos = quoted as size_t;
                        if cb2.is_some() {
                            cb2.expect("non-null function pointer")(c as ::core::ffi::c_int, data);
                        }
                        pstate = ROW_NOT_BEGUN;
                        spaces = 0 as size_t;
                        quoted = spaces as ::core::ffi::c_int;
                        entry_pos = quoted as size_t;
                    } else if (*p).options as ::core::ffi::c_int & CSV_REPALL_NL != 0 {
                        if cb2.is_some() {
                            cb2.expect("non-null function pointer")(c as ::core::ffi::c_int, data);
                        }
                        pstate = ROW_NOT_BEGUN;
                        spaces = 0 as size_t;
                        quoted = spaces as ::core::ffi::c_int;
                        entry_pos = quoted as size_t;
                    }
                } else if c as ::core::ffi::c_int == delim as ::core::ffi::c_int {
                    if quoted == 0 {
                        entry_pos = (entry_pos as ::core::ffi::c_ulong)
                            .wrapping_sub(spaces as ::core::ffi::c_ulong)
                            as size_t as size_t;
                    }
                    if (*p).options as ::core::ffi::c_int & CSV_APPEND_NULL != 0 {
                        *(*p).entry_buf.offset(entry_pos as isize) =
                            '\0' as i32 as ::core::ffi::c_uchar;
                    }
                    if cb1.is_some()
                        && (*p).options as ::core::ffi::c_int & CSV_EMPTY_IS_NULL != 0
                        && quoted == 0
                        && entry_pos == 0 as size_t
                    {
                        cb1.expect("non-null function pointer")(NULL, entry_pos, data);
                    } else if cb1.is_some() {
                        cb1.expect("non-null function pointer")(
                            (*p).entry_buf as *mut ::core::ffi::c_void,
                            entry_pos,
                            data,
                        );
                    }
                    pstate = FIELD_NOT_BEGUN;
                    spaces = 0 as size_t;
                    quoted = spaces as ::core::ffi::c_int;
                    entry_pos = quoted as size_t;
                } else if c as ::core::ffi::c_int == quote as ::core::ffi::c_int {
                    pstate = FIELD_BEGUN;
                    quoted = 1 as ::core::ffi::c_int;
                } else {
                    pstate = FIELD_BEGUN;
                    quoted = 0 as ::core::ffi::c_int;
                    let fresh1 = entry_pos;
                    entry_pos = entry_pos.wrapping_add(1);
                    *(*p).entry_buf.offset(fresh1 as isize) = c;
                }
            }
            FIELD_BEGUN => {
                if c as ::core::ffi::c_int == quote as ::core::ffi::c_int {
                    if quoted != 0 {
                        let fresh2 = entry_pos;
                        entry_pos = entry_pos.wrapping_add(1);
                        *(*p).entry_buf.offset(fresh2 as isize) = c;
                        pstate = FIELD_MIGHT_HAVE_ENDED;
                    } else {
                        if (*p).options as ::core::ffi::c_int & CSV_STRICT != 0 {
                            (*p).status = CSV_EPARSE;
                            (*p).quoted = quoted;
                            (*p).pstate = pstate;
                            (*p).spaces = spaces;
                            (*p).entry_pos = entry_pos;
                            return pos.wrapping_sub(1 as size_t);
                        }
                        let fresh3 = entry_pos;
                        entry_pos = entry_pos.wrapping_add(1);
                        *(*p).entry_buf.offset(fresh3 as isize) = c;
                        spaces = 0 as size_t;
                    }
                } else if c as ::core::ffi::c_int == delim as ::core::ffi::c_int {
                    if quoted != 0 {
                        let fresh4 = entry_pos;
                        entry_pos = entry_pos.wrapping_add(1);
                        *(*p).entry_buf.offset(fresh4 as isize) = c;
                    } else {
                        if quoted == 0 {
                            entry_pos = (entry_pos as ::core::ffi::c_ulong)
                                .wrapping_sub(spaces as ::core::ffi::c_ulong)
                                as size_t as size_t;
                        }
                        if (*p).options as ::core::ffi::c_int & CSV_APPEND_NULL != 0 {
                            *(*p).entry_buf.offset(entry_pos as isize) =
                                '\0' as i32 as ::core::ffi::c_uchar;
                        }
                        if cb1.is_some()
                            && (*p).options as ::core::ffi::c_int & CSV_EMPTY_IS_NULL != 0
                            && quoted == 0
                            && entry_pos == 0 as size_t
                        {
                            cb1.expect("non-null function pointer")(NULL, entry_pos, data);
                        } else if cb1.is_some() {
                            cb1.expect("non-null function pointer")(
                                (*p).entry_buf as *mut ::core::ffi::c_void,
                                entry_pos,
                                data,
                            );
                        }
                        pstate = FIELD_NOT_BEGUN;
                        spaces = 0 as size_t;
                        quoted = spaces as ::core::ffi::c_int;
                        entry_pos = quoted as size_t;
                    }
                } else if if is_term.is_some() {
                    is_term.expect("non-null function pointer")(c)
                } else {
                    (c as ::core::ffi::c_int == CSV_CR || c as ::core::ffi::c_int == CSV_LF)
                        as ::core::ffi::c_int
                } != 0
                {
                    if quoted == 0 {
                        if quoted == 0 {
                            entry_pos = (entry_pos as ::core::ffi::c_ulong)
                                .wrapping_sub(spaces as ::core::ffi::c_ulong)
                                as size_t as size_t;
                        }
                        if (*p).options as ::core::ffi::c_int & CSV_APPEND_NULL != 0 {
                            *(*p).entry_buf.offset(entry_pos as isize) =
                                '\0' as i32 as ::core::ffi::c_uchar;
                        }
                        if cb1.is_some()
                            && (*p).options as ::core::ffi::c_int & CSV_EMPTY_IS_NULL != 0
                            && quoted == 0
                            && entry_pos == 0 as size_t
                        {
                            cb1.expect("non-null function pointer")(NULL, entry_pos, data);
                        } else if cb1.is_some() {
                            cb1.expect("non-null function pointer")(
                                (*p).entry_buf as *mut ::core::ffi::c_void,
                                entry_pos,
                                data,
                            );
                        }
                        pstate = FIELD_NOT_BEGUN;
                        spaces = 0 as size_t;
                        quoted = spaces as ::core::ffi::c_int;
                        entry_pos = quoted as size_t;
                        if cb2.is_some() {
                            cb2.expect("non-null function pointer")(c as ::core::ffi::c_int, data);
                        }
                        pstate = ROW_NOT_BEGUN;
                        spaces = 0 as size_t;
                        quoted = spaces as ::core::ffi::c_int;
                        entry_pos = quoted as size_t;
                    } else {
                        let fresh5 = entry_pos;
                        entry_pos = entry_pos.wrapping_add(1);
                        *(*p).entry_buf.offset(fresh5 as isize) = c;
                    }
                } else if quoted == 0
                    && (if is_space.is_some() {
                        is_space.expect("non-null function pointer")(c)
                    } else {
                        (c as ::core::ffi::c_int == CSV_SPACE || c as ::core::ffi::c_int == CSV_TAB)
                            as ::core::ffi::c_int
                    }) != 0
                {
                    let fresh6 = entry_pos;
                    entry_pos = entry_pos.wrapping_add(1);
                    *(*p).entry_buf.offset(fresh6 as isize) = c;
                    spaces = spaces.wrapping_add(1);
                } else {
                    let fresh7 = entry_pos;
                    entry_pos = entry_pos.wrapping_add(1);
                    *(*p).entry_buf.offset(fresh7 as isize) = c;
                    spaces = 0 as size_t;
                }
            }
            FIELD_MIGHT_HAVE_ENDED => {
                if c as ::core::ffi::c_int == delim as ::core::ffi::c_int {
                    entry_pos = (entry_pos as ::core::ffi::c_ulong)
                        .wrapping_sub(spaces.wrapping_add(1 as size_t) as ::core::ffi::c_ulong)
                        as size_t as size_t;
                    if quoted == 0 {
                        entry_pos = (entry_pos as ::core::ffi::c_ulong)
                            .wrapping_sub(spaces as ::core::ffi::c_ulong)
                            as size_t as size_t;
                    }
                    if (*p).options as ::core::ffi::c_int & CSV_APPEND_NULL != 0 {
                        *(*p).entry_buf.offset(entry_pos as isize) =
                            '\0' as i32 as ::core::ffi::c_uchar;
                    }
                    if cb1.is_some()
                        && (*p).options as ::core::ffi::c_int & CSV_EMPTY_IS_NULL != 0
                        && quoted == 0
                        && entry_pos == 0 as size_t
                    {
                        cb1.expect("non-null function pointer")(NULL, entry_pos, data);
                    } else if cb1.is_some() {
                        cb1.expect("non-null function pointer")(
                            (*p).entry_buf as *mut ::core::ffi::c_void,
                            entry_pos,
                            data,
                        );
                    }
                    pstate = FIELD_NOT_BEGUN;
                    spaces = 0 as size_t;
                    quoted = spaces as ::core::ffi::c_int;
                    entry_pos = quoted as size_t;
                } else if if is_term.is_some() {
                    is_term.expect("non-null function pointer")(c)
                } else {
                    (c as ::core::ffi::c_int == CSV_CR || c as ::core::ffi::c_int == CSV_LF)
                        as ::core::ffi::c_int
                } != 0
                {
                    entry_pos = (entry_pos as ::core::ffi::c_ulong)
                        .wrapping_sub(spaces.wrapping_add(1 as size_t) as ::core::ffi::c_ulong)
                        as size_t as size_t;
                    if quoted == 0 {
                        entry_pos = (entry_pos as ::core::ffi::c_ulong)
                            .wrapping_sub(spaces as ::core::ffi::c_ulong)
                            as size_t as size_t;
                    }
                    if (*p).options as ::core::ffi::c_int & CSV_APPEND_NULL != 0 {
                        *(*p).entry_buf.offset(entry_pos as isize) =
                            '\0' as i32 as ::core::ffi::c_uchar;
                    }
                    if cb1.is_some()
                        && (*p).options as ::core::ffi::c_int & CSV_EMPTY_IS_NULL != 0
                        && quoted == 0
                        && entry_pos == 0 as size_t
                    {
                        cb1.expect("non-null function pointer")(NULL, entry_pos, data);
                    } else if cb1.is_some() {
                        cb1.expect("non-null function pointer")(
                            (*p).entry_buf as *mut ::core::ffi::c_void,
                            entry_pos,
                            data,
                        );
                    }
                    pstate = FIELD_NOT_BEGUN;
                    spaces = 0 as size_t;
                    quoted = spaces as ::core::ffi::c_int;
                    entry_pos = quoted as size_t;
                    if cb2.is_some() {
                        cb2.expect("non-null function pointer")(c as ::core::ffi::c_int, data);
                    }
                    pstate = ROW_NOT_BEGUN;
                    spaces = 0 as size_t;
                    quoted = spaces as ::core::ffi::c_int;
                    entry_pos = quoted as size_t;
                } else if if is_space.is_some() {
                    is_space.expect("non-null function pointer")(c)
                } else {
                    (c as ::core::ffi::c_int == CSV_SPACE || c as ::core::ffi::c_int == CSV_TAB)
                        as ::core::ffi::c_int
                } != 0
                {
                    let fresh8 = entry_pos;
                    entry_pos = entry_pos.wrapping_add(1);
                    *(*p).entry_buf.offset(fresh8 as isize) = c;
                    spaces = spaces.wrapping_add(1);
                } else if c as ::core::ffi::c_int == quote as ::core::ffi::c_int {
                    if spaces != 0 {
                        if (*p).options as ::core::ffi::c_int & CSV_STRICT != 0 {
                            (*p).status = CSV_EPARSE;
                            (*p).quoted = quoted;
                            (*p).pstate = pstate;
                            (*p).spaces = spaces;
                            (*p).entry_pos = entry_pos;
                            return pos.wrapping_sub(1 as size_t);
                        }
                        spaces = 0 as size_t;
                        let fresh9 = entry_pos;
                        entry_pos = entry_pos.wrapping_add(1);
                        *(*p).entry_buf.offset(fresh9 as isize) = c;
                    } else {
                        pstate = FIELD_BEGUN;
                    }
                } else {
                    if (*p).options as ::core::ffi::c_int & CSV_STRICT != 0 {
                        (*p).status = CSV_EPARSE;
                        (*p).quoted = quoted;
                        (*p).pstate = pstate;
                        (*p).spaces = spaces;
                        (*p).entry_pos = entry_pos;
                        return pos.wrapping_sub(1 as size_t);
                    }
                    pstate = FIELD_BEGUN;
                    spaces = 0 as size_t;
                    let fresh10 = entry_pos;
                    entry_pos = entry_pos.wrapping_add(1);
                    *(*p).entry_buf.offset(fresh10 as isize) = c;
                }
            }
            _ => {}
        }
    }
    (*p).quoted = quoted;
    (*p).pstate = pstate;
    (*p).spaces = spaces;
    (*p).entry_pos = entry_pos;
    return pos;
}
#[no_mangle]
pub unsafe extern "C" fn csv_write(
    mut dest: *mut ::core::ffi::c_void,
    mut dest_size: size_t,
    mut src: *const ::core::ffi::c_void,
    mut src_size: size_t,
) -> size_t {
    return csv_write2(
        dest,
        dest_size,
        src,
        src_size,
        CSV_QUOTE as ::core::ffi::c_uchar,
    );
}
#[no_mangle]
pub unsafe extern "C" fn csv_fwrite(
    mut fp: *mut FILE,
    mut src: *const ::core::ffi::c_void,
    mut src_size: size_t,
) -> ::core::ffi::c_int {
    return csv_fwrite2(fp, src, src_size, CSV_QUOTE as ::core::ffi::c_uchar);
}
#[no_mangle]
pub unsafe extern "C" fn csv_write2(
    mut dest: *mut ::core::ffi::c_void,
    mut dest_size: size_t,
    mut src: *const ::core::ffi::c_void,
    mut src_size: size_t,
    mut quote: ::core::ffi::c_uchar,
) -> size_t {
    let mut cdest: *mut ::core::ffi::c_uchar = dest as *mut ::core::ffi::c_uchar;
    let mut csrc: *const ::core::ffi::c_uchar = src as *const ::core::ffi::c_uchar;
    let mut chars: size_t = 0 as size_t;
    if src.is_null() {
        return 0 as size_t;
    }
    if dest.is_null() {
        dest_size = 0 as size_t;
    }
    if dest_size > 0 as size_t {
        let fresh11 = cdest;
        cdest = cdest.offset(1);
        *fresh11 = quote;
    }
    chars = chars.wrapping_add(1);
    while src_size != 0 {
        if *csrc as ::core::ffi::c_int == quote as ::core::ffi::c_int {
            if dest_size > chars {
                let fresh12 = cdest;
                cdest = cdest.offset(1);
                *fresh12 = quote;
            }
            if chars < SIZE_MAX as size_t {
                chars = chars.wrapping_add(1);
            }
        }
        if dest_size > chars {
            let fresh13 = cdest;
            cdest = cdest.offset(1);
            *fresh13 = *csrc;
        }
        if chars < SIZE_MAX as size_t {
            chars = chars.wrapping_add(1);
        }
        src_size = src_size.wrapping_sub(1);
        csrc = csrc.offset(1);
    }
    if dest_size > chars {
        *cdest = quote;
    }
    if chars < SIZE_MAX as size_t {
        chars = chars.wrapping_add(1);
    }
    return chars;
}
#[no_mangle]
pub unsafe extern "C" fn csv_fwrite2(
    mut fp: *mut FILE,
    mut src: *const ::core::ffi::c_void,
    mut src_size: size_t,
    mut quote: ::core::ffi::c_uchar,
) -> ::core::ffi::c_int {
    let mut csrc: *const ::core::ffi::c_uchar = src as *const ::core::ffi::c_uchar;
    if fp.is_null() || src.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    if fputc(quote as ::core::ffi::c_int, fp) == EOF {
        return EOF;
    }
    while src_size != 0 {
        if *csrc as ::core::ffi::c_int == quote as ::core::ffi::c_int {
            if fputc(quote as ::core::ffi::c_int, fp) == EOF {
                return EOF;
            }
        }
        if fputc(*csrc as ::core::ffi::c_int, fp) == EOF {
            return EOF;
        }
        src_size = src_size.wrapping_sub(1);
        csrc = csrc.offset(1);
    }
    if fputc(quote as ::core::ffi::c_int, fp) == EOF {
        return EOF;
    }
    return 0 as ::core::ffi::c_int;
}
