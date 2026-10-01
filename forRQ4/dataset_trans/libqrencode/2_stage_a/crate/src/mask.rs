use core::ffi::*;
use crate::src::qrspec::QRspec_getFormatInfo;
pub use crate::src::ffi::*;
pub use crate::src::c_consts::*;
pub use crate::src::c_types::*;

pub type MaskMaker = unsafe extern "C" fn(
    c_int,
    *const c_uchar,
    *mut c_uchar,
) -> c_int;

pub const INT_MAX: c_int = __INT_MAX__;

unsafe fn Mask_writeFormatInformation(
    mut width: c_int,
    mut frame: *mut c_uchar,
    mut mask: c_int,
    mut level: QRecLevel,
) -> c_int {
    let mut format: c_uint = 0;
    let mut v: c_uchar = 0;
    let mut i: c_int = 0;
    let mut blacks: c_int = 0 as c_int;
    format = QRspec_getFormatInfo(mask, level);
    i = 0 as c_int;
    while i < 8 as c_int {
        if format & 1 as c_uint != 0 {
            blacks += 2 as c_int;
            v = 0x85 as c_uchar;
        } else {
            v = 0x84 as c_uchar;
        }
        *frame.offset(
            (width * 8 as c_int + width - 1 as c_int - i) as isize,
        ) = v;
        if i < 6 as c_int {
            *frame.offset((width * i + 8 as c_int) as isize) = v;
        } else {
            *frame.offset(
                (width * (i + 1 as c_int) + 8 as c_int) as isize,
            ) = v;
        }
        format = format >> 1 as c_int;
        i += 1;
    }
    i = 0 as c_int;
    while i < 7 as c_int {
        if format & 1 as c_uint != 0 {
            blacks += 2 as c_int;
            v = 0x85 as c_uchar;
        } else {
            v = 0x84 as c_uchar;
        }
        *frame.offset(
            (width * (width - 7 as c_int + i) + 8 as c_int) as isize,
        ) = v;
        if i == 0 as c_int {
            *frame.offset((width * 8 as c_int + 7 as c_int) as isize) = v;
        } else {
            *frame
                .offset((width * 8 as c_int + 6 as c_int - i) as isize) =
                v;
        }
        format = format >> 1 as c_int;
        i += 1;
    }
    return blacks;
}
pub const N1: c_int = 3 as c_int;
pub const N2: c_int = 3 as c_int;
pub const N3: c_int = 40 as c_int;
pub const N4: c_int = 10 as c_int;
unsafe extern "C" fn Mask_mask0(
    mut width: c_int,
    mut s: *const c_uchar,
    mut d: *mut c_uchar,
) -> c_int {
    let mut x: c_int = 0;
    let mut y: c_int = 0;
    let mut b: c_int = 0 as c_int;
    y = 0 as c_int;
    while y < width {
        x = 0 as c_int;
        while x < width {
            if *s as c_int & 0x80 as c_int != 0 {
                *d = *s;
            } else {
                *d = (*s as c_int
                    ^ (x + y & 1 as c_int == 0 as c_int)
                        as c_int) as c_uchar;
            }
            b += *d as c_int & 1 as c_int;
            s = s.offset(1);
            d = d.offset(1);
            x += 1;
        }
        y += 1;
    }
    return b;
}
unsafe extern "C" fn Mask_mask1(
    mut width: c_int,
    mut s: *const c_uchar,
    mut d: *mut c_uchar,
) -> c_int {
    let mut x: c_int = 0;
    let mut y: c_int = 0;
    let mut b: c_int = 0 as c_int;
    y = 0 as c_int;
    while y < width {
        x = 0 as c_int;
        while x < width {
            if *s as c_int & 0x80 as c_int != 0 {
                *d = *s;
            } else {
                *d = (*s as c_int
                    ^ (y & 1 as c_int == 0 as c_int)
                        as c_int) as c_uchar;
            }
            b += *d as c_int & 1 as c_int;
            s = s.offset(1);
            d = d.offset(1);
            x += 1;
        }
        y += 1;
    }
    return b;
}
unsafe extern "C" fn Mask_mask2(
    mut width: c_int,
    mut s: *const c_uchar,
    mut d: *mut c_uchar,
) -> c_int {
    let mut x: c_int = 0;
    let mut y: c_int = 0;
    let mut b: c_int = 0 as c_int;
    y = 0 as c_int;
    while y < width {
        x = 0 as c_int;
        while x < width {
            if *s as c_int & 0x80 as c_int != 0 {
                *d = *s;
            } else {
                *d = (*s as c_int
                    ^ (x % 3 as c_int == 0 as c_int)
                        as c_int) as c_uchar;
            }
            b += *d as c_int & 1 as c_int;
            s = s.offset(1);
            d = d.offset(1);
            x += 1;
        }
        y += 1;
    }
    return b;
}
unsafe extern "C" fn Mask_mask3(
    mut width: c_int,
    mut s: *const c_uchar,
    mut d: *mut c_uchar,
) -> c_int {
    let mut x: c_int = 0;
    let mut y: c_int = 0;
    let mut b: c_int = 0 as c_int;
    y = 0 as c_int;
    while y < width {
        x = 0 as c_int;
        while x < width {
            if *s as c_int & 0x80 as c_int != 0 {
                *d = *s;
            } else {
                *d = (*s as c_int
                    ^ ((x + y) % 3 as c_int == 0 as c_int)
                        as c_int) as c_uchar;
            }
            b += *d as c_int & 1 as c_int;
            s = s.offset(1);
            d = d.offset(1);
            x += 1;
        }
        y += 1;
    }
    return b;
}
unsafe extern "C" fn Mask_mask4(
    mut width: c_int,
    mut s: *const c_uchar,
    mut d: *mut c_uchar,
) -> c_int {
    let mut x: c_int = 0;
    let mut y: c_int = 0;
    let mut b: c_int = 0 as c_int;
    y = 0 as c_int;
    while y < width {
        x = 0 as c_int;
        while x < width {
            if *s as c_int & 0x80 as c_int != 0 {
                *d = *s;
            } else {
                *d = (*s as c_int
                    ^ (y / 2 as c_int + x / 3 as c_int
                        & 1 as c_int
                        == 0 as c_int) as c_int)
                    as c_uchar;
            }
            b += *d as c_int & 1 as c_int;
            s = s.offset(1);
            d = d.offset(1);
            x += 1;
        }
        y += 1;
    }
    return b;
}
unsafe extern "C" fn Mask_mask5(
    mut width: c_int,
    mut s: *const c_uchar,
    mut d: *mut c_uchar,
) -> c_int {
    let mut x: c_int = 0;
    let mut y: c_int = 0;
    let mut b: c_int = 0 as c_int;
    y = 0 as c_int;
    while y < width {
        x = 0 as c_int;
        while x < width {
            if *s as c_int & 0x80 as c_int != 0 {
                *d = *s;
            } else {
                *d = (*s as c_int
                    ^ ((x * y & 1 as c_int) + x * y % 3 as c_int
                        == 0 as c_int) as c_int)
                    as c_uchar;
            }
            b += *d as c_int & 1 as c_int;
            s = s.offset(1);
            d = d.offset(1);
            x += 1;
        }
        y += 1;
    }
    return b;
}
unsafe extern "C" fn Mask_mask6(
    mut width: c_int,
    mut s: *const c_uchar,
    mut d: *mut c_uchar,
) -> c_int {
    let mut x: c_int = 0;
    let mut y: c_int = 0;
    let mut b: c_int = 0 as c_int;
    y = 0 as c_int;
    while y < width {
        x = 0 as c_int;
        while x < width {
            if *s as c_int & 0x80 as c_int != 0 {
                *d = *s;
            } else {
                *d = (*s as c_int
                    ^ ((x * y & 1 as c_int) + x * y % 3 as c_int
                        & 1 as c_int
                        == 0 as c_int) as c_int)
                    as c_uchar;
            }
            b += *d as c_int & 1 as c_int;
            s = s.offset(1);
            d = d.offset(1);
            x += 1;
        }
        y += 1;
    }
    return b;
}
unsafe extern "C" fn Mask_mask7(
    mut width: c_int,
    mut s: *const c_uchar,
    mut d: *mut c_uchar,
) -> c_int {
    let mut x: c_int = 0;
    let mut y: c_int = 0;
    let mut b: c_int = 0 as c_int;
    y = 0 as c_int;
    while y < width {
        x = 0 as c_int;
        while x < width {
            if *s as c_int & 0x80 as c_int != 0 {
                *d = *s;
            } else {
                *d = (*s as c_int
                    ^ (x * y % 3 as c_int + (x + y & 1 as c_int)
                        & 1 as c_int
                        == 0 as c_int) as c_int)
                    as c_uchar;
            }
            b += *d as c_int & 1 as c_int;
            s = s.offset(1);
            d = d.offset(1);
            x += 1;
        }
        y += 1;
    }
    return b;
}
pub const maskNum: c_int = 8 as c_int;
static mut maskMakers: [Option<MaskMaker>; 8] = [
        Some(
            Mask_mask0
                as unsafe extern "C" fn(
                    c_int,
                    *const c_uchar,
                    *mut c_uchar,
                ) -> c_int,
        ),
        Some(
            Mask_mask1
                as unsafe extern "C" fn(
                    c_int,
                    *const c_uchar,
                    *mut c_uchar,
                ) -> c_int,
        ),
        Some(
            Mask_mask2
                as unsafe extern "C" fn(
                    c_int,
                    *const c_uchar,
                    *mut c_uchar,
                ) -> c_int,
        ),
        Some(
            Mask_mask3
                as unsafe extern "C" fn(
                    c_int,
                    *const c_uchar,
                    *mut c_uchar,
                ) -> c_int,
        ),
        Some(
            Mask_mask4
                as unsafe extern "C" fn(
                    c_int,
                    *const c_uchar,
                    *mut c_uchar,
                ) -> c_int,
        ),
        Some(
            Mask_mask5
                as unsafe extern "C" fn(
                    c_int,
                    *const c_uchar,
                    *mut c_uchar,
                ) -> c_int,
        ),
        Some(
            Mask_mask6
                as unsafe extern "C" fn(
                    c_int,
                    *const c_uchar,
                    *mut c_uchar,
                ) -> c_int,
        ),
        Some(
            Mask_mask7
                as unsafe extern "C" fn(
                    c_int,
                    *const c_uchar,
                    *mut c_uchar,
                ) -> c_int,
        ),
    ];
