extern "C" {
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memcmp(
        __s1: *const ::core::ffi::c_void,
        __s2: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn cosf(__x: ::core::ffi::c_float) -> ::core::ffi::c_float;
    fn sinf(__x: ::core::ffi::c_float) -> ::core::ffi::c_float;
    fn kmVec3Normalize(pOut: *mut kmVec3, pIn: *const kmVec3) -> *mut kmVec3;
    fn kmVec3Cross(pOut: *mut kmVec3, pV1: *const kmVec3, pV2: *const kmVec3) -> *mut kmVec3;
    fn kmVec3Subtract(pOut: *mut kmVec3, pV1: *const kmVec3, pV2: *const kmVec3) -> *mut kmVec3;
    fn kmVec3Assign(pOut: *mut kmVec3, pIn: *const kmVec3) -> *mut kmVec3;
    fn kmQuaternionRotationMatrix(pOut: *mut kmQuaternion, pIn: *const kmMat3)
        -> *mut kmQuaternion;
    fn kmQuaternionToAxisAngle(
        pIn: *const kmQuaternion,
        pVector: *mut kmVec3,
        pAngle: *mut ::core::ffi::c_float,
    );
}
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kmMat4 {
    pub mat: [::core::ffi::c_float; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kmMat3 {
    pub mat: [::core::ffi::c_float; 9],
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
pub struct kmQuaternion {
    pub x: ::core::ffi::c_float,
    pub y: ::core::ffi::c_float,
    pub z: ::core::ffi::c_float,
    pub w: ::core::ffi::c_float,
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const KM_FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const KM_TRUE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const kmEpsilon: ::core::ffi::c_double = 0.0001f64;
#[no_mangle]
pub unsafe extern "C" fn kmMat3Fill(
    mut pOut: *mut kmMat3,
    mut pMat: *const ::core::ffi::c_float,
) -> *mut kmMat3 {
    memcpy(
        &raw mut (*pOut).mat as *mut ::core::ffi::c_float as *mut ::core::ffi::c_void,
        pMat as *const ::core::ffi::c_void,
        (::core::mem::size_of::<::core::ffi::c_float>() as size_t).wrapping_mul(9 as size_t),
    );
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat3Identity(mut pOut: *mut kmMat3) -> *mut kmMat3 {
    memset(
        &raw mut (*pOut).mat as *mut ::core::ffi::c_float as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        (::core::mem::size_of::<::core::ffi::c_float>() as size_t).wrapping_mul(9 as size_t),
    );
    (*pOut).mat[8 as ::core::ffi::c_int as usize] = 1.0f32;
    (*pOut).mat[4 as ::core::ffi::c_int as usize] = (*pOut).mat[8 as ::core::ffi::c_int as usize];
    (*pOut).mat[0 as ::core::ffi::c_int as usize] = (*pOut).mat[4 as ::core::ffi::c_int as usize];
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat3Determinant(mut pIn: *const kmMat3) -> ::core::ffi::c_float {
    let mut output: ::core::ffi::c_float = 0.;
    output = (*pIn).mat[0 as ::core::ffi::c_int as usize]
        * (*pIn).mat[4 as ::core::ffi::c_int as usize]
        * (*pIn).mat[8 as ::core::ffi::c_int as usize]
        + (*pIn).mat[1 as ::core::ffi::c_int as usize]
            * (*pIn).mat[5 as ::core::ffi::c_int as usize]
            * (*pIn).mat[6 as ::core::ffi::c_int as usize]
        + (*pIn).mat[2 as ::core::ffi::c_int as usize]
            * (*pIn).mat[3 as ::core::ffi::c_int as usize]
            * (*pIn).mat[7 as ::core::ffi::c_int as usize];
    output -= (*pIn).mat[2 as ::core::ffi::c_int as usize]
        * (*pIn).mat[4 as ::core::ffi::c_int as usize]
        * (*pIn).mat[6 as ::core::ffi::c_int as usize]
        + (*pIn).mat[0 as ::core::ffi::c_int as usize]
            * (*pIn).mat[5 as ::core::ffi::c_int as usize]
            * (*pIn).mat[7 as ::core::ffi::c_int as usize]
        + (*pIn).mat[1 as ::core::ffi::c_int as usize]
            * (*pIn).mat[3 as ::core::ffi::c_int as usize]
            * (*pIn).mat[8 as ::core::ffi::c_int as usize];
    return output;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat3Adjugate(
    mut pOut: *mut kmMat3,
    mut pIn: *const kmMat3,
) -> *mut kmMat3 {
    (*pOut).mat[0 as ::core::ffi::c_int as usize] = (*pIn).mat[4 as ::core::ffi::c_int as usize]
        * (*pIn).mat[8 as ::core::ffi::c_int as usize]
        - (*pIn).mat[5 as ::core::ffi::c_int as usize]
            * (*pIn).mat[7 as ::core::ffi::c_int as usize];
    (*pOut).mat[1 as ::core::ffi::c_int as usize] = (*pIn).mat[2 as ::core::ffi::c_int as usize]
        * (*pIn).mat[7 as ::core::ffi::c_int as usize]
        - (*pIn).mat[1 as ::core::ffi::c_int as usize]
            * (*pIn).mat[8 as ::core::ffi::c_int as usize];
    (*pOut).mat[2 as ::core::ffi::c_int as usize] = (*pIn).mat[1 as ::core::ffi::c_int as usize]
        * (*pIn).mat[5 as ::core::ffi::c_int as usize]
        - (*pIn).mat[2 as ::core::ffi::c_int as usize]
            * (*pIn).mat[4 as ::core::ffi::c_int as usize];
    (*pOut).mat[3 as ::core::ffi::c_int as usize] = (*pIn).mat[5 as ::core::ffi::c_int as usize]
        * (*pIn).mat[6 as ::core::ffi::c_int as usize]
        - (*pIn).mat[3 as ::core::ffi::c_int as usize]
            * (*pIn).mat[8 as ::core::ffi::c_int as usize];
    (*pOut).mat[4 as ::core::ffi::c_int as usize] = (*pIn).mat[0 as ::core::ffi::c_int as usize]
        * (*pIn).mat[8 as ::core::ffi::c_int as usize]
        - (*pIn).mat[2 as ::core::ffi::c_int as usize]
            * (*pIn).mat[6 as ::core::ffi::c_int as usize];
    (*pOut).mat[5 as ::core::ffi::c_int as usize] = (*pIn).mat[2 as ::core::ffi::c_int as usize]
        * (*pIn).mat[3 as ::core::ffi::c_int as usize]
        - (*pIn).mat[0 as ::core::ffi::c_int as usize]
            * (*pIn).mat[5 as ::core::ffi::c_int as usize];
    (*pOut).mat[6 as ::core::ffi::c_int as usize] = (*pIn).mat[3 as ::core::ffi::c_int as usize]
        * (*pIn).mat[7 as ::core::ffi::c_int as usize]
        - (*pIn).mat[4 as ::core::ffi::c_int as usize]
            * (*pIn).mat[6 as ::core::ffi::c_int as usize];
    (*pOut).mat[7 as ::core::ffi::c_int as usize] = (*pIn).mat[1 as ::core::ffi::c_int as usize]
        * (*pIn).mat[6 as ::core::ffi::c_int as usize]
        - (*pIn).mat[0 as ::core::ffi::c_int as usize]
            * (*pIn).mat[7 as ::core::ffi::c_int as usize];
    (*pOut).mat[8 as ::core::ffi::c_int as usize] = (*pIn).mat[0 as ::core::ffi::c_int as usize]
        * (*pIn).mat[4 as ::core::ffi::c_int as usize]
        - (*pIn).mat[1 as ::core::ffi::c_int as usize]
            * (*pIn).mat[3 as ::core::ffi::c_int as usize];
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat3Inverse(
    mut pOut: *mut kmMat3,
    mut pM: *const kmMat3,
) -> *mut kmMat3 {
    let mut determinate: ::core::ffi::c_float = kmMat3Determinant(pM);
    let mut detInv: ::core::ffi::c_float = 0.;
    let mut adjugate: kmMat3 = kmMat3 { mat: [0.; 9] };
    if determinate as ::core::ffi::c_double == 0.0f64 {
        return ::core::ptr::null_mut::<kmMat3>();
    }
    detInv = (1.0f64 / determinate as ::core::ffi::c_double) as ::core::ffi::c_float;
    kmMat3Adjugate(&raw mut adjugate, pM);
    kmMat3ScalarMultiply(pOut, &raw mut adjugate, detInv);
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat3IsIdentity(mut pIn: *const kmMat3) -> ::core::ffi::c_int {
    static mut identity: [::core::ffi::c_float; 9] = [
        1.0f32, 0.0f32, 0.0f32, 0.0f32, 1.0f32, 0.0f32, 0.0f32, 0.0f32, 1.0f32,
    ];
    return (memcmp(
        &raw mut identity as *mut ::core::ffi::c_float as *const ::core::ffi::c_void,
        &raw const (*pIn).mat as *const ::core::ffi::c_float as *const ::core::ffi::c_void,
        (::core::mem::size_of::<::core::ffi::c_float>() as size_t).wrapping_mul(9 as size_t),
    ) == 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat3Transpose(
    mut pOut: *mut kmMat3,
    mut pIn: *const kmMat3,
) -> *mut kmMat3 {
    let mut temp: [::core::ffi::c_float; 9] = [0.; 9];
    temp[0 as ::core::ffi::c_int as usize] = (*pIn).mat[0 as ::core::ffi::c_int as usize];
    temp[1 as ::core::ffi::c_int as usize] = (*pIn).mat[3 as ::core::ffi::c_int as usize];
    temp[2 as ::core::ffi::c_int as usize] = (*pIn).mat[6 as ::core::ffi::c_int as usize];
    temp[3 as ::core::ffi::c_int as usize] = (*pIn).mat[1 as ::core::ffi::c_int as usize];
    temp[4 as ::core::ffi::c_int as usize] = (*pIn).mat[4 as ::core::ffi::c_int as usize];
    temp[5 as ::core::ffi::c_int as usize] = (*pIn).mat[7 as ::core::ffi::c_int as usize];
    temp[6 as ::core::ffi::c_int as usize] = (*pIn).mat[2 as ::core::ffi::c_int as usize];
    temp[7 as ::core::ffi::c_int as usize] = (*pIn).mat[5 as ::core::ffi::c_int as usize];
    temp[8 as ::core::ffi::c_int as usize] = (*pIn).mat[8 as ::core::ffi::c_int as usize];
    memcpy(
        &raw mut (*pOut).mat as *mut ::core::ffi::c_void,
        &raw mut temp as *mut ::core::ffi::c_float as *const ::core::ffi::c_void,
        (::core::mem::size_of::<::core::ffi::c_float>() as size_t).wrapping_mul(9 as size_t),
    );
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat3Multiply(
    mut pOut: *mut kmMat3,
    mut pM1: *const kmMat3,
    mut pM2: *const kmMat3,
) -> *mut kmMat3 {
    let mut mat: [::core::ffi::c_float; 9] = [0.; 9];
    let mut m1: *const ::core::ffi::c_float = &raw const (*pM1).mat as *const ::core::ffi::c_float;
    let mut m2: *const ::core::ffi::c_float = &raw const (*pM2).mat as *const ::core::ffi::c_float;
    mat[0 as ::core::ffi::c_int as usize] = *m1.offset(0 as ::core::ffi::c_int as isize)
        * *m2.offset(0 as ::core::ffi::c_int as isize)
        + *m1.offset(3 as ::core::ffi::c_int as isize)
            * *m2.offset(1 as ::core::ffi::c_int as isize)
        + *m1.offset(6 as ::core::ffi::c_int as isize)
            * *m2.offset(2 as ::core::ffi::c_int as isize);
    mat[1 as ::core::ffi::c_int as usize] = *m1.offset(1 as ::core::ffi::c_int as isize)
        * *m2.offset(0 as ::core::ffi::c_int as isize)
        + *m1.offset(4 as ::core::ffi::c_int as isize)
            * *m2.offset(1 as ::core::ffi::c_int as isize)
        + *m1.offset(7 as ::core::ffi::c_int as isize)
            * *m2.offset(2 as ::core::ffi::c_int as isize);
    mat[2 as ::core::ffi::c_int as usize] = *m1.offset(2 as ::core::ffi::c_int as isize)
        * *m2.offset(0 as ::core::ffi::c_int as isize)
        + *m1.offset(5 as ::core::ffi::c_int as isize)
            * *m2.offset(1 as ::core::ffi::c_int as isize)
        + *m1.offset(8 as ::core::ffi::c_int as isize)
            * *m2.offset(2 as ::core::ffi::c_int as isize);
    mat[3 as ::core::ffi::c_int as usize] = *m1.offset(0 as ::core::ffi::c_int as isize)
        * *m2.offset(3 as ::core::ffi::c_int as isize)
        + *m1.offset(3 as ::core::ffi::c_int as isize)
            * *m2.offset(4 as ::core::ffi::c_int as isize)
        + *m1.offset(6 as ::core::ffi::c_int as isize)
            * *m2.offset(5 as ::core::ffi::c_int as isize);
    mat[4 as ::core::ffi::c_int as usize] = *m1.offset(1 as ::core::ffi::c_int as isize)
        * *m2.offset(3 as ::core::ffi::c_int as isize)
        + *m1.offset(4 as ::core::ffi::c_int as isize)
            * *m2.offset(4 as ::core::ffi::c_int as isize)
        + *m1.offset(7 as ::core::ffi::c_int as isize)
            * *m2.offset(5 as ::core::ffi::c_int as isize);
    mat[5 as ::core::ffi::c_int as usize] = *m1.offset(2 as ::core::ffi::c_int as isize)
        * *m2.offset(3 as ::core::ffi::c_int as isize)
        + *m1.offset(5 as ::core::ffi::c_int as isize)
            * *m2.offset(4 as ::core::ffi::c_int as isize)
        + *m1.offset(8 as ::core::ffi::c_int as isize)
            * *m2.offset(5 as ::core::ffi::c_int as isize);
    mat[6 as ::core::ffi::c_int as usize] = *m1.offset(0 as ::core::ffi::c_int as isize)
        * *m2.offset(6 as ::core::ffi::c_int as isize)
        + *m1.offset(3 as ::core::ffi::c_int as isize)
            * *m2.offset(7 as ::core::ffi::c_int as isize)
        + *m1.offset(6 as ::core::ffi::c_int as isize)
            * *m2.offset(8 as ::core::ffi::c_int as isize);
    mat[7 as ::core::ffi::c_int as usize] = *m1.offset(1 as ::core::ffi::c_int as isize)
        * *m2.offset(6 as ::core::ffi::c_int as isize)
        + *m1.offset(4 as ::core::ffi::c_int as isize)
            * *m2.offset(7 as ::core::ffi::c_int as isize)
        + *m1.offset(7 as ::core::ffi::c_int as isize)
            * *m2.offset(8 as ::core::ffi::c_int as isize);
    mat[8 as ::core::ffi::c_int as usize] = *m1.offset(2 as ::core::ffi::c_int as isize)
        * *m2.offset(6 as ::core::ffi::c_int as isize)
        + *m1.offset(5 as ::core::ffi::c_int as isize)
            * *m2.offset(7 as ::core::ffi::c_int as isize)
        + *m1.offset(8 as ::core::ffi::c_int as isize)
            * *m2.offset(8 as ::core::ffi::c_int as isize);
    memcpy(
        &raw mut (*pOut).mat as *mut ::core::ffi::c_float as *mut ::core::ffi::c_void,
        &raw mut mat as *mut ::core::ffi::c_float as *const ::core::ffi::c_void,
        (::core::mem::size_of::<::core::ffi::c_float>() as size_t).wrapping_mul(9 as size_t),
    );
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat3ScalarMultiply(
    mut pOut: *mut kmMat3,
    mut pM: *const kmMat3,
    pFactor: ::core::ffi::c_float,
) -> *mut kmMat3 {
    let mut mat: [::core::ffi::c_float; 9] = [0.; 9];
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < 9 as ::core::ffi::c_int {
        mat[i as usize] = (*pM).mat[i as usize] * pFactor;
        i += 1;
    }
    memcpy(
        &raw mut (*pOut).mat as *mut ::core::ffi::c_float as *mut ::core::ffi::c_void,
        &raw mut mat as *mut ::core::ffi::c_float as *const ::core::ffi::c_void,
        (::core::mem::size_of::<::core::ffi::c_float>() as size_t).wrapping_mul(9 as size_t),
    );
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat3Assign(
    mut pOut: *mut kmMat3,
    mut pIn: *const kmMat3,
) -> *mut kmMat3 {
    memcpy(
        &raw mut (*pOut).mat as *mut ::core::ffi::c_float as *mut ::core::ffi::c_void,
        &raw const (*pIn).mat as *const ::core::ffi::c_float as *const ::core::ffi::c_void,
        (::core::mem::size_of::<::core::ffi::c_float>() as size_t).wrapping_mul(9 as size_t),
    );
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat3AssignMat4(
    mut pOut: *mut kmMat3,
    mut pIn: *const kmMat4,
) -> *mut kmMat3 {
    (*pOut).mat[0 as ::core::ffi::c_int as usize] = (*pIn).mat[0 as ::core::ffi::c_int as usize];
    (*pOut).mat[1 as ::core::ffi::c_int as usize] = (*pIn).mat[1 as ::core::ffi::c_int as usize];
    (*pOut).mat[2 as ::core::ffi::c_int as usize] = (*pIn).mat[2 as ::core::ffi::c_int as usize];
    (*pOut).mat[3 as ::core::ffi::c_int as usize] = (*pIn).mat[4 as ::core::ffi::c_int as usize];
    (*pOut).mat[4 as ::core::ffi::c_int as usize] = (*pIn).mat[5 as ::core::ffi::c_int as usize];
    (*pOut).mat[5 as ::core::ffi::c_int as usize] = (*pIn).mat[6 as ::core::ffi::c_int as usize];
    (*pOut).mat[6 as ::core::ffi::c_int as usize] = (*pIn).mat[8 as ::core::ffi::c_int as usize];
    (*pOut).mat[7 as ::core::ffi::c_int as usize] = (*pIn).mat[9 as ::core::ffi::c_int as usize];
    (*pOut).mat[8 as ::core::ffi::c_int as usize] = (*pIn).mat[10 as ::core::ffi::c_int as usize];
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat3AreEqual(
    mut pMat1: *const kmMat3,
    mut pMat2: *const kmMat3,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    if pMat1 == pMat2 {
        return KM_TRUE;
    }
    i = 0 as ::core::ffi::c_int;
    while i < 9 as ::core::ffi::c_int {
        if !((*pMat1).mat[i as usize] as ::core::ffi::c_double + kmEpsilon
            > (*pMat2).mat[i as usize] as ::core::ffi::c_double
            && (*pMat1).mat[i as usize] as ::core::ffi::c_double - kmEpsilon
                < (*pMat2).mat[i as usize] as ::core::ffi::c_double)
        {
            return KM_FALSE;
        }
        i += 1;
    }
    return KM_TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat3Rotation(
    mut pOut: *mut kmMat3,
    radians: ::core::ffi::c_float,
) -> *mut kmMat3 {
    (*pOut).mat[0 as ::core::ffi::c_int as usize] = cosf(radians);
    (*pOut).mat[1 as ::core::ffi::c_int as usize] = sinf(radians);
    (*pOut).mat[2 as ::core::ffi::c_int as usize] = 0.0f32;
    (*pOut).mat[3 as ::core::ffi::c_int as usize] = -sinf(radians);
    (*pOut).mat[4 as ::core::ffi::c_int as usize] = cosf(radians);
    (*pOut).mat[5 as ::core::ffi::c_int as usize] = 0.0f32;
    (*pOut).mat[6 as ::core::ffi::c_int as usize] = 0.0f32;
    (*pOut).mat[7 as ::core::ffi::c_int as usize] = 0.0f32;
    (*pOut).mat[8 as ::core::ffi::c_int as usize] = 1.0f32;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat3Scaling(
    mut pOut: *mut kmMat3,
    x: ::core::ffi::c_float,
    y: ::core::ffi::c_float,
) -> *mut kmMat3 {
    kmMat3Identity(pOut);
    (*pOut).mat[0 as ::core::ffi::c_int as usize] = x;
    (*pOut).mat[4 as ::core::ffi::c_int as usize] = y;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat3Translation(
    mut pOut: *mut kmMat3,
    x: ::core::ffi::c_float,
    y: ::core::ffi::c_float,
) -> *mut kmMat3 {
    kmMat3Identity(pOut);
    (*pOut).mat[6 as ::core::ffi::c_int as usize] = x;
    (*pOut).mat[7 as ::core::ffi::c_int as usize] = y;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat3RotationQuaternion(
    mut pOut: *mut kmMat3,
    mut pIn: *const kmQuaternion,
) -> *mut kmMat3 {
    if pIn.is_null() || pOut.is_null() {
        return ::core::ptr::null_mut::<kmMat3>();
    }
    (*pOut).mat[0 as ::core::ffi::c_int as usize] =
        1.0f32 - 2.0f32 * ((*pIn).y * (*pIn).y + (*pIn).z * (*pIn).z);
    (*pOut).mat[1 as ::core::ffi::c_int as usize] =
        2.0f32 * ((*pIn).x * (*pIn).y - (*pIn).w * (*pIn).z);
    (*pOut).mat[2 as ::core::ffi::c_int as usize] =
        2.0f32 * ((*pIn).x * (*pIn).z + (*pIn).w * (*pIn).y);
    (*pOut).mat[3 as ::core::ffi::c_int as usize] =
        2.0f32 * ((*pIn).x * (*pIn).y + (*pIn).w * (*pIn).z);
    (*pOut).mat[4 as ::core::ffi::c_int as usize] =
        1.0f32 - 2.0f32 * ((*pIn).x * (*pIn).x + (*pIn).z * (*pIn).z);
    (*pOut).mat[5 as ::core::ffi::c_int as usize] =
        2.0f32 * ((*pIn).y * (*pIn).z - (*pIn).w * (*pIn).x);
    (*pOut).mat[6 as ::core::ffi::c_int as usize] =
        2.0f32 * ((*pIn).x * (*pIn).z - (*pIn).w * (*pIn).y);
    (*pOut).mat[7 as ::core::ffi::c_int as usize] =
        2.0f32 * ((*pIn).y * (*pIn).z + (*pIn).w * (*pIn).x);
    (*pOut).mat[8 as ::core::ffi::c_int as usize] =
        1.0f32 - 2.0f32 * ((*pIn).x * (*pIn).x + (*pIn).y * (*pIn).y);
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat3RotationAxisAngle(
    mut pOut: *mut kmMat3,
    mut axis: *const kmVec3,
    mut radians: ::core::ffi::c_float,
) -> *mut kmMat3 {
    let mut rcos: ::core::ffi::c_float = cosf(radians);
    let mut rsin: ::core::ffi::c_float = sinf(radians);
    (*pOut).mat[0 as ::core::ffi::c_int as usize] =
        rcos + (*axis).x * (*axis).x * (1 as ::core::ffi::c_int as ::core::ffi::c_float - rcos);
    (*pOut).mat[1 as ::core::ffi::c_int as usize] = (*axis).z * rsin
        + (*axis).y * (*axis).x * (1 as ::core::ffi::c_int as ::core::ffi::c_float - rcos);
    (*pOut).mat[2 as ::core::ffi::c_int as usize] = -(*axis).y * rsin
        + (*axis).z * (*axis).x * (1 as ::core::ffi::c_int as ::core::ffi::c_float - rcos);
    (*pOut).mat[3 as ::core::ffi::c_int as usize] = -(*axis).z * rsin
        + (*axis).x * (*axis).y * (1 as ::core::ffi::c_int as ::core::ffi::c_float - rcos);
    (*pOut).mat[4 as ::core::ffi::c_int as usize] =
        rcos + (*axis).y * (*axis).y * (1 as ::core::ffi::c_int as ::core::ffi::c_float - rcos);
    (*pOut).mat[5 as ::core::ffi::c_int as usize] = (*axis).x * rsin
        + (*axis).z * (*axis).y * (1 as ::core::ffi::c_int as ::core::ffi::c_float - rcos);
    (*pOut).mat[6 as ::core::ffi::c_int as usize] = (*axis).y * rsin
        + (*axis).x * (*axis).z * (1 as ::core::ffi::c_int as ::core::ffi::c_float - rcos);
    (*pOut).mat[7 as ::core::ffi::c_int as usize] = -(*axis).x * rsin
        + (*axis).y * (*axis).z * (1 as ::core::ffi::c_int as ::core::ffi::c_float - rcos);
    (*pOut).mat[8 as ::core::ffi::c_int as usize] =
        rcos + (*axis).z * (*axis).z * (1 as ::core::ffi::c_int as ::core::ffi::c_float - rcos);
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat3RotationToAxisAngle(
    mut pAxis: *mut kmVec3,
    mut radians: *mut ::core::ffi::c_float,
    mut pIn: *const kmMat3,
) -> *mut kmVec3 {
    let mut temp: kmQuaternion = kmQuaternion {
        x: 0.,
        y: 0.,
        z: 0.,
        w: 0.,
    };
    kmQuaternionRotationMatrix(&raw mut temp, pIn as *const kmMat3);
    kmQuaternionToAxisAngle(&raw mut temp, pAxis as *mut kmVec3, radians);
    return pAxis as *mut kmVec3;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat3RotationX(
    mut pOut: *mut kmMat3,
    radians: ::core::ffi::c_float,
) -> *mut kmMat3 {
    (*pOut).mat[0 as ::core::ffi::c_int as usize] = 1.0f32;
    (*pOut).mat[1 as ::core::ffi::c_int as usize] = 0.0f32;
    (*pOut).mat[2 as ::core::ffi::c_int as usize] = 0.0f32;
    (*pOut).mat[3 as ::core::ffi::c_int as usize] = 0.0f32;
    (*pOut).mat[4 as ::core::ffi::c_int as usize] = cosf(radians);
    (*pOut).mat[5 as ::core::ffi::c_int as usize] = sinf(radians);
    (*pOut).mat[6 as ::core::ffi::c_int as usize] = 0.0f32;
    (*pOut).mat[7 as ::core::ffi::c_int as usize] = -sinf(radians);
    (*pOut).mat[8 as ::core::ffi::c_int as usize] = cosf(radians);
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat3RotationY(
    mut pOut: *mut kmMat3,
    radians: ::core::ffi::c_float,
) -> *mut kmMat3 {
    (*pOut).mat[0 as ::core::ffi::c_int as usize] = cosf(radians);
    (*pOut).mat[1 as ::core::ffi::c_int as usize] = 0.0f32;
    (*pOut).mat[2 as ::core::ffi::c_int as usize] = -sinf(radians);
    (*pOut).mat[3 as ::core::ffi::c_int as usize] = 0.0f32;
    (*pOut).mat[4 as ::core::ffi::c_int as usize] = 1.0f32;
    (*pOut).mat[5 as ::core::ffi::c_int as usize] = 0.0f32;
    (*pOut).mat[6 as ::core::ffi::c_int as usize] = sinf(radians);
    (*pOut).mat[7 as ::core::ffi::c_int as usize] = 0.0f32;
    (*pOut).mat[8 as ::core::ffi::c_int as usize] = cosf(radians);
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat3RotationZ(
    mut pOut: *mut kmMat3,
    radians: ::core::ffi::c_float,
) -> *mut kmMat3 {
    (*pOut).mat[0 as ::core::ffi::c_int as usize] = cosf(radians);
    (*pOut).mat[1 as ::core::ffi::c_int as usize] = -sinf(radians);
    (*pOut).mat[2 as ::core::ffi::c_int as usize] = 0.0f32;
    (*pOut).mat[3 as ::core::ffi::c_int as usize] = sinf(radians);
    (*pOut).mat[4 as ::core::ffi::c_int as usize] = cosf(radians);
    (*pOut).mat[5 as ::core::ffi::c_int as usize] = 0.0f32;
    (*pOut).mat[6 as ::core::ffi::c_int as usize] = 0.0f32;
    (*pOut).mat[7 as ::core::ffi::c_int as usize] = 0.0f32;
    (*pOut).mat[8 as ::core::ffi::c_int as usize] = 1.0f32;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat3GetUpVec3(
    mut pOut: *mut kmVec3,
    mut pIn: *const kmMat3,
) -> *mut kmVec3 {
    (*pOut).x = (*pIn).mat[3 as ::core::ffi::c_int as usize];
    (*pOut).y = (*pIn).mat[4 as ::core::ffi::c_int as usize];
    (*pOut).z = (*pIn).mat[5 as ::core::ffi::c_int as usize];
    kmVec3Normalize(pOut, pOut);
    return pOut as *mut kmVec3;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat3GetRightVec3(
    mut pOut: *mut kmVec3,
    mut pIn: *const kmMat3,
) -> *mut kmVec3 {
    (*pOut).x = (*pIn).mat[0 as ::core::ffi::c_int as usize];
    (*pOut).y = (*pIn).mat[1 as ::core::ffi::c_int as usize];
    (*pOut).z = (*pIn).mat[2 as ::core::ffi::c_int as usize];
    kmVec3Normalize(pOut, pOut);
    return pOut as *mut kmVec3;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat3GetForwardVec3(
    mut pOut: *mut kmVec3,
    mut pIn: *const kmMat3,
) -> *mut kmVec3 {
    (*pOut).x = (*pIn).mat[6 as ::core::ffi::c_int as usize];
    (*pOut).y = (*pIn).mat[7 as ::core::ffi::c_int as usize];
    (*pOut).z = (*pIn).mat[8 as ::core::ffi::c_int as usize];
    kmVec3Normalize(pOut, pOut);
    return pOut as *mut kmVec3;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat3LookAt(
    mut pOut: *mut kmMat3,
    mut pEye: *const kmVec3,
    mut pCenter: *const kmVec3,
    mut pUp: *const kmVec3,
) -> *mut kmMat3 {
    let mut f: kmVec3 = kmVec3 {
        x: 0.,
        y: 0.,
        z: 0.,
    };
    let mut up: kmVec3 = kmVec3 {
        x: 0.,
        y: 0.,
        z: 0.,
    };
    let mut s: kmVec3 = kmVec3 {
        x: 0.,
        y: 0.,
        z: 0.,
    };
    let mut u: kmVec3 = kmVec3 {
        x: 0.,
        y: 0.,
        z: 0.,
    };
    kmVec3Subtract(&raw mut f, pCenter, pEye);
    kmVec3Normalize(&raw mut f, &raw mut f);
    kmVec3Assign(&raw mut up, pUp);
    kmVec3Normalize(&raw mut up, &raw mut up);
    kmVec3Cross(&raw mut s, &raw mut f, &raw mut up);
    kmVec3Normalize(&raw mut s, &raw mut s);
    kmVec3Cross(&raw mut u, &raw mut s, &raw mut f);
    kmVec3Normalize(&raw mut s, &raw mut s);
    (*pOut).mat[0 as ::core::ffi::c_int as usize] = s.x;
    (*pOut).mat[3 as ::core::ffi::c_int as usize] = s.y;
    (*pOut).mat[6 as ::core::ffi::c_int as usize] = s.z;
    (*pOut).mat[1 as ::core::ffi::c_int as usize] = u.x;
    (*pOut).mat[4 as ::core::ffi::c_int as usize] = u.y;
    (*pOut).mat[7 as ::core::ffi::c_int as usize] = u.z;
    (*pOut).mat[2 as ::core::ffi::c_int as usize] = -f.x;
    (*pOut).mat[5 as ::core::ffi::c_int as usize] = -f.y;
    (*pOut).mat[8 as ::core::ffi::c_int as usize] = -f.z;
    return pOut;
}
