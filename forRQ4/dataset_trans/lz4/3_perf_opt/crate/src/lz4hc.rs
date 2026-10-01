use core::ffi::*;
pub use crate::src::c_inlined_fns::*;
pub use crate::src::ffi::*;
pub use crate::src::c_consts::*;
pub use crate::src::c_structs::*;
pub use crate::src::c_types::*;
use ::libc;
use crate::src::lz4::LZ4_compressBound;
use crate::src::c_inlined_fns::LZ4_isAligned;
use crate::src::c_inlined_fns::LZ4_read16;
use crate::src::c_inlined_fns::LZ4_read32;
use crate::src::c_inlined_fns::LZ4_read_ARCH;
use crate::src::c_inlined_fns::LZ4_write16;

pub type dictCtx_directive = c_uint;
pub const usingDictCtxHc: dictCtx_directive = 1;
pub const noDictCtx: dictCtx_directive = 0;
pub type HCfavor_e = c_uint;
pub const favorDecompressionSpeed: HCfavor_e = 1;
pub const favorCompressionRatio: HCfavor_e = 0;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct cParams_t {
    pub strat: lz4hc_strat_e,
    pub nbSearches: c_int,
    pub targetLength: U32,
}
pub type lz4hc_strat_e = c_uint;
pub const lz4opt: lz4hc_strat_e = 2;
pub const lz4hc: lz4hc_strat_e = 1;
pub const lz4mid: lz4hc_strat_e = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LZ4HC_optimal_t {
    pub price: c_int,
    pub off: c_int,
    pub mlen: c_int,
    pub litlen: c_int,
}
pub type BYTE = c_uchar;

#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_hu47194827 {
    pub u: U32,
    pub c: [BYTE; 4],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct LZ4HC_match_t {
    pub off: c_int,
    pub len: c_int,
    pub back: c_int,
}

pub const rep_confirmed: repeat_state_e = 2;
pub type repeat_state_e = c_uint;
pub const rep_not: repeat_state_e = 1;
pub const rep_untested: repeat_state_e = 0;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct LZ4_unalign64 {
    pub u64_0: U64,
}
pub type LZ4MID_searchIntoDict_f = Option<
    unsafe extern "C" fn(
        *const BYTE,
        U32,
        *const BYTE,
        *const LZ4HC_CCtx_internal,
        U32,
    ) -> LZ4HC_match_t,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct t_a {
    pub c: c_char,
    pub t: LZ4_streamHC_t,
}

pub const LZ4HC_HASH_LOG: c_int = 15 as c_int;

static mut LZ4_minLength: c_int = MFLIMIT + 1 as c_int;

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
pub const OPTIMAL_ML: c_int = ML_MASK
    .wrapping_sub(1 as c_uint)
    .wrapping_add(MINMATCH as c_uint)
    as c_int;
