extern "C" {
    fn libzahl_realloc(_: *mut zahl, _: size_t);
}
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
pub const ZAHL_LB_BITS_PER_CHAR: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn libzahl_memset(mut a: *mut zahl_char_t, mut v: zahl_char_t, mut n: size_t) {
    let mut i: size_t = 0;
    i = 0 as size_t;
    while i < n {
        *a.offset(i.wrapping_add(0 as size_t) as isize) = v;
        *a.offset(i.wrapping_add(1 as size_t) as isize) = v;
        *a.offset(i.wrapping_add(2 as size_t) as isize) = v;
        *a.offset(i.wrapping_add(3 as size_t) as isize) = v;
        i = (i as ::core::ffi::c_ulong).wrapping_add(4 as ::core::ffi::c_ulong) as size_t as size_t;
    }
}
#[inline]
unsafe extern "C" fn zzero(mut a: *mut zahl) -> ::core::ffi::c_int {
    return ((*a).sign == 0) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn zbset_ll_set(mut a: *mut zahl, mut bit: size_t) {
    let mut mask: zahl_char_t = 1 as zahl_char_t;
    let mut chars: size_t = bit >> ZAHL_LB_BITS_PER_CHAR;
    if zzero(a) != 0 {
        (*a).used = 0 as size_t;
        (*a).sign = 1 as ::core::ffi::c_int;
    }
    if (chars >= (*a).used) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        if (*a).alloced < chars.wrapping_add(1 as size_t) {
            libzahl_realloc(a as *mut zahl, chars.wrapping_add(1 as size_t));
        }
        libzahl_memset(
            (*a).chars.offset((*a).used as isize),
            0 as zahl_char_t,
            chars.wrapping_add(1 as size_t).wrapping_sub((*a).used),
        );
        (*a).used = chars.wrapping_add(1 as size_t);
    }
    bit = bit & (ZAHL_BITS_PER_CHAR - 1 as ::core::ffi::c_int) as size_t;
    mask <<= bit;
    let ref mut fresh2 = *(*a).chars.offset(chars as isize);
    *fresh2 = (*fresh2 as ::core::ffi::c_ulong | mask as ::core::ffi::c_ulong) as zahl_char_t;
}
#[no_mangle]
pub unsafe extern "C" fn zbset_ll_clear(mut a: *mut zahl, mut bit: size_t) {
    let mut mask: zahl_char_t = 1 as zahl_char_t;
    let mut chars: size_t = bit >> ZAHL_LB_BITS_PER_CHAR;
    if (chars >= (*a).used) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        return;
    }
    bit = bit & (ZAHL_BITS_PER_CHAR - 1 as ::core::ffi::c_int) as size_t;
    mask <<= bit;
    let ref mut fresh0 = *(*a).chars.offset(chars as isize);
    *fresh0 = (*fresh0 as ::core::ffi::c_ulong & !mask as ::core::ffi::c_ulong) as zahl_char_t;
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
#[no_mangle]
pub unsafe extern "C" fn zbset_ll_flip(mut a: *mut zahl, mut bit: size_t) {
    let mut mask: zahl_char_t = 1 as zahl_char_t;
    let mut chars: size_t = bit >> ZAHL_LB_BITS_PER_CHAR;
    if zzero(a) != 0 {
        (*a).used = 0 as size_t;
        (*a).sign = 1 as ::core::ffi::c_int;
    }
    if (chars >= (*a).used) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        if (*a).alloced < chars.wrapping_add(1 as size_t) {
            libzahl_realloc(a as *mut zahl, chars.wrapping_add(1 as size_t));
        }
        libzahl_memset(
            (*a).chars.offset((*a).used as isize),
            0 as zahl_char_t,
            chars.wrapping_add(1 as size_t).wrapping_sub((*a).used),
        );
        (*a).used = chars.wrapping_add(1 as size_t);
    }
    bit = bit & (ZAHL_BITS_PER_CHAR - 1 as ::core::ffi::c_int) as size_t;
    mask <<= bit;
    let ref mut fresh1 = *(*a).chars.offset(chars as isize);
    *fresh1 = (*fresh1 as ::core::ffi::c_ulong ^ mask as ::core::ffi::c_ulong) as zahl_char_t;
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
