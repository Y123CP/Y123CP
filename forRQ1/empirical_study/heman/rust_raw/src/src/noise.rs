extern "C" {
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
}
pub type __int8_t = i8;
pub type __int16_t = i16;
pub type __int64_t = i64;
pub type size_t = usize;
pub type int8_t = __int8_t;
pub type int16_t = __int16_t;
pub type int64_t = __int64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct osn_context {
    pub perm: *mut int16_t,
    pub permGradIndex3D: *mut int16_t,
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const STRETCH_CONSTANT_2D: ::core::ffi::c_double = -0.211324865405187f64;
pub const SQUISH_CONSTANT_2D: ::core::ffi::c_double = 0.366025403784439f64;
pub const STRETCH_CONSTANT_3D: ::core::ffi::c_double = -1.0f64 / 6.0f64;
pub const SQUISH_CONSTANT_3D: ::core::ffi::c_double = 1.0f64 / 3.0f64;
pub const STRETCH_CONSTANT_4D: ::core::ffi::c_double = -0.138196601125011f64;
pub const SQUISH_CONSTANT_4D: ::core::ffi::c_double = 0.309016994374947f64;
pub const NORM_CONSTANT_2D: ::core::ffi::c_double = 47.0f64;
pub const NORM_CONSTANT_3D: ::core::ffi::c_double = 103.0f64;
pub const NORM_CONSTANT_4D: ::core::ffi::c_double = 30.0f64;
static mut gradients2D: [int8_t; 16] = [
    5 as ::core::ffi::c_int as int8_t,
    2 as ::core::ffi::c_int as int8_t,
    2 as ::core::ffi::c_int as int8_t,
    5 as ::core::ffi::c_int as int8_t,
    -(5 as ::core::ffi::c_int) as int8_t,
    2 as ::core::ffi::c_int as int8_t,
    -(2 as ::core::ffi::c_int) as int8_t,
    5 as ::core::ffi::c_int as int8_t,
    5 as ::core::ffi::c_int as int8_t,
    -(2 as ::core::ffi::c_int) as int8_t,
    2 as ::core::ffi::c_int as int8_t,
    -(5 as ::core::ffi::c_int) as int8_t,
    -(5 as ::core::ffi::c_int) as int8_t,
    -(2 as ::core::ffi::c_int) as int8_t,
    -(2 as ::core::ffi::c_int) as int8_t,
    -(5 as ::core::ffi::c_int) as int8_t,
];
static mut gradients3D: [::core::ffi::c_schar; 72] = [
    -(11 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    4 as ::core::ffi::c_int as ::core::ffi::c_schar,
    4 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(4 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    11 as ::core::ffi::c_int as ::core::ffi::c_schar,
    4 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(4 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    4 as ::core::ffi::c_int as ::core::ffi::c_schar,
    11 as ::core::ffi::c_int as ::core::ffi::c_schar,
    11 as ::core::ffi::c_int as ::core::ffi::c_schar,
    4 as ::core::ffi::c_int as ::core::ffi::c_schar,
    4 as ::core::ffi::c_int as ::core::ffi::c_schar,
    4 as ::core::ffi::c_int as ::core::ffi::c_schar,
    11 as ::core::ffi::c_int as ::core::ffi::c_schar,
    4 as ::core::ffi::c_int as ::core::ffi::c_schar,
    4 as ::core::ffi::c_int as ::core::ffi::c_schar,
    4 as ::core::ffi::c_int as ::core::ffi::c_schar,
    11 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(11 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(4 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    4 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(4 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(11 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    4 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(4 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(4 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    11 as ::core::ffi::c_int as ::core::ffi::c_schar,
    11 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(4 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    4 as ::core::ffi::c_int as ::core::ffi::c_schar,
    4 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(11 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    4 as ::core::ffi::c_int as ::core::ffi::c_schar,
    4 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(4 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    11 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(11 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    4 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(4 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(4 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    11 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(4 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(4 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    4 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(11 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    11 as ::core::ffi::c_int as ::core::ffi::c_schar,
    4 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(4 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    4 as ::core::ffi::c_int as ::core::ffi::c_schar,
    11 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(4 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    4 as ::core::ffi::c_int as ::core::ffi::c_schar,
    4 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(11 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(11 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(4 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(4 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(4 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(11 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(4 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(4 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(4 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(11 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    11 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(4 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(4 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    4 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(11 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(4 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    4 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(4 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(11 as ::core::ffi::c_int) as ::core::ffi::c_schar,
];
static mut gradients4D: [::core::ffi::c_schar; 256] = [
    3 as ::core::ffi::c_int as ::core::ffi::c_schar,
    1 as ::core::ffi::c_int as ::core::ffi::c_schar,
    1 as ::core::ffi::c_int as ::core::ffi::c_schar,
    1 as ::core::ffi::c_int as ::core::ffi::c_schar,
    1 as ::core::ffi::c_int as ::core::ffi::c_schar,
    3 as ::core::ffi::c_int as ::core::ffi::c_schar,
    1 as ::core::ffi::c_int as ::core::ffi::c_schar,
    1 as ::core::ffi::c_int as ::core::ffi::c_schar,
    1 as ::core::ffi::c_int as ::core::ffi::c_schar,
    1 as ::core::ffi::c_int as ::core::ffi::c_schar,
    3 as ::core::ffi::c_int as ::core::ffi::c_schar,
    1 as ::core::ffi::c_int as ::core::ffi::c_schar,
    1 as ::core::ffi::c_int as ::core::ffi::c_schar,
    1 as ::core::ffi::c_int as ::core::ffi::c_schar,
    1 as ::core::ffi::c_int as ::core::ffi::c_schar,
    3 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    1 as ::core::ffi::c_int as ::core::ffi::c_schar,
    1 as ::core::ffi::c_int as ::core::ffi::c_schar,
    1 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    3 as ::core::ffi::c_int as ::core::ffi::c_schar,
    1 as ::core::ffi::c_int as ::core::ffi::c_schar,
    1 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    1 as ::core::ffi::c_int as ::core::ffi::c_schar,
    3 as ::core::ffi::c_int as ::core::ffi::c_schar,
    1 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    1 as ::core::ffi::c_int as ::core::ffi::c_schar,
    1 as ::core::ffi::c_int as ::core::ffi::c_schar,
    3 as ::core::ffi::c_int as ::core::ffi::c_schar,
    3 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    1 as ::core::ffi::c_int as ::core::ffi::c_schar,
    1 as ::core::ffi::c_int as ::core::ffi::c_schar,
    1 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    1 as ::core::ffi::c_int as ::core::ffi::c_schar,
    1 as ::core::ffi::c_int as ::core::ffi::c_schar,
    1 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    3 as ::core::ffi::c_int as ::core::ffi::c_schar,
    1 as ::core::ffi::c_int as ::core::ffi::c_schar,
    1 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    1 as ::core::ffi::c_int as ::core::ffi::c_schar,
    3 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    1 as ::core::ffi::c_int as ::core::ffi::c_schar,
    1 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    1 as ::core::ffi::c_int as ::core::ffi::c_schar,
    1 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    3 as ::core::ffi::c_int as ::core::ffi::c_schar,
    1 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    1 as ::core::ffi::c_int as ::core::ffi::c_schar,
    3 as ::core::ffi::c_int as ::core::ffi::c_schar,
    3 as ::core::ffi::c_int as ::core::ffi::c_schar,
    1 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    1 as ::core::ffi::c_int as ::core::ffi::c_schar,
    1 as ::core::ffi::c_int as ::core::ffi::c_schar,
    3 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    1 as ::core::ffi::c_int as ::core::ffi::c_schar,
    1 as ::core::ffi::c_int as ::core::ffi::c_schar,
    1 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    1 as ::core::ffi::c_int as ::core::ffi::c_schar,
    1 as ::core::ffi::c_int as ::core::ffi::c_schar,
    1 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    3 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    1 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    1 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    3 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    1 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    1 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    1 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    1 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    3 as ::core::ffi::c_int as ::core::ffi::c_schar,
    3 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    1 as ::core::ffi::c_int as ::core::ffi::c_schar,
    1 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    1 as ::core::ffi::c_int as ::core::ffi::c_schar,
    1 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    1 as ::core::ffi::c_int as ::core::ffi::c_schar,
    1 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    3 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    1 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    1 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    1 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    3 as ::core::ffi::c_int as ::core::ffi::c_schar,
    3 as ::core::ffi::c_int as ::core::ffi::c_schar,
    1 as ::core::ffi::c_int as ::core::ffi::c_schar,
    1 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    1 as ::core::ffi::c_int as ::core::ffi::c_schar,
    3 as ::core::ffi::c_int as ::core::ffi::c_schar,
    1 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    1 as ::core::ffi::c_int as ::core::ffi::c_schar,
    1 as ::core::ffi::c_int as ::core::ffi::c_schar,
    3 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    1 as ::core::ffi::c_int as ::core::ffi::c_schar,
    1 as ::core::ffi::c_int as ::core::ffi::c_schar,
    1 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    1 as ::core::ffi::c_int as ::core::ffi::c_schar,
    1 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    3 as ::core::ffi::c_int as ::core::ffi::c_schar,
    1 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    1 as ::core::ffi::c_int as ::core::ffi::c_schar,
    3 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    1 as ::core::ffi::c_int as ::core::ffi::c_schar,
    1 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    3 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    1 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    1 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    1 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    1 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    3 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    1 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    1 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    1 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    1 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    3 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    1 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    3 as ::core::ffi::c_int as ::core::ffi::c_schar,
    1 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    1 as ::core::ffi::c_int as ::core::ffi::c_schar,
    3 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    1 as ::core::ffi::c_int as ::core::ffi::c_schar,
    1 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    1 as ::core::ffi::c_int as ::core::ffi::c_schar,
    1 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    1 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    3 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    1 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    1 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    3 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    1 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    1 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    1 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
];
unsafe extern "C" fn extrapolate2(
    mut ctx: *mut osn_context,
    mut xsb: ::core::ffi::c_int,
    mut ysb: ::core::ffi::c_int,
    mut dx: ::core::ffi::c_double,
    mut dy: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut perm: *mut int16_t = (*ctx).perm;
    let mut index: ::core::ffi::c_int = *perm.offset(
        (*perm.offset((xsb & 0xff as ::core::ffi::c_int) as isize) as ::core::ffi::c_int + ysb
            & 0xff as ::core::ffi::c_int) as isize,
    ) as ::core::ffi::c_int
        & 0xe as ::core::ffi::c_int;
    return gradients2D[index as usize] as ::core::ffi::c_int as ::core::ffi::c_double * dx
        + gradients2D[(index + 1 as ::core::ffi::c_int) as usize] as ::core::ffi::c_int
            as ::core::ffi::c_double
            * dy;
}
unsafe extern "C" fn extrapolate3(
    mut ctx: *mut osn_context,
    mut xsb: ::core::ffi::c_int,
    mut ysb: ::core::ffi::c_int,
    mut zsb: ::core::ffi::c_int,
    mut dx: ::core::ffi::c_double,
    mut dy: ::core::ffi::c_double,
    mut dz: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut perm: *mut int16_t = (*ctx).perm;
    let mut permGradIndex3D: *mut int16_t = (*ctx).permGradIndex3D;
    let mut index: ::core::ffi::c_int = *permGradIndex3D.offset(
        (*perm.offset(
            (*perm.offset((xsb & 0xff as ::core::ffi::c_int) as isize) as ::core::ffi::c_int + ysb
                & 0xff as ::core::ffi::c_int) as isize,
        ) as ::core::ffi::c_int
            + zsb
            & 0xff as ::core::ffi::c_int) as isize,
    ) as ::core::ffi::c_int;
    return gradients3D[index as usize] as ::core::ffi::c_int as ::core::ffi::c_double * dx
        + gradients3D[(index + 1 as ::core::ffi::c_int) as usize] as ::core::ffi::c_int
            as ::core::ffi::c_double
            * dy
        + gradients3D[(index + 2 as ::core::ffi::c_int) as usize] as ::core::ffi::c_int
            as ::core::ffi::c_double
            * dz;
}
unsafe extern "C" fn extrapolate4(
    mut ctx: *mut osn_context,
    mut xsb: ::core::ffi::c_int,
    mut ysb: ::core::ffi::c_int,
    mut zsb: ::core::ffi::c_int,
    mut wsb: ::core::ffi::c_int,
    mut dx: ::core::ffi::c_double,
    mut dy: ::core::ffi::c_double,
    mut dz: ::core::ffi::c_double,
    mut dw: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut perm: *mut int16_t = (*ctx).perm;
    let mut index: ::core::ffi::c_int = *perm.offset(
        (*perm.offset(
            (*perm.offset(
                (*perm.offset((xsb & 0xff as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                    + ysb
                    & 0xff as ::core::ffi::c_int) as isize,
            ) as ::core::ffi::c_int
                + zsb
                & 0xff as ::core::ffi::c_int) as isize,
        ) as ::core::ffi::c_int
            + wsb
            & 0xff as ::core::ffi::c_int) as isize,
    ) as ::core::ffi::c_int
        & 0xfc as ::core::ffi::c_int;
    return gradients4D[index as usize] as ::core::ffi::c_int as ::core::ffi::c_double * dx
        + gradients4D[(index + 1 as ::core::ffi::c_int) as usize] as ::core::ffi::c_int
            as ::core::ffi::c_double
            * dy
        + gradients4D[(index + 2 as ::core::ffi::c_int) as usize] as ::core::ffi::c_int
            as ::core::ffi::c_double
            * dz
        + gradients4D[(index + 3 as ::core::ffi::c_int) as usize] as ::core::ffi::c_int
            as ::core::ffi::c_double
            * dw;
}
#[inline]
unsafe extern "C" fn fastFloor(mut x: ::core::ffi::c_double) -> ::core::ffi::c_int {
    let mut xi: ::core::ffi::c_int = x as ::core::ffi::c_int;
    return if x < xi as ::core::ffi::c_double {
        xi - 1 as ::core::ffi::c_int
    } else {
        xi
    };
}
unsafe extern "C" fn allocate_perm(
    mut ctx: *mut osn_context,
    mut nperm: ::core::ffi::c_int,
    mut ngrad: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if !(*ctx).perm.is_null() {
        free((*ctx).perm as *mut ::core::ffi::c_void);
    }
    if !(*ctx).permGradIndex3D.is_null() {
        free((*ctx).permGradIndex3D as *mut ::core::ffi::c_void);
    }
    (*ctx).perm =
        malloc((::core::mem::size_of::<int16_t>() as size_t).wrapping_mul(nperm as size_t))
            as *mut int16_t;
    if (*ctx).perm.is_null() {
        return -ENOMEM;
    }
    (*ctx).permGradIndex3D =
        malloc((::core::mem::size_of::<int16_t>() as size_t).wrapping_mul(ngrad as size_t))
            as *mut int16_t;
    if (*ctx).permGradIndex3D.is_null() {
        free((*ctx).perm as *mut ::core::ffi::c_void);
        return -ENOMEM;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn open_simplex_noise_init_perm(
    mut ctx: *mut osn_context,
    mut p: *mut int16_t,
    mut nelements: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut rc: ::core::ffi::c_int = 0;
    rc = allocate_perm(ctx, nelements, 256 as ::core::ffi::c_int);
    if rc != 0 {
        return rc;
    }
    memcpy(
        (*ctx).perm as *mut ::core::ffi::c_void,
        p as *const ::core::ffi::c_void,
        (::core::mem::size_of::<int16_t>() as size_t).wrapping_mul(nelements as size_t),
    );
    i = 0 as ::core::ffi::c_int;
    while i < 256 as ::core::ffi::c_int {
        *(*ctx).permGradIndex3D.offset(i as isize) = (*(*ctx).perm.offset(i as isize) as usize)
            .wrapping_rem(
                (::core::mem::size_of::<[::core::ffi::c_schar; 72]>() as usize)
                    .wrapping_div(::core::mem::size_of::<::core::ffi::c_schar>() as usize)
                    .wrapping_div(3 as usize),
            )
            .wrapping_mul(3 as usize)
            as int16_t;
        i += 1;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn open_simplex_noise(
    mut seed: int64_t,
    mut ctx: *mut *mut osn_context,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = 0;
    let mut source: [int16_t; 256] = [0; 256];
    let mut i: ::core::ffi::c_int = 0;
    let mut perm: *mut int16_t = ::core::ptr::null_mut::<int16_t>();
    let mut permGradIndex3D: *mut int16_t = ::core::ptr::null_mut::<int16_t>();
    *ctx = malloc(::core::mem::size_of::<osn_context>() as size_t) as *mut osn_context;
    if (*ctx).is_null() {
        return -ENOMEM;
    }
    (**ctx).perm = ::core::ptr::null_mut::<int16_t>();
    (**ctx).permGradIndex3D = ::core::ptr::null_mut::<int16_t>();
    rc = allocate_perm(*ctx, 256 as ::core::ffi::c_int, 256 as ::core::ffi::c_int);
    if rc != 0 {
        free(*ctx as *mut ::core::ffi::c_void);
        return rc;
    }
    perm = (**ctx).perm;
    permGradIndex3D = (**ctx).permGradIndex3D;
    i = 0 as ::core::ffi::c_int;
    while i < 256 as ::core::ffi::c_int {
        source[i as usize] = i as int16_t;
        i += 1;
    }
    seed = (seed as ::core::ffi::c_longlong * 6364136223846793005 as ::core::ffi::c_longlong
        + 1442695040888963407 as ::core::ffi::c_longlong) as int64_t;
    seed = (seed as ::core::ffi::c_longlong * 6364136223846793005 as ::core::ffi::c_longlong
        + 1442695040888963407 as ::core::ffi::c_longlong) as int64_t;
    seed = (seed as ::core::ffi::c_longlong * 6364136223846793005 as ::core::ffi::c_longlong
        + 1442695040888963407 as ::core::ffi::c_longlong) as int64_t;
    i = 255 as ::core::ffi::c_int;
    while i >= 0 as ::core::ffi::c_int {
        seed = (seed as ::core::ffi::c_longlong * 6364136223846793005 as ::core::ffi::c_longlong
            + 1442695040888963407 as ::core::ffi::c_longlong) as int64_t;
        let mut r: ::core::ffi::c_int = ((seed + 31 as int64_t)
            % (i + 1 as ::core::ffi::c_int) as int64_t)
            as ::core::ffi::c_int;
        if r < 0 as ::core::ffi::c_int {
            r += i + 1 as ::core::ffi::c_int;
        }
        *perm.offset(i as isize) = source[r as usize];
        *permGradIndex3D.offset(i as isize) = (*perm.offset(i as isize) as usize)
            .wrapping_rem(
                (::core::mem::size_of::<[::core::ffi::c_schar; 72]>() as usize)
                    .wrapping_div(::core::mem::size_of::<::core::ffi::c_schar>() as usize)
                    .wrapping_div(3 as usize),
            )
            .wrapping_mul(3 as usize)
            as ::core::ffi::c_short as int16_t;
        source[r as usize] = source[i as usize];
        i -= 1;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn open_simplex_noise_free(mut ctx: *mut osn_context) {
    if ctx.is_null() {
        return;
    }
    if !(*ctx).perm.is_null() {
        free((*ctx).perm as *mut ::core::ffi::c_void);
        (*ctx).perm = ::core::ptr::null_mut::<int16_t>();
    }
    if !(*ctx).permGradIndex3D.is_null() {
        free((*ctx).permGradIndex3D as *mut ::core::ffi::c_void);
        (*ctx).permGradIndex3D = ::core::ptr::null_mut::<int16_t>();
    }
    free(ctx as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn open_simplex_noise2(
    mut ctx: *mut osn_context,
    mut x: ::core::ffi::c_double,
    mut y: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut stretchOffset: ::core::ffi::c_double = (x + y) * STRETCH_CONSTANT_2D;
    let mut xs: ::core::ffi::c_double = x + stretchOffset;
    let mut ys: ::core::ffi::c_double = y + stretchOffset;
    let mut xsb: ::core::ffi::c_int = fastFloor(xs);
    let mut ysb: ::core::ffi::c_int = fastFloor(ys);
    let mut squishOffset: ::core::ffi::c_double =
        (xsb + ysb) as ::core::ffi::c_double * SQUISH_CONSTANT_2D;
    let mut xb: ::core::ffi::c_double = xsb as ::core::ffi::c_double + squishOffset;
    let mut yb: ::core::ffi::c_double = ysb as ::core::ffi::c_double + squishOffset;
    let mut xins: ::core::ffi::c_double = xs - xsb as ::core::ffi::c_double;
    let mut yins: ::core::ffi::c_double = ys - ysb as ::core::ffi::c_double;
    let mut inSum: ::core::ffi::c_double = xins + yins;
    let mut dx0: ::core::ffi::c_double = x - xb;
    let mut dy0: ::core::ffi::c_double = y - yb;
    let mut dx_ext: ::core::ffi::c_double = 0.;
    let mut dy_ext: ::core::ffi::c_double = 0.;
    let mut xsv_ext: ::core::ffi::c_int = 0;
    let mut ysv_ext: ::core::ffi::c_int = 0;
    let mut value: ::core::ffi::c_double = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    let mut dx1: ::core::ffi::c_double =
        dx0 - 1 as ::core::ffi::c_int as ::core::ffi::c_double - SQUISH_CONSTANT_2D;
    let mut dy1: ::core::ffi::c_double =
        dy0 - 0 as ::core::ffi::c_int as ::core::ffi::c_double - SQUISH_CONSTANT_2D;
    let mut attn1: ::core::ffi::c_double =
        2 as ::core::ffi::c_int as ::core::ffi::c_double - dx1 * dx1 - dy1 * dy1;
    if attn1 > 0 as ::core::ffi::c_int as ::core::ffi::c_double {
        attn1 *= attn1;
        value += attn1
            * attn1
            * extrapolate2(
                ctx,
                xsb + 1 as ::core::ffi::c_int,
                ysb + 0 as ::core::ffi::c_int,
                dx1,
                dy1,
            );
    }
    let mut dx2: ::core::ffi::c_double =
        dx0 - 0 as ::core::ffi::c_int as ::core::ffi::c_double - SQUISH_CONSTANT_2D;
    let mut dy2: ::core::ffi::c_double =
        dy0 - 1 as ::core::ffi::c_int as ::core::ffi::c_double - SQUISH_CONSTANT_2D;
    let mut attn2: ::core::ffi::c_double =
        2 as ::core::ffi::c_int as ::core::ffi::c_double - dx2 * dx2 - dy2 * dy2;
    if attn2 > 0 as ::core::ffi::c_int as ::core::ffi::c_double {
        attn2 *= attn2;
        value += attn2
            * attn2
            * extrapolate2(
                ctx,
                xsb + 0 as ::core::ffi::c_int,
                ysb + 1 as ::core::ffi::c_int,
                dx2,
                dy2,
            );
    }
    if inSum <= 1 as ::core::ffi::c_int as ::core::ffi::c_double {
        let mut zins: ::core::ffi::c_double =
            1 as ::core::ffi::c_int as ::core::ffi::c_double - inSum;
        if zins > xins || zins > yins {
            if xins > yins {
                xsv_ext = xsb + 1 as ::core::ffi::c_int;
                ysv_ext = ysb - 1 as ::core::ffi::c_int;
                dx_ext = dx0 - 1 as ::core::ffi::c_int as ::core::ffi::c_double;
                dy_ext = dy0 + 1 as ::core::ffi::c_int as ::core::ffi::c_double;
            } else {
                xsv_ext = xsb - 1 as ::core::ffi::c_int;
                ysv_ext = ysb + 1 as ::core::ffi::c_int;
                dx_ext = dx0 + 1 as ::core::ffi::c_int as ::core::ffi::c_double;
                dy_ext = dy0 - 1 as ::core::ffi::c_int as ::core::ffi::c_double;
            }
        } else {
            xsv_ext = xsb + 1 as ::core::ffi::c_int;
            ysv_ext = ysb + 1 as ::core::ffi::c_int;
            dx_ext = dx0
                - 1 as ::core::ffi::c_int as ::core::ffi::c_double
                - 2 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_2D;
            dy_ext = dy0
                - 1 as ::core::ffi::c_int as ::core::ffi::c_double
                - 2 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_2D;
        }
    } else {
        let mut zins_0: ::core::ffi::c_double =
            2 as ::core::ffi::c_int as ::core::ffi::c_double - inSum;
        if zins_0 < xins || zins_0 < yins {
            if xins > yins {
                xsv_ext = xsb + 2 as ::core::ffi::c_int;
                ysv_ext = ysb + 0 as ::core::ffi::c_int;
                dx_ext = dx0
                    - 2 as ::core::ffi::c_int as ::core::ffi::c_double
                    - 2 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_2D;
                dy_ext = dy0 + 0 as ::core::ffi::c_int as ::core::ffi::c_double
                    - 2 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_2D;
            } else {
                xsv_ext = xsb + 0 as ::core::ffi::c_int;
                ysv_ext = ysb + 2 as ::core::ffi::c_int;
                dx_ext = dx0 + 0 as ::core::ffi::c_int as ::core::ffi::c_double
                    - 2 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_2D;
                dy_ext = dy0
                    - 2 as ::core::ffi::c_int as ::core::ffi::c_double
                    - 2 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_2D;
            }
        } else {
            dx_ext = dx0;
            dy_ext = dy0;
            xsv_ext = xsb;
            ysv_ext = ysb;
        }
        xsb += 1 as ::core::ffi::c_int;
        ysb += 1 as ::core::ffi::c_int;
        dx0 = dx0
            - 1 as ::core::ffi::c_int as ::core::ffi::c_double
            - 2 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_2D;
        dy0 = dy0
            - 1 as ::core::ffi::c_int as ::core::ffi::c_double
            - 2 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_2D;
    }
    let mut attn0: ::core::ffi::c_double =
        2 as ::core::ffi::c_int as ::core::ffi::c_double - dx0 * dx0 - dy0 * dy0;
    if attn0 > 0 as ::core::ffi::c_int as ::core::ffi::c_double {
        attn0 *= attn0;
        value += attn0 * attn0 * extrapolate2(ctx, xsb, ysb, dx0, dy0);
    }
    let mut attn_ext: ::core::ffi::c_double =
        2 as ::core::ffi::c_int as ::core::ffi::c_double - dx_ext * dx_ext - dy_ext * dy_ext;
    if attn_ext > 0 as ::core::ffi::c_int as ::core::ffi::c_double {
        attn_ext *= attn_ext;
        value += attn_ext * attn_ext * extrapolate2(ctx, xsv_ext, ysv_ext, dx_ext, dy_ext);
    }
    return value / NORM_CONSTANT_2D;
}
#[no_mangle]
pub unsafe extern "C" fn open_simplex_noise3(
    mut ctx: *mut osn_context,
    mut x: ::core::ffi::c_double,
    mut y: ::core::ffi::c_double,
    mut z: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut stretchOffset: ::core::ffi::c_double = (x + y + z) * STRETCH_CONSTANT_3D;
    let mut xs: ::core::ffi::c_double = x + stretchOffset;
    let mut ys: ::core::ffi::c_double = y + stretchOffset;
    let mut zs: ::core::ffi::c_double = z + stretchOffset;
    let mut xsb: ::core::ffi::c_int = fastFloor(xs);
    let mut ysb: ::core::ffi::c_int = fastFloor(ys);
    let mut zsb: ::core::ffi::c_int = fastFloor(zs);
    let mut squishOffset: ::core::ffi::c_double =
        (xsb + ysb + zsb) as ::core::ffi::c_double * SQUISH_CONSTANT_3D;
    let mut xb: ::core::ffi::c_double = xsb as ::core::ffi::c_double + squishOffset;
    let mut yb: ::core::ffi::c_double = ysb as ::core::ffi::c_double + squishOffset;
    let mut zb: ::core::ffi::c_double = zsb as ::core::ffi::c_double + squishOffset;
    let mut xins: ::core::ffi::c_double = xs - xsb as ::core::ffi::c_double;
    let mut yins: ::core::ffi::c_double = ys - ysb as ::core::ffi::c_double;
    let mut zins: ::core::ffi::c_double = zs - zsb as ::core::ffi::c_double;
    let mut inSum: ::core::ffi::c_double = xins + yins + zins;
    let mut dx0: ::core::ffi::c_double = x - xb;
    let mut dy0: ::core::ffi::c_double = y - yb;
    let mut dz0: ::core::ffi::c_double = z - zb;
    let mut dx_ext0: ::core::ffi::c_double = 0.;
    let mut dy_ext0: ::core::ffi::c_double = 0.;
    let mut dz_ext0: ::core::ffi::c_double = 0.;
    let mut dx_ext1: ::core::ffi::c_double = 0.;
    let mut dy_ext1: ::core::ffi::c_double = 0.;
    let mut dz_ext1: ::core::ffi::c_double = 0.;
    let mut xsv_ext0: ::core::ffi::c_int = 0;
    let mut ysv_ext0: ::core::ffi::c_int = 0;
    let mut zsv_ext0: ::core::ffi::c_int = 0;
    let mut xsv_ext1: ::core::ffi::c_int = 0;
    let mut ysv_ext1: ::core::ffi::c_int = 0;
    let mut zsv_ext1: ::core::ffi::c_int = 0;
    let mut value: ::core::ffi::c_double = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    if inSum <= 1 as ::core::ffi::c_int as ::core::ffi::c_double {
        let mut aPoint: int8_t = 0x1 as int8_t;
        let mut aScore: ::core::ffi::c_double = xins;
        let mut bPoint: int8_t = 0x2 as int8_t;
        let mut bScore: ::core::ffi::c_double = yins;
        if aScore >= bScore && zins > bScore {
            bScore = zins;
            bPoint = 0x4 as int8_t;
        } else if aScore < bScore && zins > aScore {
            aScore = zins;
            aPoint = 0x4 as int8_t;
        }
        let mut wins: ::core::ffi::c_double =
            1 as ::core::ffi::c_int as ::core::ffi::c_double - inSum;
        if wins > aScore || wins > bScore {
            let mut c: int8_t = (if bScore > aScore {
                bPoint as ::core::ffi::c_int
            } else {
                aPoint as ::core::ffi::c_int
            }) as int8_t;
            if c as ::core::ffi::c_int & 0x1 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                xsv_ext0 = xsb - 1 as ::core::ffi::c_int;
                xsv_ext1 = xsb;
                dx_ext0 = dx0 + 1 as ::core::ffi::c_int as ::core::ffi::c_double;
                dx_ext1 = dx0;
            } else {
                xsv_ext1 = xsb + 1 as ::core::ffi::c_int;
                xsv_ext0 = xsv_ext1;
                dx_ext1 = dx0 - 1 as ::core::ffi::c_int as ::core::ffi::c_double;
                dx_ext0 = dx_ext1;
            }
            if c as ::core::ffi::c_int & 0x2 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                ysv_ext1 = ysb;
                ysv_ext0 = ysv_ext1;
                dy_ext1 = dy0;
                dy_ext0 = dy_ext1;
                if c as ::core::ffi::c_int & 0x1 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                    ysv_ext1 -= 1 as ::core::ffi::c_int;
                    dy_ext1 += 1 as ::core::ffi::c_int as ::core::ffi::c_double;
                } else {
                    ysv_ext0 -= 1 as ::core::ffi::c_int;
                    dy_ext0 += 1 as ::core::ffi::c_int as ::core::ffi::c_double;
                }
            } else {
                ysv_ext1 = ysb + 1 as ::core::ffi::c_int;
                ysv_ext0 = ysv_ext1;
                dy_ext1 = dy0 - 1 as ::core::ffi::c_int as ::core::ffi::c_double;
                dy_ext0 = dy_ext1;
            }
            if c as ::core::ffi::c_int & 0x4 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                zsv_ext0 = zsb;
                zsv_ext1 = zsb - 1 as ::core::ffi::c_int;
                dz_ext0 = dz0;
                dz_ext1 = dz0 + 1 as ::core::ffi::c_int as ::core::ffi::c_double;
            } else {
                zsv_ext1 = zsb + 1 as ::core::ffi::c_int;
                zsv_ext0 = zsv_ext1;
                dz_ext1 = dz0 - 1 as ::core::ffi::c_int as ::core::ffi::c_double;
                dz_ext0 = dz_ext1;
            }
        } else {
            let mut c_0: int8_t =
                (aPoint as ::core::ffi::c_int | bPoint as ::core::ffi::c_int) as int8_t;
            if c_0 as ::core::ffi::c_int & 0x1 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                xsv_ext0 = xsb;
                xsv_ext1 = xsb - 1 as ::core::ffi::c_int;
                dx_ext0 =
                    dx0 - 2 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_3D;
                dx_ext1 =
                    dx0 + 1 as ::core::ffi::c_int as ::core::ffi::c_double - SQUISH_CONSTANT_3D;
            } else {
                xsv_ext1 = xsb + 1 as ::core::ffi::c_int;
                xsv_ext0 = xsv_ext1;
                dx_ext0 = dx0
                    - 1 as ::core::ffi::c_int as ::core::ffi::c_double
                    - 2 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_3D;
                dx_ext1 =
                    dx0 - 1 as ::core::ffi::c_int as ::core::ffi::c_double - SQUISH_CONSTANT_3D;
            }
            if c_0 as ::core::ffi::c_int & 0x2 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                ysv_ext0 = ysb;
                ysv_ext1 = ysb - 1 as ::core::ffi::c_int;
                dy_ext0 =
                    dy0 - 2 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_3D;
                dy_ext1 =
                    dy0 + 1 as ::core::ffi::c_int as ::core::ffi::c_double - SQUISH_CONSTANT_3D;
            } else {
                ysv_ext1 = ysb + 1 as ::core::ffi::c_int;
                ysv_ext0 = ysv_ext1;
                dy_ext0 = dy0
                    - 1 as ::core::ffi::c_int as ::core::ffi::c_double
                    - 2 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_3D;
                dy_ext1 =
                    dy0 - 1 as ::core::ffi::c_int as ::core::ffi::c_double - SQUISH_CONSTANT_3D;
            }
            if c_0 as ::core::ffi::c_int & 0x4 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                zsv_ext0 = zsb;
                zsv_ext1 = zsb - 1 as ::core::ffi::c_int;
                dz_ext0 =
                    dz0 - 2 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_3D;
                dz_ext1 =
                    dz0 + 1 as ::core::ffi::c_int as ::core::ffi::c_double - SQUISH_CONSTANT_3D;
            } else {
                zsv_ext1 = zsb + 1 as ::core::ffi::c_int;
                zsv_ext0 = zsv_ext1;
                dz_ext0 = dz0
                    - 1 as ::core::ffi::c_int as ::core::ffi::c_double
                    - 2 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_3D;
                dz_ext1 =
                    dz0 - 1 as ::core::ffi::c_int as ::core::ffi::c_double - SQUISH_CONSTANT_3D;
            }
        }
        let mut attn0: ::core::ffi::c_double =
            2 as ::core::ffi::c_int as ::core::ffi::c_double - dx0 * dx0 - dy0 * dy0 - dz0 * dz0;
        if attn0 > 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            attn0 *= attn0;
            value += attn0
                * attn0
                * extrapolate3(
                    ctx,
                    xsb + 0 as ::core::ffi::c_int,
                    ysb + 0 as ::core::ffi::c_int,
                    zsb + 0 as ::core::ffi::c_int,
                    dx0,
                    dy0,
                    dz0,
                );
        }
        let mut dx1: ::core::ffi::c_double =
            dx0 - 1 as ::core::ffi::c_int as ::core::ffi::c_double - SQUISH_CONSTANT_3D;
        let mut dy1: ::core::ffi::c_double =
            dy0 - 0 as ::core::ffi::c_int as ::core::ffi::c_double - SQUISH_CONSTANT_3D;
        let mut dz1: ::core::ffi::c_double =
            dz0 - 0 as ::core::ffi::c_int as ::core::ffi::c_double - SQUISH_CONSTANT_3D;
        let mut attn1: ::core::ffi::c_double =
            2 as ::core::ffi::c_int as ::core::ffi::c_double - dx1 * dx1 - dy1 * dy1 - dz1 * dz1;
        if attn1 > 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            attn1 *= attn1;
            value += attn1
                * attn1
                * extrapolate3(
                    ctx,
                    xsb + 1 as ::core::ffi::c_int,
                    ysb + 0 as ::core::ffi::c_int,
                    zsb + 0 as ::core::ffi::c_int,
                    dx1,
                    dy1,
                    dz1,
                );
        }
        let mut dx2: ::core::ffi::c_double =
            dx0 - 0 as ::core::ffi::c_int as ::core::ffi::c_double - SQUISH_CONSTANT_3D;
        let mut dy2: ::core::ffi::c_double =
            dy0 - 1 as ::core::ffi::c_int as ::core::ffi::c_double - SQUISH_CONSTANT_3D;
        let mut dz2: ::core::ffi::c_double = dz1;
        let mut attn2: ::core::ffi::c_double =
            2 as ::core::ffi::c_int as ::core::ffi::c_double - dx2 * dx2 - dy2 * dy2 - dz2 * dz2;
        if attn2 > 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            attn2 *= attn2;
            value += attn2
                * attn2
                * extrapolate3(
                    ctx,
                    xsb + 0 as ::core::ffi::c_int,
                    ysb + 1 as ::core::ffi::c_int,
                    zsb + 0 as ::core::ffi::c_int,
                    dx2,
                    dy2,
                    dz2,
                );
        }
        let mut dx3: ::core::ffi::c_double = dx2;
        let mut dy3: ::core::ffi::c_double = dy1;
        let mut dz3: ::core::ffi::c_double =
            dz0 - 1 as ::core::ffi::c_int as ::core::ffi::c_double - SQUISH_CONSTANT_3D;
        let mut attn3: ::core::ffi::c_double =
            2 as ::core::ffi::c_int as ::core::ffi::c_double - dx3 * dx3 - dy3 * dy3 - dz3 * dz3;
        if attn3 > 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            attn3 *= attn3;
            value += attn3
                * attn3
                * extrapolate3(
                    ctx,
                    xsb + 0 as ::core::ffi::c_int,
                    ysb + 0 as ::core::ffi::c_int,
                    zsb + 1 as ::core::ffi::c_int,
                    dx3,
                    dy3,
                    dz3,
                );
        }
    } else if inSum >= 2 as ::core::ffi::c_int as ::core::ffi::c_double {
        let mut aPoint_0: int8_t = 0x6 as int8_t;
        let mut aScore_0: ::core::ffi::c_double = xins;
        let mut bPoint_0: int8_t = 0x5 as int8_t;
        let mut bScore_0: ::core::ffi::c_double = yins;
        if aScore_0 <= bScore_0 && zins < bScore_0 {
            bScore_0 = zins;
            bPoint_0 = 0x3 as int8_t;
        } else if aScore_0 > bScore_0 && zins < aScore_0 {
            aScore_0 = zins;
            aPoint_0 = 0x3 as int8_t;
        }
        let mut wins_0: ::core::ffi::c_double =
            3 as ::core::ffi::c_int as ::core::ffi::c_double - inSum;
        if wins_0 < aScore_0 || wins_0 < bScore_0 {
            let mut c_1: int8_t = (if bScore_0 < aScore_0 {
                bPoint_0 as ::core::ffi::c_int
            } else {
                aPoint_0 as ::core::ffi::c_int
            }) as int8_t;
            if c_1 as ::core::ffi::c_int & 0x1 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                xsv_ext0 = xsb + 2 as ::core::ffi::c_int;
                xsv_ext1 = xsb + 1 as ::core::ffi::c_int;
                dx_ext0 = dx0
                    - 2 as ::core::ffi::c_int as ::core::ffi::c_double
                    - 3 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_3D;
                dx_ext1 = dx0
                    - 1 as ::core::ffi::c_int as ::core::ffi::c_double
                    - 3 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_3D;
            } else {
                xsv_ext1 = xsb;
                xsv_ext0 = xsv_ext1;
                dx_ext1 =
                    dx0 - 3 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_3D;
                dx_ext0 = dx_ext1;
            }
            if c_1 as ::core::ffi::c_int & 0x2 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                ysv_ext1 = ysb + 1 as ::core::ffi::c_int;
                ysv_ext0 = ysv_ext1;
                dy_ext1 = dy0
                    - 1 as ::core::ffi::c_int as ::core::ffi::c_double
                    - 3 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_3D;
                dy_ext0 = dy_ext1;
                if c_1 as ::core::ffi::c_int & 0x1 as ::core::ffi::c_int != 0 as ::core::ffi::c_int
                {
                    ysv_ext1 += 1 as ::core::ffi::c_int;
                    dy_ext1 -= 1 as ::core::ffi::c_int as ::core::ffi::c_double;
                } else {
                    ysv_ext0 += 1 as ::core::ffi::c_int;
                    dy_ext0 -= 1 as ::core::ffi::c_int as ::core::ffi::c_double;
                }
            } else {
                ysv_ext1 = ysb;
                ysv_ext0 = ysv_ext1;
                dy_ext1 =
                    dy0 - 3 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_3D;
                dy_ext0 = dy_ext1;
            }
            if c_1 as ::core::ffi::c_int & 0x4 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                zsv_ext0 = zsb + 1 as ::core::ffi::c_int;
                zsv_ext1 = zsb + 2 as ::core::ffi::c_int;
                dz_ext0 = dz0
                    - 1 as ::core::ffi::c_int as ::core::ffi::c_double
                    - 3 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_3D;
                dz_ext1 = dz0
                    - 2 as ::core::ffi::c_int as ::core::ffi::c_double
                    - 3 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_3D;
            } else {
                zsv_ext1 = zsb;
                zsv_ext0 = zsv_ext1;
                dz_ext1 =
                    dz0 - 3 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_3D;
                dz_ext0 = dz_ext1;
            }
        } else {
            let mut c_2: int8_t =
                (aPoint_0 as ::core::ffi::c_int & bPoint_0 as ::core::ffi::c_int) as int8_t;
            if c_2 as ::core::ffi::c_int & 0x1 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                xsv_ext0 = xsb + 1 as ::core::ffi::c_int;
                xsv_ext1 = xsb + 2 as ::core::ffi::c_int;
                dx_ext0 =
                    dx0 - 1 as ::core::ffi::c_int as ::core::ffi::c_double - SQUISH_CONSTANT_3D;
                dx_ext1 = dx0
                    - 2 as ::core::ffi::c_int as ::core::ffi::c_double
                    - 2 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_3D;
            } else {
                xsv_ext1 = xsb;
                xsv_ext0 = xsv_ext1;
                dx_ext0 = dx0 - SQUISH_CONSTANT_3D;
                dx_ext1 =
                    dx0 - 2 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_3D;
            }
            if c_2 as ::core::ffi::c_int & 0x2 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                ysv_ext0 = ysb + 1 as ::core::ffi::c_int;
                ysv_ext1 = ysb + 2 as ::core::ffi::c_int;
                dy_ext0 =
                    dy0 - 1 as ::core::ffi::c_int as ::core::ffi::c_double - SQUISH_CONSTANT_3D;
                dy_ext1 = dy0
                    - 2 as ::core::ffi::c_int as ::core::ffi::c_double
                    - 2 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_3D;
            } else {
                ysv_ext1 = ysb;
                ysv_ext0 = ysv_ext1;
                dy_ext0 = dy0 - SQUISH_CONSTANT_3D;
                dy_ext1 =
                    dy0 - 2 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_3D;
            }
            if c_2 as ::core::ffi::c_int & 0x4 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                zsv_ext0 = zsb + 1 as ::core::ffi::c_int;
                zsv_ext1 = zsb + 2 as ::core::ffi::c_int;
                dz_ext0 =
                    dz0 - 1 as ::core::ffi::c_int as ::core::ffi::c_double - SQUISH_CONSTANT_3D;
                dz_ext1 = dz0
                    - 2 as ::core::ffi::c_int as ::core::ffi::c_double
                    - 2 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_3D;
            } else {
                zsv_ext1 = zsb;
                zsv_ext0 = zsv_ext1;
                dz_ext0 = dz0 - SQUISH_CONSTANT_3D;
                dz_ext1 =
                    dz0 - 2 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_3D;
            }
        }
        let mut dx3_0: ::core::ffi::c_double = dx0
            - 1 as ::core::ffi::c_int as ::core::ffi::c_double
            - 2 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_3D;
        let mut dy3_0: ::core::ffi::c_double = dy0
            - 1 as ::core::ffi::c_int as ::core::ffi::c_double
            - 2 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_3D;
        let mut dz3_0: ::core::ffi::c_double = dz0
            - 0 as ::core::ffi::c_int as ::core::ffi::c_double
            - 2 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_3D;
        let mut attn3_0: ::core::ffi::c_double = 2 as ::core::ffi::c_int as ::core::ffi::c_double
            - dx3_0 * dx3_0
            - dy3_0 * dy3_0
            - dz3_0 * dz3_0;
        if attn3_0 > 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            attn3_0 *= attn3_0;
            value += attn3_0
                * attn3_0
                * extrapolate3(
                    ctx,
                    xsb + 1 as ::core::ffi::c_int,
                    ysb + 1 as ::core::ffi::c_int,
                    zsb + 0 as ::core::ffi::c_int,
                    dx3_0,
                    dy3_0,
                    dz3_0,
                );
        }
        let mut dx2_0: ::core::ffi::c_double = dx3_0;
        let mut dy2_0: ::core::ffi::c_double = dy0
            - 0 as ::core::ffi::c_int as ::core::ffi::c_double
            - 2 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_3D;
        let mut dz2_0: ::core::ffi::c_double = dz0
            - 1 as ::core::ffi::c_int as ::core::ffi::c_double
            - 2 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_3D;
        let mut attn2_0: ::core::ffi::c_double = 2 as ::core::ffi::c_int as ::core::ffi::c_double
            - dx2_0 * dx2_0
            - dy2_0 * dy2_0
            - dz2_0 * dz2_0;
        if attn2_0 > 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            attn2_0 *= attn2_0;
            value += attn2_0
                * attn2_0
                * extrapolate3(
                    ctx,
                    xsb + 1 as ::core::ffi::c_int,
                    ysb + 0 as ::core::ffi::c_int,
                    zsb + 1 as ::core::ffi::c_int,
                    dx2_0,
                    dy2_0,
                    dz2_0,
                );
        }
        let mut dx1_0: ::core::ffi::c_double = dx0
            - 0 as ::core::ffi::c_int as ::core::ffi::c_double
            - 2 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_3D;
        let mut dy1_0: ::core::ffi::c_double = dy3_0;
        let mut dz1_0: ::core::ffi::c_double = dz2_0;
        let mut attn1_0: ::core::ffi::c_double = 2 as ::core::ffi::c_int as ::core::ffi::c_double
            - dx1_0 * dx1_0
            - dy1_0 * dy1_0
            - dz1_0 * dz1_0;
        if attn1_0 > 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            attn1_0 *= attn1_0;
            value += attn1_0
                * attn1_0
                * extrapolate3(
                    ctx,
                    xsb + 0 as ::core::ffi::c_int,
                    ysb + 1 as ::core::ffi::c_int,
                    zsb + 1 as ::core::ffi::c_int,
                    dx1_0,
                    dy1_0,
                    dz1_0,
                );
        }
        dx0 = dx0
            - 1 as ::core::ffi::c_int as ::core::ffi::c_double
            - 3 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_3D;
        dy0 = dy0
            - 1 as ::core::ffi::c_int as ::core::ffi::c_double
            - 3 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_3D;
        dz0 = dz0
            - 1 as ::core::ffi::c_int as ::core::ffi::c_double
            - 3 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_3D;
        let mut attn0_0: ::core::ffi::c_double =
            2 as ::core::ffi::c_int as ::core::ffi::c_double - dx0 * dx0 - dy0 * dy0 - dz0 * dz0;
        if attn0_0 > 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            attn0_0 *= attn0_0;
            value += attn0_0
                * attn0_0
                * extrapolate3(
                    ctx,
                    xsb + 1 as ::core::ffi::c_int,
                    ysb + 1 as ::core::ffi::c_int,
                    zsb + 1 as ::core::ffi::c_int,
                    dx0,
                    dy0,
                    dz0,
                );
        }
    } else {
        let mut aScore_1: ::core::ffi::c_double = 0.;
        let mut aPoint_1: int8_t = 0;
        let mut aIsFurtherSide: ::core::ffi::c_int = 0;
        let mut bScore_1: ::core::ffi::c_double = 0.;
        let mut bPoint_1: int8_t = 0;
        let mut bIsFurtherSide: ::core::ffi::c_int = 0;
        let mut p1: ::core::ffi::c_double = xins + yins;
        if p1 > 1 as ::core::ffi::c_int as ::core::ffi::c_double {
            aScore_1 = p1 - 1 as ::core::ffi::c_int as ::core::ffi::c_double;
            aPoint_1 = 0x3 as int8_t;
            aIsFurtherSide = 1 as ::core::ffi::c_int;
        } else {
            aScore_1 = 1 as ::core::ffi::c_int as ::core::ffi::c_double - p1;
            aPoint_1 = 0x4 as int8_t;
            aIsFurtherSide = 0 as ::core::ffi::c_int;
        }
        let mut p2: ::core::ffi::c_double = xins + zins;
        if p2 > 1 as ::core::ffi::c_int as ::core::ffi::c_double {
            bScore_1 = p2 - 1 as ::core::ffi::c_int as ::core::ffi::c_double;
            bPoint_1 = 0x5 as int8_t;
            bIsFurtherSide = 1 as ::core::ffi::c_int;
        } else {
            bScore_1 = 1 as ::core::ffi::c_int as ::core::ffi::c_double - p2;
            bPoint_1 = 0x2 as int8_t;
            bIsFurtherSide = 0 as ::core::ffi::c_int;
        }
        let mut p3: ::core::ffi::c_double = yins + zins;
        if p3 > 1 as ::core::ffi::c_int as ::core::ffi::c_double {
            let mut score: ::core::ffi::c_double =
                p3 - 1 as ::core::ffi::c_int as ::core::ffi::c_double;
            if aScore_1 <= bScore_1 && aScore_1 < score {
                aScore_1 = score;
                aPoint_1 = 0x6 as int8_t;
                aIsFurtherSide = 1 as ::core::ffi::c_int;
            } else if aScore_1 > bScore_1 && bScore_1 < score {
                bScore_1 = score;
                bPoint_1 = 0x6 as int8_t;
                bIsFurtherSide = 1 as ::core::ffi::c_int;
            }
        } else {
            let mut score_0: ::core::ffi::c_double =
                1 as ::core::ffi::c_int as ::core::ffi::c_double - p3;
            if aScore_1 <= bScore_1 && aScore_1 < score_0 {
                aScore_1 = score_0;
                aPoint_1 = 0x1 as int8_t;
                aIsFurtherSide = 0 as ::core::ffi::c_int;
            } else if aScore_1 > bScore_1 && bScore_1 < score_0 {
                bScore_1 = score_0;
                bPoint_1 = 0x1 as int8_t;
                bIsFurtherSide = 0 as ::core::ffi::c_int;
            }
        }
        if aIsFurtherSide == bIsFurtherSide {
            if aIsFurtherSide != 0 {
                dx_ext0 = dx0
                    - 1 as ::core::ffi::c_int as ::core::ffi::c_double
                    - 3 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_3D;
                dy_ext0 = dy0
                    - 1 as ::core::ffi::c_int as ::core::ffi::c_double
                    - 3 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_3D;
                dz_ext0 = dz0
                    - 1 as ::core::ffi::c_int as ::core::ffi::c_double
                    - 3 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_3D;
                xsv_ext0 = xsb + 1 as ::core::ffi::c_int;
                ysv_ext0 = ysb + 1 as ::core::ffi::c_int;
                zsv_ext0 = zsb + 1 as ::core::ffi::c_int;
                let mut c_3: int8_t =
                    (aPoint_1 as ::core::ffi::c_int & bPoint_1 as ::core::ffi::c_int) as int8_t;
                if c_3 as ::core::ffi::c_int & 0x1 as ::core::ffi::c_int != 0 as ::core::ffi::c_int
                {
                    dx_ext1 = dx0
                        - 2 as ::core::ffi::c_int as ::core::ffi::c_double
                        - 2 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_3D;
                    dy_ext1 =
                        dy0 - 2 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_3D;
                    dz_ext1 =
                        dz0 - 2 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_3D;
                    xsv_ext1 = xsb + 2 as ::core::ffi::c_int;
                    ysv_ext1 = ysb;
                    zsv_ext1 = zsb;
                } else if c_3 as ::core::ffi::c_int & 0x2 as ::core::ffi::c_int
                    != 0 as ::core::ffi::c_int
                {
                    dx_ext1 =
                        dx0 - 2 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_3D;
                    dy_ext1 = dy0
                        - 2 as ::core::ffi::c_int as ::core::ffi::c_double
                        - 2 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_3D;
                    dz_ext1 =
                        dz0 - 2 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_3D;
                    xsv_ext1 = xsb;
                    ysv_ext1 = ysb + 2 as ::core::ffi::c_int;
                    zsv_ext1 = zsb;
                } else {
                    dx_ext1 =
                        dx0 - 2 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_3D;
                    dy_ext1 =
                        dy0 - 2 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_3D;
                    dz_ext1 = dz0
                        - 2 as ::core::ffi::c_int as ::core::ffi::c_double
                        - 2 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_3D;
                    xsv_ext1 = xsb;
                    ysv_ext1 = ysb;
                    zsv_ext1 = zsb + 2 as ::core::ffi::c_int;
                }
            } else {
                dx_ext0 = dx0;
                dy_ext0 = dy0;
                dz_ext0 = dz0;
                xsv_ext0 = xsb;
                ysv_ext0 = ysb;
                zsv_ext0 = zsb;
                let mut c_4: int8_t =
                    (aPoint_1 as ::core::ffi::c_int | bPoint_1 as ::core::ffi::c_int) as int8_t;
                if c_4 as ::core::ffi::c_int & 0x1 as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                {
                    dx_ext1 =
                        dx0 + 1 as ::core::ffi::c_int as ::core::ffi::c_double - SQUISH_CONSTANT_3D;
                    dy_ext1 =
                        dy0 - 1 as ::core::ffi::c_int as ::core::ffi::c_double - SQUISH_CONSTANT_3D;
                    dz_ext1 =
                        dz0 - 1 as ::core::ffi::c_int as ::core::ffi::c_double - SQUISH_CONSTANT_3D;
                    xsv_ext1 = xsb - 1 as ::core::ffi::c_int;
                    ysv_ext1 = ysb + 1 as ::core::ffi::c_int;
                    zsv_ext1 = zsb + 1 as ::core::ffi::c_int;
                } else if c_4 as ::core::ffi::c_int & 0x2 as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
                {
                    dx_ext1 =
                        dx0 - 1 as ::core::ffi::c_int as ::core::ffi::c_double - SQUISH_CONSTANT_3D;
                    dy_ext1 =
                        dy0 + 1 as ::core::ffi::c_int as ::core::ffi::c_double - SQUISH_CONSTANT_3D;
                    dz_ext1 =
                        dz0 - 1 as ::core::ffi::c_int as ::core::ffi::c_double - SQUISH_CONSTANT_3D;
                    xsv_ext1 = xsb + 1 as ::core::ffi::c_int;
                    ysv_ext1 = ysb - 1 as ::core::ffi::c_int;
                    zsv_ext1 = zsb + 1 as ::core::ffi::c_int;
                } else {
                    dx_ext1 =
                        dx0 - 1 as ::core::ffi::c_int as ::core::ffi::c_double - SQUISH_CONSTANT_3D;
                    dy_ext1 =
                        dy0 - 1 as ::core::ffi::c_int as ::core::ffi::c_double - SQUISH_CONSTANT_3D;
                    dz_ext1 =
                        dz0 + 1 as ::core::ffi::c_int as ::core::ffi::c_double - SQUISH_CONSTANT_3D;
                    xsv_ext1 = xsb + 1 as ::core::ffi::c_int;
                    ysv_ext1 = ysb + 1 as ::core::ffi::c_int;
                    zsv_ext1 = zsb - 1 as ::core::ffi::c_int;
                }
            }
        } else {
            let mut c1: int8_t = 0;
            let mut c2: int8_t = 0;
            if aIsFurtherSide != 0 {
                c1 = aPoint_1;
                c2 = bPoint_1;
            } else {
                c1 = bPoint_1;
                c2 = aPoint_1;
            }
            if c1 as ::core::ffi::c_int & 0x1 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                dx_ext0 =
                    dx0 + 1 as ::core::ffi::c_int as ::core::ffi::c_double - SQUISH_CONSTANT_3D;
                dy_ext0 =
                    dy0 - 1 as ::core::ffi::c_int as ::core::ffi::c_double - SQUISH_CONSTANT_3D;
                dz_ext0 =
                    dz0 - 1 as ::core::ffi::c_int as ::core::ffi::c_double - SQUISH_CONSTANT_3D;
                xsv_ext0 = xsb - 1 as ::core::ffi::c_int;
                ysv_ext0 = ysb + 1 as ::core::ffi::c_int;
                zsv_ext0 = zsb + 1 as ::core::ffi::c_int;
            } else if c1 as ::core::ffi::c_int & 0x2 as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
            {
                dx_ext0 =
                    dx0 - 1 as ::core::ffi::c_int as ::core::ffi::c_double - SQUISH_CONSTANT_3D;
                dy_ext0 =
                    dy0 + 1 as ::core::ffi::c_int as ::core::ffi::c_double - SQUISH_CONSTANT_3D;
                dz_ext0 =
                    dz0 - 1 as ::core::ffi::c_int as ::core::ffi::c_double - SQUISH_CONSTANT_3D;
                xsv_ext0 = xsb + 1 as ::core::ffi::c_int;
                ysv_ext0 = ysb - 1 as ::core::ffi::c_int;
                zsv_ext0 = zsb + 1 as ::core::ffi::c_int;
            } else {
                dx_ext0 =
                    dx0 - 1 as ::core::ffi::c_int as ::core::ffi::c_double - SQUISH_CONSTANT_3D;
                dy_ext0 =
                    dy0 - 1 as ::core::ffi::c_int as ::core::ffi::c_double - SQUISH_CONSTANT_3D;
                dz_ext0 =
                    dz0 + 1 as ::core::ffi::c_int as ::core::ffi::c_double - SQUISH_CONSTANT_3D;
                xsv_ext0 = xsb + 1 as ::core::ffi::c_int;
                ysv_ext0 = ysb + 1 as ::core::ffi::c_int;
                zsv_ext0 = zsb - 1 as ::core::ffi::c_int;
            }
            dx_ext1 = dx0 - 2 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_3D;
            dy_ext1 = dy0 - 2 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_3D;
            dz_ext1 = dz0 - 2 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_3D;
            xsv_ext1 = xsb;
            ysv_ext1 = ysb;
            zsv_ext1 = zsb;
            if c2 as ::core::ffi::c_int & 0x1 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                dx_ext1 -= 2 as ::core::ffi::c_int as ::core::ffi::c_double;
                xsv_ext1 += 2 as ::core::ffi::c_int;
            } else if c2 as ::core::ffi::c_int & 0x2 as ::core::ffi::c_int
                != 0 as ::core::ffi::c_int
            {
                dy_ext1 -= 2 as ::core::ffi::c_int as ::core::ffi::c_double;
                ysv_ext1 += 2 as ::core::ffi::c_int;
            } else {
                dz_ext1 -= 2 as ::core::ffi::c_int as ::core::ffi::c_double;
                zsv_ext1 += 2 as ::core::ffi::c_int;
            }
        }
        let mut dx1_1: ::core::ffi::c_double =
            dx0 - 1 as ::core::ffi::c_int as ::core::ffi::c_double - SQUISH_CONSTANT_3D;
        let mut dy1_1: ::core::ffi::c_double =
            dy0 - 0 as ::core::ffi::c_int as ::core::ffi::c_double - SQUISH_CONSTANT_3D;
        let mut dz1_1: ::core::ffi::c_double =
            dz0 - 0 as ::core::ffi::c_int as ::core::ffi::c_double - SQUISH_CONSTANT_3D;
        let mut attn1_1: ::core::ffi::c_double = 2 as ::core::ffi::c_int as ::core::ffi::c_double
            - dx1_1 * dx1_1
            - dy1_1 * dy1_1
            - dz1_1 * dz1_1;
        if attn1_1 > 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            attn1_1 *= attn1_1;
            value += attn1_1
                * attn1_1
                * extrapolate3(
                    ctx,
                    xsb + 1 as ::core::ffi::c_int,
                    ysb + 0 as ::core::ffi::c_int,
                    zsb + 0 as ::core::ffi::c_int,
                    dx1_1,
                    dy1_1,
                    dz1_1,
                );
        }
        let mut dx2_1: ::core::ffi::c_double =
            dx0 - 0 as ::core::ffi::c_int as ::core::ffi::c_double - SQUISH_CONSTANT_3D;
        let mut dy2_1: ::core::ffi::c_double =
            dy0 - 1 as ::core::ffi::c_int as ::core::ffi::c_double - SQUISH_CONSTANT_3D;
        let mut dz2_1: ::core::ffi::c_double = dz1_1;
        let mut attn2_1: ::core::ffi::c_double = 2 as ::core::ffi::c_int as ::core::ffi::c_double
            - dx2_1 * dx2_1
            - dy2_1 * dy2_1
            - dz2_1 * dz2_1;
        if attn2_1 > 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            attn2_1 *= attn2_1;
            value += attn2_1
                * attn2_1
                * extrapolate3(
                    ctx,
                    xsb + 0 as ::core::ffi::c_int,
                    ysb + 1 as ::core::ffi::c_int,
                    zsb + 0 as ::core::ffi::c_int,
                    dx2_1,
                    dy2_1,
                    dz2_1,
                );
        }
        let mut dx3_1: ::core::ffi::c_double = dx2_1;
        let mut dy3_1: ::core::ffi::c_double = dy1_1;
        let mut dz3_1: ::core::ffi::c_double =
            dz0 - 1 as ::core::ffi::c_int as ::core::ffi::c_double - SQUISH_CONSTANT_3D;
        let mut attn3_1: ::core::ffi::c_double = 2 as ::core::ffi::c_int as ::core::ffi::c_double
            - dx3_1 * dx3_1
            - dy3_1 * dy3_1
            - dz3_1 * dz3_1;
        if attn3_1 > 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            attn3_1 *= attn3_1;
            value += attn3_1
                * attn3_1
                * extrapolate3(
                    ctx,
                    xsb + 0 as ::core::ffi::c_int,
                    ysb + 0 as ::core::ffi::c_int,
                    zsb + 1 as ::core::ffi::c_int,
                    dx3_1,
                    dy3_1,
                    dz3_1,
                );
        }
        let mut dx4: ::core::ffi::c_double = dx0
            - 1 as ::core::ffi::c_int as ::core::ffi::c_double
            - 2 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_3D;
        let mut dy4: ::core::ffi::c_double = dy0
            - 1 as ::core::ffi::c_int as ::core::ffi::c_double
            - 2 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_3D;
        let mut dz4: ::core::ffi::c_double = dz0
            - 0 as ::core::ffi::c_int as ::core::ffi::c_double
            - 2 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_3D;
        let mut attn4: ::core::ffi::c_double =
            2 as ::core::ffi::c_int as ::core::ffi::c_double - dx4 * dx4 - dy4 * dy4 - dz4 * dz4;
        if attn4 > 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            attn4 *= attn4;
            value += attn4
                * attn4
                * extrapolate3(
                    ctx,
                    xsb + 1 as ::core::ffi::c_int,
                    ysb + 1 as ::core::ffi::c_int,
                    zsb + 0 as ::core::ffi::c_int,
                    dx4,
                    dy4,
                    dz4,
                );
        }
        let mut dx5: ::core::ffi::c_double = dx4;
        let mut dy5: ::core::ffi::c_double = dy0
            - 0 as ::core::ffi::c_int as ::core::ffi::c_double
            - 2 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_3D;
        let mut dz5: ::core::ffi::c_double = dz0
            - 1 as ::core::ffi::c_int as ::core::ffi::c_double
            - 2 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_3D;
        let mut attn5: ::core::ffi::c_double =
            2 as ::core::ffi::c_int as ::core::ffi::c_double - dx5 * dx5 - dy5 * dy5 - dz5 * dz5;
        if attn5 > 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            attn5 *= attn5;
            value += attn5
                * attn5
                * extrapolate3(
                    ctx,
                    xsb + 1 as ::core::ffi::c_int,
                    ysb + 0 as ::core::ffi::c_int,
                    zsb + 1 as ::core::ffi::c_int,
                    dx5,
                    dy5,
                    dz5,
                );
        }
        let mut dx6: ::core::ffi::c_double = dx0
            - 0 as ::core::ffi::c_int as ::core::ffi::c_double
            - 2 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_3D;
        let mut dy6: ::core::ffi::c_double = dy4;
        let mut dz6: ::core::ffi::c_double = dz5;
        let mut attn6: ::core::ffi::c_double =
            2 as ::core::ffi::c_int as ::core::ffi::c_double - dx6 * dx6 - dy6 * dy6 - dz6 * dz6;
        if attn6 > 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            attn6 *= attn6;
            value += attn6
                * attn6
                * extrapolate3(
                    ctx,
                    xsb + 0 as ::core::ffi::c_int,
                    ysb + 1 as ::core::ffi::c_int,
                    zsb + 1 as ::core::ffi::c_int,
                    dx6,
                    dy6,
                    dz6,
                );
        }
    }
    let mut attn_ext0: ::core::ffi::c_double = 2 as ::core::ffi::c_int as ::core::ffi::c_double
        - dx_ext0 * dx_ext0
        - dy_ext0 * dy_ext0
        - dz_ext0 * dz_ext0;
    if attn_ext0 > 0 as ::core::ffi::c_int as ::core::ffi::c_double {
        attn_ext0 *= attn_ext0;
        value += attn_ext0
            * attn_ext0
            * extrapolate3(ctx, xsv_ext0, ysv_ext0, zsv_ext0, dx_ext0, dy_ext0, dz_ext0);
    }
    let mut attn_ext1: ::core::ffi::c_double = 2 as ::core::ffi::c_int as ::core::ffi::c_double
        - dx_ext1 * dx_ext1
        - dy_ext1 * dy_ext1
        - dz_ext1 * dz_ext1;
    if attn_ext1 > 0 as ::core::ffi::c_int as ::core::ffi::c_double {
        attn_ext1 *= attn_ext1;
        value += attn_ext1
            * attn_ext1
            * extrapolate3(ctx, xsv_ext1, ysv_ext1, zsv_ext1, dx_ext1, dy_ext1, dz_ext1);
    }
    return value / NORM_CONSTANT_3D;
}
#[no_mangle]
pub unsafe extern "C" fn open_simplex_noise4(
    mut ctx: *mut osn_context,
    mut x: ::core::ffi::c_double,
    mut y: ::core::ffi::c_double,
    mut z: ::core::ffi::c_double,
    mut w: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut stretchOffset: ::core::ffi::c_double = (x + y + z + w) * STRETCH_CONSTANT_4D;
    let mut xs: ::core::ffi::c_double = x + stretchOffset;
    let mut ys: ::core::ffi::c_double = y + stretchOffset;
    let mut zs: ::core::ffi::c_double = z + stretchOffset;
    let mut ws: ::core::ffi::c_double = w + stretchOffset;
    let mut xsb: ::core::ffi::c_int = fastFloor(xs);
    let mut ysb: ::core::ffi::c_int = fastFloor(ys);
    let mut zsb: ::core::ffi::c_int = fastFloor(zs);
    let mut wsb: ::core::ffi::c_int = fastFloor(ws);
    let mut squishOffset: ::core::ffi::c_double =
        (xsb + ysb + zsb + wsb) as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
    let mut xb: ::core::ffi::c_double = xsb as ::core::ffi::c_double + squishOffset;
    let mut yb: ::core::ffi::c_double = ysb as ::core::ffi::c_double + squishOffset;
    let mut zb: ::core::ffi::c_double = zsb as ::core::ffi::c_double + squishOffset;
    let mut wb: ::core::ffi::c_double = wsb as ::core::ffi::c_double + squishOffset;
    let mut xins: ::core::ffi::c_double = xs - xsb as ::core::ffi::c_double;
    let mut yins: ::core::ffi::c_double = ys - ysb as ::core::ffi::c_double;
    let mut zins: ::core::ffi::c_double = zs - zsb as ::core::ffi::c_double;
    let mut wins: ::core::ffi::c_double = ws - wsb as ::core::ffi::c_double;
    let mut inSum: ::core::ffi::c_double = xins + yins + zins + wins;
    let mut dx0: ::core::ffi::c_double = x - xb;
    let mut dy0: ::core::ffi::c_double = y - yb;
    let mut dz0: ::core::ffi::c_double = z - zb;
    let mut dw0: ::core::ffi::c_double = w - wb;
    let mut dx_ext0: ::core::ffi::c_double = 0.;
    let mut dy_ext0: ::core::ffi::c_double = 0.;
    let mut dz_ext0: ::core::ffi::c_double = 0.;
    let mut dw_ext0: ::core::ffi::c_double = 0.;
    let mut dx_ext1: ::core::ffi::c_double = 0.;
    let mut dy_ext1: ::core::ffi::c_double = 0.;
    let mut dz_ext1: ::core::ffi::c_double = 0.;
    let mut dw_ext1: ::core::ffi::c_double = 0.;
    let mut dx_ext2: ::core::ffi::c_double = 0.;
    let mut dy_ext2: ::core::ffi::c_double = 0.;
    let mut dz_ext2: ::core::ffi::c_double = 0.;
    let mut dw_ext2: ::core::ffi::c_double = 0.;
    let mut xsv_ext0: ::core::ffi::c_int = 0;
    let mut ysv_ext0: ::core::ffi::c_int = 0;
    let mut zsv_ext0: ::core::ffi::c_int = 0;
    let mut wsv_ext0: ::core::ffi::c_int = 0;
    let mut xsv_ext1: ::core::ffi::c_int = 0;
    let mut ysv_ext1: ::core::ffi::c_int = 0;
    let mut zsv_ext1: ::core::ffi::c_int = 0;
    let mut wsv_ext1: ::core::ffi::c_int = 0;
    let mut xsv_ext2: ::core::ffi::c_int = 0;
    let mut ysv_ext2: ::core::ffi::c_int = 0;
    let mut zsv_ext2: ::core::ffi::c_int = 0;
    let mut wsv_ext2: ::core::ffi::c_int = 0;
    let mut value: ::core::ffi::c_double = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    if inSum <= 1 as ::core::ffi::c_int as ::core::ffi::c_double {
        let mut aPoint: int8_t = 0x1 as int8_t;
        let mut aScore: ::core::ffi::c_double = xins;
        let mut bPoint: int8_t = 0x2 as int8_t;
        let mut bScore: ::core::ffi::c_double = yins;
        if aScore >= bScore && zins > bScore {
            bScore = zins;
            bPoint = 0x4 as int8_t;
        } else if aScore < bScore && zins > aScore {
            aScore = zins;
            aPoint = 0x4 as int8_t;
        }
        if aScore >= bScore && wins > bScore {
            bScore = wins;
            bPoint = 0x8 as int8_t;
        } else if aScore < bScore && wins > aScore {
            aScore = wins;
            aPoint = 0x8 as int8_t;
        }
        let mut uins: ::core::ffi::c_double =
            1 as ::core::ffi::c_int as ::core::ffi::c_double - inSum;
        if uins > aScore || uins > bScore {
            let mut c: int8_t = (if bScore > aScore {
                bPoint as ::core::ffi::c_int
            } else {
                aPoint as ::core::ffi::c_int
            }) as int8_t;
            if c as ::core::ffi::c_int & 0x1 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                xsv_ext0 = xsb - 1 as ::core::ffi::c_int;
                xsv_ext2 = xsb;
                xsv_ext1 = xsv_ext2;
                dx_ext0 = dx0 + 1 as ::core::ffi::c_int as ::core::ffi::c_double;
                dx_ext2 = dx0;
                dx_ext1 = dx_ext2;
            } else {
                xsv_ext2 = xsb + 1 as ::core::ffi::c_int;
                xsv_ext1 = xsv_ext2;
                xsv_ext0 = xsv_ext1;
                dx_ext2 = dx0 - 1 as ::core::ffi::c_int as ::core::ffi::c_double;
                dx_ext1 = dx_ext2;
                dx_ext0 = dx_ext1;
            }
            if c as ::core::ffi::c_int & 0x2 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                ysv_ext2 = ysb;
                ysv_ext1 = ysv_ext2;
                ysv_ext0 = ysv_ext1;
                dy_ext2 = dy0;
                dy_ext1 = dy_ext2;
                dy_ext0 = dy_ext1;
                if c as ::core::ffi::c_int & 0x1 as ::core::ffi::c_int == 0x1 as ::core::ffi::c_int
                {
                    ysv_ext0 -= 1 as ::core::ffi::c_int;
                    dy_ext0 += 1 as ::core::ffi::c_int as ::core::ffi::c_double;
                } else {
                    ysv_ext1 -= 1 as ::core::ffi::c_int;
                    dy_ext1 += 1 as ::core::ffi::c_int as ::core::ffi::c_double;
                }
            } else {
                ysv_ext2 = ysb + 1 as ::core::ffi::c_int;
                ysv_ext1 = ysv_ext2;
                ysv_ext0 = ysv_ext1;
                dy_ext2 = dy0 - 1 as ::core::ffi::c_int as ::core::ffi::c_double;
                dy_ext1 = dy_ext2;
                dy_ext0 = dy_ext1;
            }
            if c as ::core::ffi::c_int & 0x4 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                zsv_ext2 = zsb;
                zsv_ext1 = zsv_ext2;
                zsv_ext0 = zsv_ext1;
                dz_ext2 = dz0;
                dz_ext1 = dz_ext2;
                dz_ext0 = dz_ext1;
                if c as ::core::ffi::c_int & 0x3 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                    if c as ::core::ffi::c_int & 0x3 as ::core::ffi::c_int
                        == 0x3 as ::core::ffi::c_int
                    {
                        zsv_ext0 -= 1 as ::core::ffi::c_int;
                        dz_ext0 += 1 as ::core::ffi::c_int as ::core::ffi::c_double;
                    } else {
                        zsv_ext1 -= 1 as ::core::ffi::c_int;
                        dz_ext1 += 1 as ::core::ffi::c_int as ::core::ffi::c_double;
                    }
                } else {
                    zsv_ext2 -= 1 as ::core::ffi::c_int;
                    dz_ext2 += 1 as ::core::ffi::c_int as ::core::ffi::c_double;
                }
            } else {
                zsv_ext2 = zsb + 1 as ::core::ffi::c_int;
                zsv_ext1 = zsv_ext2;
                zsv_ext0 = zsv_ext1;
                dz_ext2 = dz0 - 1 as ::core::ffi::c_int as ::core::ffi::c_double;
                dz_ext1 = dz_ext2;
                dz_ext0 = dz_ext1;
            }
            if c as ::core::ffi::c_int & 0x8 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                wsv_ext1 = wsb;
                wsv_ext0 = wsv_ext1;
                wsv_ext2 = wsb - 1 as ::core::ffi::c_int;
                dw_ext1 = dw0;
                dw_ext0 = dw_ext1;
                dw_ext2 = dw0 + 1 as ::core::ffi::c_int as ::core::ffi::c_double;
            } else {
                wsv_ext2 = wsb + 1 as ::core::ffi::c_int;
                wsv_ext1 = wsv_ext2;
                wsv_ext0 = wsv_ext1;
                dw_ext2 = dw0 - 1 as ::core::ffi::c_int as ::core::ffi::c_double;
                dw_ext1 = dw_ext2;
                dw_ext0 = dw_ext1;
            }
        } else {
            let mut c_0: int8_t =
                (aPoint as ::core::ffi::c_int | bPoint as ::core::ffi::c_int) as int8_t;
            if c_0 as ::core::ffi::c_int & 0x1 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                xsv_ext2 = xsb;
                xsv_ext0 = xsv_ext2;
                xsv_ext1 = xsb - 1 as ::core::ffi::c_int;
                dx_ext0 =
                    dx0 - 2 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
                dx_ext1 =
                    dx0 + 1 as ::core::ffi::c_int as ::core::ffi::c_double - SQUISH_CONSTANT_4D;
                dx_ext2 = dx0 - SQUISH_CONSTANT_4D;
            } else {
                xsv_ext2 = xsb + 1 as ::core::ffi::c_int;
                xsv_ext1 = xsv_ext2;
                xsv_ext0 = xsv_ext1;
                dx_ext0 = dx0
                    - 1 as ::core::ffi::c_int as ::core::ffi::c_double
                    - 2 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
                dx_ext2 =
                    dx0 - 1 as ::core::ffi::c_int as ::core::ffi::c_double - SQUISH_CONSTANT_4D;
                dx_ext1 = dx_ext2;
            }
            if c_0 as ::core::ffi::c_int & 0x2 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                ysv_ext2 = ysb;
                ysv_ext1 = ysv_ext2;
                ysv_ext0 = ysv_ext1;
                dy_ext0 =
                    dy0 - 2 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
                dy_ext2 = dy0 - SQUISH_CONSTANT_4D;
                dy_ext1 = dy_ext2;
                if c_0 as ::core::ffi::c_int & 0x1 as ::core::ffi::c_int
                    == 0x1 as ::core::ffi::c_int
                {
                    ysv_ext1 -= 1 as ::core::ffi::c_int;
                    dy_ext1 += 1 as ::core::ffi::c_int as ::core::ffi::c_double;
                } else {
                    ysv_ext2 -= 1 as ::core::ffi::c_int;
                    dy_ext2 += 1 as ::core::ffi::c_int as ::core::ffi::c_double;
                }
            } else {
                ysv_ext2 = ysb + 1 as ::core::ffi::c_int;
                ysv_ext1 = ysv_ext2;
                ysv_ext0 = ysv_ext1;
                dy_ext0 = dy0
                    - 1 as ::core::ffi::c_int as ::core::ffi::c_double
                    - 2 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
                dy_ext2 =
                    dy0 - 1 as ::core::ffi::c_int as ::core::ffi::c_double - SQUISH_CONSTANT_4D;
                dy_ext1 = dy_ext2;
            }
            if c_0 as ::core::ffi::c_int & 0x4 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                zsv_ext2 = zsb;
                zsv_ext1 = zsv_ext2;
                zsv_ext0 = zsv_ext1;
                dz_ext0 =
                    dz0 - 2 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
                dz_ext2 = dz0 - SQUISH_CONSTANT_4D;
                dz_ext1 = dz_ext2;
                if c_0 as ::core::ffi::c_int & 0x3 as ::core::ffi::c_int
                    == 0x3 as ::core::ffi::c_int
                {
                    zsv_ext1 -= 1 as ::core::ffi::c_int;
                    dz_ext1 += 1 as ::core::ffi::c_int as ::core::ffi::c_double;
                } else {
                    zsv_ext2 -= 1 as ::core::ffi::c_int;
                    dz_ext2 += 1 as ::core::ffi::c_int as ::core::ffi::c_double;
                }
            } else {
                zsv_ext2 = zsb + 1 as ::core::ffi::c_int;
                zsv_ext1 = zsv_ext2;
                zsv_ext0 = zsv_ext1;
                dz_ext0 = dz0
                    - 1 as ::core::ffi::c_int as ::core::ffi::c_double
                    - 2 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
                dz_ext2 =
                    dz0 - 1 as ::core::ffi::c_int as ::core::ffi::c_double - SQUISH_CONSTANT_4D;
                dz_ext1 = dz_ext2;
            }
            if c_0 as ::core::ffi::c_int & 0x8 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                wsv_ext1 = wsb;
                wsv_ext0 = wsv_ext1;
                wsv_ext2 = wsb - 1 as ::core::ffi::c_int;
                dw_ext0 =
                    dw0 - 2 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
                dw_ext1 = dw0 - SQUISH_CONSTANT_4D;
                dw_ext2 =
                    dw0 + 1 as ::core::ffi::c_int as ::core::ffi::c_double - SQUISH_CONSTANT_4D;
            } else {
                wsv_ext2 = wsb + 1 as ::core::ffi::c_int;
                wsv_ext1 = wsv_ext2;
                wsv_ext0 = wsv_ext1;
                dw_ext0 = dw0
                    - 1 as ::core::ffi::c_int as ::core::ffi::c_double
                    - 2 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
                dw_ext2 =
                    dw0 - 1 as ::core::ffi::c_int as ::core::ffi::c_double - SQUISH_CONSTANT_4D;
                dw_ext1 = dw_ext2;
            }
        }
        let mut attn0: ::core::ffi::c_double = 2 as ::core::ffi::c_int as ::core::ffi::c_double
            - dx0 * dx0
            - dy0 * dy0
            - dz0 * dz0
            - dw0 * dw0;
        if attn0 > 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            attn0 *= attn0;
            value += attn0
                * attn0
                * extrapolate4(
                    ctx,
                    xsb + 0 as ::core::ffi::c_int,
                    ysb + 0 as ::core::ffi::c_int,
                    zsb + 0 as ::core::ffi::c_int,
                    wsb + 0 as ::core::ffi::c_int,
                    dx0,
                    dy0,
                    dz0,
                    dw0,
                );
        }
        let mut dx1: ::core::ffi::c_double =
            dx0 - 1 as ::core::ffi::c_int as ::core::ffi::c_double - SQUISH_CONSTANT_4D;
        let mut dy1: ::core::ffi::c_double =
            dy0 - 0 as ::core::ffi::c_int as ::core::ffi::c_double - SQUISH_CONSTANT_4D;
        let mut dz1: ::core::ffi::c_double =
            dz0 - 0 as ::core::ffi::c_int as ::core::ffi::c_double - SQUISH_CONSTANT_4D;
        let mut dw1: ::core::ffi::c_double =
            dw0 - 0 as ::core::ffi::c_int as ::core::ffi::c_double - SQUISH_CONSTANT_4D;
        let mut attn1: ::core::ffi::c_double = 2 as ::core::ffi::c_int as ::core::ffi::c_double
            - dx1 * dx1
            - dy1 * dy1
            - dz1 * dz1
            - dw1 * dw1;
        if attn1 > 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            attn1 *= attn1;
            value += attn1
                * attn1
                * extrapolate4(
                    ctx,
                    xsb + 1 as ::core::ffi::c_int,
                    ysb + 0 as ::core::ffi::c_int,
                    zsb + 0 as ::core::ffi::c_int,
                    wsb + 0 as ::core::ffi::c_int,
                    dx1,
                    dy1,
                    dz1,
                    dw1,
                );
        }
        let mut dx2: ::core::ffi::c_double =
            dx0 - 0 as ::core::ffi::c_int as ::core::ffi::c_double - SQUISH_CONSTANT_4D;
        let mut dy2: ::core::ffi::c_double =
            dy0 - 1 as ::core::ffi::c_int as ::core::ffi::c_double - SQUISH_CONSTANT_4D;
        let mut dz2: ::core::ffi::c_double = dz1;
        let mut dw2: ::core::ffi::c_double = dw1;
        let mut attn2: ::core::ffi::c_double = 2 as ::core::ffi::c_int as ::core::ffi::c_double
            - dx2 * dx2
            - dy2 * dy2
            - dz2 * dz2
            - dw2 * dw2;
        if attn2 > 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            attn2 *= attn2;
            value += attn2
                * attn2
                * extrapolate4(
                    ctx,
                    xsb + 0 as ::core::ffi::c_int,
                    ysb + 1 as ::core::ffi::c_int,
                    zsb + 0 as ::core::ffi::c_int,
                    wsb + 0 as ::core::ffi::c_int,
                    dx2,
                    dy2,
                    dz2,
                    dw2,
                );
        }
        let mut dx3: ::core::ffi::c_double = dx2;
        let mut dy3: ::core::ffi::c_double = dy1;
        let mut dz3: ::core::ffi::c_double =
            dz0 - 1 as ::core::ffi::c_int as ::core::ffi::c_double - SQUISH_CONSTANT_4D;
        let mut dw3: ::core::ffi::c_double = dw1;
        let mut attn3: ::core::ffi::c_double = 2 as ::core::ffi::c_int as ::core::ffi::c_double
            - dx3 * dx3
            - dy3 * dy3
            - dz3 * dz3
            - dw3 * dw3;
        if attn3 > 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            attn3 *= attn3;
            value += attn3
                * attn3
                * extrapolate4(
                    ctx,
                    xsb + 0 as ::core::ffi::c_int,
                    ysb + 0 as ::core::ffi::c_int,
                    zsb + 1 as ::core::ffi::c_int,
                    wsb + 0 as ::core::ffi::c_int,
                    dx3,
                    dy3,
                    dz3,
                    dw3,
                );
        }
        let mut dx4: ::core::ffi::c_double = dx2;
        let mut dy4: ::core::ffi::c_double = dy1;
        let mut dz4: ::core::ffi::c_double = dz1;
        let mut dw4: ::core::ffi::c_double =
            dw0 - 1 as ::core::ffi::c_int as ::core::ffi::c_double - SQUISH_CONSTANT_4D;
        let mut attn4: ::core::ffi::c_double = 2 as ::core::ffi::c_int as ::core::ffi::c_double
            - dx4 * dx4
            - dy4 * dy4
            - dz4 * dz4
            - dw4 * dw4;
        if attn4 > 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            attn4 *= attn4;
            value += attn4
                * attn4
                * extrapolate4(
                    ctx,
                    xsb + 0 as ::core::ffi::c_int,
                    ysb + 0 as ::core::ffi::c_int,
                    zsb + 0 as ::core::ffi::c_int,
                    wsb + 1 as ::core::ffi::c_int,
                    dx4,
                    dy4,
                    dz4,
                    dw4,
                );
        }
    } else if inSum >= 3 as ::core::ffi::c_int as ::core::ffi::c_double {
        let mut aPoint_0: int8_t = 0xe as int8_t;
        let mut aScore_0: ::core::ffi::c_double = xins;
        let mut bPoint_0: int8_t = 0xd as int8_t;
        let mut bScore_0: ::core::ffi::c_double = yins;
        if aScore_0 <= bScore_0 && zins < bScore_0 {
            bScore_0 = zins;
            bPoint_0 = 0xb as int8_t;
        } else if aScore_0 > bScore_0 && zins < aScore_0 {
            aScore_0 = zins;
            aPoint_0 = 0xb as int8_t;
        }
        if aScore_0 <= bScore_0 && wins < bScore_0 {
            bScore_0 = wins;
            bPoint_0 = 0x7 as int8_t;
        } else if aScore_0 > bScore_0 && wins < aScore_0 {
            aScore_0 = wins;
            aPoint_0 = 0x7 as int8_t;
        }
        let mut uins_0: ::core::ffi::c_double =
            4 as ::core::ffi::c_int as ::core::ffi::c_double - inSum;
        if uins_0 < aScore_0 || uins_0 < bScore_0 {
            let mut c_1: int8_t = (if bScore_0 < aScore_0 {
                bPoint_0 as ::core::ffi::c_int
            } else {
                aPoint_0 as ::core::ffi::c_int
            }) as int8_t;
            if c_1 as ::core::ffi::c_int & 0x1 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                xsv_ext0 = xsb + 2 as ::core::ffi::c_int;
                xsv_ext2 = xsb + 1 as ::core::ffi::c_int;
                xsv_ext1 = xsv_ext2;
                dx_ext0 = dx0
                    - 2 as ::core::ffi::c_int as ::core::ffi::c_double
                    - 4 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
                dx_ext2 = dx0
                    - 1 as ::core::ffi::c_int as ::core::ffi::c_double
                    - 4 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
                dx_ext1 = dx_ext2;
            } else {
                xsv_ext2 = xsb;
                xsv_ext1 = xsv_ext2;
                xsv_ext0 = xsv_ext1;
                dx_ext2 =
                    dx0 - 4 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
                dx_ext1 = dx_ext2;
                dx_ext0 = dx_ext1;
            }
            if c_1 as ::core::ffi::c_int & 0x2 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                ysv_ext2 = ysb + 1 as ::core::ffi::c_int;
                ysv_ext1 = ysv_ext2;
                ysv_ext0 = ysv_ext1;
                dy_ext2 = dy0
                    - 1 as ::core::ffi::c_int as ::core::ffi::c_double
                    - 4 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
                dy_ext1 = dy_ext2;
                dy_ext0 = dy_ext1;
                if c_1 as ::core::ffi::c_int & 0x1 as ::core::ffi::c_int != 0 as ::core::ffi::c_int
                {
                    ysv_ext1 += 1 as ::core::ffi::c_int;
                    dy_ext1 -= 1 as ::core::ffi::c_int as ::core::ffi::c_double;
                } else {
                    ysv_ext0 += 1 as ::core::ffi::c_int;
                    dy_ext0 -= 1 as ::core::ffi::c_int as ::core::ffi::c_double;
                }
            } else {
                ysv_ext2 = ysb;
                ysv_ext1 = ysv_ext2;
                ysv_ext0 = ysv_ext1;
                dy_ext2 =
                    dy0 - 4 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
                dy_ext1 = dy_ext2;
                dy_ext0 = dy_ext1;
            }
            if c_1 as ::core::ffi::c_int & 0x4 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                zsv_ext2 = zsb + 1 as ::core::ffi::c_int;
                zsv_ext1 = zsv_ext2;
                zsv_ext0 = zsv_ext1;
                dz_ext2 = dz0
                    - 1 as ::core::ffi::c_int as ::core::ffi::c_double
                    - 4 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
                dz_ext1 = dz_ext2;
                dz_ext0 = dz_ext1;
                if c_1 as ::core::ffi::c_int & 0x3 as ::core::ffi::c_int
                    != 0x3 as ::core::ffi::c_int
                {
                    if c_1 as ::core::ffi::c_int & 0x3 as ::core::ffi::c_int
                        == 0 as ::core::ffi::c_int
                    {
                        zsv_ext0 += 1 as ::core::ffi::c_int;
                        dz_ext0 -= 1 as ::core::ffi::c_int as ::core::ffi::c_double;
                    } else {
                        zsv_ext1 += 1 as ::core::ffi::c_int;
                        dz_ext1 -= 1 as ::core::ffi::c_int as ::core::ffi::c_double;
                    }
                } else {
                    zsv_ext2 += 1 as ::core::ffi::c_int;
                    dz_ext2 -= 1 as ::core::ffi::c_int as ::core::ffi::c_double;
                }
            } else {
                zsv_ext2 = zsb;
                zsv_ext1 = zsv_ext2;
                zsv_ext0 = zsv_ext1;
                dz_ext2 =
                    dz0 - 4 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
                dz_ext1 = dz_ext2;
                dz_ext0 = dz_ext1;
            }
            if c_1 as ::core::ffi::c_int & 0x8 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                wsv_ext1 = wsb + 1 as ::core::ffi::c_int;
                wsv_ext0 = wsv_ext1;
                wsv_ext2 = wsb + 2 as ::core::ffi::c_int;
                dw_ext1 = dw0
                    - 1 as ::core::ffi::c_int as ::core::ffi::c_double
                    - 4 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
                dw_ext0 = dw_ext1;
                dw_ext2 = dw0
                    - 2 as ::core::ffi::c_int as ::core::ffi::c_double
                    - 4 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
            } else {
                wsv_ext2 = wsb;
                wsv_ext1 = wsv_ext2;
                wsv_ext0 = wsv_ext1;
                dw_ext2 =
                    dw0 - 4 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
                dw_ext1 = dw_ext2;
                dw_ext0 = dw_ext1;
            }
        } else {
            let mut c_2: int8_t =
                (aPoint_0 as ::core::ffi::c_int & bPoint_0 as ::core::ffi::c_int) as int8_t;
            if c_2 as ::core::ffi::c_int & 0x1 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                xsv_ext2 = xsb + 1 as ::core::ffi::c_int;
                xsv_ext0 = xsv_ext2;
                xsv_ext1 = xsb + 2 as ::core::ffi::c_int;
                dx_ext0 = dx0
                    - 1 as ::core::ffi::c_int as ::core::ffi::c_double
                    - 2 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
                dx_ext1 = dx0
                    - 2 as ::core::ffi::c_int as ::core::ffi::c_double
                    - 3 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
                dx_ext2 = dx0
                    - 1 as ::core::ffi::c_int as ::core::ffi::c_double
                    - 3 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
            } else {
                xsv_ext2 = xsb;
                xsv_ext1 = xsv_ext2;
                xsv_ext0 = xsv_ext1;
                dx_ext0 =
                    dx0 - 2 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
                dx_ext2 =
                    dx0 - 3 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
                dx_ext1 = dx_ext2;
            }
            if c_2 as ::core::ffi::c_int & 0x2 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                ysv_ext2 = ysb + 1 as ::core::ffi::c_int;
                ysv_ext1 = ysv_ext2;
                ysv_ext0 = ysv_ext1;
                dy_ext0 = dy0
                    - 1 as ::core::ffi::c_int as ::core::ffi::c_double
                    - 2 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
                dy_ext2 = dy0
                    - 1 as ::core::ffi::c_int as ::core::ffi::c_double
                    - 3 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
                dy_ext1 = dy_ext2;
                if c_2 as ::core::ffi::c_int & 0x1 as ::core::ffi::c_int != 0 as ::core::ffi::c_int
                {
                    ysv_ext2 += 1 as ::core::ffi::c_int;
                    dy_ext2 -= 1 as ::core::ffi::c_int as ::core::ffi::c_double;
                } else {
                    ysv_ext1 += 1 as ::core::ffi::c_int;
                    dy_ext1 -= 1 as ::core::ffi::c_int as ::core::ffi::c_double;
                }
            } else {
                ysv_ext2 = ysb;
                ysv_ext1 = ysv_ext2;
                ysv_ext0 = ysv_ext1;
                dy_ext0 =
                    dy0 - 2 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
                dy_ext2 =
                    dy0 - 3 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
                dy_ext1 = dy_ext2;
            }
            if c_2 as ::core::ffi::c_int & 0x4 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                zsv_ext2 = zsb + 1 as ::core::ffi::c_int;
                zsv_ext1 = zsv_ext2;
                zsv_ext0 = zsv_ext1;
                dz_ext0 = dz0
                    - 1 as ::core::ffi::c_int as ::core::ffi::c_double
                    - 2 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
                dz_ext2 = dz0
                    - 1 as ::core::ffi::c_int as ::core::ffi::c_double
                    - 3 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
                dz_ext1 = dz_ext2;
                if c_2 as ::core::ffi::c_int & 0x3 as ::core::ffi::c_int != 0 as ::core::ffi::c_int
                {
                    zsv_ext2 += 1 as ::core::ffi::c_int;
                    dz_ext2 -= 1 as ::core::ffi::c_int as ::core::ffi::c_double;
                } else {
                    zsv_ext1 += 1 as ::core::ffi::c_int;
                    dz_ext1 -= 1 as ::core::ffi::c_int as ::core::ffi::c_double;
                }
            } else {
                zsv_ext2 = zsb;
                zsv_ext1 = zsv_ext2;
                zsv_ext0 = zsv_ext1;
                dz_ext0 =
                    dz0 - 2 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
                dz_ext2 =
                    dz0 - 3 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
                dz_ext1 = dz_ext2;
            }
            if c_2 as ::core::ffi::c_int & 0x8 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                wsv_ext1 = wsb + 1 as ::core::ffi::c_int;
                wsv_ext0 = wsv_ext1;
                wsv_ext2 = wsb + 2 as ::core::ffi::c_int;
                dw_ext0 = dw0
                    - 1 as ::core::ffi::c_int as ::core::ffi::c_double
                    - 2 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
                dw_ext1 = dw0
                    - 1 as ::core::ffi::c_int as ::core::ffi::c_double
                    - 3 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
                dw_ext2 = dw0
                    - 2 as ::core::ffi::c_int as ::core::ffi::c_double
                    - 3 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
            } else {
                wsv_ext2 = wsb;
                wsv_ext1 = wsv_ext2;
                wsv_ext0 = wsv_ext1;
                dw_ext0 =
                    dw0 - 2 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
                dw_ext2 =
                    dw0 - 3 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
                dw_ext1 = dw_ext2;
            }
        }
        let mut dx4_0: ::core::ffi::c_double = dx0
            - 1 as ::core::ffi::c_int as ::core::ffi::c_double
            - 3 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
        let mut dy4_0: ::core::ffi::c_double = dy0
            - 1 as ::core::ffi::c_int as ::core::ffi::c_double
            - 3 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
        let mut dz4_0: ::core::ffi::c_double = dz0
            - 1 as ::core::ffi::c_int as ::core::ffi::c_double
            - 3 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
        let mut dw4_0: ::core::ffi::c_double =
            dw0 - 3 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
        let mut attn4_0: ::core::ffi::c_double = 2 as ::core::ffi::c_int as ::core::ffi::c_double
            - dx4_0 * dx4_0
            - dy4_0 * dy4_0
            - dz4_0 * dz4_0
            - dw4_0 * dw4_0;
        if attn4_0 > 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            attn4_0 *= attn4_0;
            value += attn4_0
                * attn4_0
                * extrapolate4(
                    ctx,
                    xsb + 1 as ::core::ffi::c_int,
                    ysb + 1 as ::core::ffi::c_int,
                    zsb + 1 as ::core::ffi::c_int,
                    wsb + 0 as ::core::ffi::c_int,
                    dx4_0,
                    dy4_0,
                    dz4_0,
                    dw4_0,
                );
        }
        let mut dx3_0: ::core::ffi::c_double = dx4_0;
        let mut dy3_0: ::core::ffi::c_double = dy4_0;
        let mut dz3_0: ::core::ffi::c_double =
            dz0 - 3 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
        let mut dw3_0: ::core::ffi::c_double = dw0
            - 1 as ::core::ffi::c_int as ::core::ffi::c_double
            - 3 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
        let mut attn3_0: ::core::ffi::c_double = 2 as ::core::ffi::c_int as ::core::ffi::c_double
            - dx3_0 * dx3_0
            - dy3_0 * dy3_0
            - dz3_0 * dz3_0
            - dw3_0 * dw3_0;
        if attn3_0 > 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            attn3_0 *= attn3_0;
            value += attn3_0
                * attn3_0
                * extrapolate4(
                    ctx,
                    xsb + 1 as ::core::ffi::c_int,
                    ysb + 1 as ::core::ffi::c_int,
                    zsb + 0 as ::core::ffi::c_int,
                    wsb + 1 as ::core::ffi::c_int,
                    dx3_0,
                    dy3_0,
                    dz3_0,
                    dw3_0,
                );
        }
        let mut dx2_0: ::core::ffi::c_double = dx4_0;
        let mut dy2_0: ::core::ffi::c_double =
            dy0 - 3 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
        let mut dz2_0: ::core::ffi::c_double = dz4_0;
        let mut dw2_0: ::core::ffi::c_double = dw3_0;
        let mut attn2_0: ::core::ffi::c_double = 2 as ::core::ffi::c_int as ::core::ffi::c_double
            - dx2_0 * dx2_0
            - dy2_0 * dy2_0
            - dz2_0 * dz2_0
            - dw2_0 * dw2_0;
        if attn2_0 > 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            attn2_0 *= attn2_0;
            value += attn2_0
                * attn2_0
                * extrapolate4(
                    ctx,
                    xsb + 1 as ::core::ffi::c_int,
                    ysb + 0 as ::core::ffi::c_int,
                    zsb + 1 as ::core::ffi::c_int,
                    wsb + 1 as ::core::ffi::c_int,
                    dx2_0,
                    dy2_0,
                    dz2_0,
                    dw2_0,
                );
        }
        let mut dx1_0: ::core::ffi::c_double =
            dx0 - 3 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
        let mut dz1_0: ::core::ffi::c_double = dz4_0;
        let mut dy1_0: ::core::ffi::c_double = dy4_0;
        let mut dw1_0: ::core::ffi::c_double = dw3_0;
        let mut attn1_0: ::core::ffi::c_double = 2 as ::core::ffi::c_int as ::core::ffi::c_double
            - dx1_0 * dx1_0
            - dy1_0 * dy1_0
            - dz1_0 * dz1_0
            - dw1_0 * dw1_0;
        if attn1_0 > 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            attn1_0 *= attn1_0;
            value += attn1_0
                * attn1_0
                * extrapolate4(
                    ctx,
                    xsb + 0 as ::core::ffi::c_int,
                    ysb + 1 as ::core::ffi::c_int,
                    zsb + 1 as ::core::ffi::c_int,
                    wsb + 1 as ::core::ffi::c_int,
                    dx1_0,
                    dy1_0,
                    dz1_0,
                    dw1_0,
                );
        }
        dx0 = dx0
            - 1 as ::core::ffi::c_int as ::core::ffi::c_double
            - 4 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
        dy0 = dy0
            - 1 as ::core::ffi::c_int as ::core::ffi::c_double
            - 4 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
        dz0 = dz0
            - 1 as ::core::ffi::c_int as ::core::ffi::c_double
            - 4 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
        dw0 = dw0
            - 1 as ::core::ffi::c_int as ::core::ffi::c_double
            - 4 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
        let mut attn0_0: ::core::ffi::c_double = 2 as ::core::ffi::c_int as ::core::ffi::c_double
            - dx0 * dx0
            - dy0 * dy0
            - dz0 * dz0
            - dw0 * dw0;
        if attn0_0 > 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            attn0_0 *= attn0_0;
            value += attn0_0
                * attn0_0
                * extrapolate4(
                    ctx,
                    xsb + 1 as ::core::ffi::c_int,
                    ysb + 1 as ::core::ffi::c_int,
                    zsb + 1 as ::core::ffi::c_int,
                    wsb + 1 as ::core::ffi::c_int,
                    dx0,
                    dy0,
                    dz0,
                    dw0,
                );
        }
    } else if inSum <= 2 as ::core::ffi::c_int as ::core::ffi::c_double {
        let mut aScore_1: ::core::ffi::c_double = 0.;
        let mut aPoint_1: int8_t = 0;
        let mut aIsBiggerSide: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
        let mut bScore_1: ::core::ffi::c_double = 0.;
        let mut bPoint_1: int8_t = 0;
        let mut bIsBiggerSide: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
        if xins + yins > zins + wins {
            aScore_1 = xins + yins;
            aPoint_1 = 0x3 as int8_t;
        } else {
            aScore_1 = zins + wins;
            aPoint_1 = 0xc as int8_t;
        }
        if xins + zins > yins + wins {
            bScore_1 = xins + zins;
            bPoint_1 = 0x5 as int8_t;
        } else {
            bScore_1 = yins + wins;
            bPoint_1 = 0xa as int8_t;
        }
        if xins + wins > yins + zins {
            let mut score: ::core::ffi::c_double = xins + wins;
            if aScore_1 >= bScore_1 && score > bScore_1 {
                bScore_1 = score;
                bPoint_1 = 0x9 as int8_t;
            } else if aScore_1 < bScore_1 && score > aScore_1 {
                aScore_1 = score;
                aPoint_1 = 0x9 as int8_t;
            }
        } else {
            let mut score_0: ::core::ffi::c_double = yins + zins;
            if aScore_1 >= bScore_1 && score_0 > bScore_1 {
                bScore_1 = score_0;
                bPoint_1 = 0x6 as int8_t;
            } else if aScore_1 < bScore_1 && score_0 > aScore_1 {
                aScore_1 = score_0;
                aPoint_1 = 0x6 as int8_t;
            }
        }
        let mut p1: ::core::ffi::c_double =
            2 as ::core::ffi::c_int as ::core::ffi::c_double - inSum + xins;
        if aScore_1 >= bScore_1 && p1 > bScore_1 {
            bScore_1 = p1;
            bPoint_1 = 0x1 as int8_t;
            bIsBiggerSide = 0 as ::core::ffi::c_int;
        } else if aScore_1 < bScore_1 && p1 > aScore_1 {
            aScore_1 = p1;
            aPoint_1 = 0x1 as int8_t;
            aIsBiggerSide = 0 as ::core::ffi::c_int;
        }
        let mut p2: ::core::ffi::c_double =
            2 as ::core::ffi::c_int as ::core::ffi::c_double - inSum + yins;
        if aScore_1 >= bScore_1 && p2 > bScore_1 {
            bScore_1 = p2;
            bPoint_1 = 0x2 as int8_t;
            bIsBiggerSide = 0 as ::core::ffi::c_int;
        } else if aScore_1 < bScore_1 && p2 > aScore_1 {
            aScore_1 = p2;
            aPoint_1 = 0x2 as int8_t;
            aIsBiggerSide = 0 as ::core::ffi::c_int;
        }
        let mut p3: ::core::ffi::c_double =
            2 as ::core::ffi::c_int as ::core::ffi::c_double - inSum + zins;
        if aScore_1 >= bScore_1 && p3 > bScore_1 {
            bScore_1 = p3;
            bPoint_1 = 0x4 as int8_t;
            bIsBiggerSide = 0 as ::core::ffi::c_int;
        } else if aScore_1 < bScore_1 && p3 > aScore_1 {
            aScore_1 = p3;
            aPoint_1 = 0x4 as int8_t;
            aIsBiggerSide = 0 as ::core::ffi::c_int;
        }
        let mut p4: ::core::ffi::c_double =
            2 as ::core::ffi::c_int as ::core::ffi::c_double - inSum + wins;
        if aScore_1 >= bScore_1 && p4 > bScore_1 {
            bScore_1 = p4;
            bPoint_1 = 0x8 as int8_t;
            bIsBiggerSide = 0 as ::core::ffi::c_int;
        } else if aScore_1 < bScore_1 && p4 > aScore_1 {
            aScore_1 = p4;
            aPoint_1 = 0x8 as int8_t;
            aIsBiggerSide = 0 as ::core::ffi::c_int;
        }
        if aIsBiggerSide == bIsBiggerSide {
            if aIsBiggerSide != 0 {
                let mut c1: int8_t =
                    (aPoint_1 as ::core::ffi::c_int | bPoint_1 as ::core::ffi::c_int) as int8_t;
                let mut c2: int8_t =
                    (aPoint_1 as ::core::ffi::c_int & bPoint_1 as ::core::ffi::c_int) as int8_t;
                if c1 as ::core::ffi::c_int & 0x1 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                    xsv_ext0 = xsb;
                    xsv_ext1 = xsb - 1 as ::core::ffi::c_int;
                    dx_ext0 =
                        dx0 - 3 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
                    dx_ext1 = dx0 + 1 as ::core::ffi::c_int as ::core::ffi::c_double
                        - 2 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
                } else {
                    xsv_ext1 = xsb + 1 as ::core::ffi::c_int;
                    xsv_ext0 = xsv_ext1;
                    dx_ext0 = dx0
                        - 1 as ::core::ffi::c_int as ::core::ffi::c_double
                        - 3 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
                    dx_ext1 = dx0
                        - 1 as ::core::ffi::c_int as ::core::ffi::c_double
                        - 2 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
                }
                if c1 as ::core::ffi::c_int & 0x2 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                    ysv_ext0 = ysb;
                    ysv_ext1 = ysb - 1 as ::core::ffi::c_int;
                    dy_ext0 =
                        dy0 - 3 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
                    dy_ext1 = dy0 + 1 as ::core::ffi::c_int as ::core::ffi::c_double
                        - 2 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
                } else {
                    ysv_ext1 = ysb + 1 as ::core::ffi::c_int;
                    ysv_ext0 = ysv_ext1;
                    dy_ext0 = dy0
                        - 1 as ::core::ffi::c_int as ::core::ffi::c_double
                        - 3 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
                    dy_ext1 = dy0
                        - 1 as ::core::ffi::c_int as ::core::ffi::c_double
                        - 2 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
                }
                if c1 as ::core::ffi::c_int & 0x4 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                    zsv_ext0 = zsb;
                    zsv_ext1 = zsb - 1 as ::core::ffi::c_int;
                    dz_ext0 =
                        dz0 - 3 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
                    dz_ext1 = dz0 + 1 as ::core::ffi::c_int as ::core::ffi::c_double
                        - 2 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
                } else {
                    zsv_ext1 = zsb + 1 as ::core::ffi::c_int;
                    zsv_ext0 = zsv_ext1;
                    dz_ext0 = dz0
                        - 1 as ::core::ffi::c_int as ::core::ffi::c_double
                        - 3 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
                    dz_ext1 = dz0
                        - 1 as ::core::ffi::c_int as ::core::ffi::c_double
                        - 2 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
                }
                if c1 as ::core::ffi::c_int & 0x8 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                    wsv_ext0 = wsb;
                    wsv_ext1 = wsb - 1 as ::core::ffi::c_int;
                    dw_ext0 =
                        dw0 - 3 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
                    dw_ext1 = dw0 + 1 as ::core::ffi::c_int as ::core::ffi::c_double
                        - 2 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
                } else {
                    wsv_ext1 = wsb + 1 as ::core::ffi::c_int;
                    wsv_ext0 = wsv_ext1;
                    dw_ext0 = dw0
                        - 1 as ::core::ffi::c_int as ::core::ffi::c_double
                        - 3 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
                    dw_ext1 = dw0
                        - 1 as ::core::ffi::c_int as ::core::ffi::c_double
                        - 2 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
                }
                xsv_ext2 = xsb;
                ysv_ext2 = ysb;
                zsv_ext2 = zsb;
                wsv_ext2 = wsb;
                dx_ext2 =
                    dx0 - 2 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
                dy_ext2 =
                    dy0 - 2 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
                dz_ext2 =
                    dz0 - 2 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
                dw_ext2 =
                    dw0 - 2 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
                if c2 as ::core::ffi::c_int & 0x1 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                    xsv_ext2 += 2 as ::core::ffi::c_int;
                    dx_ext2 -= 2 as ::core::ffi::c_int as ::core::ffi::c_double;
                } else if c2 as ::core::ffi::c_int & 0x2 as ::core::ffi::c_int
                    != 0 as ::core::ffi::c_int
                {
                    ysv_ext2 += 2 as ::core::ffi::c_int;
                    dy_ext2 -= 2 as ::core::ffi::c_int as ::core::ffi::c_double;
                } else if c2 as ::core::ffi::c_int & 0x4 as ::core::ffi::c_int
                    != 0 as ::core::ffi::c_int
                {
                    zsv_ext2 += 2 as ::core::ffi::c_int;
                    dz_ext2 -= 2 as ::core::ffi::c_int as ::core::ffi::c_double;
                } else {
                    wsv_ext2 += 2 as ::core::ffi::c_int;
                    dw_ext2 -= 2 as ::core::ffi::c_int as ::core::ffi::c_double;
                }
            } else {
                xsv_ext2 = xsb;
                ysv_ext2 = ysb;
                zsv_ext2 = zsb;
                wsv_ext2 = wsb;
                dx_ext2 = dx0;
                dy_ext2 = dy0;
                dz_ext2 = dz0;
                dw_ext2 = dw0;
                let mut c_3: int8_t =
                    (aPoint_1 as ::core::ffi::c_int | bPoint_1 as ::core::ffi::c_int) as int8_t;
                if c_3 as ::core::ffi::c_int & 0x1 as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                {
                    xsv_ext0 = xsb - 1 as ::core::ffi::c_int;
                    xsv_ext1 = xsb;
                    dx_ext0 =
                        dx0 + 1 as ::core::ffi::c_int as ::core::ffi::c_double - SQUISH_CONSTANT_4D;
                    dx_ext1 = dx0 - SQUISH_CONSTANT_4D;
                } else {
                    xsv_ext1 = xsb + 1 as ::core::ffi::c_int;
                    xsv_ext0 = xsv_ext1;
                    dx_ext1 =
                        dx0 - 1 as ::core::ffi::c_int as ::core::ffi::c_double - SQUISH_CONSTANT_4D;
                    dx_ext0 = dx_ext1;
                }
                if c_3 as ::core::ffi::c_int & 0x2 as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                {
                    ysv_ext1 = ysb;
                    ysv_ext0 = ysv_ext1;
                    dy_ext1 = dy0 - SQUISH_CONSTANT_4D;
                    dy_ext0 = dy_ext1;
                    if c_3 as ::core::ffi::c_int & 0x1 as ::core::ffi::c_int
                        == 0x1 as ::core::ffi::c_int
                    {
                        ysv_ext0 -= 1 as ::core::ffi::c_int;
                        dy_ext0 += 1 as ::core::ffi::c_int as ::core::ffi::c_double;
                    } else {
                        ysv_ext1 -= 1 as ::core::ffi::c_int;
                        dy_ext1 += 1 as ::core::ffi::c_int as ::core::ffi::c_double;
                    }
                } else {
                    ysv_ext1 = ysb + 1 as ::core::ffi::c_int;
                    ysv_ext0 = ysv_ext1;
                    dy_ext1 =
                        dy0 - 1 as ::core::ffi::c_int as ::core::ffi::c_double - SQUISH_CONSTANT_4D;
                    dy_ext0 = dy_ext1;
                }
                if c_3 as ::core::ffi::c_int & 0x4 as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                {
                    zsv_ext1 = zsb;
                    zsv_ext0 = zsv_ext1;
                    dz_ext1 = dz0 - SQUISH_CONSTANT_4D;
                    dz_ext0 = dz_ext1;
                    if c_3 as ::core::ffi::c_int & 0x3 as ::core::ffi::c_int
                        == 0x3 as ::core::ffi::c_int
                    {
                        zsv_ext0 -= 1 as ::core::ffi::c_int;
                        dz_ext0 += 1 as ::core::ffi::c_int as ::core::ffi::c_double;
                    } else {
                        zsv_ext1 -= 1 as ::core::ffi::c_int;
                        dz_ext1 += 1 as ::core::ffi::c_int as ::core::ffi::c_double;
                    }
                } else {
                    zsv_ext1 = zsb + 1 as ::core::ffi::c_int;
                    zsv_ext0 = zsv_ext1;
                    dz_ext1 =
                        dz0 - 1 as ::core::ffi::c_int as ::core::ffi::c_double - SQUISH_CONSTANT_4D;
                    dz_ext0 = dz_ext1;
                }
                if c_3 as ::core::ffi::c_int & 0x8 as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                {
                    wsv_ext0 = wsb;
                    wsv_ext1 = wsb - 1 as ::core::ffi::c_int;
                    dw_ext0 = dw0 - SQUISH_CONSTANT_4D;
                    dw_ext1 =
                        dw0 + 1 as ::core::ffi::c_int as ::core::ffi::c_double - SQUISH_CONSTANT_4D;
                } else {
                    wsv_ext1 = wsb + 1 as ::core::ffi::c_int;
                    wsv_ext0 = wsv_ext1;
                    dw_ext1 =
                        dw0 - 1 as ::core::ffi::c_int as ::core::ffi::c_double - SQUISH_CONSTANT_4D;
                    dw_ext0 = dw_ext1;
                }
            }
        } else {
            let mut c1_0: int8_t = 0;
            let mut c2_0: int8_t = 0;
            if aIsBiggerSide != 0 {
                c1_0 = aPoint_1;
                c2_0 = bPoint_1;
            } else {
                c1_0 = bPoint_1;
                c2_0 = aPoint_1;
            }
            if c1_0 as ::core::ffi::c_int & 0x1 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                xsv_ext0 = xsb - 1 as ::core::ffi::c_int;
                xsv_ext1 = xsb;
                dx_ext0 =
                    dx0 + 1 as ::core::ffi::c_int as ::core::ffi::c_double - SQUISH_CONSTANT_4D;
                dx_ext1 = dx0 - SQUISH_CONSTANT_4D;
            } else {
                xsv_ext1 = xsb + 1 as ::core::ffi::c_int;
                xsv_ext0 = xsv_ext1;
                dx_ext1 =
                    dx0 - 1 as ::core::ffi::c_int as ::core::ffi::c_double - SQUISH_CONSTANT_4D;
                dx_ext0 = dx_ext1;
            }
            if c1_0 as ::core::ffi::c_int & 0x2 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                ysv_ext1 = ysb;
                ysv_ext0 = ysv_ext1;
                dy_ext1 = dy0 - SQUISH_CONSTANT_4D;
                dy_ext0 = dy_ext1;
                if c1_0 as ::core::ffi::c_int & 0x1 as ::core::ffi::c_int
                    == 0x1 as ::core::ffi::c_int
                {
                    ysv_ext0 -= 1 as ::core::ffi::c_int;
                    dy_ext0 += 1 as ::core::ffi::c_int as ::core::ffi::c_double;
                } else {
                    ysv_ext1 -= 1 as ::core::ffi::c_int;
                    dy_ext1 += 1 as ::core::ffi::c_int as ::core::ffi::c_double;
                }
            } else {
                ysv_ext1 = ysb + 1 as ::core::ffi::c_int;
                ysv_ext0 = ysv_ext1;
                dy_ext1 =
                    dy0 - 1 as ::core::ffi::c_int as ::core::ffi::c_double - SQUISH_CONSTANT_4D;
                dy_ext0 = dy_ext1;
            }
            if c1_0 as ::core::ffi::c_int & 0x4 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                zsv_ext1 = zsb;
                zsv_ext0 = zsv_ext1;
                dz_ext1 = dz0 - SQUISH_CONSTANT_4D;
                dz_ext0 = dz_ext1;
                if c1_0 as ::core::ffi::c_int & 0x3 as ::core::ffi::c_int
                    == 0x3 as ::core::ffi::c_int
                {
                    zsv_ext0 -= 1 as ::core::ffi::c_int;
                    dz_ext0 += 1 as ::core::ffi::c_int as ::core::ffi::c_double;
                } else {
                    zsv_ext1 -= 1 as ::core::ffi::c_int;
                    dz_ext1 += 1 as ::core::ffi::c_int as ::core::ffi::c_double;
                }
            } else {
                zsv_ext1 = zsb + 1 as ::core::ffi::c_int;
                zsv_ext0 = zsv_ext1;
                dz_ext1 =
                    dz0 - 1 as ::core::ffi::c_int as ::core::ffi::c_double - SQUISH_CONSTANT_4D;
                dz_ext0 = dz_ext1;
            }
            if c1_0 as ::core::ffi::c_int & 0x8 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                wsv_ext0 = wsb;
                wsv_ext1 = wsb - 1 as ::core::ffi::c_int;
                dw_ext0 = dw0 - SQUISH_CONSTANT_4D;
                dw_ext1 =
                    dw0 + 1 as ::core::ffi::c_int as ::core::ffi::c_double - SQUISH_CONSTANT_4D;
            } else {
                wsv_ext1 = wsb + 1 as ::core::ffi::c_int;
                wsv_ext0 = wsv_ext1;
                dw_ext1 =
                    dw0 - 1 as ::core::ffi::c_int as ::core::ffi::c_double - SQUISH_CONSTANT_4D;
                dw_ext0 = dw_ext1;
            }
            xsv_ext2 = xsb;
            ysv_ext2 = ysb;
            zsv_ext2 = zsb;
            wsv_ext2 = wsb;
            dx_ext2 = dx0 - 2 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
            dy_ext2 = dy0 - 2 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
            dz_ext2 = dz0 - 2 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
            dw_ext2 = dw0 - 2 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
            if c2_0 as ::core::ffi::c_int & 0x1 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                xsv_ext2 += 2 as ::core::ffi::c_int;
                dx_ext2 -= 2 as ::core::ffi::c_int as ::core::ffi::c_double;
            } else if c2_0 as ::core::ffi::c_int & 0x2 as ::core::ffi::c_int
                != 0 as ::core::ffi::c_int
            {
                ysv_ext2 += 2 as ::core::ffi::c_int;
                dy_ext2 -= 2 as ::core::ffi::c_int as ::core::ffi::c_double;
            } else if c2_0 as ::core::ffi::c_int & 0x4 as ::core::ffi::c_int
                != 0 as ::core::ffi::c_int
            {
                zsv_ext2 += 2 as ::core::ffi::c_int;
                dz_ext2 -= 2 as ::core::ffi::c_int as ::core::ffi::c_double;
            } else {
                wsv_ext2 += 2 as ::core::ffi::c_int;
                dw_ext2 -= 2 as ::core::ffi::c_int as ::core::ffi::c_double;
            }
        }
        let mut dx1_1: ::core::ffi::c_double =
            dx0 - 1 as ::core::ffi::c_int as ::core::ffi::c_double - SQUISH_CONSTANT_4D;
        let mut dy1_1: ::core::ffi::c_double =
            dy0 - 0 as ::core::ffi::c_int as ::core::ffi::c_double - SQUISH_CONSTANT_4D;
        let mut dz1_1: ::core::ffi::c_double =
            dz0 - 0 as ::core::ffi::c_int as ::core::ffi::c_double - SQUISH_CONSTANT_4D;
        let mut dw1_1: ::core::ffi::c_double =
            dw0 - 0 as ::core::ffi::c_int as ::core::ffi::c_double - SQUISH_CONSTANT_4D;
        let mut attn1_1: ::core::ffi::c_double = 2 as ::core::ffi::c_int as ::core::ffi::c_double
            - dx1_1 * dx1_1
            - dy1_1 * dy1_1
            - dz1_1 * dz1_1
            - dw1_1 * dw1_1;
        if attn1_1 > 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            attn1_1 *= attn1_1;
            value += attn1_1
                * attn1_1
                * extrapolate4(
                    ctx,
                    xsb + 1 as ::core::ffi::c_int,
                    ysb + 0 as ::core::ffi::c_int,
                    zsb + 0 as ::core::ffi::c_int,
                    wsb + 0 as ::core::ffi::c_int,
                    dx1_1,
                    dy1_1,
                    dz1_1,
                    dw1_1,
                );
        }
        let mut dx2_1: ::core::ffi::c_double =
            dx0 - 0 as ::core::ffi::c_int as ::core::ffi::c_double - SQUISH_CONSTANT_4D;
        let mut dy2_1: ::core::ffi::c_double =
            dy0 - 1 as ::core::ffi::c_int as ::core::ffi::c_double - SQUISH_CONSTANT_4D;
        let mut dz2_1: ::core::ffi::c_double = dz1_1;
        let mut dw2_1: ::core::ffi::c_double = dw1_1;
        let mut attn2_1: ::core::ffi::c_double = 2 as ::core::ffi::c_int as ::core::ffi::c_double
            - dx2_1 * dx2_1
            - dy2_1 * dy2_1
            - dz2_1 * dz2_1
            - dw2_1 * dw2_1;
        if attn2_1 > 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            attn2_1 *= attn2_1;
            value += attn2_1
                * attn2_1
                * extrapolate4(
                    ctx,
                    xsb + 0 as ::core::ffi::c_int,
                    ysb + 1 as ::core::ffi::c_int,
                    zsb + 0 as ::core::ffi::c_int,
                    wsb + 0 as ::core::ffi::c_int,
                    dx2_1,
                    dy2_1,
                    dz2_1,
                    dw2_1,
                );
        }
        let mut dx3_1: ::core::ffi::c_double = dx2_1;
        let mut dy3_1: ::core::ffi::c_double = dy1_1;
        let mut dz3_1: ::core::ffi::c_double =
            dz0 - 1 as ::core::ffi::c_int as ::core::ffi::c_double - SQUISH_CONSTANT_4D;
        let mut dw3_1: ::core::ffi::c_double = dw1_1;
        let mut attn3_1: ::core::ffi::c_double = 2 as ::core::ffi::c_int as ::core::ffi::c_double
            - dx3_1 * dx3_1
            - dy3_1 * dy3_1
            - dz3_1 * dz3_1
            - dw3_1 * dw3_1;
        if attn3_1 > 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            attn3_1 *= attn3_1;
            value += attn3_1
                * attn3_1
                * extrapolate4(
                    ctx,
                    xsb + 0 as ::core::ffi::c_int,
                    ysb + 0 as ::core::ffi::c_int,
                    zsb + 1 as ::core::ffi::c_int,
                    wsb + 0 as ::core::ffi::c_int,
                    dx3_1,
                    dy3_1,
                    dz3_1,
                    dw3_1,
                );
        }
        let mut dx4_1: ::core::ffi::c_double = dx2_1;
        let mut dy4_1: ::core::ffi::c_double = dy1_1;
        let mut dz4_1: ::core::ffi::c_double = dz1_1;
        let mut dw4_1: ::core::ffi::c_double =
            dw0 - 1 as ::core::ffi::c_int as ::core::ffi::c_double - SQUISH_CONSTANT_4D;
        let mut attn4_1: ::core::ffi::c_double = 2 as ::core::ffi::c_int as ::core::ffi::c_double
            - dx4_1 * dx4_1
            - dy4_1 * dy4_1
            - dz4_1 * dz4_1
            - dw4_1 * dw4_1;
        if attn4_1 > 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            attn4_1 *= attn4_1;
            value += attn4_1
                * attn4_1
                * extrapolate4(
                    ctx,
                    xsb + 0 as ::core::ffi::c_int,
                    ysb + 0 as ::core::ffi::c_int,
                    zsb + 0 as ::core::ffi::c_int,
                    wsb + 1 as ::core::ffi::c_int,
                    dx4_1,
                    dy4_1,
                    dz4_1,
                    dw4_1,
                );
        }
        let mut dx5: ::core::ffi::c_double = dx0
            - 1 as ::core::ffi::c_int as ::core::ffi::c_double
            - 2 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
        let mut dy5: ::core::ffi::c_double = dy0
            - 1 as ::core::ffi::c_int as ::core::ffi::c_double
            - 2 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
        let mut dz5: ::core::ffi::c_double = dz0
            - 0 as ::core::ffi::c_int as ::core::ffi::c_double
            - 2 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
        let mut dw5: ::core::ffi::c_double = dw0
            - 0 as ::core::ffi::c_int as ::core::ffi::c_double
            - 2 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
        let mut attn5: ::core::ffi::c_double = 2 as ::core::ffi::c_int as ::core::ffi::c_double
            - dx5 * dx5
            - dy5 * dy5
            - dz5 * dz5
            - dw5 * dw5;
        if attn5 > 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            attn5 *= attn5;
            value += attn5
                * attn5
                * extrapolate4(
                    ctx,
                    xsb + 1 as ::core::ffi::c_int,
                    ysb + 1 as ::core::ffi::c_int,
                    zsb + 0 as ::core::ffi::c_int,
                    wsb + 0 as ::core::ffi::c_int,
                    dx5,
                    dy5,
                    dz5,
                    dw5,
                );
        }
        let mut dx6: ::core::ffi::c_double = dx0
            - 1 as ::core::ffi::c_int as ::core::ffi::c_double
            - 2 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
        let mut dy6: ::core::ffi::c_double = dy0
            - 0 as ::core::ffi::c_int as ::core::ffi::c_double
            - 2 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
        let mut dz6: ::core::ffi::c_double = dz0
            - 1 as ::core::ffi::c_int as ::core::ffi::c_double
            - 2 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
        let mut dw6: ::core::ffi::c_double = dw0
            - 0 as ::core::ffi::c_int as ::core::ffi::c_double
            - 2 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
        let mut attn6: ::core::ffi::c_double = 2 as ::core::ffi::c_int as ::core::ffi::c_double
            - dx6 * dx6
            - dy6 * dy6
            - dz6 * dz6
            - dw6 * dw6;
        if attn6 > 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            attn6 *= attn6;
            value += attn6
                * attn6
                * extrapolate4(
                    ctx,
                    xsb + 1 as ::core::ffi::c_int,
                    ysb + 0 as ::core::ffi::c_int,
                    zsb + 1 as ::core::ffi::c_int,
                    wsb + 0 as ::core::ffi::c_int,
                    dx6,
                    dy6,
                    dz6,
                    dw6,
                );
        }
        let mut dx7: ::core::ffi::c_double = dx0
            - 1 as ::core::ffi::c_int as ::core::ffi::c_double
            - 2 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
        let mut dy7: ::core::ffi::c_double = dy0
            - 0 as ::core::ffi::c_int as ::core::ffi::c_double
            - 2 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
        let mut dz7: ::core::ffi::c_double = dz0
            - 0 as ::core::ffi::c_int as ::core::ffi::c_double
            - 2 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
        let mut dw7: ::core::ffi::c_double = dw0
            - 1 as ::core::ffi::c_int as ::core::ffi::c_double
            - 2 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
        let mut attn7: ::core::ffi::c_double = 2 as ::core::ffi::c_int as ::core::ffi::c_double
            - dx7 * dx7
            - dy7 * dy7
            - dz7 * dz7
            - dw7 * dw7;
        if attn7 > 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            attn7 *= attn7;
            value += attn7
                * attn7
                * extrapolate4(
                    ctx,
                    xsb + 1 as ::core::ffi::c_int,
                    ysb + 0 as ::core::ffi::c_int,
                    zsb + 0 as ::core::ffi::c_int,
                    wsb + 1 as ::core::ffi::c_int,
                    dx7,
                    dy7,
                    dz7,
                    dw7,
                );
        }
        let mut dx8: ::core::ffi::c_double = dx0
            - 0 as ::core::ffi::c_int as ::core::ffi::c_double
            - 2 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
        let mut dy8: ::core::ffi::c_double = dy0
            - 1 as ::core::ffi::c_int as ::core::ffi::c_double
            - 2 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
        let mut dz8: ::core::ffi::c_double = dz0
            - 1 as ::core::ffi::c_int as ::core::ffi::c_double
            - 2 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
        let mut dw8: ::core::ffi::c_double = dw0
            - 0 as ::core::ffi::c_int as ::core::ffi::c_double
            - 2 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
        let mut attn8: ::core::ffi::c_double = 2 as ::core::ffi::c_int as ::core::ffi::c_double
            - dx8 * dx8
            - dy8 * dy8
            - dz8 * dz8
            - dw8 * dw8;
        if attn8 > 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            attn8 *= attn8;
            value += attn8
                * attn8
                * extrapolate4(
                    ctx,
                    xsb + 0 as ::core::ffi::c_int,
                    ysb + 1 as ::core::ffi::c_int,
                    zsb + 1 as ::core::ffi::c_int,
                    wsb + 0 as ::core::ffi::c_int,
                    dx8,
                    dy8,
                    dz8,
                    dw8,
                );
        }
        let mut dx9: ::core::ffi::c_double = dx0
            - 0 as ::core::ffi::c_int as ::core::ffi::c_double
            - 2 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
        let mut dy9: ::core::ffi::c_double = dy0
            - 1 as ::core::ffi::c_int as ::core::ffi::c_double
            - 2 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
        let mut dz9: ::core::ffi::c_double = dz0
            - 0 as ::core::ffi::c_int as ::core::ffi::c_double
            - 2 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
        let mut dw9: ::core::ffi::c_double = dw0
            - 1 as ::core::ffi::c_int as ::core::ffi::c_double
            - 2 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
        let mut attn9: ::core::ffi::c_double = 2 as ::core::ffi::c_int as ::core::ffi::c_double
            - dx9 * dx9
            - dy9 * dy9
            - dz9 * dz9
            - dw9 * dw9;
        if attn9 > 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            attn9 *= attn9;
            value += attn9
                * attn9
                * extrapolate4(
                    ctx,
                    xsb + 0 as ::core::ffi::c_int,
                    ysb + 1 as ::core::ffi::c_int,
                    zsb + 0 as ::core::ffi::c_int,
                    wsb + 1 as ::core::ffi::c_int,
                    dx9,
                    dy9,
                    dz9,
                    dw9,
                );
        }
        let mut dx10: ::core::ffi::c_double = dx0
            - 0 as ::core::ffi::c_int as ::core::ffi::c_double
            - 2 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
        let mut dy10: ::core::ffi::c_double = dy0
            - 0 as ::core::ffi::c_int as ::core::ffi::c_double
            - 2 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
        let mut dz10: ::core::ffi::c_double = dz0
            - 1 as ::core::ffi::c_int as ::core::ffi::c_double
            - 2 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
        let mut dw10: ::core::ffi::c_double = dw0
            - 1 as ::core::ffi::c_int as ::core::ffi::c_double
            - 2 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
        let mut attn10: ::core::ffi::c_double = 2 as ::core::ffi::c_int as ::core::ffi::c_double
            - dx10 * dx10
            - dy10 * dy10
            - dz10 * dz10
            - dw10 * dw10;
        if attn10 > 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            attn10 *= attn10;
            value += attn10
                * attn10
                * extrapolate4(
                    ctx,
                    xsb + 0 as ::core::ffi::c_int,
                    ysb + 0 as ::core::ffi::c_int,
                    zsb + 1 as ::core::ffi::c_int,
                    wsb + 1 as ::core::ffi::c_int,
                    dx10,
                    dy10,
                    dz10,
                    dw10,
                );
        }
    } else {
        let mut aScore_2: ::core::ffi::c_double = 0.;
        let mut aPoint_2: int8_t = 0;
        let mut aIsBiggerSide_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
        let mut bScore_2: ::core::ffi::c_double = 0.;
        let mut bPoint_2: int8_t = 0;
        let mut bIsBiggerSide_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
        if xins + yins < zins + wins {
            aScore_2 = xins + yins;
            aPoint_2 = 0xc as int8_t;
        } else {
            aScore_2 = zins + wins;
            aPoint_2 = 0x3 as int8_t;
        }
        if xins + zins < yins + wins {
            bScore_2 = xins + zins;
            bPoint_2 = 0xa as int8_t;
        } else {
            bScore_2 = yins + wins;
            bPoint_2 = 0x5 as int8_t;
        }
        if xins + wins < yins + zins {
            let mut score_1: ::core::ffi::c_double = xins + wins;
            if aScore_2 <= bScore_2 && score_1 < bScore_2 {
                bScore_2 = score_1;
                bPoint_2 = 0x6 as int8_t;
            } else if aScore_2 > bScore_2 && score_1 < aScore_2 {
                aScore_2 = score_1;
                aPoint_2 = 0x6 as int8_t;
            }
        } else {
            let mut score_2: ::core::ffi::c_double = yins + zins;
            if aScore_2 <= bScore_2 && score_2 < bScore_2 {
                bScore_2 = score_2;
                bPoint_2 = 0x9 as int8_t;
            } else if aScore_2 > bScore_2 && score_2 < aScore_2 {
                aScore_2 = score_2;
                aPoint_2 = 0x9 as int8_t;
            }
        }
        let mut p1_0: ::core::ffi::c_double =
            3 as ::core::ffi::c_int as ::core::ffi::c_double - inSum + xins;
        if aScore_2 <= bScore_2 && p1_0 < bScore_2 {
            bScore_2 = p1_0;
            bPoint_2 = 0xe as int8_t;
            bIsBiggerSide_0 = 0 as ::core::ffi::c_int;
        } else if aScore_2 > bScore_2 && p1_0 < aScore_2 {
            aScore_2 = p1_0;
            aPoint_2 = 0xe as int8_t;
            aIsBiggerSide_0 = 0 as ::core::ffi::c_int;
        }
        let mut p2_0: ::core::ffi::c_double =
            3 as ::core::ffi::c_int as ::core::ffi::c_double - inSum + yins;
        if aScore_2 <= bScore_2 && p2_0 < bScore_2 {
            bScore_2 = p2_0;
            bPoint_2 = 0xd as int8_t;
            bIsBiggerSide_0 = 0 as ::core::ffi::c_int;
        } else if aScore_2 > bScore_2 && p2_0 < aScore_2 {
            aScore_2 = p2_0;
            aPoint_2 = 0xd as int8_t;
            aIsBiggerSide_0 = 0 as ::core::ffi::c_int;
        }
        let mut p3_0: ::core::ffi::c_double =
            3 as ::core::ffi::c_int as ::core::ffi::c_double - inSum + zins;
        if aScore_2 <= bScore_2 && p3_0 < bScore_2 {
            bScore_2 = p3_0;
            bPoint_2 = 0xb as int8_t;
            bIsBiggerSide_0 = 0 as ::core::ffi::c_int;
        } else if aScore_2 > bScore_2 && p3_0 < aScore_2 {
            aScore_2 = p3_0;
            aPoint_2 = 0xb as int8_t;
            aIsBiggerSide_0 = 0 as ::core::ffi::c_int;
        }
        let mut p4_0: ::core::ffi::c_double =
            3 as ::core::ffi::c_int as ::core::ffi::c_double - inSum + wins;
        if aScore_2 <= bScore_2 && p4_0 < bScore_2 {
            bScore_2 = p4_0;
            bPoint_2 = 0x7 as int8_t;
            bIsBiggerSide_0 = 0 as ::core::ffi::c_int;
        } else if aScore_2 > bScore_2 && p4_0 < aScore_2 {
            aScore_2 = p4_0;
            aPoint_2 = 0x7 as int8_t;
            aIsBiggerSide_0 = 0 as ::core::ffi::c_int;
        }
        if aIsBiggerSide_0 == bIsBiggerSide_0 {
            if aIsBiggerSide_0 != 0 {
                let mut c1_1: int8_t =
                    (aPoint_2 as ::core::ffi::c_int & bPoint_2 as ::core::ffi::c_int) as int8_t;
                let mut c2_1: int8_t =
                    (aPoint_2 as ::core::ffi::c_int | bPoint_2 as ::core::ffi::c_int) as int8_t;
                xsv_ext1 = xsb;
                xsv_ext0 = xsv_ext1;
                ysv_ext1 = ysb;
                ysv_ext0 = ysv_ext1;
                zsv_ext1 = zsb;
                zsv_ext0 = zsv_ext1;
                wsv_ext1 = wsb;
                wsv_ext0 = wsv_ext1;
                dx_ext0 = dx0 - SQUISH_CONSTANT_4D;
                dy_ext0 = dy0 - SQUISH_CONSTANT_4D;
                dz_ext0 = dz0 - SQUISH_CONSTANT_4D;
                dw_ext0 = dw0 - SQUISH_CONSTANT_4D;
                dx_ext1 =
                    dx0 - 2 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
                dy_ext1 =
                    dy0 - 2 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
                dz_ext1 =
                    dz0 - 2 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
                dw_ext1 =
                    dw0 - 2 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
                if c1_1 as ::core::ffi::c_int & 0x1 as ::core::ffi::c_int != 0 as ::core::ffi::c_int
                {
                    xsv_ext0 += 1 as ::core::ffi::c_int;
                    dx_ext0 -= 1 as ::core::ffi::c_int as ::core::ffi::c_double;
                    xsv_ext1 += 2 as ::core::ffi::c_int;
                    dx_ext1 -= 2 as ::core::ffi::c_int as ::core::ffi::c_double;
                } else if c1_1 as ::core::ffi::c_int & 0x2 as ::core::ffi::c_int
                    != 0 as ::core::ffi::c_int
                {
                    ysv_ext0 += 1 as ::core::ffi::c_int;
                    dy_ext0 -= 1 as ::core::ffi::c_int as ::core::ffi::c_double;
                    ysv_ext1 += 2 as ::core::ffi::c_int;
                    dy_ext1 -= 2 as ::core::ffi::c_int as ::core::ffi::c_double;
                } else if c1_1 as ::core::ffi::c_int & 0x4 as ::core::ffi::c_int
                    != 0 as ::core::ffi::c_int
                {
                    zsv_ext0 += 1 as ::core::ffi::c_int;
                    dz_ext0 -= 1 as ::core::ffi::c_int as ::core::ffi::c_double;
                    zsv_ext1 += 2 as ::core::ffi::c_int;
                    dz_ext1 -= 2 as ::core::ffi::c_int as ::core::ffi::c_double;
                } else {
                    wsv_ext0 += 1 as ::core::ffi::c_int;
                    dw_ext0 -= 1 as ::core::ffi::c_int as ::core::ffi::c_double;
                    wsv_ext1 += 2 as ::core::ffi::c_int;
                    dw_ext1 -= 2 as ::core::ffi::c_int as ::core::ffi::c_double;
                }
                xsv_ext2 = xsb + 1 as ::core::ffi::c_int;
                ysv_ext2 = ysb + 1 as ::core::ffi::c_int;
                zsv_ext2 = zsb + 1 as ::core::ffi::c_int;
                wsv_ext2 = wsb + 1 as ::core::ffi::c_int;
                dx_ext2 = dx0
                    - 1 as ::core::ffi::c_int as ::core::ffi::c_double
                    - 2 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
                dy_ext2 = dy0
                    - 1 as ::core::ffi::c_int as ::core::ffi::c_double
                    - 2 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
                dz_ext2 = dz0
                    - 1 as ::core::ffi::c_int as ::core::ffi::c_double
                    - 2 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
                dw_ext2 = dw0
                    - 1 as ::core::ffi::c_int as ::core::ffi::c_double
                    - 2 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
                if c2_1 as ::core::ffi::c_int & 0x1 as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                {
                    xsv_ext2 -= 2 as ::core::ffi::c_int;
                    dx_ext2 += 2 as ::core::ffi::c_int as ::core::ffi::c_double;
                } else if c2_1 as ::core::ffi::c_int & 0x2 as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
                {
                    ysv_ext2 -= 2 as ::core::ffi::c_int;
                    dy_ext2 += 2 as ::core::ffi::c_int as ::core::ffi::c_double;
                } else if c2_1 as ::core::ffi::c_int & 0x4 as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
                {
                    zsv_ext2 -= 2 as ::core::ffi::c_int;
                    dz_ext2 += 2 as ::core::ffi::c_int as ::core::ffi::c_double;
                } else {
                    wsv_ext2 -= 2 as ::core::ffi::c_int;
                    dw_ext2 += 2 as ::core::ffi::c_int as ::core::ffi::c_double;
                }
            } else {
                xsv_ext2 = xsb + 1 as ::core::ffi::c_int;
                ysv_ext2 = ysb + 1 as ::core::ffi::c_int;
                zsv_ext2 = zsb + 1 as ::core::ffi::c_int;
                wsv_ext2 = wsb + 1 as ::core::ffi::c_int;
                dx_ext2 = dx0
                    - 1 as ::core::ffi::c_int as ::core::ffi::c_double
                    - 4 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
                dy_ext2 = dy0
                    - 1 as ::core::ffi::c_int as ::core::ffi::c_double
                    - 4 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
                dz_ext2 = dz0
                    - 1 as ::core::ffi::c_int as ::core::ffi::c_double
                    - 4 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
                dw_ext2 = dw0
                    - 1 as ::core::ffi::c_int as ::core::ffi::c_double
                    - 4 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
                let mut c_4: int8_t =
                    (aPoint_2 as ::core::ffi::c_int & bPoint_2 as ::core::ffi::c_int) as int8_t;
                if c_4 as ::core::ffi::c_int & 0x1 as ::core::ffi::c_int != 0 as ::core::ffi::c_int
                {
                    xsv_ext0 = xsb + 2 as ::core::ffi::c_int;
                    xsv_ext1 = xsb + 1 as ::core::ffi::c_int;
                    dx_ext0 = dx0
                        - 2 as ::core::ffi::c_int as ::core::ffi::c_double
                        - 3 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
                    dx_ext1 = dx0
                        - 1 as ::core::ffi::c_int as ::core::ffi::c_double
                        - 3 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
                } else {
                    xsv_ext1 = xsb;
                    xsv_ext0 = xsv_ext1;
                    dx_ext1 =
                        dx0 - 3 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
                    dx_ext0 = dx_ext1;
                }
                if c_4 as ::core::ffi::c_int & 0x2 as ::core::ffi::c_int != 0 as ::core::ffi::c_int
                {
                    ysv_ext1 = ysb + 1 as ::core::ffi::c_int;
                    ysv_ext0 = ysv_ext1;
                    dy_ext1 = dy0
                        - 1 as ::core::ffi::c_int as ::core::ffi::c_double
                        - 3 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
                    dy_ext0 = dy_ext1;
                    if c_4 as ::core::ffi::c_int & 0x1 as ::core::ffi::c_int
                        == 0 as ::core::ffi::c_int
                    {
                        ysv_ext0 += 1 as ::core::ffi::c_int;
                        dy_ext0 -= 1 as ::core::ffi::c_int as ::core::ffi::c_double;
                    } else {
                        ysv_ext1 += 1 as ::core::ffi::c_int;
                        dy_ext1 -= 1 as ::core::ffi::c_int as ::core::ffi::c_double;
                    }
                } else {
                    ysv_ext1 = ysb;
                    ysv_ext0 = ysv_ext1;
                    dy_ext1 =
                        dy0 - 3 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
                    dy_ext0 = dy_ext1;
                }
                if c_4 as ::core::ffi::c_int & 0x4 as ::core::ffi::c_int != 0 as ::core::ffi::c_int
                {
                    zsv_ext1 = zsb + 1 as ::core::ffi::c_int;
                    zsv_ext0 = zsv_ext1;
                    dz_ext1 = dz0
                        - 1 as ::core::ffi::c_int as ::core::ffi::c_double
                        - 3 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
                    dz_ext0 = dz_ext1;
                    if c_4 as ::core::ffi::c_int & 0x3 as ::core::ffi::c_int
                        == 0 as ::core::ffi::c_int
                    {
                        zsv_ext0 += 1 as ::core::ffi::c_int;
                        dz_ext0 -= 1 as ::core::ffi::c_int as ::core::ffi::c_double;
                    } else {
                        zsv_ext1 += 1 as ::core::ffi::c_int;
                        dz_ext1 -= 1 as ::core::ffi::c_int as ::core::ffi::c_double;
                    }
                } else {
                    zsv_ext1 = zsb;
                    zsv_ext0 = zsv_ext1;
                    dz_ext1 =
                        dz0 - 3 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
                    dz_ext0 = dz_ext1;
                }
                if c_4 as ::core::ffi::c_int & 0x8 as ::core::ffi::c_int != 0 as ::core::ffi::c_int
                {
                    wsv_ext0 = wsb + 1 as ::core::ffi::c_int;
                    wsv_ext1 = wsb + 2 as ::core::ffi::c_int;
                    dw_ext0 = dw0
                        - 1 as ::core::ffi::c_int as ::core::ffi::c_double
                        - 3 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
                    dw_ext1 = dw0
                        - 2 as ::core::ffi::c_int as ::core::ffi::c_double
                        - 3 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
                } else {
                    wsv_ext1 = wsb;
                    wsv_ext0 = wsv_ext1;
                    dw_ext1 =
                        dw0 - 3 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
                    dw_ext0 = dw_ext1;
                }
            }
        } else {
            let mut c1_2: int8_t = 0;
            let mut c2_2: int8_t = 0;
            if aIsBiggerSide_0 != 0 {
                c1_2 = aPoint_2;
                c2_2 = bPoint_2;
            } else {
                c1_2 = bPoint_2;
                c2_2 = aPoint_2;
            }
            if c1_2 as ::core::ffi::c_int & 0x1 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                xsv_ext0 = xsb + 2 as ::core::ffi::c_int;
                xsv_ext1 = xsb + 1 as ::core::ffi::c_int;
                dx_ext0 = dx0
                    - 2 as ::core::ffi::c_int as ::core::ffi::c_double
                    - 3 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
                dx_ext1 = dx0
                    - 1 as ::core::ffi::c_int as ::core::ffi::c_double
                    - 3 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
            } else {
                xsv_ext1 = xsb;
                xsv_ext0 = xsv_ext1;
                dx_ext1 =
                    dx0 - 3 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
                dx_ext0 = dx_ext1;
            }
            if c1_2 as ::core::ffi::c_int & 0x2 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                ysv_ext1 = ysb + 1 as ::core::ffi::c_int;
                ysv_ext0 = ysv_ext1;
                dy_ext1 = dy0
                    - 1 as ::core::ffi::c_int as ::core::ffi::c_double
                    - 3 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
                dy_ext0 = dy_ext1;
                if c1_2 as ::core::ffi::c_int & 0x1 as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                {
                    ysv_ext0 += 1 as ::core::ffi::c_int;
                    dy_ext0 -= 1 as ::core::ffi::c_int as ::core::ffi::c_double;
                } else {
                    ysv_ext1 += 1 as ::core::ffi::c_int;
                    dy_ext1 -= 1 as ::core::ffi::c_int as ::core::ffi::c_double;
                }
            } else {
                ysv_ext1 = ysb;
                ysv_ext0 = ysv_ext1;
                dy_ext1 =
                    dy0 - 3 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
                dy_ext0 = dy_ext1;
            }
            if c1_2 as ::core::ffi::c_int & 0x4 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                zsv_ext1 = zsb + 1 as ::core::ffi::c_int;
                zsv_ext0 = zsv_ext1;
                dz_ext1 = dz0
                    - 1 as ::core::ffi::c_int as ::core::ffi::c_double
                    - 3 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
                dz_ext0 = dz_ext1;
                if c1_2 as ::core::ffi::c_int & 0x3 as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                {
                    zsv_ext0 += 1 as ::core::ffi::c_int;
                    dz_ext0 -= 1 as ::core::ffi::c_int as ::core::ffi::c_double;
                } else {
                    zsv_ext1 += 1 as ::core::ffi::c_int;
                    dz_ext1 -= 1 as ::core::ffi::c_int as ::core::ffi::c_double;
                }
            } else {
                zsv_ext1 = zsb;
                zsv_ext0 = zsv_ext1;
                dz_ext1 =
                    dz0 - 3 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
                dz_ext0 = dz_ext1;
            }
            if c1_2 as ::core::ffi::c_int & 0x8 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                wsv_ext0 = wsb + 1 as ::core::ffi::c_int;
                wsv_ext1 = wsb + 2 as ::core::ffi::c_int;
                dw_ext0 = dw0
                    - 1 as ::core::ffi::c_int as ::core::ffi::c_double
                    - 3 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
                dw_ext1 = dw0
                    - 2 as ::core::ffi::c_int as ::core::ffi::c_double
                    - 3 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
            } else {
                wsv_ext1 = wsb;
                wsv_ext0 = wsv_ext1;
                dw_ext1 =
                    dw0 - 3 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
                dw_ext0 = dw_ext1;
            }
            xsv_ext2 = xsb + 1 as ::core::ffi::c_int;
            ysv_ext2 = ysb + 1 as ::core::ffi::c_int;
            zsv_ext2 = zsb + 1 as ::core::ffi::c_int;
            wsv_ext2 = wsb + 1 as ::core::ffi::c_int;
            dx_ext2 = dx0
                - 1 as ::core::ffi::c_int as ::core::ffi::c_double
                - 2 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
            dy_ext2 = dy0
                - 1 as ::core::ffi::c_int as ::core::ffi::c_double
                - 2 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
            dz_ext2 = dz0
                - 1 as ::core::ffi::c_int as ::core::ffi::c_double
                - 2 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
            dw_ext2 = dw0
                - 1 as ::core::ffi::c_int as ::core::ffi::c_double
                - 2 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
            if c2_2 as ::core::ffi::c_int & 0x1 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                xsv_ext2 -= 2 as ::core::ffi::c_int;
                dx_ext2 += 2 as ::core::ffi::c_int as ::core::ffi::c_double;
            } else if c2_2 as ::core::ffi::c_int & 0x2 as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
            {
                ysv_ext2 -= 2 as ::core::ffi::c_int;
                dy_ext2 += 2 as ::core::ffi::c_int as ::core::ffi::c_double;
            } else if c2_2 as ::core::ffi::c_int & 0x4 as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
            {
                zsv_ext2 -= 2 as ::core::ffi::c_int;
                dz_ext2 += 2 as ::core::ffi::c_int as ::core::ffi::c_double;
            } else {
                wsv_ext2 -= 2 as ::core::ffi::c_int;
                dw_ext2 += 2 as ::core::ffi::c_int as ::core::ffi::c_double;
            }
        }
        let mut dx4_2: ::core::ffi::c_double = dx0
            - 1 as ::core::ffi::c_int as ::core::ffi::c_double
            - 3 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
        let mut dy4_2: ::core::ffi::c_double = dy0
            - 1 as ::core::ffi::c_int as ::core::ffi::c_double
            - 3 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
        let mut dz4_2: ::core::ffi::c_double = dz0
            - 1 as ::core::ffi::c_int as ::core::ffi::c_double
            - 3 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
        let mut dw4_2: ::core::ffi::c_double =
            dw0 - 3 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
        let mut attn4_2: ::core::ffi::c_double = 2 as ::core::ffi::c_int as ::core::ffi::c_double
            - dx4_2 * dx4_2
            - dy4_2 * dy4_2
            - dz4_2 * dz4_2
            - dw4_2 * dw4_2;
        if attn4_2 > 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            attn4_2 *= attn4_2;
            value += attn4_2
                * attn4_2
                * extrapolate4(
                    ctx,
                    xsb + 1 as ::core::ffi::c_int,
                    ysb + 1 as ::core::ffi::c_int,
                    zsb + 1 as ::core::ffi::c_int,
                    wsb + 0 as ::core::ffi::c_int,
                    dx4_2,
                    dy4_2,
                    dz4_2,
                    dw4_2,
                );
        }
        let mut dx3_2: ::core::ffi::c_double = dx4_2;
        let mut dy3_2: ::core::ffi::c_double = dy4_2;
        let mut dz3_2: ::core::ffi::c_double =
            dz0 - 3 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
        let mut dw3_2: ::core::ffi::c_double = dw0
            - 1 as ::core::ffi::c_int as ::core::ffi::c_double
            - 3 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
        let mut attn3_2: ::core::ffi::c_double = 2 as ::core::ffi::c_int as ::core::ffi::c_double
            - dx3_2 * dx3_2
            - dy3_2 * dy3_2
            - dz3_2 * dz3_2
            - dw3_2 * dw3_2;
        if attn3_2 > 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            attn3_2 *= attn3_2;
            value += attn3_2
                * attn3_2
                * extrapolate4(
                    ctx,
                    xsb + 1 as ::core::ffi::c_int,
                    ysb + 1 as ::core::ffi::c_int,
                    zsb + 0 as ::core::ffi::c_int,
                    wsb + 1 as ::core::ffi::c_int,
                    dx3_2,
                    dy3_2,
                    dz3_2,
                    dw3_2,
                );
        }
        let mut dx2_2: ::core::ffi::c_double = dx4_2;
        let mut dy2_2: ::core::ffi::c_double =
            dy0 - 3 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
        let mut dz2_2: ::core::ffi::c_double = dz4_2;
        let mut dw2_2: ::core::ffi::c_double = dw3_2;
        let mut attn2_2: ::core::ffi::c_double = 2 as ::core::ffi::c_int as ::core::ffi::c_double
            - dx2_2 * dx2_2
            - dy2_2 * dy2_2
            - dz2_2 * dz2_2
            - dw2_2 * dw2_2;
        if attn2_2 > 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            attn2_2 *= attn2_2;
            value += attn2_2
                * attn2_2
                * extrapolate4(
                    ctx,
                    xsb + 1 as ::core::ffi::c_int,
                    ysb + 0 as ::core::ffi::c_int,
                    zsb + 1 as ::core::ffi::c_int,
                    wsb + 1 as ::core::ffi::c_int,
                    dx2_2,
                    dy2_2,
                    dz2_2,
                    dw2_2,
                );
        }
        let mut dx1_2: ::core::ffi::c_double =
            dx0 - 3 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
        let mut dz1_2: ::core::ffi::c_double = dz4_2;
        let mut dy1_2: ::core::ffi::c_double = dy4_2;
        let mut dw1_2: ::core::ffi::c_double = dw3_2;
        let mut attn1_2: ::core::ffi::c_double = 2 as ::core::ffi::c_int as ::core::ffi::c_double
            - dx1_2 * dx1_2
            - dy1_2 * dy1_2
            - dz1_2 * dz1_2
            - dw1_2 * dw1_2;
        if attn1_2 > 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            attn1_2 *= attn1_2;
            value += attn1_2
                * attn1_2
                * extrapolate4(
                    ctx,
                    xsb + 0 as ::core::ffi::c_int,
                    ysb + 1 as ::core::ffi::c_int,
                    zsb + 1 as ::core::ffi::c_int,
                    wsb + 1 as ::core::ffi::c_int,
                    dx1_2,
                    dy1_2,
                    dz1_2,
                    dw1_2,
                );
        }
        let mut dx5_0: ::core::ffi::c_double = dx0
            - 1 as ::core::ffi::c_int as ::core::ffi::c_double
            - 2 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
        let mut dy5_0: ::core::ffi::c_double = dy0
            - 1 as ::core::ffi::c_int as ::core::ffi::c_double
            - 2 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
        let mut dz5_0: ::core::ffi::c_double = dz0
            - 0 as ::core::ffi::c_int as ::core::ffi::c_double
            - 2 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
        let mut dw5_0: ::core::ffi::c_double = dw0
            - 0 as ::core::ffi::c_int as ::core::ffi::c_double
            - 2 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
        let mut attn5_0: ::core::ffi::c_double = 2 as ::core::ffi::c_int as ::core::ffi::c_double
            - dx5_0 * dx5_0
            - dy5_0 * dy5_0
            - dz5_0 * dz5_0
            - dw5_0 * dw5_0;
        if attn5_0 > 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            attn5_0 *= attn5_0;
            value += attn5_0
                * attn5_0
                * extrapolate4(
                    ctx,
                    xsb + 1 as ::core::ffi::c_int,
                    ysb + 1 as ::core::ffi::c_int,
                    zsb + 0 as ::core::ffi::c_int,
                    wsb + 0 as ::core::ffi::c_int,
                    dx5_0,
                    dy5_0,
                    dz5_0,
                    dw5_0,
                );
        }
        let mut dx6_0: ::core::ffi::c_double = dx0
            - 1 as ::core::ffi::c_int as ::core::ffi::c_double
            - 2 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
        let mut dy6_0: ::core::ffi::c_double = dy0
            - 0 as ::core::ffi::c_int as ::core::ffi::c_double
            - 2 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
        let mut dz6_0: ::core::ffi::c_double = dz0
            - 1 as ::core::ffi::c_int as ::core::ffi::c_double
            - 2 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
        let mut dw6_0: ::core::ffi::c_double = dw0
            - 0 as ::core::ffi::c_int as ::core::ffi::c_double
            - 2 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
        let mut attn6_0: ::core::ffi::c_double = 2 as ::core::ffi::c_int as ::core::ffi::c_double
            - dx6_0 * dx6_0
            - dy6_0 * dy6_0
            - dz6_0 * dz6_0
            - dw6_0 * dw6_0;
        if attn6_0 > 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            attn6_0 *= attn6_0;
            value += attn6_0
                * attn6_0
                * extrapolate4(
                    ctx,
                    xsb + 1 as ::core::ffi::c_int,
                    ysb + 0 as ::core::ffi::c_int,
                    zsb + 1 as ::core::ffi::c_int,
                    wsb + 0 as ::core::ffi::c_int,
                    dx6_0,
                    dy6_0,
                    dz6_0,
                    dw6_0,
                );
        }
        let mut dx7_0: ::core::ffi::c_double = dx0
            - 1 as ::core::ffi::c_int as ::core::ffi::c_double
            - 2 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
        let mut dy7_0: ::core::ffi::c_double = dy0
            - 0 as ::core::ffi::c_int as ::core::ffi::c_double
            - 2 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
        let mut dz7_0: ::core::ffi::c_double = dz0
            - 0 as ::core::ffi::c_int as ::core::ffi::c_double
            - 2 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
        let mut dw7_0: ::core::ffi::c_double = dw0
            - 1 as ::core::ffi::c_int as ::core::ffi::c_double
            - 2 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
        let mut attn7_0: ::core::ffi::c_double = 2 as ::core::ffi::c_int as ::core::ffi::c_double
            - dx7_0 * dx7_0
            - dy7_0 * dy7_0
            - dz7_0 * dz7_0
            - dw7_0 * dw7_0;
        if attn7_0 > 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            attn7_0 *= attn7_0;
            value += attn7_0
                * attn7_0
                * extrapolate4(
                    ctx,
                    xsb + 1 as ::core::ffi::c_int,
                    ysb + 0 as ::core::ffi::c_int,
                    zsb + 0 as ::core::ffi::c_int,
                    wsb + 1 as ::core::ffi::c_int,
                    dx7_0,
                    dy7_0,
                    dz7_0,
                    dw7_0,
                );
        }
        let mut dx8_0: ::core::ffi::c_double = dx0
            - 0 as ::core::ffi::c_int as ::core::ffi::c_double
            - 2 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
        let mut dy8_0: ::core::ffi::c_double = dy0
            - 1 as ::core::ffi::c_int as ::core::ffi::c_double
            - 2 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
        let mut dz8_0: ::core::ffi::c_double = dz0
            - 1 as ::core::ffi::c_int as ::core::ffi::c_double
            - 2 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
        let mut dw8_0: ::core::ffi::c_double = dw0
            - 0 as ::core::ffi::c_int as ::core::ffi::c_double
            - 2 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
        let mut attn8_0: ::core::ffi::c_double = 2 as ::core::ffi::c_int as ::core::ffi::c_double
            - dx8_0 * dx8_0
            - dy8_0 * dy8_0
            - dz8_0 * dz8_0
            - dw8_0 * dw8_0;
        if attn8_0 > 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            attn8_0 *= attn8_0;
            value += attn8_0
                * attn8_0
                * extrapolate4(
                    ctx,
                    xsb + 0 as ::core::ffi::c_int,
                    ysb + 1 as ::core::ffi::c_int,
                    zsb + 1 as ::core::ffi::c_int,
                    wsb + 0 as ::core::ffi::c_int,
                    dx8_0,
                    dy8_0,
                    dz8_0,
                    dw8_0,
                );
        }
        let mut dx9_0: ::core::ffi::c_double = dx0
            - 0 as ::core::ffi::c_int as ::core::ffi::c_double
            - 2 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
        let mut dy9_0: ::core::ffi::c_double = dy0
            - 1 as ::core::ffi::c_int as ::core::ffi::c_double
            - 2 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
        let mut dz9_0: ::core::ffi::c_double = dz0
            - 0 as ::core::ffi::c_int as ::core::ffi::c_double
            - 2 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
        let mut dw9_0: ::core::ffi::c_double = dw0
            - 1 as ::core::ffi::c_int as ::core::ffi::c_double
            - 2 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
        let mut attn9_0: ::core::ffi::c_double = 2 as ::core::ffi::c_int as ::core::ffi::c_double
            - dx9_0 * dx9_0
            - dy9_0 * dy9_0
            - dz9_0 * dz9_0
            - dw9_0 * dw9_0;
        if attn9_0 > 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            attn9_0 *= attn9_0;
            value += attn9_0
                * attn9_0
                * extrapolate4(
                    ctx,
                    xsb + 0 as ::core::ffi::c_int,
                    ysb + 1 as ::core::ffi::c_int,
                    zsb + 0 as ::core::ffi::c_int,
                    wsb + 1 as ::core::ffi::c_int,
                    dx9_0,
                    dy9_0,
                    dz9_0,
                    dw9_0,
                );
        }
        let mut dx10_0: ::core::ffi::c_double = dx0
            - 0 as ::core::ffi::c_int as ::core::ffi::c_double
            - 2 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
        let mut dy10_0: ::core::ffi::c_double = dy0
            - 0 as ::core::ffi::c_int as ::core::ffi::c_double
            - 2 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
        let mut dz10_0: ::core::ffi::c_double = dz0
            - 1 as ::core::ffi::c_int as ::core::ffi::c_double
            - 2 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
        let mut dw10_0: ::core::ffi::c_double = dw0
            - 1 as ::core::ffi::c_int as ::core::ffi::c_double
            - 2 as ::core::ffi::c_int as ::core::ffi::c_double * SQUISH_CONSTANT_4D;
        let mut attn10_0: ::core::ffi::c_double = 2 as ::core::ffi::c_int as ::core::ffi::c_double
            - dx10_0 * dx10_0
            - dy10_0 * dy10_0
            - dz10_0 * dz10_0
            - dw10_0 * dw10_0;
        if attn10_0 > 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            attn10_0 *= attn10_0;
            value += attn10_0
                * attn10_0
                * extrapolate4(
                    ctx,
                    xsb + 0 as ::core::ffi::c_int,
                    ysb + 0 as ::core::ffi::c_int,
                    zsb + 1 as ::core::ffi::c_int,
                    wsb + 1 as ::core::ffi::c_int,
                    dx10_0,
                    dy10_0,
                    dz10_0,
                    dw10_0,
                );
        }
    }
    let mut attn_ext0: ::core::ffi::c_double = 2 as ::core::ffi::c_int as ::core::ffi::c_double
        - dx_ext0 * dx_ext0
        - dy_ext0 * dy_ext0
        - dz_ext0 * dz_ext0
        - dw_ext0 * dw_ext0;
    if attn_ext0 > 0 as ::core::ffi::c_int as ::core::ffi::c_double {
        attn_ext0 *= attn_ext0;
        value += attn_ext0
            * attn_ext0
            * extrapolate4(
                ctx, xsv_ext0, ysv_ext0, zsv_ext0, wsv_ext0, dx_ext0, dy_ext0, dz_ext0, dw_ext0,
            );
    }
    let mut attn_ext1: ::core::ffi::c_double = 2 as ::core::ffi::c_int as ::core::ffi::c_double
        - dx_ext1 * dx_ext1
        - dy_ext1 * dy_ext1
        - dz_ext1 * dz_ext1
        - dw_ext1 * dw_ext1;
    if attn_ext1 > 0 as ::core::ffi::c_int as ::core::ffi::c_double {
        attn_ext1 *= attn_ext1;
        value += attn_ext1
            * attn_ext1
            * extrapolate4(
                ctx, xsv_ext1, ysv_ext1, zsv_ext1, wsv_ext1, dx_ext1, dy_ext1, dz_ext1, dw_ext1,
            );
    }
    let mut attn_ext2: ::core::ffi::c_double = 2 as ::core::ffi::c_int as ::core::ffi::c_double
        - dx_ext2 * dx_ext2
        - dy_ext2 * dy_ext2
        - dz_ext2 * dz_ext2
        - dw_ext2 * dw_ext2;
    if attn_ext2 > 0 as ::core::ffi::c_int as ::core::ffi::c_double {
        attn_ext2 *= attn_ext2;
        value += attn_ext2
            * attn_ext2
            * extrapolate4(
                ctx, xsv_ext2, ysv_ext2, zsv_ext2, wsv_ext2, dx_ext2, dy_ext2, dz_ext2, dw_ext2,
            );
    }
    return value / NORM_CONSTANT_4D;
}
pub const ENOMEM: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
