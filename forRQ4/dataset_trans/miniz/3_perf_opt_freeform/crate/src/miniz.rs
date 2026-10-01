use core::ffi::*;
use crate::src::miniz_tdef::tdefl_create_comp_flags_from_zip_params;
use crate::src::miniz_tdef::tdefl_get_adler32;
use crate::src::miniz_tdef::tdefl_init;
use crate::src::miniz_tinfl::tinfl_decompress;
pub use crate::src::ffi::*;
pub use crate::src::c_consts::*;
pub use crate::src::c_structs::*;
pub use crate::src::c_types::*;
extern "C" {
    pub type mz_internal_state;
    fn tdefl_compress(
        d: *mut tdefl_compressor,
        pIn_buf: *const c_void,
        pIn_buf_size: *mut size_t,
        pOut_buf: *mut c_void,
        pOut_buf_size: *mut size_t,
        flush: tdefl_flush,
    ) -> tdefl_status;
}

pub type C2RustUnnamed_htaaa91a68 = c_int;
pub const MZ_DEFAULT_COMPRESSION: C2RustUnnamed_htaaa91a68 = -1;
pub const MZ_DEFAULT_LEVEL: C2RustUnnamed_htaaa91a68 = 6;
pub const MZ_UBER_COMPRESSION: C2RustUnnamed_htaaa91a68 = 10;
pub const MZ_BEST_COMPRESSION: C2RustUnnamed_htaaa91a68 = 9;
pub const MZ_BEST_SPEED: C2RustUnnamed_htaaa91a68 = 1;
pub const MZ_NO_COMPRESSION: C2RustUnnamed_htaaa91a68 = 0;

pub const MZ_BLOCK: C2RustUnnamed_htdd24ee73 = 5;
pub const MZ_FINISH: C2RustUnnamed_htdd24ee73 = 4;
pub const MZ_FULL_FLUSH: C2RustUnnamed_htdd24ee73 = 3;
pub const MZ_SYNC_FLUSH: C2RustUnnamed_htdd24ee73 = 2;
pub const MZ_PARTIAL_FLUSH: C2RustUnnamed_htdd24ee73 = 1;
pub const MZ_NO_FLUSH: C2RustUnnamed_htdd24ee73 = 0;
pub const MZ_PARAM_ERROR: C2RustUnnamed_htaaa91a68 = -10000;
pub const MZ_VERSION_ERROR: C2RustUnnamed_htaaa91a68 = -6;
pub const MZ_BUF_ERROR: C2RustUnnamed_htaaa91a68 = -5;
pub const MZ_MEM_ERROR: C2RustUnnamed_htaaa91a68 = -4;
pub const MZ_DATA_ERROR: C2RustUnnamed_htaaa91a68 = -3;
pub const MZ_STREAM_ERROR: C2RustUnnamed_htaaa91a68 = -2;
pub const MZ_ERRNO: C2RustUnnamed_htaaa91a68 = -1;
pub const MZ_NEED_DICT: C2RustUnnamed_htaaa91a68 = 2;
pub const MZ_STREAM_END: C2RustUnnamed_htaaa91a68 = 1;
pub const MZ_OK: C2RustUnnamed_htaaa91a68 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mz_stream_s {
    pub next_in: *const c_uchar,
    pub avail_in: c_uint,
    pub total_in: mz_ulong,
    pub next_out: *mut c_uchar,
    pub avail_out: c_uint,
    pub total_out: mz_ulong,
    pub msg: *mut c_char,
    pub state: *mut mz_internal_state,
    pub zalloc: mz_alloc_func,
    pub zfree: mz_free_func,
    pub opaque: *mut c_void,
    pub data_type: c_int,
    pub adler: mz_ulong,
    pub reserved: mz_ulong,
}
pub type mz_stream = mz_stream_s;
pub type mz_streamp = *mut mz_stream;

pub const TDEFL_COMPUTE_ADLER32: C2RustUnnamed_htdd24ee73 = 8192;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct inflate_state {
    pub m_decomp: tinfl_decompressor,
    pub m_dict_ofs: mz_uint,
    pub m_dict_avail: mz_uint,
    pub m_first_call: mz_uint,
    pub m_has_flushed: mz_uint,
    pub m_window_bits: c_int,
    pub m_dict: [mz_uint8; 32768],
    pub m_last_status: tinfl_status,
}

pub const TINFL_FLAG_COMPUTE_ADLER32: C2RustUnnamed_htdd24ee73 = 8;
pub const TINFL_FLAG_HAS_MORE_INPUT: C2RustUnnamed_htdd24ee73 = 2;
pub const TINFL_FLAG_USING_NON_WRAPPING_OUTPUT_BUF: C2RustUnnamed_htdd24ee73 = 4;
pub const TINFL_FLAG_PARSE_ZLIB_HEADER: C2RustUnnamed_htdd24ee73 = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub m_err: c_int,
    pub m_pDesc: *const c_char,
}

pub const TDEFL_FORCE_ALL_RAW_BLOCKS: C2RustUnnamed_htdd24ee73 = 524288;
pub const TDEFL_FORCE_ALL_STATIC_BLOCKS: C2RustUnnamed_htdd24ee73 = 262144;
pub const TDEFL_FILTER_MATCHES: C2RustUnnamed_htdd24ee73 = 131072;
pub const TDEFL_RLE_MATCHES: C2RustUnnamed_htdd24ee73 = 65536;
pub const TDEFL_NONDETERMINISTIC_PARSING_FLAG: C2RustUnnamed_htdd24ee73 = 32768;
pub const TDEFL_GREEDY_PARSING_FLAG: C2RustUnnamed_htdd24ee73 = 16384;
pub const TDEFL_WRITE_ZLIB_HEADER: C2RustUnnamed_htdd24ee73 = 4096;

pub const MZ_ADLER32_INIT: c_int = 1 as c_int;

pub const MZ_VERSION: [c_char; 7] =
    unsafe { ::core::mem::transmute::<[u8; 7], [c_char; 7]>(*b"11.3.2\0") };
pub const MZ_DEFAULT_WINDOW_BITS: c_int = 15 as c_int;

