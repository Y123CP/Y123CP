extern "C" {
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
}
pub type size_t = usize;
pub type uInt = ::core::ffi::c_uint;
pub type uLong = ::core::ffi::c_ulong;
pub type voidpf = *mut ::core::ffi::c_void;
pub type __off_t = ::core::ffi::c_long;
pub type off_t = __off_t;
pub const ZLIB_VERSION: [::core::ffi::c_char; 15] =
    unsafe { ::core::mem::transmute::<[u8; 15], [::core::ffi::c_char; 15]>(*b"1.2.11-optipng\0") };
pub const Z_NEED_DICT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
#[no_mangle]
pub static mut z_errmsg: [*mut ::core::ffi::c_char; 10] = [
    b"need dictionary\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"stream end\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"file error\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"stream error\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"data error\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"insufficient memory\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"buffer error\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"incompatible version\0" as *const u8 as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
];
#[no_mangle]
pub unsafe extern "C" fn zlibVersion() -> *const ::core::ffi::c_char {
    return ZLIB_VERSION.as_ptr();
}
#[no_mangle]
pub unsafe extern "C" fn zlibCompileFlags() -> uLong {
    let mut flags: uLong = 0;
    flags = 0 as uLong;
    match ::core::mem::size_of::<uInt>() as ::core::ffi::c_int {
        2 => {}
        4 => {
            flags = (flags as ::core::ffi::c_ulong).wrapping_add(1 as ::core::ffi::c_ulong) as uLong
                as uLong;
        }
        8 => {
            flags = (flags as ::core::ffi::c_ulong).wrapping_add(2 as ::core::ffi::c_ulong) as uLong
                as uLong;
        }
        _ => {
            flags = (flags as ::core::ffi::c_ulong).wrapping_add(3 as ::core::ffi::c_ulong) as uLong
                as uLong;
        }
    }
    match ::core::mem::size_of::<uLong>() as ::core::ffi::c_int {
        2 => {}
        4 => {
            flags = (flags as ::core::ffi::c_ulong).wrapping_add(
                ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as ::core::ffi::c_ulong,
            ) as uLong as uLong;
        }
        8 => {
            flags = (flags as ::core::ffi::c_ulong).wrapping_add(
                ((2 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as ::core::ffi::c_ulong,
            ) as uLong as uLong;
        }
        _ => {
            flags = (flags as ::core::ffi::c_ulong).wrapping_add(
                ((3 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as ::core::ffi::c_ulong,
            ) as uLong as uLong;
        }
    }
    match ::core::mem::size_of::<voidpf>() as ::core::ffi::c_int {
        2 => {}
        4 => {
            flags = (flags as ::core::ffi::c_ulong).wrapping_add(
                ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as ::core::ffi::c_ulong,
            ) as uLong as uLong;
        }
        8 => {
            flags = (flags as ::core::ffi::c_ulong).wrapping_add(
                ((2 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as ::core::ffi::c_ulong,
            ) as uLong as uLong;
        }
        _ => {
            flags = (flags as ::core::ffi::c_ulong).wrapping_add(
                ((3 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as ::core::ffi::c_ulong,
            ) as uLong as uLong;
        }
    }
    match ::core::mem::size_of::<off_t>() as ::core::ffi::c_int {
        2 => {}
        4 => {
            flags = (flags as ::core::ffi::c_ulong).wrapping_add(
                ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int) as ::core::ffi::c_ulong,
            ) as uLong as uLong;
        }
        8 => {
            flags = (flags as ::core::ffi::c_ulong).wrapping_add(
                ((2 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int) as ::core::ffi::c_ulong,
            ) as uLong as uLong;
        }
        _ => {
            flags = (flags as ::core::ffi::c_ulong).wrapping_add(
                ((3 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int) as ::core::ffi::c_ulong,
            ) as uLong as uLong;
        }
    }
    flags = (flags as ::core::ffi::c_ulong).wrapping_add(
        ((1 as ::core::ffi::c_long) << 16 as ::core::ffi::c_int) as ::core::ffi::c_ulong,
    ) as uLong as uLong;
    flags = (flags as ::core::ffi::c_ulong).wrapping_add(
        ((1 as ::core::ffi::c_long) << 17 as ::core::ffi::c_int) as ::core::ffi::c_ulong,
    ) as uLong as uLong;
    return flags;
}
#[no_mangle]
pub unsafe extern "C" fn zError(mut err: ::core::ffi::c_int) -> *const ::core::ffi::c_char {
    return z_errmsg[(Z_NEED_DICT - err) as usize];
}
#[no_mangle]
pub unsafe extern "C" fn zcalloc(
    mut opaque: voidpf,
    mut items: ::core::ffi::c_uint,
    mut size: ::core::ffi::c_uint,
) -> voidpf {
    return if ::core::mem::size_of::<uInt>() as usize > 2 as usize {
        malloc(items.wrapping_mul(size) as size_t)
    } else {
        calloc(items as size_t, size as size_t)
    };
}
#[no_mangle]
pub unsafe extern "C" fn zcfree(mut opaque: voidpf, mut ptr: voidpf) {
    free(ptr as *mut ::core::ffi::c_void);
}
