use core::ffi::*;
pub use crate::src::c_inlined_fns::*;
pub use crate::src::ffi::*;
pub use crate::src::c_consts::*;
pub use crate::src::c_structs::*;
pub use crate::src::c_types::*;
use ::libc;
use crate::src::c_inlined_fns::LZ4_isAligned;
use crate::src::c_inlined_fns::LZ4_read16;
use crate::src::c_inlined_fns::LZ4_read32;
use crate::src::c_inlined_fns::LZ4_read_ARCH;
use crate::src::c_inlined_fns::LZ4_write16;

pub type dictIssue_directive = c_uint;
pub const dictSmall: dictIssue_directive = 1;
pub const noDictIssue: dictIssue_directive = 0;
pub type dict_directive = c_uint;
pub const usingDictCtx: dict_directive = 3;
pub const usingExtDict: dict_directive = 2;
pub const withPrefix64k: dict_directive = 1;
pub const noDict: dict_directive = 0;
pub type tableType_t = c_uint;
pub const byU16: tableType_t = 3;
pub const byU32: tableType_t = 2;
pub const byPtr: tableType_t = 1;
pub const clearedTable: tableType_t = 0;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct t_a {
    pub c: c_char,
    pub t: LZ4_stream_t,
}
pub type BYTE = c_uchar;

pub use crate::src::lz4hc::C2RustUnnamed_hu47194827;


pub type earlyEnd_directive = c_uint;
pub const partial_decode: earlyEnd_directive = 1;
pub const decode_full_block: earlyEnd_directive = 0;
pub type Rvl_t = size_t;

pub const LZ4_static_assert_0: C2RustUnnamed_htdd24ee73 = 1;

