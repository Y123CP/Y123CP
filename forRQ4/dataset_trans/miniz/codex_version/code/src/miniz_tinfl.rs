extern "C" {
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

#[inline(always)]
unsafe fn tinfl_copy_match_fast(
    dst: *mut mz_uint8,
    src: *const mz_uint8,
    len: usize,
    dist: usize,
) -> *mut mz_uint8 {
    if len == 0 {
        return dst;
    }
    if dist == 1 {
        ::core::ptr::write_bytes(dst, *src, len);
        return dst.add(len);
    }
    if src >= dst.cast_const() || dist >= len {
        ::core::ptr::copy_nonoverlapping(src, dst, len);
        return dst.add(len);
    }

    ::core::ptr::copy_nonoverlapping(src, dst, dist);
    let mut copied = dist;
    while copied < len {
        let chunk = if copied < len - copied {
            copied
        } else {
            len - copied
        };
        ::core::ptr::copy_nonoverlapping(dst, dst.add(copied), chunk);
        copied += chunk;
    }
    dst.add(len)
}

pub type size_t = usize;
pub type __int16_t = i16;
pub type __uint16_t = u16;
pub type __uint32_t = u32;
pub type __uint64_t = u64;
pub type int16_t = __int16_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type mz_uint8 = ::core::ffi::c_uchar;
pub type mz_int16 = int16_t;
pub type mz_uint16 = uint16_t;
pub type mz_uint32 = uint32_t;
pub type mz_uint = uint32_t;
pub type mz_uint64 = uint64_t;
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const TINFL_FLAG_COMPUTE_ADLER32: C2RustUnnamed = 8;
pub const TINFL_FLAG_USING_NON_WRAPPING_OUTPUT_BUF: C2RustUnnamed = 4;
pub const TINFL_FLAG_HAS_MORE_INPUT: C2RustUnnamed = 2;
pub const TINFL_FLAG_PARSE_ZLIB_HEADER: C2RustUnnamed = 1;
pub const TINFL_STATUS_DONE: tinfl_status = 0;
pub type tinfl_status = ::core::ffi::c_int;
pub const TINFL_STATUS_HAS_MORE_OUTPUT: tinfl_status = 2;
pub const TINFL_STATUS_NEEDS_MORE_INPUT: tinfl_status = 1;
pub const TINFL_STATUS_FAILED: tinfl_status = -1;
pub const TINFL_STATUS_ADLER32_MISMATCH: tinfl_status = -2;
pub const TINFL_STATUS_BAD_PARAM: tinfl_status = -3;
pub const TINFL_STATUS_FAILED_CANNOT_MAKE_PROGRESS: tinfl_status = -4;
pub type tinfl_decompressor = tinfl_decompressor_tag;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tinfl_decompressor_tag {
    pub m_state: mz_uint32,
    pub m_num_bits: mz_uint32,
    pub m_zhdr0: mz_uint32,
    pub m_zhdr1: mz_uint32,
    pub m_z_adler32: mz_uint32,
    pub m_final: mz_uint32,
    pub m_type: mz_uint32,
    pub m_check_adler32: mz_uint32,
    pub m_dist: mz_uint32,
    pub m_counter: mz_uint32,
    pub m_num_extra: mz_uint32,
    pub m_table_sizes: [mz_uint32; 3],
    pub m_bit_buf: tinfl_bit_buf_t,
    pub m_dist_from_out_buf_start: size_t,
    pub m_look_up: [[mz_int16; 1024]; 3],
    pub m_tree_0: [mz_int16; 576],
    pub m_tree_1: [mz_int16; 64],
    pub m_tree_2: [mz_int16; 38],
    pub m_code_size_0: [mz_uint8; 288],
    pub m_code_size_1: [mz_uint8; 32],
    pub m_code_size_2: [mz_uint8; 19],
    pub m_raw_header: [mz_uint8; 4],
    pub m_len_codes: [mz_uint8; 457],
}
pub type tinfl_bit_buf_t = mz_uint64;
pub const TINFL_FAST_LOOKUP_BITS: C2RustUnnamed_0 = 10;
pub const TINFL_FAST_LOOKUP_SIZE: C2RustUnnamed_0 = 1024;
pub type tinfl_put_buf_func_ptr = Option<
    unsafe extern "C" fn(
        *const ::core::ffi::c_void,
        ::core::ffi::c_int,
        *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int,
>;
pub type C2RustUnnamed_0 = ::core::ffi::c_uint;
pub const TINFL_MAX_HUFF_SYMBOLS_2: C2RustUnnamed_0 = 19;
pub const TINFL_MAX_HUFF_SYMBOLS_1: C2RustUnnamed_0 = 32;
pub const TINFL_MAX_HUFF_SYMBOLS_0: C2RustUnnamed_0 = 288;
pub const TINFL_MAX_HUFF_TABLES: C2RustUnnamed_0 = 3;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const TINFL_DECOMPRESS_MEM_TO_MEM_FAILED: size_t = -(1 as ::core::ffi::c_int) as size_t;
pub const TINFL_LZ_DICT_SIZE: ::core::ffi::c_int = 32768 as ::core::ffi::c_int;
unsafe extern "C" fn tinfl_clear_tree(mut r: *mut tinfl_decompressor) {
    if (*r).m_type == 0 as mz_uint32 {
        memset(
            &raw mut (*r).m_tree_0 as *mut mz_int16 as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<[mz_int16; 576]>() as size_t,
        );
    } else if (*r).m_type == 1 as mz_uint32 {
        memset(
            &raw mut (*r).m_tree_1 as *mut mz_int16 as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<[mz_int16; 64]>() as size_t,
        );
    } else {
        memset(
            &raw mut (*r).m_tree_2 as *mut mz_int16 as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<[mz_int16; 38]>() as size_t,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn tinfl_decompress(
    mut r: *mut tinfl_decompressor,
    mut pIn_buf_next: *const mz_uint8,
    mut pIn_buf_size: *mut size_t,
    mut pOut_buf_start: *mut mz_uint8,
    mut pOut_buf_next: *mut mz_uint8,
    mut pOut_buf_size: *mut size_t,
    decomp_flags: mz_uint32,
) -> tinfl_status {
    let mut s_1: mz_uint = 0;
    let mut c_12: mz_uint = 0;
    let mut c_11: mz_uint = 0;
    let mut pSrc: *mut mz_uint8 = ::core::ptr::null_mut::<mz_uint8>();
    let mut extra_bits_0: mz_uint = 0;
    let mut c_10: mz_uint = 0;
    let mut temp_1: ::core::ffi::c_int = 0;
    let mut code_len_2: mz_uint = 0;
    let mut c_9: mz_uint = 0;
    let mut extra_bits: mz_uint = 0;
    let mut c_8: mz_uint = 0;
    let mut sym2: ::core::ffi::c_int = 0;
    let mut code_len_1: mz_uint = 0;
    let mut temp_0: ::core::ffi::c_int = 0;
    let mut code_len_0: mz_uint = 0;
    let mut c_7: mz_uint = 0;
    let mut tree_next: ::core::ffi::c_int = 0;
    let mut tree_cur: ::core::ffi::c_int = 0;
    let mut pLookUp: *mut mz_int16 = ::core::ptr::null_mut::<mz_int16>();
    let mut pTree: *mut mz_int16 = ::core::ptr::null_mut::<mz_int16>();
    let mut pCode_size: *mut mz_uint8 = ::core::ptr::null_mut::<mz_uint8>();
    let mut i_0: mz_uint = 0;
    let mut j: mz_uint = 0;
    let mut used_syms: mz_uint = 0;
    let mut total: mz_uint = 0;
    let mut sym_index: mz_uint = 0;
    let mut next_code: [mz_uint; 17] = [0; 17];
    let mut total_syms: [mz_uint; 16] = [0; 16];
    let mut s_0: mz_uint = 0;
    let mut c_6: mz_uint = 0;
    let mut temp: ::core::ffi::c_int = 0;
    let mut code_len: mz_uint = 0;
    let mut c_5: mz_uint = 0;
    let mut s: mz_uint = 0;
    let mut c_4: mz_uint = 0;
    let mut c_3: mz_uint = 0;
    let mut n: size_t = 0;
    let mut c_2: mz_uint = 0;
    let mut c_1: mz_uint = 0;
    let mut c_0: mz_uint = 0;
    let mut c: mz_uint = 0;
    let mut current_block: u64;
    static mut s_length_base: [mz_uint16; 31] = [
        3 as ::core::ffi::c_int as mz_uint16,
        4 as ::core::ffi::c_int as mz_uint16,
        5 as ::core::ffi::c_int as mz_uint16,
        6 as ::core::ffi::c_int as mz_uint16,
        7 as ::core::ffi::c_int as mz_uint16,
        8 as ::core::ffi::c_int as mz_uint16,
        9 as ::core::ffi::c_int as mz_uint16,
        10 as ::core::ffi::c_int as mz_uint16,
        11 as ::core::ffi::c_int as mz_uint16,
        13 as ::core::ffi::c_int as mz_uint16,
        15 as ::core::ffi::c_int as mz_uint16,
        17 as ::core::ffi::c_int as mz_uint16,
        19 as ::core::ffi::c_int as mz_uint16,
        23 as ::core::ffi::c_int as mz_uint16,
        27 as ::core::ffi::c_int as mz_uint16,
        31 as ::core::ffi::c_int as mz_uint16,
        35 as ::core::ffi::c_int as mz_uint16,
        43 as ::core::ffi::c_int as mz_uint16,
        51 as ::core::ffi::c_int as mz_uint16,
        59 as ::core::ffi::c_int as mz_uint16,
        67 as ::core::ffi::c_int as mz_uint16,
        83 as ::core::ffi::c_int as mz_uint16,
        99 as ::core::ffi::c_int as mz_uint16,
        115 as ::core::ffi::c_int as mz_uint16,
        131 as ::core::ffi::c_int as mz_uint16,
        163 as ::core::ffi::c_int as mz_uint16,
        195 as ::core::ffi::c_int as mz_uint16,
        227 as ::core::ffi::c_int as mz_uint16,
        258 as ::core::ffi::c_int as mz_uint16,
        0 as ::core::ffi::c_int as mz_uint16,
        0 as ::core::ffi::c_int as mz_uint16,
    ];
    static mut s_length_extra: [mz_uint8; 31] = [
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
        2 as ::core::ffi::c_int as mz_uint8,
        2 as ::core::ffi::c_int as mz_uint8,
        2 as ::core::ffi::c_int as mz_uint8,
        2 as ::core::ffi::c_int as mz_uint8,
        3 as ::core::ffi::c_int as mz_uint8,
        3 as ::core::ffi::c_int as mz_uint8,
        3 as ::core::ffi::c_int as mz_uint8,
        3 as ::core::ffi::c_int as mz_uint8,
        4 as ::core::ffi::c_int as mz_uint8,
        4 as ::core::ffi::c_int as mz_uint8,
        4 as ::core::ffi::c_int as mz_uint8,
        4 as ::core::ffi::c_int as mz_uint8,
        5 as ::core::ffi::c_int as mz_uint8,
        5 as ::core::ffi::c_int as mz_uint8,
        5 as ::core::ffi::c_int as mz_uint8,
        5 as ::core::ffi::c_int as mz_uint8,
        0 as ::core::ffi::c_int as mz_uint8,
        0 as ::core::ffi::c_int as mz_uint8,
        0 as ::core::ffi::c_int as mz_uint8,
    ];
    static mut s_dist_base: [mz_uint16; 32] = [
        1 as ::core::ffi::c_int as mz_uint16,
        2 as ::core::ffi::c_int as mz_uint16,
        3 as ::core::ffi::c_int as mz_uint16,
        4 as ::core::ffi::c_int as mz_uint16,
        5 as ::core::ffi::c_int as mz_uint16,
        7 as ::core::ffi::c_int as mz_uint16,
        9 as ::core::ffi::c_int as mz_uint16,
        13 as ::core::ffi::c_int as mz_uint16,
        17 as ::core::ffi::c_int as mz_uint16,
        25 as ::core::ffi::c_int as mz_uint16,
        33 as ::core::ffi::c_int as mz_uint16,
        49 as ::core::ffi::c_int as mz_uint16,
        65 as ::core::ffi::c_int as mz_uint16,
        97 as ::core::ffi::c_int as mz_uint16,
        129 as ::core::ffi::c_int as mz_uint16,
        193 as ::core::ffi::c_int as mz_uint16,
        257 as ::core::ffi::c_int as mz_uint16,
        385 as ::core::ffi::c_int as mz_uint16,
        513 as ::core::ffi::c_int as mz_uint16,
        769 as ::core::ffi::c_int as mz_uint16,
        1025 as ::core::ffi::c_int as mz_uint16,
        1537 as ::core::ffi::c_int as mz_uint16,
        2049 as ::core::ffi::c_int as mz_uint16,
        3073 as ::core::ffi::c_int as mz_uint16,
        4097 as ::core::ffi::c_int as mz_uint16,
        6145 as ::core::ffi::c_int as mz_uint16,
        8193 as ::core::ffi::c_int as mz_uint16,
        12289 as ::core::ffi::c_int as mz_uint16,
        16385 as ::core::ffi::c_int as mz_uint16,
        24577 as ::core::ffi::c_int as mz_uint16,
        0 as ::core::ffi::c_int as mz_uint16,
        0 as ::core::ffi::c_int as mz_uint16,
    ];
    static mut s_dist_extra: [mz_uint8; 32] = [
        0 as ::core::ffi::c_int as mz_uint8,
        0 as ::core::ffi::c_int as mz_uint8,
        0 as ::core::ffi::c_int as mz_uint8,
        0 as ::core::ffi::c_int as mz_uint8,
        1 as ::core::ffi::c_int as mz_uint8,
        1 as ::core::ffi::c_int as mz_uint8,
        2 as ::core::ffi::c_int as mz_uint8,
        2 as ::core::ffi::c_int as mz_uint8,
        3 as ::core::ffi::c_int as mz_uint8,
        3 as ::core::ffi::c_int as mz_uint8,
        4 as ::core::ffi::c_int as mz_uint8,
        4 as ::core::ffi::c_int as mz_uint8,
        5 as ::core::ffi::c_int as mz_uint8,
        5 as ::core::ffi::c_int as mz_uint8,
        6 as ::core::ffi::c_int as mz_uint8,
        6 as ::core::ffi::c_int as mz_uint8,
        7 as ::core::ffi::c_int as mz_uint8,
        7 as ::core::ffi::c_int as mz_uint8,
        8 as ::core::ffi::c_int as mz_uint8,
        8 as ::core::ffi::c_int as mz_uint8,
        9 as ::core::ffi::c_int as mz_uint8,
        9 as ::core::ffi::c_int as mz_uint8,
        10 as ::core::ffi::c_int as mz_uint8,
        10 as ::core::ffi::c_int as mz_uint8,
        11 as ::core::ffi::c_int as mz_uint8,
        11 as ::core::ffi::c_int as mz_uint8,
        12 as ::core::ffi::c_int as mz_uint8,
        12 as ::core::ffi::c_int as mz_uint8,
        13 as ::core::ffi::c_int as mz_uint8,
        13 as ::core::ffi::c_int as mz_uint8,
        0,
        0,
    ];
    static mut s_length_dezigzag: [mz_uint8; 19] = [
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
    static mut s_min_table_sizes: [mz_uint16; 3] = [
        257 as ::core::ffi::c_int as mz_uint16,
        1 as ::core::ffi::c_int as mz_uint16,
        4 as ::core::ffi::c_int as mz_uint16,
    ];
    let mut pTrees: [*mut mz_int16; 3] = [::core::ptr::null_mut::<mz_int16>(); 3];
    let mut pCode_sizes: [*mut mz_uint8; 3] = [::core::ptr::null_mut::<mz_uint8>(); 3];
    let mut status: tinfl_status = TINFL_STATUS_FAILED;
    let mut num_bits: mz_uint32 = 0;
    let mut dist: mz_uint32 = 0;
    let mut counter: mz_uint32 = 0;
    let mut num_extra: mz_uint32 = 0;
    let mut bit_buf: tinfl_bit_buf_t = 0;
    let mut pIn_buf_cur: *const mz_uint8 = pIn_buf_next;
    let pIn_buf_end: *const mz_uint8 = pIn_buf_next.offset(*pIn_buf_size as isize);
    let mut pOut_buf_cur: *mut mz_uint8 = pOut_buf_next;
    let pOut_buf_end: *mut mz_uint8 = if !pOut_buf_next.is_null() {
        pOut_buf_next.offset(*pOut_buf_size as isize)
    } else {
        ::core::ptr::null_mut::<mz_uint8>()
    };
    let mut out_buf_size_mask: size_t = if decomp_flags
        & TINFL_FLAG_USING_NON_WRAPPING_OUTPUT_BUF as ::core::ffi::c_int as mz_uint32
        != 0
    {
        -(1 as ::core::ffi::c_int) as size_t
    } else {
        (pOut_buf_next.offset_from(pOut_buf_start) as ::core::ffi::c_long as size_t)
            .wrapping_add(*pOut_buf_size)
            .wrapping_sub(1 as size_t)
    };
    let mut dist_from_out_buf_start: size_t = 0;
    if out_buf_size_mask.wrapping_add(1 as size_t) & out_buf_size_mask != 0
        || pOut_buf_next < pOut_buf_start
    {
        *pOut_buf_size = 0 as size_t;
        *pIn_buf_size = *pOut_buf_size;
        return TINFL_STATUS_BAD_PARAM;
    }
    pTrees[0 as ::core::ffi::c_int as usize] = &raw mut (*r).m_tree_0 as *mut mz_int16;
    pTrees[1 as ::core::ffi::c_int as usize] = &raw mut (*r).m_tree_1 as *mut mz_int16;
    pTrees[2 as ::core::ffi::c_int as usize] = &raw mut (*r).m_tree_2 as *mut mz_int16;
    pCode_sizes[0 as ::core::ffi::c_int as usize] = &raw mut (*r).m_code_size_0 as *mut mz_uint8;
    pCode_sizes[1 as ::core::ffi::c_int as usize] = &raw mut (*r).m_code_size_1 as *mut mz_uint8;
    pCode_sizes[2 as ::core::ffi::c_int as usize] = &raw mut (*r).m_code_size_2 as *mut mz_uint8;
    num_bits = (*r).m_num_bits;
    bit_buf = (*r).m_bit_buf;
    dist = (*r).m_dist;
    counter = (*r).m_counter;
    num_extra = (*r).m_num_extra;
    dist_from_out_buf_start = (*r).m_dist_from_out_buf_start;
    match (*r).m_state {
        0 => {
            (*r).m_zhdr1 = 0 as mz_uint32;
            (*r).m_zhdr0 = (*r).m_zhdr1;
            num_extra = (*r).m_zhdr0;
            counter = num_extra;
            dist = counter;
            num_bits = dist;
            bit_buf = num_bits as tinfl_bit_buf_t;
            (*r).m_check_adler32 = 1 as mz_uint32;
            (*r).m_z_adler32 = (*r).m_check_adler32;
            if decomp_flags & TINFL_FLAG_PARSE_ZLIB_HEADER as ::core::ffi::c_int as mz_uint32 != 0 {
                current_block = 2370887241019905314;
            } else {
                current_block = 313581471991351815;
            }
        }
        1 => {
            current_block = 2370887241019905314;
        }
        2 => {
            current_block = 15125582407903384992;
        }
        36 => {
            current_block = 9007357115414505193;
        }
        3 => {
            current_block = 5891011138178424807;
        }
        5 => {
            current_block = 16779030619667747692;
        }
        6 => {
            current_block = 15237655884915618618;
        }
        7 => {
            current_block = 1428307939028130064;
        }
        39 => {
            current_block = 10863493864285401582;
        }
        51 => {
            current_block = 13014351284863956202;
        }
        52 => {
            current_block = 15460309861373144675;
        }
        9 => {
            current_block = 15614898248724990345;
        }
        38 => {
            current_block = 5908482871227205451;
        }
        10 => {
            current_block = 18316056106135622027;
        }
        11 => {
            current_block = 12387625063048049585;
        }
        14 => {
            current_block = 7510843268513837357;
        }
        35 => {
            current_block = 14308887607299961996;
        }
        16 => {
            current_block = 11990966878772509462;
        }
        17 => {
            current_block = 15726232576006602401;
        }
        18 => {
            current_block = 778403239630769426;
        }
        21 => {
            current_block = 12060859132547941340;
        }
        23 => {
            current_block = 9080448346277817863;
        }
        24 => {
            current_block = 12916373135125519917;
        }
        40 => {
            current_block = 17058499098102203106;
        }
        54 => {
            current_block = 4979785777352049371;
        }
        25 => {
            current_block = 8650459447085150701;
        }
        26 => {
            current_block = 6721946588916655032;
        }
        27 => {
            current_block = 1833465351478693723;
        }
        37 => {
            current_block = 13286354103097804402;
        }
        53 => {
            current_block = 7674033599746029549;
        }
        32 => {
            current_block = 14402387155550584875;
        }
        41 => {
            current_block = 12381699995462811668;
        }
        42 => {
            current_block = 13250426973517230943;
        }
        34 => {
            current_block = 9997576841122142810;
        }
        _ => {
            current_block = 11953489883170298063;
        }
    }
    match current_block {
        2370887241019905314 => {
            if pIn_buf_cur >= pIn_buf_end {
                status = (if decomp_flags
                    & TINFL_FLAG_HAS_MORE_INPUT as ::core::ffi::c_int as mz_uint32
                    != 0
                {
                    TINFL_STATUS_NEEDS_MORE_INPUT as ::core::ffi::c_int
                } else {
                    TINFL_STATUS_FAILED_CANNOT_MAKE_PROGRESS as ::core::ffi::c_int
                }) as tinfl_status;
                (*r).m_state = 1 as mz_uint32;
                current_block = 11953489883170298063;
            } else {
                let fresh0 = pIn_buf_cur;
                pIn_buf_cur = pIn_buf_cur.offset(1);
                (*r).m_zhdr0 = *fresh0 as mz_uint32;
                current_block = 15125582407903384992;
            }
        }
        _ => {}
    }
    match current_block {
        15125582407903384992 => {
            if pIn_buf_cur >= pIn_buf_end {
                status = (if decomp_flags
                    & TINFL_FLAG_HAS_MORE_INPUT as ::core::ffi::c_int as mz_uint32
                    != 0
                {
                    TINFL_STATUS_NEEDS_MORE_INPUT as ::core::ffi::c_int
                } else {
                    TINFL_STATUS_FAILED_CANNOT_MAKE_PROGRESS as ::core::ffi::c_int
                }) as tinfl_status;
                (*r).m_state = 2 as mz_uint32;
                current_block = 11953489883170298063;
            } else {
                let fresh1 = pIn_buf_cur;
                pIn_buf_cur = pIn_buf_cur.offset(1);
                (*r).m_zhdr1 = *fresh1 as mz_uint32;
                counter = ((*r)
                    .m_zhdr0
                    .wrapping_mul(256 as mz_uint32)
                    .wrapping_add((*r).m_zhdr1)
                    .wrapping_rem(31 as mz_uint32)
                    != 0 as mz_uint32
                    || (*r).m_zhdr1 & 32 as mz_uint32 != 0
                    || (*r).m_zhdr0 & 15 as mz_uint32 != 8 as mz_uint32)
                    as ::core::ffi::c_int as mz_uint32;
                if decomp_flags
                    & TINFL_FLAG_USING_NON_WRAPPING_OUTPUT_BUF as ::core::ffi::c_int as mz_uint32
                    == 0
                {
                    counter |= ((1 as ::core::ffi::c_uint)
                        << (8 as mz_uint32).wrapping_add((*r).m_zhdr0 >> 4 as ::core::ffi::c_int)
                        > 32768 as ::core::ffi::c_uint
                        || out_buf_size_mask.wrapping_add(1 as size_t)
                            < (1 as ::core::ffi::c_int as size_t)
                                << (8 as mz_uint32)
                                    .wrapping_add((*r).m_zhdr0 >> 4 as ::core::ffi::c_int))
                        as ::core::ffi::c_int as mz_uint32;
                }
                if counter != 0 {
                    current_block = 9007357115414505193;
                } else {
                    current_block = 313581471991351815;
                }
            }
        }
        _ => {}
    }
    match current_block {
        9007357115414505193 => {
            status = TINFL_STATUS_FAILED;
            (*r).m_state = 36 as mz_uint32;
            current_block = 11953489883170298063;
        }
        _ => {}
    }
    loop {
        match current_block {
            313581471991351815 => {
                if num_bits < 3 as ::core::ffi::c_int as mz_uint {
                    current_block = 13460095289871124136;
                } else {
                    current_block = 6545907279487748450;
                }
            }
            14402387155550584875 => {
                if pIn_buf_cur >= pIn_buf_end {
                    status = (if decomp_flags
                        & TINFL_FLAG_HAS_MORE_INPUT as ::core::ffi::c_int as mz_uint32
                        != 0
                    {
                        TINFL_STATUS_NEEDS_MORE_INPUT as ::core::ffi::c_int
                    } else {
                        TINFL_STATUS_FAILED_CANNOT_MAKE_PROGRESS as ::core::ffi::c_int
                    }) as tinfl_status;
                    (*r).m_state = 32 as mz_uint32;
                    current_block = 11953489883170298063;
                    continue;
                } else {
                    let fresh34 = pIn_buf_cur;
                    pIn_buf_cur = pIn_buf_cur.offset(1);
                    c_11 = *fresh34 as mz_uint;
                    bit_buf |= (c_11 as tinfl_bit_buf_t) << num_bits;
                    num_bits = num_bits.wrapping_add(8 as mz_uint32);
                    if num_bits < num_bits & 7 as mz_uint32 {
                        current_block = 4159430576127784976;
                    } else {
                        current_block = 955764296456093747;
                    }
                }
            }
            7674033599746029549 => {
                if pOut_buf_cur >= pOut_buf_end {
                    status = TINFL_STATUS_HAS_MORE_OUTPUT;
                    (*r).m_state = 53 as mz_uint32;
                    current_block = 11953489883170298063;
                    continue;
                } else {
                    let fresh32 = dist_from_out_buf_start;
                    dist_from_out_buf_start = dist_from_out_buf_start.wrapping_add(1);
                    let fresh33 = pOut_buf_cur;
                    pOut_buf_cur = pOut_buf_cur.offset(1);
                    *fresh33 = *pOut_buf_start.offset(
                        (fresh32.wrapping_sub(dist as size_t) & out_buf_size_mask) as isize,
                    );
                }
                current_block = 18076970406079194302;
            }
            13286354103097804402 => {
                status = TINFL_STATUS_FAILED;
                (*r).m_state = 37 as mz_uint32;
                current_block = 11953489883170298063;
                continue;
            }
            1833465351478693723 => {
                if pIn_buf_cur >= pIn_buf_end {
                    status = (if decomp_flags
                        & TINFL_FLAG_HAS_MORE_INPUT as ::core::ffi::c_int as mz_uint32
                        != 0
                    {
                        TINFL_STATUS_NEEDS_MORE_INPUT as ::core::ffi::c_int
                    } else {
                        TINFL_STATUS_FAILED_CANNOT_MAKE_PROGRESS as ::core::ffi::c_int
                    }) as tinfl_status;
                    (*r).m_state = 27 as mz_uint32;
                    current_block = 11953489883170298063;
                    continue;
                } else {
                    let fresh30 = pIn_buf_cur;
                    pIn_buf_cur = pIn_buf_cur.offset(1);
                    c_10 = *fresh30 as mz_uint;
                    bit_buf |= (c_10 as tinfl_bit_buf_t) << num_bits;
                    num_bits = num_bits.wrapping_add(8 as mz_uint32);
                    if num_bits < num_extra {
                        current_block = 14412960601270604433;
                    } else {
                        current_block = 18060968742384277177;
                    }
                }
            }
            6721946588916655032 => {
                if pIn_buf_cur >= pIn_buf_end {
                    status = (if decomp_flags
                        & TINFL_FLAG_HAS_MORE_INPUT as ::core::ffi::c_int as mz_uint32
                        != 0
                    {
                        TINFL_STATUS_NEEDS_MORE_INPUT as ::core::ffi::c_int
                    } else {
                        TINFL_STATUS_FAILED_CANNOT_MAKE_PROGRESS as ::core::ffi::c_int
                    }) as tinfl_status;
                    (*r).m_state = 26 as mz_uint32;
                    current_block = 11953489883170298063;
                    continue;
                } else {
                    let fresh28 = pIn_buf_cur;
                    pIn_buf_cur = pIn_buf_cur.offset(1);
                    c_9 = *fresh28 as mz_uint;
                    bit_buf |= (c_9 as tinfl_bit_buf_t) << num_bits;
                    num_bits = num_bits.wrapping_add(8 as mz_uint32);
                    if num_bits < 15 as mz_uint32 {
                        current_block = 3011706931690025809;
                    } else {
                        current_block = 2358125959230175008;
                    }
                }
            }
            8650459447085150701 => {
                if pIn_buf_cur >= pIn_buf_end {
                    status = (if decomp_flags
                        & TINFL_FLAG_HAS_MORE_INPUT as ::core::ffi::c_int as mz_uint32
                        != 0
                    {
                        TINFL_STATUS_NEEDS_MORE_INPUT as ::core::ffi::c_int
                    } else {
                        TINFL_STATUS_FAILED_CANNOT_MAKE_PROGRESS as ::core::ffi::c_int
                    }) as tinfl_status;
                    (*r).m_state = 25 as mz_uint32;
                    current_block = 11953489883170298063;
                    continue;
                } else {
                    let fresh26 = pIn_buf_cur;
                    pIn_buf_cur = pIn_buf_cur.offset(1);
                    c_8 = *fresh26 as mz_uint;
                    bit_buf |= (c_8 as tinfl_bit_buf_t) << num_bits;
                    num_bits = num_bits.wrapping_add(8 as mz_uint32);
                    if num_bits < num_extra {
                        current_block = 3846997171290157944;
                    } else {
                        current_block = 17809027598452024276;
                    }
                }
            }
            4979785777352049371 => {
                status = TINFL_STATUS_FAILED;
                (*r).m_state = 54 as mz_uint32;
                current_block = 11953489883170298063;
                continue;
            }
            17058499098102203106 => {
                status = TINFL_STATUS_FAILED;
                (*r).m_state = 40 as mz_uint32;
                current_block = 11953489883170298063;
                continue;
            }
            12916373135125519917 => {
                if pOut_buf_cur >= pOut_buf_end {
                    status = TINFL_STATUS_HAS_MORE_OUTPUT;
                    (*r).m_state = 24 as mz_uint32;
                    current_block = 11953489883170298063;
                    continue;
                } else {
                    let fresh23 = pOut_buf_cur;
                    pOut_buf_cur = pOut_buf_cur.offset(1);
                    *fresh23 = counter as mz_uint8;
                }
                current_block = 18019479878307254118;
            }
            9080448346277817863 => {
                if pIn_buf_cur >= pIn_buf_end {
                    status = (if decomp_flags
                        & TINFL_FLAG_HAS_MORE_INPUT as ::core::ffi::c_int as mz_uint32
                        != 0
                    {
                        TINFL_STATUS_NEEDS_MORE_INPUT as ::core::ffi::c_int
                    } else {
                        TINFL_STATUS_FAILED_CANNOT_MAKE_PROGRESS as ::core::ffi::c_int
                    }) as tinfl_status;
                    (*r).m_state = 23 as mz_uint32;
                    current_block = 11953489883170298063;
                    continue;
                } else {
                    let fresh21 = pIn_buf_cur;
                    pIn_buf_cur = pIn_buf_cur.offset(1);
                    c_7 = *fresh21 as mz_uint;
                    bit_buf |= (c_7 as tinfl_bit_buf_t) << num_bits;
                    num_bits = num_bits.wrapping_add(8 as mz_uint32);
                    if num_bits < 15 as mz_uint32 {
                        current_block = 17870985093275900527;
                    } else {
                        current_block = 15415362524153386998;
                    }
                }
            }
            12060859132547941340 => {
                status = TINFL_STATUS_FAILED;
                (*r).m_state = 21 as mz_uint32;
                current_block = 11953489883170298063;
                continue;
            }
            778403239630769426 => {
                if pIn_buf_cur >= pIn_buf_end {
                    status = (if decomp_flags
                        & TINFL_FLAG_HAS_MORE_INPUT as ::core::ffi::c_int as mz_uint32
                        != 0
                    {
                        TINFL_STATUS_NEEDS_MORE_INPUT as ::core::ffi::c_int
                    } else {
                        TINFL_STATUS_FAILED_CANNOT_MAKE_PROGRESS as ::core::ffi::c_int
                    }) as tinfl_status;
                    (*r).m_state = 18 as mz_uint32;
                    current_block = 11953489883170298063;
                    continue;
                } else {
                    let fresh19 = pIn_buf_cur;
                    pIn_buf_cur = pIn_buf_cur.offset(1);
                    c_6 = *fresh19 as mz_uint;
                    bit_buf |= (c_6 as tinfl_bit_buf_t) << num_bits;
                    num_bits = num_bits.wrapping_add(8 as mz_uint32);
                    if num_bits < num_extra {
                        current_block = 3983074877498205967;
                    } else {
                        current_block = 8752412851861032177;
                    }
                }
            }
            15726232576006602401 => {
                status = TINFL_STATUS_FAILED;
                (*r).m_state = 17 as mz_uint32;
                current_block = 11953489883170298063;
                continue;
            }
            11990966878772509462 => {
                if pIn_buf_cur >= pIn_buf_end {
                    status = (if decomp_flags
                        & TINFL_FLAG_HAS_MORE_INPUT as ::core::ffi::c_int as mz_uint32
                        != 0
                    {
                        TINFL_STATUS_NEEDS_MORE_INPUT as ::core::ffi::c_int
                    } else {
                        TINFL_STATUS_FAILED_CANNOT_MAKE_PROGRESS as ::core::ffi::c_int
                    }) as tinfl_status;
                    (*r).m_state = 16 as mz_uint32;
                    current_block = 11953489883170298063;
                    continue;
                } else {
                    let fresh16 = pIn_buf_cur;
                    pIn_buf_cur = pIn_buf_cur.offset(1);
                    c_5 = *fresh16 as mz_uint;
                    bit_buf |= (c_5 as tinfl_bit_buf_t) << num_bits;
                    num_bits = num_bits.wrapping_add(8 as mz_uint32);
                    if num_bits < 15 as mz_uint32 {
                        current_block = 3179191505073644361;
                    } else {
                        current_block = 11867310137400648068;
                    }
                }
            }
            14308887607299961996 => {
                status = TINFL_STATUS_FAILED;
                (*r).m_state = 35 as mz_uint32;
                current_block = 11953489883170298063;
                continue;
            }
            7510843268513837357 => {
                if pIn_buf_cur >= pIn_buf_end {
                    status = (if decomp_flags
                        & TINFL_FLAG_HAS_MORE_INPUT as ::core::ffi::c_int as mz_uint32
                        != 0
                    {
                        TINFL_STATUS_NEEDS_MORE_INPUT as ::core::ffi::c_int
                    } else {
                        TINFL_STATUS_FAILED_CANNOT_MAKE_PROGRESS as ::core::ffi::c_int
                    }) as tinfl_status;
                    (*r).m_state = 14 as mz_uint32;
                    current_block = 11953489883170298063;
                    continue;
                } else {
                    let fresh13 = pIn_buf_cur;
                    pIn_buf_cur = pIn_buf_cur.offset(1);
                    c_4 = *fresh13 as mz_uint;
                    bit_buf |= (c_4 as tinfl_bit_buf_t) << num_bits;
                    num_bits = num_bits.wrapping_add(8 as mz_uint32);
                    if num_bits < 3 as ::core::ffi::c_int as mz_uint {
                        current_block = 11002855972428369642;
                    } else {
                        current_block = 9239588423676249671;
                    }
                }
            }
            12387625063048049585 => {
                if pIn_buf_cur >= pIn_buf_end {
                    status = (if decomp_flags
                        & TINFL_FLAG_HAS_MORE_INPUT as ::core::ffi::c_int as mz_uint32
                        != 0
                    {
                        TINFL_STATUS_NEEDS_MORE_INPUT as ::core::ffi::c_int
                    } else {
                        TINFL_STATUS_FAILED_CANNOT_MAKE_PROGRESS as ::core::ffi::c_int
                    }) as tinfl_status;
                    (*r).m_state = 11 as mz_uint32;
                    current_block = 11953489883170298063;
                    continue;
                } else {
                    let fresh12 = pIn_buf_cur;
                    pIn_buf_cur = pIn_buf_cur.offset(1);
                    c_3 = *fresh12 as mz_uint;
                    bit_buf |= (c_3 as tinfl_bit_buf_t) << num_bits;
                    num_bits = num_bits.wrapping_add(8 as mz_uint32);
                    if num_bits
                        < ::core::mem::transmute::<[u8; 4], [::core::ffi::c_char; 4]>(
                            *b"\x05\x05\x04\0",
                        )[counter as usize] as mz_uint
                    {
                        current_block = 14535175905252782758;
                    } else {
                        current_block = 10882604075759663216;
                    }
                }
            }
            18316056106135622027 => {
                status = TINFL_STATUS_FAILED;
                (*r).m_state = 10 as mz_uint32;
                current_block = 11953489883170298063;
                continue;
            }
            5908482871227205451 => {
                if pIn_buf_cur >= pIn_buf_end {
                    status = (if decomp_flags
                        & TINFL_FLAG_HAS_MORE_INPUT as ::core::ffi::c_int as mz_uint32
                        != 0
                    {
                        TINFL_STATUS_NEEDS_MORE_INPUT as ::core::ffi::c_int
                    } else {
                        TINFL_STATUS_FAILED_CANNOT_MAKE_PROGRESS as ::core::ffi::c_int
                    }) as tinfl_status;
                    (*r).m_state = 38 as mz_uint32;
                    current_block = 11953489883170298063;
                    continue;
                } else {
                    n = if (if (pOut_buf_end.offset_from(pOut_buf_cur) as ::core::ffi::c_long
                        as size_t)
                        < pIn_buf_end.offset_from(pIn_buf_cur) as ::core::ffi::c_long as size_t
                    {
                        pOut_buf_end.offset_from(pOut_buf_cur) as ::core::ffi::c_long as size_t
                    } else {
                        pIn_buf_end.offset_from(pIn_buf_cur) as ::core::ffi::c_long as size_t
                    }) < counter as size_t
                    {
                        if (pOut_buf_end.offset_from(pOut_buf_cur) as ::core::ffi::c_long as size_t)
                            < pIn_buf_end.offset_from(pIn_buf_cur) as ::core::ffi::c_long as size_t
                        {
                            pOut_buf_end.offset_from(pOut_buf_cur) as ::core::ffi::c_long as size_t
                        } else {
                            pIn_buf_end.offset_from(pIn_buf_cur) as ::core::ffi::c_long as size_t
                        }
                    } else {
                        counter as size_t
                    };
                    memcpy(
                        pOut_buf_cur as *mut ::core::ffi::c_void,
                        pIn_buf_cur as *const ::core::ffi::c_void,
                        n,
                    );
                    pIn_buf_cur = pIn_buf_cur.offset(n as isize);
                    pOut_buf_cur = pOut_buf_cur.offset(n as isize);
                    counter = (counter as uint32_t).wrapping_sub(n as mz_uint as uint32_t)
                        as mz_uint32 as mz_uint32;
                }
                current_block = 10661265453436690952;
            }
            15614898248724990345 => {
                if !(pOut_buf_cur >= pOut_buf_end) {
                    current_block = 5908482871227205451;
                    continue;
                }
                status = TINFL_STATUS_HAS_MORE_OUTPUT;
                (*r).m_state = 9 as mz_uint32;
                current_block = 11953489883170298063;
                continue;
            }
            15460309861373144675 => {
                if pOut_buf_cur >= pOut_buf_end {
                    status = TINFL_STATUS_HAS_MORE_OUTPUT;
                    (*r).m_state = 52 as mz_uint32;
                    current_block = 11953489883170298063;
                    continue;
                } else {
                    let fresh7 = pOut_buf_cur;
                    pOut_buf_cur = pOut_buf_cur.offset(1);
                    *fresh7 = dist as mz_uint8;
                    counter = counter.wrapping_sub(1);
                }
                current_block = 10041771570435381152;
            }
            13014351284863956202 => {
                if pIn_buf_cur >= pIn_buf_end {
                    status = (if decomp_flags
                        & TINFL_FLAG_HAS_MORE_INPUT as ::core::ffi::c_int as mz_uint32
                        != 0
                    {
                        TINFL_STATUS_NEEDS_MORE_INPUT as ::core::ffi::c_int
                    } else {
                        TINFL_STATUS_FAILED_CANNOT_MAKE_PROGRESS as ::core::ffi::c_int
                    }) as tinfl_status;
                    (*r).m_state = 51 as mz_uint32;
                    current_block = 11953489883170298063;
                    continue;
                } else {
                    let fresh6 = pIn_buf_cur;
                    pIn_buf_cur = pIn_buf_cur.offset(1);
                    c_2 = *fresh6 as mz_uint;
                    bit_buf |= (c_2 as tinfl_bit_buf_t) << num_bits;
                    num_bits = num_bits.wrapping_add(8 as mz_uint32);
                    if num_bits < 8 as ::core::ffi::c_int as mz_uint {
                        current_block = 14648249180243006330;
                    } else {
                        current_block = 12625034345703806147;
                    }
                }
            }
            10863493864285401582 => {
                status = TINFL_STATUS_FAILED;
                (*r).m_state = 39 as mz_uint32;
                current_block = 11953489883170298063;
                continue;
            }
            1428307939028130064 => {
                if pIn_buf_cur >= pIn_buf_end {
                    status = (if decomp_flags
                        & TINFL_FLAG_HAS_MORE_INPUT as ::core::ffi::c_int as mz_uint32
                        != 0
                    {
                        TINFL_STATUS_NEEDS_MORE_INPUT as ::core::ffi::c_int
                    } else {
                        TINFL_STATUS_FAILED_CANNOT_MAKE_PROGRESS as ::core::ffi::c_int
                    }) as tinfl_status;
                    (*r).m_state = 7 as mz_uint32;
                    current_block = 11953489883170298063;
                    continue;
                } else {
                    let fresh5 = pIn_buf_cur;
                    pIn_buf_cur = pIn_buf_cur.offset(1);
                    (*r).m_raw_header[counter as usize] = *fresh5;
                }
                current_block = 993425571616822999;
            }
            15237655884915618618 => {
                if pIn_buf_cur >= pIn_buf_end {
                    status = (if decomp_flags
                        & TINFL_FLAG_HAS_MORE_INPUT as ::core::ffi::c_int as mz_uint32
                        != 0
                    {
                        TINFL_STATUS_NEEDS_MORE_INPUT as ::core::ffi::c_int
                    } else {
                        TINFL_STATUS_FAILED_CANNOT_MAKE_PROGRESS as ::core::ffi::c_int
                    }) as tinfl_status;
                    (*r).m_state = 6 as mz_uint32;
                    current_block = 11953489883170298063;
                    continue;
                } else {
                    let fresh4 = pIn_buf_cur;
                    pIn_buf_cur = pIn_buf_cur.offset(1);
                    c_1 = *fresh4 as mz_uint;
                    bit_buf |= (c_1 as tinfl_bit_buf_t) << num_bits;
                    num_bits = num_bits.wrapping_add(8 as mz_uint32);
                    if num_bits < 8 as ::core::ffi::c_int as mz_uint {
                        current_block = 6838274324784804404;
                    } else {
                        current_block = 2467484839200770573;
                    }
                }
            }
            16779030619667747692 => {
                if pIn_buf_cur >= pIn_buf_end {
                    status = (if decomp_flags
                        & TINFL_FLAG_HAS_MORE_INPUT as ::core::ffi::c_int as mz_uint32
                        != 0
                    {
                        TINFL_STATUS_NEEDS_MORE_INPUT as ::core::ffi::c_int
                    } else {
                        TINFL_STATUS_FAILED_CANNOT_MAKE_PROGRESS as ::core::ffi::c_int
                    }) as tinfl_status;
                    (*r).m_state = 5 as mz_uint32;
                    current_block = 11953489883170298063;
                    continue;
                } else {
                    let fresh3 = pIn_buf_cur;
                    pIn_buf_cur = pIn_buf_cur.offset(1);
                    c_0 = *fresh3 as mz_uint;
                    bit_buf |= (c_0 as tinfl_bit_buf_t) << num_bits;
                    num_bits = num_bits.wrapping_add(8 as mz_uint32);
                    if num_bits < num_bits & 7 as mz_uint32 {
                        current_block = 1874315696050160458;
                    } else {
                        current_block = 13723035087248630346;
                    }
                }
            }
            5891011138178424807 => {
                if pIn_buf_cur >= pIn_buf_end {
                    status = (if decomp_flags
                        & TINFL_FLAG_HAS_MORE_INPUT as ::core::ffi::c_int as mz_uint32
                        != 0
                    {
                        TINFL_STATUS_NEEDS_MORE_INPUT as ::core::ffi::c_int
                    } else {
                        TINFL_STATUS_FAILED_CANNOT_MAKE_PROGRESS as ::core::ffi::c_int
                    }) as tinfl_status;
                    (*r).m_state = 3 as mz_uint32;
                    current_block = 11953489883170298063;
                    continue;
                } else {
                    let fresh2 = pIn_buf_cur;
                    pIn_buf_cur = pIn_buf_cur.offset(1);
                    c = *fresh2 as mz_uint;
                    bit_buf |= (c as tinfl_bit_buf_t) << num_bits;
                    num_bits = num_bits.wrapping_add(8 as mz_uint32);
                    if num_bits < 3 as ::core::ffi::c_int as mz_uint {
                        current_block = 13460095289871124136;
                    } else {
                        current_block = 6545907279487748450;
                    }
                }
            }
            13250426973517230943 => {
                if pIn_buf_cur >= pIn_buf_end {
                    status = (if decomp_flags
                        & TINFL_FLAG_HAS_MORE_INPUT as ::core::ffi::c_int as mz_uint32
                        != 0
                    {
                        TINFL_STATUS_NEEDS_MORE_INPUT as ::core::ffi::c_int
                    } else {
                        TINFL_STATUS_FAILED_CANNOT_MAKE_PROGRESS as ::core::ffi::c_int
                    }) as tinfl_status;
                    (*r).m_state = 42 as mz_uint32;
                    current_block = 11953489883170298063;
                    continue;
                } else {
                    let fresh36 = pIn_buf_cur;
                    pIn_buf_cur = pIn_buf_cur.offset(1);
                    s_1 = *fresh36 as mz_uint;
                }
                current_block = 12800094729615117303;
            }
            9997576841122142810 => {
                status = TINFL_STATUS_DONE;
                (*r).m_state = 34 as mz_uint32;
                current_block = 11953489883170298063;
                continue;
            }
            11953489883170298063 => {
                if status as ::core::ffi::c_int
                    != TINFL_STATUS_NEEDS_MORE_INPUT as ::core::ffi::c_int
                    && status as ::core::ffi::c_int
                        != TINFL_STATUS_FAILED_CANNOT_MAKE_PROGRESS as ::core::ffi::c_int
                {
                    while pIn_buf_cur > pIn_buf_next && num_bits >= 8 as mz_uint32 {
                        pIn_buf_cur = pIn_buf_cur.offset(-1);
                        num_bits = num_bits.wrapping_sub(8 as mz_uint32);
                    }
                }
                break;
            }
            _ => {
                if pIn_buf_cur >= pIn_buf_end {
                    status = (if decomp_flags
                        & TINFL_FLAG_HAS_MORE_INPUT as ::core::ffi::c_int as mz_uint32
                        != 0
                    {
                        TINFL_STATUS_NEEDS_MORE_INPUT as ::core::ffi::c_int
                    } else {
                        TINFL_STATUS_FAILED_CANNOT_MAKE_PROGRESS as ::core::ffi::c_int
                    }) as tinfl_status;
                    (*r).m_state = 41 as mz_uint32;
                    current_block = 11953489883170298063;
                    continue;
                } else {
                    let fresh35 = pIn_buf_cur;
                    pIn_buf_cur = pIn_buf_cur.offset(1);
                    c_12 = *fresh35 as mz_uint;
                    bit_buf |= (c_12 as tinfl_bit_buf_t) << num_bits;
                    num_bits = num_bits.wrapping_add(8 as mz_uint32);
                    if num_bits < 8 as ::core::ffi::c_int as mz_uint {
                        current_block = 8585542996221928953;
                    } else {
                        current_block = 2977775588433926224;
                    }
                }
            }
        }
        match current_block {
            6545907279487748450 => {
                (*r).m_final = (bit_buf
                    & (((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
                        - 1 as ::core::ffi::c_int) as tinfl_bit_buf_t)
                    as mz_uint32;
                bit_buf >>= 3 as ::core::ffi::c_int;
                num_bits = num_bits.wrapping_sub(3 as mz_uint32);
                (*r).m_type = (*r).m_final >> 1 as ::core::ffi::c_int;
                if (*r).m_type == 0 as mz_uint32 {
                    if num_bits < num_bits & 7 as mz_uint32 {
                        current_block = 1874315696050160458;
                    } else {
                        current_block = 13723035087248630346;
                    }
                } else {
                    if (*r).m_type == 3 as mz_uint32 {
                        current_block = 18316056106135622027;
                        continue;
                    }
                    if (*r).m_type == 1 as mz_uint32 {
                        let mut p: *mut mz_uint8 = &raw mut (*r).m_code_size_0 as *mut mz_uint8;
                        let mut i: mz_uint = 0;
                        (*r).m_table_sizes[0 as ::core::ffi::c_int as usize] = 288 as mz_uint32;
                        (*r).m_table_sizes[1 as ::core::ffi::c_int as usize] = 32 as mz_uint32;
                        memset(
                            &raw mut (*r).m_code_size_1 as *mut mz_uint8
                                as *mut ::core::ffi::c_void,
                            5 as ::core::ffi::c_int,
                            32 as size_t,
                        );
                        i = 0 as mz_uint;
                        while i <= 143 as mz_uint {
                            let fresh8 = p;
                            p = p.offset(1);
                            *fresh8 = 8 as mz_uint8;
                            i = i.wrapping_add(1);
                        }
                        while i <= 255 as mz_uint {
                            let fresh9 = p;
                            p = p.offset(1);
                            *fresh9 = 9 as mz_uint8;
                            i = i.wrapping_add(1);
                        }
                        while i <= 279 as mz_uint {
                            let fresh10 = p;
                            p = p.offset(1);
                            *fresh10 = 7 as mz_uint8;
                            i = i.wrapping_add(1);
                        }
                        while i <= 287 as mz_uint {
                            let fresh11 = p;
                            p = p.offset(1);
                            *fresh11 = 8 as mz_uint8;
                            i = i.wrapping_add(1);
                        }
                        current_block = 17723783950272744135;
                    } else {
                        counter = 0 as mz_uint32;
                        current_block = 3414715837273685534;
                    }
                }
            }
            13460095289871124136 => {
                c = 0;
                current_block = 5891011138178424807;
                continue;
            }
            _ => {}
        }
        match current_block {
            13723035087248630346 => {
                bit_buf >>= num_bits & 7 as mz_uint32;
                num_bits = num_bits.wrapping_sub(num_bits & 7 as mz_uint32);
                counter = 0 as mz_uint32;
                current_block = 17485376261910781866;
            }
            1874315696050160458 => {
                c_0 = 0;
                current_block = 16779030619667747692;
                continue;
            }
            _ => {}
        }
        loop {
            match current_block {
                6838274324784804404 => {
                    c_1 = 0;
                    current_block = 15237655884915618618;
                    break;
                }
                955764296456093747 => {
                    bit_buf >>= num_bits & 7 as mz_uint32;
                    num_bits = num_bits.wrapping_sub(num_bits & 7 as mz_uint32);
                    while pIn_buf_cur > pIn_buf_next && num_bits >= 8 as mz_uint32 {
                        pIn_buf_cur = pIn_buf_cur.offset(-1);
                        num_bits = num_bits.wrapping_sub(8 as mz_uint32);
                    }
                    bit_buf &= !(!(0 as ::core::ffi::c_int as tinfl_bit_buf_t) << num_bits);
                    if !(decomp_flags
                        & TINFL_FLAG_PARSE_ZLIB_HEADER as ::core::ffi::c_int as mz_uint32
                        != 0)
                    {
                        current_block = 9997576841122142810;
                        break;
                    }
                    counter = 0 as mz_uint32;
                    current_block = 7757638761803742862;
                }
                18060968742384277177 => {
                    extra_bits_0 = (bit_buf
                        & (((1 as ::core::ffi::c_int) << num_extra) - 1 as ::core::ffi::c_int)
                            as tinfl_bit_buf_t) as mz_uint;
                    bit_buf >>= num_extra;
                    num_bits = num_bits.wrapping_sub(num_extra);
                    dist = (dist as uint32_t).wrapping_add(extra_bits_0 as uint32_t) as mz_uint32
                        as mz_uint32;
                    current_block = 14117028633286690613;
                }
                2358125959230175008 => {
                    temp_1 = (*r).m_look_up[1 as ::core::ffi::c_int as usize][(bit_buf
                        & (TINFL_FAST_LOOKUP_SIZE as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                            as tinfl_bit_buf_t)
                        as usize] as ::core::ffi::c_int;
                    if temp_1 >= 0 as ::core::ffi::c_int {
                        code_len_2 = (temp_1 >> 9 as ::core::ffi::c_int) as mz_uint;
                        temp_1 &= 511 as ::core::ffi::c_int;
                    } else {
                        code_len_2 = TINFL_FAST_LOOKUP_BITS as ::core::ffi::c_int as mz_uint;
                        loop {
                            let fresh29 = code_len_2;
                            code_len_2 = code_len_2.wrapping_add(1);
                            temp_1 = (*r).m_tree_1[(!temp_1 as tinfl_bit_buf_t)
                                .wrapping_add(bit_buf >> fresh29 & 1 as tinfl_bit_buf_t)
                                as usize]
                                as ::core::ffi::c_int;
                            if !(temp_1 < 0 as ::core::ffi::c_int) {
                                break;
                            }
                        }
                    }
                    dist = temp_1 as mz_uint32;
                    bit_buf >>= code_len_2;
                    num_bits = (num_bits as uint32_t).wrapping_sub(code_len_2 as uint32_t)
                        as mz_uint32 as mz_uint32;
                    num_extra = s_dist_extra[dist as usize] as mz_uint32;
                    dist = s_dist_base[dist as usize] as mz_uint32;
                    if num_extra != 0 {
                        extra_bits_0 = 0;
                        if num_bits < num_extra {
                            current_block = 14412960601270604433;
                            continue;
                        } else {
                            current_block = 18060968742384277177;
                            continue;
                        }
                    } else {
                        current_block = 14117028633286690613;
                    }
                }
                3011706931690025809 => {
                    temp_1 = (*r).m_look_up[1 as ::core::ffi::c_int as usize][(bit_buf
                        & (TINFL_FAST_LOOKUP_SIZE as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                            as tinfl_bit_buf_t)
                        as usize] as ::core::ffi::c_int;
                    if temp_1 >= 0 as ::core::ffi::c_int {
                        code_len_2 = (temp_1 >> 9 as ::core::ffi::c_int) as mz_uint;
                        if code_len_2 != 0 && num_bits >= code_len_2 {
                            current_block = 2358125959230175008;
                            continue;
                        } else {
                            current_block = 6721946588916655032;
                            break;
                        }
                    } else {
                        if !(num_bits > TINFL_FAST_LOOKUP_BITS as ::core::ffi::c_int as mz_uint32) {
                            current_block = 6721946588916655032;
                            break;
                        }
                        code_len_2 = TINFL_FAST_LOOKUP_BITS as ::core::ffi::c_int as mz_uint;
                        loop {
                            let fresh27 = code_len_2;
                            code_len_2 = code_len_2.wrapping_add(1);
                            temp_1 = (*r).m_tree_1[(!temp_1 as tinfl_bit_buf_t)
                                .wrapping_add(bit_buf >> fresh27 & 1 as tinfl_bit_buf_t)
                                as usize]
                                as ::core::ffi::c_int;
                            if !(temp_1 < 0 as ::core::ffi::c_int
                                && num_bits >= code_len_2.wrapping_add(1 as mz_uint))
                            {
                                break;
                            }
                        }
                        if temp_1 >= 0 as ::core::ffi::c_int {
                            current_block = 2358125959230175008;
                            continue;
                        } else {
                            current_block = 6721946588916655032;
                            break;
                        }
                    }
                }
                17809027598452024276 => {
                    extra_bits = (bit_buf
                        & (((1 as ::core::ffi::c_int) << num_extra) - 1 as ::core::ffi::c_int)
                            as tinfl_bit_buf_t) as mz_uint;
                    bit_buf >>= num_extra;
                    num_bits = num_bits.wrapping_sub(num_extra);
                    counter = (counter as uint32_t).wrapping_add(extra_bits as uint32_t)
                        as mz_uint32 as mz_uint32;
                    current_block = 18029556747650706958;
                }
                18019479878307254118 => {
                    if (pIn_buf_end.offset_from(pIn_buf_cur) as ::core::ffi::c_long)
                        < 4 as ::core::ffi::c_long
                        || (pOut_buf_end.offset_from(pOut_buf_cur) as ::core::ffi::c_long)
                            < 2 as ::core::ffi::c_long
                    {
                        temp_0 = 0;
                        code_len_0 = 0;
                        c_7 = 0;
                        if !(num_bits < 15 as mz_uint32) {
                            current_block = 15415362524153386998;
                            continue;
                        }
                        if (pIn_buf_end.offset_from(pIn_buf_cur) as ::core::ffi::c_long)
                            < 2 as ::core::ffi::c_long
                        {
                            current_block = 17870985093275900527;
                            continue;
                        }
                        bit_buf |= (*pIn_buf_cur.offset(0 as ::core::ffi::c_int as isize)
                            as tinfl_bit_buf_t)
                            << num_bits
                            | (*pIn_buf_cur.offset(1 as ::core::ffi::c_int as isize)
                                as tinfl_bit_buf_t)
                                << num_bits.wrapping_add(8 as mz_uint32);
                        pIn_buf_cur = pIn_buf_cur.offset(2 as ::core::ffi::c_int as isize);
                        num_bits = num_bits.wrapping_add(16 as mz_uint32);
                        current_block = 15415362524153386998;
                        continue;
                    } else {
                        sym2 = 0;
                        code_len_1 = 0;
                        if num_bits < 30 as mz_uint32 {
                            bit_buf |= ((*pIn_buf_cur.offset(0 as ::core::ffi::c_int as isize)
                                as mz_uint32
                                | (*pIn_buf_cur.offset(1 as ::core::ffi::c_int as isize)
                                    as mz_uint32)
                                    << 8 as ::core::ffi::c_uint
                                | (*pIn_buf_cur.offset(2 as ::core::ffi::c_int as isize)
                                    as mz_uint32)
                                    << 16 as ::core::ffi::c_uint
                                | (*pIn_buf_cur.offset(3 as ::core::ffi::c_int as isize)
                                    as mz_uint32)
                                    << 24 as ::core::ffi::c_uint)
                                as tinfl_bit_buf_t)
                                << num_bits;
                            pIn_buf_cur = pIn_buf_cur.offset(4 as ::core::ffi::c_int as isize);
                            num_bits = num_bits.wrapping_add(32 as mz_uint32);
                        }
                        sym2 = (*r).m_look_up[0 as ::core::ffi::c_int as usize][(bit_buf
                            & (TINFL_FAST_LOOKUP_SIZE as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int)
                                as tinfl_bit_buf_t)
                            as usize] as ::core::ffi::c_int;
                        if sym2 >= 0 as ::core::ffi::c_int {
                            code_len_1 = (sym2 >> 9 as ::core::ffi::c_int) as mz_uint;
                        } else {
                            code_len_1 = TINFL_FAST_LOOKUP_BITS as ::core::ffi::c_int as mz_uint;
                            loop {
                                let fresh24 = code_len_1;
                                code_len_1 = code_len_1.wrapping_add(1);
                                sym2 = (*r).m_tree_0[(!sym2 as tinfl_bit_buf_t)
                                    .wrapping_add(bit_buf >> fresh24 & 1 as tinfl_bit_buf_t)
                                    as usize]
                                    as ::core::ffi::c_int;
                                if !(sym2 < 0 as ::core::ffi::c_int) {
                                    break;
                                }
                            }
                        }
                        counter = sym2 as mz_uint32;
                        bit_buf >>= code_len_1;
                        num_bits = (num_bits as uint32_t).wrapping_sub(code_len_1 as uint32_t)
                            as mz_uint32 as mz_uint32;
                        if code_len_1 == 0 as mz_uint {
                            current_block = 17058499098102203106;
                            break;
                        }
                        if !(counter & 256 as mz_uint32 != 0) {
                            sym2 = (*r).m_look_up[0 as ::core::ffi::c_int as usize][(bit_buf
                                & (TINFL_FAST_LOOKUP_SIZE as ::core::ffi::c_int
                                    - 1 as ::core::ffi::c_int)
                                    as tinfl_bit_buf_t)
                                as usize] as ::core::ffi::c_int;
                            if sym2 >= 0 as ::core::ffi::c_int {
                                code_len_1 = (sym2 >> 9 as ::core::ffi::c_int) as mz_uint;
                            } else {
                                code_len_1 =
                                    TINFL_FAST_LOOKUP_BITS as ::core::ffi::c_int as mz_uint;
                                loop {
                                    let fresh25 = code_len_1;
                                    code_len_1 = code_len_1.wrapping_add(1);
                                    sym2 = (*r).m_tree_0[(!sym2 as tinfl_bit_buf_t)
                                        .wrapping_add(bit_buf >> fresh25 & 1 as tinfl_bit_buf_t)
                                        as usize]
                                        as ::core::ffi::c_int;
                                    if !(sym2 < 0 as ::core::ffi::c_int) {
                                        break;
                                    }
                                }
                            }
                            bit_buf >>= code_len_1;
                            num_bits = (num_bits as uint32_t).wrapping_sub(code_len_1 as uint32_t)
                                as mz_uint32 as mz_uint32;
                            if code_len_1 == 0 as mz_uint {
                                current_block = 4979785777352049371;
                                break;
                            }
                            *pOut_buf_cur.offset(0 as ::core::ffi::c_int as isize) =
                                counter as mz_uint8;
                            if sym2 & 256 as ::core::ffi::c_int != 0 {
                                pOut_buf_cur = pOut_buf_cur.offset(1);
                                counter = sym2 as mz_uint32;
                            } else {
                                *pOut_buf_cur.offset(1 as ::core::ffi::c_int as isize) =
                                    sym2 as mz_uint8;
                                pOut_buf_cur =
                                    pOut_buf_cur.offset(2 as ::core::ffi::c_int as isize);
                                current_block = 18019479878307254118;
                                continue;
                            }
                        }
                    }
                    current_block = 12369472161353711912;
                }
                15415362524153386998 => {
                    temp_0 = (*r).m_look_up[0 as ::core::ffi::c_int as usize][(bit_buf
                        & (TINFL_FAST_LOOKUP_SIZE as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                            as tinfl_bit_buf_t)
                        as usize] as ::core::ffi::c_int;
                    if temp_0 >= 0 as ::core::ffi::c_int {
                        code_len_0 = (temp_0 >> 9 as ::core::ffi::c_int) as mz_uint;
                        temp_0 &= 511 as ::core::ffi::c_int;
                    } else {
                        code_len_0 = TINFL_FAST_LOOKUP_BITS as ::core::ffi::c_int as mz_uint;
                        loop {
                            let fresh22 = code_len_0;
                            code_len_0 = code_len_0.wrapping_add(1);
                            temp_0 = (*r).m_tree_0[(!temp_0 as tinfl_bit_buf_t)
                                .wrapping_add(bit_buf >> fresh22 & 1 as tinfl_bit_buf_t)
                                as usize]
                                as ::core::ffi::c_int;
                            if !(temp_0 < 0 as ::core::ffi::c_int) {
                                break;
                            }
                        }
                    }
                    counter = temp_0 as mz_uint32;
                    bit_buf >>= code_len_0;
                    num_bits = (num_bits as uint32_t).wrapping_sub(code_len_0 as uint32_t)
                        as mz_uint32 as mz_uint32;
                    if !(counter >= 256 as mz_uint32) {
                        current_block = 12916373135125519917;
                        break;
                    }
                    current_block = 12369472161353711912;
                }
                17870985093275900527 => {
                    temp_0 = (*r).m_look_up[0 as ::core::ffi::c_int as usize][(bit_buf
                        & (TINFL_FAST_LOOKUP_SIZE as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                            as tinfl_bit_buf_t)
                        as usize] as ::core::ffi::c_int;
                    if temp_0 >= 0 as ::core::ffi::c_int {
                        code_len_0 = (temp_0 >> 9 as ::core::ffi::c_int) as mz_uint;
                        if code_len_0 != 0 && num_bits >= code_len_0 {
                            current_block = 15415362524153386998;
                            continue;
                        } else {
                            current_block = 9080448346277817863;
                            break;
                        }
                    } else {
                        if !(num_bits > TINFL_FAST_LOOKUP_BITS as ::core::ffi::c_int as mz_uint32) {
                            current_block = 9080448346277817863;
                            break;
                        }
                        code_len_0 = TINFL_FAST_LOOKUP_BITS as ::core::ffi::c_int as mz_uint;
                        loop {
                            let fresh20 = code_len_0;
                            code_len_0 = code_len_0.wrapping_add(1);
                            temp_0 = (*r).m_tree_0[(!temp_0 as tinfl_bit_buf_t)
                                .wrapping_add(bit_buf >> fresh20 & 1 as tinfl_bit_buf_t)
                                as usize]
                                as ::core::ffi::c_int;
                            if !(temp_0 < 0 as ::core::ffi::c_int
                                && num_bits >= code_len_0.wrapping_add(1 as mz_uint))
                            {
                                break;
                            }
                        }
                        if temp_0 >= 0 as ::core::ffi::c_int {
                            current_block = 15415362524153386998;
                            continue;
                        } else {
                            current_block = 9080448346277817863;
                            break;
                        }
                    }
                }
                8752412851861032177 => {
                    s_0 = (bit_buf
                        & (((1 as ::core::ffi::c_int) << num_extra) - 1 as ::core::ffi::c_int)
                            as tinfl_bit_buf_t) as mz_uint;
                    bit_buf >>= num_extra;
                    num_bits = num_bits.wrapping_sub(num_extra);
                    s_0 = s_0.wrapping_add(
                        ::core::mem::transmute::<[u8; 4], [::core::ffi::c_char; 4]>(
                            *b"\x03\x03\x0B\0",
                        )[dist.wrapping_sub(16 as mz_uint32) as usize]
                            as mz_uint,
                    );
                    memset(
                        (&raw mut (*r).m_len_codes as *mut mz_uint8).offset(counter as isize)
                            as *mut ::core::ffi::c_void,
                        if dist == 16 as mz_uint32 {
                            (*r).m_len_codes[counter.wrapping_sub(1 as mz_uint32) as usize]
                                as ::core::ffi::c_int
                        } else {
                            0 as ::core::ffi::c_int
                        },
                        s_0 as size_t,
                    );
                    counter = (counter as uint32_t).wrapping_add(s_0 as uint32_t) as mz_uint32
                        as mz_uint32;
                    current_block = 15712984148872257586;
                }
                11867310137400648068 => {
                    temp = (*r).m_look_up[2 as ::core::ffi::c_int as usize][(bit_buf
                        & (TINFL_FAST_LOOKUP_SIZE as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                            as tinfl_bit_buf_t)
                        as usize] as ::core::ffi::c_int;
                    if temp >= 0 as ::core::ffi::c_int {
                        code_len = (temp >> 9 as ::core::ffi::c_int) as mz_uint;
                        temp &= 511 as ::core::ffi::c_int;
                    } else {
                        code_len = TINFL_FAST_LOOKUP_BITS as ::core::ffi::c_int as mz_uint;
                        loop {
                            let fresh17 = code_len;
                            code_len = code_len.wrapping_add(1);
                            temp = (*r).m_tree_2[(!temp as tinfl_bit_buf_t)
                                .wrapping_add(bit_buf >> fresh17 & 1 as tinfl_bit_buf_t)
                                as usize] as ::core::ffi::c_int;
                            if !(temp < 0 as ::core::ffi::c_int) {
                                break;
                            }
                        }
                    }
                    dist = temp as mz_uint32;
                    bit_buf >>= code_len;
                    num_bits = (num_bits as uint32_t).wrapping_sub(code_len as uint32_t)
                        as mz_uint32 as mz_uint32;
                    if dist < 16 as mz_uint32 {
                        let fresh18 = counter;
                        counter = counter.wrapping_add(1);
                        (*r).m_len_codes[fresh18 as usize] = dist as mz_uint8;
                    } else {
                        if dist == 16 as mz_uint32 && counter == 0 {
                            current_block = 15726232576006602401;
                            break;
                        }
                        num_extra = ::core::mem::transmute::<[u8; 4], [::core::ffi::c_char; 4]>(
                            *b"\x02\x03\x07\0",
                        )[dist.wrapping_sub(16 as mz_uint32) as usize]
                            as mz_uint32;
                        if num_bits < num_extra {
                            current_block = 3983074877498205967;
                            continue;
                        } else {
                            current_block = 8752412851861032177;
                            continue;
                        }
                    }
                    current_block = 15712984148872257586;
                }
                3179191505073644361 => {
                    temp = (*r).m_look_up[2 as ::core::ffi::c_int as usize][(bit_buf
                        & (TINFL_FAST_LOOKUP_SIZE as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                            as tinfl_bit_buf_t)
                        as usize] as ::core::ffi::c_int;
                    if temp >= 0 as ::core::ffi::c_int {
                        code_len = (temp >> 9 as ::core::ffi::c_int) as mz_uint;
                        if code_len != 0 && num_bits >= code_len {
                            current_block = 11867310137400648068;
                            continue;
                        } else {
                            current_block = 11990966878772509462;
                            break;
                        }
                    } else {
                        if !(num_bits > TINFL_FAST_LOOKUP_BITS as ::core::ffi::c_int as mz_uint32) {
                            current_block = 11990966878772509462;
                            break;
                        }
                        code_len = TINFL_FAST_LOOKUP_BITS as ::core::ffi::c_int as mz_uint;
                        loop {
                            let fresh15 = code_len;
                            code_len = code_len.wrapping_add(1);
                            temp = (*r).m_tree_2[(!temp as tinfl_bit_buf_t)
                                .wrapping_add(bit_buf >> fresh15 & 1 as tinfl_bit_buf_t)
                                as usize] as ::core::ffi::c_int;
                            if !(temp < 0 as ::core::ffi::c_int
                                && num_bits >= code_len.wrapping_add(1 as mz_uint))
                            {
                                break;
                            }
                        }
                        if temp >= 0 as ::core::ffi::c_int {
                            current_block = 11867310137400648068;
                            continue;
                        } else {
                            current_block = 11990966878772509462;
                            break;
                        }
                    }
                }
                17723783950272744135 => {
                    if (*r).m_type as ::core::ffi::c_int >= 0 as ::core::ffi::c_int {
                        tree_next = 0;
                        tree_cur = 0;
                        pLookUp = ::core::ptr::null_mut::<mz_int16>();
                        pTree = ::core::ptr::null_mut::<mz_int16>();
                        pCode_size = ::core::ptr::null_mut::<mz_uint8>();
                        i_0 = 0;
                        j = 0;
                        used_syms = 0;
                        total = 0;
                        sym_index = 0;
                        next_code = [0; 17];
                        total_syms = [0; 16];
                        pLookUp = &raw mut *(&raw mut (*r).m_look_up as *mut [mz_int16; 1024])
                            .offset((*r).m_type as isize)
                            as *mut mz_int16;
                        pTree = pTrees[(*r).m_type as usize];
                        pCode_size = pCode_sizes[(*r).m_type as usize];
                        memset(
                            &raw mut total_syms as *mut mz_uint as *mut ::core::ffi::c_void,
                            0 as ::core::ffi::c_int,
                            ::core::mem::size_of::<[mz_uint; 16]>() as size_t,
                        );
                        memset(
                            pLookUp as *mut ::core::ffi::c_void,
                            0 as ::core::ffi::c_int,
                            ::core::mem::size_of::<[mz_int16; 1024]>() as size_t,
                        );
                        tinfl_clear_tree(r);
                        i_0 = 0 as mz_uint;
                        while i_0 < (*r).m_table_sizes[(*r).m_type as usize] {
                            total_syms[*pCode_size.offset(i_0 as isize) as usize] = total_syms
                                [*pCode_size.offset(i_0 as isize) as usize]
                                .wrapping_add(1);
                            i_0 = i_0.wrapping_add(1);
                        }
                        used_syms = 0 as mz_uint;
                        total = 0 as mz_uint;
                        next_code[1 as ::core::ffi::c_int as usize] = 0 as mz_uint;
                        next_code[0 as ::core::ffi::c_int as usize] =
                            next_code[1 as ::core::ffi::c_int as usize];
                        i_0 = 1 as mz_uint;
                        while i_0 <= 15 as mz_uint {
                            used_syms = used_syms.wrapping_add(total_syms[i_0 as usize]);
                            total = total.wrapping_add(total_syms[i_0 as usize])
                                << 1 as ::core::ffi::c_int;
                            next_code[i_0.wrapping_add(1 as mz_uint) as usize] = total;
                            i_0 = i_0.wrapping_add(1);
                        }
                        if 65536 as mz_uint != total && used_syms > 1 as mz_uint {
                            current_block = 14308887607299961996;
                            break;
                        }
                        tree_next = -(1 as ::core::ffi::c_int);
                        sym_index = 0 as mz_uint;
                        while sym_index < (*r).m_table_sizes[(*r).m_type as usize] {
                            let mut rev_code: mz_uint = 0 as mz_uint;
                            let mut l: mz_uint = 0;
                            let mut cur_code: mz_uint = 0;
                            let mut code_size: mz_uint =
                                *pCode_size.offset(sym_index as isize) as mz_uint;
                            if !(code_size == 0) {
                                let fresh14 = next_code[code_size as usize];
                                next_code[code_size as usize] =
                                    next_code[code_size as usize].wrapping_add(1);
                                cur_code = fresh14;
                                l = code_size;
                                while l > 0 as mz_uint {
                                    rev_code = rev_code << 1 as ::core::ffi::c_int
                                        | cur_code & 1 as mz_uint;
                                    l = l.wrapping_sub(1);
                                    cur_code >>= 1 as ::core::ffi::c_int;
                                }
                                if code_size
                                    <= TINFL_FAST_LOOKUP_BITS as ::core::ffi::c_int as mz_uint
                                {
                                    let mut k: mz_int16 = (code_size << 9 as ::core::ffi::c_int
                                        | sym_index)
                                        as mz_int16;
                                    while rev_code
                                        < TINFL_FAST_LOOKUP_SIZE as ::core::ffi::c_int as mz_uint
                                    {
                                        *pLookUp.offset(rev_code as isize) = k;
                                        rev_code = rev_code.wrapping_add(
                                            ((1 as ::core::ffi::c_int) << code_size) as mz_uint,
                                        );
                                    }
                                } else {
                                    tree_cur = *pLookUp.offset(
                                        (rev_code
                                            & (TINFL_FAST_LOOKUP_SIZE as ::core::ffi::c_int
                                                - 1 as ::core::ffi::c_int)
                                                as mz_uint)
                                            as isize,
                                    )
                                        as ::core::ffi::c_int;
                                    if 0 as ::core::ffi::c_int == tree_cur {
                                        *pLookUp.offset(
                                            (rev_code
                                                & (TINFL_FAST_LOOKUP_SIZE as ::core::ffi::c_int
                                                    - 1 as ::core::ffi::c_int)
                                                    as mz_uint)
                                                as isize,
                                        ) = tree_next as mz_int16;
                                        tree_cur = tree_next;
                                        tree_next -= 2 as ::core::ffi::c_int;
                                    }
                                    rev_code >>= TINFL_FAST_LOOKUP_BITS as ::core::ffi::c_int
                                        - 1 as ::core::ffi::c_int;
                                    j = code_size;
                                    while j
                                        > (TINFL_FAST_LOOKUP_BITS as ::core::ffi::c_int
                                            + 1 as ::core::ffi::c_int)
                                            as mz_uint
                                    {
                                        rev_code >>= 1 as ::core::ffi::c_int;
                                        tree_cur = (tree_cur as mz_uint)
                                            .wrapping_sub(rev_code & 1 as mz_uint)
                                            as ::core::ffi::c_int
                                            as ::core::ffi::c_int;
                                        if *pTree
                                            .offset((-tree_cur - 1 as ::core::ffi::c_int) as isize)
                                            == 0
                                        {
                                            *pTree.offset(
                                                (-tree_cur - 1 as ::core::ffi::c_int) as isize,
                                            ) = tree_next as mz_int16;
                                            tree_cur = tree_next;
                                            tree_next -= 2 as ::core::ffi::c_int;
                                        } else {
                                            tree_cur = *pTree.offset(
                                                (-tree_cur - 1 as ::core::ffi::c_int) as isize,
                                            )
                                                as ::core::ffi::c_int;
                                        }
                                        j = j.wrapping_sub(1);
                                    }
                                    rev_code >>= 1 as ::core::ffi::c_int;
                                    tree_cur = (tree_cur as mz_uint)
                                        .wrapping_sub(rev_code & 1 as mz_uint)
                                        as ::core::ffi::c_int
                                        as ::core::ffi::c_int;
                                    *pTree.offset((-tree_cur - 1 as ::core::ffi::c_int) as isize) =
                                        sym_index as mz_int16;
                                }
                            }
                            sym_index = sym_index.wrapping_add(1);
                        }
                        if (*r).m_type == 2 as mz_uint32 {
                            counter = 0 as mz_uint32;
                            current_block = 15712984148872257586;
                        } else {
                            current_block = 8038949400865391589;
                        }
                    } else {
                        current_block = 12926621472508904600;
                    }
                }
                9239588423676249671 => {
                    s = (bit_buf
                        & (((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
                            - 1 as ::core::ffi::c_int) as tinfl_bit_buf_t)
                        as mz_uint;
                    bit_buf >>= 3 as ::core::ffi::c_int;
                    num_bits = num_bits.wrapping_sub(3 as mz_uint32);
                    (*r).m_code_size_2[s_length_dezigzag[counter as usize] as usize] =
                        s as mz_uint8;
                    counter = counter.wrapping_add(1);
                    current_block = 5638484086777907119;
                }
                3414715837273685534 => {
                    if counter < 3 as mz_uint32 {
                        if num_bits
                            < ::core::mem::transmute::<[u8; 4], [::core::ffi::c_char; 4]>(
                                *b"\x05\x05\x04\0",
                            )[counter as usize] as mz_uint
                        {
                            current_block = 14535175905252782758;
                            continue;
                        } else {
                            current_block = 10882604075759663216;
                            continue;
                        }
                    } else {
                        memset(
                            &raw mut (*r).m_code_size_2 as *mut mz_uint8
                                as *mut ::core::ffi::c_void,
                            0 as ::core::ffi::c_int,
                            ::core::mem::size_of::<[mz_uint8; 19]>() as size_t,
                        );
                        counter = 0 as mz_uint32;
                    }
                    current_block = 5638484086777907119;
                }
                10882604075759663216 => {
                    (*r).m_table_sizes[counter as usize] = (bit_buf
                        & (((1 as ::core::ffi::c_int)
                            << ::core::mem::transmute::<[u8; 4], [::core::ffi::c_char; 4]>(
                                *b"\x05\x05\x04\0",
                            )[counter as usize]
                                as ::core::ffi::c_int)
                            - 1 as ::core::ffi::c_int) as tinfl_bit_buf_t)
                        as mz_uint32;
                    bit_buf >>= ::core::mem::transmute::<[u8; 4], [::core::ffi::c_char; 4]>(
                        *b"\x05\x05\x04\0",
                    )[counter as usize] as ::core::ffi::c_int;
                    num_bits = num_bits.wrapping_sub(
                        ::core::mem::transmute::<[u8; 4], [::core::ffi::c_char; 4]>(
                            *b"\x05\x05\x04\0",
                        )[counter as usize] as mz_uint32,
                    );
                    (*r).m_table_sizes[counter as usize] = (*r).m_table_sizes[counter as usize]
                        .wrapping_add(s_min_table_sizes[counter as usize] as mz_uint32);
                    counter = counter.wrapping_add(1);
                    current_block = 3414715837273685534;
                    continue;
                }
                10661265453436690952 => {
                    if counter != 0 {
                        n = 0;
                        current_block = 15614898248724990345;
                        break;
                    } else {
                        current_block = 1345366029464561491;
                    }
                }
                10041771570435381152 => {
                    if !(counter != 0 && num_bits != 0) {
                        current_block = 10661265453436690952;
                        continue;
                    }
                    if num_bits < 8 as ::core::ffi::c_int as mz_uint {
                        current_block = 14648249180243006330;
                        continue;
                    } else {
                        current_block = 12625034345703806147;
                        continue;
                    }
                }
                2467484839200770573 => {
                    (*r).m_raw_header[counter as usize] = (bit_buf
                        & (((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)
                            - 1 as ::core::ffi::c_int) as tinfl_bit_buf_t)
                        as mz_uint8;
                    bit_buf >>= 8 as ::core::ffi::c_int;
                    num_bits = num_bits.wrapping_sub(8 as mz_uint32);
                    current_block = 993425571616822999;
                    continue;
                }
                17485376261910781866 => {
                    if counter < 4 as mz_uint32 {
                        if !(num_bits != 0) {
                            current_block = 1428307939028130064;
                            break;
                        }
                        if num_bits < 8 as ::core::ffi::c_int as mz_uint {
                            current_block = 6838274324784804404;
                            continue;
                        } else {
                            current_block = 2467484839200770573;
                            continue;
                        }
                    } else {
                        counter = ((*r).m_raw_header[0 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            | ((*r).m_raw_header[1 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int)
                                << 8 as ::core::ffi::c_int)
                            as mz_uint32;
                        if counter
                            != (0xffff as ::core::ffi::c_int
                                ^ ((*r).m_raw_header[2 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int
                                    | ((*r).m_raw_header[3 as ::core::ffi::c_int as usize]
                                        as ::core::ffi::c_int)
                                        << 8 as ::core::ffi::c_int))
                                as mz_uint
                        {
                            current_block = 10863493864285401582;
                            break;
                        } else {
                            current_block = 10041771570435381152;
                            continue;
                        }
                    }
                }
                2977775588433926224 => {
                    s_1 = (bit_buf
                        & (((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)
                            - 1 as ::core::ffi::c_int) as tinfl_bit_buf_t)
                        as mz_uint;
                    bit_buf >>= 8 as ::core::ffi::c_int;
                    num_bits = num_bits.wrapping_sub(8 as mz_uint32);
                    current_block = 12800094729615117303;
                    continue;
                }
                12800094729615117303 => {
                    (*r).m_z_adler32 =
                        (*r).m_z_adler32 << 8 as ::core::ffi::c_int | s_1 as mz_uint32;
                    counter = counter.wrapping_add(1);
                    current_block = 7757638761803742862;
                }
                8585542996221928953 => {
                    c_12 = 0;
                    current_block = 12381699995462811668;
                    break;
                }
                4159430576127784976 => {
                    c_11 = 0;
                    current_block = 14402387155550584875;
                    break;
                }
                18076970406079194302 => {
                    let fresh31 = counter;
                    counter = counter.wrapping_sub(1);
                    if fresh31 != 0 {
                        current_block = 7674033599746029549;
                        break;
                    }
                    current_block = 12926621472508904600;
                }
                14412960601270604433 => {
                    c_10 = 0;
                    current_block = 1833465351478693723;
                    break;
                }
                3846997171290157944 => {
                    c_8 = 0;
                    current_block = 8650459447085150701;
                    break;
                }
                3983074877498205967 => {
                    c_6 = 0;
                    current_block = 778403239630769426;
                    break;
                }
                11002855972428369642 => {
                    c_4 = 0;
                    current_block = 7510843268513837357;
                    break;
                }
                14535175905252782758 => {
                    c_3 = 0;
                    current_block = 12387625063048049585;
                    break;
                }
                14648249180243006330 => {
                    c_2 = 0;
                    current_block = 13014351284863956202;
                    break;
                }
                993425571616822999 => {
                    counter = counter.wrapping_add(1);
                    current_block = 17485376261910781866;
                    continue;
                }
                _ => {
                    dist = (bit_buf
                        & (((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)
                            - 1 as ::core::ffi::c_int) as tinfl_bit_buf_t)
                        as mz_uint32;
                    bit_buf >>= 8 as ::core::ffi::c_int;
                    num_bits = num_bits.wrapping_sub(8 as mz_uint32);
                    current_block = 15460309861373144675;
                    break;
                }
            }
            match current_block {
                7757638761803742862 => {
                    if !(counter < 4 as mz_uint32) {
                        current_block = 9997576841122142810;
                        break;
                    }
                    s_1 = 0;
                    if !(num_bits != 0) {
                        current_block = 13250426973517230943;
                        break;
                    }
                    if num_bits < 8 as ::core::ffi::c_int as mz_uint {
                        current_block = 8585542996221928953;
                        continue;
                    } else {
                        current_block = 2977775588433926224;
                        continue;
                    }
                }
                14117028633286690613 => {
                    dist_from_out_buf_start =
                        pOut_buf_cur.offset_from(pOut_buf_start) as ::core::ffi::c_long as size_t;
                    if (dist == 0 as mz_uint32
                        || dist as size_t > dist_from_out_buf_start
                        || dist_from_out_buf_start == 0 as size_t)
                        && decomp_flags
                            & TINFL_FLAG_USING_NON_WRAPPING_OUTPUT_BUF as ::core::ffi::c_int
                                as mz_uint32
                            != 0
                    {
                        current_block = 13286354103097804402;
                        break;
                    }
                    pSrc = pOut_buf_start.offset(
                        (dist_from_out_buf_start.wrapping_sub(dist as size_t) & out_buf_size_mask)
                            as isize,
                    );
                    if (if pOut_buf_cur > pSrc {
                        pOut_buf_cur
                    } else {
                        pSrc
                    })
                    .offset(counter as isize)
                        > pOut_buf_end
                    {
                        current_block = 18076970406079194302;
                        continue;
                    }
                    pOut_buf_cur = tinfl_copy_match_fast(
                        pOut_buf_cur,
                        pSrc,
                        counter as usize,
                        dist as usize,
                    );
                    current_block = 12926621472508904600;
                }
                12369472161353711912 => {
                    counter &= 511 as mz_uint32;
                    if counter == 256 as mz_uint32 {
                        current_block = 1345366029464561491;
                    } else {
                        num_extra = s_length_extra[counter.wrapping_sub(257 as mz_uint32) as usize]
                            as mz_uint32;
                        counter = s_length_base[counter.wrapping_sub(257 as mz_uint32) as usize]
                            as mz_uint32;
                        if num_extra != 0 {
                            extra_bits = 0;
                            if num_bits < num_extra {
                                current_block = 3846997171290157944;
                                continue;
                            } else {
                                current_block = 17809027598452024276;
                                continue;
                            }
                        } else {
                            current_block = 18029556747650706958;
                        }
                    }
                }
                15712984148872257586 => {
                    if counter
                        < (*r).m_table_sizes[0 as ::core::ffi::c_int as usize]
                            .wrapping_add((*r).m_table_sizes[1 as ::core::ffi::c_int as usize])
                    {
                        s_0 = 0;
                        temp = 0;
                        code_len = 0;
                        c_5 = 0;
                        if !(num_bits < 15 as mz_uint32) {
                            current_block = 11867310137400648068;
                            continue;
                        }
                        if (pIn_buf_end.offset_from(pIn_buf_cur) as ::core::ffi::c_long)
                            < 2 as ::core::ffi::c_long
                        {
                            current_block = 3179191505073644361;
                            continue;
                        }
                        bit_buf |= (*pIn_buf_cur.offset(0 as ::core::ffi::c_int as isize)
                            as tinfl_bit_buf_t)
                            << num_bits
                            | (*pIn_buf_cur.offset(1 as ::core::ffi::c_int as isize)
                                as tinfl_bit_buf_t)
                                << num_bits.wrapping_add(8 as mz_uint32);
                        pIn_buf_cur = pIn_buf_cur.offset(2 as ::core::ffi::c_int as isize);
                        num_bits = num_bits.wrapping_add(16 as mz_uint32);
                        current_block = 11867310137400648068;
                        continue;
                    } else {
                        if (*r).m_table_sizes[0 as ::core::ffi::c_int as usize]
                            .wrapping_add((*r).m_table_sizes[1 as ::core::ffi::c_int as usize])
                            != counter
                        {
                            current_block = 12060859132547941340;
                            break;
                        }
                        memcpy(
                            &raw mut (*r).m_code_size_0 as *mut mz_uint8
                                as *mut ::core::ffi::c_void,
                            &raw mut (*r).m_len_codes as *mut mz_uint8
                                as *const ::core::ffi::c_void,
                            (*r).m_table_sizes[0 as ::core::ffi::c_int as usize] as size_t,
                        );
                        memcpy(
                            &raw mut (*r).m_code_size_1 as *mut mz_uint8
                                as *mut ::core::ffi::c_void,
                            (&raw mut (*r).m_len_codes as *mut mz_uint8).offset(
                                (*r).m_table_sizes[0 as ::core::ffi::c_int as usize] as isize,
                            ) as *const ::core::ffi::c_void,
                            (*r).m_table_sizes[1 as ::core::ffi::c_int as usize] as size_t,
                        );
                    }
                    current_block = 8038949400865391589;
                }
                5638484086777907119 => {
                    if counter < (*r).m_table_sizes[2 as ::core::ffi::c_int as usize] {
                        s = 0;
                        if num_bits < 3 as ::core::ffi::c_int as mz_uint {
                            current_block = 11002855972428369642;
                            continue;
                        } else {
                            current_block = 9239588423676249671;
                            continue;
                        }
                    } else {
                        (*r).m_table_sizes[2 as ::core::ffi::c_int as usize] = 19 as mz_uint32;
                        current_block = 17723783950272744135;
                        continue;
                    }
                }
                _ => {}
            }
            match current_block {
                8038949400865391589 => {
                    (*r).m_type = (*r).m_type.wrapping_sub(1);
                    current_block = 17723783950272744135;
                }
                18029556747650706958 => {
                    temp_1 = 0;
                    code_len_2 = 0;
                    c_9 = 0;
                    if !(num_bits < 15 as mz_uint32) {
                        current_block = 2358125959230175008;
                        continue;
                    }
                    if (pIn_buf_end.offset_from(pIn_buf_cur) as ::core::ffi::c_long)
                        < 2 as ::core::ffi::c_long
                    {
                        current_block = 3011706931690025809;
                        continue;
                    }
                    bit_buf |= (*pIn_buf_cur.offset(0 as ::core::ffi::c_int as isize)
                        as tinfl_bit_buf_t)
                        << num_bits
                        | (*pIn_buf_cur.offset(1 as ::core::ffi::c_int as isize)
                            as tinfl_bit_buf_t)
                            << num_bits.wrapping_add(8 as mz_uint32);
                    pIn_buf_cur = pIn_buf_cur.offset(2 as ::core::ffi::c_int as isize);
                    num_bits = num_bits.wrapping_add(16 as mz_uint32);
                    current_block = 2358125959230175008;
                }
                12926621472508904600 => {
                    pSrc = ::core::ptr::null_mut::<mz_uint8>();
                    current_block = 18019479878307254118;
                }
                _ => {
                    if (*r).m_final & 1 as mz_uint32 == 0 {
                        current_block = 313581471991351815;
                        break;
                    }
                    if num_bits < num_bits & 7 as mz_uint32 {
                        current_block = 4159430576127784976;
                    } else {
                        current_block = 955764296456093747;
                    }
                }
            }
        }
    }
    (*r).m_num_bits = num_bits;
    (*r).m_bit_buf = bit_buf & !(!(0 as ::core::ffi::c_int as tinfl_bit_buf_t) << num_bits);
    (*r).m_dist = dist;
    (*r).m_counter = counter;
    (*r).m_num_extra = num_extra;
    (*r).m_dist_from_out_buf_start = dist_from_out_buf_start;
    *pIn_buf_size = pIn_buf_cur.offset_from(pIn_buf_next) as ::core::ffi::c_long as size_t;
    *pOut_buf_size = pOut_buf_cur.offset_from(pOut_buf_next) as ::core::ffi::c_long as size_t;
    if decomp_flags
        & (TINFL_FLAG_PARSE_ZLIB_HEADER as ::core::ffi::c_int
            | TINFL_FLAG_COMPUTE_ADLER32 as ::core::ffi::c_int) as mz_uint32
        != 0
        && status as ::core::ffi::c_int >= 0 as ::core::ffi::c_int
    {
        let mut ptr: *const mz_uint8 = pOut_buf_next;
        let mut buf_len: size_t = *pOut_buf_size;
        let mut i_1: mz_uint32 = 0;
        let mut s1: mz_uint32 = (*r).m_check_adler32 & 0xffff as mz_uint32;
        let mut s2: mz_uint32 = (*r).m_check_adler32 >> 16 as ::core::ffi::c_int;
        let mut block_len: size_t = buf_len.wrapping_rem(5552 as size_t);
        while buf_len != 0 {
            i_1 = 0 as mz_uint32;
            while (i_1.wrapping_add(7 as mz_uint32) as size_t) < block_len {
                s1 = s1.wrapping_add(*ptr.offset(0 as ::core::ffi::c_int as isize) as mz_uint32);
                s2 = s2.wrapping_add(s1);
                s1 = s1.wrapping_add(*ptr.offset(1 as ::core::ffi::c_int as isize) as mz_uint32);
                s2 = s2.wrapping_add(s1);
                s1 = s1.wrapping_add(*ptr.offset(2 as ::core::ffi::c_int as isize) as mz_uint32);
                s2 = s2.wrapping_add(s1);
                s1 = s1.wrapping_add(*ptr.offset(3 as ::core::ffi::c_int as isize) as mz_uint32);
                s2 = s2.wrapping_add(s1);
                s1 = s1.wrapping_add(*ptr.offset(4 as ::core::ffi::c_int as isize) as mz_uint32);
                s2 = s2.wrapping_add(s1);
                s1 = s1.wrapping_add(*ptr.offset(5 as ::core::ffi::c_int as isize) as mz_uint32);
                s2 = s2.wrapping_add(s1);
                s1 = s1.wrapping_add(*ptr.offset(6 as ::core::ffi::c_int as isize) as mz_uint32);
                s2 = s2.wrapping_add(s1);
                s1 = s1.wrapping_add(*ptr.offset(7 as ::core::ffi::c_int as isize) as mz_uint32);
                s2 = s2.wrapping_add(s1);
                i_1 = i_1.wrapping_add(8 as mz_uint32);
                ptr = ptr.offset(8 as ::core::ffi::c_int as isize);
            }
            while (i_1 as size_t) < block_len {
                let fresh37 = ptr;
                ptr = ptr.offset(1);
                s1 = s1.wrapping_add(*fresh37 as mz_uint32);
                s2 = s2.wrapping_add(s1);
                i_1 = i_1.wrapping_add(1);
            }
            s1 = (s1 as ::core::ffi::c_uint).wrapping_rem(65521 as ::core::ffi::c_uint) as mz_uint32
                as mz_uint32;
            s2 = (s2 as ::core::ffi::c_uint).wrapping_rem(65521 as ::core::ffi::c_uint) as mz_uint32
                as mz_uint32;
            buf_len = buf_len.wrapping_sub(block_len);
            block_len = 5552 as size_t;
        }
        (*r).m_check_adler32 = (s2 << 16 as ::core::ffi::c_int).wrapping_add(s1);
        if status as ::core::ffi::c_int == TINFL_STATUS_DONE as ::core::ffi::c_int
            && decomp_flags & TINFL_FLAG_PARSE_ZLIB_HEADER as ::core::ffi::c_int as mz_uint32 != 0
            && (*r).m_check_adler32 != (*r).m_z_adler32
        {
            status = TINFL_STATUS_ADLER32_MISMATCH;
        }
    }
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn tinfl_decompress_mem_to_heap(
    mut pSrc_buf: *const ::core::ffi::c_void,
    mut src_buf_len: size_t,
    mut pOut_len: *mut size_t,
    mut flags: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_void {
    let mut decomp: tinfl_decompressor = tinfl_decompressor_tag {
        m_state: 0,
        m_num_bits: 0,
        m_zhdr0: 0,
        m_zhdr1: 0,
        m_z_adler32: 0,
        m_final: 0,
        m_type: 0,
        m_check_adler32: 0,
        m_dist: 0,
        m_counter: 0,
        m_num_extra: 0,
        m_table_sizes: [0; 3],
        m_bit_buf: 0,
        m_dist_from_out_buf_start: 0,
        m_look_up: [[0; 1024]; 3],
        m_tree_0: [0; 576],
        m_tree_1: [0; 64],
        m_tree_2: [0; 38],
        m_code_size_0: [0; 288],
        m_code_size_1: [0; 32],
        m_code_size_2: [0; 19],
        m_raw_header: [0; 4],
        m_len_codes: [0; 457],
    };
    let mut pBuf: *mut ::core::ffi::c_void = NULL;
    let mut pNew_buf: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut src_buf_ofs: size_t = 0 as size_t;
    let mut out_buf_capacity: size_t = 0 as size_t;
    *pOut_len = 0 as size_t;
    decomp.m_state = 0 as mz_uint32;
    loop {
        let mut src_buf_size: size_t = src_buf_len.wrapping_sub(src_buf_ofs);
        let mut dst_buf_size: size_t = out_buf_capacity.wrapping_sub(*pOut_len);
        let mut new_out_buf_capacity: size_t = 0;
        let mut status: tinfl_status = tinfl_decompress(
            &raw mut decomp,
            (pSrc_buf as *const mz_uint8).offset(src_buf_ofs as isize),
            &raw mut src_buf_size,
            pBuf as *mut mz_uint8,
            if !pBuf.is_null() {
                (pBuf as *mut mz_uint8).offset(*pOut_len as isize)
            } else {
                ::core::ptr::null_mut::<mz_uint8>()
            },
            &raw mut dst_buf_size,
            (flags & !(TINFL_FLAG_HAS_MORE_INPUT as ::core::ffi::c_int)
                | TINFL_FLAG_USING_NON_WRAPPING_OUTPUT_BUF as ::core::ffi::c_int)
                as mz_uint32,
        );
        if (status as ::core::ffi::c_int) < 0 as ::core::ffi::c_int
            || status as ::core::ffi::c_int == TINFL_STATUS_NEEDS_MORE_INPUT as ::core::ffi::c_int
        {
            free(pBuf);
            *pOut_len = 0 as size_t;
            return NULL;
        }
        src_buf_ofs = src_buf_ofs.wrapping_add(src_buf_size);
        *pOut_len = (*pOut_len).wrapping_add(dst_buf_size);
        if status as ::core::ffi::c_int == TINFL_STATUS_DONE as ::core::ffi::c_int {
            break;
        }
        new_out_buf_capacity = out_buf_capacity.wrapping_mul(2 as size_t);
        if new_out_buf_capacity < 128 as size_t {
            new_out_buf_capacity = 128 as size_t;
        }
        pNew_buf = realloc(pBuf, new_out_buf_capacity);
        if pNew_buf.is_null() {
            free(pBuf);
            *pOut_len = 0 as size_t;
            return NULL;
        }
        pBuf = pNew_buf;
        out_buf_capacity = new_out_buf_capacity;
    }
    return pBuf;
}
#[no_mangle]
pub unsafe extern "C" fn tinfl_decompress_mem_to_mem(
    mut pOut_buf: *mut ::core::ffi::c_void,
    mut out_buf_len: size_t,
    mut pSrc_buf: *const ::core::ffi::c_void,
    mut src_buf_len: size_t,
    mut flags: ::core::ffi::c_int,
) -> size_t {
    let mut decomp: tinfl_decompressor = tinfl_decompressor_tag {
        m_state: 0,
        m_num_bits: 0,
        m_zhdr0: 0,
        m_zhdr1: 0,
        m_z_adler32: 0,
        m_final: 0,
        m_type: 0,
        m_check_adler32: 0,
        m_dist: 0,
        m_counter: 0,
        m_num_extra: 0,
        m_table_sizes: [0; 3],
        m_bit_buf: 0,
        m_dist_from_out_buf_start: 0,
        m_look_up: [[0; 1024]; 3],
        m_tree_0: [0; 576],
        m_tree_1: [0; 64],
        m_tree_2: [0; 38],
        m_code_size_0: [0; 288],
        m_code_size_1: [0; 32],
        m_code_size_2: [0; 19],
        m_raw_header: [0; 4],
        m_len_codes: [0; 457],
    };
    let mut status: tinfl_status = TINFL_STATUS_DONE;
    decomp.m_state = 0 as mz_uint32;
    status = tinfl_decompress(
        &raw mut decomp,
        pSrc_buf as *const mz_uint8,
        &raw mut src_buf_len,
        pOut_buf as *mut mz_uint8,
        pOut_buf as *mut mz_uint8,
        &raw mut out_buf_len,
        (flags & !(TINFL_FLAG_HAS_MORE_INPUT as ::core::ffi::c_int)
            | TINFL_FLAG_USING_NON_WRAPPING_OUTPUT_BUF as ::core::ffi::c_int) as mz_uint32,
    );
    return if status as ::core::ffi::c_int != TINFL_STATUS_DONE as ::core::ffi::c_int {
        TINFL_DECOMPRESS_MEM_TO_MEM_FAILED
    } else {
        out_buf_len
    };
}
#[no_mangle]
pub unsafe extern "C" fn tinfl_decompress_mem_to_callback(
    mut pIn_buf: *const ::core::ffi::c_void,
    mut pIn_buf_size: *mut size_t,
    mut pPut_buf_func: tinfl_put_buf_func_ptr,
    mut pPut_buf_user: *mut ::core::ffi::c_void,
    mut flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut result: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut decomp: tinfl_decompressor = tinfl_decompressor_tag {
        m_state: 0,
        m_num_bits: 0,
        m_zhdr0: 0,
        m_zhdr1: 0,
        m_z_adler32: 0,
        m_final: 0,
        m_type: 0,
        m_check_adler32: 0,
        m_dist: 0,
        m_counter: 0,
        m_num_extra: 0,
        m_table_sizes: [0; 3],
        m_bit_buf: 0,
        m_dist_from_out_buf_start: 0,
        m_look_up: [[0; 1024]; 3],
        m_tree_0: [0; 576],
        m_tree_1: [0; 64],
        m_tree_2: [0; 38],
        m_code_size_0: [0; 288],
        m_code_size_1: [0; 32],
        m_code_size_2: [0; 19],
        m_raw_header: [0; 4],
        m_len_codes: [0; 457],
    };
    let mut pDict: *mut mz_uint8 = malloc(32768 as size_t) as *mut mz_uint8;
    let mut in_buf_ofs: size_t = 0 as size_t;
    let mut dict_ofs: size_t = 0 as size_t;
    if pDict.is_null() {
        return TINFL_STATUS_FAILED as ::core::ffi::c_int;
    }
    memset(
        pDict as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        TINFL_LZ_DICT_SIZE as size_t,
    );
    decomp.m_state = 0 as mz_uint32;
    loop {
        let mut in_buf_size: size_t = (*pIn_buf_size).wrapping_sub(in_buf_ofs);
        let mut dst_buf_size: size_t = (TINFL_LZ_DICT_SIZE as size_t).wrapping_sub(dict_ofs);
        let mut status: tinfl_status = tinfl_decompress(
            &raw mut decomp,
            (pIn_buf as *const mz_uint8).offset(in_buf_ofs as isize),
            &raw mut in_buf_size,
            pDict,
            pDict.offset(dict_ofs as isize),
            &raw mut dst_buf_size,
            (flags
                & !(TINFL_FLAG_HAS_MORE_INPUT as ::core::ffi::c_int
                    | TINFL_FLAG_USING_NON_WRAPPING_OUTPUT_BUF as ::core::ffi::c_int))
                as mz_uint32,
        );
        in_buf_ofs = in_buf_ofs.wrapping_add(in_buf_size);
        if dst_buf_size != 0
            && Some(pPut_buf_func.expect("non-null function pointer"))
                .expect("non-null function pointer")(
                pDict.offset(dict_ofs as isize) as *const ::core::ffi::c_void,
                dst_buf_size as ::core::ffi::c_int,
                pPut_buf_user,
            ) == 0
        {
            break;
        }
        if status as ::core::ffi::c_int != TINFL_STATUS_HAS_MORE_OUTPUT as ::core::ffi::c_int {
            result = (status as ::core::ffi::c_int == TINFL_STATUS_DONE as ::core::ffi::c_int)
                as ::core::ffi::c_int;
            break;
        } else {
            dict_ofs = dict_ofs.wrapping_add(dst_buf_size)
                & (TINFL_LZ_DICT_SIZE - 1 as ::core::ffi::c_int) as size_t;
        }
    }
    free(pDict as *mut ::core::ffi::c_void);
    *pIn_buf_size = in_buf_ofs;
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn tinfl_decompressor_alloc() -> *mut tinfl_decompressor {
    let mut pDecomp: *mut tinfl_decompressor =
        malloc(::core::mem::size_of::<tinfl_decompressor>() as size_t) as *mut tinfl_decompressor;
    if !pDecomp.is_null() {
        (*pDecomp).m_state = 0 as mz_uint32;
    }
    return pDecomp;
}
#[no_mangle]
pub unsafe extern "C" fn tinfl_decompressor_free(mut pDecomp: *mut tinfl_decompressor) {
    free(pDecomp as *mut ::core::ffi::c_void);
}