#[inline]
pub unsafe fn Mask_makeMask(
    mut width: c_int,
    mut frame: *mut c_uchar,
    mut mask: c_int,
    mut level: QRecLevel,
) -> *mut c_uchar {
    let mut masked: *mut c_uchar = ::core::ptr::null_mut::<c_uchar>();
    if mask < 0 as c_int || mask >= maskNum {
        *__errno_location() = EINVAL;
        return ::core::ptr::null_mut::<c_uchar>();
    }
    masked = malloc((width * width) as size_t) as *mut c_uchar;
    if masked.is_null() {
        return ::core::ptr::null_mut::<c_uchar>();
    }
    maskMakers[mask as usize].expect("non-null function pointer")(width, frame, masked);
    Mask_writeFormatInformation(width, masked, mask, level);
    return masked;
}
unsafe fn Mask_calcN1N3(
    mut length: c_int,
    mut runLength: *mut c_int,
) -> c_int {
    let mut i: c_int = 0;
    let mut demerit: c_int = 0 as c_int;
    let mut fact: c_int = 0;
    i = 0 as c_int;
    while i < length {
        if *runLength.offset(i as isize) >= 5 as c_int {
            demerit += N1 + (*runLength.offset(i as isize) - 5 as c_int);
        }
        if i & 1 as c_int != 0 {
            if i >= 3 as c_int
                && i < length - 2 as c_int
                && *runLength.offset(i as isize) % 3 as c_int
                    == 0 as c_int
            {
                fact = *runLength.offset(i as isize) / 3 as c_int;
                if *runLength.offset((i - 2 as c_int) as isize) == fact
                    && *runLength.offset((i - 1 as c_int) as isize) == fact
                    && *runLength.offset((i + 1 as c_int) as isize) == fact
                    && *runLength.offset((i + 2 as c_int) as isize) == fact
                {
                    if i == 3 as c_int
                        || *runLength.offset((i - 3 as c_int) as isize)
                            >= 4 as c_int * fact
                    {
                        demerit += N3;
                    } else if i + 4 as c_int >= length
                        || *runLength.offset((i + 3 as c_int) as isize)
                            >= 4 as c_int * fact
                    {
                        demerit += N3;
                    }
                }
            }
        }
        i += 1;
    }
    return demerit;
}
unsafe fn Mask_calcN2(
    mut width: c_int,
    mut frame: *mut c_uchar,
) -> c_int {
    let mut x: c_int = 0;
    let mut y: c_int = 0;
    let mut p: *mut c_uchar = ::core::ptr::null_mut::<c_uchar>();
    let mut b22: c_uchar = 0;
    let mut w22: c_uchar = 0;
    let mut demerit: c_int = 0 as c_int;
    p = frame
        .offset(width as isize)
        .offset(1 as c_int as isize);
    y = 1 as c_int;
    while y < width {
        x = 1 as c_int;
        while x < width {
            b22 = (*p.offset(0 as c_int as isize) as c_int
                & *p.offset(-(1 as c_int) as isize) as c_int
                & *p.offset(-width as isize) as c_int
                & *p.offset((-width - 1 as c_int) as isize) as c_int)
                as c_uchar;
            w22 = (*p.offset(0 as c_int as isize) as c_int
                | *p.offset(-(1 as c_int) as isize) as c_int
                | *p.offset(-width as isize) as c_int
                | *p.offset((-width - 1 as c_int) as isize) as c_int)
                as c_uchar;
            if (b22 as c_int | w22 as c_int ^ 1 as c_int)
                & 1 as c_int
                != 0
            {
                demerit += N2;
            }
            p = p.offset(1);
            x += 1;
        }
        p = p.offset(1);
        y += 1;
    }
    return demerit;
}
unsafe fn Mask_calcRunLengthH(
    mut width: c_int,
    mut frame: *mut c_uchar,
    mut runLength: *mut c_int,
) -> c_int {
    let mut head: c_int = 0;
    let mut i: c_int = 0;
    let mut prev: c_uchar = 0;
    if *frame.offset(0 as c_int as isize) as c_int
        & 1 as c_int
        != 0
    {
        *runLength.offset(0 as c_int as isize) = -(1 as c_int);
        head = 1 as c_int;
    } else {
        head = 0 as c_int;
    }
    *runLength.offset(head as isize) = 1 as c_int;
    prev = *frame.offset(0 as c_int as isize);
    i = 1 as c_int;
    while i < width {
        if (*frame.offset(i as isize) as c_int ^ prev as c_int)
            & 1 as c_int
            != 0
        {
            head += 1;
            *runLength.offset(head as isize) = 1 as c_int;
            prev = *frame.offset(i as isize);
        } else {
            let ref mut fresh1 = *runLength.offset(head as isize);
            *fresh1 += 1;
        }
        i += 1;
    }
    return head + 1 as c_int;
}
unsafe fn Mask_calcRunLengthV(
    mut width: c_int,
    mut frame: *mut c_uchar,
    mut runLength: *mut c_int,
) -> c_int {
    let mut head: c_int = 0;
    let mut i: c_int = 0;
    let mut prev: c_uchar = 0;
    if *frame.offset(0 as c_int as isize) as c_int
        & 1 as c_int
        != 0
    {
        *runLength.offset(0 as c_int as isize) = -(1 as c_int);
        head = 1 as c_int;
    } else {
        head = 0 as c_int;
    }
    *runLength.offset(head as isize) = 1 as c_int;
    prev = *frame.offset(0 as c_int as isize);
    i = 1 as c_int;
    while i < width {
        if (*frame.offset((i * width) as isize) as c_int ^ prev as c_int)
            & 1 as c_int
            != 0
        {
            head += 1;
            *runLength.offset(head as isize) = 1 as c_int;
            prev = *frame.offset((i * width) as isize);
        } else {
            let ref mut fresh0 = *runLength.offset(head as isize);
            *fresh0 += 1;
        }
        i += 1;
    }
    return head + 1 as c_int;
}
unsafe fn Mask_evaluateSymbol(
    mut width: c_int,
    mut frame: *mut c_uchar,
) -> c_int {
    let mut x: c_int = 0;
    let mut y: c_int = 0;
    let mut demerit: c_int = 0 as c_int;
    let mut runLength: [c_int; 178] = [0; 178];
    let mut length: c_int = 0;
    demerit += Mask_calcN2(width, frame);
    y = 0 as c_int;
    while y < width {
        length = Mask_calcRunLengthH(
            width,
            frame.offset((y * width) as isize),
            &raw mut runLength as *mut c_int,
        );
        demerit += Mask_calcN1N3(length, &raw mut runLength as *mut c_int);
        y += 1;
    }
    x = 0 as c_int;
    while x < width {
        length = Mask_calcRunLengthV(
            width,
            frame.offset(x as isize),
            &raw mut runLength as *mut c_int,
        );
        demerit += Mask_calcN1N3(length, &raw mut runLength as *mut c_int);
        x += 1;
    }
    return demerit;
}
#[inline]
pub unsafe fn Mask_mask(
    mut width: c_int,
    mut frame: *mut c_uchar,
    mut level: QRecLevel,
) -> *mut c_uchar {
    let mut i: c_int = 0;
    let mut mask: *mut c_uchar = ::core::ptr::null_mut::<c_uchar>();
    let mut bestMask: *mut c_uchar = ::core::ptr::null_mut::<c_uchar>();
    let mut minDemerit: c_int = INT_MAX;
    let mut blacks: c_int = 0;
    let mut bratio: c_int = 0;
    let mut demerit: c_int = 0;
    let mut w2: c_int = width * width;
    mask = malloc(w2 as size_t) as *mut c_uchar;
    if mask.is_null() {
        return ::core::ptr::null_mut::<c_uchar>();
    }
    bestMask = malloc(w2 as size_t) as *mut c_uchar;
    if bestMask.is_null() {
        free(mask as *mut c_void);
        return ::core::ptr::null_mut::<c_uchar>();
    }
    i = 0 as c_int;
    while i < maskNum {
        demerit = 0 as c_int;
        blacks = maskMakers[i as usize].expect("non-null function pointer")(width, frame, mask);
        blacks += Mask_writeFormatInformation(width, mask, i, level);
        bratio = (200 as c_int * blacks + w2) / w2 / 2 as c_int;
        demerit = abs(bratio - 50 as c_int) / 5 as c_int * N4;
        demerit += Mask_evaluateSymbol(width, mask);
        if demerit < minDemerit {
            minDemerit = demerit;
            memcpy(
                bestMask as *mut c_void,
                mask as *const c_void,
                w2 as size_t,
            );
        }
        i += 1;
    }
    free(mask as *mut c_void);
    return bestMask;
}
pub const __INT_MAX__: c_int = 2147483647 as c_int;
