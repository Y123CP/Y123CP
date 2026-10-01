use core::ffi::*;
pub use crate::src::ffi::*;
pub use crate::src::c_consts::*;
pub use crate::src::c_structs::*;
pub use crate::src::c_types::*;
pub use crate::src::c_extern_types::*;
extern "C" {
    fn fgetpos(__stream: *mut FILE, __pos: *mut fpos_t) -> c_int;
    fn fsetpos(__stream: *mut FILE, __pos: *const fpos_t) -> c_int;
    fn chown(
        __file: *const c_char,
        __owner: __uid_t,
        __group: __gid_t,
    ) -> c_int;
    fn chmod(__file: *const c_char, __mode: __mode_t) -> c_int;
    fn mkdir(__path: *const c_char, __mode: __mode_t) -> c_int;
    fn utimensat(
        __fd: c_int,
        __path: *const c_char,
        __times: *const timespec,
        __flags: c_int,
    ) -> c_int;
    fn __fxstat(
        __ver: c_int,
        __fildes: c_int,
        __stat_buf: *mut stat,
    ) -> c_int;
    fn __xstat(
        __ver: c_int,
        __filename: *const c_char,
        __stat_buf: *mut stat,
    ) -> c_int;
}

pub type __dev_t = c_ulong;
pub type __uid_t = c_uint;
pub type __gid_t = c_uint;
pub type __ino_t = c_ulong;
pub type __mode_t = c_uint;
pub type __nlink_t = c_ulong;

