use ::core::arch::asm;
extern "C" {
    fn libzahl_realloc(_: *mut zahl, _: size_t);
    fn zmodsqr(_: *mut zahl, _: *mut zahl, _: *mut zahl);
    fn zmodpow(_: *mut zahl, _: *mut zahl, _: *mut zahl, _: *mut zahl);
    fn zadd_unsigned(_: *mut zahl, _: *mut zahl, _: *mut zahl);
    fn zsub_unsigned(_: *mut zahl, _: *mut zahl, _: *mut zahl);
    fn zrsh(_: *mut zahl, _: *mut zahl, _: size_t);
    fn zrand(_: *mut zahl, _: zranddev, _: zranddist, _: *mut zahl);
    static mut libzahl_tmp_ptest_a: z_t;
    static mut libzahl_tmp_ptest_n4: z_t;
    static mut libzahl_tmp_ptest_n1: z_t;
    static mut libzahl_tmp_ptest_d: z_t;
    static mut libzahl_tmp_ptest_x: z_t;
    static mut libzahl_const_1: z_t;
    static mut libzahl_const_2: z_t;
    static mut libzahl_const_4: z_t;
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
pub type zprimality = ::core::ffi::c_uint;
pub const PRIME: zprimality = 2;
pub const PROBABLY_PRIME: zprimality = 1;
pub const NONPRIME: zprimality = 0;
pub type zranddev = ::core::ffi::c_uint;
pub const LIBC_RAND48_RANDOM: zranddev = 6;
pub const LIBC_RANDOM_RANDOM: zranddev = 5;
pub const LIBC_RAND_RANDOM: zranddev = 4;
pub const FASTEST_RANDOM: zranddev = 3;
pub const DEFAULT_RANDOM: zranddev = 2;
pub const SECURE_RANDOM: zranddev = 1;
pub const FAST_RANDOM: zranddev = 0;
pub type zranddist = ::core::ffi::c_uint;
pub const MODUNIFORM: zranddist = 2;
pub const UNIFORM: zranddist = 1;
pub const QUASIUNIFORM: zranddist = 0;
pub const SIZE_MAX: ::core::ffi::c_ulong = 18446744073709551615 as ::core::ffi::c_ulong;
#[inline]
unsafe extern "C" fn libzahl_memcpy(
    mut d_0: *mut zahl_char_t,
    mut s: *const zahl_char_t,
    mut n: size_t,
) {
    let mut current_block_42: u64;
    match n {
        20 => {
            *d_0.offset((20 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize) =
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
                "    jl 1b\n", lateout(reg) t, inlateout(reg) d_0, inlateout(reg) s,
                inlateout(reg) n, options(preserves_flags, att_syntax)
            );
            current_block_42 = 1836292691772056875;
        }
    }
    match current_block_42 {
        1391058008256916972 => {
            *d_0.offset((19 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize) =
                *s.offset((19 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize);
            current_block_42 = 6893763438629148997;
        }
        _ => {}
    }
    match current_block_42 {
        6893763438629148997 => {
            *d_0.offset((18 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize) =
                *s.offset((18 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize);
            current_block_42 = 6421391391121710;
        }
        _ => {}
    }
    match current_block_42 {
        6421391391121710 => {
            *d_0.offset((17 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize) =
                *s.offset((17 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize);
            current_block_42 = 17162270685673637631;
        }
        _ => {}
    }
    match current_block_42 {
        17162270685673637631 => {
            *d_0.offset((16 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize) =
                *s.offset((16 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize);
            current_block_42 = 451532537293855284;
        }
        _ => {}
    }
    match current_block_42 {
        451532537293855284 => {
            *d_0.offset((15 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize) =
                *s.offset((15 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize);
            current_block_42 = 11729455350843475820;
        }
        _ => {}
    }
    match current_block_42 {
        11729455350843475820 => {
            *d_0.offset((14 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize) =
                *s.offset((14 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize);
            current_block_42 = 3088197500626375644;
        }
        _ => {}
    }
    match current_block_42 {
        3088197500626375644 => {
            *d_0.offset((13 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize) =
                *s.offset((13 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize);
            current_block_42 = 4423598539606377485;
        }
        _ => {}
    }
    match current_block_42 {
        4423598539606377485 => {
            *d_0.offset((12 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize) =
                *s.offset((12 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize);
            current_block_42 = 5993199832596135542;
        }
        _ => {}
    }
    match current_block_42 {
        5993199832596135542 => {
            *d_0.offset((11 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize) =
                *s.offset((11 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize);
            current_block_42 = 4636481217339069480;
        }
        _ => {}
    }
    match current_block_42 {
        4636481217339069480 => {
            *d_0.offset((10 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize) =
                *s.offset((10 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize);
            current_block_42 = 12386775860387733485;
        }
        _ => {}
    }
    match current_block_42 {
        12386775860387733485 => {
            *d_0.offset((9 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize) =
                *s.offset((9 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize);
            current_block_42 = 14468891521630732394;
        }
        _ => {}
    }
    match current_block_42 {
        14468891521630732394 => {
            *d_0.offset((8 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize) =
                *s.offset((8 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize);
            current_block_42 = 13031440908851014233;
        }
        _ => {}
    }
    match current_block_42 {
        13031440908851014233 => {
            *d_0.offset((7 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize) =
                *s.offset((7 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize);
            current_block_42 = 8848739226222623272;
        }
        _ => {}
    }
    match current_block_42 {
        8848739226222623272 => {
            *d_0.offset((6 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize) =
                *s.offset((6 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize);
            current_block_42 = 11538747596021838440;
        }
        _ => {}
    }
    match current_block_42 {
        11538747596021838440 => {
            *d_0.offset((5 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize) =
                *s.offset((5 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize);
            current_block_42 = 1606853116260846100;
        }
        _ => {}
    }
    match current_block_42 {
        1606853116260846100 => {
            *d_0.offset((4 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize) =
                *s.offset((4 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize);
            current_block_42 = 1919423070859572111;
        }
        _ => {}
    }
    match current_block_42 {
        1919423070859572111 => {
            *d_0.offset((3 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize) =
                *s.offset((3 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize);
            current_block_42 = 8266687893124359221;
        }
        _ => {}
    }
    match current_block_42 {
        8266687893124359221 => {
            *d_0.offset((2 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize) =
                *s.offset((2 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize);
            current_block_42 = 1890451546187205982;
        }
        _ => {}
    }
    match current_block_42 {
        1890451546187205982 => {
            *d_0.offset((1 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize) =
                *s.offset((1 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize);
        }
        _ => {}
    };
}
#[inline]
unsafe extern "C" fn zeven(mut a_0: *mut zahl) -> ::core::ffi::c_int {
    return ((*a_0).sign == 0
        || !*(*a_0).chars.offset(0 as ::core::ffi::c_int as isize) & 1 as zahl_char_t != 0)
        as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn zzero(mut a_0: *mut zahl) -> ::core::ffi::c_int {
    return ((*a_0).sign == 0) as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn zsignum(mut a_0: *mut zahl) -> ::core::ffi::c_int {
    return (*a_0).sign;
}
#[inline]
unsafe extern "C" fn zswap(mut a_: *mut zahl, mut b_: *mut zahl) {
    let mut t: ::core::ffi::c_long = 0;
    let mut a_0: *mut ::core::ffi::c_long = a_ as *mut ::core::ffi::c_long;
    let mut b: *mut ::core::ffi::c_long = b_ as *mut ::core::ffi::c_long;
    t = *a_0.offset(0 as ::core::ffi::c_int as isize);
    *a_0.offset(0 as ::core::ffi::c_int as isize) = *b.offset(0 as ::core::ffi::c_int as isize);
    *b.offset(0 as ::core::ffi::c_int as isize) = t;
    t = *b.offset(1 as ::core::ffi::c_int as isize);
    *b.offset(1 as ::core::ffi::c_int as isize) = *a_0.offset(1 as ::core::ffi::c_int as isize);
    *a_0.offset(1 as ::core::ffi::c_int as isize) = t;
    t = *a_0.offset(2 as ::core::ffi::c_int as isize);
    *a_0.offset(2 as ::core::ffi::c_int as isize) = *b.offset(2 as ::core::ffi::c_int as isize);
    *b.offset(2 as ::core::ffi::c_int as isize) = t;
    t = *b.offset(3 as ::core::ffi::c_int as isize);
    *b.offset(3 as ::core::ffi::c_int as isize) = *a_0.offset(3 as ::core::ffi::c_int as isize);
    *a_0.offset(3 as ::core::ffi::c_int as isize) = t;
}
#[inline]
unsafe extern "C" fn zset(mut a_0: *mut zahl, mut b: *mut zahl) {
    if ((*b).sign == 0 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        (*a_0).sign = 0 as ::core::ffi::c_int;
    } else {
        (*a_0).sign = (*b).sign;
        (*a_0).used = (*b).used;
        if (*a_0).alloced < (*b).used {
            libzahl_realloc(a_0 as *mut zahl, (*b).used);
        }
        libzahl_memcpy((*a_0).chars, (*b).chars, (*b).used);
    };
}
#[inline]
unsafe extern "C" fn zsetu(mut a_0: *mut zahl, mut b: uint64_t) {
    if (b == 0) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        (*a_0).sign = 0 as ::core::ffi::c_int;
        return;
    }
    if (*a_0).alloced < 1 as size_t {
        libzahl_realloc(a_0 as *mut zahl, 1 as size_t);
    }
    (*a_0).sign = 1 as ::core::ffi::c_int;
    *(*a_0).chars.offset(0 as ::core::ffi::c_int as isize) = b;
    (*a_0).used = 1 as size_t;
}
#[inline]
unsafe extern "C" fn zlsb(mut a_0: *mut zahl) -> size_t {
    let mut i: size_t = 0 as size_t;
    if (zzero(a_0) != 0) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        return SIZE_MAX as size_t;
    }
    while *(*a_0).chars.offset(i as isize) == 0 {
        i = i.wrapping_add(1);
    }
    i = (i as ::core::ffi::c_ulong).wrapping_mul(
        (8 as usize).wrapping_mul(::core::mem::size_of::<zahl_char_t>() as usize)
            as ::core::ffi::c_ulong,
    ) as size_t as size_t;
    i = (i as ::core::ffi::c_ulong).wrapping_add(
        (*(*a_0).chars.offset(i as isize) as ::core::ffi::c_ulonglong).trailing_zeros() as i32
            as size_t as ::core::ffi::c_ulong,
    ) as size_t as size_t;
    return i;
}
#[inline]
unsafe extern "C" fn zcmpmag(mut a_0: *mut zahl, mut b: *mut zahl) -> ::core::ffi::c_int {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    if (zzero(a_0) != 0) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        return -((zzero(b) == 0) as ::core::ffi::c_int);
    }
    if (zzero(b) != 0) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        return 1 as ::core::ffi::c_int;
    }
    i = (*a_0).used.wrapping_sub(1 as size_t);
    j = (*b).used.wrapping_sub(1 as size_t);
    while i > j {
        if *(*a_0).chars.offset(i as isize) != 0 {
            return 1 as ::core::ffi::c_int;
        }
        (*a_0).used = (*a_0).used.wrapping_sub(1);
        i = i.wrapping_sub(1);
    }
    while j > i {
        if *(*b).chars.offset(j as isize) != 0 {
            return -(1 as ::core::ffi::c_int);
        }
        (*b).used = (*b).used.wrapping_sub(1);
        j = j.wrapping_sub(1);
    }
    while i != 0 && *(*a_0).chars.offset(i as isize) == *(*b).chars.offset(i as isize) {
        i = i.wrapping_sub(1);
    }
    return if *(*a_0).chars.offset(i as isize) < *(*b).chars.offset(i as isize) {
        -(1 as ::core::ffi::c_int)
    } else {
        (*(*a_0).chars.offset(i as isize) > *(*b).chars.offset(i as isize)) as ::core::ffi::c_int
    };
}
#[inline]
unsafe extern "C" fn zcmp(mut a_0: *mut zahl, mut b: *mut zahl) -> ::core::ffi::c_int {
    if zsignum(a_0) != zsignum(b) {
        return if zsignum(a_0) < zsignum(b) {
            -(1 as ::core::ffi::c_int)
        } else {
            (zsignum(a_0) > zsignum(b)) as ::core::ffi::c_int
        };
    }
    return zsignum(a_0) * zcmpmag(a_0, b);
}
#[inline]
unsafe extern "C" fn zcmpu(mut a_0: *mut zahl, mut b: uint64_t) -> ::core::ffi::c_int {
    if (b == 0) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        return zsignum(a_0);
    }
    if (zsignum(a_0) <= 0 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        return -(1 as ::core::ffi::c_int);
    }
    while *(*a_0)
        .chars
        .offset((*a_0).used.wrapping_sub(1 as size_t) as isize)
        == 0
    {
        (*a_0).used = (*a_0).used.wrapping_sub(1);
    }
    if (*a_0).used > 1 as size_t {
        return 1 as ::core::ffi::c_int;
    }
    return if *(*a_0).chars.offset(0 as ::core::ffi::c_int as isize) < b {
        -(1 as ::core::ffi::c_int)
    } else {
        (*(*a_0).chars.offset(0 as ::core::ffi::c_int as isize) > b) as ::core::ffi::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn zptest(
    mut witness: *mut zahl,
    mut n: *mut zahl,
    mut t: ::core::ffi::c_int,
) -> zprimality {
    let mut i: size_t = 0;
    let mut r: size_t = 0;
    if (zcmpu(n, 3 as uint64_t) <= 0 as ::core::ffi::c_int) as ::core::ffi::c_int
        as ::core::ffi::c_long
        != 0
    {
        if zcmpu(n, 1 as uint64_t) <= 0 as ::core::ffi::c_int {
            if !witness.is_null() {
                if witness != n {
                    zset(witness, n);
                }
            }
            return NONPRIME;
        } else {
            return PRIME;
        }
    }
    if (zeven(n) != 0) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        if !witness.is_null() {
            zsetu(witness, 2 as uint64_t);
        }
        return NONPRIME;
    }
    zsub_unsigned(
        &raw mut libzahl_tmp_ptest_n1 as *mut zahl,
        n,
        &raw mut libzahl_const_1 as *mut zahl,
    );
    zsub_unsigned(
        &raw mut libzahl_tmp_ptest_n4 as *mut zahl,
        n,
        &raw mut libzahl_const_4 as *mut zahl,
    );
    r = zlsb(&raw mut libzahl_tmp_ptest_n1 as *mut zahl);
    zrsh(
        &raw mut libzahl_tmp_ptest_d as *mut zahl,
        &raw mut libzahl_tmp_ptest_n1 as *mut zahl,
        r,
    );
    loop {
        let fresh0 = t;
        t = t - 1;
        if !(fresh0 != 0) {
            break;
        }
        zrand(
            &raw mut libzahl_tmp_ptest_a as *mut zahl,
            DEFAULT_RANDOM,
            UNIFORM,
            &raw mut libzahl_tmp_ptest_n4 as *mut zahl,
        );
        zadd_unsigned(
            &raw mut libzahl_tmp_ptest_a as *mut zahl,
            &raw mut libzahl_tmp_ptest_a as *mut zahl,
            &raw mut libzahl_const_2 as *mut zahl,
        );
        zmodpow(
            &raw mut libzahl_tmp_ptest_x as *mut zahl,
            &raw mut libzahl_tmp_ptest_a as *mut zahl,
            &raw mut libzahl_tmp_ptest_d as *mut zahl,
            n,
        );
        if zcmp(
            &raw mut libzahl_tmp_ptest_x as *mut zahl,
            &raw mut libzahl_const_1 as *mut zahl,
        ) == 0
            || zcmp(
                &raw mut libzahl_tmp_ptest_x as *mut zahl,
                &raw mut libzahl_tmp_ptest_n1 as *mut zahl,
            ) == 0
        {
            continue;
        }
        i = 1 as size_t;
        while i < r {
            zmodsqr(
                &raw mut libzahl_tmp_ptest_x as *mut zahl,
                &raw mut libzahl_tmp_ptest_x as *mut zahl,
                n,
            );
            if zcmp(
                &raw mut libzahl_tmp_ptest_x as *mut zahl,
                &raw mut libzahl_const_1 as *mut zahl,
            ) == 0
            {
                if !witness.is_null() {
                    zswap(witness, &raw mut libzahl_tmp_ptest_a as *mut zahl);
                }
                return NONPRIME;
            }
            if zcmp(
                &raw mut libzahl_tmp_ptest_x as *mut zahl,
                &raw mut libzahl_tmp_ptest_n1 as *mut zahl,
            ) == 0
            {
                break;
            }
            i = i.wrapping_add(1);
        }
        if i == r {
            if !witness.is_null() {
                zswap(witness, &raw mut libzahl_tmp_ptest_a as *mut zahl);
            }
            return NONPRIME;
        }
    }
    return PROBABLY_PRIME;
}
