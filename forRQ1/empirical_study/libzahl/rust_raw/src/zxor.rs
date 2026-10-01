use ::core::arch::asm;
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
unsafe extern "C" fn zmemcpy_range(
    mut d: *mut zahl_char_t,
    mut s: *const zahl_char_t,
    mut i: size_t,
    mut n: size_t,
) {
    d = d.offset(i as isize);
    s = s.offset(i as isize);
    n = (n as ::core::ffi::c_ulong).wrapping_sub(i as ::core::ffi::c_ulong) as size_t as size_t;
    libzahl_memcpy(d, s, n);
}
#[no_mangle]
pub unsafe extern "C" fn zxor(mut a: *mut zahl, mut b: *mut zahl, mut c: *mut zahl) {
    let mut n: size_t = 0;
    let mut m: size_t = 0;
    let mut bn: size_t = 0;
    let mut cn: size_t = 0;
    let mut bc: *const zahl_char_t = ::core::ptr::null::<zahl_char_t>();
    let mut cc: *const zahl_char_t = ::core::ptr::null::<zahl_char_t>();
    if (zzero(b) != 0) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        if a != c {
            zset(a, c);
        }
        return;
    } else if (zzero(c) != 0) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        if a != b {
            zset(a, b);
        }
        return;
    }
    bn = (*b).used;
    bc = (*b).chars;
    cn = (*c).used;
    cc = (*c).chars;
    n = (if bn < cn { bn } else { cn });
    m = (if bn > cn { bn } else { cn });
    if (*a).alloced < m {
        libzahl_realloc(a as *mut zahl, m);
    }
    if a == b {
        let mut a__: *mut zahl_char_t = (*a).chars;
        let mut b__: *const zahl_char_t = (*a).chars;
        let mut c__: *const zahl_char_t = cc;
        let mut i__: size_t = 0;
        let mut n__: size_t = n;
        if n__ <= 4 as size_t {
            if n__ >= 1 as size_t {
                *a__.offset(0 as ::core::ffi::c_int as isize) = *b__
                    .offset(0 as ::core::ffi::c_int as isize)
                    ^ *c__.offset(0 as ::core::ffi::c_int as isize);
            }
            if n__ >= 2 as size_t {
                *a__.offset(1 as ::core::ffi::c_int as isize) = *b__
                    .offset(1 as ::core::ffi::c_int as isize)
                    ^ *c__.offset(1 as ::core::ffi::c_int as isize);
            }
            if n__ >= 3 as size_t {
                *a__.offset(2 as ::core::ffi::c_int as isize) = *b__
                    .offset(2 as ::core::ffi::c_int as isize)
                    ^ *c__.offset(2 as ::core::ffi::c_int as isize);
            }
            if n__ >= 4 as size_t {
                *a__.offset(3 as ::core::ffi::c_int as isize) = *b__
                    .offset(3 as ::core::ffi::c_int as isize)
                    ^ *c__.offset(3 as ::core::ffi::c_int as isize);
            }
        } else {
            i__ = 0 as size_t;
            loop {
                i__ = (i__ as ::core::ffi::c_ulong).wrapping_add(4 as ::core::ffi::c_ulong)
                    as size_t as size_t;
                if !(i__ < n__) {
                    break;
                }
                *a__.offset(i__.wrapping_sub(1 as size_t) as isize) = *b__
                    .offset(i__.wrapping_sub(1 as size_t) as isize)
                    ^ *c__.offset(i__.wrapping_sub(1 as size_t) as isize);
                *a__.offset(i__.wrapping_sub(2 as size_t) as isize) = *b__
                    .offset(i__.wrapping_sub(2 as size_t) as isize)
                    ^ *c__.offset(i__.wrapping_sub(2 as size_t) as isize);
                *a__.offset(i__.wrapping_sub(3 as size_t) as isize) = *b__
                    .offset(i__.wrapping_sub(3 as size_t) as isize)
                    ^ *c__.offset(i__.wrapping_sub(3 as size_t) as isize);
                *a__.offset(i__.wrapping_sub(4 as size_t) as isize) = *b__
                    .offset(i__.wrapping_sub(4 as size_t) as isize)
                    ^ *c__.offset(i__.wrapping_sub(4 as size_t) as isize);
            }
            if i__ > n__ {
                i__ = (i__ as ::core::ffi::c_ulong).wrapping_sub(4 as ::core::ffi::c_ulong)
                    as size_t as size_t;
                while i__ < n__ {
                    *a__.offset(i__ as isize) =
                        *b__.offset(i__ as isize) ^ *c__.offset(i__ as isize);
                    i__ = i__.wrapping_add(1);
                }
            }
        }
        if (*a).used < cn {
            zmemcpy_range((*a).chars, cc, n, m);
        }
    } else if (a == c) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        let mut a___0: *mut zahl_char_t = (*a).chars;
        let mut b___0: *const zahl_char_t = (*a).chars;
        let mut c___0: *const zahl_char_t = bc;
        let mut i___0: size_t = 0;
        let mut n___0: size_t = n;
        if n___0 <= 4 as size_t {
            if n___0 >= 1 as size_t {
                *a___0.offset(0 as ::core::ffi::c_int as isize) = *b___0
                    .offset(0 as ::core::ffi::c_int as isize)
                    ^ *c___0.offset(0 as ::core::ffi::c_int as isize);
            }
            if n___0 >= 2 as size_t {
                *a___0.offset(1 as ::core::ffi::c_int as isize) = *b___0
                    .offset(1 as ::core::ffi::c_int as isize)
                    ^ *c___0.offset(1 as ::core::ffi::c_int as isize);
            }
            if n___0 >= 3 as size_t {
                *a___0.offset(2 as ::core::ffi::c_int as isize) = *b___0
                    .offset(2 as ::core::ffi::c_int as isize)
                    ^ *c___0.offset(2 as ::core::ffi::c_int as isize);
            }
            if n___0 >= 4 as size_t {
                *a___0.offset(3 as ::core::ffi::c_int as isize) = *b___0
                    .offset(3 as ::core::ffi::c_int as isize)
                    ^ *c___0.offset(3 as ::core::ffi::c_int as isize);
            }
        } else {
            i___0 = 0 as size_t;
            loop {
                i___0 = (i___0 as ::core::ffi::c_ulong).wrapping_add(4 as ::core::ffi::c_ulong)
                    as size_t as size_t;
                if !(i___0 < n___0) {
                    break;
                }
                *a___0.offset(i___0.wrapping_sub(1 as size_t) as isize) = *b___0
                    .offset(i___0.wrapping_sub(1 as size_t) as isize)
                    ^ *c___0.offset(i___0.wrapping_sub(1 as size_t) as isize);
                *a___0.offset(i___0.wrapping_sub(2 as size_t) as isize) = *b___0
                    .offset(i___0.wrapping_sub(2 as size_t) as isize)
                    ^ *c___0.offset(i___0.wrapping_sub(2 as size_t) as isize);
                *a___0.offset(i___0.wrapping_sub(3 as size_t) as isize) = *b___0
                    .offset(i___0.wrapping_sub(3 as size_t) as isize)
                    ^ *c___0.offset(i___0.wrapping_sub(3 as size_t) as isize);
                *a___0.offset(i___0.wrapping_sub(4 as size_t) as isize) = *b___0
                    .offset(i___0.wrapping_sub(4 as size_t) as isize)
                    ^ *c___0.offset(i___0.wrapping_sub(4 as size_t) as isize);
            }
            if i___0 > n___0 {
                i___0 = (i___0 as ::core::ffi::c_ulong).wrapping_sub(4 as ::core::ffi::c_ulong)
                    as size_t as size_t;
                while i___0 < n___0 {
                    *a___0.offset(i___0 as isize) =
                        *b___0.offset(i___0 as isize) ^ *c___0.offset(i___0 as isize);
                    i___0 = i___0.wrapping_add(1);
                }
            }
        }
        if (*a).used < bn {
            zmemcpy_range((*a).chars, bc, n, m);
        }
    } else if m == bn {
        let mut a___1: *mut zahl_char_t = (*a).chars;
        let mut b___1: *const zahl_char_t = (*c).chars;
        let mut c___1: *const zahl_char_t = (*b).chars;
        let mut i___1: size_t = 0;
        let mut n___1: size_t = n;
        i___1 = 0 as size_t;
        while i___1 < n___1 {
            *a___1.offset(i___1.wrapping_add(0 as size_t) as isize) = *b___1
                .offset(i___1.wrapping_add(0 as size_t) as isize)
                ^ *c___1.offset(i___1.wrapping_add(0 as size_t) as isize);
            *a___1.offset(i___1.wrapping_add(1 as size_t) as isize) = *b___1
                .offset(i___1.wrapping_add(1 as size_t) as isize)
                ^ *c___1.offset(i___1.wrapping_add(1 as size_t) as isize);
            *a___1.offset(i___1.wrapping_add(2 as size_t) as isize) = *b___1
                .offset(i___1.wrapping_add(2 as size_t) as isize)
                ^ *c___1.offset(i___1.wrapping_add(2 as size_t) as isize);
            *a___1.offset(i___1.wrapping_add(3 as size_t) as isize) = *b___1
                .offset(i___1.wrapping_add(3 as size_t) as isize)
                ^ *c___1.offset(i___1.wrapping_add(3 as size_t) as isize);
            i___1 = (i___1 as ::core::ffi::c_ulong).wrapping_add(4 as ::core::ffi::c_ulong)
                as size_t as size_t;
        }
        zmemcpy_range((*a).chars, (*b).chars, n, m);
    } else {
        let mut a___2: *mut zahl_char_t = (*a).chars;
        let mut b___2: *const zahl_char_t = (*b).chars;
        let mut c___2: *const zahl_char_t = (*c).chars;
        let mut i___2: size_t = 0;
        let mut n___2: size_t = n;
        i___2 = 0 as size_t;
        while i___2 < n___2 {
            *a___2.offset(i___2.wrapping_add(0 as size_t) as isize) = *b___2
                .offset(i___2.wrapping_add(0 as size_t) as isize)
                ^ *c___2.offset(i___2.wrapping_add(0 as size_t) as isize);
            *a___2.offset(i___2.wrapping_add(1 as size_t) as isize) = *b___2
                .offset(i___2.wrapping_add(1 as size_t) as isize)
                ^ *c___2.offset(i___2.wrapping_add(1 as size_t) as isize);
            *a___2.offset(i___2.wrapping_add(2 as size_t) as isize) = *b___2
                .offset(i___2.wrapping_add(2 as size_t) as isize)
                ^ *c___2.offset(i___2.wrapping_add(2 as size_t) as isize);
            *a___2.offset(i___2.wrapping_add(3 as size_t) as isize) = *b___2
                .offset(i___2.wrapping_add(3 as size_t) as isize)
                ^ *c___2.offset(i___2.wrapping_add(3 as size_t) as isize);
            i___2 = (i___2 as ::core::ffi::c_ulong).wrapping_add(4 as ::core::ffi::c_ulong)
                as size_t as size_t;
        }
        zmemcpy_range((*a).chars, (*c).chars, n, m);
    }
    (*a).used = m;
    while (*a).used != 0
        && *(*a)
            .chars
            .offset((*a).used.wrapping_sub(1 as size_t) as isize)
            == 0
    {
        (*a).used = (*a).used.wrapping_sub(1);
    }
    (*a).sign = if (*a).used != 0 {
        1 as ::core::ffi::c_int
            - 2 as ::core::ffi::c_int
                * (zsignum(b) ^ zsignum(c) < 0 as ::core::ffi::c_int) as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    };
}
