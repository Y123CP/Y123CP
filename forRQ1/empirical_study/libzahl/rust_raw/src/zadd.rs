use ::core::arch::asm;
extern "C" {
    fn libzahl_realloc(_: *mut zahl, _: size_t);
    fn zsub_unsigned(_: *mut zahl, _: *mut zahl, _: *mut zahl);
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
#[inline]
unsafe extern "C" fn zsignum(mut a: *mut zahl) -> ::core::ffi::c_int {
    return (*a).sign;
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
unsafe extern "C" fn zadd_impl_4(
    mut a: *mut zahl,
    mut b: *mut zahl,
    mut c: *mut zahl,
    mut n: size_t,
) {
    let mut ac: *mut zahl_char_t = (*a).chars;
    let mut bc: *mut zahl_char_t = (*b).chars;
    let mut cc: *mut zahl_char_t = (*c).chars;
    let mut carry: zahl_char_t = 0 as zahl_char_t;
    let mut i: size_t = 0;
    i = 0 as size_t;
    loop {
        ac = ac.offset(4 as ::core::ffi::c_int as isize);
        bc = bc.offset(4 as ::core::ffi::c_int as isize);
        cc = cc.offset(4 as ::core::ffi::c_int as isize);
        i = (i as ::core::ffi::c_ulong).wrapping_add(4 as ::core::ffi::c_ulong) as size_t as size_t;
        if !(i <= n) {
            break;
        }
        asm!(
            "\n", "    addq $-1, {0}\n", "    movq -32({2}), {0}\n",
            "    adcq -32({3}), {0}\n", "    movq {0}, -32({1})\n",
            "    movq -24({2}), {0}\n", "    adcq -24({3}), {0}\n",
            "    movq {0}, -24({1})\n", "    movq -16({2}), {0}\n",
            "    adcq -16({3}), {0}\n", "    movq {0}, -16({1})\n",
            "    movq -8({2}), {0}\n", "    adcq -8({3}), {0}\n",
            "    movq {0}, -8({1})\n", "    movq $1, {0}\n", "    jc 1f\n",
            "    movq $0, {0}\n", " 1:\n", inlateout(reg) carry, inlateout(reg) ac,
            inlateout(reg) bc, inlateout(reg) cc, options(preserves_flags, att_syntax)
        );
    }
    match n & 3 as size_t {
        3 => {
            asm!(
                "\n", "    addq $-1, {0}\n", "    movq -32({2}), {0}\n",
                "    adcq -32({3}), {0}\n", "    movq {0}, -32({1})\n",
                "    movq -24({2}), {0}\n", "    adcq -24({3}), {0}\n",
                "    movq {0}, -24({1})\n", "    movq -16({2}), {0}\n",
                "    adcq -16({3}), {0}\n", "    movq {0}, -16({1})\n",
                "    movq $1, {0}\n", "    jc 1f\n", "    movq $0, {0}\n", " 1:\n",
                inlateout(reg) carry, inlateout(reg) ac, inlateout(reg) bc,
                inlateout(reg) cc, options(preserves_flags, att_syntax)
            );
        }
        2 => {
            asm!(
                "\n", "    addq $-1, {0}\n", "    movq -32({2}), {0}\n",
                "    adcq -32({3}), {0}\n", "    movq {0}, -32({1})\n",
                "    movq -24({2}), {0}\n", "    adcq -24({3}), {0}\n",
                "    movq {0}, -24({1})\n", "    movq $1, {0}\n", "    jc 1f\n",
                "    movq $0, {0}\n", " 1:\n", inlateout(reg) carry, inlateout(reg) ac,
                inlateout(reg) bc, inlateout(reg) cc, options(preserves_flags,
                att_syntax)
            );
        }
        1 => {
            asm!(
                "\n", "    addq $-1, {0}\n", "    movq -32({2}), {0}\n",
                "    adcq -32({3}), {0}\n", "    movq {0}, -32({1})\n",
                "    movq $1, {0}\n", "    jc 1f\n", "    movq $0, {0}\n", " 1:\n",
                inlateout(reg) carry, inlateout(reg) ac, inlateout(reg) bc,
                inlateout(reg) cc, options(preserves_flags, att_syntax)
            );
        }
        _ => {}
    }
    i = n;
    while carry != 0 {
        let (fresh0, fresh1) = (*(*a).chars.offset(i as isize))
            .overflowing_add(1 as ::core::ffi::c_int as ::core::ffi::c_ulong);
        *(*a).chars.offset(i as isize) = fresh0;
        carry = fresh1 as zahl_char_t;
        i = i.wrapping_add(1);
    }
    if (*a).used < i {
        (*a).used = i;
    }
}
#[inline]
unsafe extern "C" fn zadd_impl_3(mut a: *mut zahl, mut b: *mut zahl, mut n: size_t) {
    let mut ac: *mut zahl_char_t = (*a).chars;
    let mut bc: *mut zahl_char_t = (*b).chars;
    let mut carry: zahl_char_t = 0 as zahl_char_t;
    let mut i: size_t = 0;
    i = 0 as size_t;
    loop {
        ac = ac.offset(4 as ::core::ffi::c_int as isize);
        bc = bc.offset(4 as ::core::ffi::c_int as isize);
        i = (i as ::core::ffi::c_ulong).wrapping_add(4 as ::core::ffi::c_ulong) as size_t as size_t;
        if !(i <= n) {
            break;
        }
        asm!(
            "\n", "    addq $-1, {0}\n", "    movq -32({2}), {0}\n",
            "    adcq {0}, -32({1})\n", "    movq -24({2}), {0}\n",
            "    adcq {0}, -24({1})\n", "    movq -16({2}), {0}\n",
            "    adcq {0}, -16({1})\n", "    movq -8({2}), {0}\n",
            "    adcq {0}, -8({1})\n", "    movq $1, {0}\n", "    jc 1f\n",
            "    movq $0, {0}\n", " 1:\n", inlateout(reg) carry, inlateout(reg) ac,
            inlateout(reg) bc, options(preserves_flags, att_syntax)
        );
    }
    match n & 3 as size_t {
        3 => {
            asm!(
                "\n", "    addq $-1, {0}\n", "    movq -32({2}), {0}\n",
                "    adcq {0}, -32({1})\n", "    movq -24({2}), {0}\n",
                "    adcq {0}, -24({1})\n", "    movq -16({2}), {0}\n",
                "    adcq {0}, -16({1})\n", "    movq $1, {0}\n", "    jc 1f\n",
                "    movq $0, {0}\n", " 1:\n", inlateout(reg) carry, inlateout(reg) ac,
                inlateout(reg) bc, options(preserves_flags, att_syntax)
            );
        }
        2 => {
            asm!(
                "\n", "    addq $-1, {0}\n", "    movq -32({2}), {0}\n",
                "    adcq {0}, -32({1})\n", "    movq -24({2}), {0}\n",
                "    adcq {0}, -24({1})\n", "    movq $1, {0}\n", "    jc 1f\n",
                "    movq $0, {0}\n", " 1:\n", inlateout(reg) carry, inlateout(reg) ac,
                inlateout(reg) bc, options(preserves_flags, att_syntax)
            );
        }
        1 => {
            asm!(
                "\n", "    addq $-1, {0}\n", "    movq -32({2}), {0}\n",
                "    adcq {0}, -32({1})\n", "    movq $1, {0}\n", "    jc 1f\n",
                "    movq $0, {0}\n", " 1:\n", inlateout(reg) carry, inlateout(reg) ac,
                inlateout(reg) bc, options(preserves_flags, att_syntax)
            );
        }
        _ => {}
    }
    i = n;
    while carry != 0 {
        let (fresh2, fresh3) = (*(*a).chars.offset(i as isize))
            .overflowing_add(1 as ::core::ffi::c_int as ::core::ffi::c_ulong);
        *(*a).chars.offset(i as isize) = fresh2;
        carry = fresh3 as zahl_char_t;
        i = i.wrapping_add(1);
    }
    if (*a).used < i {
        (*a).used = i;
    }
}
#[inline]
unsafe extern "C" fn libzahl_zadd_unsigned(mut a: *mut zahl, mut b: *mut zahl, mut c: *mut zahl) {
    let mut size: size_t = 0;
    let mut n: size_t = 0;
    if (zzero(b) != 0) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        zabs(a, c);
        return;
    } else if (zzero(c) != 0) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        zabs(a, b);
        return;
    }
    size = if (*b).used > (*c).used {
        (*b).used
    } else {
        (*c).used
    };
    n = (*b).used.wrapping_add((*c).used).wrapping_sub(size);
    if (*a).alloced < size.wrapping_add(1 as size_t) {
        libzahl_realloc(a as *mut zahl, size.wrapping_add(1 as size_t));
    }
    *(*a).chars.offset(size as isize) = 0 as zahl_char_t;
    if a == b {
        if (*a).used < (*c).used {
            n = (*c).used;
            libzahl_memset(
                (*a).chars.offset((*a).used as isize),
                0 as zahl_char_t,
                n.wrapping_sub((*a).used),
            );
        }
        zadd_impl_3(a, c, n);
    } else if (a == c) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        if (*a).used < (*b).used {
            n = (*b).used;
            libzahl_memset(
                (*a).chars.offset((*a).used as isize),
                0 as zahl_char_t,
                n.wrapping_sub((*a).used),
            );
        }
        zadd_impl_3(a, b, n);
    } else if ((*b).used > (*c).used) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        libzahl_memcpy(
            (*a).chars.offset(n as isize),
            (*b).chars.offset(n as isize),
            size.wrapping_sub(n),
        );
        (*a).used = size;
        zadd_impl_4(a, b, c, n);
    } else {
        libzahl_memcpy(
            (*a).chars.offset(n as isize),
            (*c).chars.offset(n as isize),
            size.wrapping_sub(n),
        );
        (*a).used = size;
        zadd_impl_4(a, b, c, n);
    }
    (*a).sign = 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn zadd_unsigned(mut a: *mut zahl, mut b: *mut zahl, mut c: *mut zahl) {
    libzahl_zadd_unsigned(a, b, c);
}
#[no_mangle]
pub unsafe extern "C" fn zadd_unsigned_assign(mut a: *mut zahl, mut b: *mut zahl) {
    let mut size: size_t = 0;
    let mut n: size_t = 0;
    if (zzero(a) != 0) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        zabs(a, b);
        return;
    } else if (zzero(b) != 0) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        return;
    }
    size = if (*a).used > (*b).used {
        (*a).used
    } else {
        (*b).used
    };
    n = (*a).used.wrapping_add((*b).used).wrapping_sub(size);
    if (*a).alloced < size.wrapping_add(1 as size_t) {
        libzahl_realloc(a as *mut zahl, size.wrapping_add(1 as size_t));
    }
    *(*a).chars.offset(size as isize) = 0 as zahl_char_t;
    if (*a).used < (*b).used {
        n = (*b).used;
        libzahl_memset(
            (*a).chars.offset((*a).used as isize),
            0 as zahl_char_t,
            n.wrapping_sub((*a).used),
        );
    }
    zadd_impl_3(a, b, n);
    (*a).sign = 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn zadd(mut a: *mut zahl, mut b: *mut zahl, mut c: *mut zahl) {
    if (zzero(b) != 0) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        if a != c {
            zset(a, c);
        }
    } else if (zzero(c) != 0) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        if a != b {
            zset(a, b);
        }
    } else if (zsignum(b) < 0 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long
        != 0
    {
        if zsignum(c) < 0 as ::core::ffi::c_int {
            libzahl_zadd_unsigned(a, b, c);
            (*a).sign = -zsignum(a);
        } else {
            zsub_unsigned(a, c, b);
        }
    } else if (zsignum(c) < 0 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long
        != 0
    {
        zsub_unsigned(a, b, c);
    } else {
        libzahl_zadd_unsigned(a, b, c);
    };
}
