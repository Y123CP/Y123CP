extern "C" {
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn MQRspec_getWidth(version: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn MQRspec_getFormatInfo(
        mask: ::core::ffi::c_int,
        version: ::core::ffi::c_int,
        level: QRecLevel,
    ) -> ::core::ffi::c_uint;
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
) -> ();
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const EINVAL: ::core::ffi::c_int = 22 as ::core::ffi::c_int;
unsafe extern "C" fn MMask_writeFormatInformation(
    mut version: ::core::ffi::c_int,
    mut width: ::core::ffi::c_int,
    mut frame: *mut ::core::ffi::c_uchar,
    mut mask: ::core::ffi::c_int,
    mut level: QRecLevel,
) {
    let mut format: ::core::ffi::c_uint = 0;
    let mut v: ::core::ffi::c_uchar = 0;
    let mut i: ::core::ffi::c_int = 0;
    format = MQRspec_getFormatInfo(mask, version, level);
    i = 0 as ::core::ffi::c_int;
    while i < 8 as ::core::ffi::c_int {
        v = (0x84 as ::core::ffi::c_uint | format & 1 as ::core::ffi::c_uint)
            as ::core::ffi::c_uchar;
        *frame.offset((width * (i + 1 as ::core::ffi::c_int) + 8 as ::core::ffi::c_int) as isize) =
            v;
        format = format >> 1 as ::core::ffi::c_int;
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < 7 as ::core::ffi::c_int {
        v = (0x84 as ::core::ffi::c_uint | format & 1 as ::core::ffi::c_uint)
            as ::core::ffi::c_uchar;
        *frame.offset((width * 8 as ::core::ffi::c_int + 7 as ::core::ffi::c_int - i) as isize) = v;
        format = format >> 1 as ::core::ffi::c_int;
        i += 1;
    }
}
unsafe extern "C" fn Mask_mask0(
    mut width: ::core::ffi::c_int,
    mut s: *const ::core::ffi::c_uchar,
    mut d: *mut ::core::ffi::c_uchar,
) {
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
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
            s = s.offset(1);
            d = d.offset(1);
            x += 1;
        }
        y += 1;
    }
}
unsafe extern "C" fn Mask_mask1(
    mut width: ::core::ffi::c_int,
    mut s: *const ::core::ffi::c_uchar,
    mut d: *mut ::core::ffi::c_uchar,
) {
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
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
            s = s.offset(1);
            d = d.offset(1);
            x += 1;
        }
        y += 1;
    }
}
unsafe extern "C" fn Mask_mask2(
    mut width: ::core::ffi::c_int,
    mut s: *const ::core::ffi::c_uchar,
    mut d: *mut ::core::ffi::c_uchar,
) {
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
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
            s = s.offset(1);
            d = d.offset(1);
            x += 1;
        }
        y += 1;
    }
}
unsafe extern "C" fn Mask_mask3(
    mut width: ::core::ffi::c_int,
    mut s: *const ::core::ffi::c_uchar,
    mut d: *mut ::core::ffi::c_uchar,
) {
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    y = 0 as ::core::ffi::c_int;
    while y < width {
        x = 0 as ::core::ffi::c_int;
        while x < width {
            if *s as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int != 0 {
                *d = *s;
            } else {
                *d = (*s as ::core::ffi::c_int
                    ^ ((x + y & 1 as ::core::ffi::c_int) + x * y % 3 as ::core::ffi::c_int
                        & 1 as ::core::ffi::c_int
                        == 0 as ::core::ffi::c_int) as ::core::ffi::c_int)
                    as ::core::ffi::c_uchar;
            }
            s = s.offset(1);
            d = d.offset(1);
            x += 1;
        }
        y += 1;
    }
}
pub const maskNum: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
static mut maskMakers: [Option<MaskMaker>; 4] = unsafe {
    [
        Some(
            Mask_mask0
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    *const ::core::ffi::c_uchar,
                    *mut ::core::ffi::c_uchar,
                ) -> (),
        ),
        Some(
            Mask_mask1
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    *const ::core::ffi::c_uchar,
                    *mut ::core::ffi::c_uchar,
                ) -> (),
        ),
        Some(
            Mask_mask2
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    *const ::core::ffi::c_uchar,
                    *mut ::core::ffi::c_uchar,
                ) -> (),
        ),
        Some(
            Mask_mask3
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    *const ::core::ffi::c_uchar,
                    *mut ::core::ffi::c_uchar,
                ) -> (),
        ),
    ]
};
#[no_mangle]
pub unsafe extern "C" fn MMask_makeMask(
    mut version: ::core::ffi::c_int,
    mut frame: *mut ::core::ffi::c_uchar,
    mut mask: ::core::ffi::c_int,
    mut level: QRecLevel,
) -> *mut ::core::ffi::c_uchar {
    let mut masked: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut width: ::core::ffi::c_int = 0;
    if mask < 0 as ::core::ffi::c_int || mask >= maskNum {
        *__errno_location() = EINVAL;
        return ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    }
    width = MQRspec_getWidth(version);
    masked = malloc((width * width) as size_t) as *mut ::core::ffi::c_uchar;
    if masked.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    }
    maskMakers[mask as usize].expect("non-null function pointer")(width, frame, masked);
    MMask_writeFormatInformation(version, width, masked, mask, level);
    return masked;
}
unsafe extern "C" fn MMask_evaluateSymbol(
    mut width: ::core::ffi::c_int,
    mut frame: *mut ::core::ffi::c_uchar,
) -> ::core::ffi::c_int {
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    let mut p: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut sum1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut sum2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    p = frame.offset((width * (width - 1 as ::core::ffi::c_int)) as isize);
    x = 1 as ::core::ffi::c_int;
    while x < width {
        sum1 += *p.offset(x as isize) as ::core::ffi::c_int & 1 as ::core::ffi::c_int;
        x += 1;
    }
    p = frame
        .offset((width * 2 as ::core::ffi::c_int) as isize)
        .offset(-(1 as ::core::ffi::c_int as isize));
    y = 1 as ::core::ffi::c_int;
    while y < width {
        sum2 += *p as ::core::ffi::c_int & 1 as ::core::ffi::c_int;
        p = p.offset(width as isize);
        y += 1;
    }
    return if sum1 <= sum2 {
        sum1 * 16 as ::core::ffi::c_int + sum2
    } else {
        sum2 * 16 as ::core::ffi::c_int + sum1
    };
}
#[no_mangle]
pub unsafe extern "C" fn MMask_mask(
    mut version: ::core::ffi::c_int,
    mut frame: *mut ::core::ffi::c_uchar,
    mut level: QRecLevel,
) -> *mut ::core::ffi::c_uchar {
    let mut i: ::core::ffi::c_int = 0;
    let mut mask: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut bestMask: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut maxScore: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut score: ::core::ffi::c_int = 0;
    let mut width: ::core::ffi::c_int = 0;
    width = MQRspec_getWidth(version);
    mask = malloc((width * width) as size_t) as *mut ::core::ffi::c_uchar;
    if mask.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    }
    bestMask = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    i = 0 as ::core::ffi::c_int;
    while i < maskNum {
        score = 0 as ::core::ffi::c_int;
        maskMakers[i as usize].expect("non-null function pointer")(width, frame, mask);
        MMask_writeFormatInformation(version, width, mask, i, level);
        score = MMask_evaluateSymbol(width, mask);
        if score > maxScore {
            maxScore = score;
            free(bestMask as *mut ::core::ffi::c_void);
            bestMask = mask;
            mask = malloc((width * width) as size_t) as *mut ::core::ffi::c_uchar;
            if mask.is_null() {
                break;
            }
        }
        i += 1;
    }
    free(mask as *mut ::core::ffi::c_void);
    return bestMask;
}
