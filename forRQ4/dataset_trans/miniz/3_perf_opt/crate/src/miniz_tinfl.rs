use core::ffi::*;
pub use crate::src::ffi::*;
pub use crate::src::c_consts::*;
pub use crate::src::c_structs::*;
pub use crate::src::c_types::*;

pub const TINFL_FLAG_COMPUTE_ADLER32: C2RustUnnamed_htdd24ee73 = 8;
pub const TINFL_FLAG_USING_NON_WRAPPING_OUTPUT_BUF: C2RustUnnamed_htdd24ee73 = 4;
pub const TINFL_FLAG_HAS_MORE_INPUT: C2RustUnnamed_htdd24ee73 = 2;
pub const TINFL_FLAG_PARSE_ZLIB_HEADER: C2RustUnnamed_htdd24ee73 = 1;

pub const TINFL_FAST_LOOKUP_BITS: C2RustUnnamed_htdd24ee73 = 10;
pub const TINFL_FAST_LOOKUP_SIZE: C2RustUnnamed_htdd24ee73 = 1024;
pub type tinfl_put_buf_func_ptr = Option<
    unsafe extern "C" fn(
        *const c_void,
        c_int,
        *mut c_void,
    ) -> c_int,
>;
pub type C2RustUnnamed_htdd24ee73 = c_uint;
pub const TINFL_MAX_HUFF_SYMBOLS_2: C2RustUnnamed_htdd24ee73 = 19;
pub const TINFL_MAX_HUFF_SYMBOLS_1: C2RustUnnamed_htdd24ee73 = 32;
pub const TINFL_MAX_HUFF_SYMBOLS_0: C2RustUnnamed_htdd24ee73 = 288;
pub const TINFL_MAX_HUFF_TABLES: C2RustUnnamed_htdd24ee73 = 3;

pub const TINFL_DECOMPRESS_MEM_TO_MEM_FAILED: size_t = -(1 as c_int) as size_t;

