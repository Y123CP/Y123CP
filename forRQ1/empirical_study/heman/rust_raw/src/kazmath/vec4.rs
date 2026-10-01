extern "C" {
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn sqrtf(__x: ::core::ffi::c_float) -> ::core::ffi::c_float;
    fn kmSQR(s: ::core::ffi::c_float) -> ::core::ffi::c_float;
}
pub type size_t = usize;
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
pub const kmEpsilon: ::core::ffi::c_double = 0.0001f64;
#[no_mangle]
pub unsafe extern "C" fn kmVec4Fill(
    mut pOut: *mut kmVec4,
    mut x: ::core::ffi::c_float,
    mut y: ::core::ffi::c_float,
    mut z: ::core::ffi::c_float,
    mut w: ::core::ffi::c_float,
) -> *mut kmVec4 {
    (*pOut).x = x;
    (*pOut).y = y;
    (*pOut).z = z;
    (*pOut).w = w;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmVec4Add(
    mut pOut: *mut kmVec4,
    mut pV1: *const kmVec4,
    mut pV2: *const kmVec4,
) -> *mut kmVec4 {
    (*pOut).x = (*pV1).x + (*pV2).x;
    (*pOut).y = (*pV1).y + (*pV2).y;
    (*pOut).z = (*pV1).z + (*pV2).z;
    (*pOut).w = (*pV1).w + (*pV2).w;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmVec4Dot(
    mut pV1: *const kmVec4,
    mut pV2: *const kmVec4,
) -> ::core::ffi::c_float {
    return (*pV1).x * (*pV2).x + (*pV1).y * (*pV2).y + (*pV1).z * (*pV2).z + (*pV1).w * (*pV2).w;
}
#[no_mangle]
pub unsafe extern "C" fn kmVec4Length(mut pIn: *const kmVec4) -> ::core::ffi::c_float {
    return sqrtf(kmSQR((*pIn).x) + kmSQR((*pIn).y) + kmSQR((*pIn).z) + kmSQR((*pIn).w));
}
#[no_mangle]
pub unsafe extern "C" fn kmVec4LengthSq(mut pIn: *const kmVec4) -> ::core::ffi::c_float {
    return kmSQR((*pIn).x) + kmSQR((*pIn).y) + kmSQR((*pIn).z) + kmSQR((*pIn).w);
}
#[no_mangle]
pub unsafe extern "C" fn kmVec4Lerp(
    mut pOut: *mut kmVec4,
    mut pV1: *const kmVec4,
    mut pV2: *const kmVec4,
    mut t: ::core::ffi::c_float,
) -> *mut kmVec4 {
    (*pOut).x = (*pV1).x + t * ((*pV2).x - (*pV1).x);
    (*pOut).y = (*pV1).y + t * ((*pV2).y - (*pV1).y);
    (*pOut).z = (*pV1).z + t * ((*pV2).z - (*pV1).z);
    (*pOut).w = (*pV1).w + t * ((*pV2).w - (*pV1).w);
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmVec4Normalize(
    mut pOut: *mut kmVec4,
    mut pIn: *const kmVec4,
) -> *mut kmVec4 {
    if (*pIn).x == 0. && (*pIn).y == 0. && (*pIn).z == 0. && (*pIn).w == 0. {
        return kmVec4Assign(pOut, pIn);
    }
    let mut l: ::core::ffi::c_float = 1.0f32 / kmVec4Length(pIn);
    (*pOut).x = (*pIn).x * l;
    (*pOut).y = (*pIn).y * l;
    (*pOut).z = (*pIn).z * l;
    (*pOut).w = (*pIn).w * l;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmVec4Scale(
    mut pOut: *mut kmVec4,
    mut pIn: *const kmVec4,
    s: ::core::ffi::c_float,
) -> *mut kmVec4 {
    kmVec4Normalize(pOut, pIn);
    (*pOut).x *= s;
    (*pOut).y *= s;
    (*pOut).z *= s;
    (*pOut).w *= s;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmVec4Subtract(
    mut pOut: *mut kmVec4,
    mut pV1: *const kmVec4,
    mut pV2: *const kmVec4,
) -> *mut kmVec4 {
    (*pOut).x = (*pV1).x - (*pV2).x;
    (*pOut).y = (*pV1).y - (*pV2).y;
    (*pOut).z = (*pV1).z - (*pV2).z;
    (*pOut).w = (*pV1).w - (*pV2).w;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmVec4Mul(
    mut pOut: *mut kmVec4,
    mut pV1: *const kmVec4,
    mut pV2: *const kmVec4,
) -> *mut kmVec4 {
    (*pOut).x = (*pV1).x * (*pV2).x;
    (*pOut).y = (*pV1).y * (*pV2).y;
    (*pOut).z = (*pV1).z * (*pV2).z;
    (*pOut).w = (*pV1).w * (*pV2).w;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmVec4Div(
    mut pOut: *mut kmVec4,
    mut pV1: *const kmVec4,
    mut pV2: *const kmVec4,
) -> *mut kmVec4 {
    if (*pV2).x != 0. && (*pV2).y != 0. && (*pV2).z != 0. && (*pV2).w != 0. {
        (*pOut).x = (*pV1).x / (*pV2).x;
        (*pOut).y = (*pV1).y / (*pV2).y;
        (*pOut).z = (*pV1).z / (*pV2).z;
        (*pOut).w = (*pV1).w / (*pV2).w;
    }
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmVec4MultiplyMat4(
    mut pOut: *mut kmVec4,
    mut pV: *const kmVec4,
    mut pM: *const kmMat4,
) -> *mut kmVec4 {
    (*pOut).x = (*pV).x * (*pM).mat[0 as ::core::ffi::c_int as usize]
        + (*pV).y * (*pM).mat[4 as ::core::ffi::c_int as usize]
        + (*pV).z * (*pM).mat[8 as ::core::ffi::c_int as usize]
        + (*pV).w * (*pM).mat[12 as ::core::ffi::c_int as usize];
    (*pOut).y = (*pV).x * (*pM).mat[1 as ::core::ffi::c_int as usize]
        + (*pV).y * (*pM).mat[5 as ::core::ffi::c_int as usize]
        + (*pV).z * (*pM).mat[9 as ::core::ffi::c_int as usize]
        + (*pV).w * (*pM).mat[13 as ::core::ffi::c_int as usize];
    (*pOut).z = (*pV).x * (*pM).mat[2 as ::core::ffi::c_int as usize]
        + (*pV).y * (*pM).mat[6 as ::core::ffi::c_int as usize]
        + (*pV).z * (*pM).mat[10 as ::core::ffi::c_int as usize]
        + (*pV).w * (*pM).mat[14 as ::core::ffi::c_int as usize];
    (*pOut).w = (*pV).x * (*pM).mat[3 as ::core::ffi::c_int as usize]
        + (*pV).y * (*pM).mat[7 as ::core::ffi::c_int as usize]
        + (*pV).z * (*pM).mat[11 as ::core::ffi::c_int as usize]
        + (*pV).w * (*pM).mat[15 as ::core::ffi::c_int as usize];
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmVec4Transform(
    mut pOut: *mut kmVec4,
    mut pV: *const kmVec4,
    mut pM: *const kmMat4,
) -> *mut kmVec4 {
    return kmVec4MultiplyMat4(pOut, pV, pM as *const kmMat4);
}
#[no_mangle]
pub unsafe extern "C" fn kmVec4TransformArray(
    mut pOut: *mut kmVec4,
    mut outStride: ::core::ffi::c_uint,
    mut pV: *const kmVec4,
    mut vStride: ::core::ffi::c_uint,
    mut pM: *const kmMat4,
    mut count: ::core::ffi::c_uint,
) -> *mut kmVec4 {
    let mut i: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    while i < count {
        let mut in_0: *const kmVec4 = pV.offset(i.wrapping_mul(vStride) as isize);
        let mut out: *mut kmVec4 = pOut.offset(i.wrapping_mul(outStride) as isize);
        kmVec4Transform(out, in_0, pM);
        i = i.wrapping_add(1);
    }
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmVec4AreEqual(
    mut p1: *const kmVec4,
    mut p2: *const kmVec4,
) -> ::core::ffi::c_int {
    return (((*p1).x as ::core::ffi::c_double) < (*p2).x as ::core::ffi::c_double + kmEpsilon
        && (*p1).x as ::core::ffi::c_double > (*p2).x as ::core::ffi::c_double - kmEpsilon
        && (((*p1).y as ::core::ffi::c_double) < (*p2).y as ::core::ffi::c_double + kmEpsilon
            && (*p1).y as ::core::ffi::c_double > (*p2).y as ::core::ffi::c_double - kmEpsilon)
        && (((*p1).z as ::core::ffi::c_double) < (*p2).z as ::core::ffi::c_double + kmEpsilon
            && (*p1).z as ::core::ffi::c_double > (*p2).z as ::core::ffi::c_double - kmEpsilon)
        && (((*p1).w as ::core::ffi::c_double) < (*p2).w as ::core::ffi::c_double + kmEpsilon
            && (*p1).w as ::core::ffi::c_double > (*p2).w as ::core::ffi::c_double - kmEpsilon))
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn kmVec4Assign(
    mut pOut: *mut kmVec4,
    mut pIn: *const kmVec4,
) -> *mut kmVec4 {
    memcpy(
        pOut as *mut ::core::ffi::c_void,
        pIn as *const ::core::ffi::c_void,
        (::core::mem::size_of::<::core::ffi::c_float>() as size_t).wrapping_mul(4 as size_t),
    );
    return pOut;
}
