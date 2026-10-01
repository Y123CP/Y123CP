use core::ffi::*;
pub use crate::src::ffi::*;
pub use crate::src::c_consts::*;
pub use crate::src::c_structs::*;
pub use crate::src::c_types::*;

pub type C2RustUnnamed_htdd24ee73 = c_uint;
pub const PNM_P7: C2RustUnnamed_htdd24ee73 = 7;
pub const PNM_P6: C2RustUnnamed_htdd24ee73 = 6;
pub const PNM_P5: C2RustUnnamed_htdd24ee73 = 5;
pub const PNM_P4: C2RustUnnamed_htdd24ee73 = 4;
pub const PNM_P3: C2RustUnnamed_htdd24ee73 = 3;
pub const PNM_P2: C2RustUnnamed_htdd24ee73 = 2;
pub const PNM_P1: C2RustUnnamed_htdd24ee73 = 1;

#[inline]
pub unsafe fn pnm_is_valid(mut pnm_ptr: *const pnm_struct) -> c_int {
    let pnm_ptr_view: &pnm_struct = unsafe { &*pnm_ptr };
    let mut format: c_uint = pnm_ptr_view.format;
    let mut depth: c_uint = pnm_ptr_view.depth;
    let mut width: c_uint = pnm_ptr_view.width;
    let mut height: c_uint = pnm_ptr_view.height;
    let mut maxval: c_uint = pnm_ptr_view.maxval;
    if depth == 0 as c_uint
        || width == 0 as c_uint
        || height == 0 as c_uint
        || maxval == 0 as c_uint
    {
        return 0 as c_int;
    }
    match format {
        1 | 4 => {
            return if depth == 1 as c_uint && maxval == 1 as c_uint {
                1 as c_int
            } else {
                0 as c_int
            };
        }
        2 | 5 => {
            return if depth == 1 as c_uint {
                1 as c_int
            } else {
                0 as c_int
            };
        }
        3 | 6 => {
            return if depth == 3 as c_uint {
                1 as c_int
            } else {
                0 as c_int
            };
        }
        7 => return 1 as c_int,
        _ => return 0 as c_int,
    };
}
#[inline]
pub unsafe fn pnm_raw_sample_size(mut pnm_ptr: *const pnm_struct) -> size_t {
    let pnm_ptr_view: &pnm_struct = unsafe { &*pnm_ptr };
    let mut maxval: c_uint = pnm_ptr_view.maxval;
    if maxval == 0 as c_uint {
        *__errno_location() = EINVAL;
    }
    if maxval <= 0xff as c_uint {
        return 1 as size_t;
    } else if maxval <= 0xffff as c_uint {
        return 2 as size_t;
    } else if maxval <= 0xffffff as c_uint {
        return 3 as size_t;
    } else if maxval <= 0xffffffff as c_uint {
        return 4 as size_t;
    } else {
        *__errno_location() = EINVAL;
        return 0 as size_t;
    };
}
#[inline]
pub unsafe fn pnm_mem_size(
    mut pnm_ptr: *const pnm_struct,
    mut sample_size: size_t,
    mut num_rows: c_uint,
) -> size_t {
    let pnm_ptr_view: &pnm_struct = unsafe { &*pnm_ptr };
    let mut depth: c_uint = pnm_ptr_view.depth;
    let mut width: c_uint = pnm_ptr_view.width;
    if sample_size == 0 as size_t
        || depth == 0 as c_uint
        || width == 0 as c_uint
    {
        *__errno_location() = EINVAL;
        return 0 as size_t;
    }
    if num_rows as size_t
        > (-(1 as c_int) as size_t)
            .wrapping_div(sample_size)
            .wrapping_div(depth as size_t)
            .wrapping_div(width as size_t)
    {
        *__errno_location() = ERANGE;
        return 0 as size_t;
    }
    return sample_size
        .wrapping_mul(depth as size_t)
        .wrapping_mul(width as size_t)
        .wrapping_mul(num_rows as size_t);
}