#[inline]
pub unsafe fn mz_adler32(
    mut adler: mz_ulong,
    mut ptr: *const c_uchar,
    mut buf_len: size_t,
) -> mz_ulong {
    let mut i: mz_uint32 = 0;
    let mut s1: mz_uint32 = (adler & 0xffff as mz_ulong) as mz_uint32;
    let mut s2: mz_uint32 = (adler >> 16 as c_int) as mz_uint32;
    let mut block_len: size_t = buf_len.wrapping_rem(5552 as size_t);
    if ptr.is_null() {
        return MZ_ADLER32_INIT as mz_ulong;
    }
    while buf_len != 0 {
        i = 0 as mz_uint32;
        while (i.wrapping_add(7 as mz_uint32) as size_t) < block_len {
            s1 = s1.wrapping_add(*ptr.offset(0 as c_int as isize) as mz_uint32);
            s2 = s2.wrapping_add(s1);
            s1 = s1.wrapping_add(*ptr.offset(1 as c_int as isize) as mz_uint32);
            s2 = s2.wrapping_add(s1);
            s1 = s1.wrapping_add(*ptr.offset(2 as c_int as isize) as mz_uint32);
            s2 = s2.wrapping_add(s1);
            s1 = s1.wrapping_add(*ptr.offset(3 as c_int as isize) as mz_uint32);
            s2 = s2.wrapping_add(s1);
            s1 = s1.wrapping_add(*ptr.offset(4 as c_int as isize) as mz_uint32);
            s2 = s2.wrapping_add(s1);
            s1 = s1.wrapping_add(*ptr.offset(5 as c_int as isize) as mz_uint32);
            s2 = s2.wrapping_add(s1);
            s1 = s1.wrapping_add(*ptr.offset(6 as c_int as isize) as mz_uint32);
            s2 = s2.wrapping_add(s1);
            s1 = s1.wrapping_add(*ptr.offset(7 as c_int as isize) as mz_uint32);
            s2 = s2.wrapping_add(s1);
            i = i.wrapping_add(8 as mz_uint32);
            ptr = ptr.offset(8 as c_int as isize);
        }
        while (i as size_t) < block_len {
            let fresh0 = ptr;
            ptr = ptr.offset(1);
            s1 = s1.wrapping_add(*fresh0 as mz_uint32);
            s2 = s2.wrapping_add(s1);
            i = i.wrapping_add(1);
        }
        s1 = (s1 as c_uint).wrapping_rem(65521 as c_uint) as mz_uint32
            as mz_uint32;
        s2 = (s2 as c_uint).wrapping_rem(65521 as c_uint) as mz_uint32
            as mz_uint32;
        buf_len = buf_len.wrapping_sub(block_len);
        block_len = 5552 as size_t;
    }
    return (s2 << 16 as c_int).wrapping_add(s1) as mz_ulong;
}
#[inline]
pub unsafe fn mz_crc32(
    mut crc: mz_ulong,
    mut ptr: *const mz_uint8,
    mut buf_len: size_t,
) -> mz_ulong {
    static mut s_crc_table: [mz_uint32; 256] = [
        0 as c_int as mz_uint32,
        0x77073096 as c_int as mz_uint32,
        0xee0e612c as c_uint,
        0x990951ba as c_uint,
        0x76dc419 as c_int as mz_uint32,
        0x706af48f as c_int as mz_uint32,
        0xe963a535 as c_uint,
        0x9e6495a3 as c_uint,
        0xedb8832 as c_int as mz_uint32,
        0x79dcb8a4 as c_int as mz_uint32,
        0xe0d5e91e as c_uint,
        0x97d2d988 as c_uint,
        0x9b64c2b as c_int as mz_uint32,
        0x7eb17cbd as c_int as mz_uint32,
        0xe7b82d07 as c_uint,
        0x90bf1d91 as c_uint,
        0x1db71064 as c_int as mz_uint32,
        0x6ab020f2 as c_int as mz_uint32,
        0xf3b97148 as c_uint,
        0x84be41de as c_uint,
        0x1adad47d as c_int as mz_uint32,
        0x6ddde4eb as c_int as mz_uint32,
        0xf4d4b551 as c_uint,
        0x83d385c7 as c_uint,
        0x136c9856 as c_int as mz_uint32,
        0x646ba8c0 as c_int as mz_uint32,
        0xfd62f97a as c_uint,
        0x8a65c9ec as c_uint,
        0x14015c4f as c_int as mz_uint32,
        0x63066cd9 as c_int as mz_uint32,
        0xfa0f3d63 as c_uint,
        0x8d080df5 as c_uint,
        0x3b6e20c8 as c_int as mz_uint32,
        0x4c69105e as c_int as mz_uint32,
        0xd56041e4 as c_uint,
        0xa2677172 as c_uint,
        0x3c03e4d1 as c_int as mz_uint32,
        0x4b04d447 as c_int as mz_uint32,
        0xd20d85fd as c_uint,
        0xa50ab56b as c_uint,
        0x35b5a8fa as c_int as mz_uint32,
        0x42b2986c as c_int as mz_uint32,
        0xdbbbc9d6 as c_uint,
        0xacbcf940 as c_uint,
        0x32d86ce3 as c_int as mz_uint32,
        0x45df5c75 as c_int as mz_uint32,
        0xdcd60dcf as c_uint,
        0xabd13d59 as c_uint,
        0x26d930ac as c_int as mz_uint32,
        0x51de003a as c_int as mz_uint32,
        0xc8d75180 as c_uint,
        0xbfd06116 as c_uint,
        0x21b4f4b5 as c_int as mz_uint32,
        0x56b3c423 as c_int as mz_uint32,
        0xcfba9599 as c_uint,
        0xb8bda50f as c_uint,
        0x2802b89e as c_int as mz_uint32,
        0x5f058808 as c_int as mz_uint32,
        0xc60cd9b2 as c_uint,
        0xb10be924 as c_uint,
        0x2f6f7c87 as c_int as mz_uint32,
        0x58684c11 as c_int as mz_uint32,
        0xc1611dab as c_uint,
        0xb6662d3d as c_uint,
        0x76dc4190 as c_int as mz_uint32,
        0x1db7106 as c_int as mz_uint32,
        0x98d220bc as c_uint,
        0xefd5102a as c_uint,
        0x71b18589 as c_int as mz_uint32,
        0x6b6b51f as c_int as mz_uint32,
        0x9fbfe4a5 as c_uint,
        0xe8b8d433 as c_uint,
        0x7807c9a2 as c_int as mz_uint32,
        0xf00f934 as c_int as mz_uint32,
        0x9609a88e as c_uint,
        0xe10e9818 as c_uint,
        0x7f6a0dbb as c_int as mz_uint32,
        0x86d3d2d as c_int as mz_uint32,
        0x91646c97 as c_uint,
        0xe6635c01 as c_uint,
        0x6b6b51f4 as c_int as mz_uint32,
        0x1c6c6162 as c_int as mz_uint32,
        0x856530d8 as c_uint,
        0xf262004e as c_uint,
        0x6c0695ed as c_int as mz_uint32,
        0x1b01a57b as c_int as mz_uint32,
        0x8208f4c1 as c_uint,
        0xf50fc457 as c_uint,
        0x65b0d9c6 as c_int as mz_uint32,
        0x12b7e950 as c_int as mz_uint32,
        0x8bbeb8ea as c_uint,
        0xfcb9887c as c_uint,
        0x62dd1ddf as c_int as mz_uint32,
        0x15da2d49 as c_int as mz_uint32,
        0x8cd37cf3 as c_uint,
        0xfbd44c65 as c_uint,
        0x4db26158 as c_int as mz_uint32,
        0x3ab551ce as c_int as mz_uint32,
        0xa3bc0074 as c_uint,
        0xd4bb30e2 as c_uint,
        0x4adfa541 as c_int as mz_uint32,
        0x3dd895d7 as c_int as mz_uint32,
        0xa4d1c46d as c_uint,
        0xd3d6f4fb as c_uint,
        0x4369e96a as c_int as mz_uint32,
        0x346ed9fc as c_int as mz_uint32,
        0xad678846 as c_uint,
        0xda60b8d0 as c_uint,
        0x44042d73 as c_int as mz_uint32,
        0x33031de5 as c_int as mz_uint32,
        0xaa0a4c5f as c_uint,
        0xdd0d7cc9 as c_uint,
        0x5005713c as c_int as mz_uint32,
        0x270241aa as c_int as mz_uint32,
        0xbe0b1010 as c_uint,
        0xc90c2086 as c_uint,
        0x5768b525 as c_int as mz_uint32,
        0x206f85b3 as c_int as mz_uint32,
        0xb966d409 as c_uint,
        0xce61e49f as c_uint,
        0x5edef90e as c_int as mz_uint32,
        0x29d9c998 as c_int as mz_uint32,
        0xb0d09822 as c_uint,
        0xc7d7a8b4 as c_uint,
        0x59b33d17 as c_int as mz_uint32,
        0x2eb40d81 as c_int as mz_uint32,
        0xb7bd5c3b as c_uint,
        0xc0ba6cad as c_uint,
        0xedb88320 as c_uint,
        0x9abfb3b6 as c_uint,
        0x3b6e20c as c_int as mz_uint32,
        0x74b1d29a as c_int as mz_uint32,
        0xead54739 as c_uint,
        0x9dd277af as c_uint,
        0x4db2615 as c_int as mz_uint32,
        0x73dc1683 as c_int as mz_uint32,
        0xe3630b12 as c_uint,
        0x94643b84 as c_uint,
        0xd6d6a3e as c_int as mz_uint32,
        0x7a6a5aa8 as c_int as mz_uint32,
        0xe40ecf0b as c_uint,
        0x9309ff9d as c_uint,
        0xa00ae27 as c_int as mz_uint32,
        0x7d079eb1 as c_int as mz_uint32,
        0xf00f9344 as c_uint,
        0x8708a3d2 as c_uint,
        0x1e01f268 as c_int as mz_uint32,
        0x6906c2fe as c_int as mz_uint32,
        0xf762575d as c_uint,
        0x806567cb as c_uint,
        0x196c3671 as c_int as mz_uint32,
        0x6e6b06e7 as c_int as mz_uint32,
        0xfed41b76 as c_uint,
        0x89d32be0 as c_uint,
        0x10da7a5a as c_int as mz_uint32,
        0x67dd4acc as c_int as mz_uint32,
        0xf9b9df6f as c_uint,
        0x8ebeeff9 as c_uint,
        0x17b7be43 as c_int as mz_uint32,
        0x60b08ed5 as c_int as mz_uint32,
        0xd6d6a3e8 as c_uint,
        0xa1d1937e as c_uint,
        0x38d8c2c4 as c_int as mz_uint32,
        0x4fdff252 as c_int as mz_uint32,
        0xd1bb67f1 as c_uint,
        0xa6bc5767 as c_uint,
        0x3fb506dd as c_int as mz_uint32,
        0x48b2364b as c_int as mz_uint32,
        0xd80d2bda as c_uint,
        0xaf0a1b4c as c_uint,
        0x36034af6 as c_int as mz_uint32,
        0x41047a60 as c_int as mz_uint32,
        0xdf60efc3 as c_uint,
        0xa867df55 as c_uint,
        0x316e8eef as c_int as mz_uint32,
        0x4669be79 as c_int as mz_uint32,
        0xcb61b38c as c_uint,
        0xbc66831a as c_uint,
        0x256fd2a0 as c_int as mz_uint32,
        0x5268e236 as c_int as mz_uint32,
        0xcc0c7795 as c_uint,
        0xbb0b4703 as c_uint,
        0x220216b9 as c_int as mz_uint32,
        0x5505262f as c_int as mz_uint32,
        0xc5ba3bbe as c_uint,
        0xb2bd0b28 as c_uint,
        0x2bb45a92 as c_int as mz_uint32,
        0x5cb36a04 as c_int as mz_uint32,
        0xc2d7ffa7 as c_uint,
        0xb5d0cf31 as c_uint,
        0x2cd99e8b as c_int as mz_uint32,
        0x5bdeae1d as c_int as mz_uint32,
        0x9b64c2b0 as c_uint,
        0xec63f226 as c_uint,
        0x756aa39c as c_int as mz_uint32,
        0x26d930a as c_int as mz_uint32,
        0x9c0906a9 as c_uint,
        0xeb0e363f as c_uint,
        0x72076785 as c_int as mz_uint32,
        0x5005713 as c_int as mz_uint32,
        0x95bf4a82 as c_uint,
        0xe2b87a14 as c_uint,
        0x7bb12bae as c_int as mz_uint32,
        0xcb61b38 as c_int as mz_uint32,
        0x92d28e9b as c_uint,
        0xe5d5be0d as c_uint,
        0x7cdcefb7 as c_int as mz_uint32,
        0xbdbdf21 as c_int as mz_uint32,
        0x86d3d2d4 as c_uint,
        0xf1d4e242 as c_uint,
        0x68ddb3f8 as c_int as mz_uint32,
        0x1fda836e as c_int as mz_uint32,
        0x81be16cd as c_uint,
        0xf6b9265b as c_uint,
        0x6fb077e1 as c_int as mz_uint32,
        0x18b74777 as c_int as mz_uint32,
        0x88085ae6 as c_uint,
        0xff0f6a70 as c_uint,
        0x66063bca as c_int as mz_uint32,
        0x11010b5c as c_int as mz_uint32,
        0x8f659eff as c_uint,
        0xf862ae69 as c_uint,
        0x616bffd3 as c_int as mz_uint32,
        0x166ccf45 as c_int as mz_uint32,
        0xa00ae278 as c_uint,
        0xd70dd2ee as c_uint,
        0x4e048354 as c_int as mz_uint32,
        0x3903b3c2 as c_int as mz_uint32,
        0xa7672661 as c_uint,
        0xd06016f7 as c_uint,
        0x4969474d as c_int as mz_uint32,
        0x3e6e77db as c_int as mz_uint32,
        0xaed16a4a as c_uint,
        0xd9d65adc as c_uint,
        0x40df0b66 as c_int as mz_uint32,
        0x37d83bf0 as c_int as mz_uint32,
        0xa9bcae53 as c_uint,
        0xdebb9ec5 as c_uint,
        0x47b2cf7f as c_int as mz_uint32,
        0x30b5ffe9 as c_int as mz_uint32,
        0xbdbdf21c as c_uint,
        0xcabac28a as c_uint,
        0x53b39330 as c_int as mz_uint32,
        0x24b4a3a6 as c_int as mz_uint32,
        0xbad03605 as c_uint,
        0xcdd70693 as c_uint,
        0x54de5729 as c_int as mz_uint32,
        0x23d967bf as c_int as mz_uint32,
        0xb3667a2e as c_uint,
        0xc4614ab8 as c_uint,
        0x5d681b02 as c_int as mz_uint32,
        0x2a6f2b94 as c_int as mz_uint32,
        0xb40bbe37 as c_uint,
        0xc30c8ea1 as c_uint,
        0x5a05df1b as c_int as mz_uint32,
        0x2d02ef8d as c_int as mz_uint32,
    ];
    let mut crc32: mz_uint32 = crc as mz_uint32 ^ 0xffffffff as mz_uint32;
    let mut pByte_buf: *const mz_uint8 = ptr;
    while buf_len >= 4 as size_t {
        crc32 = crc32 >> 8 as c_int
            ^ s_crc_table[((crc32
                ^ *pByte_buf.offset(0 as c_int as isize) as mz_uint32)
                & 0xff as mz_uint32) as usize];
        crc32 = crc32 >> 8 as c_int
            ^ s_crc_table[((crc32
                ^ *pByte_buf.offset(1 as c_int as isize) as mz_uint32)
                & 0xff as mz_uint32) as usize];
        crc32 = crc32 >> 8 as c_int
            ^ s_crc_table[((crc32
                ^ *pByte_buf.offset(2 as c_int as isize) as mz_uint32)
                & 0xff as mz_uint32) as usize];
        crc32 = crc32 >> 8 as c_int
            ^ s_crc_table[((crc32
                ^ *pByte_buf.offset(3 as c_int as isize) as mz_uint32)
                & 0xff as mz_uint32) as usize];
        pByte_buf = pByte_buf.offset(4 as c_int as isize);
        buf_len = buf_len.wrapping_sub(4 as size_t);
    }
    while buf_len != 0 {
        crc32 = crc32 >> 8 as c_int
            ^ s_crc_table[((crc32
                ^ *pByte_buf.offset(0 as c_int as isize) as mz_uint32)
                & 0xff as mz_uint32) as usize];
        pByte_buf = pByte_buf.offset(1);
        buf_len = buf_len.wrapping_sub(1);
    }
    return !crc32 as mz_ulong;
}
#[inline]
pub unsafe fn mz_free(mut p: *mut c_void) {
    free(p);
}
#[no_mangle]
pub unsafe extern "C" fn miniz_def_alloc_func(
    mut opaque: *mut c_void,
    mut items: size_t,
    mut size: size_t,
) -> *mut c_void {
    return malloc(items.wrapping_mul(size));
}
#[no_mangle]
pub unsafe extern "C" fn miniz_def_free_func(
    mut opaque: *mut c_void,
    mut address: *mut c_void,
) {
    free(address);
}
#[no_mangle]
pub unsafe extern "C" fn miniz_def_realloc_func(
    mut opaque: *mut c_void,
    mut address: *mut c_void,
    mut items: size_t,
    mut size: size_t,
) -> *mut c_void {
    return realloc(address, items.wrapping_mul(size));
}
#[inline]
pub fn mz_version() -> *const c_char { {
    return MZ_VERSION.as_ptr();
} }
#[inline]
pub fn mz_deflateInit(
    mut pStream: mz_streamp,
    mut level: c_int,
) -> c_int { {
    return mz_deflateInit2(
        pStream,
        level,
        MZ_DEFLATED,
        MZ_DEFAULT_WINDOW_BITS,
        9 as c_int,
        MZ_DEFAULT_STRATEGY as c_int,
    );
} }
#[inline]
pub fn mz_deflateInit2(
    mut pStream: mz_streamp,
    mut level: c_int,
    mut method: c_int,
    mut window_bits: c_int,
    mut mem_level: c_int,
    mut strategy: c_int,
) -> c_int { unsafe {
    let mut pComp: *mut tdefl_compressor = ::core::ptr::null_mut::<tdefl_compressor>();
    let mut comp_flags: mz_uint = TDEFL_COMPUTE_ADLER32 as c_int as mz_uint
        | tdefl_create_comp_flags_from_zip_params(level, window_bits, strategy);
    if pStream.is_null() {
        return MZ_STREAM_ERROR as c_int;
    }
    if method != MZ_DEFLATED
        || (mem_level < 1 as c_int || mem_level > 9 as c_int)
        || window_bits != MZ_DEFAULT_WINDOW_BITS && -window_bits != MZ_DEFAULT_WINDOW_BITS
    {
        return MZ_PARAM_ERROR as c_int;
    }
    (*pStream).data_type = 0 as c_int;
    (*pStream).adler = MZ_ADLER32_INIT as mz_ulong;
    (*pStream).msg = ::core::ptr::null_mut::<c_char>();
    (*pStream).reserved = 0 as mz_ulong;
    (*pStream).total_in = 0 as mz_ulong;
    (*pStream).total_out = 0 as mz_ulong;
    if (*pStream).zalloc.is_none() {
        (*pStream).zalloc = Some(
            miniz_def_alloc_func
                as unsafe extern "C" fn(
                    *mut c_void,
                    size_t,
                    size_t,
                ) -> *mut c_void,
        ) as mz_alloc_func;
    }
    if (*pStream).zfree.is_none() {
        (*pStream).zfree = Some(
            miniz_def_free_func
                as unsafe extern "C" fn(*mut c_void, *mut c_void) -> (),
        ) as mz_free_func;
    }
    pComp = (*pStream).zalloc.expect("non-null function pointer")(
        (*pStream).opaque,
        1 as size_t,
        ::core::mem::size_of::<tdefl_compressor>() as size_t,
    ) as *mut tdefl_compressor;
    if pComp.is_null() {
        return MZ_MEM_ERROR as c_int;
    }
    (*pStream).state = pComp as *mut mz_internal_state;
    if tdefl_init(pComp, None, NULL, comp_flags as c_int) as c_int
        != TDEFL_STATUS_OKAY as c_int
    {
        mz_deflateEnd(pStream);
        return MZ_PARAM_ERROR as c_int;
    }
    return MZ_OK as c_int;
} }
#[inline]
pub fn mz_deflateReset(mut pStream: mz_streamp) -> c_int { unsafe {
    if pStream.is_null()
        || (*pStream).state.is_null()
        || (*pStream).zalloc.is_none()
        || (*pStream).zfree.is_none()
    {
        return MZ_STREAM_ERROR as c_int;
    }
    (*pStream).total_out = 0 as mz_ulong;
    (*pStream).total_in = (*pStream).total_out;
    tdefl_init(
        (*pStream).state as *mut tdefl_compressor,
        None,
        NULL,
        (*((*pStream).state as *mut tdefl_compressor)).m_flags as c_int,
    );
    return MZ_OK as c_int;
} }
#[inline]
pub fn mz_deflate(
    mut pStream: mz_streamp,
    mut flush: c_int,
) -> c_int { unsafe {
    let mut in_bytes: size_t = 0;
    let mut out_bytes: size_t = 0;
    let mut orig_total_in: mz_ulong = 0;
    let mut orig_total_out: mz_ulong = 0;
    let mut mz_status: c_int = MZ_OK as c_int;
    if pStream.is_null()
        || (*pStream).state.is_null()
        || flush < 0 as c_int
        || flush > MZ_FINISH as c_int
        || (*pStream).next_out.is_null()
    {
        return MZ_STREAM_ERROR as c_int;
    }
    if (*pStream).avail_out == 0 {
        return MZ_BUF_ERROR as c_int;
    }
    if flush == MZ_PARTIAL_FLUSH as c_int {
        flush = MZ_SYNC_FLUSH as c_int;
    }
    if (*((*pStream).state as *mut tdefl_compressor)).m_prev_return_status as c_int
        == TDEFL_STATUS_DONE as c_int
    {
        return if flush == MZ_FINISH as c_int {
            MZ_STREAM_END as c_int
        } else {
            MZ_BUF_ERROR as c_int
        };
    }
    orig_total_in = (*pStream).total_in;
    orig_total_out = (*pStream).total_out;
    loop {
        let mut defl_status: tdefl_status = TDEFL_STATUS_OKAY;
        in_bytes = (*pStream).avail_in as size_t;
        out_bytes = (*pStream).avail_out as size_t;
        defl_status = tdefl_compress(
            (*pStream).state as *mut tdefl_compressor,
            (*pStream).next_in as *const c_void,
            &raw mut in_bytes,
            (*pStream).next_out as *mut c_void,
            &raw mut out_bytes,
            flush as tdefl_flush,
        );
        (*pStream).next_in = (*pStream).next_in.offset(in_bytes as mz_uint as isize);
        (*pStream).avail_in = (*pStream)
            .avail_in
            .wrapping_sub(in_bytes as mz_uint as c_uint);
        (*pStream).total_in = (*pStream)
            .total_in
            .wrapping_add(in_bytes as mz_uint as mz_ulong);
        (*pStream).adler = tdefl_get_adler32((*pStream).state as *mut tdefl_compressor) as mz_ulong;
        (*pStream).next_out = (*pStream).next_out.offset(out_bytes as mz_uint as isize);
        (*pStream).avail_out = (*pStream)
            .avail_out
            .wrapping_sub(out_bytes as mz_uint as c_uint);
        (*pStream).total_out = (*pStream)
            .total_out
            .wrapping_add(out_bytes as mz_uint as mz_ulong);
        if (defl_status as c_int) < 0 as c_int {
            mz_status = MZ_STREAM_ERROR as c_int;
            break;
        } else if defl_status as c_int == TDEFL_STATUS_DONE as c_int {
            mz_status = MZ_STREAM_END as c_int;
            break;
        } else {
            if (*pStream).avail_out == 0 {
                break;
            }
            if !((*pStream).avail_in == 0 && flush != MZ_FINISH as c_int) {
                continue;
            }
            if flush != 0
                || (*pStream).total_in != orig_total_in
                || (*pStream).total_out != orig_total_out
            {
                break;
            }
            return MZ_BUF_ERROR as c_int;
        }
    }
    return mz_status;
} }
#[inline]
pub fn mz_deflateEnd(mut pStream: mz_streamp) -> c_int { unsafe {
    if pStream.is_null() {
        return MZ_STREAM_ERROR as c_int;
    }
    if !(*pStream).state.is_null() {
        (*pStream).zfree.expect("non-null function pointer")(
            (*pStream).opaque,
            (*pStream).state as *mut c_void,
        );
        (*pStream).state = ::core::ptr::null_mut::<mz_internal_state>();
    }
    return MZ_OK as c_int;
} }
#[inline]
pub fn mz_deflateBound(
    mut pStream: mz_streamp,
    mut source_len: mz_ulong,
) -> mz_ulong { {
    return if (128 as mz_ulong).wrapping_add(
        source_len
            .wrapping_mul(110 as mz_ulong)
            .wrapping_div(100 as mz_ulong),
    ) > (128 as mz_ulong).wrapping_add(source_len).wrapping_add(
        source_len
            .wrapping_div((31 as c_int * 1024 as c_int) as mz_ulong)
            .wrapping_add(1 as mz_ulong)
            .wrapping_mul(5 as mz_ulong),
    ) {
        (128 as mz_ulong).wrapping_add(
            source_len
                .wrapping_mul(110 as mz_ulong)
                .wrapping_div(100 as mz_ulong),
        )
    } else {
        (128 as mz_ulong).wrapping_add(source_len).wrapping_add(
            source_len
                .wrapping_div((31 as c_int * 1024 as c_int) as mz_ulong)
                .wrapping_add(1 as mz_ulong)
                .wrapping_mul(5 as mz_ulong),
        )
    };
} }
#[inline]
pub unsafe fn mz_compress2(
    mut pDest: *mut c_uchar,
    mut pDest_len: *mut mz_ulong,
    mut pSource: *const c_uchar,
    mut source_len: mz_ulong,
    mut level: c_int,
) -> c_int {
    let pDest_len_view: &mut mz_ulong = unsafe { &mut *pDest_len };
    let mut status: c_int = 0;
    let mut stream: mz_stream = mz_stream_s {
        next_in: ::core::ptr::null::<c_uchar>(),
        avail_in: 0,
        total_in: 0,
        next_out: ::core::ptr::null_mut::<c_uchar>(),
        avail_out: 0,
        total_out: 0,
        msg: ::core::ptr::null_mut::<c_char>(),
        state: ::core::ptr::null_mut::<mz_internal_state>(),
        zalloc: None,
        zfree: None,
        opaque: ::core::ptr::null_mut::<c_void>(),
        data_type: 0,
        adler: 0,
        reserved: 0,
    };
    memset(
        &raw mut stream as *mut c_void,
        0 as c_int,
        ::core::mem::size_of::<mz_stream>() as size_t,
    );
    if (source_len | *pDest_len_view) as mz_uint64 > 0xffffffff as mz_uint64 {
        return MZ_PARAM_ERROR as c_int;
    }
    stream.next_in = pSource;
    stream.avail_in = source_len as mz_uint32 as c_uint;
    stream.next_out = pDest;
    stream.avail_out = *pDest_len_view as mz_uint32 as c_uint;
    status = mz_deflateInit(&raw mut stream, level);
    if status != MZ_OK as c_int {
        return status;
    }
    status = mz_deflate(&raw mut stream, MZ_FINISH as c_int);
    if status != MZ_STREAM_END as c_int {
        mz_deflateEnd(&raw mut stream);
        return if status == MZ_OK as c_int {
            MZ_BUF_ERROR as c_int
        } else {
            status
        };
    }
    *pDest_len_view = stream.total_out;
    return mz_deflateEnd(&raw mut stream);
}
#[inline]
pub unsafe fn mz_compress(
    mut pDest: *mut c_uchar,
    mut pDest_len: *mut mz_ulong,
    mut pSource: *const c_uchar,
    mut source_len: mz_ulong,
) -> c_int {
    return mz_compress2(
        pDest,
        pDest_len,
        pSource,
        source_len,
        MZ_DEFAULT_COMPRESSION as c_int,
    );
}
#[inline]
pub fn mz_compressBound(mut source_len: mz_ulong) -> mz_ulong { {
    return mz_deflateBound(::core::ptr::null_mut::<mz_stream>(), source_len);
} }
#[inline]
pub fn mz_inflateInit2(
    mut pStream: mz_streamp,
    mut window_bits: c_int,
) -> c_int { unsafe {
    let mut pDecomp: *mut inflate_state = ::core::ptr::null_mut::<inflate_state>();
    if pStream.is_null() {
        return MZ_STREAM_ERROR as c_int;
    }
    if window_bits != MZ_DEFAULT_WINDOW_BITS && -window_bits != MZ_DEFAULT_WINDOW_BITS {
        return MZ_PARAM_ERROR as c_int;
    }
    (*pStream).data_type = 0 as c_int;
    (*pStream).adler = 0 as mz_ulong;
    (*pStream).msg = ::core::ptr::null_mut::<c_char>();
    (*pStream).total_in = 0 as mz_ulong;
    (*pStream).total_out = 0 as mz_ulong;
    (*pStream).reserved = 0 as mz_ulong;
    if (*pStream).zalloc.is_none() {
        (*pStream).zalloc = Some(
            miniz_def_alloc_func
                as unsafe extern "C" fn(
                    *mut c_void,
                    size_t,
                    size_t,
                ) -> *mut c_void,
        ) as mz_alloc_func;
    }
    if (*pStream).zfree.is_none() {
        (*pStream).zfree = Some(
            miniz_def_free_func
                as unsafe extern "C" fn(*mut c_void, *mut c_void) -> (),
        ) as mz_free_func;
    }
    pDecomp = (*pStream).zalloc.expect("non-null function pointer")(
        (*pStream).opaque,
        1 as size_t,
        ::core::mem::size_of::<inflate_state>() as size_t,
    ) as *mut inflate_state;
    if pDecomp.is_null() {
        return MZ_MEM_ERROR as c_int;
    }
    (*pStream).state = pDecomp as *mut mz_internal_state;
    (*pDecomp).m_decomp.m_state = 0 as mz_uint32;
    (*pDecomp).m_dict_ofs = 0 as mz_uint;
    (*pDecomp).m_dict_avail = 0 as mz_uint;
    (*pDecomp).m_last_status = TINFL_STATUS_NEEDS_MORE_INPUT;
    (*pDecomp).m_first_call = 1 as mz_uint;
    (*pDecomp).m_has_flushed = 0 as mz_uint;
    (*pDecomp).m_window_bits = window_bits;
    return MZ_OK as c_int;
} }
#[inline]
pub fn mz_inflateInit(mut pStream: mz_streamp) -> c_int { {
    return mz_inflateInit2(pStream, MZ_DEFAULT_WINDOW_BITS);
} }
#[inline]
pub fn mz_inflateReset(mut pStream: mz_streamp) -> c_int { unsafe {
    let mut pDecomp: *mut inflate_state = ::core::ptr::null_mut::<inflate_state>();
    if pStream.is_null() {
        return MZ_STREAM_ERROR as c_int;
    }
    (*pStream).data_type = 0 as c_int;
    (*pStream).adler = 0 as mz_ulong;
    (*pStream).msg = ::core::ptr::null_mut::<c_char>();
    (*pStream).total_in = 0 as mz_ulong;
    (*pStream).total_out = 0 as mz_ulong;
    (*pStream).reserved = 0 as mz_ulong;
    pDecomp = (*pStream).state as *mut inflate_state;
    (*pDecomp).m_decomp.m_state = 0 as mz_uint32;
    (*pDecomp).m_dict_ofs = 0 as mz_uint;
    (*pDecomp).m_dict_avail = 0 as mz_uint;
    (*pDecomp).m_last_status = TINFL_STATUS_NEEDS_MORE_INPUT;
    (*pDecomp).m_first_call = 1 as mz_uint;
    (*pDecomp).m_has_flushed = 0 as mz_uint;
    return MZ_OK as c_int;
} }
#[inline]
pub fn mz_inflate(
    mut pStream: mz_streamp,
    mut flush: c_int,
) -> c_int { unsafe {
    let mut pState: *mut inflate_state = ::core::ptr::null_mut::<inflate_state>();
    let mut n: mz_uint = 0;
    let mut first_call: mz_uint = 0;
    let mut decomp_flags: mz_uint = TINFL_FLAG_COMPUTE_ADLER32 as c_int as mz_uint;
    let mut in_bytes: size_t = 0;
    let mut out_bytes: size_t = 0;
    let mut orig_avail_in: size_t = 0;
    let mut status: tinfl_status = TINFL_STATUS_DONE;
    if pStream.is_null() || (*pStream).state.is_null() {
        return MZ_STREAM_ERROR as c_int;
    }
    if flush == MZ_PARTIAL_FLUSH as c_int {
        flush = MZ_SYNC_FLUSH as c_int;
    }
    if flush != 0
        && flush != MZ_SYNC_FLUSH as c_int
        && flush != MZ_FINISH as c_int
    {
        return MZ_STREAM_ERROR as c_int;
    }
    pState = (*pStream).state as *mut inflate_state;
    if (*pState).m_window_bits > 0 as c_int {
        decomp_flags |= TINFL_FLAG_PARSE_ZLIB_HEADER as c_int as mz_uint;
    }
    orig_avail_in = (*pStream).avail_in as size_t;
    first_call = (*pState).m_first_call;
    (*pState).m_first_call = 0 as mz_uint;
    if ((*pState).m_last_status as c_int) < 0 as c_int {
        return MZ_DATA_ERROR as c_int;
    }
    if (*pState).m_has_flushed != 0 && flush != MZ_FINISH as c_int {
        return MZ_STREAM_ERROR as c_int;
    }
    (*pState).m_has_flushed |=
        (flush == MZ_FINISH as c_int) as c_int as mz_uint;
    if flush == MZ_FINISH as c_int && first_call != 0 {
        decomp_flags |= TINFL_FLAG_USING_NON_WRAPPING_OUTPUT_BUF as c_int as mz_uint;
        in_bytes = (*pStream).avail_in as size_t;
        out_bytes = (*pStream).avail_out as size_t;
        status = tinfl_decompress(
            &raw mut (*pState).m_decomp,
            (*pStream).next_in as *const mz_uint8,
            &raw mut in_bytes,
            (*pStream).next_out as *mut mz_uint8,
            (*pStream).next_out as *mut mz_uint8,
            &raw mut out_bytes,
            decomp_flags as mz_uint32,
        );
        (*pState).m_last_status = status;
        (*pStream).next_in = (*pStream).next_in.offset(in_bytes as mz_uint as isize);
        (*pStream).avail_in = (*pStream)
            .avail_in
            .wrapping_sub(in_bytes as mz_uint as c_uint);
        (*pStream).total_in = (*pStream)
            .total_in
            .wrapping_add(in_bytes as mz_uint as mz_ulong);
        (*pStream).adler = (*pState).m_decomp.m_check_adler32 as mz_ulong;
        (*pStream).next_out = (*pStream).next_out.offset(out_bytes as mz_uint as isize);
        (*pStream).avail_out = (*pStream)
            .avail_out
            .wrapping_sub(out_bytes as mz_uint as c_uint);
        (*pStream).total_out = (*pStream)
            .total_out
            .wrapping_add(out_bytes as mz_uint as mz_ulong);
        if (status as c_int) < 0 as c_int {
            return MZ_DATA_ERROR as c_int;
        } else if status as c_int != TINFL_STATUS_DONE as c_int {
            (*pState).m_last_status = TINFL_STATUS_FAILED;
            return MZ_BUF_ERROR as c_int;
        }
        return MZ_STREAM_END as c_int;
    }
    if flush != MZ_FINISH as c_int {
        decomp_flags |= TINFL_FLAG_HAS_MORE_INPUT as c_int as mz_uint;
    }
    if (*pState).m_dict_avail != 0 {
        n = if (*pState).m_dict_avail < (*pStream).avail_out as mz_uint {
            (*pState).m_dict_avail
        } else {
            (*pStream).avail_out as mz_uint
        };
        memcpy(
            (*pStream).next_out as *mut c_void,
            (&raw mut (*pState).m_dict as *mut mz_uint8).offset((*pState).m_dict_ofs as isize)
                as *const c_void,
            n as size_t,
        );
        (*pStream).next_out = (*pStream).next_out.offset(n as isize);
        (*pStream).avail_out = (*pStream).avail_out.wrapping_sub(n as c_uint);
        (*pStream).total_out = (*pStream).total_out.wrapping_add(n as mz_ulong);
        (*pState).m_dict_avail = (*pState).m_dict_avail.wrapping_sub(n);
        (*pState).m_dict_ofs = (*pState).m_dict_ofs.wrapping_add(n)
            & (TINFL_LZ_DICT_SIZE - 1 as c_int) as mz_uint;
        return if (*pState).m_last_status as c_int
            == TINFL_STATUS_DONE as c_int
            && (*pState).m_dict_avail == 0
        {
            MZ_STREAM_END as c_int
        } else {
            MZ_OK as c_int
        };
    }
    loop {
        in_bytes = (*pStream).avail_in as size_t;
        out_bytes = (TINFL_LZ_DICT_SIZE as mz_uint).wrapping_sub((*pState).m_dict_ofs) as size_t;
        status = tinfl_decompress(
            &raw mut (*pState).m_decomp,
            (*pStream).next_in as *const mz_uint8,
            &raw mut in_bytes,
            &raw mut (*pState).m_dict as *mut mz_uint8,
            (&raw mut (*pState).m_dict as *mut mz_uint8).offset((*pState).m_dict_ofs as isize),
            &raw mut out_bytes,
            decomp_flags as mz_uint32,
        );
        (*pState).m_last_status = status;
        (*pStream).next_in = (*pStream).next_in.offset(in_bytes as mz_uint as isize);
        (*pStream).avail_in = (*pStream)
            .avail_in
            .wrapping_sub(in_bytes as mz_uint as c_uint);
        (*pStream).total_in = (*pStream)
            .total_in
            .wrapping_add(in_bytes as mz_uint as mz_ulong);
        (*pStream).adler = (*pState).m_decomp.m_check_adler32 as mz_ulong;
        (*pState).m_dict_avail = out_bytes as mz_uint;
        n = if (*pState).m_dict_avail < (*pStream).avail_out as mz_uint {
            (*pState).m_dict_avail
        } else {
            (*pStream).avail_out as mz_uint
        };
        memcpy(
            (*pStream).next_out as *mut c_void,
            (&raw mut (*pState).m_dict as *mut mz_uint8).offset((*pState).m_dict_ofs as isize)
                as *const c_void,
            n as size_t,
        );
        (*pStream).next_out = (*pStream).next_out.offset(n as isize);
        (*pStream).avail_out = (*pStream).avail_out.wrapping_sub(n as c_uint);
        (*pStream).total_out = (*pStream).total_out.wrapping_add(n as mz_ulong);
        (*pState).m_dict_avail = (*pState).m_dict_avail.wrapping_sub(n);
        (*pState).m_dict_ofs = (*pState).m_dict_ofs.wrapping_add(n)
            & (TINFL_LZ_DICT_SIZE - 1 as c_int) as mz_uint;
        if (status as c_int) < 0 as c_int {
            return MZ_DATA_ERROR as c_int;
        } else if status as c_int
            == TINFL_STATUS_NEEDS_MORE_INPUT as c_int
            && orig_avail_in == 0
        {
            return MZ_BUF_ERROR as c_int;
        } else if flush == MZ_FINISH as c_int {
            if status as c_int == TINFL_STATUS_DONE as c_int {
                return if (*pState).m_dict_avail != 0 {
                    MZ_BUF_ERROR as c_int
                } else {
                    MZ_STREAM_END as c_int
                };
            } else if (*pStream).avail_out == 0 {
                return MZ_BUF_ERROR as c_int;
            }
        } else if status as c_int == TINFL_STATUS_DONE as c_int
            || (*pStream).avail_in == 0
            || (*pStream).avail_out == 0
            || (*pState).m_dict_avail != 0
        {
            break;
        }
    }
    return if status as c_int == TINFL_STATUS_DONE as c_int
        && (*pState).m_dict_avail == 0
    {
        MZ_STREAM_END as c_int
    } else {
        MZ_OK as c_int
    };
} }
#[inline]
pub fn mz_inflateEnd(mut pStream: mz_streamp) -> c_int { unsafe {
    if pStream.is_null() {
        return MZ_STREAM_ERROR as c_int;
    }
    if !(*pStream).state.is_null() {
        (*pStream).zfree.expect("non-null function pointer")(
            (*pStream).opaque,
            (*pStream).state as *mut c_void,
        );
        (*pStream).state = ::core::ptr::null_mut::<mz_internal_state>();
    }
    return MZ_OK as c_int;
} }
#[inline]
pub unsafe fn mz_uncompress2(
    mut pDest: *mut c_uchar,
    mut pDest_len: *mut mz_ulong,
    mut pSource: *const c_uchar,
    mut pSource_len: *mut mz_ulong,
) -> c_int {
    let pDest_len_view: &mut mz_ulong = unsafe { &mut *pDest_len };
    let mut stream: mz_stream = mz_stream_s {
        next_in: ::core::ptr::null::<c_uchar>(),
        avail_in: 0,
        total_in: 0,
        next_out: ::core::ptr::null_mut::<c_uchar>(),
        avail_out: 0,
        total_out: 0,
        msg: ::core::ptr::null_mut::<c_char>(),
        state: ::core::ptr::null_mut::<mz_internal_state>(),
        zalloc: None,
        zfree: None,
        opaque: ::core::ptr::null_mut::<c_void>(),
        data_type: 0,
        adler: 0,
        reserved: 0,
    };
    let mut status: c_int = 0;
    memset(
        &raw mut stream as *mut c_void,
        0 as c_int,
        ::core::mem::size_of::<mz_stream>() as size_t,
    );
    if (*pSource_len | *pDest_len_view) as mz_uint64 > 0xffffffff as mz_uint64 {
        return MZ_PARAM_ERROR as c_int;
    }
    stream.next_in = pSource;
    stream.avail_in = *pSource_len as mz_uint32 as c_uint;
    stream.next_out = pDest;
    stream.avail_out = *pDest_len_view as mz_uint32 as c_uint;
    status = mz_inflateInit(&raw mut stream);
    if status != MZ_OK as c_int {
        return status;
    }
    status = mz_inflate(&raw mut stream, MZ_FINISH as c_int);
    *pSource_len = (*pSource_len).wrapping_sub(stream.avail_in as mz_ulong);
    if status != MZ_STREAM_END as c_int {
        mz_inflateEnd(&raw mut stream);
        return if status == MZ_BUF_ERROR as c_int && stream.avail_in == 0 {
            MZ_DATA_ERROR as c_int
        } else {
            status
        };
    }
    *pDest_len_view = stream.total_out;
    return mz_inflateEnd(&raw mut stream);
}
#[inline]
pub unsafe fn mz_uncompress(
    mut pDest: *mut c_uchar,
    mut pDest_len: *mut mz_ulong,
    mut pSource: *const c_uchar,
    mut source_len: mz_ulong,
) -> c_int {
    return mz_uncompress2(pDest, pDest_len, pSource, &raw mut source_len);
}
#[inline]
pub fn mz_error(mut err: c_int) -> *const c_char { unsafe {
    static mut s_error_descs: [C2RustUnnamed_3; 10] = [
        C2RustUnnamed_3 {
            m_err: MZ_OK as c_int,
            m_pDesc: b"\0" as *const u8 as *const c_char,
        },
        C2RustUnnamed_3 {
            m_err: MZ_STREAM_END as c_int,
            m_pDesc: b"stream end\0" as *const u8 as *const c_char,
        },
        C2RustUnnamed_3 {
            m_err: MZ_NEED_DICT as c_int,
            m_pDesc: b"need dictionary\0" as *const u8 as *const c_char,
        },
        C2RustUnnamed_3 {
            m_err: MZ_ERRNO as c_int,
            m_pDesc: b"file error\0" as *const u8 as *const c_char,
        },
        C2RustUnnamed_3 {
            m_err: MZ_STREAM_ERROR as c_int,
            m_pDesc: b"stream error\0" as *const u8 as *const c_char,
        },
        C2RustUnnamed_3 {
            m_err: MZ_DATA_ERROR as c_int,
            m_pDesc: b"data error\0" as *const u8 as *const c_char,
        },
        C2RustUnnamed_3 {
            m_err: MZ_MEM_ERROR as c_int,
            m_pDesc: b"out of memory\0" as *const u8 as *const c_char,
        },
        C2RustUnnamed_3 {
            m_err: MZ_BUF_ERROR as c_int,
            m_pDesc: b"buf error\0" as *const u8 as *const c_char,
        },
        C2RustUnnamed_3 {
            m_err: MZ_VERSION_ERROR as c_int,
            m_pDesc: b"version error\0" as *const u8 as *const c_char,
        },
        C2RustUnnamed_3 {
            m_err: MZ_PARAM_ERROR as c_int,
            m_pDesc: b"parameter error\0" as *const u8 as *const c_char,
        },
    ];
    let mut i: mz_uint = 0;
    i = 0 as mz_uint;
    while (i as usize)
        < (::core::mem::size_of::<[C2RustUnnamed_3; 10]>() as usize)
            .wrapping_div(::core::mem::size_of::<C2RustUnnamed_3>() as usize)
    {
        if s_error_descs[i as usize].m_err == err {
            return s_error_descs[i as usize].m_pDesc;
        }
        i = i.wrapping_add(1);
    }
    return ::core::ptr::null::<c_char>();
} }
