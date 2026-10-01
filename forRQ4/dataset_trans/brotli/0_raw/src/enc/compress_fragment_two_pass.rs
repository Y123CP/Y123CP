extern "C" {
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn BrotliCreateHuffmanTree(
        data: *const uint32_t,
        length: size_t,
        tree_limit: ::core::ffi::c_int,
        tree: *mut HuffmanTree,
        depth: *mut uint8_t,
    );
    fn BrotliConvertBitDepthsToSymbols(depth: *const uint8_t, len: size_t, bits: *mut uint16_t);
    fn BrotliBitsEntropy(population: *const uint32_t, size: size_t) -> ::core::ffi::c_double;
    fn BrotliStoreHuffmanTree(
        depths: *const uint8_t,
        num: size_t,
        tree: *mut HuffmanTree,
        storage_ix: *mut size_t,
        storage: *mut uint8_t,
    );
    fn BrotliBuildAndStoreHuffmanTreeFast(
        tree: *mut HuffmanTree,
        histogram: *const uint32_t,
        histogram_total: size_t,
        max_bits: size_t,
        depth: *mut uint8_t,
        bits: *mut uint16_t,
        storage_ix: *mut size_t,
        storage: *mut uint8_t,
    );
}
pub type size_t = usize;
pub type __uint8_t = u8;
pub type __int16_t = i16;
pub type __uint16_t = u16;
pub type __uint32_t = u32;
pub type __uint64_t = u64;
pub type int16_t = __int16_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HuffmanTree {
    pub total_count_: uint32_t,
    pub index_left_: int16_t,
    pub index_right_or_value_: int16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BrotliTwoPassArena {
    pub lit_histo: [uint32_t; 256],
    pub lit_depth: [uint8_t; 256],
    pub lit_bits: [uint16_t; 256],
    pub cmd_histo: [uint32_t; 128],
    pub cmd_depth: [uint8_t; 128],
    pub cmd_bits: [uint16_t; 128],
    pub tmp_tree: [HuffmanTree; 513],
    pub tmp_depth: [uint8_t; 704],
    pub tmp_bits: [uint16_t; 64],
}
pub const BROTLI_TRUE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const BROTLI_FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[inline(always)]
unsafe extern "C" fn BrotliUnalignedRead32(mut p: *const ::core::ffi::c_void) -> uint32_t {
    let mut t: uint32_t = 0;
    memcpy(
        &raw mut t as *mut ::core::ffi::c_void,
        p,
        ::core::mem::size_of::<uint32_t>() as size_t,
    );
    return t;
}
#[inline(always)]
unsafe extern "C" fn BrotliUnalignedRead64(mut p: *const ::core::ffi::c_void) -> uint64_t {
    let mut t: uint64_t = 0;
    memcpy(
        &raw mut t as *mut ::core::ffi::c_void,
        p,
        ::core::mem::size_of::<uint64_t>() as size_t,
    );
    return t;
}
#[inline(always)]
unsafe extern "C" fn BrotliUnalignedWrite64(mut p: *mut ::core::ffi::c_void, mut v: uint64_t) {
    memcpy(
        p,
        &raw mut v as *const ::core::ffi::c_void,
        ::core::mem::size_of::<uint64_t>() as size_t,
    );
}
#[inline(always)]
unsafe extern "C" fn brotli_min_size_t(mut a: size_t, mut b: size_t) -> size_t {
    return if a < b { a } else { b };
}
pub const BROTLI_NUM_COMMAND_SYMBOLS: ::core::ffi::c_int = 704 as ::core::ffi::c_int;
pub const BROTLI_WINDOW_GAP: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
static mut kCompressFragmentTwoPassBlockSize: size_t =
    ((1 as ::core::ffi::c_int) << 17 as ::core::ffi::c_int) as size_t;
#[inline(always)]
unsafe extern "C" fn Hash(
    mut p: *const uint8_t,
    mut shift: size_t,
    mut length: size_t,
) -> uint32_t {
    let h: uint64_t = ((BrotliUnalignedRead64(p as *const ::core::ffi::c_void) as uint64_t)
        << (8 as size_t).wrapping_sub(length).wrapping_mul(8 as size_t))
    .wrapping_mul(kHashMul32 as uint64_t);
    return (h >> shift) as uint32_t;
}
#[inline(always)]
unsafe extern "C" fn HashBytesAtOffset(
    mut v: uint64_t,
    mut offset: size_t,
    mut shift: size_t,
    mut length: size_t,
) -> uint32_t {
    let h: uint64_t = (v >> (8 as size_t).wrapping_mul(offset)
        << (8 as size_t).wrapping_sub(length).wrapping_mul(8 as size_t))
    .wrapping_mul(kHashMul32 as uint64_t);
    return (h >> shift) as uint32_t;
}
#[inline(always)]
unsafe extern "C" fn IsMatch(
    mut p1: *const uint8_t,
    mut p2: *const uint8_t,
    mut length: size_t,
) -> ::core::ffi::c_int {
    if BrotliUnalignedRead32(p1 as *const ::core::ffi::c_void)
        == BrotliUnalignedRead32(p2 as *const ::core::ffi::c_void)
    {
        if length == 4 as size_t {
            return BROTLI_TRUE;
        }
        return if *p1.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == *p2.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            && *p1.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == *p2.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        {
            BROTLI_TRUE
        } else {
            BROTLI_FALSE
        };
    }
    return BROTLI_FALSE;
}
unsafe extern "C" fn BuildAndStoreCommandPrefixCode(
    mut s: *mut BrotliTwoPassArena,
    mut storage_ix: *mut size_t,
    mut storage: *mut uint8_t,
) {
    memset(
        &raw mut (*s).tmp_depth as *mut uint8_t as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<[uint8_t; 704]>() as size_t,
    );
    BrotliCreateHuffmanTree(
        &raw mut (*s).cmd_histo as *mut uint32_t,
        64 as size_t,
        15 as ::core::ffi::c_int,
        &raw mut (*s).tmp_tree as *mut HuffmanTree,
        &raw mut (*s).cmd_depth as *mut uint8_t,
    );
    BrotliCreateHuffmanTree(
        (&raw mut (*s).cmd_histo as *mut uint32_t).offset(64 as ::core::ffi::c_int as isize)
            as *mut uint32_t,
        64 as size_t,
        14 as ::core::ffi::c_int,
        &raw mut (*s).tmp_tree as *mut HuffmanTree,
        (&raw mut (*s).cmd_depth as *mut uint8_t).offset(64 as ::core::ffi::c_int as isize)
            as *mut uint8_t,
    );
    memcpy(
        &raw mut (*s).tmp_depth as *mut uint8_t as *mut ::core::ffi::c_void,
        (&raw mut (*s).cmd_depth as *mut uint8_t).offset(24 as ::core::ffi::c_int as isize)
            as *const ::core::ffi::c_void,
        24 as size_t,
    );
    memcpy(
        (&raw mut (*s).tmp_depth as *mut uint8_t).offset(24 as ::core::ffi::c_int as isize)
            as *mut ::core::ffi::c_void,
        &raw mut (*s).cmd_depth as *mut uint8_t as *const ::core::ffi::c_void,
        8 as size_t,
    );
    memcpy(
        (&raw mut (*s).tmp_depth as *mut uint8_t).offset(32 as ::core::ffi::c_int as isize)
            as *mut ::core::ffi::c_void,
        (&raw mut (*s).cmd_depth as *mut uint8_t).offset(48 as ::core::ffi::c_int as isize)
            as *const ::core::ffi::c_void,
        8 as size_t,
    );
    memcpy(
        (&raw mut (*s).tmp_depth as *mut uint8_t).offset(40 as ::core::ffi::c_int as isize)
            as *mut ::core::ffi::c_void,
        (&raw mut (*s).cmd_depth as *mut uint8_t).offset(8 as ::core::ffi::c_int as isize)
            as *const ::core::ffi::c_void,
        8 as size_t,
    );
    memcpy(
        (&raw mut (*s).tmp_depth as *mut uint8_t).offset(48 as ::core::ffi::c_int as isize)
            as *mut ::core::ffi::c_void,
        (&raw mut (*s).cmd_depth as *mut uint8_t).offset(56 as ::core::ffi::c_int as isize)
            as *const ::core::ffi::c_void,
        8 as size_t,
    );
    memcpy(
        (&raw mut (*s).tmp_depth as *mut uint8_t).offset(56 as ::core::ffi::c_int as isize)
            as *mut ::core::ffi::c_void,
        (&raw mut (*s).cmd_depth as *mut uint8_t).offset(16 as ::core::ffi::c_int as isize)
            as *const ::core::ffi::c_void,
        8 as size_t,
    );
    BrotliConvertBitDepthsToSymbols(
        &raw mut (*s).tmp_depth as *mut uint8_t,
        64 as size_t,
        &raw mut (*s).tmp_bits as *mut uint16_t,
    );
    memcpy(
        &raw mut (*s).cmd_bits as *mut uint16_t as *mut ::core::ffi::c_void,
        (&raw mut (*s).tmp_bits as *mut uint16_t).offset(24 as ::core::ffi::c_int as isize)
            as *const ::core::ffi::c_void,
        16 as size_t,
    );
    memcpy(
        (&raw mut (*s).cmd_bits as *mut uint16_t).offset(8 as ::core::ffi::c_int as isize)
            as *mut ::core::ffi::c_void,
        (&raw mut (*s).tmp_bits as *mut uint16_t).offset(40 as ::core::ffi::c_int as isize)
            as *const ::core::ffi::c_void,
        16 as size_t,
    );
    memcpy(
        (&raw mut (*s).cmd_bits as *mut uint16_t).offset(16 as ::core::ffi::c_int as isize)
            as *mut ::core::ffi::c_void,
        (&raw mut (*s).tmp_bits as *mut uint16_t).offset(56 as ::core::ffi::c_int as isize)
            as *const ::core::ffi::c_void,
        16 as size_t,
    );
    memcpy(
        (&raw mut (*s).cmd_bits as *mut uint16_t).offset(24 as ::core::ffi::c_int as isize)
            as *mut ::core::ffi::c_void,
        &raw mut (*s).tmp_bits as *mut uint16_t as *const ::core::ffi::c_void,
        48 as size_t,
    );
    memcpy(
        (&raw mut (*s).cmd_bits as *mut uint16_t).offset(48 as ::core::ffi::c_int as isize)
            as *mut ::core::ffi::c_void,
        (&raw mut (*s).tmp_bits as *mut uint16_t).offset(32 as ::core::ffi::c_int as isize)
            as *const ::core::ffi::c_void,
        16 as size_t,
    );
    memcpy(
        (&raw mut (*s).cmd_bits as *mut uint16_t).offset(56 as ::core::ffi::c_int as isize)
            as *mut ::core::ffi::c_void,
        (&raw mut (*s).tmp_bits as *mut uint16_t).offset(48 as ::core::ffi::c_int as isize)
            as *const ::core::ffi::c_void,
        16 as size_t,
    );
    BrotliConvertBitDepthsToSymbols(
        (&raw mut (*s).cmd_depth as *mut uint8_t).offset(64 as ::core::ffi::c_int as isize)
            as *mut uint8_t,
        64 as size_t,
        (&raw mut (*s).cmd_bits as *mut uint16_t).offset(64 as ::core::ffi::c_int as isize)
            as *mut uint16_t,
    );
    let mut i: size_t = 0;
    memset(
        &raw mut (*s).tmp_depth as *mut uint8_t as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        64 as size_t,
    );
    memcpy(
        &raw mut (*s).tmp_depth as *mut uint8_t as *mut ::core::ffi::c_void,
        (&raw mut (*s).cmd_depth as *mut uint8_t).offset(24 as ::core::ffi::c_int as isize)
            as *const ::core::ffi::c_void,
        8 as size_t,
    );
    memcpy(
        (&raw mut (*s).tmp_depth as *mut uint8_t).offset(64 as ::core::ffi::c_int as isize)
            as *mut ::core::ffi::c_void,
        (&raw mut (*s).cmd_depth as *mut uint8_t).offset(32 as ::core::ffi::c_int as isize)
            as *const ::core::ffi::c_void,
        8 as size_t,
    );
    memcpy(
        (&raw mut (*s).tmp_depth as *mut uint8_t).offset(128 as ::core::ffi::c_int as isize)
            as *mut ::core::ffi::c_void,
        (&raw mut (*s).cmd_depth as *mut uint8_t).offset(40 as ::core::ffi::c_int as isize)
            as *const ::core::ffi::c_void,
        8 as size_t,
    );
    memcpy(
        (&raw mut (*s).tmp_depth as *mut uint8_t).offset(192 as ::core::ffi::c_int as isize)
            as *mut ::core::ffi::c_void,
        (&raw mut (*s).cmd_depth as *mut uint8_t).offset(48 as ::core::ffi::c_int as isize)
            as *const ::core::ffi::c_void,
        8 as size_t,
    );
    memcpy(
        (&raw mut (*s).tmp_depth as *mut uint8_t).offset(384 as ::core::ffi::c_int as isize)
            as *mut ::core::ffi::c_void,
        (&raw mut (*s).cmd_depth as *mut uint8_t).offset(56 as ::core::ffi::c_int as isize)
            as *const ::core::ffi::c_void,
        8 as size_t,
    );
    i = 0 as size_t;
    while i < 8 as size_t {
        (*s).tmp_depth[(128 as size_t).wrapping_add((8 as size_t).wrapping_mul(i)) as usize] =
            (*s).cmd_depth[i as usize];
        (*s).tmp_depth[(256 as size_t).wrapping_add((8 as size_t).wrapping_mul(i)) as usize] =
            (*s).cmd_depth[(8 as size_t).wrapping_add(i) as usize];
        (*s).tmp_depth[(448 as size_t).wrapping_add((8 as size_t).wrapping_mul(i)) as usize] =
            (*s).cmd_depth[(16 as size_t).wrapping_add(i) as usize];
        i = i.wrapping_add(1);
    }
    BrotliStoreHuffmanTree(
        &raw mut (*s).tmp_depth as *mut uint8_t,
        BROTLI_NUM_COMMAND_SYMBOLS as size_t,
        &raw mut (*s).tmp_tree as *mut HuffmanTree,
        storage_ix,
        storage,
    );
    BrotliStoreHuffmanTree(
        (&raw mut (*s).cmd_depth as *mut uint8_t).offset(64 as ::core::ffi::c_int as isize)
            as *mut uint8_t,
        64 as size_t,
        &raw mut (*s).tmp_tree as *mut HuffmanTree,
        storage_ix,
        storage,
    );
}
#[inline(always)]
unsafe extern "C" fn EmitInsertLen(mut insertlen: uint32_t, mut commands: *mut *mut uint32_t) {
    if insertlen < 6 as uint32_t {
        **commands = insertlen;
    } else if insertlen < 130 as uint32_t {
        let tail: uint32_t = insertlen.wrapping_sub(2 as uint32_t);
        let nbits: uint32_t =
            (Log2FloorNonZero(tail as size_t) as uint32_t).wrapping_sub(1 as uint32_t);
        let prefix: uint32_t = tail >> nbits;
        let inscode: uint32_t = (nbits << 1 as ::core::ffi::c_int)
            .wrapping_add(prefix)
            .wrapping_add(2 as uint32_t);
        let extra: uint32_t = tail.wrapping_sub(prefix << nbits);
        **commands = inscode | extra << 8 as ::core::ffi::c_int;
    } else if insertlen < 2114 as uint32_t {
        let tail_0: uint32_t = insertlen.wrapping_sub(66 as uint32_t);
        let nbits_0: uint32_t = Log2FloorNonZero(tail_0 as size_t) as uint32_t;
        let code: uint32_t = nbits_0.wrapping_add(10 as uint32_t);
        let extra_0: uint32_t = tail_0.wrapping_sub((1 as uint32_t) << nbits_0);
        **commands = code | extra_0 << 8 as ::core::ffi::c_int;
    } else if insertlen < 6210 as uint32_t {
        let extra_1: uint32_t = insertlen.wrapping_sub(2114 as uint32_t);
        **commands = 21 as uint32_t | extra_1 << 8 as ::core::ffi::c_int;
    } else if insertlen < 22594 as uint32_t {
        let extra_2: uint32_t = insertlen.wrapping_sub(6210 as uint32_t);
        **commands = 22 as uint32_t | extra_2 << 8 as ::core::ffi::c_int;
    } else {
        let extra_3: uint32_t = insertlen.wrapping_sub(22594 as uint32_t);
        **commands = 23 as uint32_t | extra_3 << 8 as ::core::ffi::c_int;
    }
    *commands = (*commands).offset(1);
}
#[inline(always)]
unsafe extern "C" fn EmitCopyLen(mut copylen: size_t, mut commands: *mut *mut uint32_t) {
    if copylen < 10 as size_t {
        **commands = copylen.wrapping_add(38 as size_t) as uint32_t;
    } else if copylen < 134 as size_t {
        let tail: size_t = copylen.wrapping_sub(6 as size_t);
        let nbits: size_t = Log2FloorNonZero(tail).wrapping_sub(1 as uint32_t) as size_t;
        let prefix: size_t = tail >> nbits;
        let code: size_t = (nbits << 1 as ::core::ffi::c_int)
            .wrapping_add(prefix)
            .wrapping_add(44 as size_t);
        let extra: size_t = tail.wrapping_sub(prefix << nbits);
        **commands = (code | extra << 8 as ::core::ffi::c_int) as uint32_t;
    } else if copylen < 2118 as size_t {
        let tail_0: size_t = copylen.wrapping_sub(70 as size_t);
        let nbits_0: size_t = Log2FloorNonZero(tail_0) as size_t;
        let code_0: size_t = nbits_0.wrapping_add(52 as size_t);
        let extra_0: size_t = tail_0.wrapping_sub((1 as ::core::ffi::c_int as size_t) << nbits_0);
        **commands = (code_0 | extra_0 << 8 as ::core::ffi::c_int) as uint32_t;
    } else {
        let extra_1: size_t = copylen.wrapping_sub(2118 as size_t);
        **commands = (63 as size_t | extra_1 << 8 as ::core::ffi::c_int) as uint32_t;
    }
    *commands = (*commands).offset(1);
}
#[inline(always)]
unsafe extern "C" fn EmitCopyLenLastDistance(
    mut copylen: size_t,
    mut commands: *mut *mut uint32_t,
) {
    if copylen < 12 as size_t {
        **commands = copylen.wrapping_add(20 as size_t) as uint32_t;
        *commands = (*commands).offset(1);
    } else if copylen < 72 as size_t {
        let tail: size_t = copylen.wrapping_sub(8 as size_t);
        let nbits: size_t = Log2FloorNonZero(tail).wrapping_sub(1 as uint32_t) as size_t;
        let prefix: size_t = tail >> nbits;
        let code: size_t = (nbits << 1 as ::core::ffi::c_int)
            .wrapping_add(prefix)
            .wrapping_add(28 as size_t);
        let extra: size_t = tail.wrapping_sub(prefix << nbits);
        **commands = (code | extra << 8 as ::core::ffi::c_int) as uint32_t;
        *commands = (*commands).offset(1);
    } else if copylen < 136 as size_t {
        let tail_0: size_t = copylen.wrapping_sub(8 as size_t);
        let code_0: size_t = (tail_0 >> 5 as ::core::ffi::c_int).wrapping_add(54 as size_t);
        let extra_0: size_t = tail_0 & 31 as size_t;
        **commands = (code_0 | extra_0 << 8 as ::core::ffi::c_int) as uint32_t;
        *commands = (*commands).offset(1);
        **commands = 64 as uint32_t;
        *commands = (*commands).offset(1);
    } else if copylen < 2120 as size_t {
        let tail_1: size_t = copylen.wrapping_sub(72 as size_t);
        let nbits_0: size_t = Log2FloorNonZero(tail_1) as size_t;
        let code_1: size_t = nbits_0.wrapping_add(52 as size_t);
        let extra_1: size_t = tail_1.wrapping_sub((1 as ::core::ffi::c_int as size_t) << nbits_0);
        **commands = (code_1 | extra_1 << 8 as ::core::ffi::c_int) as uint32_t;
        *commands = (*commands).offset(1);
        **commands = 64 as uint32_t;
        *commands = (*commands).offset(1);
    } else {
        let extra_2: size_t = copylen.wrapping_sub(2120 as size_t);
        **commands = (63 as size_t | extra_2 << 8 as ::core::ffi::c_int) as uint32_t;
        *commands = (*commands).offset(1);
        **commands = 64 as uint32_t;
        *commands = (*commands).offset(1);
    };
}
#[inline(always)]
unsafe extern "C" fn EmitDistance(mut distance: uint32_t, mut commands: *mut *mut uint32_t) {
    let mut d: uint32_t = distance.wrapping_add(3 as uint32_t);
    let mut nbits: uint32_t = Log2FloorNonZero(d as size_t).wrapping_sub(1 as uint32_t);
    let prefix: uint32_t = d >> nbits & 1 as uint32_t;
    let offset: uint32_t = (2 as uint32_t).wrapping_add(prefix) << nbits;
    let distcode: uint32_t = (2 as uint32_t)
        .wrapping_mul(nbits.wrapping_sub(1 as uint32_t))
        .wrapping_add(prefix)
        .wrapping_add(80 as uint32_t);
    let mut extra: uint32_t = d.wrapping_sub(offset);
    **commands = distcode | extra << 8 as ::core::ffi::c_int;
    *commands = (*commands).offset(1);
}
unsafe extern "C" fn BrotliStoreMetaBlockHeader(
    mut len: size_t,
    mut is_uncompressed: ::core::ffi::c_int,
    mut storage_ix: *mut size_t,
    mut storage: *mut uint8_t,
) {
    let mut nibbles: size_t = 6 as size_t;
    BrotliWriteBits(1 as size_t, 0 as uint64_t, storage_ix, storage);
    if len <= ((1 as ::core::ffi::c_uint) << 16 as ::core::ffi::c_int) as size_t {
        nibbles = 4 as size_t;
    } else if len <= ((1 as ::core::ffi::c_uint) << 20 as ::core::ffi::c_int) as size_t {
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
#[inline(always)]
unsafe extern "C" fn CreateCommands(
    mut input: *const uint8_t,
    mut block_size: size_t,
    mut input_size: size_t,
    mut base_ip: *const uint8_t,
    mut table: *mut ::core::ffi::c_int,
    mut table_bits: size_t,
    mut min_match: size_t,
    mut literals: *mut *mut uint8_t,
    mut commands: *mut *mut uint32_t,
) {
    let mut current_block: u64;
    let mut ip: *const uint8_t = input;
    let shift: size_t = (64 as size_t).wrapping_sub(table_bits);
    let mut ip_end: *const uint8_t = input.offset(block_size as isize);
    let mut next_emit: *const uint8_t = input;
    let mut last_distance: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let kInputMarginBytes: size_t = BROTLI_WINDOW_GAP as size_t;
    if (block_size >= kInputMarginBytes) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        let len_limit: size_t = brotli_min_size_t(
            block_size.wrapping_sub(min_match),
            input_size.wrapping_sub(kInputMarginBytes),
        ) as size_t;
        let mut ip_limit: *const uint8_t = input.offset(len_limit as isize);
        let mut next_hash: uint32_t = 0;
        ip = ip.offset(1);
        next_hash = Hash(ip, shift, min_match);
        's_30: loop {
            let mut skip: uint32_t = 32 as uint32_t;
            let mut next_ip: *const uint8_t = ip;
            let mut candidate: *const uint8_t = ::core::ptr::null::<uint8_t>();
            loop {
                let mut hash: uint32_t = next_hash;
                let fresh1 = skip;
                skip = skip.wrapping_add(1);
                let mut bytes_between_hash_lookups: uint32_t = fresh1 >> 5 as ::core::ffi::c_int;
                ip = next_ip;
                next_ip = ip.offset(bytes_between_hash_lookups as isize);
                if (next_ip > ip_limit) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
                    break 's_30;
                }
                next_hash = Hash(next_ip, shift, min_match);
                candidate = ip.offset(-(last_distance as isize));
                if IsMatch(ip, candidate, min_match) != 0 {
                    if (candidate < ip) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
                        *table.offset(hash as isize) =
                            ip.offset_from(base_ip) as ::core::ffi::c_long as ::core::ffi::c_int;
                        current_block = 11584701595673473500;
                    } else {
                        current_block = 15976848397966268834;
                    }
                } else {
                    current_block = 15976848397966268834;
                }
                match current_block {
                    15976848397966268834 => {
                        candidate = base_ip.offset(*table.offset(hash as isize) as isize);
                        *table.offset(hash as isize) =
                            ip.offset_from(base_ip) as ::core::ffi::c_long as ::core::ffi::c_int;
                        if (IsMatch(ip, candidate, min_match) == 0) as ::core::ffi::c_int
                            as ::core::ffi::c_long
                            != 0
                        {
                            continue;
                        }
                    }
                    _ => {}
                }
                if !(ip.offset_from(candidate) as ::core::ffi::c_long
                    > ((1 as ::core::ffi::c_int as size_t) << 18 as ::core::ffi::c_int)
                        .wrapping_sub(BROTLI_WINDOW_GAP as size_t)
                        as ::core::ffi::c_long)
                {
                    break;
                }
            }
            let mut base: *const uint8_t = ip;
            let mut matched: size_t = min_match.wrapping_add(FindMatchLengthWithLimit(
                candidate.offset(min_match as isize),
                ip.offset(min_match as isize),
                (ip_end.offset_from(ip) as ::core::ffi::c_long as size_t).wrapping_sub(min_match),
            ));
            let mut distance: ::core::ffi::c_int =
                base.offset_from(candidate) as ::core::ffi::c_long as ::core::ffi::c_int;
            let mut insert: ::core::ffi::c_int =
                base.offset_from(next_emit) as ::core::ffi::c_long as ::core::ffi::c_int;
            ip = ip.offset(matched as isize);
            EmitInsertLen(insert as uint32_t, commands);
            memcpy(
                *literals as *mut ::core::ffi::c_void,
                next_emit as *const ::core::ffi::c_void,
                insert as size_t,
            );
            *literals = (*literals).offset(insert as isize);
            if distance == last_distance {
                **commands = 64 as uint32_t;
                *commands = (*commands).offset(1);
            } else {
                EmitDistance(distance as uint32_t, commands);
                last_distance = distance;
            }
            EmitCopyLenLastDistance(matched, commands);
            next_emit = ip;
            if (ip >= ip_limit) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
                break;
            }
            let mut input_bytes: uint64_t = 0;
            let mut cur_hash: uint32_t = 0;
            let mut prev_hash: uint32_t = 0;
            if min_match == 4 as size_t {
                input_bytes =
                    BrotliUnalignedRead64(ip.offset(-(3 as ::core::ffi::c_int as isize))
                        as *const ::core::ffi::c_void);
                cur_hash = HashBytesAtOffset(input_bytes, 3 as size_t, shift, min_match);
                prev_hash = HashBytesAtOffset(input_bytes, 0 as size_t, shift, min_match);
                *table.offset(prev_hash as isize) = (ip.offset_from(base_ip) as ::core::ffi::c_long
                    - 3 as ::core::ffi::c_long)
                    as ::core::ffi::c_int;
                prev_hash = HashBytesAtOffset(input_bytes, 1 as size_t, shift, min_match);
                *table.offset(prev_hash as isize) = (ip.offset_from(base_ip) as ::core::ffi::c_long
                    - 2 as ::core::ffi::c_long)
                    as ::core::ffi::c_int;
                prev_hash = HashBytesAtOffset(input_bytes, 0 as size_t, shift, min_match);
                *table.offset(prev_hash as isize) = (ip.offset_from(base_ip) as ::core::ffi::c_long
                    - 1 as ::core::ffi::c_long)
                    as ::core::ffi::c_int;
            } else {
                input_bytes =
                    BrotliUnalignedRead64(ip.offset(-(5 as ::core::ffi::c_int as isize))
                        as *const ::core::ffi::c_void);
                prev_hash = HashBytesAtOffset(input_bytes, 0 as size_t, shift, min_match);
                *table.offset(prev_hash as isize) = (ip.offset_from(base_ip) as ::core::ffi::c_long
                    - 5 as ::core::ffi::c_long)
                    as ::core::ffi::c_int;
                prev_hash = HashBytesAtOffset(input_bytes, 1 as size_t, shift, min_match);
                *table.offset(prev_hash as isize) = (ip.offset_from(base_ip) as ::core::ffi::c_long
                    - 4 as ::core::ffi::c_long)
                    as ::core::ffi::c_int;
                prev_hash = HashBytesAtOffset(input_bytes, 2 as size_t, shift, min_match);
                *table.offset(prev_hash as isize) = (ip.offset_from(base_ip) as ::core::ffi::c_long
                    - 3 as ::core::ffi::c_long)
                    as ::core::ffi::c_int;
                input_bytes =
                    BrotliUnalignedRead64(ip.offset(-(2 as ::core::ffi::c_int as isize))
                        as *const ::core::ffi::c_void);
                cur_hash = HashBytesAtOffset(input_bytes, 2 as size_t, shift, min_match);
                prev_hash = HashBytesAtOffset(input_bytes, 0 as size_t, shift, min_match);
                *table.offset(prev_hash as isize) = (ip.offset_from(base_ip) as ::core::ffi::c_long
                    - 2 as ::core::ffi::c_long)
                    as ::core::ffi::c_int;
                prev_hash = HashBytesAtOffset(input_bytes, 1 as size_t, shift, min_match);
                *table.offset(prev_hash as isize) = (ip.offset_from(base_ip) as ::core::ffi::c_long
                    - 1 as ::core::ffi::c_long)
                    as ::core::ffi::c_int;
            }
            candidate = base_ip.offset(*table.offset(cur_hash as isize) as isize);
            *table.offset(cur_hash as isize) =
                ip.offset_from(base_ip) as ::core::ffi::c_long as ::core::ffi::c_int;
            while ip.offset_from(candidate) as ::core::ffi::c_long
                <= ((1 as ::core::ffi::c_int as size_t) << 18 as ::core::ffi::c_int)
                    .wrapping_sub(BROTLI_WINDOW_GAP as size_t)
                    as ::core::ffi::c_long
                && IsMatch(ip, candidate, min_match) != 0
            {
                let mut base_0: *const uint8_t = ip;
                let mut matched_0: size_t = min_match.wrapping_add(FindMatchLengthWithLimit(
                    candidate.offset(min_match as isize),
                    ip.offset(min_match as isize),
                    (ip_end.offset_from(ip) as ::core::ffi::c_long as size_t)
                        .wrapping_sub(min_match),
                ));
                ip = ip.offset(matched_0 as isize);
                last_distance =
                    base_0.offset_from(candidate) as ::core::ffi::c_long as ::core::ffi::c_int;
                EmitCopyLen(matched_0, commands);
                EmitDistance(last_distance as uint32_t, commands);
                next_emit = ip;
                if (ip >= ip_limit) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
                    break 's_30;
                }
                let mut input_bytes_0: uint64_t = 0;
                let mut cur_hash_0: uint32_t = 0;
                let mut prev_hash_0: uint32_t = 0;
                if min_match == 4 as size_t {
                    input_bytes_0 =
                        BrotliUnalignedRead64(ip.offset(-(3 as ::core::ffi::c_int as isize))
                            as *const ::core::ffi::c_void);
                    cur_hash_0 = HashBytesAtOffset(input_bytes_0, 3 as size_t, shift, min_match);
                    prev_hash_0 = HashBytesAtOffset(input_bytes_0, 0 as size_t, shift, min_match);
                    *table.offset(prev_hash_0 as isize) =
                        (ip.offset_from(base_ip) as ::core::ffi::c_long - 3 as ::core::ffi::c_long)
                            as ::core::ffi::c_int;
                    prev_hash_0 = HashBytesAtOffset(input_bytes_0, 1 as size_t, shift, min_match);
                    *table.offset(prev_hash_0 as isize) =
                        (ip.offset_from(base_ip) as ::core::ffi::c_long - 2 as ::core::ffi::c_long)
                            as ::core::ffi::c_int;
                    prev_hash_0 = HashBytesAtOffset(input_bytes_0, 2 as size_t, shift, min_match);
                    *table.offset(prev_hash_0 as isize) =
                        (ip.offset_from(base_ip) as ::core::ffi::c_long - 1 as ::core::ffi::c_long)
                            as ::core::ffi::c_int;
                } else {
                    input_bytes_0 =
                        BrotliUnalignedRead64(ip.offset(-(5 as ::core::ffi::c_int as isize))
                            as *const ::core::ffi::c_void);
                    prev_hash_0 = HashBytesAtOffset(input_bytes_0, 0 as size_t, shift, min_match);
                    *table.offset(prev_hash_0 as isize) =
                        (ip.offset_from(base_ip) as ::core::ffi::c_long - 5 as ::core::ffi::c_long)
                            as ::core::ffi::c_int;
                    prev_hash_0 = HashBytesAtOffset(input_bytes_0, 1 as size_t, shift, min_match);
                    *table.offset(prev_hash_0 as isize) =
                        (ip.offset_from(base_ip) as ::core::ffi::c_long - 4 as ::core::ffi::c_long)
                            as ::core::ffi::c_int;
                    prev_hash_0 = HashBytesAtOffset(input_bytes_0, 2 as size_t, shift, min_match);
                    *table.offset(prev_hash_0 as isize) =
                        (ip.offset_from(base_ip) as ::core::ffi::c_long - 3 as ::core::ffi::c_long)
                            as ::core::ffi::c_int;
                    input_bytes_0 =
                        BrotliUnalignedRead64(ip.offset(-(2 as ::core::ffi::c_int as isize))
                            as *const ::core::ffi::c_void);
                    cur_hash_0 = HashBytesAtOffset(input_bytes_0, 2 as size_t, shift, min_match);
                    prev_hash_0 = HashBytesAtOffset(input_bytes_0, 0 as size_t, shift, min_match);
                    *table.offset(prev_hash_0 as isize) =
                        (ip.offset_from(base_ip) as ::core::ffi::c_long - 2 as ::core::ffi::c_long)
                            as ::core::ffi::c_int;
                    prev_hash_0 = HashBytesAtOffset(input_bytes_0, 1 as size_t, shift, min_match);
                    *table.offset(prev_hash_0 as isize) =
                        (ip.offset_from(base_ip) as ::core::ffi::c_long - 1 as ::core::ffi::c_long)
                            as ::core::ffi::c_int;
                }
                candidate = base_ip.offset(*table.offset(cur_hash_0 as isize) as isize);
                *table.offset(cur_hash_0 as isize) =
                    ip.offset_from(base_ip) as ::core::ffi::c_long as ::core::ffi::c_int;
            }
            ip = ip.offset(1);
            next_hash = Hash(ip, shift, min_match);
        }
    }
    if next_emit < ip_end {
        let insert_0: uint32_t = ip_end.offset_from(next_emit) as ::core::ffi::c_long as uint32_t;
        EmitInsertLen(insert_0, commands);
        memcpy(
            *literals as *mut ::core::ffi::c_void,
            next_emit as *const ::core::ffi::c_void,
            insert_0 as size_t,
        );
        *literals = (*literals).offset(insert_0 as isize);
    }
}
unsafe extern "C" fn StoreCommands(
    mut s: *mut BrotliTwoPassArena,
    mut literals: *const uint8_t,
    num_literals: size_t,
    mut commands: *const uint32_t,
    num_commands: size_t,
    mut storage_ix: *mut size_t,
    mut storage: *mut uint8_t,
) {
    static mut kNumExtraBits: [uint32_t; 128] = [
        0 as ::core::ffi::c_int as uint32_t,
        0 as ::core::ffi::c_int as uint32_t,
        0 as ::core::ffi::c_int as uint32_t,
        0 as ::core::ffi::c_int as uint32_t,
        0 as ::core::ffi::c_int as uint32_t,
        0 as ::core::ffi::c_int as uint32_t,
        1 as ::core::ffi::c_int as uint32_t,
        1 as ::core::ffi::c_int as uint32_t,
        2 as ::core::ffi::c_int as uint32_t,
        2 as ::core::ffi::c_int as uint32_t,
        3 as ::core::ffi::c_int as uint32_t,
        3 as ::core::ffi::c_int as uint32_t,
        4 as ::core::ffi::c_int as uint32_t,
        4 as ::core::ffi::c_int as uint32_t,
        5 as ::core::ffi::c_int as uint32_t,
        5 as ::core::ffi::c_int as uint32_t,
        6 as ::core::ffi::c_int as uint32_t,
        7 as ::core::ffi::c_int as uint32_t,
        8 as ::core::ffi::c_int as uint32_t,
        9 as ::core::ffi::c_int as uint32_t,
        10 as ::core::ffi::c_int as uint32_t,
        12 as ::core::ffi::c_int as uint32_t,
        14 as ::core::ffi::c_int as uint32_t,
        24 as ::core::ffi::c_int as uint32_t,
        0 as ::core::ffi::c_int as uint32_t,
        0 as ::core::ffi::c_int as uint32_t,
        0 as ::core::ffi::c_int as uint32_t,
        0 as ::core::ffi::c_int as uint32_t,
        0 as ::core::ffi::c_int as uint32_t,
        0 as ::core::ffi::c_int as uint32_t,
        0 as ::core::ffi::c_int as uint32_t,
        0 as ::core::ffi::c_int as uint32_t,
        1 as ::core::ffi::c_int as uint32_t,
        1 as ::core::ffi::c_int as uint32_t,
        2 as ::core::ffi::c_int as uint32_t,
        2 as ::core::ffi::c_int as uint32_t,
        3 as ::core::ffi::c_int as uint32_t,
        3 as ::core::ffi::c_int as uint32_t,
        4 as ::core::ffi::c_int as uint32_t,
        4 as ::core::ffi::c_int as uint32_t,
        0 as ::core::ffi::c_int as uint32_t,
        0 as ::core::ffi::c_int as uint32_t,
        0 as ::core::ffi::c_int as uint32_t,
        0 as ::core::ffi::c_int as uint32_t,
        0 as ::core::ffi::c_int as uint32_t,
        0 as ::core::ffi::c_int as uint32_t,
        0 as ::core::ffi::c_int as uint32_t,
        0 as ::core::ffi::c_int as uint32_t,
        1 as ::core::ffi::c_int as uint32_t,
        1 as ::core::ffi::c_int as uint32_t,
        2 as ::core::ffi::c_int as uint32_t,
        2 as ::core::ffi::c_int as uint32_t,
        3 as ::core::ffi::c_int as uint32_t,
        3 as ::core::ffi::c_int as uint32_t,
        4 as ::core::ffi::c_int as uint32_t,
        4 as ::core::ffi::c_int as uint32_t,
        5 as ::core::ffi::c_int as uint32_t,
        5 as ::core::ffi::c_int as uint32_t,
        6 as ::core::ffi::c_int as uint32_t,
        7 as ::core::ffi::c_int as uint32_t,
        8 as ::core::ffi::c_int as uint32_t,
        9 as ::core::ffi::c_int as uint32_t,
        10 as ::core::ffi::c_int as uint32_t,
        24 as ::core::ffi::c_int as uint32_t,
        0 as ::core::ffi::c_int as uint32_t,
        0 as ::core::ffi::c_int as uint32_t,
        0 as ::core::ffi::c_int as uint32_t,
        0 as ::core::ffi::c_int as uint32_t,
        0 as ::core::ffi::c_int as uint32_t,
        0 as ::core::ffi::c_int as uint32_t,
        0 as ::core::ffi::c_int as uint32_t,
        0 as ::core::ffi::c_int as uint32_t,
        0 as ::core::ffi::c_int as uint32_t,
        0 as ::core::ffi::c_int as uint32_t,
        0 as ::core::ffi::c_int as uint32_t,
        0 as ::core::ffi::c_int as uint32_t,
        0 as ::core::ffi::c_int as uint32_t,
        0 as ::core::ffi::c_int as uint32_t,
        0 as ::core::ffi::c_int as uint32_t,
        0 as ::core::ffi::c_int as uint32_t,
        1 as ::core::ffi::c_int as uint32_t,
        1 as ::core::ffi::c_int as uint32_t,
        2 as ::core::ffi::c_int as uint32_t,
        2 as ::core::ffi::c_int as uint32_t,
        3 as ::core::ffi::c_int as uint32_t,
        3 as ::core::ffi::c_int as uint32_t,
        4 as ::core::ffi::c_int as uint32_t,
        4 as ::core::ffi::c_int as uint32_t,
        5 as ::core::ffi::c_int as uint32_t,
        5 as ::core::ffi::c_int as uint32_t,
        6 as ::core::ffi::c_int as uint32_t,
        6 as ::core::ffi::c_int as uint32_t,
        7 as ::core::ffi::c_int as uint32_t,
        7 as ::core::ffi::c_int as uint32_t,
        8 as ::core::ffi::c_int as uint32_t,
        8 as ::core::ffi::c_int as uint32_t,
        9 as ::core::ffi::c_int as uint32_t,
        9 as ::core::ffi::c_int as uint32_t,
        10 as ::core::ffi::c_int as uint32_t,
        10 as ::core::ffi::c_int as uint32_t,
        11 as ::core::ffi::c_int as uint32_t,
        11 as ::core::ffi::c_int as uint32_t,
        12 as ::core::ffi::c_int as uint32_t,
        12 as ::core::ffi::c_int as uint32_t,
        13 as ::core::ffi::c_int as uint32_t,
        13 as ::core::ffi::c_int as uint32_t,
        14 as ::core::ffi::c_int as uint32_t,
        14 as ::core::ffi::c_int as uint32_t,
        15 as ::core::ffi::c_int as uint32_t,
        15 as ::core::ffi::c_int as uint32_t,
        16 as ::core::ffi::c_int as uint32_t,
        16 as ::core::ffi::c_int as uint32_t,
        17 as ::core::ffi::c_int as uint32_t,
        17 as ::core::ffi::c_int as uint32_t,
        18 as ::core::ffi::c_int as uint32_t,
        18 as ::core::ffi::c_int as uint32_t,
        19 as ::core::ffi::c_int as uint32_t,
        19 as ::core::ffi::c_int as uint32_t,
        20 as ::core::ffi::c_int as uint32_t,
        20 as ::core::ffi::c_int as uint32_t,
        21 as ::core::ffi::c_int as uint32_t,
        21 as ::core::ffi::c_int as uint32_t,
        22 as ::core::ffi::c_int as uint32_t,
        22 as ::core::ffi::c_int as uint32_t,
        23 as ::core::ffi::c_int as uint32_t,
        23 as ::core::ffi::c_int as uint32_t,
        24 as ::core::ffi::c_int as uint32_t,
        24 as ::core::ffi::c_int as uint32_t,
    ];
    static mut kInsertOffset: [uint32_t; 24] = [
        0 as ::core::ffi::c_int as uint32_t,
        1 as ::core::ffi::c_int as uint32_t,
        2 as ::core::ffi::c_int as uint32_t,
        3 as ::core::ffi::c_int as uint32_t,
        4 as ::core::ffi::c_int as uint32_t,
        5 as ::core::ffi::c_int as uint32_t,
        6 as ::core::ffi::c_int as uint32_t,
        8 as ::core::ffi::c_int as uint32_t,
        10 as ::core::ffi::c_int as uint32_t,
        14 as ::core::ffi::c_int as uint32_t,
        18 as ::core::ffi::c_int as uint32_t,
        26 as ::core::ffi::c_int as uint32_t,
        34 as ::core::ffi::c_int as uint32_t,
        50 as ::core::ffi::c_int as uint32_t,
        66 as ::core::ffi::c_int as uint32_t,
        98 as ::core::ffi::c_int as uint32_t,
        130 as ::core::ffi::c_int as uint32_t,
        194 as ::core::ffi::c_int as uint32_t,
        322 as ::core::ffi::c_int as uint32_t,
        578 as ::core::ffi::c_int as uint32_t,
        1090 as ::core::ffi::c_int as uint32_t,
        2114 as ::core::ffi::c_int as uint32_t,
        6210 as ::core::ffi::c_int as uint32_t,
        22594 as ::core::ffi::c_int as uint32_t,
    ];
    let mut i: size_t = 0;
    memset(
        &raw mut (*s).lit_histo as *mut uint32_t as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<[uint32_t; 256]>() as size_t,
    );
    memset(
        &raw mut (*s).cmd_depth as *mut uint8_t as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<[uint8_t; 128]>() as size_t,
    );
    memset(
        &raw mut (*s).cmd_bits as *mut uint16_t as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<[uint16_t; 128]>() as size_t,
    );
    memset(
        &raw mut (*s).cmd_histo as *mut uint32_t as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<[uint32_t; 128]>() as size_t,
    );
    i = 0 as size_t;
    while i < num_literals {
        (*s).lit_histo[*literals.offset(i as isize) as usize] =
            (*s).lit_histo[*literals.offset(i as isize) as usize].wrapping_add(1);
        i = i.wrapping_add(1);
    }
    BrotliBuildAndStoreHuffmanTreeFast(
        &raw mut (*s).tmp_tree as *mut HuffmanTree,
        &raw mut (*s).lit_histo as *mut uint32_t,
        num_literals,
        8 as size_t,
        &raw mut (*s).lit_depth as *mut uint8_t,
        &raw mut (*s).lit_bits as *mut uint16_t,
        storage_ix,
        storage,
    );
    i = 0 as size_t;
    while i < num_commands {
        let code: uint32_t = *commands.offset(i as isize) & 0xff as uint32_t;
        (*s).cmd_histo[code as usize] = (*s).cmd_histo[code as usize].wrapping_add(1);
        i = i.wrapping_add(1);
    }
    (*s).cmd_histo[1 as ::core::ffi::c_int as usize] =
        ((*s).cmd_histo[1 as ::core::ffi::c_int as usize] as ::core::ffi::c_uint)
            .wrapping_add(1 as ::core::ffi::c_uint) as uint32_t as uint32_t;
    (*s).cmd_histo[2 as ::core::ffi::c_int as usize] =
        ((*s).cmd_histo[2 as ::core::ffi::c_int as usize] as ::core::ffi::c_uint)
            .wrapping_add(1 as ::core::ffi::c_uint) as uint32_t as uint32_t;
    (*s).cmd_histo[64 as ::core::ffi::c_int as usize] =
        ((*s).cmd_histo[64 as ::core::ffi::c_int as usize] as ::core::ffi::c_uint)
            .wrapping_add(1 as ::core::ffi::c_uint) as uint32_t as uint32_t;
    (*s).cmd_histo[84 as ::core::ffi::c_int as usize] =
        ((*s).cmd_histo[84 as ::core::ffi::c_int as usize] as ::core::ffi::c_uint)
            .wrapping_add(1 as ::core::ffi::c_uint) as uint32_t as uint32_t;
    BuildAndStoreCommandPrefixCode(s, storage_ix, storage);
    i = 0 as size_t;
    while i < num_commands {
        let cmd: uint32_t = *commands.offset(i as isize);
        let code_0: uint32_t = cmd & 0xff as uint32_t;
        let extra: uint32_t = cmd >> 8 as ::core::ffi::c_int;
        BrotliWriteBits(
            (*s).cmd_depth[code_0 as usize] as size_t,
            (*s).cmd_bits[code_0 as usize] as uint64_t,
            storage_ix,
            storage,
        );
        BrotliWriteBits(
            kNumExtraBits[code_0 as usize] as size_t,
            extra as uint64_t,
            storage_ix,
            storage,
        );
        if code_0 < 24 as uint32_t {
            let insert: uint32_t = kInsertOffset[code_0 as usize].wrapping_add(extra);
            let mut j: uint32_t = 0;
            j = 0 as uint32_t;
            while j < insert {
                let lit: uint8_t = *literals;
                BrotliWriteBits(
                    (*s).lit_depth[lit as usize] as size_t,
                    (*s).lit_bits[lit as usize] as uint64_t,
                    storage_ix,
                    storage,
                );
                literals = literals.offset(1);
                j = j.wrapping_add(1);
            }
        }
        i = i.wrapping_add(1);
    }
}
pub const MIN_RATIO: ::core::ffi::c_double = 0.98f64;
pub const SAMPLE_RATE: ::core::ffi::c_int = 43 as ::core::ffi::c_int;
unsafe extern "C" fn ShouldCompress(
    mut s: *mut BrotliTwoPassArena,
    mut input: *const uint8_t,
    mut input_size: size_t,
    mut num_literals: size_t,
) -> ::core::ffi::c_int {
    let mut corpus_size: ::core::ffi::c_double = input_size as ::core::ffi::c_double;
    if (num_literals as ::core::ffi::c_double) < MIN_RATIO * corpus_size {
        return BROTLI_TRUE;
    } else {
        let max_total_bit_cost: ::core::ffi::c_double =
            corpus_size * 8 as ::core::ffi::c_int as ::core::ffi::c_double * MIN_RATIO
                / SAMPLE_RATE as ::core::ffi::c_double;
        let mut i: size_t = 0;
        memset(
            &raw mut (*s).lit_histo as *mut uint32_t as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<[uint32_t; 256]>() as size_t,
        );
        i = 0 as size_t;
        while i < input_size {
            (*s).lit_histo[*input.offset(i as isize) as usize] =
                (*s).lit_histo[*input.offset(i as isize) as usize].wrapping_add(1);
            i = (i as ::core::ffi::c_ulong).wrapping_add(SAMPLE_RATE as ::core::ffi::c_ulong)
                as size_t as size_t;
        }
        return if BrotliBitsEntropy(&raw mut (*s).lit_histo as *mut uint32_t, 256 as size_t)
            < max_total_bit_cost
        {
            BROTLI_TRUE
        } else {
            BROTLI_FALSE
        };
    };
}
unsafe extern "C" fn RewindBitPosition(
    new_storage_ix: size_t,
    mut storage_ix: *mut size_t,
    mut storage: *mut uint8_t,
) {
    let bitpos: size_t = new_storage_ix & 7 as size_t;
    let mask: size_t =
        ((1 as ::core::ffi::c_uint) << bitpos).wrapping_sub(1 as ::core::ffi::c_uint) as size_t;
    let ref mut fresh0 = *storage.offset((new_storage_ix >> 3 as ::core::ffi::c_int) as isize);
    *fresh0 = (*fresh0 as ::core::ffi::c_int & mask as uint8_t as ::core::ffi::c_int) as uint8_t;
    *storage_ix = new_storage_ix;
}
unsafe extern "C" fn EmitUncompressedMetaBlock(
    mut input: *const uint8_t,
    mut input_size: size_t,
    mut storage_ix: *mut size_t,
    mut storage: *mut uint8_t,
) {
    BrotliStoreMetaBlockHeader(input_size, 1 as ::core::ffi::c_int, storage_ix, storage);
    *storage_ix = (*storage_ix).wrapping_add(7 as size_t) & !(7 as ::core::ffi::c_uint) as size_t;
    memcpy(
        storage.offset((*storage_ix >> 3 as ::core::ffi::c_int) as isize) as *mut uint8_t
            as *mut ::core::ffi::c_void,
        input as *const ::core::ffi::c_void,
        input_size,
    );
    *storage_ix = (*storage_ix as ::core::ffi::c_ulong)
        .wrapping_add((input_size << 3 as ::core::ffi::c_int) as ::core::ffi::c_ulong)
        as size_t as size_t;
    *storage.offset((*storage_ix >> 3 as ::core::ffi::c_int) as isize) = 0 as uint8_t;
}
#[inline(always)]
unsafe extern "C" fn BrotliCompressFragmentTwoPassImpl(
    mut s: *mut BrotliTwoPassArena,
    mut input: *const uint8_t,
    mut input_size: size_t,
    mut is_last: ::core::ffi::c_int,
    mut command_buf: *mut uint32_t,
    mut literal_buf: *mut uint8_t,
    mut table: *mut ::core::ffi::c_int,
    mut table_bits: size_t,
    mut min_match: size_t,
    mut storage_ix: *mut size_t,
    mut storage: *mut uint8_t,
) {
    let mut base_ip: *const uint8_t = input;
    while input_size > 0 as size_t {
        let mut block_size: size_t =
            brotli_min_size_t(input_size, kCompressFragmentTwoPassBlockSize);
        let mut commands: *mut uint32_t = command_buf;
        let mut literals: *mut uint8_t = literal_buf;
        let mut num_literals: size_t = 0;
        CreateCommands(
            input,
            block_size,
            input_size,
            base_ip,
            table,
            table_bits,
            min_match,
            &raw mut literals,
            &raw mut commands,
        );
        num_literals = literals.offset_from(literal_buf) as ::core::ffi::c_long as size_t;
        if ShouldCompress(s, input, block_size, num_literals) != 0 {
            let num_commands: size_t =
                commands.offset_from(command_buf) as ::core::ffi::c_long as size_t;
            BrotliStoreMetaBlockHeader(block_size, 0 as ::core::ffi::c_int, storage_ix, storage);
            BrotliWriteBits(13 as size_t, 0 as uint64_t, storage_ix, storage);
            StoreCommands(
                s,
                literal_buf,
                num_literals,
                command_buf,
                num_commands,
                storage_ix,
                storage,
            );
        } else {
            EmitUncompressedMetaBlock(input, block_size, storage_ix, storage);
        }
        input = input.offset(block_size as isize);
        input_size = (input_size as ::core::ffi::c_ulong)
            .wrapping_sub(block_size as ::core::ffi::c_ulong) as size_t
            as size_t;
    }
}
#[inline(never)]
unsafe extern "C" fn BrotliCompressFragmentTwoPassImpl13(
    mut s: *mut BrotliTwoPassArena,
    mut input: *const uint8_t,
    mut input_size: size_t,
    mut is_last: ::core::ffi::c_int,
    mut command_buf: *mut uint32_t,
    mut literal_buf: *mut uint8_t,
    mut table: *mut ::core::ffi::c_int,
    mut storage_ix: *mut size_t,
    mut storage: *mut uint8_t,
) {
    let mut min_match: size_t = (if 13 as ::core::ffi::c_int <= 15 as ::core::ffi::c_int {
        4 as ::core::ffi::c_int
    } else {
        6 as ::core::ffi::c_int
    }) as size_t;
    BrotliCompressFragmentTwoPassImpl(
        s,
        input,
        input_size,
        is_last,
        command_buf,
        literal_buf,
        table,
        13 as size_t,
        min_match,
        storage_ix,
        storage,
    );
}
#[inline(never)]
unsafe extern "C" fn BrotliCompressFragmentTwoPassImpl17(
    mut s: *mut BrotliTwoPassArena,
    mut input: *const uint8_t,
    mut input_size: size_t,
    mut is_last: ::core::ffi::c_int,
    mut command_buf: *mut uint32_t,
    mut literal_buf: *mut uint8_t,
    mut table: *mut ::core::ffi::c_int,
    mut storage_ix: *mut size_t,
    mut storage: *mut uint8_t,
) {
    let mut min_match: size_t = (if 17 as ::core::ffi::c_int <= 15 as ::core::ffi::c_int {
        4 as ::core::ffi::c_int
    } else {
        6 as ::core::ffi::c_int
    }) as size_t;
    BrotliCompressFragmentTwoPassImpl(
        s,
        input,
        input_size,
        is_last,
        command_buf,
        literal_buf,
        table,
        17 as size_t,
        min_match,
        storage_ix,
        storage,
    );
}
#[inline(never)]
unsafe extern "C" fn BrotliCompressFragmentTwoPassImpl12(
    mut s: *mut BrotliTwoPassArena,
    mut input: *const uint8_t,
    mut input_size: size_t,
    mut is_last: ::core::ffi::c_int,
    mut command_buf: *mut uint32_t,
    mut literal_buf: *mut uint8_t,
    mut table: *mut ::core::ffi::c_int,
    mut storage_ix: *mut size_t,
    mut storage: *mut uint8_t,
) {
    let mut min_match: size_t = (if 12 as ::core::ffi::c_int <= 15 as ::core::ffi::c_int {
        4 as ::core::ffi::c_int
    } else {
        6 as ::core::ffi::c_int
    }) as size_t;
    BrotliCompressFragmentTwoPassImpl(
        s,
        input,
        input_size,
        is_last,
        command_buf,
        literal_buf,
        table,
        12 as size_t,
        min_match,
        storage_ix,
        storage,
    );
}
#[inline(never)]
unsafe extern "C" fn BrotliCompressFragmentTwoPassImpl16(
    mut s: *mut BrotliTwoPassArena,
    mut input: *const uint8_t,
    mut input_size: size_t,
    mut is_last: ::core::ffi::c_int,
    mut command_buf: *mut uint32_t,
    mut literal_buf: *mut uint8_t,
    mut table: *mut ::core::ffi::c_int,
    mut storage_ix: *mut size_t,
    mut storage: *mut uint8_t,
) {
    let mut min_match: size_t = (if 16 as ::core::ffi::c_int <= 15 as ::core::ffi::c_int {
        4 as ::core::ffi::c_int
    } else {
        6 as ::core::ffi::c_int
    }) as size_t;
    BrotliCompressFragmentTwoPassImpl(
        s,
        input,
        input_size,
        is_last,
        command_buf,
        literal_buf,
        table,
        16 as size_t,
        min_match,
        storage_ix,
        storage,
    );
}
#[inline(never)]
unsafe extern "C" fn BrotliCompressFragmentTwoPassImpl15(
    mut s: *mut BrotliTwoPassArena,
    mut input: *const uint8_t,
    mut input_size: size_t,
    mut is_last: ::core::ffi::c_int,
    mut command_buf: *mut uint32_t,
    mut literal_buf: *mut uint8_t,
    mut table: *mut ::core::ffi::c_int,
    mut storage_ix: *mut size_t,
    mut storage: *mut uint8_t,
) {
    let mut min_match: size_t = (if 15 as ::core::ffi::c_int <= 15 as ::core::ffi::c_int {
        4 as ::core::ffi::c_int
    } else {
        6 as ::core::ffi::c_int
    }) as size_t;
    BrotliCompressFragmentTwoPassImpl(
        s,
        input,
        input_size,
        is_last,
        command_buf,
        literal_buf,
        table,
        15 as size_t,
        min_match,
        storage_ix,
        storage,
    );
}
#[inline(never)]
unsafe extern "C" fn BrotliCompressFragmentTwoPassImpl14(
    mut s: *mut BrotliTwoPassArena,
    mut input: *const uint8_t,
    mut input_size: size_t,
    mut is_last: ::core::ffi::c_int,
    mut command_buf: *mut uint32_t,
    mut literal_buf: *mut uint8_t,
    mut table: *mut ::core::ffi::c_int,
    mut storage_ix: *mut size_t,
    mut storage: *mut uint8_t,
) {
    let mut min_match: size_t = (if 14 as ::core::ffi::c_int <= 15 as ::core::ffi::c_int {
        4 as ::core::ffi::c_int
    } else {
        6 as ::core::ffi::c_int
    }) as size_t;
    BrotliCompressFragmentTwoPassImpl(
        s,
        input,
        input_size,
        is_last,
        command_buf,
        literal_buf,
        table,
        14 as size_t,
        min_match,
        storage_ix,
        storage,
    );
}
#[inline(never)]
unsafe extern "C" fn BrotliCompressFragmentTwoPassImpl10(
    mut s: *mut BrotliTwoPassArena,
    mut input: *const uint8_t,
    mut input_size: size_t,
    mut is_last: ::core::ffi::c_int,
    mut command_buf: *mut uint32_t,
    mut literal_buf: *mut uint8_t,
    mut table: *mut ::core::ffi::c_int,
    mut storage_ix: *mut size_t,
    mut storage: *mut uint8_t,
) {
    let mut min_match: size_t = (if 10 as ::core::ffi::c_int <= 15 as ::core::ffi::c_int {
        4 as ::core::ffi::c_int
    } else {
        6 as ::core::ffi::c_int
    }) as size_t;
    BrotliCompressFragmentTwoPassImpl(
        s,
        input,
        input_size,
        is_last,
        command_buf,
        literal_buf,
        table,
        10 as size_t,
        min_match,
        storage_ix,
        storage,
    );
}
#[inline(never)]
unsafe extern "C" fn BrotliCompressFragmentTwoPassImpl11(
    mut s: *mut BrotliTwoPassArena,
    mut input: *const uint8_t,
    mut input_size: size_t,
    mut is_last: ::core::ffi::c_int,
    mut command_buf: *mut uint32_t,
    mut literal_buf: *mut uint8_t,
    mut table: *mut ::core::ffi::c_int,
    mut storage_ix: *mut size_t,
    mut storage: *mut uint8_t,
) {
    let mut min_match: size_t = (if 11 as ::core::ffi::c_int <= 15 as ::core::ffi::c_int {
        4 as ::core::ffi::c_int
    } else {
        6 as ::core::ffi::c_int
    }) as size_t;
    BrotliCompressFragmentTwoPassImpl(
        s,
        input,
        input_size,
        is_last,
        command_buf,
        literal_buf,
        table,
        11 as size_t,
        min_match,
        storage_ix,
        storage,
    );
}
#[inline(never)]
unsafe extern "C" fn BrotliCompressFragmentTwoPassImpl9(
    mut s: *mut BrotliTwoPassArena,
    mut input: *const uint8_t,
    mut input_size: size_t,
    mut is_last: ::core::ffi::c_int,
    mut command_buf: *mut uint32_t,
    mut literal_buf: *mut uint8_t,
    mut table: *mut ::core::ffi::c_int,
    mut storage_ix: *mut size_t,
    mut storage: *mut uint8_t,
) {
    let mut min_match: size_t = (if 9 as ::core::ffi::c_int <= 15 as ::core::ffi::c_int {
        4 as ::core::ffi::c_int
    } else {
        6 as ::core::ffi::c_int
    }) as size_t;
    BrotliCompressFragmentTwoPassImpl(
        s,
        input,
        input_size,
        is_last,
        command_buf,
        literal_buf,
        table,
        9 as size_t,
        min_match,
        storage_ix,
        storage,
    );
}
#[inline(never)]
unsafe extern "C" fn BrotliCompressFragmentTwoPassImpl8(
    mut s: *mut BrotliTwoPassArena,
    mut input: *const uint8_t,
    mut input_size: size_t,
    mut is_last: ::core::ffi::c_int,
    mut command_buf: *mut uint32_t,
    mut literal_buf: *mut uint8_t,
    mut table: *mut ::core::ffi::c_int,
    mut storage_ix: *mut size_t,
    mut storage: *mut uint8_t,
) {
    let mut min_match: size_t = (if 8 as ::core::ffi::c_int <= 15 as ::core::ffi::c_int {
        4 as ::core::ffi::c_int
    } else {
        6 as ::core::ffi::c_int
    }) as size_t;
    BrotliCompressFragmentTwoPassImpl(
        s,
        input,
        input_size,
        is_last,
        command_buf,
        literal_buf,
        table,
        8 as size_t,
        min_match,
        storage_ix,
        storage,
    );
}
#[no_mangle]
pub unsafe extern "C" fn BrotliCompressFragmentTwoPass(
    mut s: *mut BrotliTwoPassArena,
    mut input: *const uint8_t,
    mut input_size: size_t,
    mut is_last: ::core::ffi::c_int,
    mut command_buf: *mut uint32_t,
    mut literal_buf: *mut uint8_t,
    mut table: *mut ::core::ffi::c_int,
    mut table_size: size_t,
    mut storage_ix: *mut size_t,
    mut storage: *mut uint8_t,
) {
    let initial_storage_ix: size_t = *storage_ix;
    let table_bits: size_t = Log2FloorNonZero(table_size) as size_t;
    match table_bits {
        8 => {
            BrotliCompressFragmentTwoPassImpl8(
                s,
                input,
                input_size,
                is_last,
                command_buf,
                literal_buf,
                table,
                storage_ix,
                storage,
            );
        }
        9 => {
            BrotliCompressFragmentTwoPassImpl9(
                s,
                input,
                input_size,
                is_last,
                command_buf,
                literal_buf,
                table,
                storage_ix,
                storage,
            );
        }
        10 => {
            BrotliCompressFragmentTwoPassImpl10(
                s,
                input,
                input_size,
                is_last,
                command_buf,
                literal_buf,
                table,
                storage_ix,
                storage,
            );
        }
        11 => {
            BrotliCompressFragmentTwoPassImpl11(
                s,
                input,
                input_size,
                is_last,
                command_buf,
                literal_buf,
                table,
                storage_ix,
                storage,
            );
        }
        12 => {
            BrotliCompressFragmentTwoPassImpl12(
                s,
                input,
                input_size,
                is_last,
                command_buf,
                literal_buf,
                table,
                storage_ix,
                storage,
            );
        }
        13 => {
            BrotliCompressFragmentTwoPassImpl13(
                s,
                input,
                input_size,
                is_last,
                command_buf,
                literal_buf,
                table,
                storage_ix,
                storage,
            );
        }
        14 => {
            BrotliCompressFragmentTwoPassImpl14(
                s,
                input,
                input_size,
                is_last,
                command_buf,
                literal_buf,
                table,
                storage_ix,
                storage,
            );
        }
        15 => {
            BrotliCompressFragmentTwoPassImpl15(
                s,
                input,
                input_size,
                is_last,
                command_buf,
                literal_buf,
                table,
                storage_ix,
                storage,
            );
        }
        16 => {
            BrotliCompressFragmentTwoPassImpl16(
                s,
                input,
                input_size,
                is_last,
                command_buf,
                literal_buf,
                table,
                storage_ix,
                storage,
            );
        }
        17 => {
            BrotliCompressFragmentTwoPassImpl17(
                s,
                input,
                input_size,
                is_last,
                command_buf,
                literal_buf,
                table,
                storage_ix,
                storage,
            );
        }
        _ => {}
    }
    if (*storage_ix).wrapping_sub(initial_storage_ix)
        > (31 as size_t).wrapping_add(input_size << 3 as ::core::ffi::c_int)
    {
        RewindBitPosition(initial_storage_ix, storage_ix, storage);
        EmitUncompressedMetaBlock(input, input_size, storage_ix, storage);
    }
    if is_last != 0 {
        BrotliWriteBits(1 as size_t, 1 as uint64_t, storage_ix, storage);
        BrotliWriteBits(1 as size_t, 1 as uint64_t, storage_ix, storage);
        *storage_ix =
            (*storage_ix).wrapping_add(7 as size_t) & !(7 as ::core::ffi::c_uint) as size_t;
    }
}
#[inline(always)]
unsafe extern "C" fn Log2FloorNonZero(mut n: size_t) -> uint32_t {
    return 31 as uint32_t ^ (n as uint32_t).leading_zeros() as i32 as uint32_t;
}
#[inline(always)]
unsafe extern "C" fn FindMatchLengthWithLimit(
    mut s1: *const uint8_t,
    mut s2: *const uint8_t,
    mut limit: size_t,
) -> size_t {
    let mut s1_orig: *const uint8_t = s1;
    while limit >= 8 as size_t {
        let mut x: uint64_t = BrotliUnalignedRead64(s2 as *const ::core::ffi::c_void)
            ^ BrotliUnalignedRead64(s1 as *const ::core::ffi::c_void);
        s2 = s2.offset(8 as ::core::ffi::c_int as isize);
        if x != 0 as uint64_t {
            let mut matching_bits: size_t =
                (x as ::core::ffi::c_ulonglong).trailing_zeros() as i32 as size_t;
            return (s1.offset_from(s1_orig) as ::core::ffi::c_long as size_t)
                .wrapping_add(matching_bits >> 3 as ::core::ffi::c_int);
        }
        s1 = s1.offset(8 as ::core::ffi::c_int as isize);
        limit = (limit as ::core::ffi::c_ulong).wrapping_sub(8 as ::core::ffi::c_ulong) as size_t
            as size_t;
    }
    while limit != 0 && *s1 as ::core::ffi::c_int == *s2 as ::core::ffi::c_int {
        limit = limit.wrapping_sub(1);
        s2 = s2.offset(1);
        s1 = s1.offset(1);
    }
    return s1.offset_from(s1_orig) as ::core::ffi::c_long as size_t;
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
        array.offset((*pos >> 3 as ::core::ffi::c_int) as isize) as *mut uint8_t;
    let mut v: uint64_t = *p as uint64_t;
    v = (v as ::core::ffi::c_ulong | (bits << (*pos & 7 as size_t)) as ::core::ffi::c_ulong)
        as uint64_t;
    BrotliUnalignedWrite64(p as *mut ::core::ffi::c_void, v);
    *pos = (*pos as ::core::ffi::c_ulong).wrapping_add(n_bits as ::core::ffi::c_ulong) as size_t
        as size_t;
}
