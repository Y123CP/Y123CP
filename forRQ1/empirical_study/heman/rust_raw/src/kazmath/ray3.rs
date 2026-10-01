extern "C" {
    fn kmVec3Add(pOut: *mut kmVec3, pV1: *const kmVec3, pV2: *const kmVec3) -> *mut kmVec3;
    fn kmVec3Scale(pOut: *mut kmVec3, pIn: *const kmVec3, s: ::core::ffi::c_float) -> *mut kmVec3;
    fn kmVec3Assign(pOut: *mut kmVec3, pIn: *const kmVec3) -> *mut kmVec3;
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
pub const KM_FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const KM_TRUE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn kmRay3Fill(
    mut ray: *mut kmRay3,
    mut px: ::core::ffi::c_float,
    mut py: ::core::ffi::c_float,
    mut pz: ::core::ffi::c_float,
    mut vx: ::core::ffi::c_float,
    mut vy: ::core::ffi::c_float,
    mut vz: ::core::ffi::c_float,
) -> *mut kmRay3 {
    (*ray).start.x = px;
    (*ray).start.y = py;
    (*ray).start.z = pz;
    (*ray).dir.x = vx;
    (*ray).dir.y = vy;
    (*ray).dir.z = vz;
    return ray;
}
#[no_mangle]
pub unsafe extern "C" fn kmRay3FromPointAndDirection(
    mut ray: *mut kmRay3,
    mut point: *const kmVec3,
    mut direction: *const kmVec3,
) -> *mut kmRay3 {
    kmVec3Assign(&raw mut (*ray).start, point);
    kmVec3Assign(&raw mut (*ray).dir, direction);
    return ray;
}
#[no_mangle]
pub unsafe extern "C" fn kmRay3IntersectPlane(
    mut pOut: *mut kmVec3,
    mut ray: *const kmRay3,
    mut plane: *const kmPlane,
) -> ::core::ffi::c_uchar {
    let mut d: ::core::ffi::c_float =
        (*plane).a * (*ray).dir.x + (*plane).b * (*ray).dir.y + (*plane).c * (*ray).dir.z;
    if d == 0 as ::core::ffi::c_int as ::core::ffi::c_float {
        return KM_FALSE as ::core::ffi::c_uchar;
    }
    let mut t: ::core::ffi::c_float = -((*plane).a * (*ray).start.x
        + (*plane).b * (*ray).start.y
        + (*plane).c * (*ray).start.z
        + (*plane).d)
        / d;
    if t < 0 as ::core::ffi::c_int as ::core::ffi::c_float {
        return KM_FALSE as ::core::ffi::c_uchar;
    }
    let mut scaled_dir: kmVec3 = kmVec3 {
        x: 0.,
        y: 0.,
        z: 0.,
    };
    kmVec3Scale(&raw mut scaled_dir, &raw const (*ray).dir, t);
    kmVec3Add(pOut, &raw const (*ray).start, &raw mut scaled_dir);
    return KM_TRUE as ::core::ffi::c_uchar;
}
