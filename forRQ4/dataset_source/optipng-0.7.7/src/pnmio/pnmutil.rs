extern "C" {
    fn __errno_location() -> *mut ::core::ffi::c_int;
}
pub type size_t = usize;
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const PNM_P7: C2RustUnnamed = 7;
pub const PNM_P6: C2RustUnnamed = 6;
pub const PNM_P5: C2RustUnnamed = 5;
pub const PNM_P4: C2RustUnnamed = 4;
pub const PNM_P3: C2RustUnnamed = 3;
pub const PNM_P2: C2RustUnnamed = 2;
pub const PNM_P1: C2RustUnnamed = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pnm_struct {
    pub format: ::core::ffi::c_uint,
    pub depth: ::core::ffi::c_uint,
    pub width: ::core::ffi::c_uint,
    pub height: ::core::ffi::c_uint,
    pub maxval: ::core::ffi::c_uint,
}
pub const EINVAL: ::core::ffi::c_int = 22 as ::core::ffi::c_int;
pub const ERANGE: ::core::ffi::c_int = 34 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn pnm_is_valid(mut pnm_ptr: *const pnm_struct) -> ::core::ffi::c_int {
    let mut format: ::core::ffi::c_uint = (*pnm_ptr).format;
    let mut depth: ::core::ffi::c_uint = (*pnm_ptr).depth;
    let mut width: ::core::ffi::c_uint = (*pnm_ptr).width;
    let mut height: ::core::ffi::c_uint = (*pnm_ptr).height;
    let mut maxval: ::core::ffi::c_uint = (*pnm_ptr).maxval;
    if depth == 0 as ::core::ffi::c_uint
        || width == 0 as ::core::ffi::c_uint
        || height == 0 as ::core::ffi::c_uint
        || maxval == 0 as ::core::ffi::c_uint
    {
        return 0 as ::core::ffi::c_int;
    }
    match format {
        1 | 4 => {
            return if depth == 1 as ::core::ffi::c_uint && maxval == 1 as ::core::ffi::c_uint {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            };
        }
        2 | 5 => {
            return if depth == 1 as ::core::ffi::c_uint {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            };
        }
        3 | 6 => {
            return if depth == 3 as ::core::ffi::c_uint {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            };
        }
        7 => return 1 as ::core::ffi::c_int,
        _ => return 0 as ::core::ffi::c_int,
    };
}
#[no_mangle]
pub unsafe extern "C" fn pnm_raw_sample_size(mut pnm_ptr: *const pnm_struct) -> size_t {
    let mut maxval: ::core::ffi::c_uint = (*pnm_ptr).maxval;
    if maxval == 0 as ::core::ffi::c_uint {
        *__errno_location() = EINVAL;
    }
    if maxval <= 0xff as ::core::ffi::c_uint {
        return 1 as size_t;
    } else if maxval <= 0xffff as ::core::ffi::c_uint {
        return 2 as size_t;
    } else if maxval <= 0xffffff as ::core::ffi::c_uint {
        return 3 as size_t;
    } else if maxval <= 0xffffffff as ::core::ffi::c_uint {
        return 4 as size_t;
    } else {
        *__errno_location() = EINVAL;
        return 0 as size_t;
    };
}
#[no_mangle]
pub unsafe extern "C" fn pnm_mem_size(
    mut pnm_ptr: *const pnm_struct,
    mut sample_size: size_t,
    mut num_rows: ::core::ffi::c_uint,
) -> size_t {
    let mut depth: ::core::ffi::c_uint = (*pnm_ptr).depth;
    let mut width: ::core::ffi::c_uint = (*pnm_ptr).width;
    if sample_size == 0 as size_t
        || depth == 0 as ::core::ffi::c_uint
        || width == 0 as ::core::ffi::c_uint
    {
        *__errno_location() = EINVAL;
        return 0 as size_t;
    }
    if num_rows as size_t
        > (-(1 as ::core::ffi::c_int) as size_t)
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
