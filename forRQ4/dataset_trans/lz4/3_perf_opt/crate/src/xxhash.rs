use core::ffi::*;
pub use crate::src::ffi::*;
pub use crate::src::c_consts::*;
pub use crate::src::c_structs::*;
pub use crate::src::c_types::*;

pub type XXH_alignment = c_uint;
pub const XXH_unaligned: XXH_alignment = 1;
pub const XXH_aligned: XXH_alignment = 0;
pub type XXH_endianness = c_uint;
pub const XXH_littleEndian: XXH_endianness = 1;
pub const XXH_bigEndian: XXH_endianness = 0;
pub type BYTE = uint8_t;

pub use crate::src::lz4hc::C2RustUnnamed_hu47194827;


#[derive(Copy, Clone)]
#[repr(C)]
pub struct XXH32_canonical_t {
    pub digest: [c_uchar; 4],
}

pub const XXH_sa: C2RustUnnamed_htdd24ee73 = 1;
pub type XXH64_hash_t = c_ulonglong;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XXH64_state_s {
    pub total_len: uint64_t,
    pub v1: uint64_t,
    pub v2: uint64_t,
    pub v3: uint64_t,
    pub v4: uint64_t,
    pub mem64: [uint64_t; 4],
    pub memsize: uint32_t,
    pub reserved: [uint32_t; 2],
}
pub type XXH64_state_t = XXH64_state_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XXH64_canonical_t {
    pub digest: [c_uchar; 8],
}

pub const XXH_sa_0: C2RustUnnamed_htdd24ee73 = 1;
pub const XXH_FORCE_NATIVE_FORMAT: c_int = 0 as c_int;
pub const XXH_FORCE_ALIGN_CHECK: c_int = 0 as c_int;

fn XXH_malloc(mut s: size_t) -> *mut c_void { unsafe {
    return malloc(s);
} }
unsafe fn XXH_free(mut p: *mut c_void) {
    free(p);
}
unsafe fn XXH_memcpy(
    mut dest: *mut c_void,
    mut src: *const c_void,
    mut size: size_t,
) -> *mut c_void {
    return memcpy(dest, src, size);
}
pub const __ASSERT_FUNCTION: [c_char; 77] = unsafe {
    ::core::mem::transmute::<[u8; 77], [c_char; 77]>(
        *b"U32 XXH32_finalize(U32, const void *, size_t, XXH_endianness, XXH_alignment)\0",
    )
};
pub const XXH_VERSION_MAJOR: c_int = 0 as c_int;
pub const XXH_VERSION_MINOR: c_int = 6 as c_int;
pub const XXH_VERSION_RELEASE: c_int = 5 as c_int;
pub const XXH_VERSION_NUMBER: c_int =
    XXH_VERSION_MAJOR * 100 as c_int * 100 as c_int
        + XXH_VERSION_MINOR * 100 as c_int
        + XXH_VERSION_RELEASE;
