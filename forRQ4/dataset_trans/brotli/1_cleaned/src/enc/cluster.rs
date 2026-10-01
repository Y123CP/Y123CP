use core::ffi::*;
pub use crate::src::c_inlined_fns::*;
pub use crate::src::ffi::*;
pub use crate::src::c_consts::*;
pub use crate::src::c_structs::*;
pub use crate::src::c_types::*;

#[inline(always)]
unsafe extern "C" fn brotli_max_double(
    mut a: c_double,
    mut b: c_double,
) -> c_double {
    return if a > b { a } else { b };
}

#[inline(always)]
unsafe extern "C" fn HistogramAddHistogramLiteral(
    mut self_0: *mut HistogramLiteral,
    mut v: *const HistogramLiteral,
) {
    let mut i: size_t = 0;
    (*self_0).total_count_ = ((*self_0).total_count_ as c_ulong)
        .wrapping_add((*v).total_count_ as c_ulong)
        as size_t as size_t;
    i = 0 as size_t;
    while i < DATA_SIZE as size_t {
        (*self_0).data_[i as usize] = ((*self_0).data_[i as usize] as c_uint)
            .wrapping_add((*v).data_[i as usize] as c_uint)
            as uint32_t as uint32_t;
        i = i.wrapping_add(1);
    }
}

#[inline(always)]
unsafe extern "C" fn HistogramAddHistogramDistance(
    mut self_0: *mut HistogramDistance,
    mut v: *const HistogramDistance,
) {
    let mut i: size_t = 0;
    (*self_0).total_count_ = ((*self_0).total_count_ as c_ulong)
        .wrapping_add((*v).total_count_ as c_ulong)
        as size_t as size_t;
    i = 0 as size_t;
    while i < DATA_SIZE_1 as size_t {
        (*self_0).data_[i as usize] = ((*self_0).data_[i as usize] as c_uint)
            .wrapping_add((*v).data_[i as usize] as c_uint)
            as uint32_t as uint32_t;
        i = i.wrapping_add(1);
    }
}

pub const DATA_SIZE: c_int = BROTLI_NUM_LITERAL_SYMBOLS;

