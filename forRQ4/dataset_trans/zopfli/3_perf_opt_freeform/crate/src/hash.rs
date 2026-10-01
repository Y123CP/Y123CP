use core::ffi::*;
pub use crate::src::ffi::*;
pub use crate::src::c_consts::*;
pub use crate::src::c_structs::*;
pub use crate::src::c_types::*;

pub const HASH_SHIFT: c_int = 5 as c_int;
pub const HASH_MASK: c_int = 32767 as c_int;
#[inline]
pub unsafe fn ZopfliAllocHash(mut window_size: size_t, mut h: *mut ZopfliHash) {
    let h_view: &mut ZopfliHash = unsafe { &mut *h };
    h_view.head = malloc(
        (::core::mem::size_of::<c_int>() as size_t)
            .wrapping_mul(65536 as c_int as size_t),
    ) as *mut c_int;
    h_view.prev = malloc(
        (::core::mem::size_of::<c_ushort>() as size_t).wrapping_mul(window_size),
    ) as *mut c_ushort;
    h_view.hashval =
        malloc((::core::mem::size_of::<c_int>() as size_t).wrapping_mul(window_size))
            as *mut c_int;
    h_view.same = malloc(
        (::core::mem::size_of::<c_ushort>() as size_t).wrapping_mul(window_size),
    ) as *mut c_ushort;
    h_view.head2 = malloc(
        (::core::mem::size_of::<c_int>() as size_t)
            .wrapping_mul(65536 as c_int as size_t),
    ) as *mut c_int;
    h_view.prev2 = malloc(
        (::core::mem::size_of::<c_ushort>() as size_t).wrapping_mul(window_size),
    ) as *mut c_ushort;
    h_view.hashval2 =
        malloc((::core::mem::size_of::<c_int>() as size_t).wrapping_mul(window_size))
            as *mut c_int;
}
#[inline]
pub unsafe fn ZopfliResetHash(mut window_size: size_t, mut h: *mut ZopfliHash) {
    let h_view: &mut ZopfliHash = unsafe { &mut *h };
    let mut i: size_t = 0;
    h_view.val = 0 as c_int;
    i = 0 as size_t;
    while i < 65536 as c_int as size_t {
        *h_view.head.offset(i as isize) = -(1 as c_int);
        i = i.wrapping_add(1);
    }
    i = 0 as size_t;
    while i < window_size {
        *h_view.prev.offset(i as isize) = i as c_ushort;
        *h_view.hashval.offset(i as isize) = -(1 as c_int);
        i = i.wrapping_add(1);
    }
    i = 0 as size_t;
    while i < window_size {
        *h_view.same.offset(i as isize) = 0 as c_ushort;
        i = i.wrapping_add(1);
    }
    h_view.val2 = 0 as c_int;
    i = 0 as size_t;
    while i < 65536 as c_int as size_t {
        *h_view.head2.offset(i as isize) = -(1 as c_int);
        i = i.wrapping_add(1);
    }
    i = 0 as size_t;
    while i < window_size {
        *h_view.prev2.offset(i as isize) = i as c_ushort;
        *h_view.hashval2.offset(i as isize) = -(1 as c_int);
        i = i.wrapping_add(1);
    }
}
#[inline]
pub unsafe fn ZopfliCleanHash(mut h: *mut ZopfliHash) {
    let h_view: &ZopfliHash = unsafe { &*h };
    free(h_view.head as *mut c_void);
    free(h_view.prev as *mut c_void);
    free(h_view.hashval as *mut c_void);
    free(h_view.head2 as *mut c_void);
    free(h_view.prev2 as *mut c_void);
    free(h_view.hashval2 as *mut c_void);
    free(h_view.same as *mut c_void);
}
unsafe fn UpdateHashValue(mut h: *mut ZopfliHash, mut c: c_uchar) {
    let h_view: &mut ZopfliHash = unsafe { &mut *h };
    h_view.val = (h_view.val << HASH_SHIFT ^ c as c_int) & HASH_MASK;
}
#[inline]
pub unsafe fn ZopfliUpdateHash(
    mut array: *const c_uchar,
    mut pos: size_t,
    mut end: size_t,
    mut h: *mut ZopfliHash,
) {
    let hpos = (pos & ZOPFLI_WINDOW_MASK as size_t) as c_ushort;
    let hpos_i = hpos as isize;

    let h_ref = &mut *h;

    let next = if pos.wrapping_add(ZOPFLI_MIN_MATCH as size_t) <= end {
        // SAFETY: Caller upholds that `array` is valid for reads up to `end`; the condition ensures the indexed byte is within bounds.
        *array.add(pos.wrapping_add(ZOPFLI_MIN_MATCH as size_t).wrapping_sub(1 as size_t)) as c_uchar
    } else {
        0 as c_uchar
    };
    UpdateHashValue(h, next);

    let val = h_ref.val;
    let head = h_ref.head;
    let prev = h_ref.prev;
    let hashval = h_ref.hashval;

    // SAFETY: `hpos` is masked to the window size and all hash table pointers are valid per caller/invariant of `ZopfliHash`.
    *hashval.offset(hpos_i) = val;

    // SAFETY: `val` is a valid hash index by construction of `UpdateHashValue`.
    let head_idx = *head.offset(val as isize);
    let prev_val = if head_idx != -(1 as c_int)
        // SAFETY: `head_idx != -1` means it is a previously stored window position index into `hashval`.
        && *hashval.offset(head_idx as isize) == val
    {
        head_idx as c_ushort
    } else {
        hpos
    };
    // SAFETY: `hpos` is a valid window position index.
    *prev.offset(hpos_i) = prev_val;
    // SAFETY: `val` is a valid hash index by construction of `UpdateHashValue`.
    *head.offset(val as isize) = hpos as c_int;

    let prev_same_idx =
        (pos.wrapping_sub(1 as size_t) & ZOPFLI_WINDOW_MASK as size_t) as isize;
    // SAFETY: masked window index is valid for `same`.
    let prev_same = *h_ref.same.offset(prev_same_idx) as c_int;
    let mut amount: size_t = if prev_same > 1 {
        (prev_same - 1) as size_t
    } else {
        0 as size_t
    };

    // SAFETY: Caller guarantees `array` points to the input buffer; reading at `pos` matches original behavior.
    let base = *array.add(pos) as c_int;
    while pos.wrapping_add(amount).wrapping_add(1 as size_t) < end
        // SAFETY: loop condition ensures this indexed read is within the readable prefix ending at `end`.
        && *array.add(pos.wrapping_add(amount).wrapping_add(1 as size_t)) as c_int == base
        && amount < -(1 as c_int) as c_ushort as size_t
    {
        amount = amount.wrapping_add(1);
    }

    let same = h_ref.same;
    // SAFETY: `hpos` is a valid window position index.
    *same.offset(hpos_i) = amount as c_ushort;

    let val2 = ((*same.offset(hpos_i) as c_int - ZOPFLI_MIN_MATCH) & 255 as c_int) ^ val;
    h_ref.val2 = val2;

    let head2 = h_ref.head2;
    let prev2 = h_ref.prev2;
    let hashval2 = h_ref.hashval2;

    // SAFETY: `hpos` is a valid window position index.
    *hashval2.offset(hpos_i) = val2;

    // SAFETY: `val2` is masked/xored into the valid secondary hash range as in the original code.
    let head2_idx = *head2.offset(val2 as isize);
    let prev2_val = if head2_idx != -(1 as c_int)
        // SAFETY: `head2_idx != -1` means it is a previously stored window position index into `hashval2`.
        && *hashval2.offset(head2_idx as isize) == val2
    {
        head2_idx as c_ushort
    } else {
        hpos
    };
    // SAFETY: `hpos` is a valid window position index.
    *prev2.offset(hpos_i) = prev2_val;
    // SAFETY: `val2` is a valid secondary hash index.
    *head2.offset(val2 as isize) = hpos as c_int;
}
#[inline]
pub unsafe fn ZopfliWarmupHash(
    mut array: *const c_uchar,
    mut pos: size_t,
    mut end: size_t,
    mut h: *mut ZopfliHash,
) {
    UpdateHashValue(h, *array.offset(pos.wrapping_add(0 as size_t) as isize));
    if pos.wrapping_add(1 as size_t) < end {
        UpdateHashValue(h, *array.offset(pos.wrapping_add(1 as size_t) as isize));
    }
}

