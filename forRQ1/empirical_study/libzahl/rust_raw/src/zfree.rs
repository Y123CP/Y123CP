extern "C" {
    fn realloc(__ptr: *mut ::core::ffi::c_void, __size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    static mut libzahl_pool: [*mut *mut zahl_char_t; 64];
    static mut libzahl_pool_n: [size_t; 64];
    static mut libzahl_pool_alloc: [size_t; 64];
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
#[no_mangle]
pub unsafe extern "C" fn zfree(mut a: *mut zahl) {
    let mut i: size_t = 0;
    let mut x: size_t = 0;
    let mut j: size_t = 0;
    let mut new: *mut *mut zahl_char_t = ::core::ptr::null_mut::<*mut zahl_char_t>();
    if (*a).chars.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        return;
    }
    i = (8 as usize)
        .wrapping_mul(::core::mem::size_of::<::core::ffi::c_ulong>() as usize)
        .wrapping_sub(1 as usize)
        .wrapping_sub((*a).alloced.leading_zeros() as i32 as usize) as size_t;
    let fresh0 = libzahl_pool_n[i as usize];
    libzahl_pool_n[i as usize] = libzahl_pool_n[i as usize].wrapping_add(1);
    j = fresh0;
    if j == libzahl_pool_alloc[i as usize] {
        x = if j != 0 {
            j.wrapping_mul(3 as size_t) >> 1 as ::core::ffi::c_int
        } else {
            128 as size_t
        };
        new = realloc(
            libzahl_pool[i as usize] as *mut ::core::ffi::c_void,
            x.wrapping_mul(::core::mem::size_of::<*mut zahl_char_t>() as size_t),
        ) as *mut *mut zahl_char_t;
        if new.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
            free((*a).chars as *mut ::core::ffi::c_void);
            free(libzahl_pool[i as usize] as *mut ::core::ffi::c_void);
            libzahl_pool_n[i as usize] = 0 as size_t;
            libzahl_pool[i as usize] = ::core::ptr::null_mut::<*mut zahl_char_t>();
            libzahl_pool_alloc[i as usize] = 0 as size_t;
            return;
        }
        libzahl_pool[i as usize] = new;
        libzahl_pool_alloc[i as usize] = x;
    }
    let ref mut fresh1 = *libzahl_pool[i as usize].offset(j as isize);
    *fresh1 = (*a).chars;
}
