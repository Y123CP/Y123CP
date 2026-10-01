extern "C" {
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn realloc(__ptr: *mut ::core::ffi::c_void, __size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
}
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BitStream {
    pub length: size_t,
    pub datasize: size_t,
    pub data: *mut ::core::ffi::c_uchar,
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const DEFAULT_BUFSIZE: ::core::ffi::c_int = 128 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn BitStream_new() -> *mut BitStream {
    let mut bstream: *mut BitStream = ::core::ptr::null_mut::<BitStream>();
    bstream = malloc(::core::mem::size_of::<BitStream>() as size_t) as *mut BitStream;
    if bstream.is_null() {
        return ::core::ptr::null_mut::<BitStream>();
    }
    (*bstream).length = 0 as size_t;
    (*bstream).data = malloc(DEFAULT_BUFSIZE as size_t) as *mut ::core::ffi::c_uchar;
    if (*bstream).data.is_null() {
        free(bstream as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<BitStream>();
    }
    (*bstream).datasize = DEFAULT_BUFSIZE as size_t;
    return bstream;
}
unsafe extern "C" fn BitStream_expand(mut bstream: *mut BitStream) -> ::core::ffi::c_int {
    let mut data: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    data = realloc(
        (*bstream).data as *mut ::core::ffi::c_void,
        (*bstream).datasize.wrapping_mul(2 as size_t),
    ) as *mut ::core::ffi::c_uchar;
    if data.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    (*bstream).data = data;
    (*bstream).datasize = (*bstream).datasize.wrapping_mul(2 as size_t);
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn BitStream_writeNum(
    mut dest: *mut ::core::ffi::c_uchar,
    mut bits: size_t,
    mut num: ::core::ffi::c_uint,
) {
    let mut mask: ::core::ffi::c_uint = 0;
    let mut i: size_t = 0;
    let mut p: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    p = dest;
    mask = (1 as ::core::ffi::c_uint) << bits.wrapping_sub(1 as size_t);
    i = 0 as size_t;
    while i < bits {
        if num & mask != 0 {
            *p = 1 as ::core::ffi::c_uchar;
        } else {
            *p = 0 as ::core::ffi::c_uchar;
        }
        p = p.offset(1);
        mask = mask >> 1 as ::core::ffi::c_int;
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn BitStream_writeBytes(
    mut dest: *mut ::core::ffi::c_uchar,
    mut size: size_t,
    mut data: *mut ::core::ffi::c_uchar,
) {
    let mut mask: ::core::ffi::c_uchar = 0;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut p: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    p = dest;
    i = 0 as size_t;
    while i < size {
        mask = 0x80 as ::core::ffi::c_uchar;
        j = 0 as size_t;
        while j < 8 as size_t {
            if *data.offset(i as isize) as ::core::ffi::c_int & mask as ::core::ffi::c_int != 0 {
                *p = 1 as ::core::ffi::c_uchar;
            } else {
                *p = 0 as ::core::ffi::c_uchar;
            }
            p = p.offset(1);
            mask = (mask as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
            j = j.wrapping_add(1);
        }
        i = i.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn BitStream_append(
    mut bstream: *mut BitStream,
    mut arg: *mut BitStream,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;
    if arg.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    if (*arg).length == 0 as size_t {
        return 0 as ::core::ffi::c_int;
    }
    while (*bstream).length.wrapping_add((*arg).length) > (*bstream).datasize {
        ret = BitStream_expand(bstream);
        if ret < 0 as ::core::ffi::c_int {
            return ret;
        }
    }
    memcpy(
        (*bstream).data.offset((*bstream).length as isize) as *mut ::core::ffi::c_void,
        (*arg).data as *const ::core::ffi::c_void,
        (*arg).length,
    );
    (*bstream).length = (*bstream).length.wrapping_add((*arg).length);
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn BitStream_appendNum(
    mut bstream: *mut BitStream,
    mut bits: size_t,
    mut num: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;
    if bits == 0 as size_t {
        return 0 as ::core::ffi::c_int;
    }
    while (*bstream).datasize.wrapping_sub((*bstream).length) < bits {
        ret = BitStream_expand(bstream);
        if ret < 0 as ::core::ffi::c_int {
            return ret;
        }
    }
    BitStream_writeNum(
        (*bstream).data.offset((*bstream).length as isize),
        bits,
        num,
    );
    (*bstream).length = (*bstream).length.wrapping_add(bits);
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn BitStream_appendBytes(
    mut bstream: *mut BitStream,
    mut size: size_t,
    mut data: *mut ::core::ffi::c_uchar,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;
    if size == 0 as size_t {
        return 0 as ::core::ffi::c_int;
    }
    while (*bstream).datasize.wrapping_sub((*bstream).length) < size.wrapping_mul(8 as size_t) {
        ret = BitStream_expand(bstream);
        if ret < 0 as ::core::ffi::c_int {
            return ret;
        }
    }
    BitStream_writeBytes(
        (*bstream).data.offset((*bstream).length as isize),
        size,
        data,
    );
    (*bstream).length = (*bstream)
        .length
        .wrapping_add(size.wrapping_mul(8 as size_t));
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn BitStream_toByte(
    mut bstream: *mut BitStream,
) -> *mut ::core::ffi::c_uchar {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut size: size_t = 0;
    let mut bytes: size_t = 0;
    let mut oddbits: size_t = 0;
    let mut data: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut v: ::core::ffi::c_uchar = 0;
    let mut p: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    size = (*bstream).length;
    if size == 0 as size_t {
        return ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    }
    data = malloc(size.wrapping_add(7 as size_t).wrapping_div(8 as size_t))
        as *mut ::core::ffi::c_uchar;
    if data.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    }
    bytes = size.wrapping_div(8 as size_t);
    p = (*bstream).data;
    i = 0 as size_t;
    while i < bytes {
        v = 0 as ::core::ffi::c_uchar;
        j = 0 as size_t;
        while j < 8 as size_t {
            v = ((v as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
            v = (v as ::core::ffi::c_int | *p as ::core::ffi::c_int) as ::core::ffi::c_uchar;
            p = p.offset(1);
            j = j.wrapping_add(1);
        }
        *data.offset(i as isize) = v;
        i = i.wrapping_add(1);
    }
    oddbits = size & 7 as size_t;
    if oddbits > 0 as size_t {
        v = 0 as ::core::ffi::c_uchar;
        j = 0 as size_t;
        while j < oddbits {
            v = ((v as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
            v = (v as ::core::ffi::c_int | *p as ::core::ffi::c_int) as ::core::ffi::c_uchar;
            p = p.offset(1);
            j = j.wrapping_add(1);
        }
        *data.offset(bytes as isize) = ((v as ::core::ffi::c_int)
            << (8 as size_t).wrapping_sub(oddbits))
            as ::core::ffi::c_uchar;
    }
    return data;
}
#[no_mangle]
pub unsafe extern "C" fn BitStream_free(mut bstream: *mut BitStream) {
    if !bstream.is_null() {
        free((*bstream).data as *mut ::core::ffi::c_void);
        free(bstream as *mut ::core::ffi::c_void);
    }
}
