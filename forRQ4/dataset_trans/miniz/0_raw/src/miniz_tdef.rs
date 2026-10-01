extern "C" {
    fn mz_adler32(adler: mz_ulong, ptr: *const ::core::ffi::c_uchar, buf_len: size_t) -> mz_ulong;
    fn mz_crc32(crc: mz_ulong, ptr: *const ::core::ffi::c_uchar, buf_len: size_t) -> mz_ulong;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn realloc(__ptr: *mut ::core::ffi::c_void, __size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
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
}
pub type size_t = usize;
pub type __uint16_t = u16;
pub type __uint32_t = u32;
pub type mz_ulong = ::core::ffi::c_ulong;
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const MZ_FIXED: C2RustUnnamed = 4;
pub const MZ_RLE: C2RustUnnamed = 3;
pub const MZ_HUFFMAN_ONLY: C2RustUnnamed = 2;
pub const MZ_FILTERED: C2RustUnnamed = 1;
pub const MZ_DEFAULT_STRATEGY: C2RustUnnamed = 0;
pub type C2RustUnnamed_0 = ::core::ffi::c_int;
pub const MZ_DEFAULT_COMPRESSION: C2RustUnnamed_0 = -1;
pub const MZ_DEFAULT_LEVEL: C2RustUnnamed_0 = 6;
pub const MZ_UBER_COMPRESSION: C2RustUnnamed_0 = 10;
pub const MZ_BEST_COMPRESSION: C2RustUnnamed_0 = 9;
pub const MZ_BEST_SPEED: C2RustUnnamed_0 = 1;
pub const MZ_NO_COMPRESSION: C2RustUnnamed_0 = 0;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type mz_uint8 = ::core::ffi::c_uchar;
pub type mz_uint16 = uint16_t;
pub type mz_uint32 = uint32_t;
pub type mz_uint = uint32_t;
pub type mz_bool = ::core::ffi::c_int;
pub type C2RustUnnamed_1 = ::core::ffi::c_uint;
pub const TDEFL_MAX_PROBES_MASK: C2RustUnnamed_1 = 4095;
pub const TDEFL_DEFAULT_MAX_PROBES: C2RustUnnamed_1 = 128;
pub const TDEFL_HUFFMAN_ONLY: C2RustUnnamed_1 = 0;
pub type C2RustUnnamed_2 = ::core::ffi::c_uint;
pub const TDEFL_FORCE_ALL_RAW_BLOCKS: C2RustUnnamed_2 = 524288;
pub const TDEFL_FORCE_ALL_STATIC_BLOCKS: C2RustUnnamed_2 = 262144;
pub const TDEFL_FILTER_MATCHES: C2RustUnnamed_2 = 131072;
pub const TDEFL_RLE_MATCHES: C2RustUnnamed_2 = 65536;
pub const TDEFL_NONDETERMINISTIC_PARSING_FLAG: C2RustUnnamed_2 = 32768;
pub const TDEFL_GREEDY_PARSING_FLAG: C2RustUnnamed_2 = 16384;
pub const TDEFL_COMPUTE_ADLER32: C2RustUnnamed_2 = 8192;
pub const TDEFL_WRITE_ZLIB_HEADER: C2RustUnnamed_2 = 4096;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tdefl_output_buffer {
    pub m_size: size_t,
    pub m_capacity: size_t,
    pub m_pBuf: *mut mz_uint8,
    pub m_expandable: mz_bool,
}
pub type tdefl_put_buf_func_ptr = Option<
    unsafe extern "C" fn(
        *const ::core::ffi::c_void,
        ::core::ffi::c_int,
        *mut ::core::ffi::c_void,
    ) -> mz_bool,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tdefl_compressor {
    pub m_pPut_buf_func: tdefl_put_buf_func_ptr,
    pub m_pPut_buf_user: *mut ::core::ffi::c_void,
    pub m_flags: mz_uint,
    pub m_max_probes: [mz_uint; 2],
    pub m_greedy_parsing: ::core::ffi::c_int,
    pub m_adler32: mz_uint,
    pub m_lookahead_pos: mz_uint,
    pub m_lookahead_size: mz_uint,
    pub m_dict_size: mz_uint,
    pub m_pLZ_code_buf: *mut mz_uint8,
    pub m_pLZ_flags: *mut mz_uint8,
    pub m_pOutput_buf: *mut mz_uint8,
    pub m_pOutput_buf_end: *mut mz_uint8,
    pub m_num_flags_left: mz_uint,
    pub m_total_lz_bytes: mz_uint,
    pub m_lz_code_buf_dict_pos: mz_uint,
    pub m_bits_in: mz_uint,
    pub m_bit_buffer: mz_uint,
    pub m_saved_match_dist: mz_uint,
    pub m_saved_match_len: mz_uint,
    pub m_saved_lit: mz_uint,
    pub m_output_flush_ofs: mz_uint,
    pub m_output_flush_remaining: mz_uint,
    pub m_finished: mz_uint,
    pub m_block_index: mz_uint,
    pub m_wants_to_finish: mz_uint,
    pub m_prev_return_status: tdefl_status,
    pub m_pIn_buf: *const ::core::ffi::c_void,
    pub m_pOut_buf: *mut ::core::ffi::c_void,
    pub m_pIn_buf_size: *mut size_t,
    pub m_pOut_buf_size: *mut size_t,
    pub m_flush: tdefl_flush,
    pub m_pSrc: *const mz_uint8,
    pub m_src_buf_left: size_t,
    pub m_out_buf_ofs: size_t,
    pub m_dict: [mz_uint8; 33025],
    pub m_huff_count: [[mz_uint16; 288]; 3],
    pub m_huff_codes: [[mz_uint16; 288]; 3],
    pub m_huff_code_sizes: [[mz_uint8; 288]; 3],
    pub m_lz_code_buf: [mz_uint8; 65536],
    pub m_next: [mz_uint16; 32768],
    pub m_hash: [mz_uint16; 32768],
    pub m_output_buf: [mz_uint8; 85196],
}
pub type tdefl_flush = ::core::ffi::c_uint;
pub const TDEFL_FINISH: tdefl_flush = 4;
pub const TDEFL_FULL_FLUSH: tdefl_flush = 3;
pub const TDEFL_SYNC_FLUSH: tdefl_flush = 2;
pub const TDEFL_NO_FLUSH: tdefl_flush = 0;
pub type tdefl_status = ::core::ffi::c_int;
pub const TDEFL_STATUS_DONE: tdefl_status = 1;
pub const TDEFL_STATUS_OKAY: tdefl_status = 0;
pub const TDEFL_STATUS_PUT_BUF_FAILED: tdefl_status = -1;
pub const TDEFL_STATUS_BAD_PARAM: tdefl_status = -2;
pub const TDEFL_OUT_BUF_SIZE: C2RustUnnamed_4 = 85196;
pub const TDEFL_MAX_HUFF_SYMBOLS_1: C2RustUnnamed_3 = 32;
pub const TDEFL_MAX_HUFF_SYMBOLS_0: C2RustUnnamed_3 = 288;
pub const TDEFL_MAX_HUFF_SYMBOLS_2: C2RustUnnamed_3 = 19;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tdefl_sym_freq {
    pub m_key: mz_uint16,
    pub m_sym_index: mz_uint16,
}
pub const TDEFL_MAX_SUPPORTED_HUFF_CODESIZE: C2RustUnnamed_5 = 32;
pub const TDEFL_LZ_DICT_SIZE_MASK: C2RustUnnamed_3 = 32767;
pub const TDEFL_LZ_CODE_BUF_SIZE: C2RustUnnamed_4 = 65536;
pub const TDEFL_LZ_DICT_SIZE: C2RustUnnamed_3 = 32768;
pub const TDEFL_MIN_MATCH_LEN: C2RustUnnamed_3 = 3;
pub const TDEFL_MAX_MATCH_LEN: C2RustUnnamed_3 = 258;
pub const TDEFL_LZ_HASH_SIZE: C2RustUnnamed_4 = 32768;
pub const TDEFL_LZ_HASH_SHIFT: C2RustUnnamed_4 = 5;
pub type C2RustUnnamed_3 = ::core::ffi::c_uint;
pub const TDEFL_MAX_HUFF_TABLES: C2RustUnnamed_3 = 3;
pub type C2RustUnnamed_4 = ::core::ffi::c_uint;
pub const TDEFL_LEVEL1_HASH_SIZE_MASK: C2RustUnnamed_4 = 4095;
pub const TDEFL_LZ_HASH_BITS: C2RustUnnamed_4 = 15;
pub const TDEFL_MAX_HUFF_SYMBOLS: C2RustUnnamed_4 = 288;
pub type C2RustUnnamed_5 = ::core::ffi::c_uint;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const MZ_CRC32_INIT: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const MZ_FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const MZ_TRUE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
static mut s_tdefl_len_sym: [mz_uint16; 256] = [
    257 as ::core::ffi::c_int as mz_uint16,
    258 as ::core::ffi::c_int as mz_uint16,
    259 as ::core::ffi::c_int as mz_uint16,
    260 as ::core::ffi::c_int as mz_uint16,
    261 as ::core::ffi::c_int as mz_uint16,
    262 as ::core::ffi::c_int as mz_uint16,
    263 as ::core::ffi::c_int as mz_uint16,
    264 as ::core::ffi::c_int as mz_uint16,
    265 as ::core::ffi::c_int as mz_uint16,
    265 as ::core::ffi::c_int as mz_uint16,
    266 as ::core::ffi::c_int as mz_uint16,
    266 as ::core::ffi::c_int as mz_uint16,
    267 as ::core::ffi::c_int as mz_uint16,
    267 as ::core::ffi::c_int as mz_uint16,
    268 as ::core::ffi::c_int as mz_uint16,
    268 as ::core::ffi::c_int as mz_uint16,
    269 as ::core::ffi::c_int as mz_uint16,
    269 as ::core::ffi::c_int as mz_uint16,
    269 as ::core::ffi::c_int as mz_uint16,
    269 as ::core::ffi::c_int as mz_uint16,
    270 as ::core::ffi::c_int as mz_uint16,
    270 as ::core::ffi::c_int as mz_uint16,
    270 as ::core::ffi::c_int as mz_uint16,
    270 as ::core::ffi::c_int as mz_uint16,
    271 as ::core::ffi::c_int as mz_uint16,
    271 as ::core::ffi::c_int as mz_uint16,
    271 as ::core::ffi::c_int as mz_uint16,
    271 as ::core::ffi::c_int as mz_uint16,
    272 as ::core::ffi::c_int as mz_uint16,
    272 as ::core::ffi::c_int as mz_uint16,
    272 as ::core::ffi::c_int as mz_uint16,
    272 as ::core::ffi::c_int as mz_uint16,
    273 as ::core::ffi::c_int as mz_uint16,
    273 as ::core::ffi::c_int as mz_uint16,
    273 as ::core::ffi::c_int as mz_uint16,
    273 as ::core::ffi::c_int as mz_uint16,
    273 as ::core::ffi::c_int as mz_uint16,
    273 as ::core::ffi::c_int as mz_uint16,
    273 as ::core::ffi::c_int as mz_uint16,
    273 as ::core::ffi::c_int as mz_uint16,
    274 as ::core::ffi::c_int as mz_uint16,
    274 as ::core::ffi::c_int as mz_uint16,
    274 as ::core::ffi::c_int as mz_uint16,
    274 as ::core::ffi::c_int as mz_uint16,
    274 as ::core::ffi::c_int as mz_uint16,
    274 as ::core::ffi::c_int as mz_uint16,
    274 as ::core::ffi::c_int as mz_uint16,
    274 as ::core::ffi::c_int as mz_uint16,
    275 as ::core::ffi::c_int as mz_uint16,
    275 as ::core::ffi::c_int as mz_uint16,
    275 as ::core::ffi::c_int as mz_uint16,
    275 as ::core::ffi::c_int as mz_uint16,
    275 as ::core::ffi::c_int as mz_uint16,
    275 as ::core::ffi::c_int as mz_uint16,
    275 as ::core::ffi::c_int as mz_uint16,
    275 as ::core::ffi::c_int as mz_uint16,
    276 as ::core::ffi::c_int as mz_uint16,
    276 as ::core::ffi::c_int as mz_uint16,
    276 as ::core::ffi::c_int as mz_uint16,
    276 as ::core::ffi::c_int as mz_uint16,
    276 as ::core::ffi::c_int as mz_uint16,
    276 as ::core::ffi::c_int as mz_uint16,
    276 as ::core::ffi::c_int as mz_uint16,
    276 as ::core::ffi::c_int as mz_uint16,
    277 as ::core::ffi::c_int as mz_uint16,
    277 as ::core::ffi::c_int as mz_uint16,
    277 as ::core::ffi::c_int as mz_uint16,
    277 as ::core::ffi::c_int as mz_uint16,
    277 as ::core::ffi::c_int as mz_uint16,
    277 as ::core::ffi::c_int as mz_uint16,
    277 as ::core::ffi::c_int as mz_uint16,
    277 as ::core::ffi::c_int as mz_uint16,
    277 as ::core::ffi::c_int as mz_uint16,
    277 as ::core::ffi::c_int as mz_uint16,
    277 as ::core::ffi::c_int as mz_uint16,
    277 as ::core::ffi::c_int as mz_uint16,
    277 as ::core::ffi::c_int as mz_uint16,
    277 as ::core::ffi::c_int as mz_uint16,
    277 as ::core::ffi::c_int as mz_uint16,
    277 as ::core::ffi::c_int as mz_uint16,
    278 as ::core::ffi::c_int as mz_uint16,
    278 as ::core::ffi::c_int as mz_uint16,
    278 as ::core::ffi::c_int as mz_uint16,
    278 as ::core::ffi::c_int as mz_uint16,
    278 as ::core::ffi::c_int as mz_uint16,
    278 as ::core::ffi::c_int as mz_uint16,
    278 as ::core::ffi::c_int as mz_uint16,
    278 as ::core::ffi::c_int as mz_uint16,
    278 as ::core::ffi::c_int as mz_uint16,
    278 as ::core::ffi::c_int as mz_uint16,
    278 as ::core::ffi::c_int as mz_uint16,
    278 as ::core::ffi::c_int as mz_uint16,
    278 as ::core::ffi::c_int as mz_uint16,
    278 as ::core::ffi::c_int as mz_uint16,
    278 as ::core::ffi::c_int as mz_uint16,
    278 as ::core::ffi::c_int as mz_uint16,
    279 as ::core::ffi::c_int as mz_uint16,
    279 as ::core::ffi::c_int as mz_uint16,
    279 as ::core::ffi::c_int as mz_uint16,
    279 as ::core::ffi::c_int as mz_uint16,
    279 as ::core::ffi::c_int as mz_uint16,
    279 as ::core::ffi::c_int as mz_uint16,
    279 as ::core::ffi::c_int as mz_uint16,
    279 as ::core::ffi::c_int as mz_uint16,
    279 as ::core::ffi::c_int as mz_uint16,
    279 as ::core::ffi::c_int as mz_uint16,
    279 as ::core::ffi::c_int as mz_uint16,
    279 as ::core::ffi::c_int as mz_uint16,
    279 as ::core::ffi::c_int as mz_uint16,
    279 as ::core::ffi::c_int as mz_uint16,
    279 as ::core::ffi::c_int as mz_uint16,
    279 as ::core::ffi::c_int as mz_uint16,
    280 as ::core::ffi::c_int as mz_uint16,
    280 as ::core::ffi::c_int as mz_uint16,
    280 as ::core::ffi::c_int as mz_uint16,
    280 as ::core::ffi::c_int as mz_uint16,
    280 as ::core::ffi::c_int as mz_uint16,
    280 as ::core::ffi::c_int as mz_uint16,
    280 as ::core::ffi::c_int as mz_uint16,
    280 as ::core::ffi::c_int as mz_uint16,
    280 as ::core::ffi::c_int as mz_uint16,
    280 as ::core::ffi::c_int as mz_uint16,
    280 as ::core::ffi::c_int as mz_uint16,
    280 as ::core::ffi::c_int as mz_uint16,
    280 as ::core::ffi::c_int as mz_uint16,
    280 as ::core::ffi::c_int as mz_uint16,
    280 as ::core::ffi::c_int as mz_uint16,
    280 as ::core::ffi::c_int as mz_uint16,
    281 as ::core::ffi::c_int as mz_uint16,
    281 as ::core::ffi::c_int as mz_uint16,
    281 as ::core::ffi::c_int as mz_uint16,
    281 as ::core::ffi::c_int as mz_uint16,
    281 as ::core::ffi::c_int as mz_uint16,
    281 as ::core::ffi::c_int as mz_uint16,
    281 as ::core::ffi::c_int as mz_uint16,
    281 as ::core::ffi::c_int as mz_uint16,
    281 as ::core::ffi::c_int as mz_uint16,
    281 as ::core::ffi::c_int as mz_uint16,
    281 as ::core::ffi::c_int as mz_uint16,
    281 as ::core::ffi::c_int as mz_uint16,
    281 as ::core::ffi::c_int as mz_uint16,
    281 as ::core::ffi::c_int as mz_uint16,
    281 as ::core::ffi::c_int as mz_uint16,
    281 as ::core::ffi::c_int as mz_uint16,
    281 as ::core::ffi::c_int as mz_uint16,
    281 as ::core::ffi::c_int as mz_uint16,
    281 as ::core::ffi::c_int as mz_uint16,
    281 as ::core::ffi::c_int as mz_uint16,
    281 as ::core::ffi::c_int as mz_uint16,
    281 as ::core::ffi::c_int as mz_uint16,
    281 as ::core::ffi::c_int as mz_uint16,
    281 as ::core::ffi::c_int as mz_uint16,
    281 as ::core::ffi::c_int as mz_uint16,
    281 as ::core::ffi::c_int as mz_uint16,
    281 as ::core::ffi::c_int as mz_uint16,
    281 as ::core::ffi::c_int as mz_uint16,
    281 as ::core::ffi::c_int as mz_uint16,
    281 as ::core::ffi::c_int as mz_uint16,
    281 as ::core::ffi::c_int as mz_uint16,
    281 as ::core::ffi::c_int as mz_uint16,
    282 as ::core::ffi::c_int as mz_uint16,
    282 as ::core::ffi::c_int as mz_uint16,
    282 as ::core::ffi::c_int as mz_uint16,
    282 as ::core::ffi::c_int as mz_uint16,
    282 as ::core::ffi::c_int as mz_uint16,
    282 as ::core::ffi::c_int as mz_uint16,
    282 as ::core::ffi::c_int as mz_uint16,
    282 as ::core::ffi::c_int as mz_uint16,
    282 as ::core::ffi::c_int as mz_uint16,
    282 as ::core::ffi::c_int as mz_uint16,
    282 as ::core::ffi::c_int as mz_uint16,
    282 as ::core::ffi::c_int as mz_uint16,
    282 as ::core::ffi::c_int as mz_uint16,
    282 as ::core::ffi::c_int as mz_uint16,
    282 as ::core::ffi::c_int as mz_uint16,
    282 as ::core::ffi::c_int as mz_uint16,
    282 as ::core::ffi::c_int as mz_uint16,
    282 as ::core::ffi::c_int as mz_uint16,
    282 as ::core::ffi::c_int as mz_uint16,
    282 as ::core::ffi::c_int as mz_uint16,
    282 as ::core::ffi::c_int as mz_uint16,
    282 as ::core::ffi::c_int as mz_uint16,
    282 as ::core::ffi::c_int as mz_uint16,
    282 as ::core::ffi::c_int as mz_uint16,
    282 as ::core::ffi::c_int as mz_uint16,
    282 as ::core::ffi::c_int as mz_uint16,
    282 as ::core::ffi::c_int as mz_uint16,
    282 as ::core::ffi::c_int as mz_uint16,
    282 as ::core::ffi::c_int as mz_uint16,
    282 as ::core::ffi::c_int as mz_uint16,
    282 as ::core::ffi::c_int as mz_uint16,
    282 as ::core::ffi::c_int as mz_uint16,
    283 as ::core::ffi::c_int as mz_uint16,
    283 as ::core::ffi::c_int as mz_uint16,
    283 as ::core::ffi::c_int as mz_uint16,
    283 as ::core::ffi::c_int as mz_uint16,
    283 as ::core::ffi::c_int as mz_uint16,
    283 as ::core::ffi::c_int as mz_uint16,
    283 as ::core::ffi::c_int as mz_uint16,
    283 as ::core::ffi::c_int as mz_uint16,
    283 as ::core::ffi::c_int as mz_uint16,
    283 as ::core::ffi::c_int as mz_uint16,
    283 as ::core::ffi::c_int as mz_uint16,
    283 as ::core::ffi::c_int as mz_uint16,
    283 as ::core::ffi::c_int as mz_uint16,
    283 as ::core::ffi::c_int as mz_uint16,
    283 as ::core::ffi::c_int as mz_uint16,
    283 as ::core::ffi::c_int as mz_uint16,
    283 as ::core::ffi::c_int as mz_uint16,
    283 as ::core::ffi::c_int as mz_uint16,
    283 as ::core::ffi::c_int as mz_uint16,
    283 as ::core::ffi::c_int as mz_uint16,
    283 as ::core::ffi::c_int as mz_uint16,
    283 as ::core::ffi::c_int as mz_uint16,
    283 as ::core::ffi::c_int as mz_uint16,
    283 as ::core::ffi::c_int as mz_uint16,
    283 as ::core::ffi::c_int as mz_uint16,
    283 as ::core::ffi::c_int as mz_uint16,
    283 as ::core::ffi::c_int as mz_uint16,
    283 as ::core::ffi::c_int as mz_uint16,
    283 as ::core::ffi::c_int as mz_uint16,
    283 as ::core::ffi::c_int as mz_uint16,
    283 as ::core::ffi::c_int as mz_uint16,
    283 as ::core::ffi::c_int as mz_uint16,
    284 as ::core::ffi::c_int as mz_uint16,
    284 as ::core::ffi::c_int as mz_uint16,
    284 as ::core::ffi::c_int as mz_uint16,
    284 as ::core::ffi::c_int as mz_uint16,
    284 as ::core::ffi::c_int as mz_uint16,
    284 as ::core::ffi::c_int as mz_uint16,
    284 as ::core::ffi::c_int as mz_uint16,
    284 as ::core::ffi::c_int as mz_uint16,
    284 as ::core::ffi::c_int as mz_uint16,
    284 as ::core::ffi::c_int as mz_uint16,
    284 as ::core::ffi::c_int as mz_uint16,
    284 as ::core::ffi::c_int as mz_uint16,
    284 as ::core::ffi::c_int as mz_uint16,
    284 as ::core::ffi::c_int as mz_uint16,
    284 as ::core::ffi::c_int as mz_uint16,
    284 as ::core::ffi::c_int as mz_uint16,
    284 as ::core::ffi::c_int as mz_uint16,
    284 as ::core::ffi::c_int as mz_uint16,
    284 as ::core::ffi::c_int as mz_uint16,
    284 as ::core::ffi::c_int as mz_uint16,
    284 as ::core::ffi::c_int as mz_uint16,
    284 as ::core::ffi::c_int as mz_uint16,
    284 as ::core::ffi::c_int as mz_uint16,
    284 as ::core::ffi::c_int as mz_uint16,
    284 as ::core::ffi::c_int as mz_uint16,
    284 as ::core::ffi::c_int as mz_uint16,
    284 as ::core::ffi::c_int as mz_uint16,
    284 as ::core::ffi::c_int as mz_uint16,
    284 as ::core::ffi::c_int as mz_uint16,
    284 as ::core::ffi::c_int as mz_uint16,
    284 as ::core::ffi::c_int as mz_uint16,
    285 as ::core::ffi::c_int as mz_uint16,
];
static mut s_tdefl_len_extra: [mz_uint8; 256] = [
    0 as ::core::ffi::c_int as mz_uint8,
    0 as ::core::ffi::c_int as mz_uint8,
    0 as ::core::ffi::c_int as mz_uint8,
    0 as ::core::ffi::c_int as mz_uint8,
    0 as ::core::ffi::c_int as mz_uint8,
    0 as ::core::ffi::c_int as mz_uint8,
    0 as ::core::ffi::c_int as mz_uint8,
    0 as ::core::ffi::c_int as mz_uint8,
    1 as ::core::ffi::c_int as mz_uint8,
    1 as ::core::ffi::c_int as mz_uint8,
    1 as ::core::ffi::c_int as mz_uint8,
    1 as ::core::ffi::c_int as mz_uint8,
    1 as ::core::ffi::c_int as mz_uint8,
    1 as ::core::ffi::c_int as mz_uint8,
    1 as ::core::ffi::c_int as mz_uint8,
    1 as ::core::ffi::c_int as mz_uint8,
    2 as ::core::ffi::c_int as mz_uint8,
    2 as ::core::ffi::c_int as mz_uint8,
    2 as ::core::ffi::c_int as mz_uint8,
    2 as ::core::ffi::c_int as mz_uint8,
    2 as ::core::ffi::c_int as mz_uint8,
    2 as ::core::ffi::c_int as mz_uint8,
    2 as ::core::ffi::c_int as mz_uint8,
    2 as ::core::ffi::c_int as mz_uint8,
    2 as ::core::ffi::c_int as mz_uint8,
    2 as ::core::ffi::c_int as mz_uint8,
    2 as ::core::ffi::c_int as mz_uint8,
    2 as ::core::ffi::c_int as mz_uint8,
    2 as ::core::ffi::c_int as mz_uint8,
    2 as ::core::ffi::c_int as mz_uint8,
    2 as ::core::ffi::c_int as mz_uint8,
    2 as ::core::ffi::c_int as mz_uint8,
    3 as ::core::ffi::c_int as mz_uint8,
    3 as ::core::ffi::c_int as mz_uint8,
    3 as ::core::ffi::c_int as mz_uint8,
    3 as ::core::ffi::c_int as mz_uint8,
    3 as ::core::ffi::c_int as mz_uint8,
    3 as ::core::ffi::c_int as mz_uint8,
    3 as ::core::ffi::c_int as mz_uint8,
    3 as ::core::ffi::c_int as mz_uint8,
    3 as ::core::ffi::c_int as mz_uint8,
    3 as ::core::ffi::c_int as mz_uint8,
    3 as ::core::ffi::c_int as mz_uint8,
    3 as ::core::ffi::c_int as mz_uint8,
    3 as ::core::ffi::c_int as mz_uint8,
    3 as ::core::ffi::c_int as mz_uint8,
    3 as ::core::ffi::c_int as mz_uint8,
    3 as ::core::ffi::c_int as mz_uint8,
    3 as ::core::ffi::c_int as mz_uint8,
    3 as ::core::ffi::c_int as mz_uint8,
    3 as ::core::ffi::c_int as mz_uint8,
    3 as ::core::ffi::c_int as mz_uint8,
    3 as ::core::ffi::c_int as mz_uint8,
    3 as ::core::ffi::c_int as mz_uint8,
    3 as ::core::ffi::c_int as mz_uint8,
    3 as ::core::ffi::c_int as mz_uint8,
    3 as ::core::ffi::c_int as mz_uint8,
    3 as ::core::ffi::c_int as mz_uint8,
    3 as ::core::ffi::c_int as mz_uint8,
    3 as ::core::ffi::c_int as mz_uint8,
    3 as ::core::ffi::c_int as mz_uint8,
    3 as ::core::ffi::c_int as mz_uint8,
    3 as ::core::ffi::c_int as mz_uint8,
    3 as ::core::ffi::c_int as mz_uint8,
    4 as ::core::ffi::c_int as mz_uint8,
    4 as ::core::ffi::c_int as mz_uint8,
    4 as ::core::ffi::c_int as mz_uint8,
    4 as ::core::ffi::c_int as mz_uint8,
    4 as ::core::ffi::c_int as mz_uint8,
    4 as ::core::ffi::c_int as mz_uint8,
    4 as ::core::ffi::c_int as mz_uint8,
    4 as ::core::ffi::c_int as mz_uint8,
    4 as ::core::ffi::c_int as mz_uint8,
    4 as ::core::ffi::c_int as mz_uint8,
    4 as ::core::ffi::c_int as mz_uint8,
    4 as ::core::ffi::c_int as mz_uint8,
    4 as ::core::ffi::c_int as mz_uint8,
    4 as ::core::ffi::c_int as mz_uint8,
    4 as ::core::ffi::c_int as mz_uint8,
    4 as ::core::ffi::c_int as mz_uint8,
    4 as ::core::ffi::c_int as mz_uint8,
    4 as ::core::ffi::c_int as mz_uint8,
    4 as ::core::ffi::c_int as mz_uint8,
    4 as ::core::ffi::c_int as mz_uint8,
    4 as ::core::ffi::c_int as mz_uint8,
    4 as ::core::ffi::c_int as mz_uint8,
    4 as ::core::ffi::c_int as mz_uint8,
    4 as ::core::ffi::c_int as mz_uint8,
    4 as ::core::ffi::c_int as mz_uint8,
    4 as ::core::ffi::c_int as mz_uint8,
    4 as ::core::ffi::c_int as mz_uint8,
    4 as ::core::ffi::c_int as mz_uint8,
    4 as ::core::ffi::c_int as mz_uint8,
    4 as ::core::ffi::c_int as mz_uint8,
    4 as ::core::ffi::c_int as mz_uint8,
    4 as ::core::ffi::c_int as mz_uint8,
    4 as ::core::ffi::c_int as mz_uint8,
    4 as ::core::ffi::c_int as mz_uint8,
    4 as ::core::ffi::c_int as mz_uint8,
    4 as ::core::ffi::c_int as mz_uint8,
    4 as ::core::ffi::c_int as mz_uint8,
    4 as ::core::ffi::c_int as mz_uint8,
    4 as ::core::ffi::c_int as mz_uint8,
    4 as ::core::ffi::c_int as mz_uint8,
    4 as ::core::ffi::c_int as mz_uint8,
    4 as ::core::ffi::c_int as mz_uint8,
    4 as ::core::ffi::c_int as mz_uint8,
    4 as ::core::ffi::c_int as mz_uint8,
    4 as ::core::ffi::c_int as mz_uint8,
    4 as ::core::ffi::c_int as mz_uint8,
    4 as ::core::ffi::c_int as mz_uint8,
    4 as ::core::ffi::c_int as mz_uint8,
    4 as ::core::ffi::c_int as mz_uint8,
    4 as ::core::ffi::c_int as mz_uint8,
    4 as ::core::ffi::c_int as mz_uint8,
    4 as ::core::ffi::c_int as mz_uint8,
    4 as ::core::ffi::c_int as mz_uint8,
    4 as ::core::ffi::c_int as mz_uint8,
    4 as ::core::ffi::c_int as mz_uint8,
    4 as ::core::ffi::c_int as mz_uint8,
    4 as ::core::ffi::c_int as mz_uint8,
    4 as ::core::ffi::c_int as mz_uint8,
    4 as ::core::ffi::c_int as mz_uint8,
    4 as ::core::ffi::c_int as mz_uint8,
    4 as ::core::ffi::c_int as mz_uint8,
    4 as ::core::ffi::c_int as mz_uint8,
    4 as ::core::ffi::c_int as mz_uint8,
    4 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    0 as ::core::ffi::c_int as mz_uint8,
];
static mut s_tdefl_small_dist_sym: [mz_uint8; 512] = [
    0 as ::core::ffi::c_int as mz_uint8,
    1 as ::core::ffi::c_int as mz_uint8,
    2 as ::core::ffi::c_int as mz_uint8,
    3 as ::core::ffi::c_int as mz_uint8,
    4 as ::core::ffi::c_int as mz_uint8,
    4 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    6 as ::core::ffi::c_int as mz_uint8,
    6 as ::core::ffi::c_int as mz_uint8,
    6 as ::core::ffi::c_int as mz_uint8,
    6 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    8 as ::core::ffi::c_int as mz_uint8,
    8 as ::core::ffi::c_int as mz_uint8,
    8 as ::core::ffi::c_int as mz_uint8,
    8 as ::core::ffi::c_int as mz_uint8,
    8 as ::core::ffi::c_int as mz_uint8,
    8 as ::core::ffi::c_int as mz_uint8,
    8 as ::core::ffi::c_int as mz_uint8,
    8 as ::core::ffi::c_int as mz_uint8,
    9 as ::core::ffi::c_int as mz_uint8,
    9 as ::core::ffi::c_int as mz_uint8,
    9 as ::core::ffi::c_int as mz_uint8,
    9 as ::core::ffi::c_int as mz_uint8,
    9 as ::core::ffi::c_int as mz_uint8,
    9 as ::core::ffi::c_int as mz_uint8,
    9 as ::core::ffi::c_int as mz_uint8,
    9 as ::core::ffi::c_int as mz_uint8,
    10 as ::core::ffi::c_int as mz_uint8,
    10 as ::core::ffi::c_int as mz_uint8,
    10 as ::core::ffi::c_int as mz_uint8,
    10 as ::core::ffi::c_int as mz_uint8,
    10 as ::core::ffi::c_int as mz_uint8,
    10 as ::core::ffi::c_int as mz_uint8,
    10 as ::core::ffi::c_int as mz_uint8,
    10 as ::core::ffi::c_int as mz_uint8,
    10 as ::core::ffi::c_int as mz_uint8,
    10 as ::core::ffi::c_int as mz_uint8,
    10 as ::core::ffi::c_int as mz_uint8,
    10 as ::core::ffi::c_int as mz_uint8,
    10 as ::core::ffi::c_int as mz_uint8,
    10 as ::core::ffi::c_int as mz_uint8,
    10 as ::core::ffi::c_int as mz_uint8,
    10 as ::core::ffi::c_int as mz_uint8,
    11 as ::core::ffi::c_int as mz_uint8,
    11 as ::core::ffi::c_int as mz_uint8,
    11 as ::core::ffi::c_int as mz_uint8,
    11 as ::core::ffi::c_int as mz_uint8,
    11 as ::core::ffi::c_int as mz_uint8,
    11 as ::core::ffi::c_int as mz_uint8,
    11 as ::core::ffi::c_int as mz_uint8,
    11 as ::core::ffi::c_int as mz_uint8,
    11 as ::core::ffi::c_int as mz_uint8,
    11 as ::core::ffi::c_int as mz_uint8,
    11 as ::core::ffi::c_int as mz_uint8,
    11 as ::core::ffi::c_int as mz_uint8,
    11 as ::core::ffi::c_int as mz_uint8,
    11 as ::core::ffi::c_int as mz_uint8,
    11 as ::core::ffi::c_int as mz_uint8,
    11 as ::core::ffi::c_int as mz_uint8,
    12 as ::core::ffi::c_int as mz_uint8,
    12 as ::core::ffi::c_int as mz_uint8,
    12 as ::core::ffi::c_int as mz_uint8,
    12 as ::core::ffi::c_int as mz_uint8,
    12 as ::core::ffi::c_int as mz_uint8,
    12 as ::core::ffi::c_int as mz_uint8,
    12 as ::core::ffi::c_int as mz_uint8,
    12 as ::core::ffi::c_int as mz_uint8,
    12 as ::core::ffi::c_int as mz_uint8,
    12 as ::core::ffi::c_int as mz_uint8,
    12 as ::core::ffi::c_int as mz_uint8,
    12 as ::core::ffi::c_int as mz_uint8,
    12 as ::core::ffi::c_int as mz_uint8,
    12 as ::core::ffi::c_int as mz_uint8,
    12 as ::core::ffi::c_int as mz_uint8,
    12 as ::core::ffi::c_int as mz_uint8,
    12 as ::core::ffi::c_int as mz_uint8,
    12 as ::core::ffi::c_int as mz_uint8,
    12 as ::core::ffi::c_int as mz_uint8,
    12 as ::core::ffi::c_int as mz_uint8,
    12 as ::core::ffi::c_int as mz_uint8,
    12 as ::core::ffi::c_int as mz_uint8,
    12 as ::core::ffi::c_int as mz_uint8,
    12 as ::core::ffi::c_int as mz_uint8,
    12 as ::core::ffi::c_int as mz_uint8,
    12 as ::core::ffi::c_int as mz_uint8,
    12 as ::core::ffi::c_int as mz_uint8,
    12 as ::core::ffi::c_int as mz_uint8,
    12 as ::core::ffi::c_int as mz_uint8,
    12 as ::core::ffi::c_int as mz_uint8,
    12 as ::core::ffi::c_int as mz_uint8,
    12 as ::core::ffi::c_int as mz_uint8,
    13 as ::core::ffi::c_int as mz_uint8,
    13 as ::core::ffi::c_int as mz_uint8,
    13 as ::core::ffi::c_int as mz_uint8,
    13 as ::core::ffi::c_int as mz_uint8,
    13 as ::core::ffi::c_int as mz_uint8,
    13 as ::core::ffi::c_int as mz_uint8,
    13 as ::core::ffi::c_int as mz_uint8,
    13 as ::core::ffi::c_int as mz_uint8,
    13 as ::core::ffi::c_int as mz_uint8,
    13 as ::core::ffi::c_int as mz_uint8,
    13 as ::core::ffi::c_int as mz_uint8,
    13 as ::core::ffi::c_int as mz_uint8,
    13 as ::core::ffi::c_int as mz_uint8,
    13 as ::core::ffi::c_int as mz_uint8,
    13 as ::core::ffi::c_int as mz_uint8,
    13 as ::core::ffi::c_int as mz_uint8,
    13 as ::core::ffi::c_int as mz_uint8,
    13 as ::core::ffi::c_int as mz_uint8,
    13 as ::core::ffi::c_int as mz_uint8,
    13 as ::core::ffi::c_int as mz_uint8,
    13 as ::core::ffi::c_int as mz_uint8,
    13 as ::core::ffi::c_int as mz_uint8,
    13 as ::core::ffi::c_int as mz_uint8,
    13 as ::core::ffi::c_int as mz_uint8,
    13 as ::core::ffi::c_int as mz_uint8,
    13 as ::core::ffi::c_int as mz_uint8,
    13 as ::core::ffi::c_int as mz_uint8,
    13 as ::core::ffi::c_int as mz_uint8,
    13 as ::core::ffi::c_int as mz_uint8,
    13 as ::core::ffi::c_int as mz_uint8,
    13 as ::core::ffi::c_int as mz_uint8,
    13 as ::core::ffi::c_int as mz_uint8,
    14 as ::core::ffi::c_int as mz_uint8,
    14 as ::core::ffi::c_int as mz_uint8,
    14 as ::core::ffi::c_int as mz_uint8,
    14 as ::core::ffi::c_int as mz_uint8,
    14 as ::core::ffi::c_int as mz_uint8,
    14 as ::core::ffi::c_int as mz_uint8,
    14 as ::core::ffi::c_int as mz_uint8,
    14 as ::core::ffi::c_int as mz_uint8,
    14 as ::core::ffi::c_int as mz_uint8,
    14 as ::core::ffi::c_int as mz_uint8,
    14 as ::core::ffi::c_int as mz_uint8,
    14 as ::core::ffi::c_int as mz_uint8,
    14 as ::core::ffi::c_int as mz_uint8,
    14 as ::core::ffi::c_int as mz_uint8,
    14 as ::core::ffi::c_int as mz_uint8,
    14 as ::core::ffi::c_int as mz_uint8,
    14 as ::core::ffi::c_int as mz_uint8,
    14 as ::core::ffi::c_int as mz_uint8,
    14 as ::core::ffi::c_int as mz_uint8,
    14 as ::core::ffi::c_int as mz_uint8,
    14 as ::core::ffi::c_int as mz_uint8,
    14 as ::core::ffi::c_int as mz_uint8,
    14 as ::core::ffi::c_int as mz_uint8,
    14 as ::core::ffi::c_int as mz_uint8,
    14 as ::core::ffi::c_int as mz_uint8,
    14 as ::core::ffi::c_int as mz_uint8,
    14 as ::core::ffi::c_int as mz_uint8,
    14 as ::core::ffi::c_int as mz_uint8,
    14 as ::core::ffi::c_int as mz_uint8,
    14 as ::core::ffi::c_int as mz_uint8,
    14 as ::core::ffi::c_int as mz_uint8,
    14 as ::core::ffi::c_int as mz_uint8,
    14 as ::core::ffi::c_int as mz_uint8,
    14 as ::core::ffi::c_int as mz_uint8,
    14 as ::core::ffi::c_int as mz_uint8,
    14 as ::core::ffi::c_int as mz_uint8,
    14 as ::core::ffi::c_int as mz_uint8,
    14 as ::core::ffi::c_int as mz_uint8,
    14 as ::core::ffi::c_int as mz_uint8,
    14 as ::core::ffi::c_int as mz_uint8,
    14 as ::core::ffi::c_int as mz_uint8,
    14 as ::core::ffi::c_int as mz_uint8,
    14 as ::core::ffi::c_int as mz_uint8,
    14 as ::core::ffi::c_int as mz_uint8,
    14 as ::core::ffi::c_int as mz_uint8,
    14 as ::core::ffi::c_int as mz_uint8,
    14 as ::core::ffi::c_int as mz_uint8,
    14 as ::core::ffi::c_int as mz_uint8,
    14 as ::core::ffi::c_int as mz_uint8,
    14 as ::core::ffi::c_int as mz_uint8,
    14 as ::core::ffi::c_int as mz_uint8,
    14 as ::core::ffi::c_int as mz_uint8,
    14 as ::core::ffi::c_int as mz_uint8,
    14 as ::core::ffi::c_int as mz_uint8,
    14 as ::core::ffi::c_int as mz_uint8,
    14 as ::core::ffi::c_int as mz_uint8,
    14 as ::core::ffi::c_int as mz_uint8,
    14 as ::core::ffi::c_int as mz_uint8,
    14 as ::core::ffi::c_int as mz_uint8,
    14 as ::core::ffi::c_int as mz_uint8,
    14 as ::core::ffi::c_int as mz_uint8,
    14 as ::core::ffi::c_int as mz_uint8,
    14 as ::core::ffi::c_int as mz_uint8,
    14 as ::core::ffi::c_int as mz_uint8,
    15 as ::core::ffi::c_int as mz_uint8,
    15 as ::core::ffi::c_int as mz_uint8,
    15 as ::core::ffi::c_int as mz_uint8,
    15 as ::core::ffi::c_int as mz_uint8,
    15 as ::core::ffi::c_int as mz_uint8,
    15 as ::core::ffi::c_int as mz_uint8,
    15 as ::core::ffi::c_int as mz_uint8,
    15 as ::core::ffi::c_int as mz_uint8,
    15 as ::core::ffi::c_int as mz_uint8,
    15 as ::core::ffi::c_int as mz_uint8,
    15 as ::core::ffi::c_int as mz_uint8,
    15 as ::core::ffi::c_int as mz_uint8,
    15 as ::core::ffi::c_int as mz_uint8,
    15 as ::core::ffi::c_int as mz_uint8,
    15 as ::core::ffi::c_int as mz_uint8,
    15 as ::core::ffi::c_int as mz_uint8,
    15 as ::core::ffi::c_int as mz_uint8,
    15 as ::core::ffi::c_int as mz_uint8,
    15 as ::core::ffi::c_int as mz_uint8,
    15 as ::core::ffi::c_int as mz_uint8,
    15 as ::core::ffi::c_int as mz_uint8,
    15 as ::core::ffi::c_int as mz_uint8,
    15 as ::core::ffi::c_int as mz_uint8,
    15 as ::core::ffi::c_int as mz_uint8,
    15 as ::core::ffi::c_int as mz_uint8,
    15 as ::core::ffi::c_int as mz_uint8,
    15 as ::core::ffi::c_int as mz_uint8,
    15 as ::core::ffi::c_int as mz_uint8,
    15 as ::core::ffi::c_int as mz_uint8,
    15 as ::core::ffi::c_int as mz_uint8,
    15 as ::core::ffi::c_int as mz_uint8,
    15 as ::core::ffi::c_int as mz_uint8,
    15 as ::core::ffi::c_int as mz_uint8,
    15 as ::core::ffi::c_int as mz_uint8,
    15 as ::core::ffi::c_int as mz_uint8,
    15 as ::core::ffi::c_int as mz_uint8,
    15 as ::core::ffi::c_int as mz_uint8,
    15 as ::core::ffi::c_int as mz_uint8,
    15 as ::core::ffi::c_int as mz_uint8,
    15 as ::core::ffi::c_int as mz_uint8,
    15 as ::core::ffi::c_int as mz_uint8,
    15 as ::core::ffi::c_int as mz_uint8,
    15 as ::core::ffi::c_int as mz_uint8,
    15 as ::core::ffi::c_int as mz_uint8,
    15 as ::core::ffi::c_int as mz_uint8,
    15 as ::core::ffi::c_int as mz_uint8,
    15 as ::core::ffi::c_int as mz_uint8,
    15 as ::core::ffi::c_int as mz_uint8,
    15 as ::core::ffi::c_int as mz_uint8,
    15 as ::core::ffi::c_int as mz_uint8,
    15 as ::core::ffi::c_int as mz_uint8,
    15 as ::core::ffi::c_int as mz_uint8,
    15 as ::core::ffi::c_int as mz_uint8,
    15 as ::core::ffi::c_int as mz_uint8,
    15 as ::core::ffi::c_int as mz_uint8,
    15 as ::core::ffi::c_int as mz_uint8,
    15 as ::core::ffi::c_int as mz_uint8,
    15 as ::core::ffi::c_int as mz_uint8,
    15 as ::core::ffi::c_int as mz_uint8,
    15 as ::core::ffi::c_int as mz_uint8,
    15 as ::core::ffi::c_int as mz_uint8,
    15 as ::core::ffi::c_int as mz_uint8,
    15 as ::core::ffi::c_int as mz_uint8,
    15 as ::core::ffi::c_int as mz_uint8,
    16 as ::core::ffi::c_int as mz_uint8,
    16 as ::core::ffi::c_int as mz_uint8,
    16 as ::core::ffi::c_int as mz_uint8,
    16 as ::core::ffi::c_int as mz_uint8,
    16 as ::core::ffi::c_int as mz_uint8,
    16 as ::core::ffi::c_int as mz_uint8,
    16 as ::core::ffi::c_int as mz_uint8,
    16 as ::core::ffi::c_int as mz_uint8,
    16 as ::core::ffi::c_int as mz_uint8,
    16 as ::core::ffi::c_int as mz_uint8,
    16 as ::core::ffi::c_int as mz_uint8,
    16 as ::core::ffi::c_int as mz_uint8,
    16 as ::core::ffi::c_int as mz_uint8,
    16 as ::core::ffi::c_int as mz_uint8,
    16 as ::core::ffi::c_int as mz_uint8,
    16 as ::core::ffi::c_int as mz_uint8,
    16 as ::core::ffi::c_int as mz_uint8,
    16 as ::core::ffi::c_int as mz_uint8,
    16 as ::core::ffi::c_int as mz_uint8,
    16 as ::core::ffi::c_int as mz_uint8,
    16 as ::core::ffi::c_int as mz_uint8,
    16 as ::core::ffi::c_int as mz_uint8,
    16 as ::core::ffi::c_int as mz_uint8,
    16 as ::core::ffi::c_int as mz_uint8,
    16 as ::core::ffi::c_int as mz_uint8,
    16 as ::core::ffi::c_int as mz_uint8,
    16 as ::core::ffi::c_int as mz_uint8,
    16 as ::core::ffi::c_int as mz_uint8,
    16 as ::core::ffi::c_int as mz_uint8,
    16 as ::core::ffi::c_int as mz_uint8,
    16 as ::core::ffi::c_int as mz_uint8,
    16 as ::core::ffi::c_int as mz_uint8,
    16 as ::core::ffi::c_int as mz_uint8,
    16 as ::core::ffi::c_int as mz_uint8,
    16 as ::core::ffi::c_int as mz_uint8,
    16 as ::core::ffi::c_int as mz_uint8,
    16 as ::core::ffi::c_int as mz_uint8,
    16 as ::core::ffi::c_int as mz_uint8,
    16 as ::core::ffi::c_int as mz_uint8,
    16 as ::core::ffi::c_int as mz_uint8,
    16 as ::core::ffi::c_int as mz_uint8,
    16 as ::core::ffi::c_int as mz_uint8,
    16 as ::core::ffi::c_int as mz_uint8,
    16 as ::core::ffi::c_int as mz_uint8,
    16 as ::core::ffi::c_int as mz_uint8,
    16 as ::core::ffi::c_int as mz_uint8,
    16 as ::core::ffi::c_int as mz_uint8,
    16 as ::core::ffi::c_int as mz_uint8,
    16 as ::core::ffi::c_int as mz_uint8,
    16 as ::core::ffi::c_int as mz_uint8,
    16 as ::core::ffi::c_int as mz_uint8,
    16 as ::core::ffi::c_int as mz_uint8,
    16 as ::core::ffi::c_int as mz_uint8,
    16 as ::core::ffi::c_int as mz_uint8,
    16 as ::core::ffi::c_int as mz_uint8,
    16 as ::core::ffi::c_int as mz_uint8,
    16 as ::core::ffi::c_int as mz_uint8,
    16 as ::core::ffi::c_int as mz_uint8,
    16 as ::core::ffi::c_int as mz_uint8,
    16 as ::core::ffi::c_int as mz_uint8,
    16 as ::core::ffi::c_int as mz_uint8,
    16 as ::core::ffi::c_int as mz_uint8,
    16 as ::core::ffi::c_int as mz_uint8,
    16 as ::core::ffi::c_int as mz_uint8,
    16 as ::core::ffi::c_int as mz_uint8,
    16 as ::core::ffi::c_int as mz_uint8,
    16 as ::core::ffi::c_int as mz_uint8,
    16 as ::core::ffi::c_int as mz_uint8,
    16 as ::core::ffi::c_int as mz_uint8,
    16 as ::core::ffi::c_int as mz_uint8,
    16 as ::core::ffi::c_int as mz_uint8,
    16 as ::core::ffi::c_int as mz_uint8,
    16 as ::core::ffi::c_int as mz_uint8,
    16 as ::core::ffi::c_int as mz_uint8,
    16 as ::core::ffi::c_int as mz_uint8,
    16 as ::core::ffi::c_int as mz_uint8,
    16 as ::core::ffi::c_int as mz_uint8,
    16 as ::core::ffi::c_int as mz_uint8,
    16 as ::core::ffi::c_int as mz_uint8,
    16 as ::core::ffi::c_int as mz_uint8,
    16 as ::core::ffi::c_int as mz_uint8,
    16 as ::core::ffi::c_int as mz_uint8,
    16 as ::core::ffi::c_int as mz_uint8,
    16 as ::core::ffi::c_int as mz_uint8,
    16 as ::core::ffi::c_int as mz_uint8,
    16 as ::core::ffi::c_int as mz_uint8,
    16 as ::core::ffi::c_int as mz_uint8,
    16 as ::core::ffi::c_int as mz_uint8,
    16 as ::core::ffi::c_int as mz_uint8,
    16 as ::core::ffi::c_int as mz_uint8,
    16 as ::core::ffi::c_int as mz_uint8,
    16 as ::core::ffi::c_int as mz_uint8,
    16 as ::core::ffi::c_int as mz_uint8,
    16 as ::core::ffi::c_int as mz_uint8,
    16 as ::core::ffi::c_int as mz_uint8,
    16 as ::core::ffi::c_int as mz_uint8,
    16 as ::core::ffi::c_int as mz_uint8,
    16 as ::core::ffi::c_int as mz_uint8,
    16 as ::core::ffi::c_int as mz_uint8,
    16 as ::core::ffi::c_int as mz_uint8,
    16 as ::core::ffi::c_int as mz_uint8,
    16 as ::core::ffi::c_int as mz_uint8,
    16 as ::core::ffi::c_int as mz_uint8,
    16 as ::core::ffi::c_int as mz_uint8,
    16 as ::core::ffi::c_int as mz_uint8,
    16 as ::core::ffi::c_int as mz_uint8,
    16 as ::core::ffi::c_int as mz_uint8,
    16 as ::core::ffi::c_int as mz_uint8,
    16 as ::core::ffi::c_int as mz_uint8,
    16 as ::core::ffi::c_int as mz_uint8,
    16 as ::core::ffi::c_int as mz_uint8,
    16 as ::core::ffi::c_int as mz_uint8,
    16 as ::core::ffi::c_int as mz_uint8,
    16 as ::core::ffi::c_int as mz_uint8,
    16 as ::core::ffi::c_int as mz_uint8,
    16 as ::core::ffi::c_int as mz_uint8,
    16 as ::core::ffi::c_int as mz_uint8,
    16 as ::core::ffi::c_int as mz_uint8,
    16 as ::core::ffi::c_int as mz_uint8,
    16 as ::core::ffi::c_int as mz_uint8,
    16 as ::core::ffi::c_int as mz_uint8,
    16 as ::core::ffi::c_int as mz_uint8,
    16 as ::core::ffi::c_int as mz_uint8,
    16 as ::core::ffi::c_int as mz_uint8,
    16 as ::core::ffi::c_int as mz_uint8,
    16 as ::core::ffi::c_int as mz_uint8,
    16 as ::core::ffi::c_int as mz_uint8,
    16 as ::core::ffi::c_int as mz_uint8,
    17 as ::core::ffi::c_int as mz_uint8,
    17 as ::core::ffi::c_int as mz_uint8,
    17 as ::core::ffi::c_int as mz_uint8,
    17 as ::core::ffi::c_int as mz_uint8,
    17 as ::core::ffi::c_int as mz_uint8,
    17 as ::core::ffi::c_int as mz_uint8,
    17 as ::core::ffi::c_int as mz_uint8,
    17 as ::core::ffi::c_int as mz_uint8,
    17 as ::core::ffi::c_int as mz_uint8,
    17 as ::core::ffi::c_int as mz_uint8,
    17 as ::core::ffi::c_int as mz_uint8,
    17 as ::core::ffi::c_int as mz_uint8,
    17 as ::core::ffi::c_int as mz_uint8,
    17 as ::core::ffi::c_int as mz_uint8,
    17 as ::core::ffi::c_int as mz_uint8,
    17 as ::core::ffi::c_int as mz_uint8,
    17 as ::core::ffi::c_int as mz_uint8,
    17 as ::core::ffi::c_int as mz_uint8,
    17 as ::core::ffi::c_int as mz_uint8,
    17 as ::core::ffi::c_int as mz_uint8,
    17 as ::core::ffi::c_int as mz_uint8,
    17 as ::core::ffi::c_int as mz_uint8,
    17 as ::core::ffi::c_int as mz_uint8,
    17 as ::core::ffi::c_int as mz_uint8,
    17 as ::core::ffi::c_int as mz_uint8,
    17 as ::core::ffi::c_int as mz_uint8,
    17 as ::core::ffi::c_int as mz_uint8,
    17 as ::core::ffi::c_int as mz_uint8,
    17 as ::core::ffi::c_int as mz_uint8,
    17 as ::core::ffi::c_int as mz_uint8,
    17 as ::core::ffi::c_int as mz_uint8,
    17 as ::core::ffi::c_int as mz_uint8,
    17 as ::core::ffi::c_int as mz_uint8,
    17 as ::core::ffi::c_int as mz_uint8,
    17 as ::core::ffi::c_int as mz_uint8,
    17 as ::core::ffi::c_int as mz_uint8,
    17 as ::core::ffi::c_int as mz_uint8,
    17 as ::core::ffi::c_int as mz_uint8,
    17 as ::core::ffi::c_int as mz_uint8,
    17 as ::core::ffi::c_int as mz_uint8,
    17 as ::core::ffi::c_int as mz_uint8,
    17 as ::core::ffi::c_int as mz_uint8,
    17 as ::core::ffi::c_int as mz_uint8,
    17 as ::core::ffi::c_int as mz_uint8,
    17 as ::core::ffi::c_int as mz_uint8,
    17 as ::core::ffi::c_int as mz_uint8,
    17 as ::core::ffi::c_int as mz_uint8,
    17 as ::core::ffi::c_int as mz_uint8,
    17 as ::core::ffi::c_int as mz_uint8,
    17 as ::core::ffi::c_int as mz_uint8,
    17 as ::core::ffi::c_int as mz_uint8,
    17 as ::core::ffi::c_int as mz_uint8,
    17 as ::core::ffi::c_int as mz_uint8,
    17 as ::core::ffi::c_int as mz_uint8,
    17 as ::core::ffi::c_int as mz_uint8,
    17 as ::core::ffi::c_int as mz_uint8,
    17 as ::core::ffi::c_int as mz_uint8,
    17 as ::core::ffi::c_int as mz_uint8,
    17 as ::core::ffi::c_int as mz_uint8,
    17 as ::core::ffi::c_int as mz_uint8,
    17 as ::core::ffi::c_int as mz_uint8,
    17 as ::core::ffi::c_int as mz_uint8,
    17 as ::core::ffi::c_int as mz_uint8,
    17 as ::core::ffi::c_int as mz_uint8,
    17 as ::core::ffi::c_int as mz_uint8,
    17 as ::core::ffi::c_int as mz_uint8,
    17 as ::core::ffi::c_int as mz_uint8,
    17 as ::core::ffi::c_int as mz_uint8,
    17 as ::core::ffi::c_int as mz_uint8,
    17 as ::core::ffi::c_int as mz_uint8,
    17 as ::core::ffi::c_int as mz_uint8,
    17 as ::core::ffi::c_int as mz_uint8,
    17 as ::core::ffi::c_int as mz_uint8,
    17 as ::core::ffi::c_int as mz_uint8,
    17 as ::core::ffi::c_int as mz_uint8,
    17 as ::core::ffi::c_int as mz_uint8,
    17 as ::core::ffi::c_int as mz_uint8,
    17 as ::core::ffi::c_int as mz_uint8,
    17 as ::core::ffi::c_int as mz_uint8,
    17 as ::core::ffi::c_int as mz_uint8,
    17 as ::core::ffi::c_int as mz_uint8,
    17 as ::core::ffi::c_int as mz_uint8,
    17 as ::core::ffi::c_int as mz_uint8,
    17 as ::core::ffi::c_int as mz_uint8,
    17 as ::core::ffi::c_int as mz_uint8,
    17 as ::core::ffi::c_int as mz_uint8,
    17 as ::core::ffi::c_int as mz_uint8,
    17 as ::core::ffi::c_int as mz_uint8,
    17 as ::core::ffi::c_int as mz_uint8,
    17 as ::core::ffi::c_int as mz_uint8,
    17 as ::core::ffi::c_int as mz_uint8,
    17 as ::core::ffi::c_int as mz_uint8,
    17 as ::core::ffi::c_int as mz_uint8,
    17 as ::core::ffi::c_int as mz_uint8,
    17 as ::core::ffi::c_int as mz_uint8,
    17 as ::core::ffi::c_int as mz_uint8,
    17 as ::core::ffi::c_int as mz_uint8,
    17 as ::core::ffi::c_int as mz_uint8,
    17 as ::core::ffi::c_int as mz_uint8,
    17 as ::core::ffi::c_int as mz_uint8,
    17 as ::core::ffi::c_int as mz_uint8,
    17 as ::core::ffi::c_int as mz_uint8,
    17 as ::core::ffi::c_int as mz_uint8,
    17 as ::core::ffi::c_int as mz_uint8,
    17 as ::core::ffi::c_int as mz_uint8,
    17 as ::core::ffi::c_int as mz_uint8,
    17 as ::core::ffi::c_int as mz_uint8,
    17 as ::core::ffi::c_int as mz_uint8,
    17 as ::core::ffi::c_int as mz_uint8,
    17 as ::core::ffi::c_int as mz_uint8,
    17 as ::core::ffi::c_int as mz_uint8,
    17 as ::core::ffi::c_int as mz_uint8,
    17 as ::core::ffi::c_int as mz_uint8,
    17 as ::core::ffi::c_int as mz_uint8,
    17 as ::core::ffi::c_int as mz_uint8,
    17 as ::core::ffi::c_int as mz_uint8,
    17 as ::core::ffi::c_int as mz_uint8,
    17 as ::core::ffi::c_int as mz_uint8,
    17 as ::core::ffi::c_int as mz_uint8,
    17 as ::core::ffi::c_int as mz_uint8,
    17 as ::core::ffi::c_int as mz_uint8,
    17 as ::core::ffi::c_int as mz_uint8,
    17 as ::core::ffi::c_int as mz_uint8,
    17 as ::core::ffi::c_int as mz_uint8,
    17 as ::core::ffi::c_int as mz_uint8,
    17 as ::core::ffi::c_int as mz_uint8,
    17 as ::core::ffi::c_int as mz_uint8,
    17 as ::core::ffi::c_int as mz_uint8,
];
static mut s_tdefl_small_dist_extra: [mz_uint8; 512] = [
    0 as ::core::ffi::c_int as mz_uint8,
    0 as ::core::ffi::c_int as mz_uint8,
    0 as ::core::ffi::c_int as mz_uint8,
    0 as ::core::ffi::c_int as mz_uint8,
    1 as ::core::ffi::c_int as mz_uint8,
    1 as ::core::ffi::c_int as mz_uint8,
    1 as ::core::ffi::c_int as mz_uint8,
    1 as ::core::ffi::c_int as mz_uint8,
    2 as ::core::ffi::c_int as mz_uint8,
    2 as ::core::ffi::c_int as mz_uint8,
    2 as ::core::ffi::c_int as mz_uint8,
    2 as ::core::ffi::c_int as mz_uint8,
    2 as ::core::ffi::c_int as mz_uint8,
    2 as ::core::ffi::c_int as mz_uint8,
    2 as ::core::ffi::c_int as mz_uint8,
    2 as ::core::ffi::c_int as mz_uint8,
    3 as ::core::ffi::c_int as mz_uint8,
    3 as ::core::ffi::c_int as mz_uint8,
    3 as ::core::ffi::c_int as mz_uint8,
    3 as ::core::ffi::c_int as mz_uint8,
    3 as ::core::ffi::c_int as mz_uint8,
    3 as ::core::ffi::c_int as mz_uint8,
    3 as ::core::ffi::c_int as mz_uint8,
    3 as ::core::ffi::c_int as mz_uint8,
    3 as ::core::ffi::c_int as mz_uint8,
    3 as ::core::ffi::c_int as mz_uint8,
    3 as ::core::ffi::c_int as mz_uint8,
    3 as ::core::ffi::c_int as mz_uint8,
    3 as ::core::ffi::c_int as mz_uint8,
    3 as ::core::ffi::c_int as mz_uint8,
    3 as ::core::ffi::c_int as mz_uint8,
    3 as ::core::ffi::c_int as mz_uint8,
    4 as ::core::ffi::c_int as mz_uint8,
    4 as ::core::ffi::c_int as mz_uint8,
    4 as ::core::ffi::c_int as mz_uint8,
    4 as ::core::ffi::c_int as mz_uint8,
    4 as ::core::ffi::c_int as mz_uint8,
    4 as ::core::ffi::c_int as mz_uint8,
    4 as ::core::ffi::c_int as mz_uint8,
    4 as ::core::ffi::c_int as mz_uint8,
    4 as ::core::ffi::c_int as mz_uint8,
    4 as ::core::ffi::c_int as mz_uint8,
    4 as ::core::ffi::c_int as mz_uint8,
    4 as ::core::ffi::c_int as mz_uint8,
    4 as ::core::ffi::c_int as mz_uint8,
    4 as ::core::ffi::c_int as mz_uint8,
    4 as ::core::ffi::c_int as mz_uint8,
    4 as ::core::ffi::c_int as mz_uint8,
    4 as ::core::ffi::c_int as mz_uint8,
    4 as ::core::ffi::c_int as mz_uint8,
    4 as ::core::ffi::c_int as mz_uint8,
    4 as ::core::ffi::c_int as mz_uint8,
    4 as ::core::ffi::c_int as mz_uint8,
    4 as ::core::ffi::c_int as mz_uint8,
    4 as ::core::ffi::c_int as mz_uint8,
    4 as ::core::ffi::c_int as mz_uint8,
    4 as ::core::ffi::c_int as mz_uint8,
    4 as ::core::ffi::c_int as mz_uint8,
    4 as ::core::ffi::c_int as mz_uint8,
    4 as ::core::ffi::c_int as mz_uint8,
    4 as ::core::ffi::c_int as mz_uint8,
    4 as ::core::ffi::c_int as mz_uint8,
    4 as ::core::ffi::c_int as mz_uint8,
    4 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    6 as ::core::ffi::c_int as mz_uint8,
    6 as ::core::ffi::c_int as mz_uint8,
    6 as ::core::ffi::c_int as mz_uint8,
    6 as ::core::ffi::c_int as mz_uint8,
    6 as ::core::ffi::c_int as mz_uint8,
    6 as ::core::ffi::c_int as mz_uint8,
    6 as ::core::ffi::c_int as mz_uint8,
    6 as ::core::ffi::c_int as mz_uint8,
    6 as ::core::ffi::c_int as mz_uint8,
    6 as ::core::ffi::c_int as mz_uint8,
    6 as ::core::ffi::c_int as mz_uint8,
    6 as ::core::ffi::c_int as mz_uint8,
    6 as ::core::ffi::c_int as mz_uint8,
    6 as ::core::ffi::c_int as mz_uint8,
    6 as ::core::ffi::c_int as mz_uint8,
    6 as ::core::ffi::c_int as mz_uint8,
    6 as ::core::ffi::c_int as mz_uint8,
    6 as ::core::ffi::c_int as mz_uint8,
    6 as ::core::ffi::c_int as mz_uint8,
    6 as ::core::ffi::c_int as mz_uint8,
    6 as ::core::ffi::c_int as mz_uint8,
    6 as ::core::ffi::c_int as mz_uint8,
    6 as ::core::ffi::c_int as mz_uint8,
    6 as ::core::ffi::c_int as mz_uint8,
    6 as ::core::ffi::c_int as mz_uint8,
    6 as ::core::ffi::c_int as mz_uint8,
    6 as ::core::ffi::c_int as mz_uint8,
    6 as ::core::ffi::c_int as mz_uint8,
    6 as ::core::ffi::c_int as mz_uint8,
    6 as ::core::ffi::c_int as mz_uint8,
    6 as ::core::ffi::c_int as mz_uint8,
    6 as ::core::ffi::c_int as mz_uint8,
    6 as ::core::ffi::c_int as mz_uint8,
    6 as ::core::ffi::c_int as mz_uint8,
    6 as ::core::ffi::c_int as mz_uint8,
    6 as ::core::ffi::c_int as mz_uint8,
    6 as ::core::ffi::c_int as mz_uint8,
    6 as ::core::ffi::c_int as mz_uint8,
    6 as ::core::ffi::c_int as mz_uint8,
    6 as ::core::ffi::c_int as mz_uint8,
    6 as ::core::ffi::c_int as mz_uint8,
    6 as ::core::ffi::c_int as mz_uint8,
    6 as ::core::ffi::c_int as mz_uint8,
    6 as ::core::ffi::c_int as mz_uint8,
    6 as ::core::ffi::c_int as mz_uint8,
    6 as ::core::ffi::c_int as mz_uint8,
    6 as ::core::ffi::c_int as mz_uint8,
    6 as ::core::ffi::c_int as mz_uint8,
    6 as ::core::ffi::c_int as mz_uint8,
    6 as ::core::ffi::c_int as mz_uint8,
    6 as ::core::ffi::c_int as mz_uint8,
    6 as ::core::ffi::c_int as mz_uint8,
    6 as ::core::ffi::c_int as mz_uint8,
    6 as ::core::ffi::c_int as mz_uint8,
    6 as ::core::ffi::c_int as mz_uint8,
    6 as ::core::ffi::c_int as mz_uint8,
    6 as ::core::ffi::c_int as mz_uint8,
    6 as ::core::ffi::c_int as mz_uint8,
    6 as ::core::ffi::c_int as mz_uint8,
    6 as ::core::ffi::c_int as mz_uint8,
    6 as ::core::ffi::c_int as mz_uint8,
    6 as ::core::ffi::c_int as mz_uint8,
    6 as ::core::ffi::c_int as mz_uint8,
    6 as ::core::ffi::c_int as mz_uint8,
    6 as ::core::ffi::c_int as mz_uint8,
    6 as ::core::ffi::c_int as mz_uint8,
    6 as ::core::ffi::c_int as mz_uint8,
    6 as ::core::ffi::c_int as mz_uint8,
    6 as ::core::ffi::c_int as mz_uint8,
    6 as ::core::ffi::c_int as mz_uint8,
    6 as ::core::ffi::c_int as mz_uint8,
    6 as ::core::ffi::c_int as mz_uint8,
    6 as ::core::ffi::c_int as mz_uint8,
    6 as ::core::ffi::c_int as mz_uint8,
    6 as ::core::ffi::c_int as mz_uint8,
    6 as ::core::ffi::c_int as mz_uint8,
    6 as ::core::ffi::c_int as mz_uint8,
    6 as ::core::ffi::c_int as mz_uint8,
    6 as ::core::ffi::c_int as mz_uint8,
    6 as ::core::ffi::c_int as mz_uint8,
    6 as ::core::ffi::c_int as mz_uint8,
    6 as ::core::ffi::c_int as mz_uint8,
    6 as ::core::ffi::c_int as mz_uint8,
    6 as ::core::ffi::c_int as mz_uint8,
    6 as ::core::ffi::c_int as mz_uint8,
    6 as ::core::ffi::c_int as mz_uint8,
    6 as ::core::ffi::c_int as mz_uint8,
    6 as ::core::ffi::c_int as mz_uint8,
    6 as ::core::ffi::c_int as mz_uint8,
    6 as ::core::ffi::c_int as mz_uint8,
    6 as ::core::ffi::c_int as mz_uint8,
    6 as ::core::ffi::c_int as mz_uint8,
    6 as ::core::ffi::c_int as mz_uint8,
    6 as ::core::ffi::c_int as mz_uint8,
    6 as ::core::ffi::c_int as mz_uint8,
    6 as ::core::ffi::c_int as mz_uint8,
    6 as ::core::ffi::c_int as mz_uint8,
    6 as ::core::ffi::c_int as mz_uint8,
    6 as ::core::ffi::c_int as mz_uint8,
    6 as ::core::ffi::c_int as mz_uint8,
    6 as ::core::ffi::c_int as mz_uint8,
    6 as ::core::ffi::c_int as mz_uint8,
    6 as ::core::ffi::c_int as mz_uint8,
    6 as ::core::ffi::c_int as mz_uint8,
    6 as ::core::ffi::c_int as mz_uint8,
    6 as ::core::ffi::c_int as mz_uint8,
    6 as ::core::ffi::c_int as mz_uint8,
    6 as ::core::ffi::c_int as mz_uint8,
    6 as ::core::ffi::c_int as mz_uint8,
    6 as ::core::ffi::c_int as mz_uint8,
    6 as ::core::ffi::c_int as mz_uint8,
    6 as ::core::ffi::c_int as mz_uint8,
    6 as ::core::ffi::c_int as mz_uint8,
    6 as ::core::ffi::c_int as mz_uint8,
    6 as ::core::ffi::c_int as mz_uint8,
    6 as ::core::ffi::c_int as mz_uint8,
    6 as ::core::ffi::c_int as mz_uint8,
    6 as ::core::ffi::c_int as mz_uint8,
    6 as ::core::ffi::c_int as mz_uint8,
    6 as ::core::ffi::c_int as mz_uint8,
    6 as ::core::ffi::c_int as mz_uint8,
    6 as ::core::ffi::c_int as mz_uint8,
    6 as ::core::ffi::c_int as mz_uint8,
    6 as ::core::ffi::c_int as mz_uint8,
    6 as ::core::ffi::c_int as mz_uint8,
    6 as ::core::ffi::c_int as mz_uint8,
    6 as ::core::ffi::c_int as mz_uint8,
    6 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
];
static mut s_tdefl_large_dist_sym: [mz_uint8; 128] = [
    0 as ::core::ffi::c_int as mz_uint8,
    0 as ::core::ffi::c_int as mz_uint8,
    18 as ::core::ffi::c_int as mz_uint8,
    19 as ::core::ffi::c_int as mz_uint8,
    20 as ::core::ffi::c_int as mz_uint8,
    20 as ::core::ffi::c_int as mz_uint8,
    21 as ::core::ffi::c_int as mz_uint8,
    21 as ::core::ffi::c_int as mz_uint8,
    22 as ::core::ffi::c_int as mz_uint8,
    22 as ::core::ffi::c_int as mz_uint8,
    22 as ::core::ffi::c_int as mz_uint8,
    22 as ::core::ffi::c_int as mz_uint8,
    23 as ::core::ffi::c_int as mz_uint8,
    23 as ::core::ffi::c_int as mz_uint8,
    23 as ::core::ffi::c_int as mz_uint8,
    23 as ::core::ffi::c_int as mz_uint8,
    24 as ::core::ffi::c_int as mz_uint8,
    24 as ::core::ffi::c_int as mz_uint8,
    24 as ::core::ffi::c_int as mz_uint8,
    24 as ::core::ffi::c_int as mz_uint8,
    24 as ::core::ffi::c_int as mz_uint8,
    24 as ::core::ffi::c_int as mz_uint8,
    24 as ::core::ffi::c_int as mz_uint8,
    24 as ::core::ffi::c_int as mz_uint8,
    25 as ::core::ffi::c_int as mz_uint8,
    25 as ::core::ffi::c_int as mz_uint8,
    25 as ::core::ffi::c_int as mz_uint8,
    25 as ::core::ffi::c_int as mz_uint8,
    25 as ::core::ffi::c_int as mz_uint8,
    25 as ::core::ffi::c_int as mz_uint8,
    25 as ::core::ffi::c_int as mz_uint8,
    25 as ::core::ffi::c_int as mz_uint8,
    26 as ::core::ffi::c_int as mz_uint8,
    26 as ::core::ffi::c_int as mz_uint8,
    26 as ::core::ffi::c_int as mz_uint8,
    26 as ::core::ffi::c_int as mz_uint8,
    26 as ::core::ffi::c_int as mz_uint8,
    26 as ::core::ffi::c_int as mz_uint8,
    26 as ::core::ffi::c_int as mz_uint8,
    26 as ::core::ffi::c_int as mz_uint8,
    26 as ::core::ffi::c_int as mz_uint8,
    26 as ::core::ffi::c_int as mz_uint8,
    26 as ::core::ffi::c_int as mz_uint8,
    26 as ::core::ffi::c_int as mz_uint8,
    26 as ::core::ffi::c_int as mz_uint8,
    26 as ::core::ffi::c_int as mz_uint8,
    26 as ::core::ffi::c_int as mz_uint8,
    26 as ::core::ffi::c_int as mz_uint8,
    27 as ::core::ffi::c_int as mz_uint8,
    27 as ::core::ffi::c_int as mz_uint8,
    27 as ::core::ffi::c_int as mz_uint8,
    27 as ::core::ffi::c_int as mz_uint8,
    27 as ::core::ffi::c_int as mz_uint8,
    27 as ::core::ffi::c_int as mz_uint8,
    27 as ::core::ffi::c_int as mz_uint8,
    27 as ::core::ffi::c_int as mz_uint8,
    27 as ::core::ffi::c_int as mz_uint8,
    27 as ::core::ffi::c_int as mz_uint8,
    27 as ::core::ffi::c_int as mz_uint8,
    27 as ::core::ffi::c_int as mz_uint8,
    27 as ::core::ffi::c_int as mz_uint8,
    27 as ::core::ffi::c_int as mz_uint8,
    27 as ::core::ffi::c_int as mz_uint8,
    27 as ::core::ffi::c_int as mz_uint8,
    28 as ::core::ffi::c_int as mz_uint8,
    28 as ::core::ffi::c_int as mz_uint8,
    28 as ::core::ffi::c_int as mz_uint8,
    28 as ::core::ffi::c_int as mz_uint8,
    28 as ::core::ffi::c_int as mz_uint8,
    28 as ::core::ffi::c_int as mz_uint8,
    28 as ::core::ffi::c_int as mz_uint8,
    28 as ::core::ffi::c_int as mz_uint8,
    28 as ::core::ffi::c_int as mz_uint8,
    28 as ::core::ffi::c_int as mz_uint8,
    28 as ::core::ffi::c_int as mz_uint8,
    28 as ::core::ffi::c_int as mz_uint8,
    28 as ::core::ffi::c_int as mz_uint8,
    28 as ::core::ffi::c_int as mz_uint8,
    28 as ::core::ffi::c_int as mz_uint8,
    28 as ::core::ffi::c_int as mz_uint8,
    28 as ::core::ffi::c_int as mz_uint8,
    28 as ::core::ffi::c_int as mz_uint8,
    28 as ::core::ffi::c_int as mz_uint8,
    28 as ::core::ffi::c_int as mz_uint8,
    28 as ::core::ffi::c_int as mz_uint8,
    28 as ::core::ffi::c_int as mz_uint8,
    28 as ::core::ffi::c_int as mz_uint8,
    28 as ::core::ffi::c_int as mz_uint8,
    28 as ::core::ffi::c_int as mz_uint8,
    28 as ::core::ffi::c_int as mz_uint8,
    28 as ::core::ffi::c_int as mz_uint8,
    28 as ::core::ffi::c_int as mz_uint8,
    28 as ::core::ffi::c_int as mz_uint8,
    28 as ::core::ffi::c_int as mz_uint8,
    28 as ::core::ffi::c_int as mz_uint8,
    28 as ::core::ffi::c_int as mz_uint8,
    29 as ::core::ffi::c_int as mz_uint8,
    29 as ::core::ffi::c_int as mz_uint8,
    29 as ::core::ffi::c_int as mz_uint8,
    29 as ::core::ffi::c_int as mz_uint8,
    29 as ::core::ffi::c_int as mz_uint8,
    29 as ::core::ffi::c_int as mz_uint8,
    29 as ::core::ffi::c_int as mz_uint8,
    29 as ::core::ffi::c_int as mz_uint8,
    29 as ::core::ffi::c_int as mz_uint8,
    29 as ::core::ffi::c_int as mz_uint8,
    29 as ::core::ffi::c_int as mz_uint8,
    29 as ::core::ffi::c_int as mz_uint8,
    29 as ::core::ffi::c_int as mz_uint8,
    29 as ::core::ffi::c_int as mz_uint8,
    29 as ::core::ffi::c_int as mz_uint8,
    29 as ::core::ffi::c_int as mz_uint8,
    29 as ::core::ffi::c_int as mz_uint8,
    29 as ::core::ffi::c_int as mz_uint8,
    29 as ::core::ffi::c_int as mz_uint8,
    29 as ::core::ffi::c_int as mz_uint8,
    29 as ::core::ffi::c_int as mz_uint8,
    29 as ::core::ffi::c_int as mz_uint8,
    29 as ::core::ffi::c_int as mz_uint8,
    29 as ::core::ffi::c_int as mz_uint8,
    29 as ::core::ffi::c_int as mz_uint8,
    29 as ::core::ffi::c_int as mz_uint8,
    29 as ::core::ffi::c_int as mz_uint8,
    29 as ::core::ffi::c_int as mz_uint8,
    29 as ::core::ffi::c_int as mz_uint8,
    29 as ::core::ffi::c_int as mz_uint8,
    29 as ::core::ffi::c_int as mz_uint8,
    29 as ::core::ffi::c_int as mz_uint8,
];
static mut s_tdefl_large_dist_extra: [mz_uint8; 128] = [
    0 as ::core::ffi::c_int as mz_uint8,
    0 as ::core::ffi::c_int as mz_uint8,
    8 as ::core::ffi::c_int as mz_uint8,
    8 as ::core::ffi::c_int as mz_uint8,
    9 as ::core::ffi::c_int as mz_uint8,
    9 as ::core::ffi::c_int as mz_uint8,
    9 as ::core::ffi::c_int as mz_uint8,
    9 as ::core::ffi::c_int as mz_uint8,
    10 as ::core::ffi::c_int as mz_uint8,
    10 as ::core::ffi::c_int as mz_uint8,
    10 as ::core::ffi::c_int as mz_uint8,
    10 as ::core::ffi::c_int as mz_uint8,
    10 as ::core::ffi::c_int as mz_uint8,
    10 as ::core::ffi::c_int as mz_uint8,
    10 as ::core::ffi::c_int as mz_uint8,
    10 as ::core::ffi::c_int as mz_uint8,
    11 as ::core::ffi::c_int as mz_uint8,
    11 as ::core::ffi::c_int as mz_uint8,
    11 as ::core::ffi::c_int as mz_uint8,
    11 as ::core::ffi::c_int as mz_uint8,
    11 as ::core::ffi::c_int as mz_uint8,
    11 as ::core::ffi::c_int as mz_uint8,
    11 as ::core::ffi::c_int as mz_uint8,
    11 as ::core::ffi::c_int as mz_uint8,
    11 as ::core::ffi::c_int as mz_uint8,
    11 as ::core::ffi::c_int as mz_uint8,
    11 as ::core::ffi::c_int as mz_uint8,
    11 as ::core::ffi::c_int as mz_uint8,
    11 as ::core::ffi::c_int as mz_uint8,
    11 as ::core::ffi::c_int as mz_uint8,
    11 as ::core::ffi::c_int as mz_uint8,
    11 as ::core::ffi::c_int as mz_uint8,
    12 as ::core::ffi::c_int as mz_uint8,
    12 as ::core::ffi::c_int as mz_uint8,
    12 as ::core::ffi::c_int as mz_uint8,
    12 as ::core::ffi::c_int as mz_uint8,
    12 as ::core::ffi::c_int as mz_uint8,
    12 as ::core::ffi::c_int as mz_uint8,
    12 as ::core::ffi::c_int as mz_uint8,
    12 as ::core::ffi::c_int as mz_uint8,
    12 as ::core::ffi::c_int as mz_uint8,
    12 as ::core::ffi::c_int as mz_uint8,
    12 as ::core::ffi::c_int as mz_uint8,
    12 as ::core::ffi::c_int as mz_uint8,
    12 as ::core::ffi::c_int as mz_uint8,
    12 as ::core::ffi::c_int as mz_uint8,
    12 as ::core::ffi::c_int as mz_uint8,
    12 as ::core::ffi::c_int as mz_uint8,
    12 as ::core::ffi::c_int as mz_uint8,
    12 as ::core::ffi::c_int as mz_uint8,
    12 as ::core::ffi::c_int as mz_uint8,
    12 as ::core::ffi::c_int as mz_uint8,
    12 as ::core::ffi::c_int as mz_uint8,
    12 as ::core::ffi::c_int as mz_uint8,
    12 as ::core::ffi::c_int as mz_uint8,
    12 as ::core::ffi::c_int as mz_uint8,
    12 as ::core::ffi::c_int as mz_uint8,
    12 as ::core::ffi::c_int as mz_uint8,
    12 as ::core::ffi::c_int as mz_uint8,
    12 as ::core::ffi::c_int as mz_uint8,
    12 as ::core::ffi::c_int as mz_uint8,
    12 as ::core::ffi::c_int as mz_uint8,
    12 as ::core::ffi::c_int as mz_uint8,
    12 as ::core::ffi::c_int as mz_uint8,
    13 as ::core::ffi::c_int as mz_uint8,
    13 as ::core::ffi::c_int as mz_uint8,
    13 as ::core::ffi::c_int as mz_uint8,
    13 as ::core::ffi::c_int as mz_uint8,
    13 as ::core::ffi::c_int as mz_uint8,
    13 as ::core::ffi::c_int as mz_uint8,
    13 as ::core::ffi::c_int as mz_uint8,
    13 as ::core::ffi::c_int as mz_uint8,
    13 as ::core::ffi::c_int as mz_uint8,
    13 as ::core::ffi::c_int as mz_uint8,
    13 as ::core::ffi::c_int as mz_uint8,
    13 as ::core::ffi::c_int as mz_uint8,
    13 as ::core::ffi::c_int as mz_uint8,
    13 as ::core::ffi::c_int as mz_uint8,
    13 as ::core::ffi::c_int as mz_uint8,
    13 as ::core::ffi::c_int as mz_uint8,
    13 as ::core::ffi::c_int as mz_uint8,
    13 as ::core::ffi::c_int as mz_uint8,
    13 as ::core::ffi::c_int as mz_uint8,
    13 as ::core::ffi::c_int as mz_uint8,
    13 as ::core::ffi::c_int as mz_uint8,
    13 as ::core::ffi::c_int as mz_uint8,
    13 as ::core::ffi::c_int as mz_uint8,
    13 as ::core::ffi::c_int as mz_uint8,
    13 as ::core::ffi::c_int as mz_uint8,
    13 as ::core::ffi::c_int as mz_uint8,
    13 as ::core::ffi::c_int as mz_uint8,
    13 as ::core::ffi::c_int as mz_uint8,
    13 as ::core::ffi::c_int as mz_uint8,
    13 as ::core::ffi::c_int as mz_uint8,
    13 as ::core::ffi::c_int as mz_uint8,
    13 as ::core::ffi::c_int as mz_uint8,
    13 as ::core::ffi::c_int as mz_uint8,
    13 as ::core::ffi::c_int as mz_uint8,
    13 as ::core::ffi::c_int as mz_uint8,
    13 as ::core::ffi::c_int as mz_uint8,
    13 as ::core::ffi::c_int as mz_uint8,
    13 as ::core::ffi::c_int as mz_uint8,
    13 as ::core::ffi::c_int as mz_uint8,
    13 as ::core::ffi::c_int as mz_uint8,
    13 as ::core::ffi::c_int as mz_uint8,
    13 as ::core::ffi::c_int as mz_uint8,
    13 as ::core::ffi::c_int as mz_uint8,
    13 as ::core::ffi::c_int as mz_uint8,
    13 as ::core::ffi::c_int as mz_uint8,
    13 as ::core::ffi::c_int as mz_uint8,
    13 as ::core::ffi::c_int as mz_uint8,
    13 as ::core::ffi::c_int as mz_uint8,
    13 as ::core::ffi::c_int as mz_uint8,
    13 as ::core::ffi::c_int as mz_uint8,
    13 as ::core::ffi::c_int as mz_uint8,
    13 as ::core::ffi::c_int as mz_uint8,
    13 as ::core::ffi::c_int as mz_uint8,
    13 as ::core::ffi::c_int as mz_uint8,
    13 as ::core::ffi::c_int as mz_uint8,
    13 as ::core::ffi::c_int as mz_uint8,
    13 as ::core::ffi::c_int as mz_uint8,
    13 as ::core::ffi::c_int as mz_uint8,
    13 as ::core::ffi::c_int as mz_uint8,
    13 as ::core::ffi::c_int as mz_uint8,
    13 as ::core::ffi::c_int as mz_uint8,
    13 as ::core::ffi::c_int as mz_uint8,
    13 as ::core::ffi::c_int as mz_uint8,
    13 as ::core::ffi::c_int as mz_uint8,
];
unsafe extern "C" fn tdefl_radix_sort_syms(
    mut num_syms: mz_uint,
    mut pSyms0: *mut tdefl_sym_freq,
    mut pSyms1: *mut tdefl_sym_freq,
) -> *mut tdefl_sym_freq {
    let mut total_passes: mz_uint32 = 2 as mz_uint32;
    let mut pass_shift: mz_uint32 = 0;
    let mut pass: mz_uint32 = 0;
    let mut i: mz_uint32 = 0;
    let mut hist: [mz_uint32; 512] = [0; 512];
    let mut pCur_syms: *mut tdefl_sym_freq = pSyms0;
    let mut pNew_syms: *mut tdefl_sym_freq = pSyms1;
    memset(
        &raw mut hist as *mut mz_uint32 as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<[mz_uint32; 512]>() as size_t,
    );
    i = 0 as mz_uint32;
    while i < num_syms {
        let mut freq: mz_uint = (*pSyms0.offset(i as isize)).m_key as mz_uint;
        hist[(freq & 0xff as mz_uint) as usize] =
            hist[(freq & 0xff as mz_uint) as usize].wrapping_add(1);
        hist[(256 as mz_uint).wrapping_add(freq >> 8 as ::core::ffi::c_int & 0xff as mz_uint)
            as usize] = hist[(256 as mz_uint)
            .wrapping_add(freq >> 8 as ::core::ffi::c_int & 0xff as mz_uint)
            as usize]
            .wrapping_add(1);
        i = i.wrapping_add(1);
    }
    while total_passes > 1 as mz_uint32
        && num_syms
            == hist[total_passes
                .wrapping_sub(1 as mz_uint32)
                .wrapping_mul(256 as mz_uint32) as usize]
    {
        total_passes = total_passes.wrapping_sub(1);
    }
    pass_shift = 0 as mz_uint32;
    pass = 0 as mz_uint32;
    while pass < total_passes {
        let mut pHist: *const mz_uint32 = (&raw mut hist as *mut mz_uint32)
            .offset((pass << 8 as ::core::ffi::c_int) as isize)
            as *mut mz_uint32;
        let mut offsets: [mz_uint; 256] = [0; 256];
        let mut cur_ofs: mz_uint = 0 as mz_uint;
        i = 0 as mz_uint32;
        while i < 256 as mz_uint32 {
            offsets[i as usize] = cur_ofs;
            cur_ofs = (cur_ofs as uint32_t).wrapping_add(*pHist.offset(i as isize) as uint32_t)
                as mz_uint as mz_uint;
            i = i.wrapping_add(1);
        }
        i = 0 as mz_uint32;
        while i < num_syms {
            let fresh74 = offsets[((*pCur_syms.offset(i as isize)).m_key as ::core::ffi::c_int
                >> pass_shift
                & 0xff as ::core::ffi::c_int) as usize];
            offsets[((*pCur_syms.offset(i as isize)).m_key as ::core::ffi::c_int >> pass_shift
                & 0xff as ::core::ffi::c_int) as usize] =
                offsets[((*pCur_syms.offset(i as isize)).m_key as ::core::ffi::c_int >> pass_shift
                    & 0xff as ::core::ffi::c_int) as usize]
                    .wrapping_add(1);
            *pNew_syms.offset(fresh74 as isize) = *pCur_syms.offset(i as isize);
            i = i.wrapping_add(1);
        }
        let mut t: *mut tdefl_sym_freq = pCur_syms;
        pCur_syms = pNew_syms;
        pNew_syms = t;
        pass = pass.wrapping_add(1);
        pass_shift = pass_shift.wrapping_add(8 as mz_uint32);
    }
    return pCur_syms;
}
unsafe extern "C" fn tdefl_calculate_minimum_redundancy(
    mut A: *mut tdefl_sym_freq,
    mut n: ::core::ffi::c_int,
) {
    let mut root: ::core::ffi::c_int = 0;
    let mut leaf: ::core::ffi::c_int = 0;
    let mut next: ::core::ffi::c_int = 0;
    let mut avbl: ::core::ffi::c_int = 0;
    let mut used: ::core::ffi::c_int = 0;
    let mut dpth: ::core::ffi::c_int = 0;
    if n == 0 as ::core::ffi::c_int {
        return;
    } else if n == 1 as ::core::ffi::c_int {
        (*A.offset(0 as ::core::ffi::c_int as isize)).m_key = 1 as mz_uint16;
        return;
    }
    let ref mut fresh68 = (*A.offset(0 as ::core::ffi::c_int as isize)).m_key;
    *fresh68 = (*fresh68 as ::core::ffi::c_int
        + (*A.offset(1 as ::core::ffi::c_int as isize)).m_key as ::core::ffi::c_int)
        as mz_uint16;
    root = 0 as ::core::ffi::c_int;
    leaf = 2 as ::core::ffi::c_int;
    next = 1 as ::core::ffi::c_int;
    while next < n - 1 as ::core::ffi::c_int {
        if leaf >= n
            || ((*A.offset(root as isize)).m_key as ::core::ffi::c_int)
                < (*A.offset(leaf as isize)).m_key as ::core::ffi::c_int
        {
            (*A.offset(next as isize)).m_key = (*A.offset(root as isize)).m_key;
            let fresh69 = root;
            root = root + 1;
            (*A.offset(fresh69 as isize)).m_key = next as mz_uint16;
        } else {
            let fresh70 = leaf;
            leaf = leaf + 1;
            (*A.offset(next as isize)).m_key = (*A.offset(fresh70 as isize)).m_key;
        }
        if leaf >= n
            || root < next
                && ((*A.offset(root as isize)).m_key as ::core::ffi::c_int)
                    < (*A.offset(leaf as isize)).m_key as ::core::ffi::c_int
        {
            (*A.offset(next as isize)).m_key = ((*A.offset(next as isize)).m_key
                as ::core::ffi::c_int
                + (*A.offset(root as isize)).m_key as ::core::ffi::c_int)
                as mz_uint16;
            let fresh71 = root;
            root = root + 1;
            (*A.offset(fresh71 as isize)).m_key = next as mz_uint16;
        } else {
            let fresh72 = leaf;
            leaf = leaf + 1;
            (*A.offset(next as isize)).m_key = ((*A.offset(next as isize)).m_key
                as ::core::ffi::c_int
                + (*A.offset(fresh72 as isize)).m_key as ::core::ffi::c_int)
                as mz_uint16;
        }
        next += 1;
    }
    (*A.offset((n - 2 as ::core::ffi::c_int) as isize)).m_key = 0 as mz_uint16;
    next = n - 3 as ::core::ffi::c_int;
    while next >= 0 as ::core::ffi::c_int {
        (*A.offset(next as isize)).m_key = ((*A.offset((*A.offset(next as isize)).m_key as isize))
            .m_key as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int) as mz_uint16;
        next -= 1;
    }
    avbl = 1 as ::core::ffi::c_int;
    dpth = 0 as ::core::ffi::c_int;
    used = dpth;
    root = n - 2 as ::core::ffi::c_int;
    next = n - 1 as ::core::ffi::c_int;
    while avbl > 0 as ::core::ffi::c_int {
        while root >= 0 as ::core::ffi::c_int
            && (*A.offset(root as isize)).m_key as ::core::ffi::c_int == dpth
        {
            used += 1;
            root -= 1;
        }
        while avbl > used {
            let fresh73 = next;
            next = next - 1;
            (*A.offset(fresh73 as isize)).m_key = dpth as mz_uint16;
            avbl -= 1;
        }
        avbl = 2 as ::core::ffi::c_int * used;
        dpth += 1;
        used = 0 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn tdefl_huffman_enforce_max_code_size(
    mut pNum_codes: *mut ::core::ffi::c_int,
    mut code_list_len: ::core::ffi::c_int,
    mut max_code_size: ::core::ffi::c_int,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut total: mz_uint32 = 0 as mz_uint32;
    if code_list_len <= 1 as ::core::ffi::c_int {
        return;
    }
    i = max_code_size + 1 as ::core::ffi::c_int;
    while i <= TDEFL_MAX_SUPPORTED_HUFF_CODESIZE as ::core::ffi::c_int {
        *pNum_codes.offset(max_code_size as isize) += *pNum_codes.offset(i as isize);
        i += 1;
    }
    i = max_code_size;
    while i > 0 as ::core::ffi::c_int {
        total =
            total.wrapping_add((*pNum_codes.offset(i as isize) as mz_uint32) << max_code_size - i);
        i -= 1;
    }
    while total as ::core::ffi::c_ulong != (1 as ::core::ffi::c_ulong) << max_code_size {
        let ref mut fresh66 = *pNum_codes.offset(max_code_size as isize);
        *fresh66 -= 1;
        i = max_code_size - 1 as ::core::ffi::c_int;
        while i > 0 as ::core::ffi::c_int {
            if *pNum_codes.offset(i as isize) != 0 {
                let ref mut fresh67 = *pNum_codes.offset(i as isize);
                *fresh67 -= 1;
                *pNum_codes.offset((i + 1 as ::core::ffi::c_int) as isize) +=
                    2 as ::core::ffi::c_int;
                break;
            } else {
                i -= 1;
            }
        }
        total = total.wrapping_sub(1);
    }
}
unsafe extern "C" fn tdefl_optimize_huffman_table(
    mut d: *mut tdefl_compressor,
    mut table_num: ::core::ffi::c_int,
    mut table_len: ::core::ffi::c_int,
    mut code_size_limit: ::core::ffi::c_int,
    mut static_table: ::core::ffi::c_int,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut l: ::core::ffi::c_int = 0;
    let mut num_codes: [::core::ffi::c_int; 33] = [0; 33];
    let mut next_code: [mz_uint; 33] = [0; 33];
    memset(
        &raw mut num_codes as *mut ::core::ffi::c_int as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<[::core::ffi::c_int; 33]>() as size_t,
    );
    if static_table != 0 {
        i = 0 as ::core::ffi::c_int;
        while i < table_len {
            num_codes[(*d).m_huff_code_sizes[table_num as usize][i as usize] as usize] += 1;
            i += 1;
        }
    } else {
        let mut syms0: [tdefl_sym_freq; 288] = [tdefl_sym_freq {
            m_key: 0,
            m_sym_index: 0,
        }; 288];
        let mut syms1: [tdefl_sym_freq; 288] = [tdefl_sym_freq {
            m_key: 0,
            m_sym_index: 0,
        }; 288];
        let mut pSyms: *mut tdefl_sym_freq = ::core::ptr::null_mut::<tdefl_sym_freq>();
        let mut num_used_syms: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut pSym_count: *const mz_uint16 =
            (&raw mut *(&raw mut (*d).m_huff_count as *mut [mz_uint16; 288])
                .offset(table_num as isize) as *mut mz_uint16)
                .offset(0 as ::core::ffi::c_int as isize) as *mut mz_uint16;
        i = 0 as ::core::ffi::c_int;
        while i < table_len {
            if *pSym_count.offset(i as isize) != 0 {
                syms0[num_used_syms as usize].m_key = *pSym_count.offset(i as isize);
                let fresh64 = num_used_syms;
                num_used_syms = num_used_syms + 1;
                syms0[fresh64 as usize].m_sym_index = i as mz_uint16;
            }
            i += 1;
        }
        pSyms = tdefl_radix_sort_syms(
            num_used_syms as mz_uint,
            &raw mut syms0 as *mut tdefl_sym_freq,
            &raw mut syms1 as *mut tdefl_sym_freq,
        );
        tdefl_calculate_minimum_redundancy(pSyms, num_used_syms);
        i = 0 as ::core::ffi::c_int;
        while i < num_used_syms {
            num_codes[(*pSyms.offset(i as isize)).m_key as usize] += 1;
            i += 1;
        }
        tdefl_huffman_enforce_max_code_size(
            &raw mut num_codes as *mut ::core::ffi::c_int,
            num_used_syms,
            code_size_limit,
        );
        memset(
            &raw mut *(&raw mut (*d).m_huff_code_sizes as *mut [mz_uint8; 288])
                .offset(table_num as isize) as *mut mz_uint8
                as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<[mz_uint8; 288]>() as size_t,
        );
        memset(
            &raw mut *(&raw mut (*d).m_huff_codes as *mut [mz_uint16; 288])
                .offset(table_num as isize) as *mut mz_uint16
                as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<[mz_uint16; 288]>() as size_t,
        );
        i = 1 as ::core::ffi::c_int;
        j = num_used_syms;
        while i <= code_size_limit {
            l = num_codes[i as usize];
            while l > 0 as ::core::ffi::c_int {
                j -= 1;
                (*d).m_huff_code_sizes[table_num as usize]
                    [(*pSyms.offset(j as isize)).m_sym_index as usize] = i as mz_uint8;
                l -= 1;
            }
            i += 1;
        }
    }
    next_code[1 as ::core::ffi::c_int as usize] = 0 as mz_uint;
    j = 0 as ::core::ffi::c_int;
    i = 2 as ::core::ffi::c_int;
    while i <= code_size_limit {
        j = j + num_codes[(i - 1 as ::core::ffi::c_int) as usize] << 1 as ::core::ffi::c_int;
        next_code[i as usize] = j as mz_uint;
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < table_len {
        let mut rev_code: mz_uint = 0 as mz_uint;
        let mut code: mz_uint = 0;
        let mut code_size: mz_uint = 0;
        code_size = (*d).m_huff_code_sizes[table_num as usize][i as usize] as mz_uint;
        if !(code_size == 0 as mz_uint) {
            let fresh65 = next_code[code_size as usize];
            next_code[code_size as usize] = next_code[code_size as usize].wrapping_add(1);
            code = fresh65;
            l = code_size as ::core::ffi::c_int;
            while l > 0 as ::core::ffi::c_int {
                rev_code = rev_code << 1 as ::core::ffi::c_int | code & 1 as mz_uint;
                l -= 1;
                code >>= 1 as ::core::ffi::c_int;
            }
            (*d).m_huff_codes[table_num as usize][i as usize] = rev_code as mz_uint16;
        }
        i += 1;
    }
}
static mut s_tdefl_packed_code_size_syms_swizzle: [mz_uint8; 19] = [
    16 as ::core::ffi::c_int as mz_uint8,
    17 as ::core::ffi::c_int as mz_uint8,
    18 as ::core::ffi::c_int as mz_uint8,
    0 as ::core::ffi::c_int as mz_uint8,
    8 as ::core::ffi::c_int as mz_uint8,
    7 as ::core::ffi::c_int as mz_uint8,
    9 as ::core::ffi::c_int as mz_uint8,
    6 as ::core::ffi::c_int as mz_uint8,
    10 as ::core::ffi::c_int as mz_uint8,
    5 as ::core::ffi::c_int as mz_uint8,
    11 as ::core::ffi::c_int as mz_uint8,
    4 as ::core::ffi::c_int as mz_uint8,
    12 as ::core::ffi::c_int as mz_uint8,
    3 as ::core::ffi::c_int as mz_uint8,
    13 as ::core::ffi::c_int as mz_uint8,
    2 as ::core::ffi::c_int as mz_uint8,
    14 as ::core::ffi::c_int as mz_uint8,
    1 as ::core::ffi::c_int as mz_uint8,
    15 as ::core::ffi::c_int as mz_uint8,
];
unsafe extern "C" fn tdefl_start_dynamic_block(mut d: *mut tdefl_compressor) {
    let mut num_lit_codes: ::core::ffi::c_int = 0;
    let mut num_dist_codes: ::core::ffi::c_int = 0;
    let mut num_bit_lengths: ::core::ffi::c_int = 0;
    let mut i: mz_uint = 0;
    let mut total_code_sizes_to_pack: mz_uint = 0;
    let mut num_packed_code_sizes: mz_uint = 0;
    let mut rle_z_count: mz_uint = 0;
    let mut rle_repeat_count: mz_uint = 0;
    let mut packed_code_sizes_index: mz_uint = 0;
    let mut code_sizes_to_pack: [mz_uint8; 320] = [0; 320];
    let mut packed_code_sizes: [mz_uint8; 320] = [0; 320];
    let mut prev_code_size: mz_uint8 = 0xff as mz_uint8;
    (*d).m_huff_count[0 as ::core::ffi::c_int as usize][256 as ::core::ffi::c_int as usize] =
        1 as mz_uint16;
    tdefl_optimize_huffman_table(
        d,
        0 as ::core::ffi::c_int,
        TDEFL_MAX_HUFF_SYMBOLS_0 as ::core::ffi::c_int,
        15 as ::core::ffi::c_int,
        MZ_FALSE,
    );
    tdefl_optimize_huffman_table(
        d,
        1 as ::core::ffi::c_int,
        TDEFL_MAX_HUFF_SYMBOLS_1 as ::core::ffi::c_int,
        15 as ::core::ffi::c_int,
        MZ_FALSE,
    );
    num_lit_codes = 286 as ::core::ffi::c_int;
    while num_lit_codes > 257 as ::core::ffi::c_int {
        if (*d).m_huff_code_sizes[0 as ::core::ffi::c_int as usize]
            [(num_lit_codes - 1 as ::core::ffi::c_int) as usize]
            != 0
        {
            break;
        }
        num_lit_codes -= 1;
    }
    num_dist_codes = 30 as ::core::ffi::c_int;
    while num_dist_codes > 1 as ::core::ffi::c_int {
        if (*d).m_huff_code_sizes[1 as ::core::ffi::c_int as usize]
            [(num_dist_codes - 1 as ::core::ffi::c_int) as usize]
            != 0
        {
            break;
        }
        num_dist_codes -= 1;
    }
    memcpy(
        &raw mut code_sizes_to_pack as *mut mz_uint8 as *mut ::core::ffi::c_void,
        (&raw mut *(&raw mut (*d).m_huff_code_sizes as *mut [mz_uint8; 288])
            .offset(0 as ::core::ffi::c_int as isize) as *mut mz_uint8)
            .offset(0 as ::core::ffi::c_int as isize) as *mut mz_uint8
            as *const ::core::ffi::c_void,
        num_lit_codes as size_t,
    );
    memcpy(
        (&raw mut code_sizes_to_pack as *mut mz_uint8).offset(num_lit_codes as isize)
            as *mut ::core::ffi::c_void,
        (&raw mut *(&raw mut (*d).m_huff_code_sizes as *mut [mz_uint8; 288])
            .offset(1 as ::core::ffi::c_int as isize) as *mut mz_uint8)
            .offset(0 as ::core::ffi::c_int as isize) as *mut mz_uint8
            as *const ::core::ffi::c_void,
        num_dist_codes as size_t,
    );
    total_code_sizes_to_pack = (num_lit_codes + num_dist_codes) as mz_uint;
    num_packed_code_sizes = 0 as mz_uint;
    rle_z_count = 0 as mz_uint;
    rle_repeat_count = 0 as mz_uint;
    memset(
        (&raw mut *(&raw mut (*d).m_huff_count as *mut [mz_uint16; 288])
            .offset(2 as ::core::ffi::c_int as isize) as *mut mz_uint16)
            .offset(0 as ::core::ffi::c_int as isize) as *mut mz_uint16
            as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        (::core::mem::size_of::<mz_uint16>() as size_t)
            .wrapping_mul(TDEFL_MAX_HUFF_SYMBOLS_2 as ::core::ffi::c_int as size_t),
    );
    i = 0 as mz_uint;
    while i < total_code_sizes_to_pack {
        let mut code_size: mz_uint8 = code_sizes_to_pack[i as usize];
        if code_size == 0 {
            if rle_repeat_count != 0 {
                if rle_repeat_count < 3 as mz_uint {
                    (*d).m_huff_count[2 as ::core::ffi::c_int as usize][prev_code_size as usize] =
                        ((*d).m_huff_count[2 as ::core::ffi::c_int as usize]
                            [prev_code_size as usize] as mz_uint)
                            .wrapping_add(rle_repeat_count) as mz_uint16;
                    loop {
                        let fresh20 = rle_repeat_count;
                        rle_repeat_count = rle_repeat_count.wrapping_sub(1);
                        if !(fresh20 != 0) {
                            break;
                        }
                        let fresh21 = num_packed_code_sizes;
                        num_packed_code_sizes = num_packed_code_sizes.wrapping_add(1);
                        packed_code_sizes[fresh21 as usize] = prev_code_size;
                    }
                } else {
                    (*d).m_huff_count[2 as ::core::ffi::c_int as usize]
                        [16 as ::core::ffi::c_int as usize] =
                        ((*d).m_huff_count[2 as ::core::ffi::c_int as usize]
                            [16 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            + 1 as ::core::ffi::c_int) as mz_uint16;
                    let fresh22 = num_packed_code_sizes;
                    num_packed_code_sizes = num_packed_code_sizes.wrapping_add(1);
                    packed_code_sizes[fresh22 as usize] = 16 as mz_uint8;
                    let fresh23 = num_packed_code_sizes;
                    num_packed_code_sizes = num_packed_code_sizes.wrapping_add(1);
                    packed_code_sizes[fresh23 as usize] =
                        rle_repeat_count.wrapping_sub(3 as mz_uint) as mz_uint8;
                }
                rle_repeat_count = 0 as mz_uint;
            }
            rle_z_count = rle_z_count.wrapping_add(1);
            if rle_z_count == 138 as mz_uint {
                if rle_z_count != 0 {
                    if rle_z_count < 3 as mz_uint {
                        (*d).m_huff_count[2 as ::core::ffi::c_int as usize]
                            [0 as ::core::ffi::c_int as usize] =
                            ((*d).m_huff_count[2 as ::core::ffi::c_int as usize]
                                [0 as ::core::ffi::c_int as usize]
                                as mz_uint)
                                .wrapping_add(rle_z_count) as mz_uint16;
                        loop {
                            let fresh24 = rle_z_count;
                            rle_z_count = rle_z_count.wrapping_sub(1);
                            if !(fresh24 != 0) {
                                break;
                            }
                            let fresh25 = num_packed_code_sizes;
                            num_packed_code_sizes = num_packed_code_sizes.wrapping_add(1);
                            packed_code_sizes[fresh25 as usize] = 0 as mz_uint8;
                        }
                    } else if rle_z_count <= 10 as mz_uint {
                        (*d).m_huff_count[2 as ::core::ffi::c_int as usize]
                            [17 as ::core::ffi::c_int as usize] =
                            ((*d).m_huff_count[2 as ::core::ffi::c_int as usize]
                                [17 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                + 1 as ::core::ffi::c_int) as mz_uint16;
                        let fresh26 = num_packed_code_sizes;
                        num_packed_code_sizes = num_packed_code_sizes.wrapping_add(1);
                        packed_code_sizes[fresh26 as usize] = 17 as mz_uint8;
                        let fresh27 = num_packed_code_sizes;
                        num_packed_code_sizes = num_packed_code_sizes.wrapping_add(1);
                        packed_code_sizes[fresh27 as usize] =
                            rle_z_count.wrapping_sub(3 as mz_uint) as mz_uint8;
                    } else {
                        (*d).m_huff_count[2 as ::core::ffi::c_int as usize]
                            [18 as ::core::ffi::c_int as usize] =
                            ((*d).m_huff_count[2 as ::core::ffi::c_int as usize]
                                [18 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                + 1 as ::core::ffi::c_int) as mz_uint16;
                        let fresh28 = num_packed_code_sizes;
                        num_packed_code_sizes = num_packed_code_sizes.wrapping_add(1);
                        packed_code_sizes[fresh28 as usize] = 18 as mz_uint8;
                        let fresh29 = num_packed_code_sizes;
                        num_packed_code_sizes = num_packed_code_sizes.wrapping_add(1);
                        packed_code_sizes[fresh29 as usize] =
                            rle_z_count.wrapping_sub(11 as mz_uint) as mz_uint8;
                    }
                    rle_z_count = 0 as mz_uint;
                }
            }
        } else {
            if rle_z_count != 0 {
                if rle_z_count < 3 as mz_uint {
                    (*d).m_huff_count[2 as ::core::ffi::c_int as usize]
                        [0 as ::core::ffi::c_int as usize] =
                        ((*d).m_huff_count[2 as ::core::ffi::c_int as usize]
                            [0 as ::core::ffi::c_int as usize] as mz_uint)
                            .wrapping_add(rle_z_count) as mz_uint16;
                    loop {
                        let fresh30 = rle_z_count;
                        rle_z_count = rle_z_count.wrapping_sub(1);
                        if !(fresh30 != 0) {
                            break;
                        }
                        let fresh31 = num_packed_code_sizes;
                        num_packed_code_sizes = num_packed_code_sizes.wrapping_add(1);
                        packed_code_sizes[fresh31 as usize] = 0 as mz_uint8;
                    }
                } else if rle_z_count <= 10 as mz_uint {
                    (*d).m_huff_count[2 as ::core::ffi::c_int as usize]
                        [17 as ::core::ffi::c_int as usize] =
                        ((*d).m_huff_count[2 as ::core::ffi::c_int as usize]
                            [17 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            + 1 as ::core::ffi::c_int) as mz_uint16;
                    let fresh32 = num_packed_code_sizes;
                    num_packed_code_sizes = num_packed_code_sizes.wrapping_add(1);
                    packed_code_sizes[fresh32 as usize] = 17 as mz_uint8;
                    let fresh33 = num_packed_code_sizes;
                    num_packed_code_sizes = num_packed_code_sizes.wrapping_add(1);
                    packed_code_sizes[fresh33 as usize] =
                        rle_z_count.wrapping_sub(3 as mz_uint) as mz_uint8;
                } else {
                    (*d).m_huff_count[2 as ::core::ffi::c_int as usize]
                        [18 as ::core::ffi::c_int as usize] =
                        ((*d).m_huff_count[2 as ::core::ffi::c_int as usize]
                            [18 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            + 1 as ::core::ffi::c_int) as mz_uint16;
                    let fresh34 = num_packed_code_sizes;
                    num_packed_code_sizes = num_packed_code_sizes.wrapping_add(1);
                    packed_code_sizes[fresh34 as usize] = 18 as mz_uint8;
                    let fresh35 = num_packed_code_sizes;
                    num_packed_code_sizes = num_packed_code_sizes.wrapping_add(1);
                    packed_code_sizes[fresh35 as usize] =
                        rle_z_count.wrapping_sub(11 as mz_uint) as mz_uint8;
                }
                rle_z_count = 0 as mz_uint;
            }
            if code_size as ::core::ffi::c_int != prev_code_size as ::core::ffi::c_int {
                if rle_repeat_count != 0 {
                    if rle_repeat_count < 3 as mz_uint {
                        (*d).m_huff_count[2 as ::core::ffi::c_int as usize]
                            [prev_code_size as usize] =
                            ((*d).m_huff_count[2 as ::core::ffi::c_int as usize]
                                [prev_code_size as usize] as mz_uint)
                                .wrapping_add(rle_repeat_count)
                                as mz_uint16;
                        loop {
                            let fresh36 = rle_repeat_count;
                            rle_repeat_count = rle_repeat_count.wrapping_sub(1);
                            if !(fresh36 != 0) {
                                break;
                            }
                            let fresh37 = num_packed_code_sizes;
                            num_packed_code_sizes = num_packed_code_sizes.wrapping_add(1);
                            packed_code_sizes[fresh37 as usize] = prev_code_size;
                        }
                    } else {
                        (*d).m_huff_count[2 as ::core::ffi::c_int as usize]
                            [16 as ::core::ffi::c_int as usize] =
                            ((*d).m_huff_count[2 as ::core::ffi::c_int as usize]
                                [16 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                + 1 as ::core::ffi::c_int) as mz_uint16;
                        let fresh38 = num_packed_code_sizes;
                        num_packed_code_sizes = num_packed_code_sizes.wrapping_add(1);
                        packed_code_sizes[fresh38 as usize] = 16 as mz_uint8;
                        let fresh39 = num_packed_code_sizes;
                        num_packed_code_sizes = num_packed_code_sizes.wrapping_add(1);
                        packed_code_sizes[fresh39 as usize] =
                            rle_repeat_count.wrapping_sub(3 as mz_uint) as mz_uint8;
                    }
                    rle_repeat_count = 0 as mz_uint;
                }
                (*d).m_huff_count[2 as ::core::ffi::c_int as usize][code_size as usize] =
                    ((*d).m_huff_count[2 as ::core::ffi::c_int as usize][code_size as usize]
                        as ::core::ffi::c_int
                        + 1 as ::core::ffi::c_int) as mz_uint16;
                let fresh40 = num_packed_code_sizes;
                num_packed_code_sizes = num_packed_code_sizes.wrapping_add(1);
                packed_code_sizes[fresh40 as usize] = code_size;
            } else {
                rle_repeat_count = rle_repeat_count.wrapping_add(1);
                if rle_repeat_count == 6 as mz_uint {
                    if rle_repeat_count != 0 {
                        if rle_repeat_count < 3 as mz_uint {
                            (*d).m_huff_count[2 as ::core::ffi::c_int as usize]
                                [prev_code_size as usize] = ((*d).m_huff_count
                                [2 as ::core::ffi::c_int as usize]
                                [prev_code_size as usize]
                                as mz_uint)
                                .wrapping_add(rle_repeat_count)
                                as mz_uint16;
                            loop {
                                let fresh41 = rle_repeat_count;
                                rle_repeat_count = rle_repeat_count.wrapping_sub(1);
                                if !(fresh41 != 0) {
                                    break;
                                }
                                let fresh42 = num_packed_code_sizes;
                                num_packed_code_sizes = num_packed_code_sizes.wrapping_add(1);
                                packed_code_sizes[fresh42 as usize] = prev_code_size;
                            }
                        } else {
                            (*d).m_huff_count[2 as ::core::ffi::c_int as usize]
                                [16 as ::core::ffi::c_int as usize] = ((*d).m_huff_count
                                [2 as ::core::ffi::c_int as usize]
                                [16 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                + 1 as ::core::ffi::c_int)
                                as mz_uint16;
                            let fresh43 = num_packed_code_sizes;
                            num_packed_code_sizes = num_packed_code_sizes.wrapping_add(1);
                            packed_code_sizes[fresh43 as usize] = 16 as mz_uint8;
                            let fresh44 = num_packed_code_sizes;
                            num_packed_code_sizes = num_packed_code_sizes.wrapping_add(1);
                            packed_code_sizes[fresh44 as usize] =
                                rle_repeat_count.wrapping_sub(3 as mz_uint) as mz_uint8;
                        }
                        rle_repeat_count = 0 as mz_uint;
                    }
                }
            }
        }
        prev_code_size = code_size;
        i = i.wrapping_add(1);
    }
    if rle_repeat_count != 0 {
        if rle_repeat_count != 0 {
            if rle_repeat_count < 3 as mz_uint {
                (*d).m_huff_count[2 as ::core::ffi::c_int as usize][prev_code_size as usize] =
                    ((*d).m_huff_count[2 as ::core::ffi::c_int as usize][prev_code_size as usize]
                        as mz_uint)
                        .wrapping_add(rle_repeat_count) as mz_uint16;
                loop {
                    let fresh45 = rle_repeat_count;
                    rle_repeat_count = rle_repeat_count.wrapping_sub(1);
                    if !(fresh45 != 0) {
                        break;
                    }
                    let fresh46 = num_packed_code_sizes;
                    num_packed_code_sizes = num_packed_code_sizes.wrapping_add(1);
                    packed_code_sizes[fresh46 as usize] = prev_code_size;
                }
            } else {
                (*d).m_huff_count[2 as ::core::ffi::c_int as usize]
                    [16 as ::core::ffi::c_int as usize] = ((*d).m_huff_count
                    [2 as ::core::ffi::c_int as usize][16 as ::core::ffi::c_int as usize]
                    as ::core::ffi::c_int
                    + 1 as ::core::ffi::c_int)
                    as mz_uint16;
                let fresh47 = num_packed_code_sizes;
                num_packed_code_sizes = num_packed_code_sizes.wrapping_add(1);
                packed_code_sizes[fresh47 as usize] = 16 as mz_uint8;
                let fresh48 = num_packed_code_sizes;
                num_packed_code_sizes = num_packed_code_sizes.wrapping_add(1);
                packed_code_sizes[fresh48 as usize] =
                    rle_repeat_count.wrapping_sub(3 as mz_uint) as mz_uint8;
            }
            rle_repeat_count = 0 as mz_uint;
        }
    } else if rle_z_count != 0 {
        if rle_z_count < 3 as mz_uint {
            (*d).m_huff_count[2 as ::core::ffi::c_int as usize][0 as ::core::ffi::c_int as usize] =
                ((*d).m_huff_count[2 as ::core::ffi::c_int as usize]
                    [0 as ::core::ffi::c_int as usize] as mz_uint)
                    .wrapping_add(rle_z_count) as mz_uint16;
            loop {
                let fresh49 = rle_z_count;
                rle_z_count = rle_z_count.wrapping_sub(1);
                if !(fresh49 != 0) {
                    break;
                }
                let fresh50 = num_packed_code_sizes;
                num_packed_code_sizes = num_packed_code_sizes.wrapping_add(1);
                packed_code_sizes[fresh50 as usize] = 0 as mz_uint8;
            }
        } else if rle_z_count <= 10 as mz_uint {
            (*d).m_huff_count[2 as ::core::ffi::c_int as usize]
                [17 as ::core::ffi::c_int as usize] =
                ((*d).m_huff_count[2 as ::core::ffi::c_int as usize]
                    [17 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                    + 1 as ::core::ffi::c_int) as mz_uint16;
            let fresh51 = num_packed_code_sizes;
            num_packed_code_sizes = num_packed_code_sizes.wrapping_add(1);
            packed_code_sizes[fresh51 as usize] = 17 as mz_uint8;
            let fresh52 = num_packed_code_sizes;
            num_packed_code_sizes = num_packed_code_sizes.wrapping_add(1);
            packed_code_sizes[fresh52 as usize] =
                rle_z_count.wrapping_sub(3 as mz_uint) as mz_uint8;
        } else {
            (*d).m_huff_count[2 as ::core::ffi::c_int as usize]
                [18 as ::core::ffi::c_int as usize] =
                ((*d).m_huff_count[2 as ::core::ffi::c_int as usize]
                    [18 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                    + 1 as ::core::ffi::c_int) as mz_uint16;
            let fresh53 = num_packed_code_sizes;
            num_packed_code_sizes = num_packed_code_sizes.wrapping_add(1);
            packed_code_sizes[fresh53 as usize] = 18 as mz_uint8;
            let fresh54 = num_packed_code_sizes;
            num_packed_code_sizes = num_packed_code_sizes.wrapping_add(1);
            packed_code_sizes[fresh54 as usize] =
                rle_z_count.wrapping_sub(11 as mz_uint) as mz_uint8;
        }
        rle_z_count = 0 as mz_uint;
    }
    tdefl_optimize_huffman_table(
        d,
        2 as ::core::ffi::c_int,
        TDEFL_MAX_HUFF_SYMBOLS_2 as ::core::ffi::c_int,
        7 as ::core::ffi::c_int,
        MZ_FALSE,
    );
    let mut bits: mz_uint = 2 as mz_uint;
    let mut len: mz_uint = 2 as mz_uint;
    (*d).m_bit_buffer |= bits << (*d).m_bits_in;
    (*d).m_bits_in = (*d).m_bits_in.wrapping_add(len);
    while (*d).m_bits_in >= 8 as mz_uint {
        if (*d).m_pOutput_buf < (*d).m_pOutput_buf_end {
            let fresh55 = (*d).m_pOutput_buf;
            (*d).m_pOutput_buf = (*d).m_pOutput_buf.offset(1);
            *fresh55 = (*d).m_bit_buffer as mz_uint8;
        }
        (*d).m_bit_buffer >>= 8 as ::core::ffi::c_int;
        (*d).m_bits_in = (*d).m_bits_in.wrapping_sub(8 as mz_uint);
    }
    let mut bits_0: mz_uint = (num_lit_codes - 257 as ::core::ffi::c_int) as mz_uint;
    let mut len_0: mz_uint = 5 as mz_uint;
    (*d).m_bit_buffer |= bits_0 << (*d).m_bits_in;
    (*d).m_bits_in = (*d).m_bits_in.wrapping_add(len_0);
    while (*d).m_bits_in >= 8 as mz_uint {
        if (*d).m_pOutput_buf < (*d).m_pOutput_buf_end {
            let fresh56 = (*d).m_pOutput_buf;
            (*d).m_pOutput_buf = (*d).m_pOutput_buf.offset(1);
            *fresh56 = (*d).m_bit_buffer as mz_uint8;
        }
        (*d).m_bit_buffer >>= 8 as ::core::ffi::c_int;
        (*d).m_bits_in = (*d).m_bits_in.wrapping_sub(8 as mz_uint);
    }
    let mut bits_1: mz_uint = (num_dist_codes - 1 as ::core::ffi::c_int) as mz_uint;
    let mut len_1: mz_uint = 5 as mz_uint;
    (*d).m_bit_buffer |= bits_1 << (*d).m_bits_in;
    (*d).m_bits_in = (*d).m_bits_in.wrapping_add(len_1);
    while (*d).m_bits_in >= 8 as mz_uint {
        if (*d).m_pOutput_buf < (*d).m_pOutput_buf_end {
            let fresh57 = (*d).m_pOutput_buf;
            (*d).m_pOutput_buf = (*d).m_pOutput_buf.offset(1);
            *fresh57 = (*d).m_bit_buffer as mz_uint8;
        }
        (*d).m_bit_buffer >>= 8 as ::core::ffi::c_int;
        (*d).m_bits_in = (*d).m_bits_in.wrapping_sub(8 as mz_uint);
    }
    num_bit_lengths = 18 as ::core::ffi::c_int;
    while num_bit_lengths >= 0 as ::core::ffi::c_int {
        if (*d).m_huff_code_sizes[2 as ::core::ffi::c_int as usize]
            [s_tdefl_packed_code_size_syms_swizzle[num_bit_lengths as usize] as usize]
            != 0
        {
            break;
        }
        num_bit_lengths -= 1;
    }
    num_bit_lengths = if 4 as ::core::ffi::c_int > num_bit_lengths + 1 as ::core::ffi::c_int {
        4 as ::core::ffi::c_int
    } else {
        num_bit_lengths + 1 as ::core::ffi::c_int
    };
    let mut bits_2: mz_uint = (num_bit_lengths - 4 as ::core::ffi::c_int) as mz_uint;
    let mut len_2: mz_uint = 4 as mz_uint;
    (*d).m_bit_buffer |= bits_2 << (*d).m_bits_in;
    (*d).m_bits_in = (*d).m_bits_in.wrapping_add(len_2);
    while (*d).m_bits_in >= 8 as mz_uint {
        if (*d).m_pOutput_buf < (*d).m_pOutput_buf_end {
            let fresh58 = (*d).m_pOutput_buf;
            (*d).m_pOutput_buf = (*d).m_pOutput_buf.offset(1);
            *fresh58 = (*d).m_bit_buffer as mz_uint8;
        }
        (*d).m_bit_buffer >>= 8 as ::core::ffi::c_int;
        (*d).m_bits_in = (*d).m_bits_in.wrapping_sub(8 as mz_uint);
    }
    i = 0 as mz_uint;
    while (i as ::core::ffi::c_int) < num_bit_lengths {
        let mut bits_3: mz_uint = (*d).m_huff_code_sizes[2 as ::core::ffi::c_int as usize]
            [s_tdefl_packed_code_size_syms_swizzle[i as usize] as usize]
            as mz_uint;
        let mut len_3: mz_uint = 3 as mz_uint;
        (*d).m_bit_buffer |= bits_3 << (*d).m_bits_in;
        (*d).m_bits_in = (*d).m_bits_in.wrapping_add(len_3);
        while (*d).m_bits_in >= 8 as mz_uint {
            if (*d).m_pOutput_buf < (*d).m_pOutput_buf_end {
                let fresh59 = (*d).m_pOutput_buf;
                (*d).m_pOutput_buf = (*d).m_pOutput_buf.offset(1);
                *fresh59 = (*d).m_bit_buffer as mz_uint8;
            }
            (*d).m_bit_buffer >>= 8 as ::core::ffi::c_int;
            (*d).m_bits_in = (*d).m_bits_in.wrapping_sub(8 as mz_uint);
        }
        i = i.wrapping_add(1);
    }
    packed_code_sizes_index = 0 as mz_uint;
    while packed_code_sizes_index < num_packed_code_sizes {
        let fresh60 = packed_code_sizes_index;
        packed_code_sizes_index = packed_code_sizes_index.wrapping_add(1);
        let mut code: mz_uint = packed_code_sizes[fresh60 as usize] as mz_uint;
        let mut bits_4: mz_uint =
            (*d).m_huff_codes[2 as ::core::ffi::c_int as usize][code as usize] as mz_uint;
        let mut len_4: mz_uint =
            (*d).m_huff_code_sizes[2 as ::core::ffi::c_int as usize][code as usize] as mz_uint;
        (*d).m_bit_buffer |= bits_4 << (*d).m_bits_in;
        (*d).m_bits_in = (*d).m_bits_in.wrapping_add(len_4);
        while (*d).m_bits_in >= 8 as mz_uint {
            if (*d).m_pOutput_buf < (*d).m_pOutput_buf_end {
                let fresh61 = (*d).m_pOutput_buf;
                (*d).m_pOutput_buf = (*d).m_pOutput_buf.offset(1);
                *fresh61 = (*d).m_bit_buffer as mz_uint8;
            }
            (*d).m_bit_buffer >>= 8 as ::core::ffi::c_int;
            (*d).m_bits_in = (*d).m_bits_in.wrapping_sub(8 as mz_uint);
        }
        if code >= 16 as mz_uint {
            let fresh62 = packed_code_sizes_index;
            packed_code_sizes_index = packed_code_sizes_index.wrapping_add(1);
            let mut bits_5: mz_uint = packed_code_sizes[fresh62 as usize] as mz_uint;
            let mut len_5: mz_uint =
                ::core::mem::transmute::<[u8; 4], [::core::ffi::c_char; 4]>(*b"\x02\x03\x07\0")
                    [code.wrapping_sub(16 as mz_uint) as usize] as mz_uint;
            (*d).m_bit_buffer |= bits_5 << (*d).m_bits_in;
            (*d).m_bits_in = (*d).m_bits_in.wrapping_add(len_5);
            while (*d).m_bits_in >= 8 as mz_uint {
                if (*d).m_pOutput_buf < (*d).m_pOutput_buf_end {
                    let fresh63 = (*d).m_pOutput_buf;
                    (*d).m_pOutput_buf = (*d).m_pOutput_buf.offset(1);
                    *fresh63 = (*d).m_bit_buffer as mz_uint8;
                }
                (*d).m_bit_buffer >>= 8 as ::core::ffi::c_int;
                (*d).m_bits_in = (*d).m_bits_in.wrapping_sub(8 as mz_uint);
            }
        }
    }
}
unsafe extern "C" fn tdefl_start_static_block(mut d: *mut tdefl_compressor) {
    let mut i: mz_uint = 0;
    let mut p: *mut mz_uint8 = (&raw mut *(&raw mut (*d).m_huff_code_sizes as *mut [mz_uint8; 288])
        .offset(0 as ::core::ffi::c_int as isize) as *mut mz_uint8)
        .offset(0 as ::core::ffi::c_int as isize) as *mut mz_uint8;
    i = 0 as mz_uint;
    while i <= 143 as mz_uint {
        let fresh75 = p;
        p = p.offset(1);
        *fresh75 = 8 as mz_uint8;
        i = i.wrapping_add(1);
    }
    while i <= 255 as mz_uint {
        let fresh76 = p;
        p = p.offset(1);
        *fresh76 = 9 as mz_uint8;
        i = i.wrapping_add(1);
    }
    while i <= 279 as mz_uint {
        let fresh77 = p;
        p = p.offset(1);
        *fresh77 = 7 as mz_uint8;
        i = i.wrapping_add(1);
    }
    while i <= 287 as mz_uint {
        let fresh78 = p;
        p = p.offset(1);
        *fresh78 = 8 as mz_uint8;
        i = i.wrapping_add(1);
    }
    memset(
        &raw mut *(&raw mut (*d).m_huff_code_sizes as *mut [mz_uint8; 288])
            .offset(1 as ::core::ffi::c_int as isize) as *mut mz_uint8
            as *mut ::core::ffi::c_void,
        5 as ::core::ffi::c_int,
        32 as size_t,
    );
    tdefl_optimize_huffman_table(
        d,
        0 as ::core::ffi::c_int,
        288 as ::core::ffi::c_int,
        15 as ::core::ffi::c_int,
        MZ_TRUE,
    );
    tdefl_optimize_huffman_table(
        d,
        1 as ::core::ffi::c_int,
        32 as ::core::ffi::c_int,
        15 as ::core::ffi::c_int,
        MZ_TRUE,
    );
    let mut bits: mz_uint = 1 as mz_uint;
    let mut len: mz_uint = 2 as mz_uint;
    (*d).m_bit_buffer |= bits << (*d).m_bits_in;
    (*d).m_bits_in = (*d).m_bits_in.wrapping_add(len);
    while (*d).m_bits_in >= 8 as mz_uint {
        if (*d).m_pOutput_buf < (*d).m_pOutput_buf_end {
            let fresh79 = (*d).m_pOutput_buf;
            (*d).m_pOutput_buf = (*d).m_pOutput_buf.offset(1);
            *fresh79 = (*d).m_bit_buffer as mz_uint8;
        }
        (*d).m_bit_buffer >>= 8 as ::core::ffi::c_int;
        (*d).m_bits_in = (*d).m_bits_in.wrapping_sub(8 as mz_uint);
    }
}
static mut mz_bitmasks: [mz_uint; 17] = [
    0 as ::core::ffi::c_int as mz_uint,
    0x1 as ::core::ffi::c_int as mz_uint,
    0x3 as ::core::ffi::c_int as mz_uint,
    0x7 as ::core::ffi::c_int as mz_uint,
    0xf as ::core::ffi::c_int as mz_uint,
    0x1f as ::core::ffi::c_int as mz_uint,
    0x3f as ::core::ffi::c_int as mz_uint,
    0x7f as ::core::ffi::c_int as mz_uint,
    0xff as ::core::ffi::c_int as mz_uint,
    0x1ff as ::core::ffi::c_int as mz_uint,
    0x3ff as ::core::ffi::c_int as mz_uint,
    0x7ff as ::core::ffi::c_int as mz_uint,
    0xfff as ::core::ffi::c_int as mz_uint,
    0x1fff as ::core::ffi::c_int as mz_uint,
    0x3fff as ::core::ffi::c_int as mz_uint,
    0x7fff as ::core::ffi::c_int as mz_uint,
    0xffff as ::core::ffi::c_int as mz_uint,
];
unsafe extern "C" fn tdefl_compress_lz_codes(mut d: *mut tdefl_compressor) -> mz_bool {
    let mut flags: mz_uint = 0;
    let mut pLZ_codes: *mut mz_uint8 = ::core::ptr::null_mut::<mz_uint8>();
    flags = 1 as mz_uint;
    pLZ_codes = &raw mut (*d).m_lz_code_buf as *mut mz_uint8;
    while pLZ_codes < (*d).m_pLZ_code_buf {
        if flags == 1 as mz_uint {
            let fresh12 = pLZ_codes;
            pLZ_codes = pLZ_codes.offset(1);
            flags = (*fresh12 as ::core::ffi::c_int | 0x100 as ::core::ffi::c_int) as mz_uint;
        }
        if flags & 1 as mz_uint != 0 {
            let mut sym: mz_uint = 0;
            let mut num_extra_bits: mz_uint = 0;
            let mut match_len: mz_uint =
                *pLZ_codes.offset(0 as ::core::ffi::c_int as isize) as mz_uint;
            let mut match_dist: mz_uint =
                (*pLZ_codes.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    | (*pLZ_codes.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                        << 8 as ::core::ffi::c_int) as mz_uint;
            pLZ_codes = pLZ_codes.offset(3 as ::core::ffi::c_int as isize);
            let mut bits: mz_uint = (*d).m_huff_codes[0 as ::core::ffi::c_int as usize]
                [s_tdefl_len_sym[match_len as usize] as usize]
                as mz_uint;
            let mut len: mz_uint = (*d).m_huff_code_sizes[0 as ::core::ffi::c_int as usize]
                [s_tdefl_len_sym[match_len as usize] as usize]
                as mz_uint;
            (*d).m_bit_buffer |= bits << (*d).m_bits_in;
            (*d).m_bits_in = (*d).m_bits_in.wrapping_add(len);
            while (*d).m_bits_in >= 8 as mz_uint {
                if (*d).m_pOutput_buf < (*d).m_pOutput_buf_end {
                    let fresh13 = (*d).m_pOutput_buf;
                    (*d).m_pOutput_buf = (*d).m_pOutput_buf.offset(1);
                    *fresh13 = (*d).m_bit_buffer as mz_uint8;
                }
                (*d).m_bit_buffer >>= 8 as ::core::ffi::c_int;
                (*d).m_bits_in = (*d).m_bits_in.wrapping_sub(8 as mz_uint);
            }
            let mut bits_0: mz_uint =
                match_len & mz_bitmasks[s_tdefl_len_extra[match_len as usize] as usize];
            let mut len_0: mz_uint = s_tdefl_len_extra[match_len as usize] as mz_uint;
            (*d).m_bit_buffer |= bits_0 << (*d).m_bits_in;
            (*d).m_bits_in = (*d).m_bits_in.wrapping_add(len_0);
            while (*d).m_bits_in >= 8 as mz_uint {
                if (*d).m_pOutput_buf < (*d).m_pOutput_buf_end {
                    let fresh14 = (*d).m_pOutput_buf;
                    (*d).m_pOutput_buf = (*d).m_pOutput_buf.offset(1);
                    *fresh14 = (*d).m_bit_buffer as mz_uint8;
                }
                (*d).m_bit_buffer >>= 8 as ::core::ffi::c_int;
                (*d).m_bits_in = (*d).m_bits_in.wrapping_sub(8 as mz_uint);
            }
            if match_dist < 512 as mz_uint {
                sym = s_tdefl_small_dist_sym[match_dist as usize] as mz_uint;
                num_extra_bits = s_tdefl_small_dist_extra[match_dist as usize] as mz_uint;
            } else {
                sym = s_tdefl_large_dist_sym[(match_dist >> 8 as ::core::ffi::c_int) as usize]
                    as mz_uint;
                num_extra_bits = s_tdefl_large_dist_extra
                    [(match_dist >> 8 as ::core::ffi::c_int) as usize]
                    as mz_uint;
            }
            let mut bits_1: mz_uint =
                (*d).m_huff_codes[1 as ::core::ffi::c_int as usize][sym as usize] as mz_uint;
            let mut len_1: mz_uint =
                (*d).m_huff_code_sizes[1 as ::core::ffi::c_int as usize][sym as usize] as mz_uint;
            (*d).m_bit_buffer |= bits_1 << (*d).m_bits_in;
            (*d).m_bits_in = (*d).m_bits_in.wrapping_add(len_1);
            while (*d).m_bits_in >= 8 as mz_uint {
                if (*d).m_pOutput_buf < (*d).m_pOutput_buf_end {
                    let fresh15 = (*d).m_pOutput_buf;
                    (*d).m_pOutput_buf = (*d).m_pOutput_buf.offset(1);
                    *fresh15 = (*d).m_bit_buffer as mz_uint8;
                }
                (*d).m_bit_buffer >>= 8 as ::core::ffi::c_int;
                (*d).m_bits_in = (*d).m_bits_in.wrapping_sub(8 as mz_uint);
            }
            let mut bits_2: mz_uint = match_dist & mz_bitmasks[num_extra_bits as usize];
            let mut len_2: mz_uint = num_extra_bits;
            (*d).m_bit_buffer |= bits_2 << (*d).m_bits_in;
            (*d).m_bits_in = (*d).m_bits_in.wrapping_add(len_2);
            while (*d).m_bits_in >= 8 as mz_uint {
                if (*d).m_pOutput_buf < (*d).m_pOutput_buf_end {
                    let fresh16 = (*d).m_pOutput_buf;
                    (*d).m_pOutput_buf = (*d).m_pOutput_buf.offset(1);
                    *fresh16 = (*d).m_bit_buffer as mz_uint8;
                }
                (*d).m_bit_buffer >>= 8 as ::core::ffi::c_int;
                (*d).m_bits_in = (*d).m_bits_in.wrapping_sub(8 as mz_uint);
            }
        } else {
            let fresh17 = pLZ_codes;
            pLZ_codes = pLZ_codes.offset(1);
            let mut lit: mz_uint = *fresh17 as mz_uint;
            let mut bits_3: mz_uint =
                (*d).m_huff_codes[0 as ::core::ffi::c_int as usize][lit as usize] as mz_uint;
            let mut len_3: mz_uint =
                (*d).m_huff_code_sizes[0 as ::core::ffi::c_int as usize][lit as usize] as mz_uint;
            (*d).m_bit_buffer |= bits_3 << (*d).m_bits_in;
            (*d).m_bits_in = (*d).m_bits_in.wrapping_add(len_3);
            while (*d).m_bits_in >= 8 as mz_uint {
                if (*d).m_pOutput_buf < (*d).m_pOutput_buf_end {
                    let fresh18 = (*d).m_pOutput_buf;
                    (*d).m_pOutput_buf = (*d).m_pOutput_buf.offset(1);
                    *fresh18 = (*d).m_bit_buffer as mz_uint8;
                }
                (*d).m_bit_buffer >>= 8 as ::core::ffi::c_int;
                (*d).m_bits_in = (*d).m_bits_in.wrapping_sub(8 as mz_uint);
            }
        }
        flags >>= 1 as ::core::ffi::c_int;
    }
    let mut bits_4: mz_uint = (*d).m_huff_codes[0 as ::core::ffi::c_int as usize]
        [256 as ::core::ffi::c_int as usize] as mz_uint;
    let mut len_4: mz_uint = (*d).m_huff_code_sizes[0 as ::core::ffi::c_int as usize]
        [256 as ::core::ffi::c_int as usize] as mz_uint;
    (*d).m_bit_buffer |= bits_4 << (*d).m_bits_in;
    (*d).m_bits_in = (*d).m_bits_in.wrapping_add(len_4);
    while (*d).m_bits_in >= 8 as mz_uint {
        if (*d).m_pOutput_buf < (*d).m_pOutput_buf_end {
            let fresh19 = (*d).m_pOutput_buf;
            (*d).m_pOutput_buf = (*d).m_pOutput_buf.offset(1);
            *fresh19 = (*d).m_bit_buffer as mz_uint8;
        }
        (*d).m_bit_buffer >>= 8 as ::core::ffi::c_int;
        (*d).m_bits_in = (*d).m_bits_in.wrapping_sub(8 as mz_uint);
    }
    return ((*d).m_pOutput_buf < (*d).m_pOutput_buf_end) as ::core::ffi::c_int;
}
unsafe extern "C" fn tdefl_compress_block(
    mut d: *mut tdefl_compressor,
    mut static_block: mz_bool,
) -> mz_bool {
    if static_block != 0 {
        tdefl_start_static_block(d);
    } else {
        tdefl_start_dynamic_block(d);
    }
    return tdefl_compress_lz_codes(d);
}
static mut s_tdefl_num_probes: [mz_uint; 11] = [
    0 as ::core::ffi::c_int as mz_uint,
    1 as ::core::ffi::c_int as mz_uint,
    6 as ::core::ffi::c_int as mz_uint,
    32 as ::core::ffi::c_int as mz_uint,
    16 as ::core::ffi::c_int as mz_uint,
    32 as ::core::ffi::c_int as mz_uint,
    128 as ::core::ffi::c_int as mz_uint,
    256 as ::core::ffi::c_int as mz_uint,
    512 as ::core::ffi::c_int as mz_uint,
    768 as ::core::ffi::c_int as mz_uint,
    1500 as ::core::ffi::c_int as mz_uint,
];
unsafe extern "C" fn tdefl_flush_block(
    mut d: *mut tdefl_compressor,
    mut flush: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut saved_bit_buf: mz_uint = 0;
    let mut saved_bits_in: mz_uint = 0;
    let mut pSaved_output_buf: *mut mz_uint8 = ::core::ptr::null_mut::<mz_uint8>();
    let mut comp_block_succeeded: mz_bool = MZ_FALSE;
    let mut n: ::core::ffi::c_int = 0;
    let mut use_raw_block: ::core::ffi::c_int =
        ((*d).m_flags & TDEFL_FORCE_ALL_RAW_BLOCKS as ::core::ffi::c_int as mz_uint != 0 as mz_uint
            && (*d)
                .m_lookahead_pos
                .wrapping_sub((*d).m_lz_code_buf_dict_pos)
                <= (*d).m_dict_size) as ::core::ffi::c_int;
    let mut pOutput_buf_start: *mut mz_uint8 = if (*d).m_pPut_buf_func.is_none()
        && (*(*d).m_pOut_buf_size).wrapping_sub((*d).m_out_buf_ofs)
            >= TDEFL_OUT_BUF_SIZE as ::core::ffi::c_int as size_t
    {
        ((*d).m_pOut_buf as *mut mz_uint8).offset((*d).m_out_buf_ofs as isize)
    } else {
        &raw mut (*d).m_output_buf as *mut mz_uint8
    };
    (*d).m_pOutput_buf = pOutput_buf_start;
    (*d).m_pOutput_buf_end = (*d)
        .m_pOutput_buf
        .offset(TDEFL_OUT_BUF_SIZE as ::core::ffi::c_int as isize)
        .offset(-(16 as ::core::ffi::c_int as isize));
    (*d).m_output_flush_ofs = 0 as mz_uint;
    (*d).m_output_flush_remaining = 0 as mz_uint;
    *(*d).m_pLZ_flags =
        (*(*d).m_pLZ_flags as ::core::ffi::c_int >> (*d).m_num_flags_left) as mz_uint8;
    (*d).m_pLZ_code_buf = (*d)
        .m_pLZ_code_buf
        .offset(-(((*d).m_num_flags_left == 8 as mz_uint) as ::core::ffi::c_int as isize));
    if (*d).m_flags & TDEFL_WRITE_ZLIB_HEADER as ::core::ffi::c_int as mz_uint != 0
        && (*d).m_block_index == 0
    {
        let cmf: mz_uint8 = 0x78 as mz_uint8;
        let mut flg: mz_uint8 = 0;
        let mut flevel: mz_uint8 = 3 as mz_uint8;
        let mut header: mz_uint = 0;
        let mut i: mz_uint = 0;
        let mut mz_un: mz_uint = (::core::mem::size_of::<[mz_uint; 11]>() as usize)
            .wrapping_div(::core::mem::size_of::<mz_uint>() as usize)
            as mz_uint;
        i = 0 as mz_uint;
        while i < mz_un {
            if s_tdefl_num_probes[i as usize] == (*d).m_flags & 0xfff as mz_uint {
                break;
            }
            i = i.wrapping_add(1);
        }
        if i < 2 as mz_uint {
            flevel = 0 as mz_uint8;
        } else if i < 6 as mz_uint {
            flevel = 1 as mz_uint8;
        } else if i == 6 as mz_uint {
            flevel = 2 as mz_uint8;
        }
        header = ((cmf as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
            | (flevel as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
            as mz_uint;
        header =
            header.wrapping_add((31 as mz_uint).wrapping_sub(header.wrapping_rem(31 as mz_uint)));
        flg = (header & 0xff as mz_uint) as mz_uint8;
        let mut bits: mz_uint = cmf as mz_uint;
        let mut len: mz_uint = 8 as mz_uint;
        (*d).m_bit_buffer |= bits << (*d).m_bits_in;
        (*d).m_bits_in = (*d).m_bits_in.wrapping_add(len);
        while (*d).m_bits_in >= 8 as mz_uint {
            if (*d).m_pOutput_buf < (*d).m_pOutput_buf_end {
                let fresh0 = (*d).m_pOutput_buf;
                (*d).m_pOutput_buf = (*d).m_pOutput_buf.offset(1);
                *fresh0 = (*d).m_bit_buffer as mz_uint8;
            }
            (*d).m_bit_buffer >>= 8 as ::core::ffi::c_int;
            (*d).m_bits_in = (*d).m_bits_in.wrapping_sub(8 as mz_uint);
        }
        let mut bits_0: mz_uint = flg as mz_uint;
        let mut len_0: mz_uint = 8 as mz_uint;
        (*d).m_bit_buffer |= bits_0 << (*d).m_bits_in;
        (*d).m_bits_in = (*d).m_bits_in.wrapping_add(len_0);
        while (*d).m_bits_in >= 8 as mz_uint {
            if (*d).m_pOutput_buf < (*d).m_pOutput_buf_end {
                let fresh1 = (*d).m_pOutput_buf;
                (*d).m_pOutput_buf = (*d).m_pOutput_buf.offset(1);
                *fresh1 = (*d).m_bit_buffer as mz_uint8;
            }
            (*d).m_bit_buffer >>= 8 as ::core::ffi::c_int;
            (*d).m_bits_in = (*d).m_bits_in.wrapping_sub(8 as mz_uint);
        }
    }
    let mut bits_1: mz_uint =
        (flush == TDEFL_FINISH as ::core::ffi::c_int) as ::core::ffi::c_int as mz_uint;
    let mut len_1: mz_uint = 1 as mz_uint;
    (*d).m_bit_buffer |= bits_1 << (*d).m_bits_in;
    (*d).m_bits_in = (*d).m_bits_in.wrapping_add(len_1);
    while (*d).m_bits_in >= 8 as mz_uint {
        if (*d).m_pOutput_buf < (*d).m_pOutput_buf_end {
            let fresh2 = (*d).m_pOutput_buf;
            (*d).m_pOutput_buf = (*d).m_pOutput_buf.offset(1);
            *fresh2 = (*d).m_bit_buffer as mz_uint8;
        }
        (*d).m_bit_buffer >>= 8 as ::core::ffi::c_int;
        (*d).m_bits_in = (*d).m_bits_in.wrapping_sub(8 as mz_uint);
    }
    pSaved_output_buf = (*d).m_pOutput_buf;
    saved_bit_buf = (*d).m_bit_buffer;
    saved_bits_in = (*d).m_bits_in;
    if use_raw_block == 0 {
        comp_block_succeeded = tdefl_compress_block(
            d,
            ((*d).m_flags & TDEFL_FORCE_ALL_STATIC_BLOCKS as ::core::ffi::c_int as mz_uint != 0
                || (*d).m_total_lz_bytes < 48 as mz_uint) as ::core::ffi::c_int,
        );
    }
    if (use_raw_block != 0
        || (*d).m_total_lz_bytes != 0
            && (*d).m_pOutput_buf.offset_from(pSaved_output_buf) as ::core::ffi::c_long
                + 1 as ::core::ffi::c_long
                >= (*d).m_total_lz_bytes as ::core::ffi::c_long)
        && (*d)
            .m_lookahead_pos
            .wrapping_sub((*d).m_lz_code_buf_dict_pos)
            <= (*d).m_dict_size
    {
        let mut i_0: mz_uint = 0;
        (*d).m_pOutput_buf = pSaved_output_buf;
        (*d).m_bit_buffer = saved_bit_buf;
        (*d).m_bits_in = saved_bits_in;
        let mut bits_2: mz_uint = 0 as mz_uint;
        let mut len_2: mz_uint = 2 as mz_uint;
        (*d).m_bit_buffer |= bits_2 << (*d).m_bits_in;
        (*d).m_bits_in = (*d).m_bits_in.wrapping_add(len_2);
        while (*d).m_bits_in >= 8 as mz_uint {
            if (*d).m_pOutput_buf < (*d).m_pOutput_buf_end {
                let fresh3 = (*d).m_pOutput_buf;
                (*d).m_pOutput_buf = (*d).m_pOutput_buf.offset(1);
                *fresh3 = (*d).m_bit_buffer as mz_uint8;
            }
            (*d).m_bit_buffer >>= 8 as ::core::ffi::c_int;
            (*d).m_bits_in = (*d).m_bits_in.wrapping_sub(8 as mz_uint);
        }
        if (*d).m_bits_in != 0 {
            let mut bits_3: mz_uint = 0 as mz_uint;
            let mut len_3: mz_uint = (8 as mz_uint).wrapping_sub((*d).m_bits_in);
            (*d).m_bit_buffer |= bits_3 << (*d).m_bits_in;
            (*d).m_bits_in = (*d).m_bits_in.wrapping_add(len_3);
            while (*d).m_bits_in >= 8 as mz_uint {
                if (*d).m_pOutput_buf < (*d).m_pOutput_buf_end {
                    let fresh4 = (*d).m_pOutput_buf;
                    (*d).m_pOutput_buf = (*d).m_pOutput_buf.offset(1);
                    *fresh4 = (*d).m_bit_buffer as mz_uint8;
                }
                (*d).m_bit_buffer >>= 8 as ::core::ffi::c_int;
                (*d).m_bits_in = (*d).m_bits_in.wrapping_sub(8 as mz_uint);
            }
        }
        i_0 = 2 as mz_uint;
        while i_0 != 0 {
            let mut bits_4: mz_uint = (*d).m_total_lz_bytes & 0xffff as mz_uint;
            let mut len_4: mz_uint = 16 as mz_uint;
            (*d).m_bit_buffer |= bits_4 << (*d).m_bits_in;
            (*d).m_bits_in = (*d).m_bits_in.wrapping_add(len_4);
            while (*d).m_bits_in >= 8 as mz_uint {
                if (*d).m_pOutput_buf < (*d).m_pOutput_buf_end {
                    let fresh5 = (*d).m_pOutput_buf;
                    (*d).m_pOutput_buf = (*d).m_pOutput_buf.offset(1);
                    *fresh5 = (*d).m_bit_buffer as mz_uint8;
                }
                (*d).m_bit_buffer >>= 8 as ::core::ffi::c_int;
                (*d).m_bits_in = (*d).m_bits_in.wrapping_sub(8 as mz_uint);
            }
            i_0 = i_0.wrapping_sub(1);
            (*d).m_total_lz_bytes ^= 0xffff as mz_uint;
        }
        i_0 = 0 as mz_uint;
        while i_0 < (*d).m_total_lz_bytes {
            let mut bits_5: mz_uint = (*d).m_dict[((*d).m_lz_code_buf_dict_pos.wrapping_add(i_0)
                & TDEFL_LZ_DICT_SIZE_MASK as ::core::ffi::c_int as mz_uint)
                as usize] as mz_uint;
            let mut len_5: mz_uint = 8 as mz_uint;
            (*d).m_bit_buffer |= bits_5 << (*d).m_bits_in;
            (*d).m_bits_in = (*d).m_bits_in.wrapping_add(len_5);
            while (*d).m_bits_in >= 8 as mz_uint {
                if (*d).m_pOutput_buf < (*d).m_pOutput_buf_end {
                    let fresh6 = (*d).m_pOutput_buf;
                    (*d).m_pOutput_buf = (*d).m_pOutput_buf.offset(1);
                    *fresh6 = (*d).m_bit_buffer as mz_uint8;
                }
                (*d).m_bit_buffer >>= 8 as ::core::ffi::c_int;
                (*d).m_bits_in = (*d).m_bits_in.wrapping_sub(8 as mz_uint);
            }
            i_0 = i_0.wrapping_add(1);
        }
    } else if comp_block_succeeded == 0 {
        (*d).m_pOutput_buf = pSaved_output_buf;
        (*d).m_bit_buffer = saved_bit_buf;
        (*d).m_bits_in = saved_bits_in;
        tdefl_compress_block(d, MZ_TRUE);
    }
    if flush != 0 {
        if flush == TDEFL_FINISH as ::core::ffi::c_int {
            if (*d).m_bits_in != 0 {
                let mut bits_6: mz_uint = 0 as mz_uint;
                let mut len_6: mz_uint = (8 as mz_uint).wrapping_sub((*d).m_bits_in);
                (*d).m_bit_buffer |= bits_6 << (*d).m_bits_in;
                (*d).m_bits_in = (*d).m_bits_in.wrapping_add(len_6);
                while (*d).m_bits_in >= 8 as mz_uint {
                    if (*d).m_pOutput_buf < (*d).m_pOutput_buf_end {
                        let fresh7 = (*d).m_pOutput_buf;
                        (*d).m_pOutput_buf = (*d).m_pOutput_buf.offset(1);
                        *fresh7 = (*d).m_bit_buffer as mz_uint8;
                    }
                    (*d).m_bit_buffer >>= 8 as ::core::ffi::c_int;
                    (*d).m_bits_in = (*d).m_bits_in.wrapping_sub(8 as mz_uint);
                }
            }
            if (*d).m_flags & TDEFL_WRITE_ZLIB_HEADER as ::core::ffi::c_int as mz_uint != 0 {
                let mut i_1: mz_uint = 0;
                let mut a: mz_uint = (*d).m_adler32;
                i_1 = 0 as mz_uint;
                while i_1 < 4 as mz_uint {
                    let mut bits_7: mz_uint = a >> 24 as ::core::ffi::c_int & 0xff as mz_uint;
                    let mut len_7: mz_uint = 8 as mz_uint;
                    (*d).m_bit_buffer |= bits_7 << (*d).m_bits_in;
                    (*d).m_bits_in = (*d).m_bits_in.wrapping_add(len_7);
                    while (*d).m_bits_in >= 8 as mz_uint {
                        if (*d).m_pOutput_buf < (*d).m_pOutput_buf_end {
                            let fresh8 = (*d).m_pOutput_buf;
                            (*d).m_pOutput_buf = (*d).m_pOutput_buf.offset(1);
                            *fresh8 = (*d).m_bit_buffer as mz_uint8;
                        }
                        (*d).m_bit_buffer >>= 8 as ::core::ffi::c_int;
                        (*d).m_bits_in = (*d).m_bits_in.wrapping_sub(8 as mz_uint);
                    }
                    a <<= 8 as ::core::ffi::c_int;
                    i_1 = i_1.wrapping_add(1);
                }
            }
        } else {
            let mut i_2: mz_uint = 0;
            let mut z: mz_uint = 0 as mz_uint;
            let mut bits_8: mz_uint = 0 as mz_uint;
            let mut len_8: mz_uint = 3 as mz_uint;
            (*d).m_bit_buffer |= bits_8 << (*d).m_bits_in;
            (*d).m_bits_in = (*d).m_bits_in.wrapping_add(len_8);
            while (*d).m_bits_in >= 8 as mz_uint {
                if (*d).m_pOutput_buf < (*d).m_pOutput_buf_end {
                    let fresh9 = (*d).m_pOutput_buf;
                    (*d).m_pOutput_buf = (*d).m_pOutput_buf.offset(1);
                    *fresh9 = (*d).m_bit_buffer as mz_uint8;
                }
                (*d).m_bit_buffer >>= 8 as ::core::ffi::c_int;
                (*d).m_bits_in = (*d).m_bits_in.wrapping_sub(8 as mz_uint);
            }
            if (*d).m_bits_in != 0 {
                let mut bits_9: mz_uint = 0 as mz_uint;
                let mut len_9: mz_uint = (8 as mz_uint).wrapping_sub((*d).m_bits_in);
                (*d).m_bit_buffer |= bits_9 << (*d).m_bits_in;
                (*d).m_bits_in = (*d).m_bits_in.wrapping_add(len_9);
                while (*d).m_bits_in >= 8 as mz_uint {
                    if (*d).m_pOutput_buf < (*d).m_pOutput_buf_end {
                        let fresh10 = (*d).m_pOutput_buf;
                        (*d).m_pOutput_buf = (*d).m_pOutput_buf.offset(1);
                        *fresh10 = (*d).m_bit_buffer as mz_uint8;
                    }
                    (*d).m_bit_buffer >>= 8 as ::core::ffi::c_int;
                    (*d).m_bits_in = (*d).m_bits_in.wrapping_sub(8 as mz_uint);
                }
            }
            i_2 = 2 as mz_uint;
            while i_2 != 0 {
                let mut bits_10: mz_uint = z & 0xffff as mz_uint;
                let mut len_10: mz_uint = 16 as mz_uint;
                (*d).m_bit_buffer |= bits_10 << (*d).m_bits_in;
                (*d).m_bits_in = (*d).m_bits_in.wrapping_add(len_10);
                while (*d).m_bits_in >= 8 as mz_uint {
                    if (*d).m_pOutput_buf < (*d).m_pOutput_buf_end {
                        let fresh11 = (*d).m_pOutput_buf;
                        (*d).m_pOutput_buf = (*d).m_pOutput_buf.offset(1);
                        *fresh11 = (*d).m_bit_buffer as mz_uint8;
                    }
                    (*d).m_bit_buffer >>= 8 as ::core::ffi::c_int;
                    (*d).m_bits_in = (*d).m_bits_in.wrapping_sub(8 as mz_uint);
                }
                i_2 = i_2.wrapping_sub(1);
                z ^= 0xffff as mz_uint;
            }
        }
    }
    memset(
        (&raw mut *(&raw mut (*d).m_huff_count as *mut [mz_uint16; 288])
            .offset(0 as ::core::ffi::c_int as isize) as *mut mz_uint16)
            .offset(0 as ::core::ffi::c_int as isize) as *mut mz_uint16
            as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        (::core::mem::size_of::<mz_uint16>() as size_t)
            .wrapping_mul(TDEFL_MAX_HUFF_SYMBOLS_0 as ::core::ffi::c_int as size_t),
    );
    memset(
        (&raw mut *(&raw mut (*d).m_huff_count as *mut [mz_uint16; 288])
            .offset(1 as ::core::ffi::c_int as isize) as *mut mz_uint16)
            .offset(0 as ::core::ffi::c_int as isize) as *mut mz_uint16
            as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        (::core::mem::size_of::<mz_uint16>() as size_t)
            .wrapping_mul(TDEFL_MAX_HUFF_SYMBOLS_1 as ::core::ffi::c_int as size_t),
    );
    (*d).m_pLZ_code_buf =
        (&raw mut (*d).m_lz_code_buf as *mut mz_uint8).offset(1 as ::core::ffi::c_int as isize);
    (*d).m_pLZ_flags = &raw mut (*d).m_lz_code_buf as *mut mz_uint8;
    (*d).m_num_flags_left = 8 as mz_uint;
    (*d).m_lz_code_buf_dict_pos = (*d)
        .m_lz_code_buf_dict_pos
        .wrapping_add((*d).m_total_lz_bytes);
    (*d).m_total_lz_bytes = 0 as mz_uint;
    (*d).m_block_index = (*d).m_block_index.wrapping_add(1);
    n = (*d).m_pOutput_buf.offset_from(pOutput_buf_start) as ::core::ffi::c_long
        as ::core::ffi::c_int;
    if n != 0 as ::core::ffi::c_int {
        if (*d).m_pPut_buf_func.is_some() {
            *(*d).m_pIn_buf_size = (*d).m_pSrc.offset_from((*d).m_pIn_buf as *const mz_uint8)
                as ::core::ffi::c_long as size_t;
            if Some((*d).m_pPut_buf_func.expect("non-null function pointer"))
                .expect("non-null function pointer")(
                &raw mut (*d).m_output_buf as *mut mz_uint8 as *const ::core::ffi::c_void,
                n,
                (*d).m_pPut_buf_user,
            ) == 0
            {
                (*d).m_prev_return_status = TDEFL_STATUS_PUT_BUF_FAILED;
                return (*d).m_prev_return_status as ::core::ffi::c_int;
            }
        } else if pOutput_buf_start == &raw mut (*d).m_output_buf as *mut mz_uint8 {
            let mut bytes_to_copy: ::core::ffi::c_int =
                (if (n as size_t) < (*(*d).m_pOut_buf_size).wrapping_sub((*d).m_out_buf_ofs) {
                    n as size_t
                } else {
                    (*(*d).m_pOut_buf_size).wrapping_sub((*d).m_out_buf_ofs)
                }) as ::core::ffi::c_int;
            memcpy(
                ((*d).m_pOut_buf as *mut mz_uint8).offset((*d).m_out_buf_ofs as isize)
                    as *mut ::core::ffi::c_void,
                &raw mut (*d).m_output_buf as *mut mz_uint8 as *const ::core::ffi::c_void,
                bytes_to_copy as size_t,
            );
            (*d).m_out_buf_ofs = (*d).m_out_buf_ofs.wrapping_add(bytes_to_copy as size_t);
            n -= bytes_to_copy;
            if n != 0 as ::core::ffi::c_int {
                (*d).m_output_flush_ofs = bytes_to_copy as mz_uint;
                (*d).m_output_flush_remaining = n as mz_uint;
            }
        } else {
            (*d).m_out_buf_ofs = (*d).m_out_buf_ofs.wrapping_add(n as size_t);
        }
    }
    return (*d).m_output_flush_remaining as ::core::ffi::c_int;
}
#[inline(always)]
unsafe extern "C" fn tdefl_find_match(
    mut d: *mut tdefl_compressor,
    mut lookahead_pos: mz_uint,
    mut max_dist: mz_uint,
    mut max_match_len: mz_uint,
    mut pMatch_dist: *mut mz_uint,
    mut pMatch_len: *mut mz_uint,
) {
    let mut dist: mz_uint = 0;
    let mut pos: mz_uint = lookahead_pos & TDEFL_LZ_DICT_SIZE_MASK as ::core::ffi::c_int as mz_uint;
    let mut match_len: mz_uint = *pMatch_len;
    let mut probe_pos: mz_uint = pos;
    let mut next_probe_pos: mz_uint = 0;
    let mut probe_len: mz_uint = 0;
    let mut num_probes_left: mz_uint =
        (*d).m_max_probes[(match_len >= 32 as mz_uint) as ::core::ffi::c_int as usize];
    let mut s: *const mz_uint8 = (&raw mut (*d).m_dict as *mut mz_uint8).offset(pos as isize);
    let mut p: *const mz_uint8 = ::core::ptr::null::<mz_uint8>();
    let mut q: *const mz_uint8 = ::core::ptr::null::<mz_uint8>();
    let mut c0: mz_uint8 = (*d).m_dict[pos.wrapping_add(match_len) as usize];
    let mut c1: mz_uint8 =
        (*d).m_dict[pos.wrapping_add(match_len).wrapping_sub(1 as mz_uint) as usize];
    if max_match_len <= match_len {
        return;
    }
    loop {
        loop {
            num_probes_left = num_probes_left.wrapping_sub(1);
            if num_probes_left == 0 as mz_uint {
                return;
            }
            next_probe_pos = (*d).m_next[probe_pos as usize] as mz_uint;
            if next_probe_pos == 0 || {
                dist = lookahead_pos.wrapping_sub(next_probe_pos) as mz_uint16 as mz_uint;
                dist > max_dist
            } {
                return;
            }
            probe_pos = next_probe_pos & TDEFL_LZ_DICT_SIZE_MASK as ::core::ffi::c_int as mz_uint;
            if (*d).m_dict[probe_pos.wrapping_add(match_len) as usize] as ::core::ffi::c_int
                == c0 as ::core::ffi::c_int
                && (*d).m_dict
                    [probe_pos.wrapping_add(match_len).wrapping_sub(1 as mz_uint) as usize]
                    as ::core::ffi::c_int
                    == c1 as ::core::ffi::c_int
            {
                break;
            }
            next_probe_pos = (*d).m_next[probe_pos as usize] as mz_uint;
            if next_probe_pos == 0 || {
                dist = lookahead_pos.wrapping_sub(next_probe_pos) as mz_uint16 as mz_uint;
                dist > max_dist
            } {
                return;
            }
            probe_pos = next_probe_pos & TDEFL_LZ_DICT_SIZE_MASK as ::core::ffi::c_int as mz_uint;
            if (*d).m_dict[probe_pos.wrapping_add(match_len) as usize] as ::core::ffi::c_int
                == c0 as ::core::ffi::c_int
                && (*d).m_dict
                    [probe_pos.wrapping_add(match_len).wrapping_sub(1 as mz_uint) as usize]
                    as ::core::ffi::c_int
                    == c1 as ::core::ffi::c_int
            {
                break;
            }
            next_probe_pos = (*d).m_next[probe_pos as usize] as mz_uint;
            if next_probe_pos == 0 || {
                dist = lookahead_pos.wrapping_sub(next_probe_pos) as mz_uint16 as mz_uint;
                dist > max_dist
            } {
                return;
            }
            probe_pos = next_probe_pos & TDEFL_LZ_DICT_SIZE_MASK as ::core::ffi::c_int as mz_uint;
            if (*d).m_dict[probe_pos.wrapping_add(match_len) as usize] as ::core::ffi::c_int
                == c0 as ::core::ffi::c_int
                && (*d).m_dict
                    [probe_pos.wrapping_add(match_len).wrapping_sub(1 as mz_uint) as usize]
                    as ::core::ffi::c_int
                    == c1 as ::core::ffi::c_int
            {
                break;
            }
        }
        if dist == 0 {
            break;
        }
        p = s;
        q = (&raw mut (*d).m_dict as *mut mz_uint8).offset(probe_pos as isize);
        probe_len = 0 as mz_uint;
        while probe_len < max_match_len {
            let fresh85 = p;
            p = p.offset(1);
            let fresh86 = q;
            q = q.offset(1);
            if *fresh85 as ::core::ffi::c_int != *fresh86 as ::core::ffi::c_int {
                break;
            }
            probe_len = probe_len.wrapping_add(1);
        }
        if probe_len > match_len {
            *pMatch_dist = dist;
            match_len = probe_len;
            *pMatch_len = match_len;
            if *pMatch_len == max_match_len {
                return;
            }
            c0 = (*d).m_dict[pos.wrapping_add(match_len) as usize];
            c1 = (*d).m_dict[pos.wrapping_add(match_len).wrapping_sub(1 as mz_uint) as usize];
        }
    }
}
#[inline(always)]
unsafe extern "C" fn tdefl_record_literal(mut d: *mut tdefl_compressor, mut lit: mz_uint8) {
    (*d).m_total_lz_bytes = (*d).m_total_lz_bytes.wrapping_add(1);
    let fresh83 = (*d).m_pLZ_code_buf;
    (*d).m_pLZ_code_buf = (*d).m_pLZ_code_buf.offset(1);
    *fresh83 = lit;
    *(*d).m_pLZ_flags =
        (*(*d).m_pLZ_flags as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as mz_uint8;
    (*d).m_num_flags_left = (*d).m_num_flags_left.wrapping_sub(1);
    if (*d).m_num_flags_left == 0 as mz_uint {
        (*d).m_num_flags_left = 8 as mz_uint;
        let fresh84 = (*d).m_pLZ_code_buf;
        (*d).m_pLZ_code_buf = (*d).m_pLZ_code_buf.offset(1);
        (*d).m_pLZ_flags = fresh84;
    }
    (*d).m_huff_count[0 as ::core::ffi::c_int as usize][lit as usize] =
        (*d).m_huff_count[0 as ::core::ffi::c_int as usize][lit as usize].wrapping_add(1);
}
#[inline(always)]
unsafe extern "C" fn tdefl_record_match(
    mut d: *mut tdefl_compressor,
    mut match_len: mz_uint,
    mut match_dist: mz_uint,
) {
    let mut s0: mz_uint32 = 0;
    let mut s1: mz_uint32 = 0;
    (*d).m_total_lz_bytes = (*d).m_total_lz_bytes.wrapping_add(match_len);
    *(*d).m_pLZ_code_buf.offset(0 as ::core::ffi::c_int as isize) =
        match_len.wrapping_sub(TDEFL_MIN_MATCH_LEN as ::core::ffi::c_int as mz_uint) as mz_uint8;
    match_dist = match_dist.wrapping_sub(1 as mz_uint);
    *(*d).m_pLZ_code_buf.offset(1 as ::core::ffi::c_int as isize) =
        (match_dist & 0xff as mz_uint) as mz_uint8;
    *(*d).m_pLZ_code_buf.offset(2 as ::core::ffi::c_int as isize) =
        (match_dist >> 8 as ::core::ffi::c_int) as mz_uint8;
    (*d).m_pLZ_code_buf = (*d).m_pLZ_code_buf.offset(3 as ::core::ffi::c_int as isize);
    *(*d).m_pLZ_flags = (*(*d).m_pLZ_flags as ::core::ffi::c_int >> 1 as ::core::ffi::c_int
        | 0x80 as ::core::ffi::c_int) as mz_uint8;
    (*d).m_num_flags_left = (*d).m_num_flags_left.wrapping_sub(1);
    if (*d).m_num_flags_left == 0 as mz_uint {
        (*d).m_num_flags_left = 8 as mz_uint;
        let fresh82 = (*d).m_pLZ_code_buf;
        (*d).m_pLZ_code_buf = (*d).m_pLZ_code_buf.offset(1);
        (*d).m_pLZ_flags = fresh82;
    }
    s0 = s_tdefl_small_dist_sym[(match_dist & 511 as mz_uint) as usize] as mz_uint32;
    s1 = s_tdefl_large_dist_sym[(match_dist >> 8 as ::core::ffi::c_int & 127 as mz_uint) as usize]
        as mz_uint32;
    (*d).m_huff_count[1 as ::core::ffi::c_int as usize]
        [(if match_dist < 512 as mz_uint { s0 } else { s1 }) as usize] = (*d).m_huff_count
        [1 as ::core::ffi::c_int as usize]
        [(if match_dist < 512 as mz_uint { s0 } else { s1 }) as usize]
        .wrapping_add(1);
    (*d).m_huff_count[0 as ::core::ffi::c_int as usize][s_tdefl_len_sym
        [match_len.wrapping_sub(TDEFL_MIN_MATCH_LEN as ::core::ffi::c_int as mz_uint) as usize]
        as usize] = (*d).m_huff_count[0 as ::core::ffi::c_int as usize][s_tdefl_len_sym
        [match_len.wrapping_sub(TDEFL_MIN_MATCH_LEN as ::core::ffi::c_int as mz_uint) as usize]
        as usize]
        .wrapping_add(1);
}
unsafe extern "C" fn tdefl_compress_normal(mut d: *mut tdefl_compressor) -> mz_bool {
    let mut pSrc: *const mz_uint8 = (*d).m_pSrc;
    let mut src_buf_left: size_t = (*d).m_src_buf_left;
    let mut flush: tdefl_flush = (*d).m_flush;
    while src_buf_left != 0 || flush as ::core::ffi::c_uint != 0 && (*d).m_lookahead_size != 0 {
        let mut len_to_move: mz_uint = 0;
        let mut cur_match_dist: mz_uint = 0;
        let mut cur_match_len: mz_uint = 0;
        let mut cur_pos: mz_uint = 0;
        if (*d).m_lookahead_size.wrapping_add((*d).m_dict_size)
            >= (TDEFL_MIN_MATCH_LEN as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as mz_uint
        {
            let mut dst_pos: mz_uint = (*d).m_lookahead_pos.wrapping_add((*d).m_lookahead_size)
                & TDEFL_LZ_DICT_SIZE_MASK as ::core::ffi::c_int as mz_uint;
            let mut ins_pos: mz_uint = (*d)
                .m_lookahead_pos
                .wrapping_add((*d).m_lookahead_size)
                .wrapping_sub(2 as mz_uint);
            let mut hash: mz_uint = (((*d).m_dict
                [(ins_pos & TDEFL_LZ_DICT_SIZE_MASK as ::core::ffi::c_int as mz_uint) as usize]
                as ::core::ffi::c_int)
                << TDEFL_LZ_HASH_SHIFT as ::core::ffi::c_int
                ^ (*d).m_dict[(ins_pos.wrapping_add(1 as mz_uint)
                    & TDEFL_LZ_DICT_SIZE_MASK as ::core::ffi::c_int as mz_uint)
                    as usize] as ::core::ffi::c_int) as mz_uint;
            let mut num_bytes_to_process: mz_uint = (if src_buf_left
                < (TDEFL_MAX_MATCH_LEN as ::core::ffi::c_int as mz_uint)
                    .wrapping_sub((*d).m_lookahead_size) as size_t
            {
                src_buf_left
            } else {
                (TDEFL_MAX_MATCH_LEN as ::core::ffi::c_int as mz_uint)
                    .wrapping_sub((*d).m_lookahead_size) as size_t
            }) as mz_uint;
            let mut pSrc_end: *const mz_uint8 = if !pSrc.is_null() {
                pSrc.offset(num_bytes_to_process as isize)
            } else {
                ::core::ptr::null::<mz_uint8>()
            };
            src_buf_left = src_buf_left.wrapping_sub(num_bytes_to_process as size_t);
            (*d).m_lookahead_size = (*d).m_lookahead_size.wrapping_add(num_bytes_to_process);
            while pSrc != pSrc_end {
                let fresh80 = pSrc;
                pSrc = pSrc.offset(1);
                let mut c: mz_uint8 = *fresh80;
                (*d).m_dict[dst_pos as usize] = c;
                if dst_pos
                    < (TDEFL_MAX_MATCH_LEN as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                        as mz_uint
                {
                    (*d).m_dict[(TDEFL_LZ_DICT_SIZE as ::core::ffi::c_int as mz_uint)
                        .wrapping_add(dst_pos) as usize] = c;
                }
                hash = (hash << TDEFL_LZ_HASH_SHIFT as ::core::ffi::c_int ^ c as mz_uint)
                    & (TDEFL_LZ_HASH_SIZE as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                        as mz_uint;
                (*d).m_next[(ins_pos & TDEFL_LZ_DICT_SIZE_MASK as ::core::ffi::c_int as mz_uint)
                    as usize] = (*d).m_hash[hash as usize];
                (*d).m_hash[hash as usize] = ins_pos as mz_uint16;
                dst_pos = dst_pos.wrapping_add(1 as mz_uint)
                    & TDEFL_LZ_DICT_SIZE_MASK as ::core::ffi::c_int as mz_uint;
                ins_pos = ins_pos.wrapping_add(1);
            }
        } else {
            while src_buf_left != 0
                && (*d).m_lookahead_size < TDEFL_MAX_MATCH_LEN as ::core::ffi::c_int as mz_uint
            {
                let fresh81 = pSrc;
                pSrc = pSrc.offset(1);
                let mut c_0: mz_uint8 = *fresh81;
                let mut dst_pos_0: mz_uint =
                    (*d).m_lookahead_pos.wrapping_add((*d).m_lookahead_size)
                        & TDEFL_LZ_DICT_SIZE_MASK as ::core::ffi::c_int as mz_uint;
                src_buf_left = src_buf_left.wrapping_sub(1);
                (*d).m_dict[dst_pos_0 as usize] = c_0;
                if dst_pos_0
                    < (TDEFL_MAX_MATCH_LEN as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                        as mz_uint
                {
                    (*d).m_dict[(TDEFL_LZ_DICT_SIZE as ::core::ffi::c_int as mz_uint)
                        .wrapping_add(dst_pos_0) as usize] = c_0;
                }
                (*d).m_lookahead_size = (*d).m_lookahead_size.wrapping_add(1);
                if (*d).m_lookahead_size.wrapping_add((*d).m_dict_size)
                    >= TDEFL_MIN_MATCH_LEN as ::core::ffi::c_int as mz_uint
                {
                    let mut ins_pos_0: mz_uint = (*d)
                        .m_lookahead_pos
                        .wrapping_add((*d).m_lookahead_size.wrapping_sub(1 as mz_uint))
                        .wrapping_sub(2 as mz_uint);
                    let mut hash_0: mz_uint = ((((*d).m_dict[(ins_pos_0
                        & TDEFL_LZ_DICT_SIZE_MASK as ::core::ffi::c_int as mz_uint)
                        as usize]
                        as ::core::ffi::c_int)
                        << TDEFL_LZ_HASH_SHIFT as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                        ^ ((*d).m_dict[(ins_pos_0.wrapping_add(1 as mz_uint)
                            & TDEFL_LZ_DICT_SIZE_MASK as ::core::ffi::c_int as mz_uint)
                            as usize] as ::core::ffi::c_int)
                            << TDEFL_LZ_HASH_SHIFT as ::core::ffi::c_int
                        ^ c_0 as ::core::ffi::c_int)
                        & TDEFL_LZ_HASH_SIZE as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                        as mz_uint;
                    (*d).m_next[(ins_pos_0
                        & TDEFL_LZ_DICT_SIZE_MASK as ::core::ffi::c_int as mz_uint)
                        as usize] = (*d).m_hash[hash_0 as usize];
                    (*d).m_hash[hash_0 as usize] = ins_pos_0 as mz_uint16;
                }
            }
        }
        (*d).m_dict_size = if (TDEFL_LZ_DICT_SIZE as ::core::ffi::c_int as mz_uint)
            .wrapping_sub((*d).m_lookahead_size)
            < (*d).m_dict_size
        {
            (TDEFL_LZ_DICT_SIZE as ::core::ffi::c_int as mz_uint)
                .wrapping_sub((*d).m_lookahead_size)
        } else {
            (*d).m_dict_size
        };
        if flush as u64 == 0
            && (*d).m_lookahead_size < TDEFL_MAX_MATCH_LEN as ::core::ffi::c_int as mz_uint
        {
            break;
        }
        len_to_move = 1 as mz_uint;
        cur_match_dist = 0 as mz_uint;
        cur_match_len = if (*d).m_saved_match_len != 0 {
            (*d).m_saved_match_len
        } else {
            (TDEFL_MIN_MATCH_LEN as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as mz_uint
        };
        cur_pos = (*d).m_lookahead_pos & TDEFL_LZ_DICT_SIZE_MASK as ::core::ffi::c_int as mz_uint;
        if (*d).m_flags
            & (TDEFL_RLE_MATCHES as ::core::ffi::c_int
                | TDEFL_FORCE_ALL_RAW_BLOCKS as ::core::ffi::c_int) as mz_uint
            != 0
        {
            if (*d).m_dict_size != 0
                && (*d).m_flags & TDEFL_FORCE_ALL_RAW_BLOCKS as ::core::ffi::c_int as mz_uint == 0
            {
                let mut c_1: mz_uint8 = (*d).m_dict[(cur_pos.wrapping_sub(1 as mz_uint)
                    & TDEFL_LZ_DICT_SIZE_MASK as ::core::ffi::c_int as mz_uint)
                    as usize];
                cur_match_len = 0 as mz_uint;
                while cur_match_len < (*d).m_lookahead_size {
                    if (*d).m_dict[cur_pos.wrapping_add(cur_match_len) as usize]
                        as ::core::ffi::c_int
                        != c_1 as ::core::ffi::c_int
                    {
                        break;
                    }
                    cur_match_len = cur_match_len.wrapping_add(1);
                }
                if cur_match_len < TDEFL_MIN_MATCH_LEN as ::core::ffi::c_int as mz_uint {
                    cur_match_len = 0 as mz_uint;
                } else {
                    cur_match_dist = 1 as mz_uint;
                }
            }
        } else {
            tdefl_find_match(
                d,
                (*d).m_lookahead_pos,
                (*d).m_dict_size,
                (*d).m_lookahead_size,
                &raw mut cur_match_dist,
                &raw mut cur_match_len,
            );
        }
        if cur_match_len == TDEFL_MIN_MATCH_LEN as ::core::ffi::c_int as mz_uint
            && cur_match_dist >= (8 as mz_uint).wrapping_mul(1024 as mz_uint)
            || cur_pos == cur_match_dist
            || (*d).m_flags & TDEFL_FILTER_MATCHES as ::core::ffi::c_int as mz_uint != 0
                && cur_match_len <= 5 as mz_uint
        {
            cur_match_len = 0 as mz_uint;
            cur_match_dist = cur_match_len;
        }
        if (*d).m_saved_match_len != 0 {
            if cur_match_len > (*d).m_saved_match_len {
                tdefl_record_literal(d, (*d).m_saved_lit as mz_uint8);
                if cur_match_len >= 128 as mz_uint {
                    tdefl_record_match(d, cur_match_len, cur_match_dist);
                    (*d).m_saved_match_len = 0 as mz_uint;
                    len_to_move = cur_match_len;
                } else {
                    (*d).m_saved_lit = (*d).m_dict[cur_pos as usize] as mz_uint;
                    (*d).m_saved_match_dist = cur_match_dist;
                    (*d).m_saved_match_len = cur_match_len;
                }
            } else {
                tdefl_record_match(d, (*d).m_saved_match_len, (*d).m_saved_match_dist);
                len_to_move = (*d).m_saved_match_len.wrapping_sub(1 as mz_uint);
                (*d).m_saved_match_len = 0 as mz_uint;
            }
        } else if cur_match_dist == 0 {
            tdefl_record_literal(
                d,
                (*d).m_dict[(if (cur_pos as usize)
                    < (::core::mem::size_of::<[mz_uint8; 33025]>() as usize)
                        .wrapping_sub(1 as usize)
                {
                    cur_pos as usize
                } else {
                    (::core::mem::size_of::<[mz_uint8; 33025]>() as usize).wrapping_sub(1 as usize)
                }) as usize],
            );
        } else if (*d).m_greedy_parsing != 0
            || (*d).m_flags & TDEFL_RLE_MATCHES as ::core::ffi::c_int as mz_uint != 0
            || cur_match_len >= 128 as mz_uint
        {
            tdefl_record_match(d, cur_match_len, cur_match_dist);
            len_to_move = cur_match_len;
        } else {
            (*d).m_saved_lit = (*d).m_dict[(if (cur_pos as usize)
                < (::core::mem::size_of::<[mz_uint8; 33025]>() as usize).wrapping_sub(1 as usize)
            {
                cur_pos as usize
            } else {
                (::core::mem::size_of::<[mz_uint8; 33025]>() as usize).wrapping_sub(1 as usize)
            }) as usize] as mz_uint;
            (*d).m_saved_match_dist = cur_match_dist;
            (*d).m_saved_match_len = cur_match_len;
        }
        (*d).m_lookahead_pos = (*d).m_lookahead_pos.wrapping_add(len_to_move);
        (*d).m_lookahead_size = (*d).m_lookahead_size.wrapping_sub(len_to_move);
        (*d).m_dict_size = if (*d).m_dict_size.wrapping_add(len_to_move)
            < TDEFL_LZ_DICT_SIZE as ::core::ffi::c_int as mz_uint
        {
            (*d).m_dict_size.wrapping_add(len_to_move)
        } else {
            TDEFL_LZ_DICT_SIZE as ::core::ffi::c_int as mz_uint
        };
        if (*d).m_pLZ_code_buf
            > (&raw mut (*d).m_lz_code_buf as *mut mz_uint8).offset(
                (TDEFL_LZ_CODE_BUF_SIZE as ::core::ffi::c_int - 8 as ::core::ffi::c_int) as isize,
            ) as *mut mz_uint8
            || (*d).m_total_lz_bytes
                > (31 as ::core::ffi::c_int * 1024 as ::core::ffi::c_int) as mz_uint
                && (((*d)
                    .m_pLZ_code_buf
                    .offset_from(&raw mut (*d).m_lz_code_buf as *mut mz_uint8)
                    as ::core::ffi::c_long as mz_uint)
                    .wrapping_mul(115 as mz_uint)
                    >> 7 as ::core::ffi::c_int
                    >= (*d).m_total_lz_bytes
                    || (*d).m_flags & TDEFL_FORCE_ALL_RAW_BLOCKS as ::core::ffi::c_int as mz_uint
                        != 0)
        {
            let mut n: ::core::ffi::c_int = 0;
            (*d).m_pSrc = pSrc;
            (*d).m_src_buf_left = src_buf_left;
            n = tdefl_flush_block(d, 0 as ::core::ffi::c_int);
            if n != 0 as ::core::ffi::c_int {
                return if n < 0 as ::core::ffi::c_int {
                    MZ_FALSE
                } else {
                    MZ_TRUE
                };
            }
        }
    }
    (*d).m_pSrc = pSrc;
    (*d).m_src_buf_left = src_buf_left;
    return MZ_TRUE;
}
unsafe extern "C" fn tdefl_flush_output_buffer(mut d: *mut tdefl_compressor) -> tdefl_status {
    if !(*d).m_pIn_buf_size.is_null() {
        *(*d).m_pIn_buf_size = (*d).m_pSrc.offset_from((*d).m_pIn_buf as *const mz_uint8)
            as ::core::ffi::c_long as size_t;
    }
    if !(*d).m_pOut_buf_size.is_null() {
        let mut n: size_t = if (*(*d).m_pOut_buf_size).wrapping_sub((*d).m_out_buf_ofs)
            < (*d).m_output_flush_remaining as size_t
        {
            (*(*d).m_pOut_buf_size).wrapping_sub((*d).m_out_buf_ofs)
        } else {
            (*d).m_output_flush_remaining as size_t
        };
        memcpy(
            ((*d).m_pOut_buf as *mut mz_uint8).offset((*d).m_out_buf_ofs as isize)
                as *mut ::core::ffi::c_void,
            (&raw mut (*d).m_output_buf as *mut mz_uint8).offset((*d).m_output_flush_ofs as isize)
                as *const ::core::ffi::c_void,
            n,
        );
        (*d).m_output_flush_ofs = (*d).m_output_flush_ofs.wrapping_add(n as mz_uint);
        (*d).m_output_flush_remaining = (*d).m_output_flush_remaining.wrapping_sub(n as mz_uint);
        (*d).m_out_buf_ofs = (*d).m_out_buf_ofs.wrapping_add(n);
        *(*d).m_pOut_buf_size = (*d).m_out_buf_ofs;
    }
    return (if (*d).m_finished != 0 && (*d).m_output_flush_remaining == 0 {
        TDEFL_STATUS_DONE as ::core::ffi::c_int
    } else {
        TDEFL_STATUS_OKAY as ::core::ffi::c_int
    }) as tdefl_status;
}
#[no_mangle]
pub unsafe extern "C" fn tdefl_compress(
    mut d: *mut tdefl_compressor,
    mut pIn_buf: *const ::core::ffi::c_void,
    mut pIn_buf_size: *mut size_t,
    mut pOut_buf: *mut ::core::ffi::c_void,
    mut pOut_buf_size: *mut size_t,
    mut flush: tdefl_flush,
) -> tdefl_status {
    if d.is_null() {
        if !pIn_buf_size.is_null() {
            *pIn_buf_size = 0 as size_t;
        }
        if !pOut_buf_size.is_null() {
            *pOut_buf_size = 0 as size_t;
        }
        return TDEFL_STATUS_BAD_PARAM;
    }
    (*d).m_pIn_buf = pIn_buf;
    (*d).m_pIn_buf_size = pIn_buf_size;
    (*d).m_pOut_buf = pOut_buf;
    (*d).m_pOut_buf_size = pOut_buf_size;
    (*d).m_pSrc = pIn_buf as *const mz_uint8;
    (*d).m_src_buf_left = if !pIn_buf_size.is_null() {
        *pIn_buf_size
    } else {
        0 as size_t
    };
    (*d).m_out_buf_ofs = 0 as size_t;
    (*d).m_flush = flush;
    if (*d).m_pPut_buf_func.is_some() as ::core::ffi::c_int
        == (!pOut_buf.is_null() || !pOut_buf_size.is_null()) as ::core::ffi::c_int
        || (*d).m_prev_return_status as ::core::ffi::c_int
            != TDEFL_STATUS_OKAY as ::core::ffi::c_int
        || (*d).m_wants_to_finish != 0
            && flush as ::core::ffi::c_uint
                != TDEFL_FINISH as ::core::ffi::c_int as ::core::ffi::c_uint
        || !pIn_buf_size.is_null() && *pIn_buf_size != 0 && pIn_buf.is_null()
        || !pOut_buf_size.is_null() && *pOut_buf_size != 0 && pOut_buf.is_null()
    {
        if !pIn_buf_size.is_null() {
            *pIn_buf_size = 0 as size_t;
        }
        if !pOut_buf_size.is_null() {
            *pOut_buf_size = 0 as size_t;
        }
        (*d).m_prev_return_status = TDEFL_STATUS_BAD_PARAM;
        return (*d).m_prev_return_status;
    }
    (*d).m_wants_to_finish |= (flush as ::core::ffi::c_uint
        == TDEFL_FINISH as ::core::ffi::c_int as ::core::ffi::c_uint)
        as ::core::ffi::c_int as mz_uint;
    if (*d).m_output_flush_remaining != 0 || (*d).m_finished != 0 {
        (*d).m_prev_return_status = tdefl_flush_output_buffer(d);
        return (*d).m_prev_return_status;
    }
    if tdefl_compress_normal(d) == 0 {
        return (*d).m_prev_return_status;
    }
    if (*d).m_flags
        & (TDEFL_WRITE_ZLIB_HEADER as ::core::ffi::c_int
            | TDEFL_COMPUTE_ADLER32 as ::core::ffi::c_int) as mz_uint
        != 0
        && !pIn_buf.is_null()
    {
        (*d).m_adler32 = mz_adler32(
            (*d).m_adler32 as mz_ulong,
            pIn_buf as *const ::core::ffi::c_uchar,
            (*d).m_pSrc.offset_from(pIn_buf as *const mz_uint8) as ::core::ffi::c_long as size_t,
        ) as mz_uint32 as mz_uint;
    }
    if flush as ::core::ffi::c_uint != 0
        && (*d).m_lookahead_size == 0
        && (*d).m_src_buf_left == 0
        && (*d).m_output_flush_remaining == 0
    {
        if tdefl_flush_block(d, flush as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
            return (*d).m_prev_return_status;
        }
        (*d).m_finished = (flush as ::core::ffi::c_uint
            == TDEFL_FINISH as ::core::ffi::c_int as ::core::ffi::c_uint)
            as ::core::ffi::c_int as mz_uint;
        if flush as ::core::ffi::c_uint
            == TDEFL_FULL_FLUSH as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            memset(
                &raw mut (*d).m_hash as *mut mz_uint16 as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<[mz_uint16; 32768]>() as size_t,
            );
            memset(
                &raw mut (*d).m_next as *mut mz_uint16 as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<[mz_uint16; 32768]>() as size_t,
            );
            (*d).m_dict_size = 0 as mz_uint;
        }
    }
    (*d).m_prev_return_status = tdefl_flush_output_buffer(d);
    return (*d).m_prev_return_status;
}
#[no_mangle]
pub unsafe extern "C" fn tdefl_compress_buffer(
    mut d: *mut tdefl_compressor,
    mut pIn_buf: *const ::core::ffi::c_void,
    mut in_buf_size: size_t,
    mut flush: tdefl_flush,
) -> tdefl_status {
    return tdefl_compress(
        d,
        pIn_buf,
        &raw mut in_buf_size,
        NULL,
        ::core::ptr::null_mut::<size_t>(),
        flush,
    );
}
#[no_mangle]
pub unsafe extern "C" fn tdefl_init(
    mut d: *mut tdefl_compressor,
    mut pPut_buf_func: tdefl_put_buf_func_ptr,
    mut pPut_buf_user: *mut ::core::ffi::c_void,
    mut flags: ::core::ffi::c_int,
) -> tdefl_status {
    (*d).m_pPut_buf_func = pPut_buf_func;
    (*d).m_pPut_buf_user = pPut_buf_user;
    (*d).m_flags = flags as mz_uint;
    (*d).m_max_probes[0 as ::core::ffi::c_int as usize] = (1 as ::core::ffi::c_int
        + ((flags & 0xfff as ::core::ffi::c_int) + 2 as ::core::ffi::c_int)
            / 3 as ::core::ffi::c_int)
        as mz_uint;
    (*d).m_greedy_parsing = (flags & TDEFL_GREEDY_PARSING_FLAG as ::core::ffi::c_int
        != 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
    (*d).m_max_probes[1 as ::core::ffi::c_int as usize] = (1 as ::core::ffi::c_int
        + (((flags & 0xfff as ::core::ffi::c_int) >> 2 as ::core::ffi::c_int)
            + 2 as ::core::ffi::c_int)
            / 3 as ::core::ffi::c_int)
        as mz_uint;
    if flags & TDEFL_NONDETERMINISTIC_PARSING_FLAG as ::core::ffi::c_int == 0 {
        memset(
            &raw mut (*d).m_hash as *mut mz_uint16 as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<[mz_uint16; 32768]>() as size_t,
        );
    }
    (*d).m_bits_in = 0 as mz_uint;
    (*d).m_lz_code_buf_dict_pos = (*d).m_bits_in;
    (*d).m_total_lz_bytes = (*d).m_lz_code_buf_dict_pos;
    (*d).m_dict_size = (*d).m_total_lz_bytes;
    (*d).m_lookahead_size = (*d).m_dict_size;
    (*d).m_lookahead_pos = (*d).m_lookahead_size;
    (*d).m_wants_to_finish = 0 as mz_uint;
    (*d).m_bit_buffer = (*d).m_wants_to_finish;
    (*d).m_block_index = (*d).m_bit_buffer;
    (*d).m_finished = (*d).m_block_index;
    (*d).m_output_flush_remaining = (*d).m_finished;
    (*d).m_output_flush_ofs = (*d).m_output_flush_remaining;
    (*d).m_pLZ_code_buf =
        (&raw mut (*d).m_lz_code_buf as *mut mz_uint8).offset(1 as ::core::ffi::c_int as isize);
    (*d).m_pLZ_flags = &raw mut (*d).m_lz_code_buf as *mut mz_uint8;
    *(*d).m_pLZ_flags = 0 as mz_uint8;
    (*d).m_num_flags_left = 8 as mz_uint;
    (*d).m_pOutput_buf = &raw mut (*d).m_output_buf as *mut mz_uint8;
    (*d).m_pOutput_buf_end = &raw mut (*d).m_output_buf as *mut mz_uint8;
    (*d).m_prev_return_status = TDEFL_STATUS_OKAY;
    (*d).m_saved_lit = 0 as mz_uint;
    (*d).m_saved_match_len = (*d).m_saved_lit;
    (*d).m_saved_match_dist = (*d).m_saved_match_len;
    (*d).m_adler32 = 1 as mz_uint;
    (*d).m_pIn_buf = ::core::ptr::null::<::core::ffi::c_void>();
    (*d).m_pOut_buf = NULL;
    (*d).m_pIn_buf_size = ::core::ptr::null_mut::<size_t>();
    (*d).m_pOut_buf_size = ::core::ptr::null_mut::<size_t>();
    (*d).m_flush = TDEFL_NO_FLUSH;
    (*d).m_pSrc = ::core::ptr::null::<mz_uint8>();
    (*d).m_src_buf_left = 0 as size_t;
    (*d).m_out_buf_ofs = 0 as size_t;
    if flags & TDEFL_NONDETERMINISTIC_PARSING_FLAG as ::core::ffi::c_int == 0 {
        memset(
            &raw mut (*d).m_dict as *mut mz_uint8 as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<[mz_uint8; 33025]>() as size_t,
        );
    }
    memset(
        (&raw mut *(&raw mut (*d).m_huff_count as *mut [mz_uint16; 288])
            .offset(0 as ::core::ffi::c_int as isize) as *mut mz_uint16)
            .offset(0 as ::core::ffi::c_int as isize) as *mut mz_uint16
            as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        (::core::mem::size_of::<mz_uint16>() as size_t)
            .wrapping_mul(TDEFL_MAX_HUFF_SYMBOLS_0 as ::core::ffi::c_int as size_t),
    );
    memset(
        (&raw mut *(&raw mut (*d).m_huff_count as *mut [mz_uint16; 288])
            .offset(1 as ::core::ffi::c_int as isize) as *mut mz_uint16)
            .offset(0 as ::core::ffi::c_int as isize) as *mut mz_uint16
            as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        (::core::mem::size_of::<mz_uint16>() as size_t)
            .wrapping_mul(TDEFL_MAX_HUFF_SYMBOLS_1 as ::core::ffi::c_int as size_t),
    );
    return TDEFL_STATUS_OKAY;
}
#[no_mangle]
pub unsafe extern "C" fn tdefl_get_prev_return_status(
    mut d: *mut tdefl_compressor,
) -> tdefl_status {
    return (*d).m_prev_return_status;
}
#[no_mangle]
pub unsafe extern "C" fn tdefl_get_adler32(mut d: *mut tdefl_compressor) -> mz_uint32 {
    return (*d).m_adler32 as mz_uint32;
}
#[no_mangle]
pub unsafe extern "C" fn tdefl_compress_mem_to_output(
    mut pBuf: *const ::core::ffi::c_void,
    mut buf_len: size_t,
    mut pPut_buf_func: tdefl_put_buf_func_ptr,
    mut pPut_buf_user: *mut ::core::ffi::c_void,
    mut flags: ::core::ffi::c_int,
) -> mz_bool {
    let mut pComp: *mut tdefl_compressor = ::core::ptr::null_mut::<tdefl_compressor>();
    let mut succeeded: mz_bool = 0;
    if buf_len != 0 && pBuf.is_null() || pPut_buf_func.is_none() {
        return MZ_FALSE;
    }
    pComp = malloc(::core::mem::size_of::<tdefl_compressor>() as size_t) as *mut tdefl_compressor;
    if pComp.is_null() {
        return MZ_FALSE;
    }
    succeeded = (tdefl_init(pComp, pPut_buf_func, pPut_buf_user, flags) as ::core::ffi::c_int
        == TDEFL_STATUS_OKAY as ::core::ffi::c_int) as ::core::ffi::c_int
        as mz_bool;
    succeeded = (succeeded != 0
        && tdefl_compress_buffer(pComp, pBuf, buf_len, TDEFL_FINISH) as ::core::ffi::c_int
            == TDEFL_STATUS_DONE as ::core::ffi::c_int) as ::core::ffi::c_int
        as mz_bool;
    free(pComp as *mut ::core::ffi::c_void);
    return succeeded;
}
unsafe extern "C" fn tdefl_output_buffer_putter(
    mut pBuf: *const ::core::ffi::c_void,
    mut len: ::core::ffi::c_int,
    mut pUser: *mut ::core::ffi::c_void,
) -> mz_bool {
    let mut p: *mut tdefl_output_buffer = pUser as *mut tdefl_output_buffer;
    let mut new_size: size_t = (*p).m_size.wrapping_add(len as size_t);
    if new_size > (*p).m_capacity {
        let mut new_capacity: size_t = (*p).m_capacity;
        let mut pNew_buf: *mut mz_uint8 = ::core::ptr::null_mut::<mz_uint8>();
        if (*p).m_expandable == 0 {
            return MZ_FALSE;
        }
        loop {
            new_capacity = if 128 as size_t > new_capacity << 1 as ::core::ffi::c_uint {
                128 as size_t
            } else {
                new_capacity << 1 as ::core::ffi::c_uint
            };
            if !(new_size > new_capacity) {
                break;
            }
        }
        pNew_buf = realloc((*p).m_pBuf as *mut ::core::ffi::c_void, new_capacity) as *mut mz_uint8;
        if pNew_buf.is_null() {
            return MZ_FALSE;
        }
        (*p).m_pBuf = pNew_buf;
        (*p).m_capacity = new_capacity;
    }
    memcpy(
        (*p).m_pBuf.offset((*p).m_size as isize) as *mut ::core::ffi::c_void,
        pBuf,
        len as size_t,
    );
    (*p).m_size = new_size;
    return MZ_TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn tdefl_compress_mem_to_heap(
    mut pSrc_buf: *const ::core::ffi::c_void,
    mut src_buf_len: size_t,
    mut pOut_len: *mut size_t,
    mut flags: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_void {
    let mut out_buf: tdefl_output_buffer = tdefl_output_buffer {
        m_size: 0,
        m_capacity: 0,
        m_pBuf: ::core::ptr::null_mut::<mz_uint8>(),
        m_expandable: 0,
    };
    memset(
        &raw mut out_buf as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<tdefl_output_buffer>() as size_t,
    );
    if pOut_len.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    } else {
        *pOut_len = 0 as size_t;
    }
    out_buf.m_expandable = MZ_TRUE as mz_bool;
    if tdefl_compress_mem_to_output(
        pSrc_buf,
        src_buf_len,
        Some(
            tdefl_output_buffer_putter
                as unsafe extern "C" fn(
                    *const ::core::ffi::c_void,
                    ::core::ffi::c_int,
                    *mut ::core::ffi::c_void,
                ) -> mz_bool,
        ),
        &raw mut out_buf as *mut ::core::ffi::c_void,
        flags,
    ) == 0
    {
        return NULL;
    }
    *pOut_len = out_buf.m_size;
    return out_buf.m_pBuf as *mut ::core::ffi::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn tdefl_compress_mem_to_mem(
    mut pOut_buf: *mut ::core::ffi::c_void,
    mut out_buf_len: size_t,
    mut pSrc_buf: *const ::core::ffi::c_void,
    mut src_buf_len: size_t,
    mut flags: ::core::ffi::c_int,
) -> size_t {
    let mut out_buf: tdefl_output_buffer = tdefl_output_buffer {
        m_size: 0,
        m_capacity: 0,
        m_pBuf: ::core::ptr::null_mut::<mz_uint8>(),
        m_expandable: 0,
    };
    memset(
        &raw mut out_buf as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<tdefl_output_buffer>() as size_t,
    );
    if pOut_buf.is_null() {
        return 0 as size_t;
    }
    out_buf.m_pBuf = pOut_buf as *mut mz_uint8;
    out_buf.m_capacity = out_buf_len;
    if tdefl_compress_mem_to_output(
        pSrc_buf,
        src_buf_len,
        Some(
            tdefl_output_buffer_putter
                as unsafe extern "C" fn(
                    *const ::core::ffi::c_void,
                    ::core::ffi::c_int,
                    *mut ::core::ffi::c_void,
                ) -> mz_bool,
        ),
        &raw mut out_buf as *mut ::core::ffi::c_void,
        flags,
    ) == 0
    {
        return 0 as size_t;
    }
    return out_buf.m_size;
}
#[no_mangle]
pub unsafe extern "C" fn tdefl_create_comp_flags_from_zip_params(
    mut level: ::core::ffi::c_int,
    mut window_bits: ::core::ffi::c_int,
    mut strategy: ::core::ffi::c_int,
) -> mz_uint {
    let mut comp_flags: mz_uint = s_tdefl_num_probes[(if level >= 0 as ::core::ffi::c_int {
        (if (10 as ::core::ffi::c_int) < level {
            10 as ::core::ffi::c_int
        } else {
            level
        })
    } else {
        MZ_DEFAULT_LEVEL as ::core::ffi::c_int
    }) as usize]
        | (if level <= 3 as ::core::ffi::c_int {
            TDEFL_GREEDY_PARSING_FLAG as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as mz_uint;
    if window_bits > 0 as ::core::ffi::c_int {
        comp_flags |= TDEFL_WRITE_ZLIB_HEADER as ::core::ffi::c_int as mz_uint;
    }
    if level == 0 {
        comp_flags |= TDEFL_FORCE_ALL_RAW_BLOCKS as ::core::ffi::c_int as mz_uint;
    } else if strategy == MZ_FILTERED as ::core::ffi::c_int {
        comp_flags |= TDEFL_FILTER_MATCHES as ::core::ffi::c_int as mz_uint;
    } else if strategy == MZ_HUFFMAN_ONLY as ::core::ffi::c_int {
        comp_flags &= !(TDEFL_MAX_PROBES_MASK as ::core::ffi::c_int) as mz_uint;
    } else if strategy == MZ_FIXED as ::core::ffi::c_int {
        comp_flags |= TDEFL_FORCE_ALL_STATIC_BLOCKS as ::core::ffi::c_int as mz_uint;
    } else if strategy == MZ_RLE as ::core::ffi::c_int {
        comp_flags |= TDEFL_RLE_MATCHES as ::core::ffi::c_int as mz_uint;
    }
    return comp_flags;
}
#[no_mangle]
pub unsafe extern "C" fn tdefl_write_image_to_png_file_in_memory_ex(
    mut pImage: *const ::core::ffi::c_void,
    mut w: ::core::ffi::c_int,
    mut h: ::core::ffi::c_int,
    mut num_chans: ::core::ffi::c_int,
    mut pLen_out: *mut size_t,
    mut level: mz_uint,
    mut flip: mz_bool,
) -> *mut ::core::ffi::c_void {
    static mut s_tdefl_png_num_probes: [mz_uint; 11] = [
        0 as ::core::ffi::c_int as mz_uint,
        1 as ::core::ffi::c_int as mz_uint,
        6 as ::core::ffi::c_int as mz_uint,
        32 as ::core::ffi::c_int as mz_uint,
        16 as ::core::ffi::c_int as mz_uint,
        32 as ::core::ffi::c_int as mz_uint,
        128 as ::core::ffi::c_int as mz_uint,
        256 as ::core::ffi::c_int as mz_uint,
        512 as ::core::ffi::c_int as mz_uint,
        768 as ::core::ffi::c_int as mz_uint,
        1500 as ::core::ffi::c_int as mz_uint,
    ];
    let mut pComp: *mut tdefl_compressor =
        malloc(::core::mem::size_of::<tdefl_compressor>() as size_t) as *mut tdefl_compressor;
    let mut out_buf: tdefl_output_buffer = tdefl_output_buffer {
        m_size: 0,
        m_capacity: 0,
        m_pBuf: ::core::ptr::null_mut::<mz_uint8>(),
        m_expandable: 0,
    };
    let mut i: ::core::ffi::c_int = 0;
    let mut bpl: ::core::ffi::c_int = w * num_chans;
    let mut y: ::core::ffi::c_int = 0;
    let mut z: ::core::ffi::c_int = 0;
    let mut c: mz_uint32 = 0;
    *pLen_out = 0 as size_t;
    if pComp.is_null() {
        return NULL;
    }
    if w <= 0 as ::core::ffi::c_int
        || h <= 0 as ::core::ffi::c_int
        || w > 0xffff as ::core::ffi::c_int
        || h > 0xffff as ::core::ffi::c_int
        || num_chans < 1 as ::core::ffi::c_int
        || num_chans > 4 as ::core::ffi::c_int
    {
        free(pComp as *mut ::core::ffi::c_void);
        return NULL;
    }
    memset(
        &raw mut out_buf as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<tdefl_output_buffer>() as size_t,
    );
    out_buf.m_expandable = MZ_TRUE as mz_bool;
    out_buf.m_capacity = (57 as ::core::ffi::c_int
        + (if 64 as ::core::ffi::c_int > (1 as ::core::ffi::c_int + bpl) * h {
            64 as ::core::ffi::c_int
        } else {
            (1 as ::core::ffi::c_int + bpl) * h
        })) as size_t;
    out_buf.m_pBuf = malloc(out_buf.m_capacity) as *mut mz_uint8;
    if out_buf.m_pBuf.is_null() {
        free(pComp as *mut ::core::ffi::c_void);
        return NULL;
    }
    z = 41 as ::core::ffi::c_int;
    while z != 0 {
        tdefl_output_buffer_putter(
            &raw mut z as *const ::core::ffi::c_void,
            1 as ::core::ffi::c_int,
            &raw mut out_buf as *mut ::core::ffi::c_void,
        );
        z -= 1;
    }
    tdefl_init(
        pComp,
        Some(
            tdefl_output_buffer_putter
                as unsafe extern "C" fn(
                    *const ::core::ffi::c_void,
                    ::core::ffi::c_int,
                    *mut ::core::ffi::c_void,
                ) -> mz_bool,
        ),
        &raw mut out_buf as *mut ::core::ffi::c_void,
        (s_tdefl_png_num_probes[(if (10 as mz_uint) < level {
            10 as mz_uint
        } else {
            level
        }) as usize]
            | TDEFL_WRITE_ZLIB_HEADER as ::core::ffi::c_int as mz_uint)
            as ::core::ffi::c_int,
    );
    y = 0 as ::core::ffi::c_int;
    while y < h {
        tdefl_compress_buffer(
            pComp,
            &raw mut z as *const ::core::ffi::c_void,
            1 as size_t,
            TDEFL_NO_FLUSH,
        );
        tdefl_compress_buffer(
            pComp,
            (pImage as *mut mz_uint8).offset(
                ((if flip != 0 {
                    h - 1 as ::core::ffi::c_int - y
                } else {
                    y
                }) * bpl) as isize,
            ) as *const ::core::ffi::c_void,
            bpl as size_t,
            TDEFL_NO_FLUSH,
        );
        y += 1;
    }
    if tdefl_compress_buffer(
        pComp,
        ::core::ptr::null::<::core::ffi::c_void>(),
        0 as size_t,
        TDEFL_FINISH,
    ) as ::core::ffi::c_int
        != TDEFL_STATUS_DONE as ::core::ffi::c_int
    {
        free(pComp as *mut ::core::ffi::c_void);
        free(out_buf.m_pBuf as *mut ::core::ffi::c_void);
        return NULL;
    }
    *pLen_out = out_buf.m_size.wrapping_sub(41 as size_t);
    static mut chans: [mz_uint8; 5] = [
        0 as ::core::ffi::c_int as mz_uint8,
        0 as ::core::ffi::c_int as mz_uint8,
        0x4 as ::core::ffi::c_int as mz_uint8,
        0x2 as ::core::ffi::c_int as mz_uint8,
        0x6 as ::core::ffi::c_int as mz_uint8,
    ];
    let mut pnghdr: [mz_uint8; 41] = [
        0x89 as ::core::ffi::c_int as mz_uint8,
        0x50 as ::core::ffi::c_int as mz_uint8,
        0x4e as ::core::ffi::c_int as mz_uint8,
        0x47 as ::core::ffi::c_int as mz_uint8,
        0xd as ::core::ffi::c_int as mz_uint8,
        0xa as ::core::ffi::c_int as mz_uint8,
        0x1a as ::core::ffi::c_int as mz_uint8,
        0xa as ::core::ffi::c_int as mz_uint8,
        0 as ::core::ffi::c_int as mz_uint8,
        0 as ::core::ffi::c_int as mz_uint8,
        0 as ::core::ffi::c_int as mz_uint8,
        0xd as ::core::ffi::c_int as mz_uint8,
        0x49 as ::core::ffi::c_int as mz_uint8,
        0x48 as ::core::ffi::c_int as mz_uint8,
        0x44 as ::core::ffi::c_int as mz_uint8,
        0x52 as ::core::ffi::c_int as mz_uint8,
        0 as ::core::ffi::c_int as mz_uint8,
        0 as ::core::ffi::c_int as mz_uint8,
        0 as ::core::ffi::c_int as mz_uint8,
        0 as ::core::ffi::c_int as mz_uint8,
        0 as ::core::ffi::c_int as mz_uint8,
        0 as ::core::ffi::c_int as mz_uint8,
        0 as ::core::ffi::c_int as mz_uint8,
        0 as ::core::ffi::c_int as mz_uint8,
        0x8 as ::core::ffi::c_int as mz_uint8,
        0 as ::core::ffi::c_int as mz_uint8,
        0 as ::core::ffi::c_int as mz_uint8,
        0 as ::core::ffi::c_int as mz_uint8,
        0 as ::core::ffi::c_int as mz_uint8,
        0 as ::core::ffi::c_int as mz_uint8,
        0 as ::core::ffi::c_int as mz_uint8,
        0 as ::core::ffi::c_int as mz_uint8,
        0 as ::core::ffi::c_int as mz_uint8,
        0 as ::core::ffi::c_int as mz_uint8,
        0 as ::core::ffi::c_int as mz_uint8,
        0 as ::core::ffi::c_int as mz_uint8,
        0 as ::core::ffi::c_int as mz_uint8,
        0x49 as ::core::ffi::c_int as mz_uint8,
        0x44 as ::core::ffi::c_int as mz_uint8,
        0x41 as ::core::ffi::c_int as mz_uint8,
        0x54 as ::core::ffi::c_int as mz_uint8,
    ];
    pnghdr[18 as ::core::ffi::c_int as usize] = (w >> 8 as ::core::ffi::c_int) as mz_uint8;
    pnghdr[19 as ::core::ffi::c_int as usize] = w as mz_uint8;
    pnghdr[22 as ::core::ffi::c_int as usize] = (h >> 8 as ::core::ffi::c_int) as mz_uint8;
    pnghdr[23 as ::core::ffi::c_int as usize] = h as mz_uint8;
    pnghdr[25 as ::core::ffi::c_int as usize] = chans[num_chans as usize];
    pnghdr[33 as ::core::ffi::c_int as usize] = (*pLen_out >> 24 as ::core::ffi::c_int) as mz_uint8;
    pnghdr[34 as ::core::ffi::c_int as usize] = (*pLen_out >> 16 as ::core::ffi::c_int) as mz_uint8;
    pnghdr[35 as ::core::ffi::c_int as usize] = (*pLen_out >> 8 as ::core::ffi::c_int) as mz_uint8;
    pnghdr[36 as ::core::ffi::c_int as usize] = *pLen_out as mz_uint8;
    c = mz_crc32(
        MZ_CRC32_INIT as mz_ulong,
        (&raw mut pnghdr as *mut mz_uint8).offset(12 as ::core::ffi::c_int as isize),
        17 as size_t,
    ) as mz_uint32;
    i = 0 as ::core::ffi::c_int;
    while i < 4 as ::core::ffi::c_int {
        *(&raw mut pnghdr as *mut mz_uint8)
            .offset(29 as ::core::ffi::c_int as isize)
            .offset(i as isize) = (c >> 24 as ::core::ffi::c_int) as mz_uint8;
        i += 1;
        c <<= 8 as ::core::ffi::c_int;
    }
    memcpy(
        out_buf.m_pBuf as *mut ::core::ffi::c_void,
        &raw mut pnghdr as *mut mz_uint8 as *const ::core::ffi::c_void,
        41 as size_t,
    );
    if tdefl_output_buffer_putter(
        b"\0\0\0\0\0\0\0\0IEND\xAEB`\x82\0" as *const u8 as *const ::core::ffi::c_char
            as *const ::core::ffi::c_void,
        16 as ::core::ffi::c_int,
        &raw mut out_buf as *mut ::core::ffi::c_void,
    ) == 0
    {
        *pLen_out = 0 as size_t;
        free(pComp as *mut ::core::ffi::c_void);
        free(out_buf.m_pBuf as *mut ::core::ffi::c_void);
        return NULL;
    }
    c = mz_crc32(
        MZ_CRC32_INIT as mz_ulong,
        out_buf
            .m_pBuf
            .offset(41 as ::core::ffi::c_int as isize)
            .offset(-(4 as ::core::ffi::c_int as isize)),
        (*pLen_out).wrapping_add(4 as size_t),
    ) as mz_uint32;
    i = 0 as ::core::ffi::c_int;
    while i < 4 as ::core::ffi::c_int {
        *out_buf
            .m_pBuf
            .offset(out_buf.m_size as isize)
            .offset(-(16 as ::core::ffi::c_int as isize))
            .offset(i as isize) = (c >> 24 as ::core::ffi::c_int) as mz_uint8;
        i += 1;
        c <<= 8 as ::core::ffi::c_int;
    }
    *pLen_out = (*pLen_out).wrapping_add(57 as size_t);
    free(pComp as *mut ::core::ffi::c_void);
    return out_buf.m_pBuf as *mut ::core::ffi::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn tdefl_write_image_to_png_file_in_memory(
    mut pImage: *const ::core::ffi::c_void,
    mut w: ::core::ffi::c_int,
    mut h: ::core::ffi::c_int,
    mut num_chans: ::core::ffi::c_int,
    mut pLen_out: *mut size_t,
) -> *mut ::core::ffi::c_void {
    return tdefl_write_image_to_png_file_in_memory_ex(
        pImage,
        w,
        h,
        num_chans,
        pLen_out,
        6 as mz_uint,
        MZ_FALSE,
    );
}
#[no_mangle]
pub unsafe extern "C" fn tdefl_compressor_alloc() -> *mut tdefl_compressor {
    return malloc(::core::mem::size_of::<tdefl_compressor>() as size_t) as *mut tdefl_compressor;
}
#[no_mangle]
pub unsafe extern "C" fn tdefl_compressor_free(mut pComp: *mut tdefl_compressor) {
    free(pComp as *mut ::core::ffi::c_void);
}
