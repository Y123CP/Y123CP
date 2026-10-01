#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(extern_types)]
#![feature(raw_ref_op)]
#[allow(unused_imports)]
use ::lodepng_bench_raw;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type __dirstream;
    static mut stderr: *mut FILE;
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
    fn perror(__s: *const ::core::ffi::c_char);
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn opendir(__name: *const ::core::ffi::c_char) -> *mut DIR;
    fn closedir(__dirp: *mut DIR) -> ::core::ffi::c_int;
    fn readdir(__dirp: *mut DIR) -> *mut dirent;
    fn lodepng_decode32_file(
        out: *mut *mut ::core::ffi::c_uchar,
        w: *mut ::core::ffi::c_uint,
        h: *mut ::core::ffi::c_uint,
        filename: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_uint;
}
pub type size_t = usize;
pub type __ino_t = ::core::ffi::c_ulong;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dirent {
    pub d_ino: __ino_t,
    pub d_off: __off_t,
    pub d_reclen: ::core::ffi::c_ushort,
    pub d_type: ::core::ffi::c_uchar,
    pub d_name: [::core::ffi::c_char; 256],
}
pub type DIR = __dirstream;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
unsafe fn main_0(
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if argc != 2 as ::core::ffi::c_int {
        fprintf(
            stderr,
            b"usage: %s <png-dir>\n\0" as *const u8 as *const ::core::ffi::c_char,
            *argv.offset(0 as ::core::ffi::c_int as isize),
        );
        return 1 as ::core::ffi::c_int;
    }
    let mut d: *mut DIR = opendir(*argv.offset(1 as ::core::ffi::c_int as isize));
    if d.is_null() {
        perror(*argv.offset(1 as ::core::ffi::c_int as isize));
        return 1 as ::core::ffi::c_int;
    }
    let mut de: *mut dirent = ::core::ptr::null_mut::<dirent>();
    let mut total_pixels: ::core::ffi::c_ulong = 0 as ::core::ffi::c_ulong;
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut path: [::core::ffi::c_char; 2048] = [0; 2048];
    loop {
        de = readdir(d);
        if de.is_null() {
            break;
        }
        let mut name: *const ::core::ffi::c_char =
            &raw mut (*de).d_name as *mut ::core::ffi::c_char;
        let mut len: size_t = strlen(name);
        if len < 4 as size_t
            || strcmp(
                name.offset(len as isize)
                    .offset(-(4 as ::core::ffi::c_int as isize)),
                b".png\0" as *const u8 as *const ::core::ffi::c_char,
            ) != 0 as ::core::ffi::c_int
        {
            continue;
        }
        if *name.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == 'x' as i32 {
            continue;
        }
        snprintf(
            &raw mut path as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 2048]>() as size_t,
            b"%s/%s\0" as *const u8 as *const ::core::ffi::c_char,
            *argv.offset(1 as ::core::ffi::c_int as isize),
            name,
        );
        let mut image: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
        let mut w: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
        let mut h: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
        let mut err: ::core::ffi::c_uint = lodepng_decode32_file(
            &raw mut image,
            &raw mut w,
            &raw mut h,
            &raw mut path as *mut ::core::ffi::c_char,
        );
        if err != 0 {
            free(image as *mut ::core::ffi::c_void);
        } else {
            total_pixels = total_pixels
                .wrapping_add((w as ::core::ffi::c_ulong).wrapping_mul(h as ::core::ffi::c_ulong));
            count += 1;
            free(image as *mut ::core::ffi::c_void);
        }
    }
    closedir(d);
    printf(
        b"decoded=%d pixels=%lu\n\0" as *const u8 as *const ::core::ffi::c_char,
        count,
        total_pixels,
    );
    return 0 as ::core::ffi::c_int;
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
