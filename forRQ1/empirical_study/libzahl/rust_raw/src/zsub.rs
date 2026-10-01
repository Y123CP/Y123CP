use ::core::arch::asm;
extern "C" {
    fn libzahl_realloc(_: *mut zahl, _: size_t);
    fn zadd_unsigned(_: *mut zahl, _: *mut zahl, _: *mut zahl);
    static mut libzahl_tmp_sub: z_t;
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
pub type z_t = [zahl; 1];
pub const UINT64_MAX: ::core::ffi::c_ulong = 18446744073709551615 as ::core::ffi::c_ulong;
pub const ZAHL_CHAR_MAX: ::core::ffi::c_ulong = UINT64_MAX;
#[inline]
unsafe extern "C" fn libzahl_memcpy(
    mut d: *mut zahl_char_t,
    mut s: *const zahl_char_t,
    mut n: size_t,
) {
    let mut current_block_42: u64;
    match n {
        20 => {
            *d.offset((20 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize) =
                *s.offset((20 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize);
            current_block_42 = 1391058008256916972;
        }
        19 => {
            current_block_42 = 1391058008256916972;
        }
        18 => {
            current_block_42 = 6893763438629148997;
        }
        17 => {
            current_block_42 = 6421391391121710;
        }
        16 => {
            current_block_42 = 17162270685673637631;
        }
        15 => {
            current_block_42 = 451532537293855284;
        }
        14 => {
            current_block_42 = 11729455350843475820;
        }
        13 => {
            current_block_42 = 3088197500626375644;
        }
        12 => {
            current_block_42 = 4423598539606377485;
        }
        11 => {
            current_block_42 = 5993199832596135542;
        }
        10 => {
            current_block_42 = 4636481217339069480;
        }
        9 => {
            current_block_42 = 12386775860387733485;
        }
        8 => {
            current_block_42 = 14468891521630732394;
        }
        7 => {
            current_block_42 = 13031440908851014233;
        }
        6 => {
            current_block_42 = 8848739226222623272;
        }
        5 => {
            current_block_42 = 11538747596021838440;
        }
        4 => {
            current_block_42 = 1606853116260846100;
        }
        3 => {
            current_block_42 = 1919423070859572111;
        }
        2 => {
            current_block_42 = 8266687893124359221;
        }
        1 => {
            current_block_42 = 1890451546187205982;
        }
        0 => {
            current_block_42 = 1836292691772056875;
        }
        _ => {
            let mut t: zahl_char_t = 0;
            asm!(
                "\n", "    shlq $3, {3}\n", "    addq {1}, {3}\n", " 1:\n",
                "    movq 0({2}), {0}\n", "    movq {0}, 0({1})\n",
                "    movq 8({2}), {0}\n", "    movq {0}, 8({1})\n",
                "    movq 16({2}), {0}\n", "    movq {0}, 16({1})\n",
                "    movq 24({2}), {0}\n", "    movq {0}, 24({1})\n",
                "    addq $32, {2}\n", "    addq $32, {1}\n", "    cmpq {3}, {1}\n",
                "    jl 1b\n", lateout(reg) t, inlateout(reg) d, inlateout(reg) s,
                inlateout(reg) n, options(preserves_flags, att_syntax)
            );
            current_block_42 = 1836292691772056875;
        }
    }
    match current_block_42 {
        1391058008256916972 => {
            *d.offset((19 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize) =
                *s.offset((19 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize);
            current_block_42 = 6893763438629148997;
        }
        _ => {}
    }
    match current_block_42 {
        6893763438629148997 => {
            *d.offset((18 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize) =
                *s.offset((18 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize);
            current_block_42 = 6421391391121710;
        }
        _ => {}
    }
    match current_block_42 {
        6421391391121710 => {
            *d.offset((17 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize) =
                *s.offset((17 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize);
            current_block_42 = 17162270685673637631;
        }
        _ => {}
    }
    match current_block_42 {
        17162270685673637631 => {
            *d.offset((16 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize) =
                *s.offset((16 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize);
            current_block_42 = 451532537293855284;
        }
        _ => {}
    }
    match current_block_42 {
        451532537293855284 => {
            *d.offset((15 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize) =
                *s.offset((15 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize);
            current_block_42 = 11729455350843475820;
        }
        _ => {}
    }
    match current_block_42 {
        11729455350843475820 => {
            *d.offset((14 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize) =
                *s.offset((14 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize);
            current_block_42 = 3088197500626375644;
        }
        _ => {}
    }
    match current_block_42 {
        3088197500626375644 => {
            *d.offset((13 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize) =
                *s.offset((13 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize);
            current_block_42 = 4423598539606377485;
        }
        _ => {}
    }
    match current_block_42 {
        4423598539606377485 => {
            *d.offset((12 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize) =
                *s.offset((12 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize);
            current_block_42 = 5993199832596135542;
        }
        _ => {}
    }
    match current_block_42 {
        5993199832596135542 => {
            *d.offset((11 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize) =
                *s.offset((11 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize);
            current_block_42 = 4636481217339069480;
        }
        _ => {}
    }
    match current_block_42 {
        4636481217339069480 => {
            *d.offset((10 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize) =
                *s.offset((10 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize);
            current_block_42 = 12386775860387733485;
        }
        _ => {}
    }
    match current_block_42 {
        12386775860387733485 => {
            *d.offset((9 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize) =
                *s.offset((9 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize);
            current_block_42 = 14468891521630732394;
        }
        _ => {}
    }
    match current_block_42 {
        14468891521630732394 => {
            *d.offset((8 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize) =
                *s.offset((8 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize);
            current_block_42 = 13031440908851014233;
        }
        _ => {}
    }
    match current_block_42 {
        13031440908851014233 => {
            *d.offset((7 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize) =
                *s.offset((7 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize);
            current_block_42 = 8848739226222623272;
        }
        _ => {}
    }
    match current_block_42 {
        8848739226222623272 => {
            *d.offset((6 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize) =
                *s.offset((6 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize);
            current_block_42 = 11538747596021838440;
        }
        _ => {}
    }
    match current_block_42 {
        11538747596021838440 => {
            *d.offset((5 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize) =
                *s.offset((5 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize);
            current_block_42 = 1606853116260846100;
        }
        _ => {}
    }
    match current_block_42 {
        1606853116260846100 => {
            *d.offset((4 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize) =
                *s.offset((4 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize);
            current_block_42 = 1919423070859572111;
        }
        _ => {}
    }
    match current_block_42 {
        1919423070859572111 => {
            *d.offset((3 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize) =
                *s.offset((3 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize);
            current_block_42 = 8266687893124359221;
        }
        _ => {}
    }
    match current_block_42 {
        8266687893124359221 => {
            *d.offset((2 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize) =
                *s.offset((2 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize);
            current_block_42 = 1890451546187205982;
        }
        _ => {}
    }
    match current_block_42 {
        1890451546187205982 => {
            *d.offset((1 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize) =
                *s.offset((1 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize);
        }
        _ => {}
    };
}
#[inline]
unsafe extern "C" fn zzero(mut a: *mut zahl) -> ::core::ffi::c_int {
    return ((*a).sign == 0) as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn zsignum(mut a: *mut zahl) -> ::core::ffi::c_int {
    return (*a).sign;
}
#[inline]
unsafe extern "C" fn zneg(mut a: *mut zahl, mut b: *mut zahl) {
    if a != b {
        zset(a, b);
    }
    (*a).sign = -(*a).sign;
}
#[inline]
unsafe extern "C" fn zabs(mut a: *mut zahl, mut b: *mut zahl) {
    if a != b {
        zset(a, b);
    }
    (*a).sign &= 1 as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn zset(mut a: *mut zahl, mut b: *mut zahl) {
    if ((*b).sign == 0 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        (*a).sign = 0 as ::core::ffi::c_int;
    } else {
        (*a).sign = (*b).sign;
        (*a).used = (*b).used;
        if (*a).alloced < (*b).used {
            libzahl_realloc(a as *mut zahl, (*b).used);
        }
        libzahl_memcpy((*a).chars, (*b).chars, (*b).used);
    };
}
#[inline]
unsafe extern "C" fn zcmpmag(mut a: *mut zahl, mut b: *mut zahl) -> ::core::ffi::c_int {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    if (zzero(a) != 0) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        return -((zzero(b) == 0) as ::core::ffi::c_int);
    }
    if (zzero(b) != 0) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        return 1 as ::core::ffi::c_int;
    }
    i = (*a).used.wrapping_sub(1 as size_t);
    j = (*b).used.wrapping_sub(1 as size_t);
    while i > j {
        if *(*a).chars.offset(i as isize) != 0 {
            return 1 as ::core::ffi::c_int;
        }
        (*a).used = (*a).used.wrapping_sub(1);
        i = i.wrapping_sub(1);
    }
    while j > i {
        if *(*b).chars.offset(j as isize) != 0 {
            return -(1 as ::core::ffi::c_int);
        }
        (*b).used = (*b).used.wrapping_sub(1);
        j = j.wrapping_sub(1);
    }
    while i != 0 && *(*a).chars.offset(i as isize) == *(*b).chars.offset(i as isize) {
        i = i.wrapping_sub(1);
    }
    return if *(*a).chars.offset(i as isize) < *(*b).chars.offset(i as isize) {
        -(1 as ::core::ffi::c_int)
    } else {
        (*(*a).chars.offset(i as isize) > *(*b).chars.offset(i as isize)) as ::core::ffi::c_int
    };
}
#[inline]
unsafe extern "C" fn zsub_impl(mut a: *mut zahl, mut b: *mut zahl, mut n: size_t) {
    let mut carry: zahl_char_t = 0 as zahl_char_t;
    let mut tcarry: zahl_char_t = 0;
    let mut i: size_t = 0;
    i = 0 as size_t;
    while i < n {
        tcarry = (if carry != 0 {
            (*(*a).chars.offset(i as isize) <= *(*b).chars.offset(i as isize)) as ::core::ffi::c_int
        } else {
            (*(*a).chars.offset(i as isize) < *(*b).chars.offset(i as isize)) as ::core::ffi::c_int
        }) as zahl_char_t;
        let ref mut fresh0 = *(*a).chars.offset(i as isize);
        *fresh0 = (*fresh0 as ::core::ffi::c_ulong)
            .wrapping_sub(*(*b).chars.offset(i as isize) as ::core::ffi::c_ulong)
            as zahl_char_t as zahl_char_t;
        let ref mut fresh1 = *(*a).chars.offset(i as isize);
        *fresh1 = (*fresh1 as ::core::ffi::c_ulong).wrapping_sub(carry as ::core::ffi::c_ulong)
            as zahl_char_t as zahl_char_t;
        carry = tcarry;
        i = i.wrapping_add(1);
    }
    if carry != 0 {
        while *(*a).chars.offset(i as isize) == 0 {
            let fresh2 = i;
            i = i.wrapping_add(1);
            *(*a).chars.offset(fresh2 as isize) = ZAHL_CHAR_MAX as zahl_char_t;
        }
        if *(*a).chars.offset(i as isize) == 1 as zahl_char_t {
            (*a).used = (*a).used.wrapping_sub(1);
        } else {
            let ref mut fresh3 = *(*a).chars.offset(i as isize);
            *fresh3 = (*fresh3 as ::core::ffi::c_ulong).wrapping_sub(1 as ::core::ffi::c_ulong)
                as zahl_char_t as zahl_char_t;
        }
    }
}
#[inline]
unsafe extern "C" fn libzahl_zsub_unsigned(mut a: *mut zahl, mut b: *mut zahl, mut c: *mut zahl) {
    let mut magcmp: ::core::ffi::c_int = 0;
    let mut n: size_t = 0;
    if (zzero(b) != 0) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        zabs(a, c);
        zneg(a, a);
        return;
    } else if (zzero(c) != 0) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        zabs(a, b);
        return;
    }
    magcmp = zcmpmag(b, c);
    if (magcmp <= 0 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        if (magcmp == 0 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
            (*a).sign = 0 as ::core::ffi::c_int;
            return;
        }
        n = (*b).used;
        if a == b {
            zset(&raw mut libzahl_tmp_sub as *mut zahl, b);
            if a != c {
                zset(a, c);
            }
            zsub_impl(a, &raw mut libzahl_tmp_sub as *mut zahl, n);
        } else {
            if a != c {
                zset(a, c);
            }
            zsub_impl(a, b, n);
        }
    } else {
        n = (*c).used;
        if (a == c) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
            zset(&raw mut libzahl_tmp_sub as *mut zahl, c);
            if a != b {
                zset(a, b);
            }
            zsub_impl(a, &raw mut libzahl_tmp_sub as *mut zahl, n);
        } else {
            if a != b {
                zset(a, b);
            }
            zsub_impl(a, c, n);
        }
    }
    (*a).sign = magcmp;
}
#[no_mangle]
pub unsafe extern "C" fn zsub_unsigned(mut a: *mut zahl, mut b: *mut zahl, mut c: *mut zahl) {
    libzahl_zsub_unsigned(a, b, c);
}
#[no_mangle]
pub unsafe extern "C" fn zsub_nonnegative_assign(mut a: *mut zahl, mut b: *mut zahl) {
    if (zzero(b) != 0) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        zabs(a, a);
    } else if (zcmpmag(a, b) == 0) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        (*a).sign = 0 as ::core::ffi::c_int;
    } else {
        zsub_impl(a, b, (*b).used);
    };
}
#[no_mangle]
pub unsafe extern "C" fn zsub_positive_assign(mut a: *mut zahl, mut b: *mut zahl) {
    zsub_impl(a, b, (*b).used);
}
#[no_mangle]
pub unsafe extern "C" fn zsub(mut a: *mut zahl, mut b: *mut zahl, mut c: *mut zahl) {
    if (zzero(b) != 0) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        zneg(a, c);
    } else if (zzero(c) != 0) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        if a != b {
            zset(a, b);
        }
    } else if (zsignum(b) < 0 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long
        != 0
    {
        if zsignum(c) < 0 as ::core::ffi::c_int {
            libzahl_zsub_unsigned(a, c, b);
        } else {
            zadd_unsigned(a, b, c);
            (*a).sign = -zsignum(a);
        }
    } else if zsignum(c) < 0 as ::core::ffi::c_int {
        zadd_unsigned(a, b, c);
    } else {
        libzahl_zsub_unsigned(a, b, c);
    };
}