pub type __time_t = c_long;
pub type __blksize_t = c_long;
pub type __blkcnt_t = c_long;
pub type __syscall_slong_t = c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __mbstate_t {
    pub __count: c_int,
    pub __value: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub __wch: c_uint,
    pub __wchb: [c_char; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _G_fpos_t {
    pub __pos: __off_t,
    pub __state: __mbstate_t,
}
pub type __fpos_t = _G_fpos_t;

pub type fpos_t = __fpos_t;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: c_int,
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

pub const AT_FDCWD: c_int = -(100 as c_int);
pub const R_OK: c_int = 4 as c_int;
pub const W_OK: c_int = 2 as c_int;
pub const X_OK: c_int = 1 as c_int;
pub const F_OK: c_int = 0 as c_int;
pub const _STAT_VER_LINUX: c_int = 1 as c_int;
pub const _STAT_VER: c_int = _STAT_VER_LINUX;
pub const __S_IFDIR: c_int = 0o40000 as c_int;
pub const __S_IFREG: c_int = 0o100000 as c_int;
pub const S_IFDIR: c_int = __S_IFDIR;
pub const S_IFREG: c_int = __S_IFREG;
#[inline]
unsafe extern "C" fn stat(
    mut __path: *const c_char,
    mut __statbuf: *mut stat,
) -> c_int {
    return __xstat(_STAT_VER, __path, __statbuf);
}
#[inline]
unsafe extern "C" fn fstat(
    mut __fd: c_int,
    mut __statbuf: *mut stat,
) -> c_int {
    return __fxstat(_STAT_VER, __fd, __statbuf);
}
pub const OPNG_PATH_DIRSEP: c_int = '/' as i32;
pub const OPNG_PATH_DIRSEP_ALL_STR: [c_char; 2] =
    unsafe { ::core::mem::transmute::<[u8; 2], [c_char; 2]>(*b"/\0") };
pub const OPNG_PATH_EXTSEP: c_int = '.' as i32;
pub const OPNG_TEST_READ: c_int = R_OK;
pub const OPNG_TEST_WRITE: c_int = W_OK;
pub const OPNG_TEST_EXEC: c_int = X_OK;
pub const OPNG_TEST_FILE: c_int = F_OK;
#[no_mangle]
pub unsafe extern "C" fn opng_ftello(mut stream: *mut FILE) -> opng_foffset_t {
    return ftell(stream);
}
#[no_mangle]
pub unsafe extern "C" fn opng_fseeko(
    mut stream: *mut FILE,
    mut offset: opng_foffset_t,
    mut whence: c_int,
) -> c_int {
    return fseek(stream, offset, whence);
}
#[no_mangle]
pub unsafe extern "C" fn opng_freado(
    mut stream: *mut FILE,
    mut offset: opng_foffset_t,
    mut whence: c_int,
    mut block: *mut c_void,
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
    if fgetpos(stream, &raw mut pos) != 0 as c_int {
        return 0 as size_t;
    }
    if opng_fseeko(stream, offset, whence) == 0 as c_int {
        result = fread(block, 1 as size_t, blocksize, stream) as size_t;
    } else {
        result = 0 as size_t;
    }
    if fsetpos(stream, &raw mut pos) != 0 as c_int {
        result = 0 as size_t;
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn opng_fwriteo(
    mut stream: *mut FILE,
    mut offset: opng_foffset_t,
    mut whence: c_int,
    mut block: *const c_void,
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
    if fgetpos(stream, &raw mut pos) != 0 as c_int
        || fflush(stream) != 0 as c_int
    {
        return 0 as size_t;
    }
    if opng_fseeko(stream, offset, whence) == 0 as c_int {
        result = fwrite(block, 1 as size_t, blocksize, stream) as size_t;
    } else {
        result = 0 as size_t;
    }
    if fflush(stream) != 0 as c_int {
        result = 0 as size_t;
    }
    if fsetpos(stream, &raw mut pos) != 0 as c_int {
        result = 0 as size_t;
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn opng_fgetsize(
    mut stream: *mut FILE,
    mut size: *mut opng_fsize_t,
) -> c_int {
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
    if fstat(fileno(stream), &raw mut sbuf) != 0 as c_int {
        return -(1 as c_int);
    }
    if sbuf.st_size < 0 as c_long {
        return -(1 as c_int);
    }
    *size = sbuf.st_size as opng_fsize_t;
    return 0 as c_int;
}
#[no_mangle]
pub unsafe extern "C" fn opng_path_replace_dir(
    mut buffer: *mut c_char,
    mut bufsize: size_t,
    mut old_path: *const c_char,
    mut new_dirname: *const c_char,
) -> *mut c_char {
    let mut path: *const c_char = ::core::ptr::null::<c_char>();
    let mut ptr: *const c_char = ::core::ptr::null::<c_char>();
    let mut dirlen: size_t = 0;
    path = old_path;
    loop {
        ptr = strpbrk(path, OPNG_PATH_DIRSEP_ALL_STR.as_ptr());
        if ptr.is_null() {
            break;
        }
        path = ptr.offset(1 as c_int as isize);
    }
    dirlen = strlen(new_dirname);
    if dirlen.wrapping_add(strlen(path)).wrapping_add(2 as size_t) >= bufsize {
        return ::core::ptr::null_mut::<c_char>();
    }
    if dirlen > 0 as size_t {
        strcpy(buffer, new_dirname);
        if strchr(
            OPNG_PATH_DIRSEP_ALL_STR.as_ptr(),
            *buffer.offset(dirlen.wrapping_sub(1 as size_t) as isize) as c_int,
        )
        .is_null()
        {
            let fresh0 = dirlen;
            dirlen = dirlen.wrapping_add(1);
            *buffer.offset(fresh0 as isize) = OPNG_PATH_DIRSEP as c_char;
        }
    }
    strcpy(buffer.offset(dirlen as isize), path);
    return buffer;
}
#[no_mangle]
pub unsafe extern "C" fn opng_path_replace_ext(
    mut buffer: *mut c_char,
    mut bufsize: size_t,
    mut old_path: *const c_char,
    mut new_extname: *const c_char,
) -> *mut c_char {
    let mut i: size_t = 0;
    let mut pos: size_t = 0;
    if *new_extname.offset(0 as c_int as isize) as c_int
        != OPNG_PATH_EXTSEP
    {
        return ::core::ptr::null_mut::<c_char>();
    }
    i = 0 as size_t;
    pos = -(1 as c_int) as size_t;
    while *old_path.offset(i as isize) as c_int != '\0' as i32 {
        if i >= bufsize {
            return ::core::ptr::null_mut::<c_char>();
        }
        let ref mut fresh1 = *buffer.offset(i as isize);
        *fresh1 = *old_path.offset(i as isize);
        if *fresh1 as c_int == OPNG_PATH_EXTSEP {
            pos = i;
        }
        i = i.wrapping_add(1);
    }
    if i > pos {
        i = pos;
    }
    loop {
        if i >= bufsize {
            return ::core::ptr::null_mut::<c_char>();
        }
        let ref mut fresh2 = *buffer.offset(i as isize);
        *fresh2 = *new_extname;
        if *fresh2 as c_int == '\0' as i32 {
            return buffer;
        }
        i = i.wrapping_add(1);
        new_extname = new_extname.offset(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn opng_path_make_backup(
    mut buffer: *mut c_char,
    mut bufsize: size_t,
    mut path: *const c_char,
) -> *mut c_char {
    static mut bak_extname: [c_char; 5] =
        unsafe { ::core::mem::transmute::<[u8; 5], [c_char; 5]>(*b".bak\0") };
    if strlen(path).wrapping_add(::core::mem::size_of::<[c_char; 5]>() as size_t)
        > bufsize
    {
        return ::core::ptr::null_mut::<c_char>();
    }
    strcpy(buffer, path);
    strcat(buffer, &raw const bak_extname as *const c_char);
    return buffer;
}
#[no_mangle]
pub unsafe extern "C" fn opng_os_rename(
    mut src_path: *const c_char,
    mut dest_path: *const c_char,
    mut clobber: c_int,
) -> c_int {
    if clobber == 0 {
        if access(dest_path, OPNG_TEST_FILE) >= 0 as c_int {
            return -(1 as c_int);
        }
    }
    return rename(src_path, dest_path);
}
#[no_mangle]
pub unsafe extern "C" fn opng_os_copy_attr(
    mut src_path: *const c_char,
    mut dest_path: *const c_char,
) -> c_int {
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
    let mut result: c_int = 0;
    if stat(src_path, &raw mut sbuf) != 0 as c_int {
        return -(1 as c_int);
    }
    result = 0 as c_int;
    chown(dest_path, sbuf.st_uid, sbuf.st_gid) != 0 as c_int;
    if chmod(dest_path, sbuf.st_mode) != 0 as c_int {
        result = -(1 as c_int);
    }
    let mut times: [timespec; 2] = [timespec {
        tv_sec: 0,
        tv_nsec: 0,
    }; 2];
    times[0 as c_int as usize] = sbuf.st_atim;
    times[1 as c_int as usize] = sbuf.st_mtim;
    if utimensat(
        AT_FDCWD,
        dest_path,
        &raw mut times as *mut timespec as *const timespec,
        0 as c_int,
    ) != 0 as c_int
    {
        result = -(1 as c_int);
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn opng_os_create_dir(
    mut dirname: *const c_char,
) -> c_int {
    if *dirname.offset(0 as c_int as isize) as c_int == '\0' as i32 {
        return 0 as c_int;
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
    if stat(dirname, &raw mut sbuf) == 0 as c_int {
        return if sbuf.st_mode as c_uint & S_IFDIR as c_uint != 0 {
            0 as c_int
        } else {
            -(1 as c_int)
        };
    }
    return mkdir(dirname, 0o777 as __mode_t);
}
#[no_mangle]
pub unsafe extern "C" fn opng_os_test(
    mut path: *const c_char,
    mut mode: *const c_char,
) -> c_int {
    let mut faccess: c_int = 0;
    let mut freg: c_int = 0;
    freg = 0 as c_int;
    faccess = freg;
    if !strchr(mode, 'f' as i32).is_null() {
        freg = 1 as c_int;
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
    if faccess == 0 as c_int && freg == 0 {
        if strchr(mode, 'e' as i32).is_null() {
            return 0 as c_int;
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
    if stat(path, &raw mut sbuf) != 0 as c_int {
        return -(1 as c_int);
    }
    if freg != 0
        && sbuf.st_mode as c_uint & S_IFREG as c_uint
            != S_IFREG as c_uint
    {
        return -(1 as c_int);
    }
    if faccess == 0 as c_int {
        return 0 as c_int;
    }
    return access(path, faccess);
}
#[no_mangle]
pub unsafe extern "C" fn opng_os_test_eq(
    mut path1: *const c_char,
    mut path2: *const c_char,
) -> c_int {
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
    if stat(path1, &raw mut sbuf1) != 0 as c_int
        || stat(path2, &raw mut sbuf2) != 0 as c_int
    {
        return -(1 as c_int);
    }
    if sbuf1.st_dev == sbuf2.st_dev && sbuf1.st_ino == sbuf2.st_ino {
        return if sbuf1.st_ino != 0 as c_ulong {
            1 as c_int
        } else {
            -(1 as c_int)
        };
    } else {
        return 0 as c_int;
    };
}
#[no_mangle]
pub unsafe extern "C" fn opng_os_unlink(
    mut path: *const c_char,
) -> c_int {
    return unlink(path);
}
