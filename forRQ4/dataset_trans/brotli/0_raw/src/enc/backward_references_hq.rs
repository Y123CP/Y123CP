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
    static kBrotliLog2Table: [::core::ffi::c_double; 256];
    fn BrotliAllocate(m: *mut MemoryManager, n: size_t) -> *mut ::core::ffi::c_void;
    fn BrotliFree(m: *mut MemoryManager, p: *mut ::core::ffi::c_void);
    fn log2(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    static kBrotliInsExtra: [uint32_t; 24];
    static kBrotliCopyExtra: [uint32_t; 24];
    fn BrotliFindAllStaticDictionaryMatches(
        dictionary: *const BrotliEncoderDictionary,
        data: *const uint8_t,
        min_length: size_t,
        max_length: size_t,
        matches: *mut uint32_t,
    ) -> ::core::ffi::c_int;
    fn BrotliEstimateBitCostsForLiterals(
        pos: size_t,
        len: size_t,
        mask: size_t,
        data: *const uint8_t,
        histogram: *mut size_t,
        cost: *mut ::core::ffi::c_float,
    );
}
pub type size_t = usize;
pub type __int8_t = i8;
pub type __uint8_t = u8;
pub type __uint16_t = u16;
pub type __uint32_t = u32;
pub type __uint64_t = u64;
pub type int8_t = __int8_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type brotli_alloc_func =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, size_t) -> *mut ::core::ffi::c_void>;
pub type brotli_free_func =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void) -> ()>;
pub type ContextLut = *const uint8_t;
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
pub struct HasherCommon {
    pub extra: [*mut ::core::ffi::c_void; 4],
    pub is_setup_: ::core::ffi::c_int,
    pub dict_num_lookups: size_t,
    pub dict_num_matches: size_t,
    pub params: BrotliHasherParams,
    pub is_prepared_: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BackwardMatch {
    pub distance: uint32_t,
    pub length_and_code: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct H10 {
    pub window_mask_: size_t,
    pub buckets_: *mut uint32_t,
    pub invalid_pos_: uint32_t,
    pub forest_: *mut uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct H2 {
    pub common: *mut HasherCommon,
    pub buckets_: *mut uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct H3 {
    pub common: *mut HasherCommon,
    pub buckets_: *mut uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct H4 {
    pub common: *mut HasherCommon,
    pub buckets_: *mut uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct H5 {
    pub bucket_size_: size_t,
    pub block_size_: size_t,
    pub hash_shift_: ::core::ffi::c_int,
    pub block_mask_: uint32_t,
    pub block_bits_: ::core::ffi::c_int,
    pub num_last_distances_to_check_: ::core::ffi::c_int,
    pub common_: *mut HasherCommon,
    pub num_: *mut uint16_t,
    pub buckets_: *mut uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct H6 {
    pub bucket_size_: size_t,
    pub block_size_: size_t,
    pub hash_mul_: uint64_t,
    pub block_mask_: uint32_t,
    pub block_bits_: ::core::ffi::c_int,
    pub num_last_distances_to_check_: ::core::ffi::c_int,
    pub common_: *mut HasherCommon,
    pub num_: *mut uint16_t,
    pub buckets_: *mut uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct H58 {
    pub bucket_size_: size_t,
    pub block_size_: size_t,
    pub hash_shift_: ::core::ffi::c_int,
    pub block_mask_: uint32_t,
    pub block_bits_: ::core::ffi::c_int,
    pub num_last_distances_to_check_: ::core::ffi::c_int,
    pub common_: *mut HasherCommon,
    pub num_: *mut uint16_t,
    pub tags_: *mut uint8_t,
    pub buckets_: *mut uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct H68 {
    pub bucket_size_: size_t,
    pub block_size_: size_t,
    pub hash_mul_: uint64_t,
    pub block_mask_: uint32_t,
    pub block_bits_: ::core::ffi::c_int,
    pub num_last_distances_to_check_: ::core::ffi::c_int,
    pub common_: *mut HasherCommon,
    pub num_: *mut uint16_t,
    pub tags_: *mut uint8_t,
    pub buckets_: *mut uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct H40 {
    pub free_slot_idx: [uint16_t; 1],
    pub max_hops: size_t,
    pub extra: [*mut ::core::ffi::c_void; 2],
    pub common: *mut HasherCommon,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct H41 {
    pub free_slot_idx: [uint16_t; 1],
    pub max_hops: size_t,
    pub extra: [*mut ::core::ffi::c_void; 2],
    pub common: *mut HasherCommon,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct H42 {
    pub free_slot_idx: [uint16_t; 512],
    pub max_hops: size_t,
    pub extra: [*mut ::core::ffi::c_void; 2],
    pub common: *mut HasherCommon,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct H54 {
    pub common: *mut HasherCommon,
    pub buckets_: *mut uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HROLLING_FAST {
    pub state: uint32_t,
    pub table: *mut uint32_t,
    pub next_ix: size_t,
    pub chunk_len: uint32_t,
    pub factor: uint32_t,
    pub factor_remove: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HROLLING {
    pub state: uint32_t,
    pub table: *mut uint32_t,
    pub next_ix: size_t,
    pub chunk_len: uint32_t,
    pub factor: uint32_t,
    pub factor_remove: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct H35 {
    pub ha: H3,
    pub hb: HROLLING_FAST,
    pub ha_common: HasherCommon,
    pub hb_common: HasherCommon,
    pub common: *mut HasherCommon,
    pub fresh: ::core::ffi::c_int,
    pub params: *const BrotliEncoderParams,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct H55 {
    pub ha: H54,
    pub hb: HROLLING_FAST,
    pub ha_common: HasherCommon,
    pub hb_common: HasherCommon,
    pub common: *mut HasherCommon,
    pub fresh: ::core::ffi::c_int,
    pub params: *const BrotliEncoderParams,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct H65 {
    pub ha: H6,
    pub hb: HROLLING,
    pub ha_common: HasherCommon,
    pub hb_common: HasherCommon,
    pub common: *mut HasherCommon,
    pub fresh: ::core::ffi::c_int,
    pub params: *const BrotliEncoderParams,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Hasher {
    pub common: HasherCommon,
    pub privat: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub _H2: H2,
    pub _H3: H3,
    pub _H4: H4,
    pub _H5: H5,
    pub _H6: H6,
    pub _H40: H40,
    pub _H41: H41,
    pub _H42: H42,
    pub _H54: H54,
    pub _H58: H58,
    pub _H68: H68,
    pub _H35: H35,
    pub _H55: H55,
    pub _H65: H65,
    pub _H10: H10,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZopfliNode {
    pub length: uint32_t,
    pub distance: uint32_t,
    pub dcode_insert_length: uint32_t,
    pub u: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub cost: ::core::ffi::c_float,
    pub next: uint32_t,
    pub shortcut: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZopfliCostModel {
    pub cost_cmd_: [::core::ffi::c_float; 704],
    pub cost_dist_: *mut ::core::ffi::c_float,
    pub distance_histogram_size: uint32_t,
    pub literal_costs_: *mut ::core::ffi::c_float,
    pub min_cost_cmd_: ::core::ffi::c_float,
    pub num_bytes_: size_t,
    pub c2rust_unnamed: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub literal_histograms: [size_t; 768],
    pub arena: ZopfliCostModelArena,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZopfliCostModelArena {
    pub histogram_literal: [uint32_t; 256],
    pub histogram_cmd: [uint32_t; 704],
    pub histogram_dist: [uint32_t; 544],
    pub cost_literal: [::core::ffi::c_float; 256],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct StartPosQueue {
    pub q_: [PosData; 8],
    pub idx_: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PosData {
    pub pos: size_t,
    pub distance_cache: [::core::ffi::c_int; 4],
    pub costdiff: ::core::ffi::c_float,
    pub cost: ::core::ffi::c_float,
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const BROTLI_TRUE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const BROTLI_FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const BROTLI_UINT32_MAX: uint32_t = !(0 as ::core::ffi::c_int as uint32_t);
#[inline(always)]
unsafe extern "C" fn BrotliUnalignedRead32(mut p: *const ::core::ffi::c_void) -> uint32_t {
    let mut t: uint32_t = 0;
    memcpy(
        &raw mut t as *mut ::core::ffi::c_void,
        p,
        ::core::mem::size_of::<uint32_t>() as size_t,
    );
    return t;
}
#[inline(always)]
unsafe extern "C" fn BrotliUnalignedRead64(mut p: *const ::core::ffi::c_void) -> uint64_t {
    let mut t: uint64_t = 0;
    memcpy(
        &raw mut t as *mut ::core::ffi::c_void,
        p,
        ::core::mem::size_of::<uint64_t>() as size_t,
    );
    return t;
}
#[inline(always)]
unsafe extern "C" fn BROTLI_UNALIGNED_LOAD_PTR(
    mut p: *const ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    let mut v: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    memcpy(
        &raw mut v as *mut ::core::ffi::c_void,
        p,
        ::core::mem::size_of::<*mut ::core::ffi::c_void>() as size_t,
    );
    return v;
}
#[inline(always)]
unsafe extern "C" fn brotli_min_float(
    mut a: ::core::ffi::c_float,
    mut b: ::core::ffi::c_float,
) -> ::core::ffi::c_float {
    return if a < b { a } else { b };
}
#[inline(always)]
unsafe extern "C" fn brotli_max_size_t(mut a: size_t, mut b: size_t) -> size_t {
    return if a > b { a } else { b };
}
#[inline(always)]
unsafe extern "C" fn brotli_min_size_t(mut a: size_t, mut b: size_t) -> size_t {
    return if a < b { a } else { b };
}
pub const BROTLI_NUM_LITERAL_SYMBOLS: ::core::ffi::c_int = 256 as ::core::ffi::c_int;
pub const BROTLI_NUM_COMMAND_SYMBOLS: ::core::ffi::c_int = 704 as ::core::ffi::c_int;
pub const BROTLI_NUM_DISTANCE_SHORT_CODES: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const BROTLI_WINDOW_GAP: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
#[inline(always)]
unsafe extern "C" fn Log2FloorNonZero(mut n: size_t) -> uint32_t {
    return 31 as uint32_t ^ (n as uint32_t).leading_zeros() as i32 as uint32_t;
}
pub const BROTLI_LOG2_TABLE_SIZE: ::core::ffi::c_int = 256 as ::core::ffi::c_int;
#[inline(always)]
unsafe extern "C" fn FastLog2(mut v: size_t) -> ::core::ffi::c_double {
    if v < BROTLI_LOG2_TABLE_SIZE as size_t {
        return kBrotliLog2Table[v as usize];
    }
    return log2(v as ::core::ffi::c_double);
}
static mut kPreparedDictionaryMagic: uint32_t = 0xdebcede0 as uint32_t;
static mut kPreparedDictionaryHashMul64Long: uint64_t =
    (0x1fe35a7b as ::core::ffi::c_uint as uint64_t) << 32 as ::core::ffi::c_int
        | 0xd3579bd3 as uint64_t;
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
unsafe extern "C" fn CombineLengthCodes(
    mut inscode: uint16_t,
    mut copycode: uint16_t,
    mut use_last_distance: ::core::ffi::c_int,
) -> uint16_t {
    let mut bits64: uint16_t = (copycode as ::core::ffi::c_uint & 0x7 as ::core::ffi::c_uint
        | (inscode as ::core::ffi::c_uint & 0x7 as ::core::ffi::c_uint) << 3 as ::core::ffi::c_uint)
        as uint16_t;
    if use_last_distance != 0
        && (inscode as ::core::ffi::c_uint) < 8 as ::core::ffi::c_uint
        && (copycode as ::core::ffi::c_uint) < 16 as ::core::ffi::c_uint
    {
        return (if (copycode as ::core::ffi::c_uint) < 8 as ::core::ffi::c_uint {
            bits64 as ::core::ffi::c_uint
        } else {
            bits64 as ::core::ffi::c_uint | 64 as ::core::ffi::c_uint
        }) as uint16_t;
    } else {
        let mut offset: uint32_t = (2 as uint32_t).wrapping_mul(
            ((copycode as ::core::ffi::c_int >> 3 as ::core::ffi::c_uint) as uint32_t)
                .wrapping_add((3 as uint32_t).wrapping_mul(
                    (inscode as ::core::ffi::c_int >> 3 as ::core::ffi::c_uint) as uint32_t,
                )),
        );
        offset = (offset << 5 as ::core::ffi::c_uint)
            .wrapping_add(0x40 as uint32_t)
            .wrapping_add(0x520d40 as uint32_t >> offset & 0xc0 as uint32_t);
        return (offset | bits64 as uint32_t) as uint16_t;
    };
}
#[inline(always)]
unsafe extern "C" fn GetLengthCode(
    mut insertlen: size_t,
    mut copylen: size_t,
    mut use_last_distance: ::core::ffi::c_int,
    mut code: *mut uint16_t,
) {
    let mut inscode: uint16_t = GetInsertLengthCode(insertlen);
    let mut copycode: uint16_t = GetCopyLengthCode(copylen);
    *code = CombineLengthCodes(inscode, copycode, use_last_distance);
}
#[inline(always)]
unsafe extern "C" fn GetInsertExtra(mut inscode: uint16_t) -> uint32_t {
    return kBrotliInsExtra[inscode as usize];
}
#[inline(always)]
unsafe extern "C" fn GetCopyExtra(mut copycode: uint16_t) -> uint32_t {
    return kBrotliCopyExtra[copycode as usize];
}
#[inline(always)]
unsafe extern "C" fn InitCommand(
    mut self_0: *mut Command,
    mut dist: *const BrotliDistanceParams,
    mut insertlen: size_t,
    mut copylen: size_t,
    mut copylen_code_delta: ::core::ffi::c_int,
    mut distance_code: size_t,
) {
    let mut delta: uint32_t = copylen_code_delta as int8_t as uint8_t as uint32_t;
    (*self_0).insert_len_ = insertlen as uint32_t;
    (*self_0).copy_len_ = (copylen | (delta << 25 as ::core::ffi::c_int) as size_t) as uint32_t;
    PrefixEncodeCopyDistance(
        distance_code,
        (*dist).num_direct_distance_codes as size_t,
        (*dist).distance_postfix_bits as size_t,
        &raw mut (*self_0).dist_prefix_,
        &raw mut (*self_0).dist_extra_,
    );
    GetLengthCode(
        insertlen,
        (copylen as ::core::ffi::c_int + copylen_code_delta) as size_t,
        if (*self_0).dist_prefix_ as ::core::ffi::c_int & 0x3ff as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        {
            BROTLI_TRUE
        } else {
            BROTLI_FALSE
        },
        &raw mut (*self_0).cmd_prefix_,
    );
}
#[inline(always)]
unsafe extern "C" fn CommandCopyLen(mut self_0: *const Command) -> uint32_t {
    return (*self_0).copy_len_ & 0x1ffffff as uint32_t;
}
#[inline(always)]
unsafe extern "C" fn FindMatchLengthWithLimit(
    mut s1: *const uint8_t,
    mut s2: *const uint8_t,
    mut limit: size_t,
) -> size_t {
    let mut s1_orig: *const uint8_t = s1;
    while limit >= 8 as size_t {
        let mut x: uint64_t = BrotliUnalignedRead64(s2 as *const ::core::ffi::c_void)
            ^ BrotliUnalignedRead64(s1 as *const ::core::ffi::c_void);
        s2 = s2.offset(8 as ::core::ffi::c_int as isize);
        if x != 0 as uint64_t {
            let mut matching_bits: size_t =
                (x as ::core::ffi::c_ulonglong).trailing_zeros() as i32 as size_t;
            return (s1.offset_from(s1_orig) as ::core::ffi::c_long as size_t)
                .wrapping_add(matching_bits >> 3 as ::core::ffi::c_int);
        }
        s1 = s1.offset(8 as ::core::ffi::c_int as isize);
        limit = (limit as ::core::ffi::c_ulong).wrapping_sub(8 as ::core::ffi::c_ulong) as size_t
            as size_t;
    }
    while limit != 0 && *s1 as ::core::ffi::c_int == *s2 as ::core::ffi::c_int {
        limit = limit.wrapping_sub(1);
        s2 = s2.offset(1);
        s1 = s1.offset(1);
    }
    return s1.offset_from(s1_orig) as ::core::ffi::c_long as size_t;
}
static mut kHashMul32: uint32_t = 0x1e35a7bd as uint32_t;
pub const HQ_ZOPFLIFICATION_QUALITY: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const MAX_ZOPFLI_LEN_QUALITY_10: ::core::ffi::c_int = 150 as ::core::ffi::c_int;
pub const MAX_ZOPFLI_LEN_QUALITY_11: ::core::ffi::c_int = 325 as ::core::ffi::c_int;
pub const BROTLI_LONG_COPY_QUICK_STEP: ::core::ffi::c_int = 16384 as ::core::ffi::c_int;
#[inline(always)]
unsafe extern "C" fn MaxZopfliLen(mut params: *const BrotliEncoderParams) -> size_t {
    return (if (*params).quality <= 10 as ::core::ffi::c_int {
        MAX_ZOPFLI_LEN_QUALITY_10
    } else {
        MAX_ZOPFLI_LEN_QUALITY_11
    }) as size_t;
}
#[inline(always)]
unsafe extern "C" fn MaxZopfliCandidates(mut params: *const BrotliEncoderParams) -> size_t {
    return (if (*params).quality <= 10 as ::core::ffi::c_int {
        1 as ::core::ffi::c_int
    } else {
        5 as ::core::ffi::c_int
    }) as size_t;
}
pub const BROTLI_MAX_STATIC_DICTIONARY_MATCH_LEN: ::core::ffi::c_int = 37 as ::core::ffi::c_int;
static mut kInvalidMatch: uint32_t = 0xfffffff as uint32_t;
#[inline(always)]
unsafe extern "C" fn InitBackwardMatch(
    mut self_0: *mut BackwardMatch,
    mut dist: size_t,
    mut len: size_t,
) {
    (*self_0).distance = dist as uint32_t;
    (*self_0).length_and_code = (len << 5 as ::core::ffi::c_int) as uint32_t;
}
#[inline(always)]
unsafe extern "C" fn InitDictionaryBackwardMatch(
    mut self_0: *mut BackwardMatch,
    mut dist: size_t,
    mut len: size_t,
    mut len_code: size_t,
) {
    (*self_0).distance = dist as uint32_t;
    (*self_0).length_and_code = (len << 5 as ::core::ffi::c_int
        | (if len == len_code {
            0 as size_t
        } else {
            len_code
        })) as uint32_t;
}
#[inline(always)]
unsafe extern "C" fn BackwardMatchLength(mut self_0: *const BackwardMatch) -> size_t {
    return ((*self_0).length_and_code >> 5 as ::core::ffi::c_int) as size_t;
}
#[inline(always)]
unsafe extern "C" fn BackwardMatchLengthCode(mut self_0: *const BackwardMatch) -> size_t {
    let mut code: size_t = ((*self_0).length_and_code & 31 as uint32_t) as size_t;
    return if code != 0 {
        code
    } else {
        BackwardMatchLength(self_0)
    };
}
pub const BUCKET_BITS: ::core::ffi::c_int = 17 as ::core::ffi::c_int;
pub const MAX_TREE_SEARCH_DEPTH: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
pub const MAX_TREE_COMP_LENGTH: ::core::ffi::c_int = 128 as ::core::ffi::c_int;
pub const MAX_NUM_MATCHES_H10: ::core::ffi::c_int = 128 as ::core::ffi::c_int;
#[inline(always)]
unsafe extern "C" fn FindAllCompoundDictionaryMatches(
    mut self_0: *const PreparedDictionary,
    mut data: *const uint8_t,
    ring_buffer_mask: size_t,
    cur_ix: size_t,
    min_length: size_t,
    max_length: size_t,
    distance_offset: size_t,
    max_distance: size_t,
    mut matches: *mut BackwardMatch,
    mut match_limit: size_t,
) -> size_t {
    let source_size: uint32_t = (*self_0).source_size;
    let hash_bits: uint32_t = (*self_0).hash_bits;
    let bucket_bits: uint32_t = (*self_0).bucket_bits;
    let slot_bits: uint32_t = (*self_0).slot_bits;
    let hash_shift: uint32_t = (64 as uint32_t).wrapping_sub(bucket_bits);
    let slot_mask: uint32_t =
        !(0 as ::core::ffi::c_uint as uint32_t) >> (32 as uint32_t).wrapping_sub(slot_bits);
    let hash_mask: uint64_t =
        !(0 as ::core::ffi::c_uint as uint64_t) >> (64 as uint32_t).wrapping_sub(hash_bits);
    let mut slot_offsets: *const uint32_t = self_0.offset(1 as ::core::ffi::c_int as isize)
        as *const PreparedDictionary as *mut uint32_t;
    let mut heads: *const uint16_t = slot_offsets
        .offset(((1 as ::core::ffi::c_uint as size_t) << slot_bits) as isize)
        as *const uint32_t as *mut uint16_t;
    let mut items: *const uint32_t = heads
        .offset(((1 as ::core::ffi::c_uint as size_t) << bucket_bits) as isize)
        as *const uint16_t as *mut uint32_t;
    let mut source: *const uint8_t = ::core::ptr::null::<uint8_t>();
    let cur_ix_masked: size_t = cur_ix & ring_buffer_mask;
    let mut best_len: size_t = min_length;
    let h: uint64_t = (BrotliUnalignedRead64(
        data.offset(cur_ix_masked as isize) as *const uint8_t as *const ::core::ffi::c_void
    ) as uint64_t
        & hash_mask)
        .wrapping_mul(kPreparedDictionaryHashMul64Long);
    let key: uint32_t = (h >> hash_shift) as uint32_t;
    let slot: uint32_t = key & slot_mask;
    let head: uint32_t = *heads.offset(key as isize) as uint32_t;
    let mut chain: *const uint32_t = items
        .offset((*slot_offsets.offset(slot as isize)).wrapping_add(head) as isize)
        as *const uint32_t;
    let mut item: uint32_t = (if head == 0xffff as uint32_t {
        1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    }) as uint32_t;
    let mut found: size_t = 0 as size_t;
    let mut tail: *const ::core::ffi::c_void =
        items.offset((*self_0).num_items as isize) as *const uint32_t as *mut ::core::ffi::c_void;
    if (*self_0).magic == kPreparedDictionaryMagic {
        source = tail as *const uint8_t;
    } else {
        source =
            BROTLI_UNALIGNED_LOAD_PTR(tail as *mut *const uint8_t as *const ::core::ffi::c_void)
                as *const uint8_t;
    }
    while item == 0 as uint32_t {
        let mut offset: size_t = 0;
        let mut distance: size_t = 0;
        let mut limit: size_t = 0;
        let mut len: size_t = 0;
        item = *chain;
        chain = chain.offset(1);
        offset = (item & 0x7fffffff as uint32_t) as size_t;
        item = (item as ::core::ffi::c_uint & 0x80000000 as ::core::ffi::c_uint) as uint32_t;
        distance = distance_offset.wrapping_sub(offset);
        limit = (source_size as size_t).wrapping_sub(offset);
        limit = if limit > max_length {
            max_length
        } else {
            limit
        };
        if distance > max_distance {
            continue;
        }
        if cur_ix_masked.wrapping_add(best_len) > ring_buffer_mask
            || best_len >= limit
            || *data.offset(cur_ix_masked.wrapping_add(best_len) as isize) as ::core::ffi::c_int
                != *source.offset(offset.wrapping_add(best_len) as isize) as ::core::ffi::c_int
        {
            continue;
        }
        len = FindMatchLengthWithLimit(
            source.offset(offset as isize) as *const uint8_t,
            data.offset(cur_ix_masked as isize) as *const uint8_t,
            limit,
        );
        if !(len > best_len) {
            continue;
        }
        best_len = len;
        let fresh3 = matches;
        matches = matches.offset(1);
        InitBackwardMatch(fresh3, distance, len);
        found = found.wrapping_add(1);
        if found == match_limit {
            break;
        }
    }
    return found;
}
#[inline(always)]
unsafe extern "C" fn LookupAllCompoundDictionaryMatches(
    mut addon: *const CompoundDictionary,
    mut data: *const uint8_t,
    ring_buffer_mask: size_t,
    cur_ix: size_t,
    mut min_length: size_t,
    max_length: size_t,
    max_ring_buffer_distance: size_t,
    max_distance: size_t,
    mut matches: *mut BackwardMatch,
    mut match_limit: size_t,
) -> size_t {
    let mut base_offset: size_t = max_ring_buffer_distance
        .wrapping_add(1 as size_t)
        .wrapping_add((*addon).total_size)
        .wrapping_sub(1 as size_t);
    let mut d: size_t = 0;
    let mut total_found: size_t = 0 as size_t;
    d = 0 as size_t;
    while d < (*addon).num_chunks {
        total_found = (total_found as ::core::ffi::c_ulong).wrapping_add(
            FindAllCompoundDictionaryMatches(
                (*addon).chunks[d as usize],
                data,
                ring_buffer_mask,
                cur_ix,
                min_length,
                max_length,
                base_offset.wrapping_sub((*addon).chunk_offsets[d as usize]),
                max_distance,
                matches.offset(total_found as isize),
                match_limit.wrapping_sub(total_found),
            ) as ::core::ffi::c_ulong,
        ) as size_t as size_t;
        if total_found == match_limit {
            break;
        }
        if total_found > 0 as size_t {
            min_length = BackwardMatchLength(
                matches.offset(total_found.wrapping_sub(1 as size_t) as isize)
                    as *mut BackwardMatch,
            );
        }
        d = d.wrapping_add(1);
    }
    return total_found;
}
#[inline(always)]
unsafe extern "C" fn HashTypeLengthH10() -> size_t {
    return 4 as size_t;
}
#[inline(always)]
unsafe extern "C" fn StoreLookaheadH10() -> size_t {
    return MAX_TREE_COMP_LENGTH as size_t;
}
unsafe extern "C" fn HashBytesH10(mut data: *const uint8_t) -> uint32_t {
    let mut h: uint32_t =
        BrotliUnalignedRead32(data as *const ::core::ffi::c_void).wrapping_mul(kHashMul32);
    return h >> 32 as ::core::ffi::c_int - BUCKET_BITS;
}
#[inline(always)]
unsafe extern "C" fn LeftChildIndexH10(mut self_0: *mut H10, pos: size_t) -> size_t {
    return (2 as size_t).wrapping_mul(pos & (*self_0).window_mask_);
}
#[inline(always)]
unsafe extern "C" fn RightChildIndexH10(mut self_0: *mut H10, pos: size_t) -> size_t {
    return (2 as size_t)
        .wrapping_mul(pos & (*self_0).window_mask_)
        .wrapping_add(1 as size_t);
}
#[inline(always)]
unsafe extern "C" fn StoreAndFindMatchesH10(
    mut self_0: *mut H10,
    mut data: *const uint8_t,
    cur_ix: size_t,
    ring_buffer_mask: size_t,
    max_length: size_t,
    max_backward: size_t,
    best_len: *mut size_t,
    mut matches: *mut BackwardMatch,
) -> *mut BackwardMatch {
    let cur_ix_masked: size_t = cur_ix & ring_buffer_mask;
    let max_comp_len: size_t = brotli_min_size_t(max_length, 128 as size_t) as size_t;
    let should_reroot_tree: ::core::ffi::c_int = if max_length >= 128 as size_t {
        BROTLI_TRUE
    } else {
        BROTLI_FALSE
    };
    let key: uint32_t =
        HashBytesH10(data.offset(cur_ix_masked as isize) as *const uint8_t) as uint32_t;
    let mut buckets: *mut uint32_t = (*self_0).buckets_;
    let mut forest: *mut uint32_t = (*self_0).forest_;
    let mut prev_ix: size_t = *buckets.offset(key as isize) as size_t;
    let mut node_left: size_t = LeftChildIndexH10(self_0, cur_ix);
    let mut node_right: size_t = RightChildIndexH10(self_0, cur_ix);
    let mut best_len_left: size_t = 0 as size_t;
    let mut best_len_right: size_t = 0 as size_t;
    let mut depth_remaining: size_t = 0;
    if should_reroot_tree != 0 {
        *buckets.offset(key as isize) = cur_ix as uint32_t;
    }
    depth_remaining = MAX_TREE_SEARCH_DEPTH as size_t;
    loop {
        let backward: size_t = cur_ix.wrapping_sub(prev_ix);
        let prev_ix_masked: size_t = prev_ix & ring_buffer_mask;
        if backward == 0 as size_t || backward > max_backward || depth_remaining == 0 as size_t {
            if should_reroot_tree != 0 {
                *forest.offset(node_left as isize) = (*self_0).invalid_pos_;
                *forest.offset(node_right as isize) = (*self_0).invalid_pos_;
            }
            break;
        } else {
            let cur_len: size_t = brotli_min_size_t(best_len_left, best_len_right) as size_t;
            let mut len: size_t = 0;
            len = cur_len.wrapping_add(FindMatchLengthWithLimit(
                data.offset(cur_ix_masked.wrapping_add(cur_len) as isize) as *const uint8_t,
                data.offset(prev_ix_masked.wrapping_add(cur_len) as isize) as *const uint8_t,
                max_length.wrapping_sub(cur_len),
            ));
            if !matches.is_null() && len > *best_len {
                *best_len = len;
                let fresh0 = matches;
                matches = matches.offset(1);
                InitBackwardMatch(fresh0, backward, len);
            }
            if len >= max_comp_len {
                if should_reroot_tree != 0 {
                    *forest.offset(node_left as isize) =
                        *forest.offset(LeftChildIndexH10(self_0, prev_ix) as isize);
                    *forest.offset(node_right as isize) =
                        *forest.offset(RightChildIndexH10(self_0, prev_ix) as isize);
                }
                break;
            } else {
                if *data.offset(cur_ix_masked.wrapping_add(len) as isize) as ::core::ffi::c_int
                    > *data.offset(prev_ix_masked.wrapping_add(len) as isize) as ::core::ffi::c_int
                {
                    best_len_left = len;
                    if should_reroot_tree != 0 {
                        *forest.offset(node_left as isize) = prev_ix as uint32_t;
                    }
                    node_left = RightChildIndexH10(self_0, prev_ix);
                    prev_ix = *forest.offset(node_left as isize) as size_t;
                } else {
                    best_len_right = len;
                    if should_reroot_tree != 0 {
                        *forest.offset(node_right as isize) = prev_ix as uint32_t;
                    }
                    node_right = LeftChildIndexH10(self_0, prev_ix);
                    prev_ix = *forest.offset(node_right as isize) as size_t;
                }
                depth_remaining = depth_remaining.wrapping_sub(1);
            }
        }
    }
    return matches;
}
#[inline(always)]
unsafe extern "C" fn FindAllMatchesH10(
    mut self_0: *mut H10,
    mut dictionary: *const BrotliEncoderDictionary,
    mut data: *const uint8_t,
    ring_buffer_mask: size_t,
    cur_ix: size_t,
    max_length: size_t,
    max_backward: size_t,
    dictionary_distance: size_t,
    mut params: *const BrotliEncoderParams,
    mut matches: *mut BackwardMatch,
) -> size_t {
    let orig_matches: *mut BackwardMatch = matches;
    let cur_ix_masked: size_t = cur_ix & ring_buffer_mask;
    let mut best_len: size_t = 1 as size_t;
    let short_match_max_backward: size_t = (if (*params).quality != HQ_ZOPFLIFICATION_QUALITY {
        16 as ::core::ffi::c_int
    } else {
        64 as ::core::ffi::c_int
    }) as size_t;
    let mut stop: size_t = cur_ix.wrapping_sub(short_match_max_backward);
    let mut dict_matches: [uint32_t; 38] = [0; 38];
    let mut i: size_t = 0;
    if cur_ix < short_match_max_backward {
        stop = 0 as size_t;
    }
    i = cur_ix.wrapping_sub(1 as size_t);
    while i > stop && best_len <= 2 as size_t {
        let mut prev_ix: size_t = i;
        let backward: size_t = cur_ix.wrapping_sub(prev_ix);
        if (backward > max_backward) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
            break;
        }
        prev_ix =
            (prev_ix as ::core::ffi::c_ulong & ring_buffer_mask as ::core::ffi::c_ulong) as size_t;
        if !(*data.offset(cur_ix_masked as isize) as ::core::ffi::c_int
            != *data.offset(prev_ix as isize) as ::core::ffi::c_int
            || *data.offset(cur_ix_masked.wrapping_add(1 as size_t) as isize) as ::core::ffi::c_int
                != *data.offset(prev_ix.wrapping_add(1 as size_t) as isize) as ::core::ffi::c_int)
        {
            let len: size_t = FindMatchLengthWithLimit(
                data.offset(prev_ix as isize) as *const uint8_t,
                data.offset(cur_ix_masked as isize) as *const uint8_t,
                max_length,
            ) as size_t;
            if len > best_len {
                best_len = len;
                let fresh1 = matches;
                matches = matches.offset(1);
                InitBackwardMatch(fresh1, backward, len);
            }
        }
        i = i.wrapping_sub(1);
    }
    if best_len < max_length {
        matches = StoreAndFindMatchesH10(
            self_0,
            data,
            cur_ix,
            ring_buffer_mask,
            max_length,
            max_backward,
            &raw mut best_len,
            matches,
        );
    }
    i = 0 as size_t;
    while i <= BROTLI_MAX_STATIC_DICTIONARY_MATCH_LEN as size_t {
        dict_matches[i as usize] = kInvalidMatch;
        i = i.wrapping_add(1);
    }
    let mut minlen: size_t = brotli_max_size_t(4 as size_t, best_len.wrapping_add(1 as size_t));
    if BrotliFindAllStaticDictionaryMatches(
        dictionary,
        data.offset(cur_ix_masked as isize) as *const uint8_t,
        minlen,
        max_length,
        (&raw mut dict_matches as *mut uint32_t).offset(0 as ::core::ffi::c_int as isize)
            as *mut uint32_t,
    ) != 0
    {
        let mut maxlen: size_t = brotli_min_size_t(37 as size_t, max_length);
        let mut l: size_t = 0;
        l = minlen;
        while l <= maxlen {
            let mut dict_id: uint32_t = dict_matches[l as usize];
            if dict_id < kInvalidMatch {
                let mut distance: size_t = dictionary_distance
                    .wrapping_add((dict_id >> 5 as ::core::ffi::c_int) as size_t)
                    .wrapping_add(1 as size_t);
                if distance <= (*params).dist.max_distance {
                    let fresh2 = matches;
                    matches = matches.offset(1);
                    InitDictionaryBackwardMatch(
                        fresh2,
                        distance,
                        l,
                        (dict_id & 31 as uint32_t) as size_t,
                    );
                }
            }
            l = l.wrapping_add(1);
        }
    }
    return matches.offset_from(orig_matches) as ::core::ffi::c_long as size_t;
}
#[inline(always)]
unsafe extern "C" fn StoreH10(
    mut self_0: *mut H10,
    mut data: *const uint8_t,
    mask: size_t,
    ix: size_t,
) {
    let max_backward: size_t = (*self_0)
        .window_mask_
        .wrapping_sub(BROTLI_WINDOW_GAP as size_t)
        .wrapping_add(1 as size_t);
    StoreAndFindMatchesH10(
        self_0,
        data,
        ix,
        mask,
        MAX_TREE_COMP_LENGTH as size_t,
        max_backward,
        ::core::ptr::null_mut::<size_t>(),
        ::core::ptr::null_mut::<BackwardMatch>(),
    );
}
#[inline(always)]
unsafe extern "C" fn StoreRangeH10(
    mut self_0: *mut H10,
    mut data: *const uint8_t,
    mask: size_t,
    ix_start: size_t,
    ix_end: size_t,
) {
    let mut i: size_t = ix_start;
    let mut j: size_t = ix_start;
    if ix_start.wrapping_add(63 as size_t) <= ix_end {
        i = ix_end.wrapping_sub(63 as size_t);
    }
    if ix_start.wrapping_add(512 as size_t) <= i {
        while j < i {
            StoreH10(self_0, data, mask, j);
            j = (j as ::core::ffi::c_ulong).wrapping_add(8 as ::core::ffi::c_ulong) as size_t
                as size_t;
        }
    }
    while i < ix_end {
        StoreH10(self_0, data, mask, i);
        i = i.wrapping_add(1);
    }
}
static mut kInfinity: ::core::ffi::c_float = 1.7e38f32;
static mut kDistanceCacheIndex: [uint32_t; 16] = [
    0 as ::core::ffi::c_int as uint32_t,
    1 as ::core::ffi::c_int as uint32_t,
    2 as ::core::ffi::c_int as uint32_t,
    3 as ::core::ffi::c_int as uint32_t,
    0 as ::core::ffi::c_int as uint32_t,
    0 as ::core::ffi::c_int as uint32_t,
    0 as ::core::ffi::c_int as uint32_t,
    0 as ::core::ffi::c_int as uint32_t,
    0 as ::core::ffi::c_int as uint32_t,
    0 as ::core::ffi::c_int as uint32_t,
    1 as ::core::ffi::c_int as uint32_t,
    1 as ::core::ffi::c_int as uint32_t,
    1 as ::core::ffi::c_int as uint32_t,
    1 as ::core::ffi::c_int as uint32_t,
    1 as ::core::ffi::c_int as uint32_t,
    1 as ::core::ffi::c_int as uint32_t,
];
static mut kDistanceCacheOffset: [::core::ffi::c_int; 16] = [
    0 as ::core::ffi::c_int,
    0 as ::core::ffi::c_int,
    0 as ::core::ffi::c_int,
    0 as ::core::ffi::c_int,
    -(1 as ::core::ffi::c_int),
    1 as ::core::ffi::c_int,
    -(2 as ::core::ffi::c_int),
    2 as ::core::ffi::c_int,
    -(3 as ::core::ffi::c_int),
    3 as ::core::ffi::c_int,
    -(1 as ::core::ffi::c_int),
    1 as ::core::ffi::c_int,
    -(2 as ::core::ffi::c_int),
    2 as ::core::ffi::c_int,
    -(3 as ::core::ffi::c_int),
    3 as ::core::ffi::c_int,
];
#[no_mangle]
pub unsafe extern "C" fn BrotliInitZopfliNodes(mut array: *mut ZopfliNode, mut length: size_t) {
    let mut stub: ZopfliNode = ZopfliNode {
        length: 0,
        distance: 0,
        dcode_insert_length: 0,
        u: C2RustUnnamed_0 { cost: 0. },
    };
    let mut i: size_t = 0;
    stub.length = 1 as uint32_t;
    stub.distance = 0 as uint32_t;
    stub.dcode_insert_length = 0 as uint32_t;
    stub.u.cost = kInfinity;
    i = 0 as size_t;
    while i < length {
        *array.offset(i as isize) = stub;
        i = i.wrapping_add(1);
    }
}
#[inline(always)]
unsafe extern "C" fn ZopfliNodeCopyLength(mut self_0: *const ZopfliNode) -> uint32_t {
    return (*self_0).length & 0x1ffffff as uint32_t;
}
#[inline(always)]
unsafe extern "C" fn ZopfliNodeLengthCode(mut self_0: *const ZopfliNode) -> uint32_t {
    let modifier: uint32_t = (*self_0).length >> 25 as ::core::ffi::c_int;
    return ZopfliNodeCopyLength(self_0)
        .wrapping_add(9 as uint32_t)
        .wrapping_sub(modifier);
}
#[inline(always)]
unsafe extern "C" fn ZopfliNodeCopyDistance(mut self_0: *const ZopfliNode) -> uint32_t {
    return (*self_0).distance;
}
#[inline(always)]
unsafe extern "C" fn ZopfliNodeDistanceCode(mut self_0: *const ZopfliNode) -> uint32_t {
    let short_code: uint32_t = (*self_0).dcode_insert_length >> 27 as ::core::ffi::c_int;
    return if short_code == 0 as uint32_t {
        ZopfliNodeCopyDistance(self_0)
            .wrapping_add(BROTLI_NUM_DISTANCE_SHORT_CODES as uint32_t)
            .wrapping_sub(1 as uint32_t)
    } else {
        short_code.wrapping_sub(1 as uint32_t)
    };
}
#[inline(always)]
unsafe extern "C" fn ZopfliNodeCommandLength(mut self_0: *const ZopfliNode) -> uint32_t {
    return ZopfliNodeCopyLength(self_0)
        .wrapping_add((*self_0).dcode_insert_length & 0x7ffffff as uint32_t);
}
unsafe extern "C" fn InitZopfliCostModel(
    mut m: *mut MemoryManager,
    mut self_0: *mut ZopfliCostModel,
    mut dist: *const BrotliDistanceParams,
    mut num_bytes: size_t,
) {
    (*self_0).num_bytes_ = num_bytes;
    (*self_0).literal_costs_ = if num_bytes.wrapping_add(2 as size_t) > 0 as size_t {
        BrotliAllocate(
            m,
            num_bytes
                .wrapping_add(2 as size_t)
                .wrapping_mul(::core::mem::size_of::<::core::ffi::c_float>() as size_t),
        ) as *mut ::core::ffi::c_float
    } else {
        ::core::ptr::null_mut::<::core::ffi::c_float>()
    };
    (*self_0).cost_dist_ = if (*dist).alphabet_size_limit > 0 as uint32_t {
        BrotliAllocate(
            m,
            ((*dist).alphabet_size_limit as size_t)
                .wrapping_mul(::core::mem::size_of::<::core::ffi::c_float>() as size_t),
        ) as *mut ::core::ffi::c_float
    } else {
        ::core::ptr::null_mut::<::core::ffi::c_float>()
    };
    (*self_0).distance_histogram_size = (*dist).alphabet_size_limit;
    if 0 as ::core::ffi::c_int != 0 {
        return;
    }
}
unsafe extern "C" fn CleanupZopfliCostModel(
    mut m: *mut MemoryManager,
    mut self_0: *mut ZopfliCostModel,
) {
    BrotliFree(m, (*self_0).literal_costs_ as *mut ::core::ffi::c_void);
    (*self_0).literal_costs_ = ::core::ptr::null_mut::<::core::ffi::c_float>();
    BrotliFree(m, (*self_0).cost_dist_ as *mut ::core::ffi::c_void);
    (*self_0).cost_dist_ = ::core::ptr::null_mut::<::core::ffi::c_float>();
}
unsafe extern "C" fn SetCost(
    mut histogram: *const uint32_t,
    mut histogram_size: size_t,
    mut literal_histogram: ::core::ffi::c_int,
    mut cost: *mut ::core::ffi::c_float,
) {
    let mut sum: size_t = 0 as size_t;
    let mut missing_symbol_sum: size_t = 0;
    let mut log2sum: ::core::ffi::c_float = 0.;
    let mut missing_symbol_cost: ::core::ffi::c_float = 0.;
    let mut i: size_t = 0;
    i = 0 as size_t;
    while i < histogram_size {
        sum = (sum as ::core::ffi::c_ulong)
            .wrapping_add(*histogram.offset(i as isize) as ::core::ffi::c_ulong)
            as size_t as size_t;
        i = i.wrapping_add(1);
    }
    log2sum = FastLog2(sum) as ::core::ffi::c_float;
    missing_symbol_sum = sum;
    if literal_histogram == 0 {
        i = 0 as size_t;
        while i < histogram_size {
            if *histogram.offset(i as isize) == 0 as uint32_t {
                missing_symbol_sum = missing_symbol_sum.wrapping_add(1);
            }
            i = i.wrapping_add(1);
        }
    }
    missing_symbol_cost = FastLog2(missing_symbol_sum) as ::core::ffi::c_float
        + 2 as ::core::ffi::c_int as ::core::ffi::c_float;
    i = 0 as size_t;
    while i < histogram_size {
        if *histogram.offset(i as isize) == 0 as uint32_t {
            *cost.offset(i as isize) = missing_symbol_cost;
        } else {
            *cost.offset(i as isize) =
                log2sum - FastLog2(*histogram.offset(i as isize) as size_t) as ::core::ffi::c_float;
            if *cost.offset(i as isize) < 1 as ::core::ffi::c_int as ::core::ffi::c_float {
                *cost.offset(i as isize) = 1 as ::core::ffi::c_int as ::core::ffi::c_float;
            }
        }
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn ZopfliCostModelSetFromCommands(
    mut self_0: *mut ZopfliCostModel,
    mut position: size_t,
    mut ringbuffer: *const uint8_t,
    mut ringbuffer_mask: size_t,
    mut commands: *const Command,
    mut num_commands: size_t,
    mut last_insert_len: size_t,
) {
    let mut arena: *mut ZopfliCostModelArena = &raw mut (*self_0).c2rust_unnamed.arena;
    let mut pos: size_t = position.wrapping_sub(last_insert_len);
    let mut min_cost_cmd: ::core::ffi::c_float = kInfinity;
    let mut i: size_t = 0;
    let mut cost_cmd: *mut ::core::ffi::c_float =
        &raw mut (*self_0).cost_cmd_ as *mut ::core::ffi::c_float;
    memset(
        &raw mut (*arena).histogram_literal as *mut uint32_t as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<[uint32_t; 256]>() as size_t,
    );
    memset(
        &raw mut (*arena).histogram_cmd as *mut uint32_t as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<[uint32_t; 704]>() as size_t,
    );
    memset(
        &raw mut (*arena).histogram_dist as *mut uint32_t as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<[uint32_t; 544]>() as size_t,
    );
    i = 0 as size_t;
    while i < num_commands {
        let mut inslength: size_t = (*commands.offset(i as isize)).insert_len_ as size_t;
        let mut copylength: size_t =
            CommandCopyLen(commands.offset(i as isize) as *const Command) as size_t;
        let mut distcode: size_t = ((*commands.offset(i as isize)).dist_prefix_
            as ::core::ffi::c_int
            & 0x3ff as ::core::ffi::c_int) as size_t;
        let mut cmdcode: size_t = (*commands.offset(i as isize)).cmd_prefix_ as size_t;
        let mut j: size_t = 0;
        (*arena).histogram_cmd[cmdcode as usize] =
            (*arena).histogram_cmd[cmdcode as usize].wrapping_add(1);
        if cmdcode >= 128 as size_t {
            (*arena).histogram_dist[distcode as usize] =
                (*arena).histogram_dist[distcode as usize].wrapping_add(1);
        }
        j = 0 as size_t;
        while j < inslength {
            (*arena).histogram_literal
                [*ringbuffer.offset((pos.wrapping_add(j) & ringbuffer_mask) as isize) as usize] =
                (*arena).histogram_literal
                    [*ringbuffer.offset((pos.wrapping_add(j) & ringbuffer_mask) as isize) as usize]
                    .wrapping_add(1);
            j = j.wrapping_add(1);
        }
        pos = (pos as ::core::ffi::c_ulong)
            .wrapping_add(inslength.wrapping_add(copylength) as ::core::ffi::c_ulong)
            as size_t as size_t;
        i = i.wrapping_add(1);
    }
    SetCost(
        &raw mut (*arena).histogram_literal as *mut uint32_t,
        BROTLI_NUM_LITERAL_SYMBOLS as size_t,
        BROTLI_TRUE,
        &raw mut (*arena).cost_literal as *mut ::core::ffi::c_float,
    );
    SetCost(
        &raw mut (*arena).histogram_cmd as *mut uint32_t,
        BROTLI_NUM_COMMAND_SYMBOLS as size_t,
        BROTLI_FALSE,
        cost_cmd,
    );
    SetCost(
        &raw mut (*arena).histogram_dist as *mut uint32_t,
        (*self_0).distance_histogram_size as size_t,
        BROTLI_FALSE,
        (*self_0).cost_dist_,
    );
    i = 0 as size_t;
    while i < BROTLI_NUM_COMMAND_SYMBOLS as size_t {
        min_cost_cmd = brotli_min_float(min_cost_cmd, *cost_cmd.offset(i as isize));
        i = i.wrapping_add(1);
    }
    (*self_0).min_cost_cmd_ = min_cost_cmd;
    let mut literal_costs: *mut ::core::ffi::c_float = (*self_0).literal_costs_;
    let mut literal_carry: ::core::ffi::c_float = 0.0f32;
    let mut num_bytes: size_t = (*self_0).num_bytes_;
    *literal_costs.offset(0 as ::core::ffi::c_int as isize) = 0.0f32;
    i = 0 as size_t;
    while i < num_bytes {
        literal_carry += (*arena).cost_literal
            [*ringbuffer.offset((position.wrapping_add(i) & ringbuffer_mask) as isize) as usize];
        *literal_costs.offset(i.wrapping_add(1 as size_t) as isize) =
            *literal_costs.offset(i as isize) + literal_carry;
        literal_carry -= *literal_costs.offset(i.wrapping_add(1 as size_t) as isize)
            - *literal_costs.offset(i as isize);
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn ZopfliCostModelSetFromLiteralCosts(
    mut self_0: *mut ZopfliCostModel,
    mut position: size_t,
    mut ringbuffer: *const uint8_t,
    mut ringbuffer_mask: size_t,
) {
    let mut literal_costs: *mut ::core::ffi::c_float = (*self_0).literal_costs_;
    let mut literal_carry: ::core::ffi::c_float = 0.0f32;
    let mut cost_dist: *mut ::core::ffi::c_float = (*self_0).cost_dist_;
    let mut cost_cmd: *mut ::core::ffi::c_float =
        &raw mut (*self_0).cost_cmd_ as *mut ::core::ffi::c_float;
    let mut num_bytes: size_t = (*self_0).num_bytes_;
    let mut i: size_t = 0;
    BrotliEstimateBitCostsForLiterals(
        position,
        num_bytes,
        ringbuffer_mask,
        ringbuffer,
        &raw mut (*self_0).c2rust_unnamed.literal_histograms as *mut size_t,
        literal_costs.offset(1 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_float,
    );
    *literal_costs.offset(0 as ::core::ffi::c_int as isize) = 0.0f32;
    i = 0 as size_t;
    while i < num_bytes {
        literal_carry += *literal_costs.offset(i.wrapping_add(1 as size_t) as isize);
        *literal_costs.offset(i.wrapping_add(1 as size_t) as isize) =
            *literal_costs.offset(i as isize) + literal_carry;
        literal_carry -= *literal_costs.offset(i.wrapping_add(1 as size_t) as isize)
            - *literal_costs.offset(i as isize);
        i = i.wrapping_add(1);
    }
    i = 0 as size_t;
    while i < BROTLI_NUM_COMMAND_SYMBOLS as size_t {
        *cost_cmd.offset(i as isize) =
            FastLog2((11 as uint32_t).wrapping_add(i as uint32_t) as size_t)
                as ::core::ffi::c_float;
        i = i.wrapping_add(1);
    }
    i = 0 as size_t;
    while i < (*self_0).distance_histogram_size as size_t {
        *cost_dist.offset(i as isize) =
            FastLog2((20 as uint32_t).wrapping_add(i as uint32_t) as size_t)
                as ::core::ffi::c_float;
        i = i.wrapping_add(1);
    }
    (*self_0).min_cost_cmd_ = FastLog2(11 as size_t) as ::core::ffi::c_float;
}
#[inline(always)]
unsafe extern "C" fn ZopfliCostModelGetCommandCost(
    mut self_0: *const ZopfliCostModel,
    mut cmdcode: uint16_t,
) -> ::core::ffi::c_float {
    return (*self_0).cost_cmd_[cmdcode as usize];
}
#[inline(always)]
unsafe extern "C" fn ZopfliCostModelGetDistanceCost(
    mut self_0: *const ZopfliCostModel,
    mut distcode: size_t,
) -> ::core::ffi::c_float {
    return *(*self_0).cost_dist_.offset(distcode as isize);
}
#[inline(always)]
unsafe extern "C" fn ZopfliCostModelGetLiteralCosts(
    mut self_0: *const ZopfliCostModel,
    mut from: size_t,
    mut to: size_t,
) -> ::core::ffi::c_float {
    return *(*self_0).literal_costs_.offset(to as isize)
        - *(*self_0).literal_costs_.offset(from as isize);
}
#[inline(always)]
unsafe extern "C" fn ZopfliCostModelGetMinCostCmd(
    mut self_0: *const ZopfliCostModel,
) -> ::core::ffi::c_float {
    return (*self_0).min_cost_cmd_;
}
#[inline(always)]
unsafe extern "C" fn UpdateZopfliNode(
    mut nodes: *mut ZopfliNode,
    mut pos: size_t,
    mut start_pos: size_t,
    mut len: size_t,
    mut len_code: size_t,
    mut dist: size_t,
    mut short_code: size_t,
    mut cost: ::core::ffi::c_float,
) {
    let mut next: *mut ZopfliNode = nodes.offset(pos.wrapping_add(len) as isize) as *mut ZopfliNode;
    (*next).length = (len
        | len.wrapping_add(9 as size_t).wrapping_sub(len_code) << 25 as ::core::ffi::c_int)
        as uint32_t;
    (*next).distance = dist as uint32_t;
    (*next).dcode_insert_length =
        (short_code << 27 as ::core::ffi::c_int | pos.wrapping_sub(start_pos)) as uint32_t;
    (*next).u.cost = cost;
}
#[inline(always)]
unsafe extern "C" fn InitStartPosQueue(mut self_0: *mut StartPosQueue) {
    (*self_0).idx_ = 0 as size_t;
}
unsafe extern "C" fn StartPosQueueSize(mut self_0: *const StartPosQueue) -> size_t {
    return brotli_min_size_t((*self_0).idx_, 8 as size_t);
}
unsafe extern "C" fn StartPosQueuePush(
    mut self_0: *mut StartPosQueue,
    mut posdata: *const PosData,
) {
    let fresh4 = (*self_0).idx_;
    (*self_0).idx_ = (*self_0).idx_.wrapping_add(1);
    let mut offset: size_t = !fresh4 & 7 as size_t;
    let mut len: size_t = StartPosQueueSize(self_0);
    let mut i: size_t = 0;
    let mut q: *mut PosData = &raw mut (*self_0).q_ as *mut PosData;
    *q.offset(offset as isize) = *posdata;
    i = 1 as size_t;
    while i < len {
        if (*q.offset((offset & 7 as size_t) as isize)).costdiff
            > (*q.offset((offset.wrapping_add(1 as size_t) & 7 as size_t) as isize)).costdiff
        {
            let mut __brotli_swap_tmp: PosData = *q.offset((offset & 7 as size_t) as isize);
            *q.offset((offset & 7 as size_t) as isize) =
                *q.offset((offset.wrapping_add(1 as size_t) & 7 as size_t) as isize);
            *q.offset((offset.wrapping_add(1 as size_t) & 7 as size_t) as isize) =
                __brotli_swap_tmp;
        }
        offset = offset.wrapping_add(1);
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn StartPosQueueAt(
    mut self_0: *const StartPosQueue,
    mut k: size_t,
) -> *const PosData {
    return (&raw const (*self_0).q_ as *const PosData)
        .offset((k.wrapping_sub((*self_0).idx_) & 7 as size_t) as isize)
        as *const PosData;
}
unsafe extern "C" fn ComputeMinimumCopyLength(
    start_cost: ::core::ffi::c_float,
    mut nodes: *const ZopfliNode,
    num_bytes: size_t,
    pos: size_t,
) -> size_t {
    let mut min_cost: ::core::ffi::c_float = start_cost;
    let mut len: size_t = 2 as size_t;
    let mut next_len_bucket: size_t = 4 as size_t;
    let mut next_len_offset: size_t = 10 as size_t;
    while pos.wrapping_add(len) <= num_bytes
        && (*nodes.offset(pos.wrapping_add(len) as isize)).u.cost <= min_cost
    {
        len = len.wrapping_add(1);
        if len == next_len_offset {
            min_cost += 1.0f32;
            next_len_offset = (next_len_offset as ::core::ffi::c_ulong)
                .wrapping_add(next_len_bucket as ::core::ffi::c_ulong)
                as size_t as size_t;
            next_len_bucket = (next_len_bucket as ::core::ffi::c_ulong)
                .wrapping_mul(2 as ::core::ffi::c_ulong) as size_t
                as size_t;
        }
    }
    return len;
}
unsafe extern "C" fn ComputeDistanceShortcut(
    block_start: size_t,
    pos: size_t,
    max_backward_limit: size_t,
    gap: size_t,
    mut nodes: *const ZopfliNode,
) -> uint32_t {
    let c_len: size_t =
        ZopfliNodeCopyLength(nodes.offset(pos as isize) as *const ZopfliNode) as size_t;
    let i_len: size_t =
        ((*nodes.offset(pos as isize)).dcode_insert_length & 0x7ffffff as uint32_t) as size_t;
    let dist: size_t =
        ZopfliNodeCopyDistance(nodes.offset(pos as isize) as *const ZopfliNode) as size_t;
    if pos == 0 as size_t {
        return 0 as uint32_t;
    } else if dist.wrapping_add(c_len) <= block_start.wrapping_add(pos).wrapping_add(gap)
        && dist <= max_backward_limit.wrapping_add(gap)
        && ZopfliNodeDistanceCode(nodes.offset(pos as isize) as *const ZopfliNode) > 0 as uint32_t
    {
        return pos as uint32_t;
    } else {
        return (*nodes.offset(pos.wrapping_sub(c_len).wrapping_sub(i_len) as isize))
            .u
            .shortcut;
    };
}
unsafe extern "C" fn ComputeDistanceCache(
    pos: size_t,
    mut starting_dist_cache: *const ::core::ffi::c_int,
    mut nodes: *const ZopfliNode,
    mut dist_cache: *mut ::core::ffi::c_int,
) {
    let mut idx: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut p: size_t = (*nodes.offset(pos as isize)).u.shortcut as size_t;
    while idx < 4 as ::core::ffi::c_int && p > 0 as size_t {
        let i_len: size_t =
            ((*nodes.offset(p as isize)).dcode_insert_length & 0x7ffffff as uint32_t) as size_t;
        let c_len: size_t =
            ZopfliNodeCopyLength(nodes.offset(p as isize) as *const ZopfliNode) as size_t;
        let dist: size_t =
            ZopfliNodeCopyDistance(nodes.offset(p as isize) as *const ZopfliNode) as size_t;
        let fresh5 = idx;
        idx = idx + 1;
        *dist_cache.offset(fresh5 as isize) = dist as ::core::ffi::c_int;
        p = (*nodes.offset(p.wrapping_sub(c_len).wrapping_sub(i_len) as isize))
            .u
            .shortcut as size_t;
    }
    while idx < 4 as ::core::ffi::c_int {
        let fresh6 = starting_dist_cache;
        starting_dist_cache = starting_dist_cache.offset(1);
        *dist_cache.offset(idx as isize) = *fresh6;
        idx += 1;
    }
}
unsafe extern "C" fn EvaluateNode(
    block_start: size_t,
    pos: size_t,
    max_backward_limit: size_t,
    gap: size_t,
    mut starting_dist_cache: *const ::core::ffi::c_int,
    mut model: *const ZopfliCostModel,
    mut queue: *mut StartPosQueue,
    mut nodes: *mut ZopfliNode,
) {
    let mut node_cost: ::core::ffi::c_float = (*nodes.offset(pos as isize)).u.cost;
    (*nodes.offset(pos as isize)).u.shortcut =
        ComputeDistanceShortcut(block_start, pos, max_backward_limit, gap, nodes);
    if node_cost <= ZopfliCostModelGetLiteralCosts(model, 0 as size_t, pos) {
        let mut posdata: PosData = PosData {
            pos: 0,
            distance_cache: [0; 4],
            costdiff: 0.,
            cost: 0.,
        };
        posdata.pos = pos;
        posdata.cost = node_cost;
        posdata.costdiff = node_cost - ZopfliCostModelGetLiteralCosts(model, 0 as size_t, pos);
        ComputeDistanceCache(
            pos,
            starting_dist_cache,
            nodes,
            &raw mut posdata.distance_cache as *mut ::core::ffi::c_int,
        );
        StartPosQueuePush(queue, &raw mut posdata);
    }
}
unsafe extern "C" fn UpdateNodes(
    num_bytes: size_t,
    block_start: size_t,
    pos: size_t,
    mut ringbuffer: *const uint8_t,
    ringbuffer_mask: size_t,
    mut params: *const BrotliEncoderParams,
    max_backward_limit: size_t,
    mut starting_dist_cache: *const ::core::ffi::c_int,
    num_matches: size_t,
    mut matches: *const BackwardMatch,
    mut model: *const ZopfliCostModel,
    mut queue: *mut StartPosQueue,
    mut nodes: *mut ZopfliNode,
) -> size_t {
    let stream_offset: size_t = (*params).stream_offset;
    let cur_ix: size_t = block_start.wrapping_add(pos);
    let cur_ix_masked: size_t = cur_ix & ringbuffer_mask;
    let max_distance: size_t = brotli_min_size_t(cur_ix, max_backward_limit) as size_t;
    let dictionary_start: size_t =
        brotli_min_size_t(cur_ix.wrapping_add(stream_offset), max_backward_limit) as size_t;
    let max_len: size_t = num_bytes.wrapping_sub(pos);
    let max_zopfli_len: size_t = MaxZopfliLen(params) as size_t;
    let max_iters: size_t = MaxZopfliCandidates(params) as size_t;
    let mut min_len: size_t = 0;
    let mut result: size_t = 0 as size_t;
    let mut k: size_t = 0;
    let mut addon: *const CompoundDictionary = &raw const (*params).dictionary.compound;
    let mut gap: size_t = (*addon).total_size;
    EvaluateNode(
        block_start.wrapping_add(stream_offset),
        pos,
        max_backward_limit,
        gap,
        starting_dist_cache,
        model,
        queue,
        nodes,
    );
    let mut posdata: *const PosData = StartPosQueueAt(queue, 0 as size_t);
    let mut min_cost: ::core::ffi::c_float = (*posdata).cost
        + ZopfliCostModelGetMinCostCmd(model)
        + ZopfliCostModelGetLiteralCosts(model, (*posdata).pos, pos);
    min_len = ComputeMinimumCopyLength(min_cost, nodes, num_bytes, pos);
    k = 0 as size_t;
    while k < max_iters && k < StartPosQueueSize(queue) {
        let mut posdata_0: *const PosData = StartPosQueueAt(queue, k);
        let start: size_t = (*posdata_0).pos;
        let inscode: uint16_t = GetInsertLengthCode(pos.wrapping_sub(start)) as uint16_t;
        let start_costdiff: ::core::ffi::c_float = (*posdata_0).costdiff;
        let base_cost: ::core::ffi::c_float = start_costdiff
            + GetInsertExtra(inscode) as ::core::ffi::c_float
            + ZopfliCostModelGetLiteralCosts(model, 0 as size_t, pos) as ::core::ffi::c_float;
        let mut best_len: size_t = min_len.wrapping_sub(1 as size_t);
        let mut j: size_t = 0 as size_t;
        let mut current_block_23: u64;
        while j < BROTLI_NUM_DISTANCE_SHORT_CODES as size_t && best_len < max_len {
            let idx: size_t = kDistanceCacheIndex[j as usize] as size_t;
            let backward: size_t = ((*posdata_0).distance_cache[idx as usize]
                + kDistanceCacheOffset[j as usize]) as size_t;
            let mut prev_ix: size_t = cur_ix.wrapping_sub(backward);
            let mut len: size_t = 0 as size_t;
            let mut continuation: uint8_t =
                *ringbuffer.offset(cur_ix_masked.wrapping_add(best_len) as isize);
            if cur_ix_masked.wrapping_add(best_len) > ringbuffer_mask {
                break;
            }
            if !((backward > dictionary_start.wrapping_add(gap)) as ::core::ffi::c_int
                as ::core::ffi::c_long
                != 0)
            {
                if backward <= max_distance {
                    if prev_ix >= cur_ix {
                        current_block_23 = 4166486009154926805;
                    } else {
                        prev_ix = (prev_ix as ::core::ffi::c_ulong
                            & ringbuffer_mask as ::core::ffi::c_ulong)
                            as size_t;
                        if prev_ix.wrapping_add(best_len) > ringbuffer_mask
                            || continuation as ::core::ffi::c_int
                                != *ringbuffer.offset(prev_ix.wrapping_add(best_len) as isize)
                                    as ::core::ffi::c_int
                        {
                            current_block_23 = 4166486009154926805;
                        } else {
                            len = FindMatchLengthWithLimit(
                                ringbuffer.offset(prev_ix as isize) as *const uint8_t,
                                ringbuffer.offset(cur_ix_masked as isize) as *const uint8_t,
                                max_len,
                            );
                            current_block_23 = 2873832966593178012;
                        }
                    }
                } else if backward > dictionary_start {
                    let mut d: size_t = 0 as size_t;
                    let mut offset: size_t = 0;
                    let mut limit: size_t = 0;
                    let mut source: *const uint8_t = ::core::ptr::null::<uint8_t>();
                    offset = dictionary_start
                        .wrapping_add(1 as size_t)
                        .wrapping_add((*addon).total_size)
                        .wrapping_sub(1 as size_t);
                    while offset
                        >= backward.wrapping_add(
                            (*addon).chunk_offsets[d.wrapping_add(1 as size_t) as usize],
                        )
                    {
                        d = d.wrapping_add(1);
                    }
                    source = (*addon).chunk_source[d as usize];
                    offset = offset
                        .wrapping_sub((*addon).chunk_offsets[d as usize])
                        .wrapping_sub(backward);
                    limit = (*addon).chunk_offsets[d.wrapping_add(1 as size_t) as usize]
                        .wrapping_sub((*addon).chunk_offsets[d as usize])
                        .wrapping_sub(offset);
                    limit = if limit > max_len { max_len } else { limit };
                    if best_len >= limit
                        || continuation as ::core::ffi::c_int
                            != *source.offset(offset.wrapping_add(best_len) as isize)
                                as ::core::ffi::c_int
                    {
                        current_block_23 = 4166486009154926805;
                    } else {
                        len = FindMatchLengthWithLimit(
                            source.offset(offset as isize) as *const uint8_t,
                            ringbuffer.offset(cur_ix_masked as isize) as *const uint8_t,
                            limit,
                        );
                        current_block_23 = 2873832966593178012;
                    }
                } else {
                    current_block_23 = 4166486009154926805;
                }
                match current_block_23 {
                    4166486009154926805 => {}
                    _ => {
                        let dist_cost: ::core::ffi::c_float = base_cost
                            + ZopfliCostModelGetDistanceCost(model, j) as ::core::ffi::c_float;
                        let mut l: size_t = 0;
                        l = best_len.wrapping_add(1 as size_t);
                        while l <= len {
                            let copycode: uint16_t = GetCopyLengthCode(l) as uint16_t;
                            let cmdcode: uint16_t = CombineLengthCodes(
                                inscode,
                                copycode,
                                (j == 0 as size_t) as ::core::ffi::c_int,
                            ) as uint16_t;
                            let cost: ::core::ffi::c_float =
                                (if (cmdcode as ::core::ffi::c_int) < 128 as ::core::ffi::c_int {
                                    base_cost
                                } else {
                                    dist_cost
                                }) + GetCopyExtra(copycode) as ::core::ffi::c_float
                                    + ZopfliCostModelGetCommandCost(model, cmdcode)
                                        as ::core::ffi::c_float;
                            if cost < (*nodes.offset(pos.wrapping_add(l) as isize)).u.cost {
                                UpdateZopfliNode(
                                    nodes,
                                    pos,
                                    start,
                                    l,
                                    l,
                                    backward,
                                    j.wrapping_add(1 as size_t),
                                    cost,
                                );
                                result = brotli_max_size_t(result, l);
                            }
                            best_len = l;
                            l = l.wrapping_add(1);
                        }
                    }
                }
            }
            j = j.wrapping_add(1);
        }
        if !(k >= 2 as size_t) {
            let mut len_0: size_t = min_len;
            j = 0 as size_t;
            while j < num_matches {
                let mut match_0: BackwardMatch = *matches.offset(j as isize);
                let mut dist: size_t = match_0.distance as size_t;
                let mut is_dictionary_match: ::core::ffi::c_int =
                    if dist > dictionary_start.wrapping_add(gap) {
                        BROTLI_TRUE
                    } else {
                        BROTLI_FALSE
                    };
                let mut dist_code: size_t = dist
                    .wrapping_add(BROTLI_NUM_DISTANCE_SHORT_CODES as size_t)
                    .wrapping_sub(1 as size_t);
                let mut dist_symbol: uint16_t = 0;
                let mut distextra: uint32_t = 0;
                let mut distnumextra: uint32_t = 0;
                let mut dist_cost_0: ::core::ffi::c_float = 0.;
                let mut max_match_len: size_t = 0;
                PrefixEncodeCopyDistance(
                    dist_code,
                    (*params).dist.num_direct_distance_codes as size_t,
                    (*params).dist.distance_postfix_bits as size_t,
                    &raw mut dist_symbol,
                    &raw mut distextra,
                );
                distnumextra =
                    (dist_symbol as ::core::ffi::c_int >> 10 as ::core::ffi::c_int) as uint32_t;
                dist_cost_0 = base_cost
                    + distnumextra as ::core::ffi::c_float
                    + ZopfliCostModelGetDistanceCost(
                        model,
                        (dist_symbol as ::core::ffi::c_int & 0x3ff as ::core::ffi::c_int) as size_t,
                    );
                max_match_len = BackwardMatchLength(&raw mut match_0);
                if len_0 < max_match_len
                    && (is_dictionary_match != 0 || max_match_len > max_zopfli_len)
                {
                    len_0 = max_match_len;
                }
                while len_0 <= max_match_len {
                    let len_code: size_t = if is_dictionary_match != 0 {
                        BackwardMatchLengthCode(&raw mut match_0) as size_t
                    } else {
                        len_0
                    };
                    let copycode_0: uint16_t = GetCopyLengthCode(len_code) as uint16_t;
                    let cmdcode_0: uint16_t =
                        CombineLengthCodes(inscode, copycode_0, 0 as ::core::ffi::c_int)
                            as uint16_t;
                    let cost_0: ::core::ffi::c_float = dist_cost_0
                        + GetCopyExtra(copycode_0) as ::core::ffi::c_float
                        + ZopfliCostModelGetCommandCost(model, cmdcode_0) as ::core::ffi::c_float;
                    if cost_0 < (*nodes.offset(pos.wrapping_add(len_0) as isize)).u.cost {
                        UpdateZopfliNode(
                            nodes,
                            pos,
                            start,
                            len_0,
                            len_code,
                            dist,
                            0 as size_t,
                            cost_0,
                        );
                        result = brotli_max_size_t(result, len_0);
                    }
                    len_0 = len_0.wrapping_add(1);
                }
                j = j.wrapping_add(1);
            }
        }
        k = k.wrapping_add(1);
    }
    return result;
}
unsafe extern "C" fn ComputeShortestPathFromNodes(
    mut num_bytes: size_t,
    mut nodes: *mut ZopfliNode,
) -> size_t {
    let mut index: size_t = num_bytes;
    let mut num_commands: size_t = 0 as size_t;
    while (*nodes.offset(index as isize)).dcode_insert_length & 0x7ffffff as uint32_t
        == 0 as uint32_t
        && (*nodes.offset(index as isize)).length == 1 as uint32_t
    {
        index = index.wrapping_sub(1);
    }
    (*nodes.offset(index as isize)).u.next = BROTLI_UINT32_MAX;
    while index != 0 as size_t {
        let mut len: size_t =
            ZopfliNodeCommandLength(nodes.offset(index as isize) as *mut ZopfliNode) as size_t;
        index = (index as ::core::ffi::c_ulong).wrapping_sub(len as ::core::ffi::c_ulong) as size_t
            as size_t;
        (*nodes.offset(index as isize)).u.next = len as uint32_t;
        num_commands = num_commands.wrapping_add(1);
    }
    return num_commands;
}
#[no_mangle]
pub unsafe extern "C" fn BrotliZopfliCreateCommands(
    num_bytes: size_t,
    block_start: size_t,
    mut nodes: *const ZopfliNode,
    mut dist_cache: *mut ::core::ffi::c_int,
    mut last_insert_len: *mut size_t,
    mut params: *const BrotliEncoderParams,
    mut commands: *mut Command,
    mut num_literals: *mut size_t,
) {
    let stream_offset: size_t = (*params).stream_offset;
    let max_backward_limit: size_t = ((1 as ::core::ffi::c_int as size_t) << (*params).lgwin)
        .wrapping_sub(BROTLI_WINDOW_GAP as size_t);
    let mut pos: size_t = 0 as size_t;
    let mut offset: uint32_t = (*nodes.offset(0 as ::core::ffi::c_int as isize)).u.next;
    let mut i: size_t = 0;
    let mut gap: size_t = (*params).dictionary.compound.total_size;
    i = 0 as size_t;
    while offset != BROTLI_UINT32_MAX {
        let mut next: *const ZopfliNode =
            nodes.offset(pos.wrapping_add(offset as size_t) as isize) as *const ZopfliNode;
        let mut copy_length: size_t = ZopfliNodeCopyLength(next) as size_t;
        let mut insert_length: size_t =
            ((*next).dcode_insert_length & 0x7ffffff as uint32_t) as size_t;
        pos = (pos as ::core::ffi::c_ulong).wrapping_add(insert_length as ::core::ffi::c_ulong)
            as size_t as size_t;
        offset = (*next).u.next;
        if i == 0 as size_t {
            insert_length = (insert_length as ::core::ffi::c_ulong)
                .wrapping_add(*last_insert_len as ::core::ffi::c_ulong)
                as size_t as size_t;
            *last_insert_len = 0 as size_t;
        }
        let mut distance: size_t = ZopfliNodeCopyDistance(next) as size_t;
        let mut len_code: size_t = ZopfliNodeLengthCode(next) as size_t;
        let mut dictionary_start: size_t = brotli_min_size_t(
            block_start.wrapping_add(pos).wrapping_add(stream_offset),
            max_backward_limit,
        );
        let mut is_dictionary: ::core::ffi::c_int = if distance > dictionary_start.wrapping_add(gap)
        {
            BROTLI_TRUE
        } else {
            BROTLI_FALSE
        };
        let mut dist_code: size_t = ZopfliNodeDistanceCode(next) as size_t;
        InitCommand(
            commands.offset(i as isize) as *mut Command,
            &raw const (*params).dist,
            insert_length,
            copy_length,
            len_code as ::core::ffi::c_int - copy_length as ::core::ffi::c_int,
            dist_code,
        );
        if is_dictionary == 0 && dist_code > 0 as size_t {
            *dist_cache.offset(3 as ::core::ffi::c_int as isize) =
                *dist_cache.offset(2 as ::core::ffi::c_int as isize);
            *dist_cache.offset(2 as ::core::ffi::c_int as isize) =
                *dist_cache.offset(1 as ::core::ffi::c_int as isize);
            *dist_cache.offset(1 as ::core::ffi::c_int as isize) =
                *dist_cache.offset(0 as ::core::ffi::c_int as isize);
            *dist_cache.offset(0 as ::core::ffi::c_int as isize) = distance as ::core::ffi::c_int;
        }
        *num_literals = (*num_literals as ::core::ffi::c_ulong)
            .wrapping_add(insert_length as ::core::ffi::c_ulong) as size_t
            as size_t;
        pos = (pos as ::core::ffi::c_ulong).wrapping_add(copy_length as ::core::ffi::c_ulong)
            as size_t as size_t;
        i = i.wrapping_add(1);
    }
    *last_insert_len = (*last_insert_len as ::core::ffi::c_ulong)
        .wrapping_add(num_bytes.wrapping_sub(pos) as ::core::ffi::c_ulong)
        as size_t as size_t;
}
unsafe extern "C" fn ZopfliIterate(
    mut num_bytes: size_t,
    mut position: size_t,
    mut ringbuffer: *const uint8_t,
    mut ringbuffer_mask: size_t,
    mut params: *const BrotliEncoderParams,
    gap: size_t,
    mut dist_cache: *const ::core::ffi::c_int,
    mut model: *const ZopfliCostModel,
    mut num_matches: *const uint32_t,
    mut matches: *const BackwardMatch,
    mut nodes: *mut ZopfliNode,
) -> size_t {
    let stream_offset: size_t = (*params).stream_offset;
    let max_backward_limit: size_t = ((1 as ::core::ffi::c_int as size_t) << (*params).lgwin)
        .wrapping_sub(BROTLI_WINDOW_GAP as size_t);
    let max_zopfli_len: size_t = MaxZopfliLen(params) as size_t;
    let mut queue: StartPosQueue = StartPosQueue {
        q_: [PosData {
            pos: 0,
            distance_cache: [0; 4],
            costdiff: 0.,
            cost: 0.,
        }; 8],
        idx_: 0,
    };
    let mut cur_match_pos: size_t = 0 as size_t;
    let mut i: size_t = 0;
    (*nodes.offset(0 as ::core::ffi::c_int as isize)).length = 0 as uint32_t;
    (*nodes.offset(0 as ::core::ffi::c_int as isize)).u.cost =
        0 as ::core::ffi::c_int as ::core::ffi::c_float;
    InitStartPosQueue(&raw mut queue);
    i = 0 as size_t;
    while i.wrapping_add(3 as size_t) < num_bytes {
        let mut skip: size_t = UpdateNodes(
            num_bytes,
            position,
            i,
            ringbuffer,
            ringbuffer_mask,
            params,
            max_backward_limit,
            dist_cache,
            *num_matches.offset(i as isize) as size_t,
            matches.offset(cur_match_pos as isize) as *const BackwardMatch,
            model,
            &raw mut queue,
            nodes,
        );
        if skip < BROTLI_LONG_COPY_QUICK_STEP as size_t {
            skip = 0 as size_t;
        }
        cur_match_pos = (cur_match_pos as ::core::ffi::c_ulong)
            .wrapping_add(*num_matches.offset(i as isize) as ::core::ffi::c_ulong)
            as size_t as size_t;
        if *num_matches.offset(i as isize) == 1 as uint32_t
            && BackwardMatchLength(
                matches.offset(cur_match_pos.wrapping_sub(1 as size_t) as isize)
                    as *const BackwardMatch,
            ) > max_zopfli_len
        {
            skip = brotli_max_size_t(
                BackwardMatchLength(
                    matches.offset(cur_match_pos.wrapping_sub(1 as size_t) as isize)
                        as *const BackwardMatch,
                ),
                skip,
            );
        }
        if skip > 1 as size_t {
            skip = skip.wrapping_sub(1);
            while skip != 0 {
                i = i.wrapping_add(1);
                if i.wrapping_add(3 as size_t) >= num_bytes {
                    break;
                }
                EvaluateNode(
                    position.wrapping_add(stream_offset),
                    i,
                    max_backward_limit,
                    gap,
                    dist_cache,
                    model,
                    &raw mut queue,
                    nodes,
                );
                cur_match_pos = (cur_match_pos as ::core::ffi::c_ulong)
                    .wrapping_add(*num_matches.offset(i as isize) as ::core::ffi::c_ulong)
                    as size_t as size_t;
                skip = skip.wrapping_sub(1);
            }
        }
        i = i.wrapping_add(1);
    }
    return ComputeShortestPathFromNodes(num_bytes, nodes);
}
unsafe extern "C" fn MergeMatches(
    mut dst: *mut BackwardMatch,
    mut src1: *mut BackwardMatch,
    mut len1: size_t,
    mut src2: *mut BackwardMatch,
    mut len2: size_t,
) {
    while len1 > 0 as size_t && len2 > 0 as size_t {
        let mut l1: size_t = BackwardMatchLength(src1);
        let mut l2: size_t = BackwardMatchLength(src2);
        if l1 < l2 || l1 == l2 && (*src1).distance < (*src2).distance {
            let fresh7 = src1;
            src1 = src1.offset(1);
            let fresh8 = dst;
            dst = dst.offset(1);
            *fresh8 = *fresh7;
            len1 = len1.wrapping_sub(1);
        } else {
            let fresh9 = src2;
            src2 = src2.offset(1);
            let fresh10 = dst;
            dst = dst.offset(1);
            *fresh10 = *fresh9;
            len2 = len2.wrapping_sub(1);
        }
    }
    loop {
        let fresh11 = len1;
        len1 = len1.wrapping_sub(1);
        if !(fresh11 > 0 as size_t) {
            break;
        }
        let fresh12 = src1;
        src1 = src1.offset(1);
        let fresh13 = dst;
        dst = dst.offset(1);
        *fresh13 = *fresh12;
    }
    loop {
        let fresh14 = len2;
        len2 = len2.wrapping_sub(1);
        if !(fresh14 > 0 as size_t) {
            break;
        }
        let fresh15 = src2;
        src2 = src2.offset(1);
        let fresh16 = dst;
        dst = dst.offset(1);
        *fresh16 = *fresh15;
    }
}
#[no_mangle]
pub unsafe extern "C" fn BrotliZopfliComputeShortestPath(
    mut m: *mut MemoryManager,
    mut num_bytes: size_t,
    mut position: size_t,
    mut ringbuffer: *const uint8_t,
    mut ringbuffer_mask: size_t,
    mut literal_context_lut: ContextLut,
    mut params: *const BrotliEncoderParams,
    mut dist_cache: *const ::core::ffi::c_int,
    mut hasher: *mut Hasher,
    mut nodes: *mut ZopfliNode,
) -> size_t {
    let stream_offset: size_t = (*params).stream_offset;
    let max_backward_limit: size_t = ((1 as ::core::ffi::c_int as size_t) << (*params).lgwin)
        .wrapping_sub(BROTLI_WINDOW_GAP as size_t);
    let max_zopfli_len: size_t = MaxZopfliLen(params) as size_t;
    let mut queue: StartPosQueue = StartPosQueue {
        q_: [PosData {
            pos: 0,
            distance_cache: [0; 4],
            costdiff: 0.,
            cost: 0.,
        }; 8],
        idx_: 0,
    };
    let mut matches: *mut BackwardMatch = if 2 as ::core::ffi::c_int
        * (128 as ::core::ffi::c_int + 64 as ::core::ffi::c_int)
        > 0 as ::core::ffi::c_int
    {
        BrotliAllocate(
            m,
            ((2 as ::core::ffi::c_int * (128 as ::core::ffi::c_int + 64 as ::core::ffi::c_int))
                as size_t)
                .wrapping_mul(::core::mem::size_of::<BackwardMatch>() as size_t),
        ) as *mut BackwardMatch
    } else {
        ::core::ptr::null_mut::<BackwardMatch>()
    };
    let store_end: size_t = if num_bytes >= StoreLookaheadH10() {
        position
            .wrapping_add(num_bytes)
            .wrapping_sub(StoreLookaheadH10() as size_t)
            .wrapping_add(1 as size_t)
    } else {
        position
    };
    let mut i: size_t = 0;
    let mut addon: *const CompoundDictionary = &raw const (*params).dictionary.compound;
    let mut gap: size_t = (*addon).total_size;
    let mut lz_matches_offset: size_t = (if (*addon).num_chunks != 0 as size_t {
        MAX_NUM_MATCHES_H10 + 128 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    }) as size_t;
    let mut model: *mut ZopfliCostModel = if 1 as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
        BrotliAllocate(
            m,
            (1 as size_t).wrapping_mul(::core::mem::size_of::<ZopfliCostModel>() as size_t),
        ) as *mut ZopfliCostModel
    } else {
        ::core::ptr::null_mut::<ZopfliCostModel>()
    };
    if 0 as ::core::ffi::c_int != 0 || 0 as ::core::ffi::c_int != 0 || 0 as ::core::ffi::c_int != 0
    {
        return 0 as size_t;
    }
    (*nodes.offset(0 as ::core::ffi::c_int as isize)).length = 0 as uint32_t;
    (*nodes.offset(0 as ::core::ffi::c_int as isize)).u.cost =
        0 as ::core::ffi::c_int as ::core::ffi::c_float;
    InitZopfliCostModel(m, model, &raw const (*params).dist, num_bytes);
    if 0 as ::core::ffi::c_int != 0 {
        return 0 as size_t;
    }
    ZopfliCostModelSetFromLiteralCosts(model, position, ringbuffer, ringbuffer_mask);
    InitStartPosQueue(&raw mut queue);
    i = 0 as size_t;
    while i
        .wrapping_add(HashTypeLengthH10())
        .wrapping_sub(1 as size_t)
        < num_bytes
    {
        let pos: size_t = position.wrapping_add(i);
        let max_distance: size_t = brotli_min_size_t(pos, max_backward_limit) as size_t;
        let dictionary_start: size_t =
            brotli_min_size_t(pos.wrapping_add(stream_offset), max_backward_limit) as size_t;
        let mut skip: size_t = 0;
        let mut num_matches: size_t = 0;
        let mut dict_id: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        if (*params).dictionary.contextual.context_based != 0 {
            let mut p1: uint8_t = (if pos >= 1 as size_t {
                *ringbuffer.offset((pos.wrapping_sub(1 as size_t) & ringbuffer_mask) as isize)
                    as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            }) as uint8_t;
            let mut p2: uint8_t = (if pos >= 2 as size_t {
                *ringbuffer.offset((pos.wrapping_sub(2 as size_t) & ringbuffer_mask) as isize)
                    as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            }) as uint8_t;
            dict_id = (*params).dictionary.contextual.context_map[(*literal_context_lut
                .offset(p1 as isize)
                as ::core::ffi::c_int
                | *literal_context_lut
                    .offset(256 as ::core::ffi::c_int as isize)
                    .offset(p2 as isize) as ::core::ffi::c_int)
                as usize] as ::core::ffi::c_int;
        }
        num_matches = FindAllMatchesH10(
            &raw mut (*hasher).privat._H10,
            (*params).dictionary.contextual.dict[dict_id as usize],
            ringbuffer,
            ringbuffer_mask,
            pos,
            num_bytes.wrapping_sub(i),
            max_distance,
            dictionary_start.wrapping_add(gap),
            params,
            matches.offset(lz_matches_offset as isize) as *mut BackwardMatch,
        );
        if (*addon).num_chunks != 0 as size_t {
            let mut cd_matches: size_t = LookupAllCompoundDictionaryMatches(
                addon,
                ringbuffer,
                ringbuffer_mask,
                pos,
                3 as size_t,
                num_bytes.wrapping_sub(i),
                dictionary_start,
                (*params).dist.max_distance,
                matches.offset(lz_matches_offset.wrapping_sub(64 as size_t) as isize)
                    as *mut BackwardMatch,
                64 as size_t,
            );
            MergeMatches(
                matches,
                matches.offset(lz_matches_offset.wrapping_sub(64 as size_t) as isize)
                    as *mut BackwardMatch,
                cd_matches,
                matches.offset(lz_matches_offset as isize) as *mut BackwardMatch,
                num_matches,
            );
            num_matches = (num_matches as ::core::ffi::c_ulong)
                .wrapping_add(cd_matches as ::core::ffi::c_ulong)
                as size_t as size_t;
        }
        if num_matches > 0 as size_t
            && BackwardMatchLength(
                matches.offset(num_matches.wrapping_sub(1 as size_t) as isize)
                    as *mut BackwardMatch,
            ) > max_zopfli_len
        {
            *matches.offset(0 as ::core::ffi::c_int as isize) =
                *matches.offset(num_matches.wrapping_sub(1 as size_t) as isize);
            num_matches = 1 as size_t;
        }
        skip = UpdateNodes(
            num_bytes,
            position,
            i,
            ringbuffer,
            ringbuffer_mask,
            params,
            max_backward_limit,
            dist_cache,
            num_matches,
            matches,
            model,
            &raw mut queue,
            nodes,
        );
        if skip < BROTLI_LONG_COPY_QUICK_STEP as size_t {
            skip = 0 as size_t;
        }
        if num_matches == 1 as size_t
            && BackwardMatchLength(
                matches.offset(0 as ::core::ffi::c_int as isize) as *mut BackwardMatch
            ) > max_zopfli_len
        {
            skip = brotli_max_size_t(
                BackwardMatchLength(
                    matches.offset(0 as ::core::ffi::c_int as isize) as *mut BackwardMatch
                ),
                skip,
            );
        }
        if skip > 1 as size_t {
            StoreRangeH10(
                &raw mut (*hasher).privat._H10,
                ringbuffer,
                ringbuffer_mask,
                pos.wrapping_add(1 as size_t),
                brotli_min_size_t(pos.wrapping_add(skip), store_end),
            );
            skip = skip.wrapping_sub(1);
            while skip != 0 {
                i = i.wrapping_add(1);
                if i.wrapping_add(HashTypeLengthH10())
                    .wrapping_sub(1 as size_t)
                    >= num_bytes
                {
                    break;
                }
                EvaluateNode(
                    position.wrapping_add(stream_offset),
                    i,
                    max_backward_limit,
                    gap,
                    dist_cache,
                    model,
                    &raw mut queue,
                    nodes,
                );
                skip = skip.wrapping_sub(1);
            }
        }
        i = i.wrapping_add(1);
    }
    CleanupZopfliCostModel(m, model);
    BrotliFree(m, model as *mut ::core::ffi::c_void);
    model = ::core::ptr::null_mut::<ZopfliCostModel>();
    BrotliFree(m, matches as *mut ::core::ffi::c_void);
    matches = ::core::ptr::null_mut::<BackwardMatch>();
    return ComputeShortestPathFromNodes(num_bytes, nodes);
}
#[no_mangle]
pub unsafe extern "C" fn BrotliCreateZopfliBackwardReferences(
    mut m: *mut MemoryManager,
    mut num_bytes: size_t,
    mut position: size_t,
    mut ringbuffer: *const uint8_t,
    mut ringbuffer_mask: size_t,
    mut literal_context_lut: ContextLut,
    mut params: *const BrotliEncoderParams,
    mut hasher: *mut Hasher,
    mut dist_cache: *mut ::core::ffi::c_int,
    mut last_insert_len: *mut size_t,
    mut commands: *mut Command,
    mut num_commands: *mut size_t,
    mut num_literals: *mut size_t,
) {
    let mut nodes: *mut ZopfliNode = if num_bytes.wrapping_add(1 as size_t) > 0 as size_t {
        BrotliAllocate(
            m,
            num_bytes
                .wrapping_add(1 as size_t)
                .wrapping_mul(::core::mem::size_of::<ZopfliNode>() as size_t),
        ) as *mut ZopfliNode
    } else {
        ::core::ptr::null_mut::<ZopfliNode>()
    };
    if 0 as ::core::ffi::c_int != 0 || 0 as ::core::ffi::c_int != 0 {
        return;
    }
    BrotliInitZopfliNodes(nodes, num_bytes.wrapping_add(1 as size_t));
    *num_commands =
        (*num_commands as ::core::ffi::c_ulong).wrapping_add(BrotliZopfliComputeShortestPath(
            m,
            num_bytes,
            position,
            ringbuffer,
            ringbuffer_mask,
            literal_context_lut,
            params,
            dist_cache,
            hasher,
            nodes,
        ) as ::core::ffi::c_ulong) as size_t as size_t;
    if 0 as ::core::ffi::c_int != 0 {
        return;
    }
    BrotliZopfliCreateCommands(
        num_bytes,
        position,
        nodes,
        dist_cache,
        last_insert_len,
        params,
        commands,
        num_literals,
    );
    BrotliFree(m, nodes as *mut ::core::ffi::c_void);
    nodes = ::core::ptr::null_mut::<ZopfliNode>();
}
#[no_mangle]
pub unsafe extern "C" fn BrotliCreateHqZopfliBackwardReferences(
    mut m: *mut MemoryManager,
    mut num_bytes: size_t,
    mut position: size_t,
    mut ringbuffer: *const uint8_t,
    mut ringbuffer_mask: size_t,
    mut literal_context_lut: ContextLut,
    mut params: *const BrotliEncoderParams,
    mut hasher: *mut Hasher,
    mut dist_cache: *mut ::core::ffi::c_int,
    mut last_insert_len: *mut size_t,
    mut commands: *mut Command,
    mut num_commands: *mut size_t,
    mut num_literals: *mut size_t,
) {
    let stream_offset: size_t = (*params).stream_offset;
    let max_backward_limit: size_t = ((1 as ::core::ffi::c_int as size_t) << (*params).lgwin)
        .wrapping_sub(BROTLI_WINDOW_GAP as size_t);
    let mut num_matches: *mut uint32_t = if num_bytes > 0 as size_t {
        BrotliAllocate(
            m,
            num_bytes.wrapping_mul(::core::mem::size_of::<uint32_t>() as size_t),
        ) as *mut uint32_t
    } else {
        ::core::ptr::null_mut::<uint32_t>()
    };
    let mut matches_size: size_t = (4 as size_t).wrapping_mul(num_bytes);
    let store_end: size_t = if num_bytes >= StoreLookaheadH10() {
        position
            .wrapping_add(num_bytes)
            .wrapping_sub(StoreLookaheadH10() as size_t)
            .wrapping_add(1 as size_t)
    } else {
        position
    };
    let mut cur_match_pos: size_t = 0 as size_t;
    let mut i: size_t = 0;
    let mut orig_num_literals: size_t = 0;
    let mut orig_last_insert_len: size_t = 0;
    let mut orig_dist_cache: [::core::ffi::c_int; 4] = [0; 4];
    let mut orig_num_commands: size_t = 0;
    let mut model: *mut ZopfliCostModel = if 1 as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
        BrotliAllocate(
            m,
            (1 as size_t).wrapping_mul(::core::mem::size_of::<ZopfliCostModel>() as size_t),
        ) as *mut ZopfliCostModel
    } else {
        ::core::ptr::null_mut::<ZopfliCostModel>()
    };
    let mut nodes: *mut ZopfliNode = ::core::ptr::null_mut::<ZopfliNode>();
    let mut matches: *mut BackwardMatch = if matches_size > 0 as size_t {
        BrotliAllocate(
            m,
            matches_size.wrapping_mul(::core::mem::size_of::<BackwardMatch>() as size_t),
        ) as *mut BackwardMatch
    } else {
        ::core::ptr::null_mut::<BackwardMatch>()
    };
    let mut addon: *const CompoundDictionary = &raw const (*params).dictionary.compound;
    let mut gap: size_t = (*addon).total_size;
    let mut shadow_matches: size_t = (if (*addon).num_chunks != 0 as size_t {
        MAX_NUM_MATCHES_H10 + 128 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    }) as size_t;
    if 0 as ::core::ffi::c_int != 0
        || 0 as ::core::ffi::c_int != 0
        || 0 as ::core::ffi::c_int != 0
        || 0 as ::core::ffi::c_int != 0
    {
        return;
    }
    i = 0 as size_t;
    while i
        .wrapping_add(HashTypeLengthH10())
        .wrapping_sub(1 as size_t)
        < num_bytes
    {
        let pos: size_t = position.wrapping_add(i);
        let mut max_distance: size_t = brotli_min_size_t(pos, max_backward_limit);
        let mut dictionary_start: size_t =
            brotli_min_size_t(pos.wrapping_add(stream_offset), max_backward_limit);
        let mut max_length: size_t = num_bytes.wrapping_sub(i);
        let mut num_found_matches: size_t = 0;
        let mut cur_match_end: size_t = 0;
        let mut j: size_t = 0;
        let mut dict_id: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        if (*params).dictionary.contextual.context_based != 0 {
            let mut p1: uint8_t = (if pos >= 1 as size_t {
                *ringbuffer.offset((pos.wrapping_sub(1 as size_t) & ringbuffer_mask) as isize)
                    as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            }) as uint8_t;
            let mut p2: uint8_t = (if pos >= 2 as size_t {
                *ringbuffer.offset((pos.wrapping_sub(2 as size_t) & ringbuffer_mask) as isize)
                    as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            }) as uint8_t;
            dict_id = (*params).dictionary.contextual.context_map[(*literal_context_lut
                .offset(p1 as isize)
                as ::core::ffi::c_int
                | *literal_context_lut
                    .offset(256 as ::core::ffi::c_int as isize)
                    .offset(p2 as isize) as ::core::ffi::c_int)
                as usize] as ::core::ffi::c_int;
        }
        if matches_size
            < cur_match_pos
                .wrapping_add(128 as size_t)
                .wrapping_add(shadow_matches)
        {
            let mut _new_size: size_t = if matches_size == 0 as size_t {
                cur_match_pos
                    .wrapping_add(128 as size_t)
                    .wrapping_add(shadow_matches)
            } else {
                matches_size
            };
            let mut new_array: *mut BackwardMatch = ::core::ptr::null_mut::<BackwardMatch>();
            while _new_size
                < cur_match_pos
                    .wrapping_add(128 as size_t)
                    .wrapping_add(shadow_matches)
            {
                _new_size = (_new_size as ::core::ffi::c_ulong)
                    .wrapping_mul(2 as ::core::ffi::c_ulong) as size_t
                    as size_t;
            }
            new_array = if _new_size > 0 as size_t {
                BrotliAllocate(
                    m,
                    _new_size.wrapping_mul(::core::mem::size_of::<BackwardMatch>() as size_t),
                ) as *mut BackwardMatch
            } else {
                ::core::ptr::null_mut::<BackwardMatch>()
            };
            if 0 as ::core::ffi::c_int == 0
                && 0 as ::core::ffi::c_int == 0
                && matches_size != 0 as size_t
            {
                memcpy(
                    new_array as *mut ::core::ffi::c_void,
                    matches as *const ::core::ffi::c_void,
                    matches_size.wrapping_mul(::core::mem::size_of::<BackwardMatch>() as size_t),
                );
            }
            BrotliFree(m, matches as *mut ::core::ffi::c_void);
            matches = ::core::ptr::null_mut::<BackwardMatch>();
            matches = new_array;
            matches_size = _new_size;
        }
        if 0 as ::core::ffi::c_int != 0 {
            return;
        }
        num_found_matches = FindAllMatchesH10(
            &raw mut (*hasher).privat._H10,
            (*params).dictionary.contextual.dict[dict_id as usize],
            ringbuffer,
            ringbuffer_mask,
            pos,
            max_length,
            max_distance,
            dictionary_start.wrapping_add(gap),
            params,
            matches.offset(cur_match_pos.wrapping_add(shadow_matches) as isize)
                as *mut BackwardMatch,
        );
        if (*addon).num_chunks != 0 as size_t {
            let mut cd_matches: size_t = LookupAllCompoundDictionaryMatches(
                addon,
                ringbuffer,
                ringbuffer_mask,
                pos,
                3 as size_t,
                max_length,
                dictionary_start,
                (*params).dist.max_distance,
                matches.offset(
                    cur_match_pos
                        .wrapping_add(shadow_matches)
                        .wrapping_sub(64 as size_t) as isize,
                ) as *mut BackwardMatch,
                64 as size_t,
            );
            MergeMatches(
                matches.offset(cur_match_pos as isize) as *mut BackwardMatch,
                matches.offset(
                    cur_match_pos
                        .wrapping_add(shadow_matches)
                        .wrapping_sub(64 as size_t) as isize,
                ) as *mut BackwardMatch,
                cd_matches,
                matches.offset(cur_match_pos.wrapping_add(shadow_matches) as isize)
                    as *mut BackwardMatch,
                num_found_matches,
            );
            num_found_matches = (num_found_matches as ::core::ffi::c_ulong)
                .wrapping_add(cd_matches as ::core::ffi::c_ulong)
                as size_t as size_t;
        }
        cur_match_end = cur_match_pos.wrapping_add(num_found_matches);
        j = cur_match_pos;
        while j.wrapping_add(1 as size_t) < cur_match_end {
            j = j.wrapping_add(1);
        }
        *num_matches.offset(i as isize) = num_found_matches as uint32_t;
        if num_found_matches > 0 as size_t {
            let match_len: size_t = BackwardMatchLength(
                matches.offset(cur_match_end.wrapping_sub(1 as size_t) as isize)
                    as *mut BackwardMatch,
            ) as size_t;
            if match_len > MAX_ZOPFLI_LEN_QUALITY_11 as size_t {
                let skip: size_t = match_len.wrapping_sub(1 as size_t);
                let fresh17 = cur_match_pos;
                cur_match_pos = cur_match_pos.wrapping_add(1);
                *matches.offset(fresh17 as isize) =
                    *matches.offset(cur_match_end.wrapping_sub(1 as size_t) as isize);
                *num_matches.offset(i as isize) = 1 as uint32_t;
                StoreRangeH10(
                    &raw mut (*hasher).privat._H10,
                    ringbuffer,
                    ringbuffer_mask,
                    pos.wrapping_add(1 as size_t),
                    brotli_min_size_t(pos.wrapping_add(match_len), store_end),
                );
                memset(
                    num_matches.offset(i.wrapping_add(1 as size_t) as isize) as *mut uint32_t
                        as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    skip.wrapping_mul(::core::mem::size_of::<uint32_t>() as size_t),
                );
                i = (i as ::core::ffi::c_ulong).wrapping_add(skip as ::core::ffi::c_ulong) as size_t
                    as size_t;
            } else {
                cur_match_pos = cur_match_end;
            }
        }
        i = i.wrapping_add(1);
    }
    orig_num_literals = *num_literals;
    orig_last_insert_len = *last_insert_len;
    memcpy(
        &raw mut orig_dist_cache as *mut ::core::ffi::c_int as *mut ::core::ffi::c_void,
        dist_cache as *const ::core::ffi::c_void,
        (4 as size_t).wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as size_t),
    );
    orig_num_commands = *num_commands;
    nodes = if num_bytes.wrapping_add(1 as size_t) > 0 as size_t {
        BrotliAllocate(
            m,
            num_bytes
                .wrapping_add(1 as size_t)
                .wrapping_mul(::core::mem::size_of::<ZopfliNode>() as size_t),
        ) as *mut ZopfliNode
    } else {
        ::core::ptr::null_mut::<ZopfliNode>()
    };
    if 0 as ::core::ffi::c_int != 0 || 0 as ::core::ffi::c_int != 0 {
        return;
    }
    InitZopfliCostModel(m, model, &raw const (*params).dist, num_bytes);
    if 0 as ::core::ffi::c_int != 0 {
        return;
    }
    i = 0 as size_t;
    while i < 2 as size_t {
        BrotliInitZopfliNodes(nodes, num_bytes.wrapping_add(1 as size_t));
        if i == 0 as size_t {
            ZopfliCostModelSetFromLiteralCosts(model, position, ringbuffer, ringbuffer_mask);
        } else {
            ZopfliCostModelSetFromCommands(
                model,
                position,
                ringbuffer,
                ringbuffer_mask,
                commands,
                (*num_commands).wrapping_sub(orig_num_commands),
                orig_last_insert_len,
            );
        }
        *num_commands = orig_num_commands;
        *num_literals = orig_num_literals;
        *last_insert_len = orig_last_insert_len;
        memcpy(
            dist_cache as *mut ::core::ffi::c_void,
            &raw mut orig_dist_cache as *mut ::core::ffi::c_int as *const ::core::ffi::c_void,
            (4 as size_t).wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as size_t),
        );
        *num_commands = (*num_commands as ::core::ffi::c_ulong).wrapping_add(ZopfliIterate(
            num_bytes,
            position,
            ringbuffer,
            ringbuffer_mask,
            params,
            gap,
            dist_cache,
            model,
            num_matches,
            matches,
            nodes,
        )
            as ::core::ffi::c_ulong) as size_t as size_t;
        BrotliZopfliCreateCommands(
            num_bytes,
            position,
            nodes,
            dist_cache,
            last_insert_len,
            params,
            commands,
            num_literals,
        );
        i = i.wrapping_add(1);
    }
    CleanupZopfliCostModel(m, model);
    BrotliFree(m, model as *mut ::core::ffi::c_void);
    model = ::core::ptr::null_mut::<ZopfliCostModel>();
    BrotliFree(m, nodes as *mut ::core::ffi::c_void);
    nodes = ::core::ptr::null_mut::<ZopfliNode>();
    BrotliFree(m, matches as *mut ::core::ffi::c_void);
    matches = ::core::ptr::null_mut::<BackwardMatch>();
    BrotliFree(m, num_matches as *mut ::core::ffi::c_void);
    num_matches = ::core::ptr::null_mut::<uint32_t>();
}
