use core::ffi::*;
pub use crate::src::c_inlined_fns::*;
pub use crate::src::ffi::*;
pub use crate::src::c_consts::*;
pub use crate::src::c_structs::*;
pub use crate::src::c_types::*;
use ::c2rust_bitfields;
use crate::src::c_inlined_fns::BrotliBitReaderGetAvailIn;
use crate::src::c_inlined_fns::BrotliBitReaderLoadBits;
use crate::src::c_inlined_fns::BrotliBitReaderSetInput;
use crate::src::dec::huffman::BrotliBuildCodeLengthsHuffmanTable;
use crate::src::dec::huffman::BrotliBuildHuffmanTable;
use crate::src::dec::huffman::BrotliBuildSimpleHuffmanTable;
use crate::src::dec::static_init::BrotliDecoderEnsureStaticInit;
use crate::src::c_inlined_fns::BrotliDropBits;
use crate::src::c_inlined_fns::BrotliGetAvailableBits;
use crate::src::c_inlined_fns::BrotliGetBitsUnmasked;
use crate::src::dec::bit_reader::BrotliSafeReadBits32Slow;
use crate::src::common::shared_dictionary::BrotliSharedDictionaryAttach;
use crate::src::common::transform::BrotliTransformDictionaryWord;
use crate::src::c_inlined_fns::BrotliUnalignedRead32;
use crate::src::c_inlined_fns::BrotliUnalignedRead64;
use crate::src::dec::bit_reader::BrotliWarmupBitReader;
extern "C" {
    static kBrotliBitMask: [uint64_t; 33];
    static kCmdLut: [CmdLutElement; 704];
    fn BrotliDecoderStateInit(
        s: *mut BrotliDecoderStateInternal,
        alloc_func: brotli_alloc_func,
        free_func: brotli_free_func,
        opaque: *mut c_void,
    ) -> c_int;
    fn BrotliDecoderStateCleanup(s: *mut BrotliDecoderStateInternal);
    fn BrotliDecoderStateMetablockBegin(s: *mut BrotliDecoderStateInternal);
    fn BrotliDecoderStateCleanupAfterMetablock(s: *mut BrotliDecoderStateInternal);
    fn BrotliDecoderHuffmanTreeGroupInit(
        s: *mut BrotliDecoderStateInternal,
        group: *mut HuffmanTreeGroup,
        alphabet_size_max: uint64_t,
        alphabet_size_limit: uint64_t,
        ntrees: uint64_t,
    ) -> c_int;
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct BrotliSharedDictionaryStruct {
    pub num_prefix: uint32_t,
    pub prefix_size: [size_t; 15],
    pub prefix: [*const uint8_t; 15],
    pub context_based: c_int,
    pub context_map: [uint8_t; 64],
    pub num_dictionaries: uint8_t,
    pub words: [*const BrotliDictionary; 64],
    pub transforms: [*const BrotliTransforms; 64],
    pub num_word_lists: uint8_t,
    pub words_instances: *mut BrotliDictionary,
    pub num_transform_lists: uint8_t,
    pub transforms_instances: *mut BrotliTransforms,
    pub prefix_suffix_maps: *mut uint16_t,
    pub alloc_func: brotli_alloc_func,
    pub free_func: brotli_free_func,
    pub memory_manager_opaque: *mut c_void,
}

pub type BrotliSharedDictionary = BrotliSharedDictionaryStruct;

#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct BrotliDecoderStateStruct {
    pub state: BrotliRunningState,
    pub loop_counter: c_int,
    pub br: BrotliBitReader,
    pub alloc_func: brotli_alloc_func,
    pub free_func: brotli_free_func,
    pub memory_manager_opaque: *mut c_void,
    pub buffer: C2RustUnnamed_hu0150709a,
    pub buffer_length: uint64_t,
    pub pos: c_int,
    pub max_backward_distance: c_int,
    pub max_distance: c_int,
    pub ringbuffer_size: c_int,
    pub ringbuffer_mask: c_int,
    pub dist_rb_idx: c_int,
    pub dist_rb: [c_int; 4],
    pub error_code: c_int,
    pub meta_block_remaining_len: c_int,
    pub ringbuffer: *mut uint8_t,
    pub ringbuffer_end: *mut uint8_t,
    pub htree_command: *mut HuffmanCode,
    pub context_lookup: *const uint8_t,
    pub context_map_slice: *mut uint8_t,
    pub dist_context_map_slice: *mut uint8_t,
    pub literal_hgroup: HuffmanTreeGroup,
    pub insert_copy_hgroup: HuffmanTreeGroup,
    pub distance_hgroup: HuffmanTreeGroup,
    pub block_type_trees: *mut HuffmanCode,
    pub block_len_trees: *mut HuffmanCode,
    pub trivial_literal_context: c_int,
    pub distance_context: c_int,
    pub block_length: [uint64_t; 3],
    pub block_length_index: uint64_t,
    pub num_block_types: [uint64_t; 3],
    pub block_type_rb: [uint64_t; 6],
    pub distance_postfix_bits: uint64_t,
    pub num_direct_distance_codes: uint64_t,
    pub num_dist_htrees: uint64_t,
    pub dist_context_map: *mut uint8_t,
    pub literal_htree: *mut HuffmanCode,
    pub rb_roundtrips: size_t,
    pub partial_pos_out: size_t,
    pub mtf_upper_bound: uint64_t,
    pub mtf: [uint32_t; 65],
    pub copy_length: c_int,
    pub distance_code: c_int,
    pub dist_htree_index: uint8_t,
    pub metadata_start_func: brotli_decoder_metadata_start_func,
    pub metadata_chunk_func: brotli_decoder_metadata_chunk_func,
    pub metadata_callback_opaque: *mut c_void,
    pub used_input: uint64_t,
    pub substate_metablock_header: BrotliRunningMetablockHeaderState,
    pub substate_uncompressed: BrotliRunningUncompressedState,
    pub substate_decode_uint8: BrotliRunningDecodeUint8State,
    pub substate_read_block_length: BrotliRunningReadBlockLengthState,
    pub new_ringbuffer_size: c_int,
    #[bitfield(name = "is_last_metablock", ty = "c_uint", bits = "0..=0")]
    #[bitfield(name = "is_uncompressed", ty = "c_uint", bits = "1..=1")]
    #[bitfield(name = "is_metadata", ty = "c_uint", bits = "2..=2")]
    #[bitfield(
        name = "should_wrap_ringbuffer",
        ty = "c_uint",
        bits = "3..=3"
    )]
    #[bitfield(
        name = "canny_ringbuffer_allocation",
        ty = "c_uint",
        bits = "4..=4"
    )]
    #[bitfield(name = "large_window", ty = "c_uint", bits = "5..=5")]
    #[bitfield(name = "window_bits", ty = "c_uint", bits = "6..=11")]
    #[bitfield(name = "size_nibbles", ty = "c_uint", bits = "12..=19")]
    pub is_last_metablock_is_uncompressed_is_metadata_should_wrap_ringbuffer_canny_ringbuffer_allocation_large_window_window_bits_size_nibbles:
        [u8; 3],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 1],
    pub num_literal_htrees: uint64_t,
    pub context_map: *mut uint8_t,
    pub context_modes: *mut uint8_t,
    pub dictionary: *mut BrotliSharedDictionaryInternal,
    pub compound_dictionary: *mut BrotliDecoderCompoundDictionary,
    pub trivial_literal_contexts: [uint32_t; 8],
    pub arena: C2RustUnnamed_hu94e3e3d6,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_hu94e3e3d6 {
    pub header: BrotliMetablockHeaderArena,
    pub body: BrotliMetablockBodyArena,
}

pub type BrotliSharedDictionaryInternal = BrotliSharedDictionaryStruct;

#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_hu0150709a {
    pub u64_0: uint64_t,
    pub u8_0: [uint8_t; 8],
}

pub type BrotliDecoderState = BrotliDecoderStateStruct;

pub type BrotliDecoderStateInternal = BrotliDecoderStateStruct;

pub const SHARED_BROTLI_MIN_DICTIONARY_WORD_LENGTH: c_int = 4 as c_int;
pub const SHARED_BROTLI_MAX_DICTIONARY_WORD_LENGTH: c_int = 31 as c_int;

pub const SHARED_BROTLI_MAX_RAW_DICT_SIZE: c_uint = (1 as c_uint)
    << (23 as usize).wrapping_add(::core::mem::size_of::<size_t>() as usize);

pub const BROTLI_TARGET_64_BITS: c_int = 1 as c_int;
pub const BROTLI_64_BITS: c_int = BROTLI_TARGET_64_BITS;

pub const BROTLI_REPEAT_PREVIOUS_CODE_LENGTH: c_int = 16 as c_int;

pub const BROTLI_LARGE_MIN_WBITS: c_int = 10 as c_int;
pub const BROTLI_LARGE_MAX_WBITS: c_int = 30 as c_int;

#[inline(always)]
extern "C" fn BrotliCalculateDistanceCodeLimit(
    mut max_distance: uint32_t,
    mut npostfix: uint32_t,
    mut ndirect: uint32_t,
) -> BrotliDistanceCodeLimit { {
    let mut result: BrotliDistanceCodeLimit = BrotliDistanceCodeLimit {
        max_alphabet_size: 0,
        max_distance: 0,
    };
    Some(
        BrotliCalculateDistanceCodeLimit
            as unsafe extern "C" fn(uint32_t, uint32_t, uint32_t) -> BrotliDistanceCodeLimit,
    );
    if max_distance <= ndirect {
        result.max_alphabet_size =
            max_distance.wrapping_add(BROTLI_NUM_DISTANCE_SHORT_CODES as uint32_t);
        result.max_distance = max_distance;
        return result;
    } else {
        let mut forbidden_distance: uint32_t = max_distance.wrapping_add(1 as uint32_t);
        let mut offset: uint32_t = forbidden_distance
            .wrapping_sub(ndirect)
            .wrapping_sub(1 as uint32_t);
        let mut ndistbits: uint32_t = 0 as uint32_t;
        let mut tmp: uint32_t = 0;
        let mut half: uint32_t = 0;
        let mut group: uint32_t = 0;
        let mut postfix: uint32_t = ((1 as uint32_t) << npostfix).wrapping_sub(1 as uint32_t);
        let mut extra: uint32_t = 0;
        let mut start: uint32_t = 0;
        offset = (offset >> npostfix).wrapping_add(4 as uint32_t);
        tmp = offset.wrapping_div(2 as uint32_t);
        while tmp != 0 as uint32_t {
            ndistbits = ndistbits.wrapping_add(1);
            tmp = tmp >> 1 as c_int;
        }
        ndistbits = ndistbits.wrapping_sub(1);
        half = offset >> ndistbits & 1 as uint32_t;
        group = ndistbits.wrapping_sub(1 as uint32_t) << 1 as c_int | half;
        if group == 0 as uint32_t {
            result.max_alphabet_size =
                ndirect.wrapping_add(BROTLI_NUM_DISTANCE_SHORT_CODES as uint32_t);
            result.max_distance = ndirect;
            return result;
        }
        group = group.wrapping_sub(1);
        ndistbits = (group >> 1 as c_int).wrapping_add(1 as uint32_t);
        extra = ((1 as c_uint) << ndistbits).wrapping_sub(1 as c_uint)
            as uint32_t;
        start = ((1 as c_uint) << ndistbits.wrapping_add(1 as uint32_t))
            .wrapping_sub(4 as c_uint) as uint32_t;
        start = (start as c_uint)
            .wrapping_add(((group & 1 as uint32_t) << ndistbits) as c_uint)
            as uint32_t as uint32_t;
        result.max_alphabet_size = (group << npostfix | postfix)
            .wrapping_add(ndirect)
            .wrapping_add(BROTLI_NUM_DISTANCE_SHORT_CODES as uint32_t)
            .wrapping_add(1 as uint32_t);
        result.max_distance = (start.wrapping_add(extra) << npostfix)
            .wrapping_add(postfix)
            .wrapping_add(ndirect)
            .wrapping_add(1 as uint32_t);
        return result;
    };
} }
pub const BROTLI_SHORT_FILL_BIT_WINDOW_READ: usize =
    ::core::mem::size_of::<uint64_t>() as usize >> 1 as c_int;

#[inline(always)]
extern "C" fn BitMask(mut n: uint64_t) -> uint64_t { unsafe {
    if 0 != 0 || 0 as c_int != 0 {
        return !(!(0 as c_int as uint64_t) << n);
    } else {
        return kBrotliBitMask[n as usize];
    };
} }

#[inline(always)]
unsafe extern "C" fn BrotliBitReaderSaveState(
    from: *mut BrotliBitReader,
    mut to: *mut BrotliBitReaderState,
) {
    (*to).val_ = (*from).val_;
    (*to).bit_pos_ = (*from).bit_pos_;
    (*to).next_in = (*from).next_in;
    (*to).avail_in = BrotliBitReaderGetAvailIn(from);
}

#[inline(always)]
unsafe extern "C" fn BrotliBitReaderRestoreState(
    to: *mut BrotliBitReader,
    mut from: *mut BrotliBitReaderState,
) {
    (*to).val_ = (*from).val_;
    (*to).bit_pos_ = (*from).bit_pos_;
    (*to).next_in = (*from).next_in;
    BrotliBitReaderSetInput(to, (*from).next_in, (*from).avail_in);
}

#[inline(always)]
unsafe fn BrotliGetRemainingBytes(mut br: *mut BrotliBitReader) -> size_t {
    static mut kCap: size_t = (1 as c_int as size_t) << BROTLI_LARGE_MAX_WBITS;
    let mut avail_in: size_t = BrotliBitReaderGetAvailIn(br);
    if avail_in > kCap {
        return kCap;
    }
    return avail_in.wrapping_add(BrotliGetAvailableBits(br) as size_t >> 3 as c_int);
}
#[inline(always)]
unsafe fn BrotliCheckInputAmount(br: *mut BrotliBitReader) -> c_int {
    let br_view: &BrotliBitReader = unsafe { &*br };
    return if br_view.next_in < br_view.guard_in {
        BROTLI_TRUE
    } else {
        BROTLI_FALSE
    };
}

#[inline(always)]
unsafe fn BrotliFillBitWindow(br: *mut BrotliBitReader, mut n_bits: uint64_t) {
    if 1 as c_int != 0 && 0 != 0 && n_bits <= 8 as uint64_t {
        let mut bit_pos: uint64_t = (*br).bit_pos_;
        if bit_pos <= 8 as uint64_t {
            (*br).val_ = BrotliBitReaderLoadBits(
                (*br).val_,
                BrotliUnalignedRead64((*br).next_in as *const c_void),
                56 as uint64_t,
                bit_pos,
            );
            (*br).bit_pos_ = bit_pos.wrapping_add(56 as uint64_t);
            (*br).next_in = (*br).next_in.offset(7 as c_int as isize);
        }
    } else if 1 as c_int != 0 && 0 != 0 && n_bits <= 16 as uint64_t {
        let mut bit_pos_0: uint64_t = (*br).bit_pos_;
        if bit_pos_0 <= 16 as uint64_t {
            (*br).val_ = BrotliBitReaderLoadBits(
                (*br).val_,
                BrotliUnalignedRead64((*br).next_in as *const c_void),
                48 as uint64_t,
                bit_pos_0,
            );
            (*br).bit_pos_ = bit_pos_0.wrapping_add(48 as uint64_t);
            (*br).next_in = (*br).next_in.offset(6 as c_int as isize);
        }
    } else {
        let mut bit_pos_1: uint64_t = (*br).bit_pos_;
        if bit_pos_1 <= 32 as uint64_t {
            (*br).val_ = BrotliBitReaderLoadBits(
                (*br).val_,
                BrotliUnalignedRead32((*br).next_in as *const c_void) as uint64_t,
                32 as uint64_t,
                bit_pos_1,
            );
            (*br).bit_pos_ = bit_pos_1.wrapping_add(32 as uint64_t);
            (*br).next_in = (*br)
                .next_in
                .offset(BROTLI_SHORT_FILL_BIT_WINDOW_READ as isize);
        }
    };
}
#[inline(always)]
unsafe fn BrotliFillBitWindow16(br: *mut BrotliBitReader) {
    BrotliFillBitWindow(br, 17 as uint64_t);
}
#[inline(always)]
unsafe extern "C" fn BrotliPullByte(br: *mut BrotliBitReader) -> c_int {
    if (*br).next_in == (*br).last_in {
        return BROTLI_FALSE;
    }
    (*br).val_ = BrotliBitReaderLoadBits(
        (*br).val_,
        *(*br).next_in as uint64_t,
        8 as uint64_t,
        (*br).bit_pos_,
    );
    (*br).bit_pos_ = ((*br).bit_pos_ as c_ulong)
        .wrapping_add(8 as c_ulong) as uint64_t as uint64_t;
    (*br).next_in = (*br).next_in.offset(1);
    return BROTLI_TRUE;
}

#[inline(always)]
unsafe fn BrotliGet16BitsUnmasked(br: *mut BrotliBitReader) -> uint64_t {
    BrotliFillBitWindow(br, 16 as uint64_t);
    return BrotliGetBitsUnmasked(br);
}
#[inline(always)]
unsafe fn BrotliGetBits(br: *mut BrotliBitReader, mut n_bits: uint64_t) -> uint64_t {
    BrotliFillBitWindow(br, n_bits);
    return BrotliGetBitsUnmasked(br) & BitMask(n_bits);
}
#[inline(always)]
unsafe fn BrotliSafeGetBits(
    br: *mut BrotliBitReader,
    mut n_bits: uint64_t,
    mut val: *mut uint64_t,
) -> c_int {
    while BrotliGetAvailableBits(br) < n_bits {
        if BrotliPullByte(br) == 0 {
            return BROTLI_FALSE;
        }
    }
    *val = BrotliGetBitsUnmasked(br) & BitMask(n_bits);
    return BROTLI_TRUE;
}

#[inline(always)]
unsafe fn BrotliBitReaderNormalize(mut br: *mut BrotliBitReader) {
    if (*br).bit_pos_ < (::core::mem::size_of::<uint64_t>() as uint64_t) << 3 as c_uint
    {
        (*br).val_ = ((*br).val_ as c_ulong
            & ((1 as c_int as uint64_t) << (*br).bit_pos_).wrapping_sub(1 as uint64_t)
                as c_ulong) as uint64_t;
    }
}
#[inline(always)]
unsafe fn BrotliBitReaderUnload(mut br: *mut BrotliBitReader) {
    let mut unused_bytes: uint64_t = BrotliGetAvailableBits(br) >> 3 as c_int;
    let mut unused_bits: uint64_t = unused_bytes << 3 as c_int;
    (*br).next_in = if unused_bytes == 0 as uint64_t {
        (*br).next_in
    } else {
        (*br).next_in.offset(-(unused_bytes as isize))
    };
    (*br).bit_pos_ = ((*br).bit_pos_ as c_ulong)
        .wrapping_sub(unused_bits as c_ulong) as uint64_t
        as uint64_t;
    BrotliBitReaderNormalize(br);
}
#[inline(always)]
unsafe extern "C" fn BrotliTakeBits(
    br: *mut BrotliBitReader,
    mut n_bits: uint64_t,
    mut val: *mut uint64_t,
) {
    *val = BrotliGetBitsUnmasked(br) & BitMask(n_bits);
    BrotliDropBits(br, n_bits);
}
#[inline(always)]
unsafe fn BrotliReadBits24(br: *mut BrotliBitReader, mut n_bits: uint64_t) -> uint64_t {
    if BROTLI_64_BITS != 0 || n_bits <= 16 as uint64_t {
        let mut val: uint64_t = 0;
        BrotliFillBitWindow(br, n_bits);
        BrotliTakeBits(br, n_bits, &raw mut val);
        return val;
    } else {
        let mut low_val: uint64_t = 0;
        let mut high_val: uint64_t = 0;
        BrotliFillBitWindow(br, 16 as uint64_t);
        BrotliTakeBits(br, 16 as uint64_t, &raw mut low_val);
        BrotliFillBitWindow(br, 8 as uint64_t);
        BrotliTakeBits(br, n_bits.wrapping_sub(16 as uint64_t), &raw mut high_val);
        return low_val | high_val << 16 as c_int;
    };
}
#[inline(always)]
unsafe fn BrotliReadBits32(br: *mut BrotliBitReader, mut n_bits: uint64_t) -> uint64_t {
    if BROTLI_64_BITS != 0 || n_bits <= 16 as uint64_t {
        let mut val: uint64_t = 0;
        BrotliFillBitWindow(br, n_bits);
        BrotliTakeBits(br, n_bits, &raw mut val);
        return val;
    } else {
        let mut low_val: uint64_t = 0;
        let mut high_val: uint64_t = 0;
        BrotliFillBitWindow(br, 16 as uint64_t);
        BrotliTakeBits(br, 16 as uint64_t, &raw mut low_val);
        BrotliFillBitWindow(br, 16 as uint64_t);
        BrotliTakeBits(br, n_bits.wrapping_sub(16 as uint64_t), &raw mut high_val);
        return low_val | high_val << 16 as c_int;
    };
}
#[inline(always)]
unsafe extern "C" fn BrotliSafeReadBits(
    br: *mut BrotliBitReader,
    mut n_bits: uint64_t,
    mut val: *mut uint64_t,
) -> c_int {
    while BrotliGetAvailableBits(br) < n_bits {
        if BrotliPullByte(br) == 0 {
            return BROTLI_FALSE;
        }
    }
    BrotliTakeBits(br, n_bits, val);
    return BROTLI_TRUE;
}
#[inline(always)]
unsafe fn BrotliSafeReadBits32(
    br: *mut BrotliBitReader,
    mut n_bits: uint64_t,
    mut val: *mut uint64_t,
) -> c_int {
    if BROTLI_64_BITS != 0 || n_bits <= 24 as uint64_t {
        while BrotliGetAvailableBits(br) < n_bits {
            if BrotliPullByte(br) == 0 {
                return BROTLI_FALSE;
            }
        }
        BrotliTakeBits(br, n_bits, val);
        return BROTLI_TRUE;
    } else {
        return BrotliSafeReadBits32Slow(br, n_bits, val);
    };
}
#[inline(always)]
unsafe fn BrotliJumpToByteBoundary(mut br: *mut BrotliBitReader) -> c_int {
    let mut pad_bits_count: uint64_t = BrotliGetAvailableBits(br) & 0x7 as uint64_t;
    let mut pad_bits: uint64_t = 0 as uint64_t;
    if pad_bits_count != 0 as uint64_t {
        BrotliTakeBits(br, pad_bits_count, &raw mut pad_bits);
    }
    BrotliBitReaderNormalize(br);
    return if pad_bits == 0 as uint64_t {
        BROTLI_TRUE
    } else {
        BROTLI_FALSE
    };
}
#[inline(always)]
unsafe fn BrotliDropBytes(mut br: *mut BrotliBitReader, mut num: size_t) {
    (*br).next_in = (*br).next_in.offset(num as isize);
}
#[inline(always)]
unsafe fn BrotliCopyBytes(
    mut dest: *mut uint8_t,
    mut br: *mut BrotliBitReader,
    mut num: size_t,
) {
    while BrotliGetAvailableBits(br) >= 8 as uint64_t && num > 0 as size_t {
        *dest = BrotliGetBitsUnmasked(br) as uint8_t;
        BrotliDropBits(br, 8 as uint64_t);
        dest = dest.offset(1);
        num = num.wrapping_sub(1);
    }
    BrotliBitReaderNormalize(br);
    if num > 0 as size_t {
        memcpy(
            dest as *mut c_void,
            (*br).next_in as *const c_void,
            num,
        );
        BrotliDropBytes(br, num);
    }
}

