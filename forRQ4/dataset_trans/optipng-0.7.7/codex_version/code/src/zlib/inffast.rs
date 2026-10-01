extern "C" {
    pub type internal_state;
}
pub type Byte = ::core::ffi::c_uchar;
pub type uInt = ::core::ffi::c_uint;
pub type uLong = ::core::ffi::c_ulong;
pub type Bytef = Byte;
pub type voidpf = *mut ::core::ffi::c_void;
pub type alloc_func = Option<unsafe extern "C" fn(voidpf, uInt, uInt) -> voidpf>;
pub type free_func = Option<unsafe extern "C" fn(voidpf, voidpf) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct z_stream_s {
    pub next_in: *mut Bytef,
    pub avail_in: uInt,
    pub total_in: uLong,
    pub next_out: *mut Bytef,
    pub avail_out: uInt,
    pub total_out: uLong,
    pub msg: *mut ::core::ffi::c_char,
    pub state: *mut internal_state,
    pub zalloc: alloc_func,
    pub zfree: free_func,
    pub opaque: voidpf,
    pub data_type: ::core::ffi::c_int,
    pub adler: uLong,
    pub reserved: uLong,
}
pub type z_stream = z_stream_s;
pub type z_streamp = *mut z_stream;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gz_header_s {
    pub text: ::core::ffi::c_int,
    pub time: uLong,
    pub xflags: ::core::ffi::c_int,
    pub os: ::core::ffi::c_int,
    pub extra: *mut Bytef,
    pub extra_len: uInt,
    pub extra_max: uInt,
    pub name: *mut Bytef,
    pub name_max: uInt,
    pub comment: *mut Bytef,
    pub comm_max: uInt,
    pub hcrc: ::core::ffi::c_int,
    pub done: ::core::ffi::c_int,
}
pub type gz_header = gz_header_s;
pub type gz_headerp = *mut gz_header;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct code {
    pub op: ::core::ffi::c_uchar,
    pub bits: ::core::ffi::c_uchar,
    pub val: ::core::ffi::c_ushort,
}
pub type inflate_mode = ::core::ffi::c_uint;
pub const SYNC: inflate_mode = 16211;
pub const MEM: inflate_mode = 16210;
pub const BAD: inflate_mode = 16209;
pub const DONE: inflate_mode = 16208;
pub const LENGTH: inflate_mode = 16207;
pub const CHECK: inflate_mode = 16206;
pub const LIT: inflate_mode = 16205;
pub const MATCH: inflate_mode = 16204;
pub const DISTEXT: inflate_mode = 16203;
pub const DIST: inflate_mode = 16202;
pub const LENEXT: inflate_mode = 16201;
pub const LEN: inflate_mode = 16200;
pub const LEN_: inflate_mode = 16199;
pub const CODELENS: inflate_mode = 16198;
pub const LENLENS: inflate_mode = 16197;
pub const TABLE: inflate_mode = 16196;
pub const COPY: inflate_mode = 16195;
pub const COPY_: inflate_mode = 16194;
pub const STORED: inflate_mode = 16193;
pub const TYPEDO: inflate_mode = 16192;
pub const TYPE: inflate_mode = 16191;
pub const DICT: inflate_mode = 16190;
pub const DICTID: inflate_mode = 16189;
pub const HCRC: inflate_mode = 16188;
pub const COMMENT: inflate_mode = 16187;
pub const NAME: inflate_mode = 16186;
pub const EXTRA: inflate_mode = 16185;
pub const EXLEN: inflate_mode = 16184;
pub const OS: inflate_mode = 16183;
pub const TIME: inflate_mode = 16182;
pub const FLAGS: inflate_mode = 16181;
pub const HEAD: inflate_mode = 16180;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct inflate_state {
    pub strm: z_streamp,
    pub mode: inflate_mode,
    pub last: ::core::ffi::c_int,
    pub wrap: ::core::ffi::c_int,
    pub havedict: ::core::ffi::c_int,
    pub flags: ::core::ffi::c_int,
    pub dmax: ::core::ffi::c_uint,
    pub check: ::core::ffi::c_ulong,
    pub total: ::core::ffi::c_ulong,
    pub head: gz_headerp,
    pub wbits: ::core::ffi::c_uint,
    pub wsize: ::core::ffi::c_uint,
    pub whave: ::core::ffi::c_uint,
    pub wnext: ::core::ffi::c_uint,
    pub window: *mut ::core::ffi::c_uchar,
    pub hold: ::core::ffi::c_ulong,
    pub bits: ::core::ffi::c_uint,
    pub length: ::core::ffi::c_uint,
    pub offset: ::core::ffi::c_uint,
    pub extra: ::core::ffi::c_uint,
    pub lencode: *const code,
    pub distcode: *const code,
    pub lenbits: ::core::ffi::c_uint,
    pub distbits: ::core::ffi::c_uint,
    pub ncode: ::core::ffi::c_uint,
    pub nlen: ::core::ffi::c_uint,
    pub ndist: ::core::ffi::c_uint,
    pub have: ::core::ffi::c_uint,
    pub next: *mut code,
    pub lens: [::core::ffi::c_ushort; 320],
    pub work: [::core::ffi::c_ushort; 288],
    pub codes: [code; 1444],
    pub sane: ::core::ffi::c_int,
    pub back: ::core::ffi::c_int,
    pub was: ::core::ffi::c_uint,
}

