extern "C" {
    fn longjmp(__env: *mut __jmp_buf_tag, __val: ::core::ffi::c_int) -> !;
    fn libzahl_realloc(_: *mut zahl, _: size_t);
    fn zfree(_: *mut zahl);
    fn zadd_unsigned_assign(_: *mut zahl, _: *mut zahl);
    fn zlsh(_: *mut zahl, _: *mut zahl, _: size_t);
    fn zmul_ll(_: *mut zahl, _: *mut zahl, _: *mut zahl);
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn realloc(__ptr: *mut ::core::ffi::c_void, __size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    static mut libzahl_jmp_buf: jmp_buf;
    static mut libzahl_error: ::core::ffi::c_int;
    static mut libzahl_temp_stack: *mut *mut zahl;
    static mut libzahl_temp_stack_head: *mut *mut zahl;
    static mut libzahl_temp_stack_end: *mut *mut zahl;
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
pub const ZAHL_BITS_PER_CHAR: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
pub const ZAHL_LB_BITS_PER_CHAR: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const ZAHL_FLUFF: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn zinit(mut a: *mut zahl) {
    (*a).alloced = 0 as size_t;
    (*a).chars = ::core::ptr::null_mut::<zahl_char_t>();
}
#[inline]
unsafe extern "C" fn zzero(mut a: *mut zahl) -> ::core::ffi::c_int {
    return ((*a).sign == 0) as ::core::ffi::c_int;
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
pub const ENOENT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const BITS_PER_CHAR: ::core::ffi::c_int = ZAHL_BITS_PER_CHAR;
pub const LB_BITS_PER_CHAR: ::core::ffi::c_int = ZAHL_LB_BITS_PER_CHAR;
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
unsafe extern "C" fn zsplit_unsigned_fast_large_taint(
    mut high: *mut zahl,
    mut low: *mut zahl,
    mut a: *mut zahl,
    mut n: size_t,
) {
    n >>= LB_BITS_PER_CHAR;
    (*high).sign = 1 as ::core::ffi::c_int;
    (*high).used = (*a).used.wrapping_sub(n);
    (*high).chars = (*a).chars.offset(n as isize);
    (*low).sign = 1 as ::core::ffi::c_int;
    (*low).used = n;
    (*low).chars = (*a).chars;
    while (*low).used != 0
        && *(*low)
            .chars
            .offset((*low).used.wrapping_sub(1 as size_t) as isize)
            == 0
    {
        (*low).used = (*low).used.wrapping_sub(1);
    }
    if (*low).used == 0 {
        (*low).sign = 0 as ::core::ffi::c_int;
    }
}
#[inline]
unsafe extern "C" fn zsplit_unsigned_fast_small_auto(
    mut high: *mut zahl,
    mut low: *mut zahl,
    mut a: *mut zahl,
    mut n: size_t,
) {
    let mut mask: zahl_char_t = 1 as zahl_char_t;
    mask = (mask << n).wrapping_sub(1 as zahl_char_t);
    (*high).sign = 1 as ::core::ffi::c_int;
    (*high).used = 1 as size_t;
    *(*high).chars.offset(0 as ::core::ffi::c_int as isize) =
        *(*a).chars.offset(0 as ::core::ffi::c_int as isize) >> n;
    if (*a).used == 2 as size_t {
        *(*high).chars.offset(1 as ::core::ffi::c_int as isize) =
            *(*a).chars.offset(1 as ::core::ffi::c_int as isize) >> n;
        (*high).used = ((*high).used as ::core::ffi::c_ulong).wrapping_add(
            (*(*high).chars.offset(1 as ::core::ffi::c_int as isize) != 0) as ::core::ffi::c_int
                as ::core::ffi::c_ulong,
        ) as size_t as size_t;
        n = (BITS_PER_CHAR as size_t).wrapping_sub(n);
        let ref mut fresh3 = *(*high).chars.offset(0 as ::core::ffi::c_int as isize);
        *fresh3 = (*fresh3 as ::core::ffi::c_ulong
            | ((*(*a).chars.offset(1 as ::core::ffi::c_int as isize) & mask) << n)
                as ::core::ffi::c_ulong) as zahl_char_t;
    }
    (*low).sign = 1 as ::core::ffi::c_int;
    (*low).used = 1 as size_t;
    *(*low).chars.offset(0 as ::core::ffi::c_int as isize) =
        *(*a).chars.offset(0 as ::core::ffi::c_int as isize) & mask;
    if (*(*low).chars.offset(0 as ::core::ffi::c_int as isize) == 0) as ::core::ffi::c_int
        as ::core::ffi::c_long
        != 0
    {
        (*low).sign = 0 as ::core::ffi::c_int;
    }
}
#[inline]
unsafe extern "C" fn zinit_temp(mut a: *mut zahl) {
    zinit(a);
    if (libzahl_temp_stack_head == libzahl_temp_stack_end) as ::core::ffi::c_int
        as ::core::ffi::c_long
        != 0
    {
        let mut n: size_t =
            libzahl_temp_stack_end.offset_from(libzahl_temp_stack) as ::core::ffi::c_long as size_t;
        let mut old: *mut ::core::ffi::c_void = libzahl_temp_stack as *mut ::core::ffi::c_void;
        libzahl_temp_stack = realloc(
            old,
            (2 as size_t)
                .wrapping_mul(n)
                .wrapping_mul(::core::mem::size_of::<*mut zahl>() as size_t),
        ) as *mut *mut zahl;
        if libzahl_temp_stack.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
            libzahl_temp_stack = old as *mut *mut zahl;
            libzahl_memfailure();
        }
        libzahl_temp_stack_head = libzahl_temp_stack.offset(n as isize);
        libzahl_temp_stack_end = libzahl_temp_stack_head.offset(n as isize);
    }
    let fresh2 = libzahl_temp_stack_head;
    libzahl_temp_stack_head = libzahl_temp_stack_head.offset(1);
    *fresh2 = a as *mut zahl;
}
#[inline]
unsafe extern "C" fn zfree_temp(mut a: *mut zahl) {
    zfree(a);
    libzahl_temp_stack_head = libzahl_temp_stack_head.offset(-1);
}
#[inline]
unsafe extern "C" fn zsqr_ll_single_char(mut a: *mut zahl, mut b: *mut zahl) {
    if (*a).alloced < 1 as size_t {
        libzahl_realloc(a as *mut zahl, 1 as size_t);
    }
    (*a).used = 1 as size_t;
    *(*a).chars.offset(0 as ::core::ffi::c_int as isize) =
        (*(*b).chars.offset(0 as ::core::ffi::c_int as isize))
            .wrapping_mul(*(*b).chars.offset(0 as ::core::ffi::c_int as isize));
    (*a).sign = 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn zsqr_ll(mut a: *mut zahl, mut b: *mut zahl) {
    let mut z0: z_t = [zahl {
        sign: 0,
        padding__: 0,
        used: 0,
        alloced: 0,
        chars: ::core::ptr::null_mut::<zahl_char_t>(),
    }; 1];
    let mut z1: z_t = [zahl {
        sign: 0,
        padding__: 0,
        used: 0,
        alloced: 0,
        chars: ::core::ptr::null_mut::<zahl_char_t>(),
    }; 1];
    let mut high: z_t = [zahl {
        sign: 0,
        padding__: 0,
        used: 0,
        alloced: 0,
        chars: ::core::ptr::null_mut::<zahl_char_t>(),
    }; 1];
    let mut low: z_t = [zahl {
        sign: 0,
        padding__: 0,
        used: 0,
        alloced: 0,
        chars: ::core::ptr::null_mut::<zahl_char_t>(),
    }; 1];
    let mut auxchars: [zahl_char_t; 12] = [0; 12];
    let mut bits: size_t = 0;
    bits = zbits(b);
    if bits <= (BITS_PER_CHAR / 2 as ::core::ffi::c_int) as size_t {
        zsqr_ll_single_char(a, b);
        return;
    }
    bits >>= 1 as ::core::ffi::c_int;
    if bits < BITS_PER_CHAR as size_t {
        let ref mut fresh0 = (*(&raw mut low as *mut zahl)).chars;
        *fresh0 = &raw mut auxchars as *mut zahl_char_t;
        let ref mut fresh1 = (*(&raw mut high as *mut zahl)).chars;
        *fresh1 = (&raw mut auxchars as *mut zahl_char_t).offset(ZAHL_FLUFF as isize);
        zsplit_unsigned_fast_small_auto(
            &raw mut high as *mut zahl,
            &raw mut low as *mut zahl,
            b,
            bits,
        );
    } else {
        bits = bits & !((ZAHL_BITS_PER_CHAR - 1 as ::core::ffi::c_int) as size_t);
        zsplit_unsigned_fast_large_taint(
            &raw mut high as *mut zahl,
            &raw mut low as *mut zahl,
            b,
            bits,
        );
    }
    if (zzero(&raw mut low as *mut zahl) != 0) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        zsqr_ll(a, &raw mut high as *mut zahl);
        zlsh(a, a, bits << 1 as ::core::ffi::c_int);
    } else {
        zinit_temp(&raw mut z0 as *mut zahl);
        zinit_temp(&raw mut z1 as *mut zahl);
        zsqr_ll(&raw mut z0 as *mut zahl, &raw mut low as *mut zahl);
        zmul_ll(
            &raw mut z1 as *mut zahl,
            &raw mut low as *mut zahl,
            &raw mut high as *mut zahl,
        );
        zlsh(
            &raw mut z1 as *mut zahl,
            &raw mut z1 as *mut zahl,
            bits.wrapping_add(1 as size_t),
        );
        zsqr_ll(a, &raw mut high as *mut zahl);
        zlsh(a, a, bits << 1 as ::core::ffi::c_int);
        zadd_unsigned_assign(a, &raw mut z1 as *mut zahl);
        zadd_unsigned_assign(a, &raw mut z0 as *mut zahl);
        zfree_temp(&raw mut z1 as *mut zahl);
        zfree_temp(&raw mut z0 as *mut zahl);
    };
}