pub const LZ4_static_assert_1: C2RustUnnamed_htdd24ee73 = 1;
pub type C2RustUnnamed_htdd24ee73 = c_uint;
pub const LZ4_static_assert_2: C2RustUnnamed_htdd24ee73 = 1;
pub type LoadDict_mode_e = c_uint;
pub const _ld_slow: LoadDict_mode_e = 1;
pub const _ld_fast: LoadDict_mode_e = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub union LZ4_streamDecode_u {
    pub minStateSize: [c_char; 32],
    pub internal_donotuse: LZ4_streamDecode_t_internal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LZ4_streamDecode_t_internal {
    pub externalDict: *const LZ4_byte,
    pub prefixEnd: *const LZ4_byte,
    pub extDictSize: size_t,
    pub prefixSize: size_t,
}
pub type LZ4_streamDecode_t = LZ4_streamDecode_u;
pub const LZ4_static_assert_3: C2RustUnnamed_htdd24ee73 = 1;
pub const LZ4_ACCELERATION_DEFAULT: c_int = 1 as c_int;
pub const LZ4_ACCELERATION_MAX: c_int = 65537 as c_int;

pub const LZ4_VERSION_MAJOR: c_int = 1 as c_int;
pub const LZ4_VERSION_MINOR: c_int = 10 as c_int;
pub const LZ4_VERSION_RELEASE: c_int = 0 as c_int;
pub const LZ4_VERSION_NUMBER: c_int =
    LZ4_VERSION_MAJOR * 100 as c_int * 100 as c_int
        + LZ4_VERSION_MINOR * 100 as c_int
        + LZ4_VERSION_RELEASE;
pub const LZ4_MEMORY_USAGE: c_int = LZ4_MEMORY_USAGE_DEFAULT;
pub const LZ4_MEMORY_USAGE_DEFAULT: c_int = 14 as c_int;

pub const LZ4_HASHLOG: c_int = LZ4_MEMORY_USAGE - 2 as c_int;
pub const LZ4_HASH_SIZE_U32: c_int = (1 as c_int) << LZ4_HASHLOG;

pub const WILDCOPYLENGTH: c_int = 8 as c_int;

pub const MATCH_SAFEGUARD_DISTANCE: c_int =
    2 as c_int * WILDCOPYLENGTH - MINMATCH;
pub const FASTLOOP_SAFE_DISTANCE: c_int = 64 as c_int;
static mut LZ4_minLength: c_int = MFLIMIT + 1 as c_int;
pub const LZ4_DISTANCE_ABSOLUTE_MAX: c_int = 65535 as c_int;

pub const ML_MASK: c_uint =
    ((1 as c_uint) << ML_BITS).wrapping_sub(1 as c_uint);

pub const RUN_MASK: c_uint =
    ((1 as c_uint) << RUN_BITS).wrapping_sub(1 as c_uint);

extern "C" fn LZ4_isLittleEndian() -> c_uint { unsafe {
    let one: C2RustUnnamed_hu47194827 = C2RustUnnamed_hu47194827 {
        u: 1 as c_int as U32,
    };
    return one.c[0 as c_int as usize] as c_uint;
} }

unsafe fn LZ4_write32(mut memPtr: *mut c_void, mut value: U32) {
    (*(memPtr as *mut LZ4_unalign32)).u32_0 = value;
}
unsafe fn LZ4_readLE16(mut memPtr: *const c_void) -> U16 {
    if LZ4_isLittleEndian() != 0 {
        return LZ4_read16(memPtr);
    } else {
        let mut p: *const BYTE = memPtr as *const BYTE;
        return (*p.offset(0 as c_int as isize) as U16 as c_int
            | (*p.offset(1 as c_int as isize) as c_int)
                << 8 as c_int) as U16;
    };
}
unsafe extern "C" fn LZ4_writeLE16(mut memPtr: *mut c_void, mut value: U16) {
    if LZ4_isLittleEndian() != 0 {
        LZ4_write16(memPtr, value);
    } else {
        let mut p: *mut BYTE = memPtr as *mut BYTE;
        *p.offset(0 as c_int as isize) = value as BYTE;
        *p.offset(1 as c_int as isize) =
            (value as c_int >> 8 as c_int) as BYTE;
    };
}
#[inline(always)]
unsafe extern "C" fn LZ4_wildCopy8(
    mut dstPtr: *mut c_void,
    mut srcPtr: *const c_void,
    mut dstEnd: *mut c_void,
) {
    let mut d: *mut BYTE = dstPtr as *mut BYTE;
    let mut s: *const BYTE = srcPtr as *const BYTE;
    let e: *mut BYTE = dstEnd as *mut BYTE;
    loop {
        ::libc::memcpy(
            d as *mut c_void,
            s as *const c_void,
            8 as c_int as c_ulong as ::libc::size_t,
        );
        d = d.offset(8 as c_int as isize);
        s = s.offset(8 as c_int as isize);
        if !(d < e) {
            break;
        }
    }
}
static mut inc32table: [c_uint; 8] = [
    0 as c_int as c_uint,
    1 as c_int as c_uint,
    2 as c_int as c_uint,
    1 as c_int as c_uint,
    0 as c_int as c_uint,
    4 as c_int as c_uint,
    4 as c_int as c_uint,
    4 as c_int as c_uint,
];
static mut dec64table: [c_int; 8] = [
    0 as c_int,
    0 as c_int,
    0 as c_int,
    -(1 as c_int),
    -(4 as c_int),
    1 as c_int,
    2 as c_int,
    3 as c_int,
];
#[inline(always)]
unsafe fn LZ4_memcpy_using_offset_base(
    mut dstPtr: *mut BYTE,
    mut srcPtr: *const BYTE,
    mut dstEnd: *mut BYTE,
    offset: size_t,
) {
    if offset < 8 as size_t {
        LZ4_write32(dstPtr as *mut c_void, 0 as U32);
        ::libc::memcpy(
            dstPtr as *mut c_void,
            srcPtr as *const c_void,
            2 as c_int as c_ulong as ::libc::size_t,
        );
        ::libc::memcpy(
            dstPtr.offset(2 as c_int as isize) as *mut c_void,
            srcPtr.offset(2 as c_int as isize) as *const c_void,
            2 as c_int as c_ulong as ::libc::size_t,
        );
        srcPtr = srcPtr.offset(inc32table[offset as usize] as isize);
        ::libc::memcpy(
            dstPtr.offset(4 as c_int as isize) as *mut c_void,
            srcPtr as *const c_void,
            4 as c_int as c_ulong as ::libc::size_t,
        );
        srcPtr = srcPtr.offset(-(dec64table[offset as usize] as isize));
        dstPtr = dstPtr.offset(8 as c_int as isize);
    } else {
        ::libc::memcpy(
            dstPtr as *mut c_void,
            srcPtr as *const c_void,
            8 as c_int as c_ulong as ::libc::size_t,
        );
        dstPtr = dstPtr.offset(8 as c_int as isize);
        srcPtr = srcPtr.offset(8 as c_int as isize);
    }
    LZ4_wildCopy8(
        dstPtr as *mut c_void,
        srcPtr as *const c_void,
        dstEnd as *mut c_void,
    );
}
#[inline(always)]
unsafe fn LZ4_wildCopy32(
    mut dstPtr: *mut c_void,
    mut srcPtr: *const c_void,
    mut dstEnd: *mut c_void,
) {
    let mut d: *mut BYTE = dstPtr as *mut BYTE;
    let mut s: *const BYTE = srcPtr as *const BYTE;
    let e: *mut BYTE = dstEnd as *mut BYTE;
    loop {
        ::libc::memcpy(
            d as *mut c_void,
            s as *const c_void,
            16 as c_int as c_ulong as ::libc::size_t,
        );
        ::libc::memcpy(
            d.offset(16 as c_int as isize) as *mut c_void,
            s.offset(16 as c_int as isize) as *const c_void,
            16 as c_int as c_ulong as ::libc::size_t,
        );
        d = d.offset(32 as c_int as isize);
        s = s.offset(32 as c_int as isize);
        if !(d < e) {
            break;
        }
    }
}
#[inline(always)]
unsafe fn LZ4_memcpy_using_offset(
    mut dstPtr: *mut BYTE,
    mut srcPtr: *const BYTE,
    mut dstEnd: *mut BYTE,
    offset: size_t,
) {
    let mut v: [BYTE; 8] = [0; 8];
    match offset {
        1 => {
            memset(
                &raw mut v as *mut BYTE as *mut c_void,
                *srcPtr as c_int,
                8 as size_t,
            );
        }
        2 => {
            ::libc::memcpy(
                &raw mut v as *mut BYTE as *mut c_void,
                srcPtr as *const c_void,
                2 as c_int as c_ulong as ::libc::size_t,
            );
            ::libc::memcpy(
                (&raw mut v as *mut BYTE).offset(2 as c_int as isize) as *mut BYTE
                    as *mut c_void,
                srcPtr as *const c_void,
                2 as c_int as c_ulong as ::libc::size_t,
            );
            ::libc::memcpy(
                (&raw mut v as *mut BYTE).offset(4 as c_int as isize) as *mut BYTE
                    as *mut c_void,
                &raw mut v as *mut BYTE as *const c_void,
                4 as c_int as c_ulong as ::libc::size_t,
            );
        }
        4 => {
            ::libc::memcpy(
                &raw mut v as *mut BYTE as *mut c_void,
                srcPtr as *const c_void,
                4 as c_int as c_ulong as ::libc::size_t,
            );
            ::libc::memcpy(
                (&raw mut v as *mut BYTE).offset(4 as c_int as isize) as *mut BYTE
                    as *mut c_void,
                srcPtr as *const c_void,
                4 as c_int as c_ulong as ::libc::size_t,
            );
        }
        _ => {
            LZ4_memcpy_using_offset_base(dstPtr, srcPtr, dstEnd, offset);
            return;
        }
    }
    ::libc::memcpy(
        dstPtr as *mut c_void,
        &raw mut v as *mut BYTE as *const c_void,
        8 as c_int as c_ulong as ::libc::size_t,
    );
    dstPtr = dstPtr.offset(8 as c_int as isize);
    while dstPtr < dstEnd {
        ::libc::memcpy(
            dstPtr as *mut c_void,
            &raw mut v as *mut BYTE as *const c_void,
            8 as c_int as c_ulong as ::libc::size_t,
        );
        dstPtr = dstPtr.offset(8 as c_int as isize);
    }
}
extern "C" fn LZ4_NbCommonBytes(mut val: reg_t) -> c_uint { {
    if LZ4_isLittleEndian() != 0 {
        if ::core::mem::size_of::<reg_t>() as usize == 8 as usize {
            return (val as c_ulonglong).trailing_zeros() as i32
                as c_uint
                >> 3 as c_int;
        } else {
            return (val as U32).trailing_zeros() as i32 as c_uint
                >> 3 as c_int;
        }
    } else if ::core::mem::size_of::<reg_t>() as usize == 8 as usize {
        return (val as c_ulonglong).leading_zeros() as i32 as c_uint
            >> 3 as c_int;
    } else {
        return (val as U32).leading_zeros() as i32 as c_uint
            >> 3 as c_int;
    };
} }

#[inline(always)]
unsafe extern "C" fn LZ4_count(
    mut pIn: *const BYTE,
    mut pMatch: *const BYTE,
    mut pInLimit: *const BYTE,
) -> c_uint {
    let pStart: *const BYTE = pIn;
    if ((pIn
        < pInLimit.offset(
            -((::core::mem::size_of::<reg_t>() as usize).wrapping_sub(1 as usize) as isize),
        )) as c_int
        != 0 as c_int) as c_int as c_long
        != 0
    {
        let diff: reg_t = LZ4_read_ARCH(pMatch as *const c_void) as reg_t
            ^ LZ4_read_ARCH(pIn as *const c_void) as reg_t;
        if diff == 0 {
            pIn = pIn.offset(STEPSIZE as isize);
            pMatch = pMatch.offset(STEPSIZE as isize);
        } else {
            return LZ4_NbCommonBytes(diff);
        }
    }
    while ((pIn
        < pInLimit.offset(
            -((::core::mem::size_of::<reg_t>() as usize).wrapping_sub(1 as usize) as isize),
        )) as c_int
        != 0 as c_int) as c_int as c_long
        != 0
    {
        let diff_0: reg_t = LZ4_read_ARCH(pMatch as *const c_void) as reg_t
            ^ LZ4_read_ARCH(pIn as *const c_void) as reg_t;
        if diff_0 == 0 {
            pIn = pIn.offset(STEPSIZE as isize);
            pMatch = pMatch.offset(STEPSIZE as isize);
        } else {
            pIn = pIn.offset(LZ4_NbCommonBytes(diff_0) as isize);
            return pIn.offset_from(pStart) as c_long as c_uint;
        }
    }
    if STEPSIZE == 8 as usize
        && pIn < pInLimit.offset(-(3 as c_int as isize))
        && LZ4_read32(pMatch as *const c_void)
            == LZ4_read32(pIn as *const c_void)
    {
        pIn = pIn.offset(4 as c_int as isize);
        pMatch = pMatch.offset(4 as c_int as isize);
    }
    if pIn < pInLimit.offset(-(1 as c_int as isize))
        && LZ4_read16(pMatch as *const c_void) as c_int
            == LZ4_read16(pIn as *const c_void) as c_int
    {
        pIn = pIn.offset(2 as c_int as isize);
        pMatch = pMatch.offset(2 as c_int as isize);
    }
    if pIn < pInLimit && *pMatch as c_int == *pIn as c_int {
        pIn = pIn.offset(1);
    }
    return pIn.offset_from(pStart) as c_long as c_uint;
}
static mut LZ4_64Klimit: c_int = 64 as c_int
    * ((1 as c_int) << 10 as c_int)
    + (MFLIMIT - 1 as c_int);
static mut LZ4_skipTrigger: U32 = 6 as U32;
#[inline]
pub fn LZ4_versionNumber() -> c_int { {
    return LZ4_VERSION_NUMBER;
} }
#[inline]
pub fn LZ4_versionString() -> *const c_char { {
    return b"1.10.0\0" as *const u8 as *const c_char;
} }
#[inline]
pub fn LZ4_compressBound(mut isize: c_int) -> c_int { {
    return if isize as c_uint > LZ4_MAX_INPUT_SIZE as c_uint {
        0 as c_int
    } else {
        isize + isize / 255 as c_int + 16 as c_int
    };
} }
#[inline]
pub fn LZ4_sizeofState() -> c_int { {
    return ::core::mem::size_of::<LZ4_stream_t>() as c_int;
} }
#[inline(always)]
fn LZ4_hash4(mut sequence: U32, tableType: tableType_t) -> U32 { {
    if tableType as c_uint == byU16 as c_int as c_uint {
        return sequence.wrapping_mul(2654435761 as U32)
            >> MINMATCH * 8 as c_int - (LZ4_HASHLOG + 1 as c_int);
    } else {
        return sequence.wrapping_mul(2654435761 as U32)
            >> MINMATCH * 8 as c_int - LZ4_HASHLOG;
    };
} }
#[inline(always)]
fn LZ4_hash5(mut sequence: U64, tableType: tableType_t) -> U32 { {
    let hashLog: U32 = (if tableType as c_uint
        == byU16 as c_int as c_uint
    {
        LZ4_HASHLOG + 1 as c_int
    } else {
        LZ4_HASHLOG
    }) as U32;
    if LZ4_isLittleEndian() != 0 {
        let prime5bytes: U64 = 889523592379 as U64;
        return ((sequence << 24 as c_int).wrapping_mul(prime5bytes)
            >> (64 as U32).wrapping_sub(hashLog)) as U32;
    } else {
        let prime8bytes: U64 = 11400714785074694791 as U64;
        return ((sequence >> 24 as c_int).wrapping_mul(prime8bytes)
            >> (64 as U32).wrapping_sub(hashLog)) as U32;
    };
} }
#[inline(always)]
unsafe fn LZ4_hashPosition(
    p: *const c_void,
    tableType: tableType_t,
) -> U32 {
    if ::core::mem::size_of::<reg_t>() as usize == 8 as usize
        && tableType as c_uint != byU16 as c_int as c_uint
    {
        return LZ4_hash5(LZ4_read_ARCH(p) as U64, tableType);
    }
    return LZ4_hash4(LZ4_read32(p), tableType);
}
#[inline(always)]
unsafe fn LZ4_clearHash(
    mut h: U32,
    mut tableBase: *mut c_void,
    tableType: tableType_t,
) {
    match tableType as c_uint {
        1 => {
            let mut hashTable: *mut *const BYTE = tableBase as *mut *const BYTE;
            let ref mut fresh13 = *hashTable.offset(h as isize);
            *fresh13 = ::core::ptr::null::<BYTE>();
            return;
        }
        2 => {
            let mut hashTable_0: *mut U32 = tableBase as *mut U32;
            *hashTable_0.offset(h as isize) = 0 as U32;
            return;
        }
        3 => {
            let mut hashTable_1: *mut U16 = tableBase as *mut U16;
            *hashTable_1.offset(h as isize) = 0 as U16;
            return;
        }
        0 | _ => return,
    };
}
#[inline(always)]
unsafe fn LZ4_putIndexOnHash(
    mut idx: U32,
    mut h: U32,
    mut tableBase: *mut c_void,
    tableType: tableType_t,
) {
    match tableType as c_uint {
        2 => {
            let mut hashTable: *mut U32 = tableBase as *mut U32;
            *hashTable.offset(h as isize) = idx;
            return;
        }
        3 => {
            let mut hashTable_0: *mut U16 = tableBase as *mut U16;
            *hashTable_0.offset(h as isize) = idx as U16;
            return;
        }
        0 | 1 | _ => return,
    };
}
#[inline(always)]
unsafe fn LZ4_putPositionOnHash(
    mut p: *const BYTE,
    mut h: U32,
    mut tableBase: *mut c_void,
    tableType: tableType_t,
) {
    let hashTable: *mut *const BYTE = tableBase as *mut *const BYTE;
    let ref mut fresh12 = *hashTable.offset(h as isize);
    *fresh12 = p;
}
#[inline(always)]
unsafe fn LZ4_putPosition(
    mut p: *const BYTE,
    mut tableBase: *mut c_void,
    mut tableType: tableType_t,
) {
    let h: U32 = LZ4_hashPosition(p as *const c_void, tableType) as U32;
    LZ4_putPositionOnHash(p, h, tableBase, tableType);
}
#[inline(always)]
unsafe fn LZ4_getIndexOnHash(
    mut h: U32,
    mut tableBase: *const c_void,
    mut tableType: tableType_t,
) -> U32 {
    if tableType as c_uint == byU32 as c_int as c_uint {
        let hashTable: *const U32 = tableBase as *const U32;
        return *hashTable.offset(h as isize);
    }
    if tableType as c_uint == byU16 as c_int as c_uint {
        let hashTable_0: *const U16 = tableBase as *const U16;
        return *hashTable_0.offset(h as isize) as U32;
    }
    return 0 as U32;
}
unsafe fn LZ4_getPositionOnHash(
    mut h: U32,
    mut tableBase: *const c_void,
    mut tableType: tableType_t,
) -> *const BYTE {
    let mut hashTable: *const *const BYTE = tableBase as *const *const BYTE;
    return *hashTable.offset(h as isize);
}
#[inline(always)]
unsafe fn LZ4_getPosition(
    mut p: *const BYTE,
    mut tableBase: *const c_void,
    mut tableType: tableType_t,
) -> *const BYTE {
    let h: U32 = LZ4_hashPosition(p as *const c_void, tableType) as U32;
    return LZ4_getPositionOnHash(h, tableBase, tableType);
}
#[inline(always)]
unsafe fn LZ4_prepareTable(
    cctx: *mut LZ4_stream_t_internal,
    inputSize: c_int,
    tableType: tableType_t,
) {
    if (*cctx).tableType as tableType_t as c_uint
        != clearedTable as c_int as c_uint
    {
        if (*cctx).tableType as tableType_t as c_uint
            != tableType as c_uint
            || tableType as c_uint
                == byU16 as c_int as c_uint
                && (*cctx).currentOffset.wrapping_add(inputSize as LZ4_u32) >= 0xffff as LZ4_u32
            || tableType as c_uint
                == byU32 as c_int as c_uint
                && (*cctx).currentOffset
                    > (1 as LZ4_u32).wrapping_mul((1 as LZ4_u32) << 30 as c_int)
            || tableType as c_uint
                == byPtr as c_int as c_uint
            || inputSize
                >= 4 as c_int * ((1 as c_int) << 10 as c_int)
        {
            memset(
                &raw mut (*cctx).hashTable as *mut LZ4_u32 as *mut c_void,
                0 as c_int,
                ((1 as c_int) << 14 as c_int) as size_t,
            );
            (*cctx).currentOffset = 0 as LZ4_u32;
            (*cctx).tableType = clearedTable as c_int as U32 as LZ4_u32;
        }
    }
    if (*cctx).currentOffset != 0 as LZ4_u32
        && tableType as c_uint == byU32 as c_int as c_uint
    {
        (*cctx).currentOffset = (*cctx).currentOffset.wrapping_add(
            (64 as c_int * ((1 as c_int) << 10 as c_int))
                as LZ4_u32,
        );
    }
    (*cctx).dictCtx = ::core::ptr::null::<LZ4_stream_t_internal>();
    (*cctx).dictionary = ::core::ptr::null::<LZ4_byte>();
    (*cctx).dictSize = 0 as LZ4_u32;
}
#[inline(always)]
unsafe fn LZ4_compress_generic_validated(
    cctx: *mut LZ4_stream_t_internal,
    source: *const c_char,
    dest: *mut c_char,
    inputSize: c_int,
    mut inputConsumed: *mut c_int,
    maxOutputSize: c_int,
    outputDirective: limitedOutput_directive,
    tableType: tableType_t,
    dictDirective: dict_directive,
    dictIssue: dictIssue_directive,
    acceleration: c_int,
) -> c_int {
    let mut result: c_int = 0;
    let mut ip: *const BYTE = source as *const BYTE;
    let startIndex: U32 = (*cctx).currentOffset as U32;
    let mut base: *const BYTE = (source as *const BYTE).offset(-(startIndex as isize));
    let mut lowLimit: *const BYTE = ::core::ptr::null::<BYTE>();
    let mut dictCtx: *const LZ4_stream_t_internal = (*cctx).dictCtx;
    let dictionary: *const BYTE = if dictDirective as c_uint
        == usingDictCtx as c_int as c_uint
    {
        (*dictCtx).dictionary as *const BYTE
    } else {
        (*cctx).dictionary as *const BYTE
    };
    let dictSize: U32 = if dictDirective as c_uint
        == usingDictCtx as c_int as c_uint
    {
        (*dictCtx).dictSize as U32
    } else {
        (*cctx).dictSize as U32
    };
    let dictDelta: U32 = if dictDirective as c_uint
        == usingDictCtx as c_int as c_uint
    {
        startIndex.wrapping_sub((*dictCtx).currentOffset as U32)
    } else {
        0 as U32
    };
    let maybe_extMem: c_int = (dictDirective as c_uint
        == usingExtDict as c_int as c_uint
        || dictDirective as c_uint
            == usingDictCtx as c_int as c_uint)
        as c_int;
    let prefixIdxLimit: U32 = startIndex.wrapping_sub(dictSize);
    let dictEnd: *const BYTE = if !dictionary.is_null() {
        dictionary.offset(dictSize as isize)
    } else {
        dictionary
    };
    let mut anchor: *const BYTE = source as *const BYTE;
    let iend: *const BYTE = ip.offset(inputSize as isize);
    let mflimitPlusOne: *const BYTE = iend
        .offset(-(MFLIMIT as isize))
        .offset(1 as c_int as isize);
    let matchlimit: *const BYTE = iend.offset(-(LASTLITERALS as isize));
    let mut dictBase: *const BYTE = if dictionary.is_null() {
        ::core::ptr::null::<BYTE>()
    } else if dictDirective as c_uint
        == usingDictCtx as c_int as c_uint
    {
        dictionary
            .offset(dictSize as isize)
            .offset(-((*dictCtx).currentOffset as isize))
    } else {
        dictionary
            .offset(dictSize as isize)
            .offset(-(startIndex as isize))
    };
    let mut op: *mut BYTE = dest as *mut BYTE;
    let olimit: *mut BYTE = op.offset(maxOutputSize as isize);
    let mut offset: U32 = 0 as U32;
    let mut forwardH: U32 = 0;
    tableType as c_uint == byU16 as c_int as c_uint;
    tableType as c_uint == byPtr as c_int as c_uint;
    if outputDirective as c_uint
        == fillOutput as c_int as c_uint
        && maxOutputSize < 1 as c_int
    {
        return 0 as c_int;
    }
    lowLimit = (source as *const BYTE).offset(
        -((if dictDirective as c_uint
            == withPrefix64k as c_int as c_uint
        {
            dictSize
        } else {
            0 as U32
        }) as isize),
    );
    if dictDirective as c_uint
        == usingDictCtx as c_int as c_uint
    {
        (*cctx).dictCtx = ::core::ptr::null::<LZ4_stream_t_internal>();
        (*cctx).dictSize = inputSize as U32 as LZ4_u32;
    } else {
        (*cctx).dictSize = ((*cctx).dictSize as uint32_t).wrapping_add(inputSize as U32 as uint32_t)
            as LZ4_u32 as LZ4_u32;
    }
    (*cctx).currentOffset = ((*cctx).currentOffset as uint32_t)
        .wrapping_add(inputSize as U32 as uint32_t) as LZ4_u32
        as LZ4_u32;
    (*cctx).tableType = tableType as U32 as LZ4_u32;
    if !(inputSize < LZ4_minLength) {
        let h: U32 = LZ4_hashPosition(ip as *const c_void, tableType) as U32;
        if tableType as c_uint == byPtr as c_int as c_uint {
            LZ4_putPositionOnHash(
                ip,
                h,
                &raw mut (*cctx).hashTable as *mut LZ4_u32 as *mut c_void,
                byPtr,
            );
        } else {
            LZ4_putIndexOnHash(
                startIndex,
                h,
                &raw mut (*cctx).hashTable as *mut LZ4_u32 as *mut c_void,
                tableType,
            );
        }
        ip = ip.offset(1);
        forwardH = LZ4_hashPosition(ip as *const c_void, tableType);
        's_156: loop {
            let mut match_0: *const BYTE = ::core::ptr::null::<BYTE>();
            let mut token: *mut BYTE = ::core::ptr::null_mut::<BYTE>();
            let mut filledIp: *const BYTE = ::core::ptr::null::<BYTE>();
            if tableType as c_uint
                == byPtr as c_int as c_uint
            {
                let mut forwardIp: *const BYTE = ip;
                let mut step: c_int = 1 as c_int;
                let mut searchMatchNb: c_int = acceleration << LZ4_skipTrigger;
                loop {
                    let h_0: U32 = forwardH;
                    ip = forwardIp;
                    forwardIp = forwardIp.offset(step as isize);
                    let fresh0 = searchMatchNb;
                    searchMatchNb = searchMatchNb + 1;
                    step = fresh0 >> LZ4_skipTrigger;
                    if ((forwardIp > mflimitPlusOne) as c_int
                        != 0 as c_int) as c_int
                        as c_long
                        != 0
                    {
                        break 's_156;
                    }
                    match_0 = LZ4_getPositionOnHash(
                        h_0,
                        &raw mut (*cctx).hashTable as *mut LZ4_u32 as *const c_void,
                        tableType,
                    );
                    forwardH = LZ4_hashPosition(forwardIp as *const c_void, tableType);
                    LZ4_putPositionOnHash(
                        ip,
                        h_0,
                        &raw mut (*cctx).hashTable as *mut LZ4_u32 as *mut c_void,
                        tableType,
                    );
                    if !(match_0.offset(LZ4_DISTANCE_MAX as isize) < ip
                        || LZ4_read32(match_0 as *const c_void)
                            != LZ4_read32(ip as *const c_void))
                    {
                        break;
                    }
                }
            } else {
                let mut forwardIp_0: *const BYTE = ip;
                let mut step_0: c_int = 1 as c_int;
                let mut searchMatchNb_0: c_int = acceleration << LZ4_skipTrigger;
                loop {
                    let h_1: U32 = forwardH;
                    let current: U32 = forwardIp_0.offset_from(base) as c_long as U32;
                    let mut matchIndex: U32 = LZ4_getIndexOnHash(
                        h_1,
                        &raw mut (*cctx).hashTable as *mut LZ4_u32 as *const c_void,
                        tableType,
                    );
                    ip = forwardIp_0;
                    forwardIp_0 = forwardIp_0.offset(step_0 as isize);
                    let fresh1 = searchMatchNb_0;
                    searchMatchNb_0 = searchMatchNb_0 + 1;
                    step_0 = fresh1 >> LZ4_skipTrigger;
                    if ((forwardIp_0 > mflimitPlusOne) as c_int
                        != 0 as c_int) as c_int
                        as c_long
                        != 0
                    {
                        break 's_156;
                    }
                    if dictDirective as c_uint
                        == usingDictCtx as c_int as c_uint
                    {
                        if matchIndex < startIndex {
                            matchIndex = LZ4_getIndexOnHash(
                                h_1,
                                &raw const (*dictCtx).hashTable as *const LZ4_u32
                                    as *const c_void,
                                byU32,
                            );
                            match_0 = dictBase.offset(matchIndex as isize);
                            matchIndex = matchIndex.wrapping_add(dictDelta);
                            lowLimit = dictionary;
                        } else {
                            match_0 = base.offset(matchIndex as isize);
                            lowLimit = source as *const BYTE;
                        }
                    } else if dictDirective as c_uint
                        == usingExtDict as c_int as c_uint
                    {
                        if matchIndex < startIndex {
                            match_0 = dictBase.offset(matchIndex as isize);
                            lowLimit = dictionary;
                        } else {
                            match_0 = base.offset(matchIndex as isize);
                            lowLimit = source as *const BYTE;
                        }
                    } else {
                        match_0 = base.offset(matchIndex as isize);
                    }
                    forwardH =
                        LZ4_hashPosition(forwardIp_0 as *const c_void, tableType);
                    LZ4_putIndexOnHash(
                        current,
                        h_1,
                        &raw mut (*cctx).hashTable as *mut LZ4_u32 as *mut c_void,
                        tableType,
                    );
                    if dictIssue as c_uint
                        == dictSmall as c_int as c_uint
                        && matchIndex < prefixIdxLimit
                    {
                        continue;
                    }
                    if (tableType as c_uint
                        != byU16 as c_int as c_uint
                        || LZ4_DISTANCE_MAX < LZ4_DISTANCE_ABSOLUTE_MAX)
                        && matchIndex.wrapping_add(LZ4_DISTANCE_MAX as U32) < current
                    {
                        continue;
                    }
                    if !(LZ4_read32(match_0 as *const c_void)
                        == LZ4_read32(ip as *const c_void))
                    {
                        continue;
                    }
                    if maybe_extMem != 0 {
                        offset = current.wrapping_sub(matchIndex);
                    }
                    break;
                }
            }
            filledIp = ip;
            if match_0 > lowLimit
                && ((*ip.offset(-(1 as c_int) as isize) as c_int
                    == *match_0.offset(-(1 as c_int) as isize) as c_int)
                    as c_int
                    != 0 as c_int) as c_int
                    as c_long
                    != 0
            {
                loop {
                    ip = ip.offset(-1);
                    match_0 = match_0.offset(-1);
                    if !((ip > anchor) as c_int
                        & (match_0 > lowLimit) as c_int
                        != 0
                        && ((*ip.offset(-(1 as c_int) as isize) as c_int
                            == *match_0.offset(-(1 as c_int) as isize)
                                as c_int)
                            as c_int
                            != 0 as c_int)
                            as c_int as c_long
                            != 0)
                    {
                        break;
                    }
                }
            }
            let litLength: c_uint =
                ip.offset_from(anchor) as c_long as c_uint;
            let fresh2 = op;
            op = op.offset(1);
            token = fresh2;
            if outputDirective as c_uint
                == limitedOutput as c_int as c_uint
                && ((op
                    .offset(litLength as isize)
                    .offset(
                        (2 as c_int
                            + 1 as c_int
                            + 5 as c_int) as isize,
                    )
                    .offset(litLength.wrapping_div(255 as c_uint) as isize)
                    > olimit) as c_int
                    != 0 as c_int) as c_int
                    as c_long
                    != 0
            {
                return 0 as c_int;
            }
            if outputDirective as c_uint
                == fillOutput as c_int as c_uint
                && ((op
                    .offset(
                        litLength
                            .wrapping_add(240 as c_uint)
                            .wrapping_div(255 as c_uint)
                            as isize,
                    )
                    .offset(litLength as isize)
                    .offset(2 as c_int as isize)
                    .offset(1 as c_int as isize)
                    .offset(12 as c_int as isize)
                    .offset(-(4 as c_int as isize))
                    > olimit) as c_int
                    != 0 as c_int) as c_int
                    as c_long
                    != 0
            {
                op = op.offset(-1);
                break;
            } else {
                if litLength >= RUN_MASK {
                    let mut len: c_uint = litLength.wrapping_sub(RUN_MASK);
                    *token = (RUN_MASK << ML_BITS) as BYTE;
                    while len >= 255 as c_uint {
                        let fresh3 = op;
                        op = op.offset(1);
                        *fresh3 = 255 as BYTE;
                        len = len.wrapping_sub(255 as c_uint);
                    }
                    let fresh4 = op;
                    op = op.offset(1);
                    *fresh4 = len as BYTE;
                } else {
                    *token = (litLength << ML_BITS) as BYTE;
                }
                LZ4_wildCopy8(
                    op as *mut c_void,
                    anchor as *const c_void,
                    op.offset(litLength as isize) as *mut c_void,
                );
                op = op.offset(litLength as isize);
                loop {
                    if outputDirective as c_uint
                        == fillOutput as c_int as c_uint
                        && op
                            .offset(2 as c_int as isize)
                            .offset(1 as c_int as isize)
                            .offset(MFLIMIT as isize)
                            .offset(-(MINMATCH as isize))
                            > olimit
                    {
                        op = token;
                        break 's_156;
                    } else {
                        if maybe_extMem != 0 {
                            LZ4_writeLE16(op as *mut c_void, offset as U16);
                            op = op.offset(2 as c_int as isize);
                        } else {
                            LZ4_writeLE16(
                                op as *mut c_void,
                                ip.offset_from(match_0) as c_long as U16,
                            );
                            op = op.offset(2 as c_int as isize);
                        }
                        let mut matchCode: c_uint = 0;
                        if (dictDirective as c_uint
                            == usingExtDict as c_int as c_uint
                            || dictDirective as c_uint
                                == usingDictCtx as c_int as c_uint)
                            && lowLimit == dictionary
                        {
                            let mut limit: *const BYTE = ip.offset(dictEnd.offset_from(match_0)
                                as c_long
                                as isize);
                            if limit > matchlimit {
                                limit = matchlimit;
                            }
                            matchCode = LZ4_count(
                                ip.offset(MINMATCH as isize),
                                match_0.offset(MINMATCH as isize),
                                limit,
                            );
                            ip = ip.offset(
                                (matchCode as size_t).wrapping_add(MINMATCH as size_t) as isize
                            );
                            if ip == limit {
                                let more: c_uint =
                                    LZ4_count(limit, source as *const BYTE, matchlimit)
                                        as c_uint;
                                matchCode = matchCode.wrapping_add(more);
                                ip = ip.offset(more as isize);
                            }
                        } else {
                            matchCode = LZ4_count(
                                ip.offset(MINMATCH as isize),
                                match_0.offset(MINMATCH as isize),
                                matchlimit,
                            );
                            ip = ip.offset(
                                (matchCode as size_t).wrapping_add(MINMATCH as size_t) as isize
                            );
                        }
                        if outputDirective as c_uint != 0
                            && ((op
                                .offset(
                                    (1 as c_int + 5 as c_int) as isize,
                                )
                                .offset(
                                    matchCode
                                        .wrapping_add(240 as c_uint)
                                        .wrapping_div(255 as c_uint)
                                        as isize,
                                )
                                > olimit) as c_int
                                != 0 as c_int)
                                as c_int
                                as c_long
                                != 0
                        {
                            if outputDirective as c_uint
                                == fillOutput as c_int as c_uint
                            {
                                let mut newMatchCode: U32 =
                                    ((15 as c_int - 1 as c_int) as U32)
                                        .wrapping_add(
                                            (olimit.offset_from(op) as c_long as U32)
                                                .wrapping_sub(1 as U32)
                                                .wrapping_sub(LASTLITERALS as U32)
                                                .wrapping_mul(255 as U32),
                                        );
                                ip = ip.offset(
                                    -((matchCode as U32).wrapping_sub(newMatchCode) as isize),
                                );
                                matchCode = newMatchCode as c_uint;
                                if ((ip <= filledIp) as c_int
                                    != 0 as c_int)
                                    as c_int
                                    as c_long
                                    != 0
                                {
                                    let mut ptr: *const BYTE = ::core::ptr::null::<BYTE>();
                                    ptr = ip;
                                    while ptr <= filledIp {
                                        let h_2: U32 = LZ4_hashPosition(
                                            ptr as *const c_void,
                                            tableType,
                                        )
                                            as U32;
                                        LZ4_clearHash(
                                            h_2,
                                            &raw mut (*cctx).hashTable as *mut LZ4_u32
                                                as *mut c_void,
                                            tableType,
                                        );
                                        ptr = ptr.offset(1);
                                    }
                                }
                            } else {
                                return 0 as c_int;
                            }
                        }
                        if matchCode >= ML_MASK {
                            *token = (*token as c_uint).wrapping_add(ML_MASK) as BYTE
                                as BYTE;
                            matchCode = matchCode.wrapping_sub(ML_MASK);
                            LZ4_write32(op as *mut c_void, 0xffffffff as U32);
                            while matchCode
                                >= (4 as c_int * 255 as c_int)
                                    as c_uint
                            {
                                op = op.offset(4 as c_int as isize);
                                LZ4_write32(op as *mut c_void, 0xffffffff as U32);
                                matchCode = matchCode.wrapping_sub(
                                    (4 as c_int * 255 as c_int)
                                        as c_uint,
                                );
                            }
                            op =
                                op.offset(
                                    matchCode.wrapping_div(255 as c_uint) as isize
                                );
                            let fresh5 = op;
                            op = op.offset(1);
                            *fresh5 = matchCode.wrapping_rem(255 as c_uint) as BYTE;
                        } else {
                            *token = (*token as c_int
                                + matchCode as BYTE as c_int)
                                as BYTE;
                        }
                        anchor = ip;
                        if ip >= mflimitPlusOne {
                            break 's_156;
                        }
                        let h_3: U32 = LZ4_hashPosition(
                            ip.offset(-(2 as c_int as isize))
                                as *const c_void,
                            tableType,
                        ) as U32;
                        if tableType as c_uint
                            == byPtr as c_int as c_uint
                        {
                            LZ4_putPositionOnHash(
                                ip.offset(-(2 as c_int as isize)),
                                h_3,
                                &raw mut (*cctx).hashTable as *mut LZ4_u32
                                    as *mut c_void,
                                byPtr,
                            );
                        } else {
                            let idx: U32 =
                                ip.offset(-(2 as c_int as isize))
                                    .offset_from(base)
                                    as c_long as U32;
                            LZ4_putIndexOnHash(
                                idx,
                                h_3,
                                &raw mut (*cctx).hashTable as *mut LZ4_u32
                                    as *mut c_void,
                                tableType,
                            );
                        }
                        if tableType as c_uint
                            == byPtr as c_int as c_uint
                        {
                            match_0 = LZ4_getPosition(
                                ip,
                                &raw mut (*cctx).hashTable as *mut LZ4_u32
                                    as *const c_void,
                                tableType,
                            );
                            LZ4_putPosition(
                                ip,
                                &raw mut (*cctx).hashTable as *mut LZ4_u32
                                    as *mut c_void,
                                tableType,
                            );
                            if !(match_0.offset(LZ4_DISTANCE_MAX as isize) >= ip
                                && LZ4_read32(match_0 as *const c_void)
                                    == LZ4_read32(ip as *const c_void))
                            {
                                break;
                            }
                            let fresh6 = op;
                            op = op.offset(1);
                            token = fresh6;
                            *token = 0 as BYTE;
                        } else {
                            let h_4: U32 =
                                LZ4_hashPosition(ip as *const c_void, tableType)
                                    as U32;
                            let current_0: U32 = ip.offset_from(base) as c_long as U32;
                            let mut matchIndex_0: U32 = LZ4_getIndexOnHash(
                                h_4,
                                &raw mut (*cctx).hashTable as *mut LZ4_u32
                                    as *const c_void,
                                tableType,
                            );
                            if dictDirective as c_uint
                                == usingDictCtx as c_int as c_uint
                            {
                                if matchIndex_0 < startIndex {
                                    matchIndex_0 = LZ4_getIndexOnHash(
                                        h_4,
                                        &raw const (*dictCtx).hashTable as *const LZ4_u32
                                            as *const c_void,
                                        byU32,
                                    );
                                    match_0 = dictBase.offset(matchIndex_0 as isize);
                                    lowLimit = dictionary;
                                    matchIndex_0 = matchIndex_0.wrapping_add(dictDelta);
                                } else {
                                    match_0 = base.offset(matchIndex_0 as isize);
                                    lowLimit = source as *const BYTE;
                                }
                            } else if dictDirective as c_uint
                                == usingExtDict as c_int as c_uint
                            {
                                if matchIndex_0 < startIndex {
                                    match_0 = dictBase.offset(matchIndex_0 as isize);
                                    lowLimit = dictionary;
                                } else {
                                    match_0 = base.offset(matchIndex_0 as isize);
                                    lowLimit = source as *const BYTE;
                                }
                            } else {
                                match_0 = base.offset(matchIndex_0 as isize);
                            }
                            LZ4_putIndexOnHash(
                                current_0,
                                h_4,
                                &raw mut (*cctx).hashTable as *mut LZ4_u32
                                    as *mut c_void,
                                tableType,
                            );
                            if !((if dictIssue as c_uint
                                == dictSmall as c_int as c_uint
                            {
                                (matchIndex_0 >= prefixIdxLimit) as c_int
                            } else {
                                1 as c_int
                            }) != 0
                                && (if tableType as c_uint
                                    == byU16 as c_int as c_uint
                                    && LZ4_DISTANCE_MAX == LZ4_DISTANCE_ABSOLUTE_MAX
                                {
                                    1 as c_int
                                } else {
                                    (matchIndex_0.wrapping_add(LZ4_DISTANCE_MAX as U32)
                                        >= current_0)
                                        as c_int
                                }) != 0
                                && LZ4_read32(match_0 as *const c_void)
                                    == LZ4_read32(ip as *const c_void))
                            {
                                break;
                            }
                            let fresh7 = op;
                            op = op.offset(1);
                            token = fresh7;
                            *token = 0 as BYTE;
                            if maybe_extMem != 0 {
                                offset = current_0.wrapping_sub(matchIndex_0);
                            }
                        }
                    }
                }
                ip = ip.offset(1);
                forwardH = LZ4_hashPosition(ip as *const c_void, tableType);
            }
        }
    }
    let mut lastRun: size_t = iend.offset_from(anchor) as c_long as size_t;
    if outputDirective as c_uint != 0
        && op
            .offset(lastRun as isize)
            .offset(1 as c_int as isize)
            .offset(
                lastRun
                    .wrapping_add(255 as size_t)
                    .wrapping_sub(RUN_MASK as size_t)
                    .wrapping_div(255 as size_t) as isize,
            )
            > olimit
    {
        if outputDirective as c_uint
            == fillOutput as c_int as c_uint
        {
            lastRun =
                (olimit.offset_from(op) as c_long as size_t).wrapping_sub(1 as size_t);
            lastRun = lastRun.wrapping_sub(
                lastRun
                    .wrapping_add(256 as size_t)
                    .wrapping_sub(RUN_MASK as size_t)
                    .wrapping_div(256 as size_t),
            );
        } else {
            return 0 as c_int;
        }
    }
    if lastRun >= RUN_MASK as size_t {
        let mut accumulator: size_t = lastRun.wrapping_sub(RUN_MASK as size_t);
        let fresh8 = op;
        op = op.offset(1);
        *fresh8 = (RUN_MASK << ML_BITS) as BYTE;
        while accumulator >= 255 as size_t {
            let fresh9 = op;
            op = op.offset(1);
            *fresh9 = 255 as BYTE;
            accumulator = accumulator.wrapping_sub(255 as size_t);
        }
        let fresh10 = op;
        op = op.offset(1);
        *fresh10 = accumulator as BYTE;
    } else {
        let fresh11 = op;
        op = op.offset(1);
        *fresh11 = (lastRun << ML_BITS) as BYTE;
    }
    ::libc::memcpy(
        op as *mut c_void,
        anchor as *const c_void,
        lastRun as ::libc::size_t,
    );
    ip = anchor.offset(lastRun as isize);
    op = op.offset(lastRun as isize);
    if outputDirective as c_uint
        == fillOutput as c_int as c_uint
    {
        *inputConsumed = (ip as *const c_char).offset_from(source)
            as c_long as c_int;
    }
    result = (op as *mut c_char).offset_from(dest) as c_long
        as c_int;
    return result;
}
#[inline(always)]
unsafe fn LZ4_compress_generic(
    cctx: *mut LZ4_stream_t_internal,
    src: *const c_char,
    dst: *mut c_char,
    srcSize: c_int,
    mut inputConsumed: *mut c_int,
    dstCapacity: c_int,
    outputDirective: limitedOutput_directive,
    tableType: tableType_t,
    dictDirective: dict_directive,
    dictIssue: dictIssue_directive,
    acceleration: c_int,
) -> c_int {
    if srcSize as U32 > LZ4_MAX_INPUT_SIZE as U32 {
        return 0 as c_int;
    }
    if srcSize == 0 as c_int {
        if outputDirective as c_uint
            != notLimited as c_int as c_uint
            && dstCapacity <= 0 as c_int
        {
            return 0 as c_int;
        }
        *dst.offset(0 as c_int as isize) = 0 as c_char;
        if outputDirective as c_uint
            == fillOutput as c_int as c_uint
        {
            *inputConsumed = 0 as c_int;
        }
        return 1 as c_int;
    }
    return LZ4_compress_generic_validated(
        cctx,
        src,
        dst,
        srcSize,
        inputConsumed,
        dstCapacity,
        outputDirective,
        tableType,
        dictDirective,
        dictIssue,
        acceleration,
    );
}
#[inline]
pub unsafe fn LZ4_compress_fast_extState(
    mut state: *mut c_void,
    mut source: *const c_char,
    mut dest: *mut c_char,
    mut inputSize: c_int,
    mut maxOutputSize: c_int,
    mut acceleration: c_int,
) -> c_int {
    let ctx: *mut LZ4_stream_t_internal = &raw mut (*(LZ4_initStream
        as unsafe extern "C" fn(*mut c_void, size_t) -> *mut LZ4_stream_t)(
        state,
        ::core::mem::size_of::<LZ4_stream_t>() as size_t,
    ))
    .internal_donotuse;
    if acceleration < 1 as c_int {
        acceleration = LZ4_ACCELERATION_DEFAULT;
    }
    if acceleration > LZ4_ACCELERATION_MAX {
        acceleration = LZ4_ACCELERATION_MAX;
    }
    if maxOutputSize >= LZ4_compressBound(inputSize) {
        if inputSize < LZ4_64Klimit {
            return LZ4_compress_generic(
                ctx,
                source,
                dest,
                inputSize,
                ::core::ptr::null_mut::<c_int>(),
                0 as c_int,
                notLimited,
                byU16,
                noDict,
                noDictIssue,
                acceleration,
            );
        } else {
            let tableType: tableType_t =
                (if ::core::mem::size_of::<*mut c_void>() as usize == 4 as usize
                    && source as uptrval > LZ4_DISTANCE_MAX as uptrval
                {
                    byPtr as c_int
                } else {
                    byU32 as c_int
                }) as tableType_t;
            return LZ4_compress_generic(
                ctx,
                source,
                dest,
                inputSize,
                ::core::ptr::null_mut::<c_int>(),
                0 as c_int,
                notLimited,
                tableType,
                noDict,
                noDictIssue,
                acceleration,
            );
        }
    } else if inputSize < LZ4_64Klimit {
        return LZ4_compress_generic(
            ctx,
            source,
            dest,
            inputSize,
            ::core::ptr::null_mut::<c_int>(),
            maxOutputSize,
            limitedOutput,
            byU16,
            noDict,
            noDictIssue,
            acceleration,
        );
    } else {
        let tableType_0: tableType_t =
            (if ::core::mem::size_of::<*mut c_void>() as usize == 4 as usize
                && source as uptrval > LZ4_DISTANCE_MAX as uptrval
            {
                byPtr as c_int
            } else {
                byU32 as c_int
            }) as tableType_t;
        return LZ4_compress_generic(
            ctx,
            source,
            dest,
            inputSize,
            ::core::ptr::null_mut::<c_int>(),
            maxOutputSize,
            limitedOutput,
            tableType_0,
            noDict,
            noDictIssue,
            acceleration,
        );
    };
}
#[inline]
pub unsafe fn LZ4_compress_fast_extState_fastReset(
    mut state: *mut c_void,
    mut src: *const c_char,
    mut dst: *mut c_char,
    mut srcSize: c_int,
    mut dstCapacity: c_int,
    mut acceleration: c_int,
) -> c_int {
    let ctx: *mut LZ4_stream_t_internal =
        &raw mut (*(state as *mut LZ4_stream_t)).internal_donotuse;
    if acceleration < 1 as c_int {
        acceleration = LZ4_ACCELERATION_DEFAULT;
    }
    if acceleration > LZ4_ACCELERATION_MAX {
        acceleration = LZ4_ACCELERATION_MAX;
    }
    if dstCapacity >= LZ4_compressBound(srcSize) {
        if srcSize < LZ4_64Klimit {
            let tableType: tableType_t = byU16;
            LZ4_prepareTable(ctx, srcSize, tableType);
            if (*ctx).currentOffset != 0 {
                return LZ4_compress_generic(
                    ctx,
                    src,
                    dst,
                    srcSize,
                    ::core::ptr::null_mut::<c_int>(),
                    0 as c_int,
                    notLimited,
                    tableType,
                    noDict,
                    dictSmall,
                    acceleration,
                );
            } else {
                return LZ4_compress_generic(
                    ctx,
                    src,
                    dst,
                    srcSize,
                    ::core::ptr::null_mut::<c_int>(),
                    0 as c_int,
                    notLimited,
                    tableType,
                    noDict,
                    noDictIssue,
                    acceleration,
                );
            }
        } else {
            let tableType_0: tableType_t =
                (if ::core::mem::size_of::<*mut c_void>() as usize == 4 as usize
                    && src as uptrval > LZ4_DISTANCE_MAX as uptrval
                {
                    byPtr as c_int
                } else {
                    byU32 as c_int
                }) as tableType_t;
            LZ4_prepareTable(ctx, srcSize, tableType_0);
            return LZ4_compress_generic(
                ctx,
                src,
                dst,
                srcSize,
                ::core::ptr::null_mut::<c_int>(),
                0 as c_int,
                notLimited,
                tableType_0,
                noDict,
                noDictIssue,
                acceleration,
            );
        }
    } else if srcSize < LZ4_64Klimit {
        let tableType_1: tableType_t = byU16;
        LZ4_prepareTable(ctx, srcSize, tableType_1);
        if (*ctx).currentOffset != 0 {
            return LZ4_compress_generic(
                ctx,
                src,
                dst,
                srcSize,
                ::core::ptr::null_mut::<c_int>(),
                dstCapacity,
                limitedOutput,
                tableType_1,
                noDict,
                dictSmall,
                acceleration,
            );
        } else {
            return LZ4_compress_generic(
                ctx,
                src,
                dst,
                srcSize,
                ::core::ptr::null_mut::<c_int>(),
                dstCapacity,
                limitedOutput,
                tableType_1,
                noDict,
                noDictIssue,
                acceleration,
            );
        }
    } else {
        let tableType_2: tableType_t =
            (if ::core::mem::size_of::<*mut c_void>() as usize == 4 as usize
                && src as uptrval > LZ4_DISTANCE_MAX as uptrval
            {
                byPtr as c_int
            } else {
                byU32 as c_int
            }) as tableType_t;
        LZ4_prepareTable(ctx, srcSize, tableType_2);
        return LZ4_compress_generic(
            ctx,
            src,
            dst,
            srcSize,
            ::core::ptr::null_mut::<c_int>(),
            dstCapacity,
            limitedOutput,
            tableType_2,
            noDict,
            noDictIssue,
            acceleration,
        );
    };
}
#[inline]
pub unsafe fn LZ4_compress_fast(
    mut src: *const c_char,
    mut dest: *mut c_char,
    mut srcSize: c_int,
    mut dstCapacity: c_int,
    mut acceleration: c_int,
) -> c_int {
    let mut result: c_int = 0;
    let mut ctx: LZ4_stream_t = LZ4_stream_u {
        minStateSize: [0; 16416],
    };
    let ctxPtr: *mut LZ4_stream_t = &raw mut ctx;
    result = LZ4_compress_fast_extState(
        ctxPtr as *mut c_void,
        src,
        dest,
        srcSize,
        dstCapacity,
        acceleration,
    );
    return result;
}
#[inline]
pub unsafe fn LZ4_compress_default(
    mut src: *const c_char,
    mut dst: *mut c_char,
    mut srcSize: c_int,
    mut dstCapacity: c_int,
) -> c_int {
    return LZ4_compress_fast(src, dst, srcSize, dstCapacity, 1 as c_int);
}
unsafe fn LZ4_compress_destSize_extState_internal(
    mut state: *mut LZ4_stream_t,
    mut src: *const c_char,
    mut dst: *mut c_char,
    mut srcSizePtr: *mut c_int,
    mut targetDstSize: c_int,
    mut acceleration: c_int,
) -> c_int {
    let s: *mut c_void = LZ4_initStream(
        state as *mut c_void,
        ::core::mem::size_of::<LZ4_stream_t>() as size_t,
    ) as *mut c_void;
    if targetDstSize >= LZ4_compressBound(*srcSizePtr) {
        return LZ4_compress_fast_extState(
            state as *mut c_void,
            src,
            dst,
            *srcSizePtr,
            targetDstSize,
            acceleration,
        );
    } else if *srcSizePtr < LZ4_64Klimit {
        return LZ4_compress_generic(
            &raw mut (*state).internal_donotuse,
            src,
            dst,
            *srcSizePtr,
            srcSizePtr,
            targetDstSize,
            fillOutput,
            byU16,
            noDict,
            noDictIssue,
            acceleration,
        );
    } else {
        let addrMode: tableType_t = (if ::core::mem::size_of::<*mut c_void>() as usize
            == 4 as usize
            && src as uptrval > LZ4_DISTANCE_MAX as uptrval
        {
            byPtr as c_int
        } else {
            byU32 as c_int
        }) as tableType_t;
        return LZ4_compress_generic(
            &raw mut (*state).internal_donotuse,
            src,
            dst,
            *srcSizePtr,
            srcSizePtr,
            targetDstSize,
            fillOutput,
            addrMode,
            noDict,
            noDictIssue,
            acceleration,
        );
    };
}
#[inline]
pub unsafe fn LZ4_compress_destSize_extState(
    mut state: *mut c_void,
    mut src: *const c_char,
    mut dst: *mut c_char,
    mut srcSizePtr: *mut c_int,
    mut targetDstSize: c_int,
    mut acceleration: c_int,
) -> c_int {
    let r: c_int = LZ4_compress_destSize_extState_internal(
        state as *mut LZ4_stream_t,
        src,
        dst,
        srcSizePtr,
        targetDstSize,
        acceleration,
    ) as c_int;
    LZ4_initStream(state, ::core::mem::size_of::<LZ4_stream_t>() as size_t);
    return r;
}
#[inline]
pub unsafe fn LZ4_compress_destSize(
    mut src: *const c_char,
    mut dst: *mut c_char,
    mut srcSizePtr: *mut c_int,
    mut targetDstSize: c_int,
) -> c_int {
    let mut ctxBody: LZ4_stream_t = LZ4_stream_u {
        minStateSize: [0; 16416],
    };
    let ctx: *mut LZ4_stream_t = &raw mut ctxBody;
    let mut result: c_int = LZ4_compress_destSize_extState_internal(
        ctx,
        src,
        dst,
        srcSizePtr,
        targetDstSize,
        1 as c_int,
    );
    return result;
}
#[inline]
pub fn LZ4_createStream() -> *mut LZ4_stream_t { unsafe {
    let lz4s: *mut LZ4_stream_t =
        malloc(::core::mem::size_of::<LZ4_stream_t>() as size_t) as *mut LZ4_stream_t;
    if lz4s.is_null() {
        return ::core::ptr::null_mut::<LZ4_stream_t>();
    }
    LZ4_initStream(
        lz4s as *mut c_void,
        ::core::mem::size_of::<LZ4_stream_t>() as size_t,
    );
    return lz4s;
} }
fn LZ4_stream_t_alignment() -> size_t { {
    return (::core::mem::size_of::<t_a>() as size_t)
        .wrapping_sub(::core::mem::size_of::<LZ4_stream_t>() as size_t);
} }
#[no_mangle]
pub unsafe extern "C" fn LZ4_initStream(
    mut buffer: *mut c_void,
    mut size: size_t,
) -> *mut LZ4_stream_t {
    if buffer.is_null() {
        return ::core::ptr::null_mut::<LZ4_stream_t>();
    }
    if size < ::core::mem::size_of::<LZ4_stream_t>() as usize {
        return ::core::ptr::null_mut::<LZ4_stream_t>();
    }
    if LZ4_isAligned(buffer, LZ4_stream_t_alignment()) == 0 {
        return ::core::ptr::null_mut::<LZ4_stream_t>();
    }
    memset(
        buffer,
        0 as c_int,
        ::core::mem::size_of::<LZ4_stream_t_internal>() as size_t,
    );
    return buffer as *mut LZ4_stream_t;
}
#[inline]
pub unsafe fn LZ4_resetStream(mut LZ4_stream: *mut LZ4_stream_t) {
    memset(
        LZ4_stream as *mut c_void,
        0 as c_int,
        ::core::mem::size_of::<LZ4_stream_t_internal>() as size_t,
    );
}
#[inline]
pub unsafe fn LZ4_resetStream_fast(mut ctx: *mut LZ4_stream_t) {
    let ctx_view: &mut LZ4_stream_t = unsafe { &mut *ctx };
    LZ4_prepareTable(
        &raw mut ctx_view.internal_donotuse,
        0 as c_int,
        byU32,
    );
}
#[inline]
pub unsafe fn LZ4_freeStream(mut LZ4_stream: *mut LZ4_stream_t) -> c_int {
    if LZ4_stream.is_null() {
        return 0 as c_int;
    }
    free(LZ4_stream as *mut c_void);
    return 0 as c_int;
}
pub const HASH_UNIT: usize = ::core::mem::size_of::<reg_t>();
unsafe fn LZ4_loadDict_internal(
    mut LZ4_dict: *mut LZ4_stream_t,
    mut dictionary: *const c_char,
    mut dictSize: c_int,
    mut _ld: LoadDict_mode_e,
) -> c_int {
    let dict: *mut LZ4_stream_t_internal = &raw mut (*LZ4_dict).internal_donotuse;
    let tableType: tableType_t = byU32;
    let mut p: *const BYTE = dictionary as *const BYTE;
    let dictEnd: *const BYTE = p.offset(dictSize as isize);
    let mut idx32: U32 = 0;
    LZ4_resetStream(LZ4_dict);
    (*dict).currentOffset = (*dict).currentOffset.wrapping_add(
        (64 as c_int * ((1 as c_int) << 10 as c_int))
            as LZ4_u32,
    );
    if dictSize < HASH_UNIT as c_int {
        return 0 as c_int;
    }
    if dictEnd.offset_from(p) as c_long
        > (64 as c_int * ((1 as c_int) << 10 as c_int))
            as c_long
    {
        p = dictEnd.offset(
            -((64 as c_int * ((1 as c_int) << 10 as c_int))
                as isize),
        );
    }
    (*dict).dictionary = p as *const LZ4_byte;
    (*dict).dictSize = dictEnd.offset_from(p) as c_long as U32 as LZ4_u32;
    (*dict).tableType = tableType as U32 as LZ4_u32;
    idx32 = (*dict).currentOffset.wrapping_sub((*dict).dictSize) as U32;
    while p <= dictEnd.offset(-(HASH_UNIT as isize)) {
        let h: U32 = LZ4_hashPosition(p as *const c_void, tableType) as U32;
        LZ4_putIndexOnHash(
            idx32,
            h,
            &raw mut (*dict).hashTable as *mut LZ4_u32 as *mut c_void,
            tableType,
        );
        p = p.offset(3 as c_int as isize);
        idx32 = idx32.wrapping_add(3 as U32);
    }
    if _ld as c_uint == _ld_slow as c_int as c_uint {
        p = (*dict).dictionary as *const BYTE;
        idx32 = (*dict).currentOffset.wrapping_sub((*dict).dictSize) as U32;
        while p <= dictEnd.offset(-(HASH_UNIT as isize)) {
            let h_0: U32 = LZ4_hashPosition(p as *const c_void, tableType) as U32;
            let limit: U32 = ((*dict).currentOffset as U32).wrapping_sub(
                (64 as c_int * ((1 as c_int) << 10 as c_int))
                    as U32,
            );
            if LZ4_getIndexOnHash(
                h_0,
                &raw mut (*dict).hashTable as *mut LZ4_u32 as *const c_void,
                tableType,
            ) <= limit
            {
                LZ4_putIndexOnHash(
                    idx32,
                    h_0,
                    &raw mut (*dict).hashTable as *mut LZ4_u32 as *mut c_void,
                    tableType,
                );
            }
            p = p.offset(1);
            idx32 = idx32.wrapping_add(1);
        }
    }
    return (*dict).dictSize as c_int;
}
#[inline]
pub unsafe fn LZ4_loadDict(
    mut LZ4_dict: *mut LZ4_stream_t,
    mut dictionary: *const c_char,
    mut dictSize: c_int,
) -> c_int {
    return LZ4_loadDict_internal(LZ4_dict, dictionary, dictSize, _ld_fast);
}
#[inline]
pub unsafe fn LZ4_loadDictSlow(
    mut LZ4_dict: *mut LZ4_stream_t,
    mut dictionary: *const c_char,
    mut dictSize: c_int,
) -> c_int {
    return LZ4_loadDict_internal(LZ4_dict, dictionary, dictSize, _ld_slow);
}
#[inline]
pub unsafe fn LZ4_attach_dictionary(
    mut workingStream: *mut LZ4_stream_t,
    mut dictionaryStream: *const LZ4_stream_t,
) {
    let mut dictCtx: *const LZ4_stream_t_internal = if dictionaryStream.is_null() {
        ::core::ptr::null::<LZ4_stream_t_internal>()
    } else {
        &raw const (*dictionaryStream).internal_donotuse
    };
    if !dictCtx.is_null() {
        if (*workingStream).internal_donotuse.currentOffset == 0 as LZ4_u32 {
            (*workingStream).internal_donotuse.currentOffset = (64 as c_int
                * ((1 as c_int) << 10 as c_int))
                as LZ4_u32;
        }
        if (*dictCtx).dictSize == 0 as LZ4_u32 {
            dictCtx = ::core::ptr::null::<LZ4_stream_t_internal>();
        }
    }
    (*workingStream).internal_donotuse.dictCtx = dictCtx;
}
unsafe fn LZ4_renormDictT(
    mut LZ4_dict: *mut LZ4_stream_t_internal,
    mut nextSize: c_int,
) {
    let LZ4_dict_view: &mut LZ4_stream_t_internal = unsafe { &mut *LZ4_dict };
    if LZ4_dict_view.currentOffset.wrapping_add(nextSize as LZ4_u32) > 0x80000000 as LZ4_u32 {
        let delta: U32 = (LZ4_dict_view.currentOffset as U32).wrapping_sub(
            (64 as c_int * ((1 as c_int) << 10 as c_int))
                as U32,
        );
        let mut dictEnd: *const BYTE = LZ4_dict_view.dictionary.offset(LZ4_dict_view.dictSize as isize);
        let mut i: c_int = 0;
        i = 0 as c_int;
        while i < LZ4_HASH_SIZE_U32 {
            if LZ4_dict_view.hashTable[i as usize] < delta {
                LZ4_dict_view.hashTable[i as usize] = 0 as LZ4_u32;
            } else {
                LZ4_dict_view.hashTable[i as usize] = (LZ4_dict_view.hashTable[i as usize] as uint32_t)
                    .wrapping_sub(delta as uint32_t)
                    as LZ4_u32 as LZ4_u32;
            }
            i += 1;
        }
        LZ4_dict_view.currentOffset = (64 as c_int
            * ((1 as c_int) << 10 as c_int))
            as LZ4_u32;
        if LZ4_dict_view.dictSize
            > (64 as c_int * ((1 as c_int) << 10 as c_int))
                as LZ4_u32
        {
            LZ4_dict_view.dictSize = (64 as c_int
                * ((1 as c_int) << 10 as c_int))
                as LZ4_u32;
        }
        LZ4_dict_view.dictionary =
            dictEnd.offset(-(LZ4_dict_view.dictSize as isize)) as *const LZ4_byte;
    }
}
#[inline]
pub unsafe fn LZ4_compress_fast_continue(
    mut LZ4_stream: *mut LZ4_stream_t,
    mut source: *const c_char,
    mut dest: *mut c_char,
    mut inputSize: c_int,
    mut maxOutputSize: c_int,
    mut acceleration: c_int,
) -> c_int {
    let tableType: tableType_t = byU32;
    let streamPtr: *mut LZ4_stream_t_internal = &raw mut (*LZ4_stream).internal_donotuse;
    let mut dictEnd: *const c_char = if (*streamPtr).dictSize != 0 {
        ((*streamPtr).dictionary as *const c_char)
            .offset((*streamPtr).dictSize as isize)
    } else {
        ::core::ptr::null::<c_char>()
    };
    LZ4_renormDictT(streamPtr, inputSize);
    if acceleration < 1 as c_int {
        acceleration = LZ4_ACCELERATION_DEFAULT;
    }
    if acceleration > LZ4_ACCELERATION_MAX {
        acceleration = LZ4_ACCELERATION_MAX;
    }
    if (*streamPtr).dictSize < 4 as LZ4_u32
        && dictEnd != source
        && inputSize > 0 as c_int
        && (*streamPtr).dictCtx.is_null()
    {
        (*streamPtr).dictSize = 0 as LZ4_u32;
        (*streamPtr).dictionary = source as *const BYTE as *const LZ4_byte;
        dictEnd = source;
    }
    let sourceEnd: *const c_char = source.offset(inputSize as isize);
    if sourceEnd > (*streamPtr).dictionary as *const c_char && sourceEnd < dictEnd {
        (*streamPtr).dictSize =
            dictEnd.offset_from(sourceEnd) as c_long as U32 as LZ4_u32;
        if (*streamPtr).dictSize
            > (64 as c_int * ((1 as c_int) << 10 as c_int))
                as LZ4_u32
        {
            (*streamPtr).dictSize = (64 as c_int
                * ((1 as c_int) << 10 as c_int))
                as LZ4_u32;
        }
        if (*streamPtr).dictSize < 4 as LZ4_u32 {
            (*streamPtr).dictSize = 0 as LZ4_u32;
        }
        (*streamPtr).dictionary =
            (dictEnd as *const BYTE).offset(-((*streamPtr).dictSize as isize)) as *const LZ4_byte;
    }
    if dictEnd == source {
        if (*streamPtr).dictSize
            < (64 as c_int * ((1 as c_int) << 10 as c_int))
                as LZ4_u32
            && (*streamPtr).dictSize < (*streamPtr).currentOffset
        {
            return LZ4_compress_generic(
                streamPtr,
                source,
                dest,
                inputSize,
                ::core::ptr::null_mut::<c_int>(),
                maxOutputSize,
                limitedOutput,
                tableType,
                withPrefix64k,
                dictSmall,
                acceleration,
            );
        } else {
            return LZ4_compress_generic(
                streamPtr,
                source,
                dest,
                inputSize,
                ::core::ptr::null_mut::<c_int>(),
                maxOutputSize,
                limitedOutput,
                tableType,
                withPrefix64k,
                noDictIssue,
                acceleration,
            );
        }
    }
    let mut result: c_int = 0;
    if !(*streamPtr).dictCtx.is_null() {
        if inputSize
            > 4 as c_int * ((1 as c_int) << 10 as c_int)
        {
            ::libc::memcpy(
                streamPtr as *mut c_void,
                (*streamPtr).dictCtx as *const c_void,
                ::core::mem::size_of::<LZ4_stream_t_internal>() as ::libc::size_t,
            );
            result = LZ4_compress_generic(
                streamPtr,
                source,
                dest,
                inputSize,
                ::core::ptr::null_mut::<c_int>(),
                maxOutputSize,
                limitedOutput,
                tableType,
                usingExtDict,
                noDictIssue,
                acceleration,
            );
        } else {
            result = LZ4_compress_generic(
                streamPtr,
                source,
                dest,
                inputSize,
                ::core::ptr::null_mut::<c_int>(),
                maxOutputSize,
                limitedOutput,
                tableType,
                usingDictCtx,
                noDictIssue,
                acceleration,
            );
        }
    } else if (*streamPtr).dictSize
        < (64 as c_int * ((1 as c_int) << 10 as c_int))
            as LZ4_u32
        && (*streamPtr).dictSize < (*streamPtr).currentOffset
    {
        result = LZ4_compress_generic(
            streamPtr,
            source,
            dest,
            inputSize,
            ::core::ptr::null_mut::<c_int>(),
            maxOutputSize,
            limitedOutput,
            tableType,
            usingExtDict,
            dictSmall,
            acceleration,
        );
    } else {
        result = LZ4_compress_generic(
            streamPtr,
            source,
            dest,
            inputSize,
            ::core::ptr::null_mut::<c_int>(),
            maxOutputSize,
            limitedOutput,
            tableType,
            usingExtDict,
            noDictIssue,
            acceleration,
        );
    }
    (*streamPtr).dictionary = source as *const BYTE as *const LZ4_byte;
    (*streamPtr).dictSize = inputSize as U32 as LZ4_u32;
    return result;
}
#[inline]
pub unsafe fn LZ4_compress_forceExtDict(
    mut LZ4_dict: *mut LZ4_stream_t,
    mut source: *const c_char,
    mut dest: *mut c_char,
    mut srcSize: c_int,
) -> c_int {
    let streamPtr: *mut LZ4_stream_t_internal = &raw mut (*LZ4_dict).internal_donotuse;
    let mut result: c_int = 0;
    LZ4_renormDictT(streamPtr, srcSize);
    if (*streamPtr).dictSize
        < (64 as c_int * ((1 as c_int) << 10 as c_int))
            as LZ4_u32
        && (*streamPtr).dictSize < (*streamPtr).currentOffset
    {
        result = LZ4_compress_generic(
            streamPtr,
            source,
            dest,
            srcSize,
            ::core::ptr::null_mut::<c_int>(),
            0 as c_int,
            notLimited,
            byU32,
            usingExtDict,
            dictSmall,
            1 as c_int,
        );
    } else {
        result = LZ4_compress_generic(
            streamPtr,
            source,
            dest,
            srcSize,
            ::core::ptr::null_mut::<c_int>(),
            0 as c_int,
            notLimited,
            byU32,
            usingExtDict,
            noDictIssue,
            1 as c_int,
        );
    }
    (*streamPtr).dictionary = source as *const BYTE as *const LZ4_byte;
    (*streamPtr).dictSize = srcSize as U32 as LZ4_u32;
    return result;
}
#[inline]
pub unsafe fn LZ4_saveDict(
    mut LZ4_dict: *mut LZ4_stream_t,
    mut safeBuffer: *mut c_char,
    mut dictSize: c_int,
) -> c_int {
    let dict: *mut LZ4_stream_t_internal = &raw mut (*LZ4_dict).internal_donotuse;
    if dictSize as U32
        > (64 as c_int * ((1 as c_int) << 10 as c_int))
            as U32
    {
        dictSize =
            64 as c_int * ((1 as c_int) << 10 as c_int);
    }
    if dictSize as U32 > (*dict).dictSize {
        dictSize = (*dict).dictSize as c_int;
    }
    safeBuffer.is_null();
    if dictSize > 0 as c_int {
        let previousDictEnd: *const BYTE = (*dict).dictionary.offset((*dict).dictSize as isize);
        ::libc::memmove(
            safeBuffer as *mut c_void,
            previousDictEnd.offset(-(dictSize as isize)) as *const c_void,
            dictSize as size_t as ::libc::size_t,
        );
    }
    (*dict).dictionary = safeBuffer as *const BYTE as *const LZ4_byte;
    (*dict).dictSize = dictSize as U32 as LZ4_u32;
    return dictSize;
}
unsafe fn read_long_length_no_check(mut pp: *mut *const BYTE) -> size_t {
    let mut b: size_t = 0;
    let mut l: size_t = 0 as size_t;
    loop {
        b = **pp as size_t;
        *pp = (*pp).offset(1);
        l = l.wrapping_add(b);
        if !(b == 255 as size_t) {
            break;
        }
    }
    return l;
}
#[inline(always)]
unsafe fn LZ4_decompress_unsafe_generic(
    istart: *const BYTE,
    ostart: *mut BYTE,
    mut decompressedSize: c_int,
    mut prefixSize: size_t,
    dictStart: *const BYTE,
    dictSize: size_t,
) -> c_int {
    let mut ip: *const BYTE = istart;
    let mut op: *mut BYTE = ostart;
    let oend: *mut BYTE = ostart.offset(decompressedSize as isize);
    let prefixStart: *const BYTE = ostart.offset(-(prefixSize as isize));
    dictStart.is_null();
    loop {
        let fresh24 = ip;
        ip = ip.offset(1);
        let mut token: c_uint = *fresh24 as c_uint;
        let mut ll: size_t = (token >> ML_BITS) as size_t;
        if ll == 15 as size_t {
            ll = ll.wrapping_add(read_long_length_no_check(&raw mut ip));
        }
        if (oend.offset_from(op) as c_long as size_t) < ll {
            return -(1 as c_int);
        }
        ::libc::memmove(
            op as *mut c_void,
            ip as *const c_void,
            ll as ::libc::size_t,
        );
        op = op.offset(ll as isize);
        ip = ip.offset(ll as isize);
        if (oend.offset_from(op) as c_long as size_t) < MFLIMIT as size_t {
            if op == oend {
                break;
            }
            return -(1 as c_int);
        } else {
            let mut ml: size_t = (token & 15 as c_uint) as size_t;
            let offset: size_t = LZ4_readLE16(ip as *const c_void) as size_t;
            ip = ip.offset(2 as c_int as isize);
            if ml == 15 as size_t {
                ml = ml.wrapping_add(read_long_length_no_check(&raw mut ip));
            }
            ml = ml.wrapping_add(MINMATCH as size_t);
            if (oend.offset_from(op) as c_long as size_t) < ml {
                return -(1 as c_int);
            }
            let mut match_0: *const BYTE = op.offset(-(offset as isize));
            if offset
                > (op.offset_from(prefixStart) as c_long as size_t)
                    .wrapping_add(dictSize)
            {
                return -(1 as c_int);
            }
            if offset > op.offset_from(prefixStart) as c_long as size_t {
                let dictEnd: *const BYTE = dictStart.offset(dictSize as isize);
                let mut extMatch: *const BYTE = dictEnd.offset(
                    -(offset
                        .wrapping_sub(op.offset_from(prefixStart) as c_long as size_t)
                        as isize),
                );
                let extml: size_t = dictEnd.offset_from(extMatch) as c_long as size_t;
                if extml > ml {
                    ::libc::memmove(
                        op as *mut c_void,
                        extMatch as *const c_void,
                        ml as ::libc::size_t,
                    );
                    op = op.offset(ml as isize);
                    ml = 0 as size_t;
                } else {
                    ::libc::memmove(
                        op as *mut c_void,
                        extMatch as *const c_void,
                        extml as ::libc::size_t,
                    );
                    op = op.offset(extml as isize);
                    ml = ml.wrapping_sub(extml);
                }
                match_0 = prefixStart;
            }
            let mut u: size_t = 0;
            u = 0 as size_t;
            while u < ml {
                *op.offset(u as isize) = *match_0.offset(u as isize);
                u = u.wrapping_add(1);
            }
            op = op.offset(ml as isize);
            if (oend.offset_from(op) as c_long as size_t) < LASTLITERALS as size_t {
                return -(1 as c_int);
            }
        }
    }
    return ip.offset_from(istart) as c_long as c_int;
}
static mut rvl_error: Rvl_t = -(1 as c_int) as Rvl_t;
#[inline(always)]
unsafe fn read_variable_length(
    mut ipPtr: *mut *const BYTE,
    mut ilimit: *const BYTE,
) -> Rvl_t {
    let mut s: Rvl_t = 0;
    let mut length: Rvl_t = 0 as Rvl_t;
    if ((*ipPtr >= ilimit) as c_int != 0 as c_int) as c_int
        as c_long
        != 0
    {
        return rvl_error;
    }
    s = **ipPtr as Rvl_t;
    *ipPtr = (*ipPtr).offset(1);
    length = length.wrapping_add(s);
    if ((s != 255 as Rvl_t) as c_int != 0 as c_int) as c_int
        as c_long
        != 0
    {
        return length;
    }
    loop {
        if ((*ipPtr >= ilimit) as c_int != 0 as c_int)
            as c_int as c_long
            != 0
        {
            return rvl_error;
        }
        s = **ipPtr as Rvl_t;
        *ipPtr = (*ipPtr).offset(1);
        length = length.wrapping_add(s);
        if (::core::mem::size_of::<Rvl_t>() as usize) < 8 as usize
            && ((length > (-(1 as c_int) as Rvl_t).wrapping_div(2 as Rvl_t))
                as c_int
                != 0 as c_int) as c_int
                as c_long
                != 0
        {
            return rvl_error;
        }
        if !(s == 255 as Rvl_t) {
            break;
        }
    }
    return length;
}
#[inline(always)]
unsafe fn LZ4_decompress_generic(
    src: *const c_char,
    dst: *mut c_char,
    mut srcSize: c_int,
    mut outputSize: c_int,
    mut partialDecoding: earlyEnd_directive,
    mut dict: dict_directive,
    lowPrefix: *const BYTE,
    dictStart: *const BYTE,
    dictSize: size_t,
) -> c_int {
    let mut current_block: u64;
    if src.is_null() || outputSize < 0 as c_int {
        return -(1 as c_int);
    }
    let mut ip: *const BYTE = src as *const BYTE;
    let iend: *const BYTE = ip.offset(srcSize as isize);
    let mut op: *mut BYTE = dst as *mut BYTE;
    let oend: *mut BYTE = op.offset(outputSize as isize);
    let mut cpy: *mut BYTE = ::core::ptr::null_mut::<BYTE>();
    let dictEnd: *const BYTE = if dictStart.is_null() {
        ::core::ptr::null::<BYTE>()
    } else {
        dictStart.offset(dictSize as isize)
    };
    let checkOffset: c_int = (dictSize
        < (64 as c_int * ((1 as c_int) << 10 as c_int))
            as size_t) as c_int;
    let shortiend: *const BYTE = iend
        .offset(-(14 as c_int as isize))
        .offset(-(2 as c_int as isize));
    let shortoend: *const BYTE = oend
        .offset(-(14 as c_int as isize))
        .offset(-(18 as c_int as isize));
    let mut match_0: *const BYTE = ::core::ptr::null::<BYTE>();
    let mut offset: size_t = 0;
    let mut token: c_uint = 0;
    let mut length: size_t = 0;
    if ((outputSize == 0 as c_int) as c_int != 0 as c_int)
        as c_int as c_long
        != 0
    {
        if partialDecoding as u64 != 0 {
            return 0 as c_int;
        }
        return if srcSize == 1 as c_int
            && *ip as c_int == 0 as c_int
        {
            0 as c_int
        } else {
            -(1 as c_int)
        };
    }
    if ((srcSize == 0 as c_int) as c_int != 0 as c_int)
        as c_int as c_long
        != 0
    {
        return -(1 as c_int);
    }
    if (oend.offset_from(op) as c_long) < FASTLOOP_SAFE_DISTANCE as c_long
    {
        current_block = 11995618668192240200;
    } else {
        current_block = 12147880666119273379;
    }
    loop {
        match current_block {
            12147880666119273379 => {
                let fresh14 = ip;
                ip = ip.offset(1);
                token = *fresh14 as c_uint;
                length = (token >> ML_BITS) as size_t;
                if ip
                    > iend.offset(-((16 as c_int + 1 as c_int) as isize))
                {
                    current_block = 15407617374229148015;
                } else {
                    if length == RUN_MASK as size_t {
                        let addl: size_t =
                            read_variable_length(&raw mut ip, iend.offset(-(RUN_MASK as isize)))
                                as size_t;
                        if addl == rvl_error {
                            current_block = 14897191503944545335;
                            break;
                        }
                        length = length.wrapping_add(addl);
                        cpy = op.offset(length as isize);
                        if (((cpy as uptrval) < op as uptrval) as c_int
                            != 0 as c_int)
                            as c_int as c_long
                            != 0
                        {
                            current_block = 14897191503944545335;
                            break;
                        }
                        if (((ip as uptrval).wrapping_add(length as uptrval) < ip as uptrval)
                            as c_int
                            != 0 as c_int)
                            as c_int as c_long
                            != 0
                        {
                            current_block = 14897191503944545335;
                            break;
                        }
                        if cpy > oend.offset(-(32 as c_int as isize))
                            || ip.offset(length as isize)
                                > iend.offset(-(32 as c_int as isize))
                        {
                            current_block = 3567897568976182940;
                        } else {
                            LZ4_wildCopy32(
                                op as *mut c_void,
                                ip as *const c_void,
                                cpy as *mut c_void,
                            );
                            ip = ip.offset(length as isize);
                            op = cpy;
                            current_block = 15512526488502093901;
                        }
                    } else {
                        ::libc::memcpy(
                            op as *mut c_void,
                            ip as *const c_void,
                            16 as c_int as c_ulong as ::libc::size_t,
                        );
                        ip = ip.offset(length as isize);
                        op = op.offset(length as isize);
                        current_block = 15512526488502093901;
                    }
                    match current_block {
                        3567897568976182940 => {}
                        _ => {
                            offset = LZ4_readLE16(ip as *const c_void) as size_t;
                            ip = ip.offset(2 as c_int as isize);
                            match_0 = op.offset(-(offset as isize));
                            length = (token & ML_MASK) as size_t;
                            if length == ML_MASK as size_t {
                                let addl_0: size_t = read_variable_length(
                                    &raw mut ip,
                                    iend.offset(
                                        -((1 as c_int + LASTLITERALS) as isize),
                                    ),
                                ) as size_t;
                                if addl_0 == rvl_error {
                                    current_block = 14897191503944545335;
                                    break;
                                }
                                length = length.wrapping_add(addl_0);
                                length = length.wrapping_add(MINMATCH as size_t);
                                if (((op as uptrval).wrapping_add(length as uptrval)
                                    < op as uptrval)
                                    as c_int
                                    != 0 as c_int)
                                    as c_int
                                    as c_long
                                    != 0
                                {
                                    current_block = 14897191503944545335;
                                    break;
                                }
                                if op.offset(length as isize)
                                    >= oend.offset(-(FASTLOOP_SAFE_DISTANCE as isize))
                                {
                                    current_block = 14802982971050443675;
                                } else {
                                    current_block = 2606304779496145856;
                                }
                            } else {
                                length = length.wrapping_add(MINMATCH as size_t);
                                if op.offset(length as isize)
                                    >= oend.offset(-(FASTLOOP_SAFE_DISTANCE as isize))
                                {
                                    current_block = 14802982971050443675;
                                } else if dict as c_uint
                                    == withPrefix64k as c_int as c_uint
                                    || match_0 >= lowPrefix
                                {
                                    if offset >= 8 as size_t {
                                        ::libc::memcpy(
                                            op as *mut c_void,
                                            match_0 as *const c_void,
                                            8 as c_int as c_ulong
                                                as ::libc::size_t,
                                        );
                                        ::libc::memcpy(
                                            op.offset(8 as c_int as isize)
                                                as *mut c_void,
                                            match_0.offset(8 as c_int as isize)
                                                as *const c_void,
                                            8 as c_int as c_ulong
                                                as ::libc::size_t,
                                        );
                                        ::libc::memcpy(
                                            op.offset(16 as c_int as isize)
                                                as *mut c_void,
                                            match_0.offset(16 as c_int as isize)
                                                as *const c_void,
                                            2 as c_int as c_ulong
                                                as ::libc::size_t,
                                        );
                                        op = op.offset(length as isize);
                                        current_block = 12147880666119273379;
                                        continue;
                                    } else {
                                        current_block = 2606304779496145856;
                                    }
                                } else {
                                    current_block = 2606304779496145856;
                                }
                            }
                            match current_block {
                                14802982971050443675 => {}
                                _ => {
                                    if checkOffset != 0
                                        && ((match_0.offset(dictSize as isize) < lowPrefix)
                                            as c_int
                                            != 0 as c_int)
                                            as c_int
                                            as c_long
                                            != 0
                                    {
                                        current_block = 14897191503944545335;
                                        break;
                                    }
                                    if dict as c_uint
                                        == usingExtDict as c_int as c_uint
                                        && match_0 < lowPrefix
                                    {
                                        if ((op.offset(length as isize)
                                            > oend.offset(-(5 as c_int as isize)))
                                            as c_int
                                            != 0 as c_int)
                                            as c_int
                                            as c_long
                                            != 0
                                        {
                                            if !(partialDecoding as u64 != 0) {
                                                current_block = 14897191503944545335;
                                                break;
                                            }
                                            length = if length
                                                < oend.offset_from(op) as c_long
                                                    as size_t
                                            {
                                                length
                                            } else {
                                                oend.offset_from(op) as c_long
                                                    as size_t
                                            };
                                        }
                                        if length
                                            <= lowPrefix.offset_from(match_0) as c_long
                                                as size_t
                                        {
                                            ::libc::memmove(
                                                op as *mut c_void,
                                                dictEnd.offset(
                                                    -(lowPrefix.offset_from(match_0)
                                                        as c_long
                                                        as isize),
                                                )
                                                    as *const c_void,
                                                length as ::libc::size_t,
                                            );
                                            op = op.offset(length as isize);
                                        } else {
                                            let copySize: size_t = lowPrefix.offset_from(match_0)
                                                as c_long
                                                as size_t;
                                            let restSize: size_t = length.wrapping_sub(copySize);
                                            ::libc::memcpy(
                                                op as *mut c_void,
                                                dictEnd.offset(-(copySize as isize))
                                                    as *const c_void,
                                                copySize as ::libc::size_t,
                                            );
                                            op = op.offset(copySize as isize);
                                            if restSize
                                                > op.offset_from(lowPrefix) as c_long
                                                    as size_t
                                            {
                                                let endOfMatch: *mut BYTE =
                                                    op.offset(restSize as isize);
                                                let mut copyFrom: *const BYTE = lowPrefix;
                                                while op < endOfMatch {
                                                    let fresh15 = copyFrom;
                                                    copyFrom = copyFrom.offset(1);
                                                    let fresh16 = op;
                                                    op = op.offset(1);
                                                    *fresh16 = *fresh15;
                                                }
                                            } else {
                                                ::libc::memcpy(
                                                    op as *mut c_void,
                                                    lowPrefix as *const c_void,
                                                    restSize as ::libc::size_t,
                                                );
                                                op = op.offset(restSize as isize);
                                            }
                                        }
                                        current_block = 12147880666119273379;
                                        continue;
                                    } else {
                                        cpy = op.offset(length as isize);
                                        if ((offset < 16 as size_t) as c_int
                                            != 0 as c_int)
                                            as c_int
                                            as c_long
                                            != 0
                                        {
                                            LZ4_memcpy_using_offset(op, match_0, cpy, offset);
                                        } else {
                                            LZ4_wildCopy32(
                                                op as *mut c_void,
                                                match_0 as *const c_void,
                                                cpy as *mut c_void,
                                            );
                                        }
                                        op = cpy;
                                        current_block = 12147880666119273379;
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
            _ => {
                let fresh17 = ip;
                ip = ip.offset(1);
                token = *fresh17 as c_uint;
                length = (token >> ML_BITS) as size_t;
                if length != RUN_MASK as size_t
                    && ((ip < shortiend) as c_int
                        & (op <= shortoend as *mut BYTE) as c_int
                        != 0 as c_int) as c_int
                        as c_long
                        != 0
                {
                    ::libc::memcpy(
                        op as *mut c_void,
                        ip as *const c_void,
                        16 as c_int as c_ulong as ::libc::size_t,
                    );
                    op = op.offset(length as isize);
                    ip = ip.offset(length as isize);
                    length = (token & ML_MASK) as size_t;
                    offset = LZ4_readLE16(ip as *const c_void) as size_t;
                    ip = ip.offset(2 as c_int as isize);
                    match_0 = op.offset(-(offset as isize));
                    if length != ML_MASK as size_t
                        && offset >= 8 as size_t
                        && (dict as c_uint
                            == withPrefix64k as c_int as c_uint
                            || match_0 >= lowPrefix)
                    {
                        ::libc::memcpy(
                            op.offset(0 as c_int as isize) as *mut c_void,
                            match_0.offset(0 as c_int as isize)
                                as *const c_void,
                            8 as c_int as c_ulong as ::libc::size_t,
                        );
                        ::libc::memcpy(
                            op.offset(8 as c_int as isize) as *mut c_void,
                            match_0.offset(8 as c_int as isize)
                                as *const c_void,
                            8 as c_int as c_ulong as ::libc::size_t,
                        );
                        ::libc::memcpy(
                            op.offset(16 as c_int as isize)
                                as *mut c_void,
                            match_0.offset(16 as c_int as isize)
                                as *const c_void,
                            2 as c_int as c_ulong as ::libc::size_t,
                        );
                        op = op.offset(length.wrapping_add(MINMATCH as size_t) as isize);
                        current_block = 11995618668192240200;
                        continue;
                    } else {
                        current_block = 6931166572251973511;
                    }
                } else {
                    current_block = 15407617374229148015;
                }
            }
        }
        match current_block {
            15407617374229148015 => {
                if length == RUN_MASK as size_t {
                    let addl_1: size_t =
                        read_variable_length(&raw mut ip, iend.offset(-(RUN_MASK as isize)))
                            as size_t;
                    if addl_1 == rvl_error {
                        current_block = 14897191503944545335;
                        break;
                    }
                    length = length.wrapping_add(addl_1);
                    if (((op as uptrval).wrapping_add(length as uptrval) < op as uptrval)
                        as c_int
                        != 0 as c_int) as c_int
                        as c_long
                        != 0
                    {
                        current_block = 14897191503944545335;
                        break;
                    }
                    if (((ip as uptrval).wrapping_add(length as uptrval) < ip as uptrval)
                        as c_int
                        != 0 as c_int) as c_int
                        as c_long
                        != 0
                    {
                        current_block = 14897191503944545335;
                        break;
                    }
                }
                cpy = op.offset(length as isize);
                current_block = 3567897568976182940;
            }
            _ => {}
        }
        match current_block {
            3567897568976182940 => {
                if cpy > oend.offset(-(MFLIMIT as isize))
                    || ip.offset(length as isize)
                        > iend.offset(
                            -((2 as c_int + 1 as c_int + LASTLITERALS)
                                as isize),
                        )
                {
                    if partialDecoding as u64 != 0 {
                        if ip.offset(length as isize) > iend {
                            length = iend.offset_from(ip) as c_long as size_t;
                            cpy = op.offset(length as isize);
                        }
                        if cpy > oend {
                            cpy = oend;
                            length = oend.offset_from(op) as c_long as size_t;
                        }
                    } else if ip.offset(length as isize) != iend || cpy > oend {
                        current_block = 14897191503944545335;
                        break;
                    }
                    ::libc::memmove(
                        op as *mut c_void,
                        ip as *const c_void,
                        length as ::libc::size_t,
                    );
                    ip = ip.offset(length as isize);
                    op = op.offset(length as isize);
                    if partialDecoding as u64 == 0
                        || cpy == oend
                        || ip >= iend.offset(-(2 as c_int as isize))
                    {
                        current_block = 8038949400865391589;
                        break;
                    }
                } else {
                    LZ4_wildCopy8(
                        op as *mut c_void,
                        ip as *const c_void,
                        cpy as *mut c_void,
                    );
                    ip = ip.offset(length as isize);
                    op = cpy;
                }
                offset = LZ4_readLE16(ip as *const c_void) as size_t;
                ip = ip.offset(2 as c_int as isize);
                match_0 = op.offset(-(offset as isize));
                length = (token & ML_MASK) as size_t;
                current_block = 6931166572251973511;
            }
            _ => {}
        }
        match current_block {
            6931166572251973511 => {
                if length == ML_MASK as size_t {
                    let addl_2: size_t = read_variable_length(
                        &raw mut ip,
                        iend.offset(-((1 as c_int + LASTLITERALS) as isize)),
                    ) as size_t;
                    if addl_2 == rvl_error {
                        current_block = 14897191503944545335;
                        break;
                    }
                    length = length.wrapping_add(addl_2);
                    if (((op as uptrval).wrapping_add(length as uptrval) < op as uptrval)
                        as c_int
                        != 0 as c_int) as c_int
                        as c_long
                        != 0
                    {
                        current_block = 14897191503944545335;
                        break;
                    }
                }
                length = length.wrapping_add(MINMATCH as size_t);
            }
            _ => {}
        }
        if checkOffset != 0
            && ((match_0.offset(dictSize as isize) < lowPrefix) as c_int
                != 0 as c_int) as c_int
                as c_long
                != 0
        {
            current_block = 14897191503944545335;
            break;
        }
        if dict as c_uint == usingExtDict as c_int as c_uint
            && match_0 < lowPrefix
        {
            if ((op.offset(length as isize) > oend.offset(-(5 as c_int as isize)))
                as c_int
                != 0 as c_int) as c_int
                as c_long
                != 0
            {
                if !(partialDecoding as u64 != 0) {
                    current_block = 14897191503944545335;
                    break;
                }
                length = if length < oend.offset_from(op) as c_long as size_t {
                    length
                } else {
                    oend.offset_from(op) as c_long as size_t
                };
            }
            if length <= lowPrefix.offset_from(match_0) as c_long as size_t {
                ::libc::memmove(
                    op as *mut c_void,
                    dictEnd
                        .offset(-(lowPrefix.offset_from(match_0) as c_long as isize))
                        as *const c_void,
                    length as ::libc::size_t,
                );
                op = op.offset(length as isize);
            } else {
                let copySize_0: size_t =
                    lowPrefix.offset_from(match_0) as c_long as size_t;
                let restSize_0: size_t = length.wrapping_sub(copySize_0);
                ::libc::memcpy(
                    op as *mut c_void,
                    dictEnd.offset(-(copySize_0 as isize)) as *const c_void,
                    copySize_0 as ::libc::size_t,
                );
                op = op.offset(copySize_0 as isize);
                if restSize_0 > op.offset_from(lowPrefix) as c_long as size_t {
                    let endOfMatch_0: *mut BYTE = op.offset(restSize_0 as isize);
                    let mut copyFrom_0: *const BYTE = lowPrefix;
                    while op < endOfMatch_0 {
                        let fresh18 = copyFrom_0;
                        copyFrom_0 = copyFrom_0.offset(1);
                        let fresh19 = op;
                        op = op.offset(1);
                        *fresh19 = *fresh18;
                    }
                } else {
                    ::libc::memcpy(
                        op as *mut c_void,
                        lowPrefix as *const c_void,
                        restSize_0 as ::libc::size_t,
                    );
                    op = op.offset(restSize_0 as isize);
                }
            }
            current_block = 11995618668192240200;
        } else {
            cpy = op.offset(length as isize);
            if partialDecoding as c_uint != 0
                && cpy > oend.offset(-(MATCH_SAFEGUARD_DISTANCE as isize))
            {
                let mlen: size_t = if length < oend.offset_from(op) as c_long as size_t
                {
                    length
                } else {
                    oend.offset_from(op) as c_long as size_t
                };
                let matchEnd: *const BYTE = match_0.offset(mlen as isize);
                let copyEnd: *mut BYTE = op.offset(mlen as isize);
                if matchEnd > op as *const BYTE {
                    while op < copyEnd {
                        let fresh20 = match_0;
                        match_0 = match_0.offset(1);
                        let fresh21 = op;
                        op = op.offset(1);
                        *fresh21 = *fresh20;
                    }
                } else {
                    ::libc::memcpy(
                        op as *mut c_void,
                        match_0 as *const c_void,
                        mlen as ::libc::size_t,
                    );
                }
                op = copyEnd;
                if op == oend {
                    current_block = 8038949400865391589;
                    break;
                } else {
                    current_block = 11995618668192240200;
                }
            } else {
                if ((offset < 8 as size_t) as c_int != 0 as c_int)
                    as c_int as c_long
                    != 0
                {
                    LZ4_write32(op as *mut c_void, 0 as U32);
                    *op.offset(0 as c_int as isize) =
                        *match_0.offset(0 as c_int as isize);
                    *op.offset(1 as c_int as isize) =
                        *match_0.offset(1 as c_int as isize);
                    *op.offset(2 as c_int as isize) =
                        *match_0.offset(2 as c_int as isize);
                    *op.offset(3 as c_int as isize) =
                        *match_0.offset(3 as c_int as isize);
                    match_0 = match_0.offset(inc32table[offset as usize] as isize);
                    ::libc::memcpy(
                        op.offset(4 as c_int as isize) as *mut c_void,
                        match_0 as *const c_void,
                        4 as c_int as c_ulong as ::libc::size_t,
                    );
                    match_0 = match_0.offset(-(dec64table[offset as usize] as isize));
                } else {
                    ::libc::memcpy(
                        op as *mut c_void,
                        match_0 as *const c_void,
                        8 as c_int as c_ulong as ::libc::size_t,
                    );
                    match_0 = match_0.offset(8 as c_int as isize);
                }
                op = op.offset(8 as c_int as isize);
                if ((cpy
                    > oend.offset(
                        -((2 as c_int * 8 as c_int
                            - 4 as c_int) as isize),
                    )) as c_int
                    != 0 as c_int) as c_int
                    as c_long
                    != 0
                {
                    let oCopyLimit: *mut BYTE =
                        oend.offset(-((WILDCOPYLENGTH - 1 as c_int) as isize));
                    if cpy > oend.offset(-(LASTLITERALS as isize)) {
                        current_block = 14897191503944545335;
                        break;
                    }
                    if op < oCopyLimit {
                        LZ4_wildCopy8(
                            op as *mut c_void,
                            match_0 as *const c_void,
                            oCopyLimit as *mut c_void,
                        );
                        match_0 = match_0
                            .offset(oCopyLimit.offset_from(op) as c_long as isize);
                        op = oCopyLimit;
                    }
                    while op < cpy {
                        let fresh22 = match_0;
                        match_0 = match_0.offset(1);
                        let fresh23 = op;
                        op = op.offset(1);
                        *fresh23 = *fresh22;
                    }
                } else {
                    ::libc::memcpy(
                        op as *mut c_void,
                        match_0 as *const c_void,
                        8 as c_int as c_ulong as ::libc::size_t,
                    );
                    if length > 16 as size_t {
                        LZ4_wildCopy8(
                            op.offset(8 as c_int as isize) as *mut c_void,
                            match_0.offset(8 as c_int as isize)
                                as *const c_void,
                            cpy as *mut c_void,
                        );
                    }
                }
                op = cpy;
                current_block = 11995618668192240200;
            }
        }
    }
    match current_block {
        8038949400865391589 => {
            return (op as *mut c_char).offset_from(dst) as c_long
                as c_int;
        }
        _ => {
            return -((ip as *const c_char).offset_from(src) as c_long)
                as c_int
                - 1 as c_int;
        }
    };
}
#[inline]
pub unsafe fn LZ4_decompress_safe(
    mut source: *const c_char,
    mut dest: *mut c_char,
    mut compressedSize: c_int,
    mut maxDecompressedSize: c_int,
) -> c_int {
    return LZ4_decompress_generic(
        source,
        dest,
        compressedSize,
        maxDecompressedSize,
        decode_full_block,
        noDict,
        dest as *mut BYTE,
        ::core::ptr::null::<BYTE>(),
        0 as size_t,
    );
}
#[inline]
pub unsafe fn LZ4_decompress_safe_partial(
    mut src: *const c_char,
    mut dst: *mut c_char,
    mut compressedSize: c_int,
    mut targetOutputSize: c_int,
    mut dstCapacity: c_int,
) -> c_int {
    dstCapacity = if targetOutputSize < dstCapacity {
        targetOutputSize
    } else {
        dstCapacity
    };
    return LZ4_decompress_generic(
        src,
        dst,
        compressedSize,
        dstCapacity,
        partial_decode,
        noDict,
        dst as *mut BYTE,
        ::core::ptr::null::<BYTE>(),
        0 as size_t,
    );
}
#[inline]
pub unsafe fn LZ4_decompress_fast(
    mut source: *const c_char,
    mut dest: *mut c_char,
    mut originalSize: c_int,
) -> c_int {
    return LZ4_decompress_unsafe_generic(
        source as *const BYTE,
        dest as *mut BYTE,
        originalSize,
        0 as size_t,
        ::core::ptr::null::<BYTE>(),
        0 as size_t,
    );
}
#[inline]
pub unsafe fn LZ4_decompress_safe_withPrefix64k(
    mut source: *const c_char,
    mut dest: *mut c_char,
    mut compressedSize: c_int,
    mut maxOutputSize: c_int,
) -> c_int {
    return LZ4_decompress_generic(
        source,
        dest,
        compressedSize,
        maxOutputSize,
        decode_full_block,
        withPrefix64k,
        (dest as *mut BYTE).offset(
            -((64 as c_int * ((1 as c_int) << 10 as c_int))
                as isize),
        ),
        ::core::ptr::null::<BYTE>(),
        0 as size_t,
    );
}
unsafe fn LZ4_decompress_safe_partial_withPrefix64k(
    mut source: *const c_char,
    mut dest: *mut c_char,
    mut compressedSize: c_int,
    mut targetOutputSize: c_int,
    mut dstCapacity: c_int,
) -> c_int {
    dstCapacity = if targetOutputSize < dstCapacity {
        targetOutputSize
    } else {
        dstCapacity
    };
    return LZ4_decompress_generic(
        source,
        dest,
        compressedSize,
        dstCapacity,
        partial_decode,
        withPrefix64k,
        (dest as *mut BYTE).offset(
            -((64 as c_int * ((1 as c_int) << 10 as c_int))
                as isize),
        ),
        ::core::ptr::null::<BYTE>(),
        0 as size_t,
    );
}
#[inline]
pub unsafe fn LZ4_decompress_fast_withPrefix64k(
    mut source: *const c_char,
    mut dest: *mut c_char,
    mut originalSize: c_int,
) -> c_int {
    return LZ4_decompress_unsafe_generic(
        source as *const BYTE,
        dest as *mut BYTE,
        originalSize,
        (64 as c_int * ((1 as c_int) << 10 as c_int))
            as size_t,
        ::core::ptr::null::<BYTE>(),
        0 as size_t,
    );
}
unsafe fn LZ4_decompress_safe_withSmallPrefix(
    mut source: *const c_char,
    mut dest: *mut c_char,
    mut compressedSize: c_int,
    mut maxOutputSize: c_int,
    mut prefixSize: size_t,
) -> c_int {
    return LZ4_decompress_generic(
        source,
        dest,
        compressedSize,
        maxOutputSize,
        decode_full_block,
        noDict,
        (dest as *mut BYTE).offset(-(prefixSize as isize)),
        ::core::ptr::null::<BYTE>(),
        0 as size_t,
    );
}
unsafe fn LZ4_decompress_safe_partial_withSmallPrefix(
    mut source: *const c_char,
    mut dest: *mut c_char,
    mut compressedSize: c_int,
    mut targetOutputSize: c_int,
    mut dstCapacity: c_int,
    mut prefixSize: size_t,
) -> c_int {
    dstCapacity = if targetOutputSize < dstCapacity {
        targetOutputSize
    } else {
        dstCapacity
    };
    return LZ4_decompress_generic(
        source,
        dest,
        compressedSize,
        dstCapacity,
        partial_decode,
        noDict,
        (dest as *mut BYTE).offset(-(prefixSize as isize)),
        ::core::ptr::null::<BYTE>(),
        0 as size_t,
    );
}
#[inline]
pub unsafe fn LZ4_decompress_safe_forceExtDict(
    mut source: *const c_char,
    mut dest: *mut c_char,
    mut compressedSize: c_int,
    mut maxOutputSize: c_int,
    mut dictStart: *const c_void,
    mut dictSize: size_t,
) -> c_int {
    return LZ4_decompress_generic(
        source,
        dest,
        compressedSize,
        maxOutputSize,
        decode_full_block,
        usingExtDict,
        dest as *mut BYTE,
        dictStart as *const BYTE,
        dictSize,
    );
}
#[inline]
pub unsafe fn LZ4_decompress_safe_partial_forceExtDict(
    mut source: *const c_char,
    mut dest: *mut c_char,
    mut compressedSize: c_int,
    mut targetOutputSize: c_int,
    mut dstCapacity: c_int,
    mut dictStart: *const c_void,
    mut dictSize: size_t,
) -> c_int {
    dstCapacity = if targetOutputSize < dstCapacity {
        targetOutputSize
    } else {
        dstCapacity
    };
    return LZ4_decompress_generic(
        source,
        dest,
        compressedSize,
        dstCapacity,
        partial_decode,
        usingExtDict,
        dest as *mut BYTE,
        dictStart as *const BYTE,
        dictSize,
    );
}
unsafe fn LZ4_decompress_fast_extDict(
    mut source: *const c_char,
    mut dest: *mut c_char,
    mut originalSize: c_int,
    mut dictStart: *const c_void,
    mut dictSize: size_t,
) -> c_int {
    return LZ4_decompress_unsafe_generic(
        source as *const BYTE,
        dest as *mut BYTE,
        originalSize,
        0 as size_t,
        dictStart as *const BYTE,
        dictSize,
    );
}
#[inline(always)]
unsafe fn LZ4_decompress_safe_doubleDict(
    mut source: *const c_char,
    mut dest: *mut c_char,
    mut compressedSize: c_int,
    mut maxOutputSize: c_int,
    mut prefixSize: size_t,
    mut dictStart: *const c_void,
    mut dictSize: size_t,
) -> c_int {
    return LZ4_decompress_generic(
        source,
        dest,
        compressedSize,
        maxOutputSize,
        decode_full_block,
        usingExtDict,
        (dest as *mut BYTE).offset(-(prefixSize as isize)),
        dictStart as *const BYTE,
        dictSize,
    );
}
#[inline]
pub fn LZ4_createStreamDecode() -> *mut LZ4_streamDecode_t { unsafe {
    return calloc(
        1 as size_t,
        ::core::mem::size_of::<LZ4_streamDecode_t>() as size_t,
    ) as *mut LZ4_streamDecode_t;
} }
#[inline]
pub unsafe fn LZ4_freeStreamDecode(
    mut LZ4_stream: *mut LZ4_streamDecode_t,
) -> c_int {
    if LZ4_stream.is_null() {
        return 0 as c_int;
    }
    free(LZ4_stream as *mut c_void);
    return 0 as c_int;
}
#[inline]
pub unsafe fn LZ4_setStreamDecode(
    mut LZ4_streamDecode: *mut LZ4_streamDecode_t,
    mut dictionary: *const c_char,
    mut dictSize: c_int,
) -> c_int {
    let mut lz4sd: *mut LZ4_streamDecode_t_internal =
        &raw mut (*LZ4_streamDecode).internal_donotuse;
    (*lz4sd).prefixSize = dictSize as size_t;
    if dictSize != 0 {
        (*lz4sd).prefixEnd =
            (dictionary as *const BYTE).offset(dictSize as isize) as *const LZ4_byte;
    } else {
        (*lz4sd).prefixEnd = dictionary as *const BYTE as *const LZ4_byte;
    }
    (*lz4sd).externalDict = ::core::ptr::null::<LZ4_byte>();
    (*lz4sd).extDictSize = 0 as size_t;
    return 1 as c_int;
}
#[inline]
pub fn LZ4_decoderRingBufferSize(
    mut maxBlockSize: c_int,
) -> c_int { {
    if maxBlockSize < 0 as c_int {
        return 0 as c_int;
    }
    if maxBlockSize > LZ4_MAX_INPUT_SIZE {
        return 0 as c_int;
    }
    if maxBlockSize < 16 as c_int {
        maxBlockSize = 16 as c_int;
    }
    return 65536 as c_int + 14 as c_int + maxBlockSize;
} }
#[inline]
pub unsafe fn LZ4_decompress_safe_continue(
    mut LZ4_streamDecode: *mut LZ4_streamDecode_t,
    mut source: *const c_char,
    mut dest: *mut c_char,
    mut compressedSize: c_int,
    mut maxOutputSize: c_int,
) -> c_int {
    let mut lz4sd: *mut LZ4_streamDecode_t_internal =
        &raw mut (*LZ4_streamDecode).internal_donotuse;
    let mut result: c_int = 0;
    if (*lz4sd).prefixSize == 0 as size_t {
        result = LZ4_decompress_safe(source, dest, compressedSize, maxOutputSize);
        if result <= 0 as c_int {
            return result;
        }
        (*lz4sd).prefixSize = result as size_t;
        (*lz4sd).prefixEnd = (dest as *mut BYTE).offset(result as isize);
    } else if (*lz4sd).prefixEnd == dest as *mut BYTE as *const LZ4_byte {
        if (*lz4sd).prefixSize
            >= (64 as c_int * ((1 as c_int) << 10 as c_int)
                - 1 as c_int) as size_t
        {
            result = LZ4_decompress_safe_withPrefix64k(source, dest, compressedSize, maxOutputSize);
        } else if (*lz4sd).extDictSize == 0 as size_t {
            result = LZ4_decompress_safe_withSmallPrefix(
                source,
                dest,
                compressedSize,
                maxOutputSize,
                (*lz4sd).prefixSize,
            );
        } else {
            result = LZ4_decompress_safe_doubleDict(
                source,
                dest,
                compressedSize,
                maxOutputSize,
                (*lz4sd).prefixSize,
                (*lz4sd).externalDict as *const c_void,
                (*lz4sd).extDictSize,
            );
        }
        if result <= 0 as c_int {
            return result;
        }
        (*lz4sd).prefixSize = (*lz4sd).prefixSize.wrapping_add(result as size_t);
        (*lz4sd).prefixEnd = (*lz4sd).prefixEnd.offset(result as isize);
    } else {
        (*lz4sd).extDictSize = (*lz4sd).prefixSize;
        (*lz4sd).externalDict = (*lz4sd).prefixEnd.offset(-((*lz4sd).extDictSize as isize));
        result = LZ4_decompress_safe_forceExtDict(
            source,
            dest,
            compressedSize,
            maxOutputSize,
            (*lz4sd).externalDict as *const c_void,
            (*lz4sd).extDictSize,
        );
        if result <= 0 as c_int {
            return result;
        }
        (*lz4sd).prefixSize = result as size_t;
        (*lz4sd).prefixEnd = (dest as *mut BYTE).offset(result as isize);
    }
    return result;
}
#[inline]
pub unsafe fn LZ4_decompress_fast_continue(
    mut LZ4_streamDecode: *mut LZ4_streamDecode_t,
    mut source: *const c_char,
    mut dest: *mut c_char,
    mut originalSize: c_int,
) -> c_int {
    let lz4sd: *mut LZ4_streamDecode_t_internal = &raw mut (*LZ4_streamDecode).internal_donotuse;
    let mut result: c_int = 0;
    if (*lz4sd).prefixSize == 0 as size_t {
        result = LZ4_decompress_fast(source, dest, originalSize);
        if result <= 0 as c_int {
            return result;
        }
        (*lz4sd).prefixSize = originalSize as size_t;
        (*lz4sd).prefixEnd = (dest as *mut BYTE).offset(originalSize as isize);
    } else if (*lz4sd).prefixEnd == dest as *mut BYTE as *const LZ4_byte {
        result = LZ4_decompress_unsafe_generic(
            source as *const BYTE,
            dest as *mut BYTE,
            originalSize,
            (*lz4sd).prefixSize,
            (*lz4sd).externalDict as *const BYTE,
            (*lz4sd).extDictSize,
        );
        if result <= 0 as c_int {
            return result;
        }
        (*lz4sd).prefixSize = (*lz4sd).prefixSize.wrapping_add(originalSize as size_t);
        (*lz4sd).prefixEnd = (*lz4sd).prefixEnd.offset(originalSize as isize);
    } else {
        (*lz4sd).extDictSize = (*lz4sd).prefixSize;
        (*lz4sd).externalDict = (*lz4sd).prefixEnd.offset(-((*lz4sd).extDictSize as isize));
        result = LZ4_decompress_fast_extDict(
            source,
            dest,
            originalSize,
            (*lz4sd).externalDict as *const c_void,
            (*lz4sd).extDictSize,
        );
        if result <= 0 as c_int {
            return result;
        }
        (*lz4sd).prefixSize = originalSize as size_t;
        (*lz4sd).prefixEnd = (dest as *mut BYTE).offset(originalSize as isize);
    }
    return result;
}
#[inline]
pub unsafe fn LZ4_decompress_safe_usingDict(
    mut source: *const c_char,
    mut dest: *mut c_char,
    mut compressedSize: c_int,
    mut maxOutputSize: c_int,
    mut dictStart: *const c_char,
    mut dictSize: c_int,
) -> c_int {
    if dictSize == 0 as c_int {
        return LZ4_decompress_safe(source, dest, compressedSize, maxOutputSize);
    }
    if dictStart.offset(dictSize as isize) == dest as *const c_char {
        if dictSize
            >= 64 as c_int * ((1 as c_int) << 10 as c_int)
                - 1 as c_int
        {
            return LZ4_decompress_safe_withPrefix64k(source, dest, compressedSize, maxOutputSize);
        }
        return LZ4_decompress_safe_withSmallPrefix(
            source,
            dest,
            compressedSize,
            maxOutputSize,
            dictSize as size_t,
        );
    }
    return LZ4_decompress_safe_forceExtDict(
        source,
        dest,
        compressedSize,
        maxOutputSize,
        dictStart as *const c_void,
        dictSize as size_t,
    );
}
#[inline]
pub unsafe fn LZ4_decompress_safe_partial_usingDict(
    mut source: *const c_char,
    mut dest: *mut c_char,
    mut compressedSize: c_int,
    mut targetOutputSize: c_int,
    mut dstCapacity: c_int,
    mut dictStart: *const c_char,
    mut dictSize: c_int,
) -> c_int {
    if dictSize == 0 as c_int {
        return LZ4_decompress_safe_partial(
            source,
            dest,
            compressedSize,
            targetOutputSize,
            dstCapacity,
        );
    }
    if dictStart.offset(dictSize as isize) == dest as *const c_char {
        if dictSize
            >= 64 as c_int * ((1 as c_int) << 10 as c_int)
                - 1 as c_int
        {
            return LZ4_decompress_safe_partial_withPrefix64k(
                source,
                dest,
                compressedSize,
                targetOutputSize,
                dstCapacity,
            );
        }
        return LZ4_decompress_safe_partial_withSmallPrefix(
            source,
            dest,
            compressedSize,
            targetOutputSize,
            dstCapacity,
            dictSize as size_t,
        );
    }
    return LZ4_decompress_safe_partial_forceExtDict(
        source,
        dest,
        compressedSize,
        targetOutputSize,
        dstCapacity,
        dictStart as *const c_void,
        dictSize as size_t,
    );
}
#[inline]
pub unsafe fn LZ4_decompress_fast_usingDict(
    mut source: *const c_char,
    mut dest: *mut c_char,
    mut originalSize: c_int,
    mut dictStart: *const c_char,
    mut dictSize: c_int,
) -> c_int {
    if dictSize == 0 as c_int
        || dictStart.offset(dictSize as isize) == dest as *const c_char
    {
        return LZ4_decompress_unsafe_generic(
            source as *const BYTE,
            dest as *mut BYTE,
            originalSize,
            dictSize as size_t,
            ::core::ptr::null::<BYTE>(),
            0 as size_t,
        );
    }
    return LZ4_decompress_fast_extDict(
        source,
        dest,
        originalSize,
        dictStart as *const c_void,
        dictSize as size_t,
    );
}
#[inline]
pub unsafe fn LZ4_compress_limitedOutput(
    mut source: *const c_char,
    mut dest: *mut c_char,
    mut inputSize: c_int,
    mut maxOutputSize: c_int,
) -> c_int {
    return LZ4_compress_default(source, dest, inputSize, maxOutputSize);
}
#[inline]
pub unsafe fn LZ4_compress(
    mut src: *const c_char,
    mut dest: *mut c_char,
    mut srcSize: c_int,
) -> c_int {
    return LZ4_compress_default(src, dest, srcSize, LZ4_compressBound(srcSize));
}
#[inline]
pub unsafe fn LZ4_compress_limitedOutput_withState(
    mut state: *mut c_void,
    mut src: *const c_char,
    mut dst: *mut c_char,
    mut srcSize: c_int,
    mut dstSize: c_int,
) -> c_int {
    return LZ4_compress_fast_extState(state, src, dst, srcSize, dstSize, 1 as c_int);
}
#[inline]
pub unsafe fn LZ4_compress_withState(
    mut state: *mut c_void,
    mut src: *const c_char,
    mut dst: *mut c_char,
    mut srcSize: c_int,
) -> c_int {
    return LZ4_compress_fast_extState(
        state,
        src,
        dst,
        srcSize,
        LZ4_compressBound(srcSize),
        1 as c_int,
    );
}
#[inline]
pub unsafe fn LZ4_compress_limitedOutput_continue(
    mut LZ4_stream: *mut LZ4_stream_t,
    mut src: *const c_char,
    mut dst: *mut c_char,
    mut srcSize: c_int,
    mut dstCapacity: c_int,
) -> c_int {
    return LZ4_compress_fast_continue(
        LZ4_stream,
        src,
        dst,
        srcSize,
        dstCapacity,
        1 as c_int,
    );
}
#[inline]
pub unsafe fn LZ4_compress_continue(
    mut LZ4_stream: *mut LZ4_stream_t,
    mut source: *const c_char,
    mut dest: *mut c_char,
    mut inputSize: c_int,
) -> c_int {
    return LZ4_compress_fast_continue(
        LZ4_stream,
        source,
        dest,
        inputSize,
        LZ4_compressBound(inputSize),
        1 as c_int,
    );
}
#[inline]
pub unsafe fn LZ4_uncompress(
    mut source: *const c_char,
    mut dest: *mut c_char,
    mut outputSize: c_int,
) -> c_int {
    return LZ4_decompress_fast(source, dest, outputSize);
}
#[inline]
pub unsafe fn LZ4_uncompress_unknownOutputSize(
    mut source: *const c_char,
    mut dest: *mut c_char,
    mut isize: c_int,
    mut maxOutputSize: c_int,
) -> c_int {
    return LZ4_decompress_safe(source, dest, isize, maxOutputSize);
}
#[inline]
pub fn LZ4_sizeofStreamState() -> c_int { {
    return ::core::mem::size_of::<LZ4_stream_t>() as c_int;
} }
#[inline]
pub unsafe fn LZ4_resetStreamState(
    mut state: *mut c_void,
    mut inputBuffer: *mut c_char,
) -> c_int {
    LZ4_resetStream(state as *mut LZ4_stream_t);
    return 0 as c_int;
}
#[inline]
pub unsafe fn LZ4_create(
    mut inputBuffer: *mut c_char,
) -> *mut c_void {
    return LZ4_createStream() as *mut c_void;
}
#[inline]
pub unsafe fn LZ4_slideInputBuffer(
    mut state: *mut c_void,
) -> *mut c_char {
    return (*(state as *mut LZ4_stream_t)).internal_donotuse.dictionary as uptrval
        as *mut c_char;
}
