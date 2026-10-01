use ::c2rust_bitfields;
extern "C" {
    pub type BrotliSharedDictionaryStruct;
    fn BrotliDefaultAllocFunc(
        opaque: *mut ::core::ffi::c_void,
        size: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn BrotliDefaultFreeFunc(opaque: *mut ::core::ffi::c_void, address: *mut ::core::ffi::c_void);
    fn BrotliSharedDictionaryCreateInstance(
        alloc_func: brotli_alloc_func,
        free_func: brotli_free_func,
        opaque: *mut ::core::ffi::c_void,
    ) -> *mut BrotliSharedDictionary;
    fn BrotliSharedDictionaryDestroyInstance(dict: *mut BrotliSharedDictionary);
    fn BrotliInitBitReader(br: *mut BrotliBitReader);
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
pub type BrotliSharedDictionary = BrotliSharedDictionaryStruct;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct BrotliDecoderStateStruct {
    pub state: BrotliRunningState,
    pub loop_counter: ::core::ffi::c_int,
    pub br: BrotliBitReader,
    pub alloc_func: brotli_alloc_func,
    pub free_func: brotli_free_func,
    pub memory_manager_opaque: *mut ::core::ffi::c_void,
    pub buffer: C2RustUnnamed_0,
    pub buffer_length: uint64_t,
    pub pos: ::core::ffi::c_int,
    pub max_backward_distance: ::core::ffi::c_int,
    pub max_distance: ::core::ffi::c_int,
    pub ringbuffer_size: ::core::ffi::c_int,
    pub ringbuffer_mask: ::core::ffi::c_int,
    pub dist_rb_idx: ::core::ffi::c_int,
    pub dist_rb: [::core::ffi::c_int; 4],
    pub error_code: ::core::ffi::c_int,
    pub meta_block_remaining_len: ::core::ffi::c_int,
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
    pub trivial_literal_context: ::core::ffi::c_int,
    pub distance_context: ::core::ffi::c_int,
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
    pub copy_length: ::core::ffi::c_int,
    pub distance_code: ::core::ffi::c_int,
    pub dist_htree_index: uint8_t,
    pub metadata_start_func: brotli_decoder_metadata_start_func,
    pub metadata_chunk_func: brotli_decoder_metadata_chunk_func,
    pub metadata_callback_opaque: *mut ::core::ffi::c_void,
    pub used_input: uint64_t,
    pub substate_metablock_header: BrotliRunningMetablockHeaderState,
    pub substate_uncompressed: BrotliRunningUncompressedState,
    pub substate_decode_uint8: BrotliRunningDecodeUint8State,
    pub substate_read_block_length: BrotliRunningReadBlockLengthState,
    pub new_ringbuffer_size: ::core::ffi::c_int,
    #[bitfield(name = "is_last_metablock", ty = "::core::ffi::c_uint", bits = "0..=0")]
    #[bitfield(name = "is_uncompressed", ty = "::core::ffi::c_uint", bits = "1..=1")]
    #[bitfield(name = "is_metadata", ty = "::core::ffi::c_uint", bits = "2..=2")]
    #[bitfield(
        name = "should_wrap_ringbuffer",
        ty = "::core::ffi::c_uint",
        bits = "3..=3"
    )]
    #[bitfield(
        name = "canny_ringbuffer_allocation",
        ty = "::core::ffi::c_uint",
        bits = "4..=4"
    )]
    #[bitfield(name = "large_window", ty = "::core::ffi::c_uint", bits = "5..=5")]
    #[bitfield(name = "window_bits", ty = "::core::ffi::c_uint", bits = "6..=11")]
    #[bitfield(name = "size_nibbles", ty = "::core::ffi::c_uint", bits = "12..=19")]
    pub is_last_metablock_is_uncompressed_is_metadata_should_wrap_ringbuffer_canny_ringbuffer_allocation_large_window_window_bits_size_nibbles:
        [u8; 3],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 1],
    pub num_literal_htrees: uint64_t,
    pub context_map: *mut uint8_t,
    pub context_modes: *mut uint8_t,
    pub dictionary: *mut BrotliSharedDictionary,
    pub compound_dictionary: *mut BrotliDecoderCompoundDictionary,
    pub trivial_literal_contexts: [uint32_t; 8],
    pub arena: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub header: BrotliMetablockHeaderArena,
    pub body: BrotliMetablockBodyArena,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BrotliMetablockBodyArena {
    pub dist_extra_bits: [uint8_t; 544],
    pub dist_offset: [uint64_t; 544],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BrotliMetablockHeaderArena {
    pub substate_tree_group: BrotliRunningTreeGroupState,
    pub substate_context_map: BrotliRunningContextMapState,
    pub substate_huffman: BrotliRunningHuffmanState,
    pub sub_loop_counter: uint64_t,
    pub repeat_code_len: uint64_t,
    pub prev_code_len: uint64_t,
    pub symbol: uint64_t,
    pub repeat: uint64_t,
    pub space: uint64_t,
    pub table: [HuffmanCode; 32],
    pub symbol_lists: *mut uint16_t,
    pub symbols_lists_array: [uint16_t; 720],
    pub next_symbol: [::core::ffi::c_int; 32],
    pub code_length_code_lengths: [uint8_t; 18],
    pub code_length_histo: [uint16_t; 16],
    pub htree_index: ::core::ffi::c_int,
    pub next: *mut HuffmanCode,
    pub context_index: uint64_t,
    pub max_run_length_prefix: uint64_t,
    pub code: uint64_t,
    pub context_map_table: [HuffmanCode; 646],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HuffmanCode {
    pub bits: uint8_t,
    pub value: uint16_t,
}
pub type BrotliRunningHuffmanState = ::core::ffi::c_uint;
pub const BROTLI_STATE_HUFFMAN_LENGTH_SYMBOLS: BrotliRunningHuffmanState = 5;
pub const BROTLI_STATE_HUFFMAN_COMPLEX: BrotliRunningHuffmanState = 4;
pub const BROTLI_STATE_HUFFMAN_SIMPLE_BUILD: BrotliRunningHuffmanState = 3;
pub const BROTLI_STATE_HUFFMAN_SIMPLE_READ: BrotliRunningHuffmanState = 2;
pub const BROTLI_STATE_HUFFMAN_SIMPLE_SIZE: BrotliRunningHuffmanState = 1;
pub const BROTLI_STATE_HUFFMAN_NONE: BrotliRunningHuffmanState = 0;
pub type BrotliRunningContextMapState = ::core::ffi::c_uint;
pub const BROTLI_STATE_CONTEXT_MAP_TRANSFORM: BrotliRunningContextMapState = 4;
pub const BROTLI_STATE_CONTEXT_MAP_DECODE: BrotliRunningContextMapState = 3;
pub const BROTLI_STATE_CONTEXT_MAP_HUFFMAN: BrotliRunningContextMapState = 2;
pub const BROTLI_STATE_CONTEXT_MAP_READ_PREFIX: BrotliRunningContextMapState = 1;
pub const BROTLI_STATE_CONTEXT_MAP_NONE: BrotliRunningContextMapState = 0;
pub type BrotliRunningTreeGroupState = ::core::ffi::c_uint;
pub const BROTLI_STATE_TREE_GROUP_LOOP: BrotliRunningTreeGroupState = 1;
pub const BROTLI_STATE_TREE_GROUP_NONE: BrotliRunningTreeGroupState = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BrotliDecoderCompoundDictionary {
    pub num_chunks: uint8_t,
    pub block_bits: uint8_t,
    pub br_index: uint16_t,
    pub total_size: uint32_t,
    pub br_offset: uint32_t,
    pub br_length: uint32_t,
    pub br_copied: uint32_t,
    pub chunks: [*const uint8_t; 16],
    pub chunk_offsets: [uint32_t; 16],
    pub block_map: [uint8_t; 256],
}
pub type BrotliRunningReadBlockLengthState = ::core::ffi::c_uint;
pub const BROTLI_STATE_READ_BLOCK_LENGTH_SUFFIX: BrotliRunningReadBlockLengthState = 1;
pub const BROTLI_STATE_READ_BLOCK_LENGTH_NONE: BrotliRunningReadBlockLengthState = 0;
pub type BrotliRunningDecodeUint8State = ::core::ffi::c_uint;
pub const BROTLI_STATE_DECODE_UINT8_LONG: BrotliRunningDecodeUint8State = 2;
pub const BROTLI_STATE_DECODE_UINT8_SHORT: BrotliRunningDecodeUint8State = 1;
pub const BROTLI_STATE_DECODE_UINT8_NONE: BrotliRunningDecodeUint8State = 0;
pub type BrotliRunningUncompressedState = ::core::ffi::c_uint;
pub const BROTLI_STATE_UNCOMPRESSED_WRITE: BrotliRunningUncompressedState = 1;
pub const BROTLI_STATE_UNCOMPRESSED_NONE: BrotliRunningUncompressedState = 0;
pub type BrotliRunningMetablockHeaderState = ::core::ffi::c_uint;
pub const BROTLI_STATE_METABLOCK_HEADER_METADATA: BrotliRunningMetablockHeaderState = 7;
pub const BROTLI_STATE_METABLOCK_HEADER_BYTES: BrotliRunningMetablockHeaderState = 6;
pub const BROTLI_STATE_METABLOCK_HEADER_RESERVED: BrotliRunningMetablockHeaderState = 5;
pub const BROTLI_STATE_METABLOCK_HEADER_UNCOMPRESSED: BrotliRunningMetablockHeaderState = 4;
pub const BROTLI_STATE_METABLOCK_HEADER_SIZE: BrotliRunningMetablockHeaderState = 3;
pub const BROTLI_STATE_METABLOCK_HEADER_NIBBLES: BrotliRunningMetablockHeaderState = 2;
pub const BROTLI_STATE_METABLOCK_HEADER_EMPTY: BrotliRunningMetablockHeaderState = 1;
pub const BROTLI_STATE_METABLOCK_HEADER_NONE: BrotliRunningMetablockHeaderState = 0;
pub type brotli_decoder_metadata_chunk_func =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *const uint8_t, size_t) -> ()>;
pub type brotli_decoder_metadata_start_func =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, size_t) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HuffmanTreeGroup {
    pub htrees: *mut *mut HuffmanCode,
    pub codes: *mut HuffmanCode,
    pub alphabet_size_max: uint16_t,
    pub alphabet_size_limit: uint16_t,
    pub num_htrees: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub u64_0: uint64_t,
    pub u8_0: [uint8_t; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BrotliBitReader {
    pub val_: uint64_t,
    pub bit_pos_: uint64_t,
    pub next_in: *const uint8_t,
    pub guard_in: *const uint8_t,
    pub last_in: *const uint8_t,
}
pub type BrotliRunningState = ::core::ffi::c_uint;
pub const BROTLI_STATE_DONE: BrotliRunningState = 26;
pub const BROTLI_STATE_BEFORE_COMPRESSED_METABLOCK_BODY: BrotliRunningState = 25;
pub const BROTLI_STATE_TREE_GROUP: BrotliRunningState = 24;
pub const BROTLI_STATE_CONTEXT_MAP_2: BrotliRunningState = 23;
pub const BROTLI_STATE_CONTEXT_MAP_1: BrotliRunningState = 22;
pub const BROTLI_STATE_HUFFMAN_CODE_3: BrotliRunningState = 21;
pub const BROTLI_STATE_HUFFMAN_CODE_2: BrotliRunningState = 20;
pub const BROTLI_STATE_HUFFMAN_CODE_1: BrotliRunningState = 19;
pub const BROTLI_STATE_HUFFMAN_CODE_0: BrotliRunningState = 18;
pub const BROTLI_STATE_BEFORE_COMPRESSED_METABLOCK_HEADER: BrotliRunningState = 17;
pub const BROTLI_STATE_COMMAND_POST_WRITE_2: BrotliRunningState = 16;
pub const BROTLI_STATE_COMMAND_POST_WRITE_1: BrotliRunningState = 15;
pub const BROTLI_STATE_METABLOCK_DONE: BrotliRunningState = 14;
pub const BROTLI_STATE_COMMAND_INNER_WRITE: BrotliRunningState = 13;
pub const BROTLI_STATE_METADATA: BrotliRunningState = 12;
pub const BROTLI_STATE_UNCOMPRESSED: BrotliRunningState = 11;
pub const BROTLI_STATE_COMMAND_POST_WRAP_COPY: BrotliRunningState = 10;
pub const BROTLI_STATE_COMMAND_POST_DECODE_LITERALS: BrotliRunningState = 9;
pub const BROTLI_STATE_COMMAND_INNER: BrotliRunningState = 8;
pub const BROTLI_STATE_COMMAND_BEGIN: BrotliRunningState = 7;
pub const BROTLI_STATE_CONTEXT_MODES: BrotliRunningState = 6;
pub const BROTLI_STATE_METABLOCK_HEADER_2: BrotliRunningState = 5;
pub const BROTLI_STATE_METABLOCK_HEADER: BrotliRunningState = 4;
pub const BROTLI_STATE_METABLOCK_BEGIN: BrotliRunningState = 3;
pub const BROTLI_STATE_INITIALIZE: BrotliRunningState = 2;
pub const BROTLI_STATE_LARGE_WINDOW_BITS: BrotliRunningState = 1;
pub const BROTLI_STATE_UNINITED: BrotliRunningState = 0;
pub type BrotliDecoderStateInternal = BrotliDecoderStateStruct;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const BROTLI_TRUE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const BROTLI_FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const BROTLI_BLOCK_SIZE_CAP: ::core::ffi::c_uint =
    (1 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn BrotliDecoderStateInit(
    mut s: *mut BrotliDecoderStateInternal,
    mut alloc_func: brotli_alloc_func,
    mut free_func: brotli_free_func,
    mut opaque: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    if alloc_func.is_none() {
        (*s).alloc_func = Some(
            BrotliDefaultAllocFunc
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    size_t,
                ) -> *mut ::core::ffi::c_void,
        ) as brotli_alloc_func;
        (*s).free_func = Some(
            BrotliDefaultFreeFunc
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void) -> (),
        ) as brotli_free_func;
        (*s).memory_manager_opaque = ::core::ptr::null_mut::<::core::ffi::c_void>();
    } else {
        (*s).alloc_func = alloc_func;
        (*s).free_func = free_func;
        (*s).memory_manager_opaque = opaque;
    }
    (*s).error_code = 0 as ::core::ffi::c_int;
    BrotliInitBitReader(&raw mut (*s).br);
    (*s).state = BROTLI_STATE_UNINITED;
    (*s).set_large_window(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    (*s).substate_metablock_header = BROTLI_STATE_METABLOCK_HEADER_NONE;
    (*s).substate_uncompressed = BROTLI_STATE_UNCOMPRESSED_NONE;
    (*s).substate_decode_uint8 = BROTLI_STATE_DECODE_UINT8_NONE;
    (*s).substate_read_block_length = BROTLI_STATE_READ_BLOCK_LENGTH_NONE;
    (*s).buffer_length = 0 as uint64_t;
    (*s).loop_counter = 0 as ::core::ffi::c_int;
    (*s).pos = 0 as ::core::ffi::c_int;
    (*s).rb_roundtrips = 0 as size_t;
    (*s).partial_pos_out = 0 as size_t;
    (*s).used_input = 0 as uint64_t;
    (*s).block_type_trees = ::core::ptr::null_mut::<HuffmanCode>();
    (*s).block_len_trees = ::core::ptr::null_mut::<HuffmanCode>();
    (*s).ringbuffer = ::core::ptr::null_mut::<uint8_t>();
    (*s).ringbuffer_size = 0 as ::core::ffi::c_int;
    (*s).new_ringbuffer_size = 0 as ::core::ffi::c_int;
    (*s).ringbuffer_mask = 0 as ::core::ffi::c_int;
    (*s).context_map = ::core::ptr::null_mut::<uint8_t>();
    (*s).context_modes = ::core::ptr::null_mut::<uint8_t>();
    (*s).dist_context_map = ::core::ptr::null_mut::<uint8_t>();
    (*s).context_map_slice = ::core::ptr::null_mut::<uint8_t>();
    (*s).dist_context_map_slice = ::core::ptr::null_mut::<uint8_t>();
    (*s).literal_hgroup.codes = ::core::ptr::null_mut::<HuffmanCode>();
    (*s).literal_hgroup.htrees = ::core::ptr::null_mut::<*mut HuffmanCode>();
    (*s).insert_copy_hgroup.codes = ::core::ptr::null_mut::<HuffmanCode>();
    (*s).insert_copy_hgroup.htrees = ::core::ptr::null_mut::<*mut HuffmanCode>();
    (*s).distance_hgroup.codes = ::core::ptr::null_mut::<HuffmanCode>();
    (*s).distance_hgroup.htrees = ::core::ptr::null_mut::<*mut HuffmanCode>();
    (*s).set_is_last_metablock(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    (*s).set_is_uncompressed(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    (*s).set_is_metadata(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    (*s).set_should_wrap_ringbuffer(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    (*s).set_canny_ringbuffer_allocation(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    (*s).set_window_bits(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    (*s).max_distance = 0 as ::core::ffi::c_int;
    (*s).dist_rb[0 as ::core::ffi::c_int as usize] = 16 as ::core::ffi::c_int;
    (*s).dist_rb[1 as ::core::ffi::c_int as usize] = 15 as ::core::ffi::c_int;
    (*s).dist_rb[2 as ::core::ffi::c_int as usize] = 11 as ::core::ffi::c_int;
    (*s).dist_rb[3 as ::core::ffi::c_int as usize] = 4 as ::core::ffi::c_int;
    (*s).dist_rb_idx = 0 as ::core::ffi::c_int;
    (*s).block_type_trees = ::core::ptr::null_mut::<HuffmanCode>();
    (*s).block_len_trees = ::core::ptr::null_mut::<HuffmanCode>();
    (*s).mtf_upper_bound = 63 as uint64_t;
    (*s).compound_dictionary = ::core::ptr::null_mut::<BrotliDecoderCompoundDictionary>();
    (*s).dictionary = BrotliSharedDictionaryCreateInstance(alloc_func, free_func, opaque);
    if (*s).dictionary.is_null() {
        return BROTLI_FALSE;
    }
    (*s).metadata_start_func = None;
    (*s).metadata_chunk_func = None;
    (*s).metadata_callback_opaque = ::core::ptr::null_mut::<::core::ffi::c_void>();
    return BROTLI_TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn BrotliDecoderStateMetablockBegin(mut s: *mut BrotliDecoderStateInternal) {
    (*s).meta_block_remaining_len = 0 as ::core::ffi::c_int;
    (*s).block_length[0 as ::core::ffi::c_int as usize] = BROTLI_BLOCK_SIZE_CAP as uint64_t;
    (*s).block_length[1 as ::core::ffi::c_int as usize] = BROTLI_BLOCK_SIZE_CAP as uint64_t;
    (*s).block_length[2 as ::core::ffi::c_int as usize] = BROTLI_BLOCK_SIZE_CAP as uint64_t;
    (*s).num_block_types[0 as ::core::ffi::c_int as usize] = 1 as uint64_t;
    (*s).num_block_types[1 as ::core::ffi::c_int as usize] = 1 as uint64_t;
    (*s).num_block_types[2 as ::core::ffi::c_int as usize] = 1 as uint64_t;
    (*s).block_type_rb[0 as ::core::ffi::c_int as usize] = 1 as uint64_t;
    (*s).block_type_rb[1 as ::core::ffi::c_int as usize] = 0 as uint64_t;
    (*s).block_type_rb[2 as ::core::ffi::c_int as usize] = 1 as uint64_t;
    (*s).block_type_rb[3 as ::core::ffi::c_int as usize] = 0 as uint64_t;
    (*s).block_type_rb[4 as ::core::ffi::c_int as usize] = 1 as uint64_t;
    (*s).block_type_rb[5 as ::core::ffi::c_int as usize] = 0 as uint64_t;
    (*s).context_map = ::core::ptr::null_mut::<uint8_t>();
    (*s).context_modes = ::core::ptr::null_mut::<uint8_t>();
    (*s).dist_context_map = ::core::ptr::null_mut::<uint8_t>();
    (*s).context_map_slice = ::core::ptr::null_mut::<uint8_t>();
    (*s).literal_htree = ::core::ptr::null_mut::<HuffmanCode>();
    (*s).dist_context_map_slice = ::core::ptr::null_mut::<uint8_t>();
    (*s).dist_htree_index = 0 as uint8_t;
    (*s).context_lookup = ::core::ptr::null::<uint8_t>();
    (*s).literal_hgroup.codes = ::core::ptr::null_mut::<HuffmanCode>();
    (*s).literal_hgroup.htrees = ::core::ptr::null_mut::<*mut HuffmanCode>();
    (*s).insert_copy_hgroup.codes = ::core::ptr::null_mut::<HuffmanCode>();
    (*s).insert_copy_hgroup.htrees = ::core::ptr::null_mut::<*mut HuffmanCode>();
    (*s).distance_hgroup.codes = ::core::ptr::null_mut::<HuffmanCode>();
    (*s).distance_hgroup.htrees = ::core::ptr::null_mut::<*mut HuffmanCode>();
}
#[no_mangle]
pub unsafe extern "C" fn BrotliDecoderStateCleanupAfterMetablock(
    mut s: *mut BrotliDecoderStateInternal,
) {
    (*s).free_func.expect("non-null function pointer")(
        (*s).memory_manager_opaque,
        (*s).context_modes as *mut ::core::ffi::c_void,
    );
    (*s).context_modes = ::core::ptr::null_mut::<uint8_t>();
    (*s).free_func.expect("non-null function pointer")(
        (*s).memory_manager_opaque,
        (*s).context_map as *mut ::core::ffi::c_void,
    );
    (*s).context_map = ::core::ptr::null_mut::<uint8_t>();
    (*s).free_func.expect("non-null function pointer")(
        (*s).memory_manager_opaque,
        (*s).dist_context_map as *mut ::core::ffi::c_void,
    );
    (*s).dist_context_map = ::core::ptr::null_mut::<uint8_t>();
    (*s).free_func.expect("non-null function pointer")(
        (*s).memory_manager_opaque,
        (*s).literal_hgroup.htrees as *mut ::core::ffi::c_void,
    );
    (*s).literal_hgroup.htrees = ::core::ptr::null_mut::<*mut HuffmanCode>();
    (*s).free_func.expect("non-null function pointer")(
        (*s).memory_manager_opaque,
        (*s).insert_copy_hgroup.htrees as *mut ::core::ffi::c_void,
    );
    (*s).insert_copy_hgroup.htrees = ::core::ptr::null_mut::<*mut HuffmanCode>();
    (*s).free_func.expect("non-null function pointer")(
        (*s).memory_manager_opaque,
        (*s).distance_hgroup.htrees as *mut ::core::ffi::c_void,
    );
    (*s).distance_hgroup.htrees = ::core::ptr::null_mut::<*mut HuffmanCode>();
}
#[no_mangle]
pub unsafe extern "C" fn BrotliDecoderStateCleanup(mut s: *mut BrotliDecoderStateInternal) {
    BrotliDecoderStateCleanupAfterMetablock(s);
    (*s).free_func.expect("non-null function pointer")(
        (*s).memory_manager_opaque,
        (*s).compound_dictionary as *mut ::core::ffi::c_void,
    );
    (*s).compound_dictionary = ::core::ptr::null_mut::<BrotliDecoderCompoundDictionary>();
    BrotliSharedDictionaryDestroyInstance((*s).dictionary);
    (*s).dictionary = ::core::ptr::null_mut::<BrotliSharedDictionary>();
    (*s).free_func.expect("non-null function pointer")(
        (*s).memory_manager_opaque,
        (*s).ringbuffer as *mut ::core::ffi::c_void,
    );
    (*s).ringbuffer = ::core::ptr::null_mut::<uint8_t>();
    (*s).free_func.expect("non-null function pointer")(
        (*s).memory_manager_opaque,
        (*s).block_type_trees as *mut ::core::ffi::c_void,
    );
    (*s).block_type_trees = ::core::ptr::null_mut::<HuffmanCode>();
}
#[no_mangle]
pub unsafe extern "C" fn BrotliDecoderHuffmanTreeGroupInit(
    mut s: *mut BrotliDecoderStateInternal,
    mut group: *mut HuffmanTreeGroup,
    mut alphabet_size_max: uint64_t,
    mut alphabet_size_limit: uint64_t,
    mut ntrees: uint64_t,
) -> ::core::ffi::c_int {
    let max_table_size: size_t = (alphabet_size_limit as size_t).wrapping_add(376 as size_t);
    let code_size: size_t = (::core::mem::size_of::<HuffmanCode>() as size_t)
        .wrapping_mul(ntrees as size_t)
        .wrapping_mul(max_table_size);
    let htree_size: size_t =
        (::core::mem::size_of::<*mut HuffmanCode>() as size_t).wrapping_mul(ntrees as size_t);
    let mut p: *mut *mut HuffmanCode = (*s).alloc_func.expect("non-null function pointer")(
        (*s).memory_manager_opaque,
        code_size.wrapping_add(htree_size),
    ) as *mut *mut HuffmanCode;
    (*group).alphabet_size_max = alphabet_size_max as uint16_t;
    (*group).alphabet_size_limit = alphabet_size_limit as uint16_t;
    (*group).num_htrees = ntrees as uint16_t;
    (*group).htrees = p;
    (*group).codes = if !p.is_null() {
        p.offset(ntrees as isize) as *mut *mut HuffmanCode as *mut HuffmanCode
    } else {
        ::core::ptr::null_mut::<HuffmanCode>()
    };
    return !p.is_null() as ::core::ffi::c_int;
}
