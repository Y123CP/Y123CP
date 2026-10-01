use core::ffi::*;
use crate::src::c_inlined_fns::BROTLI_UNALIGNED_LOAD_PTR;
use crate::src::enc::memory::BrotliAllocate;
use crate::src::enc::literal_cost::BrotliEstimateBitCostsForLiterals;
use crate::src::enc::static_dict::BrotliFindAllStaticDictionaryMatches;
use crate::src::enc::memory::BrotliFree;
use crate::src::c_inlined_fns::BrotliUnalignedRead32;
use crate::src::c_inlined_fns::BrotliUnalignedRead64;
use crate::src::c_inlined_fns::CombineLengthCodes;
use crate::src::c_inlined_fns::FastLog2;
use crate::src::c_inlined_fns::GetCopyExtra;
use crate::src::c_inlined_fns::GetInsertExtra;
use crate::src::c_inlined_fns::HashTypeLengthH10;
use crate::src::c_inlined_fns::InitBackwardMatch;
use crate::src::c_inlined_fns::LeftChildIndexH10;
use crate::src::c_inlined_fns::Log2FloorNonZero;
use crate::src::c_inlined_fns::RightChildIndexH10;
use crate::src::c_inlined_fns::brotli_max_size_t;
use crate::src::c_inlined_fns::brotli_min_size_t;
pub use crate::src::c_inlined_fns::*;
pub use crate::src::ffi::*;
pub use crate::src::c_consts::*;
pub use crate::src::c_structs::*;
pub use crate::src::c_types::*;

pub use crate::src::enc::backward_references::BrotliEncoderDictionary;

pub use crate::src::enc::backward_references::ContextualEncoderDictionary;

pub use crate::src::enc::backward_references::SharedEncoderDictionary;


pub use crate::src::enc::backward_references::BrotliEncoderParams;

pub use crate::src::enc::backward_references::Command;


pub use crate::src::enc::backward_references::H35;

pub use crate::src::enc::backward_references::H55;

pub use crate::src::enc::backward_references::H65;

pub use crate::src::enc::backward_references::Hasher;

pub use crate::src::enc::backward_references::C2RustUnnamed_hufc2eb4bd;

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
    pub cost: c_float,
    pub next: uint32_t,
    pub shortcut: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZopfliCostModel {
    pub cost_cmd_: [c_float; 704],
    pub cost_dist_: *mut c_float,
    pub distance_histogram_size: uint32_t,
    pub literal_costs_: *mut c_float,
    pub min_cost_cmd_: c_float,
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
    pub cost_literal: [c_float; 256],
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
    pub distance_cache: [c_int; 4],
    pub costdiff: c_float,
    pub cost: c_float,
}

#[inline(always)]
fn brotli_min_float(
    mut a: c_float,
    mut b: c_float,
) -> c_float { {
    return if a < b { a } else { b };
} }

