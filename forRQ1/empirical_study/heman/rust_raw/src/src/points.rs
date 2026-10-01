extern "C" {
    fn heman_image_create(
        width: ::core::ffi::c_int,
        height: ::core::ffi::c_int,
        nbands: ::core::ffi::c_int,
    ) -> *mut heman_image;
    fn heman_image_sample(
        _: *mut heman_image,
        u: ::core::ffi::c_float,
        v: ::core::ffi::c_float,
        result: *mut ::core::ffi::c_float,
    );
    fn rand() -> ::core::ffi::c_int;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn sqrt(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn ceil(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn sqrtf(__x: ::core::ffi::c_float) -> ::core::ffi::c_float;
    fn kmVec2LengthSq(pIn: *const kmVec2) -> ::core::ffi::c_float;
    fn kmVec2Add(pOut: *mut kmVec2, pV1: *const kmVec2, pV2: *const kmVec2) -> *mut kmVec2;
    fn kmVec2Subtract(pOut: *mut kmVec2, pV1: *const kmVec2, pV2: *const kmVec2) -> *mut kmVec2;
    fn kmVec2Scale(pOut: *mut kmVec2, pIn: *const kmVec2, s: ::core::ffi::c_float) -> *mut kmVec2;
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
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct kmVec2 {
    pub x: ::core::ffi::c_float,
    pub y: ::core::ffi::c_float,
}
pub const RAND_MAX: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
pub const UINT_MAX: ::core::ffi::c_uint = (__INT_MAX__ as ::core::ffi::c_uint)
    .wrapping_mul(2 as ::core::ffi::c_uint)
    .wrapping_add(1 as ::core::ffi::c_uint);
#[no_mangle]
pub unsafe extern "C" fn randhash(mut seed: ::core::ffi::c_uint) -> ::core::ffi::c_uint {
    let mut i: ::core::ffi::c_uint =
        (seed ^ 12345391 as ::core::ffi::c_uint).wrapping_mul(2654435769 as ::core::ffi::c_uint);
    i ^= i << 6 as ::core::ffi::c_int ^ i >> 26 as ::core::ffi::c_int;
    i = i.wrapping_mul(2654435769 as ::core::ffi::c_uint);
    i = i.wrapping_add(i << 5 as ::core::ffi::c_int ^ i >> 12 as ::core::ffi::c_int);
    return i;
}
#[no_mangle]
pub unsafe extern "C" fn randhashf(
    mut seed: ::core::ffi::c_uint,
    mut a: ::core::ffi::c_float,
    mut b: ::core::ffi::c_float,
) -> ::core::ffi::c_float {
    return (b - a) * randhash(seed) as ::core::ffi::c_float / UINT_MAX as ::core::ffi::c_float + a;
}
#[no_mangle]
pub unsafe extern "C" fn heman_points_create(
    mut xy: *mut ::core::ffi::c_float,
    mut npoints: ::core::ffi::c_int,
    mut nbands: ::core::ffi::c_int,
) -> *mut heman_image {
    let mut img: *mut heman_points =
        malloc(::core::mem::size_of::<heman_image>() as size_t) as *mut heman_points;
    (*img).width = npoints;
    (*img).height = 1 as ::core::ffi::c_int;
    (*img).nbands = nbands;
    let mut nbytes: ::core::ffi::c_int = (::core::mem::size_of::<::core::ffi::c_float>() as usize)
        .wrapping_mul(npoints as usize)
        .wrapping_mul(nbands as usize)
        as ::core::ffi::c_int;
    (*img).data = malloc(nbytes as size_t) as *mut ::core::ffi::c_float;
    memcpy(
        (*img).data as *mut ::core::ffi::c_void,
        xy as *const ::core::ffi::c_void,
        nbytes as size_t,
    );
    return img as *mut heman_image;
}
#[no_mangle]
pub unsafe extern "C" fn heman_points_destroy(mut img: *mut heman_points) {
    free((*img).data as *mut ::core::ffi::c_void);
    free(img as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn heman_points_from_grid(
    mut width: ::core::ffi::c_float,
    mut height: ::core::ffi::c_float,
    mut cellsize: ::core::ffi::c_float,
    mut jitter: ::core::ffi::c_float,
) -> *mut heman_points {
    let mut cols: ::core::ffi::c_int = (width / cellsize) as ::core::ffi::c_int;
    let mut rows: ::core::ffi::c_int = (height / cellsize) as ::core::ffi::c_int;
    let mut ncells: ::core::ffi::c_int = cols * rows;
    let mut result: *mut heman_points =
        heman_image_create(ncells, 1 as ::core::ffi::c_int, 2 as ::core::ffi::c_int)
            as *mut heman_points;
    let mut rscale: ::core::ffi::c_float = (2.0f64 * jitter as ::core::ffi::c_double
        / RAND_MAX as ::core::ffi::c_float as ::core::ffi::c_double)
        as ::core::ffi::c_float;
    let mut j: ::core::ffi::c_int = 0;
    j = 0 as ::core::ffi::c_int;
    while j < rows {
        let mut dst: *mut ::core::ffi::c_float = (*result)
            .data
            .offset((j * cols * 2 as ::core::ffi::c_int) as isize);
        let mut y: ::core::ffi::c_float = (cellsize as ::core::ffi::c_double * 0.5f64
            + (cellsize * j as ::core::ffi::c_float) as ::core::ffi::c_double)
            as ::core::ffi::c_float;
        let mut x: ::core::ffi::c_float =
            (cellsize as ::core::ffi::c_double * 0.5f64) as ::core::ffi::c_float;
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < cols {
            let mut rx: ::core::ffi::c_float = rand() as ::core::ffi::c_float * rscale - jitter;
            let mut ry: ::core::ffi::c_float = rand() as ::core::ffi::c_float * rscale - jitter;
            let fresh0 = dst;
            dst = dst.offset(1);
            *fresh0 = x + rx;
            let fresh1 = dst;
            dst = dst.offset(1);
            *fresh1 = y + ry;
            x += cellsize;
            i += 1;
        }
        j += 1;
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn sample_annulus(
    mut radius: ::core::ffi::c_float,
    mut center: kmVec2,
    mut seedptr: *mut ::core::ffi::c_uint,
) -> kmVec2 {
    let mut seed: ::core::ffi::c_uint = *seedptr;
    let mut r: kmVec2 = kmVec2 { x: 0., y: 0. };
    let mut rscale: ::core::ffi::c_float = 1.0f32 / UINT_MAX as ::core::ffi::c_float;
    loop {
        let fresh12 = seed;
        seed = seed.wrapping_add(1);
        r.x = 4 as ::core::ffi::c_int as ::core::ffi::c_float
            * rscale
            * randhash(fresh12) as ::core::ffi::c_float
            - 2 as ::core::ffi::c_int as ::core::ffi::c_float;
        let fresh13 = seed;
        seed = seed.wrapping_add(1);
        r.y = 4 as ::core::ffi::c_int as ::core::ffi::c_float
            * rscale
            * randhash(fresh13) as ::core::ffi::c_float
            - 2 as ::core::ffi::c_int as ::core::ffi::c_float;
        let mut r2: ::core::ffi::c_float = kmVec2LengthSq(&raw mut r);
        if r2 > 1 as ::core::ffi::c_int as ::core::ffi::c_float
            && r2 <= 4 as ::core::ffi::c_int as ::core::ffi::c_float
        {
            break;
        }
    }
    *seedptr = seed;
    kmVec2Scale(&raw mut r, &raw mut r, radius);
    kmVec2Add(&raw mut r, &raw mut r, &raw mut center);
    return r;
}
#[no_mangle]
pub unsafe extern "C" fn heman_points_from_poisson(
    mut width: ::core::ffi::c_float,
    mut height: ::core::ffi::c_float,
    mut radius: ::core::ffi::c_float,
) -> *mut heman_points {
    let mut maxattempts: ::core::ffi::c_int = 30 as ::core::ffi::c_int;
    let mut rscale: ::core::ffi::c_float = 1.0f32 / UINT_MAX as ::core::ffi::c_float;
    let mut seed: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    let mut rvec: kmVec2 = kmVec2 { x: 0., y: 0. };
    rvec.y = radius;
    rvec.x = rvec.y;
    let mut r2: ::core::ffi::c_float = radius * radius;
    let mut cellsize: ::core::ffi::c_float =
        radius / sqrtf(2 as ::core::ffi::c_int as ::core::ffi::c_float);
    let mut invcell: ::core::ffi::c_float = 1.0f32 / cellsize;
    let mut ncols: ::core::ffi::c_int =
        ceil((width * invcell) as ::core::ffi::c_double) as ::core::ffi::c_int;
    let mut nrows: ::core::ffi::c_int =
        ceil((height * invcell) as ::core::ffi::c_double) as ::core::ffi::c_int;
    let mut maxcol: ::core::ffi::c_int = ncols - 1 as ::core::ffi::c_int;
    let mut maxrow: ::core::ffi::c_int = nrows - 1 as ::core::ffi::c_int;
    let mut ncells: ::core::ffi::c_int = ncols * nrows;
    let mut grid: *mut ::core::ffi::c_int = malloc(
        (ncells as size_t).wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as size_t),
    ) as *mut ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < ncells {
        *grid.offset(i as isize) = -(1 as ::core::ffi::c_int);
        i += 1;
    }
    let mut actives: *mut ::core::ffi::c_int = malloc(
        (ncells as size_t).wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as size_t),
    ) as *mut ::core::ffi::c_int;
    let mut nactives: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut result: *mut heman_points =
        heman_image_create(ncells, 1 as ::core::ffi::c_int, 2 as ::core::ffi::c_int)
            as *mut heman_points;
    let mut samples: *mut kmVec2 = (*result).data as *mut kmVec2;
    let mut nsamples: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut pt: kmVec2 = kmVec2 { x: 0., y: 0. };
    let fresh2 = seed;
    seed = seed.wrapping_add(1);
    pt.x = width * randhash(fresh2) as ::core::ffi::c_float * rscale;
    let fresh3 = seed;
    seed = seed.wrapping_add(1);
    pt.y = height * randhash(fresh3) as ::core::ffi::c_float * rscale;
    let fresh4 = nactives;
    nactives = nactives + 1;
    let ref mut fresh5 = *actives.offset(fresh4 as isize);
    *fresh5 = nsamples;
    *grid.offset(
        ((pt.x * invcell) as ::core::ffi::c_int + ncols * (pt.y * invcell) as ::core::ffi::c_int)
            as isize,
    ) = *fresh5;
    let fresh6 = nsamples;
    nsamples = nsamples + 1;
    *samples.offset(fresh6 as isize) = pt;
    while nsamples < ncells {
        let fresh7 = seed;
        seed = seed.wrapping_add(1);
        let mut aindex: ::core::ffi::c_int = (if randhashf(
            fresh7,
            0 as ::core::ffi::c_int as ::core::ffi::c_float,
            nactives as ::core::ffi::c_float,
        ) > (nactives - 1 as ::core::ffi::c_int)
            as ::core::ffi::c_float
        {
            (nactives - 1 as ::core::ffi::c_int) as ::core::ffi::c_float
        } else {
            let fresh8 = seed;
            seed = seed.wrapping_add(1);
            randhashf(
                fresh8,
                0 as ::core::ffi::c_int as ::core::ffi::c_float,
                nactives as ::core::ffi::c_float,
            )
        }) as ::core::ffi::c_int;
        let mut sindex: ::core::ffi::c_int = *actives.offset(aindex as isize);
        let mut found: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut j: kmVec2 = kmVec2 { x: 0., y: 0. };
        let mut minj: kmVec2 = kmVec2 { x: 0., y: 0. };
        let mut maxj: kmVec2 = kmVec2 { x: 0., y: 0. };
        let mut delta: kmVec2 = kmVec2 { x: 0., y: 0. };
        let mut attempt: ::core::ffi::c_int = 0;
        attempt = 0 as ::core::ffi::c_int;
        while attempt < maxattempts && found == 0 {
            pt = sample_annulus(radius, *samples.offset(sindex as isize), &raw mut seed);
            if !(pt.x < 0 as ::core::ffi::c_int as ::core::ffi::c_float
                || pt.x >= width
                || pt.y < 0 as ::core::ffi::c_int as ::core::ffi::c_float
                || pt.y >= height)
            {
                maxj = pt;
                minj = maxj;
                kmVec2Add(&raw mut maxj, &raw mut maxj, &raw mut rvec);
                kmVec2Subtract(&raw mut minj, &raw mut minj, &raw mut rvec);
                kmVec2Scale(&raw mut minj, &raw mut minj, invcell);
                kmVec2Scale(&raw mut maxj, &raw mut maxj, invcell);
                minj.x = (if 0 as ::core::ffi::c_int
                    > (if maxcol > minj.x as ::core::ffi::c_int {
                        minj.x as ::core::ffi::c_int
                    } else {
                        maxcol
                    }) {
                    0 as ::core::ffi::c_int
                } else if maxcol > minj.x as ::core::ffi::c_int {
                    minj.x as ::core::ffi::c_int
                } else {
                    maxcol
                }) as ::core::ffi::c_float;
                maxj.x = (if 0 as ::core::ffi::c_int
                    > (if maxcol > maxj.x as ::core::ffi::c_int {
                        maxj.x as ::core::ffi::c_int
                    } else {
                        maxcol
                    }) {
                    0 as ::core::ffi::c_int
                } else if maxcol > maxj.x as ::core::ffi::c_int {
                    maxj.x as ::core::ffi::c_int
                } else {
                    maxcol
                }) as ::core::ffi::c_float;
                minj.y = (if 0 as ::core::ffi::c_int
                    > (if maxrow > minj.y as ::core::ffi::c_int {
                        minj.y as ::core::ffi::c_int
                    } else {
                        maxrow
                    }) {
                    0 as ::core::ffi::c_int
                } else if maxrow > minj.y as ::core::ffi::c_int {
                    minj.y as ::core::ffi::c_int
                } else {
                    maxrow
                }) as ::core::ffi::c_float;
                maxj.y = (if 0 as ::core::ffi::c_int
                    > (if maxrow > maxj.y as ::core::ffi::c_int {
                        maxj.y as ::core::ffi::c_int
                    } else {
                        maxrow
                    }) {
                    0 as ::core::ffi::c_int
                } else if maxrow > maxj.y as ::core::ffi::c_int {
                    maxj.y as ::core::ffi::c_int
                } else {
                    maxrow
                }) as ::core::ffi::c_float;
                let mut reject: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                j.y = minj.y;
                while j.y <= maxj.y && reject == 0 {
                    j.x = minj.x;
                    while j.x <= maxj.x && reject == 0 {
                        let mut entry: ::core::ffi::c_int = *grid.offset(
                            (j.y as ::core::ffi::c_int * ncols + j.x as ::core::ffi::c_int)
                                as isize,
                        );
                        if entry > -(1 as ::core::ffi::c_int) && entry != sindex {
                            kmVec2Subtract(
                                &raw mut delta,
                                samples.offset(entry as isize) as *mut kmVec2,
                                &raw mut pt,
                            );
                            if kmVec2LengthSq(&raw mut delta) < r2 {
                                reject = 1 as ::core::ffi::c_int;
                            }
                        }
                        j.x += 1.;
                    }
                    j.y += 1.;
                }
                if !(reject != 0) {
                    found = 1 as ::core::ffi::c_int;
                }
            }
            attempt += 1;
        }
        if found != 0 {
            let fresh9 = nactives;
            nactives = nactives + 1;
            let ref mut fresh10 = *actives.offset(fresh9 as isize);
            *fresh10 = nsamples;
            *grid.offset(
                ((pt.x * invcell) as ::core::ffi::c_int
                    + ncols * (pt.y * invcell) as ::core::ffi::c_int) as isize,
            ) = *fresh10;
            let fresh11 = nsamples;
            nsamples = nsamples + 1;
            *samples.offset(fresh11 as isize) = pt;
        } else {
            nactives -= 1;
            if nactives <= 0 as ::core::ffi::c_int {
                break;
            }
            *actives.offset(aindex as isize) = *actives.offset(nactives as isize);
        }
    }
    (*result).width = nsamples;
    free(grid as *mut ::core::ffi::c_void);
    free(actives as *mut ::core::ffi::c_void);
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn heman_points_from_density(
    mut density: *mut heman_image,
    mut minradius: ::core::ffi::c_float,
    mut maxradius: ::core::ffi::c_float,
) -> *mut heman_points {
    let mut width: ::core::ffi::c_float = 1 as ::core::ffi::c_int as ::core::ffi::c_float;
    let mut height: ::core::ffi::c_float = 1 as ::core::ffi::c_int as ::core::ffi::c_float;
    let mut maxattempts: ::core::ffi::c_int = 30 as ::core::ffi::c_int;
    let mut rscale: ::core::ffi::c_float = 1.0f32 / UINT_MAX as ::core::ffi::c_float;
    let mut seed: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    let mut rvec: kmVec2 = kmVec2 { x: 0., y: 0. };
    rvec.y = maxradius;
    rvec.x = rvec.y;
    let mut gindex: ::core::ffi::c_int = 0;
    let mut cellsize: ::core::ffi::c_float =
        maxradius / sqrtf(2 as ::core::ffi::c_int as ::core::ffi::c_float);
    let mut invcell: ::core::ffi::c_float = 1.0f32 / cellsize;
    let mut ncols: ::core::ffi::c_int =
        ceil((width * invcell) as ::core::ffi::c_double) as ::core::ffi::c_int;
    let mut nrows: ::core::ffi::c_int =
        ceil((height * invcell) as ::core::ffi::c_double) as ::core::ffi::c_int;
    let mut maxcol: ::core::ffi::c_int = ncols - 1 as ::core::ffi::c_int;
    let mut maxrow: ::core::ffi::c_int = nrows - 1 as ::core::ffi::c_int;
    let mut ncells: ::core::ffi::c_int = ncols * nrows;
    let mut ntexels: ::core::ffi::c_int =
        (cellsize * (*density).width as ::core::ffi::c_float) as ::core::ffi::c_int;
    let mut gcapacity: ::core::ffi::c_int = ntexels * ntexels;
    let mut grid: *mut ::core::ffi::c_int = malloc(
        (ncells as size_t)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as size_t)
            .wrapping_mul(gcapacity as size_t),
    ) as *mut ::core::ffi::c_int;
    let mut ngrid: *mut ::core::ffi::c_int = malloc(
        (ncells as size_t).wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as size_t),
    ) as *mut ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < ncells {
        *ngrid.offset(i as isize) = 0 as ::core::ffi::c_int;
        i += 1;
    }
    let mut actives: *mut ::core::ffi::c_int = malloc(
        (ncells as size_t).wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as size_t),
    ) as *mut ::core::ffi::c_int;
    let mut nactives: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut maxsamples: ::core::ffi::c_int = ncells * gcapacity;
    let mut result: *mut heman_points =
        heman_image_create(maxsamples, 1 as ::core::ffi::c_int, 2 as ::core::ffi::c_int)
            as *mut heman_points;
    let mut samples: *mut kmVec2 = (*result).data as *mut kmVec2;
    let mut nsamples: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut pt: kmVec2 = kmVec2 { x: 0., y: 0. };
    let fresh14 = seed;
    seed = seed.wrapping_add(1);
    pt.x = width * randhash(fresh14) as ::core::ffi::c_float * rscale;
    let fresh15 = seed;
    seed = seed.wrapping_add(1);
    pt.y = height * randhash(fresh15) as ::core::ffi::c_float * rscale;
    let fresh16 = nactives;
    nactives = nactives + 1;
    *actives.offset(fresh16 as isize) = nsamples;
    gindex =
        (pt.x * invcell) as ::core::ffi::c_int + ncols * (pt.y * invcell) as ::core::ffi::c_int;
    *grid.offset((gcapacity * gindex + *ngrid.offset(gindex as isize)) as isize) = nsamples;
    let ref mut fresh17 = *ngrid.offset(gindex as isize);
    *fresh17 += 1;
    let fresh18 = nsamples;
    nsamples = nsamples + 1;
    *samples.offset(fresh18 as isize) = pt;
    while nsamples < maxsamples {
        let fresh19 = seed;
        seed = seed.wrapping_add(1);
        let mut aindex: ::core::ffi::c_int = (if randhashf(
            fresh19,
            0 as ::core::ffi::c_int as ::core::ffi::c_float,
            nactives as ::core::ffi::c_float,
        ) > (nactives - 1 as ::core::ffi::c_int)
            as ::core::ffi::c_float
        {
            (nactives - 1 as ::core::ffi::c_int) as ::core::ffi::c_float
        } else {
            let fresh20 = seed;
            seed = seed.wrapping_add(1);
            randhashf(
                fresh20,
                0 as ::core::ffi::c_int as ::core::ffi::c_float,
                nactives as ::core::ffi::c_float,
            )
        }) as ::core::ffi::c_int;
        let mut sindex: ::core::ffi::c_int = *actives.offset(aindex as isize);
        let mut found: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut j: kmVec2 = kmVec2 { x: 0., y: 0. };
        let mut minj: kmVec2 = kmVec2 { x: 0., y: 0. };
        let mut maxj: kmVec2 = kmVec2 { x: 0., y: 0. };
        let mut delta: kmVec2 = kmVec2 { x: 0., y: 0. };
        let mut attempt: ::core::ffi::c_int = 0;
        attempt = 0 as ::core::ffi::c_int;
        while attempt < maxattempts && found == 0 {
            pt = sample_annulus(maxradius, *samples.offset(sindex as isize), &raw mut seed);
            if !(pt.x < 0 as ::core::ffi::c_int as ::core::ffi::c_float
                || pt.x >= width
                || pt.y < 0 as ::core::ffi::c_int as ::core::ffi::c_float
                || pt.y >= height)
            {
                maxj = pt;
                minj = maxj;
                kmVec2Add(&raw mut maxj, &raw mut maxj, &raw mut rvec);
                kmVec2Subtract(&raw mut minj, &raw mut minj, &raw mut rvec);
                kmVec2Scale(&raw mut minj, &raw mut minj, invcell);
                kmVec2Scale(&raw mut maxj, &raw mut maxj, invcell);
                minj.x = (if 0 as ::core::ffi::c_int
                    > (if maxcol > minj.x as ::core::ffi::c_int {
                        minj.x as ::core::ffi::c_int
                    } else {
                        maxcol
                    }) {
                    0 as ::core::ffi::c_int
                } else if maxcol > minj.x as ::core::ffi::c_int {
                    minj.x as ::core::ffi::c_int
                } else {
                    maxcol
                }) as ::core::ffi::c_float;
                maxj.x = (if 0 as ::core::ffi::c_int
                    > (if maxcol > maxj.x as ::core::ffi::c_int {
                        maxj.x as ::core::ffi::c_int
                    } else {
                        maxcol
                    }) {
                    0 as ::core::ffi::c_int
                } else if maxcol > maxj.x as ::core::ffi::c_int {
                    maxj.x as ::core::ffi::c_int
                } else {
                    maxcol
                }) as ::core::ffi::c_float;
                minj.y = (if 0 as ::core::ffi::c_int
                    > (if maxrow > minj.y as ::core::ffi::c_int {
                        minj.y as ::core::ffi::c_int
                    } else {
                        maxrow
                    }) {
                    0 as ::core::ffi::c_int
                } else if maxrow > minj.y as ::core::ffi::c_int {
                    minj.y as ::core::ffi::c_int
                } else {
                    maxrow
                }) as ::core::ffi::c_float;
                maxj.y = (if 0 as ::core::ffi::c_int
                    > (if maxrow > maxj.y as ::core::ffi::c_int {
                        maxj.y as ::core::ffi::c_int
                    } else {
                        maxrow
                    }) {
                    0 as ::core::ffi::c_int
                } else if maxrow > maxj.y as ::core::ffi::c_int {
                    maxj.y as ::core::ffi::c_int
                } else {
                    maxrow
                }) as ::core::ffi::c_float;
                let mut reject: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                let mut densityval: ::core::ffi::c_float = 0.;
                heman_image_sample(density, pt.x, pt.y, &raw mut densityval);
                densityval = sqrt(densityval as ::core::ffi::c_double) as ::core::ffi::c_float;
                let mut mindist: ::core::ffi::c_float =
                    maxradius - densityval * (maxradius - minradius);
                let mut r2: ::core::ffi::c_float = mindist * mindist;
                j.y = minj.y;
                while j.y <= maxj.y && reject == 0 {
                    j.x = minj.x;
                    while j.x <= maxj.x && reject == 0 {
                        let mut g: ::core::ffi::c_int = (j.y as ::core::ffi::c_int * ncols
                            + j.x as ::core::ffi::c_int)
                            * gcapacity;
                        while g
                            < (j.y as ::core::ffi::c_int * ncols + j.x as ::core::ffi::c_int)
                                * gcapacity
                                + *ngrid.offset(
                                    (j.y as ::core::ffi::c_int * ncols + j.x as ::core::ffi::c_int)
                                        as isize,
                                )
                        {
                            let mut entry: ::core::ffi::c_int = *grid.offset(g as isize);
                            if entry != sindex {
                                kmVec2Subtract(
                                    &raw mut delta,
                                    samples.offset(entry as isize) as *mut kmVec2,
                                    &raw mut pt,
                                );
                                if kmVec2LengthSq(&raw mut delta) < r2 {
                                    reject = 1 as ::core::ffi::c_int;
                                }
                            }
                            g += 1;
                        }
                        j.x += 1.;
                    }
                    j.y += 1.;
                }
                if !(reject != 0) {
                    found = 1 as ::core::ffi::c_int;
                }
            }
            attempt += 1;
        }
        if found != 0
            && *ngrid.offset(
                ((pt.x * invcell) as ::core::ffi::c_int
                    + ncols * (pt.y * invcell) as ::core::ffi::c_int) as isize,
            ) >= gcapacity
        {
            found = 0 as ::core::ffi::c_int;
        }
        if found != 0 {
            let fresh21 = nactives;
            nactives = nactives + 1;
            *actives.offset(fresh21 as isize) = nsamples;
            gindex = (pt.x * invcell) as ::core::ffi::c_int
                + ncols * (pt.y * invcell) as ::core::ffi::c_int;
            *grid.offset((gcapacity * gindex + *ngrid.offset(gindex as isize)) as isize) = nsamples;
            let ref mut fresh22 = *ngrid.offset(gindex as isize);
            *fresh22 += 1;
            let fresh23 = nsamples;
            nsamples = nsamples + 1;
            *samples.offset(fresh23 as isize) = pt;
        } else {
            nactives -= 1;
            if nactives <= 0 as ::core::ffi::c_int {
                break;
            }
            *actives.offset(aindex as isize) = *actives.offset(nactives as isize);
        }
    }
    (*result).width = nsamples;
    free(grid as *mut ::core::ffi::c_void);
    free(ngrid as *mut ::core::ffi::c_void);
    free(actives as *mut ::core::ffi::c_void);
    return result;
}
pub const __INT_MAX__: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
