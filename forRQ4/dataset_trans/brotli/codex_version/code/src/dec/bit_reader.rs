pub type size_t = usize;
pub type __uint8_t = u8;
pub type __uint64_t = u64;
pub type uint8_t = __uint8_t;
pub type uint64_t = __uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BrotliBitReader {
    pub val_: uint64_t,
    pub bit_pos_: uint64_t,
    pub next_in: *const uint8_t,
    pub guard_in: *const uint8_t,
    pub last_in: *const uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BrotliBitReaderState {
    pub val_: uint64_t,
    pub bit_pos_: uint64_t,
    pub next_in: *const uint8_t,
    pub avail_in: size_t,
}
pub const BROTLI_TRUE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const BROTLI_FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const BROTLI_UNALIGNED_READ_FAST: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int != 0) as ::core::ffi::c_int;
pub const BROTLI_HAS_UBFX: ::core::ffi::c_int =
    (0 as ::core::ffi::c_int != 0) as ::core::ffi::c_int;
#[no_mangle]
pub static mut kBrotliBitMask: [uint64_t; 33] = [
    0 as ::core::ffi::c_int as uint64_t,
    0x1 as ::core::ffi::c_int as uint64_t,
    0x3 as ::core::ffi::c_int as uint64_t,
    0x7 as ::core::ffi::c_int as uint64_t,
    0xf as ::core::ffi::c_int as uint64_t,
    0x1f as ::core::ffi::c_int as uint64_t,
    0x3f as ::core::ffi::c_int as uint64_t,
    0x7f as ::core::ffi::c_int as uint64_t,
    0xff as ::core::ffi::c_int as uint64_t,
    0x1ff as ::core::ffi::c_int as uint64_t,
    0x3ff as ::core::ffi::c_int as uint64_t,
    0x7ff as ::core::ffi::c_int as uint64_t,
    0xfff as ::core::ffi::c_int as uint64_t,
    0x1fff as ::core::ffi::c_int as uint64_t,
    0x3fff as ::core::ffi::c_int as uint64_t,
    0x7fff as ::core::ffi::c_int as uint64_t,
    0xffff as ::core::ffi::c_int as uint64_t,
    0x1ffff as ::core::ffi::c_int as uint64_t,
    0x3ffff as ::core::ffi::c_int as uint64_t,
    0x7ffff as ::core::ffi::c_int as uint64_t,
    0xfffff as ::core::ffi::c_int as uint64_t,
    0x1fffff as ::core::ffi::c_int as uint64_t,
    0x3fffff as ::core::ffi::c_int as uint64_t,
    0x7fffff as ::core::ffi::c_int as uint64_t,
    0xffffff as ::core::ffi::c_int as uint64_t,
    0x1ffffff as ::core::ffi::c_int as uint64_t,
    0x3ffffff as ::core::ffi::c_int as uint64_t,
    0x7ffffff as ::core::ffi::c_int as uint64_t,
    0xfffffff as ::core::ffi::c_int as uint64_t,
    0x1fffffff as ::core::ffi::c_int as uint64_t,
    0x3fffffff as ::core::ffi::c_int as uint64_t,
    0x7fffffff as ::core::ffi::c_int as uint64_t,
    0xffffffff as ::core::ffi::c_uint as uint64_t,
];
#[no_mangle]
pub unsafe extern "C" fn BrotliInitBitReader(mut br: *mut BrotliBitReader) {
    (*br).val_ = 0 as uint64_t;
    (*br).bit_pos_ = 0 as uint64_t;
}
#[no_mangle]
pub unsafe extern "C" fn BrotliWarmupBitReader(mut br: *mut BrotliBitReader) -> ::core::ffi::c_int {
    let mut aligned_read_mask: size_t = (::core::mem::size_of::<uint64_t>() as size_t
        >> 1 as ::core::ffi::c_int)
        .wrapping_sub(1 as size_t);
    if 1 as ::core::ffi::c_int != 0 {
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
#[no_mangle]
#[inline(never)]
pub unsafe extern "C" fn BrotliSafeReadBits32Slow(
    mut br: *mut BrotliBitReader,
    mut n_bits: uint64_t,
    mut val: *mut uint64_t,
) -> ::core::ffi::c_int {
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
    *val = low_val | high_val << 16 as ::core::ffi::c_int;
    return BROTLI_TRUE;
}
pub const BROTLI_FAST_INPUT_SLACK: ::core::ffi::c_int = 28 as ::core::ffi::c_int;
#[inline(always)]
unsafe extern "C" fn BitMask(mut n: uint64_t) -> uint64_t {
    if 0 != 0 || 0 as ::core::ffi::c_int != 0 {
        return !(!(0 as ::core::ffi::c_int as uint64_t) << n);
    } else {
        return kBrotliBitMask[n as usize];
    };
}
#[inline(always)]
unsafe extern "C" fn BrotliBitReaderGetAvailIn(br: *mut BrotliBitReader) -> size_t {
    return (*br).last_in.offset_from((*br).next_in) as ::core::ffi::c_long as size_t;
}
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
unsafe extern "C" fn BrotliBitReaderSetInput(
    br: *mut BrotliBitReader,
    mut next_in: *const uint8_t,
    mut avail_in: size_t,
) {
    (*br).next_in = next_in;
    (*br).last_in = if avail_in == 0 as size_t {
        next_in
    } else {
        next_in.offset(avail_in as isize)
    };
    if avail_in.wrapping_add(1 as size_t) > BROTLI_FAST_INPUT_SLACK as size_t {
        (*br).guard_in = next_in.offset(
            avail_in
                .wrapping_add(1 as size_t)
                .wrapping_sub(BROTLI_FAST_INPUT_SLACK as size_t) as isize,
        );
    } else {
        (*br).guard_in = next_in;
    };
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
unsafe extern "C" fn BrotliGetAvailableBits(mut br: *const BrotliBitReader) -> uint64_t {
    return (*br).bit_pos_;
}
#[inline(always)]
unsafe extern "C" fn BrotliBitReaderLoadBits(
    mut val: uint64_t,
    mut new_bits: uint64_t,
    mut count: uint64_t,
    mut offset: uint64_t,
) -> uint64_t {
    return val | new_bits << offset;
}
#[inline(always)]
unsafe extern "C" fn BrotliPullByte(br: *mut BrotliBitReader) -> ::core::ffi::c_int {
    if (*br).next_in == (*br).last_in {
        return BROTLI_FALSE;
    }
    (*br).val_ = BrotliBitReaderLoadBits(
        (*br).val_,
        *(*br).next_in as uint64_t,
        8 as uint64_t,
        (*br).bit_pos_,
    );
    (*br).bit_pos_ = ((*br).bit_pos_ as ::core::ffi::c_ulong)
        .wrapping_add(8 as ::core::ffi::c_ulong) as uint64_t as uint64_t;
    (*br).next_in = (*br).next_in.offset(1);
    return BROTLI_TRUE;
}
#[inline(always)]
unsafe extern "C" fn BrotliGetBitsUnmasked(br: *mut BrotliBitReader) -> uint64_t {
    return (*br).val_;
}
#[inline(always)]
unsafe extern "C" fn BrotliDropBits(br: *mut BrotliBitReader, mut n_bits: uint64_t) {
    (*br).bit_pos_ = ((*br).bit_pos_ as ::core::ffi::c_ulong)
        .wrapping_sub(n_bits as ::core::ffi::c_ulong) as uint64_t as uint64_t;
    (*br).val_ >>= n_bits;
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
) -> ::core::ffi::c_int {
    while BrotliGetAvailableBits(br) < n_bits {
        if BrotliPullByte(br) == 0 {
            return BROTLI_FALSE;
        }
    }
    BrotliTakeBits(br, n_bits, val);
    return BROTLI_TRUE;
}
