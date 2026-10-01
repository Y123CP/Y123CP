use ::core::arch::asm;
extern "C" {
    fn longjmp(__env: *mut __jmp_buf_tag, __val: ::core::ffi::c_int) -> !;
    fn zfree(_: *mut zahl);
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn realloc(__ptr: *mut ::core::ffi::c_void, __size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    static mut libzahl_jmp_buf: jmp_buf;
    static mut libzahl_error: ::core::ffi::c_int;
    static mut libzahl_pool: [*mut *mut zahl_char_t; 64];
    static mut libzahl_pool_n: [size_t; 64];
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
pub const ZAHL_FLUFF: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
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
            current_block_42 = 18058269489410910971;
        }
        19 => {
            current_block_42 = 18058269489410910971;
        }
        18 => {
            current_block_42 = 9029766137531769078;
        }
        17 => {
            current_block_42 = 13082934213156225962;
        }
        16 => {
            current_block_42 = 8227223884413763093;
        }
        15 => {
            current_block_42 = 3315529572187411795;
        }
        14 => {
            current_block_42 = 10179170827174369810;
        }
        13 => {
            current_block_42 = 15525655650990252292;
        }
        12 => {
            current_block_42 = 4123613682176921222;
        }
        11 => {
            current_block_42 = 3219209932426741323;
        }
        10 => {
            current_block_42 = 4806439557010617502;
        }
        9 => {
            current_block_42 = 9145528991371577241;
        }
        8 => {
            current_block_42 = 3220550624149129156;
        }
        7 => {
            current_block_42 = 15593887086878320509;
        }
        6 => {
            current_block_42 = 161805956404396976;
        }
        5 => {
            current_block_42 = 598157297261247054;
        }
        4 => {
            current_block_42 = 8524442005597201120;
        }
        3 => {
            current_block_42 = 15003697346584100866;
        }
        2 => {
            current_block_42 = 1337185109221498832;
        }
        1 => {
            current_block_42 = 1888233472829453720;
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
        18058269489410910971 => {
            *d.offset((19 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize) =
                *s.offset((19 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize);
            current_block_42 = 9029766137531769078;
        }
        _ => {}
    }
    match current_block_42 {
        9029766137531769078 => {
            *d.offset((18 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize) =
                *s.offset((18 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize);
            current_block_42 = 13082934213156225962;
        }
        _ => {}
    }
    match current_block_42 {
        13082934213156225962 => {
            *d.offset((17 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize) =
                *s.offset((17 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize);
            current_block_42 = 8227223884413763093;
        }
        _ => {}
    }
    match current_block_42 {
        8227223884413763093 => {
            *d.offset((16 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize) =
                *s.offset((16 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize);
            current_block_42 = 3315529572187411795;
        }
        _ => {}
    }
    match current_block_42 {
        3315529572187411795 => {
            *d.offset((15 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize) =
                *s.offset((15 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize);
            current_block_42 = 10179170827174369810;
        }
        _ => {}
    }
    match current_block_42 {
        10179170827174369810 => {
            *d.offset((14 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize) =
                *s.offset((14 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize);
            current_block_42 = 15525655650990252292;
        }
        _ => {}
    }
    match current_block_42 {
        15525655650990252292 => {
            *d.offset((13 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize) =
                *s.offset((13 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize);
            current_block_42 = 4123613682176921222;
        }
        _ => {}
    }
    match current_block_42 {
        4123613682176921222 => {
            *d.offset((12 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize) =
                *s.offset((12 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize);
            current_block_42 = 3219209932426741323;
        }
        _ => {}
    }
    match current_block_42 {
        3219209932426741323 => {
            *d.offset((11 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize) =
                *s.offset((11 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize);
            current_block_42 = 4806439557010617502;
        }
        _ => {}
    }
    match current_block_42 {
        4806439557010617502 => {
            *d.offset((10 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize) =
                *s.offset((10 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize);
            current_block_42 = 9145528991371577241;
        }
        _ => {}
    }
    match current_block_42 {
        9145528991371577241 => {
            *d.offset((9 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize) =
                *s.offset((9 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize);
            current_block_42 = 3220550624149129156;
        }
        _ => {}
    }
    match current_block_42 {
        3220550624149129156 => {
            *d.offset((8 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize) =
                *s.offset((8 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize);
            current_block_42 = 15593887086878320509;
        }
        _ => {}
    }
    match current_block_42 {
        15593887086878320509 => {
            *d.offset((7 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize) =
                *s.offset((7 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize);
            current_block_42 = 161805956404396976;
        }
        _ => {}
    }
    match current_block_42 {
        161805956404396976 => {
            *d.offset((6 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize) =
                *s.offset((6 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize);
            current_block_42 = 598157297261247054;
        }
        _ => {}
    }
    match current_block_42 {
        598157297261247054 => {
            *d.offset((5 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize) =
                *s.offset((5 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize);
            current_block_42 = 8524442005597201120;
        }
        _ => {}
    }
    match current_block_42 {
        8524442005597201120 => {
            *d.offset((4 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize) =
                *s.offset((4 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize);
            current_block_42 = 15003697346584100866;
        }
        _ => {}
    }
    match current_block_42 {
        15003697346584100866 => {
            *d.offset((3 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize) =
                *s.offset((3 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize);
            current_block_42 = 1337185109221498832;
        }
        _ => {}
    }
    match current_block_42 {
        1337185109221498832 => {
            *d.offset((2 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize) =
                *s.offset((2 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize);
            current_block_42 = 1888233472829453720;
        }
        _ => {}
    }
    match current_block_42 {
        1888233472829453720 => {
            *d.offset((1 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize) =
                *s.offset((1 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize);
        }
        _ => {}
    };
}
pub const ENOENT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
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
pub unsafe extern "C" fn libzahl_realloc(mut a: *mut zahl, mut need: size_t) {
    let mut i: size_t = 0;
    let mut new_size: size_t = 1 as size_t;
    let mut new: *mut zahl_char_t = ::core::ptr::null_mut::<zahl_char_t>();
    i = (8 as usize)
        .wrapping_mul(::core::mem::size_of::<::core::ffi::c_ulong>() as usize)
        .wrapping_sub(1 as usize)
        .wrapping_sub(need.leading_zeros() as i32 as usize) as size_t;
    new_size <<= i;
    if (new_size != need) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        i = (i as ::core::ffi::c_ulong).wrapping_add(1 as ::core::ffi::c_ulong) as size_t as size_t;
        new_size <<= 1 as ::core::ffi::c_int;
    }
    if (libzahl_pool_n[i as usize] != 0) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        libzahl_pool_n[i as usize] = libzahl_pool_n[i as usize].wrapping_sub(1);
        new = *libzahl_pool[i as usize].offset(libzahl_pool_n[i as usize] as isize);
        libzahl_memcpy(new, (*a).chars, (*a).alloced);
        zfree(a);
        (*a).chars = new;
    } else {
        (*a).chars = realloc(
            (*a).chars as *mut ::core::ffi::c_void,
            new_size
                .wrapping_add(ZAHL_FLUFF as size_t)
                .wrapping_mul(::core::mem::size_of::<zahl_char_t>() as size_t),
        ) as *mut zahl_char_t;
        if (*a).chars.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
            libzahl_memfailure();
        }
    }
    (*a).alloced = new_size;
}
