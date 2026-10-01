extern "C" {
    fn memmove(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
}
pub type size_t = usize;
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
    pub __lock: ::core::ffi::c_int,
    pub __count: ::core::ffi::c_uint,
    pub __owner: ::core::ffi::c_int,
    pub __nusers: ::core::ffi::c_uint,
    pub __kind: ::core::ffi::c_int,
    pub __spins: ::core::ffi::c_short,
    pub __elision: ::core::ffi::c_short,
    pub __list: __pthread_list_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [::core::ffi::c_char; 40],
    pub __align: ::core::ffi::c_long,
}
pub type C2RustUnnamed = ::core::ffi::c_uint;
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
        __lock: 0 as ::core::ffi::c_int,
        __count: 0 as ::core::ffi::c_uint,
        __owner: 0 as ::core::ffi::c_int,
        __nusers: 0 as ::core::ffi::c_uint,
        __kind: PTHREAD_MUTEX_TIMED_NP as ::core::ffi::c_int,
        __spins: 0 as ::core::ffi::c_short,
        __elision: 0 as ::core::ffi::c_short,
        __list: __pthread_internal_list {
            __prev: ::core::ptr::null::<__pthread_internal_list>() as *mut __pthread_internal_list,
            __next: ::core::ptr::null::<__pthread_internal_list>() as *mut __pthread_internal_list,
        },
    },
};
static mut initialized: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SYMBOL_SIZE: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const symbols: ::core::ffi::c_uint =
    ((1 as ::core::ffi::c_uint) << SYMBOL_SIZE).wrapping_sub(1 as ::core::ffi::c_uint);
static mut proot: ::core::ffi::c_uint = 0x11d as ::core::ffi::c_uint;
pub const min_length: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const max_length: ::core::ffi::c_int = 30 as ::core::ffi::c_int;
static mut alpha: [::core::ffi::c_uchar; 256] = [0; 256];
static mut aindex: [::core::ffi::c_uchar; 256] = [0; 256];
static mut generator: [[::core::ffi::c_uchar; 31]; 29] = [[0; 31]; 29];
static mut generatorInitialized: [::core::ffi::c_uchar; 29] = [0; 29];
unsafe extern "C" fn RSECC_initLookupTable() {
    let mut i: ::core::ffi::c_uint = 0;
    let mut b: ::core::ffi::c_uint = 0;
    alpha[symbols as usize] = 0 as ::core::ffi::c_uchar;
    aindex[0 as ::core::ffi::c_int as usize] = symbols as ::core::ffi::c_uchar;
    b = 1 as ::core::ffi::c_uint;
    i = 0 as ::core::ffi::c_uint;
    while i < symbols {
        alpha[i as usize] = b as ::core::ffi::c_uchar;
        aindex[b as usize] = i as ::core::ffi::c_uchar;
        b <<= 1 as ::core::ffi::c_int;
        if b & symbols.wrapping_add(1 as ::core::ffi::c_uint) != 0 {
            b ^= proot;
        }
        b &= symbols;
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn RSECC_init() {
    RSECC_initLookupTable();
    memset(
        &raw mut generatorInitialized as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        (max_length - min_length + 1 as ::core::ffi::c_int) as size_t,
    );
    initialized = 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn generator_init(mut length: size_t) {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut g: [::core::ffi::c_int; 31] = [0; 31];
    g[0 as ::core::ffi::c_int as usize] = 1 as ::core::ffi::c_int;
    i = 0 as size_t;
    while i < length {
        g[i.wrapping_add(1 as size_t) as usize] = 1 as ::core::ffi::c_int;
        j = i;
        while j > 0 as size_t {
            g[j as usize] = g[j.wrapping_sub(1 as size_t) as usize]
                ^ alpha[(aindex[g[j as usize] as usize] as size_t)
                    .wrapping_add(i)
                    .wrapping_rem(symbols as size_t) as usize]
                    as ::core::ffi::c_int;
            j = j.wrapping_sub(1);
        }
        g[0 as ::core::ffi::c_int as usize] =
            alpha[(aindex[g[0 as ::core::ffi::c_int as usize] as usize] as size_t)
                .wrapping_add(i)
                .wrapping_rem(symbols as size_t) as usize] as ::core::ffi::c_int;
        i = i.wrapping_add(1);
    }
    i = 0 as size_t;
    while i <= length {
        generator[length.wrapping_sub(min_length as size_t) as usize][i as usize] =
            aindex[g[i as usize] as usize];
        i = i.wrapping_add(1);
    }
    generatorInitialized[length.wrapping_sub(min_length as size_t) as usize] =
        1 as ::core::ffi::c_uchar;
}
#[no_mangle]
pub unsafe extern "C" fn RSECC_encode(
    mut data_length: size_t,
    mut ecc_length: size_t,
    mut data: *const ::core::ffi::c_uchar,
    mut ecc: *mut ::core::ffi::c_uchar,
) -> ::core::ffi::c_int {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut feedback: ::core::ffi::c_uchar = 0;
    let mut gen: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    pthread_mutex_lock(&raw mut RSECC_mutex);
    if initialized == 0 {
        RSECC_init();
    }
    pthread_mutex_unlock(&raw mut RSECC_mutex);
    if ecc_length > max_length as size_t {
        return -(1 as ::core::ffi::c_int);
    }
    memset(
        ecc as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ecc_length,
    );
    pthread_mutex_lock(&raw mut RSECC_mutex);
    if generatorInitialized[ecc_length.wrapping_sub(min_length as size_t) as usize] == 0 {
        generator_init(ecc_length);
    }
    pthread_mutex_unlock(&raw mut RSECC_mutex);
    gen = &raw mut *(&raw mut generator as *mut [::core::ffi::c_uchar; 31])
        .offset(ecc_length.wrapping_sub(min_length as size_t) as isize)
        as *mut ::core::ffi::c_uchar;
    i = 0 as size_t;
    while i < data_length {
        feedback = aindex[(*data.offset(i as isize) as ::core::ffi::c_int
            ^ *ecc.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            as usize];
        if feedback as ::core::ffi::c_uint != symbols {
            j = 1 as size_t;
            while j < ecc_length {
                let ref mut fresh0 = *ecc.offset(j as isize);
                *fresh0 = (*fresh0 as ::core::ffi::c_int
                    ^ alpha[((feedback as ::core::ffi::c_int
                        + *gen.offset(ecc_length.wrapping_sub(j) as isize) as ::core::ffi::c_int)
                        as ::core::ffi::c_uint)
                        .wrapping_rem(symbols) as usize]
                        as ::core::ffi::c_int) as ::core::ffi::c_uchar;
                j = j.wrapping_add(1);
            }
        }
        memmove(
            ecc.offset(0 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_uchar
                as *mut ::core::ffi::c_void,
            ecc.offset(1 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_uchar
                as *const ::core::ffi::c_void,
            ecc_length.wrapping_sub(1 as size_t),
        );
        if feedback as ::core::ffi::c_uint != symbols {
            *ecc.offset(ecc_length.wrapping_sub(1 as size_t) as isize) =
                alpha[((feedback as ::core::ffi::c_int
                    + *gen.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                    as ::core::ffi::c_uint)
                    .wrapping_rem(symbols) as usize];
        } else {
            *ecc.offset(ecc_length.wrapping_sub(1 as size_t) as isize) = 0 as ::core::ffi::c_uchar;
        }
        i = i.wrapping_add(1);
    }
    return 0 as ::core::ffi::c_int;
}
