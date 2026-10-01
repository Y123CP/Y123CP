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
        while scan < safe_end && *(scan as *mut size_t) == *(match_0 as *mut size_t) {
            scan = scan.offset(8 as c_int as isize);
            match_0 = match_0.offset(8 as c_int as isize);
        }
    } else if ::core::mem::size_of::<c_uint>() as usize == 4 as usize {
        while scan < safe_end
            && *(scan as *mut c_uint) == *(match_0 as *mut c_uint)
        {
            scan = scan.offset(4 as c_int as isize);
            match_0 = match_0.offset(4 as c_int as isize);
        }
    } else {
        while scan < safe_end
            && *scan as c_int == *match_0 as c_int
            && {
                scan = scan.offset(1);
                match_0 = match_0.offset(1);
                *scan as c_int == *match_0 as c_int
            }
            && {
                scan = scan.offset(1);
                match_0 = match_0.offset(1);
                *scan as c_int == *match_0 as c_int
            }
            && {
                scan = scan.offset(1);
                match_0 = match_0.offset(1);
                *scan as c_int == *match_0 as c_int
            }
            && {
                scan = scan.offset(1);
                match_0 = match_0.offset(1);
                *scan as c_int == *match_0 as c_int
            }
            && {
                scan = scan.offset(1);
                match_0 = match_0.offset(1);
                *scan as c_int == *match_0 as c_int
            }
            && {
                scan = scan.offset(1);
                match_0 = match_0.offset(1);
                *scan as c_int == *match_0 as c_int
            }
            && {
                scan = scan.offset(1);
                match_0 = match_0.offset(1);
                *scan as c_int == *match_0 as c_int
            }
        {
            scan = scan.offset(1);
            match_0 = match_0.offset(1);
        }
    }
    while scan != end && *scan as c_int == *match_0 as c_int {
        scan = scan.offset(1);
        match_0 = match_0.offset(1);
    }
    return scan;
}
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
    let mut cache_available: c_uchar = (!s_view.lmc.is_null()
        && (*(*s_view.lmc).length.offset(lmcpos as isize) as c_int
            == 0 as c_int
            || *(*s_view.lmc).dist.offset(lmcpos as isize) as c_int
                != 0 as c_int))
        as c_int
        as c_uchar;
    let mut limit_ok_for_cache: c_uchar =
        (cache_available as c_int != 0
            && (*limit == ZOPFLI_MAX_MATCH as size_t
                || *(*s_view.lmc).length.offset(lmcpos as isize) as size_t <= *limit
                || !sublen.is_null()
                    && ZopfliMaxCachedSublen(
                        s_view.lmc,
                        lmcpos,
                        *(*s_view.lmc).length.offset(lmcpos as isize) as size_t,
                    ) as size_t
                        >= *limit)) as c_int as c_uchar;
    if !s_view.lmc.is_null()
        && limit_ok_for_cache as c_int != 0
        && cache_available as c_int != 0
    {
        if sublen.is_null()
            || *(*s_view.lmc).length.offset(lmcpos as isize) as c_uint
                <= ZopfliMaxCachedSublen(
                    s_view.lmc,
                    lmcpos,
                    *(*s_view.lmc).length.offset(lmcpos as isize) as size_t,
                )
        {
            *length = *(*s_view.lmc).length.offset(lmcpos as isize);
            if *length as size_t > *limit {
                *length = *limit as c_ushort;
            }
            if !sublen.is_null() {
                ZopfliCacheToSublen(s_view.lmc, lmcpos, *length as size_t, sublen);
                *distance = *sublen.offset(*length as isize);
                *limit == ZOPFLI_MAX_MATCH as size_t
                    && *length as c_int >= ZOPFLI_MIN_MATCH;
            } else {
                *distance = *(*s_view.lmc).dist.offset(lmcpos as isize);
            }
            return 1 as c_int;
        }
        *limit = *(*s_view.lmc).length.offset(lmcpos as isize) as size_t;
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
    let h_view: &ZopfliHash = unsafe { &*h }; // SAFETY: caller guarantees `h` points to a valid `ZopfliHash` for the duration of this call.

    let hpos: c_ushort = (pos & ZOPFLI_WINDOW_MASK as size_t) as c_ushort;
    let mut p: c_ushort;
    let mut pp: c_ushort;
    let mut bestdist: c_ushort = 0;
    let mut bestlength: c_ushort = 1;
    let mut chain_counter: c_int = ZOPFLI_MAX_CHAIN_HITS;

    let mut hhead: *mut c_int = h_view.head;
    let mut hprev: *mut c_ushort = h_view.prev;
    let mut hval: c_int = h_view.val;

    if TryGetFromLongestMatchCache(s, pos, &raw mut limit, sublen, distance, length) != 0 {
        return;
    }
    if size.wrapping_sub(pos) < ZOPFLI_MIN_MATCH as size_t {
        unsafe {
            // SAFETY: caller provides valid output pointers.
            *length = 0;
            *distance = 0;
        }
        return;
    }
    if pos.wrapping_add(limit) > size {
        limit = size.wrapping_sub(pos);
    }

    let base = unsafe { array.add(pos) }; // SAFETY: `pos < size` here because `size - pos >= ZOPFLI_MIN_MATCH`.
    let arrayend = unsafe { base.add(limit) }; // SAFETY: `limit <= size - pos`, so this stays within/one-past the input buffer.
    let arrayend_safe = unsafe { arrayend.offset(-8) }; // SAFETY: original code relies on `GetMatch`'s contract and valid caller inputs; preserved exactly.

    pp = unsafe { *hhead.add(hval as usize) as c_ushort }; // SAFETY: hash tables are valid and indexed exactly as in the original.
    p = unsafe { *hprev.add(pp as usize) }; // SAFETY: hash tables are valid and indexed exactly as in the original.
    let mut dist: c_uint = if (p as c_int) < pp as c_int {
        (pp as c_int - p as c_int) as c_uint
    } else {
        (ZOPFLI_WINDOW_SIZE - p as c_int + pp as c_int) as c_uint
    };

    let same_arr = h_view.same;
    let head2 = h_view.head2;
    let prev2 = h_view.prev2;
    let hashval2 = h_view.hashval2;
    let val2 = h_view.val2;

    while dist < ZOPFLI_WINDOW_SIZE as c_uint {
        let mut currentlength: c_ushort = 0;

        if dist != 0 {
            let scan0 = base;
            let match0 = unsafe { array.add(pos.wrapping_sub(dist as size_t)) }; // SAFETY: `dist > 0` and chain guarantees a previous window position exactly as in the original.

            let best_idx = bestlength as usize;
            let can_try = pos.wrapping_add(bestlength as size_t) >= size
                || unsafe { *scan0.add(best_idx) == *match0.add(best_idx) }; // SAFETY: guarded exactly as in the original logic.

            if can_try {
                let mut scan = scan0;
                let mut match_0 = match0;

                let same0 = unsafe { *same_arr.add((pos & ZOPFLI_WINDOW_MASK as size_t) as usize) }; // SAFETY: ring-buffer index is masked into range.
                if same0 > 2 && unsafe { *scan == *match_0 } {
                    let same1 = unsafe {
                        *same_arr.add(
                            (pos.wrapping_sub(dist as size_t) & ZOPFLI_WINDOW_MASK as size_t)
                                as usize,
                        )
                    }; // SAFETY: ring-buffer index is masked into range.
                    let mut same = if same0 < same1 { same0 } else { same1 };
                    if same as size_t > limit {
                        same = limit as c_ushort;
                    }
                    scan = unsafe { scan.add(same as usize) }; // SAFETY: `same <= limit`, so still within current match range.
                    match_0 = unsafe { match_0.add(same as usize) }; // SAFETY: mirrors original pointer advance for the candidate match.
                }

                scan = GetMatch(scan, match_0, arrayend, arrayend_safe);
                currentlength = unsafe { scan.offset_from(base) as c_ushort }; // SAFETY: both pointers derive from the same allocation.
            }

            if currentlength > bestlength {
                if !sublen.is_null() {
                    let mut j = bestlength.wrapping_add(1);
                    while j <= currentlength {
                        unsafe {
                            // SAFETY: caller provides a valid `sublen` buffer large enough for all written lengths, same as original.
                            *sublen.add(j as usize) = dist as c_ushort;
                        }
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

        if hhead != head2
            && bestlength as c_int >= unsafe { *same_arr.add(hpos as usize) as c_int } // SAFETY: `hpos` is masked into the ring-buffer range.
            && val2 == unsafe { *hashval2.add(p as usize) }
        {
            // SAFETY: `p` is a valid hash chain index, same as original.
            hhead = head2;
            hprev = prev2;
            hval = val2;
            let _ = hval;
        }

        pp = p;
        p = unsafe { *hprev.add(p as usize) }; // SAFETY: hash chain index is valid by construction, same as original.
        if p == pp {
            break;
        }

        dist = dist.wrapping_add(if (p as c_int) < pp as c_int {
            (pp as c_int - p as c_int) as c_uint
        } else {
            (ZOPFLI_WINDOW_SIZE - p as c_int + pp as c_int) as c_uint
        });

        chain_counter -= 1;
        if chain_counter <= 0 {
            break;
        }
    }

    StoreInLongestMatchCache(s, pos, limit, sublen, bestdist, bestlength);
    unsafe {
        // SAFETY: caller provides valid output pointers.
        *distance = bestdist;
        *length = bestlength;
    }
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

