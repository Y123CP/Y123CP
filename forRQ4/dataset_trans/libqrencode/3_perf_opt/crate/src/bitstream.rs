use core::ffi::*;
pub use crate::src::ffi::*;
pub use crate::src::c_consts::*;
pub use crate::src::c_structs::*;
pub use crate::src::c_types::*;

pub const DEFAULT_BUFSIZE: c_int = 128 as c_int;
#[inline]
pub fn BitStream_new() -> *mut BitStream { unsafe {
    let mut bstream: *mut BitStream = ::core::ptr::null_mut::<BitStream>();
    bstream = malloc(::core::mem::size_of::<BitStream>() as size_t) as *mut BitStream;
    if bstream.is_null() {
        return ::core::ptr::null_mut::<BitStream>();
    }
    (*bstream).length = 0 as size_t;
    (*bstream).data = malloc(DEFAULT_BUFSIZE as size_t) as *mut c_uchar;
    if (*bstream).data.is_null() {
        free(bstream as *mut c_void);
        return ::core::ptr::null_mut::<BitStream>();
    }
    (*bstream).datasize = DEFAULT_BUFSIZE as size_t;
    return bstream;
} }
unsafe fn BitStream_expand(mut bstream: *mut BitStream) -> c_int {
    let bstream_view: &mut BitStream = unsafe { &mut *bstream };
    let mut data: *mut c_uchar = ::core::ptr::null_mut::<c_uchar>();
    data = realloc(
        bstream_view.data as *mut c_void,
        bstream_view.datasize.wrapping_mul(2 as size_t),
    ) as *mut c_uchar;
    if data.is_null() {
        return -(1 as c_int);
    }
    bstream_view.data = data;
    bstream_view.datasize = bstream_view.datasize.wrapping_mul(2 as size_t);
    return 0 as c_int;
}
unsafe fn BitStream_writeNum(
    mut dest: *mut c_uchar,
    mut bits: size_t,
    mut num: c_uint,
) {
    let mut mask: c_uint = 0;
    let mut i: size_t = 0;
    let mut p: *mut c_uchar = ::core::ptr::null_mut::<c_uchar>();
    p = dest;
    mask = (1 as c_uint) << bits.wrapping_sub(1 as size_t);
    i = 0 as size_t;
    while i < bits {
        if num & mask != 0 {
            *p = 1 as c_uchar;
        } else {
            *p = 0 as c_uchar;
        }
        p = p.offset(1);
        mask = mask >> 1 as c_int;
        i = i.wrapping_add(1);
    }
}
// Applied rules: [III④, C10, C3]
// Skipped rules: []
unsafe fn BitStream_writeBytes(
    mut dest: *mut c_uchar,
    mut size: size_t,
    mut data: *mut c_uchar,
) {
    let size_usize = size as usize;
    let dest_len = size_usize.wrapping_mul(8);

    // SAFETY: `size` is the explicit caller-provided source length; the caller guarantees
    // `data` is non-null and points to at least `size` readable bytes for this routine, and
    // no mutable reference aliases that region for the duration of this reborrow.
    let data_view: &[c_uchar] = unsafe { core::slice::from_raw_parts(data, size_usize) };
    // SAFETY: the original loop writes exactly 8 bytes to `dest` for each of the `size`
    // source bytes, so the required destination length is `8 * size`; the caller guarantees
    // `dest` is non-null and points to at least that many writable bytes, and that this
    // mutable region does not alias the source region for the duration of this reborrow.
    let dest_view: &mut [c_uchar] = unsafe { core::slice::from_raw_parts_mut(dest, dest_len) };

    const LANES: u64 = u64::from_le_bytes([0x01; 8]);
    const HALF: u64 = u64::from_le_bytes([0x7f; 8]);
    const SELECT: u64 = u64::from_le_bytes([0x80, 0x40, 0x20, 0x10, 0x08, 0x04, 0x02, 0x01]);

    for (src, out_chunk) in data_view.iter().copied().zip(dest_view.chunks_exact_mut(8)) {
        let spread = (src as u64).wrapping_mul(LANES);
        let ones = ((spread & SELECT).wrapping_add(HALF) >> 7) & LANES;
        out_chunk.copy_from_slice(&ones.to_le_bytes());
    }
}
#[inline]
pub unsafe fn BitStream_append(
    mut bstream: *mut BitStream,
    mut arg: *mut BitStream,
) -> c_int {
    let mut ret: c_int = 0;
    if arg.is_null() {
        return -(1 as c_int);
    }
    if (*arg).length == 0 as size_t {
        return 0 as c_int;
    }
    while (*bstream).length.wrapping_add((*arg).length) > (*bstream).datasize {
        ret = BitStream_expand(bstream);
        if ret < 0 as c_int {
            return ret;
        }
    }
    memcpy(
        (*bstream).data.offset((*bstream).length as isize) as *mut c_void,
        (*arg).data as *const c_void,
        (*arg).length,
    );
    (*bstream).length = (*bstream).length.wrapping_add((*arg).length);
    return 0 as c_int;
}
#[inline]
pub unsafe fn BitStream_appendNum(
    mut bstream: *mut BitStream,
    mut bits: size_t,
    mut num: c_uint,
) -> c_int {
    let mut ret: c_int = 0;
    if bits == 0 as size_t {
        return 0 as c_int;
    }
    while (*bstream).datasize.wrapping_sub((*bstream).length) < bits {
        ret = BitStream_expand(bstream);
        if ret < 0 as c_int {
            return ret;
        }
    }
    BitStream_writeNum(
        (*bstream).data.offset((*bstream).length as isize),
        bits,
        num,
    );
    (*bstream).length = (*bstream).length.wrapping_add(bits);
    return 0 as c_int;
}
#[inline]
pub unsafe fn BitStream_appendBytes(
    mut bstream: *mut BitStream,
    mut size: size_t,
    mut data: *mut c_uchar,
) -> c_int {
    let mut ret: c_int = 0;
    if size == 0 as size_t {
        return 0 as c_int;
    }
    while (*bstream).datasize.wrapping_sub((*bstream).length) < size.wrapping_mul(8 as size_t) {
        ret = BitStream_expand(bstream);
        if ret < 0 as c_int {
            return ret;
        }
    }
    BitStream_writeBytes(
        (*bstream).data.offset((*bstream).length as isize),
        size,
        data,
    );
    (*bstream).length = (*bstream)
        .length
        .wrapping_add(size.wrapping_mul(8 as size_t));
    return 0 as c_int;
}
// Applied rules: [III④, C10, C3]
// Skipped rules: [III②: Hit [4] is a `malloc` whose pointer escapes by being returned to the caller; alloc/free are not paired in this function, so RAII container rewrite is out of scope.]
#[inline]
pub unsafe fn BitStream_toByte(
    mut bstream: *mut BitStream,
) -> *mut c_uchar {
    let bstream_view: &BitStream = unsafe { &*bstream };
    let size: size_t = bstream_view.length;
    if size == 0 as size_t {
        return ::core::ptr::null_mut::<c_uchar>();
    }

    let out_len = size.wrapping_add(7 as size_t).wrapping_div(8 as size_t);
    let data = malloc(out_len) as *mut c_uchar;
    if data.is_null() {
        return ::core::ptr::null_mut::<c_uchar>();
    }

    let bytes: size_t = size.wrapping_div(8 as size_t);
    let oddbits: size_t = size & 7 as size_t;

    // SAFETY: `bstream_view.data` is read for exactly `size` bytes, where `size`
    // comes from the stable `BitStream.length` field and is not modified here;
    // the caller of this unsafe function guarantees `data` is non-null and valid
    // for `size` bytes, and this shared slice does not alias the unique mutable
    // output slice created below.
    let src = unsafe { ::core::slice::from_raw_parts(bstream_view.data as *const c_uchar, size) };
    // SAFETY: `data` was just allocated by `malloc(out_len)` and checked non-null;
    // `out_len` is the exact output buffer length used by this function, and no
    // other reference to that allocation is created while this mutable slice lives.
    let dst = unsafe { ::core::slice::from_raw_parts_mut(data, out_len) };

    for (out, chunk) in dst[..bytes]
        .iter_mut()
        .zip(src[..bytes.wrapping_mul(8 as size_t)].chunks_exact(8))
    {
        let word = u64::from_be_bytes([
            chunk[0], chunk[1], chunk[2], chunk[3], chunk[4], chunk[5], chunk[6], chunk[7],
        ]);
        const GATHER: u64 =
            u64::from_le_bytes([0x80, 0x40, 0x20, 0x10, 0x08, 0x04, 0x02, 0x01]);
        *out = (word.wrapping_mul(GATHER) >> 56) as c_uchar;
    }

    if oddbits > 0 as size_t {
        let mut v: c_uchar = 0;
        let tail = &src[bytes.wrapping_mul(8 as size_t)..bytes.wrapping_mul(8 as size_t).wrapping_add(oddbits)];
        for &bit in tail.iter() {
            v = ((v as c_int) << 1 as c_int) as c_uchar;
            v = (v as c_int | bit as c_int) as c_uchar;
        }
        dst[bytes] = ((v as c_int) << (8 as size_t).wrapping_sub(oddbits)) as c_uchar;
    }

    data
}
#[inline]
pub unsafe fn BitStream_free(mut bstream: *mut BitStream) {
    if !bstream.is_null() {
        free((*bstream).data as *mut c_void);
        free(bstream as *mut c_void);
    }
}
