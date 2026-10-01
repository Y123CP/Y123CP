extern "C" {
    fn atan2(__y: ::core::ffi::c_double, __x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn fabs(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn cosf(__x: ::core::ffi::c_float) -> ::core::ffi::c_float;
    fn sinf(__x: ::core::ffi::c_float) -> ::core::ffi::c_float;
    fn sqrtf(__x: ::core::ffi::c_float) -> ::core::ffi::c_float;
    fn kmSQR(s: ::core::ffi::c_float) -> ::core::ffi::c_float;
    fn kmDegreesToRadians(degrees: ::core::ffi::c_float) -> ::core::ffi::c_float;
    fn kmRadiansToDegrees(radians: ::core::ffi::c_float) -> ::core::ffi::c_float;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kmMat3 {
    pub mat: [::core::ffi::c_float; 9],
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct kmVec2 {
    pub x: ::core::ffi::c_float,
    pub y: ::core::ffi::c_float,
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const kmEpsilon: ::core::ffi::c_double = 0.0001f64;
#[no_mangle]
pub static mut KM_VEC2_POS_Y: kmVec2 = kmVec2 {
    x: 0 as ::core::ffi::c_int as ::core::ffi::c_float,
    y: 1 as ::core::ffi::c_int as ::core::ffi::c_float,
};
#[no_mangle]
pub static mut KM_VEC2_NEG_Y: kmVec2 = kmVec2 {
    x: 0 as ::core::ffi::c_int as ::core::ffi::c_float,
    y: -(1 as ::core::ffi::c_int) as ::core::ffi::c_float,
};
#[no_mangle]
pub static mut KM_VEC2_NEG_X: kmVec2 = kmVec2 {
    x: -(1 as ::core::ffi::c_int) as ::core::ffi::c_float,
    y: 0 as ::core::ffi::c_int as ::core::ffi::c_float,
};
#[no_mangle]
pub static mut KM_VEC2_POS_X: kmVec2 = kmVec2 {
    x: 1 as ::core::ffi::c_int as ::core::ffi::c_float,
    y: 0 as ::core::ffi::c_int as ::core::ffi::c_float,
};
#[no_mangle]
pub static mut KM_VEC2_ZERO: kmVec2 = kmVec2 {
    x: 0 as ::core::ffi::c_int as ::core::ffi::c_float,
    y: 0 as ::core::ffi::c_int as ::core::ffi::c_float,
};
#[no_mangle]
pub unsafe extern "C" fn kmVec2Fill(
    mut pOut: *mut kmVec2,
    mut x: ::core::ffi::c_float,
    mut y: ::core::ffi::c_float,
) -> *mut kmVec2 {
    (*pOut).x = x;
    (*pOut).y = y;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmVec2Length(mut pIn: *const kmVec2) -> ::core::ffi::c_float {
    return sqrtf(kmSQR((*pIn).x) + kmSQR((*pIn).y));
}
#[no_mangle]
pub unsafe extern "C" fn kmVec2LengthSq(mut pIn: *const kmVec2) -> ::core::ffi::c_float {
    return kmSQR((*pIn).x) + kmSQR((*pIn).y);
}
#[no_mangle]
pub unsafe extern "C" fn kmVec2Lerp(
    mut pOut: *mut kmVec2,
    mut pV1: *const kmVec2,
    mut pV2: *const kmVec2,
    mut t: ::core::ffi::c_float,
) -> *mut kmVec2 {
    (*pOut).x = (*pV1).x + t * ((*pV2).x - (*pV1).x);
    (*pOut).y = (*pV1).y + t * ((*pV2).y - (*pV1).y);
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmVec2Normalize(
    mut pOut: *mut kmVec2,
    mut pIn: *const kmVec2,
) -> *mut kmVec2 {
    if (*pIn).x == 0. && (*pIn).y == 0. {
        return kmVec2Assign(pOut, pIn);
    }
    let mut l: ::core::ffi::c_float = 1.0f32 / kmVec2Length(pIn);
    let mut v: kmVec2 = kmVec2 { x: 0., y: 0. };
    v.x = (*pIn).x * l;
    v.y = (*pIn).y * l;
    (*pOut).x = v.x;
    (*pOut).y = v.y;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmVec2Add(
    mut pOut: *mut kmVec2,
    mut pV1: *const kmVec2,
    mut pV2: *const kmVec2,
) -> *mut kmVec2 {
    (*pOut).x = (*pV1).x + (*pV2).x;
    (*pOut).y = (*pV1).y + (*pV2).y;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmVec2Dot(
    mut pV1: *const kmVec2,
    mut pV2: *const kmVec2,
) -> ::core::ffi::c_float {
    return (*pV1).x * (*pV2).x + (*pV1).y * (*pV2).y;
}
#[no_mangle]
pub unsafe extern "C" fn kmVec2Cross(
    mut pV1: *const kmVec2,
    mut pV2: *const kmVec2,
) -> ::core::ffi::c_float {
    return (*pV1).x * (*pV2).y - (*pV1).y * (*pV2).x;
}
#[no_mangle]
pub unsafe extern "C" fn kmVec2Subtract(
    mut pOut: *mut kmVec2,
    mut pV1: *const kmVec2,
    mut pV2: *const kmVec2,
) -> *mut kmVec2 {
    (*pOut).x = (*pV1).x - (*pV2).x;
    (*pOut).y = (*pV1).y - (*pV2).y;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmVec2Mul(
    mut pOut: *mut kmVec2,
    mut pV1: *const kmVec2,
    mut pV2: *const kmVec2,
) -> *mut kmVec2 {
    (*pOut).x = (*pV1).x * (*pV2).x;
    (*pOut).y = (*pV1).y * (*pV2).y;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmVec2Div(
    mut pOut: *mut kmVec2,
    mut pV1: *const kmVec2,
    mut pV2: *const kmVec2,
) -> *mut kmVec2 {
    if (*pV2).x != 0. && (*pV2).y != 0. {
        (*pOut).x = (*pV1).x / (*pV2).x;
        (*pOut).y = (*pV1).y / (*pV2).y;
    }
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmVec2Transform(
    mut pOut: *mut kmVec2,
    mut pV: *const kmVec2,
    mut pM: *const kmMat3,
) -> *mut kmVec2 {
    let mut v: kmVec2 = kmVec2 { x: 0., y: 0. };
    v.x = (*pV).x * (*pM).mat[0 as ::core::ffi::c_int as usize]
        + (*pV).y * (*pM).mat[3 as ::core::ffi::c_int as usize]
        + (*pM).mat[6 as ::core::ffi::c_int as usize];
    v.y = (*pV).x * (*pM).mat[1 as ::core::ffi::c_int as usize]
        + (*pV).y * (*pM).mat[4 as ::core::ffi::c_int as usize]
        + (*pM).mat[7 as ::core::ffi::c_int as usize];
    (*pOut).x = v.x;
    (*pOut).y = v.y;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmVec2TransformCoord(
    mut pOut: *mut kmVec2,
    mut pV: *const kmVec2,
    mut pM: *const kmMat3,
) -> *mut kmVec2 {
    return ::core::ptr::null_mut::<kmVec2>();
}
#[no_mangle]
pub unsafe extern "C" fn kmVec2Scale(
    mut pOut: *mut kmVec2,
    mut pIn: *const kmVec2,
    s: ::core::ffi::c_float,
) -> *mut kmVec2 {
    (*pOut).x = (*pIn).x * s;
    (*pOut).y = (*pIn).y * s;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmVec2AreEqual(
    mut p1: *const kmVec2,
    mut p2: *const kmVec2,
) -> ::core::ffi::c_int {
    return (((*p1).x as ::core::ffi::c_double) < (*p2).x as ::core::ffi::c_double + kmEpsilon
        && (*p1).x as ::core::ffi::c_double > (*p2).x as ::core::ffi::c_double - kmEpsilon
        && (((*p1).y as ::core::ffi::c_double) < (*p2).y as ::core::ffi::c_double + kmEpsilon
            && (*p1).y as ::core::ffi::c_double > (*p2).y as ::core::ffi::c_double - kmEpsilon))
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn kmVec2Assign(
    mut pOut: *mut kmVec2,
    mut pIn: *const kmVec2,
) -> *mut kmVec2 {
    if pOut == pIn as *mut kmVec2 {
        return pOut;
    }
    (*pOut).x = (*pIn).x;
    (*pOut).y = (*pIn).y;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmVec2RotateBy(
    mut pOut: *mut kmVec2,
    mut pIn: *const kmVec2,
    degrees: ::core::ffi::c_float,
    mut center: *const kmVec2,
) -> *mut kmVec2 {
    let mut x: ::core::ffi::c_float = 0.;
    let mut y: ::core::ffi::c_float = 0.;
    let radians: ::core::ffi::c_float = kmDegreesToRadians(degrees) as ::core::ffi::c_float;
    let cs: ::core::ffi::c_float = cosf(radians) as ::core::ffi::c_float;
    let sn: ::core::ffi::c_float = sinf(radians) as ::core::ffi::c_float;
    (*pOut).x = (*pIn).x - (*center).x;
    (*pOut).y = (*pIn).y - (*center).y;
    x = (*pOut).x * cs - (*pOut).y * sn;
    y = (*pOut).x * sn + (*pOut).y * cs;
    (*pOut).x = x + (*center).x;
    (*pOut).y = y + (*center).y;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmVec2DegreesBetween(
    mut v1: *const kmVec2,
    mut v2: *const kmVec2,
) -> ::core::ffi::c_float {
    if kmVec2AreEqual(v1, v2) != 0 {
        return 0.0f32;
    }
    let mut t1: kmVec2 = kmVec2 { x: 0., y: 0. };
    let mut t2: kmVec2 = kmVec2 { x: 0., y: 0. };
    kmVec2Normalize(&raw mut t1, v1);
    kmVec2Normalize(&raw mut t2, v2);
    let mut cross: ::core::ffi::c_float = kmVec2Cross(&raw mut t1, &raw mut t2);
    let mut dot: ::core::ffi::c_float = kmVec2Dot(&raw mut t1, &raw mut t2);
    if dot as ::core::ffi::c_double > 1.0f64 {
        dot = 1.0f32;
    }
    if (dot as ::core::ffi::c_double) < -1.0f64 {
        dot = -1.0f64 as ::core::ffi::c_float;
    }
    return kmRadiansToDegrees(
        atan2(cross as ::core::ffi::c_double, dot as ::core::ffi::c_double) as ::core::ffi::c_float,
    );
}
#[no_mangle]
pub unsafe extern "C" fn kmVec2DistanceBetween(
    mut v1: *const kmVec2,
    mut v2: *const kmVec2,
) -> ::core::ffi::c_float {
    let mut diff: kmVec2 = kmVec2 { x: 0., y: 0. };
    kmVec2Subtract(&raw mut diff, v2, v1);
    return fabs(kmVec2Length(&raw mut diff) as ::core::ffi::c_double) as ::core::ffi::c_float;
}
#[no_mangle]
pub unsafe extern "C" fn kmVec2MidPointBetween(
    mut pOut: *mut kmVec2,
    mut v1: *const kmVec2,
    mut v2: *const kmVec2,
) -> *mut kmVec2 {
    let mut sum: kmVec2 = kmVec2 { x: 0., y: 0. };
    kmVec2Add(&raw mut sum, v1, v2);
    (*pOut).x = sum.x / 2.0f32;
    (*pOut).y = sum.y / 2.0f32;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmVec2Reflect(
    mut pOut: *mut kmVec2,
    mut pIn: *const kmVec2,
    mut normal: *const kmVec2,
) -> *mut kmVec2 {
    let mut tmp: kmVec2 = kmVec2 { x: 0., y: 0. };
    kmVec2Scale(&raw mut tmp, normal, 2.0f32 * kmVec2Dot(pIn, normal));
    kmVec2Subtract(pOut, pIn, &raw mut tmp);
    return pOut;
}