pub const BROTLI_HUFFMAN_MAX_SIZE_26: c_int = 396 as c_int;
pub const BROTLI_HUFFMAN_MAX_SIZE_258: c_int = 632 as c_int;

pub const HUFFMAN_TABLE_BITS: c_uint = 8 as c_uint;
pub const HUFFMAN_TABLE_MASK: c_int = 0xff as c_int;
static mut kRingBufferWriteAheadSlack: uint64_t = 542 as uint64_t;
static mut kCodeLengthCodeOrder: [uint8_t; 18] = [
    1 as c_int as uint8_t,
    2 as c_int as uint8_t,
    3 as c_int as uint8_t,
    4 as c_int as uint8_t,
    0 as c_int as uint8_t,
    5 as c_int as uint8_t,
    17 as c_int as uint8_t,
    6 as c_int as uint8_t,
    16 as c_int as uint8_t,
    7 as c_int as uint8_t,
    8 as c_int as uint8_t,
    9 as c_int as uint8_t,
    10 as c_int as uint8_t,
    11 as c_int as uint8_t,
    12 as c_int as uint8_t,
    13 as c_int as uint8_t,
    14 as c_int as uint8_t,
    15 as c_int as uint8_t,
];
static mut kCodeLengthPrefixLength: [uint8_t; 16] = [
    2 as c_int as uint8_t,
    2 as c_int as uint8_t,
    2 as c_int as uint8_t,
    3 as c_int as uint8_t,
    2 as c_int as uint8_t,
    2 as c_int as uint8_t,
    2 as c_int as uint8_t,
    4 as c_int as uint8_t,
    2 as c_int as uint8_t,
    2 as c_int as uint8_t,
    2 as c_int as uint8_t,
    3 as c_int as uint8_t,
    2 as c_int as uint8_t,
    2 as c_int as uint8_t,
    2 as c_int as uint8_t,
    4 as c_int as uint8_t,
];
static mut kCodeLengthPrefixValue: [uint8_t; 16] = [
    0 as c_int as uint8_t,
    4 as c_int as uint8_t,
    3 as c_int as uint8_t,
    2 as c_int as uint8_t,
    0 as c_int as uint8_t,
    4 as c_int as uint8_t,
    3 as c_int as uint8_t,
    1 as c_int as uint8_t,
    0 as c_int as uint8_t,
    4 as c_int as uint8_t,
    3 as c_int as uint8_t,
    2 as c_int as uint8_t,
    0 as c_int as uint8_t,
    4 as c_int as uint8_t,
    3 as c_int as uint8_t,
    5 as c_int as uint8_t,
];
#[inline]
pub unsafe fn BrotliDecoderSetParameter(
    mut state: *mut BrotliDecoderStateInternal,
    mut p: BrotliDecoderParameter,
    mut value: uint32_t,
) -> c_int {
    if (*state).state as c_uint
        != BROTLI_STATE_UNINITED as c_int as c_uint
    {
        return BROTLI_FALSE;
    }
    match p as c_uint {
        0 => {
            (*state).set_canny_ringbuffer_allocation(
                (if value != 0 {
                    0 as c_int
                } else {
                    1 as c_int
                }) as c_uint as c_uint,
            );
            return BROTLI_TRUE;
        }
        1 => {
            (*state).set_large_window(
                (if value != 0 {
                    BROTLI_TRUE
                } else {
                    BROTLI_FALSE
                }) as c_uint as c_uint,
            );
            return BROTLI_TRUE;
        }
        _ => return BROTLI_FALSE,
    };
}
#[inline]
pub unsafe fn BrotliDecoderCreateInstance(
    mut alloc_func: brotli_alloc_func,
    mut free_func: brotli_free_func,
    mut opaque: *mut c_void,
) -> *mut BrotliDecoderState {
    let mut state: *mut BrotliDecoderStateInternal =
        ::core::ptr::null_mut::<BrotliDecoderStateInternal>();
    if BrotliDecoderEnsureStaticInit() == 0 {
        return ::core::ptr::null_mut::<BrotliDecoderState>();
    }
    if alloc_func.is_none() && free_func.is_none() {
        state = malloc(::core::mem::size_of::<BrotliDecoderStateInternal>() as size_t)
            as *mut BrotliDecoderStateInternal;
    } else if alloc_func.is_some() && free_func.is_some() {
        state = alloc_func.expect("non-null function pointer")(
            opaque,
            ::core::mem::size_of::<BrotliDecoderStateInternal>() as size_t,
        ) as *mut BrotliDecoderStateInternal;
    }
    if state.is_null() {
        return ::core::ptr::null_mut::<BrotliDecoderState>();
    }
    if BrotliDecoderStateInit(state, alloc_func, free_func, opaque) == 0 {
        if alloc_func.is_none() && free_func.is_none() {
            free(state as *mut c_void);
        } else if alloc_func.is_some() && free_func.is_some() {
            free_func.expect("non-null function pointer")(
                opaque,
                state as *mut c_void,
            );
        }
        return ::core::ptr::null_mut::<BrotliDecoderState>();
    }
    return state as *mut BrotliDecoderState;
}
#[inline]
pub unsafe fn BrotliDecoderDestroyInstance(mut state: *mut BrotliDecoderStateInternal) {
    if state.is_null() {
        return;
    } else {
        let mut free_func: brotli_free_func = (*state).free_func;
        let mut opaque: *mut c_void = (*state).memory_manager_opaque;
        BrotliDecoderStateCleanup(state);
        free_func.expect("non-null function pointer")(opaque, state as *mut c_void);
    };
}
#[inline(never)]
unsafe fn SaveErrorCode(
    mut s: *mut BrotliDecoderStateInternal,
    mut e: BrotliDecoderErrorCode,
    mut consumed_input: size_t,
) -> BrotliDecoderResult {
    let s_view: &mut BrotliDecoderStateInternal = unsafe { &mut *s };
    s_view.error_code = e as c_int;
    s_view.used_input = (s_view.used_input as c_ulong)
        .wrapping_add(consumed_input as c_ulong) as uint64_t
        as uint64_t;
    if s_view.buffer_length != 0 as uint64_t && s_view.br.next_in == s_view.br.last_in {
        s_view.buffer_length = 0 as uint64_t;
    }
    match e as c_int {
        1 => return BROTLI_DECODER_RESULT_SUCCESS,
        2 => return BROTLI_DECODER_RESULT_NEEDS_MORE_INPUT,
        3 => return BROTLI_DECODER_RESULT_NEEDS_MORE_OUTPUT,
        _ => return BROTLI_DECODER_RESULT_ERROR,
    };
}
unsafe fn DecodeWindowBits(
    mut s: *mut BrotliDecoderStateInternal,
    mut br: *mut BrotliBitReader,
) -> BrotliDecoderErrorCode {
    let mut n: uint64_t = 0;
    let mut large_window: c_int = (*s).large_window() as c_int;
    (*s).set_large_window(BROTLI_FALSE as c_uint as c_uint);
    BrotliTakeBits(br, 1 as uint64_t, &raw mut n);
    if n == 0 as uint64_t {
        (*s).set_window_bits(16 as c_uint as c_uint);
        return BROTLI_DECODER_SUCCESS;
    }
    BrotliTakeBits(br, 3 as uint64_t, &raw mut n);
    if n != 0 as uint64_t {
        (*s).set_window_bits(
            ((17 as uint64_t).wrapping_add(n) & 63 as uint64_t) as c_uint
                as c_uint,
        );
        return BROTLI_DECODER_SUCCESS;
    }
    BrotliTakeBits(br, 3 as uint64_t, &raw mut n);
    if n == 1 as uint64_t {
        if large_window != 0 {
            BrotliTakeBits(br, 1 as uint64_t, &raw mut n);
            if n == 1 as uint64_t {
                return BROTLI_DECODER_ERROR_FORMAT_WINDOW_BITS as c_int
                    as BrotliDecoderErrorCode;
            }
            (*s).set_large_window(BROTLI_TRUE as c_uint as c_uint);
            return BROTLI_DECODER_SUCCESS;
        } else {
            return BROTLI_DECODER_ERROR_FORMAT_WINDOW_BITS as c_int
                as BrotliDecoderErrorCode;
        }
    }
    if n != 0 as uint64_t {
        (*s).set_window_bits(
            ((8 as uint64_t).wrapping_add(n) & 63 as uint64_t) as c_uint
                as c_uint,
        );
        return BROTLI_DECODER_SUCCESS;
    }
    (*s).set_window_bits(17 as c_uint as c_uint);
    return BROTLI_DECODER_SUCCESS;
}
#[inline(always)]
unsafe fn memmove16(mut dst: *mut uint8_t, mut src: *mut uint8_t) {
    let mut buffer: [uint32_t; 4] = [0; 4];
    memcpy(
        &raw mut buffer as *mut uint32_t as *mut c_void,
        src as *const c_void,
        16 as size_t,
    );
    memcpy(
        dst as *mut c_void,
        &raw mut buffer as *mut uint32_t as *const c_void,
        16 as size_t,
    );
}
#[inline(never)]
unsafe fn DecodeVarLenUint8(
    mut s: *mut BrotliDecoderStateInternal,
    mut br: *mut BrotliBitReader,
    mut value: *mut uint64_t,
) -> BrotliDecoderErrorCode {
    let mut bits: uint64_t = 0;
    let mut current_block_24: u64;
    match (*s).substate_decode_uint8 as c_uint {
        0 => {
            if (BrotliSafeReadBits(br, 1 as uint64_t, &raw mut bits) == 0) as c_int
                as c_long
                != 0
            {
                return BROTLI_DECODER_NEEDS_MORE_INPUT;
            }
            if bits == 0 as uint64_t {
                *value = 0 as uint64_t;
                return BROTLI_DECODER_SUCCESS;
            }
            current_block_24 = 7008795324829523344;
        }
        1 => {
            current_block_24 = 7008795324829523344;
        }
        2 => {
            current_block_24 = 9617098100945661058;
        }
        _ => {
            return BROTLI_DECODER_ERROR_UNREACHABLE as c_int
                as BrotliDecoderErrorCode;
        }
    }
    match current_block_24 {
        7008795324829523344 => {
            if (BrotliSafeReadBits(br, 3 as uint64_t, &raw mut bits) == 0) as c_int
                as c_long
                != 0
            {
                (*s).substate_decode_uint8 = BROTLI_STATE_DECODE_UINT8_SHORT;
                return BROTLI_DECODER_NEEDS_MORE_INPUT;
            }
            if bits == 0 as uint64_t {
                *value = 1 as uint64_t;
                (*s).substate_decode_uint8 = BROTLI_STATE_DECODE_UINT8_NONE;
                return BROTLI_DECODER_SUCCESS;
            }
            *value = bits;
        }
        _ => {}
    }
    if (BrotliSafeReadBits(br, *value, &raw mut bits) == 0) as c_int
        as c_long
        != 0
    {
        (*s).substate_decode_uint8 = BROTLI_STATE_DECODE_UINT8_LONG;
        return BROTLI_DECODER_NEEDS_MORE_INPUT;
    }
    *value = ((1 as c_uint as uint64_t) << *value).wrapping_add(bits);
    (*s).substate_decode_uint8 = BROTLI_STATE_DECODE_UINT8_NONE;
    return BROTLI_DECODER_SUCCESS;
}
#[inline(never)]
unsafe fn DecodeMetaBlockLength(
    mut s: *mut BrotliDecoderStateInternal,
    mut br: *mut BrotliBitReader,
) -> BrotliDecoderErrorCode {
    let mut bits: uint64_t = 0;
    let mut i: c_int = 0;
    loop {
        's_305: {
            let mut current_block_76: u64;
            match (*s).substate_metablock_header as c_uint {
                0 => {
                    if BrotliSafeReadBits(br, 1 as uint64_t, &raw mut bits) == 0 {
                        return BROTLI_DECODER_NEEDS_MORE_INPUT;
                    }
                    (*s).set_is_last_metablock(
                        (if bits != 0 {
                            1 as c_int
                        } else {
                            0 as c_int
                        }) as c_uint as c_uint,
                    );
                    (*s).meta_block_remaining_len = 0 as c_int;
                    (*s).set_is_uncompressed(0 as c_uint as c_uint);
                    (*s).set_is_metadata(0 as c_uint as c_uint);
                    if (*s).is_last_metablock() == 0 {
                        (*s).substate_metablock_header = BROTLI_STATE_METABLOCK_HEADER_NIBBLES;
                        current_block_76 = 17836213544692497527;
                    } else {
                        (*s).substate_metablock_header = BROTLI_STATE_METABLOCK_HEADER_EMPTY;
                        current_block_76 = 4252336785180995501;
                    }
                }
                1 => {
                    current_block_76 = 4252336785180995501;
                }
                2 => {
                    current_block_76 = 1923707297004211316;
                }
                3 => {
                    current_block_76 = 5546394164456084279;
                }
                4 => {
                    current_block_76 = 5105897894926936566;
                }
                5 => {
                    if BrotliSafeReadBits(br, 1 as uint64_t, &raw mut bits) == 0 {
                        return BROTLI_DECODER_NEEDS_MORE_INPUT;
                    }
                    if bits != 0 as uint64_t {
                        return BROTLI_DECODER_ERROR_FORMAT_RESERVED as c_int
                            as BrotliDecoderErrorCode;
                    }
                    (*s).substate_metablock_header = BROTLI_STATE_METABLOCK_HEADER_BYTES;
                    current_block_76 = 13217583647712742280;
                }
                6 => {
                    current_block_76 = 13217583647712742280;
                }
                7 => {
                    current_block_76 = 510015220139652934;
                }
                _ => {
                    return BROTLI_DECODER_ERROR_UNREACHABLE as c_int
                        as BrotliDecoderErrorCode;
                }
            }
            match current_block_76 {
                13217583647712742280 => {
                    if BrotliSafeReadBits(br, 2 as uint64_t, &raw mut bits) == 0 {
                        return BROTLI_DECODER_NEEDS_MORE_INPUT;
                    }
                    if bits == 0 as uint64_t {
                        (*s).substate_metablock_header = BROTLI_STATE_METABLOCK_HEADER_NONE;
                        return BROTLI_DECODER_SUCCESS;
                    }
                    (*s).set_size_nibbles(
                        bits as uint8_t as c_uint as c_uint,
                    );
                    (*s).substate_metablock_header = BROTLI_STATE_METABLOCK_HEADER_METADATA;
                    current_block_76 = 510015220139652934;
                }
                4252336785180995501 => {
                    if BrotliSafeReadBits(br, 1 as uint64_t, &raw mut bits) == 0 {
                        return BROTLI_DECODER_NEEDS_MORE_INPUT;
                    }
                    if bits != 0 {
                        (*s).substate_metablock_header = BROTLI_STATE_METABLOCK_HEADER_NONE;
                        return BROTLI_DECODER_SUCCESS;
                    }
                    (*s).substate_metablock_header = BROTLI_STATE_METABLOCK_HEADER_NIBBLES;
                    current_block_76 = 1923707297004211316;
                }
                _ => {}
            }
            match current_block_76 {
                510015220139652934 => {
                    i = (*s).loop_counter;
                    while i < (*s).size_nibbles() as c_int {
                        if BrotliSafeReadBits(br, 8 as uint64_t, &raw mut bits) == 0 {
                            (*s).loop_counter = i;
                            return BROTLI_DECODER_NEEDS_MORE_INPUT;
                        }
                        if i + 1 as c_int == (*s).size_nibbles() as c_int
                            && (*s).size_nibbles() as c_int > 1 as c_int
                            && bits == 0 as uint64_t
                        {
                            return BROTLI_DECODER_ERROR_FORMAT_EXUBERANT_META_NIBBLE
                                as c_int
                                as BrotliDecoderErrorCode;
                        }
                        (*s).meta_block_remaining_len |=
                            (bits << i * 8 as c_int) as c_int;
                        i += 1;
                    }
                    (*s).meta_block_remaining_len += 1;
                    (*s).substate_metablock_header = BROTLI_STATE_METABLOCK_HEADER_NONE;
                    return BROTLI_DECODER_SUCCESS;
                }
                1923707297004211316 => {
                    if BrotliSafeReadBits(br, 2 as uint64_t, &raw mut bits) == 0 {
                        return BROTLI_DECODER_NEEDS_MORE_INPUT;
                    }
                    (*s).set_size_nibbles(bits.wrapping_add(4 as uint64_t) as uint8_t
                        as c_uint
                        as c_uint);
                    (*s).loop_counter = 0 as c_int;
                    if bits == 3 as uint64_t {
                        (*s).set_is_metadata(1 as c_uint as c_uint);
                        (*s).substate_metablock_header = BROTLI_STATE_METABLOCK_HEADER_RESERVED;
                        current_block_76 = 17836213544692497527;
                    } else {
                        (*s).substate_metablock_header = BROTLI_STATE_METABLOCK_HEADER_SIZE;
                        current_block_76 = 5546394164456084279;
                    }
                }
                _ => {}
            }
            match current_block_76 {
                5546394164456084279 => {
                    i = (*s).loop_counter;
                    while i < (*s).size_nibbles() as c_int {
                        if BrotliSafeReadBits(br, 4 as uint64_t, &raw mut bits) == 0 {
                            (*s).loop_counter = i;
                            return BROTLI_DECODER_NEEDS_MORE_INPUT;
                        }
                        if i + 1 as c_int == (*s).size_nibbles() as c_int
                            && (*s).size_nibbles() as c_int > 4 as c_int
                            && bits == 0 as uint64_t
                        {
                            return BROTLI_DECODER_ERROR_FORMAT_EXUBERANT_NIBBLE
                                as c_int
                                as BrotliDecoderErrorCode;
                        }
                        (*s).meta_block_remaining_len |=
                            (bits << i * 4 as c_int) as c_int;
                        i += 1;
                    }
                    (*s).substate_metablock_header = BROTLI_STATE_METABLOCK_HEADER_UNCOMPRESSED;
                }
                17836213544692497527 => {
                    break 's_305;
                }
                _ => {}
            }
            if (*s).is_last_metablock() == 0 {
                if BrotliSafeReadBits(br, 1 as uint64_t, &raw mut bits) == 0 {
                    return BROTLI_DECODER_NEEDS_MORE_INPUT;
                }
                (*s).set_is_uncompressed(
                    (if bits != 0 {
                        1 as c_int
                    } else {
                        0 as c_int
                    }) as c_uint as c_uint,
                );
            }
            (*s).meta_block_remaining_len += 1;
            (*s).substate_metablock_header = BROTLI_STATE_METABLOCK_HEADER_NONE;
            return BROTLI_DECODER_SUCCESS;
        }
    }
}
#[inline(always)]
unsafe fn DecodeSymbol(
    mut bits: uint64_t,
    mut table: *const HuffmanCode,
    mut br: *mut BrotliBitReader,
) -> uint64_t {
    table = table.offset((bits & 0xff as uint64_t) as isize);
    if (*table).bits as c_uint > HUFFMAN_TABLE_BITS {
        let mut nbits: uint64_t =
            ((*table).bits as c_uint).wrapping_sub(HUFFMAN_TABLE_BITS) as uint64_t;
        BrotliDropBits(br, HUFFMAN_TABLE_BITS as uint64_t);
        table = table.offset(
            ((*table).value as uint64_t)
                .wrapping_add(bits >> 8 as c_uint & BitMask(nbits))
                as isize,
        );
    }
    BrotliDropBits(br, (*table).bits as uint64_t);
    return (*table).value as uint64_t;
}
#[inline(always)]
unsafe fn ReadSymbol(
    mut table: *const HuffmanCode,
    mut br: *mut BrotliBitReader,
) -> uint64_t {
    return DecodeSymbol(BrotliGet16BitsUnmasked(br), table, br);
}
#[inline(never)]
unsafe fn SafeDecodeSymbol(
    mut table: *const HuffmanCode,
    mut br: *mut BrotliBitReader,
    mut result: *mut uint64_t,
) -> c_int {
    let result_view: &mut uint64_t = unsafe { &mut *result };
    let mut val: uint64_t = 0;
    let mut available_bits: uint64_t = BrotliGetAvailableBits(br);
    if available_bits == 0 as uint64_t {
        if (*table).bits as c_int == 0 as c_int {
            *result_view = (*table).value as uint64_t;
            return BROTLI_TRUE;
        }
        return BROTLI_FALSE;
    }
    val = BrotliGetBitsUnmasked(br);
    table = table.offset((val & 0xff as uint64_t) as isize);
    if (*table).bits as c_uint <= HUFFMAN_TABLE_BITS {
        if (*table).bits as uint64_t <= available_bits {
            BrotliDropBits(br, (*table).bits as uint64_t);
            *result_view = (*table).value as uint64_t;
            return BROTLI_TRUE;
        } else {
            return BROTLI_FALSE;
        }
    }
    if available_bits <= HUFFMAN_TABLE_BITS as uint64_t {
        return BROTLI_FALSE;
    }
    val = (val & BitMask((*table).bits as uint64_t)) >> HUFFMAN_TABLE_BITS;
    available_bits = (available_bits as c_ulong)
        .wrapping_sub(HUFFMAN_TABLE_BITS as c_ulong) as uint64_t
        as uint64_t;
    table = table.offset(((*table).value as uint64_t).wrapping_add(val) as isize);
    if available_bits < (*table).bits as uint64_t {
        return BROTLI_FALSE;
    }
    BrotliDropBits(
        br,
        HUFFMAN_TABLE_BITS.wrapping_add((*table).bits as c_uint) as uint64_t,
    );
    *result_view = (*table).value as uint64_t;
    return BROTLI_TRUE;
}
#[inline(always)]
unsafe fn SafeReadSymbol(
    mut table: *const HuffmanCode,
    mut br: *mut BrotliBitReader,
    mut result: *mut uint64_t,
) -> c_int {
    let mut val: uint64_t = 0;
    if (BrotliSafeGetBits(br, 15 as uint64_t, &raw mut val) != 0) as c_int
        as c_long
        != 0
    {
        *result = DecodeSymbol(val, table, br);
        return BROTLI_TRUE;
    }
    return SafeDecodeSymbol(table, br, result);
}
#[inline(always)]
unsafe fn PreloadSymbol(
    mut safe: c_int,
    mut table: *const HuffmanCode,
    mut br: *mut BrotliBitReader,
    mut bits: *mut uint64_t,
    mut value: *mut uint64_t,
) {
    if safe != 0 {
        return;
    }
    table = table.offset(BrotliGetBits(br, 8 as uint64_t) as isize);
    *bits = (*table).bits as uint64_t;
    *value = (*table).value as uint64_t;
}
#[inline(always)]
unsafe fn ReadPreloadedSymbol(
    mut table: *const HuffmanCode,
    mut br: *mut BrotliBitReader,
    mut bits: *mut uint64_t,
    mut value: *mut uint64_t,
) -> uint64_t {
    let mut result: uint64_t = *value;
    if (*bits > 8 as uint64_t) as c_int as c_long != 0 {
        let mut val: uint64_t = BrotliGet16BitsUnmasked(br);
        let mut ext: *const HuffmanCode = table
            .offset((val & HUFFMAN_TABLE_MASK as uint64_t) as isize)
            .offset(*value as isize);
        let mut mask: uint64_t = BitMask((*bits).wrapping_sub(HUFFMAN_TABLE_BITS as uint64_t));
        BrotliDropBits(br, HUFFMAN_TABLE_BITS as uint64_t);
        ext = ext.offset((val >> 8 as c_uint & mask) as isize);
        BrotliDropBits(br, (*ext).bits as uint64_t);
        result = (*ext).value as uint64_t;
    } else {
        BrotliDropBits(br, *bits);
    }
    PreloadSymbol(0 as c_int, table, br, bits, value);
    return result;
}
#[inline(always)]
unsafe fn BrotliCopyPreloadedSymbolsToU8(
    mut table: *const HuffmanCode,
    mut br: *mut BrotliBitReader,
    mut bits: *mut uint64_t,
    mut value: *mut uint64_t,
    mut ringbuffer: *mut uint8_t,
    mut pos: c_int,
    limit: c_int,
) -> c_int {
    let kMaximalOverread: c_int = 4 as c_int;
    let mut pos_limit: c_int = limit;
    let mut copies: c_int = 0 as c_int;
    let mut new_lim: int64_t = (*br).guard_in.offset_from((*br).next_in) as int64_t;
    new_lim = (new_lim as c_long * 8 as c_long) as int64_t;
    new_lim = (new_lim as c_long / 15 as c_long) as int64_t;
    if new_lim - kMaximalOverread as int64_t <= limit as int64_t {
        pos_limit = (new_lim - kMaximalOverread as int64_t) as c_int;
    }
    if pos_limit < 0 as c_int {
        pos_limit = 0 as c_int;
    }
    copies = pos_limit;
    pos_limit += pos;
    while pos < pos_limit {
        *ringbuffer.offset(pos as isize) = ReadPreloadedSymbol(table, br, bits, value) as uint8_t;
        pos += 1;
    }
    while BrotliCheckInputAmount(br) != 0 && copies < limit {
        *ringbuffer.offset(pos as isize) = ReadPreloadedSymbol(table, br, bits, value) as uint8_t;
        pos += 1;
        copies += 1;
    }
    return copies;
}
#[inline(always)]
fn Log2Floor(mut x: uint64_t) -> uint64_t { {
    let mut result: uint64_t = 0 as uint64_t;
    while x != 0 {
        x >>= 1 as c_int;
        result = result.wrapping_add(1);
    }
    return result;
} }
unsafe fn ReadSimpleHuffmanSymbols(
    mut alphabet_size_max: uint64_t,
    mut alphabet_size_limit: uint64_t,
    mut s: *mut BrotliDecoderStateInternal,
) -> BrotliDecoderErrorCode {
    let s_view: &mut BrotliDecoderStateInternal = unsafe { &mut *s };
    let mut br: *mut BrotliBitReader = &raw mut s_view.br;
    let mut h: *mut BrotliMetablockHeaderArena = &raw mut s_view.arena.header;
    let mut max_bits: uint64_t = Log2Floor(alphabet_size_max.wrapping_sub(1 as uint64_t));
    let mut i: uint64_t = (*h).sub_loop_counter;
    let mut num_symbols: uint64_t = (*h).symbol;
    while i <= num_symbols {
        let mut v: uint64_t = 0;
        if (BrotliSafeReadBits(br, max_bits, &raw mut v) == 0) as c_int
            as c_long
            != 0
        {
            (*h).sub_loop_counter = i;
            (*h).substate_huffman = BROTLI_STATE_HUFFMAN_SIMPLE_READ;
            return BROTLI_DECODER_NEEDS_MORE_INPUT;
        }
        if v >= alphabet_size_limit {
            return BROTLI_DECODER_ERROR_FORMAT_SIMPLE_HUFFMAN_ALPHABET as c_int
                as BrotliDecoderErrorCode;
        }
        (*h).symbols_lists_array[i as usize] = v as uint16_t;
        i = i.wrapping_add(1);
    }
    i = 0 as uint64_t;
    while i < num_symbols {
        let mut k: uint64_t = i.wrapping_add(1 as uint64_t);
        while k <= num_symbols {
            if (*h).symbols_lists_array[i as usize] as c_int
                == (*h).symbols_lists_array[k as usize] as c_int
            {
                return BROTLI_DECODER_ERROR_FORMAT_SIMPLE_HUFFMAN_SAME as c_int
                    as BrotliDecoderErrorCode;
            }
            k = k.wrapping_add(1);
        }
        i = i.wrapping_add(1);
    }
    return BROTLI_DECODER_SUCCESS;
}
#[inline(always)]
unsafe fn ProcessSingleCodeLength(
    mut code_len: uint64_t,
    mut symbol: *mut uint64_t,
    mut repeat: *mut uint64_t,
    mut space: *mut uint64_t,
    mut prev_code_len: *mut uint64_t,
    mut symbol_lists: *mut uint16_t,
    mut code_length_histo: *mut uint16_t,
    mut next_symbol: *mut c_int,
) {
    *repeat = 0 as uint64_t;
    if code_len != 0 as uint64_t {
        *symbol_lists.offset(*next_symbol.offset(code_len as isize) as isize) = *symbol as uint16_t;
        *next_symbol.offset(code_len as isize) = *symbol as c_int;
        *prev_code_len = code_len;
        *space = (*space as c_ulong)
            .wrapping_sub((32768 as c_uint >> code_len) as c_ulong)
            as uint64_t as uint64_t;
        let ref mut fresh1 = *code_length_histo.offset(code_len as isize);
        *fresh1 = (*fresh1).wrapping_add(1);
    }
    *symbol = (*symbol).wrapping_add(1);
}
#[inline(always)]
unsafe fn ProcessRepeatedCodeLength(
    mut code_len: uint64_t,
    mut repeat_delta: uint64_t,
    mut alphabet_size: uint64_t,
    mut symbol: *mut uint64_t,
    mut repeat: *mut uint64_t,
    mut space: *mut uint64_t,
    mut prev_code_len: *mut uint64_t,
    mut repeat_code_len: *mut uint64_t,
    mut symbol_lists: *mut uint16_t,
    mut code_length_histo: *mut uint16_t,
    mut next_symbol: *mut c_int,
) {
    let mut old_repeat: uint64_t = 0;
    let mut extra_bits: uint64_t = 3 as uint64_t;
    let mut new_len: uint64_t = 0 as uint64_t;
    if code_len == BROTLI_REPEAT_PREVIOUS_CODE_LENGTH as uint64_t {
        new_len = *prev_code_len;
        extra_bits = 2 as uint64_t;
    }
    if *repeat_code_len != new_len {
        *repeat = 0 as uint64_t;
        *repeat_code_len = new_len;
    }
    old_repeat = *repeat;
    if *repeat > 0 as uint64_t {
        *repeat = (*repeat as c_ulong).wrapping_sub(2 as c_ulong)
            as uint64_t as uint64_t;
        *repeat <<= extra_bits;
    }
    *repeat = (*repeat as c_ulong)
        .wrapping_add(repeat_delta.wrapping_add(3 as uint64_t) as c_ulong)
        as uint64_t as uint64_t;
    repeat_delta = (*repeat).wrapping_sub(old_repeat);
    if (*symbol).wrapping_add(repeat_delta) > alphabet_size {
        *symbol = alphabet_size;
        *space = 0xfffff as uint64_t;
        return;
    }
    if *repeat_code_len != 0 as uint64_t {
        let mut last: uint64_t = (*symbol).wrapping_add(repeat_delta);
        let mut next: c_int = *next_symbol.offset(*repeat_code_len as isize);
        loop {
            *symbol_lists.offset(next as isize) = *symbol as uint16_t;
            next = *symbol as c_int;
            *symbol = (*symbol).wrapping_add(1);
            if !(*symbol != last) {
                break;
            }
        }
        *next_symbol.offset(*repeat_code_len as isize) = next;
        *space = (*space as c_ulong).wrapping_sub(
            (repeat_delta << (15 as uint64_t).wrapping_sub(*repeat_code_len))
                as c_ulong,
        ) as uint64_t as uint64_t;
        *code_length_histo.offset(*repeat_code_len as isize) =
            (*code_length_histo.offset(*repeat_code_len as isize) as uint64_t)
                .wrapping_add(repeat_delta) as uint16_t;
    } else {
        *symbol = (*symbol as c_ulong)
            .wrapping_add(repeat_delta as c_ulong) as uint64_t
            as uint64_t;
    };
}
unsafe fn ReadSymbolCodeLengths(
    mut alphabet_size: uint64_t,
    mut s: *mut BrotliDecoderStateInternal,
) -> BrotliDecoderErrorCode {
    let s_view: &mut BrotliDecoderStateInternal = unsafe { &mut *s };
    let mut br: *mut BrotliBitReader = &raw mut s_view.br;
    let mut h: *mut BrotliMetablockHeaderArena = &raw mut s_view.arena.header;
    let mut symbol: uint64_t = (*h).symbol;
    let mut repeat: uint64_t = (*h).repeat;
    let mut space: uint64_t = (*h).space;
    let mut prev_code_len: uint64_t = (*h).prev_code_len;
    let mut repeat_code_len: uint64_t = (*h).repeat_code_len;
    let mut symbol_lists: *mut uint16_t = (*h).symbol_lists;
    let mut code_length_histo: *mut uint16_t = &raw mut (*h).code_length_histo as *mut uint16_t;
    let mut next_symbol: *mut c_int =
        &raw mut (*h).next_symbol as *mut c_int;
    if BrotliWarmupBitReader(br) == 0 {
        return BROTLI_DECODER_NEEDS_MORE_INPUT;
    }
    while symbol < alphabet_size && space > 0 as uint64_t {
        let mut p: *const HuffmanCode = &raw mut (*h).table as *mut HuffmanCode;
        let mut code_len: uint64_t = 0;
        if BrotliCheckInputAmount(br) == 0 {
            (*h).symbol = symbol;
            (*h).repeat = repeat;
            (*h).prev_code_len = prev_code_len;
            (*h).repeat_code_len = repeat_code_len;
            (*h).space = space;
            return BROTLI_DECODER_NEEDS_MORE_INPUT;
        }
        BrotliFillBitWindow16(br);
        p = p.offset((BrotliGetBitsUnmasked(br) & BitMask(5 as uint64_t)) as isize);
        BrotliDropBits(br, (*p).bits as uint64_t);
        code_len = (*p).value as uint64_t;
        if code_len < BROTLI_REPEAT_PREVIOUS_CODE_LENGTH as uint64_t {
            ProcessSingleCodeLength(
                code_len,
                &raw mut symbol,
                &raw mut repeat,
                &raw mut space,
                &raw mut prev_code_len,
                symbol_lists,
                code_length_histo,
                next_symbol,
            );
        } else {
            let mut extra_bits: uint64_t =
                (if code_len == BROTLI_REPEAT_PREVIOUS_CODE_LENGTH as uint64_t {
                    2 as c_int
                } else {
                    3 as c_int
                }) as uint64_t;
            let mut repeat_delta: uint64_t = BrotliGetBitsUnmasked(br) & BitMask(extra_bits);
            BrotliDropBits(br, extra_bits);
            ProcessRepeatedCodeLength(
                code_len,
                repeat_delta,
                alphabet_size,
                &raw mut symbol,
                &raw mut repeat,
                &raw mut space,
                &raw mut prev_code_len,
                &raw mut repeat_code_len,
                symbol_lists,
                code_length_histo,
                next_symbol,
            );
        }
    }
    (*h).space = space;
    return BROTLI_DECODER_SUCCESS;
}
unsafe fn SafeReadSymbolCodeLengths(
    mut alphabet_size: uint64_t,
    mut s: *mut BrotliDecoderStateInternal,
) -> BrotliDecoderErrorCode {
    let s_view: &mut BrotliDecoderStateInternal = unsafe { &mut *s };
    let mut br: *mut BrotliBitReader = &raw mut s_view.br;
    let mut h: *mut BrotliMetablockHeaderArena = &raw mut s_view.arena.header;
    let mut get_byte: c_int = BROTLI_FALSE;
    while (*h).symbol < alphabet_size && (*h).space > 0 as uint64_t {
        let mut p: *const HuffmanCode = &raw mut (*h).table as *mut HuffmanCode;
        let mut code_len: uint64_t = 0;
        let mut available_bits: uint64_t = 0;
        let mut bits: uint64_t = 0 as uint64_t;
        if get_byte != 0 && BrotliPullByte(br) == 0 {
            return BROTLI_DECODER_NEEDS_MORE_INPUT;
        }
        get_byte = BROTLI_FALSE;
        available_bits = BrotliGetAvailableBits(br);
        if available_bits != 0 as uint64_t {
            bits = BrotliGetBitsUnmasked(br) as uint32_t as uint64_t;
        }
        p = p.offset((bits & BitMask(5 as uint64_t)) as isize);
        if (*p).bits as uint64_t > available_bits {
            get_byte = BROTLI_TRUE;
        } else {
            code_len = (*p).value as uint64_t;
            if code_len < BROTLI_REPEAT_PREVIOUS_CODE_LENGTH as uint64_t {
                BrotliDropBits(br, (*p).bits as uint64_t);
                ProcessSingleCodeLength(
                    code_len,
                    &raw mut (*h).symbol,
                    &raw mut (*h).repeat,
                    &raw mut (*h).space,
                    &raw mut (*h).prev_code_len,
                    (*h).symbol_lists,
                    &raw mut (*h).code_length_histo as *mut uint16_t,
                    &raw mut (*h).next_symbol as *mut c_int,
                );
            } else {
                let mut extra_bits: uint64_t = code_len.wrapping_sub(14 as uint64_t);
                let mut repeat_delta: uint64_t =
                    bits >> (*p).bits as c_int & BitMask(extra_bits);
                if available_bits < ((*p).bits as uint64_t).wrapping_add(extra_bits) {
                    get_byte = BROTLI_TRUE;
                } else {
                    BrotliDropBits(br, ((*p).bits as uint64_t).wrapping_add(extra_bits));
                    ProcessRepeatedCodeLength(
                        code_len,
                        repeat_delta,
                        alphabet_size,
                        &raw mut (*h).symbol,
                        &raw mut (*h).repeat,
                        &raw mut (*h).space,
                        &raw mut (*h).prev_code_len,
                        &raw mut (*h).repeat_code_len,
                        (*h).symbol_lists,
                        &raw mut (*h).code_length_histo as *mut uint16_t,
                        &raw mut (*h).next_symbol as *mut c_int,
                    );
                }
            }
        }
    }
    return BROTLI_DECODER_SUCCESS;
}
unsafe fn ReadCodeLengthCodeLengths(
    mut s: *mut BrotliDecoderStateInternal,
) -> BrotliDecoderErrorCode {
    let s_view: &mut BrotliDecoderStateInternal = unsafe { &mut *s };
    let mut br: *mut BrotliBitReader = &raw mut s_view.br;
    let mut h: *mut BrotliMetablockHeaderArena = &raw mut s_view.arena.header;
    let mut num_codes: uint64_t = (*h).repeat;
    let mut space: uint64_t = (*h).space;
    let mut i: uint64_t = (*h).sub_loop_counter;
    while i < BROTLI_CODE_LENGTH_CODES as uint64_t {
        let code_len_idx: uint8_t = kCodeLengthCodeOrder[i as usize];
        let mut ix: uint64_t = 0;
        let mut v: uint64_t = 0;
        if (BrotliSafeGetBits(br, 4 as uint64_t, &raw mut ix) == 0) as c_int
            as c_long
            != 0
        {
            let mut available_bits: uint64_t = BrotliGetAvailableBits(br);
            if available_bits != 0 as uint64_t {
                ix = BrotliGetBitsUnmasked(br) & 0xf as uint64_t;
            } else {
                ix = 0 as uint64_t;
            }
            if kCodeLengthPrefixLength[ix as usize] as uint64_t > available_bits {
                (*h).sub_loop_counter = i;
                (*h).repeat = num_codes;
                (*h).space = space;
                (*h).substate_huffman = BROTLI_STATE_HUFFMAN_COMPLEX;
                return BROTLI_DECODER_NEEDS_MORE_INPUT;
            }
        }
        v = kCodeLengthPrefixValue[ix as usize] as uint64_t;
        BrotliDropBits(br, kCodeLengthPrefixLength[ix as usize] as uint64_t);
        (*h).code_length_code_lengths[code_len_idx as usize] = v as uint8_t;
        if v != 0 as uint64_t {
            space = space.wrapping_sub((32 as c_uint >> v) as uint64_t);
            num_codes = num_codes.wrapping_add(1);
            (*h).code_length_histo[v as usize] = (*h).code_length_histo[v as usize].wrapping_add(1);
            if space.wrapping_sub(1 as uint64_t) >= 32 as uint64_t {
                break;
            }
        }
        i = i.wrapping_add(1);
    }
    if !(num_codes == 1 as uint64_t || space == 0 as uint64_t) {
        return BROTLI_DECODER_ERROR_FORMAT_CL_SPACE as c_int
            as BrotliDecoderErrorCode;
    }
    return BROTLI_DECODER_SUCCESS;
}
unsafe fn ReadHuffmanCode(
    mut alphabet_size_max: uint64_t,
    mut alphabet_size_limit: uint64_t,
    mut table: *mut HuffmanCode,
    mut opt_table_size: *mut uint64_t,
    mut s: *mut BrotliDecoderStateInternal,
) -> BrotliDecoderErrorCode {
    let mut br: *mut BrotliBitReader = &raw mut (*s).br;
    let mut h: *mut BrotliMetablockHeaderArena = &raw mut (*s).arena.header;
    let mut current_block_67: u64;
    loop {
        match (*h).substate_huffman as c_uint {
            0 => {
                if BrotliSafeReadBits(br, 2 as uint64_t, &raw mut (*h).sub_loop_counter) == 0 {
                    return BROTLI_DECODER_NEEDS_MORE_INPUT;
                }
                if (*h).sub_loop_counter != 1 as uint64_t {
                    (*h).space = 32 as uint64_t;
                    (*h).repeat = 0 as uint64_t;
                    memset(
                        (&raw mut (*h).code_length_histo as *mut uint16_t)
                            .offset(0 as c_int as isize)
                            as *mut uint16_t as *mut c_void,
                        0 as c_int,
                        (::core::mem::size_of::<uint16_t>() as size_t).wrapping_mul(
                            (BROTLI_HUFFMAN_MAX_CODE_LENGTH_CODE_LENGTH + 1 as c_int)
                                as size_t,
                        ),
                    );
                    memset(
                        (&raw mut (*h).code_length_code_lengths as *mut uint8_t)
                            .offset(0 as c_int as isize)
                            as *mut uint8_t as *mut c_void,
                        0 as c_int,
                        ::core::mem::size_of::<[uint8_t; 18]>() as size_t,
                    );
                    (*h).substate_huffman = BROTLI_STATE_HUFFMAN_COMPLEX;
                    continue;
                } else {
                    current_block_67 = 1993813529468123744;
                }
            }
            1 => {
                current_block_67 = 1993813529468123744;
            }
            2 => {
                current_block_67 = 4521361012956345576;
            }
            3 => {
                current_block_67 = 16397541726906695188;
            }
            4 => {
                let mut i: uint64_t = 0;
                let mut result_0: BrotliDecoderErrorCode = ReadCodeLengthCodeLengths(s);
                if result_0 as c_int != BROTLI_DECODER_SUCCESS as c_int {
                    return result_0;
                }
                BrotliBuildCodeLengthsHuffmanTable(
                    &raw mut (*h).table as *mut HuffmanCode,
                    &raw mut (*h).code_length_code_lengths as *mut uint8_t,
                    &raw mut (*h).code_length_histo as *mut uint16_t,
                );
                memset(
                    (&raw mut (*h).code_length_histo as *mut uint16_t)
                        .offset(0 as c_int as isize)
                        as *mut uint16_t as *mut c_void,
                    0 as c_int,
                    ::core::mem::size_of::<[uint16_t; 16]>() as size_t,
                );
                i = 0 as uint64_t;
                while i <= BROTLI_HUFFMAN_MAX_CODE_LENGTH as uint64_t {
                    (*h).next_symbol[i as usize] = i as c_int
                        - (BROTLI_HUFFMAN_MAX_CODE_LENGTH + 1 as c_int);
                    *(*h)
                        .symbol_lists
                        .offset((*h).next_symbol[i as usize] as isize) = 0xffff as uint16_t;
                    i = i.wrapping_add(1);
                }
                (*h).symbol = 0 as uint64_t;
                (*h).prev_code_len = BROTLI_INITIAL_REPEATED_CODE_LENGTH as uint64_t;
                (*h).repeat = 0 as uint64_t;
                (*h).repeat_code_len = 0 as uint64_t;
                (*h).space = 32768 as uint64_t;
                (*h).substate_huffman = BROTLI_STATE_HUFFMAN_LENGTH_SYMBOLS;
                current_block_67 = 9103093167384777346;
            }
            5 => {
                current_block_67 = 9103093167384777346;
            }
            _ => {
                return BROTLI_DECODER_ERROR_UNREACHABLE as c_int
                    as BrotliDecoderErrorCode;
            }
        }
        match current_block_67 {
            1993813529468123744 => {
                if BrotliSafeReadBits(br, 2 as uint64_t, &raw mut (*h).symbol) == 0 {
                    (*h).substate_huffman = BROTLI_STATE_HUFFMAN_SIMPLE_SIZE;
                    return BROTLI_DECODER_NEEDS_MORE_INPUT;
                }
                (*h).sub_loop_counter = 0 as uint64_t;
                current_block_67 = 4521361012956345576;
            }
            9103093167384777346 => {
                let mut table_size_0: uint64_t = 0;
                let mut result_1: BrotliDecoderErrorCode =
                    ReadSymbolCodeLengths(alphabet_size_limit, s);
                if result_1 as c_int
                    == BROTLI_DECODER_NEEDS_MORE_INPUT as c_int
                {
                    result_1 = SafeReadSymbolCodeLengths(alphabet_size_limit, s);
                }
                if result_1 as c_int != BROTLI_DECODER_SUCCESS as c_int {
                    return result_1;
                }
                if (*h).space != 0 as uint64_t {
                    return BROTLI_DECODER_ERROR_FORMAT_HUFFMAN_SPACE as c_int
                        as BrotliDecoderErrorCode;
                }
                table_size_0 = BrotliBuildHuffmanTable(
                    table,
                    HUFFMAN_TABLE_BITS as c_int,
                    (*h).symbol_lists,
                    &raw mut (*h).code_length_histo as *mut uint16_t,
                ) as uint64_t;
                if !opt_table_size.is_null() {
                    *opt_table_size = table_size_0;
                }
                (*h).substate_huffman = BROTLI_STATE_HUFFMAN_NONE;
                return BROTLI_DECODER_SUCCESS;
            }
            _ => {}
        }
        match current_block_67 {
            4521361012956345576 => {
                let mut result: BrotliDecoderErrorCode =
                    ReadSimpleHuffmanSymbols(alphabet_size_max, alphabet_size_limit, s);
                if result as c_int != BROTLI_DECODER_SUCCESS as c_int {
                    return result;
                }
            }
            _ => {}
        }
        let mut table_size: uint64_t = 0;
        if (*h).symbol == 3 as uint64_t {
            let mut bits: uint64_t = 0;
            if BrotliSafeReadBits(br, 1 as uint64_t, &raw mut bits) == 0 {
                (*h).substate_huffman = BROTLI_STATE_HUFFMAN_SIMPLE_BUILD;
                return BROTLI_DECODER_NEEDS_MORE_INPUT;
            }
            (*h).symbol = ((*h).symbol as c_ulong)
                .wrapping_add(bits as c_ulong) as uint64_t
                as uint64_t;
        }
        table_size = BrotliBuildSimpleHuffmanTable(
            table,
            HUFFMAN_TABLE_BITS as c_int,
            &raw mut (*h).symbols_lists_array as *mut uint16_t,
            (*h).symbol as uint32_t,
        ) as uint64_t;
        if !opt_table_size.is_null() {
            *opt_table_size = table_size;
        }
        (*h).substate_huffman = BROTLI_STATE_HUFFMAN_NONE;
        return BROTLI_DECODER_SUCCESS;
    }
}
#[inline(always)]
unsafe fn ReadBlockLength(
    mut table: *const HuffmanCode,
    mut br: *mut BrotliBitReader,
) -> uint64_t {
    let mut code: uint64_t = 0;
    let mut nbits: uint64_t = 0;
    code = ReadSymbol(table, br);
    nbits = _kBrotliPrefixCodeRanges[code as usize].nbits as uint64_t;
    return (_kBrotliPrefixCodeRanges[code as usize].offset as uint64_t)
        .wrapping_add(BrotliReadBits24(br, nbits));
}
#[inline(always)]
unsafe fn SafeReadBlockLength(
    mut s: *mut BrotliDecoderStateInternal,
    mut result: *mut uint64_t,
    mut table: *const HuffmanCode,
    mut br: *mut BrotliBitReader,
) -> c_int {
    let mut index: uint64_t = 0;
    if (*s).substate_read_block_length as c_uint
        == BROTLI_STATE_READ_BLOCK_LENGTH_NONE as c_int as c_uint
    {
        if SafeReadSymbol(table, br, &raw mut index) == 0 {
            return BROTLI_FALSE;
        }
    } else {
        index = (*s).block_length_index;
    }
    let mut bits: uint64_t = 0;
    let mut nbits: uint64_t = _kBrotliPrefixCodeRanges[index as usize].nbits as uint64_t;
    let mut offset: uint64_t = _kBrotliPrefixCodeRanges[index as usize].offset as uint64_t;
    if BrotliSafeReadBits(br, nbits, &raw mut bits) == 0 {
        (*s).block_length_index = index;
        (*s).substate_read_block_length = BROTLI_STATE_READ_BLOCK_LENGTH_SUFFIX;
        return BROTLI_FALSE;
    }
    *result = offset.wrapping_add(bits);
    (*s).substate_read_block_length = BROTLI_STATE_READ_BLOCK_LENGTH_NONE;
    return BROTLI_TRUE;
}
#[inline(never)]
unsafe fn InverseMoveToFrontTransform(
    mut v: *mut uint8_t,
    mut v_len: uint64_t,
    mut state: *mut BrotliDecoderStateInternal,
) {
    let mut i: uint64_t = 1 as uint64_t;
    let mut upper_bound: uint64_t = (*state).mtf_upper_bound;
    let mut mtf: *mut uint32_t = (&raw mut (*state).mtf as *mut uint32_t)
        .offset(1 as c_int as isize) as *mut uint32_t;
    let mut mtf_u8: *mut uint8_t = mtf as *mut uint8_t;
    let b0123: [uint8_t; 4] = [
        0 as c_int as uint8_t,
        1 as c_int as uint8_t,
        2 as c_int as uint8_t,
        3 as c_int as uint8_t,
    ];
    let mut pattern: uint32_t = 0;
    memcpy(
        &raw mut pattern as *mut c_void,
        &raw const b0123 as *const c_void,
        4 as size_t,
    );
    *mtf.offset(0 as c_int as isize) = pattern;
    loop {
        pattern = (pattern as c_uint)
            .wrapping_add(0x4040404 as c_int as c_uint)
            as uint32_t as uint32_t;
        *mtf.offset(i as isize) = pattern;
        i = i.wrapping_add(1);
        if !(i <= upper_bound) {
            break;
        }
    }
    upper_bound = 0 as uint64_t;
    i = 0 as uint64_t;
    while i < v_len {
        let mut index: c_int = *v.offset(i as isize) as c_int;
        let mut value: uint8_t = *mtf_u8.offset(index as isize);
        upper_bound = (upper_bound as c_ulong
            | *v.offset(i as isize) as c_ulong) as uint64_t;
        *v.offset(i as isize) = value;
        *mtf_u8.offset(-(1 as c_int) as isize) = value;
        loop {
            index -= 1;
            *mtf_u8.offset((index + 1 as c_int) as isize) =
                *mtf_u8.offset(index as isize);
            if !(index >= 0 as c_int) {
                break;
            }
        }
        i = i.wrapping_add(1);
    }
    (*state).mtf_upper_bound = upper_bound >> 2 as c_int;
}
unsafe fn HuffmanTreeGroupDecode(
    mut group: *mut HuffmanTreeGroup,
    mut s: *mut BrotliDecoderStateInternal,
) -> BrotliDecoderErrorCode {
    let mut h: *mut BrotliMetablockHeaderArena = &raw mut (*s).arena.header;
    if (*h).substate_tree_group as c_uint
        != BROTLI_STATE_TREE_GROUP_LOOP as c_int as c_uint
    {
        (*h).next = (*group).codes;
        (*h).htree_index = 0 as c_int;
        (*h).substate_tree_group = BROTLI_STATE_TREE_GROUP_LOOP;
    }
    while (*h).htree_index < (*group).num_htrees as c_int {
        let mut table_size: uint64_t = 0;
        let mut result: BrotliDecoderErrorCode = ReadHuffmanCode(
            (*group).alphabet_size_max as uint64_t,
            (*group).alphabet_size_limit as uint64_t,
            (*h).next,
            &raw mut table_size,
            s,
        );
        if result as c_int != BROTLI_DECODER_SUCCESS as c_int {
            return result;
        }
        let ref mut fresh0 = *(*group).htrees.offset((*h).htree_index as isize);
        *fresh0 = (*h).next;
        (*h).next = (*h).next.offset(table_size as isize);
        (*h).htree_index += 1;
    }
    (*h).substate_tree_group = BROTLI_STATE_TREE_GROUP_NONE;
    return BROTLI_DECODER_SUCCESS;
}
unsafe fn DecodeContextMap(
    mut context_map_size: uint64_t,
    mut num_htrees: *mut uint64_t,
    mut context_map_arg: *mut *mut uint8_t,
    mut s: *mut BrotliDecoderStateInternal,
) -> BrotliDecoderErrorCode {
    let mut br: *mut BrotliBitReader = &raw mut (*s).br;
    let mut result: BrotliDecoderErrorCode = BROTLI_DECODER_SUCCESS;
    let mut h: *mut BrotliMetablockHeaderArena = &raw mut (*s).arena.header;
    let mut current_block_72: u64;
    match (*h).substate_context_map as c_int {
        0 => {
            result = DecodeVarLenUint8(s, br, num_htrees);
            if result as c_int != BROTLI_DECODER_SUCCESS as c_int {
                return result;
            }
            *num_htrees = (*num_htrees).wrapping_add(1);
            (*h).context_index = 0 as uint64_t;
            *context_map_arg = (*s).alloc_func.expect("non-null function pointer")(
                (*s).memory_manager_opaque,
                context_map_size as size_t,
            ) as *mut uint8_t;
            if (*context_map_arg).is_null() {
                return BROTLI_DECODER_ERROR_ALLOC_CONTEXT_MAP as c_int
                    as BrotliDecoderErrorCode;
            }
            if *num_htrees <= 1 as uint64_t {
                memset(
                    *context_map_arg as *mut c_void,
                    0 as c_int,
                    context_map_size as size_t,
                );
                return BROTLI_DECODER_SUCCESS;
            }
            (*h).substate_context_map = BROTLI_STATE_CONTEXT_MAP_READ_PREFIX;
            current_block_72 = 10468293953885100227;
        }
        1 => {
            current_block_72 = 10468293953885100227;
        }
        2 => {
            current_block_72 = 7535020246387916674;
        }
        3 => {
            current_block_72 = 7231344791489249615;
        }
        4 => {
            current_block_72 = 3224681480744054787;
        }
        _ => {
            return BROTLI_DECODER_ERROR_UNREACHABLE as c_int
                as BrotliDecoderErrorCode;
        }
    }
    match current_block_72 {
        10468293953885100227 => {
            let mut bits: uint64_t = 0;
            if BrotliSafeGetBits(br, 5 as uint64_t, &raw mut bits) == 0 {
                return BROTLI_DECODER_NEEDS_MORE_INPUT;
            }
            if bits & 1 as uint64_t != 0 as uint64_t {
                (*h).max_run_length_prefix =
                    (bits >> 1 as c_int).wrapping_add(1 as uint64_t);
                BrotliDropBits(br, 5 as uint64_t);
            } else {
                (*h).max_run_length_prefix = 0 as uint64_t;
                BrotliDropBits(br, 1 as uint64_t);
            }
            (*h).substate_context_map = BROTLI_STATE_CONTEXT_MAP_HUFFMAN;
            current_block_72 = 7535020246387916674;
        }
        _ => {}
    }
    match current_block_72 {
        7535020246387916674 => {
            let mut alphabet_size: uint64_t =
                (*num_htrees).wrapping_add((*h).max_run_length_prefix);
            result = ReadHuffmanCode(
                alphabet_size,
                alphabet_size,
                &raw mut (*h).context_map_table as *mut HuffmanCode,
                ::core::ptr::null_mut::<uint64_t>(),
                s,
            );
            if result as c_int != BROTLI_DECODER_SUCCESS as c_int {
                return result;
            }
            (*h).code = 0xffff as uint64_t;
            (*h).substate_context_map = BROTLI_STATE_CONTEXT_MAP_DECODE;
            current_block_72 = 7231344791489249615;
        }
        _ => {}
    }
    match current_block_72 {
        7231344791489249615 => {
            let mut context_index: uint64_t = (*h).context_index;
            let mut max_run_length_prefix: uint64_t = (*h).max_run_length_prefix;
            let mut context_map: *mut uint8_t = *context_map_arg;
            let mut code: uint64_t = (*h).code;
            let mut skip_preamble: c_int =
                (code != 0xffff as uint64_t) as c_int;
            while context_index < context_map_size || skip_preamble != 0 {
                if skip_preamble == 0 {
                    if SafeReadSymbol(
                        &raw mut (*h).context_map_table as *mut HuffmanCode,
                        br,
                        &raw mut code,
                    ) == 0
                    {
                        (*h).code = 0xffff as uint64_t;
                        (*h).context_index = context_index;
                        return BROTLI_DECODER_NEEDS_MORE_INPUT;
                    }
                    if code == 0 as uint64_t {
                        let fresh2 = context_index;
                        context_index = context_index.wrapping_add(1);
                        *context_map.offset(fresh2 as isize) = 0 as uint8_t;
                        continue;
                    } else if code > max_run_length_prefix {
                        let fresh3 = context_index;
                        context_index = context_index.wrapping_add(1);
                        *context_map.offset(fresh3 as isize) =
                            code.wrapping_sub(max_run_length_prefix) as uint8_t;
                        continue;
                    }
                } else {
                    skip_preamble = BROTLI_FALSE;
                }
                let mut reps: uint64_t = 0;
                if BrotliSafeReadBits(br, code, &raw mut reps) == 0 {
                    (*h).code = code;
                    (*h).context_index = context_index;
                    return BROTLI_DECODER_NEEDS_MORE_INPUT;
                }
                reps = (reps as c_ulong).wrapping_add(
                    ((1 as c_uint as uint64_t) << code) as c_ulong,
                ) as uint64_t as uint64_t;
                if context_index.wrapping_add(reps) > context_map_size {
                    return BROTLI_DECODER_ERROR_FORMAT_CONTEXT_MAP_REPEAT as c_int
                        as BrotliDecoderErrorCode;
                }
                loop {
                    let fresh4 = context_index;
                    context_index = context_index.wrapping_add(1);
                    *context_map.offset(fresh4 as isize) = 0 as uint8_t;
                    reps = reps.wrapping_sub(1);
                    if !(reps != 0) {
                        break;
                    }
                }
            }
        }
        _ => {}
    }
    let mut bits_0: uint64_t = 0;
    if BrotliSafeReadBits(br, 1 as uint64_t, &raw mut bits_0) == 0 {
        (*h).substate_context_map = BROTLI_STATE_CONTEXT_MAP_TRANSFORM;
        return BROTLI_DECODER_NEEDS_MORE_INPUT;
    }
    if bits_0 != 0 as uint64_t {
        InverseMoveToFrontTransform(*context_map_arg, context_map_size, s);
    }
    (*h).substate_context_map = BROTLI_STATE_CONTEXT_MAP_NONE;
    return BROTLI_DECODER_SUCCESS;
}
#[inline(always)]
unsafe fn DecodeBlockTypeAndLength(
    mut safe: c_int,
    mut s: *mut BrotliDecoderStateInternal,
    mut tree_type: c_int,
) -> BrotliDecoderErrorCode {
    let mut max_block_type: uint64_t = (*s).num_block_types[tree_type as usize];
    let mut type_tree: *const HuffmanCode =
        (*s).block_type_trees
            .offset((tree_type * BROTLI_HUFFMAN_MAX_SIZE_258) as isize) as *mut HuffmanCode;
    let mut len_tree: *const HuffmanCode =
        (*s).block_len_trees
            .offset((tree_type * BROTLI_HUFFMAN_MAX_SIZE_26) as isize) as *mut HuffmanCode;
    let mut br: *mut BrotliBitReader = &raw mut (*s).br;
    let mut ringbuffer: *mut uint64_t = (&raw mut (*s).block_type_rb as *mut uint64_t)
        .offset((tree_type * 2 as c_int) as isize)
        as *mut uint64_t;
    let mut block_type: uint64_t = 0;
    if max_block_type <= 1 as uint64_t {
        return BROTLI_DECODER_ERROR_FORMAT_BLOCK_SWITCH;
    }
    if safe == 0 {
        block_type = ReadSymbol(type_tree, br);
        (*s).block_length[tree_type as usize] = ReadBlockLength(len_tree, br);
    } else {
        let mut memento: BrotliBitReaderState = BrotliBitReaderState {
            val_: 0,
            bit_pos_: 0,
            next_in: ::core::ptr::null::<uint8_t>(),
            avail_in: 0,
        };
        BrotliBitReaderSaveState(br, &raw mut memento);
        if SafeReadSymbol(type_tree, br, &raw mut block_type) == 0 {
            return BROTLI_DECODER_NEEDS_MORE_INPUT;
        }
        if SafeReadBlockLength(
            s,
            (&raw mut (*s).block_length as *mut uint64_t).offset(tree_type as isize)
                as *mut uint64_t,
            len_tree,
            br,
        ) == 0
        {
            (*s).substate_read_block_length = BROTLI_STATE_READ_BLOCK_LENGTH_NONE;
            BrotliBitReaderRestoreState(br, &raw mut memento);
            return BROTLI_DECODER_NEEDS_MORE_INPUT;
        }
    }
    if block_type == 1 as uint64_t {
        block_type =
            (*ringbuffer.offset(1 as c_int as isize)).wrapping_add(1 as uint64_t);
    } else if block_type == 0 as uint64_t {
        block_type = *ringbuffer.offset(0 as c_int as isize);
    } else {
        block_type = (block_type as c_ulong).wrapping_sub(2 as c_ulong)
            as uint64_t as uint64_t;
    }
    if block_type >= max_block_type {
        block_type = (block_type as c_ulong)
            .wrapping_sub(max_block_type as c_ulong) as uint64_t
            as uint64_t;
    }
    *ringbuffer.offset(0 as c_int as isize) =
        *ringbuffer.offset(1 as c_int as isize);
    *ringbuffer.offset(1 as c_int as isize) = block_type;
    return BROTLI_DECODER_SUCCESS;
}
#[inline(always)]
unsafe fn DetectTrivialLiteralBlockTypes(mut s: *mut BrotliDecoderStateInternal) {
    let mut i: size_t = 0;
    i = 0 as size_t;
    while i < 8 as size_t {
        (*s).trivial_literal_contexts[i as usize] = 0 as uint32_t;
        i = i.wrapping_add(1);
    }
    i = 0 as size_t;
    while i < (*s).num_block_types[0 as c_int as usize] as size_t {
        let mut offset: size_t = i << BROTLI_LITERAL_CONTEXT_BITS;
        let mut error: size_t = 0 as size_t;
        let mut sample: size_t = *(*s).context_map.offset(offset as isize) as size_t;
        let mut j: size_t = 0;
        j = 0 as size_t;
        while j < ((1 as c_uint) << BROTLI_LITERAL_CONTEXT_BITS) as size_t {
            let fresh5 = j;
            j = j.wrapping_add(1);
            error = (error as c_ulong
                | (*(*s)
                    .context_map
                    .offset(offset.wrapping_add(fresh5) as isize) as size_t
                    ^ sample) as c_ulong) as size_t;
            let fresh6 = j;
            j = j.wrapping_add(1);
            error = (error as c_ulong
                | (*(*s)
                    .context_map
                    .offset(offset.wrapping_add(fresh6) as isize) as size_t
                    ^ sample) as c_ulong) as size_t;
            let fresh7 = j;
            j = j.wrapping_add(1);
            error = (error as c_ulong
                | (*(*s)
                    .context_map
                    .offset(offset.wrapping_add(fresh7) as isize) as size_t
                    ^ sample) as c_ulong) as size_t;
            let fresh8 = j;
            j = j.wrapping_add(1);
            error = (error as c_ulong
                | (*(*s)
                    .context_map
                    .offset(offset.wrapping_add(fresh8) as isize) as size_t
                    ^ sample) as c_ulong) as size_t;
        }
        if error == 0 as size_t {
            (*s).trivial_literal_contexts[(i >> 5 as c_int) as usize] =
                ((*s).trivial_literal_contexts[(i >> 5 as c_int) as usize]
                    as c_uint
                    | (1 as c_uint) << (i & 31 as size_t)) as uint32_t;
        }
        i = i.wrapping_add(1);
    }
}
#[inline(always)]
unsafe fn PrepareLiteralDecoding(mut s: *mut BrotliDecoderStateInternal) {
    let mut context_mode: uint8_t = 0;
    let mut trivial: size_t = 0;
    let mut block_type: uint64_t = (*s).block_type_rb[1 as c_int as usize];
    let mut context_offset: uint64_t = block_type << BROTLI_LITERAL_CONTEXT_BITS;
    (*s).context_map_slice = (*s).context_map.offset(context_offset as isize);
    trivial =
        (*s).trivial_literal_contexts[(block_type >> 5 as c_int) as usize] as size_t;
    (*s).trivial_literal_context =
        (trivial >> (block_type & 31 as uint64_t) & 1 as size_t) as c_int;
    (*s).literal_htree = *(*s).literal_hgroup.htrees.offset(
        *(*s)
            .context_map_slice
            .offset(0 as c_int as isize) as isize,
    );
    context_mode = (*(*s).context_modes.offset(block_type as isize) as c_int
        & 3 as c_int) as uint8_t;
    (*s).context_lookup = (&raw const _kBrotliContextLookupTable as *const uint8_t)
        .offset(((context_mode as c_int) << 9 as c_int) as isize)
        as *const uint8_t;
}
#[inline(always)]
unsafe fn DecodeLiteralBlockSwitchInternal(
    mut safe: c_int,
    mut s: *mut BrotliDecoderStateInternal,
) -> BrotliDecoderErrorCode {
    let mut result: BrotliDecoderErrorCode =
        DecodeBlockTypeAndLength(safe, s, 0 as c_int);
    if result as c_int != BROTLI_DECODER_SUCCESS as c_int {
        return result;
    }
    PrepareLiteralDecoding(s);
    return BROTLI_DECODER_SUCCESS;
}
#[inline(never)]
unsafe fn DecodeLiteralBlockSwitch(
    mut s: *mut BrotliDecoderStateInternal,
) -> BrotliDecoderErrorCode {
    return DecodeLiteralBlockSwitchInternal(0 as c_int, s);
}
#[inline(never)]
unsafe fn SafeDecodeLiteralBlockSwitch(
    mut s: *mut BrotliDecoderStateInternal,
) -> BrotliDecoderErrorCode {
    return DecodeLiteralBlockSwitchInternal(1 as c_int, s);
}
#[inline(always)]
unsafe fn DecodeCommandBlockSwitchInternal(
    mut safe: c_int,
    mut s: *mut BrotliDecoderStateInternal,
) -> BrotliDecoderErrorCode {
    let mut result: BrotliDecoderErrorCode =
        DecodeBlockTypeAndLength(safe, s, 1 as c_int);
    if result as c_int != BROTLI_DECODER_SUCCESS as c_int {
        return result;
    }
    (*s).htree_command = *(*s)
        .insert_copy_hgroup
        .htrees
        .offset((*s).block_type_rb[3 as c_int as usize] as isize);
    return BROTLI_DECODER_SUCCESS;
}
#[inline(never)]
unsafe fn DecodeCommandBlockSwitch(
    mut s: *mut BrotliDecoderStateInternal,
) -> BrotliDecoderErrorCode {
    return DecodeCommandBlockSwitchInternal(0 as c_int, s);
}
#[inline(never)]
unsafe fn SafeDecodeCommandBlockSwitch(
    mut s: *mut BrotliDecoderStateInternal,
) -> BrotliDecoderErrorCode {
    return DecodeCommandBlockSwitchInternal(1 as c_int, s);
}
#[inline(always)]
unsafe fn DecodeDistanceBlockSwitchInternal(
    mut safe: c_int,
    mut s: *mut BrotliDecoderStateInternal,
) -> BrotliDecoderErrorCode {
    let mut result: BrotliDecoderErrorCode =
        DecodeBlockTypeAndLength(safe, s, 2 as c_int);
    if result as c_int != BROTLI_DECODER_SUCCESS as c_int {
        return result;
    }
    (*s).dist_context_map_slice = (*s).dist_context_map.offset(
        ((*s).block_type_rb[5 as c_int as usize] << BROTLI_DISTANCE_CONTEXT_BITS)
            as isize,
    );
    (*s).dist_htree_index = *(*s)
        .dist_context_map_slice
        .offset((*s).distance_context as isize);
    return BROTLI_DECODER_SUCCESS;
}
#[inline(never)]
unsafe fn DecodeDistanceBlockSwitch(
    mut s: *mut BrotliDecoderStateInternal,
) -> BrotliDecoderErrorCode {
    return DecodeDistanceBlockSwitchInternal(0 as c_int, s);
}
#[inline(never)]
unsafe fn SafeDecodeDistanceBlockSwitch(
    mut s: *mut BrotliDecoderStateInternal,
) -> c_int {
    return DecodeDistanceBlockSwitchInternal(1 as c_int, s) as c_int;
}
unsafe fn UnwrittenBytes(
    mut s: *const BrotliDecoderStateInternal,
    mut wrap: c_int,
) -> size_t {
    let s_view: &BrotliDecoderStateInternal = unsafe { &*s };
    let mut pos: size_t = if wrap != 0 && s_view.pos > s_view.ringbuffer_size {
        s_view.ringbuffer_size as size_t
    } else {
        s_view.pos as size_t
    };
    let mut partial_pos_rb: size_t = s_view
        .rb_roundtrips
        .wrapping_mul(s_view.ringbuffer_size as size_t)
        .wrapping_add(pos);
    return partial_pos_rb.wrapping_sub(s_view.partial_pos_out);
}
#[inline(never)]
unsafe fn WriteRingBuffer(
    mut s: *mut BrotliDecoderStateInternal,
    mut available_out: *mut size_t,
    mut next_out: *mut *mut uint8_t,
    mut total_out: *mut size_t,
    mut force: c_int,
) -> BrotliDecoderErrorCode {
    let available_out_view: &mut size_t = unsafe { &mut *available_out };
    let mut start: *mut uint8_t = (*s)
        .ringbuffer
        .offset(((*s).partial_pos_out & (*s).ringbuffer_mask as size_t) as isize);
    let mut to_write: size_t = UnwrittenBytes(s, BROTLI_TRUE);
    let mut num_written: size_t = *available_out_view;
    if num_written > to_write {
        num_written = to_write;
    }
    if (*s).meta_block_remaining_len < 0 as c_int {
        return BROTLI_DECODER_ERROR_FORMAT_BLOCK_LENGTH_1 as c_int
            as BrotliDecoderErrorCode;
    }
    if !next_out.is_null() && (*next_out).is_null() {
        *next_out = start;
    } else if !next_out.is_null() {
        memcpy(
            *next_out as *mut c_void,
            start as *const c_void,
            num_written,
        );
        *next_out = (*next_out).offset(num_written as isize);
    }
    *available_out_view = (*available_out_view as c_ulong)
        .wrapping_sub(num_written as c_ulong) as size_t as size_t;
    (*s).partial_pos_out = ((*s).partial_pos_out as c_ulong)
        .wrapping_add(num_written as c_ulong) as size_t
        as size_t;
    if !total_out.is_null() {
        *total_out = (*s).partial_pos_out;
    }
    if num_written < to_write {
        if (*s).ringbuffer_size
            == (1 as c_int) << (*s).window_bits() as c_int
            || force != 0
        {
            return BROTLI_DECODER_NEEDS_MORE_OUTPUT;
        } else {
            return BROTLI_DECODER_SUCCESS;
        }
    }
    if (*s).ringbuffer_size == (1 as c_int) << (*s).window_bits() as c_int
        && (*s).pos >= (*s).ringbuffer_size
    {
        (*s).pos -= (*s).ringbuffer_size;
        (*s).rb_roundtrips = (*s).rb_roundtrips.wrapping_add(1);
        (*s).set_should_wrap_ringbuffer(
            (if (*s).pos as size_t != 0 as size_t {
                1 as c_int
            } else {
                0 as c_int
            }) as c_uint as c_uint,
        );
    }
    return BROTLI_DECODER_SUCCESS;
}
#[inline(never)]
unsafe fn WrapRingBuffer(mut s: *mut BrotliDecoderStateInternal) {
    if (*s).should_wrap_ringbuffer() != 0 {
        memcpy(
            (*s).ringbuffer as *mut c_void,
            (*s).ringbuffer_end as *const c_void,
            (*s).pos as size_t,
        );
        (*s).set_should_wrap_ringbuffer(0 as c_uint as c_uint);
    }
}
#[inline(never)]
unsafe fn BrotliEnsureRingBuffer(
    mut s: *mut BrotliDecoderStateInternal,
) -> c_int {
    let s_view: &mut BrotliDecoderStateInternal = unsafe { &mut *s };
    let mut old_ringbuffer: *mut uint8_t = s_view.ringbuffer;
    if s_view.ringbuffer_size == s_view.new_ringbuffer_size {
        return BROTLI_TRUE;
    }
    s_view.ringbuffer = s_view.alloc_func.expect("non-null function pointer")(
        s_view.memory_manager_opaque,
        (s_view.new_ringbuffer_size as size_t).wrapping_add(kRingBufferWriteAheadSlack as size_t),
    ) as *mut uint8_t;
    if s_view.ringbuffer.is_null() {
        s_view.ringbuffer = old_ringbuffer;
        return BROTLI_FALSE;
    }
    *s_view
        .ringbuffer
        .offset((s_view.new_ringbuffer_size - 2 as c_int) as isize) = 0 as uint8_t;
    *s_view
        .ringbuffer
        .offset((s_view.new_ringbuffer_size - 1 as c_int) as isize) = 0 as uint8_t;
    if !old_ringbuffer.is_null() {
        memcpy(
            s_view.ringbuffer as *mut c_void,
            old_ringbuffer as *const c_void,
            s_view.pos as size_t,
        );
        s_view.free_func.expect("non-null function pointer")(
            s_view.memory_manager_opaque,
            old_ringbuffer as *mut c_void,
        );
        old_ringbuffer = ::core::ptr::null_mut::<uint8_t>();
    }
    s_view.ringbuffer_size = s_view.new_ringbuffer_size;
    s_view.ringbuffer_mask = s_view.new_ringbuffer_size - 1 as c_int;
    s_view.ringbuffer_end = s_view.ringbuffer.offset(s_view.ringbuffer_size as isize);
    return BROTLI_TRUE;
}
#[inline(never)]
unsafe fn SkipMetadataBlock(
    mut s: *mut BrotliDecoderStateInternal,
) -> BrotliDecoderErrorCode {
    let s_view: &mut BrotliDecoderStateInternal = unsafe { &mut *s };
    let mut br: *mut BrotliBitReader = &raw mut s_view.br;
    let mut nbytes: c_int = 0;
    if s_view.meta_block_remaining_len == 0 as c_int {
        return BROTLI_DECODER_SUCCESS;
    }
    if BrotliGetAvailableBits(br) >= 8 as uint64_t {
        let mut buffer: [uint8_t; 8] = [0; 8];
        nbytes = BrotliGetAvailableBits(br) as c_int >> 3 as c_int;
        if nbytes > s_view.meta_block_remaining_len {
            nbytes = s_view.meta_block_remaining_len;
        }
        BrotliCopyBytes(&raw mut buffer as *mut uint8_t, br, nbytes as size_t);
        if s_view.metadata_chunk_func.is_some() {
            s_view.metadata_chunk_func.expect("non-null function pointer")(
                s_view.metadata_callback_opaque,
                &raw mut buffer as *mut uint8_t,
                nbytes as size_t,
            );
        }
        s_view.meta_block_remaining_len -= nbytes;
        if s_view.meta_block_remaining_len == 0 as c_int {
            return BROTLI_DECODER_SUCCESS;
        }
    }
    nbytes = BrotliGetRemainingBytes(br) as c_int;
    if nbytes > s_view.meta_block_remaining_len {
        nbytes = s_view.meta_block_remaining_len;
    }
    if nbytes > 0 as c_int {
        if s_view.metadata_chunk_func.is_some() {
            s_view.metadata_chunk_func.expect("non-null function pointer")(
                s_view.metadata_callback_opaque,
                (*br).next_in,
                nbytes as size_t,
            );
        }
        BrotliDropBytes(br, nbytes as size_t);
        s_view.meta_block_remaining_len -= nbytes;
        if s_view.meta_block_remaining_len == 0 as c_int {
            return BROTLI_DECODER_SUCCESS;
        }
    }
    return BROTLI_DECODER_NEEDS_MORE_INPUT;
}
#[inline(never)]
unsafe fn CopyUncompressedBlockToOutput(
    mut available_out: *mut size_t,
    mut next_out: *mut *mut uint8_t,
    mut total_out: *mut size_t,
    mut s: *mut BrotliDecoderStateInternal,
) -> BrotliDecoderErrorCode {
    if BrotliEnsureRingBuffer(s) == 0 {
        return BROTLI_DECODER_ERROR_ALLOC_RING_BUFFER_1 as c_int
            as BrotliDecoderErrorCode;
    }
    loop {
        let mut current_block_27: u64;
        match (*s).substate_uncompressed as c_uint {
            0 => {
                let mut nbytes: c_int =
                    BrotliGetRemainingBytes(&raw mut (*s).br) as c_int;
                if nbytes > (*s).meta_block_remaining_len {
                    nbytes = (*s).meta_block_remaining_len;
                }
                if (*s).pos + nbytes > (*s).ringbuffer_size {
                    nbytes = (*s).ringbuffer_size - (*s).pos;
                }
                BrotliCopyBytes(
                    (*s).ringbuffer.offset((*s).pos as isize) as *mut uint8_t,
                    &raw mut (*s).br,
                    nbytes as size_t,
                );
                (*s).pos += nbytes;
                (*s).meta_block_remaining_len -= nbytes;
                if (*s).pos < (1 as c_int) << (*s).window_bits() as c_int
                {
                    if (*s).meta_block_remaining_len == 0 as c_int {
                        return BROTLI_DECODER_SUCCESS;
                    }
                    return BROTLI_DECODER_NEEDS_MORE_INPUT;
                }
                (*s).substate_uncompressed = BROTLI_STATE_UNCOMPRESSED_WRITE;
                current_block_27 = 7149356873433890176;
            }
            1 => {
                current_block_27 = 7149356873433890176;
            }
            _ => {
                current_block_27 = 11298138898191919651;
            }
        }
        match current_block_27 {
            7149356873433890176 => {
                let mut result: BrotliDecoderErrorCode = BROTLI_DECODER_NO_ERROR;
                result = WriteRingBuffer(s, available_out, next_out, total_out, BROTLI_FALSE);
                if result as c_int != BROTLI_DECODER_SUCCESS as c_int {
                    return result;
                }
                if (*s).ringbuffer_size
                    == (1 as c_int) << (*s).window_bits() as c_int
                {
                    (*s).max_distance = (*s).max_backward_distance;
                }
                (*s).substate_uncompressed = BROTLI_STATE_UNCOMPRESSED_NONE;
            }
            _ => {}
        }
    }
}
unsafe fn AttachCompoundDictionary(
    mut state: *mut BrotliDecoderStateInternal,
    mut data: *const uint8_t,
    mut size: size_t,
) -> c_int {
    let state_view: &mut BrotliDecoderStateInternal = unsafe { &mut *state };
    let mut addon: *mut BrotliDecoderCompoundDictionary = state_view.compound_dictionary;
    if size == 0 as size_t {
        return BROTLI_TRUE;
    }
    if size > SHARED_BROTLI_MAX_RAW_DICT_SIZE as size_t {
        return BROTLI_FALSE;
    }
    if state_view.state as c_uint
        != BROTLI_STATE_UNINITED as c_int as c_uint
    {
        return BROTLI_FALSE;
    }
    if addon.is_null() {
        addon = state_view.alloc_func.expect("non-null function pointer")(
            state_view.memory_manager_opaque,
            ::core::mem::size_of::<BrotliDecoderCompoundDictionary>() as size_t,
        ) as *mut BrotliDecoderCompoundDictionary;
        if addon.is_null() {
            return BROTLI_FALSE;
        }
        (*addon).num_chunks = 0 as uint8_t;
        (*addon).block_bits = 255 as uint8_t;
        (*addon).br_index = 0 as uint16_t;
        (*addon).total_size = 0 as c_uint as uint32_t;
        (*addon).br_offset = 0 as c_uint as uint32_t;
        (*addon).br_length = 0 as c_uint as uint32_t;
        (*addon).br_copied = 0 as c_uint as uint32_t;
        (*addon).chunk_offsets[0 as c_int as usize] =
            0 as c_uint as uint32_t;
        state_view.compound_dictionary = addon;
    }
    if (*addon).num_chunks as c_int == SHARED_BROTLI_MAX_COMPOUND_DICTS {
        return BROTLI_FALSE;
    }
    if size
        > (SHARED_BROTLI_MAX_RAW_DICT_SIZE as uint32_t).wrapping_sub((*addon).total_size) as size_t
    {
        return BROTLI_FALSE;
    }
    (*addon).chunks[(*addon).num_chunks as usize] = data;
    (*addon).num_chunks = (*addon).num_chunks.wrapping_add(1);
    (*addon).total_size = ((*addon).total_size as c_uint)
        .wrapping_add(size as uint32_t as c_uint) as uint32_t
        as uint32_t;
    (*addon).chunk_offsets[(*addon).num_chunks as usize] = (*addon).total_size;
    return BROTLI_TRUE;
}
unsafe fn EnsureCompoundDictionaryInitialized(
    mut state: *mut BrotliDecoderStateInternal,
) {
    let mut addon: *mut BrotliDecoderCompoundDictionary = (*state).compound_dictionary;
    let mut block_bits: size_t = 8 as size_t;
    let mut cursor: uint32_t = 0 as uint32_t;
    let mut index: size_t = 0 as size_t;
    let mut maximal_address: uint32_t = (*addon).total_size.wrapping_sub(1 as uint32_t);
    if (*addon).block_bits as c_uint != 255 as c_uint {
        return;
    }
    while maximal_address >> block_bits != 0 as uint32_t {
        block_bits = block_bits.wrapping_add(1);
    }
    block_bits = (block_bits as c_ulong).wrapping_sub(8 as c_ulong)
        as size_t as size_t;
    (*addon).block_bits = block_bits as uint8_t;
    while cursor <= maximal_address {
        while (*addon).chunk_offsets[index.wrapping_add(1 as size_t) as usize] < cursor {
            index = index.wrapping_add(1);
        }
        (*addon).block_map[(cursor >> block_bits) as usize] = index as uint8_t;
        cursor = (cursor as c_uint)
            .wrapping_add((1 as c_uint) << block_bits) as uint32_t
            as uint32_t;
    }
}
unsafe fn InitializeCompoundDictionaryCopy(
    mut s: *mut BrotliDecoderStateInternal,
    mut address: uint32_t,
    mut length: uint32_t,
) -> c_int {
    let mut addon: *mut BrotliDecoderCompoundDictionary = (*s).compound_dictionary;
    let mut index: size_t = 0;
    EnsureCompoundDictionaryInitialized(s);
    index = (*addon).block_map[(address >> (*addon).block_bits as c_int) as usize]
        as size_t;
    while address >= (*addon).chunk_offsets[index.wrapping_add(1 as size_t) as usize] {
        index = index.wrapping_add(1);
    }
    if length > (*addon).total_size.wrapping_sub(address) {
        return BROTLI_FALSE;
    }
    (*s).dist_rb[((*s).dist_rb_idx & 3 as c_int) as usize] = (*s).distance_code;
    (*s).dist_rb_idx += 1;
    (*s).meta_block_remaining_len = ((*s).meta_block_remaining_len as c_uint)
        .wrapping_sub(length as c_uint)
        as c_int as c_int;
    (*addon).br_index = index as uint16_t;
    (*addon).br_offset = address.wrapping_sub((*addon).chunk_offsets[index as usize]);
    (*addon).br_length = length;
    (*addon).br_copied = 0 as c_uint as uint32_t;
    return BROTLI_TRUE;
}
unsafe fn GetCompoundDictionarySize(mut s: *mut BrotliDecoderStateInternal) -> uint32_t {
    let s_view: &BrotliDecoderStateInternal = unsafe { &*s };
    return if !s_view.compound_dictionary.is_null() {
        (*s_view.compound_dictionary).total_size
    } else {
        0 as uint32_t
    };
}
unsafe fn CopyFromCompoundDictionary(
    mut s: *mut BrotliDecoderStateInternal,
    mut pos: c_int,
) -> c_int {
    let mut addon: *mut BrotliDecoderCompoundDictionary = (*s).compound_dictionary;
    let mut orig_pos: c_int = pos;
    while (*addon).br_length != (*addon).br_copied {
        let mut copy_dst: *mut uint8_t = (*s).ringbuffer.offset(pos as isize) as *mut uint8_t;
        let mut copy_src: *const uint8_t =
            (*addon).chunks[(*addon).br_index as usize].offset((*addon).br_offset as isize);
        let mut space: c_int = (*s).ringbuffer_size - pos;
        let mut rem_chunk_length: uint32_t = (*addon).chunk_offsets
            [((*addon).br_index as c_int + 1 as c_int) as usize]
            .wrapping_sub((*addon).chunk_offsets[(*addon).br_index as usize])
            .wrapping_sub((*addon).br_offset);
        let mut length: uint32_t = (*addon).br_length.wrapping_sub((*addon).br_copied);
        if length > rem_chunk_length {
            length = rem_chunk_length;
        }
        if length > space as uint32_t {
            length = space as uint32_t;
        }
        memcpy(
            copy_dst as *mut c_void,
            copy_src as *const c_void,
            length as size_t,
        );
        pos = (pos as c_uint).wrapping_add(length as c_uint)
            as c_int as c_int;
        (*addon).br_offset = ((*addon).br_offset as c_uint)
            .wrapping_add(length as c_uint) as uint32_t
            as uint32_t;
        (*addon).br_copied = ((*addon).br_copied as c_uint)
            .wrapping_add(length as c_uint) as uint32_t
            as uint32_t;
        if length == rem_chunk_length {
            (*addon).br_index = (*addon).br_index.wrapping_add(1);
            (*addon).br_offset = 0 as c_uint as uint32_t;
        }
        if pos == (*s).ringbuffer_size {
            break;
        }
    }
    return pos - orig_pos;
}
#[inline]
pub unsafe fn BrotliDecoderAttachDictionary(
    mut state: *mut BrotliDecoderStateInternal,
    mut type_0: BrotliSharedDictionaryType,
    mut data_size: size_t,
    mut data: *const uint8_t,
) -> c_int {
    let mut i: uint64_t = 0;
    let mut num_prefix_before: uint64_t = (*(*state).dictionary).num_prefix as uint64_t;
    if (*state).state as c_uint
        != BROTLI_STATE_UNINITED as c_int as c_uint
    {
        return BROTLI_FALSE;
    }
    if BrotliSharedDictionaryAttach(
        (*state).dictionary as *mut BrotliSharedDictionary,
        type_0,
        data_size,
        data,
    ) == 0
    {
        return BROTLI_FALSE;
    }
    i = num_prefix_before;
    while i < (*(*state).dictionary).num_prefix as uint64_t {
        if AttachCompoundDictionary(
            state,
            (*(*state).dictionary).prefix[i as usize],
            (*(*state).dictionary).prefix_size[i as usize],
        ) == 0
        {
            return BROTLI_FALSE;
        }
        i = i.wrapping_add(1);
    }
    return BROTLI_TRUE;
}
#[inline(never)]
unsafe fn BrotliCalculateRingBufferSize(mut s: *mut BrotliDecoderStateInternal) {
    let s_view: &mut BrotliDecoderStateInternal = unsafe { &mut *s };
    let mut window_size: c_int =
        (1 as c_int) << s_view.window_bits() as c_int;
    let mut new_ringbuffer_size: c_int = window_size;
    let mut min_size: c_int = if s_view.ringbuffer_size != 0 {
        s_view.ringbuffer_size
    } else {
        1024 as c_int
    };
    let mut output_size: c_int = 0;
    if s_view.ringbuffer_size == window_size {
        return;
    }
    if s_view.is_metadata() != 0 {
        return;
    }
    if s_view.ringbuffer.is_null() {
        output_size = 0 as c_int;
    } else {
        output_size = s_view.pos;
    }
    output_size += s_view.meta_block_remaining_len;
    min_size = if min_size < output_size {
        output_size
    } else {
        min_size
    };
    if s_view.canny_ringbuffer_allocation() != 0 {
        while new_ringbuffer_size >> 1 as c_int >= min_size {
            new_ringbuffer_size >>= 1 as c_int;
        }
    }
    s_view.new_ringbuffer_size = new_ringbuffer_size;
}
unsafe fn ReadContextModes(
    mut s: *mut BrotliDecoderStateInternal,
) -> BrotliDecoderErrorCode {
    let s_view: &mut BrotliDecoderStateInternal = unsafe { &mut *s };
    let mut br: *mut BrotliBitReader = &raw mut s_view.br;
    let mut i: c_int = s_view.loop_counter;
    while i < s_view.num_block_types[0 as c_int as usize] as c_int {
        let mut bits: uint64_t = 0;
        if BrotliSafeReadBits(br, 2 as uint64_t, &raw mut bits) == 0 {
            s_view.loop_counter = i;
            return BROTLI_DECODER_NEEDS_MORE_INPUT;
        }
        *s_view.context_modes.offset(i as isize) = bits as uint8_t;
        i += 1;
    }
    return BROTLI_DECODER_SUCCESS;
}
#[inline(always)]
unsafe fn TakeDistanceFromRingBuffer(mut s: *mut BrotliDecoderStateInternal) {
    let mut offset: c_int = (*s).distance_code - 3 as c_int;
    if (*s).distance_code <= 3 as c_int {
        (*s).distance_context = 1 as c_int >> (*s).distance_code;
        (*s).distance_code =
            (*s).dist_rb[((*s).dist_rb_idx - offset & 3 as c_int) as usize];
        (*s).dist_rb_idx -= (*s).distance_context;
    } else {
        let mut index_delta: c_int = 3 as c_int;
        let mut delta: c_int = 0;
        let mut base: c_int = (*s).distance_code - 10 as c_int;
        if (*s).distance_code < 10 as c_int {
            base = (*s).distance_code - 4 as c_int;
        } else {
            index_delta = 2 as c_int;
        }
        delta = (0x605142 as c_int >> 4 as c_int * base
            & 0xf as c_int)
            - 3 as c_int;
        (*s).distance_code = (*s).dist_rb
            [((*s).dist_rb_idx + index_delta & 0x3 as c_int) as usize]
            + delta;
        if (*s).distance_code <= 0 as c_int {
            (*s).distance_code = 0x7fffffff as c_int;
        }
    };
}
#[inline(always)]
unsafe fn SafeReadBits(
    br: *mut BrotliBitReader,
    mut n_bits: uint64_t,
    mut val: *mut uint64_t,
) -> c_int {
    if n_bits != 0 as uint64_t {
        return BrotliSafeReadBits(br, n_bits, val);
    } else {
        *val = 0 as uint64_t;
        return BROTLI_TRUE;
    };
}
#[inline(always)]
unsafe fn SafeReadBits32(
    br: *mut BrotliBitReader,
    mut n_bits: uint64_t,
    mut val: *mut uint64_t,
) -> c_int {
    if n_bits != 0 as uint64_t {
        return BrotliSafeReadBits32(br, n_bits, val);
    } else {
        *val = 0 as uint64_t;
        return BROTLI_TRUE;
    };
}
unsafe fn CalculateDistanceLut(mut s: *mut BrotliDecoderStateInternal) {
    let s_view: &mut BrotliDecoderStateInternal = unsafe { &mut *s };
    let mut b: *mut BrotliMetablockBodyArena = &raw mut s_view.arena.body;
    let mut npostfix: uint64_t = s_view.distance_postfix_bits;
    let mut ndirect: uint64_t = s_view.num_direct_distance_codes;
    let mut alphabet_size_limit: uint64_t = s_view.distance_hgroup.alphabet_size_limit as uint64_t;
    let mut postfix: uint64_t = (1 as c_uint as uint64_t) << npostfix;
    let mut j: uint64_t = 0;
    let mut bits: uint64_t = 1 as uint64_t;
    let mut half: uint64_t = 0 as uint64_t;
    let mut i: uint64_t = BROTLI_NUM_DISTANCE_SHORT_CODES as uint64_t;
    j = 0 as uint64_t;
    while j < ndirect {
        (*b).dist_extra_bits[i as usize] = 0 as uint8_t;
        (*b).dist_offset[i as usize] = j.wrapping_add(1 as uint64_t);
        i = i.wrapping_add(1);
        j = j.wrapping_add(1);
    }
    while i < alphabet_size_limit {
        let mut base: uint64_t = ndirect
            .wrapping_add(
                ((2 as uint64_t).wrapping_add(half) << bits).wrapping_sub(4 as uint64_t)
                    << npostfix,
            )
            .wrapping_add(1 as uint64_t);
        j = 0 as uint64_t;
        while j < postfix {
            (*b).dist_extra_bits[i as usize] = bits as uint8_t;
            (*b).dist_offset[i as usize] = base.wrapping_add(j);
            i = i.wrapping_add(1);
            j = j.wrapping_add(1);
        }
        bits = bits.wrapping_add(half);
        half = half ^ 1 as uint64_t;
    }
}
#[inline(always)]
unsafe fn ReadDistanceInternal(
    mut safe: c_int,
    mut s: *mut BrotliDecoderStateInternal,
    mut br: *mut BrotliBitReader,
) -> c_int {
    let mut b: *mut BrotliMetablockBodyArena = &raw mut (*s).arena.body;
    let mut code: uint64_t = 0;
    let mut bits: uint64_t = 0;
    let mut memento: BrotliBitReaderState = BrotliBitReaderState {
        val_: 0,
        bit_pos_: 0,
        next_in: ::core::ptr::null::<uint8_t>(),
        avail_in: 0,
    };
    let mut distance_tree: *mut HuffmanCode = *(*s)
        .distance_hgroup
        .htrees
        .offset((*s).dist_htree_index as isize);
    if safe == 0 {
        code = ReadSymbol(distance_tree, br);
    } else {
        BrotliBitReaderSaveState(br, &raw mut memento);
        if SafeReadSymbol(distance_tree, br, &raw mut code) == 0 {
            return BROTLI_FALSE;
        }
    }
    (*s).block_length[2 as c_int as usize] =
        (*s).block_length[2 as c_int as usize].wrapping_sub(1);
    (*s).distance_context = 0 as c_int;
    if code & !(0xf as c_uint) as uint64_t == 0 as uint64_t {
        (*s).distance_code = code as c_int;
        TakeDistanceFromRingBuffer(s);
        return BROTLI_TRUE;
    }
    if safe == 0 {
        bits = BrotliReadBits32(br, (*b).dist_extra_bits[code as usize] as uint64_t);
    } else if SafeReadBits32(
        br,
        (*b).dist_extra_bits[code as usize] as uint64_t,
        &raw mut bits,
    ) == 0
    {
        (*s).block_length[2 as c_int as usize] =
            (*s).block_length[2 as c_int as usize].wrapping_add(1);
        BrotliBitReaderRestoreState(br, &raw mut memento);
        return BROTLI_FALSE;
    }
    (*s).distance_code = (*b).dist_offset[code as usize]
        .wrapping_add(bits << (*s).distance_postfix_bits)
        as c_int;
    return BROTLI_TRUE;
}
#[inline(always)]
unsafe fn ReadDistance(
    mut s: *mut BrotliDecoderStateInternal,
    mut br: *mut BrotliBitReader,
) {
    ReadDistanceInternal(0 as c_int, s, br);
}
#[inline(always)]
unsafe fn SafeReadDistance(
    mut s: *mut BrotliDecoderStateInternal,
    mut br: *mut BrotliBitReader,
) -> c_int {
    return ReadDistanceInternal(1 as c_int, s, br);
}
#[inline(always)]
unsafe fn ReadCommandInternal(
    mut safe: c_int,
    mut s: *mut BrotliDecoderStateInternal,
    mut br: *mut BrotliBitReader,
    mut insert_length: *mut c_int,
) -> c_int {
    let mut cmd_code: uint64_t = 0;
    let mut insert_len_extra: uint64_t = 0 as uint64_t;
    let mut copy_length: uint64_t = 0;
    let mut v: CmdLutElement = CmdLutElement {
        insert_len_extra_bits: 0,
        copy_len_extra_bits: 0,
        distance_code: 0,
        context: 0,
        insert_len_offset: 0,
        copy_len_offset: 0,
    };
    let mut memento: BrotliBitReaderState = BrotliBitReaderState {
        val_: 0,
        bit_pos_: 0,
        next_in: ::core::ptr::null::<uint8_t>(),
        avail_in: 0,
    };
    if safe == 0 {
        cmd_code = ReadSymbol((*s).htree_command, br);
    } else {
        BrotliBitReaderSaveState(br, &raw mut memento);
        if SafeReadSymbol((*s).htree_command, br, &raw mut cmd_code) == 0 {
            return BROTLI_FALSE;
        }
    }
    v = kCmdLut[cmd_code as usize];
    (*s).distance_code = v.distance_code as c_int;
    (*s).distance_context = v.context as c_int;
    (*s).dist_htree_index = *(*s)
        .dist_context_map_slice
        .offset((*s).distance_context as isize);
    *insert_length = v.insert_len_offset as c_int;
    if safe == 0 {
        if (v.insert_len_extra_bits as c_int != 0 as c_int)
            as c_int as c_long
            != 0
        {
            insert_len_extra = BrotliReadBits24(br, v.insert_len_extra_bits as uint64_t);
        }
        copy_length = BrotliReadBits24(br, v.copy_len_extra_bits as uint64_t);
    } else if SafeReadBits(
        br,
        v.insert_len_extra_bits as uint64_t,
        &raw mut insert_len_extra,
    ) == 0
        || SafeReadBits(br, v.copy_len_extra_bits as uint64_t, &raw mut copy_length) == 0
    {
        BrotliBitReaderRestoreState(br, &raw mut memento);
        return BROTLI_FALSE;
    }
    (*s).copy_length = copy_length as c_int + v.copy_len_offset as c_int;
    (*s).block_length[1 as c_int as usize] =
        (*s).block_length[1 as c_int as usize].wrapping_sub(1);
    *insert_length += insert_len_extra as c_int;
    return BROTLI_TRUE;
}
#[inline(always)]
unsafe fn ReadCommand(
    mut s: *mut BrotliDecoderStateInternal,
    mut br: *mut BrotliBitReader,
    mut insert_length: *mut c_int,
) {
    ReadCommandInternal(0 as c_int, s, br, insert_length);
}
#[inline(always)]
unsafe fn SafeReadCommand(
    mut s: *mut BrotliDecoderStateInternal,
    mut br: *mut BrotliBitReader,
    mut insert_length: *mut c_int,
) -> c_int {
    return ReadCommandInternal(1 as c_int, s, br, insert_length);
}
#[inline(always)]
unsafe fn CheckInputAmount(
    mut safe: c_int,
    br: *mut BrotliBitReader,
) -> c_int {
    if safe != 0 {
        return BROTLI_TRUE;
    }
    return BrotliCheckInputAmount(br);
}
#[inline(always)]
unsafe fn ProcessCommandsInternal(
    mut safe: c_int,
    mut s: *mut BrotliDecoderStateInternal,
) -> BrotliDecoderErrorCode {
    let mut current_block: u32;
    let mut pos: c_int = (*s).pos;
    let mut i: c_int = (*s).loop_counter;
    let mut result: BrotliDecoderErrorCode = BROTLI_DECODER_SUCCESS;
    let mut br: *mut BrotliBitReader = &raw mut (*s).br;
    let mut compound_dictionary_size: uint32_t = GetCompoundDictionarySize(s);
    if CheckInputAmount(safe, br) == 0 {
        result = BROTLI_DECODER_NEEDS_MORE_INPUT;
    } else {
        if safe == 0 {
            BrotliWarmupBitReader(br);
        }
        if (*s).state as c_uint
            == BROTLI_STATE_COMMAND_BEGIN as c_int as c_uint
        {
            current_block = 0;
        } else if (*s).state as c_uint
            == BROTLI_STATE_COMMAND_INNER as c_int as c_uint
        {
            current_block = 1;
        } else if (*s).state as c_uint
            == BROTLI_STATE_COMMAND_POST_DECODE_LITERALS as c_int
                as c_uint
        {
            current_block = 2;
        } else if (*s).state as c_uint
            == BROTLI_STATE_COMMAND_POST_WRAP_COPY as c_int as c_uint
        {
            current_block = 3;
        } else {
            return BROTLI_DECODER_ERROR_UNREACHABLE as c_int
                as BrotliDecoderErrorCode;
        }
        '_CommandBegin: loop {
            match current_block {
                2 => {
                    if safe != 0 {
                        (*s).state = BROTLI_STATE_COMMAND_POST_DECODE_LITERALS;
                    }
                    if (*s).distance_code >= 0 as c_int {
                        (*s).distance_context = if (*s).distance_code != 0 {
                            0 as c_int
                        } else {
                            1 as c_int
                        };
                        (*s).dist_rb_idx -= 1;
                        (*s).distance_code =
                            (*s).dist_rb[((*s).dist_rb_idx & 3 as c_int) as usize];
                    } else {
                        if ((*s).block_length[2 as c_int as usize] == 0 as uint64_t)
                            as c_int as c_long
                            != 0
                        {
                            let mut status_0: BrotliDecoderErrorCode = BROTLI_DECODER_NO_ERROR;
                            if safe != 0 {
                                status_0 =
                                    SafeDecodeDistanceBlockSwitch(s) as BrotliDecoderErrorCode;
                            } else {
                                status_0 = DecodeDistanceBlockSwitch(s);
                            }
                            if status_0 as c_int
                                != BROTLI_DECODER_SUCCESS as c_int
                            {
                                result = status_0;
                                break;
                            }
                        }
                        if safe != 0 {
                            if SafeReadDistance(s, br) == 0 {
                                result = BROTLI_DECODER_NEEDS_MORE_INPUT;
                                break;
                            }
                        } else {
                            ReadDistance(s, br);
                        }
                    }
                    if (*s).max_distance != (*s).max_backward_distance {
                        (*s).max_distance = if pos < (*s).max_backward_distance {
                            pos
                        } else {
                            (*s).max_backward_distance
                        };
                    }
                    i = (*s).copy_length;
                    if (*s).distance_code > (*s).max_distance {
                        if (*s).distance_code > BROTLI_MAX_ALLOWED_DISTANCE {
                            return BROTLI_DECODER_ERROR_FORMAT_DISTANCE as c_int
                                as BrotliDecoderErrorCode;
                        }
                        if (((*s).distance_code - (*s).max_distance) as uint32_t)
                            .wrapping_sub(1 as uint32_t)
                            < compound_dictionary_size
                        {
                            let mut address: uint32_t = compound_dictionary_size
                                .wrapping_sub(((*s).distance_code - (*s).max_distance) as uint32_t);
                            if InitializeCompoundDictionaryCopy(s, address, i as uint32_t) == 0 {
                                return BROTLI_DECODER_ERROR_COMPOUND_DICTIONARY
                                    as c_int
                                    as BrotliDecoderErrorCode;
                            }
                            pos += CopyFromCompoundDictionary(s, pos);
                            if pos >= (*s).ringbuffer_size {
                                (*s).state = BROTLI_STATE_COMMAND_POST_WRITE_1;
                                break;
                            }
                        } else if i >= SHARED_BROTLI_MIN_DICTIONARY_WORD_LENGTH
                            && i <= SHARED_BROTLI_MAX_DICTIONARY_WORD_LENGTH
                        {
                            let mut p1_0: uint8_t = *(*s).ringbuffer.offset(
                                (pos - 1 as c_int & (*s).ringbuffer_mask) as isize,
                            );
                            let mut p2_0: uint8_t = *(*s).ringbuffer.offset(
                                (pos - 2 as c_int & (*s).ringbuffer_mask) as isize,
                            );
                            let mut dict_id: uint8_t = (if (*(*s).dictionary).context_based != 0 {
                                (*(*s).dictionary).context_map[(*(*s)
                                    .context_lookup
                                    .offset(p1_0 as isize)
                                    as c_int
                                    | *(*s)
                                        .context_lookup
                                        .offset(256 as c_int as isize)
                                        .offset(p2_0 as isize)
                                        as c_int)
                                    as usize] as c_int
                            } else {
                                0 as c_int
                            }) as uint8_t;
                            let mut words: *const BrotliDictionary =
                                (*(*s).dictionary).words[dict_id as usize];
                            let mut transforms: *const BrotliTransforms =
                                (*(*s).dictionary).transforms[dict_id as usize];
                            let mut offset: c_int =
                                (*words).offsets_by_length[i as usize] as c_int;
                            let mut shift: uint64_t =
                                (*words).size_bits_by_length[i as usize] as uint64_t;
                            let mut address_0: c_int = (*s).distance_code
                                - (*s).max_distance
                                - 1 as c_int
                                - compound_dictionary_size as c_int;
                            let mut mask: c_int = BitMask(shift) as c_int;
                            let mut word_idx: c_int = address_0 & mask;
                            let mut transform_idx: c_int = address_0 >> shift;
                            (*s).dist_rb_idx += (*s).distance_context;
                            offset += word_idx * i;
                            if (transform_idx >= (*transforms).num_transforms as c_int
                                || (*words).size_bits_by_length[i as usize] as c_int
                                    == 0 as c_int)
                                && (*(*s).dictionary).num_dictionaries as c_int
                                    > 1 as c_int
                            {
                                let mut dict_id2: uint8_t = 0;
                                let mut dist_remaining: c_int = address_0
                                    - ((1 as c_uint) << shift
                                        & !(1 as c_uint))
                                        as c_int
                                        * (*transforms).num_transforms as c_int;
                                dict_id2 = 0 as uint8_t;
                                while (dict_id2 as c_int)
                                    < (*(*s).dictionary).num_dictionaries as c_int
                                {
                                    let mut words2: *const BrotliDictionary =
                                        (*(*s).dictionary).words[dict_id2 as usize];
                                    if dict_id2 as c_int
                                        != dict_id as c_int
                                        && (*words2).size_bits_by_length[i as usize]
                                            as c_int
                                            != 0 as c_int
                                    {
                                        let mut transforms2: *const BrotliTransforms =
                                            (*(*s).dictionary).transforms[dict_id2 as usize];
                                        let mut shift2: uint64_t =
                                            (*words2).size_bits_by_length[i as usize] as uint64_t;
                                        let mut num: c_int = ((1
                                            as c_uint)
                                            << shift2
                                            & !(1 as c_uint))
                                            as c_int
                                            * (*transforms2).num_transforms as c_int;
                                        if dist_remaining < num {
                                            dict_id = dict_id2;
                                            words = words2;
                                            transforms = transforms2;
                                            address_0 = dist_remaining;
                                            shift = shift2;
                                            mask = BitMask(shift) as c_int;
                                            word_idx = address_0 & mask;
                                            transform_idx = address_0 >> shift;
                                            offset = (*words).offsets_by_length[i as usize]
                                                as c_int
                                                + word_idx * i;
                                            break;
                                        } else {
                                            dist_remaining -= num;
                                        }
                                    }
                                    dict_id2 = dict_id2.wrapping_add(1);
                                }
                            }
                            if ((*words).size_bits_by_length[i as usize] as c_int
                                == 0 as c_int)
                                as c_int
                                as c_long
                                != 0
                            {
                                return BROTLI_DECODER_ERROR_FORMAT_DICTIONARY as c_int
                                    as BrotliDecoderErrorCode;
                            }
                            if (*words).data.is_null() as c_int as c_long
                                != 0
                            {
                                return BROTLI_DECODER_ERROR_DICTIONARY_NOT_SET as c_int
                                    as BrotliDecoderErrorCode;
                            }
                            if transform_idx < (*transforms).num_transforms as c_int {
                                let mut word: *const uint8_t =
                                    (*words).data.offset(offset as isize) as *const uint8_t;
                                let mut len: c_int = i;
                                if transform_idx
                                    == (*transforms).cutOffTransforms
                                        [0 as c_int as usize]
                                        as c_int
                                {
                                    memcpy(
                                        (*s).ringbuffer.offset(pos as isize) as *mut uint8_t
                                            as *mut c_void,
                                        word as *const c_void,
                                        len as size_t,
                                    );
                                } else {
                                    len = BrotliTransformDictionaryWord(
                                        (*s).ringbuffer.offset(pos as isize) as *mut uint8_t,
                                        word,
                                        len,
                                        transforms,
                                        transform_idx,
                                    );
                                    if len == 0 as c_int
                                        && (*s).distance_code <= 120 as c_int
                                    {
                                        return BROTLI_DECODER_ERROR_FORMAT_TRANSFORM
                                            as c_int
                                            as BrotliDecoderErrorCode;
                                    }
                                }
                                pos += len;
                                (*s).meta_block_remaining_len -= len;
                                if pos >= (*s).ringbuffer_size {
                                    (*s).state = BROTLI_STATE_COMMAND_POST_WRITE_1;
                                    break;
                                }
                            } else {
                                return BROTLI_DECODER_ERROR_FORMAT_TRANSFORM as c_int
                                    as BrotliDecoderErrorCode;
                            }
                        } else {
                            return BROTLI_DECODER_ERROR_FORMAT_DICTIONARY as c_int
                                as BrotliDecoderErrorCode;
                        }
                    } else {
                        let mut src_start: c_int =
                            pos - (*s).distance_code & (*s).ringbuffer_mask;
                        let mut copy_dst: *mut uint8_t =
                            (*s).ringbuffer.offset(pos as isize) as *mut uint8_t;
                        let mut copy_src: *mut uint8_t =
                            (*s).ringbuffer.offset(src_start as isize) as *mut uint8_t;
                        let mut dst_end: c_int = pos + i;
                        let mut src_end: c_int = src_start + i;
                        (*s).dist_rb[((*s).dist_rb_idx & 3 as c_int) as usize] =
                            (*s).distance_code;
                        (*s).dist_rb_idx += 1;
                        (*s).meta_block_remaining_len -= i;
                        memmove16(copy_dst, copy_src);
                        if src_end > pos && dst_end > src_start {
                            current_block = 3;
                            continue;
                        }
                        if dst_end >= (*s).ringbuffer_size || src_end >= (*s).ringbuffer_size {
                            current_block = 3;
                            continue;
                        }
                        pos += i;
                        if i > 16 as c_int {
                            if i > 32 as c_int {
                                memcpy(
                                    copy_dst.offset(16 as c_int as isize)
                                        as *mut c_void,
                                    copy_src.offset(16 as c_int as isize)
                                        as *const c_void,
                                    (i - 16 as c_int) as size_t,
                                );
                            } else {
                                memmove16(
                                    copy_dst.offset(16 as c_int as isize),
                                    copy_src.offset(16 as c_int as isize),
                                );
                            }
                        }
                    }
                    if !((*s).meta_block_remaining_len <= 0 as c_int) {
                        current_block = 0;
                        continue;
                    }
                    (*s).state = BROTLI_STATE_METABLOCK_DONE;
                    break;
                }
                0 => {
                    if safe != 0 {
                        (*s).state = BROTLI_STATE_COMMAND_BEGIN;
                    }
                    if CheckInputAmount(safe, br) == 0 {
                        (*s).state = BROTLI_STATE_COMMAND_BEGIN;
                        result = BROTLI_DECODER_NEEDS_MORE_INPUT;
                        break;
                    } else if ((*s).block_length[1 as c_int as usize] == 0 as uint64_t)
                        as c_int as c_long
                        != 0
                    {
                        let mut status: BrotliDecoderErrorCode = BROTLI_DECODER_NO_ERROR;
                        if safe != 0 {
                            status = SafeDecodeCommandBlockSwitch(s);
                        } else {
                            status = DecodeCommandBlockSwitch(s);
                        }
                        if !(status as c_int
                            != BROTLI_DECODER_SUCCESS as c_int)
                        {
                            current_block = 0;
                            continue;
                        }
                        result = status;
                        break;
                    } else {
                        if safe != 0 {
                            if SafeReadCommand(s, br, &raw mut i) == 0 {
                                result = BROTLI_DECODER_NEEDS_MORE_INPUT;
                                break;
                            }
                        } else {
                            ReadCommand(s, br, &raw mut i);
                        }
                        if i == 0 as c_int {
                            current_block = 2;
                            continue;
                        }
                        (*s).meta_block_remaining_len -= i;
                        current_block = 1;
                    }
                }
                3 => {
                    let mut wrap_guard: c_int = (*s).ringbuffer_size - pos;
                    loop {
                        i -= 1;
                        if !(i >= 0 as c_int) {
                            break;
                        }
                        *(*s).ringbuffer.offset(pos as isize) = *(*s)
                            .ringbuffer
                            .offset((pos - (*s).distance_code & (*s).ringbuffer_mask) as isize);
                        pos += 1;
                        wrap_guard -= 1;
                        if !((wrap_guard == 0 as c_int) as c_int
                            as c_long
                            != 0)
                        {
                            continue;
                        }
                        (*s).state = BROTLI_STATE_COMMAND_POST_WRITE_2;
                        break '_CommandBegin;
                    }
                    if !((*s).meta_block_remaining_len <= 0 as c_int) {
                        current_block = 0;
                        continue;
                    }
                    (*s).state = BROTLI_STATE_METABLOCK_DONE;
                    break;
                }
                _ => {
                    if safe != 0 {
                        (*s).state = BROTLI_STATE_COMMAND_INNER;
                    }
                    if (*s).trivial_literal_context != 0 {
                        let mut bits: uint64_t = 0;
                        let mut value: uint64_t = 0;
                        PreloadSymbol(safe, (*s).literal_htree, br, &raw mut bits, &raw mut value);
                        if safe == 0 {
                            let mut num_steps: c_int = i - 1 as c_int;
                            if num_steps > 0 as c_int
                                && num_steps as uint64_t
                                    > (*s).block_length[0 as c_int as usize]
                            {
                                num_steps = (*s).block_length[0 as c_int as usize]
                                    as c_int;
                            }
                            if (*s).ringbuffer_size >= pos
                                && (*s).ringbuffer_size - pos <= num_steps
                            {
                                num_steps = (*s).ringbuffer_size - pos - 1 as c_int;
                            }
                            if num_steps < 0 as c_int {
                                num_steps = 0 as c_int;
                            }
                            num_steps = BrotliCopyPreloadedSymbolsToU8(
                                (*s).literal_htree,
                                br,
                                &raw mut bits,
                                &raw mut value,
                                (*s).ringbuffer,
                                pos,
                                num_steps,
                            );
                            pos += num_steps;
                            (*s).block_length[0 as c_int as usize] =
                                ((*s).block_length[0 as c_int as usize]
                                    as c_ulong)
                                    .wrapping_sub(num_steps as uint64_t as c_ulong)
                                    as uint64_t as uint64_t;
                            i -= num_steps;
                            loop {
                                if CheckInputAmount(safe, br) == 0 {
                                    (*s).state = BROTLI_STATE_COMMAND_INNER;
                                    result = BROTLI_DECODER_NEEDS_MORE_INPUT;
                                    break '_CommandBegin;
                                } else {
                                    if ((*s).block_length[0 as c_int as usize]
                                        == 0 as uint64_t)
                                        as c_int
                                        as c_long
                                        != 0
                                    {
                                        current_block = 4;
                                        break;
                                    }
                                    BrotliCopyPreloadedSymbolsToU8(
                                        (*s).literal_htree,
                                        br,
                                        &raw mut bits,
                                        &raw mut value,
                                        (*s).ringbuffer,
                                        pos,
                                        1 as c_int,
                                    );
                                    (*s).block_length[0 as c_int as usize] =
                                        (*s).block_length[0 as c_int as usize]
                                            .wrapping_sub(1);
                                    pos += 1;
                                    if (pos == (*s).ringbuffer_size) as c_int
                                        as c_long
                                        != 0
                                    {
                                        (*s).state = BROTLI_STATE_COMMAND_INNER_WRITE;
                                        i -= 1;
                                        break '_CommandBegin;
                                    } else {
                                        i -= 1;
                                        if !(i != 0 as c_int) {
                                            current_block = 5;
                                            break;
                                        }
                                    }
                                }
                            }
                        } else {
                            loop {
                                let mut literal: uint64_t = 0;
                                if ((*s).block_length[0 as c_int as usize]
                                    == 0 as uint64_t)
                                    as c_int
                                    as c_long
                                    != 0
                                {
                                    current_block = 4;
                                    break;
                                }
                                if SafeReadSymbol((*s).literal_htree, br, &raw mut literal) == 0 {
                                    result = BROTLI_DECODER_NEEDS_MORE_INPUT;
                                    break '_CommandBegin;
                                } else {
                                    *(*s).ringbuffer.offset(pos as isize) = literal as uint8_t;
                                    (*s).block_length[0 as c_int as usize] =
                                        (*s).block_length[0 as c_int as usize]
                                            .wrapping_sub(1);
                                    pos += 1;
                                    if (pos == (*s).ringbuffer_size) as c_int
                                        as c_long
                                        != 0
                                    {
                                        (*s).state = BROTLI_STATE_COMMAND_INNER_WRITE;
                                        i -= 1;
                                        break '_CommandBegin;
                                    } else {
                                        i -= 1;
                                        if !(i != 0 as c_int) {
                                            current_block = 5;
                                            break;
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        let mut p1: uint8_t = *(*s).ringbuffer.offset(
                            (pos - 1 as c_int & (*s).ringbuffer_mask) as isize,
                        );
                        let mut p2: uint8_t = *(*s).ringbuffer.offset(
                            (pos - 2 as c_int & (*s).ringbuffer_mask) as isize,
                        );
                        loop {
                            let mut hc: *const HuffmanCode = ::core::ptr::null::<HuffmanCode>();
                            let mut context: uint8_t = 0;
                            if CheckInputAmount(safe, br) == 0 {
                                (*s).state = BROTLI_STATE_COMMAND_INNER;
                                result = BROTLI_DECODER_NEEDS_MORE_INPUT;
                                break '_CommandBegin;
                            } else {
                                if ((*s).block_length[0 as c_int as usize]
                                    == 0 as uint64_t)
                                    as c_int
                                    as c_long
                                    != 0
                                {
                                    current_block = 4;
                                    break;
                                }
                                context = (*(*s).context_lookup.offset(p1 as isize)
                                    as c_int
                                    | *(*s)
                                        .context_lookup
                                        .offset(256 as c_int as isize)
                                        .offset(p2 as isize)
                                        as c_int)
                                    as uint8_t;
                                hc =
                                    *(*s)
                                        .literal_hgroup
                                        .htrees
                                        .offset(*(*s).context_map_slice.offset(context as isize)
                                            as isize);
                                p2 = p1;
                                if safe == 0 {
                                    p1 = ReadSymbol(hc, br) as uint8_t;
                                } else {
                                    let mut literal_0: uint64_t = 0;
                                    if SafeReadSymbol(hc, br, &raw mut literal_0) == 0 {
                                        result = BROTLI_DECODER_NEEDS_MORE_INPUT;
                                        break '_CommandBegin;
                                    } else {
                                        p1 = literal_0 as uint8_t;
                                    }
                                }
                                *(*s).ringbuffer.offset(pos as isize) = p1;
                                (*s).block_length[0 as c_int as usize] =
                                    (*s).block_length[0 as c_int as usize]
                                        .wrapping_sub(1);
                                pos += 1;
                                if (pos == (*s).ringbuffer_size) as c_int
                                    as c_long
                                    != 0
                                {
                                    (*s).state = BROTLI_STATE_COMMAND_INNER_WRITE;
                                    i -= 1;
                                    break '_CommandBegin;
                                } else {
                                    i -= 1;
                                    if !(i != 0 as c_int) {
                                        current_block = 5;
                                        break;
                                    }
                                }
                            }
                        }
                    }
                    match current_block {
                        4 => {
                            let mut status_1: BrotliDecoderErrorCode = BROTLI_DECODER_NO_ERROR;
                            if safe != 0 {
                                status_1 = SafeDecodeLiteralBlockSwitch(s);
                            } else {
                                status_1 = DecodeLiteralBlockSwitch(s);
                            }
                            if !(status_1 as c_int
                                != BROTLI_DECODER_SUCCESS as c_int)
                            {
                                current_block = 1;
                                continue;
                            }
                            result = status_1;
                            break;
                        }
                        _ => {
                            if !(((*s).meta_block_remaining_len <= 0 as c_int)
                                as c_int
                                as c_long
                                != 0)
                            {
                                current_block = 2;
                                continue;
                            }
                            (*s).state = BROTLI_STATE_METABLOCK_DONE;
                            break;
                        }
                    }
                }
            }
        }
    }
    (*s).pos = pos;
    (*s).loop_counter = i;
    return result;
}
#[inline(never)]
unsafe fn ProcessCommands(
    mut s: *mut BrotliDecoderStateInternal,
) -> BrotliDecoderErrorCode {
    return ProcessCommandsInternal(0 as c_int, s);
}
#[inline(never)]
unsafe fn SafeProcessCommands(
    mut s: *mut BrotliDecoderStateInternal,
) -> BrotliDecoderErrorCode {
    return ProcessCommandsInternal(1 as c_int, s);
}
#[inline]
pub unsafe fn BrotliDecoderDecompress(
    mut encoded_size: size_t,
    mut encoded_buffer: *const uint8_t,
    mut decoded_size: *mut size_t,
    mut decoded_buffer: *mut uint8_t,
) -> BrotliDecoderResult {
    let mut s: BrotliDecoderStateInternal = BrotliDecoderStateInternal {
        state: BROTLI_STATE_UNINITED,
        loop_counter: 0,
        br: BrotliBitReader {
            val_: 0,
            bit_pos_: 0,
            next_in: ::core::ptr::null::<uint8_t>(),
            guard_in: ::core::ptr::null::<uint8_t>(),
            last_in: ::core::ptr::null::<uint8_t>(),
        },
        alloc_func: None,
        free_func: None,
        memory_manager_opaque: ::core::ptr::null_mut::<c_void>(),
        buffer: C2RustUnnamed_hu0150709a { u64_0: 0 },
        buffer_length: 0,
        pos: 0,
        max_backward_distance: 0,
        max_distance: 0,
        ringbuffer_size: 0,
        ringbuffer_mask: 0,
        dist_rb_idx: 0,
        dist_rb: [0; 4],
        error_code: 0,
        meta_block_remaining_len: 0,
        ringbuffer: ::core::ptr::null_mut::<uint8_t>(),
        ringbuffer_end: ::core::ptr::null_mut::<uint8_t>(),
        htree_command: ::core::ptr::null_mut::<HuffmanCode>(),
        context_lookup: ::core::ptr::null::<uint8_t>(),
        context_map_slice: ::core::ptr::null_mut::<uint8_t>(),
        dist_context_map_slice: ::core::ptr::null_mut::<uint8_t>(),
        literal_hgroup: HuffmanTreeGroup {
            htrees: ::core::ptr::null_mut::<*mut HuffmanCode>(),
            codes: ::core::ptr::null_mut::<HuffmanCode>(),
            alphabet_size_max: 0,
            alphabet_size_limit: 0,
            num_htrees: 0,
        },
        insert_copy_hgroup: HuffmanTreeGroup {
            htrees: ::core::ptr::null_mut::<*mut HuffmanCode>(),
            codes: ::core::ptr::null_mut::<HuffmanCode>(),
            alphabet_size_max: 0,
            alphabet_size_limit: 0,
            num_htrees: 0,
        },
        distance_hgroup: HuffmanTreeGroup {
            htrees: ::core::ptr::null_mut::<*mut HuffmanCode>(),
            codes: ::core::ptr::null_mut::<HuffmanCode>(),
            alphabet_size_max: 0,
            alphabet_size_limit: 0,
            num_htrees: 0,
        },
        block_type_trees: ::core::ptr::null_mut::<HuffmanCode>(),
        block_len_trees: ::core::ptr::null_mut::<HuffmanCode>(),
        trivial_literal_context: 0,
        distance_context: 0,
        block_length: [0; 3],
        block_length_index: 0,
        num_block_types: [0; 3],
        block_type_rb: [0; 6],
        distance_postfix_bits: 0,
        num_direct_distance_codes: 0,
        num_dist_htrees: 0,
        dist_context_map: ::core::ptr::null_mut::<uint8_t>(),
        literal_htree: ::core::ptr::null_mut::<HuffmanCode>(),
        rb_roundtrips: 0,
        partial_pos_out: 0,
        mtf_upper_bound: 0,
        mtf: [0; 65],
        copy_length: 0,
        distance_code: 0,
        dist_htree_index: 0,
        metadata_start_func: None,
        metadata_chunk_func: None,
        metadata_callback_opaque: ::core::ptr::null_mut::<c_void>(),
        used_input: 0,
        substate_metablock_header: BROTLI_STATE_METABLOCK_HEADER_NONE,
        substate_uncompressed: BROTLI_STATE_UNCOMPRESSED_NONE,
        substate_decode_uint8: BROTLI_STATE_DECODE_UINT8_NONE,
        substate_read_block_length: BROTLI_STATE_READ_BLOCK_LENGTH_NONE,
        new_ringbuffer_size: 0,
        is_last_metablock_is_uncompressed_is_metadata_should_wrap_ringbuffer_canny_ringbuffer_allocation_large_window_window_bits_size_nibbles: [0; 3],
        c2rust_padding: [0; 1],
        num_literal_htrees: 0,
        context_map: ::core::ptr::null_mut::<uint8_t>(),
        context_modes: ::core::ptr::null_mut::<uint8_t>(),
        dictionary: ::core::ptr::null_mut::<BrotliSharedDictionaryInternal>(),
        compound_dictionary: ::core::ptr::null_mut::<BrotliDecoderCompoundDictionary>(),
        trivial_literal_contexts: [0; 8],
        arena: C2RustUnnamed_hu94e3e3d6 {
            header: BrotliMetablockHeaderArena {
                substate_tree_group: BROTLI_STATE_TREE_GROUP_NONE,
                substate_context_map: BROTLI_STATE_CONTEXT_MAP_NONE,
                substate_huffman: BROTLI_STATE_HUFFMAN_NONE,
                sub_loop_counter: 0,
                repeat_code_len: 0,
                prev_code_len: 0,
                symbol: 0,
                repeat: 0,
                space: 0,
                table: [HuffmanCode { bits: 0, value: 0 }; 32],
                symbol_lists: ::core::ptr::null_mut::<uint16_t>(),
                symbols_lists_array: [0; 720],
                next_symbol: [0; 32],
                code_length_code_lengths: [0; 18],
                code_length_histo: [0; 16],
                htree_index: 0,
                next: ::core::ptr::null_mut::<HuffmanCode>(),
                context_index: 0,
                max_run_length_prefix: 0,
                code: 0,
                context_map_table: [HuffmanCode { bits: 0, value: 0 }; 646],
            },
        },
    };
    let mut result: BrotliDecoderResult = BROTLI_DECODER_RESULT_ERROR;
    let mut total_out: size_t = 0 as size_t;
    let mut available_in: size_t = encoded_size;
    let mut next_in: *const uint8_t = encoded_buffer as *const uint8_t;
    let mut available_out: size_t = *decoded_size;
    let mut next_out: *mut uint8_t = decoded_buffer as *mut uint8_t;
    if BrotliDecoderStateInit(
        &raw mut s,
        None,
        None,
        ::core::ptr::null_mut::<c_void>(),
    ) == 0
    {
        return BROTLI_DECODER_RESULT_ERROR;
    }
    result = BrotliDecoderDecompressStream(
        &raw mut s,
        &raw mut available_in,
        &raw mut next_in,
        &raw mut available_out,
        &raw mut next_out,
        &raw mut total_out,
    );
    *decoded_size = total_out;
    BrotliDecoderStateCleanup(&raw mut s);
    if result as c_uint
        != BROTLI_DECODER_RESULT_SUCCESS as c_int as c_uint
    {
        result = BROTLI_DECODER_RESULT_ERROR;
    }
    return result;
}
pub unsafe fn BrotliDecoderDecompressStream(
    mut s: *mut BrotliDecoderStateInternal,
    mut available_in: *mut size_t,
    mut next_in: *mut *const uint8_t,
    mut available_out: *mut size_t,
    mut next_out: *mut *mut uint8_t,
    mut total_out: *mut size_t,
) -> BrotliDecoderResult {
    let available_in_view: &mut size_t = unsafe { &mut *available_in };
    let mut result: BrotliDecoderErrorCode = BROTLI_DECODER_SUCCESS;
    let mut br: *mut BrotliBitReader = &raw mut (*s).br;
    let mut input_size: size_t = *available_in_view;
    if !total_out.is_null() {
        *total_out = (*s).partial_pos_out;
    }
    if (*s).error_code < 0 as c_int {
        return BROTLI_DECODER_RESULT_ERROR;
    }
    if *available_out != 0 && (next_out.is_null() || (*next_out).is_null()) {
        return SaveErrorCode(
            s,
            BROTLI_DECODER_ERROR_INVALID_ARGUMENTS as c_int as BrotliDecoderErrorCode,
            input_size.wrapping_sub(*available_in_view),
        );
    }
    if *available_out == 0 {
        next_out = ::core::ptr::null_mut::<*mut uint8_t>();
    }
    if (*s).buffer_length == 0 as uint64_t {
        BrotliBitReaderSetInput(br, *next_in, *available_in_view);
    } else {
        result = BROTLI_DECODER_NEEDS_MORE_INPUT;
        BrotliBitReaderSetInput(
            br,
            (&raw mut (*s).buffer.u8_0 as *mut uint8_t).offset(0 as c_int as isize)
                as *mut uint8_t,
            (*s).buffer_length as size_t,
        );
    }
    let mut current_block_177: u64;
    loop {
        if result as c_int != BROTLI_DECODER_SUCCESS as c_int {
            if result as c_int == BROTLI_DECODER_NEEDS_MORE_INPUT as c_int
            {
                if !(*s).ringbuffer.is_null() {
                    let mut intermediate_result: BrotliDecoderErrorCode =
                        WriteRingBuffer(s, available_out, next_out, total_out, BROTLI_TRUE);
                    if (intermediate_result as c_int) < 0 as c_int {
                        result = intermediate_result;
                        break;
                    }
                }
                if (*s).buffer_length != 0 as uint64_t {
                    if (*br).next_in == (*br).last_in {
                        (*s).buffer_length = 0 as uint64_t;
                        result = BROTLI_DECODER_SUCCESS;
                        BrotliBitReaderSetInput(br, *next_in, *available_in_view);
                    } else {
                        if !(*available_in_view != 0 as size_t) {
                            break;
                        }
                        result = BROTLI_DECODER_SUCCESS;
                        (*s).buffer.u8_0[(*s).buffer_length as usize] = **next_in;
                        (*s).buffer_length = (*s).buffer_length.wrapping_add(1);
                        BrotliBitReaderSetInput(
                            br,
                            (&raw mut (*s).buffer.u8_0 as *mut uint8_t)
                                .offset(0 as c_int as isize)
                                as *mut uint8_t,
                            (*s).buffer_length as size_t,
                        );
                        *next_in = (*next_in).offset(1);
                        *available_in_view = available_in_view.wrapping_sub(1);
                    }
                } else {
                    *next_in = (*br).next_in;
                    *available_in_view = BrotliBitReaderGetAvailIn(br);
                    while *available_in_view != 0 {
                        (*s).buffer.u8_0[(*s).buffer_length as usize] = **next_in;
                        (*s).buffer_length = (*s).buffer_length.wrapping_add(1);
                        *next_in = (*next_in).offset(1);
                        *available_in_view = available_in_view.wrapping_sub(1);
                    }
                    break;
                }
            } else {
                if (*s).buffer_length != 0 as uint64_t {
                    (*s).buffer_length = 0 as uint64_t;
                } else {
                    BrotliBitReaderUnload(br);
                    *available_in_view = BrotliBitReaderGetAvailIn(br);
                    *next_in = (*br).next_in;
                }
                break;
            }
        } else {
            match (*s).state as c_uint {
                0 => {
                    if BrotliWarmupBitReader(br) == 0 {
                        result = BROTLI_DECODER_NEEDS_MORE_INPUT;
                        continue;
                    } else {
                        result = DecodeWindowBits(s, br);
                        if result as c_int
                            != BROTLI_DECODER_SUCCESS as c_int
                        {
                            continue;
                        }
                        if (*s).large_window() != 0 {
                            (*s).state = BROTLI_STATE_LARGE_WINDOW_BITS;
                            continue;
                        } else {
                            (*s).state = BROTLI_STATE_INITIALIZE;
                            continue;
                        }
                    }
                }
                1 => {
                    let mut bits: uint64_t = 0;
                    if BrotliSafeReadBits(br, 6 as uint64_t, &raw mut bits) == 0 {
                        result = BROTLI_DECODER_NEEDS_MORE_INPUT;
                        continue;
                    } else {
                        (*s).set_window_bits(
                            (bits & 63 as uint64_t) as c_uint as c_uint,
                        );
                        if ((*s).window_bits() as c_int) < BROTLI_LARGE_MIN_WBITS
                            || (*s).window_bits() as c_int > BROTLI_LARGE_MAX_WBITS
                        {
                            result = BROTLI_DECODER_ERROR_FORMAT_WINDOW_BITS as c_int
                                as BrotliDecoderErrorCode;
                            continue;
                        } else {
                            (*s).state = BROTLI_STATE_INITIALIZE;
                        }
                    }
                    current_block_177 = 168769493162332264;
                }
                2 => {
                    current_block_177 = 168769493162332264;
                }
                3 => {
                    current_block_177 = 9378052560089318616;
                }
                4 => {
                    current_block_177 = 15277662988523915720;
                }
                17 => {
                    current_block_177 = 11894620841426922139;
                }
                18 => {
                    current_block_177 = 4712728041281995262;
                }
                19 => {
                    current_block_177 = 14865402277128115059;
                }
                20 => {
                    current_block_177 = 722119776535234387;
                }
                21 => {
                    current_block_177 = 4804377075063615140;
                }
                11 => {
                    result = CopyUncompressedBlockToOutput(available_out, next_out, total_out, s);
                    if result as c_int != BROTLI_DECODER_SUCCESS as c_int
                    {
                        continue;
                    }
                    (*s).state = BROTLI_STATE_METABLOCK_DONE;
                    continue;
                }
                12 => {
                    result = SkipMetadataBlock(s);
                    if result as c_int != BROTLI_DECODER_SUCCESS as c_int
                    {
                        continue;
                    }
                    (*s).state = BROTLI_STATE_METABLOCK_DONE;
                    continue;
                }
                5 => {
                    let mut bits_0: uint64_t = 0;
                    if BrotliSafeReadBits(br, 6 as uint64_t, &raw mut bits_0) == 0 {
                        result = BROTLI_DECODER_NEEDS_MORE_INPUT;
                        continue;
                    } else {
                        (*s).distance_postfix_bits = bits_0 & BitMask(2 as uint64_t);
                        bits_0 >>= 2 as c_int;
                        (*s).num_direct_distance_codes = bits_0 << (*s).distance_postfix_bits;
                        (*s).context_modes = (*s).alloc_func.expect("non-null function pointer")(
                            (*s).memory_manager_opaque,
                            (*s).num_block_types[0 as c_int as usize] as size_t,
                        ) as *mut uint8_t;
                        if (*s).context_modes.is_null() {
                            result = BROTLI_DECODER_ERROR_ALLOC_CONTEXT_MODES as c_int
                                as BrotliDecoderErrorCode;
                            continue;
                        } else {
                            (*s).loop_counter = 0 as c_int;
                            (*s).state = BROTLI_STATE_CONTEXT_MODES;
                        }
                    }
                    current_block_177 = 6843233061376031846;
                }
                6 => {
                    current_block_177 = 6843233061376031846;
                }
                22 => {
                    current_block_177 = 3513440520300748193;
                }
                23 => {
                    current_block_177 = 13425230902034816933;
                }
                24 => {
                    current_block_177 = 13895078145312174667;
                }
                25 => {
                    current_block_177 = 8248800716151944504;
                }
                7 => {
                    current_block_177 = 17953379472903764582;
                }
                8 => {
                    current_block_177 = 15288402183126507535;
                }
                9 | 10 => {
                    current_block_177 = 4882717807625051864;
                }
                13 => {
                    current_block_177 = 10423162298935280580;
                }
                15 | 16 => {
                    current_block_177 = 10423162298935280580;
                }
                14 => {
                    if (*s).meta_block_remaining_len < 0 as c_int {
                        result = BROTLI_DECODER_ERROR_FORMAT_BLOCK_LENGTH_2 as c_int
                            as BrotliDecoderErrorCode;
                        continue;
                    } else {
                        BrotliDecoderStateCleanupAfterMetablock(s);
                        if (*s).is_last_metablock() == 0 {
                            (*s).state = BROTLI_STATE_METABLOCK_BEGIN;
                            continue;
                        } else if BrotliJumpToByteBoundary(br) == 0 {
                            result = BROTLI_DECODER_ERROR_FORMAT_PADDING_2 as c_int
                                as BrotliDecoderErrorCode;
                            continue;
                        } else {
                            if (*s).buffer_length == 0 as uint64_t {
                                BrotliBitReaderUnload(br);
                                *available_in_view = BrotliBitReaderGetAvailIn(br);
                                *next_in = (*br).next_in;
                            }
                            (*s).state = BROTLI_STATE_DONE;
                        }
                    }
                    current_block_177 = 5602626330413423453;
                }
                26 => {
                    current_block_177 = 5602626330413423453;
                }
                _ => {
                    continue;
                }
            }
            match current_block_177 {
                5602626330413423453 => {
                    if !(*s).ringbuffer.is_null() {
                        result =
                            WriteRingBuffer(s, available_out, next_out, total_out, BROTLI_TRUE);
                        if result as c_int
                            != BROTLI_DECODER_SUCCESS as c_int
                        {
                            continue;
                        }
                    }
                    return SaveErrorCode(s, result, input_size.wrapping_sub(*available_in_view));
                }
                10423162298935280580 => {
                    result = WriteRingBuffer(s, available_out, next_out, total_out, BROTLI_FALSE);
                    if result as c_int != BROTLI_DECODER_SUCCESS as c_int
                    {
                        continue;
                    }
                    WrapRingBuffer(s);
                    if (*s).ringbuffer_size
                        == (1 as c_int) << (*s).window_bits() as c_int
                    {
                        (*s).max_distance = (*s).max_backward_distance;
                    }
                    if (*s).state as c_uint
                        == BROTLI_STATE_COMMAND_POST_WRITE_1 as c_int
                            as c_uint
                    {
                        let mut addon: *mut BrotliDecoderCompoundDictionary =
                            (*s).compound_dictionary;
                        if !addon.is_null() && (*addon).br_length != (*addon).br_copied {
                            (*s).pos += CopyFromCompoundDictionary(s, (*s).pos);
                            if (*s).pos >= (*s).ringbuffer_size {
                                continue;
                            }
                        }
                        if (*s).meta_block_remaining_len == 0 as c_int {
                            (*s).state = BROTLI_STATE_METABLOCK_DONE;
                        } else {
                            (*s).state = BROTLI_STATE_COMMAND_BEGIN;
                        }
                        continue;
                    } else if (*s).state as c_uint
                        == BROTLI_STATE_COMMAND_POST_WRITE_2 as c_int
                            as c_uint
                    {
                        (*s).state = BROTLI_STATE_COMMAND_POST_WRAP_COPY;
                        continue;
                    } else if (*s).loop_counter == 0 as c_int {
                        if (*s).meta_block_remaining_len == 0 as c_int {
                            (*s).state = BROTLI_STATE_METABLOCK_DONE;
                        } else {
                            (*s).state = BROTLI_STATE_COMMAND_POST_DECODE_LITERALS;
                        }
                        continue;
                    } else {
                        (*s).state = BROTLI_STATE_COMMAND_INNER;
                        continue;
                    }
                }
                6843233061376031846 => {
                    result = ReadContextModes(s);
                    if result as c_int != BROTLI_DECODER_SUCCESS as c_int
                    {
                        continue;
                    }
                    (*s).state = BROTLI_STATE_CONTEXT_MAP_1;
                    current_block_177 = 3513440520300748193;
                }
                168769493162332264 => {
                    (*s).max_backward_distance = ((1 as c_int)
                        << (*s).window_bits() as c_int)
                        - BROTLI_WINDOW_GAP;
                    (*s).block_type_trees = (*s).alloc_func.expect("non-null function pointer")(
                        (*s).memory_manager_opaque,
                        (::core::mem::size_of::<HuffmanCode>() as size_t)
                            .wrapping_mul(3 as size_t)
                            .wrapping_mul(
                                (632 as c_int + 396 as c_int) as size_t,
                            ),
                    ) as *mut HuffmanCode;
                    if (*s).block_type_trees.is_null() {
                        result = BROTLI_DECODER_ERROR_ALLOC_BLOCK_TYPE_TREES as c_int
                            as BrotliDecoderErrorCode;
                        continue;
                    } else {
                        (*s).block_len_trees = (*s).block_type_trees.offset(
                            (3 as c_int * BROTLI_HUFFMAN_MAX_SIZE_258) as isize,
                        );
                        (*s).state = BROTLI_STATE_METABLOCK_BEGIN;
                    }
                    current_block_177 = 9378052560089318616;
                }
                _ => {}
            }
            match current_block_177 {
                3513440520300748193 => {
                    result = DecodeContextMap(
                        (*s).num_block_types[0 as c_int as usize]
                            << BROTLI_LITERAL_CONTEXT_BITS,
                        &raw mut (*s).num_literal_htrees,
                        &raw mut (*s).context_map,
                        s,
                    );
                    if result as c_int != BROTLI_DECODER_SUCCESS as c_int
                    {
                        continue;
                    }
                    DetectTrivialLiteralBlockTypes(s);
                    (*s).state = BROTLI_STATE_CONTEXT_MAP_2;
                    current_block_177 = 13425230902034816933;
                }
                9378052560089318616 => {
                    BrotliDecoderStateMetablockBegin(s);
                    (*s).state = BROTLI_STATE_METABLOCK_HEADER;
                    current_block_177 = 15277662988523915720;
                }
                _ => {}
            }
            match current_block_177 {
                13425230902034816933 => {
                    let mut npostfix: uint64_t = (*s).distance_postfix_bits;
                    let mut ndirect: uint64_t = (*s).num_direct_distance_codes;
                    let mut distance_alphabet_size_max: uint64_t = (BROTLI_NUM_DISTANCE_SHORT_CODES
                        as uint64_t)
                        .wrapping_add(ndirect)
                        .wrapping_add(
                            ((24 as c_uint) << npostfix.wrapping_add(1 as uint64_t))
                                as uint64_t,
                        );
                    let mut distance_alphabet_size_limit: uint64_t = distance_alphabet_size_max;
                    let mut allocation_success: c_int = BROTLI_TRUE;
                    if (*s).large_window() != 0 {
                        let mut limit: BrotliDistanceCodeLimit = BrotliCalculateDistanceCodeLimit(
                            BROTLI_MAX_ALLOWED_DISTANCE as uint32_t,
                            npostfix as uint32_t,
                            ndirect as uint32_t,
                        );
                        distance_alphabet_size_max = (BROTLI_NUM_DISTANCE_SHORT_CODES as uint64_t)
                            .wrapping_add(ndirect)
                            .wrapping_add(
                                ((62 as c_uint)
                                    << npostfix.wrapping_add(1 as uint64_t))
                                    as uint64_t,
                            );
                        distance_alphabet_size_limit = limit.max_alphabet_size as uint64_t;
                    }
                    result = DecodeContextMap(
                        (*s).num_block_types[2 as c_int as usize]
                            << BROTLI_DISTANCE_CONTEXT_BITS,
                        &raw mut (*s).num_dist_htrees,
                        &raw mut (*s).dist_context_map,
                        s,
                    );
                    if result as c_int != BROTLI_DECODER_SUCCESS as c_int
                    {
                        continue;
                    }
                    allocation_success &= BrotliDecoderHuffmanTreeGroupInit(
                        s,
                        &raw mut (*s).literal_hgroup,
                        BROTLI_NUM_LITERAL_SYMBOLS as uint64_t,
                        BROTLI_NUM_LITERAL_SYMBOLS as uint64_t,
                        (*s).num_literal_htrees,
                    );
                    allocation_success &= BrotliDecoderHuffmanTreeGroupInit(
                        s,
                        &raw mut (*s).insert_copy_hgroup,
                        BROTLI_NUM_COMMAND_SYMBOLS as uint64_t,
                        BROTLI_NUM_COMMAND_SYMBOLS as uint64_t,
                        (*s).num_block_types[1 as c_int as usize],
                    );
                    allocation_success &= BrotliDecoderHuffmanTreeGroupInit(
                        s,
                        &raw mut (*s).distance_hgroup,
                        distance_alphabet_size_max,
                        distance_alphabet_size_limit,
                        (*s).num_dist_htrees,
                    );
                    if allocation_success == 0 {
                        return SaveErrorCode(
                            s,
                            BROTLI_DECODER_ERROR_ALLOC_TREE_GROUPS as c_int
                                as BrotliDecoderErrorCode,
                            input_size.wrapping_sub(*available_in_view),
                        );
                    }
                    (*s).loop_counter = 0 as c_int;
                    (*s).state = BROTLI_STATE_TREE_GROUP;
                    current_block_177 = 13895078145312174667;
                }
                15277662988523915720 => {
                    result = DecodeMetaBlockLength(s, br);
                    if result as c_int != BROTLI_DECODER_SUCCESS as c_int
                    {
                        continue;
                    }
                    if (*s).is_metadata() as c_int != 0
                        || (*s).is_uncompressed() as c_int != 0
                    {
                        if BrotliJumpToByteBoundary(br) == 0 {
                            result = BROTLI_DECODER_ERROR_FORMAT_PADDING_1 as c_int
                                as BrotliDecoderErrorCode;
                            continue;
                        }
                    }
                    if (*s).is_metadata() != 0 {
                        (*s).state = BROTLI_STATE_METADATA;
                        if (*s).metadata_start_func.is_some() {
                            (*s).metadata_start_func.expect("non-null function pointer")(
                                (*s).metadata_callback_opaque,
                                (*s).meta_block_remaining_len as size_t,
                            );
                        }
                        continue;
                    } else if (*s).meta_block_remaining_len == 0 as c_int {
                        (*s).state = BROTLI_STATE_METABLOCK_DONE;
                        continue;
                    } else {
                        BrotliCalculateRingBufferSize(s);
                        if (*s).is_uncompressed() != 0 {
                            (*s).state = BROTLI_STATE_UNCOMPRESSED;
                            continue;
                        } else {
                            (*s).state = BROTLI_STATE_BEFORE_COMPRESSED_METABLOCK_HEADER;
                        }
                    }
                    current_block_177 = 11894620841426922139;
                }
                _ => {}
            }
            match current_block_177 {
                13895078145312174667 => {
                    let mut hgroup: *mut HuffmanTreeGroup =
                        ::core::ptr::null_mut::<HuffmanTreeGroup>();
                    match (*s).loop_counter {
                        0 => {
                            hgroup = &raw mut (*s).literal_hgroup;
                        }
                        1 => {
                            hgroup = &raw mut (*s).insert_copy_hgroup;
                        }
                        2 => {
                            hgroup = &raw mut (*s).distance_hgroup;
                        }
                        _ => {
                            return SaveErrorCode(
                                s,
                                BROTLI_DECODER_ERROR_UNREACHABLE as c_int
                                    as BrotliDecoderErrorCode,
                                input_size.wrapping_sub(*available_in_view),
                            );
                        }
                    }
                    result = HuffmanTreeGroupDecode(hgroup, s);
                    if result as c_int != BROTLI_DECODER_SUCCESS as c_int
                    {
                        continue;
                    }
                    (*s).loop_counter += 1;
                    if (*s).loop_counter < 3 as c_int {
                        continue;
                    }
                    (*s).state = BROTLI_STATE_BEFORE_COMPRESSED_METABLOCK_BODY;
                    current_block_177 = 8248800716151944504;
                }
                11894620841426922139 => {
                    let mut h: *mut BrotliMetablockHeaderArena = &raw mut (*s).arena.header;
                    (*s).loop_counter = 0 as c_int;
                    (*h).sub_loop_counter = 0 as uint64_t;
                    (*h).symbol_lists = (&raw mut (*h).symbols_lists_array as *mut uint16_t)
                        .offset((BROTLI_HUFFMAN_MAX_CODE_LENGTH + 1 as c_int) as isize)
                        as *mut uint16_t;
                    (*h).substate_huffman = BROTLI_STATE_HUFFMAN_NONE;
                    (*h).substate_tree_group = BROTLI_STATE_TREE_GROUP_NONE;
                    (*h).substate_context_map = BROTLI_STATE_CONTEXT_MAP_NONE;
                    (*s).state = BROTLI_STATE_HUFFMAN_CODE_0;
                    current_block_177 = 4712728041281995262;
                }
                _ => {}
            }
            match current_block_177 {
                8248800716151944504 => {
                    PrepareLiteralDecoding(s);
                    (*s).dist_context_map_slice = (*s).dist_context_map;
                    (*s).htree_command = *(*s)
                        .insert_copy_hgroup
                        .htrees
                        .offset(0 as c_int as isize);
                    if BrotliEnsureRingBuffer(s) == 0 {
                        result = BROTLI_DECODER_ERROR_ALLOC_RING_BUFFER_2 as c_int
                            as BrotliDecoderErrorCode;
                        continue;
                    } else {
                        CalculateDistanceLut(s);
                        (*s).state = BROTLI_STATE_COMMAND_BEGIN;
                    }
                    current_block_177 = 17953379472903764582;
                }
                4712728041281995262 => {
                    if (*s).loop_counter >= 3 as c_int {
                        (*s).state = BROTLI_STATE_METABLOCK_HEADER_2;
                        continue;
                    } else {
                        result = DecodeVarLenUint8(
                            s,
                            br,
                            (&raw mut (*s).num_block_types as *mut uint64_t)
                                .offset((*s).loop_counter as isize)
                                as *mut uint64_t,
                        );
                        if result as c_int
                            != BROTLI_DECODER_SUCCESS as c_int
                        {
                            continue;
                        }
                        (*s).num_block_types[(*s).loop_counter as usize] =
                            (*s).num_block_types[(*s).loop_counter as usize].wrapping_add(1);
                        if (*s).num_block_types[(*s).loop_counter as usize] < 2 as uint64_t {
                            (*s).loop_counter += 1;
                            continue;
                        } else {
                            (*s).state = BROTLI_STATE_HUFFMAN_CODE_1;
                        }
                    }
                    current_block_177 = 14865402277128115059;
                }
                _ => {}
            }
            match current_block_177 {
                14865402277128115059 => {
                    let mut alphabet_size: uint64_t = (*s).num_block_types
                        [(*s).loop_counter as usize]
                        .wrapping_add(2 as uint64_t);
                    let mut tree_offset: c_int =
                        (*s).loop_counter * BROTLI_HUFFMAN_MAX_SIZE_258;
                    result = ReadHuffmanCode(
                        alphabet_size,
                        alphabet_size,
                        (*s).block_type_trees.offset(tree_offset as isize) as *mut HuffmanCode,
                        ::core::ptr::null_mut::<uint64_t>(),
                        s,
                    );
                    if result as c_int != BROTLI_DECODER_SUCCESS as c_int
                    {
                        continue;
                    }
                    (*s).state = BROTLI_STATE_HUFFMAN_CODE_2;
                    current_block_177 = 722119776535234387;
                }
                17953379472903764582 => {
                    current_block_177 = 15288402183126507535;
                }
                _ => {}
            }
            match current_block_177 {
                722119776535234387 => {
                    let mut alphabet_size_0: uint64_t = BROTLI_NUM_BLOCK_LEN_SYMBOLS as uint64_t;
                    let mut tree_offset_0: c_int =
                        (*s).loop_counter * BROTLI_HUFFMAN_MAX_SIZE_26;
                    result = ReadHuffmanCode(
                        alphabet_size_0,
                        alphabet_size_0,
                        (*s).block_len_trees.offset(tree_offset_0 as isize) as *mut HuffmanCode,
                        ::core::ptr::null_mut::<uint64_t>(),
                        s,
                    );
                    if result as c_int != BROTLI_DECODER_SUCCESS as c_int
                    {
                        continue;
                    }
                    (*s).state = BROTLI_STATE_HUFFMAN_CODE_3;
                    current_block_177 = 4804377075063615140;
                }
                15288402183126507535 => {
                    current_block_177 = 4882717807625051864;
                }
                _ => {}
            }
            match current_block_177 {
                4804377075063615140 => {
                    let mut tree_offset_1: c_int =
                        (*s).loop_counter * BROTLI_HUFFMAN_MAX_SIZE_26;
                    if SafeReadBlockLength(
                        s,
                        (&raw mut (*s).block_length as *mut uint64_t)
                            .offset((*s).loop_counter as isize)
                            as *mut uint64_t,
                        (*s).block_len_trees.offset(tree_offset_1 as isize) as *mut HuffmanCode,
                        br,
                    ) == 0
                    {
                        result = BROTLI_DECODER_NEEDS_MORE_INPUT;
                    } else {
                        (*s).loop_counter += 1;
                        (*s).state = BROTLI_STATE_HUFFMAN_CODE_0;
                    }
                }
                _ => {
                    result = ProcessCommands(s);
                    if result as c_int
                        == BROTLI_DECODER_NEEDS_MORE_INPUT as c_int
                    {
                        result = SafeProcessCommands(s);
                    }
                }
            }
        }
    }
    return SaveErrorCode(s, result, input_size.wrapping_sub(*available_in_view));
}
#[inline]
pub unsafe fn BrotliDecoderHasMoreOutput(
    mut s: *const BrotliDecoderStateInternal,
) -> c_int {
    let s_view: &BrotliDecoderStateInternal = unsafe { &*s };
    if s_view.error_code < 0 as c_int {
        return BROTLI_FALSE;
    }
    return if !s_view.ringbuffer.is_null()
        && UnwrittenBytes(s, 0 as c_int) != 0 as size_t
    {
        BROTLI_TRUE
    } else {
        BROTLI_FALSE
    };
}
#[inline]
pub unsafe fn BrotliDecoderTakeOutput(
    mut s: *mut BrotliDecoderStateInternal,
    mut size: *mut size_t,
) -> *const uint8_t {
    let mut result: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut available_out: size_t = if *size != 0 {
        *size
    } else {
        ((1 as c_uint) << 24 as c_int) as size_t
    };
    let mut requested_out: size_t = available_out;
    let mut status: BrotliDecoderErrorCode = BROTLI_DECODER_NO_ERROR;
    if (*s).ringbuffer.is_null() || (*s).error_code < 0 as c_int {
        *size = 0 as size_t;
        return ::core::ptr::null::<uint8_t>();
    }
    WrapRingBuffer(s);
    status = WriteRingBuffer(
        s,
        &raw mut available_out,
        &raw mut result,
        ::core::ptr::null_mut::<size_t>(),
        BROTLI_TRUE,
    );
    if status as c_int == BROTLI_DECODER_SUCCESS as c_int
        || status as c_int == BROTLI_DECODER_NEEDS_MORE_OUTPUT as c_int
    {
        *size = requested_out.wrapping_sub(available_out);
    } else {
        if (status as c_int) < 0 as c_int {
            SaveErrorCode(s, status, 0 as size_t);
        }
        *size = 0 as size_t;
        result = ::core::ptr::null_mut::<uint8_t>();
    }
    return result;
}
#[inline]
pub unsafe fn BrotliDecoderIsUsed(
    mut s: *const BrotliDecoderStateInternal,
) -> c_int {
    let s_view: &BrotliDecoderStateInternal = unsafe { &*s };
    return if s_view.state as c_uint
        != BROTLI_STATE_UNINITED as c_int as c_uint
        || BrotliGetAvailableBits(&raw const s_view.br) != 0 as uint64_t
    {
        BROTLI_TRUE
    } else {
        BROTLI_FALSE
    };
}
#[inline]
pub unsafe fn BrotliDecoderIsFinished(
    mut s: *const BrotliDecoderStateInternal,
) -> c_int {
    let s_view: &BrotliDecoderStateInternal = unsafe { &*s };
    return ((if s_view.state as c_uint
        == BROTLI_STATE_DONE as c_int as c_uint
    {
        BROTLI_TRUE
    } else {
        BROTLI_FALSE
    }) != 0
        && BrotliDecoderHasMoreOutput(s) == 0) as c_int;
}
#[inline]
pub unsafe fn BrotliDecoderGetErrorCode(
    mut s: *const BrotliDecoderStateInternal,
) -> BrotliDecoderErrorCode {
    let s_view: &BrotliDecoderStateInternal = unsafe { &*s };
    return s_view.error_code as BrotliDecoderErrorCode;
}
#[inline]
pub fn BrotliDecoderErrorString(
    mut c: BrotliDecoderErrorCode,
) -> *const c_char { {
    match c as c_int {
        0 => return b"_NO_ERROR\0" as *const u8 as *const c_char,
        1 => return b"_SUCCESS\0" as *const u8 as *const c_char,
        2 => return b"_NEEDS_MORE_INPUT\0" as *const u8 as *const c_char,
        3 => return b"_NEEDS_MORE_OUTPUT\0" as *const u8 as *const c_char,
        -1 => {
            return b"_ERROR_FORMAT_EXUBERANT_NIBBLE\0" as *const u8 as *const c_char;
        }
        -2 => {
            return b"_ERROR_FORMAT_RESERVED\0" as *const u8 as *const c_char;
        }
        -3 => {
            return b"_ERROR_FORMAT_EXUBERANT_META_NIBBLE\0" as *const u8
                as *const c_char;
        }
        -4 => {
            return b"_ERROR_FORMAT_SIMPLE_HUFFMAN_ALPHABET\0" as *const u8
                as *const c_char;
        }
        -5 => {
            return b"_ERROR_FORMAT_SIMPLE_HUFFMAN_SAME\0" as *const u8
                as *const c_char;
        }
        -6 => {
            return b"_ERROR_FORMAT_CL_SPACE\0" as *const u8 as *const c_char;
        }
        -7 => {
            return b"_ERROR_FORMAT_HUFFMAN_SPACE\0" as *const u8 as *const c_char;
        }
        -8 => {
            return b"_ERROR_FORMAT_CONTEXT_MAP_REPEAT\0" as *const u8
                as *const c_char;
        }
        -9 => {
            return b"_ERROR_FORMAT_BLOCK_LENGTH_1\0" as *const u8 as *const c_char;
        }
        -10 => {
            return b"_ERROR_FORMAT_BLOCK_LENGTH_2\0" as *const u8 as *const c_char;
        }
        -11 => {
            return b"_ERROR_FORMAT_TRANSFORM\0" as *const u8 as *const c_char;
        }
        -12 => {
            return b"_ERROR_FORMAT_DICTIONARY\0" as *const u8 as *const c_char;
        }
        -13 => {
            return b"_ERROR_FORMAT_WINDOW_BITS\0" as *const u8 as *const c_char;
        }
        -14 => {
            return b"_ERROR_FORMAT_PADDING_1\0" as *const u8 as *const c_char;
        }
        -15 => {
            return b"_ERROR_FORMAT_PADDING_2\0" as *const u8 as *const c_char;
        }
        -16 => {
            return b"_ERROR_FORMAT_DISTANCE\0" as *const u8 as *const c_char;
        }
        -17 => {
            return b"_ERROR_FORMAT_BLOCK_SWITCH\0" as *const u8 as *const c_char;
        }
        -18 => {
            return b"_ERROR_COMPOUND_DICTIONARY\0" as *const u8 as *const c_char;
        }
        -19 => {
            return b"_ERROR_DICTIONARY_NOT_SET\0" as *const u8 as *const c_char;
        }
        -20 => {
            return b"_ERROR_INVALID_ARGUMENTS\0" as *const u8 as *const c_char;
        }
        -21 => {
            return b"_ERROR_ALLOC_CONTEXT_MODES\0" as *const u8 as *const c_char;
        }
        -22 => {
            return b"_ERROR_ALLOC_TREE_GROUPS\0" as *const u8 as *const c_char;
        }
        -25 => {
            return b"_ERROR_ALLOC_CONTEXT_MAP\0" as *const u8 as *const c_char;
        }
        -26 => {
            return b"_ERROR_ALLOC_RING_BUFFER_1\0" as *const u8 as *const c_char;
        }
        -27 => {
            return b"_ERROR_ALLOC_RING_BUFFER_2\0" as *const u8 as *const c_char;
        }
        -30 => {
            return b"_ERROR_ALLOC_BLOCK_TYPE_TREES\0" as *const u8 as *const c_char;
        }
        -31 => return b"_ERROR_UNREACHABLE\0" as *const u8 as *const c_char,
        _ => return b"INVALID\0" as *const u8 as *const c_char,
    };
} }
#[inline]
pub fn BrotliDecoderVersion() -> uint32_t { {
    return BROTLI_VERSION as uint32_t;
} }
#[inline]
pub unsafe fn BrotliDecoderSetMetadataCallbacks(
    mut state: *mut BrotliDecoderStateInternal,
    mut start_func: brotli_decoder_metadata_start_func,
    mut chunk_func: brotli_decoder_metadata_chunk_func,
    mut opaque: *mut c_void,
) {
    (*state).metadata_start_func = start_func;
    (*state).metadata_chunk_func = chunk_func;
    (*state).metadata_callback_opaque = opaque;
}
