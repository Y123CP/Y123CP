use core::ffi::*;
pub use crate::src::c_inlined_fns::*;
pub use crate::src::ffi::*;
pub use crate::src::c_consts::*;
pub use crate::src::c_structs::*;
pub use crate::src::c_types::*;

#[inline(always)]
unsafe extern "C" fn Hash(mut p: *const uint8_t, mut shift: size_t) -> uint32_t {
    let h: uint64_t = ((BrotliUnalignedRead64(p as *const c_void) as uint64_t)
        << 24 as c_int)
        .wrapping_mul(kHashMul32 as uint64_t);
    return (h >> shift) as uint32_t;
}
#[inline(always)]
unsafe extern "C" fn HashBytesAtOffset(
    mut v: uint64_t,
    mut offset: c_int,
    mut shift: size_t,
) -> uint32_t {
    let h: uint64_t = (v >> 8 as c_int * offset << 24 as c_int)
        .wrapping_mul(kHashMul32 as uint64_t);
    return (h >> shift) as uint32_t;
}
#[inline(always)]
unsafe extern "C" fn IsMatch(mut p1: *const uint8_t, mut p2: *const uint8_t) -> c_int {
    return if BrotliUnalignedRead32(p1 as *const c_void)
        == BrotliUnalignedRead32(p2 as *const c_void)
        && *p1.offset(4 as c_int as isize) as c_int
            == *p2.offset(4 as c_int as isize) as c_int
    {
        BROTLI_TRUE
    } else {
        BROTLI_FALSE
    };
}
unsafe extern "C" fn BuildAndStoreLiteralPrefixCode(
    mut s: *mut BrotliOnePassArena,
    mut input: *const uint8_t,
    input_size: size_t,
    mut depths: *mut uint8_t,
    mut bits: *mut uint16_t,
    mut storage_ix: *mut size_t,
    mut storage: *mut uint8_t,
) -> size_t {
    let histogram: *mut uint32_t = &raw mut (*s).histogram as *mut uint32_t;
    let mut histogram_total: size_t = 0;
    let mut i: size_t = 0;
    memset(
        histogram as *mut c_void,
        0 as c_int,
        ::core::mem::size_of::<[uint32_t; 256]>() as size_t,
    );
    if input_size < ((1 as c_int) << 15 as c_int) as size_t {
        i = 0 as size_t;
        while i < input_size {
            let ref mut fresh3 = *histogram.offset(*input.offset(i as isize) as isize);
            *fresh3 = (*fresh3).wrapping_add(1);
            i = i.wrapping_add(1);
        }
        histogram_total = input_size;
        i = 0 as size_t;
        while i < 256 as size_t {
            let adjust: uint32_t = (2 as uint32_t)
                .wrapping_mul(
                    brotli_min_uint32_t(*histogram.offset(i as isize), 11 as uint32_t) as uint32_t,
                );
            let ref mut fresh4 = *histogram.offset(i as isize);
            *fresh4 = (*fresh4 as c_uint).wrapping_add(adjust as c_uint)
                as uint32_t as uint32_t;
            histogram_total = (histogram_total as c_ulong)
                .wrapping_add(adjust as c_ulong)
                as size_t as size_t;
            i = i.wrapping_add(1);
        }
    } else {
        static mut kSampleRate: size_t = 29 as size_t;
        i = 0 as size_t;
        while i < input_size {
            let ref mut fresh5 = *histogram.offset(*input.offset(i as isize) as isize);
            *fresh5 = (*fresh5).wrapping_add(1);
            i = (i as c_ulong).wrapping_add(kSampleRate as c_ulong)
                as size_t as size_t;
        }
        histogram_total = input_size
            .wrapping_add(kSampleRate)
            .wrapping_sub(1 as size_t)
            .wrapping_div(kSampleRate);
        i = 0 as size_t;
        while i < 256 as size_t {
            let adjust_0: uint32_t = (1 as uint32_t).wrapping_add((2 as uint32_t).wrapping_mul(
                brotli_min_uint32_t(*histogram.offset(i as isize), 11 as uint32_t) as uint32_t,
            ));
            let ref mut fresh6 = *histogram.offset(i as isize);
            *fresh6 = (*fresh6 as c_uint).wrapping_add(adjust_0 as c_uint)
                as uint32_t as uint32_t;
            histogram_total = (histogram_total as c_ulong)
                .wrapping_add(adjust_0 as c_ulong)
                as size_t as size_t;
            i = i.wrapping_add(1);
        }
    }
    BrotliBuildAndStoreHuffmanTreeFast(
        &raw mut (*s).tree as *mut HuffmanTree,
        histogram,
        histogram_total,
        8 as size_t,
        depths as *mut uint8_t,
        bits as *mut uint16_t,
        storage_ix,
        storage,
    );
    let mut literal_ratio: size_t = 0 as size_t;
    i = 0 as size_t;
    while i < 256 as size_t {
        if *histogram.offset(i as isize) != 0 {
            literal_ratio = (literal_ratio as c_ulong).wrapping_add(
                (*histogram.offset(i as isize)).wrapping_mul(*depths.offset(i as isize) as uint32_t)
                    as c_ulong,
            ) as size_t as size_t;
        }
        i = i.wrapping_add(1);
    }
    return literal_ratio
        .wrapping_mul(125 as size_t)
        .wrapping_div(histogram_total);
}
unsafe extern "C" fn BuildAndStoreCommandPrefixCode(
    mut s: *mut BrotliOnePassArena,
    mut storage_ix: *mut size_t,
    mut storage: *mut uint8_t,
) {
    let histogram: *const uint32_t = &raw mut (*s).cmd_histo as *mut uint32_t;
    let depth: *mut uint8_t = &raw mut (*s).cmd_depth as *mut uint8_t;
    let bits: *mut uint16_t = &raw mut (*s).cmd_bits as *mut uint16_t;
    let tmp_depth: *mut uint8_t = &raw mut (*s).tmp_depth as *mut uint8_t;
    let tmp_bits: *mut uint16_t = &raw mut (*s).tmp_bits as *mut uint16_t;
    memset(
        tmp_depth as *mut c_void,
        0 as c_int,
        BROTLI_NUM_COMMAND_SYMBOLS as size_t,
    );
    BrotliCreateHuffmanTree(
        histogram,
        64 as size_t,
        15 as c_int,
        &raw mut (*s).tree as *mut HuffmanTree,
        depth,
    );
    BrotliCreateHuffmanTree(
        histogram.offset(64 as c_int as isize) as *const uint32_t,
        64 as size_t,
        14 as c_int,
        &raw mut (*s).tree as *mut HuffmanTree,
        depth.offset(64 as c_int as isize) as *mut uint8_t,
    );
    memcpy(
        tmp_depth as *mut c_void,
        depth as *const c_void,
        24 as size_t,
    );
    memcpy(
        tmp_depth.offset(24 as c_int as isize) as *mut c_void,
        depth.offset(40 as c_int as isize) as *const c_void,
        8 as size_t,
    );
    memcpy(
        tmp_depth.offset(32 as c_int as isize) as *mut c_void,
        depth.offset(24 as c_int as isize) as *const c_void,
        8 as size_t,
    );
    memcpy(
        tmp_depth.offset(40 as c_int as isize) as *mut c_void,
        depth.offset(48 as c_int as isize) as *const c_void,
        8 as size_t,
    );
    memcpy(
        tmp_depth.offset(48 as c_int as isize) as *mut c_void,
        depth.offset(32 as c_int as isize) as *const c_void,
        8 as size_t,
    );
    memcpy(
        tmp_depth.offset(56 as c_int as isize) as *mut c_void,
        depth.offset(56 as c_int as isize) as *const c_void,
        8 as size_t,
    );
    BrotliConvertBitDepthsToSymbols(tmp_depth, 64 as size_t, tmp_bits);
    memcpy(
        bits as *mut c_void,
        tmp_bits as *const c_void,
        48 as size_t,
    );
    memcpy(
        bits.offset(24 as c_int as isize) as *mut c_void,
        tmp_bits.offset(32 as c_int as isize) as *const c_void,
        16 as size_t,
    );
    memcpy(
        bits.offset(32 as c_int as isize) as *mut c_void,
        tmp_bits.offset(48 as c_int as isize) as *const c_void,
        16 as size_t,
    );
    memcpy(
        bits.offset(40 as c_int as isize) as *mut c_void,
        tmp_bits.offset(24 as c_int as isize) as *const c_void,
        16 as size_t,
    );
    memcpy(
        bits.offset(48 as c_int as isize) as *mut c_void,
        tmp_bits.offset(40 as c_int as isize) as *const c_void,
        16 as size_t,
    );
    memcpy(
        bits.offset(56 as c_int as isize) as *mut c_void,
        tmp_bits.offset(56 as c_int as isize) as *const c_void,
        16 as size_t,
    );
    BrotliConvertBitDepthsToSymbols(
        depth.offset(64 as c_int as isize) as *mut uint8_t,
        64 as size_t,
        bits.offset(64 as c_int as isize) as *mut uint16_t,
    );
    let mut i: size_t = 0;
    memset(
        tmp_depth as *mut c_void,
        0 as c_int,
        64 as size_t,
    );
    memcpy(
        tmp_depth as *mut c_void,
        depth as *const c_void,
        8 as size_t,
    );
    memcpy(
        tmp_depth.offset(64 as c_int as isize) as *mut c_void,
        depth.offset(8 as c_int as isize) as *const c_void,
        8 as size_t,
    );
    memcpy(
        tmp_depth.offset(128 as c_int as isize) as *mut c_void,
        depth.offset(16 as c_int as isize) as *const c_void,
        8 as size_t,
    );
    memcpy(
        tmp_depth.offset(192 as c_int as isize) as *mut c_void,
        depth.offset(24 as c_int as isize) as *const c_void,
        8 as size_t,
    );
    memcpy(
        tmp_depth.offset(384 as c_int as isize) as *mut c_void,
        depth.offset(32 as c_int as isize) as *const c_void,
        8 as size_t,
    );
    i = 0 as size_t;
    while i < 8 as size_t {
        *tmp_depth.offset((128 as size_t).wrapping_add((8 as size_t).wrapping_mul(i)) as isize) =
            *depth.offset((40 as size_t).wrapping_add(i) as isize);
        *tmp_depth.offset((256 as size_t).wrapping_add((8 as size_t).wrapping_mul(i)) as isize) =
            *depth.offset((48 as size_t).wrapping_add(i) as isize);
        *tmp_depth.offset((448 as size_t).wrapping_add((8 as size_t).wrapping_mul(i)) as isize) =
            *depth.offset((56 as size_t).wrapping_add(i) as isize);
        i = i.wrapping_add(1);
    }
    BrotliStoreHuffmanTree(
        tmp_depth,
        BROTLI_NUM_COMMAND_SYMBOLS as size_t,
        &raw mut (*s).tree as *mut HuffmanTree,
        storage_ix,
        storage,
    );
    BrotliStoreHuffmanTree(
        depth.offset(64 as c_int as isize) as *mut uint8_t,
        64 as size_t,
        &raw mut (*s).tree as *mut HuffmanTree,
        storage_ix,
        storage,
    );
}
#[inline(always)]
unsafe extern "C" fn EmitInsertLen(
    mut insertlen: size_t,
    mut depth: *const uint8_t,
    mut bits: *const uint16_t,
    mut histo: *mut uint32_t,
    mut storage_ix: *mut size_t,
    mut storage: *mut uint8_t,
) {
    if insertlen < 6 as size_t {
        let code: size_t = insertlen.wrapping_add(40 as size_t);
        BrotliWriteBits(
            *depth.offset(code as isize) as size_t,
            *bits.offset(code as isize) as uint64_t,
            storage_ix,
            storage,
        );
        let ref mut fresh9 = *histo.offset(code as isize);
        *fresh9 = (*fresh9).wrapping_add(1);
    } else if insertlen < 130 as size_t {
        let tail: size_t = insertlen.wrapping_sub(2 as size_t);
        let nbits: uint32_t = (Log2FloorNonZero(tail) as uint32_t).wrapping_sub(1 as uint32_t);
        let prefix: size_t = tail >> nbits;
        let inscode: size_t = ((nbits << 1 as c_int) as size_t)
            .wrapping_add(prefix)
            .wrapping_add(42 as size_t);
        BrotliWriteBits(
            *depth.offset(inscode as isize) as size_t,
            *bits.offset(inscode as isize) as uint64_t,
            storage_ix,
            storage,
        );
        BrotliWriteBits(
            nbits as size_t,
            (tail as uint64_t).wrapping_sub((prefix as uint64_t) << nbits),
            storage_ix,
            storage,
        );
        let ref mut fresh10 = *histo.offset(inscode as isize);
        *fresh10 = (*fresh10).wrapping_add(1);
    } else if insertlen < 2114 as size_t {
        let tail_0: size_t = insertlen.wrapping_sub(66 as size_t);
        let nbits_0: uint32_t = Log2FloorNonZero(tail_0) as uint32_t;
        let code_0: size_t = nbits_0.wrapping_add(50 as uint32_t) as size_t;
        BrotliWriteBits(
            *depth.offset(code_0 as isize) as size_t,
            *bits.offset(code_0 as isize) as uint64_t,
            storage_ix,
            storage,
        );
        BrotliWriteBits(
            nbits_0 as size_t,
            (tail_0 as uint64_t).wrapping_sub((1 as c_int as uint64_t) << nbits_0),
            storage_ix,
            storage,
        );
        let ref mut fresh11 = *histo.offset(code_0 as isize);
        *fresh11 = (*fresh11).wrapping_add(1);
    } else {
        BrotliWriteBits(
            *depth.offset(61 as c_int as isize) as size_t,
            *bits.offset(61 as c_int as isize) as uint64_t,
            storage_ix,
            storage,
        );
        BrotliWriteBits(
            12 as size_t,
            (insertlen as uint64_t).wrapping_sub(2114 as uint64_t),
            storage_ix,
            storage,
        );
        let ref mut fresh12 = *histo.offset(61 as c_int as isize);
        *fresh12 = (*fresh12).wrapping_add(1);
    };
}
#[inline(always)]
unsafe extern "C" fn EmitLongInsertLen(
    mut insertlen: size_t,
    mut depth: *const uint8_t,
    mut bits: *const uint16_t,
    mut histo: *mut uint32_t,
    mut storage_ix: *mut size_t,
    mut storage: *mut uint8_t,
) {
    if insertlen < 22594 as size_t {
        BrotliWriteBits(
            *depth.offset(62 as c_int as isize) as size_t,
            *bits.offset(62 as c_int as isize) as uint64_t,
            storage_ix,
            storage,
        );
        BrotliWriteBits(
            14 as size_t,
            (insertlen as uint64_t).wrapping_sub(6210 as uint64_t),
            storage_ix,
            storage,
        );
        let ref mut fresh7 = *histo.offset(62 as c_int as isize);
        *fresh7 = (*fresh7).wrapping_add(1);
    } else {
        BrotliWriteBits(
            *depth.offset(63 as c_int as isize) as size_t,
            *bits.offset(63 as c_int as isize) as uint64_t,
            storage_ix,
            storage,
        );
        BrotliWriteBits(
            24 as size_t,
            (insertlen as uint64_t).wrapping_sub(22594 as uint64_t),
            storage_ix,
            storage,
        );
        let ref mut fresh8 = *histo.offset(63 as c_int as isize);
        *fresh8 = (*fresh8).wrapping_add(1);
    };
}
#[inline(always)]
unsafe extern "C" fn EmitCopyLen(
    mut copylen: size_t,
    mut depth: *const uint8_t,
    mut bits: *const uint16_t,
    mut histo: *mut uint32_t,
    mut storage_ix: *mut size_t,
    mut storage: *mut uint8_t,
) {
    if copylen < 10 as size_t {
        BrotliWriteBits(
            *depth.offset(copylen.wrapping_add(14 as size_t) as isize) as size_t,
            *bits.offset(copylen.wrapping_add(14 as size_t) as isize) as uint64_t,
            storage_ix,
            storage,
        );
        let ref mut fresh15 = *histo.offset(copylen.wrapping_add(14 as size_t) as isize);
        *fresh15 = (*fresh15).wrapping_add(1);
    } else if copylen < 134 as size_t {
        let tail: size_t = copylen.wrapping_sub(6 as size_t);
        let nbits: uint32_t = (Log2FloorNonZero(tail) as uint32_t).wrapping_sub(1 as uint32_t);
        let prefix: size_t = tail >> nbits;
        let code: size_t = ((nbits << 1 as c_int) as size_t)
            .wrapping_add(prefix)
            .wrapping_add(20 as size_t);
        BrotliWriteBits(
            *depth.offset(code as isize) as size_t,
            *bits.offset(code as isize) as uint64_t,
            storage_ix,
            storage,
        );
        BrotliWriteBits(
            nbits as size_t,
            (tail as uint64_t).wrapping_sub((prefix as uint64_t) << nbits),
            storage_ix,
            storage,
        );
        let ref mut fresh16 = *histo.offset(code as isize);
        *fresh16 = (*fresh16).wrapping_add(1);
    } else if copylen < 2118 as size_t {
        let tail_0: size_t = copylen.wrapping_sub(70 as size_t);
        let nbits_0: uint32_t = Log2FloorNonZero(tail_0) as uint32_t;
        let code_0: size_t = nbits_0.wrapping_add(28 as uint32_t) as size_t;
        BrotliWriteBits(
            *depth.offset(code_0 as isize) as size_t,
            *bits.offset(code_0 as isize) as uint64_t,
            storage_ix,
            storage,
        );
        BrotliWriteBits(
            nbits_0 as size_t,
            (tail_0 as uint64_t).wrapping_sub((1 as c_int as uint64_t) << nbits_0),
            storage_ix,
            storage,
        );
        let ref mut fresh17 = *histo.offset(code_0 as isize);
        *fresh17 = (*fresh17).wrapping_add(1);
    } else {
        BrotliWriteBits(
            *depth.offset(39 as c_int as isize) as size_t,
            *bits.offset(39 as c_int as isize) as uint64_t,
            storage_ix,
            storage,
        );
        BrotliWriteBits(
            24 as size_t,
            (copylen as uint64_t).wrapping_sub(2118 as uint64_t),
            storage_ix,
            storage,
        );
        let ref mut fresh18 = *histo.offset(39 as c_int as isize);
        *fresh18 = (*fresh18).wrapping_add(1);
    };
}
#[inline(always)]
unsafe extern "C" fn EmitCopyLenLastDistance(
    mut copylen: size_t,
    mut depth: *const uint8_t,
    mut bits: *const uint16_t,
    mut histo: *mut uint32_t,
    mut storage_ix: *mut size_t,
    mut storage: *mut uint8_t,
) {
    if copylen < 12 as size_t {
        BrotliWriteBits(
            *depth.offset(copylen.wrapping_sub(4 as size_t) as isize) as size_t,
            *bits.offset(copylen.wrapping_sub(4 as size_t) as isize) as uint64_t,
            storage_ix,
            storage,
        );
        let ref mut fresh19 = *histo.offset(copylen.wrapping_sub(4 as size_t) as isize);
        *fresh19 = (*fresh19).wrapping_add(1);
    } else if copylen < 72 as size_t {
        let tail: size_t = copylen.wrapping_sub(8 as size_t);
        let nbits: uint32_t = (Log2FloorNonZero(tail) as uint32_t).wrapping_sub(1 as uint32_t);
        let prefix: size_t = tail >> nbits;
        let code: size_t = ((nbits << 1 as c_int) as size_t)
            .wrapping_add(prefix)
            .wrapping_add(4 as size_t);
        BrotliWriteBits(
            *depth.offset(code as isize) as size_t,
            *bits.offset(code as isize) as uint64_t,
            storage_ix,
            storage,
        );
        BrotliWriteBits(
            nbits as size_t,
            (tail as uint64_t).wrapping_sub((prefix as uint64_t) << nbits),
            storage_ix,
            storage,
        );
        let ref mut fresh20 = *histo.offset(code as isize);
        *fresh20 = (*fresh20).wrapping_add(1);
    } else if copylen < 136 as size_t {
        let tail_0: size_t = copylen.wrapping_sub(8 as size_t);
        let code_0: size_t = (tail_0 >> 5 as c_int).wrapping_add(30 as size_t);
        BrotliWriteBits(
            *depth.offset(code_0 as isize) as size_t,
            *bits.offset(code_0 as isize) as uint64_t,
            storage_ix,
            storage,
        );
        BrotliWriteBits(
            5 as size_t,
            tail_0 as uint64_t & 31 as uint64_t,
            storage_ix,
            storage,
        );
        BrotliWriteBits(
            *depth.offset(64 as c_int as isize) as size_t,
            *bits.offset(64 as c_int as isize) as uint64_t,
            storage_ix,
            storage,
        );
        let ref mut fresh21 = *histo.offset(code_0 as isize);
        *fresh21 = (*fresh21).wrapping_add(1);
        let ref mut fresh22 = *histo.offset(64 as c_int as isize);
        *fresh22 = (*fresh22).wrapping_add(1);
    } else if copylen < 2120 as size_t {
        let tail_1: size_t = copylen.wrapping_sub(72 as size_t);
        let nbits_0: uint32_t = Log2FloorNonZero(tail_1) as uint32_t;
        let code_1: size_t = nbits_0.wrapping_add(28 as uint32_t) as size_t;
        BrotliWriteBits(
            *depth.offset(code_1 as isize) as size_t,
            *bits.offset(code_1 as isize) as uint64_t,
            storage_ix,
            storage,
        );
        BrotliWriteBits(
            nbits_0 as size_t,
            (tail_1 as uint64_t).wrapping_sub((1 as c_int as uint64_t) << nbits_0),
            storage_ix,
            storage,
        );
        BrotliWriteBits(
            *depth.offset(64 as c_int as isize) as size_t,
            *bits.offset(64 as c_int as isize) as uint64_t,
            storage_ix,
            storage,
        );
        let ref mut fresh23 = *histo.offset(code_1 as isize);
        *fresh23 = (*fresh23).wrapping_add(1);
        let ref mut fresh24 = *histo.offset(64 as c_int as isize);
        *fresh24 = (*fresh24).wrapping_add(1);
    } else {
        BrotliWriteBits(
            *depth.offset(39 as c_int as isize) as size_t,
            *bits.offset(39 as c_int as isize) as uint64_t,
            storage_ix,
            storage,
        );
        BrotliWriteBits(
            24 as size_t,
            (copylen as uint64_t).wrapping_sub(2120 as uint64_t),
            storage_ix,
            storage,
        );
        BrotliWriteBits(
            *depth.offset(64 as c_int as isize) as size_t,
            *bits.offset(64 as c_int as isize) as uint64_t,
            storage_ix,
            storage,
        );
        let ref mut fresh25 = *histo.offset(39 as c_int as isize);
        *fresh25 = (*fresh25).wrapping_add(1);
        let ref mut fresh26 = *histo.offset(64 as c_int as isize);
        *fresh26 = (*fresh26).wrapping_add(1);
    };
}
#[inline(always)]
unsafe extern "C" fn EmitDistance(
    mut distance: size_t,
    mut depth: *const uint8_t,
    mut bits: *const uint16_t,
    mut histo: *mut uint32_t,
    mut storage_ix: *mut size_t,
    mut storage: *mut uint8_t,
) {
    let d: size_t = distance.wrapping_add(3 as size_t);
    let nbits: uint32_t = (Log2FloorNonZero(d) as uint32_t).wrapping_sub(1 as uint32_t);
    let prefix: size_t = d >> nbits & 1 as size_t;
    let offset: size_t = (2 as size_t).wrapping_add(prefix) << nbits;
    let distcode: size_t = ((2 as uint32_t).wrapping_mul(nbits.wrapping_sub(1 as uint32_t))
        as size_t)
        .wrapping_add(prefix)
        .wrapping_add(80 as size_t);
    BrotliWriteBits(
        *depth.offset(distcode as isize) as size_t,
        *bits.offset(distcode as isize) as uint64_t,
        storage_ix,
        storage,
    );
    BrotliWriteBits(
        nbits as size_t,
        (d as uint64_t).wrapping_sub(offset as uint64_t),
        storage_ix,
        storage,
    );
    let ref mut fresh14 = *histo.offset(distcode as isize);
    *fresh14 = (*fresh14).wrapping_add(1);
}
#[inline(always)]
unsafe extern "C" fn EmitLiterals(
    mut input: *const uint8_t,
    len: size_t,
    mut depth: *const uint8_t,
    mut bits: *const uint16_t,
    mut storage_ix: *mut size_t,
    mut storage: *mut uint8_t,
) {
    let mut j: size_t = 0;
    j = 0 as size_t;
    while j < len {
        let lit: uint8_t = *input.offset(j as isize);
        BrotliWriteBits(
            *depth.offset(lit as isize) as size_t,
            *bits.offset(lit as isize) as uint64_t,
            storage_ix,
            storage,
        );
        j = j.wrapping_add(1);
    }
}
unsafe extern "C" fn BrotliStoreMetaBlockHeader(
    mut len: size_t,
    mut is_uncompressed: c_int,
    mut storage_ix: *mut size_t,
    mut storage: *mut uint8_t,
) {
    let mut nibbles: size_t = 6 as size_t;
    BrotliWriteBits(1 as size_t, 0 as uint64_t, storage_ix, storage);
    if len <= ((1 as c_uint) << 16 as c_int) as size_t {
        nibbles = 4 as size_t;
    } else if len <= ((1 as c_uint) << 20 as c_int) as size_t {
        nibbles = 5 as size_t;
    }
    BrotliWriteBits(
        2 as size_t,
        (nibbles as uint64_t).wrapping_sub(4 as uint64_t),
        storage_ix,
        storage,
    );
    BrotliWriteBits(
        nibbles.wrapping_mul(4 as size_t),
        (len as uint64_t).wrapping_sub(1 as uint64_t),
        storage_ix,
        storage,
    );
    BrotliWriteBits(
        1 as size_t,
        is_uncompressed as uint64_t,
        storage_ix,
        storage,
    );
}
unsafe extern "C" fn UpdateBits(
    mut n_bits: size_t,
    mut bits: uint32_t,
    mut pos: size_t,
    mut array: *mut uint8_t,
) {
    while n_bits > 0 as size_t {
        let mut byte_pos: size_t = pos >> 3 as c_int;
        let mut n_unchanged_bits: size_t = pos & 7 as size_t;
        let mut n_changed_bits: size_t =
            brotli_min_size_t(n_bits, (8 as size_t).wrapping_sub(n_unchanged_bits));
        let mut total_bits: size_t = n_unchanged_bits.wrapping_add(n_changed_bits);
        let mut mask: uint32_t = !((1 as uint32_t) << total_bits).wrapping_sub(1 as uint32_t)
            | ((1 as uint32_t) << n_unchanged_bits).wrapping_sub(1 as uint32_t);
        let mut unchanged_bits: uint32_t = *array.offset(byte_pos as isize) as uint32_t & mask;
        let mut changed_bits: uint32_t =
            bits & ((1 as uint32_t) << n_changed_bits).wrapping_sub(1 as uint32_t);
        *array.offset(byte_pos as isize) =
            (changed_bits << n_unchanged_bits | unchanged_bits) as uint8_t;
        n_bits = (n_bits as c_ulong)
            .wrapping_sub(n_changed_bits as c_ulong) as size_t
            as size_t;
        bits >>= n_changed_bits;
        pos = (pos as c_ulong).wrapping_add(n_changed_bits as c_ulong)
            as size_t as size_t;
    }
}

