use core::ffi::*;
use crate::src::lz4hc::LZ4_attach_HC_dictionary;
use crate::src::lz4::LZ4_attach_dictionary;
use crate::src::lz4hc::LZ4_compress_HC_continue;
use crate::src::lz4hc::LZ4_compress_HC_extStateHC_fastReset;
use crate::src::lz4::LZ4_compress_fast_continue;
use crate::src::lz4::LZ4_compress_fast_extState_fastReset;
use crate::src::lz4::LZ4_decompress_safe_usingDict;
use crate::src::lz4hc::LZ4_favorDecompressionSpeed;
use crate::src::lz4hc::LZ4_initStreamHC;
use crate::src::lz4::LZ4_loadDict;
use crate::src::lz4hc::LZ4_loadDictHC;
use crate::src::lz4::LZ4_loadDictSlow;
use crate::src::lz4hc::LZ4_resetStreamHC_fast;
use crate::src::lz4::LZ4_resetStream_fast;
use crate::src::lz4::LZ4_saveDict;
use crate::src::lz4hc::LZ4_saveDictHC;
use crate::src::lz4hc::LZ4_setCompressionLevel;
use crate::src::lz4::LZ4_sizeofState;
use crate::src::lz4hc::LZ4_sizeofStateHC;
use crate::src::xxhash::XXH32;
use crate::src::xxhash::XXH32_digest;
use crate::src::xxhash::XXH32_reset;
use crate::src::xxhash::XXH32_update;
pub use crate::src::ffi::*;
pub use crate::src::c_consts::*;
pub use crate::src::c_structs::*;
pub use crate::src::c_types::*;
extern "C" {
    fn LZ4_initStream(stateBuffer: *mut c_void, size: size_t) -> *mut LZ4_stream_t;
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct LZ4F_CustomMem {
    pub customAlloc: LZ4F_AllocFunction,
    pub customCalloc: LZ4F_CallocFunction,
    pub customFree: LZ4F_FreeFunction,
    pub opaqueState: *mut c_void,
}
pub type LZ4F_FreeFunction =
    Option<unsafe extern "C" fn(*mut c_void, *mut c_void) -> ()>;
pub type LZ4F_CallocFunction =
    Option<unsafe extern "C" fn(*mut c_void, size_t) -> *mut c_void>;
pub type LZ4F_AllocFunction =
    Option<unsafe extern "C" fn(*mut c_void, size_t) -> *mut c_void>;
pub type LZ4F_cctx_t = LZ4F_cctx_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LZ4F_cctx_s {
    pub cmem: LZ4F_CustomMem,
    pub prefs: LZ4F_preferences_t,
    pub version: U32,
    pub cStage: U32,
    pub cdict: *const LZ4F_CDict,
    pub maxBlockSize: size_t,
    pub maxBufferSize: size_t,
    pub tmpBuff: *mut BYTE,
    pub tmpIn: *mut BYTE,
    pub tmpInSize: size_t,
    pub totalInSize: U64,
    pub xxh: XXH32_state_t,
    pub lz4CtxPtr: *mut c_void,
    pub lz4CtxAlloc: U16,
    pub lz4CtxType: U16,
    pub blockCompressMode: LZ4F_BlockCompressMode_e,
}
pub type LZ4F_BlockCompressMode_e = c_uint;
pub const LZ4B_UNCOMPRESSED: LZ4F_BlockCompressMode_e = 1;
pub const LZ4B_COMPRESSED: LZ4F_BlockCompressMode_e = 0;

pub type BYTE = uint8_t;

pub type LZ4F_CDict = LZ4F_CDict_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LZ4F_CDict_s {
    pub cmem: LZ4F_CustomMem,
    pub dictContent: *mut c_void,
    pub fastCtx: *mut LZ4_stream_t,
    pub HCCtx: *mut LZ4_streamHC_t,
}

pub type LZ4F_cctx = LZ4F_cctx_s;

pub type C2RustUnnamed_htdd24ee73 = c_uint;
pub const LZ4F_static_assert: C2RustUnnamed_htdd24ee73 = 1;

pub type compressFunc_t = Option<
    unsafe extern "C" fn(
        *mut c_void,
        *const c_char,
        *mut c_char,
        c_int,
        c_int,
        c_int,
        *const LZ4F_CDict,
    ) -> c_int,
>;

pub const fromSrcBuffer: LZ4F_lastBlockStatus = 2;
pub type LZ4F_lastBlockStatus = c_uint;
pub const fromTmpBuffer: LZ4F_lastBlockStatus = 1;
pub const notDone: LZ4F_lastBlockStatus = 0;
pub const ctxFast: C2RustUnnamed_htdd24ee73 = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LZ4F_dctx_s {
    pub cmem: LZ4F_CustomMem,
    pub frameInfo: LZ4F_frameInfo_t,
    pub version: U32,
    pub dStage: dStage_t,
    pub frameRemainingSize: U64,
    pub maxBlockSize: size_t,
    pub maxBufferSize: size_t,
    pub tmpIn: *mut BYTE,
    pub tmpInSize: size_t,
    pub tmpInTarget: size_t,
    pub tmpOutBuffer: *mut BYTE,
    pub dict: *const BYTE,
    pub dictSize: size_t,
    pub tmpOut: *mut BYTE,
    pub tmpOutSize: size_t,
    pub tmpOutStart: size_t,
    pub xxh: XXH32_state_t,
    pub blockChecksum: XXH32_state_t,
    pub skipChecksum: c_int,
    pub header: [BYTE; 19],
}
pub type dStage_t = c_uint;
pub const dstage_skipSkippable: dStage_t = 14;
pub const dstage_storeSFrameSize: dStage_t = 13;
pub const dstage_getSFrameSize: dStage_t = 12;
pub const dstage_storeSuffix: dStage_t = 11;
pub const dstage_getSuffix: dStage_t = 10;
pub const dstage_flushOut: dStage_t = 9;
pub const dstage_storeCBlock: dStage_t = 8;
pub const dstage_getCBlock: dStage_t = 7;
pub const dstage_getBlockChecksum: dStage_t = 6;
pub const dstage_copyDirect: dStage_t = 5;
pub const dstage_storeBlockHeader: dStage_t = 4;
pub const dstage_getBlockHeader: dStage_t = 3;
pub const dstage_init: dStage_t = 2;
pub const dstage_storeFrameHeader: dStage_t = 1;
pub const dstage_getFrameHeader: dStage_t = 0;
pub type LZ4F_dctx = LZ4F_dctx_s;

pub const LZ4F_static_assert_0: C2RustUnnamed_htdd24ee73 = 1;

pub const LZ4F_static_assert_1: C2RustUnnamed_htdd24ee73 = 1;

pub const ctxHC: C2RustUnnamed_htdd24ee73 = 2;
pub const ctxNone: C2RustUnnamed_htdd24ee73 = 0;
pub const NULL_0: *mut c_void = ::core::ptr::null_mut::<c_void>();

pub const LZ4F_BLOCK_HEADER_SIZE: c_int = 4 as c_int;
pub const LZ4F_BLOCK_CHECKSUM_SIZE: c_int = 4 as c_int;
pub const LZ4F_MAGICNUMBER: c_uint = 0x184d2204 as c_uint;
pub const LZ4F_MAGIC_SKIPPABLE_START: c_uint = 0x184d2a50 as c_uint;
pub const LZ4F_MIN_SIZE_TO_KNOW_HEADER_LENGTH: c_int = 5 as c_int;
static mut LZ4F_defaultCMem: LZ4F_CustomMem = LZ4F_CustomMem {
    customAlloc: None,
    customCalloc: None,
    customFree: None,
    opaqueState: NULL_0,
};
pub const LZ4HC_CLEVEL_MIN: c_int = 2 as c_int;

fn LZ4F_calloc(
    mut s: size_t,
    mut cmem: LZ4F_CustomMem,
) -> *mut c_void { unsafe {
    if cmem.customCalloc.is_some() {
        return cmem.customCalloc.expect("non-null function pointer")(cmem.opaqueState, s);
    }
    if cmem.customAlloc.is_none() {
        return calloc(1 as size_t, s);
    }
    let p: *mut c_void =
        cmem.customAlloc.expect("non-null function pointer")(cmem.opaqueState, s)
            as *mut c_void;
    if !p.is_null() {
        memset(p, 0 as c_int, s);
    }
    return p;
} }
fn LZ4F_malloc(
    mut s: size_t,
    mut cmem: LZ4F_CustomMem,
) -> *mut c_void { unsafe {
    if cmem.customAlloc.is_some() {
        return cmem.customAlloc.expect("non-null function pointer")(cmem.opaqueState, s);
    }
    return malloc(s);
} }
unsafe fn LZ4F_free(mut p: *mut c_void, mut cmem: LZ4F_CustomMem) {
    if p.is_null() {
        return;
    }
    if cmem.customFree.is_some() {
        cmem.customFree.expect("non-null function pointer")(cmem.opaqueState, p);
        return;
    }
    free(p);
}
unsafe fn LZ4F_readLE32(mut src: *const c_void) -> U32 {
    let srcPtr: *const BYTE = src as *const BYTE;
    let mut value32: U32 = *srcPtr.offset(0 as c_int as isize) as U32;
    value32 |= (*srcPtr.offset(1 as c_int as isize) as U32) << 8 as c_int;
    value32 |=
        (*srcPtr.offset(2 as c_int as isize) as U32) << 16 as c_int;
    value32 |=
        (*srcPtr.offset(3 as c_int as isize) as U32) << 24 as c_int;
    return value32;
}
unsafe fn LZ4F_writeLE32(mut dst: *mut c_void, mut value32: U32) {
    let dstPtr: *mut BYTE = dst as *mut BYTE;
    *dstPtr.offset(0 as c_int as isize) = value32 as BYTE;
    *dstPtr.offset(1 as c_int as isize) = (value32 >> 8 as c_int) as BYTE;
    *dstPtr.offset(2 as c_int as isize) =
        (value32 >> 16 as c_int) as BYTE;
    *dstPtr.offset(3 as c_int as isize) =
        (value32 >> 24 as c_int) as BYTE;
}
unsafe fn LZ4F_readLE64(mut src: *const c_void) -> U64 {
    let srcPtr: *const BYTE = src as *const BYTE;
    let mut value64: U64 = *srcPtr.offset(0 as c_int as isize) as U64;
    value64 |= (*srcPtr.offset(1 as c_int as isize) as U64) << 8 as c_int;
    value64 |=
        (*srcPtr.offset(2 as c_int as isize) as U64) << 16 as c_int;
    value64 |=
        (*srcPtr.offset(3 as c_int as isize) as U64) << 24 as c_int;
    value64 |=
        (*srcPtr.offset(4 as c_int as isize) as U64) << 32 as c_int;
    value64 |=
        (*srcPtr.offset(5 as c_int as isize) as U64) << 40 as c_int;
    value64 |=
        (*srcPtr.offset(6 as c_int as isize) as U64) << 48 as c_int;
    value64 |=
        (*srcPtr.offset(7 as c_int as isize) as U64) << 56 as c_int;
    return value64;
}
unsafe fn LZ4F_writeLE64(mut dst: *mut c_void, mut value64: U64) {
    let dstPtr: *mut BYTE = dst as *mut BYTE;
    *dstPtr.offset(0 as c_int as isize) = value64 as BYTE;
    *dstPtr.offset(1 as c_int as isize) = (value64 >> 8 as c_int) as BYTE;
    *dstPtr.offset(2 as c_int as isize) =
        (value64 >> 16 as c_int) as BYTE;
    *dstPtr.offset(3 as c_int as isize) =
        (value64 >> 24 as c_int) as BYTE;
    *dstPtr.offset(4 as c_int as isize) =
        (value64 >> 32 as c_int) as BYTE;
    *dstPtr.offset(5 as c_int as isize) =
        (value64 >> 40 as c_int) as BYTE;
    *dstPtr.offset(6 as c_int as isize) =
        (value64 >> 48 as c_int) as BYTE;
    *dstPtr.offset(7 as c_int as isize) =
        (value64 >> 56 as c_int) as BYTE;
}
pub const _1BIT: c_int = 0x1 as c_int;
pub const _2BITS: c_int = 0x3 as c_int;
pub const _3BITS: c_int = 0x7 as c_int;
pub const _4BITS: c_int = 0xf as c_int;
pub const LZ4F_BLOCKUNCOMPRESSED_FLAG: c_uint = 0x80000000 as c_uint;
static mut minFHSize: size_t = LZ4F_HEADER_SIZE_MIN as size_t;
static mut maxFHSize: size_t = LZ4F_HEADER_SIZE_MAX as size_t;
static mut BHSize: size_t = LZ4F_BLOCK_HEADER_SIZE as size_t;
static mut BFSize: size_t = LZ4F_BLOCK_CHECKSUM_SIZE as size_t;
static mut LZ4F_errorStrings: [*const c_char; 25] = [
    b"OK_NoError\0" as *const u8 as *const c_char,
    b"ERROR_GENERIC\0" as *const u8 as *const c_char,
    b"ERROR_maxBlockSize_invalid\0" as *const u8 as *const c_char,
    b"ERROR_blockMode_invalid\0" as *const u8 as *const c_char,
    b"ERROR_parameter_invalid\0" as *const u8 as *const c_char,
    b"ERROR_compressionLevel_invalid\0" as *const u8 as *const c_char,
    b"ERROR_headerVersion_wrong\0" as *const u8 as *const c_char,
    b"ERROR_blockChecksum_invalid\0" as *const u8 as *const c_char,
    b"ERROR_reservedFlag_set\0" as *const u8 as *const c_char,
    b"ERROR_allocation_failed\0" as *const u8 as *const c_char,
    b"ERROR_srcSize_tooLarge\0" as *const u8 as *const c_char,
    b"ERROR_dstMaxSize_tooSmall\0" as *const u8 as *const c_char,
    b"ERROR_frameHeader_incomplete\0" as *const u8 as *const c_char,
    b"ERROR_frameType_unknown\0" as *const u8 as *const c_char,
    b"ERROR_frameSize_wrong\0" as *const u8 as *const c_char,
    b"ERROR_srcPtr_wrong\0" as *const u8 as *const c_char,
    b"ERROR_decompressionFailed\0" as *const u8 as *const c_char,
    b"ERROR_headerChecksum_invalid\0" as *const u8 as *const c_char,
    b"ERROR_contentChecksum_invalid\0" as *const u8 as *const c_char,
    b"ERROR_frameDecoding_alreadyStarted\0" as *const u8 as *const c_char,
    b"ERROR_compressionState_uninitialized\0" as *const u8 as *const c_char,
    b"ERROR_parameter_null\0" as *const u8 as *const c_char,
    b"ERROR_io_write\0" as *const u8 as *const c_char,
    b"ERROR_io_read\0" as *const u8 as *const c_char,
    b"ERROR_maxCode\0" as *const u8 as *const c_char,
];
#[inline]
pub fn LZ4F_isError(mut code: LZ4F_errorCode_t) -> c_uint { {
    return (code > -(LZ4F_ERROR_maxCode as c_int) as LZ4F_errorCode_t)
        as c_int as c_uint;
} }
#[inline]
pub fn LZ4F_getErrorName(
    mut code: LZ4F_errorCode_t,
) -> *const c_char { unsafe {
    static mut codeError: *const c_char =
        b"Unspecified error code\0" as *const u8 as *const c_char;
    if LZ4F_isError(code) != 0 {
        return LZ4F_errorStrings[-(code as c_int) as usize];
    }
    return codeError;
} }
#[inline]
pub fn LZ4F_getErrorCode(mut functionResult: size_t) -> LZ4F_errorCodes { {
    if LZ4F_isError(functionResult as LZ4F_errorCode_t) == 0 {
        return LZ4F_OK_NoError;
    }
    return -(functionResult as ptrdiff_t) as LZ4F_errorCodes;
} }
fn LZ4F_returnErrorCode(mut code: LZ4F_errorCodes) -> LZ4F_errorCode_t { {
    return -(code as ptrdiff_t) as LZ4F_errorCode_t;
} }
#[inline]
pub fn LZ4F_getVersion() -> c_uint { {
    return LZ4F_VERSION as c_uint;
} }
#[inline]
pub fn LZ4F_compressionLevel_max() -> c_int { {
    return LZ4HC_CLEVEL_MAX;
} }
#[inline]
pub fn LZ4F_getBlockSize(mut blockSizeID: LZ4F_blockSizeID_t) -> size_t { unsafe {
    static mut blockSizes: [size_t; 4] = [
        (64 as c_int * ((1 as c_int) << 10 as c_int))
            as size_t,
        (256 as c_int * ((1 as c_int) << 10 as c_int))
            as size_t,
        (1 as c_int * ((1 as c_int) << 20 as c_int))
            as size_t,
        (4 as c_int * ((1 as c_int) << 20 as c_int))
            as size_t,
    ];
    if blockSizeID as c_uint == 0 as c_uint {
        blockSizeID = LZ4F_max64KB;
    }
    if (blockSizeID as c_uint)
        < LZ4F_max64KB as c_int as c_uint
        || blockSizeID as c_uint
            > LZ4F_max4MB as c_int as c_uint
    {
        return LZ4F_returnErrorCode(LZ4F_ERROR_maxBlockSize_invalid) as size_t;
    }
    let blockSizeIdx: c_int =
        blockSizeID as c_int - LZ4F_max64KB as c_int;
    return blockSizes[blockSizeIdx as usize];
} }
unsafe fn LZ4F_headerChecksum(
    mut header: *const c_void,
    mut length: size_t,
) -> BYTE {
    let xxh: U32 = XXH32(header, length, 0 as c_uint) as U32;
    return (xxh >> 8 as c_int) as BYTE;
}
fn LZ4F_optimalBSID(
    requestedBSID: LZ4F_blockSizeID_t,
    srcSize: size_t,
) -> LZ4F_blockSizeID_t { {
    let mut proposedBSID: LZ4F_blockSizeID_t = LZ4F_max64KB;
    let mut maxBlockSize: size_t = (64 as c_int
        * ((1 as c_int) << 10 as c_int))
        as size_t;
    while requestedBSID as c_uint > proposedBSID as c_uint {
        if srcSize <= maxBlockSize {
            return proposedBSID;
        }
        proposedBSID =
            (proposedBSID as c_int + 1 as c_int) as LZ4F_blockSizeID_t;
        maxBlockSize <<= 2 as c_int;
    }
    return requestedBSID;
} }
unsafe fn LZ4F_compressBound_internal(
    mut srcSize: size_t,
    mut preferencesPtr: *const LZ4F_preferences_t,
    mut alreadyBuffered: size_t,
) -> size_t {
    let mut prefsNull: LZ4F_preferences_t = LZ4F_preferences_t {
        frameInfo: LZ4F_frameInfo_t {
            blockSizeID: LZ4F_max64KB,
            blockMode: LZ4F_blockLinked,
            contentChecksumFlag: LZ4F_noContentChecksum,
            frameType: LZ4F_frame,
            contentSize: 0 as c_ulonglong,
            dictID: 0 as c_uint,
            blockChecksumFlag: LZ4F_noBlockChecksum,
        },
        compressionLevel: 0 as c_int,
        autoFlush: 0 as c_uint,
        favorDecSpeed: 0 as c_uint,
        reserved: [
            0 as c_uint,
            0 as c_uint,
            0 as c_uint,
        ],
    };
    prefsNull.frameInfo.contentChecksumFlag = LZ4F_contentChecksumEnabled;
    prefsNull.frameInfo.blockChecksumFlag = LZ4F_blockChecksumEnabled;
    let prefsPtr: *const LZ4F_preferences_t = if preferencesPtr.is_null() {
        &raw mut prefsNull as *const LZ4F_preferences_t
    } else {
        preferencesPtr
    };
    let flush: U32 =
        (*prefsPtr).autoFlush as U32 | (srcSize == 0 as size_t) as c_int as U32;
    let blockID: LZ4F_blockSizeID_t = (*prefsPtr).frameInfo.blockSizeID;
    let blockSize: size_t = LZ4F_getBlockSize(blockID) as size_t;
    let maxBuffered: size_t = blockSize.wrapping_sub(1 as size_t);
    let bufferedSize: size_t = if alreadyBuffered < maxBuffered {
        alreadyBuffered
    } else {
        maxBuffered
    };
    let maxSrcSize: size_t = srcSize.wrapping_add(bufferedSize);
    let nbFullBlocks: c_uint =
        maxSrcSize.wrapping_div(blockSize) as c_uint;
    let partialBlockSize: size_t = maxSrcSize & blockSize.wrapping_sub(1 as size_t);
    let lastBlockSize: size_t = if flush != 0 {
        partialBlockSize
    } else {
        0 as size_t
    };
    let nbBlocks: c_uint = nbFullBlocks
        .wrapping_add((lastBlockSize > 0 as size_t) as c_int as c_uint);
    let blockCRCSize: size_t =
        BFSize.wrapping_mul((*prefsPtr).frameInfo.blockChecksumFlag as size_t);
    let frameEnd: size_t = BHSize
        .wrapping_add(((*prefsPtr).frameInfo.contentChecksumFlag as size_t).wrapping_mul(BFSize));
    return BHSize
        .wrapping_add(blockCRCSize)
        .wrapping_mul(nbBlocks as size_t)
        .wrapping_add(blockSize.wrapping_mul(nbFullBlocks as size_t))
        .wrapping_add(lastBlockSize)
        .wrapping_add(frameEnd);
}
#[inline]
pub unsafe fn LZ4F_compressFrameBound(
    mut srcSize: size_t,
    mut preferencesPtr: *const LZ4F_preferences_t,
) -> size_t {
    let mut prefs: LZ4F_preferences_t = LZ4F_preferences_t {
        frameInfo: LZ4F_frameInfo_t {
            blockSizeID: LZ4F_default,
            blockMode: LZ4F_blockLinked,
            contentChecksumFlag: LZ4F_noContentChecksum,
            frameType: LZ4F_frame,
            contentSize: 0,
            dictID: 0,
            blockChecksumFlag: LZ4F_noBlockChecksum,
        },
        compressionLevel: 0,
        autoFlush: 0,
        favorDecSpeed: 0,
        reserved: [0; 3],
    };
    let headerSize: size_t = maxFHSize;
    if !preferencesPtr.is_null() {
        prefs = *preferencesPtr;
    } else {
        memset(
            &raw mut prefs as *mut c_void,
            0 as c_int,
            ::core::mem::size_of::<LZ4F_preferences_t>() as size_t,
        );
    }
    prefs.autoFlush = 1 as c_uint;
    return headerSize.wrapping_add(LZ4F_compressBound_internal(
        srcSize,
        &raw mut prefs,
        0 as size_t,
    ));
}
#[inline]
pub unsafe fn LZ4F_compressFrame_usingCDict(
    mut cctx: *mut LZ4F_cctx,
    mut dstBuffer: *mut c_void,
    mut dstCapacity: size_t,
    mut srcBuffer: *const c_void,
    mut srcSize: size_t,
    mut cdict: *const LZ4F_CDict,
    mut preferencesPtr: *const LZ4F_preferences_t,
) -> size_t {
    let mut prefs: LZ4F_preferences_t = LZ4F_preferences_t {
        frameInfo: LZ4F_frameInfo_t {
            blockSizeID: LZ4F_default,
            blockMode: LZ4F_blockLinked,
            contentChecksumFlag: LZ4F_noContentChecksum,
            frameType: LZ4F_frame,
            contentSize: 0,
            dictID: 0,
            blockChecksumFlag: LZ4F_noBlockChecksum,
        },
        compressionLevel: 0,
        autoFlush: 0,
        favorDecSpeed: 0,
        reserved: [0; 3],
    };
    let mut options: LZ4F_compressOptions_t = LZ4F_compressOptions_t {
        stableSrc: 0,
        reserved: [0; 3],
    };
    let dstStart: *mut BYTE = dstBuffer as *mut BYTE;
    let mut dstPtr: *mut BYTE = dstStart;
    let dstEnd: *mut BYTE = dstStart.offset(dstCapacity as isize);
    if !preferencesPtr.is_null() {
        prefs = *preferencesPtr;
    } else {
        memset(
            &raw mut prefs as *mut c_void,
            0 as c_int,
            ::core::mem::size_of::<LZ4F_preferences_t>() as size_t,
        );
    }
    if prefs.frameInfo.contentSize != 0 as c_ulonglong {
        prefs.frameInfo.contentSize = srcSize as U64 as c_ulonglong;
    }
    prefs.frameInfo.blockSizeID = LZ4F_optimalBSID(prefs.frameInfo.blockSizeID, srcSize);
    prefs.autoFlush = 1 as c_uint;
    if srcSize <= LZ4F_getBlockSize(prefs.frameInfo.blockSizeID) {
        prefs.frameInfo.blockMode = LZ4F_blockIndependent;
    }
    memset(
        &raw mut options as *mut c_void,
        0 as c_int,
        ::core::mem::size_of::<LZ4F_compressOptions_t>() as size_t,
    );
    options.stableSrc = 1 as c_uint;
    if dstCapacity < LZ4F_compressFrameBound(srcSize, &raw mut prefs) {
        return LZ4F_returnErrorCode(LZ4F_ERROR_dstMaxSize_tooSmall) as size_t;
    }
    let headerSize: size_t =
        LZ4F_compressBegin_usingCDict(cctx, dstBuffer, dstCapacity, cdict, &raw mut prefs)
            as size_t;
    if LZ4F_isError(headerSize as LZ4F_errorCode_t) != 0 {
        return headerSize;
    }
    dstPtr = dstPtr.offset(headerSize as isize);
    let cSize: size_t = LZ4F_compressUpdate(
        cctx,
        dstPtr as *mut c_void,
        dstEnd.offset_from(dstPtr) as c_long as size_t,
        srcBuffer,
        srcSize,
        &raw mut options,
    ) as size_t;
    if LZ4F_isError(cSize as LZ4F_errorCode_t) != 0 {
        return cSize;
    }
    dstPtr = dstPtr.offset(cSize as isize);
    let tailSize: size_t = LZ4F_compressEnd(
        cctx,
        dstPtr as *mut c_void,
        dstEnd.offset_from(dstPtr) as c_long as size_t,
        &raw mut options,
    ) as size_t;
    if LZ4F_isError(tailSize as LZ4F_errorCode_t) != 0 {
        return tailSize;
    }
    dstPtr = dstPtr.offset(tailSize as isize);
    return dstPtr.offset_from(dstStart) as c_long as size_t;
}
#[inline]
pub unsafe fn LZ4F_compressFrame(
    mut dstBuffer: *mut c_void,
    mut dstCapacity: size_t,
    mut srcBuffer: *const c_void,
    mut srcSize: size_t,
    mut preferencesPtr: *const LZ4F_preferences_t,
) -> size_t {
    let mut result: size_t = 0;
    let mut cctx: LZ4F_cctx_t = LZ4F_cctx_s {
        cmem: LZ4F_CustomMem {
            customAlloc: None,
            customCalloc: None,
            customFree: None,
            opaqueState: ::core::ptr::null_mut::<c_void>(),
        },
        prefs: LZ4F_preferences_t {
            frameInfo: LZ4F_frameInfo_t {
                blockSizeID: LZ4F_default,
                blockMode: LZ4F_blockLinked,
                contentChecksumFlag: LZ4F_noContentChecksum,
                frameType: LZ4F_frame,
                contentSize: 0,
                dictID: 0,
                blockChecksumFlag: LZ4F_noBlockChecksum,
            },
            compressionLevel: 0,
            autoFlush: 0,
            favorDecSpeed: 0,
            reserved: [0; 3],
        },
        version: 0,
        cStage: 0,
        cdict: ::core::ptr::null::<LZ4F_CDict>(),
        maxBlockSize: 0,
        maxBufferSize: 0,
        tmpBuff: ::core::ptr::null_mut::<BYTE>(),
        tmpIn: ::core::ptr::null_mut::<BYTE>(),
        tmpInSize: 0,
        totalInSize: 0,
        xxh: XXH32_state_s {
            total_len_32: 0,
            large_len: 0,
            v1: 0,
            v2: 0,
            v3: 0,
            v4: 0,
            mem32: [0; 4],
            memsize: 0,
            reserved: 0,
        },
        lz4CtxPtr: ::core::ptr::null_mut::<c_void>(),
        lz4CtxAlloc: 0,
        lz4CtxType: 0,
        blockCompressMode: LZ4B_COMPRESSED,
    };
    let mut lz4ctx: LZ4_stream_t = LZ4_stream_u {
        minStateSize: [0; 16416],
    };
    let cctxPtr: *mut LZ4F_cctx_t = &raw mut cctx;
    memset(
        &raw mut cctx as *mut c_void,
        0 as c_int,
        ::core::mem::size_of::<LZ4F_cctx_t>() as size_t,
    );
    cctx.version = LZ4F_VERSION as U32;
    cctx.maxBufferSize = (5 as c_int
        * ((1 as c_int) << 20 as c_int))
        as size_t;
    if preferencesPtr.is_null() || (*preferencesPtr).compressionLevel < LZ4HC_CLEVEL_MIN {
        LZ4_initStream(
            &raw mut lz4ctx as *mut c_void,
            ::core::mem::size_of::<LZ4_stream_t>() as size_t,
        );
        (*cctxPtr).lz4CtxPtr = &raw mut lz4ctx as *mut c_void;
        (*cctxPtr).lz4CtxAlloc = 1 as U16;
        (*cctxPtr).lz4CtxType = ctxFast as c_int as U16;
    }
    result = LZ4F_compressFrame_usingCDict(
        cctxPtr as *mut LZ4F_cctx,
        dstBuffer,
        dstCapacity,
        srcBuffer,
        srcSize,
        ::core::ptr::null::<LZ4F_CDict>(),
        preferencesPtr,
    );
    if !preferencesPtr.is_null() && (*preferencesPtr).compressionLevel >= LZ4HC_CLEVEL_MIN {
        LZ4F_free((*cctxPtr).lz4CtxPtr, (*cctxPtr).cmem);
    }
    return result;
}
#[inline]
pub unsafe fn LZ4F_createCDict_advanced(
    mut cmem: LZ4F_CustomMem,
    mut dictBuffer: *const c_void,
    mut dictSize: size_t,
) -> *mut LZ4F_CDict {
    let mut dictStart: *const c_char = dictBuffer as *const c_char;
    let mut cdict: *mut LZ4F_CDict = ::core::ptr::null_mut::<LZ4F_CDict>();
    if dictStart.is_null() {
        return ::core::ptr::null_mut::<LZ4F_CDict>();
    }
    cdict = LZ4F_malloc(::core::mem::size_of::<LZ4F_CDict>() as size_t, cmem) as *mut LZ4F_CDict;
    if cdict.is_null() {
        return ::core::ptr::null_mut::<LZ4F_CDict>();
    }
    (*cdict).cmem = cmem;
    if dictSize
        > (64 as c_int * ((1 as c_int) << 10 as c_int))
            as size_t
    {
        dictStart = dictStart.offset(dictSize.wrapping_sub(
            (64 as c_int * ((1 as c_int) << 10 as c_int))
                as size_t,
        ) as isize);
        dictSize = (64 as c_int
            * ((1 as c_int) << 10 as c_int)) as size_t;
    }
    (*cdict).dictContent = LZ4F_malloc(dictSize, cmem);
    (*cdict).fastCtx =
        LZ4F_malloc(::core::mem::size_of::<LZ4_stream_t>() as size_t, cmem) as *mut LZ4_stream_t;
    (*cdict).HCCtx = LZ4F_malloc(::core::mem::size_of::<LZ4_streamHC_t>() as size_t, cmem)
        as *mut LZ4_streamHC_t;
    if (*cdict).dictContent.is_null() || (*cdict).fastCtx.is_null() || (*cdict).HCCtx.is_null() {
        LZ4F_freeCDict(cdict);
        return ::core::ptr::null_mut::<LZ4F_CDict>();
    }
    memcpy(
        (*cdict).dictContent,
        dictStart as *const c_void,
        dictSize,
    );
    LZ4_initStream(
        (*cdict).fastCtx as *mut c_void,
        ::core::mem::size_of::<LZ4_stream_t>() as size_t,
    );
    LZ4_loadDictSlow(
        (*cdict).fastCtx,
        (*cdict).dictContent as *const c_char,
        dictSize as c_int,
    );
    LZ4_initStreamHC(
        (*cdict).HCCtx as *mut c_void,
        ::core::mem::size_of::<LZ4_streamHC_t>() as size_t,
    );
    LZ4_setCompressionLevel((*cdict).HCCtx, LZ4HC_CLEVEL_DEFAULT);
    LZ4_loadDictHC(
        (*cdict).HCCtx,
        (*cdict).dictContent as *const c_char,
        dictSize as c_int,
    );
    return cdict;
}
#[inline]
pub unsafe fn LZ4F_createCDict(
    mut dictBuffer: *const c_void,
    mut dictSize: size_t,
) -> *mut LZ4F_CDict {
    return LZ4F_createCDict_advanced(LZ4F_defaultCMem, dictBuffer, dictSize);
}
#[inline]
pub unsafe fn LZ4F_freeCDict(mut cdict: *mut LZ4F_CDict) {
    if cdict.is_null() {
        return;
    }
    LZ4F_free((*cdict).dictContent, (*cdict).cmem);
    LZ4F_free((*cdict).fastCtx as *mut c_void, (*cdict).cmem);
    LZ4F_free((*cdict).HCCtx as *mut c_void, (*cdict).cmem);
    LZ4F_free(cdict as *mut c_void, (*cdict).cmem);
}
#[inline]
pub fn LZ4F_createCompressionContext_advanced(
    mut customMem: LZ4F_CustomMem,
    mut version: c_uint,
) -> *mut LZ4F_cctx { unsafe {
    let cctxPtr: *mut LZ4F_cctx =
        LZ4F_calloc(::core::mem::size_of::<LZ4F_cctx>() as size_t, customMem) as *mut LZ4F_cctx;
    if cctxPtr.is_null() {
        return ::core::ptr::null_mut::<LZ4F_cctx>();
    }
    (*cctxPtr).cmem = customMem;
    (*cctxPtr).version = version as U32;
    (*cctxPtr).cStage = 0 as U32;
    return cctxPtr;
} }
#[inline]
pub unsafe fn LZ4F_createCompressionContext(
    mut LZ4F_compressionContextPtr: *mut *mut LZ4F_cctx,
    mut version: c_uint,
) -> LZ4F_errorCode_t {
    if LZ4F_compressionContextPtr.is_null() {
        return LZ4F_returnErrorCode(LZ4F_ERROR_parameter_null);
    }
    *LZ4F_compressionContextPtr = LZ4F_createCompressionContext_advanced(LZ4F_defaultCMem, version);
    if (*LZ4F_compressionContextPtr).is_null() {
        return LZ4F_returnErrorCode(LZ4F_ERROR_allocation_failed);
    }
    return LZ4F_OK_NoError as c_int as LZ4F_errorCode_t;
}
#[inline]
pub unsafe fn LZ4F_freeCompressionContext(
    mut cctxPtr: *mut LZ4F_cctx,
) -> LZ4F_errorCode_t {
    if !cctxPtr.is_null() {
        LZ4F_free((*cctxPtr).lz4CtxPtr, (*cctxPtr).cmem);
        LZ4F_free(
            (*cctxPtr).tmpBuff as *mut c_void,
            (*cctxPtr).cmem,
        );
        LZ4F_free(cctxPtr as *mut c_void, (*cctxPtr).cmem);
    }
    return LZ4F_OK_NoError as c_int as LZ4F_errorCode_t;
}
unsafe fn LZ4F_initStream(
    mut ctx: *mut c_void,
    mut cdict: *const LZ4F_CDict,
    mut level: c_int,
    mut blockMode: LZ4F_blockMode_t,
) {
    if level < LZ4HC_CLEVEL_MIN {
        if !cdict.is_null()
            || blockMode as c_uint
                == LZ4F_blockLinked as c_int as c_uint
        {
            LZ4_resetStream_fast(ctx as *mut LZ4_stream_t);
            if !cdict.is_null() {
                LZ4_attach_dictionary(ctx as *mut LZ4_stream_t, (*cdict).fastCtx);
            }
        }
    } else {
        LZ4_resetStreamHC_fast(ctx as *mut LZ4_streamHC_t, level);
        if !cdict.is_null() {
            LZ4_attach_HC_dictionary(ctx as *mut LZ4_streamHC_t, (*cdict).HCCtx);
        }
    };
}
fn ctxTypeID_to_size(mut ctxTypeID: c_int) -> c_int { {
    match ctxTypeID {
        1 => return LZ4_sizeofState(),
        2 => return LZ4_sizeofStateHC(),
        _ => return 0 as c_int,
    };
} }
#[inline]
pub unsafe fn LZ4F_cctx_size(mut cctx: *const LZ4F_cctx) -> size_t {
    if cctx.is_null() {
        return 0 as size_t;
    }
    return (::core::mem::size_of::<LZ4F_cctx>() as size_t)
        .wrapping_add((*cctx).maxBufferSize)
        .wrapping_add(ctxTypeID_to_size((*cctx).lz4CtxAlloc as c_int) as size_t);
}
unsafe fn LZ4F_compressBegin_internal(
    mut cctx: *mut LZ4F_cctx,
    mut dstBuffer: *mut c_void,
    mut dstCapacity: size_t,
    mut dictBuffer: *const c_void,
    mut dictSize: size_t,
    mut cdict: *const LZ4F_CDict,
    mut preferencesPtr: *const LZ4F_preferences_t,
) -> size_t {
    let cctx_view: &mut LZ4F_cctx = unsafe { &mut *cctx };
    let prefNull: LZ4F_preferences_t = LZ4F_preferences_t {
        frameInfo: LZ4F_frameInfo_t {
            blockSizeID: LZ4F_max64KB,
            blockMode: LZ4F_blockLinked,
            contentChecksumFlag: LZ4F_noContentChecksum,
            frameType: LZ4F_frame,
            contentSize: 0 as c_ulonglong,
            dictID: 0 as c_uint,
            blockChecksumFlag: LZ4F_noBlockChecksum,
        },
        compressionLevel: 0 as c_int,
        autoFlush: 0 as c_uint,
        favorDecSpeed: 0 as c_uint,
        reserved: [
            0 as c_uint,
            0 as c_uint,
            0 as c_uint,
        ],
    };
    let dstStart: *mut BYTE = dstBuffer as *mut BYTE;
    let mut dstPtr: *mut BYTE = dstStart;
    if dstCapacity < maxFHSize {
        return LZ4F_returnErrorCode(LZ4F_ERROR_dstMaxSize_tooSmall) as size_t;
    }
    if preferencesPtr.is_null() {
        preferencesPtr = &raw const prefNull;
    }
    cctx_view.prefs = *preferencesPtr;
    let ctxTypeID: U16 = (if cctx_view.prefs.compressionLevel < LZ4HC_CLEVEL_MIN {
        1 as c_int
    } else {
        2 as c_int
    }) as U16;
    let mut requiredSize: c_int = ctxTypeID_to_size(ctxTypeID as c_int);
    let mut allocatedSize: c_int =
        ctxTypeID_to_size(cctx_view.lz4CtxAlloc as c_int);
    if allocatedSize < requiredSize {
        LZ4F_free(cctx_view.lz4CtxPtr, cctx_view.cmem);
        if cctx_view.prefs.compressionLevel < LZ4HC_CLEVEL_MIN {
            cctx_view.lz4CtxPtr = LZ4F_malloc(
                ::core::mem::size_of::<LZ4_stream_t>() as size_t,
                cctx_view.cmem,
            );
            if !cctx_view.lz4CtxPtr.is_null() {
                LZ4_initStream(
                    cctx_view.lz4CtxPtr,
                    ::core::mem::size_of::<LZ4_stream_t>() as size_t,
                );
            }
        } else {
            cctx_view.lz4CtxPtr = LZ4F_malloc(
                ::core::mem::size_of::<LZ4_streamHC_t>() as size_t,
                cctx_view.cmem,
            );
            if !cctx_view.lz4CtxPtr.is_null() {
                LZ4_initStreamHC(
                    cctx_view.lz4CtxPtr,
                    ::core::mem::size_of::<LZ4_streamHC_t>() as size_t,
                );
            }
        }
        if cctx_view.lz4CtxPtr.is_null() {
            return LZ4F_returnErrorCode(LZ4F_ERROR_allocation_failed) as size_t;
        }
        cctx_view.lz4CtxAlloc = ctxTypeID;
        cctx_view.lz4CtxType = ctxTypeID;
    } else if cctx_view.lz4CtxType as c_int != ctxTypeID as c_int {
        if cctx_view.prefs.compressionLevel < LZ4HC_CLEVEL_MIN {
            LZ4_initStream(
                cctx_view.lz4CtxPtr as *mut LZ4_stream_t as *mut c_void,
                ::core::mem::size_of::<LZ4_stream_t>() as size_t,
            );
        } else {
            LZ4_initStreamHC(
                cctx_view.lz4CtxPtr as *mut LZ4_streamHC_t as *mut c_void,
                ::core::mem::size_of::<LZ4_streamHC_t>() as size_t,
            );
            LZ4_setCompressionLevel(
                cctx_view.lz4CtxPtr as *mut LZ4_streamHC_t,
                cctx_view.prefs.compressionLevel,
            );
        }
        cctx_view.lz4CtxType = ctxTypeID;
    }
    if cctx_view.prefs.frameInfo.blockSizeID as c_uint == 0 as c_uint {
        cctx_view.prefs.frameInfo.blockSizeID = LZ4F_max64KB;
    }
    cctx_view.maxBlockSize = LZ4F_getBlockSize(cctx_view.prefs.frameInfo.blockSizeID);
    let requiredBuffSize: size_t = if (*preferencesPtr).autoFlush != 0 {
        (if cctx_view.prefs.frameInfo.blockMode as c_uint
            == LZ4F_blockLinked as c_int as c_uint
        {
            64 as c_int * ((1 as c_int) << 10 as c_int)
        } else {
            0 as c_int
        }) as size_t
    } else {
        cctx_view.maxBlockSize.wrapping_add(
            (if cctx_view.prefs.frameInfo.blockMode as c_uint
                == LZ4F_blockLinked as c_int as c_uint
            {
                128 as c_int * ((1 as c_int) << 10 as c_int)
            } else {
                0 as c_int
            }) as size_t,
        )
    };
    if cctx_view.maxBufferSize < requiredBuffSize {
        cctx_view.maxBufferSize = 0 as size_t;
        LZ4F_free(cctx_view.tmpBuff as *mut c_void, cctx_view.cmem);
        cctx_view.tmpBuff = LZ4F_malloc(requiredBuffSize, cctx_view.cmem) as *mut BYTE;
        if cctx_view.tmpBuff.is_null() {
            return LZ4F_returnErrorCode(LZ4F_ERROR_allocation_failed) as size_t;
        }
        cctx_view.maxBufferSize = requiredBuffSize;
    }
    cctx_view.tmpIn = cctx_view.tmpBuff;
    cctx_view.tmpInSize = 0 as size_t;
    XXH32_reset(&raw mut cctx_view.xxh, 0 as c_uint);
    cctx_view.cdict = cdict;
    if cctx_view.prefs.frameInfo.blockMode as c_uint
        == LZ4F_blockLinked as c_int as c_uint
    {
        LZ4F_initStream(
            cctx_view.lz4CtxPtr,
            cdict,
            cctx_view.prefs.compressionLevel,
            LZ4F_blockLinked,
        );
    }
    if (*preferencesPtr).compressionLevel >= LZ4HC_CLEVEL_MIN {
        LZ4_favorDecompressionSpeed(
            cctx_view.lz4CtxPtr as *mut LZ4_streamHC_t,
            (*preferencesPtr).favorDecSpeed as c_int,
        );
    }
    if !dictBuffer.is_null() {
        if dictSize > 2147483647 as c_int as size_t {
            return LZ4F_returnErrorCode(LZ4F_ERROR_parameter_invalid) as size_t;
        }
        if cctx_view.lz4CtxType as c_int == ctxFast as c_int {
            LZ4_loadDict(
                cctx_view.lz4CtxPtr as *mut LZ4_stream_t,
                dictBuffer as *const c_char,
                dictSize as c_int,
            );
        } else {
            LZ4_loadDictHC(
                cctx_view.lz4CtxPtr as *mut LZ4_streamHC_t,
                dictBuffer as *const c_char,
                dictSize as c_int,
            );
        }
    }
    LZ4F_writeLE32(dstPtr as *mut c_void, LZ4F_MAGICNUMBER as U32);
    dstPtr = dstPtr.offset(4 as c_int as isize);
    let headerStart: *mut BYTE = dstPtr;
    let fresh0 = dstPtr;
    dstPtr = dstPtr.offset(1);
    *fresh0 = (((1 as c_int & _2BITS) << 6 as c_int)
        as c_uint)
        .wrapping_add(
            (cctx_view.prefs.frameInfo.blockMode as c_uint
                & _1BIT as c_uint)
                << 5 as c_int,
        )
        .wrapping_add(
            (cctx_view.prefs.frameInfo.blockChecksumFlag as c_uint
                & _1BIT as c_uint)
                << 4 as c_int,
        )
        .wrapping_add(
            ((cctx_view.prefs.frameInfo.contentSize > 0 as c_ulonglong)
                as c_int as c_uint)
                << 3 as c_int,
        )
        .wrapping_add(
            (cctx_view.prefs.frameInfo.contentChecksumFlag as c_uint
                & _1BIT as c_uint)
                << 2 as c_int,
        )
        .wrapping_add(
            (cctx_view.prefs.frameInfo.dictID > 0 as c_uint) as c_int
                as c_uint,
        ) as BYTE;
    let fresh1 = dstPtr;
    dstPtr = dstPtr.offset(1);
    *fresh1 = ((cctx_view.prefs.frameInfo.blockSizeID as c_uint
        & _3BITS as c_uint)
        << 4 as c_int) as BYTE;
    if cctx_view.prefs.frameInfo.contentSize != 0 {
        LZ4F_writeLE64(
            dstPtr as *mut c_void,
            cctx_view.prefs.frameInfo.contentSize as U64,
        );
        dstPtr = dstPtr.offset(8 as c_int as isize);
        cctx_view.totalInSize = 0 as U64;
    }
    if cctx_view.prefs.frameInfo.dictID != 0 {
        LZ4F_writeLE32(
            dstPtr as *mut c_void,
            cctx_view.prefs.frameInfo.dictID as U32,
        );
        dstPtr = dstPtr.offset(4 as c_int as isize);
    }
    *dstPtr = LZ4F_headerChecksum(
        headerStart as *const c_void,
        dstPtr.offset_from(headerStart) as c_long as size_t,
    );
    dstPtr = dstPtr.offset(1);
    cctx_view.cStage = 1 as U32;
    return dstPtr.offset_from(dstStart) as c_long as size_t;
}
#[inline]
pub unsafe fn LZ4F_compressBegin(
    mut cctx: *mut LZ4F_cctx,
    mut dstBuffer: *mut c_void,
    mut dstCapacity: size_t,
    mut preferencesPtr: *const LZ4F_preferences_t,
) -> size_t {
    return LZ4F_compressBegin_internal(
        cctx,
        dstBuffer,
        dstCapacity,
        ::core::ptr::null::<c_void>(),
        0 as size_t,
        ::core::ptr::null::<LZ4F_CDict>(),
        preferencesPtr,
    );
}
unsafe fn LZ4F_compressBegin_usingDictOnce(
    mut cctx: *mut LZ4F_cctx,
    mut dstBuffer: *mut c_void,
    mut dstCapacity: size_t,
    mut dict: *const c_void,
    mut dictSize: size_t,
    mut preferencesPtr: *const LZ4F_preferences_t,
) -> size_t {
    return LZ4F_compressBegin_internal(
        cctx,
        dstBuffer,
        dstCapacity,
        dict,
        dictSize,
        ::core::ptr::null::<LZ4F_CDict>(),
        preferencesPtr,
    );
}
#[inline]
pub unsafe fn LZ4F_compressBegin_usingDict(
    mut cctx: *mut LZ4F_cctx,
    mut dstBuffer: *mut c_void,
    mut dstCapacity: size_t,
    mut dict: *const c_void,
    mut dictSize: size_t,
    mut preferencesPtr: *const LZ4F_preferences_t,
) -> size_t {
    return LZ4F_compressBegin_usingDictOnce(
        cctx,
        dstBuffer,
        dstCapacity,
        dict,
        dictSize,
        preferencesPtr,
    );
}
#[inline]
pub unsafe fn LZ4F_compressBegin_usingCDict(
    mut cctx: *mut LZ4F_cctx,
    mut dstBuffer: *mut c_void,
    mut dstCapacity: size_t,
    mut cdict: *const LZ4F_CDict,
    mut preferencesPtr: *const LZ4F_preferences_t,
) -> size_t {
    return LZ4F_compressBegin_internal(
        cctx,
        dstBuffer,
        dstCapacity,
        ::core::ptr::null::<c_void>(),
        0 as size_t,
        cdict,
        preferencesPtr,
    );
}
#[inline]
pub unsafe fn LZ4F_compressBound(
    mut srcSize: size_t,
    mut preferencesPtr: *const LZ4F_preferences_t,
) -> size_t {
    if !preferencesPtr.is_null() && (*preferencesPtr).autoFlush != 0 {
        return LZ4F_compressBound_internal(srcSize, preferencesPtr, 0 as size_t);
    }
    return LZ4F_compressBound_internal(
        srcSize,
        preferencesPtr,
        -(1 as c_int) as size_t,
    );
}
unsafe fn LZ4F_makeBlock(
    mut dst: *mut c_void,
    mut src: *const c_void,
    mut srcSize: size_t,
    mut compress: compressFunc_t,
    mut lz4ctx: *mut c_void,
    mut level: c_int,
    mut cdict: *const LZ4F_CDict,
    mut crcFlag: LZ4F_blockChecksum_t,
) -> size_t {
    let cSizePtr: *mut BYTE = dst as *mut BYTE;
    let mut dstCapacity: c_int = if srcSize > 1 as size_t {
        srcSize as c_int - 1 as c_int
    } else {
        1 as c_int
    };
    let mut cSize: U32 = 0;
    cSize = compress.expect("non-null function pointer")(
        lz4ctx,
        src as *const c_char,
        cSizePtr.offset(BHSize as isize) as *mut c_char,
        srcSize as c_int,
        dstCapacity,
        level,
        cdict,
    ) as U32;
    if cSize == 0 as U32 || cSize as size_t >= srcSize {
        cSize = srcSize as U32;
        LZ4F_writeLE32(
            cSizePtr as *mut c_void,
            cSize | LZ4F_BLOCKUNCOMPRESSED_FLAG as U32,
        );
        memcpy(
            cSizePtr.offset(BHSize as isize) as *mut c_void,
            src,
            srcSize,
        );
    } else {
        LZ4F_writeLE32(cSizePtr as *mut c_void, cSize);
    }
    if crcFlag as u64 != 0 {
        let crc32: U32 = XXH32(
            cSizePtr.offset(BHSize as isize) as *const c_void,
            cSize as size_t,
            0 as c_uint,
        ) as U32;
        LZ4F_writeLE32(
            cSizePtr.offset(BHSize as isize).offset(cSize as isize) as *mut c_void,
            crc32,
        );
    }
    return BHSize
        .wrapping_add(cSize as size_t)
        .wrapping_add((crcFlag as U32 as size_t).wrapping_mul(BFSize));
}
unsafe extern "C" fn LZ4F_compressBlock(
    mut ctx: *mut c_void,
    mut src: *const c_char,
    mut dst: *mut c_char,
    mut srcSize: c_int,
    mut dstCapacity: c_int,
    mut level: c_int,
    mut cdict: *const LZ4F_CDict,
) -> c_int {
    let acceleration: c_int = if level < 0 as c_int {
        -level + 1 as c_int
    } else {
        1 as c_int
    };
    LZ4F_initStream(ctx, cdict, level, LZ4F_blockIndependent);
    if !cdict.is_null() {
        return LZ4_compress_fast_continue(
            ctx as *mut LZ4_stream_t,
            src,
            dst,
            srcSize,
            dstCapacity,
            acceleration,
        );
    } else {
        return LZ4_compress_fast_extState_fastReset(
            ctx,
            src,
            dst,
            srcSize,
            dstCapacity,
            acceleration,
        );
    };
}
unsafe extern "C" fn LZ4F_compressBlock_continue(
    mut ctx: *mut c_void,
    mut src: *const c_char,
    mut dst: *mut c_char,
    mut srcSize: c_int,
    mut dstCapacity: c_int,
    mut level: c_int,
    mut cdict: *const LZ4F_CDict,
) -> c_int {
    let acceleration: c_int = if level < 0 as c_int {
        -level + 1 as c_int
    } else {
        1 as c_int
    };
    return LZ4_compress_fast_continue(
        ctx as *mut LZ4_stream_t,
        src,
        dst,
        srcSize,
        dstCapacity,
        acceleration,
    );
}
unsafe extern "C" fn LZ4F_compressBlockHC(
    mut ctx: *mut c_void,
    mut src: *const c_char,
    mut dst: *mut c_char,
    mut srcSize: c_int,
    mut dstCapacity: c_int,
    mut level: c_int,
    mut cdict: *const LZ4F_CDict,
) -> c_int {
    LZ4F_initStream(ctx, cdict, level, LZ4F_blockIndependent);
    if !cdict.is_null() {
        return LZ4_compress_HC_continue(
            ctx as *mut LZ4_streamHC_t,
            src,
            dst,
            srcSize,
            dstCapacity,
        );
    }
    return LZ4_compress_HC_extStateHC_fastReset(ctx, src, dst, srcSize, dstCapacity, level);
}
unsafe extern "C" fn LZ4F_compressBlockHC_continue(
    mut ctx: *mut c_void,
    mut src: *const c_char,
    mut dst: *mut c_char,
    mut srcSize: c_int,
    mut dstCapacity: c_int,
    mut level: c_int,
    mut cdict: *const LZ4F_CDict,
) -> c_int {
    return LZ4_compress_HC_continue(ctx as *mut LZ4_streamHC_t, src, dst, srcSize, dstCapacity);
}
unsafe extern "C" fn LZ4F_doNotCompressBlock(
    mut ctx: *mut c_void,
    mut src: *const c_char,
    mut dst: *mut c_char,
    mut srcSize: c_int,
    mut dstCapacity: c_int,
    mut level: c_int,
    mut cdict: *const LZ4F_CDict,
) -> c_int {
    return 0 as c_int;
}
fn LZ4F_selectCompression(
    mut blockMode: LZ4F_blockMode_t,
    mut level: c_int,
    mut compressMode: LZ4F_BlockCompressMode_e,
) -> compressFunc_t { {
    if compressMode as c_uint
        == LZ4B_UNCOMPRESSED as c_int as c_uint
    {
        return Some(
            LZ4F_doNotCompressBlock
                as unsafe extern "C" fn(
                    *mut c_void,
                    *const c_char,
                    *mut c_char,
                    c_int,
                    c_int,
                    c_int,
                    *const LZ4F_CDict,
                ) -> c_int,
        );
    }
    if level < LZ4HC_CLEVEL_MIN {
        if blockMode as c_uint
            == LZ4F_blockIndependent as c_int as c_uint
        {
            return Some(
                LZ4F_compressBlock
                    as unsafe extern "C" fn(
                        *mut c_void,
                        *const c_char,
                        *mut c_char,
                        c_int,
                        c_int,
                        c_int,
                        *const LZ4F_CDict,
                    ) -> c_int,
            );
        }
        return Some(
            LZ4F_compressBlock_continue
                as unsafe extern "C" fn(
                    *mut c_void,
                    *const c_char,
                    *mut c_char,
                    c_int,
                    c_int,
                    c_int,
                    *const LZ4F_CDict,
                ) -> c_int,
        );
    }
    if blockMode as c_uint
        == LZ4F_blockIndependent as c_int as c_uint
    {
        return Some(
            LZ4F_compressBlockHC
                as unsafe extern "C" fn(
                    *mut c_void,
                    *const c_char,
                    *mut c_char,
                    c_int,
                    c_int,
                    c_int,
                    *const LZ4F_CDict,
                ) -> c_int,
        );
    }
    return Some(
        LZ4F_compressBlockHC_continue
            as unsafe extern "C" fn(
                *mut c_void,
                *const c_char,
                *mut c_char,
                c_int,
                c_int,
                c_int,
                *const LZ4F_CDict,
            ) -> c_int,
    );
} }
unsafe fn LZ4F_localSaveDict(mut cctxPtr: *mut LZ4F_cctx_t) {
    let cctxPtr_view: &mut LZ4F_cctx_t = unsafe { &mut *cctxPtr };
    let dictSize: c_int = if cctxPtr_view.prefs.compressionLevel < LZ4HC_CLEVEL_MIN {
        LZ4_saveDict(
            cctxPtr_view.lz4CtxPtr as *mut LZ4_stream_t,
            cctxPtr_view.tmpBuff as *mut c_char,
            64 as c_int * ((1 as c_int) << 10 as c_int),
        ) as c_int
    } else {
        LZ4_saveDictHC(
            cctxPtr_view.lz4CtxPtr as *mut LZ4_streamHC_t,
            cctxPtr_view.tmpBuff as *mut c_char,
            64 as c_int * ((1 as c_int) << 10 as c_int),
        ) as c_int
    };
    cctxPtr_view.tmpIn = cctxPtr_view.tmpBuff.offset(dictSize as isize);
}
static mut k_cOptionsNull: LZ4F_compressOptions_t = LZ4F_compressOptions_t {
    stableSrc: 0 as c_uint,
    reserved: [
        0 as c_int as c_uint,
        0 as c_int as c_uint,
        0 as c_int as c_uint,
    ],
};
unsafe fn LZ4F_compressUpdateImpl(
    mut cctxPtr: *mut LZ4F_cctx,
    mut dstBuffer: *mut c_void,
    mut dstCapacity: size_t,
    mut srcBuffer: *const c_void,
    mut srcSize: size_t,
    mut compressOptionsPtr: *const LZ4F_compressOptions_t,
    mut blockCompression: LZ4F_BlockCompressMode_e,
) -> size_t {
    let blockSize: size_t = (*cctxPtr).maxBlockSize;
    let mut srcPtr: *const BYTE = srcBuffer as *const BYTE;
    let srcEnd: *const BYTE = if srcSize != 0 {
        srcPtr.offset(srcSize as isize)
    } else {
        srcPtr
    };
    let dstStart: *mut BYTE = dstBuffer as *mut BYTE;
    let mut dstPtr: *mut BYTE = dstStart;
    let mut lastBlockCompressed: LZ4F_lastBlockStatus = notDone;
    let compress: compressFunc_t = LZ4F_selectCompression(
        (*cctxPtr).prefs.frameInfo.blockMode,
        (*cctxPtr).prefs.compressionLevel,
        blockCompression,
    ) as compressFunc_t;
    let mut bytesWritten: size_t = 0;
    if (*cctxPtr).cStage != 1 as U32 {
        return LZ4F_returnErrorCode(LZ4F_ERROR_compressionState_uninitialized) as size_t;
    }
    if dstCapacity
        < LZ4F_compressBound_internal(srcSize, &raw mut (*cctxPtr).prefs, (*cctxPtr).tmpInSize)
    {
        return LZ4F_returnErrorCode(LZ4F_ERROR_dstMaxSize_tooSmall) as size_t;
    }
    if blockCompression as c_uint
        == LZ4B_UNCOMPRESSED as c_int as c_uint
        && dstCapacity < srcSize
    {
        return LZ4F_returnErrorCode(LZ4F_ERROR_dstMaxSize_tooSmall) as size_t;
    }
    if (*cctxPtr).blockCompressMode as c_uint
        != blockCompression as c_uint
    {
        bytesWritten = LZ4F_flush(cctxPtr, dstBuffer, dstCapacity, compressOptionsPtr);
        dstPtr = dstPtr.offset(bytesWritten as isize);
        (*cctxPtr).blockCompressMode = blockCompression;
    }
    if compressOptionsPtr.is_null() {
        compressOptionsPtr = &raw const k_cOptionsNull;
    }
    if (*cctxPtr).tmpInSize > 0 as size_t {
        let sizeToCopy: size_t = blockSize.wrapping_sub((*cctxPtr).tmpInSize);
        if sizeToCopy > srcSize {
            memcpy(
                (*cctxPtr).tmpIn.offset((*cctxPtr).tmpInSize as isize) as *mut c_void,
                srcBuffer,
                srcSize,
            );
            srcPtr = srcEnd;
            (*cctxPtr).tmpInSize = (*cctxPtr).tmpInSize.wrapping_add(srcSize);
        } else {
            lastBlockCompressed = fromTmpBuffer;
            memcpy(
                (*cctxPtr).tmpIn.offset((*cctxPtr).tmpInSize as isize) as *mut c_void,
                srcBuffer,
                sizeToCopy,
            );
            srcPtr = srcPtr.offset(sizeToCopy as isize);
            dstPtr = dstPtr.offset(LZ4F_makeBlock(
                dstPtr as *mut c_void,
                (*cctxPtr).tmpIn as *const c_void,
                blockSize,
                compress,
                (*cctxPtr).lz4CtxPtr,
                (*cctxPtr).prefs.compressionLevel,
                (*cctxPtr).cdict,
                (*cctxPtr).prefs.frameInfo.blockChecksumFlag,
            ) as isize);
            if (*cctxPtr).prefs.frameInfo.blockMode as c_uint
                == LZ4F_blockLinked as c_int as c_uint
            {
                (*cctxPtr).tmpIn = (*cctxPtr).tmpIn.offset(blockSize as isize);
            }
            (*cctxPtr).tmpInSize = 0 as size_t;
        }
    }
    while srcEnd.offset_from(srcPtr) as c_long as size_t >= blockSize {
        lastBlockCompressed = fromSrcBuffer;
        dstPtr = dstPtr.offset(LZ4F_makeBlock(
            dstPtr as *mut c_void,
            srcPtr as *const c_void,
            blockSize,
            compress,
            (*cctxPtr).lz4CtxPtr,
            (*cctxPtr).prefs.compressionLevel,
            (*cctxPtr).cdict,
            (*cctxPtr).prefs.frameInfo.blockChecksumFlag,
        ) as isize);
        srcPtr = srcPtr.offset(blockSize as isize);
    }
    if (*cctxPtr).prefs.autoFlush != 0 && srcPtr < srcEnd {
        lastBlockCompressed = fromSrcBuffer;
        dstPtr = dstPtr.offset(LZ4F_makeBlock(
            dstPtr as *mut c_void,
            srcPtr as *const c_void,
            srcEnd.offset_from(srcPtr) as c_long as size_t,
            compress,
            (*cctxPtr).lz4CtxPtr,
            (*cctxPtr).prefs.compressionLevel,
            (*cctxPtr).cdict,
            (*cctxPtr).prefs.frameInfo.blockChecksumFlag,
        ) as isize);
        srcPtr = srcEnd;
    }
    if (*cctxPtr).prefs.frameInfo.blockMode as c_uint
        == LZ4F_blockLinked as c_int as c_uint
        && lastBlockCompressed as c_uint
            == fromSrcBuffer as c_int as c_uint
    {
        if (*compressOptionsPtr).stableSrc != 0 {
            (*cctxPtr).tmpIn = (*cctxPtr).tmpBuff;
        } else {
            LZ4F_localSaveDict(cctxPtr as *mut LZ4F_cctx_t);
        }
    }
    if (*cctxPtr).prefs.autoFlush == 0
        && (*cctxPtr).tmpIn.offset(blockSize as isize)
            > (*cctxPtr).tmpBuff.offset((*cctxPtr).maxBufferSize as isize)
    {
        LZ4F_localSaveDict(cctxPtr as *mut LZ4F_cctx_t);
    }
    if srcPtr < srcEnd {
        let sizeToCopy_0: size_t = srcEnd.offset_from(srcPtr) as c_long as size_t;
        memcpy(
            (*cctxPtr).tmpIn as *mut c_void,
            srcPtr as *const c_void,
            sizeToCopy_0,
        );
        (*cctxPtr).tmpInSize = sizeToCopy_0;
    }
    if (*cctxPtr).prefs.frameInfo.contentChecksumFlag as c_uint
        == LZ4F_contentChecksumEnabled as c_int as c_uint
    {
        XXH32_update(&raw mut (*cctxPtr).xxh, srcBuffer, srcSize);
    }
    (*cctxPtr).totalInSize = ((*cctxPtr).totalInSize as c_ulong)
        .wrapping_add(srcSize as c_ulong) as U64 as U64;
    return dstPtr.offset_from(dstStart) as c_long as size_t;
}
#[inline]
pub unsafe fn LZ4F_compressUpdate(
    mut cctxPtr: *mut LZ4F_cctx,
    mut dstBuffer: *mut c_void,
    mut dstCapacity: size_t,
    mut srcBuffer: *const c_void,
    mut srcSize: size_t,
    mut compressOptionsPtr: *const LZ4F_compressOptions_t,
) -> size_t {
    return LZ4F_compressUpdateImpl(
        cctxPtr,
        dstBuffer,
        dstCapacity,
        srcBuffer,
        srcSize,
        compressOptionsPtr,
        LZ4B_COMPRESSED,
    );
}
#[inline]
pub unsafe fn LZ4F_uncompressedUpdate(
    mut cctxPtr: *mut LZ4F_cctx,
    mut dstBuffer: *mut c_void,
    mut dstCapacity: size_t,
    mut srcBuffer: *const c_void,
    mut srcSize: size_t,
    mut compressOptionsPtr: *const LZ4F_compressOptions_t,
) -> size_t {
    return LZ4F_compressUpdateImpl(
        cctxPtr,
        dstBuffer,
        dstCapacity,
        srcBuffer,
        srcSize,
        compressOptionsPtr,
        LZ4B_UNCOMPRESSED,
    );
}
#[inline]
pub unsafe fn LZ4F_flush(
    mut cctxPtr: *mut LZ4F_cctx,
    mut dstBuffer: *mut c_void,
    mut dstCapacity: size_t,
    mut compressOptionsPtr: *const LZ4F_compressOptions_t,
) -> size_t {
    let dstStart: *mut BYTE = dstBuffer as *mut BYTE;
    let mut dstPtr: *mut BYTE = dstStart;
    let mut compress: compressFunc_t = None;
    if (*cctxPtr).tmpInSize == 0 as size_t {
        return 0 as size_t;
    }
    if (*cctxPtr).cStage != 1 as U32 {
        return LZ4F_returnErrorCode(LZ4F_ERROR_compressionState_uninitialized) as size_t;
    }
    if dstCapacity
        < (*cctxPtr)
            .tmpInSize
            .wrapping_add(BHSize)
            .wrapping_add(BFSize)
    {
        return LZ4F_returnErrorCode(LZ4F_ERROR_dstMaxSize_tooSmall) as size_t;
    }
    compress = LZ4F_selectCompression(
        (*cctxPtr).prefs.frameInfo.blockMode,
        (*cctxPtr).prefs.compressionLevel,
        (*cctxPtr).blockCompressMode,
    );
    dstPtr = dstPtr.offset(LZ4F_makeBlock(
        dstPtr as *mut c_void,
        (*cctxPtr).tmpIn as *const c_void,
        (*cctxPtr).tmpInSize,
        compress,
        (*cctxPtr).lz4CtxPtr,
        (*cctxPtr).prefs.compressionLevel,
        (*cctxPtr).cdict,
        (*cctxPtr).prefs.frameInfo.blockChecksumFlag,
    ) as isize);
    if (*cctxPtr).prefs.frameInfo.blockMode as c_uint
        == LZ4F_blockLinked as c_int as c_uint
    {
        (*cctxPtr).tmpIn = (*cctxPtr).tmpIn.offset((*cctxPtr).tmpInSize as isize);
    }
    (*cctxPtr).tmpInSize = 0 as size_t;
    if (*cctxPtr).tmpIn.offset((*cctxPtr).maxBlockSize as isize)
        > (*cctxPtr).tmpBuff.offset((*cctxPtr).maxBufferSize as isize)
    {
        LZ4F_localSaveDict(cctxPtr as *mut LZ4F_cctx_t);
    }
    return dstPtr.offset_from(dstStart) as c_long as size_t;
}
#[inline]
pub unsafe fn LZ4F_compressEnd(
    mut cctxPtr: *mut LZ4F_cctx,
    mut dstBuffer: *mut c_void,
    mut dstCapacity: size_t,
    mut compressOptionsPtr: *const LZ4F_compressOptions_t,
) -> size_t {
    let dstStart: *mut BYTE = dstBuffer as *mut BYTE;
    let mut dstPtr: *mut BYTE = dstStart;
    let flushSize: size_t =
        LZ4F_flush(cctxPtr, dstBuffer, dstCapacity, compressOptionsPtr) as size_t;
    if LZ4F_isError(flushSize as LZ4F_errorCode_t) != 0 {
        return flushSize;
    }
    dstPtr = dstPtr.offset(flushSize as isize);
    dstCapacity = dstCapacity.wrapping_sub(flushSize);
    if dstCapacity < 4 as size_t {
        return LZ4F_returnErrorCode(LZ4F_ERROR_dstMaxSize_tooSmall) as size_t;
    }
    LZ4F_writeLE32(dstPtr as *mut c_void, 0 as U32);
    dstPtr = dstPtr.offset(4 as c_int as isize);
    if (*cctxPtr).prefs.frameInfo.contentChecksumFlag as c_uint
        == LZ4F_contentChecksumEnabled as c_int as c_uint
    {
        let xxh: U32 = XXH32_digest(&raw mut (*cctxPtr).xxh) as U32;
        if dstCapacity < 8 as size_t {
            return LZ4F_returnErrorCode(LZ4F_ERROR_dstMaxSize_tooSmall) as size_t;
        }
        LZ4F_writeLE32(dstPtr as *mut c_void, xxh);
        dstPtr = dstPtr.offset(4 as c_int as isize);
    }
    (*cctxPtr).cStage = 0 as U32;
    if (*cctxPtr).prefs.frameInfo.contentSize != 0 {
        if (*cctxPtr).prefs.frameInfo.contentSize
            != (*cctxPtr).totalInSize as c_ulonglong
        {
            return LZ4F_returnErrorCode(LZ4F_ERROR_frameSize_wrong) as size_t;
        }
    }
    return dstPtr.offset_from(dstStart) as c_long as size_t;
}
#[inline]
pub fn LZ4F_createDecompressionContext_advanced(
    mut customMem: LZ4F_CustomMem,
    mut version: c_uint,
) -> *mut LZ4F_dctx { unsafe {
    let dctx: *mut LZ4F_dctx =
        LZ4F_calloc(::core::mem::size_of::<LZ4F_dctx>() as size_t, customMem) as *mut LZ4F_dctx;
    if dctx.is_null() {
        return ::core::ptr::null_mut::<LZ4F_dctx>();
    }
    (*dctx).cmem = customMem;
    (*dctx).version = version as U32;
    return dctx;
} }
#[inline]
pub unsafe fn LZ4F_createDecompressionContext(
    mut LZ4F_decompressionContextPtr: *mut *mut LZ4F_dctx,
    mut versionNumber: c_uint,
) -> LZ4F_errorCode_t {
    if LZ4F_decompressionContextPtr.is_null() {
        return LZ4F_returnErrorCode(LZ4F_ERROR_parameter_null);
    }
    *LZ4F_decompressionContextPtr =
        LZ4F_createDecompressionContext_advanced(LZ4F_defaultCMem, versionNumber);
    if (*LZ4F_decompressionContextPtr).is_null() {
        return LZ4F_returnErrorCode(LZ4F_ERROR_allocation_failed);
    }
    return LZ4F_OK_NoError as c_int as LZ4F_errorCode_t;
}
#[inline]
pub unsafe fn LZ4F_freeDecompressionContext(
    mut dctx: *mut LZ4F_dctx,
) -> LZ4F_errorCode_t {
    let mut result: LZ4F_errorCode_t = LZ4F_OK_NoError as c_int as LZ4F_errorCode_t;
    if !dctx.is_null() {
        result = (*dctx).dStage as LZ4F_errorCode_t;
        LZ4F_free((*dctx).tmpIn as *mut c_void, (*dctx).cmem);
        LZ4F_free(
            (*dctx).tmpOutBuffer as *mut c_void,
            (*dctx).cmem,
        );
        LZ4F_free(dctx as *mut c_void, (*dctx).cmem);
    }
    return result;
}
#[inline]
pub unsafe fn LZ4F_dctx_size(mut dctx: *const LZ4F_dctx) -> size_t {
    if dctx.is_null() {
        return 0 as size_t;
    }
    return (::core::mem::size_of::<LZ4F_dctx>() as size_t)
        .wrapping_add(
            (if !(*dctx).tmpIn.is_null() {
                (*dctx).maxBlockSize.wrapping_add(BFSize)
            } else {
                0 as size_t
            }),
        )
        .wrapping_add(
            (if !(*dctx).tmpOutBuffer.is_null() {
                (*dctx).maxBufferSize
            } else {
                0 as size_t
            }),
        );
}
#[inline]
pub unsafe fn LZ4F_resetDecompressionContext(mut dctx: *mut LZ4F_dctx) {
    let dctx_view: &mut LZ4F_dctx = unsafe { &mut *dctx };
    dctx_view.dStage = dstage_getFrameHeader;
    dctx_view.dict = ::core::ptr::null::<BYTE>();
    dctx_view.dictSize = 0 as size_t;
    dctx_view.skipChecksum = 0 as c_int;
    dctx_view.frameRemainingSize = 0 as U64;
}
unsafe fn LZ4F_decodeHeader(
    mut dctx: *mut LZ4F_dctx,
    mut src: *const c_void,
    mut srcSize: size_t,
) -> size_t {
    let mut blockMode: c_uint = 0;
    let mut blockChecksumFlag: c_uint = 0;
    let mut contentSizeFlag: c_uint = 0;
    let mut contentChecksumFlag: c_uint = 0;
    let mut dictIDFlag: c_uint = 0;
    let mut blockSizeID: c_uint = 0;
    let mut frameHeaderSize: size_t = 0;
    let mut srcPtr: *const BYTE = src as *const BYTE;
    if srcSize < minFHSize {
        return LZ4F_returnErrorCode(LZ4F_ERROR_frameHeader_incomplete) as size_t;
    }
    memset(
        &raw mut (*dctx).frameInfo as *mut c_void,
        0 as c_int,
        ::core::mem::size_of::<LZ4F_frameInfo_t>() as size_t,
    );
    if LZ4F_readLE32(srcPtr as *const c_void) & 0xfffffff0 as U32
        == LZ4F_MAGIC_SKIPPABLE_START as U32
    {
        (*dctx).frameInfo.frameType = LZ4F_skippableFrame;
        if src
            == &raw mut (*dctx).header as *mut BYTE as *mut c_void
                as *const c_void
        {
            (*dctx).tmpInSize = srcSize;
            (*dctx).tmpInTarget = 8 as size_t;
            (*dctx).dStage = dstage_storeSFrameSize;
            return srcSize;
        } else {
            (*dctx).dStage = dstage_getSFrameSize;
            return 4 as size_t;
        }
    }
    if LZ4F_readLE32(srcPtr as *const c_void) != LZ4F_MAGICNUMBER as U32 {
        return LZ4F_returnErrorCode(LZ4F_ERROR_frameType_unknown) as size_t;
    }
    (*dctx).frameInfo.frameType = LZ4F_frame;
    let FLG: U32 = *srcPtr.offset(4 as c_int as isize) as U32;
    let version: U32 = FLG >> 6 as c_int & _2BITS as U32;
    blockChecksumFlag = (FLG >> 4 as c_int & _1BIT as U32) as c_uint;
    blockMode = (FLG >> 5 as c_int & _1BIT as U32) as c_uint;
    contentSizeFlag = (FLG >> 3 as c_int & _1BIT as U32) as c_uint;
    contentChecksumFlag = (FLG >> 2 as c_int & _1BIT as U32) as c_uint;
    dictIDFlag = (FLG & _1BIT as U32) as c_uint;
    if FLG >> 1 as c_int & _1BIT as U32 != 0 as U32 {
        return LZ4F_returnErrorCode(LZ4F_ERROR_reservedFlag_set) as size_t;
    }
    if version != 1 as U32 {
        return LZ4F_returnErrorCode(LZ4F_ERROR_headerVersion_wrong) as size_t;
    }
    frameHeaderSize = minFHSize
        .wrapping_add(
            (if contentSizeFlag != 0 {
                8 as c_int
            } else {
                0 as c_int
            }) as size_t,
        )
        .wrapping_add(
            (if dictIDFlag != 0 {
                4 as c_int
            } else {
                0 as c_int
            }) as size_t,
        );
    if srcSize < frameHeaderSize {
        if srcPtr != &raw mut (*dctx).header as *mut BYTE as *const BYTE {
            memcpy(
                &raw mut (*dctx).header as *mut BYTE as *mut c_void,
                srcPtr as *const c_void,
                srcSize,
            );
        }
        (*dctx).tmpInSize = srcSize;
        (*dctx).tmpInTarget = frameHeaderSize;
        (*dctx).dStage = dstage_storeFrameHeader;
        return srcSize;
    }
    let BD: U32 = *srcPtr.offset(5 as c_int as isize) as U32;
    blockSizeID = (BD >> 4 as c_int & _3BITS as U32) as c_uint;
    if BD >> 7 as c_int & _1BIT as U32 != 0 as U32 {
        return LZ4F_returnErrorCode(LZ4F_ERROR_reservedFlag_set) as size_t;
    }
    if blockSizeID < 4 as c_uint {
        return LZ4F_returnErrorCode(LZ4F_ERROR_maxBlockSize_invalid) as size_t;
    }
    if BD >> 0 as c_int & _4BITS as U32 != 0 as U32 {
        return LZ4F_returnErrorCode(LZ4F_ERROR_reservedFlag_set) as size_t;
    }
    let HC: BYTE = LZ4F_headerChecksum(
        srcPtr.offset(4 as c_int as isize) as *const c_void,
        frameHeaderSize.wrapping_sub(5 as size_t),
    ) as BYTE;
    if HC as c_int
        != *srcPtr.offset(frameHeaderSize.wrapping_sub(1 as size_t) as isize) as c_int
    {
        return LZ4F_returnErrorCode(LZ4F_ERROR_headerChecksum_invalid) as size_t;
    }
    (*dctx).frameInfo.blockMode = blockMode as LZ4F_blockMode_t;
    (*dctx).frameInfo.blockChecksumFlag = blockChecksumFlag as LZ4F_blockChecksum_t;
    (*dctx).frameInfo.contentChecksumFlag = contentChecksumFlag as LZ4F_contentChecksum_t;
    (*dctx).frameInfo.blockSizeID = blockSizeID as LZ4F_blockSizeID_t;
    (*dctx).maxBlockSize = LZ4F_getBlockSize(blockSizeID as LZ4F_blockSizeID_t);
    if contentSizeFlag != 0 {
        (*dctx).frameInfo.contentSize = LZ4F_readLE64(
            srcPtr.offset(6 as c_int as isize) as *const c_void,
        ) as c_ulonglong;
        (*dctx).frameRemainingSize = (*dctx).frameInfo.contentSize as U64;
    }
    if dictIDFlag != 0 {
        (*dctx).frameInfo.dictID = LZ4F_readLE32(
            srcPtr
                .offset(frameHeaderSize as isize)
                .offset(-(5 as c_int as isize))
                as *const c_void,
        ) as c_uint;
    }
    (*dctx).dStage = dstage_init;
    return frameHeaderSize;
}
#[inline]
pub unsafe fn LZ4F_headerSize(
    mut src: *const c_void,
    mut srcSize: size_t,
) -> size_t {
    if src.is_null() {
        return LZ4F_returnErrorCode(LZ4F_ERROR_srcPtr_wrong) as size_t;
    }
    if srcSize < LZ4F_MIN_SIZE_TO_KNOW_HEADER_LENGTH as size_t {
        return LZ4F_returnErrorCode(LZ4F_ERROR_frameHeader_incomplete) as size_t;
    }
    if LZ4F_readLE32(src) & 0xfffffff0 as U32 == LZ4F_MAGIC_SKIPPABLE_START as U32 {
        return 8 as size_t;
    }
    if LZ4F_readLE32(src) != LZ4F_MAGICNUMBER as U32 {
        return LZ4F_returnErrorCode(LZ4F_ERROR_frameType_unknown) as size_t;
    }
    let FLG: BYTE = *(src as *const BYTE).offset(4 as c_int as isize);
    let contentSizeFlag: U32 =
        (FLG as c_int >> 3 as c_int & _1BIT) as U32;
    let dictIDFlag: U32 = (FLG as c_int & _1BIT) as U32;
    return minFHSize
        .wrapping_add(
            (if contentSizeFlag != 0 {
                8 as c_int
            } else {
                0 as c_int
            }) as size_t,
        )
        .wrapping_add(
            (if dictIDFlag != 0 {
                4 as c_int
            } else {
                0 as c_int
            }) as size_t,
        );
}
#[inline]
pub unsafe fn LZ4F_getFrameInfo(
    mut dctx: *mut LZ4F_dctx,
    mut frameInfoPtr: *mut LZ4F_frameInfo_t,
    mut srcBuffer: *const c_void,
    mut srcSizePtr: *mut size_t,
) -> size_t {
    if frameInfoPtr.is_null() {
        return LZ4F_returnErrorCode(LZ4F_ERROR_parameter_null) as size_t;
    }
    if srcSizePtr.is_null() {
        return LZ4F_returnErrorCode(LZ4F_ERROR_parameter_null) as size_t;
    }
    if (*dctx).dStage as c_uint
        > dstage_storeFrameHeader as c_int as c_uint
    {
        let mut o: size_t = 0 as size_t;
        let mut i: size_t = 0 as size_t;
        *srcSizePtr = 0 as size_t;
        *frameInfoPtr = (*dctx).frameInfo;
        return LZ4F_decompress(
            dctx,
            NULL,
            &raw mut o,
            ::core::ptr::null::<c_void>(),
            &raw mut i,
            ::core::ptr::null::<LZ4F_decompressOptions_t>(),
        );
    } else if (*dctx).dStage as c_uint
        == dstage_storeFrameHeader as c_int as c_uint
    {
        *srcSizePtr = 0 as size_t;
        return LZ4F_returnErrorCode(LZ4F_ERROR_frameDecoding_alreadyStarted) as size_t;
    } else {
        let hSize: size_t = LZ4F_headerSize(srcBuffer, *srcSizePtr) as size_t;
        if LZ4F_isError(hSize as LZ4F_errorCode_t) != 0 {
            *srcSizePtr = 0 as size_t;
            return hSize;
        }
        if *srcSizePtr < hSize {
            *srcSizePtr = 0 as size_t;
            return LZ4F_returnErrorCode(LZ4F_ERROR_frameHeader_incomplete) as size_t;
        }
        let mut decodeResult: size_t = LZ4F_decodeHeader(dctx, srcBuffer, hSize);
        if LZ4F_isError(decodeResult as LZ4F_errorCode_t) != 0 {
            *srcSizePtr = 0 as size_t;
        } else {
            *srcSizePtr = decodeResult;
            decodeResult = BHSize;
        }
        *frameInfoPtr = (*dctx).frameInfo;
        return decodeResult;
    };
}
unsafe fn LZ4F_updateDict(
    mut dctx: *mut LZ4F_dctx,
    mut dstPtr: *const BYTE,
    mut dstSize: size_t,
    mut dstBufferStart: *const BYTE,
    mut withinTmp: c_uint,
) {
    let dctx_view: &mut LZ4F_dctx = unsafe { &mut *dctx };
    if dctx_view.dictSize == 0 as size_t {
        dctx_view.dict = dstPtr;
    }
    if dctx_view.dict.offset(dctx_view.dictSize as isize) == dstPtr {
        dctx_view.dictSize = dctx_view.dictSize.wrapping_add(dstSize);
        return;
    }
    if (dstPtr.offset_from(dstBufferStart) as c_long as size_t).wrapping_add(dstSize)
        >= (64 as c_int * ((1 as c_int) << 10 as c_int))
            as size_t
    {
        dctx_view.dict = dstBufferStart;
        dctx_view.dictSize = (dstPtr.offset_from(dstBufferStart) as c_long as size_t)
            .wrapping_add(dstSize);
        return;
    }
    if withinTmp != 0 && dctx_view.dict == dctx_view.tmpOutBuffer as *const BYTE {
        dctx_view.dictSize = dctx_view.dictSize.wrapping_add(dstSize);
        return;
    }
    if withinTmp != 0 {
        let preserveSize: size_t =
            dctx_view.tmpOut.offset_from(dctx_view.tmpOutBuffer) as c_long as size_t;
        let mut copySize: size_t = ((64 as c_int
            * ((1 as c_int) << 10 as c_int))
            as size_t)
            .wrapping_sub(dctx_view.tmpOutSize);
        let oldDictEnd: *const BYTE = dctx_view
            .dict
            .offset(dctx_view.dictSize as isize)
            .offset(-(dctx_view.tmpOutStart as isize));
        if dctx_view.tmpOutSize
            > (64 as c_int * ((1 as c_int) << 10 as c_int))
                as size_t
        {
            copySize = 0 as size_t;
        }
        if copySize > preserveSize {
            copySize = preserveSize;
        }
        memcpy(
            dctx_view
                .tmpOutBuffer
                .offset(preserveSize as isize)
                .offset(-(copySize as isize)) as *mut c_void,
            oldDictEnd.offset(-(copySize as isize)) as *const c_void,
            copySize,
        );
        dctx_view.dict = dctx_view.tmpOutBuffer;
        dctx_view.dictSize = preserveSize
            .wrapping_add(dctx_view.tmpOutStart)
            .wrapping_add(dstSize);
        return;
    }
    if dctx_view.dict == dctx_view.tmpOutBuffer as *const BYTE {
        if dctx_view.dictSize.wrapping_add(dstSize) > dctx_view.maxBufferSize {
            let preserveSize_0: size_t = ((64 as c_int
                * ((1 as c_int) << 10 as c_int))
                as size_t)
                .wrapping_sub(dstSize);
            memcpy(
                dctx_view.tmpOutBuffer as *mut c_void,
                dctx_view
                    .dict
                    .offset(dctx_view.dictSize as isize)
                    .offset(-(preserveSize_0 as isize))
                    as *const c_void,
                preserveSize_0,
            );
            dctx_view.dictSize = preserveSize_0;
        }
        memcpy(
            dctx_view.tmpOutBuffer.offset(dctx_view.dictSize as isize) as *mut c_void,
            dstPtr as *const c_void,
            dstSize,
        );
        dctx_view.dictSize = dctx_view.dictSize.wrapping_add(dstSize);
        return;
    }
    let mut preserveSize_1: size_t = ((64 as c_int
        * ((1 as c_int) << 10 as c_int))
        as size_t)
        .wrapping_sub(dstSize);
    if preserveSize_1 > dctx_view.dictSize {
        preserveSize_1 = dctx_view.dictSize;
    }
    memcpy(
        dctx_view.tmpOutBuffer as *mut c_void,
        dctx_view
            .dict
            .offset(dctx_view.dictSize as isize)
            .offset(-(preserveSize_1 as isize)) as *const c_void,
        preserveSize_1,
    );
    memcpy(
        dctx_view.tmpOutBuffer.offset(preserveSize_1 as isize) as *mut c_void,
        dstPtr as *const c_void,
        dstSize,
    );
    dctx_view.dict = dctx_view.tmpOutBuffer;
    dctx_view.dictSize = preserveSize_1.wrapping_add(dstSize);
}
pub unsafe fn LZ4F_decompress(
    mut dctx: *mut LZ4F_dctx,
    mut dstBuffer: *mut c_void,
    mut dstSizePtr: *mut size_t,
    mut srcBuffer: *const c_void,
    mut srcSizePtr: *mut size_t,
    mut decompressOptionsPtr: *const LZ4F_decompressOptions_t,
) -> size_t {
    let dstSizePtr_view: &mut size_t = unsafe { &mut *dstSizePtr };
    let mut optionsNull: LZ4F_decompressOptions_t = LZ4F_decompressOptions_t {
        stableDst: 0,
        skipChecksums: 0,
        reserved1: 0,
        reserved0: 0,
    };
    let srcStart: *const BYTE = srcBuffer as *const BYTE;
    let srcEnd: *const BYTE = srcStart.offset(*srcSizePtr as isize);
    let mut srcPtr: *const BYTE = srcStart;
    let dstStart: *mut BYTE = dstBuffer as *mut BYTE;
    let dstEnd: *mut BYTE = if !dstStart.is_null() {
        dstStart.offset(*dstSizePtr_view as isize)
    } else {
        ::core::ptr::null_mut::<BYTE>()
    };
    let mut dstPtr: *mut BYTE = dstStart;
    let mut selectedIn: *const BYTE = ::core::ptr::null::<BYTE>();
    let mut doAnotherStage: c_uint = 1 as c_uint;
    let mut nextSrcSizeHint: size_t = 1 as size_t;
    dstBuffer.is_null();
    memset(
        &raw mut optionsNull as *mut c_void,
        0 as c_int,
        ::core::mem::size_of::<LZ4F_decompressOptions_t>() as size_t,
    );
    if decompressOptionsPtr.is_null() {
        decompressOptionsPtr = &raw mut optionsNull;
    }
    *srcSizePtr = 0 as size_t;
    *dstSizePtr_view = 0 as size_t;
    (*dctx).skipChecksum |=
        ((*decompressOptionsPtr).skipChecksums != 0 as c_uint) as c_int;
    while doAnotherStage != 0 {
        let mut current_block_298: u64;
        match (*dctx).dStage as c_uint {
            0 => {
                if srcEnd.offset_from(srcPtr) as c_long as size_t >= maxFHSize {
                    let hSize: size_t = LZ4F_decodeHeader(
                        dctx,
                        srcPtr as *const c_void,
                        srcEnd.offset_from(srcPtr) as c_long as size_t,
                    ) as size_t;
                    if LZ4F_isError(hSize as LZ4F_errorCode_t) != 0 {
                        return hSize;
                    }
                    srcPtr = srcPtr.offset(hSize as isize);
                    current_block_298 = 7348614267943210136;
                } else {
                    (*dctx).tmpInSize = 0 as size_t;
                    if srcEnd.offset_from(srcPtr) as c_long == 0 as c_long
                    {
                        return minFHSize;
                    }
                    (*dctx).tmpInTarget = minFHSize;
                    (*dctx).dStage = dstage_storeFrameHeader;
                    current_block_298 = 16203760046146113240;
                }
            }
            1 => {
                current_block_298 = 16203760046146113240;
            }
            2 => {
                if (*dctx).frameInfo.contentChecksumFlag as u64 != 0 {
                    XXH32_reset(&raw mut (*dctx).xxh, 0 as c_uint);
                }
                let bufferNeeded: size_t = (*dctx).maxBlockSize.wrapping_add(
                    (if (*dctx).frameInfo.blockMode as c_uint
                        == LZ4F_blockLinked as c_int as c_uint
                    {
                        128 as c_int
                            * ((1 as c_int) << 10 as c_int)
                    } else {
                        0 as c_int
                    }) as size_t,
                );
                if bufferNeeded > (*dctx).maxBufferSize {
                    (*dctx).maxBufferSize = 0 as size_t;
                    LZ4F_free((*dctx).tmpIn as *mut c_void, (*dctx).cmem);
                    (*dctx).tmpIn =
                        LZ4F_malloc((*dctx).maxBlockSize.wrapping_add(BFSize), (*dctx).cmem)
                            as *mut BYTE;
                    if (*dctx).tmpIn.is_null() {
                        return LZ4F_returnErrorCode(LZ4F_ERROR_allocation_failed) as size_t;
                    }
                    LZ4F_free(
                        (*dctx).tmpOutBuffer as *mut c_void,
                        (*dctx).cmem,
                    );
                    (*dctx).tmpOutBuffer = LZ4F_malloc(bufferNeeded, (*dctx).cmem) as *mut BYTE;
                    if (*dctx).tmpOutBuffer.is_null() {
                        return LZ4F_returnErrorCode(LZ4F_ERROR_allocation_failed) as size_t;
                    }
                    (*dctx).maxBufferSize = bufferNeeded;
                }
                (*dctx).tmpInSize = 0 as size_t;
                (*dctx).tmpInTarget = 0 as size_t;
                (*dctx).tmpOut = (*dctx).tmpOutBuffer;
                (*dctx).tmpOutStart = 0 as size_t;
                (*dctx).tmpOutSize = 0 as size_t;
                (*dctx).dStage = dstage_getBlockHeader;
                current_block_298 = 11004992593402849422;
            }
            3 => {
                current_block_298 = 11004992593402849422;
            }
            4 => {
                current_block_298 = 317151059986244064;
            }
            5 => {
                let mut sizeToCopy_1: size_t = 0;
                if dstPtr.is_null() {
                    sizeToCopy_1 = 0 as size_t;
                } else {
                    let minBuffSize: size_t = if (srcEnd.offset_from(srcPtr) as c_long
                        as size_t)
                        < dstEnd.offset_from(dstPtr) as c_long as size_t
                    {
                        srcEnd.offset_from(srcPtr) as c_long as size_t
                    } else {
                        dstEnd.offset_from(dstPtr) as c_long as size_t
                    };
                    sizeToCopy_1 = if (*dctx).tmpInTarget < minBuffSize {
                        (*dctx).tmpInTarget
                    } else {
                        minBuffSize
                    };
                    memcpy(
                        dstPtr as *mut c_void,
                        srcPtr as *const c_void,
                        sizeToCopy_1,
                    );
                    if (*dctx).skipChecksum == 0 {
                        if (*dctx).frameInfo.blockChecksumFlag as u64 != 0 {
                            XXH32_update(
                                &raw mut (*dctx).blockChecksum,
                                srcPtr as *const c_void,
                                sizeToCopy_1,
                            );
                        }
                        if (*dctx).frameInfo.contentChecksumFlag as u64 != 0 {
                            XXH32_update(
                                &raw mut (*dctx).xxh,
                                srcPtr as *const c_void,
                                sizeToCopy_1,
                            );
                        }
                    }
                    if (*dctx).frameInfo.contentSize != 0 {
                        (*dctx).frameRemainingSize = ((*dctx).frameRemainingSize
                            as c_ulong)
                            .wrapping_sub(sizeToCopy_1 as c_ulong)
                            as U64 as U64;
                    }
                    if (*dctx).frameInfo.blockMode as c_uint
                        == LZ4F_blockLinked as c_int as c_uint
                    {
                        LZ4F_updateDict(
                            dctx,
                            dstPtr,
                            sizeToCopy_1,
                            dstStart,
                            0 as c_uint,
                        );
                    }
                    srcPtr = srcPtr.offset(sizeToCopy_1 as isize);
                    dstPtr = dstPtr.offset(sizeToCopy_1 as isize);
                }
                if sizeToCopy_1 == (*dctx).tmpInTarget {
                    if (*dctx).frameInfo.blockChecksumFlag as u64 != 0 {
                        (*dctx).tmpInSize = 0 as size_t;
                        (*dctx).dStage = dstage_getBlockChecksum;
                    } else {
                        (*dctx).dStage = dstage_getBlockHeader;
                    }
                } else {
                    (*dctx).tmpInTarget = (*dctx).tmpInTarget.wrapping_sub(sizeToCopy_1);
                    nextSrcSizeHint = (*dctx)
                        .tmpInTarget
                        .wrapping_add(
                            (if (*dctx).frameInfo.blockChecksumFlag as c_uint != 0 {
                                BFSize
                            } else {
                                0 as size_t
                            }),
                        )
                        .wrapping_add(BHSize);
                    doAnotherStage = 0 as c_uint;
                }
                current_block_298 = 7348614267943210136;
            }
            6 => {
                let mut crcSrc: *const c_void =
                    ::core::ptr::null::<c_void>();
                if srcEnd.offset_from(srcPtr) as c_long >= 4 as c_long
                    && (*dctx).tmpInSize == 0 as size_t
                {
                    crcSrc = srcPtr as *const c_void;
                    srcPtr = srcPtr.offset(4 as c_int as isize);
                    current_block_298 = 17336970397495664729;
                } else {
                    let stillToCopy: size_t = (4 as size_t).wrapping_sub((*dctx).tmpInSize);
                    let sizeToCopy_2: size_t = if stillToCopy
                        < srcEnd.offset_from(srcPtr) as c_long as size_t
                    {
                        stillToCopy
                    } else {
                        srcEnd.offset_from(srcPtr) as c_long as size_t
                    };
                    memcpy(
                        (&raw mut (*dctx).header as *mut BYTE).offset((*dctx).tmpInSize as isize)
                            as *mut c_void,
                        srcPtr as *const c_void,
                        sizeToCopy_2,
                    );
                    (*dctx).tmpInSize = (*dctx).tmpInSize.wrapping_add(sizeToCopy_2);
                    srcPtr = srcPtr.offset(sizeToCopy_2 as isize);
                    if (*dctx).tmpInSize < 4 as size_t {
                        doAnotherStage = 0 as c_uint;
                        current_block_298 = 7348614267943210136;
                    } else {
                        crcSrc = &raw mut (*dctx).header as *mut BYTE as *const c_void;
                        current_block_298 = 17336970397495664729;
                    }
                }
                match current_block_298 {
                    7348614267943210136 => {}
                    _ => {
                        if (*dctx).skipChecksum == 0 {
                            let readCRC: U32 = LZ4F_readLE32(crcSrc) as U32;
                            let calcCRC: U32 = XXH32_digest(&raw mut (*dctx).blockChecksum) as U32;
                            if readCRC != calcCRC {
                                return LZ4F_returnErrorCode(LZ4F_ERROR_blockChecksum_invalid)
                                    as size_t;
                            }
                        }
                        (*dctx).dStage = dstage_getBlockHeader;
                        current_block_298 = 7348614267943210136;
                    }
                }
            }
            7 => {
                if (srcEnd.offset_from(srcPtr) as c_long as size_t)
                    < (*dctx).tmpInTarget
                {
                    (*dctx).tmpInSize = 0 as size_t;
                    (*dctx).dStage = dstage_storeCBlock;
                    current_block_298 = 7348614267943210136;
                } else {
                    selectedIn = srcPtr;
                    srcPtr = srcPtr.offset((*dctx).tmpInTarget as isize);
                    current_block_298 = 5832582820025303349;
                }
            }
            8 => {
                let wantedData_0: size_t = (*dctx).tmpInTarget.wrapping_sub((*dctx).tmpInSize);
                let inputLeft: size_t = srcEnd.offset_from(srcPtr) as c_long as size_t;
                let sizeToCopy_3: size_t = if wantedData_0 < inputLeft {
                    wantedData_0
                } else {
                    inputLeft
                };
                memcpy(
                    (*dctx).tmpIn.offset((*dctx).tmpInSize as isize) as *mut c_void,
                    srcPtr as *const c_void,
                    sizeToCopy_3,
                );
                (*dctx).tmpInSize = (*dctx).tmpInSize.wrapping_add(sizeToCopy_3);
                srcPtr = srcPtr.offset(sizeToCopy_3 as isize);
                if (*dctx).tmpInSize < (*dctx).tmpInTarget {
                    nextSrcSizeHint = (*dctx)
                        .tmpInTarget
                        .wrapping_sub((*dctx).tmpInSize)
                        .wrapping_add(
                            (if (*dctx).frameInfo.blockChecksumFlag as c_uint != 0 {
                                BFSize
                            } else {
                                0 as size_t
                            }),
                        )
                        .wrapping_add(BHSize);
                    doAnotherStage = 0 as c_uint;
                    current_block_298 = 7348614267943210136;
                } else {
                    selectedIn = (*dctx).tmpIn;
                    current_block_298 = 5832582820025303349;
                }
            }
            9 => {
                current_block_298 = 12703207391024092602;
            }
            10 => {
                if (*dctx).frameRemainingSize != 0 {
                    return LZ4F_returnErrorCode(LZ4F_ERROR_frameSize_wrong) as size_t;
                }
                if (*dctx).frameInfo.contentChecksumFlag as u64 == 0 {
                    nextSrcSizeHint = 0 as size_t;
                    LZ4F_resetDecompressionContext(dctx);
                    doAnotherStage = 0 as c_uint;
                    current_block_298 = 7348614267943210136;
                } else {
                    if (srcEnd.offset_from(srcPtr) as c_long)
                        < 4 as c_long
                    {
                        (*dctx).tmpInSize = 0 as size_t;
                        (*dctx).dStage = dstage_storeSuffix;
                    } else {
                        selectedIn = srcPtr;
                        srcPtr = srcPtr.offset(4 as c_int as isize);
                    }
                    if (*dctx).dStage as c_uint
                        == dstage_storeSuffix as c_int as c_uint
                    {
                        current_block_298 = 14034612158273580970;
                    } else {
                        current_block_298 = 3531178331085578112;
                    }
                }
            }
            11 => {
                current_block_298 = 14034612158273580970;
            }
            12 => {
                if srcEnd.offset_from(srcPtr) as c_long >= 4 as c_long {
                    selectedIn = srcPtr;
                    srcPtr = srcPtr.offset(4 as c_int as isize);
                } else {
                    (*dctx).tmpInSize = 4 as size_t;
                    (*dctx).tmpInTarget = 8 as size_t;
                    (*dctx).dStage = dstage_storeSFrameSize;
                }
                if (*dctx).dStage as c_uint
                    == dstage_storeSFrameSize as c_int as c_uint
                {
                    current_block_298 = 9028266288740425872;
                } else {
                    current_block_298 = 18361011714114715771;
                }
            }
            13 => {
                current_block_298 = 9028266288740425872;
            }
            14 => {
                let skipSize: size_t = if (*dctx).tmpInTarget
                    < srcEnd.offset_from(srcPtr) as c_long as size_t
                {
                    (*dctx).tmpInTarget
                } else {
                    srcEnd.offset_from(srcPtr) as c_long as size_t
                };
                srcPtr = srcPtr.offset(skipSize as isize);
                (*dctx).tmpInTarget = (*dctx).tmpInTarget.wrapping_sub(skipSize);
                doAnotherStage = 0 as c_uint;
                nextSrcSizeHint = (*dctx).tmpInTarget;
                if nextSrcSizeHint != 0 {
                    current_block_298 = 7348614267943210136;
                } else {
                    LZ4F_resetDecompressionContext(dctx);
                    current_block_298 = 7348614267943210136;
                }
            }
            _ => {
                current_block_298 = 7348614267943210136;
            }
        }
        match current_block_298 {
            9028266288740425872 => {
                let sizeToCopy_6: size_t = if (*dctx).tmpInTarget.wrapping_sub((*dctx).tmpInSize)
                    < srcEnd.offset_from(srcPtr) as c_long as size_t
                {
                    (*dctx).tmpInTarget.wrapping_sub((*dctx).tmpInSize)
                } else {
                    srcEnd.offset_from(srcPtr) as c_long as size_t
                };
                memcpy(
                    (&raw mut (*dctx).header as *mut BYTE).offset((*dctx).tmpInSize as isize)
                        as *mut c_void,
                    srcPtr as *const c_void,
                    sizeToCopy_6,
                );
                srcPtr = srcPtr.offset(sizeToCopy_6 as isize);
                (*dctx).tmpInSize = (*dctx).tmpInSize.wrapping_add(sizeToCopy_6);
                if (*dctx).tmpInSize < (*dctx).tmpInTarget {
                    nextSrcSizeHint = (*dctx).tmpInTarget.wrapping_sub((*dctx).tmpInSize);
                    doAnotherStage = 0 as c_uint;
                    current_block_298 = 7348614267943210136;
                } else {
                    selectedIn = (&raw mut (*dctx).header as *mut BYTE)
                        .offset(4 as c_int as isize);
                    current_block_298 = 18361011714114715771;
                }
            }
            14034612158273580970 => {
                let remainingInput_0: size_t =
                    srcEnd.offset_from(srcPtr) as c_long as size_t;
                let wantedData_1: size_t = (4 as size_t).wrapping_sub((*dctx).tmpInSize);
                let sizeToCopy_5: size_t = if wantedData_1 < remainingInput_0 {
                    wantedData_1
                } else {
                    remainingInput_0
                };
                memcpy(
                    (*dctx).tmpIn.offset((*dctx).tmpInSize as isize) as *mut c_void,
                    srcPtr as *const c_void,
                    sizeToCopy_5,
                );
                srcPtr = srcPtr.offset(sizeToCopy_5 as isize);
                (*dctx).tmpInSize = (*dctx).tmpInSize.wrapping_add(sizeToCopy_5);
                if (*dctx).tmpInSize < 4 as size_t {
                    nextSrcSizeHint = (4 as size_t).wrapping_sub((*dctx).tmpInSize);
                    doAnotherStage = 0 as c_uint;
                    current_block_298 = 7348614267943210136;
                } else {
                    selectedIn = (*dctx).tmpIn;
                    current_block_298 = 3531178331085578112;
                }
            }
            5832582820025303349 => {
                if (*dctx).frameInfo.blockChecksumFlag as u64 != 0 {
                    (*dctx).tmpInTarget = (*dctx).tmpInTarget.wrapping_sub(4 as size_t);
                    let readBlockCrc: U32 =
                        LZ4F_readLE32(selectedIn.offset((*dctx).tmpInTarget as isize)
                            as *const c_void) as U32;
                    let calcBlockCrc: U32 = XXH32(
                        selectedIn as *const c_void,
                        (*dctx).tmpInTarget,
                        0 as c_uint,
                    ) as U32;
                    if readBlockCrc != calcBlockCrc {
                        return LZ4F_returnErrorCode(LZ4F_ERROR_blockChecksum_invalid) as size_t;
                    }
                }
                if dstEnd.offset_from(dstPtr) as c_long as size_t
                    >= (*dctx).maxBlockSize
                    && !(!(*dctx).dict.is_null()
                        && (*dctx).dict.offset((*dctx).dictSize as isize)
                            == (*dctx).tmpOut as *const BYTE)
                {
                    let mut dict: *const c_char =
                        (*dctx).dict as *const c_char;
                    let mut dictSize: size_t = (*dctx).dictSize;
                    let mut decodedSize: c_int = 0;
                    if !dict.is_null()
                        && dictSize
                            > (1 as c_int
                                * ((1 as c_int) << 30 as c_int))
                                as size_t
                    {
                        dict = dict.offset(dictSize.wrapping_sub(
                            (64 as c_int
                                * ((1 as c_int) << 10 as c_int))
                                as size_t,
                        ) as isize);
                        dictSize = (64 as c_int
                            * ((1 as c_int) << 10 as c_int))
                            as size_t;
                    }
                    decodedSize = LZ4_decompress_safe_usingDict(
                        selectedIn as *const c_char,
                        dstPtr as *mut c_char,
                        (*dctx).tmpInTarget as c_int,
                        (*dctx).maxBlockSize as c_int,
                        dict,
                        dictSize as c_int,
                    );
                    if decodedSize < 0 as c_int {
                        return LZ4F_returnErrorCode(LZ4F_ERROR_decompressionFailed) as size_t;
                    }
                    if (*dctx).frameInfo.contentChecksumFlag as c_uint != 0
                        && (*dctx).skipChecksum == 0
                    {
                        XXH32_update(
                            &raw mut (*dctx).xxh,
                            dstPtr as *const c_void,
                            decodedSize as size_t,
                        );
                    }
                    if (*dctx).frameInfo.contentSize != 0 {
                        (*dctx).frameRemainingSize = ((*dctx).frameRemainingSize
                            as c_ulong)
                            .wrapping_sub(decodedSize as size_t as c_ulong)
                            as U64 as U64;
                    }
                    if (*dctx).frameInfo.blockMode as c_uint
                        == LZ4F_blockLinked as c_int as c_uint
                    {
                        LZ4F_updateDict(
                            dctx,
                            dstPtr,
                            decodedSize as size_t,
                            dstStart,
                            0 as c_uint,
                        );
                    }
                    dstPtr = dstPtr.offset(decodedSize as isize);
                    (*dctx).dStage = dstage_getBlockHeader;
                    current_block_298 = 7348614267943210136;
                } else {
                    if (*dctx).frameInfo.blockMode as c_uint
                        == LZ4F_blockLinked as c_int as c_uint
                    {
                        if (*dctx).dict == (*dctx).tmpOutBuffer as *const BYTE {
                            if (*dctx).dictSize
                                > (128 as c_int
                                    * ((1 as c_int) << 10 as c_int))
                                    as size_t
                            {
                                memcpy(
                                    (*dctx).tmpOutBuffer as *mut c_void,
                                    (*dctx).dict.offset((*dctx).dictSize as isize).offset(
                                        -((64 as c_int
                                            * ((1 as c_int)
                                                << 10 as c_int))
                                            as isize),
                                    )
                                        as *const c_void,
                                    (64 as c_int
                                        * ((1 as c_int) << 10 as c_int))
                                        as size_t,
                                );
                                (*dctx).dictSize = (64 as c_int
                                    * ((1 as c_int) << 10 as c_int))
                                    as size_t;
                            }
                            (*dctx).tmpOut = (*dctx).tmpOutBuffer.offset((*dctx).dictSize as isize);
                        } else {
                            let reservedDictSpace: size_t = if (*dctx).dictSize
                                < (64 as c_int
                                    * ((1 as c_int) << 10 as c_int))
                                    as size_t
                            {
                                (*dctx).dictSize
                            } else {
                                (64 as c_int
                                    * ((1 as c_int) << 10 as c_int))
                                    as size_t
                            };
                            (*dctx).tmpOut =
                                (*dctx).tmpOutBuffer.offset(reservedDictSpace as isize);
                        }
                    }
                    let mut dict_0: *const c_char =
                        (*dctx).dict as *const c_char;
                    let mut dictSize_0: size_t = (*dctx).dictSize;
                    let mut decodedSize_0: c_int = 0;
                    if !dict_0.is_null()
                        && dictSize_0
                            > (1 as c_int
                                * ((1 as c_int) << 30 as c_int))
                                as size_t
                    {
                        dict_0 = dict_0.offset(dictSize_0.wrapping_sub(
                            (64 as c_int
                                * ((1 as c_int) << 10 as c_int))
                                as size_t,
                        ) as isize);
                        dictSize_0 = (64 as c_int
                            * ((1 as c_int) << 10 as c_int))
                            as size_t;
                    }
                    decodedSize_0 = LZ4_decompress_safe_usingDict(
                        selectedIn as *const c_char,
                        (*dctx).tmpOut as *mut c_char,
                        (*dctx).tmpInTarget as c_int,
                        (*dctx).maxBlockSize as c_int,
                        dict_0,
                        dictSize_0 as c_int,
                    );
                    if decodedSize_0 < 0 as c_int {
                        return LZ4F_returnErrorCode(LZ4F_ERROR_decompressionFailed) as size_t;
                    }
                    if (*dctx).frameInfo.contentChecksumFlag as c_uint != 0
                        && (*dctx).skipChecksum == 0
                    {
                        XXH32_update(
                            &raw mut (*dctx).xxh,
                            (*dctx).tmpOut as *const c_void,
                            decodedSize_0 as size_t,
                        );
                    }
                    if (*dctx).frameInfo.contentSize != 0 {
                        (*dctx).frameRemainingSize = ((*dctx).frameRemainingSize
                            as c_ulong)
                            .wrapping_sub(decodedSize_0 as size_t as c_ulong)
                            as U64 as U64;
                    }
                    (*dctx).tmpOutSize = decodedSize_0 as size_t;
                    (*dctx).tmpOutStart = 0 as size_t;
                    (*dctx).dStage = dstage_flushOut;
                    current_block_298 = 12703207391024092602;
                }
            }
            11004992593402849422 => {
                if srcEnd.offset_from(srcPtr) as c_long as size_t >= BHSize {
                    selectedIn = srcPtr;
                    srcPtr = srcPtr.offset(BHSize as isize);
                } else {
                    (*dctx).tmpInSize = 0 as size_t;
                    (*dctx).dStage = dstage_storeBlockHeader;
                }
                if (*dctx).dStage as c_uint
                    == dstage_storeBlockHeader as c_int as c_uint
                {
                    current_block_298 = 317151059986244064;
                } else {
                    current_block_298 = 7158658067966855297;
                }
            }
            16203760046146113240 => {
                let sizeToCopy: size_t = if (*dctx).tmpInTarget.wrapping_sub((*dctx).tmpInSize)
                    < srcEnd.offset_from(srcPtr) as c_long as size_t
                {
                    (*dctx).tmpInTarget.wrapping_sub((*dctx).tmpInSize)
                } else {
                    srcEnd.offset_from(srcPtr) as c_long as size_t
                };
                memcpy(
                    (&raw mut (*dctx).header as *mut BYTE).offset((*dctx).tmpInSize as isize)
                        as *mut c_void,
                    srcPtr as *const c_void,
                    sizeToCopy,
                );
                (*dctx).tmpInSize = (*dctx).tmpInSize.wrapping_add(sizeToCopy);
                srcPtr = srcPtr.offset(sizeToCopy as isize);
                if (*dctx).tmpInSize < (*dctx).tmpInTarget {
                    nextSrcSizeHint = (*dctx)
                        .tmpInTarget
                        .wrapping_sub((*dctx).tmpInSize)
                        .wrapping_add(BHSize);
                    doAnotherStage = 0 as c_uint;
                } else if LZ4F_isError(LZ4F_decodeHeader(
                    dctx,
                    &raw mut (*dctx).header as *mut BYTE as *const c_void,
                    (*dctx).tmpInTarget,
                ) as LZ4F_errorCode_t)
                    != 0
                {
                    return LZ4F_decodeHeader(
                        dctx,
                        &raw mut (*dctx).header as *mut BYTE as *const c_void,
                        (*dctx).tmpInTarget,
                    );
                }
                current_block_298 = 7348614267943210136;
            }
            _ => {}
        }
        match current_block_298 {
            18361011714114715771 => {
                let SFrameSize: size_t =
                    LZ4F_readLE32(selectedIn as *const c_void) as size_t;
                (*dctx).frameInfo.contentSize = SFrameSize as c_ulonglong;
                (*dctx).tmpInTarget = SFrameSize;
                (*dctx).dStage = dstage_skipSkippable;
                current_block_298 = 7348614267943210136;
            }
            3531178331085578112 => {
                if (*dctx).skipChecksum == 0 {
                    let readCRC_0: U32 =
                        LZ4F_readLE32(selectedIn as *const c_void) as U32;
                    let resultCRC: U32 = XXH32_digest(&raw mut (*dctx).xxh) as U32;
                    if readCRC_0 != resultCRC {
                        return LZ4F_returnErrorCode(LZ4F_ERROR_contentChecksum_invalid) as size_t;
                    }
                }
                nextSrcSizeHint = 0 as size_t;
                LZ4F_resetDecompressionContext(dctx);
                doAnotherStage = 0 as c_uint;
                current_block_298 = 7348614267943210136;
            }
            12703207391024092602 => {
                if !dstPtr.is_null() {
                    let sizeToCopy_4: size_t =
                        if (*dctx).tmpOutSize.wrapping_sub((*dctx).tmpOutStart)
                            < dstEnd.offset_from(dstPtr) as c_long as size_t
                        {
                            (*dctx).tmpOutSize.wrapping_sub((*dctx).tmpOutStart)
                        } else {
                            dstEnd.offset_from(dstPtr) as c_long as size_t
                        };
                    memcpy(
                        dstPtr as *mut c_void,
                        (*dctx).tmpOut.offset((*dctx).tmpOutStart as isize)
                            as *const c_void,
                        sizeToCopy_4,
                    );
                    if (*dctx).frameInfo.blockMode as c_uint
                        == LZ4F_blockLinked as c_int as c_uint
                    {
                        LZ4F_updateDict(
                            dctx,
                            dstPtr,
                            sizeToCopy_4,
                            dstStart,
                            1 as c_uint,
                        );
                    }
                    (*dctx).tmpOutStart = (*dctx).tmpOutStart.wrapping_add(sizeToCopy_4);
                    dstPtr = dstPtr.offset(sizeToCopy_4 as isize);
                }
                if (*dctx).tmpOutStart == (*dctx).tmpOutSize {
                    (*dctx).dStage = dstage_getBlockHeader;
                } else {
                    doAnotherStage = 0 as c_uint;
                    nextSrcSizeHint = BHSize;
                }
                current_block_298 = 7348614267943210136;
            }
            317151059986244064 => {
                let remainingInput: size_t =
                    srcEnd.offset_from(srcPtr) as c_long as size_t;
                let wantedData: size_t = BHSize.wrapping_sub((*dctx).tmpInSize);
                let sizeToCopy_0: size_t = if wantedData < remainingInput {
                    wantedData
                } else {
                    remainingInput
                };
                memcpy(
                    (*dctx).tmpIn.offset((*dctx).tmpInSize as isize) as *mut c_void,
                    srcPtr as *const c_void,
                    sizeToCopy_0,
                );
                srcPtr = srcPtr.offset(sizeToCopy_0 as isize);
                (*dctx).tmpInSize = (*dctx).tmpInSize.wrapping_add(sizeToCopy_0);
                if (*dctx).tmpInSize < BHSize {
                    nextSrcSizeHint = BHSize.wrapping_sub((*dctx).tmpInSize);
                    doAnotherStage = 0 as c_uint;
                    current_block_298 = 7348614267943210136;
                } else {
                    selectedIn = (*dctx).tmpIn;
                    current_block_298 = 7158658067966855297;
                }
            }
            _ => {}
        }
        match current_block_298 {
            7158658067966855297 => {
                let blockHeader: U32 =
                    LZ4F_readLE32(selectedIn as *const c_void) as U32;
                let nextCBlockSize: size_t = (blockHeader & 0x7fffffff as U32) as size_t;
                let crcSize: size_t =
                    ((*dctx).frameInfo.blockChecksumFlag as size_t).wrapping_mul(BFSize);
                if blockHeader == 0 as U32 {
                    (*dctx).dStage = dstage_getSuffix;
                } else {
                    if nextCBlockSize > (*dctx).maxBlockSize {
                        return LZ4F_returnErrorCode(LZ4F_ERROR_maxBlockSize_invalid) as size_t;
                    }
                    if blockHeader & LZ4F_BLOCKUNCOMPRESSED_FLAG as U32 != 0 {
                        (*dctx).tmpInTarget = nextCBlockSize;
                        if (*dctx).frameInfo.blockChecksumFlag as u64 != 0 {
                            XXH32_reset(&raw mut (*dctx).blockChecksum, 0 as c_uint);
                        }
                        (*dctx).dStage = dstage_copyDirect;
                    } else {
                        (*dctx).tmpInTarget = nextCBlockSize.wrapping_add(crcSize);
                        (*dctx).dStage = dstage_getCBlock;
                        if dstPtr == dstEnd || srcPtr == srcEnd {
                            nextSrcSizeHint =
                                BHSize.wrapping_add(nextCBlockSize).wrapping_add(crcSize);
                            doAnotherStage = 0 as c_uint;
                        }
                    }
                }
            }
            _ => {}
        }
    }
    if (*dctx).frameInfo.blockMode as c_uint
        == LZ4F_blockLinked as c_int as c_uint
        && (*dctx).dict != (*dctx).tmpOutBuffer as *const BYTE
        && !(*dctx).dict.is_null()
        && (*decompressOptionsPtr).stableDst == 0
        && ((*dctx).dStage as c_uint).wrapping_sub(2 as c_uint)
            < (dstage_getSuffix as c_int as c_uint)
                .wrapping_sub(2 as c_uint)
    {
        if (*dctx).dStage as c_uint
            == dstage_flushOut as c_int as c_uint
        {
            let preserveSize: size_t =
                (*dctx).tmpOut.offset_from((*dctx).tmpOutBuffer) as c_long as size_t;
            let mut copySize: size_t = ((64 as c_int
                * ((1 as c_int) << 10 as c_int))
                as size_t)
                .wrapping_sub((*dctx).tmpOutSize);
            let mut oldDictEnd: *const BYTE = (*dctx)
                .dict
                .offset((*dctx).dictSize as isize)
                .offset(-((*dctx).tmpOutStart as isize));
            if (*dctx).tmpOutSize
                > (64 as c_int
                    * ((1 as c_int) << 10 as c_int))
                    as size_t
            {
                copySize = 0 as size_t;
            }
            if copySize > preserveSize {
                copySize = preserveSize;
            }
            memcpy(
                (*dctx)
                    .tmpOutBuffer
                    .offset(preserveSize as isize)
                    .offset(-(copySize as isize)) as *mut c_void,
                oldDictEnd.offset(-(copySize as isize)) as *const c_void,
                copySize,
            );
            (*dctx).dict = (*dctx).tmpOutBuffer;
            (*dctx).dictSize = preserveSize.wrapping_add((*dctx).tmpOutStart);
        } else {
            let oldDictEnd_0: *const BYTE = (*dctx).dict.offset((*dctx).dictSize as isize);
            let newDictSize: size_t = if (*dctx).dictSize
                < (64 as c_int
                    * ((1 as c_int) << 10 as c_int))
                    as size_t
            {
                (*dctx).dictSize
            } else {
                (64 as c_int * ((1 as c_int) << 10 as c_int))
                    as size_t
            };
            memcpy(
                (*dctx).tmpOutBuffer as *mut c_void,
                oldDictEnd_0.offset(-(newDictSize as isize)) as *const c_void,
                newDictSize,
            );
            (*dctx).dict = (*dctx).tmpOutBuffer;
            (*dctx).dictSize = newDictSize;
            (*dctx).tmpOut = (*dctx).tmpOutBuffer.offset(newDictSize as isize);
        }
    }
    *srcSizePtr = srcPtr.offset_from(srcStart) as c_long as size_t;
    *dstSizePtr_view = dstPtr.offset_from(dstStart) as c_long as size_t;
    return nextSrcSizeHint;
}
#[inline]
pub unsafe fn LZ4F_decompress_usingDict(
    mut dctx: *mut LZ4F_dctx,
    mut dstBuffer: *mut c_void,
    mut dstSizePtr: *mut size_t,
    mut srcBuffer: *const c_void,
    mut srcSizePtr: *mut size_t,
    mut dict: *const c_void,
    mut dictSize: size_t,
    mut decompressOptionsPtr: *const LZ4F_decompressOptions_t,
) -> size_t {
    if (*dctx).dStage as c_uint
        <= dstage_init as c_int as c_uint
    {
        (*dctx).dict = dict as *const BYTE;
        (*dctx).dictSize = dictSize;
    }
    return LZ4F_decompress(
        dctx,
        dstBuffer,
        dstSizePtr,
        srcBuffer,
        srcSizePtr,
        decompressOptionsPtr,
    );
}
