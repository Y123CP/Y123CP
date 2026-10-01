extern "C" {
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
    static _kBrotliContextLookupTable: [uint8_t; 2048];
    static _kBrotliPrefixCodeRanges: [BrotliPrefixCodeRange; 26];
    fn BrotliAllocate(m: *mut MemoryManager, n: size_t) -> *mut ::core::ffi::c_void;
    fn BrotliFree(m: *mut MemoryManager, p: *mut ::core::ffi::c_void);
    static kBrotliInsBase: [uint32_t; 24];
    static kBrotliInsExtra: [uint32_t; 24];
    static kBrotliCopyBase: [uint32_t; 24];
    static kBrotliCopyExtra: [uint32_t; 24];
    fn BrotliSetDepth(
        p: ::core::ffi::c_int,
        pool: *mut HuffmanTree,
        depth: *mut uint8_t,
        max_depth: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn BrotliCreateHuffmanTree(
        data: *const uint32_t,
        length: size_t,
        tree_limit: ::core::ffi::c_int,
        tree: *mut HuffmanTree,
        depth: *mut uint8_t,
    );
    fn BrotliWriteHuffmanTree(
        depth: *const uint8_t,
        length: size_t,
        tree_size: *mut size_t,
        tree: *mut uint8_t,
        extra_bits_data: *mut uint8_t,
    );
    fn BrotliConvertBitDepthsToSymbols(depth: *const uint8_t, len: size_t, bits: *mut uint16_t);
    static kBrotliShellGaps: [size_t; 6];
}
pub type size_t = usize;
pub type __int8_t = i8;
pub type __uint8_t = u8;
pub type __int16_t = i16;
pub type __uint16_t = u16;
pub type __int32_t = i32;
pub type __uint32_t = u32;
pub type __uint64_t = u64;
pub type int8_t = __int8_t;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type brotli_alloc_func =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, size_t) -> *mut ::core::ffi::c_void>;
pub type brotli_free_func =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void) -> ()>;
pub type ContextType = ::core::ffi::c_uint;
pub const CONTEXT_SIGNED: ContextType = 3;
pub const CONTEXT_UTF8: ContextType = 2;
pub const CONTEXT_MSB6: ContextType = 1;
pub const CONTEXT_LSB6: ContextType = 0;
pub type ContextLut = *const uint8_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BrotliPrefixCodeRange {
    pub offset: uint16_t,
    pub nbits: uint8_t,
}
pub type BrotliEncoderMode = ::core::ffi::c_uint;
pub const BROTLI_MODE_FONT: BrotliEncoderMode = 2;
pub const BROTLI_MODE_TEXT: BrotliEncoderMode = 1;
pub const BROTLI_MODE_GENERIC: BrotliEncoderMode = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BrotliDictionary {
    pub size_bits_by_length: [uint8_t; 32],
    pub offsets_by_length: [uint32_t; 32],
    pub data_size: size_t,
    pub data: *const uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MemoryManager {
    pub alloc_func: brotli_alloc_func,
    pub free_func: brotli_free_func,
    pub opaque: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PreparedDictionary {
    pub magic: uint32_t,
    pub num_items: uint32_t,
    pub source_size: uint32_t,
    pub hash_bits: uint32_t,
    pub bucket_bits: uint32_t,
    pub slot_bits: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CompoundDictionary {
    pub num_chunks: size_t,
    pub total_size: size_t,
    pub chunks: [*const PreparedDictionary; 16],
    pub chunk_source: [*const uint8_t; 16],
    pub chunk_offsets: [size_t; 16],
    pub num_prepared_instances_: size_t,
    pub prepared_instances_: [*mut PreparedDictionary; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DictWord {
    pub len: uint8_t,
    pub transform: uint8_t,
    pub idx: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BrotliTrieNode {
    pub single: uint8_t,
    pub c: uint8_t,
    pub len_: uint8_t,
    pub idx_: uint32_t,
    pub sub: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BrotliTrie {
    pub pool: *mut BrotliTrieNode,
    pub pool_capacity: size_t,
    pub pool_size: size_t,
    pub root: BrotliTrieNode,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BrotliEncoderDictionary {
    pub words: *const BrotliDictionary,
    pub num_transforms: uint32_t,
    pub cutoffTransformsCount: uint32_t,
    pub cutoffTransforms: uint64_t,
    pub hash_table_words: *const uint16_t,
    pub hash_table_lengths: *const uint8_t,
    pub buckets: *const uint16_t,
    pub dict_words: *const DictWord,
    pub trie: BrotliTrie,
    pub has_words_heavy: ::core::ffi::c_int,
    pub parent: *const ContextualEncoderDictionary,
    pub hash_table_data_words_: *mut uint16_t,
    pub hash_table_data_lengths_: *mut uint8_t,
    pub buckets_alloc_size_: size_t,
    pub buckets_data_: *mut uint16_t,
    pub dict_words_alloc_size_: size_t,
    pub dict_words_data_: *mut DictWord,
    pub words_instance_: *mut BrotliDictionary,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ContextualEncoderDictionary {
    pub context_based: ::core::ffi::c_int,
    pub num_dictionaries: uint8_t,
    pub context_map: [uint8_t; 64],
    pub dict: [*const BrotliEncoderDictionary; 64],
    pub num_instances_: size_t,
    pub instance_: BrotliEncoderDictionary,
    pub instances_: *mut BrotliEncoderDictionary,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SharedEncoderDictionary {
    pub magic: uint32_t,
    pub compound: CompoundDictionary,
    pub contextual: ContextualEncoderDictionary,
    pub max_quality: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BrotliHasherParams {
    pub type_0: ::core::ffi::c_int,
    pub bucket_bits: ::core::ffi::c_int,
    pub block_bits: ::core::ffi::c_int,
    pub num_last_distances_to_check: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BrotliDistanceParams {
    pub distance_postfix_bits: uint32_t,
    pub num_direct_distance_codes: uint32_t,
    pub alphabet_size_max: uint32_t,
    pub alphabet_size_limit: uint32_t,
    pub max_distance: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BrotliEncoderParams {
    pub mode: BrotliEncoderMode,
    pub quality: ::core::ffi::c_int,
    pub lgwin: ::core::ffi::c_int,
    pub lgblock: ::core::ffi::c_int,
    pub stream_offset: size_t,
    pub size_hint: size_t,
    pub disable_literal_context_modeling: ::core::ffi::c_int,
    pub large_window: ::core::ffi::c_int,
    pub hasher: BrotliHasherParams,
    pub dist: BrotliDistanceParams,
    pub dictionary: SharedEncoderDictionary,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Command {
    pub insert_len_: uint32_t,
    pub copy_len_: uint32_t,
    pub dist_extra_: uint32_t,
    pub cmd_prefix_: uint16_t,
    pub dist_prefix_: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HuffmanTree {
    pub total_count_: uint32_t,
    pub index_left_: int16_t,
    pub index_right_or_value_: int16_t,
}
pub type HuffmanTreeComparator =
    Option<unsafe extern "C" fn(*const HuffmanTree, *const HuffmanTree) -> ::core::ffi::c_int>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BlockSplit {
    pub num_types: size_t,
    pub num_blocks: size_t,
    pub types: *mut uint8_t,
    pub lengths: *mut uint32_t,
    pub types_alloc_size: size_t,
    pub lengths_alloc_size: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HistogramLiteral {
    pub data_: [uint32_t; 256],
    pub total_count_: size_t,
    pub bit_cost_: ::core::ffi::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HistogramCommand {
    pub data_: [uint32_t; 704],
    pub total_count_: size_t,
    pub bit_cost_: ::core::ffi::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HistogramDistance {
    pub data_: [uint32_t; 544],
    pub total_count_: size_t,
    pub bit_cost_: ::core::ffi::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MetaBlockSplit {
    pub literal_split: BlockSplit,
    pub command_split: BlockSplit,
    pub distance_split: BlockSplit,
    pub literal_context_map: *mut uint32_t,
    pub literal_context_map_size: size_t,
    pub distance_context_map: *mut uint32_t,
    pub distance_context_map_size: size_t,
    pub literal_histograms: *mut HistogramLiteral,
    pub literal_histograms_size: size_t,
    pub command_histograms: *mut HistogramCommand,
    pub command_histograms_size: size_t,
    pub distance_histograms: *mut HistogramDistance,
    pub distance_histograms_size: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct StoreMetablockArena {
    pub literal_enc: BlockEncoder,
    pub command_enc: BlockEncoder,
    pub distance_enc: BlockEncoder,
    pub context_map_arena: EncodeContextMapArena,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EncodeContextMapArena {
    pub histogram: [uint32_t; 272],
    pub depths: [uint8_t; 272],
    pub bits: [uint16_t; 272],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BlockEncoder {
    pub histogram_length_: size_t,
    pub num_block_types_: size_t,
    pub block_types_: *const uint8_t,
    pub block_lengths_: *const uint32_t,
    pub num_blocks_: size_t,
    pub block_split_code_: BlockSplitCode,
    pub block_ix_: size_t,
    pub block_len_: size_t,
    pub entropy_ix_: size_t,
    pub depths_: *mut uint8_t,
    pub bits_: *mut uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BlockSplitCode {
    pub type_code_calculator: BlockTypeCodeCalculator,
    pub type_depths: [uint8_t; 258],
    pub type_bits: [uint16_t; 258],
    pub length_depths: [uint8_t; 26],
    pub length_bits: [uint16_t; 26],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BlockTypeCodeCalculator {
    pub last_type: size_t,
    pub second_last_type: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MetablockArena {
    pub lit_histo: HistogramLiteral,
    pub cmd_histo: HistogramCommand,
    pub dist_histo: HistogramDistance,
    pub lit_depth: [uint8_t; 256],
    pub lit_bits: [uint16_t; 256],
    pub cmd_depth: [uint8_t; 704],
    pub cmd_bits: [uint16_t; 704],
    pub dist_depth: [uint8_t; 140],
    pub dist_bits: [uint16_t; 140],
    pub tree: [HuffmanTree; 1409],
}
static mut kSymbolMask: uint32_t = 0;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const BROTLI_TRUE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const BROTLI_FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const BROTLI_UINT32_MAX: uint32_t = !(0 as ::core::ffi::c_int as uint32_t);
#[inline(always)]
unsafe extern "C" fn BrotliUnalignedWrite64(mut p: *mut ::core::ffi::c_void, mut v: uint64_t) {
    memcpy(
        p,
        &raw mut v as *const ::core::ffi::c_void,
        ::core::mem::size_of::<uint64_t>() as size_t,
    );
}
#[inline(always)]
unsafe extern "C" fn brotli_max_uint32_t(mut a: uint32_t, mut b: uint32_t) -> uint32_t {
    return if a > b { a } else { b };
}
#[inline(always)]
unsafe extern "C" fn brotli_min_uint32_t(mut a: uint32_t, mut b: uint32_t) -> uint32_t {
    return if a < b { a } else { b };
}
pub const BROTLI_NUM_LITERAL_SYMBOLS: ::core::ffi::c_int = 256 as ::core::ffi::c_int;
pub const BROTLI_NUM_COMMAND_SYMBOLS: ::core::ffi::c_int = 704 as ::core::ffi::c_int;
pub const BROTLI_NUM_BLOCK_LEN_SYMBOLS: ::core::ffi::c_int = 26 as ::core::ffi::c_int;
pub const BROTLI_REPEAT_PREVIOUS_CODE_LENGTH: size_t = 16 as size_t;
pub const BROTLI_REPEAT_ZERO_CODE_LENGTH: ::core::ffi::c_int = 17 as ::core::ffi::c_int;
pub const BROTLI_CODE_LENGTH_CODES: ::core::ffi::c_int =
    BROTLI_REPEAT_ZERO_CODE_LENGTH + 1 as ::core::ffi::c_int;
pub const BROTLI_NUM_DISTANCE_SHORT_CODES: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const BROTLI_LITERAL_CONTEXT_BITS: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const BROTLI_DISTANCE_CONTEXT_BITS: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
#[inline(always)]
unsafe extern "C" fn Log2FloorNonZero(mut n: size_t) -> uint32_t {
    return 31 as uint32_t ^ (n as uint32_t).leading_zeros() as i32 as uint32_t;
}
#[inline(always)]
unsafe extern "C" fn GetInsertLengthCode(mut insertlen: size_t) -> uint16_t {
    if insertlen < 6 as size_t {
        return insertlen as uint16_t;
    } else if insertlen < 130 as size_t {
        let mut nbits: uint32_t =
            Log2FloorNonZero(insertlen.wrapping_sub(2 as size_t)).wrapping_sub(1 as uint32_t);
        return ((nbits << 1 as ::core::ffi::c_int) as size_t)
            .wrapping_add(insertlen.wrapping_sub(2 as size_t) >> nbits)
            .wrapping_add(2 as size_t) as uint16_t;
    } else if insertlen < 2114 as size_t {
        return Log2FloorNonZero(insertlen.wrapping_sub(66 as size_t)).wrapping_add(10 as uint32_t)
            as uint16_t;
    } else if insertlen < 6210 as size_t {
        return 21 as uint16_t;
    } else if insertlen < 22594 as size_t {
        return 22 as uint16_t;
    } else {
        return 23 as uint16_t;
    };
}
#[inline(always)]
unsafe extern "C" fn GetCopyLengthCode(mut copylen: size_t) -> uint16_t {
    if copylen < 10 as size_t {
        return copylen.wrapping_sub(2 as size_t) as uint16_t;
    } else if copylen < 134 as size_t {
        let mut nbits: uint32_t =
            Log2FloorNonZero(copylen.wrapping_sub(6 as size_t)).wrapping_sub(1 as uint32_t);
        return ((nbits << 1 as ::core::ffi::c_int) as size_t)
            .wrapping_add(copylen.wrapping_sub(6 as size_t) >> nbits)
            .wrapping_add(4 as size_t) as uint16_t;
    } else if copylen < 2118 as size_t {
        return Log2FloorNonZero(copylen.wrapping_sub(70 as size_t)).wrapping_add(12 as uint32_t)
            as uint16_t;
    } else {
        return 23 as uint16_t;
    };
}
#[inline(always)]
unsafe extern "C" fn GetInsertBase(mut inscode: uint16_t) -> uint32_t {
    return kBrotliInsBase[inscode as usize];
}
#[inline(always)]
unsafe extern "C" fn GetInsertExtra(mut inscode: uint16_t) -> uint32_t {
    return kBrotliInsExtra[inscode as usize];
}
#[inline(always)]
unsafe extern "C" fn GetCopyBase(mut copycode: uint16_t) -> uint32_t {
    return kBrotliCopyBase[copycode as usize];
}
#[inline(always)]
unsafe extern "C" fn GetCopyExtra(mut copycode: uint16_t) -> uint32_t {
    return kBrotliCopyExtra[copycode as usize];
}
#[inline(always)]
unsafe extern "C" fn CommandDistanceContext(mut self_0: *const Command) -> uint32_t {
    let mut r: uint32_t =
        ((*self_0).cmd_prefix_ as ::core::ffi::c_int >> 6 as ::core::ffi::c_int) as uint32_t;
    let mut c: uint32_t =
        ((*self_0).cmd_prefix_ as ::core::ffi::c_int & 7 as ::core::ffi::c_int) as uint32_t;
    if (r == 0 as uint32_t || r == 2 as uint32_t || r == 4 as uint32_t || r == 7 as uint32_t)
        && c <= 2 as uint32_t
    {
        return c;
    }
    return 3 as uint32_t;
}
#[inline(always)]
unsafe extern "C" fn CommandCopyLen(mut self_0: *const Command) -> uint32_t {
    return (*self_0).copy_len_ & 0x1ffffff as uint32_t;
}
#[inline(always)]
unsafe extern "C" fn CommandCopyLenCode(mut self_0: *const Command) -> uint32_t {
    let mut modifier: uint32_t = (*self_0).copy_len_ >> 25 as ::core::ffi::c_int;
    let mut delta: int32_t = (modifier | (modifier & 0x40 as uint32_t) << 1 as ::core::ffi::c_int)
        as uint8_t as int8_t as int32_t;
    return (((*self_0).copy_len_ & 0x1ffffff as uint32_t) as int32_t + delta) as uint32_t;
}
#[inline(always)]
unsafe extern "C" fn InitHuffmanTree(
    mut self_0: *mut HuffmanTree,
    mut count: uint32_t,
    mut left: int16_t,
    mut right: int16_t,
) {
    (*self_0).total_count_ = count;
    (*self_0).index_left_ = left;
    (*self_0).index_right_or_value_ = right;
}
#[inline(always)]
unsafe extern "C" fn SortHuffmanTreeItems(
    mut items: *mut HuffmanTree,
    n: size_t,
    mut comparator: HuffmanTreeComparator,
) {
    if n < 13 as size_t {
        let mut i: size_t = 0;
        i = 1 as size_t;
        while i < n {
            let mut tmp: HuffmanTree = *items.offset(i as isize);
            let mut k: size_t = i;
            let mut j: size_t = i.wrapping_sub(1 as size_t);
            while comparator.expect("non-null function pointer")(
                &raw mut tmp,
                items.offset(j as isize) as *mut HuffmanTree,
            ) != 0
            {
                *items.offset(k as isize) = *items.offset(j as isize);
                k = j;
                let fresh0 = j;
                j = j.wrapping_sub(1);
                if fresh0 == 0 {
                    break;
                }
            }
            *items.offset(k as isize) = tmp;
            i = i.wrapping_add(1);
        }
        return;
    } else {
        let mut g: ::core::ffi::c_int = if n < 57 as size_t {
            2 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        };
        while g < 6 as ::core::ffi::c_int {
            let mut gap: size_t = kBrotliShellGaps[g as usize];
            let mut i_0: size_t = 0;
            i_0 = gap;
            while i_0 < n {
                let mut j_0: size_t = i_0;
                let mut tmp_0: HuffmanTree = *items.offset(i_0 as isize);
                while j_0 >= gap
                    && comparator.expect("non-null function pointer")(
                        &raw mut tmp_0,
                        items.offset(j_0.wrapping_sub(gap) as isize) as *mut HuffmanTree,
                    ) != 0
                {
                    *items.offset(j_0 as isize) = *items.offset(j_0.wrapping_sub(gap) as isize);
                    j_0 = (j_0 as ::core::ffi::c_ulong).wrapping_sub(gap as ::core::ffi::c_ulong)
                        as size_t as size_t;
                }
                *items.offset(j_0 as isize) = tmp_0;
                i_0 = i_0.wrapping_add(1);
            }
            g += 1;
        }
    };
}
#[inline(always)]
unsafe extern "C" fn HistogramClearLiteral(mut self_0: *mut HistogramLiteral) {
    memset(
        &raw mut (*self_0).data_ as *mut uint32_t as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<[uint32_t; 256]>() as size_t,
    );
    (*self_0).total_count_ = 0 as size_t;
    (*self_0).bit_cost_ = ::core::f64::INFINITY;
}
#[inline(always)]
unsafe extern "C" fn HistogramAddLiteral(mut self_0: *mut HistogramLiteral, mut val: size_t) {
    (*self_0).data_[val as usize] = (*self_0).data_[val as usize].wrapping_add(1);
    (*self_0).total_count_ = (*self_0).total_count_.wrapping_add(1);
}
#[inline(always)]
unsafe extern "C" fn HistogramClearCommand(mut self_0: *mut HistogramCommand) {
    memset(
        &raw mut (*self_0).data_ as *mut uint32_t as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<[uint32_t; 704]>() as size_t,
    );
    (*self_0).total_count_ = 0 as size_t;
    (*self_0).bit_cost_ = ::core::f64::INFINITY;
}
#[inline(always)]
unsafe extern "C" fn HistogramAddCommand(mut self_0: *mut HistogramCommand, mut val: size_t) {
    (*self_0).data_[val as usize] = (*self_0).data_[val as usize].wrapping_add(1);
    (*self_0).total_count_ = (*self_0).total_count_.wrapping_add(1);
}
#[inline(always)]
unsafe extern "C" fn HistogramClearDistance(mut self_0: *mut HistogramDistance) {
    memset(
        &raw mut (*self_0).data_ as *mut uint32_t as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<[uint32_t; 544]>() as size_t,
    );
    (*self_0).total_count_ = 0 as size_t;
    (*self_0).bit_cost_ = ::core::f64::INFINITY;
}
#[inline(always)]
unsafe extern "C" fn HistogramAddDistance(mut self_0: *mut HistogramDistance, mut val: size_t) {
    (*self_0).data_[val as usize] = (*self_0).data_[val as usize].wrapping_add(1);
    (*self_0).total_count_ = (*self_0).total_count_.wrapping_add(1);
}
pub const MAX_SIMPLE_DISTANCE_ALPHABET_SIZE: ::core::ffi::c_uint =
    ((BROTLI_NUM_DISTANCE_SHORT_CODES + 0 as ::core::ffi::c_int) as ::core::ffi::c_uint)
        .wrapping_add(
            (62 as ::core::ffi::c_uint) << 0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int,
        );
#[inline(always)]
unsafe extern "C" fn BlockLengthPrefixCode(mut len: uint32_t) -> uint32_t {
    let mut code: uint32_t = (if len >= 177 as uint32_t {
        if len >= 753 as uint32_t {
            20 as ::core::ffi::c_int
        } else {
            14 as ::core::ffi::c_int
        }
    } else if len >= 41 as uint32_t {
        7 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    }) as uint32_t;
    while code < (BROTLI_NUM_BLOCK_LEN_SYMBOLS - 1 as ::core::ffi::c_int) as uint32_t
        && len
            >= _kBrotliPrefixCodeRanges[code.wrapping_add(1 as uint32_t) as usize].offset
                as uint32_t
    {
        code = code.wrapping_add(1);
    }
    return code;
}
#[inline(always)]
unsafe extern "C" fn GetBlockLengthPrefixCode(
    mut len: uint32_t,
    mut code: *mut size_t,
    mut n_extra: *mut uint32_t,
    mut extra: *mut uint32_t,
) {
    *code = BlockLengthPrefixCode(len) as size_t;
    *n_extra = _kBrotliPrefixCodeRanges[*code as usize].nbits as uint32_t;
    *extra = len.wrapping_sub(_kBrotliPrefixCodeRanges[*code as usize].offset as uint32_t);
}
unsafe extern "C" fn InitBlockTypeCodeCalculator(mut self_0: *mut BlockTypeCodeCalculator) {
    (*self_0).last_type = 1 as size_t;
    (*self_0).second_last_type = 0 as size_t;
}
#[inline(always)]
unsafe extern "C" fn NextBlockTypeCode(
    mut calculator: *mut BlockTypeCodeCalculator,
    mut type_0: uint8_t,
) -> size_t {
    let mut type_code: size_t =
        (if type_0 as size_t == (*calculator).last_type.wrapping_add(1 as size_t) {
            1 as ::core::ffi::c_uint
        } else if type_0 as size_t == (*calculator).second_last_type {
            0 as ::core::ffi::c_uint
        } else {
            (type_0 as ::core::ffi::c_uint).wrapping_add(2 as ::core::ffi::c_uint)
        }) as size_t;
    (*calculator).second_last_type = (*calculator).last_type;
    (*calculator).last_type = type_0 as size_t;
    return type_code;
}
unsafe extern "C" fn BrotliEncodeMlen(
    mut length: size_t,
    mut bits: *mut uint64_t,
    mut numbits: *mut size_t,
    mut nibblesbits: *mut uint64_t,
) {
    let mut lg: size_t = (if length == 1 as size_t {
        1 as uint32_t
    } else {
        Log2FloorNonZero(length.wrapping_sub(1 as size_t) as uint32_t as size_t)
            .wrapping_add(1 as uint32_t)
    }) as size_t;
    let mut mnibbles: size_t = (if lg < 16 as size_t {
        16 as size_t
    } else {
        lg.wrapping_add(3 as size_t)
    })
    .wrapping_div(4 as size_t);
    *nibblesbits = mnibbles.wrapping_sub(4 as size_t) as uint64_t;
    *numbits = mnibbles.wrapping_mul(4 as size_t);
    *bits = length.wrapping_sub(1 as size_t) as uint64_t;
}
#[inline(always)]
unsafe extern "C" fn StoreCommandExtra(
    mut cmd: *const Command,
    mut storage_ix: *mut size_t,
    mut storage: *mut uint8_t,
) {
    let mut copylen_code: uint32_t = CommandCopyLenCode(cmd);
    let mut inscode: uint16_t = GetInsertLengthCode((*cmd).insert_len_ as size_t);
    let mut copycode: uint16_t = GetCopyLengthCode(copylen_code as size_t);
    let mut insnumextra: uint32_t = GetInsertExtra(inscode);
    let mut insextraval: uint64_t =
        (*cmd).insert_len_.wrapping_sub(GetInsertBase(inscode)) as uint64_t;
    let mut copyextraval: uint64_t = copylen_code.wrapping_sub(GetCopyBase(copycode)) as uint64_t;
    let mut bits: uint64_t = copyextraval << insnumextra | insextraval;
    BrotliWriteBits(
        insnumextra.wrapping_add(GetCopyExtra(copycode)) as size_t,
        bits,
        storage_ix,
        storage,
    );
}
unsafe extern "C" fn StoreVarLenUint8(
    mut n: size_t,
    mut storage_ix: *mut size_t,
    mut storage: *mut uint8_t,
) {
    if n == 0 as size_t {
        BrotliWriteBits(1 as size_t, 0 as uint64_t, storage_ix, storage);
    } else {
        let mut nbits: size_t = Log2FloorNonZero(n) as size_t;
        BrotliWriteBits(1 as size_t, 1 as uint64_t, storage_ix, storage);
        BrotliWriteBits(3 as size_t, nbits as uint64_t, storage_ix, storage);
        BrotliWriteBits(
            nbits,
            (n as uint64_t).wrapping_sub((1 as ::core::ffi::c_int as uint64_t) << nbits),
            storage_ix,
            storage,
        );
    };
}
unsafe extern "C" fn StoreCompressedMetaBlockHeader(
    mut is_final_block: ::core::ffi::c_int,
    mut length: size_t,
    mut storage_ix: *mut size_t,
    mut storage: *mut uint8_t,
) {
    let mut lenbits: uint64_t = 0;
    let mut nlenbits: size_t = 0;
    let mut nibblesbits: uint64_t = 0;
    BrotliWriteBits(1 as size_t, is_final_block as uint64_t, storage_ix, storage);
    if is_final_block != 0 {
        BrotliWriteBits(1 as size_t, 0 as uint64_t, storage_ix, storage);
    }
    BrotliEncodeMlen(
        length,
        &raw mut lenbits,
        &raw mut nlenbits,
        &raw mut nibblesbits,
    );
    BrotliWriteBits(2 as size_t, nibblesbits, storage_ix, storage);
    BrotliWriteBits(nlenbits, lenbits, storage_ix, storage);
    if is_final_block == 0 {
        BrotliWriteBits(1 as size_t, 0 as uint64_t, storage_ix, storage);
    }
}
unsafe extern "C" fn BrotliStoreUncompressedMetaBlockHeader(
    mut length: size_t,
    mut storage_ix: *mut size_t,
    mut storage: *mut uint8_t,
) {
    let mut lenbits: uint64_t = 0;
    let mut nlenbits: size_t = 0;
    let mut nibblesbits: uint64_t = 0;
    BrotliWriteBits(1 as size_t, 0 as uint64_t, storage_ix, storage);
    BrotliEncodeMlen(
        length,
        &raw mut lenbits,
        &raw mut nlenbits,
        &raw mut nibblesbits,
    );
    BrotliWriteBits(2 as size_t, nibblesbits, storage_ix, storage);
    BrotliWriteBits(nlenbits, lenbits, storage_ix, storage);
    BrotliWriteBits(1 as size_t, 1 as uint64_t, storage_ix, storage);
}
unsafe extern "C" fn BrotliStoreHuffmanTreeOfHuffmanTreeToBitMask(
    num_codes: ::core::ffi::c_int,
    mut code_length_bitdepth: *const uint8_t,
    mut storage_ix: *mut size_t,
    mut storage: *mut uint8_t,
) {
    static mut kStorageOrder: [uint8_t; 18] = [
        1 as ::core::ffi::c_int as uint8_t,
        2 as ::core::ffi::c_int as uint8_t,
        3 as ::core::ffi::c_int as uint8_t,
        4 as ::core::ffi::c_int as uint8_t,
        0 as ::core::ffi::c_int as uint8_t,
        5 as ::core::ffi::c_int as uint8_t,
        17 as ::core::ffi::c_int as uint8_t,
        6 as ::core::ffi::c_int as uint8_t,
        16 as ::core::ffi::c_int as uint8_t,
        7 as ::core::ffi::c_int as uint8_t,
        8 as ::core::ffi::c_int as uint8_t,
        9 as ::core::ffi::c_int as uint8_t,
        10 as ::core::ffi::c_int as uint8_t,
        11 as ::core::ffi::c_int as uint8_t,
        12 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        14 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
    ];
    static mut kHuffmanBitLengthHuffmanCodeSymbols: [uint8_t; 6] = [
        0 as ::core::ffi::c_int as uint8_t,
        7 as ::core::ffi::c_int as uint8_t,
        3 as ::core::ffi::c_int as uint8_t,
        2 as ::core::ffi::c_int as uint8_t,
        1 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
    ];
    static mut kHuffmanBitLengthHuffmanCodeBitLengths: [uint8_t; 6] = [
        2 as ::core::ffi::c_int as uint8_t,
        4 as ::core::ffi::c_int as uint8_t,
        3 as ::core::ffi::c_int as uint8_t,
        2 as ::core::ffi::c_int as uint8_t,
        2 as ::core::ffi::c_int as uint8_t,
        4 as ::core::ffi::c_int as uint8_t,
    ];
    let mut skip_some: size_t = 0 as size_t;
    let mut codes_to_store: size_t = BROTLI_CODE_LENGTH_CODES as size_t;
    if num_codes > 1 as ::core::ffi::c_int {
        while codes_to_store > 0 as size_t {
            if *code_length_bitdepth
                .offset(kStorageOrder[codes_to_store.wrapping_sub(1 as size_t) as usize] as isize)
                as ::core::ffi::c_int
                != 0 as ::core::ffi::c_int
            {
                break;
            }
            codes_to_store = codes_to_store.wrapping_sub(1);
        }
    }
    if *code_length_bitdepth.offset(kStorageOrder[0 as ::core::ffi::c_int as usize] as isize)
        as ::core::ffi::c_int
        == 0 as ::core::ffi::c_int
        && *code_length_bitdepth.offset(kStorageOrder[1 as ::core::ffi::c_int as usize] as isize)
            as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
    {
        skip_some = 2 as size_t;
        if *code_length_bitdepth.offset(kStorageOrder[2 as ::core::ffi::c_int as usize] as isize)
            as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        {
            skip_some = 3 as size_t;
        }
    }
    BrotliWriteBits(2 as size_t, skip_some as uint64_t, storage_ix, storage);
    let mut i: size_t = 0;
    i = skip_some;
    while i < codes_to_store {
        let mut l: size_t =
            *code_length_bitdepth.offset(kStorageOrder[i as usize] as isize) as size_t;
        BrotliWriteBits(
            kHuffmanBitLengthHuffmanCodeBitLengths[l as usize] as size_t,
            kHuffmanBitLengthHuffmanCodeSymbols[l as usize] as uint64_t,
            storage_ix,
            storage,
        );
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn BrotliStoreHuffmanTreeToBitMask(
    huffman_tree_size: size_t,
    mut huffman_tree: *const uint8_t,
    mut huffman_tree_extra_bits: *const uint8_t,
    mut code_length_bitdepth: *const uint8_t,
    mut code_length_bitdepth_symbols: *const uint16_t,
    mut storage_ix: *mut size_t,
    mut storage: *mut uint8_t,
) {
    let mut i: size_t = 0;
    i = 0 as size_t;
    while i < huffman_tree_size {
        let mut ix: size_t = *huffman_tree.offset(i as isize) as size_t;
        BrotliWriteBits(
            *code_length_bitdepth.offset(ix as isize) as size_t,
            *code_length_bitdepth_symbols.offset(ix as isize) as uint64_t,
            storage_ix,
            storage,
        );
        match ix {
            16 => {
                BrotliWriteBits(
                    2 as size_t,
                    *huffman_tree_extra_bits.offset(i as isize) as uint64_t,
                    storage_ix,
                    storage,
                );
            }
            17 => {
                BrotliWriteBits(
                    3 as size_t,
                    *huffman_tree_extra_bits.offset(i as isize) as uint64_t,
                    storage_ix,
                    storage,
                );
            }
            _ => {}
        }
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn StoreSimpleHuffmanTree(
    mut depths: *const uint8_t,
    mut symbols: *mut size_t,
    mut num_symbols: size_t,
    mut max_bits: size_t,
    mut storage_ix: *mut size_t,
    mut storage: *mut uint8_t,
) {
    BrotliWriteBits(2 as size_t, 1 as uint64_t, storage_ix, storage);
    BrotliWriteBits(
        2 as size_t,
        (num_symbols as uint64_t).wrapping_sub(1 as uint64_t),
        storage_ix,
        storage,
    );
    let mut i: size_t = 0;
    i = 0 as size_t;
    while i < num_symbols {
        let mut j: size_t = 0;
        j = i.wrapping_add(1 as size_t);
        while j < num_symbols {
            if (*depths.offset(*symbols.offset(j as isize) as isize) as ::core::ffi::c_int)
                < *depths.offset(*symbols.offset(i as isize) as isize) as ::core::ffi::c_int
            {
                let mut __brotli_swap_tmp: size_t = *symbols.offset(j as isize);
                *symbols.offset(j as isize) = *symbols.offset(i as isize);
                *symbols.offset(i as isize) = __brotli_swap_tmp;
            }
            j = j.wrapping_add(1);
        }
        i = i.wrapping_add(1);
    }
    if num_symbols == 2 as size_t {
        BrotliWriteBits(
            max_bits,
            *symbols.offset(0 as ::core::ffi::c_int as isize) as uint64_t,
            storage_ix,
            storage,
        );
        BrotliWriteBits(
            max_bits,
            *symbols.offset(1 as ::core::ffi::c_int as isize) as uint64_t,
            storage_ix,
            storage,
        );
    } else if num_symbols == 3 as size_t {
        BrotliWriteBits(
            max_bits,
            *symbols.offset(0 as ::core::ffi::c_int as isize) as uint64_t,
            storage_ix,
            storage,
        );
        BrotliWriteBits(
            max_bits,
            *symbols.offset(1 as ::core::ffi::c_int as isize) as uint64_t,
            storage_ix,
            storage,
        );
        BrotliWriteBits(
            max_bits,
            *symbols.offset(2 as ::core::ffi::c_int as isize) as uint64_t,
            storage_ix,
            storage,
        );
    } else {
        BrotliWriteBits(
            max_bits,
            *symbols.offset(0 as ::core::ffi::c_int as isize) as uint64_t,
            storage_ix,
            storage,
        );
        BrotliWriteBits(
            max_bits,
            *symbols.offset(1 as ::core::ffi::c_int as isize) as uint64_t,
            storage_ix,
            storage,
        );
        BrotliWriteBits(
            max_bits,
            *symbols.offset(2 as ::core::ffi::c_int as isize) as uint64_t,
            storage_ix,
            storage,
        );
        BrotliWriteBits(
            max_bits,
            *symbols.offset(3 as ::core::ffi::c_int as isize) as uint64_t,
            storage_ix,
            storage,
        );
        BrotliWriteBits(
            1 as size_t,
            (if *depths.offset(*symbols.offset(0 as ::core::ffi::c_int as isize) as isize)
                as ::core::ffi::c_int
                == 1 as ::core::ffi::c_int
            {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            }) as uint64_t,
            storage_ix,
            storage,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn BrotliStoreHuffmanTree(
    mut depths: *const uint8_t,
    mut num: size_t,
    mut tree: *mut HuffmanTree,
    mut storage_ix: *mut size_t,
    mut storage: *mut uint8_t,
) {
    let mut huffman_tree: [uint8_t; 704] = [0; 704];
    let mut huffman_tree_extra_bits: [uint8_t; 704] = [0; 704];
    let mut huffman_tree_size: size_t = 0 as size_t;
    let mut code_length_bitdepth: [uint8_t; 18] = [
        0 as ::core::ffi::c_int as uint8_t,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    let mut code_length_bitdepth_symbols: [uint16_t; 18] = [0; 18];
    let mut huffman_tree_histogram: [uint32_t; 18] = [
        0 as ::core::ffi::c_int as uint32_t,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    let mut i: size_t = 0;
    let mut num_codes: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut code: size_t = 0 as size_t;
    BrotliWriteHuffmanTree(
        depths,
        num,
        &raw mut huffman_tree_size,
        &raw mut huffman_tree as *mut uint8_t,
        &raw mut huffman_tree_extra_bits as *mut uint8_t,
    );
    i = 0 as size_t;
    while i < huffman_tree_size {
        huffman_tree_histogram[huffman_tree[i as usize] as usize] =
            huffman_tree_histogram[huffman_tree[i as usize] as usize].wrapping_add(1);
        i = i.wrapping_add(1);
    }
    i = 0 as size_t;
    while i < BROTLI_CODE_LENGTH_CODES as size_t {
        if huffman_tree_histogram[i as usize] != 0 {
            if num_codes == 0 as ::core::ffi::c_int {
                code = i;
                num_codes = 1 as ::core::ffi::c_int;
            } else if num_codes == 1 as ::core::ffi::c_int {
                num_codes = 2 as ::core::ffi::c_int;
                break;
            }
        }
        i = i.wrapping_add(1);
    }
    BrotliCreateHuffmanTree(
        &raw mut huffman_tree_histogram as *mut uint32_t,
        BROTLI_CODE_LENGTH_CODES as size_t,
        5 as ::core::ffi::c_int,
        tree,
        &raw mut code_length_bitdepth as *mut uint8_t,
    );
    BrotliConvertBitDepthsToSymbols(
        &raw mut code_length_bitdepth as *mut uint8_t,
        BROTLI_CODE_LENGTH_CODES as size_t,
        &raw mut code_length_bitdepth_symbols as *mut uint16_t,
    );
    BrotliStoreHuffmanTreeOfHuffmanTreeToBitMask(
        num_codes,
        &raw mut code_length_bitdepth as *mut uint8_t,
        storage_ix,
        storage,
    );
    if num_codes == 1 as ::core::ffi::c_int {
        code_length_bitdepth[code as usize] = 0 as uint8_t;
    }
    BrotliStoreHuffmanTreeToBitMask(
        huffman_tree_size,
        &raw mut huffman_tree as *mut uint8_t,
        &raw mut huffman_tree_extra_bits as *mut uint8_t,
        &raw mut code_length_bitdepth as *mut uint8_t,
        &raw mut code_length_bitdepth_symbols as *mut uint16_t,
        storage_ix,
        storage,
    );
}
unsafe extern "C" fn BuildAndStoreHuffmanTree(
    mut histogram: *const uint32_t,
    histogram_length: size_t,
    alphabet_size: size_t,
    mut tree: *mut HuffmanTree,
    mut depth: *mut uint8_t,
    mut bits: *mut uint16_t,
    mut storage_ix: *mut size_t,
    mut storage: *mut uint8_t,
) {
    let mut count: size_t = 0 as size_t;
    let mut s4: [size_t; 4] = [0 as ::core::ffi::c_int as size_t, 0, 0, 0];
    let mut i: size_t = 0;
    let mut max_bits: size_t = 0 as size_t;
    i = 0 as size_t;
    while i < histogram_length {
        if *histogram.offset(i as isize) != 0 {
            if count < 4 as size_t {
                s4[count as usize] = i;
            } else if count > 4 as size_t {
                break;
            }
            count = count.wrapping_add(1);
        }
        i = i.wrapping_add(1);
    }
    let mut max_bits_counter: size_t = alphabet_size.wrapping_sub(1 as size_t);
    while max_bits_counter != 0 {
        max_bits_counter >>= 1 as ::core::ffi::c_int;
        max_bits = max_bits.wrapping_add(1);
    }
    if count <= 1 as size_t {
        BrotliWriteBits(4 as size_t, 1 as uint64_t, storage_ix, storage);
        BrotliWriteBits(
            max_bits,
            s4[0 as ::core::ffi::c_int as usize] as u64,
            storage_ix,
            storage,
        );
        *depth.offset(s4[0 as ::core::ffi::c_int as usize] as isize) = 0 as uint8_t;
        *bits.offset(s4[0 as ::core::ffi::c_int as usize] as isize) = 0 as uint16_t;
        return;
    }
    memset(
        depth as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        histogram_length.wrapping_mul(::core::mem::size_of::<uint8_t>() as size_t),
    );
    BrotliCreateHuffmanTree(
        histogram,
        histogram_length,
        15 as ::core::ffi::c_int,
        tree,
        depth,
    );
    BrotliConvertBitDepthsToSymbols(depth, histogram_length, bits);
    if count <= 4 as size_t {
        StoreSimpleHuffmanTree(
            depth,
            &raw mut s4 as *mut size_t,
            count,
            max_bits,
            storage_ix,
            storage,
        );
    } else {
        BrotliStoreHuffmanTree(depth, histogram_length, tree, storage_ix, storage);
    };
}
#[inline(always)]
unsafe extern "C" fn SortHuffmanTree(
    mut v0: *const HuffmanTree,
    mut v1: *const HuffmanTree,
) -> ::core::ffi::c_int {
    return if (*v0).total_count_ < (*v1).total_count_ {
        BROTLI_TRUE
    } else {
        BROTLI_FALSE
    };
}
#[no_mangle]
pub unsafe extern "C" fn BrotliBuildAndStoreHuffmanTreeFast(
    mut tree: *mut HuffmanTree,
    mut histogram: *const uint32_t,
    histogram_total: size_t,
    max_bits: size_t,
    mut depth: *mut uint8_t,
    mut bits: *mut uint16_t,
    mut storage_ix: *mut size_t,
    mut storage: *mut uint8_t,
) {
    let mut count: size_t = 0 as size_t;
    let mut symbols: [size_t; 4] = [0 as ::core::ffi::c_int as size_t, 0, 0, 0];
    let mut length: size_t = 0 as size_t;
    let mut total: size_t = histogram_total;
    while total != 0 as size_t {
        if *histogram.offset(length as isize) != 0 {
            if count < 4 as size_t {
                symbols[count as usize] = length;
            }
            count = count.wrapping_add(1);
            total = (total as ::core::ffi::c_ulong)
                .wrapping_sub(*histogram.offset(length as isize) as ::core::ffi::c_ulong)
                as size_t as size_t;
        }
        length = length.wrapping_add(1);
    }
    if count <= 1 as size_t {
        BrotliWriteBits(4 as size_t, 1 as uint64_t, storage_ix, storage);
        BrotliWriteBits(
            max_bits,
            symbols[0 as ::core::ffi::c_int as usize] as u64,
            storage_ix,
            storage,
        );
        *depth.offset(symbols[0 as ::core::ffi::c_int as usize] as isize) = 0 as uint8_t;
        *bits.offset(symbols[0 as ::core::ffi::c_int as usize] as isize) = 0 as uint16_t;
        return;
    }
    memset(
        depth as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        length.wrapping_mul(::core::mem::size_of::<uint8_t>() as size_t),
    );
    let mut count_limit: uint32_t = 0;
    count_limit = 1 as uint32_t;
    loop {
        let mut node: *mut HuffmanTree = tree;
        let mut l: size_t = 0;
        l = length;
        while l != 0 as size_t {
            l = l.wrapping_sub(1);
            if *histogram.offset(l as isize) != 0 {
                if (*histogram.offset(l as isize) >= count_limit) as ::core::ffi::c_int
                    as ::core::ffi::c_long
                    != 0
                {
                    InitHuffmanTree(
                        node,
                        *histogram.offset(l as isize),
                        -(1 as ::core::ffi::c_int) as int16_t,
                        l as int16_t,
                    );
                } else {
                    InitHuffmanTree(
                        node,
                        count_limit,
                        -(1 as ::core::ffi::c_int) as int16_t,
                        l as int16_t,
                    );
                }
                node = node.offset(1);
            }
        }
        let n: ::core::ffi::c_int =
            node.offset_from(tree) as ::core::ffi::c_long as ::core::ffi::c_int;
        let mut sentinel: HuffmanTree = HuffmanTree {
            total_count_: 0,
            index_left_: 0,
            index_right_or_value_: 0,
        };
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut j: ::core::ffi::c_int = n + 1 as ::core::ffi::c_int;
        let mut k: ::core::ffi::c_int = 0;
        SortHuffmanTreeItems(
            tree,
            n as size_t,
            Some(
                SortHuffmanTree
                    as unsafe extern "C" fn(
                        *const HuffmanTree,
                        *const HuffmanTree,
                    ) -> ::core::ffi::c_int,
            ),
        );
        InitHuffmanTree(
            &raw mut sentinel,
            BROTLI_UINT32_MAX,
            -(1 as ::core::ffi::c_int) as int16_t,
            -(1 as ::core::ffi::c_int) as int16_t,
        );
        let fresh1 = node;
        node = node.offset(1);
        *fresh1 = sentinel;
        let fresh2 = node;
        node = node.offset(1);
        *fresh2 = sentinel;
        k = n - 1 as ::core::ffi::c_int;
        while k > 0 as ::core::ffi::c_int {
            let mut left: ::core::ffi::c_int = 0;
            let mut right: ::core::ffi::c_int = 0;
            if (*tree.offset(i as isize)).total_count_ <= (*tree.offset(j as isize)).total_count_ {
                left = i;
                i += 1;
            } else {
                left = j;
                j += 1;
            }
            if (*tree.offset(i as isize)).total_count_ <= (*tree.offset(j as isize)).total_count_ {
                right = i;
                i += 1;
            } else {
                right = j;
                j += 1;
            }
            (*node.offset(-(1 as ::core::ffi::c_int) as isize)).total_count_ = (*tree
                .offset(left as isize))
            .total_count_
            .wrapping_add((*tree.offset(right as isize)).total_count_);
            (*node.offset(-(1 as ::core::ffi::c_int) as isize)).index_left_ = left as int16_t;
            (*node.offset(-(1 as ::core::ffi::c_int) as isize)).index_right_or_value_ =
                right as int16_t;
            let fresh3 = node;
            node = node.offset(1);
            *fresh3 = sentinel;
            k -= 1;
        }
        if BrotliSetDepth(
            2 as ::core::ffi::c_int * n - 1 as ::core::ffi::c_int,
            tree,
            depth,
            14 as ::core::ffi::c_int,
        ) != 0
        {
            break;
        }
        count_limit = (count_limit as ::core::ffi::c_uint).wrapping_mul(2 as ::core::ffi::c_uint)
            as uint32_t as uint32_t;
    }
    BrotliConvertBitDepthsToSymbols(depth, length, bits);
    if count <= 4 as size_t {
        let mut i_0: size_t = 0;
        BrotliWriteBits(2 as size_t, 1 as uint64_t, storage_ix, storage);
        BrotliWriteBits(
            2 as size_t,
            (count as uint64_t).wrapping_sub(1 as uint64_t),
            storage_ix,
            storage,
        );
        i_0 = 0 as size_t;
        while i_0 < count {
            let mut j_0: size_t = 0;
            j_0 = i_0.wrapping_add(1 as size_t);
            while j_0 < count {
                if (*depth.offset(symbols[j_0 as usize] as isize) as ::core::ffi::c_int)
                    < *depth.offset(symbols[i_0 as usize] as isize) as ::core::ffi::c_int
                {
                    let mut __brotli_swap_tmp: size_t = symbols[j_0 as usize];
                    symbols[j_0 as usize] = symbols[i_0 as usize];
                    symbols[i_0 as usize] = __brotli_swap_tmp;
                }
                j_0 = j_0.wrapping_add(1);
            }
            i_0 = i_0.wrapping_add(1);
        }
        if count == 2 as size_t {
            BrotliWriteBits(
                max_bits,
                symbols[0 as ::core::ffi::c_int as usize] as u64,
                storage_ix,
                storage,
            );
            BrotliWriteBits(
                max_bits,
                symbols[1 as ::core::ffi::c_int as usize] as u64,
                storage_ix,
                storage,
            );
        } else if count == 3 as size_t {
            BrotliWriteBits(
                max_bits,
                symbols[0 as ::core::ffi::c_int as usize] as u64,
                storage_ix,
                storage,
            );
            BrotliWriteBits(
                max_bits,
                symbols[1 as ::core::ffi::c_int as usize] as u64,
                storage_ix,
                storage,
            );
            BrotliWriteBits(
                max_bits,
                symbols[2 as ::core::ffi::c_int as usize] as u64,
                storage_ix,
                storage,
            );
        } else {
            BrotliWriteBits(
                max_bits,
                symbols[0 as ::core::ffi::c_int as usize] as u64,
                storage_ix,
                storage,
            );
            BrotliWriteBits(
                max_bits,
                symbols[1 as ::core::ffi::c_int as usize] as u64,
                storage_ix,
                storage,
            );
            BrotliWriteBits(
                max_bits,
                symbols[2 as ::core::ffi::c_int as usize] as u64,
                storage_ix,
                storage,
            );
            BrotliWriteBits(
                max_bits,
                symbols[3 as ::core::ffi::c_int as usize] as u64,
                storage_ix,
                storage,
            );
            BrotliWriteBits(
                1 as size_t,
                (if *depth.offset(symbols[0 as ::core::ffi::c_int as usize] as isize)
                    as ::core::ffi::c_int
                    == 1 as ::core::ffi::c_int
                {
                    1 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                }) as uint64_t,
                storage_ix,
                storage,
            );
        }
    } else {
        let mut previous_value: uint8_t = 8 as uint8_t;
        let mut i_1: size_t = 0;
        StoreStaticCodeLengthCode(storage_ix, storage);
        i_1 = 0 as size_t;
        while i_1 < length {
            let value: uint8_t = *depth.offset(i_1 as isize);
            let mut reps: size_t = 1 as size_t;
            let mut k_0: size_t = 0;
            k_0 = i_1.wrapping_add(1 as size_t);
            while k_0 < length
                && *depth.offset(k_0 as isize) as ::core::ffi::c_int == value as ::core::ffi::c_int
            {
                reps = reps.wrapping_add(1);
                k_0 = k_0.wrapping_add(1);
            }
            i_1 = (i_1 as ::core::ffi::c_ulong).wrapping_add(reps as ::core::ffi::c_ulong) as size_t
                as size_t;
            if value as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                BrotliWriteBits(
                    kZeroRepsDepth[reps as usize] as size_t,
                    kZeroRepsBits[reps as usize],
                    storage_ix,
                    storage,
                );
            } else {
                if previous_value as ::core::ffi::c_int != value as ::core::ffi::c_int {
                    BrotliWriteBits(
                        kCodeLengthDepth[value as usize] as size_t,
                        kCodeLengthBits[value as usize] as uint64_t,
                        storage_ix,
                        storage,
                    );
                    reps = reps.wrapping_sub(1);
                }
                if reps < 3 as size_t {
                    while reps != 0 as size_t {
                        reps = reps.wrapping_sub(1);
                        BrotliWriteBits(
                            kCodeLengthDepth[value as usize] as size_t,
                            kCodeLengthBits[value as usize] as uint64_t,
                            storage_ix,
                            storage,
                        );
                    }
                } else {
                    reps = (reps as ::core::ffi::c_ulong).wrapping_sub(3 as ::core::ffi::c_ulong)
                        as size_t as size_t;
                    BrotliWriteBits(
                        kNonZeroRepsDepth[reps as usize] as size_t,
                        kNonZeroRepsBits[reps as usize],
                        storage_ix,
                        storage,
                    );
                }
                previous_value = value;
            }
        }
    };
}
unsafe extern "C" fn IndexOf(
    mut v: *const uint8_t,
    mut v_size: size_t,
    mut value: uint8_t,
) -> size_t {
    let mut i: size_t = 0 as size_t;
    while i < v_size {
        if *v.offset(i as isize) as ::core::ffi::c_int == value as ::core::ffi::c_int {
            return i;
        }
        i = i.wrapping_add(1);
    }
    return i;
}
unsafe extern "C" fn MoveToFront(mut v: *mut uint8_t, mut index: size_t) {
    let mut value: uint8_t = *v.offset(index as isize);
    let mut i: size_t = 0;
    i = index;
    while i != 0 as size_t {
        *v.offset(i as isize) = *v.offset(i.wrapping_sub(1 as size_t) as isize);
        i = i.wrapping_sub(1);
    }
    *v.offset(0 as ::core::ffi::c_int as isize) = value;
}
unsafe extern "C" fn MoveToFrontTransform(
    mut v_in: *const uint32_t,
    v_size: size_t,
    mut v_out: *mut uint32_t,
) {
    let mut i: size_t = 0;
    let mut mtf: [uint8_t; 256] = [0; 256];
    let mut max_value: uint32_t = 0;
    if v_size == 0 as size_t {
        return;
    }
    max_value = *v_in.offset(0 as ::core::ffi::c_int as isize);
    i = 1 as size_t;
    while i < v_size {
        if *v_in.offset(i as isize) > max_value {
            max_value = *v_in.offset(i as isize);
        }
        i = i.wrapping_add(1);
    }
    i = 0 as size_t;
    while i <= max_value as size_t {
        mtf[i as usize] = i as uint8_t;
        i = i.wrapping_add(1);
    }
    let mut mtf_size: size_t = max_value.wrapping_add(1 as uint32_t) as size_t;
    i = 0 as size_t;
    while i < v_size {
        let mut index: size_t = IndexOf(
            &raw mut mtf as *mut uint8_t,
            mtf_size,
            *v_in.offset(i as isize) as uint8_t,
        );
        *v_out.offset(i as isize) = index as uint32_t;
        MoveToFront(&raw mut mtf as *mut uint8_t, index);
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn RunLengthCodeZeros(
    in_size: size_t,
    mut v: *mut uint32_t,
    mut out_size: *mut size_t,
    mut max_run_length_prefix: *mut uint32_t,
) {
    let mut max_reps: uint32_t = 0 as uint32_t;
    let mut i: size_t = 0;
    let mut max_prefix: uint32_t = 0;
    i = 0 as size_t;
    while i < in_size {
        let mut reps: uint32_t = 0 as uint32_t;
        while i < in_size && *v.offset(i as isize) != 0 as uint32_t {
            i = i.wrapping_add(1);
        }
        while i < in_size && *v.offset(i as isize) == 0 as uint32_t {
            reps = reps.wrapping_add(1);
            i = i.wrapping_add(1);
        }
        max_reps = brotli_max_uint32_t(reps, max_reps);
    }
    max_prefix = if max_reps > 0 as uint32_t {
        Log2FloorNonZero(max_reps as size_t)
    } else {
        0 as uint32_t
    };
    max_prefix = brotli_min_uint32_t(max_prefix, *max_run_length_prefix);
    *max_run_length_prefix = max_prefix;
    *out_size = 0 as size_t;
    i = 0 as size_t;
    while i < in_size {
        if *v.offset(i as isize) != 0 as uint32_t {
            *v.offset(*out_size as isize) =
                (*v.offset(i as isize)).wrapping_add(*max_run_length_prefix);
            i = i.wrapping_add(1);
            *out_size = (*out_size).wrapping_add(1);
        } else {
            let mut reps_0: uint32_t = 1 as uint32_t;
            let mut k: size_t = 0;
            k = i.wrapping_add(1 as size_t);
            while k < in_size && *v.offset(k as isize) == 0 as uint32_t {
                reps_0 = reps_0.wrapping_add(1);
                k = k.wrapping_add(1);
            }
            i = (i as ::core::ffi::c_ulong).wrapping_add(reps_0 as ::core::ffi::c_ulong) as size_t
                as size_t;
            while reps_0 != 0 as uint32_t {
                if reps_0 < (2 as uint32_t) << max_prefix {
                    let mut run_length_prefix: uint32_t = Log2FloorNonZero(reps_0 as size_t);
                    let extra_bits: uint32_t =
                        reps_0.wrapping_sub((1 as uint32_t) << run_length_prefix);
                    *v.offset(*out_size as isize) =
                        run_length_prefix.wrapping_add(extra_bits << 9 as ::core::ffi::c_int);
                    *out_size = (*out_size).wrapping_add(1);
                    break;
                } else {
                    let extra_bits_0: uint32_t =
                        ((1 as uint32_t) << max_prefix).wrapping_sub(1 as uint32_t);
                    *v.offset(*out_size as isize) =
                        max_prefix.wrapping_add(extra_bits_0 << 9 as ::core::ffi::c_int);
                    reps_0 = (reps_0 as ::core::ffi::c_uint).wrapping_sub(
                        ((2 as ::core::ffi::c_uint) << max_prefix)
                            .wrapping_sub(1 as ::core::ffi::c_uint),
                    ) as uint32_t as uint32_t;
                    *out_size = (*out_size).wrapping_add(1);
                }
            }
        }
    }
}
pub const SYMBOL_BITS: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
unsafe extern "C" fn EncodeContextMap(
    mut m: *mut MemoryManager,
    mut arena: *mut EncodeContextMapArena,
    mut context_map: *const uint32_t,
    mut context_map_size: size_t,
    mut num_clusters: size_t,
    mut tree: *mut HuffmanTree,
    mut storage_ix: *mut size_t,
    mut storage: *mut uint8_t,
) {
    let mut i: size_t = 0;
    let mut rle_symbols: *mut uint32_t = ::core::ptr::null_mut::<uint32_t>();
    let mut max_run_length_prefix: uint32_t = 6 as uint32_t;
    let mut num_rle_symbols: size_t = 0 as size_t;
    let histogram: *mut uint32_t = &raw mut (*arena).histogram as *mut uint32_t;
    let depths: *mut uint8_t = &raw mut (*arena).depths as *mut uint8_t;
    let bits: *mut uint16_t = &raw mut (*arena).bits as *mut uint16_t;
    StoreVarLenUint8(num_clusters.wrapping_sub(1 as size_t), storage_ix, storage);
    if num_clusters == 1 as size_t {
        return;
    }
    rle_symbols = if context_map_size > 0 as size_t {
        BrotliAllocate(
            m,
            context_map_size.wrapping_mul(::core::mem::size_of::<uint32_t>() as size_t),
        ) as *mut uint32_t
    } else {
        ::core::ptr::null_mut::<uint32_t>()
    };
    if 0 as ::core::ffi::c_int != 0 || 0 as ::core::ffi::c_int != 0 {
        return;
    }
    MoveToFrontTransform(context_map, context_map_size, rle_symbols);
    RunLengthCodeZeros(
        context_map_size,
        rle_symbols,
        &raw mut num_rle_symbols,
        &raw mut max_run_length_prefix,
    );
    memset(
        histogram as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<[uint32_t; 272]>() as size_t,
    );
    i = 0 as size_t;
    while i < num_rle_symbols {
        let ref mut fresh4 =
            *histogram.offset((*rle_symbols.offset(i as isize) & kSymbolMask) as isize);
        *fresh4 = (*fresh4).wrapping_add(1);
        i = i.wrapping_add(1);
    }
    let mut use_rle: ::core::ffi::c_int = if max_run_length_prefix > 0 as uint32_t {
        BROTLI_TRUE
    } else {
        BROTLI_FALSE
    };
    BrotliWriteBits(1 as size_t, use_rle as uint64_t, storage_ix, storage);
    if use_rle != 0 {
        BrotliWriteBits(
            4 as size_t,
            max_run_length_prefix.wrapping_sub(1 as uint32_t) as uint64_t,
            storage_ix,
            storage,
        );
    }
    BuildAndStoreHuffmanTree(
        histogram,
        num_clusters.wrapping_add(max_run_length_prefix as size_t),
        num_clusters.wrapping_add(max_run_length_prefix as size_t),
        tree,
        depths,
        bits,
        storage_ix,
        storage,
    );
    i = 0 as size_t;
    while i < num_rle_symbols {
        let rle_symbol: uint32_t = *rle_symbols.offset(i as isize) & kSymbolMask;
        let extra_bits_val: uint32_t = *rle_symbols.offset(i as isize) >> SYMBOL_BITS;
        BrotliWriteBits(
            *depths.offset(rle_symbol as isize) as size_t,
            *bits.offset(rle_symbol as isize) as uint64_t,
            storage_ix,
            storage,
        );
        if rle_symbol > 0 as uint32_t && rle_symbol <= max_run_length_prefix {
            BrotliWriteBits(
                rle_symbol as size_t,
                extra_bits_val as uint64_t,
                storage_ix,
                storage,
            );
        }
        i = i.wrapping_add(1);
    }
    BrotliWriteBits(1 as size_t, 1 as uint64_t, storage_ix, storage);
    BrotliFree(m, rle_symbols as *mut ::core::ffi::c_void);
    rle_symbols = ::core::ptr::null_mut::<uint32_t>();
}
#[inline(always)]
unsafe extern "C" fn StoreBlockSwitch(
    mut code: *mut BlockSplitCode,
    block_len: uint32_t,
    block_type: uint8_t,
    mut is_first_block: ::core::ffi::c_int,
    mut storage_ix: *mut size_t,
    mut storage: *mut uint8_t,
) {
    let mut typecode: size_t = NextBlockTypeCode(&raw mut (*code).type_code_calculator, block_type);
    let mut lencode: size_t = 0;
    let mut len_nextra: uint32_t = 0;
    let mut len_extra: uint32_t = 0;
    if is_first_block == 0 {
        BrotliWriteBits(
            (*code).type_depths[typecode as usize] as size_t,
            (*code).type_bits[typecode as usize] as uint64_t,
            storage_ix,
            storage,
        );
    }
    GetBlockLengthPrefixCode(
        block_len,
        &raw mut lencode,
        &raw mut len_nextra,
        &raw mut len_extra,
    );
    BrotliWriteBits(
        (*code).length_depths[lencode as usize] as size_t,
        (*code).length_bits[lencode as usize] as uint64_t,
        storage_ix,
        storage,
    );
    BrotliWriteBits(
        len_nextra as size_t,
        len_extra as uint64_t,
        storage_ix,
        storage,
    );
}
unsafe extern "C" fn BuildAndStoreBlockSplitCode(
    mut types: *const uint8_t,
    mut lengths: *const uint32_t,
    num_blocks: size_t,
    num_types: size_t,
    mut tree: *mut HuffmanTree,
    mut code: *mut BlockSplitCode,
    mut storage_ix: *mut size_t,
    mut storage: *mut uint8_t,
) {
    let mut type_histo: [uint32_t; 258] = [0; 258];
    let mut length_histo: [uint32_t; 26] = [0; 26];
    let mut i: size_t = 0;
    let mut type_code_calculator: BlockTypeCodeCalculator = BlockTypeCodeCalculator {
        last_type: 0,
        second_last_type: 0,
    };
    memset(
        &raw mut type_histo as *mut uint32_t as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        num_types
            .wrapping_add(2 as size_t)
            .wrapping_mul(::core::mem::size_of::<uint32_t>() as size_t),
    );
    memset(
        &raw mut length_histo as *mut uint32_t as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<[uint32_t; 26]>() as size_t,
    );
    InitBlockTypeCodeCalculator(&raw mut type_code_calculator);
    i = 0 as size_t;
    while i < num_blocks {
        let mut type_code: size_t =
            NextBlockTypeCode(&raw mut type_code_calculator, *types.offset(i as isize));
        if i != 0 as size_t {
            type_histo[type_code as usize] = type_histo[type_code as usize].wrapping_add(1);
        }
        length_histo[BlockLengthPrefixCode(*lengths.offset(i as isize)) as usize] = length_histo
            [BlockLengthPrefixCode(*lengths.offset(i as isize)) as usize]
            .wrapping_add(1);
        i = i.wrapping_add(1);
    }
    StoreVarLenUint8(num_types.wrapping_sub(1 as size_t), storage_ix, storage);
    if num_types > 1 as size_t {
        BuildAndStoreHuffmanTree(
            (&raw mut type_histo as *mut uint32_t).offset(0 as ::core::ffi::c_int as isize)
                as *mut uint32_t,
            num_types.wrapping_add(2 as size_t),
            num_types.wrapping_add(2 as size_t),
            tree,
            (&raw mut (*code).type_depths as *mut uint8_t).offset(0 as ::core::ffi::c_int as isize)
                as *mut uint8_t,
            (&raw mut (*code).type_bits as *mut uint16_t).offset(0 as ::core::ffi::c_int as isize)
                as *mut uint16_t,
            storage_ix,
            storage,
        );
        BuildAndStoreHuffmanTree(
            (&raw mut length_histo as *mut uint32_t).offset(0 as ::core::ffi::c_int as isize)
                as *mut uint32_t,
            BROTLI_NUM_BLOCK_LEN_SYMBOLS as size_t,
            BROTLI_NUM_BLOCK_LEN_SYMBOLS as size_t,
            tree,
            (&raw mut (*code).length_depths as *mut uint8_t)
                .offset(0 as ::core::ffi::c_int as isize) as *mut uint8_t,
            (&raw mut (*code).length_bits as *mut uint16_t).offset(0 as ::core::ffi::c_int as isize)
                as *mut uint16_t,
            storage_ix,
            storage,
        );
        StoreBlockSwitch(
            code,
            *lengths.offset(0 as ::core::ffi::c_int as isize),
            *types.offset(0 as ::core::ffi::c_int as isize),
            1 as ::core::ffi::c_int,
            storage_ix,
            storage,
        );
    }
}
unsafe extern "C" fn StoreTrivialContextMap(
    mut arena: *mut EncodeContextMapArena,
    mut num_types: size_t,
    mut context_bits: size_t,
    mut tree: *mut HuffmanTree,
    mut storage_ix: *mut size_t,
    mut storage: *mut uint8_t,
) {
    StoreVarLenUint8(num_types.wrapping_sub(1 as size_t), storage_ix, storage);
    if num_types > 1 as size_t {
        let mut repeat_code: size_t = context_bits.wrapping_sub(1 as size_t);
        let mut repeat_bits: size_t = ((1 as ::core::ffi::c_uint) << repeat_code)
            .wrapping_sub(1 as ::core::ffi::c_uint) as size_t;
        let mut alphabet_size: size_t = num_types.wrapping_add(repeat_code);
        let histogram: *mut uint32_t = &raw mut (*arena).histogram as *mut uint32_t;
        let depths: *mut uint8_t = &raw mut (*arena).depths as *mut uint8_t;
        let bits: *mut uint16_t = &raw mut (*arena).bits as *mut uint16_t;
        let mut i: size_t = 0;
        memset(
            histogram as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            alphabet_size.wrapping_mul(::core::mem::size_of::<uint32_t>() as size_t),
        );
        BrotliWriteBits(1 as size_t, 1 as uint64_t, storage_ix, storage);
        BrotliWriteBits(
            4 as size_t,
            (repeat_code as uint64_t).wrapping_sub(1 as uint64_t),
            storage_ix,
            storage,
        );
        *histogram.offset(repeat_code as isize) = num_types as uint32_t;
        *histogram.offset(0 as ::core::ffi::c_int as isize) = 1 as uint32_t;
        i = context_bits;
        while i < alphabet_size {
            *histogram.offset(i as isize) = 1 as uint32_t;
            i = i.wrapping_add(1);
        }
        BuildAndStoreHuffmanTree(
            histogram,
            alphabet_size,
            alphabet_size,
            tree,
            depths,
            bits,
            storage_ix,
            storage,
        );
        i = 0 as size_t;
        while i < num_types {
            let mut code: size_t = if i == 0 as size_t {
                0 as size_t
            } else {
                i.wrapping_add(context_bits).wrapping_sub(1 as size_t)
            };
            BrotliWriteBits(
                *depths.offset(code as isize) as size_t,
                *bits.offset(code as isize) as uint64_t,
                storage_ix,
                storage,
            );
            BrotliWriteBits(
                *depths.offset(repeat_code as isize) as size_t,
                *bits.offset(repeat_code as isize) as uint64_t,
                storage_ix,
                storage,
            );
            BrotliWriteBits(repeat_code, repeat_bits as uint64_t, storage_ix, storage);
            i = i.wrapping_add(1);
        }
        BrotliWriteBits(1 as size_t, 1 as uint64_t, storage_ix, storage);
    }
}
unsafe extern "C" fn InitBlockEncoder(
    mut self_0: *mut BlockEncoder,
    mut histogram_length: size_t,
    mut num_block_types: size_t,
    mut block_types: *const uint8_t,
    mut block_lengths: *const uint32_t,
    num_blocks: size_t,
) {
    (*self_0).histogram_length_ = histogram_length;
    (*self_0).num_block_types_ = num_block_types;
    (*self_0).block_types_ = block_types;
    (*self_0).block_lengths_ = block_lengths;
    (*self_0).num_blocks_ = num_blocks;
    InitBlockTypeCodeCalculator(&raw mut (*self_0).block_split_code_.type_code_calculator);
    (*self_0).block_ix_ = 0 as size_t;
    (*self_0).block_len_ = (if num_blocks == 0 as size_t {
        0 as uint32_t
    } else {
        *block_lengths.offset(0 as ::core::ffi::c_int as isize)
    }) as size_t;
    (*self_0).entropy_ix_ = 0 as size_t;
    (*self_0).depths_ = ::core::ptr::null_mut::<uint8_t>();
    (*self_0).bits_ = ::core::ptr::null_mut::<uint16_t>();
}
unsafe extern "C" fn CleanupBlockEncoder(mut m: *mut MemoryManager, mut self_0: *mut BlockEncoder) {
    BrotliFree(m, (*self_0).depths_ as *mut ::core::ffi::c_void);
    (*self_0).depths_ = ::core::ptr::null_mut::<uint8_t>();
    BrotliFree(m, (*self_0).bits_ as *mut ::core::ffi::c_void);
    (*self_0).bits_ = ::core::ptr::null_mut::<uint16_t>();
}
unsafe extern "C" fn BuildAndStoreBlockSwitchEntropyCodes(
    mut self_0: *mut BlockEncoder,
    mut tree: *mut HuffmanTree,
    mut storage_ix: *mut size_t,
    mut storage: *mut uint8_t,
) {
    BuildAndStoreBlockSplitCode(
        (*self_0).block_types_,
        (*self_0).block_lengths_,
        (*self_0).num_blocks_,
        (*self_0).num_block_types_,
        tree,
        &raw mut (*self_0).block_split_code_,
        storage_ix,
        storage,
    );
}
unsafe extern "C" fn StoreSymbol(
    mut self_0: *mut BlockEncoder,
    mut symbol: size_t,
    mut storage_ix: *mut size_t,
    mut storage: *mut uint8_t,
) {
    if (*self_0).block_len_ == 0 as size_t {
        (*self_0).block_ix_ = (*self_0).block_ix_.wrapping_add(1);
        let mut block_ix: size_t = (*self_0).block_ix_;
        let mut block_len: uint32_t = *(*self_0).block_lengths_.offset(block_ix as isize);
        let mut block_type: uint8_t = *(*self_0).block_types_.offset(block_ix as isize);
        (*self_0).block_len_ = block_len as size_t;
        (*self_0).entropy_ix_ = (block_type as size_t).wrapping_mul((*self_0).histogram_length_);
        StoreBlockSwitch(
            &raw mut (*self_0).block_split_code_,
            block_len,
            block_type,
            0 as ::core::ffi::c_int,
            storage_ix,
            storage,
        );
    }
    (*self_0).block_len_ = (*self_0).block_len_.wrapping_sub(1);
    let mut ix: size_t = (*self_0).entropy_ix_.wrapping_add(symbol);
    BrotliWriteBits(
        *(*self_0).depths_.offset(ix as isize) as size_t,
        *(*self_0).bits_.offset(ix as isize) as uint64_t,
        storage_ix,
        storage,
    );
}
unsafe extern "C" fn StoreSymbolWithContext(
    mut self_0: *mut BlockEncoder,
    mut symbol: size_t,
    mut context: size_t,
    mut context_map: *const uint32_t,
    mut storage_ix: *mut size_t,
    mut storage: *mut uint8_t,
    context_bits: size_t,
) {
    if (*self_0).block_len_ == 0 as size_t {
        (*self_0).block_ix_ = (*self_0).block_ix_.wrapping_add(1);
        let mut block_ix: size_t = (*self_0).block_ix_;
        let mut block_len: uint32_t = *(*self_0).block_lengths_.offset(block_ix as isize);
        let mut block_type: uint8_t = *(*self_0).block_types_.offset(block_ix as isize);
        (*self_0).block_len_ = block_len as size_t;
        (*self_0).entropy_ix_ = (block_type as size_t) << context_bits;
        StoreBlockSwitch(
            &raw mut (*self_0).block_split_code_,
            block_len,
            block_type,
            0 as ::core::ffi::c_int,
            storage_ix,
            storage,
        );
    }
    (*self_0).block_len_ = (*self_0).block_len_.wrapping_sub(1);
    let mut histo_ix: size_t =
        *context_map.offset((*self_0).entropy_ix_.wrapping_add(context) as isize) as size_t;
    let mut ix: size_t = histo_ix
        .wrapping_mul((*self_0).histogram_length_)
        .wrapping_add(symbol);
    BrotliWriteBits(
        *(*self_0).depths_.offset(ix as isize) as size_t,
        *(*self_0).bits_.offset(ix as isize) as uint64_t,
        storage_ix,
        storage,
    );
}
unsafe extern "C" fn JumpToByteBoundary(mut storage_ix: *mut size_t, mut storage: *mut uint8_t) {
    *storage_ix = (*storage_ix).wrapping_add(7 as size_t) & !(7 as ::core::ffi::c_uint) as size_t;
    *storage.offset((*storage_ix >> 3 as ::core::ffi::c_int) as isize) = 0 as uint8_t;
}
#[no_mangle]
pub unsafe extern "C" fn BrotliStoreMetaBlock(
    mut m: *mut MemoryManager,
    mut input: *const uint8_t,
    mut start_pos: size_t,
    mut length: size_t,
    mut mask: size_t,
    mut prev_byte: uint8_t,
    mut prev_byte2: uint8_t,
    mut is_last: ::core::ffi::c_int,
    mut params: *const BrotliEncoderParams,
    mut literal_context_mode: ContextType,
    mut commands: *const Command,
    mut n_commands: size_t,
    mut mb: *const MetaBlockSplit,
    mut storage_ix: *mut size_t,
    mut storage: *mut uint8_t,
) {
    let mut pos: size_t = start_pos;
    let mut i: size_t = 0;
    let mut num_distance_symbols: uint32_t = (*params).dist.alphabet_size_max;
    let mut num_effective_distance_symbols: uint32_t = (*params).dist.alphabet_size_limit;
    let mut tree: *mut HuffmanTree = ::core::ptr::null_mut::<HuffmanTree>();
    let mut literal_context_lut: ContextLut = (&raw const _kBrotliContextLookupTable
        as *const uint8_t)
        .offset(((literal_context_mode as ::core::ffi::c_uint) << 9 as ::core::ffi::c_int) as isize)
        as ContextLut;
    let mut arena: *mut StoreMetablockArena = ::core::ptr::null_mut::<StoreMetablockArena>();
    let mut literal_enc: *mut BlockEncoder = ::core::ptr::null_mut::<BlockEncoder>();
    let mut command_enc: *mut BlockEncoder = ::core::ptr::null_mut::<BlockEncoder>();
    let mut distance_enc: *mut BlockEncoder = ::core::ptr::null_mut::<BlockEncoder>();
    let mut dist: *const BrotliDistanceParams = &raw const (*params).dist;
    StoreCompressedMetaBlockHeader(is_last, length, storage_ix, storage);
    tree = if 2 as ::core::ffi::c_int * 704 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
        > 0 as ::core::ffi::c_int
    {
        BrotliAllocate(
            m,
            ((2 as ::core::ffi::c_int * 704 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                as size_t)
                .wrapping_mul(::core::mem::size_of::<HuffmanTree>() as size_t),
        ) as *mut HuffmanTree
    } else {
        ::core::ptr::null_mut::<HuffmanTree>()
    };
    arena = if 1 as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
        BrotliAllocate(
            m,
            (1 as size_t).wrapping_mul(::core::mem::size_of::<StoreMetablockArena>() as size_t),
        ) as *mut StoreMetablockArena
    } else {
        ::core::ptr::null_mut::<StoreMetablockArena>()
    };
    if 0 as ::core::ffi::c_int != 0 || 0 as ::core::ffi::c_int != 0 || 0 as ::core::ffi::c_int != 0
    {
        return;
    }
    literal_enc = &raw mut (*arena).literal_enc;
    command_enc = &raw mut (*arena).command_enc;
    distance_enc = &raw mut (*arena).distance_enc;
    InitBlockEncoder(
        literal_enc,
        BROTLI_NUM_LITERAL_SYMBOLS as size_t,
        (*mb).literal_split.num_types,
        (*mb).literal_split.types,
        (*mb).literal_split.lengths,
        (*mb).literal_split.num_blocks,
    );
    InitBlockEncoder(
        command_enc,
        BROTLI_NUM_COMMAND_SYMBOLS as size_t,
        (*mb).command_split.num_types,
        (*mb).command_split.types,
        (*mb).command_split.lengths,
        (*mb).command_split.num_blocks,
    );
    InitBlockEncoder(
        distance_enc,
        num_effective_distance_symbols as size_t,
        (*mb).distance_split.num_types,
        (*mb).distance_split.types,
        (*mb).distance_split.lengths,
        (*mb).distance_split.num_blocks,
    );
    BuildAndStoreBlockSwitchEntropyCodes(literal_enc, tree, storage_ix, storage);
    BuildAndStoreBlockSwitchEntropyCodes(command_enc, tree, storage_ix, storage);
    BuildAndStoreBlockSwitchEntropyCodes(distance_enc, tree, storage_ix, storage);
    BrotliWriteBits(
        2 as size_t,
        (*dist).distance_postfix_bits as uint64_t,
        storage_ix,
        storage,
    );
    BrotliWriteBits(
        4 as size_t,
        ((*dist).num_direct_distance_codes >> (*dist).distance_postfix_bits) as uint64_t,
        storage_ix,
        storage,
    );
    i = 0 as size_t;
    while i < (*mb).literal_split.num_types {
        BrotliWriteBits(
            2 as size_t,
            literal_context_mode as uint64_t,
            storage_ix,
            storage,
        );
        i = i.wrapping_add(1);
    }
    if (*mb).literal_context_map_size == 0 as size_t {
        StoreTrivialContextMap(
            &raw mut (*arena).context_map_arena,
            (*mb).literal_histograms_size,
            BROTLI_LITERAL_CONTEXT_BITS as size_t,
            tree,
            storage_ix,
            storage,
        );
    } else {
        EncodeContextMap(
            m,
            &raw mut (*arena).context_map_arena,
            (*mb).literal_context_map,
            (*mb).literal_context_map_size,
            (*mb).literal_histograms_size,
            tree,
            storage_ix,
            storage,
        );
        if 0 as ::core::ffi::c_int != 0 {
            return;
        }
    }
    if (*mb).distance_context_map_size == 0 as size_t {
        StoreTrivialContextMap(
            &raw mut (*arena).context_map_arena,
            (*mb).distance_histograms_size,
            BROTLI_DISTANCE_CONTEXT_BITS as size_t,
            tree,
            storage_ix,
            storage,
        );
    } else {
        EncodeContextMap(
            m,
            &raw mut (*arena).context_map_arena,
            (*mb).distance_context_map,
            (*mb).distance_context_map_size,
            (*mb).distance_histograms_size,
            tree,
            storage_ix,
            storage,
        );
        if 0 as ::core::ffi::c_int != 0 {
            return;
        }
    }
    BuildAndStoreEntropyCodesLiteral(
        m,
        literal_enc,
        (*mb).literal_histograms,
        (*mb).literal_histograms_size,
        BROTLI_NUM_LITERAL_SYMBOLS as size_t,
        tree,
        storage_ix,
        storage,
    );
    if 0 as ::core::ffi::c_int != 0 {
        return;
    }
    BuildAndStoreEntropyCodesCommand(
        m,
        command_enc,
        (*mb).command_histograms,
        (*mb).command_histograms_size,
        BROTLI_NUM_COMMAND_SYMBOLS as size_t,
        tree,
        storage_ix,
        storage,
    );
    if 0 as ::core::ffi::c_int != 0 {
        return;
    }
    BuildAndStoreEntropyCodesDistance(
        m,
        distance_enc,
        (*mb).distance_histograms,
        (*mb).distance_histograms_size,
        num_distance_symbols as size_t,
        tree,
        storage_ix,
        storage,
    );
    if 0 as ::core::ffi::c_int != 0 {
        return;
    }
    BrotliFree(m, tree as *mut ::core::ffi::c_void);
    tree = ::core::ptr::null_mut::<HuffmanTree>();
    i = 0 as size_t;
    while i < n_commands {
        let cmd: Command = *commands.offset(i as isize);
        let mut cmd_code: size_t = cmd.cmd_prefix_ as size_t;
        StoreSymbol(command_enc, cmd_code, storage_ix, storage);
        StoreCommandExtra(&raw const cmd, storage_ix, storage);
        if (*mb).literal_context_map_size == 0 as size_t {
            let mut j: size_t = 0;
            j = cmd.insert_len_ as size_t;
            while j != 0 as size_t {
                StoreSymbol(
                    literal_enc,
                    *input.offset((pos & mask) as isize) as size_t,
                    storage_ix,
                    storage,
                );
                pos = pos.wrapping_add(1);
                j = j.wrapping_sub(1);
            }
        } else {
            let mut j_0: size_t = 0;
            j_0 = cmd.insert_len_ as size_t;
            while j_0 != 0 as size_t {
                let mut context: size_t = (*literal_context_lut.offset(prev_byte as isize)
                    as ::core::ffi::c_int
                    | *literal_context_lut
                        .offset(256 as ::core::ffi::c_int as isize)
                        .offset(prev_byte2 as isize) as ::core::ffi::c_int)
                    as size_t;
                let mut literal: uint8_t = *input.offset((pos & mask) as isize);
                StoreSymbolWithContext(
                    literal_enc,
                    literal as size_t,
                    context,
                    (*mb).literal_context_map,
                    storage_ix,
                    storage,
                    BROTLI_LITERAL_CONTEXT_BITS as size_t,
                );
                prev_byte2 = prev_byte;
                prev_byte = literal;
                pos = pos.wrapping_add(1);
                j_0 = j_0.wrapping_sub(1);
            }
        }
        pos = (pos as ::core::ffi::c_ulong)
            .wrapping_add(CommandCopyLen(&raw const cmd) as ::core::ffi::c_ulong)
            as size_t as size_t;
        if CommandCopyLen(&raw const cmd) != 0 {
            prev_byte2 = *input.offset((pos.wrapping_sub(2 as size_t) & mask) as isize);
            prev_byte = *input.offset((pos.wrapping_sub(1 as size_t) & mask) as isize);
            if cmd.cmd_prefix_ as ::core::ffi::c_int >= 128 as ::core::ffi::c_int {
                let mut dist_code: size_t = (cmd.dist_prefix_ as ::core::ffi::c_int
                    & 0x3ff as ::core::ffi::c_int)
                    as size_t;
                let mut distnumextra: uint32_t = (cmd.dist_prefix_ as ::core::ffi::c_int
                    >> 10 as ::core::ffi::c_int)
                    as uint32_t;
                let mut distextra: uint64_t = cmd.dist_extra_ as uint64_t;
                if (*mb).distance_context_map_size == 0 as size_t {
                    StoreSymbol(distance_enc, dist_code, storage_ix, storage);
                } else {
                    let mut context_0: size_t = CommandDistanceContext(&raw const cmd) as size_t;
                    StoreSymbolWithContext(
                        distance_enc,
                        dist_code,
                        context_0,
                        (*mb).distance_context_map,
                        storage_ix,
                        storage,
                        BROTLI_DISTANCE_CONTEXT_BITS as size_t,
                    );
                }
                BrotliWriteBits(distnumextra as size_t, distextra, storage_ix, storage);
            }
        }
        i = i.wrapping_add(1);
    }
    CleanupBlockEncoder(m, distance_enc);
    CleanupBlockEncoder(m, command_enc);
    CleanupBlockEncoder(m, literal_enc);
    BrotliFree(m, arena as *mut ::core::ffi::c_void);
    arena = ::core::ptr::null_mut::<StoreMetablockArena>();
    if is_last != 0 {
        JumpToByteBoundary(storage_ix, storage);
    }
}
unsafe extern "C" fn BuildHistograms(
    mut input: *const uint8_t,
    mut start_pos: size_t,
    mut mask: size_t,
    mut commands: *const Command,
    mut n_commands: size_t,
    mut lit_histo: *mut HistogramLiteral,
    mut cmd_histo: *mut HistogramCommand,
    mut dist_histo: *mut HistogramDistance,
) {
    let mut pos: size_t = start_pos;
    let mut i: size_t = 0;
    i = 0 as size_t;
    while i < n_commands {
        let cmd: Command = *commands.offset(i as isize);
        let mut j: size_t = 0;
        HistogramAddCommand(cmd_histo, cmd.cmd_prefix_ as size_t);
        j = cmd.insert_len_ as size_t;
        while j != 0 as size_t {
            HistogramAddLiteral(lit_histo, *input.offset((pos & mask) as isize) as size_t);
            pos = pos.wrapping_add(1);
            j = j.wrapping_sub(1);
        }
        pos = (pos as ::core::ffi::c_ulong)
            .wrapping_add(CommandCopyLen(&raw const cmd) as ::core::ffi::c_ulong)
            as size_t as size_t;
        if CommandCopyLen(&raw const cmd) != 0
            && cmd.cmd_prefix_ as ::core::ffi::c_int >= 128 as ::core::ffi::c_int
        {
            HistogramAddDistance(
                dist_histo,
                (cmd.dist_prefix_ as ::core::ffi::c_int & 0x3ff as ::core::ffi::c_int) as size_t,
            );
        }
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn StoreDataWithHuffmanCodes(
    mut input: *const uint8_t,
    mut start_pos: size_t,
    mut mask: size_t,
    mut commands: *const Command,
    mut n_commands: size_t,
    mut lit_depth: *const uint8_t,
    mut lit_bits: *const uint16_t,
    mut cmd_depth: *const uint8_t,
    mut cmd_bits: *const uint16_t,
    mut dist_depth: *const uint8_t,
    mut dist_bits: *const uint16_t,
    mut storage_ix: *mut size_t,
    mut storage: *mut uint8_t,
) {
    let mut pos: size_t = start_pos;
    let mut i: size_t = 0;
    i = 0 as size_t;
    while i < n_commands {
        let cmd: Command = *commands.offset(i as isize);
        let cmd_code: size_t = cmd.cmd_prefix_ as size_t;
        let mut j: size_t = 0;
        BrotliWriteBits(
            *cmd_depth.offset(cmd_code as isize) as size_t,
            *cmd_bits.offset(cmd_code as isize) as uint64_t,
            storage_ix,
            storage,
        );
        StoreCommandExtra(&raw const cmd, storage_ix, storage);
        j = cmd.insert_len_ as size_t;
        while j != 0 as size_t {
            let literal: uint8_t = *input.offset((pos & mask) as isize);
            BrotliWriteBits(
                *lit_depth.offset(literal as isize) as size_t,
                *lit_bits.offset(literal as isize) as uint64_t,
                storage_ix,
                storage,
            );
            pos = pos.wrapping_add(1);
            j = j.wrapping_sub(1);
        }
        pos = (pos as ::core::ffi::c_ulong)
            .wrapping_add(CommandCopyLen(&raw const cmd) as ::core::ffi::c_ulong)
            as size_t as size_t;
        if CommandCopyLen(&raw const cmd) != 0
            && cmd.cmd_prefix_ as ::core::ffi::c_int >= 128 as ::core::ffi::c_int
        {
            let dist_code: size_t =
                (cmd.dist_prefix_ as ::core::ffi::c_int & 0x3ff as ::core::ffi::c_int) as size_t;
            let distnumextra: uint32_t =
                (cmd.dist_prefix_ as ::core::ffi::c_int >> 10 as ::core::ffi::c_int) as uint32_t;
            let distextra: uint32_t = cmd.dist_extra_;
            BrotliWriteBits(
                *dist_depth.offset(dist_code as isize) as size_t,
                *dist_bits.offset(dist_code as isize) as uint64_t,
                storage_ix,
                storage,
            );
            BrotliWriteBits(
                distnumextra as size_t,
                distextra as uint64_t,
                storage_ix,
                storage,
            );
        }
        i = i.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn BrotliStoreMetaBlockTrivial(
    mut m: *mut MemoryManager,
    mut input: *const uint8_t,
    mut start_pos: size_t,
    mut length: size_t,
    mut mask: size_t,
    mut is_last: ::core::ffi::c_int,
    mut params: *const BrotliEncoderParams,
    mut commands: *const Command,
    mut n_commands: size_t,
    mut storage_ix: *mut size_t,
    mut storage: *mut uint8_t,
) {
    let mut arena: *mut MetablockArena = if 1 as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
        BrotliAllocate(
            m,
            (1 as size_t).wrapping_mul(::core::mem::size_of::<MetablockArena>() as size_t),
        ) as *mut MetablockArena
    } else {
        ::core::ptr::null_mut::<MetablockArena>()
    };
    let mut num_distance_symbols: uint32_t = (*params).dist.alphabet_size_max;
    if 0 as ::core::ffi::c_int != 0 || 0 as ::core::ffi::c_int != 0 {
        return;
    }
    StoreCompressedMetaBlockHeader(is_last, length, storage_ix, storage);
    HistogramClearLiteral(&raw mut (*arena).lit_histo);
    HistogramClearCommand(&raw mut (*arena).cmd_histo);
    HistogramClearDistance(&raw mut (*arena).dist_histo);
    BuildHistograms(
        input,
        start_pos,
        mask,
        commands,
        n_commands,
        &raw mut (*arena).lit_histo,
        &raw mut (*arena).cmd_histo,
        &raw mut (*arena).dist_histo,
    );
    BrotliWriteBits(13 as size_t, 0 as uint64_t, storage_ix, storage);
    BuildAndStoreHuffmanTree(
        &raw mut (*arena).lit_histo.data_ as *mut uint32_t,
        BROTLI_NUM_LITERAL_SYMBOLS as size_t,
        BROTLI_NUM_LITERAL_SYMBOLS as size_t,
        &raw mut (*arena).tree as *mut HuffmanTree,
        &raw mut (*arena).lit_depth as *mut uint8_t,
        &raw mut (*arena).lit_bits as *mut uint16_t,
        storage_ix,
        storage,
    );
    BuildAndStoreHuffmanTree(
        &raw mut (*arena).cmd_histo.data_ as *mut uint32_t,
        BROTLI_NUM_COMMAND_SYMBOLS as size_t,
        BROTLI_NUM_COMMAND_SYMBOLS as size_t,
        &raw mut (*arena).tree as *mut HuffmanTree,
        &raw mut (*arena).cmd_depth as *mut uint8_t,
        &raw mut (*arena).cmd_bits as *mut uint16_t,
        storage_ix,
        storage,
    );
    BuildAndStoreHuffmanTree(
        &raw mut (*arena).dist_histo.data_ as *mut uint32_t,
        MAX_SIMPLE_DISTANCE_ALPHABET_SIZE as size_t,
        num_distance_symbols as size_t,
        &raw mut (*arena).tree as *mut HuffmanTree,
        &raw mut (*arena).dist_depth as *mut uint8_t,
        &raw mut (*arena).dist_bits as *mut uint16_t,
        storage_ix,
        storage,
    );
    StoreDataWithHuffmanCodes(
        input,
        start_pos,
        mask,
        commands,
        n_commands,
        &raw mut (*arena).lit_depth as *mut uint8_t,
        &raw mut (*arena).lit_bits as *mut uint16_t,
        &raw mut (*arena).cmd_depth as *mut uint8_t,
        &raw mut (*arena).cmd_bits as *mut uint16_t,
        &raw mut (*arena).dist_depth as *mut uint8_t,
        &raw mut (*arena).dist_bits as *mut uint16_t,
        storage_ix,
        storage,
    );
    BrotliFree(m, arena as *mut ::core::ffi::c_void);
    arena = ::core::ptr::null_mut::<MetablockArena>();
    if is_last != 0 {
        JumpToByteBoundary(storage_ix, storage);
    }
}
#[no_mangle]
pub unsafe extern "C" fn BrotliStoreMetaBlockFast(
    mut m: *mut MemoryManager,
    mut input: *const uint8_t,
    mut start_pos: size_t,
    mut length: size_t,
    mut mask: size_t,
    mut is_last: ::core::ffi::c_int,
    mut params: *const BrotliEncoderParams,
    mut commands: *const Command,
    mut n_commands: size_t,
    mut storage_ix: *mut size_t,
    mut storage: *mut uint8_t,
) {
    let mut arena: *mut MetablockArena = if 1 as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
        BrotliAllocate(
            m,
            (1 as size_t).wrapping_mul(::core::mem::size_of::<MetablockArena>() as size_t),
        ) as *mut MetablockArena
    } else {
        ::core::ptr::null_mut::<MetablockArena>()
    };
    let mut num_distance_symbols: uint32_t = (*params).dist.alphabet_size_max;
    let mut distance_alphabet_bits: uint32_t =
        Log2FloorNonZero(num_distance_symbols.wrapping_sub(1 as uint32_t) as size_t)
            .wrapping_add(1 as uint32_t);
    if 0 as ::core::ffi::c_int != 0 || 0 as ::core::ffi::c_int != 0 {
        return;
    }
    StoreCompressedMetaBlockHeader(is_last, length, storage_ix, storage);
    BrotliWriteBits(13 as size_t, 0 as uint64_t, storage_ix, storage);
    if n_commands <= 128 as size_t {
        let mut histogram: [uint32_t; 256] = [
            0 as ::core::ffi::c_int as uint32_t,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ];
        let mut pos: size_t = start_pos;
        let mut num_literals: size_t = 0 as size_t;
        let mut i: size_t = 0;
        i = 0 as size_t;
        while i < n_commands {
            let cmd: Command = *commands.offset(i as isize);
            let mut j: size_t = 0;
            j = cmd.insert_len_ as size_t;
            while j != 0 as size_t {
                histogram[*input.offset((pos & mask) as isize) as usize] =
                    histogram[*input.offset((pos & mask) as isize) as usize].wrapping_add(1);
                pos = pos.wrapping_add(1);
                j = j.wrapping_sub(1);
            }
            num_literals = (num_literals as ::core::ffi::c_ulong)
                .wrapping_add(cmd.insert_len_ as ::core::ffi::c_ulong)
                as size_t as size_t;
            pos = (pos as ::core::ffi::c_ulong)
                .wrapping_add(CommandCopyLen(&raw const cmd) as ::core::ffi::c_ulong)
                as size_t as size_t;
            i = i.wrapping_add(1);
        }
        BrotliBuildAndStoreHuffmanTreeFast(
            &raw mut (*arena).tree as *mut HuffmanTree,
            &raw mut histogram as *mut uint32_t,
            num_literals,
            8 as size_t,
            &raw mut (*arena).lit_depth as *mut uint8_t,
            &raw mut (*arena).lit_bits as *mut uint16_t,
            storage_ix,
            storage,
        );
        StoreStaticCommandHuffmanTree(storage_ix, storage);
        StoreStaticDistanceHuffmanTree(storage_ix, storage);
        StoreDataWithHuffmanCodes(
            input,
            start_pos,
            mask,
            commands,
            n_commands,
            &raw mut (*arena).lit_depth as *mut uint8_t,
            &raw mut (*arena).lit_bits as *mut uint16_t,
            &raw const kStaticCommandCodeDepth as *const uint8_t,
            &raw const kStaticCommandCodeBits as *const uint16_t,
            &raw const kStaticDistanceCodeDepth as *const uint8_t,
            &raw const kStaticDistanceCodeBits as *const uint16_t,
            storage_ix,
            storage,
        );
    } else {
        HistogramClearLiteral(&raw mut (*arena).lit_histo);
        HistogramClearCommand(&raw mut (*arena).cmd_histo);
        HistogramClearDistance(&raw mut (*arena).dist_histo);
        BuildHistograms(
            input,
            start_pos,
            mask,
            commands,
            n_commands,
            &raw mut (*arena).lit_histo,
            &raw mut (*arena).cmd_histo,
            &raw mut (*arena).dist_histo,
        );
        BrotliBuildAndStoreHuffmanTreeFast(
            &raw mut (*arena).tree as *mut HuffmanTree,
            &raw mut (*arena).lit_histo.data_ as *mut uint32_t,
            (*arena).lit_histo.total_count_,
            8 as size_t,
            &raw mut (*arena).lit_depth as *mut uint8_t,
            &raw mut (*arena).lit_bits as *mut uint16_t,
            storage_ix,
            storage,
        );
        BrotliBuildAndStoreHuffmanTreeFast(
            &raw mut (*arena).tree as *mut HuffmanTree,
            &raw mut (*arena).cmd_histo.data_ as *mut uint32_t,
            (*arena).cmd_histo.total_count_,
            10 as size_t,
            &raw mut (*arena).cmd_depth as *mut uint8_t,
            &raw mut (*arena).cmd_bits as *mut uint16_t,
            storage_ix,
            storage,
        );
        BrotliBuildAndStoreHuffmanTreeFast(
            &raw mut (*arena).tree as *mut HuffmanTree,
            &raw mut (*arena).dist_histo.data_ as *mut uint32_t,
            (*arena).dist_histo.total_count_,
            distance_alphabet_bits as size_t,
            &raw mut (*arena).dist_depth as *mut uint8_t,
            &raw mut (*arena).dist_bits as *mut uint16_t,
            storage_ix,
            storage,
        );
        StoreDataWithHuffmanCodes(
            input,
            start_pos,
            mask,
            commands,
            n_commands,
            &raw mut (*arena).lit_depth as *mut uint8_t,
            &raw mut (*arena).lit_bits as *mut uint16_t,
            &raw mut (*arena).cmd_depth as *mut uint8_t,
            &raw mut (*arena).cmd_bits as *mut uint16_t,
            &raw mut (*arena).dist_depth as *mut uint8_t,
            &raw mut (*arena).dist_bits as *mut uint16_t,
            storage_ix,
            storage,
        );
    }
    BrotliFree(m, arena as *mut ::core::ffi::c_void);
    arena = ::core::ptr::null_mut::<MetablockArena>();
    if is_last != 0 {
        JumpToByteBoundary(storage_ix, storage);
    }
}
#[no_mangle]
pub unsafe extern "C" fn BrotliStoreUncompressedMetaBlock(
    mut is_final_block: ::core::ffi::c_int,
    mut input: *const uint8_t,
    mut position: size_t,
    mut mask: size_t,
    mut len: size_t,
    mut storage_ix: *mut size_t,
    mut storage: *mut uint8_t,
) {
    let mut masked_pos: size_t = position & mask;
    BrotliStoreUncompressedMetaBlockHeader(len, storage_ix, storage);
    JumpToByteBoundary(storage_ix, storage);
    if masked_pos.wrapping_add(len) > mask.wrapping_add(1 as size_t) {
        let mut len1: size_t = mask.wrapping_add(1 as size_t).wrapping_sub(masked_pos);
        memcpy(
            storage.offset((*storage_ix >> 3 as ::core::ffi::c_int) as isize) as *mut uint8_t
                as *mut ::core::ffi::c_void,
            input.offset(masked_pos as isize) as *const uint8_t as *const ::core::ffi::c_void,
            len1,
        );
        *storage_ix = (*storage_ix as ::core::ffi::c_ulong)
            .wrapping_add((len1 << 3 as ::core::ffi::c_int) as ::core::ffi::c_ulong)
            as size_t as size_t;
        len = (len as ::core::ffi::c_ulong).wrapping_sub(len1 as ::core::ffi::c_ulong) as size_t
            as size_t;
        masked_pos = 0 as size_t;
    }
    memcpy(
        storage.offset((*storage_ix >> 3 as ::core::ffi::c_int) as isize) as *mut uint8_t
            as *mut ::core::ffi::c_void,
        input.offset(masked_pos as isize) as *const uint8_t as *const ::core::ffi::c_void,
        len,
    );
    *storage_ix = (*storage_ix as ::core::ffi::c_ulong)
        .wrapping_add((len << 3 as ::core::ffi::c_int) as ::core::ffi::c_ulong)
        as size_t as size_t;
    BrotliWriteBitsPrepareStorage(*storage_ix, storage);
    if is_final_block != 0 {
        BrotliWriteBits(1 as size_t, 1 as uint64_t, storage_ix, storage);
        BrotliWriteBits(1 as size_t, 1 as uint64_t, storage_ix, storage);
        JumpToByteBoundary(storage_ix, storage);
    }
}
#[inline(always)]
unsafe extern "C" fn BrotliWriteBits(
    mut n_bits: size_t,
    mut bits: uint64_t,
    mut pos: *mut size_t,
    mut array: *mut uint8_t,
) {
    let mut p: *mut uint8_t =
        array.offset((*pos >> 3 as ::core::ffi::c_int) as isize) as *mut uint8_t;
    let mut v: uint64_t = *p as uint64_t;
    v = (v as ::core::ffi::c_ulong | (bits << (*pos & 7 as size_t)) as ::core::ffi::c_ulong)
        as uint64_t;
    BrotliUnalignedWrite64(p as *mut ::core::ffi::c_void, v);
    *pos = (*pos as ::core::ffi::c_ulong).wrapping_add(n_bits as ::core::ffi::c_ulong) as size_t
        as size_t;
}
#[inline(always)]
unsafe extern "C" fn BrotliWriteBitsPrepareStorage(mut pos: size_t, mut array: *mut uint8_t) {
    *array.offset((pos >> 3 as ::core::ffi::c_int) as isize) = 0 as uint8_t;
}
static mut kCodeLengthDepth: [uint8_t; 18] = [
    4 as ::core::ffi::c_int as uint8_t,
    4 as ::core::ffi::c_int as uint8_t,
    4 as ::core::ffi::c_int as uint8_t,
    4 as ::core::ffi::c_int as uint8_t,
    4 as ::core::ffi::c_int as uint8_t,
    4 as ::core::ffi::c_int as uint8_t,
    4 as ::core::ffi::c_int as uint8_t,
    4 as ::core::ffi::c_int as uint8_t,
    4 as ::core::ffi::c_int as uint8_t,
    4 as ::core::ffi::c_int as uint8_t,
    4 as ::core::ffi::c_int as uint8_t,
    4 as ::core::ffi::c_int as uint8_t,
    4 as ::core::ffi::c_int as uint8_t,
    5 as ::core::ffi::c_int as uint8_t,
    5 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
    4 as ::core::ffi::c_int as uint8_t,
    4 as ::core::ffi::c_int as uint8_t,
];
static mut kStaticCommandCodeDepth: [uint8_t; 704] = [
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
];
static mut kStaticDistanceCodeDepth: [uint8_t; 64] = [
    6 as ::core::ffi::c_int as uint8_t,
    6 as ::core::ffi::c_int as uint8_t,
    6 as ::core::ffi::c_int as uint8_t,
    6 as ::core::ffi::c_int as uint8_t,
    6 as ::core::ffi::c_int as uint8_t,
    6 as ::core::ffi::c_int as uint8_t,
    6 as ::core::ffi::c_int as uint8_t,
    6 as ::core::ffi::c_int as uint8_t,
    6 as ::core::ffi::c_int as uint8_t,
    6 as ::core::ffi::c_int as uint8_t,
    6 as ::core::ffi::c_int as uint8_t,
    6 as ::core::ffi::c_int as uint8_t,
    6 as ::core::ffi::c_int as uint8_t,
    6 as ::core::ffi::c_int as uint8_t,
    6 as ::core::ffi::c_int as uint8_t,
    6 as ::core::ffi::c_int as uint8_t,
    6 as ::core::ffi::c_int as uint8_t,
    6 as ::core::ffi::c_int as uint8_t,
    6 as ::core::ffi::c_int as uint8_t,
    6 as ::core::ffi::c_int as uint8_t,
    6 as ::core::ffi::c_int as uint8_t,
    6 as ::core::ffi::c_int as uint8_t,
    6 as ::core::ffi::c_int as uint8_t,
    6 as ::core::ffi::c_int as uint8_t,
    6 as ::core::ffi::c_int as uint8_t,
    6 as ::core::ffi::c_int as uint8_t,
    6 as ::core::ffi::c_int as uint8_t,
    6 as ::core::ffi::c_int as uint8_t,
    6 as ::core::ffi::c_int as uint8_t,
    6 as ::core::ffi::c_int as uint8_t,
    6 as ::core::ffi::c_int as uint8_t,
    6 as ::core::ffi::c_int as uint8_t,
    6 as ::core::ffi::c_int as uint8_t,
    6 as ::core::ffi::c_int as uint8_t,
    6 as ::core::ffi::c_int as uint8_t,
    6 as ::core::ffi::c_int as uint8_t,
    6 as ::core::ffi::c_int as uint8_t,
    6 as ::core::ffi::c_int as uint8_t,
    6 as ::core::ffi::c_int as uint8_t,
    6 as ::core::ffi::c_int as uint8_t,
    6 as ::core::ffi::c_int as uint8_t,
    6 as ::core::ffi::c_int as uint8_t,
    6 as ::core::ffi::c_int as uint8_t,
    6 as ::core::ffi::c_int as uint8_t,
    6 as ::core::ffi::c_int as uint8_t,
    6 as ::core::ffi::c_int as uint8_t,
    6 as ::core::ffi::c_int as uint8_t,
    6 as ::core::ffi::c_int as uint8_t,
    6 as ::core::ffi::c_int as uint8_t,
    6 as ::core::ffi::c_int as uint8_t,
    6 as ::core::ffi::c_int as uint8_t,
    6 as ::core::ffi::c_int as uint8_t,
    6 as ::core::ffi::c_int as uint8_t,
    6 as ::core::ffi::c_int as uint8_t,
    6 as ::core::ffi::c_int as uint8_t,
    6 as ::core::ffi::c_int as uint8_t,
    6 as ::core::ffi::c_int as uint8_t,
    6 as ::core::ffi::c_int as uint8_t,
    6 as ::core::ffi::c_int as uint8_t,
    6 as ::core::ffi::c_int as uint8_t,
    6 as ::core::ffi::c_int as uint8_t,
    6 as ::core::ffi::c_int as uint8_t,
    6 as ::core::ffi::c_int as uint8_t,
    6 as ::core::ffi::c_int as uint8_t,
];
static mut kCodeLengthBits: [uint32_t; 18] = [
    0 as ::core::ffi::c_int as uint32_t,
    8 as ::core::ffi::c_int as uint32_t,
    4 as ::core::ffi::c_int as uint32_t,
    12 as ::core::ffi::c_int as uint32_t,
    2 as ::core::ffi::c_int as uint32_t,
    10 as ::core::ffi::c_int as uint32_t,
    6 as ::core::ffi::c_int as uint32_t,
    14 as ::core::ffi::c_int as uint32_t,
    1 as ::core::ffi::c_int as uint32_t,
    9 as ::core::ffi::c_int as uint32_t,
    5 as ::core::ffi::c_int as uint32_t,
    13 as ::core::ffi::c_int as uint32_t,
    3 as ::core::ffi::c_int as uint32_t,
    15 as ::core::ffi::c_int as uint32_t,
    31 as ::core::ffi::c_int as uint32_t,
    0 as ::core::ffi::c_int as uint32_t,
    11 as ::core::ffi::c_int as uint32_t,
    7 as ::core::ffi::c_int as uint32_t,
];
#[inline(always)]
unsafe extern "C" fn StoreStaticCodeLengthCode(
    mut storage_ix: *mut size_t,
    mut storage: *mut uint8_t,
) {
    BrotliWriteBits(
        40 as size_t,
        (0xff as ::core::ffi::c_uint as uint64_t) << 32 as ::core::ffi::c_int
            | 0x55555554 as uint64_t,
        storage_ix,
        storage,
    );
}
static mut kZeroRepsBits: [uint64_t; 704] = [
    0 as ::core::ffi::c_int as uint64_t,
    0 as ::core::ffi::c_int as uint64_t,
    0 as ::core::ffi::c_int as uint64_t,
    0x7 as ::core::ffi::c_int as uint64_t,
    0x17 as ::core::ffi::c_int as uint64_t,
    0x27 as ::core::ffi::c_int as uint64_t,
    0x37 as ::core::ffi::c_int as uint64_t,
    0x47 as ::core::ffi::c_int as uint64_t,
    0x57 as ::core::ffi::c_int as uint64_t,
    0x67 as ::core::ffi::c_int as uint64_t,
    0x77 as ::core::ffi::c_int as uint64_t,
    0x770 as ::core::ffi::c_int as uint64_t,
    0xb87 as ::core::ffi::c_int as uint64_t,
    0x1387 as ::core::ffi::c_int as uint64_t,
    0x1b87 as ::core::ffi::c_int as uint64_t,
    0x2387 as ::core::ffi::c_int as uint64_t,
    0x2b87 as ::core::ffi::c_int as uint64_t,
    0x3387 as ::core::ffi::c_int as uint64_t,
    0x3b87 as ::core::ffi::c_int as uint64_t,
    0x397 as ::core::ffi::c_int as uint64_t,
    0xb97 as ::core::ffi::c_int as uint64_t,
    0x1397 as ::core::ffi::c_int as uint64_t,
    0x1b97 as ::core::ffi::c_int as uint64_t,
    0x2397 as ::core::ffi::c_int as uint64_t,
    0x2b97 as ::core::ffi::c_int as uint64_t,
    0x3397 as ::core::ffi::c_int as uint64_t,
    0x3b97 as ::core::ffi::c_int as uint64_t,
    0x3a7 as ::core::ffi::c_int as uint64_t,
    0xba7 as ::core::ffi::c_int as uint64_t,
    0x13a7 as ::core::ffi::c_int as uint64_t,
    0x1ba7 as ::core::ffi::c_int as uint64_t,
    0x23a7 as ::core::ffi::c_int as uint64_t,
    0x2ba7 as ::core::ffi::c_int as uint64_t,
    0x33a7 as ::core::ffi::c_int as uint64_t,
    0x3ba7 as ::core::ffi::c_int as uint64_t,
    0x3b7 as ::core::ffi::c_int as uint64_t,
    0xbb7 as ::core::ffi::c_int as uint64_t,
    0x13b7 as ::core::ffi::c_int as uint64_t,
    0x1bb7 as ::core::ffi::c_int as uint64_t,
    0x23b7 as ::core::ffi::c_int as uint64_t,
    0x2bb7 as ::core::ffi::c_int as uint64_t,
    0x33b7 as ::core::ffi::c_int as uint64_t,
    0x3bb7 as ::core::ffi::c_int as uint64_t,
    0x3c7 as ::core::ffi::c_int as uint64_t,
    0xbc7 as ::core::ffi::c_int as uint64_t,
    0x13c7 as ::core::ffi::c_int as uint64_t,
    0x1bc7 as ::core::ffi::c_int as uint64_t,
    0x23c7 as ::core::ffi::c_int as uint64_t,
    0x2bc7 as ::core::ffi::c_int as uint64_t,
    0x33c7 as ::core::ffi::c_int as uint64_t,
    0x3bc7 as ::core::ffi::c_int as uint64_t,
    0x3d7 as ::core::ffi::c_int as uint64_t,
    0xbd7 as ::core::ffi::c_int as uint64_t,
    0x13d7 as ::core::ffi::c_int as uint64_t,
    0x1bd7 as ::core::ffi::c_int as uint64_t,
    0x23d7 as ::core::ffi::c_int as uint64_t,
    0x2bd7 as ::core::ffi::c_int as uint64_t,
    0x33d7 as ::core::ffi::c_int as uint64_t,
    0x3bd7 as ::core::ffi::c_int as uint64_t,
    0x3e7 as ::core::ffi::c_int as uint64_t,
    0xbe7 as ::core::ffi::c_int as uint64_t,
    0x13e7 as ::core::ffi::c_int as uint64_t,
    0x1be7 as ::core::ffi::c_int as uint64_t,
    0x23e7 as ::core::ffi::c_int as uint64_t,
    0x2be7 as ::core::ffi::c_int as uint64_t,
    0x33e7 as ::core::ffi::c_int as uint64_t,
    0x3be7 as ::core::ffi::c_int as uint64_t,
    0x3f7 as ::core::ffi::c_int as uint64_t,
    0xbf7 as ::core::ffi::c_int as uint64_t,
    0x13f7 as ::core::ffi::c_int as uint64_t,
    0x1bf7 as ::core::ffi::c_int as uint64_t,
    0x23f7 as ::core::ffi::c_int as uint64_t,
    0x2bf7 as ::core::ffi::c_int as uint64_t,
    0x33f7 as ::core::ffi::c_int as uint64_t,
    0x3bf7 as ::core::ffi::c_int as uint64_t,
    0x1c387 as ::core::ffi::c_int as uint64_t,
    0x5c387 as ::core::ffi::c_int as uint64_t,
    0x9c387 as ::core::ffi::c_int as uint64_t,
    0xdc387 as ::core::ffi::c_int as uint64_t,
    0x11c387 as ::core::ffi::c_int as uint64_t,
    0x15c387 as ::core::ffi::c_int as uint64_t,
    0x19c387 as ::core::ffi::c_int as uint64_t,
    0x1dc387 as ::core::ffi::c_int as uint64_t,
    0x1cb87 as ::core::ffi::c_int as uint64_t,
    0x5cb87 as ::core::ffi::c_int as uint64_t,
    0x9cb87 as ::core::ffi::c_int as uint64_t,
    0xdcb87 as ::core::ffi::c_int as uint64_t,
    0x11cb87 as ::core::ffi::c_int as uint64_t,
    0x15cb87 as ::core::ffi::c_int as uint64_t,
    0x19cb87 as ::core::ffi::c_int as uint64_t,
    0x1dcb87 as ::core::ffi::c_int as uint64_t,
    0x1d387 as ::core::ffi::c_int as uint64_t,
    0x5d387 as ::core::ffi::c_int as uint64_t,
    0x9d387 as ::core::ffi::c_int as uint64_t,
    0xdd387 as ::core::ffi::c_int as uint64_t,
    0x11d387 as ::core::ffi::c_int as uint64_t,
    0x15d387 as ::core::ffi::c_int as uint64_t,
    0x19d387 as ::core::ffi::c_int as uint64_t,
    0x1dd387 as ::core::ffi::c_int as uint64_t,
    0x1db87 as ::core::ffi::c_int as uint64_t,
    0x5db87 as ::core::ffi::c_int as uint64_t,
    0x9db87 as ::core::ffi::c_int as uint64_t,
    0xddb87 as ::core::ffi::c_int as uint64_t,
    0x11db87 as ::core::ffi::c_int as uint64_t,
    0x15db87 as ::core::ffi::c_int as uint64_t,
    0x19db87 as ::core::ffi::c_int as uint64_t,
    0x1ddb87 as ::core::ffi::c_int as uint64_t,
    0x1e387 as ::core::ffi::c_int as uint64_t,
    0x5e387 as ::core::ffi::c_int as uint64_t,
    0x9e387 as ::core::ffi::c_int as uint64_t,
    0xde387 as ::core::ffi::c_int as uint64_t,
    0x11e387 as ::core::ffi::c_int as uint64_t,
    0x15e387 as ::core::ffi::c_int as uint64_t,
    0x19e387 as ::core::ffi::c_int as uint64_t,
    0x1de387 as ::core::ffi::c_int as uint64_t,
    0x1eb87 as ::core::ffi::c_int as uint64_t,
    0x5eb87 as ::core::ffi::c_int as uint64_t,
    0x9eb87 as ::core::ffi::c_int as uint64_t,
    0xdeb87 as ::core::ffi::c_int as uint64_t,
    0x11eb87 as ::core::ffi::c_int as uint64_t,
    0x15eb87 as ::core::ffi::c_int as uint64_t,
    0x19eb87 as ::core::ffi::c_int as uint64_t,
    0x1deb87 as ::core::ffi::c_int as uint64_t,
    0x1f387 as ::core::ffi::c_int as uint64_t,
    0x5f387 as ::core::ffi::c_int as uint64_t,
    0x9f387 as ::core::ffi::c_int as uint64_t,
    0xdf387 as ::core::ffi::c_int as uint64_t,
    0x11f387 as ::core::ffi::c_int as uint64_t,
    0x15f387 as ::core::ffi::c_int as uint64_t,
    0x19f387 as ::core::ffi::c_int as uint64_t,
    0x1df387 as ::core::ffi::c_int as uint64_t,
    0x1fb87 as ::core::ffi::c_int as uint64_t,
    0x5fb87 as ::core::ffi::c_int as uint64_t,
    0x9fb87 as ::core::ffi::c_int as uint64_t,
    0xdfb87 as ::core::ffi::c_int as uint64_t,
    0x11fb87 as ::core::ffi::c_int as uint64_t,
    0x15fb87 as ::core::ffi::c_int as uint64_t,
    0x19fb87 as ::core::ffi::c_int as uint64_t,
    0x1dfb87 as ::core::ffi::c_int as uint64_t,
    0x1c397 as ::core::ffi::c_int as uint64_t,
    0x5c397 as ::core::ffi::c_int as uint64_t,
    0x9c397 as ::core::ffi::c_int as uint64_t,
    0xdc397 as ::core::ffi::c_int as uint64_t,
    0x11c397 as ::core::ffi::c_int as uint64_t,
    0x15c397 as ::core::ffi::c_int as uint64_t,
    0x19c397 as ::core::ffi::c_int as uint64_t,
    0x1dc397 as ::core::ffi::c_int as uint64_t,
    0x1cb97 as ::core::ffi::c_int as uint64_t,
    0x5cb97 as ::core::ffi::c_int as uint64_t,
    0x9cb97 as ::core::ffi::c_int as uint64_t,
    0xdcb97 as ::core::ffi::c_int as uint64_t,
    0x11cb97 as ::core::ffi::c_int as uint64_t,
    0x15cb97 as ::core::ffi::c_int as uint64_t,
    0x19cb97 as ::core::ffi::c_int as uint64_t,
    0x1dcb97 as ::core::ffi::c_int as uint64_t,
    0x1d397 as ::core::ffi::c_int as uint64_t,
    0x5d397 as ::core::ffi::c_int as uint64_t,
    0x9d397 as ::core::ffi::c_int as uint64_t,
    0xdd397 as ::core::ffi::c_int as uint64_t,
    0x11d397 as ::core::ffi::c_int as uint64_t,
    0x15d397 as ::core::ffi::c_int as uint64_t,
    0x19d397 as ::core::ffi::c_int as uint64_t,
    0x1dd397 as ::core::ffi::c_int as uint64_t,
    0x1db97 as ::core::ffi::c_int as uint64_t,
    0x5db97 as ::core::ffi::c_int as uint64_t,
    0x9db97 as ::core::ffi::c_int as uint64_t,
    0xddb97 as ::core::ffi::c_int as uint64_t,
    0x11db97 as ::core::ffi::c_int as uint64_t,
    0x15db97 as ::core::ffi::c_int as uint64_t,
    0x19db97 as ::core::ffi::c_int as uint64_t,
    0x1ddb97 as ::core::ffi::c_int as uint64_t,
    0x1e397 as ::core::ffi::c_int as uint64_t,
    0x5e397 as ::core::ffi::c_int as uint64_t,
    0x9e397 as ::core::ffi::c_int as uint64_t,
    0xde397 as ::core::ffi::c_int as uint64_t,
    0x11e397 as ::core::ffi::c_int as uint64_t,
    0x15e397 as ::core::ffi::c_int as uint64_t,
    0x19e397 as ::core::ffi::c_int as uint64_t,
    0x1de397 as ::core::ffi::c_int as uint64_t,
    0x1eb97 as ::core::ffi::c_int as uint64_t,
    0x5eb97 as ::core::ffi::c_int as uint64_t,
    0x9eb97 as ::core::ffi::c_int as uint64_t,
    0xdeb97 as ::core::ffi::c_int as uint64_t,
    0x11eb97 as ::core::ffi::c_int as uint64_t,
    0x15eb97 as ::core::ffi::c_int as uint64_t,
    0x19eb97 as ::core::ffi::c_int as uint64_t,
    0x1deb97 as ::core::ffi::c_int as uint64_t,
    0x1f397 as ::core::ffi::c_int as uint64_t,
    0x5f397 as ::core::ffi::c_int as uint64_t,
    0x9f397 as ::core::ffi::c_int as uint64_t,
    0xdf397 as ::core::ffi::c_int as uint64_t,
    0x11f397 as ::core::ffi::c_int as uint64_t,
    0x15f397 as ::core::ffi::c_int as uint64_t,
    0x19f397 as ::core::ffi::c_int as uint64_t,
    0x1df397 as ::core::ffi::c_int as uint64_t,
    0x1fb97 as ::core::ffi::c_int as uint64_t,
    0x5fb97 as ::core::ffi::c_int as uint64_t,
    0x9fb97 as ::core::ffi::c_int as uint64_t,
    0xdfb97 as ::core::ffi::c_int as uint64_t,
    0x11fb97 as ::core::ffi::c_int as uint64_t,
    0x15fb97 as ::core::ffi::c_int as uint64_t,
    0x19fb97 as ::core::ffi::c_int as uint64_t,
    0x1dfb97 as ::core::ffi::c_int as uint64_t,
    0x1c3a7 as ::core::ffi::c_int as uint64_t,
    0x5c3a7 as ::core::ffi::c_int as uint64_t,
    0x9c3a7 as ::core::ffi::c_int as uint64_t,
    0xdc3a7 as ::core::ffi::c_int as uint64_t,
    0x11c3a7 as ::core::ffi::c_int as uint64_t,
    0x15c3a7 as ::core::ffi::c_int as uint64_t,
    0x19c3a7 as ::core::ffi::c_int as uint64_t,
    0x1dc3a7 as ::core::ffi::c_int as uint64_t,
    0x1cba7 as ::core::ffi::c_int as uint64_t,
    0x5cba7 as ::core::ffi::c_int as uint64_t,
    0x9cba7 as ::core::ffi::c_int as uint64_t,
    0xdcba7 as ::core::ffi::c_int as uint64_t,
    0x11cba7 as ::core::ffi::c_int as uint64_t,
    0x15cba7 as ::core::ffi::c_int as uint64_t,
    0x19cba7 as ::core::ffi::c_int as uint64_t,
    0x1dcba7 as ::core::ffi::c_int as uint64_t,
    0x1d3a7 as ::core::ffi::c_int as uint64_t,
    0x5d3a7 as ::core::ffi::c_int as uint64_t,
    0x9d3a7 as ::core::ffi::c_int as uint64_t,
    0xdd3a7 as ::core::ffi::c_int as uint64_t,
    0x11d3a7 as ::core::ffi::c_int as uint64_t,
    0x15d3a7 as ::core::ffi::c_int as uint64_t,
    0x19d3a7 as ::core::ffi::c_int as uint64_t,
    0x1dd3a7 as ::core::ffi::c_int as uint64_t,
    0x1dba7 as ::core::ffi::c_int as uint64_t,
    0x5dba7 as ::core::ffi::c_int as uint64_t,
    0x9dba7 as ::core::ffi::c_int as uint64_t,
    0xddba7 as ::core::ffi::c_int as uint64_t,
    0x11dba7 as ::core::ffi::c_int as uint64_t,
    0x15dba7 as ::core::ffi::c_int as uint64_t,
    0x19dba7 as ::core::ffi::c_int as uint64_t,
    0x1ddba7 as ::core::ffi::c_int as uint64_t,
    0x1e3a7 as ::core::ffi::c_int as uint64_t,
    0x5e3a7 as ::core::ffi::c_int as uint64_t,
    0x9e3a7 as ::core::ffi::c_int as uint64_t,
    0xde3a7 as ::core::ffi::c_int as uint64_t,
    0x11e3a7 as ::core::ffi::c_int as uint64_t,
    0x15e3a7 as ::core::ffi::c_int as uint64_t,
    0x19e3a7 as ::core::ffi::c_int as uint64_t,
    0x1de3a7 as ::core::ffi::c_int as uint64_t,
    0x1eba7 as ::core::ffi::c_int as uint64_t,
    0x5eba7 as ::core::ffi::c_int as uint64_t,
    0x9eba7 as ::core::ffi::c_int as uint64_t,
    0xdeba7 as ::core::ffi::c_int as uint64_t,
    0x11eba7 as ::core::ffi::c_int as uint64_t,
    0x15eba7 as ::core::ffi::c_int as uint64_t,
    0x19eba7 as ::core::ffi::c_int as uint64_t,
    0x1deba7 as ::core::ffi::c_int as uint64_t,
    0x1f3a7 as ::core::ffi::c_int as uint64_t,
    0x5f3a7 as ::core::ffi::c_int as uint64_t,
    0x9f3a7 as ::core::ffi::c_int as uint64_t,
    0xdf3a7 as ::core::ffi::c_int as uint64_t,
    0x11f3a7 as ::core::ffi::c_int as uint64_t,
    0x15f3a7 as ::core::ffi::c_int as uint64_t,
    0x19f3a7 as ::core::ffi::c_int as uint64_t,
    0x1df3a7 as ::core::ffi::c_int as uint64_t,
    0x1fba7 as ::core::ffi::c_int as uint64_t,
    0x5fba7 as ::core::ffi::c_int as uint64_t,
    0x9fba7 as ::core::ffi::c_int as uint64_t,
    0xdfba7 as ::core::ffi::c_int as uint64_t,
    0x11fba7 as ::core::ffi::c_int as uint64_t,
    0x15fba7 as ::core::ffi::c_int as uint64_t,
    0x19fba7 as ::core::ffi::c_int as uint64_t,
    0x1dfba7 as ::core::ffi::c_int as uint64_t,
    0x1c3b7 as ::core::ffi::c_int as uint64_t,
    0x5c3b7 as ::core::ffi::c_int as uint64_t,
    0x9c3b7 as ::core::ffi::c_int as uint64_t,
    0xdc3b7 as ::core::ffi::c_int as uint64_t,
    0x11c3b7 as ::core::ffi::c_int as uint64_t,
    0x15c3b7 as ::core::ffi::c_int as uint64_t,
    0x19c3b7 as ::core::ffi::c_int as uint64_t,
    0x1dc3b7 as ::core::ffi::c_int as uint64_t,
    0x1cbb7 as ::core::ffi::c_int as uint64_t,
    0x5cbb7 as ::core::ffi::c_int as uint64_t,
    0x9cbb7 as ::core::ffi::c_int as uint64_t,
    0xdcbb7 as ::core::ffi::c_int as uint64_t,
    0x11cbb7 as ::core::ffi::c_int as uint64_t,
    0x15cbb7 as ::core::ffi::c_int as uint64_t,
    0x19cbb7 as ::core::ffi::c_int as uint64_t,
    0x1dcbb7 as ::core::ffi::c_int as uint64_t,
    0x1d3b7 as ::core::ffi::c_int as uint64_t,
    0x5d3b7 as ::core::ffi::c_int as uint64_t,
    0x9d3b7 as ::core::ffi::c_int as uint64_t,
    0xdd3b7 as ::core::ffi::c_int as uint64_t,
    0x11d3b7 as ::core::ffi::c_int as uint64_t,
    0x15d3b7 as ::core::ffi::c_int as uint64_t,
    0x19d3b7 as ::core::ffi::c_int as uint64_t,
    0x1dd3b7 as ::core::ffi::c_int as uint64_t,
    0x1dbb7 as ::core::ffi::c_int as uint64_t,
    0x5dbb7 as ::core::ffi::c_int as uint64_t,
    0x9dbb7 as ::core::ffi::c_int as uint64_t,
    0xddbb7 as ::core::ffi::c_int as uint64_t,
    0x11dbb7 as ::core::ffi::c_int as uint64_t,
    0x15dbb7 as ::core::ffi::c_int as uint64_t,
    0x19dbb7 as ::core::ffi::c_int as uint64_t,
    0x1ddbb7 as ::core::ffi::c_int as uint64_t,
    0x1e3b7 as ::core::ffi::c_int as uint64_t,
    0x5e3b7 as ::core::ffi::c_int as uint64_t,
    0x9e3b7 as ::core::ffi::c_int as uint64_t,
    0xde3b7 as ::core::ffi::c_int as uint64_t,
    0x11e3b7 as ::core::ffi::c_int as uint64_t,
    0x15e3b7 as ::core::ffi::c_int as uint64_t,
    0x19e3b7 as ::core::ffi::c_int as uint64_t,
    0x1de3b7 as ::core::ffi::c_int as uint64_t,
    0x1ebb7 as ::core::ffi::c_int as uint64_t,
    0x5ebb7 as ::core::ffi::c_int as uint64_t,
    0x9ebb7 as ::core::ffi::c_int as uint64_t,
    0xdebb7 as ::core::ffi::c_int as uint64_t,
    0x11ebb7 as ::core::ffi::c_int as uint64_t,
    0x15ebb7 as ::core::ffi::c_int as uint64_t,
    0x19ebb7 as ::core::ffi::c_int as uint64_t,
    0x1debb7 as ::core::ffi::c_int as uint64_t,
    0x1f3b7 as ::core::ffi::c_int as uint64_t,
    0x5f3b7 as ::core::ffi::c_int as uint64_t,
    0x9f3b7 as ::core::ffi::c_int as uint64_t,
    0xdf3b7 as ::core::ffi::c_int as uint64_t,
    0x11f3b7 as ::core::ffi::c_int as uint64_t,
    0x15f3b7 as ::core::ffi::c_int as uint64_t,
    0x19f3b7 as ::core::ffi::c_int as uint64_t,
    0x1df3b7 as ::core::ffi::c_int as uint64_t,
    0x1fbb7 as ::core::ffi::c_int as uint64_t,
    0x5fbb7 as ::core::ffi::c_int as uint64_t,
    0x9fbb7 as ::core::ffi::c_int as uint64_t,
    0xdfbb7 as ::core::ffi::c_int as uint64_t,
    0x11fbb7 as ::core::ffi::c_int as uint64_t,
    0x15fbb7 as ::core::ffi::c_int as uint64_t,
    0x19fbb7 as ::core::ffi::c_int as uint64_t,
    0x1dfbb7 as ::core::ffi::c_int as uint64_t,
    0x1c3c7 as ::core::ffi::c_int as uint64_t,
    0x5c3c7 as ::core::ffi::c_int as uint64_t,
    0x9c3c7 as ::core::ffi::c_int as uint64_t,
    0xdc3c7 as ::core::ffi::c_int as uint64_t,
    0x11c3c7 as ::core::ffi::c_int as uint64_t,
    0x15c3c7 as ::core::ffi::c_int as uint64_t,
    0x19c3c7 as ::core::ffi::c_int as uint64_t,
    0x1dc3c7 as ::core::ffi::c_int as uint64_t,
    0x1cbc7 as ::core::ffi::c_int as uint64_t,
    0x5cbc7 as ::core::ffi::c_int as uint64_t,
    0x9cbc7 as ::core::ffi::c_int as uint64_t,
    0xdcbc7 as ::core::ffi::c_int as uint64_t,
    0x11cbc7 as ::core::ffi::c_int as uint64_t,
    0x15cbc7 as ::core::ffi::c_int as uint64_t,
    0x19cbc7 as ::core::ffi::c_int as uint64_t,
    0x1dcbc7 as ::core::ffi::c_int as uint64_t,
    0x1d3c7 as ::core::ffi::c_int as uint64_t,
    0x5d3c7 as ::core::ffi::c_int as uint64_t,
    0x9d3c7 as ::core::ffi::c_int as uint64_t,
    0xdd3c7 as ::core::ffi::c_int as uint64_t,
    0x11d3c7 as ::core::ffi::c_int as uint64_t,
    0x15d3c7 as ::core::ffi::c_int as uint64_t,
    0x19d3c7 as ::core::ffi::c_int as uint64_t,
    0x1dd3c7 as ::core::ffi::c_int as uint64_t,
    0x1dbc7 as ::core::ffi::c_int as uint64_t,
    0x5dbc7 as ::core::ffi::c_int as uint64_t,
    0x9dbc7 as ::core::ffi::c_int as uint64_t,
    0xddbc7 as ::core::ffi::c_int as uint64_t,
    0x11dbc7 as ::core::ffi::c_int as uint64_t,
    0x15dbc7 as ::core::ffi::c_int as uint64_t,
    0x19dbc7 as ::core::ffi::c_int as uint64_t,
    0x1ddbc7 as ::core::ffi::c_int as uint64_t,
    0x1e3c7 as ::core::ffi::c_int as uint64_t,
    0x5e3c7 as ::core::ffi::c_int as uint64_t,
    0x9e3c7 as ::core::ffi::c_int as uint64_t,
    0xde3c7 as ::core::ffi::c_int as uint64_t,
    0x11e3c7 as ::core::ffi::c_int as uint64_t,
    0x15e3c7 as ::core::ffi::c_int as uint64_t,
    0x19e3c7 as ::core::ffi::c_int as uint64_t,
    0x1de3c7 as ::core::ffi::c_int as uint64_t,
    0x1ebc7 as ::core::ffi::c_int as uint64_t,
    0x5ebc7 as ::core::ffi::c_int as uint64_t,
    0x9ebc7 as ::core::ffi::c_int as uint64_t,
    0xdebc7 as ::core::ffi::c_int as uint64_t,
    0x11ebc7 as ::core::ffi::c_int as uint64_t,
    0x15ebc7 as ::core::ffi::c_int as uint64_t,
    0x19ebc7 as ::core::ffi::c_int as uint64_t,
    0x1debc7 as ::core::ffi::c_int as uint64_t,
    0x1f3c7 as ::core::ffi::c_int as uint64_t,
    0x5f3c7 as ::core::ffi::c_int as uint64_t,
    0x9f3c7 as ::core::ffi::c_int as uint64_t,
    0xdf3c7 as ::core::ffi::c_int as uint64_t,
    0x11f3c7 as ::core::ffi::c_int as uint64_t,
    0x15f3c7 as ::core::ffi::c_int as uint64_t,
    0x19f3c7 as ::core::ffi::c_int as uint64_t,
    0x1df3c7 as ::core::ffi::c_int as uint64_t,
    0x1fbc7 as ::core::ffi::c_int as uint64_t,
    0x5fbc7 as ::core::ffi::c_int as uint64_t,
    0x9fbc7 as ::core::ffi::c_int as uint64_t,
    0xdfbc7 as ::core::ffi::c_int as uint64_t,
    0x11fbc7 as ::core::ffi::c_int as uint64_t,
    0x15fbc7 as ::core::ffi::c_int as uint64_t,
    0x19fbc7 as ::core::ffi::c_int as uint64_t,
    0x1dfbc7 as ::core::ffi::c_int as uint64_t,
    0x1c3d7 as ::core::ffi::c_int as uint64_t,
    0x5c3d7 as ::core::ffi::c_int as uint64_t,
    0x9c3d7 as ::core::ffi::c_int as uint64_t,
    0xdc3d7 as ::core::ffi::c_int as uint64_t,
    0x11c3d7 as ::core::ffi::c_int as uint64_t,
    0x15c3d7 as ::core::ffi::c_int as uint64_t,
    0x19c3d7 as ::core::ffi::c_int as uint64_t,
    0x1dc3d7 as ::core::ffi::c_int as uint64_t,
    0x1cbd7 as ::core::ffi::c_int as uint64_t,
    0x5cbd7 as ::core::ffi::c_int as uint64_t,
    0x9cbd7 as ::core::ffi::c_int as uint64_t,
    0xdcbd7 as ::core::ffi::c_int as uint64_t,
    0x11cbd7 as ::core::ffi::c_int as uint64_t,
    0x15cbd7 as ::core::ffi::c_int as uint64_t,
    0x19cbd7 as ::core::ffi::c_int as uint64_t,
    0x1dcbd7 as ::core::ffi::c_int as uint64_t,
    0x1d3d7 as ::core::ffi::c_int as uint64_t,
    0x5d3d7 as ::core::ffi::c_int as uint64_t,
    0x9d3d7 as ::core::ffi::c_int as uint64_t,
    0xdd3d7 as ::core::ffi::c_int as uint64_t,
    0x11d3d7 as ::core::ffi::c_int as uint64_t,
    0x15d3d7 as ::core::ffi::c_int as uint64_t,
    0x19d3d7 as ::core::ffi::c_int as uint64_t,
    0x1dd3d7 as ::core::ffi::c_int as uint64_t,
    0x1dbd7 as ::core::ffi::c_int as uint64_t,
    0x5dbd7 as ::core::ffi::c_int as uint64_t,
    0x9dbd7 as ::core::ffi::c_int as uint64_t,
    0xddbd7 as ::core::ffi::c_int as uint64_t,
    0x11dbd7 as ::core::ffi::c_int as uint64_t,
    0x15dbd7 as ::core::ffi::c_int as uint64_t,
    0x19dbd7 as ::core::ffi::c_int as uint64_t,
    0x1ddbd7 as ::core::ffi::c_int as uint64_t,
    0x1e3d7 as ::core::ffi::c_int as uint64_t,
    0x5e3d7 as ::core::ffi::c_int as uint64_t,
    0x9e3d7 as ::core::ffi::c_int as uint64_t,
    0xde3d7 as ::core::ffi::c_int as uint64_t,
    0x11e3d7 as ::core::ffi::c_int as uint64_t,
    0x15e3d7 as ::core::ffi::c_int as uint64_t,
    0x19e3d7 as ::core::ffi::c_int as uint64_t,
    0x1de3d7 as ::core::ffi::c_int as uint64_t,
    0x1ebd7 as ::core::ffi::c_int as uint64_t,
    0x5ebd7 as ::core::ffi::c_int as uint64_t,
    0x9ebd7 as ::core::ffi::c_int as uint64_t,
    0xdebd7 as ::core::ffi::c_int as uint64_t,
    0x11ebd7 as ::core::ffi::c_int as uint64_t,
    0x15ebd7 as ::core::ffi::c_int as uint64_t,
    0x19ebd7 as ::core::ffi::c_int as uint64_t,
    0x1debd7 as ::core::ffi::c_int as uint64_t,
    0x1f3d7 as ::core::ffi::c_int as uint64_t,
    0x5f3d7 as ::core::ffi::c_int as uint64_t,
    0x9f3d7 as ::core::ffi::c_int as uint64_t,
    0xdf3d7 as ::core::ffi::c_int as uint64_t,
    0x11f3d7 as ::core::ffi::c_int as uint64_t,
    0x15f3d7 as ::core::ffi::c_int as uint64_t,
    0x19f3d7 as ::core::ffi::c_int as uint64_t,
    0x1df3d7 as ::core::ffi::c_int as uint64_t,
    0x1fbd7 as ::core::ffi::c_int as uint64_t,
    0x5fbd7 as ::core::ffi::c_int as uint64_t,
    0x9fbd7 as ::core::ffi::c_int as uint64_t,
    0xdfbd7 as ::core::ffi::c_int as uint64_t,
    0x11fbd7 as ::core::ffi::c_int as uint64_t,
    0x15fbd7 as ::core::ffi::c_int as uint64_t,
    0x19fbd7 as ::core::ffi::c_int as uint64_t,
    0x1dfbd7 as ::core::ffi::c_int as uint64_t,
    0x1c3e7 as ::core::ffi::c_int as uint64_t,
    0x5c3e7 as ::core::ffi::c_int as uint64_t,
    0x9c3e7 as ::core::ffi::c_int as uint64_t,
    0xdc3e7 as ::core::ffi::c_int as uint64_t,
    0x11c3e7 as ::core::ffi::c_int as uint64_t,
    0x15c3e7 as ::core::ffi::c_int as uint64_t,
    0x19c3e7 as ::core::ffi::c_int as uint64_t,
    0x1dc3e7 as ::core::ffi::c_int as uint64_t,
    0x1cbe7 as ::core::ffi::c_int as uint64_t,
    0x5cbe7 as ::core::ffi::c_int as uint64_t,
    0x9cbe7 as ::core::ffi::c_int as uint64_t,
    0xdcbe7 as ::core::ffi::c_int as uint64_t,
    0x11cbe7 as ::core::ffi::c_int as uint64_t,
    0x15cbe7 as ::core::ffi::c_int as uint64_t,
    0x19cbe7 as ::core::ffi::c_int as uint64_t,
    0x1dcbe7 as ::core::ffi::c_int as uint64_t,
    0x1d3e7 as ::core::ffi::c_int as uint64_t,
    0x5d3e7 as ::core::ffi::c_int as uint64_t,
    0x9d3e7 as ::core::ffi::c_int as uint64_t,
    0xdd3e7 as ::core::ffi::c_int as uint64_t,
    0x11d3e7 as ::core::ffi::c_int as uint64_t,
    0x15d3e7 as ::core::ffi::c_int as uint64_t,
    0x19d3e7 as ::core::ffi::c_int as uint64_t,
    0x1dd3e7 as ::core::ffi::c_int as uint64_t,
    0x1dbe7 as ::core::ffi::c_int as uint64_t,
    0x5dbe7 as ::core::ffi::c_int as uint64_t,
    0x9dbe7 as ::core::ffi::c_int as uint64_t,
    0xddbe7 as ::core::ffi::c_int as uint64_t,
    0x11dbe7 as ::core::ffi::c_int as uint64_t,
    0x15dbe7 as ::core::ffi::c_int as uint64_t,
    0x19dbe7 as ::core::ffi::c_int as uint64_t,
    0x1ddbe7 as ::core::ffi::c_int as uint64_t,
    0x1e3e7 as ::core::ffi::c_int as uint64_t,
    0x5e3e7 as ::core::ffi::c_int as uint64_t,
    0x9e3e7 as ::core::ffi::c_int as uint64_t,
    0xde3e7 as ::core::ffi::c_int as uint64_t,
    0x11e3e7 as ::core::ffi::c_int as uint64_t,
    0x15e3e7 as ::core::ffi::c_int as uint64_t,
    0x19e3e7 as ::core::ffi::c_int as uint64_t,
    0x1de3e7 as ::core::ffi::c_int as uint64_t,
    0x1ebe7 as ::core::ffi::c_int as uint64_t,
    0x5ebe7 as ::core::ffi::c_int as uint64_t,
    0x9ebe7 as ::core::ffi::c_int as uint64_t,
    0xdebe7 as ::core::ffi::c_int as uint64_t,
    0x11ebe7 as ::core::ffi::c_int as uint64_t,
    0x15ebe7 as ::core::ffi::c_int as uint64_t,
    0x19ebe7 as ::core::ffi::c_int as uint64_t,
    0x1debe7 as ::core::ffi::c_int as uint64_t,
    0x1f3e7 as ::core::ffi::c_int as uint64_t,
    0x5f3e7 as ::core::ffi::c_int as uint64_t,
    0x9f3e7 as ::core::ffi::c_int as uint64_t,
    0xdf3e7 as ::core::ffi::c_int as uint64_t,
    0x11f3e7 as ::core::ffi::c_int as uint64_t,
    0x15f3e7 as ::core::ffi::c_int as uint64_t,
    0x19f3e7 as ::core::ffi::c_int as uint64_t,
    0x1df3e7 as ::core::ffi::c_int as uint64_t,
    0x1fbe7 as ::core::ffi::c_int as uint64_t,
    0x5fbe7 as ::core::ffi::c_int as uint64_t,
    0x9fbe7 as ::core::ffi::c_int as uint64_t,
    0xdfbe7 as ::core::ffi::c_int as uint64_t,
    0x11fbe7 as ::core::ffi::c_int as uint64_t,
    0x15fbe7 as ::core::ffi::c_int as uint64_t,
    0x19fbe7 as ::core::ffi::c_int as uint64_t,
    0x1dfbe7 as ::core::ffi::c_int as uint64_t,
    0x1c3f7 as ::core::ffi::c_int as uint64_t,
    0x5c3f7 as ::core::ffi::c_int as uint64_t,
    0x9c3f7 as ::core::ffi::c_int as uint64_t,
    0xdc3f7 as ::core::ffi::c_int as uint64_t,
    0x11c3f7 as ::core::ffi::c_int as uint64_t,
    0x15c3f7 as ::core::ffi::c_int as uint64_t,
    0x19c3f7 as ::core::ffi::c_int as uint64_t,
    0x1dc3f7 as ::core::ffi::c_int as uint64_t,
    0x1cbf7 as ::core::ffi::c_int as uint64_t,
    0x5cbf7 as ::core::ffi::c_int as uint64_t,
    0x9cbf7 as ::core::ffi::c_int as uint64_t,
    0xdcbf7 as ::core::ffi::c_int as uint64_t,
    0x11cbf7 as ::core::ffi::c_int as uint64_t,
    0x15cbf7 as ::core::ffi::c_int as uint64_t,
    0x19cbf7 as ::core::ffi::c_int as uint64_t,
    0x1dcbf7 as ::core::ffi::c_int as uint64_t,
    0x1d3f7 as ::core::ffi::c_int as uint64_t,
    0x5d3f7 as ::core::ffi::c_int as uint64_t,
    0x9d3f7 as ::core::ffi::c_int as uint64_t,
    0xdd3f7 as ::core::ffi::c_int as uint64_t,
    0x11d3f7 as ::core::ffi::c_int as uint64_t,
    0x15d3f7 as ::core::ffi::c_int as uint64_t,
    0x19d3f7 as ::core::ffi::c_int as uint64_t,
    0x1dd3f7 as ::core::ffi::c_int as uint64_t,
    0x1dbf7 as ::core::ffi::c_int as uint64_t,
    0x5dbf7 as ::core::ffi::c_int as uint64_t,
    0x9dbf7 as ::core::ffi::c_int as uint64_t,
    0xddbf7 as ::core::ffi::c_int as uint64_t,
    0x11dbf7 as ::core::ffi::c_int as uint64_t,
    0x15dbf7 as ::core::ffi::c_int as uint64_t,
    0x19dbf7 as ::core::ffi::c_int as uint64_t,
    0x1ddbf7 as ::core::ffi::c_int as uint64_t,
    0x1e3f7 as ::core::ffi::c_int as uint64_t,
    0x5e3f7 as ::core::ffi::c_int as uint64_t,
    0x9e3f7 as ::core::ffi::c_int as uint64_t,
    0xde3f7 as ::core::ffi::c_int as uint64_t,
    0x11e3f7 as ::core::ffi::c_int as uint64_t,
    0x15e3f7 as ::core::ffi::c_int as uint64_t,
    0x19e3f7 as ::core::ffi::c_int as uint64_t,
    0x1de3f7 as ::core::ffi::c_int as uint64_t,
    0x1ebf7 as ::core::ffi::c_int as uint64_t,
    0x5ebf7 as ::core::ffi::c_int as uint64_t,
    0x9ebf7 as ::core::ffi::c_int as uint64_t,
    0xdebf7 as ::core::ffi::c_int as uint64_t,
    0x11ebf7 as ::core::ffi::c_int as uint64_t,
    0x15ebf7 as ::core::ffi::c_int as uint64_t,
    0x19ebf7 as ::core::ffi::c_int as uint64_t,
    0x1debf7 as ::core::ffi::c_int as uint64_t,
    0x1f3f7 as ::core::ffi::c_int as uint64_t,
    0x5f3f7 as ::core::ffi::c_int as uint64_t,
    0x9f3f7 as ::core::ffi::c_int as uint64_t,
    0xdf3f7 as ::core::ffi::c_int as uint64_t,
    0x11f3f7 as ::core::ffi::c_int as uint64_t,
    0x15f3f7 as ::core::ffi::c_int as uint64_t,
    0x19f3f7 as ::core::ffi::c_int as uint64_t,
    0x1df3f7 as ::core::ffi::c_int as uint64_t,
    0x1fbf7 as ::core::ffi::c_int as uint64_t,
    0x5fbf7 as ::core::ffi::c_int as uint64_t,
    0x9fbf7 as ::core::ffi::c_int as uint64_t,
    0xdfbf7 as ::core::ffi::c_int as uint64_t,
    0x11fbf7 as ::core::ffi::c_int as uint64_t,
    0x15fbf7 as ::core::ffi::c_int as uint64_t,
    0x19fbf7 as ::core::ffi::c_int as uint64_t,
    0x1dfbf7 as ::core::ffi::c_int as uint64_t,
    0xe1c387 as ::core::ffi::c_int as uint64_t,
    0x2e1c387 as ::core::ffi::c_int as uint64_t,
    0x4e1c387 as ::core::ffi::c_int as uint64_t,
    0x6e1c387 as ::core::ffi::c_int as uint64_t,
    0x8e1c387 as ::core::ffi::c_int as uint64_t,
    0xae1c387 as ::core::ffi::c_int as uint64_t,
    0xce1c387 as ::core::ffi::c_int as uint64_t,
    0xee1c387 as ::core::ffi::c_int as uint64_t,
    0xe5c387 as ::core::ffi::c_int as uint64_t,
    0x2e5c387 as ::core::ffi::c_int as uint64_t,
    0x4e5c387 as ::core::ffi::c_int as uint64_t,
    0x6e5c387 as ::core::ffi::c_int as uint64_t,
    0x8e5c387 as ::core::ffi::c_int as uint64_t,
    0xae5c387 as ::core::ffi::c_int as uint64_t,
    0xce5c387 as ::core::ffi::c_int as uint64_t,
    0xee5c387 as ::core::ffi::c_int as uint64_t,
    0xe9c387 as ::core::ffi::c_int as uint64_t,
    0x2e9c387 as ::core::ffi::c_int as uint64_t,
    0x4e9c387 as ::core::ffi::c_int as uint64_t,
    0x6e9c387 as ::core::ffi::c_int as uint64_t,
    0x8e9c387 as ::core::ffi::c_int as uint64_t,
    0xae9c387 as ::core::ffi::c_int as uint64_t,
    0xce9c387 as ::core::ffi::c_int as uint64_t,
    0xee9c387 as ::core::ffi::c_int as uint64_t,
    0xedc387 as ::core::ffi::c_int as uint64_t,
    0x2edc387 as ::core::ffi::c_int as uint64_t,
    0x4edc387 as ::core::ffi::c_int as uint64_t,
    0x6edc387 as ::core::ffi::c_int as uint64_t,
    0x8edc387 as ::core::ffi::c_int as uint64_t,
    0xaedc387 as ::core::ffi::c_int as uint64_t,
    0xcedc387 as ::core::ffi::c_int as uint64_t,
    0xeedc387 as ::core::ffi::c_int as uint64_t,
    0xf1c387 as ::core::ffi::c_int as uint64_t,
    0x2f1c387 as ::core::ffi::c_int as uint64_t,
    0x4f1c387 as ::core::ffi::c_int as uint64_t,
    0x6f1c387 as ::core::ffi::c_int as uint64_t,
    0x8f1c387 as ::core::ffi::c_int as uint64_t,
    0xaf1c387 as ::core::ffi::c_int as uint64_t,
    0xcf1c387 as ::core::ffi::c_int as uint64_t,
    0xef1c387 as ::core::ffi::c_int as uint64_t,
    0xf5c387 as ::core::ffi::c_int as uint64_t,
    0x2f5c387 as ::core::ffi::c_int as uint64_t,
    0x4f5c387 as ::core::ffi::c_int as uint64_t,
    0x6f5c387 as ::core::ffi::c_int as uint64_t,
    0x8f5c387 as ::core::ffi::c_int as uint64_t,
    0xaf5c387 as ::core::ffi::c_int as uint64_t,
    0xcf5c387 as ::core::ffi::c_int as uint64_t,
    0xef5c387 as ::core::ffi::c_int as uint64_t,
    0xf9c387 as ::core::ffi::c_int as uint64_t,
    0x2f9c387 as ::core::ffi::c_int as uint64_t,
    0x4f9c387 as ::core::ffi::c_int as uint64_t,
    0x6f9c387 as ::core::ffi::c_int as uint64_t,
    0x8f9c387 as ::core::ffi::c_int as uint64_t,
    0xaf9c387 as ::core::ffi::c_int as uint64_t,
    0xcf9c387 as ::core::ffi::c_int as uint64_t,
    0xef9c387 as ::core::ffi::c_int as uint64_t,
    0xfdc387 as ::core::ffi::c_int as uint64_t,
    0x2fdc387 as ::core::ffi::c_int as uint64_t,
    0x4fdc387 as ::core::ffi::c_int as uint64_t,
    0x6fdc387 as ::core::ffi::c_int as uint64_t,
    0x8fdc387 as ::core::ffi::c_int as uint64_t,
    0xafdc387 as ::core::ffi::c_int as uint64_t,
    0xcfdc387 as ::core::ffi::c_int as uint64_t,
    0xefdc387 as ::core::ffi::c_int as uint64_t,
    0xe1cb87 as ::core::ffi::c_int as uint64_t,
    0x2e1cb87 as ::core::ffi::c_int as uint64_t,
    0x4e1cb87 as ::core::ffi::c_int as uint64_t,
    0x6e1cb87 as ::core::ffi::c_int as uint64_t,
    0x8e1cb87 as ::core::ffi::c_int as uint64_t,
    0xae1cb87 as ::core::ffi::c_int as uint64_t,
    0xce1cb87 as ::core::ffi::c_int as uint64_t,
    0xee1cb87 as ::core::ffi::c_int as uint64_t,
    0xe5cb87 as ::core::ffi::c_int as uint64_t,
    0x2e5cb87 as ::core::ffi::c_int as uint64_t,
    0x4e5cb87 as ::core::ffi::c_int as uint64_t,
    0x6e5cb87 as ::core::ffi::c_int as uint64_t,
    0x8e5cb87 as ::core::ffi::c_int as uint64_t,
    0xae5cb87 as ::core::ffi::c_int as uint64_t,
    0xce5cb87 as ::core::ffi::c_int as uint64_t,
    0xee5cb87 as ::core::ffi::c_int as uint64_t,
    0xe9cb87 as ::core::ffi::c_int as uint64_t,
    0x2e9cb87 as ::core::ffi::c_int as uint64_t,
    0x4e9cb87 as ::core::ffi::c_int as uint64_t,
    0x6e9cb87 as ::core::ffi::c_int as uint64_t,
    0x8e9cb87 as ::core::ffi::c_int as uint64_t,
    0xae9cb87 as ::core::ffi::c_int as uint64_t,
    0xce9cb87 as ::core::ffi::c_int as uint64_t,
    0xee9cb87 as ::core::ffi::c_int as uint64_t,
    0xedcb87 as ::core::ffi::c_int as uint64_t,
    0x2edcb87 as ::core::ffi::c_int as uint64_t,
    0x4edcb87 as ::core::ffi::c_int as uint64_t,
    0x6edcb87 as ::core::ffi::c_int as uint64_t,
    0x8edcb87 as ::core::ffi::c_int as uint64_t,
    0xaedcb87 as ::core::ffi::c_int as uint64_t,
    0xcedcb87 as ::core::ffi::c_int as uint64_t,
    0xeedcb87 as ::core::ffi::c_int as uint64_t,
    0xf1cb87 as ::core::ffi::c_int as uint64_t,
    0x2f1cb87 as ::core::ffi::c_int as uint64_t,
    0x4f1cb87 as ::core::ffi::c_int as uint64_t,
    0x6f1cb87 as ::core::ffi::c_int as uint64_t,
    0x8f1cb87 as ::core::ffi::c_int as uint64_t,
    0xaf1cb87 as ::core::ffi::c_int as uint64_t,
    0xcf1cb87 as ::core::ffi::c_int as uint64_t,
    0xef1cb87 as ::core::ffi::c_int as uint64_t,
    0xf5cb87 as ::core::ffi::c_int as uint64_t,
    0x2f5cb87 as ::core::ffi::c_int as uint64_t,
    0x4f5cb87 as ::core::ffi::c_int as uint64_t,
    0x6f5cb87 as ::core::ffi::c_int as uint64_t,
    0x8f5cb87 as ::core::ffi::c_int as uint64_t,
    0xaf5cb87 as ::core::ffi::c_int as uint64_t,
    0xcf5cb87 as ::core::ffi::c_int as uint64_t,
    0xef5cb87 as ::core::ffi::c_int as uint64_t,
    0xf9cb87 as ::core::ffi::c_int as uint64_t,
    0x2f9cb87 as ::core::ffi::c_int as uint64_t,
    0x4f9cb87 as ::core::ffi::c_int as uint64_t,
    0x6f9cb87 as ::core::ffi::c_int as uint64_t,
    0x8f9cb87 as ::core::ffi::c_int as uint64_t,
];
static mut kZeroRepsDepth: [uint32_t; 704] = [
    0 as ::core::ffi::c_int as uint32_t,
    4 as ::core::ffi::c_int as uint32_t,
    8 as ::core::ffi::c_int as uint32_t,
    7 as ::core::ffi::c_int as uint32_t,
    7 as ::core::ffi::c_int as uint32_t,
    7 as ::core::ffi::c_int as uint32_t,
    7 as ::core::ffi::c_int as uint32_t,
    7 as ::core::ffi::c_int as uint32_t,
    7 as ::core::ffi::c_int as uint32_t,
    7 as ::core::ffi::c_int as uint32_t,
    7 as ::core::ffi::c_int as uint32_t,
    11 as ::core::ffi::c_int as uint32_t,
    14 as ::core::ffi::c_int as uint32_t,
    14 as ::core::ffi::c_int as uint32_t,
    14 as ::core::ffi::c_int as uint32_t,
    14 as ::core::ffi::c_int as uint32_t,
    14 as ::core::ffi::c_int as uint32_t,
    14 as ::core::ffi::c_int as uint32_t,
    14 as ::core::ffi::c_int as uint32_t,
    14 as ::core::ffi::c_int as uint32_t,
    14 as ::core::ffi::c_int as uint32_t,
    14 as ::core::ffi::c_int as uint32_t,
    14 as ::core::ffi::c_int as uint32_t,
    14 as ::core::ffi::c_int as uint32_t,
    14 as ::core::ffi::c_int as uint32_t,
    14 as ::core::ffi::c_int as uint32_t,
    14 as ::core::ffi::c_int as uint32_t,
    14 as ::core::ffi::c_int as uint32_t,
    14 as ::core::ffi::c_int as uint32_t,
    14 as ::core::ffi::c_int as uint32_t,
    14 as ::core::ffi::c_int as uint32_t,
    14 as ::core::ffi::c_int as uint32_t,
    14 as ::core::ffi::c_int as uint32_t,
    14 as ::core::ffi::c_int as uint32_t,
    14 as ::core::ffi::c_int as uint32_t,
    14 as ::core::ffi::c_int as uint32_t,
    14 as ::core::ffi::c_int as uint32_t,
    14 as ::core::ffi::c_int as uint32_t,
    14 as ::core::ffi::c_int as uint32_t,
    14 as ::core::ffi::c_int as uint32_t,
    14 as ::core::ffi::c_int as uint32_t,
    14 as ::core::ffi::c_int as uint32_t,
    14 as ::core::ffi::c_int as uint32_t,
    14 as ::core::ffi::c_int as uint32_t,
    14 as ::core::ffi::c_int as uint32_t,
    14 as ::core::ffi::c_int as uint32_t,
    14 as ::core::ffi::c_int as uint32_t,
    14 as ::core::ffi::c_int as uint32_t,
    14 as ::core::ffi::c_int as uint32_t,
    14 as ::core::ffi::c_int as uint32_t,
    14 as ::core::ffi::c_int as uint32_t,
    14 as ::core::ffi::c_int as uint32_t,
    14 as ::core::ffi::c_int as uint32_t,
    14 as ::core::ffi::c_int as uint32_t,
    14 as ::core::ffi::c_int as uint32_t,
    14 as ::core::ffi::c_int as uint32_t,
    14 as ::core::ffi::c_int as uint32_t,
    14 as ::core::ffi::c_int as uint32_t,
    14 as ::core::ffi::c_int as uint32_t,
    14 as ::core::ffi::c_int as uint32_t,
    14 as ::core::ffi::c_int as uint32_t,
    14 as ::core::ffi::c_int as uint32_t,
    14 as ::core::ffi::c_int as uint32_t,
    14 as ::core::ffi::c_int as uint32_t,
    14 as ::core::ffi::c_int as uint32_t,
    14 as ::core::ffi::c_int as uint32_t,
    14 as ::core::ffi::c_int as uint32_t,
    14 as ::core::ffi::c_int as uint32_t,
    14 as ::core::ffi::c_int as uint32_t,
    14 as ::core::ffi::c_int as uint32_t,
    14 as ::core::ffi::c_int as uint32_t,
    14 as ::core::ffi::c_int as uint32_t,
    14 as ::core::ffi::c_int as uint32_t,
    14 as ::core::ffi::c_int as uint32_t,
    14 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    21 as ::core::ffi::c_int as uint32_t,
    28 as ::core::ffi::c_int as uint32_t,
    28 as ::core::ffi::c_int as uint32_t,
    28 as ::core::ffi::c_int as uint32_t,
    28 as ::core::ffi::c_int as uint32_t,
    28 as ::core::ffi::c_int as uint32_t,
    28 as ::core::ffi::c_int as uint32_t,
    28 as ::core::ffi::c_int as uint32_t,
    28 as ::core::ffi::c_int as uint32_t,
    28 as ::core::ffi::c_int as uint32_t,
    28 as ::core::ffi::c_int as uint32_t,
    28 as ::core::ffi::c_int as uint32_t,
    28 as ::core::ffi::c_int as uint32_t,
    28 as ::core::ffi::c_int as uint32_t,
    28 as ::core::ffi::c_int as uint32_t,
    28 as ::core::ffi::c_int as uint32_t,
    28 as ::core::ffi::c_int as uint32_t,
    28 as ::core::ffi::c_int as uint32_t,
    28 as ::core::ffi::c_int as uint32_t,
    28 as ::core::ffi::c_int as uint32_t,
    28 as ::core::ffi::c_int as uint32_t,
    28 as ::core::ffi::c_int as uint32_t,
    28 as ::core::ffi::c_int as uint32_t,
    28 as ::core::ffi::c_int as uint32_t,
    28 as ::core::ffi::c_int as uint32_t,
    28 as ::core::ffi::c_int as uint32_t,
    28 as ::core::ffi::c_int as uint32_t,
    28 as ::core::ffi::c_int as uint32_t,
    28 as ::core::ffi::c_int as uint32_t,
    28 as ::core::ffi::c_int as uint32_t,
    28 as ::core::ffi::c_int as uint32_t,
    28 as ::core::ffi::c_int as uint32_t,
    28 as ::core::ffi::c_int as uint32_t,
    28 as ::core::ffi::c_int as uint32_t,
    28 as ::core::ffi::c_int as uint32_t,
    28 as ::core::ffi::c_int as uint32_t,
    28 as ::core::ffi::c_int as uint32_t,
    28 as ::core::ffi::c_int as uint32_t,
    28 as ::core::ffi::c_int as uint32_t,
    28 as ::core::ffi::c_int as uint32_t,
    28 as ::core::ffi::c_int as uint32_t,
    28 as ::core::ffi::c_int as uint32_t,
    28 as ::core::ffi::c_int as uint32_t,
    28 as ::core::ffi::c_int as uint32_t,
    28 as ::core::ffi::c_int as uint32_t,
    28 as ::core::ffi::c_int as uint32_t,
    28 as ::core::ffi::c_int as uint32_t,
    28 as ::core::ffi::c_int as uint32_t,
    28 as ::core::ffi::c_int as uint32_t,
    28 as ::core::ffi::c_int as uint32_t,
    28 as ::core::ffi::c_int as uint32_t,
    28 as ::core::ffi::c_int as uint32_t,
    28 as ::core::ffi::c_int as uint32_t,
    28 as ::core::ffi::c_int as uint32_t,
    28 as ::core::ffi::c_int as uint32_t,
    28 as ::core::ffi::c_int as uint32_t,
    28 as ::core::ffi::c_int as uint32_t,
    28 as ::core::ffi::c_int as uint32_t,
    28 as ::core::ffi::c_int as uint32_t,
    28 as ::core::ffi::c_int as uint32_t,
    28 as ::core::ffi::c_int as uint32_t,
    28 as ::core::ffi::c_int as uint32_t,
    28 as ::core::ffi::c_int as uint32_t,
    28 as ::core::ffi::c_int as uint32_t,
    28 as ::core::ffi::c_int as uint32_t,
    28 as ::core::ffi::c_int as uint32_t,
    28 as ::core::ffi::c_int as uint32_t,
    28 as ::core::ffi::c_int as uint32_t,
    28 as ::core::ffi::c_int as uint32_t,
    28 as ::core::ffi::c_int as uint32_t,
    28 as ::core::ffi::c_int as uint32_t,
    28 as ::core::ffi::c_int as uint32_t,
    28 as ::core::ffi::c_int as uint32_t,
    28 as ::core::ffi::c_int as uint32_t,
    28 as ::core::ffi::c_int as uint32_t,
    28 as ::core::ffi::c_int as uint32_t,
    28 as ::core::ffi::c_int as uint32_t,
    28 as ::core::ffi::c_int as uint32_t,
    28 as ::core::ffi::c_int as uint32_t,
    28 as ::core::ffi::c_int as uint32_t,
    28 as ::core::ffi::c_int as uint32_t,
    28 as ::core::ffi::c_int as uint32_t,
    28 as ::core::ffi::c_int as uint32_t,
    28 as ::core::ffi::c_int as uint32_t,
    28 as ::core::ffi::c_int as uint32_t,
    28 as ::core::ffi::c_int as uint32_t,
    28 as ::core::ffi::c_int as uint32_t,
    28 as ::core::ffi::c_int as uint32_t,
    28 as ::core::ffi::c_int as uint32_t,
    28 as ::core::ffi::c_int as uint32_t,
    28 as ::core::ffi::c_int as uint32_t,
    28 as ::core::ffi::c_int as uint32_t,
    28 as ::core::ffi::c_int as uint32_t,
    28 as ::core::ffi::c_int as uint32_t,
    28 as ::core::ffi::c_int as uint32_t,
    28 as ::core::ffi::c_int as uint32_t,
    28 as ::core::ffi::c_int as uint32_t,
    28 as ::core::ffi::c_int as uint32_t,
    28 as ::core::ffi::c_int as uint32_t,
    28 as ::core::ffi::c_int as uint32_t,
    28 as ::core::ffi::c_int as uint32_t,
    28 as ::core::ffi::c_int as uint32_t,
    28 as ::core::ffi::c_int as uint32_t,
    28 as ::core::ffi::c_int as uint32_t,
    28 as ::core::ffi::c_int as uint32_t,
    28 as ::core::ffi::c_int as uint32_t,
    28 as ::core::ffi::c_int as uint32_t,
    28 as ::core::ffi::c_int as uint32_t,
    28 as ::core::ffi::c_int as uint32_t,
    28 as ::core::ffi::c_int as uint32_t,
    28 as ::core::ffi::c_int as uint32_t,
    28 as ::core::ffi::c_int as uint32_t,
    28 as ::core::ffi::c_int as uint32_t,
    28 as ::core::ffi::c_int as uint32_t,
    28 as ::core::ffi::c_int as uint32_t,
    28 as ::core::ffi::c_int as uint32_t,
    28 as ::core::ffi::c_int as uint32_t,
    28 as ::core::ffi::c_int as uint32_t,
];
static mut kNonZeroRepsBits: [uint64_t; 704] = [
    0xb as ::core::ffi::c_int as uint64_t,
    0x1b as ::core::ffi::c_int as uint64_t,
    0x2b as ::core::ffi::c_int as uint64_t,
    0x3b as ::core::ffi::c_int as uint64_t,
    0x2cb as ::core::ffi::c_int as uint64_t,
    0x6cb as ::core::ffi::c_int as uint64_t,
    0xacb as ::core::ffi::c_int as uint64_t,
    0xecb as ::core::ffi::c_int as uint64_t,
    0x2db as ::core::ffi::c_int as uint64_t,
    0x6db as ::core::ffi::c_int as uint64_t,
    0xadb as ::core::ffi::c_int as uint64_t,
    0xedb as ::core::ffi::c_int as uint64_t,
    0x2eb as ::core::ffi::c_int as uint64_t,
    0x6eb as ::core::ffi::c_int as uint64_t,
    0xaeb as ::core::ffi::c_int as uint64_t,
    0xeeb as ::core::ffi::c_int as uint64_t,
    0x2fb as ::core::ffi::c_int as uint64_t,
    0x6fb as ::core::ffi::c_int as uint64_t,
    0xafb as ::core::ffi::c_int as uint64_t,
    0xefb as ::core::ffi::c_int as uint64_t,
    0xb2cb as ::core::ffi::c_int as uint64_t,
    0x1b2cb as ::core::ffi::c_int as uint64_t,
    0x2b2cb as ::core::ffi::c_int as uint64_t,
    0x3b2cb as ::core::ffi::c_int as uint64_t,
    0xb6cb as ::core::ffi::c_int as uint64_t,
    0x1b6cb as ::core::ffi::c_int as uint64_t,
    0x2b6cb as ::core::ffi::c_int as uint64_t,
    0x3b6cb as ::core::ffi::c_int as uint64_t,
    0xbacb as ::core::ffi::c_int as uint64_t,
    0x1bacb as ::core::ffi::c_int as uint64_t,
    0x2bacb as ::core::ffi::c_int as uint64_t,
    0x3bacb as ::core::ffi::c_int as uint64_t,
    0xbecb as ::core::ffi::c_int as uint64_t,
    0x1becb as ::core::ffi::c_int as uint64_t,
    0x2becb as ::core::ffi::c_int as uint64_t,
    0x3becb as ::core::ffi::c_int as uint64_t,
    0xb2db as ::core::ffi::c_int as uint64_t,
    0x1b2db as ::core::ffi::c_int as uint64_t,
    0x2b2db as ::core::ffi::c_int as uint64_t,
    0x3b2db as ::core::ffi::c_int as uint64_t,
    0xb6db as ::core::ffi::c_int as uint64_t,
    0x1b6db as ::core::ffi::c_int as uint64_t,
    0x2b6db as ::core::ffi::c_int as uint64_t,
    0x3b6db as ::core::ffi::c_int as uint64_t,
    0xbadb as ::core::ffi::c_int as uint64_t,
    0x1badb as ::core::ffi::c_int as uint64_t,
    0x2badb as ::core::ffi::c_int as uint64_t,
    0x3badb as ::core::ffi::c_int as uint64_t,
    0xbedb as ::core::ffi::c_int as uint64_t,
    0x1bedb as ::core::ffi::c_int as uint64_t,
    0x2bedb as ::core::ffi::c_int as uint64_t,
    0x3bedb as ::core::ffi::c_int as uint64_t,
    0xb2eb as ::core::ffi::c_int as uint64_t,
    0x1b2eb as ::core::ffi::c_int as uint64_t,
    0x2b2eb as ::core::ffi::c_int as uint64_t,
    0x3b2eb as ::core::ffi::c_int as uint64_t,
    0xb6eb as ::core::ffi::c_int as uint64_t,
    0x1b6eb as ::core::ffi::c_int as uint64_t,
    0x2b6eb as ::core::ffi::c_int as uint64_t,
    0x3b6eb as ::core::ffi::c_int as uint64_t,
    0xbaeb as ::core::ffi::c_int as uint64_t,
    0x1baeb as ::core::ffi::c_int as uint64_t,
    0x2baeb as ::core::ffi::c_int as uint64_t,
    0x3baeb as ::core::ffi::c_int as uint64_t,
    0xbeeb as ::core::ffi::c_int as uint64_t,
    0x1beeb as ::core::ffi::c_int as uint64_t,
    0x2beeb as ::core::ffi::c_int as uint64_t,
    0x3beeb as ::core::ffi::c_int as uint64_t,
    0xb2fb as ::core::ffi::c_int as uint64_t,
    0x1b2fb as ::core::ffi::c_int as uint64_t,
    0x2b2fb as ::core::ffi::c_int as uint64_t,
    0x3b2fb as ::core::ffi::c_int as uint64_t,
    0xb6fb as ::core::ffi::c_int as uint64_t,
    0x1b6fb as ::core::ffi::c_int as uint64_t,
    0x2b6fb as ::core::ffi::c_int as uint64_t,
    0x3b6fb as ::core::ffi::c_int as uint64_t,
    0xbafb as ::core::ffi::c_int as uint64_t,
    0x1bafb as ::core::ffi::c_int as uint64_t,
    0x2bafb as ::core::ffi::c_int as uint64_t,
    0x3bafb as ::core::ffi::c_int as uint64_t,
    0xbefb as ::core::ffi::c_int as uint64_t,
    0x1befb as ::core::ffi::c_int as uint64_t,
    0x2befb as ::core::ffi::c_int as uint64_t,
    0x3befb as ::core::ffi::c_int as uint64_t,
    0x2cb2cb as ::core::ffi::c_int as uint64_t,
    0x6cb2cb as ::core::ffi::c_int as uint64_t,
    0xacb2cb as ::core::ffi::c_int as uint64_t,
    0xecb2cb as ::core::ffi::c_int as uint64_t,
    0x2db2cb as ::core::ffi::c_int as uint64_t,
    0x6db2cb as ::core::ffi::c_int as uint64_t,
    0xadb2cb as ::core::ffi::c_int as uint64_t,
    0xedb2cb as ::core::ffi::c_int as uint64_t,
    0x2eb2cb as ::core::ffi::c_int as uint64_t,
    0x6eb2cb as ::core::ffi::c_int as uint64_t,
    0xaeb2cb as ::core::ffi::c_int as uint64_t,
    0xeeb2cb as ::core::ffi::c_int as uint64_t,
    0x2fb2cb as ::core::ffi::c_int as uint64_t,
    0x6fb2cb as ::core::ffi::c_int as uint64_t,
    0xafb2cb as ::core::ffi::c_int as uint64_t,
    0xefb2cb as ::core::ffi::c_int as uint64_t,
    0x2cb6cb as ::core::ffi::c_int as uint64_t,
    0x6cb6cb as ::core::ffi::c_int as uint64_t,
    0xacb6cb as ::core::ffi::c_int as uint64_t,
    0xecb6cb as ::core::ffi::c_int as uint64_t,
    0x2db6cb as ::core::ffi::c_int as uint64_t,
    0x6db6cb as ::core::ffi::c_int as uint64_t,
    0xadb6cb as ::core::ffi::c_int as uint64_t,
    0xedb6cb as ::core::ffi::c_int as uint64_t,
    0x2eb6cb as ::core::ffi::c_int as uint64_t,
    0x6eb6cb as ::core::ffi::c_int as uint64_t,
    0xaeb6cb as ::core::ffi::c_int as uint64_t,
    0xeeb6cb as ::core::ffi::c_int as uint64_t,
    0x2fb6cb as ::core::ffi::c_int as uint64_t,
    0x6fb6cb as ::core::ffi::c_int as uint64_t,
    0xafb6cb as ::core::ffi::c_int as uint64_t,
    0xefb6cb as ::core::ffi::c_int as uint64_t,
    0x2cbacb as ::core::ffi::c_int as uint64_t,
    0x6cbacb as ::core::ffi::c_int as uint64_t,
    0xacbacb as ::core::ffi::c_int as uint64_t,
    0xecbacb as ::core::ffi::c_int as uint64_t,
    0x2dbacb as ::core::ffi::c_int as uint64_t,
    0x6dbacb as ::core::ffi::c_int as uint64_t,
    0xadbacb as ::core::ffi::c_int as uint64_t,
    0xedbacb as ::core::ffi::c_int as uint64_t,
    0x2ebacb as ::core::ffi::c_int as uint64_t,
    0x6ebacb as ::core::ffi::c_int as uint64_t,
    0xaebacb as ::core::ffi::c_int as uint64_t,
    0xeebacb as ::core::ffi::c_int as uint64_t,
    0x2fbacb as ::core::ffi::c_int as uint64_t,
    0x6fbacb as ::core::ffi::c_int as uint64_t,
    0xafbacb as ::core::ffi::c_int as uint64_t,
    0xefbacb as ::core::ffi::c_int as uint64_t,
    0x2cbecb as ::core::ffi::c_int as uint64_t,
    0x6cbecb as ::core::ffi::c_int as uint64_t,
    0xacbecb as ::core::ffi::c_int as uint64_t,
    0xecbecb as ::core::ffi::c_int as uint64_t,
    0x2dbecb as ::core::ffi::c_int as uint64_t,
    0x6dbecb as ::core::ffi::c_int as uint64_t,
    0xadbecb as ::core::ffi::c_int as uint64_t,
    0xedbecb as ::core::ffi::c_int as uint64_t,
    0x2ebecb as ::core::ffi::c_int as uint64_t,
    0x6ebecb as ::core::ffi::c_int as uint64_t,
    0xaebecb as ::core::ffi::c_int as uint64_t,
    0xeebecb as ::core::ffi::c_int as uint64_t,
    0x2fbecb as ::core::ffi::c_int as uint64_t,
    0x6fbecb as ::core::ffi::c_int as uint64_t,
    0xafbecb as ::core::ffi::c_int as uint64_t,
    0xefbecb as ::core::ffi::c_int as uint64_t,
    0x2cb2db as ::core::ffi::c_int as uint64_t,
    0x6cb2db as ::core::ffi::c_int as uint64_t,
    0xacb2db as ::core::ffi::c_int as uint64_t,
    0xecb2db as ::core::ffi::c_int as uint64_t,
    0x2db2db as ::core::ffi::c_int as uint64_t,
    0x6db2db as ::core::ffi::c_int as uint64_t,
    0xadb2db as ::core::ffi::c_int as uint64_t,
    0xedb2db as ::core::ffi::c_int as uint64_t,
    0x2eb2db as ::core::ffi::c_int as uint64_t,
    0x6eb2db as ::core::ffi::c_int as uint64_t,
    0xaeb2db as ::core::ffi::c_int as uint64_t,
    0xeeb2db as ::core::ffi::c_int as uint64_t,
    0x2fb2db as ::core::ffi::c_int as uint64_t,
    0x6fb2db as ::core::ffi::c_int as uint64_t,
    0xafb2db as ::core::ffi::c_int as uint64_t,
    0xefb2db as ::core::ffi::c_int as uint64_t,
    0x2cb6db as ::core::ffi::c_int as uint64_t,
    0x6cb6db as ::core::ffi::c_int as uint64_t,
    0xacb6db as ::core::ffi::c_int as uint64_t,
    0xecb6db as ::core::ffi::c_int as uint64_t,
    0x2db6db as ::core::ffi::c_int as uint64_t,
    0x6db6db as ::core::ffi::c_int as uint64_t,
    0xadb6db as ::core::ffi::c_int as uint64_t,
    0xedb6db as ::core::ffi::c_int as uint64_t,
    0x2eb6db as ::core::ffi::c_int as uint64_t,
    0x6eb6db as ::core::ffi::c_int as uint64_t,
    0xaeb6db as ::core::ffi::c_int as uint64_t,
    0xeeb6db as ::core::ffi::c_int as uint64_t,
    0x2fb6db as ::core::ffi::c_int as uint64_t,
    0x6fb6db as ::core::ffi::c_int as uint64_t,
    0xafb6db as ::core::ffi::c_int as uint64_t,
    0xefb6db as ::core::ffi::c_int as uint64_t,
    0x2cbadb as ::core::ffi::c_int as uint64_t,
    0x6cbadb as ::core::ffi::c_int as uint64_t,
    0xacbadb as ::core::ffi::c_int as uint64_t,
    0xecbadb as ::core::ffi::c_int as uint64_t,
    0x2dbadb as ::core::ffi::c_int as uint64_t,
    0x6dbadb as ::core::ffi::c_int as uint64_t,
    0xadbadb as ::core::ffi::c_int as uint64_t,
    0xedbadb as ::core::ffi::c_int as uint64_t,
    0x2ebadb as ::core::ffi::c_int as uint64_t,
    0x6ebadb as ::core::ffi::c_int as uint64_t,
    0xaebadb as ::core::ffi::c_int as uint64_t,
    0xeebadb as ::core::ffi::c_int as uint64_t,
    0x2fbadb as ::core::ffi::c_int as uint64_t,
    0x6fbadb as ::core::ffi::c_int as uint64_t,
    0xafbadb as ::core::ffi::c_int as uint64_t,
    0xefbadb as ::core::ffi::c_int as uint64_t,
    0x2cbedb as ::core::ffi::c_int as uint64_t,
    0x6cbedb as ::core::ffi::c_int as uint64_t,
    0xacbedb as ::core::ffi::c_int as uint64_t,
    0xecbedb as ::core::ffi::c_int as uint64_t,
    0x2dbedb as ::core::ffi::c_int as uint64_t,
    0x6dbedb as ::core::ffi::c_int as uint64_t,
    0xadbedb as ::core::ffi::c_int as uint64_t,
    0xedbedb as ::core::ffi::c_int as uint64_t,
    0x2ebedb as ::core::ffi::c_int as uint64_t,
    0x6ebedb as ::core::ffi::c_int as uint64_t,
    0xaebedb as ::core::ffi::c_int as uint64_t,
    0xeebedb as ::core::ffi::c_int as uint64_t,
    0x2fbedb as ::core::ffi::c_int as uint64_t,
    0x6fbedb as ::core::ffi::c_int as uint64_t,
    0xafbedb as ::core::ffi::c_int as uint64_t,
    0xefbedb as ::core::ffi::c_int as uint64_t,
    0x2cb2eb as ::core::ffi::c_int as uint64_t,
    0x6cb2eb as ::core::ffi::c_int as uint64_t,
    0xacb2eb as ::core::ffi::c_int as uint64_t,
    0xecb2eb as ::core::ffi::c_int as uint64_t,
    0x2db2eb as ::core::ffi::c_int as uint64_t,
    0x6db2eb as ::core::ffi::c_int as uint64_t,
    0xadb2eb as ::core::ffi::c_int as uint64_t,
    0xedb2eb as ::core::ffi::c_int as uint64_t,
    0x2eb2eb as ::core::ffi::c_int as uint64_t,
    0x6eb2eb as ::core::ffi::c_int as uint64_t,
    0xaeb2eb as ::core::ffi::c_int as uint64_t,
    0xeeb2eb as ::core::ffi::c_int as uint64_t,
    0x2fb2eb as ::core::ffi::c_int as uint64_t,
    0x6fb2eb as ::core::ffi::c_int as uint64_t,
    0xafb2eb as ::core::ffi::c_int as uint64_t,
    0xefb2eb as ::core::ffi::c_int as uint64_t,
    0x2cb6eb as ::core::ffi::c_int as uint64_t,
    0x6cb6eb as ::core::ffi::c_int as uint64_t,
    0xacb6eb as ::core::ffi::c_int as uint64_t,
    0xecb6eb as ::core::ffi::c_int as uint64_t,
    0x2db6eb as ::core::ffi::c_int as uint64_t,
    0x6db6eb as ::core::ffi::c_int as uint64_t,
    0xadb6eb as ::core::ffi::c_int as uint64_t,
    0xedb6eb as ::core::ffi::c_int as uint64_t,
    0x2eb6eb as ::core::ffi::c_int as uint64_t,
    0x6eb6eb as ::core::ffi::c_int as uint64_t,
    0xaeb6eb as ::core::ffi::c_int as uint64_t,
    0xeeb6eb as ::core::ffi::c_int as uint64_t,
    0x2fb6eb as ::core::ffi::c_int as uint64_t,
    0x6fb6eb as ::core::ffi::c_int as uint64_t,
    0xafb6eb as ::core::ffi::c_int as uint64_t,
    0xefb6eb as ::core::ffi::c_int as uint64_t,
    0x2cbaeb as ::core::ffi::c_int as uint64_t,
    0x6cbaeb as ::core::ffi::c_int as uint64_t,
    0xacbaeb as ::core::ffi::c_int as uint64_t,
    0xecbaeb as ::core::ffi::c_int as uint64_t,
    0x2dbaeb as ::core::ffi::c_int as uint64_t,
    0x6dbaeb as ::core::ffi::c_int as uint64_t,
    0xadbaeb as ::core::ffi::c_int as uint64_t,
    0xedbaeb as ::core::ffi::c_int as uint64_t,
    0x2ebaeb as ::core::ffi::c_int as uint64_t,
    0x6ebaeb as ::core::ffi::c_int as uint64_t,
    0xaebaeb as ::core::ffi::c_int as uint64_t,
    0xeebaeb as ::core::ffi::c_int as uint64_t,
    0x2fbaeb as ::core::ffi::c_int as uint64_t,
    0x6fbaeb as ::core::ffi::c_int as uint64_t,
    0xafbaeb as ::core::ffi::c_int as uint64_t,
    0xefbaeb as ::core::ffi::c_int as uint64_t,
    0x2cbeeb as ::core::ffi::c_int as uint64_t,
    0x6cbeeb as ::core::ffi::c_int as uint64_t,
    0xacbeeb as ::core::ffi::c_int as uint64_t,
    0xecbeeb as ::core::ffi::c_int as uint64_t,
    0x2dbeeb as ::core::ffi::c_int as uint64_t,
    0x6dbeeb as ::core::ffi::c_int as uint64_t,
    0xadbeeb as ::core::ffi::c_int as uint64_t,
    0xedbeeb as ::core::ffi::c_int as uint64_t,
    0x2ebeeb as ::core::ffi::c_int as uint64_t,
    0x6ebeeb as ::core::ffi::c_int as uint64_t,
    0xaebeeb as ::core::ffi::c_int as uint64_t,
    0xeebeeb as ::core::ffi::c_int as uint64_t,
    0x2fbeeb as ::core::ffi::c_int as uint64_t,
    0x6fbeeb as ::core::ffi::c_int as uint64_t,
    0xafbeeb as ::core::ffi::c_int as uint64_t,
    0xefbeeb as ::core::ffi::c_int as uint64_t,
    0x2cb2fb as ::core::ffi::c_int as uint64_t,
    0x6cb2fb as ::core::ffi::c_int as uint64_t,
    0xacb2fb as ::core::ffi::c_int as uint64_t,
    0xecb2fb as ::core::ffi::c_int as uint64_t,
    0x2db2fb as ::core::ffi::c_int as uint64_t,
    0x6db2fb as ::core::ffi::c_int as uint64_t,
    0xadb2fb as ::core::ffi::c_int as uint64_t,
    0xedb2fb as ::core::ffi::c_int as uint64_t,
    0x2eb2fb as ::core::ffi::c_int as uint64_t,
    0x6eb2fb as ::core::ffi::c_int as uint64_t,
    0xaeb2fb as ::core::ffi::c_int as uint64_t,
    0xeeb2fb as ::core::ffi::c_int as uint64_t,
    0x2fb2fb as ::core::ffi::c_int as uint64_t,
    0x6fb2fb as ::core::ffi::c_int as uint64_t,
    0xafb2fb as ::core::ffi::c_int as uint64_t,
    0xefb2fb as ::core::ffi::c_int as uint64_t,
    0x2cb6fb as ::core::ffi::c_int as uint64_t,
    0x6cb6fb as ::core::ffi::c_int as uint64_t,
    0xacb6fb as ::core::ffi::c_int as uint64_t,
    0xecb6fb as ::core::ffi::c_int as uint64_t,
    0x2db6fb as ::core::ffi::c_int as uint64_t,
    0x6db6fb as ::core::ffi::c_int as uint64_t,
    0xadb6fb as ::core::ffi::c_int as uint64_t,
    0xedb6fb as ::core::ffi::c_int as uint64_t,
    0x2eb6fb as ::core::ffi::c_int as uint64_t,
    0x6eb6fb as ::core::ffi::c_int as uint64_t,
    0xaeb6fb as ::core::ffi::c_int as uint64_t,
    0xeeb6fb as ::core::ffi::c_int as uint64_t,
    0x2fb6fb as ::core::ffi::c_int as uint64_t,
    0x6fb6fb as ::core::ffi::c_int as uint64_t,
    0xafb6fb as ::core::ffi::c_int as uint64_t,
    0xefb6fb as ::core::ffi::c_int as uint64_t,
    0x2cbafb as ::core::ffi::c_int as uint64_t,
    0x6cbafb as ::core::ffi::c_int as uint64_t,
    0xacbafb as ::core::ffi::c_int as uint64_t,
    0xecbafb as ::core::ffi::c_int as uint64_t,
    0x2dbafb as ::core::ffi::c_int as uint64_t,
    0x6dbafb as ::core::ffi::c_int as uint64_t,
    0xadbafb as ::core::ffi::c_int as uint64_t,
    0xedbafb as ::core::ffi::c_int as uint64_t,
    0x2ebafb as ::core::ffi::c_int as uint64_t,
    0x6ebafb as ::core::ffi::c_int as uint64_t,
    0xaebafb as ::core::ffi::c_int as uint64_t,
    0xeebafb as ::core::ffi::c_int as uint64_t,
    0x2fbafb as ::core::ffi::c_int as uint64_t,
    0x6fbafb as ::core::ffi::c_int as uint64_t,
    0xafbafb as ::core::ffi::c_int as uint64_t,
    0xefbafb as ::core::ffi::c_int as uint64_t,
    0x2cbefb as ::core::ffi::c_int as uint64_t,
    0x6cbefb as ::core::ffi::c_int as uint64_t,
    0xacbefb as ::core::ffi::c_int as uint64_t,
    0xecbefb as ::core::ffi::c_int as uint64_t,
    0x2dbefb as ::core::ffi::c_int as uint64_t,
    0x6dbefb as ::core::ffi::c_int as uint64_t,
    0xadbefb as ::core::ffi::c_int as uint64_t,
    0xedbefb as ::core::ffi::c_int as uint64_t,
    0x2ebefb as ::core::ffi::c_int as uint64_t,
    0x6ebefb as ::core::ffi::c_int as uint64_t,
    0xaebefb as ::core::ffi::c_int as uint64_t,
    0xeebefb as ::core::ffi::c_int as uint64_t,
    0x2fbefb as ::core::ffi::c_int as uint64_t,
    0x6fbefb as ::core::ffi::c_int as uint64_t,
    0xafbefb as ::core::ffi::c_int as uint64_t,
    0xefbefb as ::core::ffi::c_int as uint64_t,
    0xb2cb2cb as ::core::ffi::c_int as uint64_t,
    0x1b2cb2cb as ::core::ffi::c_int as uint64_t,
    0x2b2cb2cb as ::core::ffi::c_int as uint64_t,
    0x3b2cb2cb as ::core::ffi::c_int as uint64_t,
    0xb6cb2cb as ::core::ffi::c_int as uint64_t,
    0x1b6cb2cb as ::core::ffi::c_int as uint64_t,
    0x2b6cb2cb as ::core::ffi::c_int as uint64_t,
    0x3b6cb2cb as ::core::ffi::c_int as uint64_t,
    0xbacb2cb as ::core::ffi::c_int as uint64_t,
    0x1bacb2cb as ::core::ffi::c_int as uint64_t,
    0x2bacb2cb as ::core::ffi::c_int as uint64_t,
    0x3bacb2cb as ::core::ffi::c_int as uint64_t,
    0xbecb2cb as ::core::ffi::c_int as uint64_t,
    0x1becb2cb as ::core::ffi::c_int as uint64_t,
    0x2becb2cb as ::core::ffi::c_int as uint64_t,
    0x3becb2cb as ::core::ffi::c_int as uint64_t,
    0xb2db2cb as ::core::ffi::c_int as uint64_t,
    0x1b2db2cb as ::core::ffi::c_int as uint64_t,
    0x2b2db2cb as ::core::ffi::c_int as uint64_t,
    0x3b2db2cb as ::core::ffi::c_int as uint64_t,
    0xb6db2cb as ::core::ffi::c_int as uint64_t,
    0x1b6db2cb as ::core::ffi::c_int as uint64_t,
    0x2b6db2cb as ::core::ffi::c_int as uint64_t,
    0x3b6db2cb as ::core::ffi::c_int as uint64_t,
    0xbadb2cb as ::core::ffi::c_int as uint64_t,
    0x1badb2cb as ::core::ffi::c_int as uint64_t,
    0x2badb2cb as ::core::ffi::c_int as uint64_t,
    0x3badb2cb as ::core::ffi::c_int as uint64_t,
    0xbedb2cb as ::core::ffi::c_int as uint64_t,
    0x1bedb2cb as ::core::ffi::c_int as uint64_t,
    0x2bedb2cb as ::core::ffi::c_int as uint64_t,
    0x3bedb2cb as ::core::ffi::c_int as uint64_t,
    0xb2eb2cb as ::core::ffi::c_int as uint64_t,
    0x1b2eb2cb as ::core::ffi::c_int as uint64_t,
    0x2b2eb2cb as ::core::ffi::c_int as uint64_t,
    0x3b2eb2cb as ::core::ffi::c_int as uint64_t,
    0xb6eb2cb as ::core::ffi::c_int as uint64_t,
    0x1b6eb2cb as ::core::ffi::c_int as uint64_t,
    0x2b6eb2cb as ::core::ffi::c_int as uint64_t,
    0x3b6eb2cb as ::core::ffi::c_int as uint64_t,
    0xbaeb2cb as ::core::ffi::c_int as uint64_t,
    0x1baeb2cb as ::core::ffi::c_int as uint64_t,
    0x2baeb2cb as ::core::ffi::c_int as uint64_t,
    0x3baeb2cb as ::core::ffi::c_int as uint64_t,
    0xbeeb2cb as ::core::ffi::c_int as uint64_t,
    0x1beeb2cb as ::core::ffi::c_int as uint64_t,
    0x2beeb2cb as ::core::ffi::c_int as uint64_t,
    0x3beeb2cb as ::core::ffi::c_int as uint64_t,
    0xb2fb2cb as ::core::ffi::c_int as uint64_t,
    0x1b2fb2cb as ::core::ffi::c_int as uint64_t,
    0x2b2fb2cb as ::core::ffi::c_int as uint64_t,
    0x3b2fb2cb as ::core::ffi::c_int as uint64_t,
    0xb6fb2cb as ::core::ffi::c_int as uint64_t,
    0x1b6fb2cb as ::core::ffi::c_int as uint64_t,
    0x2b6fb2cb as ::core::ffi::c_int as uint64_t,
    0x3b6fb2cb as ::core::ffi::c_int as uint64_t,
    0xbafb2cb as ::core::ffi::c_int as uint64_t,
    0x1bafb2cb as ::core::ffi::c_int as uint64_t,
    0x2bafb2cb as ::core::ffi::c_int as uint64_t,
    0x3bafb2cb as ::core::ffi::c_int as uint64_t,
    0xbefb2cb as ::core::ffi::c_int as uint64_t,
    0x1befb2cb as ::core::ffi::c_int as uint64_t,
    0x2befb2cb as ::core::ffi::c_int as uint64_t,
    0x3befb2cb as ::core::ffi::c_int as uint64_t,
    0xb2cb6cb as ::core::ffi::c_int as uint64_t,
    0x1b2cb6cb as ::core::ffi::c_int as uint64_t,
    0x2b2cb6cb as ::core::ffi::c_int as uint64_t,
    0x3b2cb6cb as ::core::ffi::c_int as uint64_t,
    0xb6cb6cb as ::core::ffi::c_int as uint64_t,
    0x1b6cb6cb as ::core::ffi::c_int as uint64_t,
    0x2b6cb6cb as ::core::ffi::c_int as uint64_t,
    0x3b6cb6cb as ::core::ffi::c_int as uint64_t,
    0xbacb6cb as ::core::ffi::c_int as uint64_t,
    0x1bacb6cb as ::core::ffi::c_int as uint64_t,
    0x2bacb6cb as ::core::ffi::c_int as uint64_t,
    0x3bacb6cb as ::core::ffi::c_int as uint64_t,
    0xbecb6cb as ::core::ffi::c_int as uint64_t,
    0x1becb6cb as ::core::ffi::c_int as uint64_t,
    0x2becb6cb as ::core::ffi::c_int as uint64_t,
    0x3becb6cb as ::core::ffi::c_int as uint64_t,
    0xb2db6cb as ::core::ffi::c_int as uint64_t,
    0x1b2db6cb as ::core::ffi::c_int as uint64_t,
    0x2b2db6cb as ::core::ffi::c_int as uint64_t,
    0x3b2db6cb as ::core::ffi::c_int as uint64_t,
    0xb6db6cb as ::core::ffi::c_int as uint64_t,
    0x1b6db6cb as ::core::ffi::c_int as uint64_t,
    0x2b6db6cb as ::core::ffi::c_int as uint64_t,
    0x3b6db6cb as ::core::ffi::c_int as uint64_t,
    0xbadb6cb as ::core::ffi::c_int as uint64_t,
    0x1badb6cb as ::core::ffi::c_int as uint64_t,
    0x2badb6cb as ::core::ffi::c_int as uint64_t,
    0x3badb6cb as ::core::ffi::c_int as uint64_t,
    0xbedb6cb as ::core::ffi::c_int as uint64_t,
    0x1bedb6cb as ::core::ffi::c_int as uint64_t,
    0x2bedb6cb as ::core::ffi::c_int as uint64_t,
    0x3bedb6cb as ::core::ffi::c_int as uint64_t,
    0xb2eb6cb as ::core::ffi::c_int as uint64_t,
    0x1b2eb6cb as ::core::ffi::c_int as uint64_t,
    0x2b2eb6cb as ::core::ffi::c_int as uint64_t,
    0x3b2eb6cb as ::core::ffi::c_int as uint64_t,
    0xb6eb6cb as ::core::ffi::c_int as uint64_t,
    0x1b6eb6cb as ::core::ffi::c_int as uint64_t,
    0x2b6eb6cb as ::core::ffi::c_int as uint64_t,
    0x3b6eb6cb as ::core::ffi::c_int as uint64_t,
    0xbaeb6cb as ::core::ffi::c_int as uint64_t,
    0x1baeb6cb as ::core::ffi::c_int as uint64_t,
    0x2baeb6cb as ::core::ffi::c_int as uint64_t,
    0x3baeb6cb as ::core::ffi::c_int as uint64_t,
    0xbeeb6cb as ::core::ffi::c_int as uint64_t,
    0x1beeb6cb as ::core::ffi::c_int as uint64_t,
    0x2beeb6cb as ::core::ffi::c_int as uint64_t,
    0x3beeb6cb as ::core::ffi::c_int as uint64_t,
    0xb2fb6cb as ::core::ffi::c_int as uint64_t,
    0x1b2fb6cb as ::core::ffi::c_int as uint64_t,
    0x2b2fb6cb as ::core::ffi::c_int as uint64_t,
    0x3b2fb6cb as ::core::ffi::c_int as uint64_t,
    0xb6fb6cb as ::core::ffi::c_int as uint64_t,
    0x1b6fb6cb as ::core::ffi::c_int as uint64_t,
    0x2b6fb6cb as ::core::ffi::c_int as uint64_t,
    0x3b6fb6cb as ::core::ffi::c_int as uint64_t,
    0xbafb6cb as ::core::ffi::c_int as uint64_t,
    0x1bafb6cb as ::core::ffi::c_int as uint64_t,
    0x2bafb6cb as ::core::ffi::c_int as uint64_t,
    0x3bafb6cb as ::core::ffi::c_int as uint64_t,
    0xbefb6cb as ::core::ffi::c_int as uint64_t,
    0x1befb6cb as ::core::ffi::c_int as uint64_t,
    0x2befb6cb as ::core::ffi::c_int as uint64_t,
    0x3befb6cb as ::core::ffi::c_int as uint64_t,
    0xb2cbacb as ::core::ffi::c_int as uint64_t,
    0x1b2cbacb as ::core::ffi::c_int as uint64_t,
    0x2b2cbacb as ::core::ffi::c_int as uint64_t,
    0x3b2cbacb as ::core::ffi::c_int as uint64_t,
    0xb6cbacb as ::core::ffi::c_int as uint64_t,
    0x1b6cbacb as ::core::ffi::c_int as uint64_t,
    0x2b6cbacb as ::core::ffi::c_int as uint64_t,
    0x3b6cbacb as ::core::ffi::c_int as uint64_t,
    0xbacbacb as ::core::ffi::c_int as uint64_t,
    0x1bacbacb as ::core::ffi::c_int as uint64_t,
    0x2bacbacb as ::core::ffi::c_int as uint64_t,
    0x3bacbacb as ::core::ffi::c_int as uint64_t,
    0xbecbacb as ::core::ffi::c_int as uint64_t,
    0x1becbacb as ::core::ffi::c_int as uint64_t,
    0x2becbacb as ::core::ffi::c_int as uint64_t,
    0x3becbacb as ::core::ffi::c_int as uint64_t,
    0xb2dbacb as ::core::ffi::c_int as uint64_t,
    0x1b2dbacb as ::core::ffi::c_int as uint64_t,
    0x2b2dbacb as ::core::ffi::c_int as uint64_t,
    0x3b2dbacb as ::core::ffi::c_int as uint64_t,
    0xb6dbacb as ::core::ffi::c_int as uint64_t,
    0x1b6dbacb as ::core::ffi::c_int as uint64_t,
    0x2b6dbacb as ::core::ffi::c_int as uint64_t,
    0x3b6dbacb as ::core::ffi::c_int as uint64_t,
    0xbadbacb as ::core::ffi::c_int as uint64_t,
    0x1badbacb as ::core::ffi::c_int as uint64_t,
    0x2badbacb as ::core::ffi::c_int as uint64_t,
    0x3badbacb as ::core::ffi::c_int as uint64_t,
    0xbedbacb as ::core::ffi::c_int as uint64_t,
    0x1bedbacb as ::core::ffi::c_int as uint64_t,
    0x2bedbacb as ::core::ffi::c_int as uint64_t,
    0x3bedbacb as ::core::ffi::c_int as uint64_t,
    0xb2ebacb as ::core::ffi::c_int as uint64_t,
    0x1b2ebacb as ::core::ffi::c_int as uint64_t,
    0x2b2ebacb as ::core::ffi::c_int as uint64_t,
    0x3b2ebacb as ::core::ffi::c_int as uint64_t,
    0xb6ebacb as ::core::ffi::c_int as uint64_t,
    0x1b6ebacb as ::core::ffi::c_int as uint64_t,
    0x2b6ebacb as ::core::ffi::c_int as uint64_t,
    0x3b6ebacb as ::core::ffi::c_int as uint64_t,
    0xbaebacb as ::core::ffi::c_int as uint64_t,
    0x1baebacb as ::core::ffi::c_int as uint64_t,
    0x2baebacb as ::core::ffi::c_int as uint64_t,
    0x3baebacb as ::core::ffi::c_int as uint64_t,
    0xbeebacb as ::core::ffi::c_int as uint64_t,
    0x1beebacb as ::core::ffi::c_int as uint64_t,
    0x2beebacb as ::core::ffi::c_int as uint64_t,
    0x3beebacb as ::core::ffi::c_int as uint64_t,
    0xb2fbacb as ::core::ffi::c_int as uint64_t,
    0x1b2fbacb as ::core::ffi::c_int as uint64_t,
    0x2b2fbacb as ::core::ffi::c_int as uint64_t,
    0x3b2fbacb as ::core::ffi::c_int as uint64_t,
    0xb6fbacb as ::core::ffi::c_int as uint64_t,
    0x1b6fbacb as ::core::ffi::c_int as uint64_t,
    0x2b6fbacb as ::core::ffi::c_int as uint64_t,
    0x3b6fbacb as ::core::ffi::c_int as uint64_t,
    0xbafbacb as ::core::ffi::c_int as uint64_t,
    0x1bafbacb as ::core::ffi::c_int as uint64_t,
    0x2bafbacb as ::core::ffi::c_int as uint64_t,
    0x3bafbacb as ::core::ffi::c_int as uint64_t,
    0xbefbacb as ::core::ffi::c_int as uint64_t,
    0x1befbacb as ::core::ffi::c_int as uint64_t,
    0x2befbacb as ::core::ffi::c_int as uint64_t,
    0x3befbacb as ::core::ffi::c_int as uint64_t,
    0xb2cbecb as ::core::ffi::c_int as uint64_t,
    0x1b2cbecb as ::core::ffi::c_int as uint64_t,
    0x2b2cbecb as ::core::ffi::c_int as uint64_t,
    0x3b2cbecb as ::core::ffi::c_int as uint64_t,
    0xb6cbecb as ::core::ffi::c_int as uint64_t,
    0x1b6cbecb as ::core::ffi::c_int as uint64_t,
    0x2b6cbecb as ::core::ffi::c_int as uint64_t,
    0x3b6cbecb as ::core::ffi::c_int as uint64_t,
    0xbacbecb as ::core::ffi::c_int as uint64_t,
    0x1bacbecb as ::core::ffi::c_int as uint64_t,
    0x2bacbecb as ::core::ffi::c_int as uint64_t,
    0x3bacbecb as ::core::ffi::c_int as uint64_t,
    0xbecbecb as ::core::ffi::c_int as uint64_t,
    0x1becbecb as ::core::ffi::c_int as uint64_t,
    0x2becbecb as ::core::ffi::c_int as uint64_t,
    0x3becbecb as ::core::ffi::c_int as uint64_t,
    0xb2dbecb as ::core::ffi::c_int as uint64_t,
    0x1b2dbecb as ::core::ffi::c_int as uint64_t,
    0x2b2dbecb as ::core::ffi::c_int as uint64_t,
    0x3b2dbecb as ::core::ffi::c_int as uint64_t,
    0xb6dbecb as ::core::ffi::c_int as uint64_t,
    0x1b6dbecb as ::core::ffi::c_int as uint64_t,
    0x2b6dbecb as ::core::ffi::c_int as uint64_t,
    0x3b6dbecb as ::core::ffi::c_int as uint64_t,
    0xbadbecb as ::core::ffi::c_int as uint64_t,
    0x1badbecb as ::core::ffi::c_int as uint64_t,
    0x2badbecb as ::core::ffi::c_int as uint64_t,
    0x3badbecb as ::core::ffi::c_int as uint64_t,
    0xbedbecb as ::core::ffi::c_int as uint64_t,
    0x1bedbecb as ::core::ffi::c_int as uint64_t,
    0x2bedbecb as ::core::ffi::c_int as uint64_t,
    0x3bedbecb as ::core::ffi::c_int as uint64_t,
    0xb2ebecb as ::core::ffi::c_int as uint64_t,
    0x1b2ebecb as ::core::ffi::c_int as uint64_t,
    0x2b2ebecb as ::core::ffi::c_int as uint64_t,
    0x3b2ebecb as ::core::ffi::c_int as uint64_t,
    0xb6ebecb as ::core::ffi::c_int as uint64_t,
    0x1b6ebecb as ::core::ffi::c_int as uint64_t,
    0x2b6ebecb as ::core::ffi::c_int as uint64_t,
    0x3b6ebecb as ::core::ffi::c_int as uint64_t,
    0xbaebecb as ::core::ffi::c_int as uint64_t,
    0x1baebecb as ::core::ffi::c_int as uint64_t,
    0x2baebecb as ::core::ffi::c_int as uint64_t,
    0x3baebecb as ::core::ffi::c_int as uint64_t,
    0xbeebecb as ::core::ffi::c_int as uint64_t,
    0x1beebecb as ::core::ffi::c_int as uint64_t,
    0x2beebecb as ::core::ffi::c_int as uint64_t,
    0x3beebecb as ::core::ffi::c_int as uint64_t,
    0xb2fbecb as ::core::ffi::c_int as uint64_t,
    0x1b2fbecb as ::core::ffi::c_int as uint64_t,
    0x2b2fbecb as ::core::ffi::c_int as uint64_t,
    0x3b2fbecb as ::core::ffi::c_int as uint64_t,
    0xb6fbecb as ::core::ffi::c_int as uint64_t,
    0x1b6fbecb as ::core::ffi::c_int as uint64_t,
    0x2b6fbecb as ::core::ffi::c_int as uint64_t,
    0x3b6fbecb as ::core::ffi::c_int as uint64_t,
    0xbafbecb as ::core::ffi::c_int as uint64_t,
    0x1bafbecb as ::core::ffi::c_int as uint64_t,
    0x2bafbecb as ::core::ffi::c_int as uint64_t,
    0x3bafbecb as ::core::ffi::c_int as uint64_t,
    0xbefbecb as ::core::ffi::c_int as uint64_t,
    0x1befbecb as ::core::ffi::c_int as uint64_t,
    0x2befbecb as ::core::ffi::c_int as uint64_t,
    0x3befbecb as ::core::ffi::c_int as uint64_t,
    0xb2cb2db as ::core::ffi::c_int as uint64_t,
    0x1b2cb2db as ::core::ffi::c_int as uint64_t,
    0x2b2cb2db as ::core::ffi::c_int as uint64_t,
    0x3b2cb2db as ::core::ffi::c_int as uint64_t,
    0xb6cb2db as ::core::ffi::c_int as uint64_t,
    0x1b6cb2db as ::core::ffi::c_int as uint64_t,
    0x2b6cb2db as ::core::ffi::c_int as uint64_t,
    0x3b6cb2db as ::core::ffi::c_int as uint64_t,
    0xbacb2db as ::core::ffi::c_int as uint64_t,
    0x1bacb2db as ::core::ffi::c_int as uint64_t,
    0x2bacb2db as ::core::ffi::c_int as uint64_t,
    0x3bacb2db as ::core::ffi::c_int as uint64_t,
    0xbecb2db as ::core::ffi::c_int as uint64_t,
    0x1becb2db as ::core::ffi::c_int as uint64_t,
    0x2becb2db as ::core::ffi::c_int as uint64_t,
    0x3becb2db as ::core::ffi::c_int as uint64_t,
    0xb2db2db as ::core::ffi::c_int as uint64_t,
    0x1b2db2db as ::core::ffi::c_int as uint64_t,
    0x2b2db2db as ::core::ffi::c_int as uint64_t,
    0x3b2db2db as ::core::ffi::c_int as uint64_t,
    0xb6db2db as ::core::ffi::c_int as uint64_t,
    0x1b6db2db as ::core::ffi::c_int as uint64_t,
    0x2b6db2db as ::core::ffi::c_int as uint64_t,
    0x3b6db2db as ::core::ffi::c_int as uint64_t,
    0xbadb2db as ::core::ffi::c_int as uint64_t,
    0x1badb2db as ::core::ffi::c_int as uint64_t,
    0x2badb2db as ::core::ffi::c_int as uint64_t,
    0x3badb2db as ::core::ffi::c_int as uint64_t,
    0xbedb2db as ::core::ffi::c_int as uint64_t,
    0x1bedb2db as ::core::ffi::c_int as uint64_t,
    0x2bedb2db as ::core::ffi::c_int as uint64_t,
    0x3bedb2db as ::core::ffi::c_int as uint64_t,
    0xb2eb2db as ::core::ffi::c_int as uint64_t,
    0x1b2eb2db as ::core::ffi::c_int as uint64_t,
    0x2b2eb2db as ::core::ffi::c_int as uint64_t,
    0x3b2eb2db as ::core::ffi::c_int as uint64_t,
    0xb6eb2db as ::core::ffi::c_int as uint64_t,
    0x1b6eb2db as ::core::ffi::c_int as uint64_t,
    0x2b6eb2db as ::core::ffi::c_int as uint64_t,
    0x3b6eb2db as ::core::ffi::c_int as uint64_t,
    0xbaeb2db as ::core::ffi::c_int as uint64_t,
    0x1baeb2db as ::core::ffi::c_int as uint64_t,
    0x2baeb2db as ::core::ffi::c_int as uint64_t,
    0x3baeb2db as ::core::ffi::c_int as uint64_t,
    0xbeeb2db as ::core::ffi::c_int as uint64_t,
    0x1beeb2db as ::core::ffi::c_int as uint64_t,
    0x2beeb2db as ::core::ffi::c_int as uint64_t,
    0x3beeb2db as ::core::ffi::c_int as uint64_t,
    0xb2fb2db as ::core::ffi::c_int as uint64_t,
    0x1b2fb2db as ::core::ffi::c_int as uint64_t,
    0x2b2fb2db as ::core::ffi::c_int as uint64_t,
    0x3b2fb2db as ::core::ffi::c_int as uint64_t,
    0xb6fb2db as ::core::ffi::c_int as uint64_t,
    0x1b6fb2db as ::core::ffi::c_int as uint64_t,
    0x2b6fb2db as ::core::ffi::c_int as uint64_t,
    0x3b6fb2db as ::core::ffi::c_int as uint64_t,
    0xbafb2db as ::core::ffi::c_int as uint64_t,
    0x1bafb2db as ::core::ffi::c_int as uint64_t,
    0x2bafb2db as ::core::ffi::c_int as uint64_t,
    0x3bafb2db as ::core::ffi::c_int as uint64_t,
    0xbefb2db as ::core::ffi::c_int as uint64_t,
    0x1befb2db as ::core::ffi::c_int as uint64_t,
    0x2befb2db as ::core::ffi::c_int as uint64_t,
    0x3befb2db as ::core::ffi::c_int as uint64_t,
    0xb2cb6db as ::core::ffi::c_int as uint64_t,
    0x1b2cb6db as ::core::ffi::c_int as uint64_t,
    0x2b2cb6db as ::core::ffi::c_int as uint64_t,
    0x3b2cb6db as ::core::ffi::c_int as uint64_t,
    0xb6cb6db as ::core::ffi::c_int as uint64_t,
    0x1b6cb6db as ::core::ffi::c_int as uint64_t,
    0x2b6cb6db as ::core::ffi::c_int as uint64_t,
    0x3b6cb6db as ::core::ffi::c_int as uint64_t,
    0xbacb6db as ::core::ffi::c_int as uint64_t,
    0x1bacb6db as ::core::ffi::c_int as uint64_t,
    0x2bacb6db as ::core::ffi::c_int as uint64_t,
    0x3bacb6db as ::core::ffi::c_int as uint64_t,
    0xbecb6db as ::core::ffi::c_int as uint64_t,
    0x1becb6db as ::core::ffi::c_int as uint64_t,
    0x2becb6db as ::core::ffi::c_int as uint64_t,
    0x3becb6db as ::core::ffi::c_int as uint64_t,
    0xb2db6db as ::core::ffi::c_int as uint64_t,
    0x1b2db6db as ::core::ffi::c_int as uint64_t,
    0x2b2db6db as ::core::ffi::c_int as uint64_t,
    0x3b2db6db as ::core::ffi::c_int as uint64_t,
    0xb6db6db as ::core::ffi::c_int as uint64_t,
    0x1b6db6db as ::core::ffi::c_int as uint64_t,
    0x2b6db6db as ::core::ffi::c_int as uint64_t,
    0x3b6db6db as ::core::ffi::c_int as uint64_t,
    0xbadb6db as ::core::ffi::c_int as uint64_t,
    0x1badb6db as ::core::ffi::c_int as uint64_t,
    0x2badb6db as ::core::ffi::c_int as uint64_t,
    0x3badb6db as ::core::ffi::c_int as uint64_t,
    0xbedb6db as ::core::ffi::c_int as uint64_t,
    0x1bedb6db as ::core::ffi::c_int as uint64_t,
    0x2bedb6db as ::core::ffi::c_int as uint64_t,
    0x3bedb6db as ::core::ffi::c_int as uint64_t,
    0xb2eb6db as ::core::ffi::c_int as uint64_t,
    0x1b2eb6db as ::core::ffi::c_int as uint64_t,
    0x2b2eb6db as ::core::ffi::c_int as uint64_t,
    0x3b2eb6db as ::core::ffi::c_int as uint64_t,
    0xb6eb6db as ::core::ffi::c_int as uint64_t,
    0x1b6eb6db as ::core::ffi::c_int as uint64_t,
    0x2b6eb6db as ::core::ffi::c_int as uint64_t,
    0x3b6eb6db as ::core::ffi::c_int as uint64_t,
    0xbaeb6db as ::core::ffi::c_int as uint64_t,
    0x1baeb6db as ::core::ffi::c_int as uint64_t,
    0x2baeb6db as ::core::ffi::c_int as uint64_t,
    0x3baeb6db as ::core::ffi::c_int as uint64_t,
];
static mut kNonZeroRepsDepth: [uint32_t; 704] = [
    6 as ::core::ffi::c_int as uint32_t,
    6 as ::core::ffi::c_int as uint32_t,
    6 as ::core::ffi::c_int as uint32_t,
    6 as ::core::ffi::c_int as uint32_t,
    12 as ::core::ffi::c_int as uint32_t,
    12 as ::core::ffi::c_int as uint32_t,
    12 as ::core::ffi::c_int as uint32_t,
    12 as ::core::ffi::c_int as uint32_t,
    12 as ::core::ffi::c_int as uint32_t,
    12 as ::core::ffi::c_int as uint32_t,
    12 as ::core::ffi::c_int as uint32_t,
    12 as ::core::ffi::c_int as uint32_t,
    12 as ::core::ffi::c_int as uint32_t,
    12 as ::core::ffi::c_int as uint32_t,
    12 as ::core::ffi::c_int as uint32_t,
    12 as ::core::ffi::c_int as uint32_t,
    12 as ::core::ffi::c_int as uint32_t,
    12 as ::core::ffi::c_int as uint32_t,
    12 as ::core::ffi::c_int as uint32_t,
    12 as ::core::ffi::c_int as uint32_t,
    18 as ::core::ffi::c_int as uint32_t,
    18 as ::core::ffi::c_int as uint32_t,
    18 as ::core::ffi::c_int as uint32_t,
    18 as ::core::ffi::c_int as uint32_t,
    18 as ::core::ffi::c_int as uint32_t,
    18 as ::core::ffi::c_int as uint32_t,
    18 as ::core::ffi::c_int as uint32_t,
    18 as ::core::ffi::c_int as uint32_t,
    18 as ::core::ffi::c_int as uint32_t,
    18 as ::core::ffi::c_int as uint32_t,
    18 as ::core::ffi::c_int as uint32_t,
    18 as ::core::ffi::c_int as uint32_t,
    18 as ::core::ffi::c_int as uint32_t,
    18 as ::core::ffi::c_int as uint32_t,
    18 as ::core::ffi::c_int as uint32_t,
    18 as ::core::ffi::c_int as uint32_t,
    18 as ::core::ffi::c_int as uint32_t,
    18 as ::core::ffi::c_int as uint32_t,
    18 as ::core::ffi::c_int as uint32_t,
    18 as ::core::ffi::c_int as uint32_t,
    18 as ::core::ffi::c_int as uint32_t,
    18 as ::core::ffi::c_int as uint32_t,
    18 as ::core::ffi::c_int as uint32_t,
    18 as ::core::ffi::c_int as uint32_t,
    18 as ::core::ffi::c_int as uint32_t,
    18 as ::core::ffi::c_int as uint32_t,
    18 as ::core::ffi::c_int as uint32_t,
    18 as ::core::ffi::c_int as uint32_t,
    18 as ::core::ffi::c_int as uint32_t,
    18 as ::core::ffi::c_int as uint32_t,
    18 as ::core::ffi::c_int as uint32_t,
    18 as ::core::ffi::c_int as uint32_t,
    18 as ::core::ffi::c_int as uint32_t,
    18 as ::core::ffi::c_int as uint32_t,
    18 as ::core::ffi::c_int as uint32_t,
    18 as ::core::ffi::c_int as uint32_t,
    18 as ::core::ffi::c_int as uint32_t,
    18 as ::core::ffi::c_int as uint32_t,
    18 as ::core::ffi::c_int as uint32_t,
    18 as ::core::ffi::c_int as uint32_t,
    18 as ::core::ffi::c_int as uint32_t,
    18 as ::core::ffi::c_int as uint32_t,
    18 as ::core::ffi::c_int as uint32_t,
    18 as ::core::ffi::c_int as uint32_t,
    18 as ::core::ffi::c_int as uint32_t,
    18 as ::core::ffi::c_int as uint32_t,
    18 as ::core::ffi::c_int as uint32_t,
    18 as ::core::ffi::c_int as uint32_t,
    18 as ::core::ffi::c_int as uint32_t,
    18 as ::core::ffi::c_int as uint32_t,
    18 as ::core::ffi::c_int as uint32_t,
    18 as ::core::ffi::c_int as uint32_t,
    18 as ::core::ffi::c_int as uint32_t,
    18 as ::core::ffi::c_int as uint32_t,
    18 as ::core::ffi::c_int as uint32_t,
    18 as ::core::ffi::c_int as uint32_t,
    18 as ::core::ffi::c_int as uint32_t,
    18 as ::core::ffi::c_int as uint32_t,
    18 as ::core::ffi::c_int as uint32_t,
    18 as ::core::ffi::c_int as uint32_t,
    18 as ::core::ffi::c_int as uint32_t,
    18 as ::core::ffi::c_int as uint32_t,
    18 as ::core::ffi::c_int as uint32_t,
    18 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    24 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
    30 as ::core::ffi::c_int as uint32_t,
];
static mut kStaticCommandCodeBits: [uint16_t; 704] = [
    0 as ::core::ffi::c_int as uint16_t,
    256 as ::core::ffi::c_int as uint16_t,
    128 as ::core::ffi::c_int as uint16_t,
    384 as ::core::ffi::c_int as uint16_t,
    64 as ::core::ffi::c_int as uint16_t,
    320 as ::core::ffi::c_int as uint16_t,
    192 as ::core::ffi::c_int as uint16_t,
    448 as ::core::ffi::c_int as uint16_t,
    32 as ::core::ffi::c_int as uint16_t,
    288 as ::core::ffi::c_int as uint16_t,
    160 as ::core::ffi::c_int as uint16_t,
    416 as ::core::ffi::c_int as uint16_t,
    96 as ::core::ffi::c_int as uint16_t,
    352 as ::core::ffi::c_int as uint16_t,
    224 as ::core::ffi::c_int as uint16_t,
    480 as ::core::ffi::c_int as uint16_t,
    16 as ::core::ffi::c_int as uint16_t,
    272 as ::core::ffi::c_int as uint16_t,
    144 as ::core::ffi::c_int as uint16_t,
    400 as ::core::ffi::c_int as uint16_t,
    80 as ::core::ffi::c_int as uint16_t,
    336 as ::core::ffi::c_int as uint16_t,
    208 as ::core::ffi::c_int as uint16_t,
    464 as ::core::ffi::c_int as uint16_t,
    48 as ::core::ffi::c_int as uint16_t,
    304 as ::core::ffi::c_int as uint16_t,
    176 as ::core::ffi::c_int as uint16_t,
    432 as ::core::ffi::c_int as uint16_t,
    112 as ::core::ffi::c_int as uint16_t,
    368 as ::core::ffi::c_int as uint16_t,
    240 as ::core::ffi::c_int as uint16_t,
    496 as ::core::ffi::c_int as uint16_t,
    8 as ::core::ffi::c_int as uint16_t,
    264 as ::core::ffi::c_int as uint16_t,
    136 as ::core::ffi::c_int as uint16_t,
    392 as ::core::ffi::c_int as uint16_t,
    72 as ::core::ffi::c_int as uint16_t,
    328 as ::core::ffi::c_int as uint16_t,
    200 as ::core::ffi::c_int as uint16_t,
    456 as ::core::ffi::c_int as uint16_t,
    40 as ::core::ffi::c_int as uint16_t,
    296 as ::core::ffi::c_int as uint16_t,
    168 as ::core::ffi::c_int as uint16_t,
    424 as ::core::ffi::c_int as uint16_t,
    104 as ::core::ffi::c_int as uint16_t,
    360 as ::core::ffi::c_int as uint16_t,
    232 as ::core::ffi::c_int as uint16_t,
    488 as ::core::ffi::c_int as uint16_t,
    24 as ::core::ffi::c_int as uint16_t,
    280 as ::core::ffi::c_int as uint16_t,
    152 as ::core::ffi::c_int as uint16_t,
    408 as ::core::ffi::c_int as uint16_t,
    88 as ::core::ffi::c_int as uint16_t,
    344 as ::core::ffi::c_int as uint16_t,
    216 as ::core::ffi::c_int as uint16_t,
    472 as ::core::ffi::c_int as uint16_t,
    56 as ::core::ffi::c_int as uint16_t,
    312 as ::core::ffi::c_int as uint16_t,
    184 as ::core::ffi::c_int as uint16_t,
    440 as ::core::ffi::c_int as uint16_t,
    120 as ::core::ffi::c_int as uint16_t,
    376 as ::core::ffi::c_int as uint16_t,
    248 as ::core::ffi::c_int as uint16_t,
    504 as ::core::ffi::c_int as uint16_t,
    4 as ::core::ffi::c_int as uint16_t,
    260 as ::core::ffi::c_int as uint16_t,
    132 as ::core::ffi::c_int as uint16_t,
    388 as ::core::ffi::c_int as uint16_t,
    68 as ::core::ffi::c_int as uint16_t,
    324 as ::core::ffi::c_int as uint16_t,
    196 as ::core::ffi::c_int as uint16_t,
    452 as ::core::ffi::c_int as uint16_t,
    36 as ::core::ffi::c_int as uint16_t,
    292 as ::core::ffi::c_int as uint16_t,
    164 as ::core::ffi::c_int as uint16_t,
    420 as ::core::ffi::c_int as uint16_t,
    100 as ::core::ffi::c_int as uint16_t,
    356 as ::core::ffi::c_int as uint16_t,
    228 as ::core::ffi::c_int as uint16_t,
    484 as ::core::ffi::c_int as uint16_t,
    20 as ::core::ffi::c_int as uint16_t,
    276 as ::core::ffi::c_int as uint16_t,
    148 as ::core::ffi::c_int as uint16_t,
    404 as ::core::ffi::c_int as uint16_t,
    84 as ::core::ffi::c_int as uint16_t,
    340 as ::core::ffi::c_int as uint16_t,
    212 as ::core::ffi::c_int as uint16_t,
    468 as ::core::ffi::c_int as uint16_t,
    52 as ::core::ffi::c_int as uint16_t,
    308 as ::core::ffi::c_int as uint16_t,
    180 as ::core::ffi::c_int as uint16_t,
    436 as ::core::ffi::c_int as uint16_t,
    116 as ::core::ffi::c_int as uint16_t,
    372 as ::core::ffi::c_int as uint16_t,
    244 as ::core::ffi::c_int as uint16_t,
    500 as ::core::ffi::c_int as uint16_t,
    12 as ::core::ffi::c_int as uint16_t,
    268 as ::core::ffi::c_int as uint16_t,
    140 as ::core::ffi::c_int as uint16_t,
    396 as ::core::ffi::c_int as uint16_t,
    76 as ::core::ffi::c_int as uint16_t,
    332 as ::core::ffi::c_int as uint16_t,
    204 as ::core::ffi::c_int as uint16_t,
    460 as ::core::ffi::c_int as uint16_t,
    44 as ::core::ffi::c_int as uint16_t,
    300 as ::core::ffi::c_int as uint16_t,
    172 as ::core::ffi::c_int as uint16_t,
    428 as ::core::ffi::c_int as uint16_t,
    108 as ::core::ffi::c_int as uint16_t,
    364 as ::core::ffi::c_int as uint16_t,
    236 as ::core::ffi::c_int as uint16_t,
    492 as ::core::ffi::c_int as uint16_t,
    28 as ::core::ffi::c_int as uint16_t,
    284 as ::core::ffi::c_int as uint16_t,
    156 as ::core::ffi::c_int as uint16_t,
    412 as ::core::ffi::c_int as uint16_t,
    92 as ::core::ffi::c_int as uint16_t,
    348 as ::core::ffi::c_int as uint16_t,
    220 as ::core::ffi::c_int as uint16_t,
    476 as ::core::ffi::c_int as uint16_t,
    60 as ::core::ffi::c_int as uint16_t,
    316 as ::core::ffi::c_int as uint16_t,
    188 as ::core::ffi::c_int as uint16_t,
    444 as ::core::ffi::c_int as uint16_t,
    124 as ::core::ffi::c_int as uint16_t,
    380 as ::core::ffi::c_int as uint16_t,
    252 as ::core::ffi::c_int as uint16_t,
    508 as ::core::ffi::c_int as uint16_t,
    2 as ::core::ffi::c_int as uint16_t,
    258 as ::core::ffi::c_int as uint16_t,
    130 as ::core::ffi::c_int as uint16_t,
    386 as ::core::ffi::c_int as uint16_t,
    66 as ::core::ffi::c_int as uint16_t,
    322 as ::core::ffi::c_int as uint16_t,
    194 as ::core::ffi::c_int as uint16_t,
    450 as ::core::ffi::c_int as uint16_t,
    34 as ::core::ffi::c_int as uint16_t,
    290 as ::core::ffi::c_int as uint16_t,
    162 as ::core::ffi::c_int as uint16_t,
    418 as ::core::ffi::c_int as uint16_t,
    98 as ::core::ffi::c_int as uint16_t,
    354 as ::core::ffi::c_int as uint16_t,
    226 as ::core::ffi::c_int as uint16_t,
    482 as ::core::ffi::c_int as uint16_t,
    18 as ::core::ffi::c_int as uint16_t,
    274 as ::core::ffi::c_int as uint16_t,
    146 as ::core::ffi::c_int as uint16_t,
    402 as ::core::ffi::c_int as uint16_t,
    82 as ::core::ffi::c_int as uint16_t,
    338 as ::core::ffi::c_int as uint16_t,
    210 as ::core::ffi::c_int as uint16_t,
    466 as ::core::ffi::c_int as uint16_t,
    50 as ::core::ffi::c_int as uint16_t,
    306 as ::core::ffi::c_int as uint16_t,
    178 as ::core::ffi::c_int as uint16_t,
    434 as ::core::ffi::c_int as uint16_t,
    114 as ::core::ffi::c_int as uint16_t,
    370 as ::core::ffi::c_int as uint16_t,
    242 as ::core::ffi::c_int as uint16_t,
    498 as ::core::ffi::c_int as uint16_t,
    10 as ::core::ffi::c_int as uint16_t,
    266 as ::core::ffi::c_int as uint16_t,
    138 as ::core::ffi::c_int as uint16_t,
    394 as ::core::ffi::c_int as uint16_t,
    74 as ::core::ffi::c_int as uint16_t,
    330 as ::core::ffi::c_int as uint16_t,
    202 as ::core::ffi::c_int as uint16_t,
    458 as ::core::ffi::c_int as uint16_t,
    42 as ::core::ffi::c_int as uint16_t,
    298 as ::core::ffi::c_int as uint16_t,
    170 as ::core::ffi::c_int as uint16_t,
    426 as ::core::ffi::c_int as uint16_t,
    106 as ::core::ffi::c_int as uint16_t,
    362 as ::core::ffi::c_int as uint16_t,
    234 as ::core::ffi::c_int as uint16_t,
    490 as ::core::ffi::c_int as uint16_t,
    26 as ::core::ffi::c_int as uint16_t,
    282 as ::core::ffi::c_int as uint16_t,
    154 as ::core::ffi::c_int as uint16_t,
    410 as ::core::ffi::c_int as uint16_t,
    90 as ::core::ffi::c_int as uint16_t,
    346 as ::core::ffi::c_int as uint16_t,
    218 as ::core::ffi::c_int as uint16_t,
    474 as ::core::ffi::c_int as uint16_t,
    58 as ::core::ffi::c_int as uint16_t,
    314 as ::core::ffi::c_int as uint16_t,
    186 as ::core::ffi::c_int as uint16_t,
    442 as ::core::ffi::c_int as uint16_t,
    122 as ::core::ffi::c_int as uint16_t,
    378 as ::core::ffi::c_int as uint16_t,
    250 as ::core::ffi::c_int as uint16_t,
    506 as ::core::ffi::c_int as uint16_t,
    6 as ::core::ffi::c_int as uint16_t,
    262 as ::core::ffi::c_int as uint16_t,
    134 as ::core::ffi::c_int as uint16_t,
    390 as ::core::ffi::c_int as uint16_t,
    70 as ::core::ffi::c_int as uint16_t,
    326 as ::core::ffi::c_int as uint16_t,
    198 as ::core::ffi::c_int as uint16_t,
    454 as ::core::ffi::c_int as uint16_t,
    38 as ::core::ffi::c_int as uint16_t,
    294 as ::core::ffi::c_int as uint16_t,
    166 as ::core::ffi::c_int as uint16_t,
    422 as ::core::ffi::c_int as uint16_t,
    102 as ::core::ffi::c_int as uint16_t,
    358 as ::core::ffi::c_int as uint16_t,
    230 as ::core::ffi::c_int as uint16_t,
    486 as ::core::ffi::c_int as uint16_t,
    22 as ::core::ffi::c_int as uint16_t,
    278 as ::core::ffi::c_int as uint16_t,
    150 as ::core::ffi::c_int as uint16_t,
    406 as ::core::ffi::c_int as uint16_t,
    86 as ::core::ffi::c_int as uint16_t,
    342 as ::core::ffi::c_int as uint16_t,
    214 as ::core::ffi::c_int as uint16_t,
    470 as ::core::ffi::c_int as uint16_t,
    54 as ::core::ffi::c_int as uint16_t,
    310 as ::core::ffi::c_int as uint16_t,
    182 as ::core::ffi::c_int as uint16_t,
    438 as ::core::ffi::c_int as uint16_t,
    118 as ::core::ffi::c_int as uint16_t,
    374 as ::core::ffi::c_int as uint16_t,
    246 as ::core::ffi::c_int as uint16_t,
    502 as ::core::ffi::c_int as uint16_t,
    14 as ::core::ffi::c_int as uint16_t,
    270 as ::core::ffi::c_int as uint16_t,
    142 as ::core::ffi::c_int as uint16_t,
    398 as ::core::ffi::c_int as uint16_t,
    78 as ::core::ffi::c_int as uint16_t,
    334 as ::core::ffi::c_int as uint16_t,
    206 as ::core::ffi::c_int as uint16_t,
    462 as ::core::ffi::c_int as uint16_t,
    46 as ::core::ffi::c_int as uint16_t,
    302 as ::core::ffi::c_int as uint16_t,
    174 as ::core::ffi::c_int as uint16_t,
    430 as ::core::ffi::c_int as uint16_t,
    110 as ::core::ffi::c_int as uint16_t,
    366 as ::core::ffi::c_int as uint16_t,
    238 as ::core::ffi::c_int as uint16_t,
    494 as ::core::ffi::c_int as uint16_t,
    30 as ::core::ffi::c_int as uint16_t,
    286 as ::core::ffi::c_int as uint16_t,
    158 as ::core::ffi::c_int as uint16_t,
    414 as ::core::ffi::c_int as uint16_t,
    94 as ::core::ffi::c_int as uint16_t,
    350 as ::core::ffi::c_int as uint16_t,
    222 as ::core::ffi::c_int as uint16_t,
    478 as ::core::ffi::c_int as uint16_t,
    62 as ::core::ffi::c_int as uint16_t,
    318 as ::core::ffi::c_int as uint16_t,
    190 as ::core::ffi::c_int as uint16_t,
    446 as ::core::ffi::c_int as uint16_t,
    126 as ::core::ffi::c_int as uint16_t,
    382 as ::core::ffi::c_int as uint16_t,
    254 as ::core::ffi::c_int as uint16_t,
    510 as ::core::ffi::c_int as uint16_t,
    1 as ::core::ffi::c_int as uint16_t,
    257 as ::core::ffi::c_int as uint16_t,
    129 as ::core::ffi::c_int as uint16_t,
    385 as ::core::ffi::c_int as uint16_t,
    65 as ::core::ffi::c_int as uint16_t,
    321 as ::core::ffi::c_int as uint16_t,
    193 as ::core::ffi::c_int as uint16_t,
    449 as ::core::ffi::c_int as uint16_t,
    33 as ::core::ffi::c_int as uint16_t,
    289 as ::core::ffi::c_int as uint16_t,
    161 as ::core::ffi::c_int as uint16_t,
    417 as ::core::ffi::c_int as uint16_t,
    97 as ::core::ffi::c_int as uint16_t,
    353 as ::core::ffi::c_int as uint16_t,
    225 as ::core::ffi::c_int as uint16_t,
    481 as ::core::ffi::c_int as uint16_t,
    17 as ::core::ffi::c_int as uint16_t,
    273 as ::core::ffi::c_int as uint16_t,
    145 as ::core::ffi::c_int as uint16_t,
    401 as ::core::ffi::c_int as uint16_t,
    81 as ::core::ffi::c_int as uint16_t,
    337 as ::core::ffi::c_int as uint16_t,
    209 as ::core::ffi::c_int as uint16_t,
    465 as ::core::ffi::c_int as uint16_t,
    49 as ::core::ffi::c_int as uint16_t,
    305 as ::core::ffi::c_int as uint16_t,
    177 as ::core::ffi::c_int as uint16_t,
    433 as ::core::ffi::c_int as uint16_t,
    113 as ::core::ffi::c_int as uint16_t,
    369 as ::core::ffi::c_int as uint16_t,
    241 as ::core::ffi::c_int as uint16_t,
    497 as ::core::ffi::c_int as uint16_t,
    9 as ::core::ffi::c_int as uint16_t,
    265 as ::core::ffi::c_int as uint16_t,
    137 as ::core::ffi::c_int as uint16_t,
    393 as ::core::ffi::c_int as uint16_t,
    73 as ::core::ffi::c_int as uint16_t,
    329 as ::core::ffi::c_int as uint16_t,
    201 as ::core::ffi::c_int as uint16_t,
    457 as ::core::ffi::c_int as uint16_t,
    41 as ::core::ffi::c_int as uint16_t,
    297 as ::core::ffi::c_int as uint16_t,
    169 as ::core::ffi::c_int as uint16_t,
    425 as ::core::ffi::c_int as uint16_t,
    105 as ::core::ffi::c_int as uint16_t,
    361 as ::core::ffi::c_int as uint16_t,
    233 as ::core::ffi::c_int as uint16_t,
    489 as ::core::ffi::c_int as uint16_t,
    25 as ::core::ffi::c_int as uint16_t,
    281 as ::core::ffi::c_int as uint16_t,
    153 as ::core::ffi::c_int as uint16_t,
    409 as ::core::ffi::c_int as uint16_t,
    89 as ::core::ffi::c_int as uint16_t,
    345 as ::core::ffi::c_int as uint16_t,
    217 as ::core::ffi::c_int as uint16_t,
    473 as ::core::ffi::c_int as uint16_t,
    57 as ::core::ffi::c_int as uint16_t,
    313 as ::core::ffi::c_int as uint16_t,
    185 as ::core::ffi::c_int as uint16_t,
    441 as ::core::ffi::c_int as uint16_t,
    121 as ::core::ffi::c_int as uint16_t,
    377 as ::core::ffi::c_int as uint16_t,
    249 as ::core::ffi::c_int as uint16_t,
    505 as ::core::ffi::c_int as uint16_t,
    5 as ::core::ffi::c_int as uint16_t,
    261 as ::core::ffi::c_int as uint16_t,
    133 as ::core::ffi::c_int as uint16_t,
    389 as ::core::ffi::c_int as uint16_t,
    69 as ::core::ffi::c_int as uint16_t,
    325 as ::core::ffi::c_int as uint16_t,
    197 as ::core::ffi::c_int as uint16_t,
    453 as ::core::ffi::c_int as uint16_t,
    37 as ::core::ffi::c_int as uint16_t,
    293 as ::core::ffi::c_int as uint16_t,
    165 as ::core::ffi::c_int as uint16_t,
    421 as ::core::ffi::c_int as uint16_t,
    101 as ::core::ffi::c_int as uint16_t,
    357 as ::core::ffi::c_int as uint16_t,
    229 as ::core::ffi::c_int as uint16_t,
    485 as ::core::ffi::c_int as uint16_t,
    21 as ::core::ffi::c_int as uint16_t,
    277 as ::core::ffi::c_int as uint16_t,
    149 as ::core::ffi::c_int as uint16_t,
    405 as ::core::ffi::c_int as uint16_t,
    85 as ::core::ffi::c_int as uint16_t,
    341 as ::core::ffi::c_int as uint16_t,
    213 as ::core::ffi::c_int as uint16_t,
    469 as ::core::ffi::c_int as uint16_t,
    53 as ::core::ffi::c_int as uint16_t,
    309 as ::core::ffi::c_int as uint16_t,
    181 as ::core::ffi::c_int as uint16_t,
    437 as ::core::ffi::c_int as uint16_t,
    117 as ::core::ffi::c_int as uint16_t,
    373 as ::core::ffi::c_int as uint16_t,
    245 as ::core::ffi::c_int as uint16_t,
    501 as ::core::ffi::c_int as uint16_t,
    13 as ::core::ffi::c_int as uint16_t,
    269 as ::core::ffi::c_int as uint16_t,
    141 as ::core::ffi::c_int as uint16_t,
    397 as ::core::ffi::c_int as uint16_t,
    77 as ::core::ffi::c_int as uint16_t,
    333 as ::core::ffi::c_int as uint16_t,
    205 as ::core::ffi::c_int as uint16_t,
    461 as ::core::ffi::c_int as uint16_t,
    45 as ::core::ffi::c_int as uint16_t,
    301 as ::core::ffi::c_int as uint16_t,
    173 as ::core::ffi::c_int as uint16_t,
    429 as ::core::ffi::c_int as uint16_t,
    109 as ::core::ffi::c_int as uint16_t,
    365 as ::core::ffi::c_int as uint16_t,
    237 as ::core::ffi::c_int as uint16_t,
    493 as ::core::ffi::c_int as uint16_t,
    29 as ::core::ffi::c_int as uint16_t,
    285 as ::core::ffi::c_int as uint16_t,
    157 as ::core::ffi::c_int as uint16_t,
    413 as ::core::ffi::c_int as uint16_t,
    93 as ::core::ffi::c_int as uint16_t,
    349 as ::core::ffi::c_int as uint16_t,
    221 as ::core::ffi::c_int as uint16_t,
    477 as ::core::ffi::c_int as uint16_t,
    61 as ::core::ffi::c_int as uint16_t,
    317 as ::core::ffi::c_int as uint16_t,
    189 as ::core::ffi::c_int as uint16_t,
    445 as ::core::ffi::c_int as uint16_t,
    125 as ::core::ffi::c_int as uint16_t,
    381 as ::core::ffi::c_int as uint16_t,
    253 as ::core::ffi::c_int as uint16_t,
    509 as ::core::ffi::c_int as uint16_t,
    3 as ::core::ffi::c_int as uint16_t,
    259 as ::core::ffi::c_int as uint16_t,
    131 as ::core::ffi::c_int as uint16_t,
    387 as ::core::ffi::c_int as uint16_t,
    67 as ::core::ffi::c_int as uint16_t,
    323 as ::core::ffi::c_int as uint16_t,
    195 as ::core::ffi::c_int as uint16_t,
    451 as ::core::ffi::c_int as uint16_t,
    35 as ::core::ffi::c_int as uint16_t,
    291 as ::core::ffi::c_int as uint16_t,
    163 as ::core::ffi::c_int as uint16_t,
    419 as ::core::ffi::c_int as uint16_t,
    99 as ::core::ffi::c_int as uint16_t,
    355 as ::core::ffi::c_int as uint16_t,
    227 as ::core::ffi::c_int as uint16_t,
    483 as ::core::ffi::c_int as uint16_t,
    19 as ::core::ffi::c_int as uint16_t,
    275 as ::core::ffi::c_int as uint16_t,
    147 as ::core::ffi::c_int as uint16_t,
    403 as ::core::ffi::c_int as uint16_t,
    83 as ::core::ffi::c_int as uint16_t,
    339 as ::core::ffi::c_int as uint16_t,
    211 as ::core::ffi::c_int as uint16_t,
    467 as ::core::ffi::c_int as uint16_t,
    51 as ::core::ffi::c_int as uint16_t,
    307 as ::core::ffi::c_int as uint16_t,
    179 as ::core::ffi::c_int as uint16_t,
    435 as ::core::ffi::c_int as uint16_t,
    115 as ::core::ffi::c_int as uint16_t,
    371 as ::core::ffi::c_int as uint16_t,
    243 as ::core::ffi::c_int as uint16_t,
    499 as ::core::ffi::c_int as uint16_t,
    11 as ::core::ffi::c_int as uint16_t,
    267 as ::core::ffi::c_int as uint16_t,
    139 as ::core::ffi::c_int as uint16_t,
    395 as ::core::ffi::c_int as uint16_t,
    75 as ::core::ffi::c_int as uint16_t,
    331 as ::core::ffi::c_int as uint16_t,
    203 as ::core::ffi::c_int as uint16_t,
    459 as ::core::ffi::c_int as uint16_t,
    43 as ::core::ffi::c_int as uint16_t,
    299 as ::core::ffi::c_int as uint16_t,
    171 as ::core::ffi::c_int as uint16_t,
    427 as ::core::ffi::c_int as uint16_t,
    107 as ::core::ffi::c_int as uint16_t,
    363 as ::core::ffi::c_int as uint16_t,
    235 as ::core::ffi::c_int as uint16_t,
    491 as ::core::ffi::c_int as uint16_t,
    27 as ::core::ffi::c_int as uint16_t,
    283 as ::core::ffi::c_int as uint16_t,
    155 as ::core::ffi::c_int as uint16_t,
    411 as ::core::ffi::c_int as uint16_t,
    91 as ::core::ffi::c_int as uint16_t,
    347 as ::core::ffi::c_int as uint16_t,
    219 as ::core::ffi::c_int as uint16_t,
    475 as ::core::ffi::c_int as uint16_t,
    59 as ::core::ffi::c_int as uint16_t,
    315 as ::core::ffi::c_int as uint16_t,
    187 as ::core::ffi::c_int as uint16_t,
    443 as ::core::ffi::c_int as uint16_t,
    123 as ::core::ffi::c_int as uint16_t,
    379 as ::core::ffi::c_int as uint16_t,
    251 as ::core::ffi::c_int as uint16_t,
    507 as ::core::ffi::c_int as uint16_t,
    7 as ::core::ffi::c_int as uint16_t,
    1031 as ::core::ffi::c_int as uint16_t,
    519 as ::core::ffi::c_int as uint16_t,
    1543 as ::core::ffi::c_int as uint16_t,
    263 as ::core::ffi::c_int as uint16_t,
    1287 as ::core::ffi::c_int as uint16_t,
    775 as ::core::ffi::c_int as uint16_t,
    1799 as ::core::ffi::c_int as uint16_t,
    135 as ::core::ffi::c_int as uint16_t,
    1159 as ::core::ffi::c_int as uint16_t,
    647 as ::core::ffi::c_int as uint16_t,
    1671 as ::core::ffi::c_int as uint16_t,
    391 as ::core::ffi::c_int as uint16_t,
    1415 as ::core::ffi::c_int as uint16_t,
    903 as ::core::ffi::c_int as uint16_t,
    1927 as ::core::ffi::c_int as uint16_t,
    71 as ::core::ffi::c_int as uint16_t,
    1095 as ::core::ffi::c_int as uint16_t,
    583 as ::core::ffi::c_int as uint16_t,
    1607 as ::core::ffi::c_int as uint16_t,
    327 as ::core::ffi::c_int as uint16_t,
    1351 as ::core::ffi::c_int as uint16_t,
    839 as ::core::ffi::c_int as uint16_t,
    1863 as ::core::ffi::c_int as uint16_t,
    199 as ::core::ffi::c_int as uint16_t,
    1223 as ::core::ffi::c_int as uint16_t,
    711 as ::core::ffi::c_int as uint16_t,
    1735 as ::core::ffi::c_int as uint16_t,
    455 as ::core::ffi::c_int as uint16_t,
    1479 as ::core::ffi::c_int as uint16_t,
    967 as ::core::ffi::c_int as uint16_t,
    1991 as ::core::ffi::c_int as uint16_t,
    39 as ::core::ffi::c_int as uint16_t,
    1063 as ::core::ffi::c_int as uint16_t,
    551 as ::core::ffi::c_int as uint16_t,
    1575 as ::core::ffi::c_int as uint16_t,
    295 as ::core::ffi::c_int as uint16_t,
    1319 as ::core::ffi::c_int as uint16_t,
    807 as ::core::ffi::c_int as uint16_t,
    1831 as ::core::ffi::c_int as uint16_t,
    167 as ::core::ffi::c_int as uint16_t,
    1191 as ::core::ffi::c_int as uint16_t,
    679 as ::core::ffi::c_int as uint16_t,
    1703 as ::core::ffi::c_int as uint16_t,
    423 as ::core::ffi::c_int as uint16_t,
    1447 as ::core::ffi::c_int as uint16_t,
    935 as ::core::ffi::c_int as uint16_t,
    1959 as ::core::ffi::c_int as uint16_t,
    103 as ::core::ffi::c_int as uint16_t,
    1127 as ::core::ffi::c_int as uint16_t,
    615 as ::core::ffi::c_int as uint16_t,
    1639 as ::core::ffi::c_int as uint16_t,
    359 as ::core::ffi::c_int as uint16_t,
    1383 as ::core::ffi::c_int as uint16_t,
    871 as ::core::ffi::c_int as uint16_t,
    1895 as ::core::ffi::c_int as uint16_t,
    231 as ::core::ffi::c_int as uint16_t,
    1255 as ::core::ffi::c_int as uint16_t,
    743 as ::core::ffi::c_int as uint16_t,
    1767 as ::core::ffi::c_int as uint16_t,
    487 as ::core::ffi::c_int as uint16_t,
    1511 as ::core::ffi::c_int as uint16_t,
    999 as ::core::ffi::c_int as uint16_t,
    2023 as ::core::ffi::c_int as uint16_t,
    23 as ::core::ffi::c_int as uint16_t,
    1047 as ::core::ffi::c_int as uint16_t,
    535 as ::core::ffi::c_int as uint16_t,
    1559 as ::core::ffi::c_int as uint16_t,
    279 as ::core::ffi::c_int as uint16_t,
    1303 as ::core::ffi::c_int as uint16_t,
    791 as ::core::ffi::c_int as uint16_t,
    1815 as ::core::ffi::c_int as uint16_t,
    151 as ::core::ffi::c_int as uint16_t,
    1175 as ::core::ffi::c_int as uint16_t,
    663 as ::core::ffi::c_int as uint16_t,
    1687 as ::core::ffi::c_int as uint16_t,
    407 as ::core::ffi::c_int as uint16_t,
    1431 as ::core::ffi::c_int as uint16_t,
    919 as ::core::ffi::c_int as uint16_t,
    1943 as ::core::ffi::c_int as uint16_t,
    87 as ::core::ffi::c_int as uint16_t,
    1111 as ::core::ffi::c_int as uint16_t,
    599 as ::core::ffi::c_int as uint16_t,
    1623 as ::core::ffi::c_int as uint16_t,
    343 as ::core::ffi::c_int as uint16_t,
    1367 as ::core::ffi::c_int as uint16_t,
    855 as ::core::ffi::c_int as uint16_t,
    1879 as ::core::ffi::c_int as uint16_t,
    215 as ::core::ffi::c_int as uint16_t,
    1239 as ::core::ffi::c_int as uint16_t,
    727 as ::core::ffi::c_int as uint16_t,
    1751 as ::core::ffi::c_int as uint16_t,
    471 as ::core::ffi::c_int as uint16_t,
    1495 as ::core::ffi::c_int as uint16_t,
    983 as ::core::ffi::c_int as uint16_t,
    2007 as ::core::ffi::c_int as uint16_t,
    55 as ::core::ffi::c_int as uint16_t,
    1079 as ::core::ffi::c_int as uint16_t,
    567 as ::core::ffi::c_int as uint16_t,
    1591 as ::core::ffi::c_int as uint16_t,
    311 as ::core::ffi::c_int as uint16_t,
    1335 as ::core::ffi::c_int as uint16_t,
    823 as ::core::ffi::c_int as uint16_t,
    1847 as ::core::ffi::c_int as uint16_t,
    183 as ::core::ffi::c_int as uint16_t,
    1207 as ::core::ffi::c_int as uint16_t,
    695 as ::core::ffi::c_int as uint16_t,
    1719 as ::core::ffi::c_int as uint16_t,
    439 as ::core::ffi::c_int as uint16_t,
    1463 as ::core::ffi::c_int as uint16_t,
    951 as ::core::ffi::c_int as uint16_t,
    1975 as ::core::ffi::c_int as uint16_t,
    119 as ::core::ffi::c_int as uint16_t,
    1143 as ::core::ffi::c_int as uint16_t,
    631 as ::core::ffi::c_int as uint16_t,
    1655 as ::core::ffi::c_int as uint16_t,
    375 as ::core::ffi::c_int as uint16_t,
    1399 as ::core::ffi::c_int as uint16_t,
    887 as ::core::ffi::c_int as uint16_t,
    1911 as ::core::ffi::c_int as uint16_t,
    247 as ::core::ffi::c_int as uint16_t,
    1271 as ::core::ffi::c_int as uint16_t,
    759 as ::core::ffi::c_int as uint16_t,
    1783 as ::core::ffi::c_int as uint16_t,
    503 as ::core::ffi::c_int as uint16_t,
    1527 as ::core::ffi::c_int as uint16_t,
    1015 as ::core::ffi::c_int as uint16_t,
    2039 as ::core::ffi::c_int as uint16_t,
    15 as ::core::ffi::c_int as uint16_t,
    1039 as ::core::ffi::c_int as uint16_t,
    527 as ::core::ffi::c_int as uint16_t,
    1551 as ::core::ffi::c_int as uint16_t,
    271 as ::core::ffi::c_int as uint16_t,
    1295 as ::core::ffi::c_int as uint16_t,
    783 as ::core::ffi::c_int as uint16_t,
    1807 as ::core::ffi::c_int as uint16_t,
    143 as ::core::ffi::c_int as uint16_t,
    1167 as ::core::ffi::c_int as uint16_t,
    655 as ::core::ffi::c_int as uint16_t,
    1679 as ::core::ffi::c_int as uint16_t,
    399 as ::core::ffi::c_int as uint16_t,
    1423 as ::core::ffi::c_int as uint16_t,
    911 as ::core::ffi::c_int as uint16_t,
    1935 as ::core::ffi::c_int as uint16_t,
    79 as ::core::ffi::c_int as uint16_t,
    1103 as ::core::ffi::c_int as uint16_t,
    591 as ::core::ffi::c_int as uint16_t,
    1615 as ::core::ffi::c_int as uint16_t,
    335 as ::core::ffi::c_int as uint16_t,
    1359 as ::core::ffi::c_int as uint16_t,
    847 as ::core::ffi::c_int as uint16_t,
    1871 as ::core::ffi::c_int as uint16_t,
    207 as ::core::ffi::c_int as uint16_t,
    1231 as ::core::ffi::c_int as uint16_t,
    719 as ::core::ffi::c_int as uint16_t,
    1743 as ::core::ffi::c_int as uint16_t,
    463 as ::core::ffi::c_int as uint16_t,
    1487 as ::core::ffi::c_int as uint16_t,
    975 as ::core::ffi::c_int as uint16_t,
    1999 as ::core::ffi::c_int as uint16_t,
    47 as ::core::ffi::c_int as uint16_t,
    1071 as ::core::ffi::c_int as uint16_t,
    559 as ::core::ffi::c_int as uint16_t,
    1583 as ::core::ffi::c_int as uint16_t,
    303 as ::core::ffi::c_int as uint16_t,
    1327 as ::core::ffi::c_int as uint16_t,
    815 as ::core::ffi::c_int as uint16_t,
    1839 as ::core::ffi::c_int as uint16_t,
    175 as ::core::ffi::c_int as uint16_t,
    1199 as ::core::ffi::c_int as uint16_t,
    687 as ::core::ffi::c_int as uint16_t,
    1711 as ::core::ffi::c_int as uint16_t,
    431 as ::core::ffi::c_int as uint16_t,
    1455 as ::core::ffi::c_int as uint16_t,
    943 as ::core::ffi::c_int as uint16_t,
    1967 as ::core::ffi::c_int as uint16_t,
    111 as ::core::ffi::c_int as uint16_t,
    1135 as ::core::ffi::c_int as uint16_t,
    623 as ::core::ffi::c_int as uint16_t,
    1647 as ::core::ffi::c_int as uint16_t,
    367 as ::core::ffi::c_int as uint16_t,
    1391 as ::core::ffi::c_int as uint16_t,
    879 as ::core::ffi::c_int as uint16_t,
    1903 as ::core::ffi::c_int as uint16_t,
    239 as ::core::ffi::c_int as uint16_t,
    1263 as ::core::ffi::c_int as uint16_t,
    751 as ::core::ffi::c_int as uint16_t,
    1775 as ::core::ffi::c_int as uint16_t,
    495 as ::core::ffi::c_int as uint16_t,
    1519 as ::core::ffi::c_int as uint16_t,
    1007 as ::core::ffi::c_int as uint16_t,
    2031 as ::core::ffi::c_int as uint16_t,
    31 as ::core::ffi::c_int as uint16_t,
    1055 as ::core::ffi::c_int as uint16_t,
    543 as ::core::ffi::c_int as uint16_t,
    1567 as ::core::ffi::c_int as uint16_t,
    287 as ::core::ffi::c_int as uint16_t,
    1311 as ::core::ffi::c_int as uint16_t,
    799 as ::core::ffi::c_int as uint16_t,
    1823 as ::core::ffi::c_int as uint16_t,
    159 as ::core::ffi::c_int as uint16_t,
    1183 as ::core::ffi::c_int as uint16_t,
    671 as ::core::ffi::c_int as uint16_t,
    1695 as ::core::ffi::c_int as uint16_t,
    415 as ::core::ffi::c_int as uint16_t,
    1439 as ::core::ffi::c_int as uint16_t,
    927 as ::core::ffi::c_int as uint16_t,
    1951 as ::core::ffi::c_int as uint16_t,
    95 as ::core::ffi::c_int as uint16_t,
    1119 as ::core::ffi::c_int as uint16_t,
    607 as ::core::ffi::c_int as uint16_t,
    1631 as ::core::ffi::c_int as uint16_t,
    351 as ::core::ffi::c_int as uint16_t,
    1375 as ::core::ffi::c_int as uint16_t,
    863 as ::core::ffi::c_int as uint16_t,
    1887 as ::core::ffi::c_int as uint16_t,
    223 as ::core::ffi::c_int as uint16_t,
    1247 as ::core::ffi::c_int as uint16_t,
    735 as ::core::ffi::c_int as uint16_t,
    1759 as ::core::ffi::c_int as uint16_t,
    479 as ::core::ffi::c_int as uint16_t,
    1503 as ::core::ffi::c_int as uint16_t,
    991 as ::core::ffi::c_int as uint16_t,
    2015 as ::core::ffi::c_int as uint16_t,
    63 as ::core::ffi::c_int as uint16_t,
    1087 as ::core::ffi::c_int as uint16_t,
    575 as ::core::ffi::c_int as uint16_t,
    1599 as ::core::ffi::c_int as uint16_t,
    319 as ::core::ffi::c_int as uint16_t,
    1343 as ::core::ffi::c_int as uint16_t,
    831 as ::core::ffi::c_int as uint16_t,
    1855 as ::core::ffi::c_int as uint16_t,
    191 as ::core::ffi::c_int as uint16_t,
    1215 as ::core::ffi::c_int as uint16_t,
    703 as ::core::ffi::c_int as uint16_t,
    1727 as ::core::ffi::c_int as uint16_t,
    447 as ::core::ffi::c_int as uint16_t,
    1471 as ::core::ffi::c_int as uint16_t,
    959 as ::core::ffi::c_int as uint16_t,
    1983 as ::core::ffi::c_int as uint16_t,
    127 as ::core::ffi::c_int as uint16_t,
    1151 as ::core::ffi::c_int as uint16_t,
    639 as ::core::ffi::c_int as uint16_t,
    1663 as ::core::ffi::c_int as uint16_t,
    383 as ::core::ffi::c_int as uint16_t,
    1407 as ::core::ffi::c_int as uint16_t,
    895 as ::core::ffi::c_int as uint16_t,
    1919 as ::core::ffi::c_int as uint16_t,
    255 as ::core::ffi::c_int as uint16_t,
    1279 as ::core::ffi::c_int as uint16_t,
    767 as ::core::ffi::c_int as uint16_t,
    1791 as ::core::ffi::c_int as uint16_t,
    511 as ::core::ffi::c_int as uint16_t,
    1535 as ::core::ffi::c_int as uint16_t,
    1023 as ::core::ffi::c_int as uint16_t,
    2047 as ::core::ffi::c_int as uint16_t,
];
#[inline(always)]
unsafe extern "C" fn StoreStaticCommandHuffmanTree(
    mut storage_ix: *mut size_t,
    mut storage: *mut uint8_t,
) {
    BrotliWriteBits(
        56 as size_t,
        (0x926244 as ::core::ffi::c_uint as uint64_t) << 32 as ::core::ffi::c_int
            | 0x16307003 as uint64_t,
        storage_ix,
        storage,
    );
    BrotliWriteBits(3 as size_t, 0 as uint64_t, storage_ix, storage);
}
static mut kStaticDistanceCodeBits: [uint16_t; 64] = [
    0 as ::core::ffi::c_int as uint16_t,
    32 as ::core::ffi::c_int as uint16_t,
    16 as ::core::ffi::c_int as uint16_t,
    48 as ::core::ffi::c_int as uint16_t,
    8 as ::core::ffi::c_int as uint16_t,
    40 as ::core::ffi::c_int as uint16_t,
    24 as ::core::ffi::c_int as uint16_t,
    56 as ::core::ffi::c_int as uint16_t,
    4 as ::core::ffi::c_int as uint16_t,
    36 as ::core::ffi::c_int as uint16_t,
    20 as ::core::ffi::c_int as uint16_t,
    52 as ::core::ffi::c_int as uint16_t,
    12 as ::core::ffi::c_int as uint16_t,
    44 as ::core::ffi::c_int as uint16_t,
    28 as ::core::ffi::c_int as uint16_t,
    60 as ::core::ffi::c_int as uint16_t,
    2 as ::core::ffi::c_int as uint16_t,
    34 as ::core::ffi::c_int as uint16_t,
    18 as ::core::ffi::c_int as uint16_t,
    50 as ::core::ffi::c_int as uint16_t,
    10 as ::core::ffi::c_int as uint16_t,
    42 as ::core::ffi::c_int as uint16_t,
    26 as ::core::ffi::c_int as uint16_t,
    58 as ::core::ffi::c_int as uint16_t,
    6 as ::core::ffi::c_int as uint16_t,
    38 as ::core::ffi::c_int as uint16_t,
    22 as ::core::ffi::c_int as uint16_t,
    54 as ::core::ffi::c_int as uint16_t,
    14 as ::core::ffi::c_int as uint16_t,
    46 as ::core::ffi::c_int as uint16_t,
    30 as ::core::ffi::c_int as uint16_t,
    62 as ::core::ffi::c_int as uint16_t,
    1 as ::core::ffi::c_int as uint16_t,
    33 as ::core::ffi::c_int as uint16_t,
    17 as ::core::ffi::c_int as uint16_t,
    49 as ::core::ffi::c_int as uint16_t,
    9 as ::core::ffi::c_int as uint16_t,
    41 as ::core::ffi::c_int as uint16_t,
    25 as ::core::ffi::c_int as uint16_t,
    57 as ::core::ffi::c_int as uint16_t,
    5 as ::core::ffi::c_int as uint16_t,
    37 as ::core::ffi::c_int as uint16_t,
    21 as ::core::ffi::c_int as uint16_t,
    53 as ::core::ffi::c_int as uint16_t,
    13 as ::core::ffi::c_int as uint16_t,
    45 as ::core::ffi::c_int as uint16_t,
    29 as ::core::ffi::c_int as uint16_t,
    61 as ::core::ffi::c_int as uint16_t,
    3 as ::core::ffi::c_int as uint16_t,
    35 as ::core::ffi::c_int as uint16_t,
    19 as ::core::ffi::c_int as uint16_t,
    51 as ::core::ffi::c_int as uint16_t,
    11 as ::core::ffi::c_int as uint16_t,
    43 as ::core::ffi::c_int as uint16_t,
    27 as ::core::ffi::c_int as uint16_t,
    59 as ::core::ffi::c_int as uint16_t,
    7 as ::core::ffi::c_int as uint16_t,
    39 as ::core::ffi::c_int as uint16_t,
    23 as ::core::ffi::c_int as uint16_t,
    55 as ::core::ffi::c_int as uint16_t,
    15 as ::core::ffi::c_int as uint16_t,
    47 as ::core::ffi::c_int as uint16_t,
    31 as ::core::ffi::c_int as uint16_t,
    63 as ::core::ffi::c_int as uint16_t,
];
#[inline(always)]
unsafe extern "C" fn StoreStaticDistanceHuffmanTree(
    mut storage_ix: *mut size_t,
    mut storage: *mut uint8_t,
) {
    BrotliWriteBits(28 as size_t, 0x369dc03 as uint64_t, storage_ix, storage);
}
unsafe extern "C" fn BuildAndStoreEntropyCodesLiteral(
    mut m: *mut MemoryManager,
    mut self_0: *mut BlockEncoder,
    mut histograms: *const HistogramLiteral,
    histograms_size: size_t,
    alphabet_size: size_t,
    mut tree: *mut HuffmanTree,
    mut storage_ix: *mut size_t,
    mut storage: *mut uint8_t,
) {
    let table_size: size_t = histograms_size.wrapping_mul((*self_0).histogram_length_);
    (*self_0).depths_ = if table_size > 0 as size_t {
        BrotliAllocate(
            m,
            table_size.wrapping_mul(::core::mem::size_of::<uint8_t>() as size_t),
        ) as *mut uint8_t
    } else {
        ::core::ptr::null_mut::<uint8_t>()
    };
    (*self_0).bits_ = if table_size > 0 as size_t {
        BrotliAllocate(
            m,
            table_size.wrapping_mul(::core::mem::size_of::<uint16_t>() as size_t),
        ) as *mut uint16_t
    } else {
        ::core::ptr::null_mut::<uint16_t>()
    };
    if 0 as ::core::ffi::c_int != 0 {
        return;
    }
    let mut i: size_t = 0;
    i = 0 as size_t;
    while i < histograms_size {
        let mut ix: size_t = i.wrapping_mul((*self_0).histogram_length_);
        BuildAndStoreHuffmanTree(
            (&raw const (*histograms.offset(i as isize)).data_ as *const uint32_t)
                .offset(0 as ::core::ffi::c_int as isize) as *const uint32_t,
            (*self_0).histogram_length_,
            alphabet_size,
            tree,
            (*self_0).depths_.offset(ix as isize) as *mut uint8_t,
            (*self_0).bits_.offset(ix as isize) as *mut uint16_t,
            storage_ix,
            storage,
        );
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn BuildAndStoreEntropyCodesCommand(
    mut m: *mut MemoryManager,
    mut self_0: *mut BlockEncoder,
    mut histograms: *const HistogramCommand,
    histograms_size: size_t,
    alphabet_size: size_t,
    mut tree: *mut HuffmanTree,
    mut storage_ix: *mut size_t,
    mut storage: *mut uint8_t,
) {
    let table_size: size_t = histograms_size.wrapping_mul((*self_0).histogram_length_);
    (*self_0).depths_ = if table_size > 0 as size_t {
        BrotliAllocate(
            m,
            table_size.wrapping_mul(::core::mem::size_of::<uint8_t>() as size_t),
        ) as *mut uint8_t
    } else {
        ::core::ptr::null_mut::<uint8_t>()
    };
    (*self_0).bits_ = if table_size > 0 as size_t {
        BrotliAllocate(
            m,
            table_size.wrapping_mul(::core::mem::size_of::<uint16_t>() as size_t),
        ) as *mut uint16_t
    } else {
        ::core::ptr::null_mut::<uint16_t>()
    };
    if 0 as ::core::ffi::c_int != 0 {
        return;
    }
    let mut i: size_t = 0;
    i = 0 as size_t;
    while i < histograms_size {
        let mut ix: size_t = i.wrapping_mul((*self_0).histogram_length_);
        BuildAndStoreHuffmanTree(
            (&raw const (*histograms.offset(i as isize)).data_ as *const uint32_t)
                .offset(0 as ::core::ffi::c_int as isize) as *const uint32_t,
            (*self_0).histogram_length_,
            alphabet_size,
            tree,
            (*self_0).depths_.offset(ix as isize) as *mut uint8_t,
            (*self_0).bits_.offset(ix as isize) as *mut uint16_t,
            storage_ix,
            storage,
        );
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn BuildAndStoreEntropyCodesDistance(
    mut m: *mut MemoryManager,
    mut self_0: *mut BlockEncoder,
    mut histograms: *const HistogramDistance,
    histograms_size: size_t,
    alphabet_size: size_t,
    mut tree: *mut HuffmanTree,
    mut storage_ix: *mut size_t,
    mut storage: *mut uint8_t,
) {
    let table_size: size_t = histograms_size.wrapping_mul((*self_0).histogram_length_);
    (*self_0).depths_ = if table_size > 0 as size_t {
        BrotliAllocate(
            m,
            table_size.wrapping_mul(::core::mem::size_of::<uint8_t>() as size_t),
        ) as *mut uint8_t
    } else {
        ::core::ptr::null_mut::<uint8_t>()
    };
    (*self_0).bits_ = if table_size > 0 as size_t {
        BrotliAllocate(
            m,
            table_size.wrapping_mul(::core::mem::size_of::<uint16_t>() as size_t),
        ) as *mut uint16_t
    } else {
        ::core::ptr::null_mut::<uint16_t>()
    };
    if 0 as ::core::ffi::c_int != 0 {
        return;
    }
    let mut i: size_t = 0;
    i = 0 as size_t;
    while i < histograms_size {
        let mut ix: size_t = i.wrapping_mul((*self_0).histogram_length_);
        BuildAndStoreHuffmanTree(
            (&raw const (*histograms.offset(i as isize)).data_ as *const uint32_t)
                .offset(0 as ::core::ffi::c_int as isize) as *const uint32_t,
            (*self_0).histogram_length_,
            alphabet_size,
            tree,
            (*self_0).depths_.offset(ix as isize) as *mut uint8_t,
            (*self_0).bits_.offset(ix as isize) as *mut uint16_t,
            storage_ix,
            storage,
        );
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn run_static_initializers() {
    kSymbolMask = ((1 as uint32_t) << SYMBOL_BITS).wrapping_sub(1 as uint32_t);
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
