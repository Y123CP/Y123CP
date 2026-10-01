extern "C" {
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn acos(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn asin(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn atan2(__y: ::core::ffi::c_double, __x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn cos(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn sin(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn sqrt(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn fabs(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn acosf(__x: ::core::ffi::c_float) -> ::core::ffi::c_float;
    fn cosf(__x: ::core::ffi::c_float) -> ::core::ffi::c_float;
    fn sinf(__x: ::core::ffi::c_float) -> ::core::ffi::c_float;
    fn sqrtf(__x: ::core::ffi::c_float) -> ::core::ffi::c_float;
    fn kmSQR(s: ::core::ffi::c_float) -> ::core::ffi::c_float;
    fn kmClamp(
        x: ::core::ffi::c_float,
        min: ::core::ffi::c_float,
        max: ::core::ffi::c_float,
    ) -> ::core::ffi::c_float;
    fn kmVec3LengthSq(pIn: *const kmVec3) -> ::core::ffi::c_float;
    fn kmVec3Normalize(pOut: *mut kmVec3, pIn: *const kmVec3) -> *mut kmVec3;
    fn kmVec3Cross(pOut: *mut kmVec3, pV1: *const kmVec3, pV2: *const kmVec3) -> *mut kmVec3;
    fn kmVec3Dot(pV1: *const kmVec3, pV2: *const kmVec3) -> ::core::ffi::c_float;
    fn kmVec3Add(pOut: *mut kmVec3, pV1: *const kmVec3, pV2: *const kmVec3) -> *mut kmVec3;
    fn kmVec3Scale(pOut: *mut kmVec3, pIn: *const kmVec3, s: ::core::ffi::c_float) -> *mut kmVec3;
    fn kmVec3Assign(pOut: *mut kmVec3, pIn: *const kmVec3) -> *mut kmVec3;
    static KM_VEC3_NEG_Z: kmVec3;
    static KM_VEC3_POS_Z: kmVec3;
    static KM_VEC3_POS_Y: kmVec3;
    static KM_VEC3_POS_X: kmVec3;
    static KM_VEC3_ZERO: kmVec3;
    fn kmMat3LookAt(
        pOut: *mut kmMat3,
        pEye: *const kmVec3,
        pCenter: *const kmVec3,
        pUp: *const kmVec3,
    ) -> *mut kmMat3;
}
pub type size_t = usize;
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
pub struct kmQuaternion {
    pub x: ::core::ffi::c_float,
    pub y: ::core::ffi::c_float,
    pub z: ::core::ffi::c_float,
    pub w: ::core::ffi::c_float,
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const kmPI: ::core::ffi::c_float = 3.14159265358979323846f32;
pub const kmEpsilon: ::core::ffi::c_double = 0.0001f64;
#[no_mangle]
pub unsafe extern "C" fn kmQuaternionAreEqual(
    mut p1: *const kmQuaternion,
    mut p2: *const kmQuaternion,
) -> ::core::ffi::c_int {
    if ((*p1).x as ::core::ffi::c_double) < (*p2).x as ::core::ffi::c_double + kmEpsilon
        && (*p1).x as ::core::ffi::c_double > (*p2).x as ::core::ffi::c_double - kmEpsilon
        && (((*p1).y as ::core::ffi::c_double) < (*p2).y as ::core::ffi::c_double + kmEpsilon
            && (*p1).y as ::core::ffi::c_double > (*p2).y as ::core::ffi::c_double - kmEpsilon)
        && (((*p1).z as ::core::ffi::c_double) < (*p2).z as ::core::ffi::c_double + kmEpsilon
            && (*p1).z as ::core::ffi::c_double > (*p2).z as ::core::ffi::c_double - kmEpsilon)
        && (((*p1).w as ::core::ffi::c_double) < (*p2).w as ::core::ffi::c_double + kmEpsilon
            && (*p1).w as ::core::ffi::c_double > (*p2).w as ::core::ffi::c_double - kmEpsilon)
    {
        return 1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn kmQuaternionFill(
    mut pOut: *mut kmQuaternion,
    mut x: ::core::ffi::c_float,
    mut y: ::core::ffi::c_float,
    mut z: ::core::ffi::c_float,
    mut w: ::core::ffi::c_float,
) -> *mut kmQuaternion {
    (*pOut).x = x;
    (*pOut).y = y;
    (*pOut).z = z;
    (*pOut).w = w;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmQuaternionDot(
    mut q1: *const kmQuaternion,
    mut q2: *const kmQuaternion,
) -> ::core::ffi::c_float {
    return (*q1).w * (*q2).w + (*q1).x * (*q2).x + (*q1).y * (*q2).y + (*q1).z * (*q2).z;
}
#[no_mangle]
pub unsafe extern "C" fn kmQuaternionExp(
    mut pOut: *mut kmQuaternion,
    mut pIn: *const kmQuaternion,
) -> *mut kmQuaternion {
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmQuaternionIdentity(mut pOut: *mut kmQuaternion) -> *mut kmQuaternion {
    (*pOut).x = 0.0f32;
    (*pOut).y = 0.0f32;
    (*pOut).z = 0.0f32;
    (*pOut).w = 1.0f32;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmQuaternionInverse(
    mut pOut: *mut kmQuaternion,
    mut pIn: *const kmQuaternion,
) -> *mut kmQuaternion {
    let mut l: ::core::ffi::c_float = kmQuaternionLength(pIn);
    if fabs(l as ::core::ffi::c_double) < kmEpsilon {
        (*pOut).x = 0.0f32;
        (*pOut).y = 0.0f32;
        (*pOut).z = 0.0f32;
        (*pOut).w = 0.0f32;
        return pOut;
    }
    (*pOut).x = -(*pIn).x;
    (*pOut).y = -(*pIn).y;
    (*pOut).z = -(*pIn).z;
    (*pOut).w = (*pIn).w;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmQuaternionIsIdentity(
    mut pIn: *const kmQuaternion,
) -> ::core::ffi::c_int {
    return ((*pIn).x as ::core::ffi::c_double == 0.0f64
        && (*pIn).y as ::core::ffi::c_double == 0.0f64
        && (*pIn).z as ::core::ffi::c_double == 0.0f64
        && (*pIn).w as ::core::ffi::c_double == 1.0f64) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn kmQuaternionLength(mut pIn: *const kmQuaternion) -> ::core::ffi::c_float {
    return sqrt(kmQuaternionLengthSq(pIn) as ::core::ffi::c_double) as ::core::ffi::c_float;
}
#[no_mangle]
pub unsafe extern "C" fn kmQuaternionLengthSq(
    mut pIn: *const kmQuaternion,
) -> ::core::ffi::c_float {
    return (*pIn).x * (*pIn).x + (*pIn).y * (*pIn).y + (*pIn).z * (*pIn).z + (*pIn).w * (*pIn).w;
}
#[no_mangle]
pub unsafe extern "C" fn kmQuaternionLn(
    mut pOut: *mut kmQuaternion,
    mut pIn: *const kmQuaternion,
) -> *mut kmQuaternion {
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmQuaternionMultiply(
    mut pOut: *mut kmQuaternion,
    mut qu1: *const kmQuaternion,
    mut qu2: *const kmQuaternion,
) -> *mut kmQuaternion {
    let mut tmp1: kmQuaternion = kmQuaternion {
        x: 0.,
        y: 0.,
        z: 0.,
        w: 0.,
    };
    let mut tmp2: kmQuaternion = kmQuaternion {
        x: 0.,
        y: 0.,
        z: 0.,
        w: 0.,
    };
    kmQuaternionAssign(&raw mut tmp1, qu1);
    kmQuaternionAssign(&raw mut tmp2, qu2);
    let mut q1: *mut kmQuaternion = &raw mut tmp1;
    let mut q2: *mut kmQuaternion = &raw mut tmp2;
    (*pOut).x = (*q1).w * (*q2).x + (*q1).x * (*q2).w + (*q1).y * (*q2).z - (*q1).z * (*q2).y;
    (*pOut).y = (*q1).w * (*q2).y + (*q1).y * (*q2).w + (*q1).z * (*q2).x - (*q1).x * (*q2).z;
    (*pOut).z = (*q1).w * (*q2).z + (*q1).z * (*q2).w + (*q1).x * (*q2).y - (*q1).y * (*q2).x;
    (*pOut).w = (*q1).w * (*q2).w - (*q1).x * (*q2).x - (*q1).y * (*q2).y - (*q1).z * (*q2).z;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmQuaternionNormalize(
    mut pOut: *mut kmQuaternion,
    mut pIn: *const kmQuaternion,
) -> *mut kmQuaternion {
    let mut length: ::core::ffi::c_float = kmQuaternionLength(pIn);
    if fabs(length as ::core::ffi::c_double) < kmEpsilon {
        (*pOut).x = 0.0f32;
        (*pOut).y = 0.0f32;
        (*pOut).z = 0.0f32;
        (*pOut).w = 0.0f32;
        return pOut;
    }
    kmQuaternionFill(
        pOut,
        (*pOut).x / length,
        (*pOut).y / length,
        (*pOut).z / length,
        (*pOut).w / length,
    );
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmQuaternionRotationAxisAngle(
    mut pOut: *mut kmQuaternion,
    mut pV: *const kmVec3,
    mut angle: ::core::ffi::c_float,
) -> *mut kmQuaternion {
    let mut rad: ::core::ffi::c_float = angle * 0.5f32;
    let mut scale: ::core::ffi::c_float = sinf(rad);
    (*pOut).x = (*pV).x * scale;
    (*pOut).y = (*pV).y * scale;
    (*pOut).z = (*pV).z * scale;
    (*pOut).w = cosf(rad);
    kmQuaternionNormalize(pOut, pOut);
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmQuaternionRotationMatrix(
    mut pOut: *mut kmQuaternion,
    mut pIn: *const kmMat3,
) -> *mut kmQuaternion {
    let mut x: ::core::ffi::c_float = 0.;
    let mut y: ::core::ffi::c_float = 0.;
    let mut z: ::core::ffi::c_float = 0.;
    let mut w: ::core::ffi::c_float = 0.;
    let mut pMatrix: *mut ::core::ffi::c_float = ::core::ptr::null_mut::<::core::ffi::c_float>();
    let mut m4x4: [::core::ffi::c_float; 16] = [
        0 as ::core::ffi::c_int as ::core::ffi::c_float,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
    ];
    let mut scale: ::core::ffi::c_float = 0.0f32;
    let mut diagonal: ::core::ffi::c_float = 0.0f32;
    if pIn.is_null() {
        return ::core::ptr::null_mut::<kmQuaternion>();
    }
    m4x4[0 as ::core::ffi::c_int as usize] = (*pIn).mat[0 as ::core::ffi::c_int as usize];
    m4x4[1 as ::core::ffi::c_int as usize] = (*pIn).mat[3 as ::core::ffi::c_int as usize];
    m4x4[2 as ::core::ffi::c_int as usize] = (*pIn).mat[6 as ::core::ffi::c_int as usize];
    m4x4[4 as ::core::ffi::c_int as usize] = (*pIn).mat[1 as ::core::ffi::c_int as usize];
    m4x4[5 as ::core::ffi::c_int as usize] = (*pIn).mat[4 as ::core::ffi::c_int as usize];
    m4x4[6 as ::core::ffi::c_int as usize] = (*pIn).mat[7 as ::core::ffi::c_int as usize];
    m4x4[8 as ::core::ffi::c_int as usize] = (*pIn).mat[2 as ::core::ffi::c_int as usize];
    m4x4[9 as ::core::ffi::c_int as usize] = (*pIn).mat[5 as ::core::ffi::c_int as usize];
    m4x4[10 as ::core::ffi::c_int as usize] = (*pIn).mat[8 as ::core::ffi::c_int as usize];
    m4x4[15 as ::core::ffi::c_int as usize] = 1 as ::core::ffi::c_int as ::core::ffi::c_float;
    pMatrix = (&raw mut m4x4 as *mut ::core::ffi::c_float).offset(0 as ::core::ffi::c_int as isize)
        as *mut ::core::ffi::c_float;
    diagonal = *pMatrix.offset(0 as ::core::ffi::c_int as isize)
        + *pMatrix.offset(5 as ::core::ffi::c_int as isize)
        + *pMatrix.offset(10 as ::core::ffi::c_int as isize)
        + 1 as ::core::ffi::c_int as ::core::ffi::c_float;
    if diagonal as ::core::ffi::c_double > kmEpsilon {
        scale = sqrt(diagonal as ::core::ffi::c_double) as ::core::ffi::c_float
            * 2 as ::core::ffi::c_int as ::core::ffi::c_float;
        x = (*pMatrix.offset(9 as ::core::ffi::c_int as isize)
            - *pMatrix.offset(6 as ::core::ffi::c_int as isize))
            / scale;
        y = (*pMatrix.offset(2 as ::core::ffi::c_int as isize)
            - *pMatrix.offset(8 as ::core::ffi::c_int as isize))
            / scale;
        z = (*pMatrix.offset(4 as ::core::ffi::c_int as isize)
            - *pMatrix.offset(1 as ::core::ffi::c_int as isize))
            / scale;
        w = 0.25f32 * scale;
    } else if *pMatrix.offset(0 as ::core::ffi::c_int as isize)
        > *pMatrix.offset(5 as ::core::ffi::c_int as isize)
        && *pMatrix.offset(0 as ::core::ffi::c_int as isize)
            > *pMatrix.offset(10 as ::core::ffi::c_int as isize)
    {
        scale = sqrt(
            (1.0f32 + *pMatrix.offset(0 as ::core::ffi::c_int as isize)
                - *pMatrix.offset(5 as ::core::ffi::c_int as isize)
                - *pMatrix.offset(10 as ::core::ffi::c_int as isize))
                as ::core::ffi::c_double,
        ) as ::core::ffi::c_float
            * 2.0f32;
        x = 0.25f32 * scale;
        y = (*pMatrix.offset(4 as ::core::ffi::c_int as isize)
            + *pMatrix.offset(1 as ::core::ffi::c_int as isize))
            / scale;
        z = (*pMatrix.offset(2 as ::core::ffi::c_int as isize)
            + *pMatrix.offset(8 as ::core::ffi::c_int as isize))
            / scale;
        w = (*pMatrix.offset(9 as ::core::ffi::c_int as isize)
            - *pMatrix.offset(6 as ::core::ffi::c_int as isize))
            / scale;
    } else if *pMatrix.offset(5 as ::core::ffi::c_int as isize)
        > *pMatrix.offset(10 as ::core::ffi::c_int as isize)
    {
        scale = sqrt(
            (1.0f32 + *pMatrix.offset(5 as ::core::ffi::c_int as isize)
                - *pMatrix.offset(0 as ::core::ffi::c_int as isize)
                - *pMatrix.offset(10 as ::core::ffi::c_int as isize))
                as ::core::ffi::c_double,
        ) as ::core::ffi::c_float
            * 2.0f32;
        x = (*pMatrix.offset(4 as ::core::ffi::c_int as isize)
            + *pMatrix.offset(1 as ::core::ffi::c_int as isize))
            / scale;
        y = 0.25f32 * scale;
        z = (*pMatrix.offset(9 as ::core::ffi::c_int as isize)
            + *pMatrix.offset(6 as ::core::ffi::c_int as isize))
            / scale;
        w = (*pMatrix.offset(2 as ::core::ffi::c_int as isize)
            - *pMatrix.offset(8 as ::core::ffi::c_int as isize))
            / scale;
    } else {
        scale = sqrt(
            (1.0f32 + *pMatrix.offset(10 as ::core::ffi::c_int as isize)
                - *pMatrix.offset(0 as ::core::ffi::c_int as isize)
                - *pMatrix.offset(5 as ::core::ffi::c_int as isize))
                as ::core::ffi::c_double,
        ) as ::core::ffi::c_float
            * 2.0f32;
        x = (*pMatrix.offset(2 as ::core::ffi::c_int as isize)
            + *pMatrix.offset(8 as ::core::ffi::c_int as isize))
            / scale;
        y = (*pMatrix.offset(9 as ::core::ffi::c_int as isize)
            + *pMatrix.offset(6 as ::core::ffi::c_int as isize))
            / scale;
        z = 0.25f32 * scale;
        w = (*pMatrix.offset(4 as ::core::ffi::c_int as isize)
            - *pMatrix.offset(1 as ::core::ffi::c_int as isize))
            / scale;
    }
    (*pOut).x = x;
    (*pOut).y = y;
    (*pOut).z = z;
    (*pOut).w = w;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmQuaternionRotationPitchYawRoll(
    mut pOut: *mut kmQuaternion,
    mut pitch: ::core::ffi::c_float,
    mut yaw: ::core::ffi::c_float,
    mut roll: ::core::ffi::c_float,
) -> *mut kmQuaternion {
    let mut sY: ::core::ffi::c_float =
        sinf((yaw as ::core::ffi::c_double * 0.5f64) as ::core::ffi::c_float);
    let mut cY: ::core::ffi::c_float =
        cosf((yaw as ::core::ffi::c_double * 0.5f64) as ::core::ffi::c_float);
    let mut sZ: ::core::ffi::c_float =
        sinf((roll as ::core::ffi::c_double * 0.5f64) as ::core::ffi::c_float);
    let mut cZ: ::core::ffi::c_float =
        cosf((roll as ::core::ffi::c_double * 0.5f64) as ::core::ffi::c_float);
    let mut sX: ::core::ffi::c_float =
        sinf((pitch as ::core::ffi::c_double * 0.5f64) as ::core::ffi::c_float);
    let mut cX: ::core::ffi::c_float =
        cosf((pitch as ::core::ffi::c_double * 0.5f64) as ::core::ffi::c_float);
    (*pOut).w = cY * cZ * cX - sY * sZ * sX;
    (*pOut).x = sY * sZ * cX + cY * cZ * sX;
    (*pOut).y = sY * cZ * cX + cY * sZ * sX;
    (*pOut).z = cY * sZ * cX - sY * cZ * sX;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmQuaternionSlerp(
    mut pOut: *mut kmQuaternion,
    mut q1: *const kmQuaternion,
    mut q2: *const kmQuaternion,
    mut t: ::core::ffi::c_float,
) -> *mut kmQuaternion {
    let mut dot: ::core::ffi::c_float = kmQuaternionDot(q1, q2);
    let DOT_THRESHOLD: ::core::ffi::c_double = 0.9995f64;
    if dot as ::core::ffi::c_double > DOT_THRESHOLD {
        let mut diff: kmQuaternion = kmQuaternion {
            x: 0.,
            y: 0.,
            z: 0.,
            w: 0.,
        };
        kmQuaternionSubtract(&raw mut diff, q2, q1);
        kmQuaternionScale(&raw mut diff, &raw mut diff, t);
        kmQuaternionAdd(pOut, q1, &raw mut diff);
        kmQuaternionNormalize(pOut, pOut);
        return pOut;
    }
    dot = kmClamp(
        dot,
        -(1 as ::core::ffi::c_int) as ::core::ffi::c_float,
        1 as ::core::ffi::c_int as ::core::ffi::c_float,
    );
    let mut theta_0: ::core::ffi::c_float =
        acos(dot as ::core::ffi::c_double) as ::core::ffi::c_float;
    let mut theta: ::core::ffi::c_float = theta_0 * t;
    let mut tmp: kmQuaternion = kmQuaternion {
        x: 0.,
        y: 0.,
        z: 0.,
        w: 0.,
    };
    kmQuaternionScale(&raw mut tmp, q1, dot);
    kmQuaternionSubtract(&raw mut tmp, q2, &raw mut tmp);
    kmQuaternionNormalize(&raw mut tmp, &raw mut tmp);
    let mut t1: kmQuaternion = kmQuaternion {
        x: 0.,
        y: 0.,
        z: 0.,
        w: 0.,
    };
    let mut t2: kmQuaternion = kmQuaternion {
        x: 0.,
        y: 0.,
        z: 0.,
        w: 0.,
    };
    kmQuaternionScale(
        &raw mut t1,
        q1,
        cos(theta as ::core::ffi::c_double) as ::core::ffi::c_float,
    );
    kmQuaternionScale(
        &raw mut t2,
        &raw mut tmp,
        sin(theta as ::core::ffi::c_double) as ::core::ffi::c_float,
    );
    kmQuaternionAdd(pOut, &raw mut t1, &raw mut t2);
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmQuaternionToAxisAngle(
    mut pIn: *const kmQuaternion,
    mut pAxis: *mut kmVec3,
    mut pAngle: *mut ::core::ffi::c_float,
) {
    let mut tempAngle: ::core::ffi::c_float = 0.;
    let mut scale: ::core::ffi::c_float = 0.;
    tempAngle = acosf((*pIn).w);
    scale = sqrtf(kmSQR((*pIn).x) + kmSQR((*pIn).y) + kmSQR((*pIn).z));
    if scale as ::core::ffi::c_double > -kmEpsilon && (scale as ::core::ffi::c_double) < kmEpsilon
        || (scale as ::core::ffi::c_double)
            < (2 as ::core::ffi::c_int as ::core::ffi::c_float * kmPI) as ::core::ffi::c_double
                + kmEpsilon
            && scale as ::core::ffi::c_double
                > (2 as ::core::ffi::c_int as ::core::ffi::c_float * kmPI) as ::core::ffi::c_double
                    - kmEpsilon
    {
        *pAngle = 0.0f32;
        (*pAxis).x = 0.0f32;
        (*pAxis).y = 0.0f32;
        (*pAxis).z = 1.0f32;
    } else {
        *pAngle = tempAngle * 2.0f32;
        (*pAxis).x = (*pIn).x / scale;
        (*pAxis).y = (*pIn).y / scale;
        (*pAxis).z = (*pIn).z / scale;
        kmVec3Normalize(pAxis, pAxis);
    };
}
#[no_mangle]
pub unsafe extern "C" fn kmQuaternionScale(
    mut pOut: *mut kmQuaternion,
    mut pIn: *const kmQuaternion,
    mut s: ::core::ffi::c_float,
) -> *mut kmQuaternion {
    (*pOut).x = (*pIn).x * s;
    (*pOut).y = (*pIn).y * s;
    (*pOut).z = (*pIn).z * s;
    (*pOut).w = (*pIn).w * s;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmQuaternionAssign(
    mut pOut: *mut kmQuaternion,
    mut pIn: *const kmQuaternion,
) -> *mut kmQuaternion {
    memcpy(
        pOut as *mut ::core::ffi::c_void,
        pIn as *const ::core::ffi::c_void,
        (::core::mem::size_of::<::core::ffi::c_float>() as size_t).wrapping_mul(4 as size_t),
    );
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmQuaternionSubtract(
    mut pOut: *mut kmQuaternion,
    mut pQ1: *const kmQuaternion,
    mut pQ2: *const kmQuaternion,
) -> *mut kmQuaternion {
    (*pOut).x = (*pQ1).x - (*pQ2).x;
    (*pOut).y = (*pQ1).y - (*pQ2).y;
    (*pOut).z = (*pQ1).z - (*pQ2).z;
    (*pOut).w = (*pQ1).w - (*pQ2).w;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmQuaternionAdd(
    mut pOut: *mut kmQuaternion,
    mut pQ1: *const kmQuaternion,
    mut pQ2: *const kmQuaternion,
) -> *mut kmQuaternion {
    (*pOut).x = (*pQ1).x + (*pQ2).x;
    (*pOut).y = (*pQ1).y + (*pQ2).y;
    (*pOut).z = (*pQ1).z + (*pQ2).z;
    (*pOut).w = (*pQ1).w + (*pQ2).w;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmQuaternionRotationBetweenVec3(
    mut pOut: *mut kmQuaternion,
    mut vec1: *const kmVec3,
    mut vec2: *const kmVec3,
    mut fallback: *const kmVec3,
) -> *mut kmQuaternion {
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
    let mut a: ::core::ffi::c_float = 0.;
    kmVec3Assign(&raw mut v1, vec1);
    kmVec3Assign(&raw mut v2, vec2);
    kmVec3Normalize(&raw mut v1, &raw mut v1);
    kmVec3Normalize(&raw mut v2, &raw mut v2);
    a = kmVec3Dot(&raw mut v1, &raw mut v2);
    if a as ::core::ffi::c_double >= 1.0f64 {
        kmQuaternionIdentity(pOut);
        return pOut;
    }
    if a < 1e-6f32 - 1.0f32 {
        if fabs(kmVec3LengthSq(fallback) as ::core::ffi::c_double) < kmEpsilon {
            kmQuaternionRotationAxisAngle(pOut, fallback, kmPI);
        } else {
            let mut axis: kmVec3 = kmVec3 {
                x: 0.,
                y: 0.,
                z: 0.,
            };
            let mut X: kmVec3 = kmVec3 {
                x: 0.,
                y: 0.,
                z: 0.,
            };
            X.x = 1.0f32;
            X.y = 0.0f32;
            X.z = 0.0f32;
            kmVec3Cross(&raw mut axis, &raw mut X, vec1);
            if fabs(kmVec3LengthSq(&raw mut axis) as ::core::ffi::c_double) < kmEpsilon {
                let mut Y: kmVec3 = kmVec3 {
                    x: 0.,
                    y: 0.,
                    z: 0.,
                };
                Y.x = 0.0f32;
                Y.y = 1.0f32;
                Y.z = 0.0f32;
                kmVec3Cross(&raw mut axis, &raw mut Y, vec1);
            }
            kmVec3Normalize(&raw mut axis, &raw mut axis);
            kmQuaternionRotationAxisAngle(pOut, &raw mut axis, kmPI);
        }
    } else {
        let mut s: ::core::ffi::c_float = sqrtf(
            (1 as ::core::ffi::c_int as ::core::ffi::c_float + a)
                * 2 as ::core::ffi::c_int as ::core::ffi::c_float,
        );
        let mut invs: ::core::ffi::c_float = 1 as ::core::ffi::c_int as ::core::ffi::c_float / s;
        let mut c: kmVec3 = kmVec3 {
            x: 0.,
            y: 0.,
            z: 0.,
        };
        kmVec3Cross(&raw mut c, &raw mut v1, &raw mut v2);
        (*pOut).x = c.x * invs;
        (*pOut).y = c.y * invs;
        (*pOut).z = c.z * invs;
        (*pOut).w = s * 0.5f32;
        kmQuaternionNormalize(pOut, pOut);
    }
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmQuaternionMultiplyVec3(
    mut pOut: *mut kmVec3,
    mut q: *const kmQuaternion,
    mut v: *const kmVec3,
) -> *mut kmVec3 {
    let mut uv: kmVec3 = kmVec3 {
        x: 0.,
        y: 0.,
        z: 0.,
    };
    let mut uuv: kmVec3 = kmVec3 {
        x: 0.,
        y: 0.,
        z: 0.,
    };
    let mut qvec: kmVec3 = kmVec3 {
        x: 0.,
        y: 0.,
        z: 0.,
    };
    qvec.x = (*q).x;
    qvec.y = (*q).y;
    qvec.z = (*q).z;
    kmVec3Cross(&raw mut uv, &raw mut qvec, v);
    kmVec3Cross(&raw mut uuv, &raw mut qvec, &raw mut uv);
    kmVec3Scale(&raw mut uv, &raw mut uv, 2.0f32 * (*q).w);
    kmVec3Scale(&raw mut uuv, &raw mut uuv, 2.0f32);
    kmVec3Add(pOut, v, &raw mut uv);
    kmVec3Add(pOut, pOut, &raw mut uuv);
    return pOut as *mut kmVec3;
}
#[no_mangle]
pub unsafe extern "C" fn kmQuaternionGetUpVec3(
    mut pOut: *mut kmVec3,
    mut pIn: *const kmQuaternion,
) -> *mut kmVec3 {
    return kmQuaternionMultiplyVec3(pOut, pIn, &raw const KM_VEC3_POS_Y) as *mut kmVec3;
}
#[no_mangle]
pub unsafe extern "C" fn kmQuaternionGetRightVec3(
    mut pOut: *mut kmVec3,
    mut pIn: *const kmQuaternion,
) -> *mut kmVec3 {
    return kmQuaternionMultiplyVec3(pOut, pIn, &raw const KM_VEC3_POS_X) as *mut kmVec3;
}
#[no_mangle]
pub unsafe extern "C" fn kmQuaternionGetForwardVec3RH(
    mut pOut: *mut kmVec3,
    mut pIn: *const kmQuaternion,
) -> *mut kmVec3 {
    return kmQuaternionMultiplyVec3(pOut, pIn, &raw const KM_VEC3_NEG_Z) as *mut kmVec3;
}
#[no_mangle]
pub unsafe extern "C" fn kmQuaternionGetForwardVec3LH(
    mut pOut: *mut kmVec3,
    mut pIn: *const kmQuaternion,
) -> *mut kmVec3 {
    return kmQuaternionMultiplyVec3(pOut, pIn, &raw const KM_VEC3_POS_Z) as *mut kmVec3;
}
#[no_mangle]
pub unsafe extern "C" fn kmQuaternionGetPitch(mut q: *const kmQuaternion) -> ::core::ffi::c_float {
    let mut result: ::core::ffi::c_float = atan2(
        (2 as ::core::ffi::c_int as ::core::ffi::c_float * ((*q).y * (*q).z + (*q).w * (*q).x))
            as ::core::ffi::c_double,
        ((*q).w * (*q).w - (*q).x * (*q).x - (*q).y * (*q).y + (*q).z * (*q).z)
            as ::core::ffi::c_double,
    ) as ::core::ffi::c_float;
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn kmQuaternionGetYaw(mut q: *const kmQuaternion) -> ::core::ffi::c_float {
    let mut result: ::core::ffi::c_float = asin(
        (-(2 as ::core::ffi::c_int) as ::core::ffi::c_float * ((*q).x * (*q).z - (*q).w * (*q).y))
            as ::core::ffi::c_double,
    ) as ::core::ffi::c_float;
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn kmQuaternionGetRoll(mut q: *const kmQuaternion) -> ::core::ffi::c_float {
    let mut result: ::core::ffi::c_float = atan2(
        (2 as ::core::ffi::c_int as ::core::ffi::c_float * ((*q).x * (*q).y + (*q).w * (*q).z))
            as ::core::ffi::c_double,
        ((*q).w * (*q).w + (*q).x * (*q).x - (*q).y * (*q).y - (*q).z * (*q).z)
            as ::core::ffi::c_double,
    ) as ::core::ffi::c_float;
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn kmQuaternionLookRotation(
    mut pOut: *mut kmQuaternion,
    mut direction: *const kmVec3,
    mut up: *const kmVec3,
) -> *mut kmQuaternion {
    let mut tmp: kmMat3 = kmMat3 { mat: [0.; 9] };
    kmMat3LookAt(
        &raw mut tmp,
        &raw const KM_VEC3_ZERO,
        direction as *const kmVec3,
        up as *const kmVec3,
    );
    return kmQuaternionNormalize(pOut, kmQuaternionRotationMatrix(pOut, &raw mut tmp));
}
