use core::ffi::*;
pub use crate::src::ffi::*;
pub use crate::src::c_consts::*;
pub use crate::src::c_types::*;
extern "C" {
    fn __ctype_tolower_loc() -> *mut *const __int32_t;
    fn __ctype_toupper_loc() -> *mut *const __int32_t;
}
pub type __int32_t = i32;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct match_struct {
    pub needle_len: c_int,
    pub haystack_len: c_int,
    pub lower_needle: [c_char; 1024],
    pub lower_haystack: [c_char; 1024],
    pub match_bonus: [score_t; 1024],
}
#[inline]
fn tolower(mut __c: c_int) -> c_int { unsafe {
    return if __c >= -(128 as c_int) && __c < 256 as c_int {
        *(*__ctype_tolower_loc()).offset(__c as isize) as c_int
    } else {
        __c
    };
} }
#[inline]
fn toupper(mut __c: c_int) -> c_int { unsafe {
    return if __c >= -(128 as c_int) && __c < 256 as c_int {
        *(*__ctype_toupper_loc()).offset(__c as isize) as c_int
    } else {
        __c
    };
} }

#[inline]
pub unsafe fn strcasechr(
    mut s: *const c_char,
    mut c: c_char,
) -> *mut c_char {
    let accept: [c_char; 3] = [
        c,
        ({
            let mut __res: c_int = 0;
            if ::core::mem::size_of::<c_char>() as usize > 1 as usize {
                if 0 != 0 {
                    let mut __c: c_int = c as c_int;
                    __res =
                        (if __c < -(128 as c_int) || __c > 255 as c_int {
                            __c as __int32_t
                        } else {
                            *(*__ctype_toupper_loc()).offset(__c as isize)
                        }) as c_int;
                } else {
                    __res = toupper(c as c_int);
                }
            } else {
                __res = *(*__ctype_toupper_loc()).offset(c as c_int as isize)
                    as c_int;
            }
            __res
        }) as c_char,
        0 as c_int as c_char,
    ];
    return strpbrk(s, &raw const accept as *const c_char);
}
#[no_mangle]
pub unsafe extern "C" fn has_match(
    mut needle: *const c_char,
    mut haystack: *const c_char,
) -> c_int {
    while *needle != 0 {
        let fresh0 = needle;
        needle = needle.offset(1);
        let mut nch: c_char = *fresh0;
        haystack = strcasechr(haystack, nch);
        if haystack.is_null() {
            return 0 as c_int;
        }
        haystack = haystack.offset(1);
    }
    return 1 as c_int;
}
unsafe fn precompute_bonus(
    mut haystack: *const c_char,
    mut match_bonus: *mut score_t,
) {
    let mut last_ch: c_char = '/' as i32 as c_char;
    let mut i: c_int = 0 as c_int;
    while *haystack.offset(i as isize) != 0 {
        let mut ch: c_char = *haystack.offset(i as isize);
        *match_bonus.offset(i as isize) = bonus_states
            [bonus_index[ch as c_uchar as usize] as usize]
            [last_ch as c_uchar as usize];
        last_ch = ch;
        i += 1;
    }
}
unsafe fn setup_match_struct(
    mut match_1: *mut match_struct,
    mut needle: *const c_char,
    mut haystack: *const c_char,
) {
    (*match_1).needle_len = strlen(needle) as c_int;
    (*match_1).haystack_len = strlen(haystack) as c_int;
    if (*match_1).haystack_len > MATCH_MAX_LEN || (*match_1).needle_len > (*match_1).haystack_len {
        return;
    }
    let mut i: c_int = 0 as c_int;
    while i < (*match_1).needle_len {
        (*match_1).lower_needle[i as usize] = ({
            let mut __res: c_int = 0;
            if ::core::mem::size_of::<c_char>() as usize > 1 as usize {
                if 0 != 0 {
                    let mut __c: c_int =
                        *needle.offset(i as isize) as c_int;
                    __res =
                        (if __c < -(128 as c_int) || __c > 255 as c_int {
                            __c as __int32_t
                        } else {
                            *(*__ctype_tolower_loc()).offset(__c as isize)
                        }) as c_int;
                } else {
                    __res = tolower(*needle.offset(i as isize) as c_int);
                }
            } else {
                __res = *(*__ctype_tolower_loc())
                    .offset(*needle.offset(i as isize) as c_int as isize)
                    as c_int;
            }
            __res
        }) as c_char;
        i += 1;
    }
    let mut i_0: c_int = 0 as c_int;
    while i_0 < (*match_1).haystack_len {
        (*match_1).lower_haystack[i_0 as usize] = ({
            let mut __res: c_int = 0;
            if ::core::mem::size_of::<c_char>() as usize > 1 as usize {
                if 0 != 0 {
                    let mut __c: c_int =
                        *haystack.offset(i_0 as isize) as c_int;
                    __res =
                        (if __c < -(128 as c_int) || __c > 255 as c_int {
                            __c as __int32_t
                        } else {
                            *(*__ctype_tolower_loc()).offset(__c as isize)
                        }) as c_int;
                } else {
                    __res = tolower(*haystack.offset(i_0 as isize) as c_int);
                }
            } else {
                __res = *(*__ctype_tolower_loc())
                    .offset(*haystack.offset(i_0 as isize) as c_int as isize)
                    as c_int;
            }
            __res
        }) as c_char;
        i_0 += 1;
    }
    precompute_bonus(haystack, &raw mut (*match_1).match_bonus as *mut score_t);
}
#[inline]
unsafe fn match_row(
    mut match_1: *const match_struct,
    mut row: c_int,
    mut curr_D: *mut score_t,
    mut curr_M: *mut score_t,
    mut last_D: *const score_t,
    mut last_M: *const score_t,
) {
    let mut n: c_int = (*match_1).needle_len;
    let mut m: c_int = (*match_1).haystack_len;
    let mut i: c_int = row;
    let mut lower_needle: *const c_char =
        &raw const (*match_1).lower_needle as *const c_char;
    let mut lower_haystack: *const c_char =
        &raw const (*match_1).lower_haystack as *const c_char;
    let mut match_bonus: *const score_t = &raw const (*match_1).match_bonus as *const score_t;
    let mut prev_score: score_t = -::core::f32::INFINITY as score_t;
    let mut gap_score: score_t = if i == n - 1 as c_int {
        SCORE_GAP_TRAILING
    } else {
        SCORE_GAP_INNER
    };
    let mut prev_M: score_t = -::core::f32::INFINITY as score_t;
    let mut prev_D: score_t = -::core::f32::INFINITY as score_t;
    let mut j: c_int = 0 as c_int;
    while j < m {
        if *lower_needle.offset(i as isize) as c_int
            == *lower_haystack.offset(j as isize) as c_int
        {
            let mut score: score_t = -::core::f32::INFINITY as score_t;
            if i == 0 {
                score = j as score_t * SCORE_GAP_LEADING + *match_bonus.offset(j as isize);
            } else if j != 0 {
                score = (if prev_M + *match_bonus.offset(j as isize)
                    > prev_D as c_double + 1.0f64
                {
                    prev_M as c_double
                        + *match_bonus.offset(j as isize) as c_double
                } else {
                    prev_D as c_double + 1.0f64
                }) as score_t;
            }
            prev_D = *last_D.offset(j as isize);
            prev_M = *last_M.offset(j as isize);
            *curr_D.offset(j as isize) = score;
            prev_score = if score > prev_score + gap_score {
                score
            } else {
                prev_score + gap_score
            };
            *curr_M.offset(j as isize) = prev_score;
        } else {
            prev_D = *last_D.offset(j as isize);
            prev_M = *last_M.offset(j as isize);
            *curr_D.offset(j as isize) = -::core::f32::INFINITY as score_t;
            prev_score = prev_score + gap_score;
            *curr_M.offset(j as isize) = prev_score;
        }
        j += 1;
    }
}
#[export_name = "match"]
pub unsafe extern "C" fn match_0(
    mut needle: *const c_char,
    mut haystack: *const c_char,
) -> score_t {
    if *needle == 0 {
        return -::core::f32::INFINITY as score_t;
    }
    let mut match_1: match_struct = match_struct {
        needle_len: 0,
        haystack_len: 0,
        lower_needle: [0; 1024],
        lower_haystack: [0; 1024],
        match_bonus: [0.; 1024],
    };
    setup_match_struct(&raw mut match_1, needle, haystack);
    let mut n: c_int = match_1.needle_len;
    let mut m: c_int = match_1.haystack_len;
    if m > MATCH_MAX_LEN || n > m {
        return -::core::f32::INFINITY as score_t;
    } else if n == m {
        return ::core::f32::INFINITY as score_t;
    }
    let mut D: [score_t; 1024] = [0.; 1024];
    let mut M: [score_t; 1024] = [0.; 1024];
    let mut i: c_int = 0 as c_int;
    while i < n {
        match_row(
            &raw mut match_1,
            i,
            &raw mut D as *mut score_t,
            &raw mut M as *mut score_t,
            &raw mut D as *mut score_t,
            &raw mut M as *mut score_t,
        );
        i += 1;
    }
    return M[(m - 1 as c_int) as usize];
}
#[no_mangle]
pub unsafe extern "C" fn match_positions(
    mut needle: *const c_char,
    mut haystack: *const c_char,
    mut positions: *mut size_t,
) -> score_t {
    let needle_view: &c_char = unsafe { &*needle };
    if *needle_view == 0 {
        return -::core::f32::INFINITY as score_t;
    }
    let mut match_1: match_struct = match_struct {
        needle_len: 0,
        haystack_len: 0,
        lower_needle: [0; 1024],
        lower_haystack: [0; 1024],
        match_bonus: [0.; 1024],
    };
    setup_match_struct(&raw mut match_1, needle, haystack);
    let mut n: c_int = match_1.needle_len;
    let mut m: c_int = match_1.haystack_len;
    if m > MATCH_MAX_LEN || n > m {
        return -::core::f32::INFINITY as score_t;
    } else if n == m {
        if !positions.is_null() {
            let mut i: c_int = 0 as c_int;
            while i < n {
                *positions.offset(i as isize) = i as size_t;
                i += 1;
            }
        }
        return ::core::f32::INFINITY as score_t;
    }
    let mut D: *mut [score_t; 1024] = ::core::ptr::null_mut::<[score_t; 1024]>();
    let mut M: *mut [score_t; 1024] = ::core::ptr::null_mut::<[score_t; 1024]>();
    M = malloc(
        (::core::mem::size_of::<score_t>() as size_t)
            .wrapping_mul(MATCH_MAX_LEN as size_t)
            .wrapping_mul(n as size_t),
    ) as *mut [score_t; 1024];
    D = malloc(
        (::core::mem::size_of::<score_t>() as size_t)
            .wrapping_mul(MATCH_MAX_LEN as size_t)
            .wrapping_mul(n as size_t),
    ) as *mut [score_t; 1024];
    match_row(
        &raw mut match_1,
        0 as c_int,
        &raw mut *D.offset(0 as c_int as isize) as *mut score_t,
        &raw mut *M.offset(0 as c_int as isize) as *mut score_t,
        &raw mut *D.offset(0 as c_int as isize) as *mut score_t,
        &raw mut *M.offset(0 as c_int as isize) as *mut score_t,
    );
    let mut i_0: c_int = 1 as c_int;
    while i_0 < n {
        match_row(
            &raw mut match_1,
            i_0,
            &raw mut *D.offset(i_0 as isize) as *mut score_t,
            &raw mut *M.offset(i_0 as isize) as *mut score_t,
            &raw mut *D.offset((i_0 - 1 as c_int) as isize) as *mut score_t,
            &raw mut *M.offset((i_0 - 1 as c_int) as isize) as *mut score_t,
        );
        i_0 += 1;
    }
    if !positions.is_null() {
        let mut match_required: c_int = 0 as c_int;
        let mut i_1: c_int = n - 1 as c_int;
        let mut j: c_int = m - 1 as c_int;
        while i_1 >= 0 as c_int {
            while j >= 0 as c_int {
                if (*D.offset(i_1 as isize))[j as usize] != -::core::f32::INFINITY as score_t
                    && (match_required != 0
                        || (*D.offset(i_1 as isize))[j as usize]
                            == (*M.offset(i_1 as isize))[j as usize])
                {
                    match_required = (i_1 != 0
                        && j != 0
                        && (*M.offset(i_1 as isize))[j as usize]
                            == (*D.offset((i_1 - 1 as c_int) as isize))
                                [(j - 1 as c_int) as usize]
                                + SCORE_MATCH_CONSECUTIVE)
                        as c_int;
                    let fresh1 = j;
                    j = j - 1;
                    *positions.offset(i_1 as isize) = fresh1 as size_t;
                    break;
                } else {
                    j -= 1;
                }
            }
            i_1 -= 1;
        }
    }
    let mut result: score_t =
        (*M.offset((n - 1 as c_int) as isize))[(m - 1 as c_int) as usize];
    free(M as *mut c_void);
    free(D as *mut c_void);
    return result;
}
#[no_mangle]
pub static mut bonus_states: [[score_t; 256]; 3] = [
    [
        0 as c_int as score_t,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
    ],
    [
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        SCORE_MATCH_WORD,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        SCORE_MATCH_WORD,
        SCORE_MATCH_DOT,
        SCORE_MATCH_SLASH,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        SCORE_MATCH_WORD,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
    ],
    [
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        SCORE_MATCH_WORD,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        SCORE_MATCH_WORD,
        SCORE_MATCH_DOT,
        SCORE_MATCH_SLASH,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        SCORE_MATCH_WORD,
        0.,
        0.7f64,
        0.7f64,
        0.7f64,
        0.7f64,
        0.7f64,
        0.7f64,
        0.7f64,
        0.7f64,
        0.7f64,
        0.7f64,
        0.7f64,
        0.7f64,
        0.7f64,
        0.7f64,
        0.7f64,
        0.7f64,
        0.7f64,
        0.7f64,
        0.7f64,
        0.7f64,
        0.7f64,
        0.7f64,
        0.7f64,
        0.7f64,
        0.7f64,
        0.7f64,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
    ],
];
#[no_mangle]
pub static mut bonus_index: [size_t; 256] = [
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    1 as c_int as size_t,
    1 as c_int as size_t,
    1 as c_int as size_t,
    1 as c_int as size_t,
    1 as c_int as size_t,
    1 as c_int as size_t,
    1 as c_int as size_t,
    1 as c_int as size_t,
    1 as c_int as size_t,
    1 as c_int as size_t,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    2 as c_int as size_t,
    2 as c_int as size_t,
    2 as c_int as size_t,
    2 as c_int as size_t,
    2 as c_int as size_t,
    2 as c_int as size_t,
    2 as c_int as size_t,
    2 as c_int as size_t,
    2 as c_int as size_t,
    2 as c_int as size_t,
    2 as c_int as size_t,
    2 as c_int as size_t,
    2 as c_int as size_t,
    2 as c_int as size_t,
    2 as c_int as size_t,
    2 as c_int as size_t,
    2 as c_int as size_t,
    2 as c_int as size_t,
    2 as c_int as size_t,
    2 as c_int as size_t,
    2 as c_int as size_t,
    2 as c_int as size_t,
    2 as c_int as size_t,
    2 as c_int as size_t,
    2 as c_int as size_t,
    2 as c_int as size_t,
    0,
    0,
    0,
    0,
    0,
    0,
    1 as c_int as size_t,
    1 as c_int as size_t,
    1 as c_int as size_t,
    1 as c_int as size_t,
    1 as c_int as size_t,
    1 as c_int as size_t,
    1 as c_int as size_t,
    1 as c_int as size_t,
    1 as c_int as size_t,
    1 as c_int as size_t,
    1 as c_int as size_t,
    1 as c_int as size_t,
    1 as c_int as size_t,
    1 as c_int as size_t,
    1 as c_int as size_t,
    1 as c_int as size_t,
    1 as c_int as size_t,
    1 as c_int as size_t,
    1 as c_int as size_t,
    1 as c_int as size_t,
    1 as c_int as size_t,
    1 as c_int as size_t,
    1 as c_int as size_t,
    1 as c_int as size_t,
    1 as c_int as size_t,
    1 as c_int as size_t,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
];
pub const SCORE_MATCH_SLASH: c_double = 0.9f64;
pub const SCORE_MATCH_WORD: c_double = 0.8f64;
pub const SCORE_MATCH_DOT: c_double = 0.6f64;
pub const SCORE_GAP_LEADING: c_double = -0.005f64;
pub const SCORE_GAP_TRAILING: c_double = -0.005f64;
pub const SCORE_GAP_INNER: c_double = -0.01f64;
pub const SCORE_MATCH_CONSECUTIVE: c_double = 1.0f64;
