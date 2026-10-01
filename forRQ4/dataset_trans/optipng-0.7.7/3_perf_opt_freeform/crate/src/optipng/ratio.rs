use core::ffi::*;
pub use crate::src::c_structs::*;
pub use crate::src::c_types::*;
extern "C" {
    fn vsnprintf(
        __s: *mut c_char,
        __maxlen: size_t,
        __format: *const c_char,
        __arg: ::core::ffi::VaList,
    ) -> c_int;
}

pub type opng_ullong_t = c_ulonglong;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct opng_ullratio {
    pub num: opng_ullong_t,
    pub denom: opng_ullong_t,
}
pub type opng_uxlong_impl_t = opng_ullong_t;

unsafe extern "C" fn opng_snprintf_impl(
    mut buffer: *mut c_char,
    mut buffer_size: size_t,
    mut format: *const c_char,
    mut args: ...
) -> c_int {
    let mut arg_ptr: ::core::ffi::VaListImpl;
    let mut result: c_int = 0;
    arg_ptr = args.clone();
    result = vsnprintf(buffer, buffer_size, format, arg_ptr.as_va_list());
    if result < 0 as c_int || result as size_t >= buffer_size {
        if buffer_size > 0 as size_t {
            *buffer.offset(buffer_size.wrapping_sub(1 as size_t) as isize) =
                '\0' as i32 as c_char;
        }
        return -(1 as c_int);
    }
    return result;
}
unsafe fn opng_sprint_uratio_impl(
    mut buffer: *mut c_char,
    mut buffer_size: size_t,
    mut num: opng_uxlong_impl_t,
    mut denom: opng_uxlong_impl_t,
    mut always_percent: c_int,
) -> c_int {
    let mut integer_part: opng_uxlong_impl_t = 0;
    let mut remainder: opng_uxlong_impl_t = 0;
    let mut fractional_part: c_uint = 0;
    let mut scale: c_uint = 0;
    let mut scaled_ratio: c_double = 0.;
    if denom == 0 as c_ulonglong {
        return opng_snprintf_impl(
            buffer,
            buffer_size,
            if num == 0 as c_ulonglong {
                b"??%%\0" as *const u8 as *const c_char
            } else {
                b"INFINITY%%\0" as *const u8 as *const c_char
            },
        );
    }
    if num < denom
        && denom.wrapping_div(denom.wrapping_sub(num)) < 20000 as c_ulonglong
    {
        scale = 10000 as c_uint;
        scaled_ratio = num as c_double * scale as c_double
            / denom as c_double;
        fractional_part = (scaled_ratio + 0.5f64) as c_uint;
        if fractional_part >= scale {
            fractional_part = scale.wrapping_sub(1 as c_uint);
        }
        return opng_snprintf_impl(
            buffer,
            buffer_size,
            b"%u.%02u%%\0" as *const u8 as *const c_char,
            fractional_part.wrapping_div(100 as c_uint),
            fractional_part.wrapping_rem(100 as c_uint),
        );
    }
    integer_part = num.wrapping_div(denom);
    remainder = num.wrapping_rem(denom);
    scale = 100 as c_uint;
    scaled_ratio = remainder as c_double * scale as c_double
        / denom as c_double;
    fractional_part = (scaled_ratio + 0.5f64) as c_uint;
    if fractional_part >= scale {
        fractional_part = 0 as c_uint;
        integer_part = integer_part.wrapping_add(1);
    }
    if always_percent != 0 {
        return opng_snprintf_impl(
            buffer,
            buffer_size,
            b"%llu%02u%%\0" as *const u8 as *const c_char,
            integer_part,
            fractional_part,
        );
    }
    if integer_part < 100 as c_ulonglong {
        return opng_snprintf_impl(
            buffer,
            buffer_size,
            b"%llu.%02ux\0" as *const u8 as *const c_char,
            integer_part,
            fractional_part,
        );
    }
    integer_part = num.wrapping_div(denom);
    if remainder
        > (denom as c_ulonglong)
            .wrapping_sub(1 as c_ulonglong)
            .wrapping_div(2 as c_ulonglong)
    {
        integer_part = integer_part.wrapping_add(1);
    }
    return opng_snprintf_impl(
        buffer,
        buffer_size,
        b"%llux\0" as *const u8 as *const c_char,
        integer_part,
    );
}
#[inline]
pub unsafe fn opng_ulratio_to_factor_string(
    mut buffer: *mut c_char,
    mut buffer_size: size_t,
    mut ratio: *const opng_ulratio,
) -> c_int {
    let ratio_view: &opng_ulratio = unsafe { &*ratio };
    let mut num: opng_uxlong_impl_t = ratio_view.num as opng_uxlong_impl_t;
    let mut denom: opng_uxlong_impl_t = ratio_view.denom as opng_uxlong_impl_t;
    return opng_sprint_uratio_impl(buffer, buffer_size, num, denom, 0 as c_int);
}
#[inline]
pub unsafe fn opng_ulratio_to_percent_string(
    mut buffer: *mut c_char,
    mut buffer_size: size_t,
    mut ratio: *const opng_ulratio,
) -> c_int {
    let ratio_view: &opng_ulratio = unsafe { &*ratio };
    let mut num: opng_uxlong_impl_t = ratio_view.num as opng_uxlong_impl_t;
    let mut denom: opng_uxlong_impl_t = ratio_view.denom as opng_uxlong_impl_t;
    return opng_sprint_uratio_impl(buffer, buffer_size, num, denom, 1 as c_int);
}
#[inline]
pub unsafe fn opng_ullratio_to_factor_string(
    mut buffer: *mut c_char,
    mut buffer_size: size_t,
    mut ratio: *const opng_ullratio,
) -> c_int {
    let ratio_view: &opng_ullratio = unsafe { &*ratio };
    let mut num: opng_uxlong_impl_t = ratio_view.num as opng_uxlong_impl_t;
    let mut denom: opng_uxlong_impl_t = ratio_view.denom as opng_uxlong_impl_t;
    return opng_sprint_uratio_impl(buffer, buffer_size, num, denom, 0 as c_int);
}
#[inline]
pub unsafe fn opng_ullratio_to_percent_string(
    mut buffer: *mut c_char,
    mut buffer_size: size_t,
    mut ratio: *const opng_ullratio,
) -> c_int {
    let ratio_view: &opng_ullratio = unsafe { &*ratio };
    let mut num: opng_uxlong_impl_t = ratio_view.num as opng_uxlong_impl_t;
    let mut denom: opng_uxlong_impl_t = ratio_view.denom as opng_uxlong_impl_t;
    return opng_sprint_uratio_impl(buffer, buffer_size, num, denom, 1 as c_int);
}
