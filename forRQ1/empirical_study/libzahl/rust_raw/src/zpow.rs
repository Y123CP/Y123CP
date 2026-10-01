use ::core::arch::asm;
extern "C" {
    fn longjmp(__env: *mut __jmp_buf_tag, __val: ::core::ffi::c_int) -> !;
    fn libzahl_realloc(_: *mut zahl, _: size_t);
    fn zfree(_: *mut zahl);
    fn zmul_ll(_: *mut zahl, _: *mut zahl, _: *mut zahl);
    fn zsqr_ll(_: *mut zahl, _: *mut zahl);
    fn free(__ptr: *mut ::core::ffi::c_void);
    static mut libzahl_tmp_pow_b: z_t;
    static mut libzahl_tmp_pow_c: z_t;
    static mut libzahl_jmp_buf: jmp_buf;
    static mut libzahl_error: ::core::ffi::c_int;
    static mut libzahl_temp_stack: *mut *mut zahl;
    static mut libzahl_temp_stack_head: *mut *mut zahl;
    static mut libzahl_temp_allocation: *mut ::core::ffi::c_void;
}
pub type __jmp_buf = [::core::ffi::c_long; 8];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [::core::ffi::c_ulong; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __jmp_buf_tag {
    pub __jmpbuf: __jmp_buf,
    pub __mask_was_saved: ::core::ffi::c_int,
    pub __saved_mask: __sigset_t,
}
pub type jmp_buf = [__jmp_buf_tag; 1];
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
pub type zerror = ::core::ffi::c_uint;
pub const ZERROR_INVALID_RADIX: zerror = 5;
pub const ZERROR_NEGATIVE: zerror = 4;
pub const ZERROR_DIV_0: zerror = 3;
pub const ZERROR_0_DIV_0: zerror = 2;
pub const ZERROR_0_POW_0: zerror = 1;
pub const ZERROR_ERRNO_SET: zerror = 0;
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
unsafe extern "C" fn zodd(mut a: *mut zahl) -> ::core::ffi::c_int {
    return ((*a).sign != 0
        && *(*a).chars.offset(0 as ::core::ffi::c_int as isize) & 1 as zahl_char_t != 0)
        as ::core::ffi::c_int;
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
unsafe extern "C" fn zsetu(mut a: *mut zahl, mut b: uint64_t) {
    if (b == 0) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        (*a).sign = 0 as ::core::ffi::c_int;
        return;
    }
    if (*a).alloced < 1 as size_t {
        libzahl_realloc(a as *mut zahl, 1 as size_t);
    }
    (*a).sign = 1 as ::core::ffi::c_int;
    *(*a).chars.offset(0 as ::core::ffi::c_int as isize) = b;
    (*a).used = 1 as size_t;
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
pub const BITS_PER_CHAR: ::core::ffi::c_int = ZAHL_BITS_PER_CHAR;
unsafe extern "C" fn libzahl_failure(mut error: ::core::ffi::c_int) {
    libzahl_error = error;
    if !libzahl_temp_stack.is_null() {
        while libzahl_temp_stack_head != libzahl_temp_stack {
            libzahl_temp_stack_head = libzahl_temp_stack_head.offset(-1);
            zfree(*libzahl_temp_stack_head);
        }
    }
    free(libzahl_temp_allocation);
    libzahl_temp_allocation = ::core::ptr::null_mut::<::core::ffi::c_void>();
    longjmp(
        &raw mut libzahl_jmp_buf as *mut __jmp_buf_tag,
        1 as ::core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn zpow(mut a: *mut zahl, mut b: *mut zahl, mut c: *mut zahl) {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut n: size_t = 0;
    let mut bits: size_t = 0;
    let mut x: zahl_char_t = 0;
    let mut neg: ::core::ffi::c_int = 0;
    if (zsignum(c) <= 0 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        if zzero(c) != 0 {
            if (zzero(b) != 0) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
                libzahl_failure(-(ZERROR_0_POW_0 as ::core::ffi::c_int));
            }
            zsetu(a, 1 as uint64_t);
        } else if (zzero(b) != 0) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
            libzahl_failure(-(ZERROR_DIV_0 as ::core::ffi::c_int));
        } else {
            (*a).sign = 0 as ::core::ffi::c_int;
        }
        return;
    } else if (zzero(b) != 0) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        (*a).sign = 0 as ::core::ffi::c_int;
        return;
    }
    bits = zbits(c);
    n = bits >> ZAHL_LB_BITS_PER_CHAR;
    neg = (zsignum(b) < 0 as ::core::ffi::c_int && zodd(c) != 0) as ::core::ffi::c_int;
    zabs(&raw mut libzahl_tmp_pow_b as *mut zahl, b);
    zset(&raw mut libzahl_tmp_pow_c as *mut zahl, c);
    zsetu(a, 1 as uint64_t);
    i = 0 as size_t;
    while i < n {
        x = *(*(&raw mut libzahl_tmp_pow_c as *mut zahl))
            .chars
            .offset(i as isize);
        j = BITS_PER_CHAR as size_t;
        loop {
            let fresh0 = j;
            j = j.wrapping_sub(1);
            if !(fresh0 != 0) {
                break;
            }
            if x & 1 as zahl_char_t != 0 {
                zmul_ll(a, a, &raw mut libzahl_tmp_pow_b as *mut zahl);
            }
            zsqr_ll(
                &raw mut libzahl_tmp_pow_b as *mut zahl,
                &raw mut libzahl_tmp_pow_b as *mut zahl,
            );
            x >>= 1 as ::core::ffi::c_int;
        }
        i = i.wrapping_add(1);
    }
    x = *(*(&raw mut libzahl_tmp_pow_c as *mut zahl))
        .chars
        .offset(i as isize);
    while x != 0 {
        if x & 1 as zahl_char_t != 0 {
            zmul_ll(a, a, &raw mut libzahl_tmp_pow_b as *mut zahl);
        }
        zsqr_ll(
            &raw mut libzahl_tmp_pow_b as *mut zahl,
            &raw mut libzahl_tmp_pow_b as *mut zahl,
        );
        x >>= 1 as ::core::ffi::c_int;
    }
    if neg != 0 {
        zneg(a, a);
    }
}