unsafe extern "C" fn ShouldMergeBlock(
    mut s: *mut BrotliOnePassArena,
    mut data: *const uint8_t,
    mut len: size_t,
    mut depths: *const uint8_t,
) -> c_int {
    let histo: *mut uint32_t = &raw mut (*s).histogram as *mut uint32_t;
    static mut kSampleRate: size_t = 43 as size_t;
    let mut i: size_t = 0;
    memset(
        histo as *mut c_void,
        0 as c_int,
        ::core::mem::size_of::<[uint32_t; 256]>() as size_t,
    );
    i = 0 as size_t;
    while i < len {
        let ref mut fresh13 = *histo.offset(*data.offset(i as isize) as isize);
        *fresh13 = (*fresh13).wrapping_add(1);
        i = (i as c_ulong).wrapping_add(kSampleRate as c_ulong) as size_t
            as size_t;
    }
    let total: size_t = len
        .wrapping_add(kSampleRate)
        .wrapping_sub(1 as size_t)
        .wrapping_div(kSampleRate);
    let mut r: c_double = (FastLog2(total) + 0.5f64) * total as c_double
        + 200 as c_int as c_double;
    i = 0 as size_t;
    while i < 256 as size_t {
        r -= *histo.offset(i as isize) as c_double
            * (*depths.offset(i as isize) as c_int as c_double
                + FastLog2(*histo.offset(i as isize) as size_t));
        i = i.wrapping_add(1);
    }
    return if r >= 0.0f64 {
        BROTLI_TRUE
    } else {
        BROTLI_FALSE
    };
}
#[inline(always)]
unsafe extern "C" fn ShouldUseUncompressedMode(
    mut metablock_start: *const uint8_t,
    mut next_emit: *const uint8_t,
    insertlen: size_t,
    literal_ratio: size_t,
) -> c_int {
    let compressed: size_t =
        next_emit.offset_from(metablock_start) as c_long as size_t;
    if compressed.wrapping_mul(50 as size_t) > insertlen {
        return BROTLI_FALSE;
    } else {
        return if literal_ratio > 980 as size_t {
            BROTLI_TRUE
        } else {
            BROTLI_FALSE
        };
    };
}
unsafe extern "C" fn EmitUncompressedMetaBlock(
    mut begin: *const uint8_t,
    mut end: *const uint8_t,
    storage_ix_start: size_t,
    mut storage_ix: *mut size_t,
    mut storage: *mut uint8_t,
) {
    let len: size_t = end.offset_from(begin) as c_long as size_t;
    RewindBitPosition(storage_ix_start, storage_ix, storage);
    BrotliStoreMetaBlockHeader(len, 1 as c_int, storage_ix, storage);
    *storage_ix = (*storage_ix).wrapping_add(7 as size_t) & !(7 as c_uint) as size_t;
    memcpy(
        storage.offset((*storage_ix >> 3 as c_int) as isize) as *mut uint8_t
            as *mut c_void,
        begin as *const c_void,
        len,
    );
    *storage_ix = (*storage_ix as c_ulong)
        .wrapping_add((len << 3 as c_int) as c_ulong)
        as size_t as size_t;
    *storage.offset((*storage_ix >> 3 as c_int) as isize) = 0 as uint8_t;
}
static mut kCmdHistoSeed: [uint32_t; 128] = [
    0 as c_int as uint32_t,
    1 as c_int as uint32_t,
    1 as c_int as uint32_t,
    1 as c_int as uint32_t,
    1 as c_int as uint32_t,
    1 as c_int as uint32_t,
    1 as c_int as uint32_t,
    1 as c_int as uint32_t,
    1 as c_int as uint32_t,
    1 as c_int as uint32_t,
    1 as c_int as uint32_t,
    1 as c_int as uint32_t,
    1 as c_int as uint32_t,
    1 as c_int as uint32_t,
    1 as c_int as uint32_t,
    1 as c_int as uint32_t,
    0 as c_int as uint32_t,
    0 as c_int as uint32_t,
    0 as c_int as uint32_t,
    1 as c_int as uint32_t,
    1 as c_int as uint32_t,
    1 as c_int as uint32_t,
    1 as c_int as uint32_t,
    1 as c_int as uint32_t,
    1 as c_int as uint32_t,
    1 as c_int as uint32_t,
    1 as c_int as uint32_t,
    1 as c_int as uint32_t,
    1 as c_int as uint32_t,
    1 as c_int as uint32_t,
    1 as c_int as uint32_t,
    1 as c_int as uint32_t,
    1 as c_int as uint32_t,
    1 as c_int as uint32_t,
    1 as c_int as uint32_t,
    1 as c_int as uint32_t,
    1 as c_int as uint32_t,
    1 as c_int as uint32_t,
    1 as c_int as uint32_t,
    1 as c_int as uint32_t,
    0 as c_int as uint32_t,
    1 as c_int as uint32_t,
    1 as c_int as uint32_t,
    1 as c_int as uint32_t,
    1 as c_int as uint32_t,
    1 as c_int as uint32_t,
    1 as c_int as uint32_t,
    1 as c_int as uint32_t,
    1 as c_int as uint32_t,
    1 as c_int as uint32_t,
    1 as c_int as uint32_t,
    1 as c_int as uint32_t,
    1 as c_int as uint32_t,
    1 as c_int as uint32_t,
    1 as c_int as uint32_t,
    1 as c_int as uint32_t,
    1 as c_int as uint32_t,
    1 as c_int as uint32_t,
    1 as c_int as uint32_t,
    1 as c_int as uint32_t,
    1 as c_int as uint32_t,
    1 as c_int as uint32_t,
    1 as c_int as uint32_t,
    1 as c_int as uint32_t,
    1 as c_int as uint32_t,
    0 as c_int as uint32_t,
    0 as c_int as uint32_t,
    0 as c_int as uint32_t,
    0 as c_int as uint32_t,
    0 as c_int as uint32_t,
    0 as c_int as uint32_t,
    0 as c_int as uint32_t,
    0 as c_int as uint32_t,
    0 as c_int as uint32_t,
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
    1 as c_int as uint32_t,
    1 as c_int as uint32_t,
    1 as c_int as uint32_t,
    1 as c_int as uint32_t,
    1 as c_int as uint32_t,
    1 as c_int as uint32_t,
    1 as c_int as uint32_t,
    1 as c_int as uint32_t,
    1 as c_int as uint32_t,
    1 as c_int as uint32_t,
    1 as c_int as uint32_t,
    1 as c_int as uint32_t,
    1 as c_int as uint32_t,
    1 as c_int as uint32_t,
    1 as c_int as uint32_t,
    1 as c_int as uint32_t,
    1 as c_int as uint32_t,
    1 as c_int as uint32_t,
    1 as c_int as uint32_t,
    1 as c_int as uint32_t,
    1 as c_int as uint32_t,
    1 as c_int as uint32_t,
    1 as c_int as uint32_t,
    1 as c_int as uint32_t,
    1 as c_int as uint32_t,
    1 as c_int as uint32_t,
    1 as c_int as uint32_t,
    1 as c_int as uint32_t,
    1 as c_int as uint32_t,
    1 as c_int as uint32_t,
    1 as c_int as uint32_t,
    1 as c_int as uint32_t,
    1 as c_int as uint32_t,
    1 as c_int as uint32_t,
    1 as c_int as uint32_t,
    1 as c_int as uint32_t,
    1 as c_int as uint32_t,
    1 as c_int as uint32_t,
    0 as c_int as uint32_t,
    0 as c_int as uint32_t,
    0 as c_int as uint32_t,
    0 as c_int as uint32_t,
];
#[inline(always)]
unsafe extern "C" fn BrotliCompressFragmentFastImpl(
    mut s: *mut BrotliOnePassArena,
    mut input: *const uint8_t,
    mut input_size: size_t,
    mut is_last: c_int,
    mut table: *mut c_int,
    mut table_bits: size_t,
    mut storage_ix: *mut size_t,
    mut storage: *mut uint8_t,
) {
    let mut current_block: u64;
    let cmd_depth: *mut uint8_t = &raw mut (*s).cmd_depth as *mut uint8_t;
    let cmd_bits: *mut uint16_t = &raw mut (*s).cmd_bits as *mut uint16_t;
    let cmd_histo: *mut uint32_t = &raw mut (*s).cmd_histo as *mut uint32_t;
    let lit_depth: *mut uint8_t = &raw mut (*s).lit_depth as *mut uint8_t;
    let lit_bits: *mut uint16_t = &raw mut (*s).lit_bits as *mut uint16_t;
    let mut ip_end: *const uint8_t = ::core::ptr::null::<uint8_t>();
    let mut next_emit: *const uint8_t = input;
    let mut base_ip: *const uint8_t = input;
    static mut kFirstBlockSize: size_t =
        ((3 as c_int) << 15 as c_int) as size_t;
    static mut kMergeBlockSize: size_t =
        ((1 as c_int) << 16 as c_int) as size_t;
    let kInputMarginBytes: size_t = BROTLI_WINDOW_GAP as size_t;
    let kMinMatchLen: size_t = 5 as size_t;
    let mut metablock_start: *const uint8_t = input;
    let mut block_size: size_t = brotli_min_size_t(input_size, kFirstBlockSize);
    let mut total_block_size: size_t = block_size;
    let mut mlen_storage_ix: size_t = (*storage_ix).wrapping_add(3 as size_t);
    let mut literal_ratio: size_t = 0;
    let mut ip: *const uint8_t = ::core::ptr::null::<uint8_t>();
    let mut last_distance: c_int = 0;
    let shift: size_t = (64 as size_t).wrapping_sub(table_bits);
    BrotliStoreMetaBlockHeader(block_size, 0 as c_int, storage_ix, storage);
    BrotliWriteBits(13 as size_t, 0 as uint64_t, storage_ix, storage);
    literal_ratio = BuildAndStoreLiteralPrefixCode(
        s,
        input,
        block_size,
        &raw mut (*s).lit_depth as *mut uint8_t,
        &raw mut (*s).lit_bits as *mut uint16_t,
        storage_ix,
        storage,
    );
    let mut i: size_t = 0;
    i = 0 as size_t;
    while i.wrapping_add(7 as size_t) < (*s).cmd_code_numbits {
        BrotliWriteBits(
            8 as size_t,
            (*s).cmd_code[(i >> 3 as c_int) as usize] as uint64_t,
            storage_ix,
            storage,
        );
        i = (i as c_ulong).wrapping_add(8 as c_ulong) as size_t as size_t;
    }
    BrotliWriteBits(
        (*s).cmd_code_numbits & 7 as size_t,
        (*s).cmd_code[((*s).cmd_code_numbits >> 3 as c_int) as usize] as uint64_t,
        storage_ix,
        storage,
    );
    loop {
        memcpy(
            &raw mut (*s).cmd_histo as *mut uint32_t as *mut c_void,
            &raw mut kCmdHistoSeed as *mut uint32_t as *const c_void,
            ::core::mem::size_of::<[uint32_t; 128]>() as size_t,
        );
        ip = input;
        last_distance = -(1 as c_int);
        ip_end = input.offset(block_size as isize);
        if (block_size >= kInputMarginBytes) as c_int as c_long != 0 {
            let len_limit: size_t = brotli_min_size_t(
                block_size.wrapping_sub(kMinMatchLen),
                input_size.wrapping_sub(kInputMarginBytes),
            ) as size_t;
            let mut ip_limit: *const uint8_t = input.offset(len_limit as isize);
            let mut next_hash: uint32_t = 0;
            ip = ip.offset(1);
            next_hash = Hash(ip, shift);
            's_108: loop {
                let mut skip: uint32_t = 32 as uint32_t;
                let mut next_ip: *const uint8_t = ip;
                let mut candidate: *const uint8_t = ::core::ptr::null::<uint8_t>();
                loop {
                    let mut hash: uint32_t = next_hash;
                    let fresh1 = skip;
                    skip = skip.wrapping_add(1);
                    let mut bytes_between_hash_lookups: uint32_t =
                        fresh1 >> 5 as c_int;
                    ip = next_ip;
                    next_ip = ip.offset(bytes_between_hash_lookups as isize);
                    if (next_ip > ip_limit) as c_int as c_long != 0 {
                        current_block = 54079586644752974;
                        break 's_108;
                    }
                    next_hash = Hash(next_ip, shift);
                    candidate = ip.offset(-(last_distance as isize));
                    if IsMatch(ip, candidate) != 0 {
                        if (candidate < ip) as c_int as c_long != 0 {
                            *table.offset(hash as isize) = ip.offset_from(base_ip)
                                as c_long
                                as c_int;
                            current_block = 10692455896603418738;
                        } else {
                            current_block = 18377268871191777778;
                        }
                    } else {
                        current_block = 18377268871191777778;
                    }
                    match current_block {
                        18377268871191777778 => {
                            candidate = base_ip.offset(*table.offset(hash as isize) as isize);
                            *table.offset(hash as isize) = ip.offset_from(base_ip)
                                as c_long
                                as c_int;
                            if (IsMatch(ip, candidate) == 0) as c_int
                                as c_long
                                != 0
                            {
                                continue;
                            }
                        }
                        _ => {}
                    }
                    if !(ip.offset_from(candidate) as c_long
                        > ((1 as c_int as size_t) << 18 as c_int)
                            .wrapping_sub(BROTLI_WINDOW_GAP as size_t)
                            as c_long)
                    {
                        break;
                    }
                }
                let mut base: *const uint8_t = ip;
                let mut matched: size_t = (5 as size_t).wrapping_add(FindMatchLengthWithLimit(
                    candidate.offset(5 as c_int as isize),
                    ip.offset(5 as c_int as isize),
                    (ip_end.offset_from(ip) as c_long as size_t)
                        .wrapping_sub(5 as size_t),
                ));
                let mut distance: c_int =
                    base.offset_from(candidate) as c_long as c_int;
                let mut insert: size_t =
                    base.offset_from(next_emit) as c_long as size_t;
                ip = ip.offset(matched as isize);
                if (insert < 6210 as size_t) as c_int as c_long != 0 {
                    EmitInsertLen(
                        insert,
                        cmd_depth as *const uint8_t,
                        cmd_bits as *const uint16_t,
                        cmd_histo as *mut uint32_t,
                        storage_ix,
                        storage,
                    );
                } else if ShouldUseUncompressedMode(
                    metablock_start,
                    next_emit,
                    insert,
                    literal_ratio,
                ) != 0
                {
                    EmitUncompressedMetaBlock(
                        metablock_start,
                        base,
                        mlen_storage_ix.wrapping_sub(3 as size_t),
                        storage_ix,
                        storage,
                    );
                    input_size = (input_size as c_ulong)
                        .wrapping_sub(base.offset_from(input) as c_long as size_t
                            as c_ulong) as size_t
                        as size_t;
                    input = base;
                    next_emit = input;
                    current_block = 1734113972801549217;
                    break;
                } else {
                    EmitLongInsertLen(
                        insert,
                        cmd_depth as *const uint8_t,
                        cmd_bits as *const uint16_t,
                        cmd_histo as *mut uint32_t,
                        storage_ix,
                        storage,
                    );
                }
                EmitLiterals(
                    next_emit,
                    insert,
                    lit_depth as *const uint8_t,
                    lit_bits as *const uint16_t,
                    storage_ix,
                    storage,
                );
                if distance == last_distance {
                    BrotliWriteBits(
                        *cmd_depth.offset(64 as c_int as isize) as size_t,
                        *cmd_bits.offset(64 as c_int as isize) as uint64_t,
                        storage_ix,
                        storage,
                    );
                    let ref mut fresh2 = *cmd_histo.offset(64 as c_int as isize);
                    *fresh2 = (*fresh2).wrapping_add(1);
                } else {
                    EmitDistance(
                        distance as size_t,
                        cmd_depth as *const uint8_t,
                        cmd_bits as *const uint16_t,
                        cmd_histo as *mut uint32_t,
                        storage_ix,
                        storage,
                    );
                    last_distance = distance;
                }
                EmitCopyLenLastDistance(
                    matched,
                    cmd_depth as *const uint8_t,
                    cmd_bits as *const uint16_t,
                    cmd_histo as *mut uint32_t,
                    storage_ix,
                    storage,
                );
                next_emit = ip;
                if (ip >= ip_limit) as c_int as c_long != 0 {
                    current_block = 54079586644752974;
                    break;
                }
                let mut input_bytes: uint64_t =
                    BrotliUnalignedRead64(ip.offset(-(3 as c_int as isize))
                        as *const c_void);
                let mut prev_hash: uint32_t =
                    HashBytesAtOffset(input_bytes, 0 as c_int, shift);
                let mut cur_hash: uint32_t =
                    HashBytesAtOffset(input_bytes, 3 as c_int, shift);
                *table.offset(prev_hash as isize) = (ip.offset_from(base_ip) as c_long
                    - 3 as c_long)
                    as c_int;
                prev_hash = HashBytesAtOffset(input_bytes, 1 as c_int, shift);
                *table.offset(prev_hash as isize) = (ip.offset_from(base_ip) as c_long
                    - 2 as c_long)
                    as c_int;
                prev_hash = HashBytesAtOffset(input_bytes, 2 as c_int, shift);
                *table.offset(prev_hash as isize) = (ip.offset_from(base_ip) as c_long
                    - 1 as c_long)
                    as c_int;
                candidate = base_ip.offset(*table.offset(cur_hash as isize) as isize);
                *table.offset(cur_hash as isize) =
                    ip.offset_from(base_ip) as c_long as c_int;
                while IsMatch(ip, candidate) != 0 {
                    let mut base_0: *const uint8_t = ip;
                    let mut matched_0: size_t =
                        (5 as size_t).wrapping_add(FindMatchLengthWithLimit(
                            candidate.offset(5 as c_int as isize),
                            ip.offset(5 as c_int as isize),
                            (ip_end.offset_from(ip) as c_long as size_t)
                                .wrapping_sub(5 as size_t),
                        ));
                    if ip.offset_from(candidate) as c_long
                        > ((1 as c_int as size_t) << 18 as c_int)
                            .wrapping_sub(BROTLI_WINDOW_GAP as size_t)
                            as c_long
                    {
                        break;
                    }
                    ip = ip.offset(matched_0 as isize);
                    last_distance =
                        base_0.offset_from(candidate) as c_long as c_int;
                    EmitCopyLen(
                        matched_0,
                        cmd_depth as *const uint8_t,
                        cmd_bits as *const uint16_t,
                        cmd_histo as *mut uint32_t,
                        storage_ix,
                        storage,
                    );
                    EmitDistance(
                        last_distance as size_t,
                        cmd_depth as *const uint8_t,
                        cmd_bits as *const uint16_t,
                        cmd_histo as *mut uint32_t,
                        storage_ix,
                        storage,
                    );
                    next_emit = ip;
                    if (ip >= ip_limit) as c_int as c_long != 0 {
                        current_block = 54079586644752974;
                        break 's_108;
                    }
                    let mut input_bytes_0: uint64_t =
                        BrotliUnalignedRead64(ip.offset(-(3 as c_int as isize))
                            as *const c_void);
                    let mut prev_hash_0: uint32_t =
                        HashBytesAtOffset(input_bytes_0, 0 as c_int, shift);
                    let mut cur_hash_0: uint32_t =
                        HashBytesAtOffset(input_bytes_0, 3 as c_int, shift);
                    *table.offset(prev_hash_0 as isize) =
                        (ip.offset_from(base_ip) as c_long - 3 as c_long)
                            as c_int;
                    prev_hash_0 = HashBytesAtOffset(input_bytes_0, 1 as c_int, shift);
                    *table.offset(prev_hash_0 as isize) =
                        (ip.offset_from(base_ip) as c_long - 2 as c_long)
                            as c_int;
                    prev_hash_0 = HashBytesAtOffset(input_bytes_0, 2 as c_int, shift);
                    *table.offset(prev_hash_0 as isize) =
                        (ip.offset_from(base_ip) as c_long - 1 as c_long)
                            as c_int;
                    candidate = base_ip.offset(*table.offset(cur_hash_0 as isize) as isize);
                    *table.offset(cur_hash_0 as isize) =
                        ip.offset_from(base_ip) as c_long as c_int;
                }
                ip = ip.offset(1);
                next_hash = Hash(ip, shift);
            }
        } else {
            current_block = 54079586644752974;
        }
        match current_block {
            54079586644752974 => {
                input = input.offset(block_size as isize);
                input_size = (input_size as c_ulong)
                    .wrapping_sub(block_size as c_ulong)
                    as size_t as size_t;
                block_size = brotli_min_size_t(input_size, kMergeBlockSize);
                if input_size > 0 as size_t
                    && total_block_size.wrapping_add(block_size)
                        <= ((1 as c_int) << 20 as c_int) as size_t
                    && ShouldMergeBlock(s, input, block_size, lit_depth) != 0
                {
                    total_block_size = (total_block_size as c_ulong)
                        .wrapping_add(block_size as c_ulong)
                        as size_t as size_t;
                    UpdateBits(
                        20 as size_t,
                        total_block_size.wrapping_sub(1 as size_t) as uint32_t,
                        mlen_storage_ix,
                        storage,
                    );
                    continue;
                } else {
                    if next_emit < ip_end {
                        let insert_0: size_t =
                            ip_end.offset_from(next_emit) as c_long as size_t;
                        if (insert_0 < 6210 as size_t) as c_int as c_long
                            != 0
                        {
                            EmitInsertLen(
                                insert_0,
                                cmd_depth as *const uint8_t,
                                cmd_bits as *const uint16_t,
                                cmd_histo as *mut uint32_t,
                                storage_ix,
                                storage,
                            );
                            EmitLiterals(
                                next_emit,
                                insert_0,
                                lit_depth as *const uint8_t,
                                lit_bits as *const uint16_t,
                                storage_ix,
                                storage,
                            );
                        } else if ShouldUseUncompressedMode(
                            metablock_start,
                            next_emit,
                            insert_0,
                            literal_ratio,
                        ) != 0
                        {
                            EmitUncompressedMetaBlock(
                                metablock_start,
                                ip_end,
                                mlen_storage_ix.wrapping_sub(3 as size_t),
                                storage_ix,
                                storage,
                            );
                        } else {
                            EmitLongInsertLen(
                                insert_0,
                                cmd_depth as *const uint8_t,
                                cmd_bits as *const uint16_t,
                                cmd_histo as *mut uint32_t,
                                storage_ix,
                                storage,
                            );
                            EmitLiterals(
                                next_emit,
                                insert_0,
                                lit_depth as *const uint8_t,
                                lit_bits as *const uint16_t,
                                storage_ix,
                                storage,
                            );
                        }
                    }
                    next_emit = ip_end;
                }
            }
            _ => {}
        }
        if !(input_size > 0 as size_t) {
            break;
        }
        metablock_start = input;
        block_size = brotli_min_size_t(input_size, kFirstBlockSize);
        total_block_size = block_size;
        mlen_storage_ix = (*storage_ix).wrapping_add(3 as size_t);
        BrotliStoreMetaBlockHeader(block_size, 0 as c_int, storage_ix, storage);
        BrotliWriteBits(13 as size_t, 0 as uint64_t, storage_ix, storage);
        literal_ratio = BuildAndStoreLiteralPrefixCode(
            s,
            input,
            block_size,
            lit_depth as *mut uint8_t,
            lit_bits as *mut uint16_t,
            storage_ix,
            storage,
        );
        BuildAndStoreCommandPrefixCode(s, storage_ix, storage);
    }
    if is_last == 0 {
        (*s).cmd_code[0 as c_int as usize] = 0 as uint8_t;
        (*s).cmd_code_numbits = 0 as size_t;
        BuildAndStoreCommandPrefixCode(
            s,
            &raw mut (*s).cmd_code_numbits,
            &raw mut (*s).cmd_code as *mut uint8_t,
        );
    }
}
#[inline(never)]
unsafe extern "C" fn BrotliCompressFragmentFastImpl15(
    mut s: *mut BrotliOnePassArena,
    mut input: *const uint8_t,
    mut input_size: size_t,
    mut is_last: c_int,
    mut table: *mut c_int,
    mut storage_ix: *mut size_t,
    mut storage: *mut uint8_t,
) {
    BrotliCompressFragmentFastImpl(
        s,
        input,
        input_size,
        is_last,
        table,
        15 as size_t,
        storage_ix,
        storage,
    );
}
#[inline(never)]
unsafe extern "C" fn BrotliCompressFragmentFastImpl13(
    mut s: *mut BrotliOnePassArena,
    mut input: *const uint8_t,
    mut input_size: size_t,
    mut is_last: c_int,
    mut table: *mut c_int,
    mut storage_ix: *mut size_t,
    mut storage: *mut uint8_t,
) {
    BrotliCompressFragmentFastImpl(
        s,
        input,
        input_size,
        is_last,
        table,
        13 as size_t,
        storage_ix,
        storage,
    );
}
#[inline(never)]
unsafe extern "C" fn BrotliCompressFragmentFastImpl11(
    mut s: *mut BrotliOnePassArena,
    mut input: *const uint8_t,
    mut input_size: size_t,
    mut is_last: c_int,
    mut table: *mut c_int,
    mut storage_ix: *mut size_t,
    mut storage: *mut uint8_t,
) {
    BrotliCompressFragmentFastImpl(
        s,
        input,
        input_size,
        is_last,
        table,
        11 as size_t,
        storage_ix,
        storage,
    );
}
#[inline(never)]
unsafe extern "C" fn BrotliCompressFragmentFastImpl9(
    mut s: *mut BrotliOnePassArena,
    mut input: *const uint8_t,
    mut input_size: size_t,
    mut is_last: c_int,
    mut table: *mut c_int,
    mut storage_ix: *mut size_t,
    mut storage: *mut uint8_t,
) {
    BrotliCompressFragmentFastImpl(
        s,
        input,
        input_size,
        is_last,
        table,
        9 as size_t,
        storage_ix,
        storage,
    );
}
#[no_mangle]
pub unsafe extern "C" fn BrotliCompressFragmentFast(
    mut s: *mut BrotliOnePassArena,
    mut input: *const uint8_t,
    mut input_size: size_t,
    mut is_last: c_int,
    mut table: *mut c_int,
    mut table_size: size_t,
    mut storage_ix: *mut size_t,
    mut storage: *mut uint8_t,
) {
    let initial_storage_ix: size_t = *storage_ix;
    let table_bits: size_t = Log2FloorNonZero(table_size) as size_t;
    if input_size == 0 as size_t {
        BrotliWriteBits(1 as size_t, 1 as uint64_t, storage_ix, storage);
        BrotliWriteBits(1 as size_t, 1 as uint64_t, storage_ix, storage);
        *storage_ix =
            (*storage_ix).wrapping_add(7 as size_t) & !(7 as c_uint) as size_t;
        return;
    }
    match table_bits {
        9 => {
            BrotliCompressFragmentFastImpl9(
                s, input, input_size, is_last, table, storage_ix, storage,
            );
        }
        11 => {
            BrotliCompressFragmentFastImpl11(
                s, input, input_size, is_last, table, storage_ix, storage,
            );
        }
        13 => {
            BrotliCompressFragmentFastImpl13(
                s, input, input_size, is_last, table, storage_ix, storage,
            );
        }
        15 => {
            BrotliCompressFragmentFastImpl15(
                s, input, input_size, is_last, table, storage_ix, storage,
            );
        }
        _ => {}
    }
    if (*storage_ix).wrapping_sub(initial_storage_ix)
        > (31 as size_t).wrapping_add(input_size << 3 as c_int)
    {
        EmitUncompressedMetaBlock(
            input,
            input.offset(input_size as isize),
            initial_storage_ix,
            storage_ix,
            storage,
        );
    }
    if is_last != 0 {
        BrotliWriteBits(1 as size_t, 1 as uint64_t, storage_ix, storage);
        BrotliWriteBits(1 as size_t, 1 as uint64_t, storage_ix, storage);
        *storage_ix =
            (*storage_ix).wrapping_add(7 as size_t) & !(7 as c_uint) as size_t;
    }
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
#[inline(always)]
unsafe extern "C" fn BrotliWriteBits(
    mut n_bits: size_t,
    mut bits: uint64_t,
    mut pos: *mut size_t,
    mut array: *mut uint8_t,
) {
    let mut p: *mut uint8_t =
        array.offset((*pos >> 3 as c_int) as isize) as *mut uint8_t;
    let mut v: uint64_t = *p as uint64_t;
    v = (v as c_ulong | (bits << (*pos & 7 as size_t)) as c_ulong)
        as uint64_t;
    BrotliUnalignedWrite64(p as *mut c_void, v);
    *pos = (*pos as c_ulong).wrapping_add(n_bits as c_ulong) as size_t
        as size_t;
}
