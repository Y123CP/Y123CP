use ::core::arch::asm;
extern "C" {
    fn longjmp(__env: *mut __jmp_buf_tag, __val: ::core::ffi::c_int) -> !;
    fn libzahl_realloc(_: *mut zahl, _: size_t);
    fn zfree(_: *mut zahl);
    fn zdivmod(_: *mut zahl, _: *mut zahl, _: *mut zahl, _: *mut zahl);
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memmove(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    static mut libzahl_tmp_str_rem: z_t;
    static mut libzahl_tmp_str_num: z_t;
    static mut libzahl_const_1e19: z_t;
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
pub type __uint16_t = u16;
pub type __uint64_t = u64;
pub type uint16_t = __uint16_t;
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
pub const ZAHL_BITS_PER_CHAR: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
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
pub const ENOENT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
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
#[inline]
unsafe extern "C" fn libzahl_memfailure() {
    if *__errno_location() == 0 {
        *__errno_location() = ENOENT;
    }
    libzahl_failure(*__errno_location());
}
#[inline]
unsafe extern "C" fn sprintint_fix(mut buf: *mut ::core::ffi::c_char, mut v: zahl_char_t) {
    let mut partials: *const ::core::ffi::c_char = b"00010203040506070809101112131415161718192021222324252627282930313233343536373839404142434445464748495051525354555657585960616263646566676869707172737475767778798081828384858687888990919293949596979899\0"
        as *const u8 as *const ::core::ffi::c_char;
    let mut buffer: *mut uint16_t = buf.offset(1 as ::core::ffi::c_int as isize) as *mut uint16_t;
    *buffer.offset(8 as ::core::ffi::c_int as isize) = *(partials
        .offset((2 as zahl_char_t).wrapping_mul(v.wrapping_rem(100 as zahl_char_t)) as isize)
        as *const uint16_t);
    v = (v as ::core::ffi::c_ulong).wrapping_div(100 as ::core::ffi::c_ulong) as zahl_char_t
        as zahl_char_t;
    *buffer.offset(7 as ::core::ffi::c_int as isize) = *(partials
        .offset((2 as zahl_char_t).wrapping_mul(v.wrapping_rem(100 as zahl_char_t)) as isize)
        as *const uint16_t);
    v = (v as ::core::ffi::c_ulong).wrapping_div(100 as ::core::ffi::c_ulong) as zahl_char_t
        as zahl_char_t;
    *buffer.offset(6 as ::core::ffi::c_int as isize) = *(partials
        .offset((2 as zahl_char_t).wrapping_mul(v.wrapping_rem(100 as zahl_char_t)) as isize)
        as *const uint16_t);
    v = (v as ::core::ffi::c_ulong).wrapping_div(100 as ::core::ffi::c_ulong) as zahl_char_t
        as zahl_char_t;
    *buffer.offset(5 as ::core::ffi::c_int as isize) = *(partials
        .offset((2 as zahl_char_t).wrapping_mul(v.wrapping_rem(100 as zahl_char_t)) as isize)
        as *const uint16_t);
    v = (v as ::core::ffi::c_ulong).wrapping_div(100 as ::core::ffi::c_ulong) as zahl_char_t
        as zahl_char_t;
    *buffer.offset(4 as ::core::ffi::c_int as isize) = *(partials
        .offset((2 as zahl_char_t).wrapping_mul(v.wrapping_rem(100 as zahl_char_t)) as isize)
        as *const uint16_t);
    v = (v as ::core::ffi::c_ulong).wrapping_div(100 as ::core::ffi::c_ulong) as zahl_char_t
        as zahl_char_t;
    *buffer.offset(3 as ::core::ffi::c_int as isize) = *(partials
        .offset((2 as zahl_char_t).wrapping_mul(v.wrapping_rem(100 as zahl_char_t)) as isize)
        as *const uint16_t);
    v = (v as ::core::ffi::c_ulong).wrapping_div(100 as ::core::ffi::c_ulong) as zahl_char_t
        as zahl_char_t;
    *buffer.offset(2 as ::core::ffi::c_int as isize) = *(partials
        .offset((2 as zahl_char_t).wrapping_mul(v.wrapping_rem(100 as zahl_char_t)) as isize)
        as *const uint16_t);
    v = (v as ::core::ffi::c_ulong).wrapping_div(100 as ::core::ffi::c_ulong) as zahl_char_t
        as zahl_char_t;
    *buffer.offset(1 as ::core::ffi::c_int as isize) = *(partials
        .offset((2 as zahl_char_t).wrapping_mul(v.wrapping_rem(100 as zahl_char_t)) as isize)
        as *const uint16_t);
    v = (v as ::core::ffi::c_ulong).wrapping_div(100 as ::core::ffi::c_ulong) as zahl_char_t
        as zahl_char_t;
    *buffer.offset(0 as ::core::ffi::c_int as isize) = *(partials
        .offset((2 as zahl_char_t).wrapping_mul(v.wrapping_rem(100 as zahl_char_t)) as isize)
        as *const uint16_t);
    v = (v as ::core::ffi::c_ulong).wrapping_div(100 as ::core::ffi::c_ulong) as zahl_char_t
        as zahl_char_t;
    *buf = ('0' as i32 as zahl_char_t).wrapping_add(v) as ::core::ffi::c_char;
    *buf.offset(19 as ::core::ffi::c_int as isize) = 0 as ::core::ffi::c_char;
}
#[inline]
unsafe extern "C" fn cmemmove(
    mut d: *mut ::core::ffi::c_char,
    mut s: *const ::core::ffi::c_char,
    mut n: ::core::ffi::c_long,
) {
    loop {
        let fresh0 = n;
        n = n - 1;
        if !(fresh0 != 0) {
            break;
        }
        let fresh1 = s;
        s = s.offset(1);
        let fresh2 = d;
        d = d.offset(1);
        *fresh2 = *fresh1;
    }
}
#[inline]
unsafe extern "C" fn sprintint_min(
    mut buf: *mut ::core::ffi::c_char,
    mut v: zahl_char_t,
) -> size_t {
    let mut i: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
    let mut j: ::core::ffi::c_long = 0;
    sprintint_fix(buf, v);
    while *buf.offset(i as isize) as ::core::ffi::c_int == '0' as i32 {
        i += 1;
    }
    j = 19 as ::core::ffi::c_long - i;
    cmemmove(buf, buf.offset(i as isize), j);
    *buf.offset(j as isize) = 0 as ::core::ffi::c_char;
    return j as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn zstr(
    mut a: *mut zahl,
    mut b: *mut ::core::ffi::c_char,
    mut n: size_t,
) -> *mut ::core::ffi::c_char {
    let mut buf: [::core::ffi::c_char; 20] = [0; 20];
    let mut len: size_t = 0;
    let mut neg: size_t = 0;
    let mut last: size_t = 0;
    let mut tot: size_t = 0 as size_t;
    let mut overridden: ::core::ffi::c_char = 0 as ::core::ffi::c_char;
    if (zzero(a) != 0) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        if b.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 && {
            b = malloc(2 as size_t) as *mut ::core::ffi::c_char;
            b.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0
        } {
            libzahl_memfailure();
        }
        *b.offset(0 as ::core::ffi::c_int as isize) = '0' as i32 as ::core::ffi::c_char;
        *b.offset(1 as ::core::ffi::c_int as isize) = 0 as ::core::ffi::c_char;
        return b;
    }
    if n == 0 {
        n = ((20 as ::core::ffi::c_int * BITS_PER_CHAR / 64 as ::core::ffi::c_int
            + (BITS_PER_CHAR == 8 as ::core::ffi::c_int) as ::core::ffi::c_int)
            as size_t)
            .wrapping_mul((*a).used);
    }
    if b.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 && {
        libzahl_temp_allocation = malloc(n.wrapping_add(1 as size_t));
        b = libzahl_temp_allocation as *mut ::core::ffi::c_char;
        b.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0
    } {
        libzahl_memfailure();
    }
    neg = (zsignum(a) < 0 as ::core::ffi::c_int) as ::core::ffi::c_int as size_t;
    zabs(&raw mut libzahl_tmp_str_num as *mut zahl, a);
    *b.offset(0 as ::core::ffi::c_int as isize) = '-' as i32 as ::core::ffi::c_char;
    b = b.offset(neg as isize);
    n = (n as ::core::ffi::c_ulong).wrapping_sub(neg as ::core::ffi::c_ulong) as size_t as size_t;
    last = n;
    n = if last > 19 as size_t {
        n.wrapping_sub(19 as size_t)
    } else {
        0 as size_t
    };
    loop {
        zdivmod(
            &raw mut libzahl_tmp_str_num as *mut zahl,
            &raw mut libzahl_tmp_str_rem as *mut zahl,
            &raw mut libzahl_tmp_str_num as *mut zahl,
            &raw mut libzahl_const_1e19 as *mut zahl,
        );
        if (zzero(&raw mut libzahl_tmp_str_num as *mut zahl) == 0) as ::core::ffi::c_int
            as ::core::ffi::c_long
            != 0
        {
            sprintint_fix(
                b.offset(n as isize),
                if zzero(&raw mut libzahl_tmp_str_rem as *mut zahl) != 0 {
                    0 as zahl_char_t
                } else {
                    *(*(&raw mut libzahl_tmp_str_rem as *mut zahl))
                        .chars
                        .offset(0 as ::core::ffi::c_int as isize)
                },
            );
            *b.offset(n.wrapping_add(19 as size_t) as isize) = overridden;
            overridden = *b.offset(n as isize);
            last = n;
            n = if last > 19 as size_t {
                n.wrapping_sub(19 as size_t)
            } else {
                0 as size_t
            };
            tot = (tot as ::core::ffi::c_ulong).wrapping_add(19 as ::core::ffi::c_ulong) as size_t
                as size_t;
        } else {
            len = sprintint_min(
                &raw mut buf as *mut ::core::ffi::c_char,
                *(*(&raw mut libzahl_tmp_str_rem as *mut zahl))
                    .chars
                    .offset(0 as ::core::ffi::c_int as isize),
            );
            if tot != 0 {
                memcpy(
                    b as *mut ::core::ffi::c_void,
                    &raw mut buf as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
                    len,
                );
                memmove(
                    b.offset(len as isize) as *mut ::core::ffi::c_void,
                    b.offset(last as isize) as *const ::core::ffi::c_void,
                    tot.wrapping_add(1 as size_t),
                );
            } else {
                memcpy(
                    b as *mut ::core::ffi::c_void,
                    &raw mut buf as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
                    len.wrapping_add(1 as size_t),
                );
            }
            break;
        }
    }
    libzahl_temp_allocation = ::core::ptr::null_mut::<::core::ffi::c_void>();
    return b.offset(-(neg as isize));
}
