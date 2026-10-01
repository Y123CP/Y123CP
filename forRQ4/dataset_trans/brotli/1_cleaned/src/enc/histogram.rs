use core::ffi::*;
pub use crate::src::c_inlined_fns::*;
pub use crate::src::ffi::*;
pub use crate::src::c_consts::*;
pub use crate::src::c_structs::*;
pub use crate::src::c_types::*;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Command {
    pub insert_len_: uint32_t,
    pub copy_len_: uint32_t,
    pub dist_extra_: uint32_t,
    pub cmd_prefix_: uint16_t,
    pub dist_prefix_: uint16_t,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct BlockSplitIterator {
    pub split_: *const BlockSplit,
    pub idx_: size_t,
    pub type_: size_t,
    pub length_: size_t,
}

#[inline(always)]
unsafe extern "C" fn CommandDistanceContext(mut self_0: *const Command) -> uint32_t {
    let mut r: uint32_t =
        ((*self_0).cmd_prefix_ as c_int >> 6 as c_int) as uint32_t;
    let mut c: uint32_t =
        ((*self_0).cmd_prefix_ as c_int & 7 as c_int) as uint32_t;
    if (r == 0 as uint32_t || r == 2 as uint32_t || r == 4 as uint32_t || r == 7 as uint32_t)
        && c <= 2 as uint32_t
    {
        return c;
    }
    return 3 as uint32_t;
}
#[inline(always)]
unsafe extern "C" fn CommandCopyLen(mut self_0: *const Command) -> uint32_t {
    return (*self_0).copy_len_ & 0x1ffffff as uint32_t;
}

unsafe extern "C" fn InitBlockSplitIterator(
    mut self_0: *mut BlockSplitIterator,
    mut split: *const BlockSplit,
) {
    (*self_0).split_ = split;
    (*self_0).idx_ = 0 as size_t;
    (*self_0).type_ = 0 as size_t;
    (*self_0).length_ = (if !(*split).lengths.is_null() {
        *(*split).lengths.offset(0 as c_int as isize)
    } else {
        0 as uint32_t
    }) as size_t;
}
unsafe extern "C" fn BlockSplitIteratorNext(mut self_0: *mut BlockSplitIterator) {
    if (*self_0).length_ == 0 as size_t {
        (*self_0).idx_ = (*self_0).idx_.wrapping_add(1);
        (*self_0).type_ = *(*(*self_0).split_).types.offset((*self_0).idx_ as isize) as size_t;
        (*self_0).length_ = *(*(*self_0).split_).lengths.offset((*self_0).idx_ as isize) as size_t;
    }
    (*self_0).length_ = (*self_0).length_.wrapping_sub(1);
}
#[no_mangle]
pub unsafe extern "C" fn BrotliBuildHistogramsWithContext(
    mut cmds: *const Command,
    num_commands: size_t,
    mut literal_split: *const BlockSplit,
    mut insert_and_copy_split: *const BlockSplit,
    mut dist_split: *const BlockSplit,
    mut ringbuffer: *const uint8_t,
    mut start_pos: size_t,
    mut mask: size_t,
    mut prev_byte: uint8_t,
    mut prev_byte2: uint8_t,
    mut context_modes: *const ContextType,
    mut literal_histograms: *mut HistogramLiteral,
    mut insert_and_copy_histograms: *mut HistogramCommand,
    mut copy_dist_histograms: *mut HistogramDistance,
) {
    let mut pos: size_t = start_pos;
    let mut literal_it: BlockSplitIterator = BlockSplitIterator {
        split_: ::core::ptr::null::<BlockSplit>(),
        idx_: 0,
        type_: 0,
        length_: 0,
    };
    let mut insert_and_copy_it: BlockSplitIterator = BlockSplitIterator {
        split_: ::core::ptr::null::<BlockSplit>(),
        idx_: 0,
        type_: 0,
        length_: 0,
    };
    let mut dist_it: BlockSplitIterator = BlockSplitIterator {
        split_: ::core::ptr::null::<BlockSplit>(),
        idx_: 0,
        type_: 0,
        length_: 0,
    };
    let mut i: size_t = 0;
    InitBlockSplitIterator(&raw mut literal_it, literal_split);
    InitBlockSplitIterator(&raw mut insert_and_copy_it, insert_and_copy_split);
    InitBlockSplitIterator(&raw mut dist_it, dist_split);
    i = 0 as size_t;
    while i < num_commands {
        let mut cmd: *const Command = cmds.offset(i as isize) as *const Command;
        let mut j: size_t = 0;
        BlockSplitIteratorNext(&raw mut insert_and_copy_it);
        HistogramAddCommand(
            insert_and_copy_histograms.offset(insert_and_copy_it.type_ as isize)
                as *mut HistogramCommand,
            (*cmd).cmd_prefix_ as size_t,
        );
        j = (*cmd).insert_len_ as size_t;
        while j != 0 as size_t {
            let mut context: size_t = 0;
            BlockSplitIteratorNext(&raw mut literal_it);
            context = literal_it.type_;
            if !context_modes.is_null() {
                let mut lut: ContextLut = (&raw const _kBrotliContextLookupTable as *const uint8_t)
                    .offset(
                        ((*context_modes.offset(context as isize) as c_uint)
                            << 9 as c_int) as isize,
                    ) as ContextLut;
                context = (context << BROTLI_LITERAL_CONTEXT_BITS).wrapping_add(
                    (*lut.offset(prev_byte as isize) as c_int
                        | *lut
                            .offset(256 as c_int as isize)
                            .offset(prev_byte2 as isize)
                            as c_int) as size_t,
                );
            }
            HistogramAddLiteral(
                literal_histograms.offset(context as isize) as *mut HistogramLiteral,
                *ringbuffer.offset((pos & mask) as isize) as size_t,
            );
            prev_byte2 = prev_byte;
            prev_byte = *ringbuffer.offset((pos & mask) as isize);
            pos = pos.wrapping_add(1);
            j = j.wrapping_sub(1);
        }
        pos = (pos as c_ulong)
            .wrapping_add(CommandCopyLen(cmd) as c_ulong) as size_t
            as size_t;
        if CommandCopyLen(cmd) != 0 {
            prev_byte2 = *ringbuffer.offset((pos.wrapping_sub(2 as size_t) & mask) as isize);
            prev_byte = *ringbuffer.offset((pos.wrapping_sub(1 as size_t) & mask) as isize);
            if (*cmd).cmd_prefix_ as c_int >= 128 as c_int {
                let mut context_0: size_t = 0;
                BlockSplitIteratorNext(&raw mut dist_it);
                context_0 = (dist_it.type_ << BROTLI_DISTANCE_CONTEXT_BITS)
                    .wrapping_add(CommandDistanceContext(cmd) as size_t);
                HistogramAddDistance(
                    copy_dist_histograms.offset(context_0 as isize) as *mut HistogramDistance,
                    ((*cmd).dist_prefix_ as c_int & 0x3ff as c_int)
                        as size_t,
                );
            }
        }
        i = i.wrapping_add(1);
    }
}
