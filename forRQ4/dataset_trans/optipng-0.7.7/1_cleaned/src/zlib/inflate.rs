use core::ffi::*;
pub use crate::src::ffi::*;
pub use crate::src::c_consts::*;
pub use crate::src::c_structs::*;
pub use crate::src::c_types::*;
extern "C" {
    pub type internal_state;
    fn inflate_fast(strm: z_streamp, start: c_uint);
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

pub const ZLIB_VERSION: [c_char; 15] =
    unsafe { ::core::mem::transmute::<[u8; 15], [c_char; 15]>(*b"1.2.11-optipng\0") };

pub const Z_TREES: c_int = 6 as c_int;
pub const Z_OK: c_int = 0 as c_int;
pub const Z_STREAM_END: c_int = 1 as c_int;
pub const Z_NEED_DICT: c_int = 2 as c_int;

pub const Z_DATA_ERROR: c_int = -(3 as c_int);
pub const Z_MEM_ERROR: c_int = -(4 as c_int);
pub const Z_BUF_ERROR: c_int = -(5 as c_int);
pub const Z_VERSION_ERROR: c_int = -(6 as c_int);

pub const DEF_WBITS: c_int = MAX_WBITS;

pub const ENOUGH: c_int = ENOUGH_LENS + ENOUGH_DISTS;
unsafe extern "C" fn inflateStateCheck(mut strm: z_streamp) -> c_int {
    let mut state: *mut inflate_state = ::core::ptr::null_mut::<inflate_state>();
    if strm.is_null() || (*strm).zalloc.is_none() || (*strm).zfree.is_none() {
        return 1 as c_int;
    }
    state = (*strm).state as *mut inflate_state;
    if state.is_null()
        || (*state).strm != strm
        || ((*state).mode as c_uint)
            < HEAD as c_int as c_uint
        || (*state).mode as c_uint > SYNC as c_int as c_uint
    {
        return 1 as c_int;
    }
    return 0 as c_int;
}
#[no_mangle]
pub unsafe extern "C" fn inflateResetKeep(mut strm: z_streamp) -> c_int {
    let mut state: *mut inflate_state = ::core::ptr::null_mut::<inflate_state>();
    if inflateStateCheck(strm) != 0 {
        return Z_STREAM_ERROR;
    }
    state = (*strm).state as *mut inflate_state;
    (*state).total = 0 as c_ulong;
    (*strm).total_out = (*state).total as uLong;
    (*strm).total_in = (*strm).total_out;
    (*strm).msg = ::core::ptr::null_mut::<c_char>();
    if (*state).wrap != 0 {
        (*strm).adler = ((*state).wrap & 1 as c_int) as uLong;
    }
    (*state).mode = HEAD;
    (*state).last = 0 as c_int;
    (*state).havedict = 0 as c_int;
    (*state).dmax = 32768 as c_uint;
    (*state).head = ::core::ptr::null_mut::<gz_header>();
    (*state).hold = 0 as c_ulong;
    (*state).bits = 0 as c_uint;
    (*state).next = &raw mut (*state).codes as *mut code;
    (*state).distcode = (*state).next;
    (*state).lencode = (*state).distcode;
    (*state).sane = 1 as c_int;
    (*state).back = -(1 as c_int);
    return Z_OK;
}
#[no_mangle]
pub unsafe extern "C" fn inflateReset(mut strm: z_streamp) -> c_int {
    let mut state: *mut inflate_state = ::core::ptr::null_mut::<inflate_state>();
    if inflateStateCheck(strm) != 0 {
        return Z_STREAM_ERROR;
    }
    state = (*strm).state as *mut inflate_state;
    (*state).wsize = 0 as c_uint;
    (*state).whave = 0 as c_uint;
    (*state).wnext = 0 as c_uint;
    return inflateResetKeep(strm);
}
#[no_mangle]
pub unsafe extern "C" fn inflateReset2(
    mut strm: z_streamp,
    mut windowBits: c_int,
) -> c_int {
    let mut wrap: c_int = 0;
    let mut state: *mut inflate_state = ::core::ptr::null_mut::<inflate_state>();
    if inflateStateCheck(strm) != 0 {
        return Z_STREAM_ERROR;
    }
    state = (*strm).state as *mut inflate_state;
    if windowBits < 0 as c_int {
        wrap = 0 as c_int;
        windowBits = -windowBits;
    } else {
        wrap = (windowBits >> 4 as c_int) + 5 as c_int;
    }
    if windowBits != 0
        && (windowBits < 8 as c_int || windowBits > 15 as c_int)
    {
        return Z_STREAM_ERROR;
    }
    if !(*state).window.is_null() && (*state).wbits != windowBits as c_uint {
        Some((*strm).zfree.expect("non-null function pointer")).expect("non-null function pointer")(
            (*strm).opaque,
            (*state).window as voidpf,
        );
        (*state).window = ::core::ptr::null_mut::<c_uchar>();
    }
    (*state).wrap = wrap;
    (*state).wbits = windowBits as c_uint;
    return inflateReset(strm);
}
#[no_mangle]
pub unsafe extern "C" fn inflateInit2_(
    mut strm: z_streamp,
    mut windowBits: c_int,
    mut version: *const c_char,
    mut stream_size: c_int,
) -> c_int {
    let mut ret: c_int = 0;
    let mut state: *mut inflate_state = ::core::ptr::null_mut::<inflate_state>();
    if version.is_null()
        || *version.offset(0 as c_int as isize) as c_int
            != ZLIB_VERSION[0 as c_int as usize] as c_int
        || stream_size != ::core::mem::size_of::<z_stream>() as c_int
    {
        return Z_VERSION_ERROR;
    }
    if strm.is_null() {
        return Z_STREAM_ERROR;
    }
    (*strm).msg = ::core::ptr::null_mut::<c_char>();
    if (*strm).zalloc.is_none() {
        (*strm).zalloc = Some(
            zcalloc
                as unsafe extern "C" fn(voidpf, c_uint, c_uint) -> voidpf,
        ) as alloc_func;
        (*strm).opaque = ::core::ptr::null_mut::<c_void>();
    }
    if (*strm).zfree.is_none() {
        (*strm).zfree = Some(zcfree as unsafe extern "C" fn(voidpf, voidpf) -> ()) as free_func;
    }
    state = Some((*strm).zalloc.expect("non-null function pointer"))
        .expect("non-null function pointer")(
        (*strm).opaque,
        1 as uInt,
        ::core::mem::size_of::<inflate_state>() as uInt,
    ) as *mut inflate_state;
    if state.is_null() {
        return Z_MEM_ERROR;
    }
    (*strm).state = state as *mut internal_state;
    (*state).strm = strm;
    (*state).window = ::core::ptr::null_mut::<c_uchar>();
    (*state).mode = HEAD;
    ret = inflateReset2(strm, windowBits);
    if ret != Z_OK {
        Some((*strm).zfree.expect("non-null function pointer")).expect("non-null function pointer")(
            (*strm).opaque,
            state as voidpf,
        );
        (*strm).state = ::core::ptr::null_mut::<internal_state>();
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn inflateInit_(
    mut strm: z_streamp,
    mut version: *const c_char,
    mut stream_size: c_int,
) -> c_int {
    return inflateInit2_(strm, DEF_WBITS, version, stream_size);
}
#[no_mangle]
pub unsafe extern "C" fn inflatePrime(
    mut strm: z_streamp,
    mut bits: c_int,
    mut value: c_int,
) -> c_int {
    let mut state: *mut inflate_state = ::core::ptr::null_mut::<inflate_state>();
    if inflateStateCheck(strm) != 0 {
        return Z_STREAM_ERROR;
    }
    state = (*strm).state as *mut inflate_state;
    if bits < 0 as c_int {
        (*state).hold = 0 as c_ulong;
        (*state).bits = 0 as c_uint;
        return Z_OK;
    }
    if bits > 16 as c_int
        || ((*state).bits as uInt).wrapping_add(bits as uInt) > 32 as c_uint
    {
        return Z_STREAM_ERROR;
    }
    value = (value as c_long
        & ((1 as c_long) << bits) - 1 as c_long)
        as c_int;
    (*state).hold = (*state)
        .hold
        .wrapping_add(((value as c_uint) << (*state).bits) as c_ulong);
    (*state).bits = (*state)
        .bits
        .wrapping_add(bits as uInt as c_uint);
    return Z_OK;
}
unsafe extern "C" fn fixedtables(mut state: *mut inflate_state) {
    static mut lenfix: [code; 512] = [
        code {
            op: 96 as c_uchar,
            bits: 7 as c_uchar,
            val: 0 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 80 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 16 as c_ushort,
        },
        code {
            op: 20 as c_uchar,
            bits: 8 as c_uchar,
            val: 115 as c_ushort,
        },
        code {
            op: 18 as c_uchar,
            bits: 7 as c_uchar,
            val: 31 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 112 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 48 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 9 as c_uchar,
            val: 192 as c_ushort,
        },
        code {
            op: 16 as c_uchar,
            bits: 7 as c_uchar,
            val: 10 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 96 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 32 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 9 as c_uchar,
            val: 160 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 0 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 128 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 64 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 9 as c_uchar,
            val: 224 as c_ushort,
        },
        code {
            op: 16 as c_uchar,
            bits: 7 as c_uchar,
            val: 6 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 88 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 24 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 9 as c_uchar,
            val: 144 as c_ushort,
        },
        code {
            op: 19 as c_uchar,
            bits: 7 as c_uchar,
            val: 59 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 120 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 56 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 9 as c_uchar,
            val: 208 as c_ushort,
        },
        code {
            op: 17 as c_uchar,
            bits: 7 as c_uchar,
            val: 17 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 104 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 40 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 9 as c_uchar,
            val: 176 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 8 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 136 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 72 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 9 as c_uchar,
            val: 240 as c_ushort,
        },
        code {
            op: 16 as c_uchar,
            bits: 7 as c_uchar,
            val: 4 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 84 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 20 as c_ushort,
        },
        code {
            op: 21 as c_uchar,
            bits: 8 as c_uchar,
            val: 227 as c_ushort,
        },
        code {
            op: 19 as c_uchar,
            bits: 7 as c_uchar,
            val: 43 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 116 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 52 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 9 as c_uchar,
            val: 200 as c_ushort,
        },
        code {
            op: 17 as c_uchar,
            bits: 7 as c_uchar,
            val: 13 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 100 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 36 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 9 as c_uchar,
            val: 168 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 4 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 132 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 68 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 9 as c_uchar,
            val: 232 as c_ushort,
        },
        code {
            op: 16 as c_uchar,
            bits: 7 as c_uchar,
            val: 8 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 92 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 28 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 9 as c_uchar,
            val: 152 as c_ushort,
        },
        code {
            op: 20 as c_uchar,
            bits: 7 as c_uchar,
            val: 83 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 124 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 60 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 9 as c_uchar,
            val: 216 as c_ushort,
        },
        code {
            op: 18 as c_uchar,
            bits: 7 as c_uchar,
            val: 23 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 108 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 44 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 9 as c_uchar,
            val: 184 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 12 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 140 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 76 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 9 as c_uchar,
            val: 248 as c_ushort,
        },
        code {
            op: 16 as c_uchar,
            bits: 7 as c_uchar,
            val: 3 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 82 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 18 as c_ushort,
        },
        code {
            op: 21 as c_uchar,
            bits: 8 as c_uchar,
            val: 163 as c_ushort,
        },
        code {
            op: 19 as c_uchar,
            bits: 7 as c_uchar,
            val: 35 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 114 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 50 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 9 as c_uchar,
            val: 196 as c_ushort,
        },
        code {
            op: 17 as c_uchar,
            bits: 7 as c_uchar,
            val: 11 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 98 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 34 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 9 as c_uchar,
            val: 164 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 2 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 130 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 66 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 9 as c_uchar,
            val: 228 as c_ushort,
        },
        code {
            op: 16 as c_uchar,
            bits: 7 as c_uchar,
            val: 7 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 90 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 26 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 9 as c_uchar,
            val: 148 as c_ushort,
        },
        code {
            op: 20 as c_uchar,
            bits: 7 as c_uchar,
            val: 67 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 122 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 58 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 9 as c_uchar,
            val: 212 as c_ushort,
        },
        code {
            op: 18 as c_uchar,
            bits: 7 as c_uchar,
            val: 19 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 106 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 42 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 9 as c_uchar,
            val: 180 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 10 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 138 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 74 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 9 as c_uchar,
            val: 244 as c_ushort,
        },
        code {
            op: 16 as c_uchar,
            bits: 7 as c_uchar,
            val: 5 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 86 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 22 as c_ushort,
        },
        code {
            op: 64 as c_uchar,
            bits: 8 as c_uchar,
            val: 0 as c_ushort,
        },
        code {
            op: 19 as c_uchar,
            bits: 7 as c_uchar,
            val: 51 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 118 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 54 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 9 as c_uchar,
            val: 204 as c_ushort,
        },
        code {
            op: 17 as c_uchar,
            bits: 7 as c_uchar,
            val: 15 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 102 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 38 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 9 as c_uchar,
            val: 172 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 6 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 134 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 70 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 9 as c_uchar,
            val: 236 as c_ushort,
        },
        code {
            op: 16 as c_uchar,
            bits: 7 as c_uchar,
            val: 9 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 94 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 30 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 9 as c_uchar,
            val: 156 as c_ushort,
        },
        code {
            op: 20 as c_uchar,
            bits: 7 as c_uchar,
            val: 99 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 126 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 62 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 9 as c_uchar,
            val: 220 as c_ushort,
        },
        code {
            op: 18 as c_uchar,
            bits: 7 as c_uchar,
            val: 27 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 110 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 46 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 9 as c_uchar,
            val: 188 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 14 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 142 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 78 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 9 as c_uchar,
            val: 252 as c_ushort,
        },
        code {
            op: 96 as c_uchar,
            bits: 7 as c_uchar,
            val: 0 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 81 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 17 as c_ushort,
        },
        code {
            op: 21 as c_uchar,
            bits: 8 as c_uchar,
            val: 131 as c_ushort,
        },
        code {
            op: 18 as c_uchar,
            bits: 7 as c_uchar,
            val: 31 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 113 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 49 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 9 as c_uchar,
            val: 194 as c_ushort,
        },
        code {
            op: 16 as c_uchar,
            bits: 7 as c_uchar,
            val: 10 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 97 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 33 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 9 as c_uchar,
            val: 162 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 1 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 129 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 65 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 9 as c_uchar,
            val: 226 as c_ushort,
        },
        code {
            op: 16 as c_uchar,
            bits: 7 as c_uchar,
            val: 6 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 89 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 25 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 9 as c_uchar,
            val: 146 as c_ushort,
        },
        code {
            op: 19 as c_uchar,
            bits: 7 as c_uchar,
            val: 59 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 121 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 57 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 9 as c_uchar,
            val: 210 as c_ushort,
        },
        code {
            op: 17 as c_uchar,
            bits: 7 as c_uchar,
            val: 17 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 105 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 41 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 9 as c_uchar,
            val: 178 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 9 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 137 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 73 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 9 as c_uchar,
            val: 242 as c_ushort,
        },
        code {
            op: 16 as c_uchar,
            bits: 7 as c_uchar,
            val: 4 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 85 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 21 as c_ushort,
        },
        code {
            op: 16 as c_uchar,
            bits: 8 as c_uchar,
            val: 258 as c_ushort,
        },
        code {
            op: 19 as c_uchar,
            bits: 7 as c_uchar,
            val: 43 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 117 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 53 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 9 as c_uchar,
            val: 202 as c_ushort,
        },
        code {
            op: 17 as c_uchar,
            bits: 7 as c_uchar,
            val: 13 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 101 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 37 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 9 as c_uchar,
            val: 170 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 5 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 133 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 69 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 9 as c_uchar,
            val: 234 as c_ushort,
        },
        code {
            op: 16 as c_uchar,
            bits: 7 as c_uchar,
            val: 8 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 93 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 29 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 9 as c_uchar,
            val: 154 as c_ushort,
        },
        code {
            op: 20 as c_uchar,
            bits: 7 as c_uchar,
            val: 83 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 125 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 61 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 9 as c_uchar,
            val: 218 as c_ushort,
        },
        code {
            op: 18 as c_uchar,
            bits: 7 as c_uchar,
            val: 23 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 109 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 45 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 9 as c_uchar,
            val: 186 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 13 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 141 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 77 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 9 as c_uchar,
            val: 250 as c_ushort,
        },
        code {
            op: 16 as c_uchar,
            bits: 7 as c_uchar,
            val: 3 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 83 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 19 as c_ushort,
        },
        code {
            op: 21 as c_uchar,
            bits: 8 as c_uchar,
            val: 195 as c_ushort,
        },
        code {
            op: 19 as c_uchar,
            bits: 7 as c_uchar,
            val: 35 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 115 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 51 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 9 as c_uchar,
            val: 198 as c_ushort,
        },
        code {
            op: 17 as c_uchar,
            bits: 7 as c_uchar,
            val: 11 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 99 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 35 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 9 as c_uchar,
            val: 166 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 3 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 131 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 67 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 9 as c_uchar,
            val: 230 as c_ushort,
        },
        code {
            op: 16 as c_uchar,
            bits: 7 as c_uchar,
            val: 7 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 91 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 27 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 9 as c_uchar,
            val: 150 as c_ushort,
        },
        code {
            op: 20 as c_uchar,
            bits: 7 as c_uchar,
            val: 67 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 123 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 59 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 9 as c_uchar,
            val: 214 as c_ushort,
        },
        code {
            op: 18 as c_uchar,
            bits: 7 as c_uchar,
            val: 19 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 107 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 43 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 9 as c_uchar,
            val: 182 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 11 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 139 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 75 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 9 as c_uchar,
            val: 246 as c_ushort,
        },
        code {
            op: 16 as c_uchar,
            bits: 7 as c_uchar,
            val: 5 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 87 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 23 as c_ushort,
        },
        code {
            op: 64 as c_uchar,
            bits: 8 as c_uchar,
            val: 0 as c_ushort,
        },
        code {
            op: 19 as c_uchar,
            bits: 7 as c_uchar,
            val: 51 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 119 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 55 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 9 as c_uchar,
            val: 206 as c_ushort,
        },
        code {
            op: 17 as c_uchar,
            bits: 7 as c_uchar,
            val: 15 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 103 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 39 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 9 as c_uchar,
            val: 174 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 7 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 135 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 71 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 9 as c_uchar,
            val: 238 as c_ushort,
        },
        code {
            op: 16 as c_uchar,
            bits: 7 as c_uchar,
            val: 9 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 95 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 31 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 9 as c_uchar,
            val: 158 as c_ushort,
        },
        code {
            op: 20 as c_uchar,
            bits: 7 as c_uchar,
            val: 99 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 127 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 63 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 9 as c_uchar,
            val: 222 as c_ushort,
        },
        code {
            op: 18 as c_uchar,
            bits: 7 as c_uchar,
            val: 27 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 111 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 47 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 9 as c_uchar,
            val: 190 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 15 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 143 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 79 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 9 as c_uchar,
            val: 254 as c_ushort,
        },
        code {
            op: 96 as c_uchar,
            bits: 7 as c_uchar,
            val: 0 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 80 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 16 as c_ushort,
        },
        code {
            op: 20 as c_uchar,
            bits: 8 as c_uchar,
            val: 115 as c_ushort,
        },
        code {
            op: 18 as c_uchar,
            bits: 7 as c_uchar,
            val: 31 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 112 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 48 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 9 as c_uchar,
            val: 193 as c_ushort,
        },
        code {
            op: 16 as c_uchar,
            bits: 7 as c_uchar,
            val: 10 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 96 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 32 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 9 as c_uchar,
            val: 161 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 0 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 128 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 64 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 9 as c_uchar,
            val: 225 as c_ushort,
        },
        code {
            op: 16 as c_uchar,
            bits: 7 as c_uchar,
            val: 6 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 88 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 24 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 9 as c_uchar,
            val: 145 as c_ushort,
        },
        code {
            op: 19 as c_uchar,
            bits: 7 as c_uchar,
            val: 59 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 120 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 56 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 9 as c_uchar,
            val: 209 as c_ushort,
        },
        code {
            op: 17 as c_uchar,
            bits: 7 as c_uchar,
            val: 17 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 104 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 40 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 9 as c_uchar,
            val: 177 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 8 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 136 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 72 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 9 as c_uchar,
            val: 241 as c_ushort,
        },
        code {
            op: 16 as c_uchar,
            bits: 7 as c_uchar,
            val: 4 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 84 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 20 as c_ushort,
        },
        code {
            op: 21 as c_uchar,
            bits: 8 as c_uchar,
            val: 227 as c_ushort,
        },
        code {
            op: 19 as c_uchar,
            bits: 7 as c_uchar,
            val: 43 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 116 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 52 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 9 as c_uchar,
            val: 201 as c_ushort,
        },
        code {
            op: 17 as c_uchar,
            bits: 7 as c_uchar,
            val: 13 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 100 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 36 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 9 as c_uchar,
            val: 169 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 4 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 132 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 68 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 9 as c_uchar,
            val: 233 as c_ushort,
        },
        code {
            op: 16 as c_uchar,
            bits: 7 as c_uchar,
            val: 8 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 92 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 28 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 9 as c_uchar,
            val: 153 as c_ushort,
        },
        code {
            op: 20 as c_uchar,
            bits: 7 as c_uchar,
            val: 83 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 124 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 60 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 9 as c_uchar,
            val: 217 as c_ushort,
        },
        code {
            op: 18 as c_uchar,
            bits: 7 as c_uchar,
            val: 23 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 108 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 44 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 9 as c_uchar,
            val: 185 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 12 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 140 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 76 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 9 as c_uchar,
            val: 249 as c_ushort,
        },
        code {
            op: 16 as c_uchar,
            bits: 7 as c_uchar,
            val: 3 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 82 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 18 as c_ushort,
        },
        code {
            op: 21 as c_uchar,
            bits: 8 as c_uchar,
            val: 163 as c_ushort,
        },
        code {
            op: 19 as c_uchar,
            bits: 7 as c_uchar,
            val: 35 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 114 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 50 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 9 as c_uchar,
            val: 197 as c_ushort,
        },
        code {
            op: 17 as c_uchar,
            bits: 7 as c_uchar,
            val: 11 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 98 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 34 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 9 as c_uchar,
            val: 165 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 2 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 130 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 66 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 9 as c_uchar,
            val: 229 as c_ushort,
        },
        code {
            op: 16 as c_uchar,
            bits: 7 as c_uchar,
            val: 7 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 90 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 26 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 9 as c_uchar,
            val: 149 as c_ushort,
        },
        code {
            op: 20 as c_uchar,
            bits: 7 as c_uchar,
            val: 67 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 122 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 58 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 9 as c_uchar,
            val: 213 as c_ushort,
        },
        code {
            op: 18 as c_uchar,
            bits: 7 as c_uchar,
            val: 19 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 106 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 42 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 9 as c_uchar,
            val: 181 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 10 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 138 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 74 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 9 as c_uchar,
            val: 245 as c_ushort,
        },
        code {
            op: 16 as c_uchar,
            bits: 7 as c_uchar,
            val: 5 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 86 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 22 as c_ushort,
        },
        code {
            op: 64 as c_uchar,
            bits: 8 as c_uchar,
            val: 0 as c_ushort,
        },
        code {
            op: 19 as c_uchar,
            bits: 7 as c_uchar,
            val: 51 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 118 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 54 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 9 as c_uchar,
            val: 205 as c_ushort,
        },
        code {
            op: 17 as c_uchar,
            bits: 7 as c_uchar,
            val: 15 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 102 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 38 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 9 as c_uchar,
            val: 173 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 6 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 134 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 70 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 9 as c_uchar,
            val: 237 as c_ushort,
        },
        code {
            op: 16 as c_uchar,
            bits: 7 as c_uchar,
            val: 9 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 94 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 30 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 9 as c_uchar,
            val: 157 as c_ushort,
        },
        code {
            op: 20 as c_uchar,
            bits: 7 as c_uchar,
            val: 99 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 126 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 62 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 9 as c_uchar,
            val: 221 as c_ushort,
        },
        code {
            op: 18 as c_uchar,
            bits: 7 as c_uchar,
            val: 27 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 110 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 46 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 9 as c_uchar,
            val: 189 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 14 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 142 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 78 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 9 as c_uchar,
            val: 253 as c_ushort,
        },
        code {
            op: 96 as c_uchar,
            bits: 7 as c_uchar,
            val: 0 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 81 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 17 as c_ushort,
        },
        code {
            op: 21 as c_uchar,
            bits: 8 as c_uchar,
            val: 131 as c_ushort,
        },
        code {
            op: 18 as c_uchar,
            bits: 7 as c_uchar,
            val: 31 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 113 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 49 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 9 as c_uchar,
            val: 195 as c_ushort,
        },
        code {
            op: 16 as c_uchar,
            bits: 7 as c_uchar,
            val: 10 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 97 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 33 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 9 as c_uchar,
            val: 163 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 1 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 129 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 65 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 9 as c_uchar,
            val: 227 as c_ushort,
        },
        code {
            op: 16 as c_uchar,
            bits: 7 as c_uchar,
            val: 6 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 89 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 25 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 9 as c_uchar,
            val: 147 as c_ushort,
        },
        code {
            op: 19 as c_uchar,
            bits: 7 as c_uchar,
            val: 59 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 121 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 57 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 9 as c_uchar,
            val: 211 as c_ushort,
        },
        code {
            op: 17 as c_uchar,
            bits: 7 as c_uchar,
            val: 17 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 105 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 41 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 9 as c_uchar,
            val: 179 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 9 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 137 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 73 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 9 as c_uchar,
            val: 243 as c_ushort,
        },
        code {
            op: 16 as c_uchar,
            bits: 7 as c_uchar,
            val: 4 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 85 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 21 as c_ushort,
        },
        code {
            op: 16 as c_uchar,
            bits: 8 as c_uchar,
            val: 258 as c_ushort,
        },
        code {
            op: 19 as c_uchar,
            bits: 7 as c_uchar,
            val: 43 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 117 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 53 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 9 as c_uchar,
            val: 203 as c_ushort,
        },
        code {
            op: 17 as c_uchar,
            bits: 7 as c_uchar,
            val: 13 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 101 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 37 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 9 as c_uchar,
            val: 171 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 5 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 133 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 69 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 9 as c_uchar,
            val: 235 as c_ushort,
        },
        code {
            op: 16 as c_uchar,
            bits: 7 as c_uchar,
            val: 8 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 93 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 29 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 9 as c_uchar,
            val: 155 as c_ushort,
        },
        code {
            op: 20 as c_uchar,
            bits: 7 as c_uchar,
            val: 83 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 125 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 61 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 9 as c_uchar,
            val: 219 as c_ushort,
        },
        code {
            op: 18 as c_uchar,
            bits: 7 as c_uchar,
            val: 23 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 109 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 45 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 9 as c_uchar,
            val: 187 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 13 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 141 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 77 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 9 as c_uchar,
            val: 251 as c_ushort,
        },
        code {
            op: 16 as c_uchar,
            bits: 7 as c_uchar,
            val: 3 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 83 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 19 as c_ushort,
        },
        code {
            op: 21 as c_uchar,
            bits: 8 as c_uchar,
            val: 195 as c_ushort,
        },
        code {
            op: 19 as c_uchar,
            bits: 7 as c_uchar,
            val: 35 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 115 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 51 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 9 as c_uchar,
            val: 199 as c_ushort,
        },
        code {
            op: 17 as c_uchar,
            bits: 7 as c_uchar,
            val: 11 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 99 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 35 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 9 as c_uchar,
            val: 167 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 3 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 131 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 67 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 9 as c_uchar,
            val: 231 as c_ushort,
        },
        code {
            op: 16 as c_uchar,
            bits: 7 as c_uchar,
            val: 7 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 91 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 27 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 9 as c_uchar,
            val: 151 as c_ushort,
        },
        code {
            op: 20 as c_uchar,
            bits: 7 as c_uchar,
            val: 67 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 123 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 59 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 9 as c_uchar,
            val: 215 as c_ushort,
        },
        code {
            op: 18 as c_uchar,
            bits: 7 as c_uchar,
            val: 19 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 107 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 43 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 9 as c_uchar,
            val: 183 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 11 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 139 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 75 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 9 as c_uchar,
            val: 247 as c_ushort,
        },
        code {
            op: 16 as c_uchar,
            bits: 7 as c_uchar,
            val: 5 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 87 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 23 as c_ushort,
        },
        code {
            op: 64 as c_uchar,
            bits: 8 as c_uchar,
            val: 0 as c_ushort,
        },
        code {
            op: 19 as c_uchar,
            bits: 7 as c_uchar,
            val: 51 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 119 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 55 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 9 as c_uchar,
            val: 207 as c_ushort,
        },
        code {
            op: 17 as c_uchar,
            bits: 7 as c_uchar,
            val: 15 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 103 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 39 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 9 as c_uchar,
            val: 175 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 7 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 135 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 71 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 9 as c_uchar,
            val: 239 as c_ushort,
        },
        code {
            op: 16 as c_uchar,
            bits: 7 as c_uchar,
            val: 9 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 95 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 31 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 9 as c_uchar,
            val: 159 as c_ushort,
        },
        code {
            op: 20 as c_uchar,
            bits: 7 as c_uchar,
            val: 99 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 127 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 63 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 9 as c_uchar,
            val: 223 as c_ushort,
        },
        code {
            op: 18 as c_uchar,
            bits: 7 as c_uchar,
            val: 27 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 111 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 47 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 9 as c_uchar,
            val: 191 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 15 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 143 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 8 as c_uchar,
            val: 79 as c_ushort,
        },
        code {
            op: 0 as c_uchar,
            bits: 9 as c_uchar,
            val: 255 as c_ushort,
        },
    ];
    static mut distfix: [code; 32] = [
        code {
            op: 16 as c_uchar,
            bits: 5 as c_uchar,
            val: 1 as c_ushort,
        },
        code {
            op: 23 as c_uchar,
            bits: 5 as c_uchar,
            val: 257 as c_ushort,
        },
        code {
            op: 19 as c_uchar,
            bits: 5 as c_uchar,
            val: 17 as c_ushort,
        },
        code {
            op: 27 as c_uchar,
            bits: 5 as c_uchar,
            val: 4097 as c_ushort,
        },
        code {
            op: 17 as c_uchar,
            bits: 5 as c_uchar,
            val: 5 as c_ushort,
        },
        code {
            op: 25 as c_uchar,
            bits: 5 as c_uchar,
            val: 1025 as c_ushort,
        },
        code {
            op: 21 as c_uchar,
            bits: 5 as c_uchar,
            val: 65 as c_ushort,
        },
        code {
            op: 29 as c_uchar,
            bits: 5 as c_uchar,
            val: 16385 as c_ushort,
        },
        code {
            op: 16 as c_uchar,
            bits: 5 as c_uchar,
            val: 3 as c_ushort,
        },
        code {
            op: 24 as c_uchar,
            bits: 5 as c_uchar,
            val: 513 as c_ushort,
        },
        code {
            op: 20 as c_uchar,
            bits: 5 as c_uchar,
            val: 33 as c_ushort,
        },
        code {
            op: 28 as c_uchar,
            bits: 5 as c_uchar,
            val: 8193 as c_ushort,
        },
        code {
            op: 18 as c_uchar,
            bits: 5 as c_uchar,
            val: 9 as c_ushort,
        },
        code {
            op: 26 as c_uchar,
            bits: 5 as c_uchar,
            val: 2049 as c_ushort,
        },
        code {
            op: 22 as c_uchar,
            bits: 5 as c_uchar,
            val: 129 as c_ushort,
        },
        code {
            op: 64 as c_uchar,
            bits: 5 as c_uchar,
            val: 0 as c_ushort,
        },
        code {
            op: 16 as c_uchar,
            bits: 5 as c_uchar,
            val: 2 as c_ushort,
        },
        code {
            op: 23 as c_uchar,
            bits: 5 as c_uchar,
            val: 385 as c_ushort,
        },
        code {
            op: 19 as c_uchar,
            bits: 5 as c_uchar,
            val: 25 as c_ushort,
        },
        code {
            op: 27 as c_uchar,
            bits: 5 as c_uchar,
            val: 6145 as c_ushort,
        },
        code {
            op: 17 as c_uchar,
            bits: 5 as c_uchar,
            val: 7 as c_ushort,
        },
        code {
            op: 25 as c_uchar,
            bits: 5 as c_uchar,
            val: 1537 as c_ushort,
        },
        code {
            op: 21 as c_uchar,
            bits: 5 as c_uchar,
            val: 97 as c_ushort,
        },
        code {
            op: 29 as c_uchar,
            bits: 5 as c_uchar,
            val: 24577 as c_ushort,
        },
        code {
            op: 16 as c_uchar,
            bits: 5 as c_uchar,
            val: 4 as c_ushort,
        },
        code {
            op: 24 as c_uchar,
            bits: 5 as c_uchar,
            val: 769 as c_ushort,
        },
        code {
            op: 20 as c_uchar,
            bits: 5 as c_uchar,
            val: 49 as c_ushort,
        },
        code {
            op: 28 as c_uchar,
            bits: 5 as c_uchar,
            val: 12289 as c_ushort,
        },
        code {
            op: 18 as c_uchar,
            bits: 5 as c_uchar,
            val: 13 as c_ushort,
        },
        code {
            op: 26 as c_uchar,
            bits: 5 as c_uchar,
            val: 3073 as c_ushort,
        },
        code {
            op: 22 as c_uchar,
            bits: 5 as c_uchar,
            val: 193 as c_ushort,
        },
        code {
            op: 64 as c_uchar,
            bits: 5 as c_uchar,
            val: 0 as c_ushort,
        },
    ];
    (*state).lencode = &raw const lenfix as *const code;
    (*state).lenbits = 9 as c_uint;
    (*state).distcode = &raw const distfix as *const code;
    (*state).distbits = 5 as c_uint;
}
unsafe extern "C" fn updatewindow(
    mut strm: z_streamp,
    mut end: *const Bytef,
    mut copy: c_uint,
) -> c_int {
    let mut state: *mut inflate_state = ::core::ptr::null_mut::<inflate_state>();
    let mut dist: c_uint = 0;
    state = (*strm).state as *mut inflate_state;
    if (*state).window.is_null() {
        (*state).window = Some((*strm).zalloc.expect("non-null function pointer"))
            .expect("non-null function pointer")(
            (*strm).opaque,
            (1 as uInt) << (*state).wbits,
            ::core::mem::size_of::<c_uchar>() as uInt,
        ) as *mut c_uchar;
        if (*state).window.is_null() {
            return 1 as c_int;
        }
    }
    if (*state).wsize == 0 as c_uint {
        (*state).wsize = (1 as c_uint) << (*state).wbits;
        (*state).wnext = 0 as c_uint;
        (*state).whave = 0 as c_uint;
    }
    if copy >= (*state).wsize {
        memcpy(
            (*state).window as *mut c_void,
            end.offset(-((*state).wsize as isize)) as *const c_void,
            (*state).wsize as size_t,
        );
        (*state).wnext = 0 as c_uint;
        (*state).whave = (*state).wsize;
    } else {
        dist = (*state).wsize.wrapping_sub((*state).wnext);
        if dist > copy {
            dist = copy;
        }
        memcpy(
            (*state).window.offset((*state).wnext as isize) as *mut c_void,
            end.offset(-(copy as isize)) as *const c_void,
            dist as size_t,
        );
        copy = copy.wrapping_sub(dist);
        if copy != 0 {
            memcpy(
                (*state).window as *mut c_void,
                end.offset(-(copy as isize)) as *const c_void,
                copy as size_t,
            );
            (*state).wnext = copy;
            (*state).whave = (*state).wsize;
        } else {
            (*state).wnext = (*state).wnext.wrapping_add(dist);
            if (*state).wnext == (*state).wsize {
                (*state).wnext = 0 as c_uint;
            }
            if (*state).whave < (*state).wsize {
                (*state).whave = (*state).whave.wrapping_add(dist);
            }
        }
    }
    return 0 as c_int;
}
#[no_mangle]
pub unsafe extern "C" fn inflate(
    mut strm: z_streamp,
    mut flush: c_int,
) -> c_int {
    let mut current_block: u64;
    let mut state: *mut inflate_state = ::core::ptr::null_mut::<inflate_state>();
    let mut next: *mut c_uchar = ::core::ptr::null_mut::<c_uchar>();
    let mut put: *mut c_uchar = ::core::ptr::null_mut::<c_uchar>();
    let mut have: c_uint = 0;
    let mut left: c_uint = 0;
    let mut hold: c_ulong = 0;
    let mut bits: c_uint = 0;
    let mut in_0: c_uint = 0;
    let mut out: c_uint = 0;
    let mut copy: c_uint = 0;
    let mut from: *mut c_uchar = ::core::ptr::null_mut::<c_uchar>();
    let mut here: code = code {
        op: 0,
        bits: 0,
        val: 0,
    };
    let mut last: code = code {
        op: 0,
        bits: 0,
        val: 0,
    };
    let mut len: c_uint = 0;
    let mut ret: c_int = 0;
    static mut order: [c_ushort; 19] = [
        16 as c_int as c_ushort,
        17 as c_int as c_ushort,
        18 as c_int as c_ushort,
        0 as c_int as c_ushort,
        8 as c_int as c_ushort,
        7 as c_int as c_ushort,
        9 as c_int as c_ushort,
        6 as c_int as c_ushort,
        10 as c_int as c_ushort,
        5 as c_int as c_ushort,
        11 as c_int as c_ushort,
        4 as c_int as c_ushort,
        12 as c_int as c_ushort,
        3 as c_int as c_ushort,
        13 as c_int as c_ushort,
        2 as c_int as c_ushort,
        14 as c_int as c_ushort,
        1 as c_int as c_ushort,
        15 as c_int as c_ushort,
    ];
    if inflateStateCheck(strm) != 0
        || (*strm).next_out.is_null()
        || (*strm).next_in.is_null() && (*strm).avail_in != 0 as c_uint
    {
        return Z_STREAM_ERROR;
    }
    state = (*strm).state as *mut inflate_state;
    if (*state).mode as c_uint == TYPE as c_int as c_uint {
        (*state).mode = TYPEDO;
    }
    put = (*strm).next_out as *mut c_uchar;
    left = (*strm).avail_out as c_uint;
    next = (*strm).next_in as *mut c_uchar;
    have = (*strm).avail_in as c_uint;
    hold = (*state).hold;
    bits = (*state).bits;
    in_0 = have;
    out = left;
    ret = Z_OK;
    's_86: loop {
        match (*state).mode as c_uint {
            16180 => {
                if (*state).wrap == 0 as c_int {
                    (*state).mode = TYPEDO;
                    continue;
                } else {
                    while bits < 16 as c_int as c_uint {
                        if have == 0 as c_uint {
                            break 's_86;
                        }
                        have = have.wrapping_sub(1);
                        let fresh0 = next;
                        next = next.offset(1);
                        hold = hold.wrapping_add((*fresh0 as c_ulong) << bits);
                        bits = bits.wrapping_add(8 as c_uint);
                    }
                    if (((hold as c_uint
                        & ((1 as c_uint) << 8 as c_int)
                            .wrapping_sub(1 as c_uint))
                        << 8 as c_int) as c_ulong)
                        .wrapping_add(hold >> 8 as c_int)
                        .wrapping_rem(31 as c_ulong)
                        != 0
                    {
                        (*strm).msg = b"incorrect header check\0" as *const u8
                            as *const c_char
                            as *mut c_char;
                        (*state).mode = BAD;
                        continue;
                    } else if hold as c_uint
                        & ((1 as c_uint) << 4 as c_int)
                            .wrapping_sub(1 as c_uint)
                        != Z_DEFLATED as c_uint
                    {
                        (*strm).msg = b"unknown compression method\0" as *const u8
                            as *const c_char
                            as *mut c_char;
                        (*state).mode = BAD;
                        continue;
                    } else {
                        hold >>= 4 as c_int;
                        bits = bits.wrapping_sub(4 as c_int as c_uint);
                        len = (hold as c_uint
                            & ((1 as c_uint) << 4 as c_int)
                                .wrapping_sub(1 as c_uint))
                        .wrapping_add(8 as c_uint);
                        if (*state).wbits == 0 as c_uint {
                            (*state).wbits = len;
                        }
                        if len > 15 as c_uint || len > (*state).wbits {
                            (*strm).msg = b"invalid window size\0" as *const u8
                                as *const c_char
                                as *mut c_char;
                            (*state).mode = BAD;
                            continue;
                        } else {
                            (*state).dmax = (1 as c_uint) << len;
                            (*state).check =
                                adler32(0 as uLong, ::core::ptr::null::<Bytef>(), 0 as uInt)
                                    as c_ulong;
                            (*strm).adler = (*state).check as uLong;
                            (*state).mode = (if hold & 0x200 as c_ulong != 0 {
                                DICTID as c_int
                            } else {
                                TYPE as c_int
                            }) as inflate_mode;
                            hold = 0 as c_ulong;
                            bits = 0 as c_uint;
                            continue;
                        }
                    }
                }
            }
            16189 => {
                while bits < 32 as c_int as c_uint {
                    if have == 0 as c_uint {
                        break 's_86;
                    }
                    have = have.wrapping_sub(1);
                    let fresh1 = next;
                    next = next.offset(1);
                    hold = hold.wrapping_add((*fresh1 as c_ulong) << bits);
                    bits = bits.wrapping_add(8 as c_uint);
                }
                (*state).check = (hold >> 24 as c_int & 0xff as c_ulong)
                    .wrapping_add(hold >> 8 as c_int & 0xff00 as c_ulong)
                    .wrapping_add(
                        (hold & 0xff00 as c_ulong) << 8 as c_int,
                    )
                    .wrapping_add(
                        (hold & 0xff as c_ulong) << 24 as c_int,
                    );
                (*strm).adler = (*state).check as uLong;
                hold = 0 as c_ulong;
                bits = 0 as c_uint;
                (*state).mode = DICT;
                current_block = 17881713786869885623;
            }
            16190 => {
                current_block = 17881713786869885623;
            }
            16191 => {
                current_block = 18086211173654465475;
            }
            16192 => {
                current_block = 3982790844861625541;
            }
            16193 => {
                hold >>= bits & 7 as c_uint;
                bits = bits.wrapping_sub(bits & 7 as c_uint);
                while bits < 32 as c_int as c_uint {
                    if have == 0 as c_uint {
                        break 's_86;
                    }
                    have = have.wrapping_sub(1);
                    let fresh3 = next;
                    next = next.offset(1);
                    hold = hold.wrapping_add((*fresh3 as c_ulong) << bits);
                    bits = bits.wrapping_add(8 as c_uint);
                }
                if hold & 0xffff as c_ulong
                    != hold >> 16 as c_int ^ 0xffff as c_ulong
                {
                    (*strm).msg = b"invalid stored block lengths\0" as *const u8
                        as *const c_char
                        as *mut c_char;
                    (*state).mode = BAD;
                    continue;
                } else {
                    (*state).length = hold as c_uint & 0xffff as c_uint;
                    hold = 0 as c_ulong;
                    bits = 0 as c_uint;
                    (*state).mode = COPY_;
                    if flush == Z_TREES {
                        break;
                    }
                }
                current_block = 17892661346077357722;
            }
            16194 => {
                current_block = 17892661346077357722;
            }
            16195 => {
                current_block = 16875839306739832038;
            }
            16196 => {
                while bits < 14 as c_int as c_uint {
                    if have == 0 as c_uint {
                        break 's_86;
                    }
                    have = have.wrapping_sub(1);
                    let fresh4 = next;
                    next = next.offset(1);
                    hold = hold.wrapping_add((*fresh4 as c_ulong) << bits);
                    bits = bits.wrapping_add(8 as c_uint);
                }
                (*state).nlen = (hold as c_uint
                    & ((1 as c_uint) << 5 as c_int)
                        .wrapping_sub(1 as c_uint))
                .wrapping_add(257 as c_uint);
                hold >>= 5 as c_int;
                bits = bits.wrapping_sub(5 as c_int as c_uint);
                (*state).ndist = (hold as c_uint
                    & ((1 as c_uint) << 5 as c_int)
                        .wrapping_sub(1 as c_uint))
                .wrapping_add(1 as c_uint);
                hold >>= 5 as c_int;
                bits = bits.wrapping_sub(5 as c_int as c_uint);
                (*state).ncode = (hold as c_uint
                    & ((1 as c_uint) << 4 as c_int)
                        .wrapping_sub(1 as c_uint))
                .wrapping_add(4 as c_uint);
                hold >>= 4 as c_int;
                bits = bits.wrapping_sub(4 as c_int as c_uint);
                if (*state).nlen > 286 as c_uint
                    || (*state).ndist > 30 as c_uint
                {
                    (*strm).msg = b"too many length or distance symbols\0" as *const u8
                        as *const c_char
                        as *mut c_char;
                    (*state).mode = BAD;
                    continue;
                } else {
                    (*state).have = 0 as c_uint;
                    (*state).mode = LENLENS;
                }
                current_block = 11354253847736050364;
            }
            16197 => {
                current_block = 11354253847736050364;
            }
            16198 => {
                current_block = 6692429041597365501;
            }
            16199 => {
                current_block = 16918022927008773773;
            }
            16200 => {
                current_block = 16242681590644410458;
            }
            16201 => {
                current_block = 335356216474397507;
            }
            16202 => {
                current_block = 2304730624008039899;
            }
            16203 => {
                current_block = 713470537136544505;
            }
            16204 => {
                current_block = 15629463900821410440;
            }
            16205 => {
                if left == 0 as c_uint {
                    break;
                }
                let fresh23 = put;
                put = put.offset(1);
                *fresh23 = (*state).length as c_uchar;
                left = left.wrapping_sub(1);
                (*state).mode = LEN;
                continue;
            }
            16206 => {
                if (*state).wrap != 0 {
                    while bits < 32 as c_int as c_uint {
                        if have == 0 as c_uint {
                            break 's_86;
                        }
                        have = have.wrapping_sub(1);
                        let fresh24 = next;
                        next = next.offset(1);
                        hold = hold.wrapping_add((*fresh24 as c_ulong) << bits);
                        bits = bits.wrapping_add(8 as c_uint);
                    }
                    out = out.wrapping_sub(left);
                    (*strm).total_out = ((*strm).total_out as c_ulong)
                        .wrapping_add(out as c_ulong)
                        as uLong as uLong;
                    (*state).total = (*state).total.wrapping_add(out as c_ulong);
                    if (*state).wrap & 4 as c_int != 0 && out != 0 {
                        (*state).check = adler32(
                            (*state).check as uLong,
                            put.offset(-(out as isize)),
                            out as uInt,
                        ) as c_ulong;
                        (*strm).adler = (*state).check as uLong;
                    }
                    out = left;
                    if (*state).wrap & 4 as c_int != 0
                        && (hold >> 24 as c_int & 0xff as c_ulong)
                            .wrapping_add(
                                hold >> 8 as c_int & 0xff00 as c_ulong,
                            )
                            .wrapping_add(
                                (hold & 0xff00 as c_ulong) << 8 as c_int,
                            )
                            .wrapping_add(
                                (hold & 0xff as c_ulong) << 24 as c_int,
                            )
                            != (*state).check
                    {
                        (*strm).msg = b"incorrect data check\0" as *const u8
                            as *const c_char
                            as *mut c_char;
                        (*state).mode = BAD;
                        continue;
                    } else {
                        hold = 0 as c_ulong;
                        bits = 0 as c_uint;
                    }
                }
                (*state).mode = DONE;
                current_block = 8750224895922280872;
            }
            16208 => {
                current_block = 8750224895922280872;
            }
            16209 => {
                ret = Z_DATA_ERROR;
                break;
            }
            16210 => return Z_MEM_ERROR,
            16211 | _ => return Z_STREAM_ERROR,
        }
        match current_block {
            11354253847736050364 => {
                while (*state).have < (*state).ncode {
                    while bits < 3 as c_int as c_uint {
                        if have == 0 as c_uint {
                            break 's_86;
                        }
                        have = have.wrapping_sub(1);
                        let fresh5 = next;
                        next = next.offset(1);
                        hold = hold.wrapping_add((*fresh5 as c_ulong) << bits);
                        bits = bits.wrapping_add(8 as c_uint);
                    }
                    let fresh6 = (*state).have;
                    (*state).have = (*state).have.wrapping_add(1);
                    (*state).lens[order[fresh6 as usize] as usize] = (hold as c_uint
                        & ((1 as c_uint) << 3 as c_int)
                            .wrapping_sub(1 as c_uint))
                        as c_ushort;
                    hold >>= 3 as c_int;
                    bits = bits.wrapping_sub(3 as c_int as c_uint);
                }
                while (*state).have < 19 as c_uint {
                    let fresh7 = (*state).have;
                    (*state).have = (*state).have.wrapping_add(1);
                    (*state).lens[order[fresh7 as usize] as usize] = 0 as c_ushort;
                }
                (*state).next = &raw mut (*state).codes as *mut code;
                (*state).lencode = (*state).next as *const code;
                (*state).lenbits = 7 as c_uint;
                ret = inflate_table(
                    CODES,
                    &raw mut (*state).lens as *mut c_ushort,
                    19 as c_uint,
                    &raw mut (*state).next,
                    &raw mut (*state).lenbits,
                    &raw mut (*state).work as *mut c_ushort,
                );
                if ret != 0 {
                    (*strm).msg = b"invalid code lengths set\0" as *const u8
                        as *const c_char
                        as *mut c_char;
                    (*state).mode = BAD;
                    continue;
                } else {
                    (*state).have = 0 as c_uint;
                    (*state).mode = CODELENS;
                }
                current_block = 6692429041597365501;
            }
            17881713786869885623 => {
                if (*state).havedict == 0 as c_int {
                    (*strm).next_out = put as *mut Bytef;
                    (*strm).avail_out = left as uInt;
                    (*strm).next_in = next as *mut Bytef;
                    (*strm).avail_in = have as uInt;
                    (*state).hold = hold;
                    (*state).bits = bits;
                    return Z_NEED_DICT;
                }
                (*state).check = adler32(0 as uLong, ::core::ptr::null::<Bytef>(), 0 as uInt)
                    as c_ulong;
                (*strm).adler = (*state).check as uLong;
                (*state).mode = TYPE;
                current_block = 18086211173654465475;
            }
            17892661346077357722 => {
                (*state).mode = COPY;
                current_block = 16875839306739832038;
            }
            8750224895922280872 => {
                ret = Z_STREAM_END;
                break;
            }
            _ => {}
        }
        match current_block {
            6692429041597365501 => {
                while (*state).have < (*state).nlen.wrapping_add((*state).ndist) {
                    loop {
                        here = *(*state).lencode.offset(
                            (hold as c_uint
                                & ((1 as c_uint) << (*state).lenbits)
                                    .wrapping_sub(1 as c_uint))
                                as isize,
                        );
                        if here.bits as c_uint <= bits {
                            break;
                        }
                        if have == 0 as c_uint {
                            break 's_86;
                        }
                        have = have.wrapping_sub(1);
                        let fresh8 = next;
                        next = next.offset(1);
                        hold = hold.wrapping_add((*fresh8 as c_ulong) << bits);
                        bits = bits.wrapping_add(8 as c_uint);
                    }
                    if (here.val as c_int) < 16 as c_int {
                        hold >>= here.bits as c_int;
                        bits = bits.wrapping_sub(here.bits as c_uint);
                        let fresh9 = (*state).have;
                        (*state).have = (*state).have.wrapping_add(1);
                        (*state).lens[fresh9 as usize] = here.val;
                    } else {
                        if here.val as c_int == 16 as c_int {
                            while bits
                                < (here.bits as c_int + 2 as c_int)
                                    as c_uint
                            {
                                if have == 0 as c_uint {
                                    break 's_86;
                                }
                                have = have.wrapping_sub(1);
                                let fresh10 = next;
                                next = next.offset(1);
                                hold =
                                    hold.wrapping_add((*fresh10 as c_ulong) << bits);
                                bits = bits.wrapping_add(8 as c_uint);
                            }
                            hold >>= here.bits as c_int;
                            bits = bits.wrapping_sub(here.bits as c_uint);
                            if (*state).have == 0 as c_uint {
                                (*strm).msg = b"invalid bit length repeat\0" as *const u8
                                    as *const c_char
                                    as *mut c_char;
                                (*state).mode = BAD;
                                break;
                            } else {
                                len = (*state).lens
                                    [(*state).have.wrapping_sub(1 as c_uint) as usize]
                                    as c_uint;
                                copy = (3 as c_uint).wrapping_add(
                                    hold as c_uint
                                        & ((1 as c_uint) << 2 as c_int)
                                            .wrapping_sub(1 as c_uint),
                                );
                                hold >>= 2 as c_int;
                                bits = bits
                                    .wrapping_sub(2 as c_int as c_uint);
                            }
                        } else if here.val as c_int == 17 as c_int {
                            while bits
                                < (here.bits as c_int + 3 as c_int)
                                    as c_uint
                            {
                                if have == 0 as c_uint {
                                    break 's_86;
                                }
                                have = have.wrapping_sub(1);
                                let fresh11 = next;
                                next = next.offset(1);
                                hold =
                                    hold.wrapping_add((*fresh11 as c_ulong) << bits);
                                bits = bits.wrapping_add(8 as c_uint);
                            }
                            hold >>= here.bits as c_int;
                            bits = bits.wrapping_sub(here.bits as c_uint);
                            len = 0 as c_uint;
                            copy = (3 as c_uint).wrapping_add(
                                hold as c_uint
                                    & ((1 as c_uint) << 3 as c_int)
                                        .wrapping_sub(1 as c_uint),
                            );
                            hold >>= 3 as c_int;
                            bits =
                                bits.wrapping_sub(3 as c_int as c_uint);
                        } else {
                            while bits
                                < (here.bits as c_int + 7 as c_int)
                                    as c_uint
                            {
                                if have == 0 as c_uint {
                                    break 's_86;
                                }
                                have = have.wrapping_sub(1);
                                let fresh12 = next;
                                next = next.offset(1);
                                hold =
                                    hold.wrapping_add((*fresh12 as c_ulong) << bits);
                                bits = bits.wrapping_add(8 as c_uint);
                            }
                            hold >>= here.bits as c_int;
                            bits = bits.wrapping_sub(here.bits as c_uint);
                            len = 0 as c_uint;
                            copy = (11 as c_uint).wrapping_add(
                                hold as c_uint
                                    & ((1 as c_uint) << 7 as c_int)
                                        .wrapping_sub(1 as c_uint),
                            );
                            hold >>= 7 as c_int;
                            bits =
                                bits.wrapping_sub(7 as c_int as c_uint);
                        }
                        if (*state).have.wrapping_add(copy)
                            > (*state).nlen.wrapping_add((*state).ndist)
                        {
                            (*strm).msg = b"invalid bit length repeat\0" as *const u8
                                as *const c_char
                                as *mut c_char;
                            (*state).mode = BAD;
                            break;
                        } else {
                            loop {
                                let fresh13 = copy;
                                copy = copy.wrapping_sub(1);
                                if !(fresh13 != 0) {
                                    break;
                                }
                                let fresh14 = (*state).have;
                                (*state).have = (*state).have.wrapping_add(1);
                                (*state).lens[fresh14 as usize] = len as c_ushort;
                            }
                        }
                    }
                }
                if (*state).mode as c_uint
                    == BAD as c_int as c_uint
                {
                    continue;
                }
                if (*state).lens[256 as c_int as usize] as c_int
                    == 0 as c_int
                {
                    (*strm).msg = b"invalid code -- missing end-of-block\0" as *const u8
                        as *const c_char
                        as *mut c_char;
                    (*state).mode = BAD;
                    continue;
                } else {
                    (*state).next = &raw mut (*state).codes as *mut code;
                    (*state).lencode = (*state).next as *const code;
                    (*state).lenbits = 9 as c_uint;
                    ret = inflate_table(
                        LENS,
                        &raw mut (*state).lens as *mut c_ushort,
                        (*state).nlen,
                        &raw mut (*state).next,
                        &raw mut (*state).lenbits,
                        &raw mut (*state).work as *mut c_ushort,
                    );
                    if ret != 0 {
                        (*strm).msg = b"invalid literal/lengths set\0" as *const u8
                            as *const c_char
                            as *mut c_char;
                        (*state).mode = BAD;
                        continue;
                    } else {
                        (*state).distcode = (*state).next as *const code;
                        (*state).distbits = 6 as c_uint;
                        ret = inflate_table(
                            DISTS,
                            (&raw mut (*state).lens as *mut c_ushort)
                                .offset((*state).nlen as isize),
                            (*state).ndist,
                            &raw mut (*state).next,
                            &raw mut (*state).distbits,
                            &raw mut (*state).work as *mut c_ushort,
                        );
                        if ret != 0 {
                            (*strm).msg = b"invalid distances set\0" as *const u8
                                as *const c_char
                                as *mut c_char;
                            (*state).mode = BAD;
                            continue;
                        } else {
                            (*state).mode = LEN_;
                            if flush == Z_TREES {
                                break;
                            }
                        }
                    }
                }
                current_block = 16918022927008773773;
            }
            16875839306739832038 => {
                copy = (*state).length;
                if copy != 0 {
                    if copy > have {
                        copy = have;
                    }
                    if copy > left {
                        copy = left;
                    }
                    if copy == 0 as c_uint {
                        break;
                    }
                    memcpy(
                        put as *mut c_void,
                        next as *const c_void,
                        copy as size_t,
                    );
                    have = have.wrapping_sub(copy);
                    next = next.offset(copy as isize);
                    left = left.wrapping_sub(copy);
                    put = put.offset(copy as isize);
                    (*state).length = (*state).length.wrapping_sub(copy);
                    continue;
                } else {
                    (*state).mode = TYPE;
                    continue;
                }
            }
            18086211173654465475 => {
                if flush == Z_BLOCK || flush == Z_TREES {
                    break;
                }
                current_block = 3982790844861625541;
            }
            _ => {}
        }
        match current_block {
            3982790844861625541 => {
                if (*state).last != 0 {
                    hold >>= bits & 7 as c_uint;
                    bits = bits.wrapping_sub(bits & 7 as c_uint);
                    (*state).mode = CHECK;
                    continue;
                } else {
                    while bits < 3 as c_int as c_uint {
                        if have == 0 as c_uint {
                            break 's_86;
                        }
                        have = have.wrapping_sub(1);
                        let fresh2 = next;
                        next = next.offset(1);
                        hold = hold.wrapping_add((*fresh2 as c_ulong) << bits);
                        bits = bits.wrapping_add(8 as c_uint);
                    }
                    (*state).last = (hold as c_uint
                        & ((1 as c_uint) << 1 as c_int)
                            .wrapping_sub(1 as c_uint))
                        as c_int;
                    hold >>= 1 as c_int;
                    bits = bits.wrapping_sub(1 as c_int as c_uint);
                    match hold as c_uint
                        & ((1 as c_uint) << 2 as c_int)
                            .wrapping_sub(1 as c_uint)
                    {
                        0 => {
                            (*state).mode = STORED;
                        }
                        1 => {
                            fixedtables(state);
                            (*state).mode = LEN_;
                            if flush == Z_TREES {
                                hold >>= 2 as c_int;
                                bits = bits
                                    .wrapping_sub(2 as c_int as c_uint);
                                break;
                            }
                        }
                        2 => {
                            (*state).mode = TABLE;
                        }
                        3 => {
                            (*strm).msg = b"invalid block type\0" as *const u8
                                as *const c_char
                                as *mut c_char;
                            (*state).mode = BAD;
                        }
                        _ => {}
                    }
                    hold >>= 2 as c_int;
                    bits = bits.wrapping_sub(2 as c_int as c_uint);
                    continue;
                }
            }
            16918022927008773773 => {
                (*state).mode = LEN;
                current_block = 16242681590644410458;
            }
            _ => {}
        }
        match current_block {
            16242681590644410458 => {
                if have >= 6 as c_uint && left >= 258 as c_uint {
                    (*strm).next_out = put as *mut Bytef;
                    (*strm).avail_out = left as uInt;
                    (*strm).next_in = next as *mut Bytef;
                    (*strm).avail_in = have as uInt;
                    (*state).hold = hold;
                    (*state).bits = bits;
                    inflate_fast(strm, out);
                    put = (*strm).next_out as *mut c_uchar;
                    left = (*strm).avail_out as c_uint;
                    next = (*strm).next_in as *mut c_uchar;
                    have = (*strm).avail_in as c_uint;
                    hold = (*state).hold;
                    bits = (*state).bits;
                    if (*state).mode as c_uint
                        == TYPE as c_int as c_uint
                    {
                        (*state).back = -(1 as c_int);
                    }
                    continue;
                } else {
                    (*state).back = 0 as c_int;
                    loop {
                        here = *(*state).lencode.offset(
                            (hold as c_uint
                                & ((1 as c_uint) << (*state).lenbits)
                                    .wrapping_sub(1 as c_uint))
                                as isize,
                        );
                        if here.bits as c_uint <= bits {
                            break;
                        }
                        if have == 0 as c_uint {
                            break 's_86;
                        }
                        have = have.wrapping_sub(1);
                        let fresh15 = next;
                        next = next.offset(1);
                        hold = hold.wrapping_add((*fresh15 as c_ulong) << bits);
                        bits = bits.wrapping_add(8 as c_uint);
                    }
                    if here.op as c_int != 0
                        && here.op as c_int & 0xf0 as c_int
                            == 0 as c_int
                    {
                        last = here;
                        loop {
                            here = *(*state).lencode.offset(
                                (last.val as c_uint).wrapping_add(
                                    (hold as c_uint
                                        & ((1 as c_uint)
                                            << last.bits as c_int
                                                + last.op as c_int)
                                            .wrapping_sub(1 as c_uint))
                                        >> last.bits as c_int,
                                ) as isize,
                            );
                            if (last.bits as c_int + here.bits as c_int)
                                as c_uint
                                <= bits
                            {
                                break;
                            }
                            if have == 0 as c_uint {
                                break 's_86;
                            }
                            have = have.wrapping_sub(1);
                            let fresh16 = next;
                            next = next.offset(1);
                            hold = hold.wrapping_add((*fresh16 as c_ulong) << bits);
                            bits = bits.wrapping_add(8 as c_uint);
                        }
                        hold >>= last.bits as c_int;
                        bits = bits.wrapping_sub(last.bits as c_uint);
                        (*state).back += last.bits as c_int;
                    }
                    hold >>= here.bits as c_int;
                    bits = bits.wrapping_sub(here.bits as c_uint);
                    (*state).back += here.bits as c_int;
                    (*state).length = here.val as c_uint;
                    if here.op as c_int == 0 as c_int {
                        (*state).mode = LIT;
                        continue;
                    } else if here.op as c_int & 32 as c_int != 0 {
                        (*state).back = -(1 as c_int);
                        (*state).mode = TYPE;
                        continue;
                    } else if here.op as c_int & 64 as c_int != 0 {
                        (*strm).msg = b"invalid literal/length code\0" as *const u8
                            as *const c_char
                            as *mut c_char;
                        (*state).mode = BAD;
                        continue;
                    } else {
                        (*state).extra = here.op as c_uint & 15 as c_uint;
                        (*state).mode = LENEXT;
                    }
                }
                current_block = 335356216474397507;
            }
            _ => {}
        }
        match current_block {
            335356216474397507 => {
                if (*state).extra != 0 {
                    while bits < (*state).extra {
                        if have == 0 as c_uint {
                            break 's_86;
                        }
                        have = have.wrapping_sub(1);
                        let fresh17 = next;
                        next = next.offset(1);
                        hold = hold.wrapping_add((*fresh17 as c_ulong) << bits);
                        bits = bits.wrapping_add(8 as c_uint);
                    }
                    (*state).length = (*state).length.wrapping_add(
                        hold as c_uint
                            & ((1 as c_uint) << (*state).extra)
                                .wrapping_sub(1 as c_uint),
                    );
                    hold >>= (*state).extra;
                    bits = bits.wrapping_sub((*state).extra);
                    (*state).back =
                        ((*state).back as c_uint).wrapping_add((*state).extra)
                            as c_int as c_int;
                }
                (*state).was = (*state).length;
                (*state).mode = DIST;
                current_block = 2304730624008039899;
            }
            _ => {}
        }
        loop {
            match current_block {
                713470537136544505 => {
                    if (*state).extra != 0 {
                        while bits < (*state).extra {
                            if have == 0 as c_uint {
                                break 's_86;
                            }
                            have = have.wrapping_sub(1);
                            let fresh20 = next;
                            next = next.offset(1);
                            hold = hold.wrapping_add((*fresh20 as c_ulong) << bits);
                            bits = bits.wrapping_add(8 as c_uint);
                        }
                        (*state).offset = (*state).offset.wrapping_add(
                            hold as c_uint
                                & ((1 as c_uint) << (*state).extra)
                                    .wrapping_sub(1 as c_uint),
                        );
                        hold >>= (*state).extra;
                        bits = bits.wrapping_sub((*state).extra);
                        (*state).back = ((*state).back as c_uint)
                            .wrapping_add((*state).extra)
                            as c_int
                            as c_int;
                    }
                    (*state).mode = MATCH;
                    current_block = 15629463900821410440;
                }
                2304730624008039899 => {
                    here = *(*state).distcode.offset(
                        (hold as c_uint
                            & ((1 as c_uint) << (*state).distbits)
                                .wrapping_sub(1 as c_uint))
                            as isize,
                    );
                    if here.bits as c_uint <= bits {
                        if here.op as c_int & 0xf0 as c_int
                            == 0 as c_int
                        {
                            last = here;
                            loop {
                                here = *(*state).distcode.offset(
                                    (last.val as c_uint).wrapping_add(
                                        (hold as c_uint
                                            & ((1 as c_uint)
                                                << last.bits as c_int
                                                    + last.op as c_int)
                                                .wrapping_sub(1 as c_uint))
                                            >> last.bits as c_int,
                                    ) as isize,
                                );
                                if (last.bits as c_int
                                    + here.bits as c_int)
                                    as c_uint
                                    <= bits
                                {
                                    break;
                                }
                                if have == 0 as c_uint {
                                    break 's_86;
                                }
                                have = have.wrapping_sub(1);
                                let fresh19 = next;
                                next = next.offset(1);
                                hold =
                                    hold.wrapping_add((*fresh19 as c_ulong) << bits);
                                bits = bits.wrapping_add(8 as c_uint);
                            }
                            hold >>= last.bits as c_int;
                            bits = bits.wrapping_sub(last.bits as c_uint);
                            (*state).back += last.bits as c_int;
                        }
                        hold >>= here.bits as c_int;
                        bits = bits.wrapping_sub(here.bits as c_uint);
                        (*state).back += here.bits as c_int;
                        if here.op as c_int & 64 as c_int != 0 {
                            (*strm).msg = b"invalid distance code\0" as *const u8
                                as *const c_char
                                as *mut c_char;
                            (*state).mode = BAD;
                            continue 's_86;
                        } else {
                            (*state).offset = here.val as c_uint;
                            (*state).extra =
                                here.op as c_uint & 15 as c_uint;
                            (*state).mode = DISTEXT;
                            current_block = 713470537136544505;
                        }
                    } else {
                        if have == 0 as c_uint {
                            break 's_86;
                        }
                        have = have.wrapping_sub(1);
                        let fresh18 = next;
                        next = next.offset(1);
                        hold = hold.wrapping_add((*fresh18 as c_ulong) << bits);
                        bits = bits.wrapping_add(8 as c_uint);
                        current_block = 2304730624008039899;
                    }
                }
                _ => {
                    if left == 0 as c_uint {
                        break 's_86;
                    }
                    copy = out.wrapping_sub(left);
                    if (*state).offset > copy {
                        current_block = 15451600762152631869;
                        break;
                    } else {
                        current_block = 1396549568163370148;
                        break;
                    }
                }
            }
        }
        match current_block {
            1396549568163370148 => {
                from = put.offset(-((*state).offset as isize));
                copy = (*state).length;
            }
            _ => {
                copy = (*state).offset.wrapping_sub(copy);
                if copy > (*state).whave {
                    if (*state).sane != 0 {
                        (*strm).msg = b"invalid distance too far back\0" as *const u8
                            as *const c_char
                            as *mut c_char;
                        (*state).mode = BAD;
                        continue;
                    }
                }
                if copy > (*state).wnext {
                    copy = copy.wrapping_sub((*state).wnext);
                    from = (*state)
                        .window
                        .offset((*state).wsize.wrapping_sub(copy) as isize);
                } else {
                    from = (*state)
                        .window
                        .offset((*state).wnext.wrapping_sub(copy) as isize);
                }
                if copy > (*state).length {
                    copy = (*state).length;
                }
            }
        }
        if copy > left {
            copy = left;
        }
        left = left.wrapping_sub(copy);
        (*state).length = (*state).length.wrapping_sub(copy);
        loop {
            let fresh21 = from;
            from = from.offset(1);
            let fresh22 = put;
            put = put.offset(1);
            *fresh22 = *fresh21;
            copy = copy.wrapping_sub(1);
            if !(copy != 0) {
                break;
            }
        }
        if (*state).length == 0 as c_uint {
            (*state).mode = LEN;
        }
    }
    (*strm).next_out = put as *mut Bytef;
    (*strm).avail_out = left as uInt;
    (*strm).next_in = next as *mut Bytef;
    (*strm).avail_in = have as uInt;
    (*state).hold = hold;
    (*state).bits = bits;
    if (*state).wsize != 0
        || out != (*strm).avail_out
            && ((*state).mode as c_uint)
                < BAD as c_int as c_uint
            && (((*state).mode as c_uint)
                < CHECK as c_int as c_uint
                || flush != Z_FINISH)
    {
        if updatewindow(
            strm,
            (*strm).next_out,
            out.wrapping_sub((*strm).avail_out as c_uint),
        ) != 0
        {
            (*state).mode = MEM;
            return Z_MEM_ERROR;
        }
    }
    in_0 = in_0.wrapping_sub((*strm).avail_in as c_uint);
    out = out.wrapping_sub((*strm).avail_out as c_uint);
    (*strm).total_in = ((*strm).total_in as c_ulong)
        .wrapping_add(in_0 as c_ulong) as uLong as uLong;
    (*strm).total_out = ((*strm).total_out as c_ulong)
        .wrapping_add(out as c_ulong) as uLong as uLong;
    (*state).total = (*state).total.wrapping_add(out as c_ulong);
    if (*state).wrap & 4 as c_int != 0 && out != 0 {
        (*state).check = adler32(
            (*state).check as uLong,
            (*strm).next_out.offset(-(out as isize)),
            out as uInt,
        ) as c_ulong;
        (*strm).adler = (*state).check as uLong;
    }
    (*strm).data_type = (*state).bits as c_int
        + (if (*state).last != 0 {
            64 as c_int
        } else {
            0 as c_int
        })
        + (if (*state).mode as c_uint
            == TYPE as c_int as c_uint
        {
            128 as c_int
        } else {
            0 as c_int
        })
        + (if (*state).mode as c_uint
            == LEN_ as c_int as c_uint
            || (*state).mode as c_uint
                == COPY_ as c_int as c_uint
        {
            256 as c_int
        } else {
            0 as c_int
        });
    if (in_0 == 0 as c_uint && out == 0 as c_uint || flush == Z_FINISH)
        && ret == Z_OK
    {
        ret = Z_BUF_ERROR;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn inflateEnd(mut strm: z_streamp) -> c_int {
    let mut state: *mut inflate_state = ::core::ptr::null_mut::<inflate_state>();
    if inflateStateCheck(strm) != 0 {
        return Z_STREAM_ERROR;
    }
    state = (*strm).state as *mut inflate_state;
    if !(*state).window.is_null() {
        Some((*strm).zfree.expect("non-null function pointer")).expect("non-null function pointer")(
            (*strm).opaque,
            (*state).window as voidpf,
        );
    }
    Some((*strm).zfree.expect("non-null function pointer")).expect("non-null function pointer")(
        (*strm).opaque,
        (*strm).state as voidpf,
    );
    (*strm).state = ::core::ptr::null_mut::<internal_state>();
    return Z_OK;
}
#[no_mangle]
pub unsafe extern "C" fn inflateGetDictionary(
    mut strm: z_streamp,
    mut dictionary: *mut Bytef,
    mut dictLength: *mut uInt,
) -> c_int {
    let mut state: *mut inflate_state = ::core::ptr::null_mut::<inflate_state>();
    if inflateStateCheck(strm) != 0 {
        return Z_STREAM_ERROR;
    }
    state = (*strm).state as *mut inflate_state;
    if (*state).whave != 0 && !dictionary.is_null() {
        memcpy(
            dictionary as *mut c_void,
            (*state).window.offset((*state).wnext as isize) as *const c_void,
            (*state).whave.wrapping_sub((*state).wnext) as size_t,
        );
        memcpy(
            dictionary
                .offset((*state).whave as isize)
                .offset(-((*state).wnext as isize)) as *mut c_void,
            (*state).window as *const c_void,
            (*state).wnext as size_t,
        );
    }
    if !dictLength.is_null() {
        *dictLength = (*state).whave as uInt;
    }
    return Z_OK;
}
#[no_mangle]
pub unsafe extern "C" fn inflateSetDictionary(
    mut strm: z_streamp,
    mut dictionary: *const Bytef,
    mut dictLength: uInt,
) -> c_int {
    let mut state: *mut inflate_state = ::core::ptr::null_mut::<inflate_state>();
    let mut dictid: c_ulong = 0;
    let mut ret: c_int = 0;
    if inflateStateCheck(strm) != 0 {
        return Z_STREAM_ERROR;
    }
    state = (*strm).state as *mut inflate_state;
    if (*state).wrap != 0 as c_int
        && (*state).mode as c_uint != DICT as c_int as c_uint
    {
        return Z_STREAM_ERROR;
    }
    if (*state).mode as c_uint == DICT as c_int as c_uint {
        dictid =
            adler32(0 as uLong, ::core::ptr::null::<Bytef>(), 0 as uInt) as c_ulong;
        dictid = adler32(dictid as uLong, dictionary, dictLength) as c_ulong;
        if dictid != (*state).check {
            return Z_DATA_ERROR;
        }
    }
    ret = updatewindow(
        strm,
        dictionary.offset(dictLength as isize),
        dictLength as c_uint,
    );
    if ret != 0 {
        (*state).mode = MEM;
        return Z_MEM_ERROR;
    }
    (*state).havedict = 1 as c_int;
    return Z_OK;
}
#[no_mangle]
pub unsafe extern "C" fn inflateGetHeader(
    mut strm: z_streamp,
    mut head: gz_headerp,
) -> c_int {
    let mut state: *mut inflate_state = ::core::ptr::null_mut::<inflate_state>();
    if inflateStateCheck(strm) != 0 {
        return Z_STREAM_ERROR;
    }
    state = (*strm).state as *mut inflate_state;
    if (*state).wrap & 2 as c_int == 0 as c_int {
        return Z_STREAM_ERROR;
    }
    (*state).head = head;
    (*head).done = 0 as c_int;
    return Z_OK;
}
unsafe extern "C" fn syncsearch(
    mut have: *mut c_uint,
    mut buf: *const c_uchar,
    mut len: c_uint,
) -> c_uint {
    let mut got: c_uint = 0;
    let mut next: c_uint = 0;
    got = *have;
    next = 0 as c_uint;
    while next < len && got < 4 as c_uint {
        if *buf.offset(next as isize) as c_int
            == (if got < 2 as c_uint {
                0 as c_int
            } else {
                0xff as c_int
            })
        {
            got = got.wrapping_add(1);
        } else if *buf.offset(next as isize) != 0 {
            got = 0 as c_uint;
        } else {
            got = (4 as c_uint).wrapping_sub(got);
        }
        next = next.wrapping_add(1);
    }
    *have = got;
    return next;
}
#[no_mangle]
pub unsafe extern "C" fn inflateSync(mut strm: z_streamp) -> c_int {
    let mut len: c_uint = 0;
    let mut in_0: c_ulong = 0;
    let mut out: c_ulong = 0;
    let mut buf: [c_uchar; 4] = [0; 4];
    let mut state: *mut inflate_state = ::core::ptr::null_mut::<inflate_state>();
    if inflateStateCheck(strm) != 0 {
        return Z_STREAM_ERROR;
    }
    state = (*strm).state as *mut inflate_state;
    if (*strm).avail_in == 0 as c_uint && (*state).bits < 8 as c_uint {
        return Z_BUF_ERROR;
    }
    if (*state).mode as c_uint != SYNC as c_int as c_uint {
        (*state).mode = SYNC;
        (*state).hold <<= (*state).bits & 7 as c_uint;
        (*state).bits = (*state)
            .bits
            .wrapping_sub((*state).bits & 7 as c_uint);
        len = 0 as c_uint;
        while (*state).bits >= 8 as c_uint {
            let fresh25 = len;
            len = len.wrapping_add(1);
            buf[fresh25 as usize] = (*state).hold as c_uchar;
            (*state).hold >>= 8 as c_int;
            (*state).bits = (*state).bits.wrapping_sub(8 as c_uint);
        }
        (*state).have = 0 as c_uint;
        syncsearch(
            &raw mut (*state).have,
            &raw mut buf as *mut c_uchar,
            len,
        );
    }
    len = syncsearch(
        &raw mut (*state).have,
        (*strm).next_in,
        (*strm).avail_in as c_uint,
    );
    (*strm).avail_in = ((*strm).avail_in as c_uint).wrapping_sub(len) as uInt as uInt;
    (*strm).next_in = (*strm).next_in.offset(len as isize);
    (*strm).total_in = ((*strm).total_in as c_ulong)
        .wrapping_add(len as c_ulong) as uLong as uLong;
    if (*state).have != 4 as c_uint {
        return Z_DATA_ERROR;
    }
    in_0 = (*strm).total_in as c_ulong;
    out = (*strm).total_out as c_ulong;
    inflateReset(strm);
    (*strm).total_in = in_0 as uLong;
    (*strm).total_out = out as uLong;
    (*state).mode = TYPE;
    return Z_OK;
}
#[no_mangle]
pub unsafe extern "C" fn inflateSyncPoint(mut strm: z_streamp) -> c_int {
    let mut state: *mut inflate_state = ::core::ptr::null_mut::<inflate_state>();
    if inflateStateCheck(strm) != 0 {
        return Z_STREAM_ERROR;
    }
    state = (*strm).state as *mut inflate_state;
    return ((*state).mode as c_uint
        == STORED as c_int as c_uint
        && (*state).bits == 0 as c_uint) as c_int;
}
#[no_mangle]
pub unsafe extern "C" fn inflateCopy(
    mut dest: z_streamp,
    mut source: z_streamp,
) -> c_int {
    let mut state: *mut inflate_state = ::core::ptr::null_mut::<inflate_state>();
    let mut copy: *mut inflate_state = ::core::ptr::null_mut::<inflate_state>();
    let mut window: *mut c_uchar = ::core::ptr::null_mut::<c_uchar>();
    let mut wsize: c_uint = 0;
    if inflateStateCheck(source) != 0 || dest.is_null() {
        return Z_STREAM_ERROR;
    }
    state = (*source).state as *mut inflate_state;
    copy = Some((*source).zalloc.expect("non-null function pointer"))
        .expect("non-null function pointer")(
        (*source).opaque,
        1 as uInt,
        ::core::mem::size_of::<inflate_state>() as uInt,
    ) as *mut inflate_state;
    if copy.is_null() {
        return Z_MEM_ERROR;
    }
    window = ::core::ptr::null_mut::<c_uchar>();
    if !(*state).window.is_null() {
        window = Some((*source).zalloc.expect("non-null function pointer"))
            .expect("non-null function pointer")(
            (*source).opaque,
            (1 as uInt) << (*state).wbits,
            ::core::mem::size_of::<c_uchar>() as uInt,
        ) as *mut c_uchar;
        if window.is_null() {
            Some((*source).zfree.expect("non-null function pointer"))
                .expect("non-null function pointer")((*source).opaque, copy as voidpf);
            return Z_MEM_ERROR;
        }
    }
    memcpy(
        dest as *mut c_void,
        source as voidpf as *const c_void,
        ::core::mem::size_of::<z_stream>() as size_t,
    );
    memcpy(
        copy as *mut c_void,
        state as voidpf as *const c_void,
        ::core::mem::size_of::<inflate_state>() as size_t,
    );
    (*copy).strm = dest;
    if (*state).lencode >= &raw mut (*state).codes as *mut code as *const code
        && (*state).lencode
            <= (&raw mut (*state).codes as *mut code)
                .offset(ENOUGH as isize)
                .offset(-(1 as c_int as isize)) as *const code
    {
        (*copy).lencode = (&raw mut (*copy).codes as *mut code).offset(
            (*state)
                .lencode
                .offset_from(&raw mut (*state).codes as *mut code)
                as c_long as isize,
        );
        (*copy).distcode = (&raw mut (*copy).codes as *mut code).offset(
            (*state)
                .distcode
                .offset_from(&raw mut (*state).codes as *mut code)
                as c_long as isize,
        );
    }
    (*copy).next = (&raw mut (*copy).codes as *mut code).offset(
        (*state)
            .next
            .offset_from(&raw mut (*state).codes as *mut code) as c_long
            as isize,
    );
    if !window.is_null() {
        wsize = (1 as c_uint) << (*state).wbits;
        memcpy(
            window as *mut c_void,
            (*state).window as *const c_void,
            wsize as size_t,
        );
    }
    (*copy).window = window;
    (*dest).state = copy as *mut internal_state;
    return Z_OK;
}
#[no_mangle]
pub unsafe extern "C" fn inflateUndermine(
    mut strm: z_streamp,
    mut subvert: c_int,
) -> c_int {
    let mut state: *mut inflate_state = ::core::ptr::null_mut::<inflate_state>();
    if inflateStateCheck(strm) != 0 {
        return Z_STREAM_ERROR;
    }
    state = (*strm).state as *mut inflate_state;
    (*state).sane = 1 as c_int;
    return Z_DATA_ERROR;
}
#[no_mangle]
pub unsafe extern "C" fn inflateValidate(
    mut strm: z_streamp,
    mut check: c_int,
) -> c_int {
    let mut state: *mut inflate_state = ::core::ptr::null_mut::<inflate_state>();
    if inflateStateCheck(strm) != 0 {
        return Z_STREAM_ERROR;
    }
    state = (*strm).state as *mut inflate_state;
    if check != 0 {
        (*state).wrap |= 4 as c_int;
    } else {
        (*state).wrap &= !(4 as c_int);
    }
    return Z_OK;
}
#[no_mangle]
pub unsafe extern "C" fn inflateMark(mut strm: z_streamp) -> c_long {
    let mut state: *mut inflate_state = ::core::ptr::null_mut::<inflate_state>();
    if inflateStateCheck(strm) != 0 {
        return -((1 as c_long) << 16 as c_int);
    }
    state = (*strm).state as *mut inflate_state;
    return (((*state).back as c_long as c_ulong)
        << 16 as c_int) as c_long
        + (if (*state).mode as c_uint
            == COPY as c_int as c_uint
        {
            (*state).length
        } else {
            (if (*state).mode as c_uint
                == MATCH as c_int as c_uint
            {
                (*state).was.wrapping_sub((*state).length)
            } else {
                0 as c_uint
            })
        }) as c_long;
}
#[no_mangle]
pub unsafe extern "C" fn inflateCodesUsed(mut strm: z_streamp) -> c_ulong {
    let mut state: *mut inflate_state = ::core::ptr::null_mut::<inflate_state>();
    if inflateStateCheck(strm) != 0 {
        return -(1 as c_int) as c_ulong;
    }
    state = (*strm).state as *mut inflate_state;
    return (*state)
        .next
        .offset_from(&raw mut (*state).codes as *mut code) as c_long
        as c_ulong;
}
