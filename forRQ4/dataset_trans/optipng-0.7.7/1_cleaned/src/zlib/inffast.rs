use core::ffi::*;
pub use crate::src::c_consts::*;
pub use crate::src::c_structs::*;
pub use crate::src::c_types::*;
extern "C" {
    pub type internal_state;
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct z_stream_s {
    pub next_in: *mut Bytef,
    pub avail_in: uInt,
    pub total_in: uLong,
    pub next_out: *mut Bytef,
    pub avail_out: uInt,
    pub total_out: uLong,
    pub msg: *mut c_char,
    pub state: *mut internal_state,
    pub zalloc: alloc_func,
    pub zfree: free_func,
    pub opaque: voidpf,
    pub data_type: c_int,
    pub adler: uLong,
    pub reserved: uLong,
}
pub type z_stream = z_stream_s;
pub type z_streamp = *mut z_stream;

pub const COPY: inflate_mode = 16195;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct inflate_state {
    pub strm: z_streamp,
    pub mode: inflate_mode,
    pub last: c_int,
    pub wrap: c_int,
    pub havedict: c_int,
    pub flags: c_int,
    pub dmax: c_uint,
    pub check: c_ulong,
    pub total: c_ulong,
    pub head: gz_headerp,
    pub wbits: c_uint,
    pub wsize: c_uint,
    pub whave: c_uint,
    pub wnext: c_uint,
    pub window: *mut c_uchar,
    pub hold: c_ulong,
    pub bits: c_uint,
    pub length: c_uint,
    pub offset: c_uint,
    pub extra: c_uint,
    pub lencode: *const code,
    pub distcode: *const code,
    pub lenbits: c_uint,
    pub distbits: c_uint,
    pub ncode: c_uint,
    pub nlen: c_uint,
    pub ndist: c_uint,
    pub have: c_uint,
    pub next: *mut code,
    pub lens: [c_ushort; 320],
    pub work: [c_ushort; 288],
    pub codes: [code; 1444],
    pub sane: c_int,
    pub back: c_int,
    pub was: c_uint,
}
#[no_mangle]
pub unsafe extern "C" fn inflate_fast(mut strm: z_streamp, mut start: c_uint) {
    let mut state: *mut inflate_state = ::core::ptr::null_mut::<inflate_state>();
    let mut in_0: *mut c_uchar = ::core::ptr::null_mut::<c_uchar>();
    let mut last: *mut c_uchar = ::core::ptr::null_mut::<c_uchar>();
    let mut out: *mut c_uchar = ::core::ptr::null_mut::<c_uchar>();
    let mut beg: *mut c_uchar = ::core::ptr::null_mut::<c_uchar>();
    let mut end: *mut c_uchar = ::core::ptr::null_mut::<c_uchar>();
    let mut wsize: c_uint = 0;
    let mut whave: c_uint = 0;
    let mut wnext: c_uint = 0;
    let mut window: *mut c_uchar = ::core::ptr::null_mut::<c_uchar>();
    let mut hold: c_ulong = 0;
    let mut bits: c_uint = 0;
    let mut lcode: *const code = ::core::ptr::null::<code>();
    let mut dcode: *const code = ::core::ptr::null::<code>();
    let mut lmask: c_uint = 0;
    let mut dmask: c_uint = 0;
    let mut here: code = code {
        op: 0,
        bits: 0,
        val: 0,
    };
    let mut op: c_uint = 0;
    let mut len: c_uint = 0;
    let mut dist: c_uint = 0;
    let mut from: *mut c_uchar = ::core::ptr::null_mut::<c_uchar>();
    state = (*strm).state as *mut inflate_state;
    in_0 = (*strm).next_in as *mut c_uchar;
    last = in_0.offset(
        ((*strm).avail_in as c_uint).wrapping_sub(5 as c_uint) as isize,
    );
    out = (*strm).next_out as *mut c_uchar;
    beg = out.offset(-((start as uInt).wrapping_sub((*strm).avail_out) as isize));
    end = out.offset(
        ((*strm).avail_out as c_uint).wrapping_sub(257 as c_uint)
            as isize,
    );
    wsize = (*state).wsize;
    whave = (*state).whave;
    wnext = (*state).wnext;
    window = (*state).window;
    hold = (*state).hold;
    bits = (*state).bits;
    lcode = (*state).lencode;
    dcode = (*state).distcode;
    lmask = ((1 as c_uint) << (*state).lenbits).wrapping_sub(1 as c_uint);
    dmask =
        ((1 as c_uint) << (*state).distbits).wrapping_sub(1 as c_uint);
    let mut current_block_141: u64;
    's_94: loop {
        if bits < 15 as c_uint {
            let fresh0 = in_0;
            in_0 = in_0.offset(1);
            hold = hold.wrapping_add((*fresh0 as c_ulong) << bits);
            bits = bits.wrapping_add(8 as c_uint);
            let fresh1 = in_0;
            in_0 = in_0.offset(1);
            hold = hold.wrapping_add((*fresh1 as c_ulong) << bits);
            bits = bits.wrapping_add(8 as c_uint);
        }
        here = *lcode.offset((hold & lmask as c_ulong) as isize);
        loop {
            op = here.bits as c_uint;
            hold >>= op;
            bits = bits.wrapping_sub(op);
            op = here.op as c_uint;
            if op == 0 as c_uint {
                let fresh2 = out;
                out = out.offset(1);
                *fresh2 = here.val as c_uchar;
                current_block_141 = 5689001924483802034;
                break;
            } else if op & 16 as c_uint != 0 {
                len = here.val as c_uint;
                op &= 15 as c_uint;
                if op != 0 {
                    if bits < op {
                        let fresh3 = in_0;
                        in_0 = in_0.offset(1);
                        hold = hold.wrapping_add((*fresh3 as c_ulong) << bits);
                        bits = bits.wrapping_add(8 as c_uint);
                    }
                    len = len.wrapping_add(
                        hold as c_uint
                            & ((1 as c_uint) << op)
                                .wrapping_sub(1 as c_uint),
                    );
                    hold >>= op;
                    bits = bits.wrapping_sub(op);
                }
                if bits < 15 as c_uint {
                    let fresh4 = in_0;
                    in_0 = in_0.offset(1);
                    hold = hold.wrapping_add((*fresh4 as c_ulong) << bits);
                    bits = bits.wrapping_add(8 as c_uint);
                    let fresh5 = in_0;
                    in_0 = in_0.offset(1);
                    hold = hold.wrapping_add((*fresh5 as c_ulong) << bits);
                    bits = bits.wrapping_add(8 as c_uint);
                }
                here = *dcode.offset((hold & dmask as c_ulong) as isize);
                current_block_141 = 4617688975773714446;
                break;
            } else if op & 64 as c_uint == 0 as c_uint {
                here = *lcode.offset((here.val as c_ulong).wrapping_add(
                    hold & ((1 as c_uint) << op).wrapping_sub(1 as c_uint)
                        as c_ulong,
                ) as isize);
            } else if op & 32 as c_uint != 0 {
                current_block_141 = 13505557363059842426;
                break;
            } else {
                current_block_141 = 9180031981464905198;
                break;
            }
        }
        match current_block_141 {
            4617688975773714446 => {
                loop {
                    op = here.bits as c_uint;
                    hold >>= op;
                    bits = bits.wrapping_sub(op);
                    op = here.op as c_uint;
                    if op & 16 as c_uint != 0 {
                        dist = here.val as c_uint;
                        op &= 15 as c_uint;
                        if bits < op {
                            let fresh6 = in_0;
                            in_0 = in_0.offset(1);
                            hold = hold.wrapping_add((*fresh6 as c_ulong) << bits);
                            bits = bits.wrapping_add(8 as c_uint);
                            if bits < op {
                                let fresh7 = in_0;
                                in_0 = in_0.offset(1);
                                hold = hold.wrapping_add((*fresh7 as c_ulong) << bits);
                                bits = bits.wrapping_add(8 as c_uint);
                            }
                        }
                        dist = dist.wrapping_add(
                            hold as c_uint
                                & ((1 as c_uint) << op)
                                    .wrapping_sub(1 as c_uint),
                        );
                        hold >>= op;
                        bits = bits.wrapping_sub(op);
                        op = out.offset_from(beg) as c_long as c_uint;
                        if dist > op {
                            current_block_141 = 5235537862154438448;
                            break;
                        } else {
                            current_block_141 = 6072622540298447352;
                            break;
                        }
                    } else if op & 64 as c_uint == 0 as c_uint {
                        here = *dcode.offset(
                            (here.val as c_ulong).wrapping_add(
                                hold & ((1 as c_uint) << op)
                                    .wrapping_sub(1 as c_uint)
                                    as c_ulong,
                            ) as isize,
                        );
                    } else {
                        (*strm).msg = b"invalid distance code\0" as *const u8
                            as *const c_char
                            as *mut c_char;
                        (*state).mode = BAD;
                        break 's_94;
                    }
                }
                match current_block_141 {
                    6072622540298447352 => {
                        from = out.offset(-(dist as isize));
                        loop {
                            let fresh26 = from;
                            from = from.offset(1);
                            let fresh27 = out;
                            out = out.offset(1);
                            *fresh27 = *fresh26;
                            let fresh28 = from;
                            from = from.offset(1);
                            let fresh29 = out;
                            out = out.offset(1);
                            *fresh29 = *fresh28;
                            let fresh30 = from;
                            from = from.offset(1);
                            let fresh31 = out;
                            out = out.offset(1);
                            *fresh31 = *fresh30;
                            len = len.wrapping_sub(3 as c_uint);
                            if !(len > 2 as c_uint) {
                                break;
                            }
                        }
                        if len != 0 {
                            let fresh32 = from;
                            from = from.offset(1);
                            let fresh33 = out;
                            out = out.offset(1);
                            *fresh33 = *fresh32;
                            if len > 1 as c_uint {
                                let fresh34 = from;
                                from = from.offset(1);
                                let fresh35 = out;
                                out = out.offset(1);
                                *fresh35 = *fresh34;
                            }
                        }
                    }
                    _ => {
                        op = dist.wrapping_sub(op);
                        if op > whave {
                            if (*state).sane != 0 {
                                (*strm).msg = b"invalid distance too far back\0" as *const u8
                                    as *const c_char
                                    as *mut c_char;
                                (*state).mode = BAD;
                                break;
                            }
                        }
                        from = window;
                        if wnext == 0 as c_uint {
                            from = from.offset(wsize.wrapping_sub(op) as isize);
                            if op < len {
                                len = len.wrapping_sub(op);
                                loop {
                                    let fresh8 = from;
                                    from = from.offset(1);
                                    let fresh9 = out;
                                    out = out.offset(1);
                                    *fresh9 = *fresh8;
                                    op = op.wrapping_sub(1);
                                    if !(op != 0) {
                                        break;
                                    }
                                }
                                from = out.offset(-(dist as isize));
                            }
                        } else if wnext < op {
                            from = from.offset(wsize.wrapping_add(wnext).wrapping_sub(op) as isize);
                            op = op.wrapping_sub(wnext);
                            if op < len {
                                len = len.wrapping_sub(op);
                                loop {
                                    let fresh10 = from;
                                    from = from.offset(1);
                                    let fresh11 = out;
                                    out = out.offset(1);
                                    *fresh11 = *fresh10;
                                    op = op.wrapping_sub(1);
                                    if !(op != 0) {
                                        break;
                                    }
                                }
                                from = window;
                                if wnext < len {
                                    op = wnext;
                                    len = len.wrapping_sub(op);
                                    loop {
                                        let fresh12 = from;
                                        from = from.offset(1);
                                        let fresh13 = out;
                                        out = out.offset(1);
                                        *fresh13 = *fresh12;
                                        op = op.wrapping_sub(1);
                                        if !(op != 0) {
                                            break;
                                        }
                                    }
                                    from = out.offset(-(dist as isize));
                                }
                            }
                        } else {
                            from = from.offset(wnext.wrapping_sub(op) as isize);
                            if op < len {
                                len = len.wrapping_sub(op);
                                loop {
                                    let fresh14 = from;
                                    from = from.offset(1);
                                    let fresh15 = out;
                                    out = out.offset(1);
                                    *fresh15 = *fresh14;
                                    op = op.wrapping_sub(1);
                                    if !(op != 0) {
                                        break;
                                    }
                                }
                                from = out.offset(-(dist as isize));
                            }
                        }
                        while len > 2 as c_uint {
                            let fresh16 = from;
                            from = from.offset(1);
                            let fresh17 = out;
                            out = out.offset(1);
                            *fresh17 = *fresh16;
                            let fresh18 = from;
                            from = from.offset(1);
                            let fresh19 = out;
                            out = out.offset(1);
                            *fresh19 = *fresh18;
                            let fresh20 = from;
                            from = from.offset(1);
                            let fresh21 = out;
                            out = out.offset(1);
                            *fresh21 = *fresh20;
                            len = len.wrapping_sub(3 as c_uint);
                        }
                        if len != 0 {
                            let fresh22 = from;
                            from = from.offset(1);
                            let fresh23 = out;
                            out = out.offset(1);
                            *fresh23 = *fresh22;
                            if len > 1 as c_uint {
                                let fresh24 = from;
                                from = from.offset(1);
                                let fresh25 = out;
                                out = out.offset(1);
                                *fresh25 = *fresh24;
                            }
                        }
                    }
                }
            }
            9180031981464905198 => {
                (*strm).msg = b"invalid literal/length code\0" as *const u8
                    as *const c_char
                    as *mut c_char;
                (*state).mode = BAD;
                break;
            }
            13505557363059842426 => {
                (*state).mode = TYPE;
                break;
            }
            _ => {}
        }
        if !(in_0 < last && out < end) {
            break;
        }
    }
    len = bits >> 3 as c_int;
    in_0 = in_0.offset(-(len as isize));
    bits = bits.wrapping_sub(len << 3 as c_int);
    hold &= ((1 as c_uint) << bits).wrapping_sub(1 as c_uint)
        as c_ulong;
    (*strm).next_in = in_0 as *mut Bytef;
    (*strm).next_out = out as *mut Bytef;
    (*strm).avail_in = (if in_0 < last {
        5 as c_long + last.offset_from(in_0) as c_long
    } else {
        5 as c_long - in_0.offset_from(last) as c_long
    }) as c_uint as uInt;
    (*strm).avail_out = (if out < end {
        257 as c_long + end.offset_from(out) as c_long
    } else {
        257 as c_long - out.offset_from(end) as c_long
    }) as c_uint as uInt;
    (*state).hold = hold;
    (*state).bits = bits;
}
