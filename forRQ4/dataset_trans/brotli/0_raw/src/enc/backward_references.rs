#[cfg(target_arch = "x86")]
pub use ::core::arch::x86::{
    __m128i, _mm_cmpeq_epi8, _mm_loadu_si128, _mm_movemask_epi8, _mm_set1_epi8, _mm_set_epi8,
    _mm_setzero_si128,
};
#[cfg(target_arch = "x86_64")]
pub use ::core::arch::x86_64::{
    __m128i, _mm_cmpeq_epi8, _mm_loadu_si128, _mm_movemask_epi8, _mm_set1_epi8, _mm_set_epi8,
    _mm_setzero_si128,
};
extern "C" {
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
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
#[repr(C, packed)]
pub struct __loadu_si128 {
    pub __v: __m128i,
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
pub struct HasherSearchResult {
    pub len: size_t,
    pub distance: size_t,
    pub score: size_t,
    pub len_code_delta: ::core::ffi::c_int,
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
pub struct SlotH40 {
    pub delta: uint16_t,
    pub next: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BankH40 {
    pub slots: [SlotH40; 65536],
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
pub struct SlotH41 {
    pub delta: uint16_t,
    pub next: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BankH41 {
    pub slots: [SlotH41; 65536],
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
pub struct SlotH42 {
    pub delta: uint16_t,
    pub next: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BankH42 {
    pub slots: [SlotH42; 512],
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
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const BROTLI_TRUE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const BROTLI_FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
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
unsafe extern "C" fn brotli_min_size_t(mut a: size_t, mut b: size_t) -> size_t {
    return if a < b { a } else { b };
}
#[inline(always)]
unsafe extern "C" fn brotli_max_size_t(mut a: size_t, mut b: size_t) -> size_t {
    return if a > b { a } else { b };
}
#[inline(always)]
unsafe extern "C" fn BrotliRotateRight16(value: uint16_t, mut count: size_t) -> uint16_t {
    count = (count as ::core::ffi::c_ulong & 0xf as ::core::ffi::c_ulong) as size_t;
    return (value as ::core::ffi::c_int >> count
        | ((value as ::core::ffi::c_int) << ((0 as size_t).wrapping_sub(count) & 0xf as size_t))
            as uint16_t as ::core::ffi::c_int) as uint16_t;
}
#[inline(always)]
unsafe extern "C" fn BrotliRotateRight32(value: uint32_t, mut count: size_t) -> uint32_t {
    count = (count as ::core::ffi::c_ulong & 0x1f as ::core::ffi::c_ulong) as size_t;
    return value >> count | value << ((0 as size_t).wrapping_sub(count) & 0x1f as size_t);
}
#[inline(always)]
unsafe extern "C" fn BrotliRotateRight64(value: uint64_t, mut count: size_t) -> uint64_t {
    count = (count as ::core::ffi::c_ulong & 0x3f as ::core::ffi::c_ulong) as size_t;
    return value >> count | value << ((0 as size_t).wrapping_sub(count) & 0x3f as size_t);
}
pub const BROTLI_NUM_DISTANCE_SHORT_CODES: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const BROTLI_WINDOW_GAP: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
#[inline(always)]
unsafe extern "C" fn Log2FloorNonZero(mut n: size_t) -> uint32_t {
    return 31 as uint32_t ^ (n as uint32_t).leading_zeros() as i32 as uint32_t;
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
static mut kHashMul64: uint64_t = (0x1fe35a7b as ::core::ffi::c_uint as uint64_t)
    << 32 as ::core::ffi::c_int
    | 0xd3579bd3 as uint64_t;
#[inline(always)]
unsafe extern "C" fn Hash14(mut data: *const uint8_t) -> uint32_t {
    let mut h: uint32_t =
        BrotliUnalignedRead32(data as *const ::core::ffi::c_void).wrapping_mul(kHashMul32);
    return h >> 32 as ::core::ffi::c_int - 14 as ::core::ffi::c_int;
}
#[inline(always)]
unsafe extern "C" fn GetMatchingTagMask(
    mut chunk_count: size_t,
    tag: uint8_t,
    mut tag_bucket: *const uint8_t,
    head: size_t,
) -> uint64_t {
    let mut matches: uint64_t = 0 as uint64_t;
    let comparison_mask: __m128i = _mm_set1_epi8(tag as ::core::ffi::c_char) as __m128i;
    let mut i: size_t = 0;
    i = 0 as size_t;
    while i < chunk_count && i < 4 as size_t {
        let chunk: __m128i =
            _mm_loadu_si128(tag_bucket.offset((16 as size_t).wrapping_mul(i) as isize)
                as *const ::core::ffi::c_void as *const __m128i) as __m128i;
        let equal_mask: __m128i = _mm_cmpeq_epi8(chunk, comparison_mask) as __m128i;
        matches = (matches as ::core::ffi::c_ulong
            | ((_mm_movemask_epi8(equal_mask) as uint64_t) << (16 as size_t).wrapping_mul(i))
                as ::core::ffi::c_ulong) as uint64_t;
        i = i.wrapping_add(1);
    }
    if chunk_count == 1 as size_t {
        return BrotliRotateRight16(matches as uint16_t, head) as uint64_t;
    }
    if chunk_count == 2 as size_t {
        return BrotliRotateRight32(matches as uint32_t, head) as uint64_t;
    }
    return BrotliRotateRight64(matches, head);
}
pub const MIN_QUALITY_FOR_EXTENSIVE_REFERENCE_SEARCH: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
#[inline(always)]
unsafe extern "C" fn LiteralSpreeLengthForSparseSearch(
    mut params: *const BrotliEncoderParams,
) -> size_t {
    return (if (*params).quality < 9 as ::core::ffi::c_int {
        64 as ::core::ffi::c_int
    } else {
        512 as ::core::ffi::c_int
    }) as size_t;
}
#[inline(always)]
unsafe extern "C" fn PrepareDistanceCache(
    mut distance_cache: *mut ::core::ffi::c_int,
    num_distances: ::core::ffi::c_int,
) {
    if num_distances > 4 as ::core::ffi::c_int {
        let mut last_distance: ::core::ffi::c_int =
            *distance_cache.offset(0 as ::core::ffi::c_int as isize);
        *distance_cache.offset(4 as ::core::ffi::c_int as isize) =
            last_distance - 1 as ::core::ffi::c_int;
        *distance_cache.offset(5 as ::core::ffi::c_int as isize) =
            last_distance + 1 as ::core::ffi::c_int;
        *distance_cache.offset(6 as ::core::ffi::c_int as isize) =
            last_distance - 2 as ::core::ffi::c_int;
        *distance_cache.offset(7 as ::core::ffi::c_int as isize) =
            last_distance + 2 as ::core::ffi::c_int;
        *distance_cache.offset(8 as ::core::ffi::c_int as isize) =
            last_distance - 3 as ::core::ffi::c_int;
        *distance_cache.offset(9 as ::core::ffi::c_int as isize) =
            last_distance + 3 as ::core::ffi::c_int;
        if num_distances > 10 as ::core::ffi::c_int {
            let mut next_last_distance: ::core::ffi::c_int =
                *distance_cache.offset(1 as ::core::ffi::c_int as isize);
            *distance_cache.offset(10 as ::core::ffi::c_int as isize) =
                next_last_distance - 1 as ::core::ffi::c_int;
            *distance_cache.offset(11 as ::core::ffi::c_int as isize) =
                next_last_distance + 1 as ::core::ffi::c_int;
            *distance_cache.offset(12 as ::core::ffi::c_int as isize) =
                next_last_distance - 2 as ::core::ffi::c_int;
            *distance_cache.offset(13 as ::core::ffi::c_int as isize) =
                next_last_distance + 2 as ::core::ffi::c_int;
            *distance_cache.offset(14 as ::core::ffi::c_int as isize) =
                next_last_distance - 3 as ::core::ffi::c_int;
            *distance_cache.offset(15 as ::core::ffi::c_int as isize) =
                next_last_distance + 3 as ::core::ffi::c_int;
        }
    }
}
pub const BROTLI_LITERAL_BYTE_SCORE: ::core::ffi::c_int = 135 as ::core::ffi::c_int;
pub const BROTLI_DISTANCE_BIT_PENALTY: ::core::ffi::c_int = 30 as ::core::ffi::c_int;
pub const BROTLI_SCORE_BASE: usize = ((BROTLI_DISTANCE_BIT_PENALTY * 8 as ::core::ffi::c_int)
    as usize)
    .wrapping_mul(::core::mem::size_of::<size_t>() as usize);
#[inline(always)]
unsafe extern "C" fn BackwardReferenceScore(
    mut copy_length: size_t,
    mut backward_reference_offset: size_t,
) -> size_t {
    return BROTLI_SCORE_BASE
        .wrapping_add((BROTLI_LITERAL_BYTE_SCORE as size_t).wrapping_mul(copy_length))
        .wrapping_sub(
            (BROTLI_DISTANCE_BIT_PENALTY as uint32_t)
                .wrapping_mul(Log2FloorNonZero(backward_reference_offset)) as size_t,
        );
}
#[inline(always)]
unsafe extern "C" fn BackwardReferenceScoreUsingLastDistance(mut copy_length: size_t) -> size_t {
    return (BROTLI_LITERAL_BYTE_SCORE as size_t)
        .wrapping_mul(copy_length)
        .wrapping_add(BROTLI_SCORE_BASE)
        .wrapping_add(15 as size_t);
}
#[inline(always)]
unsafe extern "C" fn BackwardReferencePenaltyUsingLastDistance(
    mut distance_short_code: size_t,
) -> size_t {
    return (39 as ::core::ffi::c_int as size_t).wrapping_add(
        (0x1ca10 as ::core::ffi::c_int >> (distance_short_code & 0xe as size_t)
            & 0xe as ::core::ffi::c_int) as size_t,
    );
}
#[inline(always)]
unsafe extern "C" fn TestStaticDictionaryItem(
    mut dictionary: *const BrotliEncoderDictionary,
    mut len: size_t,
    mut word_idx: size_t,
    mut data: *const uint8_t,
    mut max_length: size_t,
    mut max_backward: size_t,
    mut max_distance: size_t,
    mut out: *mut HasherSearchResult,
) -> ::core::ffi::c_int {
    let mut offset: size_t = 0;
    let mut matchlen: size_t = 0;
    let mut backward: size_t = 0;
    let mut score: size_t = 0;
    offset = ((*(*dictionary).words).offsets_by_length[len as usize] as size_t)
        .wrapping_add(len.wrapping_mul(word_idx));
    if len > max_length {
        return BROTLI_FALSE;
    }
    matchlen = FindMatchLengthWithLimit(
        data,
        (*(*dictionary).words).data.offset(offset as isize) as *const uint8_t,
        len,
    );
    if matchlen.wrapping_add((*dictionary).cutoffTransformsCount as size_t) <= len
        || matchlen == 0 as size_t
    {
        return BROTLI_FALSE;
    }
    let mut cut: size_t = len.wrapping_sub(matchlen);
    let mut transform_id: size_t = (cut << 2 as ::core::ffi::c_int).wrapping_add(
        ((*dictionary).cutoffTransforms >> cut.wrapping_mul(6 as size_t) & 0x3f as uint64_t)
            as size_t,
    );
    backward = max_backward
        .wrapping_add(1 as size_t)
        .wrapping_add(word_idx)
        .wrapping_add(
            transform_id
                << (*(*dictionary).words).size_bits_by_length[len as usize] as ::core::ffi::c_int,
        );
    if backward > max_distance {
        return BROTLI_FALSE;
    }
    score = BackwardReferenceScore(matchlen, backward);
    if score < (*out).score {
        return BROTLI_FALSE;
    }
    (*out).len = matchlen;
    (*out).len_code_delta = len as ::core::ffi::c_int - matchlen as ::core::ffi::c_int;
    (*out).distance = backward;
    (*out).score = score;
    return BROTLI_TRUE;
}
#[inline(always)]
unsafe extern "C" fn SearchInStaticDictionary(
    mut dictionary: *const BrotliEncoderDictionary,
    mut common: *mut HasherCommon,
    mut data: *const uint8_t,
    mut max_length: size_t,
    mut max_backward: size_t,
    mut max_distance: size_t,
    mut out: *mut HasherSearchResult,
    mut shallow: ::core::ffi::c_int,
) {
    let mut key: size_t = 0;
    let mut i: size_t = 0;
    if (*common).dict_num_matches < (*common).dict_num_lookups >> 7 as ::core::ffi::c_int {
        return;
    }
    key = (Hash14(data) << 1 as ::core::ffi::c_int) as size_t;
    i = 0 as size_t;
    while i
        < (if shallow != 0 {
            1 as ::core::ffi::c_uint
        } else {
            2 as ::core::ffi::c_uint
        }) as size_t
    {
        (*common).dict_num_lookups = (*common).dict_num_lookups.wrapping_add(1);
        if *(*dictionary).hash_table_lengths.offset(key as isize) as ::core::ffi::c_int
            != 0 as ::core::ffi::c_int
        {
            let mut item_matches: ::core::ffi::c_int = TestStaticDictionaryItem(
                dictionary,
                *(*dictionary).hash_table_lengths.offset(key as isize) as size_t,
                *(*dictionary).hash_table_words.offset(key as isize) as size_t,
                data,
                max_length,
                max_backward,
                max_distance,
                out,
            );
            if item_matches != 0 {
                (*common).dict_num_matches = (*common).dict_num_matches.wrapping_add(1);
            }
        }
        i = i.wrapping_add(1);
        key = key.wrapping_add(1);
    }
}
pub const BUCKET_BITS: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const BUCKET_SWEEP_BITS: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const HASH_LEN: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const USE_DICTIONARY: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const BUCKET_SWEEP_BITS_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const USE_DICTIONARY_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const BUCKET_BITS_0: ::core::ffi::c_int = 17 as ::core::ffi::c_int;
pub const BUCKET_SWEEP_BITS_1: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const USE_DICTIONARY_1: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const BUCKET_BITS_1: ::core::ffi::c_int = 15 as ::core::ffi::c_int;
pub const NUM_LAST_DISTANCES_TO_CHECK: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const NUM_BANKS: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const BANK_BITS: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const NUM_LAST_DISTANCES_TO_CHECK_0: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const NUM_LAST_DISTANCES_TO_CHECK_1: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const NUM_BANKS_0: ::core::ffi::c_int = 512 as ::core::ffi::c_int;
pub const BANK_BITS_0: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
pub const BUCKET_BITS_2: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
pub const BUCKET_SWEEP_BITS_2: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const HASH_LEN_0: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const USE_DICTIONARY_2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const CHUNKLEN: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
pub const JUMP: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const NUMBUCKETS: ::core::ffi::c_int = 16777216 as ::core::ffi::c_int;
pub const MASK: ::core::ffi::c_int =
    NUMBUCKETS * 64 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
pub const JUMP_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
#[inline(always)]
unsafe extern "C" fn FindCompoundDictionaryMatch(
    mut self_0: *const PreparedDictionary,
    mut data: *const uint8_t,
    ring_buffer_mask: size_t,
    mut distance_cache: *const ::core::ffi::c_int,
    cur_ix: size_t,
    max_length: size_t,
    distance_offset: size_t,
    max_distance: size_t,
    mut out: *mut HasherSearchResult,
) {
    let source_size: uint32_t = (*self_0).source_size;
    let boundary: size_t = distance_offset.wrapping_sub(source_size as size_t);
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
    let mut best_score: size_t = (*out).score;
    let mut best_len: size_t = (*out).len;
    let mut i: size_t = 0;
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
    let mut tail: *const ::core::ffi::c_void =
        items.offset((*self_0).num_items as isize) as *const uint32_t as *mut ::core::ffi::c_void;
    if (*self_0).magic == kPreparedDictionaryMagic {
        source = tail as *const uint8_t;
    } else {
        source =
            BROTLI_UNALIGNED_LOAD_PTR(tail as *mut *const uint8_t as *const ::core::ffi::c_void)
                as *const uint8_t;
    }
    i = 0 as size_t;
    while i < 4 as size_t {
        let distance: size_t = *distance_cache.offset(i as isize) as size_t;
        let mut offset: size_t = 0;
        let mut limit: size_t = 0;
        let mut len: size_t = 0;
        if !(distance <= boundary || distance > distance_offset) {
            offset = distance_offset.wrapping_sub(distance);
            limit = (source_size as size_t).wrapping_sub(offset);
            limit = if limit > max_length {
                max_length
            } else {
                limit
            };
            len = FindMatchLengthWithLimit(
                source.offset(offset as isize) as *const uint8_t,
                data.offset(cur_ix_masked as isize) as *const uint8_t,
                limit,
            );
            if len >= 2 as size_t {
                let mut score: size_t = BackwardReferenceScoreUsingLastDistance(len);
                if best_score < score {
                    if i != 0 as size_t {
                        score = (score as ::core::ffi::c_ulong)
                            .wrapping_sub(BackwardReferencePenaltyUsingLastDistance(i)
                                as ::core::ffi::c_ulong) as size_t
                            as size_t;
                    }
                    if best_score < score {
                        best_score = score;
                        if len > best_len {
                            best_len = len;
                        }
                        (*out).len = len;
                        (*out).len_code_delta = 0 as ::core::ffi::c_int;
                        (*out).distance = distance;
                        (*out).score = best_score;
                    }
                }
            }
        }
        i = i.wrapping_add(1);
    }
    if best_len < 3 as size_t {
        best_len = 3 as size_t;
    }
    while item == 0 as uint32_t {
        let mut offset_0: size_t = 0;
        let mut distance_0: size_t = 0;
        let mut limit_0: size_t = 0;
        item = *chain;
        chain = chain.offset(1);
        offset_0 = (item & 0x7fffffff as uint32_t) as size_t;
        item = (item as ::core::ffi::c_uint & 0x80000000 as ::core::ffi::c_uint) as uint32_t;
        distance_0 = distance_offset.wrapping_sub(offset_0);
        limit_0 = (source_size as size_t).wrapping_sub(offset_0);
        limit_0 = if limit_0 > max_length {
            max_length
        } else {
            limit_0
        };
        if distance_0 > max_distance {
            continue;
        }
        if cur_ix_masked.wrapping_add(best_len) > ring_buffer_mask
            || best_len >= limit_0
            || BrotliUnalignedRead32(
                data.offset(
                    cur_ix_masked
                        .wrapping_add(best_len)
                        .wrapping_sub(3 as size_t) as isize,
                ) as *const uint8_t as *const ::core::ffi::c_void,
            ) != BrotliUnalignedRead32(
                source.offset(offset_0.wrapping_add(best_len).wrapping_sub(3 as size_t) as isize)
                    as *const uint8_t as *const ::core::ffi::c_void,
            )
        {
            continue;
        }
        let len_0: size_t = FindMatchLengthWithLimit(
            source.offset(offset_0 as isize) as *const uint8_t,
            data.offset(cur_ix_masked as isize) as *const uint8_t,
            limit_0,
        ) as size_t;
        if len_0 >= 4 as size_t {
            let mut score_0: size_t = BackwardReferenceScore(len_0, distance_0);
            if best_score < score_0 {
                best_score = score_0;
                best_len = len_0;
                (*out).len = best_len;
                (*out).len_code_delta = 0 as ::core::ffi::c_int;
                (*out).distance = distance_0;
                (*out).score = best_score;
            }
        }
    }
}
#[inline(always)]
unsafe extern "C" fn LookupCompoundDictionaryMatch(
    mut addon: *const CompoundDictionary,
    mut data: *const uint8_t,
    ring_buffer_mask: size_t,
    mut distance_cache: *const ::core::ffi::c_int,
    cur_ix: size_t,
    max_length: size_t,
    max_ring_buffer_distance: size_t,
    max_distance: size_t,
    mut sr: *mut HasherSearchResult,
) {
    let mut base_offset: size_t = max_ring_buffer_distance
        .wrapping_add(1 as size_t)
        .wrapping_add((*addon).total_size)
        .wrapping_sub(1 as size_t);
    let mut d: size_t = 0;
    d = 0 as size_t;
    while d < (*addon).num_chunks {
        FindCompoundDictionaryMatch(
            (*addon).chunks[d as usize],
            data,
            ring_buffer_mask,
            distance_cache,
            cur_ix,
            max_length,
            base_offset.wrapping_sub((*addon).chunk_offsets[d as usize]),
            max_distance,
            sr,
        );
        d = d.wrapping_add(1);
    }
}
pub const BUCKET_SIZE: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << BUCKET_BITS;
pub const BUCKET_MASK: ::core::ffi::c_int = BUCKET_SIZE - 1 as ::core::ffi::c_int;
pub const BUCKET_SWEEP: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << BUCKET_SWEEP_BITS;
pub const BUCKET_SWEEP_MASK: ::core::ffi::c_int =
    (BUCKET_SWEEP - 1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int;
#[inline(always)]
unsafe extern "C" fn HashTypeLengthH2() -> size_t {
    return 8 as size_t;
}
#[inline(always)]
unsafe extern "C" fn StoreLookaheadH2() -> size_t {
    return 8 as size_t;
}
unsafe extern "C" fn HashBytesH2(mut data: *const uint8_t) -> uint32_t {
    let h: uint64_t = ((BrotliUnalignedRead64(data as *const ::core::ffi::c_void) as uint64_t)
        << 64 as ::core::ffi::c_int - 8 as ::core::ffi::c_int * HASH_LEN)
        .wrapping_mul(kHashMul64);
    return (h >> 64 as ::core::ffi::c_int - BUCKET_BITS) as uint32_t;
}
#[inline(always)]
unsafe extern "C" fn StoreH2(
    mut self_0: *mut H2,
    mut data: *const uint8_t,
    mask: size_t,
    ix: size_t,
) {
    let key: uint32_t =
        HashBytesH2(data.offset((ix & mask) as isize) as *const uint8_t) as uint32_t;
    if BUCKET_SWEEP == 1 as ::core::ffi::c_int {
        *(*self_0).buckets_.offset(key as isize) = ix as uint32_t;
    } else {
        let off: uint32_t = (ix & BUCKET_SWEEP_MASK as size_t) as uint32_t;
        *(*self_0)
            .buckets_
            .offset((key.wrapping_add(off) & BUCKET_MASK as uint32_t) as isize) = ix as uint32_t;
    };
}
#[inline(always)]
unsafe extern "C" fn StoreRangeH2(
    mut self_0: *mut H2,
    mut data: *const uint8_t,
    mask: size_t,
    ix_start: size_t,
    ix_end: size_t,
) {
    let mut i: size_t = 0;
    i = ix_start;
    while i < ix_end {
        StoreH2(self_0, data, mask, i);
        i = i.wrapping_add(1);
    }
}
#[inline(always)]
unsafe extern "C" fn PrepareDistanceCacheH2(
    mut self_0: *mut H2,
    mut distance_cache: *mut ::core::ffi::c_int,
) {
}
#[inline(always)]
unsafe extern "C" fn FindLongestMatchH2(
    mut self_0: *mut H2,
    mut dictionary: *const BrotliEncoderDictionary,
    mut data: *const uint8_t,
    ring_buffer_mask: size_t,
    mut distance_cache: *const ::core::ffi::c_int,
    cur_ix: size_t,
    max_length: size_t,
    max_backward: size_t,
    dictionary_distance: size_t,
    max_distance: size_t,
    mut out: *mut HasherSearchResult,
) {
    let mut buckets: *mut uint32_t = (*self_0).buckets_;
    let best_len_in: size_t = (*out).len;
    let cur_ix_masked: size_t = cur_ix & ring_buffer_mask;
    let mut compare_char: ::core::ffi::c_int =
        *data.offset(cur_ix_masked.wrapping_add(best_len_in) as isize) as ::core::ffi::c_int;
    let mut key: size_t =
        HashBytesH2(data.offset(cur_ix_masked as isize) as *const uint8_t) as size_t;
    let mut key_out: size_t = 0;
    let mut min_score: size_t = (*out).score;
    let mut best_score: size_t = (*out).score;
    let mut best_len: size_t = best_len_in;
    let mut cached_backward: size_t =
        *distance_cache.offset(0 as ::core::ffi::c_int as isize) as size_t;
    let mut prev_ix: size_t = cur_ix.wrapping_sub(cached_backward);
    (*out).len_code_delta = 0 as ::core::ffi::c_int;
    if prev_ix < cur_ix && cached_backward <= max_backward {
        prev_ix = (prev_ix as ::core::ffi::c_ulong
            & ring_buffer_mask as uint32_t as ::core::ffi::c_ulong) as size_t;
        if compare_char
            == *data.offset(prev_ix.wrapping_add(best_len) as isize) as ::core::ffi::c_int
        {
            let len: size_t = FindMatchLengthWithLimit(
                data.offset(prev_ix as isize) as *const uint8_t,
                data.offset(cur_ix_masked as isize) as *const uint8_t,
                max_length,
            ) as size_t;
            if len >= 4 as size_t {
                let score: size_t = BackwardReferenceScoreUsingLastDistance(len) as size_t;
                if best_score < score {
                    (*out).len = len;
                    (*out).distance = cached_backward;
                    (*out).score = score;
                    if BUCKET_SWEEP == 1 as ::core::ffi::c_int {
                        *buckets.offset(key as isize) = cur_ix as uint32_t;
                        return;
                    } else {
                        best_len = len;
                        best_score = score;
                        compare_char = *data.offset(cur_ix_masked.wrapping_add(len) as isize)
                            as ::core::ffi::c_int;
                    }
                }
            }
        }
    }
    if BUCKET_SWEEP == 1 as ::core::ffi::c_int {
        let mut backward: size_t = 0;
        let mut len_0: size_t = 0;
        prev_ix = *buckets.offset(key as isize) as size_t;
        *buckets.offset(key as isize) = cur_ix as uint32_t;
        backward = cur_ix.wrapping_sub(prev_ix);
        prev_ix = (prev_ix as ::core::ffi::c_ulong
            & ring_buffer_mask as uint32_t as ::core::ffi::c_ulong) as size_t;
        if compare_char
            != *data.offset(prev_ix.wrapping_add(best_len_in) as isize) as ::core::ffi::c_int
        {
            return;
        }
        if (backward == 0 as size_t || backward > max_backward) as ::core::ffi::c_int
            as ::core::ffi::c_long
            != 0
        {
            return;
        }
        len_0 = FindMatchLengthWithLimit(
            data.offset(prev_ix as isize) as *const uint8_t,
            data.offset(cur_ix_masked as isize) as *const uint8_t,
            max_length,
        );
        if len_0 >= 4 as size_t {
            let score_0: size_t = BackwardReferenceScore(len_0, backward) as size_t;
            if best_score < score_0 {
                (*out).len = len_0;
                (*out).distance = backward;
                (*out).score = score_0;
                return;
            }
        }
    } else {
        let mut keys: [size_t; 1] = [0; 1];
        let mut i: size_t = 0;
        i = 0 as size_t;
        while i < BUCKET_SWEEP as size_t {
            keys[i as usize] =
                key.wrapping_add(i << 3 as ::core::ffi::c_int) & BUCKET_MASK as size_t;
            i = i.wrapping_add(1);
        }
        key_out =
            keys[((cur_ix & BUCKET_SWEEP_MASK as size_t) >> 3 as ::core::ffi::c_int) as usize];
        i = 0 as size_t;
        while i < BUCKET_SWEEP as size_t {
            let mut len_1: size_t = 0;
            let mut backward_0: size_t = 0;
            prev_ix = *buckets.offset(keys[i as usize] as isize) as size_t;
            backward_0 = cur_ix.wrapping_sub(prev_ix);
            prev_ix = (prev_ix as ::core::ffi::c_ulong
                & ring_buffer_mask as uint32_t as ::core::ffi::c_ulong)
                as size_t;
            if !(compare_char
                != *data.offset(prev_ix.wrapping_add(best_len) as isize) as ::core::ffi::c_int)
            {
                if !((backward_0 == 0 as size_t || backward_0 > max_backward) as ::core::ffi::c_int
                    as ::core::ffi::c_long
                    != 0)
                {
                    len_1 = FindMatchLengthWithLimit(
                        data.offset(prev_ix as isize) as *const uint8_t,
                        data.offset(cur_ix_masked as isize) as *const uint8_t,
                        max_length,
                    );
                    if len_1 >= 4 as size_t {
                        let score_1: size_t = BackwardReferenceScore(len_1, backward_0) as size_t;
                        if best_score < score_1 {
                            best_len = len_1;
                            (*out).len = len_1;
                            compare_char = *data.offset(cur_ix_masked.wrapping_add(len_1) as isize)
                                as ::core::ffi::c_int;
                            best_score = score_1;
                            (*out).score = score_1;
                            (*out).distance = backward_0;
                        }
                    }
                }
            }
            i = i.wrapping_add(1);
        }
    }
    if USE_DICTIONARY != 0 && min_score == (*out).score {
        SearchInStaticDictionary(
            dictionary,
            (*self_0).common,
            data.offset(cur_ix_masked as isize) as *const uint8_t,
            max_length,
            dictionary_distance,
            max_distance,
            out,
            BROTLI_TRUE,
        );
    }
    if BUCKET_SWEEP != 1 as ::core::ffi::c_int {
        *buckets.offset(key_out as isize) = cur_ix as uint32_t;
    }
}
pub const BUCKET_SIZE_0: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << BUCKET_BITS;
pub const BUCKET_MASK_0: ::core::ffi::c_int = BUCKET_SIZE_0 - 1 as ::core::ffi::c_int;
pub const BUCKET_SWEEP_0: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << BUCKET_SWEEP_BITS_0;
pub const BUCKET_SWEEP_MASK_0: ::core::ffi::c_int =
    (BUCKET_SWEEP_0 - 1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int;
#[inline(always)]
unsafe extern "C" fn HashTypeLengthH3() -> size_t {
    return 8 as size_t;
}
#[inline(always)]
unsafe extern "C" fn StoreLookaheadH3() -> size_t {
    return 8 as size_t;
}
unsafe extern "C" fn HashBytesH3(mut data: *const uint8_t) -> uint32_t {
    let h: uint64_t = ((BrotliUnalignedRead64(data as *const ::core::ffi::c_void) as uint64_t)
        << 64 as ::core::ffi::c_int - 8 as ::core::ffi::c_int * HASH_LEN)
        .wrapping_mul(kHashMul64);
    return (h >> 64 as ::core::ffi::c_int - BUCKET_BITS) as uint32_t;
}
#[inline(always)]
unsafe extern "C" fn StoreH3(
    mut self_0: *mut H3,
    mut data: *const uint8_t,
    mask: size_t,
    ix: size_t,
) {
    let key: uint32_t =
        HashBytesH3(data.offset((ix & mask) as isize) as *const uint8_t) as uint32_t;
    if BUCKET_SWEEP_0 == 1 as ::core::ffi::c_int {
        *(*self_0).buckets_.offset(key as isize) = ix as uint32_t;
    } else {
        let off: uint32_t = (ix & BUCKET_SWEEP_MASK_0 as size_t) as uint32_t;
        *(*self_0)
            .buckets_
            .offset((key.wrapping_add(off) & BUCKET_MASK_0 as uint32_t) as isize) = ix as uint32_t;
    };
}
#[inline(always)]
unsafe extern "C" fn StoreRangeH3(
    mut self_0: *mut H3,
    mut data: *const uint8_t,
    mask: size_t,
    ix_start: size_t,
    ix_end: size_t,
) {
    let mut i: size_t = 0;
    i = ix_start;
    while i < ix_end {
        StoreH3(self_0, data, mask, i);
        i = i.wrapping_add(1);
    }
}
#[inline(always)]
unsafe extern "C" fn PrepareDistanceCacheH3(
    mut self_0: *mut H3,
    mut distance_cache: *mut ::core::ffi::c_int,
) {
}
#[inline(always)]
unsafe extern "C" fn FindLongestMatchH3(
    mut self_0: *mut H3,
    mut dictionary: *const BrotliEncoderDictionary,
    mut data: *const uint8_t,
    ring_buffer_mask: size_t,
    mut distance_cache: *const ::core::ffi::c_int,
    cur_ix: size_t,
    max_length: size_t,
    max_backward: size_t,
    dictionary_distance: size_t,
    max_distance: size_t,
    mut out: *mut HasherSearchResult,
) {
    let mut buckets: *mut uint32_t = (*self_0).buckets_;
    let best_len_in: size_t = (*out).len;
    let cur_ix_masked: size_t = cur_ix & ring_buffer_mask;
    let mut compare_char: ::core::ffi::c_int =
        *data.offset(cur_ix_masked.wrapping_add(best_len_in) as isize) as ::core::ffi::c_int;
    let mut key: size_t =
        HashBytesH3(data.offset(cur_ix_masked as isize) as *const uint8_t) as size_t;
    let mut key_out: size_t = 0;
    let mut min_score: size_t = (*out).score;
    let mut best_score: size_t = (*out).score;
    let mut best_len: size_t = best_len_in;
    let mut cached_backward: size_t =
        *distance_cache.offset(0 as ::core::ffi::c_int as isize) as size_t;
    let mut prev_ix: size_t = cur_ix.wrapping_sub(cached_backward);
    (*out).len_code_delta = 0 as ::core::ffi::c_int;
    if prev_ix < cur_ix && cached_backward <= max_backward {
        prev_ix = (prev_ix as ::core::ffi::c_ulong
            & ring_buffer_mask as uint32_t as ::core::ffi::c_ulong) as size_t;
        if compare_char
            == *data.offset(prev_ix.wrapping_add(best_len) as isize) as ::core::ffi::c_int
        {
            let len: size_t = FindMatchLengthWithLimit(
                data.offset(prev_ix as isize) as *const uint8_t,
                data.offset(cur_ix_masked as isize) as *const uint8_t,
                max_length,
            ) as size_t;
            if len >= 4 as size_t {
                let score: size_t = BackwardReferenceScoreUsingLastDistance(len) as size_t;
                if best_score < score {
                    (*out).len = len;
                    (*out).distance = cached_backward;
                    (*out).score = score;
                    if BUCKET_SWEEP_0 == 1 as ::core::ffi::c_int {
                        *buckets.offset(key as isize) = cur_ix as uint32_t;
                        return;
                    } else {
                        best_len = len;
                        best_score = score;
                        compare_char = *data.offset(cur_ix_masked.wrapping_add(len) as isize)
                            as ::core::ffi::c_int;
                    }
                }
            }
        }
    }
    if BUCKET_SWEEP_0 == 1 as ::core::ffi::c_int {
        let mut backward: size_t = 0;
        let mut len_0: size_t = 0;
        prev_ix = *buckets.offset(key as isize) as size_t;
        *buckets.offset(key as isize) = cur_ix as uint32_t;
        backward = cur_ix.wrapping_sub(prev_ix);
        prev_ix = (prev_ix as ::core::ffi::c_ulong
            & ring_buffer_mask as uint32_t as ::core::ffi::c_ulong) as size_t;
        if compare_char
            != *data.offset(prev_ix.wrapping_add(best_len_in) as isize) as ::core::ffi::c_int
        {
            return;
        }
        if (backward == 0 as size_t || backward > max_backward) as ::core::ffi::c_int
            as ::core::ffi::c_long
            != 0
        {
            return;
        }
        len_0 = FindMatchLengthWithLimit(
            data.offset(prev_ix as isize) as *const uint8_t,
            data.offset(cur_ix_masked as isize) as *const uint8_t,
            max_length,
        );
        if len_0 >= 4 as size_t {
            let score_0: size_t = BackwardReferenceScore(len_0, backward) as size_t;
            if best_score < score_0 {
                (*out).len = len_0;
                (*out).distance = backward;
                (*out).score = score_0;
                return;
            }
        }
    } else {
        let mut keys: [size_t; 2] = [0; 2];
        let mut i: size_t = 0;
        i = 0 as size_t;
        while i < BUCKET_SWEEP_0 as size_t {
            keys[i as usize] =
                key.wrapping_add(i << 3 as ::core::ffi::c_int) & BUCKET_MASK_0 as size_t;
            i = i.wrapping_add(1);
        }
        key_out =
            keys[((cur_ix & BUCKET_SWEEP_MASK_0 as size_t) >> 3 as ::core::ffi::c_int) as usize];
        i = 0 as size_t;
        while i < BUCKET_SWEEP_0 as size_t {
            let mut len_1: size_t = 0;
            let mut backward_0: size_t = 0;
            prev_ix = *buckets.offset(keys[i as usize] as isize) as size_t;
            backward_0 = cur_ix.wrapping_sub(prev_ix);
            prev_ix = (prev_ix as ::core::ffi::c_ulong
                & ring_buffer_mask as uint32_t as ::core::ffi::c_ulong)
                as size_t;
            if !(compare_char
                != *data.offset(prev_ix.wrapping_add(best_len) as isize) as ::core::ffi::c_int)
            {
                if !((backward_0 == 0 as size_t || backward_0 > max_backward) as ::core::ffi::c_int
                    as ::core::ffi::c_long
                    != 0)
                {
                    len_1 = FindMatchLengthWithLimit(
                        data.offset(prev_ix as isize) as *const uint8_t,
                        data.offset(cur_ix_masked as isize) as *const uint8_t,
                        max_length,
                    );
                    if len_1 >= 4 as size_t {
                        let score_1: size_t = BackwardReferenceScore(len_1, backward_0) as size_t;
                        if best_score < score_1 {
                            best_len = len_1;
                            (*out).len = len_1;
                            compare_char = *data.offset(cur_ix_masked.wrapping_add(len_1) as isize)
                                as ::core::ffi::c_int;
                            best_score = score_1;
                            (*out).score = score_1;
                            (*out).distance = backward_0;
                        }
                    }
                }
            }
            i = i.wrapping_add(1);
        }
    }
    if USE_DICTIONARY_0 != 0 && min_score == (*out).score {
        SearchInStaticDictionary(
            dictionary,
            (*self_0).common,
            data.offset(cur_ix_masked as isize) as *const uint8_t,
            max_length,
            dictionary_distance,
            max_distance,
            out,
            BROTLI_TRUE,
        );
    }
    if BUCKET_SWEEP_0 != 1 as ::core::ffi::c_int {
        *buckets.offset(key_out as isize) = cur_ix as uint32_t;
    }
}
pub const BUCKET_SIZE_1: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << BUCKET_BITS_0;
pub const BUCKET_MASK_1: ::core::ffi::c_int = BUCKET_SIZE_1 - 1 as ::core::ffi::c_int;
pub const BUCKET_SWEEP_1: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << BUCKET_SWEEP_BITS_1;
pub const BUCKET_SWEEP_MASK_1: ::core::ffi::c_int =
    (BUCKET_SWEEP_1 - 1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int;
#[inline(always)]
unsafe extern "C" fn HashTypeLengthH4() -> size_t {
    return 8 as size_t;
}
#[inline(always)]
unsafe extern "C" fn StoreLookaheadH4() -> size_t {
    return 8 as size_t;
}
unsafe extern "C" fn HashBytesH4(mut data: *const uint8_t) -> uint32_t {
    let h: uint64_t = ((BrotliUnalignedRead64(data as *const ::core::ffi::c_void) as uint64_t)
        << 64 as ::core::ffi::c_int - 8 as ::core::ffi::c_int * HASH_LEN)
        .wrapping_mul(kHashMul64);
    return (h >> 64 as ::core::ffi::c_int - BUCKET_BITS_0) as uint32_t;
}
#[inline(always)]
unsafe extern "C" fn StoreH4(
    mut self_0: *mut H4,
    mut data: *const uint8_t,
    mask: size_t,
    ix: size_t,
) {
    let key: uint32_t =
        HashBytesH4(data.offset((ix & mask) as isize) as *const uint8_t) as uint32_t;
    if BUCKET_SWEEP_1 == 1 as ::core::ffi::c_int {
        *(*self_0).buckets_.offset(key as isize) = ix as uint32_t;
    } else {
        let off: uint32_t = (ix & BUCKET_SWEEP_MASK_1 as size_t) as uint32_t;
        *(*self_0)
            .buckets_
            .offset((key.wrapping_add(off) & BUCKET_MASK_1 as uint32_t) as isize) = ix as uint32_t;
    };
}
#[inline(always)]
unsafe extern "C" fn StoreRangeH4(
    mut self_0: *mut H4,
    mut data: *const uint8_t,
    mask: size_t,
    ix_start: size_t,
    ix_end: size_t,
) {
    let mut i: size_t = 0;
    i = ix_start;
    while i < ix_end {
        StoreH4(self_0, data, mask, i);
        i = i.wrapping_add(1);
    }
}
#[inline(always)]
unsafe extern "C" fn PrepareDistanceCacheH4(
    mut self_0: *mut H4,
    mut distance_cache: *mut ::core::ffi::c_int,
) {
}
#[inline(always)]
unsafe extern "C" fn FindLongestMatchH4(
    mut self_0: *mut H4,
    mut dictionary: *const BrotliEncoderDictionary,
    mut data: *const uint8_t,
    ring_buffer_mask: size_t,
    mut distance_cache: *const ::core::ffi::c_int,
    cur_ix: size_t,
    max_length: size_t,
    max_backward: size_t,
    dictionary_distance: size_t,
    max_distance: size_t,
    mut out: *mut HasherSearchResult,
) {
    let mut buckets: *mut uint32_t = (*self_0).buckets_;
    let best_len_in: size_t = (*out).len;
    let cur_ix_masked: size_t = cur_ix & ring_buffer_mask;
    let mut compare_char: ::core::ffi::c_int =
        *data.offset(cur_ix_masked.wrapping_add(best_len_in) as isize) as ::core::ffi::c_int;
    let mut key: size_t =
        HashBytesH4(data.offset(cur_ix_masked as isize) as *const uint8_t) as size_t;
    let mut key_out: size_t = 0;
    let mut min_score: size_t = (*out).score;
    let mut best_score: size_t = (*out).score;
    let mut best_len: size_t = best_len_in;
    let mut cached_backward: size_t =
        *distance_cache.offset(0 as ::core::ffi::c_int as isize) as size_t;
    let mut prev_ix: size_t = cur_ix.wrapping_sub(cached_backward);
    (*out).len_code_delta = 0 as ::core::ffi::c_int;
    if prev_ix < cur_ix && cached_backward <= max_backward {
        prev_ix = (prev_ix as ::core::ffi::c_ulong
            & ring_buffer_mask as uint32_t as ::core::ffi::c_ulong) as size_t;
        if compare_char
            == *data.offset(prev_ix.wrapping_add(best_len) as isize) as ::core::ffi::c_int
        {
            let len: size_t = FindMatchLengthWithLimit(
                data.offset(prev_ix as isize) as *const uint8_t,
                data.offset(cur_ix_masked as isize) as *const uint8_t,
                max_length,
            ) as size_t;
            if len >= 4 as size_t {
                let score: size_t = BackwardReferenceScoreUsingLastDistance(len) as size_t;
                if best_score < score {
                    (*out).len = len;
                    (*out).distance = cached_backward;
                    (*out).score = score;
                    if BUCKET_SWEEP_1 == 1 as ::core::ffi::c_int {
                        *buckets.offset(key as isize) = cur_ix as uint32_t;
                        return;
                    } else {
                        best_len = len;
                        best_score = score;
                        compare_char = *data.offset(cur_ix_masked.wrapping_add(len) as isize)
                            as ::core::ffi::c_int;
                    }
                }
            }
        }
    }
    if BUCKET_SWEEP_1 == 1 as ::core::ffi::c_int {
        let mut backward: size_t = 0;
        let mut len_0: size_t = 0;
        prev_ix = *buckets.offset(key as isize) as size_t;
        *buckets.offset(key as isize) = cur_ix as uint32_t;
        backward = cur_ix.wrapping_sub(prev_ix);
        prev_ix = (prev_ix as ::core::ffi::c_ulong
            & ring_buffer_mask as uint32_t as ::core::ffi::c_ulong) as size_t;
        if compare_char
            != *data.offset(prev_ix.wrapping_add(best_len_in) as isize) as ::core::ffi::c_int
        {
            return;
        }
        if (backward == 0 as size_t || backward > max_backward) as ::core::ffi::c_int
            as ::core::ffi::c_long
            != 0
        {
            return;
        }
        len_0 = FindMatchLengthWithLimit(
            data.offset(prev_ix as isize) as *const uint8_t,
            data.offset(cur_ix_masked as isize) as *const uint8_t,
            max_length,
        );
        if len_0 >= 4 as size_t {
            let score_0: size_t = BackwardReferenceScore(len_0, backward) as size_t;
            if best_score < score_0 {
                (*out).len = len_0;
                (*out).distance = backward;
                (*out).score = score_0;
                return;
            }
        }
    } else {
        let mut keys: [size_t; 4] = [0; 4];
        let mut i: size_t = 0;
        i = 0 as size_t;
        while i < BUCKET_SWEEP_1 as size_t {
            keys[i as usize] =
                key.wrapping_add(i << 3 as ::core::ffi::c_int) & BUCKET_MASK_1 as size_t;
            i = i.wrapping_add(1);
        }
        key_out =
            keys[((cur_ix & BUCKET_SWEEP_MASK_1 as size_t) >> 3 as ::core::ffi::c_int) as usize];
        i = 0 as size_t;
        while i < BUCKET_SWEEP_1 as size_t {
            let mut len_1: size_t = 0;
            let mut backward_0: size_t = 0;
            prev_ix = *buckets.offset(keys[i as usize] as isize) as size_t;
            backward_0 = cur_ix.wrapping_sub(prev_ix);
            prev_ix = (prev_ix as ::core::ffi::c_ulong
                & ring_buffer_mask as uint32_t as ::core::ffi::c_ulong)
                as size_t;
            if !(compare_char
                != *data.offset(prev_ix.wrapping_add(best_len) as isize) as ::core::ffi::c_int)
            {
                if !((backward_0 == 0 as size_t || backward_0 > max_backward) as ::core::ffi::c_int
                    as ::core::ffi::c_long
                    != 0)
                {
                    len_1 = FindMatchLengthWithLimit(
                        data.offset(prev_ix as isize) as *const uint8_t,
                        data.offset(cur_ix_masked as isize) as *const uint8_t,
                        max_length,
                    );
                    if len_1 >= 4 as size_t {
                        let score_1: size_t = BackwardReferenceScore(len_1, backward_0) as size_t;
                        if best_score < score_1 {
                            best_len = len_1;
                            (*out).len = len_1;
                            compare_char = *data.offset(cur_ix_masked.wrapping_add(len_1) as isize)
                                as ::core::ffi::c_int;
                            best_score = score_1;
                            (*out).score = score_1;
                            (*out).distance = backward_0;
                        }
                    }
                }
            }
            i = i.wrapping_add(1);
        }
    }
    if USE_DICTIONARY_1 != 0 && min_score == (*out).score {
        SearchInStaticDictionary(
            dictionary,
            (*self_0).common,
            data.offset(cur_ix_masked as isize) as *const uint8_t,
            max_length,
            dictionary_distance,
            max_distance,
            out,
            BROTLI_TRUE,
        );
    }
    if BUCKET_SWEEP_1 != 1 as ::core::ffi::c_int {
        *buckets.offset(key_out as isize) = cur_ix as uint32_t;
    }
}
pub const BUCKET_SIZE_5: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << BUCKET_BITS_2;
pub const BUCKET_MASK_2: ::core::ffi::c_int = BUCKET_SIZE_5 - 1 as ::core::ffi::c_int;
pub const BUCKET_SWEEP_2: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << BUCKET_SWEEP_BITS_2;
pub const BUCKET_SWEEP_MASK_2: ::core::ffi::c_int =
    (BUCKET_SWEEP_2 - 1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int;
#[inline(always)]
unsafe extern "C" fn HashTypeLengthH54() -> size_t {
    return 8 as size_t;
}
#[inline(always)]
unsafe extern "C" fn StoreLookaheadH54() -> size_t {
    return 8 as size_t;
}
unsafe extern "C" fn HashBytesH54(mut data: *const uint8_t) -> uint32_t {
    let h: uint64_t = ((BrotliUnalignedRead64(data as *const ::core::ffi::c_void) as uint64_t)
        << 64 as ::core::ffi::c_int - 8 as ::core::ffi::c_int * HASH_LEN_0)
        .wrapping_mul(kHashMul64);
    return (h >> 64 as ::core::ffi::c_int - BUCKET_BITS_2) as uint32_t;
}
#[inline(always)]
unsafe extern "C" fn StoreH54(
    mut self_0: *mut H54,
    mut data: *const uint8_t,
    mask: size_t,
    ix: size_t,
) {
    let key: uint32_t =
        HashBytesH54(data.offset((ix & mask) as isize) as *const uint8_t) as uint32_t;
    if BUCKET_SWEEP_2 == 1 as ::core::ffi::c_int {
        *(*self_0).buckets_.offset(key as isize) = ix as uint32_t;
    } else {
        let off: uint32_t = (ix & BUCKET_SWEEP_MASK_2 as size_t) as uint32_t;
        *(*self_0)
            .buckets_
            .offset((key.wrapping_add(off) & BUCKET_MASK_2 as uint32_t) as isize) = ix as uint32_t;
    };
}
#[inline(always)]
unsafe extern "C" fn StoreRangeH54(
    mut self_0: *mut H54,
    mut data: *const uint8_t,
    mask: size_t,
    ix_start: size_t,
    ix_end: size_t,
) {
    let mut i: size_t = 0;
    i = ix_start;
    while i < ix_end {
        StoreH54(self_0, data, mask, i);
        i = i.wrapping_add(1);
    }
}
#[inline(always)]
unsafe extern "C" fn PrepareDistanceCacheH54(
    mut self_0: *mut H54,
    mut distance_cache: *mut ::core::ffi::c_int,
) {
}
#[inline(always)]
unsafe extern "C" fn FindLongestMatchH54(
    mut self_0: *mut H54,
    mut dictionary: *const BrotliEncoderDictionary,
    mut data: *const uint8_t,
    ring_buffer_mask: size_t,
    mut distance_cache: *const ::core::ffi::c_int,
    cur_ix: size_t,
    max_length: size_t,
    max_backward: size_t,
    dictionary_distance: size_t,
    max_distance: size_t,
    mut out: *mut HasherSearchResult,
) {
    let mut buckets: *mut uint32_t = (*self_0).buckets_;
    let best_len_in: size_t = (*out).len;
    let cur_ix_masked: size_t = cur_ix & ring_buffer_mask;
    let mut compare_char: ::core::ffi::c_int =
        *data.offset(cur_ix_masked.wrapping_add(best_len_in) as isize) as ::core::ffi::c_int;
    let mut key: size_t =
        HashBytesH54(data.offset(cur_ix_masked as isize) as *const uint8_t) as size_t;
    let mut key_out: size_t = 0;
    let mut min_score: size_t = (*out).score;
    let mut best_score: size_t = (*out).score;
    let mut best_len: size_t = best_len_in;
    let mut cached_backward: size_t =
        *distance_cache.offset(0 as ::core::ffi::c_int as isize) as size_t;
    let mut prev_ix: size_t = cur_ix.wrapping_sub(cached_backward);
    (*out).len_code_delta = 0 as ::core::ffi::c_int;
    if prev_ix < cur_ix && cached_backward <= max_backward {
        prev_ix = (prev_ix as ::core::ffi::c_ulong
            & ring_buffer_mask as uint32_t as ::core::ffi::c_ulong) as size_t;
        if compare_char
            == *data.offset(prev_ix.wrapping_add(best_len) as isize) as ::core::ffi::c_int
        {
            let len: size_t = FindMatchLengthWithLimit(
                data.offset(prev_ix as isize) as *const uint8_t,
                data.offset(cur_ix_masked as isize) as *const uint8_t,
                max_length,
            ) as size_t;
            if len >= 4 as size_t {
                let score: size_t = BackwardReferenceScoreUsingLastDistance(len) as size_t;
                if best_score < score {
                    (*out).len = len;
                    (*out).distance = cached_backward;
                    (*out).score = score;
                    if BUCKET_SWEEP_2 == 1 as ::core::ffi::c_int {
                        *buckets.offset(key as isize) = cur_ix as uint32_t;
                        return;
                    } else {
                        best_len = len;
                        best_score = score;
                        compare_char = *data.offset(cur_ix_masked.wrapping_add(len) as isize)
                            as ::core::ffi::c_int;
                    }
                }
            }
        }
    }
    if BUCKET_SWEEP_2 == 1 as ::core::ffi::c_int {
        let mut backward: size_t = 0;
        let mut len_0: size_t = 0;
        prev_ix = *buckets.offset(key as isize) as size_t;
        *buckets.offset(key as isize) = cur_ix as uint32_t;
        backward = cur_ix.wrapping_sub(prev_ix);
        prev_ix = (prev_ix as ::core::ffi::c_ulong
            & ring_buffer_mask as uint32_t as ::core::ffi::c_ulong) as size_t;
        if compare_char
            != *data.offset(prev_ix.wrapping_add(best_len_in) as isize) as ::core::ffi::c_int
        {
            return;
        }
        if (backward == 0 as size_t || backward > max_backward) as ::core::ffi::c_int
            as ::core::ffi::c_long
            != 0
        {
            return;
        }
        len_0 = FindMatchLengthWithLimit(
            data.offset(prev_ix as isize) as *const uint8_t,
            data.offset(cur_ix_masked as isize) as *const uint8_t,
            max_length,
        );
        if len_0 >= 4 as size_t {
            let score_0: size_t = BackwardReferenceScore(len_0, backward) as size_t;
            if best_score < score_0 {
                (*out).len = len_0;
                (*out).distance = backward;
                (*out).score = score_0;
                return;
            }
        }
    } else {
        let mut keys: [size_t; 4] = [0; 4];
        let mut i: size_t = 0;
        i = 0 as size_t;
        while i < BUCKET_SWEEP_2 as size_t {
            keys[i as usize] =
                key.wrapping_add(i << 3 as ::core::ffi::c_int) & BUCKET_MASK_2 as size_t;
            i = i.wrapping_add(1);
        }
        key_out =
            keys[((cur_ix & BUCKET_SWEEP_MASK_2 as size_t) >> 3 as ::core::ffi::c_int) as usize];
        i = 0 as size_t;
        while i < BUCKET_SWEEP_2 as size_t {
            let mut len_1: size_t = 0;
            let mut backward_0: size_t = 0;
            prev_ix = *buckets.offset(keys[i as usize] as isize) as size_t;
            backward_0 = cur_ix.wrapping_sub(prev_ix);
            prev_ix = (prev_ix as ::core::ffi::c_ulong
                & ring_buffer_mask as uint32_t as ::core::ffi::c_ulong)
                as size_t;
            if !(compare_char
                != *data.offset(prev_ix.wrapping_add(best_len) as isize) as ::core::ffi::c_int)
            {
                if !((backward_0 == 0 as size_t || backward_0 > max_backward) as ::core::ffi::c_int
                    as ::core::ffi::c_long
                    != 0)
                {
                    len_1 = FindMatchLengthWithLimit(
                        data.offset(prev_ix as isize) as *const uint8_t,
                        data.offset(cur_ix_masked as isize) as *const uint8_t,
                        max_length,
                    );
                    if len_1 >= 4 as size_t {
                        let score_1: size_t = BackwardReferenceScore(len_1, backward_0) as size_t;
                        if best_score < score_1 {
                            best_len = len_1;
                            (*out).len = len_1;
                            compare_char = *data.offset(cur_ix_masked.wrapping_add(len_1) as isize)
                                as ::core::ffi::c_int;
                            best_score = score_1;
                            (*out).score = score_1;
                            (*out).distance = backward_0;
                        }
                    }
                }
            }
            i = i.wrapping_add(1);
        }
    }
    if USE_DICTIONARY_2 != 0 && min_score == (*out).score {
        SearchInStaticDictionary(
            dictionary,
            (*self_0).common,
            data.offset(cur_ix_masked as isize) as *const uint8_t,
            max_length,
            dictionary_distance,
            max_distance,
            out,
            BROTLI_TRUE,
        );
    }
    if BUCKET_SWEEP_2 != 1 as ::core::ffi::c_int {
        *buckets.offset(key_out as isize) = cur_ix as uint32_t;
    }
}
#[inline(always)]
unsafe extern "C" fn HashTypeLengthH5() -> size_t {
    return 4 as size_t;
}
#[inline(always)]
unsafe extern "C" fn StoreLookaheadH5() -> size_t {
    return 4 as size_t;
}
unsafe extern "C" fn HashBytesH5(mut data: *const uint8_t, shift: ::core::ffi::c_int) -> uint32_t {
    let mut h: uint32_t =
        BrotliUnalignedRead32(data as *const ::core::ffi::c_void).wrapping_mul(kHashMul32);
    return h >> shift;
}
#[inline(always)]
unsafe extern "C" fn StoreH5(
    mut self_0: *mut H5,
    mut data: *const uint8_t,
    mask: size_t,
    ix: size_t,
) {
    let mut num: *mut uint16_t = (*self_0).num_;
    let mut buckets: *mut uint32_t = (*self_0).buckets_;
    let key: uint32_t = HashBytesH5(
        data.offset((ix & mask) as isize) as *const uint8_t,
        (*self_0).hash_shift_,
    ) as uint32_t;
    let minor_ix: size_t =
        (*num.offset(key as isize) as uint32_t & (*self_0).block_mask_) as size_t;
    let offset: size_t = minor_ix.wrapping_add((key << (*self_0).block_bits_) as size_t);
    let ref mut fresh0 = *num.offset(key as isize);
    *fresh0 = (*fresh0).wrapping_add(1);
    *buckets.offset(offset as isize) = ix as uint32_t;
}
#[inline(always)]
unsafe extern "C" fn StoreRangeH5(
    mut self_0: *mut H5,
    mut data: *const uint8_t,
    mask: size_t,
    ix_start: size_t,
    ix_end: size_t,
) {
    let mut i: size_t = 0;
    i = ix_start;
    while i < ix_end {
        StoreH5(self_0, data, mask, i);
        i = i.wrapping_add(1);
    }
}
#[inline(always)]
unsafe extern "C" fn PrepareDistanceCacheH5(
    mut self_0: *mut H5,
    mut distance_cache: *mut ::core::ffi::c_int,
) {
    PrepareDistanceCache(distance_cache, (*self_0).num_last_distances_to_check_);
}
#[inline(always)]
unsafe extern "C" fn FindLongestMatchH5(
    mut self_0: *mut H5,
    mut dictionary: *const BrotliEncoderDictionary,
    mut data: *const uint8_t,
    ring_buffer_mask: size_t,
    mut distance_cache: *const ::core::ffi::c_int,
    cur_ix: size_t,
    max_length: size_t,
    max_backward: size_t,
    dictionary_distance: size_t,
    max_distance: size_t,
    mut out: *mut HasherSearchResult,
) {
    let mut num: *mut uint16_t = (*self_0).num_;
    let mut buckets: *mut uint32_t = (*self_0).buckets_;
    let cur_ix_masked: size_t = cur_ix & ring_buffer_mask;
    let mut min_score: size_t = (*out).score;
    let mut best_score: size_t = (*out).score;
    let mut best_len: size_t = (*out).len;
    let mut i: size_t = 0;
    let key: uint32_t = HashBytesH5(
        data.offset(cur_ix_masked as isize) as *const uint8_t,
        (*self_0).hash_shift_,
    ) as uint32_t;
    let mut bucket: *mut uint32_t =
        buckets.offset((key << (*self_0).block_bits_) as isize) as *mut uint32_t;
    (*self_0).block_bits_ > 4 as ::core::ffi::c_int;
    (*out).len = 0 as size_t;
    (*out).len_code_delta = 0 as ::core::ffi::c_int;
    i = 0 as size_t;
    while i < (*self_0).num_last_distances_to_check_ as size_t {
        let backward: size_t = *distance_cache.offset(i as isize) as size_t;
        let mut prev_ix: size_t = cur_ix.wrapping_sub(backward);
        if !(prev_ix >= cur_ix) {
            if !((backward > max_backward) as ::core::ffi::c_int as ::core::ffi::c_long != 0) {
                prev_ix = (prev_ix as ::core::ffi::c_ulong
                    & ring_buffer_mask as ::core::ffi::c_ulong) as size_t;
                if cur_ix_masked.wrapping_add(best_len) > ring_buffer_mask {
                    break;
                }
                if !(prev_ix.wrapping_add(best_len) > ring_buffer_mask
                    || *data.offset(cur_ix_masked.wrapping_add(best_len) as isize)
                        as ::core::ffi::c_int
                        != *data.offset(prev_ix.wrapping_add(best_len) as isize)
                            as ::core::ffi::c_int)
                {
                    let len: size_t = FindMatchLengthWithLimit(
                        data.offset(prev_ix as isize) as *const uint8_t,
                        data.offset(cur_ix_masked as isize) as *const uint8_t,
                        max_length,
                    ) as size_t;
                    if len >= 3 as size_t || len == 2 as size_t && i < 2 as size_t {
                        let mut score: size_t = BackwardReferenceScoreUsingLastDistance(len);
                        if best_score < score {
                            if i != 0 as size_t {
                                score = (score as ::core::ffi::c_ulong)
                                    .wrapping_sub(BackwardReferencePenaltyUsingLastDistance(i)
                                        as ::core::ffi::c_ulong)
                                    as size_t as size_t;
                            }
                            if best_score < score {
                                best_score = score;
                                best_len = len;
                                (*out).len = best_len;
                                (*out).distance = backward;
                                (*out).score = best_score;
                            }
                        }
                    }
                }
            }
        }
        i = i.wrapping_add(1);
    }
    if best_len < 3 as size_t {
        best_len = 3 as size_t;
    }
    let down: size_t = if *num.offset(key as isize) as size_t > (*self_0).block_size_ {
        (*num.offset(key as isize) as size_t).wrapping_sub((*self_0).block_size_)
    } else {
        0 as size_t
    };
    i = *num.offset(key as isize) as size_t;
    while i > down {
        i = i.wrapping_sub(1);
        let mut prev_ix_0: size_t =
            *bucket.offset((i & (*self_0).block_mask_ as size_t) as isize) as size_t;
        let backward_0: size_t = cur_ix.wrapping_sub(prev_ix_0);
        if (backward_0 > max_backward) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
            break;
        }
        prev_ix_0 = (prev_ix_0 as ::core::ffi::c_ulong & ring_buffer_mask as ::core::ffi::c_ulong)
            as size_t;
        if cur_ix_masked.wrapping_add(best_len) > ring_buffer_mask {
            break;
        }
        if prev_ix_0.wrapping_add(best_len) > ring_buffer_mask
            || BrotliUnalignedRead32(
                data.offset(
                    cur_ix_masked
                        .wrapping_add(best_len)
                        .wrapping_sub(3 as size_t) as isize,
                ) as *const uint8_t as *const ::core::ffi::c_void,
            ) != BrotliUnalignedRead32(
                data.offset(prev_ix_0.wrapping_add(best_len).wrapping_sub(3 as size_t) as isize)
                    as *const uint8_t as *const ::core::ffi::c_void,
            )
        {
            continue;
        }
        let len_0: size_t = FindMatchLengthWithLimit(
            data.offset(prev_ix_0 as isize) as *const uint8_t,
            data.offset(cur_ix_masked as isize) as *const uint8_t,
            max_length,
        ) as size_t;
        if len_0 >= 4 as size_t {
            let mut score_0: size_t = BackwardReferenceScore(len_0, backward_0);
            if best_score < score_0 {
                best_score = score_0;
                best_len = len_0;
                (*out).len = best_len;
                (*out).distance = backward_0;
                (*out).score = best_score;
            }
        }
    }
    *bucket.offset((*num.offset(key as isize) as uint32_t & (*self_0).block_mask_) as isize) =
        cur_ix as uint32_t;
    let ref mut fresh1 = *num.offset(key as isize);
    *fresh1 = (*fresh1).wrapping_add(1);
    if min_score == (*out).score {
        SearchInStaticDictionary(
            dictionary,
            (*self_0).common_,
            data.offset(cur_ix_masked as isize) as *const uint8_t,
            max_length,
            dictionary_distance,
            max_distance,
            out,
            BROTLI_FALSE,
        );
    }
}
#[inline(always)]
unsafe extern "C" fn HashTypeLengthH6() -> size_t {
    return 8 as size_t;
}
#[inline(always)]
unsafe extern "C" fn StoreLookaheadH6() -> size_t {
    return 8 as size_t;
}
#[inline(always)]
unsafe extern "C" fn HashBytesH6(mut data: *const uint8_t, mut hash_mul: uint64_t) -> size_t {
    let h: uint64_t = (BrotliUnalignedRead64(data as *const ::core::ffi::c_void) as uint64_t)
        .wrapping_mul(hash_mul);
    return (h >> 64 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as size_t;
}
#[inline(always)]
unsafe extern "C" fn StoreH6(
    mut self_0: *mut H6,
    mut data: *const uint8_t,
    mask: size_t,
    ix: size_t,
) {
    let mut num: *mut uint16_t = (*self_0).num_;
    let mut buckets: *mut uint32_t = (*self_0).buckets_;
    let key: size_t = HashBytesH6(
        data.offset((ix & mask) as isize) as *const uint8_t,
        (*self_0).hash_mul_,
    ) as size_t;
    let minor_ix: size_t =
        (*num.offset(key as isize) as uint32_t & (*self_0).block_mask_) as size_t;
    let offset: size_t = minor_ix.wrapping_add(key << (*self_0).block_bits_);
    let ref mut fresh2 = *num.offset(key as isize);
    *fresh2 = (*fresh2).wrapping_add(1);
    *buckets.offset(offset as isize) = ix as uint32_t;
}
#[inline(always)]
unsafe extern "C" fn StoreRangeH6(
    mut self_0: *mut H6,
    mut data: *const uint8_t,
    mask: size_t,
    ix_start: size_t,
    ix_end: size_t,
) {
    let mut i: size_t = 0;
    i = ix_start;
    while i < ix_end {
        StoreH6(self_0, data, mask, i);
        i = i.wrapping_add(1);
    }
}
#[inline(always)]
unsafe extern "C" fn PrepareDistanceCacheH6(
    mut self_0: *mut H6,
    mut distance_cache: *mut ::core::ffi::c_int,
) {
    PrepareDistanceCache(distance_cache, (*self_0).num_last_distances_to_check_);
}
#[inline(always)]
unsafe extern "C" fn FindLongestMatchH6(
    mut self_0: *mut H6,
    mut dictionary: *const BrotliEncoderDictionary,
    mut data: *const uint8_t,
    ring_buffer_mask: size_t,
    mut distance_cache: *const ::core::ffi::c_int,
    cur_ix: size_t,
    max_length: size_t,
    max_backward: size_t,
    dictionary_distance: size_t,
    max_distance: size_t,
    mut out: *mut HasherSearchResult,
) {
    let mut num: *mut uint16_t = (*self_0).num_;
    let mut buckets: *mut uint32_t = (*self_0).buckets_;
    let cur_ix_masked: size_t = cur_ix & ring_buffer_mask;
    let mut min_score: size_t = (*out).score;
    let mut best_score: size_t = (*out).score;
    let mut best_len: size_t = (*out).len;
    let mut i: size_t = 0;
    let key: size_t = HashBytesH6(
        data.offset(cur_ix_masked as isize) as *const uint8_t,
        (*self_0).hash_mul_,
    ) as size_t;
    let mut bucket: *mut uint32_t =
        buckets.offset((key << (*self_0).block_bits_) as isize) as *mut uint32_t;
    (*self_0).block_bits_ > 4 as ::core::ffi::c_int;
    (*out).len = 0 as size_t;
    (*out).len_code_delta = 0 as ::core::ffi::c_int;
    i = 0 as size_t;
    while i < (*self_0).num_last_distances_to_check_ as size_t {
        let backward: size_t = *distance_cache.offset(i as isize) as size_t;
        let mut prev_ix: size_t = cur_ix.wrapping_sub(backward);
        if !(prev_ix >= cur_ix) {
            if !((backward > max_backward) as ::core::ffi::c_int as ::core::ffi::c_long != 0) {
                prev_ix = (prev_ix as ::core::ffi::c_ulong
                    & ring_buffer_mask as ::core::ffi::c_ulong) as size_t;
                if cur_ix_masked.wrapping_add(best_len) > ring_buffer_mask {
                    break;
                }
                if !(prev_ix.wrapping_add(best_len) > ring_buffer_mask
                    || *data.offset(cur_ix_masked.wrapping_add(best_len) as isize)
                        as ::core::ffi::c_int
                        != *data.offset(prev_ix.wrapping_add(best_len) as isize)
                            as ::core::ffi::c_int)
                {
                    let len: size_t = FindMatchLengthWithLimit(
                        data.offset(prev_ix as isize) as *const uint8_t,
                        data.offset(cur_ix_masked as isize) as *const uint8_t,
                        max_length,
                    ) as size_t;
                    if len >= 3 as size_t || len == 2 as size_t && i < 2 as size_t {
                        let mut score: size_t = BackwardReferenceScoreUsingLastDistance(len);
                        if best_score < score {
                            if i != 0 as size_t {
                                score = (score as ::core::ffi::c_ulong)
                                    .wrapping_sub(BackwardReferencePenaltyUsingLastDistance(i)
                                        as ::core::ffi::c_ulong)
                                    as size_t as size_t;
                            }
                            if best_score < score {
                                best_score = score;
                                best_len = len;
                                (*out).len = best_len;
                                (*out).distance = backward;
                                (*out).score = best_score;
                            }
                        }
                    }
                }
            }
        }
        i = i.wrapping_add(1);
    }
    if best_len < 3 as size_t {
        best_len = 3 as size_t;
    }
    let down: size_t = if *num.offset(key as isize) as size_t > (*self_0).block_size_ {
        (*num.offset(key as isize) as size_t).wrapping_sub((*self_0).block_size_)
    } else {
        0 as size_t
    };
    let first4: uint32_t =
        BrotliUnalignedRead32(data.offset(cur_ix_masked as isize) as *const ::core::ffi::c_void)
            as uint32_t;
    let max_length_m4: size_t = max_length.wrapping_sub(4 as size_t);
    i = *num.offset(key as isize) as size_t;
    while i > down {
        i = i.wrapping_sub(1);
        let mut prev_ix_0: size_t =
            *bucket.offset((i & (*self_0).block_mask_ as size_t) as isize) as size_t;
        let mut current4: uint32_t = 0;
        let backward_0: size_t = cur_ix.wrapping_sub(prev_ix_0);
        if (backward_0 > max_backward) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
            break;
        }
        prev_ix_0 = (prev_ix_0 as ::core::ffi::c_ulong & ring_buffer_mask as ::core::ffi::c_ulong)
            as size_t;
        if cur_ix_masked.wrapping_add(best_len) > ring_buffer_mask {
            break;
        }
        if prev_ix_0.wrapping_add(best_len) > ring_buffer_mask
            || BrotliUnalignedRead32(
                data.offset(
                    cur_ix_masked
                        .wrapping_add(best_len)
                        .wrapping_sub(3 as size_t) as isize,
                ) as *const uint8_t as *const ::core::ffi::c_void,
            ) != BrotliUnalignedRead32(
                data.offset(prev_ix_0.wrapping_add(best_len).wrapping_sub(3 as size_t) as isize)
                    as *const uint8_t as *const ::core::ffi::c_void,
            )
        {
            continue;
        }
        current4 =
            BrotliUnalignedRead32(data.offset(prev_ix_0 as isize) as *const ::core::ffi::c_void);
        if first4 != current4 {
            continue;
        }
        let len_0: size_t = (FindMatchLengthWithLimit(
            data.offset(prev_ix_0.wrapping_add(4 as size_t) as isize) as *const uint8_t,
            data.offset(cur_ix_masked.wrapping_add(4 as size_t) as isize) as *const uint8_t,
            max_length_m4,
        ) as size_t)
            .wrapping_add(4 as size_t);
        let score_0: size_t = BackwardReferenceScore(len_0, backward_0) as size_t;
        if best_score < score_0 {
            best_score = score_0;
            best_len = len_0;
            (*out).len = best_len;
            (*out).distance = backward_0;
            (*out).score = best_score;
        }
    }
    *bucket.offset((*num.offset(key as isize) as uint32_t & (*self_0).block_mask_) as isize) =
        cur_ix as uint32_t;
    let ref mut fresh3 = *num.offset(key as isize);
    *fresh3 = (*fresh3).wrapping_add(1);
    if min_score == (*out).score {
        SearchInStaticDictionary(
            dictionary,
            (*self_0).common_,
            data.offset(cur_ix_masked as isize) as *const uint8_t,
            max_length,
            dictionary_distance,
            max_distance,
            out,
            BROTLI_FALSE,
        );
    }
}
pub const TAG_HASH_BITS: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const TAG_HASH_MASK: ::core::ffi::c_int =
    ((1 as ::core::ffi::c_int) << TAG_HASH_BITS) - 1 as ::core::ffi::c_int;
#[inline(always)]
unsafe extern "C" fn HashTypeLengthH58() -> size_t {
    return 4 as size_t;
}
#[inline(always)]
unsafe extern "C" fn StoreLookaheadH58() -> size_t {
    return 4 as size_t;
}
unsafe extern "C" fn HashBytesH58(mut data: *const uint8_t, shift: ::core::ffi::c_int) -> uint32_t {
    let mut h: uint32_t =
        BrotliUnalignedRead32(data as *const ::core::ffi::c_void).wrapping_mul(kHashMul32);
    return h >> shift;
}
#[inline(always)]
unsafe extern "C" fn StoreH58(
    mut self_0: *mut H58,
    mut data: *const uint8_t,
    mask: size_t,
    ix: size_t,
) {
    let mut num: *mut uint16_t = (*self_0).num_;
    let mut tags: *mut uint8_t = (*self_0).tags_;
    let mut buckets: *mut uint32_t = (*self_0).buckets_;
    let hash: size_t = HashBytesH58(
        data.offset((ix & mask) as isize) as *const uint8_t,
        (*self_0).hash_shift_,
    ) as size_t;
    let key: size_t = hash >> TAG_HASH_BITS;
    let tag: uint8_t = (hash & TAG_HASH_MASK as size_t) as uint8_t;
    let minor_ix: size_t =
        (*num.offset(key as isize) as uint32_t & (*self_0).block_mask_) as size_t;
    let offset: size_t = minor_ix.wrapping_add(key << (*self_0).block_bits_);
    let ref mut fresh4 = *num.offset(key as isize);
    *fresh4 = (*fresh4).wrapping_sub(1);
    *buckets.offset(offset as isize) = ix as uint32_t;
    *tags.offset(offset as isize) = tag;
}
#[inline(always)]
unsafe extern "C" fn StoreRangeH58(
    mut self_0: *mut H58,
    mut data: *const uint8_t,
    mask: size_t,
    ix_start: size_t,
    ix_end: size_t,
) {
    let mut i: size_t = 0;
    i = ix_start;
    while i < ix_end {
        StoreH58(self_0, data, mask, i);
        i = i.wrapping_add(1);
    }
}
#[inline(always)]
unsafe extern "C" fn PrepareDistanceCacheH58(
    mut self_0: *mut H58,
    mut distance_cache: *mut ::core::ffi::c_int,
) {
    PrepareDistanceCache(distance_cache, (*self_0).num_last_distances_to_check_);
}
#[inline(always)]
unsafe extern "C" fn FindLongestMatchH58(
    mut self_0: *mut H58,
    mut dictionary: *const BrotliEncoderDictionary,
    mut data: *const uint8_t,
    ring_buffer_mask: size_t,
    mut distance_cache: *const ::core::ffi::c_int,
    cur_ix: size_t,
    max_length: size_t,
    max_backward: size_t,
    dictionary_distance: size_t,
    max_distance: size_t,
    mut out: *mut HasherSearchResult,
) {
    let mut num: *mut uint16_t = (*self_0).num_;
    let mut buckets: *mut uint32_t = (*self_0).buckets_;
    let mut tags: *mut uint8_t = (*self_0).tags_;
    let cur_ix_masked: size_t = cur_ix & ring_buffer_mask;
    let mut min_score: size_t = (*out).score;
    let mut best_score: size_t = (*out).score;
    let mut best_len: size_t = (*out).len;
    let mut i: size_t = 0;
    let hash: uint32_t = HashBytesH58(
        data.offset(cur_ix_masked as isize) as *const uint8_t,
        (*self_0).hash_shift_,
    ) as uint32_t;
    let key: uint32_t = hash >> TAG_HASH_BITS;
    let mut bucket: *mut uint32_t =
        buckets.offset((key << (*self_0).block_bits_) as isize) as *mut uint32_t;
    let mut tag_bucket: *mut uint8_t =
        tags.offset((key << (*self_0).block_bits_) as isize) as *mut uint8_t;
    (*self_0).block_bits_ > 4 as ::core::ffi::c_int;
    (*out).len = 0 as size_t;
    (*out).len_code_delta = 0 as ::core::ffi::c_int;
    i = 0 as size_t;
    while i < (*self_0).num_last_distances_to_check_ as size_t {
        let backward: size_t = *distance_cache.offset(i as isize) as size_t;
        let mut prev_ix: size_t = cur_ix.wrapping_sub(backward);
        if !(prev_ix >= cur_ix) {
            if !((backward > max_backward) as ::core::ffi::c_int as ::core::ffi::c_long != 0) {
                prev_ix = (prev_ix as ::core::ffi::c_ulong
                    & ring_buffer_mask as ::core::ffi::c_ulong) as size_t;
                if cur_ix_masked.wrapping_add(best_len) > ring_buffer_mask {
                    break;
                }
                if !(prev_ix.wrapping_add(best_len) > ring_buffer_mask
                    || *data.offset(cur_ix_masked.wrapping_add(best_len) as isize)
                        as ::core::ffi::c_int
                        != *data.offset(prev_ix.wrapping_add(best_len) as isize)
                            as ::core::ffi::c_int)
                {
                    let len: size_t = FindMatchLengthWithLimit(
                        data.offset(prev_ix as isize) as *const uint8_t,
                        data.offset(cur_ix_masked as isize) as *const uint8_t,
                        max_length,
                    ) as size_t;
                    if len >= 3 as size_t || len == 2 as size_t && i < 2 as size_t {
                        let mut score: size_t = BackwardReferenceScoreUsingLastDistance(len);
                        if best_score < score {
                            if i != 0 as size_t {
                                score = (score as ::core::ffi::c_ulong)
                                    .wrapping_sub(BackwardReferencePenaltyUsingLastDistance(i)
                                        as ::core::ffi::c_ulong)
                                    as size_t as size_t;
                            }
                            if best_score < score {
                                best_score = score;
                                best_len = len;
                                (*out).len = best_len;
                                (*out).distance = backward;
                                (*out).score = best_score;
                            }
                        }
                    }
                }
            }
        }
        i = i.wrapping_add(1);
    }
    if best_len < 3 as size_t {
        best_len = 3 as size_t;
    }
    let tag: uint8_t = (hash & TAG_HASH_MASK as uint32_t) as uint8_t;
    let head: size_t = ((*num.offset(key as isize) as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
        as uint32_t
        & (*self_0).block_mask_) as size_t;
    let mut matches: uint64_t = GetMatchingTagMask(
        (*self_0).block_size_.wrapping_div(16 as size_t),
        tag,
        tag_bucket,
        head,
    );
    let mut n: uint16_t =
        (65535 as ::core::ffi::c_int - *num.offset(key as isize) as ::core::ffi::c_int) as uint16_t;
    let mut block_has_unused_slots: uint64_t =
        ((*self_0).block_size_ > n as size_t) as ::core::ffi::c_int as uint64_t;
    let mut mask: uint64_t = (block_has_unused_slots
        << (n as ::core::ffi::c_int & 64 as ::core::ffi::c_int - 1 as ::core::ffi::c_int))
        .wrapping_sub(1 as uint64_t);
    matches = (matches as ::core::ffi::c_ulong & mask as ::core::ffi::c_ulong) as uint64_t;
    while matches > 0 as uint64_t {
        let rb_index: size_t = head
            .wrapping_add((matches as ::core::ffi::c_ulonglong).trailing_zeros() as i32 as size_t)
            & (*self_0).block_mask_ as size_t;
        let mut prev_ix_0: size_t = *bucket.offset(rb_index as isize) as size_t;
        let backward_0: size_t = cur_ix.wrapping_sub(prev_ix_0);
        if (backward_0 > max_backward) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
            break;
        }
        prev_ix_0 = (prev_ix_0 as ::core::ffi::c_ulong & ring_buffer_mask as ::core::ffi::c_ulong)
            as size_t;
        if cur_ix_masked.wrapping_add(best_len) > ring_buffer_mask {
            break;
        }
        if !(prev_ix_0.wrapping_add(best_len) > ring_buffer_mask
            || BrotliUnalignedRead32(
                data.offset(
                    cur_ix_masked
                        .wrapping_add(best_len)
                        .wrapping_sub(3 as size_t) as isize,
                ) as *const uint8_t as *const ::core::ffi::c_void,
            ) != BrotliUnalignedRead32(
                data.offset(prev_ix_0.wrapping_add(best_len).wrapping_sub(3 as size_t) as isize)
                    as *const uint8_t as *const ::core::ffi::c_void,
            ))
        {
            let len_0: size_t = FindMatchLengthWithLimit(
                data.offset(prev_ix_0 as isize) as *const uint8_t,
                data.offset(cur_ix_masked as isize) as *const uint8_t,
                max_length,
            ) as size_t;
            if len_0 >= 4 as size_t {
                let mut score_0: size_t = BackwardReferenceScore(len_0, backward_0);
                if best_score < score_0 {
                    best_score = score_0;
                    best_len = len_0;
                    (*out).len = best_len;
                    (*out).distance = backward_0;
                    (*out).score = best_score;
                }
            }
        }
        matches = (matches as ::core::ffi::c_ulong
            & matches.wrapping_sub(1 as uint64_t) as ::core::ffi::c_ulong)
            as uint64_t;
    }
    *bucket.offset((*num.offset(key as isize) as uint32_t & (*self_0).block_mask_) as isize) =
        cur_ix as uint32_t;
    *tag_bucket.offset((*num.offset(key as isize) as uint32_t & (*self_0).block_mask_) as isize) =
        tag;
    let ref mut fresh5 = *num.offset(key as isize);
    *fresh5 = (*fresh5).wrapping_sub(1);
    if min_score == (*out).score {
        SearchInStaticDictionary(
            dictionary,
            (*self_0).common_,
            data.offset(cur_ix_masked as isize) as *const uint8_t,
            max_length,
            dictionary_distance,
            max_distance,
            out,
            BROTLI_FALSE,
        );
    }
}
pub const BANK_SIZE: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << BANK_BITS;
pub const BUCKET_SIZE_2: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << BUCKET_BITS_1;
pub const CAPPED_CHAINS: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[inline(always)]
unsafe extern "C" fn HashTypeLengthH40() -> size_t {
    return 4 as size_t;
}
#[inline(always)]
unsafe extern "C" fn StoreLookaheadH40() -> size_t {
    return 4 as size_t;
}
#[inline(always)]
unsafe extern "C" fn HashBytesH40(mut data: *const uint8_t) -> size_t {
    let h: uint32_t = (BrotliUnalignedRead32(data as *const ::core::ffi::c_void) as uint32_t)
        .wrapping_mul(kHashMul32);
    return (h >> 32 as ::core::ffi::c_int - BUCKET_BITS_1) as size_t;
}
unsafe extern "C" fn AddrH40(mut extra: *mut ::core::ffi::c_void) -> *mut uint32_t {
    return extra as *mut uint32_t;
}
unsafe extern "C" fn HeadH40(mut extra: *mut ::core::ffi::c_void) -> *mut uint16_t {
    return (AddrH40 as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut uint32_t)(extra)
        .offset(BUCKET_SIZE_2 as isize) as *mut uint32_t as *mut uint16_t;
}
unsafe extern "C" fn TinyHashH40(mut extra: *mut ::core::ffi::c_void) -> *mut uint8_t {
    return (HeadH40 as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut uint16_t)(extra)
        .offset(BUCKET_SIZE_2 as isize) as *mut uint16_t as *mut uint8_t;
}
unsafe extern "C" fn BanksH40(mut extra: *mut ::core::ffi::c_void) -> *mut BankH40 {
    return extra as *mut BankH40;
}
#[inline(always)]
unsafe extern "C" fn StoreH40(
    mut self_0: *mut H40,
    mut data: *const uint8_t,
    mask: size_t,
    ix: size_t,
) {
    let mut addr: *mut uint32_t =
        AddrH40((*self_0).extra[0 as ::core::ffi::c_int as usize]) as *mut uint32_t;
    let mut head: *mut uint16_t =
        HeadH40((*self_0).extra[0 as ::core::ffi::c_int as usize]) as *mut uint16_t;
    let mut tiny_hash: *mut uint8_t =
        TinyHashH40((*self_0).extra[0 as ::core::ffi::c_int as usize]) as *mut uint8_t;
    let mut banks: *mut BankH40 =
        BanksH40((*self_0).extra[1 as ::core::ffi::c_int as usize]) as *mut BankH40;
    let key: size_t = HashBytesH40(data.offset((ix & mask) as isize) as *const uint8_t) as size_t;
    let bank: size_t = key & (NUM_BANKS - 1 as ::core::ffi::c_int) as size_t;
    let fresh8 = (*self_0).free_slot_idx[bank as usize];
    (*self_0).free_slot_idx[bank as usize] = (*self_0).free_slot_idx[bank as usize].wrapping_add(1);
    let idx: size_t =
        (fresh8 as ::core::ffi::c_int & BANK_SIZE - 1 as ::core::ffi::c_int) as size_t;
    let mut delta: size_t = ix.wrapping_sub(*addr.offset(key as isize) as size_t);
    *tiny_hash.offset(ix as uint16_t as isize) = key as uint8_t;
    if delta > 0xffff as size_t {
        delta = (if CAPPED_CHAINS != 0 {
            0 as ::core::ffi::c_int
        } else {
            0xffff as ::core::ffi::c_int
        }) as size_t;
    }
    (*banks.offset(bank as isize)).slots[idx as usize].delta = delta as uint16_t;
    (*banks.offset(bank as isize)).slots[idx as usize].next = *head.offset(key as isize);
    *addr.offset(key as isize) = ix as uint32_t;
    *head.offset(key as isize) = idx as uint16_t;
}
#[inline(always)]
unsafe extern "C" fn StoreRangeH40(
    mut self_0: *mut H40,
    mut data: *const uint8_t,
    mask: size_t,
    ix_start: size_t,
    ix_end: size_t,
) {
    let mut i: size_t = 0;
    i = ix_start;
    while i < ix_end {
        StoreH40(self_0, data, mask, i);
        i = i.wrapping_add(1);
    }
}
#[inline(always)]
unsafe extern "C" fn PrepareDistanceCacheH40(
    mut self_0: *mut H40,
    mut distance_cache: *mut ::core::ffi::c_int,
) {
    PrepareDistanceCache(distance_cache, NUM_LAST_DISTANCES_TO_CHECK);
}
#[inline(always)]
unsafe extern "C" fn FindLongestMatchH40(
    mut self_0: *mut H40,
    mut dictionary: *const BrotliEncoderDictionary,
    mut data: *const uint8_t,
    ring_buffer_mask: size_t,
    mut distance_cache: *const ::core::ffi::c_int,
    cur_ix: size_t,
    max_length: size_t,
    max_backward: size_t,
    dictionary_distance: size_t,
    max_distance: size_t,
    mut out: *mut HasherSearchResult,
) {
    let mut addr: *mut uint32_t =
        AddrH40((*self_0).extra[0 as ::core::ffi::c_int as usize]) as *mut uint32_t;
    let mut head: *mut uint16_t =
        HeadH40((*self_0).extra[0 as ::core::ffi::c_int as usize]) as *mut uint16_t;
    let mut tiny_hashes: *mut uint8_t =
        TinyHashH40((*self_0).extra[0 as ::core::ffi::c_int as usize]) as *mut uint8_t;
    let mut banks: *mut BankH40 =
        BanksH40((*self_0).extra[1 as ::core::ffi::c_int as usize]) as *mut BankH40;
    let cur_ix_masked: size_t = cur_ix & ring_buffer_mask;
    let mut min_score: size_t = (*out).score;
    let mut best_score: size_t = (*out).score;
    let mut best_len: size_t = (*out).len;
    let mut i: size_t = 0;
    let key: size_t = HashBytesH40(data.offset(cur_ix_masked as isize) as *const uint8_t) as size_t;
    let tiny_hash: uint8_t = key as uint8_t;
    (*out).len = 0 as size_t;
    (*out).len_code_delta = 0 as ::core::ffi::c_int;
    i = 0 as size_t;
    while i < NUM_LAST_DISTANCES_TO_CHECK as size_t {
        let backward: size_t = *distance_cache.offset(i as isize) as size_t;
        let mut prev_ix: size_t = cur_ix.wrapping_sub(backward);
        if !(i > 0 as size_t
            && *tiny_hashes.offset(prev_ix as uint16_t as isize) as ::core::ffi::c_int
                != tiny_hash as ::core::ffi::c_int)
        {
            if !(prev_ix >= cur_ix || backward > max_backward) {
                prev_ix = (prev_ix as ::core::ffi::c_ulong
                    & ring_buffer_mask as ::core::ffi::c_ulong) as size_t;
                let len: size_t = FindMatchLengthWithLimit(
                    data.offset(prev_ix as isize) as *const uint8_t,
                    data.offset(cur_ix_masked as isize) as *const uint8_t,
                    max_length,
                ) as size_t;
                if len >= 2 as size_t {
                    let mut score: size_t = BackwardReferenceScoreUsingLastDistance(len);
                    if best_score < score {
                        if i != 0 as size_t {
                            score = (score as ::core::ffi::c_ulong)
                                .wrapping_sub(BackwardReferencePenaltyUsingLastDistance(i)
                                    as ::core::ffi::c_ulong)
                                as size_t as size_t;
                        }
                        if best_score < score {
                            best_score = score;
                            best_len = len;
                            (*out).len = best_len;
                            (*out).distance = backward;
                            (*out).score = best_score;
                        }
                    }
                }
            }
        }
        i = i.wrapping_add(1);
    }
    if best_len < 3 as size_t {
        best_len = 3 as size_t;
    }
    let bank: size_t = key & (NUM_BANKS - 1 as ::core::ffi::c_int) as size_t;
    let mut backward_0: size_t = 0 as size_t;
    let mut hops: size_t = (*self_0).max_hops;
    let mut delta: size_t = cur_ix.wrapping_sub(*addr.offset(key as isize) as size_t);
    let mut slot: size_t = *head.offset(key as isize) as size_t;
    loop {
        let fresh9 = hops;
        hops = hops.wrapping_sub(1);
        if !(fresh9 != 0) {
            break;
        }
        let mut prev_ix_0: size_t = 0;
        let mut last: size_t = slot;
        backward_0 = (backward_0 as ::core::ffi::c_ulong)
            .wrapping_add(delta as ::core::ffi::c_ulong) as size_t as size_t;
        if backward_0 > max_backward || CAPPED_CHAINS != 0 && delta == 0 {
            break;
        }
        prev_ix_0 = cur_ix.wrapping_sub(backward_0) & ring_buffer_mask;
        slot = (*banks.offset(bank as isize)).slots[last as usize].next as size_t;
        delta = (*banks.offset(bank as isize)).slots[last as usize].delta as size_t;
        if cur_ix_masked.wrapping_add(best_len) > ring_buffer_mask
            || prev_ix_0.wrapping_add(best_len) > ring_buffer_mask
            || BrotliUnalignedRead32(
                data.offset(
                    cur_ix_masked
                        .wrapping_add(best_len)
                        .wrapping_sub(3 as size_t) as isize,
                ) as *const uint8_t as *const ::core::ffi::c_void,
            ) != BrotliUnalignedRead32(
                data.offset(prev_ix_0.wrapping_add(best_len).wrapping_sub(3 as size_t) as isize)
                    as *const uint8_t as *const ::core::ffi::c_void,
            )
        {
            continue;
        }
        let len_0: size_t = FindMatchLengthWithLimit(
            data.offset(prev_ix_0 as isize) as *const uint8_t,
            data.offset(cur_ix_masked as isize) as *const uint8_t,
            max_length,
        ) as size_t;
        if len_0 >= 4 as size_t {
            let mut score_0: size_t = BackwardReferenceScore(len_0, backward_0);
            if best_score < score_0 {
                best_score = score_0;
                best_len = len_0;
                (*out).len = best_len;
                (*out).distance = backward_0;
                (*out).score = best_score;
            }
        }
    }
    StoreH40(self_0, data, ring_buffer_mask, cur_ix);
    if (*out).score == min_score {
        SearchInStaticDictionary(
            dictionary,
            (*self_0).common,
            data.offset(cur_ix_masked as isize) as *const uint8_t,
            max_length,
            dictionary_distance,
            max_distance,
            out,
            BROTLI_FALSE,
        );
    }
}
pub const BANK_SIZE_0: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << BANK_BITS;
pub const BUCKET_SIZE_3: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << BUCKET_BITS_1;
pub const CAPPED_CHAINS_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[inline(always)]
unsafe extern "C" fn HashTypeLengthH41() -> size_t {
    return 4 as size_t;
}
#[inline(always)]
unsafe extern "C" fn StoreLookaheadH41() -> size_t {
    return 4 as size_t;
}
#[inline(always)]
unsafe extern "C" fn HashBytesH41(mut data: *const uint8_t) -> size_t {
    let h: uint32_t = (BrotliUnalignedRead32(data as *const ::core::ffi::c_void) as uint32_t)
        .wrapping_mul(kHashMul32);
    return (h >> 32 as ::core::ffi::c_int - BUCKET_BITS_1) as size_t;
}
unsafe extern "C" fn AddrH41(mut extra: *mut ::core::ffi::c_void) -> *mut uint32_t {
    return extra as *mut uint32_t;
}
unsafe extern "C" fn HeadH41(mut extra: *mut ::core::ffi::c_void) -> *mut uint16_t {
    return (AddrH41 as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut uint32_t)(extra)
        .offset(BUCKET_SIZE_3 as isize) as *mut uint32_t as *mut uint16_t;
}
unsafe extern "C" fn TinyHashH41(mut extra: *mut ::core::ffi::c_void) -> *mut uint8_t {
    return (HeadH41 as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut uint16_t)(extra)
        .offset(BUCKET_SIZE_3 as isize) as *mut uint16_t as *mut uint8_t;
}
unsafe extern "C" fn BanksH41(mut extra: *mut ::core::ffi::c_void) -> *mut BankH41 {
    return extra as *mut BankH41;
}
#[inline(always)]
unsafe extern "C" fn StoreH41(
    mut self_0: *mut H41,
    mut data: *const uint8_t,
    mask: size_t,
    ix: size_t,
) {
    let mut addr: *mut uint32_t =
        AddrH41((*self_0).extra[0 as ::core::ffi::c_int as usize]) as *mut uint32_t;
    let mut head: *mut uint16_t =
        HeadH41((*self_0).extra[0 as ::core::ffi::c_int as usize]) as *mut uint16_t;
    let mut tiny_hash: *mut uint8_t =
        TinyHashH41((*self_0).extra[0 as ::core::ffi::c_int as usize]) as *mut uint8_t;
    let mut banks: *mut BankH41 =
        BanksH41((*self_0).extra[1 as ::core::ffi::c_int as usize]) as *mut BankH41;
    let key: size_t = HashBytesH41(data.offset((ix & mask) as isize) as *const uint8_t) as size_t;
    let bank: size_t = key & (NUM_BANKS - 1 as ::core::ffi::c_int) as size_t;
    let fresh10 = (*self_0).free_slot_idx[bank as usize];
    (*self_0).free_slot_idx[bank as usize] = (*self_0).free_slot_idx[bank as usize].wrapping_add(1);
    let idx: size_t =
        (fresh10 as ::core::ffi::c_int & BANK_SIZE_0 - 1 as ::core::ffi::c_int) as size_t;
    let mut delta: size_t = ix.wrapping_sub(*addr.offset(key as isize) as size_t);
    *tiny_hash.offset(ix as uint16_t as isize) = key as uint8_t;
    if delta > 0xffff as size_t {
        delta = (if CAPPED_CHAINS_0 != 0 {
            0 as ::core::ffi::c_int
        } else {
            0xffff as ::core::ffi::c_int
        }) as size_t;
    }
    (*banks.offset(bank as isize)).slots[idx as usize].delta = delta as uint16_t;
    (*banks.offset(bank as isize)).slots[idx as usize].next = *head.offset(key as isize);
    *addr.offset(key as isize) = ix as uint32_t;
    *head.offset(key as isize) = idx as uint16_t;
}
#[inline(always)]
unsafe extern "C" fn StoreRangeH41(
    mut self_0: *mut H41,
    mut data: *const uint8_t,
    mask: size_t,
    ix_start: size_t,
    ix_end: size_t,
) {
    let mut i: size_t = 0;
    i = ix_start;
    while i < ix_end {
        StoreH41(self_0, data, mask, i);
        i = i.wrapping_add(1);
    }
}
#[inline(always)]
unsafe extern "C" fn PrepareDistanceCacheH41(
    mut self_0: *mut H41,
    mut distance_cache: *mut ::core::ffi::c_int,
) {
    PrepareDistanceCache(distance_cache, NUM_LAST_DISTANCES_TO_CHECK_0);
}
#[inline(always)]
unsafe extern "C" fn FindLongestMatchH41(
    mut self_0: *mut H41,
    mut dictionary: *const BrotliEncoderDictionary,
    mut data: *const uint8_t,
    ring_buffer_mask: size_t,
    mut distance_cache: *const ::core::ffi::c_int,
    cur_ix: size_t,
    max_length: size_t,
    max_backward: size_t,
    dictionary_distance: size_t,
    max_distance: size_t,
    mut out: *mut HasherSearchResult,
) {
    let mut addr: *mut uint32_t =
        AddrH41((*self_0).extra[0 as ::core::ffi::c_int as usize]) as *mut uint32_t;
    let mut head: *mut uint16_t =
        HeadH41((*self_0).extra[0 as ::core::ffi::c_int as usize]) as *mut uint16_t;
    let mut tiny_hashes: *mut uint8_t =
        TinyHashH41((*self_0).extra[0 as ::core::ffi::c_int as usize]) as *mut uint8_t;
    let mut banks: *mut BankH41 =
        BanksH41((*self_0).extra[1 as ::core::ffi::c_int as usize]) as *mut BankH41;
    let cur_ix_masked: size_t = cur_ix & ring_buffer_mask;
    let mut min_score: size_t = (*out).score;
    let mut best_score: size_t = (*out).score;
    let mut best_len: size_t = (*out).len;
    let mut i: size_t = 0;
    let key: size_t = HashBytesH41(data.offset(cur_ix_masked as isize) as *const uint8_t) as size_t;
    let tiny_hash: uint8_t = key as uint8_t;
    (*out).len = 0 as size_t;
    (*out).len_code_delta = 0 as ::core::ffi::c_int;
    i = 0 as size_t;
    while i < NUM_LAST_DISTANCES_TO_CHECK_0 as size_t {
        let backward: size_t = *distance_cache.offset(i as isize) as size_t;
        let mut prev_ix: size_t = cur_ix.wrapping_sub(backward);
        if !(i > 0 as size_t
            && *tiny_hashes.offset(prev_ix as uint16_t as isize) as ::core::ffi::c_int
                != tiny_hash as ::core::ffi::c_int)
        {
            if !(prev_ix >= cur_ix || backward > max_backward) {
                prev_ix = (prev_ix as ::core::ffi::c_ulong
                    & ring_buffer_mask as ::core::ffi::c_ulong) as size_t;
                let len: size_t = FindMatchLengthWithLimit(
                    data.offset(prev_ix as isize) as *const uint8_t,
                    data.offset(cur_ix_masked as isize) as *const uint8_t,
                    max_length,
                ) as size_t;
                if len >= 2 as size_t {
                    let mut score: size_t = BackwardReferenceScoreUsingLastDistance(len);
                    if best_score < score {
                        if i != 0 as size_t {
                            score = (score as ::core::ffi::c_ulong)
                                .wrapping_sub(BackwardReferencePenaltyUsingLastDistance(i)
                                    as ::core::ffi::c_ulong)
                                as size_t as size_t;
                        }
                        if best_score < score {
                            best_score = score;
                            best_len = len;
                            (*out).len = best_len;
                            (*out).distance = backward;
                            (*out).score = best_score;
                        }
                    }
                }
            }
        }
        i = i.wrapping_add(1);
    }
    if best_len < 3 as size_t {
        best_len = 3 as size_t;
    }
    let bank: size_t = key & (NUM_BANKS - 1 as ::core::ffi::c_int) as size_t;
    let mut backward_0: size_t = 0 as size_t;
    let mut hops: size_t = (*self_0).max_hops;
    let mut delta: size_t = cur_ix.wrapping_sub(*addr.offset(key as isize) as size_t);
    let mut slot: size_t = *head.offset(key as isize) as size_t;
    loop {
        let fresh11 = hops;
        hops = hops.wrapping_sub(1);
        if !(fresh11 != 0) {
            break;
        }
        let mut prev_ix_0: size_t = 0;
        let mut last: size_t = slot;
        backward_0 = (backward_0 as ::core::ffi::c_ulong)
            .wrapping_add(delta as ::core::ffi::c_ulong) as size_t as size_t;
        if backward_0 > max_backward || CAPPED_CHAINS_0 != 0 && delta == 0 {
            break;
        }
        prev_ix_0 = cur_ix.wrapping_sub(backward_0) & ring_buffer_mask;
        slot = (*banks.offset(bank as isize)).slots[last as usize].next as size_t;
        delta = (*banks.offset(bank as isize)).slots[last as usize].delta as size_t;
        if cur_ix_masked.wrapping_add(best_len) > ring_buffer_mask
            || prev_ix_0.wrapping_add(best_len) > ring_buffer_mask
            || BrotliUnalignedRead32(
                data.offset(
                    cur_ix_masked
                        .wrapping_add(best_len)
                        .wrapping_sub(3 as size_t) as isize,
                ) as *const uint8_t as *const ::core::ffi::c_void,
            ) != BrotliUnalignedRead32(
                data.offset(prev_ix_0.wrapping_add(best_len).wrapping_sub(3 as size_t) as isize)
                    as *const uint8_t as *const ::core::ffi::c_void,
            )
        {
            continue;
        }
        let len_0: size_t = FindMatchLengthWithLimit(
            data.offset(prev_ix_0 as isize) as *const uint8_t,
            data.offset(cur_ix_masked as isize) as *const uint8_t,
            max_length,
        ) as size_t;
        if len_0 >= 4 as size_t {
            let mut score_0: size_t = BackwardReferenceScore(len_0, backward_0);
            if best_score < score_0 {
                best_score = score_0;
                best_len = len_0;
                (*out).len = best_len;
                (*out).distance = backward_0;
                (*out).score = best_score;
            }
        }
    }
    StoreH41(self_0, data, ring_buffer_mask, cur_ix);
    if (*out).score == min_score {
        SearchInStaticDictionary(
            dictionary,
            (*self_0).common,
            data.offset(cur_ix_masked as isize) as *const uint8_t,
            max_length,
            dictionary_distance,
            max_distance,
            out,
            BROTLI_FALSE,
        );
    }
}
pub const BANK_SIZE_1: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << BANK_BITS_0;
pub const BUCKET_SIZE_4: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << BUCKET_BITS_1;
pub const CAPPED_CHAINS_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[inline(always)]
unsafe extern "C" fn HashTypeLengthH42() -> size_t {
    return 4 as size_t;
}
#[inline(always)]
unsafe extern "C" fn StoreLookaheadH42() -> size_t {
    return 4 as size_t;
}
#[inline(always)]
unsafe extern "C" fn HashBytesH42(mut data: *const uint8_t) -> size_t {
    let h: uint32_t = (BrotliUnalignedRead32(data as *const ::core::ffi::c_void) as uint32_t)
        .wrapping_mul(kHashMul32);
    return (h >> 32 as ::core::ffi::c_int - BUCKET_BITS_1) as size_t;
}
unsafe extern "C" fn AddrH42(mut extra: *mut ::core::ffi::c_void) -> *mut uint32_t {
    return extra as *mut uint32_t;
}
unsafe extern "C" fn HeadH42(mut extra: *mut ::core::ffi::c_void) -> *mut uint16_t {
    return (AddrH42 as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut uint32_t)(extra)
        .offset(BUCKET_SIZE_4 as isize) as *mut uint32_t as *mut uint16_t;
}
unsafe extern "C" fn TinyHashH42(mut extra: *mut ::core::ffi::c_void) -> *mut uint8_t {
    return (HeadH42 as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut uint16_t)(extra)
        .offset(BUCKET_SIZE_4 as isize) as *mut uint16_t as *mut uint8_t;
}
unsafe extern "C" fn BanksH42(mut extra: *mut ::core::ffi::c_void) -> *mut BankH42 {
    return extra as *mut BankH42;
}
#[inline(always)]
unsafe extern "C" fn StoreH42(
    mut self_0: *mut H42,
    mut data: *const uint8_t,
    mask: size_t,
    ix: size_t,
) {
    let mut addr: *mut uint32_t =
        AddrH42((*self_0).extra[0 as ::core::ffi::c_int as usize]) as *mut uint32_t;
    let mut head: *mut uint16_t =
        HeadH42((*self_0).extra[0 as ::core::ffi::c_int as usize]) as *mut uint16_t;
    let mut tiny_hash: *mut uint8_t =
        TinyHashH42((*self_0).extra[0 as ::core::ffi::c_int as usize]) as *mut uint8_t;
    let mut banks: *mut BankH42 =
        BanksH42((*self_0).extra[1 as ::core::ffi::c_int as usize]) as *mut BankH42;
    let key: size_t = HashBytesH42(data.offset((ix & mask) as isize) as *const uint8_t) as size_t;
    let bank: size_t = key & (NUM_BANKS_0 - 1 as ::core::ffi::c_int) as size_t;
    let fresh12 = (*self_0).free_slot_idx[bank as usize];
    (*self_0).free_slot_idx[bank as usize] = (*self_0).free_slot_idx[bank as usize].wrapping_add(1);
    let idx: size_t =
        (fresh12 as ::core::ffi::c_int & BANK_SIZE_1 - 1 as ::core::ffi::c_int) as size_t;
    let mut delta: size_t = ix.wrapping_sub(*addr.offset(key as isize) as size_t);
    *tiny_hash.offset(ix as uint16_t as isize) = key as uint8_t;
    if delta > 0xffff as size_t {
        delta = (if CAPPED_CHAINS_1 != 0 {
            0 as ::core::ffi::c_int
        } else {
            0xffff as ::core::ffi::c_int
        }) as size_t;
    }
    (*banks.offset(bank as isize)).slots[idx as usize].delta = delta as uint16_t;
    (*banks.offset(bank as isize)).slots[idx as usize].next = *head.offset(key as isize);
    *addr.offset(key as isize) = ix as uint32_t;
    *head.offset(key as isize) = idx as uint16_t;
}
#[inline(always)]
unsafe extern "C" fn StoreRangeH42(
    mut self_0: *mut H42,
    mut data: *const uint8_t,
    mask: size_t,
    ix_start: size_t,
    ix_end: size_t,
) {
    let mut i: size_t = 0;
    i = ix_start;
    while i < ix_end {
        StoreH42(self_0, data, mask, i);
        i = i.wrapping_add(1);
    }
}
#[inline(always)]
unsafe extern "C" fn PrepareDistanceCacheH42(
    mut self_0: *mut H42,
    mut distance_cache: *mut ::core::ffi::c_int,
) {
    PrepareDistanceCache(distance_cache, NUM_LAST_DISTANCES_TO_CHECK_1);
}
#[inline(always)]
unsafe extern "C" fn FindLongestMatchH42(
    mut self_0: *mut H42,
    mut dictionary: *const BrotliEncoderDictionary,
    mut data: *const uint8_t,
    ring_buffer_mask: size_t,
    mut distance_cache: *const ::core::ffi::c_int,
    cur_ix: size_t,
    max_length: size_t,
    max_backward: size_t,
    dictionary_distance: size_t,
    max_distance: size_t,
    mut out: *mut HasherSearchResult,
) {
    let mut addr: *mut uint32_t =
        AddrH42((*self_0).extra[0 as ::core::ffi::c_int as usize]) as *mut uint32_t;
    let mut head: *mut uint16_t =
        HeadH42((*self_0).extra[0 as ::core::ffi::c_int as usize]) as *mut uint16_t;
    let mut tiny_hashes: *mut uint8_t =
        TinyHashH42((*self_0).extra[0 as ::core::ffi::c_int as usize]) as *mut uint8_t;
    let mut banks: *mut BankH42 =
        BanksH42((*self_0).extra[1 as ::core::ffi::c_int as usize]) as *mut BankH42;
    let cur_ix_masked: size_t = cur_ix & ring_buffer_mask;
    let mut min_score: size_t = (*out).score;
    let mut best_score: size_t = (*out).score;
    let mut best_len: size_t = (*out).len;
    let mut i: size_t = 0;
    let key: size_t = HashBytesH42(data.offset(cur_ix_masked as isize) as *const uint8_t) as size_t;
    let tiny_hash: uint8_t = key as uint8_t;
    (*out).len = 0 as size_t;
    (*out).len_code_delta = 0 as ::core::ffi::c_int;
    i = 0 as size_t;
    while i < NUM_LAST_DISTANCES_TO_CHECK_1 as size_t {
        let backward: size_t = *distance_cache.offset(i as isize) as size_t;
        let mut prev_ix: size_t = cur_ix.wrapping_sub(backward);
        if !(i > 0 as size_t
            && *tiny_hashes.offset(prev_ix as uint16_t as isize) as ::core::ffi::c_int
                != tiny_hash as ::core::ffi::c_int)
        {
            if !(prev_ix >= cur_ix || backward > max_backward) {
                prev_ix = (prev_ix as ::core::ffi::c_ulong
                    & ring_buffer_mask as ::core::ffi::c_ulong) as size_t;
                let len: size_t = FindMatchLengthWithLimit(
                    data.offset(prev_ix as isize) as *const uint8_t,
                    data.offset(cur_ix_masked as isize) as *const uint8_t,
                    max_length,
                ) as size_t;
                if len >= 2 as size_t {
                    let mut score: size_t = BackwardReferenceScoreUsingLastDistance(len);
                    if best_score < score {
                        if i != 0 as size_t {
                            score = (score as ::core::ffi::c_ulong)
                                .wrapping_sub(BackwardReferencePenaltyUsingLastDistance(i)
                                    as ::core::ffi::c_ulong)
                                as size_t as size_t;
                        }
                        if best_score < score {
                            best_score = score;
                            best_len = len;
                            (*out).len = best_len;
                            (*out).distance = backward;
                            (*out).score = best_score;
                        }
                    }
                }
            }
        }
        i = i.wrapping_add(1);
    }
    if best_len < 3 as size_t {
        best_len = 3 as size_t;
    }
    let bank: size_t = key & (NUM_BANKS_0 - 1 as ::core::ffi::c_int) as size_t;
    let mut backward_0: size_t = 0 as size_t;
    let mut hops: size_t = (*self_0).max_hops;
    let mut delta: size_t = cur_ix.wrapping_sub(*addr.offset(key as isize) as size_t);
    let mut slot: size_t = *head.offset(key as isize) as size_t;
    loop {
        let fresh13 = hops;
        hops = hops.wrapping_sub(1);
        if !(fresh13 != 0) {
            break;
        }
        let mut prev_ix_0: size_t = 0;
        let mut last: size_t = slot;
        backward_0 = (backward_0 as ::core::ffi::c_ulong)
            .wrapping_add(delta as ::core::ffi::c_ulong) as size_t as size_t;
        if backward_0 > max_backward || CAPPED_CHAINS_1 != 0 && delta == 0 {
            break;
        }
        prev_ix_0 = cur_ix.wrapping_sub(backward_0) & ring_buffer_mask;
        slot = (*banks.offset(bank as isize)).slots[last as usize].next as size_t;
        delta = (*banks.offset(bank as isize)).slots[last as usize].delta as size_t;
        if cur_ix_masked.wrapping_add(best_len) > ring_buffer_mask
            || prev_ix_0.wrapping_add(best_len) > ring_buffer_mask
            || BrotliUnalignedRead32(
                data.offset(
                    cur_ix_masked
                        .wrapping_add(best_len)
                        .wrapping_sub(3 as size_t) as isize,
                ) as *const uint8_t as *const ::core::ffi::c_void,
            ) != BrotliUnalignedRead32(
                data.offset(prev_ix_0.wrapping_add(best_len).wrapping_sub(3 as size_t) as isize)
                    as *const uint8_t as *const ::core::ffi::c_void,
            )
        {
            continue;
        }
        let len_0: size_t = FindMatchLengthWithLimit(
            data.offset(prev_ix_0 as isize) as *const uint8_t,
            data.offset(cur_ix_masked as isize) as *const uint8_t,
            max_length,
        ) as size_t;
        if len_0 >= 4 as size_t {
            let mut score_0: size_t = BackwardReferenceScore(len_0, backward_0);
            if best_score < score_0 {
                best_score = score_0;
                best_len = len_0;
                (*out).len = best_len;
                (*out).distance = backward_0;
                (*out).score = best_score;
            }
        }
    }
    StoreH42(self_0, data, ring_buffer_mask, cur_ix);
    if (*out).score == min_score {
        SearchInStaticDictionary(
            dictionary,
            (*self_0).common,
            data.offset(cur_ix_masked as isize) as *const uint8_t,
            max_length,
            dictionary_distance,
            max_distance,
            out,
            BROTLI_FALSE,
        );
    }
}
static mut kInvalidPosHROLLING_FAST: uint32_t = 0xffffffff as uint32_t;
#[inline(always)]
unsafe extern "C" fn HashTypeLengthHROLLING_FAST() -> size_t {
    return 4 as size_t;
}
#[inline(always)]
unsafe extern "C" fn StoreLookaheadHROLLING_FAST() -> size_t {
    return 4 as size_t;
}
unsafe extern "C" fn HashByteHROLLING_FAST(mut byte: uint8_t) -> uint32_t {
    return (byte as uint32_t).wrapping_add(1 as uint32_t);
}
unsafe extern "C" fn HashRollingFunctionHROLLING_FAST(
    mut state: uint32_t,
    mut add: uint8_t,
    mut rem: uint8_t,
    mut factor: uint32_t,
    mut factor_remove: uint32_t,
) -> uint32_t {
    return factor
        .wrapping_mul(state)
        .wrapping_add(HashByteHROLLING_FAST(add))
        .wrapping_sub(factor_remove.wrapping_mul(HashByteHROLLING_FAST(rem)));
}
#[inline(always)]
unsafe extern "C" fn StoreHROLLING_FAST(
    mut self_0: *mut HROLLING_FAST,
    mut data: *const uint8_t,
    mask: size_t,
    ix: size_t,
) {
}
#[inline(always)]
unsafe extern "C" fn StoreRangeHROLLING_FAST(
    mut self_0: *mut HROLLING_FAST,
    mut data: *const uint8_t,
    mask: size_t,
    ix_start: size_t,
    ix_end: size_t,
) {
}
#[inline(always)]
unsafe extern "C" fn PrepareDistanceCacheHROLLING_FAST(
    mut self_0: *mut HROLLING_FAST,
    mut distance_cache: *mut ::core::ffi::c_int,
) {
}
#[inline(always)]
unsafe extern "C" fn FindLongestMatchHROLLING_FAST(
    mut self_0: *mut HROLLING_FAST,
    mut dictionary: *const BrotliEncoderDictionary,
    mut data: *const uint8_t,
    ring_buffer_mask: size_t,
    mut distance_cache: *const ::core::ffi::c_int,
    cur_ix: size_t,
    max_length: size_t,
    max_backward: size_t,
    dictionary_distance: size_t,
    max_distance: size_t,
    mut out: *mut HasherSearchResult,
) {
    let cur_ix_masked: size_t = cur_ix & ring_buffer_mask;
    let mut pos: size_t = 0;
    if cur_ix & (JUMP - 1 as ::core::ffi::c_int) as size_t != 0 as size_t {
        return;
    }
    if max_length < CHUNKLEN as size_t {
        return;
    }
    pos = (*self_0).next_ix;
    while pos <= cur_ix {
        let mut code: uint32_t = (*self_0).state & MASK as uint32_t;
        let mut rem: uint8_t = *data.offset((pos & ring_buffer_mask) as isize);
        let mut add: uint8_t =
            *data.offset((pos.wrapping_add(CHUNKLEN as size_t) & ring_buffer_mask) as isize);
        let mut found_ix: size_t = kInvalidPosHROLLING_FAST as size_t;
        (*self_0).state = HashRollingFunctionHROLLING_FAST(
            (*self_0).state,
            add,
            rem,
            (*self_0).factor,
            (*self_0).factor_remove,
        );
        if code < NUMBUCKETS as uint32_t {
            found_ix = *(*self_0).table.offset(code as isize) as size_t;
            *(*self_0).table.offset(code as isize) = pos as uint32_t;
            if pos == cur_ix && found_ix != kInvalidPosHROLLING_FAST as size_t {
                let mut backward: size_t = cur_ix.wrapping_sub(found_ix) as uint32_t as size_t;
                if backward <= max_backward {
                    let found_ix_masked: size_t = found_ix & ring_buffer_mask;
                    let len: size_t = FindMatchLengthWithLimit(
                        data.offset(found_ix_masked as isize) as *const uint8_t,
                        data.offset(cur_ix_masked as isize) as *const uint8_t,
                        max_length,
                    ) as size_t;
                    if len >= 4 as size_t && len > (*out).len {
                        let mut score: size_t = BackwardReferenceScore(len, backward);
                        if score > (*out).score {
                            (*out).len = len;
                            (*out).distance = backward;
                            (*out).score = score;
                            (*out).len_code_delta = 0 as ::core::ffi::c_int;
                        }
                    }
                }
            }
        }
        pos = (pos as ::core::ffi::c_ulong).wrapping_add(JUMP as ::core::ffi::c_ulong) as size_t
            as size_t;
    }
    (*self_0).next_ix = cur_ix.wrapping_add(JUMP as size_t);
}
static mut kInvalidPosHROLLING: uint32_t = 0xffffffff as uint32_t;
#[inline(always)]
unsafe extern "C" fn HashTypeLengthHROLLING() -> size_t {
    return 4 as size_t;
}
#[inline(always)]
unsafe extern "C" fn StoreLookaheadHROLLING() -> size_t {
    return 4 as size_t;
}
unsafe extern "C" fn HashByteHROLLING(mut byte: uint8_t) -> uint32_t {
    return (byte as uint32_t).wrapping_add(1 as uint32_t);
}
unsafe extern "C" fn HashRollingFunctionHROLLING(
    mut state: uint32_t,
    mut add: uint8_t,
    mut rem: uint8_t,
    mut factor: uint32_t,
    mut factor_remove: uint32_t,
) -> uint32_t {
    return factor
        .wrapping_mul(state)
        .wrapping_add(HashByteHROLLING(add))
        .wrapping_sub(factor_remove.wrapping_mul(HashByteHROLLING(rem)));
}
#[inline(always)]
unsafe extern "C" fn StoreHROLLING(
    mut self_0: *mut HROLLING,
    mut data: *const uint8_t,
    mask: size_t,
    ix: size_t,
) {
}
#[inline(always)]
unsafe extern "C" fn StoreRangeHROLLING(
    mut self_0: *mut HROLLING,
    mut data: *const uint8_t,
    mask: size_t,
    ix_start: size_t,
    ix_end: size_t,
) {
}
#[inline(always)]
unsafe extern "C" fn PrepareDistanceCacheHROLLING(
    mut self_0: *mut HROLLING,
    mut distance_cache: *mut ::core::ffi::c_int,
) {
}
#[inline(always)]
unsafe extern "C" fn FindLongestMatchHROLLING(
    mut self_0: *mut HROLLING,
    mut dictionary: *const BrotliEncoderDictionary,
    mut data: *const uint8_t,
    ring_buffer_mask: size_t,
    mut distance_cache: *const ::core::ffi::c_int,
    cur_ix: size_t,
    max_length: size_t,
    max_backward: size_t,
    dictionary_distance: size_t,
    max_distance: size_t,
    mut out: *mut HasherSearchResult,
) {
    let cur_ix_masked: size_t = cur_ix & ring_buffer_mask;
    let mut pos: size_t = 0;
    if cur_ix & (JUMP_0 - 1 as ::core::ffi::c_int) as size_t != 0 as size_t {
        return;
    }
    if max_length < CHUNKLEN as size_t {
        return;
    }
    pos = (*self_0).next_ix;
    while pos <= cur_ix {
        let mut code: uint32_t = (*self_0).state & MASK as uint32_t;
        let mut rem: uint8_t = *data.offset((pos & ring_buffer_mask) as isize);
        let mut add: uint8_t =
            *data.offset((pos.wrapping_add(CHUNKLEN as size_t) & ring_buffer_mask) as isize);
        let mut found_ix: size_t = kInvalidPosHROLLING as size_t;
        (*self_0).state = HashRollingFunctionHROLLING(
            (*self_0).state,
            add,
            rem,
            (*self_0).factor,
            (*self_0).factor_remove,
        );
        if code < NUMBUCKETS as uint32_t {
            found_ix = *(*self_0).table.offset(code as isize) as size_t;
            *(*self_0).table.offset(code as isize) = pos as uint32_t;
            if pos == cur_ix && found_ix != kInvalidPosHROLLING as size_t {
                let mut backward: size_t = cur_ix.wrapping_sub(found_ix) as uint32_t as size_t;
                if backward <= max_backward {
                    let found_ix_masked: size_t = found_ix & ring_buffer_mask;
                    let len: size_t = FindMatchLengthWithLimit(
                        data.offset(found_ix_masked as isize) as *const uint8_t,
                        data.offset(cur_ix_masked as isize) as *const uint8_t,
                        max_length,
                    ) as size_t;
                    if len >= 4 as size_t && len > (*out).len {
                        let mut score: size_t = BackwardReferenceScore(len, backward);
                        if score > (*out).score {
                            (*out).len = len;
                            (*out).distance = backward;
                            (*out).score = score;
                            (*out).len_code_delta = 0 as ::core::ffi::c_int;
                        }
                    }
                }
            }
        }
        pos = (pos as ::core::ffi::c_ulong).wrapping_add(JUMP_0 as ::core::ffi::c_ulong) as size_t
            as size_t;
    }
    (*self_0).next_ix = cur_ix.wrapping_add(JUMP_0 as size_t);
}
#[inline(always)]
unsafe extern "C" fn HashTypeLengthH35() -> size_t {
    let mut a: size_t = HashTypeLengthH3();
    let mut b: size_t = HashTypeLengthHROLLING_FAST();
    return if a > b { a } else { b };
}
#[inline(always)]
unsafe extern "C" fn StoreLookaheadH35() -> size_t {
    let mut a: size_t = StoreLookaheadH3();
    let mut b: size_t = StoreLookaheadHROLLING_FAST();
    return if a > b { a } else { b };
}
#[inline(always)]
unsafe extern "C" fn StoreH35(
    mut self_0: *mut H35,
    mut data: *const uint8_t,
    mask: size_t,
    ix: size_t,
) {
    StoreH3(&raw mut (*self_0).ha, data, mask, ix);
    StoreHROLLING_FAST(&raw mut (*self_0).hb, data, mask, ix);
}
#[inline(always)]
unsafe extern "C" fn StoreRangeH35(
    mut self_0: *mut H35,
    mut data: *const uint8_t,
    mask: size_t,
    ix_start: size_t,
    ix_end: size_t,
) {
    StoreRangeH3(&raw mut (*self_0).ha, data, mask, ix_start, ix_end);
    StoreRangeHROLLING_FAST(&raw mut (*self_0).hb, data, mask, ix_start, ix_end);
}
#[inline(always)]
unsafe extern "C" fn PrepareDistanceCacheH35(
    mut self_0: *mut H35,
    mut distance_cache: *mut ::core::ffi::c_int,
) {
    PrepareDistanceCacheH3(&raw mut (*self_0).ha, distance_cache);
    PrepareDistanceCacheHROLLING_FAST(&raw mut (*self_0).hb, distance_cache);
}
#[inline(always)]
unsafe extern "C" fn FindLongestMatchH35(
    mut self_0: *mut H35,
    mut dictionary: *const BrotliEncoderDictionary,
    mut data: *const uint8_t,
    ring_buffer_mask: size_t,
    mut distance_cache: *const ::core::ffi::c_int,
    cur_ix: size_t,
    max_length: size_t,
    max_backward: size_t,
    dictionary_distance: size_t,
    max_distance: size_t,
    mut out: *mut HasherSearchResult,
) {
    FindLongestMatchH3(
        &raw mut (*self_0).ha,
        dictionary,
        data,
        ring_buffer_mask,
        distance_cache,
        cur_ix,
        max_length,
        max_backward,
        dictionary_distance,
        max_distance,
        out,
    );
    FindLongestMatchHROLLING_FAST(
        &raw mut (*self_0).hb,
        dictionary,
        data,
        ring_buffer_mask,
        distance_cache,
        cur_ix,
        max_length,
        max_backward,
        dictionary_distance,
        max_distance,
        out,
    );
}
#[inline(always)]
unsafe extern "C" fn HashTypeLengthH55() -> size_t {
    let mut a: size_t = HashTypeLengthH54();
    let mut b: size_t = HashTypeLengthHROLLING_FAST();
    return if a > b { a } else { b };
}
#[inline(always)]
unsafe extern "C" fn StoreLookaheadH55() -> size_t {
    let mut a: size_t = StoreLookaheadH54();
    let mut b: size_t = StoreLookaheadHROLLING_FAST();
    return if a > b { a } else { b };
}
#[inline(always)]
unsafe extern "C" fn StoreH55(
    mut self_0: *mut H55,
    mut data: *const uint8_t,
    mask: size_t,
    ix: size_t,
) {
    StoreH54(&raw mut (*self_0).ha, data, mask, ix);
    StoreHROLLING_FAST(&raw mut (*self_0).hb, data, mask, ix);
}
#[inline(always)]
unsafe extern "C" fn StoreRangeH55(
    mut self_0: *mut H55,
    mut data: *const uint8_t,
    mask: size_t,
    ix_start: size_t,
    ix_end: size_t,
) {
    StoreRangeH54(&raw mut (*self_0).ha, data, mask, ix_start, ix_end);
    StoreRangeHROLLING_FAST(&raw mut (*self_0).hb, data, mask, ix_start, ix_end);
}
#[inline(always)]
unsafe extern "C" fn PrepareDistanceCacheH55(
    mut self_0: *mut H55,
    mut distance_cache: *mut ::core::ffi::c_int,
) {
    PrepareDistanceCacheH54(&raw mut (*self_0).ha, distance_cache);
    PrepareDistanceCacheHROLLING_FAST(&raw mut (*self_0).hb, distance_cache);
}
#[inline(always)]
unsafe extern "C" fn FindLongestMatchH55(
    mut self_0: *mut H55,
    mut dictionary: *const BrotliEncoderDictionary,
    mut data: *const uint8_t,
    ring_buffer_mask: size_t,
    mut distance_cache: *const ::core::ffi::c_int,
    cur_ix: size_t,
    max_length: size_t,
    max_backward: size_t,
    dictionary_distance: size_t,
    max_distance: size_t,
    mut out: *mut HasherSearchResult,
) {
    FindLongestMatchH54(
        &raw mut (*self_0).ha,
        dictionary,
        data,
        ring_buffer_mask,
        distance_cache,
        cur_ix,
        max_length,
        max_backward,
        dictionary_distance,
        max_distance,
        out,
    );
    FindLongestMatchHROLLING_FAST(
        &raw mut (*self_0).hb,
        dictionary,
        data,
        ring_buffer_mask,
        distance_cache,
        cur_ix,
        max_length,
        max_backward,
        dictionary_distance,
        max_distance,
        out,
    );
}
#[inline(always)]
unsafe extern "C" fn HashTypeLengthH65() -> size_t {
    let mut a: size_t = HashTypeLengthH6();
    let mut b: size_t = HashTypeLengthHROLLING();
    return if a > b { a } else { b };
}
#[inline(always)]
unsafe extern "C" fn StoreLookaheadH65() -> size_t {
    let mut a: size_t = StoreLookaheadH6();
    let mut b: size_t = StoreLookaheadHROLLING();
    return if a > b { a } else { b };
}
#[inline(always)]
unsafe extern "C" fn StoreH65(
    mut self_0: *mut H65,
    mut data: *const uint8_t,
    mask: size_t,
    ix: size_t,
) {
    StoreH6(&raw mut (*self_0).ha, data, mask, ix);
    StoreHROLLING(&raw mut (*self_0).hb, data, mask, ix);
}
#[inline(always)]
unsafe extern "C" fn StoreRangeH65(
    mut self_0: *mut H65,
    mut data: *const uint8_t,
    mask: size_t,
    ix_start: size_t,
    ix_end: size_t,
) {
    StoreRangeH6(&raw mut (*self_0).ha, data, mask, ix_start, ix_end);
    StoreRangeHROLLING(&raw mut (*self_0).hb, data, mask, ix_start, ix_end);
}
#[inline(always)]
unsafe extern "C" fn PrepareDistanceCacheH65(
    mut self_0: *mut H65,
    mut distance_cache: *mut ::core::ffi::c_int,
) {
    PrepareDistanceCacheH6(&raw mut (*self_0).ha, distance_cache);
    PrepareDistanceCacheHROLLING(&raw mut (*self_0).hb, distance_cache);
}
#[inline(always)]
unsafe extern "C" fn FindLongestMatchH65(
    mut self_0: *mut H65,
    mut dictionary: *const BrotliEncoderDictionary,
    mut data: *const uint8_t,
    ring_buffer_mask: size_t,
    mut distance_cache: *const ::core::ffi::c_int,
    cur_ix: size_t,
    max_length: size_t,
    max_backward: size_t,
    dictionary_distance: size_t,
    max_distance: size_t,
    mut out: *mut HasherSearchResult,
) {
    FindLongestMatchH6(
        &raw mut (*self_0).ha,
        dictionary,
        data,
        ring_buffer_mask,
        distance_cache,
        cur_ix,
        max_length,
        max_backward,
        dictionary_distance,
        max_distance,
        out,
    );
    FindLongestMatchHROLLING(
        &raw mut (*self_0).hb,
        dictionary,
        data,
        ring_buffer_mask,
        distance_cache,
        cur_ix,
        max_length,
        max_backward,
        dictionary_distance,
        max_distance,
        out,
    );
}
#[inline(always)]
unsafe extern "C" fn ComputeDistanceCode(
    mut distance: size_t,
    mut max_distance: size_t,
    mut dist_cache: *const ::core::ffi::c_int,
) -> size_t {
    if distance <= max_distance {
        let mut distance_plus_3: size_t = distance.wrapping_add(3 as size_t);
        let mut offset0: size_t = distance_plus_3
            .wrapping_sub(*dist_cache.offset(0 as ::core::ffi::c_int as isize) as size_t);
        let mut offset1: size_t = distance_plus_3
            .wrapping_sub(*dist_cache.offset(1 as ::core::ffi::c_int as isize) as size_t);
        if distance == *dist_cache.offset(0 as ::core::ffi::c_int as isize) as size_t {
            return 0 as size_t;
        } else if distance == *dist_cache.offset(1 as ::core::ffi::c_int as isize) as size_t {
            return 1 as size_t;
        } else if offset0 < 7 as size_t {
            return (0x9750468 as ::core::ffi::c_int >> (4 as size_t).wrapping_mul(offset0)
                & 0xf as ::core::ffi::c_int) as size_t;
        } else if offset1 < 7 as size_t {
            return (0xfdb1ace as ::core::ffi::c_int >> (4 as size_t).wrapping_mul(offset1)
                & 0xf as ::core::ffi::c_int) as size_t;
        } else if distance == *dist_cache.offset(2 as ::core::ffi::c_int as isize) as size_t {
            return 2 as size_t;
        } else if distance == *dist_cache.offset(3 as ::core::ffi::c_int as isize) as size_t {
            return 3 as size_t;
        }
    }
    return distance
        .wrapping_add(BROTLI_NUM_DISTANCE_SHORT_CODES as size_t)
        .wrapping_sub(1 as size_t);
}
pub const ENABLE_COMPOUND_DICTIONARY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const ENABLE_COMPOUND_DICTIONARY_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn BrotliCreateBackwardReferences(
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
    if (*params).dictionary.compound.num_chunks != 0 as size_t {
        match (*params).hasher.type_0 {
            3 => {
                CreateBackwardReferencesDH3(
                    num_bytes,
                    position,
                    ringbuffer,
                    ringbuffer_mask,
                    literal_context_lut,
                    params,
                    hasher,
                    dist_cache,
                    last_insert_len,
                    commands,
                    num_commands,
                    num_literals,
                );
                return;
            }
            4 => {
                CreateBackwardReferencesDH4(
                    num_bytes,
                    position,
                    ringbuffer,
                    ringbuffer_mask,
                    literal_context_lut,
                    params,
                    hasher,
                    dist_cache,
                    last_insert_len,
                    commands,
                    num_commands,
                    num_literals,
                );
                return;
            }
            5 => {
                CreateBackwardReferencesDH5(
                    num_bytes,
                    position,
                    ringbuffer,
                    ringbuffer_mask,
                    literal_context_lut,
                    params,
                    hasher,
                    dist_cache,
                    last_insert_len,
                    commands,
                    num_commands,
                    num_literals,
                );
                return;
            }
            6 => {
                CreateBackwardReferencesDH6(
                    num_bytes,
                    position,
                    ringbuffer,
                    ringbuffer_mask,
                    literal_context_lut,
                    params,
                    hasher,
                    dist_cache,
                    last_insert_len,
                    commands,
                    num_commands,
                    num_literals,
                );
                return;
            }
            58 => {
                CreateBackwardReferencesDH58(
                    num_bytes,
                    position,
                    ringbuffer,
                    ringbuffer_mask,
                    literal_context_lut,
                    params,
                    hasher,
                    dist_cache,
                    last_insert_len,
                    commands,
                    num_commands,
                    num_literals,
                );
                return;
            }
            68 => {
                CreateBackwardReferencesDH68(
                    num_bytes,
                    position,
                    ringbuffer,
                    ringbuffer_mask,
                    literal_context_lut,
                    params,
                    hasher,
                    dist_cache,
                    last_insert_len,
                    commands,
                    num_commands,
                    num_literals,
                );
                return;
            }
            40 => {
                CreateBackwardReferencesDH40(
                    num_bytes,
                    position,
                    ringbuffer,
                    ringbuffer_mask,
                    literal_context_lut,
                    params,
                    hasher,
                    dist_cache,
                    last_insert_len,
                    commands,
                    num_commands,
                    num_literals,
                );
                return;
            }
            41 => {
                CreateBackwardReferencesDH41(
                    num_bytes,
                    position,
                    ringbuffer,
                    ringbuffer_mask,
                    literal_context_lut,
                    params,
                    hasher,
                    dist_cache,
                    last_insert_len,
                    commands,
                    num_commands,
                    num_literals,
                );
                return;
            }
            42 => {
                CreateBackwardReferencesDH42(
                    num_bytes,
                    position,
                    ringbuffer,
                    ringbuffer_mask,
                    literal_context_lut,
                    params,
                    hasher,
                    dist_cache,
                    last_insert_len,
                    commands,
                    num_commands,
                    num_literals,
                );
                return;
            }
            55 => {
                CreateBackwardReferencesDH55(
                    num_bytes,
                    position,
                    ringbuffer,
                    ringbuffer_mask,
                    literal_context_lut,
                    params,
                    hasher,
                    dist_cache,
                    last_insert_len,
                    commands,
                    num_commands,
                    num_literals,
                );
                return;
            }
            65 => {
                CreateBackwardReferencesDH65(
                    num_bytes,
                    position,
                    ringbuffer,
                    ringbuffer_mask,
                    literal_context_lut,
                    params,
                    hasher,
                    dist_cache,
                    last_insert_len,
                    commands,
                    num_commands,
                    num_literals,
                );
                return;
            }
            _ => {}
        }
    }
    match (*params).hasher.type_0 {
        2 => {
            CreateBackwardReferencesNH2(
                num_bytes,
                position,
                ringbuffer,
                ringbuffer_mask,
                literal_context_lut,
                params,
                hasher,
                dist_cache,
                last_insert_len,
                commands,
                num_commands,
                num_literals,
            );
            return;
        }
        3 => {
            CreateBackwardReferencesNH3(
                num_bytes,
                position,
                ringbuffer,
                ringbuffer_mask,
                literal_context_lut,
                params,
                hasher,
                dist_cache,
                last_insert_len,
                commands,
                num_commands,
                num_literals,
            );
            return;
        }
        4 => {
            CreateBackwardReferencesNH4(
                num_bytes,
                position,
                ringbuffer,
                ringbuffer_mask,
                literal_context_lut,
                params,
                hasher,
                dist_cache,
                last_insert_len,
                commands,
                num_commands,
                num_literals,
            );
            return;
        }
        5 => {
            CreateBackwardReferencesNH5(
                num_bytes,
                position,
                ringbuffer,
                ringbuffer_mask,
                literal_context_lut,
                params,
                hasher,
                dist_cache,
                last_insert_len,
                commands,
                num_commands,
                num_literals,
            );
            return;
        }
        6 => {
            CreateBackwardReferencesNH6(
                num_bytes,
                position,
                ringbuffer,
                ringbuffer_mask,
                literal_context_lut,
                params,
                hasher,
                dist_cache,
                last_insert_len,
                commands,
                num_commands,
                num_literals,
            );
            return;
        }
        40 => {
            CreateBackwardReferencesNH40(
                num_bytes,
                position,
                ringbuffer,
                ringbuffer_mask,
                literal_context_lut,
                params,
                hasher,
                dist_cache,
                last_insert_len,
                commands,
                num_commands,
                num_literals,
            );
            return;
        }
        41 => {
            CreateBackwardReferencesNH41(
                num_bytes,
                position,
                ringbuffer,
                ringbuffer_mask,
                literal_context_lut,
                params,
                hasher,
                dist_cache,
                last_insert_len,
                commands,
                num_commands,
                num_literals,
            );
            return;
        }
        42 => {
            CreateBackwardReferencesNH42(
                num_bytes,
                position,
                ringbuffer,
                ringbuffer_mask,
                literal_context_lut,
                params,
                hasher,
                dist_cache,
                last_insert_len,
                commands,
                num_commands,
                num_literals,
            );
            return;
        }
        54 => {
            CreateBackwardReferencesNH54(
                num_bytes,
                position,
                ringbuffer,
                ringbuffer_mask,
                literal_context_lut,
                params,
                hasher,
                dist_cache,
                last_insert_len,
                commands,
                num_commands,
                num_literals,
            );
            return;
        }
        58 => {
            CreateBackwardReferencesNH58(
                num_bytes,
                position,
                ringbuffer,
                ringbuffer_mask,
                literal_context_lut,
                params,
                hasher,
                dist_cache,
                last_insert_len,
                commands,
                num_commands,
                num_literals,
            );
            return;
        }
        68 => {
            CreateBackwardReferencesNH68(
                num_bytes,
                position,
                ringbuffer,
                ringbuffer_mask,
                literal_context_lut,
                params,
                hasher,
                dist_cache,
                last_insert_len,
                commands,
                num_commands,
                num_literals,
            );
            return;
        }
        35 => {
            CreateBackwardReferencesNH35(
                num_bytes,
                position,
                ringbuffer,
                ringbuffer_mask,
                literal_context_lut,
                params,
                hasher,
                dist_cache,
                last_insert_len,
                commands,
                num_commands,
                num_literals,
            );
            return;
        }
        55 => {
            CreateBackwardReferencesNH55(
                num_bytes,
                position,
                ringbuffer,
                ringbuffer_mask,
                literal_context_lut,
                params,
                hasher,
                dist_cache,
                last_insert_len,
                commands,
                num_commands,
                num_literals,
            );
            return;
        }
        65 => {
            CreateBackwardReferencesNH65(
                num_bytes,
                position,
                ringbuffer,
                ringbuffer_mask,
                literal_context_lut,
                params,
                hasher,
                dist_cache,
                last_insert_len,
                commands,
                num_commands,
                num_literals,
            );
            return;
        }
        _ => {}
    };
}
#[inline(never)]
unsafe extern "C" fn CreateBackwardReferencesNH2(
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
    let mut privat: *mut H2 = &raw mut (*hasher).privat._H2;
    let max_backward_limit: size_t = ((1 as ::core::ffi::c_int as size_t) << (*params).lgwin)
        .wrapping_sub(BROTLI_WINDOW_GAP as size_t);
    let position_offset: size_t = (*params).stream_offset;
    let orig_commands: *const Command = commands;
    let mut insert_length: size_t = *last_insert_len;
    let pos_end: size_t = position.wrapping_add(num_bytes);
    let store_end: size_t = if num_bytes >= StoreLookaheadH2() {
        position
            .wrapping_add(num_bytes)
            .wrapping_sub(StoreLookaheadH2() as size_t)
            .wrapping_add(1 as size_t)
    } else {
        position
    };
    let random_heuristics_window_size: size_t = LiteralSpreeLengthForSparseSearch(params) as size_t;
    let mut apply_random_heuristics: size_t = position.wrapping_add(random_heuristics_window_size);
    let gap: size_t = (*params).dictionary.compound.total_size;
    let kMinScore: size_t = BROTLI_SCORE_BASE.wrapping_add(100 as size_t);
    PrepareDistanceCacheH2(privat, dist_cache);
    while position.wrapping_add(HashTypeLengthH2()) < pos_end {
        let mut max_length: size_t = pos_end.wrapping_sub(position);
        let mut max_distance: size_t = brotli_min_size_t(position, max_backward_limit);
        let mut dictionary_start: size_t =
            brotli_min_size_t(position.wrapping_add(position_offset), max_backward_limit);
        let mut sr: HasherSearchResult = HasherSearchResult {
            len: 0,
            distance: 0,
            score: 0,
            len_code_delta: 0,
        };
        let mut dict_id: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut p1: uint8_t = 0 as uint8_t;
        let mut p2: uint8_t = 0 as uint8_t;
        if (*params).dictionary.contextual.context_based != 0 {
            p1 = (if position >= 1 as size_t {
                *ringbuffer.offset((position.wrapping_sub(1 as size_t) & ringbuffer_mask) as isize)
                    as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            }) as uint8_t;
            p2 = (if position >= 2 as size_t {
                *ringbuffer.offset((position.wrapping_sub(2 as size_t) & ringbuffer_mask) as isize)
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
        sr.len = 0 as size_t;
        sr.len_code_delta = 0 as ::core::ffi::c_int;
        sr.distance = 0 as size_t;
        sr.score = kMinScore;
        FindLongestMatchH2(
            privat,
            (*params).dictionary.contextual.dict[dict_id as usize],
            ringbuffer,
            ringbuffer_mask,
            dist_cache,
            position,
            max_length,
            max_distance,
            dictionary_start.wrapping_add(gap),
            (*params).dist.max_distance,
            &raw mut sr,
        );
        if sr.score > kMinScore {
            let mut delayed_backward_references_in_row: ::core::ffi::c_int =
                0 as ::core::ffi::c_int;
            max_length = max_length.wrapping_sub(1);
            loop {
                let cost_diff_lazy: size_t = 175 as size_t;
                let mut sr2: HasherSearchResult = HasherSearchResult {
                    len: 0,
                    distance: 0,
                    score: 0,
                    len_code_delta: 0,
                };
                sr2.len = if (*params).quality < MIN_QUALITY_FOR_EXTENSIVE_REFERENCE_SEARCH {
                    brotli_min_size_t(sr.len.wrapping_sub(1 as size_t), max_length)
                } else {
                    0 as size_t
                };
                sr2.len_code_delta = 0 as ::core::ffi::c_int;
                sr2.distance = 0 as size_t;
                sr2.score = kMinScore;
                max_distance =
                    brotli_min_size_t(position.wrapping_add(1 as size_t), max_backward_limit);
                dictionary_start = brotli_min_size_t(
                    position
                        .wrapping_add(1 as size_t)
                        .wrapping_add(position_offset),
                    max_backward_limit,
                );
                if (*params).dictionary.contextual.context_based != 0 {
                    p2 = p1;
                    p1 = *ringbuffer.offset((position & ringbuffer_mask) as isize);
                    dict_id = (*params).dictionary.contextual.context_map[(*literal_context_lut
                        .offset(p1 as isize)
                        as ::core::ffi::c_int
                        | *literal_context_lut
                            .offset(256 as ::core::ffi::c_int as isize)
                            .offset(p2 as isize) as ::core::ffi::c_int)
                        as usize] as ::core::ffi::c_int;
                }
                FindLongestMatchH2(
                    privat,
                    (*params).dictionary.contextual.dict[dict_id as usize],
                    ringbuffer,
                    ringbuffer_mask,
                    dist_cache,
                    position.wrapping_add(1 as size_t),
                    max_length,
                    max_distance,
                    dictionary_start.wrapping_add(gap),
                    (*params).dist.max_distance,
                    &raw mut sr2,
                );
                if !(sr2.score >= sr.score.wrapping_add(cost_diff_lazy)) {
                    break;
                }
                position = position.wrapping_add(1);
                insert_length = insert_length.wrapping_add(1);
                sr = sr2;
                delayed_backward_references_in_row += 1;
                if !(delayed_backward_references_in_row < 4 as ::core::ffi::c_int
                    && position.wrapping_add(HashTypeLengthH2()) < pos_end)
                {
                    break;
                }
                max_length = max_length.wrapping_sub(1);
            }
            apply_random_heuristics = position
                .wrapping_add((2 as size_t).wrapping_mul(sr.len))
                .wrapping_add(random_heuristics_window_size);
            dictionary_start =
                brotli_min_size_t(position.wrapping_add(position_offset), max_backward_limit);
            let mut distance_code: size_t =
                ComputeDistanceCode(sr.distance, dictionary_start.wrapping_add(gap), dist_cache);
            if sr.distance <= dictionary_start.wrapping_add(gap) && distance_code > 0 as size_t {
                *dist_cache.offset(3 as ::core::ffi::c_int as isize) =
                    *dist_cache.offset(2 as ::core::ffi::c_int as isize);
                *dist_cache.offset(2 as ::core::ffi::c_int as isize) =
                    *dist_cache.offset(1 as ::core::ffi::c_int as isize);
                *dist_cache.offset(1 as ::core::ffi::c_int as isize) =
                    *dist_cache.offset(0 as ::core::ffi::c_int as isize);
                *dist_cache.offset(0 as ::core::ffi::c_int as isize) =
                    sr.distance as ::core::ffi::c_int;
                PrepareDistanceCacheH2(privat, dist_cache);
            }
            let fresh27 = commands;
            commands = commands.offset(1);
            InitCommand(
                fresh27,
                &raw const (*params).dist,
                insert_length,
                sr.len,
                sr.len_code_delta,
                distance_code,
            );
            *num_literals = (*num_literals as ::core::ffi::c_ulong)
                .wrapping_add(insert_length as ::core::ffi::c_ulong)
                as size_t as size_t;
            insert_length = 0 as size_t;
            let mut range_start: size_t = position.wrapping_add(2 as size_t);
            let mut range_end: size_t = brotli_min_size_t(position.wrapping_add(sr.len), store_end);
            if sr.distance < sr.len >> 2 as ::core::ffi::c_int {
                range_start = brotli_min_size_t(
                    range_end,
                    brotli_max_size_t(
                        range_start,
                        position
                            .wrapping_add(sr.len)
                            .wrapping_sub(sr.distance << 2 as ::core::ffi::c_int),
                    ),
                );
            }
            StoreRangeH2(privat, ringbuffer, ringbuffer_mask, range_start, range_end);
            position = (position as ::core::ffi::c_ulong)
                .wrapping_add(sr.len as ::core::ffi::c_ulong) as size_t
                as size_t;
        } else {
            insert_length = insert_length.wrapping_add(1);
            position = position.wrapping_add(1);
            if position > apply_random_heuristics {
                if position
                    > apply_random_heuristics
                        .wrapping_add((4 as size_t).wrapping_mul(random_heuristics_window_size))
                {
                    let kMargin: size_t = brotli_max_size_t(
                        StoreLookaheadH2().wrapping_sub(1 as size_t),
                        4 as size_t,
                    ) as size_t;
                    let mut pos_jump: size_t = brotli_min_size_t(
                        position.wrapping_add(16 as size_t),
                        pos_end.wrapping_sub(kMargin),
                    );
                    while position < pos_jump {
                        StoreH2(privat, ringbuffer, ringbuffer_mask, position);
                        insert_length = (insert_length as ::core::ffi::c_ulong)
                            .wrapping_add(4 as ::core::ffi::c_ulong)
                            as size_t as size_t;
                        position = (position as ::core::ffi::c_ulong)
                            .wrapping_add(4 as ::core::ffi::c_ulong)
                            as size_t as size_t;
                    }
                } else {
                    let kMargin_0: size_t = brotli_max_size_t(
                        StoreLookaheadH2().wrapping_sub(1 as size_t),
                        2 as size_t,
                    ) as size_t;
                    let mut pos_jump_0: size_t = brotli_min_size_t(
                        position.wrapping_add(8 as size_t),
                        pos_end.wrapping_sub(kMargin_0),
                    );
                    while position < pos_jump_0 {
                        StoreH2(privat, ringbuffer, ringbuffer_mask, position);
                        insert_length = (insert_length as ::core::ffi::c_ulong)
                            .wrapping_add(2 as ::core::ffi::c_ulong)
                            as size_t as size_t;
                        position = (position as ::core::ffi::c_ulong)
                            .wrapping_add(2 as ::core::ffi::c_ulong)
                            as size_t as size_t;
                    }
                }
            }
        }
    }
    insert_length = (insert_length as ::core::ffi::c_ulong)
        .wrapping_add(pos_end.wrapping_sub(position) as ::core::ffi::c_ulong)
        as size_t as size_t;
    *last_insert_len = insert_length;
    *num_commands =
        (*num_commands as ::core::ffi::c_ulong).wrapping_add(commands.offset_from(orig_commands)
            as ::core::ffi::c_long
            as size_t
            as ::core::ffi::c_ulong) as size_t as size_t;
}
#[inline(never)]
unsafe extern "C" fn CreateBackwardReferencesNH3(
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
    let mut privat: *mut H3 = &raw mut (*hasher).privat._H3;
    let max_backward_limit: size_t = ((1 as ::core::ffi::c_int as size_t) << (*params).lgwin)
        .wrapping_sub(BROTLI_WINDOW_GAP as size_t);
    let position_offset: size_t = (*params).stream_offset;
    let orig_commands: *const Command = commands;
    let mut insert_length: size_t = *last_insert_len;
    let pos_end: size_t = position.wrapping_add(num_bytes);
    let store_end: size_t = if num_bytes >= StoreLookaheadH3() {
        position
            .wrapping_add(num_bytes)
            .wrapping_sub(StoreLookaheadH3() as size_t)
            .wrapping_add(1 as size_t)
    } else {
        position
    };
    let random_heuristics_window_size: size_t = LiteralSpreeLengthForSparseSearch(params) as size_t;
    let mut apply_random_heuristics: size_t = position.wrapping_add(random_heuristics_window_size);
    let gap: size_t = (*params).dictionary.compound.total_size;
    let kMinScore: size_t = BROTLI_SCORE_BASE.wrapping_add(100 as size_t);
    PrepareDistanceCacheH3(privat, dist_cache);
    while position.wrapping_add(HashTypeLengthH3()) < pos_end {
        let mut max_length: size_t = pos_end.wrapping_sub(position);
        let mut max_distance: size_t = brotli_min_size_t(position, max_backward_limit);
        let mut dictionary_start: size_t =
            brotli_min_size_t(position.wrapping_add(position_offset), max_backward_limit);
        let mut sr: HasherSearchResult = HasherSearchResult {
            len: 0,
            distance: 0,
            score: 0,
            len_code_delta: 0,
        };
        let mut dict_id: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut p1: uint8_t = 0 as uint8_t;
        let mut p2: uint8_t = 0 as uint8_t;
        if (*params).dictionary.contextual.context_based != 0 {
            p1 = (if position >= 1 as size_t {
                *ringbuffer.offset((position.wrapping_sub(1 as size_t) & ringbuffer_mask) as isize)
                    as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            }) as uint8_t;
            p2 = (if position >= 2 as size_t {
                *ringbuffer.offset((position.wrapping_sub(2 as size_t) & ringbuffer_mask) as isize)
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
        sr.len = 0 as size_t;
        sr.len_code_delta = 0 as ::core::ffi::c_int;
        sr.distance = 0 as size_t;
        sr.score = kMinScore;
        FindLongestMatchH3(
            privat,
            (*params).dictionary.contextual.dict[dict_id as usize],
            ringbuffer,
            ringbuffer_mask,
            dist_cache,
            position,
            max_length,
            max_distance,
            dictionary_start.wrapping_add(gap),
            (*params).dist.max_distance,
            &raw mut sr,
        );
        if sr.score > kMinScore {
            let mut delayed_backward_references_in_row: ::core::ffi::c_int =
                0 as ::core::ffi::c_int;
            max_length = max_length.wrapping_sub(1);
            loop {
                let cost_diff_lazy: size_t = 175 as size_t;
                let mut sr2: HasherSearchResult = HasherSearchResult {
                    len: 0,
                    distance: 0,
                    score: 0,
                    len_code_delta: 0,
                };
                sr2.len = if (*params).quality < MIN_QUALITY_FOR_EXTENSIVE_REFERENCE_SEARCH {
                    brotli_min_size_t(sr.len.wrapping_sub(1 as size_t), max_length)
                } else {
                    0 as size_t
                };
                sr2.len_code_delta = 0 as ::core::ffi::c_int;
                sr2.distance = 0 as size_t;
                sr2.score = kMinScore;
                max_distance =
                    brotli_min_size_t(position.wrapping_add(1 as size_t), max_backward_limit);
                dictionary_start = brotli_min_size_t(
                    position
                        .wrapping_add(1 as size_t)
                        .wrapping_add(position_offset),
                    max_backward_limit,
                );
                if (*params).dictionary.contextual.context_based != 0 {
                    p2 = p1;
                    p1 = *ringbuffer.offset((position & ringbuffer_mask) as isize);
                    dict_id = (*params).dictionary.contextual.context_map[(*literal_context_lut
                        .offset(p1 as isize)
                        as ::core::ffi::c_int
                        | *literal_context_lut
                            .offset(256 as ::core::ffi::c_int as isize)
                            .offset(p2 as isize) as ::core::ffi::c_int)
                        as usize] as ::core::ffi::c_int;
                }
                FindLongestMatchH3(
                    privat,
                    (*params).dictionary.contextual.dict[dict_id as usize],
                    ringbuffer,
                    ringbuffer_mask,
                    dist_cache,
                    position.wrapping_add(1 as size_t),
                    max_length,
                    max_distance,
                    dictionary_start.wrapping_add(gap),
                    (*params).dist.max_distance,
                    &raw mut sr2,
                );
                if !(sr2.score >= sr.score.wrapping_add(cost_diff_lazy)) {
                    break;
                }
                position = position.wrapping_add(1);
                insert_length = insert_length.wrapping_add(1);
                sr = sr2;
                delayed_backward_references_in_row += 1;
                if !(delayed_backward_references_in_row < 4 as ::core::ffi::c_int
                    && position.wrapping_add(HashTypeLengthH3()) < pos_end)
                {
                    break;
                }
                max_length = max_length.wrapping_sub(1);
            }
            apply_random_heuristics = position
                .wrapping_add((2 as size_t).wrapping_mul(sr.len))
                .wrapping_add(random_heuristics_window_size);
            dictionary_start =
                brotli_min_size_t(position.wrapping_add(position_offset), max_backward_limit);
            let mut distance_code: size_t =
                ComputeDistanceCode(sr.distance, dictionary_start.wrapping_add(gap), dist_cache);
            if sr.distance <= dictionary_start.wrapping_add(gap) && distance_code > 0 as size_t {
                *dist_cache.offset(3 as ::core::ffi::c_int as isize) =
                    *dist_cache.offset(2 as ::core::ffi::c_int as isize);
                *dist_cache.offset(2 as ::core::ffi::c_int as isize) =
                    *dist_cache.offset(1 as ::core::ffi::c_int as isize);
                *dist_cache.offset(1 as ::core::ffi::c_int as isize) =
                    *dist_cache.offset(0 as ::core::ffi::c_int as isize);
                *dist_cache.offset(0 as ::core::ffi::c_int as isize) =
                    sr.distance as ::core::ffi::c_int;
                PrepareDistanceCacheH3(privat, dist_cache);
            }
            let fresh26 = commands;
            commands = commands.offset(1);
            InitCommand(
                fresh26,
                &raw const (*params).dist,
                insert_length,
                sr.len,
                sr.len_code_delta,
                distance_code,
            );
            *num_literals = (*num_literals as ::core::ffi::c_ulong)
                .wrapping_add(insert_length as ::core::ffi::c_ulong)
                as size_t as size_t;
            insert_length = 0 as size_t;
            let mut range_start: size_t = position.wrapping_add(2 as size_t);
            let mut range_end: size_t = brotli_min_size_t(position.wrapping_add(sr.len), store_end);
            if sr.distance < sr.len >> 2 as ::core::ffi::c_int {
                range_start = brotli_min_size_t(
                    range_end,
                    brotli_max_size_t(
                        range_start,
                        position
                            .wrapping_add(sr.len)
                            .wrapping_sub(sr.distance << 2 as ::core::ffi::c_int),
                    ),
                );
            }
            StoreRangeH3(privat, ringbuffer, ringbuffer_mask, range_start, range_end);
            position = (position as ::core::ffi::c_ulong)
                .wrapping_add(sr.len as ::core::ffi::c_ulong) as size_t
                as size_t;
        } else {
            insert_length = insert_length.wrapping_add(1);
            position = position.wrapping_add(1);
            if position > apply_random_heuristics {
                if position
                    > apply_random_heuristics
                        .wrapping_add((4 as size_t).wrapping_mul(random_heuristics_window_size))
                {
                    let kMargin: size_t = brotli_max_size_t(
                        StoreLookaheadH3().wrapping_sub(1 as size_t),
                        4 as size_t,
                    ) as size_t;
                    let mut pos_jump: size_t = brotli_min_size_t(
                        position.wrapping_add(16 as size_t),
                        pos_end.wrapping_sub(kMargin),
                    );
                    while position < pos_jump {
                        StoreH3(privat, ringbuffer, ringbuffer_mask, position);
                        insert_length = (insert_length as ::core::ffi::c_ulong)
                            .wrapping_add(4 as ::core::ffi::c_ulong)
                            as size_t as size_t;
                        position = (position as ::core::ffi::c_ulong)
                            .wrapping_add(4 as ::core::ffi::c_ulong)
                            as size_t as size_t;
                    }
                } else {
                    let kMargin_0: size_t = brotli_max_size_t(
                        StoreLookaheadH3().wrapping_sub(1 as size_t),
                        2 as size_t,
                    ) as size_t;
                    let mut pos_jump_0: size_t = brotli_min_size_t(
                        position.wrapping_add(8 as size_t),
                        pos_end.wrapping_sub(kMargin_0),
                    );
                    while position < pos_jump_0 {
                        StoreH3(privat, ringbuffer, ringbuffer_mask, position);
                        insert_length = (insert_length as ::core::ffi::c_ulong)
                            .wrapping_add(2 as ::core::ffi::c_ulong)
                            as size_t as size_t;
                        position = (position as ::core::ffi::c_ulong)
                            .wrapping_add(2 as ::core::ffi::c_ulong)
                            as size_t as size_t;
                    }
                }
            }
        }
    }
    insert_length = (insert_length as ::core::ffi::c_ulong)
        .wrapping_add(pos_end.wrapping_sub(position) as ::core::ffi::c_ulong)
        as size_t as size_t;
    *last_insert_len = insert_length;
    *num_commands =
        (*num_commands as ::core::ffi::c_ulong).wrapping_add(commands.offset_from(orig_commands)
            as ::core::ffi::c_long
            as size_t
            as ::core::ffi::c_ulong) as size_t as size_t;
}
#[inline(never)]
unsafe extern "C" fn CreateBackwardReferencesNH4(
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
    let mut privat: *mut H4 = &raw mut (*hasher).privat._H4;
    let max_backward_limit: size_t = ((1 as ::core::ffi::c_int as size_t) << (*params).lgwin)
        .wrapping_sub(BROTLI_WINDOW_GAP as size_t);
    let position_offset: size_t = (*params).stream_offset;
    let orig_commands: *const Command = commands;
    let mut insert_length: size_t = *last_insert_len;
    let pos_end: size_t = position.wrapping_add(num_bytes);
    let store_end: size_t = if num_bytes >= StoreLookaheadH4() {
        position
            .wrapping_add(num_bytes)
            .wrapping_sub(StoreLookaheadH4() as size_t)
            .wrapping_add(1 as size_t)
    } else {
        position
    };
    let random_heuristics_window_size: size_t = LiteralSpreeLengthForSparseSearch(params) as size_t;
    let mut apply_random_heuristics: size_t = position.wrapping_add(random_heuristics_window_size);
    let gap: size_t = (*params).dictionary.compound.total_size;
    let kMinScore: size_t = BROTLI_SCORE_BASE.wrapping_add(100 as size_t);
    PrepareDistanceCacheH4(privat, dist_cache);
    while position.wrapping_add(HashTypeLengthH4()) < pos_end {
        let mut max_length: size_t = pos_end.wrapping_sub(position);
        let mut max_distance: size_t = brotli_min_size_t(position, max_backward_limit);
        let mut dictionary_start: size_t =
            brotli_min_size_t(position.wrapping_add(position_offset), max_backward_limit);
        let mut sr: HasherSearchResult = HasherSearchResult {
            len: 0,
            distance: 0,
            score: 0,
            len_code_delta: 0,
        };
        let mut dict_id: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut p1: uint8_t = 0 as uint8_t;
        let mut p2: uint8_t = 0 as uint8_t;
        if (*params).dictionary.contextual.context_based != 0 {
            p1 = (if position >= 1 as size_t {
                *ringbuffer.offset((position.wrapping_sub(1 as size_t) & ringbuffer_mask) as isize)
                    as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            }) as uint8_t;
            p2 = (if position >= 2 as size_t {
                *ringbuffer.offset((position.wrapping_sub(2 as size_t) & ringbuffer_mask) as isize)
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
        sr.len = 0 as size_t;
        sr.len_code_delta = 0 as ::core::ffi::c_int;
        sr.distance = 0 as size_t;
        sr.score = kMinScore;
        FindLongestMatchH4(
            privat,
            (*params).dictionary.contextual.dict[dict_id as usize],
            ringbuffer,
            ringbuffer_mask,
            dist_cache,
            position,
            max_length,
            max_distance,
            dictionary_start.wrapping_add(gap),
            (*params).dist.max_distance,
            &raw mut sr,
        );
        if sr.score > kMinScore {
            let mut delayed_backward_references_in_row: ::core::ffi::c_int =
                0 as ::core::ffi::c_int;
            max_length = max_length.wrapping_sub(1);
            loop {
                let cost_diff_lazy: size_t = 175 as size_t;
                let mut sr2: HasherSearchResult = HasherSearchResult {
                    len: 0,
                    distance: 0,
                    score: 0,
                    len_code_delta: 0,
                };
                sr2.len = if (*params).quality < MIN_QUALITY_FOR_EXTENSIVE_REFERENCE_SEARCH {
                    brotli_min_size_t(sr.len.wrapping_sub(1 as size_t), max_length)
                } else {
                    0 as size_t
                };
                sr2.len_code_delta = 0 as ::core::ffi::c_int;
                sr2.distance = 0 as size_t;
                sr2.score = kMinScore;
                max_distance =
                    brotli_min_size_t(position.wrapping_add(1 as size_t), max_backward_limit);
                dictionary_start = brotli_min_size_t(
                    position
                        .wrapping_add(1 as size_t)
                        .wrapping_add(position_offset),
                    max_backward_limit,
                );
                if (*params).dictionary.contextual.context_based != 0 {
                    p2 = p1;
                    p1 = *ringbuffer.offset((position & ringbuffer_mask) as isize);
                    dict_id = (*params).dictionary.contextual.context_map[(*literal_context_lut
                        .offset(p1 as isize)
                        as ::core::ffi::c_int
                        | *literal_context_lut
                            .offset(256 as ::core::ffi::c_int as isize)
                            .offset(p2 as isize) as ::core::ffi::c_int)
                        as usize] as ::core::ffi::c_int;
                }
                FindLongestMatchH4(
                    privat,
                    (*params).dictionary.contextual.dict[dict_id as usize],
                    ringbuffer,
                    ringbuffer_mask,
                    dist_cache,
                    position.wrapping_add(1 as size_t),
                    max_length,
                    max_distance,
                    dictionary_start.wrapping_add(gap),
                    (*params).dist.max_distance,
                    &raw mut sr2,
                );
                if !(sr2.score >= sr.score.wrapping_add(cost_diff_lazy)) {
                    break;
                }
                position = position.wrapping_add(1);
                insert_length = insert_length.wrapping_add(1);
                sr = sr2;
                delayed_backward_references_in_row += 1;
                if !(delayed_backward_references_in_row < 4 as ::core::ffi::c_int
                    && position.wrapping_add(HashTypeLengthH4()) < pos_end)
                {
                    break;
                }
                max_length = max_length.wrapping_sub(1);
            }
            apply_random_heuristics = position
                .wrapping_add((2 as size_t).wrapping_mul(sr.len))
                .wrapping_add(random_heuristics_window_size);
            dictionary_start =
                brotli_min_size_t(position.wrapping_add(position_offset), max_backward_limit);
            let mut distance_code: size_t =
                ComputeDistanceCode(sr.distance, dictionary_start.wrapping_add(gap), dist_cache);
            if sr.distance <= dictionary_start.wrapping_add(gap) && distance_code > 0 as size_t {
                *dist_cache.offset(3 as ::core::ffi::c_int as isize) =
                    *dist_cache.offset(2 as ::core::ffi::c_int as isize);
                *dist_cache.offset(2 as ::core::ffi::c_int as isize) =
                    *dist_cache.offset(1 as ::core::ffi::c_int as isize);
                *dist_cache.offset(1 as ::core::ffi::c_int as isize) =
                    *dist_cache.offset(0 as ::core::ffi::c_int as isize);
                *dist_cache.offset(0 as ::core::ffi::c_int as isize) =
                    sr.distance as ::core::ffi::c_int;
                PrepareDistanceCacheH4(privat, dist_cache);
            }
            let fresh25 = commands;
            commands = commands.offset(1);
            InitCommand(
                fresh25,
                &raw const (*params).dist,
                insert_length,
                sr.len,
                sr.len_code_delta,
                distance_code,
            );
            *num_literals = (*num_literals as ::core::ffi::c_ulong)
                .wrapping_add(insert_length as ::core::ffi::c_ulong)
                as size_t as size_t;
            insert_length = 0 as size_t;
            let mut range_start: size_t = position.wrapping_add(2 as size_t);
            let mut range_end: size_t = brotli_min_size_t(position.wrapping_add(sr.len), store_end);
            if sr.distance < sr.len >> 2 as ::core::ffi::c_int {
                range_start = brotli_min_size_t(
                    range_end,
                    brotli_max_size_t(
                        range_start,
                        position
                            .wrapping_add(sr.len)
                            .wrapping_sub(sr.distance << 2 as ::core::ffi::c_int),
                    ),
                );
            }
            StoreRangeH4(privat, ringbuffer, ringbuffer_mask, range_start, range_end);
            position = (position as ::core::ffi::c_ulong)
                .wrapping_add(sr.len as ::core::ffi::c_ulong) as size_t
                as size_t;
        } else {
            insert_length = insert_length.wrapping_add(1);
            position = position.wrapping_add(1);
            if position > apply_random_heuristics {
                if position
                    > apply_random_heuristics
                        .wrapping_add((4 as size_t).wrapping_mul(random_heuristics_window_size))
                {
                    let kMargin: size_t = brotli_max_size_t(
                        StoreLookaheadH4().wrapping_sub(1 as size_t),
                        4 as size_t,
                    ) as size_t;
                    let mut pos_jump: size_t = brotli_min_size_t(
                        position.wrapping_add(16 as size_t),
                        pos_end.wrapping_sub(kMargin),
                    );
                    while position < pos_jump {
                        StoreH4(privat, ringbuffer, ringbuffer_mask, position);
                        insert_length = (insert_length as ::core::ffi::c_ulong)
                            .wrapping_add(4 as ::core::ffi::c_ulong)
                            as size_t as size_t;
                        position = (position as ::core::ffi::c_ulong)
                            .wrapping_add(4 as ::core::ffi::c_ulong)
                            as size_t as size_t;
                    }
                } else {
                    let kMargin_0: size_t = brotli_max_size_t(
                        StoreLookaheadH4().wrapping_sub(1 as size_t),
                        2 as size_t,
                    ) as size_t;
                    let mut pos_jump_0: size_t = brotli_min_size_t(
                        position.wrapping_add(8 as size_t),
                        pos_end.wrapping_sub(kMargin_0),
                    );
                    while position < pos_jump_0 {
                        StoreH4(privat, ringbuffer, ringbuffer_mask, position);
                        insert_length = (insert_length as ::core::ffi::c_ulong)
                            .wrapping_add(2 as ::core::ffi::c_ulong)
                            as size_t as size_t;
                        position = (position as ::core::ffi::c_ulong)
                            .wrapping_add(2 as ::core::ffi::c_ulong)
                            as size_t as size_t;
                    }
                }
            }
        }
    }
    insert_length = (insert_length as ::core::ffi::c_ulong)
        .wrapping_add(pos_end.wrapping_sub(position) as ::core::ffi::c_ulong)
        as size_t as size_t;
    *last_insert_len = insert_length;
    *num_commands =
        (*num_commands as ::core::ffi::c_ulong).wrapping_add(commands.offset_from(orig_commands)
            as ::core::ffi::c_long
            as size_t
            as ::core::ffi::c_ulong) as size_t as size_t;
}
#[inline(never)]
unsafe extern "C" fn CreateBackwardReferencesNH5(
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
    let mut privat: *mut H5 = &raw mut (*hasher).privat._H5;
    let max_backward_limit: size_t = ((1 as ::core::ffi::c_int as size_t) << (*params).lgwin)
        .wrapping_sub(BROTLI_WINDOW_GAP as size_t);
    let position_offset: size_t = (*params).stream_offset;
    let orig_commands: *const Command = commands;
    let mut insert_length: size_t = *last_insert_len;
    let pos_end: size_t = position.wrapping_add(num_bytes);
    let store_end: size_t = if num_bytes >= StoreLookaheadH5() {
        position
            .wrapping_add(num_bytes)
            .wrapping_sub(StoreLookaheadH5() as size_t)
            .wrapping_add(1 as size_t)
    } else {
        position
    };
    let random_heuristics_window_size: size_t = LiteralSpreeLengthForSparseSearch(params) as size_t;
    let mut apply_random_heuristics: size_t = position.wrapping_add(random_heuristics_window_size);
    let gap: size_t = (*params).dictionary.compound.total_size;
    let kMinScore: size_t = BROTLI_SCORE_BASE.wrapping_add(100 as size_t);
    PrepareDistanceCacheH5(privat, dist_cache);
    while position.wrapping_add(HashTypeLengthH5()) < pos_end {
        let mut max_length: size_t = pos_end.wrapping_sub(position);
        let mut max_distance: size_t = brotli_min_size_t(position, max_backward_limit);
        let mut dictionary_start: size_t =
            brotli_min_size_t(position.wrapping_add(position_offset), max_backward_limit);
        let mut sr: HasherSearchResult = HasherSearchResult {
            len: 0,
            distance: 0,
            score: 0,
            len_code_delta: 0,
        };
        let mut dict_id: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut p1: uint8_t = 0 as uint8_t;
        let mut p2: uint8_t = 0 as uint8_t;
        if (*params).dictionary.contextual.context_based != 0 {
            p1 = (if position >= 1 as size_t {
                *ringbuffer.offset((position.wrapping_sub(1 as size_t) & ringbuffer_mask) as isize)
                    as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            }) as uint8_t;
            p2 = (if position >= 2 as size_t {
                *ringbuffer.offset((position.wrapping_sub(2 as size_t) & ringbuffer_mask) as isize)
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
        sr.len = 0 as size_t;
        sr.len_code_delta = 0 as ::core::ffi::c_int;
        sr.distance = 0 as size_t;
        sr.score = kMinScore;
        FindLongestMatchH5(
            privat,
            (*params).dictionary.contextual.dict[dict_id as usize],
            ringbuffer,
            ringbuffer_mask,
            dist_cache,
            position,
            max_length,
            max_distance,
            dictionary_start.wrapping_add(gap),
            (*params).dist.max_distance,
            &raw mut sr,
        );
        if sr.score > kMinScore {
            let mut delayed_backward_references_in_row: ::core::ffi::c_int =
                0 as ::core::ffi::c_int;
            max_length = max_length.wrapping_sub(1);
            loop {
                let cost_diff_lazy: size_t = 175 as size_t;
                let mut sr2: HasherSearchResult = HasherSearchResult {
                    len: 0,
                    distance: 0,
                    score: 0,
                    len_code_delta: 0,
                };
                sr2.len = if (*params).quality < MIN_QUALITY_FOR_EXTENSIVE_REFERENCE_SEARCH {
                    brotli_min_size_t(sr.len.wrapping_sub(1 as size_t), max_length)
                } else {
                    0 as size_t
                };
                sr2.len_code_delta = 0 as ::core::ffi::c_int;
                sr2.distance = 0 as size_t;
                sr2.score = kMinScore;
                max_distance =
                    brotli_min_size_t(position.wrapping_add(1 as size_t), max_backward_limit);
                dictionary_start = brotli_min_size_t(
                    position
                        .wrapping_add(1 as size_t)
                        .wrapping_add(position_offset),
                    max_backward_limit,
                );
                if (*params).dictionary.contextual.context_based != 0 {
                    p2 = p1;
                    p1 = *ringbuffer.offset((position & ringbuffer_mask) as isize);
                    dict_id = (*params).dictionary.contextual.context_map[(*literal_context_lut
                        .offset(p1 as isize)
                        as ::core::ffi::c_int
                        | *literal_context_lut
                            .offset(256 as ::core::ffi::c_int as isize)
                            .offset(p2 as isize) as ::core::ffi::c_int)
                        as usize] as ::core::ffi::c_int;
                }
                FindLongestMatchH5(
                    privat,
                    (*params).dictionary.contextual.dict[dict_id as usize],
                    ringbuffer,
                    ringbuffer_mask,
                    dist_cache,
                    position.wrapping_add(1 as size_t),
                    max_length,
                    max_distance,
                    dictionary_start.wrapping_add(gap),
                    (*params).dist.max_distance,
                    &raw mut sr2,
                );
                if !(sr2.score >= sr.score.wrapping_add(cost_diff_lazy)) {
                    break;
                }
                position = position.wrapping_add(1);
                insert_length = insert_length.wrapping_add(1);
                sr = sr2;
                delayed_backward_references_in_row += 1;
                if !(delayed_backward_references_in_row < 4 as ::core::ffi::c_int
                    && position.wrapping_add(HashTypeLengthH5()) < pos_end)
                {
                    break;
                }
                max_length = max_length.wrapping_sub(1);
            }
            apply_random_heuristics = position
                .wrapping_add((2 as size_t).wrapping_mul(sr.len))
                .wrapping_add(random_heuristics_window_size);
            dictionary_start =
                brotli_min_size_t(position.wrapping_add(position_offset), max_backward_limit);
            let mut distance_code: size_t =
                ComputeDistanceCode(sr.distance, dictionary_start.wrapping_add(gap), dist_cache);
            if sr.distance <= dictionary_start.wrapping_add(gap) && distance_code > 0 as size_t {
                *dist_cache.offset(3 as ::core::ffi::c_int as isize) =
                    *dist_cache.offset(2 as ::core::ffi::c_int as isize);
                *dist_cache.offset(2 as ::core::ffi::c_int as isize) =
                    *dist_cache.offset(1 as ::core::ffi::c_int as isize);
                *dist_cache.offset(1 as ::core::ffi::c_int as isize) =
                    *dist_cache.offset(0 as ::core::ffi::c_int as isize);
                *dist_cache.offset(0 as ::core::ffi::c_int as isize) =
                    sr.distance as ::core::ffi::c_int;
                PrepareDistanceCacheH5(privat, dist_cache);
            }
            let fresh24 = commands;
            commands = commands.offset(1);
            InitCommand(
                fresh24,
                &raw const (*params).dist,
                insert_length,
                sr.len,
                sr.len_code_delta,
                distance_code,
            );
            *num_literals = (*num_literals as ::core::ffi::c_ulong)
                .wrapping_add(insert_length as ::core::ffi::c_ulong)
                as size_t as size_t;
            insert_length = 0 as size_t;
            let mut range_start: size_t = position.wrapping_add(2 as size_t);
            let mut range_end: size_t = brotli_min_size_t(position.wrapping_add(sr.len), store_end);
            if sr.distance < sr.len >> 2 as ::core::ffi::c_int {
                range_start = brotli_min_size_t(
                    range_end,
                    brotli_max_size_t(
                        range_start,
                        position
                            .wrapping_add(sr.len)
                            .wrapping_sub(sr.distance << 2 as ::core::ffi::c_int),
                    ),
                );
            }
            StoreRangeH5(privat, ringbuffer, ringbuffer_mask, range_start, range_end);
            position = (position as ::core::ffi::c_ulong)
                .wrapping_add(sr.len as ::core::ffi::c_ulong) as size_t
                as size_t;
        } else {
            insert_length = insert_length.wrapping_add(1);
            position = position.wrapping_add(1);
            if position > apply_random_heuristics {
                if position
                    > apply_random_heuristics
                        .wrapping_add((4 as size_t).wrapping_mul(random_heuristics_window_size))
                {
                    let kMargin: size_t = brotli_max_size_t(
                        StoreLookaheadH5().wrapping_sub(1 as size_t),
                        4 as size_t,
                    ) as size_t;
                    let mut pos_jump: size_t = brotli_min_size_t(
                        position.wrapping_add(16 as size_t),
                        pos_end.wrapping_sub(kMargin),
                    );
                    while position < pos_jump {
                        StoreH5(privat, ringbuffer, ringbuffer_mask, position);
                        insert_length = (insert_length as ::core::ffi::c_ulong)
                            .wrapping_add(4 as ::core::ffi::c_ulong)
                            as size_t as size_t;
                        position = (position as ::core::ffi::c_ulong)
                            .wrapping_add(4 as ::core::ffi::c_ulong)
                            as size_t as size_t;
                    }
                } else {
                    let kMargin_0: size_t = brotli_max_size_t(
                        StoreLookaheadH5().wrapping_sub(1 as size_t),
                        2 as size_t,
                    ) as size_t;
                    let mut pos_jump_0: size_t = brotli_min_size_t(
                        position.wrapping_add(8 as size_t),
                        pos_end.wrapping_sub(kMargin_0),
                    );
                    while position < pos_jump_0 {
                        StoreH5(privat, ringbuffer, ringbuffer_mask, position);
                        insert_length = (insert_length as ::core::ffi::c_ulong)
                            .wrapping_add(2 as ::core::ffi::c_ulong)
                            as size_t as size_t;
                        position = (position as ::core::ffi::c_ulong)
                            .wrapping_add(2 as ::core::ffi::c_ulong)
                            as size_t as size_t;
                    }
                }
            }
        }
    }
    insert_length = (insert_length as ::core::ffi::c_ulong)
        .wrapping_add(pos_end.wrapping_sub(position) as ::core::ffi::c_ulong)
        as size_t as size_t;
    *last_insert_len = insert_length;
    *num_commands =
        (*num_commands as ::core::ffi::c_ulong).wrapping_add(commands.offset_from(orig_commands)
            as ::core::ffi::c_long
            as size_t
            as ::core::ffi::c_ulong) as size_t as size_t;
}
#[inline(never)]
unsafe extern "C" fn CreateBackwardReferencesNH6(
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
    let mut privat: *mut H6 = &raw mut (*hasher).privat._H6;
    let max_backward_limit: size_t = ((1 as ::core::ffi::c_int as size_t) << (*params).lgwin)
        .wrapping_sub(BROTLI_WINDOW_GAP as size_t);
    let position_offset: size_t = (*params).stream_offset;
    let orig_commands: *const Command = commands;
    let mut insert_length: size_t = *last_insert_len;
    let pos_end: size_t = position.wrapping_add(num_bytes);
    let store_end: size_t = if num_bytes >= StoreLookaheadH6() {
        position
            .wrapping_add(num_bytes)
            .wrapping_sub(StoreLookaheadH6() as size_t)
            .wrapping_add(1 as size_t)
    } else {
        position
    };
    let random_heuristics_window_size: size_t = LiteralSpreeLengthForSparseSearch(params) as size_t;
    let mut apply_random_heuristics: size_t = position.wrapping_add(random_heuristics_window_size);
    let gap: size_t = (*params).dictionary.compound.total_size;
    let kMinScore: size_t = BROTLI_SCORE_BASE.wrapping_add(100 as size_t);
    PrepareDistanceCacheH6(privat, dist_cache);
    while position.wrapping_add(HashTypeLengthH6()) < pos_end {
        let mut max_length: size_t = pos_end.wrapping_sub(position);
        let mut max_distance: size_t = brotli_min_size_t(position, max_backward_limit);
        let mut dictionary_start: size_t =
            brotli_min_size_t(position.wrapping_add(position_offset), max_backward_limit);
        let mut sr: HasherSearchResult = HasherSearchResult {
            len: 0,
            distance: 0,
            score: 0,
            len_code_delta: 0,
        };
        let mut dict_id: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut p1: uint8_t = 0 as uint8_t;
        let mut p2: uint8_t = 0 as uint8_t;
        if (*params).dictionary.contextual.context_based != 0 {
            p1 = (if position >= 1 as size_t {
                *ringbuffer.offset((position.wrapping_sub(1 as size_t) & ringbuffer_mask) as isize)
                    as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            }) as uint8_t;
            p2 = (if position >= 2 as size_t {
                *ringbuffer.offset((position.wrapping_sub(2 as size_t) & ringbuffer_mask) as isize)
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
        sr.len = 0 as size_t;
        sr.len_code_delta = 0 as ::core::ffi::c_int;
        sr.distance = 0 as size_t;
        sr.score = kMinScore;
        FindLongestMatchH6(
            privat,
            (*params).dictionary.contextual.dict[dict_id as usize],
            ringbuffer,
            ringbuffer_mask,
            dist_cache,
            position,
            max_length,
            max_distance,
            dictionary_start.wrapping_add(gap),
            (*params).dist.max_distance,
            &raw mut sr,
        );
        if sr.score > kMinScore {
            let mut delayed_backward_references_in_row: ::core::ffi::c_int =
                0 as ::core::ffi::c_int;
            max_length = max_length.wrapping_sub(1);
            loop {
                let cost_diff_lazy: size_t = 175 as size_t;
                let mut sr2: HasherSearchResult = HasherSearchResult {
                    len: 0,
                    distance: 0,
                    score: 0,
                    len_code_delta: 0,
                };
                sr2.len = if (*params).quality < MIN_QUALITY_FOR_EXTENSIVE_REFERENCE_SEARCH {
                    brotli_min_size_t(sr.len.wrapping_sub(1 as size_t), max_length)
                } else {
                    0 as size_t
                };
                sr2.len_code_delta = 0 as ::core::ffi::c_int;
                sr2.distance = 0 as size_t;
                sr2.score = kMinScore;
                max_distance =
                    brotli_min_size_t(position.wrapping_add(1 as size_t), max_backward_limit);
                dictionary_start = brotli_min_size_t(
                    position
                        .wrapping_add(1 as size_t)
                        .wrapping_add(position_offset),
                    max_backward_limit,
                );
                if (*params).dictionary.contextual.context_based != 0 {
                    p2 = p1;
                    p1 = *ringbuffer.offset((position & ringbuffer_mask) as isize);
                    dict_id = (*params).dictionary.contextual.context_map[(*literal_context_lut
                        .offset(p1 as isize)
                        as ::core::ffi::c_int
                        | *literal_context_lut
                            .offset(256 as ::core::ffi::c_int as isize)
                            .offset(p2 as isize) as ::core::ffi::c_int)
                        as usize] as ::core::ffi::c_int;
                }
                FindLongestMatchH6(
                    privat,
                    (*params).dictionary.contextual.dict[dict_id as usize],
                    ringbuffer,
                    ringbuffer_mask,
                    dist_cache,
                    position.wrapping_add(1 as size_t),
                    max_length,
                    max_distance,
                    dictionary_start.wrapping_add(gap),
                    (*params).dist.max_distance,
                    &raw mut sr2,
                );
                if !(sr2.score >= sr.score.wrapping_add(cost_diff_lazy)) {
                    break;
                }
                position = position.wrapping_add(1);
                insert_length = insert_length.wrapping_add(1);
                sr = sr2;
                delayed_backward_references_in_row += 1;
                if !(delayed_backward_references_in_row < 4 as ::core::ffi::c_int
                    && position.wrapping_add(HashTypeLengthH6()) < pos_end)
                {
                    break;
                }
                max_length = max_length.wrapping_sub(1);
            }
            apply_random_heuristics = position
                .wrapping_add((2 as size_t).wrapping_mul(sr.len))
                .wrapping_add(random_heuristics_window_size);
            dictionary_start =
                brotli_min_size_t(position.wrapping_add(position_offset), max_backward_limit);
            let mut distance_code: size_t =
                ComputeDistanceCode(sr.distance, dictionary_start.wrapping_add(gap), dist_cache);
            if sr.distance <= dictionary_start.wrapping_add(gap) && distance_code > 0 as size_t {
                *dist_cache.offset(3 as ::core::ffi::c_int as isize) =
                    *dist_cache.offset(2 as ::core::ffi::c_int as isize);
                *dist_cache.offset(2 as ::core::ffi::c_int as isize) =
                    *dist_cache.offset(1 as ::core::ffi::c_int as isize);
                *dist_cache.offset(1 as ::core::ffi::c_int as isize) =
                    *dist_cache.offset(0 as ::core::ffi::c_int as isize);
                *dist_cache.offset(0 as ::core::ffi::c_int as isize) =
                    sr.distance as ::core::ffi::c_int;
                PrepareDistanceCacheH6(privat, dist_cache);
            }
            let fresh23 = commands;
            commands = commands.offset(1);
            InitCommand(
                fresh23,
                &raw const (*params).dist,
                insert_length,
                sr.len,
                sr.len_code_delta,
                distance_code,
            );
            *num_literals = (*num_literals as ::core::ffi::c_ulong)
                .wrapping_add(insert_length as ::core::ffi::c_ulong)
                as size_t as size_t;
            insert_length = 0 as size_t;
            let mut range_start: size_t = position.wrapping_add(2 as size_t);
            let mut range_end: size_t = brotli_min_size_t(position.wrapping_add(sr.len), store_end);
            if sr.distance < sr.len >> 2 as ::core::ffi::c_int {
                range_start = brotli_min_size_t(
                    range_end,
                    brotli_max_size_t(
                        range_start,
                        position
                            .wrapping_add(sr.len)
                            .wrapping_sub(sr.distance << 2 as ::core::ffi::c_int),
                    ),
                );
            }
            StoreRangeH6(privat, ringbuffer, ringbuffer_mask, range_start, range_end);
            position = (position as ::core::ffi::c_ulong)
                .wrapping_add(sr.len as ::core::ffi::c_ulong) as size_t
                as size_t;
        } else {
            insert_length = insert_length.wrapping_add(1);
            position = position.wrapping_add(1);
            if position > apply_random_heuristics {
                if position
                    > apply_random_heuristics
                        .wrapping_add((4 as size_t).wrapping_mul(random_heuristics_window_size))
                {
                    let kMargin: size_t = brotli_max_size_t(
                        StoreLookaheadH6().wrapping_sub(1 as size_t),
                        4 as size_t,
                    ) as size_t;
                    let mut pos_jump: size_t = brotli_min_size_t(
                        position.wrapping_add(16 as size_t),
                        pos_end.wrapping_sub(kMargin),
                    );
                    while position < pos_jump {
                        StoreH6(privat, ringbuffer, ringbuffer_mask, position);
                        insert_length = (insert_length as ::core::ffi::c_ulong)
                            .wrapping_add(4 as ::core::ffi::c_ulong)
                            as size_t as size_t;
                        position = (position as ::core::ffi::c_ulong)
                            .wrapping_add(4 as ::core::ffi::c_ulong)
                            as size_t as size_t;
                    }
                } else {
                    let kMargin_0: size_t = brotli_max_size_t(
                        StoreLookaheadH6().wrapping_sub(1 as size_t),
                        2 as size_t,
                    ) as size_t;
                    let mut pos_jump_0: size_t = brotli_min_size_t(
                        position.wrapping_add(8 as size_t),
                        pos_end.wrapping_sub(kMargin_0),
                    );
                    while position < pos_jump_0 {
                        StoreH6(privat, ringbuffer, ringbuffer_mask, position);
                        insert_length = (insert_length as ::core::ffi::c_ulong)
                            .wrapping_add(2 as ::core::ffi::c_ulong)
                            as size_t as size_t;
                        position = (position as ::core::ffi::c_ulong)
                            .wrapping_add(2 as ::core::ffi::c_ulong)
                            as size_t as size_t;
                    }
                }
            }
        }
    }
    insert_length = (insert_length as ::core::ffi::c_ulong)
        .wrapping_add(pos_end.wrapping_sub(position) as ::core::ffi::c_ulong)
        as size_t as size_t;
    *last_insert_len = insert_length;
    *num_commands =
        (*num_commands as ::core::ffi::c_ulong).wrapping_add(commands.offset_from(orig_commands)
            as ::core::ffi::c_long
            as size_t
            as ::core::ffi::c_ulong) as size_t as size_t;
}
#[inline(never)]
unsafe extern "C" fn CreateBackwardReferencesNH40(
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
    let mut privat: *mut H40 = &raw mut (*hasher).privat._H40;
    let max_backward_limit: size_t = ((1 as ::core::ffi::c_int as size_t) << (*params).lgwin)
        .wrapping_sub(BROTLI_WINDOW_GAP as size_t);
    let position_offset: size_t = (*params).stream_offset;
    let orig_commands: *const Command = commands;
    let mut insert_length: size_t = *last_insert_len;
    let pos_end: size_t = position.wrapping_add(num_bytes);
    let store_end: size_t = if num_bytes >= StoreLookaheadH40() {
        position
            .wrapping_add(num_bytes)
            .wrapping_sub(StoreLookaheadH40() as size_t)
            .wrapping_add(1 as size_t)
    } else {
        position
    };
    let random_heuristics_window_size: size_t = LiteralSpreeLengthForSparseSearch(params) as size_t;
    let mut apply_random_heuristics: size_t = position.wrapping_add(random_heuristics_window_size);
    let gap: size_t = (*params).dictionary.compound.total_size;
    let kMinScore: size_t = BROTLI_SCORE_BASE.wrapping_add(100 as size_t);
    PrepareDistanceCacheH40(privat, dist_cache);
    while position.wrapping_add(HashTypeLengthH40()) < pos_end {
        let mut max_length: size_t = pos_end.wrapping_sub(position);
        let mut max_distance: size_t = brotli_min_size_t(position, max_backward_limit);
        let mut dictionary_start: size_t =
            brotli_min_size_t(position.wrapping_add(position_offset), max_backward_limit);
        let mut sr: HasherSearchResult = HasherSearchResult {
            len: 0,
            distance: 0,
            score: 0,
            len_code_delta: 0,
        };
        let mut dict_id: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut p1: uint8_t = 0 as uint8_t;
        let mut p2: uint8_t = 0 as uint8_t;
        if (*params).dictionary.contextual.context_based != 0 {
            p1 = (if position >= 1 as size_t {
                *ringbuffer.offset((position.wrapping_sub(1 as size_t) & ringbuffer_mask) as isize)
                    as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            }) as uint8_t;
            p2 = (if position >= 2 as size_t {
                *ringbuffer.offset((position.wrapping_sub(2 as size_t) & ringbuffer_mask) as isize)
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
        sr.len = 0 as size_t;
        sr.len_code_delta = 0 as ::core::ffi::c_int;
        sr.distance = 0 as size_t;
        sr.score = kMinScore;
        FindLongestMatchH40(
            privat,
            (*params).dictionary.contextual.dict[dict_id as usize],
            ringbuffer,
            ringbuffer_mask,
            dist_cache,
            position,
            max_length,
            max_distance,
            dictionary_start.wrapping_add(gap),
            (*params).dist.max_distance,
            &raw mut sr,
        );
        if sr.score > kMinScore {
            let mut delayed_backward_references_in_row: ::core::ffi::c_int =
                0 as ::core::ffi::c_int;
            max_length = max_length.wrapping_sub(1);
            loop {
                let cost_diff_lazy: size_t = 175 as size_t;
                let mut sr2: HasherSearchResult = HasherSearchResult {
                    len: 0,
                    distance: 0,
                    score: 0,
                    len_code_delta: 0,
                };
                sr2.len = if (*params).quality < MIN_QUALITY_FOR_EXTENSIVE_REFERENCE_SEARCH {
                    brotli_min_size_t(sr.len.wrapping_sub(1 as size_t), max_length)
                } else {
                    0 as size_t
                };
                sr2.len_code_delta = 0 as ::core::ffi::c_int;
                sr2.distance = 0 as size_t;
                sr2.score = kMinScore;
                max_distance =
                    brotli_min_size_t(position.wrapping_add(1 as size_t), max_backward_limit);
                dictionary_start = brotli_min_size_t(
                    position
                        .wrapping_add(1 as size_t)
                        .wrapping_add(position_offset),
                    max_backward_limit,
                );
                if (*params).dictionary.contextual.context_based != 0 {
                    p2 = p1;
                    p1 = *ringbuffer.offset((position & ringbuffer_mask) as isize);
                    dict_id = (*params).dictionary.contextual.context_map[(*literal_context_lut
                        .offset(p1 as isize)
                        as ::core::ffi::c_int
                        | *literal_context_lut
                            .offset(256 as ::core::ffi::c_int as isize)
                            .offset(p2 as isize) as ::core::ffi::c_int)
                        as usize] as ::core::ffi::c_int;
                }
                FindLongestMatchH40(
                    privat,
                    (*params).dictionary.contextual.dict[dict_id as usize],
                    ringbuffer,
                    ringbuffer_mask,
                    dist_cache,
                    position.wrapping_add(1 as size_t),
                    max_length,
                    max_distance,
                    dictionary_start.wrapping_add(gap),
                    (*params).dist.max_distance,
                    &raw mut sr2,
                );
                if !(sr2.score >= sr.score.wrapping_add(cost_diff_lazy)) {
                    break;
                }
                position = position.wrapping_add(1);
                insert_length = insert_length.wrapping_add(1);
                sr = sr2;
                delayed_backward_references_in_row += 1;
                if !(delayed_backward_references_in_row < 4 as ::core::ffi::c_int
                    && position.wrapping_add(HashTypeLengthH40()) < pos_end)
                {
                    break;
                }
                max_length = max_length.wrapping_sub(1);
            }
            apply_random_heuristics = position
                .wrapping_add((2 as size_t).wrapping_mul(sr.len))
                .wrapping_add(random_heuristics_window_size);
            dictionary_start =
                brotli_min_size_t(position.wrapping_add(position_offset), max_backward_limit);
            let mut distance_code: size_t =
                ComputeDistanceCode(sr.distance, dictionary_start.wrapping_add(gap), dist_cache);
            if sr.distance <= dictionary_start.wrapping_add(gap) && distance_code > 0 as size_t {
                *dist_cache.offset(3 as ::core::ffi::c_int as isize) =
                    *dist_cache.offset(2 as ::core::ffi::c_int as isize);
                *dist_cache.offset(2 as ::core::ffi::c_int as isize) =
                    *dist_cache.offset(1 as ::core::ffi::c_int as isize);
                *dist_cache.offset(1 as ::core::ffi::c_int as isize) =
                    *dist_cache.offset(0 as ::core::ffi::c_int as isize);
                *dist_cache.offset(0 as ::core::ffi::c_int as isize) =
                    sr.distance as ::core::ffi::c_int;
                PrepareDistanceCacheH40(privat, dist_cache);
            }
            let fresh22 = commands;
            commands = commands.offset(1);
            InitCommand(
                fresh22,
                &raw const (*params).dist,
                insert_length,
                sr.len,
                sr.len_code_delta,
                distance_code,
            );
            *num_literals = (*num_literals as ::core::ffi::c_ulong)
                .wrapping_add(insert_length as ::core::ffi::c_ulong)
                as size_t as size_t;
            insert_length = 0 as size_t;
            let mut range_start: size_t = position.wrapping_add(2 as size_t);
            let mut range_end: size_t = brotli_min_size_t(position.wrapping_add(sr.len), store_end);
            if sr.distance < sr.len >> 2 as ::core::ffi::c_int {
                range_start = brotli_min_size_t(
                    range_end,
                    brotli_max_size_t(
                        range_start,
                        position
                            .wrapping_add(sr.len)
                            .wrapping_sub(sr.distance << 2 as ::core::ffi::c_int),
                    ),
                );
            }
            StoreRangeH40(privat, ringbuffer, ringbuffer_mask, range_start, range_end);
            position = (position as ::core::ffi::c_ulong)
                .wrapping_add(sr.len as ::core::ffi::c_ulong) as size_t
                as size_t;
        } else {
            insert_length = insert_length.wrapping_add(1);
            position = position.wrapping_add(1);
            if position > apply_random_heuristics {
                if position
                    > apply_random_heuristics
                        .wrapping_add((4 as size_t).wrapping_mul(random_heuristics_window_size))
                {
                    let kMargin: size_t = brotli_max_size_t(
                        StoreLookaheadH40().wrapping_sub(1 as size_t),
                        4 as size_t,
                    ) as size_t;
                    let mut pos_jump: size_t = brotli_min_size_t(
                        position.wrapping_add(16 as size_t),
                        pos_end.wrapping_sub(kMargin),
                    );
                    while position < pos_jump {
                        StoreH40(privat, ringbuffer, ringbuffer_mask, position);
                        insert_length = (insert_length as ::core::ffi::c_ulong)
                            .wrapping_add(4 as ::core::ffi::c_ulong)
                            as size_t as size_t;
                        position = (position as ::core::ffi::c_ulong)
                            .wrapping_add(4 as ::core::ffi::c_ulong)
                            as size_t as size_t;
                    }
                } else {
                    let kMargin_0: size_t = brotli_max_size_t(
                        StoreLookaheadH40().wrapping_sub(1 as size_t),
                        2 as size_t,
                    ) as size_t;
                    let mut pos_jump_0: size_t = brotli_min_size_t(
                        position.wrapping_add(8 as size_t),
                        pos_end.wrapping_sub(kMargin_0),
                    );
                    while position < pos_jump_0 {
                        StoreH40(privat, ringbuffer, ringbuffer_mask, position);
                        insert_length = (insert_length as ::core::ffi::c_ulong)
                            .wrapping_add(2 as ::core::ffi::c_ulong)
                            as size_t as size_t;
                        position = (position as ::core::ffi::c_ulong)
                            .wrapping_add(2 as ::core::ffi::c_ulong)
                            as size_t as size_t;
                    }
                }
            }
        }
    }
    insert_length = (insert_length as ::core::ffi::c_ulong)
        .wrapping_add(pos_end.wrapping_sub(position) as ::core::ffi::c_ulong)
        as size_t as size_t;
    *last_insert_len = insert_length;
    *num_commands =
        (*num_commands as ::core::ffi::c_ulong).wrapping_add(commands.offset_from(orig_commands)
            as ::core::ffi::c_long
            as size_t
            as ::core::ffi::c_ulong) as size_t as size_t;
}
#[inline(never)]
unsafe extern "C" fn CreateBackwardReferencesNH41(
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
    let mut privat: *mut H41 = &raw mut (*hasher).privat._H41;
    let max_backward_limit: size_t = ((1 as ::core::ffi::c_int as size_t) << (*params).lgwin)
        .wrapping_sub(BROTLI_WINDOW_GAP as size_t);
    let position_offset: size_t = (*params).stream_offset;
    let orig_commands: *const Command = commands;
    let mut insert_length: size_t = *last_insert_len;
    let pos_end: size_t = position.wrapping_add(num_bytes);
    let store_end: size_t = if num_bytes >= StoreLookaheadH41() {
        position
            .wrapping_add(num_bytes)
            .wrapping_sub(StoreLookaheadH41() as size_t)
            .wrapping_add(1 as size_t)
    } else {
        position
    };
    let random_heuristics_window_size: size_t = LiteralSpreeLengthForSparseSearch(params) as size_t;
    let mut apply_random_heuristics: size_t = position.wrapping_add(random_heuristics_window_size);
    let gap: size_t = (*params).dictionary.compound.total_size;
    let kMinScore: size_t = BROTLI_SCORE_BASE.wrapping_add(100 as size_t);
    PrepareDistanceCacheH41(privat, dist_cache);
    while position.wrapping_add(HashTypeLengthH41()) < pos_end {
        let mut max_length: size_t = pos_end.wrapping_sub(position);
        let mut max_distance: size_t = brotli_min_size_t(position, max_backward_limit);
        let mut dictionary_start: size_t =
            brotli_min_size_t(position.wrapping_add(position_offset), max_backward_limit);
        let mut sr: HasherSearchResult = HasherSearchResult {
            len: 0,
            distance: 0,
            score: 0,
            len_code_delta: 0,
        };
        let mut dict_id: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut p1: uint8_t = 0 as uint8_t;
        let mut p2: uint8_t = 0 as uint8_t;
        if (*params).dictionary.contextual.context_based != 0 {
            p1 = (if position >= 1 as size_t {
                *ringbuffer.offset((position.wrapping_sub(1 as size_t) & ringbuffer_mask) as isize)
                    as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            }) as uint8_t;
            p2 = (if position >= 2 as size_t {
                *ringbuffer.offset((position.wrapping_sub(2 as size_t) & ringbuffer_mask) as isize)
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
        sr.len = 0 as size_t;
        sr.len_code_delta = 0 as ::core::ffi::c_int;
        sr.distance = 0 as size_t;
        sr.score = kMinScore;
        FindLongestMatchH41(
            privat,
            (*params).dictionary.contextual.dict[dict_id as usize],
            ringbuffer,
            ringbuffer_mask,
            dist_cache,
            position,
            max_length,
            max_distance,
            dictionary_start.wrapping_add(gap),
            (*params).dist.max_distance,
            &raw mut sr,
        );
        if sr.score > kMinScore {
            let mut delayed_backward_references_in_row: ::core::ffi::c_int =
                0 as ::core::ffi::c_int;
            max_length = max_length.wrapping_sub(1);
            loop {
                let cost_diff_lazy: size_t = 175 as size_t;
                let mut sr2: HasherSearchResult = HasherSearchResult {
                    len: 0,
                    distance: 0,
                    score: 0,
                    len_code_delta: 0,
                };
                sr2.len = if (*params).quality < MIN_QUALITY_FOR_EXTENSIVE_REFERENCE_SEARCH {
                    brotli_min_size_t(sr.len.wrapping_sub(1 as size_t), max_length)
                } else {
                    0 as size_t
                };
                sr2.len_code_delta = 0 as ::core::ffi::c_int;
                sr2.distance = 0 as size_t;
                sr2.score = kMinScore;
                max_distance =
                    brotli_min_size_t(position.wrapping_add(1 as size_t), max_backward_limit);
                dictionary_start = brotli_min_size_t(
                    position
                        .wrapping_add(1 as size_t)
                        .wrapping_add(position_offset),
                    max_backward_limit,
                );
                if (*params).dictionary.contextual.context_based != 0 {
                    p2 = p1;
                    p1 = *ringbuffer.offset((position & ringbuffer_mask) as isize);
                    dict_id = (*params).dictionary.contextual.context_map[(*literal_context_lut
                        .offset(p1 as isize)
                        as ::core::ffi::c_int
                        | *literal_context_lut
                            .offset(256 as ::core::ffi::c_int as isize)
                            .offset(p2 as isize) as ::core::ffi::c_int)
                        as usize] as ::core::ffi::c_int;
                }
                FindLongestMatchH41(
                    privat,
                    (*params).dictionary.contextual.dict[dict_id as usize],
                    ringbuffer,
                    ringbuffer_mask,
                    dist_cache,
                    position.wrapping_add(1 as size_t),
                    max_length,
                    max_distance,
                    dictionary_start.wrapping_add(gap),
                    (*params).dist.max_distance,
                    &raw mut sr2,
                );
                if !(sr2.score >= sr.score.wrapping_add(cost_diff_lazy)) {
                    break;
                }
                position = position.wrapping_add(1);
                insert_length = insert_length.wrapping_add(1);
                sr = sr2;
                delayed_backward_references_in_row += 1;
                if !(delayed_backward_references_in_row < 4 as ::core::ffi::c_int
                    && position.wrapping_add(HashTypeLengthH41()) < pos_end)
                {
                    break;
                }
                max_length = max_length.wrapping_sub(1);
            }
            apply_random_heuristics = position
                .wrapping_add((2 as size_t).wrapping_mul(sr.len))
                .wrapping_add(random_heuristics_window_size);
            dictionary_start =
                brotli_min_size_t(position.wrapping_add(position_offset), max_backward_limit);
            let mut distance_code: size_t =
                ComputeDistanceCode(sr.distance, dictionary_start.wrapping_add(gap), dist_cache);
            if sr.distance <= dictionary_start.wrapping_add(gap) && distance_code > 0 as size_t {
                *dist_cache.offset(3 as ::core::ffi::c_int as isize) =
                    *dist_cache.offset(2 as ::core::ffi::c_int as isize);
                *dist_cache.offset(2 as ::core::ffi::c_int as isize) =
                    *dist_cache.offset(1 as ::core::ffi::c_int as isize);
                *dist_cache.offset(1 as ::core::ffi::c_int as isize) =
                    *dist_cache.offset(0 as ::core::ffi::c_int as isize);
                *dist_cache.offset(0 as ::core::ffi::c_int as isize) =
                    sr.distance as ::core::ffi::c_int;
                PrepareDistanceCacheH41(privat, dist_cache);
            }
            let fresh21 = commands;
            commands = commands.offset(1);
            InitCommand(
                fresh21,
                &raw const (*params).dist,
                insert_length,
                sr.len,
                sr.len_code_delta,
                distance_code,
            );
            *num_literals = (*num_literals as ::core::ffi::c_ulong)
                .wrapping_add(insert_length as ::core::ffi::c_ulong)
                as size_t as size_t;
            insert_length = 0 as size_t;
            let mut range_start: size_t = position.wrapping_add(2 as size_t);
            let mut range_end: size_t = brotli_min_size_t(position.wrapping_add(sr.len), store_end);
            if sr.distance < sr.len >> 2 as ::core::ffi::c_int {
                range_start = brotli_min_size_t(
                    range_end,
                    brotli_max_size_t(
                        range_start,
                        position
                            .wrapping_add(sr.len)
                            .wrapping_sub(sr.distance << 2 as ::core::ffi::c_int),
                    ),
                );
            }
            StoreRangeH41(privat, ringbuffer, ringbuffer_mask, range_start, range_end);
            position = (position as ::core::ffi::c_ulong)
                .wrapping_add(sr.len as ::core::ffi::c_ulong) as size_t
                as size_t;
        } else {
            insert_length = insert_length.wrapping_add(1);
            position = position.wrapping_add(1);
            if position > apply_random_heuristics {
                if position
                    > apply_random_heuristics
                        .wrapping_add((4 as size_t).wrapping_mul(random_heuristics_window_size))
                {
                    let kMargin: size_t = brotli_max_size_t(
                        StoreLookaheadH41().wrapping_sub(1 as size_t),
                        4 as size_t,
                    ) as size_t;
                    let mut pos_jump: size_t = brotli_min_size_t(
                        position.wrapping_add(16 as size_t),
                        pos_end.wrapping_sub(kMargin),
                    );
                    while position < pos_jump {
                        StoreH41(privat, ringbuffer, ringbuffer_mask, position);
                        insert_length = (insert_length as ::core::ffi::c_ulong)
                            .wrapping_add(4 as ::core::ffi::c_ulong)
                            as size_t as size_t;
                        position = (position as ::core::ffi::c_ulong)
                            .wrapping_add(4 as ::core::ffi::c_ulong)
                            as size_t as size_t;
                    }
                } else {
                    let kMargin_0: size_t = brotli_max_size_t(
                        StoreLookaheadH41().wrapping_sub(1 as size_t),
                        2 as size_t,
                    ) as size_t;
                    let mut pos_jump_0: size_t = brotli_min_size_t(
                        position.wrapping_add(8 as size_t),
                        pos_end.wrapping_sub(kMargin_0),
                    );
                    while position < pos_jump_0 {
                        StoreH41(privat, ringbuffer, ringbuffer_mask, position);
                        insert_length = (insert_length as ::core::ffi::c_ulong)
                            .wrapping_add(2 as ::core::ffi::c_ulong)
                            as size_t as size_t;
                        position = (position as ::core::ffi::c_ulong)
                            .wrapping_add(2 as ::core::ffi::c_ulong)
                            as size_t as size_t;
                    }
                }
            }
        }
    }
    insert_length = (insert_length as ::core::ffi::c_ulong)
        .wrapping_add(pos_end.wrapping_sub(position) as ::core::ffi::c_ulong)
        as size_t as size_t;
    *last_insert_len = insert_length;
    *num_commands =
        (*num_commands as ::core::ffi::c_ulong).wrapping_add(commands.offset_from(orig_commands)
            as ::core::ffi::c_long
            as size_t
            as ::core::ffi::c_ulong) as size_t as size_t;
}
#[inline(never)]
unsafe extern "C" fn CreateBackwardReferencesNH42(
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
    let mut privat: *mut H42 = &raw mut (*hasher).privat._H42;
    let max_backward_limit: size_t = ((1 as ::core::ffi::c_int as size_t) << (*params).lgwin)
        .wrapping_sub(BROTLI_WINDOW_GAP as size_t);
    let position_offset: size_t = (*params).stream_offset;
    let orig_commands: *const Command = commands;
    let mut insert_length: size_t = *last_insert_len;
    let pos_end: size_t = position.wrapping_add(num_bytes);
    let store_end: size_t = if num_bytes >= StoreLookaheadH42() {
        position
            .wrapping_add(num_bytes)
            .wrapping_sub(StoreLookaheadH42() as size_t)
            .wrapping_add(1 as size_t)
    } else {
        position
    };
    let random_heuristics_window_size: size_t = LiteralSpreeLengthForSparseSearch(params) as size_t;
    let mut apply_random_heuristics: size_t = position.wrapping_add(random_heuristics_window_size);
    let gap: size_t = (*params).dictionary.compound.total_size;
    let kMinScore: size_t = BROTLI_SCORE_BASE.wrapping_add(100 as size_t);
    PrepareDistanceCacheH42(privat, dist_cache);
    while position.wrapping_add(HashTypeLengthH42()) < pos_end {
        let mut max_length: size_t = pos_end.wrapping_sub(position);
        let mut max_distance: size_t = brotli_min_size_t(position, max_backward_limit);
        let mut dictionary_start: size_t =
            brotli_min_size_t(position.wrapping_add(position_offset), max_backward_limit);
        let mut sr: HasherSearchResult = HasherSearchResult {
            len: 0,
            distance: 0,
            score: 0,
            len_code_delta: 0,
        };
        let mut dict_id: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut p1: uint8_t = 0 as uint8_t;
        let mut p2: uint8_t = 0 as uint8_t;
        if (*params).dictionary.contextual.context_based != 0 {
            p1 = (if position >= 1 as size_t {
                *ringbuffer.offset((position.wrapping_sub(1 as size_t) & ringbuffer_mask) as isize)
                    as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            }) as uint8_t;
            p2 = (if position >= 2 as size_t {
                *ringbuffer.offset((position.wrapping_sub(2 as size_t) & ringbuffer_mask) as isize)
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
        sr.len = 0 as size_t;
        sr.len_code_delta = 0 as ::core::ffi::c_int;
        sr.distance = 0 as size_t;
        sr.score = kMinScore;
        FindLongestMatchH42(
            privat,
            (*params).dictionary.contextual.dict[dict_id as usize],
            ringbuffer,
            ringbuffer_mask,
            dist_cache,
            position,
            max_length,
            max_distance,
            dictionary_start.wrapping_add(gap),
            (*params).dist.max_distance,
            &raw mut sr,
        );
        if sr.score > kMinScore {
            let mut delayed_backward_references_in_row: ::core::ffi::c_int =
                0 as ::core::ffi::c_int;
            max_length = max_length.wrapping_sub(1);
            loop {
                let cost_diff_lazy: size_t = 175 as size_t;
                let mut sr2: HasherSearchResult = HasherSearchResult {
                    len: 0,
                    distance: 0,
                    score: 0,
                    len_code_delta: 0,
                };
                sr2.len = if (*params).quality < MIN_QUALITY_FOR_EXTENSIVE_REFERENCE_SEARCH {
                    brotli_min_size_t(sr.len.wrapping_sub(1 as size_t), max_length)
                } else {
                    0 as size_t
                };
                sr2.len_code_delta = 0 as ::core::ffi::c_int;
                sr2.distance = 0 as size_t;
                sr2.score = kMinScore;
                max_distance =
                    brotli_min_size_t(position.wrapping_add(1 as size_t), max_backward_limit);
                dictionary_start = brotli_min_size_t(
                    position
                        .wrapping_add(1 as size_t)
                        .wrapping_add(position_offset),
                    max_backward_limit,
                );
                if (*params).dictionary.contextual.context_based != 0 {
                    p2 = p1;
                    p1 = *ringbuffer.offset((position & ringbuffer_mask) as isize);
                    dict_id = (*params).dictionary.contextual.context_map[(*literal_context_lut
                        .offset(p1 as isize)
                        as ::core::ffi::c_int
                        | *literal_context_lut
                            .offset(256 as ::core::ffi::c_int as isize)
                            .offset(p2 as isize) as ::core::ffi::c_int)
                        as usize] as ::core::ffi::c_int;
                }
                FindLongestMatchH42(
                    privat,
                    (*params).dictionary.contextual.dict[dict_id as usize],
                    ringbuffer,
                    ringbuffer_mask,
                    dist_cache,
                    position.wrapping_add(1 as size_t),
                    max_length,
                    max_distance,
                    dictionary_start.wrapping_add(gap),
                    (*params).dist.max_distance,
                    &raw mut sr2,
                );
                if !(sr2.score >= sr.score.wrapping_add(cost_diff_lazy)) {
                    break;
                }
                position = position.wrapping_add(1);
                insert_length = insert_length.wrapping_add(1);
                sr = sr2;
                delayed_backward_references_in_row += 1;
                if !(delayed_backward_references_in_row < 4 as ::core::ffi::c_int
                    && position.wrapping_add(HashTypeLengthH42()) < pos_end)
                {
                    break;
                }
                max_length = max_length.wrapping_sub(1);
            }
            apply_random_heuristics = position
                .wrapping_add((2 as size_t).wrapping_mul(sr.len))
                .wrapping_add(random_heuristics_window_size);
            dictionary_start =
                brotli_min_size_t(position.wrapping_add(position_offset), max_backward_limit);
            let mut distance_code: size_t =
                ComputeDistanceCode(sr.distance, dictionary_start.wrapping_add(gap), dist_cache);
            if sr.distance <= dictionary_start.wrapping_add(gap) && distance_code > 0 as size_t {
                *dist_cache.offset(3 as ::core::ffi::c_int as isize) =
                    *dist_cache.offset(2 as ::core::ffi::c_int as isize);
                *dist_cache.offset(2 as ::core::ffi::c_int as isize) =
                    *dist_cache.offset(1 as ::core::ffi::c_int as isize);
                *dist_cache.offset(1 as ::core::ffi::c_int as isize) =
                    *dist_cache.offset(0 as ::core::ffi::c_int as isize);
                *dist_cache.offset(0 as ::core::ffi::c_int as isize) =
                    sr.distance as ::core::ffi::c_int;
                PrepareDistanceCacheH42(privat, dist_cache);
            }
            let fresh20 = commands;
            commands = commands.offset(1);
            InitCommand(
                fresh20,
                &raw const (*params).dist,
                insert_length,
                sr.len,
                sr.len_code_delta,
                distance_code,
            );
            *num_literals = (*num_literals as ::core::ffi::c_ulong)
                .wrapping_add(insert_length as ::core::ffi::c_ulong)
                as size_t as size_t;
            insert_length = 0 as size_t;
            let mut range_start: size_t = position.wrapping_add(2 as size_t);
            let mut range_end: size_t = brotli_min_size_t(position.wrapping_add(sr.len), store_end);
            if sr.distance < sr.len >> 2 as ::core::ffi::c_int {
                range_start = brotli_min_size_t(
                    range_end,
                    brotli_max_size_t(
                        range_start,
                        position
                            .wrapping_add(sr.len)
                            .wrapping_sub(sr.distance << 2 as ::core::ffi::c_int),
                    ),
                );
            }
            StoreRangeH42(privat, ringbuffer, ringbuffer_mask, range_start, range_end);
            position = (position as ::core::ffi::c_ulong)
                .wrapping_add(sr.len as ::core::ffi::c_ulong) as size_t
                as size_t;
        } else {
            insert_length = insert_length.wrapping_add(1);
            position = position.wrapping_add(1);
            if position > apply_random_heuristics {
                if position
                    > apply_random_heuristics
                        .wrapping_add((4 as size_t).wrapping_mul(random_heuristics_window_size))
                {
                    let kMargin: size_t = brotli_max_size_t(
                        StoreLookaheadH42().wrapping_sub(1 as size_t),
                        4 as size_t,
                    ) as size_t;
                    let mut pos_jump: size_t = brotli_min_size_t(
                        position.wrapping_add(16 as size_t),
                        pos_end.wrapping_sub(kMargin),
                    );
                    while position < pos_jump {
                        StoreH42(privat, ringbuffer, ringbuffer_mask, position);
                        insert_length = (insert_length as ::core::ffi::c_ulong)
                            .wrapping_add(4 as ::core::ffi::c_ulong)
                            as size_t as size_t;
                        position = (position as ::core::ffi::c_ulong)
                            .wrapping_add(4 as ::core::ffi::c_ulong)
                            as size_t as size_t;
                    }
                } else {
                    let kMargin_0: size_t = brotli_max_size_t(
                        StoreLookaheadH42().wrapping_sub(1 as size_t),
                        2 as size_t,
                    ) as size_t;
                    let mut pos_jump_0: size_t = brotli_min_size_t(
                        position.wrapping_add(8 as size_t),
                        pos_end.wrapping_sub(kMargin_0),
                    );
                    while position < pos_jump_0 {
                        StoreH42(privat, ringbuffer, ringbuffer_mask, position);
                        insert_length = (insert_length as ::core::ffi::c_ulong)
                            .wrapping_add(2 as ::core::ffi::c_ulong)
                            as size_t as size_t;
                        position = (position as ::core::ffi::c_ulong)
                            .wrapping_add(2 as ::core::ffi::c_ulong)
                            as size_t as size_t;
                    }
                }
            }
        }
    }
    insert_length = (insert_length as ::core::ffi::c_ulong)
        .wrapping_add(pos_end.wrapping_sub(position) as ::core::ffi::c_ulong)
        as size_t as size_t;
    *last_insert_len = insert_length;
    *num_commands =
        (*num_commands as ::core::ffi::c_ulong).wrapping_add(commands.offset_from(orig_commands)
            as ::core::ffi::c_long
            as size_t
            as ::core::ffi::c_ulong) as size_t as size_t;
}
#[inline(never)]
unsafe extern "C" fn CreateBackwardReferencesNH54(
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
    let mut privat: *mut H54 = &raw mut (*hasher).privat._H54;
    let max_backward_limit: size_t = ((1 as ::core::ffi::c_int as size_t) << (*params).lgwin)
        .wrapping_sub(BROTLI_WINDOW_GAP as size_t);
    let position_offset: size_t = (*params).stream_offset;
    let orig_commands: *const Command = commands;
    let mut insert_length: size_t = *last_insert_len;
    let pos_end: size_t = position.wrapping_add(num_bytes);
    let store_end: size_t = if num_bytes >= StoreLookaheadH54() {
        position
            .wrapping_add(num_bytes)
            .wrapping_sub(StoreLookaheadH54() as size_t)
            .wrapping_add(1 as size_t)
    } else {
        position
    };
    let random_heuristics_window_size: size_t = LiteralSpreeLengthForSparseSearch(params) as size_t;
    let mut apply_random_heuristics: size_t = position.wrapping_add(random_heuristics_window_size);
    let gap: size_t = (*params).dictionary.compound.total_size;
    let kMinScore: size_t = BROTLI_SCORE_BASE.wrapping_add(100 as size_t);
    PrepareDistanceCacheH54(privat, dist_cache);
    while position.wrapping_add(HashTypeLengthH54()) < pos_end {
        let mut max_length: size_t = pos_end.wrapping_sub(position);
        let mut max_distance: size_t = brotli_min_size_t(position, max_backward_limit);
        let mut dictionary_start: size_t =
            brotli_min_size_t(position.wrapping_add(position_offset), max_backward_limit);
        let mut sr: HasherSearchResult = HasherSearchResult {
            len: 0,
            distance: 0,
            score: 0,
            len_code_delta: 0,
        };
        let mut dict_id: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut p1: uint8_t = 0 as uint8_t;
        let mut p2: uint8_t = 0 as uint8_t;
        if (*params).dictionary.contextual.context_based != 0 {
            p1 = (if position >= 1 as size_t {
                *ringbuffer.offset((position.wrapping_sub(1 as size_t) & ringbuffer_mask) as isize)
                    as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            }) as uint8_t;
            p2 = (if position >= 2 as size_t {
                *ringbuffer.offset((position.wrapping_sub(2 as size_t) & ringbuffer_mask) as isize)
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
        sr.len = 0 as size_t;
        sr.len_code_delta = 0 as ::core::ffi::c_int;
        sr.distance = 0 as size_t;
        sr.score = kMinScore;
        FindLongestMatchH54(
            privat,
            (*params).dictionary.contextual.dict[dict_id as usize],
            ringbuffer,
            ringbuffer_mask,
            dist_cache,
            position,
            max_length,
            max_distance,
            dictionary_start.wrapping_add(gap),
            (*params).dist.max_distance,
            &raw mut sr,
        );
        if sr.score > kMinScore {
            let mut delayed_backward_references_in_row: ::core::ffi::c_int =
                0 as ::core::ffi::c_int;
            max_length = max_length.wrapping_sub(1);
            loop {
                let cost_diff_lazy: size_t = 175 as size_t;
                let mut sr2: HasherSearchResult = HasherSearchResult {
                    len: 0,
                    distance: 0,
                    score: 0,
                    len_code_delta: 0,
                };
                sr2.len = if (*params).quality < MIN_QUALITY_FOR_EXTENSIVE_REFERENCE_SEARCH {
                    brotli_min_size_t(sr.len.wrapping_sub(1 as size_t), max_length)
                } else {
                    0 as size_t
                };
                sr2.len_code_delta = 0 as ::core::ffi::c_int;
                sr2.distance = 0 as size_t;
                sr2.score = kMinScore;
                max_distance =
                    brotli_min_size_t(position.wrapping_add(1 as size_t), max_backward_limit);
                dictionary_start = brotli_min_size_t(
                    position
                        .wrapping_add(1 as size_t)
                        .wrapping_add(position_offset),
                    max_backward_limit,
                );
                if (*params).dictionary.contextual.context_based != 0 {
                    p2 = p1;
                    p1 = *ringbuffer.offset((position & ringbuffer_mask) as isize);
                    dict_id = (*params).dictionary.contextual.context_map[(*literal_context_lut
                        .offset(p1 as isize)
                        as ::core::ffi::c_int
                        | *literal_context_lut
                            .offset(256 as ::core::ffi::c_int as isize)
                            .offset(p2 as isize) as ::core::ffi::c_int)
                        as usize] as ::core::ffi::c_int;
                }
                FindLongestMatchH54(
                    privat,
                    (*params).dictionary.contextual.dict[dict_id as usize],
                    ringbuffer,
                    ringbuffer_mask,
                    dist_cache,
                    position.wrapping_add(1 as size_t),
                    max_length,
                    max_distance,
                    dictionary_start.wrapping_add(gap),
                    (*params).dist.max_distance,
                    &raw mut sr2,
                );
                if !(sr2.score >= sr.score.wrapping_add(cost_diff_lazy)) {
                    break;
                }
                position = position.wrapping_add(1);
                insert_length = insert_length.wrapping_add(1);
                sr = sr2;
                delayed_backward_references_in_row += 1;
                if !(delayed_backward_references_in_row < 4 as ::core::ffi::c_int
                    && position.wrapping_add(HashTypeLengthH54()) < pos_end)
                {
                    break;
                }
                max_length = max_length.wrapping_sub(1);
            }
            apply_random_heuristics = position
                .wrapping_add((2 as size_t).wrapping_mul(sr.len))
                .wrapping_add(random_heuristics_window_size);
            dictionary_start =
                brotli_min_size_t(position.wrapping_add(position_offset), max_backward_limit);
            let mut distance_code: size_t =
                ComputeDistanceCode(sr.distance, dictionary_start.wrapping_add(gap), dist_cache);
            if sr.distance <= dictionary_start.wrapping_add(gap) && distance_code > 0 as size_t {
                *dist_cache.offset(3 as ::core::ffi::c_int as isize) =
                    *dist_cache.offset(2 as ::core::ffi::c_int as isize);
                *dist_cache.offset(2 as ::core::ffi::c_int as isize) =
                    *dist_cache.offset(1 as ::core::ffi::c_int as isize);
                *dist_cache.offset(1 as ::core::ffi::c_int as isize) =
                    *dist_cache.offset(0 as ::core::ffi::c_int as isize);
                *dist_cache.offset(0 as ::core::ffi::c_int as isize) =
                    sr.distance as ::core::ffi::c_int;
                PrepareDistanceCacheH54(privat, dist_cache);
            }
            let fresh19 = commands;
            commands = commands.offset(1);
            InitCommand(
                fresh19,
                &raw const (*params).dist,
                insert_length,
                sr.len,
                sr.len_code_delta,
                distance_code,
            );
            *num_literals = (*num_literals as ::core::ffi::c_ulong)
                .wrapping_add(insert_length as ::core::ffi::c_ulong)
                as size_t as size_t;
            insert_length = 0 as size_t;
            let mut range_start: size_t = position.wrapping_add(2 as size_t);
            let mut range_end: size_t = brotli_min_size_t(position.wrapping_add(sr.len), store_end);
            if sr.distance < sr.len >> 2 as ::core::ffi::c_int {
                range_start = brotli_min_size_t(
                    range_end,
                    brotli_max_size_t(
                        range_start,
                        position
                            .wrapping_add(sr.len)
                            .wrapping_sub(sr.distance << 2 as ::core::ffi::c_int),
                    ),
                );
            }
            StoreRangeH54(privat, ringbuffer, ringbuffer_mask, range_start, range_end);
            position = (position as ::core::ffi::c_ulong)
                .wrapping_add(sr.len as ::core::ffi::c_ulong) as size_t
                as size_t;
        } else {
            insert_length = insert_length.wrapping_add(1);
            position = position.wrapping_add(1);
            if position > apply_random_heuristics {
                if position
                    > apply_random_heuristics
                        .wrapping_add((4 as size_t).wrapping_mul(random_heuristics_window_size))
                {
                    let kMargin: size_t = brotli_max_size_t(
                        StoreLookaheadH54().wrapping_sub(1 as size_t),
                        4 as size_t,
                    ) as size_t;
                    let mut pos_jump: size_t = brotli_min_size_t(
                        position.wrapping_add(16 as size_t),
                        pos_end.wrapping_sub(kMargin),
                    );
                    while position < pos_jump {
                        StoreH54(privat, ringbuffer, ringbuffer_mask, position);
                        insert_length = (insert_length as ::core::ffi::c_ulong)
                            .wrapping_add(4 as ::core::ffi::c_ulong)
                            as size_t as size_t;
                        position = (position as ::core::ffi::c_ulong)
                            .wrapping_add(4 as ::core::ffi::c_ulong)
                            as size_t as size_t;
                    }
                } else {
                    let kMargin_0: size_t = brotli_max_size_t(
                        StoreLookaheadH54().wrapping_sub(1 as size_t),
                        2 as size_t,
                    ) as size_t;
                    let mut pos_jump_0: size_t = brotli_min_size_t(
                        position.wrapping_add(8 as size_t),
                        pos_end.wrapping_sub(kMargin_0),
                    );
                    while position < pos_jump_0 {
                        StoreH54(privat, ringbuffer, ringbuffer_mask, position);
                        insert_length = (insert_length as ::core::ffi::c_ulong)
                            .wrapping_add(2 as ::core::ffi::c_ulong)
                            as size_t as size_t;
                        position = (position as ::core::ffi::c_ulong)
                            .wrapping_add(2 as ::core::ffi::c_ulong)
                            as size_t as size_t;
                    }
                }
            }
        }
    }
    insert_length = (insert_length as ::core::ffi::c_ulong)
        .wrapping_add(pos_end.wrapping_sub(position) as ::core::ffi::c_ulong)
        as size_t as size_t;
    *last_insert_len = insert_length;
    *num_commands =
        (*num_commands as ::core::ffi::c_ulong).wrapping_add(commands.offset_from(orig_commands)
            as ::core::ffi::c_long
            as size_t
            as ::core::ffi::c_ulong) as size_t as size_t;
}
#[inline(never)]
unsafe extern "C" fn CreateBackwardReferencesNH35(
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
    let mut privat: *mut H35 = &raw mut (*hasher).privat._H35;
    let max_backward_limit: size_t = ((1 as ::core::ffi::c_int as size_t) << (*params).lgwin)
        .wrapping_sub(BROTLI_WINDOW_GAP as size_t);
    let position_offset: size_t = (*params).stream_offset;
    let orig_commands: *const Command = commands;
    let mut insert_length: size_t = *last_insert_len;
    let pos_end: size_t = position.wrapping_add(num_bytes);
    let store_end: size_t = if num_bytes >= StoreLookaheadH35() {
        position
            .wrapping_add(num_bytes)
            .wrapping_sub(StoreLookaheadH35() as size_t)
            .wrapping_add(1 as size_t)
    } else {
        position
    };
    let random_heuristics_window_size: size_t = LiteralSpreeLengthForSparseSearch(params) as size_t;
    let mut apply_random_heuristics: size_t = position.wrapping_add(random_heuristics_window_size);
    let gap: size_t = (*params).dictionary.compound.total_size;
    let kMinScore: size_t = BROTLI_SCORE_BASE.wrapping_add(100 as size_t);
    PrepareDistanceCacheH35(privat, dist_cache);
    while position.wrapping_add(HashTypeLengthH35()) < pos_end {
        let mut max_length: size_t = pos_end.wrapping_sub(position);
        let mut max_distance: size_t = brotli_min_size_t(position, max_backward_limit);
        let mut dictionary_start: size_t =
            brotli_min_size_t(position.wrapping_add(position_offset), max_backward_limit);
        let mut sr: HasherSearchResult = HasherSearchResult {
            len: 0,
            distance: 0,
            score: 0,
            len_code_delta: 0,
        };
        let mut dict_id: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut p1: uint8_t = 0 as uint8_t;
        let mut p2: uint8_t = 0 as uint8_t;
        if (*params).dictionary.contextual.context_based != 0 {
            p1 = (if position >= 1 as size_t {
                *ringbuffer.offset((position.wrapping_sub(1 as size_t) & ringbuffer_mask) as isize)
                    as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            }) as uint8_t;
            p2 = (if position >= 2 as size_t {
                *ringbuffer.offset((position.wrapping_sub(2 as size_t) & ringbuffer_mask) as isize)
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
        sr.len = 0 as size_t;
        sr.len_code_delta = 0 as ::core::ffi::c_int;
        sr.distance = 0 as size_t;
        sr.score = kMinScore;
        FindLongestMatchH35(
            privat,
            (*params).dictionary.contextual.dict[dict_id as usize],
            ringbuffer,
            ringbuffer_mask,
            dist_cache,
            position,
            max_length,
            max_distance,
            dictionary_start.wrapping_add(gap),
            (*params).dist.max_distance,
            &raw mut sr,
        );
        if sr.score > kMinScore {
            let mut delayed_backward_references_in_row: ::core::ffi::c_int =
                0 as ::core::ffi::c_int;
            max_length = max_length.wrapping_sub(1);
            loop {
                let cost_diff_lazy: size_t = 175 as size_t;
                let mut sr2: HasherSearchResult = HasherSearchResult {
                    len: 0,
                    distance: 0,
                    score: 0,
                    len_code_delta: 0,
                };
                sr2.len = if (*params).quality < MIN_QUALITY_FOR_EXTENSIVE_REFERENCE_SEARCH {
                    brotli_min_size_t(sr.len.wrapping_sub(1 as size_t), max_length)
                } else {
                    0 as size_t
                };
                sr2.len_code_delta = 0 as ::core::ffi::c_int;
                sr2.distance = 0 as size_t;
                sr2.score = kMinScore;
                max_distance =
                    brotli_min_size_t(position.wrapping_add(1 as size_t), max_backward_limit);
                dictionary_start = brotli_min_size_t(
                    position
                        .wrapping_add(1 as size_t)
                        .wrapping_add(position_offset),
                    max_backward_limit,
                );
                if (*params).dictionary.contextual.context_based != 0 {
                    p2 = p1;
                    p1 = *ringbuffer.offset((position & ringbuffer_mask) as isize);
                    dict_id = (*params).dictionary.contextual.context_map[(*literal_context_lut
                        .offset(p1 as isize)
                        as ::core::ffi::c_int
                        | *literal_context_lut
                            .offset(256 as ::core::ffi::c_int as isize)
                            .offset(p2 as isize) as ::core::ffi::c_int)
                        as usize] as ::core::ffi::c_int;
                }
                FindLongestMatchH35(
                    privat,
                    (*params).dictionary.contextual.dict[dict_id as usize],
                    ringbuffer,
                    ringbuffer_mask,
                    dist_cache,
                    position.wrapping_add(1 as size_t),
                    max_length,
                    max_distance,
                    dictionary_start.wrapping_add(gap),
                    (*params).dist.max_distance,
                    &raw mut sr2,
                );
                if !(sr2.score >= sr.score.wrapping_add(cost_diff_lazy)) {
                    break;
                }
                position = position.wrapping_add(1);
                insert_length = insert_length.wrapping_add(1);
                sr = sr2;
                delayed_backward_references_in_row += 1;
                if !(delayed_backward_references_in_row < 4 as ::core::ffi::c_int
                    && position.wrapping_add(HashTypeLengthH35()) < pos_end)
                {
                    break;
                }
                max_length = max_length.wrapping_sub(1);
            }
            apply_random_heuristics = position
                .wrapping_add((2 as size_t).wrapping_mul(sr.len))
                .wrapping_add(random_heuristics_window_size);
            dictionary_start =
                brotli_min_size_t(position.wrapping_add(position_offset), max_backward_limit);
            let mut distance_code: size_t =
                ComputeDistanceCode(sr.distance, dictionary_start.wrapping_add(gap), dist_cache);
            if sr.distance <= dictionary_start.wrapping_add(gap) && distance_code > 0 as size_t {
                *dist_cache.offset(3 as ::core::ffi::c_int as isize) =
                    *dist_cache.offset(2 as ::core::ffi::c_int as isize);
                *dist_cache.offset(2 as ::core::ffi::c_int as isize) =
                    *dist_cache.offset(1 as ::core::ffi::c_int as isize);
                *dist_cache.offset(1 as ::core::ffi::c_int as isize) =
                    *dist_cache.offset(0 as ::core::ffi::c_int as isize);
                *dist_cache.offset(0 as ::core::ffi::c_int as isize) =
                    sr.distance as ::core::ffi::c_int;
                PrepareDistanceCacheH35(privat, dist_cache);
            }
            let fresh16 = commands;
            commands = commands.offset(1);
            InitCommand(
                fresh16,
                &raw const (*params).dist,
                insert_length,
                sr.len,
                sr.len_code_delta,
                distance_code,
            );
            *num_literals = (*num_literals as ::core::ffi::c_ulong)
                .wrapping_add(insert_length as ::core::ffi::c_ulong)
                as size_t as size_t;
            insert_length = 0 as size_t;
            let mut range_start: size_t = position.wrapping_add(2 as size_t);
            let mut range_end: size_t = brotli_min_size_t(position.wrapping_add(sr.len), store_end);
            if sr.distance < sr.len >> 2 as ::core::ffi::c_int {
                range_start = brotli_min_size_t(
                    range_end,
                    brotli_max_size_t(
                        range_start,
                        position
                            .wrapping_add(sr.len)
                            .wrapping_sub(sr.distance << 2 as ::core::ffi::c_int),
                    ),
                );
            }
            StoreRangeH35(privat, ringbuffer, ringbuffer_mask, range_start, range_end);
            position = (position as ::core::ffi::c_ulong)
                .wrapping_add(sr.len as ::core::ffi::c_ulong) as size_t
                as size_t;
        } else {
            insert_length = insert_length.wrapping_add(1);
            position = position.wrapping_add(1);
            if position > apply_random_heuristics {
                if position
                    > apply_random_heuristics
                        .wrapping_add((4 as size_t).wrapping_mul(random_heuristics_window_size))
                {
                    let kMargin: size_t = brotli_max_size_t(
                        StoreLookaheadH35().wrapping_sub(1 as size_t),
                        4 as size_t,
                    ) as size_t;
                    let mut pos_jump: size_t = brotli_min_size_t(
                        position.wrapping_add(16 as size_t),
                        pos_end.wrapping_sub(kMargin),
                    );
                    while position < pos_jump {
                        StoreH35(privat, ringbuffer, ringbuffer_mask, position);
                        insert_length = (insert_length as ::core::ffi::c_ulong)
                            .wrapping_add(4 as ::core::ffi::c_ulong)
                            as size_t as size_t;
                        position = (position as ::core::ffi::c_ulong)
                            .wrapping_add(4 as ::core::ffi::c_ulong)
                            as size_t as size_t;
                    }
                } else {
                    let kMargin_0: size_t = brotli_max_size_t(
                        StoreLookaheadH35().wrapping_sub(1 as size_t),
                        2 as size_t,
                    ) as size_t;
                    let mut pos_jump_0: size_t = brotli_min_size_t(
                        position.wrapping_add(8 as size_t),
                        pos_end.wrapping_sub(kMargin_0),
                    );
                    while position < pos_jump_0 {
                        StoreH35(privat, ringbuffer, ringbuffer_mask, position);
                        insert_length = (insert_length as ::core::ffi::c_ulong)
                            .wrapping_add(2 as ::core::ffi::c_ulong)
                            as size_t as size_t;
                        position = (position as ::core::ffi::c_ulong)
                            .wrapping_add(2 as ::core::ffi::c_ulong)
                            as size_t as size_t;
                    }
                }
            }
        }
    }
    insert_length = (insert_length as ::core::ffi::c_ulong)
        .wrapping_add(pos_end.wrapping_sub(position) as ::core::ffi::c_ulong)
        as size_t as size_t;
    *last_insert_len = insert_length;
    *num_commands =
        (*num_commands as ::core::ffi::c_ulong).wrapping_add(commands.offset_from(orig_commands)
            as ::core::ffi::c_long
            as size_t
            as ::core::ffi::c_ulong) as size_t as size_t;
}
#[inline(never)]
unsafe extern "C" fn CreateBackwardReferencesNH55(
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
    let mut privat: *mut H55 = &raw mut (*hasher).privat._H55;
    let max_backward_limit: size_t = ((1 as ::core::ffi::c_int as size_t) << (*params).lgwin)
        .wrapping_sub(BROTLI_WINDOW_GAP as size_t);
    let position_offset: size_t = (*params).stream_offset;
    let orig_commands: *const Command = commands;
    let mut insert_length: size_t = *last_insert_len;
    let pos_end: size_t = position.wrapping_add(num_bytes);
    let store_end: size_t = if num_bytes >= StoreLookaheadH55() {
        position
            .wrapping_add(num_bytes)
            .wrapping_sub(StoreLookaheadH55() as size_t)
            .wrapping_add(1 as size_t)
    } else {
        position
    };
    let random_heuristics_window_size: size_t = LiteralSpreeLengthForSparseSearch(params) as size_t;
    let mut apply_random_heuristics: size_t = position.wrapping_add(random_heuristics_window_size);
    let gap: size_t = (*params).dictionary.compound.total_size;
    let kMinScore: size_t = BROTLI_SCORE_BASE.wrapping_add(100 as size_t);
    PrepareDistanceCacheH55(privat, dist_cache);
    while position.wrapping_add(HashTypeLengthH55()) < pos_end {
        let mut max_length: size_t = pos_end.wrapping_sub(position);
        let mut max_distance: size_t = brotli_min_size_t(position, max_backward_limit);
        let mut dictionary_start: size_t =
            brotli_min_size_t(position.wrapping_add(position_offset), max_backward_limit);
        let mut sr: HasherSearchResult = HasherSearchResult {
            len: 0,
            distance: 0,
            score: 0,
            len_code_delta: 0,
        };
        let mut dict_id: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut p1: uint8_t = 0 as uint8_t;
        let mut p2: uint8_t = 0 as uint8_t;
        if (*params).dictionary.contextual.context_based != 0 {
            p1 = (if position >= 1 as size_t {
                *ringbuffer.offset((position.wrapping_sub(1 as size_t) & ringbuffer_mask) as isize)
                    as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            }) as uint8_t;
            p2 = (if position >= 2 as size_t {
                *ringbuffer.offset((position.wrapping_sub(2 as size_t) & ringbuffer_mask) as isize)
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
        sr.len = 0 as size_t;
        sr.len_code_delta = 0 as ::core::ffi::c_int;
        sr.distance = 0 as size_t;
        sr.score = kMinScore;
        FindLongestMatchH55(
            privat,
            (*params).dictionary.contextual.dict[dict_id as usize],
            ringbuffer,
            ringbuffer_mask,
            dist_cache,
            position,
            max_length,
            max_distance,
            dictionary_start.wrapping_add(gap),
            (*params).dist.max_distance,
            &raw mut sr,
        );
        if sr.score > kMinScore {
            let mut delayed_backward_references_in_row: ::core::ffi::c_int =
                0 as ::core::ffi::c_int;
            max_length = max_length.wrapping_sub(1);
            loop {
                let cost_diff_lazy: size_t = 175 as size_t;
                let mut sr2: HasherSearchResult = HasherSearchResult {
                    len: 0,
                    distance: 0,
                    score: 0,
                    len_code_delta: 0,
                };
                sr2.len = if (*params).quality < MIN_QUALITY_FOR_EXTENSIVE_REFERENCE_SEARCH {
                    brotli_min_size_t(sr.len.wrapping_sub(1 as size_t), max_length)
                } else {
                    0 as size_t
                };
                sr2.len_code_delta = 0 as ::core::ffi::c_int;
                sr2.distance = 0 as size_t;
                sr2.score = kMinScore;
                max_distance =
                    brotli_min_size_t(position.wrapping_add(1 as size_t), max_backward_limit);
                dictionary_start = brotli_min_size_t(
                    position
                        .wrapping_add(1 as size_t)
                        .wrapping_add(position_offset),
                    max_backward_limit,
                );
                if (*params).dictionary.contextual.context_based != 0 {
                    p2 = p1;
                    p1 = *ringbuffer.offset((position & ringbuffer_mask) as isize);
                    dict_id = (*params).dictionary.contextual.context_map[(*literal_context_lut
                        .offset(p1 as isize)
                        as ::core::ffi::c_int
                        | *literal_context_lut
                            .offset(256 as ::core::ffi::c_int as isize)
                            .offset(p2 as isize) as ::core::ffi::c_int)
                        as usize] as ::core::ffi::c_int;
                }
                FindLongestMatchH55(
                    privat,
                    (*params).dictionary.contextual.dict[dict_id as usize],
                    ringbuffer,
                    ringbuffer_mask,
                    dist_cache,
                    position.wrapping_add(1 as size_t),
                    max_length,
                    max_distance,
                    dictionary_start.wrapping_add(gap),
                    (*params).dist.max_distance,
                    &raw mut sr2,
                );
                if !(sr2.score >= sr.score.wrapping_add(cost_diff_lazy)) {
                    break;
                }
                position = position.wrapping_add(1);
                insert_length = insert_length.wrapping_add(1);
                sr = sr2;
                delayed_backward_references_in_row += 1;
                if !(delayed_backward_references_in_row < 4 as ::core::ffi::c_int
                    && position.wrapping_add(HashTypeLengthH55()) < pos_end)
                {
                    break;
                }
                max_length = max_length.wrapping_sub(1);
            }
            apply_random_heuristics = position
                .wrapping_add((2 as size_t).wrapping_mul(sr.len))
                .wrapping_add(random_heuristics_window_size);
            dictionary_start =
                brotli_min_size_t(position.wrapping_add(position_offset), max_backward_limit);
            let mut distance_code: size_t =
                ComputeDistanceCode(sr.distance, dictionary_start.wrapping_add(gap), dist_cache);
            if sr.distance <= dictionary_start.wrapping_add(gap) && distance_code > 0 as size_t {
                *dist_cache.offset(3 as ::core::ffi::c_int as isize) =
                    *dist_cache.offset(2 as ::core::ffi::c_int as isize);
                *dist_cache.offset(2 as ::core::ffi::c_int as isize) =
                    *dist_cache.offset(1 as ::core::ffi::c_int as isize);
                *dist_cache.offset(1 as ::core::ffi::c_int as isize) =
                    *dist_cache.offset(0 as ::core::ffi::c_int as isize);
                *dist_cache.offset(0 as ::core::ffi::c_int as isize) =
                    sr.distance as ::core::ffi::c_int;
                PrepareDistanceCacheH55(privat, dist_cache);
            }
            let fresh15 = commands;
            commands = commands.offset(1);
            InitCommand(
                fresh15,
                &raw const (*params).dist,
                insert_length,
                sr.len,
                sr.len_code_delta,
                distance_code,
            );
            *num_literals = (*num_literals as ::core::ffi::c_ulong)
                .wrapping_add(insert_length as ::core::ffi::c_ulong)
                as size_t as size_t;
            insert_length = 0 as size_t;
            let mut range_start: size_t = position.wrapping_add(2 as size_t);
            let mut range_end: size_t = brotli_min_size_t(position.wrapping_add(sr.len), store_end);
            if sr.distance < sr.len >> 2 as ::core::ffi::c_int {
                range_start = brotli_min_size_t(
                    range_end,
                    brotli_max_size_t(
                        range_start,
                        position
                            .wrapping_add(sr.len)
                            .wrapping_sub(sr.distance << 2 as ::core::ffi::c_int),
                    ),
                );
            }
            StoreRangeH55(privat, ringbuffer, ringbuffer_mask, range_start, range_end);
            position = (position as ::core::ffi::c_ulong)
                .wrapping_add(sr.len as ::core::ffi::c_ulong) as size_t
                as size_t;
        } else {
            insert_length = insert_length.wrapping_add(1);
            position = position.wrapping_add(1);
            if position > apply_random_heuristics {
                if position
                    > apply_random_heuristics
                        .wrapping_add((4 as size_t).wrapping_mul(random_heuristics_window_size))
                {
                    let kMargin: size_t = brotli_max_size_t(
                        StoreLookaheadH55().wrapping_sub(1 as size_t),
                        4 as size_t,
                    ) as size_t;
                    let mut pos_jump: size_t = brotli_min_size_t(
                        position.wrapping_add(16 as size_t),
                        pos_end.wrapping_sub(kMargin),
                    );
                    while position < pos_jump {
                        StoreH55(privat, ringbuffer, ringbuffer_mask, position);
                        insert_length = (insert_length as ::core::ffi::c_ulong)
                            .wrapping_add(4 as ::core::ffi::c_ulong)
                            as size_t as size_t;
                        position = (position as ::core::ffi::c_ulong)
                            .wrapping_add(4 as ::core::ffi::c_ulong)
                            as size_t as size_t;
                    }
                } else {
                    let kMargin_0: size_t = brotli_max_size_t(
                        StoreLookaheadH55().wrapping_sub(1 as size_t),
                        2 as size_t,
                    ) as size_t;
                    let mut pos_jump_0: size_t = brotli_min_size_t(
                        position.wrapping_add(8 as size_t),
                        pos_end.wrapping_sub(kMargin_0),
                    );
                    while position < pos_jump_0 {
                        StoreH55(privat, ringbuffer, ringbuffer_mask, position);
                        insert_length = (insert_length as ::core::ffi::c_ulong)
                            .wrapping_add(2 as ::core::ffi::c_ulong)
                            as size_t as size_t;
                        position = (position as ::core::ffi::c_ulong)
                            .wrapping_add(2 as ::core::ffi::c_ulong)
                            as size_t as size_t;
                    }
                }
            }
        }
    }
    insert_length = (insert_length as ::core::ffi::c_ulong)
        .wrapping_add(pos_end.wrapping_sub(position) as ::core::ffi::c_ulong)
        as size_t as size_t;
    *last_insert_len = insert_length;
    *num_commands =
        (*num_commands as ::core::ffi::c_ulong).wrapping_add(commands.offset_from(orig_commands)
            as ::core::ffi::c_long
            as size_t
            as ::core::ffi::c_ulong) as size_t as size_t;
}
#[inline(never)]
unsafe extern "C" fn CreateBackwardReferencesNH65(
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
    let mut privat: *mut H65 = &raw mut (*hasher).privat._H65;
    let max_backward_limit: size_t = ((1 as ::core::ffi::c_int as size_t) << (*params).lgwin)
        .wrapping_sub(BROTLI_WINDOW_GAP as size_t);
    let position_offset: size_t = (*params).stream_offset;
    let orig_commands: *const Command = commands;
    let mut insert_length: size_t = *last_insert_len;
    let pos_end: size_t = position.wrapping_add(num_bytes);
    let store_end: size_t = if num_bytes >= StoreLookaheadH65() {
        position
            .wrapping_add(num_bytes)
            .wrapping_sub(StoreLookaheadH65() as size_t)
            .wrapping_add(1 as size_t)
    } else {
        position
    };
    let random_heuristics_window_size: size_t = LiteralSpreeLengthForSparseSearch(params) as size_t;
    let mut apply_random_heuristics: size_t = position.wrapping_add(random_heuristics_window_size);
    let gap: size_t = (*params).dictionary.compound.total_size;
    let kMinScore: size_t = BROTLI_SCORE_BASE.wrapping_add(100 as size_t);
    PrepareDistanceCacheH65(privat, dist_cache);
    while position.wrapping_add(HashTypeLengthH65()) < pos_end {
        let mut max_length: size_t = pos_end.wrapping_sub(position);
        let mut max_distance: size_t = brotli_min_size_t(position, max_backward_limit);
        let mut dictionary_start: size_t =
            brotli_min_size_t(position.wrapping_add(position_offset), max_backward_limit);
        let mut sr: HasherSearchResult = HasherSearchResult {
            len: 0,
            distance: 0,
            score: 0,
            len_code_delta: 0,
        };
        let mut dict_id: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut p1: uint8_t = 0 as uint8_t;
        let mut p2: uint8_t = 0 as uint8_t;
        if (*params).dictionary.contextual.context_based != 0 {
            p1 = (if position >= 1 as size_t {
                *ringbuffer.offset((position.wrapping_sub(1 as size_t) & ringbuffer_mask) as isize)
                    as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            }) as uint8_t;
            p2 = (if position >= 2 as size_t {
                *ringbuffer.offset((position.wrapping_sub(2 as size_t) & ringbuffer_mask) as isize)
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
        sr.len = 0 as size_t;
        sr.len_code_delta = 0 as ::core::ffi::c_int;
        sr.distance = 0 as size_t;
        sr.score = kMinScore;
        FindLongestMatchH65(
            privat,
            (*params).dictionary.contextual.dict[dict_id as usize],
            ringbuffer,
            ringbuffer_mask,
            dist_cache,
            position,
            max_length,
            max_distance,
            dictionary_start.wrapping_add(gap),
            (*params).dist.max_distance,
            &raw mut sr,
        );
        if sr.score > kMinScore {
            let mut delayed_backward_references_in_row: ::core::ffi::c_int =
                0 as ::core::ffi::c_int;
            max_length = max_length.wrapping_sub(1);
            loop {
                let cost_diff_lazy: size_t = 175 as size_t;
                let mut sr2: HasherSearchResult = HasherSearchResult {
                    len: 0,
                    distance: 0,
                    score: 0,
                    len_code_delta: 0,
                };
                sr2.len = if (*params).quality < MIN_QUALITY_FOR_EXTENSIVE_REFERENCE_SEARCH {
                    brotli_min_size_t(sr.len.wrapping_sub(1 as size_t), max_length)
                } else {
                    0 as size_t
                };
                sr2.len_code_delta = 0 as ::core::ffi::c_int;
                sr2.distance = 0 as size_t;
                sr2.score = kMinScore;
                max_distance =
                    brotli_min_size_t(position.wrapping_add(1 as size_t), max_backward_limit);
                dictionary_start = brotli_min_size_t(
                    position
                        .wrapping_add(1 as size_t)
                        .wrapping_add(position_offset),
                    max_backward_limit,
                );
                if (*params).dictionary.contextual.context_based != 0 {
                    p2 = p1;
                    p1 = *ringbuffer.offset((position & ringbuffer_mask) as isize);
                    dict_id = (*params).dictionary.contextual.context_map[(*literal_context_lut
                        .offset(p1 as isize)
                        as ::core::ffi::c_int
                        | *literal_context_lut
                            .offset(256 as ::core::ffi::c_int as isize)
                            .offset(p2 as isize) as ::core::ffi::c_int)
                        as usize] as ::core::ffi::c_int;
                }
                FindLongestMatchH65(
                    privat,
                    (*params).dictionary.contextual.dict[dict_id as usize],
                    ringbuffer,
                    ringbuffer_mask,
                    dist_cache,
                    position.wrapping_add(1 as size_t),
                    max_length,
                    max_distance,
                    dictionary_start.wrapping_add(gap),
                    (*params).dist.max_distance,
                    &raw mut sr2,
                );
                if !(sr2.score >= sr.score.wrapping_add(cost_diff_lazy)) {
                    break;
                }
                position = position.wrapping_add(1);
                insert_length = insert_length.wrapping_add(1);
                sr = sr2;
                delayed_backward_references_in_row += 1;
                if !(delayed_backward_references_in_row < 4 as ::core::ffi::c_int
                    && position.wrapping_add(HashTypeLengthH65()) < pos_end)
                {
                    break;
                }
                max_length = max_length.wrapping_sub(1);
            }
            apply_random_heuristics = position
                .wrapping_add((2 as size_t).wrapping_mul(sr.len))
                .wrapping_add(random_heuristics_window_size);
            dictionary_start =
                brotli_min_size_t(position.wrapping_add(position_offset), max_backward_limit);
            let mut distance_code: size_t =
                ComputeDistanceCode(sr.distance, dictionary_start.wrapping_add(gap), dist_cache);
            if sr.distance <= dictionary_start.wrapping_add(gap) && distance_code > 0 as size_t {
                *dist_cache.offset(3 as ::core::ffi::c_int as isize) =
                    *dist_cache.offset(2 as ::core::ffi::c_int as isize);
                *dist_cache.offset(2 as ::core::ffi::c_int as isize) =
                    *dist_cache.offset(1 as ::core::ffi::c_int as isize);
                *dist_cache.offset(1 as ::core::ffi::c_int as isize) =
                    *dist_cache.offset(0 as ::core::ffi::c_int as isize);
                *dist_cache.offset(0 as ::core::ffi::c_int as isize) =
                    sr.distance as ::core::ffi::c_int;
                PrepareDistanceCacheH65(privat, dist_cache);
            }
            let fresh14 = commands;
            commands = commands.offset(1);
            InitCommand(
                fresh14,
                &raw const (*params).dist,
                insert_length,
                sr.len,
                sr.len_code_delta,
                distance_code,
            );
            *num_literals = (*num_literals as ::core::ffi::c_ulong)
                .wrapping_add(insert_length as ::core::ffi::c_ulong)
                as size_t as size_t;
            insert_length = 0 as size_t;
            let mut range_start: size_t = position.wrapping_add(2 as size_t);
            let mut range_end: size_t = brotli_min_size_t(position.wrapping_add(sr.len), store_end);
            if sr.distance < sr.len >> 2 as ::core::ffi::c_int {
                range_start = brotli_min_size_t(
                    range_end,
                    brotli_max_size_t(
                        range_start,
                        position
                            .wrapping_add(sr.len)
                            .wrapping_sub(sr.distance << 2 as ::core::ffi::c_int),
                    ),
                );
            }
            StoreRangeH65(privat, ringbuffer, ringbuffer_mask, range_start, range_end);
            position = (position as ::core::ffi::c_ulong)
                .wrapping_add(sr.len as ::core::ffi::c_ulong) as size_t
                as size_t;
        } else {
            insert_length = insert_length.wrapping_add(1);
            position = position.wrapping_add(1);
            if position > apply_random_heuristics {
                if position
                    > apply_random_heuristics
                        .wrapping_add((4 as size_t).wrapping_mul(random_heuristics_window_size))
                {
                    let kMargin: size_t = brotli_max_size_t(
                        StoreLookaheadH65().wrapping_sub(1 as size_t),
                        4 as size_t,
                    ) as size_t;
                    let mut pos_jump: size_t = brotli_min_size_t(
                        position.wrapping_add(16 as size_t),
                        pos_end.wrapping_sub(kMargin),
                    );
                    while position < pos_jump {
                        StoreH65(privat, ringbuffer, ringbuffer_mask, position);
                        insert_length = (insert_length as ::core::ffi::c_ulong)
                            .wrapping_add(4 as ::core::ffi::c_ulong)
                            as size_t as size_t;
                        position = (position as ::core::ffi::c_ulong)
                            .wrapping_add(4 as ::core::ffi::c_ulong)
                            as size_t as size_t;
                    }
                } else {
                    let kMargin_0: size_t = brotli_max_size_t(
                        StoreLookaheadH65().wrapping_sub(1 as size_t),
                        2 as size_t,
                    ) as size_t;
                    let mut pos_jump_0: size_t = brotli_min_size_t(
                        position.wrapping_add(8 as size_t),
                        pos_end.wrapping_sub(kMargin_0),
                    );
                    while position < pos_jump_0 {
                        StoreH65(privat, ringbuffer, ringbuffer_mask, position);
                        insert_length = (insert_length as ::core::ffi::c_ulong)
                            .wrapping_add(2 as ::core::ffi::c_ulong)
                            as size_t as size_t;
                        position = (position as ::core::ffi::c_ulong)
                            .wrapping_add(2 as ::core::ffi::c_ulong)
                            as size_t as size_t;
                    }
                }
            }
        }
    }
    insert_length = (insert_length as ::core::ffi::c_ulong)
        .wrapping_add(pos_end.wrapping_sub(position) as ::core::ffi::c_ulong)
        as size_t as size_t;
    *last_insert_len = insert_length;
    *num_commands =
        (*num_commands as ::core::ffi::c_ulong).wrapping_add(commands.offset_from(orig_commands)
            as ::core::ffi::c_long
            as size_t
            as ::core::ffi::c_ulong) as size_t as size_t;
}
#[inline(never)]
unsafe extern "C" fn CreateBackwardReferencesNH58(
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
    let mut privat: *mut H58 = &raw mut (*hasher).privat._H58;
    let max_backward_limit: size_t = ((1 as ::core::ffi::c_int as size_t) << (*params).lgwin)
        .wrapping_sub(BROTLI_WINDOW_GAP as size_t);
    let position_offset: size_t = (*params).stream_offset;
    let orig_commands: *const Command = commands;
    let mut insert_length: size_t = *last_insert_len;
    let pos_end: size_t = position.wrapping_add(num_bytes);
    let store_end: size_t = if num_bytes >= StoreLookaheadH58() {
        position
            .wrapping_add(num_bytes)
            .wrapping_sub(StoreLookaheadH58() as size_t)
            .wrapping_add(1 as size_t)
    } else {
        position
    };
    let random_heuristics_window_size: size_t = LiteralSpreeLengthForSparseSearch(params) as size_t;
    let mut apply_random_heuristics: size_t = position.wrapping_add(random_heuristics_window_size);
    let gap: size_t = (*params).dictionary.compound.total_size;
    let kMinScore: size_t = BROTLI_SCORE_BASE.wrapping_add(100 as size_t);
    PrepareDistanceCacheH58(privat, dist_cache);
    while position.wrapping_add(HashTypeLengthH58()) < pos_end {
        let mut max_length: size_t = pos_end.wrapping_sub(position);
        let mut max_distance: size_t = brotli_min_size_t(position, max_backward_limit);
        let mut dictionary_start: size_t =
            brotli_min_size_t(position.wrapping_add(position_offset), max_backward_limit);
        let mut sr: HasherSearchResult = HasherSearchResult {
            len: 0,
            distance: 0,
            score: 0,
            len_code_delta: 0,
        };
        let mut dict_id: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut p1: uint8_t = 0 as uint8_t;
        let mut p2: uint8_t = 0 as uint8_t;
        if (*params).dictionary.contextual.context_based != 0 {
            p1 = (if position >= 1 as size_t {
                *ringbuffer.offset((position.wrapping_sub(1 as size_t) & ringbuffer_mask) as isize)
                    as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            }) as uint8_t;
            p2 = (if position >= 2 as size_t {
                *ringbuffer.offset((position.wrapping_sub(2 as size_t) & ringbuffer_mask) as isize)
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
        sr.len = 0 as size_t;
        sr.len_code_delta = 0 as ::core::ffi::c_int;
        sr.distance = 0 as size_t;
        sr.score = kMinScore;
        FindLongestMatchH58(
            privat,
            (*params).dictionary.contextual.dict[dict_id as usize],
            ringbuffer,
            ringbuffer_mask,
            dist_cache,
            position,
            max_length,
            max_distance,
            dictionary_start.wrapping_add(gap),
            (*params).dist.max_distance,
            &raw mut sr,
        );
        if sr.score > kMinScore {
            let mut delayed_backward_references_in_row: ::core::ffi::c_int =
                0 as ::core::ffi::c_int;
            max_length = max_length.wrapping_sub(1);
            loop {
                let cost_diff_lazy: size_t = 175 as size_t;
                let mut sr2: HasherSearchResult = HasherSearchResult {
                    len: 0,
                    distance: 0,
                    score: 0,
                    len_code_delta: 0,
                };
                sr2.len = if (*params).quality < MIN_QUALITY_FOR_EXTENSIVE_REFERENCE_SEARCH {
                    brotli_min_size_t(sr.len.wrapping_sub(1 as size_t), max_length)
                } else {
                    0 as size_t
                };
                sr2.len_code_delta = 0 as ::core::ffi::c_int;
                sr2.distance = 0 as size_t;
                sr2.score = kMinScore;
                max_distance =
                    brotli_min_size_t(position.wrapping_add(1 as size_t), max_backward_limit);
                dictionary_start = brotli_min_size_t(
                    position
                        .wrapping_add(1 as size_t)
                        .wrapping_add(position_offset),
                    max_backward_limit,
                );
                if (*params).dictionary.contextual.context_based != 0 {
                    p2 = p1;
                    p1 = *ringbuffer.offset((position & ringbuffer_mask) as isize);
                    dict_id = (*params).dictionary.contextual.context_map[(*literal_context_lut
                        .offset(p1 as isize)
                        as ::core::ffi::c_int
                        | *literal_context_lut
                            .offset(256 as ::core::ffi::c_int as isize)
                            .offset(p2 as isize) as ::core::ffi::c_int)
                        as usize] as ::core::ffi::c_int;
                }
                FindLongestMatchH58(
                    privat,
                    (*params).dictionary.contextual.dict[dict_id as usize],
                    ringbuffer,
                    ringbuffer_mask,
                    dist_cache,
                    position.wrapping_add(1 as size_t),
                    max_length,
                    max_distance,
                    dictionary_start.wrapping_add(gap),
                    (*params).dist.max_distance,
                    &raw mut sr2,
                );
                if !(sr2.score >= sr.score.wrapping_add(cost_diff_lazy)) {
                    break;
                }
                position = position.wrapping_add(1);
                insert_length = insert_length.wrapping_add(1);
                sr = sr2;
                delayed_backward_references_in_row += 1;
                if !(delayed_backward_references_in_row < 4 as ::core::ffi::c_int
                    && position.wrapping_add(HashTypeLengthH58()) < pos_end)
                {
                    break;
                }
                max_length = max_length.wrapping_sub(1);
            }
            apply_random_heuristics = position
                .wrapping_add((2 as size_t).wrapping_mul(sr.len))
                .wrapping_add(random_heuristics_window_size);
            dictionary_start =
                brotli_min_size_t(position.wrapping_add(position_offset), max_backward_limit);
            let mut distance_code: size_t =
                ComputeDistanceCode(sr.distance, dictionary_start.wrapping_add(gap), dist_cache);
            if sr.distance <= dictionary_start.wrapping_add(gap) && distance_code > 0 as size_t {
                *dist_cache.offset(3 as ::core::ffi::c_int as isize) =
                    *dist_cache.offset(2 as ::core::ffi::c_int as isize);
                *dist_cache.offset(2 as ::core::ffi::c_int as isize) =
                    *dist_cache.offset(1 as ::core::ffi::c_int as isize);
                *dist_cache.offset(1 as ::core::ffi::c_int as isize) =
                    *dist_cache.offset(0 as ::core::ffi::c_int as isize);
                *dist_cache.offset(0 as ::core::ffi::c_int as isize) =
                    sr.distance as ::core::ffi::c_int;
                PrepareDistanceCacheH58(privat, dist_cache);
            }
            let fresh18 = commands;
            commands = commands.offset(1);
            InitCommand(
                fresh18,
                &raw const (*params).dist,
                insert_length,
                sr.len,
                sr.len_code_delta,
                distance_code,
            );
            *num_literals = (*num_literals as ::core::ffi::c_ulong)
                .wrapping_add(insert_length as ::core::ffi::c_ulong)
                as size_t as size_t;
            insert_length = 0 as size_t;
            let mut range_start: size_t = position.wrapping_add(2 as size_t);
            let mut range_end: size_t = brotli_min_size_t(position.wrapping_add(sr.len), store_end);
            if sr.distance < sr.len >> 2 as ::core::ffi::c_int {
                range_start = brotli_min_size_t(
                    range_end,
                    brotli_max_size_t(
                        range_start,
                        position
                            .wrapping_add(sr.len)
                            .wrapping_sub(sr.distance << 2 as ::core::ffi::c_int),
                    ),
                );
            }
            StoreRangeH58(privat, ringbuffer, ringbuffer_mask, range_start, range_end);
            position = (position as ::core::ffi::c_ulong)
                .wrapping_add(sr.len as ::core::ffi::c_ulong) as size_t
                as size_t;
        } else {
            insert_length = insert_length.wrapping_add(1);
            position = position.wrapping_add(1);
            if position > apply_random_heuristics {
                if position
                    > apply_random_heuristics
                        .wrapping_add((4 as size_t).wrapping_mul(random_heuristics_window_size))
                {
                    let kMargin: size_t = brotli_max_size_t(
                        StoreLookaheadH58().wrapping_sub(1 as size_t),
                        4 as size_t,
                    ) as size_t;
                    let mut pos_jump: size_t = brotli_min_size_t(
                        position.wrapping_add(16 as size_t),
                        pos_end.wrapping_sub(kMargin),
                    );
                    while position < pos_jump {
                        StoreH58(privat, ringbuffer, ringbuffer_mask, position);
                        insert_length = (insert_length as ::core::ffi::c_ulong)
                            .wrapping_add(4 as ::core::ffi::c_ulong)
                            as size_t as size_t;
                        position = (position as ::core::ffi::c_ulong)
                            .wrapping_add(4 as ::core::ffi::c_ulong)
                            as size_t as size_t;
                    }
                } else {
                    let kMargin_0: size_t = brotli_max_size_t(
                        StoreLookaheadH58().wrapping_sub(1 as size_t),
                        2 as size_t,
                    ) as size_t;
                    let mut pos_jump_0: size_t = brotli_min_size_t(
                        position.wrapping_add(8 as size_t),
                        pos_end.wrapping_sub(kMargin_0),
                    );
                    while position < pos_jump_0 {
                        StoreH58(privat, ringbuffer, ringbuffer_mask, position);
                        insert_length = (insert_length as ::core::ffi::c_ulong)
                            .wrapping_add(2 as ::core::ffi::c_ulong)
                            as size_t as size_t;
                        position = (position as ::core::ffi::c_ulong)
                            .wrapping_add(2 as ::core::ffi::c_ulong)
                            as size_t as size_t;
                    }
                }
            }
        }
    }
    insert_length = (insert_length as ::core::ffi::c_ulong)
        .wrapping_add(pos_end.wrapping_sub(position) as ::core::ffi::c_ulong)
        as size_t as size_t;
    *last_insert_len = insert_length;
    *num_commands =
        (*num_commands as ::core::ffi::c_ulong).wrapping_add(commands.offset_from(orig_commands)
            as ::core::ffi::c_long
            as size_t
            as ::core::ffi::c_ulong) as size_t as size_t;
}
#[inline(never)]
unsafe extern "C" fn CreateBackwardReferencesNH68(
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
    let mut privat: *mut H68 = &raw mut (*hasher).privat._H68;
    let max_backward_limit: size_t = ((1 as ::core::ffi::c_int as size_t) << (*params).lgwin)
        .wrapping_sub(BROTLI_WINDOW_GAP as size_t);
    let position_offset: size_t = (*params).stream_offset;
    let orig_commands: *const Command = commands;
    let mut insert_length: size_t = *last_insert_len;
    let pos_end: size_t = position.wrapping_add(num_bytes);
    let store_end: size_t = if num_bytes >= StoreLookaheadH68() {
        position
            .wrapping_add(num_bytes)
            .wrapping_sub(StoreLookaheadH68() as size_t)
            .wrapping_add(1 as size_t)
    } else {
        position
    };
    let random_heuristics_window_size: size_t = LiteralSpreeLengthForSparseSearch(params) as size_t;
    let mut apply_random_heuristics: size_t = position.wrapping_add(random_heuristics_window_size);
    let gap: size_t = (*params).dictionary.compound.total_size;
    let kMinScore: size_t = BROTLI_SCORE_BASE.wrapping_add(100 as size_t);
    PrepareDistanceCacheH68(privat, dist_cache);
    while position.wrapping_add(HashTypeLengthH68()) < pos_end {
        let mut max_length: size_t = pos_end.wrapping_sub(position);
        let mut max_distance: size_t = brotli_min_size_t(position, max_backward_limit);
        let mut dictionary_start: size_t =
            brotli_min_size_t(position.wrapping_add(position_offset), max_backward_limit);
        let mut sr: HasherSearchResult = HasherSearchResult {
            len: 0,
            distance: 0,
            score: 0,
            len_code_delta: 0,
        };
        let mut dict_id: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut p1: uint8_t = 0 as uint8_t;
        let mut p2: uint8_t = 0 as uint8_t;
        if (*params).dictionary.contextual.context_based != 0 {
            p1 = (if position >= 1 as size_t {
                *ringbuffer.offset((position.wrapping_sub(1 as size_t) & ringbuffer_mask) as isize)
                    as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            }) as uint8_t;
            p2 = (if position >= 2 as size_t {
                *ringbuffer.offset((position.wrapping_sub(2 as size_t) & ringbuffer_mask) as isize)
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
        sr.len = 0 as size_t;
        sr.len_code_delta = 0 as ::core::ffi::c_int;
        sr.distance = 0 as size_t;
        sr.score = kMinScore;
        FindLongestMatchH68(
            privat,
            (*params).dictionary.contextual.dict[dict_id as usize],
            ringbuffer,
            ringbuffer_mask,
            dist_cache,
            position,
            max_length,
            max_distance,
            dictionary_start.wrapping_add(gap),
            (*params).dist.max_distance,
            &raw mut sr,
        );
        if sr.score > kMinScore {
            let mut delayed_backward_references_in_row: ::core::ffi::c_int =
                0 as ::core::ffi::c_int;
            max_length = max_length.wrapping_sub(1);
            loop {
                let cost_diff_lazy: size_t = 175 as size_t;
                let mut sr2: HasherSearchResult = HasherSearchResult {
                    len: 0,
                    distance: 0,
                    score: 0,
                    len_code_delta: 0,
                };
                sr2.len = if (*params).quality < MIN_QUALITY_FOR_EXTENSIVE_REFERENCE_SEARCH {
                    brotli_min_size_t(sr.len.wrapping_sub(1 as size_t), max_length)
                } else {
                    0 as size_t
                };
                sr2.len_code_delta = 0 as ::core::ffi::c_int;
                sr2.distance = 0 as size_t;
                sr2.score = kMinScore;
                max_distance =
                    brotli_min_size_t(position.wrapping_add(1 as size_t), max_backward_limit);
                dictionary_start = brotli_min_size_t(
                    position
                        .wrapping_add(1 as size_t)
                        .wrapping_add(position_offset),
                    max_backward_limit,
                );
                if (*params).dictionary.contextual.context_based != 0 {
                    p2 = p1;
                    p1 = *ringbuffer.offset((position & ringbuffer_mask) as isize);
                    dict_id = (*params).dictionary.contextual.context_map[(*literal_context_lut
                        .offset(p1 as isize)
                        as ::core::ffi::c_int
                        | *literal_context_lut
                            .offset(256 as ::core::ffi::c_int as isize)
                            .offset(p2 as isize) as ::core::ffi::c_int)
                        as usize] as ::core::ffi::c_int;
                }
                FindLongestMatchH68(
                    privat,
                    (*params).dictionary.contextual.dict[dict_id as usize],
                    ringbuffer,
                    ringbuffer_mask,
                    dist_cache,
                    position.wrapping_add(1 as size_t),
                    max_length,
                    max_distance,
                    dictionary_start.wrapping_add(gap),
                    (*params).dist.max_distance,
                    &raw mut sr2,
                );
                if !(sr2.score >= sr.score.wrapping_add(cost_diff_lazy)) {
                    break;
                }
                position = position.wrapping_add(1);
                insert_length = insert_length.wrapping_add(1);
                sr = sr2;
                delayed_backward_references_in_row += 1;
                if !(delayed_backward_references_in_row < 4 as ::core::ffi::c_int
                    && position.wrapping_add(HashTypeLengthH68()) < pos_end)
                {
                    break;
                }
                max_length = max_length.wrapping_sub(1);
            }
            apply_random_heuristics = position
                .wrapping_add((2 as size_t).wrapping_mul(sr.len))
                .wrapping_add(random_heuristics_window_size);
            dictionary_start =
                brotli_min_size_t(position.wrapping_add(position_offset), max_backward_limit);
            let mut distance_code: size_t =
                ComputeDistanceCode(sr.distance, dictionary_start.wrapping_add(gap), dist_cache);
            if sr.distance <= dictionary_start.wrapping_add(gap) && distance_code > 0 as size_t {
                *dist_cache.offset(3 as ::core::ffi::c_int as isize) =
                    *dist_cache.offset(2 as ::core::ffi::c_int as isize);
                *dist_cache.offset(2 as ::core::ffi::c_int as isize) =
                    *dist_cache.offset(1 as ::core::ffi::c_int as isize);
                *dist_cache.offset(1 as ::core::ffi::c_int as isize) =
                    *dist_cache.offset(0 as ::core::ffi::c_int as isize);
                *dist_cache.offset(0 as ::core::ffi::c_int as isize) =
                    sr.distance as ::core::ffi::c_int;
                PrepareDistanceCacheH68(privat, dist_cache);
            }
            let fresh17 = commands;
            commands = commands.offset(1);
            InitCommand(
                fresh17,
                &raw const (*params).dist,
                insert_length,
                sr.len,
                sr.len_code_delta,
                distance_code,
            );
            *num_literals = (*num_literals as ::core::ffi::c_ulong)
                .wrapping_add(insert_length as ::core::ffi::c_ulong)
                as size_t as size_t;
            insert_length = 0 as size_t;
            let mut range_start: size_t = position.wrapping_add(2 as size_t);
            let mut range_end: size_t = brotli_min_size_t(position.wrapping_add(sr.len), store_end);
            if sr.distance < sr.len >> 2 as ::core::ffi::c_int {
                range_start = brotli_min_size_t(
                    range_end,
                    brotli_max_size_t(
                        range_start,
                        position
                            .wrapping_add(sr.len)
                            .wrapping_sub(sr.distance << 2 as ::core::ffi::c_int),
                    ),
                );
            }
            StoreRangeH68(privat, ringbuffer, ringbuffer_mask, range_start, range_end);
            position = (position as ::core::ffi::c_ulong)
                .wrapping_add(sr.len as ::core::ffi::c_ulong) as size_t
                as size_t;
        } else {
            insert_length = insert_length.wrapping_add(1);
            position = position.wrapping_add(1);
            if position > apply_random_heuristics {
                if position
                    > apply_random_heuristics
                        .wrapping_add((4 as size_t).wrapping_mul(random_heuristics_window_size))
                {
                    let kMargin: size_t = brotli_max_size_t(
                        StoreLookaheadH68().wrapping_sub(1 as size_t),
                        4 as size_t,
                    ) as size_t;
                    let mut pos_jump: size_t = brotli_min_size_t(
                        position.wrapping_add(16 as size_t),
                        pos_end.wrapping_sub(kMargin),
                    );
                    while position < pos_jump {
                        StoreH68(privat, ringbuffer, ringbuffer_mask, position);
                        insert_length = (insert_length as ::core::ffi::c_ulong)
                            .wrapping_add(4 as ::core::ffi::c_ulong)
                            as size_t as size_t;
                        position = (position as ::core::ffi::c_ulong)
                            .wrapping_add(4 as ::core::ffi::c_ulong)
                            as size_t as size_t;
                    }
                } else {
                    let kMargin_0: size_t = brotli_max_size_t(
                        StoreLookaheadH68().wrapping_sub(1 as size_t),
                        2 as size_t,
                    ) as size_t;
                    let mut pos_jump_0: size_t = brotli_min_size_t(
                        position.wrapping_add(8 as size_t),
                        pos_end.wrapping_sub(kMargin_0),
                    );
                    while position < pos_jump_0 {
                        StoreH68(privat, ringbuffer, ringbuffer_mask, position);
                        insert_length = (insert_length as ::core::ffi::c_ulong)
                            .wrapping_add(2 as ::core::ffi::c_ulong)
                            as size_t as size_t;
                        position = (position as ::core::ffi::c_ulong)
                            .wrapping_add(2 as ::core::ffi::c_ulong)
                            as size_t as size_t;
                    }
                }
            }
        }
    }
    insert_length = (insert_length as ::core::ffi::c_ulong)
        .wrapping_add(pos_end.wrapping_sub(position) as ::core::ffi::c_ulong)
        as size_t as size_t;
    *last_insert_len = insert_length;
    *num_commands =
        (*num_commands as ::core::ffi::c_ulong).wrapping_add(commands.offset_from(orig_commands)
            as ::core::ffi::c_long
            as size_t
            as ::core::ffi::c_ulong) as size_t as size_t;
}
#[inline(never)]
unsafe extern "C" fn CreateBackwardReferencesDH3(
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
    let mut privat: *mut H3 = &raw mut (*hasher).privat._H3;
    let max_backward_limit: size_t = ((1 as ::core::ffi::c_int as size_t) << (*params).lgwin)
        .wrapping_sub(BROTLI_WINDOW_GAP as size_t);
    let position_offset: size_t = (*params).stream_offset;
    let orig_commands: *const Command = commands;
    let mut insert_length: size_t = *last_insert_len;
    let pos_end: size_t = position.wrapping_add(num_bytes);
    let store_end: size_t = if num_bytes >= StoreLookaheadH3() {
        position
            .wrapping_add(num_bytes)
            .wrapping_sub(StoreLookaheadH3() as size_t)
            .wrapping_add(1 as size_t)
    } else {
        position
    };
    let random_heuristics_window_size: size_t = LiteralSpreeLengthForSparseSearch(params) as size_t;
    let mut apply_random_heuristics: size_t = position.wrapping_add(random_heuristics_window_size);
    let gap: size_t = (*params).dictionary.compound.total_size;
    let kMinScore: size_t = BROTLI_SCORE_BASE.wrapping_add(100 as size_t);
    PrepareDistanceCacheH3(privat, dist_cache);
    while position.wrapping_add(HashTypeLengthH3()) < pos_end {
        let mut max_length: size_t = pos_end.wrapping_sub(position);
        let mut max_distance: size_t = brotli_min_size_t(position, max_backward_limit);
        let mut dictionary_start: size_t =
            brotli_min_size_t(position.wrapping_add(position_offset), max_backward_limit);
        let mut sr: HasherSearchResult = HasherSearchResult {
            len: 0,
            distance: 0,
            score: 0,
            len_code_delta: 0,
        };
        let mut dict_id: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut p1: uint8_t = 0 as uint8_t;
        let mut p2: uint8_t = 0 as uint8_t;
        if (*params).dictionary.contextual.context_based != 0 {
            p1 = (if position >= 1 as size_t {
                *ringbuffer.offset((position.wrapping_sub(1 as size_t) & ringbuffer_mask) as isize)
                    as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            }) as uint8_t;
            p2 = (if position >= 2 as size_t {
                *ringbuffer.offset((position.wrapping_sub(2 as size_t) & ringbuffer_mask) as isize)
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
        sr.len = 0 as size_t;
        sr.len_code_delta = 0 as ::core::ffi::c_int;
        sr.distance = 0 as size_t;
        sr.score = kMinScore;
        FindLongestMatchH3(
            privat,
            (*params).dictionary.contextual.dict[dict_id as usize],
            ringbuffer,
            ringbuffer_mask,
            dist_cache,
            position,
            max_length,
            max_distance,
            dictionary_start.wrapping_add(gap),
            (*params).dist.max_distance,
            &raw mut sr,
        );
        LookupCompoundDictionaryMatch(
            &raw const (*params).dictionary.compound,
            ringbuffer,
            ringbuffer_mask,
            dist_cache,
            position,
            max_length,
            dictionary_start,
            (*params).dist.max_distance,
            &raw mut sr,
        );
        if sr.score > kMinScore {
            let mut delayed_backward_references_in_row: ::core::ffi::c_int =
                0 as ::core::ffi::c_int;
            max_length = max_length.wrapping_sub(1);
            loop {
                let cost_diff_lazy: size_t = 175 as size_t;
                let mut sr2: HasherSearchResult = HasherSearchResult {
                    len: 0,
                    distance: 0,
                    score: 0,
                    len_code_delta: 0,
                };
                sr2.len = if (*params).quality < MIN_QUALITY_FOR_EXTENSIVE_REFERENCE_SEARCH {
                    brotli_min_size_t(sr.len.wrapping_sub(1 as size_t), max_length)
                } else {
                    0 as size_t
                };
                sr2.len_code_delta = 0 as ::core::ffi::c_int;
                sr2.distance = 0 as size_t;
                sr2.score = kMinScore;
                max_distance =
                    brotli_min_size_t(position.wrapping_add(1 as size_t), max_backward_limit);
                dictionary_start = brotli_min_size_t(
                    position
                        .wrapping_add(1 as size_t)
                        .wrapping_add(position_offset),
                    max_backward_limit,
                );
                if (*params).dictionary.contextual.context_based != 0 {
                    p2 = p1;
                    p1 = *ringbuffer.offset((position & ringbuffer_mask) as isize);
                    dict_id = (*params).dictionary.contextual.context_map[(*literal_context_lut
                        .offset(p1 as isize)
                        as ::core::ffi::c_int
                        | *literal_context_lut
                            .offset(256 as ::core::ffi::c_int as isize)
                            .offset(p2 as isize) as ::core::ffi::c_int)
                        as usize] as ::core::ffi::c_int;
                }
                FindLongestMatchH3(
                    privat,
                    (*params).dictionary.contextual.dict[dict_id as usize],
                    ringbuffer,
                    ringbuffer_mask,
                    dist_cache,
                    position.wrapping_add(1 as size_t),
                    max_length,
                    max_distance,
                    dictionary_start.wrapping_add(gap),
                    (*params).dist.max_distance,
                    &raw mut sr2,
                );
                LookupCompoundDictionaryMatch(
                    &raw const (*params).dictionary.compound,
                    ringbuffer,
                    ringbuffer_mask,
                    dist_cache,
                    position.wrapping_add(1 as size_t),
                    max_length,
                    dictionary_start,
                    (*params).dist.max_distance,
                    &raw mut sr2,
                );
                if !(sr2.score >= sr.score.wrapping_add(cost_diff_lazy)) {
                    break;
                }
                position = position.wrapping_add(1);
                insert_length = insert_length.wrapping_add(1);
                sr = sr2;
                delayed_backward_references_in_row += 1;
                if !(delayed_backward_references_in_row < 4 as ::core::ffi::c_int
                    && position.wrapping_add(HashTypeLengthH3()) < pos_end)
                {
                    break;
                }
                max_length = max_length.wrapping_sub(1);
            }
            apply_random_heuristics = position
                .wrapping_add((2 as size_t).wrapping_mul(sr.len))
                .wrapping_add(random_heuristics_window_size);
            dictionary_start =
                brotli_min_size_t(position.wrapping_add(position_offset), max_backward_limit);
            let mut distance_code: size_t =
                ComputeDistanceCode(sr.distance, dictionary_start.wrapping_add(gap), dist_cache);
            if sr.distance <= dictionary_start.wrapping_add(gap) && distance_code > 0 as size_t {
                *dist_cache.offset(3 as ::core::ffi::c_int as isize) =
                    *dist_cache.offset(2 as ::core::ffi::c_int as isize);
                *dist_cache.offset(2 as ::core::ffi::c_int as isize) =
                    *dist_cache.offset(1 as ::core::ffi::c_int as isize);
                *dist_cache.offset(1 as ::core::ffi::c_int as isize) =
                    *dist_cache.offset(0 as ::core::ffi::c_int as isize);
                *dist_cache.offset(0 as ::core::ffi::c_int as isize) =
                    sr.distance as ::core::ffi::c_int;
                PrepareDistanceCacheH3(privat, dist_cache);
            }
            let fresh38 = commands;
            commands = commands.offset(1);
            InitCommand(
                fresh38,
                &raw const (*params).dist,
                insert_length,
                sr.len,
                sr.len_code_delta,
                distance_code,
            );
            *num_literals = (*num_literals as ::core::ffi::c_ulong)
                .wrapping_add(insert_length as ::core::ffi::c_ulong)
                as size_t as size_t;
            insert_length = 0 as size_t;
            let mut range_start: size_t = position.wrapping_add(2 as size_t);
            let mut range_end: size_t = brotli_min_size_t(position.wrapping_add(sr.len), store_end);
            if sr.distance < sr.len >> 2 as ::core::ffi::c_int {
                range_start = brotli_min_size_t(
                    range_end,
                    brotli_max_size_t(
                        range_start,
                        position
                            .wrapping_add(sr.len)
                            .wrapping_sub(sr.distance << 2 as ::core::ffi::c_int),
                    ),
                );
            }
            StoreRangeH3(privat, ringbuffer, ringbuffer_mask, range_start, range_end);
            position = (position as ::core::ffi::c_ulong)
                .wrapping_add(sr.len as ::core::ffi::c_ulong) as size_t
                as size_t;
        } else {
            insert_length = insert_length.wrapping_add(1);
            position = position.wrapping_add(1);
            if position > apply_random_heuristics {
                if position
                    > apply_random_heuristics
                        .wrapping_add((4 as size_t).wrapping_mul(random_heuristics_window_size))
                {
                    let kMargin: size_t = brotli_max_size_t(
                        StoreLookaheadH3().wrapping_sub(1 as size_t),
                        4 as size_t,
                    ) as size_t;
                    let mut pos_jump: size_t = brotli_min_size_t(
                        position.wrapping_add(16 as size_t),
                        pos_end.wrapping_sub(kMargin),
                    );
                    while position < pos_jump {
                        StoreH3(privat, ringbuffer, ringbuffer_mask, position);
                        insert_length = (insert_length as ::core::ffi::c_ulong)
                            .wrapping_add(4 as ::core::ffi::c_ulong)
                            as size_t as size_t;
                        position = (position as ::core::ffi::c_ulong)
                            .wrapping_add(4 as ::core::ffi::c_ulong)
                            as size_t as size_t;
                    }
                } else {
                    let kMargin_0: size_t = brotli_max_size_t(
                        StoreLookaheadH3().wrapping_sub(1 as size_t),
                        2 as size_t,
                    ) as size_t;
                    let mut pos_jump_0: size_t = brotli_min_size_t(
                        position.wrapping_add(8 as size_t),
                        pos_end.wrapping_sub(kMargin_0),
                    );
                    while position < pos_jump_0 {
                        StoreH3(privat, ringbuffer, ringbuffer_mask, position);
                        insert_length = (insert_length as ::core::ffi::c_ulong)
                            .wrapping_add(2 as ::core::ffi::c_ulong)
                            as size_t as size_t;
                        position = (position as ::core::ffi::c_ulong)
                            .wrapping_add(2 as ::core::ffi::c_ulong)
                            as size_t as size_t;
                    }
                }
            }
        }
    }
    insert_length = (insert_length as ::core::ffi::c_ulong)
        .wrapping_add(pos_end.wrapping_sub(position) as ::core::ffi::c_ulong)
        as size_t as size_t;
    *last_insert_len = insert_length;
    *num_commands =
        (*num_commands as ::core::ffi::c_ulong).wrapping_add(commands.offset_from(orig_commands)
            as ::core::ffi::c_long
            as size_t
            as ::core::ffi::c_ulong) as size_t as size_t;
}
#[inline(never)]
unsafe extern "C" fn CreateBackwardReferencesDH4(
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
    let mut privat: *mut H4 = &raw mut (*hasher).privat._H4;
    let max_backward_limit: size_t = ((1 as ::core::ffi::c_int as size_t) << (*params).lgwin)
        .wrapping_sub(BROTLI_WINDOW_GAP as size_t);
    let position_offset: size_t = (*params).stream_offset;
    let orig_commands: *const Command = commands;
    let mut insert_length: size_t = *last_insert_len;
    let pos_end: size_t = position.wrapping_add(num_bytes);
    let store_end: size_t = if num_bytes >= StoreLookaheadH4() {
        position
            .wrapping_add(num_bytes)
            .wrapping_sub(StoreLookaheadH4() as size_t)
            .wrapping_add(1 as size_t)
    } else {
        position
    };
    let random_heuristics_window_size: size_t = LiteralSpreeLengthForSparseSearch(params) as size_t;
    let mut apply_random_heuristics: size_t = position.wrapping_add(random_heuristics_window_size);
    let gap: size_t = (*params).dictionary.compound.total_size;
    let kMinScore: size_t = BROTLI_SCORE_BASE.wrapping_add(100 as size_t);
    PrepareDistanceCacheH4(privat, dist_cache);
    while position.wrapping_add(HashTypeLengthH4()) < pos_end {
        let mut max_length: size_t = pos_end.wrapping_sub(position);
        let mut max_distance: size_t = brotli_min_size_t(position, max_backward_limit);
        let mut dictionary_start: size_t =
            brotli_min_size_t(position.wrapping_add(position_offset), max_backward_limit);
        let mut sr: HasherSearchResult = HasherSearchResult {
            len: 0,
            distance: 0,
            score: 0,
            len_code_delta: 0,
        };
        let mut dict_id: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut p1: uint8_t = 0 as uint8_t;
        let mut p2: uint8_t = 0 as uint8_t;
        if (*params).dictionary.contextual.context_based != 0 {
            p1 = (if position >= 1 as size_t {
                *ringbuffer.offset((position.wrapping_sub(1 as size_t) & ringbuffer_mask) as isize)
                    as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            }) as uint8_t;
            p2 = (if position >= 2 as size_t {
                *ringbuffer.offset((position.wrapping_sub(2 as size_t) & ringbuffer_mask) as isize)
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
        sr.len = 0 as size_t;
        sr.len_code_delta = 0 as ::core::ffi::c_int;
        sr.distance = 0 as size_t;
        sr.score = kMinScore;
        FindLongestMatchH4(
            privat,
            (*params).dictionary.contextual.dict[dict_id as usize],
            ringbuffer,
            ringbuffer_mask,
            dist_cache,
            position,
            max_length,
            max_distance,
            dictionary_start.wrapping_add(gap),
            (*params).dist.max_distance,
            &raw mut sr,
        );
        LookupCompoundDictionaryMatch(
            &raw const (*params).dictionary.compound,
            ringbuffer,
            ringbuffer_mask,
            dist_cache,
            position,
            max_length,
            dictionary_start,
            (*params).dist.max_distance,
            &raw mut sr,
        );
        if sr.score > kMinScore {
            let mut delayed_backward_references_in_row: ::core::ffi::c_int =
                0 as ::core::ffi::c_int;
            max_length = max_length.wrapping_sub(1);
            loop {
                let cost_diff_lazy: size_t = 175 as size_t;
                let mut sr2: HasherSearchResult = HasherSearchResult {
                    len: 0,
                    distance: 0,
                    score: 0,
                    len_code_delta: 0,
                };
                sr2.len = if (*params).quality < MIN_QUALITY_FOR_EXTENSIVE_REFERENCE_SEARCH {
                    brotli_min_size_t(sr.len.wrapping_sub(1 as size_t), max_length)
                } else {
                    0 as size_t
                };
                sr2.len_code_delta = 0 as ::core::ffi::c_int;
                sr2.distance = 0 as size_t;
                sr2.score = kMinScore;
                max_distance =
                    brotli_min_size_t(position.wrapping_add(1 as size_t), max_backward_limit);
                dictionary_start = brotli_min_size_t(
                    position
                        .wrapping_add(1 as size_t)
                        .wrapping_add(position_offset),
                    max_backward_limit,
                );
                if (*params).dictionary.contextual.context_based != 0 {
                    p2 = p1;
                    p1 = *ringbuffer.offset((position & ringbuffer_mask) as isize);
                    dict_id = (*params).dictionary.contextual.context_map[(*literal_context_lut
                        .offset(p1 as isize)
                        as ::core::ffi::c_int
                        | *literal_context_lut
                            .offset(256 as ::core::ffi::c_int as isize)
                            .offset(p2 as isize) as ::core::ffi::c_int)
                        as usize] as ::core::ffi::c_int;
                }
                FindLongestMatchH4(
                    privat,
                    (*params).dictionary.contextual.dict[dict_id as usize],
                    ringbuffer,
                    ringbuffer_mask,
                    dist_cache,
                    position.wrapping_add(1 as size_t),
                    max_length,
                    max_distance,
                    dictionary_start.wrapping_add(gap),
                    (*params).dist.max_distance,
                    &raw mut sr2,
                );
                LookupCompoundDictionaryMatch(
                    &raw const (*params).dictionary.compound,
                    ringbuffer,
                    ringbuffer_mask,
                    dist_cache,
                    position.wrapping_add(1 as size_t),
                    max_length,
                    dictionary_start,
                    (*params).dist.max_distance,
                    &raw mut sr2,
                );
                if !(sr2.score >= sr.score.wrapping_add(cost_diff_lazy)) {
                    break;
                }
                position = position.wrapping_add(1);
                insert_length = insert_length.wrapping_add(1);
                sr = sr2;
                delayed_backward_references_in_row += 1;
                if !(delayed_backward_references_in_row < 4 as ::core::ffi::c_int
                    && position.wrapping_add(HashTypeLengthH4()) < pos_end)
                {
                    break;
                }
                max_length = max_length.wrapping_sub(1);
            }
            apply_random_heuristics = position
                .wrapping_add((2 as size_t).wrapping_mul(sr.len))
                .wrapping_add(random_heuristics_window_size);
            dictionary_start =
                brotli_min_size_t(position.wrapping_add(position_offset), max_backward_limit);
            let mut distance_code: size_t =
                ComputeDistanceCode(sr.distance, dictionary_start.wrapping_add(gap), dist_cache);
            if sr.distance <= dictionary_start.wrapping_add(gap) && distance_code > 0 as size_t {
                *dist_cache.offset(3 as ::core::ffi::c_int as isize) =
                    *dist_cache.offset(2 as ::core::ffi::c_int as isize);
                *dist_cache.offset(2 as ::core::ffi::c_int as isize) =
                    *dist_cache.offset(1 as ::core::ffi::c_int as isize);
                *dist_cache.offset(1 as ::core::ffi::c_int as isize) =
                    *dist_cache.offset(0 as ::core::ffi::c_int as isize);
                *dist_cache.offset(0 as ::core::ffi::c_int as isize) =
                    sr.distance as ::core::ffi::c_int;
                PrepareDistanceCacheH4(privat, dist_cache);
            }
            let fresh37 = commands;
            commands = commands.offset(1);
            InitCommand(
                fresh37,
                &raw const (*params).dist,
                insert_length,
                sr.len,
                sr.len_code_delta,
                distance_code,
            );
            *num_literals = (*num_literals as ::core::ffi::c_ulong)
                .wrapping_add(insert_length as ::core::ffi::c_ulong)
                as size_t as size_t;
            insert_length = 0 as size_t;
            let mut range_start: size_t = position.wrapping_add(2 as size_t);
            let mut range_end: size_t = brotli_min_size_t(position.wrapping_add(sr.len), store_end);
            if sr.distance < sr.len >> 2 as ::core::ffi::c_int {
                range_start = brotli_min_size_t(
                    range_end,
                    brotli_max_size_t(
                        range_start,
                        position
                            .wrapping_add(sr.len)
                            .wrapping_sub(sr.distance << 2 as ::core::ffi::c_int),
                    ),
                );
            }
            StoreRangeH4(privat, ringbuffer, ringbuffer_mask, range_start, range_end);
            position = (position as ::core::ffi::c_ulong)
                .wrapping_add(sr.len as ::core::ffi::c_ulong) as size_t
                as size_t;
        } else {
            insert_length = insert_length.wrapping_add(1);
            position = position.wrapping_add(1);
            if position > apply_random_heuristics {
                if position
                    > apply_random_heuristics
                        .wrapping_add((4 as size_t).wrapping_mul(random_heuristics_window_size))
                {
                    let kMargin: size_t = brotli_max_size_t(
                        StoreLookaheadH4().wrapping_sub(1 as size_t),
                        4 as size_t,
                    ) as size_t;
                    let mut pos_jump: size_t = brotli_min_size_t(
                        position.wrapping_add(16 as size_t),
                        pos_end.wrapping_sub(kMargin),
                    );
                    while position < pos_jump {
                        StoreH4(privat, ringbuffer, ringbuffer_mask, position);
                        insert_length = (insert_length as ::core::ffi::c_ulong)
                            .wrapping_add(4 as ::core::ffi::c_ulong)
                            as size_t as size_t;
                        position = (position as ::core::ffi::c_ulong)
                            .wrapping_add(4 as ::core::ffi::c_ulong)
                            as size_t as size_t;
                    }
                } else {
                    let kMargin_0: size_t = brotli_max_size_t(
                        StoreLookaheadH4().wrapping_sub(1 as size_t),
                        2 as size_t,
                    ) as size_t;
                    let mut pos_jump_0: size_t = brotli_min_size_t(
                        position.wrapping_add(8 as size_t),
                        pos_end.wrapping_sub(kMargin_0),
                    );
                    while position < pos_jump_0 {
                        StoreH4(privat, ringbuffer, ringbuffer_mask, position);
                        insert_length = (insert_length as ::core::ffi::c_ulong)
                            .wrapping_add(2 as ::core::ffi::c_ulong)
                            as size_t as size_t;
                        position = (position as ::core::ffi::c_ulong)
                            .wrapping_add(2 as ::core::ffi::c_ulong)
                            as size_t as size_t;
                    }
                }
            }
        }
    }
    insert_length = (insert_length as ::core::ffi::c_ulong)
        .wrapping_add(pos_end.wrapping_sub(position) as ::core::ffi::c_ulong)
        as size_t as size_t;
    *last_insert_len = insert_length;
    *num_commands =
        (*num_commands as ::core::ffi::c_ulong).wrapping_add(commands.offset_from(orig_commands)
            as ::core::ffi::c_long
            as size_t
            as ::core::ffi::c_ulong) as size_t as size_t;
}
#[inline(never)]
unsafe extern "C" fn CreateBackwardReferencesDH5(
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
    let mut privat: *mut H5 = &raw mut (*hasher).privat._H5;
    let max_backward_limit: size_t = ((1 as ::core::ffi::c_int as size_t) << (*params).lgwin)
        .wrapping_sub(BROTLI_WINDOW_GAP as size_t);
    let position_offset: size_t = (*params).stream_offset;
    let orig_commands: *const Command = commands;
    let mut insert_length: size_t = *last_insert_len;
    let pos_end: size_t = position.wrapping_add(num_bytes);
    let store_end: size_t = if num_bytes >= StoreLookaheadH5() {
        position
            .wrapping_add(num_bytes)
            .wrapping_sub(StoreLookaheadH5() as size_t)
            .wrapping_add(1 as size_t)
    } else {
        position
    };
    let random_heuristics_window_size: size_t = LiteralSpreeLengthForSparseSearch(params) as size_t;
    let mut apply_random_heuristics: size_t = position.wrapping_add(random_heuristics_window_size);
    let gap: size_t = (*params).dictionary.compound.total_size;
    let kMinScore: size_t = BROTLI_SCORE_BASE.wrapping_add(100 as size_t);
    PrepareDistanceCacheH5(privat, dist_cache);
    while position.wrapping_add(HashTypeLengthH5()) < pos_end {
        let mut max_length: size_t = pos_end.wrapping_sub(position);
        let mut max_distance: size_t = brotli_min_size_t(position, max_backward_limit);
        let mut dictionary_start: size_t =
            brotli_min_size_t(position.wrapping_add(position_offset), max_backward_limit);
        let mut sr: HasherSearchResult = HasherSearchResult {
            len: 0,
            distance: 0,
            score: 0,
            len_code_delta: 0,
        };
        let mut dict_id: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut p1: uint8_t = 0 as uint8_t;
        let mut p2: uint8_t = 0 as uint8_t;
        if (*params).dictionary.contextual.context_based != 0 {
            p1 = (if position >= 1 as size_t {
                *ringbuffer.offset((position.wrapping_sub(1 as size_t) & ringbuffer_mask) as isize)
                    as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            }) as uint8_t;
            p2 = (if position >= 2 as size_t {
                *ringbuffer.offset((position.wrapping_sub(2 as size_t) & ringbuffer_mask) as isize)
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
        sr.len = 0 as size_t;
        sr.len_code_delta = 0 as ::core::ffi::c_int;
        sr.distance = 0 as size_t;
        sr.score = kMinScore;
        FindLongestMatchH5(
            privat,
            (*params).dictionary.contextual.dict[dict_id as usize],
            ringbuffer,
            ringbuffer_mask,
            dist_cache,
            position,
            max_length,
            max_distance,
            dictionary_start.wrapping_add(gap),
            (*params).dist.max_distance,
            &raw mut sr,
        );
        LookupCompoundDictionaryMatch(
            &raw const (*params).dictionary.compound,
            ringbuffer,
            ringbuffer_mask,
            dist_cache,
            position,
            max_length,
            dictionary_start,
            (*params).dist.max_distance,
            &raw mut sr,
        );
        if sr.score > kMinScore {
            let mut delayed_backward_references_in_row: ::core::ffi::c_int =
                0 as ::core::ffi::c_int;
            max_length = max_length.wrapping_sub(1);
            loop {
                let cost_diff_lazy: size_t = 175 as size_t;
                let mut sr2: HasherSearchResult = HasherSearchResult {
                    len: 0,
                    distance: 0,
                    score: 0,
                    len_code_delta: 0,
                };
                sr2.len = if (*params).quality < MIN_QUALITY_FOR_EXTENSIVE_REFERENCE_SEARCH {
                    brotli_min_size_t(sr.len.wrapping_sub(1 as size_t), max_length)
                } else {
                    0 as size_t
                };
                sr2.len_code_delta = 0 as ::core::ffi::c_int;
                sr2.distance = 0 as size_t;
                sr2.score = kMinScore;
                max_distance =
                    brotli_min_size_t(position.wrapping_add(1 as size_t), max_backward_limit);
                dictionary_start = brotli_min_size_t(
                    position
                        .wrapping_add(1 as size_t)
                        .wrapping_add(position_offset),
                    max_backward_limit,
                );
                if (*params).dictionary.contextual.context_based != 0 {
                    p2 = p1;
                    p1 = *ringbuffer.offset((position & ringbuffer_mask) as isize);
                    dict_id = (*params).dictionary.contextual.context_map[(*literal_context_lut
                        .offset(p1 as isize)
                        as ::core::ffi::c_int
                        | *literal_context_lut
                            .offset(256 as ::core::ffi::c_int as isize)
                            .offset(p2 as isize) as ::core::ffi::c_int)
                        as usize] as ::core::ffi::c_int;
                }
                FindLongestMatchH5(
                    privat,
                    (*params).dictionary.contextual.dict[dict_id as usize],
                    ringbuffer,
                    ringbuffer_mask,
                    dist_cache,
                    position.wrapping_add(1 as size_t),
                    max_length,
                    max_distance,
                    dictionary_start.wrapping_add(gap),
                    (*params).dist.max_distance,
                    &raw mut sr2,
                );
                LookupCompoundDictionaryMatch(
                    &raw const (*params).dictionary.compound,
                    ringbuffer,
                    ringbuffer_mask,
                    dist_cache,
                    position.wrapping_add(1 as size_t),
                    max_length,
                    dictionary_start,
                    (*params).dist.max_distance,
                    &raw mut sr2,
                );
                if !(sr2.score >= sr.score.wrapping_add(cost_diff_lazy)) {
                    break;
                }
                position = position.wrapping_add(1);
                insert_length = insert_length.wrapping_add(1);
                sr = sr2;
                delayed_backward_references_in_row += 1;
                if !(delayed_backward_references_in_row < 4 as ::core::ffi::c_int
                    && position.wrapping_add(HashTypeLengthH5()) < pos_end)
                {
                    break;
                }
                max_length = max_length.wrapping_sub(1);
            }
            apply_random_heuristics = position
                .wrapping_add((2 as size_t).wrapping_mul(sr.len))
                .wrapping_add(random_heuristics_window_size);
            dictionary_start =
                brotli_min_size_t(position.wrapping_add(position_offset), max_backward_limit);
            let mut distance_code: size_t =
                ComputeDistanceCode(sr.distance, dictionary_start.wrapping_add(gap), dist_cache);
            if sr.distance <= dictionary_start.wrapping_add(gap) && distance_code > 0 as size_t {
                *dist_cache.offset(3 as ::core::ffi::c_int as isize) =
                    *dist_cache.offset(2 as ::core::ffi::c_int as isize);
                *dist_cache.offset(2 as ::core::ffi::c_int as isize) =
                    *dist_cache.offset(1 as ::core::ffi::c_int as isize);
                *dist_cache.offset(1 as ::core::ffi::c_int as isize) =
                    *dist_cache.offset(0 as ::core::ffi::c_int as isize);
                *dist_cache.offset(0 as ::core::ffi::c_int as isize) =
                    sr.distance as ::core::ffi::c_int;
                PrepareDistanceCacheH5(privat, dist_cache);
            }
            let fresh36 = commands;
            commands = commands.offset(1);
            InitCommand(
                fresh36,
                &raw const (*params).dist,
                insert_length,
                sr.len,
                sr.len_code_delta,
                distance_code,
            );
            *num_literals = (*num_literals as ::core::ffi::c_ulong)
                .wrapping_add(insert_length as ::core::ffi::c_ulong)
                as size_t as size_t;
            insert_length = 0 as size_t;
            let mut range_start: size_t = position.wrapping_add(2 as size_t);
            let mut range_end: size_t = brotli_min_size_t(position.wrapping_add(sr.len), store_end);
            if sr.distance < sr.len >> 2 as ::core::ffi::c_int {
                range_start = brotli_min_size_t(
                    range_end,
                    brotli_max_size_t(
                        range_start,
                        position
                            .wrapping_add(sr.len)
                            .wrapping_sub(sr.distance << 2 as ::core::ffi::c_int),
                    ),
                );
            }
            StoreRangeH5(privat, ringbuffer, ringbuffer_mask, range_start, range_end);
            position = (position as ::core::ffi::c_ulong)
                .wrapping_add(sr.len as ::core::ffi::c_ulong) as size_t
                as size_t;
        } else {
            insert_length = insert_length.wrapping_add(1);
            position = position.wrapping_add(1);
            if position > apply_random_heuristics {
                if position
                    > apply_random_heuristics
                        .wrapping_add((4 as size_t).wrapping_mul(random_heuristics_window_size))
                {
                    let kMargin: size_t = brotli_max_size_t(
                        StoreLookaheadH5().wrapping_sub(1 as size_t),
                        4 as size_t,
                    ) as size_t;
                    let mut pos_jump: size_t = brotli_min_size_t(
                        position.wrapping_add(16 as size_t),
                        pos_end.wrapping_sub(kMargin),
                    );
                    while position < pos_jump {
                        StoreH5(privat, ringbuffer, ringbuffer_mask, position);
                        insert_length = (insert_length as ::core::ffi::c_ulong)
                            .wrapping_add(4 as ::core::ffi::c_ulong)
                            as size_t as size_t;
                        position = (position as ::core::ffi::c_ulong)
                            .wrapping_add(4 as ::core::ffi::c_ulong)
                            as size_t as size_t;
                    }
                } else {
                    let kMargin_0: size_t = brotli_max_size_t(
                        StoreLookaheadH5().wrapping_sub(1 as size_t),
                        2 as size_t,
                    ) as size_t;
                    let mut pos_jump_0: size_t = brotli_min_size_t(
                        position.wrapping_add(8 as size_t),
                        pos_end.wrapping_sub(kMargin_0),
                    );
                    while position < pos_jump_0 {
                        StoreH5(privat, ringbuffer, ringbuffer_mask, position);
                        insert_length = (insert_length as ::core::ffi::c_ulong)
                            .wrapping_add(2 as ::core::ffi::c_ulong)
                            as size_t as size_t;
                        position = (position as ::core::ffi::c_ulong)
                            .wrapping_add(2 as ::core::ffi::c_ulong)
                            as size_t as size_t;
                    }
                }
            }
        }
    }
    insert_length = (insert_length as ::core::ffi::c_ulong)
        .wrapping_add(pos_end.wrapping_sub(position) as ::core::ffi::c_ulong)
        as size_t as size_t;
    *last_insert_len = insert_length;
    *num_commands =
        (*num_commands as ::core::ffi::c_ulong).wrapping_add(commands.offset_from(orig_commands)
            as ::core::ffi::c_long
            as size_t
            as ::core::ffi::c_ulong) as size_t as size_t;
}
#[inline(never)]
unsafe extern "C" fn CreateBackwardReferencesDH6(
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
    let mut privat: *mut H6 = &raw mut (*hasher).privat._H6;
    let max_backward_limit: size_t = ((1 as ::core::ffi::c_int as size_t) << (*params).lgwin)
        .wrapping_sub(BROTLI_WINDOW_GAP as size_t);
    let position_offset: size_t = (*params).stream_offset;
    let orig_commands: *const Command = commands;
    let mut insert_length: size_t = *last_insert_len;
    let pos_end: size_t = position.wrapping_add(num_bytes);
    let store_end: size_t = if num_bytes >= StoreLookaheadH6() {
        position
            .wrapping_add(num_bytes)
            .wrapping_sub(StoreLookaheadH6() as size_t)
            .wrapping_add(1 as size_t)
    } else {
        position
    };
    let random_heuristics_window_size: size_t = LiteralSpreeLengthForSparseSearch(params) as size_t;
    let mut apply_random_heuristics: size_t = position.wrapping_add(random_heuristics_window_size);
    let gap: size_t = (*params).dictionary.compound.total_size;
    let kMinScore: size_t = BROTLI_SCORE_BASE.wrapping_add(100 as size_t);
    PrepareDistanceCacheH6(privat, dist_cache);
    while position.wrapping_add(HashTypeLengthH6()) < pos_end {
        let mut max_length: size_t = pos_end.wrapping_sub(position);
        let mut max_distance: size_t = brotli_min_size_t(position, max_backward_limit);
        let mut dictionary_start: size_t =
            brotli_min_size_t(position.wrapping_add(position_offset), max_backward_limit);
        let mut sr: HasherSearchResult = HasherSearchResult {
            len: 0,
            distance: 0,
            score: 0,
            len_code_delta: 0,
        };
        let mut dict_id: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut p1: uint8_t = 0 as uint8_t;
        let mut p2: uint8_t = 0 as uint8_t;
        if (*params).dictionary.contextual.context_based != 0 {
            p1 = (if position >= 1 as size_t {
                *ringbuffer.offset((position.wrapping_sub(1 as size_t) & ringbuffer_mask) as isize)
                    as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            }) as uint8_t;
            p2 = (if position >= 2 as size_t {
                *ringbuffer.offset((position.wrapping_sub(2 as size_t) & ringbuffer_mask) as isize)
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
        sr.len = 0 as size_t;
        sr.len_code_delta = 0 as ::core::ffi::c_int;
        sr.distance = 0 as size_t;
        sr.score = kMinScore;
        FindLongestMatchH6(
            privat,
            (*params).dictionary.contextual.dict[dict_id as usize],
            ringbuffer,
            ringbuffer_mask,
            dist_cache,
            position,
            max_length,
            max_distance,
            dictionary_start.wrapping_add(gap),
            (*params).dist.max_distance,
            &raw mut sr,
        );
        LookupCompoundDictionaryMatch(
            &raw const (*params).dictionary.compound,
            ringbuffer,
            ringbuffer_mask,
            dist_cache,
            position,
            max_length,
            dictionary_start,
            (*params).dist.max_distance,
            &raw mut sr,
        );
        if sr.score > kMinScore {
            let mut delayed_backward_references_in_row: ::core::ffi::c_int =
                0 as ::core::ffi::c_int;
            max_length = max_length.wrapping_sub(1);
            loop {
                let cost_diff_lazy: size_t = 175 as size_t;
                let mut sr2: HasherSearchResult = HasherSearchResult {
                    len: 0,
                    distance: 0,
                    score: 0,
                    len_code_delta: 0,
                };
                sr2.len = if (*params).quality < MIN_QUALITY_FOR_EXTENSIVE_REFERENCE_SEARCH {
                    brotli_min_size_t(sr.len.wrapping_sub(1 as size_t), max_length)
                } else {
                    0 as size_t
                };
                sr2.len_code_delta = 0 as ::core::ffi::c_int;
                sr2.distance = 0 as size_t;
                sr2.score = kMinScore;
                max_distance =
                    brotli_min_size_t(position.wrapping_add(1 as size_t), max_backward_limit);
                dictionary_start = brotli_min_size_t(
                    position
                        .wrapping_add(1 as size_t)
                        .wrapping_add(position_offset),
                    max_backward_limit,
                );
                if (*params).dictionary.contextual.context_based != 0 {
                    p2 = p1;
                    p1 = *ringbuffer.offset((position & ringbuffer_mask) as isize);
                    dict_id = (*params).dictionary.contextual.context_map[(*literal_context_lut
                        .offset(p1 as isize)
                        as ::core::ffi::c_int
                        | *literal_context_lut
                            .offset(256 as ::core::ffi::c_int as isize)
                            .offset(p2 as isize) as ::core::ffi::c_int)
                        as usize] as ::core::ffi::c_int;
                }
                FindLongestMatchH6(
                    privat,
                    (*params).dictionary.contextual.dict[dict_id as usize],
                    ringbuffer,
                    ringbuffer_mask,
                    dist_cache,
                    position.wrapping_add(1 as size_t),
                    max_length,
                    max_distance,
                    dictionary_start.wrapping_add(gap),
                    (*params).dist.max_distance,
                    &raw mut sr2,
                );
                LookupCompoundDictionaryMatch(
                    &raw const (*params).dictionary.compound,
                    ringbuffer,
                    ringbuffer_mask,
                    dist_cache,
                    position.wrapping_add(1 as size_t),
                    max_length,
                    dictionary_start,
                    (*params).dist.max_distance,
                    &raw mut sr2,
                );
                if !(sr2.score >= sr.score.wrapping_add(cost_diff_lazy)) {
                    break;
                }
                position = position.wrapping_add(1);
                insert_length = insert_length.wrapping_add(1);
                sr = sr2;
                delayed_backward_references_in_row += 1;
                if !(delayed_backward_references_in_row < 4 as ::core::ffi::c_int
                    && position.wrapping_add(HashTypeLengthH6()) < pos_end)
                {
                    break;
                }
                max_length = max_length.wrapping_sub(1);
            }
            apply_random_heuristics = position
                .wrapping_add((2 as size_t).wrapping_mul(sr.len))
                .wrapping_add(random_heuristics_window_size);
            dictionary_start =
                brotli_min_size_t(position.wrapping_add(position_offset), max_backward_limit);
            let mut distance_code: size_t =
                ComputeDistanceCode(sr.distance, dictionary_start.wrapping_add(gap), dist_cache);
            if sr.distance <= dictionary_start.wrapping_add(gap) && distance_code > 0 as size_t {
                *dist_cache.offset(3 as ::core::ffi::c_int as isize) =
                    *dist_cache.offset(2 as ::core::ffi::c_int as isize);
                *dist_cache.offset(2 as ::core::ffi::c_int as isize) =
                    *dist_cache.offset(1 as ::core::ffi::c_int as isize);
                *dist_cache.offset(1 as ::core::ffi::c_int as isize) =
                    *dist_cache.offset(0 as ::core::ffi::c_int as isize);
                *dist_cache.offset(0 as ::core::ffi::c_int as isize) =
                    sr.distance as ::core::ffi::c_int;
                PrepareDistanceCacheH6(privat, dist_cache);
            }
            let fresh35 = commands;
            commands = commands.offset(1);
            InitCommand(
                fresh35,
                &raw const (*params).dist,
                insert_length,
                sr.len,
                sr.len_code_delta,
                distance_code,
            );
            *num_literals = (*num_literals as ::core::ffi::c_ulong)
                .wrapping_add(insert_length as ::core::ffi::c_ulong)
                as size_t as size_t;
            insert_length = 0 as size_t;
            let mut range_start: size_t = position.wrapping_add(2 as size_t);
            let mut range_end: size_t = brotli_min_size_t(position.wrapping_add(sr.len), store_end);
            if sr.distance < sr.len >> 2 as ::core::ffi::c_int {
                range_start = brotli_min_size_t(
                    range_end,
                    brotli_max_size_t(
                        range_start,
                        position
                            .wrapping_add(sr.len)
                            .wrapping_sub(sr.distance << 2 as ::core::ffi::c_int),
                    ),
                );
            }
            StoreRangeH6(privat, ringbuffer, ringbuffer_mask, range_start, range_end);
            position = (position as ::core::ffi::c_ulong)
                .wrapping_add(sr.len as ::core::ffi::c_ulong) as size_t
                as size_t;
        } else {
            insert_length = insert_length.wrapping_add(1);
            position = position.wrapping_add(1);
            if position > apply_random_heuristics {
                if position
                    > apply_random_heuristics
                        .wrapping_add((4 as size_t).wrapping_mul(random_heuristics_window_size))
                {
                    let kMargin: size_t = brotli_max_size_t(
                        StoreLookaheadH6().wrapping_sub(1 as size_t),
                        4 as size_t,
                    ) as size_t;
                    let mut pos_jump: size_t = brotli_min_size_t(
                        position.wrapping_add(16 as size_t),
                        pos_end.wrapping_sub(kMargin),
                    );
                    while position < pos_jump {
                        StoreH6(privat, ringbuffer, ringbuffer_mask, position);
                        insert_length = (insert_length as ::core::ffi::c_ulong)
                            .wrapping_add(4 as ::core::ffi::c_ulong)
                            as size_t as size_t;
                        position = (position as ::core::ffi::c_ulong)
                            .wrapping_add(4 as ::core::ffi::c_ulong)
                            as size_t as size_t;
                    }
                } else {
                    let kMargin_0: size_t = brotli_max_size_t(
                        StoreLookaheadH6().wrapping_sub(1 as size_t),
                        2 as size_t,
                    ) as size_t;
                    let mut pos_jump_0: size_t = brotli_min_size_t(
                        position.wrapping_add(8 as size_t),
                        pos_end.wrapping_sub(kMargin_0),
                    );
                    while position < pos_jump_0 {
                        StoreH6(privat, ringbuffer, ringbuffer_mask, position);
                        insert_length = (insert_length as ::core::ffi::c_ulong)
                            .wrapping_add(2 as ::core::ffi::c_ulong)
                            as size_t as size_t;
                        position = (position as ::core::ffi::c_ulong)
                            .wrapping_add(2 as ::core::ffi::c_ulong)
                            as size_t as size_t;
                    }
                }
            }
        }
    }
    insert_length = (insert_length as ::core::ffi::c_ulong)
        .wrapping_add(pos_end.wrapping_sub(position) as ::core::ffi::c_ulong)
        as size_t as size_t;
    *last_insert_len = insert_length;
    *num_commands =
        (*num_commands as ::core::ffi::c_ulong).wrapping_add(commands.offset_from(orig_commands)
            as ::core::ffi::c_long
            as size_t
            as ::core::ffi::c_ulong) as size_t as size_t;
}
#[inline(never)]
unsafe extern "C" fn CreateBackwardReferencesDH40(
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
    let mut privat: *mut H40 = &raw mut (*hasher).privat._H40;
    let max_backward_limit: size_t = ((1 as ::core::ffi::c_int as size_t) << (*params).lgwin)
        .wrapping_sub(BROTLI_WINDOW_GAP as size_t);
    let position_offset: size_t = (*params).stream_offset;
    let orig_commands: *const Command = commands;
    let mut insert_length: size_t = *last_insert_len;
    let pos_end: size_t = position.wrapping_add(num_bytes);
    let store_end: size_t = if num_bytes >= StoreLookaheadH40() {
        position
            .wrapping_add(num_bytes)
            .wrapping_sub(StoreLookaheadH40() as size_t)
            .wrapping_add(1 as size_t)
    } else {
        position
    };
    let random_heuristics_window_size: size_t = LiteralSpreeLengthForSparseSearch(params) as size_t;
    let mut apply_random_heuristics: size_t = position.wrapping_add(random_heuristics_window_size);
    let gap: size_t = (*params).dictionary.compound.total_size;
    let kMinScore: size_t = BROTLI_SCORE_BASE.wrapping_add(100 as size_t);
    PrepareDistanceCacheH40(privat, dist_cache);
    while position.wrapping_add(HashTypeLengthH40()) < pos_end {
        let mut max_length: size_t = pos_end.wrapping_sub(position);
        let mut max_distance: size_t = brotli_min_size_t(position, max_backward_limit);
        let mut dictionary_start: size_t =
            brotli_min_size_t(position.wrapping_add(position_offset), max_backward_limit);
        let mut sr: HasherSearchResult = HasherSearchResult {
            len: 0,
            distance: 0,
            score: 0,
            len_code_delta: 0,
        };
        let mut dict_id: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut p1: uint8_t = 0 as uint8_t;
        let mut p2: uint8_t = 0 as uint8_t;
        if (*params).dictionary.contextual.context_based != 0 {
            p1 = (if position >= 1 as size_t {
                *ringbuffer.offset((position.wrapping_sub(1 as size_t) & ringbuffer_mask) as isize)
                    as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            }) as uint8_t;
            p2 = (if position >= 2 as size_t {
                *ringbuffer.offset((position.wrapping_sub(2 as size_t) & ringbuffer_mask) as isize)
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
        sr.len = 0 as size_t;
        sr.len_code_delta = 0 as ::core::ffi::c_int;
        sr.distance = 0 as size_t;
        sr.score = kMinScore;
        FindLongestMatchH40(
            privat,
            (*params).dictionary.contextual.dict[dict_id as usize],
            ringbuffer,
            ringbuffer_mask,
            dist_cache,
            position,
            max_length,
            max_distance,
            dictionary_start.wrapping_add(gap),
            (*params).dist.max_distance,
            &raw mut sr,
        );
        LookupCompoundDictionaryMatch(
            &raw const (*params).dictionary.compound,
            ringbuffer,
            ringbuffer_mask,
            dist_cache,
            position,
            max_length,
            dictionary_start,
            (*params).dist.max_distance,
            &raw mut sr,
        );
        if sr.score > kMinScore {
            let mut delayed_backward_references_in_row: ::core::ffi::c_int =
                0 as ::core::ffi::c_int;
            max_length = max_length.wrapping_sub(1);
            loop {
                let cost_diff_lazy: size_t = 175 as size_t;
                let mut sr2: HasherSearchResult = HasherSearchResult {
                    len: 0,
                    distance: 0,
                    score: 0,
                    len_code_delta: 0,
                };
                sr2.len = if (*params).quality < MIN_QUALITY_FOR_EXTENSIVE_REFERENCE_SEARCH {
                    brotli_min_size_t(sr.len.wrapping_sub(1 as size_t), max_length)
                } else {
                    0 as size_t
                };
                sr2.len_code_delta = 0 as ::core::ffi::c_int;
                sr2.distance = 0 as size_t;
                sr2.score = kMinScore;
                max_distance =
                    brotli_min_size_t(position.wrapping_add(1 as size_t), max_backward_limit);
                dictionary_start = brotli_min_size_t(
                    position
                        .wrapping_add(1 as size_t)
                        .wrapping_add(position_offset),
                    max_backward_limit,
                );
                if (*params).dictionary.contextual.context_based != 0 {
                    p2 = p1;
                    p1 = *ringbuffer.offset((position & ringbuffer_mask) as isize);
                    dict_id = (*params).dictionary.contextual.context_map[(*literal_context_lut
                        .offset(p1 as isize)
                        as ::core::ffi::c_int
                        | *literal_context_lut
                            .offset(256 as ::core::ffi::c_int as isize)
                            .offset(p2 as isize) as ::core::ffi::c_int)
                        as usize] as ::core::ffi::c_int;
                }
                FindLongestMatchH40(
                    privat,
                    (*params).dictionary.contextual.dict[dict_id as usize],
                    ringbuffer,
                    ringbuffer_mask,
                    dist_cache,
                    position.wrapping_add(1 as size_t),
                    max_length,
                    max_distance,
                    dictionary_start.wrapping_add(gap),
                    (*params).dist.max_distance,
                    &raw mut sr2,
                );
                LookupCompoundDictionaryMatch(
                    &raw const (*params).dictionary.compound,
                    ringbuffer,
                    ringbuffer_mask,
                    dist_cache,
                    position.wrapping_add(1 as size_t),
                    max_length,
                    dictionary_start,
                    (*params).dist.max_distance,
                    &raw mut sr2,
                );
                if !(sr2.score >= sr.score.wrapping_add(cost_diff_lazy)) {
                    break;
                }
                position = position.wrapping_add(1);
                insert_length = insert_length.wrapping_add(1);
                sr = sr2;
                delayed_backward_references_in_row += 1;
                if !(delayed_backward_references_in_row < 4 as ::core::ffi::c_int
                    && position.wrapping_add(HashTypeLengthH40()) < pos_end)
                {
                    break;
                }
                max_length = max_length.wrapping_sub(1);
            }
            apply_random_heuristics = position
                .wrapping_add((2 as size_t).wrapping_mul(sr.len))
                .wrapping_add(random_heuristics_window_size);
            dictionary_start =
                brotli_min_size_t(position.wrapping_add(position_offset), max_backward_limit);
            let mut distance_code: size_t =
                ComputeDistanceCode(sr.distance, dictionary_start.wrapping_add(gap), dist_cache);
            if sr.distance <= dictionary_start.wrapping_add(gap) && distance_code > 0 as size_t {
                *dist_cache.offset(3 as ::core::ffi::c_int as isize) =
                    *dist_cache.offset(2 as ::core::ffi::c_int as isize);
                *dist_cache.offset(2 as ::core::ffi::c_int as isize) =
                    *dist_cache.offset(1 as ::core::ffi::c_int as isize);
                *dist_cache.offset(1 as ::core::ffi::c_int as isize) =
                    *dist_cache.offset(0 as ::core::ffi::c_int as isize);
                *dist_cache.offset(0 as ::core::ffi::c_int as isize) =
                    sr.distance as ::core::ffi::c_int;
                PrepareDistanceCacheH40(privat, dist_cache);
            }
            let fresh32 = commands;
            commands = commands.offset(1);
            InitCommand(
                fresh32,
                &raw const (*params).dist,
                insert_length,
                sr.len,
                sr.len_code_delta,
                distance_code,
            );
            *num_literals = (*num_literals as ::core::ffi::c_ulong)
                .wrapping_add(insert_length as ::core::ffi::c_ulong)
                as size_t as size_t;
            insert_length = 0 as size_t;
            let mut range_start: size_t = position.wrapping_add(2 as size_t);
            let mut range_end: size_t = brotli_min_size_t(position.wrapping_add(sr.len), store_end);
            if sr.distance < sr.len >> 2 as ::core::ffi::c_int {
                range_start = brotli_min_size_t(
                    range_end,
                    brotli_max_size_t(
                        range_start,
                        position
                            .wrapping_add(sr.len)
                            .wrapping_sub(sr.distance << 2 as ::core::ffi::c_int),
                    ),
                );
            }
            StoreRangeH40(privat, ringbuffer, ringbuffer_mask, range_start, range_end);
            position = (position as ::core::ffi::c_ulong)
                .wrapping_add(sr.len as ::core::ffi::c_ulong) as size_t
                as size_t;
        } else {
            insert_length = insert_length.wrapping_add(1);
            position = position.wrapping_add(1);
            if position > apply_random_heuristics {
                if position
                    > apply_random_heuristics
                        .wrapping_add((4 as size_t).wrapping_mul(random_heuristics_window_size))
                {
                    let kMargin: size_t = brotli_max_size_t(
                        StoreLookaheadH40().wrapping_sub(1 as size_t),
                        4 as size_t,
                    ) as size_t;
                    let mut pos_jump: size_t = brotli_min_size_t(
                        position.wrapping_add(16 as size_t),
                        pos_end.wrapping_sub(kMargin),
                    );
                    while position < pos_jump {
                        StoreH40(privat, ringbuffer, ringbuffer_mask, position);
                        insert_length = (insert_length as ::core::ffi::c_ulong)
                            .wrapping_add(4 as ::core::ffi::c_ulong)
                            as size_t as size_t;
                        position = (position as ::core::ffi::c_ulong)
                            .wrapping_add(4 as ::core::ffi::c_ulong)
                            as size_t as size_t;
                    }
                } else {
                    let kMargin_0: size_t = brotli_max_size_t(
                        StoreLookaheadH40().wrapping_sub(1 as size_t),
                        2 as size_t,
                    ) as size_t;
                    let mut pos_jump_0: size_t = brotli_min_size_t(
                        position.wrapping_add(8 as size_t),
                        pos_end.wrapping_sub(kMargin_0),
                    );
                    while position < pos_jump_0 {
                        StoreH40(privat, ringbuffer, ringbuffer_mask, position);
                        insert_length = (insert_length as ::core::ffi::c_ulong)
                            .wrapping_add(2 as ::core::ffi::c_ulong)
                            as size_t as size_t;
                        position = (position as ::core::ffi::c_ulong)
                            .wrapping_add(2 as ::core::ffi::c_ulong)
                            as size_t as size_t;
                    }
                }
            }
        }
    }
    insert_length = (insert_length as ::core::ffi::c_ulong)
        .wrapping_add(pos_end.wrapping_sub(position) as ::core::ffi::c_ulong)
        as size_t as size_t;
    *last_insert_len = insert_length;
    *num_commands =
        (*num_commands as ::core::ffi::c_ulong).wrapping_add(commands.offset_from(orig_commands)
            as ::core::ffi::c_long
            as size_t
            as ::core::ffi::c_ulong) as size_t as size_t;
}
#[inline(never)]
unsafe extern "C" fn CreateBackwardReferencesDH41(
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
    let mut privat: *mut H41 = &raw mut (*hasher).privat._H41;
    let max_backward_limit: size_t = ((1 as ::core::ffi::c_int as size_t) << (*params).lgwin)
        .wrapping_sub(BROTLI_WINDOW_GAP as size_t);
    let position_offset: size_t = (*params).stream_offset;
    let orig_commands: *const Command = commands;
    let mut insert_length: size_t = *last_insert_len;
    let pos_end: size_t = position.wrapping_add(num_bytes);
    let store_end: size_t = if num_bytes >= StoreLookaheadH41() {
        position
            .wrapping_add(num_bytes)
            .wrapping_sub(StoreLookaheadH41() as size_t)
            .wrapping_add(1 as size_t)
    } else {
        position
    };
    let random_heuristics_window_size: size_t = LiteralSpreeLengthForSparseSearch(params) as size_t;
    let mut apply_random_heuristics: size_t = position.wrapping_add(random_heuristics_window_size);
    let gap: size_t = (*params).dictionary.compound.total_size;
    let kMinScore: size_t = BROTLI_SCORE_BASE.wrapping_add(100 as size_t);
    PrepareDistanceCacheH41(privat, dist_cache);
    while position.wrapping_add(HashTypeLengthH41()) < pos_end {
        let mut max_length: size_t = pos_end.wrapping_sub(position);
        let mut max_distance: size_t = brotli_min_size_t(position, max_backward_limit);
        let mut dictionary_start: size_t =
            brotli_min_size_t(position.wrapping_add(position_offset), max_backward_limit);
        let mut sr: HasherSearchResult = HasherSearchResult {
            len: 0,
            distance: 0,
            score: 0,
            len_code_delta: 0,
        };
        let mut dict_id: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut p1: uint8_t = 0 as uint8_t;
        let mut p2: uint8_t = 0 as uint8_t;
        if (*params).dictionary.contextual.context_based != 0 {
            p1 = (if position >= 1 as size_t {
                *ringbuffer.offset((position.wrapping_sub(1 as size_t) & ringbuffer_mask) as isize)
                    as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            }) as uint8_t;
            p2 = (if position >= 2 as size_t {
                *ringbuffer.offset((position.wrapping_sub(2 as size_t) & ringbuffer_mask) as isize)
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
        sr.len = 0 as size_t;
        sr.len_code_delta = 0 as ::core::ffi::c_int;
        sr.distance = 0 as size_t;
        sr.score = kMinScore;
        FindLongestMatchH41(
            privat,
            (*params).dictionary.contextual.dict[dict_id as usize],
            ringbuffer,
            ringbuffer_mask,
            dist_cache,
            position,
            max_length,
            max_distance,
            dictionary_start.wrapping_add(gap),
            (*params).dist.max_distance,
            &raw mut sr,
        );
        LookupCompoundDictionaryMatch(
            &raw const (*params).dictionary.compound,
            ringbuffer,
            ringbuffer_mask,
            dist_cache,
            position,
            max_length,
            dictionary_start,
            (*params).dist.max_distance,
            &raw mut sr,
        );
        if sr.score > kMinScore {
            let mut delayed_backward_references_in_row: ::core::ffi::c_int =
                0 as ::core::ffi::c_int;
            max_length = max_length.wrapping_sub(1);
            loop {
                let cost_diff_lazy: size_t = 175 as size_t;
                let mut sr2: HasherSearchResult = HasherSearchResult {
                    len: 0,
                    distance: 0,
                    score: 0,
                    len_code_delta: 0,
                };
                sr2.len = if (*params).quality < MIN_QUALITY_FOR_EXTENSIVE_REFERENCE_SEARCH {
                    brotli_min_size_t(sr.len.wrapping_sub(1 as size_t), max_length)
                } else {
                    0 as size_t
                };
                sr2.len_code_delta = 0 as ::core::ffi::c_int;
                sr2.distance = 0 as size_t;
                sr2.score = kMinScore;
                max_distance =
                    brotli_min_size_t(position.wrapping_add(1 as size_t), max_backward_limit);
                dictionary_start = brotli_min_size_t(
                    position
                        .wrapping_add(1 as size_t)
                        .wrapping_add(position_offset),
                    max_backward_limit,
                );
                if (*params).dictionary.contextual.context_based != 0 {
                    p2 = p1;
                    p1 = *ringbuffer.offset((position & ringbuffer_mask) as isize);
                    dict_id = (*params).dictionary.contextual.context_map[(*literal_context_lut
                        .offset(p1 as isize)
                        as ::core::ffi::c_int
                        | *literal_context_lut
                            .offset(256 as ::core::ffi::c_int as isize)
                            .offset(p2 as isize) as ::core::ffi::c_int)
                        as usize] as ::core::ffi::c_int;
                }
                FindLongestMatchH41(
                    privat,
                    (*params).dictionary.contextual.dict[dict_id as usize],
                    ringbuffer,
                    ringbuffer_mask,
                    dist_cache,
                    position.wrapping_add(1 as size_t),
                    max_length,
                    max_distance,
                    dictionary_start.wrapping_add(gap),
                    (*params).dist.max_distance,
                    &raw mut sr2,
                );
                LookupCompoundDictionaryMatch(
                    &raw const (*params).dictionary.compound,
                    ringbuffer,
                    ringbuffer_mask,
                    dist_cache,
                    position.wrapping_add(1 as size_t),
                    max_length,
                    dictionary_start,
                    (*params).dist.max_distance,
                    &raw mut sr2,
                );
                if !(sr2.score >= sr.score.wrapping_add(cost_diff_lazy)) {
                    break;
                }
                position = position.wrapping_add(1);
                insert_length = insert_length.wrapping_add(1);
                sr = sr2;
                delayed_backward_references_in_row += 1;
                if !(delayed_backward_references_in_row < 4 as ::core::ffi::c_int
                    && position.wrapping_add(HashTypeLengthH41()) < pos_end)
                {
                    break;
                }
                max_length = max_length.wrapping_sub(1);
            }
            apply_random_heuristics = position
                .wrapping_add((2 as size_t).wrapping_mul(sr.len))
                .wrapping_add(random_heuristics_window_size);
            dictionary_start =
                brotli_min_size_t(position.wrapping_add(position_offset), max_backward_limit);
            let mut distance_code: size_t =
                ComputeDistanceCode(sr.distance, dictionary_start.wrapping_add(gap), dist_cache);
            if sr.distance <= dictionary_start.wrapping_add(gap) && distance_code > 0 as size_t {
                *dist_cache.offset(3 as ::core::ffi::c_int as isize) =
                    *dist_cache.offset(2 as ::core::ffi::c_int as isize);
                *dist_cache.offset(2 as ::core::ffi::c_int as isize) =
                    *dist_cache.offset(1 as ::core::ffi::c_int as isize);
                *dist_cache.offset(1 as ::core::ffi::c_int as isize) =
                    *dist_cache.offset(0 as ::core::ffi::c_int as isize);
                *dist_cache.offset(0 as ::core::ffi::c_int as isize) =
                    sr.distance as ::core::ffi::c_int;
                PrepareDistanceCacheH41(privat, dist_cache);
            }
            let fresh31 = commands;
            commands = commands.offset(1);
            InitCommand(
                fresh31,
                &raw const (*params).dist,
                insert_length,
                sr.len,
                sr.len_code_delta,
                distance_code,
            );
            *num_literals = (*num_literals as ::core::ffi::c_ulong)
                .wrapping_add(insert_length as ::core::ffi::c_ulong)
                as size_t as size_t;
            insert_length = 0 as size_t;
            let mut range_start: size_t = position.wrapping_add(2 as size_t);
            let mut range_end: size_t = brotli_min_size_t(position.wrapping_add(sr.len), store_end);
            if sr.distance < sr.len >> 2 as ::core::ffi::c_int {
                range_start = brotli_min_size_t(
                    range_end,
                    brotli_max_size_t(
                        range_start,
                        position
                            .wrapping_add(sr.len)
                            .wrapping_sub(sr.distance << 2 as ::core::ffi::c_int),
                    ),
                );
            }
            StoreRangeH41(privat, ringbuffer, ringbuffer_mask, range_start, range_end);
            position = (position as ::core::ffi::c_ulong)
                .wrapping_add(sr.len as ::core::ffi::c_ulong) as size_t
                as size_t;
        } else {
            insert_length = insert_length.wrapping_add(1);
            position = position.wrapping_add(1);
            if position > apply_random_heuristics {
                if position
                    > apply_random_heuristics
                        .wrapping_add((4 as size_t).wrapping_mul(random_heuristics_window_size))
                {
                    let kMargin: size_t = brotli_max_size_t(
                        StoreLookaheadH41().wrapping_sub(1 as size_t),
                        4 as size_t,
                    ) as size_t;
                    let mut pos_jump: size_t = brotli_min_size_t(
                        position.wrapping_add(16 as size_t),
                        pos_end.wrapping_sub(kMargin),
                    );
                    while position < pos_jump {
                        StoreH41(privat, ringbuffer, ringbuffer_mask, position);
                        insert_length = (insert_length as ::core::ffi::c_ulong)
                            .wrapping_add(4 as ::core::ffi::c_ulong)
                            as size_t as size_t;
                        position = (position as ::core::ffi::c_ulong)
                            .wrapping_add(4 as ::core::ffi::c_ulong)
                            as size_t as size_t;
                    }
                } else {
                    let kMargin_0: size_t = brotli_max_size_t(
                        StoreLookaheadH41().wrapping_sub(1 as size_t),
                        2 as size_t,
                    ) as size_t;
                    let mut pos_jump_0: size_t = brotli_min_size_t(
                        position.wrapping_add(8 as size_t),
                        pos_end.wrapping_sub(kMargin_0),
                    );
                    while position < pos_jump_0 {
                        StoreH41(privat, ringbuffer, ringbuffer_mask, position);
                        insert_length = (insert_length as ::core::ffi::c_ulong)
                            .wrapping_add(2 as ::core::ffi::c_ulong)
                            as size_t as size_t;
                        position = (position as ::core::ffi::c_ulong)
                            .wrapping_add(2 as ::core::ffi::c_ulong)
                            as size_t as size_t;
                    }
                }
            }
        }
    }
    insert_length = (insert_length as ::core::ffi::c_ulong)
        .wrapping_add(pos_end.wrapping_sub(position) as ::core::ffi::c_ulong)
        as size_t as size_t;
    *last_insert_len = insert_length;
    *num_commands =
        (*num_commands as ::core::ffi::c_ulong).wrapping_add(commands.offset_from(orig_commands)
            as ::core::ffi::c_long
            as size_t
            as ::core::ffi::c_ulong) as size_t as size_t;
}
#[inline(never)]
unsafe extern "C" fn CreateBackwardReferencesDH42(
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
    let mut privat: *mut H42 = &raw mut (*hasher).privat._H42;
    let max_backward_limit: size_t = ((1 as ::core::ffi::c_int as size_t) << (*params).lgwin)
        .wrapping_sub(BROTLI_WINDOW_GAP as size_t);
    let position_offset: size_t = (*params).stream_offset;
    let orig_commands: *const Command = commands;
    let mut insert_length: size_t = *last_insert_len;
    let pos_end: size_t = position.wrapping_add(num_bytes);
    let store_end: size_t = if num_bytes >= StoreLookaheadH42() {
        position
            .wrapping_add(num_bytes)
            .wrapping_sub(StoreLookaheadH42() as size_t)
            .wrapping_add(1 as size_t)
    } else {
        position
    };
    let random_heuristics_window_size: size_t = LiteralSpreeLengthForSparseSearch(params) as size_t;
    let mut apply_random_heuristics: size_t = position.wrapping_add(random_heuristics_window_size);
    let gap: size_t = (*params).dictionary.compound.total_size;
    let kMinScore: size_t = BROTLI_SCORE_BASE.wrapping_add(100 as size_t);
    PrepareDistanceCacheH42(privat, dist_cache);
    while position.wrapping_add(HashTypeLengthH42()) < pos_end {
        let mut max_length: size_t = pos_end.wrapping_sub(position);
        let mut max_distance: size_t = brotli_min_size_t(position, max_backward_limit);
        let mut dictionary_start: size_t =
            brotli_min_size_t(position.wrapping_add(position_offset), max_backward_limit);
        let mut sr: HasherSearchResult = HasherSearchResult {
            len: 0,
            distance: 0,
            score: 0,
            len_code_delta: 0,
        };
        let mut dict_id: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut p1: uint8_t = 0 as uint8_t;
        let mut p2: uint8_t = 0 as uint8_t;
        if (*params).dictionary.contextual.context_based != 0 {
            p1 = (if position >= 1 as size_t {
                *ringbuffer.offset((position.wrapping_sub(1 as size_t) & ringbuffer_mask) as isize)
                    as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            }) as uint8_t;
            p2 = (if position >= 2 as size_t {
                *ringbuffer.offset((position.wrapping_sub(2 as size_t) & ringbuffer_mask) as isize)
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
        sr.len = 0 as size_t;
        sr.len_code_delta = 0 as ::core::ffi::c_int;
        sr.distance = 0 as size_t;
        sr.score = kMinScore;
        FindLongestMatchH42(
            privat,
            (*params).dictionary.contextual.dict[dict_id as usize],
            ringbuffer,
            ringbuffer_mask,
            dist_cache,
            position,
            max_length,
            max_distance,
            dictionary_start.wrapping_add(gap),
            (*params).dist.max_distance,
            &raw mut sr,
        );
        LookupCompoundDictionaryMatch(
            &raw const (*params).dictionary.compound,
            ringbuffer,
            ringbuffer_mask,
            dist_cache,
            position,
            max_length,
            dictionary_start,
            (*params).dist.max_distance,
            &raw mut sr,
        );
        if sr.score > kMinScore {
            let mut delayed_backward_references_in_row: ::core::ffi::c_int =
                0 as ::core::ffi::c_int;
            max_length = max_length.wrapping_sub(1);
            loop {
                let cost_diff_lazy: size_t = 175 as size_t;
                let mut sr2: HasherSearchResult = HasherSearchResult {
                    len: 0,
                    distance: 0,
                    score: 0,
                    len_code_delta: 0,
                };
                sr2.len = if (*params).quality < MIN_QUALITY_FOR_EXTENSIVE_REFERENCE_SEARCH {
                    brotli_min_size_t(sr.len.wrapping_sub(1 as size_t), max_length)
                } else {
                    0 as size_t
                };
                sr2.len_code_delta = 0 as ::core::ffi::c_int;
                sr2.distance = 0 as size_t;
                sr2.score = kMinScore;
                max_distance =
                    brotli_min_size_t(position.wrapping_add(1 as size_t), max_backward_limit);
                dictionary_start = brotli_min_size_t(
                    position
                        .wrapping_add(1 as size_t)
                        .wrapping_add(position_offset),
                    max_backward_limit,
                );
                if (*params).dictionary.contextual.context_based != 0 {
                    p2 = p1;
                    p1 = *ringbuffer.offset((position & ringbuffer_mask) as isize);
                    dict_id = (*params).dictionary.contextual.context_map[(*literal_context_lut
                        .offset(p1 as isize)
                        as ::core::ffi::c_int
                        | *literal_context_lut
                            .offset(256 as ::core::ffi::c_int as isize)
                            .offset(p2 as isize) as ::core::ffi::c_int)
                        as usize] as ::core::ffi::c_int;
                }
                FindLongestMatchH42(
                    privat,
                    (*params).dictionary.contextual.dict[dict_id as usize],
                    ringbuffer,
                    ringbuffer_mask,
                    dist_cache,
                    position.wrapping_add(1 as size_t),
                    max_length,
                    max_distance,
                    dictionary_start.wrapping_add(gap),
                    (*params).dist.max_distance,
                    &raw mut sr2,
                );
                LookupCompoundDictionaryMatch(
                    &raw const (*params).dictionary.compound,
                    ringbuffer,
                    ringbuffer_mask,
                    dist_cache,
                    position.wrapping_add(1 as size_t),
                    max_length,
                    dictionary_start,
                    (*params).dist.max_distance,
                    &raw mut sr2,
                );
                if !(sr2.score >= sr.score.wrapping_add(cost_diff_lazy)) {
                    break;
                }
                position = position.wrapping_add(1);
                insert_length = insert_length.wrapping_add(1);
                sr = sr2;
                delayed_backward_references_in_row += 1;
                if !(delayed_backward_references_in_row < 4 as ::core::ffi::c_int
                    && position.wrapping_add(HashTypeLengthH42()) < pos_end)
                {
                    break;
                }
                max_length = max_length.wrapping_sub(1);
            }
            apply_random_heuristics = position
                .wrapping_add((2 as size_t).wrapping_mul(sr.len))
                .wrapping_add(random_heuristics_window_size);
            dictionary_start =
                brotli_min_size_t(position.wrapping_add(position_offset), max_backward_limit);
            let mut distance_code: size_t =
                ComputeDistanceCode(sr.distance, dictionary_start.wrapping_add(gap), dist_cache);
            if sr.distance <= dictionary_start.wrapping_add(gap) && distance_code > 0 as size_t {
                *dist_cache.offset(3 as ::core::ffi::c_int as isize) =
                    *dist_cache.offset(2 as ::core::ffi::c_int as isize);
                *dist_cache.offset(2 as ::core::ffi::c_int as isize) =
                    *dist_cache.offset(1 as ::core::ffi::c_int as isize);
                *dist_cache.offset(1 as ::core::ffi::c_int as isize) =
                    *dist_cache.offset(0 as ::core::ffi::c_int as isize);
                *dist_cache.offset(0 as ::core::ffi::c_int as isize) =
                    sr.distance as ::core::ffi::c_int;
                PrepareDistanceCacheH42(privat, dist_cache);
            }
            let fresh30 = commands;
            commands = commands.offset(1);
            InitCommand(
                fresh30,
                &raw const (*params).dist,
                insert_length,
                sr.len,
                sr.len_code_delta,
                distance_code,
            );
            *num_literals = (*num_literals as ::core::ffi::c_ulong)
                .wrapping_add(insert_length as ::core::ffi::c_ulong)
                as size_t as size_t;
            insert_length = 0 as size_t;
            let mut range_start: size_t = position.wrapping_add(2 as size_t);
            let mut range_end: size_t = brotli_min_size_t(position.wrapping_add(sr.len), store_end);
            if sr.distance < sr.len >> 2 as ::core::ffi::c_int {
                range_start = brotli_min_size_t(
                    range_end,
                    brotli_max_size_t(
                        range_start,
                        position
                            .wrapping_add(sr.len)
                            .wrapping_sub(sr.distance << 2 as ::core::ffi::c_int),
                    ),
                );
            }
            StoreRangeH42(privat, ringbuffer, ringbuffer_mask, range_start, range_end);
            position = (position as ::core::ffi::c_ulong)
                .wrapping_add(sr.len as ::core::ffi::c_ulong) as size_t
                as size_t;
        } else {
            insert_length = insert_length.wrapping_add(1);
            position = position.wrapping_add(1);
            if position > apply_random_heuristics {
                if position
                    > apply_random_heuristics
                        .wrapping_add((4 as size_t).wrapping_mul(random_heuristics_window_size))
                {
                    let kMargin: size_t = brotli_max_size_t(
                        StoreLookaheadH42().wrapping_sub(1 as size_t),
                        4 as size_t,
                    ) as size_t;
                    let mut pos_jump: size_t = brotli_min_size_t(
                        position.wrapping_add(16 as size_t),
                        pos_end.wrapping_sub(kMargin),
                    );
                    while position < pos_jump {
                        StoreH42(privat, ringbuffer, ringbuffer_mask, position);
                        insert_length = (insert_length as ::core::ffi::c_ulong)
                            .wrapping_add(4 as ::core::ffi::c_ulong)
                            as size_t as size_t;
                        position = (position as ::core::ffi::c_ulong)
                            .wrapping_add(4 as ::core::ffi::c_ulong)
                            as size_t as size_t;
                    }
                } else {
                    let kMargin_0: size_t = brotli_max_size_t(
                        StoreLookaheadH42().wrapping_sub(1 as size_t),
                        2 as size_t,
                    ) as size_t;
                    let mut pos_jump_0: size_t = brotli_min_size_t(
                        position.wrapping_add(8 as size_t),
                        pos_end.wrapping_sub(kMargin_0),
                    );
                    while position < pos_jump_0 {
                        StoreH42(privat, ringbuffer, ringbuffer_mask, position);
                        insert_length = (insert_length as ::core::ffi::c_ulong)
                            .wrapping_add(2 as ::core::ffi::c_ulong)
                            as size_t as size_t;
                        position = (position as ::core::ffi::c_ulong)
                            .wrapping_add(2 as ::core::ffi::c_ulong)
                            as size_t as size_t;
                    }
                }
            }
        }
    }
    insert_length = (insert_length as ::core::ffi::c_ulong)
        .wrapping_add(pos_end.wrapping_sub(position) as ::core::ffi::c_ulong)
        as size_t as size_t;
    *last_insert_len = insert_length;
    *num_commands =
        (*num_commands as ::core::ffi::c_ulong).wrapping_add(commands.offset_from(orig_commands)
            as ::core::ffi::c_long
            as size_t
            as ::core::ffi::c_ulong) as size_t as size_t;
}
#[inline(never)]
unsafe extern "C" fn CreateBackwardReferencesDH55(
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
    let mut privat: *mut H55 = &raw mut (*hasher).privat._H55;
    let max_backward_limit: size_t = ((1 as ::core::ffi::c_int as size_t) << (*params).lgwin)
        .wrapping_sub(BROTLI_WINDOW_GAP as size_t);
    let position_offset: size_t = (*params).stream_offset;
    let orig_commands: *const Command = commands;
    let mut insert_length: size_t = *last_insert_len;
    let pos_end: size_t = position.wrapping_add(num_bytes);
    let store_end: size_t = if num_bytes >= StoreLookaheadH55() {
        position
            .wrapping_add(num_bytes)
            .wrapping_sub(StoreLookaheadH55() as size_t)
            .wrapping_add(1 as size_t)
    } else {
        position
    };
    let random_heuristics_window_size: size_t = LiteralSpreeLengthForSparseSearch(params) as size_t;
    let mut apply_random_heuristics: size_t = position.wrapping_add(random_heuristics_window_size);
    let gap: size_t = (*params).dictionary.compound.total_size;
    let kMinScore: size_t = BROTLI_SCORE_BASE.wrapping_add(100 as size_t);
    PrepareDistanceCacheH55(privat, dist_cache);
    while position.wrapping_add(HashTypeLengthH55()) < pos_end {
        let mut max_length: size_t = pos_end.wrapping_sub(position);
        let mut max_distance: size_t = brotli_min_size_t(position, max_backward_limit);
        let mut dictionary_start: size_t =
            brotli_min_size_t(position.wrapping_add(position_offset), max_backward_limit);
        let mut sr: HasherSearchResult = HasherSearchResult {
            len: 0,
            distance: 0,
            score: 0,
            len_code_delta: 0,
        };
        let mut dict_id: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut p1: uint8_t = 0 as uint8_t;
        let mut p2: uint8_t = 0 as uint8_t;
        if (*params).dictionary.contextual.context_based != 0 {
            p1 = (if position >= 1 as size_t {
                *ringbuffer.offset((position.wrapping_sub(1 as size_t) & ringbuffer_mask) as isize)
                    as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            }) as uint8_t;
            p2 = (if position >= 2 as size_t {
                *ringbuffer.offset((position.wrapping_sub(2 as size_t) & ringbuffer_mask) as isize)
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
        sr.len = 0 as size_t;
        sr.len_code_delta = 0 as ::core::ffi::c_int;
        sr.distance = 0 as size_t;
        sr.score = kMinScore;
        FindLongestMatchH55(
            privat,
            (*params).dictionary.contextual.dict[dict_id as usize],
            ringbuffer,
            ringbuffer_mask,
            dist_cache,
            position,
            max_length,
            max_distance,
            dictionary_start.wrapping_add(gap),
            (*params).dist.max_distance,
            &raw mut sr,
        );
        LookupCompoundDictionaryMatch(
            &raw const (*params).dictionary.compound,
            ringbuffer,
            ringbuffer_mask,
            dist_cache,
            position,
            max_length,
            dictionary_start,
            (*params).dist.max_distance,
            &raw mut sr,
        );
        if sr.score > kMinScore {
            let mut delayed_backward_references_in_row: ::core::ffi::c_int =
                0 as ::core::ffi::c_int;
            max_length = max_length.wrapping_sub(1);
            loop {
                let cost_diff_lazy: size_t = 175 as size_t;
                let mut sr2: HasherSearchResult = HasherSearchResult {
                    len: 0,
                    distance: 0,
                    score: 0,
                    len_code_delta: 0,
                };
                sr2.len = if (*params).quality < MIN_QUALITY_FOR_EXTENSIVE_REFERENCE_SEARCH {
                    brotli_min_size_t(sr.len.wrapping_sub(1 as size_t), max_length)
                } else {
                    0 as size_t
                };
                sr2.len_code_delta = 0 as ::core::ffi::c_int;
                sr2.distance = 0 as size_t;
                sr2.score = kMinScore;
                max_distance =
                    brotli_min_size_t(position.wrapping_add(1 as size_t), max_backward_limit);
                dictionary_start = brotli_min_size_t(
                    position
                        .wrapping_add(1 as size_t)
                        .wrapping_add(position_offset),
                    max_backward_limit,
                );
                if (*params).dictionary.contextual.context_based != 0 {
                    p2 = p1;
                    p1 = *ringbuffer.offset((position & ringbuffer_mask) as isize);
                    dict_id = (*params).dictionary.contextual.context_map[(*literal_context_lut
                        .offset(p1 as isize)
                        as ::core::ffi::c_int
                        | *literal_context_lut
                            .offset(256 as ::core::ffi::c_int as isize)
                            .offset(p2 as isize) as ::core::ffi::c_int)
                        as usize] as ::core::ffi::c_int;
                }
                FindLongestMatchH55(
                    privat,
                    (*params).dictionary.contextual.dict[dict_id as usize],
                    ringbuffer,
                    ringbuffer_mask,
                    dist_cache,
                    position.wrapping_add(1 as size_t),
                    max_length,
                    max_distance,
                    dictionary_start.wrapping_add(gap),
                    (*params).dist.max_distance,
                    &raw mut sr2,
                );
                LookupCompoundDictionaryMatch(
                    &raw const (*params).dictionary.compound,
                    ringbuffer,
                    ringbuffer_mask,
                    dist_cache,
                    position.wrapping_add(1 as size_t),
                    max_length,
                    dictionary_start,
                    (*params).dist.max_distance,
                    &raw mut sr2,
                );
                if !(sr2.score >= sr.score.wrapping_add(cost_diff_lazy)) {
                    break;
                }
                position = position.wrapping_add(1);
                insert_length = insert_length.wrapping_add(1);
                sr = sr2;
                delayed_backward_references_in_row += 1;
                if !(delayed_backward_references_in_row < 4 as ::core::ffi::c_int
                    && position.wrapping_add(HashTypeLengthH55()) < pos_end)
                {
                    break;
                }
                max_length = max_length.wrapping_sub(1);
            }
            apply_random_heuristics = position
                .wrapping_add((2 as size_t).wrapping_mul(sr.len))
                .wrapping_add(random_heuristics_window_size);
            dictionary_start =
                brotli_min_size_t(position.wrapping_add(position_offset), max_backward_limit);
            let mut distance_code: size_t =
                ComputeDistanceCode(sr.distance, dictionary_start.wrapping_add(gap), dist_cache);
            if sr.distance <= dictionary_start.wrapping_add(gap) && distance_code > 0 as size_t {
                *dist_cache.offset(3 as ::core::ffi::c_int as isize) =
                    *dist_cache.offset(2 as ::core::ffi::c_int as isize);
                *dist_cache.offset(2 as ::core::ffi::c_int as isize) =
                    *dist_cache.offset(1 as ::core::ffi::c_int as isize);
                *dist_cache.offset(1 as ::core::ffi::c_int as isize) =
                    *dist_cache.offset(0 as ::core::ffi::c_int as isize);
                *dist_cache.offset(0 as ::core::ffi::c_int as isize) =
                    sr.distance as ::core::ffi::c_int;
                PrepareDistanceCacheH55(privat, dist_cache);
            }
            let fresh29 = commands;
            commands = commands.offset(1);
            InitCommand(
                fresh29,
                &raw const (*params).dist,
                insert_length,
                sr.len,
                sr.len_code_delta,
                distance_code,
            );
            *num_literals = (*num_literals as ::core::ffi::c_ulong)
                .wrapping_add(insert_length as ::core::ffi::c_ulong)
                as size_t as size_t;
            insert_length = 0 as size_t;
            let mut range_start: size_t = position.wrapping_add(2 as size_t);
            let mut range_end: size_t = brotli_min_size_t(position.wrapping_add(sr.len), store_end);
            if sr.distance < sr.len >> 2 as ::core::ffi::c_int {
                range_start = brotli_min_size_t(
                    range_end,
                    brotli_max_size_t(
                        range_start,
                        position
                            .wrapping_add(sr.len)
                            .wrapping_sub(sr.distance << 2 as ::core::ffi::c_int),
                    ),
                );
            }
            StoreRangeH55(privat, ringbuffer, ringbuffer_mask, range_start, range_end);
            position = (position as ::core::ffi::c_ulong)
                .wrapping_add(sr.len as ::core::ffi::c_ulong) as size_t
                as size_t;
        } else {
            insert_length = insert_length.wrapping_add(1);
            position = position.wrapping_add(1);
            if position > apply_random_heuristics {
                if position
                    > apply_random_heuristics
                        .wrapping_add((4 as size_t).wrapping_mul(random_heuristics_window_size))
                {
                    let kMargin: size_t = brotli_max_size_t(
                        StoreLookaheadH55().wrapping_sub(1 as size_t),
                        4 as size_t,
                    ) as size_t;
                    let mut pos_jump: size_t = brotli_min_size_t(
                        position.wrapping_add(16 as size_t),
                        pos_end.wrapping_sub(kMargin),
                    );
                    while position < pos_jump {
                        StoreH55(privat, ringbuffer, ringbuffer_mask, position);
                        insert_length = (insert_length as ::core::ffi::c_ulong)
                            .wrapping_add(4 as ::core::ffi::c_ulong)
                            as size_t as size_t;
                        position = (position as ::core::ffi::c_ulong)
                            .wrapping_add(4 as ::core::ffi::c_ulong)
                            as size_t as size_t;
                    }
                } else {
                    let kMargin_0: size_t = brotli_max_size_t(
                        StoreLookaheadH55().wrapping_sub(1 as size_t),
                        2 as size_t,
                    ) as size_t;
                    let mut pos_jump_0: size_t = brotli_min_size_t(
                        position.wrapping_add(8 as size_t),
                        pos_end.wrapping_sub(kMargin_0),
                    );
                    while position < pos_jump_0 {
                        StoreH55(privat, ringbuffer, ringbuffer_mask, position);
                        insert_length = (insert_length as ::core::ffi::c_ulong)
                            .wrapping_add(2 as ::core::ffi::c_ulong)
                            as size_t as size_t;
                        position = (position as ::core::ffi::c_ulong)
                            .wrapping_add(2 as ::core::ffi::c_ulong)
                            as size_t as size_t;
                    }
                }
            }
        }
    }
    insert_length = (insert_length as ::core::ffi::c_ulong)
        .wrapping_add(pos_end.wrapping_sub(position) as ::core::ffi::c_ulong)
        as size_t as size_t;
    *last_insert_len = insert_length;
    *num_commands =
        (*num_commands as ::core::ffi::c_ulong).wrapping_add(commands.offset_from(orig_commands)
            as ::core::ffi::c_long
            as size_t
            as ::core::ffi::c_ulong) as size_t as size_t;
}
#[inline(never)]
unsafe extern "C" fn CreateBackwardReferencesDH65(
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
    let mut privat: *mut H65 = &raw mut (*hasher).privat._H65;
    let max_backward_limit: size_t = ((1 as ::core::ffi::c_int as size_t) << (*params).lgwin)
        .wrapping_sub(BROTLI_WINDOW_GAP as size_t);
    let position_offset: size_t = (*params).stream_offset;
    let orig_commands: *const Command = commands;
    let mut insert_length: size_t = *last_insert_len;
    let pos_end: size_t = position.wrapping_add(num_bytes);
    let store_end: size_t = if num_bytes >= StoreLookaheadH65() {
        position
            .wrapping_add(num_bytes)
            .wrapping_sub(StoreLookaheadH65() as size_t)
            .wrapping_add(1 as size_t)
    } else {
        position
    };
    let random_heuristics_window_size: size_t = LiteralSpreeLengthForSparseSearch(params) as size_t;
    let mut apply_random_heuristics: size_t = position.wrapping_add(random_heuristics_window_size);
    let gap: size_t = (*params).dictionary.compound.total_size;
    let kMinScore: size_t = BROTLI_SCORE_BASE.wrapping_add(100 as size_t);
    PrepareDistanceCacheH65(privat, dist_cache);
    while position.wrapping_add(HashTypeLengthH65()) < pos_end {
        let mut max_length: size_t = pos_end.wrapping_sub(position);
        let mut max_distance: size_t = brotli_min_size_t(position, max_backward_limit);
        let mut dictionary_start: size_t =
            brotli_min_size_t(position.wrapping_add(position_offset), max_backward_limit);
        let mut sr: HasherSearchResult = HasherSearchResult {
            len: 0,
            distance: 0,
            score: 0,
            len_code_delta: 0,
        };
        let mut dict_id: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut p1: uint8_t = 0 as uint8_t;
        let mut p2: uint8_t = 0 as uint8_t;
        if (*params).dictionary.contextual.context_based != 0 {
            p1 = (if position >= 1 as size_t {
                *ringbuffer.offset((position.wrapping_sub(1 as size_t) & ringbuffer_mask) as isize)
                    as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            }) as uint8_t;
            p2 = (if position >= 2 as size_t {
                *ringbuffer.offset((position.wrapping_sub(2 as size_t) & ringbuffer_mask) as isize)
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
        sr.len = 0 as size_t;
        sr.len_code_delta = 0 as ::core::ffi::c_int;
        sr.distance = 0 as size_t;
        sr.score = kMinScore;
        FindLongestMatchH65(
            privat,
            (*params).dictionary.contextual.dict[dict_id as usize],
            ringbuffer,
            ringbuffer_mask,
            dist_cache,
            position,
            max_length,
            max_distance,
            dictionary_start.wrapping_add(gap),
            (*params).dist.max_distance,
            &raw mut sr,
        );
        LookupCompoundDictionaryMatch(
            &raw const (*params).dictionary.compound,
            ringbuffer,
            ringbuffer_mask,
            dist_cache,
            position,
            max_length,
            dictionary_start,
            (*params).dist.max_distance,
            &raw mut sr,
        );
        if sr.score > kMinScore {
            let mut delayed_backward_references_in_row: ::core::ffi::c_int =
                0 as ::core::ffi::c_int;
            max_length = max_length.wrapping_sub(1);
            loop {
                let cost_diff_lazy: size_t = 175 as size_t;
                let mut sr2: HasherSearchResult = HasherSearchResult {
                    len: 0,
                    distance: 0,
                    score: 0,
                    len_code_delta: 0,
                };
                sr2.len = if (*params).quality < MIN_QUALITY_FOR_EXTENSIVE_REFERENCE_SEARCH {
                    brotli_min_size_t(sr.len.wrapping_sub(1 as size_t), max_length)
                } else {
                    0 as size_t
                };
                sr2.len_code_delta = 0 as ::core::ffi::c_int;
                sr2.distance = 0 as size_t;
                sr2.score = kMinScore;
                max_distance =
                    brotli_min_size_t(position.wrapping_add(1 as size_t), max_backward_limit);
                dictionary_start = brotli_min_size_t(
                    position
                        .wrapping_add(1 as size_t)
                        .wrapping_add(position_offset),
                    max_backward_limit,
                );
                if (*params).dictionary.contextual.context_based != 0 {
                    p2 = p1;
                    p1 = *ringbuffer.offset((position & ringbuffer_mask) as isize);
                    dict_id = (*params).dictionary.contextual.context_map[(*literal_context_lut
                        .offset(p1 as isize)
                        as ::core::ffi::c_int
                        | *literal_context_lut
                            .offset(256 as ::core::ffi::c_int as isize)
                            .offset(p2 as isize) as ::core::ffi::c_int)
                        as usize] as ::core::ffi::c_int;
                }
                FindLongestMatchH65(
                    privat,
                    (*params).dictionary.contextual.dict[dict_id as usize],
                    ringbuffer,
                    ringbuffer_mask,
                    dist_cache,
                    position.wrapping_add(1 as size_t),
                    max_length,
                    max_distance,
                    dictionary_start.wrapping_add(gap),
                    (*params).dist.max_distance,
                    &raw mut sr2,
                );
                LookupCompoundDictionaryMatch(
                    &raw const (*params).dictionary.compound,
                    ringbuffer,
                    ringbuffer_mask,
                    dist_cache,
                    position.wrapping_add(1 as size_t),
                    max_length,
                    dictionary_start,
                    (*params).dist.max_distance,
                    &raw mut sr2,
                );
                if !(sr2.score >= sr.score.wrapping_add(cost_diff_lazy)) {
                    break;
                }
                position = position.wrapping_add(1);
                insert_length = insert_length.wrapping_add(1);
                sr = sr2;
                delayed_backward_references_in_row += 1;
                if !(delayed_backward_references_in_row < 4 as ::core::ffi::c_int
                    && position.wrapping_add(HashTypeLengthH65()) < pos_end)
                {
                    break;
                }
                max_length = max_length.wrapping_sub(1);
            }
            apply_random_heuristics = position
                .wrapping_add((2 as size_t).wrapping_mul(sr.len))
                .wrapping_add(random_heuristics_window_size);
            dictionary_start =
                brotli_min_size_t(position.wrapping_add(position_offset), max_backward_limit);
            let mut distance_code: size_t =
                ComputeDistanceCode(sr.distance, dictionary_start.wrapping_add(gap), dist_cache);
            if sr.distance <= dictionary_start.wrapping_add(gap) && distance_code > 0 as size_t {
                *dist_cache.offset(3 as ::core::ffi::c_int as isize) =
                    *dist_cache.offset(2 as ::core::ffi::c_int as isize);
                *dist_cache.offset(2 as ::core::ffi::c_int as isize) =
                    *dist_cache.offset(1 as ::core::ffi::c_int as isize);
                *dist_cache.offset(1 as ::core::ffi::c_int as isize) =
                    *dist_cache.offset(0 as ::core::ffi::c_int as isize);
                *dist_cache.offset(0 as ::core::ffi::c_int as isize) =
                    sr.distance as ::core::ffi::c_int;
                PrepareDistanceCacheH65(privat, dist_cache);
            }
            let fresh28 = commands;
            commands = commands.offset(1);
            InitCommand(
                fresh28,
                &raw const (*params).dist,
                insert_length,
                sr.len,
                sr.len_code_delta,
                distance_code,
            );
            *num_literals = (*num_literals as ::core::ffi::c_ulong)
                .wrapping_add(insert_length as ::core::ffi::c_ulong)
                as size_t as size_t;
            insert_length = 0 as size_t;
            let mut range_start: size_t = position.wrapping_add(2 as size_t);
            let mut range_end: size_t = brotli_min_size_t(position.wrapping_add(sr.len), store_end);
            if sr.distance < sr.len >> 2 as ::core::ffi::c_int {
                range_start = brotli_min_size_t(
                    range_end,
                    brotli_max_size_t(
                        range_start,
                        position
                            .wrapping_add(sr.len)
                            .wrapping_sub(sr.distance << 2 as ::core::ffi::c_int),
                    ),
                );
            }
            StoreRangeH65(privat, ringbuffer, ringbuffer_mask, range_start, range_end);
            position = (position as ::core::ffi::c_ulong)
                .wrapping_add(sr.len as ::core::ffi::c_ulong) as size_t
                as size_t;
        } else {
            insert_length = insert_length.wrapping_add(1);
            position = position.wrapping_add(1);
            if position > apply_random_heuristics {
                if position
                    > apply_random_heuristics
                        .wrapping_add((4 as size_t).wrapping_mul(random_heuristics_window_size))
                {
                    let kMargin: size_t = brotli_max_size_t(
                        StoreLookaheadH65().wrapping_sub(1 as size_t),
                        4 as size_t,
                    ) as size_t;
                    let mut pos_jump: size_t = brotli_min_size_t(
                        position.wrapping_add(16 as size_t),
                        pos_end.wrapping_sub(kMargin),
                    );
                    while position < pos_jump {
                        StoreH65(privat, ringbuffer, ringbuffer_mask, position);
                        insert_length = (insert_length as ::core::ffi::c_ulong)
                            .wrapping_add(4 as ::core::ffi::c_ulong)
                            as size_t as size_t;
                        position = (position as ::core::ffi::c_ulong)
                            .wrapping_add(4 as ::core::ffi::c_ulong)
                            as size_t as size_t;
                    }
                } else {
                    let kMargin_0: size_t = brotli_max_size_t(
                        StoreLookaheadH65().wrapping_sub(1 as size_t),
                        2 as size_t,
                    ) as size_t;
                    let mut pos_jump_0: size_t = brotli_min_size_t(
                        position.wrapping_add(8 as size_t),
                        pos_end.wrapping_sub(kMargin_0),
                    );
                    while position < pos_jump_0 {
                        StoreH65(privat, ringbuffer, ringbuffer_mask, position);
                        insert_length = (insert_length as ::core::ffi::c_ulong)
                            .wrapping_add(2 as ::core::ffi::c_ulong)
                            as size_t as size_t;
                        position = (position as ::core::ffi::c_ulong)
                            .wrapping_add(2 as ::core::ffi::c_ulong)
                            as size_t as size_t;
                    }
                }
            }
        }
    }
    insert_length = (insert_length as ::core::ffi::c_ulong)
        .wrapping_add(pos_end.wrapping_sub(position) as ::core::ffi::c_ulong)
        as size_t as size_t;
    *last_insert_len = insert_length;
    *num_commands =
        (*num_commands as ::core::ffi::c_ulong).wrapping_add(commands.offset_from(orig_commands)
            as ::core::ffi::c_long
            as size_t
            as ::core::ffi::c_ulong) as size_t as size_t;
}
#[inline(never)]
unsafe extern "C" fn CreateBackwardReferencesDH58(
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
    let mut privat: *mut H58 = &raw mut (*hasher).privat._H58;
    let max_backward_limit: size_t = ((1 as ::core::ffi::c_int as size_t) << (*params).lgwin)
        .wrapping_sub(BROTLI_WINDOW_GAP as size_t);
    let position_offset: size_t = (*params).stream_offset;
    let orig_commands: *const Command = commands;
    let mut insert_length: size_t = *last_insert_len;
    let pos_end: size_t = position.wrapping_add(num_bytes);
    let store_end: size_t = if num_bytes >= StoreLookaheadH58() {
        position
            .wrapping_add(num_bytes)
            .wrapping_sub(StoreLookaheadH58() as size_t)
            .wrapping_add(1 as size_t)
    } else {
        position
    };
    let random_heuristics_window_size: size_t = LiteralSpreeLengthForSparseSearch(params) as size_t;
    let mut apply_random_heuristics: size_t = position.wrapping_add(random_heuristics_window_size);
    let gap: size_t = (*params).dictionary.compound.total_size;
    let kMinScore: size_t = BROTLI_SCORE_BASE.wrapping_add(100 as size_t);
    PrepareDistanceCacheH58(privat, dist_cache);
    while position.wrapping_add(HashTypeLengthH58()) < pos_end {
        let mut max_length: size_t = pos_end.wrapping_sub(position);
        let mut max_distance: size_t = brotli_min_size_t(position, max_backward_limit);
        let mut dictionary_start: size_t =
            brotli_min_size_t(position.wrapping_add(position_offset), max_backward_limit);
        let mut sr: HasherSearchResult = HasherSearchResult {
            len: 0,
            distance: 0,
            score: 0,
            len_code_delta: 0,
        };
        let mut dict_id: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut p1: uint8_t = 0 as uint8_t;
        let mut p2: uint8_t = 0 as uint8_t;
        if (*params).dictionary.contextual.context_based != 0 {
            p1 = (if position >= 1 as size_t {
                *ringbuffer.offset((position.wrapping_sub(1 as size_t) & ringbuffer_mask) as isize)
                    as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            }) as uint8_t;
            p2 = (if position >= 2 as size_t {
                *ringbuffer.offset((position.wrapping_sub(2 as size_t) & ringbuffer_mask) as isize)
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
        sr.len = 0 as size_t;
        sr.len_code_delta = 0 as ::core::ffi::c_int;
        sr.distance = 0 as size_t;
        sr.score = kMinScore;
        FindLongestMatchH58(
            privat,
            (*params).dictionary.contextual.dict[dict_id as usize],
            ringbuffer,
            ringbuffer_mask,
            dist_cache,
            position,
            max_length,
            max_distance,
            dictionary_start.wrapping_add(gap),
            (*params).dist.max_distance,
            &raw mut sr,
        );
        LookupCompoundDictionaryMatch(
            &raw const (*params).dictionary.compound,
            ringbuffer,
            ringbuffer_mask,
            dist_cache,
            position,
            max_length,
            dictionary_start,
            (*params).dist.max_distance,
            &raw mut sr,
        );
        if sr.score > kMinScore {
            let mut delayed_backward_references_in_row: ::core::ffi::c_int =
                0 as ::core::ffi::c_int;
            max_length = max_length.wrapping_sub(1);
            loop {
                let cost_diff_lazy: size_t = 175 as size_t;
                let mut sr2: HasherSearchResult = HasherSearchResult {
                    len: 0,
                    distance: 0,
                    score: 0,
                    len_code_delta: 0,
                };
                sr2.len = if (*params).quality < MIN_QUALITY_FOR_EXTENSIVE_REFERENCE_SEARCH {
                    brotli_min_size_t(sr.len.wrapping_sub(1 as size_t), max_length)
                } else {
                    0 as size_t
                };
                sr2.len_code_delta = 0 as ::core::ffi::c_int;
                sr2.distance = 0 as size_t;
                sr2.score = kMinScore;
                max_distance =
                    brotli_min_size_t(position.wrapping_add(1 as size_t), max_backward_limit);
                dictionary_start = brotli_min_size_t(
                    position
                        .wrapping_add(1 as size_t)
                        .wrapping_add(position_offset),
                    max_backward_limit,
                );
                if (*params).dictionary.contextual.context_based != 0 {
                    p2 = p1;
                    p1 = *ringbuffer.offset((position & ringbuffer_mask) as isize);
                    dict_id = (*params).dictionary.contextual.context_map[(*literal_context_lut
                        .offset(p1 as isize)
                        as ::core::ffi::c_int
                        | *literal_context_lut
                            .offset(256 as ::core::ffi::c_int as isize)
                            .offset(p2 as isize) as ::core::ffi::c_int)
                        as usize] as ::core::ffi::c_int;
                }
                FindLongestMatchH58(
                    privat,
                    (*params).dictionary.contextual.dict[dict_id as usize],
                    ringbuffer,
                    ringbuffer_mask,
                    dist_cache,
                    position.wrapping_add(1 as size_t),
                    max_length,
                    max_distance,
                    dictionary_start.wrapping_add(gap),
                    (*params).dist.max_distance,
                    &raw mut sr2,
                );
                LookupCompoundDictionaryMatch(
                    &raw const (*params).dictionary.compound,
                    ringbuffer,
                    ringbuffer_mask,
                    dist_cache,
                    position.wrapping_add(1 as size_t),
                    max_length,
                    dictionary_start,
                    (*params).dist.max_distance,
                    &raw mut sr2,
                );
                if !(sr2.score >= sr.score.wrapping_add(cost_diff_lazy)) {
                    break;
                }
                position = position.wrapping_add(1);
                insert_length = insert_length.wrapping_add(1);
                sr = sr2;
                delayed_backward_references_in_row += 1;
                if !(delayed_backward_references_in_row < 4 as ::core::ffi::c_int
                    && position.wrapping_add(HashTypeLengthH58()) < pos_end)
                {
                    break;
                }
                max_length = max_length.wrapping_sub(1);
            }
            apply_random_heuristics = position
                .wrapping_add((2 as size_t).wrapping_mul(sr.len))
                .wrapping_add(random_heuristics_window_size);
            dictionary_start =
                brotli_min_size_t(position.wrapping_add(position_offset), max_backward_limit);
            let mut distance_code: size_t =
                ComputeDistanceCode(sr.distance, dictionary_start.wrapping_add(gap), dist_cache);
            if sr.distance <= dictionary_start.wrapping_add(gap) && distance_code > 0 as size_t {
                *dist_cache.offset(3 as ::core::ffi::c_int as isize) =
                    *dist_cache.offset(2 as ::core::ffi::c_int as isize);
                *dist_cache.offset(2 as ::core::ffi::c_int as isize) =
                    *dist_cache.offset(1 as ::core::ffi::c_int as isize);
                *dist_cache.offset(1 as ::core::ffi::c_int as isize) =
                    *dist_cache.offset(0 as ::core::ffi::c_int as isize);
                *dist_cache.offset(0 as ::core::ffi::c_int as isize) =
                    sr.distance as ::core::ffi::c_int;
                PrepareDistanceCacheH58(privat, dist_cache);
            }
            let fresh34 = commands;
            commands = commands.offset(1);
            InitCommand(
                fresh34,
                &raw const (*params).dist,
                insert_length,
                sr.len,
                sr.len_code_delta,
                distance_code,
            );
            *num_literals = (*num_literals as ::core::ffi::c_ulong)
                .wrapping_add(insert_length as ::core::ffi::c_ulong)
                as size_t as size_t;
            insert_length = 0 as size_t;
            let mut range_start: size_t = position.wrapping_add(2 as size_t);
            let mut range_end: size_t = brotli_min_size_t(position.wrapping_add(sr.len), store_end);
            if sr.distance < sr.len >> 2 as ::core::ffi::c_int {
                range_start = brotli_min_size_t(
                    range_end,
                    brotli_max_size_t(
                        range_start,
                        position
                            .wrapping_add(sr.len)
                            .wrapping_sub(sr.distance << 2 as ::core::ffi::c_int),
                    ),
                );
            }
            StoreRangeH58(privat, ringbuffer, ringbuffer_mask, range_start, range_end);
            position = (position as ::core::ffi::c_ulong)
                .wrapping_add(sr.len as ::core::ffi::c_ulong) as size_t
                as size_t;
        } else {
            insert_length = insert_length.wrapping_add(1);
            position = position.wrapping_add(1);
            if position > apply_random_heuristics {
                if position
                    > apply_random_heuristics
                        .wrapping_add((4 as size_t).wrapping_mul(random_heuristics_window_size))
                {
                    let kMargin: size_t = brotli_max_size_t(
                        StoreLookaheadH58().wrapping_sub(1 as size_t),
                        4 as size_t,
                    ) as size_t;
                    let mut pos_jump: size_t = brotli_min_size_t(
                        position.wrapping_add(16 as size_t),
                        pos_end.wrapping_sub(kMargin),
                    );
                    while position < pos_jump {
                        StoreH58(privat, ringbuffer, ringbuffer_mask, position);
                        insert_length = (insert_length as ::core::ffi::c_ulong)
                            .wrapping_add(4 as ::core::ffi::c_ulong)
                            as size_t as size_t;
                        position = (position as ::core::ffi::c_ulong)
                            .wrapping_add(4 as ::core::ffi::c_ulong)
                            as size_t as size_t;
                    }
                } else {
                    let kMargin_0: size_t = brotli_max_size_t(
                        StoreLookaheadH58().wrapping_sub(1 as size_t),
                        2 as size_t,
                    ) as size_t;
                    let mut pos_jump_0: size_t = brotli_min_size_t(
                        position.wrapping_add(8 as size_t),
                        pos_end.wrapping_sub(kMargin_0),
                    );
                    while position < pos_jump_0 {
                        StoreH58(privat, ringbuffer, ringbuffer_mask, position);
                        insert_length = (insert_length as ::core::ffi::c_ulong)
                            .wrapping_add(2 as ::core::ffi::c_ulong)
                            as size_t as size_t;
                        position = (position as ::core::ffi::c_ulong)
                            .wrapping_add(2 as ::core::ffi::c_ulong)
                            as size_t as size_t;
                    }
                }
            }
        }
    }
    insert_length = (insert_length as ::core::ffi::c_ulong)
        .wrapping_add(pos_end.wrapping_sub(position) as ::core::ffi::c_ulong)
        as size_t as size_t;
    *last_insert_len = insert_length;
    *num_commands =
        (*num_commands as ::core::ffi::c_ulong).wrapping_add(commands.offset_from(orig_commands)
            as ::core::ffi::c_long
            as size_t
            as ::core::ffi::c_ulong) as size_t as size_t;
}
#[inline(never)]
unsafe extern "C" fn CreateBackwardReferencesDH68(
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
    let mut privat: *mut H68 = &raw mut (*hasher).privat._H68;
    let max_backward_limit: size_t = ((1 as ::core::ffi::c_int as size_t) << (*params).lgwin)
        .wrapping_sub(BROTLI_WINDOW_GAP as size_t);
    let position_offset: size_t = (*params).stream_offset;
    let orig_commands: *const Command = commands;
    let mut insert_length: size_t = *last_insert_len;
    let pos_end: size_t = position.wrapping_add(num_bytes);
    let store_end: size_t = if num_bytes >= StoreLookaheadH68() {
        position
            .wrapping_add(num_bytes)
            .wrapping_sub(StoreLookaheadH68() as size_t)
            .wrapping_add(1 as size_t)
    } else {
        position
    };
    let random_heuristics_window_size: size_t = LiteralSpreeLengthForSparseSearch(params) as size_t;
    let mut apply_random_heuristics: size_t = position.wrapping_add(random_heuristics_window_size);
    let gap: size_t = (*params).dictionary.compound.total_size;
    let kMinScore: size_t = BROTLI_SCORE_BASE.wrapping_add(100 as size_t);
    PrepareDistanceCacheH68(privat, dist_cache);
    while position.wrapping_add(HashTypeLengthH68()) < pos_end {
        let mut max_length: size_t = pos_end.wrapping_sub(position);
        let mut max_distance: size_t = brotli_min_size_t(position, max_backward_limit);
        let mut dictionary_start: size_t =
            brotli_min_size_t(position.wrapping_add(position_offset), max_backward_limit);
        let mut sr: HasherSearchResult = HasherSearchResult {
            len: 0,
            distance: 0,
            score: 0,
            len_code_delta: 0,
        };
        let mut dict_id: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut p1: uint8_t = 0 as uint8_t;
        let mut p2: uint8_t = 0 as uint8_t;
        if (*params).dictionary.contextual.context_based != 0 {
            p1 = (if position >= 1 as size_t {
                *ringbuffer.offset((position.wrapping_sub(1 as size_t) & ringbuffer_mask) as isize)
                    as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            }) as uint8_t;
            p2 = (if position >= 2 as size_t {
                *ringbuffer.offset((position.wrapping_sub(2 as size_t) & ringbuffer_mask) as isize)
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
        sr.len = 0 as size_t;
        sr.len_code_delta = 0 as ::core::ffi::c_int;
        sr.distance = 0 as size_t;
        sr.score = kMinScore;
        FindLongestMatchH68(
            privat,
            (*params).dictionary.contextual.dict[dict_id as usize],
            ringbuffer,
            ringbuffer_mask,
            dist_cache,
            position,
            max_length,
            max_distance,
            dictionary_start.wrapping_add(gap),
            (*params).dist.max_distance,
            &raw mut sr,
        );
        LookupCompoundDictionaryMatch(
            &raw const (*params).dictionary.compound,
            ringbuffer,
            ringbuffer_mask,
            dist_cache,
            position,
            max_length,
            dictionary_start,
            (*params).dist.max_distance,
            &raw mut sr,
        );
        if sr.score > kMinScore {
            let mut delayed_backward_references_in_row: ::core::ffi::c_int =
                0 as ::core::ffi::c_int;
            max_length = max_length.wrapping_sub(1);
            loop {
                let cost_diff_lazy: size_t = 175 as size_t;
                let mut sr2: HasherSearchResult = HasherSearchResult {
                    len: 0,
                    distance: 0,
                    score: 0,
                    len_code_delta: 0,
                };
                sr2.len = if (*params).quality < MIN_QUALITY_FOR_EXTENSIVE_REFERENCE_SEARCH {
                    brotli_min_size_t(sr.len.wrapping_sub(1 as size_t), max_length)
                } else {
                    0 as size_t
                };
                sr2.len_code_delta = 0 as ::core::ffi::c_int;
                sr2.distance = 0 as size_t;
                sr2.score = kMinScore;
                max_distance =
                    brotli_min_size_t(position.wrapping_add(1 as size_t), max_backward_limit);
                dictionary_start = brotli_min_size_t(
                    position
                        .wrapping_add(1 as size_t)
                        .wrapping_add(position_offset),
                    max_backward_limit,
                );
                if (*params).dictionary.contextual.context_based != 0 {
                    p2 = p1;
                    p1 = *ringbuffer.offset((position & ringbuffer_mask) as isize);
                    dict_id = (*params).dictionary.contextual.context_map[(*literal_context_lut
                        .offset(p1 as isize)
                        as ::core::ffi::c_int
                        | *literal_context_lut
                            .offset(256 as ::core::ffi::c_int as isize)
                            .offset(p2 as isize) as ::core::ffi::c_int)
                        as usize] as ::core::ffi::c_int;
                }
                FindLongestMatchH68(
                    privat,
                    (*params).dictionary.contextual.dict[dict_id as usize],
                    ringbuffer,
                    ringbuffer_mask,
                    dist_cache,
                    position.wrapping_add(1 as size_t),
                    max_length,
                    max_distance,
                    dictionary_start.wrapping_add(gap),
                    (*params).dist.max_distance,
                    &raw mut sr2,
                );
                LookupCompoundDictionaryMatch(
                    &raw const (*params).dictionary.compound,
                    ringbuffer,
                    ringbuffer_mask,
                    dist_cache,
                    position.wrapping_add(1 as size_t),
                    max_length,
                    dictionary_start,
                    (*params).dist.max_distance,
                    &raw mut sr2,
                );
                if !(sr2.score >= sr.score.wrapping_add(cost_diff_lazy)) {
                    break;
                }
                position = position.wrapping_add(1);
                insert_length = insert_length.wrapping_add(1);
                sr = sr2;
                delayed_backward_references_in_row += 1;
                if !(delayed_backward_references_in_row < 4 as ::core::ffi::c_int
                    && position.wrapping_add(HashTypeLengthH68()) < pos_end)
                {
                    break;
                }
                max_length = max_length.wrapping_sub(1);
            }
            apply_random_heuristics = position
                .wrapping_add((2 as size_t).wrapping_mul(sr.len))
                .wrapping_add(random_heuristics_window_size);
            dictionary_start =
                brotli_min_size_t(position.wrapping_add(position_offset), max_backward_limit);
            let mut distance_code: size_t =
                ComputeDistanceCode(sr.distance, dictionary_start.wrapping_add(gap), dist_cache);
            if sr.distance <= dictionary_start.wrapping_add(gap) && distance_code > 0 as size_t {
                *dist_cache.offset(3 as ::core::ffi::c_int as isize) =
                    *dist_cache.offset(2 as ::core::ffi::c_int as isize);
                *dist_cache.offset(2 as ::core::ffi::c_int as isize) =
                    *dist_cache.offset(1 as ::core::ffi::c_int as isize);
                *dist_cache.offset(1 as ::core::ffi::c_int as isize) =
                    *dist_cache.offset(0 as ::core::ffi::c_int as isize);
                *dist_cache.offset(0 as ::core::ffi::c_int as isize) =
                    sr.distance as ::core::ffi::c_int;
                PrepareDistanceCacheH68(privat, dist_cache);
            }
            let fresh33 = commands;
            commands = commands.offset(1);
            InitCommand(
                fresh33,
                &raw const (*params).dist,
                insert_length,
                sr.len,
                sr.len_code_delta,
                distance_code,
            );
            *num_literals = (*num_literals as ::core::ffi::c_ulong)
                .wrapping_add(insert_length as ::core::ffi::c_ulong)
                as size_t as size_t;
            insert_length = 0 as size_t;
            let mut range_start: size_t = position.wrapping_add(2 as size_t);
            let mut range_end: size_t = brotli_min_size_t(position.wrapping_add(sr.len), store_end);
            if sr.distance < sr.len >> 2 as ::core::ffi::c_int {
                range_start = brotli_min_size_t(
                    range_end,
                    brotli_max_size_t(
                        range_start,
                        position
                            .wrapping_add(sr.len)
                            .wrapping_sub(sr.distance << 2 as ::core::ffi::c_int),
                    ),
                );
            }
            StoreRangeH68(privat, ringbuffer, ringbuffer_mask, range_start, range_end);
            position = (position as ::core::ffi::c_ulong)
                .wrapping_add(sr.len as ::core::ffi::c_ulong) as size_t
                as size_t;
        } else {
            insert_length = insert_length.wrapping_add(1);
            position = position.wrapping_add(1);
            if position > apply_random_heuristics {
                if position
                    > apply_random_heuristics
                        .wrapping_add((4 as size_t).wrapping_mul(random_heuristics_window_size))
                {
                    let kMargin: size_t = brotli_max_size_t(
                        StoreLookaheadH68().wrapping_sub(1 as size_t),
                        4 as size_t,
                    ) as size_t;
                    let mut pos_jump: size_t = brotli_min_size_t(
                        position.wrapping_add(16 as size_t),
                        pos_end.wrapping_sub(kMargin),
                    );
                    while position < pos_jump {
                        StoreH68(privat, ringbuffer, ringbuffer_mask, position);
                        insert_length = (insert_length as ::core::ffi::c_ulong)
                            .wrapping_add(4 as ::core::ffi::c_ulong)
                            as size_t as size_t;
                        position = (position as ::core::ffi::c_ulong)
                            .wrapping_add(4 as ::core::ffi::c_ulong)
                            as size_t as size_t;
                    }
                } else {
                    let kMargin_0: size_t = brotli_max_size_t(
                        StoreLookaheadH68().wrapping_sub(1 as size_t),
                        2 as size_t,
                    ) as size_t;
                    let mut pos_jump_0: size_t = brotli_min_size_t(
                        position.wrapping_add(8 as size_t),
                        pos_end.wrapping_sub(kMargin_0),
                    );
                    while position < pos_jump_0 {
                        StoreH68(privat, ringbuffer, ringbuffer_mask, position);
                        insert_length = (insert_length as ::core::ffi::c_ulong)
                            .wrapping_add(2 as ::core::ffi::c_ulong)
                            as size_t as size_t;
                        position = (position as ::core::ffi::c_ulong)
                            .wrapping_add(2 as ::core::ffi::c_ulong)
                            as size_t as size_t;
                    }
                }
            }
        }
    }
    insert_length = (insert_length as ::core::ffi::c_ulong)
        .wrapping_add(pos_end.wrapping_sub(position) as ::core::ffi::c_ulong)
        as size_t as size_t;
    *last_insert_len = insert_length;
    *num_commands =
        (*num_commands as ::core::ffi::c_ulong).wrapping_add(commands.offset_from(orig_commands)
            as ::core::ffi::c_long
            as size_t
            as ::core::ffi::c_ulong) as size_t as size_t;
}
pub const TAG_HASH_BITS_0: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const TAG_HASH_MASK_0: ::core::ffi::c_int =
    ((1 as ::core::ffi::c_int) << TAG_HASH_BITS_0) - 1 as ::core::ffi::c_int;
#[inline(always)]
unsafe extern "C" fn HashTypeLengthH68() -> size_t {
    return 8 as size_t;
}
#[inline(always)]
unsafe extern "C" fn StoreLookaheadH68() -> size_t {
    return 8 as size_t;
}
#[inline(always)]
unsafe extern "C" fn HashBytesH68(mut data: *const uint8_t, mut hash_mul: uint64_t) -> size_t {
    let h: uint64_t = (BrotliUnalignedRead64(data as *const ::core::ffi::c_void) as uint64_t)
        .wrapping_mul(hash_mul);
    return (h >> 64 as ::core::ffi::c_int - 15 as ::core::ffi::c_int - TAG_HASH_BITS_0) as size_t;
}
#[inline(always)]
unsafe extern "C" fn StoreH68(
    mut self_0: *mut H68,
    mut data: *const uint8_t,
    mask: size_t,
    ix: size_t,
) {
    let mut num: *mut uint16_t = (*self_0).num_;
    let mut tags: *mut uint8_t = (*self_0).tags_;
    let mut buckets: *mut uint32_t = (*self_0).buckets_;
    let hash: size_t = HashBytesH68(
        data.offset((ix & mask) as isize) as *const uint8_t,
        (*self_0).hash_mul_,
    ) as size_t;
    let key: size_t = hash >> TAG_HASH_BITS_0;
    let tag: uint8_t = (hash & TAG_HASH_MASK_0 as size_t) as uint8_t;
    let minor_ix: size_t =
        (*num.offset(key as isize) as uint32_t & (*self_0).block_mask_) as size_t;
    let offset: size_t = minor_ix.wrapping_add(key << (*self_0).block_bits_);
    let ref mut fresh6 = *num.offset(key as isize);
    *fresh6 = (*fresh6).wrapping_sub(1);
    *buckets.offset(offset as isize) = ix as uint32_t;
    *tags.offset(offset as isize) = tag;
}
#[inline(always)]
unsafe extern "C" fn StoreRangeH68(
    mut self_0: *mut H68,
    mut data: *const uint8_t,
    mask: size_t,
    ix_start: size_t,
    ix_end: size_t,
) {
    let mut i: size_t = 0;
    i = ix_start;
    while i < ix_end {
        StoreH68(self_0, data, mask, i);
        i = i.wrapping_add(1);
    }
}
#[inline(always)]
unsafe extern "C" fn PrepareDistanceCacheH68(
    mut self_0: *mut H68,
    mut distance_cache: *mut ::core::ffi::c_int,
) {
    PrepareDistanceCache(distance_cache, (*self_0).num_last_distances_to_check_);
}
#[inline(always)]
unsafe extern "C" fn FindLongestMatchH68(
    mut self_0: *mut H68,
    mut dictionary: *const BrotliEncoderDictionary,
    mut data: *const uint8_t,
    ring_buffer_mask: size_t,
    mut distance_cache: *const ::core::ffi::c_int,
    cur_ix: size_t,
    max_length: size_t,
    max_backward: size_t,
    dictionary_distance: size_t,
    max_distance: size_t,
    mut out: *mut HasherSearchResult,
) {
    let mut num: *mut uint16_t = (*self_0).num_;
    let mut buckets: *mut uint32_t = (*self_0).buckets_;
    let mut tags: *mut uint8_t = (*self_0).tags_;
    let cur_ix_masked: size_t = cur_ix & ring_buffer_mask;
    let mut min_score: size_t = (*out).score;
    let mut best_score: size_t = (*out).score;
    let mut best_len: size_t = (*out).len;
    let mut i: size_t = 0;
    let hash: size_t = HashBytesH68(
        data.offset(cur_ix_masked as isize) as *const uint8_t,
        (*self_0).hash_mul_,
    ) as size_t;
    let key: size_t = hash >> TAG_HASH_BITS_0;
    let mut bucket: *mut uint32_t =
        buckets.offset((key << (*self_0).block_bits_) as isize) as *mut uint32_t;
    let mut tag_bucket: *mut uint8_t =
        tags.offset((key << (*self_0).block_bits_) as isize) as *mut uint8_t;
    (*self_0).block_bits_ > 4 as ::core::ffi::c_int;
    (*out).len = 0 as size_t;
    (*out).len_code_delta = 0 as ::core::ffi::c_int;
    i = 0 as size_t;
    while i < (*self_0).num_last_distances_to_check_ as size_t {
        let backward: size_t = *distance_cache.offset(i as isize) as size_t;
        let mut prev_ix: size_t = cur_ix.wrapping_sub(backward);
        if !(prev_ix >= cur_ix) {
            if !((backward > max_backward) as ::core::ffi::c_int as ::core::ffi::c_long != 0) {
                prev_ix = (prev_ix as ::core::ffi::c_ulong
                    & ring_buffer_mask as ::core::ffi::c_ulong) as size_t;
                if cur_ix_masked.wrapping_add(best_len) > ring_buffer_mask {
                    break;
                }
                if !(prev_ix.wrapping_add(best_len) > ring_buffer_mask
                    || *data.offset(cur_ix_masked.wrapping_add(best_len) as isize)
                        as ::core::ffi::c_int
                        != *data.offset(prev_ix.wrapping_add(best_len) as isize)
                            as ::core::ffi::c_int)
                {
                    let len: size_t = FindMatchLengthWithLimit(
                        data.offset(prev_ix as isize) as *const uint8_t,
                        data.offset(cur_ix_masked as isize) as *const uint8_t,
                        max_length,
                    ) as size_t;
                    if len >= 3 as size_t || len == 2 as size_t && i < 2 as size_t {
                        let mut score: size_t = BackwardReferenceScoreUsingLastDistance(len);
                        if best_score < score {
                            if i != 0 as size_t {
                                score = (score as ::core::ffi::c_ulong)
                                    .wrapping_sub(BackwardReferencePenaltyUsingLastDistance(i)
                                        as ::core::ffi::c_ulong)
                                    as size_t as size_t;
                            }
                            if best_score < score {
                                best_score = score;
                                best_len = len;
                                (*out).len = best_len;
                                (*out).distance = backward;
                                (*out).score = best_score;
                            }
                        }
                    }
                }
            }
        }
        i = i.wrapping_add(1);
    }
    if best_len < 3 as size_t {
        best_len = 3 as size_t;
    }
    let tag: uint8_t = (hash & TAG_HASH_MASK_0 as size_t) as uint8_t;
    let first4: uint32_t =
        BrotliUnalignedRead32(data.offset(cur_ix_masked as isize) as *const ::core::ffi::c_void)
            as uint32_t;
    let max_length_m4: size_t = max_length.wrapping_sub(4 as size_t);
    let head: size_t = ((*num.offset(key as isize) as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
        as uint32_t
        & (*self_0).block_mask_) as size_t;
    let mut matches: uint64_t = GetMatchingTagMask(
        (*self_0).block_size_.wrapping_div(16 as size_t),
        tag,
        tag_bucket,
        head,
    );
    let mut n: uint16_t =
        (65535 as ::core::ffi::c_int - *num.offset(key as isize) as ::core::ffi::c_int) as uint16_t;
    let mut block_has_unused_slots: uint64_t =
        ((*self_0).block_size_ > n as size_t) as ::core::ffi::c_int as uint64_t;
    let mut mask: uint64_t = (block_has_unused_slots
        << (n as ::core::ffi::c_int & 64 as ::core::ffi::c_int - 1 as ::core::ffi::c_int))
        .wrapping_sub(1 as uint64_t);
    matches = (matches as ::core::ffi::c_ulong & mask as ::core::ffi::c_ulong) as uint64_t;
    while matches > 0 as uint64_t {
        let rb_index: size_t = head
            .wrapping_add((matches as ::core::ffi::c_ulonglong).trailing_zeros() as i32 as size_t)
            & (*self_0).block_mask_ as size_t;
        let mut prev_ix_0: size_t = *bucket.offset(rb_index as isize) as size_t;
        let mut current4: uint32_t = 0;
        let backward_0: size_t = cur_ix.wrapping_sub(prev_ix_0);
        if (backward_0 > max_backward) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
            break;
        }
        prev_ix_0 = (prev_ix_0 as ::core::ffi::c_ulong & ring_buffer_mask as ::core::ffi::c_ulong)
            as size_t;
        if cur_ix_masked.wrapping_add(best_len) > ring_buffer_mask {
            break;
        }
        if !(prev_ix_0.wrapping_add(best_len) > ring_buffer_mask
            || BrotliUnalignedRead32(
                data.offset(
                    cur_ix_masked
                        .wrapping_add(best_len)
                        .wrapping_sub(3 as size_t) as isize,
                ) as *const uint8_t as *const ::core::ffi::c_void,
            ) != BrotliUnalignedRead32(
                data.offset(prev_ix_0.wrapping_add(best_len).wrapping_sub(3 as size_t) as isize)
                    as *const uint8_t as *const ::core::ffi::c_void,
            ))
        {
            current4 = BrotliUnalignedRead32(
                data.offset(prev_ix_0 as isize) as *const ::core::ffi::c_void
            );
            if !(first4 != current4) {
                let len_0: size_t = (FindMatchLengthWithLimit(
                    data.offset(prev_ix_0.wrapping_add(4 as size_t) as isize) as *const uint8_t,
                    data.offset(cur_ix_masked.wrapping_add(4 as size_t) as isize) as *const uint8_t,
                    max_length_m4,
                ) as size_t)
                    .wrapping_add(4 as size_t);
                let score_0: size_t = BackwardReferenceScore(len_0, backward_0) as size_t;
                if best_score < score_0 {
                    best_score = score_0;
                    best_len = len_0;
                    (*out).len = best_len;
                    (*out).distance = backward_0;
                    (*out).score = best_score;
                }
            }
        }
        matches = (matches as ::core::ffi::c_ulong
            & matches.wrapping_sub(1 as uint64_t) as ::core::ffi::c_ulong)
            as uint64_t;
    }
    *bucket.offset((*num.offset(key as isize) as uint32_t & (*self_0).block_mask_) as isize) =
        cur_ix as uint32_t;
    *tag_bucket.offset((*num.offset(key as isize) as uint32_t & (*self_0).block_mask_) as isize) =
        tag;
    let ref mut fresh7 = *num.offset(key as isize);
    *fresh7 = (*fresh7).wrapping_sub(1);
    if min_score == (*out).score {
        SearchInStaticDictionary(
            dictionary,
            (*self_0).common_,
            data.offset(cur_ix_masked as isize) as *const uint8_t,
            max_length,
            dictionary_distance,
            max_distance,
            out,
            BROTLI_FALSE,
        );
    }
}