#[inline(always)]
unsafe fn inflate_fast_copy_match(
    mut out: *mut ::core::ffi::c_uchar,
    dist: ::core::ffi::c_uint,
    mut len: ::core::ffi::c_uint,
) -> *mut ::core::ffi::c_uchar {
    if len == 0 {
        return out;
    }
    if dist == 1 as ::core::ffi::c_uint {
        ::core::ptr::write_bytes(out, *out.offset(-(1 as isize)), len as usize);
        return out.offset(len as isize);
    }
    while len > dist {
        ::core::ptr::copy_nonoverlapping(
            out.offset(-(dist as isize)),
            out,
            dist as usize,
        );
        out = out.offset(dist as isize);
        len = len.wrapping_sub(dist);
    }
    ::core::ptr::copy_nonoverlapping(out.offset(-(dist as isize)), out, len as usize);
    out.offset(len as isize)
}

#[no_mangle]
pub unsafe extern "C" fn inflate_fast(mut strm: z_streamp, mut start: ::core::ffi::c_uint) {
    let mut state: *mut inflate_state = ::core::ptr::null_mut::<inflate_state>();
    let mut in_0: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut last: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut out: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut beg: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut end: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut wsize: ::core::ffi::c_uint = 0;
    let mut whave: ::core::ffi::c_uint = 0;
    let mut wnext: ::core::ffi::c_uint = 0;
    let mut window: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut hold: ::core::ffi::c_ulong = 0;
    let mut bits: ::core::ffi::c_uint = 0;
    let mut lcode: *const code = ::core::ptr::null::<code>();
    let mut dcode: *const code = ::core::ptr::null::<code>();
    let mut lmask: ::core::ffi::c_uint = 0;
    let mut dmask: ::core::ffi::c_uint = 0;
    let mut here: code = code {
        op: 0,
        bits: 0,
        val: 0,
    };
    let mut op: ::core::ffi::c_uint = 0;
    let mut len: ::core::ffi::c_uint = 0;
    let mut dist: ::core::ffi::c_uint = 0;
    let mut from: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    state = (*strm).state as *mut inflate_state;
    in_0 = (*strm).next_in as *mut ::core::ffi::c_uchar;
    last = in_0.offset(
        ((*strm).avail_in as ::core::ffi::c_uint).wrapping_sub(5 as ::core::ffi::c_uint) as isize,
    );
    out = (*strm).next_out as *mut ::core::ffi::c_uchar;
    beg = out.offset(-((start as uInt).wrapping_sub((*strm).avail_out) as isize));
    end = out.offset(
        ((*strm).avail_out as ::core::ffi::c_uint).wrapping_sub(257 as ::core::ffi::c_uint)
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
    lmask = ((1 as ::core::ffi::c_uint) << (*state).lenbits).wrapping_sub(1 as ::core::ffi::c_uint);
    dmask =
        ((1 as ::core::ffi::c_uint) << (*state).distbits).wrapping_sub(1 as ::core::ffi::c_uint);
    let mut current_block_141: u64;
    's_94: loop {
        if bits < 15 as ::core::ffi::c_uint {
            let fresh0 = in_0;
            in_0 = in_0.offset(1);
            hold = hold.wrapping_add((*fresh0 as ::core::ffi::c_ulong) << bits);
            bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
            let fresh1 = in_0;
            in_0 = in_0.offset(1);
            hold = hold.wrapping_add((*fresh1 as ::core::ffi::c_ulong) << bits);
            bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
        }
        here = *lcode.offset((hold & lmask as ::core::ffi::c_ulong) as isize);
        loop {
            op = here.bits as ::core::ffi::c_uint;
            hold >>= op;
            bits = bits.wrapping_sub(op);
            op = here.op as ::core::ffi::c_uint;
            if op == 0 as ::core::ffi::c_uint {
                let fresh2 = out;
                out = out.offset(1);
                *fresh2 = here.val as ::core::ffi::c_uchar;
                current_block_141 = 5689001924483802034;
                break;
            } else if op & 16 as ::core::ffi::c_uint != 0 {
                len = here.val as ::core::ffi::c_uint;
                op &= 15 as ::core::ffi::c_uint;
                if op != 0 {
                    if bits < op {
                        let fresh3 = in_0;
                        in_0 = in_0.offset(1);
                        hold = hold.wrapping_add((*fresh3 as ::core::ffi::c_ulong) << bits);
                        bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                    }
                    len = len.wrapping_add(
                        hold as ::core::ffi::c_uint
                            & ((1 as ::core::ffi::c_uint) << op)
                                .wrapping_sub(1 as ::core::ffi::c_uint),
                    );
                    hold >>= op;
                    bits = bits.wrapping_sub(op);
                }
                if bits < 15 as ::core::ffi::c_uint {
                    let fresh4 = in_0;
                    in_0 = in_0.offset(1);
                    hold = hold.wrapping_add((*fresh4 as ::core::ffi::c_ulong) << bits);
                    bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                    let fresh5 = in_0;
                    in_0 = in_0.offset(1);
                    hold = hold.wrapping_add((*fresh5 as ::core::ffi::c_ulong) << bits);
                    bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                }
                here = *dcode.offset((hold & dmask as ::core::ffi::c_ulong) as isize);
                current_block_141 = 4617688975773714446;
                break;
            } else if op & 64 as ::core::ffi::c_uint == 0 as ::core::ffi::c_uint {
                here = *lcode.offset((here.val as ::core::ffi::c_ulong).wrapping_add(
                    hold & ((1 as ::core::ffi::c_uint) << op).wrapping_sub(1 as ::core::ffi::c_uint)
                        as ::core::ffi::c_ulong,
                ) as isize);
            } else if op & 32 as ::core::ffi::c_uint != 0 {
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
                    op = here.bits as ::core::ffi::c_uint;
                    hold >>= op;
                    bits = bits.wrapping_sub(op);
                    op = here.op as ::core::ffi::c_uint;
                    if op & 16 as ::core::ffi::c_uint != 0 {
                        dist = here.val as ::core::ffi::c_uint;
                        op &= 15 as ::core::ffi::c_uint;
                        if bits < op {
                            let fresh6 = in_0;
                            in_0 = in_0.offset(1);
                            hold = hold.wrapping_add((*fresh6 as ::core::ffi::c_ulong) << bits);
                            bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                            if bits < op {
                                let fresh7 = in_0;
                                in_0 = in_0.offset(1);
                                hold = hold.wrapping_add((*fresh7 as ::core::ffi::c_ulong) << bits);
                                bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                            }
                        }
                        dist = dist.wrapping_add(
                            hold as ::core::ffi::c_uint
                                & ((1 as ::core::ffi::c_uint) << op)
                                    .wrapping_sub(1 as ::core::ffi::c_uint),
                        );
                        hold >>= op;
                        bits = bits.wrapping_sub(op);
                        op = out.offset_from(beg) as ::core::ffi::c_long as ::core::ffi::c_uint;
                        if dist > op {
                            current_block_141 = 5235537862154438448;
                            break;
                        } else {
                            current_block_141 = 6072622540298447352;
                            break;
                        }
                    } else if op & 64 as ::core::ffi::c_uint == 0 as ::core::ffi::c_uint {
                        here = *dcode.offset(
                            (here.val as ::core::ffi::c_ulong).wrapping_add(
                                hold & ((1 as ::core::ffi::c_uint) << op)
                                    .wrapping_sub(1 as ::core::ffi::c_uint)
                                    as ::core::ffi::c_ulong,
                            ) as isize,
                        );
                    } else {
                        (*strm).msg = b"invalid distance code\0" as *const u8
                            as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char;
                        (*state).mode = BAD;
                        break 's_94;
                    }
                }
                match current_block_141 {
                    6072622540298447352 => {
                        out = inflate_fast_copy_match(out, dist, len);
                    }
                    _ => {
                        op = dist.wrapping_sub(op);
                        if op > whave {
                            if (*state).sane != 0 {
                                (*strm).msg = b"invalid distance too far back\0" as *const u8
                                    as *const ::core::ffi::c_char
                                    as *mut ::core::ffi::c_char;
                                (*state).mode = BAD;
                                break;
                            }
                        }
                        from = window;
                        if wnext == 0 as ::core::ffi::c_uint {
                            from = from.offset(wsize.wrapping_sub(op) as isize);
                            if op < len {
                                len = len.wrapping_sub(op);
                                ::core::ptr::copy_nonoverlapping(from, out, op as usize);
                                out = out.offset(op as isize);
                            } else {
                                ::core::ptr::copy_nonoverlapping(from, out, len as usize);
                                out = out.offset(len as isize);
                                len = 0 as ::core::ffi::c_uint;
                            }
                        } else if wnext < op {
                            from = from.offset(wsize.wrapping_add(wnext).wrapping_sub(op) as isize);
                            op = op.wrapping_sub(wnext);
                            if op < len {
                                len = len.wrapping_sub(op);
                                ::core::ptr::copy_nonoverlapping(from, out, op as usize);
                                out = out.offset(op as isize);
                                from = window;
                                if wnext < len {
                                    op = wnext;
                                    len = len.wrapping_sub(op);
                                    ::core::ptr::copy_nonoverlapping(from, out, op as usize);
                                    out = out.offset(op as isize);
                                } else {
                                    ::core::ptr::copy_nonoverlapping(from, out, len as usize);
                                    out = out.offset(len as isize);
                                    len = 0 as ::core::ffi::c_uint;
                                }
                            } else {
                                ::core::ptr::copy_nonoverlapping(from, out, len as usize);
                                out = out.offset(len as isize);
                                len = 0 as ::core::ffi::c_uint;
                            }
                        } else {
                            from = from.offset(wnext.wrapping_sub(op) as isize);
                            if op < len {
                                len = len.wrapping_sub(op);
                                ::core::ptr::copy_nonoverlapping(from, out, op as usize);
                                out = out.offset(op as isize);
                            } else {
                                ::core::ptr::copy_nonoverlapping(from, out, len as usize);
                                out = out.offset(len as isize);
                                len = 0 as ::core::ffi::c_uint;
                            }
                        }
                        if len != 0 {
                            out = inflate_fast_copy_match(out, dist, len);
                        }
                    }
                }
            }
            9180031981464905198 => {
                (*strm).msg = b"invalid literal/length code\0" as *const u8
                    as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char;
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
    len = bits >> 3 as ::core::ffi::c_int;
    in_0 = in_0.offset(-(len as isize));
    bits = bits.wrapping_sub(len << 3 as ::core::ffi::c_int);
    hold &= ((1 as ::core::ffi::c_uint) << bits).wrapping_sub(1 as ::core::ffi::c_uint)
        as ::core::ffi::c_ulong;
    (*strm).next_in = in_0 as *mut Bytef;
    (*strm).next_out = out as *mut Bytef;
    (*strm).avail_in = (if in_0 < last {
        5 as ::core::ffi::c_long + last.offset_from(in_0) as ::core::ffi::c_long
    } else {
        5 as ::core::ffi::c_long - in_0.offset_from(last) as ::core::ffi::c_long
    }) as ::core::ffi::c_uint as uInt;
    (*strm).avail_out = (if out < end {
        257 as ::core::ffi::c_long + end.offset_from(out) as ::core::ffi::c_long
    } else {
        257 as ::core::ffi::c_long - out.offset_from(end) as ::core::ffi::c_long
    }) as ::core::ffi::c_uint as uInt;
    (*state).hold = hold;
    (*state).bits = bits;
}
