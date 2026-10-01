extern "C" {
    fn longjmp(__env: *mut __jmp_buf_tag, __val: ::core::ffi::c_int) -> !;
    fn libzahl_realloc(_: *mut zahl, _: size_t);
    fn zfree(_: *mut zahl);
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
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
#[no_mangle]
pub static mut libzahl_tmp_str_mag: z_t = [zahl {
    sign: 0,
    padding__: 0,
    used: 0,
    alloced: 0,
    chars: ::core::ptr::null::<zahl_char_t>() as *mut zahl_char_t,
}; 1];
#[no_mangle]
pub static mut libzahl_tmp_mod: z_t = [zahl {
    sign: 0,
    padding__: 0,
    used: 0,
    alloced: 0,
    chars: ::core::ptr::null::<zahl_char_t>() as *mut zahl_char_t,
}; 1];
#[no_mangle]
pub static mut libzahl_tmp_ptest_n4: z_t = [zahl {
    sign: 0,
    padding__: 0,
    used: 0,
    alloced: 0,
    chars: ::core::ptr::null::<zahl_char_t>() as *mut zahl_char_t,
}; 1];
#[no_mangle]
pub static mut libzahl_tmp_ptest_n1: z_t = [zahl {
    sign: 0,
    padding__: 0,
    used: 0,
    alloced: 0,
    chars: ::core::ptr::null::<zahl_char_t>() as *mut zahl_char_t,
}; 1];
#[no_mangle]
pub static mut libzahl_tmp_ptest_d: z_t = [zahl {
    sign: 0,
    padding__: 0,
    used: 0,
    alloced: 0,
    chars: ::core::ptr::null::<zahl_char_t>() as *mut zahl_char_t,
}; 1];
#[no_mangle]
pub static mut libzahl_tmp_ptest_a: z_t = [zahl {
    sign: 0,
    padding__: 0,
    used: 0,
    alloced: 0,
    chars: ::core::ptr::null::<zahl_char_t>() as *mut zahl_char_t,
}; 1];
#[no_mangle]
pub static mut libzahl_tmp_ptest_x: z_t = [zahl {
    sign: 0,
    padding__: 0,
    used: 0,
    alloced: 0,
    chars: ::core::ptr::null::<zahl_char_t>() as *mut zahl_char_t,
}; 1];
#[no_mangle]
pub static mut libzahl_tmp_divmod_b: z_t = [zahl {
    sign: 0,
    padding__: 0,
    used: 0,
    alloced: 0,
    chars: ::core::ptr::null::<zahl_char_t>() as *mut zahl_char_t,
}; 1];
#[no_mangle]
pub static mut libzahl_tmp_divmod_a: z_t = [zahl {
    sign: 0,
    padding__: 0,
    used: 0,
    alloced: 0,
    chars: ::core::ptr::null::<zahl_char_t>() as *mut zahl_char_t,
}; 1];
#[no_mangle]
pub static mut libzahl_tmp_modsqr: z_t = [zahl {
    sign: 0,
    padding__: 0,
    used: 0,
    alloced: 0,
    chars: ::core::ptr::null::<zahl_char_t>() as *mut zahl_char_t,
}; 1];
#[no_mangle]
pub static mut libzahl_tmp_pow_d: z_t = [zahl {
    sign: 0,
    padding__: 0,
    used: 0,
    alloced: 0,
    chars: ::core::ptr::null::<zahl_char_t>() as *mut zahl_char_t,
}; 1];
#[no_mangle]
pub static mut libzahl_tmp_pow_c: z_t = [zahl {
    sign: 0,
    padding__: 0,
    used: 0,
    alloced: 0,
    chars: ::core::ptr::null::<zahl_char_t>() as *mut zahl_char_t,
}; 1];
#[no_mangle]
pub static mut libzahl_tmp_pow_b: z_t = [zahl {
    sign: 0,
    padding__: 0,
    used: 0,
    alloced: 0,
    chars: ::core::ptr::null::<zahl_char_t>() as *mut zahl_char_t,
}; 1];
#[no_mangle]
pub static mut libzahl_tmp_modmul: z_t = [zahl {
    sign: 0,
    padding__: 0,
    used: 0,
    alloced: 0,
    chars: ::core::ptr::null::<zahl_char_t>() as *mut zahl_char_t,
}; 1];
#[no_mangle]
pub static mut libzahl_tmp_sub: z_t = [zahl {
    sign: 0,
    padding__: 0,
    used: 0,
    alloced: 0,
    chars: ::core::ptr::null::<zahl_char_t>() as *mut zahl_char_t,
}; 1];
#[no_mangle]
pub static mut libzahl_tmp_gcd_v: z_t = [zahl {
    sign: 0,
    padding__: 0,
    used: 0,
    alloced: 0,
    chars: ::core::ptr::null::<zahl_char_t>() as *mut zahl_char_t,
}; 1];
#[no_mangle]
pub static mut libzahl_tmp_gcd_u: z_t = [zahl {
    sign: 0,
    padding__: 0,
    used: 0,
    alloced: 0,
    chars: ::core::ptr::null::<zahl_char_t>() as *mut zahl_char_t,
}; 1];
#[no_mangle]
pub static mut libzahl_tmp_str_rem: z_t = [zahl {
    sign: 0,
    padding__: 0,
    used: 0,
    alloced: 0,
    chars: ::core::ptr::null::<zahl_char_t>() as *mut zahl_char_t,
}; 1];
#[no_mangle]
pub static mut libzahl_tmp_str_div: z_t = [zahl {
    sign: 0,
    padding__: 0,
    used: 0,
    alloced: 0,
    chars: ::core::ptr::null::<zahl_char_t>() as *mut zahl_char_t,
}; 1];
#[no_mangle]
pub static mut libzahl_tmp_div: z_t = [zahl {
    sign: 0,
    padding__: 0,
    used: 0,
    alloced: 0,
    chars: ::core::ptr::null::<zahl_char_t>() as *mut zahl_char_t,
}; 1];
#[no_mangle]
pub static mut libzahl_tmp_str_num: z_t = [zahl {
    sign: 0,
    padding__: 0,
    used: 0,
    alloced: 0,
    chars: ::core::ptr::null::<zahl_char_t>() as *mut zahl_char_t,
}; 1];
#[no_mangle]
pub static mut libzahl_tmp_divmod_d: z_t = [zahl {
    sign: 0,
    padding__: 0,
    used: 0,
    alloced: 0,
    chars: ::core::ptr::null::<zahl_char_t>() as *mut zahl_char_t,
}; 1];
#[no_mangle]
pub static mut libzahl_const_1: z_t = [zahl {
    sign: 0,
    padding__: 0,
    used: 0,
    alloced: 0,
    chars: ::core::ptr::null::<zahl_char_t>() as *mut zahl_char_t,
}; 1];
#[no_mangle]
pub static mut libzahl_const_4: z_t = [zahl {
    sign: 0,
    padding__: 0,
    used: 0,
    alloced: 0,
    chars: ::core::ptr::null::<zahl_char_t>() as *mut zahl_char_t,
}; 1];
#[no_mangle]
pub static mut libzahl_const_1e19: z_t = [zahl {
    sign: 0,
    padding__: 0,
    used: 0,
    alloced: 0,
    chars: ::core::ptr::null::<zahl_char_t>() as *mut zahl_char_t,
}; 1];
#[no_mangle]
pub static mut libzahl_const_2: z_t = [zahl {
    sign: 0,
    padding__: 0,
    used: 0,
    alloced: 0,
    chars: ::core::ptr::null::<zahl_char_t>() as *mut zahl_char_t,
}; 1];
#[no_mangle]
pub static mut libzahl_tmp_divmod_ds: [z_t; 64] = [[zahl {
    sign: 0,
    padding__: 0,
    used: 0,
    alloced: 0,
    chars: ::core::ptr::null::<zahl_char_t>() as *mut zahl_char_t,
}; 1]; 64];
#[no_mangle]
pub static mut libzahl_jmp_buf: jmp_buf = [__jmp_buf_tag {
    __jmpbuf: [0; 8],
    __mask_was_saved: 0,
    __saved_mask: __sigset_t { __val: [0; 16] },
}; 1];
#[no_mangle]
pub static mut libzahl_set_up: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub static mut libzahl_error: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut libzahl_pool: [*mut *mut zahl_char_t; 64] =
    [::core::ptr::null::<*mut zahl_char_t>() as *mut *mut zahl_char_t; 64];
#[no_mangle]
pub static mut libzahl_pool_n: [size_t; 64] = [0; 64];
#[no_mangle]
pub static mut libzahl_pool_alloc: [size_t; 64] = [0; 64];
#[no_mangle]
pub static mut libzahl_temp_stack: *mut *mut zahl =
    ::core::ptr::null::<*mut zahl>() as *mut *mut zahl;
#[no_mangle]
pub static mut libzahl_temp_stack_head: *mut *mut zahl =
    ::core::ptr::null::<*mut zahl>() as *mut *mut zahl;
#[no_mangle]
pub static mut libzahl_temp_stack_end: *mut *mut zahl =
    ::core::ptr::null::<*mut zahl>() as *mut *mut zahl;
#[no_mangle]
pub static mut libzahl_temp_allocation: *mut ::core::ffi::c_void =
    ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void;
static mut constant_chars: [zahl_char_t; 8] = [0; 8];
#[no_mangle]
pub unsafe extern "C" fn zsetup(mut env: *mut __jmp_buf_tag) {
    let mut i: size_t = 0;
    *(&raw mut libzahl_jmp_buf as *mut __jmp_buf_tag) = *env;
    if (libzahl_set_up == 0) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        libzahl_set_up = 1 as ::core::ffi::c_int;
        memset(
            &raw mut libzahl_pool as *mut *mut *mut zahl_char_t as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<[*mut *mut zahl_char_t; 64]>() as size_t,
        );
        memset(
            &raw mut libzahl_pool_n as *mut size_t as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<[size_t; 64]>() as size_t,
        );
        memset(
            &raw mut libzahl_pool_alloc as *mut size_t as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<[size_t; 64]>() as size_t,
        );
        zinit(&raw mut libzahl_tmp_div as *mut zahl);
        zinit(&raw mut libzahl_tmp_mod as *mut zahl);
        zinit(&raw mut libzahl_tmp_str_num as *mut zahl);
        zinit(&raw mut libzahl_tmp_str_mag as *mut zahl);
        zinit(&raw mut libzahl_tmp_str_div as *mut zahl);
        zinit(&raw mut libzahl_tmp_str_rem as *mut zahl);
        zinit(&raw mut libzahl_tmp_gcd_u as *mut zahl);
        zinit(&raw mut libzahl_tmp_gcd_v as *mut zahl);
        zinit(&raw mut libzahl_tmp_sub as *mut zahl);
        zinit(&raw mut libzahl_tmp_modmul as *mut zahl);
        zinit(&raw mut libzahl_tmp_pow_b as *mut zahl);
        zinit(&raw mut libzahl_tmp_pow_c as *mut zahl);
        zinit(&raw mut libzahl_tmp_pow_d as *mut zahl);
        zinit(&raw mut libzahl_tmp_modsqr as *mut zahl);
        zinit(&raw mut libzahl_tmp_divmod_a as *mut zahl);
        zinit(&raw mut libzahl_tmp_divmod_b as *mut zahl);
        zinit(&raw mut libzahl_tmp_divmod_d as *mut zahl);
        zinit(&raw mut libzahl_tmp_ptest_x as *mut zahl);
        zinit(&raw mut libzahl_tmp_ptest_a as *mut zahl);
        zinit(&raw mut libzahl_tmp_ptest_d as *mut zahl);
        zinit(&raw mut libzahl_tmp_ptest_n1 as *mut zahl);
        zinit(&raw mut libzahl_tmp_ptest_n4 as *mut zahl);
        (*(&raw mut libzahl_const_1e19 as *mut zahl)).alloced = 1 as size_t;
        let ref mut fresh0 = (*(&raw mut libzahl_const_1e19 as *mut zahl)).chars;
        *fresh0 =
            (&raw mut constant_chars as *mut zahl_char_t).offset(0 as ::core::ffi::c_int as isize);
        zsetu(
            &raw mut libzahl_const_1e19 as *mut zahl,
            10000000000000000000 as uint64_t,
        );
        (*(&raw mut libzahl_const_1 as *mut zahl)).alloced = 1 as size_t;
        let ref mut fresh1 = (*(&raw mut libzahl_const_1 as *mut zahl)).chars;
        *fresh1 =
            (&raw mut constant_chars as *mut zahl_char_t).offset(1 as ::core::ffi::c_int as isize);
        zsetu(&raw mut libzahl_const_1 as *mut zahl, 1 as uint64_t);
        (*(&raw mut libzahl_const_2 as *mut zahl)).alloced = 1 as size_t;
        let ref mut fresh2 = (*(&raw mut libzahl_const_2 as *mut zahl)).chars;
        *fresh2 =
            (&raw mut constant_chars as *mut zahl_char_t).offset(2 as ::core::ffi::c_int as isize);
        zsetu(&raw mut libzahl_const_2 as *mut zahl, 2 as uint64_t);
        (*(&raw mut libzahl_const_4 as *mut zahl)).alloced = 1 as size_t;
        let ref mut fresh3 = (*(&raw mut libzahl_const_4 as *mut zahl)).chars;
        *fresh3 =
            (&raw mut constant_chars as *mut zahl_char_t).offset(3 as ::core::ffi::c_int as isize);
        zsetu(&raw mut libzahl_const_4 as *mut zahl, 4 as uint64_t);
        i = BITS_PER_CHAR as size_t;
        loop {
            let fresh4 = i;
            i = i.wrapping_sub(1);
            if !(fresh4 != 0) {
                break;
            }
            zinit(
                &raw mut *(&raw mut libzahl_tmp_divmod_ds as *mut z_t).offset(i as isize)
                    as *mut zahl,
            );
        }
        libzahl_temp_stack =
            malloc((256 as size_t).wrapping_mul(::core::mem::size_of::<*mut zahl>() as size_t))
                as *mut *mut zahl;
        if libzahl_temp_stack.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
            libzahl_memfailure();
        }
        libzahl_temp_stack_head = libzahl_temp_stack;
        libzahl_temp_stack_end = libzahl_temp_stack.offset(256 as ::core::ffi::c_int as isize);
    }
}
