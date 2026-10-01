extern "C" {
    pub type json_object;
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn json_object_put(obj: *mut json_object) -> ::core::ffi::c_int;
    fn json_object_to_json_string(obj: *mut json_object) -> *const ::core::ffi::c_char;
    fn strncmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn _json_c_strerror(errno_in: ::core::ffi::c_int) -> *mut ::core::ffi::c_char;
    fn open(
        __file: *const ::core::ffi::c_char,
        __oflag: ::core::ffi::c_int,
        ...
    ) -> ::core::ffi::c_int;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fflush(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn printf(__format: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn putc(__c: ::core::ffi::c_int, __stream: *mut FILE) -> ::core::ffi::c_int;
    fn puts(__s: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn exit(__status: ::core::ffi::c_int) -> !;
    fn lseek(__fd: ::core::ffi::c_int, __offset: __off_t, __whence: ::core::ffi::c_int) -> __off_t;
    fn close(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn read(__fd: ::core::ffi::c_int, __buf: *mut ::core::ffi::c_void, __nbytes: size_t)
        -> ssize_t;
    fn dup2(__fd: ::core::ffi::c_int, __fd2: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn __assert_fail(
        __assertion: *const ::core::ffi::c_char,
        __file: *const ::core::ffi::c_char,
        __line: ::core::ffi::c_uint,
        __function: *const ::core::ffi::c_char,
    ) -> !;
    fn __fxstat(
        __ver: ::core::ffi::c_int,
        __fildes: ::core::ffi::c_int,
        __stat_buf: *mut stat,
    ) -> ::core::ffi::c_int;
    fn json_c_version() -> *const ::core::ffi::c_char;
    fn json_c_version_num() -> ::core::ffi::c_int;
    fn json_tokener_parse(str: *const ::core::ffi::c_char) -> *mut json_object;
    fn json_object_from_file(filename: *const ::core::ffi::c_char) -> *mut json_object;
    fn json_object_from_fd_ex(
        fd: ::core::ffi::c_int,
        depth: ::core::ffi::c_int,
    ) -> *mut json_object;
    fn json_object_from_fd(fd: ::core::ffi::c_int) -> *mut json_object;
    fn json_object_to_file(
        filename: *const ::core::ffi::c_char,
        obj: *mut json_object,
    ) -> ::core::ffi::c_int;
    fn json_object_to_file_ext(
        filename: *const ::core::ffi::c_char,
        obj: *mut json_object,
        flags: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn json_object_to_fd(
        fd: ::core::ffi::c_int,
        obj: *mut json_object,
        flags: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn json_util_get_last_err() -> *const ::core::ffi::c_char;
}
pub type __dev_t = ::core::ffi::c_ulong;
pub type __uid_t = ::core::ffi::c_uint;
pub type __gid_t = ::core::ffi::c_uint;
pub type __ino_t = ::core::ffi::c_ulong;
pub type __mode_t = ::core::ffi::c_uint;
pub type __nlink_t = ::core::ffi::c_ulong;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type __time_t = ::core::ffi::c_long;
pub type __blksize_t = ::core::ffi::c_long;
pub type __blkcnt_t = ::core::ffi::c_long;
pub type __ssize_t = ::core::ffi::c_long;
pub type __syscall_slong_t = ::core::ffi::c_long;
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: ::core::ffi::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
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
pub type ssize_t = __ssize_t;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const JSON_C_TO_STRING_PLAIN: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const JSON_C_TO_STRING_PRETTY: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int;
pub const O_RDONLY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const O_WRONLY: ::core::ffi::c_int = 0o1 as ::core::ffi::c_int;
pub const O_CREAT: ::core::ffi::c_int = 0o100 as ::core::ffi::c_int;
pub const _STAT_VER_LINUX: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const _STAT_VER: ::core::ffi::c_int = _STAT_VER_LINUX;
pub const SEEK_SET: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn putchar(mut __c: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return putc(__c, stdout);
}
pub const EXIT_FAILURE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const EXIT_SUCCESS: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const __ASSERT_FUNCTION: [::core::ffi::c_char; 50] = unsafe {
    ::core::mem::transmute::<[u8; 50], [::core::ffi::c_char; 50]>(
        *b"void test_read_valid_nested_with_fd(const char *)\0",
    )
};
#[inline]
unsafe extern "C" fn fstat(
    mut __fd: ::core::ffi::c_int,
    mut __statbuf: *mut stat,
) -> ::core::ffi::c_int {
    return __fxstat(_STAT_VER, __fd, __statbuf);
}
pub const JSON_C_MAJOR_VERSION: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const JSON_C_MINOR_VERSION: ::core::ffi::c_int = 17 as ::core::ffi::c_int;
pub const JSON_C_MICRO_VERSION: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const JSON_C_VERSION_NUM: ::core::ffi::c_int = JSON_C_MAJOR_VERSION << 16 as ::core::ffi::c_int
    | JSON_C_MINOR_VERSION << 8 as ::core::ffi::c_int
    | JSON_C_MICRO_VERSION;
pub const JSON_C_VERSION: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"0.17\0") };
unsafe extern "C" fn test_write_to_file() {
    let mut jso: *mut json_object = ::core::ptr::null_mut::<json_object>();
    jso = json_tokener_parse(
        b"{\"foo\":1234,\"foo1\":\"abcdefghijklmnopqrstuvwxyz\",\"foo2\":\"abcdefghijklmnopqrstuvwxyz\",\"foo3\":\"abcdefghijklmnopqrstuvwxyz\",\"foo4\":\"abcdefghijklmnopqrstuvwxyz\",\"foo5\":\"abcdefghijklmnopqrstuvwxyz\",\"foo6\":\"abcdefghijklmnopqrstuvwxyz\",\"foo7\":\"abcdefghijklmnopqrstuvwxyz\",\"foo8\":\"abcdefghijklmnopqrstuvwxyz\",\"foo9\":\"abcdefghijklmnopqrstuvwxyz\"}\0"
            as *const u8 as *const ::core::ffi::c_char,
    ) as *mut json_object;
    let mut outfile: *const ::core::ffi::c_char =
        b"json.out\0" as *const u8 as *const ::core::ffi::c_char;
    let mut rv: ::core::ffi::c_int = json_object_to_file(outfile, jso as *mut json_object);
    printf(
        b"%s: json_object_to_file(%s, jso)=%d\n\0" as *const u8 as *const ::core::ffi::c_char,
        if rv == 0 as ::core::ffi::c_int {
            b"OK\0" as *const u8 as *const ::core::ffi::c_char
        } else {
            b"FAIL\0" as *const u8 as *const ::core::ffi::c_char
        },
        outfile,
        rv,
    );
    if rv == 0 as ::core::ffi::c_int {
        stat_and_cat(outfile);
    }
    putchar('\n' as i32);
    let mut outfile2: *const ::core::ffi::c_char =
        b"json2.out\0" as *const u8 as *const ::core::ffi::c_char;
    rv = json_object_to_file_ext(outfile2, jso as *mut json_object, JSON_C_TO_STRING_PRETTY);
    printf(
        b"%s: json_object_to_file_ext(%s, jso, JSON_C_TO_STRING_PRETTY)=%d\n\0" as *const u8
            as *const ::core::ffi::c_char,
        if rv == 0 as ::core::ffi::c_int {
            b"OK\0" as *const u8 as *const ::core::ffi::c_char
        } else {
            b"FAIL\0" as *const u8 as *const ::core::ffi::c_char
        },
        outfile2,
        rv,
    );
    if rv == 0 as ::core::ffi::c_int {
        stat_and_cat(outfile2);
    }
    let mut outfile3: *const ::core::ffi::c_char =
        b"json3.out\0" as *const u8 as *const ::core::ffi::c_char;
    let mut d: ::core::ffi::c_int = open(outfile3, O_WRONLY | O_CREAT, 0o600 as ::core::ffi::c_int);
    if d < 0 as ::core::ffi::c_int {
        printf(
            b"FAIL: unable to open %s %s\n\0" as *const u8 as *const ::core::ffi::c_char,
            outfile3,
            _json_c_strerror(*__errno_location()),
        );
        return;
    }
    rv = json_object_to_fd(d, jso as *mut json_object, JSON_C_TO_STRING_PRETTY);
    printf(
        b"%s: json_object_to_fd(%s, jso, JSON_C_TO_STRING_PRETTY)=%d\n\0" as *const u8
            as *const ::core::ffi::c_char,
        if rv == 0 as ::core::ffi::c_int {
            b"OK\0" as *const u8 as *const ::core::ffi::c_char
        } else {
            b"FAIL\0" as *const u8 as *const ::core::ffi::c_char
        },
        outfile3,
        rv,
    );
    rv = json_object_to_fd(d, jso as *mut json_object, JSON_C_TO_STRING_PLAIN);
    printf(
        b"%s: json_object_to_fd(%s, jso, JSON_C_TO_STRING_PLAIN)=%d\n\0" as *const u8
            as *const ::core::ffi::c_char,
        if rv == 0 as ::core::ffi::c_int {
            b"OK\0" as *const u8 as *const ::core::ffi::c_char
        } else {
            b"FAIL\0" as *const u8 as *const ::core::ffi::c_char
        },
        outfile3,
        rv,
    );
    close(d);
    if rv == 0 as ::core::ffi::c_int {
        stat_and_cat(outfile3);
    }
    json_object_put(jso as *mut json_object);
}
unsafe extern "C" fn stat_and_cat(mut file: *const ::core::ffi::c_char) {
    let mut sb: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_mtim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_ctim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        __glibc_reserved: [0; 3],
    };
    let mut d: ::core::ffi::c_int = open(file, O_RDONLY);
    if d < 0 as ::core::ffi::c_int {
        printf(
            b"FAIL: unable to open %s: %s\n\0" as *const u8 as *const ::core::ffi::c_char,
            file,
            _json_c_strerror(*__errno_location()),
        );
        return;
    }
    if fstat(d, &raw mut sb) < 0 as ::core::ffi::c_int {
        printf(
            b"FAIL: unable to stat %s: %s\n\0" as *const u8 as *const ::core::ffi::c_char,
            file,
            _json_c_strerror(*__errno_location()),
        );
        close(d);
        return;
    }
    let mut buf: *mut ::core::ffi::c_char =
        malloc((sb.st_size as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as size_t)
            as *mut ::core::ffi::c_char;
    if buf.is_null() {
        printf(b"FAIL: unable to allocate memory\n\0" as *const u8 as *const ::core::ffi::c_char);
        close(d);
        return;
    }
    if read(d, buf as *mut ::core::ffi::c_void, sb.st_size as size_t) < sb.st_size {
        printf(
            b"FAIL: unable to read all of %s: %s\n\0" as *const u8 as *const ::core::ffi::c_char,
            file,
            _json_c_strerror(*__errno_location()),
        );
        free(buf as *mut ::core::ffi::c_void);
        close(d);
        return;
    }
    *buf.offset(sb.st_size as isize) = '\0' as i32 as ::core::ffi::c_char;
    printf(
        b"file[%s], size=%d, contents=%s\n\0" as *const u8 as *const ::core::ffi::c_char,
        file,
        sb.st_size as ::core::ffi::c_int,
        buf,
    );
    free(buf as *mut ::core::ffi::c_void);
    close(d);
}
unsafe fn main_0(
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut testdir: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    if argc < 2 as ::core::ffi::c_int {
        fprintf(
            stderr,
            b"Usage: %s <testdir>\n  <testdir> is the location of input files\n\0" as *const u8
                as *const ::core::ffi::c_char,
            *argv.offset(0 as ::core::ffi::c_int as isize),
        );
        return EXIT_FAILURE;
    }
    testdir = *argv.offset(1 as ::core::ffi::c_int as isize);
    if strncmp(
        json_c_version(),
        JSON_C_VERSION.as_ptr(),
        ::core::mem::size_of::<[::core::ffi::c_char; 5]>() as size_t,
    ) != 0
    {
        printf(
            b"FAIL: Output from json_c_version(): %s does not match %s\0" as *const u8
                as *const ::core::ffi::c_char,
            json_c_version(),
            JSON_C_VERSION.as_ptr(),
        );
        return EXIT_FAILURE;
    }
    if json_c_version_num() != JSON_C_VERSION_NUM {
        printf(
            b"FAIL: Output from json_c_version_num(): %d does not match %d\0" as *const u8
                as *const ::core::ffi::c_char,
            json_c_version_num(),
            JSON_C_VERSION_NUM,
        );
        return EXIT_FAILURE;
    }
    test_read_valid_with_fd(testdir);
    test_read_valid_nested_with_fd(testdir);
    test_read_nonexistant();
    test_read_closed();
    test_write_to_file();
    test_read_fd_equal(testdir);
    return EXIT_SUCCESS;
}
unsafe extern "C" fn test_read_valid_with_fd(mut testdir: *const ::core::ffi::c_char) {
    let mut filename: [::core::ffi::c_char; 4096] = [0; 4096];
    snprintf(
        &raw mut filename as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 4096]>() as size_t,
        b"%s/valid.json\0" as *const u8 as *const ::core::ffi::c_char,
        testdir,
    );
    let mut d: ::core::ffi::c_int = open(&raw mut filename as *mut ::core::ffi::c_char, O_RDONLY);
    if d < 0 as ::core::ffi::c_int {
        fprintf(
            stderr,
            b"FAIL: unable to open %s: %s\n\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut filename as *mut ::core::ffi::c_char,
            _json_c_strerror(*__errno_location()),
        );
        exit(EXIT_FAILURE);
    }
    let mut jso: *mut json_object = json_object_from_fd(d) as *mut json_object;
    if !jso.is_null() {
        printf(
            b"OK: json_object_from_fd(valid.json)=%s\n\0" as *const u8
                as *const ::core::ffi::c_char,
            json_object_to_json_string(jso as *mut json_object),
        );
        json_object_put(jso as *mut json_object);
    } else {
        fprintf(
            stderr,
            b"FAIL: unable to parse contents of %s: %s\n\0" as *const u8
                as *const ::core::ffi::c_char,
            &raw mut filename as *mut ::core::ffi::c_char,
            json_util_get_last_err(),
        );
    }
    close(d);
}
unsafe extern "C" fn test_read_valid_nested_with_fd(mut testdir: *const ::core::ffi::c_char) {
    let mut filename: [::core::ffi::c_char; 4096] = [0; 4096];
    snprintf(
        &raw mut filename as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 4096]>() as size_t,
        b"%s/valid_nested.json\0" as *const u8 as *const ::core::ffi::c_char,
        testdir,
    );
    let mut d: ::core::ffi::c_int = open(&raw mut filename as *mut ::core::ffi::c_char, O_RDONLY);
    if d < 0 as ::core::ffi::c_int {
        fprintf(
            stderr,
            b"FAIL: unable to open %s: %s\n\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut filename as *mut ::core::ffi::c_char,
            _json_c_strerror(*__errno_location()),
        );
        exit(EXIT_FAILURE);
    }
    '_c2rust_label: {
        if json_object_from_fd_ex(d, -(2 as ::core::ffi::c_int)).is_null() {
        } else {
            __assert_fail(
                b"NULL == json_object_from_fd_ex(d, -2)\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/json-c/tests/test_util_file.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                204 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    let mut jso: *mut json_object =
        json_object_from_fd_ex(d, 20 as ::core::ffi::c_int) as *mut json_object;
    if !jso.is_null() {
        printf(
            b"OK: json_object_from_fd_ex(valid_nested.json, 20)=%s\n\0" as *const u8
                as *const ::core::ffi::c_char,
            json_object_to_json_string(jso as *mut json_object),
        );
        json_object_put(jso as *mut json_object);
    } else {
        fprintf(
            stderr,
            b"FAIL: unable to parse contents of %s: %s\n\0" as *const u8
                as *const ::core::ffi::c_char,
            &raw mut filename as *mut ::core::ffi::c_char,
            json_util_get_last_err(),
        );
    }
    lseek(d, SEEK_SET as __off_t, 0 as ::core::ffi::c_int);
    jso = json_object_from_fd_ex(d, 3 as ::core::ffi::c_int) as *mut json_object;
    if !jso.is_null() {
        printf(
            b"FAIL: json_object_from_fd_ex(%s, 3)=%s\n\0" as *const u8
                as *const ::core::ffi::c_char,
            &raw mut filename as *mut ::core::ffi::c_char,
            json_object_to_json_string(jso as *mut json_object),
        );
        json_object_put(jso as *mut json_object);
    } else {
        printf(
            b"OK: correctly unable to parse contents of valid_nested.json with low max depth: %s\n\0"
                as *const u8 as *const ::core::ffi::c_char,
            json_util_get_last_err(),
        );
    }
    close(d);
}
unsafe extern "C" fn test_read_nonexistant() {
    let mut filename: *const ::core::ffi::c_char =
        b"./not_present.json\0" as *const u8 as *const ::core::ffi::c_char;
    let mut jso: *mut json_object = json_object_from_file(filename) as *mut json_object;
    if !jso.is_null() {
        printf(
            b"FAIL: json_object_from_file(%s) returned %p when NULL expected\n\0" as *const u8
                as *const ::core::ffi::c_char,
            filename,
            jso as *mut ::core::ffi::c_void,
        );
        json_object_put(jso as *mut json_object);
    } else {
        printf(
            b"OK: json_object_from_file(%s) correctly returned NULL: %s\n\0" as *const u8
                as *const ::core::ffi::c_char,
            filename,
            json_util_get_last_err(),
        );
    };
}
unsafe extern "C" fn test_read_closed() {
    let mut d: ::core::ffi::c_int = open(
        b"/dev/null\0" as *const u8 as *const ::core::ffi::c_char,
        O_RDONLY,
    );
    if d < 0 as ::core::ffi::c_int {
        puts(b"FAIL: unable to open\0" as *const u8 as *const ::core::ffi::c_char);
    }
    let mut fixed_d: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
    if dup2(d, fixed_d) < 0 as ::core::ffi::c_int {
        printf(
            b"FAIL: unable to dup to fd %d\0" as *const u8 as *const ::core::ffi::c_char,
            fixed_d,
        );
    }
    close(d);
    close(fixed_d);
    let mut jso: *mut json_object = json_object_from_fd(fixed_d) as *mut json_object;
    if !jso.is_null() {
        printf(
            b"FAIL: read from closed fd returning non-NULL: %p\n\0" as *const u8
                as *const ::core::ffi::c_char,
            jso as *mut ::core::ffi::c_void,
        );
        fflush(stdout);
        printf(
            b"  jso=%s\n\0" as *const u8 as *const ::core::ffi::c_char,
            json_object_to_json_string(jso as *mut json_object),
        );
        json_object_put(jso as *mut json_object);
        return;
    }
    printf(
        b"OK: json_object_from_fd(closed_fd), expecting NULL, EBADF, got:NULL, %s\n\0" as *const u8
            as *const ::core::ffi::c_char,
        json_util_get_last_err(),
    );
}
unsafe extern "C" fn test_read_fd_equal(mut testdir: *const ::core::ffi::c_char) {
    let mut filename: [::core::ffi::c_char; 4096] = [0; 4096];
    snprintf(
        &raw mut filename as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 4096]>() as size_t,
        b"%s/valid_nested.json\0" as *const u8 as *const ::core::ffi::c_char,
        testdir,
    );
    let mut jso: *mut json_object =
        json_object_from_file(&raw mut filename as *mut ::core::ffi::c_char) as *mut json_object;
    let mut d: ::core::ffi::c_int = open(&raw mut filename as *mut ::core::ffi::c_char, O_RDONLY);
    if d < 0 as ::core::ffi::c_int {
        fprintf(
            stderr,
            b"FAIL: unable to open %s: %s\n\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut filename as *mut ::core::ffi::c_char,
            _json_c_strerror(*__errno_location()),
        );
        exit(EXIT_FAILURE);
    }
    let mut new_jso: *mut json_object = json_object_from_fd(d) as *mut json_object;
    close(d);
    printf(
        b"OK: json_object_from_file(valid.json)=%s\n\0" as *const u8 as *const ::core::ffi::c_char,
        json_object_to_json_string(jso as *mut json_object),
    );
    printf(
        b"OK: json_object_from_fd(valid.json)=%s\n\0" as *const u8 as *const ::core::ffi::c_char,
        json_object_to_json_string(new_jso as *mut json_object),
    );
    json_object_put(jso as *mut json_object);
    json_object_put(new_jso as *mut json_object);
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
