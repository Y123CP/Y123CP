pub type size_t = usize;
pub type __uint64_t = u64;
pub type uint64_t = __uint64_t;
pub type zahl_char_t = uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct zahl {
    pub sign: ::core::ffi::c_int,
    pub padding__: ::core::ffi::c_int,
    pub used: size_t,
    pub alloced: size_t,
    pub chars: *mut zahl_char_t,
}
pub const ZAHL_BITS_PER_CHAR: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn zzero(mut a: *mut zahl) -> ::core::ffi::c_int {
    return ((*a).sign == 0) as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn zsignum(mut a: *mut zahl) -> ::core::ffi::c_int {
    return (*a).sign;
}
#[inline]
unsafe extern "C" fn zbits(mut a: *mut zahl) -> size_t {
    let mut rc: size_t = 0;
    if (zzero(a) != 0) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        return 1 as size_t;
    }
    while *(*a)
        .chars
        .offset((*a).used.wrapping_sub(1 as size_t) as isize)
        == 0
    {
        (*a).used = (*a).used.wrapping_sub(1);
    }
    rc = (*a)
        .used
        .wrapping_mul(8 as size_t)
        .wrapping_mul(::core::mem::size_of::<zahl_char_t>() as size_t);
    rc = (rc as ::core::ffi::c_ulong).wrapping_sub(
        (*(*a)
            .chars
            .offset((*a).used.wrapping_sub(1 as size_t) as isize)
            as ::core::ffi::c_ulonglong)
            .leading_zeros() as i32 as size_t as ::core::ffi::c_ulong,
    ) as size_t as size_t;
    return rc;
}
#[no_mangle]
pub unsafe extern "C" fn znot(mut a: *mut zahl, mut b: *mut zahl) {
    let mut bits: size_t = 0;
    if (zzero(b) != 0) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        (*a).sign = 0 as ::core::ffi::c_int;
        return;
    }
    bits = zbits(b);
    (*a).used = (*b).used;
    (*a).sign = -zsignum(b);
    let mut a__: *mut zahl_char_t = (*a).chars;
    let mut b__: *const zahl_char_t = (*b).chars;
    let mut i__: size_t = 0;
    let mut n__: size_t = (*a).used;
    i__ = 0 as size_t;
    while i__ < n__ {
        *a__.offset(i__.wrapping_add(0 as size_t) as isize) =
            !*b__.offset(i__.wrapping_add(0 as size_t) as isize);
        *a__.offset(i__.wrapping_add(1 as size_t) as isize) =
            !*b__.offset(i__.wrapping_add(1 as size_t) as isize);
        *a__.offset(i__.wrapping_add(2 as size_t) as isize) =
            !*b__.offset(i__.wrapping_add(2 as size_t) as isize);
        *a__.offset(i__.wrapping_add(3 as size_t) as isize) =
            !*b__.offset(i__.wrapping_add(3 as size_t) as isize);
        i__ = (i__ as ::core::ffi::c_ulong).wrapping_add(4 as ::core::ffi::c_ulong) as size_t
            as size_t;
    }
    bits = bits & (ZAHL_BITS_PER_CHAR - 1 as ::core::ffi::c_int) as size_t;
    if bits != 0 {
        let ref mut fresh0 = *(*a)
            .chars
            .offset((*a).used.wrapping_sub(1 as size_t) as isize);
        *fresh0 = (*fresh0 as ::core::ffi::c_ulong
            & ((1 as ::core::ffi::c_int as zahl_char_t) << bits).wrapping_sub(1 as zahl_char_t)
                as ::core::ffi::c_ulong) as zahl_char_t;
    }
    while (*a).used != 0
        && *(*a)
            .chars
            .offset((*a).used.wrapping_sub(1 as size_t) as isize)
            == 0
    {
        (*a).used = (*a).used.wrapping_sub(1);
    }
    if (*a).used == 0 {
        (*a).sign = 0 as ::core::ffi::c_int;
    }
}
