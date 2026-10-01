extern "C" {
    fn heman_image_create(
        width: ::core::ffi::c_int,
        height: ::core::ffi::c_int,
        nbands: ::core::ffi::c_int,
    ) -> *mut heman_image;
    fn heman_image_texel(
        _: *mut heman_image,
        x: ::core::ffi::c_int,
        y: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_float;
    fn heman_image_destroy(_: *mut heman_image);
    static mut _gamma: ::core::ffi::c_float;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn abs(__x: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn atan(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn pow(__x: ::core::ffi::c_double, __y: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn kmMax(lhs: ::core::ffi::c_float, rhs: ::core::ffi::c_float) -> ::core::ffi::c_float;
    fn kmClamp(
        x: ::core::ffi::c_float,
        min: ::core::ffi::c_float,
        max: ::core::ffi::c_float,
    ) -> ::core::ffi::c_float;
    fn kmVec3Length(pIn: *const kmVec3) -> ::core::ffi::c_float;
    fn kmVec3Lerp(
        pOut: *mut kmVec3,
        pV1: *const kmVec3,
        pV2: *const kmVec3,
        t: ::core::ffi::c_float,
    ) -> *mut kmVec3;
    fn kmVec3Normalize(pOut: *mut kmVec3, pIn: *const kmVec3) -> *mut kmVec3;
    fn kmVec3Cross(pOut: *mut kmVec3, pV1: *const kmVec3, pV2: *const kmVec3) -> *mut kmVec3;
    fn kmVec3Dot(pV1: *const kmVec3, pV2: *const kmVec3) -> ::core::ffi::c_float;
    fn kmVec3Subtract(pOut: *mut kmVec3, pV1: *const kmVec3, pV2: *const kmVec3) -> *mut kmVec3;
    fn kmVec3Scale(pOut: *mut kmVec3, pIn: *const kmVec3, s: ::core::ffi::c_float) -> *mut kmVec3;
    static KM_VEC3_POS_Z: kmVec3;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct heman_image_s {
    pub width: ::core::ffi::c_int,
    pub height: ::core::ffi::c_int,
    pub nbands: ::core::ffi::c_int,
    pub data: *mut ::core::ffi::c_float,
}
pub type heman_image = heman_image_s;
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kmVec3 {
    pub x: ::core::ffi::c_float,
    pub y: ::core::ffi::c_float,
    pub z: ::core::ffi::c_float,
}
pub const TWO_OVER_PI: ::core::ffi::c_double = 0.63661977236f64;
static mut _occlusion_scale: ::core::ffi::c_float = 1.0f32;
#[no_mangle]
pub unsafe extern "C" fn heman_lighting_set_occlusion_scale(mut s: ::core::ffi::c_float) {
    _occlusion_scale = s;
}
#[no_mangle]
pub unsafe extern "C" fn heman_lighting_compute_normals(
    mut heightmap: *mut heman_image,
) -> *mut heman_image {
    let mut width: ::core::ffi::c_int = (*heightmap).width;
    let mut height: ::core::ffi::c_int = (*heightmap).height;
    let mut result: *mut heman_image = heman_image_create(width, height, 3 as ::core::ffi::c_int);
    let mut invh: ::core::ffi::c_float = 1.0f32 / height as ::core::ffi::c_float;
    let mut invw: ::core::ffi::c_float = 1.0f32 / width as ::core::ffi::c_float;
    let mut maxx: ::core::ffi::c_int = width - 1 as ::core::ffi::c_int;
    let mut maxy: ::core::ffi::c_int = height - 1 as ::core::ffi::c_int;
    let mut normals: *mut kmVec3 = (*result).data as *mut kmVec3;
    let mut y: ::core::ffi::c_int = 0;
    y = 0 as ::core::ffi::c_int;
    while y < height {
        let mut v: ::core::ffi::c_float = y as ::core::ffi::c_float * invh;
        let mut y1: ::core::ffi::c_int = if y + 1 as ::core::ffi::c_int > maxy {
            maxy
        } else {
            y + 1 as ::core::ffi::c_int
        };
        let mut p: kmVec3 = kmVec3 {
            x: 0.,
            y: 0.,
            z: 0.,
        };
        let mut px: kmVec3 = kmVec3 {
            x: 0.,
            y: 0.,
            z: 0.,
        };
        let mut py: kmVec3 = kmVec3 {
            x: 0.,
            y: 0.,
            z: 0.,
        };
        let mut n: *mut kmVec3 = normals.offset((y * width) as isize);
        let mut x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while x < width {
            let mut u: ::core::ffi::c_float = x as ::core::ffi::c_float * invw;
            let mut x1: ::core::ffi::c_int = if x + 1 as ::core::ffi::c_int > maxx {
                maxx
            } else {
                x + 1 as ::core::ffi::c_int
            };
            p.x = u;
            p.y = v;
            p.z = *heman_image_texel(heightmap, x, y);
            px.x = u + invw;
            px.y = v;
            px.z = *heman_image_texel(heightmap, x1, y);
            py.x = u;
            py.y = v + invh;
            py.z = *heman_image_texel(heightmap, x, y1);
            kmVec3Subtract(&raw mut px, &raw mut px, &raw mut p);
            kmVec3Subtract(&raw mut py, &raw mut py, &raw mut p);
            kmVec3Cross(n, &raw mut px, &raw mut py);
            kmVec3Normalize(n, n);
            (*n).y *= -(1 as ::core::ffi::c_int) as ::core::ffi::c_float;
            x += 1;
            n = n.offset(1);
        }
        y += 1;
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn heman_lighting_apply(
    mut heightmap: *mut heman_image,
    mut albedo: *mut heman_image,
    mut occlusion: ::core::ffi::c_float,
    mut diffuse: ::core::ffi::c_float,
    mut diffuse_softening: ::core::ffi::c_float,
    mut light_position: *const ::core::ffi::c_float,
) -> *mut heman_image {
    let mut width: ::core::ffi::c_int = (*heightmap).width;
    let mut height: ::core::ffi::c_int = (*heightmap).height;
    let mut final_0: *mut heman_image = heman_image_create(width, height, 3 as ::core::ffi::c_int);
    let mut normals: *mut heman_image = heman_lighting_compute_normals(heightmap);
    let mut occ: *mut heman_image = heman_lighting_compute_occlusion(heightmap);
    !albedo.is_null();
    static mut default_pos: [::core::ffi::c_float; 3] = [-0.5f32, 0.5f32, 1.0f32];
    if light_position.is_null() {
        light_position = &raw mut default_pos as *mut ::core::ffi::c_float;
    }
    let mut colors: *mut kmVec3 = (*final_0).data as *mut kmVec3;
    let mut invgamma: ::core::ffi::c_float = 1.0f32 / _gamma;
    let mut L: kmVec3 = kmVec3 {
        x: 0.,
        y: 0.,
        z: 0.,
    };
    L.x = *light_position.offset(0 as ::core::ffi::c_int as isize);
    L.y = *light_position.offset(1 as ::core::ffi::c_int as isize);
    L.z = *light_position.offset(2 as ::core::ffi::c_int as isize);
    kmVec3Normalize(&raw mut L, &raw mut L);
    let mut y: ::core::ffi::c_int = 0;
    y = 0 as ::core::ffi::c_int;
    while y < height {
        let mut color: *mut kmVec3 = colors.offset((y * width) as isize);
        let mut x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while x < width {
            let mut N: *mut kmVec3 = heman_image_texel(normals, x, y) as *mut kmVec3;
            kmVec3Lerp(N, N, &raw const KM_VEC3_POS_Z, diffuse_softening);
            let mut df: ::core::ffi::c_float = 1 as ::core::ffi::c_int as ::core::ffi::c_float
                - diffuse
                    * (1 as ::core::ffi::c_int as ::core::ffi::c_float
                        - kmClamp(
                            kmVec3Dot(N, &raw mut L),
                            0 as ::core::ffi::c_int as ::core::ffi::c_float,
                            1 as ::core::ffi::c_int as ::core::ffi::c_float,
                        ));
            let mut of: ::core::ffi::c_float = 1 as ::core::ffi::c_int as ::core::ffi::c_float
                - occlusion
                    * (1 as ::core::ffi::c_int as ::core::ffi::c_float
                        - *heman_image_texel(occ, x, y));
            if !albedo.is_null() {
                *color = *(heman_image_texel(albedo, x, y) as *mut kmVec3);
            } else {
                (*color).z = 1 as ::core::ffi::c_int as ::core::ffi::c_float;
                (*color).y = (*color).z;
                (*color).x = (*color).y;
            }
            (*color).x = pow(
                (*color).x as ::core::ffi::c_double,
                _gamma as ::core::ffi::c_double,
            ) as ::core::ffi::c_float;
            (*color).y = pow(
                (*color).y as ::core::ffi::c_double,
                _gamma as ::core::ffi::c_double,
            ) as ::core::ffi::c_float;
            (*color).z = pow(
                (*color).z as ::core::ffi::c_double,
                _gamma as ::core::ffi::c_double,
            ) as ::core::ffi::c_float;
            kmVec3Scale(color, color, df * of);
            (*color).x = pow(
                (*color).x as ::core::ffi::c_double,
                invgamma as ::core::ffi::c_double,
            ) as ::core::ffi::c_float;
            (*color).y = pow(
                (*color).y as ::core::ffi::c_double,
                invgamma as ::core::ffi::c_double,
            ) as ::core::ffi::c_float;
            (*color).z = pow(
                (*color).z as ::core::ffi::c_double,
                invgamma as ::core::ffi::c_double,
            ) as ::core::ffi::c_float;
            x += 1;
            color = color.offset(1);
        }
        y += 1;
    }
    heman_image_destroy(normals);
    heman_image_destroy(occ);
    return final_0;
}
pub const NUM_SCANS: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const INV_SCANS: ::core::ffi::c_float = 1.0f32 / 16.0f32;
unsafe extern "C" fn azimuth_slope(mut a: kmVec3, mut b: kmVec3) -> ::core::ffi::c_float {
    let mut d: kmVec3 = kmVec3 {
        x: 0.,
        y: 0.,
        z: 0.,
    };
    kmVec3Subtract(&raw mut d, &raw mut a, &raw mut b);
    let mut x: ::core::ffi::c_float = kmVec3Length(&raw mut d);
    let mut y: ::core::ffi::c_float = b.z - a.z;
    return y / x;
}
unsafe extern "C" fn compute_occlusion(
    mut thispt: kmVec3,
    mut horizonpt: kmVec3,
) -> ::core::ffi::c_float {
    let mut direction: kmVec3 = kmVec3 {
        x: 0.,
        y: 0.,
        z: 0.,
    };
    kmVec3Subtract(&raw mut direction, &raw mut horizonpt, &raw mut thispt);
    kmVec3Normalize(&raw mut direction, &raw mut direction);
    let mut dot: ::core::ffi::c_float = kmVec3Dot(&raw mut direction, &raw const KM_VEC3_POS_Z);
    return (atan((if dot > 0.0f32 { dot } else { 0.0f32 }) as ::core::ffi::c_double) * TWO_OVER_PI)
        as ::core::ffi::c_float;
}
unsafe extern "C" fn horizon_scan(
    mut heightmap: *mut heman_image,
    mut result: *mut heman_image,
    mut startpts: *mut ::core::ffi::c_int,
    mut dx: ::core::ffi::c_int,
    mut dy: ::core::ffi::c_int,
) {
    let mut w: ::core::ffi::c_int = (*heightmap).width;
    let mut h: ::core::ffi::c_int = (*heightmap).height;
    let mut sx: ::core::ffi::c_int = (dx > 0 as ::core::ffi::c_int) as ::core::ffi::c_int
        - (dx < 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
    let mut sy: ::core::ffi::c_int = (dy > 0 as ::core::ffi::c_int) as ::core::ffi::c_int
        - (dy < 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
    let mut ax: ::core::ffi::c_int = abs(dx);
    let mut ay: ::core::ffi::c_int = abs(dy);
    let mut nsweeps: ::core::ffi::c_int = ay * w + ax * h - (ax + ay - 1 as ::core::ffi::c_int);
    let mut p: *mut ::core::ffi::c_int = startpts;
    let mut x: ::core::ffi::c_int = -ax;
    while x < w - ax {
        let mut y: ::core::ffi::c_int = -ay;
        while y < h - ay {
            if !(x >= 0 as ::core::ffi::c_int && x < w && y >= 0 as ::core::ffi::c_int && y < h) {
                let fresh0 = p;
                p = p.offset(1);
                *fresh0 = if sx < 0 as ::core::ffi::c_int {
                    w - x - 1 as ::core::ffi::c_int
                } else {
                    x
                };
                let fresh1 = p;
                p = p.offset(1);
                *fresh1 = if sy < 0 as ::core::ffi::c_int {
                    h - y - 1 as ::core::ffi::c_int
                } else {
                    y
                };
            }
            y += 1;
        }
        x += 1;
    }
    let mut pathlen: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = *startpts.offset(0 as ::core::ffi::c_int as isize);
    let mut j: ::core::ffi::c_int = *startpts.offset(1 as ::core::ffi::c_int as isize);
    loop {
        i += dx;
        j += dy;
        pathlen += 1;
        if !(i >= 0 as ::core::ffi::c_int && i < w && j >= 0 as ::core::ffi::c_int && j < h) {
            break;
        }
    }
    let mut cellw: ::core::ffi::c_float =
        _occlusion_scale / (if w > h { w } else { h }) as ::core::ffi::c_float;
    let mut cellh: ::core::ffi::c_float =
        _occlusion_scale / (if w > h { w } else { h }) as ::core::ffi::c_float;
    let mut hull_buffer: *mut kmVec3 = malloc(
        (::core::mem::size_of::<kmVec3>() as size_t)
            .wrapping_mul(pathlen as size_t)
            .wrapping_mul(nsweeps as size_t),
    ) as *mut kmVec3;
    let mut sweep: ::core::ffi::c_int = 0;
    sweep = 0 as ::core::ffi::c_int;
    while sweep < nsweeps {
        let mut convex_hull: *mut kmVec3 = hull_buffer.offset((sweep * pathlen) as isize);
        let mut p_0: *mut ::core::ffi::c_int =
            startpts.offset((sweep * 2 as ::core::ffi::c_int) as isize);
        let mut i_0: ::core::ffi::c_int = *p_0.offset(0 as ::core::ffi::c_int as isize);
        let mut j_0: ::core::ffi::c_int = *p_0.offset(1 as ::core::ffi::c_int as isize);
        let mut thispt: kmVec3 = kmVec3 {
            x: 0.,
            y: 0.,
            z: 0.,
        };
        let mut horizonpt: kmVec3 = kmVec3 {
            x: 0.,
            y: 0.,
            z: 0.,
        };
        thispt.x = i_0 as ::core::ffi::c_float * cellw;
        thispt.y = j_0 as ::core::ffi::c_float * cellh;
        thispt.z = *heman_image_texel(
            heightmap,
            if 0 as ::core::ffi::c_int
                > (if w - 1 as ::core::ffi::c_int > i_0 {
                    i_0
                } else {
                    w - 1 as ::core::ffi::c_int
                })
            {
                0 as ::core::ffi::c_int
            } else if w - 1 as ::core::ffi::c_int > i_0 {
                i_0
            } else {
                w - 1 as ::core::ffi::c_int
            },
            if 0 as ::core::ffi::c_int
                > (if h - 1 as ::core::ffi::c_int > j_0 {
                    j_0
                } else {
                    h - 1 as ::core::ffi::c_int
                })
            {
                0 as ::core::ffi::c_int
            } else if h - 1 as ::core::ffi::c_int > j_0 {
                j_0
            } else {
                h - 1 as ::core::ffi::c_int
            },
        );
        let mut stack_top: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        *convex_hull.offset(0 as ::core::ffi::c_int as isize) = thispt;
        i_0 += dx;
        j_0 += dy;
        while i_0 >= 0 as ::core::ffi::c_int && i_0 < w && j_0 >= 0 as ::core::ffi::c_int && j_0 < h
        {
            thispt.x = i_0 as ::core::ffi::c_float * cellw;
            thispt.y = j_0 as ::core::ffi::c_float * cellh;
            thispt.z = *heman_image_texel(heightmap, i_0, j_0);
            while stack_top > 0 as ::core::ffi::c_int {
                let mut s1: ::core::ffi::c_float =
                    azimuth_slope(thispt, *convex_hull.offset(stack_top as isize));
                let mut s2: ::core::ffi::c_float = azimuth_slope(
                    thispt,
                    *convex_hull.offset((stack_top - 1 as ::core::ffi::c_int) as isize),
                );
                if s1 >= s2 {
                    break;
                }
                stack_top -= 1;
            }
            let fresh2 = stack_top;
            stack_top = stack_top + 1;
            horizonpt = *convex_hull.offset(fresh2 as isize);
            *convex_hull.offset(stack_top as isize) = thispt;
            let mut occlusion: ::core::ffi::c_float = compute_occlusion(thispt, horizonpt);
            *heman_image_texel(result, i_0, j_0) += INV_SCANS * occlusion;
            i_0 += dx;
            j_0 += dy;
        }
        sweep += 1;
    }
    free(hull_buffer as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn heman_lighting_compute_occlusion(
    mut heightmap: *mut heman_image,
) -> *mut heman_image {
    let mut width: ::core::ffi::c_int = (*heightmap).width;
    let mut height: ::core::ffi::c_int = (*heightmap).height;
    let mut result: *mut heman_image = heman_image_create(width, height, 1 as ::core::ffi::c_int);
    memset(
        (*result).data as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        (::core::mem::size_of::<::core::ffi::c_float>() as size_t)
            .wrapping_mul(width as size_t)
            .wrapping_mul(height as size_t),
    );
    let scans: [::core::ffi::c_int; 32] = [
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        -(1 as ::core::ffi::c_int),
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        -(1 as ::core::ffi::c_int),
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        1 as ::core::ffi::c_int,
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        1 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
        -(1 as ::core::ffi::c_int),
        -(2 as ::core::ffi::c_int),
        1 as ::core::ffi::c_int,
        -(2 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        1 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        -(2 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        2 as ::core::ffi::c_int,
        -(1 as ::core::ffi::c_int),
        -(2 as ::core::ffi::c_int),
    ];
    let mut startpts: *mut ::core::ffi::c_int = malloc(
        ((::core::mem::size_of::<::core::ffi::c_int>() as usize)
            .wrapping_mul(2 as usize)
            .wrapping_mul(3 as usize) as ::core::ffi::c_float
            * kmMax(
                width as ::core::ffi::c_float,
                height as ::core::ffi::c_float,
            )) as size_t,
    ) as *mut ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < NUM_SCANS {
        let mut dx: ::core::ffi::c_int = scans[(i * 2 as ::core::ffi::c_int) as usize];
        let mut dy: ::core::ffi::c_int =
            scans[(i * 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as usize];
        horizon_scan(heightmap, result, startpts, dx, dy);
        i += 1;
    }
    let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_0 < width * height {
        *(*result).data.offset(i_0 as isize) = 1.0f32 - *(*result).data.offset(i_0 as isize);
        i_0 += 1;
    }
    free(startpts as *mut ::core::ffi::c_void);
    return result;
}
