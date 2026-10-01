extern "C" {
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
}
pub type size_t = usize;
#[no_mangle]
pub unsafe extern "C" fn generate_gaussian_row(
    mut target: *mut ::core::ffi::c_int,
    mut fwidth: ::core::ffi::c_int,
) {
    let mut nbytes: ::core::ffi::c_int = (fwidth as usize)
        .wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as usize)
        as ::core::ffi::c_int;
    let mut tmp: *mut ::core::ffi::c_int = malloc(nbytes as size_t) as *mut ::core::ffi::c_int;
    let ref mut fresh0 = *tmp.offset(0 as ::core::ffi::c_int as isize);
    *fresh0 = 1 as ::core::ffi::c_int;
    *target.offset(0 as ::core::ffi::c_int as isize) = *fresh0;
    let mut col: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    while col < fwidth {
        *target.offset(col as isize) = 0 as ::core::ffi::c_int;
        *tmp.offset(col as isize) = 0 as ::core::ffi::c_int;
        col += 1;
    }
    let mut row: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    while row < fwidth {
        let mut col_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
        while col_0 <= row {
            *target.offset(col_0 as isize) = *tmp.offset(col_0 as isize)
                + *tmp.offset((col_0 - 1 as ::core::ffi::c_int) as isize);
            col_0 += 1;
        }
        let mut col_1: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
        while col_1 <= row {
            *tmp.offset(col_1 as isize) = *target.offset(col_1 as isize);
            col_1 += 1;
        }
        row += 1;
    }
    free(tmp as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn generate_gaussian_splat(
    mut target: *mut ::core::ffi::c_float,
    mut fwidth: ::core::ffi::c_int,
) {
    let mut gaussian_row: *mut ::core::ffi::c_int = malloc(
        (fwidth as size_t).wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as size_t),
    ) as *mut ::core::ffi::c_int;
    generate_gaussian_row(gaussian_row, fwidth);
    let mut shift: ::core::ffi::c_int =
        (1 as ::core::ffi::c_int) << fwidth - 1 as ::core::ffi::c_int;
    let mut scale: ::core::ffi::c_float =
        (1.0f64 / (shift * shift) as ::core::ffi::c_double) as ::core::ffi::c_float;
    let mut gptr: *mut ::core::ffi::c_float = target;
    let mut j: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while j < fwidth {
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < fwidth {
            let fresh1 = gptr;
            gptr = gptr.offset(1);
            *fresh1 = (*gaussian_row.offset(i as isize) * *gaussian_row.offset(j as isize))
                as ::core::ffi::c_float
                * scale;
            i += 1;
        }
        j += 1;
    }
    free(gaussian_row as *mut ::core::ffi::c_void);
}