unsafe fn tinfl_clear_tree(mut r: *mut tinfl_decompressor) {
    let r_view: &mut tinfl_decompressor = unsafe { &mut *r };
    if r_view.m_type == 0 as mz_uint32 {
        memset(
            &raw mut r_view.m_tree_0 as *mut mz_int16 as *mut c_void,
            0 as c_int,
            ::core::mem::size_of::<[mz_int16; 576]>() as size_t,
        );
    } else if r_view.m_type == 1 as mz_uint32 {
        memset(
            &raw mut r_view.m_tree_1 as *mut mz_int16 as *mut c_void,
            0 as c_int,
            ::core::mem::size_of::<[mz_int16; 64]>() as size_t,
        );
    } else {
        memset(
            &raw mut r_view.m_tree_2 as *mut mz_int16 as *mut c_void,
            0 as c_int,
            ::core::mem::size_of::<[mz_int16; 38]>() as size_t,
        );
    };
}
pub unsafe fn tinfl_decompress(
    mut r: *mut tinfl_decompressor,
    mut pIn_buf_next: *const mz_uint8,
    mut pIn_buf_size: *mut size_t,
    mut pOut_buf_start: *mut mz_uint8,
    mut pOut_buf_next: *mut mz_uint8,
    mut pOut_buf_size: *mut size_t,
    decomp_flags: mz_uint32,
) -> tinfl_status {
    let pIn_buf_size_view: &mut size_t = unsafe { &mut *pIn_buf_size };
    let mut s_1: mz_uint = 0;
    let mut c_12: mz_uint = 0;
    let mut c_11: mz_uint = 0;
    let mut pSrc: *mut mz_uint8 = ::core::ptr::null_mut::<mz_uint8>();
    let mut extra_bits_0: mz_uint = 0;
    let mut c_10: mz_uint = 0;
    let mut temp_1: c_int = 0;
    let mut code_len_2: mz_uint = 0;
    let mut c_9: mz_uint = 0;
    let mut extra_bits: mz_uint = 0;
    let mut c_8: mz_uint = 0;
    let mut sym2: c_int = 0;
    let mut code_len_1: mz_uint = 0;
    let mut temp_0: c_int = 0;
    let mut code_len_0: mz_uint = 0;
    let mut c_7: mz_uint = 0;
    let mut tree_next: c_int = 0;
    let mut tree_cur: c_int = 0;
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
    let mut temp: c_int = 0;
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
    let mut current_block: u32;
    static mut s_length_base: [mz_uint16; 31] = [
        3 as c_int as mz_uint16,
        4 as c_int as mz_uint16,
        5 as c_int as mz_uint16,
        6 as c_int as mz_uint16,
        7 as c_int as mz_uint16,
        8 as c_int as mz_uint16,
        9 as c_int as mz_uint16,
        10 as c_int as mz_uint16,
        11 as c_int as mz_uint16,
        13 as c_int as mz_uint16,
        15 as c_int as mz_uint16,
        17 as c_int as mz_uint16,
        19 as c_int as mz_uint16,
        23 as c_int as mz_uint16,
        27 as c_int as mz_uint16,
        31 as c_int as mz_uint16,
        35 as c_int as mz_uint16,
        43 as c_int as mz_uint16,
        51 as c_int as mz_uint16,
        59 as c_int as mz_uint16,
        67 as c_int as mz_uint16,
        83 as c_int as mz_uint16,
        99 as c_int as mz_uint16,
        115 as c_int as mz_uint16,
        131 as c_int as mz_uint16,
        163 as c_int as mz_uint16,
        195 as c_int as mz_uint16,
        227 as c_int as mz_uint16,
        258 as c_int as mz_uint16,
        0 as c_int as mz_uint16,
        0 as c_int as mz_uint16,
    ];
    static mut s_length_extra: [mz_uint8; 31] = [
        0 as c_int as mz_uint8,
        0 as c_int as mz_uint8,
        0 as c_int as mz_uint8,
        0 as c_int as mz_uint8,
        0 as c_int as mz_uint8,
        0 as c_int as mz_uint8,
        0 as c_int as mz_uint8,
        0 as c_int as mz_uint8,
        1 as c_int as mz_uint8,
        1 as c_int as mz_uint8,
        1 as c_int as mz_uint8,
        1 as c_int as mz_uint8,
        2 as c_int as mz_uint8,
        2 as c_int as mz_uint8,
        2 as c_int as mz_uint8,
        2 as c_int as mz_uint8,
        3 as c_int as mz_uint8,
        3 as c_int as mz_uint8,
        3 as c_int as mz_uint8,
        3 as c_int as mz_uint8,
        4 as c_int as mz_uint8,
        4 as c_int as mz_uint8,
        4 as c_int as mz_uint8,
        4 as c_int as mz_uint8,
        5 as c_int as mz_uint8,
        5 as c_int as mz_uint8,
        5 as c_int as mz_uint8,
        5 as c_int as mz_uint8,
        0 as c_int as mz_uint8,
        0 as c_int as mz_uint8,
        0 as c_int as mz_uint8,
    ];
    static mut s_dist_base: [mz_uint16; 32] = [
        1 as c_int as mz_uint16,
        2 as c_int as mz_uint16,
        3 as c_int as mz_uint16,
        4 as c_int as mz_uint16,
        5 as c_int as mz_uint16,
        7 as c_int as mz_uint16,
        9 as c_int as mz_uint16,
        13 as c_int as mz_uint16,
        17 as c_int as mz_uint16,
        25 as c_int as mz_uint16,
        33 as c_int as mz_uint16,
        49 as c_int as mz_uint16,
        65 as c_int as mz_uint16,
        97 as c_int as mz_uint16,
        129 as c_int as mz_uint16,
        193 as c_int as mz_uint16,
        257 as c_int as mz_uint16,
        385 as c_int as mz_uint16,
        513 as c_int as mz_uint16,
        769 as c_int as mz_uint16,
        1025 as c_int as mz_uint16,
        1537 as c_int as mz_uint16,
        2049 as c_int as mz_uint16,
        3073 as c_int as mz_uint16,
        4097 as c_int as mz_uint16,
        6145 as c_int as mz_uint16,
        8193 as c_int as mz_uint16,
        12289 as c_int as mz_uint16,
        16385 as c_int as mz_uint16,
        24577 as c_int as mz_uint16,
        0 as c_int as mz_uint16,
        0 as c_int as mz_uint16,
    ];
    static mut s_dist_extra: [mz_uint8; 32] = [
        0 as c_int as mz_uint8,
        0 as c_int as mz_uint8,
        0 as c_int as mz_uint8,
        0 as c_int as mz_uint8,
        1 as c_int as mz_uint8,
        1 as c_int as mz_uint8,
        2 as c_int as mz_uint8,
        2 as c_int as mz_uint8,
        3 as c_int as mz_uint8,
        3 as c_int as mz_uint8,
        4 as c_int as mz_uint8,
        4 as c_int as mz_uint8,
        5 as c_int as mz_uint8,
        5 as c_int as mz_uint8,
        6 as c_int as mz_uint8,
        6 as c_int as mz_uint8,
        7 as c_int as mz_uint8,
        7 as c_int as mz_uint8,
        8 as c_int as mz_uint8,
        8 as c_int as mz_uint8,
        9 as c_int as mz_uint8,
        9 as c_int as mz_uint8,
        10 as c_int as mz_uint8,
        10 as c_int as mz_uint8,
        11 as c_int as mz_uint8,
        11 as c_int as mz_uint8,
        12 as c_int as mz_uint8,
        12 as c_int as mz_uint8,
        13 as c_int as mz_uint8,
        13 as c_int as mz_uint8,
        0,
        0,
    ];
    static mut s_length_dezigzag: [mz_uint8; 19] = [
        16 as c_int as mz_uint8,
        17 as c_int as mz_uint8,
        18 as c_int as mz_uint8,
        0 as c_int as mz_uint8,
        8 as c_int as mz_uint8,
        7 as c_int as mz_uint8,
        9 as c_int as mz_uint8,
        6 as c_int as mz_uint8,
        10 as c_int as mz_uint8,
        5 as c_int as mz_uint8,
        11 as c_int as mz_uint8,
        4 as c_int as mz_uint8,
        12 as c_int as mz_uint8,
        3 as c_int as mz_uint8,
        13 as c_int as mz_uint8,
        2 as c_int as mz_uint8,
        14 as c_int as mz_uint8,
        1 as c_int as mz_uint8,
        15 as c_int as mz_uint8,
    ];
    static mut s_min_table_sizes: [mz_uint16; 3] = [
        257 as c_int as mz_uint16,
        1 as c_int as mz_uint16,
        4 as c_int as mz_uint16,
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
    let pIn_buf_end: *const mz_uint8 = pIn_buf_next.offset(*pIn_buf_size_view as isize);
    let mut pOut_buf_cur: *mut mz_uint8 = pOut_buf_next;
    let pOut_buf_end: *mut mz_uint8 = if !pOut_buf_next.is_null() {
        pOut_buf_next.offset(*pOut_buf_size as isize)
    } else {
        ::core::ptr::null_mut::<mz_uint8>()
    };
    let mut out_buf_size_mask: size_t = if decomp_flags
        & TINFL_FLAG_USING_NON_WRAPPING_OUTPUT_BUF as c_int as mz_uint32
        != 0
    {
        -(1 as c_int) as size_t
    } else {
        (pOut_buf_next.offset_from(pOut_buf_start) as c_long as size_t)
            .wrapping_add(*pOut_buf_size)
            .wrapping_sub(1 as size_t)
    };
    let mut dist_from_out_buf_start: size_t = 0;
    if out_buf_size_mask.wrapping_add(1 as size_t) & out_buf_size_mask != 0
        || pOut_buf_next < pOut_buf_start
    {
        *pOut_buf_size = 0 as size_t;
        *pIn_buf_size_view = *pOut_buf_size;
        return TINFL_STATUS_BAD_PARAM;
    }
    pTrees[0 as c_int as usize] = &raw mut (*r).m_tree_0 as *mut mz_int16;
    pTrees[1 as c_int as usize] = &raw mut (*r).m_tree_1 as *mut mz_int16;
    pTrees[2 as c_int as usize] = &raw mut (*r).m_tree_2 as *mut mz_int16;
    pCode_sizes[0 as c_int as usize] = &raw mut (*r).m_code_size_0 as *mut mz_uint8;
    pCode_sizes[1 as c_int as usize] = &raw mut (*r).m_code_size_1 as *mut mz_uint8;
    pCode_sizes[2 as c_int as usize] = &raw mut (*r).m_code_size_2 as *mut mz_uint8;
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
            if decomp_flags & TINFL_FLAG_PARSE_ZLIB_HEADER as c_int as mz_uint32 != 0 {
                current_block = 0;
            } else {
                current_block = 1;
            }
        }
        1 => {
            current_block = 0;
        }
        2 => {
            current_block = 2;
        }
        36 => {
            current_block = 3;
        }
        3 => {
            current_block = 4;
        }
        5 => {
            current_block = 5;
        }
        6 => {
            current_block = 6;
        }
        7 => {
            current_block = 7;
        }
        39 => {
            current_block = 8;
        }
        51 => {
            current_block = 9;
        }
        52 => {
            current_block = 10;
        }
        9 => {
            current_block = 11;
        }
        38 => {
            current_block = 12;
        }
        10 => {
            current_block = 13;
        }
        11 => {
            current_block = 14;
        }
        14 => {
            current_block = 15;
        }
        35 => {
            current_block = 16;
        }
        16 => {
            current_block = 17;
        }
        17 => {
            current_block = 18;
        }
        18 => {
            current_block = 19;
        }
        21 => {
            current_block = 20;
        }
        23 => {
            current_block = 21;
        }
        24 => {
            current_block = 22;
        }
        40 => {
            current_block = 23;
        }
        54 => {
            current_block = 24;
        }
        25 => {
            current_block = 25;
        }
        26 => {
            current_block = 26;
        }
        27 => {
            current_block = 27;
        }
        37 => {
            current_block = 28;
        }
        53 => {
            current_block = 29;
        }
        32 => {
            current_block = 30;
        }
        41 => {
            current_block = 31;
        }
        42 => {
            current_block = 32;
        }
        34 => {
            current_block = 33;
        }
        _ => {
            current_block = 34;
        }
    }
    match current_block {
        0 => {
            if pIn_buf_cur >= pIn_buf_end {
                status = (if decomp_flags
                    & TINFL_FLAG_HAS_MORE_INPUT as c_int as mz_uint32
                    != 0
                {
                    TINFL_STATUS_NEEDS_MORE_INPUT as c_int
                } else {
                    TINFL_STATUS_FAILED_CANNOT_MAKE_PROGRESS as c_int
                }) as tinfl_status;
                (*r).m_state = 1 as mz_uint32;
                current_block = 34;
            } else {
                let fresh0 = pIn_buf_cur;
                pIn_buf_cur = pIn_buf_cur.offset(1);
                (*r).m_zhdr0 = *fresh0 as mz_uint32;
                current_block = 2;
            }
        }
        _ => {}
    }
    match current_block {
        2 => {
            if pIn_buf_cur >= pIn_buf_end {
                status = (if decomp_flags
                    & TINFL_FLAG_HAS_MORE_INPUT as c_int as mz_uint32
                    != 0
                {
                    TINFL_STATUS_NEEDS_MORE_INPUT as c_int
                } else {
                    TINFL_STATUS_FAILED_CANNOT_MAKE_PROGRESS as c_int
                }) as tinfl_status;
                (*r).m_state = 2 as mz_uint32;
                current_block = 34;
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
                    as c_int as mz_uint32;
                if decomp_flags
                    & TINFL_FLAG_USING_NON_WRAPPING_OUTPUT_BUF as c_int as mz_uint32
                    == 0
                {
                    counter |= ((1 as c_uint)
                        << (8 as mz_uint32).wrapping_add((*r).m_zhdr0 >> 4 as c_int)
                        > 32768 as c_uint
                        || out_buf_size_mask.wrapping_add(1 as size_t)
                            < (1 as c_int as size_t)
                                << (8 as mz_uint32)
                                    .wrapping_add((*r).m_zhdr0 >> 4 as c_int))
                        as c_int as mz_uint32;
                }
                if counter != 0 {
                    current_block = 3;
                } else {
                    current_block = 1;
                }
            }
        }
        _ => {}
    }
    match current_block {
        3 => {
            status = TINFL_STATUS_FAILED;
            (*r).m_state = 36 as mz_uint32;
            current_block = 34;
        }
        _ => {}
    }
    loop {
        match current_block {
            1 => {
                if num_bits < 3 as c_int as mz_uint {
                    current_block = 35;
                } else {
                    current_block = 36;
                }
            }
            30 => {
                if pIn_buf_cur >= pIn_buf_end {
                    status = (if decomp_flags
                        & TINFL_FLAG_HAS_MORE_INPUT as c_int as mz_uint32
                        != 0
                    {
                        TINFL_STATUS_NEEDS_MORE_INPUT as c_int
                    } else {
                        TINFL_STATUS_FAILED_CANNOT_MAKE_PROGRESS as c_int
                    }) as tinfl_status;
                    (*r).m_state = 32 as mz_uint32;
                    current_block = 34;
                    continue;
                } else {
                    let fresh34 = pIn_buf_cur;
                    pIn_buf_cur = pIn_buf_cur.offset(1);
                    c_11 = *fresh34 as mz_uint;
                    bit_buf |= (c_11 as tinfl_bit_buf_t) << num_bits;
                    num_bits = num_bits.wrapping_add(8 as mz_uint32);
                    if num_bits < num_bits & 7 as mz_uint32 {
                        current_block = 37;
                    } else {
                        current_block = 38;
                    }
                }
            }
            29 => {
                if pOut_buf_cur >= pOut_buf_end {
                    status = TINFL_STATUS_HAS_MORE_OUTPUT;
                    (*r).m_state = 53 as mz_uint32;
                    current_block = 34;
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
                current_block = 39;
            }
            28 => {
                status = TINFL_STATUS_FAILED;
                (*r).m_state = 37 as mz_uint32;
                current_block = 34;
                continue;
            }
            27 => {
                if pIn_buf_cur >= pIn_buf_end {
                    status = (if decomp_flags
                        & TINFL_FLAG_HAS_MORE_INPUT as c_int as mz_uint32
                        != 0
                    {
                        TINFL_STATUS_NEEDS_MORE_INPUT as c_int
                    } else {
                        TINFL_STATUS_FAILED_CANNOT_MAKE_PROGRESS as c_int
                    }) as tinfl_status;
                    (*r).m_state = 27 as mz_uint32;
                    current_block = 34;
                    continue;
                } else {
                    let fresh30 = pIn_buf_cur;
                    pIn_buf_cur = pIn_buf_cur.offset(1);
                    c_10 = *fresh30 as mz_uint;
                    bit_buf |= (c_10 as tinfl_bit_buf_t) << num_bits;
                    num_bits = num_bits.wrapping_add(8 as mz_uint32);
                    if num_bits < num_extra {
                        current_block = 40;
                    } else {
                        current_block = 41;
                    }
                }
            }
            26 => {
                if pIn_buf_cur >= pIn_buf_end {
                    status = (if decomp_flags
                        & TINFL_FLAG_HAS_MORE_INPUT as c_int as mz_uint32
                        != 0
                    {
                        TINFL_STATUS_NEEDS_MORE_INPUT as c_int
                    } else {
                        TINFL_STATUS_FAILED_CANNOT_MAKE_PROGRESS as c_int
                    }) as tinfl_status;
                    (*r).m_state = 26 as mz_uint32;
                    current_block = 34;
                    continue;
                } else {
                    let fresh28 = pIn_buf_cur;
                    pIn_buf_cur = pIn_buf_cur.offset(1);
                    c_9 = *fresh28 as mz_uint;
                    bit_buf |= (c_9 as tinfl_bit_buf_t) << num_bits;
                    num_bits = num_bits.wrapping_add(8 as mz_uint32);
                    if num_bits < 15 as mz_uint32 {
                        current_block = 42;
                    } else {
                        current_block = 43;
                    }
                }
            }
            25 => {
                if pIn_buf_cur >= pIn_buf_end {
                    status = (if decomp_flags
                        & TINFL_FLAG_HAS_MORE_INPUT as c_int as mz_uint32
                        != 0
                    {
                        TINFL_STATUS_NEEDS_MORE_INPUT as c_int
                    } else {
                        TINFL_STATUS_FAILED_CANNOT_MAKE_PROGRESS as c_int
                    }) as tinfl_status;
                    (*r).m_state = 25 as mz_uint32;
                    current_block = 34;
                    continue;
                } else {
                    let fresh26 = pIn_buf_cur;
                    pIn_buf_cur = pIn_buf_cur.offset(1);
                    c_8 = *fresh26 as mz_uint;
                    bit_buf |= (c_8 as tinfl_bit_buf_t) << num_bits;
                    num_bits = num_bits.wrapping_add(8 as mz_uint32);
                    if num_bits < num_extra {
                        current_block = 44;
                    } else {
                        current_block = 45;
                    }
                }
            }
            24 => {
                status = TINFL_STATUS_FAILED;
                (*r).m_state = 54 as mz_uint32;
                current_block = 34;
                continue;
            }
            23 => {
                status = TINFL_STATUS_FAILED;
                (*r).m_state = 40 as mz_uint32;
                current_block = 34;
                continue;
            }
            22 => {
                if pOut_buf_cur >= pOut_buf_end {
                    status = TINFL_STATUS_HAS_MORE_OUTPUT;
                    (*r).m_state = 24 as mz_uint32;
                    current_block = 34;
                    continue;
                } else {
                    let fresh23 = pOut_buf_cur;
                    pOut_buf_cur = pOut_buf_cur.offset(1);
                    *fresh23 = counter as mz_uint8;
                }
                current_block = 46;
            }
            21 => {
                if pIn_buf_cur >= pIn_buf_end {
                    status = (if decomp_flags
                        & TINFL_FLAG_HAS_MORE_INPUT as c_int as mz_uint32
                        != 0
                    {
                        TINFL_STATUS_NEEDS_MORE_INPUT as c_int
                    } else {
                        TINFL_STATUS_FAILED_CANNOT_MAKE_PROGRESS as c_int
                    }) as tinfl_status;
                    (*r).m_state = 23 as mz_uint32;
                    current_block = 34;
                    continue;
                } else {
                    let fresh21 = pIn_buf_cur;
                    pIn_buf_cur = pIn_buf_cur.offset(1);
                    c_7 = *fresh21 as mz_uint;
                    bit_buf |= (c_7 as tinfl_bit_buf_t) << num_bits;
                    num_bits = num_bits.wrapping_add(8 as mz_uint32);
                    if num_bits < 15 as mz_uint32 {
                        current_block = 47;
                    } else {
                        current_block = 48;
                    }
                }
            }
            20 => {
                status = TINFL_STATUS_FAILED;
                (*r).m_state = 21 as mz_uint32;
                current_block = 34;
                continue;
            }
            19 => {
                if pIn_buf_cur >= pIn_buf_end {
                    status = (if decomp_flags
                        & TINFL_FLAG_HAS_MORE_INPUT as c_int as mz_uint32
                        != 0
                    {
                        TINFL_STATUS_NEEDS_MORE_INPUT as c_int
                    } else {
                        TINFL_STATUS_FAILED_CANNOT_MAKE_PROGRESS as c_int
                    }) as tinfl_status;
                    (*r).m_state = 18 as mz_uint32;
                    current_block = 34;
                    continue;
                } else {
                    let fresh19 = pIn_buf_cur;
                    pIn_buf_cur = pIn_buf_cur.offset(1);
                    c_6 = *fresh19 as mz_uint;
                    bit_buf |= (c_6 as tinfl_bit_buf_t) << num_bits;
                    num_bits = num_bits.wrapping_add(8 as mz_uint32);
                    if num_bits < num_extra {
                        current_block = 49;
                    } else {
                        current_block = 50;
                    }
                }
            }
            18 => {
                status = TINFL_STATUS_FAILED;
                (*r).m_state = 17 as mz_uint32;
                current_block = 34;
                continue;
            }
            17 => {
                if pIn_buf_cur >= pIn_buf_end {
                    status = (if decomp_flags
                        & TINFL_FLAG_HAS_MORE_INPUT as c_int as mz_uint32
                        != 0
                    {
                        TINFL_STATUS_NEEDS_MORE_INPUT as c_int
                    } else {
                        TINFL_STATUS_FAILED_CANNOT_MAKE_PROGRESS as c_int
                    }) as tinfl_status;
                    (*r).m_state = 16 as mz_uint32;
                    current_block = 34;
                    continue;
                } else {
                    let fresh16 = pIn_buf_cur;
                    pIn_buf_cur = pIn_buf_cur.offset(1);
                    c_5 = *fresh16 as mz_uint;
                    bit_buf |= (c_5 as tinfl_bit_buf_t) << num_bits;
                    num_bits = num_bits.wrapping_add(8 as mz_uint32);
                    if num_bits < 15 as mz_uint32 {
                        current_block = 51;
                    } else {
                        current_block = 52;
                    }
                }
            }
            16 => {
                status = TINFL_STATUS_FAILED;
                (*r).m_state = 35 as mz_uint32;
                current_block = 34;
                continue;
            }
            15 => {
                if pIn_buf_cur >= pIn_buf_end {
                    status = (if decomp_flags
                        & TINFL_FLAG_HAS_MORE_INPUT as c_int as mz_uint32
                        != 0
                    {
                        TINFL_STATUS_NEEDS_MORE_INPUT as c_int
                    } else {
                        TINFL_STATUS_FAILED_CANNOT_MAKE_PROGRESS as c_int
                    }) as tinfl_status;
                    (*r).m_state = 14 as mz_uint32;
                    current_block = 34;
                    continue;
                } else {
                    let fresh13 = pIn_buf_cur;
                    pIn_buf_cur = pIn_buf_cur.offset(1);
                    c_4 = *fresh13 as mz_uint;
                    bit_buf |= (c_4 as tinfl_bit_buf_t) << num_bits;
                    num_bits = num_bits.wrapping_add(8 as mz_uint32);
                    if num_bits < 3 as c_int as mz_uint {
                        current_block = 53;
                    } else {
                        current_block = 54;
                    }
                }
            }
            14 => {
                if pIn_buf_cur >= pIn_buf_end {
                    status = (if decomp_flags
                        & TINFL_FLAG_HAS_MORE_INPUT as c_int as mz_uint32
                        != 0
                    {
                        TINFL_STATUS_NEEDS_MORE_INPUT as c_int
                    } else {
                        TINFL_STATUS_FAILED_CANNOT_MAKE_PROGRESS as c_int
                    }) as tinfl_status;
                    (*r).m_state = 11 as mz_uint32;
                    current_block = 34;
                    continue;
                } else {
                    let fresh12 = pIn_buf_cur;
                    pIn_buf_cur = pIn_buf_cur.offset(1);
                    c_3 = *fresh12 as mz_uint;
                    bit_buf |= (c_3 as tinfl_bit_buf_t) << num_bits;
                    num_bits = num_bits.wrapping_add(8 as mz_uint32);
                    if num_bits
                        < ::core::mem::transmute::<[u8; 4], [c_char; 4]>(
                            *b"\x05\x05\x04\0",
                        )[counter as usize] as mz_uint
                    {
                        current_block = 55;
                    } else {
                        current_block = 56;
                    }
                }
            }
            13 => {
                status = TINFL_STATUS_FAILED;
                (*r).m_state = 10 as mz_uint32;
                current_block = 34;
                continue;
            }
            12 => {
                if pIn_buf_cur >= pIn_buf_end {
                    status = (if decomp_flags
                        & TINFL_FLAG_HAS_MORE_INPUT as c_int as mz_uint32
                        != 0
                    {
                        TINFL_STATUS_NEEDS_MORE_INPUT as c_int
                    } else {
                        TINFL_STATUS_FAILED_CANNOT_MAKE_PROGRESS as c_int
                    }) as tinfl_status;
                    (*r).m_state = 38 as mz_uint32;
                    current_block = 34;
                    continue;
                } else {
                    n = if (if (pOut_buf_end.offset_from(pOut_buf_cur) as c_long
                        as size_t)
                        < pIn_buf_end.offset_from(pIn_buf_cur) as c_long as size_t
                    {
                        pOut_buf_end.offset_from(pOut_buf_cur) as c_long as size_t
                    } else {
                        pIn_buf_end.offset_from(pIn_buf_cur) as c_long as size_t
                    }) < counter as size_t
                    {
                        if (pOut_buf_end.offset_from(pOut_buf_cur) as c_long as size_t)
                            < pIn_buf_end.offset_from(pIn_buf_cur) as c_long as size_t
                        {
                            pOut_buf_end.offset_from(pOut_buf_cur) as c_long as size_t
                        } else {
                            pIn_buf_end.offset_from(pIn_buf_cur) as c_long as size_t
                        }
                    } else {
                        counter as size_t
                    };
                    memcpy(
                        pOut_buf_cur as *mut c_void,
                        pIn_buf_cur as *const c_void,
                        n,
                    );
                    pIn_buf_cur = pIn_buf_cur.offset(n as isize);
                    pOut_buf_cur = pOut_buf_cur.offset(n as isize);
                    counter = (counter as uint32_t).wrapping_sub(n as mz_uint as uint32_t)
                        as mz_uint32 as mz_uint32;
                }
                current_block = 57;
            }
            11 => {
                if !(pOut_buf_cur >= pOut_buf_end) {
                    current_block = 12;
                    continue;
                }
                status = TINFL_STATUS_HAS_MORE_OUTPUT;
                (*r).m_state = 9 as mz_uint32;
                current_block = 34;
                continue;
            }
            10 => {
                if pOut_buf_cur >= pOut_buf_end {
                    status = TINFL_STATUS_HAS_MORE_OUTPUT;
                    (*r).m_state = 52 as mz_uint32;
                    current_block = 34;
                    continue;
                } else {
                    let fresh7 = pOut_buf_cur;
                    pOut_buf_cur = pOut_buf_cur.offset(1);
                    *fresh7 = dist as mz_uint8;
                    counter = counter.wrapping_sub(1);
                }
                current_block = 58;
            }
            9 => {
                if pIn_buf_cur >= pIn_buf_end {
                    status = (if decomp_flags
                        & TINFL_FLAG_HAS_MORE_INPUT as c_int as mz_uint32
                        != 0
                    {
                        TINFL_STATUS_NEEDS_MORE_INPUT as c_int
                    } else {
                        TINFL_STATUS_FAILED_CANNOT_MAKE_PROGRESS as c_int
                    }) as tinfl_status;
                    (*r).m_state = 51 as mz_uint32;
                    current_block = 34;
                    continue;
                } else {
                    let fresh6 = pIn_buf_cur;
                    pIn_buf_cur = pIn_buf_cur.offset(1);
                    c_2 = *fresh6 as mz_uint;
                    bit_buf |= (c_2 as tinfl_bit_buf_t) << num_bits;
                    num_bits = num_bits.wrapping_add(8 as mz_uint32);
                    if num_bits < 8 as c_int as mz_uint {
                        current_block = 59;
                    } else {
                        current_block = 60;
                    }
                }
            }
            8 => {
                status = TINFL_STATUS_FAILED;
                (*r).m_state = 39 as mz_uint32;
                current_block = 34;
                continue;
            }
            7 => {
                if pIn_buf_cur >= pIn_buf_end {
                    status = (if decomp_flags
                        & TINFL_FLAG_HAS_MORE_INPUT as c_int as mz_uint32
                        != 0
                    {
                        TINFL_STATUS_NEEDS_MORE_INPUT as c_int
                    } else {
                        TINFL_STATUS_FAILED_CANNOT_MAKE_PROGRESS as c_int
                    }) as tinfl_status;
                    (*r).m_state = 7 as mz_uint32;
                    current_block = 34;
                    continue;
                } else {
                    let fresh5 = pIn_buf_cur;
                    pIn_buf_cur = pIn_buf_cur.offset(1);
                    (*r).m_raw_header[counter as usize] = *fresh5;
                }
                current_block = 61;
            }
            6 => {
                if pIn_buf_cur >= pIn_buf_end {
                    status = (if decomp_flags
                        & TINFL_FLAG_HAS_MORE_INPUT as c_int as mz_uint32
                        != 0
                    {
                        TINFL_STATUS_NEEDS_MORE_INPUT as c_int
                    } else {
                        TINFL_STATUS_FAILED_CANNOT_MAKE_PROGRESS as c_int
                    }) as tinfl_status;
                    (*r).m_state = 6 as mz_uint32;
                    current_block = 34;
                    continue;
                } else {
                    let fresh4 = pIn_buf_cur;
                    pIn_buf_cur = pIn_buf_cur.offset(1);
                    c_1 = *fresh4 as mz_uint;
                    bit_buf |= (c_1 as tinfl_bit_buf_t) << num_bits;
                    num_bits = num_bits.wrapping_add(8 as mz_uint32);
                    if num_bits < 8 as c_int as mz_uint {
                        current_block = 62;
                    } else {
                        current_block = 63;
                    }
                }
            }
            5 => {
                if pIn_buf_cur >= pIn_buf_end {
                    status = (if decomp_flags
                        & TINFL_FLAG_HAS_MORE_INPUT as c_int as mz_uint32
                        != 0
                    {
                        TINFL_STATUS_NEEDS_MORE_INPUT as c_int
                    } else {
                        TINFL_STATUS_FAILED_CANNOT_MAKE_PROGRESS as c_int
                    }) as tinfl_status;
                    (*r).m_state = 5 as mz_uint32;
                    current_block = 34;
                    continue;
                } else {
                    let fresh3 = pIn_buf_cur;
                    pIn_buf_cur = pIn_buf_cur.offset(1);
                    c_0 = *fresh3 as mz_uint;
                    bit_buf |= (c_0 as tinfl_bit_buf_t) << num_bits;
                    num_bits = num_bits.wrapping_add(8 as mz_uint32);
                    if num_bits < num_bits & 7 as mz_uint32 {
                        current_block = 64;
                    } else {
                        current_block = 65;
                    }
                }
            }
            4 => {
                if pIn_buf_cur >= pIn_buf_end {
                    status = (if decomp_flags
                        & TINFL_FLAG_HAS_MORE_INPUT as c_int as mz_uint32
                        != 0
                    {
                        TINFL_STATUS_NEEDS_MORE_INPUT as c_int
                    } else {
                        TINFL_STATUS_FAILED_CANNOT_MAKE_PROGRESS as c_int
                    }) as tinfl_status;
                    (*r).m_state = 3 as mz_uint32;
                    current_block = 34;
                    continue;
                } else {
                    let fresh2 = pIn_buf_cur;
                    pIn_buf_cur = pIn_buf_cur.offset(1);
                    c = *fresh2 as mz_uint;
                    bit_buf |= (c as tinfl_bit_buf_t) << num_bits;
                    num_bits = num_bits.wrapping_add(8 as mz_uint32);
                    if num_bits < 3 as c_int as mz_uint {
                        current_block = 35;
                    } else {
                        current_block = 36;
                    }
                }
            }
            32 => {
                if pIn_buf_cur >= pIn_buf_end {
                    status = (if decomp_flags
                        & TINFL_FLAG_HAS_MORE_INPUT as c_int as mz_uint32
                        != 0
                    {
                        TINFL_STATUS_NEEDS_MORE_INPUT as c_int
                    } else {
                        TINFL_STATUS_FAILED_CANNOT_MAKE_PROGRESS as c_int
                    }) as tinfl_status;
                    (*r).m_state = 42 as mz_uint32;
                    current_block = 34;
                    continue;
                } else {
                    let fresh36 = pIn_buf_cur;
                    pIn_buf_cur = pIn_buf_cur.offset(1);
                    s_1 = *fresh36 as mz_uint;
                }
                current_block = 66;
            }
            33 => {
                status = TINFL_STATUS_DONE;
                (*r).m_state = 34 as mz_uint32;
                current_block = 34;
                continue;
            }
            34 => {
                if status as c_int
                    != TINFL_STATUS_NEEDS_MORE_INPUT as c_int
                    && status as c_int
                        != TINFL_STATUS_FAILED_CANNOT_MAKE_PROGRESS as c_int
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
                        & TINFL_FLAG_HAS_MORE_INPUT as c_int as mz_uint32
                        != 0
                    {
                        TINFL_STATUS_NEEDS_MORE_INPUT as c_int
                    } else {
                        TINFL_STATUS_FAILED_CANNOT_MAKE_PROGRESS as c_int
                    }) as tinfl_status;
                    (*r).m_state = 41 as mz_uint32;
                    current_block = 34;
                    continue;
                } else {
                    let fresh35 = pIn_buf_cur;
                    pIn_buf_cur = pIn_buf_cur.offset(1);
                    c_12 = *fresh35 as mz_uint;
                    bit_buf |= (c_12 as tinfl_bit_buf_t) << num_bits;
                    num_bits = num_bits.wrapping_add(8 as mz_uint32);
                    if num_bits < 8 as c_int as mz_uint {
                        current_block = 67;
                    } else {
                        current_block = 68;
                    }
                }
            }
        }
        match current_block {
            36 => {
                (*r).m_final = (bit_buf
                    & (((1 as c_int) << 3 as c_int)
                        - 1 as c_int) as tinfl_bit_buf_t)
                    as mz_uint32;
                bit_buf >>= 3 as c_int;
                num_bits = num_bits.wrapping_sub(3 as mz_uint32);
                (*r).m_type = (*r).m_final >> 1 as c_int;
                if (*r).m_type == 0 as mz_uint32 {
                    if num_bits < num_bits & 7 as mz_uint32 {
                        current_block = 64;
                    } else {
                        current_block = 65;
                    }
                } else {
                    if (*r).m_type == 3 as mz_uint32 {
                        current_block = 13;
                        continue;
                    }
                    if (*r).m_type == 1 as mz_uint32 {
                        let mut p: *mut mz_uint8 = &raw mut (*r).m_code_size_0 as *mut mz_uint8;
                        let mut i: mz_uint = 0;
                        (*r).m_table_sizes[0 as c_int as usize] = 288 as mz_uint32;
                        (*r).m_table_sizes[1 as c_int as usize] = 32 as mz_uint32;
                        memset(
                            &raw mut (*r).m_code_size_1 as *mut mz_uint8
                                as *mut c_void,
                            5 as c_int,
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
                        current_block = 69;
                    } else {
                        counter = 0 as mz_uint32;
                        current_block = 70;
                    }
                }
            }
            35 => {
                c = 0;
                current_block = 4;
                continue;
            }
            _ => {}
        }
        match current_block {
            65 => {
                bit_buf >>= num_bits & 7 as mz_uint32;
                num_bits = num_bits.wrapping_sub(num_bits & 7 as mz_uint32);
                counter = 0 as mz_uint32;
                current_block = 71;
            }
            64 => {
                c_0 = 0;
                current_block = 5;
                continue;
            }
            _ => {}
        }
        loop {
            match current_block {
                62 => {
                    c_1 = 0;
                    current_block = 6;
                    break;
                }
                38 => {
                    bit_buf >>= num_bits & 7 as mz_uint32;
                    num_bits = num_bits.wrapping_sub(num_bits & 7 as mz_uint32);
                    while pIn_buf_cur > pIn_buf_next && num_bits >= 8 as mz_uint32 {
                        pIn_buf_cur = pIn_buf_cur.offset(-1);
                        num_bits = num_bits.wrapping_sub(8 as mz_uint32);
                    }
                    bit_buf &= !(!(0 as c_int as tinfl_bit_buf_t) << num_bits);
                    if !(decomp_flags
                        & TINFL_FLAG_PARSE_ZLIB_HEADER as c_int as mz_uint32
                        != 0)
                    {
                        current_block = 33;
                        break;
                    }
                    counter = 0 as mz_uint32;
                    current_block = 72;
                }
                41 => {
                    extra_bits_0 = (bit_buf
                        & (((1 as c_int) << num_extra) - 1 as c_int)
                            as tinfl_bit_buf_t) as mz_uint;
                    bit_buf >>= num_extra;
                    num_bits = num_bits.wrapping_sub(num_extra);
                    dist = (dist as uint32_t).wrapping_add(extra_bits_0 as uint32_t) as mz_uint32
                        as mz_uint32;
                    current_block = 73;
                }
                43 => {
                    temp_1 = (*r).m_look_up[1 as c_int as usize][(bit_buf
                        & (TINFL_FAST_LOOKUP_SIZE as c_int - 1 as c_int)
                            as tinfl_bit_buf_t)
                        as usize] as c_int;
                    if temp_1 >= 0 as c_int {
                        code_len_2 = (temp_1 >> 9 as c_int) as mz_uint;
                        temp_1 &= 511 as c_int;
                    } else {
                        code_len_2 = TINFL_FAST_LOOKUP_BITS as c_int as mz_uint;
                        loop {
                            let fresh29 = code_len_2;
                            code_len_2 = code_len_2.wrapping_add(1);
                            temp_1 = (*r).m_tree_1[(!temp_1 as tinfl_bit_buf_t)
                                .wrapping_add(bit_buf >> fresh29 & 1 as tinfl_bit_buf_t)
                                as usize]
                                as c_int;
                            if !(temp_1 < 0 as c_int) {
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
                            current_block = 40;
                            continue;
                        } else {
                            current_block = 41;
                            continue;
                        }
                    } else {
                        current_block = 73;
                    }
                }
                42 => {
                    temp_1 = (*r).m_look_up[1 as c_int as usize][(bit_buf
                        & (TINFL_FAST_LOOKUP_SIZE as c_int - 1 as c_int)
                            as tinfl_bit_buf_t)
                        as usize] as c_int;
                    if temp_1 >= 0 as c_int {
                        code_len_2 = (temp_1 >> 9 as c_int) as mz_uint;
                        if code_len_2 != 0 && num_bits >= code_len_2 {
                            current_block = 43;
                            continue;
                        } else {
                            current_block = 26;
                            break;
                        }
                    } else {
                        if !(num_bits > TINFL_FAST_LOOKUP_BITS as c_int as mz_uint32) {
                            current_block = 26;
                            break;
                        }
                        code_len_2 = TINFL_FAST_LOOKUP_BITS as c_int as mz_uint;
                        loop {
                            let fresh27 = code_len_2;
                            code_len_2 = code_len_2.wrapping_add(1);
                            temp_1 = (*r).m_tree_1[(!temp_1 as tinfl_bit_buf_t)
                                .wrapping_add(bit_buf >> fresh27 & 1 as tinfl_bit_buf_t)
                                as usize]
                                as c_int;
                            if !(temp_1 < 0 as c_int
                                && num_bits >= code_len_2.wrapping_add(1 as mz_uint))
                            {
                                break;
                            }
                        }
                        if temp_1 >= 0 as c_int {
                            current_block = 43;
                            continue;
                        } else {
                            current_block = 26;
                            break;
                        }
                    }
                }
                45 => {
                    extra_bits = (bit_buf
                        & (((1 as c_int) << num_extra) - 1 as c_int)
                            as tinfl_bit_buf_t) as mz_uint;
                    bit_buf >>= num_extra;
                    num_bits = num_bits.wrapping_sub(num_extra);
                    counter = (counter as uint32_t).wrapping_add(extra_bits as uint32_t)
                        as mz_uint32 as mz_uint32;
                    current_block = 74;
                }
                46 => {
                    if (pIn_buf_end.offset_from(pIn_buf_cur) as c_long)
                        < 4 as c_long
                        || (pOut_buf_end.offset_from(pOut_buf_cur) as c_long)
                            < 2 as c_long
                    {
                        temp_0 = 0;
                        code_len_0 = 0;
                        c_7 = 0;
                        if !(num_bits < 15 as mz_uint32) {
                            current_block = 48;
                            continue;
                        }
                        if (pIn_buf_end.offset_from(pIn_buf_cur) as c_long)
                            < 2 as c_long
                        {
                            current_block = 47;
                            continue;
                        }
                        bit_buf |= (*pIn_buf_cur.offset(0 as c_int as isize)
                            as tinfl_bit_buf_t)
                            << num_bits
                            | (*pIn_buf_cur.offset(1 as c_int as isize)
                                as tinfl_bit_buf_t)
                                << num_bits.wrapping_add(8 as mz_uint32);
                        pIn_buf_cur = pIn_buf_cur.offset(2 as c_int as isize);
                        num_bits = num_bits.wrapping_add(16 as mz_uint32);
                        current_block = 48;
                        continue;
                    } else {
                        sym2 = 0;
                        code_len_1 = 0;
                        if num_bits < 30 as mz_uint32 {
                            bit_buf |= ((*pIn_buf_cur.offset(0 as c_int as isize)
                                as mz_uint32
                                | (*pIn_buf_cur.offset(1 as c_int as isize)
                                    as mz_uint32)
                                    << 8 as c_uint
                                | (*pIn_buf_cur.offset(2 as c_int as isize)
                                    as mz_uint32)
                                    << 16 as c_uint
                                | (*pIn_buf_cur.offset(3 as c_int as isize)
                                    as mz_uint32)
                                    << 24 as c_uint)
                                as tinfl_bit_buf_t)
                                << num_bits;
                            pIn_buf_cur = pIn_buf_cur.offset(4 as c_int as isize);
                            num_bits = num_bits.wrapping_add(32 as mz_uint32);
                        }
                        sym2 = (*r).m_look_up[0 as c_int as usize][(bit_buf
                            & (TINFL_FAST_LOOKUP_SIZE as c_int
                                - 1 as c_int)
                                as tinfl_bit_buf_t)
                            as usize] as c_int;
                        if sym2 >= 0 as c_int {
                            code_len_1 = (sym2 >> 9 as c_int) as mz_uint;
                        } else {
                            code_len_1 = TINFL_FAST_LOOKUP_BITS as c_int as mz_uint;
                            loop {
                                let fresh24 = code_len_1;
                                code_len_1 = code_len_1.wrapping_add(1);
                                sym2 = (*r).m_tree_0[(!sym2 as tinfl_bit_buf_t)
                                    .wrapping_add(bit_buf >> fresh24 & 1 as tinfl_bit_buf_t)
                                    as usize]
                                    as c_int;
                                if !(sym2 < 0 as c_int) {
                                    break;
                                }
                            }
                        }
                        counter = sym2 as mz_uint32;
                        bit_buf >>= code_len_1;
                        num_bits = (num_bits as uint32_t).wrapping_sub(code_len_1 as uint32_t)
                            as mz_uint32 as mz_uint32;
                        if code_len_1 == 0 as mz_uint {
                            current_block = 23;
                            break;
                        }
                        if !(counter & 256 as mz_uint32 != 0) {
                            sym2 = (*r).m_look_up[0 as c_int as usize][(bit_buf
                                & (TINFL_FAST_LOOKUP_SIZE as c_int
                                    - 1 as c_int)
                                    as tinfl_bit_buf_t)
                                as usize] as c_int;
                            if sym2 >= 0 as c_int {
                                code_len_1 = (sym2 >> 9 as c_int) as mz_uint;
                            } else {
                                code_len_1 =
                                    TINFL_FAST_LOOKUP_BITS as c_int as mz_uint;
                                loop {
                                    let fresh25 = code_len_1;
                                    code_len_1 = code_len_1.wrapping_add(1);
                                    sym2 = (*r).m_tree_0[(!sym2 as tinfl_bit_buf_t)
                                        .wrapping_add(bit_buf >> fresh25 & 1 as tinfl_bit_buf_t)
                                        as usize]
                                        as c_int;
                                    if !(sym2 < 0 as c_int) {
                                        break;
                                    }
                                }
                            }
                            bit_buf >>= code_len_1;
                            num_bits = (num_bits as uint32_t).wrapping_sub(code_len_1 as uint32_t)
                                as mz_uint32 as mz_uint32;
                            if code_len_1 == 0 as mz_uint {
                                current_block = 24;
                                break;
                            }
                            *pOut_buf_cur.offset(0 as c_int as isize) =
                                counter as mz_uint8;
                            if sym2 & 256 as c_int != 0 {
                                pOut_buf_cur = pOut_buf_cur.offset(1);
                                counter = sym2 as mz_uint32;
                            } else {
                                *pOut_buf_cur.offset(1 as c_int as isize) =
                                    sym2 as mz_uint8;
                                pOut_buf_cur =
                                    pOut_buf_cur.offset(2 as c_int as isize);
                                current_block = 46;
                                continue;
                            }
                        }
                    }
                    current_block = 75;
                }
                48 => {
                    temp_0 = (*r).m_look_up[0 as c_int as usize][(bit_buf
                        & (TINFL_FAST_LOOKUP_SIZE as c_int - 1 as c_int)
                            as tinfl_bit_buf_t)
                        as usize] as c_int;
                    if temp_0 >= 0 as c_int {
                        code_len_0 = (temp_0 >> 9 as c_int) as mz_uint;
                        temp_0 &= 511 as c_int;
                    } else {
                        code_len_0 = TINFL_FAST_LOOKUP_BITS as c_int as mz_uint;
                        loop {
                            let fresh22 = code_len_0;
                            code_len_0 = code_len_0.wrapping_add(1);
                            temp_0 = (*r).m_tree_0[(!temp_0 as tinfl_bit_buf_t)
                                .wrapping_add(bit_buf >> fresh22 & 1 as tinfl_bit_buf_t)
                                as usize]
                                as c_int;
                            if !(temp_0 < 0 as c_int) {
                                break;
                            }
                        }
                    }
                    counter = temp_0 as mz_uint32;
                    bit_buf >>= code_len_0;
                    num_bits = (num_bits as uint32_t).wrapping_sub(code_len_0 as uint32_t)
                        as mz_uint32 as mz_uint32;
                    if !(counter >= 256 as mz_uint32) {
                        current_block = 22;
                        break;
                    }
                    current_block = 75;
                }
                47 => {
                    temp_0 = (*r).m_look_up[0 as c_int as usize][(bit_buf
                        & (TINFL_FAST_LOOKUP_SIZE as c_int - 1 as c_int)
                            as tinfl_bit_buf_t)
                        as usize] as c_int;
                    if temp_0 >= 0 as c_int {
                        code_len_0 = (temp_0 >> 9 as c_int) as mz_uint;
                        if code_len_0 != 0 && num_bits >= code_len_0 {
                            current_block = 48;
                            continue;
                        } else {
                            current_block = 21;
                            break;
                        }
                    } else {
                        if !(num_bits > TINFL_FAST_LOOKUP_BITS as c_int as mz_uint32) {
                            current_block = 21;
                            break;
                        }
                        code_len_0 = TINFL_FAST_LOOKUP_BITS as c_int as mz_uint;
                        loop {
                            let fresh20 = code_len_0;
                            code_len_0 = code_len_0.wrapping_add(1);
                            temp_0 = (*r).m_tree_0[(!temp_0 as tinfl_bit_buf_t)
                                .wrapping_add(bit_buf >> fresh20 & 1 as tinfl_bit_buf_t)
                                as usize]
                                as c_int;
                            if !(temp_0 < 0 as c_int
                                && num_bits >= code_len_0.wrapping_add(1 as mz_uint))
                            {
                                break;
                            }
                        }
                        if temp_0 >= 0 as c_int {
                            current_block = 48;
                            continue;
                        } else {
                            current_block = 21;
                            break;
                        }
                    }
                }
                50 => {
                    s_0 = (bit_buf
                        & (((1 as c_int) << num_extra) - 1 as c_int)
                            as tinfl_bit_buf_t) as mz_uint;
                    bit_buf >>= num_extra;
                    num_bits = num_bits.wrapping_sub(num_extra);
                    s_0 = s_0.wrapping_add(
                        ::core::mem::transmute::<[u8; 4], [c_char; 4]>(
                            *b"\x03\x03\x0B\0",
                        )[dist.wrapping_sub(16 as mz_uint32) as usize]
                            as mz_uint,
                    );
                    memset(
                        (&raw mut (*r).m_len_codes as *mut mz_uint8).offset(counter as isize)
                            as *mut c_void,
                        if dist == 16 as mz_uint32 {
                            (*r).m_len_codes[counter.wrapping_sub(1 as mz_uint32) as usize]
                                as c_int
                        } else {
                            0 as c_int
                        },
                        s_0 as size_t,
                    );
                    counter = (counter as uint32_t).wrapping_add(s_0 as uint32_t) as mz_uint32
                        as mz_uint32;
                    current_block = 76;
                }
                52 => {
                    temp = (*r).m_look_up[2 as c_int as usize][(bit_buf
                        & (TINFL_FAST_LOOKUP_SIZE as c_int - 1 as c_int)
                            as tinfl_bit_buf_t)
                        as usize] as c_int;
                    if temp >= 0 as c_int {
                        code_len = (temp >> 9 as c_int) as mz_uint;
                        temp &= 511 as c_int;
                    } else {
                        code_len = TINFL_FAST_LOOKUP_BITS as c_int as mz_uint;
                        loop {
                            let fresh17 = code_len;
                            code_len = code_len.wrapping_add(1);
                            temp = (*r).m_tree_2[(!temp as tinfl_bit_buf_t)
                                .wrapping_add(bit_buf >> fresh17 & 1 as tinfl_bit_buf_t)
                                as usize] as c_int;
                            if !(temp < 0 as c_int) {
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
                            current_block = 18;
                            break;
                        }
                        num_extra = ::core::mem::transmute::<[u8; 4], [c_char; 4]>(
                            *b"\x02\x03\x07\0",
                        )[dist.wrapping_sub(16 as mz_uint32) as usize]
                            as mz_uint32;
                        if num_bits < num_extra {
                            current_block = 49;
                            continue;
                        } else {
                            current_block = 50;
                            continue;
                        }
                    }
                    current_block = 76;
                }
                51 => {
                    temp = (*r).m_look_up[2 as c_int as usize][(bit_buf
                        & (TINFL_FAST_LOOKUP_SIZE as c_int - 1 as c_int)
                            as tinfl_bit_buf_t)
                        as usize] as c_int;
                    if temp >= 0 as c_int {
                        code_len = (temp >> 9 as c_int) as mz_uint;
                        if code_len != 0 && num_bits >= code_len {
                            current_block = 52;
                            continue;
                        } else {
                            current_block = 17;
                            break;
                        }
                    } else {
                        if !(num_bits > TINFL_FAST_LOOKUP_BITS as c_int as mz_uint32) {
                            current_block = 17;
                            break;
                        }
                        code_len = TINFL_FAST_LOOKUP_BITS as c_int as mz_uint;
                        loop {
                            let fresh15 = code_len;
                            code_len = code_len.wrapping_add(1);
                            temp = (*r).m_tree_2[(!temp as tinfl_bit_buf_t)
                                .wrapping_add(bit_buf >> fresh15 & 1 as tinfl_bit_buf_t)
                                as usize] as c_int;
                            if !(temp < 0 as c_int
                                && num_bits >= code_len.wrapping_add(1 as mz_uint))
                            {
                                break;
                            }
                        }
                        if temp >= 0 as c_int {
                            current_block = 52;
                            continue;
                        } else {
                            current_block = 17;
                            break;
                        }
                    }
                }
                69 => {
                    if (*r).m_type as c_int >= 0 as c_int {
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
                        let m_type = (*r).m_type as usize;
                        let table_size = (*r).m_table_sizes[m_type] as mz_uint;
                        pLookUp = (&raw mut (*r).m_look_up[m_type]) as *mut [mz_int16; 1024]
                            as *mut mz_int16;
                        pTree = pTrees[m_type];
                        pCode_size = pCode_sizes[m_type];
                        memset(
                            &raw mut total_syms as *mut mz_uint as *mut c_void,
                            0 as c_int,
                            ::core::mem::size_of::<[mz_uint; 16]>() as size_t,
                        );
                        memset(
                            pLookUp as *mut c_void,
                            0 as c_int,
                            ::core::mem::size_of::<[mz_int16; 1024]>() as size_t,
                        );
                        tinfl_clear_tree(r);
                        i_0 = 0 as mz_uint;
                        while i_0 < table_size {
                            let code_size = *pCode_size.offset(i_0 as isize) as usize;
                            total_syms[code_size] =
                                total_syms[code_size].wrapping_add(1);
                            i_0 = i_0.wrapping_add(1);
                        }
                        used_syms = 0 as mz_uint;
                        total = 0 as mz_uint;
                        next_code[1 as c_int as usize] = 0 as mz_uint;
                        next_code[0 as c_int as usize] =
                            next_code[1 as c_int as usize];
                        i_0 = 1 as mz_uint;
                        while i_0 <= 15 as mz_uint {
                            let ts = total_syms[i_0 as usize];
                            used_syms = used_syms.wrapping_add(ts);
                            total = total.wrapping_add(ts) << 1 as c_int;
                            next_code[i_0.wrapping_add(1 as mz_uint) as usize] = total;
                            i_0 = i_0.wrapping_add(1);
                        }
                        if 65536 as mz_uint != total && used_syms > 1 as mz_uint {
                            current_block = 16;
                            break;
                        }
                        tree_next = -(1 as c_int);
                        sym_index = 0 as mz_uint;
                        while sym_index < table_size {
                            let mut rev_code: mz_uint = 0 as mz_uint;
                            let mut l: mz_uint = 0;
                            let mut cur_code: mz_uint = 0;
                            let code_size: mz_uint =
                                *pCode_size.offset(sym_index as isize) as mz_uint;
                            if !(code_size == 0) {
                                let fresh14 = next_code[code_size as usize];
                                next_code[code_size as usize] =
                                    next_code[code_size as usize].wrapping_add(1);
                                cur_code = fresh14;
                                l = code_size;
                                while l > 0 as mz_uint {
                                    rev_code = rev_code << 1 as c_int
                                        | cur_code & 1 as mz_uint;
                                    l = l.wrapping_sub(1);
                                    cur_code >>= 1 as c_int;
                                }
                                if code_size
                                    <= TINFL_FAST_LOOKUP_BITS as c_int as mz_uint
                                {
                                    let k: mz_int16 = (code_size << 9 as c_int
                                        | sym_index)
                                        as mz_int16;
                                    while rev_code
                                        < TINFL_FAST_LOOKUP_SIZE as c_int as mz_uint
                                    {
                                        *pLookUp.offset(rev_code as isize) = k;
                                        rev_code = rev_code.wrapping_add(
                                            ((1 as c_int) << code_size) as mz_uint,
                                        );
                                    }
                                } else {
                                    let lookup_idx = (rev_code
                                        & (TINFL_FAST_LOOKUP_SIZE as c_int - 1 as c_int)
                                            as mz_uint) as isize;
                                    tree_cur = *pLookUp.offset(lookup_idx) as c_int;
                                    if 0 as c_int == tree_cur {
                                        *pLookUp.offset(lookup_idx) = tree_next as mz_int16;
                                        tree_cur = tree_next;
                                        tree_next -= 2 as c_int;
                                    }
                                    rev_code >>= TINFL_FAST_LOOKUP_BITS as c_int
                                        - 1 as c_int;
                                    j = code_size;
                                    while j
                                        > (TINFL_FAST_LOOKUP_BITS as c_int
                                            + 1 as c_int)
                                            as mz_uint
                                    {
                                        rev_code >>= 1 as c_int;
                                        tree_cur = (tree_cur as mz_uint)
                                            .wrapping_sub(rev_code & 1 as mz_uint)
                                            as c_int
                                            as c_int;
                                        let tree_idx = (-tree_cur - 1 as c_int) as isize;
                                        if *pTree.offset(tree_idx) == 0 {
                                            *pTree.offset(tree_idx) = tree_next as mz_int16;
                                            tree_cur = tree_next;
                                            tree_next -= 2 as c_int;
                                        } else {
                                            tree_cur = *pTree.offset(tree_idx) as c_int;
                                        }
                                        j = j.wrapping_sub(1);
                                    }
                                    rev_code >>= 1 as c_int;
                                    tree_cur = (tree_cur as mz_uint)
                                        .wrapping_sub(rev_code & 1 as mz_uint)
                                        as c_int
                                        as c_int;
                                    *pTree.offset((-tree_cur - 1 as c_int) as isize) =
                                        sym_index as mz_int16;
                                }
                            }
                            sym_index = sym_index.wrapping_add(1);
                        }
                        if (*r).m_type == 2 as mz_uint32 {
                            counter = 0 as mz_uint32;
                            current_block = 76;
                        } else {
                            current_block = 77;
                        }
                    } else {
                        current_block = 78;
                    }
                }
                54 => {
                    s = (bit_buf
                        & (((1 as c_int) << 3 as c_int)
                            - 1 as c_int) as tinfl_bit_buf_t)
                        as mz_uint;
                    bit_buf >>= 3 as c_int;
                    num_bits = num_bits.wrapping_sub(3 as mz_uint32);
                    (*r).m_code_size_2[s_length_dezigzag[counter as usize] as usize] =
                        s as mz_uint8;
                    counter = counter.wrapping_add(1);
                    current_block = 79;
                }
                70 => {
                    if counter < 3 as mz_uint32 {
                        if num_bits
                            < ::core::mem::transmute::<[u8; 4], [c_char; 4]>(
                                *b"\x05\x05\x04\0",
                            )[counter as usize] as mz_uint
                        {
                            current_block = 55;
                            continue;
                        } else {
                            current_block = 56;
                            continue;
                        }
                    } else {
                        memset(
                            &raw mut (*r).m_code_size_2 as *mut mz_uint8
                                as *mut c_void,
                            0 as c_int,
                            ::core::mem::size_of::<[mz_uint8; 19]>() as size_t,
                        );
                        counter = 0 as mz_uint32;
                    }
                    current_block = 79;
                }
                56 => {
                    (*r).m_table_sizes[counter as usize] = (bit_buf
                        & (((1 as c_int)
                            << ::core::mem::transmute::<[u8; 4], [c_char; 4]>(
                                *b"\x05\x05\x04\0",
                            )[counter as usize]
                                as c_int)
                            - 1 as c_int) as tinfl_bit_buf_t)
                        as mz_uint32;
                    bit_buf >>= ::core::mem::transmute::<[u8; 4], [c_char; 4]>(
                        *b"\x05\x05\x04\0",
                    )[counter as usize] as c_int;
                    num_bits = num_bits.wrapping_sub(
                        ::core::mem::transmute::<[u8; 4], [c_char; 4]>(
                            *b"\x05\x05\x04\0",
                        )[counter as usize] as mz_uint32,
                    );
                    (*r).m_table_sizes[counter as usize] = (*r).m_table_sizes[counter as usize]
                        .wrapping_add(s_min_table_sizes[counter as usize] as mz_uint32);
                    counter = counter.wrapping_add(1);
                    current_block = 70;
                    continue;
                }
                57 => {
                    if counter != 0 {
                        n = 0;
                        current_block = 11;
                        break;
                    } else {
                        current_block = 80;
                    }
                }
                58 => {
                    if !(counter != 0 && num_bits != 0) {
                        current_block = 57;
                        continue;
                    }
                    if num_bits < 8 as c_int as mz_uint {
                        current_block = 59;
                        continue;
                    } else {
                        current_block = 60;
                        continue;
                    }
                }
                63 => {
                    (*r).m_raw_header[counter as usize] = (bit_buf
                        & (((1 as c_int) << 8 as c_int)
                            - 1 as c_int) as tinfl_bit_buf_t)
                        as mz_uint8;
                    bit_buf >>= 8 as c_int;
                    num_bits = num_bits.wrapping_sub(8 as mz_uint32);
                    current_block = 61;
                    continue;
                }
                71 => {
                    if counter < 4 as mz_uint32 {
                        if !(num_bits != 0) {
                            current_block = 7;
                            break;
                        }
                        if num_bits < 8 as c_int as mz_uint {
                            current_block = 62;
                            continue;
                        } else {
                            current_block = 63;
                            continue;
                        }
                    } else {
                        counter = ((*r).m_raw_header[0 as c_int as usize]
                            as c_int
                            | ((*r).m_raw_header[1 as c_int as usize]
                                as c_int)
                                << 8 as c_int)
                            as mz_uint32;
                        if counter
                            != (0xffff as c_int
                                ^ ((*r).m_raw_header[2 as c_int as usize]
                                    as c_int
                                    | ((*r).m_raw_header[3 as c_int as usize]
                                        as c_int)
                                        << 8 as c_int))
                                as mz_uint
                        {
                            current_block = 8;
                            break;
                        } else {
                            current_block = 58;
                            continue;
                        }
                    }
                }
                68 => {
                    s_1 = (bit_buf
                        & (((1 as c_int) << 8 as c_int)
                            - 1 as c_int) as tinfl_bit_buf_t)
                        as mz_uint;
                    bit_buf >>= 8 as c_int;
                    num_bits = num_bits.wrapping_sub(8 as mz_uint32);
                    current_block = 66;
                    continue;
                }
                66 => {
                    (*r).m_z_adler32 =
                        (*r).m_z_adler32 << 8 as c_int | s_1 as mz_uint32;
                    counter = counter.wrapping_add(1);
                    current_block = 72;
                }
                67 => {
                    c_12 = 0;
                    current_block = 31;
                    break;
                }
                37 => {
                    c_11 = 0;
                    current_block = 30;
                    break;
                }
                39 => {
                    let fresh31 = counter;
                    counter = counter.wrapping_sub(1);
                    if fresh31 != 0 {
                        current_block = 29;
                        break;
                    }
                    current_block = 78;
                }
                40 => {
                    c_10 = 0;
                    current_block = 27;
                    break;
                }
                44 => {
                    c_8 = 0;
                    current_block = 25;
                    break;
                }
                49 => {
                    c_6 = 0;
                    current_block = 19;
                    break;
                }
                53 => {
                    c_4 = 0;
                    current_block = 15;
                    break;
                }
                55 => {
                    c_3 = 0;
                    current_block = 14;
                    break;
                }
                59 => {
                    c_2 = 0;
                    current_block = 9;
                    break;
                }
                61 => {
                    counter = counter.wrapping_add(1);
                    current_block = 71;
                    continue;
                }
                _ => {
                    dist = (bit_buf
                        & (((1 as c_int) << 8 as c_int)
                            - 1 as c_int) as tinfl_bit_buf_t)
                        as mz_uint32;
                    bit_buf >>= 8 as c_int;
                    num_bits = num_bits.wrapping_sub(8 as mz_uint32);
                    current_block = 10;
                    break;
                }
            }
            match current_block {
                72 => {
                    if !(counter < 4 as mz_uint32) {
                        current_block = 33;
                        break;
                    }
                    s_1 = 0;
                    if !(num_bits != 0) {
                        current_block = 32;
                        break;
                    }
                    if num_bits < 8 as c_int as mz_uint {
                        current_block = 67;
                        continue;
                    } else {
                        current_block = 68;
                        continue;
                    }
                }
                73 => {
                    dist_from_out_buf_start =
                        pOut_buf_cur.offset_from(pOut_buf_start) as c_long as size_t;
                    if (dist == 0 as mz_uint32
                        || dist as size_t > dist_from_out_buf_start
                        || dist_from_out_buf_start == 0 as size_t)
                        && decomp_flags
                            & TINFL_FLAG_USING_NON_WRAPPING_OUTPUT_BUF as c_int
                                as mz_uint32
                            != 0
                    {
                        current_block = 28;
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
                        current_block = 39;
                        continue;
                    }
                    while counter > 2 as mz_uint32 {
                        *pOut_buf_cur.offset(0 as c_int as isize) =
                            *pSrc.offset(0 as c_int as isize);
                        *pOut_buf_cur.offset(1 as c_int as isize) =
                            *pSrc.offset(1 as c_int as isize);
                        *pOut_buf_cur.offset(2 as c_int as isize) =
                            *pSrc.offset(2 as c_int as isize);
                        pOut_buf_cur = pOut_buf_cur.offset(3 as c_int as isize);
                        pSrc = pSrc.offset(3 as c_int as isize);
                        counter = counter.wrapping_sub(3 as mz_uint32);
                    }
                    if counter > 0 as mz_uint32 {
                        *pOut_buf_cur.offset(0 as c_int as isize) =
                            *pSrc.offset(0 as c_int as isize);
                        if counter > 1 as mz_uint32 {
                            *pOut_buf_cur.offset(1 as c_int as isize) =
                                *pSrc.offset(1 as c_int as isize);
                        }
                        pOut_buf_cur = pOut_buf_cur.offset(counter as isize);
                    }
                    current_block = 78;
                }
                75 => {
                    counter &= 511 as mz_uint32;
                    if counter == 256 as mz_uint32 {
                        current_block = 80;
                    } else {
                        num_extra = s_length_extra[counter.wrapping_sub(257 as mz_uint32) as usize]
                            as mz_uint32;
                        counter = s_length_base[counter.wrapping_sub(257 as mz_uint32) as usize]
                            as mz_uint32;
                        if num_extra != 0 {
                            extra_bits = 0;
                            if num_bits < num_extra {
                                current_block = 44;
                                continue;
                            } else {
                                current_block = 45;
                                continue;
                            }
                        } else {
                            current_block = 74;
                        }
                    }
                }
                76 => {
                    if counter
                        < (*r).m_table_sizes[0 as c_int as usize]
                            .wrapping_add((*r).m_table_sizes[1 as c_int as usize])
                    {
                        s_0 = 0;
                        temp = 0;
                        code_len = 0;
                        c_5 = 0;
                        if !(num_bits < 15 as mz_uint32) {
                            current_block = 52;
                            continue;
                        }
                        if (pIn_buf_end.offset_from(pIn_buf_cur) as c_long)
                            < 2 as c_long
                        {
                            current_block = 51;
                            continue;
                        }
                        bit_buf |= (*pIn_buf_cur.offset(0 as c_int as isize)
                            as tinfl_bit_buf_t)
                            << num_bits
                            | (*pIn_buf_cur.offset(1 as c_int as isize)
                                as tinfl_bit_buf_t)
                                << num_bits.wrapping_add(8 as mz_uint32);
                        pIn_buf_cur = pIn_buf_cur.offset(2 as c_int as isize);
                        num_bits = num_bits.wrapping_add(16 as mz_uint32);
                        current_block = 52;
                        continue;
                    } else {
                        if (*r).m_table_sizes[0 as c_int as usize]
                            .wrapping_add((*r).m_table_sizes[1 as c_int as usize])
                            != counter
                        {
                            current_block = 20;
                            break;
                        }
                        memcpy(
                            &raw mut (*r).m_code_size_0 as *mut mz_uint8
                                as *mut c_void,
                            &raw mut (*r).m_len_codes as *mut mz_uint8
                                as *const c_void,
                            (*r).m_table_sizes[0 as c_int as usize] as size_t,
                        );
                        memcpy(
                            &raw mut (*r).m_code_size_1 as *mut mz_uint8
                                as *mut c_void,
                            (&raw mut (*r).m_len_codes as *mut mz_uint8).offset(
                                (*r).m_table_sizes[0 as c_int as usize] as isize,
                            ) as *const c_void,
                            (*r).m_table_sizes[1 as c_int as usize] as size_t,
                        );
                    }
                    current_block = 77;
                }
                79 => {
                    if counter < (*r).m_table_sizes[2 as c_int as usize] {
                        s = 0;
                        if num_bits < 3 as c_int as mz_uint {
                            current_block = 53;
                            continue;
                        } else {
                            current_block = 54;
                            continue;
                        }
                    } else {
                        (*r).m_table_sizes[2 as c_int as usize] = 19 as mz_uint32;
                        current_block = 69;
                        continue;
                    }
                }
                _ => {}
            }
            match current_block {
                77 => {
                    (*r).m_type = (*r).m_type.wrapping_sub(1);
                    current_block = 69;
                }
                74 => {
                    temp_1 = 0;
                    code_len_2 = 0;
                    c_9 = 0;
                    if !(num_bits < 15 as mz_uint32) {
                        current_block = 43;
                        continue;
                    }
                    if (pIn_buf_end.offset_from(pIn_buf_cur) as c_long)
                        < 2 as c_long
                    {
                        current_block = 42;
                        continue;
                    }
                    bit_buf |= (*pIn_buf_cur.offset(0 as c_int as isize)
                        as tinfl_bit_buf_t)
                        << num_bits
                        | (*pIn_buf_cur.offset(1 as c_int as isize)
                            as tinfl_bit_buf_t)
                            << num_bits.wrapping_add(8 as mz_uint32);
                    pIn_buf_cur = pIn_buf_cur.offset(2 as c_int as isize);
                    num_bits = num_bits.wrapping_add(16 as mz_uint32);
                    current_block = 43;
                }
                78 => {
                    pSrc = ::core::ptr::null_mut::<mz_uint8>();
                    current_block = 46;
                }
                _ => {
                    if (*r).m_final & 1 as mz_uint32 == 0 {
                        current_block = 1;
                        break;
                    }
                    if num_bits < num_bits & 7 as mz_uint32 {
                        current_block = 37;
                    } else {
                        current_block = 38;
                    }
                }
            }
        }
    }
    (*r).m_num_bits = num_bits;
    (*r).m_bit_buf = bit_buf & !(!(0 as c_int as tinfl_bit_buf_t) << num_bits);
    (*r).m_dist = dist;
    (*r).m_counter = counter;
    (*r).m_num_extra = num_extra;
    (*r).m_dist_from_out_buf_start = dist_from_out_buf_start;
    *pIn_buf_size_view = pIn_buf_cur.offset_from(pIn_buf_next) as c_long as size_t;
    *pOut_buf_size = pOut_buf_cur.offset_from(pOut_buf_next) as c_long as size_t;
    if decomp_flags
        & (TINFL_FLAG_PARSE_ZLIB_HEADER as c_int
            | TINFL_FLAG_COMPUTE_ADLER32 as c_int) as mz_uint32
        != 0
        && status as c_int >= 0 as c_int
    {
        let mut ptr: *const mz_uint8 = pOut_buf_next;
        let mut buf_len: size_t = *pOut_buf_size;
        let mut i_1: mz_uint32 = 0;
        let mut s1: mz_uint32 = (*r).m_check_adler32 & 0xffff as mz_uint32;
        let mut s2: mz_uint32 = (*r).m_check_adler32 >> 16 as c_int;
        let mut block_len: size_t = buf_len.wrapping_rem(5552 as size_t);
        while buf_len != 0 {
            i_1 = 0 as mz_uint32;
            while (i_1.wrapping_add(7 as mz_uint32) as size_t) < block_len {
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
                i_1 = i_1.wrapping_add(8 as mz_uint32);
                ptr = ptr.offset(8 as c_int as isize);
            }
            while (i_1 as size_t) < block_len {
                let fresh37 = ptr;
                ptr = ptr.offset(1);
                s1 = s1.wrapping_add(*fresh37 as mz_uint32);
                s2 = s2.wrapping_add(s1);
                i_1 = i_1.wrapping_add(1);
            }
            s1 = (s1 as c_uint).wrapping_rem(65521 as c_uint) as mz_uint32
                as mz_uint32;
            s2 = (s2 as c_uint).wrapping_rem(65521 as c_uint) as mz_uint32
                as mz_uint32;
            buf_len = buf_len.wrapping_sub(block_len);
            block_len = 5552 as size_t;
        }
        (*r).m_check_adler32 = (s2 << 16 as c_int).wrapping_add(s1);
        if status as c_int == TINFL_STATUS_DONE as c_int
            && decomp_flags & TINFL_FLAG_PARSE_ZLIB_HEADER as c_int as mz_uint32 != 0
            && (*r).m_check_adler32 != (*r).m_z_adler32
        {
            status = TINFL_STATUS_ADLER32_MISMATCH;
        }
    }
    return status;
}
#[inline]
pub unsafe fn tinfl_decompress_mem_to_heap(
    mut pSrc_buf: *const c_void,
    mut src_buf_len: size_t,
    mut pOut_len: *mut size_t,
    mut flags: c_int,
) -> *mut c_void {
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
    let mut pBuf: *mut c_void = NULL;
    let mut pNew_buf: *mut c_void = ::core::ptr::null_mut::<c_void>();
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
            (flags & !(TINFL_FLAG_HAS_MORE_INPUT as c_int)
                | TINFL_FLAG_USING_NON_WRAPPING_OUTPUT_BUF as c_int)
                as mz_uint32,
        );
        if (status as c_int) < 0 as c_int
            || status as c_int == TINFL_STATUS_NEEDS_MORE_INPUT as c_int
        {
            free(pBuf);
            *pOut_len = 0 as size_t;
            return NULL;
        }
        src_buf_ofs = src_buf_ofs.wrapping_add(src_buf_size);
        *pOut_len = (*pOut_len).wrapping_add(dst_buf_size);
        if status as c_int == TINFL_STATUS_DONE as c_int {
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
#[inline]
pub unsafe fn tinfl_decompress_mem_to_mem(
    mut pOut_buf: *mut c_void,
    mut out_buf_len: size_t,
    mut pSrc_buf: *const c_void,
    mut src_buf_len: size_t,
    mut flags: c_int,
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
        (flags & !(TINFL_FLAG_HAS_MORE_INPUT as c_int)
            | TINFL_FLAG_USING_NON_WRAPPING_OUTPUT_BUF as c_int) as mz_uint32,
    );
    return if status as c_int != TINFL_STATUS_DONE as c_int {
        TINFL_DECOMPRESS_MEM_TO_MEM_FAILED
    } else {
        out_buf_len
    };
}
#[inline]
pub unsafe fn tinfl_decompress_mem_to_callback(
    mut pIn_buf: *const c_void,
    mut pIn_buf_size: *mut size_t,
    mut pPut_buf_func: tinfl_put_buf_func_ptr,
    mut pPut_buf_user: *mut c_void,
    mut flags: c_int,
) -> c_int {
    let mut result: c_int = 0 as c_int;
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
        return TINFL_STATUS_FAILED as c_int;
    }
    memset(
        pDict as *mut c_void,
        0 as c_int,
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
                & !(TINFL_FLAG_HAS_MORE_INPUT as c_int
                    | TINFL_FLAG_USING_NON_WRAPPING_OUTPUT_BUF as c_int))
                as mz_uint32,
        );
        in_buf_ofs = in_buf_ofs.wrapping_add(in_buf_size);
        if dst_buf_size != 0
            && Some(pPut_buf_func.expect("non-null function pointer"))
                .expect("non-null function pointer")(
                pDict.offset(dict_ofs as isize) as *const c_void,
                dst_buf_size as c_int,
                pPut_buf_user,
            ) == 0
        {
            break;
        }
        if status as c_int != TINFL_STATUS_HAS_MORE_OUTPUT as c_int {
            result = (status as c_int == TINFL_STATUS_DONE as c_int)
                as c_int;
            break;
        } else {
            dict_ofs = dict_ofs.wrapping_add(dst_buf_size)
                & (TINFL_LZ_DICT_SIZE - 1 as c_int) as size_t;
        }
    }
    free(pDict as *mut c_void);
    *pIn_buf_size = in_buf_ofs;
    return result;
}
#[inline]
pub fn tinfl_decompressor_alloc() -> *mut tinfl_decompressor { unsafe {
    let mut pDecomp: *mut tinfl_decompressor =
        malloc(::core::mem::size_of::<tinfl_decompressor>() as size_t) as *mut tinfl_decompressor;
    if !pDecomp.is_null() {
        (*pDecomp).m_state = 0 as mz_uint32;
    }
    return pDecomp;
} }
#[inline]
pub unsafe fn tinfl_decompressor_free(mut pDecomp: *mut tinfl_decompressor) {
    free(pDecomp as *mut c_void);
}
