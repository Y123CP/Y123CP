use core::ffi::*;
pub use crate::src::ffi::*;
pub use crate::src::c_consts::*;
pub use crate::src::c_structs::*;
pub use crate::src::c_types::*;

pub const HASH_SHIFT: c_int = 5 as c_int;
pub const HASH_MASK: c_int = 32767 as c_int;
#[no_mangle]
pub unsafe extern "C" fn ZopfliAllocHash(mut window_size: size_t, mut h: *mut ZopfliHash) {
    (*h).head = malloc(
        (::core::mem::size_of::<c_int>() as size_t)
            .wrapping_mul(65536 as c_int as size_t),
    ) as *mut c_int;
    (*h).prev = malloc(
        (::core::mem::size_of::<c_ushort>() as size_t).wrapping_mul(window_size),
    ) as *mut c_ushort;
    (*h).hashval =
        malloc((::core::mem::size_of::<c_int>() as size_t).wrapping_mul(window_size))
            as *mut c_int;
    (*h).same = malloc(
        (::core::mem::size_of::<c_ushort>() as size_t).wrapping_mul(window_size),
    ) as *mut c_ushort;
    (*h).head2 = malloc(
        (::core::mem::size_of::<c_int>() as size_t)
            .wrapping_mul(65536 as c_int as size_t),
    ) as *mut c_int;
    (*h).prev2 = malloc(
        (::core::mem::size_of::<c_ushort>() as size_t).wrapping_mul(window_size),
    ) as *mut c_ushort;
    (*h).hashval2 =
        malloc((::core::mem::size_of::<c_int>() as size_t).wrapping_mul(window_size))
            as *mut c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ZopfliResetHash(mut window_size: size_t, mut h: *mut ZopfliHash) {
    let mut i: size_t = 0;
    (*h).val = 0 as c_int;
    i = 0 as size_t;
    while i < 65536 as c_int as size_t {
        *(*h).head.offset(i as isize) = -(1 as c_int);
        i = i.wrapping_add(1);
    }
    i = 0 as size_t;
    while i < window_size {
        *(*h).prev.offset(i as isize) = i as c_ushort;
        *(*h).hashval.offset(i as isize) = -(1 as c_int);
        i = i.wrapping_add(1);
    }
    i = 0 as size_t;
    while i < window_size {
        *(*h).same.offset(i as isize) = 0 as c_ushort;
        i = i.wrapping_add(1);
    }
    (*h).val2 = 0 as c_int;
    i = 0 as size_t;
    while i < 65536 as c_int as size_t {
        *(*h).head2.offset(i as isize) = -(1 as c_int);
        i = i.wrapping_add(1);
    }
    i = 0 as size_t;
    while i < window_size {
        *(*h).prev2.offset(i as isize) = i as c_ushort;
        *(*h).hashval2.offset(i as isize) = -(1 as c_int);
        i = i.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn ZopfliCleanHash(mut h: *mut ZopfliHash) {
    free((*h).head as *mut c_void);
    free((*h).prev as *mut c_void);
    free((*h).hashval as *mut c_void);
    free((*h).head2 as *mut c_void);
    free((*h).prev2 as *mut c_void);
    free((*h).hashval2 as *mut c_void);
    free((*h).same as *mut c_void);
}
unsafe extern "C" fn UpdateHashValue(mut h: *mut ZopfliHash, mut c: c_uchar) {
    (*h).val = ((*h).val << HASH_SHIFT ^ c as c_int) & HASH_MASK;
}
#[no_mangle]
pub unsafe extern "C" fn ZopfliUpdateHash(
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
#[no_mangle]
pub unsafe extern "C" fn ZopfliWarmupHash(
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

