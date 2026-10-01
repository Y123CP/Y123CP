extern "C" {
    fn atan2(__y: ::core::ffi::c_double, __x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn cos(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn sin(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn sqrt(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn sqrtf(__x: ::core::ffi::c_float) -> ::core::ffi::c_float;
    fn kmSQR(s: ::core::ffi::c_float) -> ::core::ffi::c_float;
    fn kmDegreesToRadians(degrees: ::core::ffi::c_float) -> ::core::ffi::c_float;
    fn kmRadiansToDegrees(radians: ::core::ffi::c_float) -> ::core::ffi::c_float;
    fn kmVec4Fill(
        pOut: *mut kmVec4,
        x: ::core::ffi::c_float,
        y: ::core::ffi::c_float,
        z: ::core::ffi::c_float,
        w: ::core::ffi::c_float,
    ) -> *mut kmVec4;
    fn kmVec4Transform(pOut: *mut kmVec4, pV: *const kmVec4, pM: *const kmMat4) -> *mut kmVec4;
    fn kmRay3IntersectPlane(
        pOut: *mut kmVec3,
        ray: *const kmRay3,
        plane: *const kmPlane,
    ) -> ::core::ffi::c_uchar;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kmMat4 {
    pub mat: [::core::ffi::c_float; 16],
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct kmVec4 {
    pub x: ::core::ffi::c_float,
    pub y: ::core::ffi::c_float,
    pub z: ::core::ffi::c_float,
    pub w: ::core::ffi::c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kmVec3 {
    pub x: ::core::ffi::c_float,
    pub y: ::core::ffi::c_float,
    pub z: ::core::ffi::c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kmMat3 {
    pub mat: [::core::ffi::c_float; 9],
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
pub struct kmRay3 {
    pub start: kmVec3,
    pub dir: kmVec3,
}
pub const kmEpsilon: ::core::ffi::c_double = 0.0001f64;
#[no_mangle]
pub static mut KM_VEC3_POS_Z: kmVec3 = kmVec3 {
    x: 0 as ::core::ffi::c_int as ::core::ffi::c_float,
    y: 0 as ::core::ffi::c_int as ::core::ffi::c_float,
    z: 1 as ::core::ffi::c_int as ::core::ffi::c_float,
};
#[no_mangle]
pub static mut KM_VEC3_NEG_Z: kmVec3 = kmVec3 {
    x: 0 as ::core::ffi::c_int as ::core::ffi::c_float,
    y: 0 as ::core::ffi::c_int as ::core::ffi::c_float,
    z: -(1 as ::core::ffi::c_int) as ::core::ffi::c_float,
};
#[no_mangle]
pub static mut KM_VEC3_POS_Y: kmVec3 = kmVec3 {
    x: 0 as ::core::ffi::c_int as ::core::ffi::c_float,
    y: 1 as ::core::ffi::c_int as ::core::ffi::c_float,
    z: 0 as ::core::ffi::c_int as ::core::ffi::c_float,
};
#[no_mangle]
pub static mut KM_VEC3_NEG_Y: kmVec3 = kmVec3 {
    x: 0 as ::core::ffi::c_int as ::core::ffi::c_float,
    y: -(1 as ::core::ffi::c_int) as ::core::ffi::c_float,
    z: 0 as ::core::ffi::c_int as ::core::ffi::c_float,
};
#[no_mangle]
pub static mut KM_VEC3_NEG_X: kmVec3 = kmVec3 {
    x: -(1 as ::core::ffi::c_int) as ::core::ffi::c_float,
    y: 0 as ::core::ffi::c_int as ::core::ffi::c_float,
    z: 0 as ::core::ffi::c_int as ::core::ffi::c_float,
};
#[no_mangle]
pub static mut KM_VEC3_POS_X: kmVec3 = kmVec3 {
    x: 1 as ::core::ffi::c_int as ::core::ffi::c_float,
    y: 0 as ::core::ffi::c_int as ::core::ffi::c_float,
    z: 0 as ::core::ffi::c_int as ::core::ffi::c_float,
};
#[no_mangle]
pub static mut KM_VEC3_ZERO: kmVec3 = kmVec3 {
    x: 0 as ::core::ffi::c_int as ::core::ffi::c_float,
    y: 0 as ::core::ffi::c_int as ::core::ffi::c_float,
    z: 0 as ::core::ffi::c_int as ::core::ffi::c_float,
};
#[no_mangle]
pub unsafe extern "C" fn kmVec3Fill(
    mut pOut: *mut kmVec3,
    mut x: ::core::ffi::c_float,
    mut y: ::core::ffi::c_float,
    mut z: ::core::ffi::c_float,
) -> *mut kmVec3 {
    (*pOut).x = x;
    (*pOut).y = y;
    (*pOut).z = z;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmVec3Length(mut pIn: *const kmVec3) -> ::core::ffi::c_float {
    return sqrtf(kmSQR((*pIn).x) + kmSQR((*pIn).y) + kmSQR((*pIn).z));
}
#[no_mangle]
pub unsafe extern "C" fn kmVec3LengthSq(mut pIn: *const kmVec3) -> ::core::ffi::c_float {
    return kmSQR((*pIn).x) + kmSQR((*pIn).y) + kmSQR((*pIn).z);
}
#[no_mangle]
pub unsafe extern "C" fn kmVec3Lerp(
    mut pOut: *mut kmVec3,
    mut pV1: *const kmVec3,
    mut pV2: *const kmVec3,
    mut t: ::core::ffi::c_float,
) -> *mut kmVec3 {
    (*pOut).x = (*pV1).x + t * ((*pV2).x - (*pV1).x);
    (*pOut).y = (*pV1).y + t * ((*pV2).y - (*pV1).y);
    (*pOut).z = (*pV1).z + t * ((*pV2).z - (*pV1).z);
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmVec3Normalize(
    mut pOut: *mut kmVec3,
    mut pIn: *const kmVec3,
) -> *mut kmVec3 {
    if (*pIn).x == 0. && (*pIn).y == 0. && (*pIn).z == 0. {
        return kmVec3Assign(pOut, pIn);
    }
    let mut l: ::core::ffi::c_float = 1.0f32 / kmVec3Length(pIn);
    let mut v: kmVec3 = kmVec3 {
        x: 0.,
        y: 0.,
        z: 0.,
    };
    v.x = (*pIn).x * l;
    v.y = (*pIn).y * l;
    v.z = (*pIn).z * l;
    (*pOut).x = v.x;
    (*pOut).y = v.y;
    (*pOut).z = v.z;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmVec3Cross(
    mut pOut: *mut kmVec3,
    mut pV1: *const kmVec3,
    mut pV2: *const kmVec3,
) -> *mut kmVec3 {
    let mut v: kmVec3 = kmVec3 {
        x: 0.,
        y: 0.,
        z: 0.,
    };
    v.x = (*pV1).y * (*pV2).z - (*pV1).z * (*pV2).y;
    v.y = (*pV1).z * (*pV2).x - (*pV1).x * (*pV2).z;
    v.z = (*pV1).x * (*pV2).y - (*pV1).y * (*pV2).x;
    (*pOut).x = v.x;
    (*pOut).y = v.y;
    (*pOut).z = v.z;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmVec3Dot(
    mut pV1: *const kmVec3,
    mut pV2: *const kmVec3,
) -> ::core::ffi::c_float {
    return (*pV1).x * (*pV2).x + (*pV1).y * (*pV2).y + (*pV1).z * (*pV2).z;
}
#[no_mangle]
pub unsafe extern "C" fn kmVec3Add(
    mut pOut: *mut kmVec3,
    mut pV1: *const kmVec3,
    mut pV2: *const kmVec3,
) -> *mut kmVec3 {
    let mut v: kmVec3 = kmVec3 {
        x: 0.,
        y: 0.,
        z: 0.,
    };
    v.x = (*pV1).x + (*pV2).x;
    v.y = (*pV1).y + (*pV2).y;
    v.z = (*pV1).z + (*pV2).z;
    (*pOut).x = v.x;
    (*pOut).y = v.y;
    (*pOut).z = v.z;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmVec3Subtract(
    mut pOut: *mut kmVec3,
    mut pV1: *const kmVec3,
    mut pV2: *const kmVec3,
) -> *mut kmVec3 {
    let mut v: kmVec3 = kmVec3 {
        x: 0.,
        y: 0.,
        z: 0.,
    };
    v.x = (*pV1).x - (*pV2).x;
    v.y = (*pV1).y - (*pV2).y;
    v.z = (*pV1).z - (*pV2).z;
    (*pOut).x = v.x;
    (*pOut).y = v.y;
    (*pOut).z = v.z;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmVec3Mul(
    mut pOut: *mut kmVec3,
    mut pV1: *const kmVec3,
    mut pV2: *const kmVec3,
) -> *mut kmVec3 {
    (*pOut).x = (*pV1).x * (*pV2).x;
    (*pOut).y = (*pV1).y * (*pV2).y;
    (*pOut).z = (*pV1).z * (*pV2).z;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmVec3Div(
    mut pOut: *mut kmVec3,
    mut pV1: *const kmVec3,
    mut pV2: *const kmVec3,
) -> *mut kmVec3 {
    if (*pV2).x != 0. && (*pV2).y != 0. && (*pV2).z != 0. {
        (*pOut).x = (*pV1).x / (*pV2).x;
        (*pOut).y = (*pV1).y / (*pV2).y;
        (*pOut).z = (*pV1).z / (*pV2).z;
    }
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmVec3MultiplyMat3(
    mut pOut: *mut kmVec3,
    mut pV: *const kmVec3,
    mut pM: *const kmMat3,
) -> *mut kmVec3 {
    let mut v: kmVec3 = kmVec3 {
        x: 0.,
        y: 0.,
        z: 0.,
    };
    v.x = (*pV).x * (*pM).mat[0 as ::core::ffi::c_int as usize]
        + (*pV).y * (*pM).mat[3 as ::core::ffi::c_int as usize]
        + (*pV).z * (*pM).mat[6 as ::core::ffi::c_int as usize];
    v.y = (*pV).x * (*pM).mat[1 as ::core::ffi::c_int as usize]
        + (*pV).y * (*pM).mat[4 as ::core::ffi::c_int as usize]
        + (*pV).z * (*pM).mat[7 as ::core::ffi::c_int as usize];
    v.z = (*pV).x * (*pM).mat[2 as ::core::ffi::c_int as usize]
        + (*pV).y * (*pM).mat[5 as ::core::ffi::c_int as usize]
        + (*pV).z * (*pM).mat[8 as ::core::ffi::c_int as usize];
    (*pOut).x = v.x;
    (*pOut).y = v.y;
    (*pOut).z = v.z;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmVec3MultiplyMat4(
    mut pOut: *mut kmVec3,
    mut pV: *const kmVec3,
    mut pM: *const kmMat4,
) -> *mut kmVec3 {
    let mut v: kmVec3 = kmVec3 {
        x: 0.,
        y: 0.,
        z: 0.,
    };
    v.x = (*pV).x * (*pM).mat[0 as ::core::ffi::c_int as usize]
        + (*pV).y * (*pM).mat[4 as ::core::ffi::c_int as usize]
        + (*pV).z * (*pM).mat[8 as ::core::ffi::c_int as usize]
        + (*pM).mat[12 as ::core::ffi::c_int as usize];
    v.y = (*pV).x * (*pM).mat[1 as ::core::ffi::c_int as usize]
        + (*pV).y * (*pM).mat[5 as ::core::ffi::c_int as usize]
        + (*pV).z * (*pM).mat[9 as ::core::ffi::c_int as usize]
        + (*pM).mat[13 as ::core::ffi::c_int as usize];
    v.z = (*pV).x * (*pM).mat[2 as ::core::ffi::c_int as usize]
        + (*pV).y * (*pM).mat[6 as ::core::ffi::c_int as usize]
        + (*pV).z * (*pM).mat[10 as ::core::ffi::c_int as usize]
        + (*pM).mat[14 as ::core::ffi::c_int as usize];
    (*pOut).x = v.x;
    (*pOut).y = v.y;
    (*pOut).z = v.z;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmVec3Transform(
    mut pOut: *mut kmVec3,
    mut pV: *const kmVec3,
    mut pM: *const kmMat4,
) -> *mut kmVec3 {
    return kmVec3MultiplyMat4(pOut, pV, pM);
}
#[no_mangle]
pub unsafe extern "C" fn kmVec3InverseTransform(
    mut pOut: *mut kmVec3,
    mut pVect: *const kmVec3,
    mut pM: *const kmMat4,
) -> *mut kmVec3 {
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
    v1.x = (*pVect).x - (*pM).mat[12 as ::core::ffi::c_int as usize];
    v1.y = (*pVect).y - (*pM).mat[13 as ::core::ffi::c_int as usize];
    v1.z = (*pVect).z - (*pM).mat[14 as ::core::ffi::c_int as usize];
    v2.x = v1.x * (*pM).mat[0 as ::core::ffi::c_int as usize]
        + v1.y * (*pM).mat[1 as ::core::ffi::c_int as usize]
        + v1.z * (*pM).mat[2 as ::core::ffi::c_int as usize];
    v2.y = v1.x * (*pM).mat[4 as ::core::ffi::c_int as usize]
        + v1.y * (*pM).mat[5 as ::core::ffi::c_int as usize]
        + v1.z * (*pM).mat[6 as ::core::ffi::c_int as usize];
    v2.z = v1.x * (*pM).mat[8 as ::core::ffi::c_int as usize]
        + v1.y * (*pM).mat[9 as ::core::ffi::c_int as usize]
        + v1.z * (*pM).mat[10 as ::core::ffi::c_int as usize];
    (*pOut).x = v2.x;
    (*pOut).y = v2.y;
    (*pOut).z = v2.z;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmVec3InverseTransformNormal(
    mut pOut: *mut kmVec3,
    mut pVect: *const kmVec3,
    mut pM: *const kmMat4,
) -> *mut kmVec3 {
    let mut v: kmVec3 = kmVec3 {
        x: 0.,
        y: 0.,
        z: 0.,
    };
    v.x = (*pVect).x * (*pM).mat[0 as ::core::ffi::c_int as usize]
        + (*pVect).y * (*pM).mat[1 as ::core::ffi::c_int as usize]
        + (*pVect).z * (*pM).mat[2 as ::core::ffi::c_int as usize];
    v.y = (*pVect).x * (*pM).mat[4 as ::core::ffi::c_int as usize]
        + (*pVect).y * (*pM).mat[5 as ::core::ffi::c_int as usize]
        + (*pVect).z * (*pM).mat[6 as ::core::ffi::c_int as usize];
    v.z = (*pVect).x * (*pM).mat[8 as ::core::ffi::c_int as usize]
        + (*pVect).y * (*pM).mat[9 as ::core::ffi::c_int as usize]
        + (*pVect).z * (*pM).mat[10 as ::core::ffi::c_int as usize];
    (*pOut).x = v.x;
    (*pOut).y = v.y;
    (*pOut).z = v.z;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmVec3TransformCoord(
    mut pOut: *mut kmVec3,
    mut pV: *const kmVec3,
    mut pM: *const kmMat4,
) -> *mut kmVec3 {
    let mut v: kmVec4 = kmVec4 {
        x: 0.,
        y: 0.,
        z: 0.,
        w: 0.,
    };
    let mut inV: kmVec4 = kmVec4 {
        x: 0.,
        y: 0.,
        z: 0.,
        w: 0.,
    };
    kmVec4Fill(&raw mut inV, (*pV).x, (*pV).y, (*pV).z, 1.0f32);
    kmVec4Transform(&raw mut v, &raw mut inV, pM as *const kmMat4);
    (*pOut).x = v.x / v.w;
    (*pOut).y = v.y / v.w;
    (*pOut).z = v.z / v.w;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmVec3TransformNormal(
    mut pOut: *mut kmVec3,
    mut pV: *const kmVec3,
    mut pM: *const kmMat4,
) -> *mut kmVec3 {
    let mut v: kmVec3 = kmVec3 {
        x: 0.,
        y: 0.,
        z: 0.,
    };
    v.x = (*pV).x * (*pM).mat[0 as ::core::ffi::c_int as usize]
        + (*pV).y * (*pM).mat[4 as ::core::ffi::c_int as usize]
        + (*pV).z * (*pM).mat[8 as ::core::ffi::c_int as usize];
    v.y = (*pV).x * (*pM).mat[1 as ::core::ffi::c_int as usize]
        + (*pV).y * (*pM).mat[5 as ::core::ffi::c_int as usize]
        + (*pV).z * (*pM).mat[9 as ::core::ffi::c_int as usize];
    v.z = (*pV).x * (*pM).mat[2 as ::core::ffi::c_int as usize]
        + (*pV).y * (*pM).mat[6 as ::core::ffi::c_int as usize]
        + (*pV).z * (*pM).mat[10 as ::core::ffi::c_int as usize];
    (*pOut).x = v.x;
    (*pOut).y = v.y;
    (*pOut).z = v.z;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmVec3Scale(
    mut pOut: *mut kmVec3,
    mut pIn: *const kmVec3,
    s: ::core::ffi::c_float,
) -> *mut kmVec3 {
    (*pOut).x = (*pIn).x * s;
    (*pOut).y = (*pIn).y * s;
    (*pOut).z = (*pIn).z * s;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmVec3AreEqual(
    mut p1: *const kmVec3,
    mut p2: *const kmVec3,
) -> ::core::ffi::c_int {
    if ((*p1).x as ::core::ffi::c_double) < (*p2).x as ::core::ffi::c_double + kmEpsilon
        && (*p1).x as ::core::ffi::c_double > (*p2).x as ::core::ffi::c_double - kmEpsilon
        && (((*p1).y as ::core::ffi::c_double) < (*p2).y as ::core::ffi::c_double + kmEpsilon
            && (*p1).y as ::core::ffi::c_double > (*p2).y as ::core::ffi::c_double - kmEpsilon)
        && (((*p1).z as ::core::ffi::c_double) < (*p2).z as ::core::ffi::c_double + kmEpsilon
            && (*p1).z as ::core::ffi::c_double > (*p2).z as ::core::ffi::c_double - kmEpsilon)
    {
        return 1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn kmVec3Assign(
    mut pOut: *mut kmVec3,
    mut pIn: *const kmVec3,
) -> *mut kmVec3 {
    if pOut == pIn as *mut kmVec3 {
        return pOut;
    }
    (*pOut).x = (*pIn).x;
    (*pOut).y = (*pIn).y;
    (*pOut).z = (*pIn).z;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmVec3Zero(mut pOut: *mut kmVec3) -> *mut kmVec3 {
    (*pOut).x = 0.0f32;
    (*pOut).y = 0.0f32;
    (*pOut).z = 0.0f32;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmVec3GetHorizontalAngle(
    mut pOut: *mut kmVec3,
    mut pIn: *const kmVec3,
) -> *mut kmVec3 {
    let z1: ::core::ffi::c_float =
        sqrt(((*pIn).x * (*pIn).x + (*pIn).z * (*pIn).z) as ::core::ffi::c_double)
            as ::core::ffi::c_float;
    (*pOut).y = kmRadiansToDegrees(atan2(
        (*pIn).x as ::core::ffi::c_double,
        (*pIn).z as ::core::ffi::c_double,
    ) as ::core::ffi::c_float);
    if (*pOut).y < 0 as ::core::ffi::c_int as ::core::ffi::c_float {
        (*pOut).y += 360 as ::core::ffi::c_int as ::core::ffi::c_float;
    }
    if (*pOut).y >= 360 as ::core::ffi::c_int as ::core::ffi::c_float {
        (*pOut).y -= 360 as ::core::ffi::c_int as ::core::ffi::c_float;
    }
    (*pOut).x = (kmRadiansToDegrees(atan2(
        z1 as ::core::ffi::c_double,
        (*pIn).y as ::core::ffi::c_double,
    ) as ::core::ffi::c_float) as ::core::ffi::c_double
        - 90.0f64) as ::core::ffi::c_float;
    if (*pOut).x < 0 as ::core::ffi::c_int as ::core::ffi::c_float {
        (*pOut).x += 360 as ::core::ffi::c_int as ::core::ffi::c_float;
    }
    if (*pOut).x >= 360 as ::core::ffi::c_int as ::core::ffi::c_float {
        (*pOut).x -= 360 as ::core::ffi::c_int as ::core::ffi::c_float;
    }
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmVec3RotationToDirection(
    mut pOut: *mut kmVec3,
    mut pIn: *const kmVec3,
    mut forwards: *const kmVec3,
) -> *mut kmVec3 {
    let xr: ::core::ffi::c_float = kmDegreesToRadians((*pIn).x) as ::core::ffi::c_float;
    let yr: ::core::ffi::c_float = kmDegreesToRadians((*pIn).y) as ::core::ffi::c_float;
    let zr: ::core::ffi::c_float = kmDegreesToRadians((*pIn).z) as ::core::ffi::c_float;
    let cr: ::core::ffi::c_float = cos(xr as ::core::ffi::c_double) as ::core::ffi::c_float;
    let sr: ::core::ffi::c_float = sin(xr as ::core::ffi::c_double) as ::core::ffi::c_float;
    let cp: ::core::ffi::c_float = cos(yr as ::core::ffi::c_double) as ::core::ffi::c_float;
    let sp: ::core::ffi::c_float = sin(yr as ::core::ffi::c_double) as ::core::ffi::c_float;
    let cy: ::core::ffi::c_float = cos(zr as ::core::ffi::c_double) as ::core::ffi::c_float;
    let sy: ::core::ffi::c_float = sin(zr as ::core::ffi::c_double) as ::core::ffi::c_float;
    let srsp: ::core::ffi::c_float = sr * sp;
    let crsp: ::core::ffi::c_float = cr * sp;
    let pseudoMatrix: [::core::ffi::c_float; 9] = [
        cp * cy,
        cp * sy,
        -sp,
        srsp * cy - cr * sy,
        srsp * sy + cr * cy,
        sr * cp,
        crsp * cy + sr * sy,
        crsp * sy - sr * cy,
        cr * cp,
    ];
    (*pOut).x = (*forwards).x * pseudoMatrix[0 as ::core::ffi::c_int as usize]
        + (*forwards).y * pseudoMatrix[3 as ::core::ffi::c_int as usize]
        + (*forwards).z * pseudoMatrix[6 as ::core::ffi::c_int as usize];
    (*pOut).y = (*forwards).x * pseudoMatrix[1 as ::core::ffi::c_int as usize]
        + (*forwards).y * pseudoMatrix[4 as ::core::ffi::c_int as usize]
        + (*forwards).z * pseudoMatrix[7 as ::core::ffi::c_int as usize];
    (*pOut).z = (*forwards).x * pseudoMatrix[2 as ::core::ffi::c_int as usize]
        + (*forwards).y * pseudoMatrix[5 as ::core::ffi::c_int as usize]
        + (*forwards).z * pseudoMatrix[8 as ::core::ffi::c_int as usize];
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmVec3ProjectOnToPlane(
    mut pOut: *mut kmVec3,
    mut point: *const kmVec3,
    mut plane: *const kmPlane,
) -> *mut kmVec3 {
    let mut ray: kmRay3 = kmRay3 {
        start: kmVec3 {
            x: 0.,
            y: 0.,
            z: 0.,
        },
        dir: kmVec3 {
            x: 0.,
            y: 0.,
            z: 0.,
        },
    };
    kmVec3Assign(&raw mut ray.start, point);
    ray.dir.x = -(*plane).a;
    ray.dir.y = -(*plane).b;
    ray.dir.z = -(*plane).c;
    kmRay3IntersectPlane(pOut, &raw mut ray, plane);
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmVec3Reflect(
    mut pOut: *mut kmVec3,
    mut pIn: *const kmVec3,
    mut normal: *const kmVec3,
) -> *mut kmVec3 {
    let mut tmp: kmVec3 = kmVec3 {
        x: 0.,
        y: 0.,
        z: 0.,
    };
    kmVec3Scale(&raw mut tmp, normal, 2.0f32 * kmVec3Dot(pIn, normal));
    kmVec3Subtract(pOut, pIn, &raw mut tmp);
    return pOut;
}
