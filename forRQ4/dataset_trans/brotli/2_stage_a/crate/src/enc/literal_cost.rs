use core::ffi::*;
use crate::src::enc::utf8_util::BrotliIsMostlyUTF8;
use crate::src::c_inlined_fns::FastLog2;
use crate::src::c_inlined_fns::brotli_min_size_t;
pub use crate::src::c_inlined_fns::*;
pub use crate::src::ffi::*;
pub use crate::src::c_consts::*;
pub use crate::src::c_types::*;

fn UTF8Position(mut last: size_t, mut c: size_t, mut clamp: size_t) -> size_t { {
    if c < 128 as size_t {
        return 0 as size_t;
    } else if c >= 192 as size_t {
        return brotli_min_size_t(1 as size_t, clamp);
    } else if last < 0xe0 as size_t {
        return 0 as size_t;
    } else {
        return brotli_min_size_t(2 as size_t, clamp);
    };
} }
unsafe fn DecideMultiByteStatsLevel(
    mut pos: size_t,
    mut len: size_t,
    mut mask: size_t,
    mut data: *const uint8_t,
) -> size_t {
    let mut counts: [size_t; 3] = [0 as c_int as size_t, 0, 0];
    let mut max_utf8: size_t = 1 as size_t;
    let mut last_c: size_t = 0 as size_t;
    let mut i: size_t = 0;
    i = 0 as size_t;
    while i < len {
        let mut c: size_t = *data.offset((pos.wrapping_add(i) & mask) as isize) as size_t;
        counts[UTF8Position(last_c, c, 2 as size_t) as usize] =
            counts[UTF8Position(last_c, c, 2 as size_t) as usize].wrapping_add(1);
        last_c = c;
        i = i.wrapping_add(1);
    }
    if counts[2 as c_int as usize] < 500 as size_t {
        max_utf8 = 1 as size_t;
    }
    if counts[1 as c_int as usize]
        .wrapping_add(counts[2 as c_int as usize])
        < 25 as size_t
    {
        max_utf8 = 0 as size_t;
    }
    return max_utf8;
}
unsafe fn EstimateBitCostsForLiteralsUTF8(
    mut pos: size_t,
    mut len: size_t,
    mut mask: size_t,
    mut data: *const uint8_t,
    mut histogram: *mut size_t,
    mut cost: *mut c_float,
) {
    let cost_view: &mut [c_float] = unsafe { core::slice::from_raw_parts_mut(cost, (len) as usize) };
    let max_utf8: size_t = DecideMultiByteStatsLevel(pos, len, mask, data) as size_t;
    let mut window_half: size_t = 495 as size_t;
    let mut in_window: size_t = brotli_min_size_t(window_half, len);
    let mut in_window_utf8: [size_t; 3] = [0 as c_int as size_t, 0, 0];
    let mut i: size_t = 0;
    memset(
        histogram as *mut c_void,
        0 as c_int,
        ((3 as c_int * 256 as c_int) as size_t)
            .wrapping_mul(::core::mem::size_of::<size_t>() as size_t),
    );
    let mut last_c: size_t = 0 as size_t;
    let mut utf8_pos: size_t = 0 as size_t;
    i = 0 as size_t;
    while i < in_window {
        let mut c: size_t = *data.offset((pos.wrapping_add(i) & mask) as isize) as size_t;
        let ref mut fresh3 =
            *histogram.offset((256 as size_t).wrapping_mul(utf8_pos).wrapping_add(c) as isize);
        *fresh3 = (*fresh3).wrapping_add(1);
        in_window_utf8[utf8_pos as usize] = in_window_utf8[utf8_pos as usize].wrapping_add(1);
        utf8_pos = UTF8Position(last_c, c, max_utf8);
        last_c = c;
        i = i.wrapping_add(1);
    }
    i = 0 as size_t;
    while i < len {
        if i >= window_half {
            let mut c_0: size_t = (if i < window_half.wrapping_add(1 as size_t) {
                0 as c_int
            } else {
                *data.offset(
                    (pos.wrapping_add(i)
                        .wrapping_sub(window_half)
                        .wrapping_sub(1 as size_t)
                        & mask) as isize,
                ) as c_int
            }) as size_t;
            let mut last_c_0: size_t = (if i < window_half.wrapping_add(2 as size_t) {
                0 as c_int
            } else {
                *data.offset(
                    (pos.wrapping_add(i)
                        .wrapping_sub(window_half)
                        .wrapping_sub(2 as size_t)
                        & mask) as isize,
                ) as c_int
            }) as size_t;
            let mut utf8_pos2: size_t = UTF8Position(last_c_0, c_0, max_utf8);
            let ref mut fresh4 =
                *histogram.offset((256 as size_t).wrapping_mul(utf8_pos2).wrapping_add(
                    *data.offset((pos.wrapping_add(i).wrapping_sub(window_half) & mask) as isize)
                        as size_t,
                ) as isize);
            *fresh4 = (*fresh4).wrapping_sub(1);
            in_window_utf8[utf8_pos2 as usize] = in_window_utf8[utf8_pos2 as usize].wrapping_sub(1);
        }
        if i.wrapping_add(window_half) < len {
            let mut c_1: size_t = *data.offset(
                (pos.wrapping_add(i)
                    .wrapping_add(window_half)
                    .wrapping_sub(1 as size_t)
                    & mask) as isize,
            ) as size_t;
            let mut last_c_1: size_t = *data.offset(
                (pos.wrapping_add(i)
                    .wrapping_add(window_half)
                    .wrapping_sub(2 as size_t)
                    & mask) as isize,
            ) as size_t;
            let mut utf8_pos2_0: size_t = UTF8Position(last_c_1, c_1, max_utf8);
            let ref mut fresh5 =
                *histogram.offset((256 as size_t).wrapping_mul(utf8_pos2_0).wrapping_add(
                    *data.offset((pos.wrapping_add(i).wrapping_add(window_half) & mask) as isize)
                        as size_t,
                ) as isize);
            *fresh5 = (*fresh5).wrapping_add(1);
            in_window_utf8[utf8_pos2_0 as usize] =
                in_window_utf8[utf8_pos2_0 as usize].wrapping_add(1);
        }
        let mut c_2: size_t = (if i < 1 as size_t {
            0 as c_int
        } else {
            *data.offset((pos.wrapping_add(i).wrapping_sub(1 as size_t) & mask) as isize)
                as c_int
        }) as size_t;
        let mut last_c_2: size_t = (if i < 2 as size_t {
            0 as c_int
        } else {
            *data.offset((pos.wrapping_add(i).wrapping_sub(2 as size_t) & mask) as isize)
                as c_int
        }) as size_t;
        let mut utf8_pos_0: size_t = UTF8Position(last_c_2, c_2, max_utf8);
        let mut masked_pos: size_t = pos.wrapping_add(i) & mask;
        let mut histo: size_t = *histogram.offset(
            (256 as size_t)
                .wrapping_mul(utf8_pos_0)
                .wrapping_add(*data.offset(masked_pos as isize) as size_t) as isize,
        );
        static mut prologue_length: size_t = 2000 as size_t;
        static mut multiplier: c_double =
            0.35f64 / 2000 as c_int as c_double;
        let mut lit_cost: c_double = 0.;
        if histo == 0 as size_t {
            histo = 1 as size_t;
        }
        lit_cost = FastLog2(in_window_utf8[utf8_pos_0 as usize]) - FastLog2(histo);
        lit_cost += 0.02905f64;
        if lit_cost < 1.0f64 {
            lit_cost *= 0.5f64;
            lit_cost += 0.5f64;
        }
        if i < prologue_length {
            lit_cost += 0.35f64 + multiplier * i as c_double;
        }
        cost_view[(i) as usize] = lit_cost as c_float;
        i = i.wrapping_add(1);
    }
}
#[inline]
pub unsafe fn BrotliEstimateBitCostsForLiterals(
    mut pos: size_t,
    mut len: size_t,
    mut mask: size_t,
    mut data: *const uint8_t,
    mut histogram: *mut size_t,
    mut cost: *mut c_float,
) {
    if BrotliIsMostlyUTF8(data, pos, mask, len, kMinUTF8Ratio) != 0 {
        EstimateBitCostsForLiteralsUTF8(pos, len, mask, data, histogram, cost);
        return;
    } else {
        let mut window_half: size_t = 2000 as size_t;
        let mut in_window: size_t = brotli_min_size_t(window_half, len);
        let mut i: size_t = 0;
        memset(
            histogram as *mut c_void,
            0 as c_int,
            (256 as size_t).wrapping_mul(::core::mem::size_of::<size_t>() as size_t),
        );
        i = 0 as size_t;
        while i < in_window {
            let ref mut fresh0 =
                *histogram.offset(*data.offset((pos.wrapping_add(i) & mask) as isize) as isize);
            *fresh0 = (*fresh0).wrapping_add(1);
            i = i.wrapping_add(1);
        }
        i = 0 as size_t;
        while i < len {
            let mut histo: size_t = 0;
            if i >= window_half {
                let ref mut fresh1 = *histogram.offset(
                    *data.offset((pos.wrapping_add(i).wrapping_sub(window_half) & mask) as isize)
                        as isize,
                );
                *fresh1 = (*fresh1).wrapping_sub(1);
                in_window = in_window.wrapping_sub(1);
            }
            if i.wrapping_add(window_half) < len {
                let ref mut fresh2 = *histogram.offset(
                    *data.offset((pos.wrapping_add(i).wrapping_add(window_half) & mask) as isize)
                        as isize,
                );
                *fresh2 = (*fresh2).wrapping_add(1);
                in_window = in_window.wrapping_add(1);
            }
            histo = *histogram.offset(*data.offset((pos.wrapping_add(i) & mask) as isize) as isize);
            if histo == 0 as size_t {
                histo = 1 as size_t;
            }
            let mut lit_cost: c_double = FastLog2(in_window) - FastLog2(histo);
            lit_cost += 0.029f64;
            if lit_cost < 1.0f64 {
                lit_cost *= 0.5f64;
                lit_cost += 0.5f64;
            }
            *cost.offset(i as isize) = lit_cost as c_float;
            i = i.wrapping_add(1);
        }
    };
}

static mut kMinUTF8Ratio: c_double = 0.75f64;
