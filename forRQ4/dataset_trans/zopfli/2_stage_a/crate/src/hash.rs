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
    let mut hpos: c_ushort =
        (pos & ZOPFLI_WINDOW_MASK as size_t) as c_ushort;
    let mut amount: size_t = 0 as size_t;
    UpdateHashValue(
        h,
        (if pos.wrapping_add(ZOPFLI_MIN_MATCH as size_t) <= end {
            *array.offset(
                pos.wrapping_add(ZOPFLI_MIN_MATCH as size_t)
                    .wrapping_sub(1 as size_t) as isize,
            ) as c_int
        } else {
            0 as c_int
        }) as c_uchar,
    );
    *(*h).hashval.offset(hpos as isize) = (*h).val;
    if *(*h).head.offset((*h).val as isize) != -(1 as c_int)
        && *(*h)
            .hashval
            .offset(*(*h).head.offset((*h).val as isize) as isize)
            == (*h).val
    {
        *(*h).prev.offset(hpos as isize) =
            *(*h).head.offset((*h).val as isize) as c_ushort;
    } else {
        *(*h).prev.offset(hpos as isize) = hpos;
    }
    *(*h).head.offset((*h).val as isize) = hpos as c_int;
    if *(*h)
        .same
        .offset((pos.wrapping_sub(1 as size_t) & ZOPFLI_WINDOW_MASK as size_t) as isize)
        as c_int
        > 1 as c_int
    {
        amount = (*(*h)
            .same
            .offset((pos.wrapping_sub(1 as size_t) & ZOPFLI_WINDOW_MASK as size_t) as isize)
            as c_int
            - 1 as c_int) as size_t;
    }
    while pos.wrapping_add(amount).wrapping_add(1 as size_t) < end
        && *array.offset(pos as isize) as c_int
            == *array.offset(pos.wrapping_add(amount).wrapping_add(1 as size_t) as isize)
                as c_int
        && amount < -(1 as c_int) as c_ushort as size_t
    {
        amount = amount.wrapping_add(1);
    }
    *(*h).same.offset(hpos as isize) = amount as c_ushort;
    (*h).val2 = *(*h).same.offset(hpos as isize) as c_int - ZOPFLI_MIN_MATCH
        & 255 as c_int
        ^ (*h).val;
    *(*h).hashval2.offset(hpos as isize) = (*h).val2;
    if *(*h).head2.offset((*h).val2 as isize) != -(1 as c_int)
        && *(*h)
            .hashval2
            .offset(*(*h).head2.offset((*h).val2 as isize) as isize)
            == (*h).val2
    {
        *(*h).prev2.offset(hpos as isize) =
            *(*h).head2.offset((*h).val2 as isize) as c_ushort;
    } else {
        *(*h).prev2.offset(hpos as isize) = hpos;
    }
    *(*h).head2.offset((*h).val2 as isize) = hpos as c_int;
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

