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
    fn BrotliAllocate(m: *mut MemoryManager, n: size_t) -> *mut ::core::ffi::c_void;
    fn BrotliFree(m: *mut MemoryManager, p: *mut ::core::ffi::c_void);
    fn BrotliSplitBlock(
        m: *mut MemoryManager,
        cmds: *const Command,
        num_commands: size_t,
        data: *const uint8_t,
        offset: size_t,
        mask: size_t,
        params: *const BrotliEncoderParams,
        literal_split: *mut BlockSplit,
        insert_and_copy_split: *mut BlockSplit,
        dist_split: *mut BlockSplit,
    );
    fn BrotliBuildHistogramsWithContext(
        cmds: *const Command,
        num_commands: size_t,
        literal_split: *const BlockSplit,
        insert_and_copy_split: *const BlockSplit,
        dist_split: *const BlockSplit,
        ringbuffer: *const uint8_t,
        pos: size_t,
        mask: size_t,
        prev_byte: uint8_t,
        prev_byte2: uint8_t,
        context_modes: *const ContextType,
        literal_histograms: *mut HistogramLiteral,
        insert_and_copy_histograms: *mut HistogramCommand,
        copy_dist_histograms: *mut HistogramDistance,
    );
    fn BrotliBitsEntropy(population: *const uint32_t, size: size_t) -> ::core::ffi::c_double;
    fn BrotliPopulationCostDistance(histogram: *const HistogramDistance) -> ::core::ffi::c_double;
    fn BrotliClusterHistogramsLiteral(
        m: *mut MemoryManager,
        in_0: *const HistogramLiteral,
        in_size: size_t,
        max_histograms: size_t,
        out: *mut HistogramLiteral,
        out_size: *mut size_t,
        histogram_symbols: *mut uint32_t,
    );
    fn BrotliClusterHistogramsDistance(
        m: *mut MemoryManager,
        in_0: *const HistogramDistance,
        in_size: size_t,
        max_histograms: size_t,
        out: *mut HistogramDistance,
        out_size: *mut size_t,
        histogram_symbols: *mut uint32_t,
    );
    fn BrotliOptimizeHuffmanCountsForRle(
        length: size_t,
        counts: *mut uint32_t,
        good_for_rle: *mut uint8_t,
    );
}
pub type size_t = usize;
pub type __uint8_t = u8;
pub type __uint16_t = u16;
pub type __uint32_t = u32;
pub type __uint64_t = u64;
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
pub struct BrotliDistanceCodeLimit {
    pub max_alphabet_size: uint32_t,
    pub max_distance: uint32_t,
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
pub struct GreedyMetablockArena {
    pub lit_blocks: C2RustUnnamed,
    pub cmd_blocks: BlockSplitterCommand,
    pub dist_blocks: BlockSplitterDistance,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BlockSplitterDistance {
    pub alphabet_size_: size_t,
    pub min_block_size_: size_t,
    pub split_threshold_: ::core::ffi::c_double,
    pub num_blocks_: size_t,
    pub split_: *mut BlockSplit,
    pub histograms_: *mut HistogramDistance,
    pub histograms_size_: *mut size_t,
    pub combined_histo: [HistogramDistance; 2],
    pub target_block_size_: size_t,
    pub block_size_: size_t,
    pub curr_histogram_ix_: size_t,
    pub last_histogram_ix_: [size_t; 2],
    pub last_entropy_: [::core::ffi::c_double; 2],
    pub merge_last_count_: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BlockSplitterCommand {
    pub alphabet_size_: size_t,
    pub min_block_size_: size_t,
    pub split_threshold_: ::core::ffi::c_double,
    pub num_blocks_: size_t,
    pub split_: *mut BlockSplit,
    pub histograms_: *mut HistogramCommand,
    pub histograms_size_: *mut size_t,
    pub combined_histo: [HistogramCommand; 2],
    pub target_block_size_: size_t,
    pub block_size_: size_t,
    pub curr_histogram_ix_: size_t,
    pub last_histogram_ix_: [size_t; 2],
    pub last_entropy_: [::core::ffi::c_double; 2],
    pub merge_last_count_: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub plain: BlockSplitterLiteral,
    pub ctx: ContextBlockSplitter,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ContextBlockSplitter {
    pub alphabet_size_: size_t,
    pub num_contexts_: size_t,
    pub max_block_types_: size_t,
    pub min_block_size_: size_t,
    pub split_threshold_: ::core::ffi::c_double,
    pub num_blocks_: size_t,
    pub split_: *mut BlockSplit,
    pub histograms_: *mut HistogramLiteral,
    pub histograms_size_: *mut size_t,
    pub target_block_size_: size_t,
    pub block_size_: size_t,
    pub curr_histogram_ix_: size_t,
    pub last_histogram_ix_: [size_t; 2],
    pub last_entropy_: [::core::ffi::c_double; 26],
    pub merge_last_count_: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BlockSplitterLiteral {
    pub alphabet_size_: size_t,
    pub min_block_size_: size_t,
    pub split_threshold_: ::core::ffi::c_double,
    pub num_blocks_: size_t,
    pub split_: *mut BlockSplit,
    pub histograms_: *mut HistogramLiteral,
    pub histograms_size_: *mut size_t,
    pub combined_histo: [HistogramLiteral; 2],
    pub target_block_size_: size_t,
    pub block_size_: size_t,
    pub curr_histogram_ix_: size_t,
    pub last_histogram_ix_: [size_t; 2],
    pub last_entropy_: [::core::ffi::c_double; 2],
    pub merge_last_count_: size_t,
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const BROTLI_TRUE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const BROTLI_FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[inline(always)]
unsafe extern "C" fn brotli_min_size_t(mut a: size_t, mut b: size_t) -> size_t {
    return if a < b { a } else { b };
}
#[inline(always)]
unsafe extern "C" fn brotli_max_size_t(mut a: size_t, mut b: size_t) -> size_t {
    return if a > b { a } else { b };
}
pub const BROTLI_MAX_NUMBER_OF_BLOCK_TYPES: ::core::ffi::c_int = 256 as ::core::ffi::c_int;
pub const BROTLI_NUM_LITERAL_SYMBOLS: ::core::ffi::c_int = 256 as ::core::ffi::c_int;
pub const BROTLI_NUM_COMMAND_SYMBOLS: ::core::ffi::c_int = 704 as ::core::ffi::c_int;
pub const BROTLI_NUM_DISTANCE_SHORT_CODES: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const BROTLI_MAX_NPOSTFIX: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const BROTLI_MAX_DISTANCE_BITS: ::core::ffi::c_uint = 24 as ::core::ffi::c_uint;
pub const BROTLI_MAX_ALLOWED_DISTANCE: ::core::ffi::c_int = 0x7ffffffc as ::core::ffi::c_int;
pub const BROTLI_LITERAL_CONTEXT_BITS: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const BROTLI_DISTANCE_CONTEXT_BITS: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
#[inline(always)]
unsafe extern "C" fn BrotliCalculateDistanceCodeLimit(
    mut max_distance: uint32_t,
    mut npostfix: uint32_t,
    mut ndirect: uint32_t,
) -> BrotliDistanceCodeLimit {
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
            tmp = tmp >> 1 as ::core::ffi::c_int;
        }
        ndistbits = ndistbits.wrapping_sub(1);
        half = offset >> ndistbits & 1 as uint32_t;
        group = ndistbits.wrapping_sub(1 as uint32_t) << 1 as ::core::ffi::c_int | half;
        if group == 0 as uint32_t {
            result.max_alphabet_size =
                ndirect.wrapping_add(BROTLI_NUM_DISTANCE_SHORT_CODES as uint32_t);
            result.max_distance = ndirect;
            return result;
        }
        group = group.wrapping_sub(1);
        ndistbits = (group >> 1 as ::core::ffi::c_int).wrapping_add(1 as uint32_t);
        extra = ((1 as ::core::ffi::c_uint) << ndistbits).wrapping_sub(1 as ::core::ffi::c_uint)
            as uint32_t;
        start = ((1 as ::core::ffi::c_uint) << ndistbits.wrapping_add(1 as uint32_t))
            .wrapping_sub(4 as ::core::ffi::c_uint) as uint32_t;
        start = (start as ::core::ffi::c_uint)
            .wrapping_add(((group & 1 as uint32_t) << ndistbits) as ::core::ffi::c_uint)
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
}
#[inline(always)]
unsafe extern "C" fn Log2FloorNonZero(mut n: size_t) -> uint32_t {
    return 31 as uint32_t ^ (n as uint32_t).leading_zeros() as i32 as uint32_t;
}
#[inline(always)]
unsafe extern "C" fn PrefixEncodeCopyDistance(
    mut distance_code: size_t,
    mut num_direct_codes: size_t,
    mut postfix_bits: size_t,
    mut code: *mut uint16_t,
    mut extra_bits: *mut uint32_t,
) {
    if distance_code < (BROTLI_NUM_DISTANCE_SHORT_CODES as size_t).wrapping_add(num_direct_codes) {
        *code = distance_code as uint16_t;
        *extra_bits = 0 as uint32_t;
        return;
    } else {
        let mut dist: size_t = ((1 as ::core::ffi::c_int as size_t)
            << postfix_bits.wrapping_add(2 as size_t))
        .wrapping_add(
            distance_code
                .wrapping_sub(BROTLI_NUM_DISTANCE_SHORT_CODES as size_t)
                .wrapping_sub(num_direct_codes),
        );
        let mut bucket: size_t = Log2FloorNonZero(dist).wrapping_sub(1 as uint32_t) as size_t;
        let mut postfix_mask: size_t = ((1 as ::core::ffi::c_uint) << postfix_bits)
            .wrapping_sub(1 as ::core::ffi::c_uint)
            as size_t;
        let mut postfix: size_t = dist & postfix_mask;
        let mut prefix: size_t = dist >> bucket & 1 as size_t;
        let mut offset: size_t = (2 as size_t).wrapping_add(prefix) << bucket;
        let mut nbits: size_t = bucket.wrapping_sub(postfix_bits);
        *code = (nbits << 10 as ::core::ffi::c_int
            | (BROTLI_NUM_DISTANCE_SHORT_CODES as size_t)
                .wrapping_add(num_direct_codes)
                .wrapping_add(
                    (2 as size_t)
                        .wrapping_mul(nbits.wrapping_sub(1 as size_t))
                        .wrapping_add(prefix)
                        << postfix_bits,
                )
                .wrapping_add(postfix)) as uint16_t;
        *extra_bits = (dist.wrapping_sub(offset) >> postfix_bits) as uint32_t;
    };
}
#[inline(always)]
unsafe extern "C" fn CommandRestoreDistanceCode(
    mut self_0: *const Command,
    mut dist: *const BrotliDistanceParams,
) -> uint32_t {
    if ((*self_0).dist_prefix_ as uint32_t & 0x3ff as uint32_t)
        < (BROTLI_NUM_DISTANCE_SHORT_CODES as uint32_t)
            .wrapping_add((*dist).num_direct_distance_codes)
    {
        return (*self_0).dist_prefix_ as uint32_t & 0x3ff as uint32_t;
    } else {
        let mut dcode: uint32_t = (*self_0).dist_prefix_ as uint32_t & 0x3ff as uint32_t;
        let mut nbits: uint32_t =
            ((*self_0).dist_prefix_ as ::core::ffi::c_int >> 10 as ::core::ffi::c_int) as uint32_t;
        let mut extra: uint32_t = (*self_0).dist_extra_;
        let mut postfix_mask: uint32_t =
            ((1 as uint32_t) << (*dist).distance_postfix_bits).wrapping_sub(1 as uint32_t);
        let mut hcode: uint32_t = dcode
            .wrapping_sub((*dist).num_direct_distance_codes)
            .wrapping_sub(BROTLI_NUM_DISTANCE_SHORT_CODES as uint32_t)
            >> (*dist).distance_postfix_bits;
        let mut lcode: uint32_t = dcode
            .wrapping_sub((*dist).num_direct_distance_codes)
            .wrapping_sub(BROTLI_NUM_DISTANCE_SHORT_CODES as uint32_t)
            & postfix_mask;
        let mut offset: uint32_t = ((2 as uint32_t).wrapping_add(hcode & 1 as uint32_t) << nbits)
            .wrapping_sub(4 as uint32_t);
        return (offset.wrapping_add(extra) << (*dist).distance_postfix_bits)
            .wrapping_add(lcode)
            .wrapping_add((*dist).num_direct_distance_codes)
            .wrapping_add(BROTLI_NUM_DISTANCE_SHORT_CODES as uint32_t);
    };
}
#[inline(always)]
unsafe extern "C" fn CommandCopyLen(mut self_0: *const Command) -> uint32_t {
    return (*self_0).copy_len_ & 0x1ffffff as uint32_t;
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
unsafe extern "C" fn ClearHistogramsLiteral(mut array: *mut HistogramLiteral, mut length: size_t) {
    let mut i: size_t = 0;
    i = 0 as size_t;
    while i < length {
        HistogramClearLiteral(array.offset(i as isize));
        i = i.wrapping_add(1);
    }
}
#[inline(always)]
unsafe extern "C" fn HistogramAddLiteral(mut self_0: *mut HistogramLiteral, mut val: size_t) {
    (*self_0).data_[val as usize] = (*self_0).data_[val as usize].wrapping_add(1);
    (*self_0).total_count_ = (*self_0).total_count_.wrapping_add(1);
}
#[inline(always)]
unsafe extern "C" fn HistogramAddHistogramLiteral(
    mut self_0: *mut HistogramLiteral,
    mut v: *const HistogramLiteral,
) {
    let mut i: size_t = 0;
    (*self_0).total_count_ = ((*self_0).total_count_ as ::core::ffi::c_ulong)
        .wrapping_add((*v).total_count_ as ::core::ffi::c_ulong)
        as size_t as size_t;
    i = 0 as size_t;
    while i < DATA_SIZE as size_t {
        (*self_0).data_[i as usize] = ((*self_0).data_[i as usize] as ::core::ffi::c_uint)
            .wrapping_add((*v).data_[i as usize] as ::core::ffi::c_uint)
            as uint32_t as uint32_t;
        i = i.wrapping_add(1);
    }
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
unsafe extern "C" fn ClearHistogramsCommand(mut array: *mut HistogramCommand, mut length: size_t) {
    let mut i: size_t = 0;
    i = 0 as size_t;
    while i < length {
        HistogramClearCommand(array.offset(i as isize));
        i = i.wrapping_add(1);
    }
}
#[inline(always)]
unsafe extern "C" fn HistogramAddCommand(mut self_0: *mut HistogramCommand, mut val: size_t) {
    (*self_0).data_[val as usize] = (*self_0).data_[val as usize].wrapping_add(1);
    (*self_0).total_count_ = (*self_0).total_count_.wrapping_add(1);
}
#[inline(always)]
unsafe extern "C" fn HistogramAddHistogramCommand(
    mut self_0: *mut HistogramCommand,
    mut v: *const HistogramCommand,
) {
    let mut i: size_t = 0;
    (*self_0).total_count_ = ((*self_0).total_count_ as ::core::ffi::c_ulong)
        .wrapping_add((*v).total_count_ as ::core::ffi::c_ulong)
        as size_t as size_t;
    i = 0 as size_t;
    while i < DATA_SIZE_0 as size_t {
        (*self_0).data_[i as usize] = ((*self_0).data_[i as usize] as ::core::ffi::c_uint)
            .wrapping_add((*v).data_[i as usize] as ::core::ffi::c_uint)
            as uint32_t as uint32_t;
        i = i.wrapping_add(1);
    }
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
unsafe extern "C" fn ClearHistogramsDistance(
    mut array: *mut HistogramDistance,
    mut length: size_t,
) {
    let mut i: size_t = 0;
    i = 0 as size_t;
    while i < length {
        HistogramClearDistance(array.offset(i as isize));
        i = i.wrapping_add(1);
    }
}
#[inline(always)]
unsafe extern "C" fn HistogramAddDistance(mut self_0: *mut HistogramDistance, mut val: size_t) {
    (*self_0).data_[val as usize] = (*self_0).data_[val as usize].wrapping_add(1);
    (*self_0).total_count_ = (*self_0).total_count_.wrapping_add(1);
}
#[inline(always)]
unsafe extern "C" fn HistogramAddHistogramDistance(
    mut self_0: *mut HistogramDistance,
    mut v: *const HistogramDistance,
) {
    let mut i: size_t = 0;
    (*self_0).total_count_ = ((*self_0).total_count_ as ::core::ffi::c_ulong)
        .wrapping_add((*v).total_count_ as ::core::ffi::c_ulong)
        as size_t as size_t;
    i = 0 as size_t;
    while i < DATA_SIZE_1 as size_t {
        (*self_0).data_[i as usize] = ((*self_0).data_[i as usize] as ::core::ffi::c_uint)
            .wrapping_add((*v).data_[i as usize] as ::core::ffi::c_uint)
            as uint32_t as uint32_t;
        i = i.wrapping_add(1);
    }
}
pub const BROTLI_NUM_HISTOGRAM_DISTANCE_SYMBOLS: ::core::ffi::c_int = 544 as ::core::ffi::c_int;
pub const DATA_SIZE: ::core::ffi::c_int = BROTLI_NUM_LITERAL_SYMBOLS;
pub const DATA_SIZE_0: ::core::ffi::c_int = BROTLI_NUM_COMMAND_SYMBOLS;
pub const DATA_SIZE_1: ::core::ffi::c_int = BROTLI_NUM_HISTOGRAM_DISTANCE_SYMBOLS;
#[no_mangle]
pub unsafe extern "C" fn BrotliInitDistanceParams(
    mut dist_params: *mut BrotliDistanceParams,
    mut npostfix: uint32_t,
    mut ndirect: uint32_t,
    mut large_window: ::core::ffi::c_int,
) {
    let mut alphabet_size_max: uint32_t = 0;
    let mut alphabet_size_limit: uint32_t = 0;
    let mut max_distance: uint32_t = 0;
    (*dist_params).distance_postfix_bits = npostfix;
    (*dist_params).num_direct_distance_codes = ndirect;
    alphabet_size_max = (BROTLI_NUM_DISTANCE_SHORT_CODES as uint32_t)
        .wrapping_add(ndirect)
        .wrapping_add((24 as uint32_t) << npostfix.wrapping_add(1 as uint32_t));
    alphabet_size_limit = alphabet_size_max;
    max_distance = ndirect
        .wrapping_add(
            (1 as uint32_t)
                << (BROTLI_MAX_DISTANCE_BITS as uint32_t)
                    .wrapping_add(npostfix)
                    .wrapping_add(2 as uint32_t),
        )
        .wrapping_sub((1 as uint32_t) << npostfix.wrapping_add(2 as uint32_t));
    if large_window != 0 {
        let mut limit: BrotliDistanceCodeLimit = BrotliCalculateDistanceCodeLimit(
            BROTLI_MAX_ALLOWED_DISTANCE as uint32_t,
            npostfix,
            ndirect,
        );
        alphabet_size_max = (BROTLI_NUM_DISTANCE_SHORT_CODES as uint32_t)
            .wrapping_add(ndirect)
            .wrapping_add((62 as uint32_t) << npostfix.wrapping_add(1 as uint32_t));
        alphabet_size_limit = limit.max_alphabet_size;
        max_distance = limit.max_distance;
    }
    (*dist_params).alphabet_size_max = alphabet_size_max;
    (*dist_params).alphabet_size_limit = alphabet_size_limit;
    (*dist_params).max_distance = max_distance as size_t;
}
unsafe extern "C" fn RecomputeDistancePrefixes(
    mut cmds: *mut Command,
    mut num_commands: size_t,
    mut orig_params: *const BrotliDistanceParams,
    mut new_params: *const BrotliDistanceParams,
) {
    let mut i: size_t = 0;
    if (*orig_params).distance_postfix_bits == (*new_params).distance_postfix_bits
        && (*orig_params).num_direct_distance_codes == (*new_params).num_direct_distance_codes
    {
        return;
    }
    i = 0 as size_t;
    while i < num_commands {
        let mut cmd: *mut Command = cmds.offset(i as isize) as *mut Command;
        if CommandCopyLen(cmd) != 0
            && (*cmd).cmd_prefix_ as ::core::ffi::c_int >= 128 as ::core::ffi::c_int
        {
            PrefixEncodeCopyDistance(
                CommandRestoreDistanceCode(cmd, orig_params) as size_t,
                (*new_params).num_direct_distance_codes as size_t,
                (*new_params).distance_postfix_bits as size_t,
                &raw mut (*cmd).dist_prefix_,
                &raw mut (*cmd).dist_extra_,
            );
        }
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn ComputeDistanceCost(
    mut cmds: *const Command,
    mut num_commands: size_t,
    mut orig_params: *const BrotliDistanceParams,
    mut new_params: *const BrotliDistanceParams,
    mut cost: *mut ::core::ffi::c_double,
    mut tmp: *mut HistogramDistance,
) -> ::core::ffi::c_int {
    let mut i: size_t = 0;
    let mut equal_params: ::core::ffi::c_int = BROTLI_FALSE;
    let mut dist_prefix: uint16_t = 0;
    let mut dist_extra: uint32_t = 0;
    let mut extra_bits: ::core::ffi::c_double = 0.0f64;
    HistogramClearDistance(tmp);
    if (*orig_params).distance_postfix_bits == (*new_params).distance_postfix_bits
        && (*orig_params).num_direct_distance_codes == (*new_params).num_direct_distance_codes
    {
        equal_params = BROTLI_TRUE;
    }
    i = 0 as size_t;
    while i < num_commands {
        let mut cmd: *const Command = cmds.offset(i as isize) as *const Command;
        if CommandCopyLen(cmd) != 0
            && (*cmd).cmd_prefix_ as ::core::ffi::c_int >= 128 as ::core::ffi::c_int
        {
            if equal_params != 0 {
                dist_prefix = (*cmd).dist_prefix_;
            } else {
                let mut distance: uint32_t = CommandRestoreDistanceCode(cmd, orig_params);
                if distance as size_t > (*new_params).max_distance {
                    return BROTLI_FALSE;
                }
                PrefixEncodeCopyDistance(
                    distance as size_t,
                    (*new_params).num_direct_distance_codes as size_t,
                    (*new_params).distance_postfix_bits as size_t,
                    &raw mut dist_prefix,
                    &raw mut dist_extra,
                );
            }
            HistogramAddDistance(
                tmp,
                (dist_prefix as ::core::ffi::c_int & 0x3ff as ::core::ffi::c_int) as size_t,
            );
            extra_bits += (dist_prefix as ::core::ffi::c_int >> 10 as ::core::ffi::c_int)
                as ::core::ffi::c_double;
        }
        i = i.wrapping_add(1);
    }
    *cost = BrotliPopulationCostDistance(tmp) + extra_bits;
    return BROTLI_TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn BrotliBuildMetaBlock(
    mut m: *mut MemoryManager,
    mut ringbuffer: *const uint8_t,
    pos: size_t,
    mask: size_t,
    mut params: *mut BrotliEncoderParams,
    mut prev_byte: uint8_t,
    mut prev_byte2: uint8_t,
    mut cmds: *mut Command,
    mut num_commands: size_t,
    mut literal_context_mode: ContextType,
    mut mb: *mut MetaBlockSplit,
) {
    static mut kMaxNumberOfHistograms: size_t = 256 as size_t;
    let mut distance_histograms: *mut HistogramDistance =
        ::core::ptr::null_mut::<HistogramDistance>();
    let mut literal_histograms: *mut HistogramLiteral = ::core::ptr::null_mut::<HistogramLiteral>();
    let mut literal_context_modes: *mut ContextType = ::core::ptr::null_mut::<ContextType>();
    let mut literal_histograms_size: size_t = 0;
    let mut distance_histograms_size: size_t = 0;
    let mut i: size_t = 0;
    let mut literal_context_multiplier: size_t = 1 as size_t;
    let mut npostfix: uint32_t = 0;
    let mut ndirect_msb: uint32_t = 0 as uint32_t;
    let mut check_orig: ::core::ffi::c_int = BROTLI_TRUE;
    let mut best_dist_cost: ::core::ffi::c_double = 1e99f64;
    let mut orig_params: BrotliDistanceParams = (*params).dist;
    let mut new_params: BrotliDistanceParams = (*params).dist;
    let mut tmp: *mut HistogramDistance = if 1 as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
        BrotliAllocate(
            m,
            (1 as size_t).wrapping_mul(::core::mem::size_of::<HistogramDistance>() as size_t),
        ) as *mut HistogramDistance
    } else {
        ::core::ptr::null_mut::<HistogramDistance>()
    };
    if 0 as ::core::ffi::c_int != 0 || 0 as ::core::ffi::c_int != 0 {
        return;
    }
    npostfix = 0 as uint32_t;
    while npostfix <= BROTLI_MAX_NPOSTFIX as uint32_t {
        while ndirect_msb < 16 as uint32_t {
            let mut ndirect: uint32_t = ndirect_msb << npostfix;
            let mut skip: ::core::ffi::c_int = 0;
            let mut dist_cost: ::core::ffi::c_double = 0.;
            BrotliInitDistanceParams(
                &raw mut new_params,
                npostfix,
                ndirect,
                (*params).large_window,
            );
            if npostfix == orig_params.distance_postfix_bits
                && ndirect == orig_params.num_direct_distance_codes
            {
                check_orig = BROTLI_FALSE;
            }
            skip = (ComputeDistanceCost(
                cmds,
                num_commands,
                &raw mut orig_params,
                &raw mut new_params,
                &raw mut dist_cost,
                tmp,
            ) == 0) as ::core::ffi::c_int;
            if skip != 0 || dist_cost > best_dist_cost {
                break;
            }
            best_dist_cost = dist_cost;
            (*params).dist = new_params;
            ndirect_msb = ndirect_msb.wrapping_add(1);
        }
        if ndirect_msb > 0 as uint32_t {
            ndirect_msb = ndirect_msb.wrapping_sub(1);
        }
        ndirect_msb = (ndirect_msb as ::core::ffi::c_uint).wrapping_div(2 as ::core::ffi::c_uint)
            as uint32_t as uint32_t;
        npostfix = npostfix.wrapping_add(1);
    }
    if check_orig != 0 {
        let mut dist_cost_0: ::core::ffi::c_double = 0.;
        ComputeDistanceCost(
            cmds,
            num_commands,
            &raw mut orig_params,
            &raw mut orig_params,
            &raw mut dist_cost_0,
            tmp,
        );
        if dist_cost_0 < best_dist_cost {
            (*params).dist = orig_params;
        }
    }
    BrotliFree(m, tmp as *mut ::core::ffi::c_void);
    tmp = ::core::ptr::null_mut::<HistogramDistance>();
    RecomputeDistancePrefixes(
        cmds,
        num_commands,
        &raw mut orig_params,
        &raw mut (*params).dist,
    );
    BrotliSplitBlock(
        m,
        cmds,
        num_commands,
        ringbuffer,
        pos,
        mask,
        params,
        &raw mut (*mb).literal_split,
        &raw mut (*mb).command_split,
        &raw mut (*mb).distance_split,
    );
    if 0 as ::core::ffi::c_int != 0 {
        return;
    }
    if (*params).disable_literal_context_modeling == 0 {
        literal_context_multiplier =
            ((1 as ::core::ffi::c_int) << BROTLI_LITERAL_CONTEXT_BITS) as size_t;
        literal_context_modes = if (*mb).literal_split.num_types > 0 as size_t {
            BrotliAllocate(
                m,
                (*mb)
                    .literal_split
                    .num_types
                    .wrapping_mul(::core::mem::size_of::<ContextType>() as size_t),
            ) as *mut ContextType
        } else {
            ::core::ptr::null_mut::<ContextType>()
        };
        if 0 as ::core::ffi::c_int != 0 || 0 as ::core::ffi::c_int != 0 {
            return;
        }
        i = 0 as size_t;
        while i < (*mb).literal_split.num_types {
            *literal_context_modes.offset(i as isize) = literal_context_mode;
            i = i.wrapping_add(1);
        }
    }
    literal_histograms_size = (*mb)
        .literal_split
        .num_types
        .wrapping_mul(literal_context_multiplier);
    literal_histograms = if literal_histograms_size > 0 as size_t {
        BrotliAllocate(
            m,
            literal_histograms_size
                .wrapping_mul(::core::mem::size_of::<HistogramLiteral>() as size_t),
        ) as *mut HistogramLiteral
    } else {
        ::core::ptr::null_mut::<HistogramLiteral>()
    };
    if 0 as ::core::ffi::c_int != 0 || 0 as ::core::ffi::c_int != 0 {
        return;
    }
    ClearHistogramsLiteral(literal_histograms, literal_histograms_size);
    distance_histograms_size = (*mb).distance_split.num_types << BROTLI_DISTANCE_CONTEXT_BITS;
    distance_histograms = if distance_histograms_size > 0 as size_t {
        BrotliAllocate(
            m,
            distance_histograms_size
                .wrapping_mul(::core::mem::size_of::<HistogramDistance>() as size_t),
        ) as *mut HistogramDistance
    } else {
        ::core::ptr::null_mut::<HistogramDistance>()
    };
    if 0 as ::core::ffi::c_int != 0 || 0 as ::core::ffi::c_int != 0 {
        return;
    }
    ClearHistogramsDistance(distance_histograms, distance_histograms_size);
    (*mb).command_histograms_size = (*mb).command_split.num_types;
    (*mb).command_histograms = if (*mb).command_histograms_size > 0 as size_t {
        BrotliAllocate(
            m,
            (*mb)
                .command_histograms_size
                .wrapping_mul(::core::mem::size_of::<HistogramCommand>() as size_t),
        ) as *mut HistogramCommand
    } else {
        ::core::ptr::null_mut::<HistogramCommand>()
    };
    if 0 as ::core::ffi::c_int != 0 || 0 as ::core::ffi::c_int != 0 {
        return;
    }
    ClearHistogramsCommand((*mb).command_histograms, (*mb).command_histograms_size);
    BrotliBuildHistogramsWithContext(
        cmds,
        num_commands,
        &raw mut (*mb).literal_split,
        &raw mut (*mb).command_split,
        &raw mut (*mb).distance_split,
        ringbuffer,
        pos,
        mask,
        prev_byte,
        prev_byte2,
        literal_context_modes,
        literal_histograms,
        (*mb).command_histograms,
        distance_histograms,
    );
    BrotliFree(m, literal_context_modes as *mut ::core::ffi::c_void);
    literal_context_modes = ::core::ptr::null_mut::<ContextType>();
    (*mb).literal_context_map_size = (*mb).literal_split.num_types << BROTLI_LITERAL_CONTEXT_BITS;
    (*mb).literal_context_map = if (*mb).literal_context_map_size > 0 as size_t {
        BrotliAllocate(
            m,
            (*mb)
                .literal_context_map_size
                .wrapping_mul(::core::mem::size_of::<uint32_t>() as size_t),
        ) as *mut uint32_t
    } else {
        ::core::ptr::null_mut::<uint32_t>()
    };
    if 0 as ::core::ffi::c_int != 0 || 0 as ::core::ffi::c_int != 0 {
        return;
    }
    (*mb).literal_histograms_size = (*mb).literal_context_map_size;
    (*mb).literal_histograms = if (*mb).literal_histograms_size > 0 as size_t {
        BrotliAllocate(
            m,
            (*mb)
                .literal_histograms_size
                .wrapping_mul(::core::mem::size_of::<HistogramLiteral>() as size_t),
        ) as *mut HistogramLiteral
    } else {
        ::core::ptr::null_mut::<HistogramLiteral>()
    };
    if 0 as ::core::ffi::c_int != 0 || 0 as ::core::ffi::c_int != 0 {
        return;
    }
    BrotliClusterHistogramsLiteral(
        m,
        literal_histograms,
        literal_histograms_size,
        kMaxNumberOfHistograms,
        (*mb).literal_histograms,
        &raw mut (*mb).literal_histograms_size,
        (*mb).literal_context_map,
    );
    if 0 as ::core::ffi::c_int != 0 {
        return;
    }
    BrotliFree(m, literal_histograms as *mut ::core::ffi::c_void);
    literal_histograms = ::core::ptr::null_mut::<HistogramLiteral>();
    if (*params).disable_literal_context_modeling != 0 {
        i = (*mb).literal_split.num_types;
        while i != 0 as size_t {
            let mut j: size_t = 0 as size_t;
            i = i.wrapping_sub(1);
            while j < ((1 as ::core::ffi::c_int) << BROTLI_LITERAL_CONTEXT_BITS) as size_t {
                *(*mb)
                    .literal_context_map
                    .offset((i << BROTLI_LITERAL_CONTEXT_BITS).wrapping_add(j) as isize) =
                    *(*mb).literal_context_map.offset(i as isize);
                j = j.wrapping_add(1);
            }
        }
    }
    (*mb).distance_context_map_size =
        (*mb).distance_split.num_types << BROTLI_DISTANCE_CONTEXT_BITS;
    (*mb).distance_context_map = if (*mb).distance_context_map_size > 0 as size_t {
        BrotliAllocate(
            m,
            (*mb)
                .distance_context_map_size
                .wrapping_mul(::core::mem::size_of::<uint32_t>() as size_t),
        ) as *mut uint32_t
    } else {
        ::core::ptr::null_mut::<uint32_t>()
    };
    if 0 as ::core::ffi::c_int != 0 || 0 as ::core::ffi::c_int != 0 {
        return;
    }
    (*mb).distance_histograms_size = (*mb).distance_context_map_size;
    (*mb).distance_histograms = if (*mb).distance_histograms_size > 0 as size_t {
        BrotliAllocate(
            m,
            (*mb)
                .distance_histograms_size
                .wrapping_mul(::core::mem::size_of::<HistogramDistance>() as size_t),
        ) as *mut HistogramDistance
    } else {
        ::core::ptr::null_mut::<HistogramDistance>()
    };
    if 0 as ::core::ffi::c_int != 0 || 0 as ::core::ffi::c_int != 0 {
        return;
    }
    BrotliClusterHistogramsDistance(
        m,
        distance_histograms,
        (*mb).distance_context_map_size,
        kMaxNumberOfHistograms,
        (*mb).distance_histograms,
        &raw mut (*mb).distance_histograms_size,
        (*mb).distance_context_map,
    );
    if 0 as ::core::ffi::c_int != 0 {
        return;
    }
    BrotliFree(m, distance_histograms as *mut ::core::ffi::c_void);
    distance_histograms = ::core::ptr::null_mut::<HistogramDistance>();
}
unsafe extern "C" fn InitContextBlockSplitter(
    mut m: *mut MemoryManager,
    mut self_0: *mut ContextBlockSplitter,
    mut alphabet_size: size_t,
    mut num_contexts: size_t,
    mut min_block_size: size_t,
    mut split_threshold: ::core::ffi::c_double,
    mut num_symbols: size_t,
    mut split: *mut BlockSplit,
    mut histograms: *mut *mut HistogramLiteral,
    mut histograms_size: *mut size_t,
) {
    let mut max_num_blocks: size_t = num_symbols
        .wrapping_div(min_block_size)
        .wrapping_add(1 as size_t);
    let mut max_num_types: size_t = 0;
    (*self_0).alphabet_size_ = alphabet_size;
    (*self_0).num_contexts_ = num_contexts;
    (*self_0).max_block_types_ =
        (BROTLI_MAX_NUMBER_OF_BLOCK_TYPES as size_t).wrapping_div(num_contexts);
    (*self_0).min_block_size_ = min_block_size;
    (*self_0).split_threshold_ = split_threshold;
    (*self_0).num_blocks_ = 0 as size_t;
    (*self_0).split_ = split;
    (*self_0).histograms_size_ = histograms_size;
    (*self_0).target_block_size_ = min_block_size;
    (*self_0).block_size_ = 0 as size_t;
    (*self_0).curr_histogram_ix_ = 0 as size_t;
    (*self_0).merge_last_count_ = 0 as size_t;
    max_num_types = brotli_min_size_t(
        max_num_blocks,
        (*self_0).max_block_types_.wrapping_add(1 as size_t),
    );
    if (*split).types_alloc_size < max_num_blocks {
        let mut _new_size: size_t = if (*split).types_alloc_size == 0 as size_t {
            max_num_blocks
        } else {
            (*split).types_alloc_size
        };
        let mut new_array: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        while _new_size < max_num_blocks {
            _new_size = (_new_size as ::core::ffi::c_ulong).wrapping_mul(2 as ::core::ffi::c_ulong)
                as size_t as size_t;
        }
        new_array = if _new_size > 0 as size_t {
            BrotliAllocate(
                m,
                _new_size.wrapping_mul(::core::mem::size_of::<uint8_t>() as size_t),
            ) as *mut uint8_t
        } else {
            ::core::ptr::null_mut::<uint8_t>()
        };
        if 0 as ::core::ffi::c_int == 0
            && 0 as ::core::ffi::c_int == 0
            && (*split).types_alloc_size != 0 as size_t
        {
            memcpy(
                new_array as *mut ::core::ffi::c_void,
                (*split).types as *const ::core::ffi::c_void,
                (*split)
                    .types_alloc_size
                    .wrapping_mul(::core::mem::size_of::<uint8_t>() as size_t),
            );
        }
        BrotliFree(m, (*split).types as *mut ::core::ffi::c_void);
        (*split).types = ::core::ptr::null_mut::<uint8_t>();
        (*split).types = new_array;
        (*split).types_alloc_size = _new_size;
    }
    if (*split).lengths_alloc_size < max_num_blocks {
        let mut _new_size_0: size_t = if (*split).lengths_alloc_size == 0 as size_t {
            max_num_blocks
        } else {
            (*split).lengths_alloc_size
        };
        let mut new_array_0: *mut uint32_t = ::core::ptr::null_mut::<uint32_t>();
        while _new_size_0 < max_num_blocks {
            _new_size_0 = (_new_size_0 as ::core::ffi::c_ulong)
                .wrapping_mul(2 as ::core::ffi::c_ulong) as size_t
                as size_t;
        }
        new_array_0 = if _new_size_0 > 0 as size_t {
            BrotliAllocate(
                m,
                _new_size_0.wrapping_mul(::core::mem::size_of::<uint32_t>() as size_t),
            ) as *mut uint32_t
        } else {
            ::core::ptr::null_mut::<uint32_t>()
        };
        if 0 as ::core::ffi::c_int == 0
            && 0 as ::core::ffi::c_int == 0
            && (*split).lengths_alloc_size != 0 as size_t
        {
            memcpy(
                new_array_0 as *mut ::core::ffi::c_void,
                (*split).lengths as *const ::core::ffi::c_void,
                (*split)
                    .lengths_alloc_size
                    .wrapping_mul(::core::mem::size_of::<uint32_t>() as size_t),
            );
        }
        BrotliFree(m, (*split).lengths as *mut ::core::ffi::c_void);
        (*split).lengths = ::core::ptr::null_mut::<uint32_t>();
        (*split).lengths = new_array_0;
        (*split).lengths_alloc_size = _new_size_0;
    }
    if 0 as ::core::ffi::c_int != 0 {
        return;
    }
    (*split).num_blocks = max_num_blocks;
    if 0 as ::core::ffi::c_int != 0 {
        return;
    }
    *histograms_size = max_num_types.wrapping_mul(num_contexts);
    *histograms = if *histograms_size > 0 as size_t {
        BrotliAllocate(
            m,
            (*histograms_size).wrapping_mul(::core::mem::size_of::<HistogramLiteral>() as size_t),
        ) as *mut HistogramLiteral
    } else {
        ::core::ptr::null_mut::<HistogramLiteral>()
    };
    (*self_0).histograms_ = *histograms;
    if 0 as ::core::ffi::c_int != 0 || 0 as ::core::ffi::c_int != 0 {
        return;
    }
    ClearHistogramsLiteral(
        (*self_0)
            .histograms_
            .offset(0 as ::core::ffi::c_int as isize) as *mut HistogramLiteral,
        num_contexts,
    );
    (*self_0).last_histogram_ix_[1 as ::core::ffi::c_int as usize] = 0 as size_t;
    (*self_0).last_histogram_ix_[0 as ::core::ffi::c_int as usize] =
        (*self_0).last_histogram_ix_[1 as ::core::ffi::c_int as usize];
}
unsafe extern "C" fn ContextBlockSplitterFinishBlock(
    mut self_0: *mut ContextBlockSplitter,
    mut m: *mut MemoryManager,
    mut is_final: ::core::ffi::c_int,
) {
    let mut split: *mut BlockSplit = (*self_0).split_;
    let num_contexts: size_t = (*self_0).num_contexts_;
    let mut last_entropy: *mut ::core::ffi::c_double =
        &raw mut (*self_0).last_entropy_ as *mut ::core::ffi::c_double;
    let mut histograms: *mut HistogramLiteral = (*self_0).histograms_;
    if (*self_0).block_size_ < (*self_0).min_block_size_ {
        (*self_0).block_size_ = (*self_0).min_block_size_;
    }
    if (*self_0).num_blocks_ == 0 as size_t {
        let mut i: size_t = 0;
        *(*split).lengths.offset(0 as ::core::ffi::c_int as isize) =
            (*self_0).block_size_ as uint32_t;
        *(*split).types.offset(0 as ::core::ffi::c_int as isize) = 0 as uint8_t;
        i = 0 as size_t;
        while i < num_contexts {
            *last_entropy.offset(i as isize) = BrotliBitsEntropy(
                &raw mut (*histograms.offset(i as isize)).data_ as *mut uint32_t,
                (*self_0).alphabet_size_,
            );
            *last_entropy.offset(num_contexts.wrapping_add(i) as isize) =
                *last_entropy.offset(i as isize);
            i = i.wrapping_add(1);
        }
        (*self_0).num_blocks_ = (*self_0).num_blocks_.wrapping_add(1);
        (*split).num_types = (*split).num_types.wrapping_add(1);
        (*self_0).curr_histogram_ix_ = ((*self_0).curr_histogram_ix_ as ::core::ffi::c_ulong)
            .wrapping_add(num_contexts as ::core::ffi::c_ulong)
            as size_t as size_t;
        if (*self_0).curr_histogram_ix_ < *(*self_0).histograms_size_ {
            ClearHistogramsLiteral(
                (*self_0)
                    .histograms_
                    .offset((*self_0).curr_histogram_ix_ as isize)
                    as *mut HistogramLiteral,
                (*self_0).num_contexts_,
            );
        }
        (*self_0).block_size_ = 0 as size_t;
    } else if (*self_0).block_size_ > 0 as size_t {
        let mut entropy: [::core::ffi::c_double; 13] = [0.; 13];
        let mut combined_histo: *mut HistogramLiteral =
            if (2 as size_t).wrapping_mul(num_contexts) > 0 as size_t {
                BrotliAllocate(
                    m,
                    (2 as size_t)
                        .wrapping_mul(num_contexts)
                        .wrapping_mul(::core::mem::size_of::<HistogramLiteral>() as size_t),
                ) as *mut HistogramLiteral
            } else {
                ::core::ptr::null_mut::<HistogramLiteral>()
            };
        let mut combined_entropy: [::core::ffi::c_double; 26] = [0.; 26];
        let mut diff: [::core::ffi::c_double; 2] = [0.0f64, 0.];
        let mut i_0: size_t = 0;
        if 0 as ::core::ffi::c_int != 0 || 0 as ::core::ffi::c_int != 0 {
            return;
        }
        i_0 = 0 as size_t;
        while i_0 < num_contexts {
            let mut curr_histo_ix: size_t = (*self_0).curr_histogram_ix_.wrapping_add(i_0);
            let mut j: size_t = 0;
            entropy[i_0 as usize] = BrotliBitsEntropy(
                &raw mut (*histograms.offset(curr_histo_ix as isize)).data_ as *mut uint32_t,
                (*self_0).alphabet_size_,
            );
            j = 0 as size_t;
            while j < 2 as size_t {
                let mut jx: size_t = j.wrapping_mul(num_contexts).wrapping_add(i_0);
                let mut last_histogram_ix: size_t =
                    (*self_0).last_histogram_ix_[j as usize].wrapping_add(i_0);
                *combined_histo.offset(jx as isize) = *histograms.offset(curr_histo_ix as isize);
                HistogramAddHistogramLiteral(
                    combined_histo.offset(jx as isize) as *mut HistogramLiteral,
                    histograms.offset(last_histogram_ix as isize) as *mut HistogramLiteral,
                );
                combined_entropy[jx as usize] = BrotliBitsEntropy(
                    (&raw mut (*combined_histo.offset(jx as isize)).data_ as *mut uint32_t)
                        .offset(0 as ::core::ffi::c_int as isize)
                        as *mut uint32_t,
                    (*self_0).alphabet_size_,
                );
                diff[j as usize] += combined_entropy[jx as usize]
                    - entropy[i_0 as usize]
                    - *last_entropy.offset(jx as isize);
                j = j.wrapping_add(1);
            }
            i_0 = i_0.wrapping_add(1);
        }
        if (*split).num_types < (*self_0).max_block_types_
            && diff[0 as ::core::ffi::c_int as usize] > (*self_0).split_threshold_
            && diff[1 as ::core::ffi::c_int as usize] > (*self_0).split_threshold_
        {
            *(*split).lengths.offset((*self_0).num_blocks_ as isize) =
                (*self_0).block_size_ as uint32_t;
            *(*split).types.offset((*self_0).num_blocks_ as isize) = (*split).num_types as uint8_t;
            (*self_0).last_histogram_ix_[1 as ::core::ffi::c_int as usize] =
                (*self_0).last_histogram_ix_[0 as ::core::ffi::c_int as usize];
            (*self_0).last_histogram_ix_[0 as ::core::ffi::c_int as usize] =
                (*split).num_types.wrapping_mul(num_contexts);
            i_0 = 0 as size_t;
            while i_0 < num_contexts {
                *last_entropy.offset(num_contexts.wrapping_add(i_0) as isize) =
                    *last_entropy.offset(i_0 as isize);
                *last_entropy.offset(i_0 as isize) = entropy[i_0 as usize];
                i_0 = i_0.wrapping_add(1);
            }
            (*self_0).num_blocks_ = (*self_0).num_blocks_.wrapping_add(1);
            (*split).num_types = (*split).num_types.wrapping_add(1);
            (*self_0).curr_histogram_ix_ = ((*self_0).curr_histogram_ix_ as ::core::ffi::c_ulong)
                .wrapping_add(num_contexts as ::core::ffi::c_ulong)
                as size_t as size_t;
            if (*self_0).curr_histogram_ix_ < *(*self_0).histograms_size_ {
                ClearHistogramsLiteral(
                    (*self_0)
                        .histograms_
                        .offset((*self_0).curr_histogram_ix_ as isize)
                        as *mut HistogramLiteral,
                    (*self_0).num_contexts_,
                );
            }
            (*self_0).block_size_ = 0 as size_t;
            (*self_0).merge_last_count_ = 0 as size_t;
            (*self_0).target_block_size_ = (*self_0).min_block_size_;
        } else if diff[1 as ::core::ffi::c_int as usize]
            < diff[0 as ::core::ffi::c_int as usize] - 20.0f64
        {
            *(*split).lengths.offset((*self_0).num_blocks_ as isize) =
                (*self_0).block_size_ as uint32_t;
            *(*split).types.offset((*self_0).num_blocks_ as isize) = *(*split)
                .types
                .offset((*self_0).num_blocks_.wrapping_sub(2 as size_t) as isize);
            let mut __brotli_swap_tmp: size_t =
                (*self_0).last_histogram_ix_[0 as ::core::ffi::c_int as usize];
            (*self_0).last_histogram_ix_[0 as ::core::ffi::c_int as usize] =
                (*self_0).last_histogram_ix_[1 as ::core::ffi::c_int as usize];
            (*self_0).last_histogram_ix_[1 as ::core::ffi::c_int as usize] = __brotli_swap_tmp;
            i_0 = 0 as size_t;
            while i_0 < num_contexts {
                *histograms.offset(
                    (*self_0).last_histogram_ix_[0 as ::core::ffi::c_int as usize].wrapping_add(i_0)
                        as isize,
                ) = *combined_histo.offset(num_contexts.wrapping_add(i_0) as isize);
                *last_entropy.offset(num_contexts.wrapping_add(i_0) as isize) =
                    *last_entropy.offset(i_0 as isize);
                *last_entropy.offset(i_0 as isize) =
                    combined_entropy[num_contexts.wrapping_add(i_0) as usize];
                HistogramClearLiteral(
                    histograms.offset((*self_0).curr_histogram_ix_.wrapping_add(i_0) as isize)
                        as *mut HistogramLiteral,
                );
                i_0 = i_0.wrapping_add(1);
            }
            (*self_0).num_blocks_ = (*self_0).num_blocks_.wrapping_add(1);
            (*self_0).block_size_ = 0 as size_t;
            (*self_0).merge_last_count_ = 0 as size_t;
            (*self_0).target_block_size_ = (*self_0).min_block_size_;
        } else {
            let ref mut fresh2 = *(*split)
                .lengths
                .offset((*self_0).num_blocks_.wrapping_sub(1 as size_t) as isize);
            *fresh2 = (*fresh2 as ::core::ffi::c_uint)
                .wrapping_add((*self_0).block_size_ as uint32_t as ::core::ffi::c_uint)
                as uint32_t as uint32_t;
            i_0 = 0 as size_t;
            while i_0 < num_contexts {
                *histograms.offset(
                    (*self_0).last_histogram_ix_[0 as ::core::ffi::c_int as usize].wrapping_add(i_0)
                        as isize,
                ) = *combined_histo.offset(i_0 as isize);
                *last_entropy.offset(i_0 as isize) = combined_entropy[i_0 as usize];
                if (*split).num_types == 1 as size_t {
                    *last_entropy.offset(num_contexts.wrapping_add(i_0) as isize) =
                        *last_entropy.offset(i_0 as isize);
                }
                HistogramClearLiteral(
                    histograms.offset((*self_0).curr_histogram_ix_.wrapping_add(i_0) as isize)
                        as *mut HistogramLiteral,
                );
                i_0 = i_0.wrapping_add(1);
            }
            (*self_0).block_size_ = 0 as size_t;
            (*self_0).merge_last_count_ = (*self_0).merge_last_count_.wrapping_add(1);
            if (*self_0).merge_last_count_ > 1 as size_t {
                (*self_0).target_block_size_ = ((*self_0).target_block_size_
                    as ::core::ffi::c_ulong)
                    .wrapping_add((*self_0).min_block_size_ as ::core::ffi::c_ulong)
                    as size_t as size_t;
            }
        }
        BrotliFree(m, combined_histo as *mut ::core::ffi::c_void);
        combined_histo = ::core::ptr::null_mut::<HistogramLiteral>();
    }
    if is_final != 0 {
        *(*self_0).histograms_size_ = (*split).num_types.wrapping_mul(num_contexts);
        (*split).num_blocks = (*self_0).num_blocks_;
    }
}
unsafe extern "C" fn ContextBlockSplitterAddSymbol(
    mut self_0: *mut ContextBlockSplitter,
    mut m: *mut MemoryManager,
    mut symbol: size_t,
    mut context: size_t,
) {
    HistogramAddLiteral(
        (*self_0)
            .histograms_
            .offset((*self_0).curr_histogram_ix_.wrapping_add(context) as isize)
            as *mut HistogramLiteral,
        symbol,
    );
    (*self_0).block_size_ = (*self_0).block_size_.wrapping_add(1);
    if (*self_0).block_size_ == (*self_0).target_block_size_ {
        ContextBlockSplitterFinishBlock(self_0, m, BROTLI_FALSE);
        if 0 as ::core::ffi::c_int != 0 {
            return;
        }
    }
}
unsafe extern "C" fn MapStaticContexts(
    mut m: *mut MemoryManager,
    mut num_contexts: size_t,
    mut static_context_map: *const uint32_t,
    mut mb: *mut MetaBlockSplit,
) {
    let mut i: size_t = 0;
    (*mb).literal_context_map_size = (*mb).literal_split.num_types << BROTLI_LITERAL_CONTEXT_BITS;
    (*mb).literal_context_map = if (*mb).literal_context_map_size > 0 as size_t {
        BrotliAllocate(
            m,
            (*mb)
                .literal_context_map_size
                .wrapping_mul(::core::mem::size_of::<uint32_t>() as size_t),
        ) as *mut uint32_t
    } else {
        ::core::ptr::null_mut::<uint32_t>()
    };
    if 0 as ::core::ffi::c_int != 0 || 0 as ::core::ffi::c_int != 0 {
        return;
    }
    i = 0 as size_t;
    while i < (*mb).literal_split.num_types {
        let mut offset: uint32_t = i.wrapping_mul(num_contexts) as uint32_t;
        let mut j: size_t = 0;
        j = 0 as size_t;
        while j < ((1 as ::core::ffi::c_uint) << BROTLI_LITERAL_CONTEXT_BITS) as size_t {
            *(*mb)
                .literal_context_map
                .offset((i << BROTLI_LITERAL_CONTEXT_BITS).wrapping_add(j) as isize) =
                offset.wrapping_add(*static_context_map.offset(j as isize));
            j = j.wrapping_add(1);
        }
        i = i.wrapping_add(1);
    }
}
#[inline(always)]
unsafe extern "C" fn BrotliBuildMetaBlockGreedyInternal(
    mut m: *mut MemoryManager,
    mut arena: *mut GreedyMetablockArena,
    mut ringbuffer: *const uint8_t,
    mut pos: size_t,
    mut mask: size_t,
    mut prev_byte: uint8_t,
    mut prev_byte2: uint8_t,
    mut literal_context_lut: ContextLut,
    num_contexts: size_t,
    mut static_context_map: *const uint32_t,
    mut commands: *const Command,
    mut n_commands: size_t,
    mut mb: *mut MetaBlockSplit,
) {
    let mut num_literals: size_t = 0 as size_t;
    let mut i: size_t = 0;
    i = 0 as size_t;
    while i < n_commands {
        num_literals = (num_literals as ::core::ffi::c_ulong)
            .wrapping_add((*commands.offset(i as isize)).insert_len_ as ::core::ffi::c_ulong)
            as size_t as size_t;
        i = i.wrapping_add(1);
    }
    if num_contexts == 1 as size_t {
        InitBlockSplitterLiteral(
            m,
            &raw mut (*arena).lit_blocks.plain,
            256 as size_t,
            512 as size_t,
            400.0f64,
            num_literals,
            &raw mut (*mb).literal_split,
            &raw mut (*mb).literal_histograms,
            &raw mut (*mb).literal_histograms_size,
        );
    } else {
        InitContextBlockSplitter(
            m,
            &raw mut (*arena).lit_blocks.ctx,
            256 as size_t,
            num_contexts,
            512 as size_t,
            400.0f64,
            num_literals,
            &raw mut (*mb).literal_split,
            &raw mut (*mb).literal_histograms,
            &raw mut (*mb).literal_histograms_size,
        );
    }
    if 0 as ::core::ffi::c_int != 0 {
        return;
    }
    InitBlockSplitterCommand(
        m,
        &raw mut (*arena).cmd_blocks,
        BROTLI_NUM_COMMAND_SYMBOLS as size_t,
        1024 as size_t,
        500.0f64,
        n_commands,
        &raw mut (*mb).command_split,
        &raw mut (*mb).command_histograms,
        &raw mut (*mb).command_histograms_size,
    );
    if 0 as ::core::ffi::c_int != 0 {
        return;
    }
    InitBlockSplitterDistance(
        m,
        &raw mut (*arena).dist_blocks,
        64 as size_t,
        512 as size_t,
        100.0f64,
        n_commands,
        &raw mut (*mb).distance_split,
        &raw mut (*mb).distance_histograms,
        &raw mut (*mb).distance_histograms_size,
    );
    if 0 as ::core::ffi::c_int != 0 {
        return;
    }
    i = 0 as size_t;
    while i < n_commands {
        let cmd: Command = *commands.offset(i as isize);
        let mut j: size_t = 0;
        BlockSplitterAddSymbolCommand(&raw mut (*arena).cmd_blocks, cmd.cmd_prefix_ as size_t);
        j = cmd.insert_len_ as size_t;
        while j != 0 as size_t {
            let mut literal: uint8_t = *ringbuffer.offset((pos & mask) as isize);
            if num_contexts == 1 as size_t {
                BlockSplitterAddSymbolLiteral(
                    &raw mut (*arena).lit_blocks.plain,
                    literal as size_t,
                );
            } else {
                let mut context: size_t = (*literal_context_lut.offset(prev_byte as isize)
                    as ::core::ffi::c_int
                    | *literal_context_lut
                        .offset(256 as ::core::ffi::c_int as isize)
                        .offset(prev_byte2 as isize) as ::core::ffi::c_int)
                    as size_t;
                ContextBlockSplitterAddSymbol(
                    &raw mut (*arena).lit_blocks.ctx,
                    m,
                    literal as size_t,
                    *static_context_map.offset(context as isize) as size_t,
                );
                if 0 as ::core::ffi::c_int != 0 {
                    return;
                }
            }
            prev_byte2 = prev_byte;
            prev_byte = literal;
            pos = pos.wrapping_add(1);
            j = j.wrapping_sub(1);
        }
        pos = (pos as ::core::ffi::c_ulong)
            .wrapping_add(CommandCopyLen(&raw const cmd) as ::core::ffi::c_ulong)
            as size_t as size_t;
        if CommandCopyLen(&raw const cmd) != 0 {
            prev_byte2 = *ringbuffer.offset((pos.wrapping_sub(2 as size_t) & mask) as isize);
            prev_byte = *ringbuffer.offset((pos.wrapping_sub(1 as size_t) & mask) as isize);
            if cmd.cmd_prefix_ as ::core::ffi::c_int >= 128 as ::core::ffi::c_int {
                BlockSplitterAddSymbolDistance(
                    &raw mut (*arena).dist_blocks,
                    (cmd.dist_prefix_ as ::core::ffi::c_int & 0x3ff as ::core::ffi::c_int)
                        as size_t,
                );
            }
        }
        i = i.wrapping_add(1);
    }
    if num_contexts == 1 as size_t {
        BlockSplitterFinishBlockLiteral(&raw mut (*arena).lit_blocks.plain, BROTLI_TRUE);
    } else {
        ContextBlockSplitterFinishBlock(&raw mut (*arena).lit_blocks.ctx, m, BROTLI_TRUE);
        if 0 as ::core::ffi::c_int != 0 {
            return;
        }
    }
    BlockSplitterFinishBlockCommand(&raw mut (*arena).cmd_blocks, BROTLI_TRUE);
    BlockSplitterFinishBlockDistance(&raw mut (*arena).dist_blocks, BROTLI_TRUE);
    if num_contexts > 1 as size_t {
        MapStaticContexts(m, num_contexts, static_context_map, mb);
    }
}
#[no_mangle]
pub unsafe extern "C" fn BrotliBuildMetaBlockGreedy(
    mut m: *mut MemoryManager,
    mut ringbuffer: *const uint8_t,
    mut pos: size_t,
    mut mask: size_t,
    mut prev_byte: uint8_t,
    mut prev_byte2: uint8_t,
    mut literal_context_lut: ContextLut,
    mut num_contexts: size_t,
    mut static_context_map: *const uint32_t,
    mut commands: *const Command,
    mut n_commands: size_t,
    mut mb: *mut MetaBlockSplit,
) {
    let mut arena: *mut GreedyMetablockArena = if 1 as ::core::ffi::c_int > 0 as ::core::ffi::c_int
    {
        BrotliAllocate(
            m,
            (1 as size_t).wrapping_mul(::core::mem::size_of::<GreedyMetablockArena>() as size_t),
        ) as *mut GreedyMetablockArena
    } else {
        ::core::ptr::null_mut::<GreedyMetablockArena>()
    };
    if 0 as ::core::ffi::c_int != 0 || 0 as ::core::ffi::c_int != 0 {
        return;
    }
    if num_contexts == 1 as size_t {
        BrotliBuildMetaBlockGreedyInternal(
            m,
            arena,
            ringbuffer,
            pos,
            mask,
            prev_byte,
            prev_byte2,
            literal_context_lut,
            1 as size_t,
            ::core::ptr::null::<uint32_t>(),
            commands,
            n_commands,
            mb,
        );
    } else {
        BrotliBuildMetaBlockGreedyInternal(
            m,
            arena,
            ringbuffer,
            pos,
            mask,
            prev_byte,
            prev_byte2,
            literal_context_lut,
            num_contexts,
            static_context_map,
            commands,
            n_commands,
            mb,
        );
    }
    BrotliFree(m, arena as *mut ::core::ffi::c_void);
    arena = ::core::ptr::null_mut::<GreedyMetablockArena>();
}
#[no_mangle]
pub unsafe extern "C" fn BrotliOptimizeHistograms(
    mut num_distance_codes: uint32_t,
    mut mb: *mut MetaBlockSplit,
) {
    let mut good_for_rle: [uint8_t; 704] = [0; 704];
    let mut i: size_t = 0;
    i = 0 as size_t;
    while i < (*mb).literal_histograms_size {
        BrotliOptimizeHuffmanCountsForRle(
            256 as size_t,
            &raw mut (*(*mb).literal_histograms.offset(i as isize)).data_ as *mut uint32_t,
            &raw mut good_for_rle as *mut uint8_t,
        );
        i = i.wrapping_add(1);
    }
    i = 0 as size_t;
    while i < (*mb).command_histograms_size {
        BrotliOptimizeHuffmanCountsForRle(
            BROTLI_NUM_COMMAND_SYMBOLS as size_t,
            &raw mut (*(*mb).command_histograms.offset(i as isize)).data_ as *mut uint32_t,
            &raw mut good_for_rle as *mut uint8_t,
        );
        i = i.wrapping_add(1);
    }
    i = 0 as size_t;
    while i < (*mb).distance_histograms_size {
        BrotliOptimizeHuffmanCountsForRle(
            num_distance_codes as size_t,
            &raw mut (*(*mb).distance_histograms.offset(i as isize)).data_ as *mut uint32_t,
            &raw mut good_for_rle as *mut uint8_t,
        );
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn InitBlockSplitterLiteral(
    mut m: *mut MemoryManager,
    mut self_0: *mut BlockSplitterLiteral,
    mut alphabet_size: size_t,
    mut min_block_size: size_t,
    mut split_threshold: ::core::ffi::c_double,
    mut num_symbols: size_t,
    mut split: *mut BlockSplit,
    mut histograms: *mut *mut HistogramLiteral,
    mut histograms_size: *mut size_t,
) {
    let mut max_num_blocks: size_t = num_symbols
        .wrapping_div(min_block_size)
        .wrapping_add(1 as size_t);
    let mut max_num_types: size_t = brotli_min_size_t(
        max_num_blocks,
        (256 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as size_t,
    );
    (*self_0).alphabet_size_ = alphabet_size;
    (*self_0).min_block_size_ = min_block_size;
    (*self_0).split_threshold_ = split_threshold;
    (*self_0).num_blocks_ = 0 as size_t;
    (*self_0).split_ = split;
    (*self_0).histograms_size_ = histograms_size;
    (*self_0).target_block_size_ = min_block_size;
    (*self_0).block_size_ = 0 as size_t;
    (*self_0).curr_histogram_ix_ = 0 as size_t;
    (*self_0).merge_last_count_ = 0 as size_t;
    if (*split).types_alloc_size < max_num_blocks {
        let mut _new_size: size_t = if (*split).types_alloc_size == 0 as size_t {
            max_num_blocks
        } else {
            (*split).types_alloc_size
        };
        let mut new_array: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        while _new_size < max_num_blocks {
            _new_size = (_new_size as ::core::ffi::c_ulong).wrapping_mul(2 as ::core::ffi::c_ulong)
                as size_t as size_t;
        }
        new_array = if _new_size > 0 as size_t {
            BrotliAllocate(
                m,
                _new_size.wrapping_mul(::core::mem::size_of::<uint8_t>() as size_t),
            ) as *mut uint8_t
        } else {
            ::core::ptr::null_mut::<uint8_t>()
        };
        if 0 as ::core::ffi::c_int == 0
            && 0 as ::core::ffi::c_int == 0
            && (*split).types_alloc_size != 0 as size_t
        {
            memcpy(
                new_array as *mut ::core::ffi::c_void,
                (*split).types as *const ::core::ffi::c_void,
                (*split)
                    .types_alloc_size
                    .wrapping_mul(::core::mem::size_of::<uint8_t>() as size_t),
            );
        }
        BrotliFree(m, (*split).types as *mut ::core::ffi::c_void);
        (*split).types = ::core::ptr::null_mut::<uint8_t>();
        (*split).types = new_array;
        (*split).types_alloc_size = _new_size;
    }
    if (*split).lengths_alloc_size < max_num_blocks {
        let mut _new_size_0: size_t = if (*split).lengths_alloc_size == 0 as size_t {
            max_num_blocks
        } else {
            (*split).lengths_alloc_size
        };
        let mut new_array_0: *mut uint32_t = ::core::ptr::null_mut::<uint32_t>();
        while _new_size_0 < max_num_blocks {
            _new_size_0 = (_new_size_0 as ::core::ffi::c_ulong)
                .wrapping_mul(2 as ::core::ffi::c_ulong) as size_t
                as size_t;
        }
        new_array_0 = if _new_size_0 > 0 as size_t {
            BrotliAllocate(
                m,
                _new_size_0.wrapping_mul(::core::mem::size_of::<uint32_t>() as size_t),
            ) as *mut uint32_t
        } else {
            ::core::ptr::null_mut::<uint32_t>()
        };
        if 0 as ::core::ffi::c_int == 0
            && 0 as ::core::ffi::c_int == 0
            && (*split).lengths_alloc_size != 0 as size_t
        {
            memcpy(
                new_array_0 as *mut ::core::ffi::c_void,
                (*split).lengths as *const ::core::ffi::c_void,
                (*split)
                    .lengths_alloc_size
                    .wrapping_mul(::core::mem::size_of::<uint32_t>() as size_t),
            );
        }
        BrotliFree(m, (*split).lengths as *mut ::core::ffi::c_void);
        (*split).lengths = ::core::ptr::null_mut::<uint32_t>();
        (*split).lengths = new_array_0;
        (*split).lengths_alloc_size = _new_size_0;
    }
    if 0 as ::core::ffi::c_int != 0 {
        return;
    }
    (*(*self_0).split_).num_blocks = max_num_blocks;
    *histograms_size = max_num_types;
    *histograms = if *histograms_size > 0 as size_t {
        BrotliAllocate(
            m,
            (*histograms_size).wrapping_mul(::core::mem::size_of::<HistogramLiteral>() as size_t),
        ) as *mut HistogramLiteral
    } else {
        ::core::ptr::null_mut::<HistogramLiteral>()
    };
    (*self_0).histograms_ = *histograms;
    if 0 as ::core::ffi::c_int != 0 || 0 as ::core::ffi::c_int != 0 {
        return;
    }
    HistogramClearLiteral(
        (*self_0)
            .histograms_
            .offset(0 as ::core::ffi::c_int as isize) as *mut HistogramLiteral,
    );
    (*self_0).last_histogram_ix_[1 as ::core::ffi::c_int as usize] = 0 as size_t;
    (*self_0).last_histogram_ix_[0 as ::core::ffi::c_int as usize] =
        (*self_0).last_histogram_ix_[1 as ::core::ffi::c_int as usize];
}
unsafe extern "C" fn BlockSplitterFinishBlockLiteral(
    mut self_0: *mut BlockSplitterLiteral,
    mut is_final: ::core::ffi::c_int,
) {
    let mut split: *mut BlockSplit = (*self_0).split_;
    let mut last_entropy: *mut ::core::ffi::c_double =
        &raw mut (*self_0).last_entropy_ as *mut ::core::ffi::c_double;
    let mut histograms: *mut HistogramLiteral = (*self_0).histograms_;
    (*self_0).block_size_ = brotli_max_size_t((*self_0).block_size_, (*self_0).min_block_size_);
    if (*self_0).num_blocks_ == 0 as size_t {
        *(*split).lengths.offset(0 as ::core::ffi::c_int as isize) =
            (*self_0).block_size_ as uint32_t;
        *(*split).types.offset(0 as ::core::ffi::c_int as isize) = 0 as uint8_t;
        *last_entropy.offset(0 as ::core::ffi::c_int as isize) = BrotliBitsEntropy(
            &raw mut (*histograms.offset(0 as ::core::ffi::c_int as isize)).data_ as *mut uint32_t,
            (*self_0).alphabet_size_,
        );
        *last_entropy.offset(1 as ::core::ffi::c_int as isize) =
            *last_entropy.offset(0 as ::core::ffi::c_int as isize);
        (*self_0).num_blocks_ = (*self_0).num_blocks_.wrapping_add(1);
        (*split).num_types = (*split).num_types.wrapping_add(1);
        (*self_0).curr_histogram_ix_ = (*self_0).curr_histogram_ix_.wrapping_add(1);
        if (*self_0).curr_histogram_ix_ < *(*self_0).histograms_size_ {
            HistogramClearLiteral(
                histograms.offset((*self_0).curr_histogram_ix_ as isize) as *mut HistogramLiteral
            );
        }
        (*self_0).block_size_ = 0 as size_t;
    } else if (*self_0).block_size_ > 0 as size_t {
        let mut entropy: ::core::ffi::c_double = BrotliBitsEntropy(
            &raw mut (*histograms.offset((*self_0).curr_histogram_ix_ as isize)).data_
                as *mut uint32_t,
            (*self_0).alphabet_size_,
        );
        let mut combined_entropy: [::core::ffi::c_double; 2] = [0.; 2];
        let mut diff: [::core::ffi::c_double; 2] = [0.; 2];
        let mut j: size_t = 0;
        j = 0 as size_t;
        while j < 2 as size_t {
            let mut last_histogram_ix: size_t = (*self_0).last_histogram_ix_[j as usize];
            (*self_0).combined_histo[j as usize] =
                *histograms.offset((*self_0).curr_histogram_ix_ as isize);
            HistogramAddHistogramLiteral(
                (&raw mut (*self_0).combined_histo as *mut HistogramLiteral).offset(j as isize)
                    as *mut HistogramLiteral,
                histograms.offset(last_histogram_ix as isize) as *mut HistogramLiteral,
            );
            combined_entropy[j as usize] = BrotliBitsEntropy(
                (&raw mut (*(&raw mut (*self_0).combined_histo as *mut HistogramLiteral)
                    .offset(j as isize))
                .data_ as *mut uint32_t)
                    .offset(0 as ::core::ffi::c_int as isize) as *mut uint32_t,
                (*self_0).alphabet_size_,
            );
            diff[j as usize] =
                combined_entropy[j as usize] - entropy - *last_entropy.offset(j as isize);
            j = j.wrapping_add(1);
        }
        if (*split).num_types < BROTLI_MAX_NUMBER_OF_BLOCK_TYPES as size_t
            && diff[0 as ::core::ffi::c_int as usize] > (*self_0).split_threshold_
            && diff[1 as ::core::ffi::c_int as usize] > (*self_0).split_threshold_
        {
            *(*split).lengths.offset((*self_0).num_blocks_ as isize) =
                (*self_0).block_size_ as uint32_t;
            *(*split).types.offset((*self_0).num_blocks_ as isize) = (*split).num_types as uint8_t;
            (*self_0).last_histogram_ix_[1 as ::core::ffi::c_int as usize] =
                (*self_0).last_histogram_ix_[0 as ::core::ffi::c_int as usize];
            (*self_0).last_histogram_ix_[0 as ::core::ffi::c_int as usize] =
                (*split).num_types as uint8_t as size_t;
            *last_entropy.offset(1 as ::core::ffi::c_int as isize) =
                *last_entropy.offset(0 as ::core::ffi::c_int as isize);
            *last_entropy.offset(0 as ::core::ffi::c_int as isize) = entropy;
            (*self_0).num_blocks_ = (*self_0).num_blocks_.wrapping_add(1);
            (*split).num_types = (*split).num_types.wrapping_add(1);
            (*self_0).curr_histogram_ix_ = (*self_0).curr_histogram_ix_.wrapping_add(1);
            if (*self_0).curr_histogram_ix_ < *(*self_0).histograms_size_ {
                HistogramClearLiteral(histograms.offset((*self_0).curr_histogram_ix_ as isize)
                    as *mut HistogramLiteral);
            }
            (*self_0).block_size_ = 0 as size_t;
            (*self_0).merge_last_count_ = 0 as size_t;
            (*self_0).target_block_size_ = (*self_0).min_block_size_;
        } else if diff[1 as ::core::ffi::c_int as usize]
            < diff[0 as ::core::ffi::c_int as usize] - 20.0f64
        {
            *(*split).lengths.offset((*self_0).num_blocks_ as isize) =
                (*self_0).block_size_ as uint32_t;
            *(*split).types.offset((*self_0).num_blocks_ as isize) = *(*split)
                .types
                .offset((*self_0).num_blocks_.wrapping_sub(2 as size_t) as isize);
            let mut __brotli_swap_tmp: size_t =
                (*self_0).last_histogram_ix_[0 as ::core::ffi::c_int as usize];
            (*self_0).last_histogram_ix_[0 as ::core::ffi::c_int as usize] =
                (*self_0).last_histogram_ix_[1 as ::core::ffi::c_int as usize];
            (*self_0).last_histogram_ix_[1 as ::core::ffi::c_int as usize] = __brotli_swap_tmp;
            *histograms
                .offset((*self_0).last_histogram_ix_[0 as ::core::ffi::c_int as usize] as isize) =
                (*self_0).combined_histo[1 as ::core::ffi::c_int as usize];
            *last_entropy.offset(1 as ::core::ffi::c_int as isize) =
                *last_entropy.offset(0 as ::core::ffi::c_int as isize);
            *last_entropy.offset(0 as ::core::ffi::c_int as isize) =
                combined_entropy[1 as ::core::ffi::c_int as usize];
            (*self_0).num_blocks_ = (*self_0).num_blocks_.wrapping_add(1);
            (*self_0).block_size_ = 0 as size_t;
            HistogramClearLiteral(
                histograms.offset((*self_0).curr_histogram_ix_ as isize) as *mut HistogramLiteral
            );
            (*self_0).merge_last_count_ = 0 as size_t;
            (*self_0).target_block_size_ = (*self_0).min_block_size_;
        } else {
            let ref mut fresh3 = *(*split)
                .lengths
                .offset((*self_0).num_blocks_.wrapping_sub(1 as size_t) as isize);
            *fresh3 = (*fresh3 as ::core::ffi::c_uint)
                .wrapping_add((*self_0).block_size_ as uint32_t as ::core::ffi::c_uint)
                as uint32_t as uint32_t;
            *histograms
                .offset((*self_0).last_histogram_ix_[0 as ::core::ffi::c_int as usize] as isize) =
                (*self_0).combined_histo[0 as ::core::ffi::c_int as usize];
            *last_entropy.offset(0 as ::core::ffi::c_int as isize) =
                combined_entropy[0 as ::core::ffi::c_int as usize];
            if (*split).num_types == 1 as size_t {
                *last_entropy.offset(1 as ::core::ffi::c_int as isize) =
                    *last_entropy.offset(0 as ::core::ffi::c_int as isize);
            }
            (*self_0).block_size_ = 0 as size_t;
            HistogramClearLiteral(
                histograms.offset((*self_0).curr_histogram_ix_ as isize) as *mut HistogramLiteral
            );
            (*self_0).merge_last_count_ = (*self_0).merge_last_count_.wrapping_add(1);
            if (*self_0).merge_last_count_ > 1 as size_t {
                (*self_0).target_block_size_ = ((*self_0).target_block_size_
                    as ::core::ffi::c_ulong)
                    .wrapping_add((*self_0).min_block_size_ as ::core::ffi::c_ulong)
                    as size_t as size_t;
            }
        }
    }
    if is_final != 0 {
        *(*self_0).histograms_size_ = (*split).num_types;
        (*split).num_blocks = (*self_0).num_blocks_;
    }
}
unsafe extern "C" fn BlockSplitterAddSymbolLiteral(
    mut self_0: *mut BlockSplitterLiteral,
    mut symbol: size_t,
) {
    HistogramAddLiteral(
        (*self_0)
            .histograms_
            .offset((*self_0).curr_histogram_ix_ as isize) as *mut HistogramLiteral,
        symbol,
    );
    (*self_0).block_size_ = (*self_0).block_size_.wrapping_add(1);
    if (*self_0).block_size_ == (*self_0).target_block_size_ {
        BlockSplitterFinishBlockLiteral(self_0, BROTLI_FALSE);
    }
}
unsafe extern "C" fn InitBlockSplitterCommand(
    mut m: *mut MemoryManager,
    mut self_0: *mut BlockSplitterCommand,
    mut alphabet_size: size_t,
    mut min_block_size: size_t,
    mut split_threshold: ::core::ffi::c_double,
    mut num_symbols: size_t,
    mut split: *mut BlockSplit,
    mut histograms: *mut *mut HistogramCommand,
    mut histograms_size: *mut size_t,
) {
    let mut max_num_blocks: size_t = num_symbols
        .wrapping_div(min_block_size)
        .wrapping_add(1 as size_t);
    let mut max_num_types: size_t = brotli_min_size_t(
        max_num_blocks,
        (256 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as size_t,
    );
    (*self_0).alphabet_size_ = alphabet_size;
    (*self_0).min_block_size_ = min_block_size;
    (*self_0).split_threshold_ = split_threshold;
    (*self_0).num_blocks_ = 0 as size_t;
    (*self_0).split_ = split;
    (*self_0).histograms_size_ = histograms_size;
    (*self_0).target_block_size_ = min_block_size;
    (*self_0).block_size_ = 0 as size_t;
    (*self_0).curr_histogram_ix_ = 0 as size_t;
    (*self_0).merge_last_count_ = 0 as size_t;
    if (*split).types_alloc_size < max_num_blocks {
        let mut _new_size: size_t = if (*split).types_alloc_size == 0 as size_t {
            max_num_blocks
        } else {
            (*split).types_alloc_size
        };
        let mut new_array: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        while _new_size < max_num_blocks {
            _new_size = (_new_size as ::core::ffi::c_ulong).wrapping_mul(2 as ::core::ffi::c_ulong)
                as size_t as size_t;
        }
        new_array = if _new_size > 0 as size_t {
            BrotliAllocate(
                m,
                _new_size.wrapping_mul(::core::mem::size_of::<uint8_t>() as size_t),
            ) as *mut uint8_t
        } else {
            ::core::ptr::null_mut::<uint8_t>()
        };
        if 0 as ::core::ffi::c_int == 0
            && 0 as ::core::ffi::c_int == 0
            && (*split).types_alloc_size != 0 as size_t
        {
            memcpy(
                new_array as *mut ::core::ffi::c_void,
                (*split).types as *const ::core::ffi::c_void,
                (*split)
                    .types_alloc_size
                    .wrapping_mul(::core::mem::size_of::<uint8_t>() as size_t),
            );
        }
        BrotliFree(m, (*split).types as *mut ::core::ffi::c_void);
        (*split).types = ::core::ptr::null_mut::<uint8_t>();
        (*split).types = new_array;
        (*split).types_alloc_size = _new_size;
    }
    if (*split).lengths_alloc_size < max_num_blocks {
        let mut _new_size_0: size_t = if (*split).lengths_alloc_size == 0 as size_t {
            max_num_blocks
        } else {
            (*split).lengths_alloc_size
        };
        let mut new_array_0: *mut uint32_t = ::core::ptr::null_mut::<uint32_t>();
        while _new_size_0 < max_num_blocks {
            _new_size_0 = (_new_size_0 as ::core::ffi::c_ulong)
                .wrapping_mul(2 as ::core::ffi::c_ulong) as size_t
                as size_t;
        }
        new_array_0 = if _new_size_0 > 0 as size_t {
            BrotliAllocate(
                m,
                _new_size_0.wrapping_mul(::core::mem::size_of::<uint32_t>() as size_t),
            ) as *mut uint32_t
        } else {
            ::core::ptr::null_mut::<uint32_t>()
        };
        if 0 as ::core::ffi::c_int == 0
            && 0 as ::core::ffi::c_int == 0
            && (*split).lengths_alloc_size != 0 as size_t
        {
            memcpy(
                new_array_0 as *mut ::core::ffi::c_void,
                (*split).lengths as *const ::core::ffi::c_void,
                (*split)
                    .lengths_alloc_size
                    .wrapping_mul(::core::mem::size_of::<uint32_t>() as size_t),
            );
        }
        BrotliFree(m, (*split).lengths as *mut ::core::ffi::c_void);
        (*split).lengths = ::core::ptr::null_mut::<uint32_t>();
        (*split).lengths = new_array_0;
        (*split).lengths_alloc_size = _new_size_0;
    }
    if 0 as ::core::ffi::c_int != 0 {
        return;
    }
    (*(*self_0).split_).num_blocks = max_num_blocks;
    *histograms_size = max_num_types;
    *histograms = if *histograms_size > 0 as size_t {
        BrotliAllocate(
            m,
            (*histograms_size).wrapping_mul(::core::mem::size_of::<HistogramCommand>() as size_t),
        ) as *mut HistogramCommand
    } else {
        ::core::ptr::null_mut::<HistogramCommand>()
    };
    (*self_0).histograms_ = *histograms;
    if 0 as ::core::ffi::c_int != 0 || 0 as ::core::ffi::c_int != 0 {
        return;
    }
    HistogramClearCommand(
        (*self_0)
            .histograms_
            .offset(0 as ::core::ffi::c_int as isize) as *mut HistogramCommand,
    );
    (*self_0).last_histogram_ix_[1 as ::core::ffi::c_int as usize] = 0 as size_t;
    (*self_0).last_histogram_ix_[0 as ::core::ffi::c_int as usize] =
        (*self_0).last_histogram_ix_[1 as ::core::ffi::c_int as usize];
}
unsafe extern "C" fn BlockSplitterFinishBlockCommand(
    mut self_0: *mut BlockSplitterCommand,
    mut is_final: ::core::ffi::c_int,
) {
    let mut split: *mut BlockSplit = (*self_0).split_;
    let mut last_entropy: *mut ::core::ffi::c_double =
        &raw mut (*self_0).last_entropy_ as *mut ::core::ffi::c_double;
    let mut histograms: *mut HistogramCommand = (*self_0).histograms_;
    (*self_0).block_size_ = brotli_max_size_t((*self_0).block_size_, (*self_0).min_block_size_);
    if (*self_0).num_blocks_ == 0 as size_t {
        *(*split).lengths.offset(0 as ::core::ffi::c_int as isize) =
            (*self_0).block_size_ as uint32_t;
        *(*split).types.offset(0 as ::core::ffi::c_int as isize) = 0 as uint8_t;
        *last_entropy.offset(0 as ::core::ffi::c_int as isize) = BrotliBitsEntropy(
            &raw mut (*histograms.offset(0 as ::core::ffi::c_int as isize)).data_ as *mut uint32_t,
            (*self_0).alphabet_size_,
        );
        *last_entropy.offset(1 as ::core::ffi::c_int as isize) =
            *last_entropy.offset(0 as ::core::ffi::c_int as isize);
        (*self_0).num_blocks_ = (*self_0).num_blocks_.wrapping_add(1);
        (*split).num_types = (*split).num_types.wrapping_add(1);
        (*self_0).curr_histogram_ix_ = (*self_0).curr_histogram_ix_.wrapping_add(1);
        if (*self_0).curr_histogram_ix_ < *(*self_0).histograms_size_ {
            HistogramClearCommand(
                histograms.offset((*self_0).curr_histogram_ix_ as isize) as *mut HistogramCommand
            );
        }
        (*self_0).block_size_ = 0 as size_t;
    } else if (*self_0).block_size_ > 0 as size_t {
        let mut entropy: ::core::ffi::c_double = BrotliBitsEntropy(
            &raw mut (*histograms.offset((*self_0).curr_histogram_ix_ as isize)).data_
                as *mut uint32_t,
            (*self_0).alphabet_size_,
        );
        let mut combined_entropy: [::core::ffi::c_double; 2] = [0.; 2];
        let mut diff: [::core::ffi::c_double; 2] = [0.; 2];
        let mut j: size_t = 0;
        j = 0 as size_t;
        while j < 2 as size_t {
            let mut last_histogram_ix: size_t = (*self_0).last_histogram_ix_[j as usize];
            (*self_0).combined_histo[j as usize] =
                *histograms.offset((*self_0).curr_histogram_ix_ as isize);
            HistogramAddHistogramCommand(
                (&raw mut (*self_0).combined_histo as *mut HistogramCommand).offset(j as isize)
                    as *mut HistogramCommand,
                histograms.offset(last_histogram_ix as isize) as *mut HistogramCommand,
            );
            combined_entropy[j as usize] = BrotliBitsEntropy(
                (&raw mut (*(&raw mut (*self_0).combined_histo as *mut HistogramCommand)
                    .offset(j as isize))
                .data_ as *mut uint32_t)
                    .offset(0 as ::core::ffi::c_int as isize) as *mut uint32_t,
                (*self_0).alphabet_size_,
            );
            diff[j as usize] =
                combined_entropy[j as usize] - entropy - *last_entropy.offset(j as isize);
            j = j.wrapping_add(1);
        }
        if (*split).num_types < BROTLI_MAX_NUMBER_OF_BLOCK_TYPES as size_t
            && diff[0 as ::core::ffi::c_int as usize] > (*self_0).split_threshold_
            && diff[1 as ::core::ffi::c_int as usize] > (*self_0).split_threshold_
        {
            *(*split).lengths.offset((*self_0).num_blocks_ as isize) =
                (*self_0).block_size_ as uint32_t;
            *(*split).types.offset((*self_0).num_blocks_ as isize) = (*split).num_types as uint8_t;
            (*self_0).last_histogram_ix_[1 as ::core::ffi::c_int as usize] =
                (*self_0).last_histogram_ix_[0 as ::core::ffi::c_int as usize];
            (*self_0).last_histogram_ix_[0 as ::core::ffi::c_int as usize] =
                (*split).num_types as uint8_t as size_t;
            *last_entropy.offset(1 as ::core::ffi::c_int as isize) =
                *last_entropy.offset(0 as ::core::ffi::c_int as isize);
            *last_entropy.offset(0 as ::core::ffi::c_int as isize) = entropy;
            (*self_0).num_blocks_ = (*self_0).num_blocks_.wrapping_add(1);
            (*split).num_types = (*split).num_types.wrapping_add(1);
            (*self_0).curr_histogram_ix_ = (*self_0).curr_histogram_ix_.wrapping_add(1);
            if (*self_0).curr_histogram_ix_ < *(*self_0).histograms_size_ {
                HistogramClearCommand(histograms.offset((*self_0).curr_histogram_ix_ as isize)
                    as *mut HistogramCommand);
            }
            (*self_0).block_size_ = 0 as size_t;
            (*self_0).merge_last_count_ = 0 as size_t;
            (*self_0).target_block_size_ = (*self_0).min_block_size_;
        } else if diff[1 as ::core::ffi::c_int as usize]
            < diff[0 as ::core::ffi::c_int as usize] - 20.0f64
        {
            *(*split).lengths.offset((*self_0).num_blocks_ as isize) =
                (*self_0).block_size_ as uint32_t;
            *(*split).types.offset((*self_0).num_blocks_ as isize) = *(*split)
                .types
                .offset((*self_0).num_blocks_.wrapping_sub(2 as size_t) as isize);
            let mut __brotli_swap_tmp: size_t =
                (*self_0).last_histogram_ix_[0 as ::core::ffi::c_int as usize];
            (*self_0).last_histogram_ix_[0 as ::core::ffi::c_int as usize] =
                (*self_0).last_histogram_ix_[1 as ::core::ffi::c_int as usize];
            (*self_0).last_histogram_ix_[1 as ::core::ffi::c_int as usize] = __brotli_swap_tmp;
            *histograms
                .offset((*self_0).last_histogram_ix_[0 as ::core::ffi::c_int as usize] as isize) =
                (*self_0).combined_histo[1 as ::core::ffi::c_int as usize];
            *last_entropy.offset(1 as ::core::ffi::c_int as isize) =
                *last_entropy.offset(0 as ::core::ffi::c_int as isize);
            *last_entropy.offset(0 as ::core::ffi::c_int as isize) =
                combined_entropy[1 as ::core::ffi::c_int as usize];
            (*self_0).num_blocks_ = (*self_0).num_blocks_.wrapping_add(1);
            (*self_0).block_size_ = 0 as size_t;
            HistogramClearCommand(
                histograms.offset((*self_0).curr_histogram_ix_ as isize) as *mut HistogramCommand
            );
            (*self_0).merge_last_count_ = 0 as size_t;
            (*self_0).target_block_size_ = (*self_0).min_block_size_;
        } else {
            let ref mut fresh1 = *(*split)
                .lengths
                .offset((*self_0).num_blocks_.wrapping_sub(1 as size_t) as isize);
            *fresh1 = (*fresh1 as ::core::ffi::c_uint)
                .wrapping_add((*self_0).block_size_ as uint32_t as ::core::ffi::c_uint)
                as uint32_t as uint32_t;
            *histograms
                .offset((*self_0).last_histogram_ix_[0 as ::core::ffi::c_int as usize] as isize) =
                (*self_0).combined_histo[0 as ::core::ffi::c_int as usize];
            *last_entropy.offset(0 as ::core::ffi::c_int as isize) =
                combined_entropy[0 as ::core::ffi::c_int as usize];
            if (*split).num_types == 1 as size_t {
                *last_entropy.offset(1 as ::core::ffi::c_int as isize) =
                    *last_entropy.offset(0 as ::core::ffi::c_int as isize);
            }
            (*self_0).block_size_ = 0 as size_t;
            HistogramClearCommand(
                histograms.offset((*self_0).curr_histogram_ix_ as isize) as *mut HistogramCommand
            );
            (*self_0).merge_last_count_ = (*self_0).merge_last_count_.wrapping_add(1);
            if (*self_0).merge_last_count_ > 1 as size_t {
                (*self_0).target_block_size_ = ((*self_0).target_block_size_
                    as ::core::ffi::c_ulong)
                    .wrapping_add((*self_0).min_block_size_ as ::core::ffi::c_ulong)
                    as size_t as size_t;
            }
        }
    }
    if is_final != 0 {
        *(*self_0).histograms_size_ = (*split).num_types;
        (*split).num_blocks = (*self_0).num_blocks_;
    }
}
unsafe extern "C" fn BlockSplitterAddSymbolCommand(
    mut self_0: *mut BlockSplitterCommand,
    mut symbol: size_t,
) {
    HistogramAddCommand(
        (*self_0)
            .histograms_
            .offset((*self_0).curr_histogram_ix_ as isize) as *mut HistogramCommand,
        symbol,
    );
    (*self_0).block_size_ = (*self_0).block_size_.wrapping_add(1);
    if (*self_0).block_size_ == (*self_0).target_block_size_ {
        BlockSplitterFinishBlockCommand(self_0, BROTLI_FALSE);
    }
}
unsafe extern "C" fn InitBlockSplitterDistance(
    mut m: *mut MemoryManager,
    mut self_0: *mut BlockSplitterDistance,
    mut alphabet_size: size_t,
    mut min_block_size: size_t,
    mut split_threshold: ::core::ffi::c_double,
    mut num_symbols: size_t,
    mut split: *mut BlockSplit,
    mut histograms: *mut *mut HistogramDistance,
    mut histograms_size: *mut size_t,
) {
    let mut max_num_blocks: size_t = num_symbols
        .wrapping_div(min_block_size)
        .wrapping_add(1 as size_t);
    let mut max_num_types: size_t = brotli_min_size_t(
        max_num_blocks,
        (256 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as size_t,
    );
    (*self_0).alphabet_size_ = alphabet_size;
    (*self_0).min_block_size_ = min_block_size;
    (*self_0).split_threshold_ = split_threshold;
    (*self_0).num_blocks_ = 0 as size_t;
    (*self_0).split_ = split;
    (*self_0).histograms_size_ = histograms_size;
    (*self_0).target_block_size_ = min_block_size;
    (*self_0).block_size_ = 0 as size_t;
    (*self_0).curr_histogram_ix_ = 0 as size_t;
    (*self_0).merge_last_count_ = 0 as size_t;
    if (*split).types_alloc_size < max_num_blocks {
        let mut _new_size: size_t = if (*split).types_alloc_size == 0 as size_t {
            max_num_blocks
        } else {
            (*split).types_alloc_size
        };
        let mut new_array: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        while _new_size < max_num_blocks {
            _new_size = (_new_size as ::core::ffi::c_ulong).wrapping_mul(2 as ::core::ffi::c_ulong)
                as size_t as size_t;
        }
        new_array = if _new_size > 0 as size_t {
            BrotliAllocate(
                m,
                _new_size.wrapping_mul(::core::mem::size_of::<uint8_t>() as size_t),
            ) as *mut uint8_t
        } else {
            ::core::ptr::null_mut::<uint8_t>()
        };
        if 0 as ::core::ffi::c_int == 0
            && 0 as ::core::ffi::c_int == 0
            && (*split).types_alloc_size != 0 as size_t
        {
            memcpy(
                new_array as *mut ::core::ffi::c_void,
                (*split).types as *const ::core::ffi::c_void,
                (*split)
                    .types_alloc_size
                    .wrapping_mul(::core::mem::size_of::<uint8_t>() as size_t),
            );
        }
        BrotliFree(m, (*split).types as *mut ::core::ffi::c_void);
        (*split).types = ::core::ptr::null_mut::<uint8_t>();
        (*split).types = new_array;
        (*split).types_alloc_size = _new_size;
    }
    if (*split).lengths_alloc_size < max_num_blocks {
        let mut _new_size_0: size_t = if (*split).lengths_alloc_size == 0 as size_t {
            max_num_blocks
        } else {
            (*split).lengths_alloc_size
        };
        let mut new_array_0: *mut uint32_t = ::core::ptr::null_mut::<uint32_t>();
        while _new_size_0 < max_num_blocks {
            _new_size_0 = (_new_size_0 as ::core::ffi::c_ulong)
                .wrapping_mul(2 as ::core::ffi::c_ulong) as size_t
                as size_t;
        }
        new_array_0 = if _new_size_0 > 0 as size_t {
            BrotliAllocate(
                m,
                _new_size_0.wrapping_mul(::core::mem::size_of::<uint32_t>() as size_t),
            ) as *mut uint32_t
        } else {
            ::core::ptr::null_mut::<uint32_t>()
        };
        if 0 as ::core::ffi::c_int == 0
            && 0 as ::core::ffi::c_int == 0
            && (*split).lengths_alloc_size != 0 as size_t
        {
            memcpy(
                new_array_0 as *mut ::core::ffi::c_void,
                (*split).lengths as *const ::core::ffi::c_void,
                (*split)
                    .lengths_alloc_size
                    .wrapping_mul(::core::mem::size_of::<uint32_t>() as size_t),
            );
        }
        BrotliFree(m, (*split).lengths as *mut ::core::ffi::c_void);
        (*split).lengths = ::core::ptr::null_mut::<uint32_t>();
        (*split).lengths = new_array_0;
        (*split).lengths_alloc_size = _new_size_0;
    }
    if 0 as ::core::ffi::c_int != 0 {
        return;
    }
    (*(*self_0).split_).num_blocks = max_num_blocks;
    *histograms_size = max_num_types;
    *histograms = if *histograms_size > 0 as size_t {
        BrotliAllocate(
            m,
            (*histograms_size).wrapping_mul(::core::mem::size_of::<HistogramDistance>() as size_t),
        ) as *mut HistogramDistance
    } else {
        ::core::ptr::null_mut::<HistogramDistance>()
    };
    (*self_0).histograms_ = *histograms;
    if 0 as ::core::ffi::c_int != 0 || 0 as ::core::ffi::c_int != 0 {
        return;
    }
    HistogramClearDistance(
        (*self_0)
            .histograms_
            .offset(0 as ::core::ffi::c_int as isize) as *mut HistogramDistance,
    );
    (*self_0).last_histogram_ix_[1 as ::core::ffi::c_int as usize] = 0 as size_t;
    (*self_0).last_histogram_ix_[0 as ::core::ffi::c_int as usize] =
        (*self_0).last_histogram_ix_[1 as ::core::ffi::c_int as usize];
}
unsafe extern "C" fn BlockSplitterFinishBlockDistance(
    mut self_0: *mut BlockSplitterDistance,
    mut is_final: ::core::ffi::c_int,
) {
    let mut split: *mut BlockSplit = (*self_0).split_;
    let mut last_entropy: *mut ::core::ffi::c_double =
        &raw mut (*self_0).last_entropy_ as *mut ::core::ffi::c_double;
    let mut histograms: *mut HistogramDistance = (*self_0).histograms_;
    (*self_0).block_size_ = brotli_max_size_t((*self_0).block_size_, (*self_0).min_block_size_);
    if (*self_0).num_blocks_ == 0 as size_t {
        *(*split).lengths.offset(0 as ::core::ffi::c_int as isize) =
            (*self_0).block_size_ as uint32_t;
        *(*split).types.offset(0 as ::core::ffi::c_int as isize) = 0 as uint8_t;
        *last_entropy.offset(0 as ::core::ffi::c_int as isize) = BrotliBitsEntropy(
            &raw mut (*histograms.offset(0 as ::core::ffi::c_int as isize)).data_ as *mut uint32_t,
            (*self_0).alphabet_size_,
        );
        *last_entropy.offset(1 as ::core::ffi::c_int as isize) =
            *last_entropy.offset(0 as ::core::ffi::c_int as isize);
        (*self_0).num_blocks_ = (*self_0).num_blocks_.wrapping_add(1);
        (*split).num_types = (*split).num_types.wrapping_add(1);
        (*self_0).curr_histogram_ix_ = (*self_0).curr_histogram_ix_.wrapping_add(1);
        if (*self_0).curr_histogram_ix_ < *(*self_0).histograms_size_ {
            HistogramClearDistance(
                histograms.offset((*self_0).curr_histogram_ix_ as isize) as *mut HistogramDistance
            );
        }
        (*self_0).block_size_ = 0 as size_t;
    } else if (*self_0).block_size_ > 0 as size_t {
        let mut entropy: ::core::ffi::c_double = BrotliBitsEntropy(
            &raw mut (*histograms.offset((*self_0).curr_histogram_ix_ as isize)).data_
                as *mut uint32_t,
            (*self_0).alphabet_size_,
        );
        let mut combined_entropy: [::core::ffi::c_double; 2] = [0.; 2];
        let mut diff: [::core::ffi::c_double; 2] = [0.; 2];
        let mut j: size_t = 0;
        j = 0 as size_t;
        while j < 2 as size_t {
            let mut last_histogram_ix: size_t = (*self_0).last_histogram_ix_[j as usize];
            (*self_0).combined_histo[j as usize] =
                *histograms.offset((*self_0).curr_histogram_ix_ as isize);
            HistogramAddHistogramDistance(
                (&raw mut (*self_0).combined_histo as *mut HistogramDistance).offset(j as isize)
                    as *mut HistogramDistance,
                histograms.offset(last_histogram_ix as isize) as *mut HistogramDistance,
            );
            combined_entropy[j as usize] = BrotliBitsEntropy(
                (&raw mut (*(&raw mut (*self_0).combined_histo as *mut HistogramDistance)
                    .offset(j as isize))
                .data_ as *mut uint32_t)
                    .offset(0 as ::core::ffi::c_int as isize) as *mut uint32_t,
                (*self_0).alphabet_size_,
            );
            diff[j as usize] =
                combined_entropy[j as usize] - entropy - *last_entropy.offset(j as isize);
            j = j.wrapping_add(1);
        }
        if (*split).num_types < BROTLI_MAX_NUMBER_OF_BLOCK_TYPES as size_t
            && diff[0 as ::core::ffi::c_int as usize] > (*self_0).split_threshold_
            && diff[1 as ::core::ffi::c_int as usize] > (*self_0).split_threshold_
        {
            *(*split).lengths.offset((*self_0).num_blocks_ as isize) =
                (*self_0).block_size_ as uint32_t;
            *(*split).types.offset((*self_0).num_blocks_ as isize) = (*split).num_types as uint8_t;
            (*self_0).last_histogram_ix_[1 as ::core::ffi::c_int as usize] =
                (*self_0).last_histogram_ix_[0 as ::core::ffi::c_int as usize];
            (*self_0).last_histogram_ix_[0 as ::core::ffi::c_int as usize] =
                (*split).num_types as uint8_t as size_t;
            *last_entropy.offset(1 as ::core::ffi::c_int as isize) =
                *last_entropy.offset(0 as ::core::ffi::c_int as isize);
            *last_entropy.offset(0 as ::core::ffi::c_int as isize) = entropy;
            (*self_0).num_blocks_ = (*self_0).num_blocks_.wrapping_add(1);
            (*split).num_types = (*split).num_types.wrapping_add(1);
            (*self_0).curr_histogram_ix_ = (*self_0).curr_histogram_ix_.wrapping_add(1);
            if (*self_0).curr_histogram_ix_ < *(*self_0).histograms_size_ {
                HistogramClearDistance(histograms.offset((*self_0).curr_histogram_ix_ as isize)
                    as *mut HistogramDistance);
            }
            (*self_0).block_size_ = 0 as size_t;
            (*self_0).merge_last_count_ = 0 as size_t;
            (*self_0).target_block_size_ = (*self_0).min_block_size_;
        } else if diff[1 as ::core::ffi::c_int as usize]
            < diff[0 as ::core::ffi::c_int as usize] - 20.0f64
        {
            *(*split).lengths.offset((*self_0).num_blocks_ as isize) =
                (*self_0).block_size_ as uint32_t;
            *(*split).types.offset((*self_0).num_blocks_ as isize) = *(*split)
                .types
                .offset((*self_0).num_blocks_.wrapping_sub(2 as size_t) as isize);
            let mut __brotli_swap_tmp: size_t =
                (*self_0).last_histogram_ix_[0 as ::core::ffi::c_int as usize];
            (*self_0).last_histogram_ix_[0 as ::core::ffi::c_int as usize] =
                (*self_0).last_histogram_ix_[1 as ::core::ffi::c_int as usize];
            (*self_0).last_histogram_ix_[1 as ::core::ffi::c_int as usize] = __brotli_swap_tmp;
            *histograms
                .offset((*self_0).last_histogram_ix_[0 as ::core::ffi::c_int as usize] as isize) =
                (*self_0).combined_histo[1 as ::core::ffi::c_int as usize];
            *last_entropy.offset(1 as ::core::ffi::c_int as isize) =
                *last_entropy.offset(0 as ::core::ffi::c_int as isize);
            *last_entropy.offset(0 as ::core::ffi::c_int as isize) =
                combined_entropy[1 as ::core::ffi::c_int as usize];
            (*self_0).num_blocks_ = (*self_0).num_blocks_.wrapping_add(1);
            (*self_0).block_size_ = 0 as size_t;
            HistogramClearDistance(
                histograms.offset((*self_0).curr_histogram_ix_ as isize) as *mut HistogramDistance
            );
            (*self_0).merge_last_count_ = 0 as size_t;
            (*self_0).target_block_size_ = (*self_0).min_block_size_;
        } else {
            let ref mut fresh0 = *(*split)
                .lengths
                .offset((*self_0).num_blocks_.wrapping_sub(1 as size_t) as isize);
            *fresh0 = (*fresh0 as ::core::ffi::c_uint)
                .wrapping_add((*self_0).block_size_ as uint32_t as ::core::ffi::c_uint)
                as uint32_t as uint32_t;
            *histograms
                .offset((*self_0).last_histogram_ix_[0 as ::core::ffi::c_int as usize] as isize) =
                (*self_0).combined_histo[0 as ::core::ffi::c_int as usize];
            *last_entropy.offset(0 as ::core::ffi::c_int as isize) =
                combined_entropy[0 as ::core::ffi::c_int as usize];
            if (*split).num_types == 1 as size_t {
                *last_entropy.offset(1 as ::core::ffi::c_int as isize) =
                    *last_entropy.offset(0 as ::core::ffi::c_int as isize);
            }
            (*self_0).block_size_ = 0 as size_t;
            HistogramClearDistance(
                histograms.offset((*self_0).curr_histogram_ix_ as isize) as *mut HistogramDistance
            );
            (*self_0).merge_last_count_ = (*self_0).merge_last_count_.wrapping_add(1);
            if (*self_0).merge_last_count_ > 1 as size_t {
                (*self_0).target_block_size_ = ((*self_0).target_block_size_
                    as ::core::ffi::c_ulong)
                    .wrapping_add((*self_0).min_block_size_ as ::core::ffi::c_ulong)
                    as size_t as size_t;
            }
        }
    }
    if is_final != 0 {
        *(*self_0).histograms_size_ = (*split).num_types;
        (*split).num_blocks = (*self_0).num_blocks_;
    }
}
unsafe extern "C" fn BlockSplitterAddSymbolDistance(
    mut self_0: *mut BlockSplitterDistance,
    mut symbol: size_t,
) {
    HistogramAddDistance(
        (*self_0)
            .histograms_
            .offset((*self_0).curr_histogram_ix_ as isize) as *mut HistogramDistance,
        symbol,
    );
    (*self_0).block_size_ = (*self_0).block_size_.wrapping_add(1);
    if (*self_0).block_size_ == (*self_0).target_block_size_ {
        BlockSplitterFinishBlockDistance(self_0, BROTLI_FALSE);
    }
}
