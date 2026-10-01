use core::ffi::*;
pub use crate::src::ffi::*;
pub use crate::src::c_consts::*;
pub use crate::src::c_types::*;

pub type C2RustUnnamed_htdd24ee73 = c_uint;
pub const OPNG_BITSET_ELT_MAX: C2RustUnnamed_htdd24ee73 = 31;
pub const OPNG_BITSET_ELT_MIN: C2RustUnnamed_htdd24ee73 = 0;

pub const OPNG_BITSET_FULL: c_uint = !(0 as c_uint);
#[inline]
pub fn opng_bitset_count(mut set: opng_bitset_t) -> c_uint { {
    let mut result: c_uint = 0;
    result = 0 as c_uint;
    while set != 0 as c_uint {
        set &= (set as c_uint).wrapping_sub(1 as c_uint);
        result = result.wrapping_add(1);
    }
    return result;
} }
#[inline]
pub fn opng_bitset_find_first(mut set: opng_bitset_t) -> c_int { {
    let mut i: c_int = 0;
    i = 0 as c_int;
    while i <= OPNG_BITSET_ELT_MAX as c_int {
        if set as c_uint & (1 as c_uint) << i != 0 as c_uint
        {
            return i;
        }
        i += 1;
    }
    return -(1 as c_int);
} }
#[inline]
pub fn opng_bitset_find_next(
    mut set: opng_bitset_t,
    mut elt: c_int,
) -> c_int { {
    let mut i: c_int = 0;
    i = (if elt > -(1 as c_int) {
        elt
    } else {
        -(1 as c_int)
    }) + 1 as c_int;
    while i <= OPNG_BITSET_ELT_MAX as c_int {
        if set as c_uint & (1 as c_uint) << i != 0 as c_uint
        {
            return i;
        }
        i += 1;
    }
    return -(1 as c_int);
} }
#[inline]
pub fn opng_bitset_find_last(mut set: opng_bitset_t) -> c_int { {
    let mut i: c_int = 0;
    i = OPNG_BITSET_ELT_MAX as c_int;
    while i >= 0 as c_int {
        if set as c_uint & (1 as c_uint) << i != 0 as c_uint
        {
            return i;
        }
        i -= 1;
    }
    return -(1 as c_int);
} }
#[inline]
pub fn opng_bitset_find_prev(
    mut set: opng_bitset_t,
    mut elt: c_int,
) -> c_int { {
    let mut i: c_int = 0;
    i = (if elt < OPNG_BITSET_ELT_MAX as c_int + 1 as c_int {
        elt
    } else {
        OPNG_BITSET_ELT_MAX as c_int + 1 as c_int
    }) - 1 as c_int;
    while i >= 0 as c_int {
        if set as c_uint & (1 as c_uint) << i != 0 as c_uint
        {
            return i;
        }
        i -= 1;
    }
    return -(1 as c_int);
} }
// Applied rules: [III④, II_vec, III③, C3]
// Skipped rules: [C4: The loop is a parser over a NUL-terminated C string with per-byte state-machine work, digit accumulation, delimiter handling, and whitespace classification; it is not a pure byte-scan/equality cursor loop, so word-at-a-time widening would not match the rule's safety/perf envelope. No concrete C4 hit will be addressed.; III③: __errno_location hits [14, 15] are not manual memory/string operations in scope for this rule; replacing them would be semantic/API churn rather than the intended mem/string rewrite.; C3: Signature-lift S1 is abstained because this function is #[no_mangle] pub unsafe extern "C" and changing pointer parameters to references/slices would break the C ABI.]
#[no_mangle]
pub unsafe extern "C" fn opng_strparse_rangeset_to_bitset(
    mut out_set: *mut opng_bitset_t,
    mut rangeset_str: *const c_char,
    mut mask_set: opng_bitset_t,
) -> c_int {
    #[inline(always)]
    fn is_space_byte(b: u8) -> bool {
        matches!(b, b' ' | b'\t' | b'\n' | b'\r' | 0x0b | 0x0c)
    }

    let mut result: opng_bitset_t = OPNG_BITSET_EMPTY as opng_bitset_t;
    let mut ptr: *const c_char = rangeset_str;
    let mut state: c_int = 0;
    let mut num1: c_int = -(1 as c_int);
    let mut num2: c_int = -(1 as c_int);
    let mut err_invalid: c_int = 0;
    let mut err_range: c_int = 0;

    loop {
        loop {
            let ch = *ptr as u8;
            if !is_space_byte(ch) {
                break;
            }
            ptr = ptr.offset(1);
        }

        let ch = *ptr as u8;

        if state == 1 as c_int && ch == b'-' {
            ptr = ptr.offset(1);
            num2 = OPNG_BITSET_ELT_MAX as c_int;
            state += 1;
            continue;
        }

        if (state == 0 as c_int || state == 2 as c_int) && ch >= b'0' && ch <= b'9' {
            let mut num: c_int = 0;
            loop {
                let digit = *ptr as u8;
                if !(digit >= b'0' && digit <= b'9') {
                    break;
                }
                num = 10 as c_int * num + (digit as c_int - '0' as i32);
                if num > OPNG_BITSET_ELT_MAX as c_int {
                    num = OPNG_BITSET_ELT_MAX as c_int;
                    err_range = 1 as c_int;
                }
                ptr = ptr.offset(1);
            }
            if !(mask_set as c_uint & (1 as c_uint) << num != 0 as c_uint) {
                err_range = 1 as c_int;
            }
            if state == 0 as c_int {
                num1 = num;
            }
            num2 = num;
            state += 1;
            continue;
        }

        if state > 0 as c_int {
            if num1 <= num2 {
                result |= if num1 <= num2 {
                    ((1 as c_uint) << num2 - num1 << 1 as c_int)
                        .wrapping_sub(1 as c_uint)
                        << num1
                } else {
                    0 as c_uint
                };
                result &= mask_set as c_uint;
            } else {
                err_range = 1 as c_int;
            }
            state = 0 as c_int;
        }

        let ch = *ptr as u8;
        if ch == b',' || ch == b';' {
            ptr = ptr.offset(1);
        } else {
            if !(ch == b'-') {
                break;
            }
            err_invalid = 1 as c_int;
            break;
        }
    }

    loop {
        let ch = *ptr as u8;
        if !is_space_byte(ch) {
            break;
        }
        ptr = ptr.offset(1);
    }

    if *ptr as c_int != '\0' as i32 {
        err_invalid = 1 as c_int;
    }
    if err_invalid != 0 {
        *__errno_location() = EINVAL;
        *out_set = OPNG_BITSET_EMPTY as opng_bitset_t;
        return -(1 as c_int);
    } else if err_range != 0 {
        *__errno_location() = ERANGE;
        *out_set = OPNG_BITSET_FULL as opng_bitset_t;
        return -(1 as c_int);
    } else {
        *out_set = result;
        return 0 as c_int;
    };
}
