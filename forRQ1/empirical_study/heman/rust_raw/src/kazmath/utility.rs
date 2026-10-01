#[no_mangle]
pub unsafe extern "C" fn kmSQR(mut s: ::core::ffi::c_float) -> ::core::ffi::c_float {
    return s * s;
}
#[no_mangle]
pub unsafe extern "C" fn kmDegreesToRadians(
    mut degrees: ::core::ffi::c_float,
) -> ::core::ffi::c_float {
    return degrees * kmPIOver180;
}
#[no_mangle]
pub unsafe extern "C" fn kmRadiansToDegrees(
    mut radians: ::core::ffi::c_float,
) -> ::core::ffi::c_float {
    return (radians as ::core::ffi::c_double * kmPIUnder180) as ::core::ffi::c_float;
}
#[no_mangle]
pub unsafe extern "C" fn kmMin(
    mut lhs: ::core::ffi::c_float,
    mut rhs: ::core::ffi::c_float,
) -> ::core::ffi::c_float {
    return if lhs < rhs { lhs } else { rhs };
}
#[no_mangle]
pub unsafe extern "C" fn kmMax(
    mut lhs: ::core::ffi::c_float,
    mut rhs: ::core::ffi::c_float,
) -> ::core::ffi::c_float {
    return if lhs > rhs { lhs } else { rhs };
}
#[no_mangle]
pub unsafe extern "C" fn kmAlmostEqual(
    mut lhs: ::core::ffi::c_float,
    mut rhs: ::core::ffi::c_float,
) -> ::core::ffi::c_uchar {
    return (lhs as ::core::ffi::c_double + kmEpsilon > rhs as ::core::ffi::c_double
        && lhs as ::core::ffi::c_double - kmEpsilon < rhs as ::core::ffi::c_double)
        as ::core::ffi::c_int as ::core::ffi::c_uchar;
}
#[no_mangle]
pub unsafe extern "C" fn kmClamp(
    mut x: ::core::ffi::c_float,
    mut min: ::core::ffi::c_float,
    mut max: ::core::ffi::c_float,
) -> ::core::ffi::c_float {
    return if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    };
}
#[no_mangle]
pub unsafe extern "C" fn kmLerp(
    mut x: ::core::ffi::c_float,
    mut y: ::core::ffi::c_float,
    mut t: ::core::ffi::c_float,
) -> ::core::ffi::c_float {
    return x + t * (y - x);
}
pub const kmPI: ::core::ffi::c_float = 3.14159265358979323846f32;
pub const kmPIOver180: ::core::ffi::c_float = kmPI / 180.0f32;
pub const kmPIUnder180: ::core::ffi::c_double = 180.0f64 / kmPI as ::core::ffi::c_double;
pub const kmEpsilon: ::core::ffi::c_double = 0.0001f64;
