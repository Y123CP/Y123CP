use core::ffi::*;
pub use crate::src::ffi::*;
pub use crate::src::c_consts::*;
pub use crate::src::c_types::*;
extern "C" {
    fn MQRspec_getFormatInfo(
        mask: c_int,
        version: c_int,
        level: QRecLevel,
    ) -> c_uint;
}

pub type MaskMaker = unsafe extern "C" fn(
    c_int,
    *const c_uchar,
    *mut c_uchar,
) -> ();

unsafe extern "C" fn MMask_writeFormatInformation(
    mut version: c_int,
    mut width: c_int,
    mut frame: *mut c_uchar,
    mut mask: c_int,
    mut level: QRecLevel,
) {
    let mut format: c_uint = 0;
    let mut v: c_uchar = 0;
    let mut i: c_int = 0;
    format = MQRspec_getFormatInfo(mask, version, level);
    i = 0 as c_int;
    while i < 8 as c_int {
        v = (0x84 as c_uint | format & 1 as c_uint)
            as c_uchar;
        *frame.offset((width * (i + 1 as c_int) + 8 as c_int) as isize) =
            v;
        format = format >> 1 as c_int;
        i += 1;
    }
    i = 0 as c_int;
    while i < 7 as c_int {
        v = (0x84 as c_uint | format & 1 as c_uint)
            as c_uchar;
        *frame.offset((width * 8 as c_int + 7 as c_int - i) as isize) = v;
        format = format >> 1 as c_int;
        i += 1;
    }
}
unsafe extern "C" fn Mask_mask0(
    mut width: c_int,
    mut s: *const c_uchar,
    mut d: *mut c_uchar,
) {
    let mut x: c_int = 0;
    let mut y: c_int = 0;
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
            s = s.offset(1);
            d = d.offset(1);
            x += 1;
        }
        y += 1;
    }
}
unsafe extern "C" fn Mask_mask1(
    mut width: c_int,
    mut s: *const c_uchar,
    mut d: *mut c_uchar,
) {
    let mut x: c_int = 0;
    let mut y: c_int = 0;
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
            s = s.offset(1);
            d = d.offset(1);
            x += 1;
        }
        y += 1;
    }
}
unsafe extern "C" fn Mask_mask2(
    mut width: c_int,
    mut s: *const c_uchar,
    mut d: *mut c_uchar,
) {
    let mut x: c_int = 0;
    let mut y: c_int = 0;
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
            s = s.offset(1);
            d = d.offset(1);
            x += 1;
        }
        y += 1;
    }
}
unsafe extern "C" fn Mask_mask3(
    mut width: c_int,
    mut s: *const c_uchar,
    mut d: *mut c_uchar,
) {
    let mut x: c_int = 0;
    let mut y: c_int = 0;
    y = 0 as c_int;
    while y < width {
        x = 0 as c_int;
        while x < width {
            if *s as c_int & 0x80 as c_int != 0 {
                *d = *s;
            } else {
                *d = (*s as c_int
                    ^ ((x + y & 1 as c_int) + x * y % 3 as c_int
                        & 1 as c_int
                        == 0 as c_int) as c_int)
                    as c_uchar;
            }
            s = s.offset(1);
            d = d.offset(1);
            x += 1;
        }
        y += 1;
    }
}
pub const maskNum: c_int = 4 as c_int;
static mut maskMakers: [Option<MaskMaker>; 4] = unsafe {
    [
        Some(
            Mask_mask0
                as unsafe extern "C" fn(
                    c_int,
                    *const c_uchar,
                    *mut c_uchar,
                ) -> (),
        ),
        Some(
            Mask_mask1
                as unsafe extern "C" fn(
                    c_int,
                    *const c_uchar,
                    *mut c_uchar,
                ) -> (),
        ),
        Some(
            Mask_mask2
                as unsafe extern "C" fn(
                    c_int,
                    *const c_uchar,
                    *mut c_uchar,
                ) -> (),
        ),
        Some(
            Mask_mask3
                as unsafe extern "C" fn(
                    c_int,
                    *const c_uchar,
                    *mut c_uchar,
                ) -> (),
        ),
    ]
};
#[no_mangle]
pub unsafe extern "C" fn MMask_makeMask(
    mut version: c_int,
    mut frame: *mut c_uchar,
    mut mask: c_int,
    mut level: QRecLevel,
) -> *mut c_uchar {
    let mut masked: *mut c_uchar = ::core::ptr::null_mut::<c_uchar>();
    let mut width: c_int = 0;
    if mask < 0 as c_int || mask >= maskNum {
        *__errno_location() = EINVAL;
        return ::core::ptr::null_mut::<c_uchar>();
    }
    width = MQRspec_getWidth(version);
    masked = malloc((width * width) as size_t) as *mut c_uchar;
    if masked.is_null() {
        return ::core::ptr::null_mut::<c_uchar>();
    }
    maskMakers[mask as usize].expect("non-null function pointer")(width, frame, masked);
    MMask_writeFormatInformation(version, width, masked, mask, level);
    return masked;
}
unsafe extern "C" fn MMask_evaluateSymbol(
    mut width: c_int,
    mut frame: *mut c_uchar,
) -> c_int {
    let mut x: c_int = 0;
    let mut y: c_int = 0;
    let mut p: *mut c_uchar = ::core::ptr::null_mut::<c_uchar>();
    let mut sum1: c_int = 0 as c_int;
    let mut sum2: c_int = 0 as c_int;
    p = frame.offset((width * (width - 1 as c_int)) as isize);
    x = 1 as c_int;
    while x < width {
        sum1 += *p.offset(x as isize) as c_int & 1 as c_int;
        x += 1;
    }
    p = frame
        .offset((width * 2 as c_int) as isize)
        .offset(-(1 as c_int as isize));
    y = 1 as c_int;
    while y < width {
        sum2 += *p as c_int & 1 as c_int;
        p = p.offset(width as isize);
        y += 1;
    }
    return if sum1 <= sum2 {
        sum1 * 16 as c_int + sum2
    } else {
        sum2 * 16 as c_int + sum1
    };
}
#[no_mangle]
pub unsafe extern "C" fn MMask_mask(
    mut version: c_int,
    mut frame: *mut c_uchar,
    mut level: QRecLevel,
) -> *mut c_uchar {
    let mut i: c_int = 0;
    let mut mask: *mut c_uchar = ::core::ptr::null_mut::<c_uchar>();
    let mut bestMask: *mut c_uchar = ::core::ptr::null_mut::<c_uchar>();
    let mut maxScore: c_int = 0 as c_int;
    let mut score: c_int = 0;
    let mut width: c_int = 0;
    width = MQRspec_getWidth(version);
    mask = malloc((width * width) as size_t) as *mut c_uchar;
    if mask.is_null() {
        return ::core::ptr::null_mut::<c_uchar>();
    }
    bestMask = ::core::ptr::null_mut::<c_uchar>();
    i = 0 as c_int;
    while i < maskNum {
        score = 0 as c_int;
        maskMakers[i as usize].expect("non-null function pointer")(width, frame, mask);
        MMask_writeFormatInformation(version, width, mask, i, level);
        score = MMask_evaluateSymbol(width, mask);
        if score > maxScore {
            maxScore = score;
            free(bestMask as *mut c_void);
            bestMask = mask;
            mask = malloc((width * width) as size_t) as *mut c_uchar;
            if mask.is_null() {
                break;
            }
        }
        i += 1;
    }
    free(mask as *mut c_void);
    return bestMask;
}
