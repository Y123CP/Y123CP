use core::ffi::*;
pub use crate::src::ffi::*;
pub use crate::src::c_types::*;
extern "C" {
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> c_int;
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    pub __next: *mut __pthread_internal_list,
}
pub type __pthread_list_t = __pthread_internal_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_mutex_s {
    pub __lock: c_int,
    pub __count: c_uint,
    pub __owner: c_int,
    pub __nusers: c_uint,
    pub __kind: c_int,
    pub __spins: c_short,
    pub __elision: c_short,
    pub __list: __pthread_list_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [c_char; 40],
    pub __align: c_long,
}
pub type C2RustUnnamed = c_uint;
pub const PTHREAD_MUTEX_DEFAULT: C2RustUnnamed = 0;
pub const PTHREAD_MUTEX_ERRORCHECK: C2RustUnnamed = 2;
pub const PTHREAD_MUTEX_RECURSIVE: C2RustUnnamed = 1;
pub const PTHREAD_MUTEX_NORMAL: C2RustUnnamed = 0;
pub const PTHREAD_MUTEX_ADAPTIVE_NP: C2RustUnnamed = 3;
pub const PTHREAD_MUTEX_ERRORCHECK_NP: C2RustUnnamed = 2;
pub const PTHREAD_MUTEX_RECURSIVE_NP: C2RustUnnamed = 1;
pub const PTHREAD_MUTEX_TIMED_NP: C2RustUnnamed = 0;
static mut RSECC_mutex: pthread_mutex_t = pthread_mutex_t {
    __data: __pthread_mutex_s {
        __lock: 0 as c_int,
        __count: 0 as c_uint,
        __owner: 0 as c_int,
        __nusers: 0 as c_uint,
        __kind: PTHREAD_MUTEX_TIMED_NP as c_int,
        __spins: 0 as c_short,
        __elision: 0 as c_short,
        __list: __pthread_internal_list {
            __prev: ::core::ptr::null::<__pthread_internal_list>() as *mut __pthread_internal_list,
            __next: ::core::ptr::null::<__pthread_internal_list>() as *mut __pthread_internal_list,
        },
    },
};
static mut initialized: c_int = 0 as c_int;
pub const SYMBOL_SIZE: c_int = 8 as c_int;
pub const symbols: c_uint =
    ((1 as c_uint) << SYMBOL_SIZE).wrapping_sub(1 as c_uint);
