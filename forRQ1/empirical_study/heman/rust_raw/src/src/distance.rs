extern "C" {
    fn heman_image_create(
        width: ::core::ffi::c_int,
        height: ::core::ffi::c_int,
        nbands: ::core::ffi::c_int,
    ) -> *mut heman_image;
    fn heman_image_destroy(_: *mut heman_image);
    fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn sqrt(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
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
pub type uint16_t = __uint16_t;
pub type __uint16_t = u16;
pub type size_t = usize;
#[no_mangle]
pub static mut INF: ::core::ffi::c_float = 1E20f32;
unsafe extern "C" fn edt(
    mut f: *mut ::core::ffi::c_float,
    mut d: *mut ::core::ffi::c_float,
    mut z: *mut ::core::ffi::c_float,
    mut w: *mut uint16_t,
    mut n: ::core::ffi::c_int,
) {
    let mut k: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut s: ::core::ffi::c_float = 0.;
    *w.offset(0 as ::core::ffi::c_int as isize) = 0 as uint16_t;
    *z.offset(0 as ::core::ffi::c_int as isize) = -INF;
    *z.offset(1 as ::core::ffi::c_int as isize) = INF;
    let mut q: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    while q < n {
        s = (*f.offset(q as isize) + (q * q) as ::core::ffi::c_float
            - (*f.offset(*w.offset(k as isize) as isize)
                + (*w.offset(k as isize) as ::core::ffi::c_int
                    * *w.offset(k as isize) as ::core::ffi::c_int)
                    as ::core::ffi::c_float))
            / (2 as ::core::ffi::c_int * q
                - 2 as ::core::ffi::c_int * *w.offset(k as isize) as ::core::ffi::c_int)
                as ::core::ffi::c_float;
        while s <= *z.offset(k as isize) {
            k -= 1;
            s = (*f.offset(q as isize) + (q * q) as ::core::ffi::c_float
                - (*f.offset(*w.offset(k as isize) as isize)
                    + (*w.offset(k as isize) as ::core::ffi::c_int
                        * *w.offset(k as isize) as ::core::ffi::c_int)
                        as ::core::ffi::c_float))
                / (2 as ::core::ffi::c_int * q
                    - 2 as ::core::ffi::c_int * *w.offset(k as isize) as ::core::ffi::c_int)
                    as ::core::ffi::c_float;
        }
        k += 1;
        *w.offset(k as isize) = q as uint16_t;
        *z.offset(k as isize) = s;
        *z.offset((k + 1 as ::core::ffi::c_int) as isize) = INF;
        q += 1;
    }
    k = 0 as ::core::ffi::c_int;
    let mut q_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while q_0 < n {
        while *z.offset((k + 1 as ::core::ffi::c_int) as isize) < q_0 as ::core::ffi::c_float {
            k += 1;
        }
        *d.offset(q_0 as isize) = ((q_0 - *w.offset(k as isize) as ::core::ffi::c_int)
            * (q_0 - *w.offset(k as isize) as ::core::ffi::c_int))
            as ::core::ffi::c_float
            + *f.offset(*w.offset(k as isize) as isize);
        q_0 += 1;
    }
}
unsafe extern "C" fn edt_with_payload(
    mut f: *mut ::core::ffi::c_float,
    mut d: *mut ::core::ffi::c_float,
    mut z: *mut ::core::ffi::c_float,
    mut w: *mut uint16_t,
    mut n: ::core::ffi::c_int,
    mut payload_in: *mut ::core::ffi::c_float,
    mut payload_out: *mut ::core::ffi::c_float,
) {
    let mut k: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut s: ::core::ffi::c_float = 0.;
    *w.offset(0 as ::core::ffi::c_int as isize) = 0 as uint16_t;
    *z.offset(0 as ::core::ffi::c_int as isize) = -INF;
    *z.offset(1 as ::core::ffi::c_int as isize) = INF;
    let mut q: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    while q < n {
        s = (*f.offset(q as isize) + (q * q) as ::core::ffi::c_float
            - (*f.offset(*w.offset(k as isize) as isize)
                + (*w.offset(k as isize) as ::core::ffi::c_int
                    * *w.offset(k as isize) as ::core::ffi::c_int)
                    as ::core::ffi::c_float))
            / (2 as ::core::ffi::c_int * q
                - 2 as ::core::ffi::c_int * *w.offset(k as isize) as ::core::ffi::c_int)
                as ::core::ffi::c_float;
        while s <= *z.offset(k as isize) {
            k -= 1;
            s = (*f.offset(q as isize) + (q * q) as ::core::ffi::c_float
                - (*f.offset(*w.offset(k as isize) as isize)
                    + (*w.offset(k as isize) as ::core::ffi::c_int
                        * *w.offset(k as isize) as ::core::ffi::c_int)
                        as ::core::ffi::c_float))
                / (2 as ::core::ffi::c_int * q
                    - 2 as ::core::ffi::c_int * *w.offset(k as isize) as ::core::ffi::c_int)
                    as ::core::ffi::c_float;
        }
        k += 1;
        *w.offset(k as isize) = q as uint16_t;
        *z.offset(k as isize) = s;
        *z.offset((k + 1 as ::core::ffi::c_int) as isize) = INF;
        q += 1;
    }
    k = 0 as ::core::ffi::c_int;
    let mut q_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while q_0 < n {
        while *z.offset((k + 1 as ::core::ffi::c_int) as isize) < q_0 as ::core::ffi::c_float {
            k += 1;
        }
        *d.offset(q_0 as isize) = ((q_0 - *w.offset(k as isize) as ::core::ffi::c_int)
            * (q_0 - *w.offset(k as isize) as ::core::ffi::c_int))
            as ::core::ffi::c_float
            + *f.offset(*w.offset(k as isize) as isize);
        *payload_out.offset((q_0 * 2 as ::core::ffi::c_int) as isize) = *payload_in.offset(
            (*w.offset(k as isize) as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize,
        );
        *payload_out.offset((q_0 * 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize) =
            *payload_in.offset(
                (*w.offset(k as isize) as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                    + 1 as ::core::ffi::c_int) as isize,
            );
        q_0 += 1;
    }
}
unsafe extern "C" fn transform_to_distance(mut sdf: *mut heman_image) {
    let mut width: ::core::ffi::c_int = (*sdf).width;
    let mut height: ::core::ffi::c_int = (*sdf).height;
    let mut size: ::core::ffi::c_int = width * height;
    let mut ff: *mut ::core::ffi::c_float = calloc(
        size as size_t,
        ::core::mem::size_of::<::core::ffi::c_float>() as size_t,
    ) as *mut ::core::ffi::c_float;
    let mut dd: *mut ::core::ffi::c_float = calloc(
        size as size_t,
        ::core::mem::size_of::<::core::ffi::c_float>() as size_t,
    ) as *mut ::core::ffi::c_float;
    let mut zz: *mut ::core::ffi::c_float = calloc(
        ((height + 1 as ::core::ffi::c_int) * (width + 1 as ::core::ffi::c_int)) as size_t,
        ::core::mem::size_of::<::core::ffi::c_float>() as size_t,
    ) as *mut ::core::ffi::c_float;
    let mut ww: *mut uint16_t =
        calloc(size as size_t, ::core::mem::size_of::<uint16_t>() as size_t) as *mut uint16_t;
    let mut x: ::core::ffi::c_int = 0;
    x = 0 as ::core::ffi::c_int;
    while x < width {
        let mut f: *mut ::core::ffi::c_float = ff.offset((height * x) as isize);
        let mut d: *mut ::core::ffi::c_float = dd.offset((height * x) as isize);
        let mut z: *mut ::core::ffi::c_float =
            zz.offset(((height + 1 as ::core::ffi::c_int) * x) as isize);
        let mut w: *mut uint16_t = ww.offset((height * x) as isize);
        let mut y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while y < height {
            *f.offset(y as isize) = *(*sdf).data.offset((y * width) as isize).offset(x as isize);
            y += 1;
        }
        edt(f, d, z, w, height);
        let mut y_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while y_0 < height {
            *(*sdf)
                .data
                .offset((y_0 * width) as isize)
                .offset(x as isize) = *d.offset(y_0 as isize);
            y_0 += 1;
        }
        x += 1;
    }
    let mut y_1: ::core::ffi::c_int = 0;
    y_1 = 0 as ::core::ffi::c_int;
    while y_1 < height {
        let mut f_0: *mut ::core::ffi::c_float = ff.offset((width * y_1) as isize);
        let mut d_0: *mut ::core::ffi::c_float = dd.offset((width * y_1) as isize);
        let mut z_0: *mut ::core::ffi::c_float =
            zz.offset(((width + 1 as ::core::ffi::c_int) * y_1) as isize);
        let mut w_0: *mut uint16_t = ww.offset((width * y_1) as isize);
        let mut x_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while x_0 < width {
            *f_0.offset(x_0 as isize) = *(*sdf)
                .data
                .offset((y_1 * width) as isize)
                .offset(x_0 as isize);
            x_0 += 1;
        }
        edt(f_0, d_0, z_0, w_0, width);
        let mut x_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while x_1 < width {
            *(*sdf)
                .data
                .offset((y_1 * width) as isize)
                .offset(x_1 as isize) = *d_0.offset(x_1 as isize);
            x_1 += 1;
        }
        y_1 += 1;
    }
    free(ff as *mut ::core::ffi::c_void);
    free(dd as *mut ::core::ffi::c_void);
    free(zz as *mut ::core::ffi::c_void);
    free(ww as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn transform_to_coordfield(mut sdf: *mut heman_image, mut cf: *mut heman_image) {
    let mut width: ::core::ffi::c_int = (*sdf).width;
    let mut height: ::core::ffi::c_int = (*sdf).height;
    let mut size: ::core::ffi::c_int = width * height;
    let mut ff: *mut ::core::ffi::c_float = calloc(
        size as size_t,
        ::core::mem::size_of::<::core::ffi::c_float>() as size_t,
    ) as *mut ::core::ffi::c_float;
    let mut dd: *mut ::core::ffi::c_float = calloc(
        size as size_t,
        ::core::mem::size_of::<::core::ffi::c_float>() as size_t,
    ) as *mut ::core::ffi::c_float;
    let mut zz: *mut ::core::ffi::c_float = calloc(
        ((height + 1 as ::core::ffi::c_int) * (width + 1 as ::core::ffi::c_int)) as size_t,
        ::core::mem::size_of::<::core::ffi::c_float>() as size_t,
    ) as *mut ::core::ffi::c_float;
    let mut ww: *mut uint16_t =
        calloc(size as size_t, ::core::mem::size_of::<uint16_t>() as size_t) as *mut uint16_t;
    let mut x: ::core::ffi::c_int = 0;
    x = 0 as ::core::ffi::c_int;
    while x < width {
        let mut pl1: *mut ::core::ffi::c_float = calloc(
            (height * 2 as ::core::ffi::c_int) as size_t,
            ::core::mem::size_of::<::core::ffi::c_float>() as size_t,
        ) as *mut ::core::ffi::c_float;
        let mut pl2: *mut ::core::ffi::c_float = calloc(
            (height * 2 as ::core::ffi::c_int) as size_t,
            ::core::mem::size_of::<::core::ffi::c_float>() as size_t,
        ) as *mut ::core::ffi::c_float;
        let mut f: *mut ::core::ffi::c_float = ff.offset((height * x) as isize);
        let mut d: *mut ::core::ffi::c_float = dd.offset((height * x) as isize);
        let mut z: *mut ::core::ffi::c_float =
            zz.offset(((height + 1 as ::core::ffi::c_int) * x) as isize);
        let mut w: *mut uint16_t = ww.offset((height * x) as isize);
        let mut y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while y < height {
            *f.offset(y as isize) = *(*sdf).data.offset((y * width) as isize).offset(x as isize);
            *pl1.offset((y * 2 as ::core::ffi::c_int) as isize) = *(*cf)
                .data
                .offset((2 as ::core::ffi::c_int * (y * width + x)) as isize)
                .offset(0 as ::core::ffi::c_int as isize);
            *pl1.offset((y * 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize) = *(*cf)
                .data
                .offset((2 as ::core::ffi::c_int * (y * width + x)) as isize)
                .offset(1 as ::core::ffi::c_int as isize);
            y += 1;
        }
        edt_with_payload(f, d, z, w, height, pl1, pl2);
        let mut y_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while y_0 < height {
            *(*sdf)
                .data
                .offset((y_0 * width) as isize)
                .offset(x as isize) = *d.offset(y_0 as isize);
            *(*cf)
                .data
                .offset((2 as ::core::ffi::c_int * (y_0 * width + x)) as isize)
                .offset(0 as ::core::ffi::c_int as isize) =
                *pl2.offset((2 as ::core::ffi::c_int * y_0) as isize);
            *(*cf)
                .data
                .offset((2 as ::core::ffi::c_int * (y_0 * width + x)) as isize)
                .offset(1 as ::core::ffi::c_int as isize) =
                *pl2.offset((2 as ::core::ffi::c_int * y_0 + 1 as ::core::ffi::c_int) as isize);
            y_0 += 1;
        }
        free(pl1 as *mut ::core::ffi::c_void);
        free(pl2 as *mut ::core::ffi::c_void);
        x += 1;
    }
    let mut y_1: ::core::ffi::c_int = 0;
    y_1 = 0 as ::core::ffi::c_int;
    while y_1 < height {
        let mut pl1_0: *mut ::core::ffi::c_float = calloc(
            (width * 2 as ::core::ffi::c_int) as size_t,
            ::core::mem::size_of::<::core::ffi::c_float>() as size_t,
        ) as *mut ::core::ffi::c_float;
        let mut pl2_0: *mut ::core::ffi::c_float = calloc(
            (width * 2 as ::core::ffi::c_int) as size_t,
            ::core::mem::size_of::<::core::ffi::c_float>() as size_t,
        ) as *mut ::core::ffi::c_float;
        let mut f_0: *mut ::core::ffi::c_float = ff.offset((width * y_1) as isize);
        let mut d_0: *mut ::core::ffi::c_float = dd.offset((width * y_1) as isize);
        let mut z_0: *mut ::core::ffi::c_float =
            zz.offset(((width + 1 as ::core::ffi::c_int) * y_1) as isize);
        let mut w_0: *mut uint16_t = ww.offset((width * y_1) as isize);
        let mut x_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while x_0 < width {
            *f_0.offset(x_0 as isize) = *(*sdf)
                .data
                .offset((y_1 * width) as isize)
                .offset(x_0 as isize);
            *pl1_0.offset((x_0 * 2 as ::core::ffi::c_int) as isize) = *(*cf)
                .data
                .offset((2 as ::core::ffi::c_int * (y_1 * width + x_0)) as isize)
                .offset(0 as ::core::ffi::c_int as isize);
            *pl1_0.offset((x_0 * 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize) =
                *(*cf)
                    .data
                    .offset((2 as ::core::ffi::c_int * (y_1 * width + x_0)) as isize)
                    .offset(1 as ::core::ffi::c_int as isize);
            x_0 += 1;
        }
        edt_with_payload(f_0, d_0, z_0, w_0, width, pl1_0, pl2_0);
        let mut x_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while x_1 < width {
            *(*sdf)
                .data
                .offset((y_1 * width) as isize)
                .offset(x_1 as isize) = *d_0.offset(x_1 as isize);
            *(*cf)
                .data
                .offset((2 as ::core::ffi::c_int * (y_1 * width + x_1)) as isize)
                .offset(0 as ::core::ffi::c_int as isize) =
                *pl2_0.offset((2 as ::core::ffi::c_int * x_1) as isize);
            *(*cf)
                .data
                .offset((2 as ::core::ffi::c_int * (y_1 * width + x_1)) as isize)
                .offset(1 as ::core::ffi::c_int as isize) =
                *pl2_0.offset((2 as ::core::ffi::c_int * x_1 + 1 as ::core::ffi::c_int) as isize);
            x_1 += 1;
        }
        free(pl1_0 as *mut ::core::ffi::c_void);
        free(pl2_0 as *mut ::core::ffi::c_void);
        y_1 += 1;
    }
    free(ff as *mut ::core::ffi::c_void);
    free(dd as *mut ::core::ffi::c_void);
    free(zz as *mut ::core::ffi::c_void);
    free(ww as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn heman_distance_create_sdf(mut src: *mut heman_image) -> *mut heman_image {
    let mut positive: *mut heman_image =
        heman_image_create((*src).width, (*src).height, 1 as ::core::ffi::c_int);
    let mut negative: *mut heman_image =
        heman_image_create((*src).width, (*src).height, 1 as ::core::ffi::c_int);
    let mut size: ::core::ffi::c_int = (*src).height * (*src).width;
    let mut pptr: *mut ::core::ffi::c_float = (*positive).data;
    let mut nptr: *mut ::core::ffi::c_float = (*negative).data;
    let mut sptr: *mut ::core::ffi::c_float = (*src).data;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < size {
        let fresh0 = pptr;
        pptr = pptr.offset(1);
        *fresh0 = if *sptr != 0. {
            INF
        } else {
            0 as ::core::ffi::c_int as ::core::ffi::c_float
        };
        let fresh1 = nptr;
        nptr = nptr.offset(1);
        *fresh1 = if *sptr != 0. {
            0 as ::core::ffi::c_int as ::core::ffi::c_float
        } else {
            INF
        };
        i += 1;
        sptr = sptr.offset(1);
    }
    transform_to_distance(positive);
    transform_to_distance(negative);
    let mut inv: ::core::ffi::c_float = 1.0f32 / (*src).width as ::core::ffi::c_float;
    pptr = (*positive).data;
    nptr = (*negative).data;
    let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_0 < size {
        *pptr = ((sqrt(*pptr as ::core::ffi::c_double) - sqrt(*nptr as ::core::ffi::c_double))
            * inv as ::core::ffi::c_double) as ::core::ffi::c_float;
        i_0 += 1;
        pptr = pptr.offset(1);
        nptr = nptr.offset(1);
    }
    heman_image_destroy(negative);
    return positive;
}
#[no_mangle]
pub unsafe extern "C" fn heman_distance_create_df(mut src: *mut heman_image) -> *mut heman_image {
    let mut positive: *mut heman_image =
        heman_image_create((*src).width, (*src).height, 1 as ::core::ffi::c_int);
    let mut size: ::core::ffi::c_int = (*src).height * (*src).width;
    let mut pptr: *mut ::core::ffi::c_float = (*positive).data;
    let mut sptr: *mut ::core::ffi::c_float = (*src).data;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < size {
        let fresh2 = pptr;
        pptr = pptr.offset(1);
        *fresh2 = if *sptr != 0. {
            0 as ::core::ffi::c_int as ::core::ffi::c_float
        } else {
            INF
        };
        i += 1;
        sptr = sptr.offset(1);
    }
    transform_to_distance(positive);
    let mut inv: ::core::ffi::c_float = 1.0f32 / (*src).width as ::core::ffi::c_float;
    pptr = (*positive).data;
    let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_0 < size {
        *pptr = (sqrt(*pptr as ::core::ffi::c_double) * inv as ::core::ffi::c_double)
            as ::core::ffi::c_float;
        i_0 += 1;
        pptr = pptr.offset(1);
    }
    return positive;
}
#[no_mangle]
pub unsafe extern "C" fn heman_distance_identity_cpcf(
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
) -> *mut heman_image {
    let mut retval: *mut heman_image = heman_image_create(width, height, 2 as ::core::ffi::c_int);
    let mut cdata: *mut ::core::ffi::c_float = (*retval).data;
    let mut y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while y < height {
        let mut x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while x < width {
            let fresh5 = cdata;
            cdata = cdata.offset(1);
            *fresh5 = x as ::core::ffi::c_float;
            let fresh6 = cdata;
            cdata = cdata.offset(1);
            *fresh6 = y as ::core::ffi::c_float;
            x += 1;
        }
        y += 1;
    }
    return retval;
}
#[no_mangle]
pub unsafe extern "C" fn heman_distance_create_cpcf(mut src: *mut heman_image) -> *mut heman_image {
    let mut negative: *mut heman_image =
        heman_image_create((*src).width, (*src).height, 1 as ::core::ffi::c_int);
    let mut size: ::core::ffi::c_int = (*src).height * (*src).width;
    let mut nptr: *mut ::core::ffi::c_float = (*negative).data;
    let mut sptr: *mut ::core::ffi::c_float = (*src).data;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < size {
        let mut val: ::core::ffi::c_float = 0 as ::core::ffi::c_int as ::core::ffi::c_float;
        let mut b: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while b < (*src).nbands {
            let fresh3 = sptr;
            sptr = sptr.offset(1);
            val += *fresh3;
            b += 1;
        }
        let fresh4 = nptr;
        nptr = nptr.offset(1);
        *fresh4 = if val != 0. {
            0 as ::core::ffi::c_int as ::core::ffi::c_float
        } else {
            INF
        };
        i += 1;
    }
    let mut coordfield: *mut heman_image =
        heman_distance_identity_cpcf((*src).width, (*src).height);
    transform_to_coordfield(negative, coordfield);
    heman_image_destroy(negative);
    return coordfield;
}
#[no_mangle]
pub unsafe extern "C" fn heman_distance_from_cpcf(mut cf: *mut heman_image) -> *mut heman_image {
    let mut udf: *mut heman_image =
        heman_image_create((*cf).width, (*cf).height, 1 as ::core::ffi::c_int);
    let mut dptr: *mut ::core::ffi::c_float = (*udf).data;
    let mut sptr: *mut ::core::ffi::c_float = (*cf).data;
    let mut scale: ::core::ffi::c_float = (1.0f64
        / sqrt(((*cf).width * (*cf).width + (*cf).height * (*cf).height) as ::core::ffi::c_double))
        as ::core::ffi::c_float;
    let mut y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while y < (*cf).height {
        let mut x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while x < (*cf).width {
            let fresh7 = sptr;
            sptr = sptr.offset(1);
            let mut u: ::core::ffi::c_float = *fresh7;
            let fresh8 = sptr;
            sptr = sptr.offset(1);
            let mut v: ::core::ffi::c_float = *fresh8;
            let mut dist: ::core::ffi::c_float = (sqrt(
                ((u - x as ::core::ffi::c_float) * (u - x as ::core::ffi::c_float)
                    + (v - y as ::core::ffi::c_float) * (v - y as ::core::ffi::c_float))
                    as ::core::ffi::c_double,
            ) * scale as ::core::ffi::c_double)
                as ::core::ffi::c_float;
            let fresh9 = dptr;
            dptr = dptr.offset(1);
            *fresh9 = dist;
            x += 1;
        }
        y += 1;
    }
    return udf;
}
