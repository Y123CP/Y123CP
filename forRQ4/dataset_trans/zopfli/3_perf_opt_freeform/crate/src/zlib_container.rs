use core::ffi::*;
use crate::src::deflate::ZopfliDeflate;
pub use crate::src::ffi::*;
pub use crate::src::c_structs::*;
pub use crate::src::c_types::*;
pub use crate::src::c_extern_types::*;

unsafe fn adler32(
    mut data: *const c_uchar,
    mut size: size_t,
) -> c_uint {
    static mut sums_overflow: c_uint = 5550 as c_uint;
    let mut s1: c_uint = 1 as c_uint;
    let mut s2: c_uint =
        (1 as c_int >> 16 as c_int) as c_uint;
    while size > 0 as size_t {
        let mut amount: size_t = if size > sums_overflow as size_t {
            sums_overflow as size_t
        } else {
            size
        };
        size = size.wrapping_sub(amount);
        while amount > 0 as size_t {
            let fresh0 = data;
            data = data.offset(1);
            s1 = s1.wrapping_add(*fresh0 as c_uint);
            s2 = s2.wrapping_add(s1);
            amount = amount.wrapping_sub(1);
        }
        s1 = s1.wrapping_rem(65521 as c_uint);
        s2 = s2.wrapping_rem(65521 as c_uint);
    }
    return s2 << 16 as c_int | s1;
}
#[inline]
pub unsafe fn ZopfliZlibCompress(
    mut options: *const ZopfliOptions,
    mut in_0: *const c_uchar,
    mut insize: size_t,
    mut out: *mut *mut c_uchar,
    mut outsize: *mut size_t,
) {
    let options_view: &ZopfliOptions = unsafe { &*options };
    let mut bitpointer: c_uchar = 0 as c_uchar;
    let mut checksum: c_uint = adler32(in_0, insize as c_uint as size_t);
    let mut cmf: c_uint = 120 as c_uint;
    let mut flevel: c_uint = 3 as c_uint;
    let mut fdict: c_uint = 0 as c_uint;
    let mut cmfflg: c_uint = (256 as c_uint)
        .wrapping_mul(cmf)
        .wrapping_add(fdict.wrapping_mul(32 as c_uint))
        .wrapping_add(flevel.wrapping_mul(64 as c_uint));
    let mut fcheck: c_uint =
        (31 as c_uint).wrapping_sub(cmfflg.wrapping_rem(31 as c_uint));
    cmfflg = cmfflg.wrapping_add(fcheck);
    if *outsize & (*outsize).wrapping_sub(1 as size_t) == 0 {
        *out = (if *outsize == 0 as size_t {
            malloc(::core::mem::size_of::<c_uchar>() as size_t)
        } else {
            realloc(
                *out as *mut c_void,
                (*outsize)
                    .wrapping_mul(2 as size_t)
                    .wrapping_mul(::core::mem::size_of::<c_uchar>() as size_t),
            )
        }) as *mut c_uchar;
    }
    *(*out).offset(*outsize as isize) =
        cmfflg.wrapping_div(256 as c_uint) as c_uchar;
    *outsize = (*outsize).wrapping_add(1);
    if *outsize & (*outsize).wrapping_sub(1 as size_t) == 0 {
        *out = (if *outsize == 0 as size_t {
            malloc(::core::mem::size_of::<c_uchar>() as size_t)
        } else {
            realloc(
                *out as *mut c_void,
                (*outsize)
                    .wrapping_mul(2 as size_t)
                    .wrapping_mul(::core::mem::size_of::<c_uchar>() as size_t),
            )
        }) as *mut c_uchar;
    }
    *(*out).offset(*outsize as isize) =
        cmfflg.wrapping_rem(256 as c_uint) as c_uchar;
    *outsize = (*outsize).wrapping_add(1);
    ZopfliDeflate(
        options,
        2 as c_int,
        1 as c_int,
        in_0,
        insize,
        &raw mut bitpointer,
        out,
        outsize,
    );
    if *outsize & (*outsize).wrapping_sub(1 as size_t) == 0 {
        *out = (if *outsize == 0 as size_t {
            malloc(::core::mem::size_of::<c_uchar>() as size_t)
        } else {
            realloc(
                *out as *mut c_void,
                (*outsize)
                    .wrapping_mul(2 as size_t)
                    .wrapping_mul(::core::mem::size_of::<c_uchar>() as size_t),
            )
        }) as *mut c_uchar;
    }
    *(*out).offset(*outsize as isize) = (checksum >> 24 as c_int)
        .wrapping_rem(256 as c_uint)
        as c_uchar;
    *outsize = (*outsize).wrapping_add(1);
    if *outsize & (*outsize).wrapping_sub(1 as size_t) == 0 {
        *out = (if *outsize == 0 as size_t {
            malloc(::core::mem::size_of::<c_uchar>() as size_t)
        } else {
            realloc(
                *out as *mut c_void,
                (*outsize)
                    .wrapping_mul(2 as size_t)
                    .wrapping_mul(::core::mem::size_of::<c_uchar>() as size_t),
            )
        }) as *mut c_uchar;
    }
    *(*out).offset(*outsize as isize) = (checksum >> 16 as c_int)
        .wrapping_rem(256 as c_uint)
        as c_uchar;
    *outsize = (*outsize).wrapping_add(1);
    if *outsize & (*outsize).wrapping_sub(1 as size_t) == 0 {
        *out = (if *outsize == 0 as size_t {
            malloc(::core::mem::size_of::<c_uchar>() as size_t)
        } else {
            realloc(
                *out as *mut c_void,
                (*outsize)
                    .wrapping_mul(2 as size_t)
                    .wrapping_mul(::core::mem::size_of::<c_uchar>() as size_t),
            )
        }) as *mut c_uchar;
    }
    *(*out).offset(*outsize as isize) = (checksum >> 8 as c_int)
        .wrapping_rem(256 as c_uint)
        as c_uchar;
    *outsize = (*outsize).wrapping_add(1);
    if *outsize & (*outsize).wrapping_sub(1 as size_t) == 0 {
        *out = (if *outsize == 0 as size_t {
            malloc(::core::mem::size_of::<c_uchar>() as size_t)
        } else {
            realloc(
                *out as *mut c_void,
                (*outsize)
                    .wrapping_mul(2 as size_t)
                    .wrapping_mul(::core::mem::size_of::<c_uchar>() as size_t),
            )
        }) as *mut c_uchar;
    }
    *(*out).offset(*outsize as isize) =
        checksum.wrapping_rem(256 as c_uint) as c_uchar;
    *outsize = (*outsize).wrapping_add(1);
    if options_view.verbose != 0 {
        fprintf(
            stderr,
            b"Original Size: %d, Zlib: %d, Compression: %f%% Removed\n\0" as *const u8
                as *const c_char,
            insize as c_int,
            *outsize as c_int,
            100.0f64 * insize.wrapping_sub(*outsize) as c_double
                / insize as c_double,
        );
    }
}
