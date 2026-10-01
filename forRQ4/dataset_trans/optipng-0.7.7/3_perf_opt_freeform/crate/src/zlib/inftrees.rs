use core::ffi::*;
pub use crate::src::c_consts::*;
pub use crate::src::c_structs::*;
pub use crate::src::c_types::*;

pub const MAXBITS: c_int = 15 as c_int;
#[no_mangle]
pub static mut inflate_copyright: [c_char; 48] = unsafe {
    ::core::mem::transmute::<[u8; 48], [c_char; 48]>(
        *b" inflate 1.2.11 Copyright 1995-2017 Mark Adler \0",
    )
};
#[inline]
pub unsafe fn inflate_table(
    mut type_0: codetype,
    mut lens: *mut c_ushort,
    mut codes: c_uint,
    mut table: *mut *mut code,
    mut bits: *mut c_uint,
    mut work: *mut c_ushort,
) -> c_int {
    let mut len: c_uint = 0;
    let mut sym: c_uint = 0;
    let mut min: c_uint = 0;
    let mut max: c_uint = 0;
    let mut root: c_uint = 0;
    let mut curr: c_uint = 0;
    let mut drop_0: c_uint = 0;
    let mut left: c_int = 0;
    let mut used: c_uint = 0;
    let mut huff: c_uint = 0;
    let mut incr: c_uint = 0;
    let mut fill: c_uint = 0;
    let mut low: c_uint = 0;
    let mut mask: c_uint = 0;
    let mut here: code = code {
        op: 0,
        bits: 0,
        val: 0,
    };
    let mut next: *mut code = ::core::ptr::null_mut::<code>();
    let mut base: *const c_ushort = ::core::ptr::null::<c_ushort>();
    let mut extra: *const c_ushort = ::core::ptr::null::<c_ushort>();
    let mut match_0: c_uint = 0;
    let mut count: [c_ushort; 16] = [0; 16];
    let mut offs: [c_ushort; 16] = [0; 16];
    static mut lbase: [c_ushort; 31] = [
        3 as c_int as c_ushort,
        4 as c_int as c_ushort,
        5 as c_int as c_ushort,
        6 as c_int as c_ushort,
        7 as c_int as c_ushort,
        8 as c_int as c_ushort,
        9 as c_int as c_ushort,
        10 as c_int as c_ushort,
        11 as c_int as c_ushort,
        13 as c_int as c_ushort,
        15 as c_int as c_ushort,
        17 as c_int as c_ushort,
        19 as c_int as c_ushort,
        23 as c_int as c_ushort,
        27 as c_int as c_ushort,
        31 as c_int as c_ushort,
        35 as c_int as c_ushort,
        43 as c_int as c_ushort,
        51 as c_int as c_ushort,
        59 as c_int as c_ushort,
        67 as c_int as c_ushort,
        83 as c_int as c_ushort,
        99 as c_int as c_ushort,
        115 as c_int as c_ushort,
        131 as c_int as c_ushort,
        163 as c_int as c_ushort,
        195 as c_int as c_ushort,
        227 as c_int as c_ushort,
        258 as c_int as c_ushort,
        0 as c_int as c_ushort,
        0 as c_int as c_ushort,
    ];
    static mut lext: [c_ushort; 31] = [
        16 as c_int as c_ushort,
        16 as c_int as c_ushort,
        16 as c_int as c_ushort,
        16 as c_int as c_ushort,
        16 as c_int as c_ushort,
        16 as c_int as c_ushort,
        16 as c_int as c_ushort,
        16 as c_int as c_ushort,
        17 as c_int as c_ushort,
        17 as c_int as c_ushort,
        17 as c_int as c_ushort,
        17 as c_int as c_ushort,
        18 as c_int as c_ushort,
        18 as c_int as c_ushort,
        18 as c_int as c_ushort,
        18 as c_int as c_ushort,
        19 as c_int as c_ushort,
        19 as c_int as c_ushort,
        19 as c_int as c_ushort,
        19 as c_int as c_ushort,
        20 as c_int as c_ushort,
        20 as c_int as c_ushort,
        20 as c_int as c_ushort,
        20 as c_int as c_ushort,
        21 as c_int as c_ushort,
        21 as c_int as c_ushort,
        21 as c_int as c_ushort,
        21 as c_int as c_ushort,
        16 as c_int as c_ushort,
        77 as c_int as c_ushort,
        202 as c_int as c_ushort,
    ];
    static mut dbase: [c_ushort; 32] = [
        1 as c_int as c_ushort,
        2 as c_int as c_ushort,
        3 as c_int as c_ushort,
        4 as c_int as c_ushort,
        5 as c_int as c_ushort,
        7 as c_int as c_ushort,
        9 as c_int as c_ushort,
        13 as c_int as c_ushort,
        17 as c_int as c_ushort,
        25 as c_int as c_ushort,
        33 as c_int as c_ushort,
        49 as c_int as c_ushort,
        65 as c_int as c_ushort,
        97 as c_int as c_ushort,
        129 as c_int as c_ushort,
        193 as c_int as c_ushort,
        257 as c_int as c_ushort,
        385 as c_int as c_ushort,
        513 as c_int as c_ushort,
        769 as c_int as c_ushort,
        1025 as c_int as c_ushort,
        1537 as c_int as c_ushort,
        2049 as c_int as c_ushort,
        3073 as c_int as c_ushort,
        4097 as c_int as c_ushort,
        6145 as c_int as c_ushort,
        8193 as c_int as c_ushort,
        12289 as c_int as c_ushort,
        16385 as c_int as c_ushort,
        24577 as c_int as c_ushort,
        0 as c_int as c_ushort,
        0 as c_int as c_ushort,
    ];
    static mut dext: [c_ushort; 32] = [
        16 as c_int as c_ushort,
        16 as c_int as c_ushort,
        16 as c_int as c_ushort,
        16 as c_int as c_ushort,
        17 as c_int as c_ushort,
        17 as c_int as c_ushort,
        18 as c_int as c_ushort,
        18 as c_int as c_ushort,
        19 as c_int as c_ushort,
        19 as c_int as c_ushort,
        20 as c_int as c_ushort,
        20 as c_int as c_ushort,
        21 as c_int as c_ushort,
        21 as c_int as c_ushort,
        22 as c_int as c_ushort,
        22 as c_int as c_ushort,
        23 as c_int as c_ushort,
        23 as c_int as c_ushort,
        24 as c_int as c_ushort,
        24 as c_int as c_ushort,
        25 as c_int as c_ushort,
        25 as c_int as c_ushort,
        26 as c_int as c_ushort,
        26 as c_int as c_ushort,
        27 as c_int as c_ushort,
        27 as c_int as c_ushort,
        28 as c_int as c_ushort,
        28 as c_int as c_ushort,
        29 as c_int as c_ushort,
        29 as c_int as c_ushort,
        64 as c_int as c_ushort,
        64 as c_int as c_ushort,
    ];
    len = 0 as c_uint;
    while len <= MAXBITS as c_uint {
        count[len as usize] = 0 as c_ushort;
        len = len.wrapping_add(1);
    }
    sym = 0 as c_uint;
    while sym < codes {
        count[*lens.offset(sym as isize) as usize] =
            count[*lens.offset(sym as isize) as usize].wrapping_add(1);
        sym = sym.wrapping_add(1);
    }
    root = *bits;
    max = MAXBITS as c_uint;
    while max >= 1 as c_uint {
        if count[max as usize] as c_int != 0 as c_int {
            break;
        }
        max = max.wrapping_sub(1);
    }
    if root > max {
        root = max;
    }
    if max == 0 as c_uint {
        here.op = 64 as c_int as c_uchar;
        here.bits = 1 as c_int as c_uchar;
        here.val = 0 as c_int as c_ushort;
        let fresh0 = *table;
        *table = (*table).offset(1);
        *fresh0 = here;
        let fresh1 = *table;
        *table = (*table).offset(1);
        *fresh1 = here;
        *bits = 1 as c_uint;
        return 0 as c_int;
    }
    min = 1 as c_uint;
    while min < max {
        if count[min as usize] as c_int != 0 as c_int {
            break;
        }
        min = min.wrapping_add(1);
    }
    if root < min {
        root = min;
    }
    left = 1 as c_int;
    len = 1 as c_uint;
    while len <= MAXBITS as c_uint {
        left <<= 1 as c_int;
        left -= count[len as usize] as c_int;
        if left < 0 as c_int {
            return -(1 as c_int);
        }
        len = len.wrapping_add(1);
    }
    if left > 0 as c_int
        && (type_0 as c_uint == CODES as c_int as c_uint
            || max != 1 as c_uint)
    {
        return -(1 as c_int);
    }
    offs[1 as c_int as usize] = 0 as c_ushort;
    len = 1 as c_uint;
    while len < MAXBITS as c_uint {
        offs[len.wrapping_add(1 as c_uint) as usize] =
            (offs[len as usize] as c_int + count[len as usize] as c_int)
                as c_ushort;
        len = len.wrapping_add(1);
    }
    sym = 0 as c_uint;
    while sym < codes {
        if *lens.offset(sym as isize) as c_int != 0 as c_int {
            let fresh2 = offs[*lens.offset(sym as isize) as usize];
            offs[*lens.offset(sym as isize) as usize] =
                offs[*lens.offset(sym as isize) as usize].wrapping_add(1);
            *work.offset(fresh2 as isize) = sym as c_ushort;
        }
        sym = sym.wrapping_add(1);
    }
    match type_0 as c_uint {
        0 => {
            extra = work;
            base = extra;
            match_0 = 20 as c_uint;
        }
        1 => {
            base = &raw const lbase as *const c_ushort;
            extra = &raw const lext as *const c_ushort;
            match_0 = 257 as c_uint;
        }
        _ => {
            base = &raw const dbase as *const c_ushort;
            extra = &raw const dext as *const c_ushort;
            match_0 = 0 as c_uint;
        }
    }
    huff = 0 as c_uint;
    sym = 0 as c_uint;
    len = min;
    next = *table;
    curr = root;
    drop_0 = 0 as c_uint;
    low = -(1 as c_int) as c_uint;
    used = (1 as c_uint) << root;
    mask = used.wrapping_sub(1 as c_uint);
    if type_0 as c_uint == LENS as c_int as c_uint
        && used > ENOUGH_LENS as c_uint
        || type_0 as c_uint == DISTS as c_int as c_uint
            && used > ENOUGH_DISTS as c_uint
    {
        return 1 as c_int;
    }
    loop {
        here.bits = len.wrapping_sub(drop_0) as c_uchar;
        if (*work.offset(sym as isize) as c_uint)
            .wrapping_add(1 as c_uint)
            < match_0
        {
            here.op = 0 as c_int as c_uchar;
            here.val = *work.offset(sym as isize);
        } else if *work.offset(sym as isize) as c_uint >= match_0 {
            here.op = *extra.offset(
                (*work.offset(sym as isize) as c_uint).wrapping_sub(match_0) as isize,
            ) as c_uchar;
            here.val = *base.offset(
                (*work.offset(sym as isize) as c_uint).wrapping_sub(match_0) as isize,
            );
        } else {
            here.op = (32 as c_int + 64 as c_int) as c_uchar;
            here.val = 0 as c_ushort;
        }
        incr = (1 as c_uint) << len.wrapping_sub(drop_0);
        fill = (1 as c_uint) << curr;
        min = fill;
        loop {
            fill = fill.wrapping_sub(incr);
            *next.offset((huff >> drop_0).wrapping_add(fill) as isize) = here;
            if !(fill != 0 as c_uint) {
                break;
            }
        }
        incr = (1 as c_uint) << len.wrapping_sub(1 as c_uint);
        while huff & incr != 0 {
            incr >>= 1 as c_int;
        }
        if incr != 0 as c_uint {
            huff &= incr.wrapping_sub(1 as c_uint);
            huff = huff.wrapping_add(incr);
        } else {
            huff = 0 as c_uint;
        }
        sym = sym.wrapping_add(1);
        count[len as usize] = count[len as usize].wrapping_sub(1);
        if count[len as usize] as c_int == 0 as c_int {
            if len == max {
                break;
            }
            len = *lens.offset(*work.offset(sym as isize) as isize) as c_uint;
        }
        if len > root && huff & mask != low {
            if drop_0 == 0 as c_uint {
                drop_0 = root;
            }
            next = next.offset(min as isize);
            curr = len.wrapping_sub(drop_0);
            left = (1 as c_int) << curr;
            while curr.wrapping_add(drop_0) < max {
                left -= count[curr.wrapping_add(drop_0) as usize] as c_int;
                if left <= 0 as c_int {
                    break;
                }
                curr = curr.wrapping_add(1);
                left <<= 1 as c_int;
            }
            used = used.wrapping_add((1 as c_uint) << curr);
            if type_0 as c_uint == LENS as c_int as c_uint
                && used > ENOUGH_LENS as c_uint
                || type_0 as c_uint
                    == DISTS as c_int as c_uint
                    && used > ENOUGH_DISTS as c_uint
            {
                return 1 as c_int;
            }
            low = huff & mask;
            (*(*table).offset(low as isize)).op = curr as c_uchar;
            (*(*table).offset(low as isize)).bits = root as c_uchar;
            (*(*table).offset(low as isize)).val =
                next.offset_from(*table) as c_long as c_ushort;
        }
    }
    if huff != 0 as c_uint {
        here.op = 64 as c_int as c_uchar;
        here.bits = len.wrapping_sub(drop_0) as c_uchar;
        here.val = 0 as c_int as c_ushort;
        *next.offset(huff as isize) = here;
    }
    *table = (*table).offset(used as isize);
    *bits = root;
    return 0 as c_int;
}
