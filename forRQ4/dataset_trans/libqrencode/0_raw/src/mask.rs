extern "C" {
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn abs(__x: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn QRspec_getFormatInfo(mask: ::core::ffi::c_int, level: QRecLevel) -> ::core::ffi::c_uint;
}
pub type size_t = usize;
pub type QRecLevel = ::core::ffi::c_uint;
pub const QR_ECLEVEL_H: QRecLevel = 3;
pub const QR_ECLEVEL_Q: QRecLevel = 2;
pub const QR_ECLEVEL_M: QRecLevel = 1;
pub const QR_ECLEVEL_L: QRecLevel = 0;
pub type MaskMaker = unsafe extern "C" fn(
    ::core::ffi::c_int,
    *const ::core::ffi::c_uchar,
    *mut ::core::ffi::c_uchar,
) -> ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const INT_MAX: ::core::ffi::c_int = __INT_MAX__;
pub const EINVAL: ::core::ffi::c_int = 22 as ::core::ffi::c_int;
unsafe extern "C" fn Mask_writeFormatInformation(
    mut width: ::core::ffi::c_int,
    mut frame: *mut ::core::ffi::c_uchar,
    mut mask: ::core::ffi::c_int,
    mut level: QRecLevel,
) -> ::core::ffi::c_int {
    let mut format: ::core::ffi::c_uint = 0;
    let mut v: ::core::ffi::c_uchar = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut blacks: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    format = QRspec_getFormatInfo(mask, level);
    i = 0 as ::core::ffi::c_int;
    while i < 8 as ::core::ffi::c_int {
        if format & 1 as ::core::ffi::c_uint != 0 {
            blacks += 2 as ::core::ffi::c_int;
            v = 0x85 as ::core::ffi::c_uchar;
        } else {
            v = 0x84 as ::core::ffi::c_uchar;
        }
        *frame.offset(
            (width * 8 as ::core::ffi::c_int + width - 1 as ::core::ffi::c_int - i) as isize,
        ) = v;
        if i < 6 as ::core::ffi::c_int {
            *frame.offset((width * i + 8 as ::core::ffi::c_int) as isize) = v;
        } else {
            *frame.offset(
                (width * (i + 1 as ::core::ffi::c_int) + 8 as ::core::ffi::c_int) as isize,
            ) = v;
        }
        format = format >> 1 as ::core::ffi::c_int;
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < 7 as ::core::ffi::c_int {
        if format & 1 as ::core::ffi::c_uint != 0 {
            blacks += 2 as ::core::ffi::c_int;
            v = 0x85 as ::core::ffi::c_uchar;
        } else {
            v = 0x84 as ::core::ffi::c_uchar;
        }
        *frame.offset(
            (width * (width - 7 as ::core::ffi::c_int + i) + 8 as ::core::ffi::c_int) as isize,
        ) = v;
        if i == 0 as ::core::ffi::c_int {
            *frame.offset((width * 8 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as isize) = v;
        } else {
            *frame
                .offset((width * 8 as ::core::ffi::c_int + 6 as ::core::ffi::c_int - i) as isize) =
                v;
        }
        format = format >> 1 as ::core::ffi::c_int;
        i += 1;
    }
    return blacks;
}
pub const N1: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const N2: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const N3: ::core::ffi::c_int = 40 as ::core::ffi::c_int;
pub const N4: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
unsafe extern "C" fn Mask_mask0(
    mut width: ::core::ffi::c_int,
    mut s: *const ::core::ffi::c_uchar,
    mut d: *mut ::core::ffi::c_uchar,
) -> ::core::ffi::c_int {
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    let mut b: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    y = 0 as ::core::ffi::c_int;
    while y < width {
        x = 0 as ::core::ffi::c_int;
        while x < width {
            if *s as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int != 0 {
                *d = *s;
            } else {
                *d = (*s as ::core::ffi::c_int
                    ^ (x + y & 1 as ::core::ffi::c_int == 0 as ::core::ffi::c_int)
                        as ::core::ffi::c_int) as ::core::ffi::c_uchar;
            }
            b += *d as ::core::ffi::c_int & 1 as ::core::ffi::c_int;
            s = s.offset(1);
            d = d.offset(1);
            x += 1;
        }
        y += 1;
    }
    return b;
}
unsafe extern "C" fn Mask_mask1(
    mut width: ::core::ffi::c_int,
    mut s: *const ::core::ffi::c_uchar,
    mut d: *mut ::core::ffi::c_uchar,
) -> ::core::ffi::c_int {
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    let mut b: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    y = 0 as ::core::ffi::c_int;
    while y < width {
        x = 0 as ::core::ffi::c_int;
        while x < width {
            if *s as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int != 0 {
                *d = *s;
            } else {
                *d = (*s as ::core::ffi::c_int
                    ^ (y & 1 as ::core::ffi::c_int == 0 as ::core::ffi::c_int)
                        as ::core::ffi::c_int) as ::core::ffi::c_uchar;
            }
            b += *d as ::core::ffi::c_int & 1 as ::core::ffi::c_int;
            s = s.offset(1);
            d = d.offset(1);
            x += 1;
        }
        y += 1;
    }
    return b;
}
unsafe extern "C" fn Mask_mask2(
    mut width: ::core::ffi::c_int,
    mut s: *const ::core::ffi::c_uchar,
    mut d: *mut ::core::ffi::c_uchar,
) -> ::core::ffi::c_int {
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    let mut b: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    y = 0 as ::core::ffi::c_int;
    while y < width {
        x = 0 as ::core::ffi::c_int;
        while x < width {
            if *s as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int != 0 {
                *d = *s;
            } else {
                *d = (*s as ::core::ffi::c_int
                    ^ (x % 3 as ::core::ffi::c_int == 0 as ::core::ffi::c_int)
                        as ::core::ffi::c_int) as ::core::ffi::c_uchar;
            }
            b += *d as ::core::ffi::c_int & 1 as ::core::ffi::c_int;
            s = s.offset(1);
            d = d.offset(1);
            x += 1;
        }
        y += 1;
    }
    return b;
}
unsafe extern "C" fn Mask_mask3(
    mut width: ::core::ffi::c_int,
    mut s: *const ::core::ffi::c_uchar,
    mut d: *mut ::core::ffi::c_uchar,
) -> ::core::ffi::c_int {
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    let mut b: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    y = 0 as ::core::ffi::c_int;
    while y < width {
        x = 0 as ::core::ffi::c_int;
        while x < width {
            if *s as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int != 0 {
                *d = *s;
            } else {
                *d = (*s as ::core::ffi::c_int
                    ^ ((x + y) % 3 as ::core::ffi::c_int == 0 as ::core::ffi::c_int)
                        as ::core::ffi::c_int) as ::core::ffi::c_uchar;
            }
            b += *d as ::core::ffi::c_int & 1 as ::core::ffi::c_int;
            s = s.offset(1);
            d = d.offset(1);
            x += 1;
        }
        y += 1;
    }
    return b;
}
unsafe extern "C" fn Mask_mask4(
    mut width: ::core::ffi::c_int,
    mut s: *const ::core::ffi::c_uchar,
    mut d: *mut ::core::ffi::c_uchar,
) -> ::core::ffi::c_int {
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    let mut b: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    y = 0 as ::core::ffi::c_int;
    while y < width {
        x = 0 as ::core::ffi::c_int;
        while x < width {
            if *s as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int != 0 {
                *d = *s;
            } else {
                *d = (*s as ::core::ffi::c_int
                    ^ (y / 2 as ::core::ffi::c_int + x / 3 as ::core::ffi::c_int
                        & 1 as ::core::ffi::c_int
                        == 0 as ::core::ffi::c_int) as ::core::ffi::c_int)
                    as ::core::ffi::c_uchar;
            }
            b += *d as ::core::ffi::c_int & 1 as ::core::ffi::c_int;
            s = s.offset(1);
            d = d.offset(1);
            x += 1;
        }
        y += 1;
    }
    return b;
}
unsafe extern "C" fn Mask_mask5(
    mut width: ::core::ffi::c_int,
    mut s: *const ::core::ffi::c_uchar,
    mut d: *mut ::core::ffi::c_uchar,
) -> ::core::ffi::c_int {
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    let mut b: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    y = 0 as ::core::ffi::c_int;
    while y < width {
        x = 0 as ::core::ffi::c_int;
        while x < width {
            if *s as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int != 0 {
                *d = *s;
            } else {
                *d = (*s as ::core::ffi::c_int
                    ^ ((x * y & 1 as ::core::ffi::c_int) + x * y % 3 as ::core::ffi::c_int
                        == 0 as ::core::ffi::c_int) as ::core::ffi::c_int)
                    as ::core::ffi::c_uchar;
            }
            b += *d as ::core::ffi::c_int & 1 as ::core::ffi::c_int;
            s = s.offset(1);
            d = d.offset(1);
            x += 1;
        }
        y += 1;
    }
    return b;
}
unsafe extern "C" fn Mask_mask6(
    mut width: ::core::ffi::c_int,
    mut s: *const ::core::ffi::c_uchar,
    mut d: *mut ::core::ffi::c_uchar,
) -> ::core::ffi::c_int {
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    let mut b: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    y = 0 as ::core::ffi::c_int;
    while y < width {
        x = 0 as ::core::ffi::c_int;
        while x < width {
            if *s as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int != 0 {
                *d = *s;
            } else {
                *d = (*s as ::core::ffi::c_int
                    ^ ((x * y & 1 as ::core::ffi::c_int) + x * y % 3 as ::core::ffi::c_int
                        & 1 as ::core::ffi::c_int
                        == 0 as ::core::ffi::c_int) as ::core::ffi::c_int)
                    as ::core::ffi::c_uchar;
            }
            b += *d as ::core::ffi::c_int & 1 as ::core::ffi::c_int;
            s = s.offset(1);
            d = d.offset(1);
            x += 1;
        }
        y += 1;
    }
    return b;
}
unsafe extern "C" fn Mask_mask7(
    mut width: ::core::ffi::c_int,
    mut s: *const ::core::ffi::c_uchar,
    mut d: *mut ::core::ffi::c_uchar,
) -> ::core::ffi::c_int {
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    let mut b: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    y = 0 as ::core::ffi::c_int;
    while y < width {
        x = 0 as ::core::ffi::c_int;
        while x < width {
            if *s as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int != 0 {
                *d = *s;
            } else {
                *d = (*s as ::core::ffi::c_int
                    ^ (x * y % 3 as ::core::ffi::c_int + (x + y & 1 as ::core::ffi::c_int)
                        & 1 as ::core::ffi::c_int
                        == 0 as ::core::ffi::c_int) as ::core::ffi::c_int)
                    as ::core::ffi::c_uchar;
            }
            b += *d as ::core::ffi::c_int & 1 as ::core::ffi::c_int;
            s = s.offset(1);
            d = d.offset(1);
            x += 1;
        }
        y += 1;
    }
    return b;
}
pub const maskNum: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
static mut maskMakers: [Option<MaskMaker>; 8] = unsafe {
    [
        Some(
            Mask_mask0
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    *const ::core::ffi::c_uchar,
                    *mut ::core::ffi::c_uchar,
                ) -> ::core::ffi::c_int,
        ),
        Some(
            Mask_mask1
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    *const ::core::ffi::c_uchar,
                    *mut ::core::ffi::c_uchar,
                ) -> ::core::ffi::c_int,
        ),
        Some(
            Mask_mask2
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    *const ::core::ffi::c_uchar,
                    *mut ::core::ffi::c_uchar,
                ) -> ::core::ffi::c_int,
        ),
        Some(
            Mask_mask3
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    *const ::core::ffi::c_uchar,
                    *mut ::core::ffi::c_uchar,
                ) -> ::core::ffi::c_int,
        ),
        Some(
            Mask_mask4
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    *const ::core::ffi::c_uchar,
                    *mut ::core::ffi::c_uchar,
                ) -> ::core::ffi::c_int,
        ),
        Some(
            Mask_mask5
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    *const ::core::ffi::c_uchar,
                    *mut ::core::ffi::c_uchar,
                ) -> ::core::ffi::c_int,
        ),
        Some(
            Mask_mask6
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    *const ::core::ffi::c_uchar,
                    *mut ::core::ffi::c_uchar,
                ) -> ::core::ffi::c_int,
        ),
        Some(
            Mask_mask7
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    *const ::core::ffi::c_uchar,
                    *mut ::core::ffi::c_uchar,
                ) -> ::core::ffi::c_int,
        ),
    ]
};
#[no_mangle]
pub unsafe extern "C" fn Mask_makeMask(
    mut width: ::core::ffi::c_int,
    mut frame: *mut ::core::ffi::c_uchar,
    mut mask: ::core::ffi::c_int,
    mut level: QRecLevel,
) -> *mut ::core::ffi::c_uchar {
    let mut masked: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    if mask < 0 as ::core::ffi::c_int || mask >= maskNum {
        *__errno_location() = EINVAL;
        return ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    }
    masked = malloc((width * width) as size_t) as *mut ::core::ffi::c_uchar;
    if masked.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    }
    maskMakers[mask as usize].expect("non-null function pointer")(width, frame, masked);
    Mask_writeFormatInformation(width, masked, mask, level);
    return masked;
}
unsafe extern "C" fn Mask_calcN1N3(
    mut length: ::core::ffi::c_int,
    mut runLength: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut demerit: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut fact: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < length {
        if *runLength.offset(i as isize) >= 5 as ::core::ffi::c_int {
            demerit += N1 + (*runLength.offset(i as isize) - 5 as ::core::ffi::c_int);
        }
        if i & 1 as ::core::ffi::c_int != 0 {
            if i >= 3 as ::core::ffi::c_int
                && i < length - 2 as ::core::ffi::c_int
                && *runLength.offset(i as isize) % 3 as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
            {
                fact = *runLength.offset(i as isize) / 3 as ::core::ffi::c_int;
                if *runLength.offset((i - 2 as ::core::ffi::c_int) as isize) == fact
                    && *runLength.offset((i - 1 as ::core::ffi::c_int) as isize) == fact
                    && *runLength.offset((i + 1 as ::core::ffi::c_int) as isize) == fact
                    && *runLength.offset((i + 2 as ::core::ffi::c_int) as isize) == fact
                {
                    if i == 3 as ::core::ffi::c_int
                        || *runLength.offset((i - 3 as ::core::ffi::c_int) as isize)
                            >= 4 as ::core::ffi::c_int * fact
                    {
                        demerit += N3;
                    } else if i + 4 as ::core::ffi::c_int >= length
                        || *runLength.offset((i + 3 as ::core::ffi::c_int) as isize)
                            >= 4 as ::core::ffi::c_int * fact
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
unsafe extern "C" fn Mask_calcN2(
    mut width: ::core::ffi::c_int,
    mut frame: *mut ::core::ffi::c_uchar,
) -> ::core::ffi::c_int {
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    let mut p: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut b22: ::core::ffi::c_uchar = 0;
    let mut w22: ::core::ffi::c_uchar = 0;
    let mut demerit: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    p = frame
        .offset(width as isize)
        .offset(1 as ::core::ffi::c_int as isize);
    y = 1 as ::core::ffi::c_int;
    while y < width {
        x = 1 as ::core::ffi::c_int;
        while x < width {
            b22 = (*p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                & *p.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                & *p.offset(-width as isize) as ::core::ffi::c_int
                & *p.offset((-width - 1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int)
                as ::core::ffi::c_uchar;
            w22 = (*p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                | *p.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                | *p.offset(-width as isize) as ::core::ffi::c_int
                | *p.offset((-width - 1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int)
                as ::core::ffi::c_uchar;
            if (b22 as ::core::ffi::c_int | w22 as ::core::ffi::c_int ^ 1 as ::core::ffi::c_int)
                & 1 as ::core::ffi::c_int
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
unsafe extern "C" fn Mask_calcRunLengthH(
    mut width: ::core::ffi::c_int,
    mut frame: *mut ::core::ffi::c_uchar,
    mut runLength: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut head: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut prev: ::core::ffi::c_uchar = 0;
    if *frame.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        & 1 as ::core::ffi::c_int
        != 0
    {
        *runLength.offset(0 as ::core::ffi::c_int as isize) = -(1 as ::core::ffi::c_int);
        head = 1 as ::core::ffi::c_int;
    } else {
        head = 0 as ::core::ffi::c_int;
    }
    *runLength.offset(head as isize) = 1 as ::core::ffi::c_int;
    prev = *frame.offset(0 as ::core::ffi::c_int as isize);
    i = 1 as ::core::ffi::c_int;
    while i < width {
        if (*frame.offset(i as isize) as ::core::ffi::c_int ^ prev as ::core::ffi::c_int)
            & 1 as ::core::ffi::c_int
            != 0
        {
            head += 1;
            *runLength.offset(head as isize) = 1 as ::core::ffi::c_int;
            prev = *frame.offset(i as isize);
        } else {
            let ref mut fresh1 = *runLength.offset(head as isize);
            *fresh1 += 1;
        }
        i += 1;
    }
    return head + 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn Mask_calcRunLengthV(
    mut width: ::core::ffi::c_int,
    mut frame: *mut ::core::ffi::c_uchar,
    mut runLength: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut head: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut prev: ::core::ffi::c_uchar = 0;
    if *frame.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        & 1 as ::core::ffi::c_int
        != 0
    {
        *runLength.offset(0 as ::core::ffi::c_int as isize) = -(1 as ::core::ffi::c_int);
        head = 1 as ::core::ffi::c_int;
    } else {
        head = 0 as ::core::ffi::c_int;
    }
    *runLength.offset(head as isize) = 1 as ::core::ffi::c_int;
    prev = *frame.offset(0 as ::core::ffi::c_int as isize);
    i = 1 as ::core::ffi::c_int;
    while i < width {
        if (*frame.offset((i * width) as isize) as ::core::ffi::c_int ^ prev as ::core::ffi::c_int)
            & 1 as ::core::ffi::c_int
            != 0
        {
            head += 1;
            *runLength.offset(head as isize) = 1 as ::core::ffi::c_int;
            prev = *frame.offset((i * width) as isize);
        } else {
            let ref mut fresh0 = *runLength.offset(head as isize);
            *fresh0 += 1;
        }
        i += 1;
    }
    return head + 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn Mask_evaluateSymbol(
    mut width: ::core::ffi::c_int,
    mut frame: *mut ::core::ffi::c_uchar,
) -> ::core::ffi::c_int {
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    let mut demerit: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut runLength: [::core::ffi::c_int; 178] = [0; 178];
    let mut length: ::core::ffi::c_int = 0;
    demerit += Mask_calcN2(width, frame);
    y = 0 as ::core::ffi::c_int;
    while y < width {
        length = Mask_calcRunLengthH(
            width,
            frame.offset((y * width) as isize),
            &raw mut runLength as *mut ::core::ffi::c_int,
        );
        demerit += Mask_calcN1N3(length, &raw mut runLength as *mut ::core::ffi::c_int);
        y += 1;
    }
    x = 0 as ::core::ffi::c_int;
    while x < width {
        length = Mask_calcRunLengthV(
            width,
            frame.offset(x as isize),
            &raw mut runLength as *mut ::core::ffi::c_int,
        );
        demerit += Mask_calcN1N3(length, &raw mut runLength as *mut ::core::ffi::c_int);
        x += 1;
    }
    return demerit;
}
#[no_mangle]
pub unsafe extern "C" fn Mask_mask(
    mut width: ::core::ffi::c_int,
    mut frame: *mut ::core::ffi::c_uchar,
    mut level: QRecLevel,
) -> *mut ::core::ffi::c_uchar {
    let mut i: ::core::ffi::c_int = 0;
    let mut mask: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut bestMask: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut minDemerit: ::core::ffi::c_int = INT_MAX;
    let mut blacks: ::core::ffi::c_int = 0;
    let mut bratio: ::core::ffi::c_int = 0;
    let mut demerit: ::core::ffi::c_int = 0;
    let mut w2: ::core::ffi::c_int = width * width;
    mask = malloc(w2 as size_t) as *mut ::core::ffi::c_uchar;
    if mask.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    }
    bestMask = malloc(w2 as size_t) as *mut ::core::ffi::c_uchar;
    if bestMask.is_null() {
        free(mask as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    }
    i = 0 as ::core::ffi::c_int;
    while i < maskNum {
        demerit = 0 as ::core::ffi::c_int;
        blacks = maskMakers[i as usize].expect("non-null function pointer")(width, frame, mask);
        blacks += Mask_writeFormatInformation(width, mask, i, level);
        bratio = (200 as ::core::ffi::c_int * blacks + w2) / w2 / 2 as ::core::ffi::c_int;
        demerit = abs(bratio - 50 as ::core::ffi::c_int) / 5 as ::core::ffi::c_int * N4;
        demerit += Mask_evaluateSymbol(width, mask);
        if demerit < minDemerit {
            minDemerit = demerit;
            memcpy(
                bestMask as *mut ::core::ffi::c_void,
                mask as *const ::core::ffi::c_void,
                w2 as size_t,
            );
        }
        i += 1;
    }
    free(mask as *mut ::core::ffi::c_void);
    return bestMask;
}
pub const __INT_MAX__: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
