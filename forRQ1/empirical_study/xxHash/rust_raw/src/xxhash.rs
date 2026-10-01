extern "C" {
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memcmp(
        __s1: *const ::core::ffi::c_void,
        __s2: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> ::core::ffi::c_int;
}
pub type __uint128_t = u128;
pub type size_t = usize;
pub type XXH_errorcode = ::core::ffi::c_uint;
pub const XXH_ERROR: XXH_errorcode = 1;
pub const XXH_OK: XXH_errorcode = 0;
pub type __uint8_t = u8;
pub type __uint32_t = u32;
pub type __uint64_t = u64;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type XXH32_hash_t = uint32_t;
pub type xxh_u32 = XXH32_hash_t;
pub type XXH_alignment = ::core::ffi::c_uint;
pub const XXH_unaligned: XXH_alignment = 1;
pub const XXH_aligned: XXH_alignment = 0;
pub type xxh_u8 = uint8_t;
pub type xxh_unalign32 = xxh_u32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XXH32_state_s {
    pub total_len_32: XXH32_hash_t,
    pub large_len: XXH32_hash_t,
    pub acc: [XXH32_hash_t; 4],
    pub buffer: [::core::ffi::c_uchar; 16],
    pub bufferedSize: XXH32_hash_t,
    pub reserved: XXH32_hash_t,
}
pub type XXH32_state_t = XXH32_state_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XXH32_canonical_t {
    pub digest: [::core::ffi::c_uchar; 4],
}
pub type XXH64_hash_t = uint64_t;
pub type xxh_u64 = XXH64_hash_t;
pub type xxh_unalign64 = xxh_u64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XXH64_state_s {
    pub total_len: XXH64_hash_t,
    pub acc: [XXH64_hash_t; 4],
    pub buffer: [::core::ffi::c_uchar; 32],
    pub bufferedSize: XXH32_hash_t,
    pub reserved32: XXH32_hash_t,
    pub reserved64: XXH64_hash_t,
}
pub type XXH64_state_t = XXH64_state_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XXH64_canonical_t {
    pub digest: [::core::ffi::c_uchar; 8],
}
pub type XXH3_f_scrambleAcc =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *const ::core::ffi::c_void) -> ()>;
pub type XXH3_f_accumulate =
    Option<unsafe extern "C" fn(*mut xxh_u64, *const xxh_u8, *const xxh_u8, size_t) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XXH128_hash_t {
    pub low64: XXH64_hash_t,
    pub high64: XXH64_hash_t,
}
pub type XXH3_hashLong64_f = Option<
    unsafe extern "C" fn(
        *const ::core::ffi::c_void,
        size_t,
        XXH64_hash_t,
        *const xxh_u8,
        size_t,
    ) -> XXH64_hash_t,
>;
pub type XXH3_f_initCustomSecret =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, xxh_u64) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XXH3_state_s {
    pub acc: [XXH64_hash_t; 8],
    pub customSecret: [::core::ffi::c_uchar; 192],
    pub buffer: [::core::ffi::c_uchar; 256],
    pub bufferedSize: XXH32_hash_t,
    pub useSeed: XXH32_hash_t,
    pub nbStripesSoFar: size_t,
    pub totalLen: XXH64_hash_t,
    pub nbStripesPerBlock: size_t,
    pub secretLimit: size_t,
    pub seed: XXH64_hash_t,
    pub reserved64: XXH64_hash_t,
    pub extSecret: *const ::core::ffi::c_uchar,
}
pub type XXH3_state_t = XXH3_state_s;
pub type XXH3_hashLong128_f = Option<
    unsafe extern "C" fn(
        *const ::core::ffi::c_void,
        size_t,
        XXH64_hash_t,
        *const ::core::ffi::c_void,
        size_t,
    ) -> XXH128_hash_t,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XXH128_canonical_t {
    pub digest: [::core::ffi::c_uchar; 16],
}
pub const XXH_VERSION_MAJOR: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const XXH_VERSION_MINOR: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const XXH_VERSION_RELEASE: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const XXH_VERSION_NUMBER: ::core::ffi::c_int =
    XXH_VERSION_MAJOR * 100 as ::core::ffi::c_int * 100 as ::core::ffi::c_int
        + XXH_VERSION_MINOR * 100 as ::core::ffi::c_int
        + XXH_VERSION_RELEASE;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const XXH3_SECRET_SIZE_MIN: ::core::ffi::c_int = 136 as ::core::ffi::c_int;
