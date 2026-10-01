use core::ffi::*;
pub use crate::src::ffi::*;
pub use crate::src::c_structs::*;
pub use crate::src::c_types::*;
pub use crate::src::c_extern_types::*;

pub const NULL: *mut c_void = ::core::ptr::null_mut::<c_void>();
pub const EXIT_FAILURE: c_int = 1 as c_int;
#[inline]
pub unsafe fn ZopfliInitCache(
    mut blocksize: size_t,
    mut lmc: *mut ZopfliLongestMatchCache,
) {
    let lmc_view: &mut ZopfliLongestMatchCache = unsafe { &mut *lmc };
    let mut i: size_t = 0;
    lmc_view.length =
        malloc((::core::mem::size_of::<c_ushort>() as size_t).wrapping_mul(blocksize))
            as *mut c_ushort;
    lmc_view.dist =
        malloc((::core::mem::size_of::<c_ushort>() as size_t).wrapping_mul(blocksize))
            as *mut c_ushort;
    lmc_view.sublen =
        malloc(((ZOPFLI_CACHE_LENGTH * 3 as c_int) as size_t).wrapping_mul(blocksize))
            as *mut c_uchar;
    if lmc_view.sublen.is_null() {
        fprintf(
            stderr,
            b"Error: Out of memory. Tried allocating %lu bytes of memory.\n\0" as *const u8
                as *const c_char,
            (ZOPFLI_CACHE_LENGTH as size_t)
                .wrapping_mul(3 as size_t)
                .wrapping_mul(blocksize),
        );
        exit(EXIT_FAILURE);
    }
    i = 0 as size_t;
    while i < blocksize {
        *lmc_view.length.offset(i as isize) = 1 as c_ushort;
        i = i.wrapping_add(1);
    }
    i = 0 as size_t;
    while i < blocksize {
        *lmc_view.dist.offset(i as isize) = 0 as c_ushort;
        i = i.wrapping_add(1);
    }
    i = 0 as size_t;
    while i
        < (ZOPFLI_CACHE_LENGTH as size_t)
            .wrapping_mul(blocksize)
            .wrapping_mul(3 as size_t)
    {
        *lmc_view.sublen.offset(i as isize) = 0 as c_uchar;
        i = i.wrapping_add(1);
    }
}
#[inline]
pub unsafe fn ZopfliCleanCache(mut lmc: *mut ZopfliLongestMatchCache) {
    let lmc_view: &ZopfliLongestMatchCache = unsafe { &*lmc };
    free(lmc_view.length as *mut c_void);
    free(lmc_view.dist as *mut c_void);
    free(lmc_view.sublen as *mut c_void);
}
#[inline]
pub unsafe fn ZopfliSublenToCache(
    mut sublen: *const c_ushort,
    mut pos: size_t,
    mut length: size_t,
    mut lmc: *mut ZopfliLongestMatchCache,
) {
    let lmc_view: &ZopfliLongestMatchCache = unsafe { &*lmc };
    let mut i: size_t = 0;
    let mut j: size_t = 0 as size_t;
    let mut bestlength: c_uint = 0 as c_uint;
    let mut cache: *mut c_uchar = ::core::ptr::null_mut::<c_uchar>();
    cache = lmc_view.sublen.offset(
        (ZOPFLI_CACHE_LENGTH as size_t)
            .wrapping_mul(pos)
            .wrapping_mul(3 as size_t) as isize,
    ) as *mut c_uchar;
    if length < 3 as size_t {
        return;
    }
    i = 3 as size_t;
    while i <= length {
        if i == length
            || *sublen.offset(i as isize) as c_int
                != *sublen.offset(i.wrapping_add(1 as size_t) as isize) as c_int
        {
            *cache.offset(j.wrapping_mul(3 as size_t) as isize) =
                i.wrapping_sub(3 as size_t) as c_uchar;
            *cache.offset(j.wrapping_mul(3 as size_t).wrapping_add(1 as size_t) as isize) =
                (*sublen.offset(i as isize) as c_int % 256 as c_int)
                    as c_uchar;
            *cache.offset(j.wrapping_mul(3 as size_t).wrapping_add(2 as size_t) as isize) =
                ((*sublen.offset(i as isize) as c_int >> 8 as c_int)
                    % 256 as c_int) as c_uchar;
            bestlength = i as c_uint;
            j = j.wrapping_add(1);
            if j >= ZOPFLI_CACHE_LENGTH as size_t {
                break;
            }
        }
        i = i.wrapping_add(1);
    }
    if j < ZOPFLI_CACHE_LENGTH as size_t {
        *cache.offset(
            ((ZOPFLI_CACHE_LENGTH - 1 as c_int) * 3 as c_int) as isize,
        ) = bestlength.wrapping_sub(3 as c_uint) as c_uchar;
    }
}
#[inline]
pub unsafe fn ZopfliCacheToSublen(
    mut lmc: *const ZopfliLongestMatchCache,
    mut pos: size_t,
    mut length: size_t,
    mut sublen: *mut c_ushort,
) {
    let lmc_view: &ZopfliLongestMatchCache = unsafe { &*lmc };
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut maxlength: c_uint = ZopfliMaxCachedSublen(lmc, pos, length);
    let mut prevlength: c_uint = 0 as c_uint;
    let mut cache: *mut c_uchar = ::core::ptr::null_mut::<c_uchar>();
    if length < 3 as size_t {
        return;
    }
    cache = lmc_view.sublen.offset(
        (ZOPFLI_CACHE_LENGTH as size_t)
            .wrapping_mul(pos)
            .wrapping_mul(3 as size_t) as isize,
    ) as *mut c_uchar;
    j = 0 as size_t;
    while j < ZOPFLI_CACHE_LENGTH as size_t {
        let mut length_0: c_uint =
            (*cache.offset(j.wrapping_mul(3 as size_t) as isize) as c_int
                + 3 as c_int) as c_uint;
        let mut dist: c_uint =
            (*cache.offset(j.wrapping_mul(3 as size_t).wrapping_add(1 as size_t) as isize)
                as c_int
                + 256 as c_int
                    * *cache.offset(j.wrapping_mul(3 as size_t).wrapping_add(2 as size_t) as isize)
                        as c_int) as c_uint;
        i = prevlength as size_t;
        while i <= length_0 as size_t {
            *sublen.offset(i as isize) = dist as c_ushort;
            i = i.wrapping_add(1);
        }
        if length_0 == maxlength {
            break;
        }
        prevlength = length_0.wrapping_add(1 as c_uint);
        j = j.wrapping_add(1);
    }
}
#[inline]
pub unsafe fn ZopfliMaxCachedSublen(
    mut lmc: *const ZopfliLongestMatchCache,
    mut pos: size_t,
    mut length: size_t,
) -> c_uint {
    let lmc_view: &ZopfliLongestMatchCache = unsafe { &*lmc };
    let mut cache: *mut c_uchar = ::core::ptr::null_mut::<c_uchar>();
    cache = lmc_view.sublen.offset(
        (ZOPFLI_CACHE_LENGTH as size_t)
            .wrapping_mul(pos)
            .wrapping_mul(3 as size_t) as isize,
    ) as *mut c_uchar;
    if *cache.offset(1 as c_int as isize) as c_int
        == 0 as c_int
        && *cache.offset(2 as c_int as isize) as c_int
            == 0 as c_int
    {
        return 0 as c_uint;
    }
    return (*cache.offset(
        ((ZOPFLI_CACHE_LENGTH - 1 as c_int) * 3 as c_int) as isize,
    ) as c_int
        + 3 as c_int) as c_uint;
}
pub const ZOPFLI_CACHE_LENGTH: c_int = 8 as c_int;
