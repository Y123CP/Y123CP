use core::ffi::*;
use crate::src::zlib::inftrees::inflate_table;
pub use crate::src::ffi::*;
pub use crate::src::c_consts::*;
pub use crate::src::c_structs::*;
pub use crate::src::c_types::*;
pub use crate::src::zlib::deflate::internal_state;
extern "C" {
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

pub type in_func = Option<
    unsafe extern "C" fn(
        *mut c_void,
        *mut *mut c_uchar,
    ) -> c_uint,
>;
pub type out_func = Option<
    unsafe extern "C" fn(
        *mut c_void,
        *mut c_uchar,
        c_uint,
    ) -> c_int,
>;

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

pub const COPY: inflate_mode = 16195;

pub const ZLIB_VERSION: [c_char; 15] =
    unsafe { ::core::mem::transmute::<[u8; 15], [c_char; 15]>(*b"1.2.11-optipng\0") };
pub const Z_OK: c_int = 0 as c_int;
pub const Z_STREAM_END: c_int = 1 as c_int;

pub const Z_DATA_ERROR: c_int = -(3 as c_int);
pub const Z_MEM_ERROR: c_int = -(4 as c_int);
pub const Z_BUF_ERROR: c_int = -(5 as c_int);
pub const Z_VERSION_ERROR: c_int = -(6 as c_int);

#[inline]
pub unsafe fn inflateBackInit_(
    mut strm: z_streamp,
    mut windowBits: c_int,
    mut window: *mut c_uchar,
    mut version: *const c_char,
    mut stream_size: c_int,
) -> c_int {
    let mut state: *mut inflate_state = ::core::ptr::null_mut::<inflate_state>();
    if version.is_null()
        || *version.offset(0 as c_int as isize) as c_int
            != ZLIB_VERSION[0 as c_int as usize] as c_int
        || stream_size != ::core::mem::size_of::<z_stream>() as c_int
    {
        return Z_VERSION_ERROR;
    }
    if strm.is_null()
        || window.is_null()
        || windowBits < 8 as c_int
        || windowBits > 15 as c_int
    {
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
    (*state).dmax = 32768 as c_uint;
    (*state).wbits = windowBits as uInt as c_uint;
    (*state).wsize = (1 as c_uint) << windowBits;
    (*state).window = window;
    (*state).wnext = 0 as c_uint;
    (*state).whave = 0 as c_uint;
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
pub unsafe fn inflateBack(
    mut strm: z_streamp,
    mut in_0: in_func,
    mut in_desc: *mut c_void,
    mut out: out_func,
    mut out_desc: *mut c_void,
) -> c_int {
    let mut state: *mut inflate_state = ::core::ptr::null_mut::<inflate_state>();
    let mut next: *mut c_uchar = ::core::ptr::null_mut::<c_uchar>();
    let mut put: *mut c_uchar = ::core::ptr::null_mut::<c_uchar>();
    let mut have: c_uint = 0;
    let mut left: c_uint = 0;
    let mut hold: c_ulong = 0;
    let mut bits: c_uint = 0;
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
    if strm.is_null() || (*strm).state.is_null() {
        return Z_STREAM_ERROR;
    }
    state = (*strm).state as *mut inflate_state;
    (*strm).msg = ::core::ptr::null_mut::<c_char>();
    (*state).mode = TYPE;
    (*state).last = 0 as c_int;
    (*state).whave = 0 as c_uint;
    next = (*strm).next_in as *mut c_uchar;
    have = if !next.is_null() {
        (*strm).avail_in as c_uint
    } else {
        0 as c_uint
    };
    hold = 0 as c_ulong;
    bits = 0 as c_uint;
    put = (*state).window;
    left = (*state).wsize;
    's_69: loop {
        match (*state).mode as c_uint {
            16191 => {
                if (*state).last != 0 {
                    hold >>= bits & 7 as c_uint;
                    bits = bits.wrapping_sub(bits & 7 as c_uint);
                    (*state).mode = DONE;
                    continue;
                } else {
                    while bits < 3 as c_int as c_uint {
                        if have == 0 as c_uint {
                            have = in_0.expect("non-null function pointer")(in_desc, &raw mut next);
                            if have == 0 as c_uint {
                                next = ::core::ptr::null_mut::<c_uchar>();
                                ret = Z_BUF_ERROR;
                                break 's_69;
                            }
                        }
                        have = have.wrapping_sub(1);
                        let fresh0 = next;
                        next = next.offset(1);
                        hold = hold.wrapping_add((*fresh0 as c_ulong) << bits);
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
                            (*state).mode = LEN;
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
            16193 => {
                hold >>= bits & 7 as c_uint;
                bits = bits.wrapping_sub(bits & 7 as c_uint);
                while bits < 32 as c_int as c_uint {
                    if have == 0 as c_uint {
                        have = in_0.expect("non-null function pointer")(in_desc, &raw mut next);
                        if have == 0 as c_uint {
                            next = ::core::ptr::null_mut::<c_uchar>();
                            ret = Z_BUF_ERROR;
                            break 's_69;
                        }
                    }
                    have = have.wrapping_sub(1);
                    let fresh1 = next;
                    next = next.offset(1);
                    hold = hold.wrapping_add((*fresh1 as c_ulong) << bits);
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
                    while (*state).length != 0 as c_uint {
                        copy = (*state).length;
                        if have == 0 as c_uint {
                            have = in_0.expect("non-null function pointer")(in_desc, &raw mut next);
                            if have == 0 as c_uint {
                                next = ::core::ptr::null_mut::<c_uchar>();
                                ret = Z_BUF_ERROR;
                                break 's_69;
                            }
                        }
                        if left == 0 as c_uint {
                            put = (*state).window;
                            left = (*state).wsize;
                            (*state).whave = left;
                            if out.expect("non-null function pointer")(out_desc, put, left) != 0 {
                                ret = Z_BUF_ERROR;
                                break 's_69;
                            }
                        }
                        if copy > have {
                            copy = have;
                        }
                        if copy > left {
                            copy = left;
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
                    }
                    (*state).mode = TYPE;
                    continue;
                }
            }
            16196 => {
                while bits < 14 as c_int as c_uint {
                    if have == 0 as c_uint {
                        have = in_0.expect("non-null function pointer")(in_desc, &raw mut next);
                        if have == 0 as c_uint {
                            next = ::core::ptr::null_mut::<c_uchar>();
                            ret = Z_BUF_ERROR;
                            break 's_69;
                        }
                    }
                    have = have.wrapping_sub(1);
                    let fresh2 = next;
                    next = next.offset(1);
                    hold = hold.wrapping_add((*fresh2 as c_ulong) << bits);
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
                    while (*state).have < (*state).ncode {
                        while bits < 3 as c_int as c_uint {
                            if have == 0 as c_uint {
                                have = in_0.expect("non-null function pointer")(
                                    in_desc,
                                    &raw mut next,
                                );
                                if have == 0 as c_uint {
                                    next = ::core::ptr::null_mut::<c_uchar>();
                                    ret = Z_BUF_ERROR;
                                    break 's_69;
                                }
                            }
                            have = have.wrapping_sub(1);
                            let fresh3 = next;
                            next = next.offset(1);
                            hold = hold.wrapping_add((*fresh3 as c_ulong) << bits);
                            bits = bits.wrapping_add(8 as c_uint);
                        }
                        let fresh4 = (*state).have;
                        (*state).have = (*state).have.wrapping_add(1);
                        (*state).lens[order[fresh4 as usize] as usize] = (hold
                            as c_uint
                            & ((1 as c_uint) << 3 as c_int)
                                .wrapping_sub(1 as c_uint))
                            as c_ushort;
                        hold >>= 3 as c_int;
                        bits = bits.wrapping_sub(3 as c_int as c_uint);
                    }
                    while (*state).have < 19 as c_uint {
                        let fresh5 = (*state).have;
                        (*state).have = (*state).have.wrapping_add(1);
                        (*state).lens[order[fresh5 as usize] as usize] = 0 as c_ushort;
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
                                    have = in_0.expect("non-null function pointer")(
                                        in_desc,
                                        &raw mut next,
                                    );
                                    if have == 0 as c_uint {
                                        next = ::core::ptr::null_mut::<c_uchar>();
                                        ret = Z_BUF_ERROR;
                                        break 's_69;
                                    }
                                }
                                have = have.wrapping_sub(1);
                                let fresh6 = next;
                                next = next.offset(1);
                                hold = hold.wrapping_add((*fresh6 as c_ulong) << bits);
                                bits = bits.wrapping_add(8 as c_uint);
                            }
                            if (here.val as c_int) < 16 as c_int {
                                hold >>= here.bits as c_int;
                                bits = bits.wrapping_sub(here.bits as c_uint);
                                let fresh7 = (*state).have;
                                (*state).have = (*state).have.wrapping_add(1);
                                (*state).lens[fresh7 as usize] = here.val;
                            } else {
                                if here.val as c_int == 16 as c_int {
                                    while bits
                                        < (here.bits as c_int
                                            + 2 as c_int)
                                            as c_uint
                                    {
                                        if have == 0 as c_uint {
                                            have = in_0.expect("non-null function pointer")(
                                                in_desc,
                                                &raw mut next,
                                            );
                                            if have == 0 as c_uint {
                                                next =
                                                    ::core::ptr::null_mut::<c_uchar>();
                                                ret = Z_BUF_ERROR;
                                                break 's_69;
                                            }
                                        }
                                        have = have.wrapping_sub(1);
                                        let fresh8 = next;
                                        next = next.offset(1);
                                        hold = hold.wrapping_add(
                                            (*fresh8 as c_ulong) << bits,
                                        );
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
                                        len = (*state).lens[(*state)
                                            .have
                                            .wrapping_sub(1 as c_uint)
                                            as usize]
                                            as c_uint;
                                        copy = (3 as c_uint).wrapping_add(
                                            hold as c_uint
                                                & ((1 as c_uint)
                                                    << 2 as c_int)
                                                    .wrapping_sub(1 as c_uint),
                                        );
                                        hold >>= 2 as c_int;
                                        bits = bits.wrapping_sub(
                                            2 as c_int as c_uint,
                                        );
                                    }
                                } else if here.val as c_int == 17 as c_int
                                {
                                    while bits
                                        < (here.bits as c_int
                                            + 3 as c_int)
                                            as c_uint
                                    {
                                        if have == 0 as c_uint {
                                            have = in_0.expect("non-null function pointer")(
                                                in_desc,
                                                &raw mut next,
                                            );
                                            if have == 0 as c_uint {
                                                next =
                                                    ::core::ptr::null_mut::<c_uchar>();
                                                ret = Z_BUF_ERROR;
                                                break 's_69;
                                            }
                                        }
                                        have = have.wrapping_sub(1);
                                        let fresh9 = next;
                                        next = next.offset(1);
                                        hold = hold.wrapping_add(
                                            (*fresh9 as c_ulong) << bits,
                                        );
                                        bits = bits.wrapping_add(8 as c_uint);
                                    }
                                    hold >>= here.bits as c_int;
                                    bits = bits.wrapping_sub(here.bits as c_uint);
                                    len = 0 as c_uint;
                                    copy = (3 as c_uint).wrapping_add(
                                        hold as c_uint
                                            & ((1 as c_uint)
                                                << 3 as c_int)
                                                .wrapping_sub(1 as c_uint),
                                    );
                                    hold >>= 3 as c_int;
                                    bits = bits.wrapping_sub(
                                        3 as c_int as c_uint,
                                    );
                                } else {
                                    while bits
                                        < (here.bits as c_int
                                            + 7 as c_int)
                                            as c_uint
                                    {
                                        if have == 0 as c_uint {
                                            have = in_0.expect("non-null function pointer")(
                                                in_desc,
                                                &raw mut next,
                                            );
                                            if have == 0 as c_uint {
                                                next =
                                                    ::core::ptr::null_mut::<c_uchar>();
                                                ret = Z_BUF_ERROR;
                                                break 's_69;
                                            }
                                        }
                                        have = have.wrapping_sub(1);
                                        let fresh10 = next;
                                        next = next.offset(1);
                                        hold = hold.wrapping_add(
                                            (*fresh10 as c_ulong) << bits,
                                        );
                                        bits = bits.wrapping_add(8 as c_uint);
                                    }
                                    hold >>= here.bits as c_int;
                                    bits = bits.wrapping_sub(here.bits as c_uint);
                                    len = 0 as c_uint;
                                    copy = (11 as c_uint).wrapping_add(
                                        hold as c_uint
                                            & ((1 as c_uint)
                                                << 7 as c_int)
                                                .wrapping_sub(1 as c_uint),
                                    );
                                    hold >>= 7 as c_int;
                                    bits = bits.wrapping_sub(
                                        7 as c_int as c_uint,
                                    );
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
                                        let fresh11 = copy;
                                        copy = copy.wrapping_sub(1);
                                        if !(fresh11 != 0) {
                                            break;
                                        }
                                        let fresh12 = (*state).have;
                                        (*state).have = (*state).have.wrapping_add(1);
                                        (*state).lens[fresh12 as usize] =
                                            len as c_ushort;
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
                                    (*state).mode = LEN;
                                }
                            }
                        }
                    }
                }
            }
            16200 => {}
            16208 => {
                ret = Z_STREAM_END;
                if left < (*state).wsize {
                    if out.expect("non-null function pointer")(
                        out_desc,
                        (*state).window,
                        (*state).wsize.wrapping_sub(left),
                    ) != 0
                    {
                        ret = Z_BUF_ERROR;
                    }
                }
                break;
            }
            16209 => {
                ret = Z_DATA_ERROR;
                break;
            }
            _ => {
                ret = Z_STREAM_ERROR;
                break;
            }
        }
        if have >= 6 as c_uint && left >= 258 as c_uint {
            (*strm).next_out = put as *mut Bytef;
            (*strm).avail_out = left as uInt;
            (*strm).next_in = next as *mut Bytef;
            (*strm).avail_in = have as uInt;
            (*state).hold = hold;
            (*state).bits = bits;
            if (*state).whave < (*state).wsize {
                (*state).whave = (*state).wsize.wrapping_sub(left);
            }
            inflate_fast(strm, (*state).wsize);
            put = (*strm).next_out as *mut c_uchar;
            left = (*strm).avail_out as c_uint;
            next = (*strm).next_in as *mut c_uchar;
            have = (*strm).avail_in as c_uint;
            hold = (*state).hold;
            bits = (*state).bits;
        } else {
            loop {
                here = *(*state).lencode.offset(
                    (hold as c_uint
                        & ((1 as c_uint) << (*state).lenbits)
                            .wrapping_sub(1 as c_uint)) as isize,
                );
                if here.bits as c_uint <= bits {
                    break;
                }
                if have == 0 as c_uint {
                    have = in_0.expect("non-null function pointer")(in_desc, &raw mut next);
                    if have == 0 as c_uint {
                        next = ::core::ptr::null_mut::<c_uchar>();
                        ret = Z_BUF_ERROR;
                        break 's_69;
                    }
                }
                have = have.wrapping_sub(1);
                let fresh13 = next;
                next = next.offset(1);
                hold = hold.wrapping_add((*fresh13 as c_ulong) << bits);
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
                        have = in_0.expect("non-null function pointer")(in_desc, &raw mut next);
                        if have == 0 as c_uint {
                            next = ::core::ptr::null_mut::<c_uchar>();
                            ret = Z_BUF_ERROR;
                            break 's_69;
                        }
                    }
                    have = have.wrapping_sub(1);
                    let fresh14 = next;
                    next = next.offset(1);
                    hold = hold.wrapping_add((*fresh14 as c_ulong) << bits);
                    bits = bits.wrapping_add(8 as c_uint);
                }
                hold >>= last.bits as c_int;
                bits = bits.wrapping_sub(last.bits as c_uint);
            }
            hold >>= here.bits as c_int;
            bits = bits.wrapping_sub(here.bits as c_uint);
            (*state).length = here.val as c_uint;
            if here.op as c_int == 0 as c_int {
                if left == 0 as c_uint {
                    put = (*state).window;
                    left = (*state).wsize;
                    (*state).whave = left;
                    if out.expect("non-null function pointer")(out_desc, put, left) != 0 {
                        ret = Z_BUF_ERROR;
                        break;
                    }
                }
                let fresh15 = put;
                put = put.offset(1);
                *fresh15 = (*state).length as c_uchar;
                left = left.wrapping_sub(1);
                (*state).mode = LEN;
            } else if here.op as c_int & 32 as c_int != 0 {
                (*state).mode = TYPE;
            } else if here.op as c_int & 64 as c_int != 0 {
                (*strm).msg = b"invalid literal/length code\0" as *const u8
                    as *const c_char
                    as *mut c_char;
                (*state).mode = BAD;
            } else {
                (*state).extra = here.op as c_uint & 15 as c_uint;
                if (*state).extra != 0 as c_uint {
                    while bits < (*state).extra {
                        if have == 0 as c_uint {
                            have = in_0.expect("non-null function pointer")(in_desc, &raw mut next);
                            if have == 0 as c_uint {
                                next = ::core::ptr::null_mut::<c_uchar>();
                                ret = Z_BUF_ERROR;
                                break 's_69;
                            }
                        }
                        have = have.wrapping_sub(1);
                        let fresh16 = next;
                        next = next.offset(1);
                        hold = hold.wrapping_add((*fresh16 as c_ulong) << bits);
                        bits = bits.wrapping_add(8 as c_uint);
                    }
                    (*state).length = (*state).length.wrapping_add(
                        hold as c_uint
                            & ((1 as c_uint) << (*state).extra)
                                .wrapping_sub(1 as c_uint),
                    );
                    hold >>= (*state).extra;
                    bits = bits.wrapping_sub((*state).extra);
                }
                loop {
                    here = *(*state).distcode.offset(
                        (hold as c_uint
                            & ((1 as c_uint) << (*state).distbits)
                                .wrapping_sub(1 as c_uint))
                            as isize,
                    );
                    if here.bits as c_uint <= bits {
                        break;
                    }
                    if have == 0 as c_uint {
                        have = in_0.expect("non-null function pointer")(in_desc, &raw mut next);
                        if have == 0 as c_uint {
                            next = ::core::ptr::null_mut::<c_uchar>();
                            ret = Z_BUF_ERROR;
                            break 's_69;
                        }
                    }
                    have = have.wrapping_sub(1);
                    let fresh17 = next;
                    next = next.offset(1);
                    hold = hold.wrapping_add((*fresh17 as c_ulong) << bits);
                    bits = bits.wrapping_add(8 as c_uint);
                }
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
                        if (last.bits as c_int + here.bits as c_int)
                            as c_uint
                            <= bits
                        {
                            break;
                        }
                        if have == 0 as c_uint {
                            have = in_0.expect("non-null function pointer")(in_desc, &raw mut next);
                            if have == 0 as c_uint {
                                next = ::core::ptr::null_mut::<c_uchar>();
                                ret = Z_BUF_ERROR;
                                break 's_69;
                            }
                        }
                        have = have.wrapping_sub(1);
                        let fresh18 = next;
                        next = next.offset(1);
                        hold = hold.wrapping_add((*fresh18 as c_ulong) << bits);
                        bits = bits.wrapping_add(8 as c_uint);
                    }
                    hold >>= last.bits as c_int;
                    bits = bits.wrapping_sub(last.bits as c_uint);
                }
                hold >>= here.bits as c_int;
                bits = bits.wrapping_sub(here.bits as c_uint);
                if here.op as c_int & 64 as c_int != 0 {
                    (*strm).msg = b"invalid distance code\0" as *const u8
                        as *const c_char
                        as *mut c_char;
                    (*state).mode = BAD;
                } else {
                    (*state).offset = here.val as c_uint;
                    (*state).extra = here.op as c_uint & 15 as c_uint;
                    if (*state).extra != 0 as c_uint {
                        while bits < (*state).extra {
                            if have == 0 as c_uint {
                                have = in_0.expect("non-null function pointer")(
                                    in_desc,
                                    &raw mut next,
                                );
                                if have == 0 as c_uint {
                                    next = ::core::ptr::null_mut::<c_uchar>();
                                    ret = Z_BUF_ERROR;
                                    break 's_69;
                                }
                            }
                            have = have.wrapping_sub(1);
                            let fresh19 = next;
                            next = next.offset(1);
                            hold = hold.wrapping_add((*fresh19 as c_ulong) << bits);
                            bits = bits.wrapping_add(8 as c_uint);
                        }
                        (*state).offset = (*state).offset.wrapping_add(
                            hold as c_uint
                                & ((1 as c_uint) << (*state).extra)
                                    .wrapping_sub(1 as c_uint),
                        );
                        hold >>= (*state).extra;
                        bits = bits.wrapping_sub((*state).extra);
                    }
                    if (*state).offset
                        > (*state).wsize.wrapping_sub(
                            (if (*state).whave < (*state).wsize {
                                left
                            } else {
                                0 as c_uint
                            }),
                        )
                    {
                        (*strm).msg = b"invalid distance too far back\0" as *const u8
                            as *const c_char
                            as *mut c_char;
                        (*state).mode = BAD;
                    } else {
                        loop {
                            if left == 0 as c_uint {
                                put = (*state).window;
                                left = (*state).wsize;
                                (*state).whave = left;
                                if out.expect("non-null function pointer")(out_desc, put, left) != 0
                                {
                                    ret = Z_BUF_ERROR;
                                    break 's_69;
                                }
                            }
                            copy = (*state).wsize.wrapping_sub((*state).offset);
                            if copy < left {
                                from = put.offset(copy as isize);
                                copy = left.wrapping_sub(copy);
                            } else {
                                from = put.offset(-((*state).offset as isize));
                                copy = left;
                            }
                            if copy > (*state).length {
                                copy = (*state).length;
                            }
                            (*state).length = (*state).length.wrapping_sub(copy);
                            left = left.wrapping_sub(copy);
                            loop {
                                let fresh20 = from;
                                from = from.offset(1);
                                let fresh21 = put;
                                put = put.offset(1);
                                *fresh21 = *fresh20;
                                copy = copy.wrapping_sub(1);
                                if !(copy != 0) {
                                    break;
                                }
                            }
                            if !((*state).length != 0 as c_uint) {
                                break;
                            }
                        }
                    }
                }
            }
        }
    }
    (*strm).next_in = next as *mut Bytef;
    (*strm).avail_in = have as uInt;
    return ret;
}
#[inline]
pub fn inflateBackEnd(mut strm: z_streamp) -> c_int { unsafe {
    if strm.is_null() || (*strm).state.is_null() || (*strm).zfree.is_none() {
        return Z_STREAM_ERROR;
    }
    Some((*strm).zfree.expect("non-null function pointer")).expect("non-null function pointer")(
        (*strm).opaque,
        (*strm).state as voidpf,
    );
    (*strm).state = ::core::ptr::null_mut::<internal_state>();
    return Z_OK;
} }
