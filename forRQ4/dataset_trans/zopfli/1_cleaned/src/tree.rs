use core::ffi::*;
pub use crate::src::ffi::*;
pub use crate::src::c_types::*;
extern "C" {
    fn ZopfliLengthLimitedCodeLengths(
        frequencies: *const size_t,
        n: c_int,
        maxbits: c_int,
        bitlengths: *mut c_uint,
    ) -> c_int;
}

#[no_mangle]
pub unsafe extern "C" fn ZopfliLengthsToSymbols(
    mut lengths: *const c_uint,
    mut n: size_t,
    mut maxbits: c_uint,
    mut symbols: *mut c_uint,
) {
    let mut bl_count: *mut size_t = malloc(
        (::core::mem::size_of::<size_t>() as size_t)
            .wrapping_mul(maxbits.wrapping_add(1 as c_uint) as size_t),
    ) as *mut size_t;
    let mut next_code: *mut size_t = malloc(
        (::core::mem::size_of::<size_t>() as size_t)
            .wrapping_mul(maxbits.wrapping_add(1 as c_uint) as size_t),
    ) as *mut size_t;
    let mut bits: c_uint = 0;
    let mut i: c_uint = 0;
    let mut code: c_uint = 0;
    i = 0 as c_uint;
    while (i as size_t) < n {
        *symbols.offset(i as isize) = 0 as c_uint;
        i = i.wrapping_add(1);
    }
    bits = 0 as c_uint;
    while bits <= maxbits {
        *bl_count.offset(bits as isize) = 0 as size_t;
        bits = bits.wrapping_add(1);
    }
    i = 0 as c_uint;
    while (i as size_t) < n {
        let ref mut fresh0 = *bl_count.offset(*lengths.offset(i as isize) as isize);
        *fresh0 = (*fresh0).wrapping_add(1);
        i = i.wrapping_add(1);
    }
    code = 0 as c_uint;
    *bl_count.offset(0 as c_int as isize) = 0 as size_t;
    bits = 1 as c_uint;
    while bits <= maxbits {
        code = ((code as size_t)
            .wrapping_add(*bl_count.offset(bits.wrapping_sub(1 as c_uint) as isize))
            << 1 as c_int) as c_uint;
        *next_code.offset(bits as isize) = code as size_t;
        bits = bits.wrapping_add(1);
    }
    i = 0 as c_uint;
    while (i as size_t) < n {
        let mut len: c_uint = *lengths.offset(i as isize);
        if len != 0 as c_uint {
            *symbols.offset(i as isize) = *next_code.offset(len as isize) as c_uint;
            let ref mut fresh1 = *next_code.offset(len as isize);
            *fresh1 = (*fresh1).wrapping_add(1);
        }
        i = i.wrapping_add(1);
    }
    free(bl_count as *mut c_void);
    free(next_code as *mut c_void);
}
#[no_mangle]
pub unsafe extern "C" fn ZopfliCalculateEntropy(
    mut count: *const size_t,
    mut n: size_t,
    mut bitlengths: *mut c_double,
) {
    static mut kInvLog2: c_double = 1.4426950408889f64;
    let mut sum: c_uint = 0 as c_uint;
    let mut i: c_uint = 0;
    let mut log2sum: c_double = 0.;
    i = 0 as c_uint;
    while (i as size_t) < n {
        sum = (sum as size_t).wrapping_add(*count.offset(i as isize)) as c_uint
            as c_uint;
        i = i.wrapping_add(1);
    }
    log2sum = (if sum == 0 as c_uint {
        log(n as c_double)
    } else {
        log(sum as c_double)
    }) * kInvLog2;
    i = 0 as c_uint;
    while (i as size_t) < n {
        if *count.offset(i as isize) == 0 as size_t {
            *bitlengths.offset(i as isize) = log2sum;
        } else {
            *bitlengths.offset(i as isize) =
                log2sum - log(*count.offset(i as isize) as c_double) * kInvLog2;
        }
        if *bitlengths.offset(i as isize) < 0 as c_int as c_double
            && *bitlengths.offset(i as isize) > -1e-5f64
        {
            *bitlengths.offset(i as isize) = 0 as c_int as c_double;
        }
        i = i.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn ZopfliCalculateBitLengths(
    mut count: *const size_t,
    mut n: size_t,
    mut maxbits: c_int,
    mut bitlengths: *mut c_uint,
) {
    let mut error: c_int =
        ZopfliLengthLimitedCodeLengths(count, n as c_int, maxbits, bitlengths);
}
