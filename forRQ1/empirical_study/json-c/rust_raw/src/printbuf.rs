extern "C" {
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn vsnprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        __arg: ::core::ffi::VaList,
    ) -> ::core::ffi::c_int;
    fn vasprintf(
        __ptr: *mut *mut ::core::ffi::c_char,
        __f: *const ::core::ffi::c_char,
        __arg: ::core::ffi::VaList,
    ) -> ::core::ffi::c_int;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    fn realloc(__ptr: *mut ::core::ffi::c_void, __size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn memcpy(
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
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: ::core::ffi::c_uint,
    pub fp_offset: ::core::ffi::c_uint,
    pub overflow_arg_area: *mut ::core::ffi::c_void,
    pub reg_save_area: *mut ::core::ffi::c_void,
}
pub type size_t = usize;
pub type va_list = __builtin_va_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct printbuf {
    pub buf: *mut ::core::ffi::c_char,
    pub bpos: ::core::ffi::c_int,
    pub size: ::core::ffi::c_int,
}
pub const INT_MAX: ::core::ffi::c_int = __INT_MAX__;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
#[no_mangle]
pub unsafe extern "C" fn printbuf_new() -> *mut printbuf {
    let mut p: *mut printbuf = ::core::ptr::null_mut::<printbuf>();
    p = calloc(1 as size_t, ::core::mem::size_of::<printbuf>() as size_t) as *mut printbuf;
    if p.is_null() {
        return ::core::ptr::null_mut::<printbuf>();
    }
    (*p).size = 32 as ::core::ffi::c_int;
    (*p).bpos = 0 as ::core::ffi::c_int;
    (*p).buf = malloc((*p).size as size_t) as *mut ::core::ffi::c_char;
    if (*p).buf.is_null() {
        free(p as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<printbuf>();
    }
    *(*p).buf.offset(0 as ::core::ffi::c_int as isize) = '\0' as i32 as ::core::ffi::c_char;
    return p;
}
unsafe extern "C" fn printbuf_extend(
    mut p: *mut printbuf,
    mut min_size: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut t: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut new_size: ::core::ffi::c_int = 0;
    if (*p).size >= min_size {
        return 0 as ::core::ffi::c_int;
    }
    if min_size > INT_MAX - 8 as ::core::ffi::c_int {
        *__errno_location() = EFBIG;
        return -(1 as ::core::ffi::c_int);
    }
    if (*p).size > INT_MAX / 2 as ::core::ffi::c_int {
        new_size = min_size + 8 as ::core::ffi::c_int;
    } else {
        new_size = (*p).size * 2 as ::core::ffi::c_int;
        if new_size < min_size + 8 as ::core::ffi::c_int {
            new_size = min_size + 8 as ::core::ffi::c_int;
        }
    }
    t = realloc((*p).buf as *mut ::core::ffi::c_void, new_size as size_t)
        as *mut ::core::ffi::c_char;
    if t.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    (*p).size = new_size;
    (*p).buf = t;
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn printbuf_memappend(
    mut p: *mut printbuf,
    mut buf: *const ::core::ffi::c_char,
    mut size: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if size < 0 as ::core::ffi::c_int || size > INT_MAX - (*p).bpos - 1 as ::core::ffi::c_int {
        *__errno_location() = EFBIG;
        return -(1 as ::core::ffi::c_int);
    }
    if (*p).size <= (*p).bpos + size + 1 as ::core::ffi::c_int {
        if printbuf_extend(p, (*p).bpos + size + 1 as ::core::ffi::c_int) < 0 as ::core::ffi::c_int
        {
            return -(1 as ::core::ffi::c_int);
        }
    }
    memcpy(
        (*p).buf.offset((*p).bpos as isize) as *mut ::core::ffi::c_void,
        buf as *const ::core::ffi::c_void,
        size as size_t,
    );
    (*p).bpos += size;
    *(*p).buf.offset((*p).bpos as isize) = '\0' as i32 as ::core::ffi::c_char;
    return size;
}
#[no_mangle]
pub unsafe extern "C" fn printbuf_memset(
    mut pb: *mut printbuf,
    mut offset: ::core::ffi::c_int,
    mut charvalue: ::core::ffi::c_int,
    mut len: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut size_needed: ::core::ffi::c_int = 0;
    if offset == -(1 as ::core::ffi::c_int) {
        offset = (*pb).bpos;
    }
    if len < 0 as ::core::ffi::c_int
        || offset < -(1 as ::core::ffi::c_int)
        || len > INT_MAX - offset
    {
        *__errno_location() = EFBIG;
        return -(1 as ::core::ffi::c_int);
    }
    size_needed = offset + len;
    if (*pb).size < size_needed {
        if printbuf_extend(pb, size_needed) < 0 as ::core::ffi::c_int {
            return -(1 as ::core::ffi::c_int);
        }
    }
    if (*pb).bpos < offset {
        memset(
            (*pb).buf.offset((*pb).bpos as isize) as *mut ::core::ffi::c_void,
            '\0' as i32,
            (offset - (*pb).bpos) as size_t,
        );
    }
    memset(
        (*pb).buf.offset(offset as isize) as *mut ::core::ffi::c_void,
        charvalue,
        len as size_t,
    );
    if (*pb).bpos < size_needed {
        (*pb).bpos = size_needed;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn sprintbuf(
    mut p: *mut printbuf,
    mut msg: *const ::core::ffi::c_char,
    mut args: ...
) -> ::core::ffi::c_int {
    let mut ap: ::core::ffi::VaListImpl;
    let mut t: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut size: ::core::ffi::c_int = 0;
    let mut buf: [::core::ffi::c_char; 128] = [0; 128];
    ap = args.clone();
    size = vsnprintf(
        &raw mut buf as *mut ::core::ffi::c_char,
        128 as size_t,
        msg,
        ap.as_va_list(),
    );
    if size < 0 as ::core::ffi::c_int || size > 127 as ::core::ffi::c_int {
        ap = args.clone();
        size = vasprintf(&raw mut t, msg, ap.as_va_list());
        if size < 0 as ::core::ffi::c_int {
            return -(1 as ::core::ffi::c_int);
        }
        size = printbuf_memappend(p, t, size);
        free(t as *mut ::core::ffi::c_void);
    } else {
        size = printbuf_memappend(p, &raw mut buf as *mut ::core::ffi::c_char, size);
    }
    return size;
}
#[no_mangle]
pub unsafe extern "C" fn printbuf_reset(mut p: *mut printbuf) {
    *(*p).buf.offset(0 as ::core::ffi::c_int as isize) = '\0' as i32 as ::core::ffi::c_char;
    (*p).bpos = 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn printbuf_free(mut p: *mut printbuf) {
    if !p.is_null() {
        free((*p).buf as *mut ::core::ffi::c_void);
        free(p as *mut ::core::ffi::c_void);
    }
}
pub const __INT_MAX__: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
pub const EFBIG: ::core::ffi::c_int = 27 as ::core::ffi::c_int;
