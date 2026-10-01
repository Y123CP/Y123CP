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
    fn cos(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn sin(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn cosf(__x: ::core::ffi::c_float) -> ::core::ffi::c_float;
    fn sinf(__x: ::core::ffi::c_float) -> ::core::ffi::c_float;
    fn sqrtf(__x: ::core::ffi::c_float) -> ::core::ffi::c_float;
    fn kmDegreesToRadians(degrees: ::core::ffi::c_float) -> ::core::ffi::c_float;
    fn kmVec3Normalize(pOut: *mut kmVec3, pIn: *const kmVec3) -> *mut kmVec3;
    fn kmVec3Cross(pOut: *mut kmVec3, pV1: *const kmVec3, pV2: *const kmVec3) -> *mut kmVec3;
    fn kmVec3Subtract(pOut: *mut kmVec3, pV1: *const kmVec3, pV2: *const kmVec3) -> *mut kmVec3;
    fn kmVec3MultiplyMat4(pOut: *mut kmVec3, pV: *const kmVec3, pM: *const kmMat4) -> *mut kmVec3;
    fn kmVec3Assign(pOut: *mut kmVec3, pIn: *const kmVec3) -> *mut kmVec3;
    static KM_VEC3_NEG_Z: kmVec3;
    static KM_VEC3_POS_Z: kmVec3;
    static KM_VEC3_POS_Y: kmVec3;
    static KM_VEC3_POS_X: kmVec3;
    fn kmQuaternionRotationAxisAngle(
        pOut: *mut kmQuaternion,
        pV: *const kmVec3,
        angle: ::core::ffi::c_float,
    ) -> *mut kmQuaternion;
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
pub const KM_PLANE_LEFT: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
pub const KM_PLANE_RIGHT: ::core::ffi::c_uint = 1 as ::core::ffi::c_uint;
pub const KM_PLANE_BOTTOM: ::core::ffi::c_uint = 2 as ::core::ffi::c_uint;
pub const KM_PLANE_TOP: ::core::ffi::c_uint = 3 as ::core::ffi::c_uint;
pub const KM_PLANE_NEAR: ::core::ffi::c_uint = 4 as ::core::ffi::c_uint;
pub const KM_PLANE_FAR: ::core::ffi::c_uint = 5 as ::core::ffi::c_uint;
#[no_mangle]
pub unsafe extern "C" fn kmMat4Fill(
    mut pOut: *mut kmMat4,
    mut pMat: *const ::core::ffi::c_float,
) -> *mut kmMat4 {
    memcpy(
        &raw mut (*pOut).mat as *mut ::core::ffi::c_float as *mut ::core::ffi::c_void,
        pMat as *const ::core::ffi::c_void,
        (::core::mem::size_of::<::core::ffi::c_float>() as size_t).wrapping_mul(16 as size_t),
    );
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat4Identity(mut pOut: *mut kmMat4) -> *mut kmMat4 {
    memset(
        &raw mut (*pOut).mat as *mut ::core::ffi::c_float as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        (::core::mem::size_of::<::core::ffi::c_float>() as size_t).wrapping_mul(16 as size_t),
    );
    (*pOut).mat[15 as ::core::ffi::c_int as usize] = 1.0f32;
    (*pOut).mat[10 as ::core::ffi::c_int as usize] = (*pOut).mat[15 as ::core::ffi::c_int as usize];
    (*pOut).mat[5 as ::core::ffi::c_int as usize] = (*pOut).mat[10 as ::core::ffi::c_int as usize];
    (*pOut).mat[0 as ::core::ffi::c_int as usize] = (*pOut).mat[5 as ::core::ffi::c_int as usize];
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat4Inverse(
    mut pOut: *mut kmMat4,
    mut pM: *const kmMat4,
) -> *mut kmMat4 {
    let mut tmp: kmMat4 = kmMat4 { mat: [0.; 16] };
    let mut det: ::core::ffi::c_double = 0.;
    let mut i: ::core::ffi::c_int = 0;
    tmp.mat[0 as ::core::ffi::c_int as usize] = (*pM).mat[5 as ::core::ffi::c_int as usize]
        * (*pM).mat[10 as ::core::ffi::c_int as usize]
        * (*pM).mat[15 as ::core::ffi::c_int as usize]
        - (*pM).mat[5 as ::core::ffi::c_int as usize]
            * (*pM).mat[11 as ::core::ffi::c_int as usize]
            * (*pM).mat[14 as ::core::ffi::c_int as usize]
        - (*pM).mat[9 as ::core::ffi::c_int as usize]
            * (*pM).mat[6 as ::core::ffi::c_int as usize]
            * (*pM).mat[15 as ::core::ffi::c_int as usize]
        + (*pM).mat[9 as ::core::ffi::c_int as usize]
            * (*pM).mat[7 as ::core::ffi::c_int as usize]
            * (*pM).mat[14 as ::core::ffi::c_int as usize]
        + (*pM).mat[13 as ::core::ffi::c_int as usize]
            * (*pM).mat[6 as ::core::ffi::c_int as usize]
            * (*pM).mat[11 as ::core::ffi::c_int as usize]
        - (*pM).mat[13 as ::core::ffi::c_int as usize]
            * (*pM).mat[7 as ::core::ffi::c_int as usize]
            * (*pM).mat[10 as ::core::ffi::c_int as usize];
    tmp.mat[4 as ::core::ffi::c_int as usize] = -(*pM).mat[4 as ::core::ffi::c_int as usize]
        * (*pM).mat[10 as ::core::ffi::c_int as usize]
        * (*pM).mat[15 as ::core::ffi::c_int as usize]
        + (*pM).mat[4 as ::core::ffi::c_int as usize]
            * (*pM).mat[11 as ::core::ffi::c_int as usize]
            * (*pM).mat[14 as ::core::ffi::c_int as usize]
        + (*pM).mat[8 as ::core::ffi::c_int as usize]
            * (*pM).mat[6 as ::core::ffi::c_int as usize]
            * (*pM).mat[15 as ::core::ffi::c_int as usize]
        - (*pM).mat[8 as ::core::ffi::c_int as usize]
            * (*pM).mat[7 as ::core::ffi::c_int as usize]
            * (*pM).mat[14 as ::core::ffi::c_int as usize]
        - (*pM).mat[12 as ::core::ffi::c_int as usize]
            * (*pM).mat[6 as ::core::ffi::c_int as usize]
            * (*pM).mat[11 as ::core::ffi::c_int as usize]
        + (*pM).mat[12 as ::core::ffi::c_int as usize]
            * (*pM).mat[7 as ::core::ffi::c_int as usize]
            * (*pM).mat[10 as ::core::ffi::c_int as usize];
    tmp.mat[8 as ::core::ffi::c_int as usize] = (*pM).mat[4 as ::core::ffi::c_int as usize]
        * (*pM).mat[9 as ::core::ffi::c_int as usize]
        * (*pM).mat[15 as ::core::ffi::c_int as usize]
        - (*pM).mat[4 as ::core::ffi::c_int as usize]
            * (*pM).mat[11 as ::core::ffi::c_int as usize]
            * (*pM).mat[13 as ::core::ffi::c_int as usize]
        - (*pM).mat[8 as ::core::ffi::c_int as usize]
            * (*pM).mat[5 as ::core::ffi::c_int as usize]
            * (*pM).mat[15 as ::core::ffi::c_int as usize]
        + (*pM).mat[8 as ::core::ffi::c_int as usize]
            * (*pM).mat[7 as ::core::ffi::c_int as usize]
            * (*pM).mat[13 as ::core::ffi::c_int as usize]
        + (*pM).mat[12 as ::core::ffi::c_int as usize]
            * (*pM).mat[5 as ::core::ffi::c_int as usize]
            * (*pM).mat[11 as ::core::ffi::c_int as usize]
        - (*pM).mat[12 as ::core::ffi::c_int as usize]
            * (*pM).mat[7 as ::core::ffi::c_int as usize]
            * (*pM).mat[9 as ::core::ffi::c_int as usize];
    tmp.mat[12 as ::core::ffi::c_int as usize] = -(*pM).mat[4 as ::core::ffi::c_int as usize]
        * (*pM).mat[9 as ::core::ffi::c_int as usize]
        * (*pM).mat[14 as ::core::ffi::c_int as usize]
        + (*pM).mat[4 as ::core::ffi::c_int as usize]
            * (*pM).mat[10 as ::core::ffi::c_int as usize]
            * (*pM).mat[13 as ::core::ffi::c_int as usize]
        + (*pM).mat[8 as ::core::ffi::c_int as usize]
            * (*pM).mat[5 as ::core::ffi::c_int as usize]
            * (*pM).mat[14 as ::core::ffi::c_int as usize]
        - (*pM).mat[8 as ::core::ffi::c_int as usize]
            * (*pM).mat[6 as ::core::ffi::c_int as usize]
            * (*pM).mat[13 as ::core::ffi::c_int as usize]
        - (*pM).mat[12 as ::core::ffi::c_int as usize]
            * (*pM).mat[5 as ::core::ffi::c_int as usize]
            * (*pM).mat[10 as ::core::ffi::c_int as usize]
        + (*pM).mat[12 as ::core::ffi::c_int as usize]
            * (*pM).mat[6 as ::core::ffi::c_int as usize]
            * (*pM).mat[9 as ::core::ffi::c_int as usize];
    tmp.mat[1 as ::core::ffi::c_int as usize] = -(*pM).mat[1 as ::core::ffi::c_int as usize]
        * (*pM).mat[10 as ::core::ffi::c_int as usize]
        * (*pM).mat[15 as ::core::ffi::c_int as usize]
        + (*pM).mat[1 as ::core::ffi::c_int as usize]
            * (*pM).mat[11 as ::core::ffi::c_int as usize]
            * (*pM).mat[14 as ::core::ffi::c_int as usize]
        + (*pM).mat[9 as ::core::ffi::c_int as usize]
            * (*pM).mat[2 as ::core::ffi::c_int as usize]
            * (*pM).mat[15 as ::core::ffi::c_int as usize]
        - (*pM).mat[9 as ::core::ffi::c_int as usize]
            * (*pM).mat[3 as ::core::ffi::c_int as usize]
            * (*pM).mat[14 as ::core::ffi::c_int as usize]
        - (*pM).mat[13 as ::core::ffi::c_int as usize]
            * (*pM).mat[2 as ::core::ffi::c_int as usize]
            * (*pM).mat[11 as ::core::ffi::c_int as usize]
        + (*pM).mat[13 as ::core::ffi::c_int as usize]
            * (*pM).mat[3 as ::core::ffi::c_int as usize]
            * (*pM).mat[10 as ::core::ffi::c_int as usize];
    tmp.mat[5 as ::core::ffi::c_int as usize] = (*pM).mat[0 as ::core::ffi::c_int as usize]
        * (*pM).mat[10 as ::core::ffi::c_int as usize]
        * (*pM).mat[15 as ::core::ffi::c_int as usize]
        - (*pM).mat[0 as ::core::ffi::c_int as usize]
            * (*pM).mat[11 as ::core::ffi::c_int as usize]
            * (*pM).mat[14 as ::core::ffi::c_int as usize]
        - (*pM).mat[8 as ::core::ffi::c_int as usize]
            * (*pM).mat[2 as ::core::ffi::c_int as usize]
            * (*pM).mat[15 as ::core::ffi::c_int as usize]
        + (*pM).mat[8 as ::core::ffi::c_int as usize]
            * (*pM).mat[3 as ::core::ffi::c_int as usize]
            * (*pM).mat[14 as ::core::ffi::c_int as usize]
        + (*pM).mat[12 as ::core::ffi::c_int as usize]
            * (*pM).mat[2 as ::core::ffi::c_int as usize]
            * (*pM).mat[11 as ::core::ffi::c_int as usize]
        - (*pM).mat[12 as ::core::ffi::c_int as usize]
            * (*pM).mat[3 as ::core::ffi::c_int as usize]
            * (*pM).mat[10 as ::core::ffi::c_int as usize];
    tmp.mat[9 as ::core::ffi::c_int as usize] = -(*pM).mat[0 as ::core::ffi::c_int as usize]
        * (*pM).mat[9 as ::core::ffi::c_int as usize]
        * (*pM).mat[15 as ::core::ffi::c_int as usize]
        + (*pM).mat[0 as ::core::ffi::c_int as usize]
            * (*pM).mat[11 as ::core::ffi::c_int as usize]
            * (*pM).mat[13 as ::core::ffi::c_int as usize]
        + (*pM).mat[8 as ::core::ffi::c_int as usize]
            * (*pM).mat[1 as ::core::ffi::c_int as usize]
            * (*pM).mat[15 as ::core::ffi::c_int as usize]
        - (*pM).mat[8 as ::core::ffi::c_int as usize]
            * (*pM).mat[3 as ::core::ffi::c_int as usize]
            * (*pM).mat[13 as ::core::ffi::c_int as usize]
        - (*pM).mat[12 as ::core::ffi::c_int as usize]
            * (*pM).mat[1 as ::core::ffi::c_int as usize]
            * (*pM).mat[11 as ::core::ffi::c_int as usize]
        + (*pM).mat[12 as ::core::ffi::c_int as usize]
            * (*pM).mat[3 as ::core::ffi::c_int as usize]
            * (*pM).mat[9 as ::core::ffi::c_int as usize];
    tmp.mat[13 as ::core::ffi::c_int as usize] = (*pM).mat[0 as ::core::ffi::c_int as usize]
        * (*pM).mat[9 as ::core::ffi::c_int as usize]
        * (*pM).mat[14 as ::core::ffi::c_int as usize]
        - (*pM).mat[0 as ::core::ffi::c_int as usize]
            * (*pM).mat[10 as ::core::ffi::c_int as usize]
            * (*pM).mat[13 as ::core::ffi::c_int as usize]
        - (*pM).mat[8 as ::core::ffi::c_int as usize]
            * (*pM).mat[1 as ::core::ffi::c_int as usize]
            * (*pM).mat[14 as ::core::ffi::c_int as usize]
        + (*pM).mat[8 as ::core::ffi::c_int as usize]
            * (*pM).mat[2 as ::core::ffi::c_int as usize]
            * (*pM).mat[13 as ::core::ffi::c_int as usize]
        + (*pM).mat[12 as ::core::ffi::c_int as usize]
            * (*pM).mat[1 as ::core::ffi::c_int as usize]
            * (*pM).mat[10 as ::core::ffi::c_int as usize]
        - (*pM).mat[12 as ::core::ffi::c_int as usize]
            * (*pM).mat[2 as ::core::ffi::c_int as usize]
            * (*pM).mat[9 as ::core::ffi::c_int as usize];
    tmp.mat[2 as ::core::ffi::c_int as usize] = (*pM).mat[1 as ::core::ffi::c_int as usize]
        * (*pM).mat[6 as ::core::ffi::c_int as usize]
        * (*pM).mat[15 as ::core::ffi::c_int as usize]
        - (*pM).mat[1 as ::core::ffi::c_int as usize]
            * (*pM).mat[7 as ::core::ffi::c_int as usize]
            * (*pM).mat[14 as ::core::ffi::c_int as usize]
        - (*pM).mat[5 as ::core::ffi::c_int as usize]
            * (*pM).mat[2 as ::core::ffi::c_int as usize]
            * (*pM).mat[15 as ::core::ffi::c_int as usize]
        + (*pM).mat[5 as ::core::ffi::c_int as usize]
            * (*pM).mat[3 as ::core::ffi::c_int as usize]
            * (*pM).mat[14 as ::core::ffi::c_int as usize]
        + (*pM).mat[13 as ::core::ffi::c_int as usize]
            * (*pM).mat[2 as ::core::ffi::c_int as usize]
            * (*pM).mat[7 as ::core::ffi::c_int as usize]
        - (*pM).mat[13 as ::core::ffi::c_int as usize]
            * (*pM).mat[3 as ::core::ffi::c_int as usize]
            * (*pM).mat[6 as ::core::ffi::c_int as usize];
    tmp.mat[6 as ::core::ffi::c_int as usize] = -(*pM).mat[0 as ::core::ffi::c_int as usize]
        * (*pM).mat[6 as ::core::ffi::c_int as usize]
        * (*pM).mat[15 as ::core::ffi::c_int as usize]
        + (*pM).mat[0 as ::core::ffi::c_int as usize]
            * (*pM).mat[7 as ::core::ffi::c_int as usize]
            * (*pM).mat[14 as ::core::ffi::c_int as usize]
        + (*pM).mat[4 as ::core::ffi::c_int as usize]
            * (*pM).mat[2 as ::core::ffi::c_int as usize]
            * (*pM).mat[15 as ::core::ffi::c_int as usize]
        - (*pM).mat[4 as ::core::ffi::c_int as usize]
            * (*pM).mat[3 as ::core::ffi::c_int as usize]
            * (*pM).mat[14 as ::core::ffi::c_int as usize]
        - (*pM).mat[12 as ::core::ffi::c_int as usize]
            * (*pM).mat[2 as ::core::ffi::c_int as usize]
            * (*pM).mat[7 as ::core::ffi::c_int as usize]
        + (*pM).mat[12 as ::core::ffi::c_int as usize]
            * (*pM).mat[3 as ::core::ffi::c_int as usize]
            * (*pM).mat[6 as ::core::ffi::c_int as usize];
    tmp.mat[10 as ::core::ffi::c_int as usize] = (*pM).mat[0 as ::core::ffi::c_int as usize]
        * (*pM).mat[5 as ::core::ffi::c_int as usize]
        * (*pM).mat[15 as ::core::ffi::c_int as usize]
        - (*pM).mat[0 as ::core::ffi::c_int as usize]
            * (*pM).mat[7 as ::core::ffi::c_int as usize]
            * (*pM).mat[13 as ::core::ffi::c_int as usize]
        - (*pM).mat[4 as ::core::ffi::c_int as usize]
            * (*pM).mat[1 as ::core::ffi::c_int as usize]
            * (*pM).mat[15 as ::core::ffi::c_int as usize]
        + (*pM).mat[4 as ::core::ffi::c_int as usize]
            * (*pM).mat[3 as ::core::ffi::c_int as usize]
            * (*pM).mat[13 as ::core::ffi::c_int as usize]
        + (*pM).mat[12 as ::core::ffi::c_int as usize]
            * (*pM).mat[1 as ::core::ffi::c_int as usize]
            * (*pM).mat[7 as ::core::ffi::c_int as usize]
        - (*pM).mat[12 as ::core::ffi::c_int as usize]
            * (*pM).mat[3 as ::core::ffi::c_int as usize]
            * (*pM).mat[5 as ::core::ffi::c_int as usize];
    tmp.mat[14 as ::core::ffi::c_int as usize] = -(*pM).mat[0 as ::core::ffi::c_int as usize]
        * (*pM).mat[5 as ::core::ffi::c_int as usize]
        * (*pM).mat[14 as ::core::ffi::c_int as usize]
        + (*pM).mat[0 as ::core::ffi::c_int as usize]
            * (*pM).mat[6 as ::core::ffi::c_int as usize]
            * (*pM).mat[13 as ::core::ffi::c_int as usize]
        + (*pM).mat[4 as ::core::ffi::c_int as usize]
            * (*pM).mat[1 as ::core::ffi::c_int as usize]
            * (*pM).mat[14 as ::core::ffi::c_int as usize]
        - (*pM).mat[4 as ::core::ffi::c_int as usize]
            * (*pM).mat[2 as ::core::ffi::c_int as usize]
            * (*pM).mat[13 as ::core::ffi::c_int as usize]
        - (*pM).mat[12 as ::core::ffi::c_int as usize]
            * (*pM).mat[1 as ::core::ffi::c_int as usize]
            * (*pM).mat[6 as ::core::ffi::c_int as usize]
        + (*pM).mat[12 as ::core::ffi::c_int as usize]
            * (*pM).mat[2 as ::core::ffi::c_int as usize]
            * (*pM).mat[5 as ::core::ffi::c_int as usize];
    tmp.mat[3 as ::core::ffi::c_int as usize] = -(*pM).mat[1 as ::core::ffi::c_int as usize]
        * (*pM).mat[6 as ::core::ffi::c_int as usize]
        * (*pM).mat[11 as ::core::ffi::c_int as usize]
        + (*pM).mat[1 as ::core::ffi::c_int as usize]
            * (*pM).mat[7 as ::core::ffi::c_int as usize]
            * (*pM).mat[10 as ::core::ffi::c_int as usize]
        + (*pM).mat[5 as ::core::ffi::c_int as usize]
            * (*pM).mat[2 as ::core::ffi::c_int as usize]
            * (*pM).mat[11 as ::core::ffi::c_int as usize]
        - (*pM).mat[5 as ::core::ffi::c_int as usize]
            * (*pM).mat[3 as ::core::ffi::c_int as usize]
            * (*pM).mat[10 as ::core::ffi::c_int as usize]
        - (*pM).mat[9 as ::core::ffi::c_int as usize]
            * (*pM).mat[2 as ::core::ffi::c_int as usize]
            * (*pM).mat[7 as ::core::ffi::c_int as usize]
        + (*pM).mat[9 as ::core::ffi::c_int as usize]
            * (*pM).mat[3 as ::core::ffi::c_int as usize]
            * (*pM).mat[6 as ::core::ffi::c_int as usize];
    tmp.mat[7 as ::core::ffi::c_int as usize] = (*pM).mat[0 as ::core::ffi::c_int as usize]
        * (*pM).mat[6 as ::core::ffi::c_int as usize]
        * (*pM).mat[11 as ::core::ffi::c_int as usize]
        - (*pM).mat[0 as ::core::ffi::c_int as usize]
            * (*pM).mat[7 as ::core::ffi::c_int as usize]
            * (*pM).mat[10 as ::core::ffi::c_int as usize]
        - (*pM).mat[4 as ::core::ffi::c_int as usize]
            * (*pM).mat[2 as ::core::ffi::c_int as usize]
            * (*pM).mat[11 as ::core::ffi::c_int as usize]
        + (*pM).mat[4 as ::core::ffi::c_int as usize]
            * (*pM).mat[3 as ::core::ffi::c_int as usize]
            * (*pM).mat[10 as ::core::ffi::c_int as usize]
        + (*pM).mat[8 as ::core::ffi::c_int as usize]
            * (*pM).mat[2 as ::core::ffi::c_int as usize]
            * (*pM).mat[7 as ::core::ffi::c_int as usize]
        - (*pM).mat[8 as ::core::ffi::c_int as usize]
            * (*pM).mat[3 as ::core::ffi::c_int as usize]
            * (*pM).mat[6 as ::core::ffi::c_int as usize];
    tmp.mat[11 as ::core::ffi::c_int as usize] = -(*pM).mat[0 as ::core::ffi::c_int as usize]
        * (*pM).mat[5 as ::core::ffi::c_int as usize]
        * (*pM).mat[11 as ::core::ffi::c_int as usize]
        + (*pM).mat[0 as ::core::ffi::c_int as usize]
            * (*pM).mat[7 as ::core::ffi::c_int as usize]
            * (*pM).mat[9 as ::core::ffi::c_int as usize]
        + (*pM).mat[4 as ::core::ffi::c_int as usize]
            * (*pM).mat[1 as ::core::ffi::c_int as usize]
            * (*pM).mat[11 as ::core::ffi::c_int as usize]
        - (*pM).mat[4 as ::core::ffi::c_int as usize]
            * (*pM).mat[3 as ::core::ffi::c_int as usize]
            * (*pM).mat[9 as ::core::ffi::c_int as usize]
        - (*pM).mat[8 as ::core::ffi::c_int as usize]
            * (*pM).mat[1 as ::core::ffi::c_int as usize]
            * (*pM).mat[7 as ::core::ffi::c_int as usize]
        + (*pM).mat[8 as ::core::ffi::c_int as usize]
            * (*pM).mat[3 as ::core::ffi::c_int as usize]
            * (*pM).mat[5 as ::core::ffi::c_int as usize];
    tmp.mat[15 as ::core::ffi::c_int as usize] = (*pM).mat[0 as ::core::ffi::c_int as usize]
        * (*pM).mat[5 as ::core::ffi::c_int as usize]
        * (*pM).mat[10 as ::core::ffi::c_int as usize]
        - (*pM).mat[0 as ::core::ffi::c_int as usize]
            * (*pM).mat[6 as ::core::ffi::c_int as usize]
            * (*pM).mat[9 as ::core::ffi::c_int as usize]
        - (*pM).mat[4 as ::core::ffi::c_int as usize]
            * (*pM).mat[1 as ::core::ffi::c_int as usize]
            * (*pM).mat[10 as ::core::ffi::c_int as usize]
        + (*pM).mat[4 as ::core::ffi::c_int as usize]
            * (*pM).mat[2 as ::core::ffi::c_int as usize]
            * (*pM).mat[9 as ::core::ffi::c_int as usize]
        + (*pM).mat[8 as ::core::ffi::c_int as usize]
            * (*pM).mat[1 as ::core::ffi::c_int as usize]
            * (*pM).mat[6 as ::core::ffi::c_int as usize]
        - (*pM).mat[8 as ::core::ffi::c_int as usize]
            * (*pM).mat[2 as ::core::ffi::c_int as usize]
            * (*pM).mat[5 as ::core::ffi::c_int as usize];
    det = ((*pM).mat[0 as ::core::ffi::c_int as usize] * tmp.mat[0 as ::core::ffi::c_int as usize]
        + (*pM).mat[1 as ::core::ffi::c_int as usize] * tmp.mat[4 as ::core::ffi::c_int as usize]
        + (*pM).mat[2 as ::core::ffi::c_int as usize] * tmp.mat[8 as ::core::ffi::c_int as usize]
        + (*pM).mat[3 as ::core::ffi::c_int as usize] * tmp.mat[12 as ::core::ffi::c_int as usize])
        as ::core::ffi::c_double;
    if det == 0 as ::core::ffi::c_int as ::core::ffi::c_double {
        return ::core::ptr::null_mut::<kmMat4>();
    }
    det = 1.0f64 / det;
    i = 0 as ::core::ffi::c_int;
    while i < 16 as ::core::ffi::c_int {
        (*pOut).mat[i as usize] =
            (tmp.mat[i as usize] as ::core::ffi::c_double * det) as ::core::ffi::c_float;
        i += 1;
    }
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat4IsIdentity(mut pIn: *const kmMat4) -> ::core::ffi::c_int {
    static mut identity: [::core::ffi::c_float; 16] = [
        1.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32, 1.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32, 1.0f32,
        0.0f32, 0.0f32, 0.0f32, 0.0f32, 1.0f32,
    ];
    return (memcmp(
        &raw mut identity as *mut ::core::ffi::c_float as *const ::core::ffi::c_void,
        &raw const (*pIn).mat as *const ::core::ffi::c_float as *const ::core::ffi::c_void,
        (::core::mem::size_of::<::core::ffi::c_float>() as size_t).wrapping_mul(16 as size_t),
    ) == 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat4Transpose(
    mut pOut: *mut kmMat4,
    mut pIn: *const kmMat4,
) -> *mut kmMat4 {
    let mut x: ::core::ffi::c_int = 0;
    let mut z: ::core::ffi::c_int = 0;
    z = 0 as ::core::ffi::c_int;
    while z < 4 as ::core::ffi::c_int {
        x = 0 as ::core::ffi::c_int;
        while x < 4 as ::core::ffi::c_int {
            (*pOut).mat[(z * 4 as ::core::ffi::c_int + x) as usize] =
                (*pIn).mat[(x * 4 as ::core::ffi::c_int + z) as usize];
            x += 1;
        }
        z += 1;
    }
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat4Multiply(
    mut pOut: *mut kmMat4,
    mut pM1: *const kmMat4,
    mut pM2: *const kmMat4,
) -> *mut kmMat4 {
    let mut mat: [::core::ffi::c_float; 16] = [0.; 16];
    let mut m1: *const ::core::ffi::c_float = &raw const (*pM1).mat as *const ::core::ffi::c_float;
    let mut m2: *const ::core::ffi::c_float = &raw const (*pM2).mat as *const ::core::ffi::c_float;
    mat[0 as ::core::ffi::c_int as usize] = *m1.offset(0 as ::core::ffi::c_int as isize)
        * *m2.offset(0 as ::core::ffi::c_int as isize)
        + *m1.offset(4 as ::core::ffi::c_int as isize)
            * *m2.offset(1 as ::core::ffi::c_int as isize)
        + *m1.offset(8 as ::core::ffi::c_int as isize)
            * *m2.offset(2 as ::core::ffi::c_int as isize)
        + *m1.offset(12 as ::core::ffi::c_int as isize)
            * *m2.offset(3 as ::core::ffi::c_int as isize);
    mat[1 as ::core::ffi::c_int as usize] = *m1.offset(1 as ::core::ffi::c_int as isize)
        * *m2.offset(0 as ::core::ffi::c_int as isize)
        + *m1.offset(5 as ::core::ffi::c_int as isize)
            * *m2.offset(1 as ::core::ffi::c_int as isize)
        + *m1.offset(9 as ::core::ffi::c_int as isize)
            * *m2.offset(2 as ::core::ffi::c_int as isize)
        + *m1.offset(13 as ::core::ffi::c_int as isize)
            * *m2.offset(3 as ::core::ffi::c_int as isize);
    mat[2 as ::core::ffi::c_int as usize] = *m1.offset(2 as ::core::ffi::c_int as isize)
        * *m2.offset(0 as ::core::ffi::c_int as isize)
        + *m1.offset(6 as ::core::ffi::c_int as isize)
            * *m2.offset(1 as ::core::ffi::c_int as isize)
        + *m1.offset(10 as ::core::ffi::c_int as isize)
            * *m2.offset(2 as ::core::ffi::c_int as isize)
        + *m1.offset(14 as ::core::ffi::c_int as isize)
            * *m2.offset(3 as ::core::ffi::c_int as isize);
    mat[3 as ::core::ffi::c_int as usize] = *m1.offset(3 as ::core::ffi::c_int as isize)
        * *m2.offset(0 as ::core::ffi::c_int as isize)
        + *m1.offset(7 as ::core::ffi::c_int as isize)
            * *m2.offset(1 as ::core::ffi::c_int as isize)
        + *m1.offset(11 as ::core::ffi::c_int as isize)
            * *m2.offset(2 as ::core::ffi::c_int as isize)
        + *m1.offset(15 as ::core::ffi::c_int as isize)
            * *m2.offset(3 as ::core::ffi::c_int as isize);
    mat[4 as ::core::ffi::c_int as usize] = *m1.offset(0 as ::core::ffi::c_int as isize)
        * *m2.offset(4 as ::core::ffi::c_int as isize)
        + *m1.offset(4 as ::core::ffi::c_int as isize)
            * *m2.offset(5 as ::core::ffi::c_int as isize)
        + *m1.offset(8 as ::core::ffi::c_int as isize)
            * *m2.offset(6 as ::core::ffi::c_int as isize)
        + *m1.offset(12 as ::core::ffi::c_int as isize)
            * *m2.offset(7 as ::core::ffi::c_int as isize);
    mat[5 as ::core::ffi::c_int as usize] = *m1.offset(1 as ::core::ffi::c_int as isize)
        * *m2.offset(4 as ::core::ffi::c_int as isize)
        + *m1.offset(5 as ::core::ffi::c_int as isize)
            * *m2.offset(5 as ::core::ffi::c_int as isize)
        + *m1.offset(9 as ::core::ffi::c_int as isize)
            * *m2.offset(6 as ::core::ffi::c_int as isize)
        + *m1.offset(13 as ::core::ffi::c_int as isize)
            * *m2.offset(7 as ::core::ffi::c_int as isize);
    mat[6 as ::core::ffi::c_int as usize] = *m1.offset(2 as ::core::ffi::c_int as isize)
        * *m2.offset(4 as ::core::ffi::c_int as isize)
        + *m1.offset(6 as ::core::ffi::c_int as isize)
            * *m2.offset(5 as ::core::ffi::c_int as isize)
        + *m1.offset(10 as ::core::ffi::c_int as isize)
            * *m2.offset(6 as ::core::ffi::c_int as isize)
        + *m1.offset(14 as ::core::ffi::c_int as isize)
            * *m2.offset(7 as ::core::ffi::c_int as isize);
    mat[7 as ::core::ffi::c_int as usize] = *m1.offset(3 as ::core::ffi::c_int as isize)
        * *m2.offset(4 as ::core::ffi::c_int as isize)
        + *m1.offset(7 as ::core::ffi::c_int as isize)
            * *m2.offset(5 as ::core::ffi::c_int as isize)
        + *m1.offset(11 as ::core::ffi::c_int as isize)
            * *m2.offset(6 as ::core::ffi::c_int as isize)
        + *m1.offset(15 as ::core::ffi::c_int as isize)
            * *m2.offset(7 as ::core::ffi::c_int as isize);
    mat[8 as ::core::ffi::c_int as usize] = *m1.offset(0 as ::core::ffi::c_int as isize)
        * *m2.offset(8 as ::core::ffi::c_int as isize)
        + *m1.offset(4 as ::core::ffi::c_int as isize)
            * *m2.offset(9 as ::core::ffi::c_int as isize)
        + *m1.offset(8 as ::core::ffi::c_int as isize)
            * *m2.offset(10 as ::core::ffi::c_int as isize)
        + *m1.offset(12 as ::core::ffi::c_int as isize)
            * *m2.offset(11 as ::core::ffi::c_int as isize);
    mat[9 as ::core::ffi::c_int as usize] = *m1.offset(1 as ::core::ffi::c_int as isize)
        * *m2.offset(8 as ::core::ffi::c_int as isize)
        + *m1.offset(5 as ::core::ffi::c_int as isize)
            * *m2.offset(9 as ::core::ffi::c_int as isize)
        + *m1.offset(9 as ::core::ffi::c_int as isize)
            * *m2.offset(10 as ::core::ffi::c_int as isize)
        + *m1.offset(13 as ::core::ffi::c_int as isize)
            * *m2.offset(11 as ::core::ffi::c_int as isize);
    mat[10 as ::core::ffi::c_int as usize] = *m1.offset(2 as ::core::ffi::c_int as isize)
        * *m2.offset(8 as ::core::ffi::c_int as isize)
        + *m1.offset(6 as ::core::ffi::c_int as isize)
            * *m2.offset(9 as ::core::ffi::c_int as isize)
        + *m1.offset(10 as ::core::ffi::c_int as isize)
            * *m2.offset(10 as ::core::ffi::c_int as isize)
        + *m1.offset(14 as ::core::ffi::c_int as isize)
            * *m2.offset(11 as ::core::ffi::c_int as isize);
    mat[11 as ::core::ffi::c_int as usize] = *m1.offset(3 as ::core::ffi::c_int as isize)
        * *m2.offset(8 as ::core::ffi::c_int as isize)
        + *m1.offset(7 as ::core::ffi::c_int as isize)
            * *m2.offset(9 as ::core::ffi::c_int as isize)
        + *m1.offset(11 as ::core::ffi::c_int as isize)
            * *m2.offset(10 as ::core::ffi::c_int as isize)
        + *m1.offset(15 as ::core::ffi::c_int as isize)
            * *m2.offset(11 as ::core::ffi::c_int as isize);
    mat[12 as ::core::ffi::c_int as usize] = *m1.offset(0 as ::core::ffi::c_int as isize)
        * *m2.offset(12 as ::core::ffi::c_int as isize)
        + *m1.offset(4 as ::core::ffi::c_int as isize)
            * *m2.offset(13 as ::core::ffi::c_int as isize)
        + *m1.offset(8 as ::core::ffi::c_int as isize)
            * *m2.offset(14 as ::core::ffi::c_int as isize)
        + *m1.offset(12 as ::core::ffi::c_int as isize)
            * *m2.offset(15 as ::core::ffi::c_int as isize);
    mat[13 as ::core::ffi::c_int as usize] = *m1.offset(1 as ::core::ffi::c_int as isize)
        * *m2.offset(12 as ::core::ffi::c_int as isize)
        + *m1.offset(5 as ::core::ffi::c_int as isize)
            * *m2.offset(13 as ::core::ffi::c_int as isize)
        + *m1.offset(9 as ::core::ffi::c_int as isize)
            * *m2.offset(14 as ::core::ffi::c_int as isize)
        + *m1.offset(13 as ::core::ffi::c_int as isize)
            * *m2.offset(15 as ::core::ffi::c_int as isize);
    mat[14 as ::core::ffi::c_int as usize] = *m1.offset(2 as ::core::ffi::c_int as isize)
        * *m2.offset(12 as ::core::ffi::c_int as isize)
        + *m1.offset(6 as ::core::ffi::c_int as isize)
            * *m2.offset(13 as ::core::ffi::c_int as isize)
        + *m1.offset(10 as ::core::ffi::c_int as isize)
            * *m2.offset(14 as ::core::ffi::c_int as isize)
        + *m1.offset(14 as ::core::ffi::c_int as isize)
            * *m2.offset(15 as ::core::ffi::c_int as isize);
    mat[15 as ::core::ffi::c_int as usize] = *m1.offset(3 as ::core::ffi::c_int as isize)
        * *m2.offset(12 as ::core::ffi::c_int as isize)
        + *m1.offset(7 as ::core::ffi::c_int as isize)
            * *m2.offset(13 as ::core::ffi::c_int as isize)
        + *m1.offset(11 as ::core::ffi::c_int as isize)
            * *m2.offset(14 as ::core::ffi::c_int as isize)
        + *m1.offset(15 as ::core::ffi::c_int as isize)
            * *m2.offset(15 as ::core::ffi::c_int as isize);
    memcpy(
        &raw mut (*pOut).mat as *mut ::core::ffi::c_float as *mut ::core::ffi::c_void,
        &raw mut mat as *mut ::core::ffi::c_float as *const ::core::ffi::c_void,
        (::core::mem::size_of::<::core::ffi::c_float>() as size_t).wrapping_mul(16 as size_t),
    );
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat4Assign(
    mut pOut: *mut kmMat4,
    mut pIn: *const kmMat4,
) -> *mut kmMat4 {
    memcpy(
        &raw mut (*pOut).mat as *mut ::core::ffi::c_float as *mut ::core::ffi::c_void,
        &raw const (*pIn).mat as *const ::core::ffi::c_float as *const ::core::ffi::c_void,
        (::core::mem::size_of::<::core::ffi::c_float>() as size_t).wrapping_mul(16 as size_t),
    );
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat4AssignMat3(
    mut pOut: *mut kmMat4,
    mut pIn: *const kmMat3,
) -> *mut kmMat4 {
    kmMat4Identity(pOut);
    (*pOut).mat[0 as ::core::ffi::c_int as usize] = (*pIn).mat[0 as ::core::ffi::c_int as usize];
    (*pOut).mat[1 as ::core::ffi::c_int as usize] = (*pIn).mat[1 as ::core::ffi::c_int as usize];
    (*pOut).mat[2 as ::core::ffi::c_int as usize] = (*pIn).mat[2 as ::core::ffi::c_int as usize];
    (*pOut).mat[3 as ::core::ffi::c_int as usize] = 0.0f32;
    (*pOut).mat[4 as ::core::ffi::c_int as usize] = (*pIn).mat[3 as ::core::ffi::c_int as usize];
    (*pOut).mat[5 as ::core::ffi::c_int as usize] = (*pIn).mat[4 as ::core::ffi::c_int as usize];
    (*pOut).mat[6 as ::core::ffi::c_int as usize] = (*pIn).mat[5 as ::core::ffi::c_int as usize];
    (*pOut).mat[7 as ::core::ffi::c_int as usize] = 0.0f32;
    (*pOut).mat[8 as ::core::ffi::c_int as usize] = (*pIn).mat[6 as ::core::ffi::c_int as usize];
    (*pOut).mat[9 as ::core::ffi::c_int as usize] = (*pIn).mat[7 as ::core::ffi::c_int as usize];
    (*pOut).mat[10 as ::core::ffi::c_int as usize] = (*pIn).mat[8 as ::core::ffi::c_int as usize];
    (*pOut).mat[11 as ::core::ffi::c_int as usize] = 0.0f32;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat4AreEqual(
    mut pMat1: *const kmMat4,
    mut pMat2: *const kmMat4,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while i < 16 as ::core::ffi::c_int {
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
pub unsafe extern "C" fn kmMat4RotationAxisAngle(
    mut pOut: *mut kmMat4,
    mut axis: *const kmVec3,
    mut radians: ::core::ffi::c_float,
) -> *mut kmMat4 {
    let mut quat: kmQuaternion = kmQuaternion {
        x: 0.,
        y: 0.,
        z: 0.,
        w: 0.,
    };
    kmQuaternionRotationAxisAngle(&raw mut quat, axis as *const kmVec3, radians);
    kmMat4RotationQuaternion(pOut, &raw mut quat);
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat4RotationX(
    mut pOut: *mut kmMat4,
    radians: ::core::ffi::c_float,
) -> *mut kmMat4 {
    (*pOut).mat[0 as ::core::ffi::c_int as usize] = 1.0f32;
    (*pOut).mat[1 as ::core::ffi::c_int as usize] = 0.0f32;
    (*pOut).mat[2 as ::core::ffi::c_int as usize] = 0.0f32;
    (*pOut).mat[3 as ::core::ffi::c_int as usize] = 0.0f32;
    (*pOut).mat[4 as ::core::ffi::c_int as usize] = 0.0f32;
    (*pOut).mat[5 as ::core::ffi::c_int as usize] = cosf(radians);
    (*pOut).mat[6 as ::core::ffi::c_int as usize] = sinf(radians);
    (*pOut).mat[7 as ::core::ffi::c_int as usize] = 0.0f32;
    (*pOut).mat[8 as ::core::ffi::c_int as usize] = 0.0f32;
    (*pOut).mat[9 as ::core::ffi::c_int as usize] = -sinf(radians);
    (*pOut).mat[10 as ::core::ffi::c_int as usize] = cosf(radians);
    (*pOut).mat[11 as ::core::ffi::c_int as usize] = 0.0f32;
    (*pOut).mat[12 as ::core::ffi::c_int as usize] = 0.0f32;
    (*pOut).mat[13 as ::core::ffi::c_int as usize] = 0.0f32;
    (*pOut).mat[14 as ::core::ffi::c_int as usize] = 0.0f32;
    (*pOut).mat[15 as ::core::ffi::c_int as usize] = 1.0f32;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat4RotationY(
    mut pOut: *mut kmMat4,
    radians: ::core::ffi::c_float,
) -> *mut kmMat4 {
    (*pOut).mat[0 as ::core::ffi::c_int as usize] = cosf(radians);
    (*pOut).mat[1 as ::core::ffi::c_int as usize] = 0.0f32;
    (*pOut).mat[2 as ::core::ffi::c_int as usize] = -sinf(radians);
    (*pOut).mat[3 as ::core::ffi::c_int as usize] = 0.0f32;
    (*pOut).mat[4 as ::core::ffi::c_int as usize] = 0.0f32;
    (*pOut).mat[5 as ::core::ffi::c_int as usize] = 1.0f32;
    (*pOut).mat[6 as ::core::ffi::c_int as usize] = 0.0f32;
    (*pOut).mat[7 as ::core::ffi::c_int as usize] = 0.0f32;
    (*pOut).mat[8 as ::core::ffi::c_int as usize] = sinf(radians);
    (*pOut).mat[9 as ::core::ffi::c_int as usize] = 0.0f32;
    (*pOut).mat[10 as ::core::ffi::c_int as usize] = cosf(radians);
    (*pOut).mat[11 as ::core::ffi::c_int as usize] = 0.0f32;
    (*pOut).mat[12 as ::core::ffi::c_int as usize] = 0.0f32;
    (*pOut).mat[13 as ::core::ffi::c_int as usize] = 0.0f32;
    (*pOut).mat[14 as ::core::ffi::c_int as usize] = 0.0f32;
    (*pOut).mat[15 as ::core::ffi::c_int as usize] = 1.0f32;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat4RotationZ(
    mut pOut: *mut kmMat4,
    radians: ::core::ffi::c_float,
) -> *mut kmMat4 {
    (*pOut).mat[0 as ::core::ffi::c_int as usize] = cosf(radians);
    (*pOut).mat[1 as ::core::ffi::c_int as usize] = sinf(radians);
    (*pOut).mat[2 as ::core::ffi::c_int as usize] = 0.0f32;
    (*pOut).mat[3 as ::core::ffi::c_int as usize] = 0.0f32;
    (*pOut).mat[4 as ::core::ffi::c_int as usize] = -sinf(radians);
    (*pOut).mat[5 as ::core::ffi::c_int as usize] = cosf(radians);
    (*pOut).mat[6 as ::core::ffi::c_int as usize] = 0.0f32;
    (*pOut).mat[7 as ::core::ffi::c_int as usize] = 0.0f32;
    (*pOut).mat[8 as ::core::ffi::c_int as usize] = 0.0f32;
    (*pOut).mat[9 as ::core::ffi::c_int as usize] = 0.0f32;
    (*pOut).mat[10 as ::core::ffi::c_int as usize] = 1.0f32;
    (*pOut).mat[11 as ::core::ffi::c_int as usize] = 0.0f32;
    (*pOut).mat[12 as ::core::ffi::c_int as usize] = 0.0f32;
    (*pOut).mat[13 as ::core::ffi::c_int as usize] = 0.0f32;
    (*pOut).mat[14 as ::core::ffi::c_int as usize] = 0.0f32;
    (*pOut).mat[15 as ::core::ffi::c_int as usize] = 1.0f32;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat4RotationYawPitchRoll(
    mut pOut: *mut kmMat4,
    pitch: ::core::ffi::c_float,
    yaw: ::core::ffi::c_float,
    roll: ::core::ffi::c_float,
) -> *mut kmMat4 {
    let mut yaw_matrix: kmMat4 = kmMat4 { mat: [0.; 16] };
    kmMat4RotationY(&raw mut yaw_matrix, yaw);
    let mut pitch_matrix: kmMat4 = kmMat4 { mat: [0.; 16] };
    kmMat4RotationX(&raw mut pitch_matrix, pitch);
    let mut roll_matrix: kmMat4 = kmMat4 { mat: [0.; 16] };
    kmMat4RotationZ(&raw mut roll_matrix, roll);
    kmMat4Multiply(pOut, &raw mut pitch_matrix, &raw mut roll_matrix);
    kmMat4Multiply(pOut, &raw mut yaw_matrix, pOut);
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat4RotationQuaternion(
    mut pOut: *mut kmMat4,
    mut pQ: *const kmQuaternion,
) -> *mut kmMat4 {
    let mut xx: ::core::ffi::c_double = ((*pQ).x * (*pQ).x) as ::core::ffi::c_double;
    let mut xy: ::core::ffi::c_double = ((*pQ).x * (*pQ).y) as ::core::ffi::c_double;
    let mut xz: ::core::ffi::c_double = ((*pQ).x * (*pQ).z) as ::core::ffi::c_double;
    let mut xw: ::core::ffi::c_double = ((*pQ).x * (*pQ).w) as ::core::ffi::c_double;
    let mut yy: ::core::ffi::c_double = ((*pQ).y * (*pQ).y) as ::core::ffi::c_double;
    let mut yz: ::core::ffi::c_double = ((*pQ).y * (*pQ).z) as ::core::ffi::c_double;
    let mut yw: ::core::ffi::c_double = ((*pQ).y * (*pQ).w) as ::core::ffi::c_double;
    let mut zz: ::core::ffi::c_double = ((*pQ).z * (*pQ).z) as ::core::ffi::c_double;
    let mut zw: ::core::ffi::c_double = ((*pQ).z * (*pQ).w) as ::core::ffi::c_double;
    (*pOut).mat[0 as ::core::ffi::c_int as usize] = (1 as ::core::ffi::c_int
        as ::core::ffi::c_double
        - 2 as ::core::ffi::c_int as ::core::ffi::c_double * (yy + zz))
        as ::core::ffi::c_float;
    (*pOut).mat[1 as ::core::ffi::c_int as usize] =
        (2 as ::core::ffi::c_int as ::core::ffi::c_double * (xy + zw)) as ::core::ffi::c_float;
    (*pOut).mat[2 as ::core::ffi::c_int as usize] =
        (2 as ::core::ffi::c_int as ::core::ffi::c_double * (xz - yw)) as ::core::ffi::c_float;
    (*pOut).mat[3 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_int as ::core::ffi::c_float;
    (*pOut).mat[4 as ::core::ffi::c_int as usize] =
        (2 as ::core::ffi::c_int as ::core::ffi::c_double * (xy - zw)) as ::core::ffi::c_float;
    (*pOut).mat[5 as ::core::ffi::c_int as usize] = (1 as ::core::ffi::c_int
        as ::core::ffi::c_double
        - 2 as ::core::ffi::c_int as ::core::ffi::c_double * (xx + zz))
        as ::core::ffi::c_float;
    (*pOut).mat[6 as ::core::ffi::c_int as usize] =
        (2 as ::core::ffi::c_int as ::core::ffi::c_double * (yz + xw)) as ::core::ffi::c_float;
    (*pOut).mat[7 as ::core::ffi::c_int as usize] = 0.0f32;
    (*pOut).mat[8 as ::core::ffi::c_int as usize] =
        (2 as ::core::ffi::c_int as ::core::ffi::c_double * (xz + yw)) as ::core::ffi::c_float;
    (*pOut).mat[9 as ::core::ffi::c_int as usize] =
        (2 as ::core::ffi::c_int as ::core::ffi::c_double * (yz - xw)) as ::core::ffi::c_float;
    (*pOut).mat[10 as ::core::ffi::c_int as usize] = (1 as ::core::ffi::c_int
        as ::core::ffi::c_double
        - 2 as ::core::ffi::c_int as ::core::ffi::c_double * (xx + yy))
        as ::core::ffi::c_float;
    (*pOut).mat[11 as ::core::ffi::c_int as usize] = 0.0f32;
    (*pOut).mat[12 as ::core::ffi::c_int as usize] = 0.0f32;
    (*pOut).mat[13 as ::core::ffi::c_int as usize] = 0.0f32;
    (*pOut).mat[14 as ::core::ffi::c_int as usize] = 0.0f32;
    (*pOut).mat[15 as ::core::ffi::c_int as usize] = 1.0f32;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat4Scaling(
    mut pOut: *mut kmMat4,
    x: ::core::ffi::c_float,
    y: ::core::ffi::c_float,
    mut z: ::core::ffi::c_float,
) -> *mut kmMat4 {
    memset(
        &raw mut (*pOut).mat as *mut ::core::ffi::c_float as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        (::core::mem::size_of::<::core::ffi::c_float>() as size_t).wrapping_mul(16 as size_t),
    );
    (*pOut).mat[0 as ::core::ffi::c_int as usize] = x;
    (*pOut).mat[5 as ::core::ffi::c_int as usize] = y;
    (*pOut).mat[10 as ::core::ffi::c_int as usize] = z;
    (*pOut).mat[15 as ::core::ffi::c_int as usize] = 1.0f32;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat4Translation(
    mut pOut: *mut kmMat4,
    x: ::core::ffi::c_float,
    mut y: ::core::ffi::c_float,
    z: ::core::ffi::c_float,
) -> *mut kmMat4 {
    memset(
        &raw mut (*pOut).mat as *mut ::core::ffi::c_float as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        (::core::mem::size_of::<::core::ffi::c_float>() as size_t).wrapping_mul(16 as size_t),
    );
    (*pOut).mat[0 as ::core::ffi::c_int as usize] = 1.0f32;
    (*pOut).mat[5 as ::core::ffi::c_int as usize] = 1.0f32;
    (*pOut).mat[10 as ::core::ffi::c_int as usize] = 1.0f32;
    (*pOut).mat[12 as ::core::ffi::c_int as usize] = x;
    (*pOut).mat[13 as ::core::ffi::c_int as usize] = y;
    (*pOut).mat[14 as ::core::ffi::c_int as usize] = z;
    (*pOut).mat[15 as ::core::ffi::c_int as usize] = 1.0f32;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat4GetUpVec3(
    mut pOut: *mut kmVec3,
    mut pIn: *const kmMat4,
) -> *mut kmVec3 {
    kmVec3MultiplyMat4(pOut, &raw const KM_VEC3_POS_Y, pIn as *const kmMat4);
    kmVec3Normalize(pOut, pOut);
    return pOut as *mut kmVec3;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat4GetRightVec3(
    mut pOut: *mut kmVec3,
    mut pIn: *const kmMat4,
) -> *mut kmVec3 {
    kmVec3MultiplyMat4(pOut, &raw const KM_VEC3_POS_X, pIn as *const kmMat4);
    kmVec3Normalize(pOut, pOut);
    return pOut as *mut kmVec3;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat4GetForwardVec3RH(
    mut pOut: *mut kmVec3,
    mut pIn: *const kmMat4,
) -> *mut kmVec3 {
    kmVec3MultiplyMat4(pOut, &raw const KM_VEC3_NEG_Z, pIn as *const kmMat4);
    kmVec3Normalize(pOut, pOut);
    return pOut as *mut kmVec3;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat4GetForwardVec3LH(
    mut pOut: *mut kmVec3,
    mut pIn: *const kmMat4,
) -> *mut kmVec3 {
    kmVec3MultiplyMat4(pOut, &raw const KM_VEC3_POS_Z, pIn as *const kmMat4);
    kmVec3Normalize(pOut, pOut);
    return pOut as *mut kmVec3;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat4PerspectiveProjection(
    mut pOut: *mut kmMat4,
    mut fovY: ::core::ffi::c_float,
    mut aspect: ::core::ffi::c_float,
    mut zNear: ::core::ffi::c_float,
    mut zFar: ::core::ffi::c_float,
) -> *mut kmMat4 {
    let mut r: ::core::ffi::c_float =
        kmDegreesToRadians(fovY / 2 as ::core::ffi::c_int as ::core::ffi::c_float);
    let mut deltaZ: ::core::ffi::c_float = zFar - zNear;
    let mut s: ::core::ffi::c_float = sin(r as ::core::ffi::c_double) as ::core::ffi::c_float;
    let mut cotangent: ::core::ffi::c_float = 0 as ::core::ffi::c_int as ::core::ffi::c_float;
    if deltaZ == 0 as ::core::ffi::c_int as ::core::ffi::c_float
        || s == 0 as ::core::ffi::c_int as ::core::ffi::c_float
        || aspect == 0 as ::core::ffi::c_int as ::core::ffi::c_float
    {
        return ::core::ptr::null_mut::<kmMat4>();
    }
    cotangent =
        (cos(r as ::core::ffi::c_double) / s as ::core::ffi::c_double) as ::core::ffi::c_float;
    kmMat4Identity(pOut);
    (*pOut).mat[0 as ::core::ffi::c_int as usize] = cotangent / aspect;
    (*pOut).mat[5 as ::core::ffi::c_int as usize] = cotangent;
    (*pOut).mat[10 as ::core::ffi::c_int as usize] = -(zFar + zNear) / deltaZ;
    (*pOut).mat[11 as ::core::ffi::c_int as usize] =
        -(1 as ::core::ffi::c_int) as ::core::ffi::c_float;
    (*pOut).mat[14 as ::core::ffi::c_int as usize] =
        -(2 as ::core::ffi::c_int) as ::core::ffi::c_float * zNear * zFar / deltaZ;
    (*pOut).mat[15 as ::core::ffi::c_int as usize] =
        0 as ::core::ffi::c_int as ::core::ffi::c_float;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat4OrthographicProjection(
    mut pOut: *mut kmMat4,
    mut left: ::core::ffi::c_float,
    mut right: ::core::ffi::c_float,
    mut bottom: ::core::ffi::c_float,
    mut top: ::core::ffi::c_float,
    mut nearVal: ::core::ffi::c_float,
    mut farVal: ::core::ffi::c_float,
) -> *mut kmMat4 {
    let mut tx: ::core::ffi::c_float = -((right + left) / (right - left));
    let mut ty: ::core::ffi::c_float = -((top + bottom) / (top - bottom));
    let mut tz: ::core::ffi::c_float = -((farVal + nearVal) / (farVal - nearVal));
    kmMat4Identity(pOut);
    (*pOut).mat[0 as ::core::ffi::c_int as usize] =
        2 as ::core::ffi::c_int as ::core::ffi::c_float / (right - left);
    (*pOut).mat[5 as ::core::ffi::c_int as usize] =
        2 as ::core::ffi::c_int as ::core::ffi::c_float / (top - bottom);
    (*pOut).mat[10 as ::core::ffi::c_int as usize] =
        -(2 as ::core::ffi::c_int) as ::core::ffi::c_float / (farVal - nearVal);
    (*pOut).mat[12 as ::core::ffi::c_int as usize] = tx;
    (*pOut).mat[13 as ::core::ffi::c_int as usize] = ty;
    (*pOut).mat[14 as ::core::ffi::c_int as usize] = tz;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat4LookAt(
    mut pOut: *mut kmMat4,
    mut pEye: *const kmVec3,
    mut pCenter: *const kmVec3,
    mut pUp: *const kmVec3,
) -> *mut kmMat4 {
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
    let mut translate: kmMat4 = kmMat4 { mat: [0.; 16] };
    kmVec3Subtract(&raw mut f, pCenter, pEye);
    kmVec3Normalize(&raw mut f, &raw mut f);
    kmVec3Assign(&raw mut up, pUp);
    kmVec3Normalize(&raw mut up, &raw mut up);
    kmVec3Cross(&raw mut s, &raw mut f, &raw mut up);
    kmVec3Normalize(&raw mut s, &raw mut s);
    kmVec3Cross(&raw mut u, &raw mut s, &raw mut f);
    kmVec3Normalize(&raw mut s, &raw mut s);
    kmMat4Identity(pOut);
    (*pOut).mat[0 as ::core::ffi::c_int as usize] = s.x;
    (*pOut).mat[4 as ::core::ffi::c_int as usize] = s.y;
    (*pOut).mat[8 as ::core::ffi::c_int as usize] = s.z;
    (*pOut).mat[1 as ::core::ffi::c_int as usize] = u.x;
    (*pOut).mat[5 as ::core::ffi::c_int as usize] = u.y;
    (*pOut).mat[9 as ::core::ffi::c_int as usize] = u.z;
    (*pOut).mat[2 as ::core::ffi::c_int as usize] = -f.x;
    (*pOut).mat[6 as ::core::ffi::c_int as usize] = -f.y;
    (*pOut).mat[10 as ::core::ffi::c_int as usize] = -f.z;
    kmMat4Translation(&raw mut translate, -(*pEye).x, -(*pEye).y, -(*pEye).z);
    kmMat4Multiply(pOut, pOut, &raw mut translate);
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat4ExtractRotation(
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
    return pOut as *mut kmMat3;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat4RotationToAxisAngle(
    mut pAxis: *mut kmVec3,
    mut radians: *mut ::core::ffi::c_float,
    mut pIn: *const kmMat4,
) -> *mut kmVec3 {
    let mut temp: kmQuaternion = kmQuaternion {
        x: 0.,
        y: 0.,
        z: 0.,
        w: 0.,
    };
    let mut rotation: kmMat3 = kmMat3 { mat: [0.; 9] };
    kmMat4ExtractRotation(&raw mut rotation, pIn);
    kmQuaternionRotationMatrix(&raw mut temp, &raw mut rotation);
    kmQuaternionToAxisAngle(&raw mut temp, pAxis as *mut kmVec3, radians);
    return pAxis as *mut kmVec3;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat4RotationTranslation(
    mut pOut: *mut kmMat4,
    mut rotation: *const kmMat3,
    mut translation: *const kmVec3,
) -> *mut kmMat4 {
    (*pOut).mat[0 as ::core::ffi::c_int as usize] =
        (*rotation).mat[0 as ::core::ffi::c_int as usize];
    (*pOut).mat[1 as ::core::ffi::c_int as usize] =
        (*rotation).mat[1 as ::core::ffi::c_int as usize];
    (*pOut).mat[2 as ::core::ffi::c_int as usize] =
        (*rotation).mat[2 as ::core::ffi::c_int as usize];
    (*pOut).mat[3 as ::core::ffi::c_int as usize] = 0.0f32;
    (*pOut).mat[4 as ::core::ffi::c_int as usize] =
        (*rotation).mat[3 as ::core::ffi::c_int as usize];
    (*pOut).mat[5 as ::core::ffi::c_int as usize] =
        (*rotation).mat[4 as ::core::ffi::c_int as usize];
    (*pOut).mat[6 as ::core::ffi::c_int as usize] =
        (*rotation).mat[5 as ::core::ffi::c_int as usize];
    (*pOut).mat[7 as ::core::ffi::c_int as usize] = 0.0f32;
    (*pOut).mat[8 as ::core::ffi::c_int as usize] =
        (*rotation).mat[6 as ::core::ffi::c_int as usize];
    (*pOut).mat[9 as ::core::ffi::c_int as usize] =
        (*rotation).mat[7 as ::core::ffi::c_int as usize];
    (*pOut).mat[10 as ::core::ffi::c_int as usize] =
        (*rotation).mat[8 as ::core::ffi::c_int as usize];
    (*pOut).mat[11 as ::core::ffi::c_int as usize] = 0.0f32;
    (*pOut).mat[12 as ::core::ffi::c_int as usize] = (*translation).x;
    (*pOut).mat[13 as ::core::ffi::c_int as usize] = (*translation).y;
    (*pOut).mat[14 as ::core::ffi::c_int as usize] = (*translation).z;
    (*pOut).mat[15 as ::core::ffi::c_int as usize] = 1.0f32;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat4ExtractPlane(
    mut pOut: *mut kmPlane,
    mut pIn: *const kmMat4,
    plane: ::core::ffi::c_uint,
) -> *mut kmPlane {
    let mut t: ::core::ffi::c_float = 1.0f32;
    match plane {
        1 => {
            (*pOut).a = (*pIn).mat[3 as ::core::ffi::c_int as usize]
                - (*pIn).mat[0 as ::core::ffi::c_int as usize];
            (*pOut).b = (*pIn).mat[7 as ::core::ffi::c_int as usize]
                - (*pIn).mat[4 as ::core::ffi::c_int as usize];
            (*pOut).c = (*pIn).mat[11 as ::core::ffi::c_int as usize]
                - (*pIn).mat[8 as ::core::ffi::c_int as usize];
            (*pOut).d = (*pIn).mat[15 as ::core::ffi::c_int as usize]
                - (*pIn).mat[12 as ::core::ffi::c_int as usize];
        }
        0 => {
            (*pOut).a = (*pIn).mat[3 as ::core::ffi::c_int as usize]
                + (*pIn).mat[0 as ::core::ffi::c_int as usize];
            (*pOut).b = (*pIn).mat[7 as ::core::ffi::c_int as usize]
                + (*pIn).mat[4 as ::core::ffi::c_int as usize];
            (*pOut).c = (*pIn).mat[11 as ::core::ffi::c_int as usize]
                + (*pIn).mat[8 as ::core::ffi::c_int as usize];
            (*pOut).d = (*pIn).mat[15 as ::core::ffi::c_int as usize]
                + (*pIn).mat[12 as ::core::ffi::c_int as usize];
        }
        2 => {
            (*pOut).a = (*pIn).mat[3 as ::core::ffi::c_int as usize]
                + (*pIn).mat[1 as ::core::ffi::c_int as usize];
            (*pOut).b = (*pIn).mat[7 as ::core::ffi::c_int as usize]
                + (*pIn).mat[5 as ::core::ffi::c_int as usize];
            (*pOut).c = (*pIn).mat[11 as ::core::ffi::c_int as usize]
                + (*pIn).mat[9 as ::core::ffi::c_int as usize];
            (*pOut).d = (*pIn).mat[15 as ::core::ffi::c_int as usize]
                + (*pIn).mat[13 as ::core::ffi::c_int as usize];
        }
        3 => {
            (*pOut).a = (*pIn).mat[3 as ::core::ffi::c_int as usize]
                - (*pIn).mat[1 as ::core::ffi::c_int as usize];
            (*pOut).b = (*pIn).mat[7 as ::core::ffi::c_int as usize]
                - (*pIn).mat[5 as ::core::ffi::c_int as usize];
            (*pOut).c = (*pIn).mat[11 as ::core::ffi::c_int as usize]
                - (*pIn).mat[9 as ::core::ffi::c_int as usize];
            (*pOut).d = (*pIn).mat[15 as ::core::ffi::c_int as usize]
                - (*pIn).mat[13 as ::core::ffi::c_int as usize];
        }
        5 => {
            (*pOut).a = (*pIn).mat[3 as ::core::ffi::c_int as usize]
                - (*pIn).mat[2 as ::core::ffi::c_int as usize];
            (*pOut).b = (*pIn).mat[7 as ::core::ffi::c_int as usize]
                - (*pIn).mat[6 as ::core::ffi::c_int as usize];
            (*pOut).c = (*pIn).mat[11 as ::core::ffi::c_int as usize]
                - (*pIn).mat[10 as ::core::ffi::c_int as usize];
            (*pOut).d = (*pIn).mat[15 as ::core::ffi::c_int as usize]
                - (*pIn).mat[14 as ::core::ffi::c_int as usize];
        }
        4 => {
            (*pOut).a = (*pIn).mat[3 as ::core::ffi::c_int as usize]
                + (*pIn).mat[2 as ::core::ffi::c_int as usize];
            (*pOut).b = (*pIn).mat[7 as ::core::ffi::c_int as usize]
                + (*pIn).mat[6 as ::core::ffi::c_int as usize];
            (*pOut).c = (*pIn).mat[11 as ::core::ffi::c_int as usize]
                + (*pIn).mat[10 as ::core::ffi::c_int as usize];
            (*pOut).d = (*pIn).mat[15 as ::core::ffi::c_int as usize]
                + (*pIn).mat[14 as ::core::ffi::c_int as usize];
        }
        _ => {}
    }
    t = sqrtf((*pOut).a * (*pOut).a + (*pOut).b * (*pOut).b + (*pOut).c * (*pOut).c);
    (*pOut).a /= t;
    (*pOut).b /= t;
    (*pOut).c /= t;
    (*pOut).d /= t;
    return pOut as *mut kmPlane;
}