pub const DATA_SIZE_1: c_int = BROTLI_NUM_HISTOGRAM_DISTANCE_SYMBOLS;
#[no_mangle]
pub unsafe extern "C" fn BrotliCompareAndPushToQueueLiteral(
    mut out: *const HistogramLiteral,
    mut tmp: *mut HistogramLiteral,
    mut cluster_size: *const uint32_t,
    mut idx1: uint32_t,
    mut idx2: uint32_t,
    mut max_num_pairs: size_t,
    mut pairs: *mut HistogramPair,
    mut num_pairs: *mut size_t,
) {
    let mut is_good_pair: c_int = 0 as c_int;
    let mut p: HistogramPair = HistogramPair {
        idx1: 0,
        idx2: 0,
        cost_combo: 0.,
        cost_diff: 0.,
    };
    p.idx2 = 0 as uint32_t;
    p.idx1 = p.idx2;
    p.cost_combo = 0 as c_int as c_double;
    p.cost_diff = p.cost_combo;
    if idx1 == idx2 {
        return;
    }
    if idx2 < idx1 {
        let mut t: uint32_t = idx2;
        idx2 = idx1;
        idx1 = t;
    }
    p.idx1 = idx1;
    p.idx2 = idx2;
    p.cost_diff = 0.5f64
        * ClusterCostDiff(
            *cluster_size.offset(idx1 as isize) as size_t,
            *cluster_size.offset(idx2 as isize) as size_t,
        );
    p.cost_diff -= (*out.offset(idx1 as isize)).bit_cost_;
    p.cost_diff -= (*out.offset(idx2 as isize)).bit_cost_;
    if (*out.offset(idx1 as isize)).total_count_ == 0 as size_t {
        p.cost_combo = (*out.offset(idx2 as isize)).bit_cost_;
        is_good_pair = 1 as c_int;
    } else if (*out.offset(idx2 as isize)).total_count_ == 0 as size_t {
        p.cost_combo = (*out.offset(idx1 as isize)).bit_cost_;
        is_good_pair = 1 as c_int;
    } else {
        let mut threshold: c_double = if *num_pairs == 0 as size_t {
            1e99f64
        } else {
            brotli_max_double(
                0.0f64,
                (*pairs.offset(0 as c_int as isize)).cost_diff,
            )
        };
        let mut cost_combo: c_double = 0.;
        *tmp = *out.offset(idx1 as isize);
        HistogramAddHistogramLiteral(tmp, out.offset(idx2 as isize) as *const HistogramLiteral);
        cost_combo = BrotliPopulationCostLiteral(tmp);
        if cost_combo < threshold - p.cost_diff {
            p.cost_combo = cost_combo;
            is_good_pair = 1 as c_int;
        }
    }
    if is_good_pair != 0 {
        p.cost_diff += p.cost_combo;
        if *num_pairs > 0 as size_t
            && HistogramPairIsLess(
                pairs.offset(0 as c_int as isize) as *mut HistogramPair,
                &raw mut p,
            ) != 0
        {
            if *num_pairs < max_num_pairs {
                *pairs.offset(*num_pairs as isize) =
                    *pairs.offset(0 as c_int as isize);
                *num_pairs = (*num_pairs).wrapping_add(1);
            }
            *pairs.offset(0 as c_int as isize) = p;
        } else if *num_pairs < max_num_pairs {
            *pairs.offset(*num_pairs as isize) = p;
            *num_pairs = (*num_pairs).wrapping_add(1);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn BrotliHistogramCombineLiteral(
    mut out: *mut HistogramLiteral,
    mut tmp: *mut HistogramLiteral,
    mut cluster_size: *mut uint32_t,
    mut symbols: *mut uint32_t,
    mut clusters: *mut uint32_t,
    mut pairs: *mut HistogramPair,
    mut num_clusters: size_t,
    mut symbols_size: size_t,
    mut max_clusters: size_t,
    mut max_num_pairs: size_t,
) -> size_t {
    let mut cost_diff_threshold: c_double = 0.0f64;
    let mut min_cluster_size: size_t = 1 as size_t;
    let mut num_pairs: size_t = 0 as size_t;
    let mut idx1: size_t = 0;
    idx1 = 0 as size_t;
    while idx1 < num_clusters {
        let mut idx2: size_t = 0;
        idx2 = idx1.wrapping_add(1 as size_t);
        while idx2 < num_clusters {
            BrotliCompareAndPushToQueueLiteral(
                out,
                tmp,
                cluster_size,
                *clusters.offset(idx1 as isize),
                *clusters.offset(idx2 as isize),
                max_num_pairs,
                pairs.offset(0 as c_int as isize) as *mut HistogramPair,
                &raw mut num_pairs,
            );
            idx2 = idx2.wrapping_add(1);
        }
        idx1 = idx1.wrapping_add(1);
    }
    while num_clusters > min_cluster_size {
        let mut best_idx1: uint32_t = 0;
        let mut best_idx2: uint32_t = 0;
        let mut i: size_t = 0;
        if (*pairs.offset(0 as c_int as isize)).cost_diff >= cost_diff_threshold {
            cost_diff_threshold = 1e99f64;
            min_cluster_size = max_clusters;
        } else {
            best_idx1 = (*pairs.offset(0 as c_int as isize)).idx1;
            best_idx2 = (*pairs.offset(0 as c_int as isize)).idx2;
            HistogramAddHistogramLiteral(
                out.offset(best_idx1 as isize) as *mut HistogramLiteral,
                out.offset(best_idx2 as isize) as *mut HistogramLiteral,
            );
            (*out.offset(best_idx1 as isize)).bit_cost_ =
                (*pairs.offset(0 as c_int as isize)).cost_combo;
            let ref mut fresh0 = *cluster_size.offset(best_idx1 as isize);
            *fresh0 = (*fresh0 as c_uint)
                .wrapping_add(*cluster_size.offset(best_idx2 as isize) as c_uint)
                as uint32_t as uint32_t;
            i = 0 as size_t;
            while i < symbols_size {
                if *symbols.offset(i as isize) == best_idx2 {
                    *symbols.offset(i as isize) = best_idx1;
                }
                i = i.wrapping_add(1);
            }
            i = 0 as size_t;
            while i < num_clusters {
                if *clusters.offset(i as isize) == best_idx2 {
                    memmove(
                        clusters.offset(i as isize) as *mut uint32_t as *mut c_void,
                        clusters.offset(i.wrapping_add(1 as size_t) as isize) as *mut uint32_t
                            as *const c_void,
                        num_clusters
                            .wrapping_sub(i)
                            .wrapping_sub(1 as size_t)
                            .wrapping_mul(::core::mem::size_of::<uint32_t>() as size_t),
                    );
                    break;
                } else {
                    i = i.wrapping_add(1);
                }
            }
            num_clusters = num_clusters.wrapping_sub(1);
            let mut copy_to_idx: size_t = 0 as size_t;
            i = 0 as size_t;
            while i < num_pairs {
                let mut p: *mut HistogramPair = pairs.offset(i as isize) as *mut HistogramPair;
                if !((*p).idx1 == best_idx1
                    || (*p).idx2 == best_idx1
                    || (*p).idx1 == best_idx2
                    || (*p).idx2 == best_idx2)
                {
                    if HistogramPairIsLess(
                        pairs.offset(0 as c_int as isize) as *mut HistogramPair,
                        p,
                    ) != 0
                    {
                        let mut front: HistogramPair =
                            *pairs.offset(0 as c_int as isize);
                        *pairs.offset(0 as c_int as isize) = *p;
                        *pairs.offset(copy_to_idx as isize) = front;
                    } else {
                        *pairs.offset(copy_to_idx as isize) = *p;
                    }
                    copy_to_idx = copy_to_idx.wrapping_add(1);
                }
                i = i.wrapping_add(1);
            }
            num_pairs = copy_to_idx;
            i = 0 as size_t;
            while i < num_clusters {
                BrotliCompareAndPushToQueueLiteral(
                    out,
                    tmp,
                    cluster_size,
                    best_idx1,
                    *clusters.offset(i as isize),
                    max_num_pairs,
                    pairs.offset(0 as c_int as isize) as *mut HistogramPair,
                    &raw mut num_pairs,
                );
                i = i.wrapping_add(1);
            }
        }
    }
    return num_clusters;
}
#[no_mangle]
pub unsafe extern "C" fn BrotliHistogramBitCostDistanceLiteral(
    mut histogram: *const HistogramLiteral,
    mut candidate: *const HistogramLiteral,
    mut tmp: *mut HistogramLiteral,
) -> c_double {
    if (*histogram).total_count_ == 0 as size_t {
        return 0.0f64;
    } else {
        *tmp = *histogram;
        HistogramAddHistogramLiteral(tmp, candidate);
        return BrotliPopulationCostLiteral(tmp) - (*candidate).bit_cost_;
    };
}
#[no_mangle]
pub unsafe extern "C" fn BrotliHistogramRemapLiteral(
    mut in_0: *const HistogramLiteral,
    mut in_size: size_t,
    mut clusters: *const uint32_t,
    mut num_clusters: size_t,
    mut out: *mut HistogramLiteral,
    mut tmp: *mut HistogramLiteral,
    mut symbols: *mut uint32_t,
) {
    let mut i: size_t = 0;
    i = 0 as size_t;
    while i < in_size {
        let mut best_out: uint32_t = if i == 0 as size_t {
            *symbols.offset(0 as c_int as isize)
        } else {
            *symbols.offset(i.wrapping_sub(1 as size_t) as isize)
        };
        let mut best_bits: c_double = BrotliHistogramBitCostDistanceLiteral(
            in_0.offset(i as isize) as *const HistogramLiteral,
            out.offset(best_out as isize) as *mut HistogramLiteral,
            tmp,
        );
        let mut j: size_t = 0;
        j = 0 as size_t;
        while j < num_clusters {
            let cur_bits: c_double = BrotliHistogramBitCostDistanceLiteral(
                in_0.offset(i as isize) as *const HistogramLiteral,
                out.offset(*clusters.offset(j as isize) as isize) as *mut HistogramLiteral,
                tmp,
            ) as c_double;
            if cur_bits < best_bits {
                best_bits = cur_bits;
                best_out = *clusters.offset(j as isize);
            }
            j = j.wrapping_add(1);
        }
        *symbols.offset(i as isize) = best_out;
        i = i.wrapping_add(1);
    }
    i = 0 as size_t;
    while i < num_clusters {
        HistogramClearLiteral(
            out.offset(*clusters.offset(i as isize) as isize) as *mut HistogramLiteral
        );
        i = i.wrapping_add(1);
    }
    i = 0 as size_t;
    while i < in_size {
        HistogramAddHistogramLiteral(
            out.offset(*symbols.offset(i as isize) as isize) as *mut HistogramLiteral,
            in_0.offset(i as isize) as *const HistogramLiteral,
        );
        i = i.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn BrotliHistogramReindexLiteral(
    mut m: *mut MemoryManager,
    mut out: *mut HistogramLiteral,
    mut symbols: *mut uint32_t,
    mut length: size_t,
) -> size_t {
    static mut kInvalidIndex: uint32_t = !(0 as c_int as uint32_t);
    let mut new_index: *mut uint32_t = if length > 0 as size_t {
        BrotliAllocate(
            m,
            length.wrapping_mul(::core::mem::size_of::<uint32_t>() as size_t),
        ) as *mut uint32_t
    } else {
        ::core::ptr::null_mut::<uint32_t>()
    };
    let mut next_index: uint32_t = 0;
    let mut tmp: *mut HistogramLiteral = ::core::ptr::null_mut::<HistogramLiteral>();
    let mut i: size_t = 0;
    if 0 as c_int != 0 || 0 as c_int != 0 {
        return 0 as size_t;
    }
    i = 0 as size_t;
    while i < length {
        *new_index.offset(i as isize) = kInvalidIndex;
        i = i.wrapping_add(1);
    }
    next_index = 0 as uint32_t;
    i = 0 as size_t;
    while i < length {
        if *new_index.offset(*symbols.offset(i as isize) as isize) == kInvalidIndex {
            *new_index.offset(*symbols.offset(i as isize) as isize) = next_index;
            next_index = next_index.wrapping_add(1);
        }
        i = i.wrapping_add(1);
    }
    tmp = if next_index > 0 as uint32_t {
        BrotliAllocate(
            m,
            (next_index as size_t)
                .wrapping_mul(::core::mem::size_of::<HistogramLiteral>() as size_t),
        ) as *mut HistogramLiteral
    } else {
        ::core::ptr::null_mut::<HistogramLiteral>()
    };
    if 0 as c_int != 0 || 0 as c_int != 0 {
        return 0 as size_t;
    }
    next_index = 0 as uint32_t;
    i = 0 as size_t;
    while i < length {
        if *new_index.offset(*symbols.offset(i as isize) as isize) == next_index {
            *tmp.offset(next_index as isize) = *out.offset(*symbols.offset(i as isize) as isize);
            next_index = next_index.wrapping_add(1);
        }
        *symbols.offset(i as isize) = *new_index.offset(*symbols.offset(i as isize) as isize);
        i = i.wrapping_add(1);
    }
    BrotliFree(m, new_index as *mut c_void);
    new_index = ::core::ptr::null_mut::<uint32_t>();
    i = 0 as size_t;
    while i < next_index as size_t {
        *out.offset(i as isize) = *tmp.offset(i as isize);
        i = i.wrapping_add(1);
    }
    BrotliFree(m, tmp as *mut c_void);
    tmp = ::core::ptr::null_mut::<HistogramLiteral>();
    return next_index as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn BrotliClusterHistogramsLiteral(
    mut m: *mut MemoryManager,
    mut in_0: *const HistogramLiteral,
    in_size: size_t,
    mut max_histograms: size_t,
    mut out: *mut HistogramLiteral,
    mut out_size: *mut size_t,
    mut histogram_symbols: *mut uint32_t,
) {
    let mut cluster_size: *mut uint32_t = if in_size > 0 as size_t {
        BrotliAllocate(
            m,
            in_size.wrapping_mul(::core::mem::size_of::<uint32_t>() as size_t),
        ) as *mut uint32_t
    } else {
        ::core::ptr::null_mut::<uint32_t>()
    };
    let mut clusters: *mut uint32_t = if in_size > 0 as size_t {
        BrotliAllocate(
            m,
            in_size.wrapping_mul(::core::mem::size_of::<uint32_t>() as size_t),
        ) as *mut uint32_t
    } else {
        ::core::ptr::null_mut::<uint32_t>()
    };
    let mut num_clusters: size_t = 0 as size_t;
    let max_input_histograms: size_t = 64 as size_t;
    let mut pairs_capacity: size_t = max_input_histograms
        .wrapping_mul(max_input_histograms)
        .wrapping_div(2 as size_t);
    let mut pairs: *mut HistogramPair = if pairs_capacity.wrapping_add(1 as size_t) > 0 as size_t {
        BrotliAllocate(
            m,
            pairs_capacity
                .wrapping_add(1 as size_t)
                .wrapping_mul(::core::mem::size_of::<HistogramPair>() as size_t),
        ) as *mut HistogramPair
    } else {
        ::core::ptr::null_mut::<HistogramPair>()
    };
    let mut tmp: *mut HistogramLiteral = if 1 as c_int > 0 as c_int {
        BrotliAllocate(
            m,
            (1 as size_t).wrapping_mul(::core::mem::size_of::<HistogramLiteral>() as size_t),
        ) as *mut HistogramLiteral
    } else {
        ::core::ptr::null_mut::<HistogramLiteral>()
    };
    let mut i: size_t = 0;
    if 0 as c_int != 0
        || 0 as c_int != 0
        || 0 as c_int != 0
        || 0 as c_int != 0
        || 0 as c_int != 0
    {
        return;
    }
    i = 0 as size_t;
    while i < in_size {
        *cluster_size.offset(i as isize) = 1 as uint32_t;
        i = i.wrapping_add(1);
    }
    i = 0 as size_t;
    while i < in_size {
        *out.offset(i as isize) = *in_0.offset(i as isize);
        (*out.offset(i as isize)).bit_cost_ =
            BrotliPopulationCostLiteral(in_0.offset(i as isize) as *const HistogramLiteral);
        *histogram_symbols.offset(i as isize) = i as uint32_t;
        i = i.wrapping_add(1);
    }
    i = 0 as size_t;
    while i < in_size {
        let mut num_to_combine: size_t =
            brotli_min_size_t(in_size.wrapping_sub(i), max_input_histograms);
        let mut num_new_clusters: size_t = 0;
        let mut j: size_t = 0;
        j = 0 as size_t;
        while j < num_to_combine {
            *clusters.offset(num_clusters.wrapping_add(j) as isize) = i.wrapping_add(j) as uint32_t;
            j = j.wrapping_add(1);
        }
        num_new_clusters = BrotliHistogramCombineLiteral(
            out,
            tmp,
            cluster_size,
            histogram_symbols.offset(i as isize) as *mut uint32_t,
            clusters.offset(num_clusters as isize) as *mut uint32_t,
            pairs,
            num_to_combine,
            num_to_combine,
            max_histograms,
            pairs_capacity,
        );
        num_clusters = (num_clusters as c_ulong)
            .wrapping_add(num_new_clusters as c_ulong) as size_t
            as size_t;
        i = (i as c_ulong).wrapping_add(max_input_histograms as c_ulong)
            as size_t as size_t;
    }
    let mut max_num_pairs: size_t = brotli_min_size_t(
        (64 as size_t).wrapping_mul(num_clusters),
        num_clusters
            .wrapping_div(2 as size_t)
            .wrapping_mul(num_clusters),
    );
    if pairs_capacity < max_num_pairs.wrapping_add(1 as size_t) {
        let mut _new_size: size_t = if pairs_capacity == 0 as size_t {
            max_num_pairs.wrapping_add(1 as size_t)
        } else {
            pairs_capacity
        };
        let mut new_array: *mut HistogramPair = ::core::ptr::null_mut::<HistogramPair>();
        while _new_size < max_num_pairs.wrapping_add(1 as size_t) {
            _new_size = (_new_size as c_ulong).wrapping_mul(2 as c_ulong)
                as size_t as size_t;
        }
        new_array = if _new_size > 0 as size_t {
            BrotliAllocate(
                m,
                _new_size.wrapping_mul(::core::mem::size_of::<HistogramPair>() as size_t),
            ) as *mut HistogramPair
        } else {
            ::core::ptr::null_mut::<HistogramPair>()
        };
        if 0 as c_int == 0
            && 0 as c_int == 0
            && pairs_capacity != 0 as size_t
        {
            memcpy(
                new_array as *mut c_void,
                pairs as *const c_void,
                pairs_capacity.wrapping_mul(::core::mem::size_of::<HistogramPair>() as size_t),
            );
        }
        BrotliFree(m, pairs as *mut c_void);
        pairs = ::core::ptr::null_mut::<HistogramPair>();
        pairs = new_array;
        pairs_capacity = _new_size;
    }
    if 0 as c_int != 0 {
        return;
    }
    num_clusters = BrotliHistogramCombineLiteral(
        out,
        tmp,
        cluster_size,
        histogram_symbols,
        clusters,
        pairs,
        num_clusters,
        in_size,
        max_histograms,
        max_num_pairs,
    );
    BrotliFree(m, pairs as *mut c_void);
    pairs = ::core::ptr::null_mut::<HistogramPair>();
    BrotliFree(m, cluster_size as *mut c_void);
    cluster_size = ::core::ptr::null_mut::<uint32_t>();
    BrotliHistogramRemapLiteral(
        in_0,
        in_size,
        clusters,
        num_clusters,
        out,
        tmp,
        histogram_symbols,
    );
    BrotliFree(m, tmp as *mut c_void);
    tmp = ::core::ptr::null_mut::<HistogramLiteral>();
    BrotliFree(m, clusters as *mut c_void);
    clusters = ::core::ptr::null_mut::<uint32_t>();
    *out_size = BrotliHistogramReindexLiteral(m, out, histogram_symbols, in_size);
    if 0 as c_int != 0 {
        return;
    }
}
#[no_mangle]
pub unsafe extern "C" fn BrotliCompareAndPushToQueueCommand(
    mut out: *const HistogramCommand,
    mut tmp: *mut HistogramCommand,
    mut cluster_size: *const uint32_t,
    mut idx1: uint32_t,
    mut idx2: uint32_t,
    mut max_num_pairs: size_t,
    mut pairs: *mut HistogramPair,
    mut num_pairs: *mut size_t,
) {
    let mut is_good_pair: c_int = 0 as c_int;
    let mut p: HistogramPair = HistogramPair {
        idx1: 0,
        idx2: 0,
        cost_combo: 0.,
        cost_diff: 0.,
    };
    p.idx2 = 0 as uint32_t;
    p.idx1 = p.idx2;
    p.cost_combo = 0 as c_int as c_double;
    p.cost_diff = p.cost_combo;
    if idx1 == idx2 {
        return;
    }
    if idx2 < idx1 {
        let mut t: uint32_t = idx2;
        idx2 = idx1;
        idx1 = t;
    }
    p.idx1 = idx1;
    p.idx2 = idx2;
    p.cost_diff = 0.5f64
        * ClusterCostDiff(
            *cluster_size.offset(idx1 as isize) as size_t,
            *cluster_size.offset(idx2 as isize) as size_t,
        );
    p.cost_diff -= (*out.offset(idx1 as isize)).bit_cost_;
    p.cost_diff -= (*out.offset(idx2 as isize)).bit_cost_;
    if (*out.offset(idx1 as isize)).total_count_ == 0 as size_t {
        p.cost_combo = (*out.offset(idx2 as isize)).bit_cost_;
        is_good_pair = 1 as c_int;
    } else if (*out.offset(idx2 as isize)).total_count_ == 0 as size_t {
        p.cost_combo = (*out.offset(idx1 as isize)).bit_cost_;
        is_good_pair = 1 as c_int;
    } else {
        let mut threshold: c_double = if *num_pairs == 0 as size_t {
            1e99f64
        } else {
            brotli_max_double(
                0.0f64,
                (*pairs.offset(0 as c_int as isize)).cost_diff,
            )
        };
        let mut cost_combo: c_double = 0.;
        *tmp = *out.offset(idx1 as isize);
        HistogramAddHistogramCommand(tmp, out.offset(idx2 as isize) as *const HistogramCommand);
        cost_combo = BrotliPopulationCostCommand(tmp);
        if cost_combo < threshold - p.cost_diff {
            p.cost_combo = cost_combo;
            is_good_pair = 1 as c_int;
        }
    }
    if is_good_pair != 0 {
        p.cost_diff += p.cost_combo;
        if *num_pairs > 0 as size_t
            && HistogramPairIsLess(
                pairs.offset(0 as c_int as isize) as *mut HistogramPair,
                &raw mut p,
            ) != 0
        {
            if *num_pairs < max_num_pairs {
                *pairs.offset(*num_pairs as isize) =
                    *pairs.offset(0 as c_int as isize);
                *num_pairs = (*num_pairs).wrapping_add(1);
            }
            *pairs.offset(0 as c_int as isize) = p;
        } else if *num_pairs < max_num_pairs {
            *pairs.offset(*num_pairs as isize) = p;
            *num_pairs = (*num_pairs).wrapping_add(1);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn BrotliHistogramCombineCommand(
    mut out: *mut HistogramCommand,
    mut tmp: *mut HistogramCommand,
    mut cluster_size: *mut uint32_t,
    mut symbols: *mut uint32_t,
    mut clusters: *mut uint32_t,
    mut pairs: *mut HistogramPair,
    mut num_clusters: size_t,
    mut symbols_size: size_t,
    mut max_clusters: size_t,
    mut max_num_pairs: size_t,
) -> size_t {
    let mut cost_diff_threshold: c_double = 0.0f64;
    let mut min_cluster_size: size_t = 1 as size_t;
    let mut num_pairs: size_t = 0 as size_t;
    let mut idx1: size_t = 0;
    idx1 = 0 as size_t;
    while idx1 < num_clusters {
        let mut idx2: size_t = 0;
        idx2 = idx1.wrapping_add(1 as size_t);
        while idx2 < num_clusters {
            BrotliCompareAndPushToQueueCommand(
                out,
                tmp,
                cluster_size,
                *clusters.offset(idx1 as isize),
                *clusters.offset(idx2 as isize),
                max_num_pairs,
                pairs.offset(0 as c_int as isize) as *mut HistogramPair,
                &raw mut num_pairs,
            );
            idx2 = idx2.wrapping_add(1);
        }
        idx1 = idx1.wrapping_add(1);
    }
    while num_clusters > min_cluster_size {
        let mut best_idx1: uint32_t = 0;
        let mut best_idx2: uint32_t = 0;
        let mut i: size_t = 0;
        if (*pairs.offset(0 as c_int as isize)).cost_diff >= cost_diff_threshold {
            cost_diff_threshold = 1e99f64;
            min_cluster_size = max_clusters;
        } else {
            best_idx1 = (*pairs.offset(0 as c_int as isize)).idx1;
            best_idx2 = (*pairs.offset(0 as c_int as isize)).idx2;
            HistogramAddHistogramCommand(
                out.offset(best_idx1 as isize) as *mut HistogramCommand,
                out.offset(best_idx2 as isize) as *mut HistogramCommand,
            );
            (*out.offset(best_idx1 as isize)).bit_cost_ =
                (*pairs.offset(0 as c_int as isize)).cost_combo;
            let ref mut fresh1 = *cluster_size.offset(best_idx1 as isize);
            *fresh1 = (*fresh1 as c_uint)
                .wrapping_add(*cluster_size.offset(best_idx2 as isize) as c_uint)
                as uint32_t as uint32_t;
            i = 0 as size_t;
            while i < symbols_size {
                if *symbols.offset(i as isize) == best_idx2 {
                    *symbols.offset(i as isize) = best_idx1;
                }
                i = i.wrapping_add(1);
            }
            i = 0 as size_t;
            while i < num_clusters {
                if *clusters.offset(i as isize) == best_idx2 {
                    memmove(
                        clusters.offset(i as isize) as *mut uint32_t as *mut c_void,
                        clusters.offset(i.wrapping_add(1 as size_t) as isize) as *mut uint32_t
                            as *const c_void,
                        num_clusters
                            .wrapping_sub(i)
                            .wrapping_sub(1 as size_t)
                            .wrapping_mul(::core::mem::size_of::<uint32_t>() as size_t),
                    );
                    break;
                } else {
                    i = i.wrapping_add(1);
                }
            }
            num_clusters = num_clusters.wrapping_sub(1);
            let mut copy_to_idx: size_t = 0 as size_t;
            i = 0 as size_t;
            while i < num_pairs {
                let mut p: *mut HistogramPair = pairs.offset(i as isize) as *mut HistogramPair;
                if !((*p).idx1 == best_idx1
                    || (*p).idx2 == best_idx1
                    || (*p).idx1 == best_idx2
                    || (*p).idx2 == best_idx2)
                {
                    if HistogramPairIsLess(
                        pairs.offset(0 as c_int as isize) as *mut HistogramPair,
                        p,
                    ) != 0
                    {
                        let mut front: HistogramPair =
                            *pairs.offset(0 as c_int as isize);
                        *pairs.offset(0 as c_int as isize) = *p;
                        *pairs.offset(copy_to_idx as isize) = front;
                    } else {
                        *pairs.offset(copy_to_idx as isize) = *p;
                    }
                    copy_to_idx = copy_to_idx.wrapping_add(1);
                }
                i = i.wrapping_add(1);
            }
            num_pairs = copy_to_idx;
            i = 0 as size_t;
            while i < num_clusters {
                BrotliCompareAndPushToQueueCommand(
                    out,
                    tmp,
                    cluster_size,
                    best_idx1,
                    *clusters.offset(i as isize),
                    max_num_pairs,
                    pairs.offset(0 as c_int as isize) as *mut HistogramPair,
                    &raw mut num_pairs,
                );
                i = i.wrapping_add(1);
            }
        }
    }
    return num_clusters;
}
#[no_mangle]
pub unsafe extern "C" fn BrotliHistogramBitCostDistanceCommand(
    mut histogram: *const HistogramCommand,
    mut candidate: *const HistogramCommand,
    mut tmp: *mut HistogramCommand,
) -> c_double {
    if (*histogram).total_count_ == 0 as size_t {
        return 0.0f64;
    } else {
        *tmp = *histogram;
        HistogramAddHistogramCommand(tmp, candidate);
        return BrotliPopulationCostCommand(tmp) - (*candidate).bit_cost_;
    };
}
#[no_mangle]
pub unsafe extern "C" fn BrotliHistogramRemapCommand(
    mut in_0: *const HistogramCommand,
    mut in_size: size_t,
    mut clusters: *const uint32_t,
    mut num_clusters: size_t,
    mut out: *mut HistogramCommand,
    mut tmp: *mut HistogramCommand,
    mut symbols: *mut uint32_t,
) {
    let mut i: size_t = 0;
    i = 0 as size_t;
    while i < in_size {
        let mut best_out: uint32_t = if i == 0 as size_t {
            *symbols.offset(0 as c_int as isize)
        } else {
            *symbols.offset(i.wrapping_sub(1 as size_t) as isize)
        };
        let mut best_bits: c_double = BrotliHistogramBitCostDistanceCommand(
            in_0.offset(i as isize) as *const HistogramCommand,
            out.offset(best_out as isize) as *mut HistogramCommand,
            tmp,
        );
        let mut j: size_t = 0;
        j = 0 as size_t;
        while j < num_clusters {
            let cur_bits: c_double = BrotliHistogramBitCostDistanceCommand(
                in_0.offset(i as isize) as *const HistogramCommand,
                out.offset(*clusters.offset(j as isize) as isize) as *mut HistogramCommand,
                tmp,
            ) as c_double;
            if cur_bits < best_bits {
                best_bits = cur_bits;
                best_out = *clusters.offset(j as isize);
            }
            j = j.wrapping_add(1);
        }
        *symbols.offset(i as isize) = best_out;
        i = i.wrapping_add(1);
    }
    i = 0 as size_t;
    while i < num_clusters {
        HistogramClearCommand(
            out.offset(*clusters.offset(i as isize) as isize) as *mut HistogramCommand
        );
        i = i.wrapping_add(1);
    }
    i = 0 as size_t;
    while i < in_size {
        HistogramAddHistogramCommand(
            out.offset(*symbols.offset(i as isize) as isize) as *mut HistogramCommand,
            in_0.offset(i as isize) as *const HistogramCommand,
        );
        i = i.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn BrotliHistogramReindexCommand(
    mut m: *mut MemoryManager,
    mut out: *mut HistogramCommand,
    mut symbols: *mut uint32_t,
    mut length: size_t,
) -> size_t {
    static mut kInvalidIndex: uint32_t = !(0 as c_int as uint32_t);
    let mut new_index: *mut uint32_t = if length > 0 as size_t {
        BrotliAllocate(
            m,
            length.wrapping_mul(::core::mem::size_of::<uint32_t>() as size_t),
        ) as *mut uint32_t
    } else {
        ::core::ptr::null_mut::<uint32_t>()
    };
    let mut next_index: uint32_t = 0;
    let mut tmp: *mut HistogramCommand = ::core::ptr::null_mut::<HistogramCommand>();
    let mut i: size_t = 0;
    if 0 as c_int != 0 || 0 as c_int != 0 {
        return 0 as size_t;
    }
    i = 0 as size_t;
    while i < length {
        *new_index.offset(i as isize) = kInvalidIndex;
        i = i.wrapping_add(1);
    }
    next_index = 0 as uint32_t;
    i = 0 as size_t;
    while i < length {
        if *new_index.offset(*symbols.offset(i as isize) as isize) == kInvalidIndex {
            *new_index.offset(*symbols.offset(i as isize) as isize) = next_index;
            next_index = next_index.wrapping_add(1);
        }
        i = i.wrapping_add(1);
    }
    tmp = if next_index > 0 as uint32_t {
        BrotliAllocate(
            m,
            (next_index as size_t)
                .wrapping_mul(::core::mem::size_of::<HistogramCommand>() as size_t),
        ) as *mut HistogramCommand
    } else {
        ::core::ptr::null_mut::<HistogramCommand>()
    };
    if 0 as c_int != 0 || 0 as c_int != 0 {
        return 0 as size_t;
    }
    next_index = 0 as uint32_t;
    i = 0 as size_t;
    while i < length {
        if *new_index.offset(*symbols.offset(i as isize) as isize) == next_index {
            *tmp.offset(next_index as isize) = *out.offset(*symbols.offset(i as isize) as isize);
            next_index = next_index.wrapping_add(1);
        }
        *symbols.offset(i as isize) = *new_index.offset(*symbols.offset(i as isize) as isize);
        i = i.wrapping_add(1);
    }
    BrotliFree(m, new_index as *mut c_void);
    new_index = ::core::ptr::null_mut::<uint32_t>();
    i = 0 as size_t;
    while i < next_index as size_t {
        *out.offset(i as isize) = *tmp.offset(i as isize);
        i = i.wrapping_add(1);
    }
    BrotliFree(m, tmp as *mut c_void);
    tmp = ::core::ptr::null_mut::<HistogramCommand>();
    return next_index as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn BrotliClusterHistogramsCommand(
    mut m: *mut MemoryManager,
    mut in_0: *const HistogramCommand,
    in_size: size_t,
    mut max_histograms: size_t,
    mut out: *mut HistogramCommand,
    mut out_size: *mut size_t,
    mut histogram_symbols: *mut uint32_t,
) {
    let mut cluster_size: *mut uint32_t = if in_size > 0 as size_t {
        BrotliAllocate(
            m,
            in_size.wrapping_mul(::core::mem::size_of::<uint32_t>() as size_t),
        ) as *mut uint32_t
    } else {
        ::core::ptr::null_mut::<uint32_t>()
    };
    let mut clusters: *mut uint32_t = if in_size > 0 as size_t {
        BrotliAllocate(
            m,
            in_size.wrapping_mul(::core::mem::size_of::<uint32_t>() as size_t),
        ) as *mut uint32_t
    } else {
        ::core::ptr::null_mut::<uint32_t>()
    };
    let mut num_clusters: size_t = 0 as size_t;
    let max_input_histograms: size_t = 64 as size_t;
    let mut pairs_capacity: size_t = max_input_histograms
        .wrapping_mul(max_input_histograms)
        .wrapping_div(2 as size_t);
    let mut pairs: *mut HistogramPair = if pairs_capacity.wrapping_add(1 as size_t) > 0 as size_t {
        BrotliAllocate(
            m,
            pairs_capacity
                .wrapping_add(1 as size_t)
                .wrapping_mul(::core::mem::size_of::<HistogramPair>() as size_t),
        ) as *mut HistogramPair
    } else {
        ::core::ptr::null_mut::<HistogramPair>()
    };
    let mut tmp: *mut HistogramCommand = if 1 as c_int > 0 as c_int {
        BrotliAllocate(
            m,
            (1 as size_t).wrapping_mul(::core::mem::size_of::<HistogramCommand>() as size_t),
        ) as *mut HistogramCommand
    } else {
        ::core::ptr::null_mut::<HistogramCommand>()
    };
    let mut i: size_t = 0;
    if 0 as c_int != 0
        || 0 as c_int != 0
        || 0 as c_int != 0
        || 0 as c_int != 0
        || 0 as c_int != 0
    {
        return;
    }
    i = 0 as size_t;
    while i < in_size {
        *cluster_size.offset(i as isize) = 1 as uint32_t;
        i = i.wrapping_add(1);
    }
    i = 0 as size_t;
    while i < in_size {
        *out.offset(i as isize) = *in_0.offset(i as isize);
        (*out.offset(i as isize)).bit_cost_ =
            BrotliPopulationCostCommand(in_0.offset(i as isize) as *const HistogramCommand);
        *histogram_symbols.offset(i as isize) = i as uint32_t;
        i = i.wrapping_add(1);
    }
    i = 0 as size_t;
    while i < in_size {
        let mut num_to_combine: size_t =
            brotli_min_size_t(in_size.wrapping_sub(i), max_input_histograms);
        let mut num_new_clusters: size_t = 0;
        let mut j: size_t = 0;
        j = 0 as size_t;
        while j < num_to_combine {
            *clusters.offset(num_clusters.wrapping_add(j) as isize) = i.wrapping_add(j) as uint32_t;
            j = j.wrapping_add(1);
        }
        num_new_clusters = BrotliHistogramCombineCommand(
            out,
            tmp,
            cluster_size,
            histogram_symbols.offset(i as isize) as *mut uint32_t,
            clusters.offset(num_clusters as isize) as *mut uint32_t,
            pairs,
            num_to_combine,
            num_to_combine,
            max_histograms,
            pairs_capacity,
        );
        num_clusters = (num_clusters as c_ulong)
            .wrapping_add(num_new_clusters as c_ulong) as size_t
            as size_t;
        i = (i as c_ulong).wrapping_add(max_input_histograms as c_ulong)
            as size_t as size_t;
    }
    let mut max_num_pairs: size_t = brotli_min_size_t(
        (64 as size_t).wrapping_mul(num_clusters),
        num_clusters
            .wrapping_div(2 as size_t)
            .wrapping_mul(num_clusters),
    );
    if pairs_capacity < max_num_pairs.wrapping_add(1 as size_t) {
        let mut _new_size: size_t = if pairs_capacity == 0 as size_t {
            max_num_pairs.wrapping_add(1 as size_t)
        } else {
            pairs_capacity
        };
        let mut new_array: *mut HistogramPair = ::core::ptr::null_mut::<HistogramPair>();
        while _new_size < max_num_pairs.wrapping_add(1 as size_t) {
            _new_size = (_new_size as c_ulong).wrapping_mul(2 as c_ulong)
                as size_t as size_t;
        }
        new_array = if _new_size > 0 as size_t {
            BrotliAllocate(
                m,
                _new_size.wrapping_mul(::core::mem::size_of::<HistogramPair>() as size_t),
            ) as *mut HistogramPair
        } else {
            ::core::ptr::null_mut::<HistogramPair>()
        };
        if 0 as c_int == 0
            && 0 as c_int == 0
            && pairs_capacity != 0 as size_t
        {
            memcpy(
                new_array as *mut c_void,
                pairs as *const c_void,
                pairs_capacity.wrapping_mul(::core::mem::size_of::<HistogramPair>() as size_t),
            );
        }
        BrotliFree(m, pairs as *mut c_void);
        pairs = ::core::ptr::null_mut::<HistogramPair>();
        pairs = new_array;
        pairs_capacity = _new_size;
    }
    if 0 as c_int != 0 {
        return;
    }
    num_clusters = BrotliHistogramCombineCommand(
        out,
        tmp,
        cluster_size,
        histogram_symbols,
        clusters,
        pairs,
        num_clusters,
        in_size,
        max_histograms,
        max_num_pairs,
    );
    BrotliFree(m, pairs as *mut c_void);
    pairs = ::core::ptr::null_mut::<HistogramPair>();
    BrotliFree(m, cluster_size as *mut c_void);
    cluster_size = ::core::ptr::null_mut::<uint32_t>();
    BrotliHistogramRemapCommand(
        in_0,
        in_size,
        clusters,
        num_clusters,
        out,
        tmp,
        histogram_symbols,
    );
    BrotliFree(m, tmp as *mut c_void);
    tmp = ::core::ptr::null_mut::<HistogramCommand>();
    BrotliFree(m, clusters as *mut c_void);
    clusters = ::core::ptr::null_mut::<uint32_t>();
    *out_size = BrotliHistogramReindexCommand(m, out, histogram_symbols, in_size);
    if 0 as c_int != 0 {
        return;
    }
}
#[no_mangle]
pub unsafe extern "C" fn BrotliCompareAndPushToQueueDistance(
    mut out: *const HistogramDistance,
    mut tmp: *mut HistogramDistance,
    mut cluster_size: *const uint32_t,
    mut idx1: uint32_t,
    mut idx2: uint32_t,
    mut max_num_pairs: size_t,
    mut pairs: *mut HistogramPair,
    mut num_pairs: *mut size_t,
) {
    let mut is_good_pair: c_int = 0 as c_int;
    let mut p: HistogramPair = HistogramPair {
        idx1: 0,
        idx2: 0,
        cost_combo: 0.,
        cost_diff: 0.,
    };
    p.idx2 = 0 as uint32_t;
    p.idx1 = p.idx2;
    p.cost_combo = 0 as c_int as c_double;
    p.cost_diff = p.cost_combo;
    if idx1 == idx2 {
        return;
    }
    if idx2 < idx1 {
        let mut t: uint32_t = idx2;
        idx2 = idx1;
        idx1 = t;
    }
    p.idx1 = idx1;
    p.idx2 = idx2;
    p.cost_diff = 0.5f64
        * ClusterCostDiff(
            *cluster_size.offset(idx1 as isize) as size_t,
            *cluster_size.offset(idx2 as isize) as size_t,
        );
    p.cost_diff -= (*out.offset(idx1 as isize)).bit_cost_;
    p.cost_diff -= (*out.offset(idx2 as isize)).bit_cost_;
    if (*out.offset(idx1 as isize)).total_count_ == 0 as size_t {
        p.cost_combo = (*out.offset(idx2 as isize)).bit_cost_;
        is_good_pair = 1 as c_int;
    } else if (*out.offset(idx2 as isize)).total_count_ == 0 as size_t {
        p.cost_combo = (*out.offset(idx1 as isize)).bit_cost_;
        is_good_pair = 1 as c_int;
    } else {
        let mut threshold: c_double = if *num_pairs == 0 as size_t {
            1e99f64
        } else {
            brotli_max_double(
                0.0f64,
                (*pairs.offset(0 as c_int as isize)).cost_diff,
            )
        };
        let mut cost_combo: c_double = 0.;
        *tmp = *out.offset(idx1 as isize);
        HistogramAddHistogramDistance(tmp, out.offset(idx2 as isize) as *const HistogramDistance);
        cost_combo = BrotliPopulationCostDistance(tmp);
        if cost_combo < threshold - p.cost_diff {
            p.cost_combo = cost_combo;
            is_good_pair = 1 as c_int;
        }
    }
    if is_good_pair != 0 {
        p.cost_diff += p.cost_combo;
        if *num_pairs > 0 as size_t
            && HistogramPairIsLess(
                pairs.offset(0 as c_int as isize) as *mut HistogramPair,
                &raw mut p,
            ) != 0
        {
            if *num_pairs < max_num_pairs {
                *pairs.offset(*num_pairs as isize) =
                    *pairs.offset(0 as c_int as isize);
                *num_pairs = (*num_pairs).wrapping_add(1);
            }
            *pairs.offset(0 as c_int as isize) = p;
        } else if *num_pairs < max_num_pairs {
            *pairs.offset(*num_pairs as isize) = p;
            *num_pairs = (*num_pairs).wrapping_add(1);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn BrotliHistogramCombineDistance(
    mut out: *mut HistogramDistance,
    mut tmp: *mut HistogramDistance,
    mut cluster_size: *mut uint32_t,
    mut symbols: *mut uint32_t,
    mut clusters: *mut uint32_t,
    mut pairs: *mut HistogramPair,
    mut num_clusters: size_t,
    mut symbols_size: size_t,
    mut max_clusters: size_t,
    mut max_num_pairs: size_t,
) -> size_t {
    let mut cost_diff_threshold: c_double = 0.0f64;
    let mut min_cluster_size: size_t = 1 as size_t;
    let mut num_pairs: size_t = 0 as size_t;
    let mut idx1: size_t = 0;
    idx1 = 0 as size_t;
    while idx1 < num_clusters {
        let mut idx2: size_t = 0;
        idx2 = idx1.wrapping_add(1 as size_t);
        while idx2 < num_clusters {
            BrotliCompareAndPushToQueueDistance(
                out,
                tmp,
                cluster_size,
                *clusters.offset(idx1 as isize),
                *clusters.offset(idx2 as isize),
                max_num_pairs,
                pairs.offset(0 as c_int as isize) as *mut HistogramPair,
                &raw mut num_pairs,
            );
            idx2 = idx2.wrapping_add(1);
        }
        idx1 = idx1.wrapping_add(1);
    }
    while num_clusters > min_cluster_size {
        let mut best_idx1: uint32_t = 0;
        let mut best_idx2: uint32_t = 0;
        let mut i: size_t = 0;
        if (*pairs.offset(0 as c_int as isize)).cost_diff >= cost_diff_threshold {
            cost_diff_threshold = 1e99f64;
            min_cluster_size = max_clusters;
        } else {
            best_idx1 = (*pairs.offset(0 as c_int as isize)).idx1;
            best_idx2 = (*pairs.offset(0 as c_int as isize)).idx2;
            HistogramAddHistogramDistance(
                out.offset(best_idx1 as isize) as *mut HistogramDistance,
                out.offset(best_idx2 as isize) as *mut HistogramDistance,
            );
            (*out.offset(best_idx1 as isize)).bit_cost_ =
                (*pairs.offset(0 as c_int as isize)).cost_combo;
            let ref mut fresh2 = *cluster_size.offset(best_idx1 as isize);
            *fresh2 = (*fresh2 as c_uint)
                .wrapping_add(*cluster_size.offset(best_idx2 as isize) as c_uint)
                as uint32_t as uint32_t;
            i = 0 as size_t;
            while i < symbols_size {
                if *symbols.offset(i as isize) == best_idx2 {
                    *symbols.offset(i as isize) = best_idx1;
                }
                i = i.wrapping_add(1);
            }
            i = 0 as size_t;
            while i < num_clusters {
                if *clusters.offset(i as isize) == best_idx2 {
                    memmove(
                        clusters.offset(i as isize) as *mut uint32_t as *mut c_void,
                        clusters.offset(i.wrapping_add(1 as size_t) as isize) as *mut uint32_t
                            as *const c_void,
                        num_clusters
                            .wrapping_sub(i)
                            .wrapping_sub(1 as size_t)
                            .wrapping_mul(::core::mem::size_of::<uint32_t>() as size_t),
                    );
                    break;
                } else {
                    i = i.wrapping_add(1);
                }
            }
            num_clusters = num_clusters.wrapping_sub(1);
            let mut copy_to_idx: size_t = 0 as size_t;
            i = 0 as size_t;
            while i < num_pairs {
                let mut p: *mut HistogramPair = pairs.offset(i as isize) as *mut HistogramPair;
                if !((*p).idx1 == best_idx1
                    || (*p).idx2 == best_idx1
                    || (*p).idx1 == best_idx2
                    || (*p).idx2 == best_idx2)
                {
                    if HistogramPairIsLess(
                        pairs.offset(0 as c_int as isize) as *mut HistogramPair,
                        p,
                    ) != 0
                    {
                        let mut front: HistogramPair =
                            *pairs.offset(0 as c_int as isize);
                        *pairs.offset(0 as c_int as isize) = *p;
                        *pairs.offset(copy_to_idx as isize) = front;
                    } else {
                        *pairs.offset(copy_to_idx as isize) = *p;
                    }
                    copy_to_idx = copy_to_idx.wrapping_add(1);
                }
                i = i.wrapping_add(1);
            }
            num_pairs = copy_to_idx;
            i = 0 as size_t;
            while i < num_clusters {
                BrotliCompareAndPushToQueueDistance(
                    out,
                    tmp,
                    cluster_size,
                    best_idx1,
                    *clusters.offset(i as isize),
                    max_num_pairs,
                    pairs.offset(0 as c_int as isize) as *mut HistogramPair,
                    &raw mut num_pairs,
                );
                i = i.wrapping_add(1);
            }
        }
    }
    return num_clusters;
}
#[no_mangle]
pub unsafe extern "C" fn BrotliHistogramBitCostDistanceDistance(
    mut histogram: *const HistogramDistance,
    mut candidate: *const HistogramDistance,
    mut tmp: *mut HistogramDistance,
) -> c_double {
    if (*histogram).total_count_ == 0 as size_t {
        return 0.0f64;
    } else {
        *tmp = *histogram;
        HistogramAddHistogramDistance(tmp, candidate);
        return BrotliPopulationCostDistance(tmp) - (*candidate).bit_cost_;
    };
}
#[no_mangle]
pub unsafe extern "C" fn BrotliHistogramRemapDistance(
    mut in_0: *const HistogramDistance,
    mut in_size: size_t,
    mut clusters: *const uint32_t,
    mut num_clusters: size_t,
    mut out: *mut HistogramDistance,
    mut tmp: *mut HistogramDistance,
    mut symbols: *mut uint32_t,
) {
    let mut i: size_t = 0;
    i = 0 as size_t;
    while i < in_size {
        let mut best_out: uint32_t = if i == 0 as size_t {
            *symbols.offset(0 as c_int as isize)
        } else {
            *symbols.offset(i.wrapping_sub(1 as size_t) as isize)
        };
        let mut best_bits: c_double = BrotliHistogramBitCostDistanceDistance(
            in_0.offset(i as isize) as *const HistogramDistance,
            out.offset(best_out as isize) as *mut HistogramDistance,
            tmp,
        );
        let mut j: size_t = 0;
        j = 0 as size_t;
        while j < num_clusters {
            let cur_bits: c_double = BrotliHistogramBitCostDistanceDistance(
                in_0.offset(i as isize) as *const HistogramDistance,
                out.offset(*clusters.offset(j as isize) as isize) as *mut HistogramDistance,
                tmp,
            ) as c_double;
            if cur_bits < best_bits {
                best_bits = cur_bits;
                best_out = *clusters.offset(j as isize);
            }
            j = j.wrapping_add(1);
        }
        *symbols.offset(i as isize) = best_out;
        i = i.wrapping_add(1);
    }
    i = 0 as size_t;
    while i < num_clusters {
        HistogramClearDistance(
            out.offset(*clusters.offset(i as isize) as isize) as *mut HistogramDistance
        );
        i = i.wrapping_add(1);
    }
    i = 0 as size_t;
    while i < in_size {
        HistogramAddHistogramDistance(
            out.offset(*symbols.offset(i as isize) as isize) as *mut HistogramDistance,
            in_0.offset(i as isize) as *const HistogramDistance,
        );
        i = i.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn BrotliHistogramReindexDistance(
    mut m: *mut MemoryManager,
    mut out: *mut HistogramDistance,
    mut symbols: *mut uint32_t,
    mut length: size_t,
) -> size_t {
    static mut kInvalidIndex: uint32_t = !(0 as c_int as uint32_t);
    let mut new_index: *mut uint32_t = if length > 0 as size_t {
        BrotliAllocate(
            m,
            length.wrapping_mul(::core::mem::size_of::<uint32_t>() as size_t),
        ) as *mut uint32_t
    } else {
        ::core::ptr::null_mut::<uint32_t>()
    };
    let mut next_index: uint32_t = 0;
    let mut tmp: *mut HistogramDistance = ::core::ptr::null_mut::<HistogramDistance>();
    let mut i: size_t = 0;
    if 0 as c_int != 0 || 0 as c_int != 0 {
        return 0 as size_t;
    }
    i = 0 as size_t;
    while i < length {
        *new_index.offset(i as isize) = kInvalidIndex;
        i = i.wrapping_add(1);
    }
    next_index = 0 as uint32_t;
    i = 0 as size_t;
    while i < length {
        if *new_index.offset(*symbols.offset(i as isize) as isize) == kInvalidIndex {
            *new_index.offset(*symbols.offset(i as isize) as isize) = next_index;
            next_index = next_index.wrapping_add(1);
        }
        i = i.wrapping_add(1);
    }
    tmp = if next_index > 0 as uint32_t {
        BrotliAllocate(
            m,
            (next_index as size_t)
                .wrapping_mul(::core::mem::size_of::<HistogramDistance>() as size_t),
        ) as *mut HistogramDistance
    } else {
        ::core::ptr::null_mut::<HistogramDistance>()
    };
    if 0 as c_int != 0 || 0 as c_int != 0 {
        return 0 as size_t;
    }
    next_index = 0 as uint32_t;
    i = 0 as size_t;
    while i < length {
        if *new_index.offset(*symbols.offset(i as isize) as isize) == next_index {
            *tmp.offset(next_index as isize) = *out.offset(*symbols.offset(i as isize) as isize);
            next_index = next_index.wrapping_add(1);
        }
        *symbols.offset(i as isize) = *new_index.offset(*symbols.offset(i as isize) as isize);
        i = i.wrapping_add(1);
    }
    BrotliFree(m, new_index as *mut c_void);
    new_index = ::core::ptr::null_mut::<uint32_t>();
    i = 0 as size_t;
    while i < next_index as size_t {
        *out.offset(i as isize) = *tmp.offset(i as isize);
        i = i.wrapping_add(1);
    }
    BrotliFree(m, tmp as *mut c_void);
    tmp = ::core::ptr::null_mut::<HistogramDistance>();
    return next_index as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn BrotliClusterHistogramsDistance(
    mut m: *mut MemoryManager,
    mut in_0: *const HistogramDistance,
    in_size: size_t,
    mut max_histograms: size_t,
    mut out: *mut HistogramDistance,
    mut out_size: *mut size_t,
    mut histogram_symbols: *mut uint32_t,
) {
    let mut cluster_size: *mut uint32_t = if in_size > 0 as size_t {
        BrotliAllocate(
            m,
            in_size.wrapping_mul(::core::mem::size_of::<uint32_t>() as size_t),
        ) as *mut uint32_t
    } else {
        ::core::ptr::null_mut::<uint32_t>()
    };
    let mut clusters: *mut uint32_t = if in_size > 0 as size_t {
        BrotliAllocate(
            m,
            in_size.wrapping_mul(::core::mem::size_of::<uint32_t>() as size_t),
        ) as *mut uint32_t
    } else {
        ::core::ptr::null_mut::<uint32_t>()
    };
    let mut num_clusters: size_t = 0 as size_t;
    let max_input_histograms: size_t = 64 as size_t;
    let mut pairs_capacity: size_t = max_input_histograms
        .wrapping_mul(max_input_histograms)
        .wrapping_div(2 as size_t);
    let mut pairs: *mut HistogramPair = if pairs_capacity.wrapping_add(1 as size_t) > 0 as size_t {
        BrotliAllocate(
            m,
            pairs_capacity
                .wrapping_add(1 as size_t)
                .wrapping_mul(::core::mem::size_of::<HistogramPair>() as size_t),
        ) as *mut HistogramPair
    } else {
        ::core::ptr::null_mut::<HistogramPair>()
    };
    let mut tmp: *mut HistogramDistance = if 1 as c_int > 0 as c_int {
        BrotliAllocate(
            m,
            (1 as size_t).wrapping_mul(::core::mem::size_of::<HistogramDistance>() as size_t),
        ) as *mut HistogramDistance
    } else {
        ::core::ptr::null_mut::<HistogramDistance>()
    };
    let mut i: size_t = 0;
    if 0 as c_int != 0
        || 0 as c_int != 0
        || 0 as c_int != 0
        || 0 as c_int != 0
        || 0 as c_int != 0
    {
        return;
    }
    i = 0 as size_t;
    while i < in_size {
        *cluster_size.offset(i as isize) = 1 as uint32_t;
        i = i.wrapping_add(1);
    }
    i = 0 as size_t;
    while i < in_size {
        *out.offset(i as isize) = *in_0.offset(i as isize);
        (*out.offset(i as isize)).bit_cost_ =
            BrotliPopulationCostDistance(in_0.offset(i as isize) as *const HistogramDistance);
        *histogram_symbols.offset(i as isize) = i as uint32_t;
        i = i.wrapping_add(1);
    }
    i = 0 as size_t;
    while i < in_size {
        let mut num_to_combine: size_t =
            brotli_min_size_t(in_size.wrapping_sub(i), max_input_histograms);
        let mut num_new_clusters: size_t = 0;
        let mut j: size_t = 0;
        j = 0 as size_t;
        while j < num_to_combine {
            *clusters.offset(num_clusters.wrapping_add(j) as isize) = i.wrapping_add(j) as uint32_t;
            j = j.wrapping_add(1);
        }
        num_new_clusters = BrotliHistogramCombineDistance(
            out,
            tmp,
            cluster_size,
            histogram_symbols.offset(i as isize) as *mut uint32_t,
            clusters.offset(num_clusters as isize) as *mut uint32_t,
            pairs,
            num_to_combine,
            num_to_combine,
            max_histograms,
            pairs_capacity,
        );
        num_clusters = (num_clusters as c_ulong)
            .wrapping_add(num_new_clusters as c_ulong) as size_t
            as size_t;
        i = (i as c_ulong).wrapping_add(max_input_histograms as c_ulong)
            as size_t as size_t;
    }
    let mut max_num_pairs: size_t = brotli_min_size_t(
        (64 as size_t).wrapping_mul(num_clusters),
        num_clusters
            .wrapping_div(2 as size_t)
            .wrapping_mul(num_clusters),
    );
    if pairs_capacity < max_num_pairs.wrapping_add(1 as size_t) {
        let mut _new_size: size_t = if pairs_capacity == 0 as size_t {
            max_num_pairs.wrapping_add(1 as size_t)
        } else {
            pairs_capacity
        };
        let mut new_array: *mut HistogramPair = ::core::ptr::null_mut::<HistogramPair>();
        while _new_size < max_num_pairs.wrapping_add(1 as size_t) {
            _new_size = (_new_size as c_ulong).wrapping_mul(2 as c_ulong)
                as size_t as size_t;
        }
        new_array = if _new_size > 0 as size_t {
            BrotliAllocate(
                m,
                _new_size.wrapping_mul(::core::mem::size_of::<HistogramPair>() as size_t),
            ) as *mut HistogramPair
        } else {
            ::core::ptr::null_mut::<HistogramPair>()
        };
        if 0 as c_int == 0
            && 0 as c_int == 0
            && pairs_capacity != 0 as size_t
        {
            memcpy(
                new_array as *mut c_void,
                pairs as *const c_void,
                pairs_capacity.wrapping_mul(::core::mem::size_of::<HistogramPair>() as size_t),
            );
        }
        BrotliFree(m, pairs as *mut c_void);
        pairs = ::core::ptr::null_mut::<HistogramPair>();
        pairs = new_array;
        pairs_capacity = _new_size;
    }
    if 0 as c_int != 0 {
        return;
    }
    num_clusters = BrotliHistogramCombineDistance(
        out,
        tmp,
        cluster_size,
        histogram_symbols,
        clusters,
        pairs,
        num_clusters,
        in_size,
        max_histograms,
        max_num_pairs,
    );
    BrotliFree(m, pairs as *mut c_void);
    pairs = ::core::ptr::null_mut::<HistogramPair>();
    BrotliFree(m, cluster_size as *mut c_void);
    cluster_size = ::core::ptr::null_mut::<uint32_t>();
    BrotliHistogramRemapDistance(
        in_0,
        in_size,
        clusters,
        num_clusters,
        out,
        tmp,
        histogram_symbols,
    );
    BrotliFree(m, tmp as *mut c_void);
    tmp = ::core::ptr::null_mut::<HistogramDistance>();
    BrotliFree(m, clusters as *mut c_void);
    clusters = ::core::ptr::null_mut::<uint32_t>();
    *out_size = BrotliHistogramReindexDistance(m, out, histogram_symbols, in_size);
    if 0 as c_int != 0 {
        return;
    }
}
#[inline(always)]
unsafe extern "C" fn HistogramPairIsLess(
    mut p1: *const HistogramPair,
    mut p2: *const HistogramPair,
) -> c_int {
    if (*p1).cost_diff != (*p2).cost_diff {
        return if (*p1).cost_diff > (*p2).cost_diff {
            BROTLI_TRUE
        } else {
            BROTLI_FALSE
        };
    }
    return if (*p1).idx2.wrapping_sub((*p1).idx1) > (*p2).idx2.wrapping_sub((*p2).idx1) {
        BROTLI_TRUE
    } else {
        BROTLI_FALSE
    };
}
#[inline(always)]
unsafe extern "C" fn ClusterCostDiff(
    mut size_a: size_t,
    mut size_b: size_t,
) -> c_double {
    let mut size_c: size_t = size_a.wrapping_add(size_b);
    return size_a as c_double * FastLog2(size_a)
        + size_b as c_double * FastLog2(size_b)
        - size_c as c_double * FastLog2(size_c);
}
