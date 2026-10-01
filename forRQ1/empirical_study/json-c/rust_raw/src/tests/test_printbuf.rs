extern "C" {
    fn printf(__format: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
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
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn mc_set_debug(debug: ::core::ffi::c_int);
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
    fn sprintbuf(p: *mut printbuf, msg: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn printbuf_reset(p: *mut printbuf);
    fn printbuf_free(p: *mut printbuf);
}
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct printbuf {
    pub buf: *mut ::core::ffi::c_char,
    pub bpos: ::core::ffi::c_int,
    pub size: ::core::ffi::c_int,
}
pub const INT_MAX: ::core::ffi::c_int = __INT_MAX__;
pub const INT_MIN: ::core::ffi::c_int = -__INT_MAX__ - 1 as ::core::ffi::c_int;
unsafe extern "C" fn test_basic_printbuf_memset() {
    let mut pb: *mut printbuf = ::core::ptr::null_mut::<printbuf>();
    printf(
        b"%s: starting test\n\0" as *const u8 as *const ::core::ffi::c_char,
        b"test_basic_printbuf_memset\0" as *const u8 as *const ::core::ffi::c_char,
    );
    pb = printbuf_new();
    sprintbuf(
        pb,
        b"blue:%d\0" as *const u8 as *const ::core::ffi::c_char,
        1 as ::core::ffi::c_int,
    );
    printbuf_memset(
        pb,
        -(1 as ::core::ffi::c_int),
        'x' as i32,
        52 as ::core::ffi::c_int,
    );
    printf(
        b"Buffer contents:%.*s\n\0" as *const u8 as *const ::core::ffi::c_char,
        (*pb).bpos,
        (*pb).buf,
    );
    printbuf_free(pb);
    printf(
        b"%s: end test\n\0" as *const u8 as *const ::core::ffi::c_char,
        b"test_basic_printbuf_memset\0" as *const u8 as *const ::core::ffi::c_char,
    );
}
unsafe extern "C" fn test_printbuf_memset_length() {
    let mut pb: *mut printbuf = ::core::ptr::null_mut::<printbuf>();
    printf(
        b"%s: starting test\n\0" as *const u8 as *const ::core::ffi::c_char,
        b"test_printbuf_memset_length\0" as *const u8 as *const ::core::ffi::c_char,
    );
    pb = printbuf_new();
    printbuf_memset(
        pb,
        -(1 as ::core::ffi::c_int),
        ' ' as i32,
        0 as ::core::ffi::c_int,
    );
    printbuf_memset(
        pb,
        -(1 as ::core::ffi::c_int),
        ' ' as i32,
        0 as ::core::ffi::c_int,
    );
    printbuf_memset(
        pb,
        -(1 as ::core::ffi::c_int),
        ' ' as i32,
        0 as ::core::ffi::c_int,
    );
    printbuf_memset(
        pb,
        -(1 as ::core::ffi::c_int),
        ' ' as i32,
        0 as ::core::ffi::c_int,
    );
    printbuf_memset(
        pb,
        -(1 as ::core::ffi::c_int),
        ' ' as i32,
        0 as ::core::ffi::c_int,
    );
    printf(
        b"Buffer length: %d\n\0" as *const u8 as *const ::core::ffi::c_char,
        (*pb).bpos,
    );
    printbuf_memset(
        pb,
        -(1 as ::core::ffi::c_int),
        ' ' as i32,
        2 as ::core::ffi::c_int,
    );
    printbuf_memset(
        pb,
        -(1 as ::core::ffi::c_int),
        ' ' as i32,
        4 as ::core::ffi::c_int,
    );
    printbuf_memset(
        pb,
        -(1 as ::core::ffi::c_int),
        ' ' as i32,
        6 as ::core::ffi::c_int,
    );
    printf(
        b"Buffer length: %d\n\0" as *const u8 as *const ::core::ffi::c_char,
        (*pb).bpos,
    );
    printbuf_memset(
        pb,
        -(1 as ::core::ffi::c_int),
        ' ' as i32,
        6 as ::core::ffi::c_int,
    );
    printf(
        b"Buffer length: %d\n\0" as *const u8 as *const ::core::ffi::c_char,
        (*pb).bpos,
    );
    printbuf_memset(
        pb,
        -(1 as ::core::ffi::c_int),
        ' ' as i32,
        8 as ::core::ffi::c_int,
    );
    printbuf_memset(
        pb,
        -(1 as ::core::ffi::c_int),
        ' ' as i32,
        10 as ::core::ffi::c_int,
    );
    printbuf_memset(
        pb,
        -(1 as ::core::ffi::c_int),
        ' ' as i32,
        10 as ::core::ffi::c_int,
    );
    printbuf_memset(
        pb,
        -(1 as ::core::ffi::c_int),
        ' ' as i32,
        10 as ::core::ffi::c_int,
    );
    printbuf_memset(
        pb,
        -(1 as ::core::ffi::c_int),
        ' ' as i32,
        20 as ::core::ffi::c_int,
    );
    printf(
        b"Buffer length: %d\n\0" as *const u8 as *const ::core::ffi::c_char,
        (*pb).bpos,
    );
    printbuf_memset(
        pb,
        0 as ::core::ffi::c_int,
        'x' as i32,
        30 as ::core::ffi::c_int,
    );
    printf(
        b"Buffer length: %d\n\0" as *const u8 as *const ::core::ffi::c_char,
        (*pb).bpos,
    );
    printbuf_memset(
        pb,
        0 as ::core::ffi::c_int,
        'x' as i32,
        (*pb).bpos + 1 as ::core::ffi::c_int,
    );
    printf(
        b"Buffer length: %d\n\0" as *const u8 as *const ::core::ffi::c_char,
        (*pb).bpos,
    );
    printbuf_free(pb);
    printf(
        b"%s: end test\n\0" as *const u8 as *const ::core::ffi::c_char,
        b"test_printbuf_memset_length\0" as *const u8 as *const ::core::ffi::c_char,
    );
}
unsafe extern "C" fn test_printbuf_memappend(mut before_resize: *mut ::core::ffi::c_int) {
    let mut pb: *mut printbuf = ::core::ptr::null_mut::<printbuf>();
    let mut initial_size: ::core::ffi::c_int = 0;
    printf(
        b"%s: starting test\n\0" as *const u8 as *const ::core::ffi::c_char,
        b"test_printbuf_memappend\0" as *const u8 as *const ::core::ffi::c_char,
    );
    pb = printbuf_new();
    printf(
        b"Buffer length: %d\n\0" as *const u8 as *const ::core::ffi::c_char,
        (*pb).bpos,
    );
    initial_size = (*pb).size;
    while (*pb).size == initial_size {
        if (*pb).size - (*pb).bpos > 1 as ::core::ffi::c_int {
            memcpy(
                (*pb).buf.offset((*pb).bpos as isize) as *mut ::core::ffi::c_void,
                b"x\0" as *const u8 as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
                1 as size_t,
            );
            (*pb).bpos += 1 as ::core::ffi::c_int;
            *(*pb).buf.offset((*pb).bpos as isize) = '\0' as i32 as ::core::ffi::c_char;
        } else {
            printbuf_memappend(
                pb,
                b"x\0" as *const u8 as *const ::core::ffi::c_char,
                1 as ::core::ffi::c_int,
            );
        }
    }
    *before_resize = (*pb).bpos - 1 as ::core::ffi::c_int;
    printf(
        b"Appended %d bytes for resize: [%s]\n\0" as *const u8 as *const ::core::ffi::c_char,
        *before_resize + 1 as ::core::ffi::c_int,
        (*pb).buf,
    );
    printbuf_reset(pb);
    if (*pb).size - (*pb).bpos > 3 as ::core::ffi::c_int {
        memcpy(
            (*pb).buf.offset((*pb).bpos as isize) as *mut ::core::ffi::c_void,
            b"bluexyz123\0" as *const u8 as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            3 as size_t,
        );
        (*pb).bpos += 3 as ::core::ffi::c_int;
        *(*pb).buf.offset((*pb).bpos as isize) = '\0' as i32 as ::core::ffi::c_char;
    } else {
        printbuf_memappend(
            pb,
            b"bluexyz123\0" as *const u8 as *const ::core::ffi::c_char,
            3 as ::core::ffi::c_int,
        );
    }
    printf(
        b"Partial append: %d, [%s]\n\0" as *const u8 as *const ::core::ffi::c_char,
        (*pb).bpos,
        (*pb).buf,
    );
    let mut with_nulls: [::core::ffi::c_char; 4] = [
        'a' as i32 as ::core::ffi::c_char,
        'b' as i32 as ::core::ffi::c_char,
        '\0' as i32 as ::core::ffi::c_char,
        'c' as i32 as ::core::ffi::c_char,
    ];
    printbuf_reset(pb);
    if (*pb).size - (*pb).bpos
        > ::core::mem::size_of::<[::core::ffi::c_char; 4]>() as ::core::ffi::c_int
    {
        memcpy(
            (*pb).buf.offset((*pb).bpos as isize) as *mut ::core::ffi::c_void,
            &raw mut with_nulls as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
            ::core::mem::size_of::<[::core::ffi::c_char; 4]>() as ::core::ffi::c_int as size_t,
        );
        (*pb).bpos += ::core::mem::size_of::<[::core::ffi::c_char; 4]>() as ::core::ffi::c_int;
        *(*pb).buf.offset((*pb).bpos as isize) = '\0' as i32 as ::core::ffi::c_char;
    } else {
        printbuf_memappend(
            pb,
            &raw mut with_nulls as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 4]>() as ::core::ffi::c_int,
        );
    }
    printf(
        b"With embedded \\0 character: %d, [%s]\n\0" as *const u8 as *const ::core::ffi::c_char,
        (*pb).bpos,
        (*pb).buf,
    );
    printbuf_free(pb);
    pb = printbuf_new();
    let mut data: *mut ::core::ffi::c_char =
        malloc(*before_resize as size_t) as *mut ::core::ffi::c_char;
    memset(
        data as *mut ::core::ffi::c_void,
        'X' as i32,
        *before_resize as size_t,
    );
    if (*pb).size - (*pb).bpos > *before_resize {
        memcpy(
            (*pb).buf.offset((*pb).bpos as isize) as *mut ::core::ffi::c_void,
            data as *const ::core::ffi::c_void,
            *before_resize as size_t,
        );
        (*pb).bpos += *before_resize;
        *(*pb).buf.offset((*pb).bpos as isize) = '\0' as i32 as ::core::ffi::c_char;
    } else {
        printbuf_memappend(pb, data, *before_resize);
    }
    printf(
        b"Append to just before resize: %d, [%s]\n\0" as *const u8 as *const ::core::ffi::c_char,
        (*pb).bpos,
        (*pb).buf,
    );
    free(data as *mut ::core::ffi::c_void);
    printbuf_free(pb);
    pb = printbuf_new();
    data = malloc((*before_resize + 1 as ::core::ffi::c_int) as size_t) as *mut ::core::ffi::c_char;
    memset(
        data as *mut ::core::ffi::c_void,
        'X' as i32,
        (*before_resize + 1 as ::core::ffi::c_int) as size_t,
    );
    if (*pb).size - (*pb).bpos > *before_resize + 1 as ::core::ffi::c_int {
        memcpy(
            (*pb).buf.offset((*pb).bpos as isize) as *mut ::core::ffi::c_void,
            data as *const ::core::ffi::c_void,
            (*before_resize + 1 as ::core::ffi::c_int) as size_t,
        );
        (*pb).bpos += *before_resize + 1 as ::core::ffi::c_int;
        *(*pb).buf.offset((*pb).bpos as isize) = '\0' as i32 as ::core::ffi::c_char;
    } else {
        printbuf_memappend(pb, data, *before_resize + 1 as ::core::ffi::c_int);
    }
    printf(
        b"Append to just after resize: %d, [%s]\n\0" as *const u8 as *const ::core::ffi::c_char,
        (*pb).bpos,
        (*pb).buf,
    );
    free(data as *mut ::core::ffi::c_void);
    printbuf_free(pb);
    pb = printbuf_new();
    printbuf_memappend(
        pb,
        b"XXXXXXXXXXXXXXXX\0" as *const u8 as *const ::core::ffi::c_char,
        (::core::mem::size_of::<[::core::ffi::c_char; 17]>() as usize).wrapping_sub(1 as usize)
            as ::core::ffi::c_int,
    );
    printf(
        b"Buffer size after printbuf_strappend(): %d, [%s]\n\0" as *const u8
            as *const ::core::ffi::c_char,
        (*pb).bpos,
        (*pb).buf,
    );
    printbuf_free(pb);
    printf(
        b"%s: end test\n\0" as *const u8 as *const ::core::ffi::c_char,
        b"test_printbuf_memappend\0" as *const u8 as *const ::core::ffi::c_char,
    );
}
unsafe extern "C" fn test_sprintbuf(mut before_resize: ::core::ffi::c_int) {
    let mut pb: *mut printbuf = ::core::ptr::null_mut::<printbuf>();
    let mut max_char: *const ::core::ffi::c_char = b"if string is greater than stack buffer, then use dynamic string with vasprintf.  Note: some implementation of vsnprintf return -1  if output is truncated whereas some return the number of bytes that  would have been written - this code handles both cases.\0"
        as *const u8 as *const ::core::ffi::c_char;
    printf(
        b"%s: starting test\n\0" as *const u8 as *const ::core::ffi::c_char,
        b"test_sprintbuf\0" as *const u8 as *const ::core::ffi::c_char,
    );
    pb = printbuf_new();
    printf(
        b"Buffer length: %d\n\0" as *const u8 as *const ::core::ffi::c_char,
        (*pb).bpos,
    );
    let mut data: *mut ::core::ffi::c_char =
        malloc((before_resize + 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as size_t)
            as *mut ::core::ffi::c_char;
    memset(
        data as *mut ::core::ffi::c_void,
        'X' as i32,
        (before_resize + 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as size_t,
    );
    *data.offset((before_resize + 1 as ::core::ffi::c_int) as isize) =
        '\0' as i32 as ::core::ffi::c_char;
    sprintbuf(pb, b"%s\0" as *const u8 as *const ::core::ffi::c_char, data);
    free(data as *mut ::core::ffi::c_void);
    printf(
        b"sprintbuf to just after resize(%d+1): %d, [%s], strlen(buf)=%d\n\0" as *const u8
            as *const ::core::ffi::c_char,
        before_resize,
        (*pb).bpos,
        (*pb).buf,
        strlen((*pb).buf) as ::core::ffi::c_int,
    );
    printbuf_reset(pb);
    sprintbuf(pb, b"plain\0" as *const u8 as *const ::core::ffi::c_char);
    printf(
        b"%d, [%s]\n\0" as *const u8 as *const ::core::ffi::c_char,
        (*pb).bpos,
        (*pb).buf,
    );
    sprintbuf(
        pb,
        b"%d\0" as *const u8 as *const ::core::ffi::c_char,
        1 as ::core::ffi::c_int,
    );
    printf(
        b"%d, [%s]\n\0" as *const u8 as *const ::core::ffi::c_char,
        (*pb).bpos,
        (*pb).buf,
    );
    sprintbuf(
        pb,
        b"%d\0" as *const u8 as *const ::core::ffi::c_char,
        INT_MAX,
    );
    printf(
        b"%d, [%s]\n\0" as *const u8 as *const ::core::ffi::c_char,
        (*pb).bpos,
        (*pb).buf,
    );
    sprintbuf(
        pb,
        b"%d\0" as *const u8 as *const ::core::ffi::c_char,
        INT_MIN,
    );
    printf(
        b"%d, [%s]\n\0" as *const u8 as *const ::core::ffi::c_char,
        (*pb).bpos,
        (*pb).buf,
    );
    sprintbuf(
        pb,
        b"%s\0" as *const u8 as *const ::core::ffi::c_char,
        b"%s\0" as *const u8 as *const ::core::ffi::c_char,
    );
    printf(
        b"%d, [%s]\n\0" as *const u8 as *const ::core::ffi::c_char,
        (*pb).bpos,
        (*pb).buf,
    );
    sprintbuf(pb, max_char);
    printf(
        b"%d, [%s]\n\0" as *const u8 as *const ::core::ffi::c_char,
        (*pb).bpos,
        (*pb).buf,
    );
    printbuf_free(pb);
    printf(
        b"%s: end test\n\0" as *const u8 as *const ::core::ffi::c_char,
        b"test_sprintbuf\0" as *const u8 as *const ::core::ffi::c_char,
    );
}
unsafe fn main_0(
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut before_resize: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    test_basic_printbuf_memset();
    printf(
        b"========================================\n\0" as *const u8 as *const ::core::ffi::c_char,
    );
    test_printbuf_memset_length();
    printf(
        b"========================================\n\0" as *const u8 as *const ::core::ffi::c_char,
    );
    test_printbuf_memappend(&raw mut before_resize);
    printf(
        b"========================================\n\0" as *const u8 as *const ::core::ffi::c_char,
    );
    test_sprintbuf(before_resize);
    printf(
        b"========================================\n\0" as *const u8 as *const ::core::ffi::c_char,
    );
    return 0 as ::core::ffi::c_int;
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
