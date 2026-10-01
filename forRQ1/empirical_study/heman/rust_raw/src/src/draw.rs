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
    fn heman_image_clear(_: *mut heman_image, value: ::core::ffi::c_float);
    fn heman_points_destroy(_: *mut heman_points);
    fn generate_gaussian_splat(target: *mut ::core::ffi::c_float, fwidth: ::core::ffi::c_int);
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn heman_internal_draw_seeds(
        target: *mut heman_image,
        pts: *mut heman_points,
        filterd: ::core::ffi::c_int,
    );
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
#[no_mangle]
pub unsafe extern "C" fn heman_draw_points(
    mut target: *mut heman_image,
    mut pts: *mut heman_points,
    mut val: ::core::ffi::c_float,
) {
    let mut src: *mut ::core::ffi::c_float = (*pts).data;
    let mut k: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while k < (*pts).width {
        let mut x: ::core::ffi::c_float = *src.offset(0 as ::core::ffi::c_int as isize);
        let mut y: ::core::ffi::c_float = *src.offset(1 as ::core::ffi::c_int as isize);
        src = src.offset((*pts).nbands as isize);
        let mut i: ::core::ffi::c_int =
            (x * (*target).width as ::core::ffi::c_float) as ::core::ffi::c_int;
        let mut j: ::core::ffi::c_int =
            (y * (*target).height as ::core::ffi::c_float) as ::core::ffi::c_int;
        if !(i < 0 as ::core::ffi::c_int
            || i >= (*target).width
            || j < 0 as ::core::ffi::c_int
            || j >= (*target).height)
        {
            let mut texel: *mut ::core::ffi::c_float = heman_image_texel(target, i, j);
            let mut c: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while c < (*target).nbands {
                let fresh0 = texel;
                texel = texel.offset(1);
                *fresh0 = val;
                c += 1;
            }
        }
        k += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn heman_draw_colored_points(
    mut target: *mut heman_image,
    mut pts: *mut heman_points,
    mut colors: *const heman_color,
) {
    let mut src: *mut ::core::ffi::c_float = (*pts).data;
    let mut inv: ::core::ffi::c_float = 1.0f32 / 255.0f32;
    let mut k: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while k < (*pts).width {
        let mut x: ::core::ffi::c_float = *src.offset(0 as ::core::ffi::c_int as isize);
        let mut y: ::core::ffi::c_float = *src.offset(1 as ::core::ffi::c_int as isize);
        src = src.offset((*pts).nbands as isize);
        let mut i: ::core::ffi::c_int =
            (x * (*target).width as ::core::ffi::c_float) as ::core::ffi::c_int;
        let mut j: ::core::ffi::c_int =
            (y * (*target).height as ::core::ffi::c_float) as ::core::ffi::c_int;
        if !(i < 0 as ::core::ffi::c_int
            || i >= (*target).width
            || j < 0 as ::core::ffi::c_int
            || j >= (*target).height)
        {
            let mut texel: *mut ::core::ffi::c_float = heman_image_texel(target, i, j);
            let mut rgb: heman_color = *colors.offset(k as isize);
            let fresh1 = texel;
            texel = texel.offset(1);
            *fresh1 = (rgb as ::core::ffi::c_uint >> 16 as ::core::ffi::c_int
                & 0xff as ::core::ffi::c_uint) as ::core::ffi::c_float
                * inv;
            let fresh2 = texel;
            texel = texel.offset(1);
            *fresh2 = (rgb as ::core::ffi::c_uint >> 8 as ::core::ffi::c_int
                & 0xff as ::core::ffi::c_uint) as ::core::ffi::c_float
                * inv;
            let fresh3 = texel;
            texel = texel.offset(1);
            *fresh3 = (rgb as ::core::ffi::c_uint & 0xff as ::core::ffi::c_uint)
                as ::core::ffi::c_float
                * inv;
            if (*target).nbands == 4 as ::core::ffi::c_int {
                *texel = (rgb >> 24 as ::core::ffi::c_int) as ::core::ffi::c_float * inv;
            }
        }
        k += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn heman_draw_colored_circles(
    mut target: *mut heman_image,
    mut pts: *mut heman_points,
    mut radius: ::core::ffi::c_int,
    mut colors: *const heman_color,
) {
    let mut fwidth: ::core::ffi::c_int = radius * 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int;
    let mut radius2: ::core::ffi::c_int = radius * radius;
    let mut src: *mut ::core::ffi::c_float = (*pts).data;
    let mut inv: ::core::ffi::c_float = 1.0f32 / 255.0f32;
    let mut w: ::core::ffi::c_int = (*target).width;
    let mut h: ::core::ffi::c_int = (*target).height;
    let mut k: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while k < (*pts).width {
        let mut x: ::core::ffi::c_float = *src.offset(0 as ::core::ffi::c_int as isize);
        let mut y: ::core::ffi::c_float = *src.offset(1 as ::core::ffi::c_int as isize);
        src = src.offset((*pts).nbands as isize);
        let mut ii: ::core::ffi::c_int =
            (x * w as ::core::ffi::c_float - radius as ::core::ffi::c_float) as ::core::ffi::c_int;
        let mut jj: ::core::ffi::c_int =
            (y * h as ::core::ffi::c_float - radius as ::core::ffi::c_float) as ::core::ffi::c_int;
        let mut kj: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while kj < fwidth {
            let mut ki: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while ki < fwidth {
                let mut i: ::core::ffi::c_int = ii + ki;
                let mut j: ::core::ffi::c_int = jj + kj;
                let mut r2: ::core::ffi::c_int = ((i as ::core::ffi::c_float
                    - x * w as ::core::ffi::c_float)
                    * (i as ::core::ffi::c_float - x * w as ::core::ffi::c_float)
                    + (j as ::core::ffi::c_float - y * h as ::core::ffi::c_float)
                        * (j as ::core::ffi::c_float - y * h as ::core::ffi::c_float))
                    as ::core::ffi::c_int;
                if !(r2 > radius2) {
                    let mut texel: *mut ::core::ffi::c_float = heman_image_texel(target, i, j);
                    let mut rgb: heman_color = *colors.offset(k as isize);
                    let fresh4 = texel;
                    texel = texel.offset(1);
                    *fresh4 = (rgb >> 16 as ::core::ffi::c_int) as ::core::ffi::c_float * inv;
                    let fresh5 = texel;
                    texel = texel.offset(1);
                    *fresh5 = (rgb as ::core::ffi::c_uint >> 8 as ::core::ffi::c_int
                        & 0xff as ::core::ffi::c_uint)
                        as ::core::ffi::c_float
                        * inv;
                    *texel = (rgb as ::core::ffi::c_uint & 0xff as ::core::ffi::c_uint)
                        as ::core::ffi::c_float
                        * inv;
                }
                ki += 1;
            }
            kj += 1;
        }
        k += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn heman_draw_splats(
    mut target: *mut heman_image,
    mut pts: *mut heman_points,
    mut radius: ::core::ffi::c_int,
    mut blend_mode: ::core::ffi::c_int,
) {
    let mut fwidth: ::core::ffi::c_int = radius * 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int;
    let mut gaussian_splat: *mut ::core::ffi::c_float = malloc(
        ((fwidth * fwidth) as size_t)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_float>() as size_t),
    ) as *mut ::core::ffi::c_float;
    generate_gaussian_splat(gaussian_splat, fwidth);
    let mut src: *mut ::core::ffi::c_float = (*pts).data;
    let mut w: ::core::ffi::c_int = (*target).width;
    let mut h: ::core::ffi::c_int = (*target).height;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*pts).width {
        let fresh6 = src;
        src = src.offset(1);
        let mut x: ::core::ffi::c_float = *fresh6;
        let fresh7 = src;
        src = src.offset(1);
        let mut y: ::core::ffi::c_float = *fresh7;
        let mut ii: ::core::ffi::c_int =
            (x * w as ::core::ffi::c_float - radius as ::core::ffi::c_float) as ::core::ffi::c_int;
        let mut jj: ::core::ffi::c_int =
            (y * h as ::core::ffi::c_float - radius as ::core::ffi::c_float) as ::core::ffi::c_int;
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
                    let mut c: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    while c < (*target).nbands {
                        let fresh8 = texel;
                        texel = texel.offset(1);
                        *fresh8 += *gaussian_splat.offset((kj * fwidth + ki) as isize);
                        c += 1;
                    }
                }
                ki += 1;
            }
            kj += 1;
        }
        i += 1;
    }
    free(gaussian_splat as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn heman_draw_contour_from_points(
    mut target: *mut heman_image,
    mut coords: *mut heman_points,
    mut rgb: heman_color,
    mut mind: ::core::ffi::c_float,
    mut maxd: ::core::ffi::c_float,
    mut filterd: ::core::ffi::c_int,
) {
    let mut width: ::core::ffi::c_int = (*target).width;
    let mut height: ::core::ffi::c_int = (*target).height;
    let mut seed: *mut heman_image = heman_image_create(width, height, 1 as ::core::ffi::c_int);
    heman_image_clear(seed, 0 as ::core::ffi::c_int as ::core::ffi::c_float);
    heman_internal_draw_seeds(seed, coords, filterd);
    let mut inv: ::core::ffi::c_float = 1.0f32 / 255.0f32;
    let mut r: ::core::ffi::c_float = (rgb as ::core::ffi::c_uint >> 16 as ::core::ffi::c_int
        & 0xff as ::core::ffi::c_uint)
        as ::core::ffi::c_float
        * inv;
    let mut g: ::core::ffi::c_float = (rgb as ::core::ffi::c_uint >> 8 as ::core::ffi::c_int
        & 0xff as ::core::ffi::c_uint)
        as ::core::ffi::c_float
        * inv;
    let mut b: ::core::ffi::c_float =
        (rgb as ::core::ffi::c_uint & 0xff as ::core::ffi::c_uint) as ::core::ffi::c_float * inv;
    let mut a: ::core::ffi::c_float = 1 as ::core::ffi::c_int as ::core::ffi::c_float;
    if (*target).nbands == 4 as ::core::ffi::c_int {
        a = (rgb >> 24 as ::core::ffi::c_int) as ::core::ffi::c_float * inv;
    }
    let mut y: ::core::ffi::c_int = 0;
    y = 0 as ::core::ffi::c_int;
    while y < height {
        let mut dst: *mut ::core::ffi::c_float = (*target)
            .data
            .offset((y * width * (*target).nbands) as isize);
        let mut x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while x < width {
            let mut dist: ::core::ffi::c_float = *heman_image_texel(seed, x, y);
            if dist > mind && dist < maxd {
                *dst.offset(0 as ::core::ffi::c_int as isize) = r;
                *dst.offset(1 as ::core::ffi::c_int as isize) = g;
                *dst.offset(2 as ::core::ffi::c_int as isize) = b;
                if (*target).nbands == 4 as ::core::ffi::c_int {
                    *dst.offset(3 as ::core::ffi::c_int as isize) = a;
                }
            }
            dst = dst.offset((*target).nbands as isize);
            x += 1;
        }
        y += 1;
    }
    heman_points_destroy(seed as *mut heman_points);
}
