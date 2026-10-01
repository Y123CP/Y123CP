extern "C" {
    fn longjmp(__env: *mut __jmp_buf_tag, __val: ::core::ffi::c_int) -> !;
    fn libzahl_realloc(_: *mut zahl, _: size_t);
    fn zfree(_: *mut zahl);
    fn zadd_unsigned_assign(_: *mut zahl, _: *mut zahl);
    fn zsub_nonnegative_assign(_: *mut zahl, _: *mut zahl);
    fn zlsh(_: *mut zahl, _: *mut zahl, _: size_t);
    fn zrsh(_: *mut zahl, _: *mut zahl, _: size_t);
    fn ztrunc(_: *mut zahl, _: *mut zahl, _: size_t);
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
#[inline]
unsafe extern "C" fn zsplit(
    mut high: *mut zahl,
    mut low: *mut zahl,
    mut a: *mut zahl,
    mut delim: size_t,
) {
    if (high == a) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        ztrunc(low, a, delim);
        zrsh(high, a, delim);
    } else {
        zrsh(high, a, delim);
        ztrunc(low, a, delim);
    };
}
pub const ENOENT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const BITS_PER_CHAR: ::core::ffi::c_int = ZAHL_BITS_PER_CHAR;
#[inline]
unsafe extern "C" fn zzero1(mut a: *mut zahl, mut b: *mut zahl) -> ::core::ffi::c_int {
    return (zzero(a) != 0 || zzero(b) != 0) as ::core::ffi::c_int;
}
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
unsafe extern "C" fn zsplit_pz(
    mut high: *mut zahl,
    mut low: *mut zahl,
    mut a: *mut zahl,
    mut delim: size_t,
) {
    if (zzero(a) != 0) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        (*high).sign = 0 as ::core::ffi::c_int;
        (*low).sign = 0 as ::core::ffi::c_int;
    } else {
        zsplit(high, low, a, delim);
    };
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
    let fresh0 = libzahl_temp_stack_head;
    libzahl_temp_stack_head = libzahl_temp_stack_head.offset(1);
    *fresh0 = a as *mut zahl;
}
#[inline]
unsafe extern "C" fn zfree_temp(mut a: *mut zahl) {
    zfree(a);
    libzahl_temp_stack_head = libzahl_temp_stack_head.offset(-1);
}
#[inline]
unsafe extern "C" fn zmul_ll_single_char(mut a: *mut zahl, mut b: *mut zahl, mut c: *mut zahl) {
    if (*a).alloced < 1 as size_t {
        libzahl_realloc(a as *mut zahl, 1 as size_t);
    }
    (*a).used = 1 as size_t;
    *(*a).chars.offset(0 as ::core::ffi::c_int as isize) =
        (*(*b).chars.offset(0 as ::core::ffi::c_int as isize))
            .wrapping_mul(*(*c).chars.offset(0 as ::core::ffi::c_int as isize));
    (*a).sign = 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn zmul_ll(mut a: *mut zahl, mut b: *mut zahl, mut c: *mut zahl) {
    let mut m: size_t = 0;
    let mut m2: size_t = 0;
    let mut b_high: z_t = [zahl {
        sign: 0,
        padding__: 0,
        used: 0,
        alloced: 0,
        chars: ::core::ptr::null_mut::<zahl_char_t>(),
    }; 1];
    let mut b_low: z_t = [zahl {
        sign: 0,
        padding__: 0,
        used: 0,
        alloced: 0,
        chars: ::core::ptr::null_mut::<zahl_char_t>(),
    }; 1];
    let mut c_high: z_t = [zahl {
        sign: 0,
        padding__: 0,
        used: 0,
        alloced: 0,
        chars: ::core::ptr::null_mut::<zahl_char_t>(),
    }; 1];
    let mut c_low: z_t = [zahl {
        sign: 0,
        padding__: 0,
        used: 0,
        alloced: 0,
        chars: ::core::ptr::null_mut::<zahl_char_t>(),
    }; 1];
    if (zzero1(b, c) != 0) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        (*a).sign = 0 as ::core::ffi::c_int;
        return;
    }
    m = zbits(b);
    m2 = if b == c { m } else { zbits(c) };
    if m.wrapping_add(m2) <= BITS_PER_CHAR as size_t {
        zmul_ll_single_char(a, b, c);
        return;
    }
    m = if m > m2 { m } else { m2 };
    m2 = m >> 1 as ::core::ffi::c_int;
    zinit_temp(&raw mut b_high as *mut zahl);
    zinit_temp(&raw mut b_low as *mut zahl);
    zinit_temp(&raw mut c_high as *mut zahl);
    zinit_temp(&raw mut c_low as *mut zahl);
    zsplit_pz(
        &raw mut b_high as *mut zahl,
        &raw mut b_low as *mut zahl,
        b,
        m2,
    );
    zsplit_pz(
        &raw mut c_high as *mut zahl,
        &raw mut c_low as *mut zahl,
        c,
        m2,
    );
    zmul_ll(a, &raw mut b_low as *mut zahl, &raw mut c_low as *mut zahl);
    zadd_unsigned_assign(&raw mut b_low as *mut zahl, &raw mut b_high as *mut zahl);
    zadd_unsigned_assign(&raw mut c_low as *mut zahl, &raw mut c_high as *mut zahl);
    zmul_ll(
        &raw mut b_low as *mut zahl,
        &raw mut b_low as *mut zahl,
        &raw mut c_low as *mut zahl,
    );
    zmul_ll(
        &raw mut c_low as *mut zahl,
        &raw mut b_high as *mut zahl,
        &raw mut c_high as *mut zahl,
    );
    zsub_nonnegative_assign(&raw mut b_low as *mut zahl, a);
    zsub_nonnegative_assign(&raw mut b_low as *mut zahl, &raw mut c_low as *mut zahl);
    zlsh(&raw mut b_low as *mut zahl, &raw mut b_low as *mut zahl, m2);
    m2 <<= 1 as ::core::ffi::c_int;
    zlsh(&raw mut c_low as *mut zahl, &raw mut c_low as *mut zahl, m2);
    zadd_unsigned_assign(a, &raw mut b_low as *mut zahl);
    zadd_unsigned_assign(a, &raw mut c_low as *mut zahl);
    zfree_temp(&raw mut c_low as *mut zahl);
    zfree_temp(&raw mut c_high as *mut zahl);
    zfree_temp(&raw mut b_low as *mut zahl);
    zfree_temp(&raw mut b_high as *mut zahl);
}