unsafe fn XXH_read32(mut memPtr: *const c_void) -> U32 {
    let mut val: U32 = 0;
    memcpy(
        &raw mut val as *mut c_void,
        memPtr,
        ::core::mem::size_of::<U32>() as size_t,
    );
    return val;
}
fn XXH_swap32(mut x: U32) -> U32 { {
    return x << 24 as c_int & 0xff000000 as U32
        | x << 8 as c_int & 0xff0000 as U32
        | x >> 8 as c_int & 0xff00 as U32
        | x >> 24 as c_int & 0xff as U32;
} }
fn XXH_isLittleEndian() -> c_int { unsafe {
    let one: C2RustUnnamed_hu47194827 = C2RustUnnamed_hu47194827 {
        u: 1 as c_int as U32,
    };
    return one.c[0 as c_int as usize] as c_int;
} }
#[inline(always)]
unsafe fn XXH_readLE32_align(
    mut ptr: *const c_void,
    mut endian: XXH_endianness,
    mut align: XXH_alignment,
) -> U32 {
    if align as c_uint == XXH_unaligned as c_int as c_uint {
        return if endian as c_uint
            == XXH_littleEndian as c_int as c_uint
        {
            XXH_read32(ptr)
        } else {
            XXH_swap32(XXH_read32(ptr))
        };
    } else {
        return if endian as c_uint
            == XXH_littleEndian as c_int as c_uint
        {
            *(ptr as *const U32)
        } else {
            XXH_swap32(*(ptr as *const U32))
        };
    };
}
#[inline(always)]
unsafe fn XXH_readLE32(
    mut ptr: *const c_void,
    mut endian: XXH_endianness,
) -> U32 {
    return XXH_readLE32_align(ptr, endian, XXH_unaligned);
}
unsafe fn XXH_readBE32(mut ptr: *const c_void) -> U32 {
    return if XXH_isLittleEndian() != 0 {
        XXH_swap32(XXH_read32(ptr))
    } else {
        XXH_read32(ptr)
    };
}
#[inline]
pub fn XXH_versionNumber() -> c_uint { {
    return XXH_VERSION_NUMBER as c_uint;
} }
static mut PRIME32_1: U32 = 2654435761 as U32;
static mut PRIME32_2: U32 = 2246822519 as U32;
static mut PRIME32_3: U32 = 3266489917 as U32;
static mut PRIME32_4: U32 = 668265263 as U32;
static mut PRIME32_5: U32 = 374761393 as U32;
fn XXH32_round(mut seed: U32, mut input: U32) -> U32 { unsafe {
    seed = seed.wrapping_add(input.wrapping_mul(PRIME32_2));
    seed = seed << 13 as c_int
        | seed >> 32 as c_int - 13 as c_int;
    seed = seed.wrapping_mul(PRIME32_1);
    return seed;
} }
fn XXH32_avalanche(mut h32: U32) -> U32 { unsafe {
    h32 ^= h32 >> 15 as c_int;
    h32 = h32.wrapping_mul(PRIME32_2);
    h32 ^= h32 >> 13 as c_int;
    h32 = h32.wrapping_mul(PRIME32_3);
    h32 ^= h32 >> 16 as c_int;
    return h32;
} }
unsafe fn XXH32_finalize(
    mut h32: U32,
    mut ptr: *const c_void,
    mut len: size_t,
    mut endian: XXH_endianness,
    mut align: XXH_alignment,
) -> U32 {
    let mut p: *const BYTE = ptr as *const BYTE;
    's_248: {
        let mut current_block_69: u64;
        match len & 15 as size_t {
            12 => {
                h32 = h32.wrapping_add(
                    XXH_readLE32_align(p as *const c_void, endian, align)
                        .wrapping_mul(PRIME32_3),
                );
                p = p.offset(4 as c_int as isize);
                h32 = (h32 << 17 as c_int
                    | h32 >> 32 as c_int - 17 as c_int)
                    .wrapping_mul(PRIME32_4);
                current_block_69 = 16323027167700247563;
            }
            8 => {
                current_block_69 = 16323027167700247563;
            }
            4 => {
                current_block_69 = 17655365834016723002;
            }
            13 => {
                h32 = h32.wrapping_add(
                    XXH_readLE32_align(p as *const c_void, endian, align)
                        .wrapping_mul(PRIME32_3),
                );
                p = p.offset(4 as c_int as isize);
                h32 = (h32 << 17 as c_int
                    | h32 >> 32 as c_int - 17 as c_int)
                    .wrapping_mul(PRIME32_4);
                current_block_69 = 12230907133420509661;
            }
            9 => {
                current_block_69 = 12230907133420509661;
            }
            5 => {
                current_block_69 = 11796471193791360725;
            }
            14 => {
                h32 = h32.wrapping_add(
                    XXH_readLE32_align(p as *const c_void, endian, align)
                        .wrapping_mul(PRIME32_3),
                );
                p = p.offset(4 as c_int as isize);
                h32 = (h32 << 17 as c_int
                    | h32 >> 32 as c_int - 17 as c_int)
                    .wrapping_mul(PRIME32_4);
                current_block_69 = 1324721632189282014;
            }
            10 => {
                current_block_69 = 1324721632189282014;
            }
            6 => {
                current_block_69 = 4549614084285131665;
            }
            15 => {
                h32 = h32.wrapping_add(
                    XXH_readLE32_align(p as *const c_void, endian, align)
                        .wrapping_mul(PRIME32_3),
                );
                p = p.offset(4 as c_int as isize);
                h32 = (h32 << 17 as c_int
                    | h32 >> 32 as c_int - 17 as c_int)
                    .wrapping_mul(PRIME32_4);
                current_block_69 = 4841276931187788602;
            }
            11 => {
                current_block_69 = 4841276931187788602;
            }
            7 => {
                current_block_69 = 13491517230464793282;
            }
            3 => {
                current_block_69 = 18105360017097329158;
            }
            2 => {
                current_block_69 = 4494532374983470909;
            }
            1 => {
                current_block_69 = 11414580168150322258;
            }
            0 => {
                current_block_69 = 10790171032397215446;
            }
            _ => {
                break 's_248;
            }
        }
        match current_block_69 {
            4841276931187788602 => {
                h32 = h32.wrapping_add(
                    XXH_readLE32_align(p as *const c_void, endian, align)
                        .wrapping_mul(PRIME32_3),
                );
                p = p.offset(4 as c_int as isize);
                h32 = (h32 << 17 as c_int
                    | h32 >> 32 as c_int - 17 as c_int)
                    .wrapping_mul(PRIME32_4);
                current_block_69 = 13491517230464793282;
            }
            1324721632189282014 => {
                h32 = h32.wrapping_add(
                    XXH_readLE32_align(p as *const c_void, endian, align)
                        .wrapping_mul(PRIME32_3),
                );
                p = p.offset(4 as c_int as isize);
                h32 = (h32 << 17 as c_int
                    | h32 >> 32 as c_int - 17 as c_int)
                    .wrapping_mul(PRIME32_4);
                current_block_69 = 4549614084285131665;
            }
            12230907133420509661 => {
                h32 = h32.wrapping_add(
                    XXH_readLE32_align(p as *const c_void, endian, align)
                        .wrapping_mul(PRIME32_3),
                );
                p = p.offset(4 as c_int as isize);
                h32 = (h32 << 17 as c_int
                    | h32 >> 32 as c_int - 17 as c_int)
                    .wrapping_mul(PRIME32_4);
                current_block_69 = 11796471193791360725;
            }
            16323027167700247563 => {
                h32 = h32.wrapping_add(
                    XXH_readLE32_align(p as *const c_void, endian, align)
                        .wrapping_mul(PRIME32_3),
                );
                p = p.offset(4 as c_int as isize);
                h32 = (h32 << 17 as c_int
                    | h32 >> 32 as c_int - 17 as c_int)
                    .wrapping_mul(PRIME32_4);
                current_block_69 = 17655365834016723002;
            }
            _ => {}
        }
        match current_block_69 {
            13491517230464793282 => {
                h32 = h32.wrapping_add(
                    XXH_readLE32_align(p as *const c_void, endian, align)
                        .wrapping_mul(PRIME32_3),
                );
                p = p.offset(4 as c_int as isize);
                h32 = (h32 << 17 as c_int
                    | h32 >> 32 as c_int - 17 as c_int)
                    .wrapping_mul(PRIME32_4);
                current_block_69 = 18105360017097329158;
            }
            4549614084285131665 => {
                h32 = h32.wrapping_add(
                    XXH_readLE32_align(p as *const c_void, endian, align)
                        .wrapping_mul(PRIME32_3),
                );
                p = p.offset(4 as c_int as isize);
                h32 = (h32 << 17 as c_int
                    | h32 >> 32 as c_int - 17 as c_int)
                    .wrapping_mul(PRIME32_4);
                let fresh1 = p;
                p = p.offset(1);
                h32 = h32.wrapping_add((*fresh1 as U32).wrapping_mul(PRIME32_5));
                h32 = (h32 << 11 as c_int
                    | h32 >> 32 as c_int - 11 as c_int)
                    .wrapping_mul(PRIME32_1);
                let fresh2 = p;
                p = p.offset(1);
                h32 = h32.wrapping_add((*fresh2 as U32).wrapping_mul(PRIME32_5));
                h32 = (h32 << 11 as c_int
                    | h32 >> 32 as c_int - 11 as c_int)
                    .wrapping_mul(PRIME32_1);
                return XXH32_avalanche(h32);
            }
            11796471193791360725 => {
                h32 = h32.wrapping_add(
                    XXH_readLE32_align(p as *const c_void, endian, align)
                        .wrapping_mul(PRIME32_3),
                );
                p = p.offset(4 as c_int as isize);
                h32 = (h32 << 17 as c_int
                    | h32 >> 32 as c_int - 17 as c_int)
                    .wrapping_mul(PRIME32_4);
                let fresh0 = p;
                p = p.offset(1);
                h32 = h32.wrapping_add((*fresh0 as U32).wrapping_mul(PRIME32_5));
                h32 = (h32 << 11 as c_int
                    | h32 >> 32 as c_int - 11 as c_int)
                    .wrapping_mul(PRIME32_1);
                return XXH32_avalanche(h32);
            }
            17655365834016723002 => {
                h32 = h32.wrapping_add(
                    XXH_readLE32_align(p as *const c_void, endian, align)
                        .wrapping_mul(PRIME32_3),
                );
                p = p.offset(4 as c_int as isize);
                h32 = (h32 << 17 as c_int
                    | h32 >> 32 as c_int - 17 as c_int)
                    .wrapping_mul(PRIME32_4);
                return XXH32_avalanche(h32);
            }
            _ => {}
        }
        match current_block_69 {
            18105360017097329158 => {
                let fresh3 = p;
                p = p.offset(1);
                h32 = h32.wrapping_add((*fresh3 as U32).wrapping_mul(PRIME32_5));
                h32 = (h32 << 11 as c_int
                    | h32 >> 32 as c_int - 11 as c_int)
                    .wrapping_mul(PRIME32_1);
                current_block_69 = 4494532374983470909;
            }
            _ => {}
        }
        match current_block_69 {
            4494532374983470909 => {
                let fresh4 = p;
                p = p.offset(1);
                h32 = h32.wrapping_add((*fresh4 as U32).wrapping_mul(PRIME32_5));
                h32 = (h32 << 11 as c_int
                    | h32 >> 32 as c_int - 11 as c_int)
                    .wrapping_mul(PRIME32_1);
                current_block_69 = 11414580168150322258;
            }
            _ => {}
        }
        match current_block_69 {
            11414580168150322258 => {
                let fresh5 = p;
                p = p.offset(1);
                h32 = h32.wrapping_add((*fresh5 as U32).wrapping_mul(PRIME32_5));
                h32 = (h32 << 11 as c_int
                    | h32 >> 32 as c_int - 11 as c_int)
                    .wrapping_mul(PRIME32_1);
            }
            _ => {}
        }
        return XXH32_avalanche(h32);
    }
    '_c2rust_label: {
        __assert_fail(
            b"0\0" as *const u8 as *const c_char,
            b"lib/xxhash.c\0" as *const u8 as *const c_char,
            346 as c_uint,
            __ASSERT_FUNCTION.as_ptr(),
        );
    };
    return h32;
}
#[inline(always)]
unsafe fn XXH32_endian_align(
    mut input: *const c_void,
    mut len: size_t,
    mut seed: U32,
    mut endian: XXH_endianness,
    mut align: XXH_alignment,
) -> U32 {
    let mut p: *const BYTE = input as *const BYTE;
    let mut bEnd: *const BYTE = p.offset(len as isize);
    let mut h32: U32 = 0;
    if len >= 16 as size_t {
        let limit: *const BYTE = bEnd.offset(-(15 as c_int as isize));
        let mut v1: U32 = seed.wrapping_add(PRIME32_1).wrapping_add(PRIME32_2);
        let mut v2: U32 = seed.wrapping_add(PRIME32_2);
        let mut v3: U32 = seed.wrapping_add(0 as U32);
        let mut v4: U32 = seed.wrapping_sub(PRIME32_1);
        loop {
            v1 = XXH32_round(
                v1,
                XXH_readLE32_align(p as *const c_void, endian, align),
            );
            p = p.offset(4 as c_int as isize);
            v2 = XXH32_round(
                v2,
                XXH_readLE32_align(p as *const c_void, endian, align),
            );
            p = p.offset(4 as c_int as isize);
            v3 = XXH32_round(
                v3,
                XXH_readLE32_align(p as *const c_void, endian, align),
            );
            p = p.offset(4 as c_int as isize);
            v4 = XXH32_round(
                v4,
                XXH_readLE32_align(p as *const c_void, endian, align),
            );
            p = p.offset(4 as c_int as isize);
            if !(p < limit) {
                break;
            }
        }
        h32 = (v1 << 1 as c_int
            | v1 >> 32 as c_int - 1 as c_int)
            .wrapping_add(
                v2 << 7 as c_int
                    | v2 >> 32 as c_int - 7 as c_int,
            )
            .wrapping_add(
                v3 << 12 as c_int
                    | v3 >> 32 as c_int - 12 as c_int,
            )
            .wrapping_add(
                v4 << 18 as c_int
                    | v4 >> 32 as c_int - 18 as c_int,
            );
    } else {
        h32 = seed.wrapping_add(PRIME32_5);
    }
    h32 = h32.wrapping_add(len as U32);
    return XXH32_finalize(
        h32,
        p as *const c_void,
        len & 15 as size_t,
        endian,
        align,
    );
}
#[inline]
pub unsafe fn XXH32(
    mut input: *const c_void,
    mut len: size_t,
    mut seed: c_uint,
) -> XXH32_hash_t {
    let mut endian_detected: XXH_endianness = XXH_isLittleEndian() as XXH_endianness;
    if endian_detected as c_uint
        == XXH_littleEndian as c_int as c_uint
        || XXH_FORCE_NATIVE_FORMAT != 0
    {
        return XXH32_endian_align(input, len, seed as U32, XXH_littleEndian, XXH_unaligned)
            as XXH32_hash_t;
    } else {
        return XXH32_endian_align(input, len, seed as U32, XXH_bigEndian, XXH_unaligned)
            as XXH32_hash_t;
    };
}
#[inline]
pub fn XXH32_createState() -> *mut XXH32_state_t { {
    return XXH_malloc(::core::mem::size_of::<XXH32_state_t>() as size_t) as *mut XXH32_state_t;
} }
#[inline]
pub unsafe fn XXH32_freeState(mut statePtr: *mut XXH32_state_t) -> XXH_errorcode {
    XXH_free(statePtr as *mut c_void);
    return XXH_OK;
}
#[inline]
pub unsafe fn XXH32_copyState(
    mut dstState: *mut XXH32_state_t,
    mut srcState: *const XXH32_state_t,
) {
    memcpy(
        dstState as *mut c_void,
        srcState as *const c_void,
        ::core::mem::size_of::<XXH32_state_t>() as size_t,
    );
}
#[inline]
pub unsafe fn XXH32_reset(
    mut statePtr: *mut XXH32_state_t,
    mut seed: c_uint,
) -> XXH_errorcode {
    let mut state: XXH32_state_t = XXH32_state_s {
        total_len_32: 0,
        large_len: 0,
        v1: 0,
        v2: 0,
        v3: 0,
        v4: 0,
        mem32: [0; 4],
        memsize: 0,
        reserved: 0,
    };
    memset(
        &raw mut state as *mut c_void,
        0 as c_int,
        ::core::mem::size_of::<XXH32_state_t>() as size_t,
    );
    state.v1 = (seed as U32)
        .wrapping_add(PRIME32_1)
        .wrapping_add(PRIME32_2) as uint32_t;
    state.v2 = (seed as U32).wrapping_add(PRIME32_2) as uint32_t;
    state.v3 = seed.wrapping_add(0 as c_uint) as uint32_t;
    state.v4 = (seed as U32).wrapping_sub(PRIME32_1) as uint32_t;
    memcpy(
        statePtr as *mut c_void,
        &raw mut state as *const c_void,
        (::core::mem::size_of::<XXH32_state_t>() as size_t)
            .wrapping_sub(::core::mem::size_of::<uint32_t>() as size_t),
    );
    return XXH_OK;
}
#[inline(always)]
unsafe fn XXH32_update_endian(
    mut state: *mut XXH32_state_t,
    mut input: *const c_void,
    mut len: size_t,
    mut endian: XXH_endianness,
) -> XXH_errorcode {
    if input.is_null() {
        return XXH_ERROR;
    }
    let mut p: *const BYTE = input as *const BYTE;
    let bEnd: *const BYTE = p.offset(len as isize);
    (*state).total_len_32 = ((*state).total_len_32 as c_uint)
        .wrapping_add(len as c_uint) as uint32_t
        as uint32_t;
    (*state).large_len |= ((len >= 16 as size_t) as c_int
        | ((*state).total_len_32 >= 16 as uint32_t) as c_int)
        as uint32_t;
    if ((*state).memsize as size_t).wrapping_add(len) < 16 as size_t {
        XXH_memcpy(
            (&raw mut (*state).mem32 as *mut uint32_t as *mut BYTE)
                .offset((*state).memsize as isize) as *mut c_void,
            input,
            len,
        );
        (*state).memsize = ((*state).memsize as c_uint)
            .wrapping_add(len as c_uint) as uint32_t
            as uint32_t;
        return XXH_OK;
    }
    if (*state).memsize != 0 {
        XXH_memcpy(
            (&raw mut (*state).mem32 as *mut uint32_t as *mut BYTE)
                .offset((*state).memsize as isize) as *mut c_void,
            input,
            (16 as uint32_t).wrapping_sub((*state).memsize) as size_t,
        );
        let mut p32: *const U32 = &raw mut (*state).mem32 as *mut uint32_t;
        (*state).v1 = XXH32_round(
            (*state).v1 as U32,
            XXH_readLE32(p32 as *const c_void, endian),
        ) as uint32_t;
        p32 = p32.offset(1);
        (*state).v2 = XXH32_round(
            (*state).v2 as U32,
            XXH_readLE32(p32 as *const c_void, endian),
        ) as uint32_t;
        p32 = p32.offset(1);
        (*state).v3 = XXH32_round(
            (*state).v3 as U32,
            XXH_readLE32(p32 as *const c_void, endian),
        ) as uint32_t;
        p32 = p32.offset(1);
        (*state).v4 = XXH32_round(
            (*state).v4 as U32,
            XXH_readLE32(p32 as *const c_void, endian),
        ) as uint32_t;
        p = p.offset((16 as uint32_t).wrapping_sub((*state).memsize) as isize);
        (*state).memsize = 0 as uint32_t;
    }
    if p <= bEnd.offset(-(16 as c_int as isize)) {
        let limit: *const BYTE = bEnd.offset(-(16 as c_int as isize));
        let mut v1: U32 = (*state).v1 as U32;
        let mut v2: U32 = (*state).v2 as U32;
        let mut v3: U32 = (*state).v3 as U32;
        let mut v4: U32 = (*state).v4 as U32;
        loop {
            v1 = XXH32_round(v1, XXH_readLE32(p as *const c_void, endian));
            p = p.offset(4 as c_int as isize);
            v2 = XXH32_round(v2, XXH_readLE32(p as *const c_void, endian));
            p = p.offset(4 as c_int as isize);
            v3 = XXH32_round(v3, XXH_readLE32(p as *const c_void, endian));
            p = p.offset(4 as c_int as isize);
            v4 = XXH32_round(v4, XXH_readLE32(p as *const c_void, endian));
            p = p.offset(4 as c_int as isize);
            if !(p <= limit) {
                break;
            }
        }
        (*state).v1 = v1 as uint32_t;
        (*state).v2 = v2 as uint32_t;
        (*state).v3 = v3 as uint32_t;
        (*state).v4 = v4 as uint32_t;
    }
    if p < bEnd {
        XXH_memcpy(
            &raw mut (*state).mem32 as *mut uint32_t as *mut c_void,
            p as *const c_void,
            bEnd.offset_from(p) as c_long as size_t,
        );
        (*state).memsize =
            bEnd.offset_from(p) as c_long as c_uint as uint32_t;
    }
    return XXH_OK;
}
#[inline]
pub unsafe fn XXH32_update(
    mut state_in: *mut XXH32_state_t,
    mut input: *const c_void,
    mut len: size_t,
) -> XXH_errorcode {
    let mut endian_detected: XXH_endianness = XXH_isLittleEndian() as XXH_endianness;
    if endian_detected as c_uint
        == XXH_littleEndian as c_int as c_uint
        || XXH_FORCE_NATIVE_FORMAT != 0
    {
        return XXH32_update_endian(state_in, input, len, XXH_littleEndian);
    } else {
        return XXH32_update_endian(state_in, input, len, XXH_bigEndian);
    };
}
#[inline(always)]
unsafe fn XXH32_digest_endian(
    mut state: *const XXH32_state_t,
    mut endian: XXH_endianness,
) -> U32 {
    let mut h32: U32 = 0;
    if (*state).large_len != 0 {
        h32 = ((*state).v1 << 1 as c_int
            | (*state).v1 >> 32 as c_int - 1 as c_int)
            .wrapping_add(
                (*state).v2 << 7 as c_int
                    | (*state).v2 >> 32 as c_int - 7 as c_int,
            )
            .wrapping_add(
                (*state).v3 << 12 as c_int
                    | (*state).v3 >> 32 as c_int - 12 as c_int,
            )
            .wrapping_add(
                (*state).v4 << 18 as c_int
                    | (*state).v4 >> 32 as c_int - 18 as c_int,
            ) as U32;
    } else {
        h32 = (*state).v3.wrapping_add(PRIME32_5 as uint32_t) as U32;
    }
    h32 = (h32 as uint32_t).wrapping_add((*state).total_len_32) as U32 as U32;
    return XXH32_finalize(
        h32,
        &raw const (*state).mem32 as *const uint32_t as *const c_void,
        (*state).memsize as size_t,
        endian,
        XXH_aligned,
    );
}
#[inline]
pub unsafe fn XXH32_digest(mut state_in: *const XXH32_state_t) -> XXH32_hash_t {
    let mut endian_detected: XXH_endianness = XXH_isLittleEndian() as XXH_endianness;
    if endian_detected as c_uint
        == XXH_littleEndian as c_int as c_uint
        || XXH_FORCE_NATIVE_FORMAT != 0
    {
        return XXH32_digest_endian(state_in, XXH_littleEndian) as XXH32_hash_t;
    } else {
        return XXH32_digest_endian(state_in, XXH_bigEndian) as XXH32_hash_t;
    };
}
#[inline]
pub unsafe fn XXH32_canonicalFromHash(
    mut dst: *mut XXH32_canonical_t,
    mut hash: XXH32_hash_t,
) {
    if XXH_isLittleEndian() != 0 {
        hash = XXH_swap32(hash as U32) as XXH32_hash_t;
    }
    memcpy(
        dst as *mut c_void,
        &raw mut hash as *const c_void,
        ::core::mem::size_of::<XXH32_canonical_t>() as size_t,
    );
}
#[inline]
pub unsafe fn XXH32_hashFromCanonical(
    mut src: *const XXH32_canonical_t,
) -> XXH32_hash_t {
    return XXH_readBE32(src as *const c_void) as XXH32_hash_t;
}
unsafe fn XXH_read64(mut memPtr: *const c_void) -> U64 {
    let mut val: U64 = 0;
    memcpy(
        &raw mut val as *mut c_void,
        memPtr,
        ::core::mem::size_of::<U64>() as size_t,
    );
    return val;
}
fn XXH_swap64(mut x: U64) -> U64 { {
    return ((x << 56 as c_int) as c_ulonglong
        & 0xff00000000000000 as c_ulonglong
        | (x << 40 as c_int) as c_ulonglong
            & 0xff000000000000 as c_ulonglong
        | (x << 24 as c_int) as c_ulonglong
            & 0xff0000000000 as c_ulonglong
        | (x << 8 as c_int) as c_ulonglong
            & 0xff00000000 as c_ulonglong
        | (x >> 8 as c_int) as c_ulonglong
            & 0xff000000 as c_ulonglong
        | (x >> 24 as c_int) as c_ulonglong
            & 0xff0000 as c_ulonglong
        | (x >> 40 as c_int) as c_ulonglong
            & 0xff00 as c_ulonglong
        | (x >> 56 as c_int) as c_ulonglong
            & 0xff as c_ulonglong) as U64;
} }
#[inline(always)]
unsafe fn XXH_readLE64_align(
    mut ptr: *const c_void,
    mut endian: XXH_endianness,
    mut align: XXH_alignment,
) -> U64 {
    if align as c_uint == XXH_unaligned as c_int as c_uint {
        return if endian as c_uint
            == XXH_littleEndian as c_int as c_uint
        {
            XXH_read64(ptr)
        } else {
            XXH_swap64(XXH_read64(ptr))
        };
    } else {
        return if endian as c_uint
            == XXH_littleEndian as c_int as c_uint
        {
            *(ptr as *const U64)
        } else {
            XXH_swap64(*(ptr as *const U64))
        };
    };
}
#[inline(always)]
unsafe fn XXH_readLE64(
    mut ptr: *const c_void,
    mut endian: XXH_endianness,
) -> U64 {
    return XXH_readLE64_align(ptr, endian, XXH_unaligned);
}
unsafe fn XXH_readBE64(mut ptr: *const c_void) -> U64 {
    return if XXH_isLittleEndian() != 0 {
        XXH_swap64(XXH_read64(ptr))
    } else {
        XXH_read64(ptr)
    };
}
static mut PRIME64_1: U64 = 11400714785074694791 as U64;
static mut PRIME64_2: U64 = 14029467366897019727 as U64;
static mut PRIME64_3: U64 = 1609587929392839161 as U64;
static mut PRIME64_4: U64 = 9650029242287828579 as U64;
static mut PRIME64_5: U64 = 2870177450012600261 as U64;
fn XXH64_round(mut acc: U64, mut input: U64) -> U64 { unsafe {
    acc = acc.wrapping_add(input.wrapping_mul(PRIME64_2));
    acc = acc << 31 as c_int
        | acc >> 64 as c_int - 31 as c_int;
    acc = acc.wrapping_mul(PRIME64_1);
    return acc;
} }
fn XXH64_mergeRound(mut acc: U64, mut val: U64) -> U64 { unsafe {
    val = XXH64_round(0 as U64, val);
    acc ^= val;
    acc = acc.wrapping_mul(PRIME64_1).wrapping_add(PRIME64_4);
    return acc;
} }
fn XXH64_avalanche(mut h64: U64) -> U64 { unsafe {
    h64 ^= h64 >> 33 as c_int;
    h64 = h64.wrapping_mul(PRIME64_2);
    h64 ^= h64 >> 29 as c_int;
    h64 = h64.wrapping_mul(PRIME64_3);
    h64 ^= h64 >> 32 as c_int;
    return h64;
} }
unsafe fn XXH64_finalize(
    mut h64: U64,
    mut ptr: *const c_void,
    mut len: size_t,
    mut endian: XXH_endianness,
    mut align: XXH_alignment,
) -> U64 {
    let mut p: *const BYTE = ptr as *const BYTE;
    's_682: {
        let mut current_block_179: u64;
        match len & 31 as size_t {
            24 => {
                let k1: U64 = XXH64_round(
                    0 as U64,
                    XXH_readLE64_align(p as *const c_void, endian, align),
                ) as U64;
                p = p.offset(8 as c_int as isize);
                h64 ^= k1;
                h64 = (h64 << 27 as c_int
                    | h64 >> 64 as c_int - 27 as c_int)
                    .wrapping_mul(PRIME64_1)
                    .wrapping_add(PRIME64_4);
                current_block_179 = 8372684985505791519;
            }
            16 => {
                current_block_179 = 8372684985505791519;
            }
            8 => {
                current_block_179 = 10302601536639739651;
            }
            28 => {
                let k1_2: U64 = XXH64_round(
                    0 as U64,
                    XXH_readLE64_align(p as *const c_void, endian, align),
                ) as U64;
                p = p.offset(8 as c_int as isize);
                h64 ^= k1_2;
                h64 = (h64 << 27 as c_int
                    | h64 >> 64 as c_int - 27 as c_int)
                    .wrapping_mul(PRIME64_1)
                    .wrapping_add(PRIME64_4);
                current_block_179 = 604787549499749060;
            }
            20 => {
                current_block_179 = 604787549499749060;
            }
            12 => {
                current_block_179 = 13715103069806510364;
            }
            4 => {
                current_block_179 = 8535930476165407060;
            }
            25 => {
                let k1_5: U64 = XXH64_round(
                    0 as U64,
                    XXH_readLE64_align(p as *const c_void, endian, align),
                ) as U64;
                p = p.offset(8 as c_int as isize);
                h64 ^= k1_5;
                h64 = (h64 << 27 as c_int
                    | h64 >> 64 as c_int - 27 as c_int)
                    .wrapping_mul(PRIME64_1)
                    .wrapping_add(PRIME64_4);
                current_block_179 = 11262300678209375147;
            }
            17 => {
                current_block_179 = 11262300678209375147;
            }
            9 => {
                current_block_179 = 7906241891543994190;
            }
            29 => {
                let k1_8: U64 = XXH64_round(
                    0 as U64,
                    XXH_readLE64_align(p as *const c_void, endian, align),
                ) as U64;
                p = p.offset(8 as c_int as isize);
                h64 ^= k1_8;
                h64 = (h64 << 27 as c_int
                    | h64 >> 64 as c_int - 27 as c_int)
                    .wrapping_mul(PRIME64_1)
                    .wrapping_add(PRIME64_4);
                current_block_179 = 9794517881464047692;
            }
            21 => {
                current_block_179 = 9794517881464047692;
            }
            13 => {
                current_block_179 = 8995154546670143402;
            }
            5 => {
                current_block_179 = 17816985401463414052;
            }
            26 => {
                let k1_11: U64 = XXH64_round(
                    0 as U64,
                    XXH_readLE64_align(p as *const c_void, endian, align),
                ) as U64;
                p = p.offset(8 as c_int as isize);
                h64 ^= k1_11;
                h64 = (h64 << 27 as c_int
                    | h64 >> 64 as c_int - 27 as c_int)
                    .wrapping_mul(PRIME64_1)
                    .wrapping_add(PRIME64_4);
                current_block_179 = 12313048873678212722;
            }
            18 => {
                current_block_179 = 12313048873678212722;
            }
            10 => {
                current_block_179 = 5286356035089507792;
            }
            30 => {
                let k1_14: U64 = XXH64_round(
                    0 as U64,
                    XXH_readLE64_align(p as *const c_void, endian, align),
                ) as U64;
                p = p.offset(8 as c_int as isize);
                h64 ^= k1_14;
                h64 = (h64 << 27 as c_int
                    | h64 >> 64 as c_int - 27 as c_int)
                    .wrapping_mul(PRIME64_1)
                    .wrapping_add(PRIME64_4);
                current_block_179 = 4686027536167621817;
            }
            22 => {
                current_block_179 = 4686027536167621817;
            }
            14 => {
                current_block_179 = 3087231908759333057;
            }
            6 => {
                current_block_179 = 8840419618977828003;
            }
            27 => {
                let k1_17: U64 = XXH64_round(
                    0 as U64,
                    XXH_readLE64_align(p as *const c_void, endian, align),
                ) as U64;
                p = p.offset(8 as c_int as isize);
                h64 ^= k1_17;
                h64 = (h64 << 27 as c_int
                    | h64 >> 64 as c_int - 27 as c_int)
                    .wrapping_mul(PRIME64_1)
                    .wrapping_add(PRIME64_4);
                current_block_179 = 9657247297953441877;
            }
            19 => {
                current_block_179 = 9657247297953441877;
            }
            11 => {
                current_block_179 = 14466796006854896790;
            }
            31 => {
                let k1_20: U64 = XXH64_round(
                    0 as U64,
                    XXH_readLE64_align(p as *const c_void, endian, align),
                ) as U64;
                p = p.offset(8 as c_int as isize);
                h64 ^= k1_20;
                h64 = (h64 << 27 as c_int
                    | h64 >> 64 as c_int - 27 as c_int)
                    .wrapping_mul(PRIME64_1)
                    .wrapping_add(PRIME64_4);
                current_block_179 = 3186350004348426273;
            }
            23 => {
                current_block_179 = 3186350004348426273;
            }
            15 => {
                current_block_179 = 8098838428831486434;
            }
            7 => {
                current_block_179 = 12092646618281579414;
            }
            3 => {
                current_block_179 = 15619826725282577858;
            }
            2 => {
                current_block_179 = 16691260058094425145;
            }
            1 => {
                current_block_179 = 18377272633697497253;
            }
            0 => {
                current_block_179 = 18008367969863957366;
            }
            _ => {
                break 's_682;
            }
        }
        match current_block_179 {
            8372684985505791519 => {
                let k1_0: U64 = XXH64_round(
                    0 as U64,
                    XXH_readLE64_align(p as *const c_void, endian, align),
                ) as U64;
                p = p.offset(8 as c_int as isize);
                h64 ^= k1_0;
                h64 = (h64 << 27 as c_int
                    | h64 >> 64 as c_int - 27 as c_int)
                    .wrapping_mul(PRIME64_1)
                    .wrapping_add(PRIME64_4);
                current_block_179 = 10302601536639739651;
            }
            604787549499749060 => {
                let k1_3: U64 = XXH64_round(
                    0 as U64,
                    XXH_readLE64_align(p as *const c_void, endian, align),
                ) as U64;
                p = p.offset(8 as c_int as isize);
                h64 ^= k1_3;
                h64 = (h64 << 27 as c_int
                    | h64 >> 64 as c_int - 27 as c_int)
                    .wrapping_mul(PRIME64_1)
                    .wrapping_add(PRIME64_4);
                current_block_179 = 13715103069806510364;
            }
            11262300678209375147 => {
                let k1_6: U64 = XXH64_round(
                    0 as U64,
                    XXH_readLE64_align(p as *const c_void, endian, align),
                ) as U64;
                p = p.offset(8 as c_int as isize);
                h64 ^= k1_6;
                h64 = (h64 << 27 as c_int
                    | h64 >> 64 as c_int - 27 as c_int)
                    .wrapping_mul(PRIME64_1)
                    .wrapping_add(PRIME64_4);
                current_block_179 = 7906241891543994190;
            }
            9794517881464047692 => {
                let k1_9: U64 = XXH64_round(
                    0 as U64,
                    XXH_readLE64_align(p as *const c_void, endian, align),
                ) as U64;
                p = p.offset(8 as c_int as isize);
                h64 ^= k1_9;
                h64 = (h64 << 27 as c_int
                    | h64 >> 64 as c_int - 27 as c_int)
                    .wrapping_mul(PRIME64_1)
                    .wrapping_add(PRIME64_4);
                current_block_179 = 8995154546670143402;
            }
            12313048873678212722 => {
                let k1_12: U64 = XXH64_round(
                    0 as U64,
                    XXH_readLE64_align(p as *const c_void, endian, align),
                ) as U64;
                p = p.offset(8 as c_int as isize);
                h64 ^= k1_12;
                h64 = (h64 << 27 as c_int
                    | h64 >> 64 as c_int - 27 as c_int)
                    .wrapping_mul(PRIME64_1)
                    .wrapping_add(PRIME64_4);
                current_block_179 = 5286356035089507792;
            }
            4686027536167621817 => {
                let k1_15: U64 = XXH64_round(
                    0 as U64,
                    XXH_readLE64_align(p as *const c_void, endian, align),
                ) as U64;
                p = p.offset(8 as c_int as isize);
                h64 ^= k1_15;
                h64 = (h64 << 27 as c_int
                    | h64 >> 64 as c_int - 27 as c_int)
                    .wrapping_mul(PRIME64_1)
                    .wrapping_add(PRIME64_4);
                current_block_179 = 3087231908759333057;
            }
            9657247297953441877 => {
                let k1_18: U64 = XXH64_round(
                    0 as U64,
                    XXH_readLE64_align(p as *const c_void, endian, align),
                ) as U64;
                p = p.offset(8 as c_int as isize);
                h64 ^= k1_18;
                h64 = (h64 << 27 as c_int
                    | h64 >> 64 as c_int - 27 as c_int)
                    .wrapping_mul(PRIME64_1)
                    .wrapping_add(PRIME64_4);
                current_block_179 = 14466796006854896790;
            }
            3186350004348426273 => {
                let k1_21: U64 = XXH64_round(
                    0 as U64,
                    XXH_readLE64_align(p as *const c_void, endian, align),
                ) as U64;
                p = p.offset(8 as c_int as isize);
                h64 ^= k1_21;
                h64 = (h64 << 27 as c_int
                    | h64 >> 64 as c_int - 27 as c_int)
                    .wrapping_mul(PRIME64_1)
                    .wrapping_add(PRIME64_4);
                current_block_179 = 8098838428831486434;
            }
            _ => {}
        }
        match current_block_179 {
            14466796006854896790 => {
                let k1_19: U64 = XXH64_round(
                    0 as U64,
                    XXH_readLE64_align(p as *const c_void, endian, align),
                ) as U64;
                p = p.offset(8 as c_int as isize);
                h64 ^= k1_19;
                h64 = (h64 << 27 as c_int
                    | h64 >> 64 as c_int - 27 as c_int)
                    .wrapping_mul(PRIME64_1)
                    .wrapping_add(PRIME64_4);
                let fresh12 = p;
                p = p.offset(1);
                h64 ^= (*fresh12 as U64).wrapping_mul(PRIME64_5);
                h64 = (h64 << 11 as c_int
                    | h64 >> 64 as c_int - 11 as c_int)
                    .wrapping_mul(PRIME64_1);
                let fresh13 = p;
                p = p.offset(1);
                h64 ^= (*fresh13 as U64).wrapping_mul(PRIME64_5);
                h64 = (h64 << 11 as c_int
                    | h64 >> 64 as c_int - 11 as c_int)
                    .wrapping_mul(PRIME64_1);
                let fresh14 = p;
                p = p.offset(1);
                h64 ^= (*fresh14 as U64).wrapping_mul(PRIME64_5);
                h64 = (h64 << 11 as c_int
                    | h64 >> 64 as c_int - 11 as c_int)
                    .wrapping_mul(PRIME64_1);
                return XXH64_avalanche(h64);
            }
            5286356035089507792 => {
                let k1_13: U64 = XXH64_round(
                    0 as U64,
                    XXH_readLE64_align(p as *const c_void, endian, align),
                ) as U64;
                p = p.offset(8 as c_int as isize);
                h64 ^= k1_13;
                h64 = (h64 << 27 as c_int
                    | h64 >> 64 as c_int - 27 as c_int)
                    .wrapping_mul(PRIME64_1)
                    .wrapping_add(PRIME64_4);
                let fresh8 = p;
                p = p.offset(1);
                h64 ^= (*fresh8 as U64).wrapping_mul(PRIME64_5);
                h64 = (h64 << 11 as c_int
                    | h64 >> 64 as c_int - 11 as c_int)
                    .wrapping_mul(PRIME64_1);
                let fresh9 = p;
                p = p.offset(1);
                h64 ^= (*fresh9 as U64).wrapping_mul(PRIME64_5);
                h64 = (h64 << 11 as c_int
                    | h64 >> 64 as c_int - 11 as c_int)
                    .wrapping_mul(PRIME64_1);
                return XXH64_avalanche(h64);
            }
            7906241891543994190 => {
                let k1_7: U64 = XXH64_round(
                    0 as U64,
                    XXH_readLE64_align(p as *const c_void, endian, align),
                ) as U64;
                p = p.offset(8 as c_int as isize);
                h64 ^= k1_7;
                h64 = (h64 << 27 as c_int
                    | h64 >> 64 as c_int - 27 as c_int)
                    .wrapping_mul(PRIME64_1)
                    .wrapping_add(PRIME64_4);
                let fresh6 = p;
                p = p.offset(1);
                h64 ^= (*fresh6 as U64).wrapping_mul(PRIME64_5);
                h64 = (h64 << 11 as c_int
                    | h64 >> 64 as c_int - 11 as c_int)
                    .wrapping_mul(PRIME64_1);
                return XXH64_avalanche(h64);
            }
            10302601536639739651 => {
                let k1_1: U64 = XXH64_round(
                    0 as U64,
                    XXH_readLE64_align(p as *const c_void, endian, align),
                ) as U64;
                p = p.offset(8 as c_int as isize);
                h64 ^= k1_1;
                h64 = (h64 << 27 as c_int
                    | h64 >> 64 as c_int - 27 as c_int)
                    .wrapping_mul(PRIME64_1)
                    .wrapping_add(PRIME64_4);
                return XXH64_avalanche(h64);
            }
            13715103069806510364 => {
                let k1_4: U64 = XXH64_round(
                    0 as U64,
                    XXH_readLE64_align(p as *const c_void, endian, align),
                ) as U64;
                p = p.offset(8 as c_int as isize);
                h64 ^= k1_4;
                h64 = (h64 << 27 as c_int
                    | h64 >> 64 as c_int - 27 as c_int)
                    .wrapping_mul(PRIME64_1)
                    .wrapping_add(PRIME64_4);
                current_block_179 = 8535930476165407060;
            }
            8995154546670143402 => {
                let k1_10: U64 = XXH64_round(
                    0 as U64,
                    XXH_readLE64_align(p as *const c_void, endian, align),
                ) as U64;
                p = p.offset(8 as c_int as isize);
                h64 ^= k1_10;
                h64 = (h64 << 27 as c_int
                    | h64 >> 64 as c_int - 27 as c_int)
                    .wrapping_mul(PRIME64_1)
                    .wrapping_add(PRIME64_4);
                current_block_179 = 17816985401463414052;
            }
            3087231908759333057 => {
                let k1_16: U64 = XXH64_round(
                    0 as U64,
                    XXH_readLE64_align(p as *const c_void, endian, align),
                ) as U64;
                p = p.offset(8 as c_int as isize);
                h64 ^= k1_16;
                h64 = (h64 << 27 as c_int
                    | h64 >> 64 as c_int - 27 as c_int)
                    .wrapping_mul(PRIME64_1)
                    .wrapping_add(PRIME64_4);
                current_block_179 = 8840419618977828003;
            }
            8098838428831486434 => {
                let k1_22: U64 = XXH64_round(
                    0 as U64,
                    XXH_readLE64_align(p as *const c_void, endian, align),
                ) as U64;
                p = p.offset(8 as c_int as isize);
                h64 ^= k1_22;
                h64 = (h64 << 27 as c_int
                    | h64 >> 64 as c_int - 27 as c_int)
                    .wrapping_mul(PRIME64_1)
                    .wrapping_add(PRIME64_4);
                current_block_179 = 12092646618281579414;
            }
            _ => {}
        }
        match current_block_179 {
            12092646618281579414 => {
                h64 ^= (XXH_readLE32_align(p as *const c_void, endian, align) as U64)
                    .wrapping_mul(PRIME64_1);
                p = p.offset(4 as c_int as isize);
                h64 = (h64 << 23 as c_int
                    | h64 >> 64 as c_int - 23 as c_int)
                    .wrapping_mul(PRIME64_2)
                    .wrapping_add(PRIME64_3);
                current_block_179 = 15619826725282577858;
            }
            8840419618977828003 => {
                h64 ^= (XXH_readLE32_align(p as *const c_void, endian, align) as U64)
                    .wrapping_mul(PRIME64_1);
                p = p.offset(4 as c_int as isize);
                h64 = (h64 << 23 as c_int
                    | h64 >> 64 as c_int - 23 as c_int)
                    .wrapping_mul(PRIME64_2)
                    .wrapping_add(PRIME64_3);
                let fresh10 = p;
                p = p.offset(1);
                h64 ^= (*fresh10 as U64).wrapping_mul(PRIME64_5);
                h64 = (h64 << 11 as c_int
                    | h64 >> 64 as c_int - 11 as c_int)
                    .wrapping_mul(PRIME64_1);
                let fresh11 = p;
                p = p.offset(1);
                h64 ^= (*fresh11 as U64).wrapping_mul(PRIME64_5);
                h64 = (h64 << 11 as c_int
                    | h64 >> 64 as c_int - 11 as c_int)
                    .wrapping_mul(PRIME64_1);
                return XXH64_avalanche(h64);
            }
            17816985401463414052 => {
                h64 ^= (XXH_readLE32_align(p as *const c_void, endian, align) as U64)
                    .wrapping_mul(PRIME64_1);
                p = p.offset(4 as c_int as isize);
                h64 = (h64 << 23 as c_int
                    | h64 >> 64 as c_int - 23 as c_int)
                    .wrapping_mul(PRIME64_2)
                    .wrapping_add(PRIME64_3);
                let fresh7 = p;
                p = p.offset(1);
                h64 ^= (*fresh7 as U64).wrapping_mul(PRIME64_5);
                h64 = (h64 << 11 as c_int
                    | h64 >> 64 as c_int - 11 as c_int)
                    .wrapping_mul(PRIME64_1);
                return XXH64_avalanche(h64);
            }
            8535930476165407060 => {
                h64 ^= (XXH_readLE32_align(p as *const c_void, endian, align) as U64)
                    .wrapping_mul(PRIME64_1);
                p = p.offset(4 as c_int as isize);
                h64 = (h64 << 23 as c_int
                    | h64 >> 64 as c_int - 23 as c_int)
                    .wrapping_mul(PRIME64_2)
                    .wrapping_add(PRIME64_3);
                return XXH64_avalanche(h64);
            }
            _ => {}
        }
        match current_block_179 {
            15619826725282577858 => {
                let fresh15 = p;
                p = p.offset(1);
                h64 ^= (*fresh15 as U64).wrapping_mul(PRIME64_5);
                h64 = (h64 << 11 as c_int
                    | h64 >> 64 as c_int - 11 as c_int)
                    .wrapping_mul(PRIME64_1);
                current_block_179 = 16691260058094425145;
            }
            _ => {}
        }
        match current_block_179 {
            16691260058094425145 => {
                let fresh16 = p;
                p = p.offset(1);
                h64 ^= (*fresh16 as U64).wrapping_mul(PRIME64_5);
                h64 = (h64 << 11 as c_int
                    | h64 >> 64 as c_int - 11 as c_int)
                    .wrapping_mul(PRIME64_1);
                current_block_179 = 18377272633697497253;
            }
            _ => {}
        }
        match current_block_179 {
            18377272633697497253 => {
                let fresh17 = p;
                p = p.offset(1);
                h64 ^= (*fresh17 as U64).wrapping_mul(PRIME64_5);
                h64 = (h64 << 11 as c_int
                    | h64 >> 64 as c_int - 11 as c_int)
                    .wrapping_mul(PRIME64_1);
            }
            _ => {}
        }
        return XXH64_avalanche(h64);
    }
    '_c2rust_label: {
        __assert_fail(
            b"0\0" as *const u8 as *const c_char,
            b"lib/xxhash.c\0" as *const u8 as *const c_char,
            806 as c_uint,
            __ASSERT_FUNCTION.as_ptr(),
        );
    };
    return 0 as U64;
}
#[inline(always)]
unsafe fn XXH64_endian_align(
    mut input: *const c_void,
    mut len: size_t,
    mut seed: U64,
    mut endian: XXH_endianness,
    mut align: XXH_alignment,
) -> U64 {
    let mut p: *const BYTE = input as *const BYTE;
    let mut bEnd: *const BYTE = p.offset(len as isize);
    let mut h64: U64 = 0;
    if len >= 32 as size_t {
        let limit: *const BYTE = bEnd.offset(-(32 as c_int as isize));
        let mut v1: U64 = seed.wrapping_add(PRIME64_1).wrapping_add(PRIME64_2);
        let mut v2: U64 = seed.wrapping_add(PRIME64_2);
        let mut v3: U64 = seed.wrapping_add(0 as U64);
        let mut v4: U64 = seed.wrapping_sub(PRIME64_1);
        loop {
            v1 = XXH64_round(
                v1,
                XXH_readLE64_align(p as *const c_void, endian, align),
            );
            p = p.offset(8 as c_int as isize);
            v2 = XXH64_round(
                v2,
                XXH_readLE64_align(p as *const c_void, endian, align),
            );
            p = p.offset(8 as c_int as isize);
            v3 = XXH64_round(
                v3,
                XXH_readLE64_align(p as *const c_void, endian, align),
            );
            p = p.offset(8 as c_int as isize);
            v4 = XXH64_round(
                v4,
                XXH_readLE64_align(p as *const c_void, endian, align),
            );
            p = p.offset(8 as c_int as isize);
            if !(p <= limit) {
                break;
            }
        }
        h64 = (v1 << 1 as c_int
            | v1 >> 64 as c_int - 1 as c_int)
            .wrapping_add(
                v2 << 7 as c_int
                    | v2 >> 64 as c_int - 7 as c_int,
            )
            .wrapping_add(
                v3 << 12 as c_int
                    | v3 >> 64 as c_int - 12 as c_int,
            )
            .wrapping_add(
                v4 << 18 as c_int
                    | v4 >> 64 as c_int - 18 as c_int,
            );
        h64 = XXH64_mergeRound(h64, v1);
        h64 = XXH64_mergeRound(h64, v2);
        h64 = XXH64_mergeRound(h64, v3);
        h64 = XXH64_mergeRound(h64, v4);
    } else {
        h64 = seed.wrapping_add(PRIME64_5);
    }
    h64 = h64.wrapping_add(len as U64);
    return XXH64_finalize(h64, p as *const c_void, len, endian, align);
}
#[inline]
pub unsafe fn XXH64(
    mut input: *const c_void,
    mut len: size_t,
    mut seed: c_ulonglong,
) -> XXH64_hash_t {
    let mut endian_detected: XXH_endianness = XXH_isLittleEndian() as XXH_endianness;
    if endian_detected as c_uint
        == XXH_littleEndian as c_int as c_uint
        || XXH_FORCE_NATIVE_FORMAT != 0
    {
        return XXH64_endian_align(input, len, seed as U64, XXH_littleEndian, XXH_unaligned)
            as XXH64_hash_t;
    } else {
        return XXH64_endian_align(input, len, seed as U64, XXH_bigEndian, XXH_unaligned)
            as XXH64_hash_t;
    };
}
#[inline]
pub fn XXH64_createState() -> *mut XXH64_state_t { {
    return XXH_malloc(::core::mem::size_of::<XXH64_state_t>() as size_t) as *mut XXH64_state_t;
} }
#[inline]
pub unsafe fn XXH64_freeState(mut statePtr: *mut XXH64_state_t) -> XXH_errorcode {
    XXH_free(statePtr as *mut c_void);
    return XXH_OK;
}
#[inline]
pub unsafe fn XXH64_copyState(
    mut dstState: *mut XXH64_state_t,
    mut srcState: *const XXH64_state_t,
) {
    memcpy(
        dstState as *mut c_void,
        srcState as *const c_void,
        ::core::mem::size_of::<XXH64_state_t>() as size_t,
    );
}
#[inline]
pub unsafe fn XXH64_reset(
    mut statePtr: *mut XXH64_state_t,
    mut seed: c_ulonglong,
) -> XXH_errorcode {
    let mut state: XXH64_state_t = XXH64_state_s {
        total_len: 0,
        v1: 0,
        v2: 0,
        v3: 0,
        v4: 0,
        mem64: [0; 4],
        memsize: 0,
        reserved: [0; 2],
    };
    memset(
        &raw mut state as *mut c_void,
        0 as c_int,
        ::core::mem::size_of::<XXH64_state_t>() as size_t,
    );
    state.v1 = seed
        .wrapping_add(PRIME64_1 as c_ulonglong)
        .wrapping_add(PRIME64_2 as c_ulonglong) as uint64_t;
    state.v2 = seed.wrapping_add(PRIME64_2 as c_ulonglong) as uint64_t;
    state.v3 = seed.wrapping_add(0 as c_ulonglong) as uint64_t;
    state.v4 = seed.wrapping_sub(PRIME64_1 as c_ulonglong) as uint64_t;
    memcpy(
        statePtr as *mut c_void,
        &raw mut state as *const c_void,
        (::core::mem::size_of::<XXH64_state_t>() as size_t)
            .wrapping_sub(::core::mem::size_of::<[uint32_t; 2]>() as size_t),
    );
    return XXH_OK;
}
#[inline(always)]
unsafe fn XXH64_update_endian(
    mut state: *mut XXH64_state_t,
    mut input: *const c_void,
    mut len: size_t,
    mut endian: XXH_endianness,
) -> XXH_errorcode {
    if input.is_null() {
        return XXH_ERROR;
    }
    let mut p: *const BYTE = input as *const BYTE;
    let bEnd: *const BYTE = p.offset(len as isize);
    (*state).total_len = ((*state).total_len as c_ulong)
        .wrapping_add(len as c_ulong) as uint64_t as uint64_t;
    if ((*state).memsize as size_t).wrapping_add(len) < 32 as size_t {
        XXH_memcpy(
            (&raw mut (*state).mem64 as *mut uint64_t as *mut BYTE)
                .offset((*state).memsize as isize) as *mut c_void,
            input,
            len,
        );
        (*state).memsize = (*state).memsize.wrapping_add(len as U32 as uint32_t);
        return XXH_OK;
    }
    if (*state).memsize != 0 {
        XXH_memcpy(
            (&raw mut (*state).mem64 as *mut uint64_t as *mut BYTE)
                .offset((*state).memsize as isize) as *mut c_void,
            input,
            (32 as uint32_t).wrapping_sub((*state).memsize) as size_t,
        );
        (*state).v1 = XXH64_round(
            (*state).v1 as U64,
            XXH_readLE64(
                (&raw mut (*state).mem64 as *mut uint64_t).offset(0 as c_int as isize)
                    as *const c_void,
                endian,
            ),
        ) as uint64_t;
        (*state).v2 = XXH64_round(
            (*state).v2 as U64,
            XXH_readLE64(
                (&raw mut (*state).mem64 as *mut uint64_t).offset(1 as c_int as isize)
                    as *const c_void,
                endian,
            ),
        ) as uint64_t;
        (*state).v3 = XXH64_round(
            (*state).v3 as U64,
            XXH_readLE64(
                (&raw mut (*state).mem64 as *mut uint64_t).offset(2 as c_int as isize)
                    as *const c_void,
                endian,
            ),
        ) as uint64_t;
        (*state).v4 = XXH64_round(
            (*state).v4 as U64,
            XXH_readLE64(
                (&raw mut (*state).mem64 as *mut uint64_t).offset(3 as c_int as isize)
                    as *const c_void,
                endian,
            ),
        ) as uint64_t;
        p = p.offset((32 as uint32_t).wrapping_sub((*state).memsize) as isize);
        (*state).memsize = 0 as uint32_t;
    }
    if p.offset(32 as c_int as isize) <= bEnd {
        let limit: *const BYTE = bEnd.offset(-(32 as c_int as isize));
        let mut v1: U64 = (*state).v1 as U64;
        let mut v2: U64 = (*state).v2 as U64;
        let mut v3: U64 = (*state).v3 as U64;
        let mut v4: U64 = (*state).v4 as U64;
        loop {
            v1 = XXH64_round(v1, XXH_readLE64(p as *const c_void, endian));
            p = p.offset(8 as c_int as isize);
            v2 = XXH64_round(v2, XXH_readLE64(p as *const c_void, endian));
            p = p.offset(8 as c_int as isize);
            v3 = XXH64_round(v3, XXH_readLE64(p as *const c_void, endian));
            p = p.offset(8 as c_int as isize);
            v4 = XXH64_round(v4, XXH_readLE64(p as *const c_void, endian));
            p = p.offset(8 as c_int as isize);
            if !(p <= limit) {
                break;
            }
        }
        (*state).v1 = v1 as uint64_t;
        (*state).v2 = v2 as uint64_t;
        (*state).v3 = v3 as uint64_t;
        (*state).v4 = v4 as uint64_t;
    }
    if p < bEnd {
        XXH_memcpy(
            &raw mut (*state).mem64 as *mut uint64_t as *mut c_void,
            p as *const c_void,
            bEnd.offset_from(p) as c_long as size_t,
        );
        (*state).memsize =
            bEnd.offset_from(p) as c_long as c_uint as uint32_t;
    }
    return XXH_OK;
}
#[inline]
pub unsafe fn XXH64_update(
    mut state_in: *mut XXH64_state_t,
    mut input: *const c_void,
    mut len: size_t,
) -> XXH_errorcode {
    let mut endian_detected: XXH_endianness = XXH_isLittleEndian() as XXH_endianness;
    if endian_detected as c_uint
        == XXH_littleEndian as c_int as c_uint
        || XXH_FORCE_NATIVE_FORMAT != 0
    {
        return XXH64_update_endian(state_in, input, len, XXH_littleEndian);
    } else {
        return XXH64_update_endian(state_in, input, len, XXH_bigEndian);
    };
}
#[inline(always)]
unsafe fn XXH64_digest_endian(
    mut state: *const XXH64_state_t,
    mut endian: XXH_endianness,
) -> U64 {
    let mut h64: U64 = 0;
    if (*state).total_len >= 32 as uint64_t {
        let v1: U64 = (*state).v1 as U64;
        let v2: U64 = (*state).v2 as U64;
        let v3: U64 = (*state).v3 as U64;
        let v4: U64 = (*state).v4 as U64;
        h64 = (v1 << 1 as c_int
            | v1 >> 64 as c_int - 1 as c_int)
            .wrapping_add(
                v2 << 7 as c_int
                    | v2 >> 64 as c_int - 7 as c_int,
            )
            .wrapping_add(
                v3 << 12 as c_int
                    | v3 >> 64 as c_int - 12 as c_int,
            )
            .wrapping_add(
                v4 << 18 as c_int
                    | v4 >> 64 as c_int - 18 as c_int,
            );
        h64 = XXH64_mergeRound(h64, v1);
        h64 = XXH64_mergeRound(h64, v2);
        h64 = XXH64_mergeRound(h64, v3);
        h64 = XXH64_mergeRound(h64, v4);
    } else {
        h64 = (*state).v3.wrapping_add(PRIME64_5 as uint64_t) as U64;
    }
    h64 = h64.wrapping_add((*state).total_len);
    return XXH64_finalize(
        h64,
        &raw const (*state).mem64 as *const uint64_t as *const c_void,
        (*state).total_len as size_t,
        endian,
        XXH_aligned,
    );
}
#[inline]
pub unsafe fn XXH64_digest(mut state_in: *const XXH64_state_t) -> XXH64_hash_t {
    let mut endian_detected: XXH_endianness = XXH_isLittleEndian() as XXH_endianness;
    if endian_detected as c_uint
        == XXH_littleEndian as c_int as c_uint
        || XXH_FORCE_NATIVE_FORMAT != 0
    {
        return XXH64_digest_endian(state_in, XXH_littleEndian) as XXH64_hash_t;
    } else {
        return XXH64_digest_endian(state_in, XXH_bigEndian) as XXH64_hash_t;
    };
}
#[inline]
pub unsafe fn XXH64_canonicalFromHash(
    mut dst: *mut XXH64_canonical_t,
    mut hash: XXH64_hash_t,
) {
    if XXH_isLittleEndian() != 0 {
        hash = XXH_swap64(hash as U64) as XXH64_hash_t;
    }
    memcpy(
        dst as *mut c_void,
        &raw mut hash as *const c_void,
        ::core::mem::size_of::<XXH64_canonical_t>() as size_t,
    );
}
#[inline]
pub unsafe fn XXH64_hashFromCanonical(
    mut src: *const XXH64_canonical_t,
) -> XXH64_hash_t {
    return XXH_readBE64(src as *const c_void) as XXH64_hash_t;
}
