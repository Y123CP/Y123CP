use ::core::arch::asm;
extern "C" {
    fn libzahl_realloc(_: *mut zahl, _: size_t);
}
pub type size_t = usize;
pub type __uint64_t = u64;
pub type __ssize_t = ::core::ffi::c_long;
pub type uint64_t = __uint64_t;
pub type ssize_t = __ssize_t;
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
unsafe extern "C" fn libzahl_memset_precise(
    mut a: *mut zahl_char_t,
    mut v: zahl_char_t,
    mut n: size_t,
) {
    let mut i: size_t = 0;
    if n <= 4 as size_t {
        if n >= 1 as size_t {
            *a.offset(0 as ::core::ffi::c_int as isize) = v;
        }
        if n >= 2 as size_t {
            *a.offset(1 as ::core::ffi::c_int as isize) = v;
        }
        if n >= 3 as size_t {
            *a.offset(2 as ::core::ffi::c_int as isize) = v;
        }
        if n >= 4 as size_t {
            *a.offset(3 as ::core::ffi::c_int as isize) = v;
        }
    } else {
        i = 0 as size_t;
        loop {
            i = (i as ::core::ffi::c_ulong).wrapping_add(4 as ::core::ffi::c_ulong) as size_t
                as size_t;
            if !(i <= n) {
                break;
            }
            *a.offset(i.wrapping_sub(1 as size_t) as isize) = v;
            *a.offset(i.wrapping_sub(2 as size_t) as isize) = v;
            *a.offset(i.wrapping_sub(3 as size_t) as isize) = v;
            *a.offset(i.wrapping_sub(4 as size_t) as isize) = v;
        }
        if i > n {
            i = (i as ::core::ffi::c_ulong).wrapping_sub(4 as ::core::ffi::c_ulong) as size_t
                as size_t;
            while i < n {
                *a.offset(i as isize) = v;
                i = i.wrapping_add(1);
            }
        }
    };
}
#[inline]
unsafe extern "C" fn libzahl_memmoveb(
    mut d: *mut zahl_char_t,
    mut s: *const zahl_char_t,
    mut n: size_t,
) {
    let mut i: ssize_t = 0;
    let mut current_block_47: u64;
    match n {
        20 => {
            *d.offset((20 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize) =
                *s.offset((20 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize);
            current_block_47 = 17705359978624174692;
        }
        19 => {
            current_block_47 = 17705359978624174692;
        }
        18 => {
            current_block_47 = 2310515158819956057;
        }
        17 => {
            current_block_47 = 43976425737172999;
        }
        16 => {
            current_block_47 = 9417284841268954486;
        }
        15 => {
            current_block_47 = 4101696738521056335;
        }
        14 => {
            current_block_47 = 6792483586390751536;
        }
        13 => {
            current_block_47 = 1792514854997551161;
        }
        12 => {
            current_block_47 = 6404334001140195680;
        }
        11 => {
            current_block_47 = 7114361717014759348;
        }
        10 => {
            current_block_47 = 12010700182439148840;
        }
        9 => {
            current_block_47 = 710957976789303753;
        }
        8 => {
            current_block_47 = 776449361283660306;
        }
        7 => {
            current_block_47 = 9278403117376987587;
        }
        6 => {
            current_block_47 = 3184704105973856247;
        }
        5 => {
            current_block_47 = 4803163523842464808;
        }
        4 => {
            current_block_47 = 6558758903416202777;
        }
        3 => {
            current_block_47 = 1708148710342859856;
        }
        2 => {
            current_block_47 = 17295929920418635907;
        }
        1 => {
            current_block_47 = 14710078419898575087;
        }
        0 => {
            current_block_47 = 7226443171521532240;
        }
        _ => {
            i = (n as ::core::ffi::c_long + 3 as ::core::ffi::c_long
                & !(3 as ::core::ffi::c_int) as ::core::ffi::c_long) as ssize_t;
            loop {
                i -= 4 as ::core::ffi::c_long;
                if !(i >= 0 as ::core::ffi::c_long) {
                    break;
                }
                *d.offset((i as ::core::ffi::c_long + 3 as ::core::ffi::c_long) as isize) =
                    *s.offset((i as ::core::ffi::c_long + 3 as ::core::ffi::c_long) as isize);
                *d.offset((i as ::core::ffi::c_long + 2 as ::core::ffi::c_long) as isize) =
                    *s.offset((i as ::core::ffi::c_long + 2 as ::core::ffi::c_long) as isize);
                *d.offset((i as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize) =
                    *s.offset((i as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize);
                *d.offset((i as ::core::ffi::c_long + 0 as ::core::ffi::c_long) as isize) =
                    *s.offset((i as ::core::ffi::c_long + 0 as ::core::ffi::c_long) as isize);
            }
            current_block_47 = 7226443171521532240;
        }
    }
    match current_block_47 {
        17705359978624174692 => {
            *d.offset((19 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize) =
                *s.offset((19 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize);
            current_block_47 = 2310515158819956057;
        }
        _ => {}
    }
    match current_block_47 {
        2310515158819956057 => {
            *d.offset((18 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize) =
                *s.offset((18 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize);
            current_block_47 = 43976425737172999;
        }
        _ => {}
    }
    match current_block_47 {
        43976425737172999 => {
            *d.offset((17 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize) =
                *s.offset((17 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize);
            current_block_47 = 9417284841268954486;
        }
        _ => {}
    }
    match current_block_47 {
        9417284841268954486 => {
            *d.offset((16 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize) =
                *s.offset((16 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize);
            current_block_47 = 4101696738521056335;
        }
        _ => {}
    }
    match current_block_47 {
        4101696738521056335 => {
            *d.offset((15 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize) =
                *s.offset((15 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize);
            current_block_47 = 6792483586390751536;
        }
        _ => {}
    }
    match current_block_47 {
        6792483586390751536 => {
            *d.offset((14 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize) =
                *s.offset((14 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize);
            current_block_47 = 1792514854997551161;
        }
        _ => {}
    }
    match current_block_47 {
        1792514854997551161 => {
            *d.offset((13 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize) =
                *s.offset((13 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize);
            current_block_47 = 6404334001140195680;
        }
        _ => {}
    }
    match current_block_47 {
        6404334001140195680 => {
            *d.offset((12 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize) =
                *s.offset((12 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize);
            current_block_47 = 7114361717014759348;
        }
        _ => {}
    }
    match current_block_47 {
        7114361717014759348 => {
            *d.offset((11 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize) =
                *s.offset((11 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize);
            current_block_47 = 12010700182439148840;
        }
        _ => {}
    }
    match current_block_47 {
        12010700182439148840 => {
            *d.offset((10 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize) =
                *s.offset((10 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize);
            current_block_47 = 710957976789303753;
        }
        _ => {}
    }
    match current_block_47 {
        710957976789303753 => {
            *d.offset((9 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize) =
                *s.offset((9 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize);
            current_block_47 = 776449361283660306;
        }
        _ => {}
    }
    match current_block_47 {
        776449361283660306 => {
            *d.offset((8 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize) =
                *s.offset((8 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize);
            current_block_47 = 9278403117376987587;
        }
        _ => {}
    }
    match current_block_47 {
        9278403117376987587 => {
            *d.offset((7 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize) =
                *s.offset((7 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize);
            current_block_47 = 3184704105973856247;
        }
        _ => {}
    }
    match current_block_47 {
        3184704105973856247 => {
            *d.offset((6 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize) =
                *s.offset((6 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize);
            current_block_47 = 4803163523842464808;
        }
        _ => {}
    }
    match current_block_47 {
        4803163523842464808 => {
            *d.offset((5 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize) =
                *s.offset((5 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize);
            current_block_47 = 6558758903416202777;
        }
        _ => {}
    }
    match current_block_47 {
        6558758903416202777 => {
            *d.offset((4 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize) =
                *s.offset((4 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize);
            current_block_47 = 1708148710342859856;
        }
        _ => {}
    }
    match current_block_47 {
        1708148710342859856 => {
            *d.offset((3 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize) =
                *s.offset((3 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize);
            current_block_47 = 17295929920418635907;
        }
        _ => {}
    }
    match current_block_47 {
        17295929920418635907 => {
            *d.offset((2 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize) =
                *s.offset((2 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize);
            current_block_47 = 14710078419898575087;
        }
        _ => {}
    }
    match current_block_47 {
        14710078419898575087 => {
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
pub const BITS_PER_CHAR: ::core::ffi::c_int = ZAHL_BITS_PER_CHAR;
#[no_mangle]
pub unsafe extern "C" fn zlsh(mut a: *mut zahl, mut b: *mut zahl, mut bits: size_t) {
    let mut i: size_t = 0;
    let mut chars: size_t = 0;
    let mut cbits: size_t = 0;
    let mut carry: zahl_char_t = 0 as zahl_char_t;
    let mut tcarry: zahl_char_t = 0;
    if (zzero(b) != 0) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        (*a).sign = 0 as ::core::ffi::c_int;
        return;
    }
    chars = bits >> ZAHL_LB_BITS_PER_CHAR;
    bits = bits & (ZAHL_BITS_PER_CHAR - 1 as ::core::ffi::c_int) as size_t;
    cbits = (BITS_PER_CHAR as size_t).wrapping_sub(bits);
    if (*a).alloced < (*b).used.wrapping_add(chars).wrapping_add(1 as size_t) {
        libzahl_realloc(
            a as *mut zahl,
            (*b).used.wrapping_add(chars).wrapping_add(1 as size_t),
        );
    }
    if (a == b) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        libzahl_memmoveb((*a).chars.offset(chars as isize), (*b).chars, (*b).used);
    } else {
        libzahl_memcpy((*a).chars.offset(chars as isize), (*b).chars, (*b).used);
    }
    libzahl_memset_precise((*a).chars, 0 as zahl_char_t, chars);
    (*a).used = (*b).used.wrapping_add(chars);
    if (bits != 0) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        i = chars;
        while i < (*a).used {
            tcarry = *(*a).chars.offset(i as isize) >> cbits;
            *(*a).chars.offset(i as isize) <<= bits;
            let ref mut fresh0 = *(*a).chars.offset(i as isize);
            *fresh0 =
                (*fresh0 as ::core::ffi::c_ulong | carry as ::core::ffi::c_ulong) as zahl_char_t;
            carry = tcarry;
            i = i.wrapping_add(1);
        }
        if carry != 0 {
            let fresh1 = (*a).used;
            (*a).used = (*a).used.wrapping_add(1);
            *(*a).chars.offset(fresh1 as isize) = carry;
        }
    }
    (*a).sign = zsignum(b);
}
