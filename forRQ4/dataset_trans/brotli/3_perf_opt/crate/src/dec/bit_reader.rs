use core::ffi::*;
use crate::src::c_inlined_fns::BrotliBitReaderGetAvailIn;
use crate::src::c_inlined_fns::BrotliBitReaderLoadBits;
use crate::src::c_inlined_fns::BrotliBitReaderSetInput;
use crate::src::c_inlined_fns::BrotliDropBits;
use crate::src::c_inlined_fns::BrotliGetAvailableBits;
use crate::src::c_inlined_fns::BrotliGetBitsUnmasked;
pub use crate::src::c_inlined_fns::*;
pub use crate::src::c_consts::*;
pub use crate::src::c_structs::*;
pub use crate::src::c_types::*;

#[no_mangle]
pub static mut kBrotliBitMask: [uint64_t; 33] = [
    0 as c_int as uint64_t,
    0x1 as c_int as uint64_t,
    0x3 as c_int as uint64_t,
    0x7 as c_int as uint64_t,
    0xf as c_int as uint64_t,
    0x1f as c_int as uint64_t,
    0x3f as c_int as uint64_t,
    0x7f as c_int as uint64_t,
    0xff as c_int as uint64_t,
    0x1ff as c_int as uint64_t,
    0x3ff as c_int as uint64_t,
    0x7ff as c_int as uint64_t,
    0xfff as c_int as uint64_t,
    0x1fff as c_int as uint64_t,
    0x3fff as c_int as uint64_t,
    0x7fff as c_int as uint64_t,
    0xffff as c_int as uint64_t,
    0x1ffff as c_int as uint64_t,
    0x3ffff as c_int as uint64_t,
    0x7ffff as c_int as uint64_t,
    0xfffff as c_int as uint64_t,
    0x1fffff as c_int as uint64_t,
    0x3fffff as c_int as uint64_t,
    0x7fffff as c_int as uint64_t,
    0xffffff as c_int as uint64_t,
    0x1ffffff as c_int as uint64_t,
    0x3ffffff as c_int as uint64_t,
    0x7ffffff as c_int as uint64_t,
    0xfffffff as c_int as uint64_t,
    0x1fffffff as c_int as uint64_t,
    0x3fffffff as c_int as uint64_t,
    0x7fffffff as c_int as uint64_t,
    0xffffffff as c_uint as uint64_t,
];
#[inline]
pub unsafe fn BrotliInitBitReader(mut br: *mut BrotliBitReader) {
    let br_view: &mut BrotliBitReader = unsafe { &mut *br };
    br_view.val_ = 0 as uint64_t;
    br_view.bit_pos_ = 0 as uint64_t;
}
#[inline]
pub unsafe fn BrotliWarmupBitReader(mut br: *mut BrotliBitReader) -> c_int {
    let mut aligned_read_mask: size_t = (::core::mem::size_of::<uint64_t>() as size_t
        >> 1 as c_int)
        .wrapping_sub(1 as size_t);
    if 1 as c_int != 0 {
        aligned_read_mask = 0 as size_t;
    }
    if BrotliGetAvailableBits(br) == 0 as uint64_t {
        (*br).val_ = 0 as uint64_t;
        if BrotliPullByte(br) == 0 {
            return BROTLI_FALSE;
        }
    }
    while (*br).next_in as size_t & aligned_read_mask != 0 as size_t {
        if BrotliPullByte(br) == 0 {
            return BROTLI_TRUE;
        }
    }
    return BROTLI_TRUE;
}
#[inline(never)]
pub unsafe fn BrotliSafeReadBits32Slow(
    mut br: *mut BrotliBitReader,
    mut n_bits: uint64_t,
    mut val: *mut uint64_t,
) -> c_int {
    let mut low_val: uint64_t = 0;
    let mut high_val: uint64_t = 0;
    let mut memento: BrotliBitReaderState = BrotliBitReaderState {
        val_: 0,
        bit_pos_: 0,
        next_in: ::core::ptr::null::<uint8_t>(),
        avail_in: 0,
    };
    BrotliBitReaderSaveState(br, &raw mut memento);
    if BrotliSafeReadBits(br, 16 as uint64_t, &raw mut low_val) == 0
        || BrotliSafeReadBits(br, n_bits.wrapping_sub(16 as uint64_t), &raw mut high_val) == 0
    {
        BrotliBitReaderRestoreState(br, &raw mut memento);
        return BROTLI_FALSE;
    }
    *val = low_val | high_val << 16 as c_int;
    return BROTLI_TRUE;
}

#[inline(always)]
extern "C" fn BitMask(mut n: uint64_t) -> uint64_t { unsafe {
    if 0 != 0 || 0 as c_int != 0 {
        return !(!(0 as c_int as uint64_t) << n);
    } else {
        return kBrotliBitMask[n as usize];
    };
} }

#[inline(always)]
unsafe extern "C" fn BrotliBitReaderSaveState(
    from: *mut BrotliBitReader,
    mut to: *mut BrotliBitReaderState,
) {
    (*to).val_ = (*from).val_;
    (*to).bit_pos_ = (*from).bit_pos_;
    (*to).next_in = (*from).next_in;
    (*to).avail_in = BrotliBitReaderGetAvailIn(from);
}

#[inline(always)]
unsafe extern "C" fn BrotliBitReaderRestoreState(
    to: *mut BrotliBitReader,
    mut from: *mut BrotliBitReaderState,
) {
    (*to).val_ = (*from).val_;
    (*to).bit_pos_ = (*from).bit_pos_;
    (*to).next_in = (*from).next_in;
    BrotliBitReaderSetInput(to, (*from).next_in, (*from).avail_in);
}

#[inline(always)]
unsafe extern "C" fn BrotliPullByte(br: *mut BrotliBitReader) -> c_int {
    if (*br).next_in == (*br).last_in {
        return BROTLI_FALSE;
    }
    (*br).val_ = BrotliBitReaderLoadBits(
        (*br).val_,
        *(*br).next_in as uint64_t,
        8 as uint64_t,
        (*br).bit_pos_,
    );
    (*br).bit_pos_ = ((*br).bit_pos_ as c_ulong)
        .wrapping_add(8 as c_ulong) as uint64_t as uint64_t;
    (*br).next_in = (*br).next_in.offset(1);
    return BROTLI_TRUE;
}

#[inline(always)]
unsafe extern "C" fn BrotliTakeBits(
    br: *mut BrotliBitReader,
    mut n_bits: uint64_t,
    mut val: *mut uint64_t,
) {
    *val = BrotliGetBitsUnmasked(br) & BitMask(n_bits);
    BrotliDropBits(br, n_bits);
}
#[inline(always)]
unsafe extern "C" fn BrotliSafeReadBits(
    br: *mut BrotliBitReader,
    mut n_bits: uint64_t,
    mut val: *mut uint64_t,
) -> c_int {
    while BrotliGetAvailableBits(br) < n_bits {
        if BrotliPullByte(br) == 0 {
            return BROTLI_FALSE;
        }
    }
    BrotliTakeBits(br, n_bits, val);
    return BROTLI_TRUE;
}
