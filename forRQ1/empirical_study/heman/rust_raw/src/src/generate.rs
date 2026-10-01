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
    fn heman_image_sample(
        _: *mut heman_image,
        u: ::core::ffi::c_float,
        v: ::core::ffi::c_float,
        result: *mut ::core::ffi::c_float,
    );
    fn heman_image_clear(_: *mut heman_image, value: ::core::ffi::c_float);
    fn heman_image_destroy(_: *mut heman_image);
    fn heman_color_from_cpcf(
        cfield: *mut heman_image,
        texture: *mut heman_image,
    ) -> *mut heman_image;
    fn heman_distance_create_sdf(monochrome: *mut heman_image) -> *mut heman_image;
    fn heman_distance_create_cpcf(seed: *mut heman_image) -> *mut heman_image;
    fn heman_ops_warp(
        src: *mut heman_image,
        seed: ::core::ffi::c_int,
        octaves: ::core::ffi::c_int,
    ) -> *mut heman_image;
    fn heman_ops_extract_mask(
        src: *mut heman_image,
        color: heman_color,
        invert: ::core::ffi::c_int,
    ) -> *mut heman_image;
    fn heman_draw_colored_points(
        target: *mut heman_image,
        coords: *mut heman_points,
        colors: *const heman_color,
    );
    fn heman_draw_contour_from_points(
        target: *mut heman_image,
        coords: *mut heman_points,
        color: heman_color,
        mind: ::core::ffi::c_float,
        maxd: ::core::ffi::c_float,
        filterd: ::core::ffi::c_int,
    );
    fn open_simplex_noise(seed: int64_t, ctx: *mut *mut osn_context) -> ::core::ffi::c_int;
    fn open_simplex_noise_free(ctx: *mut osn_context);
    fn open_simplex_noise2(
        ctx: *mut osn_context,
        x: ::core::ffi::c_double,
        y: ::core::ffi::c_double,
    ) -> ::core::ffi::c_double;
    fn open_simplex_noise3(
        ctx: *mut osn_context,
        x: ::core::ffi::c_double,
        y: ::core::ffi::c_double,
        z: ::core::ffi::c_double,
    ) -> ::core::ffi::c_double;
    fn cos(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn sin(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn sqrt(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn fabs(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
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
pub type int64_t = __int64_t;
pub type __int64_t = i64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kmVec3 {
    pub x: ::core::ffi::c_float,
    pub y: ::core::ffi::c_float,
    pub z: ::core::ffi::c_float,
}
pub type size_t = usize;
pub const PI: ::core::ffi::c_double = 3.1415926535f64;
static mut SEALEVEL: ::core::ffi::c_float = 0.5f32;
static mut DEFAULT_STRENGTH: ::core::ffi::c_float = 0.6f32;
#[no_mangle]
pub unsafe extern "C" fn heman_internal_generate_island_noise(
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
    mut seed: ::core::ffi::c_int,
) -> *mut heman_image {
    let mut ctx: *mut osn_context = ::core::ptr::null_mut::<osn_context>();
    open_simplex_noise(seed as int64_t, &raw mut ctx);
    let mut img: *mut heman_image = heman_image_create(width, height, 3 as ::core::ffi::c_int);
    let mut data: *mut ::core::ffi::c_float = (*img).data;
    let mut invh: ::core::ffi::c_float =
        1.0f32 / (if width > height { width } else { height }) as ::core::ffi::c_float;
    let mut invw: ::core::ffi::c_float =
        1.0f32 / (if width > height { width } else { height }) as ::core::ffi::c_float;
    let mut freqs: [::core::ffi::c_float; 5] = [
        4.0f64 as ::core::ffi::c_float,
        16.0f64 as ::core::ffi::c_float,
        32.0f64 as ::core::ffi::c_float,
        64.0f64 as ::core::ffi::c_float,
        128.0f64 as ::core::ffi::c_float,
    ];
    let mut ampls: [::core::ffi::c_float; 5] = [
        0.2f64 as ::core::ffi::c_float,
        0.1f64 as ::core::ffi::c_float,
        0.05f64 as ::core::ffi::c_float,
        0.025f64 as ::core::ffi::c_float,
        0.0125f64 as ::core::ffi::c_float,
    ];
    let mut y: ::core::ffi::c_int = 0;
    y = 0 as ::core::ffi::c_int;
    while y < height {
        let mut v: ::core::ffi::c_float = y as ::core::ffi::c_float * invh;
        let mut dst: *mut ::core::ffi::c_float =
            data.offset((y * width * 3 as ::core::ffi::c_int) as isize);
        let mut x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while x < width {
            let mut u: ::core::ffi::c_float = x as ::core::ffi::c_float * invw;
            let fresh2 = dst;
            dst = dst.offset(1);
            *fresh2 = (ampls[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_double
                * open_simplex_noise2(
                    ctx,
                    (u * freqs[0 as ::core::ffi::c_int as usize]) as ::core::ffi::c_double,
                    (v * freqs[0 as ::core::ffi::c_int as usize]) as ::core::ffi::c_double,
                )
                + ampls[1 as ::core::ffi::c_int as usize] as ::core::ffi::c_double
                    * open_simplex_noise2(
                        ctx,
                        (u * freqs[1 as ::core::ffi::c_int as usize]) as ::core::ffi::c_double,
                        (v * freqs[1 as ::core::ffi::c_int as usize]) as ::core::ffi::c_double,
                    )
                + ampls[2 as ::core::ffi::c_int as usize] as ::core::ffi::c_double
                    * open_simplex_noise2(
                        ctx,
                        (u * freqs[2 as ::core::ffi::c_int as usize]) as ::core::ffi::c_double,
                        (v * freqs[2 as ::core::ffi::c_int as usize]) as ::core::ffi::c_double,
                    )) as ::core::ffi::c_float;
            let fresh3 = dst;
            dst = dst.offset(1);
            *fresh3 = (ampls[3 as ::core::ffi::c_int as usize] as ::core::ffi::c_double
                * open_simplex_noise2(
                    ctx,
                    (u * freqs[3 as ::core::ffi::c_int as usize]) as ::core::ffi::c_double,
                    (v * freqs[3 as ::core::ffi::c_int as usize]) as ::core::ffi::c_double,
                )
                + ampls[4 as ::core::ffi::c_int as usize] as ::core::ffi::c_double
                    * open_simplex_noise2(
                        ctx,
                        (u * freqs[4 as ::core::ffi::c_int as usize]) as ::core::ffi::c_double,
                        (v * freqs[4 as ::core::ffi::c_int as usize]) as ::core::ffi::c_double,
                    )) as ::core::ffi::c_float;
            u = (u as ::core::ffi::c_double + 0.5f64) as ::core::ffi::c_float;
            let fresh4 = dst;
            dst = dst.offset(1);
            *fresh4 = (ampls[3 as ::core::ffi::c_int as usize] as ::core::ffi::c_double
                * open_simplex_noise2(
                    ctx,
                    (u * freqs[3 as ::core::ffi::c_int as usize]) as ::core::ffi::c_double,
                    (v * freqs[3 as ::core::ffi::c_int as usize]) as ::core::ffi::c_double,
                )
                + ampls[4 as ::core::ffi::c_int as usize] as ::core::ffi::c_double
                    * open_simplex_noise2(
                        ctx,
                        (u * freqs[4 as ::core::ffi::c_int as usize]) as ::core::ffi::c_double,
                        (v * freqs[4 as ::core::ffi::c_int as usize]) as ::core::ffi::c_double,
                    )) as ::core::ffi::c_float;
            x += 1;
        }
        y += 1;
    }
    open_simplex_noise_free(ctx);
    return img;
}
#[no_mangle]
pub unsafe extern "C" fn heman_internal_generate_rock_noise(
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
    mut seed: ::core::ffi::c_int,
) -> *mut heman_image {
    let mut ctx: *mut osn_context = ::core::ptr::null_mut::<osn_context>();
    open_simplex_noise(seed as int64_t, &raw mut ctx);
    let mut img: *mut heman_image = heman_image_create(width, height, 1 as ::core::ffi::c_int);
    let mut data: *mut ::core::ffi::c_float = (*img).data;
    let mut invh: ::core::ffi::c_float =
        1.0f32 / (if width > height { width } else { height }) as ::core::ffi::c_float;
    let mut invw: ::core::ffi::c_float =
        1.0f32 / (if width > height { width } else { height }) as ::core::ffi::c_float;
    let mut freqs: [::core::ffi::c_float; 3] = [
        2.0f64 as ::core::ffi::c_float,
        4.0f64 as ::core::ffi::c_float,
        16.0f64 as ::core::ffi::c_float,
    ];
    let mut ampls: [::core::ffi::c_float; 3] = [
        0.2f64 as ::core::ffi::c_float,
        0.05f64 as ::core::ffi::c_float,
        0.01f64 as ::core::ffi::c_float,
    ];
    let mut y: ::core::ffi::c_int = 0;
    y = 0 as ::core::ffi::c_int;
    while y < height {
        let mut v: ::core::ffi::c_float = y as ::core::ffi::c_float * invh;
        let mut dst: *mut ::core::ffi::c_float = data.offset((y * width) as isize);
        let mut x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while x < width {
            let mut u: ::core::ffi::c_float = x as ::core::ffi::c_float * invw;
            let fresh7 = dst;
            dst = dst.offset(1);
            *fresh7 = (ampls[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_double
                * open_simplex_noise2(
                    ctx,
                    (u * freqs[0 as ::core::ffi::c_int as usize]) as ::core::ffi::c_double,
                    (v * freqs[0 as ::core::ffi::c_int as usize]) as ::core::ffi::c_double,
                )
                + ampls[1 as ::core::ffi::c_int as usize] as ::core::ffi::c_double
                    * open_simplex_noise2(
                        ctx,
                        (u * freqs[1 as ::core::ffi::c_int as usize]) as ::core::ffi::c_double,
                        (v * freqs[1 as ::core::ffi::c_int as usize]) as ::core::ffi::c_double,
                    )
                + ampls[2 as ::core::ffi::c_int as usize] as ::core::ffi::c_double
                    * open_simplex_noise2(
                        ctx,
                        (u * freqs[2 as ::core::ffi::c_int as usize]) as ::core::ffi::c_double,
                        (v * freqs[2 as ::core::ffi::c_int as usize]) as ::core::ffi::c_double,
                    )) as ::core::ffi::c_float;
            x += 1;
        }
        y += 1;
    }
    open_simplex_noise_free(ctx);
    return img;
}
#[no_mangle]
pub unsafe extern "C" fn heman_generate_island_heightmap(
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
    mut seed: ::core::ffi::c_int,
) -> *mut heman_image {
    let mut noisetex: *mut heman_image = heman_internal_generate_island_noise(width, height, seed);
    let mut coastmask: *mut heman_image =
        heman_image_create(width, height, 1 as ::core::ffi::c_int);
    let mut data: *mut ::core::ffi::c_float = (*coastmask).data;
    let mut invh: ::core::ffi::c_float = 1.0f32 / height as ::core::ffi::c_float;
    let mut invw: ::core::ffi::c_float = 1.0f32 / width as ::core::ffi::c_float;
    let mut hh: ::core::ffi::c_int = height / 2 as ::core::ffi::c_int;
    let mut hw: ::core::ffi::c_int = width / 2 as ::core::ffi::c_int;
    let mut y: ::core::ffi::c_int = 0;
    y = 0 as ::core::ffi::c_int;
    while y < height {
        let mut vv: ::core::ffi::c_float = (y - hh) as ::core::ffi::c_float * invh;
        let mut dst: *mut ::core::ffi::c_float = data.offset((y * width) as isize);
        let mut x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while x < width {
            let mut n: [::core::ffi::c_float; 3] = [0.; 3];
            let mut v: ::core::ffi::c_float = y as ::core::ffi::c_float * invh;
            let mut u: ::core::ffi::c_float = x as ::core::ffi::c_float * invw;
            heman_image_sample(noisetex, u, v, &raw mut n as *mut ::core::ffi::c_float);
            u = (x - hw) as ::core::ffi::c_float * invw;
            v = vv;
            u += n[1 as ::core::ffi::c_int as usize];
            v += n[2 as ::core::ffi::c_int as usize];
            let mut m: ::core::ffi::c_float =
                (0.707f64 - sqrt((u * u + v * v) as ::core::ffi::c_double)) as ::core::ffi::c_float;
            m += n[0 as ::core::ffi::c_int as usize];
            let fresh0 = dst;
            dst = dst.offset(1);
            *fresh0 = (if m < SEALEVEL {
                0 as ::core::ffi::c_int
            } else {
                1 as ::core::ffi::c_int
            }) as ::core::ffi::c_float;
            x += 1;
        }
        y += 1;
    }
    let mut heightmap: *mut heman_image = heman_distance_create_sdf(coastmask);
    heman_image_destroy(coastmask);
    let mut result: *mut heman_image = heman_image_create(width, height, 1 as ::core::ffi::c_int);
    data = (*result).data;
    y = 0 as ::core::ffi::c_int;
    while y < height {
        let mut dst_0: *mut ::core::ffi::c_float = data.offset((y * width) as isize);
        let mut x_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while x_0 < width {
            let mut n_0: [::core::ffi::c_float; 3] = [0.; 3];
            let mut u_0: ::core::ffi::c_float = x_0 as ::core::ffi::c_float * invw;
            let mut v_0: ::core::ffi::c_float = y as ::core::ffi::c_float * invh;
            heman_image_sample(
                noisetex,
                u_0,
                v_0,
                &raw mut n_0 as *mut ::core::ffi::c_float,
            );
            let mut z: ::core::ffi::c_float = 0.;
            heman_image_sample(heightmap, u_0, v_0, &raw mut z);
            if z as ::core::ffi::c_double > 0.0f64 {
                let mut influence: ::core::ffi::c_float = z;
                u_0 += influence * n_0[1 as ::core::ffi::c_int as usize];
                v_0 += influence * n_0[2 as ::core::ffi::c_int as usize];
                heman_image_sample(heightmap, u_0, v_0, &raw mut z);
                z += 6 as ::core::ffi::c_int as ::core::ffi::c_float
                    * influence
                    * n_0[0 as ::core::ffi::c_int as usize];
            }
            let fresh1 = dst_0;
            dst_0 = dst_0.offset(1);
            *fresh1 = z;
            x_0 += 1;
        }
        y += 1;
    }
    heman_image_destroy(noisetex);
    heman_image_destroy(heightmap);
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn heman_generate_rock_heightmap(
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
    mut seed: ::core::ffi::c_int,
) -> *mut heman_image {
    let mut noisetex: *mut heman_image = heman_internal_generate_rock_noise(width, height, seed);
    let mut heightmap: *mut heman_image =
        heman_image_create(width, height, 1 as ::core::ffi::c_int);
    let mut data: *mut ::core::ffi::c_float = (*heightmap).data;
    let mut invh: ::core::ffi::c_float = 1.0f32 / height as ::core::ffi::c_float;
    let mut invw: ::core::ffi::c_float = 1.0f32 / width as ::core::ffi::c_float;
    let mut hh: ::core::ffi::c_int = height / 2 as ::core::ffi::c_int;
    let mut hw: ::core::ffi::c_int = width / 2 as ::core::ffi::c_int;
    let mut y: ::core::ffi::c_int = 0;
    y = 0 as ::core::ffi::c_int;
    while y < height {
        let mut vv: ::core::ffi::c_float = (y - hh) as ::core::ffi::c_float * invh;
        let mut dst: *mut ::core::ffi::c_float = data.offset((y * width) as isize);
        let mut x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while x < width {
            let mut v: ::core::ffi::c_float = y as ::core::ffi::c_float * invh;
            let mut u: ::core::ffi::c_float = x as ::core::ffi::c_float * invw;
            let mut n: ::core::ffi::c_float = 0.;
            heman_image_sample(noisetex, u, v, &raw mut n);
            u = (x - hw) as ::core::ffi::c_float * invw;
            v = vv;
            let mut r: ::core::ffi::c_float =
                (0.3f64 + n as ::core::ffi::c_double) as ::core::ffi::c_float;
            if u * u + v * v > r * r {
                let fresh5 = dst;
                dst = dst.offset(1);
                *fresh5 = 0 as ::core::ffi::c_int as ::core::ffi::c_float;
            } else {
                let mut z: ::core::ffi::c_float =
                    sqrt((r * r - u * u - v * v) as ::core::ffi::c_double) as ::core::ffi::c_float;
                let fresh6 = dst;
                dst = dst.offset(1);
                *fresh6 = z;
            }
            x += 1;
        }
        y += 1;
    }
    heman_image_destroy(noisetex);
    return heightmap;
}
#[no_mangle]
pub unsafe extern "C" fn heman_generate_simplex_fbm(
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
    mut frequency: ::core::ffi::c_float,
    mut amplitude: ::core::ffi::c_float,
    mut octaves: ::core::ffi::c_int,
    mut lacunarity: ::core::ffi::c_float,
    mut gain: ::core::ffi::c_float,
    mut seed: ::core::ffi::c_int,
) -> *mut heman_image {
    let mut ctx: *mut osn_context = ::core::ptr::null_mut::<osn_context>();
    open_simplex_noise(seed as int64_t, &raw mut ctx);
    let mut img: *mut heman_image = heman_image_create(width, height, 1 as ::core::ffi::c_int);
    let mut data: *mut ::core::ffi::c_float = (*img).data;
    let mut invh: ::core::ffi::c_float = 1.0f32 / height as ::core::ffi::c_float;
    let mut invw: ::core::ffi::c_float = 1.0f32 / width as ::core::ffi::c_float;
    let mut ampl: ::core::ffi::c_float = amplitude;
    let mut freq: ::core::ffi::c_float = frequency;
    memset(
        data as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        (::core::mem::size_of::<::core::ffi::c_float>() as size_t)
            .wrapping_mul(width as size_t)
            .wrapping_mul(height as size_t),
    );
    loop {
        let fresh16 = octaves;
        octaves = octaves - 1;
        if !(fresh16 != 0) {
            break;
        }
        let mut y: ::core::ffi::c_int = 0;
        y = 0 as ::core::ffi::c_int;
        while y < height {
            let mut v: ::core::ffi::c_float = y as ::core::ffi::c_float * invh;
            let mut dst: *mut ::core::ffi::c_float = data.offset((y * width) as isize);
            let mut x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while x < width {
                let mut u: ::core::ffi::c_float = x as ::core::ffi::c_float * invw;
                let fresh17 = dst;
                dst = dst.offset(1);
                *fresh17 = (*fresh17 as ::core::ffi::c_double
                    + ampl as ::core::ffi::c_double
                        * open_simplex_noise2(
                            ctx,
                            (u * freq) as ::core::ffi::c_double,
                            (v * freq) as ::core::ffi::c_double,
                        )) as ::core::ffi::c_float;
                x += 1;
            }
            y += 1;
        }
        ampl *= gain;
        freq *= lacunarity;
    }
    open_simplex_noise_free(ctx);
    return img;
}
unsafe extern "C" fn sphere(
    mut u: ::core::ffi::c_float,
    mut v: ::core::ffi::c_float,
    mut r: ::core::ffi::c_float,
    mut dst: *mut kmVec3,
) {
    (*dst).x = (r as ::core::ffi::c_double
        * sin(v as ::core::ffi::c_double)
        * cos(u as ::core::ffi::c_double)) as ::core::ffi::c_float;
    (*dst).y =
        (r as ::core::ffi::c_double * cos(v as ::core::ffi::c_double)) as ::core::ffi::c_float;
    (*dst).z = (r as ::core::ffi::c_double
        * -sin(v as ::core::ffi::c_double)
        * sin(u as ::core::ffi::c_double)) as ::core::ffi::c_float;
}
#[no_mangle]
pub unsafe extern "C" fn heman_generate_planet_heightmap(
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
    mut seed: ::core::ffi::c_int,
) -> *mut heman_image {
    let mut ctx: *mut osn_context = ::core::ptr::null_mut::<osn_context>();
    open_simplex_noise(seed as int64_t, &raw mut ctx);
    let mut result: *mut heman_image = heman_image_create(width, height, 1 as ::core::ffi::c_int);
    let mut scalex: ::core::ffi::c_float =
        (2.0f64 * PI / width as ::core::ffi::c_double) as ::core::ffi::c_float;
    let mut scaley: ::core::ffi::c_float =
        (PI / height as ::core::ffi::c_double) as ::core::ffi::c_float;
    let mut invh: ::core::ffi::c_float = 1.0f32 / height as ::core::ffi::c_float;
    let mut y: ::core::ffi::c_int = 0;
    y = 0 as ::core::ffi::c_int;
    while y < height {
        let mut dst: *mut ::core::ffi::c_float = (*result).data.offset((y * width) as isize);
        let mut p: kmVec3 = kmVec3 {
            x: 0.,
            y: 0.,
            z: 0.,
        };
        let mut v: ::core::ffi::c_float = y as ::core::ffi::c_float * invh;
        let mut s: ::core::ffi::c_float = 0.95f32;
        let mut antarctic_influence: ::core::ffi::c_float =
            (if (10 as ::core::ffi::c_int as ::core::ffi::c_float * (v - s) / s)
                as ::core::ffi::c_double
                > -0.5f64
            {
                (10 as ::core::ffi::c_int as ::core::ffi::c_float * (v - s) / s)
                    as ::core::ffi::c_double
            } else {
                -0.5f64
            }) as ::core::ffi::c_float;
        v = fabs(v as ::core::ffi::c_double - 0.5f64) as ::core::ffi::c_float;
        v = (1.5f64 * (0.5f64 - v as ::core::ffi::c_double)) as ::core::ffi::c_float;
        let mut equatorial_influence: ::core::ffi::c_float = v * v;
        v = y as ::core::ffi::c_float * scaley;
        let mut x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while x < width {
            let mut u: ::core::ffi::c_float = x as ::core::ffi::c_float * scalex;
            let mut freq: ::core::ffi::c_float = 1 as ::core::ffi::c_int as ::core::ffi::c_float;
            let mut amp: ::core::ffi::c_float = 1 as ::core::ffi::c_int as ::core::ffi::c_float;
            let mut h: ::core::ffi::c_float = antarctic_influence + equatorial_influence;
            let mut oct: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while oct < 6 as ::core::ffi::c_int {
                sphere(u, v, freq, &raw mut p);
                h = (h as ::core::ffi::c_double
                    + amp as ::core::ffi::c_double
                        * open_simplex_noise3(
                            ctx,
                            p.x as ::core::ffi::c_double,
                            p.y as ::core::ffi::c_double,
                            p.z as ::core::ffi::c_double,
                        )) as ::core::ffi::c_float;
                amp = (amp as ::core::ffi::c_double * 0.5f64) as ::core::ffi::c_float;
                freq *= 2 as ::core::ffi::c_int as ::core::ffi::c_float;
                oct += 1;
            }
            let fresh8 = dst;
            dst = dst.offset(1);
            *fresh8 = h;
            x += 1;
        }
        y += 1;
    }
    open_simplex_noise_free(ctx);
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn heman_internal_draw_seeds(
    mut target: *mut heman_image,
    mut pts: *mut heman_points,
    mut filterd: ::core::ffi::c_int,
) {
    let mut radius: ::core::ffi::c_int = (*target).width / filterd;
    let mut fwidth: ::core::ffi::c_int = radius * 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int;
    let mut src: *mut ::core::ffi::c_float = (*pts).data;
    let mut w: ::core::ffi::c_int = (*target).width;
    let mut h: ::core::ffi::c_int = (*target).height;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*pts).width {
        let fresh11 = src;
        src = src.offset(1);
        let mut x: ::core::ffi::c_float = *fresh11;
        let fresh12 = src;
        src = src.offset(1);
        let mut y: ::core::ffi::c_float = *fresh12;
        let mut strength: ::core::ffi::c_float = DEFAULT_STRENGTH;
        if (*pts).nbands == 3 as ::core::ffi::c_int {
            let fresh13 = src;
            src = src.offset(1);
            strength = *fresh13;
        }
        strength = (SEALEVEL as ::core::ffi::c_double + strength as ::core::ffi::c_double * 0.1f64)
            as ::core::ffi::c_float;
        let mut ix: ::core::ffi::c_int = (x * w as ::core::ffi::c_float) as ::core::ffi::c_int;
        let mut iy: ::core::ffi::c_int = (y * h as ::core::ffi::c_float) as ::core::ffi::c_int;
        let mut ii: ::core::ffi::c_int = ix - radius;
        let mut jj: ::core::ffi::c_int = iy - radius;
        let mut kj: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while kj < fwidth {
            let mut ki: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while ki < fwidth {
                let mut i_0: ::core::ffi::c_int = ii + ki;
                let mut j: ::core::ffi::c_int = jj + kj;
                if !(i_0 < 0 as ::core::ffi::c_int
                    || i_0 >= w
                    || j < 0 as ::core::ffi::c_int
                    || j >= h)
                {
                    let mut texel: *mut ::core::ffi::c_float = heman_image_texel(target, i_0, j);
                    let mut d2: ::core::ffi::c_int = (i_0 - ix) * (i_0 - ix) + (j - iy) * (j - iy);
                    let mut dist: ::core::ffi::c_float = (1 as ::core::ffi::c_int
                        as ::core::ffi::c_double
                        - sqrt(d2 as ::core::ffi::c_double) / radius as ::core::ffi::c_double)
                        as ::core::ffi::c_float;
                    *texel = if *texel > strength * dist {
                        *texel
                    } else {
                        strength * dist
                    };
                }
                ki += 1;
            }
            kj += 1;
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn heman_generate_archipelago_heightmap(
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
    mut points: *mut heman_points,
    mut noiseamt: ::core::ffi::c_float,
    mut seed: ::core::ffi::c_int,
) -> *mut heman_image {
    let mut noisetex: *mut heman_image = heman_internal_generate_island_noise(width, height, seed);
    let mut coastmask: *mut heman_image =
        heman_image_create(width, height, 1 as ::core::ffi::c_int);
    heman_image_clear(coastmask, 0 as ::core::ffi::c_int as ::core::ffi::c_float);
    heman_internal_draw_seeds(coastmask, points, 1 as ::core::ffi::c_int);
    let mut data: *mut ::core::ffi::c_float = (*coastmask).data;
    let mut invh: ::core::ffi::c_float = 1.0f32 / height as ::core::ffi::c_float;
    let mut invw: ::core::ffi::c_float = 1.0f32 / width as ::core::ffi::c_float;
    let mut hh: ::core::ffi::c_int = height / 2 as ::core::ffi::c_int;
    let mut hw: ::core::ffi::c_int = width / 2 as ::core::ffi::c_int;
    let mut y: ::core::ffi::c_int = 0;
    y = 0 as ::core::ffi::c_int;
    while y < height {
        let mut vv: ::core::ffi::c_float = (y - hh) as ::core::ffi::c_float * invh;
        let mut dst: *mut ::core::ffi::c_float = data.offset((y * width) as isize);
        let mut x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while x < width {
            let mut n: [::core::ffi::c_float; 3] =
                [0 as ::core::ffi::c_int as ::core::ffi::c_float, 0., 0.];
            let mut v: ::core::ffi::c_float = y as ::core::ffi::c_float * invh;
            let mut u: ::core::ffi::c_float = x as ::core::ffi::c_float * invw;
            heman_image_sample(noisetex, u, v, &raw mut n as *mut ::core::ffi::c_float);
            u = (x - hw) as ::core::ffi::c_float * invw;
            v = vv;
            u += noiseamt * n[1 as ::core::ffi::c_int as usize];
            v += noiseamt * n[2 as ::core::ffi::c_int as usize];
            let mut m: ::core::ffi::c_float = *dst;
            m += noiseamt * n[0 as ::core::ffi::c_int as usize];
            let fresh9 = dst;
            dst = dst.offset(1);
            *fresh9 = (if m < SEALEVEL {
                0 as ::core::ffi::c_int
            } else {
                1 as ::core::ffi::c_int
            }) as ::core::ffi::c_float;
            x += 1;
        }
        y += 1;
    }
    let mut heightmap: *mut heman_image = heman_distance_create_sdf(coastmask);
    heman_image_destroy(coastmask);
    let mut result: *mut heman_image = heman_image_create(width, height, 1 as ::core::ffi::c_int);
    data = (*result).data;
    y = 0 as ::core::ffi::c_int;
    while y < height {
        let mut dst_0: *mut ::core::ffi::c_float = data.offset((y * width) as isize);
        let mut x_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while x_0 < width {
            let mut n_0: [::core::ffi::c_float; 3] = [0.; 3];
            let mut u_0: ::core::ffi::c_float = x_0 as ::core::ffi::c_float * invw;
            let mut v_0: ::core::ffi::c_float = y as ::core::ffi::c_float * invh;
            heman_image_sample(
                noisetex,
                u_0,
                v_0,
                &raw mut n_0 as *mut ::core::ffi::c_float,
            );
            let mut z: ::core::ffi::c_float = 0.;
            heman_image_sample(heightmap, u_0, v_0, &raw mut z);
            if z as ::core::ffi::c_double > 0.0f64 {
                let mut influence: ::core::ffi::c_float = z;
                u_0 += influence * n_0[1 as ::core::ffi::c_int as usize];
                v_0 += influence * n_0[2 as ::core::ffi::c_int as usize];
                heman_image_sample(heightmap, u_0, v_0, &raw mut z);
                z += 6 as ::core::ffi::c_int as ::core::ffi::c_float
                    * influence
                    * n_0[0 as ::core::ffi::c_int as usize];
            }
            let fresh10 = dst_0;
            dst_0 = dst_0.offset(1);
            *fresh10 = z;
            x_0 += 1;
        }
        y += 1;
    }
    heman_image_destroy(noisetex);
    heman_image_destroy(heightmap);
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn heman_generate_archipelago_political_1(
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
    mut points: *mut heman_points,
    mut colors: *const heman_color,
    mut ocean: heman_color,
    mut seed: ::core::ffi::c_int,
) -> *mut heman_image {
    let mut contour: *mut heman_image = heman_image_create(width, height, 3 as ::core::ffi::c_int);
    heman_image_clear(contour, 0 as ::core::ffi::c_int as ::core::ffi::c_float);
    heman_draw_contour_from_points(
        contour,
        points,
        ocean,
        0.40f32,
        0.41f32,
        1 as ::core::ffi::c_int,
    );
    heman_draw_colored_points(contour, points, colors);
    let mut cf: *mut heman_image = heman_distance_create_cpcf(contour);
    let mut warped_cpcf: *mut heman_image = heman_ops_warp(cf, seed, 4 as ::core::ffi::c_int);
    let mut political: *mut heman_image = heman_color_from_cpcf(warped_cpcf, contour);
    heman_image_destroy(warped_cpcf);
    heman_image_destroy(cf);
    heman_image_destroy(contour);
    return political;
}
#[no_mangle]
pub unsafe extern "C" fn heman_generate_archipelago_political_2(
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
    mut ocean: heman_color,
    mut seed: ::core::ffi::c_int,
    mut political: *mut heman_image,
    mut invert: ::core::ffi::c_int,
) -> *mut heman_image {
    let mut coastmask: *mut heman_image = heman_ops_extract_mask(political, ocean, invert);
    let mut sdf: *mut heman_image = heman_distance_create_sdf(coastmask);
    heman_image_destroy(coastmask);
    let mut elevation: *mut heman_image =
        heman_image_create(width, height, 1 as ::core::ffi::c_int);
    let mut noisetex: *mut heman_image = heman_internal_generate_island_noise(width, height, seed);
    let mut data: *mut ::core::ffi::c_float = (*elevation).data;
    let mut invw: ::core::ffi::c_float =
        (1.0f64 / width as ::core::ffi::c_double) as ::core::ffi::c_float;
    let mut invh: ::core::ffi::c_float =
        (1.0f64 / height as ::core::ffi::c_double) as ::core::ffi::c_float;
    let mut y: ::core::ffi::c_int = 0;
    y = 0 as ::core::ffi::c_int;
    while y < height {
        let mut dst: *mut ::core::ffi::c_float = data.offset((y * width) as isize);
        let mut x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while x < width {
            let mut n: [::core::ffi::c_float; 3] = [0.; 3];
            let mut u: ::core::ffi::c_float = x as ::core::ffi::c_float * invw;
            let mut v: ::core::ffi::c_float = y as ::core::ffi::c_float * invh;
            heman_image_sample(noisetex, u, v, &raw mut n as *mut ::core::ffi::c_float);
            let mut z: ::core::ffi::c_float = 0.;
            heman_image_sample(sdf, u, v, &raw mut z);
            if z as ::core::ffi::c_double > 0.0f64 {
                let mut influence: ::core::ffi::c_float = z;
                u += influence * n[1 as ::core::ffi::c_int as usize];
                v += influence * n[2 as ::core::ffi::c_int as usize];
                heman_image_sample(sdf, u, v, &raw mut z);
                z += 6 as ::core::ffi::c_int as ::core::ffi::c_float
                    * influence
                    * n[0 as ::core::ffi::c_int as usize];
            }
            let fresh15 = dst;
            dst = dst.offset(1);
            *fresh15 = z;
            x += 1;
        }
        y += 1;
    }
    heman_image_destroy(noisetex);
    heman_image_destroy(sdf);
    return elevation;
}
#[no_mangle]
pub unsafe extern "C" fn heman_generate_archipelago_political_3(
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
    mut colors: *const heman_color,
    mut ncolors: ::core::ffi::c_int,
    mut ocean: heman_color,
    mut seed: ::core::ffi::c_int,
    mut political: *mut heman_image,
) -> *mut heman_image {
    let mut elevations: *mut *mut heman_image = malloc(
        (::core::mem::size_of::<*mut heman_image>() as size_t).wrapping_mul(ncolors as size_t),
    ) as *mut *mut heman_image;
    let mut cindex: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while cindex < ncolors {
        let ref mut fresh14 = *elevations.offset(cindex as isize);
        *fresh14 = heman_generate_archipelago_political_2(
            width,
            height,
            *colors.offset(cindex as isize),
            seed,
            political,
            1 as ::core::ffi::c_int,
        );
        cindex += 1;
    }
    let mut elevation: *mut heman_image =
        heman_image_create(width, height, 1 as ::core::ffi::c_int);
    heman_image_clear(elevation, 0 as ::core::ffi::c_int as ::core::ffi::c_float);
    let mut cindex_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while cindex_0 < ncolors {
        let mut y: ::core::ffi::c_int = 0;
        y = 0 as ::core::ffi::c_int;
        while y < height {
            let mut dst: *mut ::core::ffi::c_float = (*elevation).data.offset((y * width) as isize);
            let mut src: *mut ::core::ffi::c_float = (**elevations.offset(cindex_0 as isize))
                .data
                .offset((y * width) as isize);
            let mut x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while x < width {
                *dst = if *src > *dst { *src } else { *dst };
                x += 1;
                dst = dst.offset(1);
                src = src.offset(1);
            }
            y += 1;
        }
        heman_image_destroy(*elevations.offset(cindex_0 as isize));
        cindex_0 += 1;
    }
    free(elevations as *mut ::core::ffi::c_void);
    let mut ocean_elevation: *mut heman_image = heman_generate_archipelago_political_2(
        width,
        height,
        ocean,
        seed,
        political,
        0 as ::core::ffi::c_int,
    );
    let mut y_0: ::core::ffi::c_int = 0;
    y_0 = 0 as ::core::ffi::c_int;
    while y_0 < height {
        let mut dst_0: *mut ::core::ffi::c_float = (*elevation).data.offset((y_0 * width) as isize);
        let mut src_0: *mut ::core::ffi::c_float =
            (*ocean_elevation).data.offset((y_0 * width) as isize);
        let mut x_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while x_0 < width {
            if *src_0 < 0 as ::core::ffi::c_int as ::core::ffi::c_float {
                *dst_0 = *src_0;
            }
            x_0 += 1;
            dst_0 = dst_0.offset(1);
            src_0 = src_0.offset(1);
        }
        y_0 += 1;
    }
    heman_image_destroy(ocean_elevation);
    return elevation;
}
#[no_mangle]
pub unsafe extern "C" fn heman_generate_archipelago_political(
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
    mut points: *mut heman_points,
    mut colors: *const heman_color,
    mut ocean: heman_color,
    mut seed: ::core::ffi::c_int,
    mut elevation: *mut *mut heman_image,
    mut political: *mut *mut heman_image,
    mut elevation_mode: ::core::ffi::c_int,
) {
    *political = heman_generate_archipelago_political_1(width, height, points, colors, ocean, seed);
    if elevation_mode == 0 as ::core::ffi::c_int {
        *elevation = heman_generate_archipelago_political_2(
            width,
            height,
            ocean,
            seed,
            *political,
            0 as ::core::ffi::c_int,
        );
    } else {
        let mut ncolors: ::core::ffi::c_int = (*points).width;
        *elevation = heman_generate_archipelago_political_3(
            width, height, colors, ncolors, ocean, seed, *political,
        );
    };
}
