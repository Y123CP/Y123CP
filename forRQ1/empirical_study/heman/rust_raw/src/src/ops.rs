extern "C" {
    pub type osn_context;
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
    fn heman_color_to_grayscale(colorimg: *mut heman_image) -> *mut heman_image;
    fn heman_distance_identity_cpcf(
        width: ::core::ffi::c_int,
        height: ::core::ffi::c_int,
    ) -> *mut heman_image;
    fn open_simplex_noise(
        seed: int64_t,
        ctx: *mut *mut osn_context,
    ) -> ::core::ffi::c_int;
    fn open_simplex_noise_free(ctx: *mut osn_context);
    fn open_simplex_noise2(
        ctx: *mut osn_context,
        x: ::core::ffi::c_double,
        y: ::core::ffi::c_double,
    ) -> ::core::ffi::c_double;
    fn pow(
        __x: ::core::ffi::c_double,
        __y: ::core::ffi::c_double,
    ) -> ::core::ffi::c_double;
    fn floor(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn fabsf(__x: ::core::ffi::c_float) -> ::core::ffi::c_float;
    fn kmVec3Lerp(
        pOut: *mut kmVec3,
        pV1: *const kmVec3,
        pV2: *const kmVec3,
        t: ::core::ffi::c_float,
    ) -> *mut kmVec3;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn omp_get_max_threads() -> ::core::ffi::c_int;
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
pub type heman_points = heman_image_s;
pub type heman_color = ::core::ffi::c_uint;
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kmVec3 {
    pub x: ::core::ffi::c_float,
    pub y: ::core::ffi::c_float,
    pub z: ::core::ffi::c_float,
}
pub type int64_t = __int64_t;
pub type __int64_t = i64;
#[no_mangle]
pub unsafe extern "C" fn heman_get_num_threads() -> ::core::ffi::c_int {
    return omp_get_max_threads();
}
#[no_mangle]
pub unsafe extern "C" fn heman_ops_step(
    mut hmap: *mut heman_image,
    mut threshold: ::core::ffi::c_float,
) -> *mut heman_image {
    let mut result: *mut heman_image = heman_image_create(
        (*hmap).width,
        (*hmap).height,
        1 as ::core::ffi::c_int,
    );
    let mut size: ::core::ffi::c_int = (*hmap).height * (*hmap).width;
    let mut src: *mut ::core::ffi::c_float = (*hmap).data;
    let mut dst: *mut ::core::ffi::c_float = (*result).data;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < size {
        let fresh5 = src;
        src = src.offset(1);
        let fresh6 = dst;
        dst = dst.offset(1);
        *fresh6 = (if *fresh5 >= threshold {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as ::core::ffi::c_float;
        i += 1;
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn heman_ops_max(
    mut imga: *mut heman_image,
    mut imgb: *mut heman_image,
) -> *mut heman_image {
    let mut result: *mut heman_image = heman_image_create(
        (*imga).width,
        (*imga).height,
        (*imga).nbands,
    );
    let mut size: ::core::ffi::c_int = (*imga).height * (*imga).width * (*imga).nbands;
    let mut srca: *mut ::core::ffi::c_float = (*imga).data;
    let mut srcb: *mut ::core::ffi::c_float = (*imgb).data;
    let mut dst: *mut ::core::ffi::c_float = (*result).data;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < size {
        *dst = if *srca > *srcb { *srca } else { *srcb };
        i += 1;
        dst = dst.offset(1);
        srca = srca.offset(1);
        srcb = srcb.offset(1);
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn heman_ops_sweep(
    mut hmap: *mut heman_image,
) -> *mut heman_image {
    let mut result: *mut heman_image = heman_image_create(
        (*hmap).height,
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
    );
    let mut dst: *mut ::core::ffi::c_float = (*result).data;
    let mut src: *const ::core::ffi::c_float = (*hmap).data;
    let mut invw: ::core::ffi::c_float = 1.0f32 / (*hmap).width as ::core::ffi::c_float;
    let mut y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while y < (*hmap).height {
        let mut acc: ::core::ffi::c_float = 0 as ::core::ffi::c_int
            as ::core::ffi::c_float;
        let mut x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while x < (*hmap).width {
            let fresh10 = src;
            src = src.offset(1);
            acc += *fresh10;
            x += 1;
        }
        let fresh11 = dst;
        dst = dst.offset(1);
        *fresh11 = acc * invw;
        y += 1;
    }
    return result;
}
unsafe extern "C" fn copy_row(
    mut src: *mut heman_image,
    mut dst: *mut heman_image,
    mut dstx: ::core::ffi::c_int,
    mut y: ::core::ffi::c_int,
) {
    let mut width: ::core::ffi::c_int = (*src).width;
    if (*src).nbands == 1 as ::core::ffi::c_int {
        let mut x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while x < width {
            let mut srcp: *mut ::core::ffi::c_float = heman_image_texel(src, x, y);
            let mut dstp: *mut ::core::ffi::c_float = heman_image_texel(
                dst,
                dstx + x,
                y,
            );
            *dstp = *srcp;
            x += 1;
        }
        return;
    }
    let mut x_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while x_0 < width {
        let mut srcp_0: *mut ::core::ffi::c_float = heman_image_texel(src, x_0, y);
        let mut dstp_0: *mut ::core::ffi::c_float = heman_image_texel(
            dst,
            dstx + x_0,
            y,
        );
        let mut nbands: ::core::ffi::c_int = (*src).nbands;
        loop {
            let fresh0 = nbands;
            nbands = nbands - 1;
            if !(fresh0 != 0) {
                break;
            }
            let fresh1 = srcp_0;
            srcp_0 = srcp_0.offset(1);
            let fresh2 = dstp_0;
            dstp_0 = dstp_0.offset(1);
            *fresh2 = *fresh1;
        }
        x_0 += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn heman_ops_stitch_horizontal(
    mut images: *mut *mut heman_image,
    mut count: ::core::ffi::c_int,
) -> *mut heman_image {
    let mut width: ::core::ffi::c_int = (**images
        .offset(0 as ::core::ffi::c_int as isize))
        .width;
    let mut height: ::core::ffi::c_int = (**images
        .offset(0 as ::core::ffi::c_int as isize))
        .height;
    let mut nbands: ::core::ffi::c_int = (**images
        .offset(0 as ::core::ffi::c_int as isize))
        .nbands;
    let mut i: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    while i < count {
        i += 1;
    }
    let mut result: *mut heman_image = heman_image_create(width * count, height, nbands);
    let mut y: ::core::ffi::c_int = 0;
    y = 0 as ::core::ffi::c_int;
    while y < height {
        let mut tile: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while tile < count {
            copy_row(*images.offset(tile as isize), result, tile * width, y);
            tile += 1;
        }
        y += 1;
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn heman_ops_stitch_vertical(
    mut images: *mut *mut heman_image,
    mut count: ::core::ffi::c_int,
) -> *mut heman_image {
    let mut width: ::core::ffi::c_int = (**images
        .offset(0 as ::core::ffi::c_int as isize))
        .width;
    let mut height: ::core::ffi::c_int = (**images
        .offset(0 as ::core::ffi::c_int as isize))
        .height;
    let mut nbands: ::core::ffi::c_int = (**images
        .offset(0 as ::core::ffi::c_int as isize))
        .nbands;
    let mut i: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    while i < count {
        i += 1;
    }
    let mut result: *mut heman_image = heman_image_create(width, height * count, nbands);
    let mut size: ::core::ffi::c_int = width * height * nbands;
    let mut dst: *mut ::core::ffi::c_float = (*result).data;
    let mut tile: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while tile < count {
        memcpy(
            dst as *mut ::core::ffi::c_void,
            (**images.offset(tile as isize)).data as *const ::core::ffi::c_void,
            (size as size_t)
                .wrapping_mul(::core::mem::size_of::<::core::ffi::c_float>() as size_t),
        );
        dst = dst.offset(size as isize);
        tile += 1;
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn heman_ops_normalize_f32(
    mut source: *mut heman_image,
    mut minv: ::core::ffi::c_float,
    mut maxv: ::core::ffi::c_float,
) -> *mut heman_image {
    let mut result: *mut heman_image = heman_image_create(
        (*source).width,
        (*source).height,
        (*source).nbands,
    );
    let mut src: *mut ::core::ffi::c_float = (*source).data;
    let mut dst: *mut ::core::ffi::c_float = (*result).data;
    let mut scale: ::core::ffi::c_float = 1.0f32 / (maxv - minv);
    let mut size: ::core::ffi::c_int = (*source).height * (*source).width
        * (*source).nbands;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < size {
        let fresh3 = src;
        src = src.offset(1);
        let mut v: ::core::ffi::c_float = (*fresh3 - minv) * scale;
        let fresh4 = dst;
        dst = dst.offset(1);
        *fresh4 = if 0 as ::core::ffi::c_int as ::core::ffi::c_float
            > (if 1 as ::core::ffi::c_int as ::core::ffi::c_float > v {
                v
            } else {
                1 as ::core::ffi::c_int as ::core::ffi::c_float
            })
        {
            0 as ::core::ffi::c_int as ::core::ffi::c_float
        } else if 1 as ::core::ffi::c_int as ::core::ffi::c_float > v {
            v
        } else {
            1 as ::core::ffi::c_int as ::core::ffi::c_float
        };
        i += 1;
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn heman_ops_laplacian(
    mut heightmap: *mut heman_image,
) -> *mut heman_image {
    let mut width: ::core::ffi::c_int = (*heightmap).width;
    let mut height: ::core::ffi::c_int = (*heightmap).height;
    let mut result: *mut heman_image = heman_image_create(
        width,
        height,
        1 as ::core::ffi::c_int,
    );
    let mut maxx: ::core::ffi::c_int = width - 1 as ::core::ffi::c_int;
    let mut maxy: ::core::ffi::c_int = height - 1 as ::core::ffi::c_int;
    let mut y: ::core::ffi::c_int = 0;
    y = 0 as ::core::ffi::c_int;
    while y < height {
        let mut y1: ::core::ffi::c_int = if y + 1 as ::core::ffi::c_int > maxy {
            maxy
        } else {
            y + 1 as ::core::ffi::c_int
        };
        let mut dst: *mut ::core::ffi::c_float = (*result)
            .data
            .offset((y * width) as isize);
        let mut x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while x < width {
            let mut x1: ::core::ffi::c_int = if x + 1 as ::core::ffi::c_int > maxx {
                maxx
            } else {
                x + 1 as ::core::ffi::c_int
            };
            let mut p: ::core::ffi::c_float = *heman_image_texel(heightmap, x, y);
            let mut px: ::core::ffi::c_float = *heman_image_texel(heightmap, x1, y);
            let mut py: ::core::ffi::c_float = *heman_image_texel(heightmap, x, y1);
            let fresh12 = dst;
            dst = dst.offset(1);
            *fresh12 = (p - px) * (p - px) + (p - py) * (p - py);
            x += 1;
        }
        y += 1;
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn heman_ops_accumulate(
    mut dst: *mut heman_image,
    mut src: *mut heman_image,
) {
    let mut size: ::core::ffi::c_int = (*dst).height * (*dst).width;
    let mut sdata: *mut ::core::ffi::c_float = (*src).data;
    let mut ddata: *mut ::core::ffi::c_float = (*dst).data;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < size {
        let fresh15 = sdata;
        sdata = sdata.offset(1);
        let fresh16 = ddata;
        ddata = ddata.offset(1);
        *fresh16 += *fresh15;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn heman_ops_sobel(
    mut img: *mut heman_image,
    mut rgb: heman_color,
) -> *mut heman_image {
    let mut width: ::core::ffi::c_int = (*img).width;
    let mut height: ::core::ffi::c_int = (*img).height;
    let mut result: *mut heman_image = heman_image_create(
        width,
        height,
        3 as ::core::ffi::c_int,
    );
    let mut gray: *mut heman_image = heman_color_to_grayscale(img);
    let mut inv: ::core::ffi::c_float = 1.0f32 / 255.0f32;
    let mut edge_rgb: kmVec3 = kmVec3 { x: 0., y: 0., z: 0. };
    edge_rgb.x = (rgb >> 16 as ::core::ffi::c_int) as ::core::ffi::c_float * inv;
    edge_rgb.y = (rgb as ::core::ffi::c_uint >> 8 as ::core::ffi::c_int
        & 0xff as ::core::ffi::c_uint) as ::core::ffi::c_float * inv;
    edge_rgb.z = (rgb as ::core::ffi::c_uint & 0xff as ::core::ffi::c_uint)
        as ::core::ffi::c_float * inv;
    let mut y: ::core::ffi::c_int = 0;
    y = 0 as ::core::ffi::c_int;
    while y < height {
        let mut dst: *mut kmVec3 = ((*result).data as *mut kmVec3)
            .offset((y * width) as isize);
        let mut src: *const kmVec3 = ((*img).data as *mut kmVec3)
            .offset((y * width) as isize);
        let mut x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while x < width {
            let mut xm1: ::core::ffi::c_int = if x - 1 as ::core::ffi::c_int
                > 0 as ::core::ffi::c_int
            {
                x - 1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            };
            let mut xp1: ::core::ffi::c_int = if x + 1 as ::core::ffi::c_int
                > width - 1 as ::core::ffi::c_int
            {
                width - 1 as ::core::ffi::c_int
            } else {
                x + 1 as ::core::ffi::c_int
            };
            let mut ym1: ::core::ffi::c_int = if y - 1 as ::core::ffi::c_int
                > 0 as ::core::ffi::c_int
            {
                y - 1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            };
            let mut yp1: ::core::ffi::c_int = if y + 1 as ::core::ffi::c_int
                > height - 1 as ::core::ffi::c_int
            {
                height - 1 as ::core::ffi::c_int
            } else {
                y + 1 as ::core::ffi::c_int
            };
            let mut t00: ::core::ffi::c_float = *heman_image_texel(gray, xm1, ym1);
            let mut t10: ::core::ffi::c_float = *heman_image_texel(gray, x, ym1);
            let mut t20: ::core::ffi::c_float = *heman_image_texel(gray, xp1, ym1);
            let mut t01: ::core::ffi::c_float = *heman_image_texel(
                gray,
                xm1,
                0 as ::core::ffi::c_int,
            );
            let mut t21: ::core::ffi::c_float = *heman_image_texel(
                gray,
                xp1,
                0 as ::core::ffi::c_int,
            );
            let mut t02: ::core::ffi::c_float = *heman_image_texel(gray, xm1, yp1);
            let mut t12: ::core::ffi::c_float = *heman_image_texel(gray, x, yp1);
            let mut t22: ::core::ffi::c_float = *heman_image_texel(gray, xp1, yp1);
            let mut gx: ::core::ffi::c_float = (t00 as ::core::ffi::c_double
                + 2.0f64 * t01 as ::core::ffi::c_double + t02 as ::core::ffi::c_double
                - t20 as ::core::ffi::c_double - 2.0f64 * t21 as ::core::ffi::c_double
                - t22 as ::core::ffi::c_double) as ::core::ffi::c_float;
            let mut gy: ::core::ffi::c_float = (t00 as ::core::ffi::c_double
                + 2.0f64 * t10 as ::core::ffi::c_double + t20 as ::core::ffi::c_double
                - t02 as ::core::ffi::c_double - 2.0f64 * t12 as ::core::ffi::c_double
                - t22 as ::core::ffi::c_double) as ::core::ffi::c_float;
            let mut is_edge: ::core::ffi::c_float = ((gx * gx + gy * gy)
                as ::core::ffi::c_double > 1e-5f64) as ::core::ffi::c_int
                as ::core::ffi::c_float;
            let fresh13 = dst;
            dst = dst.offset(1);
            let fresh14 = src;
            src = src.offset(1);
            kmVec3Lerp(fresh13, fresh14, &raw mut edge_rgb, is_edge);
            x += 1;
        }
        y += 1;
    }
    heman_image_destroy(gray);
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn heman_ops_warp_core(
    mut img: *mut heman_image,
    mut secondary: *mut heman_image,
    mut seed: ::core::ffi::c_int,
    mut octaves: ::core::ffi::c_int,
) -> *mut heman_image {
    let mut ctx: *mut osn_context = ::core::ptr::null_mut::<osn_context>();
    open_simplex_noise(seed as int64_t, &raw mut ctx);
    let mut width: ::core::ffi::c_int = (*img).width;
    let mut height: ::core::ffi::c_int = (*img).height;
    let mut nbands: ::core::ffi::c_int = (*img).nbands;
    let mut result: *mut heman_image = heman_image_create(width, height, nbands);
    let mut result2: *mut heman_image = if !secondary.is_null() {
        heman_image_create(width, height, (*secondary).nbands)
    } else {
        ::core::ptr::null_mut::<heman_image>()
    };
    let mut invw: ::core::ffi::c_float = (1.0f64 / width as ::core::ffi::c_double)
        as ::core::ffi::c_float;
    let mut invh: ::core::ffi::c_float = (1.0f64 / height as ::core::ffi::c_double)
        as ::core::ffi::c_float;
    let mut inv: ::core::ffi::c_float = if invw > invh { invh } else { invw };
    let mut aspect: ::core::ffi::c_float = width as ::core::ffi::c_float
        / height as ::core::ffi::c_float;
    let mut gain: ::core::ffi::c_float = 0.6f32;
    let mut lacunarity: ::core::ffi::c_float = 2.0f32;
    let mut initial_amplitude: ::core::ffi::c_float = 0.05f32;
    let mut initial_frequency: ::core::ffi::c_float = 8.0f32;
    let mut y: ::core::ffi::c_int = 0;
    y = 0 as ::core::ffi::c_int;
    while y < height {
        let mut dst: *mut ::core::ffi::c_float = (*result)
            .data
            .offset((y * width * nbands) as isize);
        let mut x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while x < width {
            let mut a: ::core::ffi::c_float = initial_amplitude;
            let mut f: ::core::ffi::c_float = initial_frequency;
            let mut src: *mut ::core::ffi::c_float = ::core::ptr::null_mut::<
                ::core::ffi::c_float,
            >();
            if nbands == 4 as ::core::ffi::c_int {
                src = heman_image_texel(img, x, y);
                let mut elev: ::core::ffi::c_float = 1 as ::core::ffi::c_int
                    as ::core::ffi::c_float
                    - *src.offset(3 as ::core::ffi::c_int as isize);
                a = (a as ::core::ffi::c_double
                    * pow(
                        elev as ::core::ffi::c_double,
                        4 as ::core::ffi::c_int as ::core::ffi::c_double,
                    )) as ::core::ffi::c_float;
            }
            let mut s: ::core::ffi::c_float = x as ::core::ffi::c_float * inv;
            let mut t: ::core::ffi::c_float = y as ::core::ffi::c_float * inv;
            let mut u: ::core::ffi::c_float = x as ::core::ffi::c_float * invw;
            let mut v: ::core::ffi::c_float = y as ::core::ffi::c_float * invh;
            let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i < octaves {
                u = (u as ::core::ffi::c_double
                    + a as ::core::ffi::c_double
                        * open_simplex_noise2(
                            ctx,
                            (s * f) as ::core::ffi::c_double,
                            (t * f) as ::core::ffi::c_double,
                        )) as ::core::ffi::c_float;
                v = (v as ::core::ffi::c_double
                    + aspect as ::core::ffi::c_double
                        * (a as ::core::ffi::c_double
                            * open_simplex_noise2(
                                ctx,
                                (s * f) as ::core::ffi::c_double + 0.5f64,
                                (t * f) as ::core::ffi::c_double,
                            ))) as ::core::ffi::c_float;
                a *= gain;
                f *= lacunarity;
                i += 1;
            }
            let mut i_0: ::core::ffi::c_int = (if 0 as ::core::ffi::c_int
                as ::core::ffi::c_float
                > (if (width - 1 as ::core::ffi::c_int) as ::core::ffi::c_float
                    > u * width as ::core::ffi::c_float
                {
                    u * width as ::core::ffi::c_float
                } else {
                    (width - 1 as ::core::ffi::c_int) as ::core::ffi::c_float
                })
            {
                0 as ::core::ffi::c_int as ::core::ffi::c_float
            } else if (width - 1 as ::core::ffi::c_int) as ::core::ffi::c_float
                > u * width as ::core::ffi::c_float
            {
                u * width as ::core::ffi::c_float
            } else {
                (width - 1 as ::core::ffi::c_int) as ::core::ffi::c_float
            }) as ::core::ffi::c_int;
            let mut j: ::core::ffi::c_int = (if 0 as ::core::ffi::c_int
                as ::core::ffi::c_float
                > (if (height - 1 as ::core::ffi::c_int) as ::core::ffi::c_float
                    > v * height as ::core::ffi::c_float
                {
                    v * height as ::core::ffi::c_float
                } else {
                    (height - 1 as ::core::ffi::c_int) as ::core::ffi::c_float
                })
            {
                0 as ::core::ffi::c_int as ::core::ffi::c_float
            } else if (height - 1 as ::core::ffi::c_int) as ::core::ffi::c_float
                > v * height as ::core::ffi::c_float
            {
                v * height as ::core::ffi::c_float
            } else {
                (height - 1 as ::core::ffi::c_int) as ::core::ffi::c_float
            }) as ::core::ffi::c_int;
            src = heman_image_texel(img, i_0, j);
            let mut n: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while n < nbands {
                let fresh17 = src;
                src = src.offset(1);
                let fresh18 = dst;
                dst = dst.offset(1);
                *fresh18 = *fresh17;
                n += 1;
            }
            if !secondary.is_null() {
                src = heman_image_texel(secondary, x, y);
                let mut dst2: *mut ::core::ffi::c_float = heman_image_texel(
                    result2,
                    i_0,
                    j,
                );
                let mut n_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                while n_0 < (*secondary).nbands {
                    let fresh19 = src;
                    src = src.offset(1);
                    let fresh20 = dst2;
                    dst2 = dst2.offset(1);
                    *fresh20 = *fresh19;
                    n_0 += 1;
                }
            }
            x += 1;
        }
        y += 1;
    }
    open_simplex_noise_free(ctx);
    if !secondary.is_null() {
        free((*secondary).data as *mut ::core::ffi::c_void);
        (*secondary).data = (*result2).data;
        free(result2 as *mut ::core::ffi::c_void);
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn heman_ops_warp_points(
    mut img: *mut heman_image,
    mut seed: ::core::ffi::c_int,
    mut octaves: ::core::ffi::c_int,
    mut pts: *mut heman_points,
) -> *mut heman_image {
    let mut width: ::core::ffi::c_int = (*img).width;
    let mut height: ::core::ffi::c_int = (*img).height;
    let mut mapping: *mut heman_image = heman_distance_identity_cpcf(width, height);
    let mut retval: *mut heman_image = heman_ops_warp_core(img, mapping, seed, octaves);
    let mut src: *mut ::core::ffi::c_float = (*pts).data;
    let mut k: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while k < (*pts).width {
        let mut x: ::core::ffi::c_float = *src.offset(0 as ::core::ffi::c_int as isize);
        let mut y: ::core::ffi::c_float = *src.offset(1 as ::core::ffi::c_int as isize);
        let mut i: ::core::ffi::c_int = (x * (*mapping).width as ::core::ffi::c_float)
            as ::core::ffi::c_int;
        let mut j: ::core::ffi::c_int = (y * (*mapping).height as ::core::ffi::c_float)
            as ::core::ffi::c_int;
        if !(i < 0 as ::core::ffi::c_int || i >= (*mapping).width
            || j < 0 as ::core::ffi::c_int || j >= (*mapping).height)
        {
            let mut texel: *mut ::core::ffi::c_float = heman_image_texel(mapping, i, j);
            *src.offset(0 as ::core::ffi::c_int as isize) = *texel
                .offset(0 as ::core::ffi::c_int as isize)
                / (*mapping).width as ::core::ffi::c_float;
            *src.offset(1 as ::core::ffi::c_int as isize) = *texel
                .offset(1 as ::core::ffi::c_int as isize)
                / (*mapping).height as ::core::ffi::c_float;
        }
        k += 1;
        src = src.offset((*pts).nbands as isize);
    }
    heman_image_destroy(mapping);
    return retval;
}
#[no_mangle]
pub unsafe extern "C" fn heman_ops_warp(
    mut img: *mut heman_image,
    mut seed: ::core::ffi::c_int,
    mut octaves: ::core::ffi::c_int,
) -> *mut heman_image {
    return heman_ops_warp_core(
        img,
        ::core::ptr::null_mut::<heman_image>(),
        seed,
        octaves,
    );
}
#[no_mangle]
pub unsafe extern "C" fn heman_ops_extract_mask(
    mut source: *mut heman_image,
    mut color: heman_color,
    mut invert: ::core::ffi::c_int,
) -> *mut heman_image {
    let mut inv: ::core::ffi::c_float = 1.0f32 / 255.0f32;
    let mut r: ::core::ffi::c_float = (color >> 16 as ::core::ffi::c_int)
        as ::core::ffi::c_float * inv;
    let mut g: ::core::ffi::c_float = (color as ::core::ffi::c_uint
        >> 8 as ::core::ffi::c_int & 0xff as ::core::ffi::c_uint) as ::core::ffi::c_float
        * inv;
    let mut b: ::core::ffi::c_float = (color as ::core::ffi::c_uint
        & 0xff as ::core::ffi::c_uint) as ::core::ffi::c_float * inv;
    let mut height: ::core::ffi::c_int = (*source).height;
    let mut width: ::core::ffi::c_int = (*source).width;
    let mut result: *mut heman_image = heman_image_create(
        width,
        height,
        1 as ::core::ffi::c_int,
    );
    let mut y: ::core::ffi::c_int = 0;
    y = 0 as ::core::ffi::c_int;
    while y < height {
        let mut dst: *mut ::core::ffi::c_float = (*result)
            .data
            .offset((y * width) as isize);
        let mut src: *mut ::core::ffi::c_float = (*source)
            .data
            .offset((y * width * 3 as ::core::ffi::c_int) as isize);
        let mut x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while x < width {
            let mut val: ::core::ffi::c_float = (*src
                .offset(0 as ::core::ffi::c_int as isize) == r
                && *src.offset(1 as ::core::ffi::c_int as isize) == g
                && *src.offset(2 as ::core::ffi::c_int as isize) == b)
                as ::core::ffi::c_int as ::core::ffi::c_float;
            if invert == 0 {
                val = 1 as ::core::ffi::c_int as ::core::ffi::c_float - val;
            }
            let fresh21 = dst;
            dst = dst.offset(1);
            *fresh21 = val;
            x += 1;
            src = src.offset(3 as ::core::ffi::c_int as isize);
        }
        y += 1;
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn heman_ops_replace_color(
    mut source: *mut heman_image,
    mut color: heman_color,
    mut texture: *mut heman_image,
) -> *mut heman_image {
    let mut height: ::core::ffi::c_int = (*source).height;
    let mut width: ::core::ffi::c_int = (*source).width;
    let mut inv: ::core::ffi::c_float = 1.0f32 / 255.0f32;
    let mut r: ::core::ffi::c_float = (color >> 16 as ::core::ffi::c_int)
        as ::core::ffi::c_float * inv;
    let mut g: ::core::ffi::c_float = (color as ::core::ffi::c_uint
        >> 8 as ::core::ffi::c_int & 0xff as ::core::ffi::c_uint) as ::core::ffi::c_float
        * inv;
    let mut b: ::core::ffi::c_float = (color as ::core::ffi::c_uint
        & 0xff as ::core::ffi::c_uint) as ::core::ffi::c_float * inv;
    let mut result: *mut heman_image = heman_image_create(
        width,
        height,
        3 as ::core::ffi::c_int,
    );
    let mut y: ::core::ffi::c_int = 0;
    y = 0 as ::core::ffi::c_int;
    while y < height {
        let mut dst: *mut ::core::ffi::c_float = (*result)
            .data
            .offset((y * width * 3 as ::core::ffi::c_int) as isize);
        let mut src: *mut ::core::ffi::c_float = (*source)
            .data
            .offset((y * width * 3 as ::core::ffi::c_int) as isize);
        let mut tex: *mut ::core::ffi::c_float = (*texture)
            .data
            .offset((y * width * 3 as ::core::ffi::c_int) as isize);
        let mut x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while x < width {
            if *src.offset(0 as ::core::ffi::c_int as isize) == r
                && *src.offset(1 as ::core::ffi::c_int as isize) == g
                && *src.offset(2 as ::core::ffi::c_int as isize) == b
            {
                *dst.offset(0 as ::core::ffi::c_int as isize) = *tex
                    .offset(0 as ::core::ffi::c_int as isize);
                *dst.offset(1 as ::core::ffi::c_int as isize) = *tex
                    .offset(1 as ::core::ffi::c_int as isize);
                *dst.offset(2 as ::core::ffi::c_int as isize) = *tex
                    .offset(2 as ::core::ffi::c_int as isize);
            } else {
                *dst.offset(0 as ::core::ffi::c_int as isize) = *src
                    .offset(0 as ::core::ffi::c_int as isize);
                *dst.offset(1 as ::core::ffi::c_int as isize) = *src
                    .offset(1 as ::core::ffi::c_int as isize);
                *dst.offset(2 as ::core::ffi::c_int as isize) = *src
                    .offset(2 as ::core::ffi::c_int as isize);
            }
            x += 1;
            src = src.offset(3 as ::core::ffi::c_int as isize);
            dst = dst.offset(3 as ::core::ffi::c_int as isize);
            tex = tex.offset(3 as ::core::ffi::c_int as isize);
        }
        y += 1;
    }
    return result;
}
unsafe extern "C" fn _match(
    mut mask: *mut heman_image,
    mut mask_color: heman_color,
    mut invert_mask: ::core::ffi::c_int,
    mut pixel_index: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut mcolor: *mut ::core::ffi::c_float = (*mask)
        .data
        .offset((pixel_index * 3 as ::core::ffi::c_int) as isize);
    let mut r1: ::core::ffi::c_uchar = (*mcolor.offset(0 as ::core::ffi::c_int as isize)
        * 255 as ::core::ffi::c_int as ::core::ffi::c_float) as ::core::ffi::c_uchar;
    let mut g1: ::core::ffi::c_uchar = (*mcolor.offset(1 as ::core::ffi::c_int as isize)
        * 255 as ::core::ffi::c_int as ::core::ffi::c_float) as ::core::ffi::c_uchar;
    let mut b1: ::core::ffi::c_uchar = (*mcolor.offset(2 as ::core::ffi::c_int as isize)
        * 255 as ::core::ffi::c_int as ::core::ffi::c_float) as ::core::ffi::c_uchar;
    let mut r2: ::core::ffi::c_uchar = (mask_color >> 16 as ::core::ffi::c_int)
        as ::core::ffi::c_uchar;
    let mut g2: ::core::ffi::c_uchar = (mask_color as ::core::ffi::c_uint
        >> 8 as ::core::ffi::c_int & 0xff as ::core::ffi::c_uint)
        as ::core::ffi::c_uchar;
    let mut b2: ::core::ffi::c_uchar = (mask_color as ::core::ffi::c_uint
        & 0xff as ::core::ffi::c_uint) as ::core::ffi::c_uchar;
    let mut retval: ::core::ffi::c_int = (r1 as ::core::ffi::c_int
        == r2 as ::core::ffi::c_int
        && g1 as ::core::ffi::c_int == g2 as ::core::ffi::c_int
        && b1 as ::core::ffi::c_int == b2 as ::core::ffi::c_int) as ::core::ffi::c_int;
    return if invert_mask != 0 { 1 as ::core::ffi::c_int - retval } else { retval };
}
unsafe extern "C" fn qselect(
    mut v: *mut ::core::ffi::c_float,
    mut len: ::core::ffi::c_int,
    mut k: ::core::ffi::c_int,
) -> ::core::ffi::c_float {
    let mut i: ::core::ffi::c_int = 0;
    let mut st: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    st = i;
    while i < len - 1 as ::core::ffi::c_int {
        if !(*v.offset(i as isize) > *v.offset((len - 1 as ::core::ffi::c_int) as isize))
        {
            let mut _tmp: ::core::ffi::c_float = *v.offset(i as isize);
            *v.offset(i as isize) = *v.offset(st as isize);
            *v.offset(st as isize) = _tmp;
            st += 1;
        }
        i += 1;
    }
    let mut __0: ::core::ffi::c_float = *v
        .offset((len - 1 as ::core::ffi::c_int) as isize);
    *v.offset((len - 1 as ::core::ffi::c_int) as isize) = *v.offset(st as isize);
    *v.offset(st as isize) = __0;
    return if k == st {
        *v.offset(st as isize)
    } else if st > k {
        qselect(v, st, k)
    } else {
        qselect(v.offset(st as isize), len - st, k - st)
    };
}
#[no_mangle]
pub unsafe extern "C" fn heman_ops_percentiles(
    mut hmap: *mut heman_image,
    mut nsteps: ::core::ffi::c_int,
    mut mask: *mut heman_image,
    mut mask_color: heman_color,
    mut invert_mask: ::core::ffi::c_int,
    mut offset: ::core::ffi::c_float,
) -> *mut heman_image {
    let mut size: ::core::ffi::c_int = (*hmap).height * (*hmap).width;
    let mut src: *mut ::core::ffi::c_float = (*hmap).data;
    let mut minv: ::core::ffi::c_float = 1000 as ::core::ffi::c_int
        as ::core::ffi::c_float;
    let mut maxv: ::core::ffi::c_float = -(1000 as ::core::ffi::c_int)
        as ::core::ffi::c_float;
    let mut npixels: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < size {
        if mask.is_null() || _match(mask, mask_color, invert_mask, i) != 0 {
            minv = if minv > *src.offset(i as isize) {
                *src.offset(i as isize)
            } else {
                minv
            };
            maxv = if maxv > *src.offset(i as isize) {
                maxv
            } else {
                *src.offset(i as isize)
            };
            npixels += 1;
        }
        i += 1;
    }
    let mut vals: *mut ::core::ffi::c_float = malloc(
        (::core::mem::size_of::<::core::ffi::c_float>() as size_t)
            .wrapping_mul(npixels as size_t),
    ) as *mut ::core::ffi::c_float;
    npixels = 0 as ::core::ffi::c_int;
    let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_0 < size {
        if mask.is_null() || _match(mask, mask_color, invert_mask, i_0) != 0 {
            let fresh8 = npixels;
            npixels = npixels + 1;
            *vals.offset(fresh8 as isize) = *src.offset(i_0 as isize);
        }
        i_0 += 1;
    }
    let mut percentiles: *mut ::core::ffi::c_float = malloc(
        (::core::mem::size_of::<::core::ffi::c_float>() as size_t)
            .wrapping_mul(nsteps as size_t),
    ) as *mut ::core::ffi::c_float;
    let mut tier: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while tier < nsteps {
        let mut height: ::core::ffi::c_float = qselect(
            vals,
            npixels,
            tier * npixels / nsteps,
        );
        *percentiles.offset(tier as isize) = height;
        tier += 1;
    }
    free(vals as *mut ::core::ffi::c_void);
    let mut i_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_1 < size {
        let mut e: ::core::ffi::c_float = *src;
        if mask.is_null() || _match(mask, mask_color, invert_mask, i_1) != 0 {
            let mut tier_0: ::core::ffi::c_int = nsteps - 1 as ::core::ffi::c_int;
            while tier_0 >= 0 as ::core::ffi::c_int {
                if e > *percentiles.offset(tier_0 as isize) {
                    e = *percentiles.offset(tier_0 as isize);
                    break;
                } else {
                    tier_0 -= 1;
                }
            }
        }
        let fresh9 = src;
        src = src.offset(1);
        *fresh9 = e + offset;
        i_1 += 1;
    }
    free(percentiles as *mut ::core::ffi::c_void);
    return hmap;
}
#[no_mangle]
pub unsafe extern "C" fn heman_ops_stairstep(
    mut hmap: *mut heman_image,
    mut nsteps: ::core::ffi::c_int,
    mut mask: *mut heman_image,
    mut mask_color: heman_color,
    mut invert_mask: ::core::ffi::c_int,
    mut offset: ::core::ffi::c_float,
) -> *mut heman_image {
    let mut size: ::core::ffi::c_int = (*hmap).height * (*hmap).width;
    let mut src: *mut ::core::ffi::c_float = (*hmap).data;
    let mut minv: ::core::ffi::c_float = 1000 as ::core::ffi::c_int
        as ::core::ffi::c_float;
    let mut maxv: ::core::ffi::c_float = -(1000 as ::core::ffi::c_int)
        as ::core::ffi::c_float;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < size {
        if mask.is_null() || _match(mask, mask_color, invert_mask, i) != 0 {
            minv = if minv > *src.offset(i as isize) {
                *src.offset(i as isize)
            } else {
                minv
            };
            maxv = if maxv > *src.offset(i as isize) {
                maxv
            } else {
                *src.offset(i as isize)
            };
        }
        i += 1;
    }
    let mut range: ::core::ffi::c_float = maxv - minv;
    let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_0 < size {
        let mut e: ::core::ffi::c_float = *src;
        if mask.is_null() || _match(mask, mask_color, invert_mask, i_0) != 0 {
            e = e - minv;
            e /= range;
            e = (floor((e * nsteps as ::core::ffi::c_float) as ::core::ffi::c_double)
                / nsteps as ::core::ffi::c_double) as ::core::ffi::c_float;
            e = e * range + minv;
        }
        let fresh7 = src;
        src = src.offset(1);
        *fresh7 = e + offset;
        i_0 += 1;
    }
    return hmap;
}
#[no_mangle]
pub unsafe extern "C" fn heman_ops_merge_political(
    mut hmap: *mut heman_image,
    mut cmap: *mut heman_image,
    mut ocean: heman_color,
) -> *mut heman_image {
    let mut result: *mut heman_image = heman_image_create(
        (*hmap).width,
        (*hmap).height,
        4 as ::core::ffi::c_int,
    );
    let mut pheight: *mut ::core::ffi::c_float = (*hmap).data;
    let mut pcolour: *mut ::core::ffi::c_float = (*cmap).data;
    let mut pmerged: *mut ::core::ffi::c_float = (*result).data;
    let mut inv: ::core::ffi::c_float = 1.0f32 / 255.0f32;
    let mut oceanr: ::core::ffi::c_float = (ocean >> 16 as ::core::ffi::c_int)
        as ::core::ffi::c_float * inv;
    let mut oceang: ::core::ffi::c_float = (ocean as ::core::ffi::c_uint
        >> 8 as ::core::ffi::c_int & 0xff as ::core::ffi::c_uint) as ::core::ffi::c_float
        * inv;
    let mut oceanb: ::core::ffi::c_float = (ocean as ::core::ffi::c_uint
        & 0xff as ::core::ffi::c_uint) as ::core::ffi::c_float * inv;
    let mut size: ::core::ffi::c_int = (*hmap).height * (*hmap).width;
    let mut minh: ::core::ffi::c_float = 1000 as ::core::ffi::c_int
        as ::core::ffi::c_float;
    let mut maxh: ::core::ffi::c_float = -(1000 as ::core::ffi::c_int)
        as ::core::ffi::c_float;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < size {
        minh = if minh > *pheight.offset(i as isize) {
            *pheight.offset(i as isize)
        } else {
            minh
        };
        maxh = if maxh > *pheight.offset(i as isize) {
            *pheight.offset(i as isize)
        } else {
            maxh
        };
        i += 1;
    }
    let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_0 < size {
        let fresh22 = pheight;
        pheight = pheight.offset(1);
        let mut h: ::core::ffi::c_float = *fresh22;
        if h < 0 as ::core::ffi::c_int as ::core::ffi::c_float {
            let fresh23 = pmerged;
            pmerged = pmerged.offset(1);
            *fresh23 = oceanr;
            let fresh24 = pmerged;
            pmerged = pmerged.offset(1);
            *fresh24 = oceang;
            let fresh25 = pmerged;
            pmerged = pmerged.offset(1);
            *fresh25 = oceanb;
            pcolour = pcolour.offset(3 as ::core::ffi::c_int as isize);
        } else {
            let fresh26 = pcolour;
            pcolour = pcolour.offset(1);
            let fresh27 = pmerged;
            pmerged = pmerged.offset(1);
            *fresh27 = *fresh26;
            let fresh28 = pcolour;
            pcolour = pcolour.offset(1);
            let fresh29 = pmerged;
            pmerged = pmerged.offset(1);
            *fresh29 = *fresh28;
            let fresh30 = pcolour;
            pcolour = pcolour.offset(1);
            let fresh31 = pmerged;
            pmerged = pmerged.offset(1);
            *fresh31 = *fresh30;
        }
        let fresh32 = pmerged;
        pmerged = pmerged.offset(1);
        *fresh32 = (h - minh) / (maxh - minh);
        i_0 += 1;
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn heman_ops_emboss(
    mut img: *mut heman_image,
    mut mode: ::core::ffi::c_int,
) -> *mut heman_image {
    let mut seed: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut octaves: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
    let mut ctx: *mut osn_context = ::core::ptr::null_mut::<osn_context>();
    open_simplex_noise(seed as int64_t, &raw mut ctx);
    let mut width: ::core::ffi::c_int = (*img).width;
    let mut height: ::core::ffi::c_int = (*img).height;
    let mut result: *mut heman_image = heman_image_create(
        width,
        height,
        1 as ::core::ffi::c_int,
    );
    let mut invw: ::core::ffi::c_float = (1.0f64 / width as ::core::ffi::c_double)
        as ::core::ffi::c_float;
    let mut invh: ::core::ffi::c_float = (1.0f64 / height as ::core::ffi::c_double)
        as ::core::ffi::c_float;
    let mut inv: ::core::ffi::c_float = if invw > invh { invh } else { invw };
    let mut gain: ::core::ffi::c_float = 0.6f32;
    let mut lacunarity: ::core::ffi::c_float = 2.0f32;
    let mut land_amplitude: ::core::ffi::c_float = 0.0005f32;
    let mut land_frequency: ::core::ffi::c_float = 256.0f32;
    let mut ocean_amplitude: ::core::ffi::c_float = 0.5f32;
    let mut ocean_frequency: ::core::ffi::c_float = 1.0f32;
    let mut y: ::core::ffi::c_int = 0;
    y = 0 as ::core::ffi::c_int;
    while y < height {
        let mut dst: *mut ::core::ffi::c_float = (*result)
            .data
            .offset((y * width) as isize);
        let mut x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while x < width {
            let mut z: ::core::ffi::c_float = *heman_image_texel(img, x, y);
            if z > 0 as ::core::ffi::c_int as ::core::ffi::c_float
                && mode == 1 as ::core::ffi::c_int
            {
                let mut s: ::core::ffi::c_float = x as ::core::ffi::c_float * inv;
                let mut t: ::core::ffi::c_float = y as ::core::ffi::c_float * inv;
                let mut a: ::core::ffi::c_float = land_amplitude;
                let mut f: ::core::ffi::c_float = land_frequency;
                let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                while i < octaves {
                    z = (z as ::core::ffi::c_double
                        + a as ::core::ffi::c_double
                            * open_simplex_noise2(
                                ctx,
                                (s * f) as ::core::ffi::c_double,
                                (t * f) as ::core::ffi::c_double,
                            )) as ::core::ffi::c_float;
                    a *= gain;
                    f *= lacunarity;
                    i += 1;
                }
            } else if z <= 0 as ::core::ffi::c_int as ::core::ffi::c_float
                && mode == -(1 as ::core::ffi::c_int)
            {
                z = (if z as ::core::ffi::c_double > -0.1f64 {
                    z as ::core::ffi::c_double
                } else {
                    -0.1f64
                }) as ::core::ffi::c_float;
                let mut soften: ::core::ffi::c_float = fabsf(z);
                let mut s_0: ::core::ffi::c_float = x as ::core::ffi::c_float * inv;
                let mut t_0: ::core::ffi::c_float = y as ::core::ffi::c_float * inv;
                let mut a_0: ::core::ffi::c_float = ocean_amplitude;
                let mut f_0: ::core::ffi::c_float = ocean_frequency;
                let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                while i_0 < octaves {
                    z = (z as ::core::ffi::c_double
                        + soften as ::core::ffi::c_double
                            * (a_0 as ::core::ffi::c_double
                                * open_simplex_noise2(
                                    ctx,
                                    (s_0 * f_0) as ::core::ffi::c_double,
                                    (t_0 * f_0) as ::core::ffi::c_double,
                                ))) as ::core::ffi::c_float;
                    a_0 *= gain;
                    f_0 *= lacunarity;
                    i_0 += 1;
                }
            }
            let fresh33 = dst;
            dst = dst.offset(1);
            *fresh33 = z;
            x += 1;
        }
        y += 1;
    }
    open_simplex_noise_free(ctx);
    return result;
}
