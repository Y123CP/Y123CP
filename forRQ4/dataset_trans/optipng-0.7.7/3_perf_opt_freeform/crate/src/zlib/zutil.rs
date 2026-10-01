use core::ffi::*;
pub use crate::src::ffi::*;
pub use crate::src::c_types::*;

pub const ZLIB_VERSION: [c_char; 15] =
    unsafe { ::core::mem::transmute::<[u8; 15], [c_char; 15]>(*b"1.2.11-optipng\0") };
pub const Z_NEED_DICT: c_int = 2 as c_int;
#[no_mangle]
pub static mut z_errmsg: [*mut c_char; 10] = [
    b"need dictionary\0" as *const u8 as *const c_char as *mut c_char,
    b"stream end\0" as *const u8 as *const c_char as *mut c_char,
    b"\0" as *const u8 as *const c_char as *mut c_char,
    b"file error\0" as *const u8 as *const c_char as *mut c_char,
    b"stream error\0" as *const u8 as *const c_char as *mut c_char,
    b"data error\0" as *const u8 as *const c_char as *mut c_char,
    b"insufficient memory\0" as *const u8 as *const c_char as *mut c_char,
    b"buffer error\0" as *const u8 as *const c_char as *mut c_char,
    b"incompatible version\0" as *const u8 as *const c_char
        as *mut c_char,
    b"\0" as *const u8 as *const c_char as *mut c_char,
];
#[no_mangle]
pub extern "C" fn zlibVersion() -> *const c_char { {
    return ZLIB_VERSION.as_ptr();
} }
#[inline]
pub fn zlibCompileFlags() -> uLong { {
    let mut flags: uLong = 0;
    flags = 0 as uLong;
    match ::core::mem::size_of::<uInt>() as c_int {
        2 => {}
        4 => {
            flags = (flags as c_ulong).wrapping_add(1 as c_ulong) as uLong
                as uLong;
        }
        8 => {
            flags = (flags as c_ulong).wrapping_add(2 as c_ulong) as uLong
                as uLong;
        }
        _ => {
            flags = (flags as c_ulong).wrapping_add(3 as c_ulong) as uLong
                as uLong;
        }
    }
    match ::core::mem::size_of::<uLong>() as c_int {
        2 => {}
        4 => {
            flags = (flags as c_ulong).wrapping_add(
                ((1 as c_int) << 2 as c_int) as c_ulong,
            ) as uLong as uLong;
        }
        8 => {
            flags = (flags as c_ulong).wrapping_add(
                ((2 as c_int) << 2 as c_int) as c_ulong,
            ) as uLong as uLong;
        }
        _ => {
            flags = (flags as c_ulong).wrapping_add(
                ((3 as c_int) << 2 as c_int) as c_ulong,
            ) as uLong as uLong;
        }
    }
    match ::core::mem::size_of::<voidpf>() as c_int {
        2 => {}
        4 => {
            flags = (flags as c_ulong).wrapping_add(
                ((1 as c_int) << 4 as c_int) as c_ulong,
            ) as uLong as uLong;
        }
        8 => {
            flags = (flags as c_ulong).wrapping_add(
                ((2 as c_int) << 4 as c_int) as c_ulong,
            ) as uLong as uLong;
        }
        _ => {
            flags = (flags as c_ulong).wrapping_add(
                ((3 as c_int) << 4 as c_int) as c_ulong,
            ) as uLong as uLong;
        }
    }
    match ::core::mem::size_of::<off_t>() as c_int {
        2 => {}
        4 => {
            flags = (flags as c_ulong).wrapping_add(
                ((1 as c_int) << 6 as c_int) as c_ulong,
            ) as uLong as uLong;
        }
        8 => {
            flags = (flags as c_ulong).wrapping_add(
                ((2 as c_int) << 6 as c_int) as c_ulong,
            ) as uLong as uLong;
        }
        _ => {
            flags = (flags as c_ulong).wrapping_add(
                ((3 as c_int) << 6 as c_int) as c_ulong,
            ) as uLong as uLong;
        }
    }
    flags = (flags as c_ulong).wrapping_add(
        ((1 as c_long) << 16 as c_int) as c_ulong,
    ) as uLong as uLong;
    flags = (flags as c_ulong).wrapping_add(
        ((1 as c_long) << 17 as c_int) as c_ulong,
    ) as uLong as uLong;
    return flags;
} }
#[inline]
pub fn zError(mut err: c_int) -> *const c_char { unsafe {
    return z_errmsg[(Z_NEED_DICT - err) as usize];
} }
#[no_mangle]
pub extern "C" fn zcalloc(
    mut opaque: voidpf,
    mut items: c_uint,
    mut size: c_uint,
) -> voidpf { unsafe {
    return if ::core::mem::size_of::<uInt>() as usize > 2 as usize {
        malloc(items.wrapping_mul(size) as size_t)
    } else {
        calloc(items as size_t, size as size_t)
    };
} }
#[no_mangle]
pub extern "C" fn zcfree(mut opaque: voidpf, mut ptr: voidpf) { unsafe {
    free(ptr as *mut c_void);
} }
