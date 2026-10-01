#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(extern_types)]
#![feature(linkage)]
#![feature(raw_ref_op)]
#[allow(unused_imports)]
use ::bzip2_1_0_8_raw;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn remove(__filename: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn fclose(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn fflush(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn fopen(
        __filename: *const ::core::ffi::c_char,
        __modes: *const ::core::ffi::c_char,
    ) -> *mut FILE;
    fn fdopen(__fd: ::core::ffi::c_int, __modes: *const ::core::ffi::c_char) -> *mut FILE;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn fgetc(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn ungetc(__c: ::core::ffi::c_int, __stream: *mut FILE) -> ::core::ffi::c_int;
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
    fn rewind(__stream: *mut FILE);
    fn ferror(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn perror(__s: *const ::core::ffi::c_char);
    fn fileno(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn exit(__status: ::core::ffi::c_int) -> !;
    fn getenv(__name: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn strcpy(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strncpy(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> *mut ::core::ffi::c_char;
    fn strcat(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strncmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn strstr(
        __haystack: *const ::core::ffi::c_char,
        __needle: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn strerror(__errnum: ::core::ffi::c_int) -> *mut ::core::ffi::c_char;
    fn signal(__sig: ::core::ffi::c_int, __handler: __sighandler_t) -> __sighandler_t;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn __ctype_b_loc() -> *mut *const ::core::ffi::c_ushort;
    fn BZ2_bzReadOpen(
        bzerror: *mut ::core::ffi::c_int,
        f: *mut FILE,
        verbosity_0: ::core::ffi::c_int,
        small: ::core::ffi::c_int,
        unused: *mut ::core::ffi::c_void,
        nUnused: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_void;
    fn BZ2_bzReadClose(bzerror: *mut ::core::ffi::c_int, b: *mut ::core::ffi::c_void);
    fn BZ2_bzReadGetUnused(
        bzerror: *mut ::core::ffi::c_int,
        b: *mut ::core::ffi::c_void,
        unused: *mut *mut ::core::ffi::c_void,
        nUnused: *mut ::core::ffi::c_int,
    );
    fn BZ2_bzRead(
        bzerror: *mut ::core::ffi::c_int,
        b: *mut ::core::ffi::c_void,
        buf: *mut ::core::ffi::c_void,
        len: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn BZ2_bzWriteOpen(
        bzerror: *mut ::core::ffi::c_int,
        f: *mut FILE,
        blockSize100k_0: ::core::ffi::c_int,
        verbosity_0: ::core::ffi::c_int,
        workFactor_0: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_void;
    fn BZ2_bzWrite(
        bzerror: *mut ::core::ffi::c_int,
        b: *mut ::core::ffi::c_void,
        buf: *mut ::core::ffi::c_void,
        len: ::core::ffi::c_int,
    );
    fn BZ2_bzWriteClose64(
        bzerror: *mut ::core::ffi::c_int,
        b: *mut ::core::ffi::c_void,
        abandon: ::core::ffi::c_int,
        nbytes_in_lo32: *mut ::core::ffi::c_uint,
        nbytes_in_hi32: *mut ::core::ffi::c_uint,
        nbytes_out_lo32: *mut ::core::ffi::c_uint,
        nbytes_out_hi32: *mut ::core::ffi::c_uint,
    );
    fn BZ2_bzlibVersion() -> *const ::core::ffi::c_char;
    fn open(
        __file: *const ::core::ffi::c_char,
        __oflag: ::core::ffi::c_int,
        ...
    ) -> ::core::ffi::c_int;
    fn utime(
        __file: *const ::core::ffi::c_char,
        __file_times: *const utimbuf,
    ) -> ::core::ffi::c_int;
    fn close(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn fchown(__fd: ::core::ffi::c_int, __owner: __uid_t, __group: __gid_t) -> ::core::ffi::c_int;
    fn isatty(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn fchmod(__fd: ::core::ffi::c_int, __mode: __mode_t) -> ::core::ffi::c_int;
    fn __xstat(
        __ver: ::core::ffi::c_int,
        __filename: *const ::core::ffi::c_char,
        __stat_buf: *mut stat,
    ) -> ::core::ffi::c_int;
    fn __lxstat(
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
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type __sighandler_t = Option<unsafe extern "C" fn(::core::ffi::c_int) -> ()>;
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
pub type BZFILE = ();
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
pub struct utimbuf {
    pub actime: __time_t,
    pub modtime: __time_t,
}
pub type Char = ::core::ffi::c_char;
pub type Bool = ::core::ffi::c_uchar;
pub type UChar = ::core::ffi::c_uchar;
pub type Int32 = ::core::ffi::c_int;
pub type UInt32 = ::core::ffi::c_uint;
pub type Int16 = ::core::ffi::c_short;
pub type UInt16 = ::core::ffi::c_ushort;
pub type IntNative = ::core::ffi::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct UInt64 {
    pub b: [UChar; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct zzzz {
    pub name: *mut Char,
    pub link: *mut zzzz,
}
pub type Cell = zzzz;
pub const O_WRONLY: ::core::ffi::c_int = 0o1 as ::core::ffi::c_int;
pub const O_CREAT: ::core::ffi::c_int = 0o100 as ::core::ffi::c_int;
pub const O_EXCL: ::core::ffi::c_int = 0o200 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const EOF: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
pub const BZ_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const BZ_STREAM_END: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const BZ_MEM_ERROR: ::core::ffi::c_int = -3;
pub const BZ_DATA_ERROR: ::core::ffi::c_int = -4;
pub const BZ_DATA_ERROR_MAGIC: ::core::ffi::c_int = -5;
pub const BZ_IO_ERROR: ::core::ffi::c_int = -6;
pub const BZ_UNEXPECTED_EOF: ::core::ffi::c_int = -7;
pub const BZ_CONFIG_ERROR: ::core::ffi::c_int = -9;
pub const _STAT_VER_LINUX: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const _STAT_VER: ::core::ffi::c_int = _STAT_VER_LINUX;
pub const __S_IFMT: ::core::ffi::c_int = 0o170000 as ::core::ffi::c_int;
pub const __S_IREAD: ::core::ffi::c_int = 0o400 as ::core::ffi::c_int;
pub const __S_IWRITE: ::core::ffi::c_int = 0o200 as ::core::ffi::c_int;
pub const S_IRUSR: ::core::ffi::c_int = __S_IREAD;
pub const S_IWUSR: ::core::ffi::c_int = __S_IWRITE;
#[inline]
unsafe extern "C" fn stat(
    mut __path: *const ::core::ffi::c_char,
    mut __statbuf: *mut stat,
) -> ::core::ffi::c_int {
    return __xstat(_STAT_VER, __path, __statbuf);
}
#[inline]
unsafe extern "C" fn lstat(
    mut __path: *const ::core::ffi::c_char,
    mut __statbuf: *mut stat,
) -> ::core::ffi::c_int {
    return __lxstat(_STAT_VER, __path, __statbuf);
}
pub const PATH_SEP: ::core::ffi::c_int = '/' as i32;
pub const True: Bool = 1 as ::core::ffi::c_int as Bool;
pub const False: Bool = 0 as ::core::ffi::c_int as Bool;
#[no_mangle]
pub static mut verbosity: Int32 = 0;
#[no_mangle]
pub static mut smallMode: Bool = 0;
#[no_mangle]
pub static mut deleteOutputOnInterrupt: Bool = 0;
#[no_mangle]
pub static mut keepInputFiles: Bool = 0;
#[no_mangle]
pub static mut noisy: Bool = 0;
#[no_mangle]
pub static mut forceOverwrite: Bool = 0;
#[no_mangle]
pub static mut testFailsExist: Bool = 0;
#[no_mangle]
pub static mut unzFailsExist: Bool = 0;
#[no_mangle]
pub static mut numFileNames: Int32 = 0;
#[no_mangle]
pub static mut numFilesProcessed: Int32 = 0;
#[no_mangle]
pub static mut blockSize100k: Int32 = 0;
#[no_mangle]
pub static mut exitValue: Int32 = 0;
pub const SM_I2O: ::core::ffi::c_int = 1;
pub const SM_F2O: ::core::ffi::c_int = 2;
pub const SM_F2F: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const OM_Z: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const OM_UNZ: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const OM_TEST: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
#[no_mangle]
pub static mut opMode: Int32 = 0;
#[no_mangle]
pub static mut srcMode: Int32 = 0;
pub const FILE_NAME_LEN: ::core::ffi::c_int = 1034 as ::core::ffi::c_int;
#[no_mangle]
pub static mut longestFileName: Int32 = 0;
#[no_mangle]
pub static mut inName: [Char; 1034] = [0; 1034];
#[no_mangle]
pub static mut outName: [Char; 1034] = [0; 1034];
#[no_mangle]
pub static mut tmpName: [Char; 1034] = [0; 1034];
#[no_mangle]
pub static mut progName: *mut Char = ::core::ptr::null::<Char>() as *mut Char;
#[no_mangle]
pub static mut progNameReally: [Char; 1034] = [0; 1034];
#[no_mangle]
pub static mut outputHandleJustInCase: *mut FILE = ::core::ptr::null::<FILE>() as *mut FILE;
#[no_mangle]
pub static mut workFactor: Int32 = 0;
unsafe extern "C" fn uInt64_from_UInt32s(mut n: *mut UInt64, mut lo32: UInt32, mut hi32: UInt32) {
    (*n).b[7 as ::core::ffi::c_int as usize] = (hi32 as ::core::ffi::c_uint
        >> 24 as ::core::ffi::c_int
        & 0xff as ::core::ffi::c_uint) as UChar;
    (*n).b[6 as ::core::ffi::c_int as usize] = (hi32 as ::core::ffi::c_uint
        >> 16 as ::core::ffi::c_int
        & 0xff as ::core::ffi::c_uint) as UChar;
    (*n).b[5 as ::core::ffi::c_int as usize] = (hi32 as ::core::ffi::c_uint
        >> 8 as ::core::ffi::c_int
        & 0xff as ::core::ffi::c_uint) as UChar;
    (*n).b[4 as ::core::ffi::c_int as usize] =
        (hi32 as ::core::ffi::c_uint & 0xff as ::core::ffi::c_uint) as UChar;
    (*n).b[3 as ::core::ffi::c_int as usize] = (lo32 as ::core::ffi::c_uint
        >> 24 as ::core::ffi::c_int
        & 0xff as ::core::ffi::c_uint) as UChar;
    (*n).b[2 as ::core::ffi::c_int as usize] = (lo32 as ::core::ffi::c_uint
        >> 16 as ::core::ffi::c_int
        & 0xff as ::core::ffi::c_uint) as UChar;
    (*n).b[1 as ::core::ffi::c_int as usize] = (lo32 as ::core::ffi::c_uint
        >> 8 as ::core::ffi::c_int
        & 0xff as ::core::ffi::c_uint) as UChar;
    (*n).b[0 as ::core::ffi::c_int as usize] =
        (lo32 as ::core::ffi::c_uint & 0xff as ::core::ffi::c_uint) as UChar;
}
unsafe extern "C" fn uInt64_to_double(mut n: *mut UInt64) -> ::core::ffi::c_double {
    let mut i: Int32 = 0;
    let mut base: ::core::ffi::c_double = 1.0f64;
    let mut sum: ::core::ffi::c_double = 0.0f64;
    i = 0 as ::core::ffi::c_int as Int32;
    while i < 8 as ::core::ffi::c_int {
        sum += base * (*n).b[i as usize] as ::core::ffi::c_double;
        base *= 256.0f64;
        i += 1;
    }
    return sum;
}
unsafe extern "C" fn uInt64_isZero(mut n: *mut UInt64) -> Bool {
    let mut i: Int32 = 0;
    i = 0 as ::core::ffi::c_int as Int32;
    while i < 8 as ::core::ffi::c_int {
        if (*n).b[i as usize] as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
            return 0 as Bool;
        }
        i += 1;
    }
    return 1 as Bool;
}
unsafe extern "C" fn uInt64_qrm10(mut n: *mut UInt64) -> Int32 {
    let mut rem: UInt32 = 0;
    let mut tmp: UInt32 = 0;
    let mut i: Int32 = 0;
    rem = 0 as UInt32;
    i = 7 as ::core::ffi::c_int as Int32;
    while i >= 0 as ::core::ffi::c_int {
        tmp = (rem as ::core::ffi::c_uint)
            .wrapping_mul(256 as ::core::ffi::c_uint)
            .wrapping_add((*n).b[i as usize] as ::core::ffi::c_uint) as UInt32;
        (*n).b[i as usize] =
            (tmp as ::core::ffi::c_uint).wrapping_div(10 as ::core::ffi::c_uint) as UChar;
        rem = (tmp as ::core::ffi::c_uint).wrapping_rem(10 as ::core::ffi::c_uint) as UInt32;
        i -= 1;
    }
    return rem as Int32;
}
unsafe extern "C" fn uInt64_toAscii(mut outbuf: *mut ::core::ffi::c_char, mut n: *mut UInt64) {
    let mut i: Int32 = 0;
    let mut q: Int32 = 0;
    let mut buf: [UChar; 32] = [0; 32];
    let mut nBuf: Int32 = 0 as Int32;
    let mut n_copy: UInt64 = *n;
    loop {
        q = uInt64_qrm10(&raw mut n_copy);
        buf[nBuf as usize] = (q as ::core::ffi::c_int + '0' as i32) as UChar;
        nBuf += 1;
        if !(uInt64_isZero(&raw mut n_copy) == 0) {
            break;
        }
    }
    *outbuf.offset(nBuf as isize) = 0 as ::core::ffi::c_char;
    i = 0 as ::core::ffi::c_int as Int32;
    while i < nBuf {
        *outbuf.offset(i as isize) =
            buf[(nBuf as ::core::ffi::c_int - i as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                as usize] as ::core::ffi::c_char;
        i += 1;
    }
}
unsafe extern "C" fn myfeof(mut f: *mut FILE) -> Bool {
    let mut c: Int32 = fgetc(f) as Int32;
    if c == EOF {
        return True;
    }
    ungetc(c as ::core::ffi::c_int, f);
    return False;
}
unsafe extern "C" fn compressStream(mut stream: *mut FILE, mut zStream: *mut FILE) {
    let mut current_block: u64;
    let mut bzf: *mut ::core::ffi::c_void = NULL as *mut ::core::ffi::c_void;
    let mut ibuf: [UChar; 5000] = [0; 5000];
    let mut nIbuf: Int32 = 0;
    let mut nbytes_in_lo32: UInt32 = 0;
    let mut nbytes_in_hi32: UInt32 = 0;
    let mut nbytes_out_lo32: UInt32 = 0;
    let mut nbytes_out_hi32: UInt32 = 0;
    let mut bzerr: Int32 = 0;
    let mut bzerr_dummy: Int32 = 0;
    let mut ret: Int32 = 0;
    if !(ferror(stream) != 0) {
        if !(ferror(zStream) != 0) {
            bzf = BZ2_bzWriteOpen(
                &raw mut bzerr,
                zStream,
                blockSize100k as ::core::ffi::c_int,
                verbosity as ::core::ffi::c_int,
                workFactor as ::core::ffi::c_int,
            );
            if bzerr != BZ_OK {
                current_block = 12129979386315422097;
            } else {
                if verbosity >= 2 as ::core::ffi::c_int {
                    fprintf(stderr, b"\n\0" as *const u8 as *const ::core::ffi::c_char);
                }
                loop {
                    if !(True != 0) {
                        current_block = 9606288038608642794;
                        break;
                    }
                    if myfeof(stream) != 0 {
                        current_block = 9606288038608642794;
                        break;
                    }
                    nIbuf = fread(
                        &raw mut ibuf as *mut UChar as *mut ::core::ffi::c_void,
                        ::core::mem::size_of::<UChar>() as size_t,
                        5000 as size_t,
                        stream,
                    ) as Int32;
                    if ferror(stream) != 0 {
                        current_block = 3591254770243040807;
                        break;
                    }
                    if nIbuf > 0 as ::core::ffi::c_int {
                        BZ2_bzWrite(
                            &raw mut bzerr,
                            bzf,
                            &raw mut ibuf as *mut UChar as *mut ::core::ffi::c_void,
                            nIbuf as ::core::ffi::c_int,
                        );
                    }
                    if bzerr != BZ_OK {
                        current_block = 12129979386315422097;
                        break;
                    }
                }
                match current_block {
                    3591254770243040807 => {}
                    12129979386315422097 => {}
                    _ => {
                        BZ2_bzWriteClose64(
                            &raw mut bzerr,
                            bzf,
                            0 as ::core::ffi::c_int,
                            &raw mut nbytes_in_lo32,
                            &raw mut nbytes_in_hi32,
                            &raw mut nbytes_out_lo32,
                            &raw mut nbytes_out_hi32,
                        );
                        if bzerr != BZ_OK {
                            current_block = 12129979386315422097;
                        } else if ferror(zStream) != 0 {
                            current_block = 3591254770243040807;
                        } else {
                            ret = fflush(zStream) as Int32;
                            if ret == EOF {
                                current_block = 3591254770243040807;
                            } else {
                                if zStream != stdout {
                                    let mut fd: Int32 = fileno(zStream) as Int32;
                                    if fd < 0 as ::core::ffi::c_int {
                                        current_block = 3591254770243040807;
                                    } else {
                                        applySavedFileAttrToOutputFile(fd as IntNative);
                                        ret = fclose(zStream) as Int32;
                                        outputHandleJustInCase = ::core::ptr::null_mut::<FILE>();
                                        if ret == EOF {
                                            current_block = 3591254770243040807;
                                        } else {
                                            current_block = 9828876828309294594;
                                        }
                                    }
                                } else {
                                    current_block = 9828876828309294594;
                                }
                                match current_block {
                                    3591254770243040807 => {}
                                    _ => {
                                        outputHandleJustInCase = ::core::ptr::null_mut::<FILE>();
                                        if ferror(stream) != 0 {
                                            current_block = 3591254770243040807;
                                        } else {
                                            ret = fclose(stream) as Int32;
                                            if ret == EOF {
                                                current_block = 3591254770243040807;
                                            } else {
                                                if verbosity >= 1 as ::core::ffi::c_int {
                                                    if nbytes_in_lo32 == 0 as ::core::ffi::c_uint
                                                        && nbytes_in_hi32
                                                            == 0 as ::core::ffi::c_uint
                                                    {
                                                        fprintf(
                                                            stderr,
                                                            b" no data compressed.\n\0" as *const u8
                                                                as *const ::core::ffi::c_char,
                                                        );
                                                    } else {
                                                        let mut buf_nin: [Char; 32] = [0; 32];
                                                        let mut buf_nout: [Char; 32] = [0; 32];
                                                        let mut nbytes_in: UInt64 =
                                                            UInt64 { b: [0; 8] };
                                                        let mut nbytes_out: UInt64 =
                                                            UInt64 { b: [0; 8] };
                                                        let mut nbytes_in_d: ::core::ffi::c_double =
                                                            0.;
                                                        let mut nbytes_out_d: ::core::ffi::c_double = 0.;
                                                        uInt64_from_UInt32s(
                                                            &raw mut nbytes_in,
                                                            nbytes_in_lo32,
                                                            nbytes_in_hi32,
                                                        );
                                                        uInt64_from_UInt32s(
                                                            &raw mut nbytes_out,
                                                            nbytes_out_lo32,
                                                            nbytes_out_hi32,
                                                        );
                                                        nbytes_in_d =
                                                            uInt64_to_double(&raw mut nbytes_in);
                                                        nbytes_out_d =
                                                            uInt64_to_double(&raw mut nbytes_out);
                                                        uInt64_toAscii(
                                                            &raw mut buf_nin
                                                                as *mut ::core::ffi::c_char,
                                                            &raw mut nbytes_in,
                                                        );
                                                        uInt64_toAscii(
                                                            &raw mut buf_nout
                                                                as *mut ::core::ffi::c_char,
                                                            &raw mut nbytes_out,
                                                        );
                                                        fprintf(
                                                            stderr,
                                                            b"%6.3f:1, %6.3f bits/byte, %5.2f%% saved, %s in, %s out.\n\0"
                                                                as *const u8 as *const ::core::ffi::c_char,
                                                            nbytes_in_d / nbytes_out_d,
                                                            8.0f64 * nbytes_out_d / nbytes_in_d,
                                                            100.0f64 * (1.0f64 - nbytes_out_d / nbytes_in_d),
                                                            &raw mut buf_nin as *mut Char,
                                                            &raw mut buf_nout as *mut Char,
                                                        );
                                                    }
                                                }
                                                return;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            match current_block {
                3591254770243040807 => {}
                _ => {
                    BZ2_bzWriteClose64(
                        &raw mut bzerr_dummy,
                        bzf,
                        1 as ::core::ffi::c_int,
                        &raw mut nbytes_in_lo32,
                        &raw mut nbytes_in_hi32,
                        &raw mut nbytes_out_lo32,
                        &raw mut nbytes_out_hi32,
                    );
                    match bzerr {
                        BZ_CONFIG_ERROR => {
                            current_block = 16680933752313891718;
                            match current_block {
                                1303145445373317379 => {
                                    panic(
                                        b"compress:unexpected error\0" as *const u8 as *const Char,
                                    );
                                }
                                8332388060466130611 => {
                                    outOfMemory();
                                }
                                _ => {
                                    configError();
                                }
                            }
                        }
                        BZ_MEM_ERROR => {
                            current_block = 8332388060466130611;
                            match current_block {
                                1303145445373317379 => {
                                    panic(
                                        b"compress:unexpected error\0" as *const u8 as *const Char,
                                    );
                                }
                                8332388060466130611 => {
                                    outOfMemory();
                                }
                                _ => {
                                    configError();
                                }
                            }
                        }
                        BZ_IO_ERROR => {}
                        _ => {
                            current_block = 1303145445373317379;
                            match current_block {
                                1303145445373317379 => {
                                    panic(
                                        b"compress:unexpected error\0" as *const u8 as *const Char,
                                    );
                                }
                                8332388060466130611 => {
                                    outOfMemory();
                                }
                                _ => {
                                    configError();
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    ioError();
}
unsafe extern "C" fn uncompressStream(mut zStream: *mut FILE, mut stream: *mut FILE) -> Bool {
    let mut current_block: u64;
    let mut bzf: *mut ::core::ffi::c_void = NULL as *mut ::core::ffi::c_void;
    let mut bzerr: Int32 = 0;
    let mut bzerr_dummy: Int32 = 0;
    let mut ret: Int32 = 0;
    let mut nread: Int32 = 0;
    let mut streamNo: Int32 = 0;
    let mut i: Int32 = 0;
    let mut obuf: [UChar; 5000] = [0; 5000];
    let mut unused: [UChar; 5000] = [0; 5000];
    let mut nUnused: Int32 = 0;
    let mut unusedTmpV: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut unusedTmp: *mut UChar = ::core::ptr::null_mut::<UChar>();
    nUnused = 0 as ::core::ffi::c_int as Int32;
    streamNo = 0 as ::core::ffi::c_int as Int32;
    if !(ferror(stream) != 0) {
        if !(ferror(zStream) != 0) {
            's_37: loop {
                if !(True != 0) {
                    current_block = 6901905606216337417;
                    break;
                }
                bzf = BZ2_bzReadOpen(
                    &raw mut bzerr,
                    zStream,
                    verbosity as ::core::ffi::c_int,
                    smallMode as ::core::ffi::c_int,
                    &raw mut unused as *mut UChar as *mut ::core::ffi::c_void,
                    nUnused as ::core::ffi::c_int,
                );
                if bzf.is_null() || bzerr != BZ_OK {
                    current_block = 17481014958606503716;
                    break;
                }
                streamNo += 1;
                while bzerr == BZ_OK {
                    nread = BZ2_bzRead(
                        &raw mut bzerr,
                        bzf,
                        &raw mut obuf as *mut UChar as *mut ::core::ffi::c_void,
                        5000 as ::core::ffi::c_int,
                    ) as Int32;
                    if bzerr == BZ_DATA_ERROR_MAGIC {
                        current_block = 11119926950007664940;
                        break 's_37;
                    }
                    if (bzerr == BZ_OK || bzerr == BZ_STREAM_END) && nread > 0 as ::core::ffi::c_int
                    {
                        fwrite(
                            &raw mut obuf as *mut UChar as *const ::core::ffi::c_void,
                            ::core::mem::size_of::<UChar>() as size_t,
                            nread as size_t,
                            stream,
                        );
                    }
                    if ferror(stream) != 0 {
                        current_block = 9121920734004483035;
                        break 's_37;
                    }
                }
                if bzerr != BZ_STREAM_END {
                    current_block = 17481014958606503716;
                    break;
                }
                BZ2_bzReadGetUnused(&raw mut bzerr, bzf, &raw mut unusedTmpV, &raw mut nUnused);
                if bzerr != BZ_OK {
                    panic(b"decompress:bzReadGetUnused\0" as *const u8 as *const Char);
                }
                unusedTmp = unusedTmpV as *mut UChar;
                i = 0 as ::core::ffi::c_int as Int32;
                while i < nUnused {
                    unused[i as usize] = *unusedTmp.offset(i as isize);
                    i += 1;
                }
                BZ2_bzReadClose(&raw mut bzerr, bzf);
                if bzerr != BZ_OK {
                    panic(b"decompress:bzReadGetUnused\0" as *const u8 as *const Char);
                }
                if nUnused == 0 as ::core::ffi::c_int && myfeof(zStream) as ::core::ffi::c_int != 0
                {
                    current_block = 6901905606216337417;
                    break;
                }
            }
            match current_block {
                9121920734004483035 => {}
                _ => {
                    match current_block {
                        11119926950007664940 => {
                            if forceOverwrite != 0 {
                                rewind(zStream);
                                loop {
                                    if !(True != 0) {
                                        current_block = 6901905606216337417;
                                        break;
                                    }
                                    if myfeof(zStream) != 0 {
                                        current_block = 6901905606216337417;
                                        break;
                                    }
                                    nread = fread(
                                        &raw mut obuf as *mut UChar as *mut ::core::ffi::c_void,
                                        ::core::mem::size_of::<UChar>() as size_t,
                                        5000 as size_t,
                                        zStream,
                                    ) as Int32;
                                    if ferror(zStream) != 0 {
                                        current_block = 9121920734004483035;
                                        break;
                                    }
                                    if nread > 0 as ::core::ffi::c_int {
                                        fwrite(
                                            &raw mut obuf as *mut UChar
                                                as *const ::core::ffi::c_void,
                                            ::core::mem::size_of::<UChar>() as size_t,
                                            nread as size_t,
                                            stream,
                                        );
                                    }
                                    if ferror(stream) != 0 {
                                        current_block = 9121920734004483035;
                                        break;
                                    }
                                }
                            } else {
                                current_block = 17481014958606503716;
                            }
                        }
                        _ => {}
                    }
                    match current_block {
                        9121920734004483035 => {}
                        _ => match current_block {
                            17481014958606503716 => {
                                BZ2_bzReadClose(&raw mut bzerr_dummy, bzf);
                                match bzerr {
                                    BZ_CONFIG_ERROR => {
                                        current_block = 1237003964653440741;
                                        match current_block {
                                            18124707625368173585 => {
                                                panic(
                                                    b"decompress:unexpected error\0" as *const u8
                                                        as *const Char,
                                                );
                                            }
                                            1237003964653440741 => {
                                                configError();
                                            }
                                            8219249204218837607 => {
                                                crcError();
                                            }
                                            8179156205194423916 => {
                                                outOfMemory();
                                            }
                                            3716497938086561230 => {
                                                compressedStreamEOF();
                                            }
                                            _ => {
                                                if zStream != stdin {
                                                    fclose(zStream);
                                                }
                                                if stream != stdout {
                                                    fclose(stream);
                                                }
                                                if streamNo == 1 as ::core::ffi::c_int {
                                                    return False;
                                                } else {
                                                    if noisy != 0 {
                                                        fprintf(
                                                                stderr,
                                                                b"\n%s: %s: trailing garbage after EOF ignored\n\0"
                                                                    as *const u8 as *const ::core::ffi::c_char,
                                                                progName,
                                                                &raw mut inName as *mut Char,
                                                            );
                                                    }
                                                    return True;
                                                }
                                            }
                                        }
                                    }
                                    BZ_IO_ERROR => {}
                                    BZ_DATA_ERROR => {
                                        current_block = 8219249204218837607;
                                        match current_block {
                                            18124707625368173585 => {
                                                panic(
                                                    b"decompress:unexpected error\0" as *const u8
                                                        as *const Char,
                                                );
                                            }
                                            1237003964653440741 => {
                                                configError();
                                            }
                                            8219249204218837607 => {
                                                crcError();
                                            }
                                            8179156205194423916 => {
                                                outOfMemory();
                                            }
                                            3716497938086561230 => {
                                                compressedStreamEOF();
                                            }
                                            _ => {
                                                if zStream != stdin {
                                                    fclose(zStream);
                                                }
                                                if stream != stdout {
                                                    fclose(stream);
                                                }
                                                if streamNo == 1 as ::core::ffi::c_int {
                                                    return False;
                                                } else {
                                                    if noisy != 0 {
                                                        fprintf(
                                                                stderr,
                                                                b"\n%s: %s: trailing garbage after EOF ignored\n\0"
                                                                    as *const u8 as *const ::core::ffi::c_char,
                                                                progName,
                                                                &raw mut inName as *mut Char,
                                                            );
                                                    }
                                                    return True;
                                                }
                                            }
                                        }
                                    }
                                    BZ_MEM_ERROR => {
                                        current_block = 8179156205194423916;
                                        match current_block {
                                            18124707625368173585 => {
                                                panic(
                                                    b"decompress:unexpected error\0" as *const u8
                                                        as *const Char,
                                                );
                                            }
                                            1237003964653440741 => {
                                                configError();
                                            }
                                            8219249204218837607 => {
                                                crcError();
                                            }
                                            8179156205194423916 => {
                                                outOfMemory();
                                            }
                                            3716497938086561230 => {
                                                compressedStreamEOF();
                                            }
                                            _ => {
                                                if zStream != stdin {
                                                    fclose(zStream);
                                                }
                                                if stream != stdout {
                                                    fclose(stream);
                                                }
                                                if streamNo == 1 as ::core::ffi::c_int {
                                                    return False;
                                                } else {
                                                    if noisy != 0 {
                                                        fprintf(
                                                                stderr,
                                                                b"\n%s: %s: trailing garbage after EOF ignored\n\0"
                                                                    as *const u8 as *const ::core::ffi::c_char,
                                                                progName,
                                                                &raw mut inName as *mut Char,
                                                            );
                                                    }
                                                    return True;
                                                }
                                            }
                                        }
                                    }
                                    BZ_UNEXPECTED_EOF => {
                                        current_block = 3716497938086561230;
                                        match current_block {
                                            18124707625368173585 => {
                                                panic(
                                                    b"decompress:unexpected error\0" as *const u8
                                                        as *const Char,
                                                );
                                            }
                                            1237003964653440741 => {
                                                configError();
                                            }
                                            8219249204218837607 => {
                                                crcError();
                                            }
                                            8179156205194423916 => {
                                                outOfMemory();
                                            }
                                            3716497938086561230 => {
                                                compressedStreamEOF();
                                            }
                                            _ => {
                                                if zStream != stdin {
                                                    fclose(zStream);
                                                }
                                                if stream != stdout {
                                                    fclose(stream);
                                                }
                                                if streamNo == 1 as ::core::ffi::c_int {
                                                    return False;
                                                } else {
                                                    if noisy != 0 {
                                                        fprintf(
                                                                stderr,
                                                                b"\n%s: %s: trailing garbage after EOF ignored\n\0"
                                                                    as *const u8 as *const ::core::ffi::c_char,
                                                                progName,
                                                                &raw mut inName as *mut Char,
                                                            );
                                                    }
                                                    return True;
                                                }
                                            }
                                        }
                                    }
                                    BZ_DATA_ERROR_MAGIC => {
                                        current_block = 17634614082919660375;
                                        match current_block {
                                            18124707625368173585 => {
                                                panic(
                                                    b"decompress:unexpected error\0" as *const u8
                                                        as *const Char,
                                                );
                                            }
                                            1237003964653440741 => {
                                                configError();
                                            }
                                            8219249204218837607 => {
                                                crcError();
                                            }
                                            8179156205194423916 => {
                                                outOfMemory();
                                            }
                                            3716497938086561230 => {
                                                compressedStreamEOF();
                                            }
                                            _ => {
                                                if zStream != stdin {
                                                    fclose(zStream);
                                                }
                                                if stream != stdout {
                                                    fclose(stream);
                                                }
                                                if streamNo == 1 as ::core::ffi::c_int {
                                                    return False;
                                                } else {
                                                    if noisy != 0 {
                                                        fprintf(
                                                                stderr,
                                                                b"\n%s: %s: trailing garbage after EOF ignored\n\0"
                                                                    as *const u8 as *const ::core::ffi::c_char,
                                                                progName,
                                                                &raw mut inName as *mut Char,
                                                            );
                                                    }
                                                    return True;
                                                }
                                            }
                                        }
                                    }
                                    _ => {
                                        current_block = 18124707625368173585;
                                        match current_block {
                                            18124707625368173585 => {
                                                panic(
                                                    b"decompress:unexpected error\0" as *const u8
                                                        as *const Char,
                                                );
                                            }
                                            1237003964653440741 => {
                                                configError();
                                            }
                                            8219249204218837607 => {
                                                crcError();
                                            }
                                            8179156205194423916 => {
                                                outOfMemory();
                                            }
                                            3716497938086561230 => {
                                                compressedStreamEOF();
                                            }
                                            _ => {
                                                if zStream != stdin {
                                                    fclose(zStream);
                                                }
                                                if stream != stdout {
                                                    fclose(stream);
                                                }
                                                if streamNo == 1 as ::core::ffi::c_int {
                                                    return False;
                                                } else {
                                                    if noisy != 0 {
                                                        fprintf(
                                                                stderr,
                                                                b"\n%s: %s: trailing garbage after EOF ignored\n\0"
                                                                    as *const u8 as *const ::core::ffi::c_char,
                                                                progName,
                                                                &raw mut inName as *mut Char,
                                                            );
                                                    }
                                                    return True;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            _ => {
                                if !(ferror(zStream) != 0) {
                                    if stream != stdout {
                                        let mut fd: Int32 = fileno(stream) as Int32;
                                        if fd < 0 as ::core::ffi::c_int {
                                            current_block = 9121920734004483035;
                                        } else {
                                            applySavedFileAttrToOutputFile(fd as IntNative);
                                            current_block = 11459959175219260272;
                                        }
                                    } else {
                                        current_block = 11459959175219260272;
                                    }
                                    match current_block {
                                        9121920734004483035 => {}
                                        _ => {
                                            ret = fclose(zStream) as Int32;
                                            if !(ret == EOF) {
                                                if !(ferror(stream) != 0) {
                                                    ret = fflush(stream) as Int32;
                                                    if !(ret != 0 as ::core::ffi::c_int) {
                                                        if stream != stdout {
                                                            ret = fclose(stream) as Int32;
                                                            outputHandleJustInCase =
                                                                ::core::ptr::null_mut::<FILE>();
                                                            if ret == EOF {
                                                                current_block = 9121920734004483035;
                                                            } else {
                                                                current_block = 3123434771885419771;
                                                            }
                                                        } else {
                                                            current_block = 3123434771885419771;
                                                        }
                                                        match current_block {
                                                            9121920734004483035 => {}
                                                            _ => {
                                                                outputHandleJustInCase =
                                                                    ::core::ptr::null_mut::<FILE>();
                                                                if verbosity
                                                                    >= 2 as ::core::ffi::c_int
                                                                {
                                                                    fprintf(
                                                                            stderr,
                                                                            b"\n    \0" as *const u8 as *const ::core::ffi::c_char,
                                                                        );
                                                                }
                                                                return True;
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        },
                    }
                }
            }
        }
    }
    ioError();
}
unsafe extern "C" fn testStream(mut zStream: *mut FILE) -> Bool {
    let mut current_block: u64;
    let mut bzf: *mut ::core::ffi::c_void = NULL as *mut ::core::ffi::c_void;
    let mut bzerr: Int32 = 0;
    let mut bzerr_dummy: Int32 = 0;
    let mut ret: Int32 = 0;
    let mut streamNo: Int32 = 0;
    let mut i: Int32 = 0;
    let mut obuf: [UChar; 5000] = [0; 5000];
    let mut unused: [UChar; 5000] = [0; 5000];
    let mut nUnused: Int32 = 0;
    let mut unusedTmpV: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut unusedTmp: *mut UChar = ::core::ptr::null_mut::<UChar>();
    nUnused = 0 as ::core::ffi::c_int as Int32;
    streamNo = 0 as ::core::ffi::c_int as Int32;
    if !(ferror(zStream) != 0) {
        's_29: loop {
            if !(True != 0) {
                current_block = 5783071609795492627;
                break;
            }
            bzf = BZ2_bzReadOpen(
                &raw mut bzerr,
                zStream,
                verbosity as ::core::ffi::c_int,
                smallMode as ::core::ffi::c_int,
                &raw mut unused as *mut UChar as *mut ::core::ffi::c_void,
                nUnused as ::core::ffi::c_int,
            );
            if bzf.is_null() || bzerr != BZ_OK {
                current_block = 17245701858542999285;
                break;
            }
            streamNo += 1;
            while bzerr == BZ_OK {
                BZ2_bzRead(
                    &raw mut bzerr,
                    bzf,
                    &raw mut obuf as *mut UChar as *mut ::core::ffi::c_void,
                    5000 as ::core::ffi::c_int,
                );
                if bzerr == BZ_DATA_ERROR_MAGIC {
                    current_block = 17245701858542999285;
                    break 's_29;
                }
            }
            if bzerr != BZ_STREAM_END {
                current_block = 17245701858542999285;
                break;
            }
            BZ2_bzReadGetUnused(&raw mut bzerr, bzf, &raw mut unusedTmpV, &raw mut nUnused);
            if bzerr != BZ_OK {
                panic(b"test:bzReadGetUnused\0" as *const u8 as *const Char);
            }
            unusedTmp = unusedTmpV as *mut UChar;
            i = 0 as ::core::ffi::c_int as Int32;
            while i < nUnused {
                unused[i as usize] = *unusedTmp.offset(i as isize);
                i += 1;
            }
            BZ2_bzReadClose(&raw mut bzerr, bzf);
            if bzerr != BZ_OK {
                panic(b"test:bzReadGetUnused\0" as *const u8 as *const Char);
            }
            if nUnused == 0 as ::core::ffi::c_int && myfeof(zStream) as ::core::ffi::c_int != 0 {
                current_block = 5783071609795492627;
                break;
            }
        }
        match current_block {
            5783071609795492627 => {
                if !(ferror(zStream) != 0) {
                    ret = fclose(zStream) as Int32;
                    if !(ret == EOF) {
                        if verbosity >= 2 as ::core::ffi::c_int {
                            fprintf(
                                stderr,
                                b"\n    \0" as *const u8 as *const ::core::ffi::c_char,
                            );
                        }
                        return True;
                    }
                }
            }
            _ => {
                BZ2_bzReadClose(&raw mut bzerr_dummy, bzf);
                if verbosity == 0 as ::core::ffi::c_int {
                    fprintf(
                        stderr,
                        b"%s: %s: \0" as *const u8 as *const ::core::ffi::c_char,
                        progName,
                        &raw mut inName as *mut Char,
                    );
                }
                match bzerr {
                    BZ_CONFIG_ERROR => {
                        current_block = 8482013969978590419;
                        match current_block {
                            9496168864435797670 => {
                                panic(b"test:unexpected error\0" as *const u8 as *const Char);
                            }
                            7691887327196114895 => {
                                fprintf(
                                    stderr,
                                    b"file ends unexpectedly\n\0" as *const u8
                                        as *const ::core::ffi::c_char,
                                );
                                return False;
                            }
                            4091293050044138987 => {
                                if zStream != stdin {
                                    fclose(zStream);
                                }
                                if streamNo == 1 as ::core::ffi::c_int {
                                    fprintf(
                                        stderr,
                                        b"bad magic number (file not created by bzip2)\n\0"
                                            as *const u8
                                            as *const ::core::ffi::c_char,
                                    );
                                    return False;
                                } else {
                                    if noisy != 0 {
                                        fprintf(
                                            stderr,
                                            b"trailing garbage after EOF ignored\n\0" as *const u8
                                                as *const ::core::ffi::c_char,
                                        );
                                    }
                                    return True;
                                }
                            }
                            8482013969978590419 => {
                                configError();
                            }
                            498754368799237199 => {
                                outOfMemory();
                            }
                            _ => {
                                fprintf(
                                    stderr,
                                    b"data integrity (CRC) error in data\n\0" as *const u8
                                        as *const ::core::ffi::c_char,
                                );
                                return False;
                            }
                        }
                    }
                    BZ_IO_ERROR => {}
                    BZ_DATA_ERROR => {
                        current_block = 16852933939073280729;
                        match current_block {
                            9496168864435797670 => {
                                panic(b"test:unexpected error\0" as *const u8 as *const Char);
                            }
                            7691887327196114895 => {
                                fprintf(
                                    stderr,
                                    b"file ends unexpectedly\n\0" as *const u8
                                        as *const ::core::ffi::c_char,
                                );
                                return False;
                            }
                            4091293050044138987 => {
                                if zStream != stdin {
                                    fclose(zStream);
                                }
                                if streamNo == 1 as ::core::ffi::c_int {
                                    fprintf(
                                        stderr,
                                        b"bad magic number (file not created by bzip2)\n\0"
                                            as *const u8
                                            as *const ::core::ffi::c_char,
                                    );
                                    return False;
                                } else {
                                    if noisy != 0 {
                                        fprintf(
                                            stderr,
                                            b"trailing garbage after EOF ignored\n\0" as *const u8
                                                as *const ::core::ffi::c_char,
                                        );
                                    }
                                    return True;
                                }
                            }
                            8482013969978590419 => {
                                configError();
                            }
                            498754368799237199 => {
                                outOfMemory();
                            }
                            _ => {
                                fprintf(
                                    stderr,
                                    b"data integrity (CRC) error in data\n\0" as *const u8
                                        as *const ::core::ffi::c_char,
                                );
                                return False;
                            }
                        }
                    }
                    BZ_MEM_ERROR => {
                        current_block = 498754368799237199;
                        match current_block {
                            9496168864435797670 => {
                                panic(b"test:unexpected error\0" as *const u8 as *const Char);
                            }
                            7691887327196114895 => {
                                fprintf(
                                    stderr,
                                    b"file ends unexpectedly\n\0" as *const u8
                                        as *const ::core::ffi::c_char,
                                );
                                return False;
                            }
                            4091293050044138987 => {
                                if zStream != stdin {
                                    fclose(zStream);
                                }
                                if streamNo == 1 as ::core::ffi::c_int {
                                    fprintf(
                                        stderr,
                                        b"bad magic number (file not created by bzip2)\n\0"
                                            as *const u8
                                            as *const ::core::ffi::c_char,
                                    );
                                    return False;
                                } else {
                                    if noisy != 0 {
                                        fprintf(
                                            stderr,
                                            b"trailing garbage after EOF ignored\n\0" as *const u8
                                                as *const ::core::ffi::c_char,
                                        );
                                    }
                                    return True;
                                }
                            }
                            8482013969978590419 => {
                                configError();
                            }
                            498754368799237199 => {
                                outOfMemory();
                            }
                            _ => {
                                fprintf(
                                    stderr,
                                    b"data integrity (CRC) error in data\n\0" as *const u8
                                        as *const ::core::ffi::c_char,
                                );
                                return False;
                            }
                        }
                    }
                    BZ_UNEXPECTED_EOF => {
                        current_block = 7691887327196114895;
                        match current_block {
                            9496168864435797670 => {
                                panic(b"test:unexpected error\0" as *const u8 as *const Char);
                            }
                            7691887327196114895 => {
                                fprintf(
                                    stderr,
                                    b"file ends unexpectedly\n\0" as *const u8
                                        as *const ::core::ffi::c_char,
                                );
                                return False;
                            }
                            4091293050044138987 => {
                                if zStream != stdin {
                                    fclose(zStream);
                                }
                                if streamNo == 1 as ::core::ffi::c_int {
                                    fprintf(
                                        stderr,
                                        b"bad magic number (file not created by bzip2)\n\0"
                                            as *const u8
                                            as *const ::core::ffi::c_char,
                                    );
                                    return False;
                                } else {
                                    if noisy != 0 {
                                        fprintf(
                                            stderr,
                                            b"trailing garbage after EOF ignored\n\0" as *const u8
                                                as *const ::core::ffi::c_char,
                                        );
                                    }
                                    return True;
                                }
                            }
                            8482013969978590419 => {
                                configError();
                            }
                            498754368799237199 => {
                                outOfMemory();
                            }
                            _ => {
                                fprintf(
                                    stderr,
                                    b"data integrity (CRC) error in data\n\0" as *const u8
                                        as *const ::core::ffi::c_char,
                                );
                                return False;
                            }
                        }
                    }
                    BZ_DATA_ERROR_MAGIC => {
                        current_block = 4091293050044138987;
                        match current_block {
                            9496168864435797670 => {
                                panic(b"test:unexpected error\0" as *const u8 as *const Char);
                            }
                            7691887327196114895 => {
                                fprintf(
                                    stderr,
                                    b"file ends unexpectedly\n\0" as *const u8
                                        as *const ::core::ffi::c_char,
                                );
                                return False;
                            }
                            4091293050044138987 => {
                                if zStream != stdin {
                                    fclose(zStream);
                                }
                                if streamNo == 1 as ::core::ffi::c_int {
                                    fprintf(
                                        stderr,
                                        b"bad magic number (file not created by bzip2)\n\0"
                                            as *const u8
                                            as *const ::core::ffi::c_char,
                                    );
                                    return False;
                                } else {
                                    if noisy != 0 {
                                        fprintf(
                                            stderr,
                                            b"trailing garbage after EOF ignored\n\0" as *const u8
                                                as *const ::core::ffi::c_char,
                                        );
                                    }
                                    return True;
                                }
                            }
                            8482013969978590419 => {
                                configError();
                            }
                            498754368799237199 => {
                                outOfMemory();
                            }
                            _ => {
                                fprintf(
                                    stderr,
                                    b"data integrity (CRC) error in data\n\0" as *const u8
                                        as *const ::core::ffi::c_char,
                                );
                                return False;
                            }
                        }
                    }
                    _ => {
                        current_block = 9496168864435797670;
                        match current_block {
                            9496168864435797670 => {
                                panic(b"test:unexpected error\0" as *const u8 as *const Char);
                            }
                            7691887327196114895 => {
                                fprintf(
                                    stderr,
                                    b"file ends unexpectedly\n\0" as *const u8
                                        as *const ::core::ffi::c_char,
                                );
                                return False;
                            }
                            4091293050044138987 => {
                                if zStream != stdin {
                                    fclose(zStream);
                                }
                                if streamNo == 1 as ::core::ffi::c_int {
                                    fprintf(
                                        stderr,
                                        b"bad magic number (file not created by bzip2)\n\0"
                                            as *const u8
                                            as *const ::core::ffi::c_char,
                                    );
                                    return False;
                                } else {
                                    if noisy != 0 {
                                        fprintf(
                                            stderr,
                                            b"trailing garbage after EOF ignored\n\0" as *const u8
                                                as *const ::core::ffi::c_char,
                                        );
                                    }
                                    return True;
                                }
                            }
                            8482013969978590419 => {
                                configError();
                            }
                            498754368799237199 => {
                                outOfMemory();
                            }
                            _ => {
                                fprintf(
                                    stderr,
                                    b"data integrity (CRC) error in data\n\0" as *const u8
                                        as *const ::core::ffi::c_char,
                                );
                                return False;
                            }
                        }
                    }
                }
            }
        }
    }
    ioError();
}
unsafe extern "C" fn setExit(mut v: Int32) {
    if v > exitValue {
        exitValue = v;
    }
}
unsafe extern "C" fn cadvise() {
    if noisy != 0 {
        fprintf(
            stderr,
            b"\nIt is possible that the compressed file(s) have become corrupted.\nYou can use the -tvv option to test integrity of such files.\n\nYou can use the `bzip2recover' program to attempt to recover\ndata from undamaged sections of corrupted files.\n\n\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
    }
}
unsafe extern "C" fn showFileNames() {
    if noisy != 0 {
        fprintf(
            stderr,
            b"\tInput file = %s, output file = %s\n\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut inName as *mut Char,
            &raw mut outName as *mut Char,
        );
    }
}
unsafe extern "C" fn cleanUpAndFail(mut ec: Int32) -> ! {
    let mut retVal: IntNative = 0;
    let mut statBuf: stat = stat {
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
    if srcMode == SM_F2F && opMode != OM_TEST && deleteOutputOnInterrupt as ::core::ffi::c_int != 0
    {
        retVal = stat(&raw mut inName as *mut Char, &raw mut statBuf) as IntNative;
        if retVal == 0 as ::core::ffi::c_int {
            if noisy != 0 {
                fprintf(
                    stderr,
                    b"%s: Deleting output file %s, if it exists.\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    progName,
                    &raw mut outName as *mut Char,
                );
            }
            if !outputHandleJustInCase.is_null() {
                fclose(outputHandleJustInCase);
            }
            retVal = remove(&raw mut outName as *mut Char) as IntNative;
            if retVal != 0 as ::core::ffi::c_int {
                fprintf(
                    stderr,
                    b"%s: WARNING: deletion of output file (apparently) failed.\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    progName,
                );
            }
        } else {
            fprintf(
                stderr,
                b"%s: WARNING: deletion of output file suppressed\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                progName,
            );
            fprintf(
                stderr,
                b"%s:    since input file no longer exists.  Output file\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                progName,
            );
            fprintf(
                stderr,
                b"%s:    `%s' may be incomplete.\n\0" as *const u8 as *const ::core::ffi::c_char,
                progName,
                &raw mut outName as *mut Char,
            );
            fprintf(
                stderr,
                b"%s:    I suggest doing an integrity test (bzip2 -tv) of it.\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                progName,
            );
        }
    }
    if noisy as ::core::ffi::c_int != 0
        && numFileNames > 0 as ::core::ffi::c_int
        && numFilesProcessed < numFileNames
    {
        fprintf(
            stderr,
            b"%s: WARNING: some files have not been processed:\n%s:    %d specified on command line, %d not processed yet.\n\n\0"
                as *const u8 as *const ::core::ffi::c_char,
            progName,
            progName,
            numFileNames,
            numFileNames - numFilesProcessed,
        );
    }
    setExit(ec);
    exit(exitValue as ::core::ffi::c_int);
}
unsafe extern "C" fn panic(mut s: *const Char) -> ! {
    fprintf(
        stderr,
        b"\n%s: PANIC -- internal consistency error:\n\t%s\n\tThis is a BUG.  Please report it to:\n\tbzip2-devel@sourceware.org\n\0"
            as *const u8 as *const ::core::ffi::c_char,
        progName,
        s,
    );
    showFileNames();
    cleanUpAndFail(3 as Int32);
}
unsafe extern "C" fn crcError() -> ! {
    fprintf(
        stderr,
        b"\n%s: Data integrity error when decompressing.\n\0" as *const u8
            as *const ::core::ffi::c_char,
        progName,
    );
    showFileNames();
    cadvise();
    cleanUpAndFail(2 as Int32);
}
unsafe extern "C" fn compressedStreamEOF() -> ! {
    if noisy != 0 {
        fprintf(
            stderr,
            b"\n%s: Compressed file ends unexpectedly;\n\tperhaps it is corrupted?  *Possible* reason follows.\n\0"
                as *const u8 as *const ::core::ffi::c_char,
            progName,
        );
        perror(progName);
        showFileNames();
        cadvise();
    }
    cleanUpAndFail(2 as Int32);
}
unsafe extern "C" fn ioError() -> ! {
    fprintf(
        stderr,
        b"\n%s: I/O or other error, bailing out.  Possible reason follows.\n\0" as *const u8
            as *const ::core::ffi::c_char,
        progName,
    );
    perror(progName);
    showFileNames();
    cleanUpAndFail(1 as Int32);
}
unsafe extern "C" fn mySignalCatcher(mut n: IntNative) {
    fprintf(
        stderr,
        b"\n%s: Control-C or similar caught, quitting.\n\0" as *const u8
            as *const ::core::ffi::c_char,
        progName,
    );
    cleanUpAndFail(1 as Int32);
}
unsafe extern "C" fn mySIGSEGVorSIGBUScatcher(mut n: IntNative) {
    if opMode == OM_Z {
        fprintf(
            stderr,
            b"\n%s: Caught a SIGSEGV or SIGBUS whilst compressing.\n\n   Possible causes are (most likely first):\n   (1) This computer has unreliable memory or cache hardware\n       (a surprisingly common problem; try a different machine.)\n   (2) A bug in the compiler used to create this executable\n       (unlikely, if you didn't compile bzip2 yourself.)\n   (3) A real bug in bzip2 -- I hope this should never be the case.\n   The user's manual, Section 4.3, has more info on (1) and (2).\n   \n   If you suspect this is a bug in bzip2, or are unsure about (1)\n   or (2), feel free to report it to: bzip2-devel@sourceware.org.\n   Section 4.3 of the user's manual describes the info a useful\n   bug report should have.  If the manual is available on your\n   system, please try and read it before mailing me.  If you don't\n   have the manual or can't be bothered to read it, mail me anyway.\n\n\0"
                as *const u8 as *const ::core::ffi::c_char,
            progName,
        );
    } else {
        fprintf(
            stderr,
            b"\n%s: Caught a SIGSEGV or SIGBUS whilst decompressing.\n\n   Possible causes are (most likely first):\n   (1) The compressed data is corrupted, and bzip2's usual checks\n       failed to detect this.  Try bzip2 -tvv my_file.bz2.\n   (2) This computer has unreliable memory or cache hardware\n       (a surprisingly common problem; try a different machine.)\n   (3) A bug in the compiler used to create this executable\n       (unlikely, if you didn't compile bzip2 yourself.)\n   (4) A real bug in bzip2 -- I hope this should never be the case.\n   The user's manual, Section 4.3, has more info on (2) and (3).\n   \n   If you suspect this is a bug in bzip2, or are unsure about (2)\n   or (3), feel free to report it to: bzip2-devel@sourceware.org.\n   Section 4.3 of the user's manual describes the info a useful\n   bug report should have.  If the manual is available on your\n   system, please try and read it before mailing me.  If you don't\n   have the manual or can't be bothered to read it, mail me anyway.\n\n\0"
                as *const u8 as *const ::core::ffi::c_char,
            progName,
        );
    }
    showFileNames();
    if opMode == OM_Z {
        cleanUpAndFail(3 as Int32);
    } else {
        cadvise();
        cleanUpAndFail(2 as Int32);
    };
}
unsafe extern "C" fn outOfMemory() -> ! {
    fprintf(
        stderr,
        b"\n%s: couldn't allocate enough memory\n\0" as *const u8 as *const ::core::ffi::c_char,
        progName,
    );
    showFileNames();
    cleanUpAndFail(1 as Int32);
}
unsafe extern "C" fn configError() -> ! {
    fprintf(
        stderr,
        b"bzip2: I'm not configured correctly for this platform!\n\tI require Int32, Int16 and Char to have sizes\n\tof 4, 2 and 1 bytes to run properly, and they don't.\n\tProbably you can fix this by defining them correctly,\n\tand recompiling.  Bye!\n\0"
            as *const u8 as *const ::core::ffi::c_char,
    );
    setExit(3 as Int32);
    exit(exitValue as ::core::ffi::c_int);
}
unsafe extern "C" fn pad(mut s: *mut Char) {
    let mut i: Int32 = 0;
    if strlen(s) as Int32 >= longestFileName {
        return;
    }
    i = 1 as ::core::ffi::c_int as Int32;
    while i <= longestFileName - strlen(s) as Int32 {
        fprintf(stderr, b" \0" as *const u8 as *const ::core::ffi::c_char);
        i += 1;
    }
}
unsafe extern "C" fn copyFileName(mut to: *mut Char, mut from: *mut Char) {
    if strlen(from) > (FILE_NAME_LEN - 10 as ::core::ffi::c_int) as size_t {
        fprintf(
            stderr,
            b"bzip2: file name\n`%s'\nis suspiciously (more than %d chars) long.\nTry using a reasonable file name instead.  Sorry! :-)\n\0"
                as *const u8 as *const ::core::ffi::c_char,
            from,
            FILE_NAME_LEN - 10 as ::core::ffi::c_int,
        );
        setExit(1 as Int32);
        exit(exitValue as ::core::ffi::c_int);
    }
    strncpy(
        to as *mut ::core::ffi::c_char,
        from,
        (FILE_NAME_LEN - 10 as ::core::ffi::c_int) as size_t,
    );
    *to.offset((FILE_NAME_LEN - 10 as ::core::ffi::c_int) as isize) = '\0' as i32 as Char;
}
unsafe extern "C" fn fileExists(mut name: *mut Char) -> Bool {
    let mut tmp: *mut FILE = fopen(name, b"rb\0" as *const u8 as *const ::core::ffi::c_char);
    let mut exists: Bool = (tmp != NULL as *mut FILE) as ::core::ffi::c_int as Bool;
    if !tmp.is_null() {
        fclose(tmp);
    }
    return exists;
}
unsafe extern "C" fn fopen_output_safely(
    mut name: *mut Char,
    mut mode: *const ::core::ffi::c_char,
) -> *mut FILE {
    let mut fp: *mut FILE = ::core::ptr::null_mut::<FILE>();
    let mut fh: IntNative = 0;
    fh = open(name, O_WRONLY | O_CREAT | O_EXCL, S_IWUSR | S_IRUSR) as IntNative;
    if fh == -(1 as ::core::ffi::c_int) {
        return ::core::ptr::null_mut::<FILE>();
    }
    fp = fdopen(fh as ::core::ffi::c_int, mode);
    if fp.is_null() {
        close(fh as ::core::ffi::c_int);
    }
    return fp;
}
unsafe extern "C" fn notAStandardFile(mut name: *mut Char) -> Bool {
    let mut i: IntNative = 0;
    let mut statBuf: stat = stat {
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
    i = lstat(name, &raw mut statBuf) as IntNative;
    if i != 0 as ::core::ffi::c_int {
        return True;
    }
    if statBuf.st_mode as ::core::ffi::c_uint & __S_IFMT as ::core::ffi::c_uint
        == 0o100000 as ::core::ffi::c_uint
    {
        return False;
    }
    return True;
}
unsafe extern "C" fn countHardLinks(mut name: *mut Char) -> Int32 {
    let mut i: IntNative = 0;
    let mut statBuf: stat = stat {
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
    i = lstat(name, &raw mut statBuf) as IntNative;
    if i != 0 as ::core::ffi::c_int {
        return 0 as Int32;
    }
    return (statBuf.st_nlink as ::core::ffi::c_ulong).wrapping_sub(1 as ::core::ffi::c_ulong)
        as Int32;
}
static mut fileMetaInfo: stat = stat {
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
unsafe extern "C" fn saveInputFileMetaInfo(mut srcName: *mut Char) {
    let mut retVal: IntNative = 0;
    retVal = stat(srcName, &raw mut fileMetaInfo) as IntNative;
    if retVal != 0 as ::core::ffi::c_int {
        ioError();
    }
}
unsafe extern "C" fn applySavedTimeInfoToOutputFile(mut dstName: *mut Char) {
    let mut retVal: IntNative = 0;
    let mut uTimBuf: utimbuf = utimbuf {
        actime: 0,
        modtime: 0,
    };
    uTimBuf.actime = fileMetaInfo.st_atim.tv_sec;
    uTimBuf.modtime = fileMetaInfo.st_mtim.tv_sec;
    retVal = utime(dstName, &raw mut uTimBuf) as IntNative;
    if retVal != 0 as ::core::ffi::c_int {
        ioError();
    }
}
unsafe extern "C" fn applySavedFileAttrToOutputFile(mut fd: IntNative) {
    let mut retVal: IntNative = 0;
    retVal = fchmod(fd as ::core::ffi::c_int, fileMetaInfo.st_mode) as IntNative;
    if retVal != 0 as ::core::ffi::c_int {
        ioError();
    }
    fchown(
        fd as ::core::ffi::c_int,
        fileMetaInfo.st_uid,
        fileMetaInfo.st_gid,
    );
}
unsafe extern "C" fn containsDubiousChars(mut name: *mut Char) -> Bool {
    return False;
}
pub const BZ_N_SUFFIX_PAIRS: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
#[no_mangle]
pub static mut zSuffix: [*const Char; 4] = [
    b".bz2\0" as *const u8 as *const ::core::ffi::c_char,
    b".bz\0" as *const u8 as *const ::core::ffi::c_char,
    b".tbz2\0" as *const u8 as *const ::core::ffi::c_char,
    b".tbz\0" as *const u8 as *const ::core::ffi::c_char,
];
#[no_mangle]
pub static mut unzSuffix: [*const Char; 4] = [
    b"\0" as *const u8 as *const ::core::ffi::c_char,
    b"\0" as *const u8 as *const ::core::ffi::c_char,
    b".tar\0" as *const u8 as *const ::core::ffi::c_char,
    b".tar\0" as *const u8 as *const ::core::ffi::c_char,
];
unsafe extern "C" fn hasSuffix(mut s: *mut Char, mut suffix: *const Char) -> Bool {
    let mut ns: Int32 = strlen(s) as Int32;
    let mut nx: Int32 = strlen(suffix as *const ::core::ffi::c_char) as Int32;
    if ns < nx {
        return False;
    }
    if strcmp(
        s.offset(ns as isize).offset(-(nx as isize)),
        suffix as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        return True;
    }
    return False;
}
unsafe extern "C" fn mapSuffix(
    mut name: *mut Char,
    mut oldSuffix: *const Char,
    mut newSuffix: *const Char,
) -> Bool {
    if hasSuffix(name, oldSuffix) == 0 {
        return False;
    }
    *name.offset(
        strlen(name).wrapping_sub(strlen(oldSuffix as *const ::core::ffi::c_char)) as isize,
    ) = 0 as Char;
    strcat(
        name as *mut ::core::ffi::c_char,
        newSuffix as *const ::core::ffi::c_char,
    );
    return True;
}
unsafe extern "C" fn compress(mut name: *mut Char) {
    let mut inStr: *mut FILE = ::core::ptr::null_mut::<FILE>();
    let mut outStr: *mut FILE = ::core::ptr::null_mut::<FILE>();
    let mut n: Int32 = 0;
    let mut i: Int32 = 0;
    let mut statBuf: stat = stat {
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
    deleteOutputOnInterrupt = False;
    if name.is_null() && srcMode != SM_I2O {
        panic(b"compress: bad modes\n\0" as *const u8 as *const Char);
    }
    match srcMode {
        SM_I2O => {
            copyFileName(
                &raw mut inName as *mut Char,
                b"(stdin)\0" as *const u8 as *const ::core::ffi::c_char as *mut Char,
            );
            copyFileName(
                &raw mut outName as *mut Char,
                b"(stdout)\0" as *const u8 as *const ::core::ffi::c_char as *mut Char,
            );
        }
        SM_F2F => {
            copyFileName(&raw mut inName as *mut Char, name);
            copyFileName(&raw mut outName as *mut Char, name);
            strcat(
                &raw mut outName as *mut ::core::ffi::c_char,
                b".bz2\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        SM_F2O => {
            copyFileName(&raw mut inName as *mut Char, name);
            copyFileName(
                &raw mut outName as *mut Char,
                b"(stdout)\0" as *const u8 as *const ::core::ffi::c_char as *mut Char,
            );
        }
        _ => {}
    }
    if srcMode != SM_I2O
        && containsDubiousChars(&raw mut inName as *mut Char) as ::core::ffi::c_int != 0
    {
        if noisy != 0 {
            fprintf(
                stderr,
                b"%s: There are no files matching `%s'.\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                progName,
                &raw mut inName as *mut Char,
            );
        }
        setExit(1 as Int32);
        return;
    }
    if srcMode != SM_I2O && fileExists(&raw mut inName as *mut Char) == 0 {
        fprintf(
            stderr,
            b"%s: Can't open input file %s: %s.\n\0" as *const u8 as *const ::core::ffi::c_char,
            progName,
            &raw mut inName as *mut Char,
            strerror(*__errno_location()),
        );
        setExit(1 as Int32);
        return;
    }
    i = 0 as ::core::ffi::c_int as Int32;
    while i < BZ_N_SUFFIX_PAIRS {
        if hasSuffix(&raw mut inName as *mut Char, zSuffix[i as usize]) != 0 {
            if noisy != 0 {
                fprintf(
                    stderr,
                    b"%s: Input file %s already has %s suffix.\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    progName,
                    &raw mut inName as *mut Char,
                    zSuffix[i as usize],
                );
            }
            setExit(1 as Int32);
            return;
        }
        i += 1;
    }
    if srcMode == SM_F2F || srcMode == SM_F2O {
        stat(&raw mut inName as *mut Char, &raw mut statBuf);
        if statBuf.st_mode as ::core::ffi::c_uint & __S_IFMT as ::core::ffi::c_uint
            == 0o40000 as ::core::ffi::c_uint
        {
            fprintf(
                stderr,
                b"%s: Input file %s is a directory.\n\0" as *const u8 as *const ::core::ffi::c_char,
                progName,
                &raw mut inName as *mut Char,
            );
            setExit(1 as Int32);
            return;
        }
    }
    if srcMode == SM_F2F
        && forceOverwrite == 0
        && notAStandardFile(&raw mut inName as *mut Char) as ::core::ffi::c_int != 0
    {
        if noisy != 0 {
            fprintf(
                stderr,
                b"%s: Input file %s is not a normal file.\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                progName,
                &raw mut inName as *mut Char,
            );
        }
        setExit(1 as Int32);
        return;
    }
    if srcMode == SM_F2F && fileExists(&raw mut outName as *mut Char) as ::core::ffi::c_int != 0 {
        if forceOverwrite != 0 {
            remove(&raw mut outName as *mut Char);
        } else {
            fprintf(
                stderr,
                b"%s: Output file %s already exists.\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                progName,
                &raw mut outName as *mut Char,
            );
            setExit(1 as Int32);
            return;
        }
    }
    if srcMode == SM_F2F && forceOverwrite == 0 && {
        n = countHardLinks(&raw mut inName as *mut Char);
        n > 0 as ::core::ffi::c_int
    } {
        fprintf(
            stderr,
            b"%s: Input file %s has %d other link%s.\n\0" as *const u8
                as *const ::core::ffi::c_char,
            progName,
            &raw mut inName as *mut Char,
            n,
            if n > 1 as ::core::ffi::c_int {
                b"s\0" as *const u8 as *const ::core::ffi::c_char
            } else {
                b"\0" as *const u8 as *const ::core::ffi::c_char
            },
        );
        setExit(1 as Int32);
        return;
    }
    if srcMode == SM_F2F {
        saveInputFileMetaInfo(&raw mut inName as *mut Char);
    }
    match srcMode {
        SM_I2O => {
            inStr = stdin;
            outStr = stdout;
            if isatty(fileno(stdout)) != 0 {
                fprintf(
                    stderr,
                    b"%s: I won't write compressed data to a terminal.\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    progName,
                );
                fprintf(
                    stderr,
                    b"%s: For help, type: `%s --help'.\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    progName,
                    progName,
                );
                setExit(1 as Int32);
                return;
            }
        }
        SM_F2O => {
            inStr = fopen(
                &raw mut inName as *mut Char,
                b"rb\0" as *const u8 as *const ::core::ffi::c_char,
            );
            outStr = stdout;
            if isatty(fileno(stdout)) != 0 {
                fprintf(
                    stderr,
                    b"%s: I won't write compressed data to a terminal.\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    progName,
                );
                fprintf(
                    stderr,
                    b"%s: For help, type: `%s --help'.\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    progName,
                    progName,
                );
                if !inStr.is_null() {
                    fclose(inStr);
                }
                setExit(1 as Int32);
                return;
            }
            if inStr.is_null() {
                fprintf(
                    stderr,
                    b"%s: Can't open input file %s: %s.\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    progName,
                    &raw mut inName as *mut Char,
                    strerror(*__errno_location()),
                );
                setExit(1 as Int32);
                return;
            }
        }
        SM_F2F => {
            inStr = fopen(
                &raw mut inName as *mut Char,
                b"rb\0" as *const u8 as *const ::core::ffi::c_char,
            );
            outStr = fopen_output_safely(
                &raw mut outName as *mut Char,
                b"wb\0" as *const u8 as *const ::core::ffi::c_char,
            );
            if outStr.is_null() {
                fprintf(
                    stderr,
                    b"%s: Can't create output file %s: %s.\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    progName,
                    &raw mut outName as *mut Char,
                    strerror(*__errno_location()),
                );
                if !inStr.is_null() {
                    fclose(inStr);
                }
                setExit(1 as Int32);
                return;
            }
            if inStr.is_null() {
                fprintf(
                    stderr,
                    b"%s: Can't open input file %s: %s.\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    progName,
                    &raw mut inName as *mut Char,
                    strerror(*__errno_location()),
                );
                if !outStr.is_null() {
                    fclose(outStr);
                }
                setExit(1 as Int32);
                return;
            }
        }
        _ => {
            panic(b"compress: bad srcMode\0" as *const u8 as *const Char);
        }
    }
    if verbosity >= 1 as ::core::ffi::c_int {
        fprintf(
            stderr,
            b"  %s: \0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut inName as *mut Char,
        );
        pad(&raw mut inName as *mut Char);
        fflush(stderr);
    }
    outputHandleJustInCase = outStr;
    deleteOutputOnInterrupt = True;
    compressStream(inStr, outStr);
    outputHandleJustInCase = ::core::ptr::null_mut::<FILE>();
    if srcMode == SM_F2F {
        applySavedTimeInfoToOutputFile(&raw mut outName as *mut Char);
        deleteOutputOnInterrupt = False;
        if keepInputFiles == 0 {
            let mut retVal: IntNative = remove(&raw mut inName as *mut Char) as IntNative;
            if retVal != 0 as ::core::ffi::c_int {
                ioError();
            }
        }
    }
    deleteOutputOnInterrupt = False;
}
unsafe extern "C" fn uncompress(mut name: *mut Char) {
    let mut current_block: u64;
    let mut inStr: *mut FILE = ::core::ptr::null_mut::<FILE>();
    let mut outStr: *mut FILE = ::core::ptr::null_mut::<FILE>();
    let mut n: Int32 = 0;
    let mut i: Int32 = 0;
    let mut magicNumberOK: Bool = 0;
    let mut cantGuess: Bool = 0;
    let mut statBuf: stat = stat {
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
    deleteOutputOnInterrupt = False;
    if name.is_null() && srcMode != SM_I2O {
        panic(b"uncompress: bad modes\n\0" as *const u8 as *const Char);
    }
    cantGuess = False;
    match srcMode {
        SM_I2O => {
            copyFileName(
                &raw mut inName as *mut Char,
                b"(stdin)\0" as *const u8 as *const ::core::ffi::c_char as *mut Char,
            );
            copyFileName(
                &raw mut outName as *mut Char,
                b"(stdout)\0" as *const u8 as *const ::core::ffi::c_char as *mut Char,
            );
        }
        SM_F2F => {
            copyFileName(&raw mut inName as *mut Char, name);
            copyFileName(&raw mut outName as *mut Char, name);
            i = 0 as ::core::ffi::c_int as Int32;
            loop {
                if !(i < BZ_N_SUFFIX_PAIRS) {
                    current_block = 7651349459974463963;
                    break;
                }
                if mapSuffix(
                    &raw mut outName as *mut Char,
                    zSuffix[i as usize],
                    unzSuffix[i as usize],
                ) != 0
                {
                    current_block = 15347117668156188170;
                    break;
                }
                i += 1;
            }
            match current_block {
                15347117668156188170 => {}
                _ => {
                    cantGuess = True;
                    strcat(
                        &raw mut outName as *mut ::core::ffi::c_char,
                        b".out\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                }
            }
        }
        SM_F2O => {
            copyFileName(&raw mut inName as *mut Char, name);
            copyFileName(
                &raw mut outName as *mut Char,
                b"(stdout)\0" as *const u8 as *const ::core::ffi::c_char as *mut Char,
            );
        }
        _ => {}
    }
    if srcMode != SM_I2O
        && containsDubiousChars(&raw mut inName as *mut Char) as ::core::ffi::c_int != 0
    {
        if noisy != 0 {
            fprintf(
                stderr,
                b"%s: There are no files matching `%s'.\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                progName,
                &raw mut inName as *mut Char,
            );
        }
        setExit(1 as Int32);
        return;
    }
    if srcMode != SM_I2O && fileExists(&raw mut inName as *mut Char) == 0 {
        fprintf(
            stderr,
            b"%s: Can't open input file %s: %s.\n\0" as *const u8 as *const ::core::ffi::c_char,
            progName,
            &raw mut inName as *mut Char,
            strerror(*__errno_location()),
        );
        setExit(1 as Int32);
        return;
    }
    if srcMode == SM_F2F || srcMode == SM_F2O {
        stat(&raw mut inName as *mut Char, &raw mut statBuf);
        if statBuf.st_mode as ::core::ffi::c_uint & __S_IFMT as ::core::ffi::c_uint
            == 0o40000 as ::core::ffi::c_uint
        {
            fprintf(
                stderr,
                b"%s: Input file %s is a directory.\n\0" as *const u8 as *const ::core::ffi::c_char,
                progName,
                &raw mut inName as *mut Char,
            );
            setExit(1 as Int32);
            return;
        }
    }
    if srcMode == SM_F2F
        && forceOverwrite == 0
        && notAStandardFile(&raw mut inName as *mut Char) as ::core::ffi::c_int != 0
    {
        if noisy != 0 {
            fprintf(
                stderr,
                b"%s: Input file %s is not a normal file.\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                progName,
                &raw mut inName as *mut Char,
            );
        }
        setExit(1 as Int32);
        return;
    }
    if cantGuess != 0 {
        if noisy != 0 {
            fprintf(
                stderr,
                b"%s: Can't guess original name for %s -- using %s\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                progName,
                &raw mut inName as *mut Char,
                &raw mut outName as *mut Char,
            );
        }
    }
    if srcMode == SM_F2F && fileExists(&raw mut outName as *mut Char) as ::core::ffi::c_int != 0 {
        if forceOverwrite != 0 {
            remove(&raw mut outName as *mut Char);
        } else {
            fprintf(
                stderr,
                b"%s: Output file %s already exists.\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                progName,
                &raw mut outName as *mut Char,
            );
            setExit(1 as Int32);
            return;
        }
    }
    if srcMode == SM_F2F && forceOverwrite == 0 && {
        n = countHardLinks(&raw mut inName as *mut Char);
        n > 0 as ::core::ffi::c_int
    } {
        fprintf(
            stderr,
            b"%s: Input file %s has %d other link%s.\n\0" as *const u8
                as *const ::core::ffi::c_char,
            progName,
            &raw mut inName as *mut Char,
            n,
            if n > 1 as ::core::ffi::c_int {
                b"s\0" as *const u8 as *const ::core::ffi::c_char
            } else {
                b"\0" as *const u8 as *const ::core::ffi::c_char
            },
        );
        setExit(1 as Int32);
        return;
    }
    if srcMode == SM_F2F {
        saveInputFileMetaInfo(&raw mut inName as *mut Char);
    }
    match srcMode {
        SM_I2O => {
            inStr = stdin;
            outStr = stdout;
            if isatty(fileno(stdin)) != 0 {
                fprintf(
                    stderr,
                    b"%s: I won't read compressed data from a terminal.\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    progName,
                );
                fprintf(
                    stderr,
                    b"%s: For help, type: `%s --help'.\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    progName,
                    progName,
                );
                setExit(1 as Int32);
                return;
            }
        }
        SM_F2O => {
            inStr = fopen(
                &raw mut inName as *mut Char,
                b"rb\0" as *const u8 as *const ::core::ffi::c_char,
            );
            outStr = stdout;
            if inStr.is_null() {
                fprintf(
                    stderr,
                    b"%s: Can't open input file %s:%s.\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    progName,
                    &raw mut inName as *mut Char,
                    strerror(*__errno_location()),
                );
                if !inStr.is_null() {
                    fclose(inStr);
                }
                setExit(1 as Int32);
                return;
            }
        }
        SM_F2F => {
            inStr = fopen(
                &raw mut inName as *mut Char,
                b"rb\0" as *const u8 as *const ::core::ffi::c_char,
            );
            outStr = fopen_output_safely(
                &raw mut outName as *mut Char,
                b"wb\0" as *const u8 as *const ::core::ffi::c_char,
            );
            if outStr.is_null() {
                fprintf(
                    stderr,
                    b"%s: Can't create output file %s: %s.\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    progName,
                    &raw mut outName as *mut Char,
                    strerror(*__errno_location()),
                );
                if !inStr.is_null() {
                    fclose(inStr);
                }
                setExit(1 as Int32);
                return;
            }
            if inStr.is_null() {
                fprintf(
                    stderr,
                    b"%s: Can't open input file %s: %s.\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    progName,
                    &raw mut inName as *mut Char,
                    strerror(*__errno_location()),
                );
                if !outStr.is_null() {
                    fclose(outStr);
                }
                setExit(1 as Int32);
                return;
            }
        }
        _ => {
            panic(b"uncompress: bad srcMode\0" as *const u8 as *const Char);
        }
    }
    if verbosity >= 1 as ::core::ffi::c_int {
        fprintf(
            stderr,
            b"  %s: \0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut inName as *mut Char,
        );
        pad(&raw mut inName as *mut Char);
        fflush(stderr);
    }
    outputHandleJustInCase = outStr;
    deleteOutputOnInterrupt = True;
    magicNumberOK = uncompressStream(inStr, outStr);
    outputHandleJustInCase = ::core::ptr::null_mut::<FILE>();
    if magicNumberOK != 0 {
        if srcMode == SM_F2F {
            applySavedTimeInfoToOutputFile(&raw mut outName as *mut Char);
            deleteOutputOnInterrupt = False;
            if keepInputFiles == 0 {
                let mut retVal: IntNative = remove(&raw mut inName as *mut Char) as IntNative;
                if retVal != 0 as ::core::ffi::c_int {
                    ioError();
                }
            }
        }
    } else {
        unzFailsExist = True;
        deleteOutputOnInterrupt = False;
        if srcMode == SM_F2F {
            let mut retVal_0: IntNative = remove(&raw mut outName as *mut Char) as IntNative;
            if retVal_0 != 0 as ::core::ffi::c_int {
                ioError();
            }
        }
    }
    deleteOutputOnInterrupt = False;
    if magicNumberOK != 0 {
        if verbosity >= 1 as ::core::ffi::c_int {
            fprintf(
                stderr,
                b"done\n\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    } else {
        setExit(2 as Int32);
        if verbosity >= 1 as ::core::ffi::c_int {
            fprintf(
                stderr,
                b"not a bzip2 file.\n\0" as *const u8 as *const ::core::ffi::c_char,
            );
        } else {
            fprintf(
                stderr,
                b"%s: %s is not a bzip2 file.\n\0" as *const u8 as *const ::core::ffi::c_char,
                progName,
                &raw mut inName as *mut Char,
            );
        }
    };
}
unsafe extern "C" fn testf(mut name: *mut Char) {
    let mut inStr: *mut FILE = ::core::ptr::null_mut::<FILE>();
    let mut allOK: Bool = 0;
    let mut statBuf: stat = stat {
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
    deleteOutputOnInterrupt = False;
    if name.is_null() && srcMode != SM_I2O {
        panic(b"testf: bad modes\n\0" as *const u8 as *const Char);
    }
    copyFileName(
        &raw mut outName as *mut Char,
        b"(none)\0" as *const u8 as *const ::core::ffi::c_char as *mut Char,
    );
    match srcMode {
        SM_I2O => {
            copyFileName(
                &raw mut inName as *mut Char,
                b"(stdin)\0" as *const u8 as *const ::core::ffi::c_char as *mut Char,
            );
        }
        SM_F2F => {
            copyFileName(&raw mut inName as *mut Char, name);
        }
        SM_F2O => {
            copyFileName(&raw mut inName as *mut Char, name);
        }
        _ => {}
    }
    if srcMode != SM_I2O
        && containsDubiousChars(&raw mut inName as *mut Char) as ::core::ffi::c_int != 0
    {
        if noisy != 0 {
            fprintf(
                stderr,
                b"%s: There are no files matching `%s'.\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                progName,
                &raw mut inName as *mut Char,
            );
        }
        setExit(1 as Int32);
        return;
    }
    if srcMode != SM_I2O && fileExists(&raw mut inName as *mut Char) == 0 {
        fprintf(
            stderr,
            b"%s: Can't open input %s: %s.\n\0" as *const u8 as *const ::core::ffi::c_char,
            progName,
            &raw mut inName as *mut Char,
            strerror(*__errno_location()),
        );
        setExit(1 as Int32);
        return;
    }
    if srcMode != SM_I2O {
        stat(&raw mut inName as *mut Char, &raw mut statBuf);
        if statBuf.st_mode as ::core::ffi::c_uint & __S_IFMT as ::core::ffi::c_uint
            == 0o40000 as ::core::ffi::c_uint
        {
            fprintf(
                stderr,
                b"%s: Input file %s is a directory.\n\0" as *const u8 as *const ::core::ffi::c_char,
                progName,
                &raw mut inName as *mut Char,
            );
            setExit(1 as Int32);
            return;
        }
    }
    match srcMode {
        SM_I2O => {
            if isatty(fileno(stdin)) != 0 {
                fprintf(
                    stderr,
                    b"%s: I won't read compressed data from a terminal.\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    progName,
                );
                fprintf(
                    stderr,
                    b"%s: For help, type: `%s --help'.\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    progName,
                    progName,
                );
                setExit(1 as Int32);
                return;
            }
            inStr = stdin;
        }
        SM_F2O | SM_F2F => {
            inStr = fopen(
                &raw mut inName as *mut Char,
                b"rb\0" as *const u8 as *const ::core::ffi::c_char,
            );
            if inStr.is_null() {
                fprintf(
                    stderr,
                    b"%s: Can't open input file %s:%s.\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    progName,
                    &raw mut inName as *mut Char,
                    strerror(*__errno_location()),
                );
                setExit(1 as Int32);
                return;
            }
        }
        _ => {
            panic(b"testf: bad srcMode\0" as *const u8 as *const Char);
        }
    }
    if verbosity >= 1 as ::core::ffi::c_int {
        fprintf(
            stderr,
            b"  %s: \0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut inName as *mut Char,
        );
        pad(&raw mut inName as *mut Char);
        fflush(stderr);
    }
    outputHandleJustInCase = ::core::ptr::null_mut::<FILE>();
    allOK = testStream(inStr);
    if allOK as ::core::ffi::c_int != 0 && verbosity >= 1 as ::core::ffi::c_int {
        fprintf(stderr, b"ok\n\0" as *const u8 as *const ::core::ffi::c_char);
    }
    if allOK == 0 {
        testFailsExist = True;
    }
}
unsafe extern "C" fn license() {
    fprintf(
        stderr,
        b"bzip2, a block-sorting file compressor.  Version %s.\n   \n   Copyright (C) 1996-2019 by Julian Seward.\n   \n   This program is free software; you can redistribute it and/or modify\n   it under the terms set out in the LICENSE file, which is included\n   in the bzip2 source distribution.\n   \n   This program is distributed in the hope that it will be useful,\n   but WITHOUT ANY WARRANTY; without even the implied warranty of\n   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the\n   LICENSE file for more details.\n   \n\0"
            as *const u8 as *const ::core::ffi::c_char,
        BZ2_bzlibVersion(),
    );
}
unsafe extern "C" fn usage(mut fullProgName: *mut Char) {
    fprintf(
        stderr,
        b"bzip2, a block-sorting file compressor.  Version %s.\n\n   usage: %s [flags and input files in any order]\n\n   -h --help           print this message\n   -d --decompress     force decompression\n   -z --compress       force compression\n   -k --keep           keep (don't delete) input files\n   -f --force          overwrite existing output files\n   -t --test           test compressed file integrity\n   -c --stdout         output to standard out\n   -q --quiet          suppress noncritical error messages\n   -v --verbose        be verbose (a 2nd -v gives more)\n   -L --license        display software version & license\n   -V --version        display software version & license\n   -s --small          use less memory (at most 2500k)\n   -1 .. -9            set block size to 100k .. 900k\n   --fast              alias for -1\n   --best              alias for -9\n\n   If invoked as `bzip2', default action is to compress.\n              as `bunzip2',  default action is to decompress.\n              as `bzcat', default action is to decompress to stdout.\n\n   If no file names are given, bzip2 compresses or decompresses\n   from standard input to standard output.  You can combine\n   short flags, so `-v -4' means the same as -v4 or -4v, &c.\n\n\0"
            as *const u8 as *const ::core::ffi::c_char,
        BZ2_bzlibVersion(),
        fullProgName,
    );
}
unsafe extern "C" fn redundant(mut flag: *mut Char) {
    fprintf(
        stderr,
        b"%s: %s is redundant in versions 0.9.5 and above\n\0" as *const u8
            as *const ::core::ffi::c_char,
        progName,
        flag,
    );
}
unsafe extern "C" fn myMalloc(mut n: Int32) -> *mut ::core::ffi::c_void {
    let mut p: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    p = malloc(n as size_t);
    if p.is_null() {
        outOfMemory();
    }
    return p;
}
unsafe extern "C" fn mkCell() -> *mut Cell {
    let mut c: *mut Cell = ::core::ptr::null_mut::<Cell>();
    c = myMalloc(::core::mem::size_of::<Cell>() as Int32) as *mut Cell;
    (*c).name = ::core::ptr::null_mut::<Char>();
    (*c).link = ::core::ptr::null_mut::<zzzz>();
    return c;
}
unsafe extern "C" fn snocString(mut root: *mut Cell, mut name: *mut Char) -> *mut Cell {
    if root.is_null() {
        let mut tmp: *mut Cell = mkCell();
        (*tmp).name = myMalloc((5 as size_t).wrapping_add(strlen(name)) as Int32) as *mut Char;
        strcpy((*tmp).name as *mut ::core::ffi::c_char, name);
        return tmp;
    } else {
        let mut tmp_0: *mut Cell = root;
        while !(*tmp_0).link.is_null() {
            tmp_0 = (*tmp_0).link as *mut Cell;
        }
        (*tmp_0).link = snocString((*tmp_0).link as *mut Cell, name) as *mut zzzz;
        return root;
    };
}
unsafe extern "C" fn addFlagsFromEnvVar(mut argList: *mut *mut Cell, mut varName: *mut Char) {
    let mut i: Int32 = 0;
    let mut j: Int32 = 0;
    let mut k: Int32 = 0;
    let mut envbase: *mut Char = ::core::ptr::null_mut::<Char>();
    let mut p: *mut Char = ::core::ptr::null_mut::<Char>();
    envbase = getenv(varName) as *mut Char;
    if !envbase.is_null() {
        p = envbase;
        i = 0 as ::core::ffi::c_int as Int32;
        while True != 0 {
            if *p.offset(i as isize) as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                break;
            }
            p = p.offset(i as isize);
            i = 0 as ::core::ffi::c_int as Int32;
            while *(*__ctype_b_loc())
                .offset(*p.offset(0 as ::core::ffi::c_int as isize) as Int32 as isize)
                as ::core::ffi::c_int
                & _ISspace as ::core::ffi::c_int as ::core::ffi::c_ushort as ::core::ffi::c_int
                != 0
            {
                p = p.offset(1);
            }
            while *p.offset(i as isize) as ::core::ffi::c_int != 0 as ::core::ffi::c_int
                && *(*__ctype_b_loc()).offset(*p.offset(i as isize) as Int32 as isize)
                    as ::core::ffi::c_int
                    & _ISspace as ::core::ffi::c_int as ::core::ffi::c_ushort as ::core::ffi::c_int
                    == 0
            {
                i += 1;
            }
            if i > 0 as ::core::ffi::c_int {
                k = i;
                if k > FILE_NAME_LEN - 10 as ::core::ffi::c_int {
                    k = (FILE_NAME_LEN - 10 as ::core::ffi::c_int) as Int32;
                }
                j = 0 as ::core::ffi::c_int as Int32;
                while j < k {
                    tmpName[j as usize] = *p.offset(j as isize);
                    j += 1;
                }
                tmpName[k as usize] = 0 as Char;
                *argList = snocString(*argList, &raw mut tmpName as *mut Char);
            }
        }
    }
}
unsafe fn main_0(mut argc: IntNative, mut argv: *mut *mut Char) -> IntNative {
    let mut i: Int32 = 0;
    let mut j: Int32 = 0;
    let mut tmp: *mut Char = ::core::ptr::null_mut::<Char>();
    let mut argList: *mut Cell = ::core::ptr::null_mut::<Cell>();
    let mut aa: *mut Cell = ::core::ptr::null_mut::<Cell>();
    let mut decode: Bool = 0;
    if ::core::mem::size_of::<Int32>() as usize != 4 as usize
        || ::core::mem::size_of::<UInt32>() as usize != 4 as usize
        || ::core::mem::size_of::<Int16>() as usize != 2 as usize
        || ::core::mem::size_of::<UInt16>() as usize != 2 as usize
        || ::core::mem::size_of::<Char>() as usize != 1 as usize
        || ::core::mem::size_of::<UChar>() as usize != 1 as usize
    {
        configError();
    }
    outputHandleJustInCase = ::core::ptr::null_mut::<FILE>();
    smallMode = False;
    keepInputFiles = False;
    forceOverwrite = False;
    noisy = True;
    verbosity = 0 as ::core::ffi::c_int as Int32;
    blockSize100k = 9 as ::core::ffi::c_int as Int32;
    testFailsExist = False;
    unzFailsExist = False;
    numFileNames = 0 as ::core::ffi::c_int as Int32;
    numFilesProcessed = 0 as ::core::ffi::c_int as Int32;
    workFactor = 30 as ::core::ffi::c_int as Int32;
    deleteOutputOnInterrupt = False;
    exitValue = 0 as ::core::ffi::c_int as Int32;
    j = 0 as ::core::ffi::c_int as Int32;
    i = j;
    signal(
        SIGSEGV,
        Some(mySIGSEGVorSIGBUScatcher as unsafe extern "C" fn(IntNative) -> ()),
    );
    signal(
        SIGBUS,
        Some(mySIGSEGVorSIGBUScatcher as unsafe extern "C" fn(IntNative) -> ()),
    );
    copyFileName(
        &raw mut inName as *mut Char,
        b"(none)\0" as *const u8 as *const ::core::ffi::c_char as *mut Char,
    );
    copyFileName(
        &raw mut outName as *mut Char,
        b"(none)\0" as *const u8 as *const ::core::ffi::c_char as *mut Char,
    );
    copyFileName(
        &raw mut progNameReally as *mut Char,
        *argv.offset(0 as ::core::ffi::c_int as isize),
    );
    progName = (&raw mut progNameReally as *mut Char).offset(0 as ::core::ffi::c_int as isize)
        as *mut Char;
    tmp = (&raw mut progNameReally as *mut Char).offset(0 as ::core::ffi::c_int as isize)
        as *mut Char;
    while *tmp as ::core::ffi::c_int != '\0' as i32 {
        if *tmp as ::core::ffi::c_int == PATH_SEP {
            progName = tmp.offset(1 as ::core::ffi::c_int as isize);
        }
        tmp = tmp.offset(1);
    }
    argList = ::core::ptr::null_mut::<Cell>();
    addFlagsFromEnvVar(
        &raw mut argList,
        b"BZIP2\0" as *const u8 as *const ::core::ffi::c_char as *mut Char,
    );
    addFlagsFromEnvVar(
        &raw mut argList,
        b"BZIP\0" as *const u8 as *const ::core::ffi::c_char as *mut Char,
    );
    i = 1 as ::core::ffi::c_int as Int32;
    while i <= argc as ::core::ffi::c_int - 1 as ::core::ffi::c_int {
        argList = snocString(argList, *argv.offset(i as isize));
        i += 1;
    }
    longestFileName = 7 as ::core::ffi::c_int as Int32;
    numFileNames = 0 as ::core::ffi::c_int as Int32;
    decode = True;
    aa = argList;
    while !aa.is_null() {
        if strcmp(
            (*aa).name,
            b"--\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            decode = False;
        } else if !(*(*aa).name.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == '-' as i32
            && decode as ::core::ffi::c_int != 0)
        {
            numFileNames += 1;
            if longestFileName < strlen((*aa).name) as Int32 {
                longestFileName = strlen((*aa).name) as Int32;
            }
        }
        aa = (*aa).link as *mut Cell;
    }
    if numFileNames == 0 as ::core::ffi::c_int {
        srcMode = SM_I2O as Int32;
    } else {
        srcMode = SM_F2F as Int32;
    }
    opMode = OM_Z as Int32;
    if !strstr(
        progName,
        b"unzip\0" as *const u8 as *const ::core::ffi::c_char,
    )
    .is_null()
        || !strstr(
            progName,
            b"UNZIP\0" as *const u8 as *const ::core::ffi::c_char,
        )
        .is_null()
    {
        opMode = OM_UNZ as Int32;
    }
    if !strstr(
        progName,
        b"z2cat\0" as *const u8 as *const ::core::ffi::c_char,
    )
    .is_null()
        || !strstr(
            progName,
            b"Z2CAT\0" as *const u8 as *const ::core::ffi::c_char,
        )
        .is_null()
        || !strstr(
            progName,
            b"zcat\0" as *const u8 as *const ::core::ffi::c_char,
        )
        .is_null()
        || !strstr(
            progName,
            b"ZCAT\0" as *const u8 as *const ::core::ffi::c_char,
        )
        .is_null()
    {
        opMode = OM_UNZ as Int32;
        srcMode = (if numFileNames == 0 as ::core::ffi::c_int {
            SM_I2O
        } else {
            SM_F2O
        }) as Int32;
    }
    aa = argList;
    while !aa.is_null() {
        if strcmp(
            (*aa).name,
            b"--\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            break;
        }
        if *(*aa).name.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '-' as i32
            && *(*aa).name.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                != '-' as i32
        {
            j = 1 as ::core::ffi::c_int as Int32;
            while *(*aa).name.offset(j as isize) as ::core::ffi::c_int != '\0' as i32 {
                match *(*aa).name.offset(j as isize) as ::core::ffi::c_int {
                    99 => {
                        srcMode = SM_F2O as Int32;
                    }
                    100 => {
                        opMode = OM_UNZ as Int32;
                    }
                    122 => {
                        opMode = OM_Z as Int32;
                    }
                    102 => {
                        forceOverwrite = True;
                    }
                    116 => {
                        opMode = OM_TEST as Int32;
                    }
                    107 => {
                        keepInputFiles = True;
                    }
                    115 => {
                        smallMode = True;
                    }
                    113 => {
                        noisy = False;
                    }
                    49 => {
                        blockSize100k = 1 as ::core::ffi::c_int as Int32;
                    }
                    50 => {
                        blockSize100k = 2 as ::core::ffi::c_int as Int32;
                    }
                    51 => {
                        blockSize100k = 3 as ::core::ffi::c_int as Int32;
                    }
                    52 => {
                        blockSize100k = 4 as ::core::ffi::c_int as Int32;
                    }
                    53 => {
                        blockSize100k = 5 as ::core::ffi::c_int as Int32;
                    }
                    54 => {
                        blockSize100k = 6 as ::core::ffi::c_int as Int32;
                    }
                    55 => {
                        blockSize100k = 7 as ::core::ffi::c_int as Int32;
                    }
                    56 => {
                        blockSize100k = 8 as ::core::ffi::c_int as Int32;
                    }
                    57 => {
                        blockSize100k = 9 as ::core::ffi::c_int as Int32;
                    }
                    86 | 76 => {
                        license();
                    }
                    118 => {
                        verbosity += 1;
                    }
                    104 => {
                        usage(progName);
                        exit(0 as ::core::ffi::c_int);
                    }
                    _ => {
                        fprintf(
                            stderr,
                            b"%s: Bad flag `%s'\n\0" as *const u8 as *const ::core::ffi::c_char,
                            progName,
                            (*aa).name,
                        );
                        usage(progName);
                        exit(1 as ::core::ffi::c_int);
                    }
                }
                j += 1;
            }
        }
        aa = (*aa).link as *mut Cell;
    }
    aa = argList;
    while !aa.is_null() {
        if strcmp(
            (*aa).name,
            b"--\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            break;
        }
        if strcmp(
            (*aa).name,
            b"--stdout\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            srcMode = SM_F2O as Int32;
        } else if strcmp(
            (*aa).name,
            b"--decompress\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            opMode = OM_UNZ as Int32;
        } else if strcmp(
            (*aa).name,
            b"--compress\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            opMode = OM_Z as Int32;
        } else if strcmp(
            (*aa).name,
            b"--force\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            forceOverwrite = True;
        } else if strcmp(
            (*aa).name,
            b"--test\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            opMode = OM_TEST as Int32;
        } else if strcmp(
            (*aa).name,
            b"--keep\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            keepInputFiles = True;
        } else if strcmp(
            (*aa).name,
            b"--small\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            smallMode = True;
        } else if strcmp(
            (*aa).name,
            b"--quiet\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            noisy = False;
        } else if strcmp(
            (*aa).name,
            b"--version\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            license();
        } else if strcmp(
            (*aa).name,
            b"--license\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            license();
        } else if strcmp(
            (*aa).name,
            b"--exponential\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            workFactor = 1 as ::core::ffi::c_int as Int32;
        } else if strcmp(
            (*aa).name,
            b"--repetitive-best\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            redundant((*aa).name);
        } else if strcmp(
            (*aa).name,
            b"--repetitive-fast\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            redundant((*aa).name);
        } else if strcmp(
            (*aa).name,
            b"--fast\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            blockSize100k = 1 as ::core::ffi::c_int as Int32;
        } else if strcmp(
            (*aa).name,
            b"--best\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            blockSize100k = 9 as ::core::ffi::c_int as Int32;
        } else if strcmp(
            (*aa).name,
            b"--verbose\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            verbosity += 1;
        } else if strcmp(
            (*aa).name,
            b"--help\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            usage(progName);
            exit(0 as ::core::ffi::c_int);
        } else if strncmp(
            (*aa).name,
            b"--\0" as *const u8 as *const ::core::ffi::c_char,
            2 as size_t,
        ) == 0 as ::core::ffi::c_int
        {
            fprintf(
                stderr,
                b"%s: Bad flag `%s'\n\0" as *const u8 as *const ::core::ffi::c_char,
                progName,
                (*aa).name,
            );
            usage(progName);
            exit(1 as ::core::ffi::c_int);
        }
        aa = (*aa).link as *mut Cell;
    }
    if verbosity > 4 as ::core::ffi::c_int {
        verbosity = 4 as ::core::ffi::c_int as Int32;
    }
    if opMode == OM_Z
        && smallMode as ::core::ffi::c_int != 0
        && blockSize100k > 2 as ::core::ffi::c_int
    {
        blockSize100k = 2 as ::core::ffi::c_int as Int32;
    }
    if opMode == OM_TEST && srcMode == SM_F2O {
        fprintf(
            stderr,
            b"%s: -c and -t cannot be used together.\n\0" as *const u8
                as *const ::core::ffi::c_char,
            progName,
        );
        exit(1 as ::core::ffi::c_int);
    }
    if srcMode == SM_F2O && numFileNames == 0 as ::core::ffi::c_int {
        srcMode = SM_I2O as Int32;
    }
    if opMode != OM_Z {
        blockSize100k = 0 as ::core::ffi::c_int as Int32;
    }
    if srcMode == SM_F2F {
        signal(
            SIGINT,
            Some(mySignalCatcher as unsafe extern "C" fn(IntNative) -> ()),
        );
        signal(
            SIGTERM,
            Some(mySignalCatcher as unsafe extern "C" fn(IntNative) -> ()),
        );
        signal(
            SIGHUP,
            Some(mySignalCatcher as unsafe extern "C" fn(IntNative) -> ()),
        );
    }
    if opMode == OM_Z {
        if srcMode == SM_I2O {
            compress(::core::ptr::null_mut::<Char>());
        } else {
            decode = True;
            aa = argList;
            while !aa.is_null() {
                if strcmp(
                    (*aa).name,
                    b"--\0" as *const u8 as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
                {
                    decode = False;
                } else if !(*(*aa).name.offset(0 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    == '-' as i32
                    && decode as ::core::ffi::c_int != 0)
                {
                    numFilesProcessed += 1;
                    compress((*aa).name);
                }
                aa = (*aa).link as *mut Cell;
            }
        }
    } else if opMode == OM_UNZ {
        unzFailsExist = False;
        if srcMode == SM_I2O {
            uncompress(::core::ptr::null_mut::<Char>());
        } else {
            decode = True;
            aa = argList;
            while !aa.is_null() {
                if strcmp(
                    (*aa).name,
                    b"--\0" as *const u8 as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
                {
                    decode = False;
                } else if !(*(*aa).name.offset(0 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    == '-' as i32
                    && decode as ::core::ffi::c_int != 0)
                {
                    numFilesProcessed += 1;
                    uncompress((*aa).name);
                }
                aa = (*aa).link as *mut Cell;
            }
        }
        if unzFailsExist != 0 {
            setExit(2 as Int32);
            exit(exitValue as ::core::ffi::c_int);
        }
    } else {
        testFailsExist = False;
        if srcMode == SM_I2O {
            testf(::core::ptr::null_mut::<Char>());
        } else {
            decode = True;
            aa = argList;
            while !aa.is_null() {
                if strcmp(
                    (*aa).name,
                    b"--\0" as *const u8 as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
                {
                    decode = False;
                } else if !(*(*aa).name.offset(0 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    == '-' as i32
                    && decode as ::core::ffi::c_int != 0)
                {
                    numFilesProcessed += 1;
                    testf((*aa).name);
                }
                aa = (*aa).link as *mut Cell;
            }
        }
        if testFailsExist != 0 {
            if noisy != 0 {
                fprintf(
                    stderr,
                    b"\nYou can use the `bzip2recover' program to attempt to recover\ndata from undamaged sections of corrupted files.\n\n\0"
                        as *const u8 as *const ::core::ffi::c_char,
                );
            }
            setExit(2 as Int32);
            exit(exitValue as ::core::ffi::c_int);
        }
    }
    aa = argList;
    while !aa.is_null() {
        let mut aa2: *mut Cell = (*aa).link as *mut Cell;
        if !(*aa).name.is_null() {
            free((*aa).name as *mut ::core::ffi::c_void);
        }
        free(aa as *mut ::core::ffi::c_void);
        aa = aa2;
    }
    return exitValue as IntNative;
}
pub const SIGBUS: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const SIGINT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const SIGSEGV: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const SIGTERM: ::core::ffi::c_int = 15 as ::core::ffi::c_int;
pub const SIGHUP: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
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
            (args_ptrs.len() - 1) as IntNative,
            args_ptrs.as_mut_ptr() as *mut *mut Char,
        ) as i32)
    }
}
