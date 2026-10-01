use core::ffi::*;
pub use crate::src::c_inlined_fns::*;
pub use crate::src::ffi::*;
pub use crate::src::c_consts::*;
pub use crate::src::c_structs::*;
pub use crate::src::c_types::*;
extern "C" {
    fn BrotliHistogramCombineLiteral(
        out: *mut HistogramLiteral,
        tmp: *mut HistogramLiteral,
        cluster_size: *mut uint32_t,
        symbols: *mut uint32_t,
        clusters: *mut uint32_t,
        pairs: *mut HistogramPair,
        num_clusters: size_t,
        symbols_size: size_t,
        max_clusters: size_t,
        max_num_pairs: size_t,
    ) -> size_t;
    fn BrotliHistogramBitCostDistanceLiteral(
        histogram: *const HistogramLiteral,
        candidate: *const HistogramLiteral,
        tmp: *mut HistogramLiteral,
    ) -> c_double;
    fn BrotliHistogramCombineCommand(
        out: *mut HistogramCommand,
        tmp: *mut HistogramCommand,
        cluster_size: *mut uint32_t,
        symbols: *mut uint32_t,
        clusters: *mut uint32_t,
        pairs: *mut HistogramPair,
        num_clusters: size_t,
        symbols_size: size_t,
        max_clusters: size_t,
        max_num_pairs: size_t,
    ) -> size_t;
    fn BrotliHistogramBitCostDistanceCommand(
        histogram: *const HistogramCommand,
        candidate: *const HistogramCommand,
        tmp: *mut HistogramCommand,
    ) -> c_double;
    fn BrotliHistogramCombineDistance(
        out: *mut HistogramDistance,
        tmp: *mut HistogramDistance,
        cluster_size: *mut uint32_t,
        symbols: *mut uint32_t,
        clusters: *mut uint32_t,
        pairs: *mut HistogramPair,
        num_clusters: size_t,
        symbols_size: size_t,
        max_clusters: size_t,
        max_num_pairs: size_t,
    ) -> size_t;
    fn BrotliHistogramBitCostDistanceDistance(
        histogram: *const HistogramDistance,
        candidate: *const HistogramDistance,
        tmp: *mut HistogramDistance,
    ) -> c_double;
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct BrotliEncoderDictionary {
    pub words: *const BrotliDictionary,
    pub num_transforms: uint32_t,
    pub cutoffTransformsCount: uint32_t,
    pub cutoffTransforms: uint64_t,
    pub hash_table_words: *const uint16_t,
    pub hash_table_lengths: *const uint8_t,
    pub buckets: *const uint16_t,
    pub dict_words: *const DictWord,
    pub trie: BrotliTrie,
    pub has_words_heavy: c_int,
    pub parent: *const ContextualEncoderDictionary,
    pub hash_table_data_words_: *mut uint16_t,
    pub hash_table_data_lengths_: *mut uint8_t,
    pub buckets_alloc_size_: size_t,
    pub buckets_data_: *mut uint16_t,
    pub dict_words_alloc_size_: size_t,
    pub dict_words_data_: *mut DictWord,
    pub words_instance_: *mut BrotliDictionary,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ContextualEncoderDictionary {
    pub context_based: c_int,
    pub num_dictionaries: uint8_t,
    pub context_map: [uint8_t; 64],
    pub dict: [*const BrotliEncoderDictionary; 64],
    pub num_instances_: size_t,
    pub instance_: BrotliEncoderDictionary,
    pub instances_: *mut BrotliEncoderDictionary,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SharedEncoderDictionary {
    pub magic: uint32_t,
    pub compound: CompoundDictionary,
    pub contextual: ContextualEncoderDictionary,
    pub max_quality: c_int,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct BrotliEncoderParams {
    pub mode: BrotliEncoderMode,
    pub quality: c_int,
    pub lgwin: c_int,
    pub lgblock: c_int,
    pub stream_offset: size_t,
    pub size_hint: size_t,
    pub disable_literal_context_modeling: c_int,
    pub large_window: c_int,
    pub hasher: BrotliHasherParams,
    pub dist: BrotliDistanceParams,
    pub dictionary: SharedEncoderDictionary,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Command {
    pub insert_len_: uint32_t,
    pub copy_len_: uint32_t,
    pub dist_extra_: uint32_t,
    pub cmd_prefix_: uint16_t,
    pub dist_prefix_: uint16_t,
}

#[inline(always)]
unsafe extern "C" fn brotli_max_uint8_t(mut a: uint8_t, mut b: uint8_t) -> uint8_t {
    return (if a as c_int > b as c_int {
        a as c_int
    } else {
        b as c_int
    }) as uint8_t;
}

#[inline(always)]
unsafe extern "C" fn CommandCopyLen(mut self_0: *const Command) -> uint32_t {
    return (*self_0).copy_len_ & 0x1ffffff as uint32_t;
}
static mut kMaxLiteralHistograms: size_t = 100 as size_t;
static mut kMaxCommandHistograms: size_t = 50 as size_t;
static mut kLiteralBlockSwitchCost: c_double = 28.1f64;
static mut kCommandBlockSwitchCost: c_double = 13.5f64;
static mut kDistanceBlockSwitchCost: c_double = 14.6f64;
static mut kLiteralStrideLength: size_t = 70 as size_t;
static mut kCommandStrideLength: size_t = 40 as size_t;
static mut kDistanceStrideLength: size_t = 40 as size_t;
static mut kSymbolsPerLiteralHistogram: size_t = 544 as size_t;
static mut kSymbolsPerCommandHistogram: size_t = 530 as size_t;
static mut kSymbolsPerDistanceHistogram: size_t = 544 as size_t;
static mut kMinLengthForBlockSplitting: size_t = 128 as size_t;
static mut kIterMulForRefining: size_t = 2 as size_t;
static mut kMinItersForRefining: size_t = 100 as size_t;
unsafe extern "C" fn CountLiterals(mut cmds: *const Command, num_commands: size_t) -> size_t {
    let mut total_length: size_t = 0 as size_t;
    let mut i: size_t = 0;
    i = 0 as size_t;
    while i < num_commands {
        total_length = (total_length as c_ulong)
            .wrapping_add((*cmds.offset(i as isize)).insert_len_ as c_ulong)
            as size_t as size_t;
        i = i.wrapping_add(1);
    }
    return total_length;
}
unsafe extern "C" fn CopyLiteralsToByteArray(
    mut cmds: *const Command,
    num_commands: size_t,
    mut data: *const uint8_t,
    offset: size_t,
    mask: size_t,
    mut literals: *mut uint8_t,
) {
    let mut pos: size_t = 0 as size_t;
    let mut from_pos: size_t = offset & mask;
    let mut i: size_t = 0;
    i = 0 as size_t;
    while i < num_commands {
        let mut insert_len: size_t = (*cmds.offset(i as isize)).insert_len_ as size_t;
        if from_pos.wrapping_add(insert_len) > mask {
            let mut head_size: size_t = mask.wrapping_add(1 as size_t).wrapping_sub(from_pos);
            memcpy(
                literals.offset(pos as isize) as *mut c_void,
                data.offset(from_pos as isize) as *const c_void,
                head_size,
            );
            from_pos = 0 as size_t;
            pos = (pos as c_ulong).wrapping_add(head_size as c_ulong)
                as size_t as size_t;
            insert_len = (insert_len as c_ulong)
                .wrapping_sub(head_size as c_ulong) as size_t
                as size_t;
        }
        if insert_len > 0 as size_t {
            memcpy(
                literals.offset(pos as isize) as *mut c_void,
                data.offset(from_pos as isize) as *const c_void,
                insert_len,
            );
            pos = (pos as c_ulong).wrapping_add(insert_len as c_ulong)
                as size_t as size_t;
        }
        from_pos = from_pos
            .wrapping_add(insert_len)
            .wrapping_add(CommandCopyLen(cmds.offset(i as isize) as *const Command) as size_t)
            & mask;
        i = i.wrapping_add(1);
    }
}
#[inline(always)]
unsafe extern "C" fn MyRand(mut seed: *mut uint32_t) -> uint32_t {
    *seed = (*seed as c_uint).wrapping_mul(16807 as c_uint) as uint32_t
        as uint32_t;
    return *seed;
}
#[inline(always)]
unsafe extern "C" fn BitCost(mut count: size_t) -> c_double {
    return if count == 0 as size_t {
        -2.0f64
    } else {
        FastLog2(count)
    };
}
pub const HISTOGRAMS_PER_BATCH: c_int = 64 as c_int;
pub const CLUSTERS_PER_BATCH: c_int = 16 as c_int;
#[no_mangle]
pub unsafe extern "C" fn BrotliInitBlockSplit(mut self_0: *mut BlockSplit) {
    (*self_0).num_types = 0 as size_t;
    (*self_0).num_blocks = 0 as size_t;
    (*self_0).types = ::core::ptr::null_mut::<uint8_t>();
    (*self_0).lengths = ::core::ptr::null_mut::<uint32_t>();
    (*self_0).types_alloc_size = 0 as size_t;
    (*self_0).lengths_alloc_size = 0 as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn BrotliDestroyBlockSplit(
    mut m: *mut MemoryManager,
    mut self_0: *mut BlockSplit,
) {
    BrotliFree(m, (*self_0).types as *mut c_void);
    (*self_0).types = ::core::ptr::null_mut::<uint8_t>();
    BrotliFree(m, (*self_0).lengths as *mut c_void);
    (*self_0).lengths = ::core::ptr::null_mut::<uint32_t>();
}
#[no_mangle]
pub unsafe extern "C" fn BrotliSplitBlock(
    mut m: *mut MemoryManager,
    mut cmds: *const Command,
    num_commands: size_t,
    mut data: *const uint8_t,
    pos: size_t,
    mask: size_t,
    mut params: *const BrotliEncoderParams,
    mut literal_split: *mut BlockSplit,
    mut insert_and_copy_split: *mut BlockSplit,
    mut dist_split: *mut BlockSplit,
) {
    let mut literals_count: size_t = CountLiterals(cmds, num_commands);
    let mut literals: *mut uint8_t = if literals_count > 0 as size_t {
        BrotliAllocate(
            m,
            literals_count.wrapping_mul(::core::mem::size_of::<uint8_t>() as size_t),
        ) as *mut uint8_t
    } else {
        ::core::ptr::null_mut::<uint8_t>()
    };
    if 0 as c_int != 0 || 0 as c_int != 0 {
        return;
    }
    CopyLiteralsToByteArray(cmds, num_commands, data, pos, mask, literals);
    SplitByteVectorLiteral(
        m,
        literals,
        literals_count,
        kSymbolsPerLiteralHistogram,
        kMaxLiteralHistograms,
        kLiteralStrideLength,
        kLiteralBlockSwitchCost,
        params,
        literal_split,
    );
    if 0 as c_int != 0 {
        return;
    }
    BrotliFree(m, literals as *mut c_void);
    literals = ::core::ptr::null_mut::<uint8_t>();
    let mut insert_and_copy_codes: *mut uint16_t = if num_commands > 0 as size_t {
        BrotliAllocate(
            m,
            num_commands.wrapping_mul(::core::mem::size_of::<uint16_t>() as size_t),
        ) as *mut uint16_t
    } else {
        ::core::ptr::null_mut::<uint16_t>()
    };
    let mut i: size_t = 0;
    if 0 as c_int != 0 || 0 as c_int != 0 {
        return;
    }
    i = 0 as size_t;
    while i < num_commands {
        *insert_and_copy_codes.offset(i as isize) = (*cmds.offset(i as isize)).cmd_prefix_;
        i = i.wrapping_add(1);
    }
    SplitByteVectorCommand(
        m,
        insert_and_copy_codes,
        num_commands,
        kSymbolsPerCommandHistogram,
        kMaxCommandHistograms,
        kCommandStrideLength,
        kCommandBlockSwitchCost,
        params,
        insert_and_copy_split,
    );
    if 0 as c_int != 0 {
        return;
    }
    BrotliFree(m, insert_and_copy_codes as *mut c_void);
    insert_and_copy_codes = ::core::ptr::null_mut::<uint16_t>();
    let mut distance_prefixes: *mut uint16_t = if num_commands > 0 as size_t {
        BrotliAllocate(
            m,
            num_commands.wrapping_mul(::core::mem::size_of::<uint16_t>() as size_t),
        ) as *mut uint16_t
    } else {
        ::core::ptr::null_mut::<uint16_t>()
    };
    let mut j: size_t = 0 as size_t;
    let mut i_0: size_t = 0;
    if 0 as c_int != 0 || 0 as c_int != 0 {
        return;
    }
    i_0 = 0 as size_t;
    while i_0 < num_commands {
        let mut cmd: *const Command = cmds.offset(i_0 as isize) as *const Command;
        if CommandCopyLen(cmd) != 0
            && (*cmd).cmd_prefix_ as c_int >= 128 as c_int
        {
            let fresh0 = j;
            j = j.wrapping_add(1);
            *distance_prefixes.offset(fresh0 as isize) = ((*cmd).dist_prefix_ as c_int
                & 0x3ff as c_int)
                as uint16_t;
        }
        i_0 = i_0.wrapping_add(1);
    }
    SplitByteVectorDistance(
        m,
        distance_prefixes,
        j,
        kSymbolsPerDistanceHistogram,
        kMaxCommandHistograms,
        kDistanceStrideLength,
        kDistanceBlockSwitchCost,
        params,
        dist_split,
    );
    if 0 as c_int != 0 {
        return;
    }
    BrotliFree(m, distance_prefixes as *mut c_void);
    distance_prefixes = ::core::ptr::null_mut::<uint16_t>();
}

#[inline(always)]
unsafe extern "C" fn ClearHistogramsLiteral(mut array: *mut HistogramLiteral, mut length: size_t) {
    let mut i: size_t = 0;
    i = 0 as size_t;
    while i < length {
        HistogramClearLiteral(array.offset(i as isize));
        i = i.wrapping_add(1);
    }
}

#[inline(always)]
unsafe extern "C" fn HistogramAddVectorLiteral(
    mut self_0: *mut HistogramLiteral,
    mut p: *const uint8_t,
    mut n: size_t,
) {
    (*self_0).total_count_ = ((*self_0).total_count_ as c_ulong)
        .wrapping_add(n as c_ulong) as size_t as size_t;
    n = (n as c_ulong).wrapping_add(1 as c_ulong) as size_t as size_t;
    loop {
        n = n.wrapping_sub(1);
        if !(n != 0) {
            break;
        }
        let fresh27 = p;
        p = p.offset(1);
        (*self_0).data_[*fresh27 as usize] = (*self_0).data_[*fresh27 as usize].wrapping_add(1);
    }
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
    while i < DATA_SIZE_1 as size_t {
        (*self_0).data_[i as usize] = ((*self_0).data_[i as usize] as c_uint)
            .wrapping_add((*v).data_[i as usize] as c_uint)
            as uint32_t as uint32_t;
        i = i.wrapping_add(1);
    }
}
#[inline(always)]
unsafe extern "C" fn HistogramDataSizeLiteral() -> size_t {
    return DATA_SIZE_1 as size_t;
}

#[inline(always)]
unsafe extern "C" fn ClearHistogramsCommand(mut array: *mut HistogramCommand, mut length: size_t) {
    let mut i: size_t = 0;
    i = 0 as size_t;
    while i < length {
        HistogramClearCommand(array.offset(i as isize));
        i = i.wrapping_add(1);
    }
}

#[inline(always)]
unsafe extern "C" fn HistogramAddVectorCommand(
    mut self_0: *mut HistogramCommand,
    mut p: *const uint16_t,
    mut n: size_t,
) {
    (*self_0).total_count_ = ((*self_0).total_count_ as c_ulong)
        .wrapping_add(n as c_ulong) as size_t as size_t;
    n = (n as c_ulong).wrapping_add(1 as c_ulong) as size_t as size_t;
    loop {
        n = n.wrapping_sub(1);
        if !(n != 0) {
            break;
        }
        let fresh18 = p;
        p = p.offset(1);
        (*self_0).data_[*fresh18 as usize] = (*self_0).data_[*fresh18 as usize].wrapping_add(1);
    }
}

#[inline(always)]
unsafe extern "C" fn ClearHistogramsDistance(
    mut array: *mut HistogramDistance,
    mut length: size_t,
) {
    let mut i: size_t = 0;
    i = 0 as size_t;
    while i < length {
        HistogramClearDistance(array.offset(i as isize));
        i = i.wrapping_add(1);
    }
}

#[inline(always)]
unsafe extern "C" fn HistogramAddVectorDistance(
    mut self_0: *mut HistogramDistance,
    mut p: *const uint16_t,
    mut n: size_t,
) {
    (*self_0).total_count_ = ((*self_0).total_count_ as c_ulong)
        .wrapping_add(n as c_ulong) as size_t as size_t;
    n = (n as c_ulong).wrapping_add(1 as c_ulong) as size_t as size_t;
    loop {
        n = n.wrapping_sub(1);
        if !(n != 0) {
            break;
        }
        let fresh9 = p;
        p = p.offset(1);
        (*self_0).data_[*fresh9 as usize] = (*self_0).data_[*fresh9 as usize].wrapping_add(1);
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
    while i < DATA_SIZE as size_t {
        (*self_0).data_[i as usize] = ((*self_0).data_[i as usize] as c_uint)
            .wrapping_add((*v).data_[i as usize] as c_uint)
            as uint32_t as uint32_t;
        i = i.wrapping_add(1);
    }
}
#[inline(always)]
unsafe extern "C" fn HistogramDataSizeDistance() -> size_t {
    return DATA_SIZE as size_t;
}

pub const DATA_SIZE_1: c_int = BROTLI_NUM_LITERAL_SYMBOLS;

pub const DATA_SIZE: c_int = BROTLI_NUM_HISTOGRAM_DISTANCE_SYMBOLS;

unsafe extern "C" fn InitialEntropyCodesLiteral(
    mut data: *const uint8_t,
    mut length: size_t,
    mut stride: size_t,
    mut num_histograms: size_t,
    mut histograms: *mut HistogramLiteral,
) {
    let mut seed: uint32_t = 7 as uint32_t;
    let mut block_length: size_t = length.wrapping_div(num_histograms);
    let mut i: size_t = 0;
    ClearHistogramsLiteral(histograms, num_histograms);
    i = 0 as size_t;
    while i < num_histograms {
        let mut pos: size_t = length.wrapping_mul(i).wrapping_div(num_histograms);
        if i != 0 as size_t {
            pos = (pos as c_ulong)
                .wrapping_add((MyRand(&raw mut seed) as size_t).wrapping_rem(block_length)
                    as c_ulong) as size_t as size_t;
        }
        if pos.wrapping_add(stride) >= length {
            pos = length.wrapping_sub(stride).wrapping_sub(1 as size_t);
        }
        HistogramAddVectorLiteral(
            histograms.offset(i as isize) as *mut HistogramLiteral,
            data.offset(pos as isize),
            stride,
        );
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn RandomSampleLiteral(
    mut seed: *mut uint32_t,
    mut data: *const uint8_t,
    mut length: size_t,
    mut stride: size_t,
    mut sample: *mut HistogramLiteral,
) {
    let mut pos: size_t = 0 as size_t;
    if stride >= length {
        stride = length;
    } else {
        pos = (MyRand(seed) as size_t)
            .wrapping_rem(length.wrapping_sub(stride).wrapping_add(1 as size_t));
    }
    HistogramAddVectorLiteral(sample, data.offset(pos as isize), stride);
}
unsafe extern "C" fn RefineEntropyCodesLiteral(
    mut data: *const uint8_t,
    mut length: size_t,
    mut stride: size_t,
    mut num_histograms: size_t,
    mut histograms: *mut HistogramLiteral,
    mut tmp: *mut HistogramLiteral,
) {
    let mut iters: size_t = kIterMulForRefining
        .wrapping_mul(length)
        .wrapping_div(stride)
        .wrapping_add(kMinItersForRefining);
    let mut seed: uint32_t = 7 as uint32_t;
    let mut iter: size_t = 0;
    iters = iters
        .wrapping_add(num_histograms)
        .wrapping_sub(1 as size_t)
        .wrapping_div(num_histograms)
        .wrapping_mul(num_histograms);
    iter = 0 as size_t;
    while iter < iters {
        HistogramClearLiteral(tmp);
        RandomSampleLiteral(&raw mut seed, data, length, stride, tmp);
        HistogramAddHistogramLiteral(
            histograms.offset(iter.wrapping_rem(num_histograms) as isize) as *mut HistogramLiteral,
            tmp,
        );
        iter = iter.wrapping_add(1);
    }
}
unsafe extern "C" fn FindBlocksLiteral(
    mut data: *const uint8_t,
    length: size_t,
    block_switch_bitcost: c_double,
    num_histograms: size_t,
    mut histograms: *const HistogramLiteral,
    mut insert_cost: *mut c_double,
    mut cost: *mut c_double,
    mut switch_signal: *mut uint8_t,
    mut block_id: *mut uint8_t,
) -> size_t {
    let alphabet_size: size_t = HistogramDataSizeLiteral() as size_t;
    let bitmap_len: size_t = num_histograms.wrapping_add(7 as size_t) >> 3 as c_int;
    let mut num_blocks: size_t = 1 as size_t;
    let mut byte_ix: size_t = 0;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    if num_histograms <= 1 as size_t {
        i = 0 as size_t;
        while i < length {
            *block_id.offset(i as isize) = 0 as uint8_t;
            i = i.wrapping_add(1);
        }
        return 1 as size_t;
    }
    memset(
        insert_cost as *mut c_void,
        0 as c_int,
        (::core::mem::size_of::<c_double>() as size_t)
            .wrapping_mul(alphabet_size)
            .wrapping_mul(num_histograms),
    );
    i = 0 as size_t;
    while i < num_histograms {
        *insert_cost.offset(i as isize) =
            FastLog2((*histograms.offset(i as isize)).total_count_ as uint32_t as size_t);
        i = i.wrapping_add(1);
    }
    i = alphabet_size;
    while i != 0 as size_t {
        i = i.wrapping_sub(1);
        j = 0 as size_t;
        while j < num_histograms {
            *insert_cost.offset(i.wrapping_mul(num_histograms).wrapping_add(j) as isize) =
                *insert_cost.offset(j as isize)
                    - BitCost((*histograms.offset(j as isize)).data_[i as usize] as size_t);
            j = j.wrapping_add(1);
        }
    }
    memset(
        cost as *mut c_void,
        0 as c_int,
        (::core::mem::size_of::<c_double>() as size_t).wrapping_mul(num_histograms),
    );
    memset(
        switch_signal as *mut c_void,
        0 as c_int,
        (::core::mem::size_of::<uint8_t>() as size_t)
            .wrapping_mul(length)
            .wrapping_mul(bitmap_len),
    );
    byte_ix = 0 as size_t;
    while byte_ix < length {
        let mut ix: size_t = byte_ix.wrapping_mul(bitmap_len);
        let mut symbol: size_t = *data.offset(byte_ix as isize) as size_t;
        let mut insert_cost_ix: size_t = symbol.wrapping_mul(num_histograms);
        let mut min_cost: c_double = 1e99f64;
        let mut block_switch_cost: c_double = block_switch_bitcost;
        static mut prologue_length: size_t = 2000 as size_t;
        static mut multiplier: c_double =
            0.07f64 / 2000 as c_int as c_double;
        let mut k: size_t = 0;
        k = 0 as size_t;
        while k < num_histograms {
            *cost.offset(k as isize) +=
                *insert_cost.offset(insert_cost_ix.wrapping_add(k) as isize);
            if *cost.offset(k as isize) < min_cost {
                min_cost = *cost.offset(k as isize);
                *block_id.offset(byte_ix as isize) = k as uint8_t;
            }
            k = k.wrapping_add(1);
        }
        if byte_ix < prologue_length {
            block_switch_cost *= 0.77f64 + multiplier * byte_ix as c_double;
        }
        k = 0 as size_t;
        while k < num_histograms {
            *cost.offset(k as isize) -= min_cost;
            if *cost.offset(k as isize) >= block_switch_cost {
                let mask: uint8_t = ((1 as c_uint) << (k & 7 as size_t)) as uint8_t;
                *cost.offset(k as isize) = block_switch_cost;
                let ref mut fresh26 =
                    *switch_signal.offset(ix.wrapping_add(k >> 3 as c_int) as isize);
                *fresh26 = (*fresh26 as c_int | mask as c_int) as uint8_t;
            }
            k = k.wrapping_add(1);
        }
        byte_ix = byte_ix.wrapping_add(1);
    }
    byte_ix = length.wrapping_sub(1 as size_t);
    let mut ix_0: size_t = byte_ix.wrapping_mul(bitmap_len);
    let mut cur_id: uint8_t = *block_id.offset(byte_ix as isize);
    while byte_ix > 0 as size_t {
        let mask_0: uint8_t = ((1 as c_uint)
            << (cur_id as c_int & 7 as c_int))
            as uint8_t;
        byte_ix = byte_ix.wrapping_sub(1);
        ix_0 = (ix_0 as c_ulong).wrapping_sub(bitmap_len as c_ulong)
            as size_t as size_t;
        if *switch_signal.offset(
            ix_0.wrapping_add((cur_id as c_int >> 3 as c_int) as size_t)
                as isize,
        ) as c_int
            & mask_0 as c_int
            != 0
        {
            if cur_id as c_int
                != *block_id.offset(byte_ix as isize) as c_int
            {
                cur_id = *block_id.offset(byte_ix as isize);
                num_blocks = num_blocks.wrapping_add(1);
            }
        }
        *block_id.offset(byte_ix as isize) = cur_id;
    }
    return num_blocks;
}
unsafe extern "C" fn RemapBlockIdsLiteral(
    mut block_ids: *mut uint8_t,
    length: size_t,
    mut new_id: *mut uint16_t,
    num_histograms: size_t,
) -> size_t {
    static mut kInvalidId: uint16_t = 256 as uint16_t;
    let mut next_id: uint16_t = 0 as uint16_t;
    let mut i: size_t = 0;
    i = 0 as size_t;
    while i < num_histograms {
        *new_id.offset(i as isize) = kInvalidId;
        i = i.wrapping_add(1);
    }
    i = 0 as size_t;
    while i < length {
        if *new_id.offset(*block_ids.offset(i as isize) as isize) as c_int
            == kInvalidId as c_int
        {
            let fresh25 = next_id;
            next_id = next_id.wrapping_add(1);
            *new_id.offset(*block_ids.offset(i as isize) as isize) = fresh25;
        }
        i = i.wrapping_add(1);
    }
    i = 0 as size_t;
    while i < length {
        *block_ids.offset(i as isize) =
            *new_id.offset(*block_ids.offset(i as isize) as isize) as uint8_t;
        i = i.wrapping_add(1);
    }
    return next_id as size_t;
}
unsafe extern "C" fn BuildBlockHistogramsLiteral(
    mut data: *const uint8_t,
    length: size_t,
    mut block_ids: *const uint8_t,
    num_histograms: size_t,
    mut histograms: *mut HistogramLiteral,
) {
    let mut i: size_t = 0;
    ClearHistogramsLiteral(histograms, num_histograms);
    i = 0 as size_t;
    while i < length {
        HistogramAddLiteral(
            histograms.offset(*block_ids.offset(i as isize) as isize) as *mut HistogramLiteral,
            *data.offset(i as isize) as size_t,
        );
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn ClusterBlocksLiteral(
    mut m: *mut MemoryManager,
    mut data: *const uint8_t,
    length: size_t,
    num_blocks: size_t,
    mut block_ids: *mut uint8_t,
    mut split: *mut BlockSplit,
) {
    let mut histogram_symbols: *mut uint32_t = if num_blocks > 0 as size_t {
        BrotliAllocate(
            m,
            num_blocks.wrapping_mul(::core::mem::size_of::<uint32_t>() as size_t),
        ) as *mut uint32_t
    } else {
        ::core::ptr::null_mut::<uint32_t>()
    };
    let mut u32: *mut uint32_t = if num_blocks
        .wrapping_add((4 as c_int * 64 as c_int) as size_t)
        > 0 as size_t
    {
        BrotliAllocate(
            m,
            num_blocks
                .wrapping_add((4 as c_int * 64 as c_int) as size_t)
                .wrapping_mul(::core::mem::size_of::<uint32_t>() as size_t),
        ) as *mut uint32_t
    } else {
        ::core::ptr::null_mut::<uint32_t>()
    };
    let expected_num_clusters: size_t = (CLUSTERS_PER_BATCH as size_t)
        .wrapping_mul(
            num_blocks
                .wrapping_add(HISTOGRAMS_PER_BATCH as size_t)
                .wrapping_sub(1 as size_t),
        )
        .wrapping_div(HISTOGRAMS_PER_BATCH as size_t);
    let mut all_histograms_size: size_t = 0 as size_t;
    let mut all_histograms_capacity: size_t = expected_num_clusters;
    let mut all_histograms: *mut HistogramLiteral = if all_histograms_capacity > 0 as size_t {
        BrotliAllocate(
            m,
            all_histograms_capacity
                .wrapping_mul(::core::mem::size_of::<HistogramLiteral>() as size_t),
        ) as *mut HistogramLiteral
    } else {
        ::core::ptr::null_mut::<HistogramLiteral>()
    };
    let mut cluster_size_size: size_t = 0 as size_t;
    let mut cluster_size_capacity: size_t = expected_num_clusters;
    let mut cluster_size: *mut uint32_t = if cluster_size_capacity > 0 as size_t {
        BrotliAllocate(
            m,
            cluster_size_capacity.wrapping_mul(::core::mem::size_of::<uint32_t>() as size_t),
        ) as *mut uint32_t
    } else {
        ::core::ptr::null_mut::<uint32_t>()
    };
    let mut num_clusters: size_t = 0 as size_t;
    let mut histograms: *mut HistogramLiteral =
        if brotli_min_size_t(num_blocks, 64 as size_t) > 0 as size_t {
            BrotliAllocate(
                m,
                brotli_min_size_t(num_blocks, 64 as size_t)
                    .wrapping_mul(::core::mem::size_of::<HistogramLiteral>() as size_t),
            ) as *mut HistogramLiteral
        } else {
            ::core::ptr::null_mut::<HistogramLiteral>()
        };
    let mut max_num_pairs: size_t =
        (HISTOGRAMS_PER_BATCH * HISTOGRAMS_PER_BATCH / 2 as c_int) as size_t;
    let mut pairs_capacity: size_t = max_num_pairs.wrapping_add(1 as size_t);
    let mut pairs: *mut HistogramPair = if pairs_capacity > 0 as size_t {
        BrotliAllocate(
            m,
            pairs_capacity.wrapping_mul(::core::mem::size_of::<HistogramPair>() as size_t),
        ) as *mut HistogramPair
    } else {
        ::core::ptr::null_mut::<HistogramPair>()
    };
    let mut pos: size_t = 0 as size_t;
    let mut clusters: *mut uint32_t = ::core::ptr::null_mut::<uint32_t>();
    let mut num_final_clusters: size_t = 0;
    static mut kInvalidIndex: uint32_t = BROTLI_UINT32_MAX;
    let mut new_index: *mut uint32_t = ::core::ptr::null_mut::<uint32_t>();
    let mut i: size_t = 0;
    let sizes: *mut uint32_t = if !u32.is_null() {
        u32.offset((0 as c_int * HISTOGRAMS_PER_BATCH) as isize)
    } else {
        ::core::ptr::null_mut::<uint32_t>()
    };
    let new_clusters: *mut uint32_t = if !u32.is_null() {
        u32.offset((1 as c_int * HISTOGRAMS_PER_BATCH) as isize)
    } else {
        ::core::ptr::null_mut::<uint32_t>()
    };
    let symbols: *mut uint32_t = if !u32.is_null() {
        u32.offset((2 as c_int * HISTOGRAMS_PER_BATCH) as isize)
    } else {
        ::core::ptr::null_mut::<uint32_t>()
    };
    let remap: *mut uint32_t = if !u32.is_null() {
        u32.offset((3 as c_int * HISTOGRAMS_PER_BATCH) as isize)
    } else {
        ::core::ptr::null_mut::<uint32_t>()
    };
    let block_lengths: *mut uint32_t = if !u32.is_null() {
        u32.offset((4 as c_int * HISTOGRAMS_PER_BATCH) as isize)
    } else {
        ::core::ptr::null_mut::<uint32_t>()
    };
    let mut tmp: *mut HistogramLiteral = if 2 as c_int > 0 as c_int {
        BrotliAllocate(
            m,
            (2 as size_t).wrapping_mul(::core::mem::size_of::<HistogramLiteral>() as size_t),
        ) as *mut HistogramLiteral
    } else {
        ::core::ptr::null_mut::<HistogramLiteral>()
    };
    if 0 as c_int != 0
        || 0 as c_int != 0
        || 0 as c_int != 0
        || 0 as c_int != 0
        || 0 as c_int != 0
        || 0 as c_int != 0
        || 0 as c_int != 0
        || 0 as c_int != 0
    {
        return;
    }
    memset(
        u32 as *mut c_void,
        0 as c_int,
        num_blocks
            .wrapping_add((4 as c_int * HISTOGRAMS_PER_BATCH) as size_t)
            .wrapping_mul(::core::mem::size_of::<uint32_t>() as size_t),
    );
    let mut block_idx: size_t = 0 as size_t;
    i = 0 as size_t;
    while i < length {
        let ref mut fresh19 = *block_lengths.offset(block_idx as isize);
        *fresh19 = (*fresh19).wrapping_add(1);
        if i.wrapping_add(1 as size_t) == length
            || *block_ids.offset(i as isize) as c_int
                != *block_ids.offset(i.wrapping_add(1 as size_t) as isize) as c_int
        {
            block_idx = block_idx.wrapping_add(1);
        }
        i = i.wrapping_add(1);
    }
    i = 0 as size_t;
    while i < num_blocks {
        let num_to_combine: size_t =
            brotli_min_size_t(num_blocks.wrapping_sub(i), 64 as size_t) as size_t;
        let mut num_new_clusters: size_t = 0;
        let mut j: size_t = 0;
        j = 0 as size_t;
        while j < num_to_combine {
            let mut k: size_t = 0;
            let mut block_length: size_t =
                *block_lengths.offset(i.wrapping_add(j) as isize) as size_t;
            HistogramClearLiteral(histograms.offset(j as isize) as *mut HistogramLiteral);
            k = 0 as size_t;
            while k < block_length {
                let fresh20 = pos;
                pos = pos.wrapping_add(1);
                HistogramAddLiteral(
                    histograms.offset(j as isize) as *mut HistogramLiteral,
                    *data.offset(fresh20 as isize) as size_t,
                );
                k = k.wrapping_add(1);
            }
            (*histograms.offset(j as isize)).bit_cost_ =
                BrotliPopulationCostLiteral(histograms.offset(j as isize) as *mut HistogramLiteral);
            *new_clusters.offset(j as isize) = j as uint32_t;
            *symbols.offset(j as isize) = j as uint32_t;
            *sizes.offset(j as isize) = 1 as uint32_t;
            j = j.wrapping_add(1);
        }
        num_new_clusters = BrotliHistogramCombineLiteral(
            histograms,
            tmp,
            sizes,
            symbols,
            new_clusters,
            pairs,
            num_to_combine,
            num_to_combine,
            HISTOGRAMS_PER_BATCH as size_t,
            max_num_pairs,
        );
        if all_histograms_capacity < all_histograms_size.wrapping_add(num_new_clusters) {
            let mut _new_size: size_t = if all_histograms_capacity == 0 as size_t {
                all_histograms_size.wrapping_add(num_new_clusters)
            } else {
                all_histograms_capacity
            };
            let mut new_array: *mut HistogramLiteral = ::core::ptr::null_mut::<HistogramLiteral>();
            while _new_size < all_histograms_size.wrapping_add(num_new_clusters) {
                _new_size = (_new_size as c_ulong)
                    .wrapping_mul(2 as c_ulong) as size_t
                    as size_t;
            }
            new_array = if _new_size > 0 as size_t {
                BrotliAllocate(
                    m,
                    _new_size.wrapping_mul(::core::mem::size_of::<HistogramLiteral>() as size_t),
                ) as *mut HistogramLiteral
            } else {
                ::core::ptr::null_mut::<HistogramLiteral>()
            };
            if 0 as c_int == 0
                && 0 as c_int == 0
                && all_histograms_capacity != 0 as size_t
            {
                memcpy(
                    new_array as *mut c_void,
                    all_histograms as *const c_void,
                    all_histograms_capacity
                        .wrapping_mul(::core::mem::size_of::<HistogramLiteral>() as size_t),
                );
            }
            BrotliFree(m, all_histograms as *mut c_void);
            all_histograms = ::core::ptr::null_mut::<HistogramLiteral>();
            all_histograms = new_array;
            all_histograms_capacity = _new_size;
        }
        if cluster_size_capacity < cluster_size_size.wrapping_add(num_new_clusters) {
            let mut _new_size_0: size_t = if cluster_size_capacity == 0 as size_t {
                cluster_size_size.wrapping_add(num_new_clusters)
            } else {
                cluster_size_capacity
            };
            let mut new_array_0: *mut uint32_t = ::core::ptr::null_mut::<uint32_t>();
            while _new_size_0 < cluster_size_size.wrapping_add(num_new_clusters) {
                _new_size_0 = (_new_size_0 as c_ulong)
                    .wrapping_mul(2 as c_ulong) as size_t
                    as size_t;
            }
            new_array_0 = if _new_size_0 > 0 as size_t {
                BrotliAllocate(
                    m,
                    _new_size_0.wrapping_mul(::core::mem::size_of::<uint32_t>() as size_t),
                ) as *mut uint32_t
            } else {
                ::core::ptr::null_mut::<uint32_t>()
            };
            if 0 as c_int == 0
                && 0 as c_int == 0
                && cluster_size_capacity != 0 as size_t
            {
                memcpy(
                    new_array_0 as *mut c_void,
                    cluster_size as *const c_void,
                    cluster_size_capacity
                        .wrapping_mul(::core::mem::size_of::<uint32_t>() as size_t),
                );
            }
            BrotliFree(m, cluster_size as *mut c_void);
            cluster_size = ::core::ptr::null_mut::<uint32_t>();
            cluster_size = new_array_0;
            cluster_size_capacity = _new_size_0;
        }
        if 0 as c_int != 0 {
            return;
        }
        j = 0 as size_t;
        while j < num_new_clusters {
            let fresh21 = all_histograms_size;
            all_histograms_size = all_histograms_size.wrapping_add(1);
            *all_histograms.offset(fresh21 as isize) =
                *histograms.offset(*new_clusters.offset(j as isize) as isize);
            let fresh22 = cluster_size_size;
            cluster_size_size = cluster_size_size.wrapping_add(1);
            *cluster_size.offset(fresh22 as isize) =
                *sizes.offset(*new_clusters.offset(j as isize) as isize);
            *remap.offset(*new_clusters.offset(j as isize) as isize) = j as uint32_t;
            j = j.wrapping_add(1);
        }
        j = 0 as size_t;
        while j < num_to_combine {
            *histogram_symbols.offset(i.wrapping_add(j) as isize) = (num_clusters as uint32_t)
                .wrapping_add(*remap.offset(*symbols.offset(j as isize) as isize));
            j = j.wrapping_add(1);
        }
        num_clusters = (num_clusters as c_ulong)
            .wrapping_add(num_new_clusters as c_ulong) as size_t
            as size_t;
        i = (i as c_ulong).wrapping_add(HISTOGRAMS_PER_BATCH as c_ulong)
            as size_t as size_t;
    }
    BrotliFree(m, histograms as *mut c_void);
    histograms = ::core::ptr::null_mut::<HistogramLiteral>();
    max_num_pairs = brotli_min_size_t(
        (64 as size_t).wrapping_mul(num_clusters),
        num_clusters
            .wrapping_div(2 as size_t)
            .wrapping_mul(num_clusters),
    );
    if pairs_capacity < max_num_pairs.wrapping_add(1 as size_t) {
        BrotliFree(m, pairs as *mut c_void);
        pairs = ::core::ptr::null_mut::<HistogramPair>();
        pairs = if max_num_pairs.wrapping_add(1 as size_t) > 0 as size_t {
            BrotliAllocate(
                m,
                max_num_pairs
                    .wrapping_add(1 as size_t)
                    .wrapping_mul(::core::mem::size_of::<HistogramPair>() as size_t),
            ) as *mut HistogramPair
        } else {
            ::core::ptr::null_mut::<HistogramPair>()
        };
        if 0 as c_int != 0 || 0 as c_int != 0 {
            return;
        }
    }
    clusters = if num_clusters > 0 as size_t {
        BrotliAllocate(
            m,
            num_clusters.wrapping_mul(::core::mem::size_of::<uint32_t>() as size_t),
        ) as *mut uint32_t
    } else {
        ::core::ptr::null_mut::<uint32_t>()
    };
    if 0 as c_int != 0 || 0 as c_int != 0 {
        return;
    }
    i = 0 as size_t;
    while i < num_clusters {
        *clusters.offset(i as isize) = i as uint32_t;
        i = i.wrapping_add(1);
    }
    num_final_clusters = BrotliHistogramCombineLiteral(
        all_histograms,
        tmp,
        cluster_size,
        histogram_symbols,
        clusters,
        pairs,
        num_clusters,
        num_blocks,
        BROTLI_MAX_NUMBER_OF_BLOCK_TYPES as size_t,
        max_num_pairs,
    );
    BrotliFree(m, pairs as *mut c_void);
    pairs = ::core::ptr::null_mut::<HistogramPair>();
    BrotliFree(m, cluster_size as *mut c_void);
    cluster_size = ::core::ptr::null_mut::<uint32_t>();
    new_index = if num_clusters > 0 as size_t {
        BrotliAllocate(
            m,
            num_clusters.wrapping_mul(::core::mem::size_of::<uint32_t>() as size_t),
        ) as *mut uint32_t
    } else {
        ::core::ptr::null_mut::<uint32_t>()
    };
    if 0 as c_int != 0 || 0 as c_int != 0 {
        return;
    }
    i = 0 as size_t;
    while i < num_clusters {
        *new_index.offset(i as isize) = kInvalidIndex;
        i = i.wrapping_add(1);
    }
    pos = 0 as size_t;
    let mut next_index: uint32_t = 0 as uint32_t;
    i = 0 as size_t;
    while i < num_blocks {
        let mut j_0: size_t = 0;
        let mut best_out: uint32_t = 0;
        let mut best_bits: c_double = 0.;
        HistogramClearLiteral(tmp);
        j_0 = 0 as size_t;
        while j_0 < *block_lengths.offset(i as isize) as size_t {
            let fresh23 = pos;
            pos = pos.wrapping_add(1);
            HistogramAddLiteral(tmp, *data.offset(fresh23 as isize) as size_t);
            j_0 = j_0.wrapping_add(1);
        }
        best_out = if i == 0 as size_t {
            *histogram_symbols.offset(0 as c_int as isize)
        } else {
            *histogram_symbols.offset(i.wrapping_sub(1 as size_t) as isize)
        };
        best_bits = BrotliHistogramBitCostDistanceLiteral(
            tmp,
            all_histograms.offset(best_out as isize) as *mut HistogramLiteral,
            tmp.offset(1 as c_int as isize),
        );
        j_0 = 0 as size_t;
        while j_0 < num_final_clusters {
            let cur_bits: c_double = BrotliHistogramBitCostDistanceLiteral(
                tmp,
                all_histograms.offset(*clusters.offset(j_0 as isize) as isize)
                    as *mut HistogramLiteral,
                tmp.offset(1 as c_int as isize),
            ) as c_double;
            if cur_bits < best_bits {
                best_bits = cur_bits;
                best_out = *clusters.offset(j_0 as isize);
            }
            j_0 = j_0.wrapping_add(1);
        }
        *histogram_symbols.offset(i as isize) = best_out;
        if *new_index.offset(best_out as isize) == kInvalidIndex {
            let fresh24 = next_index;
            next_index = next_index.wrapping_add(1);
            *new_index.offset(best_out as isize) = fresh24;
        }
        i = i.wrapping_add(1);
    }
    BrotliFree(m, tmp as *mut c_void);
    tmp = ::core::ptr::null_mut::<HistogramLiteral>();
    BrotliFree(m, clusters as *mut c_void);
    clusters = ::core::ptr::null_mut::<uint32_t>();
    BrotliFree(m, all_histograms as *mut c_void);
    all_histograms = ::core::ptr::null_mut::<HistogramLiteral>();
    if (*split).types_alloc_size < num_blocks {
        let mut _new_size_1: size_t = if (*split).types_alloc_size == 0 as size_t {
            num_blocks
        } else {
            (*split).types_alloc_size
        };
        let mut new_array_1: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        while _new_size_1 < num_blocks {
            _new_size_1 = (_new_size_1 as c_ulong)
                .wrapping_mul(2 as c_ulong) as size_t
                as size_t;
        }
        new_array_1 = if _new_size_1 > 0 as size_t {
            BrotliAllocate(
                m,
                _new_size_1.wrapping_mul(::core::mem::size_of::<uint8_t>() as size_t),
            ) as *mut uint8_t
        } else {
            ::core::ptr::null_mut::<uint8_t>()
        };
        if 0 as c_int == 0
            && 0 as c_int == 0
            && (*split).types_alloc_size != 0 as size_t
        {
            memcpy(
                new_array_1 as *mut c_void,
                (*split).types as *const c_void,
                (*split)
                    .types_alloc_size
                    .wrapping_mul(::core::mem::size_of::<uint8_t>() as size_t),
            );
        }
        BrotliFree(m, (*split).types as *mut c_void);
        (*split).types = ::core::ptr::null_mut::<uint8_t>();
        (*split).types = new_array_1;
        (*split).types_alloc_size = _new_size_1;
    }
    if (*split).lengths_alloc_size < num_blocks {
        let mut _new_size_2: size_t = if (*split).lengths_alloc_size == 0 as size_t {
            num_blocks
        } else {
            (*split).lengths_alloc_size
        };
        let mut new_array_2: *mut uint32_t = ::core::ptr::null_mut::<uint32_t>();
        while _new_size_2 < num_blocks {
            _new_size_2 = (_new_size_2 as c_ulong)
                .wrapping_mul(2 as c_ulong) as size_t
                as size_t;
        }
        new_array_2 = if _new_size_2 > 0 as size_t {
            BrotliAllocate(
                m,
                _new_size_2.wrapping_mul(::core::mem::size_of::<uint32_t>() as size_t),
            ) as *mut uint32_t
        } else {
            ::core::ptr::null_mut::<uint32_t>()
        };
        if 0 as c_int == 0
            && 0 as c_int == 0
            && (*split).lengths_alloc_size != 0 as size_t
        {
            memcpy(
                new_array_2 as *mut c_void,
                (*split).lengths as *const c_void,
                (*split)
                    .lengths_alloc_size
                    .wrapping_mul(::core::mem::size_of::<uint32_t>() as size_t),
            );
        }
        BrotliFree(m, (*split).lengths as *mut c_void);
        (*split).lengths = ::core::ptr::null_mut::<uint32_t>();
        (*split).lengths = new_array_2;
        (*split).lengths_alloc_size = _new_size_2;
    }
    if 0 as c_int != 0 {
        return;
    }
    let mut cur_length: uint32_t = 0 as uint32_t;
    let mut block_idx_0: size_t = 0 as size_t;
    let mut max_type: uint8_t = 0 as uint8_t;
    i = 0 as size_t;
    while i < num_blocks {
        cur_length = (cur_length as c_uint)
            .wrapping_add(*block_lengths.offset(i as isize) as c_uint)
            as uint32_t as uint32_t;
        if i.wrapping_add(1 as size_t) == num_blocks
            || *histogram_symbols.offset(i as isize)
                != *histogram_symbols.offset(i.wrapping_add(1 as size_t) as isize)
        {
            let id: uint8_t =
                *new_index.offset(*histogram_symbols.offset(i as isize) as isize) as uint8_t;
            *(*split).types.offset(block_idx_0 as isize) = id;
            *(*split).lengths.offset(block_idx_0 as isize) = cur_length;
            max_type = brotli_max_uint8_t(max_type, id);
            cur_length = 0 as uint32_t;
            block_idx_0 = block_idx_0.wrapping_add(1);
        }
        i = i.wrapping_add(1);
    }
    (*split).num_blocks = block_idx_0;
    (*split).num_types = (max_type as size_t).wrapping_add(1 as size_t);
    BrotliFree(m, new_index as *mut c_void);
    new_index = ::core::ptr::null_mut::<uint32_t>();
    BrotliFree(m, u32 as *mut c_void);
    u32 = ::core::ptr::null_mut::<uint32_t>();
    BrotliFree(m, histogram_symbols as *mut c_void);
    histogram_symbols = ::core::ptr::null_mut::<uint32_t>();
}
unsafe extern "C" fn SplitByteVectorLiteral(
    mut m: *mut MemoryManager,
    mut data: *const uint8_t,
    length: size_t,
    symbols_per_histogram: size_t,
    max_histograms: size_t,
    sampling_stride_length: size_t,
    block_switch_cost: c_double,
    mut params: *const BrotliEncoderParams,
    mut split: *mut BlockSplit,
) {
    let data_size: size_t = HistogramDataSizeLiteral() as size_t;
    let mut histograms: *mut HistogramLiteral = ::core::ptr::null_mut::<HistogramLiteral>();
    let mut tmp: *mut HistogramLiteral = ::core::ptr::null_mut::<HistogramLiteral>();
    let mut num_histograms: size_t = length
        .wrapping_div(symbols_per_histogram)
        .wrapping_add(1 as size_t);
    if num_histograms > max_histograms {
        num_histograms = max_histograms;
    }
    if length == 0 as size_t {
        (*split).num_types = 1 as size_t;
        return;
    }
    if length < kMinLengthForBlockSplitting {
        if (*split).types_alloc_size < (*split).num_blocks.wrapping_add(1 as size_t) {
            let mut _new_size: size_t = if (*split).types_alloc_size == 0 as size_t {
                (*split).num_blocks.wrapping_add(1 as size_t)
            } else {
                (*split).types_alloc_size
            };
            let mut new_array: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
            while _new_size < (*split).num_blocks.wrapping_add(1 as size_t) {
                _new_size = (_new_size as c_ulong)
                    .wrapping_mul(2 as c_ulong) as size_t
                    as size_t;
            }
            new_array = if _new_size > 0 as size_t {
                BrotliAllocate(
                    m,
                    _new_size.wrapping_mul(::core::mem::size_of::<uint8_t>() as size_t),
                ) as *mut uint8_t
            } else {
                ::core::ptr::null_mut::<uint8_t>()
            };
            if 0 as c_int == 0
                && 0 as c_int == 0
                && (*split).types_alloc_size != 0 as size_t
            {
                memcpy(
                    new_array as *mut c_void,
                    (*split).types as *const c_void,
                    (*split)
                        .types_alloc_size
                        .wrapping_mul(::core::mem::size_of::<uint8_t>() as size_t),
                );
            }
            BrotliFree(m, (*split).types as *mut c_void);
            (*split).types = ::core::ptr::null_mut::<uint8_t>();
            (*split).types = new_array;
            (*split).types_alloc_size = _new_size;
        }
        if (*split).lengths_alloc_size < (*split).num_blocks.wrapping_add(1 as size_t) {
            let mut _new_size_0: size_t = if (*split).lengths_alloc_size == 0 as size_t {
                (*split).num_blocks.wrapping_add(1 as size_t)
            } else {
                (*split).lengths_alloc_size
            };
            let mut new_array_0: *mut uint32_t = ::core::ptr::null_mut::<uint32_t>();
            while _new_size_0 < (*split).num_blocks.wrapping_add(1 as size_t) {
                _new_size_0 = (_new_size_0 as c_ulong)
                    .wrapping_mul(2 as c_ulong) as size_t
                    as size_t;
            }
            new_array_0 = if _new_size_0 > 0 as size_t {
                BrotliAllocate(
                    m,
                    _new_size_0.wrapping_mul(::core::mem::size_of::<uint32_t>() as size_t),
                ) as *mut uint32_t
            } else {
                ::core::ptr::null_mut::<uint32_t>()
            };
            if 0 as c_int == 0
                && 0 as c_int == 0
                && (*split).lengths_alloc_size != 0 as size_t
            {
                memcpy(
                    new_array_0 as *mut c_void,
                    (*split).lengths as *const c_void,
                    (*split)
                        .lengths_alloc_size
                        .wrapping_mul(::core::mem::size_of::<uint32_t>() as size_t),
                );
            }
            BrotliFree(m, (*split).lengths as *mut c_void);
            (*split).lengths = ::core::ptr::null_mut::<uint32_t>();
            (*split).lengths = new_array_0;
            (*split).lengths_alloc_size = _new_size_0;
        }
        if 0 as c_int != 0 {
            return;
        }
        (*split).num_types = 1 as size_t;
        *(*split).types.offset((*split).num_blocks as isize) = 0 as uint8_t;
        *(*split).lengths.offset((*split).num_blocks as isize) = length as uint32_t;
        (*split).num_blocks = (*split).num_blocks.wrapping_add(1);
        return;
    }
    histograms = if num_histograms.wrapping_add(1 as size_t) > 0 as size_t {
        BrotliAllocate(
            m,
            num_histograms
                .wrapping_add(1 as size_t)
                .wrapping_mul(::core::mem::size_of::<HistogramLiteral>() as size_t),
        ) as *mut HistogramLiteral
    } else {
        ::core::ptr::null_mut::<HistogramLiteral>()
    };
    tmp = histograms.offset(num_histograms as isize);
    if 0 as c_int != 0 || 0 as c_int != 0 {
        return;
    }
    InitialEntropyCodesLiteral(
        data,
        length,
        sampling_stride_length,
        num_histograms,
        histograms,
    );
    RefineEntropyCodesLiteral(
        data,
        length,
        sampling_stride_length,
        num_histograms,
        histograms,
        tmp,
    );
    let mut block_ids: *mut uint8_t = if length > 0 as size_t {
        BrotliAllocate(
            m,
            length.wrapping_mul(::core::mem::size_of::<uint8_t>() as size_t),
        ) as *mut uint8_t
    } else {
        ::core::ptr::null_mut::<uint8_t>()
    };
    let mut num_blocks: size_t = 0 as size_t;
    let bitmaplen: size_t = num_histograms.wrapping_add(7 as size_t) >> 3 as c_int;
    let mut insert_cost: *mut c_double =
        if data_size.wrapping_mul(num_histograms) > 0 as size_t {
            BrotliAllocate(
                m,
                data_size
                    .wrapping_mul(num_histograms)
                    .wrapping_mul(::core::mem::size_of::<c_double>() as size_t),
            ) as *mut c_double
        } else {
            ::core::ptr::null_mut::<c_double>()
        };
    let mut cost: *mut c_double = if num_histograms > 0 as size_t {
        BrotliAllocate(
            m,
            num_histograms.wrapping_mul(::core::mem::size_of::<c_double>() as size_t),
        ) as *mut c_double
    } else {
        ::core::ptr::null_mut::<c_double>()
    };
    let mut switch_signal: *mut uint8_t = if length.wrapping_mul(bitmaplen) > 0 as size_t {
        BrotliAllocate(
            m,
            length
                .wrapping_mul(bitmaplen)
                .wrapping_mul(::core::mem::size_of::<uint8_t>() as size_t),
        ) as *mut uint8_t
    } else {
        ::core::ptr::null_mut::<uint8_t>()
    };
    let mut new_id: *mut uint16_t = if num_histograms > 0 as size_t {
        BrotliAllocate(
            m,
            num_histograms.wrapping_mul(::core::mem::size_of::<uint16_t>() as size_t),
        ) as *mut uint16_t
    } else {
        ::core::ptr::null_mut::<uint16_t>()
    };
    let iters: size_t = (if (*params).quality < HQ_ZOPFLIFICATION_QUALITY {
        3 as c_int
    } else {
        10 as c_int
    }) as size_t;
    let mut i: size_t = 0;
    if 0 as c_int != 0
        || 0 as c_int != 0
        || 0 as c_int != 0
        || 0 as c_int != 0
        || 0 as c_int != 0
        || 0 as c_int != 0
    {
        return;
    }
    i = 0 as size_t;
    while i < iters {
        num_blocks = FindBlocksLiteral(
            data,
            length,
            block_switch_cost,
            num_histograms,
            histograms,
            insert_cost,
            cost,
            switch_signal,
            block_ids,
        );
        num_histograms = RemapBlockIdsLiteral(block_ids, length, new_id, num_histograms);
        BuildBlockHistogramsLiteral(data, length, block_ids, num_histograms, histograms);
        i = i.wrapping_add(1);
    }
    BrotliFree(m, insert_cost as *mut c_void);
    insert_cost = ::core::ptr::null_mut::<c_double>();
    BrotliFree(m, cost as *mut c_void);
    cost = ::core::ptr::null_mut::<c_double>();
    BrotliFree(m, switch_signal as *mut c_void);
    switch_signal = ::core::ptr::null_mut::<uint8_t>();
    BrotliFree(m, new_id as *mut c_void);
    new_id = ::core::ptr::null_mut::<uint16_t>();
    BrotliFree(m, histograms as *mut c_void);
    histograms = ::core::ptr::null_mut::<HistogramLiteral>();
    ClusterBlocksLiteral(m, data, length, num_blocks, block_ids, split);
    if 0 as c_int != 0 {
        return;
    }
    BrotliFree(m, block_ids as *mut c_void);
    block_ids = ::core::ptr::null_mut::<uint8_t>();
}
unsafe extern "C" fn InitialEntropyCodesCommand(
    mut data: *const uint16_t,
    mut length: size_t,
    mut stride: size_t,
    mut num_histograms: size_t,
    mut histograms: *mut HistogramCommand,
) {
    let mut seed: uint32_t = 7 as uint32_t;
    let mut block_length: size_t = length.wrapping_div(num_histograms);
    let mut i: size_t = 0;
    ClearHistogramsCommand(histograms, num_histograms);
    i = 0 as size_t;
    while i < num_histograms {
        let mut pos: size_t = length.wrapping_mul(i).wrapping_div(num_histograms);
        if i != 0 as size_t {
            pos = (pos as c_ulong)
                .wrapping_add((MyRand(&raw mut seed) as size_t).wrapping_rem(block_length)
                    as c_ulong) as size_t as size_t;
        }
        if pos.wrapping_add(stride) >= length {
            pos = length.wrapping_sub(stride).wrapping_sub(1 as size_t);
        }
        HistogramAddVectorCommand(
            histograms.offset(i as isize) as *mut HistogramCommand,
            data.offset(pos as isize),
            stride,
        );
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn RandomSampleCommand(
    mut seed: *mut uint32_t,
    mut data: *const uint16_t,
    mut length: size_t,
    mut stride: size_t,
    mut sample: *mut HistogramCommand,
) {
    let mut pos: size_t = 0 as size_t;
    if stride >= length {
        stride = length;
    } else {
        pos = (MyRand(seed) as size_t)
            .wrapping_rem(length.wrapping_sub(stride).wrapping_add(1 as size_t));
    }
    HistogramAddVectorCommand(sample, data.offset(pos as isize), stride);
}
unsafe extern "C" fn RefineEntropyCodesCommand(
    mut data: *const uint16_t,
    mut length: size_t,
    mut stride: size_t,
    mut num_histograms: size_t,
    mut histograms: *mut HistogramCommand,
    mut tmp: *mut HistogramCommand,
) {
    let mut iters: size_t = kIterMulForRefining
        .wrapping_mul(length)
        .wrapping_div(stride)
        .wrapping_add(kMinItersForRefining);
    let mut seed: uint32_t = 7 as uint32_t;
    let mut iter: size_t = 0;
    iters = iters
        .wrapping_add(num_histograms)
        .wrapping_sub(1 as size_t)
        .wrapping_div(num_histograms)
        .wrapping_mul(num_histograms);
    iter = 0 as size_t;
    while iter < iters {
        HistogramClearCommand(tmp);
        RandomSampleCommand(&raw mut seed, data, length, stride, tmp);
        HistogramAddHistogramCommand(
            histograms.offset(iter.wrapping_rem(num_histograms) as isize) as *mut HistogramCommand,
            tmp,
        );
        iter = iter.wrapping_add(1);
    }
}
unsafe extern "C" fn FindBlocksCommand(
    mut data: *const uint16_t,
    length: size_t,
    block_switch_bitcost: c_double,
    num_histograms: size_t,
    mut histograms: *const HistogramCommand,
    mut insert_cost: *mut c_double,
    mut cost: *mut c_double,
    mut switch_signal: *mut uint8_t,
    mut block_id: *mut uint8_t,
) -> size_t {
    let alphabet_size: size_t = HistogramDataSizeCommand() as size_t;
    let bitmap_len: size_t = num_histograms.wrapping_add(7 as size_t) >> 3 as c_int;
    let mut num_blocks: size_t = 1 as size_t;
    let mut byte_ix: size_t = 0;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    if num_histograms <= 1 as size_t {
        i = 0 as size_t;
        while i < length {
            *block_id.offset(i as isize) = 0 as uint8_t;
            i = i.wrapping_add(1);
        }
        return 1 as size_t;
    }
    memset(
        insert_cost as *mut c_void,
        0 as c_int,
        (::core::mem::size_of::<c_double>() as size_t)
            .wrapping_mul(alphabet_size)
            .wrapping_mul(num_histograms),
    );
    i = 0 as size_t;
    while i < num_histograms {
        *insert_cost.offset(i as isize) =
            FastLog2((*histograms.offset(i as isize)).total_count_ as uint32_t as size_t);
        i = i.wrapping_add(1);
    }
    i = alphabet_size;
    while i != 0 as size_t {
        i = i.wrapping_sub(1);
        j = 0 as size_t;
        while j < num_histograms {
            *insert_cost.offset(i.wrapping_mul(num_histograms).wrapping_add(j) as isize) =
                *insert_cost.offset(j as isize)
                    - BitCost((*histograms.offset(j as isize)).data_[i as usize] as size_t);
            j = j.wrapping_add(1);
        }
    }
    memset(
        cost as *mut c_void,
        0 as c_int,
        (::core::mem::size_of::<c_double>() as size_t).wrapping_mul(num_histograms),
    );
    memset(
        switch_signal as *mut c_void,
        0 as c_int,
        (::core::mem::size_of::<uint8_t>() as size_t)
            .wrapping_mul(length)
            .wrapping_mul(bitmap_len),
    );
    byte_ix = 0 as size_t;
    while byte_ix < length {
        let mut ix: size_t = byte_ix.wrapping_mul(bitmap_len);
        let mut symbol: size_t = *data.offset(byte_ix as isize) as size_t;
        let mut insert_cost_ix: size_t = symbol.wrapping_mul(num_histograms);
        let mut min_cost: c_double = 1e99f64;
        let mut block_switch_cost: c_double = block_switch_bitcost;
        static mut prologue_length: size_t = 2000 as size_t;
        static mut multiplier: c_double =
            0.07f64 / 2000 as c_int as c_double;
        let mut k: size_t = 0;
        k = 0 as size_t;
        while k < num_histograms {
            *cost.offset(k as isize) +=
                *insert_cost.offset(insert_cost_ix.wrapping_add(k) as isize);
            if *cost.offset(k as isize) < min_cost {
                min_cost = *cost.offset(k as isize);
                *block_id.offset(byte_ix as isize) = k as uint8_t;
            }
            k = k.wrapping_add(1);
        }
        if byte_ix < prologue_length {
            block_switch_cost *= 0.77f64 + multiplier * byte_ix as c_double;
        }
        k = 0 as size_t;
        while k < num_histograms {
            *cost.offset(k as isize) -= min_cost;
            if *cost.offset(k as isize) >= block_switch_cost {
                let mask: uint8_t = ((1 as c_uint) << (k & 7 as size_t)) as uint8_t;
                *cost.offset(k as isize) = block_switch_cost;
                let ref mut fresh17 =
                    *switch_signal.offset(ix.wrapping_add(k >> 3 as c_int) as isize);
                *fresh17 = (*fresh17 as c_int | mask as c_int) as uint8_t;
            }
            k = k.wrapping_add(1);
        }
        byte_ix = byte_ix.wrapping_add(1);
    }
    byte_ix = length.wrapping_sub(1 as size_t);
    let mut ix_0: size_t = byte_ix.wrapping_mul(bitmap_len);
    let mut cur_id: uint8_t = *block_id.offset(byte_ix as isize);
    while byte_ix > 0 as size_t {
        let mask_0: uint8_t = ((1 as c_uint)
            << (cur_id as c_int & 7 as c_int))
            as uint8_t;
        byte_ix = byte_ix.wrapping_sub(1);
        ix_0 = (ix_0 as c_ulong).wrapping_sub(bitmap_len as c_ulong)
            as size_t as size_t;
        if *switch_signal.offset(
            ix_0.wrapping_add((cur_id as c_int >> 3 as c_int) as size_t)
                as isize,
        ) as c_int
            & mask_0 as c_int
            != 0
        {
            if cur_id as c_int
                != *block_id.offset(byte_ix as isize) as c_int
            {
                cur_id = *block_id.offset(byte_ix as isize);
                num_blocks = num_blocks.wrapping_add(1);
            }
        }
        *block_id.offset(byte_ix as isize) = cur_id;
    }
    return num_blocks;
}
unsafe extern "C" fn RemapBlockIdsCommand(
    mut block_ids: *mut uint8_t,
    length: size_t,
    mut new_id: *mut uint16_t,
    num_histograms: size_t,
) -> size_t {
    static mut kInvalidId: uint16_t = 256 as uint16_t;
    let mut next_id: uint16_t = 0 as uint16_t;
    let mut i: size_t = 0;
    i = 0 as size_t;
    while i < num_histograms {
        *new_id.offset(i as isize) = kInvalidId;
        i = i.wrapping_add(1);
    }
    i = 0 as size_t;
    while i < length {
        if *new_id.offset(*block_ids.offset(i as isize) as isize) as c_int
            == kInvalidId as c_int
        {
            let fresh16 = next_id;
            next_id = next_id.wrapping_add(1);
            *new_id.offset(*block_ids.offset(i as isize) as isize) = fresh16;
        }
        i = i.wrapping_add(1);
    }
    i = 0 as size_t;
    while i < length {
        *block_ids.offset(i as isize) =
            *new_id.offset(*block_ids.offset(i as isize) as isize) as uint8_t;
        i = i.wrapping_add(1);
    }
    return next_id as size_t;
}
unsafe extern "C" fn BuildBlockHistogramsCommand(
    mut data: *const uint16_t,
    length: size_t,
    mut block_ids: *const uint8_t,
    num_histograms: size_t,
    mut histograms: *mut HistogramCommand,
) {
    let mut i: size_t = 0;
    ClearHistogramsCommand(histograms, num_histograms);
    i = 0 as size_t;
    while i < length {
        HistogramAddCommand(
            histograms.offset(*block_ids.offset(i as isize) as isize) as *mut HistogramCommand,
            *data.offset(i as isize) as size_t,
        );
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn ClusterBlocksCommand(
    mut m: *mut MemoryManager,
    mut data: *const uint16_t,
    length: size_t,
    num_blocks: size_t,
    mut block_ids: *mut uint8_t,
    mut split: *mut BlockSplit,
) {
    let mut histogram_symbols: *mut uint32_t = if num_blocks > 0 as size_t {
        BrotliAllocate(
            m,
            num_blocks.wrapping_mul(::core::mem::size_of::<uint32_t>() as size_t),
        ) as *mut uint32_t
    } else {
        ::core::ptr::null_mut::<uint32_t>()
    };
    let mut u32: *mut uint32_t = if num_blocks
        .wrapping_add((4 as c_int * 64 as c_int) as size_t)
        > 0 as size_t
    {
        BrotliAllocate(
            m,
            num_blocks
                .wrapping_add((4 as c_int * 64 as c_int) as size_t)
                .wrapping_mul(::core::mem::size_of::<uint32_t>() as size_t),
        ) as *mut uint32_t
    } else {
        ::core::ptr::null_mut::<uint32_t>()
    };
    let expected_num_clusters: size_t = (CLUSTERS_PER_BATCH as size_t)
        .wrapping_mul(
            num_blocks
                .wrapping_add(HISTOGRAMS_PER_BATCH as size_t)
                .wrapping_sub(1 as size_t),
        )
        .wrapping_div(HISTOGRAMS_PER_BATCH as size_t);
    let mut all_histograms_size: size_t = 0 as size_t;
    let mut all_histograms_capacity: size_t = expected_num_clusters;
    let mut all_histograms: *mut HistogramCommand = if all_histograms_capacity > 0 as size_t {
        BrotliAllocate(
            m,
            all_histograms_capacity
                .wrapping_mul(::core::mem::size_of::<HistogramCommand>() as size_t),
        ) as *mut HistogramCommand
    } else {
        ::core::ptr::null_mut::<HistogramCommand>()
    };
    let mut cluster_size_size: size_t = 0 as size_t;
    let mut cluster_size_capacity: size_t = expected_num_clusters;
    let mut cluster_size: *mut uint32_t = if cluster_size_capacity > 0 as size_t {
        BrotliAllocate(
            m,
            cluster_size_capacity.wrapping_mul(::core::mem::size_of::<uint32_t>() as size_t),
        ) as *mut uint32_t
    } else {
        ::core::ptr::null_mut::<uint32_t>()
    };
    let mut num_clusters: size_t = 0 as size_t;
    let mut histograms: *mut HistogramCommand =
        if brotli_min_size_t(num_blocks, 64 as size_t) > 0 as size_t {
            BrotliAllocate(
                m,
                brotli_min_size_t(num_blocks, 64 as size_t)
                    .wrapping_mul(::core::mem::size_of::<HistogramCommand>() as size_t),
            ) as *mut HistogramCommand
        } else {
            ::core::ptr::null_mut::<HistogramCommand>()
        };
    let mut max_num_pairs: size_t =
        (HISTOGRAMS_PER_BATCH * HISTOGRAMS_PER_BATCH / 2 as c_int) as size_t;
    let mut pairs_capacity: size_t = max_num_pairs.wrapping_add(1 as size_t);
    let mut pairs: *mut HistogramPair = if pairs_capacity > 0 as size_t {
        BrotliAllocate(
            m,
            pairs_capacity.wrapping_mul(::core::mem::size_of::<HistogramPair>() as size_t),
        ) as *mut HistogramPair
    } else {
        ::core::ptr::null_mut::<HistogramPair>()
    };
    let mut pos: size_t = 0 as size_t;
    let mut clusters: *mut uint32_t = ::core::ptr::null_mut::<uint32_t>();
    let mut num_final_clusters: size_t = 0;
    static mut kInvalidIndex: uint32_t = BROTLI_UINT32_MAX;
    let mut new_index: *mut uint32_t = ::core::ptr::null_mut::<uint32_t>();
    let mut i: size_t = 0;
    let sizes: *mut uint32_t = if !u32.is_null() {
        u32.offset((0 as c_int * HISTOGRAMS_PER_BATCH) as isize)
    } else {
        ::core::ptr::null_mut::<uint32_t>()
    };
    let new_clusters: *mut uint32_t = if !u32.is_null() {
        u32.offset((1 as c_int * HISTOGRAMS_PER_BATCH) as isize)
    } else {
        ::core::ptr::null_mut::<uint32_t>()
    };
    let symbols: *mut uint32_t = if !u32.is_null() {
        u32.offset((2 as c_int * HISTOGRAMS_PER_BATCH) as isize)
    } else {
        ::core::ptr::null_mut::<uint32_t>()
    };
    let remap: *mut uint32_t = if !u32.is_null() {
        u32.offset((3 as c_int * HISTOGRAMS_PER_BATCH) as isize)
    } else {
        ::core::ptr::null_mut::<uint32_t>()
    };
    let block_lengths: *mut uint32_t = if !u32.is_null() {
        u32.offset((4 as c_int * HISTOGRAMS_PER_BATCH) as isize)
    } else {
        ::core::ptr::null_mut::<uint32_t>()
    };
    let mut tmp: *mut HistogramCommand = if 2 as c_int > 0 as c_int {
        BrotliAllocate(
            m,
            (2 as size_t).wrapping_mul(::core::mem::size_of::<HistogramCommand>() as size_t),
        ) as *mut HistogramCommand
    } else {
        ::core::ptr::null_mut::<HistogramCommand>()
    };
    if 0 as c_int != 0
        || 0 as c_int != 0
        || 0 as c_int != 0
        || 0 as c_int != 0
        || 0 as c_int != 0
        || 0 as c_int != 0
        || 0 as c_int != 0
        || 0 as c_int != 0
    {
        return;
    }
    memset(
        u32 as *mut c_void,
        0 as c_int,
        num_blocks
            .wrapping_add((4 as c_int * HISTOGRAMS_PER_BATCH) as size_t)
            .wrapping_mul(::core::mem::size_of::<uint32_t>() as size_t),
    );
    let mut block_idx: size_t = 0 as size_t;
    i = 0 as size_t;
    while i < length {
        let ref mut fresh10 = *block_lengths.offset(block_idx as isize);
        *fresh10 = (*fresh10).wrapping_add(1);
        if i.wrapping_add(1 as size_t) == length
            || *block_ids.offset(i as isize) as c_int
                != *block_ids.offset(i.wrapping_add(1 as size_t) as isize) as c_int
        {
            block_idx = block_idx.wrapping_add(1);
        }
        i = i.wrapping_add(1);
    }
    i = 0 as size_t;
    while i < num_blocks {
        let num_to_combine: size_t =
            brotli_min_size_t(num_blocks.wrapping_sub(i), 64 as size_t) as size_t;
        let mut num_new_clusters: size_t = 0;
        let mut j: size_t = 0;
        j = 0 as size_t;
        while j < num_to_combine {
            let mut k: size_t = 0;
            let mut block_length: size_t =
                *block_lengths.offset(i.wrapping_add(j) as isize) as size_t;
            HistogramClearCommand(histograms.offset(j as isize) as *mut HistogramCommand);
            k = 0 as size_t;
            while k < block_length {
                let fresh11 = pos;
                pos = pos.wrapping_add(1);
                HistogramAddCommand(
                    histograms.offset(j as isize) as *mut HistogramCommand,
                    *data.offset(fresh11 as isize) as size_t,
                );
                k = k.wrapping_add(1);
            }
            (*histograms.offset(j as isize)).bit_cost_ =
                BrotliPopulationCostCommand(histograms.offset(j as isize) as *mut HistogramCommand);
            *new_clusters.offset(j as isize) = j as uint32_t;
            *symbols.offset(j as isize) = j as uint32_t;
            *sizes.offset(j as isize) = 1 as uint32_t;
            j = j.wrapping_add(1);
        }
        num_new_clusters = BrotliHistogramCombineCommand(
            histograms,
            tmp,
            sizes,
            symbols,
            new_clusters,
            pairs,
            num_to_combine,
            num_to_combine,
            HISTOGRAMS_PER_BATCH as size_t,
            max_num_pairs,
        );
        if all_histograms_capacity < all_histograms_size.wrapping_add(num_new_clusters) {
            let mut _new_size: size_t = if all_histograms_capacity == 0 as size_t {
                all_histograms_size.wrapping_add(num_new_clusters)
            } else {
                all_histograms_capacity
            };
            let mut new_array: *mut HistogramCommand = ::core::ptr::null_mut::<HistogramCommand>();
            while _new_size < all_histograms_size.wrapping_add(num_new_clusters) {
                _new_size = (_new_size as c_ulong)
                    .wrapping_mul(2 as c_ulong) as size_t
                    as size_t;
            }
            new_array = if _new_size > 0 as size_t {
                BrotliAllocate(
                    m,
                    _new_size.wrapping_mul(::core::mem::size_of::<HistogramCommand>() as size_t),
                ) as *mut HistogramCommand
            } else {
                ::core::ptr::null_mut::<HistogramCommand>()
            };
            if 0 as c_int == 0
                && 0 as c_int == 0
                && all_histograms_capacity != 0 as size_t
            {
                memcpy(
                    new_array as *mut c_void,
                    all_histograms as *const c_void,
                    all_histograms_capacity
                        .wrapping_mul(::core::mem::size_of::<HistogramCommand>() as size_t),
                );
            }
            BrotliFree(m, all_histograms as *mut c_void);
            all_histograms = ::core::ptr::null_mut::<HistogramCommand>();
            all_histograms = new_array;
            all_histograms_capacity = _new_size;
        }
        if cluster_size_capacity < cluster_size_size.wrapping_add(num_new_clusters) {
            let mut _new_size_0: size_t = if cluster_size_capacity == 0 as size_t {
                cluster_size_size.wrapping_add(num_new_clusters)
            } else {
                cluster_size_capacity
            };
            let mut new_array_0: *mut uint32_t = ::core::ptr::null_mut::<uint32_t>();
            while _new_size_0 < cluster_size_size.wrapping_add(num_new_clusters) {
                _new_size_0 = (_new_size_0 as c_ulong)
                    .wrapping_mul(2 as c_ulong) as size_t
                    as size_t;
            }
            new_array_0 = if _new_size_0 > 0 as size_t {
                BrotliAllocate(
                    m,
                    _new_size_0.wrapping_mul(::core::mem::size_of::<uint32_t>() as size_t),
                ) as *mut uint32_t
            } else {
                ::core::ptr::null_mut::<uint32_t>()
            };
            if 0 as c_int == 0
                && 0 as c_int == 0
                && cluster_size_capacity != 0 as size_t
            {
                memcpy(
                    new_array_0 as *mut c_void,
                    cluster_size as *const c_void,
                    cluster_size_capacity
                        .wrapping_mul(::core::mem::size_of::<uint32_t>() as size_t),
                );
            }
            BrotliFree(m, cluster_size as *mut c_void);
            cluster_size = ::core::ptr::null_mut::<uint32_t>();
            cluster_size = new_array_0;
            cluster_size_capacity = _new_size_0;
        }
        if 0 as c_int != 0 {
            return;
        }
        j = 0 as size_t;
        while j < num_new_clusters {
            let fresh12 = all_histograms_size;
            all_histograms_size = all_histograms_size.wrapping_add(1);
            *all_histograms.offset(fresh12 as isize) =
                *histograms.offset(*new_clusters.offset(j as isize) as isize);
            let fresh13 = cluster_size_size;
            cluster_size_size = cluster_size_size.wrapping_add(1);
            *cluster_size.offset(fresh13 as isize) =
                *sizes.offset(*new_clusters.offset(j as isize) as isize);
            *remap.offset(*new_clusters.offset(j as isize) as isize) = j as uint32_t;
            j = j.wrapping_add(1);
        }
        j = 0 as size_t;
        while j < num_to_combine {
            *histogram_symbols.offset(i.wrapping_add(j) as isize) = (num_clusters as uint32_t)
                .wrapping_add(*remap.offset(*symbols.offset(j as isize) as isize));
            j = j.wrapping_add(1);
        }
        num_clusters = (num_clusters as c_ulong)
            .wrapping_add(num_new_clusters as c_ulong) as size_t
            as size_t;
        i = (i as c_ulong).wrapping_add(HISTOGRAMS_PER_BATCH as c_ulong)
            as size_t as size_t;
    }
    BrotliFree(m, histograms as *mut c_void);
    histograms = ::core::ptr::null_mut::<HistogramCommand>();
    max_num_pairs = brotli_min_size_t(
        (64 as size_t).wrapping_mul(num_clusters),
        num_clusters
            .wrapping_div(2 as size_t)
            .wrapping_mul(num_clusters),
    );
    if pairs_capacity < max_num_pairs.wrapping_add(1 as size_t) {
        BrotliFree(m, pairs as *mut c_void);
        pairs = ::core::ptr::null_mut::<HistogramPair>();
        pairs = if max_num_pairs.wrapping_add(1 as size_t) > 0 as size_t {
            BrotliAllocate(
                m,
                max_num_pairs
                    .wrapping_add(1 as size_t)
                    .wrapping_mul(::core::mem::size_of::<HistogramPair>() as size_t),
            ) as *mut HistogramPair
        } else {
            ::core::ptr::null_mut::<HistogramPair>()
        };
        if 0 as c_int != 0 || 0 as c_int != 0 {
            return;
        }
    }
    clusters = if num_clusters > 0 as size_t {
        BrotliAllocate(
            m,
            num_clusters.wrapping_mul(::core::mem::size_of::<uint32_t>() as size_t),
        ) as *mut uint32_t
    } else {
        ::core::ptr::null_mut::<uint32_t>()
    };
    if 0 as c_int != 0 || 0 as c_int != 0 {
        return;
    }
    i = 0 as size_t;
    while i < num_clusters {
        *clusters.offset(i as isize) = i as uint32_t;
        i = i.wrapping_add(1);
    }
    num_final_clusters = BrotliHistogramCombineCommand(
        all_histograms,
        tmp,
        cluster_size,
        histogram_symbols,
        clusters,
        pairs,
        num_clusters,
        num_blocks,
        BROTLI_MAX_NUMBER_OF_BLOCK_TYPES as size_t,
        max_num_pairs,
    );
    BrotliFree(m, pairs as *mut c_void);
    pairs = ::core::ptr::null_mut::<HistogramPair>();
    BrotliFree(m, cluster_size as *mut c_void);
    cluster_size = ::core::ptr::null_mut::<uint32_t>();
    new_index = if num_clusters > 0 as size_t {
        BrotliAllocate(
            m,
            num_clusters.wrapping_mul(::core::mem::size_of::<uint32_t>() as size_t),
        ) as *mut uint32_t
    } else {
        ::core::ptr::null_mut::<uint32_t>()
    };
    if 0 as c_int != 0 || 0 as c_int != 0 {
        return;
    }
    i = 0 as size_t;
    while i < num_clusters {
        *new_index.offset(i as isize) = kInvalidIndex;
        i = i.wrapping_add(1);
    }
    pos = 0 as size_t;
    let mut next_index: uint32_t = 0 as uint32_t;
    i = 0 as size_t;
    while i < num_blocks {
        let mut j_0: size_t = 0;
        let mut best_out: uint32_t = 0;
        let mut best_bits: c_double = 0.;
        HistogramClearCommand(tmp);
        j_0 = 0 as size_t;
        while j_0 < *block_lengths.offset(i as isize) as size_t {
            let fresh14 = pos;
            pos = pos.wrapping_add(1);
            HistogramAddCommand(tmp, *data.offset(fresh14 as isize) as size_t);
            j_0 = j_0.wrapping_add(1);
        }
        best_out = if i == 0 as size_t {
            *histogram_symbols.offset(0 as c_int as isize)
        } else {
            *histogram_symbols.offset(i.wrapping_sub(1 as size_t) as isize)
        };
        best_bits = BrotliHistogramBitCostDistanceCommand(
            tmp,
            all_histograms.offset(best_out as isize) as *mut HistogramCommand,
            tmp.offset(1 as c_int as isize),
        );
        j_0 = 0 as size_t;
        while j_0 < num_final_clusters {
            let cur_bits: c_double = BrotliHistogramBitCostDistanceCommand(
                tmp,
                all_histograms.offset(*clusters.offset(j_0 as isize) as isize)
                    as *mut HistogramCommand,
                tmp.offset(1 as c_int as isize),
            ) as c_double;
            if cur_bits < best_bits {
                best_bits = cur_bits;
                best_out = *clusters.offset(j_0 as isize);
            }
            j_0 = j_0.wrapping_add(1);
        }
        *histogram_symbols.offset(i as isize) = best_out;
        if *new_index.offset(best_out as isize) == kInvalidIndex {
            let fresh15 = next_index;
            next_index = next_index.wrapping_add(1);
            *new_index.offset(best_out as isize) = fresh15;
        }
        i = i.wrapping_add(1);
    }
    BrotliFree(m, tmp as *mut c_void);
    tmp = ::core::ptr::null_mut::<HistogramCommand>();
    BrotliFree(m, clusters as *mut c_void);
    clusters = ::core::ptr::null_mut::<uint32_t>();
    BrotliFree(m, all_histograms as *mut c_void);
    all_histograms = ::core::ptr::null_mut::<HistogramCommand>();
    if (*split).types_alloc_size < num_blocks {
        let mut _new_size_1: size_t = if (*split).types_alloc_size == 0 as size_t {
            num_blocks
        } else {
            (*split).types_alloc_size
        };
        let mut new_array_1: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        while _new_size_1 < num_blocks {
            _new_size_1 = (_new_size_1 as c_ulong)
                .wrapping_mul(2 as c_ulong) as size_t
                as size_t;
        }
        new_array_1 = if _new_size_1 > 0 as size_t {
            BrotliAllocate(
                m,
                _new_size_1.wrapping_mul(::core::mem::size_of::<uint8_t>() as size_t),
            ) as *mut uint8_t
        } else {
            ::core::ptr::null_mut::<uint8_t>()
        };
        if 0 as c_int == 0
            && 0 as c_int == 0
            && (*split).types_alloc_size != 0 as size_t
        {
            memcpy(
                new_array_1 as *mut c_void,
                (*split).types as *const c_void,
                (*split)
                    .types_alloc_size
                    .wrapping_mul(::core::mem::size_of::<uint8_t>() as size_t),
            );
        }
        BrotliFree(m, (*split).types as *mut c_void);
        (*split).types = ::core::ptr::null_mut::<uint8_t>();
        (*split).types = new_array_1;
        (*split).types_alloc_size = _new_size_1;
    }
    if (*split).lengths_alloc_size < num_blocks {
        let mut _new_size_2: size_t = if (*split).lengths_alloc_size == 0 as size_t {
            num_blocks
        } else {
            (*split).lengths_alloc_size
        };
        let mut new_array_2: *mut uint32_t = ::core::ptr::null_mut::<uint32_t>();
        while _new_size_2 < num_blocks {
            _new_size_2 = (_new_size_2 as c_ulong)
                .wrapping_mul(2 as c_ulong) as size_t
                as size_t;
        }
        new_array_2 = if _new_size_2 > 0 as size_t {
            BrotliAllocate(
                m,
                _new_size_2.wrapping_mul(::core::mem::size_of::<uint32_t>() as size_t),
            ) as *mut uint32_t
        } else {
            ::core::ptr::null_mut::<uint32_t>()
        };
        if 0 as c_int == 0
            && 0 as c_int == 0
            && (*split).lengths_alloc_size != 0 as size_t
        {
            memcpy(
                new_array_2 as *mut c_void,
                (*split).lengths as *const c_void,
                (*split)
                    .lengths_alloc_size
                    .wrapping_mul(::core::mem::size_of::<uint32_t>() as size_t),
            );
        }
        BrotliFree(m, (*split).lengths as *mut c_void);
        (*split).lengths = ::core::ptr::null_mut::<uint32_t>();
        (*split).lengths = new_array_2;
        (*split).lengths_alloc_size = _new_size_2;
    }
    if 0 as c_int != 0 {
        return;
    }
    let mut cur_length: uint32_t = 0 as uint32_t;
    let mut block_idx_0: size_t = 0 as size_t;
    let mut max_type: uint8_t = 0 as uint8_t;
    i = 0 as size_t;
    while i < num_blocks {
        cur_length = (cur_length as c_uint)
            .wrapping_add(*block_lengths.offset(i as isize) as c_uint)
            as uint32_t as uint32_t;
        if i.wrapping_add(1 as size_t) == num_blocks
            || *histogram_symbols.offset(i as isize)
                != *histogram_symbols.offset(i.wrapping_add(1 as size_t) as isize)
        {
            let id: uint8_t =
                *new_index.offset(*histogram_symbols.offset(i as isize) as isize) as uint8_t;
            *(*split).types.offset(block_idx_0 as isize) = id;
            *(*split).lengths.offset(block_idx_0 as isize) = cur_length;
            max_type = brotli_max_uint8_t(max_type, id);
            cur_length = 0 as uint32_t;
            block_idx_0 = block_idx_0.wrapping_add(1);
        }
        i = i.wrapping_add(1);
    }
    (*split).num_blocks = block_idx_0;
    (*split).num_types = (max_type as size_t).wrapping_add(1 as size_t);
    BrotliFree(m, new_index as *mut c_void);
    new_index = ::core::ptr::null_mut::<uint32_t>();
    BrotliFree(m, u32 as *mut c_void);
    u32 = ::core::ptr::null_mut::<uint32_t>();
    BrotliFree(m, histogram_symbols as *mut c_void);
    histogram_symbols = ::core::ptr::null_mut::<uint32_t>();
}
unsafe extern "C" fn SplitByteVectorCommand(
    mut m: *mut MemoryManager,
    mut data: *const uint16_t,
    length: size_t,
    symbols_per_histogram: size_t,
    max_histograms: size_t,
    sampling_stride_length: size_t,
    block_switch_cost: c_double,
    mut params: *const BrotliEncoderParams,
    mut split: *mut BlockSplit,
) {
    let data_size: size_t = HistogramDataSizeCommand() as size_t;
    let mut histograms: *mut HistogramCommand = ::core::ptr::null_mut::<HistogramCommand>();
    let mut tmp: *mut HistogramCommand = ::core::ptr::null_mut::<HistogramCommand>();
    let mut num_histograms: size_t = length
        .wrapping_div(symbols_per_histogram)
        .wrapping_add(1 as size_t);
    if num_histograms > max_histograms {
        num_histograms = max_histograms;
    }
    if length == 0 as size_t {
        (*split).num_types = 1 as size_t;
        return;
    }
    if length < kMinLengthForBlockSplitting {
        if (*split).types_alloc_size < (*split).num_blocks.wrapping_add(1 as size_t) {
            let mut _new_size: size_t = if (*split).types_alloc_size == 0 as size_t {
                (*split).num_blocks.wrapping_add(1 as size_t)
            } else {
                (*split).types_alloc_size
            };
            let mut new_array: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
            while _new_size < (*split).num_blocks.wrapping_add(1 as size_t) {
                _new_size = (_new_size as c_ulong)
                    .wrapping_mul(2 as c_ulong) as size_t
                    as size_t;
            }
            new_array = if _new_size > 0 as size_t {
                BrotliAllocate(
                    m,
                    _new_size.wrapping_mul(::core::mem::size_of::<uint8_t>() as size_t),
                ) as *mut uint8_t
            } else {
                ::core::ptr::null_mut::<uint8_t>()
            };
            if 0 as c_int == 0
                && 0 as c_int == 0
                && (*split).types_alloc_size != 0 as size_t
            {
                memcpy(
                    new_array as *mut c_void,
                    (*split).types as *const c_void,
                    (*split)
                        .types_alloc_size
                        .wrapping_mul(::core::mem::size_of::<uint8_t>() as size_t),
                );
            }
            BrotliFree(m, (*split).types as *mut c_void);
            (*split).types = ::core::ptr::null_mut::<uint8_t>();
            (*split).types = new_array;
            (*split).types_alloc_size = _new_size;
        }
        if (*split).lengths_alloc_size < (*split).num_blocks.wrapping_add(1 as size_t) {
            let mut _new_size_0: size_t = if (*split).lengths_alloc_size == 0 as size_t {
                (*split).num_blocks.wrapping_add(1 as size_t)
            } else {
                (*split).lengths_alloc_size
            };
            let mut new_array_0: *mut uint32_t = ::core::ptr::null_mut::<uint32_t>();
            while _new_size_0 < (*split).num_blocks.wrapping_add(1 as size_t) {
                _new_size_0 = (_new_size_0 as c_ulong)
                    .wrapping_mul(2 as c_ulong) as size_t
                    as size_t;
            }
            new_array_0 = if _new_size_0 > 0 as size_t {
                BrotliAllocate(
                    m,
                    _new_size_0.wrapping_mul(::core::mem::size_of::<uint32_t>() as size_t),
                ) as *mut uint32_t
            } else {
                ::core::ptr::null_mut::<uint32_t>()
            };
            if 0 as c_int == 0
                && 0 as c_int == 0
                && (*split).lengths_alloc_size != 0 as size_t
            {
                memcpy(
                    new_array_0 as *mut c_void,
                    (*split).lengths as *const c_void,
                    (*split)
                        .lengths_alloc_size
                        .wrapping_mul(::core::mem::size_of::<uint32_t>() as size_t),
                );
            }
            BrotliFree(m, (*split).lengths as *mut c_void);
            (*split).lengths = ::core::ptr::null_mut::<uint32_t>();
            (*split).lengths = new_array_0;
            (*split).lengths_alloc_size = _new_size_0;
        }
        if 0 as c_int != 0 {
            return;
        }
        (*split).num_types = 1 as size_t;
        *(*split).types.offset((*split).num_blocks as isize) = 0 as uint8_t;
        *(*split).lengths.offset((*split).num_blocks as isize) = length as uint32_t;
        (*split).num_blocks = (*split).num_blocks.wrapping_add(1);
        return;
    }
    histograms = if num_histograms.wrapping_add(1 as size_t) > 0 as size_t {
        BrotliAllocate(
            m,
            num_histograms
                .wrapping_add(1 as size_t)
                .wrapping_mul(::core::mem::size_of::<HistogramCommand>() as size_t),
        ) as *mut HistogramCommand
    } else {
        ::core::ptr::null_mut::<HistogramCommand>()
    };
    tmp = histograms.offset(num_histograms as isize);
    if 0 as c_int != 0 || 0 as c_int != 0 {
        return;
    }
    InitialEntropyCodesCommand(
        data,
        length,
        sampling_stride_length,
        num_histograms,
        histograms,
    );
    RefineEntropyCodesCommand(
        data,
        length,
        sampling_stride_length,
        num_histograms,
        histograms,
        tmp,
    );
    let mut block_ids: *mut uint8_t = if length > 0 as size_t {
        BrotliAllocate(
            m,
            length.wrapping_mul(::core::mem::size_of::<uint8_t>() as size_t),
        ) as *mut uint8_t
    } else {
        ::core::ptr::null_mut::<uint8_t>()
    };
    let mut num_blocks: size_t = 0 as size_t;
    let bitmaplen: size_t = num_histograms.wrapping_add(7 as size_t) >> 3 as c_int;
    let mut insert_cost: *mut c_double =
        if data_size.wrapping_mul(num_histograms) > 0 as size_t {
            BrotliAllocate(
                m,
                data_size
                    .wrapping_mul(num_histograms)
                    .wrapping_mul(::core::mem::size_of::<c_double>() as size_t),
            ) as *mut c_double
        } else {
            ::core::ptr::null_mut::<c_double>()
        };
    let mut cost: *mut c_double = if num_histograms > 0 as size_t {
        BrotliAllocate(
            m,
            num_histograms.wrapping_mul(::core::mem::size_of::<c_double>() as size_t),
        ) as *mut c_double
    } else {
        ::core::ptr::null_mut::<c_double>()
    };
    let mut switch_signal: *mut uint8_t = if length.wrapping_mul(bitmaplen) > 0 as size_t {
        BrotliAllocate(
            m,
            length
                .wrapping_mul(bitmaplen)
                .wrapping_mul(::core::mem::size_of::<uint8_t>() as size_t),
        ) as *mut uint8_t
    } else {
        ::core::ptr::null_mut::<uint8_t>()
    };
    let mut new_id: *mut uint16_t = if num_histograms > 0 as size_t {
        BrotliAllocate(
            m,
            num_histograms.wrapping_mul(::core::mem::size_of::<uint16_t>() as size_t),
        ) as *mut uint16_t
    } else {
        ::core::ptr::null_mut::<uint16_t>()
    };
    let iters: size_t = (if (*params).quality < HQ_ZOPFLIFICATION_QUALITY {
        3 as c_int
    } else {
        10 as c_int
    }) as size_t;
    let mut i: size_t = 0;
    if 0 as c_int != 0
        || 0 as c_int != 0
        || 0 as c_int != 0
        || 0 as c_int != 0
        || 0 as c_int != 0
        || 0 as c_int != 0
    {
        return;
    }
    i = 0 as size_t;
    while i < iters {
        num_blocks = FindBlocksCommand(
            data,
            length,
            block_switch_cost,
            num_histograms,
            histograms,
            insert_cost,
            cost,
            switch_signal,
            block_ids,
        );
        num_histograms = RemapBlockIdsCommand(block_ids, length, new_id, num_histograms);
        BuildBlockHistogramsCommand(data, length, block_ids, num_histograms, histograms);
        i = i.wrapping_add(1);
    }
    BrotliFree(m, insert_cost as *mut c_void);
    insert_cost = ::core::ptr::null_mut::<c_double>();
    BrotliFree(m, cost as *mut c_void);
    cost = ::core::ptr::null_mut::<c_double>();
    BrotliFree(m, switch_signal as *mut c_void);
    switch_signal = ::core::ptr::null_mut::<uint8_t>();
    BrotliFree(m, new_id as *mut c_void);
    new_id = ::core::ptr::null_mut::<uint16_t>();
    BrotliFree(m, histograms as *mut c_void);
    histograms = ::core::ptr::null_mut::<HistogramCommand>();
    ClusterBlocksCommand(m, data, length, num_blocks, block_ids, split);
    if 0 as c_int != 0 {
        return;
    }
    BrotliFree(m, block_ids as *mut c_void);
    block_ids = ::core::ptr::null_mut::<uint8_t>();
}
unsafe extern "C" fn InitialEntropyCodesDistance(
    mut data: *const uint16_t,
    mut length: size_t,
    mut stride: size_t,
    mut num_histograms: size_t,
    mut histograms: *mut HistogramDistance,
) {
    let mut seed: uint32_t = 7 as uint32_t;
    let mut block_length: size_t = length.wrapping_div(num_histograms);
    let mut i: size_t = 0;
    ClearHistogramsDistance(histograms, num_histograms);
    i = 0 as size_t;
    while i < num_histograms {
        let mut pos: size_t = length.wrapping_mul(i).wrapping_div(num_histograms);
        if i != 0 as size_t {
            pos = (pos as c_ulong)
                .wrapping_add((MyRand(&raw mut seed) as size_t).wrapping_rem(block_length)
                    as c_ulong) as size_t as size_t;
        }
        if pos.wrapping_add(stride) >= length {
            pos = length.wrapping_sub(stride).wrapping_sub(1 as size_t);
        }
        HistogramAddVectorDistance(
            histograms.offset(i as isize) as *mut HistogramDistance,
            data.offset(pos as isize),
            stride,
        );
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn RandomSampleDistance(
    mut seed: *mut uint32_t,
    mut data: *const uint16_t,
    mut length: size_t,
    mut stride: size_t,
    mut sample: *mut HistogramDistance,
) {
    let mut pos: size_t = 0 as size_t;
    if stride >= length {
        stride = length;
    } else {
        pos = (MyRand(seed) as size_t)
            .wrapping_rem(length.wrapping_sub(stride).wrapping_add(1 as size_t));
    }
    HistogramAddVectorDistance(sample, data.offset(pos as isize), stride);
}
unsafe extern "C" fn RefineEntropyCodesDistance(
    mut data: *const uint16_t,
    mut length: size_t,
    mut stride: size_t,
    mut num_histograms: size_t,
    mut histograms: *mut HistogramDistance,
    mut tmp: *mut HistogramDistance,
) {
    let mut iters: size_t = kIterMulForRefining
        .wrapping_mul(length)
        .wrapping_div(stride)
        .wrapping_add(kMinItersForRefining);
    let mut seed: uint32_t = 7 as uint32_t;
    let mut iter: size_t = 0;
    iters = iters
        .wrapping_add(num_histograms)
        .wrapping_sub(1 as size_t)
        .wrapping_div(num_histograms)
        .wrapping_mul(num_histograms);
    iter = 0 as size_t;
    while iter < iters {
        HistogramClearDistance(tmp);
        RandomSampleDistance(&raw mut seed, data, length, stride, tmp);
        HistogramAddHistogramDistance(
            histograms.offset(iter.wrapping_rem(num_histograms) as isize) as *mut HistogramDistance,
            tmp,
        );
        iter = iter.wrapping_add(1);
    }
}
unsafe extern "C" fn FindBlocksDistance(
    mut data: *const uint16_t,
    length: size_t,
    block_switch_bitcost: c_double,
    num_histograms: size_t,
    mut histograms: *const HistogramDistance,
    mut insert_cost: *mut c_double,
    mut cost: *mut c_double,
    mut switch_signal: *mut uint8_t,
    mut block_id: *mut uint8_t,
) -> size_t {
    let alphabet_size: size_t = HistogramDataSizeDistance() as size_t;
    let bitmap_len: size_t = num_histograms.wrapping_add(7 as size_t) >> 3 as c_int;
    let mut num_blocks: size_t = 1 as size_t;
    let mut byte_ix: size_t = 0;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    if num_histograms <= 1 as size_t {
        i = 0 as size_t;
        while i < length {
            *block_id.offset(i as isize) = 0 as uint8_t;
            i = i.wrapping_add(1);
        }
        return 1 as size_t;
    }
    memset(
        insert_cost as *mut c_void,
        0 as c_int,
        (::core::mem::size_of::<c_double>() as size_t)
            .wrapping_mul(alphabet_size)
            .wrapping_mul(num_histograms),
    );
    i = 0 as size_t;
    while i < num_histograms {
        *insert_cost.offset(i as isize) =
            FastLog2((*histograms.offset(i as isize)).total_count_ as uint32_t as size_t);
        i = i.wrapping_add(1);
    }
    i = alphabet_size;
    while i != 0 as size_t {
        i = i.wrapping_sub(1);
        j = 0 as size_t;
        while j < num_histograms {
            *insert_cost.offset(i.wrapping_mul(num_histograms).wrapping_add(j) as isize) =
                *insert_cost.offset(j as isize)
                    - BitCost((*histograms.offset(j as isize)).data_[i as usize] as size_t);
            j = j.wrapping_add(1);
        }
    }
    memset(
        cost as *mut c_void,
        0 as c_int,
        (::core::mem::size_of::<c_double>() as size_t).wrapping_mul(num_histograms),
    );
    memset(
        switch_signal as *mut c_void,
        0 as c_int,
        (::core::mem::size_of::<uint8_t>() as size_t)
            .wrapping_mul(length)
            .wrapping_mul(bitmap_len),
    );
    byte_ix = 0 as size_t;
    while byte_ix < length {
        let mut ix: size_t = byte_ix.wrapping_mul(bitmap_len);
        let mut symbol: size_t = *data.offset(byte_ix as isize) as size_t;
        let mut insert_cost_ix: size_t = symbol.wrapping_mul(num_histograms);
        let mut min_cost: c_double = 1e99f64;
        let mut block_switch_cost: c_double = block_switch_bitcost;
        static mut prologue_length: size_t = 2000 as size_t;
        static mut multiplier: c_double =
            0.07f64 / 2000 as c_int as c_double;
        let mut k: size_t = 0;
        k = 0 as size_t;
        while k < num_histograms {
            *cost.offset(k as isize) +=
                *insert_cost.offset(insert_cost_ix.wrapping_add(k) as isize);
            if *cost.offset(k as isize) < min_cost {
                min_cost = *cost.offset(k as isize);
                *block_id.offset(byte_ix as isize) = k as uint8_t;
            }
            k = k.wrapping_add(1);
        }
        if byte_ix < prologue_length {
            block_switch_cost *= 0.77f64 + multiplier * byte_ix as c_double;
        }
        k = 0 as size_t;
        while k < num_histograms {
            *cost.offset(k as isize) -= min_cost;
            if *cost.offset(k as isize) >= block_switch_cost {
                let mask: uint8_t = ((1 as c_uint) << (k & 7 as size_t)) as uint8_t;
                *cost.offset(k as isize) = block_switch_cost;
                let ref mut fresh8 =
                    *switch_signal.offset(ix.wrapping_add(k >> 3 as c_int) as isize);
                *fresh8 = (*fresh8 as c_int | mask as c_int) as uint8_t;
            }
            k = k.wrapping_add(1);
        }
        byte_ix = byte_ix.wrapping_add(1);
    }
    byte_ix = length.wrapping_sub(1 as size_t);
    let mut ix_0: size_t = byte_ix.wrapping_mul(bitmap_len);
    let mut cur_id: uint8_t = *block_id.offset(byte_ix as isize);
    while byte_ix > 0 as size_t {
        let mask_0: uint8_t = ((1 as c_uint)
            << (cur_id as c_int & 7 as c_int))
            as uint8_t;
        byte_ix = byte_ix.wrapping_sub(1);
        ix_0 = (ix_0 as c_ulong).wrapping_sub(bitmap_len as c_ulong)
            as size_t as size_t;
        if *switch_signal.offset(
            ix_0.wrapping_add((cur_id as c_int >> 3 as c_int) as size_t)
                as isize,
        ) as c_int
            & mask_0 as c_int
            != 0
        {
            if cur_id as c_int
                != *block_id.offset(byte_ix as isize) as c_int
            {
                cur_id = *block_id.offset(byte_ix as isize);
                num_blocks = num_blocks.wrapping_add(1);
            }
        }
        *block_id.offset(byte_ix as isize) = cur_id;
    }
    return num_blocks;
}
unsafe extern "C" fn RemapBlockIdsDistance(
    mut block_ids: *mut uint8_t,
    length: size_t,
    mut new_id: *mut uint16_t,
    num_histograms: size_t,
) -> size_t {
    static mut kInvalidId: uint16_t = 256 as uint16_t;
    let mut next_id: uint16_t = 0 as uint16_t;
    let mut i: size_t = 0;
    i = 0 as size_t;
    while i < num_histograms {
        *new_id.offset(i as isize) = kInvalidId;
        i = i.wrapping_add(1);
    }
    i = 0 as size_t;
    while i < length {
        if *new_id.offset(*block_ids.offset(i as isize) as isize) as c_int
            == kInvalidId as c_int
        {
            let fresh7 = next_id;
            next_id = next_id.wrapping_add(1);
            *new_id.offset(*block_ids.offset(i as isize) as isize) = fresh7;
        }
        i = i.wrapping_add(1);
    }
    i = 0 as size_t;
    while i < length {
        *block_ids.offset(i as isize) =
            *new_id.offset(*block_ids.offset(i as isize) as isize) as uint8_t;
        i = i.wrapping_add(1);
    }
    return next_id as size_t;
}
unsafe extern "C" fn BuildBlockHistogramsDistance(
    mut data: *const uint16_t,
    length: size_t,
    mut block_ids: *const uint8_t,
    num_histograms: size_t,
    mut histograms: *mut HistogramDistance,
) {
    let mut i: size_t = 0;
    ClearHistogramsDistance(histograms, num_histograms);
    i = 0 as size_t;
    while i < length {
        HistogramAddDistance(
            histograms.offset(*block_ids.offset(i as isize) as isize) as *mut HistogramDistance,
            *data.offset(i as isize) as size_t,
        );
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn ClusterBlocksDistance(
    mut m: *mut MemoryManager,
    mut data: *const uint16_t,
    length: size_t,
    num_blocks: size_t,
    mut block_ids: *mut uint8_t,
    mut split: *mut BlockSplit,
) {
    let mut histogram_symbols: *mut uint32_t = if num_blocks > 0 as size_t {
        BrotliAllocate(
            m,
            num_blocks.wrapping_mul(::core::mem::size_of::<uint32_t>() as size_t),
        ) as *mut uint32_t
    } else {
        ::core::ptr::null_mut::<uint32_t>()
    };
    let mut u32: *mut uint32_t = if num_blocks
        .wrapping_add((4 as c_int * 64 as c_int) as size_t)
        > 0 as size_t
    {
        BrotliAllocate(
            m,
            num_blocks
                .wrapping_add((4 as c_int * 64 as c_int) as size_t)
                .wrapping_mul(::core::mem::size_of::<uint32_t>() as size_t),
        ) as *mut uint32_t
    } else {
        ::core::ptr::null_mut::<uint32_t>()
    };
    let expected_num_clusters: size_t = (CLUSTERS_PER_BATCH as size_t)
        .wrapping_mul(
            num_blocks
                .wrapping_add(HISTOGRAMS_PER_BATCH as size_t)
                .wrapping_sub(1 as size_t),
        )
        .wrapping_div(HISTOGRAMS_PER_BATCH as size_t);
    let mut all_histograms_size: size_t = 0 as size_t;
    let mut all_histograms_capacity: size_t = expected_num_clusters;
    let mut all_histograms: *mut HistogramDistance = if all_histograms_capacity > 0 as size_t {
        BrotliAllocate(
            m,
            all_histograms_capacity
                .wrapping_mul(::core::mem::size_of::<HistogramDistance>() as size_t),
        ) as *mut HistogramDistance
    } else {
        ::core::ptr::null_mut::<HistogramDistance>()
    };
    let mut cluster_size_size: size_t = 0 as size_t;
    let mut cluster_size_capacity: size_t = expected_num_clusters;
    let mut cluster_size: *mut uint32_t = if cluster_size_capacity > 0 as size_t {
        BrotliAllocate(
            m,
            cluster_size_capacity.wrapping_mul(::core::mem::size_of::<uint32_t>() as size_t),
        ) as *mut uint32_t
    } else {
        ::core::ptr::null_mut::<uint32_t>()
    };
    let mut num_clusters: size_t = 0 as size_t;
    let mut histograms: *mut HistogramDistance =
        if brotli_min_size_t(num_blocks, 64 as size_t) > 0 as size_t {
            BrotliAllocate(
                m,
                brotli_min_size_t(num_blocks, 64 as size_t)
                    .wrapping_mul(::core::mem::size_of::<HistogramDistance>() as size_t),
            ) as *mut HistogramDistance
        } else {
            ::core::ptr::null_mut::<HistogramDistance>()
        };
    let mut max_num_pairs: size_t =
        (HISTOGRAMS_PER_BATCH * HISTOGRAMS_PER_BATCH / 2 as c_int) as size_t;
    let mut pairs_capacity: size_t = max_num_pairs.wrapping_add(1 as size_t);
    let mut pairs: *mut HistogramPair = if pairs_capacity > 0 as size_t {
        BrotliAllocate(
            m,
            pairs_capacity.wrapping_mul(::core::mem::size_of::<HistogramPair>() as size_t),
        ) as *mut HistogramPair
    } else {
        ::core::ptr::null_mut::<HistogramPair>()
    };
    let mut pos: size_t = 0 as size_t;
    let mut clusters: *mut uint32_t = ::core::ptr::null_mut::<uint32_t>();
    let mut num_final_clusters: size_t = 0;
    static mut kInvalidIndex: uint32_t = BROTLI_UINT32_MAX;
    let mut new_index: *mut uint32_t = ::core::ptr::null_mut::<uint32_t>();
    let mut i: size_t = 0;
    let sizes: *mut uint32_t = if !u32.is_null() {
        u32.offset((0 as c_int * HISTOGRAMS_PER_BATCH) as isize)
    } else {
        ::core::ptr::null_mut::<uint32_t>()
    };
    let new_clusters: *mut uint32_t = if !u32.is_null() {
        u32.offset((1 as c_int * HISTOGRAMS_PER_BATCH) as isize)
    } else {
        ::core::ptr::null_mut::<uint32_t>()
    };
    let symbols: *mut uint32_t = if !u32.is_null() {
        u32.offset((2 as c_int * HISTOGRAMS_PER_BATCH) as isize)
    } else {
        ::core::ptr::null_mut::<uint32_t>()
    };
    let remap: *mut uint32_t = if !u32.is_null() {
        u32.offset((3 as c_int * HISTOGRAMS_PER_BATCH) as isize)
    } else {
        ::core::ptr::null_mut::<uint32_t>()
    };
    let block_lengths: *mut uint32_t = if !u32.is_null() {
        u32.offset((4 as c_int * HISTOGRAMS_PER_BATCH) as isize)
    } else {
        ::core::ptr::null_mut::<uint32_t>()
    };
    let mut tmp: *mut HistogramDistance = if 2 as c_int > 0 as c_int {
        BrotliAllocate(
            m,
            (2 as size_t).wrapping_mul(::core::mem::size_of::<HistogramDistance>() as size_t),
        ) as *mut HistogramDistance
    } else {
        ::core::ptr::null_mut::<HistogramDistance>()
    };
    if 0 as c_int != 0
        || 0 as c_int != 0
        || 0 as c_int != 0
        || 0 as c_int != 0
        || 0 as c_int != 0
        || 0 as c_int != 0
        || 0 as c_int != 0
        || 0 as c_int != 0
    {
        return;
    }
    memset(
        u32 as *mut c_void,
        0 as c_int,
        num_blocks
            .wrapping_add((4 as c_int * HISTOGRAMS_PER_BATCH) as size_t)
            .wrapping_mul(::core::mem::size_of::<uint32_t>() as size_t),
    );
    let mut block_idx: size_t = 0 as size_t;
    i = 0 as size_t;
    while i < length {
        let ref mut fresh1 = *block_lengths.offset(block_idx as isize);
        *fresh1 = (*fresh1).wrapping_add(1);
        if i.wrapping_add(1 as size_t) == length
            || *block_ids.offset(i as isize) as c_int
                != *block_ids.offset(i.wrapping_add(1 as size_t) as isize) as c_int
        {
            block_idx = block_idx.wrapping_add(1);
        }
        i = i.wrapping_add(1);
    }
    i = 0 as size_t;
    while i < num_blocks {
        let num_to_combine: size_t =
            brotli_min_size_t(num_blocks.wrapping_sub(i), 64 as size_t) as size_t;
        let mut num_new_clusters: size_t = 0;
        let mut j: size_t = 0;
        j = 0 as size_t;
        while j < num_to_combine {
            let mut k: size_t = 0;
            let mut block_length: size_t =
                *block_lengths.offset(i.wrapping_add(j) as isize) as size_t;
            HistogramClearDistance(histograms.offset(j as isize) as *mut HistogramDistance);
            k = 0 as size_t;
            while k < block_length {
                let fresh2 = pos;
                pos = pos.wrapping_add(1);
                HistogramAddDistance(
                    histograms.offset(j as isize) as *mut HistogramDistance,
                    *data.offset(fresh2 as isize) as size_t,
                );
                k = k.wrapping_add(1);
            }
            (*histograms.offset(j as isize)).bit_cost_ = BrotliPopulationCostDistance(
                histograms.offset(j as isize) as *mut HistogramDistance,
            );
            *new_clusters.offset(j as isize) = j as uint32_t;
            *symbols.offset(j as isize) = j as uint32_t;
            *sizes.offset(j as isize) = 1 as uint32_t;
            j = j.wrapping_add(1);
        }
        num_new_clusters = BrotliHistogramCombineDistance(
            histograms,
            tmp,
            sizes,
            symbols,
            new_clusters,
            pairs,
            num_to_combine,
            num_to_combine,
            HISTOGRAMS_PER_BATCH as size_t,
            max_num_pairs,
        );
        if all_histograms_capacity < all_histograms_size.wrapping_add(num_new_clusters) {
            let mut _new_size: size_t = if all_histograms_capacity == 0 as size_t {
                all_histograms_size.wrapping_add(num_new_clusters)
            } else {
                all_histograms_capacity
            };
            let mut new_array: *mut HistogramDistance =
                ::core::ptr::null_mut::<HistogramDistance>();
            while _new_size < all_histograms_size.wrapping_add(num_new_clusters) {
                _new_size = (_new_size as c_ulong)
                    .wrapping_mul(2 as c_ulong) as size_t
                    as size_t;
            }
            new_array = if _new_size > 0 as size_t {
                BrotliAllocate(
                    m,
                    _new_size.wrapping_mul(::core::mem::size_of::<HistogramDistance>() as size_t),
                ) as *mut HistogramDistance
            } else {
                ::core::ptr::null_mut::<HistogramDistance>()
            };
            if 0 as c_int == 0
                && 0 as c_int == 0
                && all_histograms_capacity != 0 as size_t
            {
                memcpy(
                    new_array as *mut c_void,
                    all_histograms as *const c_void,
                    all_histograms_capacity
                        .wrapping_mul(::core::mem::size_of::<HistogramDistance>() as size_t),
                );
            }
            BrotliFree(m, all_histograms as *mut c_void);
            all_histograms = ::core::ptr::null_mut::<HistogramDistance>();
            all_histograms = new_array;
            all_histograms_capacity = _new_size;
        }
        if cluster_size_capacity < cluster_size_size.wrapping_add(num_new_clusters) {
            let mut _new_size_0: size_t = if cluster_size_capacity == 0 as size_t {
                cluster_size_size.wrapping_add(num_new_clusters)
            } else {
                cluster_size_capacity
            };
            let mut new_array_0: *mut uint32_t = ::core::ptr::null_mut::<uint32_t>();
            while _new_size_0 < cluster_size_size.wrapping_add(num_new_clusters) {
                _new_size_0 = (_new_size_0 as c_ulong)
                    .wrapping_mul(2 as c_ulong) as size_t
                    as size_t;
            }
            new_array_0 = if _new_size_0 > 0 as size_t {
                BrotliAllocate(
                    m,
                    _new_size_0.wrapping_mul(::core::mem::size_of::<uint32_t>() as size_t),
                ) as *mut uint32_t
            } else {
                ::core::ptr::null_mut::<uint32_t>()
            };
            if 0 as c_int == 0
                && 0 as c_int == 0
                && cluster_size_capacity != 0 as size_t
            {
                memcpy(
                    new_array_0 as *mut c_void,
                    cluster_size as *const c_void,
                    cluster_size_capacity
                        .wrapping_mul(::core::mem::size_of::<uint32_t>() as size_t),
                );
            }
            BrotliFree(m, cluster_size as *mut c_void);
            cluster_size = ::core::ptr::null_mut::<uint32_t>();
            cluster_size = new_array_0;
            cluster_size_capacity = _new_size_0;
        }
        if 0 as c_int != 0 {
            return;
        }
        j = 0 as size_t;
        while j < num_new_clusters {
            let fresh3 = all_histograms_size;
            all_histograms_size = all_histograms_size.wrapping_add(1);
            *all_histograms.offset(fresh3 as isize) =
                *histograms.offset(*new_clusters.offset(j as isize) as isize);
            let fresh4 = cluster_size_size;
            cluster_size_size = cluster_size_size.wrapping_add(1);
            *cluster_size.offset(fresh4 as isize) =
                *sizes.offset(*new_clusters.offset(j as isize) as isize);
            *remap.offset(*new_clusters.offset(j as isize) as isize) = j as uint32_t;
            j = j.wrapping_add(1);
        }
        j = 0 as size_t;
        while j < num_to_combine {
            *histogram_symbols.offset(i.wrapping_add(j) as isize) = (num_clusters as uint32_t)
                .wrapping_add(*remap.offset(*symbols.offset(j as isize) as isize));
            j = j.wrapping_add(1);
        }
        num_clusters = (num_clusters as c_ulong)
            .wrapping_add(num_new_clusters as c_ulong) as size_t
            as size_t;
        i = (i as c_ulong).wrapping_add(HISTOGRAMS_PER_BATCH as c_ulong)
            as size_t as size_t;
    }
    BrotliFree(m, histograms as *mut c_void);
    histograms = ::core::ptr::null_mut::<HistogramDistance>();
    max_num_pairs = brotli_min_size_t(
        (64 as size_t).wrapping_mul(num_clusters),
        num_clusters
            .wrapping_div(2 as size_t)
            .wrapping_mul(num_clusters),
    );
    if pairs_capacity < max_num_pairs.wrapping_add(1 as size_t) {
        BrotliFree(m, pairs as *mut c_void);
        pairs = ::core::ptr::null_mut::<HistogramPair>();
        pairs = if max_num_pairs.wrapping_add(1 as size_t) > 0 as size_t {
            BrotliAllocate(
                m,
                max_num_pairs
                    .wrapping_add(1 as size_t)
                    .wrapping_mul(::core::mem::size_of::<HistogramPair>() as size_t),
            ) as *mut HistogramPair
        } else {
            ::core::ptr::null_mut::<HistogramPair>()
        };
        if 0 as c_int != 0 || 0 as c_int != 0 {
            return;
        }
    }
    clusters = if num_clusters > 0 as size_t {
        BrotliAllocate(
            m,
            num_clusters.wrapping_mul(::core::mem::size_of::<uint32_t>() as size_t),
        ) as *mut uint32_t
    } else {
        ::core::ptr::null_mut::<uint32_t>()
    };
    if 0 as c_int != 0 || 0 as c_int != 0 {
        return;
    }
    i = 0 as size_t;
    while i < num_clusters {
        *clusters.offset(i as isize) = i as uint32_t;
        i = i.wrapping_add(1);
    }
    num_final_clusters = BrotliHistogramCombineDistance(
        all_histograms,
        tmp,
        cluster_size,
        histogram_symbols,
        clusters,
        pairs,
        num_clusters,
        num_blocks,
        BROTLI_MAX_NUMBER_OF_BLOCK_TYPES as size_t,
        max_num_pairs,
    );
    BrotliFree(m, pairs as *mut c_void);
    pairs = ::core::ptr::null_mut::<HistogramPair>();
    BrotliFree(m, cluster_size as *mut c_void);
    cluster_size = ::core::ptr::null_mut::<uint32_t>();
    new_index = if num_clusters > 0 as size_t {
        BrotliAllocate(
            m,
            num_clusters.wrapping_mul(::core::mem::size_of::<uint32_t>() as size_t),
        ) as *mut uint32_t
    } else {
        ::core::ptr::null_mut::<uint32_t>()
    };
    if 0 as c_int != 0 || 0 as c_int != 0 {
        return;
    }
    i = 0 as size_t;
    while i < num_clusters {
        *new_index.offset(i as isize) = kInvalidIndex;
        i = i.wrapping_add(1);
    }
    pos = 0 as size_t;
    let mut next_index: uint32_t = 0 as uint32_t;
    i = 0 as size_t;
    while i < num_blocks {
        let mut j_0: size_t = 0;
        let mut best_out: uint32_t = 0;
        let mut best_bits: c_double = 0.;
        HistogramClearDistance(tmp);
        j_0 = 0 as size_t;
        while j_0 < *block_lengths.offset(i as isize) as size_t {
            let fresh5 = pos;
            pos = pos.wrapping_add(1);
            HistogramAddDistance(tmp, *data.offset(fresh5 as isize) as size_t);
            j_0 = j_0.wrapping_add(1);
        }
        best_out = if i == 0 as size_t {
            *histogram_symbols.offset(0 as c_int as isize)
        } else {
            *histogram_symbols.offset(i.wrapping_sub(1 as size_t) as isize)
        };
        best_bits = BrotliHistogramBitCostDistanceDistance(
            tmp,
            all_histograms.offset(best_out as isize) as *mut HistogramDistance,
            tmp.offset(1 as c_int as isize),
        );
        j_0 = 0 as size_t;
        while j_0 < num_final_clusters {
            let cur_bits: c_double = BrotliHistogramBitCostDistanceDistance(
                tmp,
                all_histograms.offset(*clusters.offset(j_0 as isize) as isize)
                    as *mut HistogramDistance,
                tmp.offset(1 as c_int as isize),
            ) as c_double;
            if cur_bits < best_bits {
                best_bits = cur_bits;
                best_out = *clusters.offset(j_0 as isize);
            }
            j_0 = j_0.wrapping_add(1);
        }
        *histogram_symbols.offset(i as isize) = best_out;
        if *new_index.offset(best_out as isize) == kInvalidIndex {
            let fresh6 = next_index;
            next_index = next_index.wrapping_add(1);
            *new_index.offset(best_out as isize) = fresh6;
        }
        i = i.wrapping_add(1);
    }
    BrotliFree(m, tmp as *mut c_void);
    tmp = ::core::ptr::null_mut::<HistogramDistance>();
    BrotliFree(m, clusters as *mut c_void);
    clusters = ::core::ptr::null_mut::<uint32_t>();
    BrotliFree(m, all_histograms as *mut c_void);
    all_histograms = ::core::ptr::null_mut::<HistogramDistance>();
    if (*split).types_alloc_size < num_blocks {
        let mut _new_size_1: size_t = if (*split).types_alloc_size == 0 as size_t {
            num_blocks
        } else {
            (*split).types_alloc_size
        };
        let mut new_array_1: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        while _new_size_1 < num_blocks {
            _new_size_1 = (_new_size_1 as c_ulong)
                .wrapping_mul(2 as c_ulong) as size_t
                as size_t;
        }
        new_array_1 = if _new_size_1 > 0 as size_t {
            BrotliAllocate(
                m,
                _new_size_1.wrapping_mul(::core::mem::size_of::<uint8_t>() as size_t),
            ) as *mut uint8_t
        } else {
            ::core::ptr::null_mut::<uint8_t>()
        };
        if 0 as c_int == 0
            && 0 as c_int == 0
            && (*split).types_alloc_size != 0 as size_t
        {
            memcpy(
                new_array_1 as *mut c_void,
                (*split).types as *const c_void,
                (*split)
                    .types_alloc_size
                    .wrapping_mul(::core::mem::size_of::<uint8_t>() as size_t),
            );
        }
        BrotliFree(m, (*split).types as *mut c_void);
        (*split).types = ::core::ptr::null_mut::<uint8_t>();
        (*split).types = new_array_1;
        (*split).types_alloc_size = _new_size_1;
    }
    if (*split).lengths_alloc_size < num_blocks {
        let mut _new_size_2: size_t = if (*split).lengths_alloc_size == 0 as size_t {
            num_blocks
        } else {
            (*split).lengths_alloc_size
        };
        let mut new_array_2: *mut uint32_t = ::core::ptr::null_mut::<uint32_t>();
        while _new_size_2 < num_blocks {
            _new_size_2 = (_new_size_2 as c_ulong)
                .wrapping_mul(2 as c_ulong) as size_t
                as size_t;
        }
        new_array_2 = if _new_size_2 > 0 as size_t {
            BrotliAllocate(
                m,
                _new_size_2.wrapping_mul(::core::mem::size_of::<uint32_t>() as size_t),
            ) as *mut uint32_t
        } else {
            ::core::ptr::null_mut::<uint32_t>()
        };
        if 0 as c_int == 0
            && 0 as c_int == 0
            && (*split).lengths_alloc_size != 0 as size_t
        {
            memcpy(
                new_array_2 as *mut c_void,
                (*split).lengths as *const c_void,
                (*split)
                    .lengths_alloc_size
                    .wrapping_mul(::core::mem::size_of::<uint32_t>() as size_t),
            );
        }
        BrotliFree(m, (*split).lengths as *mut c_void);
        (*split).lengths = ::core::ptr::null_mut::<uint32_t>();
        (*split).lengths = new_array_2;
        (*split).lengths_alloc_size = _new_size_2;
    }
    if 0 as c_int != 0 {
        return;
    }
    let mut cur_length: uint32_t = 0 as uint32_t;
    let mut block_idx_0: size_t = 0 as size_t;
    let mut max_type: uint8_t = 0 as uint8_t;
    i = 0 as size_t;
    while i < num_blocks {
        cur_length = (cur_length as c_uint)
            .wrapping_add(*block_lengths.offset(i as isize) as c_uint)
            as uint32_t as uint32_t;
        if i.wrapping_add(1 as size_t) == num_blocks
            || *histogram_symbols.offset(i as isize)
                != *histogram_symbols.offset(i.wrapping_add(1 as size_t) as isize)
        {
            let id: uint8_t =
                *new_index.offset(*histogram_symbols.offset(i as isize) as isize) as uint8_t;
            *(*split).types.offset(block_idx_0 as isize) = id;
            *(*split).lengths.offset(block_idx_0 as isize) = cur_length;
            max_type = brotli_max_uint8_t(max_type, id);
            cur_length = 0 as uint32_t;
            block_idx_0 = block_idx_0.wrapping_add(1);
        }
        i = i.wrapping_add(1);
    }
    (*split).num_blocks = block_idx_0;
    (*split).num_types = (max_type as size_t).wrapping_add(1 as size_t);
    BrotliFree(m, new_index as *mut c_void);
    new_index = ::core::ptr::null_mut::<uint32_t>();
    BrotliFree(m, u32 as *mut c_void);
    u32 = ::core::ptr::null_mut::<uint32_t>();
    BrotliFree(m, histogram_symbols as *mut c_void);
    histogram_symbols = ::core::ptr::null_mut::<uint32_t>();
}
unsafe extern "C" fn SplitByteVectorDistance(
    mut m: *mut MemoryManager,
    mut data: *const uint16_t,
    length: size_t,
    symbols_per_histogram: size_t,
    max_histograms: size_t,
    sampling_stride_length: size_t,
    block_switch_cost: c_double,
    mut params: *const BrotliEncoderParams,
    mut split: *mut BlockSplit,
) {
    let data_size: size_t = HistogramDataSizeDistance() as size_t;
    let mut histograms: *mut HistogramDistance = ::core::ptr::null_mut::<HistogramDistance>();
    let mut tmp: *mut HistogramDistance = ::core::ptr::null_mut::<HistogramDistance>();
    let mut num_histograms: size_t = length
        .wrapping_div(symbols_per_histogram)
        .wrapping_add(1 as size_t);
    if num_histograms > max_histograms {
        num_histograms = max_histograms;
    }
    if length == 0 as size_t {
        (*split).num_types = 1 as size_t;
        return;
    }
    if length < kMinLengthForBlockSplitting {
        if (*split).types_alloc_size < (*split).num_blocks.wrapping_add(1 as size_t) {
            let mut _new_size: size_t = if (*split).types_alloc_size == 0 as size_t {
                (*split).num_blocks.wrapping_add(1 as size_t)
            } else {
                (*split).types_alloc_size
            };
            let mut new_array: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
            while _new_size < (*split).num_blocks.wrapping_add(1 as size_t) {
                _new_size = (_new_size as c_ulong)
                    .wrapping_mul(2 as c_ulong) as size_t
                    as size_t;
            }
            new_array = if _new_size > 0 as size_t {
                BrotliAllocate(
                    m,
                    _new_size.wrapping_mul(::core::mem::size_of::<uint8_t>() as size_t),
                ) as *mut uint8_t
            } else {
                ::core::ptr::null_mut::<uint8_t>()
            };
            if 0 as c_int == 0
                && 0 as c_int == 0
                && (*split).types_alloc_size != 0 as size_t
            {
                memcpy(
                    new_array as *mut c_void,
                    (*split).types as *const c_void,
                    (*split)
                        .types_alloc_size
                        .wrapping_mul(::core::mem::size_of::<uint8_t>() as size_t),
                );
            }
            BrotliFree(m, (*split).types as *mut c_void);
            (*split).types = ::core::ptr::null_mut::<uint8_t>();
            (*split).types = new_array;
            (*split).types_alloc_size = _new_size;
        }
        if (*split).lengths_alloc_size < (*split).num_blocks.wrapping_add(1 as size_t) {
            let mut _new_size_0: size_t = if (*split).lengths_alloc_size == 0 as size_t {
                (*split).num_blocks.wrapping_add(1 as size_t)
            } else {
                (*split).lengths_alloc_size
            };
            let mut new_array_0: *mut uint32_t = ::core::ptr::null_mut::<uint32_t>();
            while _new_size_0 < (*split).num_blocks.wrapping_add(1 as size_t) {
                _new_size_0 = (_new_size_0 as c_ulong)
                    .wrapping_mul(2 as c_ulong) as size_t
                    as size_t;
            }
            new_array_0 = if _new_size_0 > 0 as size_t {
                BrotliAllocate(
                    m,
                    _new_size_0.wrapping_mul(::core::mem::size_of::<uint32_t>() as size_t),
                ) as *mut uint32_t
            } else {
                ::core::ptr::null_mut::<uint32_t>()
            };
            if 0 as c_int == 0
                && 0 as c_int == 0
                && (*split).lengths_alloc_size != 0 as size_t
            {
                memcpy(
                    new_array_0 as *mut c_void,
                    (*split).lengths as *const c_void,
                    (*split)
                        .lengths_alloc_size
                        .wrapping_mul(::core::mem::size_of::<uint32_t>() as size_t),
                );
            }
            BrotliFree(m, (*split).lengths as *mut c_void);
            (*split).lengths = ::core::ptr::null_mut::<uint32_t>();
            (*split).lengths = new_array_0;
            (*split).lengths_alloc_size = _new_size_0;
        }
        if 0 as c_int != 0 {
            return;
        }
        (*split).num_types = 1 as size_t;
        *(*split).types.offset((*split).num_blocks as isize) = 0 as uint8_t;
        *(*split).lengths.offset((*split).num_blocks as isize) = length as uint32_t;
        (*split).num_blocks = (*split).num_blocks.wrapping_add(1);
        return;
    }
    histograms = if num_histograms.wrapping_add(1 as size_t) > 0 as size_t {
        BrotliAllocate(
            m,
            num_histograms
                .wrapping_add(1 as size_t)
                .wrapping_mul(::core::mem::size_of::<HistogramDistance>() as size_t),
        ) as *mut HistogramDistance
    } else {
        ::core::ptr::null_mut::<HistogramDistance>()
    };
    tmp = histograms.offset(num_histograms as isize);
    if 0 as c_int != 0 || 0 as c_int != 0 {
        return;
    }
    InitialEntropyCodesDistance(
        data,
        length,
        sampling_stride_length,
        num_histograms,
        histograms,
    );
    RefineEntropyCodesDistance(
        data,
        length,
        sampling_stride_length,
        num_histograms,
        histograms,
        tmp,
    );
    let mut block_ids: *mut uint8_t = if length > 0 as size_t {
        BrotliAllocate(
            m,
            length.wrapping_mul(::core::mem::size_of::<uint8_t>() as size_t),
        ) as *mut uint8_t
    } else {
        ::core::ptr::null_mut::<uint8_t>()
    };
    let mut num_blocks: size_t = 0 as size_t;
    let bitmaplen: size_t = num_histograms.wrapping_add(7 as size_t) >> 3 as c_int;
    let mut insert_cost: *mut c_double =
        if data_size.wrapping_mul(num_histograms) > 0 as size_t {
            BrotliAllocate(
                m,
                data_size
                    .wrapping_mul(num_histograms)
                    .wrapping_mul(::core::mem::size_of::<c_double>() as size_t),
            ) as *mut c_double
        } else {
            ::core::ptr::null_mut::<c_double>()
        };
    let mut cost: *mut c_double = if num_histograms > 0 as size_t {
        BrotliAllocate(
            m,
            num_histograms.wrapping_mul(::core::mem::size_of::<c_double>() as size_t),
        ) as *mut c_double
    } else {
        ::core::ptr::null_mut::<c_double>()
    };
    let mut switch_signal: *mut uint8_t = if length.wrapping_mul(bitmaplen) > 0 as size_t {
        BrotliAllocate(
            m,
            length
                .wrapping_mul(bitmaplen)
                .wrapping_mul(::core::mem::size_of::<uint8_t>() as size_t),
        ) as *mut uint8_t
    } else {
        ::core::ptr::null_mut::<uint8_t>()
    };
    let mut new_id: *mut uint16_t = if num_histograms > 0 as size_t {
        BrotliAllocate(
            m,
            num_histograms.wrapping_mul(::core::mem::size_of::<uint16_t>() as size_t),
        ) as *mut uint16_t
    } else {
        ::core::ptr::null_mut::<uint16_t>()
    };
    let iters: size_t = (if (*params).quality < HQ_ZOPFLIFICATION_QUALITY {
        3 as c_int
    } else {
        10 as c_int
    }) as size_t;
    let mut i: size_t = 0;
    if 0 as c_int != 0
        || 0 as c_int != 0
        || 0 as c_int != 0
        || 0 as c_int != 0
        || 0 as c_int != 0
        || 0 as c_int != 0
    {
        return;
    }
    i = 0 as size_t;
    while i < iters {
        num_blocks = FindBlocksDistance(
            data,
            length,
            block_switch_cost,
            num_histograms,
            histograms,
            insert_cost,
            cost,
            switch_signal,
            block_ids,
        );
        num_histograms = RemapBlockIdsDistance(block_ids, length, new_id, num_histograms);
        BuildBlockHistogramsDistance(data, length, block_ids, num_histograms, histograms);
        i = i.wrapping_add(1);
    }
    BrotliFree(m, insert_cost as *mut c_void);
    insert_cost = ::core::ptr::null_mut::<c_double>();
    BrotliFree(m, cost as *mut c_void);
    cost = ::core::ptr::null_mut::<c_double>();
    BrotliFree(m, switch_signal as *mut c_void);
    switch_signal = ::core::ptr::null_mut::<uint8_t>();
    BrotliFree(m, new_id as *mut c_void);
    new_id = ::core::ptr::null_mut::<uint16_t>();
    BrotliFree(m, histograms as *mut c_void);
    histograms = ::core::ptr::null_mut::<HistogramDistance>();
    ClusterBlocksDistance(m, data, length, num_blocks, block_ids, split);
    if 0 as c_int != 0 {
        return;
    }
    BrotliFree(m, block_ids as *mut c_void);
    block_ids = ::core::ptr::null_mut::<uint8_t>();
}
