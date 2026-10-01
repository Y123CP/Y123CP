extern "C" {
    fn abs(__x: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn fabs(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn kmAlmostEqual(lhs: ::core::ffi::c_float, rhs: ::core::ffi::c_float) -> ::core::ffi::c_uchar;
    fn kmVec3Fill(
        pOut: *mut kmVec3,
        x: ::core::ffi::c_float,
        y: ::core::ffi::c_float,
        z: ::core::ffi::c_float,
    ) -> *mut kmVec3;
    fn kmVec3Length(pIn: *const kmVec3) -> ::core::ffi::c_float;
    fn kmVec3Normalize(pOut: *mut kmVec3, pIn: *const kmVec3) -> *mut kmVec3;
    fn kmVec3Cross(pOut: *mut kmVec3, pV1: *const kmVec3, pV2: *const kmVec3) -> *mut kmVec3;
    fn kmVec3Dot(pV1: *const kmVec3, pV2: *const kmVec3) -> ::core::ffi::c_float;
    fn kmVec3Subtract(pOut: *mut kmVec3, pV1: *const kmVec3, pV2: *const kmVec3) -> *mut kmVec3;
    fn kmVec3Scale(pOut: *mut kmVec3, pIn: *const kmVec3, s: ::core::ffi::c_float) -> *mut kmVec3;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kmMat4 {
    pub mat: [::core::ffi::c_float; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kmPlane {
    pub a: ::core::ffi::c_float,
    pub b: ::core::ffi::c_float,
    pub c: ::core::ffi::c_float,
    pub d: ::core::ffi::c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kmVec3 {
    pub x: ::core::ffi::c_float,
    pub y: ::core::ffi::c_float,
    pub z: ::core::ffi::c_float,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct kmVec4 {
    pub x: ::core::ffi::c_float,
    pub y: ::core::ffi::c_float,
    pub z: ::core::ffi::c_float,
    pub w: ::core::ffi::c_float,
}
pub type KM_POINT_CLASSIFICATION = ::core::ffi::c_int;
pub const POINT_INFRONT_OF_PLANE: KM_POINT_CLASSIFICATION = 1;
pub const POINT_ON_PLANE: KM_POINT_CLASSIFICATION = 0;
pub const POINT_BEHIND_PLANE: KM_POINT_CLASSIFICATION = -1;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const kmEpsilon: ::core::ffi::c_double = 0.0001f64;
#[no_mangle]
pub unsafe extern "C" fn kmPlaneDot(
    mut pP: *const kmPlane,
    mut pV: *const kmVec4,
) -> ::core::ffi::c_float {
    return (*pP).a * (*pV).x + (*pP).b * (*pV).y + (*pP).c * (*pV).z + (*pP).d * (*pV).w;
}
#[no_mangle]
pub unsafe extern "C" fn kmPlaneDotCoord(
    mut pP: *const kmPlane,
    mut pV: *const kmVec3,
) -> ::core::ffi::c_float {
    return (*pP).a * (*pV).x + (*pP).b * (*pV).y + (*pP).c * (*pV).z + (*pP).d;
}
#[no_mangle]
pub unsafe extern "C" fn kmPlaneDotNormal(
    mut pP: *const kmPlane,
    mut pV: *const kmVec3,
) -> ::core::ffi::c_float {
    return (*pP).a * (*pV).x + (*pP).b * (*pV).y + (*pP).c * (*pV).z;
}
#[no_mangle]
pub unsafe extern "C" fn kmPlaneFromNormalAndDistance(
    mut plane: *mut kmPlane,
    mut normal: *const kmVec3,
    dist: ::core::ffi::c_float,
) -> *mut kmPlane {
    (*plane).a = (*normal).x;
    (*plane).b = (*normal).y;
    (*plane).c = (*normal).z;
    (*plane).d = dist;
    return plane;
}
#[no_mangle]
pub unsafe extern "C" fn kmPlaneFromPointAndNormal(
    mut pOut: *mut kmPlane,
    mut pPoint: *const kmVec3,
    mut pNormal: *const kmVec3,
) -> *mut kmPlane {
    (*pOut).a = (*pNormal).x;
    (*pOut).b = (*pNormal).y;
    (*pOut).c = (*pNormal).z;
    (*pOut).d = -kmVec3Dot(pNormal, pPoint);
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmPlaneFromPoints(
    mut pOut: *mut kmPlane,
    mut p1: *const kmVec3,
    mut p2: *const kmVec3,
    mut p3: *const kmVec3,
) -> *mut kmPlane {
    let mut n: kmVec3 = kmVec3 {
        x: 0.,
        y: 0.,
        z: 0.,
    };
    let mut v1: kmVec3 = kmVec3 {
        x: 0.,
        y: 0.,
        z: 0.,
    };
    let mut v2: kmVec3 = kmVec3 {
        x: 0.,
        y: 0.,
        z: 0.,
    };
    kmVec3Subtract(&raw mut v1, p2, p1);
    kmVec3Subtract(&raw mut v2, p3, p1);
    kmVec3Cross(&raw mut n, &raw mut v1, &raw mut v2);
    kmVec3Normalize(&raw mut n, &raw mut n);
    (*pOut).a = n.x;
    (*pOut).b = n.y;
    (*pOut).c = n.z;
    (*pOut).d = kmVec3Dot(
        kmVec3Scale(&raw mut n, &raw mut n, -1.0f64 as ::core::ffi::c_float),
        p1,
    );
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmPlaneIntersectLine(
    mut pOut: *mut kmVec3,
    mut pP: *const kmPlane,
    mut pV1: *const kmVec3,
    mut pV2: *const kmVec3,
) -> *mut kmVec3 {
    let mut d: kmVec3 = kmVec3 {
        x: 0.,
        y: 0.,
        z: 0.,
    };
    kmVec3Subtract(&raw mut d, pV2, pV1);
    let mut n: kmVec3 = kmVec3 {
        x: 0.,
        y: 0.,
        z: 0.,
    };
    n.x = (*pP).a;
    n.y = (*pP).b;
    n.z = (*pP).c;
    kmVec3Normalize(&raw mut n, &raw mut n);
    let mut nt: ::core::ffi::c_float =
        -(n.x * (*pV1).x + n.y * (*pV1).y + n.z * (*pV1).z + (*pP).d);
    let mut dt: ::core::ffi::c_float = n.x * d.x + n.y * d.y + n.z * d.z;
    if fabs(dt as ::core::ffi::c_double) < kmEpsilon {
        pOut = ::core::ptr::null_mut::<kmVec3>();
        return pOut as *mut kmVec3;
    }
    let mut t: ::core::ffi::c_float = nt / dt;
    (*pOut).x = (*pV1).x + d.x * t;
    (*pOut).y = (*pV1).y + d.y * t;
    (*pOut).z = (*pV1).z + d.z * t;
    return pOut as *mut kmVec3;
}
#[no_mangle]
pub unsafe extern "C" fn kmPlaneNormalize(
    mut pOut: *mut kmPlane,
    mut pP: *const kmPlane,
) -> *mut kmPlane {
    let mut n: kmVec3 = kmVec3 {
        x: 0.,
        y: 0.,
        z: 0.,
    };
    let mut l: ::core::ffi::c_float = 0 as ::core::ffi::c_int as ::core::ffi::c_float;
    if (*pP).a == 0. && (*pP).b == 0. && (*pP).c == 0. {
        (*pOut).a = (*pP).a;
        (*pOut).b = (*pP).b;
        (*pOut).c = (*pP).c;
        (*pOut).d = (*pP).d;
        return pOut;
    }
    n.x = (*pP).a;
    n.y = (*pP).b;
    n.z = (*pP).c;
    l = 1.0f32 / kmVec3Length(&raw mut n);
    kmVec3Normalize(&raw mut n, &raw mut n);
    (*pOut).a = n.x;
    (*pOut).b = n.y;
    (*pOut).c = n.z;
    (*pOut).d = (*pP).d * l;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmPlaneScale(
    mut pOut: *mut kmPlane,
    mut pP: *const kmPlane,
    mut s: ::core::ffi::c_float,
) -> *mut kmPlane {
    return ::core::ptr::null_mut::<kmPlane>();
}
#[no_mangle]
pub unsafe extern "C" fn kmPlaneClassifyPoint(
    mut pIn: *const kmPlane,
    mut pP: *const kmVec3,
) -> KM_POINT_CLASSIFICATION {
    let mut distance: ::core::ffi::c_float =
        (*pIn).a * (*pP).x + (*pIn).b * (*pP).y + (*pIn).c * (*pP).z + (*pIn).d;
    if distance as ::core::ffi::c_double > kmEpsilon {
        return POINT_INFRONT_OF_PLANE;
    }
    if (distance as ::core::ffi::c_double) < -kmEpsilon {
        return POINT_BEHIND_PLANE;
    }
    return POINT_ON_PLANE;
}
#[no_mangle]
pub unsafe extern "C" fn kmPlaneExtractFromMat4(
    mut pOut: *mut kmPlane,
    mut pIn: *const kmMat4,
    mut row: ::core::ffi::c_int,
) -> *mut kmPlane {
    let mut scale: ::core::ffi::c_int = if row < 0 as ::core::ffi::c_int {
        -(1 as ::core::ffi::c_int)
    } else {
        1 as ::core::ffi::c_int
    };
    row = abs(row) - 1 as ::core::ffi::c_int;
    (*pOut).a = (*pIn).mat[3 as ::core::ffi::c_int as usize]
        + scale as ::core::ffi::c_float * (*pIn).mat[row as usize];
    (*pOut).b = (*pIn).mat[7 as ::core::ffi::c_int as usize]
        + scale as ::core::ffi::c_float * (*pIn).mat[(row + 4 as ::core::ffi::c_int) as usize];
    (*pOut).c = (*pIn).mat[11 as ::core::ffi::c_int as usize]
        + scale as ::core::ffi::c_float * (*pIn).mat[(row + 8 as ::core::ffi::c_int) as usize];
    (*pOut).d = (*pIn).mat[15 as ::core::ffi::c_int as usize]
        + scale as ::core::ffi::c_float * (*pIn).mat[(row + 12 as ::core::ffi::c_int) as usize];
    return kmPlaneNormalize(pOut, pOut);
}
#[no_mangle]
pub unsafe extern "C" fn kmPlaneGetIntersection(
    mut pOut: *mut kmVec3,
    mut p1: *const kmPlane,
    mut p2: *const kmPlane,
    mut p3: *const kmPlane,
) -> *mut kmVec3 {
    let mut n1: kmVec3 = kmVec3 {
        x: 0.,
        y: 0.,
        z: 0.,
    };
    let mut n2: kmVec3 = kmVec3 {
        x: 0.,
        y: 0.,
        z: 0.,
    };
    let mut n3: kmVec3 = kmVec3 {
        x: 0.,
        y: 0.,
        z: 0.,
    };
    let mut cross: kmVec3 = kmVec3 {
        x: 0.,
        y: 0.,
        z: 0.,
    };
    let mut r1: kmVec3 = kmVec3 {
        x: 0.,
        y: 0.,
        z: 0.,
    };
    let mut r2: kmVec3 = kmVec3 {
        x: 0.,
        y: 0.,
        z: 0.,
    };
    let mut r3: kmVec3 = kmVec3 {
        x: 0.,
        y: 0.,
        z: 0.,
    };
    let mut denom: ::core::ffi::c_double = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    kmVec3Fill(&raw mut n1, (*p1).a, (*p1).b, (*p1).c);
    kmVec3Fill(&raw mut n2, (*p2).a, (*p2).b, (*p2).c);
    kmVec3Fill(&raw mut n3, (*p3).a, (*p3).b, (*p3).c);
    kmVec3Cross(&raw mut cross, &raw mut n2, &raw mut n3);
    denom = kmVec3Dot(&raw mut n1, &raw mut cross) as ::core::ffi::c_double;
    if kmAlmostEqual(denom as ::core::ffi::c_float, 0.0f32) != 0 {
        return ::core::ptr::null_mut::<kmVec3>();
    }
    kmVec3Cross(&raw mut r1, &raw mut n2, &raw mut n3);
    kmVec3Cross(&raw mut r2, &raw mut n3, &raw mut n1);
    kmVec3Cross(&raw mut r3, &raw mut n1, &raw mut n2);
    kmVec3Scale(&raw mut r1, &raw mut r1, -(*p1).d);
    kmVec3Scale(&raw mut r2, &raw mut r2, (*p2).d);
    kmVec3Scale(&raw mut r3, &raw mut r3, (*p3).d);
    kmVec3Subtract(pOut, &raw mut r1, &raw mut r2);
    kmVec3Subtract(pOut, pOut, &raw mut r3);
    kmVec3Scale(pOut, pOut, (1.0f64 / denom) as ::core::ffi::c_float);
    return pOut as *mut kmVec3;
}
#[no_mangle]
pub unsafe extern "C" fn kmPlaneFill(
    mut plane: *mut kmPlane,
    mut a: ::core::ffi::c_float,
    mut b: ::core::ffi::c_float,
    mut c: ::core::ffi::c_float,
    mut d: ::core::ffi::c_float,
) -> *mut kmPlane {
    (*plane).a = a;
    (*plane).b = b;
    (*plane).c = c;
    (*plane).d = d;
    return plane;
}