pub const LZ4_OPT_NUM: c_int = (1 as c_int) << 12 as c_int;
static mut k_clTable: [cParams_t; 13] = [
    cParams_t {
        strat: lz4mid,
        nbSearches: 2 as c_int,
        targetLength: 16 as U32,
    },
    cParams_t {
        strat: lz4mid,
        nbSearches: 2 as c_int,
        targetLength: 16 as U32,
    },
    cParams_t {
        strat: lz4mid,
        nbSearches: 2 as c_int,
        targetLength: 16 as U32,
    },
    cParams_t {
        strat: lz4hc,
        nbSearches: 4 as c_int,
        targetLength: 16 as U32,
    },
    cParams_t {
        strat: lz4hc,
        nbSearches: 8 as c_int,
        targetLength: 16 as U32,
    },
    cParams_t {
        strat: lz4hc,
        nbSearches: 16 as c_int,
        targetLength: 16 as U32,
    },
    cParams_t {
        strat: lz4hc,
        nbSearches: 32 as c_int,
        targetLength: 16 as U32,
    },
    cParams_t {
        strat: lz4hc,
        nbSearches: 64 as c_int,
        targetLength: 16 as U32,
    },
    cParams_t {
        strat: lz4hc,
        nbSearches: 128 as c_int,
        targetLength: 16 as U32,
    },
    cParams_t {
        strat: lz4hc,
        nbSearches: 256 as c_int,
        targetLength: 16 as U32,
    },
    cParams_t {
        strat: lz4opt,
        nbSearches: 96 as c_int,
        targetLength: 64 as U32,
    },
    cParams_t {
        strat: lz4opt,
        nbSearches: 512 as c_int,
        targetLength: 128 as U32,
    },
    cParams_t {
        strat: lz4opt,
        nbSearches: 16384 as c_int,
        targetLength: LZ4_OPT_NUM as U32,
    },
];
fn LZ4HC_getCLevelParams(mut cLevel: c_int) -> cParams_t { unsafe {
    if cLevel < 1 as c_int {
        cLevel = LZ4HC_CLEVEL_DEFAULT;
    }
    cLevel = if (12 as c_int) < cLevel {
        12 as c_int
    } else {
        cLevel
    };
    return k_clTable[cLevel as usize];
} }
pub const LZ4HC_HASHSIZE: c_int = 4 as c_int;
unsafe fn LZ4HC_hashPtr(mut ptr: *const c_void) -> U32 {
    return LZ4_read32(ptr).wrapping_mul(2654435761 as U32)
        >> MINMATCH * 8 as c_int - LZ4HC_HASH_LOG;
}
unsafe fn LZ4_read64(mut ptr: *const c_void) -> U64 {
    return (*(ptr as *const LZ4_unalign64)).u64_0;
}
pub const LZ4MID_HASHSIZE: c_int = 8 as c_int;
pub const LZ4MID_HASHLOG: c_int = LZ4HC_HASH_LOG - 1 as c_int;
pub const LZ4MID_HASHTABLESIZE: c_int = (1 as c_int) << LZ4MID_HASHLOG;
fn LZ4MID_hash4(mut v: U32) -> U32 { {
    return v.wrapping_mul(2654435761 as U32) >> 32 as c_int - LZ4MID_HASHLOG;
} }
unsafe fn LZ4MID_hash4Ptr(mut ptr: *const c_void) -> U32 {
    return LZ4MID_hash4(LZ4_read32(ptr));
}
fn LZ4MID_hash7(mut v: U64) -> U32 { {
    return (((v << 64 as c_int - 56 as c_int)
        as c_ulonglong)
        .wrapping_mul(58295818150454627 as c_ulonglong)
        >> 64 as c_int - LZ4MID_HASHLOG) as U32;
} }
unsafe fn LZ4MID_hash8Ptr(mut ptr: *const c_void) -> U32 {
    return LZ4MID_hash7(LZ4_readLE64(ptr));
}
unsafe fn LZ4_readLE64(mut memPtr: *const c_void) -> U64 {
    if LZ4_isLittleEndian() != 0 {
        return LZ4_read64(memPtr);
    } else {
        let mut p: *const BYTE = memPtr as *const BYTE;
        return *p.offset(0 as c_int as isize) as U64
            | (*p.offset(1 as c_int as isize) as U64) << 8 as c_int
            | (*p.offset(2 as c_int as isize) as U64) << 16 as c_int
            | (*p.offset(3 as c_int as isize) as U64) << 24 as c_int
            | (*p.offset(4 as c_int as isize) as U64) << 32 as c_int
            | (*p.offset(5 as c_int as isize) as U64) << 40 as c_int
            | (*p.offset(6 as c_int as isize) as U64) << 48 as c_int
            | (*p.offset(7 as c_int as isize) as U64) << 56 as c_int;
    };
}
#[inline(always)]
fn LZ4HC_NbCommonBytes32(mut val: U32) -> c_uint { {
    if LZ4_isLittleEndian() != 0 {
        return val.leading_zeros() as i32 as c_uint >> 3 as c_int;
    } else {
        return val.trailing_zeros() as i32 as c_uint >> 3 as c_int;
    };
} }
#[inline(always)]
unsafe fn LZ4HC_countBack(
    ip_0: *const BYTE,
    match_0: *const BYTE,
    iMin: *const BYTE,
    mMin: *const BYTE,
) -> c_int {
    let mut back: c_int = 0 as c_int;
    let min: c_int = (if iMin.offset_from(ip_0) as c_long
        > mMin.offset_from(match_0) as c_long
    {
        iMin.offset_from(ip_0) as c_long
    } else {
        mMin.offset_from(match_0) as c_long
    }) as c_int;
    while back - min > 3 as c_int {
        let v: U32 = LZ4_read32(
            ip_0.offset(back as isize)
                .offset(-(4 as c_int as isize))
                as *const c_void,
        ) as U32
            ^ LZ4_read32(
                match_0
                    .offset(back as isize)
                    .offset(-(4 as c_int as isize))
                    as *const c_void,
            ) as U32;
        if v != 0 {
            return back - LZ4HC_NbCommonBytes32(v) as c_int;
        } else {
            back -= 4 as c_int;
        }
    }
    while back > min
        && *ip_0.offset((back - 1 as c_int) as isize) as c_int
            == *match_0.offset((back - 1 as c_int) as isize) as c_int
    {
        back -= 1;
    }
    return back;
}
unsafe fn LZ4HC_clearTables(mut hc4: *mut LZ4HC_CCtx_internal) {
    let hc4_view: &mut LZ4HC_CCtx_internal = unsafe { &mut *hc4 };
    memset(
        &raw mut hc4_view.hashTable as *mut LZ4_u32 as *mut c_void,
        0 as c_int,
        ::core::mem::size_of::<[LZ4_u32; 32768]>() as size_t,
    );
    memset(
        &raw mut hc4_view.chainTable as *mut LZ4_u16 as *mut c_void,
        0xff as c_int,
        ::core::mem::size_of::<[LZ4_u16; 65536]>() as size_t,
    );
}
unsafe fn LZ4HC_init_internal(
    mut hc4: *mut LZ4HC_CCtx_internal,
    mut start: *const BYTE,
) {
    let bufferSize: size_t =
        (*hc4).end.offset_from((*hc4).prefixStart) as c_long as size_t;
    let mut newStartingOffset: size_t = bufferSize.wrapping_add((*hc4).dictLimit as size_t);
    if newStartingOffset
        > (1 as c_uint)
            .wrapping_mul((1 as c_uint) << 30 as c_int)
            as size_t
    {
        LZ4HC_clearTables(hc4);
        newStartingOffset = 0 as size_t;
    }
    newStartingOffset = newStartingOffset.wrapping_add(
        (64 as c_int * ((1 as c_int) << 10 as c_int))
            as size_t,
    );
    (*hc4).nextToUpdate = newStartingOffset as U32 as LZ4_u32;
    (*hc4).prefixStart = start as *const LZ4_byte;
    (*hc4).end = start as *const LZ4_byte;
    (*hc4).dictStart = start as *const LZ4_byte;
    (*hc4).dictLimit = newStartingOffset as U32 as LZ4_u32;
    (*hc4).lowLimit = newStartingOffset as U32 as LZ4_u32;
}
#[inline(always)]
unsafe fn LZ4HC_encodeSequence(
    mut _ip: *mut *const BYTE,
    mut _op: *mut *mut BYTE,
    mut _anchor: *mut *const BYTE,
    mut matchLength: c_int,
    mut offset: c_int,
    mut limit: limitedOutput_directive,
    mut oend: *mut BYTE,
) -> c_int {
    let fresh4 = *_op;
    *_op = (*_op).offset(1);
    let token: *mut BYTE = fresh4;
    let mut litLen: size_t = (*_ip).offset_from(*_anchor) as c_long as size_t;
    if limit as c_uint != 0
        && (*_op)
            .offset(litLen.wrapping_div(255 as size_t) as isize)
            .offset(litLen as isize)
            .offset((2 as c_int + 1 as c_int + LASTLITERALS) as isize)
            > oend
    {
        return 1 as c_int;
    }
    if litLen >= RUN_MASK as size_t {
        let mut len: size_t = litLen.wrapping_sub(RUN_MASK as size_t);
        *token = (RUN_MASK << ML_BITS) as BYTE;
        while len >= 255 as size_t {
            let fresh5 = *_op;
            *_op = (*_op).offset(1);
            *fresh5 = 255 as BYTE;
            len = len.wrapping_sub(255 as size_t);
        }
        let fresh6 = *_op;
        *_op = (*_op).offset(1);
        *fresh6 = len as BYTE;
    } else {
        *token = (litLen << ML_BITS) as BYTE;
    }
    LZ4_wildCopy8(
        *_op as *mut c_void,
        *_anchor as *const c_void,
        (*_op).offset(litLen as isize) as *mut c_void,
    );
    *_op = (*_op).offset(litLen as isize);
    LZ4_writeLE16(*_op as *mut c_void, offset as U16);
    *_op = (*_op).offset(2 as c_int as isize);
    let mut mlCode: size_t = (matchLength as size_t).wrapping_sub(MINMATCH as size_t);
    if limit as c_uint != 0
        && (*_op)
            .offset(mlCode.wrapping_div(255 as size_t) as isize)
            .offset((1 as c_int + LASTLITERALS) as isize)
            > oend
    {
        return 1 as c_int;
    }
    if mlCode >= ML_MASK as size_t {
        *token = (*token as c_uint).wrapping_add(ML_MASK) as BYTE as BYTE;
        mlCode = mlCode.wrapping_sub(ML_MASK as size_t);
        while mlCode >= 510 as size_t {
            let fresh7 = *_op;
            *_op = (*_op).offset(1);
            *fresh7 = 255 as BYTE;
            let fresh8 = *_op;
            *_op = (*_op).offset(1);
            *fresh8 = 255 as BYTE;
            mlCode = mlCode.wrapping_sub(510 as size_t);
        }
        if mlCode >= 255 as size_t {
            mlCode = mlCode.wrapping_sub(255 as size_t);
            let fresh9 = *_op;
            *_op = (*_op).offset(1);
            *fresh9 = 255 as BYTE;
        }
        let fresh10 = *_op;
        *_op = (*_op).offset(1);
        *fresh10 = mlCode as BYTE;
    } else {
        *token = (*token as c_int + mlCode as BYTE as c_int) as BYTE;
    }
    *_ip = (*_ip).offset(matchLength as isize);
    *_anchor = *_ip;
    return 0 as c_int;
}
unsafe fn LZ4HC_searchExtDict(
    mut ip_0: *const BYTE,
    mut ipIndex: U32,
    iLowLimit: *const BYTE,
    iHighLimit: *const BYTE,
    mut dictCtx: *const LZ4HC_CCtx_internal,
    mut gDictEndIndex: U32,
    mut currentBestML: c_int,
    mut nbAttempts: c_int,
) -> LZ4HC_match_t {
    let dictCtx_view: &LZ4HC_CCtx_internal = unsafe { &*dictCtx };
    let lDictEndIndex: size_t = (dictCtx_view.end.offset_from(dictCtx_view.prefixStart)
        as c_long as size_t)
        .wrapping_add(dictCtx_view.dictLimit as size_t);
    let mut lDictMatchIndex: U32 =
        dictCtx_view.hashTable[LZ4HC_hashPtr(ip_0 as *const c_void) as usize];
    let mut matchIndex: U32 = lDictMatchIndex
        .wrapping_add(gDictEndIndex)
        .wrapping_sub(lDictEndIndex as U32);
    let mut offset: c_int = 0 as c_int;
    let mut sBack: c_int = 0 as c_int;
    lDictMatchIndex > 0 as U32;
    while ipIndex.wrapping_sub(matchIndex) <= LZ4_DISTANCE_MAX as U32 && {
        let fresh21 = nbAttempts;
        nbAttempts = nbAttempts - 1;
        fresh21 != 0
    } {
        let matchPtr: *const BYTE = dictCtx_view
            .prefixStart
            .offset(-(dictCtx_view.dictLimit as isize))
            .offset(lDictMatchIndex as isize);
        if LZ4_read32(matchPtr as *const c_void)
            == LZ4_read32(ip_0 as *const c_void)
        {
            let mut mlt: c_int = 0;
            let mut back: c_int = 0 as c_int;
            let mut vLimit: *const BYTE =
                ip_0.offset(lDictEndIndex.wrapping_sub(lDictMatchIndex as size_t) as isize);
            if vLimit > iHighLimit {
                vLimit = iHighLimit;
            }
            mlt = LZ4_count(
                ip_0.offset(MINMATCH as isize),
                matchPtr.offset(MINMATCH as isize),
                vLimit,
            ) as c_int
                + MINMATCH;
            back = if ip_0 > iLowLimit {
                LZ4HC_countBack(
                    ip_0,
                    matchPtr,
                    iLowLimit,
                    dictCtx_view.prefixStart as *const BYTE,
                )
            } else {
                0 as c_int
            };
            mlt -= back;
            if mlt > currentBestML {
                currentBestML = mlt;
                offset = ipIndex.wrapping_sub(matchIndex) as c_int;
                sBack = back;
            }
        }
        let nextOffset: U32 = dictCtx_view.chainTable[lDictMatchIndex as U16 as usize] as U32;
        lDictMatchIndex = lDictMatchIndex.wrapping_sub(nextOffset);
        matchIndex = matchIndex.wrapping_sub(nextOffset);
    }
    let mut md: LZ4HC_match_t = LZ4HC_match_t {
        off: 0,
        len: 0,
        back: 0,
    };
    md.len = currentBestML;
    md.off = offset;
    md.back = sBack;
    return md;
}
unsafe extern "C" fn LZ4MID_searchHCDict(
    mut ip_0: *const BYTE,
    mut ipIndex: U32,
    iHighLimit: *const BYTE,
    mut dictCtx: *const LZ4HC_CCtx_internal,
    mut gDictEndIndex: U32,
) -> LZ4HC_match_t {
    return LZ4HC_searchExtDict(
        ip_0,
        ipIndex,
        ip_0,
        iHighLimit,
        dictCtx,
        gDictEndIndex,
        MINMATCH - 1 as c_int,
        2 as c_int,
    );
}
unsafe extern "C" fn LZ4MID_searchExtDict(
    mut ip_0: *const BYTE,
    mut ipIndex: U32,
    iHighLimit: *const BYTE,
    mut dictCtx: *const LZ4HC_CCtx_internal,
    mut gDictEndIndex: U32,
) -> LZ4HC_match_t {
    let dictCtx_view: &LZ4HC_CCtx_internal = unsafe { &*dictCtx };
    let lDictEndIndex: size_t = (dictCtx_view.end.offset_from(dictCtx_view.prefixStart)
        as c_long as size_t)
        .wrapping_add(dictCtx_view.dictLimit as size_t);
    let hash4Table: *const U32 = &raw const dictCtx_view.hashTable as *const U32;
    let hash8Table: *const U32 = hash4Table.offset(LZ4MID_HASHTABLESIZE as isize);
    let mut l8DictMatchIndex: U32 =
        *hash8Table.offset(LZ4MID_hash8Ptr(ip_0 as *const c_void) as isize);
    let mut m8Index: U32 = l8DictMatchIndex
        .wrapping_add(gDictEndIndex)
        .wrapping_sub(lDictEndIndex as U32);
    if ipIndex.wrapping_sub(m8Index) <= LZ4_DISTANCE_MAX as U32 {
        let matchPtr: *const BYTE = dictCtx_view
            .prefixStart
            .offset(-(dictCtx_view.dictLimit as isize))
            .offset(l8DictMatchIndex as isize);
        let safeLen: size_t = if lDictEndIndex.wrapping_sub(l8DictMatchIndex as size_t)
            < iHighLimit.offset_from(ip_0) as c_long as size_t
        {
            lDictEndIndex.wrapping_sub(l8DictMatchIndex as size_t)
        } else {
            iHighLimit.offset_from(ip_0) as c_long as size_t
        };
        let mut mlt: c_int =
            LZ4_count(ip_0, matchPtr, ip_0.offset(safeLen as isize)) as c_int;
        if mlt >= MINMATCH {
            let mut md: LZ4HC_match_t = LZ4HC_match_t {
                off: 0,
                len: 0,
                back: 0,
            };
            md.len = mlt;
            md.off = ipIndex.wrapping_sub(m8Index) as c_int;
            md.back = 0 as c_int;
            return md;
        }
    }
    let mut l4DictMatchIndex: U32 =
        *hash4Table.offset(LZ4MID_hash4Ptr(ip_0 as *const c_void) as isize);
    let mut m4Index: U32 = l4DictMatchIndex
        .wrapping_add(gDictEndIndex)
        .wrapping_sub(lDictEndIndex as U32);
    if ipIndex.wrapping_sub(m4Index) <= LZ4_DISTANCE_MAX as U32 {
        let matchPtr_0: *const BYTE = dictCtx_view
            .prefixStart
            .offset(-(dictCtx_view.dictLimit as isize))
            .offset(l4DictMatchIndex as isize);
        let safeLen_0: size_t = if lDictEndIndex.wrapping_sub(l4DictMatchIndex as size_t)
            < iHighLimit.offset_from(ip_0) as c_long as size_t
        {
            lDictEndIndex.wrapping_sub(l4DictMatchIndex as size_t)
        } else {
            iHighLimit.offset_from(ip_0) as c_long as size_t
        };
        let mut mlt_0: c_int =
            LZ4_count(ip_0, matchPtr_0, ip_0.offset(safeLen_0 as isize)) as c_int;
        if mlt_0 >= MINMATCH {
            let mut md_0: LZ4HC_match_t = LZ4HC_match_t {
                off: 0,
                len: 0,
                back: 0,
            };
            md_0.len = mlt_0;
            md_0.off = ipIndex.wrapping_sub(m4Index) as c_int;
            md_0.back = 0 as c_int;
            return md_0;
        }
    }
    let md_1: LZ4HC_match_t = LZ4HC_match_t {
        off: 0 as c_int,
        len: 0 as c_int,
        back: 0 as c_int,
    };
    return md_1;
}
#[inline(always)]
unsafe fn LZ4MID_addPosition(mut hTable: *mut U32, mut hValue: U32, mut index: U32) {
    *hTable.offset(hValue as isize) = index;
}
unsafe fn LZ4MID_fillHTable(
    mut cctx: *mut LZ4HC_CCtx_internal,
    mut dict: *const c_void,
    mut size: size_t,
) {
    let hash4Table: *mut U32 = &raw mut (*cctx).hashTable as *mut U32;
    let hash8Table: *mut U32 = hash4Table.offset(LZ4MID_HASHTABLESIZE as isize);
    let prefixPtr: *const BYTE = dict as *const BYTE;
    let prefixIdx: U32 = (*cctx).dictLimit as U32;
    let target: U32 = prefixIdx
        .wrapping_add(size as U32)
        .wrapping_sub(LZ4MID_HASHSIZE as U32);
    let mut idx: U32 = (*cctx).nextToUpdate as U32;
    if size <= LZ4MID_HASHSIZE as size_t {
        return;
    }
    while idx < target {
        LZ4MID_addPosition(
            hash4Table,
            LZ4MID_hash4Ptr(prefixPtr.offset(idx as isize).offset(-(prefixIdx as isize))
                as *const c_void),
            idx,
        );
        LZ4MID_addPosition(
            hash8Table,
            LZ4MID_hash8Ptr(
                prefixPtr
                    .offset(idx as isize)
                    .offset(1 as c_int as isize)
                    .offset(-(prefixIdx as isize)) as *const c_void,
            ),
            idx.wrapping_add(1 as U32),
        );
        idx = idx.wrapping_add(3 as U32);
    }
    idx = if size
        > (32 as c_int * ((1 as c_int) << 10 as c_int)
            + LZ4MID_HASHSIZE) as size_t
    {
        target.wrapping_sub(
            (32 as c_int * ((1 as c_int) << 10 as c_int))
                as U32,
        )
    } else {
        (*cctx).nextToUpdate as U32
    };
    while idx < target {
        LZ4MID_addPosition(
            hash8Table,
            LZ4MID_hash8Ptr(prefixPtr.offset(idx as isize).offset(-(prefixIdx as isize))
                as *const c_void),
            idx,
        );
        idx = idx.wrapping_add(1 as U32);
    }
    (*cctx).nextToUpdate = target as LZ4_u32;
}
unsafe fn select_searchDict_function(
    mut dictCtx: *const LZ4HC_CCtx_internal,
) -> LZ4MID_searchIntoDict_f {
    if dictCtx.is_null() {
        return None;
    }
    if LZ4HC_getCLevelParams((*dictCtx).compressionLevel as c_int).strat
        as c_uint
        == lz4mid as c_int as c_uint
    {
        return Some(
            LZ4MID_searchExtDict
                as unsafe extern "C" fn(
                    *const BYTE,
                    U32,
                    *const BYTE,
                    *const LZ4HC_CCtx_internal,
                    U32,
                ) -> LZ4HC_match_t,
        );
    }
    return Some(
        LZ4MID_searchHCDict
            as unsafe extern "C" fn(
                *const BYTE,
                U32,
                *const BYTE,
                *const LZ4HC_CCtx_internal,
                U32,
            ) -> LZ4HC_match_t,
    );
}
unsafe fn LZ4MID_compress(
    ctx: *mut LZ4HC_CCtx_internal,
    src: *const c_char,
    dst: *mut c_char,
    mut srcSizePtr: *mut c_int,
    maxOutputSize: c_int,
    limit: limitedOutput_directive,
    dict: dictCtx_directive,
) -> c_int {
    let srcSizePtr_view: &mut c_int = unsafe { &mut *srcSizePtr };
    let mut current_block: u64;
    let hash4Table: *mut U32 = &raw mut (*ctx).hashTable as *mut U32;
    let hash8Table: *mut U32 = hash4Table.offset(LZ4MID_HASHTABLESIZE as isize);
    let mut ip_0: *const BYTE = src as *const BYTE;
    let mut anchor_0: *const BYTE = ip_0;
    let iend: *const BYTE = ip_0.offset(*srcSizePtr_view as isize);
    let mflimit: *const BYTE = iend.offset(-(MFLIMIT as isize));
    let matchlimit: *const BYTE = iend.offset(-(LASTLITERALS as isize));
    let ilimit: *const BYTE = iend.offset(-(LZ4MID_HASHSIZE as isize));
    let mut op_0: *mut BYTE = dst as *mut BYTE;
    let mut oend: *mut BYTE = op_0.offset(maxOutputSize as isize);
    let prefixPtr: *const BYTE = (*ctx).prefixStart as *const BYTE;
    let prefixIdx: U32 = (*ctx).dictLimit as U32;
    let ilimitIdx: U32 =
        (ilimit.offset_from(prefixPtr) as c_long as U32).wrapping_add(prefixIdx);
    let dictStart: *const BYTE = (*ctx).dictStart as *const BYTE;
    let dictIdx: U32 = (*ctx).lowLimit as U32;
    let gDictEndIndex: U32 = (*ctx).lowLimit as U32;
    let searchIntoDict: LZ4MID_searchIntoDict_f = if dict as c_uint
        == usingDictCtxHc as c_int as c_uint
    {
        select_searchDict_function((*ctx).dictCtx) as LZ4MID_searchIntoDict_f
    } else {
        None
    };
    let mut matchLength: c_uint = 0;
    let mut matchDistance: c_uint = 0;
    dict as c_uint == usingDictCtxHc as c_int as c_uint;
    if limit as c_uint == fillOutput as c_int as c_uint {
        oend = oend.offset(-(LASTLITERALS as isize));
    }
    if *srcSizePtr_view < LZ4_minLength {
        current_block = 17215225966118807657;
    } else {
        current_block = 15976848397966268834;
    }
    loop {
        match current_block {
            15976848397966268834 => {
                if !(ip_0 <= mflimit) {
                    current_block = 17215225966118807657;
                    continue;
                }
                let ipIndex: U32 = (ip_0.offset_from(prefixPtr) as c_long as U32)
                    .wrapping_add(prefixIdx);
                let h8: U32 = LZ4MID_hash8Ptr(ip_0 as *const c_void) as U32;
                let pos8: U32 = *hash8Table.offset(h8 as isize);
                LZ4MID_addPosition(hash8Table, h8, ipIndex);
                if ipIndex.wrapping_sub(pos8) <= LZ4_DISTANCE_MAX as U32 {
                    if pos8 >= prefixIdx {
                        let matchPtr: *const BYTE = prefixPtr
                            .offset(pos8 as isize)
                            .offset(-(prefixIdx as isize));
                        matchLength = LZ4_count(ip_0, matchPtr, matchlimit);
                        if matchLength >= MINMATCH as c_uint {
                            matchDistance = ipIndex.wrapping_sub(pos8) as c_uint;
                            current_block = 2714784867065232716;
                        } else {
                            current_block = 10692455896603418738;
                        }
                    } else if pos8 >= dictIdx {
                        let matchPtr_0: *const BYTE =
                            dictStart.offset(pos8.wrapping_sub(dictIdx) as isize);
                        let safeLen: size_t = if (prefixIdx.wrapping_sub(pos8) as size_t)
                            < matchlimit.offset_from(ip_0) as c_long as size_t
                        {
                            prefixIdx.wrapping_sub(pos8) as size_t
                        } else {
                            matchlimit.offset_from(ip_0) as c_long as size_t
                        };
                        matchLength = LZ4_count(ip_0, matchPtr_0, ip_0.offset(safeLen as isize));
                        if matchLength >= MINMATCH as c_uint {
                            matchDistance = ipIndex.wrapping_sub(pos8) as c_uint;
                            current_block = 2714784867065232716;
                        } else {
                            current_block = 10692455896603418738;
                        }
                    } else {
                        current_block = 10692455896603418738;
                    }
                } else {
                    current_block = 10692455896603418738;
                }
                match current_block {
                    10692455896603418738 => {
                        let h4: U32 = LZ4MID_hash4Ptr(ip_0 as *const c_void) as U32;
                        let pos4: U32 = *hash4Table.offset(h4 as isize);
                        LZ4MID_addPosition(hash4Table, h4, ipIndex);
                        if ipIndex.wrapping_sub(pos4) <= LZ4_DISTANCE_MAX as U32 {
                            if pos4 >= prefixIdx {
                                let matchPtr_1: *const BYTE =
                                    prefixPtr.offset(pos4.wrapping_sub(prefixIdx) as isize);
                                matchLength = LZ4_count(ip_0, matchPtr_1, matchlimit);
                                if matchLength >= MINMATCH as c_uint {
                                    let h8_0: U32 = LZ4MID_hash8Ptr(
                                        ip_0.offset(1 as c_int as isize)
                                            as *const c_void,
                                    ) as U32;
                                    let pos8_0: U32 = *hash8Table.offset(h8_0 as isize);
                                    let m2Distance: U32 =
                                        ipIndex.wrapping_add(1 as U32).wrapping_sub(pos8_0);
                                    matchDistance =
                                        ipIndex.wrapping_sub(pos4) as c_uint;
                                    if m2Distance <= LZ4_DISTANCE_MAX as U32
                                        && pos8_0 >= prefixIdx
                                        && ((ip_0 < mflimit) as c_int
                                            != 0 as c_int)
                                            as c_int
                                            as c_long
                                            != 0
                                    {
                                        let m2Ptr: *const BYTE = prefixPtr
                                            .offset(pos8_0.wrapping_sub(prefixIdx) as isize);
                                        let mut ml2: c_uint = LZ4_count(
                                            ip_0.offset(1 as c_int as isize),
                                            m2Ptr,
                                            matchlimit,
                                        );
                                        if ml2 > matchLength {
                                            LZ4MID_addPosition(
                                                hash8Table,
                                                h8_0,
                                                ipIndex.wrapping_add(1 as U32),
                                            );
                                            ip_0 = ip_0.offset(1);
                                            matchLength = ml2;
                                            matchDistance = m2Distance as c_uint;
                                        }
                                    }
                                    current_block = 2714784867065232716;
                                } else {
                                    current_block = 3546145585875536353;
                                }
                            } else if pos4 >= dictIdx {
                                let matchPtr_2: *const BYTE =
                                    dictStart.offset(pos4.wrapping_sub(dictIdx) as isize);
                                let safeLen_0: size_t = if (prefixIdx.wrapping_sub(pos4) as size_t)
                                    < matchlimit.offset_from(ip_0) as c_long as size_t
                                {
                                    prefixIdx.wrapping_sub(pos4) as size_t
                                } else {
                                    matchlimit.offset_from(ip_0) as c_long as size_t
                                };
                                matchLength =
                                    LZ4_count(ip_0, matchPtr_2, ip_0.offset(safeLen_0 as isize));
                                if matchLength >= MINMATCH as c_uint {
                                    matchDistance =
                                        ipIndex.wrapping_sub(pos4) as c_uint;
                                    current_block = 2714784867065232716;
                                } else {
                                    current_block = 3546145585875536353;
                                }
                            } else {
                                current_block = 3546145585875536353;
                            }
                        } else {
                            current_block = 3546145585875536353;
                        }
                        match current_block {
                            2714784867065232716 => {}
                            _ => {
                                if dict as c_uint
                                    == usingDictCtxHc as c_int as c_uint
                                    && ipIndex.wrapping_sub(gDictEndIndex)
                                        < (LZ4_DISTANCE_MAX - 8 as c_int) as U32
                                {
                                    let mut dMatch: LZ4HC_match_t = searchIntoDict
                                        .expect("non-null function pointer")(
                                        ip_0,
                                        ipIndex,
                                        matchlimit,
                                        (*ctx).dictCtx,
                                        gDictEndIndex,
                                    );
                                    if dMatch.len >= MINMATCH {
                                        matchLength = dMatch.len as c_uint;
                                        matchDistance = dMatch.off as c_uint;
                                        current_block = 2714784867065232716;
                                    } else {
                                        current_block = 13660591889533726445;
                                    }
                                } else {
                                    current_block = 13660591889533726445;
                                }
                                match current_block {
                                    2714784867065232716 => {}
                                    _ => {
                                        ip_0 = ip_0.offset(
                                            (1 as c_long
                                                + (ip_0.offset_from(anchor_0)
                                                    as c_long
                                                    >> 9 as c_int))
                                                as isize,
                                        );
                                        current_block = 15976848397966268834;
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                    _ => {}
                }
                while (ip_0 > anchor_0) as c_int
                    & (ip_0.offset_from(prefixPtr) as c_long as U32
                        > matchDistance as U32) as c_int
                    != 0
                    && ((*ip_0.offset(-(1 as c_int) as isize) as c_int
                        == *ip_0.offset(
                            (-(matchDistance as c_int) - 1 as c_int)
                                as isize,
                        ) as c_int) as c_int
                        != 0 as c_int) as c_int
                        as c_long
                        != 0
                {
                    ip_0 = ip_0.offset(-1);
                    matchLength = matchLength.wrapping_add(1);
                }
                LZ4MID_addPosition(
                    hash8Table,
                    LZ4MID_hash8Ptr(
                        ip_0.offset(1 as c_int as isize) as *const c_void
                    ),
                    ipIndex.wrapping_add(1 as U32),
                );
                LZ4MID_addPosition(
                    hash8Table,
                    LZ4MID_hash8Ptr(
                        ip_0.offset(2 as c_int as isize) as *const c_void
                    ),
                    ipIndex.wrapping_add(2 as U32),
                );
                LZ4MID_addPosition(
                    hash4Table,
                    LZ4MID_hash4Ptr(
                        ip_0.offset(1 as c_int as isize) as *const c_void
                    ),
                    ipIndex.wrapping_add(1 as U32),
                );
                let saved_op: *mut BYTE = op_0;
                if LZ4HC_encodeSequence(
                    &raw mut ip_0,
                    &raw mut op_0,
                    &raw mut anchor_0,
                    matchLength as c_int,
                    matchDistance as c_int,
                    limit,
                    oend,
                ) != 0
                {
                    op_0 = saved_op;
                    if limit as c_uint
                        == fillOutput as c_int as c_uint
                    {
                        let ll: size_t =
                            ip_0.offset_from(anchor_0) as c_long as size_t;
                        let ll_addbytes: size_t =
                            ll.wrapping_add(240 as size_t).wrapping_div(255 as size_t);
                        let ll_totalCost: size_t =
                            (1 as size_t).wrapping_add(ll_addbytes).wrapping_add(ll);
                        let maxLitPos: *mut BYTE = oend.offset(-(3 as c_int as isize));
                        if op_0.offset(ll_totalCost as isize) <= maxLitPos {
                            let bytesLeftForMl: size_t = maxLitPos
                                .offset_from(op_0.offset(ll_totalCost as isize))
                                as c_long
                                as size_t;
                            let maxMlSize: size_t = ((MINMATCH as c_uint)
                                .wrapping_add(ML_MASK.wrapping_sub(1 as c_uint))
                                as size_t)
                                .wrapping_add(bytesLeftForMl.wrapping_mul(255 as size_t));
                            if matchLength as size_t > maxMlSize {
                                matchLength = maxMlSize as c_uint;
                            }
                            if oend.offset(LASTLITERALS as isize).offset_from(
                                op_0.offset(ll_totalCost as isize)
                                    .offset(2 as c_int as isize),
                            ) as c_long
                                - 1 as c_long
                                + matchLength as c_long
                                >= MFLIMIT as c_long
                            {
                                LZ4HC_encodeSequence(
                                    &raw mut ip_0,
                                    &raw mut op_0,
                                    &raw mut anchor_0,
                                    matchLength as c_int,
                                    matchDistance as c_int,
                                    notLimited,
                                    oend,
                                );
                            }
                        }
                        current_block = 17215225966118807657;
                    } else {
                        return 0 as c_int;
                    }
                } else {
                    let mut endMatchIdx: U32 = (ip_0.offset_from(prefixPtr) as c_long
                        as U32)
                        .wrapping_add(prefixIdx);
                    let mut pos_m2: U32 = endMatchIdx.wrapping_sub(2 as U32);
                    if pos_m2 < ilimitIdx {
                        if ((ip_0.offset_from(prefixPtr) as c_long
                            > 5 as c_long)
                            as c_int
                            != 0 as c_int)
                            as c_int as c_long
                            != 0
                        {
                            LZ4MID_addPosition(
                                hash8Table,
                                LZ4MID_hash8Ptr(ip_0.offset(-(5 as c_int as isize))
                                    as *const c_void),
                                endMatchIdx.wrapping_sub(5 as U32),
                            );
                        }
                        LZ4MID_addPosition(
                            hash8Table,
                            LZ4MID_hash8Ptr(ip_0.offset(-(3 as c_int as isize))
                                as *const c_void),
                            endMatchIdx.wrapping_sub(3 as U32),
                        );
                        LZ4MID_addPosition(
                            hash8Table,
                            LZ4MID_hash8Ptr(ip_0.offset(-(2 as c_int as isize))
                                as *const c_void),
                            endMatchIdx.wrapping_sub(2 as U32),
                        );
                        LZ4MID_addPosition(
                            hash4Table,
                            LZ4MID_hash4Ptr(ip_0.offset(-(2 as c_int as isize))
                                as *const c_void),
                            endMatchIdx.wrapping_sub(2 as U32),
                        );
                        LZ4MID_addPosition(
                            hash4Table,
                            LZ4MID_hash4Ptr(ip_0.offset(-(1 as c_int as isize))
                                as *const c_void),
                            endMatchIdx.wrapping_sub(1 as U32),
                        );
                    }
                    current_block = 15976848397966268834;
                }
            }
            _ => {
                let mut lastRunSize: size_t =
                    iend.offset_from(anchor_0) as c_long as size_t;
                let mut llAdd: size_t = lastRunSize
                    .wrapping_add(255 as size_t)
                    .wrapping_sub(RUN_MASK as size_t)
                    .wrapping_div(255 as size_t);
                let totalSize: size_t = (1 as size_t).wrapping_add(llAdd).wrapping_add(lastRunSize);
                if limit as c_uint
                    == fillOutput as c_int as c_uint
                {
                    oend = oend.offset(LASTLITERALS as isize);
                }
                if limit as c_uint != 0 && op_0.offset(totalSize as isize) > oend {
                    if limit as c_uint
                        == limitedOutput as c_int as c_uint
                    {
                        return 0 as c_int;
                    }
                    lastRunSize = (oend.offset_from(op_0) as c_long as size_t)
                        .wrapping_sub(1 as size_t);
                    llAdd = lastRunSize
                        .wrapping_add(256 as size_t)
                        .wrapping_sub(RUN_MASK as size_t)
                        .wrapping_div(256 as size_t);
                    lastRunSize = lastRunSize.wrapping_sub(llAdd);
                }
                ip_0 = anchor_0.offset(lastRunSize as isize);
                if lastRunSize >= RUN_MASK as size_t {
                    let mut accumulator: size_t = lastRunSize.wrapping_sub(RUN_MASK as size_t);
                    let fresh17 = op_0;
                    op_0 = op_0.offset(1);
                    *fresh17 = (RUN_MASK << ML_BITS) as BYTE;
                    while accumulator >= 255 as size_t {
                        let fresh18 = op_0;
                        op_0 = op_0.offset(1);
                        *fresh18 = 255 as BYTE;
                        accumulator = accumulator.wrapping_sub(255 as size_t);
                    }
                    let fresh19 = op_0;
                    op_0 = op_0.offset(1);
                    *fresh19 = accumulator as BYTE;
                } else {
                    let fresh20 = op_0;
                    op_0 = op_0.offset(1);
                    *fresh20 = (lastRunSize << ML_BITS) as BYTE;
                }
                ::libc::memcpy(
                    op_0 as *mut c_void,
                    anchor_0 as *const c_void,
                    lastRunSize as ::libc::size_t,
                );
                op_0 = op_0.offset(lastRunSize as isize);
                *srcSizePtr_view = ip_0.offset_from(src as *const BYTE) as c_long
                    as c_int;
                return (op_0 as *mut c_char).offset_from(dst) as c_long
                    as c_int;
            }
        }
    }
}
#[inline(always)]
unsafe fn LZ4HC_Insert(mut hc4: *mut LZ4HC_CCtx_internal, mut ip_0: *const BYTE) {
    let chainTable: *mut U16 = &raw mut (*hc4).chainTable as *mut U16;
    let hashTable: *mut U32 = &raw mut (*hc4).hashTable as *mut U32;
    let prefixPtr: *const BYTE = (*hc4).prefixStart as *const BYTE;
    let prefixIdx: U32 = (*hc4).dictLimit as U32;
    let target: U32 =
        (ip_0.offset_from(prefixPtr) as c_long as U32).wrapping_add(prefixIdx);
    let mut idx: U32 = (*hc4).nextToUpdate as U32;
    while idx < target {
        let h: U32 = LZ4HC_hashPtr(prefixPtr.offset(idx as isize).offset(-(prefixIdx as isize))
            as *const c_void) as U32;
        let mut delta: size_t = idx.wrapping_sub(*hashTable.offset(h as isize)) as size_t;
        if delta > LZ4_DISTANCE_MAX as size_t {
            delta = LZ4_DISTANCE_MAX as size_t;
        }
        *chainTable.offset(idx as U16 as isize) = delta as U16;
        *hashTable.offset(h as isize) = idx;
        idx = idx.wrapping_add(1);
    }
    (*hc4).nextToUpdate = target as LZ4_u32;
}
fn LZ4HC_rotatePattern(rotate: size_t, pattern: U32) -> U32 { {
    let bitsToRotate: size_t = (rotate
        & (::core::mem::size_of::<U32>() as size_t).wrapping_sub(1 as size_t))
        << 3 as c_int;
    if bitsToRotate == 0 as size_t {
        return pattern;
    }
    return pattern << bitsToRotate as c_int
        | pattern >> 32 as c_int - bitsToRotate as c_int;
} }
unsafe fn LZ4HC_countPattern(
    mut ip_0: *const BYTE,
    iEnd: *const BYTE,
    pattern32: U32,
) -> c_uint {
    let iStart: *const BYTE = ip_0;
    let pattern: reg_t = if ::core::mem::size_of::<reg_t>() as usize == 8 as usize {
        (pattern32 as reg_t).wrapping_add(
            (pattern32 as reg_t)
                << (::core::mem::size_of::<reg_t>() as usize).wrapping_mul(4 as usize),
        )
    } else {
        pattern32 as reg_t
    };
    while ((ip_0
        < iEnd.offset(
            -((::core::mem::size_of::<reg_t>() as usize).wrapping_sub(1 as usize) as isize),
        )) as c_int
        != 0 as c_int) as c_int as c_long
        != 0
    {
        let diff: reg_t = LZ4_read_ARCH(ip_0 as *const c_void) as reg_t ^ pattern;
        if diff == 0 {
            ip_0 = ip_0.offset(::core::mem::size_of::<reg_t>() as usize as isize);
        } else {
            ip_0 = ip_0.offset(LZ4_NbCommonBytes(diff) as isize);
            return ip_0.offset_from(iStart) as c_long as c_uint;
        }
    }
    if LZ4_isLittleEndian() != 0 {
        let mut patternByte: reg_t = pattern;
        while ip_0 < iEnd
            && *ip_0 as c_int == patternByte as BYTE as c_int
        {
            ip_0 = ip_0.offset(1);
            patternByte >>= 8 as c_int;
        }
    } else {
        let mut bitOffset: U32 = (::core::mem::size_of::<reg_t>() as usize)
            .wrapping_mul(8 as usize)
            .wrapping_sub(8 as usize) as U32;
        while ip_0 < iEnd {
            let byte: BYTE = (pattern >> bitOffset) as BYTE;
            if *ip_0 as c_int != byte as c_int {
                break;
            }
            ip_0 = ip_0.offset(1);
            bitOffset = bitOffset.wrapping_sub(8 as U32);
        }
    }
    return ip_0.offset_from(iStart) as c_long as c_uint;
}
unsafe fn LZ4HC_reverseCountPattern(
    mut ip_0: *const BYTE,
    iLow: *const BYTE,
    mut pattern: U32,
) -> c_uint {
    let iStart: *const BYTE = ip_0;
    while ((ip_0 >= iLow.offset(4 as c_int as isize)) as c_int
        != 0 as c_int) as c_int as c_long
        != 0
    {
        if LZ4_read32(ip_0.offset(-(4 as c_int as isize)) as *const c_void)
            != pattern
        {
            break;
        }
        ip_0 = ip_0.offset(-(4 as c_int as isize));
    }
    let mut bytePtr: *const BYTE =
        (&raw mut pattern as *const BYTE).offset(3 as c_int as isize);
    while ((ip_0 > iLow) as c_int != 0 as c_int) as c_int
        as c_long
        != 0
    {
        if *ip_0.offset(-(1 as c_int) as isize) as c_int
            != *bytePtr as c_int
        {
            break;
        }
        ip_0 = ip_0.offset(-1);
        bytePtr = bytePtr.offset(-1);
    }
    return iStart.offset_from(ip_0) as c_long as c_uint;
}
fn LZ4HC_protectDictEnd(dictLimit: U32, matchIndex: U32) -> c_int { {
    return (dictLimit.wrapping_sub(1 as U32).wrapping_sub(matchIndex) >= 3 as U32)
        as c_int;
} }
#[inline(always)]
unsafe fn LZ4HC_InsertAndGetWiderMatch(
    hc4: *mut LZ4HC_CCtx_internal,
    ip_0: *const BYTE,
    iLowLimit: *const BYTE,
    iHighLimit: *const BYTE,
    mut longest: c_int,
    maxNbAttempts: c_int,
    patternAnalysis: c_int,
    chainSwap: c_int,
    dict: dictCtx_directive,
    favorDecSpeed: HCfavor_e,
) -> LZ4HC_match_t {
    let chainTable: *mut U16 = &raw mut (*hc4).chainTable as *mut U16;
    let hashTable: *mut U32 = &raw mut (*hc4).hashTable as *mut U32;
    let dictCtx: *const LZ4HC_CCtx_internal = (*hc4).dictCtx;
    let prefixPtr: *const BYTE = (*hc4).prefixStart as *const BYTE;
    let prefixIdx: U32 = (*hc4).dictLimit as U32;
    let ipIndex: U32 =
        (ip_0.offset_from(prefixPtr) as c_long as U32).wrapping_add(prefixIdx);
    let withinStartDistance: c_int = ((*hc4)
        .lowLimit
        .wrapping_add((LZ4_DISTANCE_MAX + 1 as c_int) as LZ4_u32)
        > ipIndex) as c_int;
    let lowestMatchIndex: U32 = if withinStartDistance != 0 {
        (*hc4).lowLimit as U32
    } else {
        ipIndex.wrapping_sub(LZ4_DISTANCE_MAX as U32)
    };
    let dictStart: *const BYTE = (*hc4).dictStart as *const BYTE;
    let dictIdx: U32 = (*hc4).lowLimit as U32;
    let dictEnd: *const BYTE = dictStart
        .offset(prefixIdx as isize)
        .offset(-(dictIdx as isize));
    let lookBackLength: c_int =
        ip_0.offset_from(iLowLimit) as c_long as c_int;
    let mut nbAttempts: c_int = maxNbAttempts;
    let mut matchChainPos: U32 = 0 as U32;
    let pattern: U32 = LZ4_read32(ip_0 as *const c_void) as U32;
    let mut matchIndex: U32 = 0;
    let mut repeat: repeat_state_e = rep_untested;
    let mut srcPatternLength: size_t = 0 as size_t;
    let mut offset: c_int = 0 as c_int;
    let mut sBack: c_int = 0 as c_int;
    LZ4HC_Insert(hc4, ip_0);
    matchIndex = *hashTable.offset(LZ4HC_hashPtr(ip_0 as *const c_void) as isize);
    while matchIndex >= lowestMatchIndex && nbAttempts > 0 as c_int {
        let mut matchLength: c_int = 0 as c_int;
        nbAttempts -= 1;
        if !(favorDecSpeed as c_uint != 0
            && ipIndex.wrapping_sub(matchIndex) < 8 as U32)
        {
            if matchIndex >= prefixIdx {
                let matchPtr: *const BYTE =
                    prefixPtr.offset(matchIndex.wrapping_sub(prefixIdx) as isize);
                if LZ4_read16(
                    iLowLimit
                        .offset(longest as isize)
                        .offset(-(1 as c_int as isize))
                        as *const c_void,
                ) as c_int
                    == LZ4_read16(
                        matchPtr
                            .offset(-(lookBackLength as isize))
                            .offset(longest as isize)
                            .offset(-(1 as c_int as isize))
                            as *const c_void,
                    ) as c_int
                {
                    if LZ4_read32(matchPtr as *const c_void) == pattern {
                        let back: c_int = if lookBackLength != 0 {
                            LZ4HC_countBack(ip_0, matchPtr, iLowLimit, prefixPtr)
                                as c_int
                        } else {
                            0 as c_int
                        };
                        matchLength = MINMATCH
                            + LZ4_count(
                                ip_0.offset(MINMATCH as isize),
                                matchPtr.offset(MINMATCH as isize),
                                iHighLimit,
                            ) as c_int;
                        matchLength -= back;
                        if matchLength > longest {
                            longest = matchLength;
                            offset = ipIndex.wrapping_sub(matchIndex) as c_int;
                            sBack = back;
                        }
                    }
                }
            } else {
                let matchPtr_0: *const BYTE =
                    dictStart.offset(matchIndex.wrapping_sub(dictIdx) as isize);
                if ((matchIndex <= prefixIdx.wrapping_sub(4 as U32)) as c_int
                    != 0 as c_int) as c_int
                    as c_long
                    != 0
                    && LZ4_read32(matchPtr_0 as *const c_void) == pattern
                {
                    let mut back_0: c_int = 0 as c_int;
                    let mut vLimit: *const BYTE =
                        ip_0.offset(prefixIdx.wrapping_sub(matchIndex) as isize);
                    if vLimit > iHighLimit {
                        vLimit = iHighLimit;
                    }
                    matchLength = LZ4_count(
                        ip_0.offset(MINMATCH as isize),
                        matchPtr_0.offset(MINMATCH as isize),
                        vLimit,
                    ) as c_int
                        + MINMATCH;
                    if ip_0.offset(matchLength as isize) == vLimit && vLimit < iHighLimit {
                        matchLength = (matchLength as c_uint).wrapping_add(LZ4_count(
                            ip_0.offset(matchLength as isize),
                            prefixPtr,
                            iHighLimit,
                        )) as c_int
                            as c_int;
                    }
                    back_0 = if lookBackLength != 0 {
                        LZ4HC_countBack(ip_0, matchPtr_0, iLowLimit, dictStart)
                    } else {
                        0 as c_int
                    };
                    matchLength -= back_0;
                    if matchLength > longest {
                        longest = matchLength;
                        offset = ipIndex.wrapping_sub(matchIndex) as c_int;
                        sBack = back_0;
                    }
                }
            }
        }
        if chainSwap != 0 && matchLength == longest {
            if matchIndex.wrapping_add(longest as U32) <= ipIndex {
                let kTrigger: c_int = 4 as c_int;
                let mut distanceToNextMatch: U32 = 1 as U32;
                let end: c_int = longest - MINMATCH + 1 as c_int;
                let mut step: c_int = 1 as c_int;
                let mut accel: c_int = (1 as c_int) << kTrigger;
                let mut pos: c_int = 0;
                pos = 0 as c_int;
                while pos < end {
                    let candidateDist: U32 = *chainTable
                        .offset(matchIndex.wrapping_add(pos as U32) as U16 as isize)
                        as U32;
                    let fresh11 = accel;
                    accel = accel + 1;
                    step = fresh11 >> kTrigger;
                    if candidateDist > distanceToNextMatch {
                        distanceToNextMatch = candidateDist;
                        matchChainPos = pos as U32;
                        accel = (1 as c_int) << kTrigger;
                    }
                    pos += step;
                }
                if distanceToNextMatch > 1 as U32 {
                    if distanceToNextMatch > matchIndex {
                        break;
                    }
                    matchIndex = matchIndex.wrapping_sub(distanceToNextMatch);
                    continue;
                }
            }
        }
        let distNextMatch: U32 = *chainTable.offset(matchIndex as U16 as isize) as U32;
        if patternAnalysis != 0 && distNextMatch == 1 as U32 && matchChainPos == 0 as U32 {
            let matchCandidateIdx: U32 = matchIndex.wrapping_sub(1 as U32);
            if repeat as c_uint
                == rep_untested as c_int as c_uint
            {
                if (pattern & 0xffff as U32 == pattern >> 16 as c_int)
                    as c_int
                    & (pattern & 0xff as U32 == pattern >> 24 as c_int)
                        as c_int
                    != 0
                {
                    repeat = rep_confirmed;
                    srcPatternLength = (LZ4HC_countPattern(
                        ip_0.offset(::core::mem::size_of::<U32>() as usize as isize),
                        iHighLimit,
                        pattern,
                    ) as usize)
                        .wrapping_add(::core::mem::size_of::<U32>() as usize)
                        as size_t;
                } else {
                    repeat = rep_not;
                }
            }
            if repeat as c_uint
                == rep_confirmed as c_int as c_uint
                && matchCandidateIdx >= lowestMatchIndex
                && LZ4HC_protectDictEnd(prefixIdx, matchCandidateIdx) != 0
            {
                let extDict: c_int =
                    (matchCandidateIdx < prefixIdx) as c_int;
                let matchPtr_1: *const BYTE = if extDict != 0 {
                    dictStart.offset(matchCandidateIdx.wrapping_sub(dictIdx) as isize)
                } else {
                    prefixPtr.offset(matchCandidateIdx.wrapping_sub(prefixIdx) as isize)
                };
                if LZ4_read32(matchPtr_1 as *const c_void) == pattern {
                    let iLimit: *const BYTE = if extDict != 0 { dictEnd } else { iHighLimit };
                    let mut forwardPatternLength: size_t = (LZ4HC_countPattern(
                        matchPtr_1.offset(::core::mem::size_of::<U32>() as usize as isize),
                        iLimit,
                        pattern,
                    ) as size_t)
                        .wrapping_add(::core::mem::size_of::<U32>() as size_t);
                    if extDict != 0 && matchPtr_1.offset(forwardPatternLength as isize) == iLimit {
                        let rotatedPattern: U32 =
                            LZ4HC_rotatePattern(forwardPatternLength, pattern) as U32;
                        forwardPatternLength = forwardPatternLength.wrapping_add(
                            LZ4HC_countPattern(prefixPtr, iHighLimit, rotatedPattern) as size_t,
                        );
                    }
                    let lowestMatchPtr: *const BYTE =
                        if extDict != 0 { dictStart } else { prefixPtr };
                    let mut backLength: size_t =
                        LZ4HC_reverseCountPattern(matchPtr_1, lowestMatchPtr, pattern) as size_t;
                    let mut currentSegmentLength: size_t = 0;
                    if extDict == 0
                        && matchPtr_1.offset(-(backLength as isize)) == prefixPtr
                        && dictIdx < prefixIdx
                    {
                        let rotatedPattern_0: U32 = LZ4HC_rotatePattern(
                            -(backLength as c_int) as U32 as size_t,
                            pattern,
                        ) as U32;
                        backLength = backLength.wrapping_add(LZ4HC_reverseCountPattern(
                            dictEnd,
                            dictStart,
                            rotatedPattern_0,
                        ) as size_t);
                    }
                    backLength = matchCandidateIdx.wrapping_sub(
                        (if matchCandidateIdx.wrapping_sub(backLength as U32) > lowestMatchIndex {
                            matchCandidateIdx.wrapping_sub(backLength as U32)
                        } else {
                            lowestMatchIndex
                        }),
                    ) as size_t;
                    currentSegmentLength = backLength.wrapping_add(forwardPatternLength);
                    if currentSegmentLength >= srcPatternLength
                        && forwardPatternLength <= srcPatternLength
                    {
                        let newMatchIndex: U32 = matchCandidateIdx
                            .wrapping_add(forwardPatternLength as U32)
                            .wrapping_sub(srcPatternLength as U32);
                        if LZ4HC_protectDictEnd(prefixIdx, newMatchIndex) != 0 {
                            matchIndex = newMatchIndex;
                        } else {
                            matchIndex = prefixIdx;
                        }
                        continue;
                    } else {
                        let newMatchIndex_0: U32 =
                            matchCandidateIdx.wrapping_sub(backLength as U32);
                        if LZ4HC_protectDictEnd(prefixIdx, newMatchIndex_0) == 0 {
                            matchIndex = prefixIdx;
                            continue;
                        } else {
                            matchIndex = newMatchIndex_0;
                            if !(lookBackLength == 0 as c_int) {
                                continue;
                            }
                            let maxML: size_t = if currentSegmentLength < srcPatternLength {
                                currentSegmentLength
                            } else {
                                srcPatternLength
                            };
                            if (longest as size_t) < maxML {
                                if (ip_0.offset_from(prefixPtr) as c_long as size_t)
                                    .wrapping_add(prefixIdx as size_t)
                                    .wrapping_sub(matchIndex as size_t)
                                    > LZ4_DISTANCE_MAX as size_t
                                {
                                    break;
                                }
                                longest = maxML as c_int;
                                offset = ipIndex.wrapping_sub(matchIndex) as c_int;
                            }
                            let distToNextPattern: U32 =
                                *chainTable.offset(matchIndex as U16 as isize) as U32;
                            if distToNextPattern > matchIndex {
                                break;
                            }
                            matchIndex = matchIndex.wrapping_sub(distToNextPattern);
                            continue;
                        }
                    }
                }
            }
        }
        matchIndex = matchIndex.wrapping_sub(
            *chainTable.offset(matchIndex.wrapping_add(matchChainPos) as U16 as isize) as U32,
        );
    }
    if dict as c_uint == usingDictCtxHc as c_int as c_uint
        && nbAttempts > 0 as c_int
        && withinStartDistance != 0
    {
        let dictEndOffset: size_t = ((*dictCtx).end.offset_from((*dictCtx).prefixStart)
            as c_long as size_t)
            .wrapping_add((*dictCtx).dictLimit as size_t);
        let mut dictMatchIndex: U32 =
            (*dictCtx).hashTable[LZ4HC_hashPtr(ip_0 as *const c_void) as usize];
        matchIndex = dictMatchIndex
            .wrapping_add(lowestMatchIndex)
            .wrapping_sub(dictEndOffset as U32);
        dictMatchIndex > 0 as U32;
        while ipIndex.wrapping_sub(matchIndex) <= LZ4_DISTANCE_MAX as U32 && {
            let fresh12 = nbAttempts;
            nbAttempts = nbAttempts - 1;
            fresh12 != 0
        } {
            let matchPtr_2: *const BYTE = (*dictCtx)
                .prefixStart
                .offset(-((*dictCtx).dictLimit as isize))
                .offset(dictMatchIndex as isize);
            if LZ4_read32(matchPtr_2 as *const c_void) == pattern {
                let mut mlt: c_int = 0;
                let mut back_1: c_int = 0 as c_int;
                let mut vLimit_0: *const BYTE =
                    ip_0.offset(dictEndOffset.wrapping_sub(dictMatchIndex as size_t) as isize);
                if vLimit_0 > iHighLimit {
                    vLimit_0 = iHighLimit;
                }
                mlt = LZ4_count(
                    ip_0.offset(MINMATCH as isize),
                    matchPtr_2.offset(MINMATCH as isize),
                    vLimit_0,
                ) as c_int
                    + MINMATCH;
                back_1 = if lookBackLength != 0 {
                    LZ4HC_countBack(
                        ip_0,
                        matchPtr_2,
                        iLowLimit,
                        (*dictCtx).prefixStart as *const BYTE,
                    )
                } else {
                    0 as c_int
                };
                mlt -= back_1;
                if mlt > longest {
                    longest = mlt;
                    offset = ipIndex.wrapping_sub(matchIndex) as c_int;
                    sBack = back_1;
                }
            }
            let nextOffset: U32 = (*dictCtx).chainTable[dictMatchIndex as U16 as usize] as U32;
            dictMatchIndex = dictMatchIndex.wrapping_sub(nextOffset);
            matchIndex = matchIndex.wrapping_sub(nextOffset);
        }
    }
    let mut md: LZ4HC_match_t = LZ4HC_match_t {
        off: 0,
        len: 0,
        back: 0,
    };
    md.len = longest;
    md.off = offset;
    md.back = sBack;
    return md;
}
#[inline(always)]
unsafe fn LZ4HC_InsertAndFindBestMatch(
    hc4: *mut LZ4HC_CCtx_internal,
    ip_0: *const BYTE,
    iLimit: *const BYTE,
    maxNbAttempts: c_int,
    patternAnalysis: c_int,
    dict: dictCtx_directive,
) -> LZ4HC_match_t {
    return LZ4HC_InsertAndGetWiderMatch(
        hc4,
        ip_0,
        ip_0,
        iLimit,
        MINMATCH - 1 as c_int,
        maxNbAttempts,
        patternAnalysis,
        0 as c_int,
        dict,
        favorCompressionRatio,
    );
}
#[inline(always)]
unsafe fn LZ4HC_compress_hashChain(
    ctx: *mut LZ4HC_CCtx_internal,
    src: *const c_char,
    dst: *mut c_char,
    mut srcSizePtr: *mut c_int,
    maxOutputSize: c_int,
    mut maxNbAttempts: c_int,
    limit: limitedOutput_directive,
    dict: dictCtx_directive,
) -> c_int {
    let mut current_block: u64;
    let inputSize: c_int = *srcSizePtr;
    let patternAnalysis: c_int =
        (maxNbAttempts > 128 as c_int) as c_int;
    let mut ip_0: *const BYTE = src as *const BYTE;
    let mut anchor_0: *const BYTE = ip_0;
    let iend: *const BYTE = ip_0.offset(inputSize as isize);
    let mflimit: *const BYTE = iend.offset(-(MFLIMIT as isize));
    let matchlimit: *const BYTE = iend.offset(-(LASTLITERALS as isize));
    let mut optr: *mut BYTE = dst as *mut BYTE;
    let mut op_0: *mut BYTE = dst as *mut BYTE;
    let mut oend: *mut BYTE = op_0.offset(maxOutputSize as isize);
    let mut start0: *const BYTE = ::core::ptr::null::<BYTE>();
    let mut start2: *const BYTE = ::core::ptr::null::<BYTE>();
    let mut start3: *const BYTE = ::core::ptr::null::<BYTE>();
    let mut m0: LZ4HC_match_t = LZ4HC_match_t {
        off: 0,
        len: 0,
        back: 0,
    };
    let mut m1: LZ4HC_match_t = LZ4HC_match_t {
        off: 0,
        len: 0,
        back: 0,
    };
    let mut m2: LZ4HC_match_t = LZ4HC_match_t {
        off: 0,
        len: 0,
        back: 0,
    };
    let mut m3: LZ4HC_match_t = LZ4HC_match_t {
        off: 0,
        len: 0,
        back: 0,
    };
    let nomatch: LZ4HC_match_t = LZ4HC_match_t {
        off: 0 as c_int,
        len: 0 as c_int,
        back: 0 as c_int,
    };
    *srcSizePtr = 0 as c_int;
    if limit as c_uint == fillOutput as c_int as c_uint {
        oend = oend.offset(-(LASTLITERALS as isize));
    }
    if inputSize < LZ4_minLength {
        current_block = 1525518218708800755;
    } else {
        current_block = 11050875288958768710;
    }
    '__last_literals: loop {
        match current_block {
            11050875288958768710 => {
                if !(ip_0 <= mflimit) {
                    current_block = 1525518218708800755;
                    continue;
                }
                m1 = LZ4HC_InsertAndFindBestMatch(
                    ctx,
                    ip_0,
                    matchlimit,
                    maxNbAttempts,
                    patternAnalysis,
                    dict,
                );
                if m1.len < MINMATCH {
                    ip_0 = ip_0.offset(1);
                    current_block = 11050875288958768710;
                } else {
                    start0 = ip_0;
                    m0 = m1;
                    's_97: loop {
                        if ip_0.offset(m1.len as isize) <= mflimit {
                            start2 = ip_0
                                .offset(m1.len as isize)
                                .offset(-(2 as c_int as isize));
                            m2 = LZ4HC_InsertAndGetWiderMatch(
                                ctx,
                                start2,
                                ip_0.offset(0 as c_int as isize),
                                matchlimit,
                                m1.len,
                                maxNbAttempts,
                                patternAnalysis,
                                0 as c_int,
                                dict,
                                favorCompressionRatio,
                            );
                            start2 = start2.offset(m2.back as isize);
                        } else {
                            m2 = nomatch;
                        }
                        if m2.len <= m1.len {
                            optr = op_0;
                            if LZ4HC_encodeSequence(
                                &raw mut ip_0,
                                &raw mut op_0,
                                &raw mut anchor_0,
                                m1.len,
                                m1.off,
                                limit,
                                oend,
                            ) != 0
                            {
                                current_block = 1719972829996836699;
                                break;
                            } else {
                                current_block = 11050875288958768710;
                                continue '__last_literals;
                            }
                        } else {
                            if start0 < ip_0 {
                                if start2 < ip_0.offset(m0.len as isize) {
                                    ip_0 = start0;
                                    m1 = m0;
                                }
                            }
                            if (start2.offset_from(ip_0) as c_long)
                                < 3 as c_long
                            {
                                ip_0 = start2;
                                m1 = m2;
                            } else {
                                loop {
                                    if (start2.offset_from(ip_0) as c_long)
                                        < OPTIMAL_ML as c_long
                                    {
                                        let mut correction: c_int = 0;
                                        let mut new_ml: c_int = m1.len;
                                        if new_ml > OPTIMAL_ML {
                                            new_ml = OPTIMAL_ML;
                                        }
                                        if ip_0.offset(new_ml as isize)
                                            > start2
                                                .offset(m2.len as isize)
                                                .offset(-(MINMATCH as isize))
                                        {
                                            new_ml = start2.offset_from(ip_0) as c_long
                                                as c_int
                                                + m2.len
                                                - MINMATCH;
                                        }
                                        correction = new_ml
                                            - start2.offset_from(ip_0) as c_long
                                                as c_int;
                                        if correction > 0 as c_int {
                                            start2 = start2.offset(correction as isize);
                                            m2.len -= correction;
                                        }
                                    }
                                    if start2.offset(m2.len as isize) <= mflimit {
                                        start3 = start2
                                            .offset(m2.len as isize)
                                            .offset(-(3 as c_int as isize));
                                        m3 = LZ4HC_InsertAndGetWiderMatch(
                                            ctx,
                                            start3,
                                            start2,
                                            matchlimit,
                                            m2.len,
                                            maxNbAttempts,
                                            patternAnalysis,
                                            0 as c_int,
                                            dict,
                                            favorCompressionRatio,
                                        );
                                        start3 = start3.offset(m3.back as isize);
                                    } else {
                                        m3 = nomatch;
                                    }
                                    if m3.len <= m2.len {
                                        if start2 < ip_0.offset(m1.len as isize) {
                                            m1.len = start2.offset_from(ip_0) as c_long
                                                as c_int;
                                        }
                                        optr = op_0;
                                        if LZ4HC_encodeSequence(
                                            &raw mut ip_0,
                                            &raw mut op_0,
                                            &raw mut anchor_0,
                                            m1.len,
                                            m1.off,
                                            limit,
                                            oend,
                                        ) != 0
                                        {
                                            current_block = 1719972829996836699;
                                            break 's_97;
                                        } else {
                                            current_block = 2290177392965769716;
                                            break 's_97;
                                        }
                                    } else if start3
                                        < ip_0
                                            .offset(m1.len as isize)
                                            .offset(3 as c_int as isize)
                                    {
                                        if start3 >= ip_0.offset(m1.len as isize) {
                                            if start2 < ip_0.offset(m1.len as isize) {
                                                let mut correction_0: c_int = ip_0
                                                    .offset(m1.len as isize)
                                                    .offset_from(start2)
                                                    as c_long
                                                    as c_int;
                                                start2 = start2.offset(correction_0 as isize);
                                                m2.len -= correction_0;
                                                if m2.len < MINMATCH {
                                                    start2 = start3;
                                                    m2 = m3;
                                                }
                                            }
                                            optr = op_0;
                                            if LZ4HC_encodeSequence(
                                                &raw mut ip_0,
                                                &raw mut op_0,
                                                &raw mut anchor_0,
                                                m1.len,
                                                m1.off,
                                                limit,
                                                oend,
                                            ) != 0
                                            {
                                                current_block = 1719972829996836699;
                                                break 's_97;
                                            }
                                            ip_0 = start3;
                                            m1 = m3;
                                            start0 = start2;
                                            m0 = m2;
                                            break;
                                        } else {
                                            start2 = start3;
                                            m2 = m3;
                                        }
                                    } else {
                                        if start2 < ip_0.offset(m1.len as isize) {
                                            if (start2.offset_from(ip_0) as c_long)
                                                < OPTIMAL_ML as c_long
                                            {
                                                let mut correction_1: c_int = 0;
                                                if m1.len > OPTIMAL_ML {
                                                    m1.len = OPTIMAL_ML;
                                                }
                                                if ip_0.offset(m1.len as isize)
                                                    > start2
                                                        .offset(m2.len as isize)
                                                        .offset(-(MINMATCH as isize))
                                                {
                                                    m1.len = start2.offset_from(ip_0)
                                                        as c_long
                                                        as c_int
                                                        + m2.len
                                                        - MINMATCH;
                                                }
                                                correction_1 = m1.len
                                                    - start2.offset_from(ip_0)
                                                        as c_long
                                                        as c_int;
                                                if correction_1 > 0 as c_int {
                                                    start2 = start2.offset(correction_1 as isize);
                                                    m2.len -= correction_1;
                                                }
                                            } else {
                                                m1.len = start2.offset_from(ip_0)
                                                    as c_long
                                                    as c_int;
                                            }
                                        }
                                        optr = op_0;
                                        if LZ4HC_encodeSequence(
                                            &raw mut ip_0,
                                            &raw mut op_0,
                                            &raw mut anchor_0,
                                            m1.len,
                                            m1.off,
                                            limit,
                                            oend,
                                        ) != 0
                                        {
                                            current_block = 1719972829996836699;
                                            break 's_97;
                                        }
                                        ip_0 = start2;
                                        m1 = m2;
                                        start2 = start3;
                                        m2 = m3;
                                    }
                                }
                            }
                        }
                    }
                    match current_block {
                        2290177392965769716 => {
                            ip_0 = start2;
                            optr = op_0;
                            if !(LZ4HC_encodeSequence(
                                &raw mut ip_0,
                                &raw mut op_0,
                                &raw mut anchor_0,
                                m2.len,
                                m2.off,
                                limit,
                                oend,
                            ) != 0)
                            {
                                current_block = 11050875288958768710;
                                continue;
                            }
                            m1 = m2;
                        }
                        _ => {}
                    }
                    if limit as c_uint
                        == fillOutput as c_int as c_uint
                    {
                        let ll: size_t =
                            ip_0.offset_from(anchor_0) as c_long as size_t;
                        let ll_addbytes: size_t =
                            ll.wrapping_add(240 as size_t).wrapping_div(255 as size_t);
                        let ll_totalCost: size_t =
                            (1 as size_t).wrapping_add(ll_addbytes).wrapping_add(ll);
                        let maxLitPos: *mut BYTE = oend.offset(-(3 as c_int as isize));
                        op_0 = optr;
                        if op_0.offset(ll_totalCost as isize) <= maxLitPos {
                            let bytesLeftForMl: size_t = maxLitPos
                                .offset_from(op_0.offset(ll_totalCost as isize))
                                as c_long
                                as size_t;
                            let maxMlSize: size_t = ((MINMATCH as c_uint)
                                .wrapping_add(ML_MASK.wrapping_sub(1 as c_uint))
                                as size_t)
                                .wrapping_add(bytesLeftForMl.wrapping_mul(255 as size_t));
                            if m1.len as size_t > maxMlSize {
                                m1.len = maxMlSize as c_int;
                            }
                            if oend.offset(LASTLITERALS as isize).offset_from(
                                op_0.offset(ll_totalCost as isize)
                                    .offset(2 as c_int as isize),
                            ) as c_long
                                - 1 as c_long
                                + m1.len as c_long
                                >= MFLIMIT as c_long
                            {
                                LZ4HC_encodeSequence(
                                    &raw mut ip_0,
                                    &raw mut op_0,
                                    &raw mut anchor_0,
                                    m1.len,
                                    m1.off,
                                    notLimited,
                                    oend,
                                );
                            }
                        }
                        current_block = 1525518218708800755;
                    } else {
                        return 0 as c_int;
                    }
                }
            }
            _ => {
                let mut lastRunSize: size_t =
                    iend.offset_from(anchor_0) as c_long as size_t;
                let mut llAdd: size_t = lastRunSize
                    .wrapping_add(255 as size_t)
                    .wrapping_sub(RUN_MASK as size_t)
                    .wrapping_div(255 as size_t);
                let totalSize: size_t = (1 as size_t).wrapping_add(llAdd).wrapping_add(lastRunSize);
                if limit as c_uint
                    == fillOutput as c_int as c_uint
                {
                    oend = oend.offset(LASTLITERALS as isize);
                }
                if limit as c_uint != 0 && op_0.offset(totalSize as isize) > oend {
                    if limit as c_uint
                        == limitedOutput as c_int as c_uint
                    {
                        return 0 as c_int;
                    }
                    lastRunSize = (oend.offset_from(op_0) as c_long as size_t)
                        .wrapping_sub(1 as size_t);
                    llAdd = lastRunSize
                        .wrapping_add(256 as size_t)
                        .wrapping_sub(RUN_MASK as size_t)
                        .wrapping_div(256 as size_t);
                    lastRunSize = lastRunSize.wrapping_sub(llAdd);
                }
                ip_0 = anchor_0.offset(lastRunSize as isize);
                if lastRunSize >= RUN_MASK as size_t {
                    let mut accumulator: size_t = lastRunSize.wrapping_sub(RUN_MASK as size_t);
                    let fresh13 = op_0;
                    op_0 = op_0.offset(1);
                    *fresh13 = (RUN_MASK << ML_BITS) as BYTE;
                    while accumulator >= 255 as size_t {
                        let fresh14 = op_0;
                        op_0 = op_0.offset(1);
                        *fresh14 = 255 as BYTE;
                        accumulator = accumulator.wrapping_sub(255 as size_t);
                    }
                    let fresh15 = op_0;
                    op_0 = op_0.offset(1);
                    *fresh15 = accumulator as BYTE;
                } else {
                    let fresh16 = op_0;
                    op_0 = op_0.offset(1);
                    *fresh16 = (lastRunSize << ML_BITS) as BYTE;
                }
                ::libc::memcpy(
                    op_0 as *mut c_void,
                    anchor_0 as *const c_void,
                    lastRunSize as ::libc::size_t,
                );
                op_0 = op_0.offset(lastRunSize as isize);
                *srcSizePtr = (ip_0 as *const c_char).offset_from(src)
                    as c_long as c_int;
                return (op_0 as *mut c_char).offset_from(dst) as c_long
                    as c_int;
            }
        }
    }
}
unsafe fn LZ4HC_compress_generic_internal(
    ctx: *mut LZ4HC_CCtx_internal,
    src: *const c_char,
    dst: *mut c_char,
    srcSizePtr: *mut c_int,
    dstCapacity: c_int,
    mut cLevel: c_int,
    limit: limitedOutput_directive,
    dict: dictCtx_directive,
) -> c_int {
    if *srcSizePtr as U32 > LZ4_MAX_INPUT_SIZE as U32 {
        return 0 as c_int;
    }
    if dstCapacity < 1 as c_int {
        return 0 as c_int;
    }
    if *srcSizePtr == 0 as c_int {
        *dst = 0 as c_char;
        return 1 as c_int;
    }
    (*ctx).end = (*ctx).end.offset(*srcSizePtr as isize);
    let cParam: cParams_t = LZ4HC_getCLevelParams(cLevel) as cParams_t;
    let favor: HCfavor_e = (if (*ctx).favorDecSpeed as c_int != 0 {
        favorDecompressionSpeed as c_int
    } else {
        favorCompressionRatio as c_int
    }) as HCfavor_e;
    let mut result: c_int = 0;
    if cParam.strat as c_uint == lz4mid as c_int as c_uint {
        result = LZ4MID_compress(ctx, src, dst, srcSizePtr, dstCapacity, limit, dict);
    } else if cParam.strat as c_uint
        == lz4hc as c_int as c_uint
    {
        result = LZ4HC_compress_hashChain(
            ctx,
            src,
            dst,
            srcSizePtr,
            dstCapacity,
            cParam.nbSearches,
            limit,
            dict,
        );
    } else {
        result = LZ4HC_compress_optimal(
            ctx,
            src,
            dst,
            srcSizePtr,
            dstCapacity,
            cParam.nbSearches,
            cParam.targetLength as size_t,
            limit,
            (cLevel >= LZ4HC_CLEVEL_MAX) as c_int,
            dict,
            favor,
        );
    }
    if result <= 0 as c_int {
        (*ctx).dirty = 1 as LZ4_i8;
    }
    return result;
}
unsafe fn LZ4HC_compress_generic_noDictCtx(
    ctx: *mut LZ4HC_CCtx_internal,
    src: *const c_char,
    dst: *mut c_char,
    srcSizePtr: *mut c_int,
    dstCapacity: c_int,
    mut cLevel: c_int,
    mut limit: limitedOutput_directive,
) -> c_int {
    return LZ4HC_compress_generic_internal(
        ctx,
        src,
        dst,
        srcSizePtr,
        dstCapacity,
        cLevel,
        limit,
        noDictCtx,
    );
}
unsafe fn isStateCompatible(
    mut ctx1: *const LZ4HC_CCtx_internal,
    mut ctx2: *const LZ4HC_CCtx_internal,
) -> c_int {
    let ctx1_view: &LZ4HC_CCtx_internal = unsafe { &*ctx1 };
    let isMid1: c_int =
        (LZ4HC_getCLevelParams(ctx1_view.compressionLevel as c_int).strat
            as c_uint
            == lz4mid as c_int as c_uint) as c_int;
    let isMid2: c_int =
        (LZ4HC_getCLevelParams((*ctx2).compressionLevel as c_int).strat
            as c_uint
            == lz4mid as c_int as c_uint) as c_int;
    return (isMid1 ^ isMid2 == 0) as c_int;
}
unsafe fn LZ4HC_compress_generic_dictCtx(
    ctx: *mut LZ4HC_CCtx_internal,
    src: *const c_char,
    dst: *mut c_char,
    srcSizePtr: *mut c_int,
    dstCapacity: c_int,
    mut cLevel: c_int,
    mut limit: limitedOutput_directive,
) -> c_int {
    let position: size_t = ((*ctx).end.offset_from((*ctx).prefixStart) as c_long
        as size_t)
        .wrapping_add((*ctx).dictLimit.wrapping_sub((*ctx).lowLimit) as size_t);
    if position
        >= (64 as c_int * ((1 as c_int) << 10 as c_int))
            as size_t
    {
        (*ctx).dictCtx = ::core::ptr::null::<LZ4HC_CCtx_internal>();
        return LZ4HC_compress_generic_noDictCtx(
            ctx,
            src,
            dst,
            srcSizePtr,
            dstCapacity,
            cLevel,
            limit,
        );
    } else if position == 0 as size_t
        && *srcSizePtr
            > 4 as c_int * ((1 as c_int) << 10 as c_int)
        && isStateCompatible(ctx, (*ctx).dictCtx) != 0
    {
        ::libc::memcpy(
            ctx as *mut c_void,
            (*ctx).dictCtx as *const c_void,
            ::core::mem::size_of::<LZ4HC_CCtx_internal>() as ::libc::size_t,
        );
        LZ4HC_setExternalDict(ctx, src as *const BYTE);
        (*ctx).compressionLevel = cLevel as c_short;
        return LZ4HC_compress_generic_noDictCtx(
            ctx,
            src,
            dst,
            srcSizePtr,
            dstCapacity,
            cLevel,
            limit,
        );
    } else {
        return LZ4HC_compress_generic_internal(
            ctx,
            src,
            dst,
            srcSizePtr,
            dstCapacity,
            cLevel,
            limit,
            usingDictCtxHc,
        );
    };
}
unsafe fn LZ4HC_compress_generic(
    ctx: *mut LZ4HC_CCtx_internal,
    src: *const c_char,
    dst: *mut c_char,
    srcSizePtr: *mut c_int,
    dstCapacity: c_int,
    mut cLevel: c_int,
    mut limit: limitedOutput_directive,
) -> c_int {
    if (*ctx).dictCtx.is_null() {
        return LZ4HC_compress_generic_noDictCtx(
            ctx,
            src,
            dst,
            srcSizePtr,
            dstCapacity,
            cLevel,
            limit,
        );
    } else {
        return LZ4HC_compress_generic_dictCtx(
            ctx,
            src,
            dst,
            srcSizePtr,
            dstCapacity,
            cLevel,
            limit,
        );
    };
}
#[inline]
pub fn LZ4_sizeofStateHC() -> c_int { {
    return ::core::mem::size_of::<LZ4_streamHC_t>() as c_int;
} }
fn LZ4_streamHC_t_alignment() -> size_t { {
    return (::core::mem::size_of::<t_a>() as size_t)
        .wrapping_sub(::core::mem::size_of::<LZ4_streamHC_t>() as size_t);
} }
#[inline]
pub unsafe fn LZ4_compress_HC_extStateHC_fastReset(
    mut state: *mut c_void,
    mut src: *const c_char,
    mut dst: *mut c_char,
    mut srcSize: c_int,
    mut dstCapacity: c_int,
    mut compressionLevel: c_int,
) -> c_int {
    let ctx: *mut LZ4HC_CCtx_internal =
        &raw mut (*(state as *mut LZ4_streamHC_t)).internal_donotuse;
    if LZ4_isAligned(state, LZ4_streamHC_t_alignment()) == 0 {
        return 0 as c_int;
    }
    LZ4_resetStreamHC_fast(state as *mut LZ4_streamHC_t, compressionLevel);
    LZ4HC_init_internal(ctx, src as *const BYTE);
    if dstCapacity < LZ4_compressBound(srcSize) {
        return LZ4HC_compress_generic(
            ctx,
            src,
            dst,
            &raw mut srcSize,
            dstCapacity,
            compressionLevel,
            limitedOutput,
        );
    } else {
        return LZ4HC_compress_generic(
            ctx,
            src,
            dst,
            &raw mut srcSize,
            dstCapacity,
            compressionLevel,
            notLimited,
        );
    };
}
#[inline]
pub unsafe fn LZ4_compress_HC_extStateHC(
    mut state: *mut c_void,
    mut src: *const c_char,
    mut dst: *mut c_char,
    mut srcSize: c_int,
    mut dstCapacity: c_int,
    mut compressionLevel: c_int,
) -> c_int {
    let ctx: *mut LZ4_streamHC_t =
        LZ4_initStreamHC(state, ::core::mem::size_of::<LZ4_streamHC_t>() as size_t)
            as *mut LZ4_streamHC_t;
    if ctx.is_null() {
        return 0 as c_int;
    }
    return LZ4_compress_HC_extStateHC_fastReset(
        state,
        src,
        dst,
        srcSize,
        dstCapacity,
        compressionLevel,
    );
}
#[inline]
pub unsafe fn LZ4_compress_HC(
    mut src: *const c_char,
    mut dst: *mut c_char,
    mut srcSize: c_int,
    mut dstCapacity: c_int,
    mut compressionLevel: c_int,
) -> c_int {
    let mut cSize: c_int = 0;
    let statePtr: *mut LZ4_streamHC_t =
        malloc(::core::mem::size_of::<LZ4_streamHC_t>() as size_t) as *mut LZ4_streamHC_t;
    if statePtr.is_null() {
        return 0 as c_int;
    }
    cSize = LZ4_compress_HC_extStateHC(
        statePtr as *mut c_void,
        src,
        dst,
        srcSize,
        dstCapacity,
        compressionLevel,
    );
    free(statePtr as *mut c_void);
    return cSize;
}
#[inline]
pub unsafe fn LZ4_compress_HC_destSize(
    mut state: *mut c_void,
    mut source: *const c_char,
    mut dest: *mut c_char,
    mut sourceSizePtr: *mut c_int,
    mut targetDestSize: c_int,
    mut cLevel: c_int,
) -> c_int {
    let ctx: *mut LZ4_streamHC_t =
        LZ4_initStreamHC(state, ::core::mem::size_of::<LZ4_streamHC_t>() as size_t)
            as *mut LZ4_streamHC_t;
    if ctx.is_null() {
        return 0 as c_int;
    }
    LZ4HC_init_internal(&raw mut (*ctx).internal_donotuse, source as *const BYTE);
    LZ4_setCompressionLevel(ctx, cLevel);
    return LZ4HC_compress_generic(
        &raw mut (*ctx).internal_donotuse,
        source,
        dest,
        sourceSizePtr,
        targetDestSize,
        cLevel,
        fillOutput,
    );
}
#[inline]
pub fn LZ4_createStreamHC() -> *mut LZ4_streamHC_t { unsafe {
    let state: *mut LZ4_streamHC_t = calloc(
        1 as size_t,
        ::core::mem::size_of::<LZ4_streamHC_t>() as size_t,
    ) as *mut LZ4_streamHC_t;
    if state.is_null() {
        return ::core::ptr::null_mut::<LZ4_streamHC_t>();
    }
    LZ4_setCompressionLevel(state, LZ4HC_CLEVEL_DEFAULT);
    return state;
} }
#[inline]
pub unsafe fn LZ4_freeStreamHC(
    mut LZ4_streamHCPtr: *mut LZ4_streamHC_t,
) -> c_int {
    if LZ4_streamHCPtr.is_null() {
        return 0 as c_int;
    }
    free(LZ4_streamHCPtr as *mut c_void);
    return 0 as c_int;
}
#[inline]
pub unsafe fn LZ4_initStreamHC(
    mut buffer: *mut c_void,
    mut size: size_t,
) -> *mut LZ4_streamHC_t {
    let LZ4_streamHCPtr: *mut LZ4_streamHC_t = buffer as *mut LZ4_streamHC_t;
    if buffer.is_null() {
        return ::core::ptr::null_mut::<LZ4_streamHC_t>();
    }
    if size < ::core::mem::size_of::<LZ4_streamHC_t>() as usize {
        return ::core::ptr::null_mut::<LZ4_streamHC_t>();
    }
    if LZ4_isAligned(buffer, LZ4_streamHC_t_alignment()) == 0 {
        return ::core::ptr::null_mut::<LZ4_streamHC_t>();
    }
    let hcstate: *mut LZ4HC_CCtx_internal = &raw mut (*LZ4_streamHCPtr).internal_donotuse;
    memset(
        hcstate as *mut c_void,
        0 as c_int,
        ::core::mem::size_of::<LZ4HC_CCtx_internal>() as size_t,
    );
    LZ4_setCompressionLevel(LZ4_streamHCPtr, LZ4HC_CLEVEL_DEFAULT);
    return LZ4_streamHCPtr;
}
#[inline]
pub unsafe fn LZ4_resetStreamHC(
    mut LZ4_streamHCPtr: *mut LZ4_streamHC_t,
    mut compressionLevel: c_int,
) {
    LZ4_initStreamHC(
        LZ4_streamHCPtr as *mut c_void,
        ::core::mem::size_of::<LZ4_streamHC_t>() as size_t,
    );
    LZ4_setCompressionLevel(LZ4_streamHCPtr, compressionLevel);
}
#[inline]
pub unsafe fn LZ4_resetStreamHC_fast(
    mut LZ4_streamHCPtr: *mut LZ4_streamHC_t,
    mut compressionLevel: c_int,
) {
    let s: *mut LZ4HC_CCtx_internal = &raw mut (*LZ4_streamHCPtr).internal_donotuse;
    if (*s).dirty != 0 {
        LZ4_initStreamHC(
            LZ4_streamHCPtr as *mut c_void,
            ::core::mem::size_of::<LZ4_streamHC_t>() as size_t,
        );
    } else {
        (*s).dictLimit =
            ((*s).dictLimit as uint32_t).wrapping_add((*s).end.offset_from((*s).prefixStart)
                as c_long as U32
                as uint32_t) as LZ4_u32 as LZ4_u32;
        (*s).prefixStart = ::core::ptr::null::<LZ4_byte>();
        (*s).end = ::core::ptr::null::<LZ4_byte>();
        (*s).dictCtx = ::core::ptr::null::<LZ4HC_CCtx_internal>();
    }
    LZ4_setCompressionLevel(LZ4_streamHCPtr, compressionLevel);
}
#[inline]
pub unsafe fn LZ4_setCompressionLevel(
    mut LZ4_streamHCPtr: *mut LZ4_streamHC_t,
    mut compressionLevel: c_int,
) {
    let LZ4_streamHCPtr_view: &mut LZ4_streamHC_t = unsafe { &mut *LZ4_streamHCPtr };
    if compressionLevel < 1 as c_int {
        compressionLevel = LZ4HC_CLEVEL_DEFAULT;
    }
    if compressionLevel > LZ4HC_CLEVEL_MAX {
        compressionLevel = LZ4HC_CLEVEL_MAX;
    }
    LZ4_streamHCPtr_view.internal_donotuse.compressionLevel =
        compressionLevel as c_short;
}
#[inline]
pub unsafe fn LZ4_favorDecompressionSpeed(
    mut LZ4_streamHCPtr: *mut LZ4_streamHC_t,
    mut favor: c_int,
) {
    let LZ4_streamHCPtr_view: &mut LZ4_streamHC_t = unsafe { &mut *LZ4_streamHCPtr };
    LZ4_streamHCPtr_view.internal_donotuse.favorDecSpeed =
        (favor != 0 as c_int) as c_int as LZ4_i8;
}
#[inline]
pub unsafe fn LZ4_loadDictHC(
    mut LZ4_streamHCPtr: *mut LZ4_streamHC_t,
    mut dictionary: *const c_char,
    mut dictSize: c_int,
) -> c_int {
    let ctxPtr: *mut LZ4HC_CCtx_internal = &raw mut (*LZ4_streamHCPtr).internal_donotuse;
    let mut cp: cParams_t = cParams_t {
        strat: lz4mid,
        nbSearches: 0,
        targetLength: 0,
    };
    if dictSize > 64 as c_int * ((1 as c_int) << 10 as c_int)
    {
        dictionary = dictionary.offset((dictSize as size_t).wrapping_sub(
            (64 as c_int * ((1 as c_int) << 10 as c_int))
                as size_t,
        ) as isize);
        dictSize =
            64 as c_int * ((1 as c_int) << 10 as c_int);
    }
    let cLevel: c_int = (*ctxPtr).compressionLevel as c_int;
    LZ4_initStreamHC(
        LZ4_streamHCPtr as *mut c_void,
        ::core::mem::size_of::<LZ4_streamHC_t>() as size_t,
    );
    LZ4_setCompressionLevel(LZ4_streamHCPtr, cLevel);
    cp = LZ4HC_getCLevelParams(cLevel);
    LZ4HC_init_internal(ctxPtr, dictionary as *const BYTE);
    (*ctxPtr).end = (dictionary as *const BYTE).offset(dictSize as isize) as *const LZ4_byte;
    if cp.strat as c_uint == lz4mid as c_int as c_uint {
        LZ4MID_fillHTable(
            ctxPtr,
            dictionary as *const c_void,
            dictSize as size_t,
        );
    } else if dictSize >= LZ4HC_HASHSIZE {
        LZ4HC_Insert(
            ctxPtr,
            (*ctxPtr).end.offset(-(3 as c_int as isize)),
        );
    }
    return dictSize;
}
#[inline]
pub unsafe fn LZ4_attach_HC_dictionary(
    mut working_stream: *mut LZ4_streamHC_t,
    mut dictionary_stream: *const LZ4_streamHC_t,
) {
    (*working_stream).internal_donotuse.dictCtx = if !dictionary_stream.is_null() {
        &raw const (*dictionary_stream).internal_donotuse
    } else {
        ::core::ptr::null::<LZ4HC_CCtx_internal>()
    };
}
unsafe fn LZ4HC_setExternalDict(
    mut ctxPtr: *mut LZ4HC_CCtx_internal,
    mut newBlock: *const BYTE,
) {
    if (*ctxPtr).end
        >= (*ctxPtr)
            .prefixStart
            .offset(4 as c_int as isize)
        && LZ4HC_getCLevelParams((*ctxPtr).compressionLevel as c_int).strat
            as c_uint
            != lz4mid as c_int as c_uint
    {
        LZ4HC_Insert(
            ctxPtr,
            (*ctxPtr).end.offset(-(3 as c_int as isize)),
        );
    }
    (*ctxPtr).lowLimit = (*ctxPtr).dictLimit;
    (*ctxPtr).dictStart = (*ctxPtr).prefixStart;
    (*ctxPtr).dictLimit = ((*ctxPtr).dictLimit as uint32_t).wrapping_add(
        (*ctxPtr).end.offset_from((*ctxPtr).prefixStart) as c_long as U32 as uint32_t,
    ) as LZ4_u32 as LZ4_u32;
    (*ctxPtr).prefixStart = newBlock as *const LZ4_byte;
    (*ctxPtr).end = newBlock as *const LZ4_byte;
    (*ctxPtr).nextToUpdate = (*ctxPtr).dictLimit;
    (*ctxPtr).dictCtx = ::core::ptr::null::<LZ4HC_CCtx_internal>();
}
unsafe fn LZ4_compressHC_continue_generic(
    mut LZ4_streamHCPtr: *mut LZ4_streamHC_t,
    mut src: *const c_char,
    mut dst: *mut c_char,
    mut srcSizePtr: *mut c_int,
    mut dstCapacity: c_int,
    mut limit: limitedOutput_directive,
) -> c_int {
    let ctxPtr: *mut LZ4HC_CCtx_internal = &raw mut (*LZ4_streamHCPtr).internal_donotuse;
    if (*ctxPtr).prefixStart.is_null() {
        LZ4HC_init_internal(ctxPtr, src as *const BYTE);
    }
    if ((*ctxPtr).end.offset_from((*ctxPtr).prefixStart) as c_long as size_t)
        .wrapping_add((*ctxPtr).dictLimit as size_t)
        > (2 as c_uint)
            .wrapping_mul((1 as c_uint) << 30 as c_int)
            as size_t
    {
        let mut dictSize: size_t =
            (*ctxPtr).end.offset_from((*ctxPtr).prefixStart) as c_long as size_t;
        if dictSize
            > (64 as c_int * ((1 as c_int) << 10 as c_int))
                as size_t
        {
            dictSize = (64 as c_int
                * ((1 as c_int) << 10 as c_int))
                as size_t;
        }
        LZ4_loadDictHC(
            LZ4_streamHCPtr,
            ((*ctxPtr).end as *const c_char).offset(-(dictSize as isize)),
            dictSize as c_int,
        );
    }
    if src as *const BYTE != (*ctxPtr).end {
        LZ4HC_setExternalDict(ctxPtr, src as *const BYTE);
    }
    let mut sourceEnd: *const BYTE = (src as *const BYTE).offset(*srcSizePtr as isize);
    let dictBegin: *const BYTE = (*ctxPtr).dictStart as *const BYTE;
    let dictEnd: *const BYTE = (*ctxPtr)
        .dictStart
        .offset((*ctxPtr).dictLimit.wrapping_sub((*ctxPtr).lowLimit) as isize);
    if sourceEnd > dictBegin && (src as *const BYTE) < dictEnd {
        if sourceEnd > dictEnd {
            sourceEnd = dictEnd;
        }
        (*ctxPtr).lowLimit = ((*ctxPtr).lowLimit as uint32_t).wrapping_add(
            sourceEnd.offset_from((*ctxPtr).dictStart) as c_long as U32 as uint32_t,
        ) as LZ4_u32 as LZ4_u32;
        (*ctxPtr).dictStart = (*ctxPtr).dictStart.offset(
            sourceEnd.offset_from((*ctxPtr).dictStart) as c_long as U32 as isize,
        );
        if (*ctxPtr).dictLimit.wrapping_sub((*ctxPtr).lowLimit) < LZ4HC_HASHSIZE as LZ4_u32 {
            (*ctxPtr).lowLimit = (*ctxPtr).dictLimit;
            (*ctxPtr).dictStart = (*ctxPtr).prefixStart;
        }
    }
    return LZ4HC_compress_generic(
        ctxPtr,
        src,
        dst,
        srcSizePtr,
        dstCapacity,
        (*ctxPtr).compressionLevel as c_int,
        limit,
    );
}
#[inline]
pub unsafe fn LZ4_compress_HC_continue(
    mut LZ4_streamHCPtr: *mut LZ4_streamHC_t,
    mut src: *const c_char,
    mut dst: *mut c_char,
    mut srcSize: c_int,
    mut dstCapacity: c_int,
) -> c_int {
    if dstCapacity < LZ4_compressBound(srcSize) {
        return LZ4_compressHC_continue_generic(
            LZ4_streamHCPtr,
            src,
            dst,
            &raw mut srcSize,
            dstCapacity,
            limitedOutput,
        );
    } else {
        return LZ4_compressHC_continue_generic(
            LZ4_streamHCPtr,
            src,
            dst,
            &raw mut srcSize,
            dstCapacity,
            notLimited,
        );
    };
}
#[inline]
pub unsafe fn LZ4_compress_HC_continue_destSize(
    mut LZ4_streamHCPtr: *mut LZ4_streamHC_t,
    mut src: *const c_char,
    mut dst: *mut c_char,
    mut srcSizePtr: *mut c_int,
    mut targetDestSize: c_int,
) -> c_int {
    return LZ4_compressHC_continue_generic(
        LZ4_streamHCPtr,
        src,
        dst,
        srcSizePtr,
        targetDestSize,
        fillOutput,
    );
}
#[inline]
pub unsafe fn LZ4_saveDictHC(
    mut LZ4_streamHCPtr: *mut LZ4_streamHC_t,
    mut safeBuffer: *mut c_char,
    mut dictSize: c_int,
) -> c_int {
    let streamPtr: *mut LZ4HC_CCtx_internal = &raw mut (*LZ4_streamHCPtr).internal_donotuse;
    let prefixSize: c_int = (*streamPtr).end.offset_from((*streamPtr).prefixStart)
        as c_long as c_int;
    if dictSize > 64 as c_int * ((1 as c_int) << 10 as c_int)
    {
        dictSize =
            64 as c_int * ((1 as c_int) << 10 as c_int);
    }
    if dictSize < 4 as c_int {
        dictSize = 0 as c_int;
    }
    if dictSize > prefixSize {
        dictSize = prefixSize;
    }
    safeBuffer.is_null();
    if dictSize > 0 as c_int {
        ::libc::memmove(
            safeBuffer as *mut c_void,
            (*streamPtr).end.offset(-(dictSize as isize)) as *const c_void,
            dictSize as size_t as ::libc::size_t,
        );
    }
    let endIndex: U32 = ((*streamPtr).end.offset_from((*streamPtr).prefixStart)
        as c_long as U32)
        .wrapping_add((*streamPtr).dictLimit as U32);
    (*streamPtr).end = (if safeBuffer.is_null() {
        ::core::ptr::null::<BYTE>()
    } else {
        (safeBuffer as *const BYTE).offset(dictSize as isize)
    }) as *const LZ4_byte;
    (*streamPtr).prefixStart = safeBuffer as *const BYTE as *const LZ4_byte;
    (*streamPtr).dictLimit = endIndex.wrapping_sub(dictSize as U32) as LZ4_u32;
    (*streamPtr).lowLimit = endIndex.wrapping_sub(dictSize as U32) as LZ4_u32;
    (*streamPtr).dictStart = (*streamPtr).prefixStart;
    if (*streamPtr).nextToUpdate < (*streamPtr).dictLimit {
        (*streamPtr).nextToUpdate = (*streamPtr).dictLimit;
    }
    return dictSize;
}
#[inline(always)]
fn LZ4HC_literalsPrice(litlen: c_int) -> c_int { {
    let mut price: c_int = litlen;
    if litlen >= RUN_MASK as c_int {
        price += 1 as c_int
            + (litlen - RUN_MASK as c_int) / 255 as c_int;
    }
    return price;
} }
#[inline(always)]
fn LZ4HC_sequencePrice(
    mut litlen: c_int,
    mut mlen: c_int,
) -> c_int { {
    let mut price: c_int = 1 as c_int + 2 as c_int;
    price += LZ4HC_literalsPrice(litlen);
    if mlen >= ML_MASK.wrapping_add(MINMATCH as c_uint) as c_int {
        price += 1 as c_int
            + (mlen - ML_MASK.wrapping_add(MINMATCH as c_uint) as c_int)
                / 255 as c_int;
    }
    return price;
} }
#[inline(always)]
unsafe fn LZ4HC_FindLongerMatch(
    ctx: *mut LZ4HC_CCtx_internal,
    mut ip_0: *const BYTE,
    iHighLimit: *const BYTE,
    mut minLen: c_int,
    mut nbSearches: c_int,
    dict: dictCtx_directive,
    favorDecSpeed: HCfavor_e,
) -> LZ4HC_match_t {
    let match0: LZ4HC_match_t = LZ4HC_match_t {
        off: 0 as c_int,
        len: 0 as c_int,
        back: 0 as c_int,
    };
    let mut md: LZ4HC_match_t = LZ4HC_InsertAndGetWiderMatch(
        ctx,
        ip_0,
        ip_0,
        iHighLimit,
        minLen,
        nbSearches,
        1 as c_int,
        1 as c_int,
        dict,
        favorDecSpeed,
    );
    if md.len <= minLen {
        return match0;
    }
    if favorDecSpeed as u64 != 0 {
        if (md.len > 18 as c_int) as c_int
            & (md.len <= 36 as c_int) as c_int
            != 0
        {
            md.len = 18 as c_int;
        }
    }
    return md;
}
unsafe fn LZ4HC_compress_optimal(
    mut ctx: *mut LZ4HC_CCtx_internal,
    source: *const c_char,
    mut dst: *mut c_char,
    mut srcSizePtr: *mut c_int,
    mut dstCapacity: c_int,
    nbSearches: c_int,
    mut sufficient_len: size_t,
    limit: limitedOutput_directive,
    fullUpdate: c_int,
    dict: dictCtx_directive,
    favorDecSpeed: HCfavor_e,
) -> c_int {
    let srcSizePtr_view: &mut c_int = unsafe { &mut *srcSizePtr };
    let mut current_block: u64;
    let mut retval: c_int = 0 as c_int;
    let opt: *mut LZ4HC_optimal_t = malloc(
        (::core::mem::size_of::<LZ4HC_optimal_t>() as size_t).wrapping_mul(
            (((1 as c_int) << 12 as c_int) + 3 as c_int)
                as size_t,
        ),
    ) as *mut LZ4HC_optimal_t;
    let mut ip_0: *const BYTE = source as *const BYTE;
    let mut anchor_0: *const BYTE = ip_0;
    let iend: *const BYTE = ip_0.offset(*srcSizePtr_view as isize);
    let mflimit: *const BYTE = iend.offset(-(MFLIMIT as isize));
    let matchlimit: *const BYTE = iend.offset(-(LASTLITERALS as isize));
    let mut op_0: *mut BYTE = dst as *mut BYTE;
    let mut opSaved: *mut BYTE = dst as *mut BYTE;
    let mut oend: *mut BYTE = op_0.offset(dstCapacity as isize);
    let mut ovml: c_int = MINMATCH;
    let mut ovoff: c_int = 0 as c_int;
    if !opt.is_null() {
        *srcSizePtr_view = 0 as c_int;
        if limit as c_uint == fillOutput as c_int as c_uint {
            oend = oend.offset(-(LASTLITERALS as isize));
        }
        if sufficient_len >= LZ4_OPT_NUM as size_t {
            sufficient_len = (LZ4_OPT_NUM - 1 as c_int) as size_t;
        }
        's_68: loop {
            if !(ip_0 <= mflimit) {
                current_block = 16717036447570235413;
                break;
            }
            let llen: c_int =
                ip_0.offset_from(anchor_0) as c_long as c_int;
            let mut best_mlen: c_int = 0;
            let mut best_off: c_int = 0;
            let mut cur: c_int = 0;
            let mut last_match_pos: c_int = 0 as c_int;
            let firstMatch: LZ4HC_match_t = LZ4HC_FindLongerMatch(
                ctx,
                ip_0,
                matchlimit,
                MINMATCH - 1 as c_int,
                nbSearches,
                dict,
                favorDecSpeed,
            ) as LZ4HC_match_t;
            if firstMatch.len == 0 as c_int {
                ip_0 = ip_0.offset(1);
            } else if firstMatch.len as size_t > sufficient_len {
                let firstML: c_int = firstMatch.len;
                opSaved = op_0;
                if !(LZ4HC_encodeSequence(
                    &raw mut ip_0,
                    &raw mut op_0,
                    &raw mut anchor_0,
                    firstML,
                    firstMatch.off,
                    limit,
                    oend,
                ) != 0)
                {
                    continue;
                }
                ovml = firstML;
                ovoff = firstMatch.off;
                current_block = 15291905670752363622;
                break;
            } else {
                let mut rPos: c_int = 0;
                rPos = 0 as c_int;
                while rPos < MINMATCH {
                    let cost: c_int =
                        LZ4HC_literalsPrice(llen + rPos) as c_int;
                    (*opt.offset(rPos as isize)).mlen = 1 as c_int;
                    (*opt.offset(rPos as isize)).off = 0 as c_int;
                    (*opt.offset(rPos as isize)).litlen = llen + rPos;
                    (*opt.offset(rPos as isize)).price = cost;
                    rPos += 1;
                }
                let matchML: c_int = firstMatch.len;
                let offset: c_int = firstMatch.off;
                let mut mlen: c_int = 0;
                mlen = MINMATCH;
                while mlen <= matchML {
                    let cost_0: c_int =
                        LZ4HC_sequencePrice(llen, mlen) as c_int;
                    (*opt.offset(mlen as isize)).mlen = mlen;
                    (*opt.offset(mlen as isize)).off = offset;
                    (*opt.offset(mlen as isize)).litlen = llen;
                    (*opt.offset(mlen as isize)).price = cost_0;
                    mlen += 1;
                }
                last_match_pos = firstMatch.len;
                let mut addLit: c_int = 0;
                addLit = 1 as c_int;
                while addLit <= TRAILING_LITERALS {
                    (*opt.offset((last_match_pos + addLit) as isize)).mlen =
                        1 as c_int;
                    (*opt.offset((last_match_pos + addLit) as isize)).off = 0 as c_int;
                    (*opt.offset((last_match_pos + addLit) as isize)).litlen = addLit;
                    (*opt.offset((last_match_pos + addLit) as isize)).price =
                        (*opt.offset(last_match_pos as isize)).price + LZ4HC_literalsPrice(addLit);
                    addLit += 1;
                }
                cur = 1 as c_int;
                loop {
                    if !(cur < last_match_pos) {
                        current_block = 4983594971376015098;
                        break;
                    }
                    let curPtr: *const BYTE = ip_0.offset(cur as isize);
                    let mut newMatch: LZ4HC_match_t = LZ4HC_match_t {
                        off: 0,
                        len: 0,
                        back: 0,
                    };
                    if curPtr > mflimit {
                        current_block = 4983594971376015098;
                        break;
                    }
                    if fullUpdate != 0 {
                        if (*opt.offset((cur + 1 as c_int) as isize)).price
                            <= (*opt.offset(cur as isize)).price
                            && (*opt.offset((cur + MINMATCH) as isize)).price
                                < (*opt.offset(cur as isize)).price + 3 as c_int
                        {
                            current_block = 1423531122933789233;
                        } else {
                            current_block = 9512719473022792396;
                        }
                    } else if (*opt.offset((cur + 1 as c_int) as isize)).price
                        <= (*opt.offset(cur as isize)).price
                    {
                        current_block = 1423531122933789233;
                    } else {
                        current_block = 9512719473022792396;
                    }
                    match current_block {
                        9512719473022792396 => {
                            if fullUpdate != 0 {
                                newMatch = LZ4HC_FindLongerMatch(
                                    ctx,
                                    curPtr,
                                    matchlimit,
                                    MINMATCH - 1 as c_int,
                                    nbSearches,
                                    dict,
                                    favorDecSpeed,
                                );
                            } else {
                                newMatch = LZ4HC_FindLongerMatch(
                                    ctx,
                                    curPtr,
                                    matchlimit,
                                    last_match_pos - cur,
                                    nbSearches,
                                    dict,
                                    favorDecSpeed,
                                );
                            }
                            if !(newMatch.len == 0) {
                                if newMatch.len as size_t > sufficient_len
                                    || newMatch.len + cur >= LZ4_OPT_NUM
                                {
                                    best_mlen = newMatch.len;
                                    best_off = newMatch.off;
                                    last_match_pos = cur + 1 as c_int;
                                    current_block = 15862093489155937825;
                                    break;
                                } else {
                                    let baseLitlen: c_int =
                                        (*opt.offset(cur as isize)).litlen;
                                    let mut litlen: c_int = 0;
                                    litlen = 1 as c_int;
                                    while litlen < MINMATCH {
                                        let price: c_int = (*opt.offset(cur as isize))
                                            .price
                                            - LZ4HC_literalsPrice(baseLitlen) as c_int
                                            + LZ4HC_literalsPrice(baseLitlen + litlen)
                                                as c_int;
                                        let pos: c_int = cur + litlen;
                                        if price < (*opt.offset(pos as isize)).price {
                                            (*opt.offset(pos as isize)).mlen =
                                                1 as c_int;
                                            (*opt.offset(pos as isize)).off =
                                                0 as c_int;
                                            (*opt.offset(pos as isize)).litlen =
                                                baseLitlen + litlen;
                                            (*opt.offset(pos as isize)).price = price;
                                        }
                                        litlen += 1;
                                    }
                                    let matchML_0: c_int = newMatch.len;
                                    let mut ml: c_int = MINMATCH;
                                    while ml <= matchML_0 {
                                        let pos_0: c_int = cur + ml;
                                        let offset_0: c_int = newMatch.off;
                                        let mut price_0: c_int = 0;
                                        let mut ll: c_int = 0;
                                        if (*opt.offset(cur as isize)).mlen
                                            == 1 as c_int
                                        {
                                            ll = (*opt.offset(cur as isize)).litlen;
                                            price_0 = (if cur > ll {
                                                (*opt.offset((cur - ll) as isize)).price
                                            } else {
                                                0 as c_int
                                            }) + LZ4HC_sequencePrice(ll, ml);
                                        } else {
                                            ll = 0 as c_int;
                                            price_0 = (*opt.offset(cur as isize)).price
                                                + LZ4HC_sequencePrice(0 as c_int, ml);
                                        }
                                        if pos_0 > last_match_pos + TRAILING_LITERALS
                                            || price_0
                                                <= (*opt.offset(pos_0 as isize)).price
                                                    - favorDecSpeed as c_int
                                        {
                                            if ml == matchML_0 && last_match_pos < pos_0 {
                                                last_match_pos = pos_0;
                                            }
                                            (*opt.offset(pos_0 as isize)).mlen = ml;
                                            (*opt.offset(pos_0 as isize)).off = offset_0;
                                            (*opt.offset(pos_0 as isize)).litlen = ll;
                                            (*opt.offset(pos_0 as isize)).price = price_0;
                                        }
                                        ml += 1;
                                    }
                                    let mut addLit_0: c_int = 0;
                                    addLit_0 = 1 as c_int;
                                    while addLit_0 <= TRAILING_LITERALS {
                                        (*opt.offset((last_match_pos + addLit_0) as isize)).mlen =
                                            1 as c_int;
                                        (*opt.offset((last_match_pos + addLit_0) as isize)).off =
                                            0 as c_int;
                                        (*opt.offset((last_match_pos + addLit_0) as isize))
                                            .litlen = addLit_0;
                                        (*opt.offset((last_match_pos + addLit_0) as isize)).price =
                                            (*opt.offset(last_match_pos as isize)).price
                                                + LZ4HC_literalsPrice(addLit_0);
                                        addLit_0 += 1;
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                    cur += 1;
                }
                match current_block {
                    4983594971376015098 => {
                        best_mlen = (*opt.offset(last_match_pos as isize)).mlen;
                        best_off = (*opt.offset(last_match_pos as isize)).off;
                        cur = last_match_pos - best_mlen;
                    }
                    _ => {}
                }
                let mut candidate_pos: c_int = cur;
                let mut selected_matchLength: c_int = best_mlen;
                let mut selected_offset: c_int = best_off;
                loop {
                    let next_matchLength: c_int =
                        (*opt.offset(candidate_pos as isize)).mlen;
                    let next_offset: c_int = (*opt.offset(candidate_pos as isize)).off;
                    (*opt.offset(candidate_pos as isize)).mlen = selected_matchLength;
                    (*opt.offset(candidate_pos as isize)).off = selected_offset;
                    selected_matchLength = next_matchLength;
                    selected_offset = next_offset;
                    if next_matchLength > candidate_pos {
                        break;
                    }
                    candidate_pos -= next_matchLength;
                }
                let mut rPos_0: c_int = 0 as c_int;
                while rPos_0 < last_match_pos {
                    let ml_0: c_int = (*opt.offset(rPos_0 as isize)).mlen;
                    let offset_1: c_int = (*opt.offset(rPos_0 as isize)).off;
                    if ml_0 == 1 as c_int {
                        ip_0 = ip_0.offset(1);
                        rPos_0 += 1;
                    } else {
                        rPos_0 += ml_0;
                        opSaved = op_0;
                        if !(LZ4HC_encodeSequence(
                            &raw mut ip_0,
                            &raw mut op_0,
                            &raw mut anchor_0,
                            ml_0,
                            offset_1,
                            limit,
                            oend,
                        ) != 0)
                        {
                            continue;
                        }
                        ovml = ml_0;
                        ovoff = offset_1;
                        current_block = 15291905670752363622;
                        break 's_68;
                    }
                }
            }
        }
        match current_block {
            16717036447570235413 => {
                current_block = 12608488225262500095;
            }
            _ => {
                if limit as c_uint
                    == fillOutput as c_int as c_uint
                {
                    let ll_0: size_t = ip_0.offset_from(anchor_0) as c_long as size_t;
                    let ll_addbytes: size_t =
                        ll_0.wrapping_add(240 as size_t).wrapping_div(255 as size_t);
                    let ll_totalCost: size_t =
                        (1 as size_t).wrapping_add(ll_addbytes).wrapping_add(ll_0);
                    let maxLitPos: *mut BYTE = oend.offset(-(3 as c_int as isize));
                    op_0 = opSaved;
                    if op_0.offset(ll_totalCost as isize) <= maxLitPos {
                        let bytesLeftForMl: size_t =
                            maxLitPos.offset_from(op_0.offset(ll_totalCost as isize))
                                as c_long as size_t;
                        let maxMlSize: size_t = ((MINMATCH as c_uint)
                            .wrapping_add(ML_MASK.wrapping_sub(1 as c_uint))
                            as size_t)
                            .wrapping_add(bytesLeftForMl.wrapping_mul(255 as size_t));
                        if ovml as size_t > maxMlSize {
                            ovml = maxMlSize as c_int;
                        }
                        if oend.offset(LASTLITERALS as isize).offset_from(
                            op_0.offset(ll_totalCost as isize)
                                .offset(2 as c_int as isize),
                        ) as c_long
                            - 1 as c_long
                            + ovml as c_long
                            >= MFLIMIT as c_long
                        {
                            LZ4HC_encodeSequence(
                                &raw mut ip_0,
                                &raw mut op_0,
                                &raw mut anchor_0,
                                ovml,
                                ovoff,
                                notLimited,
                                oend,
                            );
                        }
                    }
                    current_block = 12608488225262500095;
                } else {
                    current_block = 13620957476428869110;
                }
            }
        }
        match current_block {
            13620957476428869110 => {}
            _ => {
                let mut lastRunSize: size_t =
                    iend.offset_from(anchor_0) as c_long as size_t;
                let mut llAdd: size_t = lastRunSize
                    .wrapping_add(255 as size_t)
                    .wrapping_sub(RUN_MASK as size_t)
                    .wrapping_div(255 as size_t);
                let totalSize: size_t = (1 as size_t).wrapping_add(llAdd).wrapping_add(lastRunSize);
                if limit as c_uint
                    == fillOutput as c_int as c_uint
                {
                    oend = oend.offset(LASTLITERALS as isize);
                }
                if limit as c_uint != 0 && op_0.offset(totalSize as isize) > oend {
                    if limit as c_uint
                        == limitedOutput as c_int as c_uint
                    {
                        retval = 0 as c_int;
                        current_block = 13620957476428869110;
                    } else {
                        lastRunSize = (oend.offset_from(op_0) as c_long as size_t)
                            .wrapping_sub(1 as size_t);
                        llAdd = lastRunSize
                            .wrapping_add(256 as size_t)
                            .wrapping_sub(RUN_MASK as size_t)
                            .wrapping_div(256 as size_t);
                        lastRunSize = lastRunSize.wrapping_sub(llAdd);
                        current_block = 5219368551394180541;
                    }
                } else {
                    current_block = 5219368551394180541;
                }
                match current_block {
                    13620957476428869110 => {}
                    _ => {
                        ip_0 = anchor_0.offset(lastRunSize as isize);
                        if lastRunSize >= RUN_MASK as size_t {
                            let mut accumulator: size_t =
                                lastRunSize.wrapping_sub(RUN_MASK as size_t);
                            let fresh0 = op_0;
                            op_0 = op_0.offset(1);
                            *fresh0 = (RUN_MASK << ML_BITS) as BYTE;
                            while accumulator >= 255 as size_t {
                                let fresh1 = op_0;
                                op_0 = op_0.offset(1);
                                *fresh1 = 255 as BYTE;
                                accumulator = accumulator.wrapping_sub(255 as size_t);
                            }
                            let fresh2 = op_0;
                            op_0 = op_0.offset(1);
                            *fresh2 = accumulator as BYTE;
                        } else {
                            let fresh3 = op_0;
                            op_0 = op_0.offset(1);
                            *fresh3 = (lastRunSize << ML_BITS) as BYTE;
                        }
                        ::libc::memcpy(
                            op_0 as *mut c_void,
                            anchor_0 as *const c_void,
                            lastRunSize as ::libc::size_t,
                        );
                        op_0 = op_0.offset(lastRunSize as isize);
                        *srcSizePtr_view = (ip_0 as *const c_char).offset_from(source)
                            as c_long
                            as c_int;
                        retval = (op_0 as *mut c_char).offset_from(dst)
                            as c_long
                            as c_int;
                    }
                }
            }
        }
    }
    if !opt.is_null() {
        free(opt as *mut c_void);
    }
    return retval;
}
pub const TRAILING_LITERALS: c_int = 3 as c_int;
#[inline]
pub unsafe fn LZ4_compressHC(
    mut src: *const c_char,
    mut dst: *mut c_char,
    mut srcSize: c_int,
) -> c_int {
    return LZ4_compress_HC(
        src,
        dst,
        srcSize,
        LZ4_compressBound(srcSize),
        0 as c_int,
    );
}
#[inline]
pub unsafe fn LZ4_compressHC_limitedOutput(
    mut src: *const c_char,
    mut dst: *mut c_char,
    mut srcSize: c_int,
    mut maxDstSize: c_int,
) -> c_int {
    return LZ4_compress_HC(src, dst, srcSize, maxDstSize, 0 as c_int);
}
#[inline]
pub unsafe fn LZ4_compressHC2(
    mut src: *const c_char,
    mut dst: *mut c_char,
    mut srcSize: c_int,
    mut cLevel: c_int,
) -> c_int {
    return LZ4_compress_HC(src, dst, srcSize, LZ4_compressBound(srcSize), cLevel);
}
#[inline]
pub unsafe fn LZ4_compressHC2_limitedOutput(
    mut src: *const c_char,
    mut dst: *mut c_char,
    mut srcSize: c_int,
    mut maxDstSize: c_int,
    mut cLevel: c_int,
) -> c_int {
    return LZ4_compress_HC(src, dst, srcSize, maxDstSize, cLevel);
}
#[inline]
pub unsafe fn LZ4_compressHC_withStateHC(
    mut state: *mut c_void,
    mut src: *const c_char,
    mut dst: *mut c_char,
    mut srcSize: c_int,
) -> c_int {
    return LZ4_compress_HC_extStateHC(
        state,
        src,
        dst,
        srcSize,
        LZ4_compressBound(srcSize),
        0 as c_int,
    );
}
#[inline]
pub unsafe fn LZ4_compressHC_limitedOutput_withStateHC(
    mut state: *mut c_void,
    mut src: *const c_char,
    mut dst: *mut c_char,
    mut srcSize: c_int,
    mut maxDstSize: c_int,
) -> c_int {
    return LZ4_compress_HC_extStateHC(
        state,
        src,
        dst,
        srcSize,
        maxDstSize,
        0 as c_int,
    );
}
#[inline]
pub unsafe fn LZ4_compressHC2_withStateHC(
    mut state: *mut c_void,
    mut src: *const c_char,
    mut dst: *mut c_char,
    mut srcSize: c_int,
    mut cLevel: c_int,
) -> c_int {
    return LZ4_compress_HC_extStateHC(
        state,
        src,
        dst,
        srcSize,
        LZ4_compressBound(srcSize),
        cLevel,
    );
}
#[inline]
pub unsafe fn LZ4_compressHC2_limitedOutput_withStateHC(
    mut state: *mut c_void,
    mut src: *const c_char,
    mut dst: *mut c_char,
    mut srcSize: c_int,
    mut maxDstSize: c_int,
    mut cLevel: c_int,
) -> c_int {
    return LZ4_compress_HC_extStateHC(state, src, dst, srcSize, maxDstSize, cLevel);
}
#[inline]
pub unsafe fn LZ4_compressHC_continue(
    mut ctx: *mut LZ4_streamHC_t,
    mut src: *const c_char,
    mut dst: *mut c_char,
    mut srcSize: c_int,
) -> c_int {
    return LZ4_compress_HC_continue(ctx, src, dst, srcSize, LZ4_compressBound(srcSize));
}
#[inline]
pub unsafe fn LZ4_compressHC_limitedOutput_continue(
    mut ctx: *mut LZ4_streamHC_t,
    mut src: *const c_char,
    mut dst: *mut c_char,
    mut srcSize: c_int,
    mut maxDstSize: c_int,
) -> c_int {
    return LZ4_compress_HC_continue(ctx, src, dst, srcSize, maxDstSize);
}
#[inline]
pub fn LZ4_sizeofStreamStateHC() -> c_int { {
    return ::core::mem::size_of::<LZ4_streamHC_t>() as c_int;
} }
#[inline]
pub unsafe fn LZ4_resetStreamStateHC(
    mut state: *mut c_void,
    mut inputBuffer: *mut c_char,
) -> c_int {
    let hc4: *mut LZ4_streamHC_t =
        LZ4_initStreamHC(state, ::core::mem::size_of::<LZ4_streamHC_t>() as size_t)
            as *mut LZ4_streamHC_t;
    if hc4.is_null() {
        return 1 as c_int;
    }
    LZ4HC_init_internal(
        &raw mut (*hc4).internal_donotuse,
        inputBuffer as *const BYTE,
    );
    return 0 as c_int;
}
#[inline]
pub unsafe fn LZ4_createHC(
    mut inputBuffer: *const c_char,
) -> *mut c_void {
    let hc4: *mut LZ4_streamHC_t = LZ4_createStreamHC() as *mut LZ4_streamHC_t;
    if hc4.is_null() {
        return NULL;
    }
    LZ4HC_init_internal(
        &raw mut (*hc4).internal_donotuse,
        inputBuffer as *const BYTE,
    );
    return hc4 as *mut c_void;
}
#[inline]
pub unsafe fn LZ4_freeHC(
    mut LZ4HC_Data: *mut c_void,
) -> c_int {
    if LZ4HC_Data.is_null() {
        return 0 as c_int;
    }
    free(LZ4HC_Data);
    return 0 as c_int;
}
#[inline]
pub unsafe fn LZ4_compressHC2_continue(
    mut LZ4HC_Data: *mut c_void,
    mut src: *const c_char,
    mut dst: *mut c_char,
    mut srcSize: c_int,
    mut cLevel: c_int,
) -> c_int {
    return LZ4HC_compress_generic(
        &raw mut (*(LZ4HC_Data as *mut LZ4_streamHC_t)).internal_donotuse,
        src,
        dst,
        &raw mut srcSize,
        0 as c_int,
        cLevel,
        notLimited,
    );
}
#[inline]
pub unsafe fn LZ4_compressHC2_limitedOutput_continue(
    mut LZ4HC_Data: *mut c_void,
    mut src: *const c_char,
    mut dst: *mut c_char,
    mut srcSize: c_int,
    mut dstCapacity: c_int,
    mut cLevel: c_int,
) -> c_int {
    return LZ4HC_compress_generic(
        &raw mut (*(LZ4HC_Data as *mut LZ4_streamHC_t)).internal_donotuse,
        src,
        dst,
        &raw mut srcSize,
        dstCapacity,
        cLevel,
        limitedOutput,
    );
}
#[inline]
pub unsafe fn LZ4_slideInputBufferHC(
    mut LZ4HC_Data: *mut c_void,
) -> *mut c_char {
    let s: *mut LZ4HC_CCtx_internal =
        &raw mut (*(LZ4HC_Data as *mut LZ4_streamHC_t)).internal_donotuse;
    let bufferStart: *const BYTE = (*s)
        .prefixStart
        .offset(-((*s).dictLimit as isize))
        .offset((*s).lowLimit as isize);
    LZ4_resetStreamHC_fast(
        LZ4HC_Data as *mut LZ4_streamHC_t,
        (*s).compressionLevel as c_int,
    );
    return bufferStart as uptrval as *mut c_char;
}
