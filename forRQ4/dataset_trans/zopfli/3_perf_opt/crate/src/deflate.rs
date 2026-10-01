use core::ffi::*;
use crate::src::lz77::ZopfliAppendLZ77Store;
use crate::src::blocksplitter::ZopfliBlockSplit;
use crate::src::blocksplitter::ZopfliBlockSplitLZ77;
use crate::src::tree::ZopfliCalculateBitLengths;
use crate::src::lz77::ZopfliCleanBlockState;
use crate::src::lz77::ZopfliCleanLZ77Store;
use crate::src::c_inlined_fns::ZopfliGetDistExtraBits;
use crate::src::c_inlined_fns::ZopfliGetDistSymbol;
use crate::src::c_inlined_fns::ZopfliGetLengthExtraBits;
use crate::src::c_inlined_fns::ZopfliGetLengthSymbol;
use crate::src::lz77::ZopfliInitBlockState;
use crate::src::lz77::ZopfliInitLZ77Store;
use crate::src::lz77::ZopfliLZ77GetByteRange;
use crate::src::lz77::ZopfliLZ77GetHistogram;
use crate::src::squeeze::ZopfliLZ77Optimal;
use crate::src::squeeze::ZopfliLZ77OptimalFixed;
use crate::src::tree::ZopfliLengthsToSymbols;
pub use crate::src::c_inlined_fns::*;
pub use crate::src::ffi::*;
pub use crate::src::c_consts::*;
pub use crate::src::c_structs::*;
pub use crate::src::c_types::*;
pub use crate::src::c_extern_types::*;

