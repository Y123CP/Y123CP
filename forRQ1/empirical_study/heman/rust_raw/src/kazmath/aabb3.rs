extern "C" {
    fn fabs(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn kmVec3Fill(
        pOut: *mut kmVec3,
        x: ::core::ffi::c_float,
        y: ::core::ffi::c_float,
        z: ::core::ffi::c_float,
    ) -> *mut kmVec3;
    fn kmVec3Add(pOut: *mut kmVec3, pV1: *const kmVec3, pV2: *const kmVec3) -> *mut kmVec3;
    fn kmVec3Scale(pOut: *mut kmVec3, pIn: *const kmVec3, s: ::core::ffi::c_float) -> *mut kmVec3;
    fn kmVec3Assign(pOut: *mut kmVec3, pIn: *const kmVec3) -> *mut kmVec3;
    fn kmVec3Zero(pOut: *mut kmVec3) -> *mut kmVec3;
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
pub struct kmAABB3 {
    pub min: kmVec3,
    pub max: kmVec3,
}
pub const KM_FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const KM_TRUE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const KM_CONTAINS_NONE: ::core::ffi::c_uint = 0 as ::core::ffi::c_int as ::core::ffi::c_uint;
pub const KM_CONTAINS_PARTIAL: ::core::ffi::c_uint = 1 as ::core::ffi::c_int as ::core::ffi::c_uint;
pub const KM_CONTAINS_ALL: ::core::ffi::c_uint = 2 as ::core::ffi::c_int as ::core::ffi::c_uint;
#[no_mangle]
pub unsafe extern "C" fn kmAABB3Initialize(
    mut pBox: *mut kmAABB3,
    mut centre: *const kmVec3,
    width: ::core::ffi::c_float,
    height: ::core::ffi::c_float,
    depth: ::core::ffi::c_float,
) -> *mut kmAABB3 {
    if pBox.is_null() {
        return ::core::ptr::null_mut::<kmAABB3>();
    }
    let mut origin: kmVec3 = kmVec3 {
        x: 0.,
        y: 0.,
        z: 0.,
    };
    let mut point: *mut kmVec3 = if !centre.is_null() {
        centre as *mut kmVec3
    } else {
        &raw mut origin
    };
    kmVec3Zero(&raw mut origin);
    (*pBox).min.x = (*point).x - width / 2 as ::core::ffi::c_int as ::core::ffi::c_float;
    (*pBox).min.y = (*point).y - height / 2 as ::core::ffi::c_int as ::core::ffi::c_float;
    (*pBox).min.z = (*point).z - depth / 2 as ::core::ffi::c_int as ::core::ffi::c_float;
    (*pBox).max.x = (*point).x + width / 2 as ::core::ffi::c_int as ::core::ffi::c_float;
    (*pBox).max.y = (*point).y + height / 2 as ::core::ffi::c_int as ::core::ffi::c_float;
    (*pBox).max.z = (*point).z + depth / 2 as ::core::ffi::c_int as ::core::ffi::c_float;
    return pBox;
}
#[no_mangle]
pub unsafe extern "C" fn kmAABB3ContainsPoint(
    mut pBox: *const kmAABB3,
    mut pPoint: *const kmVec3,
) -> ::core::ffi::c_int {
    if (*pPoint).x >= (*pBox).min.x
        && (*pPoint).x <= (*pBox).max.x
        && (*pPoint).y >= (*pBox).min.y
        && (*pPoint).y <= (*pBox).max.y
        && (*pPoint).z >= (*pBox).min.z
        && (*pPoint).z <= (*pBox).max.z
    {
        return KM_TRUE;
    }
    return KM_FALSE;
}
#[no_mangle]
pub unsafe extern "C" fn kmAABB3Assign(
    mut pOut: *mut kmAABB3,
    mut pIn: *const kmAABB3,
) -> *mut kmAABB3 {
    kmVec3Assign(&raw mut (*pOut).min, &raw const (*pIn).min);
    kmVec3Assign(&raw mut (*pOut).max, &raw const (*pIn).max);
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmAABB3Scale(
    mut pOut: *mut kmAABB3,
    mut pIn: *const kmAABB3,
    mut s: ::core::ffi::c_float,
) -> *mut kmAABB3 {
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmAABB3IntersectsTriangle(
    mut box_0: *mut kmAABB3,
    mut p1: *const kmVec3,
    mut p2: *const kmVec3,
    mut p3: *const kmVec3,
) -> ::core::ffi::c_uchar {
    return KM_TRUE as ::core::ffi::c_uchar;
}
#[no_mangle]
pub unsafe extern "C" fn kmAABB3IntersectsAABB(
    mut box_0: *const kmAABB3,
    mut other: *const kmAABB3,
) -> ::core::ffi::c_uchar {
    return (kmAABB3ContainsAABB(box_0, other) != KM_CONTAINS_NONE) as ::core::ffi::c_int
        as ::core::ffi::c_uchar;
}
#[no_mangle]
pub unsafe extern "C" fn kmAABB3ContainsAABB(
    mut container: *const kmAABB3,
    mut to_check: *const kmAABB3,
) -> ::core::ffi::c_uint {
    let mut corners: [kmVec3; 8] = [kmVec3 {
        x: 0.,
        y: 0.,
        z: 0.,
    }; 8];
    let mut result: ::core::ffi::c_uint = KM_CONTAINS_ALL;
    let mut found: ::core::ffi::c_uchar = KM_FALSE as ::core::ffi::c_uchar;
    kmVec3Fill(
        (&raw mut corners as *mut kmVec3).offset(0 as ::core::ffi::c_int as isize) as *mut kmVec3,
        (*to_check).min.x,
        (*to_check).min.y,
        (*to_check).min.z,
    );
    kmVec3Fill(
        (&raw mut corners as *mut kmVec3).offset(1 as ::core::ffi::c_int as isize) as *mut kmVec3,
        (*to_check).max.x,
        (*to_check).min.y,
        (*to_check).min.z,
    );
    kmVec3Fill(
        (&raw mut corners as *mut kmVec3).offset(2 as ::core::ffi::c_int as isize) as *mut kmVec3,
        (*to_check).max.x,
        (*to_check).max.y,
        (*to_check).min.z,
    );
    kmVec3Fill(
        (&raw mut corners as *mut kmVec3).offset(3 as ::core::ffi::c_int as isize) as *mut kmVec3,
        (*to_check).min.x,
        (*to_check).max.y,
        (*to_check).min.z,
    );
    kmVec3Fill(
        (&raw mut corners as *mut kmVec3).offset(4 as ::core::ffi::c_int as isize) as *mut kmVec3,
        (*to_check).min.x,
        (*to_check).min.y,
        (*to_check).max.z,
    );
    kmVec3Fill(
        (&raw mut corners as *mut kmVec3).offset(5 as ::core::ffi::c_int as isize) as *mut kmVec3,
        (*to_check).max.x,
        (*to_check).min.y,
        (*to_check).max.z,
    );
    kmVec3Fill(
        (&raw mut corners as *mut kmVec3).offset(6 as ::core::ffi::c_int as isize) as *mut kmVec3,
        (*to_check).max.x,
        (*to_check).max.y,
        (*to_check).max.z,
    );
    kmVec3Fill(
        (&raw mut corners as *mut kmVec3).offset(7 as ::core::ffi::c_int as isize) as *mut kmVec3,
        (*to_check).min.x,
        (*to_check).max.y,
        (*to_check).max.z,
    );
    let mut i: ::core::ffi::c_uchar = 0 as ::core::ffi::c_uchar;
    while (i as ::core::ffi::c_int) < 8 as ::core::ffi::c_int {
        if kmAABB3ContainsPoint(
            container,
            (&raw mut corners as *mut kmVec3).offset(i as isize) as *mut kmVec3,
        ) == 0
        {
            result = KM_CONTAINS_PARTIAL;
            if found != 0 {
                return result;
            }
        } else {
            found = KM_TRUE as ::core::ffi::c_uchar;
        }
        i = i.wrapping_add(1);
    }
    if found == 0 {
        result = KM_CONTAINS_NONE;
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn kmAABB3DiameterX(mut aabb: *const kmAABB3) -> ::core::ffi::c_float {
    return fabs(((*aabb).max.x - (*aabb).min.x) as ::core::ffi::c_double) as ::core::ffi::c_float;
}
#[no_mangle]
pub unsafe extern "C" fn kmAABB3DiameterY(mut aabb: *const kmAABB3) -> ::core::ffi::c_float {
    return fabs(((*aabb).max.y - (*aabb).min.y) as ::core::ffi::c_double) as ::core::ffi::c_float;
}
#[no_mangle]
pub unsafe extern "C" fn kmAABB3DiameterZ(mut aabb: *const kmAABB3) -> ::core::ffi::c_float {
    return fabs(((*aabb).max.z - (*aabb).min.z) as ::core::ffi::c_double) as ::core::ffi::c_float;
}
#[no_mangle]
pub unsafe extern "C" fn kmAABB3Centre(
    mut aabb: *const kmAABB3,
    mut pOut: *mut kmVec3,
) -> *mut kmVec3 {
    kmVec3Add(pOut, &raw const (*aabb).min, &raw const (*aabb).max);
    kmVec3Scale(pOut, pOut, 0.5f32);
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmAABB3ExpandToContain(
    mut pOut: *mut kmAABB3,
    mut pIn: *const kmAABB3,
    mut other: *const kmAABB3,
) -> *mut kmAABB3 {
    let mut result: kmAABB3 = kmAABB3 {
        min: kmVec3 {
            x: 0.,
            y: 0.,
            z: 0.,
        },
        max: kmVec3 {
            x: 0.,
            y: 0.,
            z: 0.,
        },
    };
    result.min.x = if (*pIn).min.x < (*other).min.x {
        (*pIn).min.x
    } else {
        (*other).min.x
    };
    result.max.x = if (*pIn).max.x > (*other).max.x {
        (*pIn).max.x
    } else {
        (*other).max.x
    };
    result.min.y = if (*pIn).min.y < (*other).min.y {
        (*pIn).min.y
    } else {
        (*other).min.y
    };
    result.max.y = if (*pIn).max.y > (*other).max.y {
        (*pIn).max.y
    } else {
        (*other).max.y
    };
    result.min.z = if (*pIn).min.z < (*other).min.z {
        (*pIn).min.z
    } else {
        (*other).min.z
    };
    result.max.z = if (*pIn).max.z > (*other).max.z {
        (*pIn).max.z
    } else {
        (*other).max.z
    };
    kmAABB3Assign(pOut, &raw mut result);
    return pOut;
}
