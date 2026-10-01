extern "C" {
    static mut libzahl_tmp_div: [zahl; 1];
    static mut libzahl_tmp_mod: [zahl; 1];
    fn free(__ptr: *mut ::core::ffi::c_void);
    static mut libzahl_tmp_modmul: z_t;
    static mut libzahl_tmp_ptest_n1: z_t;
    static mut libzahl_tmp_ptest_a: z_t;
    static mut libzahl_tmp_ptest_n4: z_t;
    static mut libzahl_tmp_ptest_x: z_t;
    static mut libzahl_tmp_ptest_d: z_t;
    static mut libzahl_tmp_divmod_d: z_t;
    static mut libzahl_tmp_divmod_b: z_t;
    static mut libzahl_tmp_divmod_a: z_t;
    static mut libzahl_tmp_modsqr: z_t;
    static mut libzahl_tmp_pow_d: z_t;
    static mut libzahl_tmp_pow_c: z_t;
    static mut libzahl_tmp_pow_b: z_t;
    static mut libzahl_tmp_str_num: z_t;
    static mut libzahl_tmp_sub: z_t;
    static mut libzahl_tmp_gcd_v: z_t;
    static mut libzahl_tmp_gcd_u: z_t;
    static mut libzahl_tmp_str_rem: z_t;
    static mut libzahl_tmp_str_div: z_t;
    static mut libzahl_tmp_str_mag: z_t;
    static mut libzahl_tmp_divmod_ds: [z_t; 64];
    static mut libzahl_set_up: ::core::ffi::c_int;
    static mut libzahl_pool: [*mut *mut zahl_char_t; 64];
    static mut libzahl_pool_n: [size_t; 64];
    static mut libzahl_temp_stack: *mut *mut zahl;
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
pub type z_t = [zahl; 1];
pub const ZAHL_BITS_PER_CHAR: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
pub const BITS_PER_CHAR: ::core::ffi::c_int = ZAHL_BITS_PER_CHAR;
#[no_mangle]
pub unsafe extern "C" fn zunsetup() {
    let mut i: size_t = 0;
    if libzahl_set_up != 0 {
        libzahl_set_up = 0 as ::core::ffi::c_int;
        free((*(&raw mut libzahl_tmp_div as *mut zahl)).chars as *mut ::core::ffi::c_void);
        free((*(&raw mut libzahl_tmp_mod as *mut zahl)).chars as *mut ::core::ffi::c_void);
        free((*(&raw mut libzahl_tmp_str_num as *mut zahl)).chars as *mut ::core::ffi::c_void);
        free((*(&raw mut libzahl_tmp_str_mag as *mut zahl)).chars as *mut ::core::ffi::c_void);
        free((*(&raw mut libzahl_tmp_str_div as *mut zahl)).chars as *mut ::core::ffi::c_void);
        free((*(&raw mut libzahl_tmp_str_rem as *mut zahl)).chars as *mut ::core::ffi::c_void);
        free((*(&raw mut libzahl_tmp_gcd_u as *mut zahl)).chars as *mut ::core::ffi::c_void);
        free((*(&raw mut libzahl_tmp_gcd_v as *mut zahl)).chars as *mut ::core::ffi::c_void);
        free((*(&raw mut libzahl_tmp_sub as *mut zahl)).chars as *mut ::core::ffi::c_void);
        free((*(&raw mut libzahl_tmp_modmul as *mut zahl)).chars as *mut ::core::ffi::c_void);
        free((*(&raw mut libzahl_tmp_pow_b as *mut zahl)).chars as *mut ::core::ffi::c_void);
        free((*(&raw mut libzahl_tmp_pow_c as *mut zahl)).chars as *mut ::core::ffi::c_void);
        free((*(&raw mut libzahl_tmp_pow_d as *mut zahl)).chars as *mut ::core::ffi::c_void);
        free((*(&raw mut libzahl_tmp_modsqr as *mut zahl)).chars as *mut ::core::ffi::c_void);
        free((*(&raw mut libzahl_tmp_divmod_a as *mut zahl)).chars as *mut ::core::ffi::c_void);
        free((*(&raw mut libzahl_tmp_divmod_b as *mut zahl)).chars as *mut ::core::ffi::c_void);
        free((*(&raw mut libzahl_tmp_divmod_d as *mut zahl)).chars as *mut ::core::ffi::c_void);
        free((*(&raw mut libzahl_tmp_ptest_x as *mut zahl)).chars as *mut ::core::ffi::c_void);
        free((*(&raw mut libzahl_tmp_ptest_a as *mut zahl)).chars as *mut ::core::ffi::c_void);
        free((*(&raw mut libzahl_tmp_ptest_d as *mut zahl)).chars as *mut ::core::ffi::c_void);
        free((*(&raw mut libzahl_tmp_ptest_n1 as *mut zahl)).chars as *mut ::core::ffi::c_void);
        free((*(&raw mut libzahl_tmp_ptest_n4 as *mut zahl)).chars as *mut ::core::ffi::c_void);
        i = BITS_PER_CHAR as size_t;
        loop {
            let fresh0 = i;
            i = i.wrapping_sub(1);
            if !(fresh0 != 0) {
                break;
            }
            free(
                (*(&raw mut *(&raw mut libzahl_tmp_divmod_ds as *mut z_t).offset(i as isize)
                    as *mut zahl))
                    .chars as *mut ::core::ffi::c_void,
            );
        }
        i = (::core::mem::size_of::<[*mut *mut zahl_char_t; 64]>() as usize)
            .wrapping_div(::core::mem::size_of::<*mut *mut zahl_char_t>() as usize)
            as size_t;
        loop {
            let fresh1 = i;
            i = i.wrapping_sub(1);
            if !(fresh1 != 0) {
                break;
            }
            loop {
                let fresh2 = libzahl_pool_n[i as usize];
                libzahl_pool_n[i as usize] = libzahl_pool_n[i as usize].wrapping_sub(1);
                if !(fresh2 != 0) {
                    break;
                }
                free(
                    *libzahl_pool[i as usize].offset(libzahl_pool_n[i as usize] as isize)
                        as *mut ::core::ffi::c_void,
                );
            }
            free(libzahl_pool[i as usize] as *mut ::core::ffi::c_void);
        }
        free(libzahl_temp_stack as *mut ::core::ffi::c_void);
    }
}