pub const XXH3_INTERNALBUFFER_SIZE: ::core::ffi::c_int = 256 as ::core::ffi::c_int;
pub const XXH3_MIDSIZE_MAX: ::core::ffi::c_int = 240 as ::core::ffi::c_int;
pub const XXH_FORCE_ALIGN_CHECK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const XXH32_ENDJMP: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
unsafe extern "C" fn XXH_malloc(mut s: size_t) -> *mut ::core::ffi::c_void {
    return malloc(s);
}
unsafe extern "C" fn XXH_free(mut p: *mut ::core::ffi::c_void) {
    free(p);
}
unsafe extern "C" fn XXH_read32(mut ptr: *const ::core::ffi::c_void) -> xxh_u32 {
    return *(ptr as *const xxh_unalign32);
}
pub const XXH_CPU_LITTLE_ENDIAN: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
unsafe extern "C" fn XXH_swap32(mut x: xxh_u32) -> xxh_u32 {
    return x << 24 as ::core::ffi::c_int & 0xff000000 as xxh_u32
        | x << 8 as ::core::ffi::c_int & 0xff0000 as xxh_u32
        | x >> 8 as ::core::ffi::c_int & 0xff00 as xxh_u32
        | x >> 24 as ::core::ffi::c_int & 0xff as xxh_u32;
}
unsafe extern "C" fn XXH_readLE32(mut ptr: *const ::core::ffi::c_void) -> xxh_u32 {
    return if XXH_CPU_LITTLE_ENDIAN != 0 {
        XXH_read32(ptr)
    } else {
        XXH_swap32(XXH_read32(ptr))
    };
}
unsafe extern "C" fn XXH_readBE32(mut ptr: *const ::core::ffi::c_void) -> xxh_u32 {
    return if XXH_CPU_LITTLE_ENDIAN != 0 {
        XXH_swap32(XXH_read32(ptr))
    } else {
        XXH_read32(ptr)
    };
}
unsafe extern "C" fn XXH_readLE32_align(
    mut ptr: *const ::core::ffi::c_void,
    mut align: XXH_alignment,
) -> xxh_u32 {
    if align as ::core::ffi::c_uint == XXH_unaligned as ::core::ffi::c_int as ::core::ffi::c_uint {
        return XXH_readLE32(ptr);
    } else {
        return if XXH_CPU_LITTLE_ENDIAN != 0 {
            *(ptr as *const xxh_u32)
        } else {
            XXH_swap32(*(ptr as *const xxh_u32))
        };
    };
}
#[no_mangle]
pub unsafe extern "C" fn XXH_versionNumber() -> ::core::ffi::c_uint {
    return XXH_VERSION_NUMBER as ::core::ffi::c_uint;
}
pub const XXH_PRIME32_1: ::core::ffi::c_uint = 0x9e3779b1 as ::core::ffi::c_uint;
pub const XXH_PRIME32_2: ::core::ffi::c_uint = 0x85ebca77 as ::core::ffi::c_uint;
pub const XXH_PRIME32_3: ::core::ffi::c_uint = 0xc2b2ae3d as ::core::ffi::c_uint;
pub const XXH_PRIME32_4: ::core::ffi::c_uint = 0x27d4eb2f as ::core::ffi::c_uint;
pub const XXH_PRIME32_5: ::core::ffi::c_uint = 0x165667b1 as ::core::ffi::c_uint;
unsafe extern "C" fn XXH32_round(mut acc: xxh_u32, mut input: xxh_u32) -> xxh_u32 {
    acc = (acc as ::core::ffi::c_uint)
        .wrapping_add(input.wrapping_mul(XXH_PRIME32_2 as xxh_u32) as ::core::ffi::c_uint)
        as xxh_u32 as xxh_u32;
    acc = acc.rotate_left(13 as ::core::ffi::c_int as ::core::ffi::c_uint as u32) as xxh_u32;
    acc = (acc as ::core::ffi::c_uint).wrapping_mul(XXH_PRIME32_1) as xxh_u32 as xxh_u32;
    return acc;
}
unsafe extern "C" fn XXH32_avalanche(mut hash: xxh_u32) -> xxh_u32 {
    hash ^= hash >> 15 as ::core::ffi::c_int;
    hash = (hash as ::core::ffi::c_uint).wrapping_mul(XXH_PRIME32_2) as xxh_u32 as xxh_u32;
    hash ^= hash >> 13 as ::core::ffi::c_int;
    hash = (hash as ::core::ffi::c_uint).wrapping_mul(XXH_PRIME32_3) as xxh_u32 as xxh_u32;
    hash ^= hash >> 16 as ::core::ffi::c_int;
    return hash;
}
unsafe extern "C" fn XXH32_initAccs(mut acc: *mut xxh_u32, mut seed: xxh_u32) {
    *acc.offset(0 as ::core::ffi::c_int as isize) = seed
        .wrapping_add(XXH_PRIME32_1 as xxh_u32)
        .wrapping_add(XXH_PRIME32_2 as xxh_u32);
    *acc.offset(1 as ::core::ffi::c_int as isize) = seed.wrapping_add(XXH_PRIME32_2 as xxh_u32);
    *acc.offset(2 as ::core::ffi::c_int as isize) = seed.wrapping_add(0 as xxh_u32);
    *acc.offset(3 as ::core::ffi::c_int as isize) = seed.wrapping_sub(XXH_PRIME32_1 as xxh_u32);
}
unsafe extern "C" fn XXH32_consumeLong(
    mut acc: *mut xxh_u32,
    mut input: *const xxh_u8,
    mut len: size_t,
    mut align: XXH_alignment,
) -> *const xxh_u8 {
    let bEnd: *const xxh_u8 = input.offset(len as isize);
    let limit: *const xxh_u8 = bEnd.offset(-(15 as ::core::ffi::c_int as isize));
    loop {
        *acc.offset(0 as ::core::ffi::c_int as isize) = XXH32_round(
            *acc.offset(0 as ::core::ffi::c_int as isize),
            XXH_readLE32_align(input as *const ::core::ffi::c_void, align),
        );
        input = input.offset(4 as ::core::ffi::c_int as isize);
        *acc.offset(1 as ::core::ffi::c_int as isize) = XXH32_round(
            *acc.offset(1 as ::core::ffi::c_int as isize),
            XXH_readLE32_align(input as *const ::core::ffi::c_void, align),
        );
        input = input.offset(4 as ::core::ffi::c_int as isize);
        *acc.offset(2 as ::core::ffi::c_int as isize) = XXH32_round(
            *acc.offset(2 as ::core::ffi::c_int as isize),
            XXH_readLE32_align(input as *const ::core::ffi::c_void, align),
        );
        input = input.offset(4 as ::core::ffi::c_int as isize);
        *acc.offset(3 as ::core::ffi::c_int as isize) = XXH32_round(
            *acc.offset(3 as ::core::ffi::c_int as isize),
            XXH_readLE32_align(input as *const ::core::ffi::c_void, align),
        );
        input = input.offset(4 as ::core::ffi::c_int as isize);
        if !(input < limit) {
            break;
        }
    }
    return input;
}
unsafe extern "C" fn XXH32_mergeAccs(mut acc: *const xxh_u32) -> xxh_u32 {
    return (*acc.offset(0 as ::core::ffi::c_int as isize))
        .rotate_left(1 as ::core::ffi::c_int as ::core::ffi::c_uint as u32)
        .wrapping_add(
            (*acc.offset(1 as ::core::ffi::c_int as isize))
                .rotate_left(7 as ::core::ffi::c_int as ::core::ffi::c_uint as u32),
        )
        .wrapping_add(
            (*acc.offset(2 as ::core::ffi::c_int as isize))
                .rotate_left(12 as ::core::ffi::c_int as ::core::ffi::c_uint as u32),
        )
        .wrapping_add(
            (*acc.offset(3 as ::core::ffi::c_int as isize))
                .rotate_left(18 as ::core::ffi::c_int as ::core::ffi::c_uint as u32),
        );
}
unsafe extern "C" fn XXH32_finalize(
    mut hash: xxh_u32,
    mut ptr: *const xxh_u8,
    mut len: size_t,
    mut align: XXH_alignment,
) -> xxh_u32 {
    ptr.is_null();
    if XXH32_ENDJMP == 0 {
        len &= 15 as size_t;
        while len >= 4 as size_t {
            hash = (hash as ::core::ffi::c_uint).wrapping_add(
                XXH_readLE32_align(ptr as *const ::core::ffi::c_void, align)
                    .wrapping_mul(XXH_PRIME32_3 as xxh_u32) as ::core::ffi::c_uint,
            ) as xxh_u32 as xxh_u32;
            ptr = ptr.offset(4 as ::core::ffi::c_int as isize);
            hash = hash
                .rotate_left(17 as ::core::ffi::c_int as ::core::ffi::c_uint as u32)
                .wrapping_mul(XXH_PRIME32_4) as xxh_u32;
            len = len.wrapping_sub(4 as size_t);
        }
        while len > 0 as size_t {
            let fresh0 = ptr;
            ptr = ptr.offset(1);
            hash = (hash as ::core::ffi::c_uint)
                .wrapping_add((*fresh0 as ::core::ffi::c_uint).wrapping_mul(XXH_PRIME32_5))
                as xxh_u32 as xxh_u32;
            hash = hash
                .rotate_left(11 as ::core::ffi::c_int as ::core::ffi::c_uint as u32)
                .wrapping_mul(XXH_PRIME32_1) as xxh_u32;
            len = len.wrapping_sub(1);
        }
        return XXH32_avalanche(hash);
    } else {
        's_489: {
            let mut current_block_119: u64;
            match len & 15 as size_t {
                12 => {
                    hash = (hash as ::core::ffi::c_uint).wrapping_add(
                        XXH_readLE32_align(ptr as *const ::core::ffi::c_void, align)
                            .wrapping_mul(XXH_PRIME32_3 as xxh_u32)
                            as ::core::ffi::c_uint,
                    ) as xxh_u32 as xxh_u32;
                    ptr = ptr.offset(4 as ::core::ffi::c_int as isize);
                    hash = hash
                        .rotate_left(17 as ::core::ffi::c_int as ::core::ffi::c_uint as u32)
                        .wrapping_mul(XXH_PRIME32_4) as xxh_u32;
                    current_block_119 = 16176279447558025744;
                }
                8 => {
                    current_block_119 = 16176279447558025744;
                }
                4 => {
                    current_block_119 = 18267544059428562525;
                }
                13 => {
                    hash = (hash as ::core::ffi::c_uint).wrapping_add(
                        XXH_readLE32_align(ptr as *const ::core::ffi::c_void, align)
                            .wrapping_mul(XXH_PRIME32_3 as xxh_u32)
                            as ::core::ffi::c_uint,
                    ) as xxh_u32 as xxh_u32;
                    ptr = ptr.offset(4 as ::core::ffi::c_int as isize);
                    hash = hash
                        .rotate_left(17 as ::core::ffi::c_int as ::core::ffi::c_uint as u32)
                        .wrapping_mul(XXH_PRIME32_4) as xxh_u32;
                    current_block_119 = 15917348852798289458;
                }
                9 => {
                    current_block_119 = 15917348852798289458;
                }
                5 => {
                    current_block_119 = 8855541136600540486;
                }
                14 => {
                    hash = (hash as ::core::ffi::c_uint).wrapping_add(
                        XXH_readLE32_align(ptr as *const ::core::ffi::c_void, align)
                            .wrapping_mul(XXH_PRIME32_3 as xxh_u32)
                            as ::core::ffi::c_uint,
                    ) as xxh_u32 as xxh_u32;
                    ptr = ptr.offset(4 as ::core::ffi::c_int as isize);
                    hash = hash
                        .rotate_left(17 as ::core::ffi::c_int as ::core::ffi::c_uint as u32)
                        .wrapping_mul(XXH_PRIME32_4) as xxh_u32;
                    current_block_119 = 11798019441063049682;
                }
                10 => {
                    current_block_119 = 11798019441063049682;
                }
                6 => {
                    current_block_119 = 8569828448656383210;
                }
                15 => {
                    hash = (hash as ::core::ffi::c_uint).wrapping_add(
                        XXH_readLE32_align(ptr as *const ::core::ffi::c_void, align)
                            .wrapping_mul(XXH_PRIME32_3 as xxh_u32)
                            as ::core::ffi::c_uint,
                    ) as xxh_u32 as xxh_u32;
                    ptr = ptr.offset(4 as ::core::ffi::c_int as isize);
                    hash = hash
                        .rotate_left(17 as ::core::ffi::c_int as ::core::ffi::c_uint as u32)
                        .wrapping_mul(XXH_PRIME32_4) as xxh_u32;
                    current_block_119 = 10034200926836878083;
                }
                11 => {
                    current_block_119 = 10034200926836878083;
                }
                7 => {
                    current_block_119 = 15317998733655598183;
                }
                3 => {
                    current_block_119 = 3588806136846431469;
                }
                2 => {
                    current_block_119 = 6175889709884974968;
                }
                1 => {
                    current_block_119 = 17830551781433467319;
                }
                0 => {
                    current_block_119 = 9567532787691974947;
                }
                _ => {
                    break 's_489;
                }
            }
            match current_block_119 {
                16176279447558025744 => {
                    hash = (hash as ::core::ffi::c_uint).wrapping_add(
                        XXH_readLE32_align(ptr as *const ::core::ffi::c_void, align)
                            .wrapping_mul(XXH_PRIME32_3 as xxh_u32)
                            as ::core::ffi::c_uint,
                    ) as xxh_u32 as xxh_u32;
                    ptr = ptr.offset(4 as ::core::ffi::c_int as isize);
                    hash = hash
                        .rotate_left(17 as ::core::ffi::c_int as ::core::ffi::c_uint as u32)
                        .wrapping_mul(XXH_PRIME32_4) as xxh_u32;
                    current_block_119 = 18267544059428562525;
                }
                15917348852798289458 => {
                    hash = (hash as ::core::ffi::c_uint).wrapping_add(
                        XXH_readLE32_align(ptr as *const ::core::ffi::c_void, align)
                            .wrapping_mul(XXH_PRIME32_3 as xxh_u32)
                            as ::core::ffi::c_uint,
                    ) as xxh_u32 as xxh_u32;
                    ptr = ptr.offset(4 as ::core::ffi::c_int as isize);
                    hash = hash
                        .rotate_left(17 as ::core::ffi::c_int as ::core::ffi::c_uint as u32)
                        .wrapping_mul(XXH_PRIME32_4) as xxh_u32;
                    current_block_119 = 8855541136600540486;
                }
                11798019441063049682 => {
                    hash = (hash as ::core::ffi::c_uint).wrapping_add(
                        XXH_readLE32_align(ptr as *const ::core::ffi::c_void, align)
                            .wrapping_mul(XXH_PRIME32_3 as xxh_u32)
                            as ::core::ffi::c_uint,
                    ) as xxh_u32 as xxh_u32;
                    ptr = ptr.offset(4 as ::core::ffi::c_int as isize);
                    hash = hash
                        .rotate_left(17 as ::core::ffi::c_int as ::core::ffi::c_uint as u32)
                        .wrapping_mul(XXH_PRIME32_4) as xxh_u32;
                    current_block_119 = 8569828448656383210;
                }
                10034200926836878083 => {
                    hash = (hash as ::core::ffi::c_uint).wrapping_add(
                        XXH_readLE32_align(ptr as *const ::core::ffi::c_void, align)
                            .wrapping_mul(XXH_PRIME32_3 as xxh_u32)
                            as ::core::ffi::c_uint,
                    ) as xxh_u32 as xxh_u32;
                    ptr = ptr.offset(4 as ::core::ffi::c_int as isize);
                    hash = hash
                        .rotate_left(17 as ::core::ffi::c_int as ::core::ffi::c_uint as u32)
                        .wrapping_mul(XXH_PRIME32_4) as xxh_u32;
                    current_block_119 = 15317998733655598183;
                }
                _ => {}
            }
            match current_block_119 {
                8569828448656383210 => {
                    hash = (hash as ::core::ffi::c_uint).wrapping_add(
                        XXH_readLE32_align(ptr as *const ::core::ffi::c_void, align)
                            .wrapping_mul(XXH_PRIME32_3 as xxh_u32)
                            as ::core::ffi::c_uint,
                    ) as xxh_u32 as xxh_u32;
                    ptr = ptr.offset(4 as ::core::ffi::c_int as isize);
                    hash = hash
                        .rotate_left(17 as ::core::ffi::c_int as ::core::ffi::c_uint as u32)
                        .wrapping_mul(XXH_PRIME32_4) as xxh_u32;
                    let fresh2 = ptr;
                    ptr = ptr.offset(1);
                    hash = (hash as ::core::ffi::c_uint)
                        .wrapping_add((*fresh2 as ::core::ffi::c_uint).wrapping_mul(XXH_PRIME32_5))
                        as xxh_u32 as xxh_u32;
                    hash = hash
                        .rotate_left(11 as ::core::ffi::c_int as ::core::ffi::c_uint as u32)
                        .wrapping_mul(XXH_PRIME32_1) as xxh_u32;
                    let fresh3 = ptr;
                    ptr = ptr.offset(1);
                    hash = (hash as ::core::ffi::c_uint)
                        .wrapping_add((*fresh3 as ::core::ffi::c_uint).wrapping_mul(XXH_PRIME32_5))
                        as xxh_u32 as xxh_u32;
                    hash = hash
                        .rotate_left(11 as ::core::ffi::c_int as ::core::ffi::c_uint as u32)
                        .wrapping_mul(XXH_PRIME32_1) as xxh_u32;
                    return XXH32_avalanche(hash);
                }
                8855541136600540486 => {
                    hash = (hash as ::core::ffi::c_uint).wrapping_add(
                        XXH_readLE32_align(ptr as *const ::core::ffi::c_void, align)
                            .wrapping_mul(XXH_PRIME32_3 as xxh_u32)
                            as ::core::ffi::c_uint,
                    ) as xxh_u32 as xxh_u32;
                    ptr = ptr.offset(4 as ::core::ffi::c_int as isize);
                    hash = hash
                        .rotate_left(17 as ::core::ffi::c_int as ::core::ffi::c_uint as u32)
                        .wrapping_mul(XXH_PRIME32_4) as xxh_u32;
                    let fresh1 = ptr;
                    ptr = ptr.offset(1);
                    hash = (hash as ::core::ffi::c_uint)
                        .wrapping_add((*fresh1 as ::core::ffi::c_uint).wrapping_mul(XXH_PRIME32_5))
                        as xxh_u32 as xxh_u32;
                    hash = hash
                        .rotate_left(11 as ::core::ffi::c_int as ::core::ffi::c_uint as u32)
                        .wrapping_mul(XXH_PRIME32_1) as xxh_u32;
                    return XXH32_avalanche(hash);
                }
                18267544059428562525 => {
                    hash = (hash as ::core::ffi::c_uint).wrapping_add(
                        XXH_readLE32_align(ptr as *const ::core::ffi::c_void, align)
                            .wrapping_mul(XXH_PRIME32_3 as xxh_u32)
                            as ::core::ffi::c_uint,
                    ) as xxh_u32 as xxh_u32;
                    ptr = ptr.offset(4 as ::core::ffi::c_int as isize);
                    hash = hash
                        .rotate_left(17 as ::core::ffi::c_int as ::core::ffi::c_uint as u32)
                        .wrapping_mul(XXH_PRIME32_4) as xxh_u32;
                    return XXH32_avalanche(hash);
                }
                15317998733655598183 => {
                    hash = (hash as ::core::ffi::c_uint).wrapping_add(
                        XXH_readLE32_align(ptr as *const ::core::ffi::c_void, align)
                            .wrapping_mul(XXH_PRIME32_3 as xxh_u32)
                            as ::core::ffi::c_uint,
                    ) as xxh_u32 as xxh_u32;
                    ptr = ptr.offset(4 as ::core::ffi::c_int as isize);
                    hash = hash
                        .rotate_left(17 as ::core::ffi::c_int as ::core::ffi::c_uint as u32)
                        .wrapping_mul(XXH_PRIME32_4) as xxh_u32;
                    current_block_119 = 3588806136846431469;
                }
                _ => {}
            }
            match current_block_119 {
                3588806136846431469 => {
                    let fresh4 = ptr;
                    ptr = ptr.offset(1);
                    hash = (hash as ::core::ffi::c_uint)
                        .wrapping_add((*fresh4 as ::core::ffi::c_uint).wrapping_mul(XXH_PRIME32_5))
                        as xxh_u32 as xxh_u32;
                    hash = hash
                        .rotate_left(11 as ::core::ffi::c_int as ::core::ffi::c_uint as u32)
                        .wrapping_mul(XXH_PRIME32_1) as xxh_u32;
                    current_block_119 = 6175889709884974968;
                }
                _ => {}
            }
            match current_block_119 {
                6175889709884974968 => {
                    let fresh5 = ptr;
                    ptr = ptr.offset(1);
                    hash = (hash as ::core::ffi::c_uint)
                        .wrapping_add((*fresh5 as ::core::ffi::c_uint).wrapping_mul(XXH_PRIME32_5))
                        as xxh_u32 as xxh_u32;
                    hash = hash
                        .rotate_left(11 as ::core::ffi::c_int as ::core::ffi::c_uint as u32)
                        .wrapping_mul(XXH_PRIME32_1) as xxh_u32;
                    current_block_119 = 17830551781433467319;
                }
                _ => {}
            }
            match current_block_119 {
                17830551781433467319 => {
                    let fresh6 = ptr;
                    ptr = ptr.offset(1);
                    hash = (hash as ::core::ffi::c_uint)
                        .wrapping_add((*fresh6 as ::core::ffi::c_uint).wrapping_mul(XXH_PRIME32_5))
                        as xxh_u32 as xxh_u32;
                    hash = hash
                        .rotate_left(11 as ::core::ffi::c_int as ::core::ffi::c_uint as u32)
                        .wrapping_mul(XXH_PRIME32_1) as xxh_u32;
                }
                _ => {}
            }
            return XXH32_avalanche(hash);
        }
        return hash;
    };
}
unsafe extern "C" fn XXH32_endian_align(
    mut input: *const xxh_u8,
    mut len: size_t,
    mut seed: xxh_u32,
    mut align: XXH_alignment,
) -> xxh_u32 {
    let mut h32: xxh_u32 = 0;
    input.is_null();
    if len >= 16 as size_t {
        let mut acc: [xxh_u32; 4] = [0; 4];
        XXH32_initAccs(&raw mut acc as *mut xxh_u32, seed);
        input = XXH32_consumeLong(&raw mut acc as *mut xxh_u32, input, len, align);
        h32 = XXH32_mergeAccs(&raw mut acc as *mut xxh_u32);
    } else {
        h32 = seed.wrapping_add(XXH_PRIME32_5 as xxh_u32);
    }
    h32 = h32.wrapping_add(len as xxh_u32);
    return XXH32_finalize(h32, input, len & 15 as size_t, align);
}
#[no_mangle]
pub unsafe extern "C" fn XXH32(
    mut input: *const ::core::ffi::c_void,
    mut len: size_t,
    mut seed: XXH32_hash_t,
) -> XXH32_hash_t {
    return XXH32_endian_align(input as *const xxh_u8, len, seed as xxh_u32, XXH_unaligned)
        as XXH32_hash_t;
}
#[no_mangle]
pub unsafe extern "C" fn XXH32_createState() -> *mut XXH32_state_t {
    return XXH_malloc(::core::mem::size_of::<XXH32_state_t>() as size_t) as *mut XXH32_state_t;
}
#[no_mangle]
pub unsafe extern "C" fn XXH32_freeState(mut statePtr: *mut XXH32_state_t) -> XXH_errorcode {
    XXH_free(statePtr as *mut ::core::ffi::c_void);
    return XXH_OK;
}
#[no_mangle]
pub unsafe extern "C" fn XXH32_copyState(
    mut dstState: *mut XXH32_state_t,
    mut srcState: *const XXH32_state_t,
) {
    memcpy(
        dstState as *mut ::core::ffi::c_void,
        srcState as *const ::core::ffi::c_void,
        ::core::mem::size_of::<XXH32_state_t>() as size_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn XXH32_reset(
    mut statePtr: *mut XXH32_state_t,
    mut seed: XXH32_hash_t,
) -> XXH_errorcode {
    memset(
        statePtr as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<XXH32_state_t>() as size_t,
    );
    XXH32_initAccs(&raw mut (*statePtr).acc as *mut xxh_u32, seed as xxh_u32);
    return XXH_OK;
}
#[no_mangle]
pub unsafe extern "C" fn XXH32_update(
    mut state: *mut XXH32_state_t,
    mut input: *const ::core::ffi::c_void,
    mut len: size_t,
) -> XXH_errorcode {
    if input.is_null() {
        return XXH_OK;
    }
    (*state).total_len_32 = (*state).total_len_32.wrapping_add(len as XXH32_hash_t);
    (*state).large_len |= ((len >= 16 as size_t) as ::core::ffi::c_int
        | ((*state).total_len_32 >= 16 as XXH32_hash_t) as ::core::ffi::c_int)
        as XXH32_hash_t;
    if len
        < (::core::mem::size_of::<[::core::ffi::c_uchar; 16]>() as usize)
            .wrapping_sub((*state).bufferedSize as usize)
    {
        memcpy(
            (&raw mut (*state).buffer as *mut ::core::ffi::c_uchar)
                .offset((*state).bufferedSize as isize) as *mut ::core::ffi::c_void,
            input,
            len,
        );
        (*state).bufferedSize = (*state).bufferedSize.wrapping_add(len as XXH32_hash_t);
        return XXH_OK;
    }
    let mut xinput: *const xxh_u8 = input as *const xxh_u8;
    let bEnd: *const xxh_u8 = xinput.offset(len as isize);
    if (*state).bufferedSize != 0 {
        memcpy(
            (&raw mut (*state).buffer as *mut ::core::ffi::c_uchar)
                .offset((*state).bufferedSize as isize) as *mut ::core::ffi::c_void,
            xinput as *const ::core::ffi::c_void,
            (::core::mem::size_of::<[::core::ffi::c_uchar; 16]>() as size_t)
                .wrapping_sub((*state).bufferedSize as size_t),
        );
        xinput = xinput.offset(
            (::core::mem::size_of::<[::core::ffi::c_uchar; 16]>() as usize)
                .wrapping_sub((*state).bufferedSize as usize) as isize,
        );
        XXH32_consumeLong(
            &raw mut (*state).acc as *mut xxh_u32,
            &raw mut (*state).buffer as *mut ::core::ffi::c_uchar,
            ::core::mem::size_of::<[::core::ffi::c_uchar; 16]>() as size_t,
            XXH_aligned,
        );
        (*state).bufferedSize = 0 as XXH32_hash_t;
    }
    if bEnd.offset_from(xinput) as ::core::ffi::c_long as size_t
        >= ::core::mem::size_of::<[::core::ffi::c_uchar; 16]>() as usize
    {
        xinput = XXH32_consumeLong(
            &raw mut (*state).acc as *mut xxh_u32,
            xinput,
            bEnd.offset_from(xinput) as ::core::ffi::c_long as size_t,
            XXH_unaligned,
        );
    }
    if xinput < bEnd {
        memcpy(
            &raw mut (*state).buffer as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_void,
            xinput as *const ::core::ffi::c_void,
            bEnd.offset_from(xinput) as ::core::ffi::c_long as size_t,
        );
        (*state).bufferedSize =
            bEnd.offset_from(xinput) as ::core::ffi::c_long as ::core::ffi::c_uint as XXH32_hash_t;
    }
    return XXH_OK;
}
#[no_mangle]
pub unsafe extern "C" fn XXH32_digest(mut state: *const XXH32_state_t) -> XXH32_hash_t {
    let mut h32: xxh_u32 = 0;
    if (*state).large_len != 0 {
        h32 = XXH32_mergeAccs(&raw const (*state).acc as *const xxh_u32);
    } else {
        h32 = (*state).acc[2 as ::core::ffi::c_int as usize]
            .wrapping_add(XXH_PRIME32_5 as XXH32_hash_t) as xxh_u32;
    }
    h32 = (h32 as XXH32_hash_t).wrapping_add((*state).total_len_32) as xxh_u32 as xxh_u32;
    return XXH32_finalize(
        h32,
        &raw const (*state).buffer as *const xxh_u8,
        (*state).bufferedSize as size_t,
        XXH_aligned,
    ) as XXH32_hash_t;
}
#[no_mangle]
pub unsafe extern "C" fn XXH32_canonicalFromHash(
    mut dst: *mut XXH32_canonical_t,
    mut hash: XXH32_hash_t,
) {
    hash = XXH_swap32(hash as xxh_u32) as XXH32_hash_t;
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw mut hash as *const ::core::ffi::c_void,
        ::core::mem::size_of::<XXH32_canonical_t>() as size_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn XXH32_hashFromCanonical(
    mut src: *const XXH32_canonical_t,
) -> XXH32_hash_t {
    return XXH_readBE32(src as *const ::core::ffi::c_void) as XXH32_hash_t;
}
unsafe extern "C" fn XXH_read64(mut ptr: *const ::core::ffi::c_void) -> xxh_u64 {
    return *(ptr as *const xxh_unalign64);
}
unsafe extern "C" fn XXH_swap64(mut x: xxh_u64) -> xxh_u64 {
    return ((x << 56 as ::core::ffi::c_int) as ::core::ffi::c_ulonglong
        & 0xff00000000000000 as ::core::ffi::c_ulonglong
        | (x << 40 as ::core::ffi::c_int) as ::core::ffi::c_ulonglong
            & 0xff000000000000 as ::core::ffi::c_ulonglong
        | (x << 24 as ::core::ffi::c_int) as ::core::ffi::c_ulonglong
            & 0xff0000000000 as ::core::ffi::c_ulonglong
        | (x << 8 as ::core::ffi::c_int) as ::core::ffi::c_ulonglong
            & 0xff00000000 as ::core::ffi::c_ulonglong
        | (x >> 8 as ::core::ffi::c_int) as ::core::ffi::c_ulonglong
            & 0xff000000 as ::core::ffi::c_ulonglong
        | (x >> 24 as ::core::ffi::c_int) as ::core::ffi::c_ulonglong
            & 0xff0000 as ::core::ffi::c_ulonglong
        | (x >> 40 as ::core::ffi::c_int) as ::core::ffi::c_ulonglong
            & 0xff00 as ::core::ffi::c_ulonglong
        | (x >> 56 as ::core::ffi::c_int) as ::core::ffi::c_ulonglong
            & 0xff as ::core::ffi::c_ulonglong) as xxh_u64;
}
unsafe extern "C" fn XXH_readLE64(mut ptr: *const ::core::ffi::c_void) -> xxh_u64 {
    return if XXH_CPU_LITTLE_ENDIAN != 0 {
        XXH_read64(ptr)
    } else {
        XXH_swap64(XXH_read64(ptr))
    };
}
unsafe extern "C" fn XXH_readBE64(mut ptr: *const ::core::ffi::c_void) -> xxh_u64 {
    return if XXH_CPU_LITTLE_ENDIAN != 0 {
        XXH_swap64(XXH_read64(ptr))
    } else {
        XXH_read64(ptr)
    };
}
unsafe extern "C" fn XXH_readLE64_align(
    mut ptr: *const ::core::ffi::c_void,
    mut align: XXH_alignment,
) -> xxh_u64 {
    if align as ::core::ffi::c_uint == XXH_unaligned as ::core::ffi::c_int as ::core::ffi::c_uint {
        return XXH_readLE64(ptr);
    } else {
        return if XXH_CPU_LITTLE_ENDIAN != 0 {
            *(ptr as *const xxh_u64)
        } else {
            XXH_swap64(*(ptr as *const xxh_u64))
        };
    };
}
pub const XXH_PRIME64_1: ::core::ffi::c_ulonglong = 0x9e3779b185ebca87 as ::core::ffi::c_ulonglong;
pub const XXH_PRIME64_2: ::core::ffi::c_ulonglong = 0xc2b2ae3d27d4eb4f as ::core::ffi::c_ulonglong;
pub const XXH_PRIME64_3: ::core::ffi::c_ulonglong = 0x165667b19e3779f9 as ::core::ffi::c_ulonglong;
pub const XXH_PRIME64_4: ::core::ffi::c_ulonglong = 0x85ebca77c2b2ae63 as ::core::ffi::c_ulonglong;
pub const XXH_PRIME64_5: ::core::ffi::c_ulonglong = 0x27d4eb2f165667c5 as ::core::ffi::c_ulonglong;
unsafe extern "C" fn XXH64_round(mut acc: xxh_u64, mut input: xxh_u64) -> xxh_u64 {
    acc = (acc as ::core::ffi::c_ulonglong)
        .wrapping_add((input as ::core::ffi::c_ulonglong).wrapping_mul(XXH_PRIME64_2))
        as xxh_u64 as xxh_u64;
    acc = acc.rotate_left(31 as ::core::ffi::c_int as ::core::ffi::c_ulong as u32) as xxh_u64;
    acc = (acc as ::core::ffi::c_ulonglong).wrapping_mul(XXH_PRIME64_1) as xxh_u64 as xxh_u64;
    return acc;
}
unsafe extern "C" fn XXH64_mergeRound(mut acc: xxh_u64, mut val: xxh_u64) -> xxh_u64 {
    val = XXH64_round(0 as xxh_u64, val);
    acc ^= val;
    acc = (acc as ::core::ffi::c_ulonglong)
        .wrapping_mul(XXH_PRIME64_1)
        .wrapping_add(XXH_PRIME64_4) as xxh_u64;
    return acc;
}
unsafe extern "C" fn XXH64_avalanche(mut hash: xxh_u64) -> xxh_u64 {
    hash ^= hash >> 33 as ::core::ffi::c_int;
    hash = (hash as ::core::ffi::c_ulonglong).wrapping_mul(XXH_PRIME64_2) as xxh_u64 as xxh_u64;
    hash ^= hash >> 29 as ::core::ffi::c_int;
    hash = (hash as ::core::ffi::c_ulonglong).wrapping_mul(XXH_PRIME64_3) as xxh_u64 as xxh_u64;
    hash ^= hash >> 32 as ::core::ffi::c_int;
    return hash;
}
unsafe extern "C" fn XXH64_initAccs(mut acc: *mut xxh_u64, mut seed: xxh_u64) {
    *acc.offset(0 as ::core::ffi::c_int as isize) = (seed as ::core::ffi::c_ulonglong)
        .wrapping_add(XXH_PRIME64_1)
        .wrapping_add(XXH_PRIME64_2) as xxh_u64;
    *acc.offset(1 as ::core::ffi::c_int as isize) =
        (seed as ::core::ffi::c_ulonglong).wrapping_add(XXH_PRIME64_2) as xxh_u64;
    *acc.offset(2 as ::core::ffi::c_int as isize) = seed.wrapping_add(0 as xxh_u64);
    *acc.offset(3 as ::core::ffi::c_int as isize) =
        (seed as ::core::ffi::c_ulonglong).wrapping_sub(XXH_PRIME64_1) as xxh_u64;
}
unsafe extern "C" fn XXH64_consumeLong(
    mut acc: *mut xxh_u64,
    mut input: *const xxh_u8,
    mut len: size_t,
    mut align: XXH_alignment,
) -> *const xxh_u8 {
    let bEnd: *const xxh_u8 = input.offset(len as isize);
    let limit: *const xxh_u8 = bEnd.offset(-(31 as ::core::ffi::c_int as isize));
    loop {
        if (::core::mem::size_of::<*mut ::core::ffi::c_void>() as usize)
            < ::core::mem::size_of::<xxh_u64>() as usize
        {
            let mut i: size_t = 0;
            i = 0 as size_t;
            while i < 4 as size_t {
                *acc.offset(i as isize) = XXH64_round(
                    *acc.offset(i as isize),
                    XXH_readLE64_align(input as *const ::core::ffi::c_void, align),
                );
                input = input.offset(8 as ::core::ffi::c_int as isize);
                i = i.wrapping_add(1);
            }
        } else {
            *acc.offset(0 as ::core::ffi::c_int as isize) = XXH64_round(
                *acc.offset(0 as ::core::ffi::c_int as isize),
                XXH_readLE64_align(input as *const ::core::ffi::c_void, align),
            );
            input = input.offset(8 as ::core::ffi::c_int as isize);
            *acc.offset(1 as ::core::ffi::c_int as isize) = XXH64_round(
                *acc.offset(1 as ::core::ffi::c_int as isize),
                XXH_readLE64_align(input as *const ::core::ffi::c_void, align),
            );
            input = input.offset(8 as ::core::ffi::c_int as isize);
            *acc.offset(2 as ::core::ffi::c_int as isize) = XXH64_round(
                *acc.offset(2 as ::core::ffi::c_int as isize),
                XXH_readLE64_align(input as *const ::core::ffi::c_void, align),
            );
            input = input.offset(8 as ::core::ffi::c_int as isize);
            *acc.offset(3 as ::core::ffi::c_int as isize) = XXH64_round(
                *acc.offset(3 as ::core::ffi::c_int as isize),
                XXH_readLE64_align(input as *const ::core::ffi::c_void, align),
            );
            input = input.offset(8 as ::core::ffi::c_int as isize);
        }
        if !(input < limit) {
            break;
        }
    }
    return input;
}
unsafe extern "C" fn XXH64_mergeAccs(mut acc: *const xxh_u64) -> xxh_u64 {
    let mut h64: xxh_u64 = (*acc.offset(0 as ::core::ffi::c_int as isize))
        .rotate_left(1 as ::core::ffi::c_int as ::core::ffi::c_ulong as u32)
        .wrapping_add(
            (*acc.offset(1 as ::core::ffi::c_int as isize))
                .rotate_left(7 as ::core::ffi::c_int as ::core::ffi::c_ulong as u32),
        )
        .wrapping_add(
            (*acc.offset(2 as ::core::ffi::c_int as isize))
                .rotate_left(12 as ::core::ffi::c_int as ::core::ffi::c_ulong as u32),
        )
        .wrapping_add(
            (*acc.offset(3 as ::core::ffi::c_int as isize))
                .rotate_left(18 as ::core::ffi::c_int as ::core::ffi::c_ulong as u32),
        );
    if (::core::mem::size_of::<*mut ::core::ffi::c_void>() as usize)
        < ::core::mem::size_of::<xxh_u64>() as usize
    {
        let mut i: size_t = 0;
        i = 0 as size_t;
        while i < 4 as size_t {
            h64 = XXH64_mergeRound(h64, *acc.offset(i as isize));
            i = i.wrapping_add(1);
        }
    } else {
        h64 = XXH64_mergeRound(h64, *acc.offset(0 as ::core::ffi::c_int as isize));
        h64 = XXH64_mergeRound(h64, *acc.offset(1 as ::core::ffi::c_int as isize));
        h64 = XXH64_mergeRound(h64, *acc.offset(2 as ::core::ffi::c_int as isize));
        h64 = XXH64_mergeRound(h64, *acc.offset(3 as ::core::ffi::c_int as isize));
    }
    return h64;
}
unsafe extern "C" fn XXH64_finalize(
    mut hash: xxh_u64,
    mut ptr: *const xxh_u8,
    mut len: size_t,
    mut align: XXH_alignment,
) -> xxh_u64 {
    ptr.is_null();
    len &= 31 as size_t;
    while len >= 8 as size_t {
        let k1: xxh_u64 = XXH64_round(
            0 as xxh_u64,
            XXH_readLE64_align(ptr as *const ::core::ffi::c_void, align),
        ) as xxh_u64;
        ptr = ptr.offset(8 as ::core::ffi::c_int as isize);
        hash ^= k1;
        hash = (hash.rotate_left(27 as ::core::ffi::c_int as ::core::ffi::c_ulong as u32)
            as ::core::ffi::c_ulonglong)
            .wrapping_mul(XXH_PRIME64_1)
            .wrapping_add(XXH_PRIME64_4) as xxh_u64;
        len = len.wrapping_sub(8 as size_t);
    }
    if len >= 4 as size_t {
        hash = (hash as ::core::ffi::c_ulonglong
            ^ (XXH_readLE32_align(ptr as *const ::core::ffi::c_void, align) as xxh_u64
                as ::core::ffi::c_ulonglong)
                .wrapping_mul(XXH_PRIME64_1)) as xxh_u64;
        ptr = ptr.offset(4 as ::core::ffi::c_int as isize);
        hash = (hash.rotate_left(23 as ::core::ffi::c_int as ::core::ffi::c_ulong as u32)
            as ::core::ffi::c_ulonglong)
            .wrapping_mul(XXH_PRIME64_2)
            .wrapping_add(XXH_PRIME64_3) as xxh_u64;
        len = len.wrapping_sub(4 as size_t);
    }
    while len > 0 as size_t {
        let fresh7 = ptr;
        ptr = ptr.offset(1);
        hash = (hash as ::core::ffi::c_ulonglong
            ^ (*fresh7 as ::core::ffi::c_ulonglong).wrapping_mul(XXH_PRIME64_5))
            as xxh_u64;
        hash = (hash.rotate_left(11 as ::core::ffi::c_int as ::core::ffi::c_ulong as u32)
            as ::core::ffi::c_ulonglong)
            .wrapping_mul(XXH_PRIME64_1) as xxh_u64;
        len = len.wrapping_sub(1);
    }
    return XXH64_avalanche(hash);
}
unsafe extern "C" fn XXH64_endian_align(
    mut input: *const xxh_u8,
    mut len: size_t,
    mut seed: xxh_u64,
    mut align: XXH_alignment,
) -> xxh_u64 {
    let mut h64: xxh_u64 = 0;
    input.is_null();
    if len >= 32 as size_t {
        let mut acc: [xxh_u64; 4] = [0; 4];
        XXH64_initAccs(&raw mut acc as *mut xxh_u64, seed);
        input = XXH64_consumeLong(&raw mut acc as *mut xxh_u64, input, len, align);
        h64 = XXH64_mergeAccs(&raw mut acc as *mut xxh_u64);
    } else {
        h64 = (seed as ::core::ffi::c_ulonglong).wrapping_add(XXH_PRIME64_5) as xxh_u64;
    }
    h64 = h64.wrapping_add(len as xxh_u64);
    return XXH64_finalize(h64, input, len, align);
}
#[no_mangle]
pub unsafe extern "C" fn XXH64(
    mut input: *const ::core::ffi::c_void,
    mut len: size_t,
    mut seed: XXH64_hash_t,
) -> XXH64_hash_t {
    return XXH64_endian_align(input as *const xxh_u8, len, seed as xxh_u64, XXH_unaligned)
        as XXH64_hash_t;
}
#[no_mangle]
pub unsafe extern "C" fn XXH64_createState() -> *mut XXH64_state_t {
    return XXH_malloc(::core::mem::size_of::<XXH64_state_t>() as size_t) as *mut XXH64_state_t;
}
#[no_mangle]
pub unsafe extern "C" fn XXH64_freeState(mut statePtr: *mut XXH64_state_t) -> XXH_errorcode {
    XXH_free(statePtr as *mut ::core::ffi::c_void);
    return XXH_OK;
}
#[no_mangle]
pub unsafe extern "C" fn XXH64_copyState(
    mut dstState: *mut XXH64_state_t,
    mut srcState: *const XXH64_state_t,
) {
    memcpy(
        dstState as *mut ::core::ffi::c_void,
        srcState as *const ::core::ffi::c_void,
        ::core::mem::size_of::<XXH64_state_t>() as size_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn XXH64_reset(
    mut statePtr: *mut XXH64_state_t,
    mut seed: XXH64_hash_t,
) -> XXH_errorcode {
    memset(
        statePtr as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<XXH64_state_t>() as size_t,
    );
    XXH64_initAccs(&raw mut (*statePtr).acc as *mut xxh_u64, seed as xxh_u64);
    return XXH_OK;
}
#[no_mangle]
pub unsafe extern "C" fn XXH64_update(
    mut state: *mut XXH64_state_t,
    mut input: *const ::core::ffi::c_void,
    mut len: size_t,
) -> XXH_errorcode {
    if input.is_null() {
        return XXH_OK;
    }
    (*state).total_len = ((*state).total_len as ::core::ffi::c_ulong)
        .wrapping_add(len as ::core::ffi::c_ulong) as XXH64_hash_t
        as XXH64_hash_t;
    if len
        < (::core::mem::size_of::<[::core::ffi::c_uchar; 32]>() as usize)
            .wrapping_sub((*state).bufferedSize as usize)
    {
        memcpy(
            (&raw mut (*state).buffer as *mut ::core::ffi::c_uchar)
                .offset((*state).bufferedSize as isize) as *mut ::core::ffi::c_void,
            input,
            len,
        );
        (*state).bufferedSize = (*state).bufferedSize.wrapping_add(len as XXH32_hash_t);
        return XXH_OK;
    }
    let mut xinput: *const xxh_u8 = input as *const xxh_u8;
    let bEnd: *const xxh_u8 = xinput.offset(len as isize);
    if (*state).bufferedSize != 0 {
        memcpy(
            (&raw mut (*state).buffer as *mut ::core::ffi::c_uchar)
                .offset((*state).bufferedSize as isize) as *mut ::core::ffi::c_void,
            xinput as *const ::core::ffi::c_void,
            (::core::mem::size_of::<[::core::ffi::c_uchar; 32]>() as size_t)
                .wrapping_sub((*state).bufferedSize as size_t),
        );
        xinput = xinput.offset(
            (::core::mem::size_of::<[::core::ffi::c_uchar; 32]>() as usize)
                .wrapping_sub((*state).bufferedSize as usize) as isize,
        );
        XXH64_consumeLong(
            &raw mut (*state).acc as *mut xxh_u64,
            &raw mut (*state).buffer as *mut ::core::ffi::c_uchar,
            ::core::mem::size_of::<[::core::ffi::c_uchar; 32]>() as size_t,
            XXH_aligned,
        );
        (*state).bufferedSize = 0 as XXH32_hash_t;
    }
    if bEnd.offset_from(xinput) as ::core::ffi::c_long as size_t
        >= ::core::mem::size_of::<[::core::ffi::c_uchar; 32]>() as usize
    {
        xinput = XXH64_consumeLong(
            &raw mut (*state).acc as *mut xxh_u64,
            xinput,
            bEnd.offset_from(xinput) as ::core::ffi::c_long as size_t,
            XXH_unaligned,
        );
    }
    if xinput < bEnd {
        memcpy(
            &raw mut (*state).buffer as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_void,
            xinput as *const ::core::ffi::c_void,
            bEnd.offset_from(xinput) as ::core::ffi::c_long as size_t,
        );
        (*state).bufferedSize =
            bEnd.offset_from(xinput) as ::core::ffi::c_long as ::core::ffi::c_uint as XXH32_hash_t;
    }
    return XXH_OK;
}
#[no_mangle]
pub unsafe extern "C" fn XXH64_digest(mut state: *const XXH64_state_t) -> XXH64_hash_t {
    let mut h64: xxh_u64 = 0;
    if (*state).total_len >= 32 as XXH64_hash_t {
        h64 = XXH64_mergeAccs(&raw const (*state).acc as *const xxh_u64);
    } else {
        h64 = ((*state).acc[2 as ::core::ffi::c_int as usize] as ::core::ffi::c_ulonglong)
            .wrapping_add(XXH_PRIME64_5) as xxh_u64;
    }
    h64 = h64.wrapping_add((*state).total_len);
    return XXH64_finalize(
        h64,
        &raw const (*state).buffer as *const xxh_u8,
        (*state).total_len as size_t,
        XXH_aligned,
    ) as XXH64_hash_t;
}
#[no_mangle]
pub unsafe extern "C" fn XXH64_canonicalFromHash(
    mut dst: *mut XXH64_canonical_t,
    mut hash: XXH64_hash_t,
) {
    hash = XXH_swap64(hash as xxh_u64) as XXH64_hash_t;
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw mut hash as *const ::core::ffi::c_void,
        ::core::mem::size_of::<XXH64_canonical_t>() as size_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn XXH64_hashFromCanonical(
    mut src: *const XXH64_canonical_t,
) -> XXH64_hash_t {
    return XXH_readBE64(src as *const ::core::ffi::c_void) as XXH64_hash_t;
}
pub const XXH_SECRET_DEFAULT_SIZE: ::core::ffi::c_int = 192 as ::core::ffi::c_int;
static mut XXH3_kSecret: [xxh_u8; 192] = [
    0xb8 as ::core::ffi::c_int as xxh_u8,
    0xfe as ::core::ffi::c_int as xxh_u8,
    0x6c as ::core::ffi::c_int as xxh_u8,
    0x39 as ::core::ffi::c_int as xxh_u8,
    0x23 as ::core::ffi::c_int as xxh_u8,
    0xa4 as ::core::ffi::c_int as xxh_u8,
    0x4b as ::core::ffi::c_int as xxh_u8,
    0xbe as ::core::ffi::c_int as xxh_u8,
    0x7c as ::core::ffi::c_int as xxh_u8,
    0x1 as ::core::ffi::c_int as xxh_u8,
    0x81 as ::core::ffi::c_int as xxh_u8,
    0x2c as ::core::ffi::c_int as xxh_u8,
    0xf7 as ::core::ffi::c_int as xxh_u8,
    0x21 as ::core::ffi::c_int as xxh_u8,
    0xad as ::core::ffi::c_int as xxh_u8,
    0x1c as ::core::ffi::c_int as xxh_u8,
    0xde as ::core::ffi::c_int as xxh_u8,
    0xd4 as ::core::ffi::c_int as xxh_u8,
    0x6d as ::core::ffi::c_int as xxh_u8,
    0xe9 as ::core::ffi::c_int as xxh_u8,
    0x83 as ::core::ffi::c_int as xxh_u8,
    0x90 as ::core::ffi::c_int as xxh_u8,
    0x97 as ::core::ffi::c_int as xxh_u8,
    0xdb as ::core::ffi::c_int as xxh_u8,
    0x72 as ::core::ffi::c_int as xxh_u8,
    0x40 as ::core::ffi::c_int as xxh_u8,
    0xa4 as ::core::ffi::c_int as xxh_u8,
    0xa4 as ::core::ffi::c_int as xxh_u8,
    0xb7 as ::core::ffi::c_int as xxh_u8,
    0xb3 as ::core::ffi::c_int as xxh_u8,
    0x67 as ::core::ffi::c_int as xxh_u8,
    0x1f as ::core::ffi::c_int as xxh_u8,
    0xcb as ::core::ffi::c_int as xxh_u8,
    0x79 as ::core::ffi::c_int as xxh_u8,
    0xe6 as ::core::ffi::c_int as xxh_u8,
    0x4e as ::core::ffi::c_int as xxh_u8,
    0xcc as ::core::ffi::c_int as xxh_u8,
    0xc0 as ::core::ffi::c_int as xxh_u8,
    0xe5 as ::core::ffi::c_int as xxh_u8,
    0x78 as ::core::ffi::c_int as xxh_u8,
    0x82 as ::core::ffi::c_int as xxh_u8,
    0x5a as ::core::ffi::c_int as xxh_u8,
    0xd0 as ::core::ffi::c_int as xxh_u8,
    0x7d as ::core::ffi::c_int as xxh_u8,
    0xcc as ::core::ffi::c_int as xxh_u8,
    0xff as ::core::ffi::c_int as xxh_u8,
    0x72 as ::core::ffi::c_int as xxh_u8,
    0x21 as ::core::ffi::c_int as xxh_u8,
    0xb8 as ::core::ffi::c_int as xxh_u8,
    0x8 as ::core::ffi::c_int as xxh_u8,
    0x46 as ::core::ffi::c_int as xxh_u8,
    0x74 as ::core::ffi::c_int as xxh_u8,
    0xf7 as ::core::ffi::c_int as xxh_u8,
    0x43 as ::core::ffi::c_int as xxh_u8,
    0x24 as ::core::ffi::c_int as xxh_u8,
    0x8e as ::core::ffi::c_int as xxh_u8,
    0xe0 as ::core::ffi::c_int as xxh_u8,
    0x35 as ::core::ffi::c_int as xxh_u8,
    0x90 as ::core::ffi::c_int as xxh_u8,
    0xe6 as ::core::ffi::c_int as xxh_u8,
    0x81 as ::core::ffi::c_int as xxh_u8,
    0x3a as ::core::ffi::c_int as xxh_u8,
    0x26 as ::core::ffi::c_int as xxh_u8,
    0x4c as ::core::ffi::c_int as xxh_u8,
    0x3c as ::core::ffi::c_int as xxh_u8,
    0x28 as ::core::ffi::c_int as xxh_u8,
    0x52 as ::core::ffi::c_int as xxh_u8,
    0xbb as ::core::ffi::c_int as xxh_u8,
    0x91 as ::core::ffi::c_int as xxh_u8,
    0xc3 as ::core::ffi::c_int as xxh_u8,
    0 as ::core::ffi::c_int as xxh_u8,
    0xcb as ::core::ffi::c_int as xxh_u8,
    0x88 as ::core::ffi::c_int as xxh_u8,
    0xd0 as ::core::ffi::c_int as xxh_u8,
    0x65 as ::core::ffi::c_int as xxh_u8,
    0x8b as ::core::ffi::c_int as xxh_u8,
    0x1b as ::core::ffi::c_int as xxh_u8,
    0x53 as ::core::ffi::c_int as xxh_u8,
    0x2e as ::core::ffi::c_int as xxh_u8,
    0xa3 as ::core::ffi::c_int as xxh_u8,
    0x71 as ::core::ffi::c_int as xxh_u8,
    0x64 as ::core::ffi::c_int as xxh_u8,
    0x48 as ::core::ffi::c_int as xxh_u8,
    0x97 as ::core::ffi::c_int as xxh_u8,
    0xa2 as ::core::ffi::c_int as xxh_u8,
    0xd as ::core::ffi::c_int as xxh_u8,
    0xf9 as ::core::ffi::c_int as xxh_u8,
    0x4e as ::core::ffi::c_int as xxh_u8,
    0x38 as ::core::ffi::c_int as xxh_u8,
    0x19 as ::core::ffi::c_int as xxh_u8,
    0xef as ::core::ffi::c_int as xxh_u8,
    0x46 as ::core::ffi::c_int as xxh_u8,
    0xa9 as ::core::ffi::c_int as xxh_u8,
    0xde as ::core::ffi::c_int as xxh_u8,
    0xac as ::core::ffi::c_int as xxh_u8,
    0xd8 as ::core::ffi::c_int as xxh_u8,
    0xa8 as ::core::ffi::c_int as xxh_u8,
    0xfa as ::core::ffi::c_int as xxh_u8,
    0x76 as ::core::ffi::c_int as xxh_u8,
    0x3f as ::core::ffi::c_int as xxh_u8,
    0xe3 as ::core::ffi::c_int as xxh_u8,
    0x9c as ::core::ffi::c_int as xxh_u8,
    0x34 as ::core::ffi::c_int as xxh_u8,
    0x3f as ::core::ffi::c_int as xxh_u8,
    0xf9 as ::core::ffi::c_int as xxh_u8,
    0xdc as ::core::ffi::c_int as xxh_u8,
    0xbb as ::core::ffi::c_int as xxh_u8,
    0xc7 as ::core::ffi::c_int as xxh_u8,
    0xc7 as ::core::ffi::c_int as xxh_u8,
    0xb as ::core::ffi::c_int as xxh_u8,
    0x4f as ::core::ffi::c_int as xxh_u8,
    0x1d as ::core::ffi::c_int as xxh_u8,
    0x8a as ::core::ffi::c_int as xxh_u8,
    0x51 as ::core::ffi::c_int as xxh_u8,
    0xe0 as ::core::ffi::c_int as xxh_u8,
    0x4b as ::core::ffi::c_int as xxh_u8,
    0xcd as ::core::ffi::c_int as xxh_u8,
    0xb4 as ::core::ffi::c_int as xxh_u8,
    0x59 as ::core::ffi::c_int as xxh_u8,
    0x31 as ::core::ffi::c_int as xxh_u8,
    0xc8 as ::core::ffi::c_int as xxh_u8,
    0x9f as ::core::ffi::c_int as xxh_u8,
    0x7e as ::core::ffi::c_int as xxh_u8,
    0xc9 as ::core::ffi::c_int as xxh_u8,
    0xd9 as ::core::ffi::c_int as xxh_u8,
    0x78 as ::core::ffi::c_int as xxh_u8,
    0x73 as ::core::ffi::c_int as xxh_u8,
    0x64 as ::core::ffi::c_int as xxh_u8,
    0xea as ::core::ffi::c_int as xxh_u8,
    0xc5 as ::core::ffi::c_int as xxh_u8,
    0xac as ::core::ffi::c_int as xxh_u8,
    0x83 as ::core::ffi::c_int as xxh_u8,
    0x34 as ::core::ffi::c_int as xxh_u8,
    0xd3 as ::core::ffi::c_int as xxh_u8,
    0xeb as ::core::ffi::c_int as xxh_u8,
    0xc3 as ::core::ffi::c_int as xxh_u8,
    0xc5 as ::core::ffi::c_int as xxh_u8,
    0x81 as ::core::ffi::c_int as xxh_u8,
    0xa0 as ::core::ffi::c_int as xxh_u8,
    0xff as ::core::ffi::c_int as xxh_u8,
    0xfa as ::core::ffi::c_int as xxh_u8,
    0x13 as ::core::ffi::c_int as xxh_u8,
    0x63 as ::core::ffi::c_int as xxh_u8,
    0xeb as ::core::ffi::c_int as xxh_u8,
    0x17 as ::core::ffi::c_int as xxh_u8,
    0xd as ::core::ffi::c_int as xxh_u8,
    0xdd as ::core::ffi::c_int as xxh_u8,
    0x51 as ::core::ffi::c_int as xxh_u8,
    0xb7 as ::core::ffi::c_int as xxh_u8,
    0xf0 as ::core::ffi::c_int as xxh_u8,
    0xda as ::core::ffi::c_int as xxh_u8,
    0x49 as ::core::ffi::c_int as xxh_u8,
    0xd3 as ::core::ffi::c_int as xxh_u8,
    0x16 as ::core::ffi::c_int as xxh_u8,
    0x55 as ::core::ffi::c_int as xxh_u8,
    0x26 as ::core::ffi::c_int as xxh_u8,
    0x29 as ::core::ffi::c_int as xxh_u8,
    0xd4 as ::core::ffi::c_int as xxh_u8,
    0x68 as ::core::ffi::c_int as xxh_u8,
    0x9e as ::core::ffi::c_int as xxh_u8,
    0x2b as ::core::ffi::c_int as xxh_u8,
    0x16 as ::core::ffi::c_int as xxh_u8,
    0xbe as ::core::ffi::c_int as xxh_u8,
    0x58 as ::core::ffi::c_int as xxh_u8,
    0x7d as ::core::ffi::c_int as xxh_u8,
    0x47 as ::core::ffi::c_int as xxh_u8,
    0xa1 as ::core::ffi::c_int as xxh_u8,
    0xfc as ::core::ffi::c_int as xxh_u8,
    0x8f as ::core::ffi::c_int as xxh_u8,
    0xf8 as ::core::ffi::c_int as xxh_u8,
    0xb8 as ::core::ffi::c_int as xxh_u8,
    0xd1 as ::core::ffi::c_int as xxh_u8,
    0x7a as ::core::ffi::c_int as xxh_u8,
    0xd0 as ::core::ffi::c_int as xxh_u8,
    0x31 as ::core::ffi::c_int as xxh_u8,
    0xce as ::core::ffi::c_int as xxh_u8,
    0x45 as ::core::ffi::c_int as xxh_u8,
    0xcb as ::core::ffi::c_int as xxh_u8,
    0x3a as ::core::ffi::c_int as xxh_u8,
    0x8f as ::core::ffi::c_int as xxh_u8,
    0x95 as ::core::ffi::c_int as xxh_u8,
    0x16 as ::core::ffi::c_int as xxh_u8,
    0x4 as ::core::ffi::c_int as xxh_u8,
    0x28 as ::core::ffi::c_int as xxh_u8,
    0xaf as ::core::ffi::c_int as xxh_u8,
    0xd7 as ::core::ffi::c_int as xxh_u8,
    0xfb as ::core::ffi::c_int as xxh_u8,
    0xca as ::core::ffi::c_int as xxh_u8,
    0xbb as ::core::ffi::c_int as xxh_u8,
    0x4b as ::core::ffi::c_int as xxh_u8,
    0x40 as ::core::ffi::c_int as xxh_u8,
    0x7e as ::core::ffi::c_int as xxh_u8,
];
static mut PRIME_MX1: xxh_u64 = 0x165667919e3779f9 as xxh_u64;
static mut PRIME_MX2: xxh_u64 = 0x9fb21c651e98df25 as xxh_u64;
unsafe extern "C" fn XXH_mult64to128(mut lhs: xxh_u64, mut rhs: xxh_u64) -> XXH128_hash_t {
    let product: __uint128_t = (lhs as __uint128_t).wrapping_mul(rhs as __uint128_t);
    let mut r128: XXH128_hash_t = XXH128_hash_t {
        low64: 0,
        high64: 0,
    };
    r128.low64 = product as xxh_u64 as XXH64_hash_t;
    r128.high64 = (product >> 64 as ::core::ffi::c_int) as xxh_u64 as XXH64_hash_t;
    return r128;
}
unsafe extern "C" fn XXH3_mul128_fold64(mut lhs: xxh_u64, mut rhs: xxh_u64) -> xxh_u64 {
    let mut product: XXH128_hash_t = XXH_mult64to128(lhs, rhs);
    return product.low64 as xxh_u64 ^ product.high64 as xxh_u64;
}
unsafe extern "C" fn XXH_xorshift64(mut v64: xxh_u64, mut shift: ::core::ffi::c_int) -> xxh_u64 {
    return v64 ^ v64 >> shift;
}
unsafe extern "C" fn XXH3_avalanche(mut h64: xxh_u64) -> XXH64_hash_t {
    h64 = XXH_xorshift64(h64, 37 as ::core::ffi::c_int);
    h64 = h64.wrapping_mul(PRIME_MX1);
    h64 = XXH_xorshift64(h64, 32 as ::core::ffi::c_int);
    return h64 as XXH64_hash_t;
}
unsafe extern "C" fn XXH3_rrmxmx(mut h64: xxh_u64, mut len: xxh_u64) -> XXH64_hash_t {
    h64 = (h64 as ::core::ffi::c_ulong
        ^ (h64.rotate_left(49 as ::core::ffi::c_int as ::core::ffi::c_ulong as u32)
            ^ h64.rotate_left(24 as ::core::ffi::c_int as ::core::ffi::c_ulong as u32)))
        as xxh_u64;
    h64 = h64.wrapping_mul(PRIME_MX2);
    h64 ^= (h64 >> 35 as ::core::ffi::c_int).wrapping_add(len);
    h64 = h64.wrapping_mul(PRIME_MX2);
    return XXH_xorshift64(h64, 28 as ::core::ffi::c_int) as XXH64_hash_t;
}
unsafe extern "C" fn XXH3_len_1to3_64b(
    mut input: *const xxh_u8,
    mut len: size_t,
    mut secret: *const xxh_u8,
    mut seed: XXH64_hash_t,
) -> XXH64_hash_t {
    let c1: xxh_u8 = *input.offset(0 as ::core::ffi::c_int as isize);
    let c2: xxh_u8 = *input.offset((len >> 1 as ::core::ffi::c_int) as isize);
    let c3: xxh_u8 = *input.offset(len.wrapping_sub(1 as size_t) as isize);
    let combined: xxh_u32 = (c1 as xxh_u32) << 16 as ::core::ffi::c_int
        | (c2 as xxh_u32) << 24 as ::core::ffi::c_int
        | (c3 as xxh_u32) << 0 as ::core::ffi::c_int
        | (len as xxh_u32) << 8 as ::core::ffi::c_int;
    let bitflip: xxh_u64 = ((XXH_readLE32(secret as *const ::core::ffi::c_void)
        ^ XXH_readLE32(
            secret.offset(4 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_void
        )) as xxh_u64)
        .wrapping_add(seed as xxh_u64);
    let keyed: xxh_u64 = combined as xxh_u64 ^ bitflip;
    return XXH64_avalanche(keyed) as XXH64_hash_t;
}
unsafe extern "C" fn XXH3_len_4to8_64b(
    mut input: *const xxh_u8,
    mut len: size_t,
    mut secret: *const xxh_u8,
    mut seed: XXH64_hash_t,
) -> XXH64_hash_t {
    seed ^= ((XXH_swap32(seed as xxh_u32) as xxh_u64) << 32 as ::core::ffi::c_int) as XXH64_hash_t;
    let input1: xxh_u32 = XXH_readLE32(input as *const ::core::ffi::c_void) as xxh_u32;
    let input2: xxh_u32 = XXH_readLE32(
        input
            .offset(len as isize)
            .offset(-(4 as ::core::ffi::c_int as isize)) as *const ::core::ffi::c_void,
    ) as xxh_u32;
    let bitflip: xxh_u64 = (XXH_readLE64(
        secret.offset(8 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_void
    ) as xxh_u64
        ^ XXH_readLE64(
            secret.offset(16 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_void
        ) as xxh_u64)
        .wrapping_sub(seed as xxh_u64);
    let input64: xxh_u64 =
        (input2 as xxh_u64).wrapping_add((input1 as xxh_u64) << 32 as ::core::ffi::c_int);
    let keyed: xxh_u64 = input64 ^ bitflip;
    return XXH3_rrmxmx(keyed, len as xxh_u64);
}
unsafe extern "C" fn XXH3_len_9to16_64b(
    mut input: *const xxh_u8,
    mut len: size_t,
    mut secret: *const xxh_u8,
    mut seed: XXH64_hash_t,
) -> XXH64_hash_t {
    let bitflip1: xxh_u64 = (XXH_readLE64(
        secret.offset(24 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_void
    ) as xxh_u64
        ^ XXH_readLE64(
            secret.offset(32 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_void
        ) as xxh_u64)
        .wrapping_add(seed as xxh_u64);
    let bitflip2: xxh_u64 = (XXH_readLE64(
        secret.offset(40 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_void
    ) as xxh_u64
        ^ XXH_readLE64(
            secret.offset(48 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_void
        ) as xxh_u64)
        .wrapping_sub(seed as xxh_u64);
    let input_lo: xxh_u64 = XXH_readLE64(input as *const ::core::ffi::c_void) as xxh_u64 ^ bitflip1;
    let input_hi: xxh_u64 = XXH_readLE64(
        input
            .offset(len as isize)
            .offset(-(8 as ::core::ffi::c_int as isize)) as *const ::core::ffi::c_void,
    ) as xxh_u64
        ^ bitflip2;
    let acc: xxh_u64 = (len as xxh_u64)
        .wrapping_add(XXH_swap64(input_lo) as xxh_u64)
        .wrapping_add(input_hi)
        .wrapping_add(XXH3_mul128_fold64(input_lo, input_hi) as xxh_u64);
    return XXH3_avalanche(acc);
}
unsafe extern "C" fn XXH3_len_0to16_64b(
    mut input: *const xxh_u8,
    mut len: size_t,
    mut secret: *const xxh_u8,
    mut seed: XXH64_hash_t,
) -> XXH64_hash_t {
    if (len > 8 as size_t) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        return XXH3_len_9to16_64b(input, len, secret, seed);
    }
    if (len >= 4 as size_t) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        return XXH3_len_4to8_64b(input, len, secret, seed);
    }
    if len != 0 {
        return XXH3_len_1to3_64b(input, len, secret, seed);
    }
    return XXH64_avalanche(
        seed as xxh_u64
            ^ (XXH_readLE64(
                secret.offset(56 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_void
            ) ^ XXH_readLE64(
                secret.offset(64 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_void
            )),
    ) as XXH64_hash_t;
}
unsafe extern "C" fn XXH3_mix16B(
    mut input: *const xxh_u8,
    mut secret: *const xxh_u8,
    mut seed64: xxh_u64,
) -> xxh_u64 {
    let input_lo: xxh_u64 = XXH_readLE64(input as *const ::core::ffi::c_void) as xxh_u64;
    let input_hi: xxh_u64 =
        XXH_readLE64(input.offset(8 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_void)
            as xxh_u64;
    return XXH3_mul128_fold64(
        input_lo ^ XXH_readLE64(secret as *const ::core::ffi::c_void).wrapping_add(seed64),
        input_hi
            ^ XXH_readLE64(
                secret.offset(8 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_void
            )
            .wrapping_sub(seed64),
    );
}
unsafe extern "C" fn XXH3_len_17to128_64b(
    mut input: *const xxh_u8,
    mut len: size_t,
    mut secret: *const xxh_u8,
    mut secretSize: size_t,
    mut seed: XXH64_hash_t,
) -> XXH64_hash_t {
    let mut acc: xxh_u64 = (len as ::core::ffi::c_ulonglong).wrapping_mul(XXH_PRIME64_1) as xxh_u64;
    if len > 32 as size_t {
        if len > 64 as size_t {
            if len > 96 as size_t {
                acc = acc.wrapping_add(XXH3_mix16B(
                    input.offset(48 as ::core::ffi::c_int as isize),
                    secret.offset(96 as ::core::ffi::c_int as isize),
                    seed as xxh_u64,
                ));
                acc = acc.wrapping_add(XXH3_mix16B(
                    input
                        .offset(len as isize)
                        .offset(-(64 as ::core::ffi::c_int as isize)),
                    secret.offset(112 as ::core::ffi::c_int as isize),
                    seed as xxh_u64,
                ));
            }
            acc = acc.wrapping_add(XXH3_mix16B(
                input.offset(32 as ::core::ffi::c_int as isize),
                secret.offset(64 as ::core::ffi::c_int as isize),
                seed as xxh_u64,
            ));
            acc = acc.wrapping_add(XXH3_mix16B(
                input
                    .offset(len as isize)
                    .offset(-(48 as ::core::ffi::c_int as isize)),
                secret.offset(80 as ::core::ffi::c_int as isize),
                seed as xxh_u64,
            ));
        }
        acc = acc.wrapping_add(XXH3_mix16B(
            input.offset(16 as ::core::ffi::c_int as isize),
            secret.offset(32 as ::core::ffi::c_int as isize),
            seed as xxh_u64,
        ));
        acc = acc.wrapping_add(XXH3_mix16B(
            input
                .offset(len as isize)
                .offset(-(32 as ::core::ffi::c_int as isize)),
            secret.offset(48 as ::core::ffi::c_int as isize),
            seed as xxh_u64,
        ));
    }
    acc = acc.wrapping_add(XXH3_mix16B(
        input.offset(0 as ::core::ffi::c_int as isize),
        secret.offset(0 as ::core::ffi::c_int as isize),
        seed as xxh_u64,
    ));
    acc = acc.wrapping_add(XXH3_mix16B(
        input
            .offset(len as isize)
            .offset(-(16 as ::core::ffi::c_int as isize)),
        secret.offset(16 as ::core::ffi::c_int as isize),
        seed as xxh_u64,
    ));
    return XXH3_avalanche(acc);
}
unsafe extern "C" fn XXH3_len_129to240_64b(
    mut input: *const xxh_u8,
    mut len: size_t,
    mut secret: *const xxh_u8,
    mut secretSize: size_t,
    mut seed: XXH64_hash_t,
) -> XXH64_hash_t {
    let mut acc: xxh_u64 = (len as ::core::ffi::c_ulonglong).wrapping_mul(XXH_PRIME64_1) as xxh_u64;
    let mut acc_end: xxh_u64 = 0;
    let nbRounds: ::core::ffi::c_uint =
        (len as ::core::ffi::c_uint).wrapping_div(16 as ::core::ffi::c_uint);
    let mut i: ::core::ffi::c_uint = 0;
    i = 0 as ::core::ffi::c_uint;
    while i < 8 as ::core::ffi::c_uint {
        acc = acc.wrapping_add(XXH3_mix16B(
            input.offset((16 as ::core::ffi::c_uint).wrapping_mul(i) as isize),
            secret.offset((16 as ::core::ffi::c_uint).wrapping_mul(i) as isize),
            seed as xxh_u64,
        ));
        i = i.wrapping_add(1);
    }
    acc_end = XXH3_mix16B(
        input
            .offset(len as isize)
            .offset(-(16 as ::core::ffi::c_int as isize)),
        secret
            .offset(XXH3_SECRET_SIZE_MIN as isize)
            .offset(-(XXH3_MIDSIZE_LASTOFFSET as isize)),
        seed as xxh_u64,
    );
    acc = XXH3_avalanche(acc) as xxh_u64;
    i = 8 as ::core::ffi::c_uint;
    while i < nbRounds {
        acc_end = acc_end.wrapping_add(XXH3_mix16B(
            input.offset((16 as ::core::ffi::c_uint).wrapping_mul(i) as isize),
            secret
                .offset(
                    (16 as ::core::ffi::c_uint)
                        .wrapping_mul(i.wrapping_sub(8 as ::core::ffi::c_uint))
                        as isize,
                )
                .offset(XXH3_MIDSIZE_STARTOFFSET as isize),
            seed as xxh_u64,
        ));
        i = i.wrapping_add(1);
    }
    return XXH3_avalanche(acc.wrapping_add(acc_end));
}
pub const XXH3_MIDSIZE_STARTOFFSET: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const XXH3_MIDSIZE_LASTOFFSET: ::core::ffi::c_int = 17 as ::core::ffi::c_int;
pub const XXH_STRIPE_LEN: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
pub const XXH_SECRET_CONSUME_RATE: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const XXH_ACC_NB: usize =
    (XXH_STRIPE_LEN as usize).wrapping_div(::core::mem::size_of::<xxh_u64>() as usize);
unsafe extern "C" fn XXH_writeLE64(mut dst: *mut ::core::ffi::c_void, mut v64: xxh_u64) {
    if XXH_CPU_LITTLE_ENDIAN == 0 {
        v64 = XXH_swap64(v64);
    }
    memcpy(
        dst,
        &raw mut v64 as *const ::core::ffi::c_void,
        ::core::mem::size_of::<xxh_u64>() as size_t,
    );
}
unsafe extern "C" fn XXH_mult32to64_add64(
    mut lhs: xxh_u64,
    mut rhs: xxh_u64,
    mut acc: xxh_u64,
) -> xxh_u64 {
    return (lhs as xxh_u32 as xxh_u64)
        .wrapping_mul(rhs as xxh_u32 as xxh_u64)
        .wrapping_add(acc);
}
unsafe extern "C" fn XXH3_scalarRound(
    mut acc: *mut ::core::ffi::c_void,
    mut input: *const ::core::ffi::c_void,
    mut secret: *const ::core::ffi::c_void,
    mut lane: size_t,
) {
    let mut xacc: *mut xxh_u64 = acc as *mut xxh_u64;
    let mut xinput: *const xxh_u8 = input as *const xxh_u8;
    let mut xsecret: *const xxh_u8 = secret as *const xxh_u8;
    let data_val: xxh_u64 = XXH_readLE64(
        xinput.offset(lane.wrapping_mul(8 as size_t) as isize) as *const ::core::ffi::c_void
    ) as xxh_u64;
    let data_key: xxh_u64 = data_val
        ^ XXH_readLE64(
            xsecret.offset(lane.wrapping_mul(8 as size_t) as isize) as *const ::core::ffi::c_void
        ) as xxh_u64;
    let ref mut fresh8 = *xacc.offset((lane ^ 1 as size_t) as isize);
    *fresh8 = (*fresh8).wrapping_add(data_val);
    *xacc.offset(lane as isize) = XXH_mult32to64_add64(
        data_key,
        data_key >> 32 as ::core::ffi::c_int,
        *xacc.offset(lane as isize),
    );
}
unsafe extern "C" fn XXH3_accumulate_512_scalar(
    mut acc: *mut ::core::ffi::c_void,
    mut input: *const ::core::ffi::c_void,
    mut secret: *const ::core::ffi::c_void,
) {
    let mut i: size_t = 0;
    i = 0 as size_t;
    while i < XXH_ACC_NB {
        XXH3_scalarRound(acc, input, secret, i);
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn XXH3_accumulate_scalar(
    mut acc: *mut xxh_u64,
    mut input: *const xxh_u8,
    mut secret: *const xxh_u8,
    mut nbStripes: size_t,
) {
    let mut n: size_t = 0;
    n = 0 as size_t;
    while n < nbStripes {
        let in_0: *const xxh_u8 = input.offset(n.wrapping_mul(XXH_STRIPE_LEN as size_t) as isize);
        XXH3_accumulate_512_scalar(
            acc as *mut ::core::ffi::c_void,
            in_0 as *const ::core::ffi::c_void,
            secret.offset(n.wrapping_mul(XXH_SECRET_CONSUME_RATE as size_t) as isize)
                as *const ::core::ffi::c_void,
        );
        n = n.wrapping_add(1);
    }
}
unsafe extern "C" fn XXH3_scalarScrambleRound(
    mut acc: *mut ::core::ffi::c_void,
    mut secret: *const ::core::ffi::c_void,
    mut lane: size_t,
) {
    let xacc: *mut xxh_u64 = acc as *mut xxh_u64;
    let xsecret: *const xxh_u8 = secret as *const xxh_u8;
    let key64: xxh_u64 = XXH_readLE64(
        xsecret.offset(lane.wrapping_mul(8 as size_t) as isize) as *const ::core::ffi::c_void
    ) as xxh_u64;
    let mut acc64: xxh_u64 = *xacc.offset(lane as isize);
    acc64 = XXH_xorshift64(acc64, 47 as ::core::ffi::c_int);
    acc64 ^= key64;
    acc64 = acc64.wrapping_mul(XXH_PRIME32_1 as xxh_u64);
    *xacc.offset(lane as isize) = acc64;
}
unsafe extern "C" fn XXH3_scrambleAcc_scalar(
    mut acc: *mut ::core::ffi::c_void,
    mut secret: *const ::core::ffi::c_void,
) {
    let mut i: size_t = 0;
    i = 0 as size_t;
    while i < XXH_ACC_NB {
        XXH3_scalarScrambleRound(acc, secret, i);
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn XXH3_initCustomSecret_scalar(
    mut customSecret: *mut ::core::ffi::c_void,
    mut seed64: xxh_u64,
) {
    let mut kSecretPtr: *const xxh_u8 = &raw const XXH3_kSecret as *const xxh_u8;
    let nbRounds: ::core::ffi::c_int = XXH_SECRET_DEFAULT_SIZE / 16 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < nbRounds {
        let mut lo: xxh_u64 =
            XXH_readLE64(kSecretPtr.offset((16 as ::core::ffi::c_int * i) as isize)
                as *const ::core::ffi::c_void)
            .wrapping_add(seed64);
        let mut hi: xxh_u64 = XXH_readLE64(
            kSecretPtr
                .offset((16 as ::core::ffi::c_int * i) as isize)
                .offset(8 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_void,
        )
        .wrapping_sub(seed64);
        XXH_writeLE64(
            (customSecret as *mut xxh_u8).offset((16 as ::core::ffi::c_int * i) as isize)
                as *mut ::core::ffi::c_void,
            lo,
        );
        XXH_writeLE64(
            (customSecret as *mut xxh_u8)
                .offset((16 as ::core::ffi::c_int * i) as isize)
                .offset(8 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_void,
            hi,
        );
        i += 1;
    }
}
unsafe extern "C" fn XXH3_hashLong_internal_loop(
    mut acc: *mut xxh_u64,
    mut input: *const xxh_u8,
    mut len: size_t,
    mut secret: *const xxh_u8,
    mut secretSize: size_t,
    mut f_acc: XXH3_f_accumulate,
    mut f_scramble: XXH3_f_scrambleAcc,
) {
    let nbStripesPerBlock: size_t = secretSize
        .wrapping_sub(XXH_STRIPE_LEN as size_t)
        .wrapping_div(XXH_SECRET_CONSUME_RATE as size_t);
    let block_len: size_t = (XXH_STRIPE_LEN as size_t).wrapping_mul(nbStripesPerBlock);
    let nb_blocks: size_t = len.wrapping_sub(1 as size_t).wrapping_div(block_len);
    let mut n: size_t = 0;
    n = 0 as size_t;
    while n < nb_blocks {
        f_acc.expect("non-null function pointer")(
            acc,
            input.offset(n.wrapping_mul(block_len) as isize),
            secret,
            nbStripesPerBlock,
        );
        f_scramble.expect("non-null function pointer")(
            acc as *mut ::core::ffi::c_void,
            secret
                .offset(secretSize as isize)
                .offset(-(XXH_STRIPE_LEN as isize)) as *const ::core::ffi::c_void,
        );
        n = n.wrapping_add(1);
    }
    let nbStripes: size_t = len
        .wrapping_sub(1 as size_t)
        .wrapping_sub(block_len.wrapping_mul(nb_blocks))
        .wrapping_div(XXH_STRIPE_LEN as size_t);
    f_acc.expect("non-null function pointer")(
        acc,
        input.offset(nb_blocks.wrapping_mul(block_len) as isize),
        secret,
        nbStripes,
    );
    let p: *const xxh_u8 = input
        .offset(len as isize)
        .offset(-(XXH_STRIPE_LEN as isize));
    XXH3_accumulate_512_scalar(
        acc as *mut ::core::ffi::c_void,
        p as *const ::core::ffi::c_void,
        secret
            .offset(secretSize as isize)
            .offset(-(XXH_STRIPE_LEN as isize))
            .offset(-(XXH_SECRET_LASTACC_START as isize)) as *const ::core::ffi::c_void,
    );
}
pub const XXH_SECRET_LASTACC_START: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
unsafe extern "C" fn XXH3_mix2Accs(mut acc: *const xxh_u64, mut secret: *const xxh_u8) -> xxh_u64 {
    return XXH3_mul128_fold64(
        *acc.offset(0 as ::core::ffi::c_int as isize)
            ^ XXH_readLE64(secret as *const ::core::ffi::c_void),
        *acc.offset(1 as ::core::ffi::c_int as isize)
            ^ XXH_readLE64(
                secret.offset(8 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_void
            ),
    );
}
unsafe extern "C" fn XXH3_mergeAccs(
    mut acc: *const xxh_u64,
    mut secret: *const xxh_u8,
    mut start: xxh_u64,
) -> XXH64_hash_t {
    let mut result64: xxh_u64 = start;
    let mut i: size_t = 0 as size_t;
    i = 0 as size_t;
    while i < 4 as size_t {
        result64 = result64.wrapping_add(XXH3_mix2Accs(
            acc.offset((2 as size_t).wrapping_mul(i) as isize),
            secret.offset((16 as size_t).wrapping_mul(i) as isize),
        ));
        i = i.wrapping_add(1);
    }
    return XXH3_avalanche(result64);
}
pub const XXH_SECRET_MERGEACCS_START: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
unsafe extern "C" fn XXH3_finalizeLong_64b(
    mut acc: *const xxh_u64,
    mut secret: *const xxh_u8,
    mut len: xxh_u64,
) -> XXH64_hash_t {
    return XXH3_mergeAccs(
        acc,
        secret.offset(XXH_SECRET_MERGEACCS_START as isize),
        (len as ::core::ffi::c_ulonglong).wrapping_mul(XXH_PRIME64_1) as xxh_u64,
    );
}
unsafe extern "C" fn XXH3_hashLong_64b_internal(
    mut input: *const ::core::ffi::c_void,
    mut len: size_t,
    mut secret: *const ::core::ffi::c_void,
    mut secretSize: size_t,
    mut f_acc: XXH3_f_accumulate,
    mut f_scramble: XXH3_f_scrambleAcc,
) -> XXH64_hash_t {
    let mut acc: [xxh_u64; 8] = [
        XXH_PRIME32_3 as xxh_u64,
        XXH_PRIME64_1 as xxh_u64,
        XXH_PRIME64_2 as xxh_u64,
        XXH_PRIME64_3 as xxh_u64,
        XXH_PRIME64_4 as xxh_u64,
        XXH_PRIME32_2 as xxh_u64,
        XXH_PRIME64_5 as xxh_u64,
        XXH_PRIME32_1 as xxh_u64,
    ];
    XXH3_hashLong_internal_loop(
        &raw mut acc as *mut xxh_u64,
        input as *const xxh_u8,
        len,
        secret as *const xxh_u8,
        secretSize,
        f_acc,
        f_scramble,
    );
    return XXH3_finalizeLong_64b(
        &raw mut acc as *mut xxh_u64,
        secret as *const xxh_u8,
        len as xxh_u64,
    );
}
unsafe extern "C" fn XXH3_hashLong_64b_withSecret(
    mut input: *const ::core::ffi::c_void,
    mut len: size_t,
    mut seed64: XXH64_hash_t,
    mut secret: *const xxh_u8,
    mut secretLen: size_t,
) -> XXH64_hash_t {
    return XXH3_hashLong_64b_internal(
        input,
        len,
        secret as *const ::core::ffi::c_void,
        secretLen,
        Some(
            XXH3_accumulate_scalar
                as unsafe extern "C" fn(*mut xxh_u64, *const xxh_u8, *const xxh_u8, size_t) -> (),
        ),
        Some(
            XXH3_scrambleAcc_scalar
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, *const ::core::ffi::c_void) -> (),
        ),
    );
}
unsafe extern "C" fn XXH3_hashLong_64b_default(
    mut input: *const ::core::ffi::c_void,
    mut len: size_t,
    mut seed64: XXH64_hash_t,
    mut secret: *const xxh_u8,
    mut secretLen: size_t,
) -> XXH64_hash_t {
    return XXH3_hashLong_64b_internal(
        input,
        len,
        &raw const XXH3_kSecret as *const xxh_u8 as *const ::core::ffi::c_void,
        ::core::mem::size_of::<[xxh_u8; 192]>() as size_t,
        Some(
            XXH3_accumulate_scalar
                as unsafe extern "C" fn(*mut xxh_u64, *const xxh_u8, *const xxh_u8, size_t) -> (),
        ),
        Some(
            XXH3_scrambleAcc_scalar
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, *const ::core::ffi::c_void) -> (),
        ),
    );
}
unsafe extern "C" fn XXH3_hashLong_64b_withSeed_internal(
    mut input: *const ::core::ffi::c_void,
    mut len: size_t,
    mut seed: XXH64_hash_t,
    mut f_acc: XXH3_f_accumulate,
    mut f_scramble: XXH3_f_scrambleAcc,
    mut f_initSec: XXH3_f_initCustomSecret,
) -> XXH64_hash_t {
    if seed == 0 as XXH64_hash_t {
        return XXH3_hashLong_64b_internal(
            input,
            len,
            &raw const XXH3_kSecret as *const xxh_u8 as *const ::core::ffi::c_void,
            ::core::mem::size_of::<[xxh_u8; 192]>() as size_t,
            f_acc,
            f_scramble,
        );
    }
    let mut secret: [xxh_u8; 192] = [0; 192];
    f_initSec.expect("non-null function pointer")(
        &raw mut secret as *mut xxh_u8 as *mut ::core::ffi::c_void,
        seed as xxh_u64,
    );
    return XXH3_hashLong_64b_internal(
        input,
        len,
        &raw mut secret as *mut xxh_u8 as *const ::core::ffi::c_void,
        ::core::mem::size_of::<[xxh_u8; 192]>() as size_t,
        f_acc,
        f_scramble,
    );
}
unsafe extern "C" fn XXH3_hashLong_64b_withSeed(
    mut input: *const ::core::ffi::c_void,
    mut len: size_t,
    mut seed: XXH64_hash_t,
    mut secret: *const xxh_u8,
    mut secretLen: size_t,
) -> XXH64_hash_t {
    return XXH3_hashLong_64b_withSeed_internal(
        input,
        len,
        seed,
        Some(
            XXH3_accumulate_scalar
                as unsafe extern "C" fn(*mut xxh_u64, *const xxh_u8, *const xxh_u8, size_t) -> (),
        ),
        Some(
            XXH3_scrambleAcc_scalar
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, *const ::core::ffi::c_void) -> (),
        ),
        Some(
            XXH3_initCustomSecret_scalar
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, xxh_u64) -> (),
        ),
    );
}
unsafe extern "C" fn XXH3_64bits_internal(
    mut input: *const ::core::ffi::c_void,
    mut len: size_t,
    mut seed64: XXH64_hash_t,
    mut secret: *const ::core::ffi::c_void,
    mut secretLen: size_t,
    mut f_hashLong: XXH3_hashLong64_f,
) -> XXH64_hash_t {
    if len <= 16 as size_t {
        return XXH3_len_0to16_64b(input as *const xxh_u8, len, secret as *const xxh_u8, seed64);
    }
    if len <= 128 as size_t {
        return XXH3_len_17to128_64b(
            input as *const xxh_u8,
            len,
            secret as *const xxh_u8,
            secretLen,
            seed64,
        );
    }
    if len <= XXH3_MIDSIZE_MAX as size_t {
        return XXH3_len_129to240_64b(
            input as *const xxh_u8,
            len,
            secret as *const xxh_u8,
            secretLen,
            seed64,
        );
    }
    return f_hashLong.expect("non-null function pointer")(
        input,
        len,
        seed64,
        secret as *const xxh_u8,
        secretLen,
    );
}
#[no_mangle]
pub unsafe extern "C" fn XXH3_64bits(
    mut input: *const ::core::ffi::c_void,
    mut length: size_t,
) -> XXH64_hash_t {
    return XXH3_64bits_internal(
        input,
        length,
        0 as XXH64_hash_t,
        &raw const XXH3_kSecret as *const xxh_u8 as *const ::core::ffi::c_void,
        ::core::mem::size_of::<[xxh_u8; 192]>() as size_t,
        Some(
            XXH3_hashLong_64b_default
                as unsafe extern "C" fn(
                    *const ::core::ffi::c_void,
                    size_t,
                    XXH64_hash_t,
                    *const xxh_u8,
                    size_t,
                ) -> XXH64_hash_t,
        ),
    );
}
#[no_mangle]
pub unsafe extern "C" fn XXH3_64bits_withSecret(
    mut input: *const ::core::ffi::c_void,
    mut length: size_t,
    mut secret: *const ::core::ffi::c_void,
    mut secretSize: size_t,
) -> XXH64_hash_t {
    return XXH3_64bits_internal(
        input,
        length,
        0 as XXH64_hash_t,
        secret,
        secretSize,
        Some(
            XXH3_hashLong_64b_withSecret
                as unsafe extern "C" fn(
                    *const ::core::ffi::c_void,
                    size_t,
                    XXH64_hash_t,
                    *const xxh_u8,
                    size_t,
                ) -> XXH64_hash_t,
        ),
    );
}
#[no_mangle]
pub unsafe extern "C" fn XXH3_64bits_withSeed(
    mut input: *const ::core::ffi::c_void,
    mut length: size_t,
    mut seed: XXH64_hash_t,
) -> XXH64_hash_t {
    return XXH3_64bits_internal(
        input,
        length,
        seed,
        &raw const XXH3_kSecret as *const xxh_u8 as *const ::core::ffi::c_void,
        ::core::mem::size_of::<[xxh_u8; 192]>() as size_t,
        Some(
            XXH3_hashLong_64b_withSeed
                as unsafe extern "C" fn(
                    *const ::core::ffi::c_void,
                    size_t,
                    XXH64_hash_t,
                    *const xxh_u8,
                    size_t,
                ) -> XXH64_hash_t,
        ),
    );
}
#[no_mangle]
pub unsafe extern "C" fn XXH3_64bits_withSecretandSeed(
    mut input: *const ::core::ffi::c_void,
    mut length: size_t,
    mut secret: *const ::core::ffi::c_void,
    mut secretSize: size_t,
    mut seed: XXH64_hash_t,
) -> XXH64_hash_t {
    if length <= XXH3_MIDSIZE_MAX as size_t {
        return XXH3_64bits_internal(
            input,
            length,
            seed,
            &raw const XXH3_kSecret as *const xxh_u8 as *const ::core::ffi::c_void,
            ::core::mem::size_of::<[xxh_u8; 192]>() as size_t,
            None,
        );
    }
    return XXH3_hashLong_64b_withSecret(input, length, seed, secret as *const xxh_u8, secretSize);
}
unsafe extern "C" fn XXH_alignedMalloc(
    mut s: size_t,
    mut align: size_t,
) -> *mut ::core::ffi::c_void {
    let mut base: *mut xxh_u8 = XXH_malloc(s.wrapping_add(align)) as *mut xxh_u8;
    if !base.is_null() {
        let mut offset: size_t =
            align.wrapping_sub(base as size_t & align.wrapping_sub(1 as size_t));
        let mut ptr: *mut xxh_u8 = base.offset(offset as isize);
        *ptr.offset(-(1 as ::core::ffi::c_int) as isize) = offset as xxh_u8;
        return ptr as *mut ::core::ffi::c_void;
    }
    return NULL;
}
unsafe extern "C" fn XXH_alignedFree(mut p: *mut ::core::ffi::c_void) {
    if !p.is_null() {
        let mut ptr: *mut xxh_u8 = p as *mut xxh_u8;
        let mut offset: xxh_u8 = *ptr.offset(-(1 as ::core::ffi::c_int) as isize);
        let mut base: *mut xxh_u8 = ptr.offset(-(offset as ::core::ffi::c_int as isize));
        XXH_free(base as *mut ::core::ffi::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn XXH3_createState() -> *mut XXH3_state_t {
    let state: *mut XXH3_state_t = XXH_alignedMalloc(
        ::core::mem::size_of::<XXH3_state_t>() as size_t,
        64 as size_t,
    ) as *mut XXH3_state_t;
    if state.is_null() {
        return ::core::ptr::null_mut::<XXH3_state_t>();
    }
    let mut tmp_xxh3_state_ptr: *mut XXH3_state_t = state;
    (*tmp_xxh3_state_ptr).seed = 0 as XXH64_hash_t;
    (*tmp_xxh3_state_ptr).extSecret = ::core::ptr::null::<::core::ffi::c_uchar>();
    return state;
}
#[no_mangle]
pub unsafe extern "C" fn XXH3_freeState(mut statePtr: *mut XXH3_state_t) -> XXH_errorcode {
    XXH_alignedFree(statePtr as *mut ::core::ffi::c_void);
    return XXH_OK;
}
#[no_mangle]
pub unsafe extern "C" fn XXH3_copyState(
    mut dst_state: *mut XXH3_state_t,
    mut src_state: *const XXH3_state_t,
) {
    memcpy(
        dst_state as *mut ::core::ffi::c_void,
        src_state as *const ::core::ffi::c_void,
        ::core::mem::size_of::<XXH3_state_t>() as size_t,
    );
}
unsafe extern "C" fn XXH3_reset_internal(
    mut statePtr: *mut XXH3_state_t,
    mut seed: XXH64_hash_t,
    mut secret: *const ::core::ffi::c_void,
    mut secretSize: size_t,
) {
    let initStart: size_t = 512 as size_t;
    let initLength: size_t = (536 as size_t).wrapping_sub(initStart);
    memset(
        (statePtr as *mut ::core::ffi::c_char).offset(initStart as isize)
            as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        initLength,
    );
    (*statePtr).acc[0 as ::core::ffi::c_int as usize] = XXH_PRIME32_3 as XXH64_hash_t;
    (*statePtr).acc[1 as ::core::ffi::c_int as usize] = XXH_PRIME64_1 as XXH64_hash_t;
    (*statePtr).acc[2 as ::core::ffi::c_int as usize] = XXH_PRIME64_2 as XXH64_hash_t;
    (*statePtr).acc[3 as ::core::ffi::c_int as usize] = XXH_PRIME64_3 as XXH64_hash_t;
    (*statePtr).acc[4 as ::core::ffi::c_int as usize] = XXH_PRIME64_4 as XXH64_hash_t;
    (*statePtr).acc[5 as ::core::ffi::c_int as usize] = XXH_PRIME32_2 as XXH64_hash_t;
    (*statePtr).acc[6 as ::core::ffi::c_int as usize] = XXH_PRIME64_5 as XXH64_hash_t;
    (*statePtr).acc[7 as ::core::ffi::c_int as usize] = XXH_PRIME32_1 as XXH64_hash_t;
    (*statePtr).seed = seed;
    (*statePtr).useSeed = (seed != 0 as XXH64_hash_t) as ::core::ffi::c_int as XXH32_hash_t;
    (*statePtr).extSecret = secret as *const ::core::ffi::c_uchar;
    (*statePtr).secretLimit = secretSize.wrapping_sub(XXH_STRIPE_LEN as size_t);
    (*statePtr).nbStripesPerBlock = (*statePtr)
        .secretLimit
        .wrapping_div(XXH_SECRET_CONSUME_RATE as size_t);
}
#[no_mangle]
pub unsafe extern "C" fn XXH3_64bits_reset(mut statePtr: *mut XXH3_state_t) -> XXH_errorcode {
    if statePtr.is_null() {
        return XXH_ERROR;
    }
    XXH3_reset_internal(
        statePtr,
        0 as XXH64_hash_t,
        &raw const XXH3_kSecret as *const xxh_u8 as *const ::core::ffi::c_void,
        XXH_SECRET_DEFAULT_SIZE as size_t,
    );
    return XXH_OK;
}
#[no_mangle]
pub unsafe extern "C" fn XXH3_64bits_reset_withSecret(
    mut statePtr: *mut XXH3_state_t,
    mut secret: *const ::core::ffi::c_void,
    mut secretSize: size_t,
) -> XXH_errorcode {
    if statePtr.is_null() {
        return XXH_ERROR;
    }
    XXH3_reset_internal(statePtr, 0 as XXH64_hash_t, secret, secretSize);
    if secret.is_null() {
        return XXH_ERROR;
    }
    if secretSize < XXH3_SECRET_SIZE_MIN as size_t {
        return XXH_ERROR;
    }
    return XXH_OK;
}
#[no_mangle]
pub unsafe extern "C" fn XXH3_64bits_reset_withSeed(
    mut statePtr: *mut XXH3_state_t,
    mut seed: XXH64_hash_t,
) -> XXH_errorcode {
    if statePtr.is_null() {
        return XXH_ERROR;
    }
    if seed == 0 as XXH64_hash_t {
        return XXH3_64bits_reset(statePtr);
    }
    if seed != (*statePtr).seed || !(*statePtr).extSecret.is_null() {
        XXH3_initCustomSecret_scalar(
            &raw mut (*statePtr).customSecret as *mut ::core::ffi::c_uchar
                as *mut ::core::ffi::c_void,
            seed as xxh_u64,
        );
    }
    XXH3_reset_internal(
        statePtr,
        seed,
        ::core::ptr::null::<::core::ffi::c_void>(),
        XXH_SECRET_DEFAULT_SIZE as size_t,
    );
    return XXH_OK;
}
#[no_mangle]
pub unsafe extern "C" fn XXH3_64bits_reset_withSecretandSeed(
    mut statePtr: *mut XXH3_state_t,
    mut secret: *const ::core::ffi::c_void,
    mut secretSize: size_t,
    mut seed64: XXH64_hash_t,
) -> XXH_errorcode {
    if statePtr.is_null() {
        return XXH_ERROR;
    }
    if secret.is_null() {
        return XXH_ERROR;
    }
    if secretSize < XXH3_SECRET_SIZE_MIN as size_t {
        return XXH_ERROR;
    }
    XXH3_reset_internal(statePtr, seed64, secret, secretSize);
    (*statePtr).useSeed = 1 as XXH32_hash_t;
    return XXH_OK;
}
unsafe extern "C" fn XXH3_consumeStripes(
    mut acc: *mut xxh_u64,
    mut nbStripesSoFarPtr: *mut size_t,
    mut nbStripesPerBlock: size_t,
    mut input: *const xxh_u8,
    mut nbStripes: size_t,
    mut secret: *const xxh_u8,
    mut secretLimit: size_t,
    mut f_acc: XXH3_f_accumulate,
    mut f_scramble: XXH3_f_scrambleAcc,
) -> *const xxh_u8 {
    let mut initialSecret: *const xxh_u8 = secret
        .offset((*nbStripesSoFarPtr).wrapping_mul(XXH_SECRET_CONSUME_RATE as size_t) as isize);
    if nbStripes >= nbStripesPerBlock.wrapping_sub(*nbStripesSoFarPtr) {
        let mut nbStripesThisIter: size_t = nbStripesPerBlock.wrapping_sub(*nbStripesSoFarPtr);
        loop {
            f_acc.expect("non-null function pointer")(acc, input, initialSecret, nbStripesThisIter);
            f_scramble.expect("non-null function pointer")(
                acc as *mut ::core::ffi::c_void,
                secret.offset(secretLimit as isize) as *const ::core::ffi::c_void,
            );
            input = input.offset(nbStripesThisIter.wrapping_mul(XXH_STRIPE_LEN as size_t) as isize);
            nbStripes = nbStripes.wrapping_sub(nbStripesThisIter);
            nbStripesThisIter = nbStripesPerBlock;
            initialSecret = secret;
            if !(nbStripes >= nbStripesPerBlock) {
                break;
            }
        }
        *nbStripesSoFarPtr = 0 as size_t;
    }
    if nbStripes > 0 as size_t {
        f_acc.expect("non-null function pointer")(acc, input, initialSecret, nbStripes);
        input = input.offset(nbStripes.wrapping_mul(XXH_STRIPE_LEN as size_t) as isize);
        *nbStripesSoFarPtr = (*nbStripesSoFarPtr).wrapping_add(nbStripes);
    }
    return input;
}
unsafe extern "C" fn XXH3_update(
    state: *mut XXH3_state_t,
    mut input: *const xxh_u8,
    mut len: size_t,
    mut f_acc: XXH3_f_accumulate,
    mut f_scramble: XXH3_f_scrambleAcc,
) -> XXH_errorcode {
    if input.is_null() {
        return XXH_OK;
    }
    (*state).totalLen = ((*state).totalLen as ::core::ffi::c_ulong)
        .wrapping_add(len as ::core::ffi::c_ulong) as XXH64_hash_t
        as XXH64_hash_t;
    if len
        <= (XXH3_INTERNALBUFFER_SIZE as XXH32_hash_t).wrapping_sub((*state).bufferedSize) as size_t
    {
        memcpy(
            (&raw mut (*state).buffer as *mut ::core::ffi::c_uchar)
                .offset((*state).bufferedSize as isize) as *mut ::core::ffi::c_void,
            input as *const ::core::ffi::c_void,
            len,
        );
        (*state).bufferedSize = (*state).bufferedSize.wrapping_add(len as XXH32_hash_t);
        return XXH_OK;
    }
    let bEnd: *const xxh_u8 = input.offset(len as isize);
    let secret: *const ::core::ffi::c_uchar = if (*state).extSecret.is_null() {
        &raw mut (*state).customSecret as *mut ::core::ffi::c_uchar as *const ::core::ffi::c_uchar
    } else {
        (*state).extSecret
    };
    let acc: *mut xxh_u64 = &raw mut (*state).acc as *mut xxh_u64;
    if (*state).bufferedSize != 0 {
        let loadSize: size_t = (XXH3_INTERNALBUFFER_SIZE as XXH32_hash_t)
            .wrapping_sub((*state).bufferedSize) as size_t;
        memcpy(
            (&raw mut (*state).buffer as *mut ::core::ffi::c_uchar)
                .offset((*state).bufferedSize as isize) as *mut ::core::ffi::c_void,
            input as *const ::core::ffi::c_void,
            loadSize,
        );
        input = input.offset(loadSize as isize);
        XXH3_consumeStripes(
            acc,
            &raw mut (*state).nbStripesSoFar,
            (*state).nbStripesPerBlock,
            &raw mut (*state).buffer as *mut ::core::ffi::c_uchar,
            XXH3_INTERNALBUFFER_STRIPES as size_t,
            secret as *const xxh_u8,
            (*state).secretLimit,
            f_acc,
            f_scramble,
        );
        (*state).bufferedSize = 0 as XXH32_hash_t;
    }
    if bEnd.offset_from(input) as ::core::ffi::c_long
        > XXH3_INTERNALBUFFER_SIZE as ::core::ffi::c_long
    {
        let mut nbStripes: size_t = (bEnd
            .offset(-(1 as ::core::ffi::c_int as isize))
            .offset_from(input) as ::core::ffi::c_long
            as size_t)
            .wrapping_div(XXH_STRIPE_LEN as size_t);
        input = XXH3_consumeStripes(
            acc,
            &raw mut (*state).nbStripesSoFar,
            (*state).nbStripesPerBlock,
            input,
            nbStripes,
            secret as *const xxh_u8,
            (*state).secretLimit,
            f_acc,
            f_scramble,
        );
        memcpy(
            (&raw mut (*state).buffer as *mut ::core::ffi::c_uchar)
                .offset(::core::mem::size_of::<[::core::ffi::c_uchar; 256]>() as usize as isize)
                .offset(-(XXH_STRIPE_LEN as isize)) as *mut ::core::ffi::c_void,
            input.offset(-(XXH_STRIPE_LEN as isize)) as *const ::core::ffi::c_void,
            XXH_STRIPE_LEN as size_t,
        );
    }
    memcpy(
        &raw mut (*state).buffer as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_void,
        input as *const ::core::ffi::c_void,
        bEnd.offset_from(input) as ::core::ffi::c_long as size_t,
    );
    (*state).bufferedSize = bEnd.offset_from(input) as ::core::ffi::c_long as XXH32_hash_t;
    return XXH_OK;
}
pub const XXH3_INTERNALBUFFER_STRIPES: ::core::ffi::c_int =
    XXH3_INTERNALBUFFER_SIZE / XXH_STRIPE_LEN;
unsafe extern "C" fn XXH3_update_regular(
    mut state: *mut XXH3_state_t,
    mut input: *const ::core::ffi::c_void,
    mut len: size_t,
) -> XXH_errorcode {
    return XXH3_update(
        state,
        input as *const xxh_u8,
        len,
        Some(
            XXH3_accumulate_scalar
                as unsafe extern "C" fn(*mut xxh_u64, *const xxh_u8, *const xxh_u8, size_t) -> (),
        ),
        Some(
            XXH3_scrambleAcc_scalar
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, *const ::core::ffi::c_void) -> (),
        ),
    );
}
#[no_mangle]
pub unsafe extern "C" fn XXH3_64bits_update(
    mut state: *mut XXH3_state_t,
    mut input: *const ::core::ffi::c_void,
    mut len: size_t,
) -> XXH_errorcode {
    return XXH3_update_regular(state, input, len);
}
unsafe extern "C" fn XXH3_digest_long(
    mut acc: *mut XXH64_hash_t,
    mut state: *const XXH3_state_t,
    mut secret: *const ::core::ffi::c_uchar,
) {
    let mut lastStripe: [xxh_u8; 64] = [0; 64];
    let mut lastStripePtr: *const xxh_u8 = ::core::ptr::null::<xxh_u8>();
    memcpy(
        acc as *mut ::core::ffi::c_void,
        &raw const (*state).acc as *const XXH64_hash_t as *const ::core::ffi::c_void,
        ::core::mem::size_of::<[XXH64_hash_t; 8]>() as size_t,
    );
    if (*state).bufferedSize >= XXH_STRIPE_LEN as XXH32_hash_t {
        let nbStripes: size_t = (*state)
            .bufferedSize
            .wrapping_sub(1 as XXH32_hash_t)
            .wrapping_div(XXH_STRIPE_LEN as XXH32_hash_t) as size_t;
        let mut nbStripesSoFar: size_t = (*state).nbStripesSoFar;
        XXH3_consumeStripes(
            acc as *mut xxh_u64,
            &raw mut nbStripesSoFar,
            (*state).nbStripesPerBlock,
            &raw const (*state).buffer as *const xxh_u8,
            nbStripes,
            secret as *const xxh_u8,
            (*state).secretLimit,
            Some(
                XXH3_accumulate_scalar
                    as unsafe extern "C" fn(
                        *mut xxh_u64,
                        *const xxh_u8,
                        *const xxh_u8,
                        size_t,
                    ) -> (),
            ),
            Some(
                XXH3_scrambleAcc_scalar
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const ::core::ffi::c_void,
                    ) -> (),
            ),
        );
        lastStripePtr = (&raw const (*state).buffer as *const ::core::ffi::c_uchar)
            .offset((*state).bufferedSize as isize)
            .offset(-(XXH_STRIPE_LEN as isize)) as *const xxh_u8;
    } else {
        let catchupSize: size_t =
            (XXH_STRIPE_LEN as XXH32_hash_t).wrapping_sub((*state).bufferedSize) as size_t;
        memcpy(
            &raw mut lastStripe as *mut xxh_u8 as *mut ::core::ffi::c_void,
            (&raw const (*state).buffer as *const ::core::ffi::c_uchar)
                .offset(::core::mem::size_of::<[::core::ffi::c_uchar; 256]>() as usize as isize)
                .offset(-(catchupSize as isize)) as *const ::core::ffi::c_void,
            catchupSize,
        );
        memcpy(
            (&raw mut lastStripe as *mut xxh_u8).offset(catchupSize as isize)
                as *mut ::core::ffi::c_void,
            &raw const (*state).buffer as *const ::core::ffi::c_uchar as *const ::core::ffi::c_void,
            (*state).bufferedSize as size_t,
        );
        lastStripePtr = &raw mut lastStripe as *mut xxh_u8;
    }
    XXH3_accumulate_512_scalar(
        acc as *mut ::core::ffi::c_void,
        lastStripePtr as *const ::core::ffi::c_void,
        secret
            .offset((*state).secretLimit as isize)
            .offset(-(XXH_SECRET_LASTACC_START as isize)) as *const ::core::ffi::c_void,
    );
}
#[no_mangle]
pub unsafe extern "C" fn XXH3_64bits_digest(mut state: *const XXH3_state_t) -> XXH64_hash_t {
    let secret: *const ::core::ffi::c_uchar = if (*state).extSecret.is_null() {
        &raw const (*state).customSecret as *const ::core::ffi::c_uchar
    } else {
        (*state).extSecret
    };
    if (*state).totalLen > XXH3_MIDSIZE_MAX as XXH64_hash_t {
        let mut acc: [XXH64_hash_t; 8] = [0; 8];
        XXH3_digest_long(&raw mut acc as *mut XXH64_hash_t, state, secret);
        return XXH3_finalizeLong_64b(
            &raw mut acc as *mut XXH64_hash_t,
            secret as *const xxh_u8,
            (*state).totalLen,
        );
    }
    if (*state).useSeed != 0 {
        return XXH3_64bits_withSeed(
            &raw const (*state).buffer as *const ::core::ffi::c_uchar as *const ::core::ffi::c_void,
            (*state).totalLen as size_t,
            (*state).seed,
        );
    }
    return XXH3_64bits_withSecret(
        &raw const (*state).buffer as *const ::core::ffi::c_uchar as *const ::core::ffi::c_void,
        (*state).totalLen as size_t,
        secret as *const ::core::ffi::c_void,
        (*state).secretLimit.wrapping_add(XXH_STRIPE_LEN as size_t),
    );
}
unsafe extern "C" fn XXH3_len_1to3_128b(
    mut input: *const xxh_u8,
    mut len: size_t,
    mut secret: *const xxh_u8,
    mut seed: XXH64_hash_t,
) -> XXH128_hash_t {
    let c1: xxh_u8 = *input.offset(0 as ::core::ffi::c_int as isize);
    let c2: xxh_u8 = *input.offset((len >> 1 as ::core::ffi::c_int) as isize);
    let c3: xxh_u8 = *input.offset(len.wrapping_sub(1 as size_t) as isize);
    let combinedl: xxh_u32 = (c1 as xxh_u32) << 16 as ::core::ffi::c_int
        | (c2 as xxh_u32) << 24 as ::core::ffi::c_int
        | (c3 as xxh_u32) << 0 as ::core::ffi::c_int
        | (len as xxh_u32) << 8 as ::core::ffi::c_int;
    let combinedh: xxh_u32 =
        XXH_swap32(combinedl).rotate_left(13 as ::core::ffi::c_int as ::core::ffi::c_uint as u32);
    let bitflipl: xxh_u64 = ((XXH_readLE32(secret as *const ::core::ffi::c_void)
        ^ XXH_readLE32(
            secret.offset(4 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_void
        )) as xxh_u64)
        .wrapping_add(seed as xxh_u64);
    let bitfliph: xxh_u64 = ((XXH_readLE32(
        secret.offset(8 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_void
    ) ^ XXH_readLE32(
        secret.offset(12 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_void
    )) as xxh_u64)
        .wrapping_sub(seed as xxh_u64);
    let keyed_lo: xxh_u64 = combinedl as xxh_u64 ^ bitflipl;
    let keyed_hi: xxh_u64 = combinedh as xxh_u64 ^ bitfliph;
    let mut h128: XXH128_hash_t = XXH128_hash_t {
        low64: 0,
        high64: 0,
    };
    h128.low64 = XXH64_avalanche(keyed_lo) as XXH64_hash_t;
    h128.high64 = XXH64_avalanche(keyed_hi) as XXH64_hash_t;
    return h128;
}
unsafe extern "C" fn XXH3_len_4to8_128b(
    mut input: *const xxh_u8,
    mut len: size_t,
    mut secret: *const xxh_u8,
    mut seed: XXH64_hash_t,
) -> XXH128_hash_t {
    seed ^= ((XXH_swap32(seed as xxh_u32) as xxh_u64) << 32 as ::core::ffi::c_int) as XXH64_hash_t;
    let input_lo: xxh_u32 = XXH_readLE32(input as *const ::core::ffi::c_void) as xxh_u32;
    let input_hi: xxh_u32 = XXH_readLE32(
        input
            .offset(len as isize)
            .offset(-(4 as ::core::ffi::c_int as isize)) as *const ::core::ffi::c_void,
    ) as xxh_u32;
    let input_64: xxh_u64 =
        (input_lo as xxh_u64).wrapping_add((input_hi as xxh_u64) << 32 as ::core::ffi::c_int);
    let bitflip: xxh_u64 = (XXH_readLE64(
        secret.offset(16 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_void
    ) as xxh_u64
        ^ XXH_readLE64(
            secret.offset(24 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_void
        ) as xxh_u64)
        .wrapping_add(seed as xxh_u64);
    let keyed: xxh_u64 = input_64 ^ bitflip;
    let mut m128: XXH128_hash_t = XXH_mult64to128(
        keyed,
        XXH_PRIME64_1.wrapping_add((len << 2 as ::core::ffi::c_int) as ::core::ffi::c_ulonglong)
            as xxh_u64,
    );
    m128.high64 = m128
        .high64
        .wrapping_add(m128.low64 << 1 as ::core::ffi::c_int);
    m128.low64 ^= m128.high64 >> 3 as ::core::ffi::c_int;
    m128.low64 = XXH_xorshift64(m128.low64 as xxh_u64, 35 as ::core::ffi::c_int) as XXH64_hash_t;
    m128.low64 = m128.low64.wrapping_mul(PRIME_MX2 as XXH64_hash_t);
    m128.low64 = XXH_xorshift64(m128.low64 as xxh_u64, 28 as ::core::ffi::c_int) as XXH64_hash_t;
    m128.high64 = XXH3_avalanche(m128.high64 as xxh_u64);
    return m128;
}
unsafe extern "C" fn XXH3_len_9to16_128b(
    mut input: *const xxh_u8,
    mut len: size_t,
    mut secret: *const xxh_u8,
    mut seed: XXH64_hash_t,
) -> XXH128_hash_t {
    let bitflipl: xxh_u64 = (XXH_readLE64(
        secret.offset(32 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_void
    ) as xxh_u64
        ^ XXH_readLE64(
            secret.offset(40 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_void
        ) as xxh_u64)
        .wrapping_sub(seed as xxh_u64);
    let bitfliph: xxh_u64 = (XXH_readLE64(
        secret.offset(48 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_void
    ) as xxh_u64
        ^ XXH_readLE64(
            secret.offset(56 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_void
        ) as xxh_u64)
        .wrapping_add(seed as xxh_u64);
    let input_lo: xxh_u64 = XXH_readLE64(input as *const ::core::ffi::c_void) as xxh_u64;
    let mut input_hi: xxh_u64 = XXH_readLE64(
        input
            .offset(len as isize)
            .offset(-(8 as ::core::ffi::c_int as isize)) as *const ::core::ffi::c_void,
    );
    let mut m128: XXH128_hash_t =
        XXH_mult64to128(input_lo ^ input_hi ^ bitflipl, XXH_PRIME64_1 as xxh_u64);
    m128.low64 = m128.low64.wrapping_add(
        ((len.wrapping_sub(1 as size_t) as xxh_u64) << 54 as ::core::ffi::c_int) as XXH64_hash_t,
    );
    input_hi ^= bitfliph;
    if (::core::mem::size_of::<*mut ::core::ffi::c_void>() as usize)
        < ::core::mem::size_of::<xxh_u64>() as usize
    {
        m128.high64 = (m128.high64 as ::core::ffi::c_ulonglong).wrapping_add(
            (input_hi as ::core::ffi::c_ulonglong & 0xffffffff00000000 as ::core::ffi::c_ulonglong)
                .wrapping_add(
                    (input_hi as xxh_u32 as xxh_u64)
                        .wrapping_mul(0x85ebca77 as ::core::ffi::c_uint as xxh_u32 as xxh_u64)
                        as ::core::ffi::c_ulonglong,
                ),
        ) as XXH64_hash_t as XXH64_hash_t;
    } else {
        m128.high64 = m128.high64.wrapping_add(input_hi.wrapping_add(
            (input_hi as xxh_u32 as xxh_u64).wrapping_mul(
                (0x85ebca77 as ::core::ffi::c_uint).wrapping_sub(1 as ::core::ffi::c_uint)
                    as xxh_u32 as xxh_u64,
            ),
        ) as XXH64_hash_t);
    }
    m128.low64 ^= XXH_swap64(m128.high64 as xxh_u64) as XXH64_hash_t;
    let mut h128: XXH128_hash_t = XXH_mult64to128(m128.low64 as xxh_u64, XXH_PRIME64_2 as xxh_u64);
    h128.high64 = (h128.high64 as ::core::ffi::c_ulonglong)
        .wrapping_add((m128.high64 as ::core::ffi::c_ulonglong).wrapping_mul(XXH_PRIME64_2))
        as XXH64_hash_t as XXH64_hash_t;
    h128.low64 = XXH3_avalanche(h128.low64 as xxh_u64);
    h128.high64 = XXH3_avalanche(h128.high64 as xxh_u64);
    return h128;
}
unsafe extern "C" fn XXH3_len_0to16_128b(
    mut input: *const xxh_u8,
    mut len: size_t,
    mut secret: *const xxh_u8,
    mut seed: XXH64_hash_t,
) -> XXH128_hash_t {
    if len > 8 as size_t {
        return XXH3_len_9to16_128b(input, len, secret, seed);
    }
    if len >= 4 as size_t {
        return XXH3_len_4to8_128b(input, len, secret, seed);
    }
    if len != 0 {
        return XXH3_len_1to3_128b(input, len, secret, seed);
    }
    let mut h128: XXH128_hash_t = XXH128_hash_t {
        low64: 0,
        high64: 0,
    };
    let bitflipl: xxh_u64 = XXH_readLE64(
        secret.offset(64 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_void
    ) as xxh_u64
        ^ XXH_readLE64(
            secret.offset(72 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_void
        ) as xxh_u64;
    let bitfliph: xxh_u64 = XXH_readLE64(
        secret.offset(80 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_void
    ) as xxh_u64
        ^ XXH_readLE64(
            secret.offset(88 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_void
        ) as xxh_u64;
    h128.low64 = XXH64_avalanche(seed as xxh_u64 ^ bitflipl) as XXH64_hash_t;
    h128.high64 = XXH64_avalanche(seed as xxh_u64 ^ bitfliph) as XXH64_hash_t;
    return h128;
}
unsafe extern "C" fn XXH128_mix32B(
    mut acc: XXH128_hash_t,
    mut input_1: *const xxh_u8,
    mut input_2: *const xxh_u8,
    mut secret: *const xxh_u8,
    mut seed: XXH64_hash_t,
) -> XXH128_hash_t {
    acc.low64 = acc.low64.wrapping_add(XXH3_mix16B(
        input_1,
        secret.offset(0 as ::core::ffi::c_int as isize),
        seed as xxh_u64,
    ) as XXH64_hash_t);
    acc.low64 ^= XXH_readLE64(input_2 as *const ::core::ffi::c_void).wrapping_add(XXH_readLE64(
        input_2.offset(8 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_void,
    )) as XXH64_hash_t;
    acc.high64 = acc.high64.wrapping_add(XXH3_mix16B(
        input_2,
        secret.offset(16 as ::core::ffi::c_int as isize),
        seed as xxh_u64,
    ) as XXH64_hash_t);
    acc.high64 ^= XXH_readLE64(input_1 as *const ::core::ffi::c_void).wrapping_add(XXH_readLE64(
        input_1.offset(8 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_void,
    )) as XXH64_hash_t;
    return acc;
}
unsafe extern "C" fn XXH3_len_17to128_128b(
    mut input: *const xxh_u8,
    mut len: size_t,
    mut secret: *const xxh_u8,
    mut secretSize: size_t,
    mut seed: XXH64_hash_t,
) -> XXH128_hash_t {
    let mut acc: XXH128_hash_t = XXH128_hash_t {
        low64: 0,
        high64: 0,
    };
    acc.low64 = (len as ::core::ffi::c_ulonglong).wrapping_mul(XXH_PRIME64_1) as XXH64_hash_t;
    acc.high64 = 0 as XXH64_hash_t;
    if len > 32 as size_t {
        if len > 64 as size_t {
            if len > 96 as size_t {
                acc = XXH128_mix32B(
                    acc,
                    input.offset(48 as ::core::ffi::c_int as isize),
                    input
                        .offset(len as isize)
                        .offset(-(64 as ::core::ffi::c_int as isize)),
                    secret.offset(96 as ::core::ffi::c_int as isize),
                    seed,
                );
            }
            acc = XXH128_mix32B(
                acc,
                input.offset(32 as ::core::ffi::c_int as isize),
                input
                    .offset(len as isize)
                    .offset(-(48 as ::core::ffi::c_int as isize)),
                secret.offset(64 as ::core::ffi::c_int as isize),
                seed,
            );
        }
        acc = XXH128_mix32B(
            acc,
            input.offset(16 as ::core::ffi::c_int as isize),
            input
                .offset(len as isize)
                .offset(-(32 as ::core::ffi::c_int as isize)),
            secret.offset(32 as ::core::ffi::c_int as isize),
            seed,
        );
    }
    acc = XXH128_mix32B(
        acc,
        input,
        input
            .offset(len as isize)
            .offset(-(16 as ::core::ffi::c_int as isize)),
        secret,
        seed,
    );
    let mut h128: XXH128_hash_t = XXH128_hash_t {
        low64: 0,
        high64: 0,
    };
    h128.low64 = acc.low64.wrapping_add(acc.high64);
    h128.high64 = (acc.low64 as ::core::ffi::c_ulonglong)
        .wrapping_mul(XXH_PRIME64_1)
        .wrapping_add((acc.high64 as ::core::ffi::c_ulonglong).wrapping_mul(XXH_PRIME64_4))
        .wrapping_add(
            (len.wrapping_sub(seed as size_t) as ::core::ffi::c_ulonglong)
                .wrapping_mul(XXH_PRIME64_2),
        ) as XXH64_hash_t;
    h128.low64 = XXH3_avalanche(h128.low64 as xxh_u64);
    h128.high64 = (0 as ::core::ffi::c_int as XXH64_hash_t)
        .wrapping_sub(XXH3_avalanche(h128.high64 as xxh_u64));
    return h128;
}
unsafe extern "C" fn XXH3_len_129to240_128b(
    mut input: *const xxh_u8,
    mut len: size_t,
    mut secret: *const xxh_u8,
    mut secretSize: size_t,
    mut seed: XXH64_hash_t,
) -> XXH128_hash_t {
    let mut acc: XXH128_hash_t = XXH128_hash_t {
        low64: 0,
        high64: 0,
    };
    let mut i: ::core::ffi::c_uint = 0;
    acc.low64 = (len as ::core::ffi::c_ulonglong).wrapping_mul(XXH_PRIME64_1) as XXH64_hash_t;
    acc.high64 = 0 as XXH64_hash_t;
    i = 32 as ::core::ffi::c_uint;
    while i < 160 as ::core::ffi::c_uint {
        acc = XXH128_mix32B(
            acc,
            input
                .offset(i as isize)
                .offset(-(32 as ::core::ffi::c_int as isize)),
            input
                .offset(i as isize)
                .offset(-(16 as ::core::ffi::c_int as isize)),
            secret
                .offset(i as isize)
                .offset(-(32 as ::core::ffi::c_int as isize)),
            seed,
        );
        i = i.wrapping_add(32 as ::core::ffi::c_uint);
    }
    acc.low64 = XXH3_avalanche(acc.low64 as xxh_u64);
    acc.high64 = XXH3_avalanche(acc.high64 as xxh_u64);
    i = 160 as ::core::ffi::c_uint;
    while i as size_t <= len {
        acc = XXH128_mix32B(
            acc,
            input
                .offset(i as isize)
                .offset(-(32 as ::core::ffi::c_int as isize)),
            input
                .offset(i as isize)
                .offset(-(16 as ::core::ffi::c_int as isize)),
            secret
                .offset(XXH3_MIDSIZE_STARTOFFSET as isize)
                .offset(i as isize)
                .offset(-(160 as ::core::ffi::c_int as isize)),
            seed,
        );
        i = i.wrapping_add(32 as ::core::ffi::c_uint);
    }
    acc = XXH128_mix32B(
        acc,
        input
            .offset(len as isize)
            .offset(-(16 as ::core::ffi::c_int as isize)),
        input
            .offset(len as isize)
            .offset(-(32 as ::core::ffi::c_int as isize)),
        secret
            .offset(XXH3_SECRET_SIZE_MIN as isize)
            .offset(-(XXH3_MIDSIZE_LASTOFFSET as isize))
            .offset(-(16 as ::core::ffi::c_int as isize)),
        (0 as ::core::ffi::c_int as XXH64_hash_t).wrapping_sub(seed),
    );
    let mut h128: XXH128_hash_t = XXH128_hash_t {
        low64: 0,
        high64: 0,
    };
    h128.low64 = acc.low64.wrapping_add(acc.high64);
    h128.high64 = (acc.low64 as ::core::ffi::c_ulonglong)
        .wrapping_mul(XXH_PRIME64_1)
        .wrapping_add((acc.high64 as ::core::ffi::c_ulonglong).wrapping_mul(XXH_PRIME64_4))
        .wrapping_add(
            (len.wrapping_sub(seed as size_t) as ::core::ffi::c_ulonglong)
                .wrapping_mul(XXH_PRIME64_2),
        ) as XXH64_hash_t;
    h128.low64 = XXH3_avalanche(h128.low64 as xxh_u64);
    h128.high64 = (0 as ::core::ffi::c_int as XXH64_hash_t)
        .wrapping_sub(XXH3_avalanche(h128.high64 as xxh_u64));
    return h128;
}
unsafe extern "C" fn XXH3_finalizeLong_128b(
    mut acc: *const xxh_u64,
    mut secret: *const xxh_u8,
    mut secretSize: size_t,
    mut len: xxh_u64,
) -> XXH128_hash_t {
    let mut h128: XXH128_hash_t = XXH128_hash_t {
        low64: 0,
        high64: 0,
    };
    h128.low64 = XXH3_finalizeLong_64b(acc, secret, len);
    h128.high64 = XXH3_mergeAccs(
        acc,
        secret
            .offset(secretSize as isize)
            .offset(-(XXH_STRIPE_LEN as isize))
            .offset(-(XXH_SECRET_MERGEACCS_START as isize)),
        !(len as ::core::ffi::c_ulonglong).wrapping_mul(XXH_PRIME64_2) as xxh_u64,
    );
    return h128;
}
unsafe extern "C" fn XXH3_hashLong_128b_internal(
    mut input: *const ::core::ffi::c_void,
    mut len: size_t,
    mut secret: *const xxh_u8,
    mut secretSize: size_t,
    mut f_acc: XXH3_f_accumulate,
    mut f_scramble: XXH3_f_scrambleAcc,
) -> XXH128_hash_t {
    let mut acc: [xxh_u64; 8] = [
        XXH_PRIME32_3 as xxh_u64,
        XXH_PRIME64_1 as xxh_u64,
        XXH_PRIME64_2 as xxh_u64,
        XXH_PRIME64_3 as xxh_u64,
        XXH_PRIME64_4 as xxh_u64,
        XXH_PRIME32_2 as xxh_u64,
        XXH_PRIME64_5 as xxh_u64,
        XXH_PRIME32_1 as xxh_u64,
    ];
    XXH3_hashLong_internal_loop(
        &raw mut acc as *mut xxh_u64,
        input as *const xxh_u8,
        len,
        secret,
        secretSize,
        f_acc,
        f_scramble,
    );
    return XXH3_finalizeLong_128b(
        &raw mut acc as *mut xxh_u64,
        secret,
        secretSize,
        len as xxh_u64,
    );
}
unsafe extern "C" fn XXH3_hashLong_128b_default(
    mut input: *const ::core::ffi::c_void,
    mut len: size_t,
    mut seed64: XXH64_hash_t,
    mut secret: *const ::core::ffi::c_void,
    mut secretLen: size_t,
) -> XXH128_hash_t {
    return XXH3_hashLong_128b_internal(
        input,
        len,
        &raw const XXH3_kSecret as *const xxh_u8,
        ::core::mem::size_of::<[xxh_u8; 192]>() as size_t,
        Some(
            XXH3_accumulate_scalar
                as unsafe extern "C" fn(*mut xxh_u64, *const xxh_u8, *const xxh_u8, size_t) -> (),
        ),
        Some(
            XXH3_scrambleAcc_scalar
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, *const ::core::ffi::c_void) -> (),
        ),
    );
}
unsafe extern "C" fn XXH3_hashLong_128b_withSecret(
    mut input: *const ::core::ffi::c_void,
    mut len: size_t,
    mut seed64: XXH64_hash_t,
    mut secret: *const ::core::ffi::c_void,
    mut secretLen: size_t,
) -> XXH128_hash_t {
    return XXH3_hashLong_128b_internal(
        input,
        len,
        secret as *const xxh_u8,
        secretLen,
        Some(
            XXH3_accumulate_scalar
                as unsafe extern "C" fn(*mut xxh_u64, *const xxh_u8, *const xxh_u8, size_t) -> (),
        ),
        Some(
            XXH3_scrambleAcc_scalar
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, *const ::core::ffi::c_void) -> (),
        ),
    );
}
unsafe extern "C" fn XXH3_hashLong_128b_withSeed_internal(
    mut input: *const ::core::ffi::c_void,
    mut len: size_t,
    mut seed64: XXH64_hash_t,
    mut f_acc: XXH3_f_accumulate,
    mut f_scramble: XXH3_f_scrambleAcc,
    mut f_initSec: XXH3_f_initCustomSecret,
) -> XXH128_hash_t {
    if seed64 == 0 as XXH64_hash_t {
        return XXH3_hashLong_128b_internal(
            input,
            len,
            &raw const XXH3_kSecret as *const xxh_u8,
            ::core::mem::size_of::<[xxh_u8; 192]>() as size_t,
            f_acc,
            f_scramble,
        );
    }
    let mut secret: [xxh_u8; 192] = [0; 192];
    f_initSec.expect("non-null function pointer")(
        &raw mut secret as *mut xxh_u8 as *mut ::core::ffi::c_void,
        seed64 as xxh_u64,
    );
    return XXH3_hashLong_128b_internal(
        input,
        len,
        &raw mut secret as *mut xxh_u8 as *const xxh_u8,
        ::core::mem::size_of::<[xxh_u8; 192]>() as size_t,
        f_acc,
        f_scramble,
    );
}
unsafe extern "C" fn XXH3_hashLong_128b_withSeed(
    mut input: *const ::core::ffi::c_void,
    mut len: size_t,
    mut seed64: XXH64_hash_t,
    mut secret: *const ::core::ffi::c_void,
    mut secretLen: size_t,
) -> XXH128_hash_t {
    return XXH3_hashLong_128b_withSeed_internal(
        input,
        len,
        seed64,
        Some(
            XXH3_accumulate_scalar
                as unsafe extern "C" fn(*mut xxh_u64, *const xxh_u8, *const xxh_u8, size_t) -> (),
        ),
        Some(
            XXH3_scrambleAcc_scalar
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, *const ::core::ffi::c_void) -> (),
        ),
        Some(
            XXH3_initCustomSecret_scalar
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, xxh_u64) -> (),
        ),
    );
}
unsafe extern "C" fn XXH3_128bits_internal(
    mut input: *const ::core::ffi::c_void,
    mut len: size_t,
    mut seed64: XXH64_hash_t,
    mut secret: *const ::core::ffi::c_void,
    mut secretLen: size_t,
    mut f_hl128: XXH3_hashLong128_f,
) -> XXH128_hash_t {
    if len <= 16 as size_t {
        return XXH3_len_0to16_128b(input as *const xxh_u8, len, secret as *const xxh_u8, seed64);
    }
    if len <= 128 as size_t {
        return XXH3_len_17to128_128b(
            input as *const xxh_u8,
            len,
            secret as *const xxh_u8,
            secretLen,
            seed64,
        );
    }
    if len <= XXH3_MIDSIZE_MAX as size_t {
        return XXH3_len_129to240_128b(
            input as *const xxh_u8,
            len,
            secret as *const xxh_u8,
            secretLen,
            seed64,
        );
    }
    return f_hl128.expect("non-null function pointer")(input, len, seed64, secret, secretLen);
}
#[no_mangle]
pub unsafe extern "C" fn XXH3_128bits(
    mut input: *const ::core::ffi::c_void,
    mut len: size_t,
) -> XXH128_hash_t {
    return XXH3_128bits_internal(
        input,
        len,
        0 as XXH64_hash_t,
        &raw const XXH3_kSecret as *const xxh_u8 as *const ::core::ffi::c_void,
        ::core::mem::size_of::<[xxh_u8; 192]>() as size_t,
        Some(
            XXH3_hashLong_128b_default
                as unsafe extern "C" fn(
                    *const ::core::ffi::c_void,
                    size_t,
                    XXH64_hash_t,
                    *const ::core::ffi::c_void,
                    size_t,
                ) -> XXH128_hash_t,
        ),
    );
}
#[no_mangle]
pub unsafe extern "C" fn XXH3_128bits_withSecret(
    mut input: *const ::core::ffi::c_void,
    mut len: size_t,
    mut secret: *const ::core::ffi::c_void,
    mut secretSize: size_t,
) -> XXH128_hash_t {
    return XXH3_128bits_internal(
        input,
        len,
        0 as XXH64_hash_t,
        secret as *const xxh_u8 as *const ::core::ffi::c_void,
        secretSize,
        Some(
            XXH3_hashLong_128b_withSecret
                as unsafe extern "C" fn(
                    *const ::core::ffi::c_void,
                    size_t,
                    XXH64_hash_t,
                    *const ::core::ffi::c_void,
                    size_t,
                ) -> XXH128_hash_t,
        ),
    );
}
#[no_mangle]
pub unsafe extern "C" fn XXH3_128bits_withSeed(
    mut input: *const ::core::ffi::c_void,
    mut len: size_t,
    mut seed: XXH64_hash_t,
) -> XXH128_hash_t {
    return XXH3_128bits_internal(
        input,
        len,
        seed,
        &raw const XXH3_kSecret as *const xxh_u8 as *const ::core::ffi::c_void,
        ::core::mem::size_of::<[xxh_u8; 192]>() as size_t,
        Some(
            XXH3_hashLong_128b_withSeed
                as unsafe extern "C" fn(
                    *const ::core::ffi::c_void,
                    size_t,
                    XXH64_hash_t,
                    *const ::core::ffi::c_void,
                    size_t,
                ) -> XXH128_hash_t,
        ),
    );
}
#[no_mangle]
pub unsafe extern "C" fn XXH3_128bits_withSecretandSeed(
    mut input: *const ::core::ffi::c_void,
    mut len: size_t,
    mut secret: *const ::core::ffi::c_void,
    mut secretSize: size_t,
    mut seed: XXH64_hash_t,
) -> XXH128_hash_t {
    if len <= XXH3_MIDSIZE_MAX as size_t {
        return XXH3_128bits_internal(
            input,
            len,
            seed,
            &raw const XXH3_kSecret as *const xxh_u8 as *const ::core::ffi::c_void,
            ::core::mem::size_of::<[xxh_u8; 192]>() as size_t,
            None,
        );
    }
    return XXH3_hashLong_128b_withSecret(input, len, seed, secret, secretSize);
}
#[no_mangle]
pub unsafe extern "C" fn XXH128(
    mut input: *const ::core::ffi::c_void,
    mut len: size_t,
    mut seed: XXH64_hash_t,
) -> XXH128_hash_t {
    return XXH3_128bits_withSeed(input, len, seed);
}
#[no_mangle]
pub unsafe extern "C" fn XXH3_128bits_reset(mut statePtr: *mut XXH3_state_t) -> XXH_errorcode {
    return XXH3_64bits_reset(statePtr);
}
#[no_mangle]
pub unsafe extern "C" fn XXH3_128bits_reset_withSecret(
    mut statePtr: *mut XXH3_state_t,
    mut secret: *const ::core::ffi::c_void,
    mut secretSize: size_t,
) -> XXH_errorcode {
    return XXH3_64bits_reset_withSecret(statePtr, secret, secretSize);
}
#[no_mangle]
pub unsafe extern "C" fn XXH3_128bits_reset_withSeed(
    mut statePtr: *mut XXH3_state_t,
    mut seed: XXH64_hash_t,
) -> XXH_errorcode {
    return XXH3_64bits_reset_withSeed(statePtr, seed);
}
#[no_mangle]
pub unsafe extern "C" fn XXH3_128bits_reset_withSecretandSeed(
    mut statePtr: *mut XXH3_state_t,
    mut secret: *const ::core::ffi::c_void,
    mut secretSize: size_t,
    mut seed: XXH64_hash_t,
) -> XXH_errorcode {
    return XXH3_64bits_reset_withSecretandSeed(statePtr, secret, secretSize, seed);
}
#[no_mangle]
pub unsafe extern "C" fn XXH3_128bits_update(
    mut state: *mut XXH3_state_t,
    mut input: *const ::core::ffi::c_void,
    mut len: size_t,
) -> XXH_errorcode {
    return XXH3_update_regular(state, input, len);
}
#[no_mangle]
pub unsafe extern "C" fn XXH3_128bits_digest(mut state: *const XXH3_state_t) -> XXH128_hash_t {
    let secret: *const ::core::ffi::c_uchar = if (*state).extSecret.is_null() {
        &raw const (*state).customSecret as *const ::core::ffi::c_uchar
    } else {
        (*state).extSecret
    };
    if (*state).totalLen > XXH3_MIDSIZE_MAX as XXH64_hash_t {
        let mut acc: [XXH64_hash_t; 8] = [0; 8];
        XXH3_digest_long(&raw mut acc as *mut XXH64_hash_t, state, secret);
        return XXH3_finalizeLong_128b(
            &raw mut acc as *mut XXH64_hash_t,
            secret as *const xxh_u8,
            (*state).secretLimit.wrapping_add(XXH_STRIPE_LEN as size_t),
            (*state).totalLen,
        );
    }
    if (*state).useSeed != 0 {
        return XXH3_128bits_withSeed(
            &raw const (*state).buffer as *const ::core::ffi::c_uchar as *const ::core::ffi::c_void,
            (*state).totalLen as size_t,
            (*state).seed,
        );
    }
    return XXH3_128bits_withSecret(
        &raw const (*state).buffer as *const ::core::ffi::c_uchar as *const ::core::ffi::c_void,
        (*state).totalLen as size_t,
        secret as *const ::core::ffi::c_void,
        (*state).secretLimit.wrapping_add(XXH_STRIPE_LEN as size_t),
    );
}
#[no_mangle]
pub unsafe extern "C" fn XXH128_isEqual(
    mut h1: XXH128_hash_t,
    mut h2: XXH128_hash_t,
) -> ::core::ffi::c_int {
    return (memcmp(
        &raw mut h1 as *const ::core::ffi::c_void,
        &raw mut h2 as *const ::core::ffi::c_void,
        ::core::mem::size_of::<XXH128_hash_t>() as size_t,
    ) == 0) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn XXH128_cmp(
    mut h128_1: *const ::core::ffi::c_void,
    mut h128_2: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let h1: XXH128_hash_t = *(h128_1 as *const XXH128_hash_t);
    let h2: XXH128_hash_t = *(h128_2 as *const XXH128_hash_t);
    let hcmp: ::core::ffi::c_int = (h1.high64 > h2.high64) as ::core::ffi::c_int
        - (h2.high64 > h1.high64) as ::core::ffi::c_int;
    if hcmp != 0 {
        return hcmp;
    }
    return (h1.low64 > h2.low64) as ::core::ffi::c_int
        - (h2.low64 > h1.low64) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn XXH128_canonicalFromHash(
    mut dst: *mut XXH128_canonical_t,
    mut hash: XXH128_hash_t,
) {
    hash.high64 = XXH_swap64(hash.high64 as xxh_u64) as XXH64_hash_t;
    hash.low64 = XXH_swap64(hash.low64 as xxh_u64) as XXH64_hash_t;
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw mut hash.high64 as *const ::core::ffi::c_void,
        ::core::mem::size_of::<XXH64_hash_t>() as size_t,
    );
    memcpy(
        (dst as *mut ::core::ffi::c_char)
            .offset(::core::mem::size_of::<XXH64_hash_t>() as usize as isize)
            as *mut ::core::ffi::c_void,
        &raw mut hash.low64 as *const ::core::ffi::c_void,
        ::core::mem::size_of::<XXH64_hash_t>() as size_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn XXH128_hashFromCanonical(
    mut src: *const XXH128_canonical_t,
) -> XXH128_hash_t {
    let mut h: XXH128_hash_t = XXH128_hash_t {
        low64: 0,
        high64: 0,
    };
    h.high64 = XXH_readBE64(src as *const ::core::ffi::c_void) as XXH64_hash_t;
    h.low64 = XXH_readBE64(
        (&raw const (*src).digest as *const ::core::ffi::c_uchar)
            .offset(8 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_void,
    ) as XXH64_hash_t;
    return h;
}
unsafe extern "C" fn XXH3_combine16(mut dst: *mut ::core::ffi::c_void, mut h128: XXH128_hash_t) {
    XXH_writeLE64(dst, XXH_readLE64(dst) ^ h128.low64 as xxh_u64);
    XXH_writeLE64(
        (dst as *mut ::core::ffi::c_char).offset(8 as ::core::ffi::c_int as isize)
            as *mut ::core::ffi::c_void,
        XXH_readLE64(
            (dst as *mut ::core::ffi::c_char).offset(8 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_void,
        ) ^ h128.high64 as xxh_u64,
    );
}
#[no_mangle]
pub unsafe extern "C" fn XXH3_generateSecret(
    mut secretBuffer: *mut ::core::ffi::c_void,
    mut secretSize: size_t,
    mut customSeed: *const ::core::ffi::c_void,
    mut customSeedSize: size_t,
) -> XXH_errorcode {
    if secretBuffer.is_null() {
        return XXH_ERROR;
    }
    if secretSize < XXH3_SECRET_SIZE_MIN as size_t {
        return XXH_ERROR;
    }
    if customSeedSize == 0 as size_t {
        customSeed = &raw const XXH3_kSecret as *const xxh_u8 as *const ::core::ffi::c_void;
        customSeedSize = XXH_SECRET_DEFAULT_SIZE as size_t;
    }
    if customSeed.is_null() {
        return XXH_ERROR;
    }
    let mut pos: size_t = 0 as size_t;
    while pos < secretSize {
        let toCopy: size_t = if secretSize.wrapping_sub(pos) > customSeedSize {
            customSeedSize
        } else {
            secretSize.wrapping_sub(pos)
        };
        memcpy(
            (secretBuffer as *mut ::core::ffi::c_char).offset(pos as isize)
                as *mut ::core::ffi::c_void,
            customSeed,
            toCopy,
        );
        pos = pos.wrapping_add(toCopy);
    }
    let nbSeg16: size_t = secretSize.wrapping_div(16 as size_t);
    let mut n: size_t = 0;
    let mut scrambler: XXH128_canonical_t = XXH128_canonical_t { digest: [0; 16] };
    XXH128_canonicalFromHash(
        &raw mut scrambler,
        XXH128(customSeed, customSeedSize, 0 as XXH64_hash_t),
    );
    n = 0 as size_t;
    while n < nbSeg16 {
        let h128: XXH128_hash_t = XXH128(
            &raw mut scrambler as *const ::core::ffi::c_void,
            ::core::mem::size_of::<XXH128_canonical_t>() as size_t,
            n as XXH64_hash_t,
        ) as XXH128_hash_t;
        XXH3_combine16(
            (secretBuffer as *mut ::core::ffi::c_char).offset(n.wrapping_mul(16 as size_t) as isize)
                as *mut ::core::ffi::c_void,
            h128,
        );
        n = n.wrapping_add(1);
    }
    XXH3_combine16(
        (secretBuffer as *mut ::core::ffi::c_char)
            .offset(secretSize as isize)
            .offset(-(16 as ::core::ffi::c_int as isize)) as *mut ::core::ffi::c_void,
        XXH128_hashFromCanonical(&raw mut scrambler),
    );
    return XXH_OK;
}
#[no_mangle]
pub unsafe extern "C" fn XXH3_generateSecret_fromSeed(
    mut secretBuffer: *mut ::core::ffi::c_void,
    mut seed: XXH64_hash_t,
) {
    let mut secret: [xxh_u8; 192] = [0; 192];
    XXH3_initCustomSecret_scalar(
        &raw mut secret as *mut xxh_u8 as *mut ::core::ffi::c_void,
        seed as xxh_u64,
    );
    memcpy(
        secretBuffer,
        &raw mut secret as *mut xxh_u8 as *const ::core::ffi::c_void,
        XXH_SECRET_DEFAULT_SIZE as size_t,
    );
}
