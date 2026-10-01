extern "C" {
    static kBrotliLog2Table: [::core::ffi::c_double; 256];
    fn log2(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
}
pub type size_t = usize;
pub type __uint32_t = u32;
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HistogramLiteral {
    pub data_: [uint32_t; 256],
    pub total_count_: size_t,
    pub bit_cost_: ::core::ffi::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HistogramCommand {
    pub data_: [uint32_t; 704],
    pub total_count_: size_t,
    pub bit_cost_: ::core::ffi::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HistogramDistance {
    pub data_: [uint32_t; 544],
    pub total_count_: size_t,
    pub bit_cost_: ::core::ffi::c_double,
}
#[inline(always)]
unsafe extern "C" fn brotli_max_uint32_t(mut a: uint32_t, mut b: uint32_t) -> uint32_t {
    return if a > b { a } else { b };
}
pub const BROTLI_NUM_LITERAL_SYMBOLS: ::core::ffi::c_int = 256 as ::core::ffi::c_int;
pub const BROTLI_NUM_COMMAND_SYMBOLS: ::core::ffi::c_int = 704 as ::core::ffi::c_int;
pub const BROTLI_REPEAT_ZERO_CODE_LENGTH: ::core::ffi::c_int = 17 as ::core::ffi::c_int;
pub const BROTLI_CODE_LENGTH_CODES: ::core::ffi::c_int =
    BROTLI_REPEAT_ZERO_CODE_LENGTH + 1 as ::core::ffi::c_int;
pub const BROTLI_LOG2_TABLE_SIZE: ::core::ffi::c_int = 256 as ::core::ffi::c_int;
#[inline(always)]
unsafe extern "C" fn FastLog2(mut v: size_t) -> ::core::ffi::c_double {
    if v < BROTLI_LOG2_TABLE_SIZE as size_t {
        return kBrotliLog2Table[v as usize];
    }
    return log2(v as ::core::ffi::c_double);
}
#[inline(always)]
unsafe extern "C" fn HistogramDataSizeLiteral() -> size_t {
    return DATA_SIZE as size_t;
}
#[inline(always)]
unsafe extern "C" fn HistogramDataSizeCommand() -> size_t {
    return DATA_SIZE_0 as size_t;
}
#[inline(always)]
unsafe extern "C" fn HistogramDataSizeDistance() -> size_t {
    return DATA_SIZE_1 as size_t;
}
pub const BROTLI_NUM_HISTOGRAM_DISTANCE_SYMBOLS: ::core::ffi::c_int = 544 as ::core::ffi::c_int;
pub const DATA_SIZE: ::core::ffi::c_int = BROTLI_NUM_LITERAL_SYMBOLS;
pub const DATA_SIZE_0: ::core::ffi::c_int = BROTLI_NUM_COMMAND_SYMBOLS;
pub const DATA_SIZE_1: ::core::ffi::c_int = BROTLI_NUM_HISTOGRAM_DISTANCE_SYMBOLS;
#[no_mangle]
pub unsafe extern "C" fn BrotliBitsEntropy(
    mut population: *const uint32_t,
    mut size: size_t,
) -> ::core::ffi::c_double {
    let mut current_block: u64;
    let mut sum: size_t = 0 as size_t;
    let mut retval: ::core::ffi::c_double = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    let mut population_end: *const uint32_t = population.offset(size as isize);
    let mut p: size_t = 0;
    if size & 1 as size_t != 0 {
        current_block = 16935294885289056218;
    } else {
        current_block = 12517898123489920830;
    }
    loop {
        match current_block {
            12517898123489920830 => {
                if !(population < population_end) {
                    break;
                }
                let fresh0 = population;
                population = population.offset(1);
                p = *fresh0 as size_t;
                sum = (sum as ::core::ffi::c_ulong).wrapping_add(p as ::core::ffi::c_ulong)
                    as size_t as size_t;
                retval -= p as ::core::ffi::c_double * FastLog2(p);
                current_block = 16935294885289056218;
            }
            _ => {
                let fresh1 = population;
                population = population.offset(1);
                p = *fresh1 as size_t;
                sum = (sum as ::core::ffi::c_ulong).wrapping_add(p as ::core::ffi::c_ulong)
                    as size_t as size_t;
                retval -= p as ::core::ffi::c_double * FastLog2(p);
                current_block = 12517898123489920830;
            }
        }
    }
    if sum != 0 {
        retval += sum as ::core::ffi::c_double * FastLog2(sum);
    }
    if retval < sum as ::core::ffi::c_double {
        retval = sum as ::core::ffi::c_double;
    }
    return retval;
}
#[no_mangle]
pub unsafe extern "C" fn BrotliPopulationCostLiteral(
    mut histogram: *const HistogramLiteral,
) -> ::core::ffi::c_double {
    static mut kOneSymbolHistogramCost: ::core::ffi::c_double =
        12 as ::core::ffi::c_int as ::core::ffi::c_double;
    static mut kTwoSymbolHistogramCost: ::core::ffi::c_double =
        20 as ::core::ffi::c_int as ::core::ffi::c_double;
    static mut kThreeSymbolHistogramCost: ::core::ffi::c_double =
        28 as ::core::ffi::c_int as ::core::ffi::c_double;
    static mut kFourSymbolHistogramCost: ::core::ffi::c_double =
        37 as ::core::ffi::c_int as ::core::ffi::c_double;
    let data_size: size_t = HistogramDataSizeLiteral() as size_t;
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut s: [size_t; 5] = [0; 5];
    let mut bits: ::core::ffi::c_double = 0.0f64;
    let mut i: size_t = 0;
    if (*histogram).total_count_ == 0 as size_t {
        return kOneSymbolHistogramCost;
    }
    i = 0 as size_t;
    while i < data_size {
        if (*histogram).data_[i as usize] > 0 as uint32_t {
            s[count as usize] = i;
            count += 1;
            if count > 4 as ::core::ffi::c_int {
                break;
            }
        }
        i = i.wrapping_add(1);
    }
    if count == 1 as ::core::ffi::c_int {
        return kOneSymbolHistogramCost;
    }
    if count == 2 as ::core::ffi::c_int {
        return kTwoSymbolHistogramCost + (*histogram).total_count_ as ::core::ffi::c_double;
    }
    if count == 3 as ::core::ffi::c_int {
        let histo0: uint32_t = (*histogram).data_[s[0 as ::core::ffi::c_int as usize] as usize];
        let histo1: uint32_t = (*histogram).data_[s[1 as ::core::ffi::c_int as usize] as usize];
        let histo2: uint32_t = (*histogram).data_[s[2 as ::core::ffi::c_int as usize] as usize];
        let histomax: uint32_t =
            brotli_max_uint32_t(histo0, brotli_max_uint32_t(histo1, histo2)) as uint32_t;
        return kThreeSymbolHistogramCost
            + (2 as uint32_t).wrapping_mul(histo0.wrapping_add(histo1).wrapping_add(histo2))
                as ::core::ffi::c_double
            - histomax as ::core::ffi::c_double;
    }
    if count == 4 as ::core::ffi::c_int {
        let mut histo: [uint32_t; 4] = [0; 4];
        let mut h23: uint32_t = 0;
        let mut histomax_0: uint32_t = 0;
        i = 0 as size_t;
        while i < 4 as size_t {
            histo[i as usize] = (*histogram).data_[s[i as usize] as usize];
            i = i.wrapping_add(1);
        }
        i = 0 as size_t;
        while i < 4 as size_t {
            let mut j: size_t = 0;
            j = i.wrapping_add(1 as size_t);
            while j < 4 as size_t {
                if histo[j as usize] > histo[i as usize] {
                    let mut __brotli_swap_tmp: uint32_t = histo[j as usize];
                    histo[j as usize] = histo[i as usize];
                    histo[i as usize] = __brotli_swap_tmp;
                }
                j = j.wrapping_add(1);
            }
            i = i.wrapping_add(1);
        }
        h23 = histo[2 as ::core::ffi::c_int as usize]
            .wrapping_add(histo[3 as ::core::ffi::c_int as usize]);
        histomax_0 = brotli_max_uint32_t(h23, histo[0 as ::core::ffi::c_int as usize]);
        return kFourSymbolHistogramCost
            + (3 as uint32_t).wrapping_mul(h23) as ::core::ffi::c_double
            + (2 as uint32_t).wrapping_mul(
                histo[0 as ::core::ffi::c_int as usize]
                    .wrapping_add(histo[1 as ::core::ffi::c_int as usize]),
            ) as ::core::ffi::c_double
            - histomax_0 as ::core::ffi::c_double;
    }
    let mut max_depth: size_t = 1 as size_t;
    let mut depth_histo: [uint32_t; 18] = [
        0 as ::core::ffi::c_int as uint32_t,
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
    let log2total: ::core::ffi::c_double =
        FastLog2((*histogram).total_count_) as ::core::ffi::c_double;
    i = 0 as size_t;
    while i < data_size {
        if (*histogram).data_[i as usize] > 0 as uint32_t {
            let mut log2p: ::core::ffi::c_double =
                log2total - FastLog2((*histogram).data_[i as usize] as size_t);
            let mut depth: size_t = (log2p + 0.5f64) as size_t;
            bits += (*histogram).data_[i as usize] as ::core::ffi::c_double * log2p;
            if depth > 15 as size_t {
                depth = 15 as size_t;
            }
            if depth > max_depth {
                max_depth = depth;
            }
            depth_histo[depth as usize] = depth_histo[depth as usize].wrapping_add(1);
            i = i.wrapping_add(1);
        } else {
            let mut reps: uint32_t = 1 as uint32_t;
            let mut k: size_t = 0;
            k = i.wrapping_add(1 as size_t);
            while k < data_size && (*histogram).data_[k as usize] == 0 as uint32_t {
                reps = reps.wrapping_add(1);
                k = k.wrapping_add(1);
            }
            i = (i as ::core::ffi::c_ulong).wrapping_add(reps as ::core::ffi::c_ulong) as size_t
                as size_t;
            if i == data_size {
                break;
            }
            if reps < 3 as uint32_t {
                depth_histo[0 as ::core::ffi::c_int as usize] =
                    (depth_histo[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_uint)
                        .wrapping_add(reps as ::core::ffi::c_uint) as uint32_t
                        as uint32_t;
            } else {
                reps = (reps as ::core::ffi::c_uint).wrapping_sub(2 as ::core::ffi::c_uint)
                    as uint32_t as uint32_t;
                while reps > 0 as uint32_t {
                    depth_histo[BROTLI_REPEAT_ZERO_CODE_LENGTH as usize] =
                        depth_histo[BROTLI_REPEAT_ZERO_CODE_LENGTH as usize].wrapping_add(1);
                    bits += 3 as ::core::ffi::c_int as ::core::ffi::c_double;
                    reps >>= 3 as ::core::ffi::c_int;
                }
            }
        }
    }
    bits +=
        (18 as size_t).wrapping_add((2 as size_t).wrapping_mul(max_depth)) as ::core::ffi::c_double;
    bits += BrotliBitsEntropy(
        &raw mut depth_histo as *mut uint32_t,
        BROTLI_CODE_LENGTH_CODES as size_t,
    );
    return bits;
}
#[no_mangle]
pub unsafe extern "C" fn BrotliPopulationCostCommand(
    mut histogram: *const HistogramCommand,
) -> ::core::ffi::c_double {
    static mut kOneSymbolHistogramCost: ::core::ffi::c_double =
        12 as ::core::ffi::c_int as ::core::ffi::c_double;
    static mut kTwoSymbolHistogramCost: ::core::ffi::c_double =
        20 as ::core::ffi::c_int as ::core::ffi::c_double;
    static mut kThreeSymbolHistogramCost: ::core::ffi::c_double =
        28 as ::core::ffi::c_int as ::core::ffi::c_double;
    static mut kFourSymbolHistogramCost: ::core::ffi::c_double =
        37 as ::core::ffi::c_int as ::core::ffi::c_double;
    let data_size: size_t = HistogramDataSizeCommand() as size_t;
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut s: [size_t; 5] = [0; 5];
    let mut bits: ::core::ffi::c_double = 0.0f64;
    let mut i: size_t = 0;
    if (*histogram).total_count_ == 0 as size_t {
        return kOneSymbolHistogramCost;
    }
    i = 0 as size_t;
    while i < data_size {
        if (*histogram).data_[i as usize] > 0 as uint32_t {
            s[count as usize] = i;
            count += 1;
            if count > 4 as ::core::ffi::c_int {
                break;
            }
        }
        i = i.wrapping_add(1);
    }
    if count == 1 as ::core::ffi::c_int {
        return kOneSymbolHistogramCost;
    }
    if count == 2 as ::core::ffi::c_int {
        return kTwoSymbolHistogramCost + (*histogram).total_count_ as ::core::ffi::c_double;
    }
    if count == 3 as ::core::ffi::c_int {
        let histo0: uint32_t = (*histogram).data_[s[0 as ::core::ffi::c_int as usize] as usize];
        let histo1: uint32_t = (*histogram).data_[s[1 as ::core::ffi::c_int as usize] as usize];
        let histo2: uint32_t = (*histogram).data_[s[2 as ::core::ffi::c_int as usize] as usize];
        let histomax: uint32_t =
            brotli_max_uint32_t(histo0, brotli_max_uint32_t(histo1, histo2)) as uint32_t;
        return kThreeSymbolHistogramCost
            + (2 as uint32_t).wrapping_mul(histo0.wrapping_add(histo1).wrapping_add(histo2))
                as ::core::ffi::c_double
            - histomax as ::core::ffi::c_double;
    }
    if count == 4 as ::core::ffi::c_int {
        let mut histo: [uint32_t; 4] = [0; 4];
        let mut h23: uint32_t = 0;
        let mut histomax_0: uint32_t = 0;
        i = 0 as size_t;
        while i < 4 as size_t {
            histo[i as usize] = (*histogram).data_[s[i as usize] as usize];
            i = i.wrapping_add(1);
        }
        i = 0 as size_t;
        while i < 4 as size_t {
            let mut j: size_t = 0;
            j = i.wrapping_add(1 as size_t);
            while j < 4 as size_t {
                if histo[j as usize] > histo[i as usize] {
                    let mut __brotli_swap_tmp: uint32_t = histo[j as usize];
                    histo[j as usize] = histo[i as usize];
                    histo[i as usize] = __brotli_swap_tmp;
                }
                j = j.wrapping_add(1);
            }
            i = i.wrapping_add(1);
        }
        h23 = histo[2 as ::core::ffi::c_int as usize]
            .wrapping_add(histo[3 as ::core::ffi::c_int as usize]);
        histomax_0 = brotli_max_uint32_t(h23, histo[0 as ::core::ffi::c_int as usize]);
        return kFourSymbolHistogramCost
            + (3 as uint32_t).wrapping_mul(h23) as ::core::ffi::c_double
            + (2 as uint32_t).wrapping_mul(
                histo[0 as ::core::ffi::c_int as usize]
                    .wrapping_add(histo[1 as ::core::ffi::c_int as usize]),
            ) as ::core::ffi::c_double
            - histomax_0 as ::core::ffi::c_double;
    }
    let mut max_depth: size_t = 1 as size_t;
    let mut depth_histo: [uint32_t; 18] = [
        0 as ::core::ffi::c_int as uint32_t,
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
    let log2total: ::core::ffi::c_double =
        FastLog2((*histogram).total_count_) as ::core::ffi::c_double;
    i = 0 as size_t;
    while i < data_size {
        if (*histogram).data_[i as usize] > 0 as uint32_t {
            let mut log2p: ::core::ffi::c_double =
                log2total - FastLog2((*histogram).data_[i as usize] as size_t);
            let mut depth: size_t = (log2p + 0.5f64) as size_t;
            bits += (*histogram).data_[i as usize] as ::core::ffi::c_double * log2p;
            if depth > 15 as size_t {
                depth = 15 as size_t;
            }
            if depth > max_depth {
                max_depth = depth;
            }
            depth_histo[depth as usize] = depth_histo[depth as usize].wrapping_add(1);
            i = i.wrapping_add(1);
        } else {
            let mut reps: uint32_t = 1 as uint32_t;
            let mut k: size_t = 0;
            k = i.wrapping_add(1 as size_t);
            while k < data_size && (*histogram).data_[k as usize] == 0 as uint32_t {
                reps = reps.wrapping_add(1);
                k = k.wrapping_add(1);
            }
            i = (i as ::core::ffi::c_ulong).wrapping_add(reps as ::core::ffi::c_ulong) as size_t
                as size_t;
            if i == data_size {
                break;
            }
            if reps < 3 as uint32_t {
                depth_histo[0 as ::core::ffi::c_int as usize] =
                    (depth_histo[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_uint)
                        .wrapping_add(reps as ::core::ffi::c_uint) as uint32_t
                        as uint32_t;
            } else {
                reps = (reps as ::core::ffi::c_uint).wrapping_sub(2 as ::core::ffi::c_uint)
                    as uint32_t as uint32_t;
                while reps > 0 as uint32_t {
                    depth_histo[BROTLI_REPEAT_ZERO_CODE_LENGTH as usize] =
                        depth_histo[BROTLI_REPEAT_ZERO_CODE_LENGTH as usize].wrapping_add(1);
                    bits += 3 as ::core::ffi::c_int as ::core::ffi::c_double;
                    reps >>= 3 as ::core::ffi::c_int;
                }
            }
        }
    }
    bits +=
        (18 as size_t).wrapping_add((2 as size_t).wrapping_mul(max_depth)) as ::core::ffi::c_double;
    bits += BrotliBitsEntropy(
        &raw mut depth_histo as *mut uint32_t,
        BROTLI_CODE_LENGTH_CODES as size_t,
    );
    return bits;
}
#[no_mangle]
pub unsafe extern "C" fn BrotliPopulationCostDistance(
    mut histogram: *const HistogramDistance,
) -> ::core::ffi::c_double {
    static mut kOneSymbolHistogramCost: ::core::ffi::c_double =
        12 as ::core::ffi::c_int as ::core::ffi::c_double;
    static mut kTwoSymbolHistogramCost: ::core::ffi::c_double =
        20 as ::core::ffi::c_int as ::core::ffi::c_double;
    static mut kThreeSymbolHistogramCost: ::core::ffi::c_double =
        28 as ::core::ffi::c_int as ::core::ffi::c_double;
    static mut kFourSymbolHistogramCost: ::core::ffi::c_double =
        37 as ::core::ffi::c_int as ::core::ffi::c_double;
    let data_size: size_t = HistogramDataSizeDistance() as size_t;
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut s: [size_t; 5] = [0; 5];
    let mut bits: ::core::ffi::c_double = 0.0f64;
    let mut i: size_t = 0;
    if (*histogram).total_count_ == 0 as size_t {
        return kOneSymbolHistogramCost;
    }
    i = 0 as size_t;
    while i < data_size {
        if (*histogram).data_[i as usize] > 0 as uint32_t {
            s[count as usize] = i;
            count += 1;
            if count > 4 as ::core::ffi::c_int {
                break;
            }
        }
        i = i.wrapping_add(1);
    }
    if count == 1 as ::core::ffi::c_int {
        return kOneSymbolHistogramCost;
    }
    if count == 2 as ::core::ffi::c_int {
        return kTwoSymbolHistogramCost + (*histogram).total_count_ as ::core::ffi::c_double;
    }
    if count == 3 as ::core::ffi::c_int {
        let histo0: uint32_t = (*histogram).data_[s[0 as ::core::ffi::c_int as usize] as usize];
        let histo1: uint32_t = (*histogram).data_[s[1 as ::core::ffi::c_int as usize] as usize];
        let histo2: uint32_t = (*histogram).data_[s[2 as ::core::ffi::c_int as usize] as usize];
        let histomax: uint32_t =
            brotli_max_uint32_t(histo0, brotli_max_uint32_t(histo1, histo2)) as uint32_t;
        return kThreeSymbolHistogramCost
            + (2 as uint32_t).wrapping_mul(histo0.wrapping_add(histo1).wrapping_add(histo2))
                as ::core::ffi::c_double
            - histomax as ::core::ffi::c_double;
    }
    if count == 4 as ::core::ffi::c_int {
        let mut histo: [uint32_t; 4] = [0; 4];
        let mut h23: uint32_t = 0;
        let mut histomax_0: uint32_t = 0;
        i = 0 as size_t;
        while i < 4 as size_t {
            histo[i as usize] = (*histogram).data_[s[i as usize] as usize];
            i = i.wrapping_add(1);
        }
        i = 0 as size_t;
        while i < 4 as size_t {
            let mut j: size_t = 0;
            j = i.wrapping_add(1 as size_t);
            while j < 4 as size_t {
                if histo[j as usize] > histo[i as usize] {
                    let mut __brotli_swap_tmp: uint32_t = histo[j as usize];
                    histo[j as usize] = histo[i as usize];
                    histo[i as usize] = __brotli_swap_tmp;
                }
                j = j.wrapping_add(1);
            }
            i = i.wrapping_add(1);
        }
        h23 = histo[2 as ::core::ffi::c_int as usize]
            .wrapping_add(histo[3 as ::core::ffi::c_int as usize]);
        histomax_0 = brotli_max_uint32_t(h23, histo[0 as ::core::ffi::c_int as usize]);
        return kFourSymbolHistogramCost
            + (3 as uint32_t).wrapping_mul(h23) as ::core::ffi::c_double
            + (2 as uint32_t).wrapping_mul(
                histo[0 as ::core::ffi::c_int as usize]
                    .wrapping_add(histo[1 as ::core::ffi::c_int as usize]),
            ) as ::core::ffi::c_double
            - histomax_0 as ::core::ffi::c_double;
    }
    let mut max_depth: size_t = 1 as size_t;
    let mut depth_histo: [uint32_t; 18] = [
        0 as ::core::ffi::c_int as uint32_t,
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
    let log2total: ::core::ffi::c_double =
        FastLog2((*histogram).total_count_) as ::core::ffi::c_double;
    i = 0 as size_t;
    while i < data_size {
        if (*histogram).data_[i as usize] > 0 as uint32_t {
            let mut log2p: ::core::ffi::c_double =
                log2total - FastLog2((*histogram).data_[i as usize] as size_t);
            let mut depth: size_t = (log2p + 0.5f64) as size_t;
            bits += (*histogram).data_[i as usize] as ::core::ffi::c_double * log2p;
            if depth > 15 as size_t {
                depth = 15 as size_t;
            }
            if depth > max_depth {
                max_depth = depth;
            }
            depth_histo[depth as usize] = depth_histo[depth as usize].wrapping_add(1);
            i = i.wrapping_add(1);
        } else {
            let mut reps: uint32_t = 1 as uint32_t;
            let mut k: size_t = 0;
            k = i.wrapping_add(1 as size_t);
            while k < data_size && (*histogram).data_[k as usize] == 0 as uint32_t {
                reps = reps.wrapping_add(1);
                k = k.wrapping_add(1);
            }
            i = (i as ::core::ffi::c_ulong).wrapping_add(reps as ::core::ffi::c_ulong) as size_t
                as size_t;
            if i == data_size {
                break;
            }
            if reps < 3 as uint32_t {
                depth_histo[0 as ::core::ffi::c_int as usize] =
                    (depth_histo[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_uint)
                        .wrapping_add(reps as ::core::ffi::c_uint) as uint32_t
                        as uint32_t;
            } else {
                reps = (reps as ::core::ffi::c_uint).wrapping_sub(2 as ::core::ffi::c_uint)
                    as uint32_t as uint32_t;
                while reps > 0 as uint32_t {
                    depth_histo[BROTLI_REPEAT_ZERO_CODE_LENGTH as usize] =
                        depth_histo[BROTLI_REPEAT_ZERO_CODE_LENGTH as usize].wrapping_add(1);
                    bits += 3 as ::core::ffi::c_int as ::core::ffi::c_double;
                    reps >>= 3 as ::core::ffi::c_int;
                }
            }
        }
    }
    bits +=
        (18 as size_t).wrapping_add((2 as size_t).wrapping_mul(max_depth)) as ::core::ffi::c_double;
    bits += BrotliBitsEntropy(
        &raw mut depth_histo as *mut uint32_t,
        BROTLI_CODE_LENGTH_CODES as size_t,
    );
    return bits;
}
