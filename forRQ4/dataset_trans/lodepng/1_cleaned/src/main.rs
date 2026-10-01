#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(extern_types)]
#![feature(raw_ref_op)]
use core::ffi::*;
use ::lodepng_cleaned::src::ffi::*;
use ::lodepng_cleaned::src::c_consts::*;
use ::lodepng_cleaned::src::c_structs::*;
use ::lodepng_cleaned::src::c_types::*;
use ::lodepng_cleaned::src::c_extern_types::*;
#[allow(unused_imports)]
use ::lodepng_cleaned;
extern "C" {
    pub type __dirstream;
    fn opendir(__name: *const c_char) -> *mut DIR;
    fn closedir(__dirp: *mut DIR) -> c_int;
    fn readdir(__dirp: *mut DIR) -> *mut dirent;
    fn lodepng_decode32_file(
        out: *mut *mut c_uchar,
        w: *mut c_uint,
        h: *mut c_uint,
        filename: *const c_char,
    ) -> c_uint;
}

pub type __ino_t = c_ulong;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct dirent {
    pub d_ino: __ino_t,
    pub d_off: __off_t,
    pub d_reclen: c_ushort,
    pub d_type: c_uchar,
    pub d_name: [c_char; 256],
}
pub type DIR = __dirstream;

unsafe fn main_0(
    mut argc: c_int,
    mut argv: *mut *mut c_char,
) -> c_int {
    if argc != 2 as c_int {
        fprintf(
            stderr,
            b"usage: %s <png-dir>\n\0" as *const u8 as *const c_char,
            *argv.offset(0 as c_int as isize),
        );
        return 1 as c_int;
    }
    let mut d: *mut DIR = opendir(*argv.offset(1 as c_int as isize));
    if d.is_null() {
        perror(*argv.offset(1 as c_int as isize));
        return 1 as c_int;
    }
    let mut de: *mut dirent = ::core::ptr::null_mut::<dirent>();
    let mut total_pixels: c_ulong = 0 as c_ulong;
    let mut count: c_int = 0 as c_int;
    let mut path: [c_char; 2048] = [0; 2048];
    loop {
        de = readdir(d);
        if de.is_null() {
            break;
        }
        let mut name: *const c_char =
            &raw mut (*de).d_name as *mut c_char;
        let mut len: size_t = strlen(name);
        if len < 4 as size_t
            || strcmp(
                name.offset(len as isize)
                    .offset(-(4 as c_int as isize)),
                b".png\0" as *const u8 as *const c_char,
            ) != 0 as c_int
        {
            continue;
        }
        if *name.offset(0 as c_int as isize) as c_int == 'x' as i32 {
            continue;
        }
        snprintf(
            &raw mut path as *mut c_char,
            ::core::mem::size_of::<[c_char; 2048]>() as size_t,
            b"%s/%s\0" as *const u8 as *const c_char,
            *argv.offset(1 as c_int as isize),
            name,
        );
        let mut image: *mut c_uchar = ::core::ptr::null_mut::<c_uchar>();
        let mut w: c_uint = 0 as c_uint;
        let mut h: c_uint = 0 as c_uint;
        let mut err: c_uint = lodepng_decode32_file(
            &raw mut image,
            &raw mut w,
            &raw mut h,
            &raw mut path as *mut c_char,
        );
        if err != 0 {
            free(image as *mut c_void);
        } else {
            total_pixels = total_pixels
                .wrapping_add((w as c_ulong).wrapping_mul(h as c_ulong));
            count += 1;
            free(image as *mut c_void);
        }
    }
    closedir(d);
    printf(
        b"decoded=%d pixels=%lu\n\0" as *const u8 as *const c_char,
        count,
        total_pixels,
    );
    return 0 as c_int;
}
pub fn main() {
    let mut args_strings: Vec<Vec<u8>> = ::std::env::args()
        .map(|arg| {
            ::std::ffi::CString::new(arg)
                .expect("Failed to convert argument into CString.")
                .into_bytes_with_nul()
        })
        .collect();
    let mut args_ptrs: Vec<*mut c_char> = args_strings
        .iter_mut()
        .map(|arg| arg.as_mut_ptr() as *mut c_char)
        .chain(::core::iter::once(::core::ptr::null_mut()))
        .collect();
    unsafe {
        ::std::process::exit(main_0(
            (args_ptrs.len() - 1) as c_int,
            args_ptrs.as_mut_ptr() as *mut *mut c_char,
        ) as i32)
    }
}
