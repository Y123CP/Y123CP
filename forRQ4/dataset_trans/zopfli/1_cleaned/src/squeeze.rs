use core::ffi::*;
pub use crate::src::c_inlined_fns::*;
pub use crate::src::ffi::*;
pub use crate::src::c_consts::*;
pub use crate::src::c_structs::*;
pub use crate::src::c_types::*;
pub use crate::src::c_extern_types::*;
extern "C" {
    fn ZopfliCopyLZ77Store(source: *const ZopfliLZ77Store, dest: *mut ZopfliLZ77Store);
    fn ZopfliStoreLitLenDist(
        length: c_ushort,
        dist: c_ushort,
        pos: size_t,
        store: *mut ZopfliLZ77Store,
    );
    fn ZopfliFindLongestMatch(
        s: *mut ZopfliBlockState,
        h: *const ZopfliHash,
        array: *const c_uchar,
        pos: size_t,
        size: size_t,
        limit: size_t,
        sublen: *mut c_ushort,
        distance: *mut c_ushort,
        length: *mut c_ushort,
    );
    fn ZopfliVerifyLenDist(
        data: *const c_uchar,
        datasize: size_t,
        pos: size_t,
        dist: c_ushort,
        length: c_ushort,
    );
    fn ZopfliCalculateBlockSize(
        lz77: *const ZopfliLZ77Store,
        lstart: size_t,
        lend: size_t,
        btype: c_int,
    ) -> c_double;
    fn ZopfliCalculateEntropy(
        count: *const size_t,
        n: size_t,
        bitlengths: *mut c_double,
    );
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct SymbolStats {
    pub litlens: [size_t; 288],
    pub dists: [size_t; 32],
    pub ll_symbols: [c_double; 288],
    pub d_symbols: [c_double; 32],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RanState {
    pub m_w: c_uint,
    pub m_z: c_uint,
}

pub type CostModelFun = unsafe extern "C" fn(
    c_uint,
    c_uint,
    *mut c_void,
) -> c_double;
unsafe extern "C" fn InitStats(mut stats: *mut SymbolStats) {
    memset(
        &raw mut (*stats).litlens as *mut size_t as *mut c_void,
        0 as c_int,
        (ZOPFLI_NUM_LL as size_t).wrapping_mul(::core::mem::size_of::<size_t>() as size_t),
    );
    memset(
        &raw mut (*stats).dists as *mut size_t as *mut c_void,
        0 as c_int,
        (ZOPFLI_NUM_D as size_t).wrapping_mul(::core::mem::size_of::<size_t>() as size_t),
    );
    memset(
        &raw mut (*stats).ll_symbols as *mut c_double as *mut c_void,
        0 as c_int,
        (ZOPFLI_NUM_LL as size_t)
            .wrapping_mul(::core::mem::size_of::<c_double>() as size_t),
    );
    memset(
        &raw mut (*stats).d_symbols as *mut c_double as *mut c_void,
        0 as c_int,
        (ZOPFLI_NUM_D as size_t)
            .wrapping_mul(::core::mem::size_of::<c_double>() as size_t),
    );
}
unsafe extern "C" fn CopyStats(mut source: *mut SymbolStats, mut dest: *mut SymbolStats) {
    memcpy(
        &raw mut (*dest).litlens as *mut size_t as *mut c_void,
        &raw mut (*source).litlens as *mut size_t as *const c_void,
        (ZOPFLI_NUM_LL as size_t).wrapping_mul(::core::mem::size_of::<size_t>() as size_t),
    );
    memcpy(
        &raw mut (*dest).dists as *mut size_t as *mut c_void,
        &raw mut (*source).dists as *mut size_t as *const c_void,
        (ZOPFLI_NUM_D as size_t).wrapping_mul(::core::mem::size_of::<size_t>() as size_t),
    );
    memcpy(
        &raw mut (*dest).ll_symbols as *mut c_double as *mut c_void,
        &raw mut (*source).ll_symbols as *mut c_double as *const c_void,
        (ZOPFLI_NUM_LL as size_t)
            .wrapping_mul(::core::mem::size_of::<c_double>() as size_t),
    );
    memcpy(
        &raw mut (*dest).d_symbols as *mut c_double as *mut c_void,
        &raw mut (*source).d_symbols as *mut c_double as *const c_void,
        (ZOPFLI_NUM_D as size_t)
            .wrapping_mul(::core::mem::size_of::<c_double>() as size_t),
    );
}
unsafe extern "C" fn AddWeighedStatFreqs(
    mut stats1: *const SymbolStats,
    mut w1: c_double,
    mut stats2: *const SymbolStats,
    mut w2: c_double,
    mut result: *mut SymbolStats,
) {
    let mut i: size_t = 0;
    i = 0 as size_t;
    while i < ZOPFLI_NUM_LL as size_t {
        (*result).litlens[i as usize] = ((*stats1).litlens[i as usize] as c_double
            * w1
            + (*stats2).litlens[i as usize] as c_double * w2)
            as size_t;
        i = i.wrapping_add(1);
    }
    i = 0 as size_t;
    while i < ZOPFLI_NUM_D as size_t {
        (*result).dists[i as usize] = ((*stats1).dists[i as usize] as c_double * w1
            + (*stats2).dists[i as usize] as c_double * w2)
            as size_t;
        i = i.wrapping_add(1);
    }
    (*result).litlens[256 as c_int as usize] = 1 as size_t;
}
unsafe extern "C" fn InitRanState(mut state: *mut RanState) {
    (*state).m_w = 1 as c_uint;
    (*state).m_z = 2 as c_uint;
}
unsafe extern "C" fn Ran(mut state: *mut RanState) -> c_uint {
    (*state).m_z = (36969 as c_uint)
        .wrapping_mul((*state).m_z & 65535 as c_uint)
        .wrapping_add((*state).m_z >> 16 as c_int);
    (*state).m_w = (18000 as c_uint)
        .wrapping_mul((*state).m_w & 65535 as c_uint)
        .wrapping_add((*state).m_w >> 16 as c_int);
    return ((*state).m_z << 16 as c_int).wrapping_add((*state).m_w);
}
unsafe extern "C" fn RandomizeFreqs(
    mut state: *mut RanState,
    mut freqs: *mut size_t,
    mut n: c_int,
) {
    let mut i: c_int = 0;
    i = 0 as c_int;
    while i < n {
        if (Ran(state) >> 4 as c_int).wrapping_rem(3 as c_uint)
            == 0 as c_uint
        {
            *freqs.offset(i as isize) =
                *freqs.offset(Ran(state).wrapping_rem(n as c_uint) as isize);
        }
        i += 1;
    }
}
unsafe extern "C" fn RandomizeStatFreqs(mut state: *mut RanState, mut stats: *mut SymbolStats) {
    RandomizeFreqs(
        state,
        &raw mut (*stats).litlens as *mut size_t,
        ZOPFLI_NUM_LL,
    );
    RandomizeFreqs(state, &raw mut (*stats).dists as *mut size_t, ZOPFLI_NUM_D);
    (*stats).litlens[256 as c_int as usize] = 1 as size_t;
}
unsafe extern "C" fn ClearStatFreqs(mut stats: *mut SymbolStats) {
    let mut i: size_t = 0;
    i = 0 as size_t;
    while i < ZOPFLI_NUM_LL as size_t {
        (*stats).litlens[i as usize] = 0 as size_t;
        i = i.wrapping_add(1);
    }
    i = 0 as size_t;
    while i < ZOPFLI_NUM_D as size_t {
        (*stats).dists[i as usize] = 0 as size_t;
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn GetCostFixed(
    mut litlen: c_uint,
    mut dist: c_uint,
    mut unused: *mut c_void,
) -> c_double {
    if dist == 0 as c_uint {
        if litlen <= 143 as c_uint {
            return 8 as c_int as c_double;
        } else {
            return 9 as c_int as c_double;
        }
    } else {
        let mut dbits: c_int = ZopfliGetDistExtraBits(dist as c_int);
        let mut lbits: c_int = ZopfliGetLengthExtraBits(litlen as c_int);
        let mut lsym: c_int = ZopfliGetLengthSymbol(litlen as c_int);
        let mut cost: c_int = 0 as c_int;
        if lsym <= 279 as c_int {
            cost += 7 as c_int;
        } else {
            cost += 8 as c_int;
        }
        cost += 5 as c_int;
        return (cost + dbits + lbits) as c_double;
    };
}
unsafe extern "C" fn GetCostStat(
    mut litlen: c_uint,
    mut dist: c_uint,
    mut context: *mut c_void,
) -> c_double {
    let mut stats: *mut SymbolStats = context as *mut SymbolStats;
    if dist == 0 as c_uint {
        return (*stats).ll_symbols[litlen as usize];
    } else {
        let mut lsym: c_int = ZopfliGetLengthSymbol(litlen as c_int);
        let mut lbits: c_int = ZopfliGetLengthExtraBits(litlen as c_int);
        let mut dsym: c_int = ZopfliGetDistSymbol(dist as c_int);
        let mut dbits: c_int = ZopfliGetDistExtraBits(dist as c_int);
        return (lbits + dbits) as c_double
            + (*stats).ll_symbols[lsym as usize]
            + (*stats).d_symbols[dsym as usize];
    };
}
unsafe extern "C" fn GetCostModelMinCost(
    mut costmodel: Option<CostModelFun>,
    mut costcontext: *mut c_void,
) -> c_double {
    let mut mincost: c_double = 0.;
    let mut bestlength: c_int = 0 as c_int;
    let mut bestdist: c_int = 0 as c_int;
    let mut i: c_int = 0;
    static mut dsymbols: [c_int; 30] = [
        1 as c_int,
        2 as c_int,
        3 as c_int,
        4 as c_int,
        5 as c_int,
        7 as c_int,
        9 as c_int,
        13 as c_int,
        17 as c_int,
        25 as c_int,
        33 as c_int,
        49 as c_int,
        65 as c_int,
        97 as c_int,
        129 as c_int,
        193 as c_int,
        257 as c_int,
        385 as c_int,
        513 as c_int,
        769 as c_int,
        1025 as c_int,
        1537 as c_int,
        2049 as c_int,
        3073 as c_int,
        4097 as c_int,
        6145 as c_int,
        8193 as c_int,
        12289 as c_int,
        16385 as c_int,
        24577 as c_int,
    ];
    mincost = ZOPFLI_LARGE_FLOAT;
    i = 3 as c_int;
    while i < 259 as c_int {
        let mut c: c_double = costmodel.expect("non-null function pointer")(
            i as c_uint,
            1 as c_uint,
            costcontext,
        );
        if c < mincost {
            bestlength = i;
            mincost = c;
        }
        i += 1;
    }
    mincost = ZOPFLI_LARGE_FLOAT;
    i = 0 as c_int;
    while i < 30 as c_int {
        let mut c_0: c_double = costmodel.expect("non-null function pointer")(
            3 as c_uint,
            dsymbols[i as usize] as c_uint,
            costcontext,
        );
        if c_0 < mincost {
            bestdist = dsymbols[i as usize];
            mincost = c_0;
        }
        i += 1;
    }
    return costmodel.expect("non-null function pointer")(
        bestlength as c_uint,
        bestdist as c_uint,
        costcontext,
    );
}
unsafe extern "C" fn zopfli_min(mut a: size_t, mut b: size_t) -> size_t {
    return if a < b { a } else { b };
}
unsafe extern "C" fn GetBestLengths(
    mut s: *mut ZopfliBlockState,
    mut in_0: *const c_uchar,
    mut instart: size_t,
    mut inend: size_t,
    mut costmodel: Option<CostModelFun>,
    mut costcontext: *mut c_void,
    mut length_array: *mut c_ushort,
    mut h: *mut ZopfliHash,
    mut costs: *mut c_float,
) -> c_double {
    let mut blocksize: size_t = inend.wrapping_sub(instart);
    let mut i: size_t = 0 as size_t;
    let mut k: size_t = 0;
    let mut kend: size_t = 0;
    let mut leng: c_ushort = 0;
    let mut dist: c_ushort = 0;
    let mut sublen: [c_ushort; 259] = [0; 259];
    let mut windowstart: size_t = if instart > ZOPFLI_WINDOW_SIZE as size_t {
        instart.wrapping_sub(ZOPFLI_WINDOW_SIZE as size_t)
    } else {
        0 as size_t
    };
    let mut result: c_double = 0.;
    let mut mincost: c_double = GetCostModelMinCost(costmodel, costcontext);
    let mut mincostaddcostj: c_double = 0.;
    if instart == inend {
        return 0 as c_int as c_double;
    }
    ZopfliResetHash(ZOPFLI_WINDOW_SIZE as size_t, h);
    ZopfliWarmupHash(in_0, windowstart, inend, h);
    i = windowstart;
    while i < instart {
        ZopfliUpdateHash(in_0, i, inend, h);
        i = i.wrapping_add(1);
    }
    i = 1 as size_t;
    while i < blocksize.wrapping_add(1 as size_t) {
        *costs.offset(i as isize) = ZOPFLI_LARGE_FLOAT as c_float;
        i = i.wrapping_add(1);
    }
    *costs.offset(0 as c_int as isize) =
        0 as c_int as c_float;
    *length_array.offset(0 as c_int as isize) = 0 as c_ushort;
    i = instart;
    while i < inend {
        let mut j: size_t = i.wrapping_sub(instart);
        ZopfliUpdateHash(in_0, i, inend, h);
        if *(*h)
            .same
            .offset((i & ZOPFLI_WINDOW_MASK as size_t) as isize) as c_int
            > ZOPFLI_MAX_MATCH * 2 as c_int
            && i > instart
                .wrapping_add(ZOPFLI_MAX_MATCH as size_t)
                .wrapping_add(1 as size_t)
            && i.wrapping_add((ZOPFLI_MAX_MATCH * 2 as c_int) as size_t)
                .wrapping_add(1 as size_t)
                < inend
            && *(*h).same.offset(
                (i.wrapping_sub(ZOPFLI_MAX_MATCH as size_t) & ZOPFLI_WINDOW_MASK as size_t)
                    as isize,
            ) as c_int
                > ZOPFLI_MAX_MATCH
        {
            let mut symbolcost: c_double = costmodel
                .expect("non-null function pointer")(
                ZOPFLI_MAX_MATCH as c_uint,
                1 as c_uint,
                costcontext,
            );
            k = 0 as size_t;
            while k < ZOPFLI_MAX_MATCH as size_t {
                *costs.offset(j.wrapping_add(ZOPFLI_MAX_MATCH as size_t) as isize) =
                    (*costs.offset(j as isize) as c_double + symbolcost)
                        as c_float;
                *length_array.offset(j.wrapping_add(ZOPFLI_MAX_MATCH as size_t) as isize) =
                    ZOPFLI_MAX_MATCH as c_ushort;
                i = i.wrapping_add(1);
                j = j.wrapping_add(1);
                ZopfliUpdateHash(in_0, i, inend, h);
                k = k.wrapping_add(1);
            }
        }
        ZopfliFindLongestMatch(
            s,
            h,
            in_0,
            i,
            inend,
            ZOPFLI_MAX_MATCH as size_t,
            &raw mut sublen as *mut c_ushort,
            &raw mut dist,
            &raw mut leng,
        );
        if i.wrapping_add(1 as size_t) <= inend {
            let mut newCost: c_double = costmodel.expect("non-null function pointer")(
                *in_0.offset(i as isize) as c_uint,
                0 as c_uint,
                costcontext,
            ) + *costs.offset(j as isize)
                as c_double;
            if newCost
                < *costs.offset(j.wrapping_add(1 as size_t) as isize) as c_double
            {
                *costs.offset(j.wrapping_add(1 as size_t) as isize) =
                    newCost as c_float;
                *length_array.offset(j.wrapping_add(1 as size_t) as isize) =
                    1 as c_ushort;
            }
        }
        kend = zopfli_min(leng as size_t, inend.wrapping_sub(i));
        mincostaddcostj = mincost + *costs.offset(j as isize) as c_double;
        k = 3 as size_t;
        while k <= kend {
            let mut newCost_0: c_double = 0.;
            if !(*costs.offset(j.wrapping_add(k) as isize) as c_double
                <= mincostaddcostj)
            {
                newCost_0 = costmodel.expect("non-null function pointer")(
                    k as c_uint,
                    sublen[k as usize] as c_uint,
                    costcontext,
                ) + *costs.offset(j as isize) as c_double;
                if newCost_0 < *costs.offset(j.wrapping_add(k) as isize) as c_double {
                    *costs.offset(j.wrapping_add(k) as isize) = newCost_0 as c_float;
                    *length_array.offset(j.wrapping_add(k) as isize) = k as c_ushort;
                }
            }
            k = k.wrapping_add(1);
        }
        i = i.wrapping_add(1);
    }
    result = *costs.offset(blocksize as isize) as c_double;
    return result;
}
unsafe extern "C" fn TraceBackwards(
    mut size: size_t,
    mut length_array: *const c_ushort,
    mut path: *mut *mut c_ushort,
    mut pathsize: *mut size_t,
) {
    let mut index: size_t = size;
    if size == 0 as size_t {
        return;
    }
    loop {
        if *pathsize & (*pathsize).wrapping_sub(1 as size_t) == 0 {
            *path = (if *pathsize == 0 as size_t {
                malloc(::core::mem::size_of::<c_ushort>() as size_t)
            } else {
                realloc(
                    *path as *mut c_void,
                    (*pathsize)
                        .wrapping_mul(2 as size_t)
                        .wrapping_mul(::core::mem::size_of::<c_ushort>() as size_t),
                )
            }) as *mut c_ushort;
        }
        *(*path).offset(*pathsize as isize) = *length_array.offset(index as isize);
        *pathsize = (*pathsize).wrapping_add(1);
        index = index.wrapping_sub(*length_array.offset(index as isize) as size_t);
        if index == 0 as size_t {
            break;
        }
    }
    index = 0 as size_t;
    while index < (*pathsize).wrapping_div(2 as size_t) {
        let mut temp: c_ushort = *(*path).offset(index as isize);
        *(*path).offset(index as isize) =
            *(*path).offset((*pathsize).wrapping_sub(index).wrapping_sub(1 as size_t) as isize);
        *(*path).offset((*pathsize).wrapping_sub(index).wrapping_sub(1 as size_t) as isize) = temp;
        index = index.wrapping_add(1);
    }
}
unsafe extern "C" fn FollowPath(
    mut s: *mut ZopfliBlockState,
    mut in_0: *const c_uchar,
    mut instart: size_t,
    mut inend: size_t,
    mut path: *mut c_ushort,
    mut pathsize: size_t,
    mut store: *mut ZopfliLZ77Store,
    mut h: *mut ZopfliHash,
) {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut pos: size_t = 0 as size_t;
    let mut windowstart: size_t = if instart > ZOPFLI_WINDOW_SIZE as size_t {
        instart.wrapping_sub(ZOPFLI_WINDOW_SIZE as size_t)
    } else {
        0 as size_t
    };
    let mut total_length_test: size_t = 0 as size_t;
    if instart == inend {
        return;
    }
    ZopfliResetHash(ZOPFLI_WINDOW_SIZE as size_t, h);
    ZopfliWarmupHash(in_0, windowstart, inend, h);
    i = windowstart;
    while i < instart {
        ZopfliUpdateHash(in_0, i, inend, h);
        i = i.wrapping_add(1);
    }
    pos = instart;
    i = 0 as size_t;
    while i < pathsize {
        let mut length: c_ushort = *path.offset(i as isize);
        let mut dummy_length: c_ushort = 0;
        let mut dist: c_ushort = 0;
        ZopfliUpdateHash(in_0, pos, inend, h);
        if length as c_int >= ZOPFLI_MIN_MATCH {
            ZopfliFindLongestMatch(
                s,
                h,
                in_0,
                pos,
                inend,
                length as size_t,
                ::core::ptr::null_mut::<c_ushort>(),
                &raw mut dist,
                &raw mut dummy_length,
            );
            ZopfliVerifyLenDist(in_0, inend, pos, dist, length);
            ZopfliStoreLitLenDist(length, dist, pos, store);
            total_length_test = total_length_test.wrapping_add(length as size_t);
        } else {
            length = 1 as c_ushort;
            ZopfliStoreLitLenDist(
                *in_0.offset(pos as isize) as c_ushort,
                0 as c_ushort,
                pos,
                store,
            );
            total_length_test = total_length_test.wrapping_add(1);
        }
        j = 1 as size_t;
        while j < length as size_t {
            ZopfliUpdateHash(in_0, pos.wrapping_add(j), inend, h);
            j = j.wrapping_add(1);
        }
        pos = pos.wrapping_add(length as size_t);
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn CalculateStatistics(mut stats: *mut SymbolStats) {
    ZopfliCalculateEntropy(
        &raw mut (*stats).litlens as *mut size_t,
        ZOPFLI_NUM_LL as size_t,
        &raw mut (*stats).ll_symbols as *mut c_double,
    );
    ZopfliCalculateEntropy(
        &raw mut (*stats).dists as *mut size_t,
        ZOPFLI_NUM_D as size_t,
        &raw mut (*stats).d_symbols as *mut c_double,
    );
}
unsafe extern "C" fn GetStatistics(mut store: *const ZopfliLZ77Store, mut stats: *mut SymbolStats) {
    let mut i: size_t = 0;
    i = 0 as size_t;
    while i < (*store).size {
        if *(*store).dists.offset(i as isize) as c_int == 0 as c_int {
            (*stats).litlens[*(*store).litlens.offset(i as isize) as usize] =
                (*stats).litlens[*(*store).litlens.offset(i as isize) as usize].wrapping_add(1);
        } else {
            (*stats).litlens[ZopfliGetLengthSymbol(
                *(*store).litlens.offset(i as isize) as c_int
            ) as usize] = (*stats).litlens[ZopfliGetLengthSymbol(
                *(*store).litlens.offset(i as isize) as c_int,
            ) as usize]
                .wrapping_add(1);
            (*stats).dists[ZopfliGetDistSymbol(
                *(*store).dists.offset(i as isize) as c_int
            ) as usize] = (*stats).dists[ZopfliGetDistSymbol(
                *(*store).dists.offset(i as isize) as c_int
            ) as usize]
                .wrapping_add(1);
        }
        i = i.wrapping_add(1);
    }
    (*stats).litlens[256 as c_int as usize] = 1 as size_t;
    CalculateStatistics(stats);
}
unsafe extern "C" fn LZ77OptimalRun(
    mut s: *mut ZopfliBlockState,
    mut in_0: *const c_uchar,
    mut instart: size_t,
    mut inend: size_t,
    mut path: *mut *mut c_ushort,
    mut pathsize: *mut size_t,
    mut length_array: *mut c_ushort,
    mut costmodel: Option<CostModelFun>,
    mut costcontext: *mut c_void,
    mut store: *mut ZopfliLZ77Store,
    mut h: *mut ZopfliHash,
    mut costs: *mut c_float,
) -> c_double {
    let mut cost: c_double = GetBestLengths(
        s,
        in_0,
        instart,
        inend,
        costmodel,
        costcontext,
        length_array,
        h,
        costs,
    );
    free(*path as *mut c_void);
    *path = ::core::ptr::null_mut::<c_ushort>();
    *pathsize = 0 as size_t;
    TraceBackwards(inend.wrapping_sub(instart), length_array, path, pathsize);
    FollowPath(s, in_0, instart, inend, *path, *pathsize, store, h);
    return cost;
}
#[no_mangle]
pub unsafe extern "C" fn ZopfliLZ77Optimal(
    mut s: *mut ZopfliBlockState,
    mut in_0: *const c_uchar,
    mut instart: size_t,
    mut inend: size_t,
    mut numiterations: c_int,
    mut store: *mut ZopfliLZ77Store,
) {
    let mut blocksize: size_t = inend.wrapping_sub(instart);
    let mut length_array: *mut c_ushort = malloc(
        (::core::mem::size_of::<c_ushort>() as size_t)
            .wrapping_mul(blocksize.wrapping_add(1 as size_t)),
    ) as *mut c_ushort;
    let mut path: *mut c_ushort = ::core::ptr::null_mut::<c_ushort>();
    let mut pathsize: size_t = 0 as size_t;
    let mut currentstore: ZopfliLZ77Store = ZopfliLZ77Store {
        litlens: ::core::ptr::null_mut::<c_ushort>(),
        dists: ::core::ptr::null_mut::<c_ushort>(),
        size: 0,
        data: ::core::ptr::null::<c_uchar>(),
        pos: ::core::ptr::null_mut::<size_t>(),
        ll_symbol: ::core::ptr::null_mut::<c_ushort>(),
        d_symbol: ::core::ptr::null_mut::<c_ushort>(),
        ll_counts: ::core::ptr::null_mut::<size_t>(),
        d_counts: ::core::ptr::null_mut::<size_t>(),
    };
    let mut hash: ZopfliHash = ZopfliHash {
        head: ::core::ptr::null_mut::<c_int>(),
        prev: ::core::ptr::null_mut::<c_ushort>(),
        hashval: ::core::ptr::null_mut::<c_int>(),
        val: 0,
        head2: ::core::ptr::null_mut::<c_int>(),
        prev2: ::core::ptr::null_mut::<c_ushort>(),
        hashval2: ::core::ptr::null_mut::<c_int>(),
        val2: 0,
        same: ::core::ptr::null_mut::<c_ushort>(),
    };
    let mut h: *mut ZopfliHash = &raw mut hash;
    let mut stats: SymbolStats = SymbolStats {
        litlens: [0; 288],
        dists: [0; 32],
        ll_symbols: [0.; 288],
        d_symbols: [0.; 32],
    };
    let mut beststats: SymbolStats = SymbolStats {
        litlens: [0; 288],
        dists: [0; 32],
        ll_symbols: [0.; 288],
        d_symbols: [0.; 32],
    };
    let mut laststats: SymbolStats = SymbolStats {
        litlens: [0; 288],
        dists: [0; 32],
        ll_symbols: [0.; 288],
        d_symbols: [0.; 32],
    };
    let mut i: c_int = 0;
    let mut costs: *mut c_float = malloc(
        (::core::mem::size_of::<c_float>() as size_t)
            .wrapping_mul(blocksize.wrapping_add(1 as size_t)),
    ) as *mut c_float;
    let mut cost: c_double = 0.;
    let mut bestcost: c_double = ZOPFLI_LARGE_FLOAT;
    let mut lastcost: c_double = 0 as c_int as c_double;
    let mut ran_state: RanState = RanState { m_w: 0, m_z: 0 };
    let mut lastrandomstep: c_int = -(1 as c_int);
    if costs.is_null() {
        exit(-(1 as c_int));
    }
    if length_array.is_null() {
        exit(-(1 as c_int));
    }
    InitRanState(&raw mut ran_state);
    InitStats(&raw mut stats);
    ZopfliInitLZ77Store(in_0, &raw mut currentstore);
    ZopfliAllocHash(ZOPFLI_WINDOW_SIZE as size_t, h);
    ZopfliLZ77Greedy(s, in_0, instart, inend, &raw mut currentstore, h);
    GetStatistics(&raw mut currentstore, &raw mut stats);
    i = 0 as c_int;
    while i < numiterations {
        ZopfliCleanLZ77Store(&raw mut currentstore);
        ZopfliInitLZ77Store(in_0, &raw mut currentstore);
        LZ77OptimalRun(
            s,
            in_0,
            instart,
            inend,
            &raw mut path,
            &raw mut pathsize,
            length_array,
            Some(
                GetCostStat
                    as unsafe extern "C" fn(
                        c_uint,
                        c_uint,
                        *mut c_void,
                    ) -> c_double,
            ),
            &raw mut stats as *mut c_void,
            &raw mut currentstore,
            h,
            costs,
        );
        cost = ZopfliCalculateBlockSize(
            &raw mut currentstore,
            0 as size_t,
            currentstore.size,
            2 as c_int,
        );
        if (*(*s).options).verbose_more != 0 || (*(*s).options).verbose != 0 && cost < bestcost {
            fprintf(
                stderr,
                b"Iteration %d: %d bit\n\0" as *const u8 as *const c_char,
                i,
                cost as c_int,
            );
        }
        if cost < bestcost {
            ZopfliCopyLZ77Store(&raw mut currentstore, store);
            CopyStats(&raw mut stats, &raw mut beststats);
            bestcost = cost;
        }
        CopyStats(&raw mut stats, &raw mut laststats);
        ClearStatFreqs(&raw mut stats);
        GetStatistics(&raw mut currentstore, &raw mut stats);
        if lastrandomstep != -(1 as c_int) {
            AddWeighedStatFreqs(
                &raw mut stats,
                1.0f64,
                &raw mut laststats,
                0.5f64,
                &raw mut stats,
            );
            CalculateStatistics(&raw mut stats);
        }
        if i > 5 as c_int && cost == lastcost {
            CopyStats(&raw mut beststats, &raw mut stats);
            RandomizeStatFreqs(&raw mut ran_state, &raw mut stats);
            CalculateStatistics(&raw mut stats);
            lastrandomstep = i;
        }
        lastcost = cost;
        i += 1;
    }
    free(length_array as *mut c_void);
    free(path as *mut c_void);
    free(costs as *mut c_void);
    ZopfliCleanLZ77Store(&raw mut currentstore);
    ZopfliCleanHash(h);
}
#[no_mangle]
pub unsafe extern "C" fn ZopfliLZ77OptimalFixed(
    mut s: *mut ZopfliBlockState,
    mut in_0: *const c_uchar,
    mut instart: size_t,
    mut inend: size_t,
    mut store: *mut ZopfliLZ77Store,
) {
    let mut blocksize: size_t = inend.wrapping_sub(instart);
    let mut length_array: *mut c_ushort = malloc(
        (::core::mem::size_of::<c_ushort>() as size_t)
            .wrapping_mul(blocksize.wrapping_add(1 as size_t)),
    ) as *mut c_ushort;
    let mut path: *mut c_ushort = ::core::ptr::null_mut::<c_ushort>();
    let mut pathsize: size_t = 0 as size_t;
    let mut hash: ZopfliHash = ZopfliHash {
        head: ::core::ptr::null_mut::<c_int>(),
        prev: ::core::ptr::null_mut::<c_ushort>(),
        hashval: ::core::ptr::null_mut::<c_int>(),
        val: 0,
        head2: ::core::ptr::null_mut::<c_int>(),
        prev2: ::core::ptr::null_mut::<c_ushort>(),
        hashval2: ::core::ptr::null_mut::<c_int>(),
        val2: 0,
        same: ::core::ptr::null_mut::<c_ushort>(),
    };
    let mut h: *mut ZopfliHash = &raw mut hash;
    let mut costs: *mut c_float = malloc(
        (::core::mem::size_of::<c_float>() as size_t)
            .wrapping_mul(blocksize.wrapping_add(1 as size_t)),
    ) as *mut c_float;
    if costs.is_null() {
        exit(-(1 as c_int));
    }
    if length_array.is_null() {
        exit(-(1 as c_int));
    }
    ZopfliAllocHash(ZOPFLI_WINDOW_SIZE as size_t, h);
    (*s).blockstart = instart;
    (*s).blockend = inend;
    LZ77OptimalRun(
        s,
        in_0,
        instart,
        inend,
        &raw mut path,
        &raw mut pathsize,
        length_array,
        Some(
            GetCostFixed
                as unsafe extern "C" fn(
                    c_uint,
                    c_uint,
                    *mut c_void,
                ) -> c_double,
        ),
        ::core::ptr::null_mut::<c_void>(),
        store,
        h,
        costs,
    );
    free(length_array as *mut c_void);
    free(path as *mut c_void);
    free(costs as *mut c_void);
    ZopfliCleanHash(h);
}