static mut kPreparedDictionaryMagic: uint32_t = 0xdebcede0 as uint32_t;
static mut kPreparedDictionaryHashMul64Long: uint64_t =
    (0x1fe35a7b as c_uint as uint64_t) << 32 as c_int
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
        let mut dist: size_t = ((1 as c_int as size_t)
            << postfix_bits.wrapping_add(2 as size_t))
        .wrapping_add(
            distance_code
                .wrapping_sub(BROTLI_NUM_DISTANCE_SHORT_CODES as size_t)
                .wrapping_sub(num_direct_codes),
        );
        let mut bucket: size_t = Log2FloorNonZero(dist).wrapping_sub(1 as uint32_t) as size_t;
        let mut postfix_mask: size_t = ((1 as c_uint) << postfix_bits)
            .wrapping_sub(1 as c_uint)
            as size_t;
        let mut postfix: size_t = dist & postfix_mask;
        let mut prefix: size_t = dist >> bucket & 1 as size_t;
        let mut offset: size_t = (2 as size_t).wrapping_add(prefix) << bucket;
        let mut nbits: size_t = bucket.wrapping_sub(postfix_bits);
        *code = (nbits << 10 as c_int
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
extern "C" fn GetInsertLengthCode(mut insertlen: size_t) -> uint16_t { {
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
} }
#[inline(always)]
extern "C" fn GetCopyLengthCode(mut copylen: size_t) -> uint16_t { {
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
} }

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
unsafe extern "C" fn InitCommand(
    mut self_0: *mut Command,
    mut dist: *const BrotliDistanceParams,
    mut insertlen: size_t,
    mut copylen: size_t,
    mut copylen_code_delta: c_int,
    mut distance_code: size_t,
) {
    let mut delta: uint32_t = copylen_code_delta as int8_t as uint8_t as uint32_t;
    (*self_0).insert_len_ = insertlen as uint32_t;
    (*self_0).copy_len_ = (copylen | (delta << 25 as c_int) as size_t) as uint32_t;
    PrefixEncodeCopyDistance(
        distance_code,
        (*dist).num_direct_distance_codes as size_t,
        (*dist).distance_postfix_bits as size_t,
        &raw mut (*self_0).dist_prefix_,
        &raw mut (*self_0).dist_extra_,
    );
    GetLengthCode(
        insertlen,
        (copylen as c_int + copylen_code_delta) as size_t,
        if (*self_0).dist_prefix_ as c_int & 0x3ff as c_int
            == 0 as c_int
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
    let self_0_view: &Command = unsafe { &*self_0 };
    return self_0_view.copy_len_ & 0x1ffffff as uint32_t;
}
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

pub const MAX_ZOPFLI_LEN_QUALITY_10: c_int = 150 as c_int;
pub const MAX_ZOPFLI_LEN_QUALITY_11: c_int = 325 as c_int;
pub const BROTLI_LONG_COPY_QUICK_STEP: c_int = 16384 as c_int;
#[inline(always)]
unsafe fn MaxZopfliLen(mut params: *const BrotliEncoderParams) -> size_t {
    let params_view: &BrotliEncoderParams = unsafe { &*params };
    return (if params_view.quality <= 10 as c_int {
        MAX_ZOPFLI_LEN_QUALITY_10
    } else {
        MAX_ZOPFLI_LEN_QUALITY_11
    }) as size_t;
}
#[inline(always)]
unsafe fn MaxZopfliCandidates(mut params: *const BrotliEncoderParams) -> size_t {
    let params_view: &BrotliEncoderParams = unsafe { &*params };
    return (if params_view.quality <= 10 as c_int {
        1 as c_int
    } else {
        5 as c_int
    }) as size_t;
}

static mut kInvalidMatch: uint32_t = 0xfffffff as uint32_t;

#[inline(always)]
unsafe fn InitDictionaryBackwardMatch(
    mut self_0: *mut BackwardMatch,
    mut dist: size_t,
    mut len: size_t,
    mut len_code: size_t,
) {
    (*self_0).distance = dist as uint32_t;
    (*self_0).length_and_code = (len << 5 as c_int
        | (if len == len_code {
            0 as size_t
        } else {
            len_code
        })) as uint32_t;
}
#[inline(always)]
unsafe fn BackwardMatchLength(mut self_0: *const BackwardMatch) -> size_t {
    let self_0_view: &BackwardMatch = unsafe { &*self_0 };
    return (self_0_view.length_and_code >> 5 as c_int) as size_t;
}
#[inline(always)]
unsafe fn BackwardMatchLengthCode(mut self_0: *const BackwardMatch) -> size_t {
    let mut code: size_t = ((*self_0).length_and_code & 31 as uint32_t) as size_t;
    return if code != 0 {
        code
    } else {
        BackwardMatchLength(self_0)
    };
}
pub const BUCKET_BITS: c_int = 17 as c_int;

pub const MAX_NUM_MATCHES_H10: c_int = 128 as c_int;
#[inline(always)]
unsafe fn FindAllCompoundDictionaryMatches(
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
        !(0 as c_uint as uint32_t) >> (32 as uint32_t).wrapping_sub(slot_bits);
    let hash_mask: uint64_t =
        !(0 as c_uint as uint64_t) >> (64 as uint32_t).wrapping_sub(hash_bits);
    let mut slot_offsets: *const uint32_t = self_0.offset(1 as c_int as isize)
        as *const PreparedDictionary as *mut uint32_t;
    let mut heads: *const uint16_t = slot_offsets
        .offset(((1 as c_uint as size_t) << slot_bits) as isize)
        as *const uint32_t as *mut uint16_t;
    let mut items: *const uint32_t = heads
        .offset(((1 as c_uint as size_t) << bucket_bits) as isize)
        as *const uint16_t as *mut uint32_t;
    let mut source: *const uint8_t = ::core::ptr::null::<uint8_t>();
    let cur_ix_masked: size_t = cur_ix & ring_buffer_mask;
    let mut best_len: size_t = min_length;
    let h: uint64_t = (BrotliUnalignedRead64(
        data.offset(cur_ix_masked as isize) as *const uint8_t as *const c_void
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
        1 as c_int
    } else {
        0 as c_int
    }) as uint32_t;
    let mut found: size_t = 0 as size_t;
    let mut tail: *const c_void =
        items.offset((*self_0).num_items as isize) as *const uint32_t as *mut c_void;
    if (*self_0).magic == kPreparedDictionaryMagic {
        source = tail as *const uint8_t;
    } else {
        source =
            BROTLI_UNALIGNED_LOAD_PTR(tail as *mut *const uint8_t as *const c_void)
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
        item = (item as c_uint & 0x80000000 as c_uint) as uint32_t;
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
            || *data.offset(cur_ix_masked.wrapping_add(best_len) as isize) as c_int
                != *source.offset(offset.wrapping_add(best_len) as isize) as c_int
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
unsafe fn LookupAllCompoundDictionaryMatches(
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
        total_found = (total_found as c_ulong).wrapping_add(
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
            ) as c_ulong,
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
fn StoreLookaheadH10() -> size_t { {
    return MAX_TREE_COMP_LENGTH as size_t;
} }
unsafe extern "C" fn HashBytesH10(mut data: *const uint8_t) -> uint32_t {
    let mut h: uint32_t =
        BrotliUnalignedRead32(data as *const c_void).wrapping_mul(kHashMul32);
    return h >> 32 as c_int - BUCKET_BITS;
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
unsafe fn FindAllMatchesH10(
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
        16 as c_int
    } else {
        64 as c_int
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
        if (backward > max_backward) as c_int as c_long != 0 {
            break;
        }
        prev_ix =
            (prev_ix as c_ulong & ring_buffer_mask as c_ulong) as size_t;
        if !(*data.offset(cur_ix_masked as isize) as c_int
            != *data.offset(prev_ix as isize) as c_int
            || *data.offset(cur_ix_masked.wrapping_add(1 as size_t) as isize) as c_int
                != *data.offset(prev_ix.wrapping_add(1 as size_t) as isize) as c_int)
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
        (&raw mut dict_matches as *mut uint32_t).offset(0 as c_int as isize)
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
                    .wrapping_add((dict_id >> 5 as c_int) as size_t)
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
    return matches.offset_from(orig_matches) as c_long as size_t;
}
#[inline(always)]
unsafe fn StoreH10(
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
unsafe fn StoreRangeH10(
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
            j = (j as c_ulong).wrapping_add(8 as c_ulong) as size_t
                as size_t;
        }
    }
    while i < ix_end {
        StoreH10(self_0, data, mask, i);
        i = i.wrapping_add(1);
    }
}
static mut kInfinity: c_float = 1.7e38f32;
static mut kDistanceCacheIndex: [uint32_t; 16] = [
    0 as c_int as uint32_t,
    1 as c_int as uint32_t,
    2 as c_int as uint32_t,
    3 as c_int as uint32_t,
    0 as c_int as uint32_t,
    0 as c_int as uint32_t,
    0 as c_int as uint32_t,
    0 as c_int as uint32_t,
    0 as c_int as uint32_t,
    0 as c_int as uint32_t,
    1 as c_int as uint32_t,
    1 as c_int as uint32_t,
    1 as c_int as uint32_t,
    1 as c_int as uint32_t,
    1 as c_int as uint32_t,
    1 as c_int as uint32_t,
];
static mut kDistanceCacheOffset: [c_int; 16] = [
    0 as c_int,
    0 as c_int,
    0 as c_int,
    0 as c_int,
    -(1 as c_int),
    1 as c_int,
    -(2 as c_int),
    2 as c_int,
    -(3 as c_int),
    3 as c_int,
    -(1 as c_int),
    1 as c_int,
    -(2 as c_int),
    2 as c_int,
    -(3 as c_int),
    3 as c_int,
];
#[inline]
pub unsafe fn BrotliInitZopfliNodes(mut array: *mut ZopfliNode, mut length: size_t) {
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
unsafe fn ZopfliNodeCopyLength(mut self_0: *const ZopfliNode) -> uint32_t {
    let self_0_view: &ZopfliNode = unsafe { &*self_0 };
    return self_0_view.length & 0x1ffffff as uint32_t;
}
#[inline(always)]
unsafe fn ZopfliNodeLengthCode(mut self_0: *const ZopfliNode) -> uint32_t {
    let modifier: uint32_t = (*self_0).length >> 25 as c_int;
    return ZopfliNodeCopyLength(self_0)
        .wrapping_add(9 as uint32_t)
        .wrapping_sub(modifier);
}
#[inline(always)]
unsafe fn ZopfliNodeCopyDistance(mut self_0: *const ZopfliNode) -> uint32_t {
    let self_0_view: &ZopfliNode = unsafe { &*self_0 };
    return self_0_view.distance;
}
#[inline(always)]
unsafe fn ZopfliNodeDistanceCode(mut self_0: *const ZopfliNode) -> uint32_t {
    let short_code: uint32_t = (*self_0).dcode_insert_length >> 27 as c_int;
    return if short_code == 0 as uint32_t {
        ZopfliNodeCopyDistance(self_0)
            .wrapping_add(BROTLI_NUM_DISTANCE_SHORT_CODES as uint32_t)
            .wrapping_sub(1 as uint32_t)
    } else {
        short_code.wrapping_sub(1 as uint32_t)
    };
}
#[inline(always)]
unsafe fn ZopfliNodeCommandLength(mut self_0: *const ZopfliNode) -> uint32_t {
    return ZopfliNodeCopyLength(self_0)
        .wrapping_add((*self_0).dcode_insert_length & 0x7ffffff as uint32_t);
}
unsafe fn InitZopfliCostModel(
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
                .wrapping_mul(::core::mem::size_of::<c_float>() as size_t),
        ) as *mut c_float
    } else {
        ::core::ptr::null_mut::<c_float>()
    };
    (*self_0).cost_dist_ = if (*dist).alphabet_size_limit > 0 as uint32_t {
        BrotliAllocate(
            m,
            ((*dist).alphabet_size_limit as size_t)
                .wrapping_mul(::core::mem::size_of::<c_float>() as size_t),
        ) as *mut c_float
    } else {
        ::core::ptr::null_mut::<c_float>()
    };
    (*self_0).distance_histogram_size = (*dist).alphabet_size_limit;
    if 0 as c_int != 0 {
        return;
    }
}
unsafe fn CleanupZopfliCostModel(
    mut m: *mut MemoryManager,
    mut self_0: *mut ZopfliCostModel,
) {
    BrotliFree(m, (*self_0).literal_costs_ as *mut c_void);
    (*self_0).literal_costs_ = ::core::ptr::null_mut::<c_float>();
    BrotliFree(m, (*self_0).cost_dist_ as *mut c_void);
    (*self_0).cost_dist_ = ::core::ptr::null_mut::<c_float>();
}
unsafe fn SetCost(
    mut histogram: *const uint32_t,
    mut histogram_size: size_t,
    mut literal_histogram: c_int,
    mut cost: *mut c_float,
) {
    let mut sum: size_t = 0 as size_t;
    let mut missing_symbol_sum: size_t = 0;
    let mut log2sum: c_float = 0.;
    let mut missing_symbol_cost: c_float = 0.;
    let mut i: size_t = 0;
    i = 0 as size_t;
    while i < histogram_size {
        sum = (sum as c_ulong)
            .wrapping_add(*histogram.offset(i as isize) as c_ulong)
            as size_t as size_t;
        i = i.wrapping_add(1);
    }
    log2sum = FastLog2(sum) as c_float;
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
    missing_symbol_cost = FastLog2(missing_symbol_sum) as c_float
        + 2 as c_int as c_float;
    i = 0 as size_t;
    while i < histogram_size {
        if *histogram.offset(i as isize) == 0 as uint32_t {
            *cost.offset(i as isize) = missing_symbol_cost;
        } else {
            *cost.offset(i as isize) =
                log2sum - FastLog2(*histogram.offset(i as isize) as size_t) as c_float;
            if *cost.offset(i as isize) < 1 as c_int as c_float {
                *cost.offset(i as isize) = 1 as c_int as c_float;
            }
        }
        i = i.wrapping_add(1);
    }
}
unsafe fn ZopfliCostModelSetFromCommands(
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
    let mut min_cost_cmd: c_float = kInfinity;
    let mut i: size_t = 0;
    let mut cost_cmd: *mut c_float =
        &raw mut (*self_0).cost_cmd_ as *mut c_float;
    memset(
        &raw mut (*arena).histogram_literal as *mut uint32_t as *mut c_void,
        0 as c_int,
        ::core::mem::size_of::<[uint32_t; 256]>() as size_t,
    );
    memset(
        &raw mut (*arena).histogram_cmd as *mut uint32_t as *mut c_void,
        0 as c_int,
        ::core::mem::size_of::<[uint32_t; 704]>() as size_t,
    );
    memset(
        &raw mut (*arena).histogram_dist as *mut uint32_t as *mut c_void,
        0 as c_int,
        ::core::mem::size_of::<[uint32_t; 544]>() as size_t,
    );
    i = 0 as size_t;
    while i < num_commands {
        let mut inslength: size_t = (*commands.offset(i as isize)).insert_len_ as size_t;
        let mut copylength: size_t =
            CommandCopyLen(commands.offset(i as isize) as *const Command) as size_t;
        let mut distcode: size_t = ((*commands.offset(i as isize)).dist_prefix_
            as c_int
            & 0x3ff as c_int) as size_t;
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
        pos = (pos as c_ulong)
            .wrapping_add(inslength.wrapping_add(copylength) as c_ulong)
            as size_t as size_t;
        i = i.wrapping_add(1);
    }
    SetCost(
        &raw mut (*arena).histogram_literal as *mut uint32_t,
        BROTLI_NUM_LITERAL_SYMBOLS as size_t,
        BROTLI_TRUE,
        &raw mut (*arena).cost_literal as *mut c_float,
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
    let mut literal_costs: *mut c_float = (*self_0).literal_costs_;
    let mut literal_carry: c_float = 0.0f32;
    let mut num_bytes: size_t = (*self_0).num_bytes_;
    *literal_costs.offset(0 as c_int as isize) = 0.0f32;
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
unsafe fn ZopfliCostModelSetFromLiteralCosts(
    mut self_0: *mut ZopfliCostModel,
    mut position: size_t,
    mut ringbuffer: *const uint8_t,
    mut ringbuffer_mask: size_t,
) {
    let mut literal_costs: *mut c_float = (*self_0).literal_costs_;
    let mut literal_carry: c_float = 0.0f32;
    let mut cost_dist: *mut c_float = (*self_0).cost_dist_;
    let mut cost_cmd: *mut c_float =
        &raw mut (*self_0).cost_cmd_ as *mut c_float;
    let mut num_bytes: size_t = (*self_0).num_bytes_;
    let mut i: size_t = 0;
    BrotliEstimateBitCostsForLiterals(
        position,
        num_bytes,
        ringbuffer_mask,
        ringbuffer,
        &raw mut (*self_0).c2rust_unnamed.literal_histograms as *mut size_t,
        literal_costs.offset(1 as c_int as isize) as *mut c_float,
    );
    *literal_costs.offset(0 as c_int as isize) = 0.0f32;
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
                as c_float;
        i = i.wrapping_add(1);
    }
    i = 0 as size_t;
    while i < (*self_0).distance_histogram_size as size_t {
        *cost_dist.offset(i as isize) =
            FastLog2((20 as uint32_t).wrapping_add(i as uint32_t) as size_t)
                as c_float;
        i = i.wrapping_add(1);
    }
    (*self_0).min_cost_cmd_ = FastLog2(11 as size_t) as c_float;
}
#[inline(always)]
unsafe fn ZopfliCostModelGetCommandCost(
    mut self_0: *const ZopfliCostModel,
    mut cmdcode: uint16_t,
) -> c_float {
    let self_0_view: &ZopfliCostModel = unsafe { &*self_0 };
    return self_0_view.cost_cmd_[cmdcode as usize];
}
#[inline(always)]
unsafe fn ZopfliCostModelGetDistanceCost(
    mut self_0: *const ZopfliCostModel,
    mut distcode: size_t,
) -> c_float {
    let self_0_view: &ZopfliCostModel = unsafe { &*self_0 };
    return *self_0_view.cost_dist_.offset(distcode as isize);
}
#[inline(always)]
unsafe fn ZopfliCostModelGetLiteralCosts(
    mut self_0: *const ZopfliCostModel,
    mut from: size_t,
    mut to: size_t,
) -> c_float {
    let self_0_view: &ZopfliCostModel = unsafe { &*self_0 };
    return *self_0_view.literal_costs_.offset(to as isize)
        - *self_0_view.literal_costs_.offset(from as isize);
}
#[inline(always)]
unsafe fn ZopfliCostModelGetMinCostCmd(
    mut self_0: *const ZopfliCostModel,
) -> c_float {
    let self_0_view: &ZopfliCostModel = unsafe { &*self_0 };
    return self_0_view.min_cost_cmd_;
}
#[inline(always)]
unsafe fn UpdateZopfliNode(
    mut nodes: *mut ZopfliNode,
    mut pos: size_t,
    mut start_pos: size_t,
    mut len: size_t,
    mut len_code: size_t,
    mut dist: size_t,
    mut short_code: size_t,
    mut cost: c_float,
) {
    let mut next: *mut ZopfliNode = nodes.offset(pos.wrapping_add(len) as isize) as *mut ZopfliNode;
    (*next).length = (len
        | len.wrapping_add(9 as size_t).wrapping_sub(len_code) << 25 as c_int)
        as uint32_t;
    (*next).distance = dist as uint32_t;
    (*next).dcode_insert_length =
        (short_code << 27 as c_int | pos.wrapping_sub(start_pos)) as uint32_t;
    (*next).u.cost = cost;
}
#[inline(always)]
unsafe fn InitStartPosQueue(mut self_0: *mut StartPosQueue) {
    (*self_0).idx_ = 0 as size_t;
}
unsafe fn StartPosQueueSize(mut self_0: *const StartPosQueue) -> size_t {
    return brotli_min_size_t((*self_0).idx_, 8 as size_t);
}
unsafe fn StartPosQueuePush(
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
unsafe fn StartPosQueueAt(
    mut self_0: *const StartPosQueue,
    mut k: size_t,
) -> *const PosData {
    let self_0_view: &StartPosQueue = unsafe { &*self_0 };
    return (&raw const self_0_view.q_ as *const PosData)
        .offset((k.wrapping_sub(self_0_view.idx_) & 7 as size_t) as isize)
        as *const PosData;
}
unsafe fn ComputeMinimumCopyLength(
    start_cost: c_float,
    mut nodes: *const ZopfliNode,
    num_bytes: size_t,
    pos: size_t,
) -> size_t {
    let mut min_cost: c_float = start_cost;
    let mut len: size_t = 2 as size_t;
    let mut next_len_bucket: size_t = 4 as size_t;
    let mut next_len_offset: size_t = 10 as size_t;
    while pos.wrapping_add(len) <= num_bytes
        && (*nodes.offset(pos.wrapping_add(len) as isize)).u.cost <= min_cost
    {
        len = len.wrapping_add(1);
        if len == next_len_offset {
            min_cost += 1.0f32;
            next_len_offset = (next_len_offset as c_ulong)
                .wrapping_add(next_len_bucket as c_ulong)
                as size_t as size_t;
            next_len_bucket = (next_len_bucket as c_ulong)
                .wrapping_mul(2 as c_ulong) as size_t
                as size_t;
        }
    }
    return len;
}
unsafe fn ComputeDistanceShortcut(
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
unsafe fn ComputeDistanceCache(
    pos: size_t,
    mut starting_dist_cache: *const c_int,
    mut nodes: *const ZopfliNode,
    mut dist_cache: *mut c_int,
) {
    let mut idx: c_int = 0 as c_int;
    let mut p: size_t = (*nodes.offset(pos as isize)).u.shortcut as size_t;
    while idx < 4 as c_int && p > 0 as size_t {
        let i_len: size_t =
            ((*nodes.offset(p as isize)).dcode_insert_length & 0x7ffffff as uint32_t) as size_t;
        let c_len: size_t =
            ZopfliNodeCopyLength(nodes.offset(p as isize) as *const ZopfliNode) as size_t;
        let dist: size_t =
            ZopfliNodeCopyDistance(nodes.offset(p as isize) as *const ZopfliNode) as size_t;
        let fresh5 = idx;
        idx = idx + 1;
        *dist_cache.offset(fresh5 as isize) = dist as c_int;
        p = (*nodes.offset(p.wrapping_sub(c_len).wrapping_sub(i_len) as isize))
            .u
            .shortcut as size_t;
    }
    while idx < 4 as c_int {
        let fresh6 = starting_dist_cache;
        starting_dist_cache = starting_dist_cache.offset(1);
        *dist_cache.offset(idx as isize) = *fresh6;
        idx += 1;
    }
}
unsafe fn EvaluateNode(
    block_start: size_t,
    pos: size_t,
    max_backward_limit: size_t,
    gap: size_t,
    mut starting_dist_cache: *const c_int,
    mut model: *const ZopfliCostModel,
    mut queue: *mut StartPosQueue,
    mut nodes: *mut ZopfliNode,
) {
    let mut node_cost: c_float = (*nodes.offset(pos as isize)).u.cost;
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
            &raw mut posdata.distance_cache as *mut c_int,
        );
        StartPosQueuePush(queue, &raw mut posdata);
    }
}
unsafe fn UpdateNodes(
    num_bytes: size_t,
    block_start: size_t,
    pos: size_t,
    mut ringbuffer: *const uint8_t,
    ringbuffer_mask: size_t,
    mut params: *const BrotliEncoderParams,
    max_backward_limit: size_t,
    mut starting_dist_cache: *const c_int,
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
    let mut min_cost: c_float = (*posdata).cost
        + ZopfliCostModelGetMinCostCmd(model)
        + ZopfliCostModelGetLiteralCosts(model, (*posdata).pos, pos);
    min_len = ComputeMinimumCopyLength(min_cost, nodes, num_bytes, pos);
    k = 0 as size_t;
    while k < max_iters && k < StartPosQueueSize(queue) {
        let mut posdata_0: *const PosData = StartPosQueueAt(queue, k);
        let start: size_t = (*posdata_0).pos;
        let inscode: uint16_t = GetInsertLengthCode(pos.wrapping_sub(start)) as uint16_t;
        let start_costdiff: c_float = (*posdata_0).costdiff;
        let base_cost: c_float = start_costdiff
            + GetInsertExtra(inscode) as c_float
            + ZopfliCostModelGetLiteralCosts(model, 0 as size_t, pos) as c_float;
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
            if !((backward > dictionary_start.wrapping_add(gap)) as c_int
                as c_long
                != 0)
            {
                if backward <= max_distance {
                    if prev_ix >= cur_ix {
                        current_block_23 = 4166486009154926805;
                    } else {
                        prev_ix = (prev_ix as c_ulong
                            & ringbuffer_mask as c_ulong)
                            as size_t;
                        if prev_ix.wrapping_add(best_len) > ringbuffer_mask
                            || continuation as c_int
                                != *ringbuffer.offset(prev_ix.wrapping_add(best_len) as isize)
                                    as c_int
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
                        || continuation as c_int
                            != *source.offset(offset.wrapping_add(best_len) as isize)
                                as c_int
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
                        let dist_cost: c_float = base_cost
                            + ZopfliCostModelGetDistanceCost(model, j) as c_float;
                        let mut l: size_t = 0;
                        l = best_len.wrapping_add(1 as size_t);
                        while l <= len {
                            let copycode: uint16_t = GetCopyLengthCode(l) as uint16_t;
                            let cmdcode: uint16_t = CombineLengthCodes(
                                inscode,
                                copycode,
                                (j == 0 as size_t) as c_int,
                            ) as uint16_t;
                            let cost: c_float =
                                (if (cmdcode as c_int) < 128 as c_int {
                                    base_cost
                                } else {
                                    dist_cost
                                }) + GetCopyExtra(copycode) as c_float
                                    + ZopfliCostModelGetCommandCost(model, cmdcode)
                                        as c_float;
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
                let mut is_dictionary_match: c_int =
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
                let mut dist_cost_0: c_float = 0.;
                let mut max_match_len: size_t = 0;
                PrefixEncodeCopyDistance(
                    dist_code,
                    (*params).dist.num_direct_distance_codes as size_t,
                    (*params).dist.distance_postfix_bits as size_t,
                    &raw mut dist_symbol,
                    &raw mut distextra,
                );
                distnumextra =
                    (dist_symbol as c_int >> 10 as c_int) as uint32_t;
                dist_cost_0 = base_cost
                    + distnumextra as c_float
                    + ZopfliCostModelGetDistanceCost(
                        model,
                        (dist_symbol as c_int & 0x3ff as c_int) as size_t,
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
                        CombineLengthCodes(inscode, copycode_0, 0 as c_int)
                            as uint16_t;
                    let cost_0: c_float = dist_cost_0
                        + GetCopyExtra(copycode_0) as c_float
                        + ZopfliCostModelGetCommandCost(model, cmdcode_0) as c_float;
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
unsafe fn ComputeShortestPathFromNodes(
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
        index = (index as c_ulong).wrapping_sub(len as c_ulong) as size_t
            as size_t;
        (*nodes.offset(index as isize)).u.next = len as uint32_t;
        num_commands = num_commands.wrapping_add(1);
    }
    return num_commands;
}
#[inline]
pub unsafe fn BrotliZopfliCreateCommands(
    num_bytes: size_t,
    block_start: size_t,
    mut nodes: *const ZopfliNode,
    mut dist_cache: *mut c_int,
    mut last_insert_len: *mut size_t,
    mut params: *const BrotliEncoderParams,
    mut commands: *mut Command,
    mut num_literals: *mut size_t,
) {
    let stream_offset: size_t = (*params).stream_offset;
    let max_backward_limit: size_t = ((1 as c_int as size_t) << (*params).lgwin)
        .wrapping_sub(BROTLI_WINDOW_GAP as size_t);
    let mut pos: size_t = 0 as size_t;
    let mut offset: uint32_t = (*nodes.offset(0 as c_int as isize)).u.next;
    let mut i: size_t = 0;
    let mut gap: size_t = (*params).dictionary.compound.total_size;
    i = 0 as size_t;
    while offset != BROTLI_UINT32_MAX {
        let mut next: *const ZopfliNode =
            nodes.offset(pos.wrapping_add(offset as size_t) as isize) as *const ZopfliNode;
        let mut copy_length: size_t = ZopfliNodeCopyLength(next) as size_t;
        let mut insert_length: size_t =
            ((*next).dcode_insert_length & 0x7ffffff as uint32_t) as size_t;
        pos = (pos as c_ulong).wrapping_add(insert_length as c_ulong)
            as size_t as size_t;
        offset = (*next).u.next;
        if i == 0 as size_t {
            insert_length = (insert_length as c_ulong)
                .wrapping_add(*last_insert_len as c_ulong)
                as size_t as size_t;
            *last_insert_len = 0 as size_t;
        }
        let mut distance: size_t = ZopfliNodeCopyDistance(next) as size_t;
        let mut len_code: size_t = ZopfliNodeLengthCode(next) as size_t;
        let mut dictionary_start: size_t = brotli_min_size_t(
            block_start.wrapping_add(pos).wrapping_add(stream_offset),
            max_backward_limit,
        );
        let mut is_dictionary: c_int = if distance > dictionary_start.wrapping_add(gap)
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
            len_code as c_int - copy_length as c_int,
            dist_code,
        );
        if is_dictionary == 0 && dist_code > 0 as size_t {
            *dist_cache.offset(3 as c_int as isize) =
                *dist_cache.offset(2 as c_int as isize);
            *dist_cache.offset(2 as c_int as isize) =
                *dist_cache.offset(1 as c_int as isize);
            *dist_cache.offset(1 as c_int as isize) =
                *dist_cache.offset(0 as c_int as isize);
            *dist_cache.offset(0 as c_int as isize) = distance as c_int;
        }
        *num_literals = (*num_literals as c_ulong)
            .wrapping_add(insert_length as c_ulong) as size_t
            as size_t;
        pos = (pos as c_ulong).wrapping_add(copy_length as c_ulong)
            as size_t as size_t;
        i = i.wrapping_add(1);
    }
    *last_insert_len = (*last_insert_len as c_ulong)
        .wrapping_add(num_bytes.wrapping_sub(pos) as c_ulong)
        as size_t as size_t;
}
unsafe fn ZopfliIterate(
    mut num_bytes: size_t,
    mut position: size_t,
    mut ringbuffer: *const uint8_t,
    mut ringbuffer_mask: size_t,
    mut params: *const BrotliEncoderParams,
    gap: size_t,
    mut dist_cache: *const c_int,
    mut model: *const ZopfliCostModel,
    mut num_matches: *const uint32_t,
    mut matches: *const BackwardMatch,
    mut nodes: *mut ZopfliNode,
) -> size_t {
    let stream_offset: size_t = (*params).stream_offset;
    let max_backward_limit: size_t = ((1 as c_int as size_t) << (*params).lgwin)
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
    (*nodes.offset(0 as c_int as isize)).length = 0 as uint32_t;
    (*nodes.offset(0 as c_int as isize)).u.cost =
        0 as c_int as c_float;
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
        cur_match_pos = (cur_match_pos as c_ulong)
            .wrapping_add(*num_matches.offset(i as isize) as c_ulong)
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
                cur_match_pos = (cur_match_pos as c_ulong)
                    .wrapping_add(*num_matches.offset(i as isize) as c_ulong)
                    as size_t as size_t;
                skip = skip.wrapping_sub(1);
            }
        }
        i = i.wrapping_add(1);
    }
    return ComputeShortestPathFromNodes(num_bytes, nodes);
}
unsafe fn MergeMatches(
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
#[inline]
pub unsafe fn BrotliZopfliComputeShortestPath(
    mut m: *mut MemoryManager,
    mut num_bytes: size_t,
    mut position: size_t,
    mut ringbuffer: *const uint8_t,
    mut ringbuffer_mask: size_t,
    mut literal_context_lut: ContextLut,
    mut params: *const BrotliEncoderParams,
    mut dist_cache: *const c_int,
    mut hasher: *mut Hasher,
    mut nodes: *mut ZopfliNode,
) -> size_t {
    let stream_offset: size_t = (*params).stream_offset;
    let max_backward_limit: size_t = ((1 as c_int as size_t) << (*params).lgwin)
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
    let mut matches: *mut BackwardMatch = if 2 as c_int
        * (128 as c_int + 64 as c_int)
        > 0 as c_int
    {
        BrotliAllocate(
            m,
            ((2 as c_int * (128 as c_int + 64 as c_int))
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
        MAX_NUM_MATCHES_H10 + 128 as c_int
    } else {
        0 as c_int
    }) as size_t;
    let mut model: *mut ZopfliCostModel = if 1 as c_int > 0 as c_int {
        BrotliAllocate(
            m,
            (1 as size_t).wrapping_mul(::core::mem::size_of::<ZopfliCostModel>() as size_t),
        ) as *mut ZopfliCostModel
    } else {
        ::core::ptr::null_mut::<ZopfliCostModel>()
    };
    if 0 as c_int != 0 || 0 as c_int != 0 || 0 as c_int != 0
    {
        return 0 as size_t;
    }
    (*nodes.offset(0 as c_int as isize)).length = 0 as uint32_t;
    (*nodes.offset(0 as c_int as isize)).u.cost =
        0 as c_int as c_float;
    InitZopfliCostModel(m, model, &raw const (*params).dist, num_bytes);
    if 0 as c_int != 0 {
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
        let mut dict_id: c_int = 0 as c_int;
        if (*params).dictionary.contextual.context_based != 0 {
            let mut p1: uint8_t = (if pos >= 1 as size_t {
                *ringbuffer.offset((pos.wrapping_sub(1 as size_t) & ringbuffer_mask) as isize)
                    as c_int
            } else {
                0 as c_int
            }) as uint8_t;
            let mut p2: uint8_t = (if pos >= 2 as size_t {
                *ringbuffer.offset((pos.wrapping_sub(2 as size_t) & ringbuffer_mask) as isize)
                    as c_int
            } else {
                0 as c_int
            }) as uint8_t;
            dict_id = (*params).dictionary.contextual.context_map[(*literal_context_lut
                .offset(p1 as isize)
                as c_int
                | *literal_context_lut
                    .offset(256 as c_int as isize)
                    .offset(p2 as isize) as c_int)
                as usize] as c_int;
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
            num_matches = (num_matches as c_ulong)
                .wrapping_add(cd_matches as c_ulong)
                as size_t as size_t;
        }
        if num_matches > 0 as size_t
            && BackwardMatchLength(
                matches.offset(num_matches.wrapping_sub(1 as size_t) as isize)
                    as *mut BackwardMatch,
            ) > max_zopfli_len
        {
            *matches.offset(0 as c_int as isize) =
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
                matches.offset(0 as c_int as isize) as *mut BackwardMatch
            ) > max_zopfli_len
        {
            skip = brotli_max_size_t(
                BackwardMatchLength(
                    matches.offset(0 as c_int as isize) as *mut BackwardMatch
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
    BrotliFree(m, model as *mut c_void);
    model = ::core::ptr::null_mut::<ZopfliCostModel>();
    BrotliFree(m, matches as *mut c_void);
    matches = ::core::ptr::null_mut::<BackwardMatch>();
    return ComputeShortestPathFromNodes(num_bytes, nodes);
}
#[inline]
pub unsafe fn BrotliCreateZopfliBackwardReferences(
    mut m: *mut MemoryManager,
    mut num_bytes: size_t,
    mut position: size_t,
    mut ringbuffer: *const uint8_t,
    mut ringbuffer_mask: size_t,
    mut literal_context_lut: ContextLut,
    mut params: *const BrotliEncoderParams,
    mut hasher: *mut Hasher,
    mut dist_cache: *mut c_int,
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
    if 0 as c_int != 0 || 0 as c_int != 0 {
        return;
    }
    BrotliInitZopfliNodes(nodes, num_bytes.wrapping_add(1 as size_t));
    *num_commands =
        (*num_commands as c_ulong).wrapping_add(BrotliZopfliComputeShortestPath(
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
        ) as c_ulong) as size_t as size_t;
    if 0 as c_int != 0 {
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
    BrotliFree(m, nodes as *mut c_void);
    nodes = ::core::ptr::null_mut::<ZopfliNode>();
}
#[inline]
pub unsafe fn BrotliCreateHqZopfliBackwardReferences(
    mut m: *mut MemoryManager,
    mut num_bytes: size_t,
    mut position: size_t,
    mut ringbuffer: *const uint8_t,
    mut ringbuffer_mask: size_t,
    mut literal_context_lut: ContextLut,
    mut params: *const BrotliEncoderParams,
    mut hasher: *mut Hasher,
    mut dist_cache: *mut c_int,
    mut last_insert_len: *mut size_t,
    mut commands: *mut Command,
    mut num_commands: *mut size_t,
    mut num_literals: *mut size_t,
) {
    let stream_offset: size_t = (*params).stream_offset;
    let max_backward_limit: size_t = ((1 as c_int as size_t) << (*params).lgwin)
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
    let mut orig_dist_cache: [c_int; 4] = [0; 4];
    let mut orig_num_commands: size_t = 0;
    let mut model: *mut ZopfliCostModel = if 1 as c_int > 0 as c_int {
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
        MAX_NUM_MATCHES_H10 + 128 as c_int
    } else {
        0 as c_int
    }) as size_t;
    if 0 as c_int != 0
        || 0 as c_int != 0
        || 0 as c_int != 0
        || 0 as c_int != 0
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
        let mut dict_id: c_int = 0 as c_int;
        if (*params).dictionary.contextual.context_based != 0 {
            let mut p1: uint8_t = (if pos >= 1 as size_t {
                *ringbuffer.offset((pos.wrapping_sub(1 as size_t) & ringbuffer_mask) as isize)
                    as c_int
            } else {
                0 as c_int
            }) as uint8_t;
            let mut p2: uint8_t = (if pos >= 2 as size_t {
                *ringbuffer.offset((pos.wrapping_sub(2 as size_t) & ringbuffer_mask) as isize)
                    as c_int
            } else {
                0 as c_int
            }) as uint8_t;
            dict_id = (*params).dictionary.contextual.context_map[(*literal_context_lut
                .offset(p1 as isize)
                as c_int
                | *literal_context_lut
                    .offset(256 as c_int as isize)
                    .offset(p2 as isize) as c_int)
                as usize] as c_int;
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
                _new_size = (_new_size as c_ulong)
                    .wrapping_mul(2 as c_ulong) as size_t
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
            if 0 as c_int == 0
                && 0 as c_int == 0
                && matches_size != 0 as size_t
            {
                memcpy(
                    new_array as *mut c_void,
                    matches as *const c_void,
                    matches_size.wrapping_mul(::core::mem::size_of::<BackwardMatch>() as size_t),
                );
            }
            BrotliFree(m, matches as *mut c_void);
            matches = ::core::ptr::null_mut::<BackwardMatch>();
            matches = new_array;
            matches_size = _new_size;
        }
        if 0 as c_int != 0 {
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
            num_found_matches = (num_found_matches as c_ulong)
                .wrapping_add(cd_matches as c_ulong)
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
                        as *mut c_void,
                    0 as c_int,
                    skip.wrapping_mul(::core::mem::size_of::<uint32_t>() as size_t),
                );
                i = (i as c_ulong).wrapping_add(skip as c_ulong) as size_t
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
        &raw mut orig_dist_cache as *mut c_int as *mut c_void,
        dist_cache as *const c_void,
        (4 as size_t).wrapping_mul(::core::mem::size_of::<c_int>() as size_t),
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
    if 0 as c_int != 0 || 0 as c_int != 0 {
        return;
    }
    InitZopfliCostModel(m, model, &raw const (*params).dist, num_bytes);
    if 0 as c_int != 0 {
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
            dist_cache as *mut c_void,
            &raw mut orig_dist_cache as *mut c_int as *const c_void,
            (4 as size_t).wrapping_mul(::core::mem::size_of::<c_int>() as size_t),
        );
        *num_commands = (*num_commands as c_ulong).wrapping_add(ZopfliIterate(
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
            as c_ulong) as size_t as size_t;
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
    BrotliFree(m, model as *mut c_void);
    model = ::core::ptr::null_mut::<ZopfliCostModel>();
    BrotliFree(m, nodes as *mut c_void);
    nodes = ::core::ptr::null_mut::<ZopfliNode>();
    BrotliFree(m, matches as *mut c_void);
    matches = ::core::ptr::null_mut::<BackwardMatch>();
    BrotliFree(m, num_matches as *mut c_void);
    num_matches = ::core::ptr::null_mut::<uint32_t>();
}
