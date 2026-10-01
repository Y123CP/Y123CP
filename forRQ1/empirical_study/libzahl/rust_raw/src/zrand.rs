extern "C" {
    fn longjmp(__env: *mut __jmp_buf_tag, __val: ::core::ffi::c_int) -> !;
    fn close(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn read(__fd: ::core::ffi::c_int, __buf: *mut ::core::ffi::c_void, __nbytes: size_t)
        -> ssize_t;
    fn libzahl_realloc(_: *mut zahl, _: size_t);
    fn zfree(_: *mut zahl);
    fn zadd(_: *mut zahl, _: *mut zahl, _: *mut zahl);
    fn zsub(_: *mut zahl, _: *mut zahl, _: *mut zahl);
    fn zrsh(_: *mut zahl, _: *mut zahl, _: size_t);
    fn zmul_ll(_: *mut zahl, _: *mut zahl, _: *mut zahl);
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn random() -> ::core::ffi::c_long;
    fn srandom(__seed: ::core::ffi::c_uint);
    fn rand() -> ::core::ffi::c_int;
    fn srand(__seed: ::core::ffi::c_uint);
    fn lrand48() -> ::core::ffi::c_long;
    fn srand48(__seedval: ::core::ffi::c_long);
    fn free(__ptr: *mut ::core::ffi::c_void);
    static mut libzahl_const_1: z_t;
    static mut libzahl_jmp_buf: jmp_buf;
    static mut libzahl_error: ::core::ffi::c_int;
    static mut libzahl_temp_stack: *mut *mut zahl;
    static mut libzahl_temp_stack_head: *mut *mut zahl;
    static mut libzahl_temp_allocation: *mut ::core::ffi::c_void;
    fn open(
        __file: *const ::core::ffi::c_char,
        __oflag: ::core::ffi::c_int,
        ...
    ) -> ::core::ffi::c_int;
    fn time(__timer: *mut time_t) -> time_t;
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
pub type __time_t = ::core::ffi::c_long;
pub type __ssize_t = ::core::ffi::c_long;
pub type uint64_t = __uint64_t;
pub type intptr_t = isize;
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
pub type z_t = [zahl; 1];
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
pub type zerror = ::core::ffi::c_uint;
pub const ZERROR_INVALID_RADIX: zerror = 5;
pub const ZERROR_NEGATIVE: zerror = 4;
pub const ZERROR_DIV_0: zerror = 3;
pub const ZERROR_0_DIV_0: zerror = 2;
pub const ZERROR_0_POW_0: zerror = 1;
pub const ZERROR_ERRNO_SET: zerror = 0;
pub type time_t = __time_t;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const ZAHL_BITS_PER_CHAR: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
pub const ZAHL_LB_BITS_PER_CHAR: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn zzero(mut a: *mut zahl) -> ::core::ffi::c_int {
    return ((*a).sign == 0) as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn zsignum(mut a: *mut zahl) -> ::core::ffi::c_int {
    return (*a).sign;
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
#[inline]
unsafe extern "C" fn zmul(mut a: *mut zahl, mut b: *mut zahl, mut c: *mut zahl) {
    let mut b_sign: ::core::ffi::c_int = 0;
    let mut c_sign: ::core::ffi::c_int = 0;
    b_sign = (*b).sign;
    (*b).sign *= b_sign;
    c_sign = (*c).sign;
    (*c).sign *= c_sign;
    zmul_ll(a, b, c);
    (*c).sign = c_sign;
    (*b).sign = b_sign;
    (*a).sign = zsignum(b) * zsignum(c);
}
pub const EINVAL: ::core::ffi::c_int = 22 as ::core::ffi::c_int;
pub const RAND_MAX: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
pub const O_RDONLY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
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
pub const FAST_RANDOM_PATHNAME: [::core::ffi::c_char; 13] =
    unsafe { ::core::mem::transmute::<[u8; 13], [::core::ffi::c_char; 13]>(*b"/dev/urandom\0") };
pub const SECURE_RANDOM_PATHNAME: [::core::ffi::c_char; 12] =
    unsafe { ::core::mem::transmute::<[u8; 12], [::core::ffi::c_char; 12]>(*b"/dev/random\0") };
unsafe extern "C" fn zrand_libc_rand(
    mut out: *mut ::core::ffi::c_void,
    mut n: size_t,
    mut statep: *mut ::core::ffi::c_void,
) {
    static mut inited: ::core::ffi::c_char = 0 as ::core::ffi::c_char;
    let mut ri: ::core::ffi::c_uint = 0;
    let mut rd: ::core::ffi::c_double = 0.;
    let mut buf: *mut ::core::ffi::c_uchar = out as *mut ::core::ffi::c_uchar;
    if inited == 0 {
        inited = 1 as ::core::ffi::c_char;
        srand(
            (out as intptr_t | time(::core::ptr::null_mut::<time_t>()) as intptr_t)
                as ::core::ffi::c_uint,
        );
    }
    loop {
        let fresh6 = n;
        n = n.wrapping_sub(1);
        if !(fresh6 != 0) {
            break;
        }
        ri = rand() as ::core::ffi::c_uint;
        rd = ri as ::core::ffi::c_double
            / (RAND_MAX as ::core::ffi::c_double
                + 1 as ::core::ffi::c_int as ::core::ffi::c_double);
        rd *= 256 as ::core::ffi::c_int as ::core::ffi::c_double;
        *buf.offset(n as isize) = rd as ::core::ffi::c_uchar;
    }
}
unsafe extern "C" fn zrand_libc_rand48(
    mut out: *mut ::core::ffi::c_void,
    mut n: size_t,
    mut statep: *mut ::core::ffi::c_void,
) {
    static mut inited: ::core::ffi::c_char = 0 as ::core::ffi::c_char;
    let mut r0: ::core::ffi::c_long = 0;
    let mut r1: ::core::ffi::c_long = 0;
    let mut buf: *mut ::core::ffi::c_uchar = out as *mut ::core::ffi::c_uchar;
    if inited == 0 {
        inited = 1 as ::core::ffi::c_char;
        srand48(
            out as ::core::ffi::c_long
                | time(::core::ptr::null_mut::<time_t>()) as ::core::ffi::c_long,
        );
    }
    loop {
        let fresh2 = n;
        n = n.wrapping_sub(1);
        if !(fresh2 != 0) {
            break;
        }
        r0 = lrand48() & 15 as ::core::ffi::c_long;
        r1 = lrand48() & 15 as ::core::ffi::c_long;
        *buf.offset(n as isize) = (r0 << 4 as ::core::ffi::c_int | r1) as ::core::ffi::c_uchar;
    }
}
unsafe extern "C" fn zrand_libc_random(
    mut out: *mut ::core::ffi::c_void,
    mut n: size_t,
    mut statep: *mut ::core::ffi::c_void,
) {
    static mut inited: ::core::ffi::c_char = 0 as ::core::ffi::c_char;
    let mut ri: ::core::ffi::c_long = 0;
    let mut buf: *mut ::core::ffi::c_uchar = out as *mut ::core::ffi::c_uchar;
    if inited == 0 {
        inited = 1 as ::core::ffi::c_char;
        srandom(
            (out as intptr_t | time(::core::ptr::null_mut::<time_t>()) as intptr_t)
                as ::core::ffi::c_uint,
        );
    }
    loop {
        let fresh3 = n;
        n = n.wrapping_sub(1);
        if !(fresh3 != 0) {
            break;
        }
        ri = random();
        *buf.offset(n as isize) =
            (ri >> 0 as ::core::ffi::c_int & 255 as ::core::ffi::c_long) as ::core::ffi::c_uchar;
        let fresh4 = n;
        n = n.wrapping_sub(1);
        if fresh4 == 0 {
            break;
        }
        *buf.offset(n as isize) =
            (ri >> 8 as ::core::ffi::c_int & 255 as ::core::ffi::c_long) as ::core::ffi::c_uchar;
        let fresh5 = n;
        n = n.wrapping_sub(1);
        if fresh5 == 0 {
            break;
        }
        *buf.offset(n as isize) =
            (ri >> 16 as ::core::ffi::c_int & 255 as ::core::ffi::c_long) as ::core::ffi::c_uchar;
    }
}
unsafe extern "C" fn zrand_fd(
    mut out: *mut ::core::ffi::c_void,
    mut n: size_t,
    mut statep: *mut ::core::ffi::c_void,
) {
    let mut fd: ::core::ffi::c_int = *(statep as *mut ::core::ffi::c_int);
    let mut read_just: ssize_t = 0;
    let mut read_total: size_t = 0 as size_t;
    let mut buf: *mut ::core::ffi::c_char = out as *mut ::core::ffi::c_char;
    while n != 0 {
        read_just = read(
            fd,
            buf.offset(read_total as isize) as *mut ::core::ffi::c_void,
            n,
        );
        if (read_just < 0 as ::core::ffi::c_long) as ::core::ffi::c_int as ::core::ffi::c_long != 0
        {
            libzahl_failure(*__errno_location());
        }
        read_total = (read_total as ::core::ffi::c_ulong)
            .wrapping_add(read_just as size_t as ::core::ffi::c_ulong)
            as size_t as size_t;
        n = (n as ::core::ffi::c_ulong).wrapping_sub(read_just as size_t as ::core::ffi::c_ulong)
            as size_t as size_t;
    }
}
unsafe extern "C" fn zrand_get_random_bits(
    mut r: *mut zahl,
    mut bits: size_t,
    mut fun: Option<
        unsafe extern "C" fn(*mut ::core::ffi::c_void, size_t, *mut ::core::ffi::c_void) -> (),
    >,
    mut statep: *mut ::core::ffi::c_void,
) {
    let mut n: size_t = 0;
    let mut chars: size_t = bits
        .wrapping_add((ZAHL_BITS_PER_CHAR - 1 as ::core::ffi::c_int) as size_t)
        >> ZAHL_LB_BITS_PER_CHAR;
    let mut mask: zahl_char_t = 1 as zahl_char_t;
    if (*r).alloced < chars {
        libzahl_realloc(r as *mut zahl, chars);
    }
    fun.expect("non-null function pointer")(
        (*r).chars as *mut ::core::ffi::c_void,
        chars.wrapping_mul(::core::mem::size_of::<zahl_char_t>() as size_t),
        statep,
    );
    bits = bits & (ZAHL_BITS_PER_CHAR - 1 as ::core::ffi::c_int) as size_t;
    mask <<= bits;
    mask = (mask as ::core::ffi::c_ulong).wrapping_sub(1 as ::core::ffi::c_ulong) as zahl_char_t
        as zahl_char_t;
    let ref mut fresh0 = *(*r).chars.offset(chars.wrapping_sub(1 as size_t) as isize);
    *fresh0 = (*fresh0 as ::core::ffi::c_ulong & mask as ::core::ffi::c_ulong) as zahl_char_t;
    n = chars;
    loop {
        let fresh1 = n;
        n = n.wrapping_sub(1);
        if !(fresh1 != 0) {
            break;
        }
        if (*(*r).chars.offset(n as isize) != 0) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
            (*r).used = n.wrapping_add(1 as size_t);
            (*r).sign = 1 as ::core::ffi::c_int;
            return;
        }
    }
    (*r).sign = 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn zrand(
    mut r: *mut zahl,
    mut dev: zranddev,
    mut dist: zranddist,
    mut n: *mut zahl,
) {
    let mut pathname: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut bits: size_t = 0;
    let mut fd: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut statep: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut random_fun: Option<
        unsafe extern "C" fn(*mut ::core::ffi::c_void, size_t, *mut ::core::ffi::c_void) -> (),
    > = Some(
        zrand_fd
            as unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                size_t,
                *mut ::core::ffi::c_void,
            ) -> (),
    );
    match dev as ::core::ffi::c_uint {
        0 => {
            pathname = FAST_RANDOM_PATHNAME.as_ptr();
        }
        1 => {
            pathname = SECURE_RANDOM_PATHNAME.as_ptr();
        }
        4 => {
            random_fun = Some(
                zrand_libc_rand
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        size_t,
                        *mut ::core::ffi::c_void,
                    ) -> (),
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        size_t,
                        *mut ::core::ffi::c_void,
                    ) -> (),
                >;
        }
        2 | 3 | 5 => {
            random_fun = Some(
                zrand_libc_random
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        size_t,
                        *mut ::core::ffi::c_void,
                    ) -> (),
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        size_t,
                        *mut ::core::ffi::c_void,
                    ) -> (),
                >;
        }
        6 => {
            random_fun = Some(
                zrand_libc_rand48
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        size_t,
                        *mut ::core::ffi::c_void,
                    ) -> (),
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        size_t,
                        *mut ::core::ffi::c_void,
                    ) -> (),
                >;
        }
        _ => {
            libzahl_failure(EINVAL);
        }
    }
    if (zzero(n) != 0) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        (*r).sign = 0 as ::core::ffi::c_int;
        return;
    }
    if !pathname.is_null() {
        fd = open(pathname, O_RDONLY);
        if (fd < 0 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
            libzahl_failure(*__errno_location());
        }
        statep = &raw mut fd as *mut ::core::ffi::c_void;
    }
    match dist as ::core::ffi::c_uint {
        0 => {
            if (zsignum(n) < 0 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long
                != 0
            {
                libzahl_failure(-(ZERROR_NEGATIVE as ::core::ffi::c_int));
            }
            bits = zbits(n);
            loop {
                zrand_get_random_bits(r, bits, random_fun, statep);
                if !(0 as ::core::ffi::c_int != 0
                    && (zcmpmag(r, n) > 0 as ::core::ffi::c_int) as ::core::ffi::c_int
                        as ::core::ffi::c_long
                        != 0)
                {
                    break;
                }
            }
            zadd(r, r, &raw mut libzahl_const_1 as *mut zahl);
            zmul(r, r, n);
            zrsh(r, r, bits);
        }
        1 => {
            if (zsignum(n) < 0 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long
                != 0
            {
                libzahl_failure(-(ZERROR_NEGATIVE as ::core::ffi::c_int));
            }
            bits = zbits(n);
            loop {
                zrand_get_random_bits(r, bits, random_fun, statep);
                if !(1 as ::core::ffi::c_int != 0
                    && (zcmpmag(r, n) > 0 as ::core::ffi::c_int) as ::core::ffi::c_int
                        as ::core::ffi::c_long
                        != 0)
                {
                    break;
                }
            }
        }
        2 => {
            if (zsignum(n) < 0 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long
                != 0
            {
                libzahl_failure(-(ZERROR_NEGATIVE as ::core::ffi::c_int));
            }
            bits = zbits(n);
            loop {
                zrand_get_random_bits(r, bits, random_fun, statep);
                if !(0 as ::core::ffi::c_int != 0
                    && (zcmpmag(r, n) > 0 as ::core::ffi::c_int) as ::core::ffi::c_int
                        as ::core::ffi::c_long
                        != 0)
                {
                    break;
                }
            }
            if (zcmpmag(r, n) > 0 as ::core::ffi::c_int) as ::core::ffi::c_int
                as ::core::ffi::c_long
                != 0
            {
                zsub(r, r, n);
            }
        }
        _ => {
            libzahl_failure(EINVAL);
        }
    }
    if fd >= 0 as ::core::ffi::c_int {
        close(fd);
    }
}
