use core::ffi::*;
use crate::src::enc::memory::BrotliAllocate;
use crate::src::enc::entropy_encode::BrotliConvertBitDepthsToSymbols;
use crate::src::enc::entropy_encode::BrotliCreateHuffmanTree;
use crate::src::enc::memory::BrotliFree;
use crate::src::c_inlined_fns::BrotliUnalignedWrite64;
use crate::src::enc::entropy_encode::BrotliWriteHuffmanTree;
use crate::src::c_inlined_fns::GetCopyExtra;
use crate::src::c_inlined_fns::GetInsertExtra;
use crate::src::c_inlined_fns::HistogramAddCommand;
use crate::src::c_inlined_fns::HistogramAddDistance;
use crate::src::c_inlined_fns::HistogramAddLiteral;
use crate::src::c_inlined_fns::HistogramClearCommand;
use crate::src::c_inlined_fns::HistogramClearDistance;
use crate::src::c_inlined_fns::HistogramClearLiteral;
use crate::src::c_inlined_fns::InitHuffmanTree;
use crate::src::c_inlined_fns::Log2FloorNonZero;
use crate::src::c_inlined_fns::brotli_max_uint32_t;
use crate::src::c_inlined_fns::brotli_min_uint32_t;
pub use crate::src::c_inlined_fns::*;
pub use crate::src::ffi::*;
pub use crate::src::c_consts::*;
pub use crate::src::c_structs::*;
pub use crate::src::c_types::*;
extern "C" {
    static kBrotliInsBase: [uint32_t; 24];
    static kBrotliCopyBase: [uint32_t; 24];
    fn BrotliSetDepth(
        p: c_int,
        pool: *mut HuffmanTree,
        depth: *mut uint8_t,
        max_depth: c_int,
    ) -> c_int;
    static kBrotliShellGaps: [size_t; 6];
}

pub type __int32_t = i32;

pub type int32_t = __int32_t;

pub use crate::src::enc::backward_references::BrotliEncoderDictionary;

pub use crate::src::enc::backward_references::ContextualEncoderDictionary;

pub use crate::src::enc::backward_references::SharedEncoderDictionary;


pub use crate::src::enc::backward_references::BrotliEncoderParams;

pub use crate::src::enc::backward_references::Command;


