extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn _json_c_strerror(errno_in: ::core::ffi::c_int) -> *mut ::core::ffi::c_char;
    static mut stderr: *mut FILE;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn getrandom(
        __buffer: *mut ::core::ffi::c_void,
        __length: size_t,
        __flags: ::core::ffi::c_uint,
    ) -> ssize_t;
    fn open(
        __file: *const ::core::ffi::c_char,
        __oflag: ::core::ffi::c_int,
        ...
    ) -> ::core::ffi::c_int;
    fn close(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn read(__fd: ::core::ffi::c_int, __buf: *mut ::core::ffi::c_void, __nbytes: size_t)
        -> ssize_t;
    fn __xstat(
        __ver: ::core::ffi::c_int,
        __filename: *const ::core::ffi::c_char,
        __stat_buf: *mut stat,
    ) -> ::core::ffi::c_int;
    fn time(__timer: *mut time_t) -> time_t;
}
pub type time_t = __time_t;
pub type __time_t = ::core::ffi::c_long;
pub type FILE = _IO_FILE;
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
pub type size_t = usize;
pub type __off64_t = ::core::ffi::c_long;
pub type _IO_lock_t = ();
pub type __off_t = ::core::ffi::c_long;
pub type ssize_t = __ssize_t;
pub type __ssize_t = ::core::ffi::c_long;
pub type __mode_t = ::core::ffi::c_uint;
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
pub type __syscall_slong_t = ::core::ffi::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type __blkcnt_t = ::core::ffi::c_long;
pub type __blksize_t = ::core::ffi::c_long;
pub type __dev_t = ::core::ffi::c_ulong;
pub type __gid_t = ::core::ffi::c_uint;
pub type __uid_t = ::core::ffi::c_uint;
pub type __nlink_t = ::core::ffi::c_ulong;
pub type __ino_t = ::core::ffi::c_ulong;
pub const EINTR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const EAGAIN: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const ENOSYS: ::core::ffi::c_int = 38 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const GRND_NONBLOCK: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
unsafe extern "C" fn get_getrandom_seed(mut seed: *mut ::core::ffi::c_int) -> ::core::ffi::c_int {
    let mut ret: ssize_t = 0;
    loop {
        ret = getrandom(
            seed as *mut ::core::ffi::c_void,
            ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
            GRND_NONBLOCK as ::core::ffi::c_uint,
        );
        if !(ret == -(1 as ::core::ffi::c_int) as ::core::ffi::c_long
            && *__errno_location() == EINTR)
        {
            break;
        }
    }
    if ret == -(1 as ::core::ffi::c_int) as ::core::ffi::c_long {
        if *__errno_location() == ENOSYS {
            return -(1 as ::core::ffi::c_int);
        }
        if *__errno_location() == EAGAIN {
            return -(1 as ::core::ffi::c_int);
        }
        fprintf(
            stderr,
            b"error from getrandom(): %s\0" as *const u8 as *const ::core::ffi::c_char,
            _json_c_strerror(*__errno_location()),
        );
        return -(1 as ::core::ffi::c_int);
    }
    if ret as usize != ::core::mem::size_of::<::core::ffi::c_int>() as usize {
        return -(1 as ::core::ffi::c_int);
    }
    return 0 as ::core::ffi::c_int;
}
pub const O_RDONLY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const _STAT_VER_LINUX: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const _STAT_VER: ::core::ffi::c_int = _STAT_VER_LINUX;
pub const __S_IFCHR: ::core::ffi::c_int = 0o20000 as ::core::ffi::c_int;
pub const S_IFCHR: ::core::ffi::c_int = __S_IFCHR;
#[inline]
unsafe extern "C" fn stat(
    mut __path: *const ::core::ffi::c_char,
    mut __statbuf: *mut stat,
) -> ::core::ffi::c_int {
    return __xstat(_STAT_VER, __path, __statbuf);
}
static mut dev_random_file: *const ::core::ffi::c_char =
    b"/dev/urandom\0" as *const u8 as *const ::core::ffi::c_char;
unsafe extern "C" fn get_dev_random_seed(mut seed: *mut ::core::ffi::c_int) -> ::core::ffi::c_int {
    let mut buf: stat = stat {
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
    if stat(dev_random_file, &raw mut buf) != 0 {
        return -(1 as ::core::ffi::c_int);
    }
    if buf.st_mode as ::core::ffi::c_uint & S_IFCHR as ::core::ffi::c_uint
        == 0 as ::core::ffi::c_uint
    {
        return -(1 as ::core::ffi::c_int);
    }
    let mut fd: ::core::ffi::c_int = open(dev_random_file, O_RDONLY);
    if fd < 0 as ::core::ffi::c_int {
        fprintf(
            stderr,
            b"error opening %s: %s\0" as *const u8 as *const ::core::ffi::c_char,
            dev_random_file,
            _json_c_strerror(*__errno_location()),
        );
        return -(1 as ::core::ffi::c_int);
    }
    let mut nread: ssize_t = read(
        fd,
        seed as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
    );
    close(fd);
    if nread as usize != ::core::mem::size_of::<::core::ffi::c_int>() as usize {
        fprintf(
            stderr,
            b"error short read %s: %s\0" as *const u8 as *const ::core::ffi::c_char,
            dev_random_file,
            _json_c_strerror(*__errno_location()),
        );
        return -(1 as ::core::ffi::c_int);
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn get_time_seed() -> ::core::ffi::c_int {
    return (time(::core::ptr::null_mut::<time_t>()) as ::core::ffi::c_uint)
        .wrapping_mul(433494437 as ::core::ffi::c_int as ::core::ffi::c_uint)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn json_c_get_random_seed() -> ::core::ffi::c_int {
    let mut seed: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if get_getrandom_seed(&raw mut seed) == 0 as ::core::ffi::c_int {
        return seed;
    }
    let mut seed_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if get_dev_random_seed(&raw mut seed_0) == 0 as ::core::ffi::c_int {
        return seed_0;
    }
    return get_time_seed();
}
