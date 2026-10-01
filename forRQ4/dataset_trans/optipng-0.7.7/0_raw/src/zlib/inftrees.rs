#[derive(Copy, Clone)]
#[repr(C)]
pub struct code {
    pub op: ::core::ffi::c_uchar,
    pub bits: ::core::ffi::c_uchar,
    pub val: ::core::ffi::c_ushort,
}
pub type codetype = ::core::ffi::c_uint;
pub const DISTS: codetype = 2;
pub const LENS: codetype = 1;
pub const CODES: codetype = 0;
pub const ENOUGH_LENS: ::core::ffi::c_int = 852 as ::core::ffi::c_int;
pub const ENOUGH_DISTS: ::core::ffi::c_int = 592 as ::core::ffi::c_int;
pub const MAXBITS: ::core::ffi::c_int = 15 as ::core::ffi::c_int;
#[no_mangle]
pub static mut inflate_copyright: [::core::ffi::c_char; 48] = unsafe {
    ::core::mem::transmute::<[u8; 48], [::core::ffi::c_char; 48]>(
        *b" inflate 1.2.11 Copyright 1995-2017 Mark Adler \0",
    )
};
#[no_mangle]
pub unsafe extern "C" fn inflate_table(
    mut type_0: codetype,
    mut lens: *mut ::core::ffi::c_ushort,
    mut codes: ::core::ffi::c_uint,
    mut table: *mut *mut code,
    mut bits: *mut ::core::ffi::c_uint,
    mut work: *mut ::core::ffi::c_ushort,
) -> ::core::ffi::c_int {
    let mut len: ::core::ffi::c_uint = 0;
    let mut sym: ::core::ffi::c_uint = 0;
    let mut min: ::core::ffi::c_uint = 0;
    let mut max: ::core::ffi::c_uint = 0;
    let mut root: ::core::ffi::c_uint = 0;
    let mut curr: ::core::ffi::c_uint = 0;
    let mut drop_0: ::core::ffi::c_uint = 0;
    let mut left: ::core::ffi::c_int = 0;
    let mut used: ::core::ffi::c_uint = 0;
    let mut huff: ::core::ffi::c_uint = 0;
    let mut incr: ::core::ffi::c_uint = 0;
    let mut fill: ::core::ffi::c_uint = 0;
    let mut low: ::core::ffi::c_uint = 0;
    let mut mask: ::core::ffi::c_uint = 0;
    let mut here: code = code {
        op: 0,
        bits: 0,
        val: 0,
    };
    let mut next: *mut code = ::core::ptr::null_mut::<code>();
    let mut base: *const ::core::ffi::c_ushort = ::core::ptr::null::<::core::ffi::c_ushort>();
    let mut extra: *const ::core::ffi::c_ushort = ::core::ptr::null::<::core::ffi::c_ushort>();
    let mut match_0: ::core::ffi::c_uint = 0;
    let mut count: [::core::ffi::c_ushort; 16] = [0; 16];
    let mut offs: [::core::ffi::c_ushort; 16] = [0; 16];
    static mut lbase: [::core::ffi::c_ushort; 31] = [
        3 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        4 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        5 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        6 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        7 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        8 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        9 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        10 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        11 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        13 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        15 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        17 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        19 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        23 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        27 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        31 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        35 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        43 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        51 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        59 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        67 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        83 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        99 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        115 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        131 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        163 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        227 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    ];
    static mut lext: [::core::ffi::c_ushort; 31] = [
        16 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        16 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        16 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        16 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        16 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        16 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        16 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        16 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        17 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        17 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        17 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        17 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        18 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        18 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        18 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        18 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        19 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        19 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        19 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        19 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        20 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        20 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        20 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        20 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        21 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        21 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        21 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        21 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        16 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        77 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        202 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    ];
    static mut dbase: [::core::ffi::c_ushort; 32] = [
        1 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        2 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        3 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        4 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        5 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        7 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        9 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        13 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        17 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        25 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        33 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        49 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        65 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        97 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        129 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        193 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        257 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        385 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        513 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        769 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        1025 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        1537 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        2049 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        3073 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        4097 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        6145 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        8193 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        12289 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        16385 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        24577 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    ];
    static mut dext: [::core::ffi::c_ushort; 32] = [
        16 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        16 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        16 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        16 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        17 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        17 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        18 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        18 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        19 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        19 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        20 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        20 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        21 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        21 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        22 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        22 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        23 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        23 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        24 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        24 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        25 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        25 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        26 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        26 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        27 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        27 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        28 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        28 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        29 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        29 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        64 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        64 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    ];
    len = 0 as ::core::ffi::c_uint;
    while len <= MAXBITS as ::core::ffi::c_uint {
        count[len as usize] = 0 as ::core::ffi::c_ushort;
        len = len.wrapping_add(1);
    }
    sym = 0 as ::core::ffi::c_uint;
    while sym < codes {
        count[*lens.offset(sym as isize) as usize] =
            count[*lens.offset(sym as isize) as usize].wrapping_add(1);
        sym = sym.wrapping_add(1);
    }
    root = *bits;
    max = MAXBITS as ::core::ffi::c_uint;
    while max >= 1 as ::core::ffi::c_uint {
        if count[max as usize] as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
            break;
        }
        max = max.wrapping_sub(1);
    }
    if root > max {
        root = max;
    }
    if max == 0 as ::core::ffi::c_uint {
        here.op = 64 as ::core::ffi::c_int as ::core::ffi::c_uchar;
        here.bits = 1 as ::core::ffi::c_int as ::core::ffi::c_uchar;
        here.val = 0 as ::core::ffi::c_int as ::core::ffi::c_ushort;
        let fresh0 = *table;
        *table = (*table).offset(1);
        *fresh0 = here;
        let fresh1 = *table;
        *table = (*table).offset(1);
        *fresh1 = here;
        *bits = 1 as ::core::ffi::c_uint;
        return 0 as ::core::ffi::c_int;
    }
    min = 1 as ::core::ffi::c_uint;
    while min < max {
        if count[min as usize] as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
            break;
        }
        min = min.wrapping_add(1);
    }
    if root < min {
        root = min;
    }
    left = 1 as ::core::ffi::c_int;
    len = 1 as ::core::ffi::c_uint;
    while len <= MAXBITS as ::core::ffi::c_uint {
        left <<= 1 as ::core::ffi::c_int;
        left -= count[len as usize] as ::core::ffi::c_int;
        if left < 0 as ::core::ffi::c_int {
            return -(1 as ::core::ffi::c_int);
        }
        len = len.wrapping_add(1);
    }
    if left > 0 as ::core::ffi::c_int
        && (type_0 as ::core::ffi::c_uint == CODES as ::core::ffi::c_int as ::core::ffi::c_uint
            || max != 1 as ::core::ffi::c_uint)
    {
        return -(1 as ::core::ffi::c_int);
    }
    offs[1 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_ushort;
    len = 1 as ::core::ffi::c_uint;
    while len < MAXBITS as ::core::ffi::c_uint {
        offs[len.wrapping_add(1 as ::core::ffi::c_uint) as usize] =
            (offs[len as usize] as ::core::ffi::c_int + count[len as usize] as ::core::ffi::c_int)
                as ::core::ffi::c_ushort;
        len = len.wrapping_add(1);
    }
    sym = 0 as ::core::ffi::c_uint;
    while sym < codes {
        if *lens.offset(sym as isize) as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
            let fresh2 = offs[*lens.offset(sym as isize) as usize];
            offs[*lens.offset(sym as isize) as usize] =
                offs[*lens.offset(sym as isize) as usize].wrapping_add(1);
            *work.offset(fresh2 as isize) = sym as ::core::ffi::c_ushort;
        }
        sym = sym.wrapping_add(1);
    }
    match type_0 as ::core::ffi::c_uint {
        0 => {
            extra = work;
            base = extra;
            match_0 = 20 as ::core::ffi::c_uint;
        }
        1 => {
            base = &raw const lbase as *const ::core::ffi::c_ushort;
            extra = &raw const lext as *const ::core::ffi::c_ushort;
            match_0 = 257 as ::core::ffi::c_uint;
        }
        _ => {
            base = &raw const dbase as *const ::core::ffi::c_ushort;
            extra = &raw const dext as *const ::core::ffi::c_ushort;
            match_0 = 0 as ::core::ffi::c_uint;
        }
    }
    huff = 0 as ::core::ffi::c_uint;
    sym = 0 as ::core::ffi::c_uint;
    len = min;
    next = *table;
    curr = root;
    drop_0 = 0 as ::core::ffi::c_uint;
    low = -(1 as ::core::ffi::c_int) as ::core::ffi::c_uint;
    used = (1 as ::core::ffi::c_uint) << root;
    mask = used.wrapping_sub(1 as ::core::ffi::c_uint);
    if type_0 as ::core::ffi::c_uint == LENS as ::core::ffi::c_int as ::core::ffi::c_uint
        && used > ENOUGH_LENS as ::core::ffi::c_uint
        || type_0 as ::core::ffi::c_uint == DISTS as ::core::ffi::c_int as ::core::ffi::c_uint
            && used > ENOUGH_DISTS as ::core::ffi::c_uint
    {
        return 1 as ::core::ffi::c_int;
    }
    loop {
        here.bits = len.wrapping_sub(drop_0) as ::core::ffi::c_uchar;
        if (*work.offset(sym as isize) as ::core::ffi::c_uint)
            .wrapping_add(1 as ::core::ffi::c_uint)
            < match_0
        {
            here.op = 0 as ::core::ffi::c_int as ::core::ffi::c_uchar;
            here.val = *work.offset(sym as isize);
        } else if *work.offset(sym as isize) as ::core::ffi::c_uint >= match_0 {
            here.op = *extra.offset(
                (*work.offset(sym as isize) as ::core::ffi::c_uint).wrapping_sub(match_0) as isize,
            ) as ::core::ffi::c_uchar;
            here.val = *base.offset(
                (*work.offset(sym as isize) as ::core::ffi::c_uint).wrapping_sub(match_0) as isize,
            );
        } else {
            here.op = (32 as ::core::ffi::c_int + 64 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
            here.val = 0 as ::core::ffi::c_ushort;
        }
        incr = (1 as ::core::ffi::c_uint) << len.wrapping_sub(drop_0);
        fill = (1 as ::core::ffi::c_uint) << curr;
        min = fill;
        loop {
            fill = fill.wrapping_sub(incr);
            *next.offset((huff >> drop_0).wrapping_add(fill) as isize) = here;
            if !(fill != 0 as ::core::ffi::c_uint) {
                break;
            }
        }
        incr = (1 as ::core::ffi::c_uint) << len.wrapping_sub(1 as ::core::ffi::c_uint);
        while huff & incr != 0 {
            incr >>= 1 as ::core::ffi::c_int;
        }
        if incr != 0 as ::core::ffi::c_uint {
            huff &= incr.wrapping_sub(1 as ::core::ffi::c_uint);
            huff = huff.wrapping_add(incr);
        } else {
            huff = 0 as ::core::ffi::c_uint;
        }
        sym = sym.wrapping_add(1);
        count[len as usize] = count[len as usize].wrapping_sub(1);
        if count[len as usize] as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            if len == max {
                break;
            }
            len = *lens.offset(*work.offset(sym as isize) as isize) as ::core::ffi::c_uint;
        }
        if len > root && huff & mask != low {
            if drop_0 == 0 as ::core::ffi::c_uint {
                drop_0 = root;
            }
            next = next.offset(min as isize);
            curr = len.wrapping_sub(drop_0);
            left = (1 as ::core::ffi::c_int) << curr;
            while curr.wrapping_add(drop_0) < max {
                left -= count[curr.wrapping_add(drop_0) as usize] as ::core::ffi::c_int;
                if left <= 0 as ::core::ffi::c_int {
                    break;
                }
                curr = curr.wrapping_add(1);
                left <<= 1 as ::core::ffi::c_int;
            }
            used = used.wrapping_add((1 as ::core::ffi::c_uint) << curr);
            if type_0 as ::core::ffi::c_uint == LENS as ::core::ffi::c_int as ::core::ffi::c_uint
                && used > ENOUGH_LENS as ::core::ffi::c_uint
                || type_0 as ::core::ffi::c_uint
                    == DISTS as ::core::ffi::c_int as ::core::ffi::c_uint
                    && used > ENOUGH_DISTS as ::core::ffi::c_uint
            {
                return 1 as ::core::ffi::c_int;
            }
            low = huff & mask;
            (*(*table).offset(low as isize)).op = curr as ::core::ffi::c_uchar;
            (*(*table).offset(low as isize)).bits = root as ::core::ffi::c_uchar;
            (*(*table).offset(low as isize)).val =
                next.offset_from(*table) as ::core::ffi::c_long as ::core::ffi::c_ushort;
        }
    }
    if huff != 0 as ::core::ffi::c_uint {
        here.op = 64 as ::core::ffi::c_int as ::core::ffi::c_uchar;
        here.bits = len.wrapping_sub(drop_0) as ::core::ffi::c_uchar;
        here.val = 0 as ::core::ffi::c_int as ::core::ffi::c_ushort;
        *next.offset(huff as isize) = here;
    }
    *table = (*table).offset(used as isize);
    *bits = root;
    return 0 as ::core::ffi::c_int;
}