static mut proot: c_uint = 0x11d as c_uint;
pub const min_length: c_int = 2 as c_int;
pub const max_length: c_int = 30 as c_int;
static mut alpha: [c_uchar; 256] = [0; 256];
static mut aindex: [c_uchar; 256] = [0; 256];
static mut generator: [[c_uchar; 31]; 29] = [[0; 31]; 29];
static mut generatorInitialized: [c_uchar; 29] = [0; 29];
unsafe extern "C" fn RSECC_initLookupTable() {
    let mut i: c_uint = 0;
    let mut b: c_uint = 0;
    alpha[symbols as usize] = 0 as c_uchar;
    aindex[0 as c_int as usize] = symbols as c_uchar;
    b = 1 as c_uint;
    i = 0 as c_uint;
    while i < symbols {
        alpha[i as usize] = b as c_uchar;
        aindex[b as usize] = i as c_uchar;
        b <<= 1 as c_int;
        if b & symbols.wrapping_add(1 as c_uint) != 0 {
            b ^= proot;
        }
        b &= symbols;
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn RSECC_init() {
    RSECC_initLookupTable();
    memset(
        &raw mut generatorInitialized as *mut c_uchar as *mut c_void,
        0 as c_int,
        (max_length - min_length + 1 as c_int) as size_t,
    );
    initialized = 1 as c_int;
}
unsafe extern "C" fn generator_init(mut length: size_t) {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut g: [c_int; 31] = [0; 31];
    g[0 as c_int as usize] = 1 as c_int;
    i = 0 as size_t;
    while i < length {
        g[i.wrapping_add(1 as size_t) as usize] = 1 as c_int;
        j = i;
        while j > 0 as size_t {
            g[j as usize] = g[j.wrapping_sub(1 as size_t) as usize]
                ^ alpha[(aindex[g[j as usize] as usize] as size_t)
                    .wrapping_add(i)
                    .wrapping_rem(symbols as size_t) as usize]
                    as c_int;
            j = j.wrapping_sub(1);
        }
        g[0 as c_int as usize] =
            alpha[(aindex[g[0 as c_int as usize] as usize] as size_t)
                .wrapping_add(i)
                .wrapping_rem(symbols as size_t) as usize] as c_int;
        i = i.wrapping_add(1);
    }
    i = 0 as size_t;
    while i <= length {
        generator[length.wrapping_sub(min_length as size_t) as usize][i as usize] =
            aindex[g[i as usize] as usize];
        i = i.wrapping_add(1);
    }
    generatorInitialized[length.wrapping_sub(min_length as size_t) as usize] =
        1 as c_uchar;
}
#[no_mangle]
pub unsafe extern "C" fn RSECC_encode(
    mut data_length: size_t,
    mut ecc_length: size_t,
    mut data: *const c_uchar,
    mut ecc: *mut c_uchar,
) -> c_int {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut feedback: c_uchar = 0;
    let mut gen: *mut c_uchar = ::core::ptr::null_mut::<c_uchar>();
    pthread_mutex_lock(&raw mut RSECC_mutex);
    if initialized == 0 {
        RSECC_init();
    }
    pthread_mutex_unlock(&raw mut RSECC_mutex);
    if ecc_length > max_length as size_t {
        return -(1 as c_int);
    }
    memset(
        ecc as *mut c_void,
        0 as c_int,
        ecc_length,
    );
    pthread_mutex_lock(&raw mut RSECC_mutex);
    if generatorInitialized[ecc_length.wrapping_sub(min_length as size_t) as usize] == 0 {
        generator_init(ecc_length);
    }
    pthread_mutex_unlock(&raw mut RSECC_mutex);
    gen = &raw mut *(&raw mut generator as *mut [c_uchar; 31])
        .offset(ecc_length.wrapping_sub(min_length as size_t) as isize)
        as *mut c_uchar;
    i = 0 as size_t;
    while i < data_length {
        feedback = aindex[(*data.offset(i as isize) as c_int
            ^ *ecc.offset(0 as c_int as isize) as c_int)
            as usize];
        if feedback as c_uint != symbols {
            j = 1 as size_t;
            while j < ecc_length {
                let ref mut fresh0 = *ecc.offset(j as isize);
                *fresh0 = (*fresh0 as c_int
                    ^ alpha[((feedback as c_int
                        + *gen.offset(ecc_length.wrapping_sub(j) as isize) as c_int)
                        as c_uint)
                        .wrapping_rem(symbols) as usize]
                        as c_int) as c_uchar;
                j = j.wrapping_add(1);
            }
        }
        memmove(
            ecc.offset(0 as c_int as isize) as *mut c_uchar
                as *mut c_void,
            ecc.offset(1 as c_int as isize) as *mut c_uchar
                as *const c_void,
            ecc_length.wrapping_sub(1 as size_t),
        );
        if feedback as c_uint != symbols {
            *ecc.offset(ecc_length.wrapping_sub(1 as size_t) as isize) =
                alpha[((feedback as c_int
                    + *gen.offset(0 as c_int as isize) as c_int)
                    as c_uint)
                    .wrapping_rem(symbols) as usize];
        } else {
            *ecc.offset(ecc_length.wrapping_sub(1 as size_t) as isize) = 0 as c_uchar;
        }
        i = i.wrapping_add(1);
    }
    return 0 as c_int;
}
