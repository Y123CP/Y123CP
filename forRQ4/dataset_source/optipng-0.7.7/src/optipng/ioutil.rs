extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn rename(
        __old: *const ::core::ffi::c_char,
        __new: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn fflush(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn fread(
        __ptr: *mut ::core::ffi::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> ::core::ffi::c_ulong;
    fn fwrite(
        __ptr: *const ::core::ffi::c_void,
        __size: size_t,
        __n: size_t,
        __s: *mut FILE,
    ) -> ::core::ffi::c_ulong;
    fn fseek(
        __stream: *mut FILE,
        __off: ::core::ffi::c_long,
        __whence: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn ftell(__stream: *mut FILE) -> ::core::ffi::c_long;
    fn fgetpos(__stream: *mut FILE, __pos: *mut fpos_t) -> ::core::ffi::c_int;
    fn fsetpos(__stream: *mut FILE, __pos: *const fpos_t) -> ::core::ffi::c_int;
    fn fileno(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn strcpy(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strcat(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strchr(__s: *const ::core::ffi::c_char, __c: ::core::ffi::c_int)
        -> *mut ::core::ffi::c_char;
    fn strpbrk(
        __s: *const ::core::ffi::c_char,
        __accept: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn access(__name: *const ::core::ffi::c_char, __type: ::core::ffi::c_int)
        -> ::core::ffi::c_int;
    fn chown(
        __file: *const ::core::ffi::c_char,
        __owner: __uid_t,
        __group: __gid_t,
    ) -> ::core::ffi::c_int;
    fn unlink(__name: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn chmod(__file: *const ::core::ffi::c_char, __mode: __mode_t) -> ::core::ffi::c_int;
    fn mkdir(__path: *const ::core::ffi::c_char, __mode: __mode_t) -> ::core::ffi::c_int;
    fn utimensat(
        __fd: ::core::ffi::c_int,
        __path: *const ::core::ffi::c_char,
        __times: *const timespec,
        __flags: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn __fxstat(
        __ver: ::core::ffi::c_int,
        __fildes: ::core::ffi::c_int,
        __stat_buf: *mut stat,
    ) -> ::core::ffi::c_int;
    fn __xstat(
        __ver: ::core::ffi::c_int,
        __filename: *const ::core::ffi::c_char,
        __stat_buf: *mut stat,
    ) -> ::core::ffi::c_int;
}
pub type size_t = usize;
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
pub type __syscall_slong_t = ::core::ffi::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __mbstate_t {
    pub __count: ::core::ffi::c_int,
    pub __value: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub __wch: ::core::ffi::c_uint,
    pub __wchb: [::core::ffi::c_char; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _G_fpos_t {
    pub __pos: __off_t,
    pub __state: __mbstate_t,
}
pub type __fpos_t = _G_fpos_t;
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
pub type fpos_t = __fpos_t;
pub type opng_foffset_t = ::core::ffi::c_long;
pub type opng_fsize_t = ::core::ffi::c_ulong;
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
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const AT_FDCWD: ::core::ffi::c_int = -(100 as ::core::ffi::c_int);
pub const R_OK: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const W_OK: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const X_OK: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const F_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const _STAT_VER_LINUX: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const _STAT_VER: ::core::ffi::c_int = _STAT_VER_LINUX;
pub const __S_IFDIR: ::core::ffi::c_int = 0o40000 as ::core::ffi::c_int;
pub const __S_IFREG: ::core::ffi::c_int = 0o100000 as ::core::ffi::c_int;
pub const S_IFDIR: ::core::ffi::c_int = __S_IFDIR;
pub const S_IFREG: ::core::ffi::c_int = __S_IFREG;
#[inline]
unsafe extern "C" fn stat(
    mut __path: *const ::core::ffi::c_char,
    mut __statbuf: *mut stat,
) -> ::core::ffi::c_int {
    return __xstat(_STAT_VER, __path, __statbuf);
}
#[inline]
unsafe extern "C" fn fstat(
    mut __fd: ::core::ffi::c_int,
    mut __statbuf: *mut stat,
) -> ::core::ffi::c_int {
    return __fxstat(_STAT_VER, __fd, __statbuf);
}
pub const OPNG_PATH_DIRSEP: ::core::ffi::c_int = '/' as i32;
pub const OPNG_PATH_DIRSEP_ALL_STR: [::core::ffi::c_char; 2] =
    unsafe { ::core::mem::transmute::<[u8; 2], [::core::ffi::c_char; 2]>(*b"/\0") };
pub const OPNG_PATH_EXTSEP: ::core::ffi::c_int = '.' as i32;
pub const OPNG_TEST_READ: ::core::ffi::c_int = R_OK;
pub const OPNG_TEST_WRITE: ::core::ffi::c_int = W_OK;
pub const OPNG_TEST_EXEC: ::core::ffi::c_int = X_OK;
pub const OPNG_TEST_FILE: ::core::ffi::c_int = F_OK;
#[no_mangle]
pub unsafe extern "C" fn opng_ftello(mut stream: *mut FILE) -> opng_foffset_t {
    return ftell(stream);
}
#[no_mangle]
pub unsafe extern "C" fn opng_fseeko(
    mut stream: *mut FILE,
    mut offset: opng_foffset_t,
    mut whence: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return fseek(stream, offset, whence);
}
#[no_mangle]
pub unsafe extern "C" fn opng_freado(
    mut stream: *mut FILE,
    mut offset: opng_foffset_t,
    mut whence: ::core::ffi::c_int,
    mut block: *mut ::core::ffi::c_void,
    mut blocksize: size_t,
) -> size_t {
    let mut pos: fpos_t = fpos_t {
        __pos: 0,
        __state: __mbstate_t {
            __count: 0,
            __value: C2RustUnnamed { __wch: 0 },
        },
    };
    let mut result: size_t = 0;
    if fgetpos(stream, &raw mut pos) != 0 as ::core::ffi::c_int {
        return 0 as size_t;
    }
    if opng_fseeko(stream, offset, whence) == 0 as ::core::ffi::c_int {
        result = fread(block, 1 as size_t, blocksize, stream) as size_t;
    } else {
        result = 0 as size_t;
    }
    if fsetpos(stream, &raw mut pos) != 0 as ::core::ffi::c_int {
        result = 0 as size_t;
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn opng_fwriteo(
    mut stream: *mut FILE,
    mut offset: opng_foffset_t,
    mut whence: ::core::ffi::c_int,
    mut block: *const ::core::ffi::c_void,
    mut blocksize: size_t,
) -> size_t {
    let mut pos: fpos_t = fpos_t {
        __pos: 0,
        __state: __mbstate_t {
            __count: 0,
            __value: C2RustUnnamed { __wch: 0 },
        },
    };
    let mut result: size_t = 0;
    if fgetpos(stream, &raw mut pos) != 0 as ::core::ffi::c_int
        || fflush(stream) != 0 as ::core::ffi::c_int
    {
        return 0 as size_t;
    }
    if opng_fseeko(stream, offset, whence) == 0 as ::core::ffi::c_int {
        result = fwrite(block, 1 as size_t, blocksize, stream) as size_t;
    } else {
        result = 0 as size_t;
    }
    if fflush(stream) != 0 as ::core::ffi::c_int {
        result = 0 as size_t;
    }
    if fsetpos(stream, &raw mut pos) != 0 as ::core::ffi::c_int {
        result = 0 as size_t;
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn opng_fgetsize(
    mut stream: *mut FILE,
    mut size: *mut opng_fsize_t,
) -> ::core::ffi::c_int {
    let mut sbuf: stat = stat {
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
    if fstat(fileno(stream), &raw mut sbuf) != 0 as ::core::ffi::c_int {
        return -(1 as ::core::ffi::c_int);
    }
    if sbuf.st_size < 0 as ::core::ffi::c_long {
        return -(1 as ::core::ffi::c_int);
    }
    *size = sbuf.st_size as opng_fsize_t;
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn opng_path_replace_dir(
    mut buffer: *mut ::core::ffi::c_char,
    mut bufsize: size_t,
    mut old_path: *const ::core::ffi::c_char,
    mut new_dirname: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut path: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut ptr: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut dirlen: size_t = 0;
    path = old_path;
    loop {
        ptr = strpbrk(path, OPNG_PATH_DIRSEP_ALL_STR.as_ptr());
        if ptr.is_null() {
            break;
        }
        path = ptr.offset(1 as ::core::ffi::c_int as isize);
    }
    dirlen = strlen(new_dirname);
    if dirlen.wrapping_add(strlen(path)).wrapping_add(2 as size_t) >= bufsize {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if dirlen > 0 as size_t {
        strcpy(buffer, new_dirname);
        if strchr(
            OPNG_PATH_DIRSEP_ALL_STR.as_ptr(),
            *buffer.offset(dirlen.wrapping_sub(1 as size_t) as isize) as ::core::ffi::c_int,
        )
        .is_null()
        {
            let fresh0 = dirlen;
            dirlen = dirlen.wrapping_add(1);
            *buffer.offset(fresh0 as isize) = OPNG_PATH_DIRSEP as ::core::ffi::c_char;
        }
    }
    strcpy(buffer.offset(dirlen as isize), path);
    return buffer;
}
#[no_mangle]
pub unsafe extern "C" fn opng_path_replace_ext(
    mut buffer: *mut ::core::ffi::c_char,
    mut bufsize: size_t,
    mut old_path: *const ::core::ffi::c_char,
    mut new_extname: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut i: size_t = 0;
    let mut pos: size_t = 0;
    if *new_extname.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        != OPNG_PATH_EXTSEP
    {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    i = 0 as size_t;
    pos = -(1 as ::core::ffi::c_int) as size_t;
    while *old_path.offset(i as isize) as ::core::ffi::c_int != '\0' as i32 {
        if i >= bufsize {
            return ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        let ref mut fresh1 = *buffer.offset(i as isize);
        *fresh1 = *old_path.offset(i as isize);
        if *fresh1 as ::core::ffi::c_int == OPNG_PATH_EXTSEP {
            pos = i;
        }
        i = i.wrapping_add(1);
    }
    if i > pos {
        i = pos;
    }
    loop {
        if i >= bufsize {
            return ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        let ref mut fresh2 = *buffer.offset(i as isize);
        *fresh2 = *new_extname;
        if *fresh2 as ::core::ffi::c_int == '\0' as i32 {
            return buffer;
        }
        i = i.wrapping_add(1);
        new_extname = new_extname.offset(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn opng_path_make_backup(
    mut buffer: *mut ::core::ffi::c_char,
    mut bufsize: size_t,
    mut path: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    static mut bak_extname: [::core::ffi::c_char; 5] =
        unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b".bak\0") };
    if strlen(path).wrapping_add(::core::mem::size_of::<[::core::ffi::c_char; 5]>() as size_t)
        > bufsize
    {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    strcpy(buffer, path);
    strcat(buffer, &raw const bak_extname as *const ::core::ffi::c_char);
    return buffer;
}
#[no_mangle]
pub unsafe extern "C" fn opng_os_rename(
    mut src_path: *const ::core::ffi::c_char,
    mut dest_path: *const ::core::ffi::c_char,
    mut clobber: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if clobber == 0 {
        if access(dest_path, OPNG_TEST_FILE) >= 0 as ::core::ffi::c_int {
            return -(1 as ::core::ffi::c_int);
        }
    }
    return rename(src_path, dest_path);
}
#[no_mangle]
pub unsafe extern "C" fn opng_os_copy_attr(
    mut src_path: *const ::core::ffi::c_char,
    mut dest_path: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut sbuf: stat = stat {
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
    let mut result: ::core::ffi::c_int = 0;
    if stat(src_path, &raw mut sbuf) != 0 as ::core::ffi::c_int {
        return -(1 as ::core::ffi::c_int);
    }
    result = 0 as ::core::ffi::c_int;
    chown(dest_path, sbuf.st_uid, sbuf.st_gid) != 0 as ::core::ffi::c_int;
    if chmod(dest_path, sbuf.st_mode) != 0 as ::core::ffi::c_int {
        result = -(1 as ::core::ffi::c_int);
    }
    let mut times: [timespec; 2] = [timespec {
        tv_sec: 0,
        tv_nsec: 0,
    }; 2];
    times[0 as ::core::ffi::c_int as usize] = sbuf.st_atim;
    times[1 as ::core::ffi::c_int as usize] = sbuf.st_mtim;
    if utimensat(
        AT_FDCWD,
        dest_path,
        &raw mut times as *mut timespec as *const timespec,
        0 as ::core::ffi::c_int,
    ) != 0 as ::core::ffi::c_int
    {
        result = -(1 as ::core::ffi::c_int);
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn opng_os_create_dir(
    mut dirname: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if *dirname.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '\0' as i32 {
        return 0 as ::core::ffi::c_int;
    }
    let mut sbuf: stat = stat {
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
    if stat(dirname, &raw mut sbuf) == 0 as ::core::ffi::c_int {
        return if sbuf.st_mode as ::core::ffi::c_uint & S_IFDIR as ::core::ffi::c_uint != 0 {
            0 as ::core::ffi::c_int
        } else {
            -(1 as ::core::ffi::c_int)
        };
    }
    return mkdir(dirname, 0o777 as __mode_t);
}
#[no_mangle]
pub unsafe extern "C" fn opng_os_test(
    mut path: *const ::core::ffi::c_char,
    mut mode: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut faccess: ::core::ffi::c_int = 0;
    let mut freg: ::core::ffi::c_int = 0;
    freg = 0 as ::core::ffi::c_int;
    faccess = freg;
    if !strchr(mode, 'f' as i32).is_null() {
        freg = 1 as ::core::ffi::c_int;
    }
    if !strchr(mode, 'r' as i32).is_null() {
        faccess |= OPNG_TEST_READ;
    }
    if !strchr(mode, 'w' as i32).is_null() {
        faccess |= OPNG_TEST_WRITE;
    }
    if !strchr(mode, 'x' as i32).is_null() {
        faccess |= OPNG_TEST_EXEC;
    }
    if faccess == 0 as ::core::ffi::c_int && freg == 0 {
        if strchr(mode, 'e' as i32).is_null() {
            return 0 as ::core::ffi::c_int;
        }
    }
    let mut sbuf: stat = stat {
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
    if stat(path, &raw mut sbuf) != 0 as ::core::ffi::c_int {
        return -(1 as ::core::ffi::c_int);
    }
    if freg != 0
        && sbuf.st_mode as ::core::ffi::c_uint & S_IFREG as ::core::ffi::c_uint
            != S_IFREG as ::core::ffi::c_uint
    {
        return -(1 as ::core::ffi::c_int);
    }
    if faccess == 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    return access(path, faccess);
}
#[no_mangle]
pub unsafe extern "C" fn opng_os_test_eq(
    mut path1: *const ::core::ffi::c_char,
    mut path2: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut sbuf1: stat = stat {
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
    let mut sbuf2: stat = stat {
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
    if stat(path1, &raw mut sbuf1) != 0 as ::core::ffi::c_int
        || stat(path2, &raw mut sbuf2) != 0 as ::core::ffi::c_int
    {
        return -(1 as ::core::ffi::c_int);
    }
    if sbuf1.st_dev == sbuf2.st_dev && sbuf1.st_ino == sbuf2.st_ino {
        return if sbuf1.st_ino != 0 as ::core::ffi::c_ulong {
            1 as ::core::ffi::c_int
        } else {
            -(1 as ::core::ffi::c_int)
        };
    } else {
        return 0 as ::core::ffi::c_int;
    };
}
#[no_mangle]
pub unsafe extern "C" fn opng_os_unlink(
    mut path: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    return unlink(path);
}
