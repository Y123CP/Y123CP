extern "C" {
    fn kmVec2Length(pIn: *const kmVec2) -> ::core::ffi::c_float;
    fn kmVec2Normalize(pOut: *mut kmVec2, pIn: *const kmVec2) -> *mut kmVec2;
    fn kmVec2Dot(pV1: *const kmVec2, pV2: *const kmVec2) -> ::core::ffi::c_float;
    fn kmVec2Subtract(pOut: *mut kmVec2, pV1: *const kmVec2, pV2: *const kmVec2) -> *mut kmVec2;
    fn kmVec2Assign(pOut: *mut kmVec2, pIn: *const kmVec2) -> *mut kmVec2;
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct kmVec2 {
    pub x: ::core::ffi::c_float,
    pub y: ::core::ffi::c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kmRay2 {
    pub start: kmVec2,
    pub dir: kmVec2,
}
pub const KM_FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const KM_TRUE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const kmEpsilon: ::core::ffi::c_double = 0.0001f64;
#[no_mangle]
pub unsafe extern "C" fn kmRay2Fill(
    mut ray: *mut kmRay2,
    mut px: ::core::ffi::c_float,
    mut py: ::core::ffi::c_float,
    mut vx: ::core::ffi::c_float,
    mut vy: ::core::ffi::c_float,
) {
    (*ray).start.x = px;
    (*ray).start.y = py;
    (*ray).dir.x = vx;
    (*ray).dir.y = vy;
}
#[no_mangle]
pub unsafe extern "C" fn kmRay2FillWithEndpoints(
    mut ray: *mut kmRay2,
    mut start: *const kmVec2,
    mut end: *const kmVec2,
) {
    (*ray).start.x = (*start).x;
    (*ray).start.y = (*start).y;
    (*ray).dir.x = (*end).x - (*start).x;
    (*ray).dir.y = (*end).y - (*start).y;
}
#[no_mangle]
pub unsafe extern "C" fn kmLine2WithLineIntersection(
    mut ptA: *const kmVec2,
    mut vecA: *const kmVec2,
    mut ptB: *const kmVec2,
    mut vecB: *const kmVec2,
    mut outTA: *mut ::core::ffi::c_float,
    mut outTB: *mut ::core::ffi::c_float,
    mut outIntersection: *mut kmVec2,
) -> ::core::ffi::c_uchar {
    let mut x1: ::core::ffi::c_float = (*ptA).x;
    let mut y1: ::core::ffi::c_float = (*ptA).y;
    let mut x2: ::core::ffi::c_float = x1 + (*vecA).x;
    let mut y2: ::core::ffi::c_float = y1 + (*vecA).y;
    let mut x3: ::core::ffi::c_float = (*ptB).x;
    let mut y3: ::core::ffi::c_float = (*ptB).y;
    let mut x4: ::core::ffi::c_float = x3 + (*vecB).x;
    let mut y4: ::core::ffi::c_float = y3 + (*vecB).y;
    let mut denom: ::core::ffi::c_float = (y4 - y3) * (x2 - x1) - (x4 - x3) * (y2 - y1);
    if denom as ::core::ffi::c_double > -kmEpsilon && (denom as ::core::ffi::c_double) < kmEpsilon {
        return KM_FALSE as ::core::ffi::c_uchar;
    }
    let mut ua: ::core::ffi::c_float = ((x4 - x3) * (y1 - y3) - (y4 - y3) * (x1 - x3)) / denom;
    let mut ub: ::core::ffi::c_float = ((x2 - x1) * (y1 - y3) - (y2 - y1) * (x1 - x3)) / denom;
    let mut x: ::core::ffi::c_float = x1 + ua * (x2 - x1);
    let mut y: ::core::ffi::c_float = y1 + ua * (y2 - y1);
    if !outTA.is_null() {
        *outTA = ua;
    }
    if !outTB.is_null() {
        *outTB = ub;
    }
    if !outIntersection.is_null() {
        (*outIntersection).x = x;
        (*outIntersection).y = y;
    }
    return KM_TRUE as ::core::ffi::c_uchar;
}
#[no_mangle]
pub unsafe extern "C" fn kmSegment2WithSegmentIntersection(
    mut segmentA: *const kmRay2,
    mut segmentB: *const kmRay2,
    mut intersection: *mut kmVec2,
) -> ::core::ffi::c_uchar {
    let mut ua: ::core::ffi::c_float = 0.;
    let mut ub: ::core::ffi::c_float = 0.;
    let mut pt: kmVec2 = kmVec2 { x: 0., y: 0. };
    if kmLine2WithLineIntersection(
        &raw const (*segmentA).start,
        &raw const (*segmentA).dir,
        &raw const (*segmentB).start,
        &raw const (*segmentB).start,
        &raw mut ua,
        &raw mut ub,
        &raw mut pt,
    ) as ::core::ffi::c_int
        != 0
        && 0.0f64 <= ua as ::core::ffi::c_double
        && ua as ::core::ffi::c_double <= 1.0f64
        && 0.0f64 <= ub as ::core::ffi::c_double
        && ub as ::core::ffi::c_double <= 1.0f64
    {
        (*intersection).x = pt.x;
        (*intersection).y = pt.y;
        return KM_TRUE as ::core::ffi::c_uchar;
    }
    return KM_FALSE as ::core::ffi::c_uchar;
}
#[no_mangle]
pub unsafe extern "C" fn kmRay2IntersectLineSegment(
    mut ray: *const kmRay2,
    mut p1: *const kmVec2,
    mut p2: *const kmVec2,
    mut intersection: *mut kmVec2,
) -> ::core::ffi::c_uchar {
    let mut ua: ::core::ffi::c_float = 0.;
    let mut ub: ::core::ffi::c_float = 0.;
    let mut pt: kmVec2 = kmVec2 { x: 0., y: 0. };
    let mut otherSegment: kmRay2 = kmRay2 {
        start: kmVec2 { x: 0., y: 0. },
        dir: kmVec2 { x: 0., y: 0. },
    };
    kmRay2FillWithEndpoints(&raw mut otherSegment, p1, p2);
    if kmLine2WithLineIntersection(
        &raw const (*ray).start,
        &raw const (*ray).dir,
        &raw mut otherSegment.start,
        &raw mut otherSegment.dir,
        &raw mut ua,
        &raw mut ub,
        &raw mut pt,
    ) as ::core::ffi::c_int
        != 0
        && 0.0f64 <= ua as ::core::ffi::c_double
        && 0.0f64 <= ub as ::core::ffi::c_double
        && ub as ::core::ffi::c_double <= 1.0f64
    {
        (*intersection).x = pt.x;
        (*intersection).y = pt.y;
        return KM_TRUE as ::core::ffi::c_uchar;
    }
    return KM_FALSE as ::core::ffi::c_uchar;
}
#[no_mangle]
pub unsafe extern "C" fn calculate_line_normal(
    mut p1: kmVec2,
    mut p2: kmVec2,
    mut other_point: kmVec2,
    mut normal_out: *mut kmVec2,
) {
    let mut edge: kmVec2 = kmVec2 { x: 0., y: 0. };
    let mut other_edge: kmVec2 = kmVec2 { x: 0., y: 0. };
    kmVec2Subtract(&raw mut edge, &raw mut p2, &raw mut p1);
    kmVec2Subtract(&raw mut other_edge, &raw mut other_point, &raw mut p1);
    kmVec2Normalize(&raw mut edge, &raw mut edge);
    kmVec2Normalize(&raw mut other_edge, &raw mut other_edge);
    let mut n: kmVec2 = kmVec2 { x: 0., y: 0. };
    n.x = edge.y;
    n.y = -edge.x;
    let mut d: ::core::ffi::c_float = kmVec2Dot(&raw mut n, &raw mut other_edge);
    if d > 0.0f32 {
        n.x = -n.x;
        n.y = -n.y;
    }
    (*normal_out).x = n.x;
    (*normal_out).y = n.y;
    kmVec2Normalize(normal_out, normal_out);
}
#[no_mangle]
pub unsafe extern "C" fn kmRay2IntersectTriangle(
    mut ray: *const kmRay2,
    mut p1: *const kmVec2,
    mut p2: *const kmVec2,
    mut p3: *const kmVec2,
    mut intersection: *mut kmVec2,
    mut normal_out: *mut kmVec2,
    mut distance_out: *mut ::core::ffi::c_float,
) -> ::core::ffi::c_uchar {
    let mut intersect: kmVec2 = kmVec2 { x: 0., y: 0. };
    let mut final_intersect: kmVec2 = kmVec2 { x: 0., y: 0. };
    let mut normal: kmVec2 = kmVec2 { x: 0., y: 0. };
    let mut distance: ::core::ffi::c_float = 10000.0f32;
    let mut intersected: ::core::ffi::c_uchar = KM_FALSE as ::core::ffi::c_uchar;
    if kmRay2IntersectLineSegment(ray, p1, p2, &raw mut intersect) != 0 {
        let mut tmp: kmVec2 = kmVec2 { x: 0., y: 0. };
        let mut this_distance: ::core::ffi::c_float = kmVec2Length(kmVec2Subtract(
            &raw mut tmp,
            &raw mut intersect,
            &raw const (*ray).start,
        ));
        let mut this_normal: kmVec2 = kmVec2 { x: 0., y: 0. };
        calculate_line_normal(*p1, *p2, *p3, &raw mut this_normal);
        if this_distance < distance
            && kmVec2Dot(&raw mut this_normal, &raw const (*ray).dir) < 0.0f32
        {
            final_intersect.x = intersect.x;
            final_intersect.y = intersect.y;
            distance = this_distance;
            kmVec2Assign(&raw mut normal, &raw mut this_normal);
            intersected = KM_TRUE as ::core::ffi::c_uchar;
        }
    }
    if kmRay2IntersectLineSegment(ray, p2, p3, &raw mut intersect) != 0 {
        let mut tmp_0: kmVec2 = kmVec2 { x: 0., y: 0. };
        let mut this_distance_0: ::core::ffi::c_float = kmVec2Length(kmVec2Subtract(
            &raw mut tmp_0,
            &raw mut intersect,
            &raw const (*ray).start,
        ));
        let mut this_normal_0: kmVec2 = kmVec2 { x: 0., y: 0. };
        calculate_line_normal(*p2, *p3, *p1, &raw mut this_normal_0);
        if this_distance_0 < distance
            && kmVec2Dot(&raw mut this_normal_0, &raw const (*ray).dir) < 0.0f32
        {
            final_intersect.x = intersect.x;
            final_intersect.y = intersect.y;
            distance = this_distance_0;
            kmVec2Assign(&raw mut normal, &raw mut this_normal_0);
            intersected = KM_TRUE as ::core::ffi::c_uchar;
        }
    }
    if kmRay2IntersectLineSegment(ray, p3, p1, &raw mut intersect) != 0 {
        let mut tmp_1: kmVec2 = kmVec2 { x: 0., y: 0. };
        let mut this_distance_1: ::core::ffi::c_float = kmVec2Length(kmVec2Subtract(
            &raw mut tmp_1,
            &raw mut intersect,
            &raw const (*ray).start,
        ));
        let mut this_normal_1: kmVec2 = kmVec2 { x: 0., y: 0. };
        calculate_line_normal(*p3, *p1, *p2, &raw mut this_normal_1);
        if this_distance_1 < distance
            && kmVec2Dot(&raw mut this_normal_1, &raw const (*ray).dir) < 0.0f32
        {
            final_intersect.x = intersect.x;
            final_intersect.y = intersect.y;
            distance = this_distance_1;
            kmVec2Assign(&raw mut normal, &raw mut this_normal_1);
            intersected = KM_TRUE as ::core::ffi::c_uchar;
        }
    }
    if intersected != 0 {
        (*intersection).x = final_intersect.x;
        (*intersection).y = final_intersect.y;
        if !normal_out.is_null() {
            (*normal_out).x = normal.x;
            (*normal_out).y = normal.y;
        }
        if distance != 0. {
            *distance_out = distance;
        }
    }
    return intersected;
}
#[no_mangle]
pub unsafe extern "C" fn kmRay2IntersectBox(
    mut ray: *const kmRay2,
    mut p1: *const kmVec2,
    mut p2: *const kmVec2,
    mut p3: *const kmVec2,
    mut p4: *const kmVec2,
    mut intersection: *mut kmVec2,
    mut normal_out: *mut kmVec2,
) -> ::core::ffi::c_uchar {
    let mut intersected: ::core::ffi::c_uchar = KM_FALSE as ::core::ffi::c_uchar;
    let mut intersect: kmVec2 = kmVec2 { x: 0., y: 0. };
    let mut final_intersect: kmVec2 = kmVec2 { x: 0., y: 0. };
    let mut normal: kmVec2 = kmVec2 { x: 0., y: 0. };
    let mut distance: ::core::ffi::c_float = 10000.0f32;
    let mut points: [*const kmVec2; 4] = [::core::ptr::null::<kmVec2>(); 4];
    points[0 as ::core::ffi::c_int as usize] = p1;
    points[1 as ::core::ffi::c_int as usize] = p2;
    points[2 as ::core::ffi::c_int as usize] = p3;
    points[3 as ::core::ffi::c_int as usize] = p4;
    let mut i: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    while i < 4 as ::core::ffi::c_uint {
        let mut this_point: *const kmVec2 = points[i as usize];
        let mut next_point: *const kmVec2 = if i == 3 as ::core::ffi::c_uint {
            points[0 as ::core::ffi::c_int as usize]
        } else {
            points[i.wrapping_add(1 as ::core::ffi::c_uint) as usize]
        };
        let mut other_point: *const kmVec2 =
            if i == 3 as ::core::ffi::c_uint || i == 0 as ::core::ffi::c_uint {
                points[1 as ::core::ffi::c_int as usize]
            } else {
                points[0 as ::core::ffi::c_int as usize]
            };
        if kmRay2IntersectLineSegment(ray, this_point, next_point, &raw mut intersect) != 0 {
            let mut tmp: kmVec2 = kmVec2 { x: 0., y: 0. };
            let mut this_distance: ::core::ffi::c_float = kmVec2Length(kmVec2Subtract(
                &raw mut tmp,
                &raw mut intersect,
                &raw const (*ray).start,
            ));
            let mut this_normal: kmVec2 = kmVec2 { x: 0., y: 0. };
            calculate_line_normal(*this_point, *next_point, *other_point, &raw mut this_normal);
            if this_distance < distance
                && kmVec2Dot(&raw mut this_normal, &raw const (*ray).dir) < 0.0f32
            {
                kmVec2Assign(&raw mut final_intersect, &raw mut intersect);
                distance = this_distance;
                intersected = KM_TRUE as ::core::ffi::c_uchar;
                kmVec2Assign(&raw mut normal, &raw mut this_normal);
            }
        }
        i = i.wrapping_add(1);
    }
    if intersected != 0 {
        (*intersection).x = final_intersect.x;
        (*intersection).y = final_intersect.y;
        if !normal_out.is_null() {
            (*normal_out).x = normal.x;
            (*normal_out).y = normal.y;
        }
    }
    return intersected;
}
#[no_mangle]
pub unsafe extern "C" fn kmRay2IntersectCircle(
    mut ray: *const kmRay2,
    centre: kmVec2,
    radius: ::core::ffi::c_float,
    mut intersection: *mut kmVec2,
) -> ::core::ffi::c_uchar {
    return KM_TRUE as ::core::ffi::c_uchar;
}
