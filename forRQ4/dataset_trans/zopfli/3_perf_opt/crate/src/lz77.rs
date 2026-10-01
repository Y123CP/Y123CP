use core::ffi::*;
use crate::src::cache::ZopfliCacheToSublen;
use crate::src::cache::ZopfliCleanCache;
use crate::src::c_inlined_fns::ZopfliGetDistSymbol;
use crate::src::c_inlined_fns::ZopfliGetLengthSymbol;
use crate::src::cache::ZopfliInitCache;
use crate::src::cache::ZopfliMaxCachedSublen;
use crate::src::hash::ZopfliResetHash;
use crate::src::cache::ZopfliSublenToCache;
use crate::src::hash::ZopfliUpdateHash;
use crate::src::hash::ZopfliWarmupHash;
pub use crate::src::c_inlined_fns::*;
pub use crate::src::ffi::*;
pub use crate::src::c_consts::*;
pub use crate::src::c_structs::*;
pub use crate::src::c_types::*;

pub const ZOPFLI_MAX_CHAIN_HITS: c_int = 8192 as c_int;
#[inline]
pub unsafe fn ZopfliInitLZ77Store(
    mut data: *const c_uchar,
    mut store: *mut ZopfliLZ77Store,
) {
    let store_view: &mut ZopfliLZ77Store = unsafe { &mut *store };
    store_view.size = 0 as size_t;
    store_view.litlens = ::core::ptr::null_mut::<c_ushort>();
    store_view.dists = ::core::ptr::null_mut::<c_ushort>();
    store_view.pos = ::core::ptr::null_mut::<size_t>();
    store_view.data = data;
    store_view.ll_symbol = ::core::ptr::null_mut::<c_ushort>();
    store_view.d_symbol = ::core::ptr::null_mut::<c_ushort>();
    store_view.ll_counts = ::core::ptr::null_mut::<size_t>();
    store_view.d_counts = ::core::ptr::null_mut::<size_t>();
}
#[inline]
pub unsafe fn ZopfliCleanLZ77Store(mut store: *mut ZopfliLZ77Store) {
    let store_view: &ZopfliLZ77Store = unsafe { &*store };
    free(store_view.litlens as *mut c_void);
    free(store_view.dists as *mut c_void);
    free(store_view.pos as *mut c_void);
    free(store_view.ll_symbol as *mut c_void);
    free(store_view.d_symbol as *mut c_void);
    free(store_view.ll_counts as *mut c_void);
    free(store_view.d_counts as *mut c_void);
}
fn CeilDiv(mut a: size_t, mut b: size_t) -> size_t { {
    return a.wrapping_add(b).wrapping_sub(1 as size_t).wrapping_div(b);
} }
#[inline]
pub unsafe fn ZopfliCopyLZ77Store(
    mut source: *const ZopfliLZ77Store,
    mut dest: *mut ZopfliLZ77Store,
) {
    let mut i: size_t = 0;
    let mut llsize: size_t =
        (ZOPFLI_NUM_LL as size_t).wrapping_mul(CeilDiv((*source).size, ZOPFLI_NUM_LL as size_t));
    let mut dsize: size_t =
        (ZOPFLI_NUM_D as size_t).wrapping_mul(CeilDiv((*source).size, ZOPFLI_NUM_D as size_t));
    ZopfliCleanLZ77Store(dest);
    ZopfliInitLZ77Store((*source).data, dest);
    (*dest).litlens = malloc(
        (::core::mem::size_of::<c_ushort>() as size_t).wrapping_mul((*source).size),
    ) as *mut c_ushort;
    (*dest).dists = malloc(
        (::core::mem::size_of::<c_ushort>() as size_t).wrapping_mul((*source).size),
    ) as *mut c_ushort;
    (*dest).pos = malloc((::core::mem::size_of::<size_t>() as size_t).wrapping_mul((*source).size))
        as *mut size_t;
    (*dest).ll_symbol = malloc(
        (::core::mem::size_of::<c_ushort>() as size_t).wrapping_mul((*source).size),
    ) as *mut c_ushort;
    (*dest).d_symbol = malloc(
        (::core::mem::size_of::<c_ushort>() as size_t).wrapping_mul((*source).size),
    ) as *mut c_ushort;
    (*dest).ll_counts =
        malloc((::core::mem::size_of::<size_t>() as size_t).wrapping_mul(llsize)) as *mut size_t;
    (*dest).d_counts =
        malloc((::core::mem::size_of::<size_t>() as size_t).wrapping_mul(dsize)) as *mut size_t;
    if (*dest).litlens.is_null() || (*dest).dists.is_null() {
        exit(-(1 as c_int));
    }
    if (*dest).pos.is_null() {
        exit(-(1 as c_int));
    }
    if (*dest).ll_symbol.is_null() || (*dest).d_symbol.is_null() {
        exit(-(1 as c_int));
    }
    if (*dest).ll_counts.is_null() || (*dest).d_counts.is_null() {
        exit(-(1 as c_int));
    }
    (*dest).size = (*source).size;
    i = 0 as size_t;
    while i < (*source).size {
        *(*dest).litlens.offset(i as isize) = *(*source).litlens.offset(i as isize);
        *(*dest).dists.offset(i as isize) = *(*source).dists.offset(i as isize);
        *(*dest).pos.offset(i as isize) = *(*source).pos.offset(i as isize);
        *(*dest).ll_symbol.offset(i as isize) = *(*source).ll_symbol.offset(i as isize);
        *(*dest).d_symbol.offset(i as isize) = *(*source).d_symbol.offset(i as isize);
        i = i.wrapping_add(1);
    }
    i = 0 as size_t;
    while i < llsize {
        *(*dest).ll_counts.offset(i as isize) = *(*source).ll_counts.offset(i as isize);
        i = i.wrapping_add(1);
    }
    i = 0 as size_t;
    while i < dsize {
        *(*dest).d_counts.offset(i as isize) = *(*source).d_counts.offset(i as isize);
        i = i.wrapping_add(1);
    }
}
#[inline]
pub unsafe fn ZopfliStoreLitLenDist(
    mut length: c_ushort,
    mut dist: c_ushort,
    mut pos: size_t,
    mut store: *mut ZopfliLZ77Store,
) {
    let store_view: &mut ZopfliLZ77Store = unsafe { &mut *store };
    let mut i: size_t = 0;
    let mut origsize: size_t = store_view.size;
    let mut llstart: size_t =
        (ZOPFLI_NUM_LL as size_t).wrapping_mul(origsize.wrapping_div(ZOPFLI_NUM_LL as size_t));
    let mut dstart: size_t =
        (ZOPFLI_NUM_D as size_t).wrapping_mul(origsize.wrapping_div(ZOPFLI_NUM_D as size_t));
    if origsize.wrapping_rem(ZOPFLI_NUM_LL as size_t) == 0 as size_t {
        let mut llsize: size_t = origsize;
        i = 0 as size_t;
        while i < ZOPFLI_NUM_LL as size_t {
            if llsize & llsize.wrapping_sub(1 as size_t) == 0 {
                store_view.ll_counts = (if llsize == 0 as size_t {
                    malloc(::core::mem::size_of::<size_t>() as size_t)
                } else {
                    realloc(
                        store_view.ll_counts as *mut c_void,
                        llsize
                            .wrapping_mul(2 as size_t)
                            .wrapping_mul(::core::mem::size_of::<size_t>() as size_t),
                    )
                }) as *mut size_t;
            }
            *store_view.ll_counts.offset(llsize as isize) = if origsize == 0 as size_t {
                0 as size_t
            } else {
                *store_view
                    .ll_counts
                    .offset(origsize.wrapping_sub(288 as size_t).wrapping_add(i) as isize)
            };
            llsize = llsize.wrapping_add(1);
            i = i.wrapping_add(1);
        }
    }
    if origsize.wrapping_rem(ZOPFLI_NUM_D as size_t) == 0 as size_t {
        let mut dsize: size_t = origsize;
        i = 0 as size_t;
        while i < ZOPFLI_NUM_D as size_t {
            if dsize & dsize.wrapping_sub(1 as size_t) == 0 {
                store_view.d_counts = (if dsize == 0 as size_t {
                    malloc(::core::mem::size_of::<size_t>() as size_t)
                } else {
                    realloc(
                        store_view.d_counts as *mut c_void,
                        dsize
                            .wrapping_mul(2 as size_t)
                            .wrapping_mul(::core::mem::size_of::<size_t>() as size_t),
                    )
                }) as *mut size_t;
            }
            *store_view.d_counts.offset(dsize as isize) = if origsize == 0 as size_t {
                0 as size_t
            } else {
                *store_view
                    .d_counts
                    .offset(origsize.wrapping_sub(32 as size_t).wrapping_add(i) as isize)
            };
            dsize = dsize.wrapping_add(1);
            i = i.wrapping_add(1);
        }
    }
    if store_view.size & store_view.size.wrapping_sub(1 as size_t) == 0 {
        store_view.litlens = (if store_view.size == 0 as size_t {
            malloc(::core::mem::size_of::<c_ushort>() as size_t)
        } else {
            realloc(
                store_view.litlens as *mut c_void,
                store_view
                    .size
                    .wrapping_mul(2 as size_t)
                    .wrapping_mul(::core::mem::size_of::<c_ushort>() as size_t),
            )
        }) as *mut c_ushort;
    }
    *store_view.litlens.offset(store_view.size as isize) = length;
    store_view.size = store_view.size.wrapping_add(1);
    store_view.size = origsize;
    if store_view.size & store_view.size.wrapping_sub(1 as size_t) == 0 {
        store_view.dists = (if store_view.size == 0 as size_t {
            malloc(::core::mem::size_of::<c_ushort>() as size_t)
        } else {
            realloc(
                store_view.dists as *mut c_void,
                store_view
                    .size
                    .wrapping_mul(2 as size_t)
                    .wrapping_mul(::core::mem::size_of::<c_ushort>() as size_t),
            )
        }) as *mut c_ushort;
    }
    *store_view.dists.offset(store_view.size as isize) = dist;
    store_view.size = store_view.size.wrapping_add(1);
    store_view.size = origsize;
    if store_view.size & store_view.size.wrapping_sub(1 as size_t) == 0 {
        store_view.pos = (if store_view.size == 0 as size_t {
            malloc(::core::mem::size_of::<size_t>() as size_t)
        } else {
            realloc(
                store_view.pos as *mut c_void,
                store_view
                    .size
                    .wrapping_mul(2 as size_t)
                    .wrapping_mul(::core::mem::size_of::<size_t>() as size_t),
            )
        }) as *mut size_t;
    }
    *store_view.pos.offset(store_view.size as isize) = pos;
    store_view.size = store_view.size.wrapping_add(1);
    if dist as c_int == 0 as c_int {
        store_view.size = origsize;
        if store_view.size & store_view.size.wrapping_sub(1 as size_t) == 0 {
            store_view.ll_symbol = (if store_view.size == 0 as size_t {
                malloc(::core::mem::size_of::<c_ushort>() as size_t)
            } else {
                realloc(
                    store_view.ll_symbol as *mut c_void,
                    store_view
                        .size
                        .wrapping_mul(2 as size_t)
                        .wrapping_mul(::core::mem::size_of::<c_ushort>() as size_t),
                )
            }) as *mut c_ushort;
        }
        *store_view.ll_symbol.offset(store_view.size as isize) = length;
        store_view.size = store_view.size.wrapping_add(1);
        store_view.size = origsize;
        if store_view.size & store_view.size.wrapping_sub(1 as size_t) == 0 {
            store_view.d_symbol = (if store_view.size == 0 as size_t {
                malloc(::core::mem::size_of::<c_ushort>() as size_t)
            } else {
                realloc(
                    store_view.d_symbol as *mut c_void,
                    store_view
                        .size
                        .wrapping_mul(2 as size_t)
                        .wrapping_mul(::core::mem::size_of::<c_ushort>() as size_t),
                )
            }) as *mut c_ushort;
        }
        *store_view.d_symbol.offset(store_view.size as isize) = 0 as c_ushort;
        store_view.size = store_view.size.wrapping_add(1);
        let ref mut fresh0 = *store_view
            .ll_counts
            .offset(llstart.wrapping_add(length as size_t) as isize);
        *fresh0 = (*fresh0).wrapping_add(1);
    } else {
        store_view.size = origsize;
        if store_view.size & store_view.size.wrapping_sub(1 as size_t) == 0 {
            store_view.ll_symbol = (if store_view.size == 0 as size_t {
                malloc(::core::mem::size_of::<c_ushort>() as size_t)
            } else {
                realloc(
                    store_view.ll_symbol as *mut c_void,
                    store_view
                        .size
                        .wrapping_mul(2 as size_t)
                        .wrapping_mul(::core::mem::size_of::<c_ushort>() as size_t),
                )
            }) as *mut c_ushort;
        }
        *store_view.ll_symbol.offset(store_view.size as isize) =
            ZopfliGetLengthSymbol(length as c_int) as c_ushort;
        store_view.size = store_view.size.wrapping_add(1);
        store_view.size = origsize;
        if store_view.size & store_view.size.wrapping_sub(1 as size_t) == 0 {
            store_view.d_symbol = (if store_view.size == 0 as size_t {
                malloc(::core::mem::size_of::<c_ushort>() as size_t)
            } else {
                realloc(
                    store_view.d_symbol as *mut c_void,
                    store_view
                        .size
                        .wrapping_mul(2 as size_t)
                        .wrapping_mul(::core::mem::size_of::<c_ushort>() as size_t),
                )
            }) as *mut c_ushort;
        }
        *store_view.d_symbol.offset(store_view.size as isize) =
            ZopfliGetDistSymbol(dist as c_int) as c_ushort;
        store_view.size = store_view.size.wrapping_add(1);
        let ref mut fresh1 = *store_view.ll_counts.offset(
            llstart.wrapping_add(ZopfliGetLengthSymbol(length as c_int) as size_t)
                as isize,
        );
        *fresh1 = (*fresh1).wrapping_add(1);
        let ref mut fresh2 = *store_view.d_counts.offset(
            dstart.wrapping_add(ZopfliGetDistSymbol(dist as c_int) as size_t) as isize,
        );
        *fresh2 = (*fresh2).wrapping_add(1);
    };
}
#[inline]
pub unsafe fn ZopfliAppendLZ77Store(
    mut store: *const ZopfliLZ77Store,
    mut target: *mut ZopfliLZ77Store,
) {
    let store_view: &ZopfliLZ77Store = unsafe { &*store };
    let mut i: size_t = 0;
    i = 0 as size_t;
    while i < store_view.size {
        ZopfliStoreLitLenDist(
            *store_view.litlens.offset(i as isize),
            *store_view.dists.offset(i as isize),
            *store_view.pos.offset(i as isize),
            target,
        );
        i = i.wrapping_add(1);
    }
}
#[inline]
pub unsafe fn ZopfliLZ77GetByteRange(
    mut lz77: *const ZopfliLZ77Store,
    mut lstart: size_t,
    mut lend: size_t,
) -> size_t {
    let lz77_view: &ZopfliLZ77Store = unsafe { &*lz77 };
    let mut l: size_t = lend.wrapping_sub(1 as size_t);
    if lstart == lend {
        return 0 as size_t;
    }
    return (*lz77_view.pos.offset(l as isize))
        .wrapping_add(
            (if *lz77_view.dists.offset(l as isize) as c_int == 0 as c_int {
                1 as c_int
            } else {
                *lz77_view.litlens.offset(l as isize) as c_int
            }) as size_t,
        )
        .wrapping_sub(*lz77_view.pos.offset(lstart as isize));
}
unsafe fn ZopfliLZ77GetHistogramAt(
    mut lz77: *const ZopfliLZ77Store,
    mut lpos: size_t,
    mut ll_counts: *mut size_t,
    mut d_counts: *mut size_t,
) {
    let lz77_view: &ZopfliLZ77Store = unsafe { &*lz77 };
    let mut llpos: size_t =
        (ZOPFLI_NUM_LL as size_t).wrapping_mul(lpos.wrapping_div(ZOPFLI_NUM_LL as size_t));
    let mut dpos: size_t =
        (ZOPFLI_NUM_D as size_t).wrapping_mul(lpos.wrapping_div(ZOPFLI_NUM_D as size_t));
    let mut i: size_t = 0;
    i = 0 as size_t;
    while i < ZOPFLI_NUM_LL as size_t {
        *ll_counts.offset(i as isize) = *lz77_view.ll_counts.offset(llpos.wrapping_add(i) as isize);
        i = i.wrapping_add(1);
    }
    i = lpos.wrapping_add(1 as size_t);
    while i < llpos.wrapping_add(ZOPFLI_NUM_LL as size_t) && i < lz77_view.size {
        let ref mut fresh7 = *ll_counts.offset(*lz77_view.ll_symbol.offset(i as isize) as isize);
        *fresh7 = (*fresh7).wrapping_sub(1);
        i = i.wrapping_add(1);
    }
    i = 0 as size_t;
    while i < ZOPFLI_NUM_D as size_t {
        *d_counts.offset(i as isize) = *lz77_view.d_counts.offset(dpos.wrapping_add(i) as isize);
        i = i.wrapping_add(1);
    }
    i = lpos.wrapping_add(1 as size_t);
    while i < dpos.wrapping_add(ZOPFLI_NUM_D as size_t) && i < lz77_view.size {
        if *lz77_view.dists.offset(i as isize) as c_int != 0 as c_int {
            let ref mut fresh8 = *d_counts.offset(*lz77_view.d_symbol.offset(i as isize) as isize);
            *fresh8 = (*fresh8).wrapping_sub(1);
        }
        i = i.wrapping_add(1);
    }
}
#[inline]
pub unsafe fn ZopfliLZ77GetHistogram(
    mut lz77: *const ZopfliLZ77Store,
    mut lstart: size_t,
    mut lend: size_t,
    mut ll_counts: *mut size_t,
    mut d_counts: *mut size_t,
) {
    let lz77_view: &ZopfliLZ77Store = unsafe { &*lz77 };
    let mut i: size_t = 0;
    if lstart.wrapping_add((ZOPFLI_NUM_LL * 3 as c_int) as size_t) > lend {
        memset(
            ll_counts as *mut c_void,
            0 as c_int,
            (::core::mem::size_of::<size_t>() as size_t).wrapping_mul(ZOPFLI_NUM_LL as size_t),
        );
        memset(
            d_counts as *mut c_void,
            0 as c_int,
            (::core::mem::size_of::<size_t>() as size_t).wrapping_mul(ZOPFLI_NUM_D as size_t),
        );
        i = lstart;
        while i < lend {
            let ref mut fresh3 = *ll_counts.offset(*lz77_view.ll_symbol.offset(i as isize) as isize);
            *fresh3 = (*fresh3).wrapping_add(1);
            if *lz77_view.dists.offset(i as isize) as c_int != 0 as c_int {
                let ref mut fresh4 =
                    *d_counts.offset(*lz77_view.d_symbol.offset(i as isize) as isize);
                *fresh4 = (*fresh4).wrapping_add(1);
            }
            i = i.wrapping_add(1);
        }
    } else {
        ZopfliLZ77GetHistogramAt(lz77, lend.wrapping_sub(1 as size_t), ll_counts, d_counts);
        if lstart > 0 as size_t {
            let mut ll_counts2: [size_t; 288] = [0; 288];
            let mut d_counts2: [size_t; 32] = [0; 32];
            ZopfliLZ77GetHistogramAt(
                lz77,
                lstart.wrapping_sub(1 as size_t),
                &raw mut ll_counts2 as *mut size_t,
                &raw mut d_counts2 as *mut size_t,
            );
            i = 0 as size_t;
            while i < ZOPFLI_NUM_LL as size_t {
                let ref mut fresh5 = *ll_counts.offset(i as isize);
                *fresh5 = (*fresh5).wrapping_sub(ll_counts2[i as usize]);
                i = i.wrapping_add(1);
            }
            i = 0 as size_t;
            while i < ZOPFLI_NUM_D as size_t {
                let ref mut fresh6 = *d_counts.offset(i as isize);
                *fresh6 = (*fresh6).wrapping_sub(d_counts2[i as usize]);
                i = i.wrapping_add(1);
            }
        }
    };
}
#[inline]
pub unsafe fn ZopfliInitBlockState(
    mut options: *const ZopfliOptions,
    mut blockstart: size_t,
    mut blockend: size_t,
    mut add_lmc: c_int,
    mut s: *mut ZopfliBlockState,
) {
    let s_view: &mut ZopfliBlockState = unsafe { &mut *s };
    s_view.options = options;
    s_view.blockstart = blockstart;
    s_view.blockend = blockend;
    if add_lmc != 0 {
        s_view.lmc = malloc(::core::mem::size_of::<ZopfliLongestMatchCache>() as size_t)
            as *mut ZopfliLongestMatchCache;
        ZopfliInitCache(blockend.wrapping_sub(blockstart), s_view.lmc);
    } else {
        s_view.lmc = ::core::ptr::null_mut::<ZopfliLongestMatchCache>();
    };
}
#[inline]
pub unsafe fn ZopfliCleanBlockState(mut s: *mut ZopfliBlockState) {
    let s_view: &ZopfliBlockState = unsafe { &*s };
    if !s_view.lmc.is_null() {
        ZopfliCleanCache(s_view.lmc);
        free(s_view.lmc as *mut c_void);
    }
}
fn GetLengthScore(
    mut length: c_int,
    mut distance: c_int,
) -> c_int { {
    return if distance > 1024 as c_int {
        length - 1 as c_int
    } else {
        length
    };
} }
#[inline]
pub unsafe fn ZopfliVerifyLenDist(
    mut data: *const c_uchar,
    mut datasize: size_t,
    mut pos: size_t,
    mut dist: c_ushort,
    mut length: c_ushort,
) {
    let mut i: size_t = 0;
    i = 0 as size_t;
    while i < length as size_t {
        if *data.offset(pos.wrapping_sub(dist as size_t).wrapping_add(i) as isize)
            as c_int
            != *data.offset(pos.wrapping_add(i) as isize) as c_int
        {
            break;
        }
        i = i.wrapping_add(1);
    }
}
unsafe fn GetMatch(
    mut scan: *const c_uchar,
    mut match_0: *const c_uchar,
    mut end: *const c_uchar,
    mut safe_end: *const c_uchar,
) -> *const c_uchar {
    if ::core::mem::size_of::<size_t>() as usize == 8 as usize {
    while (safe_end as usize).wrapping_sub(scan as usize) >= 8 {
        let sw = ::core::ptr::read_unaligned(scan as *const u64);
        let mw = ::core::ptr::read_unaligned(match_0 as *const u64);
        let x = sw ^ mw;
        if x != 0 {
            let adv = (x.trailing_zeros() >> 3) as isize;
            scan = scan.offset(adv);
            match_0 = match_0.offset(adv);
            break;
        }
        scan = scan.offset(8);
        match_0 = match_0.offset(8);
    }
} else if ::core::mem::size_of::<c_uint>() as usize == 4 as usize {
    while (safe_end as usize).wrapping_sub(scan as usize) >= 4 {
        let sw = ::core::ptr::read_unaligned(scan as *const u32);
        let mw = ::core::ptr::read_unaligned(match_0 as *const u32);
        let x = sw ^ mw;
        if x != 0 {
            let adv = (x.trailing_zeros() >> 3) as isize;
            scan = scan.offset(adv);
            match_0 = match_0.offset(adv);
            break;
        }
        scan = scan.offset(4);
        match_0 = match_0.offset(4);
    }
}
while scan != end && *scan as c_int == *match_0 as c_int {
    scan = scan.offset(1);
    match_0 = match_0.offset(1);
}
return scan;
}
// Applied rules: [III④, C3]
// Skipped rules: [III③: Hits [4, 5] name ZopfliMaxCachedSublen, but this function is not a manual memory operation/string wrapper site under III③ semantics; it is a project helper query, so rewriting it under the memory-op card would be misclassified.]
unsafe fn TryGetFromLongestMatchCache(
    mut s: *mut ZopfliBlockState,
    mut pos: size_t,
    mut limit: *mut size_t,
    mut sublen: *mut c_ushort,
    mut distance: *mut c_ushort,
    mut length: *mut c_ushort,
) -> c_int {
    let s_view: &ZopfliBlockState = unsafe { &*s };
    let mut lmcpos: size_t = pos.wrapping_sub(s_view.blockstart);
    let lmc = s_view.lmc;

    let mut cache_available: c_uchar = 0;
    let mut limit_ok_for_cache: c_uchar = 0;

    if !lmc.is_null() {
        let lmc_ref: &ZopfliLongestMatchCache = unsafe { &*lmc };
        let length_ptr = lmc_ref.length;
        let dist_ptr = lmc_ref.dist;

        let cached_length: c_ushort = unsafe {
            // SAFETY: `lmc` is non-null in this branch, and `lmcpos` is the same
            // in-bounds cache index used by the original raw-pointer offset loads.
            *length_ptr.offset(lmcpos as isize)
        };
        let cached_dist: c_ushort = unsafe {
            // SAFETY: `lmc` is non-null in this branch, and `lmcpos` is the same
            // in-bounds cache index used by the original raw-pointer offset loads.
            *dist_ptr.offset(lmcpos as isize)
        };

        cache_available =
            (cached_length as c_int == 0 as c_int || cached_dist as c_int != 0 as c_int) as c_int
                as c_uchar;

        limit_ok_for_cache = (cache_available as c_int != 0
            && (*limit == ZOPFLI_MAX_MATCH as size_t
                || cached_length as size_t <= *limit
                || !sublen.is_null()
                    && ZopfliMaxCachedSublen(lmc, lmcpos, cached_length as size_t) as size_t
                        >= *limit)) as c_int as c_uchar;

        if limit_ok_for_cache as c_int != 0 && cache_available as c_int != 0 {
            if sublen.is_null()
                || cached_length as c_uint
                    <= ZopfliMaxCachedSublen(lmc, lmcpos, cached_length as size_t)
            {
                *length = cached_length;
                if *length as size_t > *limit {
                    *length = *limit as c_ushort;
                }
                if !sublen.is_null() {
                    ZopfliCacheToSublen(lmc, lmcpos, *length as size_t, sublen);
                    *distance = unsafe {
                        // SAFETY: `sublen` is non-null in this branch, and the original
                        // code reads the element at `*length` via the same unchecked offset.
                        *sublen.offset(*length as isize)
                    };
                    *limit == ZOPFLI_MAX_MATCH as size_t
                        && *length as c_int >= ZOPFLI_MIN_MATCH;
                } else {
                    *distance = cached_dist;
                }
                return 1 as c_int;
            }
            *limit = cached_length as size_t;
        }
    }
    return 0 as c_int;
}
unsafe fn StoreInLongestMatchCache(
    mut s: *mut ZopfliBlockState,
    mut pos: size_t,
    mut limit: size_t,
    mut sublen: *const c_ushort,
    mut distance: c_ushort,
    mut length: c_ushort,
) {
    let s_view: &ZopfliBlockState = unsafe { &*s };
    let mut lmcpos: size_t = pos.wrapping_sub(s_view.blockstart);
    let mut cache_available: c_uchar = (!s_view.lmc.is_null()
        && (*(*s_view.lmc).length.offset(lmcpos as isize) as c_int
            == 0 as c_int
            || *(*s_view.lmc).dist.offset(lmcpos as isize) as c_int
                != 0 as c_int))
        as c_int
        as c_uchar;
    if !s_view.lmc.is_null()
        && limit == ZOPFLI_MAX_MATCH as size_t
        && !sublen.is_null()
        && cache_available == 0
    {
        *(*s_view.lmc).dist.offset(lmcpos as isize) =
            (if (length as c_int) < ZOPFLI_MIN_MATCH {
                0 as c_int
            } else {
                distance as c_int
            }) as c_ushort;
        *(*s_view.lmc).length.offset(lmcpos as isize) =
            (if (length as c_int) < ZOPFLI_MIN_MATCH {
                0 as c_int
            } else {
                length as c_int
            }) as c_ushort;
        ZopfliSublenToCache(sublen, lmcpos, length as size_t, s_view.lmc);
    }
}
#[inline]
pub unsafe fn ZopfliFindLongestMatch(
    mut s: *mut ZopfliBlockState,
    mut h: *const ZopfliHash,
    mut array: *const c_uchar,
    mut pos: size_t,
    mut size: size_t,
    mut limit: size_t,
    mut sublen: *mut c_ushort,
    mut distance: *mut c_ushort,
    mut length: *mut c_ushort,
) {
    let h_view: &ZopfliHash = unsafe { &*h };
    let mut hpos: c_ushort =
        (pos & ZOPFLI_WINDOW_MASK as size_t) as c_ushort;
    let mut p: c_ushort = 0;
    let mut pp: c_ushort = 0;
    let mut bestdist: c_ushort = 0 as c_ushort;
    let mut bestlength: c_ushort = 1 as c_ushort;
    let mut scan: *const c_uchar = ::core::ptr::null::<c_uchar>();
    let mut match_0: *const c_uchar = ::core::ptr::null::<c_uchar>();
    let mut arrayend: *const c_uchar = ::core::ptr::null::<c_uchar>();
    let mut arrayend_safe: *const c_uchar =
        ::core::ptr::null::<c_uchar>();
    let mut chain_counter: c_int = ZOPFLI_MAX_CHAIN_HITS;
    let mut dist: c_uint = 0 as c_uint;
    let mut hhead: *mut c_int = h_view.head;
    let mut hprev: *mut c_ushort = h_view.prev;
    let mut hhashval: *mut c_int = h_view.hashval;
    let mut hval: c_int = h_view.val;
    if TryGetFromLongestMatchCache(s, pos, &raw mut limit, sublen, distance, length) != 0 {
        return;
    }
    if size.wrapping_sub(pos) < ZOPFLI_MIN_MATCH as size_t {
        *length = 0 as c_ushort;
        *distance = 0 as c_ushort;
        return;
    }
    if pos.wrapping_add(limit) > size {
        limit = size.wrapping_sub(pos);
    }
    arrayend = (array.offset(pos as isize) as *const c_uchar).offset(limit as isize);
    arrayend_safe = arrayend.offset(-(8 as c_int as isize));
    pp = *hhead.offset(hval as isize) as c_ushort;
    p = *hprev.offset(pp as isize);
    dist = (if (p as c_int) < pp as c_int {
        pp as c_int - p as c_int
    } else {
        ZOPFLI_WINDOW_SIZE - p as c_int + pp as c_int
    }) as c_uint;
    while dist < ZOPFLI_WINDOW_SIZE as c_uint {
        let mut currentlength: c_ushort = 0 as c_ushort;
        if dist > 0 as c_uint {
            scan = array.offset(pos as isize) as *const c_uchar;
            match_0 = array.offset(pos.wrapping_sub(dist as size_t) as isize)
                as *const c_uchar;
            if pos.wrapping_add(bestlength as size_t) >= size
                || *scan.offset(bestlength as c_int as isize) as c_int
                    == *match_0.offset(bestlength as c_int as isize)
                        as c_int
            {
                let mut same0: c_ushort = *h_view
                    .same
                    .offset((pos & ZOPFLI_WINDOW_MASK as size_t) as isize);
                if same0 as c_int > 2 as c_int
                    && *scan as c_int == *match_0 as c_int
                {
                    let mut same1: c_ushort = *h_view.same.offset(
                        (pos.wrapping_sub(dist as size_t) & ZOPFLI_WINDOW_MASK as size_t) as isize,
                    );
                    let mut same: c_ushort =
                        (if (same0 as c_int) < same1 as c_int {
                            same0 as c_int
                        } else {
                            same1 as c_int
                        }) as c_ushort;
                    if same as size_t > limit {
                        same = limit as c_ushort;
                    }
                    scan = scan.offset(same as c_int as isize);
                    match_0 = match_0.offset(same as c_int as isize);
                }
                scan = GetMatch(scan, match_0, arrayend, arrayend_safe);
                currentlength = scan
                    .offset_from(array.offset(pos as isize) as *const c_uchar)
                    as c_long as c_ushort;
            }
            if currentlength as c_int > bestlength as c_int {
                if !sublen.is_null() {
                    let mut j: c_ushort = 0;
                    j = (bestlength as c_int + 1 as c_int)
                        as c_ushort;
                    while j as c_int <= currentlength as c_int {
                        *sublen.offset(j as isize) = dist as c_ushort;
                        j = j.wrapping_add(1);
                    }
                }
                bestdist = dist as c_ushort;
                bestlength = currentlength;
                if currentlength as size_t >= limit {
                    break;
                }
            }
        }
        if hhead != h_view.head2
            && bestlength as c_int
                >= *h_view.same.offset(hpos as isize) as c_int
            && h_view.val2 == *h_view.hashval2.offset(p as isize)
        {
            hhead = h_view.head2;
            hprev = h_view.prev2;
            hhashval = h_view.hashval2;
            hval = h_view.val2;
        }
        pp = p;
        p = *hprev.offset(p as isize);
        if p as c_int == pp as c_int {
            break;
        }
        dist = dist.wrapping_add(
            (if (p as c_int) < pp as c_int {
                pp as c_int - p as c_int
            } else {
                ZOPFLI_WINDOW_SIZE - p as c_int + pp as c_int
            }) as c_uint,
        );
        chain_counter -= 1;
        if chain_counter <= 0 as c_int {
            break;
        }
    }
    StoreInLongestMatchCache(s, pos, limit, sublen, bestdist, bestlength);
    *distance = bestdist;
    *length = bestlength;
}
#[inline]
pub unsafe fn ZopfliLZ77Greedy(
    mut s: *mut ZopfliBlockState,
    mut in_0: *const c_uchar,
    mut instart: size_t,
    mut inend: size_t,
    mut store: *mut ZopfliLZ77Store,
    mut h: *mut ZopfliHash,
) {
    let mut i: size_t = 0 as size_t;
    let mut j: size_t = 0;
    let mut leng: c_ushort = 0;
    let mut dist: c_ushort = 0;
    let mut lengthscore: c_int = 0;
    let mut windowstart: size_t = if instart > ZOPFLI_WINDOW_SIZE as size_t {
        instart.wrapping_sub(ZOPFLI_WINDOW_SIZE as size_t)
    } else {
        0 as size_t
    };
    let mut dummysublen: [c_ushort; 259] = [0; 259];
    let mut prev_length: c_uint = 0 as c_uint;
    let mut prev_match: c_uint = 0 as c_uint;
    let mut prevlengthscore: c_int = 0;
    let mut match_available: c_int = 0 as c_int;
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
    let mut current_block_44: u64;
    i = instart;
    while i < inend {
        ZopfliUpdateHash(in_0, i, inend, h);
        ZopfliFindLongestMatch(
            s,
            h,
            in_0,
            i,
            inend,
            ZOPFLI_MAX_MATCH as size_t,
            &raw mut dummysublen as *mut c_ushort,
            &raw mut dist,
            &raw mut leng,
        );
        lengthscore = GetLengthScore(leng as c_int, dist as c_int);
        prevlengthscore = GetLengthScore(
            prev_length as c_int,
            prev_match as c_int,
        );
        if match_available != 0 {
            match_available = 0 as c_int;
            if lengthscore > prevlengthscore + 1 as c_int {
                ZopfliStoreLitLenDist(
                    *in_0.offset(i.wrapping_sub(1 as size_t) as isize) as c_ushort,
                    0 as c_ushort,
                    i.wrapping_sub(1 as size_t),
                    store,
                );
                if lengthscore >= ZOPFLI_MIN_MATCH
                    && (leng as c_int) < ZOPFLI_MAX_MATCH
                {
                    match_available = 1 as c_int;
                    prev_length = leng as c_uint;
                    prev_match = dist as c_uint;
                    current_block_44 = 11650488183268122163;
                } else {
                    current_block_44 = 8704759739624374314;
                }
            } else {
                leng = prev_length as c_ushort;
                dist = prev_match as c_ushort;
                lengthscore = prevlengthscore;
                ZopfliVerifyLenDist(in_0, inend, i.wrapping_sub(1 as size_t), dist, leng);
                ZopfliStoreLitLenDist(leng, dist, i.wrapping_sub(1 as size_t), store);
                j = 2 as size_t;
                while j < leng as size_t {
                    i = i.wrapping_add(1);
                    ZopfliUpdateHash(in_0, i, inend, h);
                    j = j.wrapping_add(1);
                }
                current_block_44 = 11650488183268122163;
            }
        } else if lengthscore >= ZOPFLI_MIN_MATCH && (leng as c_int) < ZOPFLI_MAX_MATCH
        {
            match_available = 1 as c_int;
            prev_length = leng as c_uint;
            prev_match = dist as c_uint;
            current_block_44 = 11650488183268122163;
        } else {
            current_block_44 = 8704759739624374314;
        }
        match current_block_44 {
            8704759739624374314 => {
                if lengthscore >= ZOPFLI_MIN_MATCH {
                    ZopfliVerifyLenDist(in_0, inend, i, dist, leng);
                    ZopfliStoreLitLenDist(leng, dist, i, store);
                } else {
                    leng = 1 as c_ushort;
                    ZopfliStoreLitLenDist(
                        *in_0.offset(i as isize) as c_ushort,
                        0 as c_ushort,
                        i,
                        store,
                    );
                }
                j = 1 as size_t;
                while j < leng as size_t {
                    i = i.wrapping_add(1);
                    ZopfliUpdateHash(in_0, i, inend, h);
                    j = j.wrapping_add(1);
                }
            }
            _ => {}
        }
        i = i.wrapping_add(1);
    }
}

