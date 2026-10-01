use ::core::arch::asm;
extern "C" {
    fn libzahl_realloc(_: *mut zahl, _: size_t);
    fn zsub_positive_assign(_: *mut zahl, _: *mut zahl);
    fn zlsh(_: *mut zahl, _: *mut zahl, _: size_t);
    fn zrsh(_: *mut zahl, _: *mut zahl, _: size_t);
    static mut libzahl_tmp_gcd_v: z_t;
    static mut libzahl_tmp_gcd_u: z_t;
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
pub const SIZE_MAX: ::core::ffi::c_ulong = 18446744073709551615 as ::core::ffi::c_ulong;
pub const ZAHL_BITS_PER_CHAR: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
pub const ZAHL_LB_BITS_PER_CHAR: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
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
unsafe extern "C" fn zlsb(mut a: *mut zahl) -> size_t {
    let mut i: size_t = 0 as size_t;
    if (zzero(a) != 0) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        return SIZE_MAX as size_t;
    }
    while *(*a).chars.offset(i as isize) == 0 {
        i = i.wrapping_add(1);
    }
    i = (i as ::core::ffi::c_ulong).wrapping_mul(
        (8 as usize).wrapping_mul(::core::mem::size_of::<zahl_char_t>() as usize)
            as ::core::ffi::c_ulong,
    ) as size_t as size_t;
    i = (i as ::core::ffi::c_ulong).wrapping_add(
        (*(*a).chars.offset(i as isize) as ::core::ffi::c_ulonglong).trailing_zeros() as i32
            as size_t as ::core::ffi::c_ulong,
    ) as size_t as size_t;
    return i;
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
pub const BITS_PER_CHAR: ::core::ffi::c_int = ZAHL_BITS_PER_CHAR;
#[inline]
unsafe extern "C" fn zrsh_taint(mut a: *mut zahl, mut bits: size_t) {
    let mut i: size_t = 0;
    let mut chars: size_t = 0;
    let mut cbits: size_t = 0;
    if (bits == 0) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        return;
    }
    if (zzero(a) != 0) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        return;
    }
    chars = bits >> ZAHL_LB_BITS_PER_CHAR;
    if (chars >= (*a).used || zbits(a) <= bits) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        (*a).sign = 0 as ::core::ffi::c_int;
        return;
    }
    bits = bits & (ZAHL_BITS_PER_CHAR - 1 as ::core::ffi::c_int) as size_t;
    cbits = (BITS_PER_CHAR as size_t).wrapping_sub(bits);
    if (chars != 0) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        (*a).used = ((*a).used as ::core::ffi::c_ulong).wrapping_sub(chars as ::core::ffi::c_ulong)
            as size_t as size_t;
        (*a).chars = (*a).chars.offset(chars as isize);
    }
    if (bits != 0) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        *(*a).chars.offset(0 as ::core::ffi::c_int as isize) >>= bits;
        i = 1 as size_t;
        while i < (*a).used {
            let ref mut fresh2 = *(*a).chars.offset(i.wrapping_sub(1 as size_t) as isize);
            *fresh2 = (*fresh2 as ::core::ffi::c_ulong
                | (*(*a).chars.offset(i as isize) << cbits) as ::core::ffi::c_ulong)
                as zahl_char_t;
            *(*a).chars.offset(i as isize) >>= bits;
            i = i.wrapping_add(1);
        }
        while *(*a)
            .chars
            .offset((*a).used.wrapping_sub(1 as size_t) as isize)
            == 0
        {
            (*a).used = (*a).used.wrapping_sub(1);
        }
    }
}
#[inline]
unsafe extern "C" fn zswap_tainted_unsigned(mut a: *mut zahl, mut b: *mut zahl) {
    let mut t: z_t = [zahl {
        sign: 0,
        padding__: 0,
        used: 0,
        alloced: 0,
        chars: ::core::ptr::null::<zahl_char_t>() as *mut zahl_char_t,
    }; 1];
    (*(&raw mut t as *mut zahl)).used = (*a).used;
    (*a).used = (*b).used;
    (*b).used = (*(&raw mut t as *mut zahl)).used;
    let ref mut fresh3 = (*(&raw mut t as *mut zahl)).chars;
    *fresh3 = (*b).chars;
    (*b).chars = (*a).chars;
    (*a).chars = (*(&raw mut t as *mut zahl)).chars;
}
#[no_mangle]
pub unsafe extern "C" fn zgcd(mut a: *mut zahl, mut b: *mut zahl, mut c: *mut zahl) {
    let mut shifts: size_t = 0;
    let mut u_orig: *mut zahl_char_t = ::core::ptr::null_mut::<zahl_char_t>();
    let mut v_orig: *mut zahl_char_t = ::core::ptr::null_mut::<zahl_char_t>();
    let mut u_lsb: size_t = 0;
    let mut v_lsb: size_t = 0;
    let mut neg: ::core::ffi::c_int = 0;
    let mut cmpmag: ::core::ffi::c_int = 0;
    if (zzero(b) != 0) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        if a != c {
            zset(a, c);
        }
        return;
    }
    if (zzero(c) != 0) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        if a != b {
            zset(a, b);
        }
        return;
    }
    neg = (zsignum(b) & zsignum(c) < 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
    u_lsb = zlsb(b);
    v_lsb = zlsb(c);
    shifts = if u_lsb < v_lsb { u_lsb } else { v_lsb };
    zrsh(&raw mut libzahl_tmp_gcd_u as *mut zahl, b, u_lsb);
    zrsh(&raw mut libzahl_tmp_gcd_v as *mut zahl, c, v_lsb);
    u_orig = (*(&raw mut libzahl_tmp_gcd_u as *mut zahl)).chars;
    v_orig = (*(&raw mut libzahl_tmp_gcd_v as *mut zahl)).chars;
    loop {
        cmpmag = zcmpmag(
            &raw mut libzahl_tmp_gcd_u as *mut zahl,
            &raw mut libzahl_tmp_gcd_v as *mut zahl,
        );
        if (cmpmag >= 0 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
            if (cmpmag == 0 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long != 0
            {
                break;
            }
            zswap_tainted_unsigned(
                &raw mut libzahl_tmp_gcd_u as *mut zahl,
                &raw mut libzahl_tmp_gcd_v as *mut zahl,
            );
        }
        zsub_positive_assign(
            &raw mut libzahl_tmp_gcd_v as *mut zahl,
            &raw mut libzahl_tmp_gcd_u as *mut zahl,
        );
        zrsh_taint(
            &raw mut libzahl_tmp_gcd_v as *mut zahl,
            zlsb(&raw mut libzahl_tmp_gcd_v as *mut zahl),
        );
    }
    zlsh(a, &raw mut libzahl_tmp_gcd_u as *mut zahl, shifts);
    (*a).sign = if neg != 0 {
        -(1 as ::core::ffi::c_int)
    } else {
        1 as ::core::ffi::c_int
    };
    let ref mut fresh0 = (*(&raw mut libzahl_tmp_gcd_u as *mut zahl)).chars;
    *fresh0 = u_orig;
    let ref mut fresh1 = (*(&raw mut libzahl_tmp_gcd_v as *mut zahl)).chars;
    *fresh1 = v_orig;
}
