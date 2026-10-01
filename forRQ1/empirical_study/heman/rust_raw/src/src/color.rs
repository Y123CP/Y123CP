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
    fn heman_image_sample(
        _: *mut heman_image,
        u: ::core::ffi::c_float,
        v: ::core::ffi::c_float,
        result: *mut ::core::ffi::c_float,
    );
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn pow(__x: ::core::ffi::c_double, __y: ::core::ffi::c_double) -> ::core::ffi::c_double;
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
pub type heman_color = ::core::ffi::c_uint;
pub type size_t = usize;
#[no_mangle]
pub static mut _gamma: ::core::ffi::c_float = 2.2f32;
#[no_mangle]
pub unsafe extern "C" fn heman_color_set_gamma(mut g: ::core::ffi::c_float) {
    _gamma = g;
}
#[no_mangle]
pub unsafe extern "C" fn heman_color_create_gradient(
    mut width: ::core::ffi::c_int,
    mut num_colors: ::core::ffi::c_int,
    mut cp_locations: *const ::core::ffi::c_int,
    mut cp_values: *const heman_color,
) -> *mut heman_image {
    let mut f32colors: *mut ::core::ffi::c_float = malloc(
        (::core::mem::size_of::<::core::ffi::c_float>() as size_t)
            .wrapping_mul(3 as size_t)
            .wrapping_mul(num_colors as size_t),
    ) as *mut ::core::ffi::c_float;
    let mut inv: ::core::ffi::c_float = 1.0f32 / 255.0f32;
    let mut f32color: *mut ::core::ffi::c_float = f32colors;
    let mut u32color: *const heman_color = cp_values;
    let mut index: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while index < num_colors {
        let fresh0 = u32color;
        u32color = u32color.offset(1);
        let mut rgb: heman_color = *fresh0;
        let mut r: ::core::ffi::c_float =
            (rgb >> 16 as ::core::ffi::c_int) as ::core::ffi::c_float * inv;
        let mut g: ::core::ffi::c_float = (rgb as ::core::ffi::c_uint >> 8 as ::core::ffi::c_int
            & 0xff as ::core::ffi::c_uint)
            as ::core::ffi::c_float
            * inv;
        let mut b: ::core::ffi::c_float = (rgb as ::core::ffi::c_uint & 0xff as ::core::ffi::c_uint)
            as ::core::ffi::c_float
            * inv;
        let fresh1 = f32color;
        f32color = f32color.offset(1);
        *fresh1 = pow(r as ::core::ffi::c_double, _gamma as ::core::ffi::c_double)
            as ::core::ffi::c_float;
        let fresh2 = f32color;
        f32color = f32color.offset(1);
        *fresh2 = pow(g as ::core::ffi::c_double, _gamma as ::core::ffi::c_double)
            as ::core::ffi::c_float;
        let fresh3 = f32color;
        f32color = f32color.offset(1);
        *fresh3 = pow(b as ::core::ffi::c_double, _gamma as ::core::ffi::c_double)
            as ::core::ffi::c_float;
        index += 1;
    }
    let mut result: *mut heman_image =
        heman_image_create(width, 1 as ::core::ffi::c_int, 3 as ::core::ffi::c_int);
    let mut index0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut index1: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut dst: *mut ::core::ffi::c_float = (*result).data;
    let mut t: ::core::ffi::c_float = 0.;
    let mut invgamma: ::core::ffi::c_float = 1.0f32 / _gamma;
    let mut current_block_16: u64;
    let mut x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while x < width {
        let mut loc0: ::core::ffi::c_int = *cp_locations.offset(index0 as isize);
        let mut loc1: ::core::ffi::c_int = *cp_locations.offset(index1 as isize);
        if loc0 == loc1 {
            t = 0 as ::core::ffi::c_int as ::core::ffi::c_float;
            current_block_16 = 11584701595673473500;
        } else {
            t = (x - loc0) as ::core::ffi::c_float / (loc1 - loc0) as ::core::ffi::c_float;
            if t >= 1 as ::core::ffi::c_int as ::core::ffi::c_float {
                x -= 1;
                index0 += 1;
                index1 = if index1 + 1 as ::core::ffi::c_int > num_colors - 1 as ::core::ffi::c_int
                {
                    num_colors - 1 as ::core::ffi::c_int
                } else {
                    index1 + 1 as ::core::ffi::c_int
                };
                current_block_16 = 5399440093318478209;
            } else {
                current_block_16 = 11584701595673473500;
            }
        }
        match current_block_16 {
            11584701595673473500 => {
                let mut r0: ::core::ffi::c_float =
                    *f32colors.offset((index0 * 3 as ::core::ffi::c_int) as isize);
                let mut g0: ::core::ffi::c_float = *f32colors
                    .offset((index0 * 3 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize);
                let mut b0: ::core::ffi::c_float = *f32colors
                    .offset((index0 * 3 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize);
                let mut r1: ::core::ffi::c_float =
                    *f32colors.offset((index1 * 3 as ::core::ffi::c_int) as isize);
                let mut g1: ::core::ffi::c_float = *f32colors
                    .offset((index1 * 3 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize);
                let mut b1: ::core::ffi::c_float = *f32colors
                    .offset((index1 * 3 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize);
                let mut invt: ::core::ffi::c_float = 1.0f32 - t;
                let mut r_0: ::core::ffi::c_float = r0 * invt + r1 * t;
                let mut g_0: ::core::ffi::c_float = g0 * invt + g1 * t;
                let mut b_0: ::core::ffi::c_float = b0 * invt + b1 * t;
                let fresh4 = dst;
                dst = dst.offset(1);
                *fresh4 = pow(
                    r_0 as ::core::ffi::c_double,
                    invgamma as ::core::ffi::c_double,
                ) as ::core::ffi::c_float;
                let fresh5 = dst;
                dst = dst.offset(1);
                *fresh5 = pow(
                    g_0 as ::core::ffi::c_double,
                    invgamma as ::core::ffi::c_double,
                ) as ::core::ffi::c_float;
                let fresh6 = dst;
                dst = dst.offset(1);
                *fresh6 = pow(
                    b_0 as ::core::ffi::c_double,
                    invgamma as ::core::ffi::c_double,
                ) as ::core::ffi::c_float;
            }
            _ => {}
        }
        x += 1;
    }
    free(f32colors as *mut ::core::ffi::c_void);
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn heman_color_apply_gradient(
    mut heightmap: *mut heman_image,
    mut minheight: ::core::ffi::c_float,
    mut maxheight: ::core::ffi::c_float,
    mut gradient: *mut heman_image,
) -> *mut heman_image {
    let mut w: ::core::ffi::c_int = (*heightmap).width;
    let mut h: ::core::ffi::c_int = (*heightmap).height;
    let mut result: *mut heman_image = heman_image_create(w, h, 3 as ::core::ffi::c_int);
    let mut size: ::core::ffi::c_int = (*result).height * (*result).width;
    let mut dst: *mut ::core::ffi::c_float = (*result).data;
    let mut src: *const ::core::ffi::c_float = (*heightmap).data;
    let mut scale: ::core::ffi::c_float = 1.0f32 / (maxheight - minheight);
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < size {
        let mut u: ::core::ffi::c_float = if 0.0f32
            > (if 1.0f32 > (*src - minheight) * scale {
                (*src - minheight) * scale
            } else {
                1.0f32
            }) {
            0.0f32
        } else if 1.0f32 > (*src - minheight) * scale {
            (*src - minheight) * scale
        } else {
            1.0f32
        };
        heman_image_sample(gradient, u, 0.5f32, dst);
        i += 1;
        dst = dst.offset(3 as ::core::ffi::c_int as isize);
        src = src.offset(1);
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn heman_color_from_grayscale(
    mut grayscale: *mut heman_image,
) -> *mut heman_image {
    let mut w: ::core::ffi::c_int = (*grayscale).width;
    let mut h: ::core::ffi::c_int = (*grayscale).height;
    let mut result: *mut heman_image = heman_image_create(w, h, 3 as ::core::ffi::c_int);
    let mut size: ::core::ffi::c_int = w * h;
    let mut dst: *mut ::core::ffi::c_float = (*result).data;
    let mut src: *const ::core::ffi::c_float = (*grayscale).data;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < size {
        let fresh7 = src;
        src = src.offset(1);
        let mut v: ::core::ffi::c_float = *fresh7;
        let fresh8 = dst;
        dst = dst.offset(1);
        *fresh8 = v;
        let fresh9 = dst;
        dst = dst.offset(1);
        *fresh9 = v;
        let fresh10 = dst;
        dst = dst.offset(1);
        *fresh10 = v;
        i += 1;
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn heman_color_to_grayscale(
    mut colorimg: *mut heman_image,
) -> *mut heman_image {
    let mut w: ::core::ffi::c_int = (*colorimg).width;
    let mut h: ::core::ffi::c_int = (*colorimg).height;
    let mut result: *mut heman_image = heman_image_create(w, h, 1 as ::core::ffi::c_int);
    let mut size: ::core::ffi::c_int = w * h;
    let mut dst: *mut ::core::ffi::c_float = (*result).data;
    let mut src: *const ::core::ffi::c_float = (*colorimg).data;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < size {
        let fresh11 = src;
        src = src.offset(1);
        let mut r: ::core::ffi::c_float = *fresh11;
        let fresh12 = src;
        src = src.offset(1);
        let mut g: ::core::ffi::c_float = *fresh12;
        let fresh13 = src;
        src = src.offset(1);
        let mut b: ::core::ffi::c_float = *fresh13;
        let fresh14 = dst;
        dst = dst.offset(1);
        *fresh14 = (0.299f64 * r as ::core::ffi::c_double
            + 0.587f64 * g as ::core::ffi::c_double
            + 0.114f64 * b as ::core::ffi::c_double) as ::core::ffi::c_float;
        i += 1;
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn heman_internal_rg(mut cfield: *mut heman_image) -> *mut heman_image {
    let mut w: ::core::ffi::c_int = (*cfield).width;
    let mut h: ::core::ffi::c_int = (*cfield).height;
    let mut target: *mut heman_image = heman_image_create(w, h, 3 as ::core::ffi::c_int);
    let mut dst: *mut ::core::ffi::c_float = (*target).data;
    let mut src: *mut ::core::ffi::c_float = (*cfield).data;
    let mut size: ::core::ffi::c_int = w * h;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < size {
        let fresh19 = src;
        src = src.offset(1);
        let mut u: ::core::ffi::c_float = *fresh19 / w as ::core::ffi::c_float;
        let fresh20 = src;
        src = src.offset(1);
        let mut v: ::core::ffi::c_float = *fresh20 / h as ::core::ffi::c_float;
        let fresh21 = dst;
        dst = dst.offset(1);
        *fresh21 = u;
        let fresh22 = dst;
        dst = dst.offset(1);
        *fresh22 = v;
        let fresh23 = dst;
        dst = dst.offset(1);
        *fresh23 = 0 as ::core::ffi::c_int as ::core::ffi::c_float;
        i += 1;
    }
    return target;
}
#[no_mangle]
pub unsafe extern "C" fn heman_color_from_cpcf(
    mut cfield: *mut heman_image,
    mut texture: *mut heman_image,
) -> *mut heman_image {
    if texture.is_null() {
        return heman_internal_rg(cfield);
    }
    let mut w: ::core::ffi::c_int = (*cfield).width;
    let mut h: ::core::ffi::c_int = (*cfield).height;
    let mut target: *mut heman_image = heman_image_create(w, h, (*texture).nbands);
    let mut dst: *mut ::core::ffi::c_float = (*target).data;
    let mut src: *mut ::core::ffi::c_float = (*cfield).data;
    let mut size: ::core::ffi::c_int = w * h;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < size {
        let fresh15 = src;
        src = src.offset(1);
        let mut u: ::core::ffi::c_float = *fresh15;
        let fresh16 = src;
        src = src.offset(1);
        let mut v: ::core::ffi::c_float = *fresh16;
        let mut texel: *mut ::core::ffi::c_float =
            heman_image_texel(texture, u as ::core::ffi::c_int, v as ::core::ffi::c_int);
        let mut c: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while c < (*texture).nbands {
            let fresh17 = texel;
            texel = texel.offset(1);
            let fresh18 = dst;
            dst = dst.offset(1);
            *fresh18 = *fresh17;
            c += 1;
        }
        i += 1;
    }
    return target;
}
