use core::ffi::*;
pub use crate::src::ffi::*;
pub use crate::src::c_consts::*;
pub use crate::src::c_structs::*;
pub use crate::src::c_types::*;
pub use crate::src::c_extern_types::*;
extern "C" {
    fn ZopfliCalculateBlockSizeAutoType(
        lz77: *const ZopfliLZ77Store,
        lstart: size_t,
        lend: size_t,
    ) -> c_double;
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct SplitCostContext {
    pub lz77: *const ZopfliLZ77Store,
    pub start: size_t,
    pub end: size_t,
}
pub type FindMinimumFun =
    unsafe extern "C" fn(size_t, *mut c_void) -> c_double;

unsafe extern "C" fn FindMinimum(
    mut f: Option<FindMinimumFun>,
    mut context: *mut c_void,
    mut start: size_t,
    mut end: size_t,
    mut smallest: *mut c_double,
) -> size_t {
    if end.wrapping_sub(start) < 1024 as size_t {
        let mut best: c_double = ZOPFLI_LARGE_FLOAT;
        let mut result: size_t = start;
        let mut i: size_t = 0;
        i = start;
        while i < end {
            let mut v: c_double = f.expect("non-null function pointer")(i, context);
            if v < best {
                best = v;
                result = i;
            }
            i = i.wrapping_add(1);
        }
        *smallest = best;
        return result;
    } else {
        let mut i_0: size_t = 0;
        let mut p: [size_t; 9] = [0; 9];
        let mut vp: [c_double; 9] = [0.; 9];
        let mut besti: size_t = 0;
        let mut best_0: c_double = 0.;
        let mut lastbest: c_double = ZOPFLI_LARGE_FLOAT;
        let mut pos: size_t = start;
        while !(end.wrapping_sub(start) <= NUM as size_t) {
            i_0 = 0 as size_t;
            while i_0 < NUM as size_t {
                p[i_0 as usize] = start.wrapping_add(
                    i_0.wrapping_add(1 as size_t).wrapping_mul(
                        end.wrapping_sub(start)
                            .wrapping_div((NUM + 1 as c_int) as size_t),
                    ),
                );
                vp[i_0 as usize] = f.expect("non-null function pointer")(p[i_0 as usize], context);
                i_0 = i_0.wrapping_add(1);
            }
            besti = 0 as size_t;
            best_0 = vp[0 as c_int as usize];
            i_0 = 1 as size_t;
            while i_0 < NUM as size_t {
                if vp[i_0 as usize] < best_0 {
                    best_0 = vp[i_0 as usize];
                    besti = i_0;
                }
                i_0 = i_0.wrapping_add(1);
            }
            if best_0 > lastbest {
                break;
            }
            start = if besti == 0 as size_t {
                start
            } else {
                p[besti.wrapping_sub(1 as size_t) as usize]
            };
            end = if besti == (NUM - 1 as c_int) as size_t {
                end
            } else {
                p[besti.wrapping_add(1 as size_t) as usize]
            };
            pos = p[besti as usize];
            lastbest = best_0;
        }
        *smallest = lastbest;
        return pos;
    };
}
pub const NUM: c_int = 9 as c_int;
unsafe extern "C" fn EstimateCost(
    mut lz77: *const ZopfliLZ77Store,
    mut lstart: size_t,
    mut lend: size_t,
) -> c_double {
    return ZopfliCalculateBlockSizeAutoType(lz77, lstart, lend);
}
unsafe extern "C" fn SplitCost(
    mut i: size_t,
    mut context: *mut c_void,
) -> c_double {
    let mut c: *mut SplitCostContext = context as *mut SplitCostContext;
    return EstimateCost((*c).lz77, (*c).start, i) + EstimateCost((*c).lz77, i, (*c).end);
}
unsafe extern "C" fn AddSorted(
    mut value: size_t,
    mut out: *mut *mut size_t,
    mut outsize: *mut size_t,
) {
    let mut i: size_t = 0;
    if *outsize & (*outsize).wrapping_sub(1 as size_t) == 0 {
        *out = (if *outsize == 0 as size_t {
            malloc(::core::mem::size_of::<size_t>() as size_t)
        } else {
            realloc(
                *out as *mut c_void,
                (*outsize)
                    .wrapping_mul(2 as size_t)
                    .wrapping_mul(::core::mem::size_of::<size_t>() as size_t),
            )
        }) as *mut size_t;
    }
    *(*out).offset(*outsize as isize) = value;
    *outsize = (*outsize).wrapping_add(1);
    i = 0 as size_t;
    while i.wrapping_add(1 as size_t) < *outsize {
        if *(*out).offset(i as isize) > value {
            let mut j: size_t = 0;
            j = (*outsize).wrapping_sub(1 as size_t);
            while j > i {
                *(*out).offset(j as isize) = *(*out).offset(j.wrapping_sub(1 as size_t) as isize);
                j = j.wrapping_sub(1);
            }
            *(*out).offset(i as isize) = value;
            break;
        } else {
            i = i.wrapping_add(1);
        }
    }
}
unsafe extern "C" fn PrintBlockSplitPoints(
    mut lz77: *const ZopfliLZ77Store,
    mut lz77splitpoints: *const size_t,
    mut nlz77points: size_t,
) {
    let mut splitpoints: *mut size_t = ::core::ptr::null_mut::<size_t>();
    let mut npoints: size_t = 0 as size_t;
    let mut i: size_t = 0;
    let mut pos: size_t = 0 as size_t;
    if nlz77points > 0 as size_t {
        i = 0 as size_t;
        while i < (*lz77).size {
            let mut length: size_t = (if *(*lz77).dists.offset(i as isize) as c_int
                == 0 as c_int
            {
                1 as c_int
            } else {
                *(*lz77).litlens.offset(i as isize) as c_int
            }) as size_t;
            if *lz77splitpoints.offset(npoints as isize) == i {
                if npoints & npoints.wrapping_sub(1 as size_t) == 0 {
                    splitpoints = (if npoints == 0 as size_t {
                        malloc(::core::mem::size_of::<size_t>() as size_t)
                    } else {
                        realloc(
                            splitpoints as *mut c_void,
                            npoints
                                .wrapping_mul(2 as size_t)
                                .wrapping_mul(::core::mem::size_of::<size_t>() as size_t),
                        )
                    }) as *mut size_t;
                }
                *splitpoints.offset(npoints as isize) = pos;
                npoints = npoints.wrapping_add(1);
                if npoints == nlz77points {
                    break;
                }
            }
            pos = pos.wrapping_add(length);
            i = i.wrapping_add(1);
        }
    }
    fprintf(
        stderr,
        b"block split points: \0" as *const u8 as *const c_char,
    );
    i = 0 as size_t;
    while i < npoints {
        fprintf(
            stderr,
            b"%d \0" as *const u8 as *const c_char,
            *splitpoints.offset(i as isize) as c_int,
        );
        i = i.wrapping_add(1);
    }
    fprintf(
        stderr,
        b"(hex:\0" as *const u8 as *const c_char,
    );
    i = 0 as size_t;
    while i < npoints {
        fprintf(
            stderr,
            b" %x\0" as *const u8 as *const c_char,
            *splitpoints.offset(i as isize) as c_int,
        );
        i = i.wrapping_add(1);
    }
    fprintf(stderr, b")\n\0" as *const u8 as *const c_char);
    free(splitpoints as *mut c_void);
}
unsafe extern "C" fn FindLargestSplittableBlock(
    mut lz77size: size_t,
    mut done: *const c_uchar,
    mut splitpoints: *const size_t,
    mut npoints: size_t,
    mut lstart: *mut size_t,
    mut lend: *mut size_t,
) -> c_int {
    let mut longest: size_t = 0 as size_t;
    let mut found: c_int = 0 as c_int;
    let mut i: size_t = 0;
    i = 0 as size_t;
    while i <= npoints {
        let mut start: size_t = if i == 0 as size_t {
            0 as size_t
        } else {
            *splitpoints.offset(i.wrapping_sub(1 as size_t) as isize)
        };
        let mut end: size_t = if i == npoints {
            lz77size.wrapping_sub(1 as size_t)
        } else {
            *splitpoints.offset(i as isize)
        };
        if *done.offset(start as isize) == 0 && end.wrapping_sub(start) > longest {
            *lstart = start;
            *lend = end;
            found = 1 as c_int;
            longest = end.wrapping_sub(start);
        }
        i = i.wrapping_add(1);
    }
    return found;
}
#[no_mangle]
pub unsafe extern "C" fn ZopfliBlockSplitLZ77(
    mut options: *const ZopfliOptions,
    mut lz77: *const ZopfliLZ77Store,
    mut maxblocks: size_t,
    mut splitpoints: *mut *mut size_t,
    mut npoints: *mut size_t,
) {
    let mut lstart: size_t = 0;
    let mut lend: size_t = 0;
    let mut i: size_t = 0;
    let mut llpos: size_t = 0 as size_t;
    let mut numblocks: size_t = 1 as size_t;
    let mut done: *mut c_uchar = ::core::ptr::null_mut::<c_uchar>();
    let mut splitcost: c_double = 0.;
    let mut origcost: c_double = 0.;
    if (*lz77).size < 10 as size_t {
        return;
    }
    done = malloc((*lz77).size) as *mut c_uchar;
    if done.is_null() {
        exit(-(1 as c_int));
    }
    i = 0 as size_t;
    while i < (*lz77).size {
        *done.offset(i as isize) = 0 as c_uchar;
        i = i.wrapping_add(1);
    }
    lstart = 0 as size_t;
    lend = (*lz77).size;
    loop {
        let mut c: SplitCostContext = SplitCostContext {
            lz77: ::core::ptr::null::<ZopfliLZ77Store>(),
            start: 0,
            end: 0,
        };
        if maxblocks > 0 as size_t && numblocks >= maxblocks {
            break;
        }
        c.lz77 = lz77;
        c.start = lstart;
        c.end = lend;
        llpos = FindMinimum(
            Some(
                SplitCost
                    as unsafe extern "C" fn(
                        size_t,
                        *mut c_void,
                    ) -> c_double,
            ),
            &raw mut c as *mut c_void,
            lstart.wrapping_add(1 as size_t),
            lend,
            &raw mut splitcost,
        );
        origcost = EstimateCost(lz77, lstart, lend);
        if splitcost > origcost || llpos == lstart.wrapping_add(1 as size_t) || llpos == lend {
            *done.offset(lstart as isize) = 1 as c_uchar;
        } else {
            AddSorted(llpos, splitpoints, npoints);
            numblocks = numblocks.wrapping_add(1);
        }
        if FindLargestSplittableBlock(
            (*lz77).size,
            done,
            *splitpoints,
            *npoints,
            &raw mut lstart,
            &raw mut lend,
        ) == 0
        {
            break;
        }
        if lend.wrapping_sub(lstart) < 10 as size_t {
            break;
        }
    }
    if (*options).verbose != 0 {
        PrintBlockSplitPoints(lz77, *splitpoints, *npoints);
    }
    free(done as *mut c_void);
}
#[no_mangle]
pub unsafe extern "C" fn ZopfliBlockSplit(
    mut options: *const ZopfliOptions,
    mut in_0: *const c_uchar,
    mut instart: size_t,
    mut inend: size_t,
    mut maxblocks: size_t,
    mut splitpoints: *mut *mut size_t,
    mut npoints: *mut size_t,
) {
    let mut pos: size_t = 0 as size_t;
    let mut i: size_t = 0;
    let mut s: ZopfliBlockState = ZopfliBlockState {
        options: ::core::ptr::null::<ZopfliOptions>(),
        lmc: ::core::ptr::null_mut::<ZopfliLongestMatchCache>(),
        blockstart: 0,
        blockend: 0,
    };
    let mut lz77splitpoints: *mut size_t = ::core::ptr::null_mut::<size_t>();
    let mut nlz77points: size_t = 0 as size_t;
    let mut store: ZopfliLZ77Store = ZopfliLZ77Store {
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
    ZopfliInitLZ77Store(in_0, &raw mut store);
    ZopfliInitBlockState(options, instart, inend, 0 as c_int, &raw mut s);
    ZopfliAllocHash(ZOPFLI_WINDOW_SIZE as size_t, h);
    *npoints = 0 as size_t;
    *splitpoints = ::core::ptr::null_mut::<size_t>();
    ZopfliLZ77Greedy(&raw mut s, in_0, instart, inend, &raw mut store, h);
    ZopfliBlockSplitLZ77(
        options,
        &raw mut store,
        maxblocks,
        &raw mut lz77splitpoints,
        &raw mut nlz77points,
    );
    pos = instart;
    if nlz77points > 0 as size_t {
        i = 0 as size_t;
        while i < store.size {
            let mut length: size_t = (if *store.dists.offset(i as isize) as c_int
                == 0 as c_int
            {
                1 as c_int
            } else {
                *store.litlens.offset(i as isize) as c_int
            }) as size_t;
            if *lz77splitpoints.offset(*npoints as isize) == i {
                if *npoints & (*npoints).wrapping_sub(1 as size_t) == 0 {
                    *splitpoints = (if *npoints == 0 as size_t {
                        malloc(::core::mem::size_of::<size_t>() as size_t)
                    } else {
                        realloc(
                            *splitpoints as *mut c_void,
                            (*npoints)
                                .wrapping_mul(2 as size_t)
                                .wrapping_mul(::core::mem::size_of::<size_t>() as size_t),
                        )
                    }) as *mut size_t;
                }
                *(*splitpoints).offset(*npoints as isize) = pos;
                *npoints = (*npoints).wrapping_add(1);
                if *npoints == nlz77points {
                    break;
                }
            }
            pos = pos.wrapping_add(length);
            i = i.wrapping_add(1);
        }
    }
    free(lz77splitpoints as *mut c_void);
    ZopfliCleanBlockState(&raw mut s);
    ZopfliCleanLZ77Store(&raw mut store);
    ZopfliCleanHash(h);
}
#[no_mangle]
pub unsafe extern "C" fn ZopfliBlockSplitSimple(
    mut in_0: *const c_uchar,
    mut instart: size_t,
    mut inend: size_t,
    mut blocksize: size_t,
    mut splitpoints: *mut *mut size_t,
    mut npoints: *mut size_t,
) {
    let mut i: size_t = instart;
    while i < inend {
        if *npoints & (*npoints).wrapping_sub(1 as size_t) == 0 {
            *splitpoints = (if *npoints == 0 as size_t {
                malloc(::core::mem::size_of::<size_t>() as size_t)
            } else {
                realloc(
                    *splitpoints as *mut c_void,
                    (*npoints)
                        .wrapping_mul(2 as size_t)
                        .wrapping_mul(::core::mem::size_of::<size_t>() as size_t),
                )
            }) as *mut size_t;
        }
        *(*splitpoints).offset(*npoints as isize) = i;
        *npoints = (*npoints).wrapping_add(1);
        i = i.wrapping_add(blocksize);
    }
}