pub const ZOPFLI_MASTER_BLOCK_SIZE: c_int = 1000000 as c_int;
unsafe fn AddBit(
    mut bit: c_int,
    mut bp: *mut c_uchar,
    mut out: *mut *mut c_uchar,
    mut outsize: *mut size_t,
) {
    let bp_view: &mut c_uchar = unsafe { &mut *bp };
    if *bp_view as c_int == 0 as c_int {
        if *outsize & (*outsize).wrapping_sub(1 as size_t) == 0 {
            *out = (if *outsize == 0 as size_t {
                malloc(::core::mem::size_of::<c_uchar>() as size_t)
            } else {
                realloc(
                    *out as *mut c_void,
                    (*outsize)
                        .wrapping_mul(2 as size_t)
                        .wrapping_mul(::core::mem::size_of::<c_uchar>() as size_t),
                )
            }) as *mut c_uchar;
        }
        *(*out).offset(*outsize as isize) = 0 as c_uchar;
        *outsize = (*outsize).wrapping_add(1);
    }
    let ref mut fresh3 = *(*out).offset((*outsize).wrapping_sub(1 as size_t) as isize);
    *fresh3 =
        (*fresh3 as c_int | bit << *bp_view as c_int) as c_uchar;
    *bp_view = (*bp_view as c_int + 1 as c_int & 7 as c_int)
        as c_uchar;
}
unsafe fn AddBits(
    mut symbol: c_uint,
    mut length: c_uint,
    mut bp: *mut c_uchar,
    mut out: *mut *mut c_uchar,
    mut outsize: *mut size_t,
) {
    let bp_view: &mut c_uchar = unsafe { &mut *bp };
    let mut i: c_uint = 0;
    i = 0 as c_uint;
    while i < length {
        let mut bit: c_uint = symbol >> i & 1 as c_uint;
        if *bp_view as c_int == 0 as c_int {
            if *outsize & (*outsize).wrapping_sub(1 as size_t) == 0 {
                *out = (if *outsize == 0 as size_t {
                    malloc(::core::mem::size_of::<c_uchar>() as size_t)
                } else {
                    realloc(
                        *out as *mut c_void,
                        (*outsize)
                            .wrapping_mul(2 as size_t)
                            .wrapping_mul(::core::mem::size_of::<c_uchar>() as size_t),
                    )
                }) as *mut c_uchar;
            }
            *(*out).offset(*outsize as isize) = 0 as c_uchar;
            *outsize = (*outsize).wrapping_add(1);
        }
        let ref mut fresh1 = *(*out).offset((*outsize).wrapping_sub(1 as size_t) as isize);
        *fresh1 = (*fresh1 as c_uint | bit << *bp_view as c_int)
            as c_uchar;
        *bp_view = (*bp_view as c_int + 1 as c_int & 7 as c_int)
            as c_uchar;
        i = i.wrapping_add(1);
    }
}
unsafe fn AddHuffmanBits(
    mut symbol: c_uint,
    mut length: c_uint,
    mut bp: *mut c_uchar,
    mut out: *mut *mut c_uchar,
    mut outsize: *mut size_t,
) {
    let bp_view: &mut c_uchar = unsafe { &mut *bp };
    let mut i: c_uint = 0;
    i = 0 as c_uint;
    while i < length {
        let mut bit: c_uint = symbol
            >> length
                .wrapping_sub(i)
                .wrapping_sub(1 as c_uint)
            & 1 as c_uint;
        if *bp_view as c_int == 0 as c_int {
            if *outsize & (*outsize).wrapping_sub(1 as size_t) == 0 {
                *out = (if *outsize == 0 as size_t {
                    malloc(::core::mem::size_of::<c_uchar>() as size_t)
                } else {
                    realloc(
                        *out as *mut c_void,
                        (*outsize)
                            .wrapping_mul(2 as size_t)
                            .wrapping_mul(::core::mem::size_of::<c_uchar>() as size_t),
                    )
                }) as *mut c_uchar;
            }
            *(*out).offset(*outsize as isize) = 0 as c_uchar;
            *outsize = (*outsize).wrapping_add(1);
        }
        let ref mut fresh0 = *(*out).offset((*outsize).wrapping_sub(1 as size_t) as isize);
        *fresh0 = (*fresh0 as c_uint | bit << *bp_view as c_int)
            as c_uchar;
        *bp_view = (*bp_view as c_int + 1 as c_int & 7 as c_int)
            as c_uchar;
        i = i.wrapping_add(1);
    }
}
unsafe fn PatchDistanceCodesForBuggyDecoders(mut d_lengths: *mut c_uint) {
    let mut num_dist_codes: c_int = 0 as c_int;
    let mut i: c_int = 0;
    i = 0 as c_int;
    while i < 30 as c_int {
        if *d_lengths.offset(i as isize) != 0 {
            num_dist_codes += 1;
        }
        if num_dist_codes >= 2 as c_int {
            return;
        }
        i += 1;
    }
    if num_dist_codes == 0 as c_int {
        let ref mut fresh2 = *d_lengths.offset(1 as c_int as isize);
        *fresh2 = 1 as c_uint;
        *d_lengths.offset(0 as c_int as isize) = *fresh2;
    } else if num_dist_codes == 1 as c_int {
        *d_lengths.offset(
            (if *d_lengths.offset(0 as c_int as isize) != 0 {
                1 as c_int
            } else {
                0 as c_int
            }) as isize,
        ) = 1 as c_uint;
    }
}
unsafe fn EncodeTree(
    mut ll_lengths: *const c_uint,
    mut d_lengths: *const c_uint,
    mut use_16: c_int,
    mut use_17: c_int,
    mut use_18: c_int,
    mut bp: *mut c_uchar,
    mut out: *mut *mut c_uchar,
    mut outsize: *mut size_t,
) -> size_t {
    let mut lld_total: c_uint = 0;
    let mut rle: *mut c_uint = ::core::ptr::null_mut::<c_uint>();
    let mut rle_bits: *mut c_uint = ::core::ptr::null_mut::<c_uint>();
    let mut rle_size: size_t = 0 as size_t;
    let mut rle_bits_size: size_t = 0 as size_t;
    let mut hlit: c_uint = 29 as c_uint;
    let mut hdist: c_uint = 29 as c_uint;
    let mut hclen: c_uint = 0;
    let mut hlit2: c_uint = 0;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut clcounts: [size_t; 19] = [0; 19];
    let mut clcl: [c_uint; 19] = [0; 19];
    let mut clsymbols: [c_uint; 19] = [0; 19];
    static mut order: [c_uint; 19] = [
        16 as c_int as c_uint,
        17 as c_int as c_uint,
        18 as c_int as c_uint,
        0 as c_int as c_uint,
        8 as c_int as c_uint,
        7 as c_int as c_uint,
        9 as c_int as c_uint,
        6 as c_int as c_uint,
        10 as c_int as c_uint,
        5 as c_int as c_uint,
        11 as c_int as c_uint,
        4 as c_int as c_uint,
        12 as c_int as c_uint,
        3 as c_int as c_uint,
        13 as c_int as c_uint,
        2 as c_int as c_uint,
        14 as c_int as c_uint,
        1 as c_int as c_uint,
        15 as c_int as c_uint,
    ];
    let mut size_only: c_int = out.is_null() as c_int;
    let mut result_size: size_t = 0 as size_t;
    i = 0 as size_t;
    while i < 19 as size_t {
        clcounts[i as usize] = 0 as size_t;
        i = i.wrapping_add(1);
    }
    while hlit > 0 as c_uint
        && *ll_lengths.offset(
            (257 as c_uint)
                .wrapping_add(hlit)
                .wrapping_sub(1 as c_uint) as isize,
        ) == 0 as c_uint
    {
        hlit = hlit.wrapping_sub(1);
    }
    while hdist > 0 as c_uint
        && *d_lengths.offset(
            (1 as c_uint)
                .wrapping_add(hdist)
                .wrapping_sub(1 as c_uint) as isize,
        ) == 0 as c_uint
    {
        hdist = hdist.wrapping_sub(1);
    }
    hlit2 = hlit.wrapping_add(257 as c_uint);
    lld_total = hlit2
        .wrapping_add(hdist)
        .wrapping_add(1 as c_uint);
    i = 0 as size_t;
    while i < lld_total as size_t {
        let mut symbol: c_uchar = (if i < hlit2 as size_t {
            *ll_lengths.offset(i as isize)
        } else {
            *d_lengths.offset(i.wrapping_sub(hlit2 as size_t) as isize)
        }) as c_uchar;
        let mut count: c_uint = 1 as c_uint;
        if use_16 != 0
            || symbol as c_int == 0 as c_int
                && (use_17 != 0 || use_18 != 0)
        {
            j = i.wrapping_add(1 as size_t);
            while j < lld_total as size_t
                && symbol as c_uint
                    == (if j < hlit2 as size_t {
                        *ll_lengths.offset(j as isize)
                    } else {
                        *d_lengths.offset(j.wrapping_sub(hlit2 as size_t) as isize)
                    })
            {
                count = count.wrapping_add(1);
                j = j.wrapping_add(1);
            }
        }
        i = i.wrapping_add(count.wrapping_sub(1 as c_uint) as size_t);
        if symbol as c_int == 0 as c_int
            && count >= 3 as c_uint
        {
            if use_18 != 0 {
                while count >= 11 as c_uint {
                    let mut count2: c_uint = if count > 138 as c_uint {
                        138 as c_uint
                    } else {
                        count
                    };
                    if size_only == 0 {
                        if rle_size & rle_size.wrapping_sub(1 as size_t) == 0 {
                            rle = (if rle_size == 0 as size_t {
                                malloc(::core::mem::size_of::<c_uint>() as size_t)
                            } else {
                                realloc(
                                    rle as *mut c_void,
                                    rle_size.wrapping_mul(2 as size_t).wrapping_mul(
                                        ::core::mem::size_of::<c_uint>() as size_t,
                                    ),
                                )
                            }) as *mut c_uint;
                        }
                        *rle.offset(rle_size as isize) = 18 as c_uint;
                        rle_size = rle_size.wrapping_add(1);
                        if rle_bits_size & rle_bits_size.wrapping_sub(1 as size_t) == 0 {
                            rle_bits = (if rle_bits_size == 0 as size_t {
                                malloc(::core::mem::size_of::<c_uint>() as size_t)
                            } else {
                                realloc(
                                    rle_bits as *mut c_void,
                                    rle_bits_size.wrapping_mul(2 as size_t).wrapping_mul(
                                        ::core::mem::size_of::<c_uint>() as size_t,
                                    ),
                                )
                            }) as *mut c_uint;
                        }
                        *rle_bits.offset(rle_bits_size as isize) =
                            count2.wrapping_sub(11 as c_uint);
                        rle_bits_size = rle_bits_size.wrapping_add(1);
                    }
                    clcounts[18 as c_int as usize] =
                        clcounts[18 as c_int as usize].wrapping_add(1);
                    count = count.wrapping_sub(count2);
                }
            }
            if use_17 != 0 {
                while count >= 3 as c_uint {
                    let mut count2_0: c_uint = if count > 10 as c_uint {
                        10 as c_uint
                    } else {
                        count
                    };
                    if size_only == 0 {
                        if rle_size & rle_size.wrapping_sub(1 as size_t) == 0 {
                            rle = (if rle_size == 0 as size_t {
                                malloc(::core::mem::size_of::<c_uint>() as size_t)
                            } else {
                                realloc(
                                    rle as *mut c_void,
                                    rle_size.wrapping_mul(2 as size_t).wrapping_mul(
                                        ::core::mem::size_of::<c_uint>() as size_t,
                                    ),
                                )
                            }) as *mut c_uint;
                        }
                        *rle.offset(rle_size as isize) = 17 as c_uint;
                        rle_size = rle_size.wrapping_add(1);
                        if rle_bits_size & rle_bits_size.wrapping_sub(1 as size_t) == 0 {
                            rle_bits = (if rle_bits_size == 0 as size_t {
                                malloc(::core::mem::size_of::<c_uint>() as size_t)
                            } else {
                                realloc(
                                    rle_bits as *mut c_void,
                                    rle_bits_size.wrapping_mul(2 as size_t).wrapping_mul(
                                        ::core::mem::size_of::<c_uint>() as size_t,
                                    ),
                                )
                            }) as *mut c_uint;
                        }
                        *rle_bits.offset(rle_bits_size as isize) =
                            count2_0.wrapping_sub(3 as c_uint);
                        rle_bits_size = rle_bits_size.wrapping_add(1);
                    }
                    clcounts[17 as c_int as usize] =
                        clcounts[17 as c_int as usize].wrapping_add(1);
                    count = count.wrapping_sub(count2_0);
                }
            }
        }
        if use_16 != 0 && count >= 4 as c_uint {
            count = count.wrapping_sub(1);
            clcounts[symbol as usize] = clcounts[symbol as usize].wrapping_add(1);
            if size_only == 0 {
                if rle_size & rle_size.wrapping_sub(1 as size_t) == 0 {
                    rle = (if rle_size == 0 as size_t {
                        malloc(::core::mem::size_of::<c_uint>() as size_t)
                    } else {
                        realloc(
                            rle as *mut c_void,
                            rle_size.wrapping_mul(2 as size_t).wrapping_mul(
                                ::core::mem::size_of::<c_uint>() as size_t,
                            ),
                        )
                    }) as *mut c_uint;
                }
                *rle.offset(rle_size as isize) = symbol as c_uint;
                rle_size = rle_size.wrapping_add(1);
                if rle_bits_size & rle_bits_size.wrapping_sub(1 as size_t) == 0 {
                    rle_bits = (if rle_bits_size == 0 as size_t {
                        malloc(::core::mem::size_of::<c_uint>() as size_t)
                    } else {
                        realloc(
                            rle_bits as *mut c_void,
                            rle_bits_size.wrapping_mul(2 as size_t).wrapping_mul(
                                ::core::mem::size_of::<c_uint>() as size_t,
                            ),
                        )
                    }) as *mut c_uint;
                }
                *rle_bits.offset(rle_bits_size as isize) = 0 as c_uint;
                rle_bits_size = rle_bits_size.wrapping_add(1);
            }
            while count >= 3 as c_uint {
                let mut count2_1: c_uint = if count > 6 as c_uint {
                    6 as c_uint
                } else {
                    count
                };
                if size_only == 0 {
                    if rle_size & rle_size.wrapping_sub(1 as size_t) == 0 {
                        rle = (if rle_size == 0 as size_t {
                            malloc(::core::mem::size_of::<c_uint>() as size_t)
                        } else {
                            realloc(
                                rle as *mut c_void,
                                rle_size.wrapping_mul(2 as size_t).wrapping_mul(
                                    ::core::mem::size_of::<c_uint>() as size_t,
                                ),
                            )
                        }) as *mut c_uint;
                    }
                    *rle.offset(rle_size as isize) = 16 as c_uint;
                    rle_size = rle_size.wrapping_add(1);
                    if rle_bits_size & rle_bits_size.wrapping_sub(1 as size_t) == 0 {
                        rle_bits = (if rle_bits_size == 0 as size_t {
                            malloc(::core::mem::size_of::<c_uint>() as size_t)
                        } else {
                            realloc(
                                rle_bits as *mut c_void,
                                rle_bits_size.wrapping_mul(2 as size_t).wrapping_mul(
                                    ::core::mem::size_of::<c_uint>() as size_t,
                                ),
                            )
                        }) as *mut c_uint;
                    }
                    *rle_bits.offset(rle_bits_size as isize) =
                        count2_1.wrapping_sub(3 as c_uint);
                    rle_bits_size = rle_bits_size.wrapping_add(1);
                }
                clcounts[16 as c_int as usize] =
                    clcounts[16 as c_int as usize].wrapping_add(1);
                count = count.wrapping_sub(count2_1);
            }
        }
        clcounts[symbol as usize] = clcounts[symbol as usize].wrapping_add(count as size_t);
        while count > 0 as c_uint {
            if size_only == 0 {
                if rle_size & rle_size.wrapping_sub(1 as size_t) == 0 {
                    rle = (if rle_size == 0 as size_t {
                        malloc(::core::mem::size_of::<c_uint>() as size_t)
                    } else {
                        realloc(
                            rle as *mut c_void,
                            rle_size.wrapping_mul(2 as size_t).wrapping_mul(
                                ::core::mem::size_of::<c_uint>() as size_t,
                            ),
                        )
                    }) as *mut c_uint;
                }
                *rle.offset(rle_size as isize) = symbol as c_uint;
                rle_size = rle_size.wrapping_add(1);
                if rle_bits_size & rle_bits_size.wrapping_sub(1 as size_t) == 0 {
                    rle_bits = (if rle_bits_size == 0 as size_t {
                        malloc(::core::mem::size_of::<c_uint>() as size_t)
                    } else {
                        realloc(
                            rle_bits as *mut c_void,
                            rle_bits_size.wrapping_mul(2 as size_t).wrapping_mul(
                                ::core::mem::size_of::<c_uint>() as size_t,
                            ),
                        )
                    }) as *mut c_uint;
                }
                *rle_bits.offset(rle_bits_size as isize) = 0 as c_uint;
                rle_bits_size = rle_bits_size.wrapping_add(1);
            }
            count = count.wrapping_sub(1);
        }
        i = i.wrapping_add(1);
    }
    ZopfliCalculateBitLengths(
        &raw mut clcounts as *mut size_t,
        19 as size_t,
        7 as c_int,
        &raw mut clcl as *mut c_uint,
    );
    if size_only == 0 {
        ZopfliLengthsToSymbols(
            &raw mut clcl as *mut c_uint,
            19 as size_t,
            7 as c_uint,
            &raw mut clsymbols as *mut c_uint,
        );
    }
    hclen = 15 as c_uint;
    while hclen > 0 as c_uint
        && clcounts[order[hclen
            .wrapping_add(4 as c_uint)
            .wrapping_sub(1 as c_uint) as usize] as usize]
            == 0 as size_t
    {
        hclen = hclen.wrapping_sub(1);
    }
    if size_only == 0 {
        AddBits(hlit, 5 as c_uint, bp, out, outsize);
        AddBits(hdist, 5 as c_uint, bp, out, outsize);
        AddBits(hclen, 4 as c_uint, bp, out, outsize);
        i = 0 as size_t;
        while i < hclen.wrapping_add(4 as c_uint) as size_t {
            AddBits(
                clcl[order[i as usize] as usize],
                3 as c_uint,
                bp,
                out,
                outsize,
            );
            i = i.wrapping_add(1);
        }
        i = 0 as size_t;
        while i < rle_size {
            let mut symbol_0: c_uint = clsymbols[*rle.offset(i as isize) as usize];
            AddHuffmanBits(
                symbol_0,
                clcl[*rle.offset(i as isize) as usize],
                bp,
                out,
                outsize,
            );
            if *rle.offset(i as isize) == 16 as c_uint {
                AddBits(
                    *rle_bits.offset(i as isize),
                    2 as c_uint,
                    bp,
                    out,
                    outsize,
                );
            } else if *rle.offset(i as isize) == 17 as c_uint {
                AddBits(
                    *rle_bits.offset(i as isize),
                    3 as c_uint,
                    bp,
                    out,
                    outsize,
                );
            } else if *rle.offset(i as isize) == 18 as c_uint {
                AddBits(
                    *rle_bits.offset(i as isize),
                    7 as c_uint,
                    bp,
                    out,
                    outsize,
                );
            }
            i = i.wrapping_add(1);
        }
    }
    result_size = result_size.wrapping_add(14 as size_t);
    result_size = result_size.wrapping_add(
        hclen
            .wrapping_add(4 as c_uint)
            .wrapping_mul(3 as c_uint) as size_t,
    );
    i = 0 as size_t;
    while i < 19 as size_t {
        result_size = result_size
            .wrapping_add((clcl[i as usize] as size_t).wrapping_mul(clcounts[i as usize]));
        i = i.wrapping_add(1);
    }
    result_size = result_size
        .wrapping_add(clcounts[16 as c_int as usize].wrapping_mul(2 as size_t));
    result_size = result_size
        .wrapping_add(clcounts[17 as c_int as usize].wrapping_mul(3 as size_t));
    result_size = result_size
        .wrapping_add(clcounts[18 as c_int as usize].wrapping_mul(7 as size_t));
    free(rle as *mut c_void);
    free(rle_bits as *mut c_void);
    return result_size;
}
unsafe fn AddDynamicTree(
    mut ll_lengths: *const c_uint,
    mut d_lengths: *const c_uint,
    mut bp: *mut c_uchar,
    mut out: *mut *mut c_uchar,
    mut outsize: *mut size_t,
) {
    let mut i: c_int = 0;
    let mut best: c_int = 0 as c_int;
    let mut bestsize: size_t = 0 as size_t;
    i = 0 as c_int;
    while i < 8 as c_int {
        let mut size: size_t = EncodeTree(
            ll_lengths,
            d_lengths,
            i & 1 as c_int,
            i & 2 as c_int,
            i & 4 as c_int,
            ::core::ptr::null_mut::<c_uchar>(),
            ::core::ptr::null_mut::<*mut c_uchar>(),
            ::core::ptr::null_mut::<size_t>(),
        );
        if bestsize == 0 as size_t || size < bestsize {
            bestsize = size;
            best = i;
        }
        i += 1;
    }
    EncodeTree(
        ll_lengths,
        d_lengths,
        best & 1 as c_int,
        best & 2 as c_int,
        best & 4 as c_int,
        bp,
        out,
        outsize,
    );
}
unsafe fn CalculateTreeSize(
    mut ll_lengths: *const c_uint,
    mut d_lengths: *const c_uint,
) -> size_t {
    let mut result: size_t = 0 as size_t;
    let mut i: c_int = 0;
    i = 0 as c_int;
    while i < 8 as c_int {
        let mut size: size_t = EncodeTree(
            ll_lengths,
            d_lengths,
            i & 1 as c_int,
            i & 2 as c_int,
            i & 4 as c_int,
            ::core::ptr::null_mut::<c_uchar>(),
            ::core::ptr::null_mut::<*mut c_uchar>(),
            ::core::ptr::null_mut::<size_t>(),
        );
        if result == 0 as size_t || size < result {
            result = size;
        }
        i += 1;
    }
    return result;
}
unsafe fn AddLZ77Data(
    mut lz77: *const ZopfliLZ77Store,
    mut lstart: size_t,
    mut lend: size_t,
    mut expected_data_size: size_t,
    mut ll_symbols: *const c_uint,
    mut ll_lengths: *const c_uint,
    mut d_symbols: *const c_uint,
    mut d_lengths: *const c_uint,
    mut bp: *mut c_uchar,
    mut out: *mut *mut c_uchar,
    mut outsize: *mut size_t,
) {
    let lz77_view: &ZopfliLZ77Store = unsafe { &*lz77 };
    let mut testlength: size_t = 0 as size_t;
    let mut i: size_t = 0;
    i = lstart;
    while i < lend {
        let mut dist: c_uint =
            *lz77_view.dists.offset(i as isize) as c_uint;
        let mut litlen: c_uint =
            *lz77_view.litlens.offset(i as isize) as c_uint;
        if dist == 0 as c_uint {
            AddHuffmanBits(
                *ll_symbols.offset(litlen as isize),
                *ll_lengths.offset(litlen as isize),
                bp,
                out,
                outsize,
            );
            testlength = testlength.wrapping_add(1);
        } else {
            let mut lls: c_uint =
                ZopfliGetLengthSymbol(litlen as c_int) as c_uint;
            let mut ds: c_uint =
                ZopfliGetDistSymbol(dist as c_int) as c_uint;
            AddHuffmanBits(
                *ll_symbols.offset(lls as isize),
                *ll_lengths.offset(lls as isize),
                bp,
                out,
                outsize,
            );
            AddBits(
                ZopfliGetLengthExtraBitsValue(litlen as c_int) as c_uint,
                ZopfliGetLengthExtraBits(litlen as c_int) as c_uint,
                bp,
                out,
                outsize,
            );
            AddHuffmanBits(
                *d_symbols.offset(ds as isize),
                *d_lengths.offset(ds as isize),
                bp,
                out,
                outsize,
            );
            AddBits(
                ZopfliGetDistExtraBitsValue(dist as c_int) as c_uint,
                ZopfliGetDistExtraBits(dist as c_int) as c_uint,
                bp,
                out,
                outsize,
            );
            testlength = testlength.wrapping_add(litlen as size_t);
        }
        i = i.wrapping_add(1);
    }
}
unsafe fn GetFixedTree(
    mut ll_lengths: *mut c_uint,
    mut d_lengths: *mut c_uint,
) {
    let d_lengths_view: &mut [c_uint] = unsafe { core::slice::from_raw_parts_mut(d_lengths, (32 as size_t) as usize) };
    let mut i: size_t = 0;
    i = 0 as size_t;
    while i < 144 as size_t {
        *ll_lengths.offset(i as isize) = 8 as c_uint;
        i = i.wrapping_add(1);
    }
    i = 144 as size_t;
    while i < 256 as size_t {
        *ll_lengths.offset(i as isize) = 9 as c_uint;
        i = i.wrapping_add(1);
    }
    i = 256 as size_t;
    while i < 280 as size_t {
        *ll_lengths.offset(i as isize) = 7 as c_uint;
        i = i.wrapping_add(1);
    }
    i = 280 as size_t;
    while i < 288 as size_t {
        *ll_lengths.offset(i as isize) = 8 as c_uint;
        i = i.wrapping_add(1);
    }
    i = 0 as size_t;
    while i < 32 as size_t {
        d_lengths_view[(i) as usize] = 5 as c_uint;
        i = i.wrapping_add(1);
    }
}
unsafe fn CalculateBlockSymbolSizeSmall(
    mut ll_lengths: *const c_uint,
    mut d_lengths: *const c_uint,
    mut lz77: *const ZopfliLZ77Store,
    mut lstart: size_t,
    mut lend: size_t,
) -> size_t {
    let lz77_view: &ZopfliLZ77Store = unsafe { &*lz77 };
    let mut result: size_t = 0 as size_t;
    let mut i: size_t = 0;
    i = lstart;
    while i < lend {
        if *lz77_view.dists.offset(i as isize) as c_int == 0 as c_int {
            result = result.wrapping_add(
                *ll_lengths.offset(*lz77_view.litlens.offset(i as isize) as isize) as size_t,
            );
        } else {
            let mut ll_symbol: c_int =
                ZopfliGetLengthSymbol(*lz77_view.litlens.offset(i as isize) as c_int);
            let mut d_symbol: c_int =
                ZopfliGetDistSymbol(*lz77_view.dists.offset(i as isize) as c_int);
            result = result.wrapping_add(*ll_lengths.offset(ll_symbol as isize) as size_t);
            result = result.wrapping_add(*d_lengths.offset(d_symbol as isize) as size_t);
            result = result.wrapping_add(ZopfliGetLengthSymbolExtraBits(ll_symbol) as size_t);
            result = result.wrapping_add(ZopfliGetDistSymbolExtraBits(d_symbol) as size_t);
        }
        i = i.wrapping_add(1);
    }
    result = result.wrapping_add(*ll_lengths.offset(256 as c_int as isize) as size_t);
    return result;
}
unsafe fn CalculateBlockSymbolSizeGivenCounts(
    mut ll_counts: *const size_t,
    mut d_counts: *const size_t,
    mut ll_lengths: *const c_uint,
    mut d_lengths: *const c_uint,
    mut lz77: *const ZopfliLZ77Store,
    mut lstart: size_t,
    mut lend: size_t,
) -> size_t {
    let d_counts_view: &[size_t] = unsafe { core::slice::from_raw_parts(d_counts, (30 as size_t) as usize) };
    let mut result: size_t = 0 as size_t;
    let mut i: size_t = 0;
    if lstart.wrapping_add((ZOPFLI_NUM_LL * 3 as c_int) as size_t) > lend {
        return CalculateBlockSymbolSizeSmall(ll_lengths, d_lengths, lz77, lstart, lend);
    } else {
        i = 0 as size_t;
        while i < 256 as size_t {
            result = result.wrapping_add(
                (*ll_lengths.offset(i as isize) as size_t)
                    .wrapping_mul(*ll_counts.offset(i as isize)),
            );
            i = i.wrapping_add(1);
        }
        i = 257 as size_t;
        while i < 286 as size_t {
            result = result.wrapping_add(
                (*ll_lengths.offset(i as isize) as size_t)
                    .wrapping_mul(*ll_counts.offset(i as isize)),
            );
            result = result.wrapping_add(
                (ZopfliGetLengthSymbolExtraBits(i as c_int) as size_t)
                    .wrapping_mul(*ll_counts.offset(i as isize)),
            );
            i = i.wrapping_add(1);
        }
        i = 0 as size_t;
        while i < 30 as size_t {
            result = result.wrapping_add(
                (*d_lengths.offset(i as isize) as size_t)
                    .wrapping_mul(d_counts_view[(i) as usize]),
            );
            result = result.wrapping_add(
                (ZopfliGetDistSymbolExtraBits(i as c_int) as size_t)
                    .wrapping_mul(d_counts_view[(i) as usize]),
            );
            i = i.wrapping_add(1);
        }
        result =
            result.wrapping_add(*ll_lengths.offset(256 as c_int as isize) as size_t);
        return result;
    };
}
unsafe fn CalculateBlockSymbolSize(
    mut ll_lengths: *const c_uint,
    mut d_lengths: *const c_uint,
    mut lz77: *const ZopfliLZ77Store,
    mut lstart: size_t,
    mut lend: size_t,
) -> size_t {
    if lstart.wrapping_add((ZOPFLI_NUM_LL * 3 as c_int) as size_t) > lend {
        return CalculateBlockSymbolSizeSmall(ll_lengths, d_lengths, lz77, lstart, lend);
    } else {
        let mut ll_counts: [size_t; 288] = [0; 288];
        let mut d_counts: [size_t; 32] = [0; 32];
        ZopfliLZ77GetHistogram(
            lz77,
            lstart,
            lend,
            &raw mut ll_counts as *mut size_t,
            &raw mut d_counts as *mut size_t,
        );
        return CalculateBlockSymbolSizeGivenCounts(
            &raw mut ll_counts as *mut size_t,
            &raw mut d_counts as *mut size_t,
            ll_lengths,
            d_lengths,
            lz77,
            lstart,
            lend,
        );
    };
}
fn AbsDiff(mut x: size_t, mut y: size_t) -> size_t { {
    if x > y {
        return x.wrapping_sub(y);
    } else {
        return y.wrapping_sub(x);
    };
} }
// Applied rules: [III②, III④, C3]
// Skipped rules: [C3.S1: Would require changing `counts: *mut size_t` to a reference/slice and therefore cross-function caller proof for a public function; body-local III④ + C3.S2 captures most of the payoff without signature churn.]
#[inline]
pub unsafe fn OptimizeHuffmanForRle(
    mut length: c_int,
    mut counts: *mut size_t,
) {
    let mut i: c_int = 0;
    let mut k: c_int = 0;
    let mut stride: c_int = 0;
    let mut symbol: size_t = 0;
    let mut sum: size_t = 0;
    let mut limit: size_t = 0;

    while length >= 0 as c_int {
        if length == 0 as c_int {
            return;
        }
        // SAFETY: caller provides `counts` for at least the current `length` elements;
        // while `length > 0`, index `length - 1` is in-bounds.
        let tail = unsafe { *counts.offset((length - 1 as c_int) as isize) };
        if tail != 0 as size_t {
            break;
        }
        length -= 1;
    }

    // SAFETY: after trimming, all dereferences in this function are guarded so that any
    // actual access into `counts` uses an index in `0..length`. Caller guarantees `counts`
    // points to `length` writable `size_t` elements, is non-null for that region, and no
    // Rust references alias that memory for the duration of this reborrow.
    let counts = unsafe { core::slice::from_raw_parts_mut(counts, length as usize) };
    let mut good_for_rle = vec![0 as c_int; length as usize];

    // SAFETY: `length > 0` here because the loop above returned on zero length.
    symbol = unsafe { *counts.get_unchecked(0) };
    stride = 0 as c_int;
    i = 0 as c_int;
    while i < length + 1 as c_int {
        let count_i = if i != length {
            // SAFETY: this arm only runs when `i < length`, so `i as usize` is in `0..counts.len()`.
            unsafe { *counts.get_unchecked(i as usize) }
        } else {
            0 as size_t
        };
        if i == length || count_i != symbol {
            if symbol == 0 as size_t && stride >= 5 as c_int
                || symbol != 0 as size_t && stride >= 7 as c_int
            {
                k = 0 as c_int;
                while k < stride {
                    let idx = (i - k - 1 as c_int) as usize;
                    // SAFETY: this loop runs only for `0 <= k < stride`, with `stride` being the
                    // current run length ending at `i - 1`; therefore `idx` ranges over the just-seen
                    // run and is within `0..length`, matching `good_for_rle.len()`.
                    unsafe {
                        *good_for_rle.get_unchecked_mut(idx) = 1 as c_int;
                    }
                    k += 1;
                }
            }
            stride = 1 as c_int;
            if i != length {
                symbol = count_i;
            }
        } else {
            stride += 1;
        }
        i += 1;
    }

    stride = 0 as c_int;
    // SAFETY: `length > 0` here because the early return above handled zero.
    limit = unsafe { *counts.get_unchecked(0) };
    sum = 0 as size_t;
    i = 0 as c_int;
    while i < length + 1 as c_int {
        let good_i = if i != length {
            // SAFETY: this arm only runs when `i < length`, so `i as usize` is in `0..good_for_rle.len()`.
            unsafe { *good_for_rle.get_unchecked(i as usize) }
        } else {
            0 as c_int
        };
        let count_i = if i != length {
            // SAFETY: this arm only runs when `i < length`, so `i as usize` is in `0..counts.len()`.
            unsafe { *counts.get_unchecked(i as usize) }
        } else {
            0 as size_t
        };
        if i == length || good_i != 0 || AbsDiff(count_i, limit) >= 4 as size_t {
            if stride >= 4 as c_int
                || stride >= 3 as c_int && sum == 0 as size_t
            {
                let mut count: c_int =
                    sum.wrapping_add((stride / 2 as c_int) as size_t)
                        .wrapping_div(stride as size_t) as c_int;
                if count < 1 as c_int {
                    count = 1 as c_int;
                }
                if sum == 0 as size_t {
                    count = 0 as c_int;
                }
                k = 0 as c_int;
                while k < stride {
                    let idx = (i - k - 1 as c_int) as usize;
                    // SAFETY: this loop writes back over the current completed stride ending at `i - 1`;
                    // as above, `idx` ranges over valid positions within `0..length`, so it is in-bounds
                    // for `counts`.
                    unsafe {
                        *counts.get_unchecked_mut(idx) = count as size_t;
                    }
                    k += 1;
                }
            }
            stride = 0 as c_int;
            sum = 0 as size_t;
            if i < length - 3 as c_int {
                let base = i as usize;
                // SAFETY: `i < length - 3` implies `base + 3 < length`, so `base..=base+3`
                // are all valid indices into `counts`.
                let (c0, c1, c2, c3) = unsafe {
                    (
                        *counts.get_unchecked(base),
                        *counts.get_unchecked(base + 1),
                        *counts.get_unchecked(base + 2),
                        *counts.get_unchecked(base + 3),
                    )
                };
                limit = c0
                    .wrapping_add(c1)
                    .wrapping_add(c2)
                    .wrapping_add(c3)
                    .wrapping_add(2 as size_t)
                    .wrapping_div(4 as size_t);
            } else if i < length {
                limit = count_i;
            } else {
                limit = 0 as size_t;
            }
        }
        stride += 1;
        if i != length {
            sum = sum.wrapping_add(count_i);
        }
        i += 1;
    }
}
unsafe fn TryOptimizeHuffmanForRle(
    mut lz77: *const ZopfliLZ77Store,
    mut lstart: size_t,
    mut lend: size_t,
    mut ll_counts: *const size_t,
    mut d_counts: *const size_t,
    mut ll_lengths: *mut c_uint,
    mut d_lengths: *mut c_uint,
) -> c_double {
    let mut ll_counts2: [size_t; 288] = [0; 288];
    let mut d_counts2: [size_t; 32] = [0; 32];
    let mut ll_lengths2: [c_uint; 288] = [0; 288];
    let mut d_lengths2: [c_uint; 32] = [0; 32];
    let mut treesize: c_double = 0.;
    let mut datasize: c_double = 0.;
    let mut treesize2: c_double = 0.;
    let mut datasize2: c_double = 0.;
    treesize = CalculateTreeSize(ll_lengths, d_lengths) as c_double;
    datasize = CalculateBlockSymbolSizeGivenCounts(
        ll_counts, d_counts, ll_lengths, d_lengths, lz77, lstart, lend,
    ) as c_double;
    memcpy(
        &raw mut ll_counts2 as *mut size_t as *mut c_void,
        ll_counts as *const c_void,
        ::core::mem::size_of::<[size_t; 288]>() as size_t,
    );
    memcpy(
        &raw mut d_counts2 as *mut size_t as *mut c_void,
        d_counts as *const c_void,
        ::core::mem::size_of::<[size_t; 32]>() as size_t,
    );
    OptimizeHuffmanForRle(ZOPFLI_NUM_LL, &raw mut ll_counts2 as *mut size_t);
    OptimizeHuffmanForRle(ZOPFLI_NUM_D, &raw mut d_counts2 as *mut size_t);
    ZopfliCalculateBitLengths(
        &raw mut ll_counts2 as *mut size_t,
        ZOPFLI_NUM_LL as size_t,
        15 as c_int,
        &raw mut ll_lengths2 as *mut c_uint,
    );
    ZopfliCalculateBitLengths(
        &raw mut d_counts2 as *mut size_t,
        ZOPFLI_NUM_D as size_t,
        15 as c_int,
        &raw mut d_lengths2 as *mut c_uint,
    );
    PatchDistanceCodesForBuggyDecoders(&raw mut d_lengths2 as *mut c_uint);
    treesize2 = CalculateTreeSize(
        &raw mut ll_lengths2 as *mut c_uint,
        &raw mut d_lengths2 as *mut c_uint,
    ) as c_double;
    datasize2 = CalculateBlockSymbolSizeGivenCounts(
        ll_counts,
        d_counts,
        &raw mut ll_lengths2 as *mut c_uint,
        &raw mut d_lengths2 as *mut c_uint,
        lz77,
        lstart,
        lend,
    ) as c_double;
    if treesize2 + datasize2 < treesize + datasize {
        memcpy(
            ll_lengths as *mut c_void,
            &raw mut ll_lengths2 as *mut c_uint as *const c_void,
            ::core::mem::size_of::<[c_uint; 288]>() as size_t,
        );
        memcpy(
            d_lengths as *mut c_void,
            &raw mut d_lengths2 as *mut c_uint as *const c_void,
            ::core::mem::size_of::<[c_uint; 32]>() as size_t,
        );
        return treesize2 + datasize2;
    }
    return treesize + datasize;
}
unsafe fn GetDynamicLengths(
    mut lz77: *const ZopfliLZ77Store,
    mut lstart: size_t,
    mut lend: size_t,
    mut ll_lengths: *mut c_uint,
    mut d_lengths: *mut c_uint,
) -> c_double {
    let mut ll_counts: [size_t; 288] = [0; 288];
    let mut d_counts: [size_t; 32] = [0; 32];
    ZopfliLZ77GetHistogram(
        lz77,
        lstart,
        lend,
        &raw mut ll_counts as *mut size_t,
        &raw mut d_counts as *mut size_t,
    );
    ll_counts[256 as c_int as usize] = 1 as size_t;
    ZopfliCalculateBitLengths(
        &raw mut ll_counts as *mut size_t,
        ZOPFLI_NUM_LL as size_t,
        15 as c_int,
        ll_lengths,
    );
    ZopfliCalculateBitLengths(
        &raw mut d_counts as *mut size_t,
        ZOPFLI_NUM_D as size_t,
        15 as c_int,
        d_lengths,
    );
    PatchDistanceCodesForBuggyDecoders(d_lengths);
    return TryOptimizeHuffmanForRle(
        lz77,
        lstart,
        lend,
        &raw mut ll_counts as *mut size_t,
        &raw mut d_counts as *mut size_t,
        ll_lengths,
        d_lengths,
    );
}
#[inline]
pub unsafe fn ZopfliCalculateBlockSize(
    mut lz77: *const ZopfliLZ77Store,
    mut lstart: size_t,
    mut lend: size_t,
    mut btype: c_int,
) -> c_double {
    let mut ll_lengths: [c_uint; 288] = [0; 288];
    let mut d_lengths: [c_uint; 32] = [0; 32];
    let mut result: c_double = 3 as c_int as c_double;
    if btype == 0 as c_int {
        let mut length: size_t = ZopfliLZ77GetByteRange(lz77, lstart, lend);
        let mut rem: size_t = length.wrapping_rem(65535 as size_t);
        let mut blocks: size_t = length.wrapping_div(65535 as size_t).wrapping_add(
            (if rem != 0 {
                1 as c_int
            } else {
                0 as c_int
            }) as size_t,
        );
        return blocks
            .wrapping_mul(5 as size_t)
            .wrapping_mul(8 as size_t)
            .wrapping_add(length.wrapping_mul(8 as size_t))
            as c_double;
    }
    if btype == 1 as c_int {
        GetFixedTree(
            &raw mut ll_lengths as *mut c_uint,
            &raw mut d_lengths as *mut c_uint,
        );
        result += CalculateBlockSymbolSize(
            &raw mut ll_lengths as *mut c_uint,
            &raw mut d_lengths as *mut c_uint,
            lz77,
            lstart,
            lend,
        ) as c_double;
    } else {
        result += GetDynamicLengths(
            lz77,
            lstart,
            lend,
            &raw mut ll_lengths as *mut c_uint,
            &raw mut d_lengths as *mut c_uint,
        );
    }
    return result;
}
#[inline]
pub unsafe fn ZopfliCalculateBlockSizeAutoType(
    mut lz77: *const ZopfliLZ77Store,
    mut lstart: size_t,
    mut lend: size_t,
) -> c_double {
    let lz77_view: &ZopfliLZ77Store = unsafe { &*lz77 };
    let mut uncompressedcost: c_double =
        ZopfliCalculateBlockSize(lz77, lstart, lend, 0 as c_int);
    let mut fixedcost: c_double = if lz77_view.size > 1000 as size_t {
        uncompressedcost
    } else {
        ZopfliCalculateBlockSize(lz77, lstart, lend, 1 as c_int)
    };
    let mut dyncost: c_double =
        ZopfliCalculateBlockSize(lz77, lstart, lend, 2 as c_int);
    return if uncompressedcost < fixedcost && uncompressedcost < dyncost {
        uncompressedcost
    } else if fixedcost < dyncost {
        fixedcost
    } else {
        dyncost
    };
}
unsafe fn AddNonCompressedBlock(
    mut options: *const ZopfliOptions,
    mut final_0: c_int,
    mut in_0: *const c_uchar,
    mut instart: size_t,
    mut inend: size_t,
    mut bp: *mut c_uchar,
    mut out: *mut *mut c_uchar,
    mut outsize: *mut size_t,
) {
    let mut pos: size_t = instart;
    loop {
        let mut i: size_t = 0;
        let mut blocksize: c_ushort = 65535 as c_ushort;
        let mut nlen: c_ushort = 0;
        let mut currentfinal: c_int = 0;
        if pos.wrapping_add(blocksize as size_t) > inend {
            blocksize = inend.wrapping_sub(pos) as c_ushort;
        }
        currentfinal = (pos.wrapping_add(blocksize as size_t) >= inend) as c_int;
        nlen = !(blocksize as c_int) as c_ushort;
        AddBit(
            (final_0 != 0 && currentfinal != 0) as c_int,
            bp,
            out,
            outsize,
        );
        AddBit(0 as c_int, bp, out, outsize);
        AddBit(0 as c_int, bp, out, outsize);
        *bp = 0 as c_uchar;
        if *outsize & (*outsize).wrapping_sub(1 as size_t) == 0 {
            *out = (if *outsize == 0 as size_t {
                malloc(::core::mem::size_of::<c_uchar>() as size_t)
            } else {
                realloc(
                    *out as *mut c_void,
                    (*outsize)
                        .wrapping_mul(2 as size_t)
                        .wrapping_mul(::core::mem::size_of::<c_uchar>() as size_t),
                )
            }) as *mut c_uchar;
        }
        *(*out).offset(*outsize as isize) =
            (blocksize as c_int % 256 as c_int) as c_uchar;
        *outsize = (*outsize).wrapping_add(1);
        if *outsize & (*outsize).wrapping_sub(1 as size_t) == 0 {
            *out = (if *outsize == 0 as size_t {
                malloc(::core::mem::size_of::<c_uchar>() as size_t)
            } else {
                realloc(
                    *out as *mut c_void,
                    (*outsize)
                        .wrapping_mul(2 as size_t)
                        .wrapping_mul(::core::mem::size_of::<c_uchar>() as size_t),
                )
            }) as *mut c_uchar;
        }
        *(*out).offset(*outsize as isize) =
            (blocksize as c_int / 256 as c_int
                % 256 as c_int) as c_uchar;
        *outsize = (*outsize).wrapping_add(1);
        if *outsize & (*outsize).wrapping_sub(1 as size_t) == 0 {
            *out = (if *outsize == 0 as size_t {
                malloc(::core::mem::size_of::<c_uchar>() as size_t)
            } else {
                realloc(
                    *out as *mut c_void,
                    (*outsize)
                        .wrapping_mul(2 as size_t)
                        .wrapping_mul(::core::mem::size_of::<c_uchar>() as size_t),
                )
            }) as *mut c_uchar;
        }
        *(*out).offset(*outsize as isize) =
            (nlen as c_int % 256 as c_int) as c_uchar;
        *outsize = (*outsize).wrapping_add(1);
        if *outsize & (*outsize).wrapping_sub(1 as size_t) == 0 {
            *out = (if *outsize == 0 as size_t {
                malloc(::core::mem::size_of::<c_uchar>() as size_t)
            } else {
                realloc(
                    *out as *mut c_void,
                    (*outsize)
                        .wrapping_mul(2 as size_t)
                        .wrapping_mul(::core::mem::size_of::<c_uchar>() as size_t),
                )
            }) as *mut c_uchar;
        }
        *(*out).offset(*outsize as isize) = (nlen as c_int / 256 as c_int
            % 256 as c_int)
            as c_uchar;
        *outsize = (*outsize).wrapping_add(1);
        i = 0 as size_t;
        while i < blocksize as size_t {
            if *outsize & (*outsize).wrapping_sub(1 as size_t) == 0 {
                *out = (if *outsize == 0 as size_t {
                    malloc(::core::mem::size_of::<c_uchar>() as size_t)
                } else {
                    realloc(
                        *out as *mut c_void,
                        (*outsize)
                            .wrapping_mul(2 as size_t)
                            .wrapping_mul(::core::mem::size_of::<c_uchar>() as size_t),
                    )
                }) as *mut c_uchar;
            }
            *(*out).offset(*outsize as isize) = *in_0.offset(pos.wrapping_add(i) as isize);
            *outsize = (*outsize).wrapping_add(1);
            i = i.wrapping_add(1);
        }
        if currentfinal != 0 {
            break;
        }
        pos = pos.wrapping_add(blocksize as size_t);
    }
}
unsafe fn AddLZ77Block(
    mut options: *const ZopfliOptions,
    mut btype: c_int,
    mut final_0: c_int,
    mut lz77: *const ZopfliLZ77Store,
    mut lstart: size_t,
    mut lend: size_t,
    mut expected_data_size: size_t,
    mut bp: *mut c_uchar,
    mut out: *mut *mut c_uchar,
    mut outsize: *mut size_t,
) {
    let options_view: &ZopfliOptions = unsafe { &*options };
    let mut ll_lengths: [c_uint; 288] = [0; 288];
    let mut d_lengths: [c_uint; 32] = [0; 32];
    let mut ll_symbols: [c_uint; 288] = [0; 288];
    let mut d_symbols: [c_uint; 32] = [0; 32];
    let mut detect_block_size: size_t = *outsize;
    let mut compressed_size: size_t = 0;
    let mut uncompressed_size: size_t = 0 as size_t;
    let mut i: size_t = 0;
    if btype == 0 as c_int {
        let mut length: size_t = ZopfliLZ77GetByteRange(lz77, lstart, lend);
        let mut pos: size_t = if lstart == lend {
            0 as size_t
        } else {
            *(*lz77).pos.offset(lstart as isize)
        };
        let mut end: size_t = pos.wrapping_add(length);
        AddNonCompressedBlock(options, final_0, (*lz77).data, pos, end, bp, out, outsize);
        return;
    }
    AddBit(final_0, bp, out, outsize);
    AddBit(btype & 1 as c_int, bp, out, outsize);
    AddBit(
        (btype & 2 as c_int) >> 1 as c_int,
        bp,
        out,
        outsize,
    );
    if btype == 1 as c_int {
        GetFixedTree(
            &raw mut ll_lengths as *mut c_uint,
            &raw mut d_lengths as *mut c_uint,
        );
    } else {
        let mut detect_tree_size: c_uint = 0;
        GetDynamicLengths(
            lz77,
            lstart,
            lend,
            &raw mut ll_lengths as *mut c_uint,
            &raw mut d_lengths as *mut c_uint,
        );
        detect_tree_size = *outsize as c_uint;
        AddDynamicTree(
            &raw mut ll_lengths as *mut c_uint,
            &raw mut d_lengths as *mut c_uint,
            bp,
            out,
            outsize,
        );
        if options_view.verbose != 0 {
            fprintf(
                stderr,
                b"treesize: %d\n\0" as *const u8 as *const c_char,
                (*outsize).wrapping_sub(detect_tree_size as size_t) as c_int,
            );
        }
    }
    ZopfliLengthsToSymbols(
        &raw mut ll_lengths as *mut c_uint,
        ZOPFLI_NUM_LL as size_t,
        15 as c_uint,
        &raw mut ll_symbols as *mut c_uint,
    );
    ZopfliLengthsToSymbols(
        &raw mut d_lengths as *mut c_uint,
        ZOPFLI_NUM_D as size_t,
        15 as c_uint,
        &raw mut d_symbols as *mut c_uint,
    );
    detect_block_size = *outsize;
    AddLZ77Data(
        lz77,
        lstart,
        lend,
        expected_data_size,
        &raw mut ll_symbols as *mut c_uint,
        &raw mut ll_lengths as *mut c_uint,
        &raw mut d_symbols as *mut c_uint,
        &raw mut d_lengths as *mut c_uint,
        bp,
        out,
        outsize,
    );
    AddHuffmanBits(
        ll_symbols[256 as c_int as usize],
        ll_lengths[256 as c_int as usize],
        bp,
        out,
        outsize,
    );
    i = lstart;
    while i < lend {
        uncompressed_size = uncompressed_size.wrapping_add(
            (if *(*lz77).dists.offset(i as isize) as c_int == 0 as c_int {
                1 as c_int
            } else {
                *(*lz77).litlens.offset(i as isize) as c_int
            }) as size_t,
        );
        i = i.wrapping_add(1);
    }
    compressed_size = (*outsize).wrapping_sub(detect_block_size);
    if options_view.verbose != 0 {
        fprintf(
            stderr,
            b"compressed block size: %d (%dk) (unc: %d)\n\0" as *const u8
                as *const c_char,
            compressed_size as c_int,
            compressed_size.wrapping_div(1024 as size_t) as c_int,
            uncompressed_size as c_int,
        );
    }
}
unsafe fn AddLZ77BlockAutoType(
    mut options: *const ZopfliOptions,
    mut final_0: c_int,
    mut lz77: *const ZopfliLZ77Store,
    mut lstart: size_t,
    mut lend: size_t,
    mut expected_data_size: size_t,
    mut bp: *mut c_uchar,
    mut out: *mut *mut c_uchar,
    mut outsize: *mut size_t,
) {
    let mut uncompressedcost: c_double =
        ZopfliCalculateBlockSize(lz77, lstart, lend, 0 as c_int);
    let mut fixedcost: c_double =
        ZopfliCalculateBlockSize(lz77, lstart, lend, 1 as c_int);
    let mut dyncost: c_double =
        ZopfliCalculateBlockSize(lz77, lstart, lend, 2 as c_int);
    let mut expensivefixed: c_int =
        ((*lz77).size < 1000 as size_t || fixedcost <= dyncost * 1.1f64) as c_int;
    let mut fixedstore: ZopfliLZ77Store = ZopfliLZ77Store {
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
    if lstart == lend {
        AddBits(
            final_0 as c_uint,
            1 as c_uint,
            bp,
            out,
            outsize,
        );
        AddBits(
            1 as c_uint,
            2 as c_uint,
            bp,
            out,
            outsize,
        );
        AddBits(
            0 as c_uint,
            7 as c_uint,
            bp,
            out,
            outsize,
        );
        return;
    }
    ZopfliInitLZ77Store((*lz77).data, &raw mut fixedstore);
    if expensivefixed != 0 {
        let mut instart: size_t = *(*lz77).pos.offset(lstart as isize);
        let mut inend: size_t = instart.wrapping_add(ZopfliLZ77GetByteRange(lz77, lstart, lend));
        let mut s: ZopfliBlockState = ZopfliBlockState {
            options: ::core::ptr::null::<ZopfliOptions>(),
            lmc: ::core::ptr::null_mut::<ZopfliLongestMatchCache>(),
            blockstart: 0,
            blockend: 0,
        };
        ZopfliInitBlockState(options, instart, inend, 1 as c_int, &raw mut s);
        ZopfliLZ77OptimalFixed(
            &raw mut s,
            (*lz77).data,
            instart,
            inend,
            &raw mut fixedstore,
        );
        fixedcost = ZopfliCalculateBlockSize(
            &raw mut fixedstore,
            0 as size_t,
            fixedstore.size,
            1 as c_int,
        );
        ZopfliCleanBlockState(&raw mut s);
    }
    if uncompressedcost < fixedcost && uncompressedcost < dyncost {
        AddLZ77Block(
            options,
            0 as c_int,
            final_0,
            lz77,
            lstart,
            lend,
            expected_data_size,
            bp,
            out,
            outsize,
        );
    } else if fixedcost < dyncost {
        if expensivefixed != 0 {
            AddLZ77Block(
                options,
                1 as c_int,
                final_0,
                &raw mut fixedstore,
                0 as size_t,
                fixedstore.size,
                expected_data_size,
                bp,
                out,
                outsize,
            );
        } else {
            AddLZ77Block(
                options,
                1 as c_int,
                final_0,
                lz77,
                lstart,
                lend,
                expected_data_size,
                bp,
                out,
                outsize,
            );
        }
    } else {
        AddLZ77Block(
            options,
            2 as c_int,
            final_0,
            lz77,
            lstart,
            lend,
            expected_data_size,
            bp,
            out,
            outsize,
        );
    }
    ZopfliCleanLZ77Store(&raw mut fixedstore);
}
#[inline]
pub unsafe fn ZopfliDeflatePart(
    mut options: *const ZopfliOptions,
    mut btype: c_int,
    mut final_0: c_int,
    mut in_0: *const c_uchar,
    mut instart: size_t,
    mut inend: size_t,
    mut bp: *mut c_uchar,
    mut out: *mut *mut c_uchar,
    mut outsize: *mut size_t,
) {
    let options_view: &ZopfliOptions = unsafe { &*options };
    let mut i: size_t = 0;
    let mut splitpoints_uncompressed: *mut size_t = ::core::ptr::null_mut::<size_t>();
    let mut npoints: size_t = 0 as size_t;
    let mut splitpoints: *mut size_t = ::core::ptr::null_mut::<size_t>();
    let mut totalcost: c_double = 0 as c_int as c_double;
    let mut lz77: ZopfliLZ77Store = ZopfliLZ77Store {
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
    if btype == 0 as c_int {
        AddNonCompressedBlock(options, final_0, in_0, instart, inend, bp, out, outsize);
        return;
    } else if btype == 1 as c_int {
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
        let mut s: ZopfliBlockState = ZopfliBlockState {
            options: ::core::ptr::null::<ZopfliOptions>(),
            lmc: ::core::ptr::null_mut::<ZopfliLongestMatchCache>(),
            blockstart: 0,
            blockend: 0,
        };
        ZopfliInitLZ77Store(in_0, &raw mut store);
        ZopfliInitBlockState(options, instart, inend, 1 as c_int, &raw mut s);
        ZopfliLZ77OptimalFixed(&raw mut s, in_0, instart, inend, &raw mut store);
        AddLZ77Block(
            options,
            btype,
            final_0,
            &raw mut store,
            0 as size_t,
            store.size,
            0 as size_t,
            bp,
            out,
            outsize,
        );
        ZopfliCleanBlockState(&raw mut s);
        ZopfliCleanLZ77Store(&raw mut store);
        return;
    }
    if options_view.blocksplitting != 0 {
        ZopfliBlockSplit(
            options,
            in_0,
            instart,
            inend,
            options_view.blocksplittingmax as size_t,
            &raw mut splitpoints_uncompressed,
            &raw mut npoints,
        );
        splitpoints = malloc((::core::mem::size_of::<size_t>() as size_t).wrapping_mul(npoints))
            as *mut size_t;
    }
    ZopfliInitLZ77Store(in_0, &raw mut lz77);
    i = 0 as size_t;
    while i <= npoints {
        let mut start: size_t = if i == 0 as size_t {
            instart
        } else {
            *splitpoints_uncompressed.offset(i.wrapping_sub(1 as size_t) as isize)
        };
        let mut end: size_t = if i == npoints {
            inend
        } else {
            *splitpoints_uncompressed.offset(i as isize)
        };
        let mut s_0: ZopfliBlockState = ZopfliBlockState {
            options: ::core::ptr::null::<ZopfliOptions>(),
            lmc: ::core::ptr::null_mut::<ZopfliLongestMatchCache>(),
            blockstart: 0,
            blockend: 0,
        };
        let mut store_0: ZopfliLZ77Store = ZopfliLZ77Store {
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
        ZopfliInitLZ77Store(in_0, &raw mut store_0);
        ZopfliInitBlockState(options, start, end, 1 as c_int, &raw mut s_0);
        ZopfliLZ77Optimal(
            &raw mut s_0,
            in_0,
            start,
            end,
            options_view.numiterations,
            &raw mut store_0,
        );
        totalcost += ZopfliCalculateBlockSizeAutoType(&raw mut store_0, 0 as size_t, store_0.size);
        ZopfliAppendLZ77Store(&raw mut store_0, &raw mut lz77);
        if i < npoints {
            *splitpoints.offset(i as isize) = lz77.size;
        }
        ZopfliCleanBlockState(&raw mut s_0);
        ZopfliCleanLZ77Store(&raw mut store_0);
        i = i.wrapping_add(1);
    }
    if options_view.blocksplitting != 0 && npoints > 1 as size_t {
        let mut splitpoints2: *mut size_t = ::core::ptr::null_mut::<size_t>();
        let mut npoints2: size_t = 0 as size_t;
        let mut totalcost2: c_double =
            0 as c_int as c_double;
        ZopfliBlockSplitLZ77(
            options,
            &raw mut lz77,
            options_view.blocksplittingmax as size_t,
            &raw mut splitpoints2,
            &raw mut npoints2,
        );
        i = 0 as size_t;
        while i <= npoints2 {
            let mut start_0: size_t = if i == 0 as size_t {
                0 as size_t
            } else {
                *splitpoints2.offset(i.wrapping_sub(1 as size_t) as isize)
            };
            let mut end_0: size_t = if i == npoints2 {
                lz77.size
            } else {
                *splitpoints2.offset(i as isize)
            };
            totalcost2 += ZopfliCalculateBlockSizeAutoType(&raw mut lz77, start_0, end_0);
            i = i.wrapping_add(1);
        }
        if totalcost2 < totalcost {
            free(splitpoints as *mut c_void);
            splitpoints = splitpoints2;
            npoints = npoints2;
        } else {
            free(splitpoints2 as *mut c_void);
        }
    }
    i = 0 as size_t;
    while i <= npoints {
        let mut start_1: size_t = if i == 0 as size_t {
            0 as size_t
        } else {
            *splitpoints.offset(i.wrapping_sub(1 as size_t) as isize)
        };
        let mut end_1: size_t = if i == npoints {
            lz77.size
        } else {
            *splitpoints.offset(i as isize)
        };
        AddLZ77BlockAutoType(
            options,
            (i == npoints && final_0 != 0) as c_int,
            &raw mut lz77,
            start_1,
            end_1,
            0 as size_t,
            bp,
            out,
            outsize,
        );
        i = i.wrapping_add(1);
    }
    ZopfliCleanLZ77Store(&raw mut lz77);
    free(splitpoints as *mut c_void);
    free(splitpoints_uncompressed as *mut c_void);
}
#[inline]
pub unsafe fn ZopfliDeflate(
    mut options: *const ZopfliOptions,
    mut btype: c_int,
    mut final_0: c_int,
    mut in_0: *const c_uchar,
    mut insize: size_t,
    mut bp: *mut c_uchar,
    mut out: *mut *mut c_uchar,
    mut outsize: *mut size_t,
) {
    let options_view: &ZopfliOptions = unsafe { &*options };
    let mut offset: size_t = *outsize;
    let mut i: size_t = 0 as size_t;
    loop {
        let mut masterfinal: c_int =
            (i.wrapping_add(ZOPFLI_MASTER_BLOCK_SIZE as size_t) >= insize) as c_int;
        let mut final2: c_int =
            (final_0 != 0 && masterfinal != 0) as c_int;
        let mut size: size_t = if masterfinal != 0 {
            insize.wrapping_sub(i)
        } else {
            ZOPFLI_MASTER_BLOCK_SIZE as size_t
        };
        ZopfliDeflatePart(
            options,
            btype,
            final2,
            in_0,
            i,
            i.wrapping_add(size),
            bp,
            out,
            outsize,
        );
        i = i.wrapping_add(size);
        if !(i < insize) {
            break;
        }
    }
    if options_view.verbose != 0 {
        fprintf(
            stderr,
            b"Original Size: %lu, Deflate: %lu, Compression: %f%% Removed\n\0" as *const u8
                as *const c_char,
            insize as c_ulong,
            (*outsize).wrapping_sub(offset) as c_ulong,
            100.0f64
                * insize.wrapping_sub((*outsize).wrapping_sub(offset)) as c_double
                / insize as c_double,
        );
    }
}

fn ZopfliGetDistExtraBitsValue(
    mut dist: c_int,
) -> c_int { {
    if dist < 5 as c_int {
        return 0 as c_int;
    } else {
        let mut l: c_int = 31 as c_int
            ^ ((dist - 1 as c_int) as c_uint).leading_zeros() as i32;
        return dist - (1 as c_int + ((1 as c_int) << l))
            & ((1 as c_int) << l - 1 as c_int) - 1 as c_int;
    };
} }

fn ZopfliGetLengthExtraBitsValue(
    mut l: c_int,
) -> c_int { unsafe {
    static mut table: [c_int; 259] = [
        0 as c_int,
        0 as c_int,
        0 as c_int,
        0 as c_int,
        0 as c_int,
        0 as c_int,
        0 as c_int,
        0 as c_int,
        0 as c_int,
        0 as c_int,
        0 as c_int,
        0 as c_int,
        1 as c_int,
        0 as c_int,
        1 as c_int,
        0 as c_int,
        1 as c_int,
        0 as c_int,
        1 as c_int,
        0 as c_int,
        1 as c_int,
        2 as c_int,
        3 as c_int,
        0 as c_int,
        1 as c_int,
        2 as c_int,
        3 as c_int,
        0 as c_int,
        1 as c_int,
        2 as c_int,
        3 as c_int,
        0 as c_int,
        1 as c_int,
        2 as c_int,
        3 as c_int,
        0 as c_int,
        1 as c_int,
        2 as c_int,
        3 as c_int,
        4 as c_int,
        5 as c_int,
        6 as c_int,
        7 as c_int,
        0 as c_int,
        1 as c_int,
        2 as c_int,
        3 as c_int,
        4 as c_int,
        5 as c_int,
        6 as c_int,
        7 as c_int,
        0 as c_int,
        1 as c_int,
        2 as c_int,
        3 as c_int,
        4 as c_int,
        5 as c_int,
        6 as c_int,
        7 as c_int,
        0 as c_int,
        1 as c_int,
        2 as c_int,
        3 as c_int,
        4 as c_int,
        5 as c_int,
        6 as c_int,
        7 as c_int,
        0 as c_int,
        1 as c_int,
        2 as c_int,
        3 as c_int,
        4 as c_int,
        5 as c_int,
        6 as c_int,
        7 as c_int,
        8 as c_int,
        9 as c_int,
        10 as c_int,
        11 as c_int,
        12 as c_int,
        13 as c_int,
        14 as c_int,
        15 as c_int,
        0 as c_int,
        1 as c_int,
        2 as c_int,
        3 as c_int,
        4 as c_int,
        5 as c_int,
        6 as c_int,
        7 as c_int,
        8 as c_int,
        9 as c_int,
        10 as c_int,
        11 as c_int,
        12 as c_int,
        13 as c_int,
        14 as c_int,
        15 as c_int,
        0 as c_int,
        1 as c_int,
        2 as c_int,
        3 as c_int,
        4 as c_int,
        5 as c_int,
        6 as c_int,
        7 as c_int,
        8 as c_int,
        9 as c_int,
        10 as c_int,
        11 as c_int,
        12 as c_int,
        13 as c_int,
        14 as c_int,
        15 as c_int,
        0 as c_int,
        1 as c_int,
        2 as c_int,
        3 as c_int,
        4 as c_int,
        5 as c_int,
        6 as c_int,
        7 as c_int,
        8 as c_int,
        9 as c_int,
        10 as c_int,
        11 as c_int,
        12 as c_int,
        13 as c_int,
        14 as c_int,
        15 as c_int,
        0 as c_int,
        1 as c_int,
        2 as c_int,
        3 as c_int,
        4 as c_int,
        5 as c_int,
        6 as c_int,
        7 as c_int,
        8 as c_int,
        9 as c_int,
        10 as c_int,
        11 as c_int,
        12 as c_int,
        13 as c_int,
        14 as c_int,
        15 as c_int,
        16 as c_int,
        17 as c_int,
        18 as c_int,
        19 as c_int,
        20 as c_int,
        21 as c_int,
        22 as c_int,
        23 as c_int,
        24 as c_int,
        25 as c_int,
        26 as c_int,
        27 as c_int,
        28 as c_int,
        29 as c_int,
        30 as c_int,
        31 as c_int,
        0 as c_int,
        1 as c_int,
        2 as c_int,
        3 as c_int,
        4 as c_int,
        5 as c_int,
        6 as c_int,
        7 as c_int,
        8 as c_int,
        9 as c_int,
        10 as c_int,
        11 as c_int,
        12 as c_int,
        13 as c_int,
        14 as c_int,
        15 as c_int,
        16 as c_int,
        17 as c_int,
        18 as c_int,
        19 as c_int,
        20 as c_int,
        21 as c_int,
        22 as c_int,
        23 as c_int,
        24 as c_int,
        25 as c_int,
        26 as c_int,
        27 as c_int,
        28 as c_int,
        29 as c_int,
        30 as c_int,
        31 as c_int,
        0 as c_int,
        1 as c_int,
        2 as c_int,
        3 as c_int,
        4 as c_int,
        5 as c_int,
        6 as c_int,
        7 as c_int,
        8 as c_int,
        9 as c_int,
        10 as c_int,
        11 as c_int,
        12 as c_int,
        13 as c_int,
        14 as c_int,
        15 as c_int,
        16 as c_int,
        17 as c_int,
        18 as c_int,
        19 as c_int,
        20 as c_int,
        21 as c_int,
        22 as c_int,
        23 as c_int,
        24 as c_int,
        25 as c_int,
        26 as c_int,
        27 as c_int,
        28 as c_int,
        29 as c_int,
        30 as c_int,
        31 as c_int,
        0 as c_int,
        1 as c_int,
        2 as c_int,
        3 as c_int,
        4 as c_int,
        5 as c_int,
        6 as c_int,
        7 as c_int,
        8 as c_int,
        9 as c_int,
        10 as c_int,
        11 as c_int,
        12 as c_int,
        13 as c_int,
        14 as c_int,
        15 as c_int,
        16 as c_int,
        17 as c_int,
        18 as c_int,
        19 as c_int,
        20 as c_int,
        21 as c_int,
        22 as c_int,
        23 as c_int,
        24 as c_int,
        25 as c_int,
        26 as c_int,
        27 as c_int,
        28 as c_int,
        29 as c_int,
        30 as c_int,
        0 as c_int,
    ];
    return table[l as usize];
} }

fn ZopfliGetLengthSymbolExtraBits(
    mut s: c_int,
) -> c_int { unsafe {
    static mut table: [c_int; 29] = [
        0 as c_int,
        0 as c_int,
        0 as c_int,
        0 as c_int,
        0 as c_int,
        0 as c_int,
        0 as c_int,
        0 as c_int,
        1 as c_int,
        1 as c_int,
        1 as c_int,
        1 as c_int,
        2 as c_int,
        2 as c_int,
        2 as c_int,
        2 as c_int,
        3 as c_int,
        3 as c_int,
        3 as c_int,
        3 as c_int,
        4 as c_int,
        4 as c_int,
        4 as c_int,
        4 as c_int,
        5 as c_int,
        5 as c_int,
        5 as c_int,
        5 as c_int,
        0 as c_int,
    ];
    return table[(s - 257 as c_int) as usize];
} }
fn ZopfliGetDistSymbolExtraBits(mut s: c_int) -> c_int { unsafe {
    static mut table: [c_int; 30] = [
        0 as c_int,
        0 as c_int,
        0 as c_int,
        0 as c_int,
        1 as c_int,
        1 as c_int,
        2 as c_int,
        2 as c_int,
        3 as c_int,
        3 as c_int,
        4 as c_int,
        4 as c_int,
        5 as c_int,
        5 as c_int,
        6 as c_int,
        6 as c_int,
        7 as c_int,
        7 as c_int,
        8 as c_int,
        8 as c_int,
        9 as c_int,
        9 as c_int,
        10 as c_int,
        10 as c_int,
        11 as c_int,
        11 as c_int,
        12 as c_int,
        12 as c_int,
        13 as c_int,
        13 as c_int,
    ];
    return table[s as usize];
} }
