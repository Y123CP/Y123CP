use core::ffi::*;
pub use crate::src::c_inlined_fns::*;
pub use crate::src::ffi::*;
pub use crate::src::c_consts::*;
pub use crate::src::c_structs::*;
pub use crate::src::c_types::*;
pub use crate::src::c_extern_types::*;
extern "C" {
    fn BrotliWipeOutMemoryManager(m: *mut MemoryManager);
    fn CreatePreparedDictionary(
        m: *mut MemoryManager,
        source: *const uint8_t,
        source_size: size_t,
    ) -> *mut PreparedDictionary;
    fn AttachPreparedDictionary(
        compound: *mut CompoundDictionary,
        dictionary: *const PreparedDictionary,
    ) -> c_int;
    fn BrotliInitSharedEncoderDictionary(dict: *mut SharedEncoderDictionary);
    fn BrotliCleanupSharedEncoderDictionary(
        m: *mut MemoryManager,
        dict: *mut SharedEncoderDictionary,
    );
    fn BrotliCreateManagedDictionary(
        alloc_func: brotli_alloc_func,
        free_func: brotli_free_func,
        opaque: *mut c_void,
    ) -> *mut ManagedDictionary;
    fn BrotliDestroyManagedDictionary(dictionary: *mut ManagedDictionary);
    fn BrotliCreateZopfliBackwardReferences(
        m: *mut MemoryManager,
        num_bytes: size_t,
        position: size_t,
        ringbuffer: *const uint8_t,
        ringbuffer_mask: size_t,
        literal_context_lut: ContextLut,
        params: *const BrotliEncoderParams,
        hasher: *mut Hasher,
        dist_cache: *mut c_int,
        last_insert_len: *mut size_t,
        commands: *mut Command,
        num_commands: *mut size_t,
        num_literals: *mut size_t,
    );
    fn BrotliCreateHqZopfliBackwardReferences(
        m: *mut MemoryManager,
        num_bytes: size_t,
        position: size_t,
        ringbuffer: *const uint8_t,
        ringbuffer_mask: size_t,
        literal_context_lut: ContextLut,
        params: *const BrotliEncoderParams,
        hasher: *mut Hasher,
        dist_cache: *mut c_int,
        last_insert_len: *mut size_t,
        commands: *mut Command,
        num_commands: *mut size_t,
        num_literals: *mut size_t,
    );
    fn BrotliCreateBackwardReferences(
        num_bytes: size_t,
        position: size_t,
        ringbuffer: *const uint8_t,
        ringbuffer_mask: size_t,
        literal_context_lut: ContextLut,
        params: *const BrotliEncoderParams,
        hasher: *mut Hasher,
        dist_cache: *mut c_int,
        last_insert_len: *mut size_t,
        commands: *mut Command,
        num_commands: *mut size_t,
        num_literals: *mut size_t,
    );
    fn BrotliInitBlockSplit(self_0: *mut BlockSplit);
    fn BrotliDestroyBlockSplit(m: *mut MemoryManager, self_0: *mut BlockSplit);
    fn BrotliBuildMetaBlock(
        m: *mut MemoryManager,
        ringbuffer: *const uint8_t,
        pos: size_t,
        mask: size_t,
        params: *mut BrotliEncoderParams,
        prev_byte: uint8_t,
        prev_byte2: uint8_t,
        cmds: *mut Command,
        num_commands: size_t,
        literal_context_mode: ContextType,
        mb: *mut MetaBlockSplit,
    );
    fn BrotliBuildMetaBlockGreedy(
        m: *mut MemoryManager,
        ringbuffer: *const uint8_t,
        pos: size_t,
        mask: size_t,
        prev_byte: uint8_t,
        prev_byte2: uint8_t,
        literal_context_lut: ContextLut,
        num_contexts: size_t,
        static_context_map: *const uint32_t,
        commands: *const Command,
        n_commands: size_t,
        mb: *mut MetaBlockSplit,
    );
    fn BrotliOptimizeHistograms(num_distance_codes: uint32_t, mb: *mut MetaBlockSplit);
    fn BrotliInitDistanceParams(
        params: *mut BrotliDistanceParams,
        npostfix: uint32_t,
        ndirect: uint32_t,
        large_window: c_int,
    );
    fn BrotliStoreMetaBlock(
        m: *mut MemoryManager,
        input: *const uint8_t,
        start_pos: size_t,
        length: size_t,
        mask: size_t,
        prev_byte: uint8_t,
        prev_byte2: uint8_t,
        is_last: c_int,
        params: *const BrotliEncoderParams,
        literal_context_mode: ContextType,
        commands: *const Command,
        n_commands: size_t,
        mb: *const MetaBlockSplit,
        storage_ix: *mut size_t,
        storage: *mut uint8_t,
    );
    fn BrotliStoreMetaBlockTrivial(
        m: *mut MemoryManager,
        input: *const uint8_t,
        start_pos: size_t,
        length: size_t,
        mask: size_t,
        is_last: c_int,
        params: *const BrotliEncoderParams,
        commands: *const Command,
        n_commands: size_t,
        storage_ix: *mut size_t,
        storage: *mut uint8_t,
    );
    fn BrotliStoreMetaBlockFast(
        m: *mut MemoryManager,
        input: *const uint8_t,
        start_pos: size_t,
        length: size_t,
        mask: size_t,
        is_last: c_int,
        params: *const BrotliEncoderParams,
        commands: *const Command,
        n_commands: size_t,
        storage_ix: *mut size_t,
        storage: *mut uint8_t,
    );
    fn BrotliStoreUncompressedMetaBlock(
        is_final_block: c_int,
        input: *const uint8_t,
        position: size_t,
        mask: size_t,
        len: size_t,
        storage_ix: *mut size_t,
        storage: *mut uint8_t,
    );
    fn BrotliEncoderEnsureStaticInit() -> c_int;
    fn BrotliCompressFragmentFast(
        s: *mut BrotliOnePassArena,
        input: *const uint8_t,
        input_size: size_t,
        is_last: c_int,
        table: *mut c_int,
        table_size: size_t,
        storage_ix: *mut size_t,
        storage: *mut uint8_t,
    );
    fn BrotliCompressFragmentTwoPass(
        s: *mut BrotliTwoPassArena,
        input: *const uint8_t,
        input_size: size_t,
        is_last: c_int,
        command_buf: *mut uint32_t,
        literal_buf: *mut uint8_t,
        table: *mut c_int,
        table_size: size_t,
        storage_ix: *mut size_t,
        storage: *mut uint8_t,
    );
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct BrotliEncoderStateStruct {
    pub params: BrotliEncoderParams,
    pub memory_manager_: MemoryManager,
    pub input_pos_: uint64_t,
    pub ringbuffer_: RingBuffer,
    pub cmd_alloc_size_: size_t,
    pub commands_: *mut Command,
    pub num_commands_: size_t,
    pub num_literals_: size_t,
    pub last_insert_len_: size_t,
    pub last_flush_pos_: uint64_t,
    pub last_processed_pos_: uint64_t,
    pub dist_cache_: [c_int; 16],
    pub saved_dist_cache_: [c_int; 4],
    pub last_bytes_: uint16_t,
    pub last_bytes_bits_: uint8_t,
    pub flint_: int8_t,
    pub prev_byte_: uint8_t,
    pub prev_byte2_: uint8_t,
    pub storage_size_: size_t,
    pub storage_: *mut uint8_t,
    pub hasher_: Hasher,
    pub small_table_: [c_int; 1024],
    pub large_table_: *mut c_int,
    pub large_table_size_: size_t,
    pub one_pass_arena_: *mut BrotliOnePassArena,
    pub two_pass_arena_: *mut BrotliTwoPassArena,
    pub command_buf_: *mut uint32_t,
    pub literal_buf_: *mut uint8_t,
    pub total_in_: uint64_t,
    pub next_out_: *mut uint8_t,
    pub available_out_: size_t,
    pub total_out_: uint64_t,
    pub tiny_buf_: C2RustUnnamed,
    pub remaining_metadata_bytes_: uint32_t,
    pub stream_state_: BrotliEncoderStreamState,
    pub is_last_block_emitted_: c_int,
    pub is_initialized_: c_int,
}
pub type BrotliEncoderStreamState = c_uint;
pub const BROTLI_STREAM_METADATA_BODY: BrotliEncoderStreamState = 4;
pub const BROTLI_STREAM_METADATA_HEAD: BrotliEncoderStreamState = 3;
pub const BROTLI_STREAM_FINISHED: BrotliEncoderStreamState = 2;
pub const BROTLI_STREAM_FLUSH_REQUESTED: BrotliEncoderStreamState = 1;
pub const BROTLI_STREAM_PROCESSING: BrotliEncoderStreamState = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub u64_0: [uint64_t; 2],
    pub u8_0: [uint8_t; 16],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Hasher {
    pub common: HasherCommon,
    pub privat: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
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
pub struct H65 {
    pub ha: H6,
    pub hb: HROLLING,
    pub ha_common: HasherCommon,
    pub hb_common: HasherCommon,
    pub common: *mut HasherCommon,
    pub fresh: c_int,
    pub params: *const BrotliEncoderParams,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BrotliEncoderParams {
    pub mode: BrotliEncoderMode,
    pub quality: c_int,
    pub lgwin: c_int,
    pub lgblock: c_int,
    pub stream_offset: size_t,
    pub size_hint: size_t,
    pub disable_literal_context_modeling: c_int,
    pub large_window: c_int,
    pub hasher: BrotliHasherParams,
    pub dist: BrotliDistanceParams,
    pub dictionary: SharedEncoderDictionary,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SharedEncoderDictionary {
    pub magic: uint32_t,
    pub compound: CompoundDictionary,
    pub contextual: ContextualEncoderDictionary,
    pub max_quality: c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ContextualEncoderDictionary {
    pub context_based: c_int,
    pub num_dictionaries: uint8_t,
    pub context_map: [uint8_t; 64],
    pub dict: [*const BrotliEncoderDictionary; 64],
    pub num_instances_: size_t,
    pub instance_: BrotliEncoderDictionary,
    pub instances_: *mut BrotliEncoderDictionary,
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
    pub has_words_heavy: c_int,
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
pub struct H55 {
    pub ha: H54,
    pub hb: HROLLING_FAST,
    pub ha_common: HasherCommon,
    pub hb_common: HasherCommon,
    pub common: *mut HasherCommon,
    pub fresh: c_int,
    pub params: *const BrotliEncoderParams,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct H35 {
    pub ha: H3,
    pub hb: HROLLING_FAST,
    pub ha_common: HasherCommon,
    pub hb_common: HasherCommon,
    pub common: *mut HasherCommon,
    pub fresh: c_int,
    pub params: *const BrotliEncoderParams,
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
pub struct RingBuffer {
    pub size_: uint32_t,
    pub mask_: uint32_t,
    pub tail_size_: uint32_t,
    pub total_size_: uint32_t,
    pub cur_size_: uint32_t,
    pub pos_: uint32_t,
    pub data_: *mut uint8_t,
    pub buffer_: *mut uint8_t,
}

pub type BrotliEncoderState = BrotliEncoderStateStruct;
pub type BrotliEncoderStateInternal = BrotliEncoderStateStruct;

pub const BROTLI_FLINT_WAITING_FOR_FLUSHING: BrotliEncoderFlintState = -1;
pub const BROTLI_FLINT_DONE: BrotliEncoderFlintState = -2;
pub const BROTLI_FLINT_NEEDS_2_BYTES: BrotliEncoderFlintState = 2;
pub type BrotliEncoderFlintState = c_int;
pub const BROTLI_FLINT_WAITING_FOR_PROCESSING: BrotliEncoderFlintState = 0;
pub const BROTLI_FLINT_NEEDS_1_BYTE: BrotliEncoderFlintState = 1;

pub const BROTLI_DEFAULT_QUALITY: c_int = 11 as c_int;
pub const BROTLI_DEFAULT_WINDOW: c_int = 22 as c_int;

#[inline(always)]
unsafe extern "C" fn brotli_min_int(
    mut a: c_int,
    mut b: c_int,
) -> c_int {
    return if a < b { a } else { b };
}
#[inline(always)]
unsafe extern "C" fn brotli_max_int(
    mut a: c_int,
    mut b: c_int,
) -> c_int {
    return if a > b { a } else { b };
}

pub const BROTLI_MAX_SIMD_QUALITY: c_int = 6 as c_int;

pub const BROTLI_MAX_NDIRECT: c_int = 120 as c_int;

#[inline(always)]
unsafe extern "C" fn FindMatchLengthWithLimit(
    mut s1: *const uint8_t,
    mut s2: *const uint8_t,
    mut limit: size_t,
) -> size_t {
    let mut s1_orig: *const uint8_t = s1;
    while limit >= 8 as size_t {
        let mut x: uint64_t = BrotliUnalignedRead64(s2 as *const c_void)
            ^ BrotliUnalignedRead64(s1 as *const c_void);
        s2 = s2.offset(8 as c_int as isize);
        if x != 0 as uint64_t {
            let mut matching_bits: size_t =
                (x as c_ulonglong).trailing_zeros() as i32 as size_t;
            return (s1.offset_from(s1_orig) as c_long as size_t)
                .wrapping_add(matching_bits >> 3 as c_int);
        }
        s1 = s1.offset(8 as c_int as isize);
        limit = (limit as c_ulong).wrapping_sub(8 as c_ulong) as size_t
            as size_t;
    }
    while limit != 0 && *s1 as c_int == *s2 as c_int {
        limit = limit.wrapping_sub(1);
        s2 = s2.offset(1);
        s1 = s1.offset(1);
    }
    return s1.offset_from(s1_orig) as c_long as size_t;
}
static mut kHashMul32: uint32_t = 0x1e35a7bd as uint32_t;
static mut kHashMul64: uint64_t = (0x1fe35a7b as c_uint as uint64_t)
    << 32 as c_int
    | 0xd3579bd3 as uint64_t;
pub const BROTLI_ENCODER_MEMORY_MANAGER_SLOTS: c_int = 0 as c_int;
static mut kPreparedDictionaryMagic: uint32_t = 0xdebcede0 as uint32_t;
static mut kSharedDictionaryMagic: uint32_t = 0xdebcede1 as uint32_t;
static mut kManagedDictionaryMagic: uint32_t = 0xdebcede2 as uint32_t;
static mut kLeanPreparedDictionaryMagic: uint32_t = 0xdebcede3 as uint32_t;
#[inline(always)]
unsafe extern "C" fn GetInsertLengthCode(mut insertlen: size_t) -> uint16_t {
    if insertlen < 6 as size_t {
        return insertlen as uint16_t;
    } else if insertlen < 130 as size_t {
        let mut nbits: uint32_t =
            Log2FloorNonZero(insertlen.wrapping_sub(2 as size_t)).wrapping_sub(1 as uint32_t);
        return ((nbits << 1 as c_int) as size_t)
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
        return ((nbits << 1 as c_int) as size_t)
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
unsafe extern "C" fn GetLengthCode(
    mut insertlen: size_t,
    mut copylen: size_t,
    mut use_last_distance: c_int,
    mut code: *mut uint16_t,
) {
    let mut inscode: uint16_t = GetInsertLengthCode(insertlen);
    let mut copycode: uint16_t = GetCopyLengthCode(copylen);
    *code = CombineLengthCodes(inscode, copycode, use_last_distance);
}
#[inline(always)]
unsafe extern "C" fn InitInsertCommand(mut self_0: *mut Command, mut insertlen: size_t) {
    (*self_0).insert_len_ = insertlen as uint32_t;
    (*self_0).copy_len_ = ((4 as c_int) << 25 as c_int) as uint32_t;
    (*self_0).dist_extra_ = 0 as uint32_t;
    (*self_0).dist_prefix_ = BROTLI_NUM_DISTANCE_SHORT_CODES as uint16_t;
    GetLengthCode(
        insertlen,
        4 as size_t,
        BROTLI_FALSE,
        &raw mut (*self_0).cmd_prefix_,
    );
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
            ((*self_0).dist_prefix_ as c_int >> 10 as c_int) as uint32_t;
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
pub const FAST_ONE_PASS_COMPRESSION_QUALITY: c_int = 0 as c_int;
pub const FAST_TWO_PASS_COMPRESSION_QUALITY: c_int = 1 as c_int;
pub const ZOPFLIFICATION_QUALITY: c_int = 10 as c_int;

pub const MAX_QUALITY_FOR_STATIC_ENTROPY_CODES: c_int = 2 as c_int;
pub const MIN_QUALITY_FOR_BLOCK_SPLIT: c_int = 4 as c_int;
pub const MIN_QUALITY_FOR_NONZERO_DISTANCE_PARAMS: c_int = 4 as c_int;
pub const MIN_QUALITY_FOR_OPTIMIZE_HISTOGRAMS: c_int = 4 as c_int;
pub const MIN_QUALITY_FOR_CONTEXT_MODELING: c_int = 5 as c_int;
pub const MIN_QUALITY_FOR_HQ_CONTEXT_MODELING: c_int = 7 as c_int;
pub const MIN_QUALITY_FOR_HQ_BLOCK_SPLITTING: c_int = 10 as c_int;
#[inline(always)]
unsafe extern "C" fn MaxHashTableSize(mut quality: c_int) -> size_t {
    return (if quality == FAST_ONE_PASS_COMPRESSION_QUALITY {
        (1 as c_int) << 15 as c_int
    } else {
        (1 as c_int) << 17 as c_int
    }) as size_t;
}
#[inline(always)]
unsafe extern "C" fn SanitizeParams(mut params: *mut BrotliEncoderParams) {
    (*params).quality = brotli_min_int(
        11 as c_int,
        brotli_max_int(0 as c_int, (*params).quality),
    );
    if (*params).quality <= MAX_QUALITY_FOR_STATIC_ENTROPY_CODES {
        (*params).large_window = BROTLI_FALSE;
    }
    if (*params).lgwin < BROTLI_MIN_WINDOW_BITS {
        (*params).lgwin = BROTLI_MIN_WINDOW_BITS;
    } else {
        let mut max_lgwin: c_int = if (*params).large_window != 0 {
            BROTLI_LARGE_MAX_WINDOW_BITS
        } else {
            BROTLI_MAX_WINDOW_BITS
        };
        if (*params).lgwin > max_lgwin {
            (*params).lgwin = max_lgwin;
        }
    };
}
#[inline(always)]
unsafe extern "C" fn ComputeLgBlock(mut params: *const BrotliEncoderParams) -> c_int {
    let mut lgblock: c_int = (*params).lgblock;
    if (*params).quality == FAST_ONE_PASS_COMPRESSION_QUALITY
        || (*params).quality == FAST_TWO_PASS_COMPRESSION_QUALITY
    {
        lgblock = (*params).lgwin;
    } else if (*params).quality < MIN_QUALITY_FOR_BLOCK_SPLIT {
        lgblock = 14 as c_int;
    } else if lgblock == 0 as c_int {
        lgblock = 16 as c_int;
        if (*params).quality >= 9 as c_int && (*params).lgwin > lgblock {
            lgblock = brotli_min_int(18 as c_int, (*params).lgwin);
        }
    } else {
        lgblock = brotli_min_int(
            24 as c_int,
            brotli_max_int(16 as c_int, lgblock),
        );
    }
    return lgblock;
}
#[inline(always)]
unsafe extern "C" fn ComputeRbBits(mut params: *const BrotliEncoderParams) -> c_int {
    return 1 as c_int + brotli_max_int((*params).lgwin, (*params).lgblock);
}
#[inline(always)]
unsafe extern "C" fn MaxMetablockSize(mut params: *const BrotliEncoderParams) -> size_t {
    let mut bits: c_int =
        brotli_min_int(ComputeRbBits(params), 24 as c_int);
    return (1 as c_int as size_t) << bits;
}
#[inline(always)]
unsafe extern "C" fn ChooseHasher(
    mut params: *const BrotliEncoderParams,
    mut hparams: *mut BrotliHasherParams,
) {
    if (*params).quality > 9 as c_int {
        (*hparams).type_0 = 10 as c_int;
    } else if (*params).quality == 4 as c_int
        && (*params).size_hint >= ((1 as c_int) << 20 as c_int) as size_t
    {
        (*hparams).type_0 = 54 as c_int;
    } else if (*params).quality < 5 as c_int {
        (*hparams).type_0 = (*params).quality;
    } else if (*params).lgwin <= 16 as c_int {
        (*hparams).type_0 = if (*params).quality < 7 as c_int {
            40 as c_int
        } else if (*params).quality < 9 as c_int {
            41 as c_int
        } else {
            42 as c_int
        };
    } else if (*params).size_hint
        >= ((1 as c_int) << 20 as c_int) as size_t
        && (*params).lgwin >= 19 as c_int
    {
        (*hparams).type_0 = if (*params).quality <= BROTLI_MAX_SIMD_QUALITY {
            68 as c_int
        } else {
            6 as c_int
        };
        (*hparams).block_bits = (*params).quality - 1 as c_int;
        (*hparams).bucket_bits = 15 as c_int;
        (*hparams).num_last_distances_to_check = if (*params).quality < 7 as c_int {
            4 as c_int
        } else if (*params).quality < 9 as c_int {
            10 as c_int
        } else {
            16 as c_int
        };
    } else {
        (*hparams).type_0 = if (*params).quality <= BROTLI_MAX_SIMD_QUALITY {
            58 as c_int
        } else {
            5 as c_int
        };
        (*hparams).block_bits = (*params).quality - 1 as c_int;
        (*hparams).bucket_bits = if (*params).quality < 7 as c_int {
            14 as c_int
        } else {
            15 as c_int
        };
        (*hparams).num_last_distances_to_check = if (*params).quality < 7 as c_int {
            4 as c_int
        } else if (*params).quality < 9 as c_int {
            10 as c_int
        } else {
            16 as c_int
        };
    }
    if (*params).lgwin > 24 as c_int {
        if (*hparams).type_0 == 3 as c_int {
            (*hparams).type_0 = 35 as c_int;
        }
        if (*hparams).type_0 == 54 as c_int {
            (*hparams).type_0 = 55 as c_int;
        }
        if (*hparams).type_0 == 6 as c_int
            || (*hparams).type_0 == 68 as c_int
        {
            (*hparams).type_0 = 65 as c_int;
        }
    }
}
pub const BROTLI_MAX_STATIC_CONTEXTS: c_int = 13 as c_int;
#[inline(always)]
unsafe extern "C" fn InitMetaBlockSplit(mut mb: *mut MetaBlockSplit) {
    BrotliInitBlockSplit(&raw mut (*mb).literal_split);
    BrotliInitBlockSplit(&raw mut (*mb).command_split);
    BrotliInitBlockSplit(&raw mut (*mb).distance_split);
    (*mb).literal_context_map = ::core::ptr::null_mut::<uint32_t>();
    (*mb).literal_context_map_size = 0 as size_t;
    (*mb).distance_context_map = ::core::ptr::null_mut::<uint32_t>();
    (*mb).distance_context_map_size = 0 as size_t;
    (*mb).literal_histograms = ::core::ptr::null_mut::<HistogramLiteral>();
    (*mb).literal_histograms_size = 0 as size_t;
    (*mb).command_histograms = ::core::ptr::null_mut::<HistogramCommand>();
    (*mb).command_histograms_size = 0 as size_t;
    (*mb).distance_histograms = ::core::ptr::null_mut::<HistogramDistance>();
    (*mb).distance_histograms_size = 0 as size_t;
}
#[inline(always)]
unsafe extern "C" fn DestroyMetaBlockSplit(mut m: *mut MemoryManager, mut mb: *mut MetaBlockSplit) {
    BrotliDestroyBlockSplit(m, &raw mut (*mb).literal_split);
    BrotliDestroyBlockSplit(m, &raw mut (*mb).command_split);
    BrotliDestroyBlockSplit(m, &raw mut (*mb).distance_split);
    BrotliFree(m, (*mb).literal_context_map as *mut c_void);
    (*mb).literal_context_map = ::core::ptr::null_mut::<uint32_t>();
    BrotliFree(m, (*mb).distance_context_map as *mut c_void);
    (*mb).distance_context_map = ::core::ptr::null_mut::<uint32_t>();
    BrotliFree(m, (*mb).literal_histograms as *mut c_void);
    (*mb).literal_histograms = ::core::ptr::null_mut::<HistogramLiteral>();
    BrotliFree(m, (*mb).command_histograms as *mut c_void);
    (*mb).command_histograms = ::core::ptr::null_mut::<HistogramCommand>();
    BrotliFree(m, (*mb).distance_histograms as *mut c_void);
    (*mb).distance_histograms = ::core::ptr::null_mut::<HistogramDistance>();
}
static mut kMinUTF8Ratio: c_double = 0.75f64;
#[inline(always)]
unsafe extern "C" fn BrotliWriteBits(
    mut n_bits: size_t,
    mut bits: uint64_t,
    mut pos: *mut size_t,
    mut array: *mut uint8_t,
) {
    let mut p: *mut uint8_t =
        array.offset((*pos >> 3 as c_int) as isize) as *mut uint8_t;
    let mut v: uint64_t = *p as uint64_t;
    v = (v as c_ulong | (bits << (*pos & 7 as size_t)) as c_ulong)
        as uint64_t;
    BrotliUnalignedWrite64(p as *mut c_void, v);
    *pos = (*pos as c_ulong).wrapping_add(n_bits as c_ulong) as size_t
        as size_t;
}

pub const BUCKET_BITS: c_int = 17 as c_int;

pub const BUCKET_BITS_1: c_int = 16 as c_int;
pub const BUCKET_SWEEP_BITS_2: c_int = 0 as c_int;
pub const HASH_LEN_0: c_int = 5 as c_int;
pub const BUCKET_SIZE_5: c_int = (1 as c_int) << BUCKET_BITS_1;
pub const BUCKET_MASK_2: c_int = BUCKET_SIZE_5 - 1 as c_int;
pub const BUCKET_SWEEP_2: c_int = (1 as c_int) << BUCKET_SWEEP_BITS_2;
pub const BUCKET_SWEEP_MASK_2: c_int =
    (BUCKET_SWEEP_2 - 1 as c_int) << 3 as c_int;

unsafe extern "C" fn HashBytesH2(mut data: *const uint8_t) -> uint32_t {
    let h: uint64_t = ((BrotliUnalignedRead64(data as *const c_void) as uint64_t)
        << 64 as c_int - 8 as c_int * HASH_LEN_0)
        .wrapping_mul(kHashMul64);
    return (h >> 64 as c_int - BUCKET_BITS_1) as uint32_t;
}
unsafe extern "C" fn InitializeH2(
    mut common: *mut HasherCommon,
    mut self_0: *mut H2,
    mut params: *const BrotliEncoderParams,
) {
    (*self_0).common = common;
    (*self_0).buckets_ = (*common).extra[0 as c_int as usize] as *mut uint32_t;
}
unsafe extern "C" fn PrepareH2(
    mut self_0: *mut H2,
    mut one_shot: c_int,
    mut input_size: size_t,
    mut data: *const uint8_t,
) {
    let mut buckets: *mut uint32_t = (*self_0).buckets_;
    let mut partial_prepare_threshold: size_t =
        (BUCKET_SIZE_5 >> 5 as c_int) as size_t;
    if one_shot != 0 && input_size <= partial_prepare_threshold {
        let mut i: size_t = 0;
        i = 0 as size_t;
        while i < input_size {
            let key: uint32_t = HashBytesH2(data.offset(i as isize) as *const uint8_t) as uint32_t;
            if BUCKET_SWEEP_2 == 1 as c_int {
                *buckets.offset(key as isize) = 0 as uint32_t;
            } else {
                let mut j: uint32_t = 0;
                j = 0 as uint32_t;
                while j < BUCKET_SWEEP_2 as uint32_t {
                    *buckets.offset(
                        (key.wrapping_add(j << 3 as c_int) & BUCKET_MASK_2 as uint32_t)
                            as isize,
                    ) = 0 as uint32_t;
                    j = j.wrapping_add(1);
                }
            }
            i = i.wrapping_add(1);
        }
    } else {
        memset(
            buckets as *mut c_void,
            0 as c_int,
            (::core::mem::size_of::<uint32_t>() as size_t).wrapping_mul(BUCKET_SIZE_5 as size_t),
        );
    };
}
#[inline(always)]
unsafe extern "C" fn HashMemAllocInBytesH2(
    mut params: *const BrotliEncoderParams,
    mut one_shot: c_int,
    mut input_size: size_t,
    mut alloc_size: *mut size_t,
) {
    *alloc_size.offset(0 as c_int as isize) =
        (::core::mem::size_of::<uint32_t>() as usize).wrapping_mul(BUCKET_SIZE_5 as usize)
            as size_t;
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
    if BUCKET_SWEEP_2 == 1 as c_int {
        *(*self_0).buckets_.offset(key as isize) = ix as uint32_t;
    } else {
        let off: uint32_t = (ix & BUCKET_SWEEP_MASK_2 as size_t) as uint32_t;
        *(*self_0)
            .buckets_
            .offset((key.wrapping_add(off) & BUCKET_MASK_2 as uint32_t) as isize) = ix as uint32_t;
    };
}
#[inline(always)]
unsafe extern "C" fn StitchToPreviousBlockH2(
    mut self_0: *mut H2,
    mut num_bytes: size_t,
    mut position: size_t,
    mut ringbuffer: *const uint8_t,
    mut ringbuffer_mask: size_t,
) {
    if num_bytes >= HashTypeLengthH2().wrapping_sub(1 as size_t) && position >= 3 as size_t {
        StoreH2(
            self_0,
            ringbuffer,
            ringbuffer_mask,
            position.wrapping_sub(3 as size_t),
        );
        StoreH2(
            self_0,
            ringbuffer,
            ringbuffer_mask,
            position.wrapping_sub(2 as size_t),
        );
        StoreH2(
            self_0,
            ringbuffer,
            ringbuffer_mask,
            position.wrapping_sub(1 as size_t),
        );
    }
}
pub const BUCKET_SIZE_0: c_int = (1 as c_int) << BUCKET_BITS_1;
pub const BUCKET_MASK_0: c_int = BUCKET_SIZE_0 - 1 as c_int;

unsafe extern "C" fn HashBytesH3(mut data: *const uint8_t) -> uint32_t {
    let h: uint64_t = ((BrotliUnalignedRead64(data as *const c_void) as uint64_t)
        << 64 as c_int - 8 as c_int * HASH_LEN_0)
        .wrapping_mul(kHashMul64);
    return (h >> 64 as c_int - BUCKET_BITS_1) as uint32_t;
}
unsafe extern "C" fn InitializeH3(
    mut common: *mut HasherCommon,
    mut self_0: *mut H3,
    mut params: *const BrotliEncoderParams,
) {
    (*self_0).common = common;
    (*self_0).buckets_ = (*common).extra[0 as c_int as usize] as *mut uint32_t;
}
unsafe extern "C" fn PrepareH3(
    mut self_0: *mut H3,
    mut one_shot: c_int,
    mut input_size: size_t,
    mut data: *const uint8_t,
) {
    let mut buckets: *mut uint32_t = (*self_0).buckets_;
    let mut partial_prepare_threshold: size_t =
        (BUCKET_SIZE_0 >> 5 as c_int) as size_t;
    if one_shot != 0 && input_size <= partial_prepare_threshold {
        let mut i: size_t = 0;
        i = 0 as size_t;
        while i < input_size {
            let key: uint32_t = HashBytesH3(data.offset(i as isize) as *const uint8_t) as uint32_t;
            if BUCKET_SWEEP_0 == 1 as c_int {
                *buckets.offset(key as isize) = 0 as uint32_t;
            } else {
                let mut j: uint32_t = 0;
                j = 0 as uint32_t;
                while j < BUCKET_SWEEP_0 as uint32_t {
                    *buckets.offset(
                        (key.wrapping_add(j << 3 as c_int) & BUCKET_MASK_0 as uint32_t)
                            as isize,
                    ) = 0 as uint32_t;
                    j = j.wrapping_add(1);
                }
            }
            i = i.wrapping_add(1);
        }
    } else {
        memset(
            buckets as *mut c_void,
            0 as c_int,
            (::core::mem::size_of::<uint32_t>() as size_t).wrapping_mul(BUCKET_SIZE_0 as size_t),
        );
    };
}
#[inline(always)]
unsafe extern "C" fn HashMemAllocInBytesH3(
    mut params: *const BrotliEncoderParams,
    mut one_shot: c_int,
    mut input_size: size_t,
    mut alloc_size: *mut size_t,
) {
    *alloc_size.offset(0 as c_int as isize) =
        (::core::mem::size_of::<uint32_t>() as usize).wrapping_mul(BUCKET_SIZE_0 as usize)
            as size_t;
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
    if BUCKET_SWEEP_0 == 1 as c_int {
        *(*self_0).buckets_.offset(key as isize) = ix as uint32_t;
    } else {
        let off: uint32_t = (ix & BUCKET_SWEEP_MASK_0 as size_t) as uint32_t;
        *(*self_0)
            .buckets_
            .offset((key.wrapping_add(off) & BUCKET_MASK_0 as uint32_t) as isize) = ix as uint32_t;
    };
}
#[inline(always)]
unsafe extern "C" fn StitchToPreviousBlockH3(
    mut self_0: *mut H3,
    mut num_bytes: size_t,
    mut position: size_t,
    mut ringbuffer: *const uint8_t,
    mut ringbuffer_mask: size_t,
) {
    if num_bytes >= HashTypeLengthH3().wrapping_sub(1 as size_t) && position >= 3 as size_t {
        StoreH3(
            self_0,
            ringbuffer,
            ringbuffer_mask,
            position.wrapping_sub(3 as size_t),
        );
        StoreH3(
            self_0,
            ringbuffer,
            ringbuffer_mask,
            position.wrapping_sub(2 as size_t),
        );
        StoreH3(
            self_0,
            ringbuffer,
            ringbuffer_mask,
            position.wrapping_sub(1 as size_t),
        );
    }
}
pub const BUCKET_SIZE_4: c_int = (1 as c_int) << BUCKET_BITS_3;
pub const BUCKET_MASK_1: c_int = BUCKET_SIZE_4 - 1 as c_int;

unsafe extern "C" fn HashBytesH4(mut data: *const uint8_t) -> uint32_t {
    let h: uint64_t = ((BrotliUnalignedRead64(data as *const c_void) as uint64_t)
        << 64 as c_int - 8 as c_int * HASH_LEN_0)
        .wrapping_mul(kHashMul64);
    return (h >> 64 as c_int - BUCKET_BITS_3) as uint32_t;
}
unsafe extern "C" fn InitializeH4(
    mut common: *mut HasherCommon,
    mut self_0: *mut H4,
    mut params: *const BrotliEncoderParams,
) {
    (*self_0).common = common;
    (*self_0).buckets_ = (*common).extra[0 as c_int as usize] as *mut uint32_t;
}
unsafe extern "C" fn PrepareH4(
    mut self_0: *mut H4,
    mut one_shot: c_int,
    mut input_size: size_t,
    mut data: *const uint8_t,
) {
    let mut buckets: *mut uint32_t = (*self_0).buckets_;
    let mut partial_prepare_threshold: size_t =
        (BUCKET_SIZE_4 >> 5 as c_int) as size_t;
    if one_shot != 0 && input_size <= partial_prepare_threshold {
        let mut i: size_t = 0;
        i = 0 as size_t;
        while i < input_size {
            let key: uint32_t = HashBytesH4(data.offset(i as isize) as *const uint8_t) as uint32_t;
            if BUCKET_SWEEP_1 == 1 as c_int {
                *buckets.offset(key as isize) = 0 as uint32_t;
            } else {
                let mut j: uint32_t = 0;
                j = 0 as uint32_t;
                while j < BUCKET_SWEEP_1 as uint32_t {
                    *buckets.offset(
                        (key.wrapping_add(j << 3 as c_int) & BUCKET_MASK_1 as uint32_t)
                            as isize,
                    ) = 0 as uint32_t;
                    j = j.wrapping_add(1);
                }
            }
            i = i.wrapping_add(1);
        }
    } else {
        memset(
            buckets as *mut c_void,
            0 as c_int,
            (::core::mem::size_of::<uint32_t>() as size_t).wrapping_mul(BUCKET_SIZE_4 as size_t),
        );
    };
}
#[inline(always)]
unsafe extern "C" fn HashMemAllocInBytesH4(
    mut params: *const BrotliEncoderParams,
    mut one_shot: c_int,
    mut input_size: size_t,
    mut alloc_size: *mut size_t,
) {
    *alloc_size.offset(0 as c_int as isize) =
        (::core::mem::size_of::<uint32_t>() as usize).wrapping_mul(BUCKET_SIZE_4 as usize)
            as size_t;
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
    if BUCKET_SWEEP_1 == 1 as c_int {
        *(*self_0).buckets_.offset(key as isize) = ix as uint32_t;
    } else {
        let off: uint32_t = (ix & BUCKET_SWEEP_MASK_1 as size_t) as uint32_t;
        *(*self_0)
            .buckets_
            .offset((key.wrapping_add(off) & BUCKET_MASK_1 as uint32_t) as isize) = ix as uint32_t;
    };
}
#[inline(always)]
unsafe extern "C" fn StitchToPreviousBlockH4(
    mut self_0: *mut H4,
    mut num_bytes: size_t,
    mut position: size_t,
    mut ringbuffer: *const uint8_t,
    mut ringbuffer_mask: size_t,
) {
    if num_bytes >= HashTypeLengthH4().wrapping_sub(1 as size_t) && position >= 3 as size_t {
        StoreH4(
            self_0,
            ringbuffer,
            ringbuffer_mask,
            position.wrapping_sub(3 as size_t),
        );
        StoreH4(
            self_0,
            ringbuffer,
            ringbuffer_mask,
            position.wrapping_sub(2 as size_t),
        );
        StoreH4(
            self_0,
            ringbuffer,
            ringbuffer_mask,
            position.wrapping_sub(1 as size_t),
        );
    }
}
pub const BUCKET_SIZE: c_int = (1 as c_int) << BUCKET_BITS_0;
pub const BUCKET_MASK: c_int = BUCKET_SIZE - 1 as c_int;
pub const BUCKET_SWEEP: c_int = (1 as c_int) << BUCKET_SWEEP_BITS;
pub const BUCKET_SWEEP_MASK: c_int =
    (BUCKET_SWEEP - 1 as c_int) << 3 as c_int;

unsafe extern "C" fn HashBytesH54(mut data: *const uint8_t) -> uint32_t {
    let h: uint64_t = ((BrotliUnalignedRead64(data as *const c_void) as uint64_t)
        << 64 as c_int - 8 as c_int * HASH_LEN)
        .wrapping_mul(kHashMul64);
    return (h >> 64 as c_int - BUCKET_BITS_0) as uint32_t;
}
unsafe extern "C" fn InitializeH54(
    mut common: *mut HasherCommon,
    mut self_0: *mut H54,
    mut params: *const BrotliEncoderParams,
) {
    (*self_0).common = common;
    (*self_0).buckets_ = (*common).extra[0 as c_int as usize] as *mut uint32_t;
}
unsafe extern "C" fn PrepareH54(
    mut self_0: *mut H54,
    mut one_shot: c_int,
    mut input_size: size_t,
    mut data: *const uint8_t,
) {
    let mut buckets: *mut uint32_t = (*self_0).buckets_;
    let mut partial_prepare_threshold: size_t = (BUCKET_SIZE >> 5 as c_int) as size_t;
    if one_shot != 0 && input_size <= partial_prepare_threshold {
        let mut i: size_t = 0;
        i = 0 as size_t;
        while i < input_size {
            let key: uint32_t = HashBytesH54(data.offset(i as isize) as *const uint8_t) as uint32_t;
            if BUCKET_SWEEP == 1 as c_int {
                *buckets.offset(key as isize) = 0 as uint32_t;
            } else {
                let mut j: uint32_t = 0;
                j = 0 as uint32_t;
                while j < BUCKET_SWEEP as uint32_t {
                    *buckets.offset(
                        (key.wrapping_add(j << 3 as c_int) & BUCKET_MASK as uint32_t)
                            as isize,
                    ) = 0 as uint32_t;
                    j = j.wrapping_add(1);
                }
            }
            i = i.wrapping_add(1);
        }
    } else {
        memset(
            buckets as *mut c_void,
            0 as c_int,
            (::core::mem::size_of::<uint32_t>() as size_t).wrapping_mul(BUCKET_SIZE as size_t),
        );
    };
}
#[inline(always)]
unsafe extern "C" fn HashMemAllocInBytesH54(
    mut params: *const BrotliEncoderParams,
    mut one_shot: c_int,
    mut input_size: size_t,
    mut alloc_size: *mut size_t,
) {
    *alloc_size.offset(0 as c_int as isize) =
        (::core::mem::size_of::<uint32_t>() as usize).wrapping_mul(BUCKET_SIZE as usize) as size_t;
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
    if BUCKET_SWEEP == 1 as c_int {
        *(*self_0).buckets_.offset(key as isize) = ix as uint32_t;
    } else {
        let off: uint32_t = (ix & BUCKET_SWEEP_MASK as size_t) as uint32_t;
        *(*self_0)
            .buckets_
            .offset((key.wrapping_add(off) & BUCKET_MASK as uint32_t) as isize) = ix as uint32_t;
    };
}
#[inline(always)]
unsafe extern "C" fn StitchToPreviousBlockH54(
    mut self_0: *mut H54,
    mut num_bytes: size_t,
    mut position: size_t,
    mut ringbuffer: *const uint8_t,
    mut ringbuffer_mask: size_t,
) {
    if num_bytes >= HashTypeLengthH54().wrapping_sub(1 as size_t) && position >= 3 as size_t {
        StoreH54(
            self_0,
            ringbuffer,
            ringbuffer_mask,
            position.wrapping_sub(3 as size_t),
        );
        StoreH54(
            self_0,
            ringbuffer,
            ringbuffer_mask,
            position.wrapping_sub(2 as size_t),
        );
        StoreH54(
            self_0,
            ringbuffer,
            ringbuffer_mask,
            position.wrapping_sub(1 as size_t),
        );
    }
}

unsafe extern "C" fn HashBytesH5(mut data: *const uint8_t, shift: c_int) -> uint32_t {
    let mut h: uint32_t =
        BrotliUnalignedRead32(data as *const c_void).wrapping_mul(kHashMul32);
    return h >> shift;
}
unsafe extern "C" fn InitializeH5(
    mut common: *mut HasherCommon,
    mut self_0: *mut H5,
    mut params: *const BrotliEncoderParams,
) {
    (*self_0).common_ = common;
    (*self_0).hash_shift_ = 32 as c_int - (*common).params.bucket_bits;
    (*self_0).bucket_size_ = (1 as c_int as size_t) << (*common).params.bucket_bits;
    (*self_0).block_size_ = (1 as c_int as size_t) << (*common).params.block_bits;
    (*self_0).block_mask_ = (*self_0).block_size_.wrapping_sub(1 as size_t) as uint32_t;
    (*self_0).num_ = (*common).extra[0 as c_int as usize] as *mut uint16_t;
    (*self_0).buckets_ = (*common).extra[1 as c_int as usize] as *mut uint32_t;
    (*self_0).block_bits_ = (*common).params.block_bits;
    (*self_0).num_last_distances_to_check_ = (*common).params.num_last_distances_to_check;
}
unsafe extern "C" fn PrepareH5(
    mut self_0: *mut H5,
    mut one_shot: c_int,
    mut input_size: size_t,
    mut data: *const uint8_t,
) {
    let mut num: *mut uint16_t = (*self_0).num_;
    let mut partial_prepare_threshold: size_t = (*self_0).bucket_size_ >> 6 as c_int;
    if one_shot != 0 && input_size <= partial_prepare_threshold {
        let mut i: size_t = 0;
        i = 0 as size_t;
        while i < input_size {
            let key: uint32_t = HashBytesH5(
                data.offset(i as isize) as *const uint8_t,
                (*self_0).hash_shift_,
            ) as uint32_t;
            *num.offset(key as isize) = 0 as uint16_t;
            i = i.wrapping_add(1);
        }
    } else {
        memset(
            num as *mut c_void,
            0 as c_int,
            (*self_0)
                .bucket_size_
                .wrapping_mul(::core::mem::size_of::<uint16_t>() as size_t),
        );
    };
}
#[inline(always)]
unsafe extern "C" fn HashMemAllocInBytesH5(
    mut params: *const BrotliEncoderParams,
    mut one_shot: c_int,
    mut input_size: size_t,
    mut alloc_size: *mut size_t,
) {
    let mut bucket_size: size_t =
        (1 as c_int as size_t) << (*params).hasher.bucket_bits;
    let mut block_size: size_t = (1 as c_int as size_t) << (*params).hasher.block_bits;
    *alloc_size.offset(0 as c_int as isize) =
        (::core::mem::size_of::<uint16_t>() as usize).wrapping_mul(bucket_size as usize) as size_t;
    *alloc_size.offset(1 as c_int as isize) =
        (::core::mem::size_of::<uint32_t>() as usize)
            .wrapping_mul(bucket_size as usize)
            .wrapping_mul(block_size as usize) as size_t;
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
    let ref mut fresh18 = *num.offset(key as isize);
    *fresh18 = (*fresh18).wrapping_add(1);
    *buckets.offset(offset as isize) = ix as uint32_t;
}
#[inline(always)]
unsafe extern "C" fn StitchToPreviousBlockH5(
    mut self_0: *mut H5,
    mut num_bytes: size_t,
    mut position: size_t,
    mut ringbuffer: *const uint8_t,
    mut ringbuffer_mask: size_t,
) {
    if num_bytes >= HashTypeLengthH5().wrapping_sub(1 as size_t) && position >= 3 as size_t {
        StoreH5(
            self_0,
            ringbuffer,
            ringbuffer_mask,
            position.wrapping_sub(3 as size_t),
        );
        StoreH5(
            self_0,
            ringbuffer,
            ringbuffer_mask,
            position.wrapping_sub(2 as size_t),
        );
        StoreH5(
            self_0,
            ringbuffer,
            ringbuffer_mask,
            position.wrapping_sub(1 as size_t),
        );
    }
}
pub const BUCKET_BITS_3: c_int = 17 as c_int;

#[inline(always)]
unsafe extern "C" fn HashBytesH6(mut data: *const uint8_t, mut hash_mul: uint64_t) -> size_t {
    let h: uint64_t = (BrotliUnalignedRead64(data as *const c_void) as uint64_t)
        .wrapping_mul(hash_mul);
    return (h >> 64 as c_int - 15 as c_int) as size_t;
}
unsafe extern "C" fn InitializeH6(
    mut common: *mut HasherCommon,
    mut self_0: *mut H6,
    mut params: *const BrotliEncoderParams,
) {
    (*self_0).common_ = common;
    (*self_0).hash_mul_ =
        kHashMul64 << 64 as c_int - 5 as c_int * 8 as c_int;
    (*self_0).bucket_size_ = (1 as c_int as size_t) << (*common).params.bucket_bits;
    (*self_0).block_bits_ = (*common).params.block_bits;
    (*self_0).block_size_ = (1 as c_int as size_t) << (*common).params.block_bits;
    (*self_0).block_mask_ = (*self_0).block_size_.wrapping_sub(1 as size_t) as uint32_t;
    (*self_0).num_last_distances_to_check_ = (*common).params.num_last_distances_to_check;
    (*self_0).num_ = (*common).extra[0 as c_int as usize] as *mut uint16_t;
    (*self_0).buckets_ = (*common).extra[1 as c_int as usize] as *mut uint32_t;
}
unsafe extern "C" fn PrepareH6(
    mut self_0: *mut H6,
    mut one_shot: c_int,
    mut input_size: size_t,
    mut data: *const uint8_t,
) {
    let mut num: *mut uint16_t = (*self_0).num_;
    let mut partial_prepare_threshold: size_t = (*self_0).bucket_size_ >> 6 as c_int;
    if one_shot != 0 && input_size <= partial_prepare_threshold {
        let mut i: size_t = 0;
        i = 0 as size_t;
        while i < input_size {
            let key: size_t = HashBytesH6(
                data.offset(i as isize) as *const uint8_t,
                (*self_0).hash_mul_,
            ) as size_t;
            *num.offset(key as isize) = 0 as uint16_t;
            i = i.wrapping_add(1);
        }
    } else {
        memset(
            num as *mut c_void,
            0 as c_int,
            (*self_0)
                .bucket_size_
                .wrapping_mul(::core::mem::size_of::<uint16_t>() as size_t),
        );
    };
}
#[inline(always)]
unsafe extern "C" fn HashMemAllocInBytesH6(
    mut params: *const BrotliEncoderParams,
    mut one_shot: c_int,
    mut input_size: size_t,
    mut alloc_size: *mut size_t,
) {
    let mut bucket_size: size_t =
        (1 as c_int as size_t) << (*params).hasher.bucket_bits;
    let mut block_size: size_t = (1 as c_int as size_t) << (*params).hasher.block_bits;
    *alloc_size.offset(0 as c_int as isize) =
        (::core::mem::size_of::<uint16_t>() as usize).wrapping_mul(bucket_size as usize) as size_t;
    *alloc_size.offset(1 as c_int as isize) =
        (::core::mem::size_of::<uint32_t>() as usize)
            .wrapping_mul(bucket_size as usize)
            .wrapping_mul(block_size as usize) as size_t;
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
    let ref mut fresh12 = *num.offset(key as isize);
    *fresh12 = (*fresh12).wrapping_add(1);
    *buckets.offset(offset as isize) = ix as uint32_t;
}
#[inline(always)]
unsafe extern "C" fn StitchToPreviousBlockH6(
    mut self_0: *mut H6,
    mut num_bytes: size_t,
    mut position: size_t,
    mut ringbuffer: *const uint8_t,
    mut ringbuffer_mask: size_t,
) {
    if num_bytes >= HashTypeLengthH6().wrapping_sub(1 as size_t) && position >= 3 as size_t {
        StoreH6(
            self_0,
            ringbuffer,
            ringbuffer_mask,
            position.wrapping_sub(3 as size_t),
        );
        StoreH6(
            self_0,
            ringbuffer,
            ringbuffer_mask,
            position.wrapping_sub(2 as size_t),
        );
        StoreH6(
            self_0,
            ringbuffer,
            ringbuffer_mask,
            position.wrapping_sub(1 as size_t),
        );
    }
}
pub const BANK_SIZE_1: c_int = (1 as c_int) << BANK_BITS_0;
pub const BUCKET_SIZE_3: c_int = (1 as c_int) << BUCKET_BITS_2;

#[inline(always)]
unsafe extern "C" fn HashBytesH40(mut data: *const uint8_t) -> size_t {
    let h: uint32_t = (BrotliUnalignedRead32(data as *const c_void) as uint32_t)
        .wrapping_mul(kHashMul32);
    return (h >> 32 as c_int - BUCKET_BITS_2) as size_t;
}

unsafe extern "C" fn HeadH40(mut extra: *mut c_void) -> *mut uint16_t {
    return (AddrH40 as unsafe extern "C" fn(*mut c_void) -> *mut uint32_t)(extra)
        .offset(BUCKET_SIZE_3 as isize) as *mut uint32_t as *mut uint16_t;
}
unsafe extern "C" fn TinyHashH40(mut extra: *mut c_void) -> *mut uint8_t {
    return (HeadH40 as unsafe extern "C" fn(*mut c_void) -> *mut uint16_t)(extra)
        .offset(BUCKET_SIZE_3 as isize) as *mut uint16_t as *mut uint8_t;
}

unsafe extern "C" fn InitializeH40(
    mut common: *mut HasherCommon,
    mut self_0: *mut H40,
    mut params: *const BrotliEncoderParams,
) {
    (*self_0).common = common;
    (*self_0).extra[0 as c_int as usize] =
        (*common).extra[0 as c_int as usize];
    (*self_0).extra[1 as c_int as usize] =
        (*common).extra[1 as c_int as usize];
    (*self_0).max_hops = ((if (*params).quality > 6 as c_int {
        7 as c_uint
    } else {
        8 as c_uint
    }) << (*params).quality - 4 as c_int) as size_t;
}
unsafe extern "C" fn PrepareH40(
    mut self_0: *mut H40,
    mut one_shot: c_int,
    mut input_size: size_t,
    mut data: *const uint8_t,
) {
    let mut addr: *mut uint32_t =
        AddrH40((*self_0).extra[0 as c_int as usize]) as *mut uint32_t;
    let mut head: *mut uint16_t =
        HeadH40((*self_0).extra[0 as c_int as usize]) as *mut uint16_t;
    let mut tiny_hash: *mut uint8_t =
        TinyHashH40((*self_0).extra[0 as c_int as usize]) as *mut uint8_t;
    let mut partial_prepare_threshold: size_t =
        (BUCKET_SIZE_3 >> 6 as c_int) as size_t;
    if one_shot != 0 && input_size <= partial_prepare_threshold {
        let mut i: size_t = 0;
        i = 0 as size_t;
        while i < input_size {
            let mut bucket: size_t = HashBytesH40(data.offset(i as isize) as *const uint8_t);
            *addr.offset(bucket as isize) = 0xcccccccc as c_uint as uint32_t;
            *head.offset(bucket as isize) = 0xcccc as uint16_t;
            i = i.wrapping_add(1);
        }
    } else {
        memset(
            addr as *mut c_void,
            0xcc as c_int,
            (::core::mem::size_of::<uint32_t>() as size_t).wrapping_mul(BUCKET_SIZE_3 as size_t),
        );
        memset(
            head as *mut c_void,
            0 as c_int,
            (::core::mem::size_of::<uint16_t>() as size_t).wrapping_mul(BUCKET_SIZE_3 as size_t),
        );
    }
    memset(
        tiny_hash as *mut c_void,
        0 as c_int,
        (::core::mem::size_of::<uint8_t>() as size_t)
            .wrapping_mul(65536 as c_int as size_t),
    );
    memset(
        &raw mut (*self_0).free_slot_idx as *mut uint16_t as *mut c_void,
        0 as c_int,
        ::core::mem::size_of::<[uint16_t; 1]>() as size_t,
    );
}
#[inline(always)]
unsafe extern "C" fn HashMemAllocInBytesH40(
    mut params: *const BrotliEncoderParams,
    mut one_shot: c_int,
    mut input_size: size_t,
    mut alloc_size: *mut size_t,
) {
    *alloc_size.offset(0 as c_int as isize) = (::core::mem::size_of::<uint32_t>()
        as usize)
        .wrapping_mul(BUCKET_SIZE_3 as usize)
        .wrapping_add(
            (::core::mem::size_of::<uint16_t>() as usize).wrapping_mul(BUCKET_SIZE_3 as usize),
        )
        .wrapping_add(
            (::core::mem::size_of::<uint8_t>() as usize)
                .wrapping_mul(65536 as c_int as usize),
        ) as size_t;
    *alloc_size.offset(1 as c_int as isize) =
        (::core::mem::size_of::<BankH40>() as usize).wrapping_mul(NUM_BANKS_0 as usize) as size_t;
}
#[inline(always)]
unsafe extern "C" fn StoreH40(
    mut self_0: *mut H40,
    mut data: *const uint8_t,
    mask: size_t,
    ix: size_t,
) {
    let mut addr: *mut uint32_t =
        AddrH40((*self_0).extra[0 as c_int as usize]) as *mut uint32_t;
    let mut head: *mut uint16_t =
        HeadH40((*self_0).extra[0 as c_int as usize]) as *mut uint16_t;
    let mut tiny_hash: *mut uint8_t =
        TinyHashH40((*self_0).extra[0 as c_int as usize]) as *mut uint8_t;
    let mut banks: *mut BankH40 =
        BanksH40((*self_0).extra[1 as c_int as usize]) as *mut BankH40;
    let key: size_t = HashBytesH40(data.offset((ix & mask) as isize) as *const uint8_t) as size_t;
    let bank: size_t = key & (NUM_BANKS_0 - 1 as c_int) as size_t;
    let fresh17 = (*self_0).free_slot_idx[bank as usize];
    (*self_0).free_slot_idx[bank as usize] = (*self_0).free_slot_idx[bank as usize].wrapping_add(1);
    let idx: size_t =
        (fresh17 as c_int & BANK_SIZE_1 - 1 as c_int) as size_t;
    let mut delta: size_t = ix.wrapping_sub(*addr.offset(key as isize) as size_t);
    *tiny_hash.offset(ix as uint16_t as isize) = key as uint8_t;
    if delta > 0xffff as size_t {
        delta = (if CAPPED_CHAINS_1 != 0 {
            0 as c_int
        } else {
            0xffff as c_int
        }) as size_t;
    }
    (*banks.offset(bank as isize)).slots[idx as usize].delta = delta as uint16_t;
    (*banks.offset(bank as isize)).slots[idx as usize].next = *head.offset(key as isize);
    *addr.offset(key as isize) = ix as uint32_t;
    *head.offset(key as isize) = idx as uint16_t;
}
#[inline(always)]
unsafe extern "C" fn StitchToPreviousBlockH40(
    mut self_0: *mut H40,
    mut num_bytes: size_t,
    mut position: size_t,
    mut ringbuffer: *const uint8_t,
    mut ring_buffer_mask: size_t,
) {
    if num_bytes >= HashTypeLengthH40().wrapping_sub(1 as size_t) && position >= 3 as size_t {
        StoreH40(
            self_0,
            ringbuffer,
            ring_buffer_mask,
            position.wrapping_sub(3 as size_t),
        );
        StoreH40(
            self_0,
            ringbuffer,
            ring_buffer_mask,
            position.wrapping_sub(2 as size_t),
        );
        StoreH40(
            self_0,
            ringbuffer,
            ring_buffer_mask,
            position.wrapping_sub(1 as size_t),
        );
    }
}
pub const BANK_SIZE_0: c_int = (1 as c_int) << BANK_BITS_0;
pub const BUCKET_SIZE_2: c_int = (1 as c_int) << BUCKET_BITS_2;

#[inline(always)]
unsafe extern "C" fn HashBytesH41(mut data: *const uint8_t) -> size_t {
    let h: uint32_t = (BrotliUnalignedRead32(data as *const c_void) as uint32_t)
        .wrapping_mul(kHashMul32);
    return (h >> 32 as c_int - BUCKET_BITS_2) as size_t;
}

unsafe extern "C" fn HeadH41(mut extra: *mut c_void) -> *mut uint16_t {
    return (AddrH41 as unsafe extern "C" fn(*mut c_void) -> *mut uint32_t)(extra)
        .offset(BUCKET_SIZE_2 as isize) as *mut uint32_t as *mut uint16_t;
}
unsafe extern "C" fn TinyHashH41(mut extra: *mut c_void) -> *mut uint8_t {
    return (HeadH41 as unsafe extern "C" fn(*mut c_void) -> *mut uint16_t)(extra)
        .offset(BUCKET_SIZE_2 as isize) as *mut uint16_t as *mut uint8_t;
}

unsafe extern "C" fn InitializeH41(
    mut common: *mut HasherCommon,
    mut self_0: *mut H41,
    mut params: *const BrotliEncoderParams,
) {
    (*self_0).common = common;
    (*self_0).extra[0 as c_int as usize] =
        (*common).extra[0 as c_int as usize];
    (*self_0).extra[1 as c_int as usize] =
        (*common).extra[1 as c_int as usize];
    (*self_0).max_hops = ((if (*params).quality > 6 as c_int {
        7 as c_uint
    } else {
        8 as c_uint
    }) << (*params).quality - 4 as c_int) as size_t;
}
unsafe extern "C" fn PrepareH41(
    mut self_0: *mut H41,
    mut one_shot: c_int,
    mut input_size: size_t,
    mut data: *const uint8_t,
) {
    let mut addr: *mut uint32_t =
        AddrH41((*self_0).extra[0 as c_int as usize]) as *mut uint32_t;
    let mut head: *mut uint16_t =
        HeadH41((*self_0).extra[0 as c_int as usize]) as *mut uint16_t;
    let mut tiny_hash: *mut uint8_t =
        TinyHashH41((*self_0).extra[0 as c_int as usize]) as *mut uint8_t;
    let mut partial_prepare_threshold: size_t =
        (BUCKET_SIZE_2 >> 6 as c_int) as size_t;
    if one_shot != 0 && input_size <= partial_prepare_threshold {
        let mut i: size_t = 0;
        i = 0 as size_t;
        while i < input_size {
            let mut bucket: size_t = HashBytesH41(data.offset(i as isize) as *const uint8_t);
            *addr.offset(bucket as isize) = 0xcccccccc as c_uint as uint32_t;
            *head.offset(bucket as isize) = 0xcccc as uint16_t;
            i = i.wrapping_add(1);
        }
    } else {
        memset(
            addr as *mut c_void,
            0xcc as c_int,
            (::core::mem::size_of::<uint32_t>() as size_t).wrapping_mul(BUCKET_SIZE_2 as size_t),
        );
        memset(
            head as *mut c_void,
            0 as c_int,
            (::core::mem::size_of::<uint16_t>() as size_t).wrapping_mul(BUCKET_SIZE_2 as size_t),
        );
    }
    memset(
        tiny_hash as *mut c_void,
        0 as c_int,
        (::core::mem::size_of::<uint8_t>() as size_t)
            .wrapping_mul(65536 as c_int as size_t),
    );
    memset(
        &raw mut (*self_0).free_slot_idx as *mut uint16_t as *mut c_void,
        0 as c_int,
        ::core::mem::size_of::<[uint16_t; 1]>() as size_t,
    );
}
#[inline(always)]
unsafe extern "C" fn HashMemAllocInBytesH41(
    mut params: *const BrotliEncoderParams,
    mut one_shot: c_int,
    mut input_size: size_t,
    mut alloc_size: *mut size_t,
) {
    *alloc_size.offset(0 as c_int as isize) = (::core::mem::size_of::<uint32_t>()
        as usize)
        .wrapping_mul(BUCKET_SIZE_2 as usize)
        .wrapping_add(
            (::core::mem::size_of::<uint16_t>() as usize).wrapping_mul(BUCKET_SIZE_2 as usize),
        )
        .wrapping_add(
            (::core::mem::size_of::<uint8_t>() as usize)
                .wrapping_mul(65536 as c_int as usize),
        ) as size_t;
    *alloc_size.offset(1 as c_int as isize) =
        (::core::mem::size_of::<BankH41>() as usize).wrapping_mul(NUM_BANKS_0 as usize) as size_t;
}
#[inline(always)]
unsafe extern "C" fn StoreH41(
    mut self_0: *mut H41,
    mut data: *const uint8_t,
    mask: size_t,
    ix: size_t,
) {
    let mut addr: *mut uint32_t =
        AddrH41((*self_0).extra[0 as c_int as usize]) as *mut uint32_t;
    let mut head: *mut uint16_t =
        HeadH41((*self_0).extra[0 as c_int as usize]) as *mut uint16_t;
    let mut tiny_hash: *mut uint8_t =
        TinyHashH41((*self_0).extra[0 as c_int as usize]) as *mut uint8_t;
    let mut banks: *mut BankH41 =
        BanksH41((*self_0).extra[1 as c_int as usize]) as *mut BankH41;
    let key: size_t = HashBytesH41(data.offset((ix & mask) as isize) as *const uint8_t) as size_t;
    let bank: size_t = key & (NUM_BANKS_0 - 1 as c_int) as size_t;
    let fresh16 = (*self_0).free_slot_idx[bank as usize];
    (*self_0).free_slot_idx[bank as usize] = (*self_0).free_slot_idx[bank as usize].wrapping_add(1);
    let idx: size_t =
        (fresh16 as c_int & BANK_SIZE_0 - 1 as c_int) as size_t;
    let mut delta: size_t = ix.wrapping_sub(*addr.offset(key as isize) as size_t);
    *tiny_hash.offset(ix as uint16_t as isize) = key as uint8_t;
    if delta > 0xffff as size_t {
        delta = (if CAPPED_CHAINS_0 != 0 {
            0 as c_int
        } else {
            0xffff as c_int
        }) as size_t;
    }
    (*banks.offset(bank as isize)).slots[idx as usize].delta = delta as uint16_t;
    (*banks.offset(bank as isize)).slots[idx as usize].next = *head.offset(key as isize);
    *addr.offset(key as isize) = ix as uint32_t;
    *head.offset(key as isize) = idx as uint16_t;
}
#[inline(always)]
unsafe extern "C" fn StitchToPreviousBlockH41(
    mut self_0: *mut H41,
    mut num_bytes: size_t,
    mut position: size_t,
    mut ringbuffer: *const uint8_t,
    mut ring_buffer_mask: size_t,
) {
    if num_bytes >= HashTypeLengthH41().wrapping_sub(1 as size_t) && position >= 3 as size_t {
        StoreH41(
            self_0,
            ringbuffer,
            ring_buffer_mask,
            position.wrapping_sub(3 as size_t),
        );
        StoreH41(
            self_0,
            ringbuffer,
            ring_buffer_mask,
            position.wrapping_sub(2 as size_t),
        );
        StoreH41(
            self_0,
            ringbuffer,
            ring_buffer_mask,
            position.wrapping_sub(1 as size_t),
        );
    }
}
pub const BANK_SIZE: c_int = (1 as c_int) << BANK_BITS;
pub const BUCKET_SIZE_1: c_int = (1 as c_int) << BUCKET_BITS_2;

#[inline(always)]
unsafe extern "C" fn HashBytesH42(mut data: *const uint8_t) -> size_t {
    let h: uint32_t = (BrotliUnalignedRead32(data as *const c_void) as uint32_t)
        .wrapping_mul(kHashMul32);
    return (h >> 32 as c_int - BUCKET_BITS_2) as size_t;
}

unsafe extern "C" fn HeadH42(mut extra: *mut c_void) -> *mut uint16_t {
    return (AddrH42 as unsafe extern "C" fn(*mut c_void) -> *mut uint32_t)(extra)
        .offset(BUCKET_SIZE_1 as isize) as *mut uint32_t as *mut uint16_t;
}
unsafe extern "C" fn TinyHashH42(mut extra: *mut c_void) -> *mut uint8_t {
    return (HeadH42 as unsafe extern "C" fn(*mut c_void) -> *mut uint16_t)(extra)
        .offset(BUCKET_SIZE_1 as isize) as *mut uint16_t as *mut uint8_t;
}

unsafe extern "C" fn InitializeH42(
    mut common: *mut HasherCommon,
    mut self_0: *mut H42,
    mut params: *const BrotliEncoderParams,
) {
    (*self_0).common = common;
    (*self_0).extra[0 as c_int as usize] =
        (*common).extra[0 as c_int as usize];
    (*self_0).extra[1 as c_int as usize] =
        (*common).extra[1 as c_int as usize];
    (*self_0).max_hops = ((if (*params).quality > 6 as c_int {
        7 as c_uint
    } else {
        8 as c_uint
    }) << (*params).quality - 4 as c_int) as size_t;
}
unsafe extern "C" fn PrepareH42(
    mut self_0: *mut H42,
    mut one_shot: c_int,
    mut input_size: size_t,
    mut data: *const uint8_t,
) {
    let mut addr: *mut uint32_t =
        AddrH42((*self_0).extra[0 as c_int as usize]) as *mut uint32_t;
    let mut head: *mut uint16_t =
        HeadH42((*self_0).extra[0 as c_int as usize]) as *mut uint16_t;
    let mut tiny_hash: *mut uint8_t =
        TinyHashH42((*self_0).extra[0 as c_int as usize]) as *mut uint8_t;
    let mut partial_prepare_threshold: size_t =
        (BUCKET_SIZE_1 >> 6 as c_int) as size_t;
    if one_shot != 0 && input_size <= partial_prepare_threshold {
        let mut i: size_t = 0;
        i = 0 as size_t;
        while i < input_size {
            let mut bucket: size_t = HashBytesH42(data.offset(i as isize) as *const uint8_t);
            *addr.offset(bucket as isize) = 0xcccccccc as c_uint as uint32_t;
            *head.offset(bucket as isize) = 0xcccc as uint16_t;
            i = i.wrapping_add(1);
        }
    } else {
        memset(
            addr as *mut c_void,
            0xcc as c_int,
            (::core::mem::size_of::<uint32_t>() as size_t).wrapping_mul(BUCKET_SIZE_1 as size_t),
        );
        memset(
            head as *mut c_void,
            0 as c_int,
            (::core::mem::size_of::<uint16_t>() as size_t).wrapping_mul(BUCKET_SIZE_1 as size_t),
        );
    }
    memset(
        tiny_hash as *mut c_void,
        0 as c_int,
        (::core::mem::size_of::<uint8_t>() as size_t)
            .wrapping_mul(65536 as c_int as size_t),
    );
    memset(
        &raw mut (*self_0).free_slot_idx as *mut uint16_t as *mut c_void,
        0 as c_int,
        ::core::mem::size_of::<[uint16_t; 512]>() as size_t,
    );
}
#[inline(always)]
unsafe extern "C" fn HashMemAllocInBytesH42(
    mut params: *const BrotliEncoderParams,
    mut one_shot: c_int,
    mut input_size: size_t,
    mut alloc_size: *mut size_t,
) {
    *alloc_size.offset(0 as c_int as isize) = (::core::mem::size_of::<uint32_t>()
        as usize)
        .wrapping_mul(BUCKET_SIZE_1 as usize)
        .wrapping_add(
            (::core::mem::size_of::<uint16_t>() as usize).wrapping_mul(BUCKET_SIZE_1 as usize),
        )
        .wrapping_add(
            (::core::mem::size_of::<uint8_t>() as usize)
                .wrapping_mul(65536 as c_int as usize),
        ) as size_t;
    *alloc_size.offset(1 as c_int as isize) =
        (::core::mem::size_of::<BankH42>() as usize).wrapping_mul(NUM_BANKS as usize) as size_t;
}
#[inline(always)]
unsafe extern "C" fn StoreH42(
    mut self_0: *mut H42,
    mut data: *const uint8_t,
    mask: size_t,
    ix: size_t,
) {
    let mut addr: *mut uint32_t =
        AddrH42((*self_0).extra[0 as c_int as usize]) as *mut uint32_t;
    let mut head: *mut uint16_t =
        HeadH42((*self_0).extra[0 as c_int as usize]) as *mut uint16_t;
    let mut tiny_hash: *mut uint8_t =
        TinyHashH42((*self_0).extra[0 as c_int as usize]) as *mut uint8_t;
    let mut banks: *mut BankH42 =
        BanksH42((*self_0).extra[1 as c_int as usize]) as *mut BankH42;
    let key: size_t = HashBytesH42(data.offset((ix & mask) as isize) as *const uint8_t) as size_t;
    let bank: size_t = key & (NUM_BANKS - 1 as c_int) as size_t;
    let fresh15 = (*self_0).free_slot_idx[bank as usize];
    (*self_0).free_slot_idx[bank as usize] = (*self_0).free_slot_idx[bank as usize].wrapping_add(1);
    let idx: size_t =
        (fresh15 as c_int & BANK_SIZE - 1 as c_int) as size_t;
    let mut delta: size_t = ix.wrapping_sub(*addr.offset(key as isize) as size_t);
    *tiny_hash.offset(ix as uint16_t as isize) = key as uint8_t;
    if delta > 0xffff as size_t {
        delta = (if CAPPED_CHAINS != 0 {
            0 as c_int
        } else {
            0xffff as c_int
        }) as size_t;
    }
    (*banks.offset(bank as isize)).slots[idx as usize].delta = delta as uint16_t;
    (*banks.offset(bank as isize)).slots[idx as usize].next = *head.offset(key as isize);
    *addr.offset(key as isize) = ix as uint32_t;
    *head.offset(key as isize) = idx as uint16_t;
}
#[inline(always)]
unsafe extern "C" fn StitchToPreviousBlockH42(
    mut self_0: *mut H42,
    mut num_bytes: size_t,
    mut position: size_t,
    mut ringbuffer: *const uint8_t,
    mut ring_buffer_mask: size_t,
) {
    if num_bytes >= HashTypeLengthH42().wrapping_sub(1 as size_t) && position >= 3 as size_t {
        StoreH42(
            self_0,
            ringbuffer,
            ring_buffer_mask,
            position.wrapping_sub(3 as size_t),
        );
        StoreH42(
            self_0,
            ringbuffer,
            ring_buffer_mask,
            position.wrapping_sub(2 as size_t),
        );
        StoreH42(
            self_0,
            ringbuffer,
            ring_buffer_mask,
            position.wrapping_sub(1 as size_t),
        );
    }
}

unsafe extern "C" fn HashBytesH58(mut data: *const uint8_t, shift: c_int) -> uint32_t {
    let mut h: uint32_t =
        BrotliUnalignedRead32(data as *const c_void).wrapping_mul(kHashMul32);
    return h >> shift;
}
unsafe extern "C" fn InitializeH58(
    mut common: *mut HasherCommon,
    mut self_0: *mut H58,
    mut params: *const BrotliEncoderParams,
) {
    (*self_0).common_ = common;
    (*self_0).hash_shift_ =
        32 as c_int - (*common).params.bucket_bits - TAG_HASH_BITS_0;
    (*self_0).bucket_size_ = (1 as c_int as size_t) << (*common).params.bucket_bits;
    (*self_0).block_size_ = (1 as c_int as size_t) << (*common).params.block_bits;
    (*self_0).block_mask_ = (*self_0).block_size_.wrapping_sub(1 as size_t) as uint32_t;
    (*self_0).num_ = (*common).extra[0 as c_int as usize] as *mut uint16_t;
    (*self_0).tags_ = (*common).extra[1 as c_int as usize] as *mut uint8_t;
    (*self_0).buckets_ = (*common).extra[2 as c_int as usize] as *mut uint32_t;
    (*self_0).block_bits_ = (*common).params.block_bits;
    (*self_0).num_last_distances_to_check_ = (*common).params.num_last_distances_to_check;
}
unsafe extern "C" fn PrepareH58(
    mut self_0: *mut H58,
    mut one_shot: c_int,
    mut input_size: size_t,
    mut data: *const uint8_t,
) {
    let mut num: *mut uint16_t = (*self_0).num_;
    let mut partial_prepare_threshold: size_t = (*self_0).bucket_size_ >> 6 as c_int;
    if one_shot != 0 && input_size <= partial_prepare_threshold {
        let mut i: size_t = 0;
        i = 0 as size_t;
        while i < input_size {
            let hash: uint32_t = HashBytesH58(
                data.offset(i as isize) as *const uint8_t,
                (*self_0).hash_shift_,
            ) as uint32_t;
            let key: uint32_t = hash >> TAG_HASH_BITS_0;
            *num.offset(key as isize) = 65535 as uint16_t;
            i = i.wrapping_add(1);
        }
    } else {
        memset(
            num as *mut c_void,
            255 as c_int,
            (*self_0)
                .bucket_size_
                .wrapping_mul(::core::mem::size_of::<uint16_t>() as size_t),
        );
    };
}
#[inline(always)]
unsafe extern "C" fn HashMemAllocInBytesH58(
    mut params: *const BrotliEncoderParams,
    mut one_shot: c_int,
    mut input_size: size_t,
    mut alloc_size: *mut size_t,
) {
    let mut bucket_size: size_t =
        (1 as c_int as size_t) << (*params).hasher.bucket_bits;
    let mut block_size: size_t = (1 as c_int as size_t) << (*params).hasher.block_bits;
    *alloc_size.offset(0 as c_int as isize) =
        (::core::mem::size_of::<uint16_t>() as usize).wrapping_mul(bucket_size as usize) as size_t;
    *alloc_size.offset(1 as c_int as isize) =
        (::core::mem::size_of::<uint8_t>() as usize)
            .wrapping_mul(bucket_size as usize)
            .wrapping_mul(block_size as usize) as size_t;
    *alloc_size.offset(2 as c_int as isize) =
        (::core::mem::size_of::<uint32_t>() as usize)
            .wrapping_mul(bucket_size as usize)
            .wrapping_mul(block_size as usize) as size_t;
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
    let key: size_t = hash >> TAG_HASH_BITS_0;
    let tag: uint8_t = (hash & TAG_HASH_MASK_0 as size_t) as uint8_t;
    let minor_ix: size_t =
        (*num.offset(key as isize) as uint32_t & (*self_0).block_mask_) as size_t;
    let offset: size_t = minor_ix.wrapping_add(key << (*self_0).block_bits_);
    let ref mut fresh14 = *num.offset(key as isize);
    *fresh14 = (*fresh14).wrapping_sub(1);
    *buckets.offset(offset as isize) = ix as uint32_t;
    *tags.offset(offset as isize) = tag;
}
#[inline(always)]
unsafe extern "C" fn StitchToPreviousBlockH58(
    mut self_0: *mut H58,
    mut num_bytes: size_t,
    mut position: size_t,
    mut ringbuffer: *const uint8_t,
    mut ringbuffer_mask: size_t,
) {
    if num_bytes >= HashTypeLengthH58().wrapping_sub(1 as size_t) && position >= 3 as size_t {
        StoreH58(
            self_0,
            ringbuffer,
            ringbuffer_mask,
            position.wrapping_sub(3 as size_t),
        );
        StoreH58(
            self_0,
            ringbuffer,
            ringbuffer_mask,
            position.wrapping_sub(2 as size_t),
        );
        StoreH58(
            self_0,
            ringbuffer,
            ringbuffer_mask,
            position.wrapping_sub(1 as size_t),
        );
    }
}
pub const BUCKET_BITS_2: c_int = 15 as c_int;
pub const NUM_BANKS_0: c_int = 1 as c_int;
pub const BANK_BITS_0: c_int = 16 as c_int;

#[inline(always)]
unsafe extern "C" fn HashBytesH68(mut data: *const uint8_t, mut hash_mul: uint64_t) -> size_t {
    let h: uint64_t = (BrotliUnalignedRead64(data as *const c_void) as uint64_t)
        .wrapping_mul(hash_mul);
    return (h >> 64 as c_int - 15 as c_int - TAG_HASH_BITS) as size_t;
}
unsafe extern "C" fn InitializeH68(
    mut common: *mut HasherCommon,
    mut self_0: *mut H68,
    mut params: *const BrotliEncoderParams,
) {
    (*self_0).common_ = common;
    (*self_0).hash_mul_ =
        kHashMul64 << 64 as c_int - 5 as c_int * 8 as c_int;
    (*self_0).bucket_size_ = (1 as c_int as size_t) << (*common).params.bucket_bits;
    (*self_0).block_bits_ = (*common).params.block_bits;
    (*self_0).block_size_ = (1 as c_int as size_t) << (*common).params.block_bits;
    (*self_0).block_mask_ = (*self_0).block_size_.wrapping_sub(1 as size_t) as uint32_t;
    (*self_0).num_last_distances_to_check_ = (*common).params.num_last_distances_to_check;
    (*self_0).num_ = (*common).extra[0 as c_int as usize] as *mut uint16_t;
    (*self_0).tags_ = (*common).extra[1 as c_int as usize] as *mut uint8_t;
    (*self_0).buckets_ = (*common).extra[2 as c_int as usize] as *mut uint32_t;
}
unsafe extern "C" fn PrepareH68(
    mut self_0: *mut H68,
    mut one_shot: c_int,
    mut input_size: size_t,
    mut data: *const uint8_t,
) {
    let mut num: *mut uint16_t = (*self_0).num_;
    let mut partial_prepare_threshold: size_t = (*self_0).bucket_size_ >> 6 as c_int;
    if one_shot != 0 && input_size <= partial_prepare_threshold {
        let mut i: size_t = 0;
        i = 0 as size_t;
        while i < input_size {
            let hash: size_t = HashBytesH68(
                data.offset(i as isize) as *const uint8_t,
                (*self_0).hash_mul_,
            ) as size_t;
            let key: size_t = hash >> TAG_HASH_BITS;
            *num.offset(key as isize) = 65535 as uint16_t;
            i = i.wrapping_add(1);
        }
    } else {
        memset(
            num as *mut c_void,
            255 as c_int,
            (*self_0)
                .bucket_size_
                .wrapping_mul(::core::mem::size_of::<uint16_t>() as size_t),
        );
    };
}
#[inline(always)]
unsafe extern "C" fn HashMemAllocInBytesH68(
    mut params: *const BrotliEncoderParams,
    mut one_shot: c_int,
    mut input_size: size_t,
    mut alloc_size: *mut size_t,
) {
    let mut bucket_size: size_t =
        (1 as c_int as size_t) << (*params).hasher.bucket_bits;
    let mut block_size: size_t = (1 as c_int as size_t) << (*params).hasher.block_bits;
    *alloc_size.offset(0 as c_int as isize) =
        (::core::mem::size_of::<uint16_t>() as usize).wrapping_mul(bucket_size as usize) as size_t;
    *alloc_size.offset(1 as c_int as isize) =
        (::core::mem::size_of::<uint8_t>() as usize)
            .wrapping_mul(bucket_size as usize)
            .wrapping_mul(block_size as usize) as size_t;
    *alloc_size.offset(2 as c_int as isize) =
        (::core::mem::size_of::<uint32_t>() as usize)
            .wrapping_mul(bucket_size as usize)
            .wrapping_mul(block_size as usize) as size_t;
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
    let key: size_t = hash >> TAG_HASH_BITS;
    let tag: uint8_t = (hash & TAG_HASH_MASK as size_t) as uint8_t;
    let minor_ix: size_t =
        (*num.offset(key as isize) as uint32_t & (*self_0).block_mask_) as size_t;
    let offset: size_t = minor_ix.wrapping_add(key << (*self_0).block_bits_);
    let ref mut fresh13 = *num.offset(key as isize);
    *fresh13 = (*fresh13).wrapping_sub(1);
    *buckets.offset(offset as isize) = ix as uint32_t;
    *tags.offset(offset as isize) = tag;
}
#[inline(always)]
unsafe extern "C" fn StitchToPreviousBlockH68(
    mut self_0: *mut H68,
    mut num_bytes: size_t,
    mut position: size_t,
    mut ringbuffer: *const uint8_t,
    mut ringbuffer_mask: size_t,
) {
    if num_bytes >= HashTypeLengthH68().wrapping_sub(1 as size_t) && position >= 3 as size_t {
        StoreH68(
            self_0,
            ringbuffer,
            ringbuffer_mask,
            position.wrapping_sub(3 as size_t),
        );
        StoreH68(
            self_0,
            ringbuffer,
            ringbuffer_mask,
            position.wrapping_sub(2 as size_t),
        );
        StoreH68(
            self_0,
            ringbuffer,
            ringbuffer_mask,
            position.wrapping_sub(1 as size_t),
        );
    }
}
unsafe extern "C" fn InitializeH35(
    mut common: *mut HasherCommon,
    mut self_0: *mut H35,
    mut params: *const BrotliEncoderParams,
) {
    (*self_0).common = common;
    (*self_0).ha_common = *(*self_0).common;
    (*self_0).hb_common = *(*self_0).common;
    (*self_0).fresh = BROTLI_TRUE;
    (*self_0).params = params;
}
unsafe extern "C" fn PrepareH35(
    mut self_0: *mut H35,
    mut one_shot: c_int,
    mut input_size: size_t,
    mut data: *const uint8_t,
) {
    if (*self_0).fresh != 0 {
        (*self_0).fresh = BROTLI_FALSE;
        (*self_0).ha_common.extra[0 as c_int as usize] =
            (*(*self_0).common).extra[0 as c_int as usize];
        (*self_0).ha_common.extra[1 as c_int as usize] =
            (*(*self_0).common).extra[1 as c_int as usize];
        (*self_0).ha_common.extra[2 as c_int as usize] = NULL;
        (*self_0).ha_common.extra[3 as c_int as usize] = NULL;
        (*self_0).hb_common.extra[0 as c_int as usize] =
            (*(*self_0).common).extra[2 as c_int as usize];
        (*self_0).hb_common.extra[1 as c_int as usize] =
            (*(*self_0).common).extra[3 as c_int as usize];
        (*self_0).hb_common.extra[2 as c_int as usize] = NULL;
        (*self_0).hb_common.extra[3 as c_int as usize] = NULL;
        InitializeH3(
            &raw mut (*self_0).ha_common,
            &raw mut (*self_0).ha,
            (*self_0).params,
        );
        InitializeHROLLING_FAST(
            &raw mut (*self_0).hb_common,
            &raw mut (*self_0).hb,
            (*self_0).params,
        );
    }
    PrepareH3(&raw mut (*self_0).ha, one_shot, input_size, data);
    PrepareHROLLING_FAST(&raw mut (*self_0).hb, one_shot, input_size, data);
}
#[inline(always)]
unsafe extern "C" fn HashMemAllocInBytesH35(
    mut params: *const BrotliEncoderParams,
    mut one_shot: c_int,
    mut input_size: size_t,
    mut alloc_size: *mut size_t,
) {
    let mut alloc_size_a: [size_t; 4] = [0 as c_int as size_t, 0, 0, 0];
    let mut alloc_size_b: [size_t; 4] = [0 as c_int as size_t, 0, 0, 0];
    HashMemAllocInBytesH3(
        params,
        one_shot,
        input_size,
        &raw mut alloc_size_a as *mut size_t,
    );
    HashMemAllocInBytesHROLLING_FAST(
        params,
        one_shot,
        input_size,
        &raw mut alloc_size_b as *mut size_t,
    );
    if alloc_size_a[2 as c_int as usize] != 0 as size_t
        || alloc_size_a[3 as c_int as usize] != 0 as size_t
    {
        exit(EXIT_FAILURE);
    }
    if alloc_size_b[2 as c_int as usize] != 0 as size_t
        || alloc_size_b[3 as c_int as usize] != 0 as size_t
    {
        exit(EXIT_FAILURE);
    }
    *alloc_size.offset(0 as c_int as isize) =
        alloc_size_a[0 as c_int as usize];
    *alloc_size.offset(1 as c_int as isize) =
        alloc_size_a[1 as c_int as usize];
    *alloc_size.offset(2 as c_int as isize) =
        alloc_size_b[0 as c_int as usize];
    *alloc_size.offset(3 as c_int as isize) =
        alloc_size_b[1 as c_int as usize];
}
#[inline(always)]
unsafe extern "C" fn StitchToPreviousBlockH35(
    mut self_0: *mut H35,
    mut num_bytes: size_t,
    mut position: size_t,
    mut ringbuffer: *const uint8_t,
    mut ring_buffer_mask: size_t,
) {
    StitchToPreviousBlockH3(
        &raw mut (*self_0).ha,
        num_bytes,
        position,
        ringbuffer,
        ring_buffer_mask,
    );
    StitchToPreviousBlockHROLLING_FAST(
        &raw mut (*self_0).hb,
        num_bytes,
        position,
        ringbuffer,
        ring_buffer_mask,
    );
}
unsafe extern "C" fn InitializeH55(
    mut common: *mut HasherCommon,
    mut self_0: *mut H55,
    mut params: *const BrotliEncoderParams,
) {
    (*self_0).common = common;
    (*self_0).ha_common = *(*self_0).common;
    (*self_0).hb_common = *(*self_0).common;
    (*self_0).fresh = BROTLI_TRUE;
    (*self_0).params = params;
}
unsafe extern "C" fn PrepareH55(
    mut self_0: *mut H55,
    mut one_shot: c_int,
    mut input_size: size_t,
    mut data: *const uint8_t,
) {
    if (*self_0).fresh != 0 {
        (*self_0).fresh = BROTLI_FALSE;
        (*self_0).ha_common.extra[0 as c_int as usize] =
            (*(*self_0).common).extra[0 as c_int as usize];
        (*self_0).ha_common.extra[1 as c_int as usize] =
            (*(*self_0).common).extra[1 as c_int as usize];
        (*self_0).ha_common.extra[2 as c_int as usize] = NULL;
        (*self_0).ha_common.extra[3 as c_int as usize] = NULL;
        (*self_0).hb_common.extra[0 as c_int as usize] =
            (*(*self_0).common).extra[2 as c_int as usize];
        (*self_0).hb_common.extra[1 as c_int as usize] =
            (*(*self_0).common).extra[3 as c_int as usize];
        (*self_0).hb_common.extra[2 as c_int as usize] = NULL;
        (*self_0).hb_common.extra[3 as c_int as usize] = NULL;
        InitializeH54(
            &raw mut (*self_0).ha_common,
            &raw mut (*self_0).ha,
            (*self_0).params,
        );
        InitializeHROLLING_FAST(
            &raw mut (*self_0).hb_common,
            &raw mut (*self_0).hb,
            (*self_0).params,
        );
    }
    PrepareH54(&raw mut (*self_0).ha, one_shot, input_size, data);
    PrepareHROLLING_FAST(&raw mut (*self_0).hb, one_shot, input_size, data);
}
#[inline(always)]
unsafe extern "C" fn HashMemAllocInBytesH55(
    mut params: *const BrotliEncoderParams,
    mut one_shot: c_int,
    mut input_size: size_t,
    mut alloc_size: *mut size_t,
) {
    let mut alloc_size_a: [size_t; 4] = [0 as c_int as size_t, 0, 0, 0];
    let mut alloc_size_b: [size_t; 4] = [0 as c_int as size_t, 0, 0, 0];
    HashMemAllocInBytesH54(
        params,
        one_shot,
        input_size,
        &raw mut alloc_size_a as *mut size_t,
    );
    HashMemAllocInBytesHROLLING_FAST(
        params,
        one_shot,
        input_size,
        &raw mut alloc_size_b as *mut size_t,
    );
    if alloc_size_a[2 as c_int as usize] != 0 as size_t
        || alloc_size_a[3 as c_int as usize] != 0 as size_t
    {
        exit(EXIT_FAILURE);
    }
    if alloc_size_b[2 as c_int as usize] != 0 as size_t
        || alloc_size_b[3 as c_int as usize] != 0 as size_t
    {
        exit(EXIT_FAILURE);
    }
    *alloc_size.offset(0 as c_int as isize) =
        alloc_size_a[0 as c_int as usize];
    *alloc_size.offset(1 as c_int as isize) =
        alloc_size_a[1 as c_int as usize];
    *alloc_size.offset(2 as c_int as isize) =
        alloc_size_b[0 as c_int as usize];
    *alloc_size.offset(3 as c_int as isize) =
        alloc_size_b[1 as c_int as usize];
}
#[inline(always)]
unsafe extern "C" fn StitchToPreviousBlockH55(
    mut self_0: *mut H55,
    mut num_bytes: size_t,
    mut position: size_t,
    mut ringbuffer: *const uint8_t,
    mut ring_buffer_mask: size_t,
) {
    StitchToPreviousBlockH54(
        &raw mut (*self_0).ha,
        num_bytes,
        position,
        ringbuffer,
        ring_buffer_mask,
    );
    StitchToPreviousBlockHROLLING_FAST(
        &raw mut (*self_0).hb,
        num_bytes,
        position,
        ringbuffer,
        ring_buffer_mask,
    );
}
unsafe extern "C" fn InitializeH65(
    mut common: *mut HasherCommon,
    mut self_0: *mut H65,
    mut params: *const BrotliEncoderParams,
) {
    (*self_0).common = common;
    (*self_0).ha_common = *(*self_0).common;
    (*self_0).hb_common = *(*self_0).common;
    (*self_0).fresh = BROTLI_TRUE;
    (*self_0).params = params;
}
unsafe extern "C" fn PrepareH65(
    mut self_0: *mut H65,
    mut one_shot: c_int,
    mut input_size: size_t,
    mut data: *const uint8_t,
) {
    if (*self_0).fresh != 0 {
        (*self_0).fresh = BROTLI_FALSE;
        (*self_0).ha_common.extra[0 as c_int as usize] =
            (*(*self_0).common).extra[0 as c_int as usize];
        (*self_0).ha_common.extra[1 as c_int as usize] =
            (*(*self_0).common).extra[1 as c_int as usize];
        (*self_0).ha_common.extra[2 as c_int as usize] = NULL;
        (*self_0).ha_common.extra[3 as c_int as usize] = NULL;
        (*self_0).hb_common.extra[0 as c_int as usize] =
            (*(*self_0).common).extra[2 as c_int as usize];
        (*self_0).hb_common.extra[1 as c_int as usize] =
            (*(*self_0).common).extra[3 as c_int as usize];
        (*self_0).hb_common.extra[2 as c_int as usize] = NULL;
        (*self_0).hb_common.extra[3 as c_int as usize] = NULL;
        InitializeH6(
            &raw mut (*self_0).ha_common,
            &raw mut (*self_0).ha,
            (*self_0).params,
        );
        InitializeHROLLING(
            &raw mut (*self_0).hb_common,
            &raw mut (*self_0).hb,
            (*self_0).params,
        );
    }
    PrepareH6(&raw mut (*self_0).ha, one_shot, input_size, data);
    PrepareHROLLING(&raw mut (*self_0).hb, one_shot, input_size, data);
}
#[inline(always)]
unsafe extern "C" fn HashMemAllocInBytesH65(
    mut params: *const BrotliEncoderParams,
    mut one_shot: c_int,
    mut input_size: size_t,
    mut alloc_size: *mut size_t,
) {
    let mut alloc_size_a: [size_t; 4] = [0 as c_int as size_t, 0, 0, 0];
    let mut alloc_size_b: [size_t; 4] = [0 as c_int as size_t, 0, 0, 0];
    HashMemAllocInBytesH6(
        params,
        one_shot,
        input_size,
        &raw mut alloc_size_a as *mut size_t,
    );
    HashMemAllocInBytesHROLLING(
        params,
        one_shot,
        input_size,
        &raw mut alloc_size_b as *mut size_t,
    );
    if alloc_size_a[2 as c_int as usize] != 0 as size_t
        || alloc_size_a[3 as c_int as usize] != 0 as size_t
    {
        exit(EXIT_FAILURE);
    }
    if alloc_size_b[2 as c_int as usize] != 0 as size_t
        || alloc_size_b[3 as c_int as usize] != 0 as size_t
    {
        exit(EXIT_FAILURE);
    }
    *alloc_size.offset(0 as c_int as isize) =
        alloc_size_a[0 as c_int as usize];
    *alloc_size.offset(1 as c_int as isize) =
        alloc_size_a[1 as c_int as usize];
    *alloc_size.offset(2 as c_int as isize) =
        alloc_size_b[0 as c_int as usize];
    *alloc_size.offset(3 as c_int as isize) =
        alloc_size_b[1 as c_int as usize];
}
#[inline(always)]
unsafe extern "C" fn StitchToPreviousBlockH65(
    mut self_0: *mut H65,
    mut num_bytes: size_t,
    mut position: size_t,
    mut ringbuffer: *const uint8_t,
    mut ring_buffer_mask: size_t,
) {
    StitchToPreviousBlockH6(
        &raw mut (*self_0).ha,
        num_bytes,
        position,
        ringbuffer,
        ring_buffer_mask,
    );
    StitchToPreviousBlockHROLLING(
        &raw mut (*self_0).hb,
        num_bytes,
        position,
        ringbuffer,
        ring_buffer_mask,
    );
}
pub const NUM_BANKS: c_int = 512 as c_int;
pub const BANK_BITS: c_int = 9 as c_int;
static mut kRollingHashMul32HROLLING_FAST: uint32_t = 69069 as uint32_t;
static mut kInvalidPosHROLLING_FAST: uint32_t = 0xffffffff as uint32_t;

unsafe extern "C" fn HashRollingFunctionInitialHROLLING_FAST(
    mut state: uint32_t,
    mut add: uint8_t,
    mut factor: uint32_t,
) -> uint32_t {
    return factor
        .wrapping_mul(state)
        .wrapping_add(HashByteHROLLING_FAST(add));
}
unsafe extern "C" fn InitializeHROLLING_FAST(
    mut common: *mut HasherCommon,
    mut self_0: *mut HROLLING_FAST,
    mut params: *const BrotliEncoderParams,
) {
    let mut i: size_t = 0;
    (*self_0).state = 0 as uint32_t;
    (*self_0).next_ix = 0 as size_t;
    (*self_0).factor = kRollingHashMul32HROLLING_FAST;
    (*self_0).factor_remove = 1 as uint32_t;
    i = 0 as size_t;
    while i < CHUNKLEN as size_t {
        (*self_0).factor_remove = ((*self_0).factor_remove as c_uint)
            .wrapping_mul((*self_0).factor as c_uint)
            as uint32_t as uint32_t;
        i = (i as c_ulong).wrapping_add(JUMP_0 as c_ulong) as size_t
            as size_t;
    }
    (*self_0).table = (*common).extra[0 as c_int as usize] as *mut uint32_t;
    i = 0 as size_t;
    while i < NUMBUCKETS as size_t {
        *(*self_0).table.offset(i as isize) = kInvalidPosHROLLING_FAST;
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn PrepareHROLLING_FAST(
    mut self_0: *mut HROLLING_FAST,
    mut one_shot: c_int,
    mut input_size: size_t,
    mut data: *const uint8_t,
) {
    let mut i: size_t = 0;
    if input_size < CHUNKLEN as size_t {
        return;
    }
    (*self_0).state = 0 as uint32_t;
    i = 0 as size_t;
    while i < CHUNKLEN as size_t {
        (*self_0).state = HashRollingFunctionInitialHROLLING_FAST(
            (*self_0).state,
            *data.offset(i as isize),
            (*self_0).factor,
        );
        i = (i as c_ulong).wrapping_add(JUMP_0 as c_ulong) as size_t
            as size_t;
    }
}
#[inline(always)]
unsafe extern "C" fn HashMemAllocInBytesHROLLING_FAST(
    mut params: *const BrotliEncoderParams,
    mut one_shot: c_int,
    mut input_size: size_t,
    mut alloc_size: *mut size_t,
) {
    *alloc_size.offset(0 as c_int as isize) =
        (NUMBUCKETS as usize).wrapping_mul(::core::mem::size_of::<uint32_t>() as usize) as size_t;
}
#[inline(always)]
unsafe extern "C" fn StitchToPreviousBlockHROLLING_FAST(
    mut self_0: *mut HROLLING_FAST,
    mut num_bytes: size_t,
    mut position: size_t,
    mut ringbuffer: *const uint8_t,
    mut ring_buffer_mask: size_t,
) {
    let mut position_masked: size_t = 0;
    let mut available: size_t = num_bytes;
    if position & (JUMP_0 - 1 as c_int) as size_t != 0 as size_t {
        let mut diff: size_t = (JUMP_0 as size_t)
            .wrapping_sub(position & (JUMP_0 - 1 as c_int) as size_t);
        available = if diff > available {
            0 as size_t
        } else {
            available.wrapping_sub(diff)
        };
        position = (position as c_ulong).wrapping_add(diff as c_ulong)
            as size_t as size_t;
    }
    position_masked = position & ring_buffer_mask;
    if available > ring_buffer_mask.wrapping_sub(position_masked) {
        available = ring_buffer_mask.wrapping_sub(position_masked);
    }
    PrepareHROLLING_FAST(
        self_0,
        BROTLI_FALSE,
        available,
        ringbuffer.offset((position & ring_buffer_mask) as isize),
    );
    (*self_0).next_ix = position;
}
static mut kRollingHashMul32HROLLING: uint32_t = 69069 as uint32_t;
static mut kInvalidPosHROLLING: uint32_t = 0xffffffff as uint32_t;

unsafe extern "C" fn HashRollingFunctionInitialHROLLING(
    mut state: uint32_t,
    mut add: uint8_t,
    mut factor: uint32_t,
) -> uint32_t {
    return factor
        .wrapping_mul(state)
        .wrapping_add(HashByteHROLLING(add));
}
unsafe extern "C" fn InitializeHROLLING(
    mut common: *mut HasherCommon,
    mut self_0: *mut HROLLING,
    mut params: *const BrotliEncoderParams,
) {
    let mut i: size_t = 0;
    (*self_0).state = 0 as uint32_t;
    (*self_0).next_ix = 0 as size_t;
    (*self_0).factor = kRollingHashMul32HROLLING;
    (*self_0).factor_remove = 1 as uint32_t;
    i = 0 as size_t;
    while i < CHUNKLEN as size_t {
        (*self_0).factor_remove = ((*self_0).factor_remove as c_uint)
            .wrapping_mul((*self_0).factor as c_uint)
            as uint32_t as uint32_t;
        i = (i as c_ulong).wrapping_add(JUMP as c_ulong) as size_t
            as size_t;
    }
    (*self_0).table = (*common).extra[0 as c_int as usize] as *mut uint32_t;
    i = 0 as size_t;
    while i < NUMBUCKETS as size_t {
        *(*self_0).table.offset(i as isize) = kInvalidPosHROLLING;
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn PrepareHROLLING(
    mut self_0: *mut HROLLING,
    mut one_shot: c_int,
    mut input_size: size_t,
    mut data: *const uint8_t,
) {
    let mut i: size_t = 0;
    if input_size < CHUNKLEN as size_t {
        return;
    }
    (*self_0).state = 0 as uint32_t;
    i = 0 as size_t;
    while i < CHUNKLEN as size_t {
        (*self_0).state = HashRollingFunctionInitialHROLLING(
            (*self_0).state,
            *data.offset(i as isize),
            (*self_0).factor,
        );
        i = (i as c_ulong).wrapping_add(JUMP as c_ulong) as size_t
            as size_t;
    }
}
#[inline(always)]
unsafe extern "C" fn HashMemAllocInBytesHROLLING(
    mut params: *const BrotliEncoderParams,
    mut one_shot: c_int,
    mut input_size: size_t,
    mut alloc_size: *mut size_t,
) {
    *alloc_size.offset(0 as c_int as isize) =
        (NUMBUCKETS as usize).wrapping_mul(::core::mem::size_of::<uint32_t>() as usize) as size_t;
}
#[inline(always)]
unsafe extern "C" fn StitchToPreviousBlockHROLLING(
    mut self_0: *mut HROLLING,
    mut num_bytes: size_t,
    mut position: size_t,
    mut ringbuffer: *const uint8_t,
    mut ring_buffer_mask: size_t,
) {
    let mut position_masked: size_t = 0;
    let mut available: size_t = num_bytes;
    if position & (JUMP - 1 as c_int) as size_t != 0 as size_t {
        let mut diff: size_t =
            (JUMP as size_t).wrapping_sub(position & (JUMP - 1 as c_int) as size_t);
        available = if diff > available {
            0 as size_t
        } else {
            available.wrapping_sub(diff)
        };
        position = (position as c_ulong).wrapping_add(diff as c_ulong)
            as size_t as size_t;
    }
    position_masked = position & ring_buffer_mask;
    if available > ring_buffer_mask.wrapping_sub(position_masked) {
        available = ring_buffer_mask.wrapping_sub(position_masked);
    }
    PrepareHROLLING(
        self_0,
        BROTLI_FALSE,
        available,
        ringbuffer.offset((position & ring_buffer_mask) as isize),
    );
    (*self_0).next_ix = position;
}
pub const BUCKET_BITS_0: c_int = 20 as c_int;
pub const BUCKET_SWEEP_BITS: c_int = 2 as c_int;
pub const HASH_LEN: c_int = 7 as c_int;
pub const BUCKET_SIZE_6: c_int = (1 as c_int) << BUCKET_BITS;

unsafe extern "C" fn HashBytesH10(mut data: *const uint8_t) -> uint32_t {
    let mut h: uint32_t =
        BrotliUnalignedRead32(data as *const c_void).wrapping_mul(kHashMul32);
    return h >> 32 as c_int - BUCKET_BITS;
}
unsafe extern "C" fn InitializeH10(
    mut common: *mut HasherCommon,
    mut self_0: *mut H10,
    mut params: *const BrotliEncoderParams,
) {
    (*self_0).buckets_ = (*common).extra[0 as c_int as usize] as *mut uint32_t;
    (*self_0).forest_ = (*common).extra[1 as c_int as usize] as *mut uint32_t;
    (*self_0).window_mask_ = ((1 as c_uint) << (*params).lgwin)
        .wrapping_sub(1 as c_uint) as size_t;
    (*self_0).invalid_pos_ = (0 as size_t).wrapping_sub((*self_0).window_mask_) as uint32_t;
}
unsafe extern "C" fn PrepareH10(
    mut self_0: *mut H10,
    mut one_shot: c_int,
    mut input_size: size_t,
    mut data: *const uint8_t,
) {
    let mut invalid_pos: uint32_t = (*self_0).invalid_pos_;
    let mut i: uint32_t = 0;
    let mut buckets: *mut uint32_t = (*self_0).buckets_;
    i = 0 as uint32_t;
    while i < BUCKET_SIZE_6 as uint32_t {
        *buckets.offset(i as isize) = invalid_pos;
        i = i.wrapping_add(1);
    }
}
#[inline(always)]
unsafe extern "C" fn HashMemAllocInBytesH10(
    mut params: *const BrotliEncoderParams,
    mut one_shot: c_int,
    mut input_size: size_t,
    mut alloc_size: *mut size_t,
) {
    let mut num_nodes: size_t = (1 as c_int as size_t) << (*params).lgwin;
    if one_shot != 0 && input_size < num_nodes {
        num_nodes = input_size;
    }
    *alloc_size.offset(0 as c_int as isize) =
        (::core::mem::size_of::<uint32_t>() as usize).wrapping_mul(BUCKET_SIZE_6 as usize)
            as size_t;
    *alloc_size.offset(1 as c_int as isize) = (2 as usize)
        .wrapping_mul(::core::mem::size_of::<uint32_t>() as usize)
        .wrapping_mul(num_nodes as usize)
        as size_t;
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
    let should_reroot_tree: c_int = if max_length >= 128 as size_t {
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
                let fresh11 = matches;
                matches = matches.offset(1);
                InitBackwardMatch(fresh11, backward, len);
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
                if *data.offset(cur_ix_masked.wrapping_add(len) as isize) as c_int
                    > *data.offset(prev_ix_masked.wrapping_add(len) as isize) as c_int
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
unsafe extern "C" fn StitchToPreviousBlockH10(
    mut self_0: *mut H10,
    mut num_bytes: size_t,
    mut position: size_t,
    mut ringbuffer: *const uint8_t,
    mut ringbuffer_mask: size_t,
) {
    if num_bytes >= HashTypeLengthH10().wrapping_sub(1 as size_t)
        && position >= MAX_TREE_COMP_LENGTH as size_t
    {
        let i_start: size_t = position
            .wrapping_sub(MAX_TREE_COMP_LENGTH as size_t)
            .wrapping_add(1 as size_t);
        let i_end: size_t = brotli_min_size_t(position, i_start.wrapping_add(num_bytes)) as size_t;
        let mut i: size_t = 0;
        i = i_start;
        while i < i_end {
            let max_backward: size_t = (*self_0).window_mask_.wrapping_sub(brotli_max_size_t(
                (16 as c_int - 1 as c_int) as size_t,
                position.wrapping_sub(i),
            ) as size_t);
            StoreAndFindMatchesH10(
                self_0,
                ringbuffer,
                i,
                ringbuffer_mask,
                MAX_TREE_COMP_LENGTH as size_t,
                max_backward,
                ::core::ptr::null_mut::<size_t>(),
                ::core::ptr::null_mut::<BackwardMatch>(),
            );
            i = i.wrapping_add(1);
        }
    }
}

pub const JUMP_0: c_int = 4 as c_int;

unsafe extern "C" fn InputBlockSize(mut s: *mut BrotliEncoderStateInternal) -> size_t {
    return (1 as c_int as size_t) << (*s).params.lgblock;
}
unsafe extern "C" fn UnprocessedInputSize(mut s: *mut BrotliEncoderStateInternal) -> uint64_t {
    return (*s).input_pos_.wrapping_sub((*s).last_processed_pos_);
}
unsafe extern "C" fn RemainingInputBlockSize(mut s: *mut BrotliEncoderStateInternal) -> size_t {
    let delta: uint64_t = UnprocessedInputSize(s) as uint64_t;
    let mut block_size: size_t = InputBlockSize(s);
    if delta >= block_size as uint64_t {
        return 0 as size_t;
    }
    return block_size.wrapping_sub(delta as size_t);
}
#[no_mangle]
pub unsafe extern "C" fn BrotliEncoderSetParameter(
    mut state: *mut BrotliEncoderStateInternal,
    mut p: BrotliEncoderParameter,
    mut value: uint32_t,
) -> c_int {
    if (*state).is_initialized_ != 0 {
        return BROTLI_FALSE;
    }
    match p as c_uint {
        0 => {
            (*state).params.mode = value as BrotliEncoderMode;
            return BROTLI_TRUE;
        }
        1 => {
            (*state).params.quality = value as c_int;
            return BROTLI_TRUE;
        }
        2 => {
            (*state).params.lgwin = value as c_int;
            return BROTLI_TRUE;
        }
        3 => {
            (*state).params.lgblock = value as c_int;
            return BROTLI_TRUE;
        }
        4 => {
            if value != 0 as uint32_t && value != 1 as uint32_t {
                return BROTLI_FALSE;
            }
            (*state).params.disable_literal_context_modeling = if value != 0 {
                BROTLI_TRUE
            } else {
                BROTLI_FALSE
            };
            return BROTLI_TRUE;
        }
        5 => {
            (*state).params.size_hint = value as size_t;
            return BROTLI_TRUE;
        }
        6 => {
            (*state).params.large_window = if value != 0 {
                BROTLI_TRUE
            } else {
                BROTLI_FALSE
            };
            return BROTLI_TRUE;
        }
        7 => {
            (*state).params.dist.distance_postfix_bits = value;
            return BROTLI_TRUE;
        }
        8 => {
            (*state).params.dist.num_direct_distance_codes = value;
            return BROTLI_TRUE;
        }
        9 => {
            if value > (1 as uint32_t) << 30 as c_int {
                return BROTLI_FALSE;
            }
            (*state).params.stream_offset = value as size_t;
            return BROTLI_TRUE;
        }
        _ => return BROTLI_FALSE,
    };
}
unsafe extern "C" fn WrapPosition(mut position: uint64_t) -> uint32_t {
    let mut result: uint32_t = position as uint32_t;
    let mut gb: uint64_t = position >> 30 as c_int;
    if gb > 2 as uint64_t {
        result = result & ((1 as uint32_t) << 30 as c_int).wrapping_sub(1 as uint32_t)
            | ((gb.wrapping_sub(1 as uint64_t) & 1 as uint64_t) as uint32_t)
                .wrapping_add(1 as uint32_t)
                << 30 as c_int;
    }
    return result;
}
unsafe extern "C" fn GetBrotliStorage(
    mut s: *mut BrotliEncoderStateInternal,
    mut size: size_t,
) -> *mut uint8_t {
    let mut m: *mut MemoryManager = &raw mut (*s).memory_manager_;
    if (*s).storage_size_ < size {
        BrotliFree(m, (*s).storage_ as *mut c_void);
        (*s).storage_ = ::core::ptr::null_mut::<uint8_t>();
        (*s).storage_ = if size > 0 as size_t {
            BrotliAllocate(
                m,
                size.wrapping_mul(::core::mem::size_of::<uint8_t>() as size_t),
            ) as *mut uint8_t
        } else {
            ::core::ptr::null_mut::<uint8_t>()
        };
        if 0 as c_int != 0 || 0 as c_int != 0 {
            return ::core::ptr::null_mut::<uint8_t>();
        }
        (*s).storage_size_ = size;
    }
    return (*s).storage_;
}
unsafe extern "C" fn HashTableSize(mut max_table_size: size_t, mut input_size: size_t) -> size_t {
    let mut htsize: size_t = 256 as size_t;
    while htsize < max_table_size && htsize < input_size {
        htsize <<= 1 as c_int;
    }
    return htsize;
}
unsafe extern "C" fn GetHashTable(
    mut s: *mut BrotliEncoderStateInternal,
    mut quality: c_int,
    mut input_size: size_t,
    mut table_size: *mut size_t,
) -> *mut c_int {
    let mut m: *mut MemoryManager = &raw mut (*s).memory_manager_;
    let max_table_size: size_t = MaxHashTableSize(quality) as size_t;
    let mut htsize: size_t = HashTableSize(max_table_size, input_size);
    let mut table: *mut c_int = ::core::ptr::null_mut::<c_int>();
    if quality == FAST_ONE_PASS_COMPRESSION_QUALITY {
        if htsize & 0xaaaaa as c_int as size_t == 0 as size_t {
            htsize <<= 1 as c_int;
        }
    }
    if htsize
        <= (::core::mem::size_of::<[c_int; 1024]>() as usize)
            .wrapping_div(::core::mem::size_of::<c_int>() as usize)
    {
        table = &raw mut (*s).small_table_ as *mut c_int;
    } else {
        if htsize > (*s).large_table_size_ {
            (*s).large_table_size_ = htsize;
            BrotliFree(m, (*s).large_table_ as *mut c_void);
            (*s).large_table_ = ::core::ptr::null_mut::<c_int>();
            (*s).large_table_ = if htsize > 0 as size_t {
                BrotliAllocate(
                    m,
                    htsize.wrapping_mul(::core::mem::size_of::<c_int>() as size_t),
                ) as *mut c_int
            } else {
                ::core::ptr::null_mut::<c_int>()
            };
            if 0 as c_int != 0 || 0 as c_int != 0 {
                return ::core::ptr::null_mut::<c_int>();
            }
        }
        table = (*s).large_table_;
    }
    *table_size = htsize;
    memset(
        table as *mut c_void,
        0 as c_int,
        htsize.wrapping_mul(::core::mem::size_of::<c_int>() as size_t),
    );
    return table;
}
unsafe extern "C" fn EncodeWindowBits(
    mut lgwin: c_int,
    mut large_window: c_int,
    mut last_bytes: *mut uint16_t,
    mut last_bytes_bits: *mut uint8_t,
) {
    if large_window != 0 {
        *last_bytes = ((lgwin & 0x3f as c_int) << 8 as c_int
            | 0x11 as c_int) as uint16_t;
        *last_bytes_bits = 14 as uint8_t;
    } else if lgwin == 16 as c_int {
        *last_bytes = 0 as uint16_t;
        *last_bytes_bits = 1 as uint8_t;
    } else if lgwin == 17 as c_int {
        *last_bytes = 1 as uint16_t;
        *last_bytes_bits = 7 as uint8_t;
    } else if lgwin > 17 as c_int {
        *last_bytes = ((lgwin - 17 as c_int) << 1 as c_int
            | 0x1 as c_int) as uint16_t;
        *last_bytes_bits = 4 as uint8_t;
    } else {
        *last_bytes = ((lgwin - 8 as c_int) << 4 as c_int
            | 0x1 as c_int) as uint16_t;
        *last_bytes_bits = 7 as uint8_t;
    };
}
unsafe extern "C" fn InitCommandPrefixCodes(mut s: *mut BrotliOnePassArena) {
    static mut kDefaultCommandDepths: [uint8_t; 128] = [
        0 as c_int as uint8_t,
        4 as c_int as uint8_t,
        4 as c_int as uint8_t,
        5 as c_int as uint8_t,
        6 as c_int as uint8_t,
        6 as c_int as uint8_t,
        7 as c_int as uint8_t,
        7 as c_int as uint8_t,
        7 as c_int as uint8_t,
        7 as c_int as uint8_t,
        7 as c_int as uint8_t,
        8 as c_int as uint8_t,
        8 as c_int as uint8_t,
        8 as c_int as uint8_t,
        8 as c_int as uint8_t,
        8 as c_int as uint8_t,
        0 as c_int as uint8_t,
        0 as c_int as uint8_t,
        0 as c_int as uint8_t,
        4 as c_int as uint8_t,
        4 as c_int as uint8_t,
        4 as c_int as uint8_t,
        4 as c_int as uint8_t,
        4 as c_int as uint8_t,
        5 as c_int as uint8_t,
        5 as c_int as uint8_t,
        6 as c_int as uint8_t,
        6 as c_int as uint8_t,
        6 as c_int as uint8_t,
        6 as c_int as uint8_t,
        7 as c_int as uint8_t,
        7 as c_int as uint8_t,
        7 as c_int as uint8_t,
        7 as c_int as uint8_t,
        10 as c_int as uint8_t,
        10 as c_int as uint8_t,
        10 as c_int as uint8_t,
        10 as c_int as uint8_t,
        10 as c_int as uint8_t,
        10 as c_int as uint8_t,
        0 as c_int as uint8_t,
        4 as c_int as uint8_t,
        4 as c_int as uint8_t,
        5 as c_int as uint8_t,
        5 as c_int as uint8_t,
        5 as c_int as uint8_t,
        6 as c_int as uint8_t,
        6 as c_int as uint8_t,
        7 as c_int as uint8_t,
        8 as c_int as uint8_t,
        8 as c_int as uint8_t,
        9 as c_int as uint8_t,
        10 as c_int as uint8_t,
        10 as c_int as uint8_t,
        10 as c_int as uint8_t,
        10 as c_int as uint8_t,
        10 as c_int as uint8_t,
        10 as c_int as uint8_t,
        10 as c_int as uint8_t,
        10 as c_int as uint8_t,
        10 as c_int as uint8_t,
        10 as c_int as uint8_t,
        10 as c_int as uint8_t,
        10 as c_int as uint8_t,
        5 as c_int as uint8_t,
        0 as c_int as uint8_t,
        0 as c_int as uint8_t,
        0 as c_int as uint8_t,
        0 as c_int as uint8_t,
        0 as c_int as uint8_t,
        0 as c_int as uint8_t,
        0 as c_int as uint8_t,
        0 as c_int as uint8_t,
        0 as c_int as uint8_t,
        0 as c_int as uint8_t,
        0 as c_int as uint8_t,
        0 as c_int as uint8_t,
        0 as c_int as uint8_t,
        0 as c_int as uint8_t,
        0 as c_int as uint8_t,
        6 as c_int as uint8_t,
        6 as c_int as uint8_t,
        6 as c_int as uint8_t,
        6 as c_int as uint8_t,
        6 as c_int as uint8_t,
        6 as c_int as uint8_t,
        5 as c_int as uint8_t,
        5 as c_int as uint8_t,
        5 as c_int as uint8_t,
        5 as c_int as uint8_t,
        5 as c_int as uint8_t,
        5 as c_int as uint8_t,
        4 as c_int as uint8_t,
        4 as c_int as uint8_t,
        4 as c_int as uint8_t,
        4 as c_int as uint8_t,
        4 as c_int as uint8_t,
        4 as c_int as uint8_t,
        4 as c_int as uint8_t,
        5 as c_int as uint8_t,
        5 as c_int as uint8_t,
        5 as c_int as uint8_t,
        5 as c_int as uint8_t,
        5 as c_int as uint8_t,
        5 as c_int as uint8_t,
        6 as c_int as uint8_t,
        6 as c_int as uint8_t,
        7 as c_int as uint8_t,
        7 as c_int as uint8_t,
        7 as c_int as uint8_t,
        8 as c_int as uint8_t,
        10 as c_int as uint8_t,
        12 as c_int as uint8_t,
        12 as c_int as uint8_t,
        12 as c_int as uint8_t,
        12 as c_int as uint8_t,
        12 as c_int as uint8_t,
        12 as c_int as uint8_t,
        12 as c_int as uint8_t,
        12 as c_int as uint8_t,
        12 as c_int as uint8_t,
        12 as c_int as uint8_t,
        12 as c_int as uint8_t,
        12 as c_int as uint8_t,
        0,
        0,
        0,
        0,
    ];
    static mut kDefaultCommandBits: [uint16_t; 128] = [
        0 as c_int as uint16_t,
        0 as c_int as uint16_t,
        8 as c_int as uint16_t,
        9 as c_int as uint16_t,
        3 as c_int as uint16_t,
        35 as c_int as uint16_t,
        7 as c_int as uint16_t,
        71 as c_int as uint16_t,
        39 as c_int as uint16_t,
        103 as c_int as uint16_t,
        23 as c_int as uint16_t,
        47 as c_int as uint16_t,
        175 as c_int as uint16_t,
        111 as c_int as uint16_t,
        239 as c_int as uint16_t,
        31 as c_int as uint16_t,
        0 as c_int as uint16_t,
        0 as c_int as uint16_t,
        0 as c_int as uint16_t,
        4 as c_int as uint16_t,
        12 as c_int as uint16_t,
        2 as c_int as uint16_t,
        10 as c_int as uint16_t,
        6 as c_int as uint16_t,
        13 as c_int as uint16_t,
        29 as c_int as uint16_t,
        11 as c_int as uint16_t,
        43 as c_int as uint16_t,
        27 as c_int as uint16_t,
        59 as c_int as uint16_t,
        87 as c_int as uint16_t,
        55 as c_int as uint16_t,
        15 as c_int as uint16_t,
        79 as c_int as uint16_t,
        319 as c_int as uint16_t,
        831 as c_int as uint16_t,
        191 as c_int as uint16_t,
        703 as c_int as uint16_t,
        447 as c_int as uint16_t,
        959 as c_int as uint16_t,
        0 as c_int as uint16_t,
        14 as c_int as uint16_t,
        1 as c_int as uint16_t,
        25 as c_int as uint16_t,
        5 as c_int as uint16_t,
        21 as c_int as uint16_t,
        19 as c_int as uint16_t,
        51 as c_int as uint16_t,
        119 as c_int as uint16_t,
        159 as c_int as uint16_t,
        95 as c_int as uint16_t,
        223 as c_int as uint16_t,
        479 as c_int as uint16_t,
        991 as c_int as uint16_t,
        63 as c_int as uint16_t,
        575 as c_int as uint16_t,
        127 as c_int as uint16_t,
        639 as c_int as uint16_t,
        383 as c_int as uint16_t,
        895 as c_int as uint16_t,
        255 as c_int as uint16_t,
        767 as c_int as uint16_t,
        511 as c_int as uint16_t,
        1023 as c_int as uint16_t,
        14 as c_int as uint16_t,
        0 as c_int as uint16_t,
        0 as c_int as uint16_t,
        0 as c_int as uint16_t,
        0 as c_int as uint16_t,
        0 as c_int as uint16_t,
        0 as c_int as uint16_t,
        0 as c_int as uint16_t,
        0 as c_int as uint16_t,
        0 as c_int as uint16_t,
        0 as c_int as uint16_t,
        0 as c_int as uint16_t,
        0 as c_int as uint16_t,
        0 as c_int as uint16_t,
        0 as c_int as uint16_t,
        0 as c_int as uint16_t,
        27 as c_int as uint16_t,
        59 as c_int as uint16_t,
        7 as c_int as uint16_t,
        39 as c_int as uint16_t,
        23 as c_int as uint16_t,
        55 as c_int as uint16_t,
        30 as c_int as uint16_t,
        1 as c_int as uint16_t,
        17 as c_int as uint16_t,
        9 as c_int as uint16_t,
        25 as c_int as uint16_t,
        5 as c_int as uint16_t,
        0 as c_int as uint16_t,
        8 as c_int as uint16_t,
        4 as c_int as uint16_t,
        12 as c_int as uint16_t,
        2 as c_int as uint16_t,
        10 as c_int as uint16_t,
        6 as c_int as uint16_t,
        21 as c_int as uint16_t,
        13 as c_int as uint16_t,
        29 as c_int as uint16_t,
        3 as c_int as uint16_t,
        19 as c_int as uint16_t,
        11 as c_int as uint16_t,
        15 as c_int as uint16_t,
        47 as c_int as uint16_t,
        31 as c_int as uint16_t,
        95 as c_int as uint16_t,
        63 as c_int as uint16_t,
        127 as c_int as uint16_t,
        255 as c_int as uint16_t,
        767 as c_int as uint16_t,
        2815 as c_int as uint16_t,
        1791 as c_int as uint16_t,
        3839 as c_int as uint16_t,
        511 as c_int as uint16_t,
        2559 as c_int as uint16_t,
        1535 as c_int as uint16_t,
        3583 as c_int as uint16_t,
        1023 as c_int as uint16_t,
        3071 as c_int as uint16_t,
        2047 as c_int as uint16_t,
        4095 as c_int as uint16_t,
        0,
        0,
        0,
        0,
    ];
    static mut kDefaultCommandCode: [uint8_t; 57] = [
        0xff as c_int as uint8_t,
        0x77 as c_int as uint8_t,
        0xd5 as c_int as uint8_t,
        0xbf as c_int as uint8_t,
        0xe7 as c_int as uint8_t,
        0xde as c_int as uint8_t,
        0xea as c_int as uint8_t,
        0x9e as c_int as uint8_t,
        0x51 as c_int as uint8_t,
        0x5d as c_int as uint8_t,
        0xde as c_int as uint8_t,
        0xc6 as c_int as uint8_t,
        0x70 as c_int as uint8_t,
        0x57 as c_int as uint8_t,
        0xbc as c_int as uint8_t,
        0x58 as c_int as uint8_t,
        0x58 as c_int as uint8_t,
        0x58 as c_int as uint8_t,
        0xd8 as c_int as uint8_t,
        0xd8 as c_int as uint8_t,
        0x58 as c_int as uint8_t,
        0xd5 as c_int as uint8_t,
        0xcb as c_int as uint8_t,
        0x8c as c_int as uint8_t,
        0xea as c_int as uint8_t,
        0xe0 as c_int as uint8_t,
        0xc3 as c_int as uint8_t,
        0x87 as c_int as uint8_t,
        0x1f as c_int as uint8_t,
        0x83 as c_int as uint8_t,
        0xc1 as c_int as uint8_t,
        0x60 as c_int as uint8_t,
        0x1c as c_int as uint8_t,
        0x67 as c_int as uint8_t,
        0xb2 as c_int as uint8_t,
        0xaa as c_int as uint8_t,
        0x6 as c_int as uint8_t,
        0x83 as c_int as uint8_t,
        0xc1 as c_int as uint8_t,
        0x60 as c_int as uint8_t,
        0x30 as c_int as uint8_t,
        0x18 as c_int as uint8_t,
        0xcc as c_int as uint8_t,
        0xa1 as c_int as uint8_t,
        0xce as c_int as uint8_t,
        0x88 as c_int as uint8_t,
        0x54 as c_int as uint8_t,
        0x94 as c_int as uint8_t,
        0x46 as c_int as uint8_t,
        0xe1 as c_int as uint8_t,
        0xb0 as c_int as uint8_t,
        0xd0 as c_int as uint8_t,
        0x4e as c_int as uint8_t,
        0xb2 as c_int as uint8_t,
        0xf7 as c_int as uint8_t,
        0x4 as c_int as uint8_t,
        0 as c_int as uint8_t,
    ];
    static mut kDefaultCommandCodeNumBits: size_t = 448 as size_t;
    memcpy(
        &raw mut (*s).cmd_depth as *mut uint8_t as *mut c_void,
        &raw const kDefaultCommandDepths as *const uint8_t as *const c_void,
        ::core::mem::size_of::<[uint8_t; 128]>() as size_t,
    );
    memcpy(
        &raw mut (*s).cmd_bits as *mut uint16_t as *mut c_void,
        &raw const kDefaultCommandBits as *const uint16_t as *const c_void,
        ::core::mem::size_of::<[uint16_t; 128]>() as size_t,
    );
    memcpy(
        &raw mut (*s).cmd_code as *mut uint8_t as *mut c_void,
        &raw const kDefaultCommandCode as *const uint8_t as *const c_void,
        ::core::mem::size_of::<[uint8_t; 57]>() as size_t,
    );
    (*s).cmd_code_numbits = kDefaultCommandCodeNumBits;
}
unsafe extern "C" fn EstimateEntropy(
    mut population: *const uint32_t,
    mut size: size_t,
) -> c_double {
    let mut total: size_t = 0 as size_t;
    let mut result: c_double = 0 as c_int as c_double;
    let mut i: size_t = 0;
    i = 0 as size_t;
    while i < size {
        let mut p: uint32_t = *population.offset(i as isize);
        total = (total as c_ulong).wrapping_add(p as c_ulong) as size_t
            as size_t;
        result += p as c_double * FastLog2(p as size_t);
        i = i.wrapping_add(1);
    }
    result = total as c_double * FastLog2(total) - result;
    return result;
}
unsafe extern "C" fn ChooseContextMap(
    mut quality: c_int,
    mut bigram_histo: *mut uint32_t,
    mut num_literal_contexts: *mut size_t,
    mut literal_context_map: *mut *const uint32_t,
) {
    static mut kStaticContextMapContinuation: [uint32_t; 64] = [
        1 as c_int as uint32_t,
        1 as c_int as uint32_t,
        2 as c_int as uint32_t,
        2 as c_int as uint32_t,
        0 as c_int as uint32_t,
        0 as c_int as uint32_t,
        0 as c_int as uint32_t,
        0 as c_int as uint32_t,
        0 as c_int as uint32_t,
        0 as c_int as uint32_t,
        0 as c_int as uint32_t,
        0 as c_int as uint32_t,
        0 as c_int as uint32_t,
        0 as c_int as uint32_t,
        0 as c_int as uint32_t,
        0 as c_int as uint32_t,
        0 as c_int as uint32_t,
        0 as c_int as uint32_t,
        0 as c_int as uint32_t,
        0 as c_int as uint32_t,
        0 as c_int as uint32_t,
        0 as c_int as uint32_t,
        0 as c_int as uint32_t,
        0 as c_int as uint32_t,
        0 as c_int as uint32_t,
        0 as c_int as uint32_t,
        0 as c_int as uint32_t,
        0 as c_int as uint32_t,
        0 as c_int as uint32_t,
        0 as c_int as uint32_t,
        0 as c_int as uint32_t,
        0 as c_int as uint32_t,
        0 as c_int as uint32_t,
        0 as c_int as uint32_t,
        0 as c_int as uint32_t,
        0 as c_int as uint32_t,
        0 as c_int as uint32_t,
        0 as c_int as uint32_t,
        0 as c_int as uint32_t,
        0 as c_int as uint32_t,
        0 as c_int as uint32_t,
        0 as c_int as uint32_t,
        0 as c_int as uint32_t,
        0 as c_int as uint32_t,
        0 as c_int as uint32_t,
        0 as c_int as uint32_t,
        0 as c_int as uint32_t,
        0 as c_int as uint32_t,
        0 as c_int as uint32_t,
        0 as c_int as uint32_t,
        0 as c_int as uint32_t,
        0 as c_int as uint32_t,
        0 as c_int as uint32_t,
        0 as c_int as uint32_t,
        0 as c_int as uint32_t,
        0 as c_int as uint32_t,
        0 as c_int as uint32_t,
        0 as c_int as uint32_t,
        0 as c_int as uint32_t,
        0 as c_int as uint32_t,
        0 as c_int as uint32_t,
        0 as c_int as uint32_t,
        0 as c_int as uint32_t,
        0 as c_int as uint32_t,
    ];
    static mut kStaticContextMapSimpleUTF8: [uint32_t; 64] = [
        0 as c_int as uint32_t,
        0 as c_int as uint32_t,
        1 as c_int as uint32_t,
        1 as c_int as uint32_t,
        0 as c_int as uint32_t,
        0 as c_int as uint32_t,
        0 as c_int as uint32_t,
        0 as c_int as uint32_t,
        0 as c_int as uint32_t,
        0 as c_int as uint32_t,
        0 as c_int as uint32_t,
        0 as c_int as uint32_t,
        0 as c_int as uint32_t,
        0 as c_int as uint32_t,
        0 as c_int as uint32_t,
        0 as c_int as uint32_t,
        0 as c_int as uint32_t,
        0 as c_int as uint32_t,
        0 as c_int as uint32_t,
        0 as c_int as uint32_t,
        0 as c_int as uint32_t,
        0 as c_int as uint32_t,
        0 as c_int as uint32_t,
        0 as c_int as uint32_t,
        0 as c_int as uint32_t,
        0 as c_int as uint32_t,
        0 as c_int as uint32_t,
        0 as c_int as uint32_t,
        0 as c_int as uint32_t,
        0 as c_int as uint32_t,
        0 as c_int as uint32_t,
        0 as c_int as uint32_t,
        0 as c_int as uint32_t,
        0 as c_int as uint32_t,
        0 as c_int as uint32_t,
        0 as c_int as uint32_t,
        0 as c_int as uint32_t,
        0 as c_int as uint32_t,
        0 as c_int as uint32_t,
        0 as c_int as uint32_t,
        0 as c_int as uint32_t,
        0 as c_int as uint32_t,
        0 as c_int as uint32_t,
        0 as c_int as uint32_t,
        0 as c_int as uint32_t,
        0 as c_int as uint32_t,
        0 as c_int as uint32_t,
        0 as c_int as uint32_t,
        0 as c_int as uint32_t,
        0 as c_int as uint32_t,
        0 as c_int as uint32_t,
        0 as c_int as uint32_t,
        0 as c_int as uint32_t,
        0 as c_int as uint32_t,
        0 as c_int as uint32_t,
        0 as c_int as uint32_t,
        0 as c_int as uint32_t,
        0 as c_int as uint32_t,
        0 as c_int as uint32_t,
        0 as c_int as uint32_t,
        0 as c_int as uint32_t,
        0 as c_int as uint32_t,
        0 as c_int as uint32_t,
        0 as c_int as uint32_t,
    ];
    let mut monogram_histo: [uint32_t; 3] = [0 as c_int as uint32_t, 0, 0];
    let mut two_prefix_histo: [uint32_t; 6] = [0 as c_int as uint32_t, 0, 0, 0, 0, 0];
    let mut total: size_t = 0;
    let mut i: size_t = 0;
    let mut entropy: [c_double; 4] = [0.; 4];
    i = 0 as size_t;
    while i < 9 as size_t {
        monogram_histo[i.wrapping_rem(3 as size_t) as usize] =
            (monogram_histo[i.wrapping_rem(3 as size_t) as usize] as c_uint)
                .wrapping_add(*bigram_histo.offset(i as isize) as c_uint)
                as uint32_t as uint32_t;
        two_prefix_histo[i.wrapping_rem(6 as size_t) as usize] =
            (two_prefix_histo[i.wrapping_rem(6 as size_t) as usize] as c_uint)
                .wrapping_add(*bigram_histo.offset(i as isize) as c_uint)
                as uint32_t as uint32_t;
        i = i.wrapping_add(1);
    }
    entropy[1 as c_int as usize] =
        EstimateEntropy(&raw mut monogram_histo as *mut uint32_t, 3 as size_t);
    entropy[2 as c_int as usize] =
        EstimateEntropy(&raw mut two_prefix_histo as *mut uint32_t, 3 as size_t)
            + EstimateEntropy(
                (&raw mut two_prefix_histo as *mut uint32_t)
                    .offset(3 as c_int as isize),
                3 as size_t,
            );
    entropy[3 as c_int as usize] = 0 as c_int as c_double;
    i = 0 as size_t;
    while i < 3 as size_t {
        entropy[3 as c_int as usize] += EstimateEntropy(
            bigram_histo.offset((3 as size_t).wrapping_mul(i) as isize),
            3 as size_t,
        );
        i = i.wrapping_add(1);
    }
    total = monogram_histo[0 as c_int as usize]
        .wrapping_add(monogram_histo[1 as c_int as usize])
        .wrapping_add(monogram_histo[2 as c_int as usize]) as size_t;
    entropy[0 as c_int as usize] = 1.0f64 / total as c_double;
    entropy[1 as c_int as usize] *= entropy[0 as c_int as usize];
    entropy[2 as c_int as usize] *= entropy[0 as c_int as usize];
    entropy[3 as c_int as usize] *= entropy[0 as c_int as usize];
    if quality < MIN_QUALITY_FOR_HQ_CONTEXT_MODELING {
        entropy[3 as c_int as usize] = entropy[1 as c_int as usize]
            * 10 as c_int as c_double;
    }
    if entropy[1 as c_int as usize] - entropy[2 as c_int as usize]
        < 0.2f64
        && entropy[1 as c_int as usize] - entropy[3 as c_int as usize]
            < 0.2f64
    {
        *num_literal_contexts = 1 as size_t;
    } else if entropy[2 as c_int as usize] - entropy[3 as c_int as usize]
        < 0.02f64
    {
        *num_literal_contexts = 2 as size_t;
        *literal_context_map = &raw const kStaticContextMapSimpleUTF8 as *const uint32_t;
    } else {
        *num_literal_contexts = 3 as size_t;
        *literal_context_map = &raw const kStaticContextMapContinuation as *const uint32_t;
    };
}
unsafe extern "C" fn ShouldUseComplexStaticContextMap(
    mut input: *const uint8_t,
    mut start_pos: size_t,
    mut length: size_t,
    mut mask: size_t,
    mut quality: c_int,
    mut size_hint: size_t,
    mut num_literal_contexts: *mut size_t,
    mut literal_context_map: *mut *const uint32_t,
    mut arena: *mut uint32_t,
) -> c_int {
    static mut kStaticContextMapComplexUTF8: [uint32_t; 64] = [
        11 as c_int as uint32_t,
        11 as c_int as uint32_t,
        12 as c_int as uint32_t,
        12 as c_int as uint32_t,
        0 as c_int as uint32_t,
        0 as c_int as uint32_t,
        0 as c_int as uint32_t,
        0 as c_int as uint32_t,
        1 as c_int as uint32_t,
        1 as c_int as uint32_t,
        9 as c_int as uint32_t,
        9 as c_int as uint32_t,
        2 as c_int as uint32_t,
        2 as c_int as uint32_t,
        2 as c_int as uint32_t,
        2 as c_int as uint32_t,
        1 as c_int as uint32_t,
        1 as c_int as uint32_t,
        1 as c_int as uint32_t,
        1 as c_int as uint32_t,
        8 as c_int as uint32_t,
        3 as c_int as uint32_t,
        3 as c_int as uint32_t,
        3 as c_int as uint32_t,
        1 as c_int as uint32_t,
        1 as c_int as uint32_t,
        1 as c_int as uint32_t,
        1 as c_int as uint32_t,
        2 as c_int as uint32_t,
        2 as c_int as uint32_t,
        2 as c_int as uint32_t,
        2 as c_int as uint32_t,
        8 as c_int as uint32_t,
        4 as c_int as uint32_t,
        4 as c_int as uint32_t,
        4 as c_int as uint32_t,
        8 as c_int as uint32_t,
        7 as c_int as uint32_t,
        4 as c_int as uint32_t,
        4 as c_int as uint32_t,
        8 as c_int as uint32_t,
        0 as c_int as uint32_t,
        0 as c_int as uint32_t,
        0 as c_int as uint32_t,
        3 as c_int as uint32_t,
        3 as c_int as uint32_t,
        3 as c_int as uint32_t,
        3 as c_int as uint32_t,
        5 as c_int as uint32_t,
        5 as c_int as uint32_t,
        10 as c_int as uint32_t,
        5 as c_int as uint32_t,
        5 as c_int as uint32_t,
        5 as c_int as uint32_t,
        10 as c_int as uint32_t,
        5 as c_int as uint32_t,
        6 as c_int as uint32_t,
        6 as c_int as uint32_t,
        6 as c_int as uint32_t,
        6 as c_int as uint32_t,
        6 as c_int as uint32_t,
        6 as c_int as uint32_t,
        6 as c_int as uint32_t,
        6 as c_int as uint32_t,
    ];
    if size_hint < ((1 as c_int) << 20 as c_int) as size_t {
        return BROTLI_FALSE;
    } else {
        let end_pos: size_t = start_pos.wrapping_add(length);
        let combined_histo: *mut uint32_t = arena;
        let context_histo: *mut uint32_t = arena.offset(32 as c_int as isize);
        let mut total: uint32_t = 0 as uint32_t;
        let mut entropy: [c_double; 3] = [0.; 3];
        let mut i: size_t = 0;
        let mut utf8_lut: ContextLut = (&raw const _kBrotliContextLookupTable as *const uint8_t)
            .offset(((CONTEXT_UTF8 as c_int) << 9 as c_int) as isize)
            as ContextLut;
        memset(
            arena as *mut c_void,
            0 as c_int,
            (::core::mem::size_of::<uint32_t>() as size_t)
                .wrapping_mul(32 as size_t)
                .wrapping_mul((BROTLI_MAX_STATIC_CONTEXTS + 1 as c_int) as size_t),
        );
        while start_pos.wrapping_add(64 as size_t) <= end_pos {
            let stride_end_pos: size_t = start_pos.wrapping_add(64 as size_t);
            let mut prev2: uint8_t = *input.offset((start_pos & mask) as isize);
            let mut prev1: uint8_t =
                *input.offset((start_pos.wrapping_add(1 as size_t) & mask) as isize);
            let mut pos: size_t = 0;
            pos = start_pos.wrapping_add(2 as size_t);
            while pos < stride_end_pos {
                let literal: uint8_t = *input.offset((pos & mask) as isize);
                let context: uint8_t = kStaticContextMapComplexUTF8[(*utf8_lut
                    .offset(prev1 as isize)
                    as c_int
                    | *utf8_lut
                        .offset(256 as c_int as isize)
                        .offset(prev2 as isize) as c_int)
                    as usize] as uint8_t;
                total = total.wrapping_add(1);
                let ref mut fresh9 = *combined_histo
                    .offset((literal as c_int >> 3 as c_int) as isize);
                *fresh9 = (*fresh9).wrapping_add(1);
                let ref mut fresh10 = *context_histo.offset(
                    (((context as c_int) << 5 as c_int)
                        + (literal as c_int >> 3 as c_int))
                        as isize,
                );
                *fresh10 = (*fresh10).wrapping_add(1);
                prev2 = prev1;
                prev1 = literal;
                pos = pos.wrapping_add(1);
            }
            start_pos = (start_pos as c_ulong)
                .wrapping_add(4096 as c_ulong) as size_t
                as size_t;
        }
        entropy[1 as c_int as usize] = EstimateEntropy(combined_histo, 32 as size_t);
        entropy[2 as c_int as usize] =
            0 as c_int as c_double;
        i = 0 as size_t;
        while i < BROTLI_MAX_STATIC_CONTEXTS as size_t {
            entropy[2 as c_int as usize] += EstimateEntropy(
                context_histo.offset((i << 5 as c_int) as isize),
                32 as size_t,
            );
            i = i.wrapping_add(1);
        }
        entropy[0 as c_int as usize] = 1.0f64 / total as c_double;
        entropy[1 as c_int as usize] *= entropy[0 as c_int as usize];
        entropy[2 as c_int as usize] *= entropy[0 as c_int as usize];
        if entropy[2 as c_int as usize] > 3.0f64
            || entropy[1 as c_int as usize] - entropy[2 as c_int as usize]
                < 0.2f64
        {
            return BROTLI_FALSE;
        } else {
            *num_literal_contexts = BROTLI_MAX_STATIC_CONTEXTS as size_t;
            *literal_context_map = &raw const kStaticContextMapComplexUTF8 as *const uint32_t;
            return BROTLI_TRUE;
        }
    };
}
unsafe extern "C" fn DecideOverLiteralContextModeling(
    mut input: *const uint8_t,
    mut start_pos: size_t,
    mut length: size_t,
    mut mask: size_t,
    mut quality: c_int,
    mut size_hint: size_t,
    mut num_literal_contexts: *mut size_t,
    mut literal_context_map: *mut *const uint32_t,
    mut arena: *mut uint32_t,
) {
    if quality < MIN_QUALITY_FOR_CONTEXT_MODELING || length < 64 as size_t {
        return;
    } else if !(ShouldUseComplexStaticContextMap(
        input,
        start_pos,
        length,
        mask,
        quality,
        size_hint,
        num_literal_contexts,
        literal_context_map,
        arena,
    ) != 0)
    {
        let end_pos: size_t = start_pos.wrapping_add(length);
        let bigram_prefix_histo: *mut uint32_t = arena;
        memset(
            bigram_prefix_histo as *mut c_void,
            0 as c_int,
            (::core::mem::size_of::<uint32_t>() as size_t).wrapping_mul(9 as size_t),
        );
        while start_pos.wrapping_add(64 as size_t) <= end_pos {
            static mut lut: [c_int; 4] = [
                0 as c_int,
                0 as c_int,
                1 as c_int,
                2 as c_int,
            ];
            let stride_end_pos: size_t = start_pos.wrapping_add(64 as size_t);
            let mut prev: c_int =
                lut[(*input.offset((start_pos & mask) as isize) as c_int
                    >> 6 as c_int) as usize]
                    * 3 as c_int;
            let mut pos: size_t = 0;
            pos = start_pos.wrapping_add(1 as size_t);
            while pos < stride_end_pos {
                let literal: uint8_t = *input.offset((pos & mask) as isize);
                let ref mut fresh8 = *bigram_prefix_histo.offset(
                    (prev
                        + lut[(literal as c_int >> 6 as c_int) as usize])
                        as isize,
                );
                *fresh8 = (*fresh8).wrapping_add(1);
                prev = lut[(literal as c_int >> 6 as c_int) as usize]
                    * 3 as c_int;
                pos = pos.wrapping_add(1);
            }
            start_pos = (start_pos as c_ulong)
                .wrapping_add(4096 as c_ulong) as size_t
                as size_t;
        }
        ChooseContextMap(
            quality,
            bigram_prefix_histo.offset(0 as c_int as isize) as *mut uint32_t,
            num_literal_contexts,
            literal_context_map,
        );
    }
}
unsafe extern "C" fn ShouldCompress(
    mut data: *const uint8_t,
    mask: size_t,
    last_flush_pos: uint64_t,
    bytes: size_t,
    num_literals: size_t,
    num_commands: size_t,
) -> c_int {
    if bytes <= 2 as size_t {
        return BROTLI_FALSE;
    }
    if num_commands < (bytes >> 8 as c_int).wrapping_add(2 as size_t) {
        if num_literals as c_double > 0.99f64 * bytes as c_double {
            let mut literal_histo: [uint32_t; 256] = [
                0 as c_int as uint32_t,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
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
            static mut kSampleRate: uint32_t = 13 as uint32_t;
            static mut kInvSampleRate: c_double = 1.0f64 / 13.0f64;
            static mut kMinEntropy: c_double = 7.92f64;
            let bit_cost_threshold: c_double =
                bytes as c_double * kMinEntropy * kInvSampleRate;
            let mut t: size_t = bytes
                .wrapping_add(kSampleRate as size_t)
                .wrapping_sub(1 as size_t)
                .wrapping_div(kSampleRate as size_t);
            let mut pos: uint32_t = last_flush_pos as uint32_t;
            let mut i: size_t = 0;
            i = 0 as size_t;
            while i < t {
                literal_histo[*data.offset((pos as size_t & mask) as isize) as usize] =
                    literal_histo[*data.offset((pos as size_t & mask) as isize) as usize]
                        .wrapping_add(1);
                pos = (pos as c_uint).wrapping_add(kSampleRate as c_uint)
                    as uint32_t as uint32_t;
                i = i.wrapping_add(1);
            }
            if BrotliBitsEntropy(&raw mut literal_histo as *mut uint32_t, 256 as size_t)
                > bit_cost_threshold
            {
                return BROTLI_FALSE;
            }
        }
    }
    return BROTLI_TRUE;
}
unsafe extern "C" fn ChooseContextMode(
    mut params: *const BrotliEncoderParams,
    mut data: *const uint8_t,
    pos: size_t,
    mask: size_t,
    length: size_t,
) -> ContextType {
    if (*params).quality >= MIN_QUALITY_FOR_HQ_BLOCK_SPLITTING
        && BrotliIsMostlyUTF8(data, pos, mask, length, kMinUTF8Ratio) == 0
    {
        return CONTEXT_SIGNED;
    }
    return CONTEXT_UTF8;
}
unsafe extern "C" fn WriteMetaBlockInternal(
    mut m: *mut MemoryManager,
    mut data: *const uint8_t,
    mask: size_t,
    last_flush_pos: uint64_t,
    bytes: size_t,
    is_last: c_int,
    mut literal_context_mode: ContextType,
    mut params: *const BrotliEncoderParams,
    prev_byte: uint8_t,
    prev_byte2: uint8_t,
    num_literals: size_t,
    num_commands: size_t,
    mut commands: *mut Command,
    mut saved_dist_cache: *const c_int,
    mut dist_cache: *mut c_int,
    mut storage_ix: *mut size_t,
    mut storage: *mut uint8_t,
) {
    let wrapped_last_flush_pos: uint32_t = WrapPosition(last_flush_pos) as uint32_t;
    let mut last_bytes: uint16_t = 0;
    let mut last_bytes_bits: uint8_t = 0;
    let mut literal_context_lut: ContextLut = (&raw const _kBrotliContextLookupTable
        as *const uint8_t)
        .offset(((literal_context_mode as c_uint) << 9 as c_int) as isize)
        as ContextLut;
    let mut block_params: BrotliEncoderParams = *params;
    if bytes == 0 as size_t {
        BrotliWriteBits(2 as size_t, 3 as uint64_t, storage_ix, storage);
        *storage_ix =
            (*storage_ix).wrapping_add(7 as size_t) & !(7 as c_uint) as size_t;
        return;
    }
    if ShouldCompress(
        data,
        mask,
        last_flush_pos,
        bytes,
        num_literals,
        num_commands,
    ) == 0
    {
        memcpy(
            dist_cache as *mut c_void,
            saved_dist_cache as *const c_void,
            (4 as size_t).wrapping_mul(::core::mem::size_of::<c_int>() as size_t),
        );
        BrotliStoreUncompressedMetaBlock(
            is_last,
            data,
            wrapped_last_flush_pos as size_t,
            mask,
            bytes,
            storage_ix,
            storage,
        );
        return;
    }
    last_bytes = ((*storage.offset(1 as c_int as isize) as c_int)
        << 8 as c_int
        | *storage.offset(0 as c_int as isize) as c_int)
        as uint16_t;
    last_bytes_bits = *storage_ix as uint8_t;
    if (*params).quality <= MAX_QUALITY_FOR_STATIC_ENTROPY_CODES {
        BrotliStoreMetaBlockFast(
            m,
            data,
            wrapped_last_flush_pos as size_t,
            bytes,
            mask,
            is_last,
            params,
            commands,
            num_commands,
            storage_ix,
            storage,
        );
        if 0 as c_int != 0 {
            return;
        }
    } else if (*params).quality < MIN_QUALITY_FOR_BLOCK_SPLIT {
        BrotliStoreMetaBlockTrivial(
            m,
            data,
            wrapped_last_flush_pos as size_t,
            bytes,
            mask,
            is_last,
            params,
            commands,
            num_commands,
            storage_ix,
            storage,
        );
        if 0 as c_int != 0 {
            return;
        }
    } else {
        let mut mb: MetaBlockSplit = MetaBlockSplit {
            literal_split: BlockSplit {
                num_types: 0,
                num_blocks: 0,
                types: ::core::ptr::null_mut::<uint8_t>(),
                lengths: ::core::ptr::null_mut::<uint32_t>(),
                types_alloc_size: 0,
                lengths_alloc_size: 0,
            },
            command_split: BlockSplit {
                num_types: 0,
                num_blocks: 0,
                types: ::core::ptr::null_mut::<uint8_t>(),
                lengths: ::core::ptr::null_mut::<uint32_t>(),
                types_alloc_size: 0,
                lengths_alloc_size: 0,
            },
            distance_split: BlockSplit {
                num_types: 0,
                num_blocks: 0,
                types: ::core::ptr::null_mut::<uint8_t>(),
                lengths: ::core::ptr::null_mut::<uint32_t>(),
                types_alloc_size: 0,
                lengths_alloc_size: 0,
            },
            literal_context_map: ::core::ptr::null_mut::<uint32_t>(),
            literal_context_map_size: 0,
            distance_context_map: ::core::ptr::null_mut::<uint32_t>(),
            distance_context_map_size: 0,
            literal_histograms: ::core::ptr::null_mut::<HistogramLiteral>(),
            literal_histograms_size: 0,
            command_histograms: ::core::ptr::null_mut::<HistogramCommand>(),
            command_histograms_size: 0,
            distance_histograms: ::core::ptr::null_mut::<HistogramDistance>(),
            distance_histograms_size: 0,
        };
        InitMetaBlockSplit(&raw mut mb);
        if (*params).quality < MIN_QUALITY_FOR_HQ_BLOCK_SPLITTING {
            let mut num_literal_contexts: size_t = 1 as size_t;
            let mut literal_context_map: *const uint32_t = ::core::ptr::null::<uint32_t>();
            if (*params).disable_literal_context_modeling == 0 {
                let mut arena: *mut uint32_t = if 32 as c_int
                    * (13 as c_int + 1 as c_int)
                    > 0 as c_int
                {
                    BrotliAllocate(
                        m,
                        ((32 as c_int
                            * (13 as c_int + 1 as c_int))
                            as size_t)
                            .wrapping_mul(::core::mem::size_of::<uint32_t>() as size_t),
                    ) as *mut uint32_t
                } else {
                    ::core::ptr::null_mut::<uint32_t>()
                };
                if 0 as c_int != 0 || 0 as c_int != 0 {
                    return;
                }
                DecideOverLiteralContextModeling(
                    data,
                    wrapped_last_flush_pos as size_t,
                    bytes,
                    mask,
                    (*params).quality,
                    (*params).size_hint,
                    &raw mut num_literal_contexts,
                    &raw mut literal_context_map,
                    arena,
                );
                BrotliFree(m, arena as *mut c_void);
                arena = ::core::ptr::null_mut::<uint32_t>();
            }
            BrotliBuildMetaBlockGreedy(
                m,
                data,
                wrapped_last_flush_pos as size_t,
                mask,
                prev_byte,
                prev_byte2,
                literal_context_lut,
                num_literal_contexts,
                literal_context_map,
                commands,
                num_commands,
                &raw mut mb,
            );
            if 0 as c_int != 0 {
                return;
            }
        } else {
            BrotliBuildMetaBlock(
                m,
                data,
                wrapped_last_flush_pos as size_t,
                mask,
                &raw mut block_params,
                prev_byte,
                prev_byte2,
                commands,
                num_commands,
                literal_context_mode,
                &raw mut mb,
            );
            if 0 as c_int != 0 {
                return;
            }
        }
        if (*params).quality >= MIN_QUALITY_FOR_OPTIMIZE_HISTOGRAMS {
            BrotliOptimizeHistograms(block_params.dist.alphabet_size_limit, &raw mut mb);
        }
        BrotliStoreMetaBlock(
            m,
            data,
            wrapped_last_flush_pos as size_t,
            bytes,
            mask,
            prev_byte,
            prev_byte2,
            is_last,
            &raw mut block_params,
            literal_context_mode,
            commands,
            num_commands,
            &raw mut mb,
            storage_ix,
            storage,
        );
        if 0 as c_int != 0 {
            return;
        }
        DestroyMetaBlockSplit(m, &raw mut mb);
    }
    if bytes.wrapping_add(4 as size_t) < *storage_ix >> 3 as c_int {
        memcpy(
            dist_cache as *mut c_void,
            saved_dist_cache as *const c_void,
            (4 as size_t).wrapping_mul(::core::mem::size_of::<c_int>() as size_t),
        );
        *storage.offset(0 as c_int as isize) = last_bytes as uint8_t;
        *storage.offset(1 as c_int as isize) =
            (last_bytes as c_int >> 8 as c_int) as uint8_t;
        *storage_ix = last_bytes_bits as size_t;
        BrotliStoreUncompressedMetaBlock(
            is_last,
            data,
            wrapped_last_flush_pos as size_t,
            mask,
            bytes,
            storage_ix,
            storage,
        );
    }
}
unsafe extern "C" fn ChooseDistanceParams(mut params: *mut BrotliEncoderParams) {
    let mut distance_postfix_bits: uint32_t = 0 as uint32_t;
    let mut num_direct_distance_codes: uint32_t = 0 as uint32_t;
    if (*params).quality >= MIN_QUALITY_FOR_NONZERO_DISTANCE_PARAMS {
        let mut ndirect_msb: uint32_t = 0;
        if (*params).mode as c_uint
            == BROTLI_MODE_FONT as c_int as c_uint
        {
            distance_postfix_bits = 1 as uint32_t;
            num_direct_distance_codes = 12 as uint32_t;
        } else {
            distance_postfix_bits = (*params).dist.distance_postfix_bits;
            num_direct_distance_codes = (*params).dist.num_direct_distance_codes;
        }
        ndirect_msb = num_direct_distance_codes >> distance_postfix_bits & 0xf as uint32_t;
        if distance_postfix_bits > BROTLI_MAX_NPOSTFIX as uint32_t
            || num_direct_distance_codes > BROTLI_MAX_NDIRECT as uint32_t
            || ndirect_msb << distance_postfix_bits != num_direct_distance_codes
        {
            distance_postfix_bits = 0 as uint32_t;
            num_direct_distance_codes = 0 as uint32_t;
        }
    }
    BrotliInitDistanceParams(
        &raw mut (*params).dist,
        distance_postfix_bits,
        num_direct_distance_codes,
        (*params).large_window,
    );
}
unsafe extern "C" fn EnsureInitialized(
    mut s: *mut BrotliEncoderStateInternal,
) -> c_int {
    let mut m: *mut MemoryManager = &raw mut (*s).memory_manager_;
    if 0 as c_int != 0 {
        return BROTLI_FALSE;
    }
    if (*s).is_initialized_ != 0 {
        return BROTLI_TRUE;
    }
    (*s).last_bytes_bits_ = 0 as uint8_t;
    (*s).last_bytes_ = 0 as uint16_t;
    (*s).flint_ = BROTLI_FLINT_DONE as c_int as int8_t;
    (*s).remaining_metadata_bytes_ = BROTLI_UINT32_MAX;
    SanitizeParams(&raw mut (*s).params);
    (*s).params.lgblock = ComputeLgBlock(&raw mut (*s).params);
    ChooseDistanceParams(&raw mut (*s).params);
    if (*s).params.stream_offset != 0 as size_t {
        (*s).flint_ = BROTLI_FLINT_NEEDS_2_BYTES as c_int as int8_t;
        (*s).dist_cache_[0 as c_int as usize] = -(16 as c_int);
        (*s).dist_cache_[1 as c_int as usize] = -(16 as c_int);
        (*s).dist_cache_[2 as c_int as usize] = -(16 as c_int);
        (*s).dist_cache_[3 as c_int as usize] = -(16 as c_int);
        memcpy(
            &raw mut (*s).saved_dist_cache_ as *mut c_int as *mut c_void,
            &raw mut (*s).dist_cache_ as *mut c_int as *const c_void,
            ::core::mem::size_of::<[c_int; 4]>() as size_t,
        );
    }
    RingBufferSetup(&raw mut (*s).params, &raw mut (*s).ringbuffer_);
    let mut lgwin: c_int = (*s).params.lgwin;
    if (*s).params.quality == FAST_ONE_PASS_COMPRESSION_QUALITY
        || (*s).params.quality == FAST_TWO_PASS_COMPRESSION_QUALITY
    {
        lgwin = brotli_max_int(lgwin, 18 as c_int);
    }
    if (*s).params.stream_offset == 0 as size_t {
        EncodeWindowBits(
            lgwin,
            (*s).params.large_window,
            &raw mut (*s).last_bytes_,
            &raw mut (*s).last_bytes_bits_,
        );
    } else {
        (*s).params.stream_offset = brotli_min_size_t(
            (*s).params.stream_offset,
            ((1 as c_int as size_t) << lgwin).wrapping_sub(16 as size_t),
        );
    }
    if (*s).params.quality == FAST_ONE_PASS_COMPRESSION_QUALITY {
        (*s).one_pass_arena_ = if 1 as c_int > 0 as c_int {
            BrotliAllocate(
                m,
                (1 as size_t).wrapping_mul(::core::mem::size_of::<BrotliOnePassArena>() as size_t),
            ) as *mut BrotliOnePassArena
        } else {
            ::core::ptr::null_mut::<BrotliOnePassArena>()
        };
        if 0 as c_int != 0 {
            return BROTLI_FALSE;
        }
        InitCommandPrefixCodes((*s).one_pass_arena_);
    } else if (*s).params.quality == FAST_TWO_PASS_COMPRESSION_QUALITY {
        (*s).two_pass_arena_ = if 1 as c_int > 0 as c_int {
            BrotliAllocate(
                m,
                (1 as size_t).wrapping_mul(::core::mem::size_of::<BrotliTwoPassArena>() as size_t),
            ) as *mut BrotliTwoPassArena
        } else {
            ::core::ptr::null_mut::<BrotliTwoPassArena>()
        };
        if 0 as c_int != 0 {
            return BROTLI_FALSE;
        }
    }
    (*s).is_initialized_ = BROTLI_TRUE;
    return BROTLI_TRUE;
}
unsafe extern "C" fn BrotliEncoderInitParams(mut params: *mut BrotliEncoderParams) {
    (*params).mode = BROTLI_MODE_GENERIC;
    (*params).large_window = BROTLI_FALSE;
    (*params).quality = BROTLI_DEFAULT_QUALITY;
    (*params).lgwin = BROTLI_DEFAULT_WINDOW;
    (*params).lgblock = 0 as c_int;
    (*params).stream_offset = 0 as size_t;
    (*params).size_hint = 0 as size_t;
    (*params).disable_literal_context_modeling = BROTLI_FALSE;
    BrotliInitSharedEncoderDictionary(&raw mut (*params).dictionary);
    (*params).dist.distance_postfix_bits = 0 as uint32_t;
    (*params).dist.num_direct_distance_codes = 0 as uint32_t;
    (*params).dist.alphabet_size_max = ((BROTLI_NUM_DISTANCE_SHORT_CODES + 0 as c_int)
        as c_uint)
        .wrapping_add(
            (24 as c_uint) << 0 as c_int + 1 as c_int,
        ) as uint32_t;
    (*params).dist.alphabet_size_limit = (*params).dist.alphabet_size_max;
    (*params).dist.max_distance = BROTLI_MAX_DISTANCE as size_t;
}
unsafe extern "C" fn BrotliEncoderCleanupParams(
    mut m: *mut MemoryManager,
    mut params: *mut BrotliEncoderParams,
) {
    BrotliCleanupSharedEncoderDictionary(m, &raw mut (*params).dictionary);
}
unsafe extern "C" fn BrotliEncoderInitState(mut s: *mut BrotliEncoderStateInternal) {
    BrotliEncoderInitParams(&raw mut (*s).params);
    (*s).input_pos_ = 0 as uint64_t;
    (*s).num_commands_ = 0 as size_t;
    (*s).num_literals_ = 0 as size_t;
    (*s).last_insert_len_ = 0 as size_t;
    (*s).last_flush_pos_ = 0 as uint64_t;
    (*s).last_processed_pos_ = 0 as uint64_t;
    (*s).prev_byte_ = 0 as uint8_t;
    (*s).prev_byte2_ = 0 as uint8_t;
    (*s).storage_size_ = 0 as size_t;
    (*s).storage_ = ::core::ptr::null_mut::<uint8_t>();
    HasherInit(&raw mut (*s).hasher_);
    (*s).large_table_ = ::core::ptr::null_mut::<c_int>();
    (*s).large_table_size_ = 0 as size_t;
    (*s).one_pass_arena_ = ::core::ptr::null_mut::<BrotliOnePassArena>();
    (*s).two_pass_arena_ = ::core::ptr::null_mut::<BrotliTwoPassArena>();
    (*s).command_buf_ = ::core::ptr::null_mut::<uint32_t>();
    (*s).literal_buf_ = ::core::ptr::null_mut::<uint8_t>();
    (*s).total_in_ = 0 as uint64_t;
    (*s).next_out_ = ::core::ptr::null_mut::<uint8_t>();
    (*s).available_out_ = 0 as size_t;
    (*s).total_out_ = 0 as uint64_t;
    (*s).stream_state_ = BROTLI_STREAM_PROCESSING;
    (*s).is_last_block_emitted_ = BROTLI_FALSE;
    (*s).is_initialized_ = BROTLI_FALSE;
    RingBufferInit(&raw mut (*s).ringbuffer_);
    (*s).commands_ = ::core::ptr::null_mut::<Command>();
    (*s).cmd_alloc_size_ = 0 as size_t;
    (*s).dist_cache_[0 as c_int as usize] = 4 as c_int;
    (*s).dist_cache_[1 as c_int as usize] = 11 as c_int;
    (*s).dist_cache_[2 as c_int as usize] = 15 as c_int;
    (*s).dist_cache_[3 as c_int as usize] = 16 as c_int;
    memcpy(
        &raw mut (*s).saved_dist_cache_ as *mut c_int as *mut c_void,
        &raw mut (*s).dist_cache_ as *mut c_int as *const c_void,
        ::core::mem::size_of::<[c_int; 4]>() as size_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn BrotliEncoderCreateInstance(
    mut alloc_func: brotli_alloc_func,
    mut free_func: brotli_free_func,
    mut opaque: *mut c_void,
) -> *mut BrotliEncoderState {
    let mut state: *mut BrotliEncoderStateInternal =
        ::core::ptr::null_mut::<BrotliEncoderStateInternal>();
    let mut healthy: c_int = BrotliEncoderEnsureStaticInit();
    if healthy == 0 {
        return ::core::ptr::null_mut::<BrotliEncoderState>();
    }
    state = BrotliBootstrapAlloc(
        ::core::mem::size_of::<BrotliEncoderStateInternal>() as size_t,
        alloc_func,
        free_func,
        opaque,
    ) as *mut BrotliEncoderStateInternal;
    if state.is_null() {
        return ::core::ptr::null_mut::<BrotliEncoderState>();
    }
    BrotliInitMemoryManager(
        &raw mut (*state).memory_manager_,
        alloc_func,
        free_func,
        opaque,
    );
    BrotliEncoderInitState(state);
    return state as *mut BrotliEncoderState;
}
unsafe extern "C" fn BrotliEncoderCleanupState(mut s: *mut BrotliEncoderStateInternal) {
    let mut m: *mut MemoryManager = &raw mut (*s).memory_manager_;
    if 0 as c_int != 0 {
        BrotliWipeOutMemoryManager(m);
        return;
    }
    BrotliFree(m, (*s).storage_ as *mut c_void);
    (*s).storage_ = ::core::ptr::null_mut::<uint8_t>();
    BrotliFree(m, (*s).commands_ as *mut c_void);
    (*s).commands_ = ::core::ptr::null_mut::<Command>();
    RingBufferFree(m, &raw mut (*s).ringbuffer_);
    DestroyHasher(m, &raw mut (*s).hasher_);
    BrotliFree(m, (*s).large_table_ as *mut c_void);
    (*s).large_table_ = ::core::ptr::null_mut::<c_int>();
    BrotliFree(m, (*s).one_pass_arena_ as *mut c_void);
    (*s).one_pass_arena_ = ::core::ptr::null_mut::<BrotliOnePassArena>();
    BrotliFree(m, (*s).two_pass_arena_ as *mut c_void);
    (*s).two_pass_arena_ = ::core::ptr::null_mut::<BrotliTwoPassArena>();
    BrotliFree(m, (*s).command_buf_ as *mut c_void);
    (*s).command_buf_ = ::core::ptr::null_mut::<uint32_t>();
    BrotliFree(m, (*s).literal_buf_ as *mut c_void);
    (*s).literal_buf_ = ::core::ptr::null_mut::<uint8_t>();
    BrotliEncoderCleanupParams(m, &raw mut (*s).params);
}
#[no_mangle]
pub unsafe extern "C" fn BrotliEncoderDestroyInstance(mut state: *mut BrotliEncoderStateInternal) {
    if state.is_null() {
        return;
    } else {
        BrotliEncoderCleanupState(state);
        BrotliBootstrapFree(
            state as *mut c_void,
            &raw mut (*state).memory_manager_,
        );
    };
}
unsafe extern "C" fn CopyInputToRingBuffer(
    mut s: *mut BrotliEncoderStateInternal,
    input_size: size_t,
    mut input_buffer: *const uint8_t,
) {
    let mut ringbuffer_: *mut RingBuffer = &raw mut (*s).ringbuffer_;
    let mut m: *mut MemoryManager = &raw mut (*s).memory_manager_;
    RingBufferWrite(m, input_buffer, input_size, ringbuffer_);
    if 0 as c_int != 0 {
        return;
    }
    (*s).input_pos_ = ((*s).input_pos_ as c_ulong)
        .wrapping_add(input_size as c_ulong) as uint64_t
        as uint64_t;
    if (*ringbuffer_).pos_ <= (*ringbuffer_).mask_ {
        memset(
            (*ringbuffer_).buffer_.offset((*ringbuffer_).pos_ as isize) as *mut c_void,
            0 as c_int,
            7 as size_t,
        );
    }
}
unsafe extern "C" fn UpdateLastProcessedPos(
    mut s: *mut BrotliEncoderStateInternal,
) -> c_int {
    let mut wrapped_last_processed_pos: uint32_t = WrapPosition((*s).last_processed_pos_);
    let mut wrapped_input_pos: uint32_t = WrapPosition((*s).input_pos_);
    (*s).last_processed_pos_ = (*s).input_pos_;
    return if wrapped_input_pos < wrapped_last_processed_pos {
        BROTLI_TRUE
    } else {
        BROTLI_FALSE
    };
}
unsafe extern "C" fn ExtendLastCommand(
    mut s: *mut BrotliEncoderStateInternal,
    mut bytes: *mut uint32_t,
    mut wrapped_last_processed_pos: *mut uint32_t,
) {
    let mut last_command: *mut Command =
        (*s).commands_
            .offset((*s).num_commands_.wrapping_sub(1 as size_t) as isize) as *mut Command;
    let mut data: *const uint8_t = (*s).ringbuffer_.buffer_;
    let mask: uint32_t = (*s).ringbuffer_.mask_;
    let mut max_backward_distance: uint64_t = ((1 as c_int as uint64_t)
        << (*s).params.lgwin)
        .wrapping_sub(BROTLI_WINDOW_GAP as uint64_t);
    let mut last_copy_len: uint64_t =
        ((*last_command).copy_len_ & 0x1ffffff as uint32_t) as uint64_t;
    let mut last_processed_pos: uint64_t = (*s).last_processed_pos_.wrapping_sub(last_copy_len);
    let mut max_distance: uint64_t = if last_processed_pos < max_backward_distance {
        last_processed_pos
    } else {
        max_backward_distance
    };
    let mut cmd_dist: uint64_t = (*s).dist_cache_[0 as c_int as usize] as uint64_t;
    let mut distance_code: uint32_t =
        CommandRestoreDistanceCode(last_command, &raw mut (*s).params.dist);
    let mut dict: *const CompoundDictionary = &raw mut (*s).params.dictionary.compound;
    let mut compound_dictionary_size: size_t = (*dict).total_size;
    if distance_code < BROTLI_NUM_DISTANCE_SHORT_CODES as uint32_t
        || distance_code
            .wrapping_sub((BROTLI_NUM_DISTANCE_SHORT_CODES - 1 as c_int) as uint32_t)
            as uint64_t
            == cmd_dist
    {
        if cmd_dist <= max_distance {
            while *bytes != 0 as uint32_t
                && *data.offset((*wrapped_last_processed_pos & mask) as isize) as c_int
                    == *data.offset(
                        ((*wrapped_last_processed_pos as uint64_t).wrapping_sub(cmd_dist)
                            & mask as uint64_t) as isize,
                    ) as c_int
            {
                (*last_command).copy_len_ = (*last_command).copy_len_.wrapping_add(1);
                *bytes = (*bytes).wrapping_sub(1);
                *wrapped_last_processed_pos = (*wrapped_last_processed_pos).wrapping_add(1);
            }
        } else if cmd_dist
            .wrapping_sub(max_distance)
            .wrapping_sub(1 as uint64_t)
            < compound_dictionary_size as uint64_t
            && last_copy_len < cmd_dist.wrapping_sub(max_distance)
        {
            let mut address: size_t = compound_dictionary_size
                .wrapping_sub(cmd_dist.wrapping_sub(max_distance) as size_t)
                .wrapping_add(last_copy_len as size_t);
            let mut br_index: size_t = 0 as size_t;
            let mut br_offset: size_t = 0;
            let mut chunk: *const uint8_t = ::core::ptr::null::<uint8_t>();
            let mut chunk_length: size_t = 0;
            while address >= (*dict).chunk_offsets[br_index.wrapping_add(1 as size_t) as usize] {
                br_index = br_index.wrapping_add(1);
            }
            br_offset = address.wrapping_sub((*dict).chunk_offsets[br_index as usize]);
            chunk = (*dict).chunk_source[br_index as usize];
            chunk_length = (*dict).chunk_offsets[br_index.wrapping_add(1 as size_t) as usize]
                .wrapping_sub((*dict).chunk_offsets[br_index as usize]);
            while *bytes != 0 as uint32_t
                && *data.offset((*wrapped_last_processed_pos & mask) as isize) as c_int
                    == *chunk.offset(br_offset as isize) as c_int
            {
                (*last_command).copy_len_ = (*last_command).copy_len_.wrapping_add(1);
                *bytes = (*bytes).wrapping_sub(1);
                *wrapped_last_processed_pos = (*wrapped_last_processed_pos).wrapping_add(1);
                br_offset = br_offset.wrapping_add(1);
                if !(br_offset == chunk_length) {
                    continue;
                }
                br_index = br_index.wrapping_add(1);
                br_offset = 0 as size_t;
                if !(br_index != (*dict).num_chunks) {
                    break;
                }
                chunk = (*dict).chunk_source[br_index as usize];
                chunk_length = (*dict).chunk_offsets[br_index.wrapping_add(1 as size_t) as usize]
                    .wrapping_sub((*dict).chunk_offsets[br_index as usize]);
            }
        }
        GetLengthCode(
            (*last_command).insert_len_ as size_t,
            (((*last_command).copy_len_ & 0x1ffffff as uint32_t) as c_int
                + ((*last_command).copy_len_ >> 25 as c_int) as c_int)
                as size_t,
            if (*last_command).dist_prefix_ as c_int & 0x3ff as c_int
                == 0 as c_int
            {
                BROTLI_TRUE
            } else {
                BROTLI_FALSE
            },
            &raw mut (*last_command).cmd_prefix_,
        );
    }
}
unsafe extern "C" fn EncodeData(
    mut s: *mut BrotliEncoderStateInternal,
    is_last: c_int,
    force_flush: c_int,
    mut out_size: *mut size_t,
    mut output: *mut *mut uint8_t,
) -> c_int {
    let delta: uint64_t = UnprocessedInputSize(s) as uint64_t;
    let mut bytes: uint32_t = delta as uint32_t;
    let mut wrapped_last_processed_pos: uint32_t = WrapPosition((*s).last_processed_pos_);
    let mut data: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut mask: uint32_t = 0;
    let mut m: *mut MemoryManager = &raw mut (*s).memory_manager_;
    let mut literal_context_mode: ContextType = CONTEXT_LSB6;
    let mut literal_context_lut: ContextLut = ::core::ptr::null::<uint8_t>();
    let mut fast_compress: c_int = ((*s).params.quality
        == FAST_ONE_PASS_COMPRESSION_QUALITY
        || (*s).params.quality == FAST_TWO_PASS_COMPRESSION_QUALITY)
        as c_int;
    data = (*s).ringbuffer_.buffer_;
    mask = (*s).ringbuffer_.mask_;
    if delta == 0 as uint64_t {
        if data.is_null() {
            if is_last != 0 {
                (*s).last_bytes_ = ((*s).last_bytes_ as c_int
                    | ((3 as c_uint) << (*s).last_bytes_bits_ as c_int)
                        as uint16_t as c_int)
                    as uint16_t;
                (*s).last_bytes_bits_ = ((*s).last_bytes_bits_ as c_uint)
                    .wrapping_add(2 as c_uint)
                    as uint8_t;
                (*s).tiny_buf_.u8_0[0 as c_int as usize] = (*s).last_bytes_ as uint8_t;
                (*s).tiny_buf_.u8_0[1 as c_int as usize] =
                    ((*s).last_bytes_ as c_int >> 8 as c_int) as uint8_t;
                *output = &raw mut (*s).tiny_buf_.u8_0 as *mut uint8_t;
                *out_size = (((*s).last_bytes_bits_ as c_uint)
                    .wrapping_add(7 as c_uint)
                    >> 3 as c_uint) as size_t;
                return BROTLI_TRUE;
            } else {
                *out_size = 0 as size_t;
                return BROTLI_TRUE;
            }
        } else if is_last == 0 && (force_flush == 0 || fast_compress != 0) {
            *out_size = 0 as size_t;
            return BROTLI_TRUE;
        }
    }
    if (*s).params.quality > (*s).params.dictionary.max_quality {
        return BROTLI_FALSE;
    }
    if (*s).is_last_block_emitted_ != 0 {
        return BROTLI_FALSE;
    }
    if is_last != 0 {
        (*s).is_last_block_emitted_ = BROTLI_TRUE;
    }
    if delta > InputBlockSize(s) as uint64_t {
        return BROTLI_FALSE;
    }
    if (*s).params.quality == FAST_TWO_PASS_COMPRESSION_QUALITY && (*s).command_buf_.is_null() {
        (*s).command_buf_ = if kCompressFragmentTwoPassBlockSize > 0 as size_t {
            BrotliAllocate(
                m,
                kCompressFragmentTwoPassBlockSize
                    .wrapping_mul(::core::mem::size_of::<uint32_t>() as size_t),
            ) as *mut uint32_t
        } else {
            ::core::ptr::null_mut::<uint32_t>()
        };
        (*s).literal_buf_ = if kCompressFragmentTwoPassBlockSize > 0 as size_t {
            BrotliAllocate(
                m,
                kCompressFragmentTwoPassBlockSize
                    .wrapping_mul(::core::mem::size_of::<uint8_t>() as size_t),
            ) as *mut uint8_t
        } else {
            ::core::ptr::null_mut::<uint8_t>()
        };
        if 0 as c_int != 0
            || 0 as c_int != 0
            || 0 as c_int != 0
        {
            return BROTLI_FALSE;
        }
    }
    if fast_compress != 0 {
        let mut storage: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut storage_ix: size_t = (*s).last_bytes_bits_ as size_t;
        let mut table_size: size_t = 0;
        let mut table: *mut c_int = ::core::ptr::null_mut::<c_int>();
        storage = GetBrotliStorage(
            s,
            (2 as uint32_t)
                .wrapping_mul(bytes)
                .wrapping_add(503 as uint32_t) as size_t,
        );
        if 0 as c_int != 0 {
            return BROTLI_FALSE;
        }
        *storage.offset(0 as c_int as isize) = (*s).last_bytes_ as uint8_t;
        *storage.offset(1 as c_int as isize) =
            ((*s).last_bytes_ as c_int >> 8 as c_int) as uint8_t;
        table = GetHashTable(s, (*s).params.quality, bytes as size_t, &raw mut table_size);
        if 0 as c_int != 0 {
            return BROTLI_FALSE;
        }
        if (*s).params.quality == FAST_ONE_PASS_COMPRESSION_QUALITY {
            BrotliCompressFragmentFast(
                (*s).one_pass_arena_,
                data.offset((wrapped_last_processed_pos & mask) as isize) as *mut uint8_t,
                bytes as size_t,
                is_last,
                table,
                table_size,
                &raw mut storage_ix,
                storage,
            );
            if 0 as c_int != 0 {
                return BROTLI_FALSE;
            }
        } else {
            BrotliCompressFragmentTwoPass(
                (*s).two_pass_arena_,
                data.offset((wrapped_last_processed_pos & mask) as isize) as *mut uint8_t,
                bytes as size_t,
                is_last,
                (*s).command_buf_,
                (*s).literal_buf_,
                table,
                table_size,
                &raw mut storage_ix,
                storage,
            );
            if 0 as c_int != 0 {
                return BROTLI_FALSE;
            }
        }
        (*s).last_bytes_ =
            *storage.offset((storage_ix >> 3 as c_int) as isize) as uint16_t;
        (*s).last_bytes_bits_ = (storage_ix & 7 as size_t) as uint8_t;
        UpdateLastProcessedPos(s);
        *output = storage.offset(0 as c_int as isize) as *mut uint8_t;
        *out_size = storage_ix >> 3 as c_int;
        return BROTLI_TRUE;
    }
    let mut newsize: size_t = (*s)
        .num_commands_
        .wrapping_add(bytes.wrapping_div(2 as uint32_t) as size_t)
        .wrapping_add(1 as size_t);
    if newsize > (*s).cmd_alloc_size_ {
        let mut new_commands: *mut Command = ::core::ptr::null_mut::<Command>();
        newsize = (newsize as c_ulong).wrapping_add(
            bytes
                .wrapping_div(4 as uint32_t)
                .wrapping_add(16 as uint32_t) as c_ulong,
        ) as size_t as size_t;
        (*s).cmd_alloc_size_ = newsize;
        new_commands = if newsize > 0 as size_t {
            BrotliAllocate(
                m,
                newsize.wrapping_mul(::core::mem::size_of::<Command>() as size_t),
            ) as *mut Command
        } else {
            ::core::ptr::null_mut::<Command>()
        };
        if 0 as c_int != 0 || 0 as c_int != 0 {
            return BROTLI_FALSE;
        }
        if !(*s).commands_.is_null() {
            memcpy(
                new_commands as *mut c_void,
                (*s).commands_ as *const c_void,
                (::core::mem::size_of::<Command>() as size_t).wrapping_mul((*s).num_commands_),
            );
            BrotliFree(m, (*s).commands_ as *mut c_void);
            (*s).commands_ = ::core::ptr::null_mut::<Command>();
        }
        (*s).commands_ = new_commands;
    }
    InitOrStitchToPreviousBlock(
        m,
        &raw mut (*s).hasher_,
        data,
        mask as size_t,
        &raw mut (*s).params,
        wrapped_last_processed_pos as size_t,
        bytes as size_t,
        is_last,
    );
    literal_context_mode = ChooseContextMode(
        &raw mut (*s).params,
        data,
        WrapPosition((*s).last_flush_pos_) as size_t,
        mask as size_t,
        (*s).input_pos_.wrapping_sub((*s).last_flush_pos_) as size_t,
    );
    literal_context_lut = (&raw const _kBrotliContextLookupTable as *const uint8_t)
        .offset(((literal_context_mode as c_uint) << 9 as c_int) as isize)
        as *const uint8_t as ContextLut;
    if 0 as c_int != 0 {
        return BROTLI_FALSE;
    }
    if (*s).num_commands_ != 0 && (*s).last_insert_len_ == 0 as size_t {
        ExtendLastCommand(s, &raw mut bytes, &raw mut wrapped_last_processed_pos);
    }
    if (*s).params.quality == ZOPFLIFICATION_QUALITY {
        BrotliCreateZopfliBackwardReferences(
            m,
            bytes as size_t,
            wrapped_last_processed_pos as size_t,
            data,
            mask as size_t,
            literal_context_lut,
            &raw mut (*s).params,
            &raw mut (*s).hasher_,
            &raw mut (*s).dist_cache_ as *mut c_int,
            &raw mut (*s).last_insert_len_,
            (*s).commands_.offset((*s).num_commands_ as isize) as *mut Command,
            &raw mut (*s).num_commands_,
            &raw mut (*s).num_literals_,
        );
        if 0 as c_int != 0 {
            return BROTLI_FALSE;
        }
    } else if (*s).params.quality == HQ_ZOPFLIFICATION_QUALITY {
        BrotliCreateHqZopfliBackwardReferences(
            m,
            bytes as size_t,
            wrapped_last_processed_pos as size_t,
            data,
            mask as size_t,
            literal_context_lut,
            &raw mut (*s).params,
            &raw mut (*s).hasher_,
            &raw mut (*s).dist_cache_ as *mut c_int,
            &raw mut (*s).last_insert_len_,
            (*s).commands_.offset((*s).num_commands_ as isize) as *mut Command,
            &raw mut (*s).num_commands_,
            &raw mut (*s).num_literals_,
        );
        if 0 as c_int != 0 {
            return BROTLI_FALSE;
        }
    } else {
        BrotliCreateBackwardReferences(
            bytes as size_t,
            wrapped_last_processed_pos as size_t,
            data,
            mask as size_t,
            literal_context_lut,
            &raw mut (*s).params,
            &raw mut (*s).hasher_,
            &raw mut (*s).dist_cache_ as *mut c_int,
            &raw mut (*s).last_insert_len_,
            (*s).commands_.offset((*s).num_commands_ as isize) as *mut Command,
            &raw mut (*s).num_commands_,
            &raw mut (*s).num_literals_,
        );
    }
    let max_length: size_t = MaxMetablockSize(&raw mut (*s).params) as size_t;
    let max_literals: size_t = max_length.wrapping_div(8 as size_t);
    let max_commands: size_t = max_length.wrapping_div(8 as size_t);
    let processed_bytes: size_t = (*s).input_pos_.wrapping_sub((*s).last_flush_pos_) as size_t;
    let next_input_fits_metablock: c_int =
        if processed_bytes.wrapping_add(InputBlockSize(s)) <= max_length {
            BROTLI_TRUE
        } else {
            BROTLI_FALSE
        };
    let should_flush: c_int = if (*s).params.quality < 4 as c_int
        && (*s).num_literals_.wrapping_add((*s).num_commands_) >= 0x2fff as size_t
    {
        BROTLI_TRUE
    } else {
        BROTLI_FALSE
    };
    if is_last == 0
        && force_flush == 0
        && should_flush == 0
        && next_input_fits_metablock != 0
        && (*s).num_literals_ < max_literals
        && (*s).num_commands_ < max_commands
    {
        if UpdateLastProcessedPos(s) != 0 {
            HasherReset(&raw mut (*s).hasher_);
        }
        *out_size = 0 as size_t;
        return BROTLI_TRUE;
    }
    if (*s).last_insert_len_ > 0 as size_t {
        let fresh7 = (*s).num_commands_;
        (*s).num_commands_ = (*s).num_commands_.wrapping_add(1);
        InitInsertCommand(
            (*s).commands_.offset(fresh7 as isize) as *mut Command,
            (*s).last_insert_len_,
        );
        (*s).num_literals_ = ((*s).num_literals_ as c_ulong)
            .wrapping_add((*s).last_insert_len_ as c_ulong)
            as size_t as size_t;
        (*s).last_insert_len_ = 0 as size_t;
    }
    if is_last == 0 && (*s).input_pos_ == (*s).last_flush_pos_ {
        *out_size = 0 as size_t;
        return BROTLI_TRUE;
    }
    let metablock_size: uint32_t = (*s).input_pos_.wrapping_sub((*s).last_flush_pos_) as uint32_t;
    let mut storage_0: *mut uint8_t = GetBrotliStorage(
        s,
        (2 as uint32_t)
            .wrapping_mul(metablock_size)
            .wrapping_add(503 as uint32_t) as size_t,
    );
    let mut storage_ix_0: size_t = (*s).last_bytes_bits_ as size_t;
    if 0 as c_int != 0 {
        return BROTLI_FALSE;
    }
    *storage_0.offset(0 as c_int as isize) = (*s).last_bytes_ as uint8_t;
    *storage_0.offset(1 as c_int as isize) =
        ((*s).last_bytes_ as c_int >> 8 as c_int) as uint8_t;
    WriteMetaBlockInternal(
        m,
        data,
        mask as size_t,
        (*s).last_flush_pos_,
        metablock_size as size_t,
        is_last,
        literal_context_mode,
        &raw mut (*s).params,
        (*s).prev_byte_,
        (*s).prev_byte2_,
        (*s).num_literals_,
        (*s).num_commands_,
        (*s).commands_,
        &raw mut (*s).saved_dist_cache_ as *mut c_int,
        &raw mut (*s).dist_cache_ as *mut c_int,
        &raw mut storage_ix_0,
        storage_0,
    );
    if 0 as c_int != 0 {
        return BROTLI_FALSE;
    }
    (*s).last_bytes_ =
        *storage_0.offset((storage_ix_0 >> 3 as c_int) as isize) as uint16_t;
    (*s).last_bytes_bits_ = (storage_ix_0 & 7 as size_t) as uint8_t;
    (*s).last_flush_pos_ = (*s).input_pos_;
    if UpdateLastProcessedPos(s) != 0 {
        HasherReset(&raw mut (*s).hasher_);
    }
    if (*s).last_flush_pos_ > 0 as uint64_t {
        (*s).prev_byte_ = *data.offset(
            (((*s).last_flush_pos_ as uint32_t).wrapping_sub(1 as uint32_t) & mask) as isize,
        );
    }
    if (*s).last_flush_pos_ > 1 as uint64_t {
        (*s).prev_byte2_ = *data
            .offset(((*s).last_flush_pos_.wrapping_sub(2 as uint64_t) as uint32_t & mask) as isize);
    }
    (*s).num_commands_ = 0 as size_t;
    (*s).num_literals_ = 0 as size_t;
    memcpy(
        &raw mut (*s).saved_dist_cache_ as *mut c_int as *mut c_void,
        &raw mut (*s).dist_cache_ as *mut c_int as *const c_void,
        ::core::mem::size_of::<[c_int; 4]>() as size_t,
    );
    *output = storage_0.offset(0 as c_int as isize) as *mut uint8_t;
    *out_size = storage_ix_0 >> 3 as c_int;
    return BROTLI_TRUE;
}
unsafe extern "C" fn WriteMetadataHeader(
    mut s: *mut BrotliEncoderStateInternal,
    block_size: size_t,
    mut header: *mut uint8_t,
) -> size_t {
    let mut storage_ix: size_t = 0;
    storage_ix = (*s).last_bytes_bits_ as size_t;
    *header.offset(0 as c_int as isize) = (*s).last_bytes_ as uint8_t;
    *header.offset(1 as c_int as isize) =
        ((*s).last_bytes_ as c_int >> 8 as c_int) as uint8_t;
    (*s).last_bytes_ = 0 as uint16_t;
    (*s).last_bytes_bits_ = 0 as uint8_t;
    BrotliWriteBits(1 as size_t, 0 as uint64_t, &raw mut storage_ix, header);
    BrotliWriteBits(2 as size_t, 3 as uint64_t, &raw mut storage_ix, header);
    BrotliWriteBits(1 as size_t, 0 as uint64_t, &raw mut storage_ix, header);
    if block_size == 0 as size_t {
        BrotliWriteBits(2 as size_t, 0 as uint64_t, &raw mut storage_ix, header);
    } else {
        let mut nbits: uint32_t = if block_size == 1 as size_t {
            1 as uint32_t
        } else {
            Log2FloorNonZero((block_size as uint32_t).wrapping_sub(1 as uint32_t) as size_t)
                .wrapping_add(1 as uint32_t)
        };
        let mut nbytes: uint32_t = nbits
            .wrapping_add(7 as uint32_t)
            .wrapping_div(8 as uint32_t);
        BrotliWriteBits(2 as size_t, nbytes as uint64_t, &raw mut storage_ix, header);
        BrotliWriteBits(
            (8 as uint32_t).wrapping_mul(nbytes) as size_t,
            (block_size as uint64_t).wrapping_sub(1 as uint64_t),
            &raw mut storage_ix,
            header,
        );
    }
    return storage_ix.wrapping_add(7 as size_t) >> 3 as c_int;
}
#[no_mangle]
pub unsafe extern "C" fn BrotliEncoderMaxCompressedSize(mut input_size: size_t) -> size_t {
    let mut num_large_blocks: size_t = input_size >> 14 as c_int;
    let mut overhead: size_t = (2 as size_t)
        .wrapping_add((4 as size_t).wrapping_mul(num_large_blocks))
        .wrapping_add(3 as size_t)
        .wrapping_add(1 as size_t);
    let mut result: size_t = input_size.wrapping_add(overhead);
    if input_size == 0 as size_t {
        return 2 as size_t;
    }
    return if result < input_size {
        0 as size_t
    } else {
        result
    };
}
unsafe extern "C" fn MakeUncompressedStream(
    mut input: *const uint8_t,
    mut input_size: size_t,
    mut output: *mut uint8_t,
) -> size_t {
    let mut size: size_t = input_size;
    let mut result: size_t = 0 as size_t;
    let mut offset: size_t = 0 as size_t;
    if input_size == 0 as size_t {
        *output.offset(0 as c_int as isize) = 6 as uint8_t;
        return 1 as size_t;
    }
    let fresh0 = result;
    result = result.wrapping_add(1);
    *output.offset(fresh0 as isize) = 0x21 as uint8_t;
    let fresh1 = result;
    result = result.wrapping_add(1);
    *output.offset(fresh1 as isize) = 0x3 as uint8_t;
    while size > 0 as size_t {
        let mut nibbles: uint32_t = 0 as uint32_t;
        let mut chunk_size: uint32_t = 0;
        let mut bits: uint32_t = 0;
        chunk_size = if size > ((1 as c_uint) << 24 as c_int) as size_t {
            (1 as uint32_t) << 24 as c_int
        } else {
            size as uint32_t
        };
        if chunk_size > (1 as uint32_t) << 16 as c_int {
            nibbles = (if chunk_size > (1 as uint32_t) << 20 as c_int {
                2 as c_int
            } else {
                1 as c_int
            }) as uint32_t;
        }
        bits = nibbles << 1 as c_int
            | chunk_size.wrapping_sub(1 as uint32_t) << 3 as c_int
            | (1 as uint32_t)
                << (19 as uint32_t).wrapping_add((4 as uint32_t).wrapping_mul(nibbles));
        let fresh2 = result;
        result = result.wrapping_add(1);
        *output.offset(fresh2 as isize) = bits as uint8_t;
        let fresh3 = result;
        result = result.wrapping_add(1);
        *output.offset(fresh3 as isize) = (bits >> 8 as c_int) as uint8_t;
        let fresh4 = result;
        result = result.wrapping_add(1);
        *output.offset(fresh4 as isize) = (bits >> 16 as c_int) as uint8_t;
        if nibbles == 2 as uint32_t {
            let fresh5 = result;
            result = result.wrapping_add(1);
            *output.offset(fresh5 as isize) = (bits >> 24 as c_int) as uint8_t;
        }
        memcpy(
            output.offset(result as isize) as *mut uint8_t as *mut c_void,
            input.offset(offset as isize) as *const uint8_t as *const c_void,
            chunk_size as size_t,
        );
        result = (result as c_ulong).wrapping_add(chunk_size as c_ulong)
            as size_t as size_t;
        offset = (offset as c_ulong).wrapping_add(chunk_size as c_ulong)
            as size_t as size_t;
        size = (size as c_ulong).wrapping_sub(chunk_size as c_ulong)
            as size_t as size_t;
    }
    let fresh6 = result;
    result = result.wrapping_add(1);
    *output.offset(fresh6 as isize) = 3 as uint8_t;
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn BrotliEncoderCompress(
    mut quality: c_int,
    mut lgwin: c_int,
    mut mode: BrotliEncoderMode,
    mut input_size: size_t,
    mut input_buffer: *const uint8_t,
    mut encoded_size: *mut size_t,
    mut encoded_buffer: *mut uint8_t,
) -> c_int {
    let mut s: *mut BrotliEncoderStateInternal =
        ::core::ptr::null_mut::<BrotliEncoderStateInternal>();
    let mut out_size: size_t = *encoded_size;
    let mut input_start: *const uint8_t = input_buffer as *const uint8_t;
    let mut output_start: *mut uint8_t = encoded_buffer as *mut uint8_t;
    let mut max_out_size: size_t = BrotliEncoderMaxCompressedSize(input_size);
    if out_size == 0 as size_t {
        return BROTLI_FALSE;
    }
    if input_size == 0 as size_t {
        *encoded_size = 1 as size_t;
        *encoded_buffer = 6 as uint8_t;
        return BROTLI_TRUE;
    }
    s = BrotliEncoderCreateInstance(None, None, ::core::ptr::null_mut::<c_void>())
        as *mut BrotliEncoderStateInternal;
    if s.is_null() {
        return BROTLI_FALSE;
    } else {
        let mut available_in: size_t = input_size;
        let mut next_in: *const uint8_t = input_buffer as *const uint8_t;
        let mut available_out: size_t = *encoded_size;
        let mut next_out: *mut uint8_t = encoded_buffer as *mut uint8_t;
        let mut total_out: size_t = 0 as size_t;
        let mut result: c_int = BROTLI_FALSE;
        BrotliEncoderSetParameter(s, BROTLI_PARAM_QUALITY, quality as uint32_t);
        BrotliEncoderSetParameter(s, BROTLI_PARAM_LGWIN, lgwin as uint32_t);
        BrotliEncoderSetParameter(s, BROTLI_PARAM_MODE, mode as uint32_t);
        BrotliEncoderSetParameter(s, BROTLI_PARAM_SIZE_HINT, input_size as uint32_t);
        if lgwin > BROTLI_MAX_WINDOW_BITS {
            BrotliEncoderSetParameter(s, BROTLI_PARAM_LARGE_WINDOW, BROTLI_TRUE as uint32_t);
        }
        result = BrotliEncoderCompressStream(
            s,
            BROTLI_OPERATION_FINISH,
            &raw mut available_in,
            &raw mut next_in,
            &raw mut available_out,
            &raw mut next_out,
            &raw mut total_out,
        );
        if BrotliEncoderIsFinished(s) == 0 {
            result = 0 as c_int;
        }
        *encoded_size = total_out;
        BrotliEncoderDestroyInstance(s);
        if result == 0 || max_out_size != 0 && *encoded_size > max_out_size {
            *encoded_size = 0 as size_t;
            if max_out_size == 0 {
                return BROTLI_FALSE;
            }
            if out_size >= max_out_size {
                *encoded_size = MakeUncompressedStream(input_start, input_size, output_start);
                return BROTLI_TRUE;
            }
            return BROTLI_FALSE;
        } else {
            return BROTLI_TRUE;
        }
    };
}
unsafe extern "C" fn InjectBytePaddingBlock(mut s: *mut BrotliEncoderStateInternal) {
    let mut seal: uint32_t = (*s).last_bytes_ as uint32_t;
    let mut seal_bits: size_t = (*s).last_bytes_bits_ as size_t;
    let mut destination: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    (*s).last_bytes_ = 0 as uint16_t;
    (*s).last_bytes_bits_ = 0 as uint8_t;
    seal = (seal as c_uint | (0x6 as c_uint) << seal_bits) as uint32_t;
    seal_bits = (seal_bits as c_ulong).wrapping_add(6 as c_ulong)
        as size_t as size_t;
    if !(*s).next_out_.is_null() {
        destination = (*s).next_out_.offset((*s).available_out_ as isize);
    } else {
        destination = &raw mut (*s).tiny_buf_.u8_0 as *mut uint8_t;
        (*s).next_out_ = destination;
    }
    *destination.offset(0 as c_int as isize) = seal as uint8_t;
    if seal_bits > 8 as size_t {
        *destination.offset(1 as c_int as isize) =
            (seal >> 8 as c_int) as uint8_t;
    }
    if seal_bits > 16 as size_t {
        *destination.offset(2 as c_int as isize) =
            (seal >> 16 as c_int) as uint8_t;
    }
    (*s).available_out_ = ((*s).available_out_ as c_ulong).wrapping_add(
        (seal_bits.wrapping_add(7 as size_t) >> 3 as c_int) as c_ulong,
    ) as size_t as size_t;
}
unsafe extern "C" fn SetTotalOut(
    mut s: *mut BrotliEncoderStateInternal,
    mut total_out: *mut size_t,
) {
    if !total_out.is_null() {
        let mut result: size_t = -(1 as c_int) as size_t;
        if (*s).total_out_ < result as uint64_t {
            result = (*s).total_out_ as size_t;
        }
        *total_out = result;
    }
}
unsafe extern "C" fn InjectFlushOrPushOutput(
    mut s: *mut BrotliEncoderStateInternal,
    mut available_out: *mut size_t,
    mut next_out: *mut *mut uint8_t,
    mut total_out: *mut size_t,
) -> c_int {
    if (*s).stream_state_ as c_uint
        == BROTLI_STREAM_FLUSH_REQUESTED as c_int as c_uint
        && (*s).last_bytes_bits_ as c_int != 0 as c_int
    {
        InjectBytePaddingBlock(s);
        return BROTLI_TRUE;
    }
    if (*s).available_out_ != 0 as size_t && *available_out != 0 as size_t {
        let mut copy_output_size: size_t = brotli_min_size_t((*s).available_out_, *available_out);
        memcpy(
            *next_out as *mut c_void,
            (*s).next_out_ as *const c_void,
            copy_output_size,
        );
        *next_out = (*next_out).offset(copy_output_size as isize);
        *available_out = (*available_out as c_ulong)
            .wrapping_sub(copy_output_size as c_ulong)
            as size_t as size_t;
        (*s).next_out_ = (*s).next_out_.offset(copy_output_size as isize);
        (*s).available_out_ = ((*s).available_out_ as c_ulong)
            .wrapping_sub(copy_output_size as c_ulong)
            as size_t as size_t;
        (*s).total_out_ = ((*s).total_out_ as c_ulong)
            .wrapping_add(copy_output_size as c_ulong)
            as uint64_t as uint64_t;
        SetTotalOut(s, total_out);
        return BROTLI_TRUE;
    }
    return BROTLI_FALSE;
}
unsafe extern "C" fn CheckFlushComplete(mut s: *mut BrotliEncoderStateInternal) {
    if (*s).stream_state_ as c_uint
        == BROTLI_STREAM_FLUSH_REQUESTED as c_int as c_uint
        && (*s).available_out_ == 0 as size_t
    {
        (*s).stream_state_ = BROTLI_STREAM_PROCESSING;
        (*s).next_out_ = ::core::ptr::null_mut::<uint8_t>();
    }
}
unsafe extern "C" fn BrotliEncoderCompressStreamFast(
    mut s: *mut BrotliEncoderStateInternal,
    mut op: BrotliEncoderOperation,
    mut available_in: *mut size_t,
    mut next_in: *mut *const uint8_t,
    mut available_out: *mut size_t,
    mut next_out: *mut *mut uint8_t,
    mut total_out: *mut size_t,
) -> c_int {
    let block_size_limit: size_t = (1 as c_int as size_t) << (*s).params.lgwin;
    let buf_size: size_t = brotli_min_size_t(
        kCompressFragmentTwoPassBlockSize,
        brotli_min_size_t(*available_in, block_size_limit),
    ) as size_t;
    let mut tmp_command_buf: *mut uint32_t = ::core::ptr::null_mut::<uint32_t>();
    let mut command_buf: *mut uint32_t = ::core::ptr::null_mut::<uint32_t>();
    let mut tmp_literal_buf: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut literal_buf: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut m: *mut MemoryManager = &raw mut (*s).memory_manager_;
    if (*s).params.quality != FAST_ONE_PASS_COMPRESSION_QUALITY
        && (*s).params.quality != FAST_TWO_PASS_COMPRESSION_QUALITY
    {
        return BROTLI_FALSE;
    }
    if (*s).params.quality == FAST_TWO_PASS_COMPRESSION_QUALITY {
        if (*s).command_buf_.is_null() && buf_size == kCompressFragmentTwoPassBlockSize {
            (*s).command_buf_ = if kCompressFragmentTwoPassBlockSize > 0 as size_t {
                BrotliAllocate(
                    m,
                    kCompressFragmentTwoPassBlockSize
                        .wrapping_mul(::core::mem::size_of::<uint32_t>() as size_t),
                ) as *mut uint32_t
            } else {
                ::core::ptr::null_mut::<uint32_t>()
            };
            (*s).literal_buf_ = if kCompressFragmentTwoPassBlockSize > 0 as size_t {
                BrotliAllocate(
                    m,
                    kCompressFragmentTwoPassBlockSize
                        .wrapping_mul(::core::mem::size_of::<uint8_t>() as size_t),
                ) as *mut uint8_t
            } else {
                ::core::ptr::null_mut::<uint8_t>()
            };
            if 0 as c_int != 0
                || 0 as c_int != 0
                || 0 as c_int != 0
            {
                return BROTLI_FALSE;
            }
        }
        if !(*s).command_buf_.is_null() {
            command_buf = (*s).command_buf_;
            literal_buf = (*s).literal_buf_;
        } else {
            tmp_command_buf = if buf_size > 0 as size_t {
                BrotliAllocate(
                    m,
                    buf_size.wrapping_mul(::core::mem::size_of::<uint32_t>() as size_t),
                ) as *mut uint32_t
            } else {
                ::core::ptr::null_mut::<uint32_t>()
            };
            tmp_literal_buf = if buf_size > 0 as size_t {
                BrotliAllocate(
                    m,
                    buf_size.wrapping_mul(::core::mem::size_of::<uint8_t>() as size_t),
                ) as *mut uint8_t
            } else {
                ::core::ptr::null_mut::<uint8_t>()
            };
            if 0 as c_int != 0
                || 0 as c_int != 0
                || 0 as c_int != 0
            {
                return BROTLI_FALSE;
            }
            command_buf = tmp_command_buf;
            literal_buf = tmp_literal_buf;
        }
    }
    loop {
        if InjectFlushOrPushOutput(s, available_out, next_out, total_out) != 0 {
            continue;
        }
        if !((*s).available_out_ == 0 as size_t
            && (*s).stream_state_ as c_uint
                == BROTLI_STREAM_PROCESSING as c_int as c_uint
            && (*available_in != 0 as size_t
                || op as c_uint
                    != BROTLI_OPERATION_PROCESS as c_int as c_uint))
        {
            break;
        }
        let mut block_size: size_t = brotli_min_size_t(block_size_limit, *available_in);
        let mut is_last: c_int = (*available_in == block_size
            && op as c_uint
                == BROTLI_OPERATION_FINISH as c_int as c_uint)
            as c_int;
        let mut force_flush: c_int = (*available_in == block_size
            && op as c_uint
                == BROTLI_OPERATION_FLUSH as c_int as c_uint)
            as c_int;
        let mut max_out_size: size_t = (2 as size_t)
            .wrapping_mul(block_size)
            .wrapping_add(503 as size_t);
        let mut inplace: c_int = BROTLI_TRUE;
        let mut storage: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut storage_ix: size_t = (*s).last_bytes_bits_ as size_t;
        let mut table_size: size_t = 0;
        let mut table: *mut c_int = ::core::ptr::null_mut::<c_int>();
        if force_flush != 0 && block_size == 0 as size_t {
            (*s).stream_state_ = BROTLI_STREAM_FLUSH_REQUESTED;
        } else {
            if max_out_size <= *available_out {
                storage = *next_out;
            } else {
                inplace = BROTLI_FALSE;
                storage = GetBrotliStorage(s, max_out_size);
                if 0 as c_int != 0 {
                    return BROTLI_FALSE;
                }
            }
            *storage.offset(0 as c_int as isize) = (*s).last_bytes_ as uint8_t;
            *storage.offset(1 as c_int as isize) =
                ((*s).last_bytes_ as c_int >> 8 as c_int) as uint8_t;
            table = GetHashTable(s, (*s).params.quality, block_size, &raw mut table_size);
            if 0 as c_int != 0 {
                return BROTLI_FALSE;
            }
            if (*s).params.quality == FAST_ONE_PASS_COMPRESSION_QUALITY {
                BrotliCompressFragmentFast(
                    (*s).one_pass_arena_,
                    *next_in,
                    block_size,
                    is_last,
                    table,
                    table_size,
                    &raw mut storage_ix,
                    storage,
                );
                if 0 as c_int != 0 {
                    return BROTLI_FALSE;
                }
            } else {
                BrotliCompressFragmentTwoPass(
                    (*s).two_pass_arena_,
                    *next_in,
                    block_size,
                    is_last,
                    command_buf,
                    literal_buf,
                    table,
                    table_size,
                    &raw mut storage_ix,
                    storage,
                );
                if 0 as c_int != 0 {
                    return BROTLI_FALSE;
                }
            }
            if block_size != 0 as size_t {
                *next_in = (*next_in).offset(block_size as isize);
                *available_in = (*available_in as c_ulong)
                    .wrapping_sub(block_size as c_ulong)
                    as size_t as size_t;
                (*s).total_in_ = ((*s).total_in_ as c_ulong)
                    .wrapping_add(block_size as c_ulong)
                    as uint64_t as uint64_t;
            }
            if inplace != 0 {
                let mut out_bytes: size_t = storage_ix >> 3 as c_int;
                *next_out = (*next_out).offset(out_bytes as isize);
                *available_out = (*available_out as c_ulong)
                    .wrapping_sub(out_bytes as c_ulong)
                    as size_t as size_t;
                (*s).total_out_ = ((*s).total_out_ as c_ulong)
                    .wrapping_add(out_bytes as c_ulong)
                    as uint64_t as uint64_t;
                SetTotalOut(s, total_out);
            } else {
                let mut out_bytes_0: size_t = storage_ix >> 3 as c_int;
                (*s).next_out_ = storage;
                (*s).available_out_ = out_bytes_0;
            }
            (*s).last_bytes_ =
                *storage.offset((storage_ix >> 3 as c_int) as isize) as uint16_t;
            (*s).last_bytes_bits_ = (storage_ix & 7 as size_t) as uint8_t;
            if force_flush != 0 {
                (*s).stream_state_ = BROTLI_STREAM_FLUSH_REQUESTED;
            }
            if is_last != 0 {
                (*s).stream_state_ = BROTLI_STREAM_FINISHED;
            }
        }
    }
    BrotliFree(m, tmp_command_buf as *mut c_void);
    tmp_command_buf = ::core::ptr::null_mut::<uint32_t>();
    BrotliFree(m, tmp_literal_buf as *mut c_void);
    tmp_literal_buf = ::core::ptr::null_mut::<uint8_t>();
    CheckFlushComplete(s);
    return BROTLI_TRUE;
}
unsafe extern "C" fn ProcessMetadata(
    mut s: *mut BrotliEncoderStateInternal,
    mut available_in: *mut size_t,
    mut next_in: *mut *const uint8_t,
    mut available_out: *mut size_t,
    mut next_out: *mut *mut uint8_t,
    mut total_out: *mut size_t,
) -> c_int {
    if *available_in > ((1 as c_uint) << 24 as c_int) as size_t {
        return BROTLI_FALSE;
    }
    if (*s).stream_state_ as c_uint
        == BROTLI_STREAM_PROCESSING as c_int as c_uint
    {
        (*s).remaining_metadata_bytes_ = *available_in as uint32_t;
        (*s).stream_state_ = BROTLI_STREAM_METADATA_HEAD;
    }
    if (*s).stream_state_ as c_uint
        != BROTLI_STREAM_METADATA_HEAD as c_int as c_uint
        && (*s).stream_state_ as c_uint
            != BROTLI_STREAM_METADATA_BODY as c_int as c_uint
    {
        return BROTLI_FALSE;
    }
    loop {
        if InjectFlushOrPushOutput(s, available_out, next_out, total_out) != 0 {
            continue;
        }
        if (*s).available_out_ != 0 as size_t {
            break;
        }
        if (*s).input_pos_ != (*s).last_flush_pos_ {
            let mut result: c_int = EncodeData(
                s,
                BROTLI_FALSE,
                BROTLI_TRUE,
                &raw mut (*s).available_out_,
                &raw mut (*s).next_out_,
            );
            if result == 0 {
                return BROTLI_FALSE;
            }
        } else if (*s).stream_state_ as c_uint
            == BROTLI_STREAM_METADATA_HEAD as c_int as c_uint
        {
            (*s).next_out_ = &raw mut (*s).tiny_buf_.u8_0 as *mut uint8_t;
            (*s).available_out_ =
                WriteMetadataHeader(s, (*s).remaining_metadata_bytes_ as size_t, (*s).next_out_);
            (*s).stream_state_ = BROTLI_STREAM_METADATA_BODY;
        } else if (*s).remaining_metadata_bytes_ == 0 as uint32_t {
            (*s).remaining_metadata_bytes_ = BROTLI_UINT32_MAX;
            (*s).stream_state_ = BROTLI_STREAM_PROCESSING;
            break;
        } else if *available_out != 0 {
            let mut copy: uint32_t =
                brotli_min_size_t((*s).remaining_metadata_bytes_ as size_t, *available_out)
                    as uint32_t;
            memcpy(
                *next_out as *mut c_void,
                *next_in as *const c_void,
                copy as size_t,
            );
            *next_in = (*next_in).offset(copy as isize);
            *available_in = (*available_in as c_ulong)
                .wrapping_sub(copy as c_ulong) as size_t
                as size_t;
            (*s).total_in_ = ((*s).total_in_ as c_ulong)
                .wrapping_add(copy as c_ulong) as uint64_t
                as uint64_t;
            (*s).remaining_metadata_bytes_ = ((*s).remaining_metadata_bytes_ as c_uint)
                .wrapping_sub(copy as c_uint)
                as uint32_t as uint32_t;
            *next_out = (*next_out).offset(copy as isize);
            *available_out = (*available_out as c_ulong)
                .wrapping_sub(copy as c_ulong) as size_t
                as size_t;
        } else {
            let mut copy_0: uint32_t =
                brotli_min_uint32_t((*s).remaining_metadata_bytes_, 16 as uint32_t);
            (*s).next_out_ = &raw mut (*s).tiny_buf_.u8_0 as *mut uint8_t;
            memcpy(
                (*s).next_out_ as *mut c_void,
                *next_in as *const c_void,
                copy_0 as size_t,
            );
            *next_in = (*next_in).offset(copy_0 as isize);
            *available_in = (*available_in as c_ulong)
                .wrapping_sub(copy_0 as c_ulong) as size_t
                as size_t;
            (*s).total_in_ = ((*s).total_in_ as c_ulong)
                .wrapping_add(copy_0 as c_ulong)
                as uint64_t as uint64_t;
            (*s).remaining_metadata_bytes_ = ((*s).remaining_metadata_bytes_ as c_uint)
                .wrapping_sub(copy_0 as c_uint)
                as uint32_t as uint32_t;
            (*s).available_out_ = copy_0 as size_t;
        }
    }
    return BROTLI_TRUE;
}
unsafe extern "C" fn UpdateSizeHint(
    mut s: *mut BrotliEncoderStateInternal,
    mut available_in: size_t,
) {
    if (*s).params.size_hint == 0 as size_t {
        let mut delta: uint64_t = UnprocessedInputSize(s);
        let mut tail: uint64_t = available_in as uint64_t;
        let mut limit: uint32_t = (1 as uint32_t) << 30 as c_int;
        let mut total: uint32_t = 0;
        if delta >= limit as uint64_t
            || tail >= limit as uint64_t
            || delta.wrapping_add(tail) >= limit as uint64_t
        {
            total = limit;
        } else {
            total = delta.wrapping_add(tail) as uint32_t;
        }
        (*s).params.size_hint = total as size_t;
    }
}
#[no_mangle]
pub unsafe extern "C" fn BrotliEncoderCompressStream(
    mut s: *mut BrotliEncoderStateInternal,
    mut op: BrotliEncoderOperation,
    mut available_in: *mut size_t,
    mut next_in: *mut *const uint8_t,
    mut available_out: *mut size_t,
    mut next_out: *mut *mut uint8_t,
    mut total_out: *mut size_t,
) -> c_int {
    if EnsureInitialized(s) == 0 {
        return BROTLI_FALSE;
    }
    if (*s).remaining_metadata_bytes_ != BROTLI_UINT32_MAX {
        if *available_in != (*s).remaining_metadata_bytes_ as size_t {
            return BROTLI_FALSE;
        }
        if op as c_uint
            != BROTLI_OPERATION_EMIT_METADATA as c_int as c_uint
        {
            return BROTLI_FALSE;
        }
    }
    if op as c_uint
        == BROTLI_OPERATION_EMIT_METADATA as c_int as c_uint
    {
        UpdateSizeHint(s, 0 as size_t);
        return ProcessMetadata(s, available_in, next_in, available_out, next_out, total_out);
    }
    if (*s).stream_state_ as c_uint
        == BROTLI_STREAM_METADATA_HEAD as c_int as c_uint
        || (*s).stream_state_ as c_uint
            == BROTLI_STREAM_METADATA_BODY as c_int as c_uint
    {
        return BROTLI_FALSE;
    }
    if (*s).stream_state_ as c_uint
        != BROTLI_STREAM_PROCESSING as c_int as c_uint
        && *available_in != 0 as size_t
    {
        return BROTLI_FALSE;
    }
    if (*s).params.quality == FAST_ONE_PASS_COMPRESSION_QUALITY
        || (*s).params.quality == FAST_TWO_PASS_COMPRESSION_QUALITY
    {
        return BrotliEncoderCompressStreamFast(
            s,
            op,
            available_in,
            next_in,
            available_out,
            next_out,
            total_out,
        );
    }
    loop {
        let mut remaining_block_size: size_t = RemainingInputBlockSize(s);
        if (*s).flint_ as c_int >= 0 as c_int
            && remaining_block_size > (*s).flint_ as size_t
        {
            remaining_block_size = (*s).flint_ as size_t;
        }
        if remaining_block_size != 0 as size_t && *available_in != 0 as size_t {
            let mut copy_input_size: size_t =
                brotli_min_size_t(remaining_block_size, *available_in);
            CopyInputToRingBuffer(s, copy_input_size, *next_in);
            *next_in = (*next_in).offset(copy_input_size as isize);
            *available_in = (*available_in as c_ulong)
                .wrapping_sub(copy_input_size as c_ulong)
                as size_t as size_t;
            (*s).total_in_ = ((*s).total_in_ as c_ulong)
                .wrapping_add(copy_input_size as c_ulong)
                as uint64_t as uint64_t;
            if (*s).flint_ as c_int > 0 as c_int {
                (*s).flint_ = ((*s).flint_ as c_int
                    - copy_input_size as c_int)
                    as int8_t;
            }
        } else if InjectFlushOrPushOutput(s, available_out, next_out, total_out) != 0 {
            if (*s).flint_ as c_int
                == BROTLI_FLINT_WAITING_FOR_FLUSHING as c_int
            {
                CheckFlushComplete(s);
                if (*s).stream_state_ as c_uint
                    == BROTLI_STREAM_PROCESSING as c_int as c_uint
                {
                    (*s).flint_ = BROTLI_FLINT_DONE as c_int as int8_t;
                }
            }
        } else {
            if !((*s).available_out_ == 0 as size_t
                && (*s).stream_state_ as c_uint
                    == BROTLI_STREAM_PROCESSING as c_int as c_uint)
            {
                break;
            }
            if !(remaining_block_size == 0 as size_t
                || op as c_uint
                    != BROTLI_OPERATION_PROCESS as c_int as c_uint)
            {
                break;
            }
            let mut is_last: c_int = if *available_in == 0 as size_t
                && op as c_uint
                    == BROTLI_OPERATION_FINISH as c_int as c_uint
            {
                BROTLI_TRUE
            } else {
                BROTLI_FALSE
            };
            let mut force_flush: c_int = if *available_in == 0 as size_t
                && op as c_uint
                    == BROTLI_OPERATION_FLUSH as c_int as c_uint
            {
                BROTLI_TRUE
            } else {
                BROTLI_FALSE
            };
            let mut result: c_int = 0;
            if is_last == 0 && (*s).flint_ as c_int == 0 as c_int {
                (*s).flint_ = BROTLI_FLINT_WAITING_FOR_FLUSHING as c_int as int8_t;
                force_flush = BROTLI_TRUE;
            }
            UpdateSizeHint(s, *available_in);
            result = EncodeData(
                s,
                is_last,
                force_flush,
                &raw mut (*s).available_out_,
                &raw mut (*s).next_out_,
            );
            if result == 0 {
                return BROTLI_FALSE;
            }
            if force_flush != 0 {
                (*s).stream_state_ = BROTLI_STREAM_FLUSH_REQUESTED;
            }
            if is_last != 0 {
                (*s).stream_state_ = BROTLI_STREAM_FINISHED;
            }
        }
    }
    CheckFlushComplete(s);
    return BROTLI_TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn BrotliEncoderIsFinished(
    mut s: *mut BrotliEncoderStateInternal,
) -> c_int {
    return if (*s).stream_state_ as c_uint
        == BROTLI_STREAM_FINISHED as c_int as c_uint
        && BrotliEncoderHasMoreOutput(s) == 0
    {
        BROTLI_TRUE
    } else {
        BROTLI_FALSE
    };
}
#[no_mangle]
pub unsafe extern "C" fn BrotliEncoderHasMoreOutput(
    mut s: *mut BrotliEncoderStateInternal,
) -> c_int {
    return if (*s).available_out_ != 0 as size_t {
        BROTLI_TRUE
    } else {
        BROTLI_FALSE
    };
}
#[no_mangle]
pub unsafe extern "C" fn BrotliEncoderTakeOutput(
    mut s: *mut BrotliEncoderStateInternal,
    mut size: *mut size_t,
) -> *const uint8_t {
    let mut consumed_size: size_t = (*s).available_out_;
    let mut result: *mut uint8_t = (*s).next_out_;
    if *size != 0 {
        consumed_size = brotli_min_size_t(*size, (*s).available_out_);
    }
    if consumed_size != 0 {
        (*s).next_out_ = (*s).next_out_.offset(consumed_size as isize);
        (*s).available_out_ = ((*s).available_out_ as c_ulong)
            .wrapping_sub(consumed_size as c_ulong)
            as size_t as size_t;
        (*s).total_out_ = ((*s).total_out_ as c_ulong)
            .wrapping_add(consumed_size as c_ulong)
            as uint64_t as uint64_t;
        CheckFlushComplete(s);
        *size = consumed_size;
    } else {
        *size = 0 as size_t;
        result = ::core::ptr::null_mut::<uint8_t>();
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn BrotliEncoderVersion() -> uint32_t {
    return BROTLI_VERSION as uint32_t;
}
#[no_mangle]
pub unsafe extern "C" fn BrotliEncoderPrepareDictionary(
    mut type_0: BrotliSharedDictionaryType,
    mut size: size_t,
    mut data: *const uint8_t,
    mut quality: c_int,
    mut alloc_func: brotli_alloc_func,
    mut free_func: brotli_free_func,
    mut opaque: *mut c_void,
) -> *mut BrotliEncoderPreparedDictionary {
    let mut managed_dictionary: *mut ManagedDictionary =
        ::core::ptr::null_mut::<ManagedDictionary>();
    let mut type_is_known: c_int = BROTLI_FALSE;
    type_is_known |= (type_0 as c_uint
        == BROTLI_SHARED_DICTIONARY_RAW as c_int as c_uint)
        as c_int;
    if type_is_known == 0 {
        return ::core::ptr::null_mut::<BrotliEncoderPreparedDictionary>();
    }
    managed_dictionary = BrotliCreateManagedDictionary(alloc_func, free_func, opaque);
    if managed_dictionary.is_null() {
        return ::core::ptr::null_mut::<BrotliEncoderPreparedDictionary>();
    }
    if type_0 as c_uint
        == BROTLI_SHARED_DICTIONARY_RAW as c_int as c_uint
    {
        (*managed_dictionary).dictionary = CreatePreparedDictionary(
            &raw mut (*managed_dictionary).memory_manager_,
            data as *const uint8_t,
            size,
        ) as *mut uint32_t;
    }
    if (*managed_dictionary).dictionary.is_null() {
        BrotliDestroyManagedDictionary(managed_dictionary);
        return ::core::ptr::null_mut::<BrotliEncoderPreparedDictionary>();
    }
    return managed_dictionary as *mut BrotliEncoderPreparedDictionary;
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn BrotliEncoderDestroyPreparedDictionary(
    mut dictionary: *mut BrotliEncoderPreparedDictionary,
) {
    let mut dict: *mut ManagedDictionary = dictionary as *mut ManagedDictionary;
    if dictionary.is_null() {
        return;
    }
    if (*dict).magic != kManagedDictionaryMagic {
        return;
    }
    if !(*dict).dictionary.is_null() {
        if *(*dict).dictionary == kLeanPreparedDictionaryMagic {
            DestroyPreparedDictionary(
                &raw mut (*dict).memory_manager_,
                (*dict).dictionary as *mut PreparedDictionary,
            );
        } else if *(*dict).dictionary == kSharedDictionaryMagic {
            BrotliCleanupSharedEncoderDictionary(
                &raw mut (*dict).memory_manager_,
                (*dict).dictionary as *mut SharedEncoderDictionary,
            );
            BrotliFree(
                &raw mut (*dict).memory_manager_,
                (*dict).dictionary as *mut c_void,
            );
        }
    }
    (*dict).dictionary = ::core::ptr::null_mut::<uint32_t>();
    BrotliDestroyManagedDictionary(dict);
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn BrotliEncoderAttachPreparedDictionary(
    mut state: *mut BrotliEncoderStateInternal,
    mut dictionary: *const BrotliEncoderPreparedDictionary,
) -> c_int {
    let mut dict: *const BrotliEncoderPreparedDictionary = dictionary;
    let mut magic: uint32_t = *(dict as *const uint32_t);
    let mut current: *mut SharedEncoderDictionary =
        ::core::ptr::null_mut::<SharedEncoderDictionary>();
    if magic == kManagedDictionaryMagic {
        let mut managed_dictionary: *mut ManagedDictionary = dict as *mut ManagedDictionary;
        magic = *(*managed_dictionary).dictionary;
        dict = (*managed_dictionary).dictionary as *mut BrotliEncoderPreparedDictionary;
    }
    current = &raw mut (*state).params.dictionary;
    if magic == kPreparedDictionaryMagic || magic == kLeanPreparedDictionaryMagic {
        let mut prepared: *const PreparedDictionary = dict as *const PreparedDictionary;
        if AttachPreparedDictionary(&raw mut (*current).compound, prepared) == 0 {
            return BROTLI_FALSE;
        }
    } else if magic == kSharedDictionaryMagic {
        let mut attached: *const SharedEncoderDictionary = dict as *const SharedEncoderDictionary;
        let mut was_default: c_int = ((*current).contextual.context_based == 0
            && (*current).contextual.num_dictionaries as c_int
                == 1 as c_int
            && (*(*current).contextual.dict[0 as c_int as usize]).hash_table_words
                == &raw const kStaticDictionaryHashWords as *const uint16_t
            && (*(*current).contextual.dict[0 as c_int as usize]).hash_table_lengths
                == &raw const kStaticDictionaryHashLengths as *const uint8_t)
            as c_int;
        let mut new_default: c_int = ((*attached).contextual.context_based == 0
            && (*attached).contextual.num_dictionaries as c_int
                == 1 as c_int
            && (*(*attached).contextual.dict[0 as c_int as usize]).hash_table_words
                == &raw const kStaticDictionaryHashWords as *const uint16_t
            && (*(*attached).contextual.dict[0 as c_int as usize]).hash_table_lengths
                == &raw const kStaticDictionaryHashLengths as *const uint8_t)
            as c_int;
        let mut i: size_t = 0;
        if (*state).is_initialized_ != 0 {
            return BROTLI_FALSE;
        }
        (*current).max_quality = brotli_min_int((*current).max_quality, (*attached).max_quality);
        i = 0 as size_t;
        while i < (*attached).compound.num_chunks {
            if AttachPreparedDictionary(
                &raw mut (*current).compound,
                (*attached).compound.chunks[i as usize],
            ) == 0
            {
                return BROTLI_FALSE;
            }
            i = i.wrapping_add(1);
        }
        if new_default == 0 {
            if was_default == 0 {
                return BROTLI_FALSE;
            }
            (*current).contextual = (*attached).contextual;
            (*current).contextual.num_instances_ = 0 as size_t;
        }
    } else {
        return BROTLI_FALSE;
    }
    return BROTLI_TRUE;
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn BrotliEncoderEstimatePeakMemoryUsage(
    mut quality: c_int,
    mut lgwin: c_int,
    mut input_size: size_t,
) -> size_t {
    let mut params: BrotliEncoderParams = BrotliEncoderParams {
        mode: BROTLI_MODE_GENERIC,
        quality: 0,
        lgwin: 0,
        lgblock: 0,
        stream_offset: 0,
        size_hint: 0,
        disable_literal_context_modeling: 0,
        large_window: 0,
        hasher: BrotliHasherParams {
            type_0: 0,
            bucket_bits: 0,
            block_bits: 0,
            num_last_distances_to_check: 0,
        },
        dist: BrotliDistanceParams {
            distance_postfix_bits: 0,
            num_direct_distance_codes: 0,
            alphabet_size_max: 0,
            alphabet_size_limit: 0,
            max_distance: 0,
        },
        dictionary: SharedEncoderDictionary {
            magic: 0,
            compound: CompoundDictionary {
                num_chunks: 0,
                total_size: 0,
                chunks: [::core::ptr::null::<PreparedDictionary>(); 16],
                chunk_source: [::core::ptr::null::<uint8_t>(); 16],
                chunk_offsets: [0; 16],
                num_prepared_instances_: 0,
                prepared_instances_: [::core::ptr::null_mut::<PreparedDictionary>(); 16],
            },
            contextual: ContextualEncoderDictionary {
                context_based: 0,
                num_dictionaries: 0,
                context_map: [0; 64],
                dict: [::core::ptr::null::<BrotliEncoderDictionary>(); 64],
                num_instances_: 0,
                instance_: BrotliEncoderDictionary {
                    words: ::core::ptr::null::<BrotliDictionary>(),
                    num_transforms: 0,
                    cutoffTransformsCount: 0,
                    cutoffTransforms: 0,
                    hash_table_words: ::core::ptr::null::<uint16_t>(),
                    hash_table_lengths: ::core::ptr::null::<uint8_t>(),
                    buckets: ::core::ptr::null::<uint16_t>(),
                    dict_words: ::core::ptr::null::<DictWord>(),
                    trie: BrotliTrie {
                        pool: ::core::ptr::null_mut::<BrotliTrieNode>(),
                        pool_capacity: 0,
                        pool_size: 0,
                        root: BrotliTrieNode {
                            single: 0,
                            c: 0,
                            len_: 0,
                            idx_: 0,
                            sub: 0,
                        },
                    },
                    has_words_heavy: 0,
                    parent: ::core::ptr::null::<ContextualEncoderDictionary>(),
                    hash_table_data_words_: ::core::ptr::null_mut::<uint16_t>(),
                    hash_table_data_lengths_: ::core::ptr::null_mut::<uint8_t>(),
                    buckets_alloc_size_: 0,
                    buckets_data_: ::core::ptr::null_mut::<uint16_t>(),
                    dict_words_alloc_size_: 0,
                    dict_words_data_: ::core::ptr::null_mut::<DictWord>(),
                    words_instance_: ::core::ptr::null_mut::<BrotliDictionary>(),
                },
                instances_: ::core::ptr::null_mut::<BrotliEncoderDictionary>(),
            },
            max_quality: 0,
        },
    };
    let mut memory_manager_slots: size_t = BROTLI_ENCODER_MEMORY_MANAGER_SLOTS as size_t;
    let mut memory_manager_size: size_t = memory_manager_slots
        .wrapping_mul(::core::mem::size_of::<*mut c_void>() as size_t);
    BrotliEncoderInitParams(&raw mut params);
    params.quality = quality;
    params.lgwin = lgwin;
    params.size_hint = input_size;
    params.large_window = (lgwin > BROTLI_MAX_WINDOW_BITS) as c_int;
    SanitizeParams(&raw mut params);
    params.lgblock = ComputeLgBlock(&raw mut params);
    ChooseHasher(&raw mut params, &raw mut params.hasher);
    if params.quality == FAST_ONE_PASS_COMPRESSION_QUALITY
        || params.quality == FAST_TWO_PASS_COMPRESSION_QUALITY
    {
        let mut state_size: size_t = ::core::mem::size_of::<BrotliEncoderStateInternal>() as size_t;
        let mut block_size: size_t = brotli_min_size_t(
            input_size,
            (1 as c_ulong as size_t) << params.lgwin,
        );
        let mut hash_table_size: size_t =
            HashTableSize(MaxHashTableSize(params.quality), block_size);
        let mut hash_size: size_t = if hash_table_size
            < ((1 as c_uint) << 10 as c_int) as size_t
        {
            0 as size_t
        } else {
            (::core::mem::size_of::<c_int>() as size_t).wrapping_mul(hash_table_size)
        };
        let mut cmdbuf_size: size_t = if params.quality == FAST_TWO_PASS_COMPRESSION_QUALITY {
            (5 as size_t).wrapping_mul(brotli_min_size_t(
                block_size,
                (1 as size_t) << 17 as c_int,
            ))
        } else {
            0 as size_t
        };
        if params.quality == FAST_ONE_PASS_COMPRESSION_QUALITY {
            state_size = (state_size as c_ulong)
                .wrapping_add(
                    ::core::mem::size_of::<BrotliOnePassArena>() as usize as c_ulong
                ) as size_t as size_t;
        } else {
            state_size = (state_size as c_ulong)
                .wrapping_add(
                    ::core::mem::size_of::<BrotliTwoPassArena>() as usize as c_ulong
                ) as size_t as size_t;
        }
        return hash_size.wrapping_add(cmdbuf_size).wrapping_add(state_size);
    } else {
        let mut short_ringbuffer_size: size_t =
            (1 as c_int as size_t) << params.lgblock;
        let mut ringbuffer_bits: c_int = ComputeRbBits(&raw mut params);
        let mut ringbuffer_size: size_t = if input_size < short_ringbuffer_size {
            input_size
        } else {
            ((1 as c_uint as size_t) << ringbuffer_bits)
                .wrapping_add(short_ringbuffer_size)
        };
        let mut hash_size_0: [size_t; 4] = [0 as c_int as size_t, 0, 0, 0];
        let mut metablock_size: size_t =
            brotli_min_size_t(input_size, MaxMetablockSize(&raw mut params));
        let mut inputblock_size: size_t = brotli_min_size_t(
            input_size,
            (1 as c_int as size_t) << params.lgblock,
        );
        let mut cmdbuf_size_0: size_t = metablock_size
            .wrapping_mul(2 as size_t)
            .wrapping_add(inputblock_size.wrapping_mul(6 as size_t));
        let mut outbuf_size: size_t = metablock_size
            .wrapping_mul(2 as size_t)
            .wrapping_add(503 as size_t);
        let mut histogram_size: size_t = 0 as size_t;
        HasherSize(
            &raw mut params,
            BROTLI_TRUE,
            input_size,
            &raw mut hash_size_0 as *mut size_t,
        );
        if params.quality < MIN_QUALITY_FOR_BLOCK_SPLIT {
            cmdbuf_size_0 = brotli_min_size_t(
                cmdbuf_size_0,
                (0x2fff as size_t)
                    .wrapping_mul(::core::mem::size_of::<Command>() as size_t)
                    .wrapping_add(inputblock_size.wrapping_mul(12 as size_t)),
            );
        }
        if params.quality >= MIN_QUALITY_FOR_HQ_BLOCK_SPLITTING {
            histogram_size = ((200 as c_int) << 20 as c_int) as size_t;
        } else if params.quality >= MIN_QUALITY_FOR_BLOCK_SPLIT {
            let mut literal_histograms: size_t =
                brotli_min_size_t(metablock_size.wrapping_div(6144 as size_t), 256 as size_t);
            let mut command_histograms: size_t =
                brotli_min_size_t(metablock_size.wrapping_div(6144 as size_t), 256 as size_t);
            let mut distance_histograms: size_t =
                brotli_min_size_t(metablock_size.wrapping_div(6144 as size_t), 256 as size_t);
            histogram_size = literal_histograms
                .wrapping_mul(::core::mem::size_of::<HistogramLiteral>() as size_t)
                .wrapping_add(
                    command_histograms
                        .wrapping_mul(::core::mem::size_of::<HistogramCommand>() as size_t),
                )
                .wrapping_add(
                    distance_histograms
                        .wrapping_mul(::core::mem::size_of::<HistogramDistance>() as size_t),
                );
        }
        return memory_manager_size
            .wrapping_add(ringbuffer_size)
            .wrapping_add(hash_size_0[0 as c_int as usize])
            .wrapping_add(hash_size_0[1 as c_int as usize])
            .wrapping_add(hash_size_0[2 as c_int as usize])
            .wrapping_add(hash_size_0[3 as c_int as usize])
            .wrapping_add(cmdbuf_size_0)
            .wrapping_add(outbuf_size)
            .wrapping_add(histogram_size);
    };
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn BrotliEncoderGetPreparedDictionarySize(
    mut prepared_dictionary: *const BrotliEncoderPreparedDictionary,
) -> size_t {
    let mut prepared: *const BrotliEncoderPreparedDictionary = prepared_dictionary;
    let mut magic: uint32_t = *(prepared as *const uint32_t);
    let mut overhead: size_t = 0 as size_t;
    if magic == kManagedDictionaryMagic {
        let mut managed: *const ManagedDictionary = prepared as *const ManagedDictionary;
        overhead = ::core::mem::size_of::<ManagedDictionary>() as usize as size_t;
        magic = *(*managed).dictionary;
        prepared = (*managed).dictionary as *const BrotliEncoderPreparedDictionary;
    }
    if magic == kPreparedDictionaryMagic {
        let mut dictionary: *const PreparedDictionary = prepared as *const PreparedDictionary;
        return (::core::mem::size_of::<PreparedDictionary>() as size_t)
            .wrapping_add((*dictionary).source_size as size_t)
            .wrapping_add((::core::mem::size_of::<uint32_t>() as size_t) << (*dictionary).slot_bits)
            .wrapping_add(
                (::core::mem::size_of::<uint16_t>() as size_t) << (*dictionary).bucket_bits,
            )
            .wrapping_add(
                (::core::mem::size_of::<uint32_t>() as size_t)
                    .wrapping_mul((*dictionary).num_items as size_t),
            )
            .wrapping_add(overhead);
    } else if magic == kLeanPreparedDictionaryMagic {
        let mut dictionary_0: *const PreparedDictionary = prepared as *const PreparedDictionary;
        return (::core::mem::size_of::<PreparedDictionary>() as size_t)
            .wrapping_add(::core::mem::size_of::<*mut uint8_t>() as size_t)
            .wrapping_add(
                (::core::mem::size_of::<uint32_t>() as size_t) << (*dictionary_0).slot_bits,
            )
            .wrapping_add(
                (::core::mem::size_of::<uint16_t>() as size_t) << (*dictionary_0).bucket_bits,
            )
            .wrapping_add(
                (::core::mem::size_of::<uint32_t>() as size_t)
                    .wrapping_mul((*dictionary_0).num_items as size_t),
            )
            .wrapping_add(overhead);
    } else if magic == kSharedDictionaryMagic {
        let mut dictionary_1: *const SharedEncoderDictionary =
            prepared as *const SharedEncoderDictionary;
        let mut compound: *const CompoundDictionary = &raw const (*dictionary_1).compound;
        let mut contextual: *const ContextualEncoderDictionary =
            &raw const (*dictionary_1).contextual;
        let mut result: size_t = ::core::mem::size_of::<SharedEncoderDictionary>() as size_t;
        let mut i: size_t = 0;
        let mut num_instances: size_t = 0;
        let mut instances: *const BrotliEncoderDictionary =
            ::core::ptr::null::<BrotliEncoderDictionary>();
        i = 0 as size_t;
        while i < (*compound).num_prepared_instances_ {
            let mut size: size_t = BrotliEncoderGetPreparedDictionarySize(
                (*compound).prepared_instances_[i as usize]
                    as *const BrotliEncoderPreparedDictionary,
            );
            if size == 0 {
                return 0 as size_t;
            }
            result = (result as c_ulong).wrapping_add(size as c_ulong)
                as size_t as size_t;
            i = i.wrapping_add(1);
        }
        if (*contextual).context_based != 0 {
            num_instances = (*contextual).num_instances_;
            instances = (*contextual).instances_;
            result = (result as c_ulong).wrapping_add(
                (::core::mem::size_of::<BrotliEncoderDictionary>() as usize)
                    .wrapping_mul(num_instances as usize) as c_ulong,
            ) as size_t as size_t;
        } else {
            num_instances = 1 as size_t;
            instances = &raw const (*contextual).instance_;
        }
        i = 0 as size_t;
        while i < num_instances {
            let mut dict: *const BrotliEncoderDictionary =
                instances.offset(i as isize) as *const BrotliEncoderDictionary;
            result = (result as c_ulong).wrapping_add(
                (*dict)
                    .trie
                    .pool_capacity
                    .wrapping_mul(::core::mem::size_of::<BrotliTrieNode>() as size_t)
                    as c_ulong,
            ) as size_t as size_t;
            if !(*dict).hash_table_data_words_.is_null() {
                result = (result as c_ulong)
                    .wrapping_add(::core::mem::size_of::<[uint16_t; 32768]>() as usize
                        as c_ulong) as size_t as size_t;
            }
            if !(*dict).hash_table_data_lengths_.is_null() {
                result = (result as c_ulong)
                    .wrapping_add(
                        ::core::mem::size_of::<[uint8_t; 32768]>() as usize as c_ulong
                    ) as size_t as size_t;
            }
            if !(*dict).buckets_data_.is_null() {
                result = (result as c_ulong).wrapping_add(
                    (::core::mem::size_of::<uint16_t>() as usize)
                        .wrapping_mul((*dict).buckets_alloc_size_ as usize)
                        as c_ulong,
                ) as size_t as size_t;
            }
            if !(*dict).dict_words_data_.is_null() {
                result = (result as c_ulong).wrapping_add(
                    (::core::mem::size_of::<DictWord>() as usize)
                        .wrapping_mul((*dict).dict_words_alloc_size_ as usize)
                        as c_ulong,
                ) as size_t as size_t;
            }
            if !(*dict).words_instance_.is_null() {
                result = (result as c_ulong)
                    .wrapping_add(
                        ::core::mem::size_of::<BrotliDictionary>() as usize as c_ulong
                    ) as size_t as size_t;
            }
            i = i.wrapping_add(1);
        }
        return result.wrapping_add(overhead);
    }
    return 0 as size_t;
}
pub const JUMP: c_int = 1 as c_int;
static mut kCompressFragmentTwoPassBlockSize: size_t =
    ((1 as c_int) << 17 as c_int) as size_t;
#[inline(always)]
unsafe extern "C" fn HasherInit(mut hasher: *mut Hasher) {
    (*hasher).common.is_setup_ = BROTLI_FALSE;
    (*hasher).common.extra[0 as c_int as usize] = NULL;
    (*hasher).common.extra[1 as c_int as usize] = NULL;
    (*hasher).common.extra[2 as c_int as usize] = NULL;
    (*hasher).common.extra[3 as c_int as usize] = NULL;
}
#[inline(always)]
unsafe extern "C" fn DestroyHasher(mut m: *mut MemoryManager, mut hasher: *mut Hasher) {
    if !(*hasher).common.extra[0 as c_int as usize].is_null() {
        BrotliFree(m, (*hasher).common.extra[0 as c_int as usize]);
        (*hasher).common.extra[0 as c_int as usize] = NULL;
    }
    if !(*hasher).common.extra[1 as c_int as usize].is_null() {
        BrotliFree(m, (*hasher).common.extra[1 as c_int as usize]);
        (*hasher).common.extra[1 as c_int as usize] = NULL;
    }
    if !(*hasher).common.extra[2 as c_int as usize].is_null() {
        BrotliFree(m, (*hasher).common.extra[2 as c_int as usize]);
        (*hasher).common.extra[2 as c_int as usize] = NULL;
    }
    if !(*hasher).common.extra[3 as c_int as usize].is_null() {
        BrotliFree(m, (*hasher).common.extra[3 as c_int as usize]);
        (*hasher).common.extra[3 as c_int as usize] = NULL;
    }
}
#[inline(always)]
unsafe extern "C" fn HasherReset(mut hasher: *mut Hasher) {
    (*hasher).common.is_prepared_ = BROTLI_FALSE;
}
#[inline(always)]
unsafe extern "C" fn HasherSize(
    mut params: *const BrotliEncoderParams,
    mut one_shot: c_int,
    input_size: size_t,
    mut alloc_size: *mut size_t,
) {
    match (*params).hasher.type_0 {
        2 => {
            HashMemAllocInBytesH2(params, one_shot, input_size, alloc_size);
        }
        3 => {
            HashMemAllocInBytesH3(params, one_shot, input_size, alloc_size);
        }
        4 => {
            HashMemAllocInBytesH4(params, one_shot, input_size, alloc_size);
        }
        5 => {
            HashMemAllocInBytesH5(params, one_shot, input_size, alloc_size);
        }
        6 => {
            HashMemAllocInBytesH6(params, one_shot, input_size, alloc_size);
        }
        40 => {
            HashMemAllocInBytesH40(params, one_shot, input_size, alloc_size);
        }
        41 => {
            HashMemAllocInBytesH41(params, one_shot, input_size, alloc_size);
        }
        42 => {
            HashMemAllocInBytesH42(params, one_shot, input_size, alloc_size);
        }
        54 => {
            HashMemAllocInBytesH54(params, one_shot, input_size, alloc_size);
        }
        58 => {
            HashMemAllocInBytesH58(params, one_shot, input_size, alloc_size);
        }
        68 => {
            HashMemAllocInBytesH68(params, one_shot, input_size, alloc_size);
        }
        35 => {
            HashMemAllocInBytesH35(params, one_shot, input_size, alloc_size);
        }
        55 => {
            HashMemAllocInBytesH55(params, one_shot, input_size, alloc_size);
        }
        65 => {
            HashMemAllocInBytesH65(params, one_shot, input_size, alloc_size);
        }
        10 => {
            HashMemAllocInBytesH10(params, one_shot, input_size, alloc_size);
        }
        _ => {}
    };
}
#[inline(always)]
unsafe extern "C" fn HasherSetup(
    mut m: *mut MemoryManager,
    mut hasher: *mut Hasher,
    mut params: *mut BrotliEncoderParams,
    mut data: *const uint8_t,
    mut position: size_t,
    mut input_size: size_t,
    mut is_last: c_int,
) {
    let mut one_shot: c_int =
        (position == 0 as size_t && is_last != 0) as c_int;
    if (*hasher).common.is_setup_ == 0 {
        let mut alloc_size: [size_t; 4] = [0 as c_int as size_t, 0, 0, 0];
        let mut i: size_t = 0;
        ChooseHasher(params, &raw mut (*params).hasher);
        (*hasher).common.params = (*params).hasher;
        (*hasher).common.dict_num_lookups = 0 as size_t;
        (*hasher).common.dict_num_matches = 0 as size_t;
        HasherSize(
            params,
            one_shot,
            input_size,
            &raw mut alloc_size as *mut size_t,
        );
        i = 0 as size_t;
        while i < 4 as size_t {
            if !(alloc_size[i as usize] == 0 as size_t) {
                (*hasher).common.extra[i as usize] = (if alloc_size[i as usize] > 0 as size_t {
                    BrotliAllocate(
                        m,
                        alloc_size[i as usize]
                            .wrapping_mul(::core::mem::size_of::<uint8_t>() as size_t),
                    ) as *mut uint8_t
                } else {
                    ::core::ptr::null_mut::<uint8_t>()
                }) as *mut c_void;
                if 0 as c_int != 0 || 0 as c_int != 0 {
                    return;
                }
            }
            i = i.wrapping_add(1);
        }
        match (*hasher).common.params.type_0 {
            2 => {
                InitializeH2(
                    &raw mut (*hasher).common,
                    &raw mut (*hasher).privat._H2,
                    params,
                );
            }
            3 => {
                InitializeH3(
                    &raw mut (*hasher).common,
                    &raw mut (*hasher).privat._H3,
                    params,
                );
            }
            4 => {
                InitializeH4(
                    &raw mut (*hasher).common,
                    &raw mut (*hasher).privat._H4,
                    params,
                );
            }
            5 => {
                InitializeH5(
                    &raw mut (*hasher).common,
                    &raw mut (*hasher).privat._H5,
                    params,
                );
            }
            6 => {
                InitializeH6(
                    &raw mut (*hasher).common,
                    &raw mut (*hasher).privat._H6,
                    params,
                );
            }
            40 => {
                InitializeH40(
                    &raw mut (*hasher).common,
                    &raw mut (*hasher).privat._H40,
                    params,
                );
            }
            41 => {
                InitializeH41(
                    &raw mut (*hasher).common,
                    &raw mut (*hasher).privat._H41,
                    params,
                );
            }
            42 => {
                InitializeH42(
                    &raw mut (*hasher).common,
                    &raw mut (*hasher).privat._H42,
                    params,
                );
            }
            54 => {
                InitializeH54(
                    &raw mut (*hasher).common,
                    &raw mut (*hasher).privat._H54,
                    params,
                );
            }
            58 => {
                InitializeH58(
                    &raw mut (*hasher).common,
                    &raw mut (*hasher).privat._H58,
                    params,
                );
            }
            68 => {
                InitializeH68(
                    &raw mut (*hasher).common,
                    &raw mut (*hasher).privat._H68,
                    params,
                );
            }
            35 => {
                InitializeH35(
                    &raw mut (*hasher).common,
                    &raw mut (*hasher).privat._H35,
                    params,
                );
            }
            55 => {
                InitializeH55(
                    &raw mut (*hasher).common,
                    &raw mut (*hasher).privat._H55,
                    params,
                );
            }
            65 => {
                InitializeH65(
                    &raw mut (*hasher).common,
                    &raw mut (*hasher).privat._H65,
                    params,
                );
            }
            10 => {
                InitializeH10(
                    &raw mut (*hasher).common,
                    &raw mut (*hasher).privat._H10,
                    params,
                );
            }
            _ => {}
        }
        HasherReset(hasher);
        (*hasher).common.is_setup_ = BROTLI_TRUE;
    }
    if (*hasher).common.is_prepared_ == 0 {
        match (*hasher).common.params.type_0 {
            2 => {
                PrepareH2(&raw mut (*hasher).privat._H2, one_shot, input_size, data);
            }
            3 => {
                PrepareH3(&raw mut (*hasher).privat._H3, one_shot, input_size, data);
            }
            4 => {
                PrepareH4(&raw mut (*hasher).privat._H4, one_shot, input_size, data);
            }
            5 => {
                PrepareH5(&raw mut (*hasher).privat._H5, one_shot, input_size, data);
            }
            6 => {
                PrepareH6(&raw mut (*hasher).privat._H6, one_shot, input_size, data);
            }
            40 => {
                PrepareH40(&raw mut (*hasher).privat._H40, one_shot, input_size, data);
            }
            41 => {
                PrepareH41(&raw mut (*hasher).privat._H41, one_shot, input_size, data);
            }
            42 => {
                PrepareH42(&raw mut (*hasher).privat._H42, one_shot, input_size, data);
            }
            54 => {
                PrepareH54(&raw mut (*hasher).privat._H54, one_shot, input_size, data);
            }
            58 => {
                PrepareH58(&raw mut (*hasher).privat._H58, one_shot, input_size, data);
            }
            68 => {
                PrepareH68(&raw mut (*hasher).privat._H68, one_shot, input_size, data);
            }
            35 => {
                PrepareH35(&raw mut (*hasher).privat._H35, one_shot, input_size, data);
            }
            55 => {
                PrepareH55(&raw mut (*hasher).privat._H55, one_shot, input_size, data);
            }
            65 => {
                PrepareH65(&raw mut (*hasher).privat._H65, one_shot, input_size, data);
            }
            10 => {
                PrepareH10(&raw mut (*hasher).privat._H10, one_shot, input_size, data);
            }
            _ => {}
        }
        (*hasher).common.is_prepared_ = BROTLI_TRUE;
    }
}
#[inline(always)]
unsafe extern "C" fn InitOrStitchToPreviousBlock(
    mut m: *mut MemoryManager,
    mut hasher: *mut Hasher,
    mut data: *const uint8_t,
    mut mask: size_t,
    mut params: *mut BrotliEncoderParams,
    mut position: size_t,
    mut input_size: size_t,
    mut is_last: c_int,
) {
    HasherSetup(m, hasher, params, data, position, input_size, is_last);
    if 0 as c_int != 0 {
        return;
    }
    match (*hasher).common.params.type_0 {
        2 => {
            StitchToPreviousBlockH2(
                &raw mut (*hasher).privat._H2,
                input_size,
                position,
                data,
                mask,
            );
        }
        3 => {
            StitchToPreviousBlockH3(
                &raw mut (*hasher).privat._H3,
                input_size,
                position,
                data,
                mask,
            );
        }
        4 => {
            StitchToPreviousBlockH4(
                &raw mut (*hasher).privat._H4,
                input_size,
                position,
                data,
                mask,
            );
        }
        5 => {
            StitchToPreviousBlockH5(
                &raw mut (*hasher).privat._H5,
                input_size,
                position,
                data,
                mask,
            );
        }
        6 => {
            StitchToPreviousBlockH6(
                &raw mut (*hasher).privat._H6,
                input_size,
                position,
                data,
                mask,
            );
        }
        40 => {
            StitchToPreviousBlockH40(
                &raw mut (*hasher).privat._H40,
                input_size,
                position,
                data,
                mask,
            );
        }
        41 => {
            StitchToPreviousBlockH41(
                &raw mut (*hasher).privat._H41,
                input_size,
                position,
                data,
                mask,
            );
        }
        42 => {
            StitchToPreviousBlockH42(
                &raw mut (*hasher).privat._H42,
                input_size,
                position,
                data,
                mask,
            );
        }
        54 => {
            StitchToPreviousBlockH54(
                &raw mut (*hasher).privat._H54,
                input_size,
                position,
                data,
                mask,
            );
        }
        58 => {
            StitchToPreviousBlockH58(
                &raw mut (*hasher).privat._H58,
                input_size,
                position,
                data,
                mask,
            );
        }
        68 => {
            StitchToPreviousBlockH68(
                &raw mut (*hasher).privat._H68,
                input_size,
                position,
                data,
                mask,
            );
        }
        35 => {
            StitchToPreviousBlockH35(
                &raw mut (*hasher).privat._H35,
                input_size,
                position,
                data,
                mask,
            );
        }
        55 => {
            StitchToPreviousBlockH55(
                &raw mut (*hasher).privat._H55,
                input_size,
                position,
                data,
                mask,
            );
        }
        65 => {
            StitchToPreviousBlockH65(
                &raw mut (*hasher).privat._H65,
                input_size,
                position,
                data,
                mask,
            );
        }
        10 => {
            StitchToPreviousBlockH10(
                &raw mut (*hasher).privat._H10,
                input_size,
                position,
                data,
                mask,
            );
        }
        _ => {}
    };
}

#[inline(always)]
unsafe extern "C" fn RingBufferInit(mut rb: *mut RingBuffer) {
    (*rb).cur_size_ = 0 as uint32_t;
    (*rb).pos_ = 0 as uint32_t;
    (*rb).data_ = ::core::ptr::null_mut::<uint8_t>();
    (*rb).buffer_ = ::core::ptr::null_mut::<uint8_t>();
}
#[inline(always)]
unsafe extern "C" fn RingBufferSetup(
    mut params: *const BrotliEncoderParams,
    mut rb: *mut RingBuffer,
) {
    let mut window_bits: c_int = ComputeRbBits(params);
    let mut tail_bits: c_int = (*params).lgblock;
    *(&raw const (*rb).size_ as *mut uint32_t) =
        ((1 as c_uint) << window_bits) as uint32_t;
    *(&raw const (*rb).mask_ as *mut uint32_t) = ((1 as c_uint) << window_bits)
        .wrapping_sub(1 as c_uint)
        as uint32_t;
    *(&raw const (*rb).tail_size_ as *mut uint32_t) =
        ((1 as c_uint) << tail_bits) as uint32_t;
    *(&raw const (*rb).total_size_ as *mut uint32_t) = (*rb).size_.wrapping_add((*rb).tail_size_);
}
#[inline(always)]
unsafe extern "C" fn RingBufferFree(mut m: *mut MemoryManager, mut rb: *mut RingBuffer) {
    BrotliFree(m, (*rb).data_ as *mut c_void);
    (*rb).data_ = ::core::ptr::null_mut::<uint8_t>();
}
#[inline(always)]
unsafe extern "C" fn RingBufferInitBuffer(
    mut m: *mut MemoryManager,
    buflen: uint32_t,
    mut rb: *mut RingBuffer,
) {
    static mut kSlackForEightByteHashingEverywhere: size_t = 7 as size_t;
    let mut new_data: *mut uint8_t = if ((2 as uint32_t).wrapping_add(buflen) as size_t)
        .wrapping_add(kSlackForEightByteHashingEverywhere)
        > 0 as size_t
    {
        BrotliAllocate(
            m,
            ((2 as uint32_t).wrapping_add(buflen) as size_t)
                .wrapping_add(kSlackForEightByteHashingEverywhere)
                .wrapping_mul(::core::mem::size_of::<uint8_t>() as size_t),
        ) as *mut uint8_t
    } else {
        ::core::ptr::null_mut::<uint8_t>()
    };
    let mut i: size_t = 0;
    if 0 as c_int != 0 || 0 as c_int != 0 {
        return;
    }
    if !(*rb).data_.is_null() {
        memcpy(
            new_data as *mut c_void,
            (*rb).data_ as *const c_void,
            ((2 as uint32_t).wrapping_add((*rb).cur_size_) as size_t)
                .wrapping_add(kSlackForEightByteHashingEverywhere),
        );
        BrotliFree(m, (*rb).data_ as *mut c_void);
        (*rb).data_ = ::core::ptr::null_mut::<uint8_t>();
    }
    (*rb).data_ = new_data;
    (*rb).cur_size_ = buflen;
    (*rb).buffer_ = (*rb).data_.offset(2 as c_int as isize);
    let ref mut fresh19 = *(*rb).buffer_.offset(-(1 as c_int) as isize);
    *fresh19 = 0 as uint8_t;
    *(*rb).buffer_.offset(-(2 as c_int) as isize) = *fresh19;
    i = 0 as size_t;
    while i < kSlackForEightByteHashingEverywhere {
        *(*rb)
            .buffer_
            .offset(((*rb).cur_size_ as size_t).wrapping_add(i) as isize) = 0 as uint8_t;
        i = i.wrapping_add(1);
    }
}
#[inline(always)]
unsafe extern "C" fn RingBufferWriteTail(
    mut bytes: *const uint8_t,
    mut n: size_t,
    mut rb: *mut RingBuffer,
) {
    let masked_pos: size_t = ((*rb).pos_ & (*rb).mask_) as size_t;
    if (masked_pos < (*rb).tail_size_ as size_t) as c_int as c_long != 0 {
        let p: size_t = ((*rb).size_ as size_t).wrapping_add(masked_pos);
        memcpy(
            (*rb).buffer_.offset(p as isize) as *mut uint8_t as *mut c_void,
            bytes as *const c_void,
            brotli_min_size_t(n, ((*rb).tail_size_ as size_t).wrapping_sub(masked_pos)),
        );
    }
}
#[inline(always)]
unsafe extern "C" fn RingBufferWrite(
    mut m: *mut MemoryManager,
    mut bytes: *const uint8_t,
    mut n: size_t,
    mut rb: *mut RingBuffer,
) {
    if (*rb).pos_ == 0 as uint32_t && n < (*rb).tail_size_ as size_t {
        (*rb).pos_ = n as uint32_t;
        RingBufferInitBuffer(m, (*rb).pos_, rb);
        if 0 as c_int != 0 {
            return;
        }
        memcpy(
            (*rb).buffer_ as *mut c_void,
            bytes as *const c_void,
            n,
        );
        return;
    }
    if (*rb).cur_size_ < (*rb).total_size_ {
        RingBufferInitBuffer(m, (*rb).total_size_, rb);
        if 0 as c_int != 0 {
            return;
        }
        *(*rb)
            .buffer_
            .offset((*rb).size_.wrapping_sub(2 as uint32_t) as isize) = 0 as uint8_t;
        *(*rb)
            .buffer_
            .offset((*rb).size_.wrapping_sub(1 as uint32_t) as isize) = 0 as uint8_t;
        *(*rb).buffer_.offset((*rb).size_ as isize) = 241 as uint8_t;
    }
    let masked_pos: size_t = ((*rb).pos_ & (*rb).mask_) as size_t;
    RingBufferWriteTail(bytes, n, rb);
    if (masked_pos.wrapping_add(n) <= (*rb).size_ as size_t) as c_int
        as c_long
        != 0
    {
        memcpy(
            (*rb).buffer_.offset(masked_pos as isize) as *mut uint8_t as *mut c_void,
            bytes as *const c_void,
            n,
        );
    } else {
        memcpy(
            (*rb).buffer_.offset(masked_pos as isize) as *mut uint8_t as *mut c_void,
            bytes as *const c_void,
            brotli_min_size_t(n, ((*rb).total_size_ as size_t).wrapping_sub(masked_pos)),
        );
        memcpy(
            (*rb).buffer_.offset(0 as c_int as isize) as *mut uint8_t
                as *mut c_void,
            bytes.offset(((*rb).size_ as size_t).wrapping_sub(masked_pos) as isize)
                as *const c_void,
            n.wrapping_sub(((*rb).size_ as size_t).wrapping_sub(masked_pos)),
        );
    }
    let mut not_first_lap: c_int = ((*rb).pos_
        & (1 as uint32_t) << 31 as c_int
        != 0 as uint32_t) as c_int;
    let mut rb_pos_mask: uint32_t =
        ((1 as uint32_t) << 31 as c_int).wrapping_sub(1 as uint32_t);
    *(*rb).buffer_.offset(-(2 as c_int) as isize) = *(*rb)
        .buffer_
        .offset((*rb).size_.wrapping_sub(2 as uint32_t) as isize);
    *(*rb).buffer_.offset(-(1 as c_int) as isize) = *(*rb)
        .buffer_
        .offset((*rb).size_.wrapping_sub(1 as uint32_t) as isize);
    (*rb).pos_ = ((*rb).pos_ & rb_pos_mask).wrapping_add((n & rb_pos_mask as size_t) as uint32_t);
    if not_first_lap != 0 {
        (*rb).pos_ = ((*rb).pos_ as c_uint
            | (1 as c_uint) << 31 as c_int)
            as uint32_t;
    }
}