#[derive(Copy, Clone)]
#[repr(C)]
pub struct StoreMetablockArena {
    pub literal_enc: BlockEncoder,
    pub command_enc: BlockEncoder,
    pub distance_enc: BlockEncoder,
    pub context_map_arena: EncodeContextMapArena,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EncodeContextMapArena {
    pub histogram: [uint32_t; 272],
    pub depths: [uint8_t; 272],
    pub bits: [uint16_t; 272],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BlockEncoder {
    pub histogram_length_: size_t,
    pub num_block_types_: size_t,
    pub block_types_: *const uint8_t,
    pub block_lengths_: *const uint32_t,
    pub num_blocks_: size_t,
    pub block_split_code_: BlockSplitCode,
    pub block_ix_: size_t,
    pub block_len_: size_t,
    pub entropy_ix_: size_t,
    pub depths_: *mut uint8_t,
    pub bits_: *mut uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BlockSplitCode {
    pub type_code_calculator: BlockTypeCodeCalculator,
    pub type_depths: [uint8_t; 258],
    pub type_bits: [uint16_t; 258],
    pub length_depths: [uint8_t; 26],
    pub length_bits: [uint16_t; 26],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BlockTypeCodeCalculator {
    pub last_type: size_t,
    pub second_last_type: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MetablockArena {
    pub lit_histo: HistogramLiteral,
    pub cmd_histo: HistogramCommand,
    pub dist_histo: HistogramDistance,
    pub lit_depth: [uint8_t; 256],
    pub lit_bits: [uint16_t; 256],
    pub cmd_depth: [uint8_t; 704],
    pub cmd_bits: [uint16_t; 704],
    pub dist_depth: [uint8_t; 140],
    pub dist_bits: [uint16_t; 140],
    pub tree: [HuffmanTree; 1409],
}
static mut kSymbolMask: uint32_t = 0;

pub const BROTLI_REPEAT_PREVIOUS_CODE_LENGTH: size_t = 16 as size_t;

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
fn GetInsertBase(mut inscode: uint16_t) -> uint32_t { unsafe {
    return kBrotliInsBase[inscode as usize];
} }

#[inline(always)]
fn GetCopyBase(mut copycode: uint16_t) -> uint32_t { unsafe {
    return kBrotliCopyBase[copycode as usize];
} }

#[inline(always)]
unsafe extern "C" fn CommandDistanceContext(mut self_0: *const Command) -> uint32_t {
    let self_0_view: &Command = unsafe { &*self_0 };
    let mut r: uint32_t =
        (self_0_view.cmd_prefix_ as c_int >> 6 as c_int) as uint32_t;
    let mut c: uint32_t =
        (self_0_view.cmd_prefix_ as c_int & 7 as c_int) as uint32_t;
    if (r == 0 as uint32_t || r == 2 as uint32_t || r == 4 as uint32_t || r == 7 as uint32_t)
        && c <= 2 as uint32_t
    {
        return c;
    }
    return 3 as uint32_t;
}
#[inline(always)]
unsafe extern "C" fn CommandCopyLen(mut self_0: *const Command) -> uint32_t {
    let self_0_view: &Command = unsafe { &*self_0 };
    return self_0_view.copy_len_ & 0x1ffffff as uint32_t;
}
#[inline(always)]
unsafe fn CommandCopyLenCode(mut self_0: *const Command) -> uint32_t {
    let self_0_view: &Command = unsafe { &*self_0 };
    let mut modifier: uint32_t = self_0_view.copy_len_ >> 25 as c_int;
    let mut delta: int32_t = (modifier | (modifier & 0x40 as uint32_t) << 1 as c_int)
        as uint8_t as int8_t as int32_t;
    return ((self_0_view.copy_len_ & 0x1ffffff as uint32_t) as int32_t + delta) as uint32_t;
}

#[inline(always)]
unsafe extern "C" fn SortHuffmanTreeItems(
    mut items: *mut HuffmanTree,
    n: size_t,
    mut comparator: HuffmanTreeComparator,
) {
    if n < 13 as size_t {
        let mut i: size_t = 0;
        i = 1 as size_t;
        while i < n {
            let mut tmp: HuffmanTree = *items.offset(i as isize);
            let mut k: size_t = i;
            let mut j: size_t = i.wrapping_sub(1 as size_t);
            while comparator.expect("non-null function pointer")(
                &raw mut tmp,
                items.offset(j as isize) as *mut HuffmanTree,
            ) != 0
            {
                *items.offset(k as isize) = *items.offset(j as isize);
                k = j;
                let fresh0 = j;
                j = j.wrapping_sub(1);
                if fresh0 == 0 {
                    break;
                }
            }
            *items.offset(k as isize) = tmp;
            i = i.wrapping_add(1);
        }
        return;
    } else {
        let mut g: c_int = if n < 57 as size_t {
            2 as c_int
        } else {
            0 as c_int
        };
        while g < 6 as c_int {
            let mut gap: size_t = kBrotliShellGaps[g as usize];
            let mut i_0: size_t = 0;
            i_0 = gap;
            while i_0 < n {
                let mut j_0: size_t = i_0;
                let mut tmp_0: HuffmanTree = *items.offset(i_0 as isize);
                while j_0 >= gap
                    && comparator.expect("non-null function pointer")(
                        &raw mut tmp_0,
                        items.offset(j_0.wrapping_sub(gap) as isize) as *mut HuffmanTree,
                    ) != 0
                {
                    *items.offset(j_0 as isize) = *items.offset(j_0.wrapping_sub(gap) as isize);
                    j_0 = (j_0 as c_ulong).wrapping_sub(gap as c_ulong)
                        as size_t as size_t;
                }
                *items.offset(j_0 as isize) = tmp_0;
                i_0 = i_0.wrapping_add(1);
            }
            g += 1;
        }
    };
}

pub const MAX_SIMPLE_DISTANCE_ALPHABET_SIZE: c_uint =
    ((BROTLI_NUM_DISTANCE_SHORT_CODES + 0 as c_int) as c_uint)
        .wrapping_add(
            (62 as c_uint) << 0 as c_int + 1 as c_int,
        );
#[inline(always)]
fn BlockLengthPrefixCode(mut len: uint32_t) -> uint32_t { unsafe {
    let mut code: uint32_t = (if len >= 177 as uint32_t {
        if len >= 753 as uint32_t {
            20 as c_int
        } else {
            14 as c_int
        }
    } else if len >= 41 as uint32_t {
        7 as c_int
    } else {
        0 as c_int
    }) as uint32_t;
    while code < (BROTLI_NUM_BLOCK_LEN_SYMBOLS - 1 as c_int) as uint32_t
        && len
            >= _kBrotliPrefixCodeRanges[code.wrapping_add(1 as uint32_t) as usize].offset
                as uint32_t
    {
        code = code.wrapping_add(1);
    }
    return code;
} }
#[inline(always)]
unsafe fn GetBlockLengthPrefixCode(
    mut len: uint32_t,
    mut code: *mut size_t,
    mut n_extra: *mut uint32_t,
    mut extra: *mut uint32_t,
) {
    *code = BlockLengthPrefixCode(len) as size_t;
    *n_extra = _kBrotliPrefixCodeRanges[*code as usize].nbits as uint32_t;
    *extra = len.wrapping_sub(_kBrotliPrefixCodeRanges[*code as usize].offset as uint32_t);
}
unsafe fn InitBlockTypeCodeCalculator(mut self_0: *mut BlockTypeCodeCalculator) {
    (*self_0).last_type = 1 as size_t;
    (*self_0).second_last_type = 0 as size_t;
}
#[inline(always)]
unsafe fn NextBlockTypeCode(
    mut calculator: *mut BlockTypeCodeCalculator,
    mut type_0: uint8_t,
) -> size_t {
    let mut type_code: size_t =
        (if type_0 as size_t == (*calculator).last_type.wrapping_add(1 as size_t) {
            1 as c_uint
        } else if type_0 as size_t == (*calculator).second_last_type {
            0 as c_uint
        } else {
            (type_0 as c_uint).wrapping_add(2 as c_uint)
        }) as size_t;
    (*calculator).second_last_type = (*calculator).last_type;
    (*calculator).last_type = type_0 as size_t;
    return type_code;
}
unsafe fn BrotliEncodeMlen(
    mut length: size_t,
    mut bits: *mut uint64_t,
    mut numbits: *mut size_t,
    mut nibblesbits: *mut uint64_t,
) {
    let bits_view: &mut uint64_t = unsafe { &mut *bits };
    let mut lg: size_t = (if length == 1 as size_t {
        1 as uint32_t
    } else {
        Log2FloorNonZero(length.wrapping_sub(1 as size_t) as uint32_t as size_t)
            .wrapping_add(1 as uint32_t)
    }) as size_t;
    let mut mnibbles: size_t = (if lg < 16 as size_t {
        16 as size_t
    } else {
        lg.wrapping_add(3 as size_t)
    })
    .wrapping_div(4 as size_t);
    *nibblesbits = mnibbles.wrapping_sub(4 as size_t) as uint64_t;
    *numbits = mnibbles.wrapping_mul(4 as size_t);
    *bits_view = length.wrapping_sub(1 as size_t) as uint64_t;
}
#[inline(always)]
unsafe fn StoreCommandExtra(
    mut cmd: *const Command,
    mut storage_ix: *mut size_t,
    mut storage: *mut uint8_t,
) {
    let mut copylen_code: uint32_t = CommandCopyLenCode(cmd);
    let mut inscode: uint16_t = GetInsertLengthCode((*cmd).insert_len_ as size_t);
    let mut copycode: uint16_t = GetCopyLengthCode(copylen_code as size_t);
    let mut insnumextra: uint32_t = GetInsertExtra(inscode);
    let mut insextraval: uint64_t =
        (*cmd).insert_len_.wrapping_sub(GetInsertBase(inscode)) as uint64_t;
    let mut copyextraval: uint64_t = copylen_code.wrapping_sub(GetCopyBase(copycode)) as uint64_t;
    let mut bits: uint64_t = copyextraval << insnumextra | insextraval;
    BrotliWriteBits(
        insnumextra.wrapping_add(GetCopyExtra(copycode)) as size_t,
        bits,
        storage_ix,
        storage,
    );
}
unsafe fn StoreVarLenUint8(
    mut n: size_t,
    mut storage_ix: *mut size_t,
    mut storage: *mut uint8_t,
) {
    if n == 0 as size_t {
        BrotliWriteBits(1 as size_t, 0 as uint64_t, storage_ix, storage);
    } else {
        let mut nbits: size_t = Log2FloorNonZero(n) as size_t;
        BrotliWriteBits(1 as size_t, 1 as uint64_t, storage_ix, storage);
        BrotliWriteBits(3 as size_t, nbits as uint64_t, storage_ix, storage);
        BrotliWriteBits(
            nbits,
            (n as uint64_t).wrapping_sub((1 as c_int as uint64_t) << nbits),
            storage_ix,
            storage,
        );
    };
}
unsafe fn StoreCompressedMetaBlockHeader(
    mut is_final_block: c_int,
    mut length: size_t,
    mut storage_ix: *mut size_t,
    mut storage: *mut uint8_t,
) {
    let mut lenbits: uint64_t = 0;
    let mut nlenbits: size_t = 0;
    let mut nibblesbits: uint64_t = 0;
    BrotliWriteBits(1 as size_t, is_final_block as uint64_t, storage_ix, storage);
    if is_final_block != 0 {
        BrotliWriteBits(1 as size_t, 0 as uint64_t, storage_ix, storage);
    }
    BrotliEncodeMlen(
        length,
        &raw mut lenbits,
        &raw mut nlenbits,
        &raw mut nibblesbits,
    );
    BrotliWriteBits(2 as size_t, nibblesbits, storage_ix, storage);
    BrotliWriteBits(nlenbits, lenbits, storage_ix, storage);
    if is_final_block == 0 {
        BrotliWriteBits(1 as size_t, 0 as uint64_t, storage_ix, storage);
    }
}
unsafe fn BrotliStoreUncompressedMetaBlockHeader(
    mut length: size_t,
    mut storage_ix: *mut size_t,
    mut storage: *mut uint8_t,
) {
    let mut lenbits: uint64_t = 0;
    let mut nlenbits: size_t = 0;
    let mut nibblesbits: uint64_t = 0;
    BrotliWriteBits(1 as size_t, 0 as uint64_t, storage_ix, storage);
    BrotliEncodeMlen(
        length,
        &raw mut lenbits,
        &raw mut nlenbits,
        &raw mut nibblesbits,
    );
    BrotliWriteBits(2 as size_t, nibblesbits, storage_ix, storage);
    BrotliWriteBits(nlenbits, lenbits, storage_ix, storage);
    BrotliWriteBits(1 as size_t, 1 as uint64_t, storage_ix, storage);
}
unsafe fn BrotliStoreHuffmanTreeOfHuffmanTreeToBitMask(
    num_codes: c_int,
    mut code_length_bitdepth: *const uint8_t,
    mut storage_ix: *mut size_t,
    mut storage: *mut uint8_t,
) {
    static mut kStorageOrder: [uint8_t; 18] = [
        1 as c_int as uint8_t,
        2 as c_int as uint8_t,
        3 as c_int as uint8_t,
        4 as c_int as uint8_t,
        0 as c_int as uint8_t,
        5 as c_int as uint8_t,
        17 as c_int as uint8_t,
        6 as c_int as uint8_t,
        16 as c_int as uint8_t,
        7 as c_int as uint8_t,
        8 as c_int as uint8_t,
        9 as c_int as uint8_t,
        10 as c_int as uint8_t,
        11 as c_int as uint8_t,
        12 as c_int as uint8_t,
        13 as c_int as uint8_t,
        14 as c_int as uint8_t,
        15 as c_int as uint8_t,
    ];
    static mut kHuffmanBitLengthHuffmanCodeSymbols: [uint8_t; 6] = [
        0 as c_int as uint8_t,
        7 as c_int as uint8_t,
        3 as c_int as uint8_t,
        2 as c_int as uint8_t,
        1 as c_int as uint8_t,
        15 as c_int as uint8_t,
    ];
    static mut kHuffmanBitLengthHuffmanCodeBitLengths: [uint8_t; 6] = [
        2 as c_int as uint8_t,
        4 as c_int as uint8_t,
        3 as c_int as uint8_t,
        2 as c_int as uint8_t,
        2 as c_int as uint8_t,
        4 as c_int as uint8_t,
    ];
    let mut skip_some: size_t = 0 as size_t;
    let mut codes_to_store: size_t = BROTLI_CODE_LENGTH_CODES as size_t;
    if num_codes > 1 as c_int {
        while codes_to_store > 0 as size_t {
            if *code_length_bitdepth
                .offset(kStorageOrder[codes_to_store.wrapping_sub(1 as size_t) as usize] as isize)
                as c_int
                != 0 as c_int
            {
                break;
            }
            codes_to_store = codes_to_store.wrapping_sub(1);
        }
    }
    if *code_length_bitdepth.offset(kStorageOrder[0 as c_int as usize] as isize)
        as c_int
        == 0 as c_int
        && *code_length_bitdepth.offset(kStorageOrder[1 as c_int as usize] as isize)
            as c_int
            == 0 as c_int
    {
        skip_some = 2 as size_t;
        if *code_length_bitdepth.offset(kStorageOrder[2 as c_int as usize] as isize)
            as c_int
            == 0 as c_int
        {
            skip_some = 3 as size_t;
        }
    }
    BrotliWriteBits(2 as size_t, skip_some as uint64_t, storage_ix, storage);
    let mut i: size_t = 0;
    i = skip_some;
    while i < codes_to_store {
        let mut l: size_t =
            *code_length_bitdepth.offset(kStorageOrder[i as usize] as isize) as size_t;
        BrotliWriteBits(
            kHuffmanBitLengthHuffmanCodeBitLengths[l as usize] as size_t,
            kHuffmanBitLengthHuffmanCodeSymbols[l as usize] as uint64_t,
            storage_ix,
            storage,
        );
        i = i.wrapping_add(1);
    }
}
unsafe fn BrotliStoreHuffmanTreeToBitMask(
    huffman_tree_size: size_t,
    mut huffman_tree: *const uint8_t,
    mut huffman_tree_extra_bits: *const uint8_t,
    mut code_length_bitdepth: *const uint8_t,
    mut code_length_bitdepth_symbols: *const uint16_t,
    mut storage_ix: *mut size_t,
    mut storage: *mut uint8_t,
) {
    let huffman_tree_view: &[uint8_t] = unsafe { core::slice::from_raw_parts(huffman_tree, (huffman_tree_size) as usize) };
    let mut i: size_t = 0;
    i = 0 as size_t;
    while i < huffman_tree_size {
        let mut ix: size_t = huffman_tree_view[(i) as usize] as size_t;
        BrotliWriteBits(
            *code_length_bitdepth.offset(ix as isize) as size_t,
            *code_length_bitdepth_symbols.offset(ix as isize) as uint64_t,
            storage_ix,
            storage,
        );
        match ix {
            16 => {
                BrotliWriteBits(
                    2 as size_t,
                    *huffman_tree_extra_bits.offset(i as isize) as uint64_t,
                    storage_ix,
                    storage,
                );
            }
            17 => {
                BrotliWriteBits(
                    3 as size_t,
                    *huffman_tree_extra_bits.offset(i as isize) as uint64_t,
                    storage_ix,
                    storage,
                );
            }
            _ => {}
        }
        i = i.wrapping_add(1);
    }
}
unsafe fn StoreSimpleHuffmanTree(
    mut depths: *const uint8_t,
    mut symbols: *mut size_t,
    mut num_symbols: size_t,
    mut max_bits: size_t,
    mut storage_ix: *mut size_t,
    mut storage: *mut uint8_t,
) {
    BrotliWriteBits(2 as size_t, 1 as uint64_t, storage_ix, storage);
    BrotliWriteBits(
        2 as size_t,
        (num_symbols as uint64_t).wrapping_sub(1 as uint64_t),
        storage_ix,
        storage,
    );
    let mut i: size_t = 0;
    i = 0 as size_t;
    while i < num_symbols {
        let mut j: size_t = 0;
        j = i.wrapping_add(1 as size_t);
        while j < num_symbols {
            if (*depths.offset(*symbols.offset(j as isize) as isize) as c_int)
                < *depths.offset(*symbols.offset(i as isize) as isize) as c_int
            {
                let mut __brotli_swap_tmp: size_t = *symbols.offset(j as isize);
                *symbols.offset(j as isize) = *symbols.offset(i as isize);
                *symbols.offset(i as isize) = __brotli_swap_tmp;
            }
            j = j.wrapping_add(1);
        }
        i = i.wrapping_add(1);
    }
    if num_symbols == 2 as size_t {
        BrotliWriteBits(
            max_bits,
            *symbols.offset(0 as c_int as isize) as uint64_t,
            storage_ix,
            storage,
        );
        BrotliWriteBits(
            max_bits,
            *symbols.offset(1 as c_int as isize) as uint64_t,
            storage_ix,
            storage,
        );
    } else if num_symbols == 3 as size_t {
        BrotliWriteBits(
            max_bits,
            *symbols.offset(0 as c_int as isize) as uint64_t,
            storage_ix,
            storage,
        );
        BrotliWriteBits(
            max_bits,
            *symbols.offset(1 as c_int as isize) as uint64_t,
            storage_ix,
            storage,
        );
        BrotliWriteBits(
            max_bits,
            *symbols.offset(2 as c_int as isize) as uint64_t,
            storage_ix,
            storage,
        );
    } else {
        BrotliWriteBits(
            max_bits,
            *symbols.offset(0 as c_int as isize) as uint64_t,
            storage_ix,
            storage,
        );
        BrotliWriteBits(
            max_bits,
            *symbols.offset(1 as c_int as isize) as uint64_t,
            storage_ix,
            storage,
        );
        BrotliWriteBits(
            max_bits,
            *symbols.offset(2 as c_int as isize) as uint64_t,
            storage_ix,
            storage,
        );
        BrotliWriteBits(
            max_bits,
            *symbols.offset(3 as c_int as isize) as uint64_t,
            storage_ix,
            storage,
        );
        BrotliWriteBits(
            1 as size_t,
            (if *depths.offset(*symbols.offset(0 as c_int as isize) as isize)
                as c_int
                == 1 as c_int
            {
                1 as c_int
            } else {
                0 as c_int
            }) as uint64_t,
            storage_ix,
            storage,
        );
    };
}
#[inline]
pub unsafe fn BrotliStoreHuffmanTree(
    mut depths: *const uint8_t,
    mut num: size_t,
    mut tree: *mut HuffmanTree,
    mut storage_ix: *mut size_t,
    mut storage: *mut uint8_t,
) {
    let mut huffman_tree: [uint8_t; 704] = [0; 704];
    let mut huffman_tree_extra_bits: [uint8_t; 704] = [0; 704];
    let mut huffman_tree_size: size_t = 0 as size_t;
    let mut code_length_bitdepth: [uint8_t; 18] = [
        0 as c_int as uint8_t,
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
    let mut code_length_bitdepth_symbols: [uint16_t; 18] = [0; 18];
    let mut huffman_tree_histogram: [uint32_t; 18] = [
        0 as c_int as uint32_t,
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
    let mut i: size_t = 0;
    let mut num_codes: c_int = 0 as c_int;
    let mut code: size_t = 0 as size_t;
    BrotliWriteHuffmanTree(
        depths,
        num,
        &raw mut huffman_tree_size,
        &raw mut huffman_tree as *mut uint8_t,
        &raw mut huffman_tree_extra_bits as *mut uint8_t,
    );
    i = 0 as size_t;
    while i < huffman_tree_size {
        huffman_tree_histogram[huffman_tree[i as usize] as usize] =
            huffman_tree_histogram[huffman_tree[i as usize] as usize].wrapping_add(1);
        i = i.wrapping_add(1);
    }
    i = 0 as size_t;
    while i < BROTLI_CODE_LENGTH_CODES as size_t {
        if huffman_tree_histogram[i as usize] != 0 {
            if num_codes == 0 as c_int {
                code = i;
                num_codes = 1 as c_int;
            } else if num_codes == 1 as c_int {
                num_codes = 2 as c_int;
                break;
            }
        }
        i = i.wrapping_add(1);
    }
    BrotliCreateHuffmanTree(
        &raw mut huffman_tree_histogram as *mut uint32_t,
        BROTLI_CODE_LENGTH_CODES as size_t,
        5 as c_int,
        tree,
        &raw mut code_length_bitdepth as *mut uint8_t,
    );
    BrotliConvertBitDepthsToSymbols(
        &raw mut code_length_bitdepth as *mut uint8_t,
        BROTLI_CODE_LENGTH_CODES as size_t,
        &raw mut code_length_bitdepth_symbols as *mut uint16_t,
    );
    BrotliStoreHuffmanTreeOfHuffmanTreeToBitMask(
        num_codes,
        &raw mut code_length_bitdepth as *mut uint8_t,
        storage_ix,
        storage,
    );
    if num_codes == 1 as c_int {
        code_length_bitdepth[code as usize] = 0 as uint8_t;
    }
    BrotliStoreHuffmanTreeToBitMask(
        huffman_tree_size,
        &raw mut huffman_tree as *mut uint8_t,
        &raw mut huffman_tree_extra_bits as *mut uint8_t,
        &raw mut code_length_bitdepth as *mut uint8_t,
        &raw mut code_length_bitdepth_symbols as *mut uint16_t,
        storage_ix,
        storage,
    );
}
unsafe fn BuildAndStoreHuffmanTree(
    mut histogram: *const uint32_t,
    histogram_length: size_t,
    alphabet_size: size_t,
    mut tree: *mut HuffmanTree,
    mut depth: *mut uint8_t,
    mut bits: *mut uint16_t,
    mut storage_ix: *mut size_t,
    mut storage: *mut uint8_t,
) {
    let histogram_view: &[uint32_t] = unsafe { core::slice::from_raw_parts(histogram, (histogram_length) as usize) };
    let mut count: size_t = 0 as size_t;
    let mut s4: [size_t; 4] = [0 as c_int as size_t, 0, 0, 0];
    let mut i: size_t = 0;
    let mut max_bits: size_t = 0 as size_t;
    i = 0 as size_t;
    while i < histogram_length {
        if histogram_view[(i) as usize] != 0 {
            if count < 4 as size_t {
                s4[count as usize] = i;
            } else if count > 4 as size_t {
                break;
            }
            count = count.wrapping_add(1);
        }
        i = i.wrapping_add(1);
    }
    let mut max_bits_counter: size_t = alphabet_size.wrapping_sub(1 as size_t);
    while max_bits_counter != 0 {
        max_bits_counter >>= 1 as c_int;
        max_bits = max_bits.wrapping_add(1);
    }
    if count <= 1 as size_t {
        BrotliWriteBits(4 as size_t, 1 as uint64_t, storage_ix, storage);
        BrotliWriteBits(
            max_bits,
            s4[0 as c_int as usize] as u64,
            storage_ix,
            storage,
        );
        *depth.offset(s4[0 as c_int as usize] as isize) = 0 as uint8_t;
        *bits.offset(s4[0 as c_int as usize] as isize) = 0 as uint16_t;
        return;
    }
    memset(
        depth as *mut c_void,
        0 as c_int,
        histogram_length.wrapping_mul(::core::mem::size_of::<uint8_t>() as size_t),
    );
    BrotliCreateHuffmanTree(
        histogram,
        histogram_length,
        15 as c_int,
        tree,
        depth,
    );
    BrotliConvertBitDepthsToSymbols(depth, histogram_length, bits);
    if count <= 4 as size_t {
        StoreSimpleHuffmanTree(
            depth,
            &raw mut s4 as *mut size_t,
            count,
            max_bits,
            storage_ix,
            storage,
        );
    } else {
        BrotliStoreHuffmanTree(depth, histogram_length, tree, storage_ix, storage);
    };
}
#[inline(always)]
unsafe extern "C" fn SortHuffmanTree(
    mut v0: *const HuffmanTree,
    mut v1: *const HuffmanTree,
) -> c_int {
    let v0_view: &HuffmanTree = unsafe { &*v0 };
    return if v0_view.total_count_ < (*v1).total_count_ {
        BROTLI_TRUE
    } else {
        BROTLI_FALSE
    };
}
#[inline]
pub unsafe fn BrotliBuildAndStoreHuffmanTreeFast(
    mut tree: *mut HuffmanTree,
    mut histogram: *const uint32_t,
    histogram_total: size_t,
    max_bits: size_t,
    mut depth: *mut uint8_t,
    mut bits: *mut uint16_t,
    mut storage_ix: *mut size_t,
    mut storage: *mut uint8_t,
) {
    let mut count: size_t = 0 as size_t;
    let mut symbols: [size_t; 4] = [0 as c_int as size_t, 0, 0, 0];
    let mut length: size_t = 0 as size_t;
    let mut total: size_t = histogram_total;
    while total != 0 as size_t {
        if *histogram.offset(length as isize) != 0 {
            if count < 4 as size_t {
                symbols[count as usize] = length;
            }
            count = count.wrapping_add(1);
            total = (total as c_ulong)
                .wrapping_sub(*histogram.offset(length as isize) as c_ulong)
                as size_t as size_t;
        }
        length = length.wrapping_add(1);
    }
    if count <= 1 as size_t {
        BrotliWriteBits(4 as size_t, 1 as uint64_t, storage_ix, storage);
        BrotliWriteBits(
            max_bits,
            symbols[0 as c_int as usize] as u64,
            storage_ix,
            storage,
        );
        *depth.offset(symbols[0 as c_int as usize] as isize) = 0 as uint8_t;
        *bits.offset(symbols[0 as c_int as usize] as isize) = 0 as uint16_t;
        return;
    }
    memset(
        depth as *mut c_void,
        0 as c_int,
        length.wrapping_mul(::core::mem::size_of::<uint8_t>() as size_t),
    );
    let mut count_limit: uint32_t = 0;
    count_limit = 1 as uint32_t;
    loop {
        let mut node: *mut HuffmanTree = tree;
        let mut l: size_t = 0;
        l = length;
        while l != 0 as size_t {
            l = l.wrapping_sub(1);
            if *histogram.offset(l as isize) != 0 {
                if (*histogram.offset(l as isize) >= count_limit) as c_int
                    as c_long
                    != 0
                {
                    InitHuffmanTree(
                        node,
                        *histogram.offset(l as isize),
                        -(1 as c_int) as int16_t,
                        l as int16_t,
                    );
                } else {
                    InitHuffmanTree(
                        node,
                        count_limit,
                        -(1 as c_int) as int16_t,
                        l as int16_t,
                    );
                }
                node = node.offset(1);
            }
        }
        let n: c_int =
            node.offset_from(tree) as c_long as c_int;
        let mut sentinel: HuffmanTree = HuffmanTree {
            total_count_: 0,
            index_left_: 0,
            index_right_or_value_: 0,
        };
        let mut i: c_int = 0 as c_int;
        let mut j: c_int = n + 1 as c_int;
        let mut k: c_int = 0;
        SortHuffmanTreeItems(
            tree,
            n as size_t,
            Some(
                SortHuffmanTree
                    as unsafe extern "C" fn(
                        *const HuffmanTree,
                        *const HuffmanTree,
                    ) -> c_int,
            ),
        );
        InitHuffmanTree(
            &raw mut sentinel,
            BROTLI_UINT32_MAX,
            -(1 as c_int) as int16_t,
            -(1 as c_int) as int16_t,
        );
        let fresh1 = node;
        node = node.offset(1);
        *fresh1 = sentinel;
        let fresh2 = node;
        node = node.offset(1);
        *fresh2 = sentinel;
        k = n - 1 as c_int;
        while k > 0 as c_int {
            let mut left: c_int = 0;
            let mut right: c_int = 0;
            if (*tree.offset(i as isize)).total_count_ <= (*tree.offset(j as isize)).total_count_ {
                left = i;
                i += 1;
            } else {
                left = j;
                j += 1;
            }
            if (*tree.offset(i as isize)).total_count_ <= (*tree.offset(j as isize)).total_count_ {
                right = i;
                i += 1;
            } else {
                right = j;
                j += 1;
            }
            (*node.offset(-(1 as c_int) as isize)).total_count_ = (*tree
                .offset(left as isize))
            .total_count_
            .wrapping_add((*tree.offset(right as isize)).total_count_);
            (*node.offset(-(1 as c_int) as isize)).index_left_ = left as int16_t;
            (*node.offset(-(1 as c_int) as isize)).index_right_or_value_ =
                right as int16_t;
            let fresh3 = node;
            node = node.offset(1);
            *fresh3 = sentinel;
            k -= 1;
        }
        if BrotliSetDepth(
            2 as c_int * n - 1 as c_int,
            tree,
            depth,
            14 as c_int,
        ) != 0
        {
            break;
        }
        count_limit = (count_limit as c_uint).wrapping_mul(2 as c_uint)
            as uint32_t as uint32_t;
    }
    BrotliConvertBitDepthsToSymbols(depth, length, bits);
    if count <= 4 as size_t {
        let mut i_0: size_t = 0;
        BrotliWriteBits(2 as size_t, 1 as uint64_t, storage_ix, storage);
        BrotliWriteBits(
            2 as size_t,
            (count as uint64_t).wrapping_sub(1 as uint64_t),
            storage_ix,
            storage,
        );
        i_0 = 0 as size_t;
        while i_0 < count {
            let mut j_0: size_t = 0;
            j_0 = i_0.wrapping_add(1 as size_t);
            while j_0 < count {
                if (*depth.offset(symbols[j_0 as usize] as isize) as c_int)
                    < *depth.offset(symbols[i_0 as usize] as isize) as c_int
                {
                    let mut __brotli_swap_tmp: size_t = symbols[j_0 as usize];
                    symbols[j_0 as usize] = symbols[i_0 as usize];
                    symbols[i_0 as usize] = __brotli_swap_tmp;
                }
                j_0 = j_0.wrapping_add(1);
            }
            i_0 = i_0.wrapping_add(1);
        }
        if count == 2 as size_t {
            BrotliWriteBits(
                max_bits,
                symbols[0 as c_int as usize] as u64,
                storage_ix,
                storage,
            );
            BrotliWriteBits(
                max_bits,
                symbols[1 as c_int as usize] as u64,
                storage_ix,
                storage,
            );
        } else if count == 3 as size_t {
            BrotliWriteBits(
                max_bits,
                symbols[0 as c_int as usize] as u64,
                storage_ix,
                storage,
            );
            BrotliWriteBits(
                max_bits,
                symbols[1 as c_int as usize] as u64,
                storage_ix,
                storage,
            );
            BrotliWriteBits(
                max_bits,
                symbols[2 as c_int as usize] as u64,
                storage_ix,
                storage,
            );
        } else {
            BrotliWriteBits(
                max_bits,
                symbols[0 as c_int as usize] as u64,
                storage_ix,
                storage,
            );
            BrotliWriteBits(
                max_bits,
                symbols[1 as c_int as usize] as u64,
                storage_ix,
                storage,
            );
            BrotliWriteBits(
                max_bits,
                symbols[2 as c_int as usize] as u64,
                storage_ix,
                storage,
            );
            BrotliWriteBits(
                max_bits,
                symbols[3 as c_int as usize] as u64,
                storage_ix,
                storage,
            );
            BrotliWriteBits(
                1 as size_t,
                (if *depth.offset(symbols[0 as c_int as usize] as isize)
                    as c_int
                    == 1 as c_int
                {
                    1 as c_int
                } else {
                    0 as c_int
                }) as uint64_t,
                storage_ix,
                storage,
            );
        }
    } else {
        let mut previous_value: uint8_t = 8 as uint8_t;
        let mut i_1: size_t = 0;
        StoreStaticCodeLengthCode(storage_ix, storage);
        i_1 = 0 as size_t;
        while i_1 < length {
            let value: uint8_t = *depth.offset(i_1 as isize);
            let mut reps: size_t = 1 as size_t;
            let mut k_0: size_t = 0;
            k_0 = i_1.wrapping_add(1 as size_t);
            while k_0 < length
                && *depth.offset(k_0 as isize) as c_int == value as c_int
            {
                reps = reps.wrapping_add(1);
                k_0 = k_0.wrapping_add(1);
            }
            i_1 = (i_1 as c_ulong).wrapping_add(reps as c_ulong) as size_t
                as size_t;
            if value as c_int == 0 as c_int {
                BrotliWriteBits(
                    kZeroRepsDepth[reps as usize] as size_t,
                    kZeroRepsBits[reps as usize],
                    storage_ix,
                    storage,
                );
            } else {
                if previous_value as c_int != value as c_int {
                    BrotliWriteBits(
                        kCodeLengthDepth[value as usize] as size_t,
                        kCodeLengthBits[value as usize] as uint64_t,
                        storage_ix,
                        storage,
                    );
                    reps = reps.wrapping_sub(1);
                }
                if reps < 3 as size_t {
                    while reps != 0 as size_t {
                        reps = reps.wrapping_sub(1);
                        BrotliWriteBits(
                            kCodeLengthDepth[value as usize] as size_t,
                            kCodeLengthBits[value as usize] as uint64_t,
                            storage_ix,
                            storage,
                        );
                    }
                } else {
                    reps = (reps as c_ulong).wrapping_sub(3 as c_ulong)
                        as size_t as size_t;
                    BrotliWriteBits(
                        kNonZeroRepsDepth[reps as usize] as size_t,
                        kNonZeroRepsBits[reps as usize],
                        storage_ix,
                        storage,
                    );
                }
                previous_value = value;
            }
        }
    };
}
unsafe fn IndexOf(
    mut v: *const uint8_t,
    mut v_size: size_t,
    mut value: uint8_t,
) -> size_t {
    let v_view: &[uint8_t] = unsafe { core::slice::from_raw_parts(v, (v_size) as usize) };
    let mut i: size_t = 0 as size_t;
    while i < v_size {
        if v_view[(i) as usize] as c_int == value as c_int {
            return i;
        }
        i = i.wrapping_add(1);
    }
    return i;
}
unsafe fn MoveToFront(mut v: *mut uint8_t, mut index: size_t) {
    let mut value: uint8_t = *v.offset(index as isize);
    let mut i: size_t = 0;
    i = index;
    while i != 0 as size_t {
        *v.offset(i as isize) = *v.offset(i.wrapping_sub(1 as size_t) as isize);
        i = i.wrapping_sub(1);
    }
    *v.offset(0 as c_int as isize) = value;
}
unsafe fn MoveToFrontTransform(
    mut v_in: *const uint32_t,
    v_size: size_t,
    mut v_out: *mut uint32_t,
) {
    let v_out_view: &mut [uint32_t] = unsafe { core::slice::from_raw_parts_mut(v_out, (v_size) as usize) };
    let mut i: size_t = 0;
    let mut mtf: [uint8_t; 256] = [0; 256];
    let mut max_value: uint32_t = 0;
    if v_size == 0 as size_t {
        return;
    }
    max_value = *v_in.offset(0 as c_int as isize);
    i = 1 as size_t;
    while i < v_size {
        if *v_in.offset(i as isize) > max_value {
            max_value = *v_in.offset(i as isize);
        }
        i = i.wrapping_add(1);
    }
    i = 0 as size_t;
    while i <= max_value as size_t {
        mtf[i as usize] = i as uint8_t;
        i = i.wrapping_add(1);
    }
    let mut mtf_size: size_t = max_value.wrapping_add(1 as uint32_t) as size_t;
    i = 0 as size_t;
    while i < v_size {
        let mut index: size_t = IndexOf(
            &raw mut mtf as *mut uint8_t,
            mtf_size,
            *v_in.offset(i as isize) as uint8_t,
        );
        v_out_view[(i) as usize] = index as uint32_t;
        MoveToFront(&raw mut mtf as *mut uint8_t, index);
        i = i.wrapping_add(1);
    }
}
unsafe fn RunLengthCodeZeros(
    in_size: size_t,
    mut v: *mut uint32_t,
    mut out_size: *mut size_t,
    mut max_run_length_prefix: *mut uint32_t,
) {
    let out_size_view: &mut size_t = unsafe { &mut *out_size };
    let mut max_reps: uint32_t = 0 as uint32_t;
    let mut i: size_t = 0;
    let mut max_prefix: uint32_t = 0;
    i = 0 as size_t;
    while i < in_size {
        let mut reps: uint32_t = 0 as uint32_t;
        while i < in_size && *v.offset(i as isize) != 0 as uint32_t {
            i = i.wrapping_add(1);
        }
        while i < in_size && *v.offset(i as isize) == 0 as uint32_t {
            reps = reps.wrapping_add(1);
            i = i.wrapping_add(1);
        }
        max_reps = brotli_max_uint32_t(reps, max_reps);
    }
    max_prefix = if max_reps > 0 as uint32_t {
        Log2FloorNonZero(max_reps as size_t)
    } else {
        0 as uint32_t
    };
    max_prefix = brotli_min_uint32_t(max_prefix, *max_run_length_prefix);
    *max_run_length_prefix = max_prefix;
    *out_size_view = 0 as size_t;
    i = 0 as size_t;
    while i < in_size {
        if *v.offset(i as isize) != 0 as uint32_t {
            *v.offset(*out_size_view as isize) =
                (*v.offset(i as isize)).wrapping_add(*max_run_length_prefix);
            i = i.wrapping_add(1);
            *out_size_view = out_size_view.wrapping_add(1);
        } else {
            let mut reps_0: uint32_t = 1 as uint32_t;
            let mut k: size_t = 0;
            k = i.wrapping_add(1 as size_t);
            while k < in_size && *v.offset(k as isize) == 0 as uint32_t {
                reps_0 = reps_0.wrapping_add(1);
                k = k.wrapping_add(1);
            }
            i = (i as c_ulong).wrapping_add(reps_0 as c_ulong) as size_t
                as size_t;
            while reps_0 != 0 as uint32_t {
                if reps_0 < (2 as uint32_t) << max_prefix {
                    let mut run_length_prefix: uint32_t = Log2FloorNonZero(reps_0 as size_t);
                    let extra_bits: uint32_t =
                        reps_0.wrapping_sub((1 as uint32_t) << run_length_prefix);
                    *v.offset(*out_size_view as isize) =
                        run_length_prefix.wrapping_add(extra_bits << 9 as c_int);
                    *out_size_view = out_size_view.wrapping_add(1);
                    break;
                } else {
                    let extra_bits_0: uint32_t =
                        ((1 as uint32_t) << max_prefix).wrapping_sub(1 as uint32_t);
                    *v.offset(*out_size_view as isize) =
                        max_prefix.wrapping_add(extra_bits_0 << 9 as c_int);
                    reps_0 = (reps_0 as c_uint).wrapping_sub(
                        ((2 as c_uint) << max_prefix)
                            .wrapping_sub(1 as c_uint),
                    ) as uint32_t as uint32_t;
                    *out_size_view = out_size_view.wrapping_add(1);
                }
            }
        }
    }
}
pub const SYMBOL_BITS: c_int = 9 as c_int;
unsafe fn EncodeContextMap(
    mut m: *mut MemoryManager,
    mut arena: *mut EncodeContextMapArena,
    mut context_map: *const uint32_t,
    mut context_map_size: size_t,
    mut num_clusters: size_t,
    mut tree: *mut HuffmanTree,
    mut storage_ix: *mut size_t,
    mut storage: *mut uint8_t,
) {
    let arena_view: &mut EncodeContextMapArena = unsafe { &mut *arena };
    let mut i: size_t = 0;
    let mut rle_symbols: *mut uint32_t = ::core::ptr::null_mut::<uint32_t>();
    let mut max_run_length_prefix: uint32_t = 6 as uint32_t;
    let mut num_rle_symbols: size_t = 0 as size_t;
    let histogram: *mut uint32_t = &raw mut arena_view.histogram as *mut uint32_t;
    let depths: *mut uint8_t = &raw mut arena_view.depths as *mut uint8_t;
    let bits: *mut uint16_t = &raw mut arena_view.bits as *mut uint16_t;
    StoreVarLenUint8(num_clusters.wrapping_sub(1 as size_t), storage_ix, storage);
    if num_clusters == 1 as size_t {
        return;
    }
    rle_symbols = if context_map_size > 0 as size_t {
        BrotliAllocate(
            m,
            context_map_size.wrapping_mul(::core::mem::size_of::<uint32_t>() as size_t),
        ) as *mut uint32_t
    } else {
        ::core::ptr::null_mut::<uint32_t>()
    };
    if 0 as c_int != 0 || 0 as c_int != 0 {
        return;
    }
    MoveToFrontTransform(context_map, context_map_size, rle_symbols);
    RunLengthCodeZeros(
        context_map_size,
        rle_symbols,
        &raw mut num_rle_symbols,
        &raw mut max_run_length_prefix,
    );
    memset(
        histogram as *mut c_void,
        0 as c_int,
        ::core::mem::size_of::<[uint32_t; 272]>() as size_t,
    );
    i = 0 as size_t;
    while i < num_rle_symbols {
        let ref mut fresh4 =
            *histogram.offset((*rle_symbols.offset(i as isize) & kSymbolMask) as isize);
        *fresh4 = (*fresh4).wrapping_add(1);
        i = i.wrapping_add(1);
    }
    let mut use_rle: c_int = if max_run_length_prefix > 0 as uint32_t {
        BROTLI_TRUE
    } else {
        BROTLI_FALSE
    };
    BrotliWriteBits(1 as size_t, use_rle as uint64_t, storage_ix, storage);
    if use_rle != 0 {
        BrotliWriteBits(
            4 as size_t,
            max_run_length_prefix.wrapping_sub(1 as uint32_t) as uint64_t,
            storage_ix,
            storage,
        );
    }
    BuildAndStoreHuffmanTree(
        histogram,
        num_clusters.wrapping_add(max_run_length_prefix as size_t),
        num_clusters.wrapping_add(max_run_length_prefix as size_t),
        tree,
        depths,
        bits,
        storage_ix,
        storage,
    );
    i = 0 as size_t;
    while i < num_rle_symbols {
        let rle_symbol: uint32_t = *rle_symbols.offset(i as isize) & kSymbolMask;
        let extra_bits_val: uint32_t = *rle_symbols.offset(i as isize) >> SYMBOL_BITS;
        BrotliWriteBits(
            *depths.offset(rle_symbol as isize) as size_t,
            *bits.offset(rle_symbol as isize) as uint64_t,
            storage_ix,
            storage,
        );
        if rle_symbol > 0 as uint32_t && rle_symbol <= max_run_length_prefix {
            BrotliWriteBits(
                rle_symbol as size_t,
                extra_bits_val as uint64_t,
                storage_ix,
                storage,
            );
        }
        i = i.wrapping_add(1);
    }
    BrotliWriteBits(1 as size_t, 1 as uint64_t, storage_ix, storage);
    BrotliFree(m, rle_symbols as *mut c_void);
    rle_symbols = ::core::ptr::null_mut::<uint32_t>();
}
#[inline(always)]
unsafe fn StoreBlockSwitch(
    mut code: *mut BlockSplitCode,
    block_len: uint32_t,
    block_type: uint8_t,
    mut is_first_block: c_int,
    mut storage_ix: *mut size_t,
    mut storage: *mut uint8_t,
) {
    let mut typecode: size_t = NextBlockTypeCode(&raw mut (*code).type_code_calculator, block_type);
    let mut lencode: size_t = 0;
    let mut len_nextra: uint32_t = 0;
    let mut len_extra: uint32_t = 0;
    if is_first_block == 0 {
        BrotliWriteBits(
            (*code).type_depths[typecode as usize] as size_t,
            (*code).type_bits[typecode as usize] as uint64_t,
            storage_ix,
            storage,
        );
    }
    GetBlockLengthPrefixCode(
        block_len,
        &raw mut lencode,
        &raw mut len_nextra,
        &raw mut len_extra,
    );
    BrotliWriteBits(
        (*code).length_depths[lencode as usize] as size_t,
        (*code).length_bits[lencode as usize] as uint64_t,
        storage_ix,
        storage,
    );
    BrotliWriteBits(
        len_nextra as size_t,
        len_extra as uint64_t,
        storage_ix,
        storage,
    );
}
unsafe fn BuildAndStoreBlockSplitCode(
    mut types: *const uint8_t,
    mut lengths: *const uint32_t,
    num_blocks: size_t,
    num_types: size_t,
    mut tree: *mut HuffmanTree,
    mut code: *mut BlockSplitCode,
    mut storage_ix: *mut size_t,
    mut storage: *mut uint8_t,
) {
    let mut type_histo: [uint32_t; 258] = [0; 258];
    let mut length_histo: [uint32_t; 26] = [0; 26];
    let mut i: size_t = 0;
    let mut type_code_calculator: BlockTypeCodeCalculator = BlockTypeCodeCalculator {
        last_type: 0,
        second_last_type: 0,
    };
    memset(
        &raw mut type_histo as *mut uint32_t as *mut c_void,
        0 as c_int,
        num_types
            .wrapping_add(2 as size_t)
            .wrapping_mul(::core::mem::size_of::<uint32_t>() as size_t),
    );
    memset(
        &raw mut length_histo as *mut uint32_t as *mut c_void,
        0 as c_int,
        ::core::mem::size_of::<[uint32_t; 26]>() as size_t,
    );
    InitBlockTypeCodeCalculator(&raw mut type_code_calculator);
    i = 0 as size_t;
    while i < num_blocks {
        let mut type_code: size_t =
            NextBlockTypeCode(&raw mut type_code_calculator, *types.offset(i as isize));
        if i != 0 as size_t {
            type_histo[type_code as usize] = type_histo[type_code as usize].wrapping_add(1);
        }
        length_histo[BlockLengthPrefixCode(*lengths.offset(i as isize)) as usize] = length_histo
            [BlockLengthPrefixCode(*lengths.offset(i as isize)) as usize]
            .wrapping_add(1);
        i = i.wrapping_add(1);
    }
    StoreVarLenUint8(num_types.wrapping_sub(1 as size_t), storage_ix, storage);
    if num_types > 1 as size_t {
        BuildAndStoreHuffmanTree(
            (&raw mut type_histo as *mut uint32_t).offset(0 as c_int as isize)
                as *mut uint32_t,
            num_types.wrapping_add(2 as size_t),
            num_types.wrapping_add(2 as size_t),
            tree,
            (&raw mut (*code).type_depths as *mut uint8_t).offset(0 as c_int as isize)
                as *mut uint8_t,
            (&raw mut (*code).type_bits as *mut uint16_t).offset(0 as c_int as isize)
                as *mut uint16_t,
            storage_ix,
            storage,
        );
        BuildAndStoreHuffmanTree(
            (&raw mut length_histo as *mut uint32_t).offset(0 as c_int as isize)
                as *mut uint32_t,
            BROTLI_NUM_BLOCK_LEN_SYMBOLS as size_t,
            BROTLI_NUM_BLOCK_LEN_SYMBOLS as size_t,
            tree,
            (&raw mut (*code).length_depths as *mut uint8_t)
                .offset(0 as c_int as isize) as *mut uint8_t,
            (&raw mut (*code).length_bits as *mut uint16_t).offset(0 as c_int as isize)
                as *mut uint16_t,
            storage_ix,
            storage,
        );
        StoreBlockSwitch(
            code,
            *lengths.offset(0 as c_int as isize),
            *types.offset(0 as c_int as isize),
            1 as c_int,
            storage_ix,
            storage,
        );
    }
}
unsafe fn StoreTrivialContextMap(
    mut arena: *mut EncodeContextMapArena,
    mut num_types: size_t,
    mut context_bits: size_t,
    mut tree: *mut HuffmanTree,
    mut storage_ix: *mut size_t,
    mut storage: *mut uint8_t,
) {
    let arena_view: &mut EncodeContextMapArena = unsafe { &mut *arena };
    StoreVarLenUint8(num_types.wrapping_sub(1 as size_t), storage_ix, storage);
    if num_types > 1 as size_t {
        let mut repeat_code: size_t = context_bits.wrapping_sub(1 as size_t);
        let mut repeat_bits: size_t = ((1 as c_uint) << repeat_code)
            .wrapping_sub(1 as c_uint) as size_t;
        let mut alphabet_size: size_t = num_types.wrapping_add(repeat_code);
        let histogram: *mut uint32_t = &raw mut arena_view.histogram as *mut uint32_t;
        let depths: *mut uint8_t = &raw mut arena_view.depths as *mut uint8_t;
        let bits: *mut uint16_t = &raw mut arena_view.bits as *mut uint16_t;
        let mut i: size_t = 0;
        memset(
            histogram as *mut c_void,
            0 as c_int,
            alphabet_size.wrapping_mul(::core::mem::size_of::<uint32_t>() as size_t),
        );
        BrotliWriteBits(1 as size_t, 1 as uint64_t, storage_ix, storage);
        BrotliWriteBits(
            4 as size_t,
            (repeat_code as uint64_t).wrapping_sub(1 as uint64_t),
            storage_ix,
            storage,
        );
        *histogram.offset(repeat_code as isize) = num_types as uint32_t;
        *histogram.offset(0 as c_int as isize) = 1 as uint32_t;
        i = context_bits;
        while i < alphabet_size {
            *histogram.offset(i as isize) = 1 as uint32_t;
            i = i.wrapping_add(1);
        }
        BuildAndStoreHuffmanTree(
            histogram,
            alphabet_size,
            alphabet_size,
            tree,
            depths,
            bits,
            storage_ix,
            storage,
        );
        i = 0 as size_t;
        while i < num_types {
            let mut code: size_t = if i == 0 as size_t {
                0 as size_t
            } else {
                i.wrapping_add(context_bits).wrapping_sub(1 as size_t)
            };
            BrotliWriteBits(
                *depths.offset(code as isize) as size_t,
                *bits.offset(code as isize) as uint64_t,
                storage_ix,
                storage,
            );
            BrotliWriteBits(
                *depths.offset(repeat_code as isize) as size_t,
                *bits.offset(repeat_code as isize) as uint64_t,
                storage_ix,
                storage,
            );
            BrotliWriteBits(repeat_code, repeat_bits as uint64_t, storage_ix, storage);
            i = i.wrapping_add(1);
        }
        BrotliWriteBits(1 as size_t, 1 as uint64_t, storage_ix, storage);
    }
}
unsafe fn InitBlockEncoder(
    mut self_0: *mut BlockEncoder,
    mut histogram_length: size_t,
    mut num_block_types: size_t,
    mut block_types: *const uint8_t,
    mut block_lengths: *const uint32_t,
    num_blocks: size_t,
) {
    (*self_0).histogram_length_ = histogram_length;
    (*self_0).num_block_types_ = num_block_types;
    (*self_0).block_types_ = block_types;
    (*self_0).block_lengths_ = block_lengths;
    (*self_0).num_blocks_ = num_blocks;
    InitBlockTypeCodeCalculator(&raw mut (*self_0).block_split_code_.type_code_calculator);
    (*self_0).block_ix_ = 0 as size_t;
    (*self_0).block_len_ = (if num_blocks == 0 as size_t {
        0 as uint32_t
    } else {
        *block_lengths.offset(0 as c_int as isize)
    }) as size_t;
    (*self_0).entropy_ix_ = 0 as size_t;
    (*self_0).depths_ = ::core::ptr::null_mut::<uint8_t>();
    (*self_0).bits_ = ::core::ptr::null_mut::<uint16_t>();
}
unsafe fn CleanupBlockEncoder(mut m: *mut MemoryManager, mut self_0: *mut BlockEncoder) {
    BrotliFree(m, (*self_0).depths_ as *mut c_void);
    (*self_0).depths_ = ::core::ptr::null_mut::<uint8_t>();
    BrotliFree(m, (*self_0).bits_ as *mut c_void);
    (*self_0).bits_ = ::core::ptr::null_mut::<uint16_t>();
}
unsafe fn BuildAndStoreBlockSwitchEntropyCodes(
    mut self_0: *mut BlockEncoder,
    mut tree: *mut HuffmanTree,
    mut storage_ix: *mut size_t,
    mut storage: *mut uint8_t,
) {
    BuildAndStoreBlockSplitCode(
        (*self_0).block_types_,
        (*self_0).block_lengths_,
        (*self_0).num_blocks_,
        (*self_0).num_block_types_,
        tree,
        &raw mut (*self_0).block_split_code_,
        storage_ix,
        storage,
    );
}
unsafe fn StoreSymbol(
    mut self_0: *mut BlockEncoder,
    mut symbol: size_t,
    mut storage_ix: *mut size_t,
    mut storage: *mut uint8_t,
) {
    if (*self_0).block_len_ == 0 as size_t {
        (*self_0).block_ix_ = (*self_0).block_ix_.wrapping_add(1);
        let mut block_ix: size_t = (*self_0).block_ix_;
        let mut block_len: uint32_t = *(*self_0).block_lengths_.offset(block_ix as isize);
        let mut block_type: uint8_t = *(*self_0).block_types_.offset(block_ix as isize);
        (*self_0).block_len_ = block_len as size_t;
        (*self_0).entropy_ix_ = (block_type as size_t).wrapping_mul((*self_0).histogram_length_);
        StoreBlockSwitch(
            &raw mut (*self_0).block_split_code_,
            block_len,
            block_type,
            0 as c_int,
            storage_ix,
            storage,
        );
    }
    (*self_0).block_len_ = (*self_0).block_len_.wrapping_sub(1);
    let mut ix: size_t = (*self_0).entropy_ix_.wrapping_add(symbol);
    BrotliWriteBits(
        *(*self_0).depths_.offset(ix as isize) as size_t,
        *(*self_0).bits_.offset(ix as isize) as uint64_t,
        storage_ix,
        storage,
    );
}
unsafe fn StoreSymbolWithContext(
    mut self_0: *mut BlockEncoder,
    mut symbol: size_t,
    mut context: size_t,
    mut context_map: *const uint32_t,
    mut storage_ix: *mut size_t,
    mut storage: *mut uint8_t,
    context_bits: size_t,
) {
    if (*self_0).block_len_ == 0 as size_t {
        (*self_0).block_ix_ = (*self_0).block_ix_.wrapping_add(1);
        let mut block_ix: size_t = (*self_0).block_ix_;
        let mut block_len: uint32_t = *(*self_0).block_lengths_.offset(block_ix as isize);
        let mut block_type: uint8_t = *(*self_0).block_types_.offset(block_ix as isize);
        (*self_0).block_len_ = block_len as size_t;
        (*self_0).entropy_ix_ = (block_type as size_t) << context_bits;
        StoreBlockSwitch(
            &raw mut (*self_0).block_split_code_,
            block_len,
            block_type,
            0 as c_int,
            storage_ix,
            storage,
        );
    }
    (*self_0).block_len_ = (*self_0).block_len_.wrapping_sub(1);
    let mut histo_ix: size_t =
        *context_map.offset((*self_0).entropy_ix_.wrapping_add(context) as isize) as size_t;
    let mut ix: size_t = histo_ix
        .wrapping_mul((*self_0).histogram_length_)
        .wrapping_add(symbol);
    BrotliWriteBits(
        *(*self_0).depths_.offset(ix as isize) as size_t,
        *(*self_0).bits_.offset(ix as isize) as uint64_t,
        storage_ix,
        storage,
    );
}
unsafe fn JumpToByteBoundary(mut storage_ix: *mut size_t, mut storage: *mut uint8_t) {
    let storage_ix_view: &mut size_t = unsafe { &mut *storage_ix };
    *storage_ix_view = storage_ix_view.wrapping_add(7 as size_t) & !(7 as c_uint) as size_t;
    *storage.offset((*storage_ix_view >> 3 as c_int) as isize) = 0 as uint8_t;
}
#[inline]
pub unsafe fn BrotliStoreMetaBlock(
    mut m: *mut MemoryManager,
    mut input: *const uint8_t,
    mut start_pos: size_t,
    mut length: size_t,
    mut mask: size_t,
    mut prev_byte: uint8_t,
    mut prev_byte2: uint8_t,
    mut is_last: c_int,
    mut params: *const BrotliEncoderParams,
    mut literal_context_mode: ContextType,
    mut commands: *const Command,
    mut n_commands: size_t,
    mut mb: *const MetaBlockSplit,
    mut storage_ix: *mut size_t,
    mut storage: *mut uint8_t,
) {
    let params_view: &BrotliEncoderParams = unsafe { &*params };
    let mut pos: size_t = start_pos;
    let mut i: size_t = 0;
    let mut num_distance_symbols: uint32_t = params_view.dist.alphabet_size_max;
    let mut num_effective_distance_symbols: uint32_t = params_view.dist.alphabet_size_limit;
    let mut tree: *mut HuffmanTree = ::core::ptr::null_mut::<HuffmanTree>();
    let mut literal_context_lut: ContextLut = (&raw const _kBrotliContextLookupTable
        as *const uint8_t)
        .offset(((literal_context_mode as c_uint) << 9 as c_int) as isize)
        as ContextLut;
    let mut arena: *mut StoreMetablockArena = ::core::ptr::null_mut::<StoreMetablockArena>();
    let mut literal_enc: *mut BlockEncoder = ::core::ptr::null_mut::<BlockEncoder>();
    let mut command_enc: *mut BlockEncoder = ::core::ptr::null_mut::<BlockEncoder>();
    let mut distance_enc: *mut BlockEncoder = ::core::ptr::null_mut::<BlockEncoder>();
    let mut dist: *const BrotliDistanceParams = &raw const params_view.dist;
    StoreCompressedMetaBlockHeader(is_last, length, storage_ix, storage);
    tree = if 2 as c_int * 704 as c_int + 1 as c_int
        > 0 as c_int
    {
        BrotliAllocate(
            m,
            ((2 as c_int * 704 as c_int + 1 as c_int)
                as size_t)
                .wrapping_mul(::core::mem::size_of::<HuffmanTree>() as size_t),
        ) as *mut HuffmanTree
    } else {
        ::core::ptr::null_mut::<HuffmanTree>()
    };
    arena = if 1 as c_int > 0 as c_int {
        BrotliAllocate(
            m,
            (1 as size_t).wrapping_mul(::core::mem::size_of::<StoreMetablockArena>() as size_t),
        ) as *mut StoreMetablockArena
    } else {
        ::core::ptr::null_mut::<StoreMetablockArena>()
    };
    if 0 as c_int != 0 || 0 as c_int != 0 || 0 as c_int != 0
    {
        return;
    }
    literal_enc = &raw mut (*arena).literal_enc;
    command_enc = &raw mut (*arena).command_enc;
    distance_enc = &raw mut (*arena).distance_enc;
    InitBlockEncoder(
        literal_enc,
        BROTLI_NUM_LITERAL_SYMBOLS as size_t,
        (*mb).literal_split.num_types,
        (*mb).literal_split.types,
        (*mb).literal_split.lengths,
        (*mb).literal_split.num_blocks,
    );
    InitBlockEncoder(
        command_enc,
        BROTLI_NUM_COMMAND_SYMBOLS as size_t,
        (*mb).command_split.num_types,
        (*mb).command_split.types,
        (*mb).command_split.lengths,
        (*mb).command_split.num_blocks,
    );
    InitBlockEncoder(
        distance_enc,
        num_effective_distance_symbols as size_t,
        (*mb).distance_split.num_types,
        (*mb).distance_split.types,
        (*mb).distance_split.lengths,
        (*mb).distance_split.num_blocks,
    );
    BuildAndStoreBlockSwitchEntropyCodes(literal_enc, tree, storage_ix, storage);
    BuildAndStoreBlockSwitchEntropyCodes(command_enc, tree, storage_ix, storage);
    BuildAndStoreBlockSwitchEntropyCodes(distance_enc, tree, storage_ix, storage);
    BrotliWriteBits(
        2 as size_t,
        (*dist).distance_postfix_bits as uint64_t,
        storage_ix,
        storage,
    );
    BrotliWriteBits(
        4 as size_t,
        ((*dist).num_direct_distance_codes >> (*dist).distance_postfix_bits) as uint64_t,
        storage_ix,
        storage,
    );
    i = 0 as size_t;
    while i < (*mb).literal_split.num_types {
        BrotliWriteBits(
            2 as size_t,
            literal_context_mode as uint64_t,
            storage_ix,
            storage,
        );
        i = i.wrapping_add(1);
    }
    if (*mb).literal_context_map_size == 0 as size_t {
        StoreTrivialContextMap(
            &raw mut (*arena).context_map_arena,
            (*mb).literal_histograms_size,
            BROTLI_LITERAL_CONTEXT_BITS as size_t,
            tree,
            storage_ix,
            storage,
        );
    } else {
        EncodeContextMap(
            m,
            &raw mut (*arena).context_map_arena,
            (*mb).literal_context_map,
            (*mb).literal_context_map_size,
            (*mb).literal_histograms_size,
            tree,
            storage_ix,
            storage,
        );
        if 0 as c_int != 0 {
            return;
        }
    }
    if (*mb).distance_context_map_size == 0 as size_t {
        StoreTrivialContextMap(
            &raw mut (*arena).context_map_arena,
            (*mb).distance_histograms_size,
            BROTLI_DISTANCE_CONTEXT_BITS as size_t,
            tree,
            storage_ix,
            storage,
        );
    } else {
        EncodeContextMap(
            m,
            &raw mut (*arena).context_map_arena,
            (*mb).distance_context_map,
            (*mb).distance_context_map_size,
            (*mb).distance_histograms_size,
            tree,
            storage_ix,
            storage,
        );
        if 0 as c_int != 0 {
            return;
        }
    }
    BuildAndStoreEntropyCodesLiteral(
        m,
        literal_enc,
        (*mb).literal_histograms,
        (*mb).literal_histograms_size,
        BROTLI_NUM_LITERAL_SYMBOLS as size_t,
        tree,
        storage_ix,
        storage,
    );
    if 0 as c_int != 0 {
        return;
    }
    BuildAndStoreEntropyCodesCommand(
        m,
        command_enc,
        (*mb).command_histograms,
        (*mb).command_histograms_size,
        BROTLI_NUM_COMMAND_SYMBOLS as size_t,
        tree,
        storage_ix,
        storage,
    );
    if 0 as c_int != 0 {
        return;
    }
    BuildAndStoreEntropyCodesDistance(
        m,
        distance_enc,
        (*mb).distance_histograms,
        (*mb).distance_histograms_size,
        num_distance_symbols as size_t,
        tree,
        storage_ix,
        storage,
    );
    if 0 as c_int != 0 {
        return;
    }
    BrotliFree(m, tree as *mut c_void);
    tree = ::core::ptr::null_mut::<HuffmanTree>();
    i = 0 as size_t;
    while i < n_commands {
        let cmd: Command = *commands.offset(i as isize);
        let mut cmd_code: size_t = cmd.cmd_prefix_ as size_t;
        StoreSymbol(command_enc, cmd_code, storage_ix, storage);
        StoreCommandExtra(&raw const cmd, storage_ix, storage);
        if (*mb).literal_context_map_size == 0 as size_t {
            let mut j: size_t = 0;
            j = cmd.insert_len_ as size_t;
            while j != 0 as size_t {
                StoreSymbol(
                    literal_enc,
                    *input.offset((pos & mask) as isize) as size_t,
                    storage_ix,
                    storage,
                );
                pos = pos.wrapping_add(1);
                j = j.wrapping_sub(1);
            }
        } else {
            let mut j_0: size_t = 0;
            j_0 = cmd.insert_len_ as size_t;
            while j_0 != 0 as size_t {
                let mut context: size_t = (*literal_context_lut.offset(prev_byte as isize)
                    as c_int
                    | *literal_context_lut
                        .offset(256 as c_int as isize)
                        .offset(prev_byte2 as isize) as c_int)
                    as size_t;
                let mut literal: uint8_t = *input.offset((pos & mask) as isize);
                StoreSymbolWithContext(
                    literal_enc,
                    literal as size_t,
                    context,
                    (*mb).literal_context_map,
                    storage_ix,
                    storage,
                    BROTLI_LITERAL_CONTEXT_BITS as size_t,
                );
                prev_byte2 = prev_byte;
                prev_byte = literal;
                pos = pos.wrapping_add(1);
                j_0 = j_0.wrapping_sub(1);
            }
        }
        pos = (pos as c_ulong)
            .wrapping_add(CommandCopyLen(&raw const cmd) as c_ulong)
            as size_t as size_t;
        if CommandCopyLen(&raw const cmd) != 0 {
            prev_byte2 = *input.offset((pos.wrapping_sub(2 as size_t) & mask) as isize);
            prev_byte = *input.offset((pos.wrapping_sub(1 as size_t) & mask) as isize);
            if cmd.cmd_prefix_ as c_int >= 128 as c_int {
                let mut dist_code: size_t = (cmd.dist_prefix_ as c_int
                    & 0x3ff as c_int)
                    as size_t;
                let mut distnumextra: uint32_t = (cmd.dist_prefix_ as c_int
                    >> 10 as c_int)
                    as uint32_t;
                let mut distextra: uint64_t = cmd.dist_extra_ as uint64_t;
                if (*mb).distance_context_map_size == 0 as size_t {
                    StoreSymbol(distance_enc, dist_code, storage_ix, storage);
                } else {
                    let mut context_0: size_t = CommandDistanceContext(&raw const cmd) as size_t;
                    StoreSymbolWithContext(
                        distance_enc,
                        dist_code,
                        context_0,
                        (*mb).distance_context_map,
                        storage_ix,
                        storage,
                        BROTLI_DISTANCE_CONTEXT_BITS as size_t,
                    );
                }
                BrotliWriteBits(distnumextra as size_t, distextra, storage_ix, storage);
            }
        }
        i = i.wrapping_add(1);
    }
    CleanupBlockEncoder(m, distance_enc);
    CleanupBlockEncoder(m, command_enc);
    CleanupBlockEncoder(m, literal_enc);
    BrotliFree(m, arena as *mut c_void);
    arena = ::core::ptr::null_mut::<StoreMetablockArena>();
    if is_last != 0 {
        JumpToByteBoundary(storage_ix, storage);
    }
}
unsafe fn BuildHistograms(
    mut input: *const uint8_t,
    mut start_pos: size_t,
    mut mask: size_t,
    mut commands: *const Command,
    mut n_commands: size_t,
    mut lit_histo: *mut HistogramLiteral,
    mut cmd_histo: *mut HistogramCommand,
    mut dist_histo: *mut HistogramDistance,
) {
    let commands_view: &[Command] = unsafe { core::slice::from_raw_parts(commands, (n_commands) as usize) };
    let mut pos: size_t = start_pos;
    let mut i: size_t = 0;
    i = 0 as size_t;
    while i < n_commands {
        let cmd: Command = commands_view[(i) as usize];
        let mut j: size_t = 0;
        HistogramAddCommand(cmd_histo, cmd.cmd_prefix_ as size_t);
        j = cmd.insert_len_ as size_t;
        while j != 0 as size_t {
            HistogramAddLiteral(lit_histo, *input.offset((pos & mask) as isize) as size_t);
            pos = pos.wrapping_add(1);
            j = j.wrapping_sub(1);
        }
        pos = (pos as c_ulong)
            .wrapping_add(CommandCopyLen(&raw const cmd) as c_ulong)
            as size_t as size_t;
        if CommandCopyLen(&raw const cmd) != 0
            && cmd.cmd_prefix_ as c_int >= 128 as c_int
        {
            HistogramAddDistance(
                dist_histo,
                (cmd.dist_prefix_ as c_int & 0x3ff as c_int) as size_t,
            );
        }
        i = i.wrapping_add(1);
    }
}
unsafe fn StoreDataWithHuffmanCodes(
    mut input: *const uint8_t,
    mut start_pos: size_t,
    mut mask: size_t,
    mut commands: *const Command,
    mut n_commands: size_t,
    mut lit_depth: *const uint8_t,
    mut lit_bits: *const uint16_t,
    mut cmd_depth: *const uint8_t,
    mut cmd_bits: *const uint16_t,
    mut dist_depth: *const uint8_t,
    mut dist_bits: *const uint16_t,
    mut storage_ix: *mut size_t,
    mut storage: *mut uint8_t,
) {
    let commands_view: &[Command] = unsafe { core::slice::from_raw_parts(commands, (n_commands) as usize) };
    let mut pos: size_t = start_pos;
    let mut i: size_t = 0;
    i = 0 as size_t;
    while i < n_commands {
        let cmd: Command = commands_view[(i) as usize];
        let cmd_code: size_t = cmd.cmd_prefix_ as size_t;
        let mut j: size_t = 0;
        BrotliWriteBits(
            *cmd_depth.offset(cmd_code as isize) as size_t,
            *cmd_bits.offset(cmd_code as isize) as uint64_t,
            storage_ix,
            storage,
        );
        StoreCommandExtra(&raw const cmd, storage_ix, storage);
        j = cmd.insert_len_ as size_t;
        while j != 0 as size_t {
            let literal: uint8_t = *input.offset((pos & mask) as isize);
            BrotliWriteBits(
                *lit_depth.offset(literal as isize) as size_t,
                *lit_bits.offset(literal as isize) as uint64_t,
                storage_ix,
                storage,
            );
            pos = pos.wrapping_add(1);
            j = j.wrapping_sub(1);
        }
        pos = (pos as c_ulong)
            .wrapping_add(CommandCopyLen(&raw const cmd) as c_ulong)
            as size_t as size_t;
        if CommandCopyLen(&raw const cmd) != 0
            && cmd.cmd_prefix_ as c_int >= 128 as c_int
        {
            let dist_code: size_t =
                (cmd.dist_prefix_ as c_int & 0x3ff as c_int) as size_t;
            let distnumextra: uint32_t =
                (cmd.dist_prefix_ as c_int >> 10 as c_int) as uint32_t;
            let distextra: uint32_t = cmd.dist_extra_;
            BrotliWriteBits(
                *dist_depth.offset(dist_code as isize) as size_t,
                *dist_bits.offset(dist_code as isize) as uint64_t,
                storage_ix,
                storage,
            );
            BrotliWriteBits(
                distnumextra as size_t,
                distextra as uint64_t,
                storage_ix,
                storage,
            );
        }
        i = i.wrapping_add(1);
    }
}
#[inline]
pub unsafe fn BrotliStoreMetaBlockTrivial(
    mut m: *mut MemoryManager,
    mut input: *const uint8_t,
    mut start_pos: size_t,
    mut length: size_t,
    mut mask: size_t,
    mut is_last: c_int,
    mut params: *const BrotliEncoderParams,
    mut commands: *const Command,
    mut n_commands: size_t,
    mut storage_ix: *mut size_t,
    mut storage: *mut uint8_t,
) {
    let params_view: &BrotliEncoderParams = unsafe { &*params };
    let mut arena: *mut MetablockArena = if 1 as c_int > 0 as c_int {
        BrotliAllocate(
            m,
            (1 as size_t).wrapping_mul(::core::mem::size_of::<MetablockArena>() as size_t),
        ) as *mut MetablockArena
    } else {
        ::core::ptr::null_mut::<MetablockArena>()
    };
    let mut num_distance_symbols: uint32_t = params_view.dist.alphabet_size_max;
    if 0 as c_int != 0 || 0 as c_int != 0 {
        return;
    }
    StoreCompressedMetaBlockHeader(is_last, length, storage_ix, storage);
    HistogramClearLiteral(&raw mut (*arena).lit_histo);
    HistogramClearCommand(&raw mut (*arena).cmd_histo);
    HistogramClearDistance(&raw mut (*arena).dist_histo);
    BuildHistograms(
        input,
        start_pos,
        mask,
        commands,
        n_commands,
        &raw mut (*arena).lit_histo,
        &raw mut (*arena).cmd_histo,
        &raw mut (*arena).dist_histo,
    );
    BrotliWriteBits(13 as size_t, 0 as uint64_t, storage_ix, storage);
    BuildAndStoreHuffmanTree(
        &raw mut (*arena).lit_histo.data_ as *mut uint32_t,
        BROTLI_NUM_LITERAL_SYMBOLS as size_t,
        BROTLI_NUM_LITERAL_SYMBOLS as size_t,
        &raw mut (*arena).tree as *mut HuffmanTree,
        &raw mut (*arena).lit_depth as *mut uint8_t,
        &raw mut (*arena).lit_bits as *mut uint16_t,
        storage_ix,
        storage,
    );
    BuildAndStoreHuffmanTree(
        &raw mut (*arena).cmd_histo.data_ as *mut uint32_t,
        BROTLI_NUM_COMMAND_SYMBOLS as size_t,
        BROTLI_NUM_COMMAND_SYMBOLS as size_t,
        &raw mut (*arena).tree as *mut HuffmanTree,
        &raw mut (*arena).cmd_depth as *mut uint8_t,
        &raw mut (*arena).cmd_bits as *mut uint16_t,
        storage_ix,
        storage,
    );
    BuildAndStoreHuffmanTree(
        &raw mut (*arena).dist_histo.data_ as *mut uint32_t,
        MAX_SIMPLE_DISTANCE_ALPHABET_SIZE as size_t,
        num_distance_symbols as size_t,
        &raw mut (*arena).tree as *mut HuffmanTree,
        &raw mut (*arena).dist_depth as *mut uint8_t,
        &raw mut (*arena).dist_bits as *mut uint16_t,
        storage_ix,
        storage,
    );
    StoreDataWithHuffmanCodes(
        input,
        start_pos,
        mask,
        commands,
        n_commands,
        &raw mut (*arena).lit_depth as *mut uint8_t,
        &raw mut (*arena).lit_bits as *mut uint16_t,
        &raw mut (*arena).cmd_depth as *mut uint8_t,
        &raw mut (*arena).cmd_bits as *mut uint16_t,
        &raw mut (*arena).dist_depth as *mut uint8_t,
        &raw mut (*arena).dist_bits as *mut uint16_t,
        storage_ix,
        storage,
    );
    BrotliFree(m, arena as *mut c_void);
    arena = ::core::ptr::null_mut::<MetablockArena>();
    if is_last != 0 {
        JumpToByteBoundary(storage_ix, storage);
    }
}
#[inline]
pub unsafe fn BrotliStoreMetaBlockFast(
    mut m: *mut MemoryManager,
    mut input: *const uint8_t,
    mut start_pos: size_t,
    mut length: size_t,
    mut mask: size_t,
    mut is_last: c_int,
    mut params: *const BrotliEncoderParams,
    mut commands: *const Command,
    mut n_commands: size_t,
    mut storage_ix: *mut size_t,
    mut storage: *mut uint8_t,
) {
    let params_view: &BrotliEncoderParams = unsafe { &*params };
    let mut arena: *mut MetablockArena = if 1 as c_int > 0 as c_int {
        BrotliAllocate(
            m,
            (1 as size_t).wrapping_mul(::core::mem::size_of::<MetablockArena>() as size_t),
        ) as *mut MetablockArena
    } else {
        ::core::ptr::null_mut::<MetablockArena>()
    };
    let mut num_distance_symbols: uint32_t = params_view.dist.alphabet_size_max;
    let mut distance_alphabet_bits: uint32_t =
        Log2FloorNonZero(num_distance_symbols.wrapping_sub(1 as uint32_t) as size_t)
            .wrapping_add(1 as uint32_t);
    if 0 as c_int != 0 || 0 as c_int != 0 {
        return;
    }
    StoreCompressedMetaBlockHeader(is_last, length, storage_ix, storage);
    BrotliWriteBits(13 as size_t, 0 as uint64_t, storage_ix, storage);
    if n_commands <= 128 as size_t {
        let mut histogram: [uint32_t; 256] = [
            0 as c_int as uint32_t,
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
        let mut pos: size_t = start_pos;
        let mut num_literals: size_t = 0 as size_t;
        let mut i: size_t = 0;
        i = 0 as size_t;
        while i < n_commands {
            let cmd: Command = *commands.offset(i as isize);
            let mut j: size_t = 0;
            j = cmd.insert_len_ as size_t;
            while j != 0 as size_t {
                histogram[*input.offset((pos & mask) as isize) as usize] =
                    histogram[*input.offset((pos & mask) as isize) as usize].wrapping_add(1);
                pos = pos.wrapping_add(1);
                j = j.wrapping_sub(1);
            }
            num_literals = (num_literals as c_ulong)
                .wrapping_add(cmd.insert_len_ as c_ulong)
                as size_t as size_t;
            pos = (pos as c_ulong)
                .wrapping_add(CommandCopyLen(&raw const cmd) as c_ulong)
                as size_t as size_t;
            i = i.wrapping_add(1);
        }
        BrotliBuildAndStoreHuffmanTreeFast(
            &raw mut (*arena).tree as *mut HuffmanTree,
            &raw mut histogram as *mut uint32_t,
            num_literals,
            8 as size_t,
            &raw mut (*arena).lit_depth as *mut uint8_t,
            &raw mut (*arena).lit_bits as *mut uint16_t,
            storage_ix,
            storage,
        );
        StoreStaticCommandHuffmanTree(storage_ix, storage);
        StoreStaticDistanceHuffmanTree(storage_ix, storage);
        StoreDataWithHuffmanCodes(
            input,
            start_pos,
            mask,
            commands,
            n_commands,
            &raw mut (*arena).lit_depth as *mut uint8_t,
            &raw mut (*arena).lit_bits as *mut uint16_t,
            &raw const kStaticCommandCodeDepth as *const uint8_t,
            &raw const kStaticCommandCodeBits as *const uint16_t,
            &raw const kStaticDistanceCodeDepth as *const uint8_t,
            &raw const kStaticDistanceCodeBits as *const uint16_t,
            storage_ix,
            storage,
        );
    } else {
        HistogramClearLiteral(&raw mut (*arena).lit_histo);
        HistogramClearCommand(&raw mut (*arena).cmd_histo);
        HistogramClearDistance(&raw mut (*arena).dist_histo);
        BuildHistograms(
            input,
            start_pos,
            mask,
            commands,
            n_commands,
            &raw mut (*arena).lit_histo,
            &raw mut (*arena).cmd_histo,
            &raw mut (*arena).dist_histo,
        );
        BrotliBuildAndStoreHuffmanTreeFast(
            &raw mut (*arena).tree as *mut HuffmanTree,
            &raw mut (*arena).lit_histo.data_ as *mut uint32_t,
            (*arena).lit_histo.total_count_,
            8 as size_t,
            &raw mut (*arena).lit_depth as *mut uint8_t,
            &raw mut (*arena).lit_bits as *mut uint16_t,
            storage_ix,
            storage,
        );
        BrotliBuildAndStoreHuffmanTreeFast(
            &raw mut (*arena).tree as *mut HuffmanTree,
            &raw mut (*arena).cmd_histo.data_ as *mut uint32_t,
            (*arena).cmd_histo.total_count_,
            10 as size_t,
            &raw mut (*arena).cmd_depth as *mut uint8_t,
            &raw mut (*arena).cmd_bits as *mut uint16_t,
            storage_ix,
            storage,
        );
        BrotliBuildAndStoreHuffmanTreeFast(
            &raw mut (*arena).tree as *mut HuffmanTree,
            &raw mut (*arena).dist_histo.data_ as *mut uint32_t,
            (*arena).dist_histo.total_count_,
            distance_alphabet_bits as size_t,
            &raw mut (*arena).dist_depth as *mut uint8_t,
            &raw mut (*arena).dist_bits as *mut uint16_t,
            storage_ix,
            storage,
        );
        StoreDataWithHuffmanCodes(
            input,
            start_pos,
            mask,
            commands,
            n_commands,
            &raw mut (*arena).lit_depth as *mut uint8_t,
            &raw mut (*arena).lit_bits as *mut uint16_t,
            &raw mut (*arena).cmd_depth as *mut uint8_t,
            &raw mut (*arena).cmd_bits as *mut uint16_t,
            &raw mut (*arena).dist_depth as *mut uint8_t,
            &raw mut (*arena).dist_bits as *mut uint16_t,
            storage_ix,
            storage,
        );
    }
    BrotliFree(m, arena as *mut c_void);
    arena = ::core::ptr::null_mut::<MetablockArena>();
    if is_last != 0 {
        JumpToByteBoundary(storage_ix, storage);
    }
}
#[inline]
pub unsafe fn BrotliStoreUncompressedMetaBlock(
    mut is_final_block: c_int,
    mut input: *const uint8_t,
    mut position: size_t,
    mut mask: size_t,
    mut len: size_t,
    mut storage_ix: *mut size_t,
    mut storage: *mut uint8_t,
) {
    let mut masked_pos: size_t = position & mask;
    BrotliStoreUncompressedMetaBlockHeader(len, storage_ix, storage);
    JumpToByteBoundary(storage_ix, storage);
    if masked_pos.wrapping_add(len) > mask.wrapping_add(1 as size_t) {
        let mut len1: size_t = mask.wrapping_add(1 as size_t).wrapping_sub(masked_pos);
        memcpy(
            storage.offset((*storage_ix >> 3 as c_int) as isize) as *mut uint8_t
                as *mut c_void,
            input.offset(masked_pos as isize) as *const uint8_t as *const c_void,
            len1,
        );
        *storage_ix = (*storage_ix as c_ulong)
            .wrapping_add((len1 << 3 as c_int) as c_ulong)
            as size_t as size_t;
        len = (len as c_ulong).wrapping_sub(len1 as c_ulong) as size_t
            as size_t;
        masked_pos = 0 as size_t;
    }
    memcpy(
        storage.offset((*storage_ix >> 3 as c_int) as isize) as *mut uint8_t
            as *mut c_void,
        input.offset(masked_pos as isize) as *const uint8_t as *const c_void,
        len,
    );
    *storage_ix = (*storage_ix as c_ulong)
        .wrapping_add((len << 3 as c_int) as c_ulong)
        as size_t as size_t;
    BrotliWriteBitsPrepareStorage(*storage_ix, storage);
    if is_final_block != 0 {
        BrotliWriteBits(1 as size_t, 1 as uint64_t, storage_ix, storage);
        BrotliWriteBits(1 as size_t, 1 as uint64_t, storage_ix, storage);
        JumpToByteBoundary(storage_ix, storage);
    }
}
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
#[inline(always)]
unsafe fn BrotliWriteBitsPrepareStorage(mut pos: size_t, mut array: *mut uint8_t) {
    *array.offset((pos >> 3 as c_int) as isize) = 0 as uint8_t;
}
static mut kCodeLengthDepth: [uint8_t; 18] = [
    4 as c_int as uint8_t,
    4 as c_int as uint8_t,
    4 as c_int as uint8_t,
    4 as c_int as uint8_t,
    4 as c_int as uint8_t,
    4 as c_int as uint8_t,
    4 as c_int as uint8_t,
    4 as c_int as uint8_t,
    4 as c_int as uint8_t,
    4 as c_int as uint8_t,
    4 as c_int as uint8_t,
    4 as c_int as uint8_t,
    4 as c_int as uint8_t,
    5 as c_int as uint8_t,
    5 as c_int as uint8_t,
    0 as c_int as uint8_t,
    4 as c_int as uint8_t,
    4 as c_int as uint8_t,
];
static mut kStaticCommandCodeDepth: [uint8_t; 704] = [
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
];
static mut kStaticDistanceCodeDepth: [uint8_t; 64] = [
    6 as c_int as uint8_t,
    6 as c_int as uint8_t,
    6 as c_int as uint8_t,
    6 as c_int as uint8_t,
    6 as c_int as uint8_t,
    6 as c_int as uint8_t,
    6 as c_int as uint8_t,
    6 as c_int as uint8_t,
    6 as c_int as uint8_t,
    6 as c_int as uint8_t,
    6 as c_int as uint8_t,
    6 as c_int as uint8_t,
    6 as c_int as uint8_t,
    6 as c_int as uint8_t,
    6 as c_int as uint8_t,
    6 as c_int as uint8_t,
    6 as c_int as uint8_t,
    6 as c_int as uint8_t,
    6 as c_int as uint8_t,
    6 as c_int as uint8_t,
    6 as c_int as uint8_t,
    6 as c_int as uint8_t,
    6 as c_int as uint8_t,
    6 as c_int as uint8_t,
    6 as c_int as uint8_t,
    6 as c_int as uint8_t,
    6 as c_int as uint8_t,
    6 as c_int as uint8_t,
    6 as c_int as uint8_t,
    6 as c_int as uint8_t,
    6 as c_int as uint8_t,
    6 as c_int as uint8_t,
    6 as c_int as uint8_t,
    6 as c_int as uint8_t,
    6 as c_int as uint8_t,
    6 as c_int as uint8_t,
    6 as c_int as uint8_t,
    6 as c_int as uint8_t,
    6 as c_int as uint8_t,
    6 as c_int as uint8_t,
    6 as c_int as uint8_t,
    6 as c_int as uint8_t,
    6 as c_int as uint8_t,
    6 as c_int as uint8_t,
    6 as c_int as uint8_t,
    6 as c_int as uint8_t,
    6 as c_int as uint8_t,
    6 as c_int as uint8_t,
    6 as c_int as uint8_t,
    6 as c_int as uint8_t,
    6 as c_int as uint8_t,
    6 as c_int as uint8_t,
    6 as c_int as uint8_t,
    6 as c_int as uint8_t,
    6 as c_int as uint8_t,
    6 as c_int as uint8_t,
    6 as c_int as uint8_t,
    6 as c_int as uint8_t,
    6 as c_int as uint8_t,
    6 as c_int as uint8_t,
    6 as c_int as uint8_t,
    6 as c_int as uint8_t,
    6 as c_int as uint8_t,
    6 as c_int as uint8_t,
];
static mut kCodeLengthBits: [uint32_t; 18] = [
    0 as c_int as uint32_t,
    8 as c_int as uint32_t,
    4 as c_int as uint32_t,
    12 as c_int as uint32_t,
    2 as c_int as uint32_t,
    10 as c_int as uint32_t,
    6 as c_int as uint32_t,
    14 as c_int as uint32_t,
    1 as c_int as uint32_t,
    9 as c_int as uint32_t,
    5 as c_int as uint32_t,
    13 as c_int as uint32_t,
    3 as c_int as uint32_t,
    15 as c_int as uint32_t,
    31 as c_int as uint32_t,
    0 as c_int as uint32_t,
    11 as c_int as uint32_t,
    7 as c_int as uint32_t,
];
#[inline(always)]
unsafe fn StoreStaticCodeLengthCode(
    mut storage_ix: *mut size_t,
    mut storage: *mut uint8_t,
) {
    BrotliWriteBits(
        40 as size_t,
        (0xff as c_uint as uint64_t) << 32 as c_int
            | 0x55555554 as uint64_t,
        storage_ix,
        storage,
    );
}
static mut kZeroRepsBits: [uint64_t; 704] = [
    0 as c_int as uint64_t,
    0 as c_int as uint64_t,
    0 as c_int as uint64_t,
    0x7 as c_int as uint64_t,
    0x17 as c_int as uint64_t,
    0x27 as c_int as uint64_t,
    0x37 as c_int as uint64_t,
    0x47 as c_int as uint64_t,
    0x57 as c_int as uint64_t,
    0x67 as c_int as uint64_t,
    0x77 as c_int as uint64_t,
    0x770 as c_int as uint64_t,
    0xb87 as c_int as uint64_t,
    0x1387 as c_int as uint64_t,
    0x1b87 as c_int as uint64_t,
    0x2387 as c_int as uint64_t,
    0x2b87 as c_int as uint64_t,
    0x3387 as c_int as uint64_t,
    0x3b87 as c_int as uint64_t,
    0x397 as c_int as uint64_t,
    0xb97 as c_int as uint64_t,
    0x1397 as c_int as uint64_t,
    0x1b97 as c_int as uint64_t,
    0x2397 as c_int as uint64_t,
    0x2b97 as c_int as uint64_t,
    0x3397 as c_int as uint64_t,
    0x3b97 as c_int as uint64_t,
    0x3a7 as c_int as uint64_t,
    0xba7 as c_int as uint64_t,
    0x13a7 as c_int as uint64_t,
    0x1ba7 as c_int as uint64_t,
    0x23a7 as c_int as uint64_t,
    0x2ba7 as c_int as uint64_t,
    0x33a7 as c_int as uint64_t,
    0x3ba7 as c_int as uint64_t,
    0x3b7 as c_int as uint64_t,
    0xbb7 as c_int as uint64_t,
    0x13b7 as c_int as uint64_t,
    0x1bb7 as c_int as uint64_t,
    0x23b7 as c_int as uint64_t,
    0x2bb7 as c_int as uint64_t,
    0x33b7 as c_int as uint64_t,
    0x3bb7 as c_int as uint64_t,
    0x3c7 as c_int as uint64_t,
    0xbc7 as c_int as uint64_t,
    0x13c7 as c_int as uint64_t,
    0x1bc7 as c_int as uint64_t,
    0x23c7 as c_int as uint64_t,
    0x2bc7 as c_int as uint64_t,
    0x33c7 as c_int as uint64_t,
    0x3bc7 as c_int as uint64_t,
    0x3d7 as c_int as uint64_t,
    0xbd7 as c_int as uint64_t,
    0x13d7 as c_int as uint64_t,
    0x1bd7 as c_int as uint64_t,
    0x23d7 as c_int as uint64_t,
    0x2bd7 as c_int as uint64_t,
    0x33d7 as c_int as uint64_t,
    0x3bd7 as c_int as uint64_t,
    0x3e7 as c_int as uint64_t,
    0xbe7 as c_int as uint64_t,
    0x13e7 as c_int as uint64_t,
    0x1be7 as c_int as uint64_t,
    0x23e7 as c_int as uint64_t,
    0x2be7 as c_int as uint64_t,
    0x33e7 as c_int as uint64_t,
    0x3be7 as c_int as uint64_t,
    0x3f7 as c_int as uint64_t,
    0xbf7 as c_int as uint64_t,
    0x13f7 as c_int as uint64_t,
    0x1bf7 as c_int as uint64_t,
    0x23f7 as c_int as uint64_t,
    0x2bf7 as c_int as uint64_t,
    0x33f7 as c_int as uint64_t,
    0x3bf7 as c_int as uint64_t,
    0x1c387 as c_int as uint64_t,
    0x5c387 as c_int as uint64_t,
    0x9c387 as c_int as uint64_t,
    0xdc387 as c_int as uint64_t,
    0x11c387 as c_int as uint64_t,
    0x15c387 as c_int as uint64_t,
    0x19c387 as c_int as uint64_t,
    0x1dc387 as c_int as uint64_t,
    0x1cb87 as c_int as uint64_t,
    0x5cb87 as c_int as uint64_t,
    0x9cb87 as c_int as uint64_t,
    0xdcb87 as c_int as uint64_t,
    0x11cb87 as c_int as uint64_t,
    0x15cb87 as c_int as uint64_t,
    0x19cb87 as c_int as uint64_t,
    0x1dcb87 as c_int as uint64_t,
    0x1d387 as c_int as uint64_t,
    0x5d387 as c_int as uint64_t,
    0x9d387 as c_int as uint64_t,
    0xdd387 as c_int as uint64_t,
    0x11d387 as c_int as uint64_t,
    0x15d387 as c_int as uint64_t,
    0x19d387 as c_int as uint64_t,
    0x1dd387 as c_int as uint64_t,
    0x1db87 as c_int as uint64_t,
    0x5db87 as c_int as uint64_t,
    0x9db87 as c_int as uint64_t,
    0xddb87 as c_int as uint64_t,
    0x11db87 as c_int as uint64_t,
    0x15db87 as c_int as uint64_t,
    0x19db87 as c_int as uint64_t,
    0x1ddb87 as c_int as uint64_t,
    0x1e387 as c_int as uint64_t,
    0x5e387 as c_int as uint64_t,
    0x9e387 as c_int as uint64_t,
    0xde387 as c_int as uint64_t,
    0x11e387 as c_int as uint64_t,
    0x15e387 as c_int as uint64_t,
    0x19e387 as c_int as uint64_t,
    0x1de387 as c_int as uint64_t,
    0x1eb87 as c_int as uint64_t,
    0x5eb87 as c_int as uint64_t,
    0x9eb87 as c_int as uint64_t,
    0xdeb87 as c_int as uint64_t,
    0x11eb87 as c_int as uint64_t,
    0x15eb87 as c_int as uint64_t,
    0x19eb87 as c_int as uint64_t,
    0x1deb87 as c_int as uint64_t,
    0x1f387 as c_int as uint64_t,
    0x5f387 as c_int as uint64_t,
    0x9f387 as c_int as uint64_t,
    0xdf387 as c_int as uint64_t,
    0x11f387 as c_int as uint64_t,
    0x15f387 as c_int as uint64_t,
    0x19f387 as c_int as uint64_t,
    0x1df387 as c_int as uint64_t,
    0x1fb87 as c_int as uint64_t,
    0x5fb87 as c_int as uint64_t,
    0x9fb87 as c_int as uint64_t,
    0xdfb87 as c_int as uint64_t,
    0x11fb87 as c_int as uint64_t,
    0x15fb87 as c_int as uint64_t,
    0x19fb87 as c_int as uint64_t,
    0x1dfb87 as c_int as uint64_t,
    0x1c397 as c_int as uint64_t,
    0x5c397 as c_int as uint64_t,
    0x9c397 as c_int as uint64_t,
    0xdc397 as c_int as uint64_t,
    0x11c397 as c_int as uint64_t,
    0x15c397 as c_int as uint64_t,
    0x19c397 as c_int as uint64_t,
    0x1dc397 as c_int as uint64_t,
    0x1cb97 as c_int as uint64_t,
    0x5cb97 as c_int as uint64_t,
    0x9cb97 as c_int as uint64_t,
    0xdcb97 as c_int as uint64_t,
    0x11cb97 as c_int as uint64_t,
    0x15cb97 as c_int as uint64_t,
    0x19cb97 as c_int as uint64_t,
    0x1dcb97 as c_int as uint64_t,
    0x1d397 as c_int as uint64_t,
    0x5d397 as c_int as uint64_t,
    0x9d397 as c_int as uint64_t,
    0xdd397 as c_int as uint64_t,
    0x11d397 as c_int as uint64_t,
    0x15d397 as c_int as uint64_t,
    0x19d397 as c_int as uint64_t,
    0x1dd397 as c_int as uint64_t,
    0x1db97 as c_int as uint64_t,
    0x5db97 as c_int as uint64_t,
    0x9db97 as c_int as uint64_t,
    0xddb97 as c_int as uint64_t,
    0x11db97 as c_int as uint64_t,
    0x15db97 as c_int as uint64_t,
    0x19db97 as c_int as uint64_t,
    0x1ddb97 as c_int as uint64_t,
    0x1e397 as c_int as uint64_t,
    0x5e397 as c_int as uint64_t,
    0x9e397 as c_int as uint64_t,
    0xde397 as c_int as uint64_t,
    0x11e397 as c_int as uint64_t,
    0x15e397 as c_int as uint64_t,
    0x19e397 as c_int as uint64_t,
    0x1de397 as c_int as uint64_t,
    0x1eb97 as c_int as uint64_t,
    0x5eb97 as c_int as uint64_t,
    0x9eb97 as c_int as uint64_t,
    0xdeb97 as c_int as uint64_t,
    0x11eb97 as c_int as uint64_t,
    0x15eb97 as c_int as uint64_t,
    0x19eb97 as c_int as uint64_t,
    0x1deb97 as c_int as uint64_t,
    0x1f397 as c_int as uint64_t,
    0x5f397 as c_int as uint64_t,
    0x9f397 as c_int as uint64_t,
    0xdf397 as c_int as uint64_t,
    0x11f397 as c_int as uint64_t,
    0x15f397 as c_int as uint64_t,
    0x19f397 as c_int as uint64_t,
    0x1df397 as c_int as uint64_t,
    0x1fb97 as c_int as uint64_t,
    0x5fb97 as c_int as uint64_t,
    0x9fb97 as c_int as uint64_t,
    0xdfb97 as c_int as uint64_t,
    0x11fb97 as c_int as uint64_t,
    0x15fb97 as c_int as uint64_t,
    0x19fb97 as c_int as uint64_t,
    0x1dfb97 as c_int as uint64_t,
    0x1c3a7 as c_int as uint64_t,
    0x5c3a7 as c_int as uint64_t,
    0x9c3a7 as c_int as uint64_t,
    0xdc3a7 as c_int as uint64_t,
    0x11c3a7 as c_int as uint64_t,
    0x15c3a7 as c_int as uint64_t,
    0x19c3a7 as c_int as uint64_t,
    0x1dc3a7 as c_int as uint64_t,
    0x1cba7 as c_int as uint64_t,
    0x5cba7 as c_int as uint64_t,
    0x9cba7 as c_int as uint64_t,
    0xdcba7 as c_int as uint64_t,
    0x11cba7 as c_int as uint64_t,
    0x15cba7 as c_int as uint64_t,
    0x19cba7 as c_int as uint64_t,
    0x1dcba7 as c_int as uint64_t,
    0x1d3a7 as c_int as uint64_t,
    0x5d3a7 as c_int as uint64_t,
    0x9d3a7 as c_int as uint64_t,
    0xdd3a7 as c_int as uint64_t,
    0x11d3a7 as c_int as uint64_t,
    0x15d3a7 as c_int as uint64_t,
    0x19d3a7 as c_int as uint64_t,
    0x1dd3a7 as c_int as uint64_t,
    0x1dba7 as c_int as uint64_t,
    0x5dba7 as c_int as uint64_t,
    0x9dba7 as c_int as uint64_t,
    0xddba7 as c_int as uint64_t,
    0x11dba7 as c_int as uint64_t,
    0x15dba7 as c_int as uint64_t,
    0x19dba7 as c_int as uint64_t,
    0x1ddba7 as c_int as uint64_t,
    0x1e3a7 as c_int as uint64_t,
    0x5e3a7 as c_int as uint64_t,
    0x9e3a7 as c_int as uint64_t,
    0xde3a7 as c_int as uint64_t,
    0x11e3a7 as c_int as uint64_t,
    0x15e3a7 as c_int as uint64_t,
    0x19e3a7 as c_int as uint64_t,
    0x1de3a7 as c_int as uint64_t,
    0x1eba7 as c_int as uint64_t,
    0x5eba7 as c_int as uint64_t,
    0x9eba7 as c_int as uint64_t,
    0xdeba7 as c_int as uint64_t,
    0x11eba7 as c_int as uint64_t,
    0x15eba7 as c_int as uint64_t,
    0x19eba7 as c_int as uint64_t,
    0x1deba7 as c_int as uint64_t,
    0x1f3a7 as c_int as uint64_t,
    0x5f3a7 as c_int as uint64_t,
    0x9f3a7 as c_int as uint64_t,
    0xdf3a7 as c_int as uint64_t,
    0x11f3a7 as c_int as uint64_t,
    0x15f3a7 as c_int as uint64_t,
    0x19f3a7 as c_int as uint64_t,
    0x1df3a7 as c_int as uint64_t,
    0x1fba7 as c_int as uint64_t,
    0x5fba7 as c_int as uint64_t,
    0x9fba7 as c_int as uint64_t,
    0xdfba7 as c_int as uint64_t,
    0x11fba7 as c_int as uint64_t,
    0x15fba7 as c_int as uint64_t,
    0x19fba7 as c_int as uint64_t,
    0x1dfba7 as c_int as uint64_t,
    0x1c3b7 as c_int as uint64_t,
    0x5c3b7 as c_int as uint64_t,
    0x9c3b7 as c_int as uint64_t,
    0xdc3b7 as c_int as uint64_t,
    0x11c3b7 as c_int as uint64_t,
    0x15c3b7 as c_int as uint64_t,
    0x19c3b7 as c_int as uint64_t,
    0x1dc3b7 as c_int as uint64_t,
    0x1cbb7 as c_int as uint64_t,
    0x5cbb7 as c_int as uint64_t,
    0x9cbb7 as c_int as uint64_t,
    0xdcbb7 as c_int as uint64_t,
    0x11cbb7 as c_int as uint64_t,
    0x15cbb7 as c_int as uint64_t,
    0x19cbb7 as c_int as uint64_t,
    0x1dcbb7 as c_int as uint64_t,
    0x1d3b7 as c_int as uint64_t,
    0x5d3b7 as c_int as uint64_t,
    0x9d3b7 as c_int as uint64_t,
    0xdd3b7 as c_int as uint64_t,
    0x11d3b7 as c_int as uint64_t,
    0x15d3b7 as c_int as uint64_t,
    0x19d3b7 as c_int as uint64_t,
    0x1dd3b7 as c_int as uint64_t,
    0x1dbb7 as c_int as uint64_t,
    0x5dbb7 as c_int as uint64_t,
    0x9dbb7 as c_int as uint64_t,
    0xddbb7 as c_int as uint64_t,
    0x11dbb7 as c_int as uint64_t,
    0x15dbb7 as c_int as uint64_t,
    0x19dbb7 as c_int as uint64_t,
    0x1ddbb7 as c_int as uint64_t,
    0x1e3b7 as c_int as uint64_t,
    0x5e3b7 as c_int as uint64_t,
    0x9e3b7 as c_int as uint64_t,
    0xde3b7 as c_int as uint64_t,
    0x11e3b7 as c_int as uint64_t,
    0x15e3b7 as c_int as uint64_t,
    0x19e3b7 as c_int as uint64_t,
    0x1de3b7 as c_int as uint64_t,
    0x1ebb7 as c_int as uint64_t,
    0x5ebb7 as c_int as uint64_t,
    0x9ebb7 as c_int as uint64_t,
    0xdebb7 as c_int as uint64_t,
    0x11ebb7 as c_int as uint64_t,
    0x15ebb7 as c_int as uint64_t,
    0x19ebb7 as c_int as uint64_t,
    0x1debb7 as c_int as uint64_t,
    0x1f3b7 as c_int as uint64_t,
    0x5f3b7 as c_int as uint64_t,
    0x9f3b7 as c_int as uint64_t,
    0xdf3b7 as c_int as uint64_t,
    0x11f3b7 as c_int as uint64_t,
    0x15f3b7 as c_int as uint64_t,
    0x19f3b7 as c_int as uint64_t,
    0x1df3b7 as c_int as uint64_t,
    0x1fbb7 as c_int as uint64_t,
    0x5fbb7 as c_int as uint64_t,
    0x9fbb7 as c_int as uint64_t,
    0xdfbb7 as c_int as uint64_t,
    0x11fbb7 as c_int as uint64_t,
    0x15fbb7 as c_int as uint64_t,
    0x19fbb7 as c_int as uint64_t,
    0x1dfbb7 as c_int as uint64_t,
    0x1c3c7 as c_int as uint64_t,
    0x5c3c7 as c_int as uint64_t,
    0x9c3c7 as c_int as uint64_t,
    0xdc3c7 as c_int as uint64_t,
    0x11c3c7 as c_int as uint64_t,
    0x15c3c7 as c_int as uint64_t,
    0x19c3c7 as c_int as uint64_t,
    0x1dc3c7 as c_int as uint64_t,
    0x1cbc7 as c_int as uint64_t,
    0x5cbc7 as c_int as uint64_t,
    0x9cbc7 as c_int as uint64_t,
    0xdcbc7 as c_int as uint64_t,
    0x11cbc7 as c_int as uint64_t,
    0x15cbc7 as c_int as uint64_t,
    0x19cbc7 as c_int as uint64_t,
    0x1dcbc7 as c_int as uint64_t,
    0x1d3c7 as c_int as uint64_t,
    0x5d3c7 as c_int as uint64_t,
    0x9d3c7 as c_int as uint64_t,
    0xdd3c7 as c_int as uint64_t,
    0x11d3c7 as c_int as uint64_t,
    0x15d3c7 as c_int as uint64_t,
    0x19d3c7 as c_int as uint64_t,
    0x1dd3c7 as c_int as uint64_t,
    0x1dbc7 as c_int as uint64_t,
    0x5dbc7 as c_int as uint64_t,
    0x9dbc7 as c_int as uint64_t,
    0xddbc7 as c_int as uint64_t,
    0x11dbc7 as c_int as uint64_t,
    0x15dbc7 as c_int as uint64_t,
    0x19dbc7 as c_int as uint64_t,
    0x1ddbc7 as c_int as uint64_t,
    0x1e3c7 as c_int as uint64_t,
    0x5e3c7 as c_int as uint64_t,
    0x9e3c7 as c_int as uint64_t,
    0xde3c7 as c_int as uint64_t,
    0x11e3c7 as c_int as uint64_t,
    0x15e3c7 as c_int as uint64_t,
    0x19e3c7 as c_int as uint64_t,
    0x1de3c7 as c_int as uint64_t,
    0x1ebc7 as c_int as uint64_t,
    0x5ebc7 as c_int as uint64_t,
    0x9ebc7 as c_int as uint64_t,
    0xdebc7 as c_int as uint64_t,
    0x11ebc7 as c_int as uint64_t,
    0x15ebc7 as c_int as uint64_t,
    0x19ebc7 as c_int as uint64_t,
    0x1debc7 as c_int as uint64_t,
    0x1f3c7 as c_int as uint64_t,
    0x5f3c7 as c_int as uint64_t,
    0x9f3c7 as c_int as uint64_t,
    0xdf3c7 as c_int as uint64_t,
    0x11f3c7 as c_int as uint64_t,
    0x15f3c7 as c_int as uint64_t,
    0x19f3c7 as c_int as uint64_t,
    0x1df3c7 as c_int as uint64_t,
    0x1fbc7 as c_int as uint64_t,
    0x5fbc7 as c_int as uint64_t,
    0x9fbc7 as c_int as uint64_t,
    0xdfbc7 as c_int as uint64_t,
    0x11fbc7 as c_int as uint64_t,
    0x15fbc7 as c_int as uint64_t,
    0x19fbc7 as c_int as uint64_t,
    0x1dfbc7 as c_int as uint64_t,
    0x1c3d7 as c_int as uint64_t,
    0x5c3d7 as c_int as uint64_t,
    0x9c3d7 as c_int as uint64_t,
    0xdc3d7 as c_int as uint64_t,
    0x11c3d7 as c_int as uint64_t,
    0x15c3d7 as c_int as uint64_t,
    0x19c3d7 as c_int as uint64_t,
    0x1dc3d7 as c_int as uint64_t,
    0x1cbd7 as c_int as uint64_t,
    0x5cbd7 as c_int as uint64_t,
    0x9cbd7 as c_int as uint64_t,
    0xdcbd7 as c_int as uint64_t,
    0x11cbd7 as c_int as uint64_t,
    0x15cbd7 as c_int as uint64_t,
    0x19cbd7 as c_int as uint64_t,
    0x1dcbd7 as c_int as uint64_t,
    0x1d3d7 as c_int as uint64_t,
    0x5d3d7 as c_int as uint64_t,
    0x9d3d7 as c_int as uint64_t,
    0xdd3d7 as c_int as uint64_t,
    0x11d3d7 as c_int as uint64_t,
    0x15d3d7 as c_int as uint64_t,
    0x19d3d7 as c_int as uint64_t,
    0x1dd3d7 as c_int as uint64_t,
    0x1dbd7 as c_int as uint64_t,
    0x5dbd7 as c_int as uint64_t,
    0x9dbd7 as c_int as uint64_t,
    0xddbd7 as c_int as uint64_t,
    0x11dbd7 as c_int as uint64_t,
    0x15dbd7 as c_int as uint64_t,
    0x19dbd7 as c_int as uint64_t,
    0x1ddbd7 as c_int as uint64_t,
    0x1e3d7 as c_int as uint64_t,
    0x5e3d7 as c_int as uint64_t,
    0x9e3d7 as c_int as uint64_t,
    0xde3d7 as c_int as uint64_t,
    0x11e3d7 as c_int as uint64_t,
    0x15e3d7 as c_int as uint64_t,
    0x19e3d7 as c_int as uint64_t,
    0x1de3d7 as c_int as uint64_t,
    0x1ebd7 as c_int as uint64_t,
    0x5ebd7 as c_int as uint64_t,
    0x9ebd7 as c_int as uint64_t,
    0xdebd7 as c_int as uint64_t,
    0x11ebd7 as c_int as uint64_t,
    0x15ebd7 as c_int as uint64_t,
    0x19ebd7 as c_int as uint64_t,
    0x1debd7 as c_int as uint64_t,
    0x1f3d7 as c_int as uint64_t,
    0x5f3d7 as c_int as uint64_t,
    0x9f3d7 as c_int as uint64_t,
    0xdf3d7 as c_int as uint64_t,
    0x11f3d7 as c_int as uint64_t,
    0x15f3d7 as c_int as uint64_t,
    0x19f3d7 as c_int as uint64_t,
    0x1df3d7 as c_int as uint64_t,
    0x1fbd7 as c_int as uint64_t,
    0x5fbd7 as c_int as uint64_t,
    0x9fbd7 as c_int as uint64_t,
    0xdfbd7 as c_int as uint64_t,
    0x11fbd7 as c_int as uint64_t,
    0x15fbd7 as c_int as uint64_t,
    0x19fbd7 as c_int as uint64_t,
    0x1dfbd7 as c_int as uint64_t,
    0x1c3e7 as c_int as uint64_t,
    0x5c3e7 as c_int as uint64_t,
    0x9c3e7 as c_int as uint64_t,
    0xdc3e7 as c_int as uint64_t,
    0x11c3e7 as c_int as uint64_t,
    0x15c3e7 as c_int as uint64_t,
    0x19c3e7 as c_int as uint64_t,
    0x1dc3e7 as c_int as uint64_t,
    0x1cbe7 as c_int as uint64_t,
    0x5cbe7 as c_int as uint64_t,
    0x9cbe7 as c_int as uint64_t,
    0xdcbe7 as c_int as uint64_t,
    0x11cbe7 as c_int as uint64_t,
    0x15cbe7 as c_int as uint64_t,
    0x19cbe7 as c_int as uint64_t,
    0x1dcbe7 as c_int as uint64_t,
    0x1d3e7 as c_int as uint64_t,
    0x5d3e7 as c_int as uint64_t,
    0x9d3e7 as c_int as uint64_t,
    0xdd3e7 as c_int as uint64_t,
    0x11d3e7 as c_int as uint64_t,
    0x15d3e7 as c_int as uint64_t,
    0x19d3e7 as c_int as uint64_t,
    0x1dd3e7 as c_int as uint64_t,
    0x1dbe7 as c_int as uint64_t,
    0x5dbe7 as c_int as uint64_t,
    0x9dbe7 as c_int as uint64_t,
    0xddbe7 as c_int as uint64_t,
    0x11dbe7 as c_int as uint64_t,
    0x15dbe7 as c_int as uint64_t,
    0x19dbe7 as c_int as uint64_t,
    0x1ddbe7 as c_int as uint64_t,
    0x1e3e7 as c_int as uint64_t,
    0x5e3e7 as c_int as uint64_t,
    0x9e3e7 as c_int as uint64_t,
    0xde3e7 as c_int as uint64_t,
    0x11e3e7 as c_int as uint64_t,
    0x15e3e7 as c_int as uint64_t,
    0x19e3e7 as c_int as uint64_t,
    0x1de3e7 as c_int as uint64_t,
    0x1ebe7 as c_int as uint64_t,
    0x5ebe7 as c_int as uint64_t,
    0x9ebe7 as c_int as uint64_t,
    0xdebe7 as c_int as uint64_t,
    0x11ebe7 as c_int as uint64_t,
    0x15ebe7 as c_int as uint64_t,
    0x19ebe7 as c_int as uint64_t,
    0x1debe7 as c_int as uint64_t,
    0x1f3e7 as c_int as uint64_t,
    0x5f3e7 as c_int as uint64_t,
    0x9f3e7 as c_int as uint64_t,
    0xdf3e7 as c_int as uint64_t,
    0x11f3e7 as c_int as uint64_t,
    0x15f3e7 as c_int as uint64_t,
    0x19f3e7 as c_int as uint64_t,
    0x1df3e7 as c_int as uint64_t,
    0x1fbe7 as c_int as uint64_t,
    0x5fbe7 as c_int as uint64_t,
    0x9fbe7 as c_int as uint64_t,
    0xdfbe7 as c_int as uint64_t,
    0x11fbe7 as c_int as uint64_t,
    0x15fbe7 as c_int as uint64_t,
    0x19fbe7 as c_int as uint64_t,
    0x1dfbe7 as c_int as uint64_t,
    0x1c3f7 as c_int as uint64_t,
    0x5c3f7 as c_int as uint64_t,
    0x9c3f7 as c_int as uint64_t,
    0xdc3f7 as c_int as uint64_t,
    0x11c3f7 as c_int as uint64_t,
    0x15c3f7 as c_int as uint64_t,
    0x19c3f7 as c_int as uint64_t,
    0x1dc3f7 as c_int as uint64_t,
    0x1cbf7 as c_int as uint64_t,
    0x5cbf7 as c_int as uint64_t,
    0x9cbf7 as c_int as uint64_t,
    0xdcbf7 as c_int as uint64_t,
    0x11cbf7 as c_int as uint64_t,
    0x15cbf7 as c_int as uint64_t,
    0x19cbf7 as c_int as uint64_t,
    0x1dcbf7 as c_int as uint64_t,
    0x1d3f7 as c_int as uint64_t,
    0x5d3f7 as c_int as uint64_t,
    0x9d3f7 as c_int as uint64_t,
    0xdd3f7 as c_int as uint64_t,
    0x11d3f7 as c_int as uint64_t,
    0x15d3f7 as c_int as uint64_t,
    0x19d3f7 as c_int as uint64_t,
    0x1dd3f7 as c_int as uint64_t,
    0x1dbf7 as c_int as uint64_t,
    0x5dbf7 as c_int as uint64_t,
    0x9dbf7 as c_int as uint64_t,
    0xddbf7 as c_int as uint64_t,
    0x11dbf7 as c_int as uint64_t,
    0x15dbf7 as c_int as uint64_t,
    0x19dbf7 as c_int as uint64_t,
    0x1ddbf7 as c_int as uint64_t,
    0x1e3f7 as c_int as uint64_t,
    0x5e3f7 as c_int as uint64_t,
    0x9e3f7 as c_int as uint64_t,
    0xde3f7 as c_int as uint64_t,
    0x11e3f7 as c_int as uint64_t,
    0x15e3f7 as c_int as uint64_t,
    0x19e3f7 as c_int as uint64_t,
    0x1de3f7 as c_int as uint64_t,
    0x1ebf7 as c_int as uint64_t,
    0x5ebf7 as c_int as uint64_t,
    0x9ebf7 as c_int as uint64_t,
    0xdebf7 as c_int as uint64_t,
    0x11ebf7 as c_int as uint64_t,
    0x15ebf7 as c_int as uint64_t,
    0x19ebf7 as c_int as uint64_t,
    0x1debf7 as c_int as uint64_t,
    0x1f3f7 as c_int as uint64_t,
    0x5f3f7 as c_int as uint64_t,
    0x9f3f7 as c_int as uint64_t,
    0xdf3f7 as c_int as uint64_t,
    0x11f3f7 as c_int as uint64_t,
    0x15f3f7 as c_int as uint64_t,
    0x19f3f7 as c_int as uint64_t,
    0x1df3f7 as c_int as uint64_t,
    0x1fbf7 as c_int as uint64_t,
    0x5fbf7 as c_int as uint64_t,
    0x9fbf7 as c_int as uint64_t,
    0xdfbf7 as c_int as uint64_t,
    0x11fbf7 as c_int as uint64_t,
    0x15fbf7 as c_int as uint64_t,
    0x19fbf7 as c_int as uint64_t,
    0x1dfbf7 as c_int as uint64_t,
    0xe1c387 as c_int as uint64_t,
    0x2e1c387 as c_int as uint64_t,
    0x4e1c387 as c_int as uint64_t,
    0x6e1c387 as c_int as uint64_t,
    0x8e1c387 as c_int as uint64_t,
    0xae1c387 as c_int as uint64_t,
    0xce1c387 as c_int as uint64_t,
    0xee1c387 as c_int as uint64_t,
    0xe5c387 as c_int as uint64_t,
    0x2e5c387 as c_int as uint64_t,
    0x4e5c387 as c_int as uint64_t,
    0x6e5c387 as c_int as uint64_t,
    0x8e5c387 as c_int as uint64_t,
    0xae5c387 as c_int as uint64_t,
    0xce5c387 as c_int as uint64_t,
    0xee5c387 as c_int as uint64_t,
    0xe9c387 as c_int as uint64_t,
    0x2e9c387 as c_int as uint64_t,
    0x4e9c387 as c_int as uint64_t,
    0x6e9c387 as c_int as uint64_t,
    0x8e9c387 as c_int as uint64_t,
    0xae9c387 as c_int as uint64_t,
    0xce9c387 as c_int as uint64_t,
    0xee9c387 as c_int as uint64_t,
    0xedc387 as c_int as uint64_t,
    0x2edc387 as c_int as uint64_t,
    0x4edc387 as c_int as uint64_t,
    0x6edc387 as c_int as uint64_t,
    0x8edc387 as c_int as uint64_t,
    0xaedc387 as c_int as uint64_t,
    0xcedc387 as c_int as uint64_t,
    0xeedc387 as c_int as uint64_t,
    0xf1c387 as c_int as uint64_t,
    0x2f1c387 as c_int as uint64_t,
    0x4f1c387 as c_int as uint64_t,
    0x6f1c387 as c_int as uint64_t,
    0x8f1c387 as c_int as uint64_t,
    0xaf1c387 as c_int as uint64_t,
    0xcf1c387 as c_int as uint64_t,
    0xef1c387 as c_int as uint64_t,
    0xf5c387 as c_int as uint64_t,
    0x2f5c387 as c_int as uint64_t,
    0x4f5c387 as c_int as uint64_t,
    0x6f5c387 as c_int as uint64_t,
    0x8f5c387 as c_int as uint64_t,
    0xaf5c387 as c_int as uint64_t,
    0xcf5c387 as c_int as uint64_t,
    0xef5c387 as c_int as uint64_t,
    0xf9c387 as c_int as uint64_t,
    0x2f9c387 as c_int as uint64_t,
    0x4f9c387 as c_int as uint64_t,
    0x6f9c387 as c_int as uint64_t,
    0x8f9c387 as c_int as uint64_t,
    0xaf9c387 as c_int as uint64_t,
    0xcf9c387 as c_int as uint64_t,
    0xef9c387 as c_int as uint64_t,
    0xfdc387 as c_int as uint64_t,
    0x2fdc387 as c_int as uint64_t,
    0x4fdc387 as c_int as uint64_t,
    0x6fdc387 as c_int as uint64_t,
    0x8fdc387 as c_int as uint64_t,
    0xafdc387 as c_int as uint64_t,
    0xcfdc387 as c_int as uint64_t,
    0xefdc387 as c_int as uint64_t,
    0xe1cb87 as c_int as uint64_t,
    0x2e1cb87 as c_int as uint64_t,
    0x4e1cb87 as c_int as uint64_t,
    0x6e1cb87 as c_int as uint64_t,
    0x8e1cb87 as c_int as uint64_t,
    0xae1cb87 as c_int as uint64_t,
    0xce1cb87 as c_int as uint64_t,
    0xee1cb87 as c_int as uint64_t,
    0xe5cb87 as c_int as uint64_t,
    0x2e5cb87 as c_int as uint64_t,
    0x4e5cb87 as c_int as uint64_t,
    0x6e5cb87 as c_int as uint64_t,
    0x8e5cb87 as c_int as uint64_t,
    0xae5cb87 as c_int as uint64_t,
    0xce5cb87 as c_int as uint64_t,
    0xee5cb87 as c_int as uint64_t,
    0xe9cb87 as c_int as uint64_t,
    0x2e9cb87 as c_int as uint64_t,
    0x4e9cb87 as c_int as uint64_t,
    0x6e9cb87 as c_int as uint64_t,
    0x8e9cb87 as c_int as uint64_t,
    0xae9cb87 as c_int as uint64_t,
    0xce9cb87 as c_int as uint64_t,
    0xee9cb87 as c_int as uint64_t,
    0xedcb87 as c_int as uint64_t,
    0x2edcb87 as c_int as uint64_t,
    0x4edcb87 as c_int as uint64_t,
    0x6edcb87 as c_int as uint64_t,
    0x8edcb87 as c_int as uint64_t,
    0xaedcb87 as c_int as uint64_t,
    0xcedcb87 as c_int as uint64_t,
    0xeedcb87 as c_int as uint64_t,
    0xf1cb87 as c_int as uint64_t,
    0x2f1cb87 as c_int as uint64_t,
    0x4f1cb87 as c_int as uint64_t,
    0x6f1cb87 as c_int as uint64_t,
    0x8f1cb87 as c_int as uint64_t,
    0xaf1cb87 as c_int as uint64_t,
    0xcf1cb87 as c_int as uint64_t,
    0xef1cb87 as c_int as uint64_t,
    0xf5cb87 as c_int as uint64_t,
    0x2f5cb87 as c_int as uint64_t,
    0x4f5cb87 as c_int as uint64_t,
    0x6f5cb87 as c_int as uint64_t,
    0x8f5cb87 as c_int as uint64_t,
    0xaf5cb87 as c_int as uint64_t,
    0xcf5cb87 as c_int as uint64_t,
    0xef5cb87 as c_int as uint64_t,
    0xf9cb87 as c_int as uint64_t,
    0x2f9cb87 as c_int as uint64_t,
    0x4f9cb87 as c_int as uint64_t,
    0x6f9cb87 as c_int as uint64_t,
    0x8f9cb87 as c_int as uint64_t,
];
static mut kZeroRepsDepth: [uint32_t; 704] = [
    0 as c_int as uint32_t,
    4 as c_int as uint32_t,
    8 as c_int as uint32_t,
    7 as c_int as uint32_t,
    7 as c_int as uint32_t,
    7 as c_int as uint32_t,
    7 as c_int as uint32_t,
    7 as c_int as uint32_t,
    7 as c_int as uint32_t,
    7 as c_int as uint32_t,
    7 as c_int as uint32_t,
    11 as c_int as uint32_t,
    14 as c_int as uint32_t,
    14 as c_int as uint32_t,
    14 as c_int as uint32_t,
    14 as c_int as uint32_t,
    14 as c_int as uint32_t,
    14 as c_int as uint32_t,
    14 as c_int as uint32_t,
    14 as c_int as uint32_t,
    14 as c_int as uint32_t,
    14 as c_int as uint32_t,
    14 as c_int as uint32_t,
    14 as c_int as uint32_t,
    14 as c_int as uint32_t,
    14 as c_int as uint32_t,
    14 as c_int as uint32_t,
    14 as c_int as uint32_t,
    14 as c_int as uint32_t,
    14 as c_int as uint32_t,
    14 as c_int as uint32_t,
    14 as c_int as uint32_t,
    14 as c_int as uint32_t,
    14 as c_int as uint32_t,
    14 as c_int as uint32_t,
    14 as c_int as uint32_t,
    14 as c_int as uint32_t,
    14 as c_int as uint32_t,
    14 as c_int as uint32_t,
    14 as c_int as uint32_t,
    14 as c_int as uint32_t,
    14 as c_int as uint32_t,
    14 as c_int as uint32_t,
    14 as c_int as uint32_t,
    14 as c_int as uint32_t,
    14 as c_int as uint32_t,
    14 as c_int as uint32_t,
    14 as c_int as uint32_t,
    14 as c_int as uint32_t,
    14 as c_int as uint32_t,
    14 as c_int as uint32_t,
    14 as c_int as uint32_t,
    14 as c_int as uint32_t,
    14 as c_int as uint32_t,
    14 as c_int as uint32_t,
    14 as c_int as uint32_t,
    14 as c_int as uint32_t,
    14 as c_int as uint32_t,
    14 as c_int as uint32_t,
    14 as c_int as uint32_t,
    14 as c_int as uint32_t,
    14 as c_int as uint32_t,
    14 as c_int as uint32_t,
    14 as c_int as uint32_t,
    14 as c_int as uint32_t,
    14 as c_int as uint32_t,
    14 as c_int as uint32_t,
    14 as c_int as uint32_t,
    14 as c_int as uint32_t,
    14 as c_int as uint32_t,
    14 as c_int as uint32_t,
    14 as c_int as uint32_t,
    14 as c_int as uint32_t,
    14 as c_int as uint32_t,
    14 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    21 as c_int as uint32_t,
    28 as c_int as uint32_t,
    28 as c_int as uint32_t,
    28 as c_int as uint32_t,
    28 as c_int as uint32_t,
    28 as c_int as uint32_t,
    28 as c_int as uint32_t,
    28 as c_int as uint32_t,
    28 as c_int as uint32_t,
    28 as c_int as uint32_t,
    28 as c_int as uint32_t,
    28 as c_int as uint32_t,
    28 as c_int as uint32_t,
    28 as c_int as uint32_t,
    28 as c_int as uint32_t,
    28 as c_int as uint32_t,
    28 as c_int as uint32_t,
    28 as c_int as uint32_t,
    28 as c_int as uint32_t,
    28 as c_int as uint32_t,
    28 as c_int as uint32_t,
    28 as c_int as uint32_t,
    28 as c_int as uint32_t,
    28 as c_int as uint32_t,
    28 as c_int as uint32_t,
    28 as c_int as uint32_t,
    28 as c_int as uint32_t,
    28 as c_int as uint32_t,
    28 as c_int as uint32_t,
    28 as c_int as uint32_t,
    28 as c_int as uint32_t,
    28 as c_int as uint32_t,
    28 as c_int as uint32_t,
    28 as c_int as uint32_t,
    28 as c_int as uint32_t,
    28 as c_int as uint32_t,
    28 as c_int as uint32_t,
    28 as c_int as uint32_t,
    28 as c_int as uint32_t,
    28 as c_int as uint32_t,
    28 as c_int as uint32_t,
    28 as c_int as uint32_t,
    28 as c_int as uint32_t,
    28 as c_int as uint32_t,
    28 as c_int as uint32_t,
    28 as c_int as uint32_t,
    28 as c_int as uint32_t,
    28 as c_int as uint32_t,
    28 as c_int as uint32_t,
    28 as c_int as uint32_t,
    28 as c_int as uint32_t,
    28 as c_int as uint32_t,
    28 as c_int as uint32_t,
    28 as c_int as uint32_t,
    28 as c_int as uint32_t,
    28 as c_int as uint32_t,
    28 as c_int as uint32_t,
    28 as c_int as uint32_t,
    28 as c_int as uint32_t,
    28 as c_int as uint32_t,
    28 as c_int as uint32_t,
    28 as c_int as uint32_t,
    28 as c_int as uint32_t,
    28 as c_int as uint32_t,
    28 as c_int as uint32_t,
    28 as c_int as uint32_t,
    28 as c_int as uint32_t,
    28 as c_int as uint32_t,
    28 as c_int as uint32_t,
    28 as c_int as uint32_t,
    28 as c_int as uint32_t,
    28 as c_int as uint32_t,
    28 as c_int as uint32_t,
    28 as c_int as uint32_t,
    28 as c_int as uint32_t,
    28 as c_int as uint32_t,
    28 as c_int as uint32_t,
    28 as c_int as uint32_t,
    28 as c_int as uint32_t,
    28 as c_int as uint32_t,
    28 as c_int as uint32_t,
    28 as c_int as uint32_t,
    28 as c_int as uint32_t,
    28 as c_int as uint32_t,
    28 as c_int as uint32_t,
    28 as c_int as uint32_t,
    28 as c_int as uint32_t,
    28 as c_int as uint32_t,
    28 as c_int as uint32_t,
    28 as c_int as uint32_t,
    28 as c_int as uint32_t,
    28 as c_int as uint32_t,
    28 as c_int as uint32_t,
    28 as c_int as uint32_t,
    28 as c_int as uint32_t,
    28 as c_int as uint32_t,
    28 as c_int as uint32_t,
    28 as c_int as uint32_t,
    28 as c_int as uint32_t,
    28 as c_int as uint32_t,
    28 as c_int as uint32_t,
    28 as c_int as uint32_t,
    28 as c_int as uint32_t,
    28 as c_int as uint32_t,
    28 as c_int as uint32_t,
    28 as c_int as uint32_t,
    28 as c_int as uint32_t,
    28 as c_int as uint32_t,
    28 as c_int as uint32_t,
    28 as c_int as uint32_t,
    28 as c_int as uint32_t,
    28 as c_int as uint32_t,
    28 as c_int as uint32_t,
    28 as c_int as uint32_t,
    28 as c_int as uint32_t,
    28 as c_int as uint32_t,
    28 as c_int as uint32_t,
    28 as c_int as uint32_t,
];
static mut kNonZeroRepsBits: [uint64_t; 704] = [
    0xb as c_int as uint64_t,
    0x1b as c_int as uint64_t,
    0x2b as c_int as uint64_t,
    0x3b as c_int as uint64_t,
    0x2cb as c_int as uint64_t,
    0x6cb as c_int as uint64_t,
    0xacb as c_int as uint64_t,
    0xecb as c_int as uint64_t,
    0x2db as c_int as uint64_t,
    0x6db as c_int as uint64_t,
    0xadb as c_int as uint64_t,
    0xedb as c_int as uint64_t,
    0x2eb as c_int as uint64_t,
    0x6eb as c_int as uint64_t,
    0xaeb as c_int as uint64_t,
    0xeeb as c_int as uint64_t,
    0x2fb as c_int as uint64_t,
    0x6fb as c_int as uint64_t,
    0xafb as c_int as uint64_t,
    0xefb as c_int as uint64_t,
    0xb2cb as c_int as uint64_t,
    0x1b2cb as c_int as uint64_t,
    0x2b2cb as c_int as uint64_t,
    0x3b2cb as c_int as uint64_t,
    0xb6cb as c_int as uint64_t,
    0x1b6cb as c_int as uint64_t,
    0x2b6cb as c_int as uint64_t,
    0x3b6cb as c_int as uint64_t,
    0xbacb as c_int as uint64_t,
    0x1bacb as c_int as uint64_t,
    0x2bacb as c_int as uint64_t,
    0x3bacb as c_int as uint64_t,
    0xbecb as c_int as uint64_t,
    0x1becb as c_int as uint64_t,
    0x2becb as c_int as uint64_t,
    0x3becb as c_int as uint64_t,
    0xb2db as c_int as uint64_t,
    0x1b2db as c_int as uint64_t,
    0x2b2db as c_int as uint64_t,
    0x3b2db as c_int as uint64_t,
    0xb6db as c_int as uint64_t,
    0x1b6db as c_int as uint64_t,
    0x2b6db as c_int as uint64_t,
    0x3b6db as c_int as uint64_t,
    0xbadb as c_int as uint64_t,
    0x1badb as c_int as uint64_t,
    0x2badb as c_int as uint64_t,
    0x3badb as c_int as uint64_t,
    0xbedb as c_int as uint64_t,
    0x1bedb as c_int as uint64_t,
    0x2bedb as c_int as uint64_t,
    0x3bedb as c_int as uint64_t,
    0xb2eb as c_int as uint64_t,
    0x1b2eb as c_int as uint64_t,
    0x2b2eb as c_int as uint64_t,
    0x3b2eb as c_int as uint64_t,
    0xb6eb as c_int as uint64_t,
    0x1b6eb as c_int as uint64_t,
    0x2b6eb as c_int as uint64_t,
    0x3b6eb as c_int as uint64_t,
    0xbaeb as c_int as uint64_t,
    0x1baeb as c_int as uint64_t,
    0x2baeb as c_int as uint64_t,
    0x3baeb as c_int as uint64_t,
    0xbeeb as c_int as uint64_t,
    0x1beeb as c_int as uint64_t,
    0x2beeb as c_int as uint64_t,
    0x3beeb as c_int as uint64_t,
    0xb2fb as c_int as uint64_t,
    0x1b2fb as c_int as uint64_t,
    0x2b2fb as c_int as uint64_t,
    0x3b2fb as c_int as uint64_t,
    0xb6fb as c_int as uint64_t,
    0x1b6fb as c_int as uint64_t,
    0x2b6fb as c_int as uint64_t,
    0x3b6fb as c_int as uint64_t,
    0xbafb as c_int as uint64_t,
    0x1bafb as c_int as uint64_t,
    0x2bafb as c_int as uint64_t,
    0x3bafb as c_int as uint64_t,
    0xbefb as c_int as uint64_t,
    0x1befb as c_int as uint64_t,
    0x2befb as c_int as uint64_t,
    0x3befb as c_int as uint64_t,
    0x2cb2cb as c_int as uint64_t,
    0x6cb2cb as c_int as uint64_t,
    0xacb2cb as c_int as uint64_t,
    0xecb2cb as c_int as uint64_t,
    0x2db2cb as c_int as uint64_t,
    0x6db2cb as c_int as uint64_t,
    0xadb2cb as c_int as uint64_t,
    0xedb2cb as c_int as uint64_t,
    0x2eb2cb as c_int as uint64_t,
    0x6eb2cb as c_int as uint64_t,
    0xaeb2cb as c_int as uint64_t,
    0xeeb2cb as c_int as uint64_t,
    0x2fb2cb as c_int as uint64_t,
    0x6fb2cb as c_int as uint64_t,
    0xafb2cb as c_int as uint64_t,
    0xefb2cb as c_int as uint64_t,
    0x2cb6cb as c_int as uint64_t,
    0x6cb6cb as c_int as uint64_t,
    0xacb6cb as c_int as uint64_t,
    0xecb6cb as c_int as uint64_t,
    0x2db6cb as c_int as uint64_t,
    0x6db6cb as c_int as uint64_t,
    0xadb6cb as c_int as uint64_t,
    0xedb6cb as c_int as uint64_t,
    0x2eb6cb as c_int as uint64_t,
    0x6eb6cb as c_int as uint64_t,
    0xaeb6cb as c_int as uint64_t,
    0xeeb6cb as c_int as uint64_t,
    0x2fb6cb as c_int as uint64_t,
    0x6fb6cb as c_int as uint64_t,
    0xafb6cb as c_int as uint64_t,
    0xefb6cb as c_int as uint64_t,
    0x2cbacb as c_int as uint64_t,
    0x6cbacb as c_int as uint64_t,
    0xacbacb as c_int as uint64_t,
    0xecbacb as c_int as uint64_t,
    0x2dbacb as c_int as uint64_t,
    0x6dbacb as c_int as uint64_t,
    0xadbacb as c_int as uint64_t,
    0xedbacb as c_int as uint64_t,
    0x2ebacb as c_int as uint64_t,
    0x6ebacb as c_int as uint64_t,
    0xaebacb as c_int as uint64_t,
    0xeebacb as c_int as uint64_t,
    0x2fbacb as c_int as uint64_t,
    0x6fbacb as c_int as uint64_t,
    0xafbacb as c_int as uint64_t,
    0xefbacb as c_int as uint64_t,
    0x2cbecb as c_int as uint64_t,
    0x6cbecb as c_int as uint64_t,
    0xacbecb as c_int as uint64_t,
    0xecbecb as c_int as uint64_t,
    0x2dbecb as c_int as uint64_t,
    0x6dbecb as c_int as uint64_t,
    0xadbecb as c_int as uint64_t,
    0xedbecb as c_int as uint64_t,
    0x2ebecb as c_int as uint64_t,
    0x6ebecb as c_int as uint64_t,
    0xaebecb as c_int as uint64_t,
    0xeebecb as c_int as uint64_t,
    0x2fbecb as c_int as uint64_t,
    0x6fbecb as c_int as uint64_t,
    0xafbecb as c_int as uint64_t,
    0xefbecb as c_int as uint64_t,
    0x2cb2db as c_int as uint64_t,
    0x6cb2db as c_int as uint64_t,
    0xacb2db as c_int as uint64_t,
    0xecb2db as c_int as uint64_t,
    0x2db2db as c_int as uint64_t,
    0x6db2db as c_int as uint64_t,
    0xadb2db as c_int as uint64_t,
    0xedb2db as c_int as uint64_t,
    0x2eb2db as c_int as uint64_t,
    0x6eb2db as c_int as uint64_t,
    0xaeb2db as c_int as uint64_t,
    0xeeb2db as c_int as uint64_t,
    0x2fb2db as c_int as uint64_t,
    0x6fb2db as c_int as uint64_t,
    0xafb2db as c_int as uint64_t,
    0xefb2db as c_int as uint64_t,
    0x2cb6db as c_int as uint64_t,
    0x6cb6db as c_int as uint64_t,
    0xacb6db as c_int as uint64_t,
    0xecb6db as c_int as uint64_t,
    0x2db6db as c_int as uint64_t,
    0x6db6db as c_int as uint64_t,
    0xadb6db as c_int as uint64_t,
    0xedb6db as c_int as uint64_t,
    0x2eb6db as c_int as uint64_t,
    0x6eb6db as c_int as uint64_t,
    0xaeb6db as c_int as uint64_t,
    0xeeb6db as c_int as uint64_t,
    0x2fb6db as c_int as uint64_t,
    0x6fb6db as c_int as uint64_t,
    0xafb6db as c_int as uint64_t,
    0xefb6db as c_int as uint64_t,
    0x2cbadb as c_int as uint64_t,
    0x6cbadb as c_int as uint64_t,
    0xacbadb as c_int as uint64_t,
    0xecbadb as c_int as uint64_t,
    0x2dbadb as c_int as uint64_t,
    0x6dbadb as c_int as uint64_t,
    0xadbadb as c_int as uint64_t,
    0xedbadb as c_int as uint64_t,
    0x2ebadb as c_int as uint64_t,
    0x6ebadb as c_int as uint64_t,
    0xaebadb as c_int as uint64_t,
    0xeebadb as c_int as uint64_t,
    0x2fbadb as c_int as uint64_t,
    0x6fbadb as c_int as uint64_t,
    0xafbadb as c_int as uint64_t,
    0xefbadb as c_int as uint64_t,
    0x2cbedb as c_int as uint64_t,
    0x6cbedb as c_int as uint64_t,
    0xacbedb as c_int as uint64_t,
    0xecbedb as c_int as uint64_t,
    0x2dbedb as c_int as uint64_t,
    0x6dbedb as c_int as uint64_t,
    0xadbedb as c_int as uint64_t,
    0xedbedb as c_int as uint64_t,
    0x2ebedb as c_int as uint64_t,
    0x6ebedb as c_int as uint64_t,
    0xaebedb as c_int as uint64_t,
    0xeebedb as c_int as uint64_t,
    0x2fbedb as c_int as uint64_t,
    0x6fbedb as c_int as uint64_t,
    0xafbedb as c_int as uint64_t,
    0xefbedb as c_int as uint64_t,
    0x2cb2eb as c_int as uint64_t,
    0x6cb2eb as c_int as uint64_t,
    0xacb2eb as c_int as uint64_t,
    0xecb2eb as c_int as uint64_t,
    0x2db2eb as c_int as uint64_t,
    0x6db2eb as c_int as uint64_t,
    0xadb2eb as c_int as uint64_t,
    0xedb2eb as c_int as uint64_t,
    0x2eb2eb as c_int as uint64_t,
    0x6eb2eb as c_int as uint64_t,
    0xaeb2eb as c_int as uint64_t,
    0xeeb2eb as c_int as uint64_t,
    0x2fb2eb as c_int as uint64_t,
    0x6fb2eb as c_int as uint64_t,
    0xafb2eb as c_int as uint64_t,
    0xefb2eb as c_int as uint64_t,
    0x2cb6eb as c_int as uint64_t,
    0x6cb6eb as c_int as uint64_t,
    0xacb6eb as c_int as uint64_t,
    0xecb6eb as c_int as uint64_t,
    0x2db6eb as c_int as uint64_t,
    0x6db6eb as c_int as uint64_t,
    0xadb6eb as c_int as uint64_t,
    0xedb6eb as c_int as uint64_t,
    0x2eb6eb as c_int as uint64_t,
    0x6eb6eb as c_int as uint64_t,
    0xaeb6eb as c_int as uint64_t,
    0xeeb6eb as c_int as uint64_t,
    0x2fb6eb as c_int as uint64_t,
    0x6fb6eb as c_int as uint64_t,
    0xafb6eb as c_int as uint64_t,
    0xefb6eb as c_int as uint64_t,
    0x2cbaeb as c_int as uint64_t,
    0x6cbaeb as c_int as uint64_t,
    0xacbaeb as c_int as uint64_t,
    0xecbaeb as c_int as uint64_t,
    0x2dbaeb as c_int as uint64_t,
    0x6dbaeb as c_int as uint64_t,
    0xadbaeb as c_int as uint64_t,
    0xedbaeb as c_int as uint64_t,
    0x2ebaeb as c_int as uint64_t,
    0x6ebaeb as c_int as uint64_t,
    0xaebaeb as c_int as uint64_t,
    0xeebaeb as c_int as uint64_t,
    0x2fbaeb as c_int as uint64_t,
    0x6fbaeb as c_int as uint64_t,
    0xafbaeb as c_int as uint64_t,
    0xefbaeb as c_int as uint64_t,
    0x2cbeeb as c_int as uint64_t,
    0x6cbeeb as c_int as uint64_t,
    0xacbeeb as c_int as uint64_t,
    0xecbeeb as c_int as uint64_t,
    0x2dbeeb as c_int as uint64_t,
    0x6dbeeb as c_int as uint64_t,
    0xadbeeb as c_int as uint64_t,
    0xedbeeb as c_int as uint64_t,
    0x2ebeeb as c_int as uint64_t,
    0x6ebeeb as c_int as uint64_t,
    0xaebeeb as c_int as uint64_t,
    0xeebeeb as c_int as uint64_t,
    0x2fbeeb as c_int as uint64_t,
    0x6fbeeb as c_int as uint64_t,
    0xafbeeb as c_int as uint64_t,
    0xefbeeb as c_int as uint64_t,
    0x2cb2fb as c_int as uint64_t,
    0x6cb2fb as c_int as uint64_t,
    0xacb2fb as c_int as uint64_t,
    0xecb2fb as c_int as uint64_t,
    0x2db2fb as c_int as uint64_t,
    0x6db2fb as c_int as uint64_t,
    0xadb2fb as c_int as uint64_t,
    0xedb2fb as c_int as uint64_t,
    0x2eb2fb as c_int as uint64_t,
    0x6eb2fb as c_int as uint64_t,
    0xaeb2fb as c_int as uint64_t,
    0xeeb2fb as c_int as uint64_t,
    0x2fb2fb as c_int as uint64_t,
    0x6fb2fb as c_int as uint64_t,
    0xafb2fb as c_int as uint64_t,
    0xefb2fb as c_int as uint64_t,
    0x2cb6fb as c_int as uint64_t,
    0x6cb6fb as c_int as uint64_t,
    0xacb6fb as c_int as uint64_t,
    0xecb6fb as c_int as uint64_t,
    0x2db6fb as c_int as uint64_t,
    0x6db6fb as c_int as uint64_t,
    0xadb6fb as c_int as uint64_t,
    0xedb6fb as c_int as uint64_t,
    0x2eb6fb as c_int as uint64_t,
    0x6eb6fb as c_int as uint64_t,
    0xaeb6fb as c_int as uint64_t,
    0xeeb6fb as c_int as uint64_t,
    0x2fb6fb as c_int as uint64_t,
    0x6fb6fb as c_int as uint64_t,
    0xafb6fb as c_int as uint64_t,
    0xefb6fb as c_int as uint64_t,
    0x2cbafb as c_int as uint64_t,
    0x6cbafb as c_int as uint64_t,
    0xacbafb as c_int as uint64_t,
    0xecbafb as c_int as uint64_t,
    0x2dbafb as c_int as uint64_t,
    0x6dbafb as c_int as uint64_t,
    0xadbafb as c_int as uint64_t,
    0xedbafb as c_int as uint64_t,
    0x2ebafb as c_int as uint64_t,
    0x6ebafb as c_int as uint64_t,
    0xaebafb as c_int as uint64_t,
    0xeebafb as c_int as uint64_t,
    0x2fbafb as c_int as uint64_t,
    0x6fbafb as c_int as uint64_t,
    0xafbafb as c_int as uint64_t,
    0xefbafb as c_int as uint64_t,
    0x2cbefb as c_int as uint64_t,
    0x6cbefb as c_int as uint64_t,
    0xacbefb as c_int as uint64_t,
    0xecbefb as c_int as uint64_t,
    0x2dbefb as c_int as uint64_t,
    0x6dbefb as c_int as uint64_t,
    0xadbefb as c_int as uint64_t,
    0xedbefb as c_int as uint64_t,
    0x2ebefb as c_int as uint64_t,
    0x6ebefb as c_int as uint64_t,
    0xaebefb as c_int as uint64_t,
    0xeebefb as c_int as uint64_t,
    0x2fbefb as c_int as uint64_t,
    0x6fbefb as c_int as uint64_t,
    0xafbefb as c_int as uint64_t,
    0xefbefb as c_int as uint64_t,
    0xb2cb2cb as c_int as uint64_t,
    0x1b2cb2cb as c_int as uint64_t,
    0x2b2cb2cb as c_int as uint64_t,
    0x3b2cb2cb as c_int as uint64_t,
    0xb6cb2cb as c_int as uint64_t,
    0x1b6cb2cb as c_int as uint64_t,
    0x2b6cb2cb as c_int as uint64_t,
    0x3b6cb2cb as c_int as uint64_t,
    0xbacb2cb as c_int as uint64_t,
    0x1bacb2cb as c_int as uint64_t,
    0x2bacb2cb as c_int as uint64_t,
    0x3bacb2cb as c_int as uint64_t,
    0xbecb2cb as c_int as uint64_t,
    0x1becb2cb as c_int as uint64_t,
    0x2becb2cb as c_int as uint64_t,
    0x3becb2cb as c_int as uint64_t,
    0xb2db2cb as c_int as uint64_t,
    0x1b2db2cb as c_int as uint64_t,
    0x2b2db2cb as c_int as uint64_t,
    0x3b2db2cb as c_int as uint64_t,
    0xb6db2cb as c_int as uint64_t,
    0x1b6db2cb as c_int as uint64_t,
    0x2b6db2cb as c_int as uint64_t,
    0x3b6db2cb as c_int as uint64_t,
    0xbadb2cb as c_int as uint64_t,
    0x1badb2cb as c_int as uint64_t,
    0x2badb2cb as c_int as uint64_t,
    0x3badb2cb as c_int as uint64_t,
    0xbedb2cb as c_int as uint64_t,
    0x1bedb2cb as c_int as uint64_t,
    0x2bedb2cb as c_int as uint64_t,
    0x3bedb2cb as c_int as uint64_t,
    0xb2eb2cb as c_int as uint64_t,
    0x1b2eb2cb as c_int as uint64_t,
    0x2b2eb2cb as c_int as uint64_t,
    0x3b2eb2cb as c_int as uint64_t,
    0xb6eb2cb as c_int as uint64_t,
    0x1b6eb2cb as c_int as uint64_t,
    0x2b6eb2cb as c_int as uint64_t,
    0x3b6eb2cb as c_int as uint64_t,
    0xbaeb2cb as c_int as uint64_t,
    0x1baeb2cb as c_int as uint64_t,
    0x2baeb2cb as c_int as uint64_t,
    0x3baeb2cb as c_int as uint64_t,
    0xbeeb2cb as c_int as uint64_t,
    0x1beeb2cb as c_int as uint64_t,
    0x2beeb2cb as c_int as uint64_t,
    0x3beeb2cb as c_int as uint64_t,
    0xb2fb2cb as c_int as uint64_t,
    0x1b2fb2cb as c_int as uint64_t,
    0x2b2fb2cb as c_int as uint64_t,
    0x3b2fb2cb as c_int as uint64_t,
    0xb6fb2cb as c_int as uint64_t,
    0x1b6fb2cb as c_int as uint64_t,
    0x2b6fb2cb as c_int as uint64_t,
    0x3b6fb2cb as c_int as uint64_t,
    0xbafb2cb as c_int as uint64_t,
    0x1bafb2cb as c_int as uint64_t,
    0x2bafb2cb as c_int as uint64_t,
    0x3bafb2cb as c_int as uint64_t,
    0xbefb2cb as c_int as uint64_t,
    0x1befb2cb as c_int as uint64_t,
    0x2befb2cb as c_int as uint64_t,
    0x3befb2cb as c_int as uint64_t,
    0xb2cb6cb as c_int as uint64_t,
    0x1b2cb6cb as c_int as uint64_t,
    0x2b2cb6cb as c_int as uint64_t,
    0x3b2cb6cb as c_int as uint64_t,
    0xb6cb6cb as c_int as uint64_t,
    0x1b6cb6cb as c_int as uint64_t,
    0x2b6cb6cb as c_int as uint64_t,
    0x3b6cb6cb as c_int as uint64_t,
    0xbacb6cb as c_int as uint64_t,
    0x1bacb6cb as c_int as uint64_t,
    0x2bacb6cb as c_int as uint64_t,
    0x3bacb6cb as c_int as uint64_t,
    0xbecb6cb as c_int as uint64_t,
    0x1becb6cb as c_int as uint64_t,
    0x2becb6cb as c_int as uint64_t,
    0x3becb6cb as c_int as uint64_t,
    0xb2db6cb as c_int as uint64_t,
    0x1b2db6cb as c_int as uint64_t,
    0x2b2db6cb as c_int as uint64_t,
    0x3b2db6cb as c_int as uint64_t,
    0xb6db6cb as c_int as uint64_t,
    0x1b6db6cb as c_int as uint64_t,
    0x2b6db6cb as c_int as uint64_t,
    0x3b6db6cb as c_int as uint64_t,
    0xbadb6cb as c_int as uint64_t,
    0x1badb6cb as c_int as uint64_t,
    0x2badb6cb as c_int as uint64_t,
    0x3badb6cb as c_int as uint64_t,
    0xbedb6cb as c_int as uint64_t,
    0x1bedb6cb as c_int as uint64_t,
    0x2bedb6cb as c_int as uint64_t,
    0x3bedb6cb as c_int as uint64_t,
    0xb2eb6cb as c_int as uint64_t,
    0x1b2eb6cb as c_int as uint64_t,
    0x2b2eb6cb as c_int as uint64_t,
    0x3b2eb6cb as c_int as uint64_t,
    0xb6eb6cb as c_int as uint64_t,
    0x1b6eb6cb as c_int as uint64_t,
    0x2b6eb6cb as c_int as uint64_t,
    0x3b6eb6cb as c_int as uint64_t,
    0xbaeb6cb as c_int as uint64_t,
    0x1baeb6cb as c_int as uint64_t,
    0x2baeb6cb as c_int as uint64_t,
    0x3baeb6cb as c_int as uint64_t,
    0xbeeb6cb as c_int as uint64_t,
    0x1beeb6cb as c_int as uint64_t,
    0x2beeb6cb as c_int as uint64_t,
    0x3beeb6cb as c_int as uint64_t,
    0xb2fb6cb as c_int as uint64_t,
    0x1b2fb6cb as c_int as uint64_t,
    0x2b2fb6cb as c_int as uint64_t,
    0x3b2fb6cb as c_int as uint64_t,
    0xb6fb6cb as c_int as uint64_t,
    0x1b6fb6cb as c_int as uint64_t,
    0x2b6fb6cb as c_int as uint64_t,
    0x3b6fb6cb as c_int as uint64_t,
    0xbafb6cb as c_int as uint64_t,
    0x1bafb6cb as c_int as uint64_t,
    0x2bafb6cb as c_int as uint64_t,
    0x3bafb6cb as c_int as uint64_t,
    0xbefb6cb as c_int as uint64_t,
    0x1befb6cb as c_int as uint64_t,
    0x2befb6cb as c_int as uint64_t,
    0x3befb6cb as c_int as uint64_t,
    0xb2cbacb as c_int as uint64_t,
    0x1b2cbacb as c_int as uint64_t,
    0x2b2cbacb as c_int as uint64_t,
    0x3b2cbacb as c_int as uint64_t,
    0xb6cbacb as c_int as uint64_t,
    0x1b6cbacb as c_int as uint64_t,
    0x2b6cbacb as c_int as uint64_t,
    0x3b6cbacb as c_int as uint64_t,
    0xbacbacb as c_int as uint64_t,
    0x1bacbacb as c_int as uint64_t,
    0x2bacbacb as c_int as uint64_t,
    0x3bacbacb as c_int as uint64_t,
    0xbecbacb as c_int as uint64_t,
    0x1becbacb as c_int as uint64_t,
    0x2becbacb as c_int as uint64_t,
    0x3becbacb as c_int as uint64_t,
    0xb2dbacb as c_int as uint64_t,
    0x1b2dbacb as c_int as uint64_t,
    0x2b2dbacb as c_int as uint64_t,
    0x3b2dbacb as c_int as uint64_t,
    0xb6dbacb as c_int as uint64_t,
    0x1b6dbacb as c_int as uint64_t,
    0x2b6dbacb as c_int as uint64_t,
    0x3b6dbacb as c_int as uint64_t,
    0xbadbacb as c_int as uint64_t,
    0x1badbacb as c_int as uint64_t,
    0x2badbacb as c_int as uint64_t,
    0x3badbacb as c_int as uint64_t,
    0xbedbacb as c_int as uint64_t,
    0x1bedbacb as c_int as uint64_t,
    0x2bedbacb as c_int as uint64_t,
    0x3bedbacb as c_int as uint64_t,
    0xb2ebacb as c_int as uint64_t,
    0x1b2ebacb as c_int as uint64_t,
    0x2b2ebacb as c_int as uint64_t,
    0x3b2ebacb as c_int as uint64_t,
    0xb6ebacb as c_int as uint64_t,
    0x1b6ebacb as c_int as uint64_t,
    0x2b6ebacb as c_int as uint64_t,
    0x3b6ebacb as c_int as uint64_t,
    0xbaebacb as c_int as uint64_t,
    0x1baebacb as c_int as uint64_t,
    0x2baebacb as c_int as uint64_t,
    0x3baebacb as c_int as uint64_t,
    0xbeebacb as c_int as uint64_t,
    0x1beebacb as c_int as uint64_t,
    0x2beebacb as c_int as uint64_t,
    0x3beebacb as c_int as uint64_t,
    0xb2fbacb as c_int as uint64_t,
    0x1b2fbacb as c_int as uint64_t,
    0x2b2fbacb as c_int as uint64_t,
    0x3b2fbacb as c_int as uint64_t,
    0xb6fbacb as c_int as uint64_t,
    0x1b6fbacb as c_int as uint64_t,
    0x2b6fbacb as c_int as uint64_t,
    0x3b6fbacb as c_int as uint64_t,
    0xbafbacb as c_int as uint64_t,
    0x1bafbacb as c_int as uint64_t,
    0x2bafbacb as c_int as uint64_t,
    0x3bafbacb as c_int as uint64_t,
    0xbefbacb as c_int as uint64_t,
    0x1befbacb as c_int as uint64_t,
    0x2befbacb as c_int as uint64_t,
    0x3befbacb as c_int as uint64_t,
    0xb2cbecb as c_int as uint64_t,
    0x1b2cbecb as c_int as uint64_t,
    0x2b2cbecb as c_int as uint64_t,
    0x3b2cbecb as c_int as uint64_t,
    0xb6cbecb as c_int as uint64_t,
    0x1b6cbecb as c_int as uint64_t,
    0x2b6cbecb as c_int as uint64_t,
    0x3b6cbecb as c_int as uint64_t,
    0xbacbecb as c_int as uint64_t,
    0x1bacbecb as c_int as uint64_t,
    0x2bacbecb as c_int as uint64_t,
    0x3bacbecb as c_int as uint64_t,
    0xbecbecb as c_int as uint64_t,
    0x1becbecb as c_int as uint64_t,
    0x2becbecb as c_int as uint64_t,
    0x3becbecb as c_int as uint64_t,
    0xb2dbecb as c_int as uint64_t,
    0x1b2dbecb as c_int as uint64_t,
    0x2b2dbecb as c_int as uint64_t,
    0x3b2dbecb as c_int as uint64_t,
    0xb6dbecb as c_int as uint64_t,
    0x1b6dbecb as c_int as uint64_t,
    0x2b6dbecb as c_int as uint64_t,
    0x3b6dbecb as c_int as uint64_t,
    0xbadbecb as c_int as uint64_t,
    0x1badbecb as c_int as uint64_t,
    0x2badbecb as c_int as uint64_t,
    0x3badbecb as c_int as uint64_t,
    0xbedbecb as c_int as uint64_t,
    0x1bedbecb as c_int as uint64_t,
    0x2bedbecb as c_int as uint64_t,
    0x3bedbecb as c_int as uint64_t,
    0xb2ebecb as c_int as uint64_t,
    0x1b2ebecb as c_int as uint64_t,
    0x2b2ebecb as c_int as uint64_t,
    0x3b2ebecb as c_int as uint64_t,
    0xb6ebecb as c_int as uint64_t,
    0x1b6ebecb as c_int as uint64_t,
    0x2b6ebecb as c_int as uint64_t,
    0x3b6ebecb as c_int as uint64_t,
    0xbaebecb as c_int as uint64_t,
    0x1baebecb as c_int as uint64_t,
    0x2baebecb as c_int as uint64_t,
    0x3baebecb as c_int as uint64_t,
    0xbeebecb as c_int as uint64_t,
    0x1beebecb as c_int as uint64_t,
    0x2beebecb as c_int as uint64_t,
    0x3beebecb as c_int as uint64_t,
    0xb2fbecb as c_int as uint64_t,
    0x1b2fbecb as c_int as uint64_t,
    0x2b2fbecb as c_int as uint64_t,
    0x3b2fbecb as c_int as uint64_t,
    0xb6fbecb as c_int as uint64_t,
    0x1b6fbecb as c_int as uint64_t,
    0x2b6fbecb as c_int as uint64_t,
    0x3b6fbecb as c_int as uint64_t,
    0xbafbecb as c_int as uint64_t,
    0x1bafbecb as c_int as uint64_t,
    0x2bafbecb as c_int as uint64_t,
    0x3bafbecb as c_int as uint64_t,
    0xbefbecb as c_int as uint64_t,
    0x1befbecb as c_int as uint64_t,
    0x2befbecb as c_int as uint64_t,
    0x3befbecb as c_int as uint64_t,
    0xb2cb2db as c_int as uint64_t,
    0x1b2cb2db as c_int as uint64_t,
    0x2b2cb2db as c_int as uint64_t,
    0x3b2cb2db as c_int as uint64_t,
    0xb6cb2db as c_int as uint64_t,
    0x1b6cb2db as c_int as uint64_t,
    0x2b6cb2db as c_int as uint64_t,
    0x3b6cb2db as c_int as uint64_t,
    0xbacb2db as c_int as uint64_t,
    0x1bacb2db as c_int as uint64_t,
    0x2bacb2db as c_int as uint64_t,
    0x3bacb2db as c_int as uint64_t,
    0xbecb2db as c_int as uint64_t,
    0x1becb2db as c_int as uint64_t,
    0x2becb2db as c_int as uint64_t,
    0x3becb2db as c_int as uint64_t,
    0xb2db2db as c_int as uint64_t,
    0x1b2db2db as c_int as uint64_t,
    0x2b2db2db as c_int as uint64_t,
    0x3b2db2db as c_int as uint64_t,
    0xb6db2db as c_int as uint64_t,
    0x1b6db2db as c_int as uint64_t,
    0x2b6db2db as c_int as uint64_t,
    0x3b6db2db as c_int as uint64_t,
    0xbadb2db as c_int as uint64_t,
    0x1badb2db as c_int as uint64_t,
    0x2badb2db as c_int as uint64_t,
    0x3badb2db as c_int as uint64_t,
    0xbedb2db as c_int as uint64_t,
    0x1bedb2db as c_int as uint64_t,
    0x2bedb2db as c_int as uint64_t,
    0x3bedb2db as c_int as uint64_t,
    0xb2eb2db as c_int as uint64_t,
    0x1b2eb2db as c_int as uint64_t,
    0x2b2eb2db as c_int as uint64_t,
    0x3b2eb2db as c_int as uint64_t,
    0xb6eb2db as c_int as uint64_t,
    0x1b6eb2db as c_int as uint64_t,
    0x2b6eb2db as c_int as uint64_t,
    0x3b6eb2db as c_int as uint64_t,
    0xbaeb2db as c_int as uint64_t,
    0x1baeb2db as c_int as uint64_t,
    0x2baeb2db as c_int as uint64_t,
    0x3baeb2db as c_int as uint64_t,
    0xbeeb2db as c_int as uint64_t,
    0x1beeb2db as c_int as uint64_t,
    0x2beeb2db as c_int as uint64_t,
    0x3beeb2db as c_int as uint64_t,
    0xb2fb2db as c_int as uint64_t,
    0x1b2fb2db as c_int as uint64_t,
    0x2b2fb2db as c_int as uint64_t,
    0x3b2fb2db as c_int as uint64_t,
    0xb6fb2db as c_int as uint64_t,
    0x1b6fb2db as c_int as uint64_t,
    0x2b6fb2db as c_int as uint64_t,
    0x3b6fb2db as c_int as uint64_t,
    0xbafb2db as c_int as uint64_t,
    0x1bafb2db as c_int as uint64_t,
    0x2bafb2db as c_int as uint64_t,
    0x3bafb2db as c_int as uint64_t,
    0xbefb2db as c_int as uint64_t,
    0x1befb2db as c_int as uint64_t,
    0x2befb2db as c_int as uint64_t,
    0x3befb2db as c_int as uint64_t,
    0xb2cb6db as c_int as uint64_t,
    0x1b2cb6db as c_int as uint64_t,
    0x2b2cb6db as c_int as uint64_t,
    0x3b2cb6db as c_int as uint64_t,
    0xb6cb6db as c_int as uint64_t,
    0x1b6cb6db as c_int as uint64_t,
    0x2b6cb6db as c_int as uint64_t,
    0x3b6cb6db as c_int as uint64_t,
    0xbacb6db as c_int as uint64_t,
    0x1bacb6db as c_int as uint64_t,
    0x2bacb6db as c_int as uint64_t,
    0x3bacb6db as c_int as uint64_t,
    0xbecb6db as c_int as uint64_t,
    0x1becb6db as c_int as uint64_t,
    0x2becb6db as c_int as uint64_t,
    0x3becb6db as c_int as uint64_t,
    0xb2db6db as c_int as uint64_t,
    0x1b2db6db as c_int as uint64_t,
    0x2b2db6db as c_int as uint64_t,
    0x3b2db6db as c_int as uint64_t,
    0xb6db6db as c_int as uint64_t,
    0x1b6db6db as c_int as uint64_t,
    0x2b6db6db as c_int as uint64_t,
    0x3b6db6db as c_int as uint64_t,
    0xbadb6db as c_int as uint64_t,
    0x1badb6db as c_int as uint64_t,
    0x2badb6db as c_int as uint64_t,
    0x3badb6db as c_int as uint64_t,
    0xbedb6db as c_int as uint64_t,
    0x1bedb6db as c_int as uint64_t,
    0x2bedb6db as c_int as uint64_t,
    0x3bedb6db as c_int as uint64_t,
    0xb2eb6db as c_int as uint64_t,
    0x1b2eb6db as c_int as uint64_t,
    0x2b2eb6db as c_int as uint64_t,
    0x3b2eb6db as c_int as uint64_t,
    0xb6eb6db as c_int as uint64_t,
    0x1b6eb6db as c_int as uint64_t,
    0x2b6eb6db as c_int as uint64_t,
    0x3b6eb6db as c_int as uint64_t,
    0xbaeb6db as c_int as uint64_t,
    0x1baeb6db as c_int as uint64_t,
    0x2baeb6db as c_int as uint64_t,
    0x3baeb6db as c_int as uint64_t,
];
static mut kNonZeroRepsDepth: [uint32_t; 704] = [
    6 as c_int as uint32_t,
    6 as c_int as uint32_t,
    6 as c_int as uint32_t,
    6 as c_int as uint32_t,
    12 as c_int as uint32_t,
    12 as c_int as uint32_t,
    12 as c_int as uint32_t,
    12 as c_int as uint32_t,
    12 as c_int as uint32_t,
    12 as c_int as uint32_t,
    12 as c_int as uint32_t,
    12 as c_int as uint32_t,
    12 as c_int as uint32_t,
    12 as c_int as uint32_t,
    12 as c_int as uint32_t,
    12 as c_int as uint32_t,
    12 as c_int as uint32_t,
    12 as c_int as uint32_t,
    12 as c_int as uint32_t,
    12 as c_int as uint32_t,
    18 as c_int as uint32_t,
    18 as c_int as uint32_t,
    18 as c_int as uint32_t,
    18 as c_int as uint32_t,
    18 as c_int as uint32_t,
    18 as c_int as uint32_t,
    18 as c_int as uint32_t,
    18 as c_int as uint32_t,
    18 as c_int as uint32_t,
    18 as c_int as uint32_t,
    18 as c_int as uint32_t,
    18 as c_int as uint32_t,
    18 as c_int as uint32_t,
    18 as c_int as uint32_t,
    18 as c_int as uint32_t,
    18 as c_int as uint32_t,
    18 as c_int as uint32_t,
    18 as c_int as uint32_t,
    18 as c_int as uint32_t,
    18 as c_int as uint32_t,
    18 as c_int as uint32_t,
    18 as c_int as uint32_t,
    18 as c_int as uint32_t,
    18 as c_int as uint32_t,
    18 as c_int as uint32_t,
    18 as c_int as uint32_t,
    18 as c_int as uint32_t,
    18 as c_int as uint32_t,
    18 as c_int as uint32_t,
    18 as c_int as uint32_t,
    18 as c_int as uint32_t,
    18 as c_int as uint32_t,
    18 as c_int as uint32_t,
    18 as c_int as uint32_t,
    18 as c_int as uint32_t,
    18 as c_int as uint32_t,
    18 as c_int as uint32_t,
    18 as c_int as uint32_t,
    18 as c_int as uint32_t,
    18 as c_int as uint32_t,
    18 as c_int as uint32_t,
    18 as c_int as uint32_t,
    18 as c_int as uint32_t,
    18 as c_int as uint32_t,
    18 as c_int as uint32_t,
    18 as c_int as uint32_t,
    18 as c_int as uint32_t,
    18 as c_int as uint32_t,
    18 as c_int as uint32_t,
    18 as c_int as uint32_t,
    18 as c_int as uint32_t,
    18 as c_int as uint32_t,
    18 as c_int as uint32_t,
    18 as c_int as uint32_t,
    18 as c_int as uint32_t,
    18 as c_int as uint32_t,
    18 as c_int as uint32_t,
    18 as c_int as uint32_t,
    18 as c_int as uint32_t,
    18 as c_int as uint32_t,
    18 as c_int as uint32_t,
    18 as c_int as uint32_t,
    18 as c_int as uint32_t,
    18 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    24 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
    30 as c_int as uint32_t,
];
static mut kStaticCommandCodeBits: [uint16_t; 704] = [
    0 as c_int as uint16_t,
    256 as c_int as uint16_t,
    128 as c_int as uint16_t,
    384 as c_int as uint16_t,
    64 as c_int as uint16_t,
    320 as c_int as uint16_t,
    192 as c_int as uint16_t,
    448 as c_int as uint16_t,
    32 as c_int as uint16_t,
    288 as c_int as uint16_t,
    160 as c_int as uint16_t,
    416 as c_int as uint16_t,
    96 as c_int as uint16_t,
    352 as c_int as uint16_t,
    224 as c_int as uint16_t,
    480 as c_int as uint16_t,
    16 as c_int as uint16_t,
    272 as c_int as uint16_t,
    144 as c_int as uint16_t,
    400 as c_int as uint16_t,
    80 as c_int as uint16_t,
    336 as c_int as uint16_t,
    208 as c_int as uint16_t,
    464 as c_int as uint16_t,
    48 as c_int as uint16_t,
    304 as c_int as uint16_t,
    176 as c_int as uint16_t,
    432 as c_int as uint16_t,
    112 as c_int as uint16_t,
    368 as c_int as uint16_t,
    240 as c_int as uint16_t,
    496 as c_int as uint16_t,
    8 as c_int as uint16_t,
    264 as c_int as uint16_t,
    136 as c_int as uint16_t,
    392 as c_int as uint16_t,
    72 as c_int as uint16_t,
    328 as c_int as uint16_t,
    200 as c_int as uint16_t,
    456 as c_int as uint16_t,
    40 as c_int as uint16_t,
    296 as c_int as uint16_t,
    168 as c_int as uint16_t,
    424 as c_int as uint16_t,
    104 as c_int as uint16_t,
    360 as c_int as uint16_t,
    232 as c_int as uint16_t,
    488 as c_int as uint16_t,
    24 as c_int as uint16_t,
    280 as c_int as uint16_t,
    152 as c_int as uint16_t,
    408 as c_int as uint16_t,
    88 as c_int as uint16_t,
    344 as c_int as uint16_t,
    216 as c_int as uint16_t,
    472 as c_int as uint16_t,
    56 as c_int as uint16_t,
    312 as c_int as uint16_t,
    184 as c_int as uint16_t,
    440 as c_int as uint16_t,
    120 as c_int as uint16_t,
    376 as c_int as uint16_t,
    248 as c_int as uint16_t,
    504 as c_int as uint16_t,
    4 as c_int as uint16_t,
    260 as c_int as uint16_t,
    132 as c_int as uint16_t,
    388 as c_int as uint16_t,
    68 as c_int as uint16_t,
    324 as c_int as uint16_t,
    196 as c_int as uint16_t,
    452 as c_int as uint16_t,
    36 as c_int as uint16_t,
    292 as c_int as uint16_t,
    164 as c_int as uint16_t,
    420 as c_int as uint16_t,
    100 as c_int as uint16_t,
    356 as c_int as uint16_t,
    228 as c_int as uint16_t,
    484 as c_int as uint16_t,
    20 as c_int as uint16_t,
    276 as c_int as uint16_t,
    148 as c_int as uint16_t,
    404 as c_int as uint16_t,
    84 as c_int as uint16_t,
    340 as c_int as uint16_t,
    212 as c_int as uint16_t,
    468 as c_int as uint16_t,
    52 as c_int as uint16_t,
    308 as c_int as uint16_t,
    180 as c_int as uint16_t,
    436 as c_int as uint16_t,
    116 as c_int as uint16_t,
    372 as c_int as uint16_t,
    244 as c_int as uint16_t,
    500 as c_int as uint16_t,
    12 as c_int as uint16_t,
    268 as c_int as uint16_t,
    140 as c_int as uint16_t,
    396 as c_int as uint16_t,
    76 as c_int as uint16_t,
    332 as c_int as uint16_t,
    204 as c_int as uint16_t,
    460 as c_int as uint16_t,
    44 as c_int as uint16_t,
    300 as c_int as uint16_t,
    172 as c_int as uint16_t,
    428 as c_int as uint16_t,
    108 as c_int as uint16_t,
    364 as c_int as uint16_t,
    236 as c_int as uint16_t,
    492 as c_int as uint16_t,
    28 as c_int as uint16_t,
    284 as c_int as uint16_t,
    156 as c_int as uint16_t,
    412 as c_int as uint16_t,
    92 as c_int as uint16_t,
    348 as c_int as uint16_t,
    220 as c_int as uint16_t,
    476 as c_int as uint16_t,
    60 as c_int as uint16_t,
    316 as c_int as uint16_t,
    188 as c_int as uint16_t,
    444 as c_int as uint16_t,
    124 as c_int as uint16_t,
    380 as c_int as uint16_t,
    252 as c_int as uint16_t,
    508 as c_int as uint16_t,
    2 as c_int as uint16_t,
    258 as c_int as uint16_t,
    130 as c_int as uint16_t,
    386 as c_int as uint16_t,
    66 as c_int as uint16_t,
    322 as c_int as uint16_t,
    194 as c_int as uint16_t,
    450 as c_int as uint16_t,
    34 as c_int as uint16_t,
    290 as c_int as uint16_t,
    162 as c_int as uint16_t,
    418 as c_int as uint16_t,
    98 as c_int as uint16_t,
    354 as c_int as uint16_t,
    226 as c_int as uint16_t,
    482 as c_int as uint16_t,
    18 as c_int as uint16_t,
    274 as c_int as uint16_t,
    146 as c_int as uint16_t,
    402 as c_int as uint16_t,
    82 as c_int as uint16_t,
    338 as c_int as uint16_t,
    210 as c_int as uint16_t,
    466 as c_int as uint16_t,
    50 as c_int as uint16_t,
    306 as c_int as uint16_t,
    178 as c_int as uint16_t,
    434 as c_int as uint16_t,
    114 as c_int as uint16_t,
    370 as c_int as uint16_t,
    242 as c_int as uint16_t,
    498 as c_int as uint16_t,
    10 as c_int as uint16_t,
    266 as c_int as uint16_t,
    138 as c_int as uint16_t,
    394 as c_int as uint16_t,
    74 as c_int as uint16_t,
    330 as c_int as uint16_t,
    202 as c_int as uint16_t,
    458 as c_int as uint16_t,
    42 as c_int as uint16_t,
    298 as c_int as uint16_t,
    170 as c_int as uint16_t,
    426 as c_int as uint16_t,
    106 as c_int as uint16_t,
    362 as c_int as uint16_t,
    234 as c_int as uint16_t,
    490 as c_int as uint16_t,
    26 as c_int as uint16_t,
    282 as c_int as uint16_t,
    154 as c_int as uint16_t,
    410 as c_int as uint16_t,
    90 as c_int as uint16_t,
    346 as c_int as uint16_t,
    218 as c_int as uint16_t,
    474 as c_int as uint16_t,
    58 as c_int as uint16_t,
    314 as c_int as uint16_t,
    186 as c_int as uint16_t,
    442 as c_int as uint16_t,
    122 as c_int as uint16_t,
    378 as c_int as uint16_t,
    250 as c_int as uint16_t,
    506 as c_int as uint16_t,
    6 as c_int as uint16_t,
    262 as c_int as uint16_t,
    134 as c_int as uint16_t,
    390 as c_int as uint16_t,
    70 as c_int as uint16_t,
    326 as c_int as uint16_t,
    198 as c_int as uint16_t,
    454 as c_int as uint16_t,
    38 as c_int as uint16_t,
    294 as c_int as uint16_t,
    166 as c_int as uint16_t,
    422 as c_int as uint16_t,
    102 as c_int as uint16_t,
    358 as c_int as uint16_t,
    230 as c_int as uint16_t,
    486 as c_int as uint16_t,
    22 as c_int as uint16_t,
    278 as c_int as uint16_t,
    150 as c_int as uint16_t,
    406 as c_int as uint16_t,
    86 as c_int as uint16_t,
    342 as c_int as uint16_t,
    214 as c_int as uint16_t,
    470 as c_int as uint16_t,
    54 as c_int as uint16_t,
    310 as c_int as uint16_t,
    182 as c_int as uint16_t,
    438 as c_int as uint16_t,
    118 as c_int as uint16_t,
    374 as c_int as uint16_t,
    246 as c_int as uint16_t,
    502 as c_int as uint16_t,
    14 as c_int as uint16_t,
    270 as c_int as uint16_t,
    142 as c_int as uint16_t,
    398 as c_int as uint16_t,
    78 as c_int as uint16_t,
    334 as c_int as uint16_t,
    206 as c_int as uint16_t,
    462 as c_int as uint16_t,
    46 as c_int as uint16_t,
    302 as c_int as uint16_t,
    174 as c_int as uint16_t,
    430 as c_int as uint16_t,
    110 as c_int as uint16_t,
    366 as c_int as uint16_t,
    238 as c_int as uint16_t,
    494 as c_int as uint16_t,
    30 as c_int as uint16_t,
    286 as c_int as uint16_t,
    158 as c_int as uint16_t,
    414 as c_int as uint16_t,
    94 as c_int as uint16_t,
    350 as c_int as uint16_t,
    222 as c_int as uint16_t,
    478 as c_int as uint16_t,
    62 as c_int as uint16_t,
    318 as c_int as uint16_t,
    190 as c_int as uint16_t,
    446 as c_int as uint16_t,
    126 as c_int as uint16_t,
    382 as c_int as uint16_t,
    254 as c_int as uint16_t,
    510 as c_int as uint16_t,
    1 as c_int as uint16_t,
    257 as c_int as uint16_t,
    129 as c_int as uint16_t,
    385 as c_int as uint16_t,
    65 as c_int as uint16_t,
    321 as c_int as uint16_t,
    193 as c_int as uint16_t,
    449 as c_int as uint16_t,
    33 as c_int as uint16_t,
    289 as c_int as uint16_t,
    161 as c_int as uint16_t,
    417 as c_int as uint16_t,
    97 as c_int as uint16_t,
    353 as c_int as uint16_t,
    225 as c_int as uint16_t,
    481 as c_int as uint16_t,
    17 as c_int as uint16_t,
    273 as c_int as uint16_t,
    145 as c_int as uint16_t,
    401 as c_int as uint16_t,
    81 as c_int as uint16_t,
    337 as c_int as uint16_t,
    209 as c_int as uint16_t,
    465 as c_int as uint16_t,
    49 as c_int as uint16_t,
    305 as c_int as uint16_t,
    177 as c_int as uint16_t,
    433 as c_int as uint16_t,
    113 as c_int as uint16_t,
    369 as c_int as uint16_t,
    241 as c_int as uint16_t,
    497 as c_int as uint16_t,
    9 as c_int as uint16_t,
    265 as c_int as uint16_t,
    137 as c_int as uint16_t,
    393 as c_int as uint16_t,
    73 as c_int as uint16_t,
    329 as c_int as uint16_t,
    201 as c_int as uint16_t,
    457 as c_int as uint16_t,
    41 as c_int as uint16_t,
    297 as c_int as uint16_t,
    169 as c_int as uint16_t,
    425 as c_int as uint16_t,
    105 as c_int as uint16_t,
    361 as c_int as uint16_t,
    233 as c_int as uint16_t,
    489 as c_int as uint16_t,
    25 as c_int as uint16_t,
    281 as c_int as uint16_t,
    153 as c_int as uint16_t,
    409 as c_int as uint16_t,
    89 as c_int as uint16_t,
    345 as c_int as uint16_t,
    217 as c_int as uint16_t,
    473 as c_int as uint16_t,
    57 as c_int as uint16_t,
    313 as c_int as uint16_t,
    185 as c_int as uint16_t,
    441 as c_int as uint16_t,
    121 as c_int as uint16_t,
    377 as c_int as uint16_t,
    249 as c_int as uint16_t,
    505 as c_int as uint16_t,
    5 as c_int as uint16_t,
    261 as c_int as uint16_t,
    133 as c_int as uint16_t,
    389 as c_int as uint16_t,
    69 as c_int as uint16_t,
    325 as c_int as uint16_t,
    197 as c_int as uint16_t,
    453 as c_int as uint16_t,
    37 as c_int as uint16_t,
    293 as c_int as uint16_t,
    165 as c_int as uint16_t,
    421 as c_int as uint16_t,
    101 as c_int as uint16_t,
    357 as c_int as uint16_t,
    229 as c_int as uint16_t,
    485 as c_int as uint16_t,
    21 as c_int as uint16_t,
    277 as c_int as uint16_t,
    149 as c_int as uint16_t,
    405 as c_int as uint16_t,
    85 as c_int as uint16_t,
    341 as c_int as uint16_t,
    213 as c_int as uint16_t,
    469 as c_int as uint16_t,
    53 as c_int as uint16_t,
    309 as c_int as uint16_t,
    181 as c_int as uint16_t,
    437 as c_int as uint16_t,
    117 as c_int as uint16_t,
    373 as c_int as uint16_t,
    245 as c_int as uint16_t,
    501 as c_int as uint16_t,
    13 as c_int as uint16_t,
    269 as c_int as uint16_t,
    141 as c_int as uint16_t,
    397 as c_int as uint16_t,
    77 as c_int as uint16_t,
    333 as c_int as uint16_t,
    205 as c_int as uint16_t,
    461 as c_int as uint16_t,
    45 as c_int as uint16_t,
    301 as c_int as uint16_t,
    173 as c_int as uint16_t,
    429 as c_int as uint16_t,
    109 as c_int as uint16_t,
    365 as c_int as uint16_t,
    237 as c_int as uint16_t,
    493 as c_int as uint16_t,
    29 as c_int as uint16_t,
    285 as c_int as uint16_t,
    157 as c_int as uint16_t,
    413 as c_int as uint16_t,
    93 as c_int as uint16_t,
    349 as c_int as uint16_t,
    221 as c_int as uint16_t,
    477 as c_int as uint16_t,
    61 as c_int as uint16_t,
    317 as c_int as uint16_t,
    189 as c_int as uint16_t,
    445 as c_int as uint16_t,
    125 as c_int as uint16_t,
    381 as c_int as uint16_t,
    253 as c_int as uint16_t,
    509 as c_int as uint16_t,
    3 as c_int as uint16_t,
    259 as c_int as uint16_t,
    131 as c_int as uint16_t,
    387 as c_int as uint16_t,
    67 as c_int as uint16_t,
    323 as c_int as uint16_t,
    195 as c_int as uint16_t,
    451 as c_int as uint16_t,
    35 as c_int as uint16_t,
    291 as c_int as uint16_t,
    163 as c_int as uint16_t,
    419 as c_int as uint16_t,
    99 as c_int as uint16_t,
    355 as c_int as uint16_t,
    227 as c_int as uint16_t,
    483 as c_int as uint16_t,
    19 as c_int as uint16_t,
    275 as c_int as uint16_t,
    147 as c_int as uint16_t,
    403 as c_int as uint16_t,
    83 as c_int as uint16_t,
    339 as c_int as uint16_t,
    211 as c_int as uint16_t,
    467 as c_int as uint16_t,
    51 as c_int as uint16_t,
    307 as c_int as uint16_t,
    179 as c_int as uint16_t,
    435 as c_int as uint16_t,
    115 as c_int as uint16_t,
    371 as c_int as uint16_t,
    243 as c_int as uint16_t,
    499 as c_int as uint16_t,
    11 as c_int as uint16_t,
    267 as c_int as uint16_t,
    139 as c_int as uint16_t,
    395 as c_int as uint16_t,
    75 as c_int as uint16_t,
    331 as c_int as uint16_t,
    203 as c_int as uint16_t,
    459 as c_int as uint16_t,
    43 as c_int as uint16_t,
    299 as c_int as uint16_t,
    171 as c_int as uint16_t,
    427 as c_int as uint16_t,
    107 as c_int as uint16_t,
    363 as c_int as uint16_t,
    235 as c_int as uint16_t,
    491 as c_int as uint16_t,
    27 as c_int as uint16_t,
    283 as c_int as uint16_t,
    155 as c_int as uint16_t,
    411 as c_int as uint16_t,
    91 as c_int as uint16_t,
    347 as c_int as uint16_t,
    219 as c_int as uint16_t,
    475 as c_int as uint16_t,
    59 as c_int as uint16_t,
    315 as c_int as uint16_t,
    187 as c_int as uint16_t,
    443 as c_int as uint16_t,
    123 as c_int as uint16_t,
    379 as c_int as uint16_t,
    251 as c_int as uint16_t,
    507 as c_int as uint16_t,
    7 as c_int as uint16_t,
    1031 as c_int as uint16_t,
    519 as c_int as uint16_t,
    1543 as c_int as uint16_t,
    263 as c_int as uint16_t,
    1287 as c_int as uint16_t,
    775 as c_int as uint16_t,
    1799 as c_int as uint16_t,
    135 as c_int as uint16_t,
    1159 as c_int as uint16_t,
    647 as c_int as uint16_t,
    1671 as c_int as uint16_t,
    391 as c_int as uint16_t,
    1415 as c_int as uint16_t,
    903 as c_int as uint16_t,
    1927 as c_int as uint16_t,
    71 as c_int as uint16_t,
    1095 as c_int as uint16_t,
    583 as c_int as uint16_t,
    1607 as c_int as uint16_t,
    327 as c_int as uint16_t,
    1351 as c_int as uint16_t,
    839 as c_int as uint16_t,
    1863 as c_int as uint16_t,
    199 as c_int as uint16_t,
    1223 as c_int as uint16_t,
    711 as c_int as uint16_t,
    1735 as c_int as uint16_t,
    455 as c_int as uint16_t,
    1479 as c_int as uint16_t,
    967 as c_int as uint16_t,
    1991 as c_int as uint16_t,
    39 as c_int as uint16_t,
    1063 as c_int as uint16_t,
    551 as c_int as uint16_t,
    1575 as c_int as uint16_t,
    295 as c_int as uint16_t,
    1319 as c_int as uint16_t,
    807 as c_int as uint16_t,
    1831 as c_int as uint16_t,
    167 as c_int as uint16_t,
    1191 as c_int as uint16_t,
    679 as c_int as uint16_t,
    1703 as c_int as uint16_t,
    423 as c_int as uint16_t,
    1447 as c_int as uint16_t,
    935 as c_int as uint16_t,
    1959 as c_int as uint16_t,
    103 as c_int as uint16_t,
    1127 as c_int as uint16_t,
    615 as c_int as uint16_t,
    1639 as c_int as uint16_t,
    359 as c_int as uint16_t,
    1383 as c_int as uint16_t,
    871 as c_int as uint16_t,
    1895 as c_int as uint16_t,
    231 as c_int as uint16_t,
    1255 as c_int as uint16_t,
    743 as c_int as uint16_t,
    1767 as c_int as uint16_t,
    487 as c_int as uint16_t,
    1511 as c_int as uint16_t,
    999 as c_int as uint16_t,
    2023 as c_int as uint16_t,
    23 as c_int as uint16_t,
    1047 as c_int as uint16_t,
    535 as c_int as uint16_t,
    1559 as c_int as uint16_t,
    279 as c_int as uint16_t,
    1303 as c_int as uint16_t,
    791 as c_int as uint16_t,
    1815 as c_int as uint16_t,
    151 as c_int as uint16_t,
    1175 as c_int as uint16_t,
    663 as c_int as uint16_t,
    1687 as c_int as uint16_t,
    407 as c_int as uint16_t,
    1431 as c_int as uint16_t,
    919 as c_int as uint16_t,
    1943 as c_int as uint16_t,
    87 as c_int as uint16_t,
    1111 as c_int as uint16_t,
    599 as c_int as uint16_t,
    1623 as c_int as uint16_t,
    343 as c_int as uint16_t,
    1367 as c_int as uint16_t,
    855 as c_int as uint16_t,
    1879 as c_int as uint16_t,
    215 as c_int as uint16_t,
    1239 as c_int as uint16_t,
    727 as c_int as uint16_t,
    1751 as c_int as uint16_t,
    471 as c_int as uint16_t,
    1495 as c_int as uint16_t,
    983 as c_int as uint16_t,
    2007 as c_int as uint16_t,
    55 as c_int as uint16_t,
    1079 as c_int as uint16_t,
    567 as c_int as uint16_t,
    1591 as c_int as uint16_t,
    311 as c_int as uint16_t,
    1335 as c_int as uint16_t,
    823 as c_int as uint16_t,
    1847 as c_int as uint16_t,
    183 as c_int as uint16_t,
    1207 as c_int as uint16_t,
    695 as c_int as uint16_t,
    1719 as c_int as uint16_t,
    439 as c_int as uint16_t,
    1463 as c_int as uint16_t,
    951 as c_int as uint16_t,
    1975 as c_int as uint16_t,
    119 as c_int as uint16_t,
    1143 as c_int as uint16_t,
    631 as c_int as uint16_t,
    1655 as c_int as uint16_t,
    375 as c_int as uint16_t,
    1399 as c_int as uint16_t,
    887 as c_int as uint16_t,
    1911 as c_int as uint16_t,
    247 as c_int as uint16_t,
    1271 as c_int as uint16_t,
    759 as c_int as uint16_t,
    1783 as c_int as uint16_t,
    503 as c_int as uint16_t,
    1527 as c_int as uint16_t,
    1015 as c_int as uint16_t,
    2039 as c_int as uint16_t,
    15 as c_int as uint16_t,
    1039 as c_int as uint16_t,
    527 as c_int as uint16_t,
    1551 as c_int as uint16_t,
    271 as c_int as uint16_t,
    1295 as c_int as uint16_t,
    783 as c_int as uint16_t,
    1807 as c_int as uint16_t,
    143 as c_int as uint16_t,
    1167 as c_int as uint16_t,
    655 as c_int as uint16_t,
    1679 as c_int as uint16_t,
    399 as c_int as uint16_t,
    1423 as c_int as uint16_t,
    911 as c_int as uint16_t,
    1935 as c_int as uint16_t,
    79 as c_int as uint16_t,
    1103 as c_int as uint16_t,
    591 as c_int as uint16_t,
    1615 as c_int as uint16_t,
    335 as c_int as uint16_t,
    1359 as c_int as uint16_t,
    847 as c_int as uint16_t,
    1871 as c_int as uint16_t,
    207 as c_int as uint16_t,
    1231 as c_int as uint16_t,
    719 as c_int as uint16_t,
    1743 as c_int as uint16_t,
    463 as c_int as uint16_t,
    1487 as c_int as uint16_t,
    975 as c_int as uint16_t,
    1999 as c_int as uint16_t,
    47 as c_int as uint16_t,
    1071 as c_int as uint16_t,
    559 as c_int as uint16_t,
    1583 as c_int as uint16_t,
    303 as c_int as uint16_t,
    1327 as c_int as uint16_t,
    815 as c_int as uint16_t,
    1839 as c_int as uint16_t,
    175 as c_int as uint16_t,
    1199 as c_int as uint16_t,
    687 as c_int as uint16_t,
    1711 as c_int as uint16_t,
    431 as c_int as uint16_t,
    1455 as c_int as uint16_t,
    943 as c_int as uint16_t,
    1967 as c_int as uint16_t,
    111 as c_int as uint16_t,
    1135 as c_int as uint16_t,
    623 as c_int as uint16_t,
    1647 as c_int as uint16_t,
    367 as c_int as uint16_t,
    1391 as c_int as uint16_t,
    879 as c_int as uint16_t,
    1903 as c_int as uint16_t,
    239 as c_int as uint16_t,
    1263 as c_int as uint16_t,
    751 as c_int as uint16_t,
    1775 as c_int as uint16_t,
    495 as c_int as uint16_t,
    1519 as c_int as uint16_t,
    1007 as c_int as uint16_t,
    2031 as c_int as uint16_t,
    31 as c_int as uint16_t,
    1055 as c_int as uint16_t,
    543 as c_int as uint16_t,
    1567 as c_int as uint16_t,
    287 as c_int as uint16_t,
    1311 as c_int as uint16_t,
    799 as c_int as uint16_t,
    1823 as c_int as uint16_t,
    159 as c_int as uint16_t,
    1183 as c_int as uint16_t,
    671 as c_int as uint16_t,
    1695 as c_int as uint16_t,
    415 as c_int as uint16_t,
    1439 as c_int as uint16_t,
    927 as c_int as uint16_t,
    1951 as c_int as uint16_t,
    95 as c_int as uint16_t,
    1119 as c_int as uint16_t,
    607 as c_int as uint16_t,
    1631 as c_int as uint16_t,
    351 as c_int as uint16_t,
    1375 as c_int as uint16_t,
    863 as c_int as uint16_t,
    1887 as c_int as uint16_t,
    223 as c_int as uint16_t,
    1247 as c_int as uint16_t,
    735 as c_int as uint16_t,
    1759 as c_int as uint16_t,
    479 as c_int as uint16_t,
    1503 as c_int as uint16_t,
    991 as c_int as uint16_t,
    2015 as c_int as uint16_t,
    63 as c_int as uint16_t,
    1087 as c_int as uint16_t,
    575 as c_int as uint16_t,
    1599 as c_int as uint16_t,
    319 as c_int as uint16_t,
    1343 as c_int as uint16_t,
    831 as c_int as uint16_t,
    1855 as c_int as uint16_t,
    191 as c_int as uint16_t,
    1215 as c_int as uint16_t,
    703 as c_int as uint16_t,
    1727 as c_int as uint16_t,
    447 as c_int as uint16_t,
    1471 as c_int as uint16_t,
    959 as c_int as uint16_t,
    1983 as c_int as uint16_t,
    127 as c_int as uint16_t,
    1151 as c_int as uint16_t,
    639 as c_int as uint16_t,
    1663 as c_int as uint16_t,
    383 as c_int as uint16_t,
    1407 as c_int as uint16_t,
    895 as c_int as uint16_t,
    1919 as c_int as uint16_t,
    255 as c_int as uint16_t,
    1279 as c_int as uint16_t,
    767 as c_int as uint16_t,
    1791 as c_int as uint16_t,
    511 as c_int as uint16_t,
    1535 as c_int as uint16_t,
    1023 as c_int as uint16_t,
    2047 as c_int as uint16_t,
];
#[inline(always)]
unsafe fn StoreStaticCommandHuffmanTree(
    mut storage_ix: *mut size_t,
    mut storage: *mut uint8_t,
) {
    BrotliWriteBits(
        56 as size_t,
        (0x926244 as c_uint as uint64_t) << 32 as c_int
            | 0x16307003 as uint64_t,
        storage_ix,
        storage,
    );
    BrotliWriteBits(3 as size_t, 0 as uint64_t, storage_ix, storage);
}
static mut kStaticDistanceCodeBits: [uint16_t; 64] = [
    0 as c_int as uint16_t,
    32 as c_int as uint16_t,
    16 as c_int as uint16_t,
    48 as c_int as uint16_t,
    8 as c_int as uint16_t,
    40 as c_int as uint16_t,
    24 as c_int as uint16_t,
    56 as c_int as uint16_t,
    4 as c_int as uint16_t,
    36 as c_int as uint16_t,
    20 as c_int as uint16_t,
    52 as c_int as uint16_t,
    12 as c_int as uint16_t,
    44 as c_int as uint16_t,
    28 as c_int as uint16_t,
    60 as c_int as uint16_t,
    2 as c_int as uint16_t,
    34 as c_int as uint16_t,
    18 as c_int as uint16_t,
    50 as c_int as uint16_t,
    10 as c_int as uint16_t,
    42 as c_int as uint16_t,
    26 as c_int as uint16_t,
    58 as c_int as uint16_t,
    6 as c_int as uint16_t,
    38 as c_int as uint16_t,
    22 as c_int as uint16_t,
    54 as c_int as uint16_t,
    14 as c_int as uint16_t,
    46 as c_int as uint16_t,
    30 as c_int as uint16_t,
    62 as c_int as uint16_t,
    1 as c_int as uint16_t,
    33 as c_int as uint16_t,
    17 as c_int as uint16_t,
    49 as c_int as uint16_t,
    9 as c_int as uint16_t,
    41 as c_int as uint16_t,
    25 as c_int as uint16_t,
    57 as c_int as uint16_t,
    5 as c_int as uint16_t,
    37 as c_int as uint16_t,
    21 as c_int as uint16_t,
    53 as c_int as uint16_t,
    13 as c_int as uint16_t,
    45 as c_int as uint16_t,
    29 as c_int as uint16_t,
    61 as c_int as uint16_t,
    3 as c_int as uint16_t,
    35 as c_int as uint16_t,
    19 as c_int as uint16_t,
    51 as c_int as uint16_t,
    11 as c_int as uint16_t,
    43 as c_int as uint16_t,
    27 as c_int as uint16_t,
    59 as c_int as uint16_t,
    7 as c_int as uint16_t,
    39 as c_int as uint16_t,
    23 as c_int as uint16_t,
    55 as c_int as uint16_t,
    15 as c_int as uint16_t,
    47 as c_int as uint16_t,
    31 as c_int as uint16_t,
    63 as c_int as uint16_t,
];
#[inline(always)]
unsafe fn StoreStaticDistanceHuffmanTree(
    mut storage_ix: *mut size_t,
    mut storage: *mut uint8_t,
) {
    BrotliWriteBits(28 as size_t, 0x369dc03 as uint64_t, storage_ix, storage);
}
unsafe fn BuildAndStoreEntropyCodesLiteral(
    mut m: *mut MemoryManager,
    mut self_0: *mut BlockEncoder,
    mut histograms: *const HistogramLiteral,
    histograms_size: size_t,
    alphabet_size: size_t,
    mut tree: *mut HuffmanTree,
    mut storage_ix: *mut size_t,
    mut storage: *mut uint8_t,
) {
    let histograms_view: &[HistogramLiteral] = unsafe { core::slice::from_raw_parts(histograms, (histograms_size) as usize) };
    let table_size: size_t = histograms_size.wrapping_mul((*self_0).histogram_length_);
    (*self_0).depths_ = if table_size > 0 as size_t {
        BrotliAllocate(
            m,
            table_size.wrapping_mul(::core::mem::size_of::<uint8_t>() as size_t),
        ) as *mut uint8_t
    } else {
        ::core::ptr::null_mut::<uint8_t>()
    };
    (*self_0).bits_ = if table_size > 0 as size_t {
        BrotliAllocate(
            m,
            table_size.wrapping_mul(::core::mem::size_of::<uint16_t>() as size_t),
        ) as *mut uint16_t
    } else {
        ::core::ptr::null_mut::<uint16_t>()
    };
    if 0 as c_int != 0 {
        return;
    }
    let mut i: size_t = 0;
    i = 0 as size_t;
    while i < histograms_size {
        let mut ix: size_t = i.wrapping_mul((*self_0).histogram_length_);
        BuildAndStoreHuffmanTree(
            (&raw const (histograms_view[(i) as usize]).data_ as *const uint32_t)
                .offset(0 as c_int as isize) as *const uint32_t,
            (*self_0).histogram_length_,
            alphabet_size,
            tree,
            (*self_0).depths_.offset(ix as isize) as *mut uint8_t,
            (*self_0).bits_.offset(ix as isize) as *mut uint16_t,
            storage_ix,
            storage,
        );
        i = i.wrapping_add(1);
    }
}
unsafe fn BuildAndStoreEntropyCodesCommand(
    mut m: *mut MemoryManager,
    mut self_0: *mut BlockEncoder,
    mut histograms: *const HistogramCommand,
    histograms_size: size_t,
    alphabet_size: size_t,
    mut tree: *mut HuffmanTree,
    mut storage_ix: *mut size_t,
    mut storage: *mut uint8_t,
) {
    let histograms_view: &[HistogramCommand] = unsafe { core::slice::from_raw_parts(histograms, (histograms_size) as usize) };
    let table_size: size_t = histograms_size.wrapping_mul((*self_0).histogram_length_);
    (*self_0).depths_ = if table_size > 0 as size_t {
        BrotliAllocate(
            m,
            table_size.wrapping_mul(::core::mem::size_of::<uint8_t>() as size_t),
        ) as *mut uint8_t
    } else {
        ::core::ptr::null_mut::<uint8_t>()
    };
    (*self_0).bits_ = if table_size > 0 as size_t {
        BrotliAllocate(
            m,
            table_size.wrapping_mul(::core::mem::size_of::<uint16_t>() as size_t),
        ) as *mut uint16_t
    } else {
        ::core::ptr::null_mut::<uint16_t>()
    };
    if 0 as c_int != 0 {
        return;
    }
    let mut i: size_t = 0;
    i = 0 as size_t;
    while i < histograms_size {
        let mut ix: size_t = i.wrapping_mul((*self_0).histogram_length_);
        BuildAndStoreHuffmanTree(
            (&raw const (histograms_view[(i) as usize]).data_ as *const uint32_t)
                .offset(0 as c_int as isize) as *const uint32_t,
            (*self_0).histogram_length_,
            alphabet_size,
            tree,
            (*self_0).depths_.offset(ix as isize) as *mut uint8_t,
            (*self_0).bits_.offset(ix as isize) as *mut uint16_t,
            storage_ix,
            storage,
        );
        i = i.wrapping_add(1);
    }
}
unsafe fn BuildAndStoreEntropyCodesDistance(
    mut m: *mut MemoryManager,
    mut self_0: *mut BlockEncoder,
    mut histograms: *const HistogramDistance,
    histograms_size: size_t,
    alphabet_size: size_t,
    mut tree: *mut HuffmanTree,
    mut storage_ix: *mut size_t,
    mut storage: *mut uint8_t,
) {
    let histograms_view: &[HistogramDistance] = unsafe { core::slice::from_raw_parts(histograms, (histograms_size) as usize) };
    let table_size: size_t = histograms_size.wrapping_mul((*self_0).histogram_length_);
    (*self_0).depths_ = if table_size > 0 as size_t {
        BrotliAllocate(
            m,
            table_size.wrapping_mul(::core::mem::size_of::<uint8_t>() as size_t),
        ) as *mut uint8_t
    } else {
        ::core::ptr::null_mut::<uint8_t>()
    };
    (*self_0).bits_ = if table_size > 0 as size_t {
        BrotliAllocate(
            m,
            table_size.wrapping_mul(::core::mem::size_of::<uint16_t>() as size_t),
        ) as *mut uint16_t
    } else {
        ::core::ptr::null_mut::<uint16_t>()
    };
    if 0 as c_int != 0 {
        return;
    }
    let mut i: size_t = 0;
    i = 0 as size_t;
    while i < histograms_size {
        let mut ix: size_t = i.wrapping_mul((*self_0).histogram_length_);
        BuildAndStoreHuffmanTree(
            (&raw const (histograms_view[(i) as usize]).data_ as *const uint32_t)
                .offset(0 as c_int as isize) as *const uint32_t,
            (*self_0).histogram_length_,
            alphabet_size,
            tree,
            (*self_0).depths_.offset(ix as isize) as *mut uint8_t,
            (*self_0).bits_.offset(ix as isize) as *mut uint16_t,
            storage_ix,
            storage,
        );
        i = i.wrapping_add(1);
    }
}
extern "C" fn run_static_initializers() { unsafe {
    kSymbolMask = ((1 as uint32_t) << SYMBOL_BITS).wrapping_sub(1 as uint32_t);
} }
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
