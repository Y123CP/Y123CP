extern "C" {
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
pub type size_t = usize;
#[no_mangle]
pub unsafe extern "C" fn heman_image_data(mut img: *mut heman_image) -> *mut ::core::ffi::c_float {
    return (*img).data;
}
#[no_mangle]
pub unsafe extern "C" fn heman_image_array(
    mut img: *mut heman_image,
    mut data: *mut *mut ::core::ffi::c_float,
    mut nfloats: *mut ::core::ffi::c_int,
) {
    *data = (*img).data;
    *nfloats = (*img).width * (*img).height * (*img).nbands;
}
#[no_mangle]
pub unsafe extern "C" fn heman_image_info(
    mut img: *mut heman_image,
    mut width: *mut ::core::ffi::c_int,
    mut height: *mut ::core::ffi::c_int,
    mut nbands: *mut ::core::ffi::c_int,
) {
    *width = (*img).width;
    *height = (*img).height;
    *nbands = (*img).nbands;
}
#[no_mangle]
pub unsafe extern "C" fn heman_image_texel(
    mut img: *mut heman_image,
    mut x: ::core::ffi::c_int,
    mut y: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_float {
    return (*img)
        .data
        .offset((y * (*img).width * (*img).nbands) as isize)
        .offset((x * (*img).nbands) as isize);
}
#[no_mangle]
pub unsafe extern "C" fn heman_image_create(
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
    mut nbands: ::core::ffi::c_int,
) -> *mut heman_image {
    let mut img: *mut heman_image =
        malloc(::core::mem::size_of::<heman_image>() as size_t) as *mut heman_image;
    (*img).width = width;
    (*img).height = height;
    (*img).nbands = nbands;
    (*img).data = malloc(
        (::core::mem::size_of::<::core::ffi::c_float>() as size_t)
            .wrapping_mul(width as size_t)
            .wrapping_mul(height as size_t)
            .wrapping_mul(nbands as size_t),
    ) as *mut ::core::ffi::c_float;
    return img;
}
#[no_mangle]
pub unsafe extern "C" fn heman_image_destroy(mut img: *mut heman_image) {
    free((*img).data as *mut ::core::ffi::c_void);
    free(img as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn heman_image_sample(
    mut img: *mut heman_image,
    mut u: ::core::ffi::c_float,
    mut v: ::core::ffi::c_float,
    mut result: *mut ::core::ffi::c_float,
) {
    let mut x: ::core::ffi::c_int = (if 0 as ::core::ffi::c_int as ::core::ffi::c_float
        > (if ((*img).width - 1 as ::core::ffi::c_int) as ::core::ffi::c_float
            > (*img).width as ::core::ffi::c_float * u
        {
            (*img).width as ::core::ffi::c_float * u
        } else {
            ((*img).width - 1 as ::core::ffi::c_int) as ::core::ffi::c_float
        }) {
        0 as ::core::ffi::c_int as ::core::ffi::c_float
    } else if ((*img).width - 1 as ::core::ffi::c_int) as ::core::ffi::c_float
        > (*img).width as ::core::ffi::c_float * u
    {
        (*img).width as ::core::ffi::c_float * u
    } else {
        ((*img).width - 1 as ::core::ffi::c_int) as ::core::ffi::c_float
    }) as ::core::ffi::c_int;
    let mut y: ::core::ffi::c_int = (if 0 as ::core::ffi::c_int as ::core::ffi::c_float
        > (if ((*img).height - 1 as ::core::ffi::c_int) as ::core::ffi::c_float
            > (*img).height as ::core::ffi::c_float * v
        {
            (*img).height as ::core::ffi::c_float * v
        } else {
            ((*img).height - 1 as ::core::ffi::c_int) as ::core::ffi::c_float
        }) {
        0 as ::core::ffi::c_int as ::core::ffi::c_float
    } else if ((*img).height - 1 as ::core::ffi::c_int) as ::core::ffi::c_float
        > (*img).height as ::core::ffi::c_float * v
    {
        (*img).height as ::core::ffi::c_float * v
    } else {
        ((*img).height - 1 as ::core::ffi::c_int) as ::core::ffi::c_float
    }) as ::core::ffi::c_int;
    let mut data: *mut ::core::ffi::c_float = heman_image_texel(img, x, y);
    let mut b: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while b < (*img).nbands {
        let fresh0 = data;
        data = data.offset(1);
        let fresh1 = result;
        result = result.offset(1);
        *fresh1 = *fresh0;
        b += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn heman_image_clear(
    mut img: *mut heman_image,
    mut value: ::core::ffi::c_float,
) {
    let mut size: ::core::ffi::c_int = (*img).width * (*img).height * (*img).nbands;
    let mut dst: *mut ::core::ffi::c_float = (*img).data;
    loop {
        let fresh2 = size;
        size = size - 1;
        if !(fresh2 != 0) {
            break;
        }
        let fresh3 = dst;
        dst = dst.offset(1);
        *fresh3 = value;
    }
}
#[no_mangle]
pub unsafe extern "C" fn heman_image_extract_alpha(mut img: *mut heman_image) -> *mut heman_image {
    let mut retval: *mut heman_image =
        heman_image_create((*img).width, (*img).height, 1 as ::core::ffi::c_int);
    let mut size: ::core::ffi::c_int = (*img).width * (*img).height;
    let mut src: *mut ::core::ffi::c_float = (*img).data;
    let mut dst: *mut ::core::ffi::c_float = (*retval).data;
    loop {
        let fresh4 = size;
        size = size - 1;
        if !(fresh4 != 0) {
            break;
        }
        src = src.offset(3 as ::core::ffi::c_int as isize);
        let fresh5 = src;
        src = src.offset(1);
        let fresh6 = dst;
        dst = dst.offset(1);
        *fresh6 = *fresh5;
    }
    return retval;
}
#[no_mangle]
pub unsafe extern "C" fn heman_image_extract_rgb(mut img: *mut heman_image) -> *mut heman_image {
    let mut retval: *mut heman_image =
        heman_image_create((*img).width, (*img).height, 3 as ::core::ffi::c_int);
    let mut size: ::core::ffi::c_int = (*img).width * (*img).height;
    let mut src: *mut ::core::ffi::c_float = (*img).data;
    let mut dst: *mut ::core::ffi::c_float = (*retval).data;
    loop {
        let fresh7 = size;
        size = size - 1;
        if !(fresh7 != 0) {
            break;
        }
        let fresh8 = src;
        src = src.offset(1);
        let fresh9 = dst;
        dst = dst.offset(1);
        *fresh9 = *fresh8;
        let fresh10 = src;
        src = src.offset(1);
        let fresh11 = dst;
        dst = dst.offset(1);
        *fresh11 = *fresh10;
        let fresh12 = src;
        src = src.offset(1);
        let fresh13 = dst;
        dst = dst.offset(1);
        *fresh13 = *fresh12;
        src = src.offset(1);
    }
    return retval;
}
