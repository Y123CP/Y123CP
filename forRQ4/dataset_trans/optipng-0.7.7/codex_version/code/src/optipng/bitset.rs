extern "C" {
    fn isspace(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn __errno_location() -> *mut ::core::ffi::c_int;
}
pub type opng_bitset_t = ::core::ffi::c_uint;
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const OPNG_BITSET_ELT_MAX: C2RustUnnamed = 31;
pub const OPNG_BITSET_ELT_MIN: C2RustUnnamed = 0;
pub const EINVAL: ::core::ffi::c_int = 22 as ::core::ffi::c_int;
pub const ERANGE: ::core::ffi::c_int = 34 as ::core::ffi::c_int;
pub const OPNG_BITSET_EMPTY: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
pub const OPNG_BITSET_FULL: ::core::ffi::c_uint = !(0 as ::core::ffi::c_uint);
#[no_mangle]
pub unsafe extern "C" fn opng_bitset_count(mut set: opng_bitset_t) -> ::core::ffi::c_uint {
    let mut result: ::core::ffi::c_uint = 0;
    result = 0 as ::core::ffi::c_uint;
    while set != 0 as ::core::ffi::c_uint {
        set &= (set as ::core::ffi::c_uint).wrapping_sub(1 as ::core::ffi::c_uint);
        result = result.wrapping_add(1);
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn opng_bitset_find_first(mut set: opng_bitset_t) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i <= OPNG_BITSET_ELT_MAX as ::core::ffi::c_int {
        if set as ::core::ffi::c_uint & (1 as ::core::ffi::c_uint) << i != 0 as ::core::ffi::c_uint
        {
            return i;
        }
        i += 1;
    }
    return -(1 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn opng_bitset_find_next(
    mut set: opng_bitset_t,
    mut elt: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    i = (if elt > -(1 as ::core::ffi::c_int) {
        elt
    } else {
        -(1 as ::core::ffi::c_int)
    }) + 1 as ::core::ffi::c_int;
    while i <= OPNG_BITSET_ELT_MAX as ::core::ffi::c_int {
        if set as ::core::ffi::c_uint & (1 as ::core::ffi::c_uint) << i != 0 as ::core::ffi::c_uint
        {
            return i;
        }
        i += 1;
    }
    return -(1 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn opng_bitset_find_last(mut set: opng_bitset_t) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    i = OPNG_BITSET_ELT_MAX as ::core::ffi::c_int;
    while i >= 0 as ::core::ffi::c_int {
        if set as ::core::ffi::c_uint & (1 as ::core::ffi::c_uint) << i != 0 as ::core::ffi::c_uint
        {
            return i;
        }
        i -= 1;
    }
    return -(1 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn opng_bitset_find_prev(
    mut set: opng_bitset_t,
    mut elt: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    i = (if elt < OPNG_BITSET_ELT_MAX as ::core::ffi::c_int + 1 as ::core::ffi::c_int {
        elt
    } else {
        OPNG_BITSET_ELT_MAX as ::core::ffi::c_int + 1 as ::core::ffi::c_int
    }) - 1 as ::core::ffi::c_int;
    while i >= 0 as ::core::ffi::c_int {
        if set as ::core::ffi::c_uint & (1 as ::core::ffi::c_uint) << i != 0 as ::core::ffi::c_uint
        {
            return i;
        }
        i -= 1;
    }
    return -(1 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn opng_strparse_rangeset_to_bitset(
    mut out_set: *mut opng_bitset_t,
    mut rangeset_str: *const ::core::ffi::c_char,
    mut mask_set: opng_bitset_t,
) -> ::core::ffi::c_int {
    let mut result: opng_bitset_t = 0;
    let mut ptr: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut state: ::core::ffi::c_int = 0;
    let mut num: ::core::ffi::c_int = 0;
    let mut num1: ::core::ffi::c_int = 0;
    let mut num2: ::core::ffi::c_int = 0;
    let mut err_invalid: ::core::ffi::c_int = 0;
    let mut err_range: ::core::ffi::c_int = 0;
    result = OPNG_BITSET_EMPTY as opng_bitset_t;
    ptr = rangeset_str;
    state = 0 as ::core::ffi::c_int;
    err_range = 0 as ::core::ffi::c_int;
    err_invalid = err_range;
    num2 = -(1 as ::core::ffi::c_int);
    num1 = num2;
    let mut current_block_37: u64;
    loop {
        while isspace(*ptr as ::core::ffi::c_int) != 0 {
            ptr = ptr.offset(1);
        }
        match state {
            0 => {
                current_block_37 = 6958789677802808342;
            }
            2 => {
                current_block_37 = 6958789677802808342;
            }
            1 => {
                if *ptr as ::core::ffi::c_int == '-' as i32 {
                    ptr = ptr.offset(1);
                    num2 = OPNG_BITSET_ELT_MAX as ::core::ffi::c_int;
                    state += 1;
                    continue;
                } else {
                    current_block_37 = 15345278821338558188;
                }
            }
            _ => {
                current_block_37 = 15345278821338558188;
            }
        }
        match current_block_37 {
            6958789677802808342 => {
                if *ptr as ::core::ffi::c_int >= '0' as i32
                    && *ptr as ::core::ffi::c_int <= '9' as i32
                {
                    num = 0 as ::core::ffi::c_int;
                    loop {
                        num = 10 as ::core::ffi::c_int * num
                            + (*ptr as ::core::ffi::c_int - '0' as i32);
                        if num > OPNG_BITSET_ELT_MAX as ::core::ffi::c_int {
                            num = OPNG_BITSET_ELT_MAX as ::core::ffi::c_int;
                            err_range = 1 as ::core::ffi::c_int;
                        }
                        ptr = ptr.offset(1);
                        if !(*ptr as ::core::ffi::c_int >= '0' as i32
                            && *ptr as ::core::ffi::c_int <= '9' as i32)
                        {
                            break;
                        }
                    }
                    if !(mask_set as ::core::ffi::c_uint & (1 as ::core::ffi::c_uint) << num
                        != 0 as ::core::ffi::c_uint)
                    {
                        err_range = 1 as ::core::ffi::c_int;
                    }
                    if state == 0 as ::core::ffi::c_int {
                        num1 = num;
                    }
                    num2 = num;
                    state += 1;
                    continue;
                }
            }
            _ => {}
        }
        if state > 0 as ::core::ffi::c_int {
            if num1 <= num2 {
                result |= if num1 <= num2 {
                    ((1 as ::core::ffi::c_uint) << num2 - num1 << 1 as ::core::ffi::c_int)
                        .wrapping_sub(1 as ::core::ffi::c_uint)
                        << num1
                } else {
                    0 as ::core::ffi::c_uint
                };
                result &= mask_set as ::core::ffi::c_uint;
            } else {
                err_range = 1 as ::core::ffi::c_int;
            }
            state = 0 as ::core::ffi::c_int;
        }
        if *ptr as ::core::ffi::c_int == ',' as i32 || *ptr as ::core::ffi::c_int == ';' as i32 {
            ptr = ptr.offset(1);
        } else {
            if !(*ptr as ::core::ffi::c_int == '-' as i32) {
                break;
            }
            err_invalid = 1 as ::core::ffi::c_int;
            break;
        }
    }
    while isspace(*ptr as ::core::ffi::c_int) != 0 {
        ptr = ptr.offset(1);
    }
    if *ptr as ::core::ffi::c_int != '\0' as i32 {
        err_invalid = 1 as ::core::ffi::c_int;
    }
    if err_invalid != 0 {
        *__errno_location() = EINVAL;
        *out_set = OPNG_BITSET_EMPTY as opng_bitset_t;
        return -(1 as ::core::ffi::c_int);
    } else if err_range != 0 {
        *__errno_location() = ERANGE;
        *out_set = OPNG_BITSET_FULL as opng_bitset_t;
        return -(1 as ::core::ffi::c_int);
    } else {
        *out_set = result;
        return 0 as ::core::ffi::c_int;
    };
}
